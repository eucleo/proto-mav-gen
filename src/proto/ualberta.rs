/// Accelerometer and Gyro biases from the navigation filter
///
/// MavLink id: 220
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct NavFilterBias {
    /// Timestamp (microseconds)
    #[prost(uint64, tag = "1")]
    pub usec: u64,
    /// b_f[0]
    #[prost(float, tag = "2")]
    pub accel_0: f32,
    /// b_f[1]
    #[prost(float, tag = "3")]
    pub accel_1: f32,
    /// b_f[2]
    #[prost(float, tag = "4")]
    pub accel_2: f32,
    /// b_f[0]
    #[prost(float, tag = "5")]
    pub gyro_0: f32,
    /// b_f[1]
    #[prost(float, tag = "6")]
    pub gyro_1: f32,
    /// b_f[2]
    #[prost(float, tag = "7")]
    pub gyro_2: f32,
}
/// Complete set of calibration parameters for the radio
///
/// MavLink id: 221
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct RadioCalibration {
    /// Aileron setpoints: left, center, right
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub aileron: ::prost::alloc::vec::Vec<u32>,
    /// Elevator setpoints: nose down, center, nose up
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub elevator: ::prost::alloc::vec::Vec<u32>,
    /// Rudder setpoints: nose left, center, nose right
    #[prost(uint32, repeated, packed = "false", tag = "3")]
    pub rudder: ::prost::alloc::vec::Vec<u32>,
    /// Tail gyro mode/gain setpoints: heading hold, rate mode
    #[prost(uint32, repeated, packed = "false", tag = "4")]
    pub gyro: ::prost::alloc::vec::Vec<u32>,
    /// Pitch curve setpoints (every 25%)
    #[prost(uint32, repeated, packed = "false", tag = "5")]
    pub pitch: ::prost::alloc::vec::Vec<u32>,
    /// Throttle curve setpoints (every 25%)
    #[prost(uint32, repeated, packed = "false", tag = "6")]
    pub throttle: ::prost::alloc::vec::Vec<u32>,
}
/// System status specific to ualberta uav
///
/// MavLink id: 222
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct UalbertaSysStatus {
    /// System mode, see UALBERTA_AUTOPILOT_MODE ENUM
    #[prost(uint32, tag = "1")]
    pub mode: u32,
    /// Navigation mode, see UALBERTA_NAV_MODE ENUM
    #[prost(uint32, tag = "2")]
    pub nav_mode: u32,
    /// Pilot mode, see UALBERTA_PILOT_MODE
    #[prost(uint32, tag = "3")]
    pub pilot: u32,
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
pub enum UalbertaAutopilotMode {
    /// Available autopilot modes for ualberta uav
    /// Raw input pulse widts sent to output
    ModeManualDirect = 0,
    /// Inputs are normalized using calibration, the converted back to raw pulse widths for output
    ModeManualScaled = 1,
    ///  dfsdfs
    ModeAutoPidAtt = 2,
    ///  dfsfds
    ModeAutoPidVel = 3,
    ///  dfsdfsdfs
    ModeAutoPidPos = 4,
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
pub enum UalbertaNavMode {
    /// Navigation filter mode
    NavAhrsInit = 0,
    /// AHRS mode
    NavAhrs = 1,
    /// INS/GPS initialization mode
    NavInsGpsInit = 2,
    /// INS/GPS mode
    NavInsGps = 3,
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
pub enum UalbertaPilotMode {
    /// Mode currently commanded by pilot
    ///  sdf
    PilotManual = 0,
    ///  dfs
    PilotAuto = 1,
    ///  Rotomotion mode
    PilotRoto = 2,
}
