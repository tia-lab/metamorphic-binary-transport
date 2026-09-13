# MBT Compression Summary

This summary is generated from the latest compression benchmark report.

| Label     | Lane                 |   Rows | Uncompressed bytes | Compressed bytes |    Ratio | Compress MB/s | Decompress MB/s |
| --------- | -------------------- | -----: | -----------------: | ---------------: | -------: | ------------: | --------------: |
| one       | mbt_full             |      1 |                184 |              108 | 0.586957 |      9.976864 |       71.892852 |
| one       | mbt_temperature_only |      1 |                158 |               94 | 0.594937 |     11.533887 |      128.138429 |
| small     | mbt_full             |    100 |               4738 |              563 | 0.118827 |    177.759822 |      504.954987 |
| small     | mbt_temperature_only |    100 |               2138 |              406 | 0.189897 |     67.061690 |      491.316992 |
| page_500  | mbt_full             |    500 |              23138 |             1858 | 0.080301 |    379.007439 |      969.302635 |
| page_500  | mbt_temperature_only |    500 |              10138 |             1348 | 0.132965 |    250.914820 |      812.466405 |
| page_1000 | mbt_full             |   1000 |              46138 |             3483 | 0.075491 |    705.534642 |     1535.575484 |
| page_1000 | mbt_temperature_only |   1000 |              20138 |             2501 | 0.124193 |    451.008003 |     1335.614406 |
| medium    | mbt_full             |  10000 |             460138 |            32645 | 0.070946 |   1082.953613 |     2361.422857 |
| medium    | mbt_temperature_only |  10000 |             200138 |            23055 | 0.115196 |   1053.541162 |     2190.931124 |
| large     | mbt_full             | 100000 |            4600138 |           321302 | 0.069846 |   1248.128475 |     2875.355542 |
| large     | mbt_temperature_only | 100000 |            2000138 |           207540 | 0.103763 |   1202.930350 |     2777.488794 |
