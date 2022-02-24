# constata-cli
Puedes utilizar constata-client para sellar tus documentos (cuentas con hasta diez tokens mensuales sin costo para hacerlo).

Si necesitas más información puedes consultar nuestra [wiki](https://github.com/constata-eu/constata-client/wiki) en donde encontrarás [documentación](https://github.com/constata-eu/constata-client/wiki) de uso más detallada.

## ¿Qué es?
Se trata de un sello de tiempo (timestamping) almacenado en el blockchain de Bitcoin. Una vez sellado y publicado en la red de Bitcoin, puedes descargar un certificado html de tu documento sellado. Además puedes vincular un sitio web con tu public key.
***
## ¿Cómo lo utilizo?  

1. Descarga el binario

[constata-cli-linux](https://github.com/constata-eu/constata-client/releases/download/rc-3/constata-cli-linux)


[constata-cli-macos](https://github.com/constata-eu/constata-client/releases/download/rc-3/constata-cli-macos)


[constata-cli-win.exe](https://github.com/constata-eu/constata-client/releases/download/rc-3/constata-cli-win.exe)  


Dale permisos de ejecución y ejecútalo

    chmod +x constata-cli
    ./constata-cli

***
2. Crea las credenciales 

Observarás esto en pantalla, debes seguir las instrucciones para generar tu clave privada antes de continuar, debés seleccionar "Let's create one now":


    Constata's API authenticates you using your own private key.  
     This key is never sent to our servers, and is stored encrypted in your drive.  
     We looked here for a config file named constata_conf.json and couldn't find any.  
     If you already have a config file bring it over, otherwise, we can create one now.


    ? What do you want to do? ›  
    ❯ Let's create one now.  
      Exit for now. I'll bring my config over.

***
3. Sella un documento

Para sellar un documento, simplemente ejecutas ./constata-cli-linux stamp *[DOCUMENTO]*  
Por ejemplo:

`./constata-cli stamp mi_certificado.pdf`

Una vez presiones ENTER, te solicitará tu clave habitual

`? Enter your password › `

Luego observarás la siguiente salida:

        {
          "bulletin_id": 36,
          "bulletins": {
            "36": {
              "id": 36,
              "started_at": "2021-10-04T16:44:17.124013Z",
              "state": "Draft"
            }
          },
          "id": "12-978ee01df6b3558105de59de795cfaa5e0acf3fc44c8958d81e77c832bc709cd",
          "parts": [
            {
              "content_type": "application/pdf",
              "document_id": "12-978ee01df6b3558105de59de795cfaa5e0acf3fc44c8958d81e77c832bc709cd",
              "friendly_name": "document.pdf",
              "hash": "978ee01df6b3558105de59de795cfaa5e0acf3fc44c8958d81e77c832bc709cd",
              "id": "59200a46b2e904fa46ca1c7c01a13c56ffd109a0853c84b7be5ece186e5caba7",
              "signatures": [
                {
                  "bulletin_id": 36,
                  "document_part_id": "59200a46b2e904fa46ca1c7c01a13c56ffd109a0853c84b7be5ece186e5caba7",
                  "endorsements": [],
                  "id": 80,
                  "pubkey_id": "1No2jACMbYCxqFuekCBW8DyRBmQnY1fiyd",
                  "signature": "H/nDa0nMrZ5gshoRRhvK8sUMcKTBcmUpXuHxNozBzO8OLsB8p1xrQgfcYt8VBRZvxaM0sceaVX99zSptFen4WFg=",
                  "signature_hash": "a6626bfdfa86a861e619bf27876a60ade1b7082bf33fc05d6c2524dc44431e27"
                }
              ],
              "size_in_bytes": 1815374
            }
          ],
          "person_id": 12
          "state": "parked"
        }

Aquí hay dos líneas muy importantes: "state" y "document_id". Puedes observar "state": "Draft", esto significa que el boletín en el que está incluido el sellado de tu documento aún no ha sido publicado y se encuentra como draft (borrador). Luego cambiará a submitted (enviado) y, finalmente, será publicado y podrás observar "state: published".

"document_id" es la cadena de caracteres que identifica a tu documento, puedes observar en nuestra salida de ejemplo "document_id": "12-978ee01df6b3558105de59de795cfaa5e0acf3fc44c8958d81e77c832bc709cd". Nos será de utilidad para hacer consultas sobre el estado de este documento a través del subcomando show de la siguiente forma.

        ./constata-cli show DOCUMENT_ID

***
4. Descarga el certificado

Los certificados de Constata se descargan en un html autocontenido.

Esto significa que el certificado html que descargas contiene el documento certificado
(puedes acceder a este sin conexión a Internet) y además lo valida consultando a
la API de Constata (con conexión a Internet).

Para descargar un certificado utilizas el subcomando fetch-proof seguido del "document_id" y rediriges la salida hacia el nombre que deseas para tu certificado html:

*./constata-cli fetch-proof DOCUMENT_ID > NOMBRE_DEL_CERTIFICADO.html*

Por ejemplo,

`./constata-cli fetch-proof 12-978ee01df6b3558105de59de795cfaa5e0acf3fc44c8958d81e77c832bc709cd > mi_certificado.html`


Abre el html autocontenido con tu navegador web preferido para verificarlo:

`google-chrome NOMBRE_DEL_CERTIFICADO.html`

***
5. Si utilizas todos tus tokens disponibles, al hacer stamp verás en la salida una url en el campo buy_tokens_link. Puedes utilizarla para comprar más tokens. Como ejemplo, puede apreciarse en la última línea de la siguiente salida:

       {
         "bulletin_id": 37,
         "bulletins": {
           "37": {
             "block_hash": "00000000000000313c2faa76d2087540654df4a93e3c5ea30320810a4d41f956",
             "block_time": "2022-02-02T19:31:17Z",
             "hash": "67884d229142e7dcd7301f90c8ce575833cef185b4f0175f0d9d3037f38cb586",
             "id": 37,
             "started_at": "2022-02-02T12:34:44.468338Z",
             "state": "Draft",
             "transaction": "020000000001018a3c4ee3823ddb32fde61415b98d33c4f8d2a89582c01d0313f001c5c9ae35160100000000ffffffff022202000000000000226a2067884d229142e7dcd7301f90c8ce575833cef185b4f0175f0d9d3037f38cb58686c20c0000000000160014e0d1765c2746af9a5e0c7e67a9d0a3c996971ece024830450221008e0ea2dd656be775e48eb7e8c106749d36dcdd65fcce99dc02d1564709dd90d902205837b0ce62b0bf1603344c1ecbedff73757e7c7a6c8a4947ab3ee8275a52aff80121023afe241c5a425dd8db699b52e4fb35bd35da00cea4be8304907b9e9c537225ef00000000",
             "transaction_hash": "bc763ee170dbb16ea1fadfec27bd22071baa7184fec88fbb6b054bf5388ae429"
           }
         },
         "buy_tokens_link": "https://api.constata.eu/invoices/#link_token=urologist+envelope+capped+mutt+unelected&minimum_suggested=0",
  
