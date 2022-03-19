use simplestcrypt::{deserialize_and_decrypt, encrypt_and_serialize};
use std::str::FromStr;

use  bitcoin::{
  secp256k1::{self, All, Secp256k1},
  util::{
    bip32::{DerivationPath, ExtendedPrivKey},
    misc::MessageSignature,
  },
  Address, Network, PrivateKey,
};

use bitcoin_wallet::{account::MasterKeyEntropy, mnemonic::Mnemonic};
use crate::signed_payload::SignedPayload;

use super::*;

#[derive(Debug)]
pub struct Signature {
  key: PrivateKey,
}

impl Signature {
  pub fn create(
    env: &str,
    backup_passphrase: &str,
    mut daily_passphrase: &str,
  ) -> Result<(Config, Mnemonic)> {
    if daily_passphrase.len() > 32 {
      daily_passphrase = &daily_passphrase[..32];
    }

    let mnemonic = Mnemonic::new_random(MasterKeyEntropy::Sufficient)?;
    let seed = mnemonic.to_seed(Some(backup_passphrase));
    let context: Secp256k1<All> = Secp256k1::new();
    let master_key = ExtendedPrivKey::new_master(Network::Bitcoin, &seed.0)?;
    let for_signing = master_key.derive_priv(
      &context,
      &DerivationPath::from_str("m/44'/80'/80'").unwrap(),
    )?;
    let public_key = for_signing.private_key.public_key(&context);

    let encrypted_key = encrypt_and_serialize(
      &daily_passphrase.as_bytes(),
      for_signing.private_key.to_wif().as_bytes(),
    )
    .map_err(|_| Error::DailyKeyEncriptionError)?;

    Ok((
      Config {
        encrypted_key,
        public_key,
        environment: env.to_string(),
      },
      mnemonic,
    ))
  }

  pub fn load(stored_key: Config, daily_passphrase: &str) -> Result<Signature> {
    let decrypted = deserialize_and_decrypt(daily_passphrase.as_bytes(), &stored_key.encrypted_key)
      .unwrap_or_else(|_| {
        eprintln!("\n {} Password incorrect\n", Emoji("🚨", "*"));
        std::process::exit(1); // Exit with code 1 (fail)
      });

    let key = PrivateKey::from_wif(&String::from_utf8(decrypted)?)?;

    if key.public_key(&Secp256k1::new()) != stored_key.public_key {
      return Err(Error::ConfigKeyMismatch);
    }

    Ok(Signature { key })
  }

  pub fn public_key(&self) -> Address {
    Address::p2pkh(
      &self.key.public_key(&secp256k1::Secp256k1::new()),
      Network::Bitcoin,
    )
  }

  pub fn sign_message(&self, payload: &[u8]) -> SignedPayload {
    let secp = secp256k1::Secp256k1::new();
    let msg_hash = SignedPayload::signed_msg_hash(payload);
    let msg = secp256k1::Message::from_slice(&msg_hash).unwrap();

    let secp_sig = secp.sign_recoverable(&msg, &self.key.key);
    let signature = MessageSignature {
      signature: secp_sig,
      compressed: self.key.compressed,
    };

    SignedPayload {
      payload: payload.to_vec(),
      signer: self.public_key(),
      signature,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn creates_a_signature() {
    let (config, _mnemonic) = Signature::create("production", "very_secret", "not_so_secret").unwrap();
    let signature = Signature::load(config, "not_so_secret").unwrap();
    let signed_payload = signature.sign_message(b"hello_world");
    assert!(signed_payload.signed_ok().unwrap());
  }

  #[test]
  fn loads_existing_signature() {
    let stored: Config = serde_json::from_str("{\"public_key\":\"0203846a050544f640b3a4cf512011d73555f7b267511d3490b6f1d2deab981a3d\",\"encrypted_key\":\"85c0b2b00da46ca0e75b9bb372c571c244000000000000009e956444f91f695a759137b1e783f893f1cd8091d5c6306bf26c310284b501f21a0c99643d00b2d823f3446b93e5f0d74687acb4749e5ffd3f4be02ad356f1afdd5ed861\",\"environment\":\"development\"}").unwrap();
    let signature = Signature::load(stored, "not_so_secret").unwrap();
    let signed_payload = signature.sign_message(b"good_bye");
    assert!(signed_payload.signed_ok().unwrap());
  }
}
