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
use clap::Parser;

#[allow(unused_imports)]
use nextgraph::local_broker::{
    app_request, app_request_stream, init_local_broker, session_start, session_stop, user_connect,
    user_disconnect, wallet_close, wallet_create_v0, wallet_get, wallet_get_file, wallet_import,
    wallet_open_with_pazzle, wallet_open_with_pazzle_words, wallet_read_file, wallet_was_opened,
    LocalBrokerConfig, SessionConfig,
};
use ng_repo::types::PubKey;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, env = "NG_PEER_ID", default_value = "s2YM98jAU80Eo_l43GDnDDH33fmHc3FpE2GdCJyo5hYA")]
    peer_id: String,

    #[arg(short, long, env = "NG_USER_ID", default_value = "4yzJccQX0G6dyNrh7vGiiwD6FeOPCZBNy2ChFJsYe8oA")]
    user_id: String,

    #[arg(short, long, env = "NG_WALLET_NAME", default_value = "generated_user_wallet")]
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

    let user_id: PubKey = args.user_id.as_str().try_into()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("Invalid user ID: {}", e)))?;

    let peer_id_of_server_broker: PubKey = args.peer_id.as_str().try_into()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("Invalid peer ID: {}", e)))?;

    println!("Using peer ID: {}", args.peer_id);
    println!("Using user ID: {}", args.user_id);
    println!("Using wallet: {}", args.wallet_name);

    // as we have previously saved the wallet,
    // we can retrieve it, display the security phrase and image to the user, ask for the pazzle or mnemonic, and then open the wallet
    let wallet = wallet_get(&args.wallet_name).await?;

    // at this point, the wallet is kept in the internal memory of the LocalBroker
    // and it hasn't been opened yet, so it is not usable right away.
    // now let's open the wallet, by providing the pazzle and PIN code
    let opened_wallet = wallet_open_with_pazzle(
        &wallet,
        vec![7, 17, 102, 43, 126, 84, 135, 56, 64],
        [1, 2, 1, 2],
    )?;

    // let user_id = opened_wallet.personal_identity(); // Use generated user instead

    // once the wallet is opened, we notify the LocalBroker that we have opened it.
    let _client = wallet_was_opened(opened_wallet).await?;

    // now that the wallet is opened, let's start a session.
    // we pass the user_id and the wallet_name
    let _session = session_start(SessionConfig::new_save(&user_id, &args.wallet_name)).await?;

    // if the user has internet access, they can now decide to connect to its Server Broker, in order to sync data
    let status = user_connect(&user_id).await?;

    // The connection cannot succeed because we miss-configured the core_bootstrap of the wallet. its Peer ID is invalid.
    let error_reason = status[0].3.as_ref().unwrap();
    assert!(error_reason == "NoiseHandshakeFailed" || error_reason == "ConnectionError" || error_reason == "InvitationRequired" || error_reason == "AccessDenied");
    println!("Connection was : {:?}", status[0]);

    // Then we should disconnect
    user_disconnect(&user_id).await?;

    // stop the session
    session_stop(&user_id).await?;

    // closes the wallet
    wallet_close(&args.wallet_name).await?;

    Ok(())
}
