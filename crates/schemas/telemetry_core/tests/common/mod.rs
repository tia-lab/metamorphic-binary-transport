#![allow(dead_code)]
use mbt_schema_telemetry::telemetry_v1::*;
pub const CAP: usize = 1 << 20;
pub type TestResult = Result<(), Box<dyn std::error::Error>>;

pub fn rows() -> Vec<TelemetryRowV1> {
    vec![
        TelemetryRowV1 {
            schema_version: 1,
            device_ordinal: DEVICE_SENSOR_A,
            recorded_at_ms: 0,
            temperature_c: 21.5,
            battery_percent: 0.0,
            status_ordinal: STATUS_ACTIVE,
            tags_mask: 5,
            presence_bits: 0,
        },
        TelemetryRowV1 {
            schema_version: 1,
            device_ordinal: DEVICE_SENSOR_A,
            recorded_at_ms: 1000,
            temperature_c: 0.0,
            battery_percent: 0.0,
            status_ordinal: STATUS_IDLE,
            tags_mask: 6,
            presence_bits: 1,
        },
        TelemetryRowV1 {
            schema_version: 1,
            device_ordinal: DEVICE_SENSOR_B,
            recorded_at_ms: -1000,
            temperature_c: -4.25,
            battery_percent: 50.0,
            status_ordinal: STATUS_OFFLINE,
            tags_mask: 0,
            presence_bits: 1,
        },
    ]
}

#[cfg(any(feature = "json", feature = "csv", feature = "protobuf"))]
pub fn check_row_format(format: &str, projected: bool, bytes: &[u8]) -> TestResult {
    use std::io::Write;
    use std::process::{Command, Stdio};
    // Independent standard-library decoders compare logical values, including absence.
    let script = r#"
import sys,json,csv,io,struct
fmt,projected=sys.argv[1],sys.argv[2]=='true'
data=sys.stdin.buffer.read()
expected=[dict(schema_version=1,device='sensor_a',recorded_at_ms=0,recorded_at_utc='1970-01-01T00:00:00Z',temperature_c=21.5,status='active',tags=['indoor','test']),dict(schema_version=1,device='sensor_a',recorded_at_ms=1000,recorded_at_utc='1970-01-01T00:00:01Z',temperature_c=0.,battery_percent=0.,status='idle',tags=['outdoor','test']),dict(schema_version=1,device='sensor_b',recorded_at_ms=-1000,recorded_at_utc='1969-12-31T23:59:59Z',temperature_c=-4.25,battery_percent=50.,status='offline',tags=[])]
if projected:
 expected=[{k:r[k] for k in ('schema_version','device','recorded_at_ms','recorded_at_utc','temperature_c')} for r in expected]
def varint(data,i):
 value=0
 for shift in range(0,70,7):
  b=data[i];i+=1;value|=(b&127)<<shift
  if b<128:return value,i
 raise ValueError('invalid varint')
def fields(data):
 out=[];i=0
 while i<len(data):
  key,i=varint(data,i);wire=key&7
  if wire==0:value,i=varint(data,i)
  elif wire==1:value=struct.unpack('<d',data[i:i+8])[0];i+=8
  elif wire==2:
   size,i=varint(data,i);value=data[i:i+size];i+=size
  else:raise ValueError('unexpected wire type')
  out.append((key>>3,value))
 assert i==len(data)
 return out
if fmt=='json':
 response=json.loads(data);assert response['schema_version']==1;actual=response['rows']
elif fmt=='csv':
 actual=list(csv.DictReader(io.StringIO(data.decode())))
 for row in actual:
  for k in ('schema_version','recorded_at_ms'):row[k]=int(row[k])
  row['temperature_c']=float(row['temperature_c'])
  if 'battery_percent' in row:
   if row['battery_percent']=='':del row['battery_percent']
   else:row['battery_percent']=float(row['battery_percent'])
  if 'tags' in row:row['tags']=json.loads(row['tags'])
else:
 root=fields(data);assert root[0]==(1,1);actual=[]
 names={1:'schema_version',2:'device',3:'recorded_at_ms',4:'recorded_at_utc',5:'temperature_c',6:'battery_percent',7:'status',8:'tags'}
 for tag,payload in root[1:]:
  assert tag==2;row={} if projected else {'tags':[]}
  for tag,value in fields(payload):
   name=names[tag]
   if isinstance(value,bytes):value=value.decode()
   if name=='recorded_at_ms' and value>=2**63:value-=2**64
   if name=='tags':row['tags'].append(value)
   else:row[name]=value
  actual.append(row)
assert actual==expected,(fmt,actual,expected)
"#;
    let mut child = Command::new("python3")
        .args([
            "-c",
            script,
            format,
            if projected { "true" } else { "false" },
        ])
        .stdin(Stdio::piped())
        .spawn()?;
    child
        .stdin
        .take()
        .ok_or("missing oracle stdin")?
        .write_all(bytes)?;
    if !child.wait()?.success() {
        return Err("row-format oracle failed".into());
    }
    Ok(())
}

#[cfg(feature = "arrow")]
pub fn check_arrow(batch: &mbt_adapter_arrow::ArrowRecordBatch, projected: bool) -> TestResult {
    let source = rows();
    let mut expected: Vec<(&str, Vec<u8>)> = vec![
        (
            "schema_version",
            source
                .iter()
                .flat_map(|r| r.schema_version.to_ne_bytes())
                .collect(),
        ),
        (
            "device",
            source
                .iter()
                .flat_map(|r| r.device_ordinal.to_ne_bytes())
                .collect(),
        ),
        (
            "recorded_at_ms",
            source
                .iter()
                .flat_map(|r| r.recorded_at_ms.to_ne_bytes())
                .collect(),
        ),
        (
            "temperature_c",
            source
                .iter()
                .flat_map(|r| r.temperature_c.to_ne_bytes())
                .collect(),
        ),
    ];
    if !projected {
        expected.extend([
            (
                "battery_percent",
                source
                    .iter()
                    .flat_map(|r| r.battery_percent.to_ne_bytes())
                    .collect(),
            ),
            (
                "status",
                source
                    .iter()
                    .flat_map(|r| r.status_ordinal.to_ne_bytes())
                    .collect(),
            ),
            (
                "tags",
                source
                    .iter()
                    .flat_map(|r| r.tags_mask.to_ne_bytes())
                    .collect(),
            ),
        ]);
    }
    if batch.num_rows() != source.len() || batch.num_columns() != expected.len() {
        return Err("unexpected columnar shape".into());
    }
    for (idx, (name, values)) in expected.iter().enumerate() {
        if batch.schema().field(idx).name() != name {
            return Err("unexpected column name".into());
        }
        let column = batch.column(idx);
        let data = column.to_data();
        if data.buffers().len() != 1 || data.buffers()[0].as_slice() != values {
            return Err(format!("column values differ: {name}").into());
        }
        for row in 0..source.len() {
            if column.is_null(row) != (*name == "battery_percent" && row == 0) {
                return Err("column nullability differs".into());
            }
        }
    }
    Ok(())
}
