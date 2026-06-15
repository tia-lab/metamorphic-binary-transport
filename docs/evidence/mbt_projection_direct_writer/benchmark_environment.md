# MBT Projection Direct Writer Benchmark Environment

Recorded: 2026-06-15T07:48:57Z

This companion artifact records the environment for the final benchmark
artifacts. The benchmark JSON files themselves contain only row measurements,
so this file supplies the environment metadata required by the benchmark
protocol.

## Raw Artifacts

| Artifact | Filesystem timestamp |
|---|---:|
| `docs/evidence/mbt_projection_direct_writer/current_owned_row_projection_baseline.json` | 2026-06-14 19:34:51.896261499 +0000 |
| `docs/evidence/mbt_projection_direct_writer/projection_run_1.json` | 2026-06-14 20:05:49.777413571 +0000 |
| `docs/evidence/mbt_projection_direct_writer/projection_run_2.json` | 2026-06-14 20:06:04.617231030 +0000 |
| `docs/evidence/mbt_projection_direct_writer/projection_run_3.json` | 2026-06-14 20:06:18.149064578 +0000 |

## Operator And Git State

Operator:

```text
tia
```

Git commit:

```text
d035f90bc3a6b312293dc7bba8096e78487ab83b
```

Worktree state at metadata capture:

```text
dirty
```

The dirty state is expected for this implementation branch because the
projection direct-writer source, generated schema files, tests, specs, reviews,
and evidence artifacts are not committed yet.

## Command And Profile

Final benchmark command, run three sequential times:

```bash
cargo run --release -p metamorphic_binary_transport_benches --bin mbt_projection_bench -- --report-dir docs/evidence/mbt_projection_direct_writer
```

Build profile:

```text
release
```

Warm/cold mode:

```text
in-process CPU benchmark; no storage warm/cold split applies
```

## Dataset Identity

Bars schema:

```text
crates/schemas/bars_core/proto/mathilde/binary_transport/v1/bars.proto
root = mathilde.binary_transport.v1.MathildeTransportResponseV1
module = bars_v1
surface = projection
row counts = 1, 100, 500, 1000, 10000, 100000
```

Test compatibility schema:

```text
crates/schemas/test_compatibility_core/proto/mathilde/binary_transport/test_compatibility/v1/all_fields.proto
root = mathilde.binary_transport.test_compatibility.v1.TestCompatibilityResponseV1
module = test_compatibility_v1
surface = projection
row counts = 1, 100, 500, 1000, 10000, 100000
```

## System

OS and kernel:

```text
Linux tia 5.15.0-156-generic #166-Ubuntu SMP Sat Aug 9 00:02:46 UTC 2025 x86_64 x86_64 x86_64 GNU/Linux
```

CPU:

```text
Intel(R) Xeon(R) W-2295 CPU @ 3.00GHz
36 logical CPUs
18 cores
2 threads per core
1 socket
L3 cache: 24.8 MiB
```

RAM:

```text
503 GiB total
454 GiB available at metadata capture
```

Rust toolchain:

```text
rustc 1.90.0 (1159e78c4 2025-09-14)
cargo 1.90.0 (840b83a10 2025-07-30)
host = x86_64-unknown-linux-gnu
LLVM = 20.1.8
```

## Raw Output Paths

```text
docs/evidence/mbt_projection_direct_writer/projection_run_1.json
docs/evidence/mbt_projection_direct_writer/projection_run_2.json
docs/evidence/mbt_projection_direct_writer/projection_run_3.json
docs/evidence/mbt_projection_direct_writer/projection_summary.md
docs/evidence/mbt_projection_direct_writer/current_owned_row_projection_baseline.json
```
