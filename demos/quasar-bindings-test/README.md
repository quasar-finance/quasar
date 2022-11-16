# qOracle bindings test

* in a separate terminal window, run ./quasar_localnet.sh
* in a separate terminal window, run ./osmo_localnet.sh
* in a separate terminal window, run ./run_hermes.sh
* wait for "Hermes has started" message
* In a separate terminal window, run ./deploy_and_exec_contract.sh 
    * You will have to run set up every time you restart the chains + hermes
    * If you keep the chains running, you'll only have to run set up the first time