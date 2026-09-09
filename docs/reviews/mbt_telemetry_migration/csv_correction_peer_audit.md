# CSV correction amendment audit

Classification: `PEER_AUDIT_PASSED` for the proposed correction design.
Method: single-agent falsification review; no independent reviewer claimed.
Owner approved the amendment on 2026-09-09.

The observed failing read-back test and inspected writer/emitter establish a
quoting-layer mismatch. A method dedicated to array string elements avoids
changing standalone CSV strings or numeric arrays. JSON escaping before CSV
quote doubling covers embedded quotes, backslashes, ASCII controls and Unicode.
Byte-wise processing preserves non-ASCII UTF-8. Existing CheckedBytes propagates
cap failures and avoids a hidden allocation. The specified tests include those
edge cases and independent standard-library decoders.

Scope is exact: one adapter method, one generator call, adapter/generator tests,
and two regenerated CSV outputs. Schema hashes and archive bytes are unaffected.
No new dependency is required. The parent plan's other constraints remain.

Adapter tests (5) and generator tests (45) pass in csv_correction_tests.txt.
Final workspace and benchmark evidence is bound by the migration result review.
