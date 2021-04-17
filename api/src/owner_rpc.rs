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

//! JSON-RPC Stub generation for the Owner API
use uuid::Uuid;

use crate::core::core::Transaction;
use crate::keychain::{Identifier, Keychain};
use crate::libwallet::slate_versions::v3::TransactionV3;
use crate::libwallet::{
	AcctPathMapping, ErrorKind, InitTxArgs, IssueInvoiceTxArgs, NodeClient, NodeHeightResult,
	OutputCommitMapping, Slate, SlateVersion, TxLogEntry, VersionedSlate, WalletInfo,
	WalletLCProvider,
};
use crate::util::Mutex;
use crate::{Owner, OwnerRpcS};
use easy_jsonrpc_mw;
use std::sync::Arc;

/// Public definition used to generate Owner jsonrpc api.
/// * When running `epic-wallet owner_api` with defaults, the V2 api is available at
/// `localhost:3420/v2/owner`
/// * The endpoint only supports POST operations, with the json-rpc request as the body
#[easy_jsonrpc_mw::rpc]
pub trait OwnerRpc: Sync + Send {
	/**
	Networked version of [Owner::accounts](struct.Owner.html#method.accounts).

	# Json rpc example

	```
	# epic_wallet_api::doctest_helper_json_rpc_owner_assert_response!(
	# r#"
	{
		"jsonrpc": "2.0",
		"method": "accounts",
		"params": [],
		"id": 1
	}
	# "#
	# ,
	# r#"
	{
		"jsonrpc": "2.0",
		"result": {
			"Ok": [
				{
					"label": "default",
					"path": "0200000000000000000000000000000000"
				}
			]
		},
		"id": 1
	}
	# "#
	# , false, 4, false, false, false);
	```
	*/
	#[deprecated(
		since = "3.0.0",
		note = "The V2 Owner API (OwnerRpc) will be removed in epic-wallet 4.0.0. Please migrate to the V3 (OwnerRpcS) API as soon as possible."
	)]
	fn accounts(&self) -> Result<Vec<AcctPathMapping>, ErrorKind>;

	/**
	Networked version of [Owner::create_account_path](struct.Owner.html#method.create_account_path).

	# Json rpc example

	```
	# epic_wallet_api::doctest_helper_json_rpc_owner_assert_response!(
	# r#"
	{
		"jsonrpc": "2.0",
		"method": "create_account_path",
		"params": ["account1"],
		"id": 1
	}
	# "#
	# ,
	# r#"
	{
		"jsonrpc": "2.0",
		"result": {
			"Ok": "0200000001000000000000000000000000"
		},
		"id": 1
	}
	# "#
	# ,false, 4, false, false, false);
	```
	 */
	fn create_account_path(&self, label: &String) -> Result<Identifier, ErrorKind>;

	/**
	Networked version of [Owner::set_active_account](struct.Owner.html#method.set_active_account).

	# Json rpc example

	```
	# epic_wallet_api::doctest_helper_json_rpc_owner_assert_response!(
	# r#"
	{
		"jsonrpc": "2.0",
		"method": "set_active_account",
		"params": ["default"],
		"id": 1
	}
	# "#
	# ,
	# r#"
	{
		"jsonrpc": "2.0",
		"result": {
			"Ok": null
		},
		"id": 1
	}
	# "#
	# , false, 4, false, false, false);
	```
	 */
	fn set_active_account(&self, label: &String) -> Result<(), ErrorKind>;

	/**
	Networked version of [Owner::retrieve_outputs](struct.Owner.html#method.retrieve_outputs).

	# Json rpc example

	```
	# epic_wallet_api::doctest_helper_json_rpc_owner_assert_response!(
	# r#"
	{
		"jsonrpc": "2.0",
		"method": "retrieve_outputs",
		"params": [false, true, null],
		"id": 1
	}
	# "#
	# ,
	# r#"
	{
		"id": 1,
		"jsonrpc": "2.0",
		"result": {
			"Ok": [
				true,
				[
					{
						"commit": "089be87c488db1e7c783b19272a83b23bce56a5263163554b345c6f7ffedac517e",
						"output": {
							"commit": "089be87c488db1e7c783b19272a83b23bce56a5263163554b345c6f7ffedac517e",
							"height": "1",
							"is_coinbase": true,
							"key_id": "0300000000000000000000000000000000",
							"lock_height": "4",
							"mmr_index": null,
							"n_child": 0,
							"root_key_id": "0200000000000000000000000000000000",
							"status": "Unspent",
							"tx_log_entry": 0,
							"value": "1457920000"
						}
					},
					{
						"commit": "09d8836ffd38ffca42567ef965fdcf1f35b05aeb357664d70cd482438ca0ca0c9e",
						"output": {
							"commit": "09d8836ffd38ffca42567ef965fdcf1f35b05aeb357664d70cd482438ca0ca0c9e",
							"height": "2",
							"is_coinbase": true,
							"key_id": "0300000000000000000000000100000000",
							"lock_height": "5",
							"mmr_index": null,
							"n_child": 1,
							"root_key_id": "0200000000000000000000000000000000",
							"status": "Unspent",
							"tx_log_entry": 1,
							"value": "1457920000"
						}
					}
				]
			]
		}
	}
	# "#
	# , false, 2, false, false, false);
	```
	*/
	fn retrieve_outputs(
		&self,
		include_spent: bool,
		refresh_from_node: bool,
		tx_id: Option<u32>,
	) -> Result<(bool, Vec<OutputCommitMapping>), ErrorKind>;

	/**
	Networked version of [Owner::retrieve_txs](struct.Owner.html#method.retrieve_txs).

	# Json rpc example

	```
		# epic_wallet_api::doctest_helper_json_rpc_owner_assert_response!(
		# r#"
		{
			"jsonrpc": "2.0",
			"method": "retrieve_txs",
			"params": [true, null, null],
			"id": 1
		}
		# "#
		# ,
		# r#"
		{
		"id": 1,
		"jsonrpc": "2.0",
	  "result": {
		"Ok": [
		  true,
		  [
			{
			  "amount_credited": "1457920000",
			  "amount_debited": "0",
			  "confirmation_ts": "2019-01-15T16:01:26Z",
			  "confirmed": true,
			  "creation_ts": "2019-01-15T16:01:26Z",
			  "fee": null,
			  "id": 0,
			  "kernel_excess": "09a89280fa8d888358ab730383f00a3d990b7f2c6b17fc960501f30aac8e014478",
			  "kernel_lookup_min_height": 1,
			  "messages": null,
			  "num_inputs": 0,
			  "num_outputs": 1,
			  "parent_key_id": "0200000000000000000000000000000000",
			  "stored_tx": null,
			  "ttl_cutoff_height": null,
			  "tx_slate_id": null,
			  "payment_proof": null,
			  "tx_type": "ConfirmedCoinbase"
			},
			{
			  "amount_credited": "1457920000",
			  "amount_debited": "0",
			  "confirmation_ts": "2019-01-15T16:01:26Z",
			  "confirmed": true,
			  "creation_ts": "2019-01-15T16:01:26Z",
			  "fee": null,
			  "id": 1,
			  "kernel_excess": "08bae42ff7d5fa5aca058fd0889dd1e40df16bf3ee2eea6e5db720c0a6d638a7f8",
			  "kernel_lookup_min_height": 2,
			  "messages": null,
			  "num_inputs": 0,
			  "num_outputs": 1,
			  "parent_key_id": "0200000000000000000000000000000000",
			  "stored_tx": null,
			  "ttl_cutoff_height": null,
			  "tx_slate_id": null,
			  "payment_proof": null,
			  "tx_type": "ConfirmedCoinbase"
			}
		  ]
		]
	  }
	}
	# "#
	# , false, 2, false, false, false);
	```
	*/

	fn retrieve_txs(
		&self,
		refresh_from_node: bool,
		tx_id: Option<u32>,
		tx_slate_id: Option<Uuid>,
	) -> Result<(bool, Vec<TxLogEntry>), ErrorKind>;

	/**
	Networked version of [Owner::retrieve_summary_info](struct.Owner.html#method.retrieve_summary_info).

	```
	# epic_wallet_api::doctest_helper_json_rpc_owner_assert_response!(
	# r#"
	{
		"jsonrpc": "2.0",
		"method": "retrieve_summary_info",
		"params": [true, 1],
		"id": 1
	}
	# "#
	# ,
	# r#"
	{
	"id": 1,
		"jsonrpc": "2.0",
		"result": {
			"Ok": [
			true,
				{
					"amount_awaiting_confirmation": "0",
					"amount_awaiting_finalization": "0",
					"amount_currently_spendable": "1457920000",
					"amount_immature": "4373760000",
					"amount_locked": "0",
					"last_confirmed_height": "4",
					"minimum_confirmations": "1",
					"total": "5831680000"
				}

			]
		}
	}
	# "#
	# ,false, 4, false, false, false);
	```
	 */

	fn retrieve_summary_info(
		&self,
		refresh_from_node: bool,
		minimum_confirmations: u64,
	) -> Result<(bool, WalletInfo), ErrorKind>;

	/**
		Networked version of [Owner::init_send_tx](struct.Owner.html#method.init_send_tx).

	```
		# epic_wallet_api::doctest_helper_json_rpc_owner_assert_response!(
		# r#"
		{
			"jsonrpc": "2.0",
			"method": "init_send_tx",
			"params": {
				"args": {
					"src_acct_name": null,
					"amount": "60000000",
					"minimum_confirmations": 2,
					"max_outputs": 500,
					"num_change_outputs": 1,
					"selection_strategy_is_use_all": true,
					"message": "my message",
					"target_slate_version": null,
					"payment_proof_recipient_address": null,
					"ttl_blocks": null,
					"send_args": null
				}
			},
			"id": 1
		}
		# "#
		# ,
		# r#"
		{
	  "id": 1,
	  "jsonrpc": "2.0",
	  "result": {
		"Ok": {
		  "amount": "60000000",
		  "fee": "800000",
		  "height": "4",
		  "id": "0436430c-2b02-624c-2032-570501212b00",
		  "lock_height": "0",
			"ttl_cutoff_height": null,
			"payment_proof": null,
		  "num_participants": 2,
		  "participant_data": [
			{
			  "id": "0",
			  "message": "my message",
			  "message_sig": "8f07ddd5e9f5179cff19486034181ed76505baaad53e5d994064127b56c5841bea065fb74c27c31d611427ac5fa1459d1db340d7475e2967f19e2fa95687d88c",
			  "part_sig": null,
			  "public_blind_excess": "039fbac4782fa1600aa704c38073eece85e3a085a90446ded19a9fec90e432b330",
			  "public_nonce": "031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f"
			}
		  ],
		  "tx": {
			"body": {
			  "inputs": [
				{
				  "commit": "089be87c488db1e7c783b19272a83b23bce56a5263163554b345c6f7ffedac517e",
				  "features": "Coinbase"
				}
			  ],
			  "kernels": [
				{
				  "excess": "000000000000000000000000000000000000000000000000000000000000000000",
				  "excess_sig": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
				  "features": "Plain",
				  "fee": "800000",
				  "lock_height": "0"
				}
			  ],
			  "outputs": [
				{
				  "commit": "0832ca73c2049ee0c8f555c6297aa3658eb3f8ce711dfd63d6d5234cf3191c7756",
				  "features": "Plain",
				  "proof": "1393386202b2ba345e131efb1dfc3730c000ee6b6f3cb8d56e3d0680796b11940abeaed75e18458b544d28765b530a7ca2ed170b42c8bd5e07d4451b0cfa59c20fee43c9107091a328a9f745a255702ea17ac50f5e92b2daf51a7851e8ffde5e11a6837b5b6058aee5ab1e68bcd756646f36b38c6262aa20ff9c194d2e3e155b608216186fee66ef8ca269d31a015c63cca3cf5f0d40e4cccd13d390ed1aeed7a7c7d0709e9b509844462098d97e983ba6963ee3a68955d5317ecec47caeb8911784598be71d319ba8441c36388bf4a10ba2538f4f54f648b22a8f1e1f713dec36376936100b0fc5fb5ad4f51811ec96a76b564c3ee08305f5a2ad79a80152a03eb86d4dcd854a23818621d80771ceb169e45a692b45db77667beecd545e08b8afe8f8a3d049ae18e1cee5522769cd6b0131e036ee81d70df4cd679959fd82684bf9e1f4784325ef271eec2fb73ef4a9569fd76f7b1e55d8e2e87a5daa5ad5357cb401af13c2c695afc4b6a8a2004da1b0f5ebe7cb70cb2e15f0f3ca41eaca969abcb452f7a15fe9d004e66ff646e423366713632f1dedcb33bac1abbc47f1cf2b280f04cf85a7291bb4ecb2c1c252d65e933f5819ba4984b1018ec11ae36d2445af56900b9b6e746f84ddd6b06baab9d7c8f82f0b0bc7a61ade6eabe762ac0d3afe4b2102518361a9e54a4d9d51e4a25ccf1d40c36f6444d2271d03d91eb0f1f6895345c8758a7375926cf0ab75212ef7b4a0efa59a31decd995be2933e3da51efec22365521b8942f997789f9618cbbb422607c2414fc64bc558eca27df5fe7156954b98335a5cc63e6bfe7e076149c93e2314dd626f48bf6721b506b81962b6ca81bff28c7e216f49fcbf989045f97452f3b4ccdcaa7ca5a4ce0bd3f5e16440c6c0b73a42bfa6cfe8e31265b73b81b81c2d54e4f7aefb16ebfa1273adbfd57c08a6"
				}
			  ]
			},
			"offset": "d202964900000000d302964900000000d402964900000000d502964900000000"
		  },
		  "version_info": {
				"orig_version": 3,
				"version": 3,
				"block_header_version": 6
		  }
		}
	  }
	}
		# "#
		# ,false, 4, false, false, false);
	```
	*/

	fn init_send_tx(&self, args: InitTxArgs) -> Result<VersionedSlate, ErrorKind>;

	/**
		Networked version of [Owner::issue_invoice_tx](struct.Owner.html#method.issue_invoice_tx).

	```
		# epic_wallet_api::doctest_helper_json_rpc_owner_assert_response!(
		# r#"
		{
			"jsonrpc": "2.0",
			"method": "issue_invoice_tx",
			"params": {
				"args": {
					"amount": "60000000",
					"message": "Please give me your epics",
					"dest_acct_name": null,
					"target_slate_version": null
				}
			},
			"id": 1
		}
		# "#
		# ,
		# r#"
		{
			"id": 1,
			"jsonrpc": "2.0",
			"result": {
				"Ok": {
					"amount": "60000000",
					"fee": "0",
					"height": "4",
					"id": "0436430c-2b02-624c-2032-570501212b00",
					"lock_height": "0",
					"ttl_cutoff_height": null,
					"payment_proof": null,
					"num_participants": 2,
					"participant_data": [
						{
							"id": "1",
							"message": "Please give me your epics",
							"message_sig": "8f07ddd5e9f5179cff19486034181ed76505baaad53e5d994064127b56c5841bb06e3894e0db51e6015d2181f101d06722094128dbc316f7186b57edd68731cb",
							"part_sig": null,
							"public_blind_excess": "035ca9d2d82e0e31bc2add7ef9200066b257d9f72cb16c0e57455277b90e2b3503",
							"public_nonce": "031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f"
						}
					],
					"tx": {
						"body": {
							"inputs": [],
							"kernels": [
								{
									"excess": "000000000000000000000000000000000000000000000000000000000000000000",
									"excess_sig": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
									"features": "Plain",
									"fee": "0",
									"lock_height": "0"
								}
							],
							"outputs": [
								{
									"commit": "083fe68fb96a0941f70e9412b4483621326abee9eccf10e7a7efef2a9b4e97df25",
									"features": "Plain",
									"proof": "2df8ba188baa701a7c07e23de108e8318797ba029319b4557db3c1c8af917f6361d9871520e33423131d5771b06566aba2469f1e9fcc8eda8203a5c241759e45041af26939c57946372bf330d09670eb30a08cae27b3015dac2efb518fbe2325487d8417f4014cb854aec9e4e1770d2f5d7a6e32e2bc904245505ebd952eda10b5c401f315a8cb969da4cc2dc1e656f33d870ca07ffbc45f58cae9f28b836d4bf3c1786b805ba9d8789cc211998981a8b4115aa7383ddbd10e656fc0a590c3e2cf8dd07e414217d9a1d1af32bcbd933448d0a89033cb93ba2eb0d3b973136d61ee7f109d0476ed3475b7328eff5e9d3362b5db4621682a443a382f7ef09304f9bff422885d23f62f9d7d1a9bbf888e5ba5678e347182770cbbf41cfb3002269607f085881ce0f0df01f34f34433ef04dd6008f9a0c13e47e6e386d62151386dfd20bdf812ae2fb580edc38f38bfc9cc543d1023889ee646d4e75a7382caa3bb00b970062ffdc1fc643ce56d25e2e73b556162c8441d5a667b36b840cc244f69395b46900dd1edc562ed741c239804588e94c071b621766b55f738802c376012fa577e0d82bdf7bf2f229a867d91ed177bacde44faadb6901066f84e21a5fb0b73ed7ef9ef4a1e2c65e6a28a0ae834a99ed1694889d885fc8e90c8e7507078603a9705cc3c57b8b0125ad385cb5ec564f9ca69b530307d91ef2c6bb49a39e30d9e68f2f67d99915d87d1a7776f4c0b61913ea661ebe320b8e99919c69d9dbdc527e787d46e772da9ab9f9cc60e43b41fb0981b6b882ed7a535451158c711210fe25e68d12719192c3d332aea9e047a0f7a292b8e6f13fd76ed47afbadf070392cc3f4a4ebb8ec9853587e30ad9b9794717c87bf962e2ab99ec543f5a24efda0cfc2bf51f23c8132aee6058189925febe1d9d3a145f580ef9835db3c1f3b6e97bd36331e"
								}
							]
						},
						"offset": "d202964900000000d302964900000000d402964900000000d502964900000000"
					},
					"version_info": {
						"orig_version": 3,
						"version": 3,
						"block_header_version": 6
					}
				}
			}
		}
		# "#
		# ,false, 4, false, false, false);
	```
	*/

	fn issue_invoice_tx(&self, args: IssueInvoiceTxArgs) -> Result<VersionedSlate, ErrorKind>;

	/**
		 Networked version of [Owner::process_invoice_tx](struct.Owner.html#method.process_invoice_tx).

	```
		# epic_wallet_api::doctest_helper_json_rpc_owner_assert_response!(
		# r#"
		{
			"jsonrpc": "2.0",
			"method": "process_invoice_tx",
			"params": [
				{
					"amount": "60000000",
					"fee": "0",
					"height": "4",
					"id": "0436430c-2b02-624c-2032-570501212b00",
					"lock_height": "0",
					"ttl_cutoff_height": null,
					"payment_proof": null,
					"num_participants": 2,
					"participant_data": [
						{
							"id": "1",
							"message": "Please give me your epics",
							"message_sig": "1b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078fd2599ab38942986602e943f684a85992893a6d34367dc7cc2b403a5dcfcdbcd9",
							"part_sig": null,
							"public_blind_excess": "028e95921cc0d5be5922362265d352c9bdabe51a9e1502a3f0d4a10387f1893f40",
							"public_nonce": "031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f"
						}
					],
					"tx": {
						"body": {
							"inputs": [],
							"kernels": [
								{
									"excess": "000000000000000000000000000000000000000000000000000000000000000000",
									"excess_sig": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
									"features": "Plain",
									"fee": "0",
									"lock_height": "0"
								}
							],
							"outputs": [
								{
									"commit": "09cf47204446c326e361a1a92f34b174deff732daaedb80d7339fbe3db5ca2f6ba",
									"features": "Plain",
									"proof": "8f511614315626b5f39224482351d766f5a8ef136262befc050d839be8479b0a13470cd88f4436346d213d83847a4055c6e0ac63681556470349a1aab47034a3015eb64d8163955998e2dd4165dd24386b1e279974b05deb5d46ba2bc321f7000c0784f8f10690605ffe717119d045e02b141ed12d8fc6d20930483a8af889ef533495eb442fcff36d98ebc104f13fc645c28431b3296e4a11f7c991ff97f9abbc2f8886762d7f29fdacb31d52c6850e6ccf5386117d89e8ea4ca3071c56c218dd5d3bcd65f6c06ed9f51f848507ca1d594f41796d1cf99f68a5c3f0c5dd9873602284cff31269b102fcc6c68607565faaf0adb04ed4ff3ea5d41f3b5235ac6cb90e4046c808c9c48c27172c891b20085c56a99913ef47fd8b3dc4920cef50534b9319a7cefe0df10a0206a634ac837e11da92df83ff58b1a14de81313400988aa48b946fcbe1b81f0e79e13f7c6c639b1c10983b424bda08d0ce593a20f1f47e0aa01473e7144f116b76d9ebc60599053d8f1542d60747793d99064e51fce8f8866390325d48d6e8e3bbdbc1822c864303451525c6cb4c6902f105a70134186fb32110d8192fc2528a9483fc8a4001f4bdeab1dd7b3d1ccb9ae2e746a78013ef74043f0b2436f0ca49627af1768b7c791c669bd331fd18c16ef88ad0a29861db70f2f76f3e74fde5accb91b73573e31333333223693d6fbc786e740c085e4fc6e7bde0a3f54e9703f816c54f012d3b1f41ec4d253d9337af61e7f1f1383bd929421ac346e3d2771dfee0b60503b33938e7c83eb37af3b6bf66041a3519a2b4cb557b34e3b9afcf95524f9a011425a34d32e7b6e9f255291094930acae26e8f7a1e4e6bc405d0f88e919f354f3ba85356a34f1aba5f7da1fad88e2692f4129cc1fb80a2122b2d996c6ccf7f08d8248e511d92af9ce49039de728848a2dc74101f4e94a"
								}
							]
						},
						"offset": "d202964900000000d302964900000000d402964900000000d502964900000000"
					},
					"version_info": {
						"orig_version": 3,
						"version": 3,
						"block_header_version": 6
					}
				},
				{
					"src_acct_name": null,
					"amount": "0",
					"minimum_confirmations": 2,
					"max_outputs": 500,
					"num_change_outputs": 1,
					"selection_strategy_is_use_all": true,
					"message": "Ok, here are your epics",
					"target_slate_version": null,
					"payment_proof_recipient_address": null,
					"ttl_blocks": null,
					"send_args": null
				}
			],
			"id": 1
		}
		# "#
		# ,
		# r#"
		{
		"id": 1,
		"jsonrpc": "2.0",
		"result": {
			"Ok": {
				"amount": "60000000",
				"fee": "800000",
				"height": "4",
				"id": "0436430c-2b02-624c-2032-570501212b00",
				"lock_height": "0",
				"ttl_cutoff_height": null,
				"payment_proof": null,
				"num_participants": 2,
				"participant_data": [
					{
						"id": "1",
						"message": "Please give me your epics",
						"message_sig": "1b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078fd2599ab38942986602e943f684a85992893a6d34367dc7cc2b403a5dcfcdbcd9",
						"part_sig": null,
						"public_blind_excess": "028e95921cc0d5be5922362265d352c9bdabe51a9e1502a3f0d4a10387f1893f40",
						"public_nonce": "031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f"
					},
					{
						"id": "0",
						"message": "Ok, here are your epics",
						"message_sig": "8f07ddd5e9f5179cff19486034181ed76505baaad53e5d994064127b56c5841bec8c1cac6cb5770a3c62c9bb95063581cc08bfccd72dac72be8ec4ba5374a9f3",
						"part_sig": "8f07ddd5e9f5179cff19486034181ed76505baaad53e5d994064127b56c5841bcec20532cbe7ce0a3152b61566785684fea3534b7f834f02f733fa524123ee54",
						"public_blind_excess": "02802124f21ba02769a3f05ecfe9662e8783fa0bd1a7b7d63cf3aea0ebc0d7af3a",
						"public_nonce": "031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f"
					}
				],
				"tx": {
					"body": {
						"inputs": [
							{
								"commit": "089be87c488db1e7c783b19272a83b23bce56a5263163554b345c6f7ffedac517e",
								"features": "Coinbase"
							}
						],
						"kernels": [
							{
								"excess": "000000000000000000000000000000000000000000000000000000000000000000",
								"excess_sig": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
								"features": "Plain",
								"fee": "800000",
								"lock_height": "0"
							}
						],
						"outputs": [
							{
								"commit": "0832ca73c2049ee0c8f555c6297aa3658eb3f8ce711dfd63d6d5234cf3191c7756",
								"features": "Plain",
								"proof": "1393386202b2ba345e131efb1dfc3730c000ee6b6f3cb8d56e3d0680796b11940abeaed75e18458b544d28765b530a7ca2ed170b42c8bd5e07d4451b0cfa59c20fee43c9107091a328a9f745a255702ea17ac50f5e92b2daf51a7851e8ffde5e11a6837b5b6058aee5ab1e68bcd756646f36b38c6262aa20ff9c194d2e3e155b608216186fee66ef8ca269d31a015c63cca3cf5f0d40e4cccd13d390ed1aeed7a7c7d0709e9b509844462098d97e983ba6963ee3a68955d5317ecec47caeb8911784598be71d319ba8441c36388bf4a10ba2538f4f54f648b22a8f1e1f713dec36376936100b0fc5fb5ad4f51811ec96a76b564c3ee08305f5a2ad79a80152a03eb86d4dcd854a23818621d80771ceb169e45a692b45db77667beecd545e08b8afe8f8a3d049ae18e1cee5522769cd6b0131e036ee81d70df4cd679959fd82684bf9e1f4784325ef271eec2fb73ef4a9569fd76f7b1e55d8e2e87a5daa5ad5357cb401af13c2c695afc4b6a8a2004da1b0f5ebe7cb70cb2e15f0f3ca41eaca969abcb452f7a15fe9d004e66ff646e423366713632f1dedcb33bac1abbc47f1cf2b280f04cf85a7291bb4ecb2c1c252d65e933f5819ba4984b1018ec11ae36d2445af56900b9b6e746f84ddd6b06baab9d7c8f82f0b0bc7a61ade6eabe762ac0d3afe4b2102518361a9e54a4d9d51e4a25ccf1d40c36f6444d2271d03d91eb0f1f6895345c8758a7375926cf0ab75212ef7b4a0efa59a31decd995be2933e3da51efec22365521b8942f997789f9618cbbb422607c2414fc64bc558eca27df5fe7156954b98335a5cc63e6bfe7e076149c93e2314dd626f48bf6721b506b81962b6ca81bff28c7e216f49fcbf989045f97452f3b4ccdcaa7ca5a4ce0bd3f5e16440c6c0b73a42bfa6cfe8e31265b73b81b81c2d54e4f7aefb16ebfa1273adbfd57c08a6"
							},
							{
								"commit": "09cf47204446c326e361a1a92f34b174deff732daaedb80d7339fbe3db5ca2f6ba",
								"features": "Plain",
								"proof": "8f511614315626b5f39224482351d766f5a8ef136262befc050d839be8479b0a13470cd88f4436346d213d83847a4055c6e0ac63681556470349a1aab47034a3015eb64d8163955998e2dd4165dd24386b1e279974b05deb5d46ba2bc321f7000c0784f8f10690605ffe717119d045e02b141ed12d8fc6d20930483a8af889ef533495eb442fcff36d98ebc104f13fc645c28431b3296e4a11f7c991ff97f9abbc2f8886762d7f29fdacb31d52c6850e6ccf5386117d89e8ea4ca3071c56c218dd5d3bcd65f6c06ed9f51f848507ca1d594f41796d1cf99f68a5c3f0c5dd9873602284cff31269b102fcc6c68607565faaf0adb04ed4ff3ea5d41f3b5235ac6cb90e4046c808c9c48c27172c891b20085c56a99913ef47fd8b3dc4920cef50534b9319a7cefe0df10a0206a634ac837e11da92df83ff58b1a14de81313400988aa48b946fcbe1b81f0e79e13f7c6c639b1c10983b424bda08d0ce593a20f1f47e0aa01473e7144f116b76d9ebc60599053d8f1542d60747793d99064e51fce8f8866390325d48d6e8e3bbdbc1822c864303451525c6cb4c6902f105a70134186fb32110d8192fc2528a9483fc8a4001f4bdeab1dd7b3d1ccb9ae2e746a78013ef74043f0b2436f0ca49627af1768b7c791c669bd331fd18c16ef88ad0a29861db70f2f76f3e74fde5accb91b73573e31333333223693d6fbc786e740c085e4fc6e7bde0a3f54e9703f816c54f012d3b1f41ec4d253d9337af61e7f1f1383bd929421ac346e3d2771dfee0b60503b33938e7c83eb37af3b6bf66041a3519a2b4cb557b34e3b9afcf95524f9a011425a34d32e7b6e9f255291094930acae26e8f7a1e4e6bc405d0f88e919f354f3ba85356a34f1aba5f7da1fad88e2692f4129cc1fb80a2122b2d996c6ccf7f08d8248e511d92af9ce49039de728848a2dc74101f4e94a"
							}
						]
					},
					"offset": "d202964900000000d302964900000000d402964900000000d502964900000000"
				},
				"version_info": {
					"orig_version": 3,
					"version": 3,
					"block_header_version": 6
				}
			}
		}
	}
	# "#
	# ,false, 4, false, false, false);
	```
	*/

	fn process_invoice_tx(
		&self,
		slate: VersionedSlate,
		args: InitTxArgs,
	) -> Result<VersionedSlate, ErrorKind>;

	/**
	Networked version of [Owner::tx_lock_outputs](struct.Owner.html#method.tx_lock_outputs).

	```
	# epic_wallet_api::doctest_helper_json_rpc_owner_assert_response!(
	# r#"
	{
		"jsonrpc": "2.0",
		"method": "tx_lock_outputs",
		"id": 1,
		"params": [ {
				"amount": "1457920000",
				"fee": "8000000",
				"height": "4",
				"id": "0436430c-2b02-624c-2032-570501212b00",
				"lock_height": "4",
				"ttl_cutoff_height": null,
				"payment_proof": null,
				"num_participants": 2,
				"participant_data": [
				{
					"id": "0",
					"message": "my message",
					"message_sig": "8f07ddd5e9f5179cff19486034181ed76505baaad53e5d994064127b56c5841b1d4c1358be398f801eb90d933774b5218fa7e769b11c4c640402253353656f75",
					"part_sig": null,
					"public_blind_excess": "034b4df2f0558b73ea72a1ca5c4ab20217c66bbe0829056fca7abe76888e9349ee",
					"public_nonce": "031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f"
				}
				],
				"tx": {
					"body": {
						"inputs": [
						{
							"commit": "08e1da9e6dc4d6e808a718b2f110a991dd775d65ce5ae408a4e1f002a4961aa9e7",
							"features": "Coinbase"
						}
						],
						"kernels": [
						{
							"excess": "000000000000000000000000000000000000000000000000000000000000000000",
							"excess_sig": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
							"features": "HeightLocked",
							"fee": "8000000",
							"lock_height": "4"
						}
						],
						"outputs": [
						{
							"commit": "094be57c91787fc2033d5d97fae099f1a6ddb37ea48370f1a138f09524c767fdd3",
							"features": "Plain",
							"proof": "2a42e9e902b70ce44e1fccb14de87ee0a97100bddf12c6bead1b9c5f4eb60300f29c13094fa12ffeee238fb4532b18f6b61cf51b23c1c7e1ad2e41560dc27edc0a2b9e647a0b3e4e806fced5b65e61d0f1f5197d3e2285c632d359e27b6b9206b2caffea4f67e0c7a2812e7a22c134b98cf89bd43d9f28b8bec25cce037a0ac5b1ae8f667e54e1250813a5263004486b4465ad4e641ab2b535736ea26535a11013564f08f483b7dab1c2bcc3ee38eadf2f7850eff7e3459a4bbabf9f0cf6c50d0c0a4120565cd4a2ce3e354c11721cd695760a24c70e0d5a0dfc3c5dcd51dfad6de2c237a682f36dc0b271f21bb3655e5333016aaa42c2efa1446e5f3c0a79ec417c4d30f77556951cb0f05dbfafb82d9f95951a9ea241fda2a6388f73ace036b98acce079f0e4feebccc96290a86dcc89118a901210b245f2d114cf94396e4dbb461e82aa26a0581389707957968c7cdc466213bb1cd417db207ef40c05842ab67a01a9b96eb1430ebc26e795bb491258d326d5174ad549401059e41782121e506744af8af9d8e493644a87d613600888541cbbe538c625883f3eb4aa3102c5cfcc25de8e97af8927619ce6a731b3b8462d51d993066b935b0648d2344ad72e4fd70f347fbd81041042e5ea31cc7b2e3156a920b80ecba487b950ca32ca95fae85b759c936246ecf441a9fdd95e8fee932d6782cdec686064018c857efc47fb4b2a122600d5fdd79af2486f44df7e629184e1c573bc0a9b3feb40b190ef2861a1ab45e2ac2201b9cd42e495deea247269820ed32389a2810ad6c0f9a296d2a2d9c54089fed50b7f5ecfcd33ab9954360e1d7f5598c32128cfcf2a1d8bf14616818da8a5343bfa88f0eedf392e9d4ab1ace1b60324129cd4852c2e27813a9cf71a6ae6229a4fcecc1a756b3e664c5f50af333082616815a3bec8fc0b75b8e4e767d719"
						}
						]
					},
					"offset": "d202964900000000d302964900000000d402964900000000d502964900000000"
				},
				"version_info": {
					"orig_version": 3,
					"version": 3,
					"block_header_version": 6
				}
			},
			0
		]
	}
	# "#
	# ,
	# r#"
	{
		"jsonrpc": "2.0",
		"id": 1,
		"result": {
			"Ok": null
		}
	}
	# "#
	# ,false, 5 ,true, false, false);

	```
	 */
	fn tx_lock_outputs(
		&self,
		slate: VersionedSlate,
		participant_id: usize,
	) -> Result<(), ErrorKind>;

	/**
	Networked version of [Owner::finalize_tx](struct.Owner.html#method.finalize_tx).

	```
	# epic_wallet_api::doctest_helper_json_rpc_owner_assert_response!(
	# r#"
	{
		"jsonrpc": "2.0",
		"method": "finalize_tx",
		"id": 1,
		"params": [
		{
			"version_info": {
				"version": 3,
				"orig_version": 3,
				"block_header_version": 6
			},
			"num_participants": 2,
			"id": "0436430c-2b02-624c-2032-570501212b00",
			"ttl_cutoff_height": null,
			"payment_proof": null,
			"tx": {
				"offset": "d202964900000000d302964900000000d402964900000000d502964900000000",
				"body": {
					"inputs": [
						{
							"features": "Coinbase",
							"commit": "09d8836ffd38ffca42567ef965fdcf1f35b05aeb357664d70cd482438ca0ca0c9e"
						},
						{
							"features": "Coinbase",
							"commit": "089be87c488db1e7c783b19272a83b23bce56a5263163554b345c6f7ffedac517e"
						}
					],
					"outputs": [
						{
							"features": "Plain",
							"commit": "091454e23b4dbc71f546a41035d69f4c87d0f6efb5ceb119cc0d2eef80ba1928d7",
							"proof": "1a32a93de1dad833b4ae66d042784c435f60ac452f769d2d778772b3e2f2ca9fb16191636222b35866f273935f657ff37e1d38b877e12b7bcce98b1aa71f47a701b9ed8c648e2d6ab18ac0f8f7cf4a7c0aebb2c15681a684ec6f4d385e5db20e7bf9e6f3d8554ada1b82ac2fa9b77cb0a4c4c6b6c740d938fc0c6031a1cc0c0839701e6dab439c4dcdb32ca87d510b582efbabe8f8b783a330bc2c4451d1c2949a6ad901d40f7abc6103fadebba22016a955eaec4a0215398afbc7d22a4ad5bf3103446f4fe5440ded3bd9db607a69b8ca7c005c09e82fa367febc532b8d5c573e2bcc65a972bf76cea98943d9baaf209c84b4b70d56444c22cd334c7299000122de110f957b7af1f4d7f3816e053c94731113fd098bd2c0ccbe4c19152dd07a8d137b453e5a9d19cca576b494f448c5673babf9122297e4d2f4bd4a5a768c4da040527816d6ff91edb57da4053df167a44d2d5cf194bf30a47bcdd4ff541638b3db02e8ac882fb00767bf50fe5bf1b6077c8ad4f163ce75f21c99f708a9bcc0676034351e5ca68894550fcca5ee868d3d9d87e164612f09c79f2676a4acd8a8266e0f794c49318f8a1595ee1ff4e55e9cf5f3361cc473a032bd3bbd36a085f0c03f9b451b472d6a6a7ea9d858fd42a49c2e68c25bf8f18dd8e691168fe6f10602c6ec04cbc2601afa479294da84ecb79bc9b225d8758f682a2df52882c586ead779711258a9443e43365df9d326ca9052615ce33efac4bd0452a18d5b294b9fcf86e860786a692bfbd84a8bf3a751adedd978b969177cd8897871c43cd28df40a4beefcc86b10e6822ba18673f396294c799e756c8a5f03c92499127ec567e9f5b794442c63be7119ce741e4e056f502ca4809f7c76dd6dad754a1b31201ca2e2540e125637e1da5d16c61e3bea90ded06892076268893c167e0faed26172f304900e"
						},
						{
							"features": "Plain",
							"commit": "09414416856d650cd42abad97943f8ea32ff19e7d5d10201ff790d1ca941f578ed",
							"proof": "bdd12075099d53912b42073fd9c2841f2e21dff01656e7f909e1bbd30ada9a18b2f645128676ecddaecbffdcce43e9ff0e850acbce0f9a1e3fc525a7424d09040da752a8db0c8173f31ec4696bf007bf76801f63cedeadc66f4198836494de20a3d48150776c819d2e0a8ef376622d8a1cef78cd6928b3aa38883f51594fa50c3a772c539071c1c05ac4fce08768076618e2d5c7b3d46e28f1459f84f143a943957a4294011b093caf0e077020caf0668b379525df35f626641be6e81d7b711f1b32a98596c1829b9671d574f793e7f9f08c9118bdda60577053456caace5071cc14b10a67205e1c263bb53990fcf4fbcaea9cae652bd9e7ad6c1573ff96cd9271ecf0fabb895cea13b80d59bf7093fa03907911f526cb60df2bf0d3e2d4b81be4bbae55c466d6b221fa70cb145e6550e37856d080304e104fb23be97ae1499b4d3a2f7a4550545a03c20d374c081ac4f592477e23a20f418bcc59d9b02f665b898400a74350b88d793d383a0dc57618d58711e85e221383abb170c4a7f1640f30f2fc8258074f882b56453befecf3a61ed194a8ad98d1f6ab38c565b7cde60a7bb258066d9c5363c6bd618a9b3473b70a516ad4a67c2571e62fec4970eb4df902143aa130d333825f0a4cde9f93d8249c32f26bfadb26be8a5ceb6b5b6cdd076baa1cbde1973d83e64a1b35075dba69682e51cedfb82484276d56cf9e0601a272c0148ce070c6019ab2882405900164871f6b59d2c2a9f5d92674fe58cd9e036752eae8fb58e0fc29e3d59330ac92c1f263988f67add07a22770c381f29a602785244dbd46e4416ca56f25fe0cdd21714bcdf58c28329e22124247416b8de61297b6bd1630b93692a3a81c3107689f35cf4be5a8472b31552973ef2bcee5a298a858a768eefd0e31a3936790dd1c6e1379fffa0235c188b2c0f8b8b41abb84c32c608"
						}
					],
					"kernels": [
						{
							"features": "Plain",
							"fee": "700000",
							"lock_height": "0",
							"excess": "000000000000000000000000000000000000000000000000000000000000000000",
							"excess_sig": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
						}
					]
				}
			},
			"amount": "600000000",
			"fee": "700000",
			"height": "5",
			"lock_height": "0",
			"participant_data": [
				{
					"id": "0",
					"public_blind_excess": "028e1bbb43e6038efc42054778d0a1aa184b2cf4d51acb40a2a8dc049788d97bd2",
					"public_nonce": "031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f",
					"part_sig": null,
					"message": null,
					"message_sig": null
				},
				{
					"id": "1",
					"public_blind_excess": "03e14bacb4cfeda43edf6c1b0ffced9a358a119c7936bc68af724477eb91d9e4eb",
					"public_nonce": "031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f",
					"part_sig": "8f07ddd5e9f5179cff19486034181ed76505baaad53e5d994064127b56c5841b129340313ed7db02f6fd9a16c23ae8d5801af4fdc2ea580e2dec26e3821d5c17",
					"message": null,
					"message_sig": null
				}
			]
		}
		]
	}
	# "#
	# ,
	# r#"
	{
		"jsonrpc": "2.0",
		"id": 1,
		"result": {
		"Ok": {
				"amount": "600000000",
				"fee": "700000",
				"height": "5",
				"id": "0436430c-2b02-624c-2032-570501212b00",
				"ttl_cutoff_height": null,
				"payment_proof": null,
				"lock_height": "0",
				"num_participants": 2,
				"participant_data": [
					{
						"id": "0",
						"message": null,
						"message_sig": null,
						"part_sig": "8f07ddd5e9f5179cff19486034181ed76505baaad53e5d994064127b56c5841b93f888685e13250c5cb6b830ff898264ce247c73d3ab47845c01bcc6455ecbe5",
						"public_blind_excess": "028e1bbb43e6038efc42054778d0a1aa184b2cf4d51acb40a2a8dc049788d97bd2",
						"public_nonce": "031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f"
					},
					{
						"id": "1",
						"message": null,
						"message_sig": null,
						"part_sig": "8f07ddd5e9f5179cff19486034181ed76505baaad53e5d994064127b56c5841b129340313ed7db02f6fd9a16c23ae8d5801af4fdc2ea580e2dec26e3821d5c17",
						"public_blind_excess": "03e14bacb4cfeda43edf6c1b0ffced9a358a119c7936bc68af724477eb91d9e4eb",
						"public_nonce": "031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f"
					}
				],
				"tx": {
					"body": {
						"inputs": [
							{
								"commit": "09d8836ffd38ffca42567ef965fdcf1f35b05aeb357664d70cd482438ca0ca0c9e",
								"features": "Coinbase"
							},
							{
								"commit": "089be87c488db1e7c783b19272a83b23bce56a5263163554b345c6f7ffedac517e",
								"features": "Coinbase"
							}
						],
						"kernels": [
							{
								"excess": "08d09187cb93cf5d6b97b28e8ca529912bf35ec8773d3e9af9b3c174a270dc7f05",
								"excess_sig": "66074d25a751c4743342c90ad8ead9454daa00d9b9aed29bca321036d16c4b4da58bc9999cea000f52b45347c1c46a3a4f3f70719696a09289ede2a9c87b27fd",
								"features": "Plain",
								"fee": "700000",
								"lock_height": "0"
							}
						],
						"outputs": [
							{
								"commit": "091454e23b4dbc71f546a41035d69f4c87d0f6efb5ceb119cc0d2eef80ba1928d7",
								"features": "Plain",
								"proof": "1a32a93de1dad833b4ae66d042784c435f60ac452f769d2d778772b3e2f2ca9fb16191636222b35866f273935f657ff37e1d38b877e12b7bcce98b1aa71f47a701b9ed8c648e2d6ab18ac0f8f7cf4a7c0aebb2c15681a684ec6f4d385e5db20e7bf9e6f3d8554ada1b82ac2fa9b77cb0a4c4c6b6c740d938fc0c6031a1cc0c0839701e6dab439c4dcdb32ca87d510b582efbabe8f8b783a330bc2c4451d1c2949a6ad901d40f7abc6103fadebba22016a955eaec4a0215398afbc7d22a4ad5bf3103446f4fe5440ded3bd9db607a69b8ca7c005c09e82fa367febc532b8d5c573e2bcc65a972bf76cea98943d9baaf209c84b4b70d56444c22cd334c7299000122de110f957b7af1f4d7f3816e053c94731113fd098bd2c0ccbe4c19152dd07a8d137b453e5a9d19cca576b494f448c5673babf9122297e4d2f4bd4a5a768c4da040527816d6ff91edb57da4053df167a44d2d5cf194bf30a47bcdd4ff541638b3db02e8ac882fb00767bf50fe5bf1b6077c8ad4f163ce75f21c99f708a9bcc0676034351e5ca68894550fcca5ee868d3d9d87e164612f09c79f2676a4acd8a8266e0f794c49318f8a1595ee1ff4e55e9cf5f3361cc473a032bd3bbd36a085f0c03f9b451b472d6a6a7ea9d858fd42a49c2e68c25bf8f18dd8e691168fe6f10602c6ec04cbc2601afa479294da84ecb79bc9b225d8758f682a2df52882c586ead779711258a9443e43365df9d326ca9052615ce33efac4bd0452a18d5b294b9fcf86e860786a692bfbd84a8bf3a751adedd978b969177cd8897871c43cd28df40a4beefcc86b10e6822ba18673f396294c799e756c8a5f03c92499127ec567e9f5b794442c63be7119ce741e4e056f502ca4809f7c76dd6dad754a1b31201ca2e2540e125637e1da5d16c61e3bea90ded06892076268893c167e0faed26172f304900e"
							},
							{
								"commit": "09414416856d650cd42abad97943f8ea32ff19e7d5d10201ff790d1ca941f578ed",
								"features": "Plain",
								"proof": "bdd12075099d53912b42073fd9c2841f2e21dff01656e7f909e1bbd30ada9a18b2f645128676ecddaecbffdcce43e9ff0e850acbce0f9a1e3fc525a7424d09040da752a8db0c8173f31ec4696bf007bf76801f63cedeadc66f4198836494de20a3d48150776c819d2e0a8ef376622d8a1cef78cd6928b3aa38883f51594fa50c3a772c539071c1c05ac4fce08768076618e2d5c7b3d46e28f1459f84f143a943957a4294011b093caf0e077020caf0668b379525df35f626641be6e81d7b711f1b32a98596c1829b9671d574f793e7f9f08c9118bdda60577053456caace5071cc14b10a67205e1c263bb53990fcf4fbcaea9cae652bd9e7ad6c1573ff96cd9271ecf0fabb895cea13b80d59bf7093fa03907911f526cb60df2bf0d3e2d4b81be4bbae55c466d6b221fa70cb145e6550e37856d080304e104fb23be97ae1499b4d3a2f7a4550545a03c20d374c081ac4f592477e23a20f418bcc59d9b02f665b898400a74350b88d793d383a0dc57618d58711e85e221383abb170c4a7f1640f30f2fc8258074f882b56453befecf3a61ed194a8ad98d1f6ab38c565b7cde60a7bb258066d9c5363c6bd618a9b3473b70a516ad4a67c2571e62fec4970eb4df902143aa130d333825f0a4cde9f93d8249c32f26bfadb26be8a5ceb6b5b6cdd076baa1cbde1973d83e64a1b35075dba69682e51cedfb82484276d56cf9e0601a272c0148ce070c6019ab2882405900164871f6b59d2c2a9f5d92674fe58cd9e036752eae8fb58e0fc29e3d59330ac92c1f263988f67add07a22770c381f29a602785244dbd46e4416ca56f25fe0cdd21714bcdf58c28329e22124247416b8de61297b6bd1630b93692a3a81c3107689f35cf4be5a8472b31552973ef2bcee5a298a858a768eefd0e31a3936790dd1c6e1379fffa0235c188b2c0f8b8b41abb84c32c608"
							}
						]
					},
					"offset": "d202964900000000d302964900000000d402964900000000d502964900000000"
				},
				"version_info": {
					"orig_version": 3,
					"version": 3,
					"block_header_version": 6
				}
			}
		}
	}
	# "#
	# , false, 5, true, true, false);
	```
	 */
	fn finalize_tx(&self, slate: VersionedSlate) -> Result<VersionedSlate, ErrorKind>;

	/**
	Networked version of [Owner::post_tx](struct.Owner.html#method.post_tx).

	```
	# epic_wallet_api::doctest_helper_json_rpc_owner_assert_response!(
	# r#"
	{
		"jsonrpc": "2.0",
		"id": 1,
		"method": "post_tx",
		"params": [
		{
			"offset": "d202964900000000d302964900000000d402964900000000d502964900000000",
			"body": {
				"inputs": [
					{
						"features": "Coinbase",
						"commit": "09d8836ffd38ffca42567ef965fdcf1f35b05aeb357664d70cd482438ca0ca0c9e"
					},
					{
						"features": "Coinbase",
						"commit": "089be87c488db1e7c783b19272a83b23bce56a5263163554b345c6f7ffedac517e"
					}
				],
				"kernels": [
					{
						"features": "Plain",
						"fee": "700000",
						"lock_height": "0",
						"excess": "08d09187cb93cf5d6b97b28e8ca529912bf35ec8773d3e9af9b3c174a270dc7f05",
						"excess_sig": "66074d25a751c4743342c90ad8ead9454daa00d9b9aed29bca321036d16c4b4da58bc9999cea000f52b45347c1c46a3a4f3f70719696a09289ede2a9c87b27fd"
					}
				],
				"outputs": [
					{
						"features": "Plain",
						"commit": "091454e23b4dbc71f546a41035d69f4c87d0f6efb5ceb119cc0d2eef80ba1928d7",
						"proof": "1a32a93de1dad833b4ae66d042784c435f60ac452f769d2d778772b3e2f2ca9fb16191636222b35866f273935f657ff37e1d38b877e12b7bcce98b1aa71f47a701b9ed8c648e2d6ab18ac0f8f7cf4a7c0aebb2c15681a684ec6f4d385e5db20e7bf9e6f3d8554ada1b82ac2fa9b77cb0a4c4c6b6c740d938fc0c6031a1cc0c0839701e6dab439c4dcdb32ca87d510b582efbabe8f8b783a330bc2c4451d1c2949a6ad901d40f7abc6103fadebba22016a955eaec4a0215398afbc7d22a4ad5bf3103446f4fe5440ded3bd9db607a69b8ca7c005c09e82fa367febc532b8d5c573e2bcc65a972bf76cea98943d9baaf209c84b4b70d56444c22cd334c7299000122de110f957b7af1f4d7f3816e053c94731113fd098bd2c0ccbe4c19152dd07a8d137b453e5a9d19cca576b494f448c5673babf9122297e4d2f4bd4a5a768c4da040527816d6ff91edb57da4053df167a44d2d5cf194bf30a47bcdd4ff541638b3db02e8ac882fb00767bf50fe5bf1b6077c8ad4f163ce75f21c99f708a9bcc0676034351e5ca68894550fcca5ee868d3d9d87e164612f09c79f2676a4acd8a8266e0f794c49318f8a1595ee1ff4e55e9cf5f3361cc473a032bd3bbd36a085f0c03f9b451b472d6a6a7ea9d858fd42a49c2e68c25bf8f18dd8e691168fe6f10602c6ec04cbc2601afa479294da84ecb79bc9b225d8758f682a2df52882c586ead779711258a9443e43365df9d326ca9052615ce33efac4bd0452a18d5b294b9fcf86e860786a692bfbd84a8bf3a751adedd978b969177cd8897871c43cd28df40a4beefcc86b10e6822ba18673f396294c799e756c8a5f03c92499127ec567e9f5b794442c63be7119ce741e4e056f502ca4809f7c76dd6dad754a1b31201ca2e2540e125637e1da5d16c61e3bea90ded06892076268893c167e0faed26172f304900e"
					},
					{
						"features": "Plain",
						"commit": "09414416856d650cd42abad97943f8ea32ff19e7d5d10201ff790d1ca941f578ed",
						"proof": "bdd12075099d53912b42073fd9c2841f2e21dff01656e7f909e1bbd30ada9a18b2f645128676ecddaecbffdcce43e9ff0e850acbce0f9a1e3fc525a7424d09040da752a8db0c8173f31ec4696bf007bf76801f63cedeadc66f4198836494de20a3d48150776c819d2e0a8ef376622d8a1cef78cd6928b3aa38883f51594fa50c3a772c539071c1c05ac4fce08768076618e2d5c7b3d46e28f1459f84f143a943957a4294011b093caf0e077020caf0668b379525df35f626641be6e81d7b711f1b32a98596c1829b9671d574f793e7f9f08c9118bdda60577053456caace5071cc14b10a67205e1c263bb53990fcf4fbcaea9cae652bd9e7ad6c1573ff96cd9271ecf0fabb895cea13b80d59bf7093fa03907911f526cb60df2bf0d3e2d4b81be4bbae55c466d6b221fa70cb145e6550e37856d080304e104fb23be97ae1499b4d3a2f7a4550545a03c20d374c081ac4f592477e23a20f418bcc59d9b02f665b898400a74350b88d793d383a0dc57618d58711e85e221383abb170c4a7f1640f30f2fc8258074f882b56453befecf3a61ed194a8ad98d1f6ab38c565b7cde60a7bb258066d9c5363c6bd618a9b3473b70a516ad4a67c2571e62fec4970eb4df902143aa130d333825f0a4cde9f93d8249c32f26bfadb26be8a5ceb6b5b6cdd076baa1cbde1973d83e64a1b35075dba69682e51cedfb82484276d56cf9e0601a272c0148ce070c6019ab2882405900164871f6b59d2c2a9f5d92674fe58cd9e036752eae8fb58e0fc29e3d59330ac92c1f263988f67add07a22770c381f29a602785244dbd46e4416ca56f25fe0cdd21714bcdf58c28329e22124247416b8de61297b6bd1630b93692a3a81c3107689f35cf4be5a8472b31552973ef2bcee5a298a858a768eefd0e31a3936790dd1c6e1379fffa0235c188b2c0f8b8b41abb84c32c608"
					}
				]

			}
		},
		false
		]
	}
	# "#
	# ,
	# r#"
	{
		"id": 1,
		"jsonrpc": "2.0",
		"result": {
			"Ok": null
		}
	}
	# "#
	# , false, 5, true, true, true);
	```
	 */

	fn post_tx(&self, tx: TransactionV3, fluff: bool) -> Result<(), ErrorKind>;

	/**
	Networked version of [Owner::cancel_tx](struct.Owner.html#method.cancel_tx).


	```
	# epic_wallet_api::doctest_helper_json_rpc_owner_assert_response!(
	# r#"
	{
		"jsonrpc": "2.0",
		"method": "cancel_tx",
		"params": [null, "0436430c-2b02-624c-2032-570501212b00"],
		"id": 1
	}
	# "#
	# ,
	# r#"
	{
		"id": 1,
		"jsonrpc": "2.0",
		"result": {
			"Ok": null
		}
	}
	# "#
	# , false, 5, true, true, false);
	```
	 */
	fn cancel_tx(&self, tx_id: Option<u32>, tx_slate_id: Option<Uuid>) -> Result<(), ErrorKind>;

	/**
	Networked version of [Owner::get_stored_tx](struct.Owner.html#method.get_stored_tx).

	```
	# epic_wallet_api::doctest_helper_json_rpc_owner_assert_response!(
	# r#"
	{
		"jsonrpc": "2.0",
		"method": "get_stored_tx",
		"id": 1,
		"params": [
			{
				"amount_credited": "59993000000",
				"amount_debited": "120000000000",
				"confirmation_ts": "2019-01-15T16:01:26Z",
				"confirmed": false,
				"creation_ts": "2019-01-15T16:01:26Z",
				"fee": "7000000",
				"id": 5,
				"messages": {
					"messages": [
						{
							"id": "0",
							"message": null,
							"message_sig": null,
							"public_key": "033ac2158fa0077f087de60c19d8e431753baa5b63b6e1477f05a2a6e7190d4592"
						},
						{
							"id": "1",
							"message": null,
							"message_sig": null,
							"public_key": "024f9bc78c984c78d6e916d3a00746aa30fa1172124c8dbc0cbddcb7b486719bc7"
						}
					]
				},
				"num_inputs": 2,
				"num_outputs": 1,
				"parent_key_id": "0200000000000000000000000000000000",
				"stored_tx": "0436430c-2b02-624c-2032-570501212b00.epictx",
				"tx_slate_id": "0436430c-2b02-624c-2032-570501212b00",
				"tx_type": "TxSent",
				"kernel_excess": null,
				"kernel_lookup_min_height": null
			}
		]
	}
	# "#
	# ,
	# r#"
	{
		"jsonrpc": "2.0",
		"id": 1,
		"result": {
			"Ok": {
				"body": {
					"inputs": [
						{
							"commit": "09d8836ffd38ffca42567ef965fdcf1f35b05aeb357664d70cd482438ca0ca0c9e",
							"features": "Coinbase"
						},
						{
							"commit": "089be87c488db1e7c783b19272a83b23bce56a5263163554b345c6f7ffedac517e",
							"features": "Coinbase"
						}
					],
					"kernels": [
						{
							"excess": "000000000000000000000000000000000000000000000000000000000000000000",
							"excess_sig": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
							"features": "Plain",
							"fee": "700000",
							"lock_height": "0"
						}
					],
					"outputs": [
						{
							"commit": "091454e23b4dbc71f546a41035d69f4c87d0f6efb5ceb119cc0d2eef80ba1928d7",
							"features": "Plain",
							"proof": "1a32a93de1dad833b4ae66d042784c435f60ac452f769d2d778772b3e2f2ca9fb16191636222b35866f273935f657ff37e1d38b877e12b7bcce98b1aa71f47a701b9ed8c648e2d6ab18ac0f8f7cf4a7c0aebb2c15681a684ec6f4d385e5db20e7bf9e6f3d8554ada1b82ac2fa9b77cb0a4c4c6b6c740d938fc0c6031a1cc0c0839701e6dab439c4dcdb32ca87d510b582efbabe8f8b783a330bc2c4451d1c2949a6ad901d40f7abc6103fadebba22016a955eaec4a0215398afbc7d22a4ad5bf3103446f4fe5440ded3bd9db607a69b8ca7c005c09e82fa367febc532b8d5c573e2bcc65a972bf76cea98943d9baaf209c84b4b70d56444c22cd334c7299000122de110f957b7af1f4d7f3816e053c94731113fd098bd2c0ccbe4c19152dd07a8d137b453e5a9d19cca576b494f448c5673babf9122297e4d2f4bd4a5a768c4da040527816d6ff91edb57da4053df167a44d2d5cf194bf30a47bcdd4ff541638b3db02e8ac882fb00767bf50fe5bf1b6077c8ad4f163ce75f21c99f708a9bcc0676034351e5ca68894550fcca5ee868d3d9d87e164612f09c79f2676a4acd8a8266e0f794c49318f8a1595ee1ff4e55e9cf5f3361cc473a032bd3bbd36a085f0c03f9b451b472d6a6a7ea9d858fd42a49c2e68c25bf8f18dd8e691168fe6f10602c6ec04cbc2601afa479294da84ecb79bc9b225d8758f682a2df52882c586ead779711258a9443e43365df9d326ca9052615ce33efac4bd0452a18d5b294b9fcf86e860786a692bfbd84a8bf3a751adedd978b969177cd8897871c43cd28df40a4beefcc86b10e6822ba18673f396294c799e756c8a5f03c92499127ec567e9f5b794442c63be7119ce741e4e056f502ca4809f7c76dd6dad754a1b31201ca2e2540e125637e1da5d16c61e3bea90ded06892076268893c167e0faed26172f304900e"
						},
						{
							"commit": "09414416856d650cd42abad97943f8ea32ff19e7d5d10201ff790d1ca941f578ed",
							"features": "Plain",
							"proof": "bdd12075099d53912b42073fd9c2841f2e21dff01656e7f909e1bbd30ada9a18b2f645128676ecddaecbffdcce43e9ff0e850acbce0f9a1e3fc525a7424d09040da752a8db0c8173f31ec4696bf007bf76801f63cedeadc66f4198836494de20a3d48150776c819d2e0a8ef376622d8a1cef78cd6928b3aa38883f51594fa50c3a772c539071c1c05ac4fce08768076618e2d5c7b3d46e28f1459f84f143a943957a4294011b093caf0e077020caf0668b379525df35f626641be6e81d7b711f1b32a98596c1829b9671d574f793e7f9f08c9118bdda60577053456caace5071cc14b10a67205e1c263bb53990fcf4fbcaea9cae652bd9e7ad6c1573ff96cd9271ecf0fabb895cea13b80d59bf7093fa03907911f526cb60df2bf0d3e2d4b81be4bbae55c466d6b221fa70cb145e6550e37856d080304e104fb23be97ae1499b4d3a2f7a4550545a03c20d374c081ac4f592477e23a20f418bcc59d9b02f665b898400a74350b88d793d383a0dc57618d58711e85e221383abb170c4a7f1640f30f2fc8258074f882b56453befecf3a61ed194a8ad98d1f6ab38c565b7cde60a7bb258066d9c5363c6bd618a9b3473b70a516ad4a67c2571e62fec4970eb4df902143aa130d333825f0a4cde9f93d8249c32f26bfadb26be8a5ceb6b5b6cdd076baa1cbde1973d83e64a1b35075dba69682e51cedfb82484276d56cf9e0601a272c0148ce070c6019ab2882405900164871f6b59d2c2a9f5d92674fe58cd9e036752eae8fb58e0fc29e3d59330ac92c1f263988f67add07a22770c381f29a602785244dbd46e4416ca56f25fe0cdd21714bcdf58c28329e22124247416b8de61297b6bd1630b93692a3a81c3107689f35cf4be5a8472b31552973ef2bcee5a298a858a768eefd0e31a3936790dd1c6e1379fffa0235c188b2c0f8b8b41abb84c32c608"
						}
					]
				},
				"offset": "d202964900000000d302964900000000d402964900000000d502964900000000"
			}
		}
	}
	# "#
	# , false, 5, true, true, false);
	```
	 */
	fn get_stored_tx(&self, tx: &TxLogEntry) -> Result<Option<TransactionV3>, ErrorKind>;

	/**
	Networked version of [Owner::verify_slate_messages](struct.Owner.html#method.verify_slate_messages).

	```
	# epic_wallet_api::doctest_helper_json_rpc_owner_assert_response!(
	# r#"
	{
		"jsonrpc": "2.0",
		"method": "verify_slate_messages",
		"id": 1,
		"params": [ {
				"amount": "1457920000",
				"fee": "8000000",
				"height": "4",
				"id": "0436430c-2b02-624c-2032-570501212b00",
				"lock_height": "4",
				"ttl_cutoff_height": null,
				"payment_proof": null,
				"num_participants": 2,
				"participant_data": [
				{
					"id": "0",
					"message": "my message",
					"message_sig": "8f07ddd5e9f5179cff19486034181ed76505baaad53e5d994064127b56c5841b1d4c1358be398f801eb90d933774b5218fa7e769b11c4c640402253353656f75",
					"part_sig": null,
					"public_blind_excess": "034b4df2f0558b73ea72a1ca5c4ab20217c66bbe0829056fca7abe76888e9349ee",
					"public_nonce": "031b84c5567b126440995d3ed5aaba0565d71e1834604819ff9c17f5e9d5dd078f"
				}
				],
				"tx": {
					"body": {
						"inputs": [
						{
							"commit": "08e1da9e6dc4d6e808a718b2f110a991dd775d65ce5ae408a4e1f002a4961aa9e7",
							"features": "Coinbase"
						}
						],
						"kernels": [
						{
							"excess": "000000000000000000000000000000000000000000000000000000000000000000",
							"excess_sig": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
							"features": "HeightLocked",
							"fee": "8000000",
							"lock_height": "4"
						}
						],
						"outputs": [
						{
							"commit": "094be57c91787fc2033d5d97fae099f1a6ddb37ea48370f1a138f09524c767fdd3",
							"features": "Plain",
							"proof": "2a42e9e902b70ce44e1fccb14de87ee0a97100bddf12c6bead1b9c5f4eb60300f29c13094fa12ffeee238fb4532b18f6b61cf51b23c1c7e1ad2e41560dc27edc0a2b9e647a0b3e4e806fced5b65e61d0f1f5197d3e2285c632d359e27b6b9206b2caffea4f67e0c7a2812e7a22c134b98cf89bd43d9f28b8bec25cce037a0ac5b1ae8f667e54e1250813a5263004486b4465ad4e641ab2b535736ea26535a11013564f08f483b7dab1c2bcc3ee38eadf2f7850eff7e3459a4bbabf9f0cf6c50d0c0a4120565cd4a2ce3e354c11721cd695760a24c70e0d5a0dfc3c5dcd51dfad6de2c237a682f36dc0b271f21bb3655e5333016aaa42c2efa1446e5f3c0a79ec417c4d30f77556951cb0f05dbfafb82d9f95951a9ea241fda2a6388f73ace036b98acce079f0e4feebccc96290a86dcc89118a901210b245f2d114cf94396e4dbb461e82aa26a0581389707957968c7cdc466213bb1cd417db207ef40c05842ab67a01a9b96eb1430ebc26e795bb491258d326d5174ad549401059e41782121e506744af8af9d8e493644a87d613600888541cbbe538c625883f3eb4aa3102c5cfcc25de8e97af8927619ce6a731b3b8462d51d993066b935b0648d2344ad72e4fd70f347fbd81041042e5ea31cc7b2e3156a920b80ecba487b950ca32ca95fae85b759c936246ecf441a9fdd95e8fee932d6782cdec686064018c857efc47fb4b2a122600d5fdd79af2486f44df7e629184e1c573bc0a9b3feb40b190ef2861a1ab45e2ac2201b9cd42e495deea247269820ed32389a2810ad6c0f9a296d2a2d9c54089fed50b7f5ecfcd33ab9954360e1d7f5598c32128cfcf2a1d8bf14616818da8a5343bfa88f0eedf392e9d4ab1ace1b60324129cd4852c2e27813a9cf71a6ae6229a4fcecc1a756b3e664c5f50af333082616815a3bec8fc0b75b8e4e767d719"
						}
						]
					},
					"offset": "d202964900000000d302964900000000d402964900000000d502964900000000"
				},
				"version_info": {
					"orig_version": 3,
					"version": 3,
					"block_header_version": 6
				}
			}
		]
	}
	# "#
	# ,
	# r#"
	{
		"jsonrpc": "2.0",
		"id": 1,
		"result": {
			"Ok": null
		}
	}
	# "#
	# ,false, 0 ,false, false, false);
	```
	*/
	fn verify_slate_messages(&self, slate: VersionedSlate) -> Result<(), ErrorKind>;

	/**
	Networked version of [Owner::scan](struct.Owner.html#method.scan).


	```
	# epic_wallet_api::doctest_helper_json_rpc_owner_assert_response!(
	# r#"
	{
		"jsonrpc": "2.0",
		"method": "scan",
		"params": [null, false],
		"id": 1
	}
	# "#
	# ,
	# r#"
	{
		"id": 1,
		"jsonrpc": "2.0",
		"result": {
			"Ok": null
		}
	}
	# "#
	# , false, 1, false, false, false);
	```
	 */
	fn scan(&self, start_height: Option<u64>, delete_unconfirmed: bool) -> Result<(), ErrorKind>;

	/**
	Networked version of [Owner::node_height](struct.Owner.html#method.node_height).


	```
	# epic_wallet_api::doctest_helper_json_rpc_owner_assert_response!(
	# r#"
	{
		"jsonrpc": "2.0",
		"method": "node_height",
		"params": [],
		"id": 1
	}
	# "#
	# ,
	# r#"
	{
		"id": 1,
		"jsonrpc": "2.0",
		"result": {
			"Ok": {
				"header_hash": "d4b3d3c40695afd8c7760f8fc423565f7d41310b7a4e1c4a4a7950a66f16240d",
				"height": "5",
				"updated_from_node": true
			}
		}
	}
	# "#
	# , false, 5, false, false, false);
	```
	 */
	fn node_height(&self) -> Result<NodeHeightResult, ErrorKind>;
}

impl<'a, L, C, K> OwnerRpc for Owner<L, C, K>
where
	L: WalletLCProvider<'static, C, K>,
	C: NodeClient + 'static,
	K: Keychain + 'static,
{
	fn accounts(&self) -> Result<Vec<AcctPathMapping>, ErrorKind> {
		Owner::accounts(self, None).map_err(|e| e.kind())
	}

	fn create_account_path(&self, label: &String) -> Result<Identifier, ErrorKind> {
		Owner::create_account_path(self, None, label).map_err(|e| e.kind())
	}

	fn set_active_account(&self, label: &String) -> Result<(), ErrorKind> {
		Owner::set_active_account(self, None, label).map_err(|e| e.kind())
	}

	fn retrieve_outputs(
		&self,
		include_spent: bool,
		refresh_from_node: bool,
		tx_id: Option<u32>,
	) -> Result<(bool, Vec<OutputCommitMapping>), ErrorKind> {
		Owner::retrieve_outputs(self, None, include_spent, refresh_from_node, tx_id)
			.map_err(|e| e.kind())
	}

	fn retrieve_txs(
		&self,
		refresh_from_node: bool,
		tx_id: Option<u32>,
		tx_slate_id: Option<Uuid>,
	) -> Result<(bool, Vec<TxLogEntry>), ErrorKind> {
		Owner::retrieve_txs(self, None, refresh_from_node, tx_id, tx_slate_id).map_err(|e| e.kind())
	}

	fn retrieve_summary_info(
		&self,
		refresh_from_node: bool,
		minimum_confirmations: u64,
	) -> Result<(bool, WalletInfo), ErrorKind> {
		Owner::retrieve_summary_info(self, None, refresh_from_node, minimum_confirmations)
			.map_err(|e| e.kind())
	}

	fn init_send_tx(&self, args: InitTxArgs) -> Result<VersionedSlate, ErrorKind> {
		let slate = Owner::init_send_tx(self, None, args).map_err(|e| e.kind())?;
		let version = SlateVersion::V3;
		Ok(VersionedSlate::into_version(slate, version))
	}

	fn issue_invoice_tx(&self, args: IssueInvoiceTxArgs) -> Result<VersionedSlate, ErrorKind> {
		let slate = Owner::issue_invoice_tx(self, None, args).map_err(|e| e.kind())?;
		let version = SlateVersion::V3;
		Ok(VersionedSlate::into_version(slate, version))
	}

	fn process_invoice_tx(
		&self,
		in_slate: VersionedSlate,
		args: InitTxArgs,
	) -> Result<VersionedSlate, ErrorKind> {
		let out_slate = Owner::process_invoice_tx(self, None, &Slate::from(in_slate), args)
			.map_err(|e| e.kind())?;
		let version = SlateVersion::V3;
		Ok(VersionedSlate::into_version(out_slate, version))
	}

	fn finalize_tx(&self, in_slate: VersionedSlate) -> Result<VersionedSlate, ErrorKind> {
		let out_slate =
			Owner::finalize_tx(self, None, &Slate::from(in_slate)).map_err(|e| e.kind())?;
		let version = SlateVersion::V3;
		Ok(VersionedSlate::into_version(out_slate, version))
	}

	fn tx_lock_outputs(
		&self,
		slate: VersionedSlate,
		participant_id: usize,
	) -> Result<(), ErrorKind> {
		Owner::tx_lock_outputs(self, None, &Slate::from(slate), participant_id)
			.map_err(|e| e.kind())
	}

	fn cancel_tx(&self, tx_id: Option<u32>, tx_slate_id: Option<Uuid>) -> Result<(), ErrorKind> {
		Owner::cancel_tx(self, None, tx_id, tx_slate_id).map_err(|e| e.kind())
	}

	fn get_stored_tx(&self, tx: &TxLogEntry) -> Result<Option<TransactionV3>, ErrorKind> {
		Owner::get_stored_tx(self, None, tx)
			.map(|x| x.map(|y| TransactionV3::from(y)))
			.map_err(|e| e.kind())
	}

	fn post_tx(&self, tx: TransactionV3, fluff: bool) -> Result<(), ErrorKind> {
		Owner::post_tx(self, None, &Transaction::from(tx), fluff).map_err(|e| e.kind())
	}

	fn verify_slate_messages(&self, slate: VersionedSlate) -> Result<(), ErrorKind> {
		Owner::verify_slate_messages(self, None, &Slate::from(slate)).map_err(|e| e.kind())
	}

	fn scan(&self, start_height: Option<u64>, delete_unconfirmed: bool) -> Result<(), ErrorKind> {
		Owner::scan(self, None, start_height, delete_unconfirmed).map_err(|e| e.kind())
	}

	fn node_height(&self) -> Result<NodeHeightResult, ErrorKind> {
		Owner::node_height(self, None).map_err(|e| e.kind())
	}
}

/// helper to set up a real environment to run integrated doctests
pub fn run_doctest_owner(
	request: serde_json::Value,
	test_dir: &str,
	use_token: bool,
	blocks_to_mine: u64,
	perform_tx: bool,
	lock_tx: bool,
	finalize_tx: bool,
) -> Result<Option<serde_json::Value>, String> {
	use easy_jsonrpc_mw::Handler;
	use epic_wallet_impls::test_framework::{self, LocalWalletClient, WalletProxy};
	use epic_wallet_impls::{DefaultLCProvider, DefaultWalletImpl};
	use epic_wallet_libwallet::{api_impl, WalletInst};
	use epic_wallet_util::epic_keychain::ExtKeychain;

	use crate::core::global::ChainTypes;
	use crate::core::{core::feijoada, global};
	use epic_wallet_util::epic_util as util;

	use std::fs;
	use std::thread;

	util::init_test_logger();
	let _ = fs::remove_dir_all(test_dir);
	global::set_mining_mode(ChainTypes::AutomatedTesting);
	global::set_foundation_path("../tests/assets/foundation.json".to_string());
	let mut policies: feijoada::Policy = feijoada::get_bottles_default();
	policies.insert(feijoada::PoWType::Cuckatoo, 100);
	global::set_policy_config(feijoada::PolicyConfig {
		policies: vec![policies.clone()],
		..Default::default()
	});

	let mut wallet_proxy: WalletProxy<
		DefaultLCProvider<LocalWalletClient, ExtKeychain>,
		LocalWalletClient,
		ExtKeychain,
	> = WalletProxy::new(test_dir);
	let chain = wallet_proxy.chain.clone();

	let rec_phrase_1 = util::ZeroingString::from(
		"fat twenty mean degree forget shell check candy immense awful \
		 flame next during february bulb bike sun wink theory day kiwi embrace peace lunch",
	);
	let empty_string = util::ZeroingString::from("");

	let client1 = LocalWalletClient::new("wallet1", wallet_proxy.tx.clone());
	let mut wallet1 =
		Box::new(DefaultWalletImpl::<LocalWalletClient>::new(client1.clone()).unwrap())
			as Box<
				dyn WalletInst<
					'static,
					DefaultLCProvider<LocalWalletClient, ExtKeychain>,
					LocalWalletClient,
					ExtKeychain,
				>,
			>;
	let lc = wallet1.lc_provider().unwrap();
	let _ = lc.set_top_level_directory(&format!("{}/wallet1", test_dir));
	lc.create_wallet(None, Some(rec_phrase_1), 32, empty_string.clone(), false)
		.unwrap();
	let mask1 = lc
		.open_wallet(None, empty_string.clone(), use_token, true)
		.unwrap();
	let wallet1 = Arc::new(Mutex::new(wallet1));

	if mask1.is_some() {
		println!("WALLET 1 MASK: {:?}", mask1.clone().unwrap());
	}

	wallet_proxy.add_wallet(
		"wallet1",
		client1.get_send_instance(),
		wallet1.clone(),
		mask1.clone(),
	);

	let rec_phrase_2 = util::ZeroingString::from(
		"hour kingdom ripple lunch razor inquiry coyote clay stamp mean \
		 sell finish magic kid tiny wage stand panther inside settle feed song hole exile",
	);
	let client2 = LocalWalletClient::new("wallet2", wallet_proxy.tx.clone());
	let mut wallet2 =
		Box::new(DefaultWalletImpl::<LocalWalletClient>::new(client2.clone()).unwrap())
			as Box<
				dyn WalletInst<
					'static,
					DefaultLCProvider<LocalWalletClient, ExtKeychain>,
					LocalWalletClient,
					ExtKeychain,
				>,
			>;
	let lc = wallet2.lc_provider().unwrap();
	let _ = lc.set_top_level_directory(&format!("{}/wallet2", test_dir));
	lc.create_wallet(None, Some(rec_phrase_2), 32, empty_string.clone(), false)
		.unwrap();
	let mask2 = lc
		.open_wallet(None, empty_string.clone(), use_token, true)
		.unwrap();
	let wallet2 = Arc::new(Mutex::new(wallet2));

	if mask2.is_some() {
		println!("WALLET 2 MASK: {:?}", mask2.clone().unwrap());
	}

	wallet_proxy.add_wallet(
		"wallet2",
		client2.get_send_instance(),
		wallet2.clone(),
		mask2.clone(),
	);

	// Set the wallet proxy listener running
	thread::spawn(move || {
		if let Err(e) = wallet_proxy.run() {
			error!("Wallet Proxy error: {}", e);
		}
	});

	// Mine a few blocks to wallet 1 so there's something to send
	for _ in 0..blocks_to_mine {
		let _ = test_framework::award_blocks_to_wallet(
			&chain,
			wallet1.clone(),
			(&mask1).as_ref(),
			1 as usize,
			false,
		);
		//update local outputs after each block, so transaction IDs stay consistent
		let (wallet_refreshed, _) = api_impl::owner::retrieve_summary_info(
			wallet1.clone(),
			(&mask1).as_ref(),
			&None,
			true,
			1,
		)
		.unwrap();
		assert!(wallet_refreshed);
	}

	//let proof_address = api_impl::owner::get_public_proof_address(wallet2.clone(), (&mask2).as_ref(), 0).unwrap();

	if perform_tx {
		let amount = 600_000_000;
		let mut w_lock = wallet1.lock();
		let w = w_lock.lc_provider().unwrap().wallet_inst().unwrap();
		let args = InitTxArgs {
			src_acct_name: None,
			amount,
			minimum_confirmations: 2,
			max_outputs: 500,
			num_change_outputs: 1,
			selection_strategy_is_use_all: true,
			..Default::default()
		};
		let mut slate =
			api_impl::owner::init_send_tx(&mut **w, (&mask1).as_ref(), args, true).unwrap();
		println!("INITIAL SLATE");
		println!("{}", serde_json::to_string_pretty(&slate).unwrap());
		{
			let mut w_lock = wallet2.lock();
			let w2 = w_lock.lc_provider().unwrap().wallet_inst().unwrap();
			slate = api_impl::foreign::receive_tx(
				&mut **w2,
				(&mask2).as_ref(),
				&slate,
				None,
				None,
				true,
			)
			.unwrap();
			w2.close().unwrap();
		}
		// Spit out slate for input to finalize_tx
		if lock_tx {
			api_impl::owner::tx_lock_outputs(&mut **w, (&mask2).as_ref(), &slate, 0).unwrap();
		}
		println!("RECEIPIENT SLATE");
		println!("{}", serde_json::to_string_pretty(&slate).unwrap());
		if finalize_tx {
			slate = api_impl::owner::finalize_tx(&mut **w, (&mask2).as_ref(), &slate).unwrap();
			error!("FINALIZED TX SLATE");
			println!("{}", serde_json::to_string_pretty(&slate).unwrap());
		}
	}

	if perform_tx && lock_tx && finalize_tx {
		// mine to move the chain on
		let _ = test_framework::award_blocks_to_wallet(
			&chain,
			wallet1.clone(),
			(&mask1).as_ref(),
			3 as usize,
			false,
		);
	}

	let mut api_owner = Owner::new(wallet1);
	api_owner.doctest_mode = true;
	let res = if use_token {
		let owner_api = &api_owner as &dyn OwnerRpcS;
		owner_api.handle_request(request).as_option()
	} else {
		let owner_api = &api_owner as &dyn OwnerRpc;
		owner_api.handle_request(request).as_option()
	};
	let _ = fs::remove_dir_all(test_dir);
	Ok(res)
}

#[doc(hidden)]
#[macro_export]
macro_rules! doctest_helper_json_rpc_owner_assert_response {
	($request:expr, $expected_response:expr, $use_token:expr, $blocks_to_mine:expr, $perform_tx:expr, $lock_tx:expr, $finalize_tx:expr) => {
		// create temporary wallet, run jsonrpc request on owner api of wallet, delete wallet, return
		// json response.
		// In order to prevent leaking tempdirs, This function should not panic.

		// These cause LMDB to run out of disk space on CircleCI
		// disable for now on windows
		// TODO: Fix properly
		#[cfg(not(target_os = "windows"))]
			{
			use epic_wallet_api::run_doctest_owner;
			use serde_json;
			use serde_json::Value;
			use tempfile::tempdir;

			let dir = tempdir().map_err(|e| format!("{:#?}", e)).unwrap();
			let dir = dir
				.path()
				.to_str()
				.ok_or("Failed to convert tmpdir path to string.".to_owned())
				.unwrap();

			let request_val: Value = serde_json::from_str($request).unwrap();
			let expected_response: Value = serde_json::from_str($expected_response).unwrap();

			let response = run_doctest_owner(
				request_val,
				dir,
				$use_token,
				$blocks_to_mine,
				$perform_tx,
				$lock_tx,
				$finalize_tx,
				)
			.unwrap()
			.unwrap();

			if response != expected_response {
				panic!(
					"(left != right) \nleft: {}\nright: {}",
					serde_json::to_string_pretty(&response).unwrap(),
					serde_json::to_string_pretty(&expected_response).unwrap()
				);
				}
			}
	};
}
