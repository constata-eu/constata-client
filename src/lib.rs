pub mod signature;
pub mod signed_payload;

use signature::Signature;

use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_json::Number;
use serde_json::Value;
use bitcoin::PublicKey;

use dialoguer::console::{Emoji, style};

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  SignerError(#[from] bitcoin_wallet::error::Error),
  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[error(transparent)]
  Json(#[from] serde_json::Error),
  #[error(transparent)]
  DerivationError(#[from] bitcoin::util::bip32::Error),
  #[error(transparent)]
  FromUtf8Error(#[from] std::string::FromUtf8Error),
  #[error(transparent)]
  PrivateKey(#[from] bitcoin::util::key::Error),
  #[error(transparent)]
  Network(#[from] Box<ureq::Error>),
  #[error("An error ocurred when encrypting or decrypting the daily key")]
  DailyKeyEncriptionError,
  #[error("The decrypted signing key does not match the expected one")]
  ConfigKeyMismatch,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Serialize, Deserialize)]
struct AccountState {
  missing: String,
  parked_count: Number,
  person_id: Number,
  token_balance: String,
  total_document_count: Number,
}

#[derive(Serialize, Deserialize)]
struct DocumentBundle {
  bulletins: Value,
  cost: String,
  created_at: String,
  gift_id: Value,
  id: String,
  parts: Value,
  person_id: Number,
  state: String,
  buy_tokens_link: Value,
}

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct Config {
  public_key: PublicKey,
  #[serde_as(as = "serde_with::hex::Hex")]
  encrypted_key: Vec<u8>,
  environment: String,
}

pub struct Client {
  signature: Signature,
  api_url: String,
}

/* The Client knows about managing local secrets, the local filesystem,
 * and making signed requests to the public API.
 * It can be used by client libraries or by the command line utility.
 * These methods should ideally be FFI able.
 */
impl Client {
  pub fn create(
    custom_config: Option<&str>,
    env: Option<&str>,
    backup_pass: &str,
    daily_pass: &str,
  ) -> Result<Vec<String>> {
    let (config, mnemonic) =
      Signature::create(env.unwrap_or("production"), backup_pass, daily_pass)?;
    std::fs::write(
      Self::config_path(custom_config),
      serde_json::to_string(&config)?,
    )?;
    Ok(mnemonic.iter().map(|x| x.into()).collect())
  }

  pub fn config_needed(custom_config: Option<&str>) -> bool {
    !std::path::Path::new(Self::config_path(custom_config)).exists()
  }

  pub fn config_path(custom_config: Option<&str>) -> &str {
    custom_config.unwrap_or("constata_conf.json")
  }

  pub fn load(custom_config: Option<&str>, daily_passphrase: &str) -> Result<Client> {
    let stored: Config =
      serde_json::from_str(&std::fs::read_to_string(Self::config_path(custom_config))?)?;
    let api_url = match stored.environment.as_str() {
      "staging" => "https://api-staging.constata.eu",
      "production" => "https://api.constata.eu",
      _ => "http://localhost:8000",
    }
    .to_string();
    let signature = Signature::load(stored, daily_passphrase)?;

    ureq::post(&format!("{}/signup", api_url))
      .send_json(ureq::json!({
        "signed_payload": signature.sign_message(b"Hello Constata.eu"),
      }))
      .unwrap()
      .into_string()?;

    Ok(Client { signature, api_url })
  }

  pub fn sign_and_timestamp(&self, bytes: &[u8]) -> Result<String> {
    let response: serde_json::Value = ureq::post(&format!("{}/documents/", self.api_url))
      .send_json(ureq::json!({
        "signed_payload": self.signature.sign_message(&bytes),
      }))
      .map_err(Box::new)?
      .into_json()?;
    Ok(serde_json::to_string_pretty(&response)?)
  }

  pub fn verify_website(&self, website: &[u8]) -> Result<(String, String)> {
    let signed_payload = self.signature.sign_message(&website);
    let response: serde_json::Value = ureq::post(&format!("{}/pubkey_domain_endorsements/", self.api_url))
      .send_json(ureq::json!({
        "signed_payload": &signed_payload,
      }))
      .map_err(Box::new)?
      .into_json()?;
    Ok((serde_json::to_string_pretty(&response)?, signed_payload.signature.to_string()))
  }

  pub fn website_verifications(&self) -> Result<String> {
    self.get_json("/pubkey_domain_endorsements")
  }

  pub fn get_response(&self, url: &str) -> Result<ureq::Response> {
    let payload = ureq::json![{
      "constata_eu_action": url,
      "expires": chrono::offset::Utc::now() + chrono::Duration::hours(100),
    }]
    .to_string();

    let auth_token = serde_json::to_string(&self.signature.sign_message(payload.as_bytes()))?;

    ureq::get(&format!("{}{}", self.api_url, url))
      .set("Authentication", &auth_token)
      .call()
      .map_err(|e| Box::new(e).into())
  }

  pub fn get(&self, url: &str) -> Result<String> {
    Ok(self.get_response(url)?.into_string()?)
  }

  pub fn get_json(&self, url: &str) -> Result<String> {
    let response: serde_json::Value = self.get_response(url)?.into_json()?;
    Ok(serde_json::to_string_pretty(&response)?)
  }

  pub fn sign_and_timestamp_path(&self, path: &str) -> Result<String> {
    let file_path = match std::fs::read(path) {
      Ok(res) => res,
      Err(ref e) if e.raw_os_error() == Some(21) => {
        eprintln!("\n {} {} is a directory. Stamping could only be applied on files.\n   If you want to stamp an entire directory, consider compress it into a zip file\n", Emoji("🚨", "*"), style(path).bold().bright());
        std::process::exit(1); // Exit with code 1 (fail)
      },
      Err(ref e) if e.raw_os_error() == Some(2) => {
        eprintln!("\n {} File not found using path {}\n", Emoji("🚨", "*"), style(path).bold().bright());
        std::process::exit(1); // Exit with code 1 (fail)
      },
      Err(err) => return Err(err.into()),
    };
    self.sign_and_timestamp(&file_path)
  }

  pub fn documents(&self) -> Result<String> {
    self.get_json("/documents")
  }

  pub fn document(&self, document_id: &str) -> Result<String> {
    let response: DocumentBundle = self.get_response(&format!("/documents/{}", document_id))?.into_json()?;
    Ok(serde_json::to_string_pretty(&response)?)
  }

  pub fn fetch_proof(&self, document_id: &str) -> Result<String> {
    self.get(&format!("/documents/{}/html_proof", document_id))
  }

  pub fn fetch_each_proof(&self, document_id: &str) -> Result<Vec<u8>> {
    use std::io::Read;
    let response =
      self.get_response(&format!("/documents/{}/each_part_html_proof", document_id))?;
    let mut bytes: Vec<u8> = vec![];
    response
      .into_reader()
      .take(10_000_000)
      .read_to_end(&mut bytes)?;
    Ok(bytes)
  }

  pub fn account_state(&self) -> Result<String> {
    let response: AccountState = self.get_response("/account_state")?.into_json()?;
    Ok(serde_json::to_string_pretty(&response)?)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use mockito;

  #[test]
  fn account_state_response() {
    let (config, _mnemonic) = Signature::create("production", "very_secret", "not_so_secret").unwrap();
    let signature = Signature::load(config, "not_so_secret").unwrap();

    let api_url = mockito::server_url();
    let client = Client { signature, api_url };
    let mock = mockito::mock("GET", "/account_state")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"invoices": [], "missing": "1", "parked_count": 1, "person_id": 19, "token_balance": "0", "total_document_count": 367}"#)
        .expect(1)
        .create();

    let json_response = client.account_state().unwrap();

    assert_eq!(
      json_response,
r#"{
  "missing": "1",
  "parked_count": 1,
  "person_id": 19,
  "token_balance": "0",
  "total_document_count": 367
}"#.to_string()
    );

    mock.assert();
  }
  
  #[test]
  fn document_response() {
    let (config, _mnemonic) = Signature::create("production", "very_secret", "not_so_secret").unwrap();
    let signature = Signature::load(config, "not_so_secret").unwrap();

    let api_url = mockito::server_url();
    let client = Client { signature, api_url };
    let mock = mockito::mock("GET", "/documents/1")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"state":"Parked","id":"1-1","person_id":1,"parts":[{"id":"bc","document_id":"1-9","friendly_name":"doc","hash":"9","content_type":"multipart/alternative","size_in_bytes":1410,"signatures":[{"id":4,"document_part_id":"bc","pubkey_id":"mw","signature":"HN","signature_hash":"be","endorsements":[]}],"is_base":true},{"id":"f1","document_id":"1-9","friendly_name":"hello.txt","hash":"68","content_type":"text/plain","size_in_bytes":59,"signatures":[],"is_base":false},{"id":"9f","document_id":"1-9","friendly_name":"unnamed_attachment.txt","hash":"6f","content_type":"text/html","size_in_bytes":94,"signatures":[],"is_base":false},{"id":"b6","document_id":"1-9","friendly_name":"unnamed_attachment.zip","hash":"83","content_type":"application/zip","size_in_bytes":530,"signatures":[],"is_base":false},{"id":"c7","document_id":"1-95","friendly_name":"bar/baz.txt","hash":"bf0","content_type":"text/plain","size_in_bytes":4,"signatures":[],"is_base":false},{"id":"59","document_id":"1-99","friendly_name":"foo.txt","hash":"b5","content_type":"text/plain","size_in_bytes":4,"signatures":[],"is_base":false}],"created_at":"2022-01-05T08:04:47.166681Z","cost":"1","gift_id":null,"bulletins":{},"buy_tokens_link":"https://localhost:8000/invoices/#link_token=boss+almighty+registrar+ashes+unsalted&minimum_suggested=4"}"#)
        .expect(1)
        .create();

    let json_response = client.document(&"1".to_string()).unwrap();

    assert_eq!(
      json_response,
      
r#"{
  "bulletins": {},
  "cost": "1",
  "created_at": "2022-01-05T08:04:47.166681Z",
  "gift_id": null,
  "id": "1-1",
  "parts": [
    {
      "content_type": "multipart/alternative",
      "document_id": "1-9",
      "friendly_name": "doc",
      "hash": "9",
      "id": "bc",
      "is_base": true,
      "signatures": [
        {
          "document_part_id": "bc",
          "endorsements": [],
          "id": 4,
          "pubkey_id": "mw",
          "signature": "HN",
          "signature_hash": "be"
        }
      ],
      "size_in_bytes": 1410
    },
    {
      "content_type": "text/plain",
      "document_id": "1-9",
      "friendly_name": "hello.txt",
      "hash": "68",
      "id": "f1",
      "is_base": false,
      "signatures": [],
      "size_in_bytes": 59
    },
    {
      "content_type": "text/html",
      "document_id": "1-9",
      "friendly_name": "unnamed_attachment.txt",
      "hash": "6f",
      "id": "9f",
      "is_base": false,
      "signatures": [],
      "size_in_bytes": 94
    },
    {
      "content_type": "application/zip",
      "document_id": "1-9",
      "friendly_name": "unnamed_attachment.zip",
      "hash": "83",
      "id": "b6",
      "is_base": false,
      "signatures": [],
      "size_in_bytes": 530
    },
    {
      "content_type": "text/plain",
      "document_id": "1-95",
      "friendly_name": "bar/baz.txt",
      "hash": "bf0",
      "id": "c7",
      "is_base": false,
      "signatures": [],
      "size_in_bytes": 4
    },
    {
      "content_type": "text/plain",
      "document_id": "1-99",
      "friendly_name": "foo.txt",
      "hash": "b5",
      "id": "59",
      "is_base": false,
      "signatures": [],
      "size_in_bytes": 4
    }
  ],
  "person_id": 1,
  "state": "Parked",
  "buy_tokens_link": "https://localhost:8000/invoices/#link_token=boss+almighty+registrar+ashes+unsalted&minimum_suggested=4"
}"#.to_string()
    );

    mock.assert();
  }
}