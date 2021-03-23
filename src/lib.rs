// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

pub mod attestation;

pub const LOCAL_LISTEN_ADDRESS: &str = "0.0.0.0";

pub const PROTO_VERSION: f32 = 0.1;
pub const PROTO_NAME: &str = "Enarx-Keep-Manager";
pub const BIND_PORT: u16 = 3030;

pub enum LoaderState {
    Indeterminate,
    Ready,
    Running,
    Shutdown,
    Error,
}
//these are initial values used in existing an PoC implementation,
// many are expected to be changed
pub const KEEP_INFO_COMMAND: &str = "keep-info";
pub const CONTRACT_COMMAND: &str = "command";
pub const KEEP_COMMAND: &str = "command";
pub const KEEP_AUTH: &str = "auth-token";
pub const KEEP_PORT: &str = "keep-port";
pub const KEEP_ADDR: &str = "keep-addr";
pub const KEEP_KUUID: &str = "kuuid";
pub const KEEP_ARCH: &str = "keep-arch";
pub const WASMLDR_BIND_PORT_CMD: &str = "wasmldr-bind-port";
pub const WASMLDR_ADDR_CMD: &str = "wasmldr-addr";

pub enum Backend {
    Nil,
    Sev,
    Sgx,
    Kvm,
}
pub type KeepList = Arc<Mutex<Vec<Keep>>>;

#[derive(Serialize, Deserialize, Clone)]
pub struct KeepMgr {
    pub ipaddr: String,
    pub port: u16,
    pub keeps: Vec<Keep>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct KeepContract {
    pub keepmgr: KeepMgr,
    pub backend: String,
    //TODO - add duration of contract availability
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Wasmldr {
    pub wasmldr_ipaddr: String,
    pub wasmldr_port: u16,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Keep {
    pub backend: String,
    pub kuuid: Uuid,
    pub state: String,
    pub wasmldr: Option<Wasmldr>,
    pub human_readable_info: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Workload {
    pub wasm_binary: Vec<u8>,
    pub human_readable_info: String,
}

//TODO - rename in favour of cbor, possibly remove
#[derive(Serialize, Deserialize, Clone)]
pub struct Command {
    pub commandtype: String,
    pub commandcontents: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct KeepVec {
    pub klvec: Vec<Keep>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UndefinedReply {
    pub text: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
