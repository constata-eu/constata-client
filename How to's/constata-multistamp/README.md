# Firmar digitalmente y sellar de tiempo varios documentos

Es posible que quieras firmar y sellar varios documentos en un único proceso, por ejemplo si perteneces a una institución educativa sellando los diplomas de todos tus alumnos.

Para eso te ofrecemos este script constata-multistamp, escrito en *bash* (probado en linux/mac) que puedes ver y descargar de este mismo repositorio.

**Nos interesa saber como estás usando nuestro API y como podemos facilitarte el trabajo. Escríbenos sin compromiso a <a href="mailto:hola@constata.eu">hola@constata.eu</a>**

### Requisitos previos

- [Descargar y ejecutar constata-cli](https://github.com/constata-eu/constata-client), para crear tu firma digital y darte de alta en nuestro API.

- Un directorio con archivos que quieras sellar. Si eres de una institución educativa y todavía no sabes como generar los diplomas para tus alumnos, te recomendamos [ver nuestro generador de diplomas](https://github.com/constata-eu/constata-client/tree/main/How%20to's/diploma-generator)

***

### Pedido de firma y sellado

1) Copia o descarga el script constata-multistamp.sh que se encuentra en este directorio.

2) Guárdalo en el mismo directorio en el que se aloja constata-cli-linux.

3) Otórgale permisos de ejecución:

`chdmod +x constata-multistamp.sh`

Para su ejecución, debes pasarle dos argumentos en orden: en primer lugar, el directorio en el que se encuentran todos los documentos a sellar; en segundo lugar, tu password de constata-cli.

    ./constata-multistamp.sh DIRECTORIO PASSWORD

Veamos un ejemplo. Para evitar rutas absolutas sumamente largas, recomendamos copiar el directorio que contiene los documentos a sellar en el mismo directorio desde el que ejecutas constata-cli. Si nuestro directorio se llama "diplomas", entonces el comando de ejecución es:

`./constata-multistamp diplomas mipassword`

Con este comando ya se envían a sellar todos los archivos de tu directorio diplomas, ahora solo queda esperar aproximadamente una hora, hasta que todos reciban el sello de tiempo.

Cada archivo que enviaste es interpretado por constata como un documento al que le asignamos un identificador único. El script constata-multistamp genera un archivo 'csv' (diplomas.csv en este ejemplo) donde lista todos los identificadores únicos de cada documento asociados al nombre de archivo original que enviaste a sellar. El siguiente paso utiliza este archivo csv para descargar todos los certificados de sello de tiempo usando el API de constata.

Puedes usar el identificador de cada documento para consultar su estado y otros datos relacionados usando:
`constata-cli show <identificador del documento>`

También uedes constular el listado de los documentos que hayas enviado a constata históricamente usando
`constata-cli list`

***

### Luego, descarga de los sellos.

Aproximadamente 90 minutos después de enviar tus archivos, los documentos ya deberían haber recibido su sello de tiempo.

Si intentas ejecutar este script antes de que todos reciban el sello de tiempo, verás un mensaje pidiéndote esperar más.

1) Copia o descarga el script constata-multiproof.sh que se encuentra en este directorio.

2) Guárdalo en el mismo directorio en el que se aloja constata-cli-linux.

3) Otórgale permisos de ejecución:

`chdmod +x constata-multiproof.sh`

Para su ejecución, debes pasarle dos argumentos en orden: en primer lugar, archivo .csv en el que se encuentran los datos de los documentos (document_id y nombre). Aquí utilizarás el csv que generó constata-multistamp.sh y fue almacenado en el mismo directorio en el que ejecutaste constata-multistamp; en segundo lugar, tu password de constata-cli:

    ./constata-multiproof.sh CSV PASSWORD

Veamos un ejemplo. Para evitar rutas absolutas sumamente largas, recomendamos copiar el directorio que contiene los documentos a sellar en el mismo directorio desde el que ejecutas constata-cli. Si nuestro archivo csv se llama "diplomas.csv", entonces el comando de ejecución es:

`./constata-multiproof.sh diplomas.csv mipassword`

Observarás que en el mismo directorio se creo una carpeta denominada *proofs_DDMMAAAA* que contiene los certificados *.html* de tus documentos sellados.  

Esos archivos HTML generados son los certificados de sello de tiempo con el diploma de cada alumno. Puedes enviarle a cada uno su certificado html adjuntándolo en un correo electrónico, por telegram, compartírselo en google drive, o lo que prefieras.

El certificado de sello de tiempo contiene todos los documentos originales y la información necesaria para validar el sello recibido en cualquier momento del futuro. Es una cápsula de tiempo de información digital que tiene plena validez por si misma, y recomendamos que cada interesado guarde una copia.
