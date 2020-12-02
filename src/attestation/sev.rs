// SPDX-License-Identifier: Apache-2.0

//! The `sev` module exports the Message types required for both the
//! client and server components of the remote AMD SEV attestation
//! protocol[1].
//!
//! [1] https://github.com/enarx/enarx-keepldr/wiki/AMD-SEV-Remote-Attestation-Protocol

use serde::{Deserialize, Serialize};

/// The `Finish` struct contains useful information regarding the launch
/// configuration for the successfully launched VM.
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Finish;

/// The `LaunchStart` struct is a container payload for transporting the
/// sev::launch::Start struct. Its fields are CBOR-encoded with the
/// exception of the certificate, which is a binary blob. See the
/// footnote for the `Chain` struct for more information.
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct LaunchStart {
    pub policy: Vec<u8>,
    pub cert: Vec<u8>,
    pub session: Vec<u8>,
}

/// The `Chain` struct is a container payload for transporting the entire
/// certificate chain across the wire. It is important to note the following:
///
/// 1. The ARK and ASK come in different sizes. Right now, the Naples generation
///    certificates are small and the Rome generation certificates are large.
///    The enclosing Message struct's mimetype can be checked to see which one
///    you're dealing with. This hint will be useful for others who are
///    implementing their own tooling/parsers.
/// 2. The certificates are _not_ CBOR-encoded. These are opaque binary blobs
///    that are laid out exactly as described in the AMD SEV specification[1].
///
/// [1] See appendices B & C for certificate layout details:
/// https://www.amd.com/system/files/TechDocs/55766_SEV-KM_API_Specification.pdf
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Chain {
    pub ark: Vec<u8>,
    pub ask: Vec<u8>,
    pub oca: Vec<u8>,
    pub cek: Vec<u8>,
    pub pek: Vec<u8>,
    pub pdh: Vec<u8>,
}

/// The `Measurement` struct is a container payload for concatenating
/// a `sev` crate `Build` type with a `sev` crate `Measurement` type.
///
/// The `build` and `measurement` fields are CBOR-encoded structures.
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Measurement {
    pub build: Vec<u8>,
    pub measurement: Vec<u8>,
}

/// A CBOR-encoded attestation message.
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Message {
    /// The Naples certificate chain carries the SEV platform
    /// certificate chain, but the AMD ARK and ASK are "small"
    /// CA certificates (using 256 byte components).
    CertificateChainNaples(Chain),

    /// The Rome certificate chain also carries the SEV platform
    /// certificate chain, but the AMD ARK and ASK are "large"
    /// CA certificates (using 512 byte components).
    CertificateChainRome(Chain),

    /// The launch start buffer establishes a secure channel with
    /// the remote SEV platform and furnishes with information
    /// that the tenant has tailored to match their expectations
    /// for the secure VM.
    LaunchStart(LaunchStart),

    /// The measurement buffer contains the AMD Secure Processor's
    /// Build and Measurement buffers so that the tenant may
    /// individually determine if the environment is correct.
    Measurement(Measurement),

    /// The secret packet contains the secret to inject into the
    /// secure VM. The payload is a CBOR-encoded `sev::launch::Secret`.
    Secret(Vec<u8>),

    /// The finish message signals a successful attestation & launch.
    /// Its payload describes the relevant launch configuration details.
    Finish(Finish),
}

#[cfg(test)]
mod tests {
    use ciborium::de::from_reader;
    use ciborium::ser::into_writer;

    use super::*;

    fn test_message_representation(bytes: &[u8], expected: Message) {
        let actual: Message = from_reader(bytes).unwrap();
        assert_eq!(actual, expected);

        let mut out = vec![];
        into_writer(&actual, &mut out).unwrap();
        let actual: Message = from_reader(&out[..]).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_message_certificate_chain_naples() {
        let input = b"\xA1\x78\x18\x63\x65\x72\x74\x69\x66\x69\x63\x61\x74\x65\
                    \x2D\x63\x68\x61\x69\x6E\x2D\x6E\x61\x70\x6C\x65\x73\xA6\
                    \x63\x61\x72\x6B\x84\x01\x02\x03\x04\x63\x61\x73\x6B\x84\
                    \x05\x06\x07\x08\x63\x70\x65\x6B\x84\x09\x0A\x0B\x0C\x63\
                    \x63\x65\x6B\x84\x0D\x0E\x0F\x10\x63\x70\x64\x68\x84\x11\
                    \x12\x13\x14\x63\x6F\x63\x61\x84\x15\x16\x17\x18\x18";
        let expected = Message::CertificateChainNaples(Chain {
            ark: vec![1, 2, 3, 4],
            ask: vec![5, 6, 7, 8],
            pek: vec![9, 10, 11, 12],
            cek: vec![13, 14, 15, 16],
            pdh: vec![17, 18, 19, 20],
            oca: vec![21, 22, 23, 24],
        });

        test_message_representation(&input[..], expected);
    }

    #[test]
    fn test_message_certificate_chain_rome() {
        let input = b"\xA1\x76\x63\x65\x72\x74\x69\x66\x69\x63\x61\x74\x65\x2D\
                    \x63\x68\x61\x69\x6E\x2D\x72\x6F\x6D\x65\xA6\x63\x61\x72\
                    \x6B\x84\x01\x02\x03\x04\x63\x61\x73\x6B\x84\x05\x06\x07\
                    \x08\x63\x70\x65\x6B\x84\x09\x0A\x0B\x0C\x63\x63\x65\x6B\
                    \x84\x0D\x0E\x0F\x10\x63\x70\x64\x68\x84\x11\x12\x13\x14\
                    \x63\x6F\x63\x61\x84\x15\x16\x17\x18\x18";
        let expected = Message::CertificateChainRome(Chain {
            ark: vec![1, 2, 3, 4],
            ask: vec![5, 6, 7, 8],
            pek: vec![9, 10, 11, 12],
            cek: vec![13, 14, 15, 16],
            pdh: vec![17, 18, 19, 20],
            oca: vec![21, 22, 23, 24],
        });

        test_message_representation(&input[..], expected);
    }

    #[test]
    fn test_message_launch_start() {
        let input = b"\xA1\x6C\x6C\x61\x75\x6E\x63\x68\x2D\x73\x74\x61\x72\x74\
                    \xA3\x66\x70\x6F\x6C\x69\x63\x79\x84\x01\x02\x03\x04\x64\
                    \x63\x65\x72\x74\x84\x05\x06\x07\x08\x67\x73\x65\x73\x73\
                    \x69\x6F\x6E\x84\x09\x0A\x0B\x0C";
        let expected = Message::LaunchStart(LaunchStart {
            policy: vec![1, 2, 3, 4],
            cert: vec![5, 6, 7, 8],
            session: vec![9, 10, 11, 12],
        });

        test_message_representation(&input[..], expected);
    }

    #[test]
    fn test_message_measurement() {
        let input = b"\xA1\x6B\x6D\x65\x61\x73\x75\x72\x65\x6D\x65\x6E\x74\xA2\
                    \x65\x62\x75\x69\x6C\x64\x84\x01\x02\x03\x04\x6B\x6D\x65\
                    \x61\x73\x75\x72\x65\x6D\x65\x6E\x74\x84\x05\x06\x07\x08";
        let expected = Message::Measurement(Measurement {
            build: vec![1, 2, 3, 4],
            measurement: vec![5, 6, 7, 8],
        });

        test_message_representation(&input[..], expected);
    }

    #[test]
    fn test_message_secret() {
        let input = b"\xA1\x66\x73\x65\x63\x72\x65\x74\x84\x01\x02\x03\x04";
        let expected = Message::Secret(vec![1, 2, 3, 4]);

        test_message_representation(&input[..], expected);
    }

    #[test]
    fn test_message_finish() {
        let input = b"\xA1\x66\x66\x69\x6E\x69\x73\x68\xF6";
        let expected = Message::Finish(Finish);

        test_message_representation(&input[..], expected);
    }
}
