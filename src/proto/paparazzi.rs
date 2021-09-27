/// Message encoding a mission script item. This message is emitted upon a request for the next script item.
///
/// MavLink id: 180
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ScriptItem {
    /// Sequence
    #[prost(uint32, tag = "1")]
    pub seq: u32,
    /// System ID
    #[prost(uint32, tag = "2")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "3")]
    pub target_component: u32,
    /// The name of the mission script, NULL terminated.
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
}
/// Request script item with the sequence number seq. The response of the system to this message should be a SCRIPT_ITEM message.
///
/// MavLink id: 181
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ScriptRequest {
    /// Sequence
    #[prost(uint32, tag = "1")]
    pub seq: u32,
    /// System ID
    #[prost(uint32, tag = "2")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "3")]
    pub target_component: u32,
}
/// Request the overall list of mission items from the system/component.
///
/// MavLink id: 182
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ScriptRequestList {
    /// System ID
    #[prost(uint32, tag = "1")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "2")]
    pub target_component: u32,
}
/// This message is emitted as response to SCRIPT_REQUEST_LIST by the MAV to get the number of mission scripts.
///
/// MavLink id: 183
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ScriptCount {
    /// Number of script items in the sequence
    #[prost(uint32, tag = "1")]
    pub count: u32,
    /// System ID
    #[prost(uint32, tag = "2")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "3")]
    pub target_component: u32,
}
/// This message informs about the currently active SCRIPT.
///
/// MavLink id: 184
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ScriptCurrent {
    /// Active Sequence
    #[prost(uint32, tag = "1")]
    pub seq: u32,
}
