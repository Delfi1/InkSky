use spacetimedb_sdk::{
    Address,
    table::TableType,
    identity::{
        Credentials,
        once_on_connect,
        load_credentials,
        save_credentials,
    },
    subscribe
};
use spacetimedb_sdk::identity::Identity;
use crate::network::bindings::User;

mod bindings;

const CREDS_DIR: &str = ".inksky";
const SPACETIME_URI: &str = "https://testnet.spacetimedb.com";

const DB_NAME: &str = "InkSky_server";

fn init_callbacks() {
    println!("Register callbacks...");
    once_on_connect(on_connected);
}

fn subscribe_to_tables() {
    println!("Subscribing to tables...");
    subscribe(&["SELECT * FROM User;"]).unwrap();
}

pub fn connect_to_db() {
    println!("Connecting to DB...");
    bindings::connect(
        SPACETIME_URI,
        DB_NAME,
        load_credentials(CREDS_DIR).expect("Error reading stored credentials"),
    ).expect("Failed to connect");
}

pub fn reconnect() {
    spacetimedb_sdk::disconnect();
    init_callbacks();
    connect_to_db();
    subscribe_to_tables();
}

pub fn on_connected(creds: &Credentials, _address: Address) {
    if let Err(e) = save_credentials(CREDS_DIR, creds) {
        eprintln!("Failed to save credentials: {:?}", e);
    }
}

pub fn is_login() -> bool {
    let identity = spacetimedb_sdk::identity::identity().expect("Read identity error");
    User::find(|u| u.identities.contains(&identity)).is_some()
}