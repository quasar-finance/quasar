#!/bin/bash 
#set -o xtrace
: '
# Samples 
num_channels=`quasard q ibc channel channels --node tcp://localhost:26659 -o json | jq ".channels | length"`
state=`quasard q ibc channel channels --node tcp://localhost:26659 -o json | jq ".channels[0].state" | tr -d '"'`
port_id=`quasard q ibc channel channels --node tcp://localhost:26659 -o json | jq ".channels[0].port_id" | tr -d '"'`
channel_id=`quasard q ibc channel channels --node tcp://localhost:26659 -o json | jq ".channels[0].channel_id" | tr -d '"'`
cp_port_id=`quasard q ibc channel channels --node tcp://localhost:26659 -o json | jq ".channels[0].counterparty.port_id" | tr -d '"'`
cp_channel_id=`quasard q ibc channel channels --node tcp://localhost:26659 -o json | jq ".channels[0].counterparty.channel_id" | tr -d '"'`
cp_chain_id=`quasard q ibc channel client-state transfer channel-0 --node tcp://localhost:26659 -o json | jq ".client_state.chain_id" | tr -d '"'`

echo $num_channels
echo "state | port_id | channel_id | cp_port_id | cp_channel_id "
echo "$state | $port_id | $channel_id | $cp_port_id | $cp_channel_id "
' 
test_error() {
if [ "$1" = "" ] 
then
  echo "Previous command did not run properly. Exiting."
  exit 1 
fi 
}


binary_name=""
grpc_port=""
declare -a binary_arr=("quasard" "osmosisd" "gaiad")
#declare -a binary_arr=("quasard")

for b in "${binary_arr[@]}"
do  
	binary_name=$b
	echo " "
	echo "### $binary_name ############################"
	case "$binary_name" in
	"quasard") grpc_port="26659"
	;;
	"osmosisd") grpc_port="26679"
	;;
	"gaiad") grpc_port="26669" 
	;;
	esac
    echo " "
	echo "binary_name=$binary_name"
	echo "grpc_port=$grpc_port"

	# Get the list of channel info under a connection-id
	# quasard q ibc channel connections connection-1  --node tcp://localhost:26659 -o json | jq
	# Get the number of channel under a connection-id
	#num=`$binary_name q ibc channel connections connection-1  --node tcp://localhost:$grpc_port -o json | jq ".channels | length"`
	declare -a conn_arr=("connection-0" "connection-1" "connection-2")
	for conn in "${conn_arr[@]}"
	do
		if [[ "$conn" == "connection-2" && "$binary_name" != "quasard" ]]
		then
          continue   
		fi
		 
		echo " " 
		echo "### $conn ##########################"
		nc=`$binary_name q ibc channel connections $conn  --node tcp://localhost:$grpc_port -o json | jq ".channels | length" | tr -d '"'`
        test_error $nc

		ibc_client_id=`$binary_name q ibc connection end $conn --node tcp://localhost:$grpc_port -o json | jq '.connection.client_id' | tr -d '"'` 
		test_error $ibc_client_id

		ibc_cp_chainid=`$binary_name q ibc client state $ibc_client_id --node tcp://localhost:$grpc_port  -o json | jq ".client_state.chain_id" | tr -d '""'`
		test_error $ibc_cp_chainid		

		ibc_cp_client_id=`$binary_name q ibc connection end $conn --node tcp://localhost:$grpc_port  -o json | jq ".connection.counterparty.client_id" | tr -d '""'`
		test_error $ibc_cp_client_id

		ibc_cp_conn_id=`$binary_name q ibc connection end $conn --node tcp://localhost:$grpc_port  -o json | jq ".connection.counterparty.connection_id" | tr -d '""'`
		test_error $ibc_cp_conn_id
		
		for (( i=0; i < $nc; ++i ))
		do
			port_id=`$binary_name q ibc channel connections $conn --node tcp://localhost:$grpc_port -o json | jq ".channels[$i].port_id" | tr -d '"'`
			test_error $port_id

			channel_id=`$binary_name q ibc channel connections $conn  --node tcp://localhost:$grpc_port -o json | jq ".channels[$i].channel_id" | tr -d '"'`
			test_error $channel_id

			cp_port_id=`$binary_name q ibc channel connections $conn  --node tcp://localhost:$grpc_port -o json | jq ".channels[$i].counterparty.port_id" | tr -d '"'`
			test_error $cp_port_id

			cp_channel_id=`$binary_name q ibc channel connections $conn  --node tcp://localhost:$grpc_port -o json | jq ".channels[$i].counterparty.channel_id" | tr -d '"'`
			test_error $cp_channel_id

			state=`$binary_name q ibc channel channels --node tcp://localhost:$grpc_port -o json | jq ".channels[$i].state" | tr -d '"'`
			test_error $state

			echo "### $channel_id #############################"	
			echo "number_of_channels=$nc"
			echo "connection_id=$conn"	
			echo "client_id=$ibc_client_id"
			echo "port_id=$port_id"
			echo "channel_id=$channel_id"
			echo "cp_chainid=$ibc_cp_chainid"
			echo "cp_connection_id=$ibc_cp_conn_id"
			echo "cp_client_id=$ibc_cp_client_id"
			echo "cp_port_id=$cp_port_id"
			echo "cp_channel_id=$cp_channel_id"
			echo "channel_state=$state"
		done # channel
	done # connection
done # binary 
