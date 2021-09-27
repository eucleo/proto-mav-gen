/// ICAROUS heartbeat
///
/// MavLink id: 42000
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct IcarousHeartbeat {
    /// See the FMS_STATE enum.
    #[prost(enumeration = "IcarousFmsState", tag = "1")]
    pub status: i32,
}
/// Kinematic multi bands (track) output from Daidalus
///
/// MavLink id: 42001
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct IcarousKinematicBands {
    /// min angle (degrees)
    #[prost(float, tag = "1")]
    pub min1: f32,
    /// max angle (degrees)
    #[prost(float, tag = "2")]
    pub max1: f32,
    /// min angle (degrees)
    #[prost(float, tag = "3")]
    pub min2: f32,
    /// max angle (degrees)
    #[prost(float, tag = "4")]
    pub max2: f32,
    /// min angle (degrees)
    #[prost(float, tag = "5")]
    pub min3: f32,
    /// max angle (degrees)
    #[prost(float, tag = "6")]
    pub max3: f32,
    /// min angle (degrees)
    #[prost(float, tag = "7")]
    pub min4: f32,
    /// max angle (degrees)
    #[prost(float, tag = "8")]
    pub max4: f32,
    /// min angle (degrees)
    #[prost(float, tag = "9")]
    pub min5: f32,
    /// max angle (degrees)
    #[prost(float, tag = "10")]
    pub max5: f32,
    /// Number of track bands
    #[prost(int32, tag = "11")]
    pub num_bands: i32,
    /// See the TRACK_BAND_TYPES enum.
    #[prost(enumeration = "IcarousTrackBandTypes", tag = "12")]
    pub type1: i32,
    /// See the TRACK_BAND_TYPES enum.
    #[prost(enumeration = "IcarousTrackBandTypes", tag = "13")]
    pub type2: i32,
    /// See the TRACK_BAND_TYPES enum.
    #[prost(enumeration = "IcarousTrackBandTypes", tag = "14")]
    pub type3: i32,
    /// See the TRACK_BAND_TYPES enum.
    #[prost(enumeration = "IcarousTrackBandTypes", tag = "15")]
    pub type4: i32,
    /// See the TRACK_BAND_TYPES enum.
    #[prost(enumeration = "IcarousTrackBandTypes", tag = "16")]
    pub type5: i32,
}
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::prost::Enumeration,
)]
#[repr(i32)]
pub enum IcarousTrackBandTypes {
    IcarousTrackBandTypeNone = 0,
    IcarousTrackBandTypeNear = 1,
    IcarousTrackBandTypeRecovery = 2,
}
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::prost::Enumeration,
)]
#[repr(i32)]
pub enum IcarousFmsState {
    Idle = 0,
    Takeoff = 1,
    Climb = 2,
    Cruise = 3,
    Approach = 4,
    Land = 5,
}
