//! [`Authenticator`] implementation that authenticates against a JSON file.
//!
//! [`Authenticator`]: trait.Authenticator.html

use crate::auth::*;

use async_trait::async_trait;
use log::{info, warn};
use serde::Deserialize;
use std::fs;
use std::time::Duration;
use tokio::time::delay_for;

#[derive(Deserialize, Clone, Debug)]
struct Credentials {
    username: String,
    password: String,
}

/// [`Authenticator`] implementation that authenticates against a JSON file.
///
/// [`Authenticator`]: ../trait.Authenticator.html
///
/// Example credentials file format:
/// [
//   {
//     "username": "alice",
//     "password": "12345678"
//   },
//   {
//     "username": "bob",
//     "password": "secret"
//   }
// ]
#[derive(Clone, Debug)]
pub struct JsonFileAuthenticator {
    credentials_list: Vec<Credentials>,
}

impl JsonFileAuthenticator {
    /// Initialize a new [`JsonFileAuthenticator`] from file.
    pub fn new<T: Into<String>>(filename: T) -> Result<Self, Box<dyn std::error::Error>> {
        let s = fs::read_to_string(filename.into())?;
        let credentials_list: Vec<Credentials> = serde_json::from_str(&s)?;
        Ok(JsonFileAuthenticator { credentials_list })
    }
}

#[async_trait]
impl Authenticator<DefaultUser> for JsonFileAuthenticator {
    async fn authenticate(&self, _username: &str, _password: &str) -> Result<DefaultUser, Box<dyn std::error::Error + Send + Sync>> {
        let username = _username.to_string();
        let password = _password.to_string();
        let credentials_list = self.credentials_list.clone();

        for c in credentials_list.iter() {
            if username == c.username {
                if password == c.password {
                    info!("Successful login by user {}", username);
                    return Ok(DefaultUser {});
                } else {
                    warn!("Failed login for user {}: bad password", username);
                    // punish the failed login with a 1500ms delay before returning the error
                    delay_for(Duration::from_millis(1500)).await;
                    return Err(Box::new(BadPasswordError));
                }
            }
        }
        warn!("Failed login for user \"{}\": unknown user", username);

        // punish the failed login with a 1500ms delay before returning the error
        delay_for(Duration::from_millis(1500)).await;
        Err(Box::new(UnknownUsernameError))
    }
}
