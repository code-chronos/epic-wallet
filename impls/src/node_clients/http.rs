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

//! Client functions, implementations of the NodeClient trait
//! specific to the FileWallet

use futures::{stream, Stream};

use crate::api::{self, LocatedTxKernel};
use crate::core::core::TxKernel;
use crate::libwallet::{NodeClient, NodeVersionInfo, TxWrapper};
use semver::Version;
use std::collections::HashMap;
use tokio::runtime::Runtime;

use crate::client_utils::Client;
use crate::libwallet;
use crate::util::secp::pedersen;
use crate::util::{self, to_hex};

#[derive(Clone)]
pub struct HTTPNodeClient {
	pub node_url: String,
	node_api_secret: Option<String>,
	node_version_info: Option<NodeVersionInfo>,
}

impl HTTPNodeClient {
	/// Create a new client that will communicate with the given epic node
	pub fn new(node_url: &str, node_api_secret: Option<String>) -> HTTPNodeClient {
		HTTPNodeClient {
			node_url: node_url.to_owned(),
			node_api_secret: node_api_secret,
			node_version_info: None,
		}
	}

	/// Allow returning the chain height without needing a wallet instantiated
	pub fn chain_height(&self) -> Result<(u64, String), libwallet::Error> {
		self.get_chain_tip()
	}
}

impl NodeClient for HTTPNodeClient {
	fn node_url(&self) -> &str {
		&self.node_url
	}
	fn node_api_secret(&self) -> Option<String> {
		self.node_api_secret.clone()
	}

	fn set_node_url(&mut self, node_url: &str) {
		self.node_url = node_url.to_owned();
	}

	fn set_node_api_secret(&mut self, node_api_secret: Option<String>) {
		self.node_api_secret = node_api_secret;
	}

	fn get_version_info(&mut self) -> Option<NodeVersionInfo> {
		if let Some(v) = self.node_version_info.as_ref() {
			return Some(v.clone());
		}
		let url = format!("{}/v1/version", self.node_url());
		let client = Client::new();
		let mut retval = match client.get::<NodeVersionInfo>(url.as_str(), self.node_api_secret()) {
			Ok(n) => n,
			Err(e) => {
				// If node isn't available, allow offline functions
				// unfortunately have to parse string due to error structure
				let err_string = format!("{}", e);
				return if err_string.contains("404") {
					Some(NodeVersionInfo {
						node_version: "1.0.0".into(),
						block_header_version: 1,
						verified: Some(false),
					})
				} else {
					error!("Unable to contact Node to get version info: {}", e);
					None
				};
			}
		};
		retval.verified = Some(true);
		self.node_version_info = Some(retval.clone());
		Some(retval)
	}

	/// Posts a transaction to a epic node
	fn post_tx(&self, tx: &TxWrapper, fluff: bool) -> Result<(), libwallet::Error> {
		let url;
		let dest = self.node_url();
		if fluff {
			url = format!("{}/v1/pool/push_tx?fluff", dest);
		} else {
			url = format!("{}/v1/pool/push_tx", dest);
		}
		let client = Client::new();
		let res = client.post_no_ret(url.as_str(), self.node_api_secret(), tx);
		if let Err(e) = res {
			let report = format!("Posting transaction to node: {}", e);
			error!("Post TX Error: {}", e);
			return Err(libwallet::ErrorKind::ClientCallback(report).into());
		}
		Ok(())
	}

	/// Return the chain tip from a given node
	fn get_chain_tip(&self) -> Result<(u64, String), libwallet::Error> {
		let addr = self.node_url();
		let url = format!("{}/v1/chain", addr);
		let client = Client::new();
		let res = client.get::<api::Tip>(url.as_str(), self.node_api_secret());
		match res {
			Err(e) => {
				let report = format!("Getting chain height from node: {}", e);
				error!("Get chain height error: {}", e);
				Err(libwallet::ErrorKind::ClientCallback(report).into())
			}
			Ok(r) => Ok((r.height, r.last_block_pushed)),
		}
	}

	/// Get kernel implementation
	fn get_kernel(
		&mut self,
		excess: &pedersen::Commitment,
		min_height: Option<u64>,
		max_height: Option<u64>,
	) -> Result<Option<(TxKernel, u64, u64)>, libwallet::Error> {
		let version = self
			.get_version_info()
			.ok_or(libwallet::ErrorKind::ClientCallback(
				"Unable to get version".into(),
			))?;
		let version = Version::parse(&version.node_version)
			.map_err(|_| libwallet::ErrorKind::ClientCallback("Unable to parse version".into()))?;
		if version <= Version::new(2, 0, 0) {
			return Err(libwallet::ErrorKind::ClientCallback(
				"Kernel lookup not supported by node, please upgrade it".into(),
			)
			.into());
		}

		let mut query = String::new();
		if let Some(h) = min_height {
			query += &format!("min_height={}", h);
		}
		if let Some(h) = max_height {
			if query.len() > 0 {
				query += "&";
			}
			query += &format!("max_height={}", h);
		}
		if query.len() > 0 {
			query.insert_str(0, "?");
		}

		let url = format!(
			"{}/v1/chain/kernels/{}{}",
			self.node_url(),
			to_hex(excess.0.to_vec()),
			query
		);
		let client = Client::new();
		let res: Option<LocatedTxKernel> = client
			.get(url.as_str(), self.node_api_secret())
			.map_err(|e| libwallet::ErrorKind::ClientCallback(format!("Kernel lookup: {}", e)))?;

		Ok(res.map(|k| (k.tx_kernel, k.height, k.mmr_index)))
	}

	/// Retrieve outputs from node
	fn get_outputs_from_node(
		&self,
		wallet_outputs: Vec<pedersen::Commitment>,
	) -> Result<HashMap<pedersen::Commitment, (String, u64, u64)>, libwallet::Error> {
		let addr = self.node_url();
		// build the necessary query params -
		// ?id=xxx&id=yyy&id=zzz
		let query_params: Vec<String> = wallet_outputs
			.iter()
			.map(|commit| format!("id={}", util::to_hex(commit.as_ref().to_vec())))
			.collect();

		// build a map of api outputs by commit so we can look them up efficiently
		let mut api_outputs: HashMap<pedersen::Commitment, (String, u64, u64)> = HashMap::new();
		let mut tasks = Vec::new();

		let client = Client::new();

		for query_chunk in query_params.chunks(200) {
			let url = format!("{}/v1/chain/outputs/byids?{}", addr, query_chunk.join("&"),);
			tasks.push(client.get_async::<Vec<api::Output>>(url.as_str(), self.node_api_secret()));
		}

		let task = stream::futures_unordered(tasks).collect();

		let mut rt = Runtime::new().unwrap();
		let results = match rt.block_on(task) {
			Ok(outputs) => outputs,
			Err(e) => {
				let report = format!("Getting outputs by id: {}", e);
				error!("Outputs by id failed: {}", e);
				return Err(libwallet::ErrorKind::ClientCallback(report).into());
			}
		};

		for res in results {
			for out in res {
				api_outputs.insert(
					out.commit.commit(),
					(util::to_hex(out.commit.to_vec()), out.height, out.mmr_index),
				);
			}
		}
		Ok(api_outputs)
	}

	fn get_outputs_by_pmmr_index(
		&self,
		start_index: u64,
		end_index: Option<u64>,
		max_outputs: u64,
	) -> Result<
		(
			u64,
			u64,
			Vec<(pedersen::Commitment, pedersen::RangeProof, bool, u64, u64)>,
		),
		libwallet::Error,
	> {
		let addr = self.node_url();
		let mut query_param = format!("start_index={}&max={}", start_index, max_outputs);

		if let Some(e) = end_index {
			query_param = format!("{}&end_index={}", query_param, e);
		};

		let url = format!("{}/v1/txhashset/outputs?{}", addr, query_param,);

		let mut api_outputs: Vec<(pedersen::Commitment, pedersen::RangeProof, bool, u64, u64)> =
			Vec::new();

		let client = Client::new();

		match client.get::<api::OutputListing>(url.as_str(), self.node_api_secret()) {
			Ok(o) => {
				for out in o.outputs {
					let is_coinbase = match out.output_type {
						api::OutputType::Coinbase => true,
						api::OutputType::Transaction => false,
					};
					let range_proof = match out.range_proof() {
						Ok(r) => r,
						Err(e) => {
							let msg = format!("Unexpected error in returned output (missing range proof): {:?}. {:?}, {}",
									out.commit,
									out,
									e);
							error!("{}", msg);
							Err(libwallet::ErrorKind::ClientCallback(msg))?
						}
					};
					let block_height = match out.block_height {
						Some(h) => h,
						None => {
							let msg = format!("Unexpected error in returned output (missing block height): {:?}. {:?}",
									out.commit,
									out);
							error!("{}", msg);
							Err(libwallet::ErrorKind::ClientCallback(msg))?
						}
					};
					api_outputs.push((
						out.commit,
						range_proof,
						is_coinbase,
						block_height,
						out.mmr_index,
					));
				}
				Ok((o.highest_index, o.last_retrieved_index, api_outputs))
			}
			Err(e) => {
				// if we got anything other than 200 back from server, bye
				error!(
					"get_outputs_by_pmmr_index: error contacting {}. Error: {}",
					addr, e
				);
				let report = format!("outputs by pmmr index: {}", e);
				Err(libwallet::ErrorKind::ClientCallback(report))?
			}
		}
	}

	fn height_range_to_pmmr_indices(
		&self,
		start_height: u64,
		end_height: Option<u64>,
	) -> Result<(u64, u64), libwallet::Error> {
		debug!("Indices start");
		let addr = self.node_url();
		let mut query_param = format!("start_height={}", start_height);
		if let Some(e) = end_height {
			query_param = format!("{}&end_height={}", query_param, e);
		};

		let url = format!("{}/v1/txhashset/heightstopmmr?{}", addr, query_param,);

		let client = Client::new();

		match client.get::<api::OutputListing>(url.as_str(), self.node_api_secret()) {
			Ok(o) => Ok((o.last_retrieved_index, o.highest_index)),
			Err(e) => {
				// if we got anything other than 200 back from server, bye
				error!("heightstopmmr: error contacting {}. Error: {}", addr, e);
				let report = format!(": {}", e);
				Err(libwallet::ErrorKind::ClientCallback(report))?
			}
		}
	}
}

/*
/// Call the wallet API to create a coinbase output for the given block_fees.
/// Will retry based on default "retry forever with backoff" behavior.
pub fn create_coinbase(dest: &str, block_fees: &BlockFees) -> Result<CbData, Error> {
	let url = format!("{}/v1/wallet/foreign/build_coinbase", dest);
	match single_create_coinbase(&url, &block_fees) {
		Err(e) => {
			error!(
				"Failed to get coinbase from {}. Run epic wallet listen?",
				url
			);
			error!("Underlying Error: {}", e.cause().unwrap());
			error!("Backtrace: {}", e.backtrace().unwrap());
			Err(e)?
		}
		Ok(res) => Ok(res),
	}
}

/// Makes a single request to the wallet API to create a new coinbase output.
fn single_create_coinbase(url: &str, block_fees: &BlockFees) -> Result<CbData, Error> {
	let res = Client::post(url, None, block_fees).context(ErrorKind::GenericError(
		"Posting create coinbase".to_string(),
	))?;
	Ok(res)
}*/
