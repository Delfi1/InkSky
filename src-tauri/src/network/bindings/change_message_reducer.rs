// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused_imports)]
use spacetimedb_sdk::{
    anyhow::{anyhow, Result},
    identity::Identity,
    reducer::{Reducer, ReducerCallbackId, Status},
    sats::{de::Deserialize, ser::Serialize},
    spacetimedb_lib,
    table::{TableIter, TableType, TableWithPrimaryKey},
    Address,
};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ChangeMessageArgs {
    pub message_id: u64,
    pub new_content: String,
}

impl Reducer for ChangeMessageArgs {
    const REDUCER_NAME: &'static str = "change_message";
}

#[allow(unused)]
pub fn change_message(message_id: u64, new_content: String) {
    ChangeMessageArgs {
        message_id,
        new_content,
    }
    .invoke();
}

#[allow(unused)]
pub fn on_change_message(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &u64, &String) + Send + 'static,
) -> ReducerCallbackId<ChangeMessageArgs> {
    ChangeMessageArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let ChangeMessageArgs {
            message_id,
            new_content,
        } = __args;
        __callback(__identity, __addr, __status, message_id, new_content);
    })
}

#[allow(unused)]
pub fn once_on_change_message(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &u64, &String) + Send + 'static,
) -> ReducerCallbackId<ChangeMessageArgs> {
    ChangeMessageArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let ChangeMessageArgs {
            message_id,
            new_content,
        } = __args;
        __callback(__identity, __addr, __status, message_id, new_content);
    })
}

#[allow(unused)]
pub fn remove_on_change_message(id: ReducerCallbackId<ChangeMessageArgs>) {
    ChangeMessageArgs::remove_on_reducer(id);
}
