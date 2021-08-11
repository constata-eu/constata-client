use serde::{Deserialize, Serialize};

use bitcoin::{
  consensus::{encode, Encodable},
  hashes::{sha256d, Hash, HashEngine},
  secp256k1,
  util::misc::{MessageSignature, BITCOIN_SIGNED_MSG_PREFIX},
  Address,
};

use base64_serde::base64_serde_type;
base64_serde_type!(Base64Standard, base64::STANDARD);

use serde_with::{serde_as, DisplayFromStr};
use sha2::{Digest, Sha256};

pub fn hexdigest(bytes: &[u8]) -> String {
  let mut hasher = Sha256::new();
  hasher.update(bytes);
  format!("{:x}", hasher.finalize())
}

#[serde_as]
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct SignedPayload {
  #[serde(with = "Base64Standard")]
  pub payload: Vec<u8>,
  pub signer: Address,
  #[serde_as(as = "DisplayFromStr")]
  pub signature: MessageSignature,
}

impl SignedPayload {
  pub fn signed_msg_hash(msg: &[u8]) -> sha256d::Hash {
    let mut engine = sha256d::Hash::engine();
    engine.input(BITCOIN_SIGNED_MSG_PREFIX);
    let msg_len = encode::VarInt(msg.len() as u64);
    msg_len.consensus_encode(&mut engine).unwrap();
    engine.input(msg);
    sha256d::Hash::from_engine(engine)
  }

  pub fn payload_hash(&self) -> String {
    hexdigest(&self.payload)
  }

  pub fn unique_id(&self) -> String {
    hexdigest(format!("{}{}", &self.signer, &self.payload_hash()).as_bytes())
  }

  pub fn signed_ok(&self) -> Result<bool, bitcoin::secp256k1::Error> {
    Ok(self.signature.is_signed_by_address(
      &secp256k1::Secp256k1::new(),
      &self.signer,
      SignedPayload::signed_msg_hash(&self.payload),
    )?)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn deserializes_signed_payload_json() {
    let signed_payload: SignedPayload = serde_json::from_str(r#"{
      "payload":"aGVsbG8gd29ybGQ=",
      "signer":"mqwpxxvfv3QbM8PU8uBx2jaNt9btQqvQNx",
      "signature":"H6O6iC1NL18vjMVllny5oQz87Ir7O6n0v/rup8zBPjjAXWENMkJRcEQ69SRKXfw2QYen2PLt3amkY2bE+Fw623w="
    }"#).unwrap();
    assert!(signed_payload.signed_ok().unwrap());
    assert_eq!(signed_payload.payload, b"hello world".to_vec());
  }

  #[test]
  fn deserializes_bad_signatures_too() {
    let signed_payload: SignedPayload = serde_json::from_str(r#"{
      "payload":"bGVsbG8gd29ybGA=",
      "signer":"mqwpxxvfv3QbM8PU8uBx2jaNt9btQqvQNx",
      "signature":"H6O6iC1NL18vjMVllny5oQz87Ir7O6n0v/rup8zBPjjAXWENMkJRcEQ69SRKXfw2QYen2PLt3amkY2bE+Fw623w="
    }"#).unwrap();
    assert!(!signed_payload.signed_ok().unwrap());
  }
}
