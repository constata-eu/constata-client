pub mod signature;
pub mod signed_payload;

use signature::Signature;

use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use bitcoin::PublicKey;

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
      Err(ref e) if e.raw_os_error() == Some(21) => panic!("{} is a directory. Stamping could only be applied on files. If you want to stamp an entire directory, consider compress it into a zip file", path),
      Err(ref e) if e.raw_os_error() == Some(2) => panic!("File not found using path {}", path),
      Err(err) => return Err(err.into()),
    };
    self.sign_and_timestamp(&file_path)
  }

  pub fn documents(&self) -> Result<String> {
    self.get_json("/documents")
  }

  pub fn document(&self, document_id: &str) -> Result<String> {
    self.get_json(&format!("/documents/{}", document_id))
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
    self.get_json("/account_state")
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use mockito;
  use cool_asserts::assert_panics;

  #[test]
  fn is_a_directory_friendly_response() {
    let (config, _mnemonic) = Signature::create("production", "very_secret", "not_so_secret").unwrap();
    let signature = Signature::load(config, "not_so_secret").unwrap();

    let api_url = mockito::server_url();
    let client = Client { signature, api_url };
    let mock = mockito::mock("POST", "/documents")
        .with_status(200)
        .expect(0)
        .create();


    assert_panics!(
      client.sign_and_timestamp_path(&"./".to_string()),
      includes("./ is a directory. Stamping could only be applied")
    );

    mock.assert();
  }

  #[test]
  fn file_not_found_friendly_response() {
    let (config, _mnemonic) = Signature::create("production", "very_secret", "not_so_secret").unwrap();
    let signature = Signature::load(config, "not_so_secret").unwrap();

    let api_url = mockito::server_url();
    let client = Client { signature, api_url };
    let mock = mockito::mock("POST", "/documents")
        .with_status(200)
        .expect(0)
        .create();


    assert_panics!(
      client.sign_and_timestamp_path(&"./cuca".to_string()),
      includes("File not found using path ./cuca")
    );

    mock.assert();
  }
}