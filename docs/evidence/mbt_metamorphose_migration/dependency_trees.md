# MBT Metamorphose Migration Dependency Tree Evidence

Status: partial validation complete.

Date: 2026-06-15

## Selected Feature Boundary Checks

Command:

```bash
cargo tree -p metamorphic_binary_transport_schema_bars --features arrow_ipc | rg "metamorphic_binary_transport_adapter_arrow($| )|metamorphic_binary_transport_adapter_arrow_ipc|metamorphic_binary_transport_transponding"
```

Observed relevant output:

```text
├── metamorphic_binary_transport_adapter_arrow_ipc v0.1.0 (/media/Development/MATHILDE/metamorphic-binary-transport/crates/adapters/arrow_ipc)
│   └── metamorphic_binary_transport_transponding v0.1.0 (/media/Development/MATHILDE/metamorphic-binary-transport/crates/transponding)
├── metamorphic_binary_transport_transponding v0.1.0 (/media/Development/MATHILDE/metamorphic-binary-transport/crates/transponding) (*)
```

This output did not include `metamorphic_binary_transport_adapter_arrow`.

Command:

```bash
cargo tree -p metamorphic_binary_transport_schema_bars --features parquet | rg "metamorphic_binary_transport_adapter_arrow($| )|metamorphic_binary_transport_adapter_parquet|metamorphic_binary_transport_transponding"
```

Observed relevant output:

```text
├── metamorphic_binary_transport_adapter_parquet v0.1.0 (/media/Development/MATHILDE/metamorphic-binary-transport/crates/adapters/parquet)
│   ├── metamorphic_binary_transport_transponding v0.1.0 (/media/Development/MATHILDE/metamorphic-binary-transport/crates/transponding)
├── metamorphic_binary_transport_transponding v0.1.0 (/media/Development/MATHILDE/metamorphic-binary-transport/crates/transponding) (*)
```

This output did not include `metamorphic_binary_transport_adapter_arrow`.

## Interpretation

The selected `arrow_ipc` and `parquet` schema features no longer pull the Arrow
adapter crate. They pull their selected writer adapter plus transponding, which
matches the approved compile-surface boundary.
