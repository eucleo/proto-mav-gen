/// Static data to configure the ADS-B transponder (send within 10 sec of a POR and every 10 sec thereafter)
///
/// MavLink id: 10001
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct UavionixAdsbOutCfg {
    /// Vehicle address (24 bit)
    #[prost(uint32, tag = "1")]
    pub icao: u32,
    /// Aircraft stall speed in cm/s
    #[prost(uint32, tag = "2")]
    pub stall_speed: u32,
    /// Vehicle identifier (8 characters, null terminated, valid characters are A-Z, 0-9, " " only)
    #[prost(string, tag = "3")]
    pub callsign: ::prost::alloc::string::String,
    /// Transmitting vehicle type. See ADSB_EMITTER_TYPE enum
    #[prost(enumeration = "super::common::AdsbEmitterType", tag = "4")]
    pub emitter_type: i32,
    /// Aircraft length and width encoding (table 2-35 of DO-282B)
    #[prost(enumeration = "UavionixAdsbOutCfgAircraftSize", tag = "5")]
    pub aircraft_size: i32,
    /// GPS antenna lateral offset (table 2-36 of DO-282B)
    #[prost(enumeration = "UavionixAdsbOutCfgGpsOffsetLat", tag = "6")]
    pub gps_offset_lat: i32,
    /// GPS antenna longitudinal offset from nose [if non-zero, take position (in meters) divide by 2 and add one] (table 2-37 DO-282B)
    #[prost(enumeration = "UavionixAdsbOutCfgGpsOffsetLon", tag = "7")]
    pub gps_offset_lon: i32,
    /// ADS-B transponder reciever and transmit enable flags
    /// bitfield defined by enum UAVIONIX_ADSB_OUT_RF_SELECT
    #[prost(uint32, tag = "8")]
    pub rf_select: u32,
}
/// Dynamic data used to generate ADS-B out transponder data (send at 5Hz)
///
/// MavLink id: 10002
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct UavionixAdsbOutDynamic {
    /// UTC time in seconds since GPS epoch (Jan 6, 1980). If unknown set to UINT32_MAX
    #[prost(uint32, tag = "1")]
    pub utc_time: u32,
    /// Latitude WGS84 (deg * 1E7). If unknown set to INT32_MAX
    #[prost(int32, tag = "2")]
    pub gps_lat: i32,
    /// Longitude WGS84 (deg * 1E7). If unknown set to INT32_MAX
    #[prost(int32, tag = "3")]
    pub gps_lon: i32,
    /// Altitude (WGS84). UP +ve. If unknown set to INT32_MAX
    #[prost(int32, tag = "4")]
    pub gps_alt: i32,
    /// Barometric pressure altitude (MSL) relative to a standard atmosphere of 1013.2 mBar and NOT bar corrected altitude (m * 1E-3). (up +ve). If unknown set to INT32_MAX
    #[prost(int32, tag = "5")]
    pub baro_alt_msl: i32,
    /// Horizontal accuracy in mm (m * 1E-3). If unknown set to UINT32_MAX
    #[prost(uint32, tag = "6")]
    pub accuracy_hor: u32,
    /// Vertical accuracy in cm. If unknown set to UINT16_MAX
    #[prost(uint32, tag = "7")]
    pub accuracy_vert: u32,
    /// Velocity accuracy in mm/s (m * 1E-3). If unknown set to UINT16_MAX
    #[prost(uint32, tag = "8")]
    pub accuracy_vel: u32,
    /// GPS vertical speed in cm/s. If unknown set to INT16_MAX
    #[prost(int32, tag = "9")]
    pub vel_vert: i32,
    /// North-South velocity over ground in cm/s North +ve. If unknown set to INT16_MAX
    #[prost(int32, tag = "10")]
    pub vel_ns: i32,
    /// East-West velocity over ground in cm/s East +ve. If unknown set to INT16_MAX
    #[prost(int32, tag = "11")]
    pub vel_ew: i32,
    /// ADS-B transponder dynamic input state flags
    /// bitfield defined by enum UAVIONIX_ADSB_OUT_DYNAMIC_STATE
    #[prost(uint32, tag = "12")]
    pub state: u32,
    /// Mode A code (typically 1200 \[0x04B0\] for VFR)
    #[prost(uint32, tag = "13")]
    pub squawk: u32,
    /// 0-1: no fix, 2: 2D fix, 3: 3D fix, 4: DGPS, 5: RTK
    #[prost(enumeration = "UavionixAdsbOutDynamicGpsFix", tag = "14")]
    pub gps_fix: i32,
    /// Number of satellites visible. If unknown set to UINT8_MAX
    #[prost(uint32, tag = "15")]
    pub num_sats: u32,
    /// Emergency status
    #[prost(enumeration = "UavionixAdsbEmergencyStatus", tag = "16")]
    pub emergency_status: i32,
}
/// Transceiver heartbeat with health report (updated every 10s)
///
/// MavLink id: 10003
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct UavionixAdsbTransceiverHealthReport {
    /// ADS-B transponder messages
    /// bitfield defined by enum UAVIONIX_ADSB_RF_HEALTH
    #[prost(uint32, tag = "1")]
    pub rf_health: u32,
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
pub enum UavionixAdsbOutDynamicState {
    /// State flags for ADS-B transponder dynamic report
    /// This enum is used to define bitmasks (work around protobuf limitations).
    /// Not used in MavLink, make protobuf happy.
    Undefined = 0,
    /// bit 1
    IntentChange = 1,
    /// bit 2
    AutopilotEnabled = 2,
    /// bit 3
    NicbaroCrosschecked = 4,
    /// bit 4
    OnGround = 8,
    /// bit 5
    Ident = 16,
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
pub enum UavionixAdsbOutRfSelect {
    /// Transceiver RF control flags for ADS-B transponder dynamic reports
    /// This enum is used to define bitmasks (work around protobuf limitations).
    /// bit 0
    Standby = 0,
    /// bit 1
    RxEnabled = 1,
    /// bit 2
    TxEnabled = 2,
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
pub enum UavionixAdsbOutDynamicGpsFix {
    /// Status for ADS-B transponder dynamic input
    None0 = 0,
    None1 = 1,
    UavionixAdsbOutDynamicGpsFix2d = 2,
    UavionixAdsbOutDynamicGpsFix3d = 3,
    Dgps = 4,
    Rtk = 5,
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
pub enum UavionixAdsbRfHealth {
    /// Status flags for ADS-B transponder dynamic output
    /// This enum is used to define bitmasks (work around protobuf limitations).
    /// bit 0
    Initializing = 0,
    /// bit 1
    Ok = 1,
    /// bit 2
    FailTx = 2,
    /// bit 5
    FailRx = 16,
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
pub enum UavionixAdsbOutCfgAircraftSize {
    /// Definitions for aircraft size
    NoData = 0,
    L15mW23m = 1,
    L25mW28p5m = 2,
    L2534m = 3,
    L3533m = 4,
    L3538m = 5,
    L4539p5m = 6,
    L4545m = 7,
    L5545m = 8,
    L5552m = 9,
    L6559p5m = 10,
    L6567m = 11,
    L75W72p5m = 12,
    L75W80m = 13,
    L85W80m = 14,
    L85W90m = 15,
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
pub enum UavionixAdsbOutCfgGpsOffsetLat {
    /// GPS lataral offset encoding
    NoData = 0,
    Left2m = 1,
    Left4m = 2,
    Left6m = 3,
    Right0m = 4,
    Right2m = 5,
    Right4m = 6,
    Right6m = 7,
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
pub enum UavionixAdsbOutCfgGpsOffsetLon {
    /// GPS longitudinal offset encoding
    NoData = 0,
    AppliedBySensor = 1,
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
pub enum UavionixAdsbEmergencyStatus {
    /// Emergency status encoding
    UavionixAdsbOutNoEmergency = 0,
    UavionixAdsbOutGeneralEmergency = 1,
    UavionixAdsbOutLifeguardEmergency = 2,
    UavionixAdsbOutMinimumFuelEmergency = 3,
    UavionixAdsbOutNoCommEmergency = 4,
    UavionixAdsbOutUnlawfulInterferanceEmergency = 5,
    UavionixAdsbOutDownedAircraftEmergency = 6,
    UavionixAdsbOutReserved = 7,
}
