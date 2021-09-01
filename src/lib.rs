pub mod signature;
pub mod signed_payload;

//use std::ops::Sub;

use signature::Signature;

use bitcoin::PublicKey;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_with::serde_as;

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

/**
  Uso std::result::Result porque el Result común está redefinido sin Err.
*/
pub enum SdkResult {
    Binary(std::result::Result<Vec<u8>, String>),
    Json(std::result::Result<serde_json::Value, serde_json::Value>),
}
pub enum CliResult {
    Binary(std::result::Result<Vec<u8>, String>),
    Json(std::result::Result<serde_json::Value, serde_json::Value>),
}
pub enum SubcommandResult {
    Sdk(SdkResult),
    Cli(CliResult),
}

impl SubcommandResult {
    pub fn cli_ok(value: Vec<u8>) -> SubcommandResult {
        SubcommandResult::Cli(CliResult::Binary(Ok(value)))
    }

    pub fn cli_err(value: String) -> SubcommandResult {
        SubcommandResult::Cli(CliResult::Binary(Err(value)))
    }

    pub fn sdk_ok(value: Vec<u8>) -> SubcommandResult {
        SubcommandResult::Sdk(SdkResult::Binary(Ok(value)))
    }

    pub fn sdk_err(value: String) -> SubcommandResult {
        SubcommandResult::Sdk(SdkResult::Binary(Err(value)))
    }

    pub fn _cli_json(value: serde_json::Value) -> SubcommandResult {
        SubcommandResult::Cli(CliResult::Json(Ok(value)))
    }

    pub fn _cli_json_err(value: serde_json::Value) -> SubcommandResult {
        SubcommandResult::Cli(CliResult::Json(Err(value)))
    }
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

    pub fn sign_and_timestamp_path(&self, path: &str) -> SubcommandResult {
        let result = self.sign_and_timestamp(&std::fs::read(path).unwrap());

        match result {
            Ok(data) => SubcommandResult::cli_ok(data.as_bytes().to_vec()),
            Err(error) => SubcommandResult::cli_err(error.to_string()),
        }
    }

    pub fn documents(&self) -> SubcommandResult {
        let result = self.get_json("/documents");

        match result {
            Ok(data) => SubcommandResult::cli_ok(data.as_bytes().to_vec()),
            Err(error) => SubcommandResult::cli_err(error.to_string()),
        }
    }

    pub fn document(&self, document_id: &str) -> SubcommandResult {
        let result = self.get_json(&format!("/documents/{}", document_id));
        match result {
            Ok(data) => SubcommandResult::cli_ok(data.as_bytes().to_vec()),
            Err(error) => SubcommandResult::cli_err(error.to_string()),
        }
    }

    pub fn fetch_proof(&self, document_id: &str, is_cli: bool) -> SubcommandResult {
        let result = self.get(&format!("/documents/{}/html_proof", document_id));

        if is_cli {
            match result {
                Ok(value) => SubcommandResult::cli_ok(value.as_bytes().to_vec()),
                Err(error) => SubcommandResult::cli_err(error.to_string()),
            }
        } else {
            match result {
                Ok(value) => SubcommandResult::cli_ok(value.as_bytes().to_vec()),
                Err(error) => SubcommandResult::cli_err(error.to_string()),
            }
        }
    }

    pub fn fetch_each_proof(&self, document_id: &str) -> SubcommandResult {
        use std::io::Read;

        let response =
            self.get_response(&format!("/documents/{}/each_part_html_proof", document_id));

        match response {
            Ok(result) => {
                let mut bytes: Vec<u8> = vec![];
                result
                    .into_reader()
                    .take(10_000_000)
                    .read_to_end(&mut bytes);
                SubcommandResult::cli_ok(bytes)
            }
            Err(error) => SubcommandResult::cli_err(error.to_string()),
        }
    }
}
