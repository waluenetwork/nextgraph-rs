// Copyright (c) 2022-2025 Niko Bonnieure, Par le Peuple, NextGraph.org developers
// All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE2 or http://www.apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

use std::env::current_dir;
use std::fs::create_dir_all;
use std::fs;
use std::fs::read;
use std::fs::write;
use clap::Parser;

use std::io;
use std::path::Path;
#[allow(unused_imports)]
use nextgraph::local_broker::{
    app_request, app_request_stream, init_local_broker, session_start, session_stop, user_connect,
    user_disconnect, wallet_close, wallet_create_v0, wallet_get, wallet_get_file, wallet_import,
    wallet_open_with_pazzle_words, wallet_read_file, wallet_was_opened, LocalBrokerConfig,
    SessionConfig,
};
use nextgraph::net::types::BootstrapContentV0;
use nextgraph::repo::types::PubKey;
use nextgraph::wallet::types::CreateWalletV0;
use nextgraph::wallet::{display_mnemonic, emojis::display_pazzle};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, env = "NG_PEER_ID", default_value = "s2YM98jAU80Eo_l43GDnDDH33fmHc3FpE2GdCJyo5hYA")]
    peer_id: String,

    #[arg(short, long, env = "NG_USER_ID", default_value = "4yzJccQX0G6dyNrh7vGiiwD6FeOPCZBNy2ChFJsYe8oA")]
    user_id: String,

    #[arg(short, long, env = "NG_WALLET_NAME", default_value = "existing_user_wallet")]
    wallet_name: String,
}

#[async_std::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    // get the current working directory
    let mut current_path = current_dir()?;
    current_path.push(".ng");
    current_path.push("example");
    create_dir_all(current_path.clone())?;

    // initialize the local_broker with config to save to disk in a folder called `.ng/example` in the current directory
    init_local_broker(Box::new(move || {
        LocalBrokerConfig::BasePath(current_path.clone())
    }))
    .await;

    // load some image that will be used as security_img
    // we assume here for the sake of this example,
    // that the current directory contains this demo image file
    let security_img = read("nextgraph/examples/wallet-security-image-demo.png")?;

    let peer_id_of_server_broker: PubKey = args.peer_id.as_str().try_into()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("Invalid peer ID: {}", e)))?;
    
    let user_id: PubKey = args.user_id.as_str().try_into()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("Invalid user ID: {}", e)))?;
    
    println!("Using peer ID: {}", args.peer_id);
    println!("Using user ID: {}", args.user_id);
    println!("Using wallet: {}", args.wallet_name);

    let _session = session_start(SessionConfig::new_save(&user_id, &args.wallet_name)).await?;
    println!("Session started successfully");

    // if the user has internet access, they can now decide to connect to its Server Broker, in order to sync data
    let status = user_connect(&user_id).await?;
    println!("çıkış");

    // The connection cannot succeed because we miss-configured the core_bootstrap of the wallet. its Peer ID is invalid.
    println!("Connection status: {:?}", status[0]);
    if let Some(error_reason) = status[0].3.as_ref() {
        println!("Connection failed with: {}", error_reason);
        assert!(error_reason == "NoiseHandshakeFailed" || error_reason == "ConnectionError" || error_reason == "InvitationRequired" || error_reason == "AccessDenied");
    } else {
        println!("✅ Connection successful!");
    }

    // a session ID has been assigned to you in `wallet_result.session_id` you can use it to fetch a document
    //let _ = doc_fetch(wallet_result.session_id, "ng:example".to_string(), None).await?;

    // Then we should disconnect
    user_disconnect(&user_id).await?;

    session_stop(&user_id).await?;

    println!("Testing connection with existing user credentials...");

    Ok(())
}
