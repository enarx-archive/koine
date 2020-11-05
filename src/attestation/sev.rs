// SPDX-License-Identifier: Apache-2.0

//! The `sev` module exports the Message types required for both the
//! client and server components of the remote AMD SEV attestation
//! protocol[1].
//!
//! [1] https://github.com/enarx/enarx-keepldr/wiki/AMD-SEV-Remote-Attestation-Protocol

use serde::{Deserialize, Serialize};

/// The `Measurement` struct is a container payload for concatenating
/// an `sev` crate `Build` type with an `sev` crate `Measurement`
/// type.
///
/// This struct's fields follow the same rules described in the
/// `Message` type's `payload` field.
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Measurement {
    pub build: Vec<u8>,
    pub measurement: Vec<u8>,
}

/// The `MimeType` identifies the payload carried in the message.
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum MimeType {
    /// The Naples certificate chain carries the SEV platform
    /// certificate chain, but the AMD ARK and ASK are "small"
    /// CA certificates (using 256 byte components).
    #[serde(rename = "certificate-chain-naples")]
    CertificateChainNaples,

    /// The Rome certificate chain also carries the SEV platform
    /// certificate chain, but the AMD ARK and ASK are "large"
    /// CA certificates (using 512 byte components).
    #[serde(rename = "certificate-chain-rome")]
    CertificateChainRome,

    /// The launch start buffer establishes a secure channel with
    /// the remote SEV platform and furnishes with information
    /// that the tenant has tailored to match their expectations
    /// for the secure VM.
    #[serde(rename = "launch-start")]
    LaunchStart,

    /// The measurement buffer contains the AMD Secure Processor's
    /// Build and Measurement buffers so that the tenant may
    /// individually determine if the environment is correct.
    #[serde(rename = "measurement")]
    Measurement,

    /// The secret packet contains the secret to inject into the
    /// secure VM.
    #[serde(rename = "secret")]
    Secret,

    /// The finish message signals a successful attestation & launch.
    /// Its payload describes the relevant launch configuration details.
    #[serde(rename = "finish")]
    Finish,
}

#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Message {
    pub mimetype: MimeType,
    // A valid payload is either:
    //   1. A CBOR-encoded SEV data structure (to make this protocol accessible
    //      to other SEV implementations, not just the Enarx `sev` crate)
    //   2. A binary blob that corresponds _exactly_ to the structures outlined
    //      in the `sev` crate.
    // The SEV backend will attempt to deserialize the payload one way, failing
    // that, it will then try deserializing the other way.
    //
    // NOTE: Container types (such as `Message` and `Measurement` **MUST** be
    // CBOR-encoded)
    //
    // As an added bonus, this strategy allows koine to avoid depending on the
    // 'sev' crate at all.
    pub payload: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use serde_cbor as serde_flavor;

    use super::*;

    #[test]
    fn test_message_representation() {
        let test_cases = &[
            // {"mimetype": "certificate-chain-naples", "payload": [1, 2, 3, 4]}
            Message {
                mimetype: MimeType::CertificateChainNaples,
                payload: vec![1, 2, 3, 4],
            },
            // {"mimetype": "certificate-chain-rome", "payload": [1, 2, 3, 4]}
            Message {
                mimetype: MimeType::CertificateChainRome,
                payload: vec![1, 2, 3, 4],
            },
            // {"mimetype": "launch-start", "payload": [1, 2, 3, 4]}
            Message {
                mimetype: MimeType::LaunchStart,
                payload: vec![1, 2, 3, 4],
            },
            // {"mimetype": "measurement", "payload": [1, 2, 3, 4]}
            Message {
                mimetype: MimeType::Measurement,
                payload: vec![1, 2, 3, 4],
            },
            // {"mimetype": "secret", "payload": [1, 2, 3, 4]}
            Message {
                mimetype: MimeType::Secret,
                payload: vec![1, 2, 3, 4],
            },
            // {"mimetype": "finish", "payload": [1, 2, 3, 4]}
            Message {
                mimetype: MimeType::Finish,
                payload: vec![1, 2, 3, 4],
            },
        ];

        for test_case in test_cases {
            let mut encoded = vec![];
            serde_flavor::to_writer(&mut encoded, test_case).unwrap();
            let actual = serde_flavor::from_reader(&encoded[..]).unwrap();
            assert_eq!(*test_case, actual);
        }
    }
}
