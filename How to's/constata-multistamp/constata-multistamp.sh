#!/bin/bash

stampPath=$(pwd)/$1
cliPass=$2
ls $stampPath | while read -r FILE; do [[ "$FILE" == *" "* ]] && mv $stampPath/"$FILE" $stampPath/"${FILE// /_}" ; done
line=document_id
stampFiles=$(ls $stampPath)
total=$(echo $stampFiles | wc -w)
count=1

for i in $stampFiles; do
  printf "\rProcesando %5d/%-5d %-60s" $count $total $i
  ./constata-cli-linux --password $cliPass stamp $stampPath/"$i" | ( grep "$line" -m 1 | sed 's/"document_id"://' | tr -d '\n "'; echo "$i" ) >> $1.csv;
  let "count=count+1"
done
printf "\nFinalizado\n"
