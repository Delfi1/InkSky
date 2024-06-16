use spacetimedb_sdk::{Address, identity::{
    Credentials,
    load_credentials,
    save_credentials
}, subscribe};
use spacetimedb_sdk::identity::{Identity, once_on_connect};
use spacetimedb_sdk::reducer::Status;
use spacetimedb_sdk::table::TableType;
use tauri::Manager;

mod bindings;

const CREDS_DIR: &str = ".inksky";
const SPACETIME_URI: &str = "https://testnet.spacetimedb.com";

const DB_NAME: &str = "InkSky_server";

fn init_callbacks() {
    println!("Register callbacks...");
    once_on_connect(on_connected);
    bindings::on_register(on_register_callback);
    bindings::on_login(on_login_callback);
}

pub fn reconnect() {
    spacetimedb_sdk::disconnect();
    connect_to_db();
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

pub fn init() {
    init_callbacks();
    connect_to_db();
    subscribe_to_tables();
}

fn emit_login_err(err: String) {
    let window = crate::main_window();

    println!("emitting... {}", err);
    let cmd = format!("document.getElementById(\"err_field\").textContent = {} ", err);
    println!("Command: {}", cmd.clone());
    window.eval(cmd.as_str()).unwrap()
}

fn on_register_callback(_sender_id: &Identity, _sender_address: Option<Address>, status: &Status, email: &String, password: &String, name: &String) {
    if let Status::Failed(err) = status {
        eprintln!("Failed to register {:?}: {}", email, err);
        emit_login_err(err.clone());
    } else {
        println!("Register success: {:?} {:?}", email, password)
    }
}

fn on_login_callback(_sender_id: &Identity, _sender_address: Option<Address>, status: &Status, email: &String, password: &String) {
    if let Status::Failed(err) = status {
        eprintln!("Failed to login {:?}: {}", email, err);
        emit_login_err(err.clone());
    } else {
        println!("Login success: {:?} {:?}", email, password)
    }
}

fn on_connected(creds: &Credentials, _client_address: Address) {
    if let Err(e) = save_credentials(CREDS_DIR, creds) {
        eprintln!("Failed to save credentials: {:?}", e);
    }
}

pub fn is_email_taken(email: &String) -> bool {
    bindings::user::User::find_by_email(email.clone()).is_some()
}

pub fn register(email: String, name: String, password: String) -> String {
    if is_email_taken(&email) {
        return "This email is taken".to_string();
    }

    bindings::register(email, password, name);
    return String::new();
}

pub fn login(email: String, password: String) {
    bindings::login(email, password)
}