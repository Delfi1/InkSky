// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused_imports)]
use spacetimedb_sdk::callbacks::{DbCallbacks, ReducerCallbacks};
use spacetimedb_sdk::client_api_messages::{Event, TableUpdate};
use spacetimedb_sdk::client_cache::{ClientCache, RowCallbackReminders};
use spacetimedb_sdk::global_connection::with_connection_mut;
use spacetimedb_sdk::identity::Credentials;
use spacetimedb_sdk::reducer::AnyReducerEvent;
use spacetimedb_sdk::spacetime_module::SpacetimeModule;
use spacetimedb_sdk::{
    anyhow::{anyhow, Result},
    identity::Identity,
    reducer::{Reducer, ReducerCallbackId, Status},
    sats::{de::Deserialize, ser::Serialize},
    spacetimedb_lib,
    table::{TableIter, TableType, TableWithPrimaryKey},
    Address,
};
use std::sync::Arc;

pub mod change_message_reducer;
pub mod change_name_reducer;
pub mod credentials;
pub mod delete_account_reducer;
pub mod login_reducer;
pub mod logout_reducer;
pub mod message;
pub mod register_reducer;
pub mod send_message_reducer;
pub mod session;
pub mod user;

pub use change_message_reducer::*;
pub use change_name_reducer::*;
pub use credentials::*;
pub use delete_account_reducer::*;
pub use login_reducer::*;
pub use logout_reducer::*;
pub use message::*;
pub use register_reducer::*;
pub use send_message_reducer::*;
pub use session::*;
pub use user::*;

#[allow(unused)]
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum ReducerEvent {
    ChangeMessage(change_message_reducer::ChangeMessageArgs),
    ChangeName(change_name_reducer::ChangeNameArgs),
    DeleteAccount(delete_account_reducer::DeleteAccountArgs),
    Login(login_reducer::LoginArgs),
    Logout(logout_reducer::LogoutArgs),
    Register(register_reducer::RegisterArgs),
    SendMessage(send_message_reducer::SendMessageArgs),
}

#[allow(unused)]
pub struct Module;
impl SpacetimeModule for Module {
    fn handle_table_update(
        &self,
        table_update: TableUpdate,
        client_cache: &mut ClientCache,
        callbacks: &mut RowCallbackReminders,
    ) {
        let table_name = &table_update.table_name[..];
        match table_name {
            "Credentials" => client_cache
                .handle_table_update_no_primary_key::<credentials::Credentials>(
                    callbacks,
                    table_update,
                ),
            "Message" => client_cache
                .handle_table_update_with_primary_key::<message::Message>(callbacks, table_update),
            "User" => client_cache
                .handle_table_update_with_primary_key::<user::User>(callbacks, table_update),
            _ => {
                spacetimedb_sdk::log::error!("TableRowOperation on unknown table {:?}", table_name)
            }
        }
    }
    fn invoke_row_callbacks(
        &self,
        reminders: &mut RowCallbackReminders,
        worker: &mut DbCallbacks,
        reducer_event: Option<Arc<AnyReducerEvent>>,
        state: &Arc<ClientCache>,
    ) {
        reminders.invoke_callbacks::<credentials::Credentials>(worker, &reducer_event, state);
        reminders.invoke_callbacks::<message::Message>(worker, &reducer_event, state);
        reminders.invoke_callbacks::<user::User>(worker, &reducer_event, state);
    }
    fn handle_event(
        &self,
        event: Event,
        _reducer_callbacks: &mut ReducerCallbacks,
        _state: Arc<ClientCache>,
    ) -> Option<Arc<AnyReducerEvent>> {
        let Some(function_call) = &event.function_call else {
            spacetimedb_sdk::log::warn!("Received Event with None function_call");
            return None;
        };
        #[allow(clippy::match_single_binding)]
        match &function_call.reducer[..] {
            "change_message" => _reducer_callbacks
                .handle_event_of_type::<change_message_reducer::ChangeMessageArgs, ReducerEvent>(
                    event,
                    _state,
                    ReducerEvent::ChangeMessage,
                ),
            "change_name" => _reducer_callbacks
                .handle_event_of_type::<change_name_reducer::ChangeNameArgs, ReducerEvent>(
                    event,
                    _state,
                    ReducerEvent::ChangeName,
                ),
            "delete_account" => _reducer_callbacks
                .handle_event_of_type::<delete_account_reducer::DeleteAccountArgs, ReducerEvent>(
                    event,
                    _state,
                    ReducerEvent::DeleteAccount,
                ),
            "login" => _reducer_callbacks
                .handle_event_of_type::<login_reducer::LoginArgs, ReducerEvent>(
                    event,
                    _state,
                    ReducerEvent::Login,
                ),
            "logout" => _reducer_callbacks
                .handle_event_of_type::<logout_reducer::LogoutArgs, ReducerEvent>(
                    event,
                    _state,
                    ReducerEvent::Logout,
                ),
            "register" => _reducer_callbacks
                .handle_event_of_type::<register_reducer::RegisterArgs, ReducerEvent>(
                    event,
                    _state,
                    ReducerEvent::Register,
                ),
            "send_message" => _reducer_callbacks
                .handle_event_of_type::<send_message_reducer::SendMessageArgs, ReducerEvent>(
                    event,
                    _state,
                    ReducerEvent::SendMessage,
                ),
            unknown => {
                spacetimedb_sdk::log::error!("Event on an unknown reducer: {:?}", unknown);
                None
            }
        }
    }
    fn handle_resubscribe(
        &self,
        new_subs: TableUpdate,
        client_cache: &mut ClientCache,
        callbacks: &mut RowCallbackReminders,
    ) {
        let table_name = &new_subs.table_name[..];
        match table_name {
            "Credentials" => client_cache
                .handle_resubscribe_for_type::<credentials::Credentials>(callbacks, new_subs),
            "Message" => {
                client_cache.handle_resubscribe_for_type::<message::Message>(callbacks, new_subs)
            }
            "User" => client_cache.handle_resubscribe_for_type::<user::User>(callbacks, new_subs),
            _ => {
                spacetimedb_sdk::log::error!("TableRowOperation on unknown table {:?}", table_name)
            }
        }
    }
}

/// Connect to a database named `db_name` accessible over the internet at the URI `spacetimedb_uri`.
///
/// If `credentials` are supplied, they will be passed to the new connection to
/// identify and authenticate the user. Otherwise, a set of `Credentials` will be
/// generated by the server.
pub fn connect<IntoUri>(
    spacetimedb_uri: IntoUri,
    db_name: &str,
    credentials: Option<Credentials>,
) -> Result<()>
where
    IntoUri: TryInto<spacetimedb_sdk::http::Uri>,
    <IntoUri as TryInto<spacetimedb_sdk::http::Uri>>::Error:
        std::error::Error + Send + Sync + 'static,
{
    with_connection_mut(|connection| {
        connection.connect(spacetimedb_uri, db_name, credentials, Arc::new(Module))?;
        Ok(())
    })
}
