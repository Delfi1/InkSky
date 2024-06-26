use spacetimedb::{spacetimedb, ReducerContext, Identity, Timestamp, Address, SpacetimeType};
use log::{log, debug, info, warn, error};

#[spacetimedb(table(private))]
pub struct Credentials {
    #[unique]
    primary_id: u64,
    #[unique]
    pub email: String,
    password: String,

    // Identity, address, online status
    sessions: Vec<Session>,
}

#[derive(SpacetimeType)]
pub struct Session {
    identity: Identity,
    address: Option<Address>,
    online: bool
}

#[spacetimedb(table(public))]
pub struct User {
    #[autoinc]
    #[primarykey]
    pub primary_id: u64,
    pub name: String,
    #[unique]
    pub email: String,
    pub online: bool,
    pub identities: Vec<Identity>
}


/// Get credentials structure
fn get_creds_by_ctx(ctx: &ReducerContext) -> Option<Credentials> {
    Credentials::iter().find(|c| c.sessions.iter().find(|s| s.identity == ctx.sender).is_some())
}

/// Get user structure by sender
fn get_user_by_ctx(ctx: &ReducerContext) -> Option<User> {
    if let Some(creds) = get_creds_by_ctx(ctx) {
        User::filter_by_primary_id(&creds.primary_id)
    } else {
        None
    }
}

/// Get user by indentity
fn get_user_by_identity(identity: &Identity) -> Option<User> {
    User::iter().find(|u| u.identities.contains(identity))
}

fn set_online_by_ctx(ctx: &ReducerContext, state: bool) {
    if let Some(mut creds) = get_creds_by_ctx(ctx) {
        creds.sessions
            .iter_mut()
            .find(|s| s.identity == ctx.sender)
            .unwrap().online = state;

        let mut user = User::filter_by_primary_id(&creds.primary_id).unwrap();
        user.online = !user.identities.is_empty();

        User::update_by_primary_id(&user.primary_id.clone(), user);
        Credentials::update_by_primary_id(&creds.primary_id.clone(), creds);
    }
}

#[spacetimedb(reducer)]
fn register(ctx: ReducerContext, email: String, password: String, name: String) -> Result<(), String> {
    if get_creds_by_ctx(&ctx).is_some() {
        return Err("Already in session".to_string())
    }

    if User::filter_by_email(&email).is_some() {
        return Err("This email is taken".to_string())
    }

    let user = User::insert(User {
        primary_id: 0,
        name,
        email: email.clone(),
        online: true,
        identities: vec![ctx.sender.clone()]
    }).expect("User insert error");

    Credentials::insert(Credentials {
        primary_id: user.primary_id,
        email,
        password,
        sessions: vec![Session {identity: ctx.sender, address: ctx.address, online: true}]
    }).expect("Credentials insert error");

    Ok(())
}

#[spacetimedb(reducer)]
fn logout(ctx: ReducerContext) -> Result<(), String> {
    if let Some(mut creds) = get_creds_by_ctx(&ctx) {
        let index = creds.sessions.iter().position(|s| s.identity == ctx.sender).unwrap();
        creds.sessions.remove(index);
        Credentials::update_by_primary_id(&creds.primary_id.clone(), creds);
        Ok(())
    } else {
        Err("User not found".to_string())
    }
}

#[spacetimedb(reducer)]
fn login(ctx: ReducerContext, email: String, password: String) -> Result<(), String> {
    if get_creds_by_ctx(&ctx).is_some() {
        return Err("Already in session".to_string())
    }

    if let Some(mut creds) = Credentials::filter_by_email(&email) {
        if password == creds.password {
            creds.sessions.push(Session {identity: ctx.sender, address: ctx.address, online: true});
            Credentials::update_by_primary_id(&creds.primary_id.clone(), creds);
            Ok(())
        } else {
            Err("Incorrect password".to_string())
        }
    } else {
        Err("User not found".to_string())
    }
}

#[spacetimedb(reducer)]
pub fn delete_account(ctx: ReducerContext) {
    if let Some(creds) = get_creds_by_ctx(&ctx) {
        debug!("Deleting...");
        User::delete_by_primary_id(&creds.primary_id);
        Credentials::delete_by_primary_id(&creds.primary_id);
    }
}

#[spacetimedb(connect)]
pub fn connect(ctx: ReducerContext) {
    info!("Identity connected: {}", ctx.sender.to_string());
    if let Some(mut user) = get_user_by_ctx(&ctx) {
        user.identities.push(ctx.sender);
        User::update_by_primary_id(&user.primary_id.clone(), user);
    }
    set_online_by_ctx(&ctx, true);
}

#[spacetimedb(disconnect)]
pub fn disconnect(ctx: ReducerContext) {
    info!("Identity disconnected: {}", ctx.sender.to_string());
    if let Some(mut user) = get_user_by_identity(&ctx.sender) {
        let index = user.identities.iter().position(|i| i == &ctx.sender).unwrap();
        user.identities.remove(index);
        User::update_by_primary_id(&user.primary_id.clone(), user);
    }
    set_online_by_ctx(&ctx, false);
}
