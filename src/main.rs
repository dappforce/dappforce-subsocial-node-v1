//! Substrate Node Template CLI library.

#![warn(missing_docs)]
#![warn(unused_extern_crates)]

mod chain_spec;
#[macro_use]
mod service;
mod cli;

pub use sc_cli::{VersionInfo, IntoExit, error};

fn main() -> Result<(), cli::error::Error> {
	let version = VersionInfo {
		name: "Subsocial Node",
		commit: env!("VERGEN_SHA_SHORT"),
		version: env!("CARGO_PKG_VERSION"),
		executable_name: "subsocial-node",
		author: "Dappforce",
		description: "Dappforce Subsocial Substrate node",
		support_url: "http://dappforce.io",
	};

	cli::run(std::env::args(), cli::Exit, version)
}
