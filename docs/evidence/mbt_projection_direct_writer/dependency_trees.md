# MBT Projection Direct Writer Dependency Trees

```bash
cargo tree -p metamorphic_binary_transport_schema_bars
```

```text
metamorphic_binary_transport_schema_bars v0.1.0 (/media/Development/MATHILDE/metamorphic-binary-transport/crates/schemas/bars_core)
├── metamorphic_binary_transport_core v0.1.0 (/media/Development/MATHILDE/metamorphic-binary-transport/crates/core)
│   └── thiserror v2.0.17
│       └── thiserror-impl v2.0.17 (proc-macro)
│           ├── proc-macro2 v1.0.106
│           │   └── unicode-ident v1.0.24
│           ├── quote v1.0.45
│           │   └── proc-macro2 v1.0.106 (*)
│           └── syn v2.0.117
│               ├── proc-macro2 v1.0.106 (*)
│               ├── quote v1.0.45 (*)
│               └── unicode-ident v1.0.24
└── rkyv v0.8.16
    ├── bytecheck v0.8.2
    │   ├── bytecheck_derive v0.8.2 (proc-macro)
    │   │   ├── proc-macro2 v1.0.106 (*)
    │   │   ├── quote v1.0.45 (*)
    │   │   └── syn v2.0.117 (*)
    │   ├── ptr_meta v0.3.1
    │   │   └── ptr_meta_derive v0.3.1 (proc-macro)
    │   │       ├── proc-macro2 v1.0.106 (*)
    │   │       ├── quote v1.0.45 (*)
    │   │       └── syn v2.0.117 (*)
    │   ├── rancor v0.1.1
    │   │   └── ptr_meta v0.3.1 (*)
    │   └── simdutf8 v0.1.5
    ├── hashbrown v0.17.1
    ├── munge v0.4.7
    │   └── munge_macro v0.4.7 (proc-macro)
    │       ├── proc-macro2 v1.0.106 (*)
    │       ├── quote v1.0.45 (*)
    │       └── syn v2.0.117 (*)
    ├── ptr_meta v0.3.1 (*)
    ├── rancor v0.1.1 (*)
    ├── rend v0.5.3
    │   └── bytecheck v0.8.2 (*)
    └── rkyv_derive v0.8.16 (proc-macro)
        ├── proc-macro2 v1.0.106 (*)
        ├── quote v1.0.45 (*)
        └── syn v2.0.117 (*)
```

```bash
cargo tree -p metamorphic_binary_transport_schema_test_compatibility
```

```text
metamorphic_binary_transport_schema_test_compatibility v0.1.0 (/media/Development/MATHILDE/metamorphic-binary-transport/crates/schemas/test_compatibility_core)
├── metamorphic_binary_transport_core v0.1.0 (/media/Development/MATHILDE/metamorphic-binary-transport/crates/core)
│   └── thiserror v2.0.17
│       └── thiserror-impl v2.0.17 (proc-macro)
│           ├── proc-macro2 v1.0.106
│           │   └── unicode-ident v1.0.24
│           ├── quote v1.0.45
│           │   └── proc-macro2 v1.0.106 (*)
│           └── syn v2.0.117
│               ├── proc-macro2 v1.0.106 (*)
│               ├── quote v1.0.45 (*)
│               └── unicode-ident v1.0.24
└── rkyv v0.8.16
    ├── bytecheck v0.8.2
    │   ├── bytecheck_derive v0.8.2 (proc-macro)
    │   │   ├── proc-macro2 v1.0.106 (*)
    │   │   ├── quote v1.0.45 (*)
    │   │   └── syn v2.0.117 (*)
    │   ├── ptr_meta v0.3.1
    │   │   └── ptr_meta_derive v0.3.1 (proc-macro)
    │   │       ├── proc-macro2 v1.0.106 (*)
    │   │       ├── quote v1.0.45 (*)
    │   │       └── syn v2.0.117 (*)
    │   ├── rancor v0.1.1
    │   │   └── ptr_meta v0.3.1 (*)
    │   └── simdutf8 v0.1.5
    ├── hashbrown v0.17.1
    ├── munge v0.4.7
    │   └── munge_macro v0.4.7 (proc-macro)
    │       ├── proc-macro2 v1.0.106 (*)
    │       ├── quote v1.0.45 (*)
    │       └── syn v2.0.117 (*)
    ├── ptr_meta v0.3.1 (*)
    ├── rancor v0.1.1 (*)
    ├── rend v0.5.3
    │   └── bytecheck v0.8.2 (*)
    └── rkyv_derive v0.8.16 (proc-macro)
        ├── proc-macro2 v1.0.106 (*)
        ├── quote v1.0.45 (*)
        └── syn v2.0.117 (*)
```

```bash
cargo tree -p metamorphic_binary_transport_benches
```

```text
metamorphic_binary_transport_benches v0.1.0 (/media/Development/MATHILDE/metamorphic-binary-transport/crates/benches)
├── metamorphic_binary_transport_core v0.1.0 (/media/Development/MATHILDE/metamorphic-binary-transport/crates/core)
│   └── thiserror v2.0.17
│       └── thiserror-impl v2.0.17 (proc-macro)
│           ├── proc-macro2 v1.0.106
│           │   └── unicode-ident v1.0.24
│           ├── quote v1.0.45
│           │   └── proc-macro2 v1.0.106 (*)
│           └── syn v2.0.117
│               ├── proc-macro2 v1.0.106 (*)
│               ├── quote v1.0.45 (*)
│               └── unicode-ident v1.0.24
├── metamorphic_binary_transport_schema_bars v0.1.0 (/media/Development/MATHILDE/metamorphic-binary-transport/crates/schemas/bars_core)
│   ├── metamorphic_binary_transport_core v0.1.0 (/media/Development/MATHILDE/metamorphic-binary-transport/crates/core) (*)
│   └── rkyv v0.8.16
│       ├── bytecheck v0.8.2
│       │   ├── bytecheck_derive v0.8.2 (proc-macro)
│       │   │   ├── proc-macro2 v1.0.106 (*)
│       │   │   ├── quote v1.0.45 (*)
│       │   │   └── syn v2.0.117 (*)
│       │   ├── ptr_meta v0.3.1
│       │   │   └── ptr_meta_derive v0.3.1 (proc-macro)
│       │   │       ├── proc-macro2 v1.0.106 (*)
│       │   │       ├── quote v1.0.45 (*)
│       │   │       └── syn v2.0.117 (*)
│       │   ├── rancor v0.1.1
│       │   │   └── ptr_meta v0.3.1 (*)
│       │   └── simdutf8 v0.1.5
│       ├── hashbrown v0.17.1
│       ├── munge v0.4.7
│       │   └── munge_macro v0.4.7 (proc-macro)
│       │       ├── proc-macro2 v1.0.106 (*)
│       │       ├── quote v1.0.45 (*)
│       │       └── syn v2.0.117 (*)
│       ├── ptr_meta v0.3.1 (*)
│       ├── rancor v0.1.1 (*)
│       ├── rend v0.5.3
│       │   └── bytecheck v0.8.2 (*)
│       └── rkyv_derive v0.8.16 (proc-macro)
│           ├── proc-macro2 v1.0.106 (*)
│           ├── quote v1.0.45 (*)
│           └── syn v2.0.117 (*)
└── metamorphic_binary_transport_schema_test_compatibility v0.1.0 (/media/Development/MATHILDE/metamorphic-binary-transport/crates/schemas/test_compatibility_core)
    ├── metamorphic_binary_transport_core v0.1.0 (/media/Development/MATHILDE/metamorphic-binary-transport/crates/core) (*)
    └── rkyv v0.8.16 (*)
```
