// Copyright 2019 The Epic Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Public types for config modules

use std::fmt;
use std::io;
use std::path::PathBuf;

use crate::core::global::ChainTypes;
use crate::util::logger::LoggingConfig;

/// Command-line wallet configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WalletConfig {
	/// Chain parameters (default to Mainnet if none at the moment)
	pub chain_type: Option<ChainTypes>,
	/// The api interface/ip_address that this api server (i.e. this wallet) will run
	/// by default this is 127.0.0.1 (and will not accept connections from external clients)
	pub api_listen_interface: String,
	/// The port this wallet will run on
	pub api_listen_port: u16,
	/// The port this wallet's owner API will run on
	pub owner_api_listen_port: Option<u16>,
	/// Location of the secret for basic auth on the Owner API
	pub api_secret_path: Option<String>,
	/// Location of the node api secret for basic auth on the Epic API
	pub node_api_secret_path: Option<String>,
	/// The api address of a running server node against which transaction inputs
	/// will be checked during send
	pub check_node_api_http_addr: String,
	/// Whether to include foreign API endpoints on the Owner API
	pub owner_api_include_foreign: Option<bool>,
	/// The directory in which wallet files are stored
	pub data_file_dir: String,
	/// If Some(true), don't cache commits alongside output data
	/// speed improvement, but your commits are in the database
	pub no_commit_cache: Option<bool>,
	/// TLS certificate file
	pub tls_certificate_file: Option<String>,
	/// TLS certificate private key file
	pub tls_certificate_key: Option<String>,
	/// Whether to use the black background color scheme for command line
	/// if enabled, wallet command output color will be suitable for black background terminal
	pub dark_background_color_scheme: Option<bool>,
	/// The exploding lifetime (minutes) for keybase notification on coins received
	pub keybase_notify_ttl: Option<u16>,
}

impl Default for WalletConfig {
	fn default() -> WalletConfig {
		WalletConfig {
			chain_type: Some(ChainTypes::Mainnet),
			api_listen_interface: "127.0.0.1".to_string(),
			api_listen_port: 3415,
			owner_api_listen_port: Some(WalletConfig::default_owner_api_listen_port()),
			api_secret_path: Some(".owner_api_secret".to_string()),
			node_api_secret_path: Some(".api_secret".to_string()),
			check_node_api_http_addr: "http://127.0.0.1:3413".to_string(),
			owner_api_include_foreign: Some(false),
			data_file_dir: ".".to_string(),
			no_commit_cache: Some(false),
			tls_certificate_file: None,
			tls_certificate_key: None,
			dark_background_color_scheme: Some(true),
			keybase_notify_ttl: Some(1440),
		}
	}
}

impl WalletConfig {
	/// API Listen address
	pub fn api_listen_addr(&self) -> String {
		format!("{}:{}", self.api_listen_interface, self.api_listen_port)
	}

	/// Default listener port
	pub fn default_owner_api_listen_port() -> u16 {
		3420
	}

	/// Use value from config file, defaulting to sensible value if missing.
	pub fn owner_api_listen_port(&self) -> u16 {
		self.owner_api_listen_port
			.unwrap_or(WalletConfig::default_owner_api_listen_port())
	}

	/// Owner API listen address
	pub fn owner_api_listen_addr(&self) -> String {
		format!("127.0.0.1:{}", self.owner_api_listen_port())
	}
}
/// Error type wrapping config errors.
#[derive(Debug)]
pub enum ConfigError {
	/// Error with parsing of config file
	ParseError(String, String),

	/// Error with fileIO while reading config file
	FileIOError(String, String),

	/// No file found
	FileNotFoundError(String),

	/// Error serializing config values
	SerializationError(String),
}

impl fmt::Display for ConfigError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match *self {
			ConfigError::ParseError(ref file_name, ref message) => write!(
				f,
				"Error parsing configuration file at {} - {}",
				file_name, message
			),
			ConfigError::FileIOError(ref file_name, ref message) => {
				write!(f, "{} {}", message, file_name)
			}
			ConfigError::FileNotFoundError(ref file_name) => {
				write!(f, "Configuration file not found: {}", file_name)
			}
			ConfigError::SerializationError(ref message) => {
				write!(f, "Error serializing configuration: {}", message)
			}
		}
	}
}

/// Tor configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TorConfig {
	/// Whether to start tor listener on listener startup (default true)
	pub use_tor_listener: bool,
	/// Just the address of the socks proxy for now
	pub socks_proxy_addr: String,
	/// Send configuration directory
	pub send_config_dir: String,
}

impl Default for TorConfig {
	fn default() -> TorConfig {
		TorConfig {
			use_tor_listener: true,
			socks_proxy_addr: "127.0.0.1:59050".to_owned(),
			send_config_dir: ".".into(),
		}
	}
}

/// Epicbox configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EpicboxConfig {
	/// Default epicbox Domain/Server
	pub epicbox_domain: String,
	/// Connect to epicbox port 443 or 80
	pub epicbox_port: Option<u16>,
	/// Use to epicbox port 443 or 80
	pub epicbox_protocol_unsecure: Option<bool>,
	/// Epicbox address id
	pub epicbox_address_index: Option<u32>,
}

impl Default for EpicboxConfig {
	fn default() -> EpicboxConfig {
		EpicboxConfig {
			epicbox_domain: "epicbox.epic.tech".to_owned(),
			epicbox_port: Some(443),
			epicbox_protocol_unsecure: Some(false),
			epicbox_address_index: Some(0),
		}
	}
}

impl From<io::Error> for ConfigError {
	fn from(error: io::Error) -> ConfigError {
		ConfigError::FileIOError(
			String::from(""),
			String::from(format!("Error loading config file: {}", error)),
		)
	}
}

/// Wallet should be split into a separate configuration file
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GlobalWalletConfig {
	/// Keep track of the file we've read
	pub config_file_path: Option<PathBuf>,
	/// Wallet members
	pub members: Option<GlobalWalletConfigMembers>,
}

/// Wallet internal members
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GlobalWalletConfigMembers {
	/// Wallet configuration
	#[serde(default)]
	pub wallet: WalletConfig,
	/// Tor config
	pub tor: Option<TorConfig>,
	/// Epicbox config
	pub epicbox: Option<EpicboxConfig>,
	/// Logging config
	pub logging: Option<LoggingConfig>,
}
