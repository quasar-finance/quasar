#!/bin/sh 


qpid=`ps -ef | grep quasard | grep -v "grep" |  awk '{ printf $2 }'`
if [ -z "$qpid" ]
then
      echo "quasard not running."
else
      echo "quasard is running with process id $qpid - killing"
 	  pkill quasard
fi

opid=`ps -ef | grep osmosisd | grep -v "grep" |  awk '{ printf $2 }'`
if [ -z "$opid" ]
then
      echo "osmosisd not running"
else
      echo "osmosisd is running with process id $opid - killing"
	  pkill osmosisd
fi

gpid=`ps -ef | grep "gaiad" | grep -v "grep" |  awk '{ printf $2 }'`
if [ -z "$gpid" ]
then
      echo "gaiad not running"
else
      echo "gaiad is running with process id $gpid - killing"
	  pkill gaiad
fi


