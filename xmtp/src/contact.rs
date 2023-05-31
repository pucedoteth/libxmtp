use prost::{DecodeError, EncodeError, Message};
use thiserror::Error;

use vodozemac::Curve25519PublicKey;
use xmtp_cryptography::hash::keccak256;
use xmtp_proto::xmtp::v3::message_contents::{
    installation_contact_bundle::Version as ContactBundleVersionProto, InstallationContactBundle,
};

use crate::{utils::base64_encode, vmac_protos::ProtoWrapper};

#[derive(Debug, Error)]
pub enum ContactError {
    #[error("decode error")]
    Decode(#[from] DecodeError),
    #[error("encode error")]
    Encode(#[from] EncodeError),
    #[error("unknown error")]
    Unknown,
}
#[derive(Clone)]
pub struct Contact {
    pub(crate) bundle: InstallationContactBundle,
}

impl Contact {
    pub fn new(bundle: InstallationContactBundle) -> Self {
        Self { bundle }
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, ContactError> {
        let bundle = InstallationContactBundle::decode(bytes.as_slice())?;

        Ok(Self { bundle })
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, ContactError> {
        let mut buf = Vec::new();
        self.bundle.encode(&mut buf)?;

        Ok(buf)
    }

    // The id of a contact is the base64 encoding of the keccak256 hash of the identity key
    pub fn id(&self) -> String {
        base64_encode(keccak256(self.vmac_identity_key().to_string().as_str()).as_slice())
    }

    pub fn vmac_identity_key(&self) -> Curve25519PublicKey {
        let identity_key = match self.bundle.clone().version.unwrap() {
            ContactBundleVersionProto::V1(v1) => v1.identity_key.unwrap(),
        };

        let proto_key = ProtoWrapper {
            proto: identity_key,
        };

        proto_key.into()
    }

    pub fn vmac_fallback_key(&self) -> Curve25519PublicKey {
        let fallback_key = match self.bundle.clone().version.unwrap() {
            ContactBundleVersionProto::V1(v1) => v1.fallback_key.unwrap(),
        };
        let proto_key = ProtoWrapper {
            proto: fallback_key,
        };

        proto_key.into()
    }
}

#[cfg(test)]
mod tests {
    use crate::account::{tests::test_wallet_signer, Account};

    use super::Contact;

    #[test]
    fn serialize_round_trip() {
        let account = Account::generate(test_wallet_signer).unwrap();
        let contact = account.contact();
        let contact_bytes = contact.to_bytes().unwrap();
        let contact2 = Contact::from_bytes(contact_bytes.clone()).unwrap();
        assert_eq!(contact2.to_bytes().unwrap(), contact_bytes);
    }
}
