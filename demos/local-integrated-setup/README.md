# Description

This guide shows how to set up a local integrated environment with quasar, osmosis and gaia chains running in the local machine of developers. 

## Clone and compile the chain binary
In this step you need to clone the chain source code from the right repository with your choice of branch or tag. 

This guide shows to clone from their projects main repository except gaia and main branch. 

1. Clone and compile quasar 
```
git clone git@github.com:bandprotocol/chain.git band
cd band
make install
``` 

2. Clone and compile osmosis
   
```
git clone git@github.com:osmosis-labs/osmosis.git
cd osmosis
make install
```

3. Clone and compile gaia

```
git clone git@github.com:quasar-finance/gaia.git -b bugfix/replace_default_transfer_with_router_module
cd gaia
make install
```

4. Clone and compile band protocol
```
git clone git@github.com:bandprotocol/chain.git band
cd band
make install 
```


## Clean the environment
Run the following command to clean the environment from previous state.
```
./kill_n_clean.sh
```
Check the running status command.
```
./running_status.sh
```

## Start the chains with gen tx procedure
```
./run_all_chains_with_gentx.sh
``` 
Check the running status command.
```
./running_status.sh
```
## Establish the ibc connections 

To check the hermes configurations please go to the hermes/v[0/1] directory and verify that the hermes configuration is as per your expectations. 


### Option #1 
If you want quasar to connect to band chain also for getting stable prices from the band. Please not that the v0 and v1 in the script name for the specific version of hermes that you are using.  If you are using v0.y.z version, use the script whch has v0 in its name, if you are using v1.y.z version, use the script with v1 in its name. 

``` 
cd hermes 
./run_hermes_v[0/1]_withbandchain.sh 
```

### Option #2 
If you don't want quasar to connect to bandchain, maybe because your testing use case don't require stable prices.
``` 
cd hermes 
./run_hermes_v[0/1]_nobandchain.sh
```

### Check the ibc connection among chains 

You can use below two scrips for output the connection and channel mappings among the chains. 
Use the band chain version if you are using the band chain too. The script will exit if something is wrong with the connection or the connection or channel are not yet established. 

It works well when connection and channels are established properly. 
```
./ibc_connection_status_with_bandchain.sh
./ibc_connection_status.sh
./ibc_channel_status.sh
./ibc_channel_status_with_bandchain.sh
```
