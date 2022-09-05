#!/bin/sh

qpid=`ps -auf | grep quasarnoded | grep -v "grep" |  awk '{ printf $2 }'`
if [ -z "$qpid" ]
then
      echo "quasarnoded not running"
else
      echo "quasarnoded is running with process id $qpid"   
fi

opid=`ps -auf | grep osmosisd | grep -v "grep" |  awk '{ printf $2 }'`
if [ -z "$opid" ]
then
      echo "osmosisd not running"
else
      echo "osmosisd is running with process id $opid"
fi

gpid=`ps -auf | grep "gaiad" | grep -v "grep" |  awk '{ printf $2 }'`
if [ -z "$gpid" ]
then
      echo "gaiad not running"
else
      echo "gaiad is running with process id $gpid"
fi

if [ -z "$qpid"  ] || [ -z "$opid" ] || [ -z "$gpid" ]
then
    echo "Some processes are not running."
    exit 1
else 
	echo "All processes are running."
	exit 0
fi









