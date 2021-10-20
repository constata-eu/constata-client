#!/bin/bash

csvPath=$(pwd)/$1
cliPass=$2
folder=html_proofs_$(date +%d-%m-%Y)
mkdir $folder


	if [ -f $csvPath ];
	then
	        echo "Archivo csv localizado"
	else
        	echo "La ruta ingresada no es correcta o no existe el archivo"
	        exit 1
	fi

	while IFS=, read col1 col2
	do
		echo ${col1}
		./constata-cli-linux --password $cliPass fetch-proof ${col1} > $folder/"${col2}".html
	done < $csvPath
