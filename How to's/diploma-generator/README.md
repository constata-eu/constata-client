# Generador de diplomas

Si deseas generar diplomas de forma masiva, convertirlos en imágenes y luego sellarlos utilizando nuestro script [constata-multistamp.sh](https://github.com/constata-eu/constata-client/tree/main/How%20to's/constata-multistamp), puedes utilizar *diploma-generator.sh* para ello.

**Si necesitas asistencia para la integración de nuestro servicio a tu proyecto, contáctate con nosotros.**


#### Cómo utilizarlo
Solo necesitas descargar los tres archivos contenidos en esta carpeta.


Además del script, *diploma-generator.sh*, cuentas con *alum.csv*. Este es un ejemplo de las entradas necesarias en el csv para generar los diplomas.
Debes copiar allí los datos de las personas a quienes les correspondan los diplomas.
Cuenta con cinco entradas:
Nombre completo	| Nombre del certificado	| Motivo o Curso realizado |	Fecha |	Lugar

Por lo que el orden del csv es el siguiente:
Satoshi Nakamoto, Certificado de Reconocimiento, Completar el curso de BTC, 03/01/2009, Online
Andreas Antonopoulos, Certificado de Reconocimiento a la Difusión, Difundir conocimiento, 08/08/2028, Metaverso

Cada línea representa la información de una persona distinta y las comas (,) delimitan cada dato.  
**Es muy importante separar los datos con una coma (,)**

#### Plantilla html (template.html)
También observarás el archivo *template.html*. Se trata de una plantilla en html a modo de ejemplo. Puedes crear tu propia plantilla en html,
para ello debes incluir las palabras reservadas que se reemplazarán por los datos contenidos en el archivo *alum.csv*:

Palabras reservadas (en MAYÚSCULAS): dato por el que será reemplazada

*NAME: Nombre completo*  
*CERTNAM, Nombre del certificado*  
*REASON, Motivo o Curso realizado*  
*DATEOF, Fecha*  
*SITE, Lugar*

#### Ejecutar diploma-generator.sh
Si ya tuviste en cuenta la función de los archivos alum.csv y template.html, es momento de ejecutar el script diploma-generator.sh

#### Le otorgamos permisos de ejecución

`chdmod +x diploma-generator.sh`

#### Lo ejecutas

./diploma-generator.sh

##### Necesitas la herramienta *wkhtmltoimage* para que el script convierta los diplomas generados a imágenes  
Si no la tienes instalada, *diploma-generator.sh* te mostrará el siguiente mensaje:

    Es necesario instalar wkhtmltoimage
    Puedes descargarlo desde https://wkhtmltopdf.org/downloads.html

En https://wkhtmltopdf.org/downloads.html encontrarás el paquete necesario para tu versión de GNU/Linux.

##### Si ya cuentas con la herramienta *wkhtmltoimage* instalada
*diploma-generator.sh* generará una carpeta denominada *diplomas_DD-MM-AAAA* con las imágenes de los diplomas de cada una de las personas en el listado *alum.csv*.
