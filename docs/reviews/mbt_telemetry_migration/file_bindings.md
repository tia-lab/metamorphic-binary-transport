# Telemetry migration file bindings

Status: pre-implementation working-tree snapshot; not a clean-HEAD baseline.

The spec controls the operation for each path. Unchanged benchmark CLI/module files
listed here are read/preserve bindings. Existing edits must be retained. Hashes
identify observed content, not approval or test evidence.

| Existing path | SHA-256 |
| --- | --- |
| `.github/workflows/ci.yml` | `2a5ca08ca7c5fb01a046793d76660d5d26b2edc0b419d1879b3e09db085f61b1` |
| `.gitignore` | `43a442aaf8af6d4b91988175f9024007aa25a3226b4796f91d9c5aed91a2573f` |
| `Cargo.toml` | `782cc4304191f21a8da2983531360cfbef7e81bea207678234dddf2353ec0f3f` |
| `README.md` | `d05e881e64d2a60473763b7893b326b5d896ffbb44bc0327bc7dd2060d2eb5f0` |
| `architecture.md` | `8ca6030cc078438125126f0f9433474b61fa6f67099a329ff8c038c001ea222a` |
| `crates/benches/Cargo.toml` | `fc56407183a258b7ba1cc4a9d7842d07745289d6e7df6f07ef5967d21e6c3b8c` |
| `crates/benches/docs/inventory.md` | `b46e8a1103f06ac678eba10a50940541aa9fa881397fc36b66f7cfcaefe6bf0b` |
| `crates/benches/src/bars_regression.rs` | `5077d7e99a5464ddccf3bfab43b185ff0a1cf73f5b122dd17ca74de65cc9765c` |
| `crates/benches/src/bin/mbt_bars_regression_bench.rs` | `320315903f779f26628b45c40b9da7aa43f254e621a8edacf77c556279bff57b` |
| `crates/benches/src/bin/mbt_compression_bench.rs` | `88d20b513f3f4bd4f709128ecb362f45e6b0d5c7355be799bd50360ac823a984` |
| `crates/benches/src/bin/mbt_projection_bench.rs` | `c578c58400c1952af77b23f872eb8edcd76b93943115fb1a03809eeb72a7dffd` |
| `crates/benches/src/compression.rs` | `fd1fd8cddf2578d7aef2e534b9a8ead4a90609361b2430b1596738d7b396a21c` |
| `crates/benches/src/lib.rs` | `270e2650a74af0e23985b86de979de7c72864459c34fa7d93af1a5f9fcb2186d` |
| `crates/benches/src/projection.rs` | `098a14ca8b19b650c8ad31a08e696c4b5bcd378714d7da86cb20bdac705184a7` |
| `crates/benches/src/tests/mod.rs` | `b659412d9642b180712b6f19a66af393b09757b22a1c338ca5ec5bb1c5357509` |
| `crates/benches/src/tests/test_bars_regression_bench_output.rs` | `246cfb5e5773560776206655e0c206bf2d73b6b3a68c6e2fa5fe91d9bc9eb9e0` |
| `crates/benches/src/tests/test_compression_bench_output.rs` | `9e9aacb60bb29fbfb1870bff7df136ba51db351c5d75661666c58652b5bad3a4` |
| `crates/benches/src/tests/test_projection_bench_output.rs` | `e1759d137e6dc46a4e6c02b7b18b2eea5d898e5aa5f81c7b34b93cb6dd2cb8e9` |
| `crates/codegen/src/options.rs` | `8c90d540a569c0af4457b51c7d0d6fbdfd3ae962c6bcfa4c9fc5e27a6f164154` |
| `crates/codegen/src/tests/mod.rs` | `ab22521ee29b3ae76583d9edb3cbfb4f28d076e32ff5b5c98fb5388ff9c5e56d` |
| `crates/codegen/src/tests/test_descriptor.rs` | `62bf338beddd4905d5549a7fafce8466e0260df298c5bd57377b26b4095b4dbd` |
| `crates/codegen/src/tests/test_rust_emit_metamorphose.rs` | `a1fe338d180ceadf753cfc2352aba437851dcb3fa0fb4e0c2c3ec1f57ade1750` |
| `crates/codegen/target/mbt-codegen-fixtures/model-3154518-26/mathilde/options.proto` | `e5bbbc2f8dab1cadbccd99f75e28d6318e52c6bddec9dfd1a27d8fe9c4afadec` |
| `crates/compression/src/tests/test_runtime.rs` | `f9ab74d68b21ed271f221e75860cc0058c236d2410a2b891ead7cf561b557b8a` |
| `crates/core/src/envelope.rs` | `3548ed4fc8461f0575989448a75cf29668a0a5e5da50e6700f9a4ced49c5f1ac` |
| `crates/schemas/bars_core/Cargo.toml` | `63279d82e38894c658f88fd16bb6ca00569e02449fa39105f0f0a30105e29d9b` |
| `crates/schemas/bars_core/docs/inventory.md` | `99ef4577c273c5b2dc9ddeeeb52f02d8904cfbdfc8939b67ee98a1b87239710f` |
| `crates/schemas/bars_core/proto/mathilde/binary_transport/v1/bars.proto` | `c31b85723a70c96de30db171ac979fa6f80600ca9c29286ff64b04ecd80adb58` |
| `crates/schemas/bars_core/src/bars_v1.rs` | `f57efd4368269b4c76a5ff331dbf941c3c91e386f3ee7a7ffc2229fce7cc5cd6` |
| `crates/schemas/bars_core/src/bars_v1_arrow.rs` | `480175e6914dfd334d26349f4e7bd13a216b0457d67ac0b2d33851d8f341d495` |
| `crates/schemas/bars_core/src/bars_v1_arrow_ipc.rs` | `ced7f83e7857142c0b20b99042738f2dfa6311b1d65d2b71403f9b1797bf95d8` |
| `crates/schemas/bars_core/src/bars_v1_csv.rs` | `f6f2edab697ba2f6d12ffd27d353be878c6dbf0eedbc5d7beeac67c363843a50` |
| `crates/schemas/bars_core/src/bars_v1_json.rs` | `a1b30bd40ff7f2c8b7a80b6d34694f1a1c1501a67482c335cbbdc79fa638cd8a` |
| `crates/schemas/bars_core/src/bars_v1_parquet.rs` | `24ce87f06f9069a08615c91e8e1aba11f89d1fbcd08195b9335fa719d7e82275` |
| `crates/schemas/bars_core/src/bars_v1_protobuf.rs` | `0b19274974a85bc3d4d192ada50616cb0e6e86bd11206315fe9b2aa7cb59209d` |
| `crates/schemas/bars_core/src/bars_v1_transponding.rs` | `e114b0f60f66034f2e11f69693cd7be377ed063ef80d7642bb6aebb8ec88b51e` |
| `crates/schemas/bars_core/src/lib.rs` | `4f26e49010747b9c65998ba196f9dce8e6f871997fdbec0200d44e9e7cb5ef3a` |
| `crates/schemas/bars_core/tests/expected_bars_projection_inspect.txt` | `34fc29794df8a74b4ee6385b7317423581f01f626e56ed7549aaf6273f5ab62b` |
| `crates/schemas/bars_core/tests/test_bars_metamorphose.rs` | `7bd738cf8d97617fcf299fde20b03b22495ca40ce6b1900e1b4b0fe7fd62b34f` |
| `crates/schemas/bars_core/tests/test_bars_projection.rs` | `c6229da76e97f197d79133807f98d02758eab364c2b7cdc2afd7941605e926c8` |
| `crates/schemas/bars_core/tests/test_bars_shape.rs` | `ff0542395e902865a06223a8bdc9ab090f693a53e794d6380b5ab87a294ab966` |
| `crates/schemas/bars_core/tests/test_projection_metamorphose.rs` | `bec0608c95458f585eae6cbb8a6a773fff4ac2939c445bf61d6fddb87c3cd03d` |
| `crates/schemas/test_compatibility_core/proto/mathilde/binary_transport/test_compatibility/v1/all_fields.proto` | `13104d73677d518592f88a9210123bff48a6e1f042723060b055ed5bcdbe9bac` |
| `crates/schemas/test_compatibility_core/src/lib.rs` | `0c37eaecd65ede20d419ce316269ae0a78337b6e15246c41c572ada3da836c1b` |
| `crates/schemas/test_compatibility_core/src/test_compatibility_v1.rs` | `cab8fb165daecca76e72e862d7e7bf98fe019ad458e4c84136ce544332d53ff4` |
| `crates/schemas/test_compatibility_core/src/test_compatibility_v1_arrow.rs` | `66df369a4f05b8ed9526e7fe0adf604798dd49699a01633bdbb37b2f2c45535c` |
| `crates/schemas/test_compatibility_core/src/test_compatibility_v1_arrow_ipc.rs` | `3323bd8f57f52b3928a3b9c1bf8eeeb0bf2c94fb203b01dcc68dd296243ad325` |
| `crates/schemas/test_compatibility_core/src/test_compatibility_v1_csv.rs` | `cc98486bcff10501152cd7855c9e293ca80bb09fe4285a5248d21609c2e17119` |
| `crates/schemas/test_compatibility_core/src/test_compatibility_v1_json.rs` | `8e67d679ac776142e77c607fa68d62bfa6a65d1d35bff24d441f5daaf4efc543` |
| `crates/schemas/test_compatibility_core/src/test_compatibility_v1_parquet.rs` | `665dbdeb839dfe6c0c089a64cbf88be1e592be0776b16c473a77bd16ca9f540c` |
| `crates/schemas/test_compatibility_core/src/test_compatibility_v1_protobuf.rs` | `08d24376577df931a984060e1fbf3a170b270c29641d258231e9adf49ff517d3` |
| `crates/schemas/test_compatibility_core/src/test_compatibility_v1_transponding.rs` | `ee801ac2d8d469f931fccffc1dc59268c5e70c7177e0ee77f6572ea5ed4b722b` |
| `crates/schemas/test_compatibility_core/tests/test_determinism.rs` | `fc05404cdc8417bdd8a27938b5a13303747fb4d935e3f91ac66fca268f639f2a` |
| `crates/schemas/test_compatibility_core/tests/test_failures.rs` | `22b0c07be7d1b5d860e4c6f1d99034df1f8aec233420ee8348c631186159138d` |
| `crates/schemas/test_compatibility_core/tests/test_projection.rs` | `03aaaad629139d1fabcabf73ef4a7e56f3f80bbf88faddcfccff8e26c41dd3bc` |
| `crates/schemas/test_compatibility_core/tests/test_projection_metamorphose.rs` | `fbcda2c17ea36e5f8ab62e3a3e531a8f3bd5600fd99bc1b598c4877c8f304851` |
| `crates/schemas/test_compatibility_core/tests/test_roundtrip.rs` | `879fb7a024b8ad879900e7f028111af216d7c64a91425bf720bd19e0e4016cd3` |
| `docs/architecture/repository_structure.md` | `11996f12a862b2f8ac12e9a2fc686c04f647241b1e82f4e9a874702fa07c45da` |
| `inventory.md` | `c654bf48a85ed7e8538f25d524467781a1292d3f8ff313beff3008c3bfd5f026` |
| `proto/mathilde/options.proto` | `e5bbbc2f8dab1cadbccd99f75e28d6318e52c6bddec9dfd1a27d8fe9c4afadec` |

New paths and output ownership are defined by spec sections 6, 9, 18–21.

## Approved CSV amendment snapshots

| `crates/adapters/csv/src/lib.rs` | `6e80b163675b01afc20bf9192d5070c472fa5e2efca5bd7059d8169ab98a2632` |
| `crates/adapters/csv/src/tests/mod.rs` | `af64d80adc4c3ee3fa91f87df0be706a0f0cb6f350d05cf52082152589c94e68` |
| `crates/codegen/src/rust_emit.rs` | `9a0f06b9d79086c7bfac54a8ee67c03f24706e6461efe8114d051c8628ee6bbc` |
