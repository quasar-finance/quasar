#!/bin/sh

set -o xtrace 


sleep 3 

qpid=`ps -ef | grep quasard | grep -v "grep" |  awk '{ printf $2 }'`
if [ -z "$qpid" ]
then
      echo "quasard not running"
else 
      echo "quasard is running with process id $qpid"
fi

opid=`ps -ef | grep osmosisd | grep -v "grep" |  awk '{ printf $2 }'`
if [ -z "$opid" ]
then
      echo "osmosisd not running"
else
      echo "osmosisd is running with process id $opid"
fi

gpid=`ps -ef | grep "gaiad" | grep -v "grep" |  awk '{ printf $2 }'`
if [ -z "$gpid" ]
then
      echo "gaiad not running"
else
      echo "gaiad is running with process id $gpid"
fi

if [ ! -z "$qpid"  ] || [ ! -z "$opid" ] || [ ! -z "$gpid" ]
then
	echo "Some processes are already running."
	exit 1	
fi 

./clean_all_chains.sh

echo "----STARTING CHAIN PROCESSES----"
sleep 3
 
./quasar_localnet.sh
echo "-----------------------------------------"
sleep 3
./osmo_localnet.sh
echo "-----------------------------------------"
sleep 3
./cosmos_localnet.sh
echo "-----------------------------------------"

sleep 10

exit 10
#watch ./pid_running_status.sh

