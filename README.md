# constata-cli
Puedes utilizar constata-client para sellar tus documentos. Se trata de un sello de tiempo (timestamping) almacenado en el blockchain de Bitcoin. Una vez sellado y publicado en la red de Bitcoin, puedes descargar un certificado html de tu documento sellado. Además puedes vincular un sitio web con tu public key.

**Este es nuestro 2° release** y si eres desarrollador **puedes probar nuestro servicio sin costo alguno**. Debés descargar el binario que corresponda a tu Sistema Operativo. Aquí cuentas con un Quickstart, si necesitas más información puedes consultar nuestra [wiki](https://github.com/constata-eu/constata-client/wiki) en donde encontrarás [documentación](https://github.com/constata-eu/constata-client/wiki) de uso más detallada.

#### Segundo release. Soporta Sellado de tiempo desde línea de comandos, obtención de Certificados de sello de tiempo autocontenidos y verificación de Firma digital asociada a un sitio web.
----
#### Second release, supports command line Timestamping, stand-alone Certificate retrieval and Digital signature verification associated with a website.


## Quickstart
### GNU/Linux

1. Descarga el binario "constata-cli-linux" desde esta misma página.

2. Abre una terminal y posiciónate en el directorio en el que se encuentra el binario.
Si lo descargaste en el directorio "Descargas":


` cd Descargas`

3. Otórgale permisos de ejecución al binario:


` chmod +x constata-cli-linux`

4. Ejecuta el binario con la opción **help** para ver las opciones de ejecución:


` ./constata-cli-linux help`

5. Ejecuta la opción **stamp** y luego la ruta/nombre del documento que deseas sellar:


` ./constata-cli-linux stamp DOCUMENTO_A_SELLAR`


_Ten en cuenta que si es la primera vez que lo utilizás, luego de ejecutar este comando te solicitará la creación de tu llave privada. Debés seleccionar "Let's create one now." y seguir los pasos._

6. Ejecuta la opción **fetch-proof** y redirecciona su salida para obtener el **Certificado de Sello de Tiempo** autocontenido en un html:


` ./constata-cli-linux fetch-proof ID_DOCUMENTO_SELLADO > CERTIFICADO_SELLO_DE_TIEMPO.html`


_El ID de tu documento lo puedes observar en la terminal luego de ejecutar la opción **stamp**, también puedes consultarlo con la opción **list**._  

_Ten en cuenta que los documentos a sellar pueden tardar hasta 100'._

7. Abre el html autocontenido con tu navegador web preferido para verificarlo:


`google-chrome CERTIFICADO_SELLO_DE_TIEMPO.html`

Descargas:  

[constata-cli-linux](https://github.com/constata-eu/constata-client/releases/download/rc-2/constata-cli-linux)


[constata-cli-macos](https://github.com/constata-eu/constata-client/releases/download/rc-2/constata-cli-macos)


[constata-cli-win.exe](https://github.com/constata-eu/constata-client/releases/download/rc-2/constata-cli-win.exe)
