#!/bin/bash

stampPath=$(pwd)/$1
cliPass=$2
stampFiles=$(ls $stampPath)
line=document_id
for i in $stampFiles; do
    ./constata-cli-linux --password $cliPass stamp $stampPath/"$i" | ( grep "$line" -m 1 | sed 's/"document_id"://' | tr -d '\n "'; echo "$i" ) >> $1.csv;
done
