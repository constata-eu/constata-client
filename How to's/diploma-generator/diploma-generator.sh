#!/bin/bash

diplomaGenerator () {
  folder=diplomas_$(date +%d-%m-%Y)
  mkdir $folder
  while IFS="," read name certName reason dateOf site
  do
    sed -e "s|NAME|$name|g" -e "s|CERTNAM|$certName|" -e "s|REASON|$reason|" -e "s|DATEOF|$dateOf|" -e "s|SITE|$site|" template.html > "${name}".html
    wkhtmltoimage -q --width 700 --quality 90 "${name}".html $folder/"${name}".jpg
    rm "${name}".html
  done < alum.csv
}

which wkhtmltoimage &> /dev/null
if [ $? -ne 0 ];
  then
    echo "Es necesario instalar wkhtmltoimage"
    echo "Puedes descargarlo desde https://wkhtmltopdf.org/downloads.html"
    exit 1
  else
    diplomaGenerator
fi
