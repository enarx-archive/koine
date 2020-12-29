// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;
use tokio::sync::Mutex;
use super::super::*;

pub type KeepList = Arc<Mutex<Vec<Keep>>>;
pub type ContractList = Arc<Mutex<Vec<KeepContract>>>;
