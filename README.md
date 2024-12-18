# はじめての Goose

serve this site on local server: [makinzm/rust-wasm-github](https://github.com/makinzm/rust-wasm-github)

```
❯ cargo run -- --host http://127.0.0.1:8080 --debug-log debug.log

15:22:06 [INFO] Output verbosity level: INFO
15:22:06 [INFO] Logfile verbosity level: WARN
15:22:06 [INFO] users defaulted to number of CPUs = 20
15:22:06 [INFO] iterations = 0
15:22:06 [INFO] global host configured: http://127.0.0.1:8080
15:22:06 [INFO] allocating transactions and scenarios with RoundRobin scheduler
15:22:06 [INFO] initializing 20 user states...
15:22:06 [INFO] entering GooseAttack phase: Increase
15:22:06 [INFO] launching user 1 from LoadtestTransactions...
15:22:07 [INFO] writing debug file to: debug.log
15:22:07 [INFO] launching user 2 from LoadtestTransactions...
15:22:08 [INFO] launching user 3 from LoadtestTransactions...
15:22:09 [INFO] launching user 4 from LoadtestTransactions...
15:22:10 [INFO] launching user 5 from LoadtestTransactions...
15:22:11 [INFO] launching user 6 from LoadtestTransactions...
15:22:12 [INFO] launching user 7 from LoadtestTransactions...
15:22:13 [INFO] launching user 8 from LoadtestTransactions...
15:22:14 [INFO] launching user 9 from LoadtestTransactions...
15:22:16 [INFO] launching user 10 from LoadtestTransactions...
15:22:18 [INFO] launching user 11 from LoadtestTransactions...
15:22:19 [INFO] launching user 12 from LoadtestTransactions...
15:22:20 [INFO] launching user 13 from LoadtestTransactions...
15:22:21 [INFO] launching user 14 from LoadtestTransactions...
15:22:23 [INFO] launching user 15 from LoadtestTransactions...
15:22:24 [INFO] launching user 16 from LoadtestTransactions...
15:22:25 [INFO] launching user 17 from LoadtestTransactions...
15:22:26 [INFO] launching user 18 from LoadtestTransactions...
15:22:27 [INFO] launching user 19 from LoadtestTransactions...
15:22:29 [INFO] printing running metrics after 22 seconds...

 === PER SCENARIO METRICS ===
 ------------------------------------------------------------------------------
 Name                     |  # users |  # times run | scenarios/s | iterations
 ------------------------------------------------------------------------------
 1: LoadtestTransactions  |       19 |        37621 |        1710 |       1980
 ------------------------------------------------------------------------------
 Name                     |    Avg (ms) |        Min |         Max |     Median
 ------------------------------------------------------------------------------
   1: LoadtestTransacti.. |        5.67 |          2 |          14 |          5

 === PER TRANSACTION METRICS ===
 ------------------------------------------------------------------------------
 Name                     |   # times run |        # fails |  trans/s |  fail/s
 ------------------------------------------------------------------------------
 1: LoadtestTransactions
   1:                     |        37,621 |         0 (0%) |     1710 |    0.00
 ------------------------------------------------------------------------------
 Name                     |    Avg (ms) |        Min |         Max |     Median
 ------------------------------------------------------------------------------
 1: LoadtestTransactions
   1:                     |        5.67 |          2 |          14 |          5

 === PER REQUEST METRICS ===
 ------------------------------------------------------------------------------
 Name                     |        # reqs |        # fails |    req/s |  fail/s
 ------------------------------------------------------------------------------
 GET /rust-wasm-github/   |        37,635 |         0 (0%) |     1711 |    0.00
 GET static asset: css    |        37,621 |         0 (0%) |     1710 |    0.00
 -------------------------+---------------+----------------+----------+--------
 Aggregated               |        75,256 |         0 (0%) |     3421 |    0.00
 ------------------------------------------------------------------------------
 Name                     |    Avg (ms) |        Min |         Max |     Median
 ------------------------------------------------------------------------------
 GET /rust-wasm-github/   |        0.84 |          1 |           7 |          1
 GET static asset: css    |        1.37 |          1 |           7 |          1
 -------------------------+-------------+------------+-------------+-----------
 Aggregated               |        1.11 |          1 |           7 |          1
 ------------------------------------------------------------------------------
 Name                     |                                        Status codes
 ------------------------------------------------------------------------------
 GET /rust-wasm-github/   |                                        37,635 [200]
 GET static asset: css    |                                        37,621 [200]
 -------------------------+----------------------------------------------------
 Aggregated               |                                        75,256 [200]

All 20 users hatched, resetting metrics (disable with --no-reset-metrics).

15:22:29 [INFO] launching user 20 from LoadtestTransactions...
15:22:29 [INFO] entering GooseAttack phase: Maintain
```


Ref: [What Is Goose? - The Goose Book](https://book.goose.rs/title-page.html)
