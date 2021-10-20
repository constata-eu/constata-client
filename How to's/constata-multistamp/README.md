# Casos de Uso

Si deseas sellar varios documentos, puede resultar una tarea repetitiva en la que ocupas mucho de tu tiempo. Por ejemplo, si perteneces a una institución educativa y necesitas sellar diplomas o certificados de tus cursos para un número considerable de personas.
En ese caso, puedes utilizar nuestro script constata-multistamp. Se trata de un sencillo script escrito en *bash* que puedes <a href="#!">descargar de nuestro repositorio</a>.  
**Si necesitas asistencia para la integración de nuestro servicio a tu proyecto, contáctate con nosotros.**

***

### Firma múltiples documentos

1) Copia o descarga el script constata-multistamp.sh que se encuentra en este directorio.

2) Guárdalo en el mismo directorio en el que se aloja constata-cli-linux.

3) Otórgale permisos de ejecución:

`chdmod +x constata-multistamp.sh`

Para su ejecución, debes pasarle dos argumentos en orden: en primer lugar, el directorio en el que se encuentran todos los documentos a sellar; en segundo lugar, tu password de constata-cli.

    ./constata-multistamp.sh [DIRECTORIO] [PASSWORD]

Veamos un ejemplo. Para evitar rutas absolutas sumamente largas, recomendamos copiar el directorio que contiene los documentos a sellar en el mismo directorio desde el que ejecutas constata-cli. Si nuestra carpeta se llama "diploma", entonces el comando de ejecución es:

`./constata-multistamp diplomas mipassword`

Ya habrás enviado a sellar todos los documentos de tu directorio diplomas. Puedes revisarlo con el subcomando list. Además se ha generado un csv, en este ejemplo diplomas.csv, que contiene el nombre del archivo junto a su document_id y puedes consultar particularmente cada documento gracias al comando show. Este csv creado, podrás utilizarlo para descargar todos los certificados html en un solo para, como se explica en el siguiente punto.

***

### Descarga múltiples certificados

1) Copia o descarga el script constata-multiproof.sh que se encuentra en este directorio.

2) Guárdalo en el mismo directorio en el que se aloja constata-cli-linux.

3) Otórgale permisos de ejecución:

`chdmod +x constata-multiproof.sh`


Para su ejecución, debes pasarle dos argumentos en orden: en primer lugar, archivo .csv en el que se encuentran los datos de los documentos (document_id y nombre). Aquí utilizarás el csv que generó constata-multistamp.sh y fue almacenado en el mismo directorio en el que ejecutaste constata-multistamp; en segundo lugar, tu password de constata-cli:

    ./constata-multiproof.sh [CSV] [PASSWORD]

Veamos un ejemplo. Para evitar rutas absolutas sumamente largas, recomendamos copiar el directorio que contiene los documentos a sellar en el mismo directorio desde el que ejecutas constata-cli. Si nuestro archivo csv se llama "diplomas.csv", entonces el comando de ejecución es:

`./constata-multiproof.sh diplomas.csv mipassword`

Observarás que en el mismo directorio se creo una carpeta denominada *proofs_DDMMAAAA* que contiene los certificados *.html* de tus documentos sellados.
**¡IMPORTANTE! Recuerda asegurarte de que el estado de los documentos sellados sea *published*, de lo contrario deberás esperar a que sean publicados para ejecutar constata-multiproof.sh satisfactoriamente**
