/// The heartbeat message shows that a system or component is present and responding. The type and autopilot fields (along with the message component id), allow the receiving system to treat further messages from this system appropriately (e.g. by laying out the user interface based on the autopilot). This microservice is documented at https://mavlink.io/en/services/heartbeat.html
///
/// MavLink id: 55000
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Heartbeat2 {
    /// A bitfield for use for autopilot-specific flags
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub custom_mode: ::prost::alloc::vec::Vec<u32>,
    /// Vehicle or component type. For a flight controller component the vehicle type (quadrotor, helicopter, etc.). For other components the component type (e.g. camera, gimbal, etc.). This should be used in preference to component id for identifying the component type.
    #[prost(enumeration = "MavType", tag = "2")]
    pub r#type: i32,
    /// Autopilot type / class. Use MAV_AUTOPILOT_INVALID for components that are not flight controllers.
    #[prost(enumeration = "super::common::MavAutopilot", tag = "3")]
    pub autopilot: i32,
    /// System mode bitmap.
    /// bitfield defined by enum common.MAV_MODE_FLAG
    #[prost(uint32, tag = "4")]
    pub base_mode: u32,
    /// A bitfield for use for autopilot-specific flags
    #[prost(string, tag = "5")]
    pub string_test: ::prost::alloc::string::String,
    /// Some bytes (256)!
    #[prost(uint32, repeated, packed = "false", tag = "6")]
    pub bytes_test: ::prost::alloc::vec::Vec<u32>,
    /// System status flag.
    #[prost(enumeration = "super::common::MavState", tag = "7")]
    pub system_status: i32,
    /// MAVLink version, not writable by user, gets added by protocol because of magic data type: uint8_t_mavlink_version
    #[prost(uint32, tag = "8")]
    pub mavlink_version: u32,
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
pub enum MavNoVals {
    /// uno!
    MavTypeOne0 = 0,
    /// duo!
    MavTypeTwo1 = 1,
    /// tres!
    MavTypeThree2 = 2,
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
pub enum MavNoVal100 {
    /// Not used in MavLink, make protobuf happy.
    Undefined = 0,
    /// sparta!
    MavTypeHundred = 100,
    /// uno!
    MavTypeOne101 = 101,
    /// tres!
    MavTypeThree102 = 102,
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
pub enum MavType {
    /// Generic micro air vehicle
    Generic = 0,
    /// Fixed wing aircraft.
    FixedWing = 1,
    /// Quadrotor
    Quadrotor = 2,
    /// Coaxial helicopter
    Coaxial = 3,
    /// Normal helicopter with tail rotor.
    Helicopter = 4,
    /// Ground installation
    AntennaTracker = 5,
    /// Operator control unit / ground control station
    Gcs = 6,
    /// Airship, controlled
    Airship = 7,
    /// Free balloon, uncontrolled
    FreeBalloon = 8,
    /// Rocket
    Rocket = 9,
    /// Ground rover
    GroundRover = 10,
    /// Surface vessel, boat, ship
    SurfaceBoat = 11,
    /// Submarine
    Submarine = 12,
    /// Hexarotor
    Hexarotor = 13,
    /// Octorotor
    Octorotor = 14,
    /// Tricopter
    Tricopter = 15,
    /// Flapping wing
    FlappingWing = 16,
    /// Kite
    Kite = 17,
    /// Onboard companion controller
    OnboardController = 18,
    /// Two-rotor VTOL using control surfaces in vertical operation in addition. Tailsitter.
    VtolDuorotor = 19,
    /// Quad-rotor VTOL using a V-shaped quad config in vertical operation. Tailsitter.
    VtolQuadrotor = 20,
    /// Tiltrotor VTOL
    VtolTiltrotor = 21,
    /// VTOL reserved 2
    VtolReserved2 = 22,
    /// VTOL reserved 3
    VtolReserved3 = 23,
    /// VTOL reserved 4
    VtolReserved4 = 24,
    /// VTOL reserved 5
    VtolReserved5 = 25,
    /// Gimbal
    Gimbal = 26,
    /// ADSB system
    Adsb = 27,
    /// Steerable, nonrigid airfoil
    Parafoil = 28,
    /// Dodecarotor
    Dodecarotor = 29,
    /// Camera
    Camera = 30,
    /// Charging station
    ChargingStation = 31,
    /// FLARM collision avoidance system
    Flarm = 32,
    /// Servo
    Servo = 33,
    /// Open Drone ID. See https://mavlink.io/en/services/opendroneid.html.
    Odid = 34,
    /// aliens!
    Ufo = 100,
}
