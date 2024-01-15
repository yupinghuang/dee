pub mod systemd;

use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Protection {
    // uid of the user
    pub uid: u32,
    /// The current username of the user.
    pub username: String,
}
