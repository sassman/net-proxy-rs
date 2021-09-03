use std::process::Command;

use assert_cmd::prelude::*;

pub(crate) mod mock;

///
/// proves access to the binary
///
pub fn binary() -> Command {
    Command::cargo_bin(env!("CARGO_PKG_NAME").replace("-rs", "")).unwrap()
}
