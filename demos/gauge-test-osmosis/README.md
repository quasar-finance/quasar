## OSMOSIS GAUGE TEST 

### Background 
Osmosis has a way to incentives pool locked tokens users that can receive incentives on epoch basis from the gauge created 
by user to attract users to bond the liquidity that they have provided for more yield on their locked assets.

### Steps to run this test :
* Clone the Osmosis repo and checkout the latest version
* Change current directory to gauge-test-osmosis
    ```shell
    cd demos/gauge-test-osmosis
  ```
* Start Osmosis chain using 
    ```shell
    sh osmo_localnet.sh
  ```
* Wait for the chain to start, this can be monitored from :
    ```shell
  osmosisd status
  ```
* Once the chain is producing blocks, run (**as soon as the chain is up to match timings of minute epoch**) : 
    ```shell
  sh gauge-test.sh 
  ```
* You would be able to see 2 transactions being executed and then balance of the account appears every 60s.