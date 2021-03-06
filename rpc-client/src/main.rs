// Copyright 2019 Polkabyte Network
// This file is part of Polkabyte library.
//
// The Polkabyte library is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// The Polkabyte library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Lesser General Public License for more details.

#![warn(missing_docs)]

//! Example polkabyte RPC client code.
//!
//! This module shows how you can write a Rust RPC client that connects to a running
//! polkabyte node and use statically typed RPC wrappers.

use futures::Future;
use hyper::rt;
use node_primitives::Hash;
use sc_rpc::author::{
	AuthorClient,
	hash::ExtrinsicOrHash,
};
use jsonrpc_core_client::{
	transports::http,
	RpcError,
};

fn main() {
	sp_tracing::try_init_simple();

	rt::run(rt::lazy(|| {
		let uri = "http://localhost:9933";

		http::connect(uri)
			.and_then(|client: AuthorClient<Hash, Hash>| {
				remove_all_extrinsics(client)
			})
			.map_err(|e| {
				println!("Error: {:?}", e);
			})
	}))
}

/// Remove all pending extrinsics from the node.
///
/// The example code takes `AuthorClient` and first:
/// 1. Calls the `pending_extrinsics` method to get all extrinsics in the pool.
/// 2. Then calls `remove_extrinsic` passing the obtained raw extrinsics.
///
/// As the result of running the code the entire content of the transaction pool is going
/// to be removed and the extrinsics are going to be temporarily banned.
fn remove_all_extrinsics(client: AuthorClient<Hash, Hash>) -> impl Future<Item=(), Error=RpcError> {
	client.pending_extrinsics()
		.and_then(move |pending| {
			client.remove_extrinsic(
				pending.into_iter().map(|tx| ExtrinsicOrHash::Extrinsic(tx.into())).collect()
			)
		})
		.map(|removed| {
			println!("Removed extrinsics: {:?}", removed);
		})
}
