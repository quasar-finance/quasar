#!/bin/bash 
#set -o xtrace 

# This script will output the connection-id, client-id and chain-id mapping fo the local testing environment. 
# This is very useful to know which connections belongs to which counter party chain
# Note - A generic script with configurations can be created for the production monitoring and analysis."
declare -a conn_arr=("connection-0" "connection-1" "connection-2")

test_error() {
if [ "$1" = "" ] 
then
  echo "Previous command did not run properly. Exiting."
  exit 1 
fi 
}

echo " "
echo "QUASAR CONNECTIONS "
for i in "${conn_arr[@]}"
do
   echo "##########################"
   echo "connection-id - $i"
   ibc_q_client_id=`quasard q ibc connection end $i --node tcp://localhost:26659 -o json | jq '.connection.client_id' | tr -d '"'`
   echo "client id - $ibc_q_client_id"
   test_error $ibc_q_client_id
   ibc_q_cp_chainid=`quasard q ibc client state $ibc_q_client_id --node tcp://localhost:26659  -o json | jq ".client_state.chain_id" | tr -d '""'`
   test_error $ibc_q_cp_chainid
   echo "cp chain id - $ibc_q_cp_chainid"
done

declare -a conn_arr=("connection-0" "connection-1")
echo " "
echo "OSMOSIS CONNECTIONS "
for i in "${conn_arr[@]}"
do
   echo "##########################"
   echo "connection-id - $i"
   ibc_q_client_id=`osmosisd q ibc connection end $i --node tcp://localhost:26679 -o json | jq '.connection.client_id' | tr -d '"'`
   test_error $ibc_q_client_id
   echo "client id - $ibc_q_client_id"
   ibc_q_cp_chainid=`quasard q ibc client state $ibc_q_client_id --node tcp://localhost:26679  -o json | jq ".client_state.chain_id" | tr -d '""'`
   test_error $ibc_q_cp_chainid
   echo "cp chain id - $ibc_q_cp_chainid"
done

echo " "
echo "COSMOS CONNECTIONS "
for i in "${conn_arr[@]}"
do
   echo "##########################"
   echo "connection-id - $i"
   ibc_q_client_id=`osmosisd q ibc connection end $i --node tcp://localhost:26669 -o json | jq '.connection.client_id' | tr -d '"'`
   test_error $ibc_q_client_id
   echo "client id - $ibc_q_client_id"
   ibc_q_cp_chainid=`quasard q ibc client state $ibc_q_client_id --node tcp://localhost:26669  -o json | jq ".client_state.chain_id" | tr -d '""'`
   test_error $ibc_q_cp_chainid
   echo "cp chain id - $ibc_q_cp_chainid"
done
