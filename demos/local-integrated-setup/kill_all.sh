#!/bin/sh 


qpid=`ps -ef | grep quasarnoded | grep -v "grep" |  awk '{ printf $2 }'`
if [ -z "$qpid" ]
then
      echo "quasarnoded not running."
else
      echo "quasarnoded is running with process id $qpid - killing" 
 	  pkill quasarnoded  
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


