// Copyright (c) 2022-2025 Niko Bonnieure, Par le Peuple, NextGraph.org developers
// All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE2 or http://www.apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

use std::fs::read;

#[allow(unused_imports)]
use nextgraph::local_broker::{
    app_request, app_request_stream, init_local_broker, session_start, session_stop, user_connect,
    user_disconnect, wallet_close, wallet_create_v0, wallet_get, wallet_get_file, wallet_import,
    wallet_open_with_pazzle_words, wallet_read_file, wallet_was_opened, LocalBrokerConfig,
    SessionConfig,
};
use nextgraph::net::types::BootstrapContentV0;
use nextgraph::repo::errors::NgError;
use nextgraph::repo::types::PubKey;
use nextgraph::wallet::types::CreateWalletV0;
use nextgraph::wallet::{display_mnemonic, emojis::display_pazzle};

#[async_std::main]
async fn main() -> std::io::Result<()> {
    // initialize the local_broker with in-memory config.
    // all sessions will be lost when the program exits
    init_local_broker(Box::new(|| LocalBrokerConfig::InMemory)).await;

    // load some image that will be used as security_img
    // we assume here for the sake of this example,
    // that the current directory contains this demo image file
    let security_img = read("nextgraph/examples/wallet-security-image-demo.png")?;

    // the peer_id should come from somewhere else.
    // this is just given for the sake of an example
    #[allow(deprecated)]
    let peer_id_of_server_broker: PubKey = "s2YM98jAU80Eo_l43GDnDDH33fmHc3FpE2GdCJyo5hYA".try_into().unwrap();

    let wallet_result = wallet_create_v0(CreateWalletV0 {
        security_img,
        security_txt: "Generated User".to_string(),
        pin: [1, 2, 1, 2],
        pazzle_length: 9,
        send_bootstrap: false,
        send_wallet: false,
        result_with_wallet_file: true,
        local_save: false,
        core_bootstrap: BootstrapContentV0::new_localhost(peer_id_of_server_broker),
        core_registration: None,
        additional_bootstrap: None,
        pdf: false,
        device_name: "test".to_string(),
    })
    .await?;

    let user_id = wallet_result.personal_identity();
    println!("Using wallet user: {}", user_id.to_string());

    // if the user has internet access, they can now decide to connect to its Server Broker, in order to sync data
    let status = user_connect(&user_id).await?;

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


    // if the user has internet access, they can now decide to connect to its Server Broker, in order to sync data
    let status = user_connect(&user_id).await?;

    println!("Connection status: {:?}", status[0]);
    if let Some(error_reason) = status[0].3.as_ref() {
        println!("Connection failed with: {}", error_reason);
        assert!(error_reason == "NoiseHandshakeFailed" || error_reason == "ConnectionError" || error_reason == "InvitationRequired" || error_reason == "AccessDenied");
    } else {
        println!("✅ Connection successful!");
    }

    // then you can make some calls to the APP protocol
    // with app_request or app_request_stream
    // more to be detailed soon.

    // Then we should disconnect
    user_disconnect(&user_id).await?;

    Ok(())
}
