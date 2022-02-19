#!/bin/bash

csvPath=$(pwd)/$1
cliPass=$2
folder=certificados_$(date +%d-%m-%Y-%T)
total=$(cat $csvPath | wc -l)
count=1

if [ -f $csvPath ]; then
	echo "Archivo csv localizado"
else
	echo "La ruta ingresada no es correcta o no existe el archivo"
  exit 1
fi

while IFS=, read col1 col2
do
	state=$(./constata-cli-linux --password $cliPass show ${col1} | ( grep state -m 1 | sed 's/"state"://' | tr -d '\n ",'))
	if [ $state == "published" ]; then
		[ $count -eq 1 ] && mkdir $folder && echo "Directorio de destino:" $folder
		printf "\rProcesando %5d/%-5d %-60s" $count $total "Documento: "${col2}
		./constata-cli-linux --password $cliPass fetch-proof ${col1} > $folder/"${col2}".html
		let "count=count+1"
	else
		printf "\rDocumento NO publicado, intentelo más tarde | %-60s " ${col2}
	fi
done < $csvPath
printf "\nFinalizado\n"
