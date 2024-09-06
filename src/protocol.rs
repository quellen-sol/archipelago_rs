use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::utils::is_important;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "cmd")]
pub enum ClientMessage {
    Connect(Connect),
    Sync,
    LocationChecks(LocationChecks),
    LocationScouts(LocationScouts),
    StatusUpdate(StatusUpdate),
    Say(Say),
    GetDataPackage(GetDataPackage),
    Bounce(Bounce),
    Get(Get),
    Set(Set),
    SetNotify(SetNotify),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "cmd")]
pub enum ServerMessage {
    RoomInfo(RoomInfo),
    ConnectionRefused(ConnectionRefused),
    Connected(Connected),
    ReceivedItems(ReceivedItems),
    LocationInfo(LocationInfo),
    RoomUpdate(RoomUpdate),
    Print(Print),
    PrintJSON(PrintJSON),
    DataPackage(DataPackage),
    Bounced(Bounced),
    InvalidPacket(InvalidPacket),
    Retrieved(Retrieved),
    SetReply(SetReply),
}

#[derive(Clone, Debug, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum Permission {
    Disabled = 0,
    Enabled = 1,
    Goal = 2,
    Auto = 6,
    AutoEnabled = 7,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetworkVersion {
    pub major: i32,
    pub minor: i32,
    pub build: i32,
    pub class: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkPlayer {
    pub team: i32,
    pub slot: i32,
    pub alias: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct NetworkItem {
    pub item: i32,
    pub location: i32,
    pub player: i32,
    pub flags: i32,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum SlotType {
    Spectator = 0,
    Player = 1,
    Group = 2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkSlot {
    pub name: String,
    pub game: String,
    pub r#type: SlotType,
    pub group_members: Vec<i32>,
}

pub fn network_version() -> NetworkVersion {
    NetworkVersion {
        major: 0,
        minor: 5,
        build: 0,
        class: "Version".to_string(),
    }
}

// REQUESTS

#[derive(Debug, Serialize, Deserialize)]
pub struct Connect {
    pub password: Option<String>,
    pub name: String,
    pub version: NetworkVersion,
    pub items_handling: Option<i32>,
    pub tags: Vec<String>,
    pub uuid: String,
    pub game: String,
    pub slot_data: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectUpdate {
    pub items_handling: i32,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationChecks {
    pub locations: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationScouts {
    pub locations: Vec<i32>,
    pub create_as_hint: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusUpdate {
    pub status: ClientStatus,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum ClientStatus {
    ClientUnknown = 0,
    ClientConnected = 5,
    ClientReady = 10,
    ClientPlaying = 20,
    ClientGoal = 30,
}

impl From<u16> for ClientStatus {
    fn from(value: u16) -> Self {
        match value {
            0 => ClientStatus::ClientUnknown,
            5 => ClientStatus::ClientConnected,
            10 => ClientStatus::ClientReady,
            20 => ClientStatus::ClientPlaying,
            30 => ClientStatus::ClientGoal,
            _ => panic!("Bad value provided for ClientStatus ({value})"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Say {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDataPackage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub games: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bounce {
    pub games: Option<Vec<String>>,
    pub slots: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub data: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Get {
    pub keys: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Set {
    pub key: String,
    pub default: Value,
    pub want_reply: bool,
    pub operations: Vec<DataStorageOperation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataStorageOperation {
    pub replace: String, // TODO: enum-ify?
    pub value: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetNotify {
    pub keys: Vec<String>,
}

// RESPONSES

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PermissionsMap {
    pub release: Permission,
    pub collect: Permission,
    pub remaining: Permission,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoomInfo {
    pub version: NetworkVersion,
    pub generator_version: NetworkVersion,
    pub tags: Vec<String>,
    pub password: bool,
    pub permissions: PermissionsMap,
    pub hint_cost: i32,
    pub location_check_points: i32,
    pub games: Option<Vec<String>>,
    pub datapackage_checksums: HashMap<String, String>,
    pub seed_name: String,
    pub time: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionRefused {
    pub errors: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Connected {
    pub team: i32,
    pub slot: i32,
    pub players: Vec<NetworkPlayer>,
    pub missing_locations: Vec<i32>,
    pub checked_locations: Vec<i32>,
    pub slot_data: Value,
    pub slot_info: HashMap<String, NetworkSlot>, // TODO: docs claim this is an int key. they are lying?
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReceivedItems {
    pub index: i32,
    pub items: Vec<NetworkItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationInfo {
    pub locations: Vec<NetworkItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomUpdate {
    // Copied from RoomInfo
    pub version: Option<NetworkVersion>,
    pub tags: Option<Vec<String>>,
    pub password: Option<bool>,
    pub permissions: Option<HashMap<String, Permission>>,
    pub hint_cost: Option<i32>,
    pub location_check_points: Option<i32>,
    pub games: Option<Vec<String>>,
    pub datapackage_versions: Option<HashMap<String, i32>>,
    pub seed_name: Option<String>,
    pub time: Option<f32>,
    // Exclusive to RoomUpdate
    pub hint_points: Option<i32>,
    pub players: Option<Vec<NetworkPlayer>>,
    pub checked_locations: Option<Vec<i32>>,
    pub missing_locations: Option<Vec<i32>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Print {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrintJSON {
    pub data: Vec<JSONMessagePart>,
    pub r#type: Option<String>,
    pub receiving: Option<i32>,
    pub item: Option<NetworkItem>,
    pub found: Option<bool>,
    pub countdown: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct HintData {
    pub receiving: i32,
    pub item: NetworkItem,
    pub found: bool,
    pub is_important: bool,
}

impl From<PrintJSON> for HintData {
    fn from(value: PrintJSON) -> Self {
        let item = value
            .item
            .expect("`item` field is required, but missing from PrintJSON packet");
        let is_important = is_important(item.flags);

        Self {
            receiving: value
                .receiving
                .expect("`receiving` field is required, but missing from PrintJSON packet"),
            item,
            found: value
                .found
                .expect("`found` field is required, but missing from PrintJSON packet"),
            is_important,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Hint {
    receiving_player: i32,
    finding_player: i32,
    location: i32,
    item: i32,
    found: bool,
    entrance: String,
    item_flags: i32,
}

impl From<Hint> for HintData {
    fn from(value: Hint) -> Self {
        let item = NetworkItem {
            item: value.item,
            location: value.location,
            player: value.finding_player,
            flags: value.item_flags,
        };

        Self {
            receiving: value.receiving_player,
            item,
            found: value.found,
            is_important: is_important(value.item_flags),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JSONMessagePart {
    pub r#type: Option<String>,
    pub text: Option<String>,
    pub color: Option<String>,
    pub flags: Option<i32>,
    pub player: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataPackage {
    pub data: DataPackageObject,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataPackageObject {
    pub games: HashMap<String, GameData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameData {
    pub item_name_to_id: HashMap<String, i32>,
    pub location_name_to_id: HashMap<String, i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bounced {
    pub games: Option<Vec<String>>,
    pub slots: Option<Vec<i32>>,
    pub tags: Option<Vec<String>>,
    pub data: Bounce,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvalidPacket {
    pub r#type: String,
    pub original_cmd: Option<String>,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Retrieved {
    pub keys: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetReply {
    key: String,
    value: Value,
    original_value: Value,
}
