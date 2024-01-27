use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;
use std::collections::HashMap;

use crate::protocols::types::CommonPlayer;
use crate::protocols::types::CommonResponse;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientTokenResponse {
    pub access_token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub criteria: Vec<SessionFilter>,
}

impl Request {
    pub fn new() -> Self {
        Self {
            criteria: Vec::new(),
        }
    }

    pub fn add_filter(mut self, filter: SessionFilter) -> Self {
        self.criteria.push(filter);
        self
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionFilter {
    pub key: String,
    pub op: String,
    pub value: String,
}

impl SessionFilter {
    pub fn new(key: &str, op: &str, value: &str) -> Self {
        Self {
            key: key.to_string(),
            op: op.to_string(),
            value: value.to_string(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub sessions: Vec<Session>,
    pub count: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub deployment: String,
    pub id: String,
    pub bucket: String,
    pub settings: Settings,
    pub total_players: u32,
    pub open_public_players: u32,
    pub public_players: Vec<String>,
    pub started: bool,
    pub last_updated: Option<String>,
    pub attributes: HashMap<String, Value>,
    pub owner: String,
    pub owner_platform_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub max_public_players: u32,
    pub allow_invites: bool,
    pub should_advertise: bool,
    pub allow_read_by_id: bool,
    pub allow_join_via_presence: bool,
    pub allow_join_in_progress: bool,
    pub allow_conference_room: bool,
    pub check_sanctions: bool,
    pub allow_migration: bool,
    pub rejoin_after_kick: String,
    pub platforms: Option<Vec<String>>,
}
