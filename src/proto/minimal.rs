/// The heartbeat message shows that a system or component is present and responding. The type and autopilot fields (along with the message component id), allow the receiving system to treat further messages from this system appropriately (e.g. by laying out the user interface based on the autopilot). This microservice is documented at <https://mavlink.io/en/services/heartbeat.html>
///
/// MavLink id: 0
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Heartbeat {
    /// A bitfield for use for autopilot-specific flags
    #[prost(uint32, tag = "1")]
    pub custom_mode: u32,
    /// Type of the system (quadrotor, helicopter, etc.). Components use the same type as their associated system.
    #[prost(enumeration = "MavType", tag = "2")]
    pub r#type: i32,
    /// Autopilot type / class.
    #[prost(enumeration = "MavAutopilot", tag = "3")]
    pub autopilot: i32,
    /// System mode bitmap.
    /// bitfield defined by enum MAV_MODE_FLAG
    #[prost(uint32, tag = "4")]
    pub base_mode: u32,
    /// System status flag.
    #[prost(enumeration = "MavState", tag = "5")]
    pub system_status: i32,
    /// MAVLink version, not writable by user, gets added by protocol because of magic data type: uint8_t_mavlink_version
    #[prost(uint32, tag = "6")]
    pub mavlink_version: u32,
}
/// Version and capability of protocol version. This message is the response to REQUEST_PROTOCOL_VERSION and is used as part of the handshaking to establish which MAVLink version should be used on the network. Every node should respond to REQUEST_PROTOCOL_VERSION to enable the handshaking. Library implementers should consider adding this into the default decoding state machine to allow the protocol core to respond directly.
///
/// MavLink id: 300
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ProtocolVersion {
    /// Currently active MAVLink version number * 100: v1.0 is 100, v2.0 is 200, etc.
    #[prost(uint32, tag = "1")]
    pub version: u32,
    /// Minimum MAVLink version supported
    #[prost(uint32, tag = "2")]
    pub min_version: u32,
    /// Maximum MAVLink version supported (set to the same value as version by default)
    #[prost(uint32, tag = "3")]
    pub max_version: u32,
    /// The first 8 bytes (not characters printed in hex!) of the git hash.
    #[prost(uint32, repeated, packed = "false", tag = "4")]
    pub spec_version_hash: ::prost::alloc::vec::Vec<u32>,
    /// The first 8 bytes (not characters printed in hex!) of the git hash.
    #[prost(uint32, repeated, packed = "false", tag = "5")]
    pub library_version_hash: ::prost::alloc::vec::Vec<u32>,
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
pub enum MavAutopilot {
    /// Micro air vehicle / autopilot classes. This identifies the individual model.
    /// Generic autopilot, full support for everything
    Generic = 0,
    /// Reserved for future use.
    Reserved = 1,
    /// SLUGS autopilot, <http://slugsuav.soe.ucsc.edu>
    Slugs = 2,
    /// ArduPilot - Plane/Copter/Rover/Sub/Tracker, <http://ardupilot.org>
    Ardupilotmega = 3,
    /// OpenPilot, <http://openpilot.org>
    Openpilot = 4,
    /// Generic autopilot only supporting simple waypoints
    GenericWaypointsOnly = 5,
    /// Generic autopilot supporting waypoints and other simple navigation commands
    GenericWaypointsAndSimpleNavigationOnly = 6,
    /// Generic autopilot supporting the full mission command set
    GenericMissionFull = 7,
    /// No valid autopilot, e.g. a GCS or other MAVLink component
    Invalid = 8,
    /// PPZ UAV - <http://nongnu.org/paparazzi>
    Ppz = 9,
    /// UAV Dev Board
    Udb = 10,
    /// FlexiPilot
    Fp = 11,
    /// PX4 Autopilot - <http://px4.io/>
    Px4 = 12,
    /// SMACCMPilot - <http://smaccmpilot.org>
    Smaccmpilot = 13,
    /// AutoQuad -- <http://autoquad.org>
    Autoquad = 14,
    /// Armazila -- <http://armazila.com>
    Armazila = 15,
    /// Aerob -- <http://aerob.ru>
    Aerob = 16,
    /// ASLUAV autopilot -- <http://www.asl.ethz.ch>
    Asluav = 17,
    /// SmartAP Autopilot - <http://sky-drones.com>
    Smartap = 18,
    /// AirRails - <http://uaventure.com>
    Airrails = 19,
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
    /// MAVLINK component type reported in HEARTBEAT message. Flight controllers must report the type of the vehicle on which they are mounted (e.g. MAV_TYPE_OCTOROTOR). All other components must report a value appropriate for their type (e.g. a camera must use MAV_TYPE_CAMERA).
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
pub enum MavModeFlag {
    /// These flags encode the MAV mode.
    /// This enum is used to define bitmasks (work around protobuf limitations).
    /// Not used in MavLink, make protobuf happy.
    Undefined = 0,
    /// 0b00000001 Reserved for future use.
    /// bit 1
    CustomModeEnabled = 1,
    /// 0b00000010 system has a test mode enabled. This flag is intended for temporary system tests and should not be used for stable implementations.
    /// bit 2
    TestEnabled = 2,
    /// 0b00000100 autonomous mode enabled, system finds its own goal positions. Guided flag can be set or not, depends on the actual implementation.
    /// bit 3
    AutoEnabled = 4,
    /// 0b00001000 guided mode enabled, system flies waypoints / mission items.
    /// bit 4
    GuidedEnabled = 8,
    /// 0b00010000 system stabilizes electronically its attitude (and optionally position). It needs however further control inputs to move around.
    /// bit 5
    StabilizeEnabled = 16,
    /// 0b00100000 hardware in the loop simulation. All motors / actuators are blocked, but internal software is full operational.
    /// bit 6
    HilEnabled = 32,
    /// 0b01000000 remote control input is enabled.
    /// bit 7
    ManualInputEnabled = 64,
    /// 0b10000000 MAV safety set to armed. Motors are enabled / running / can start. Ready to fly. Additional note: this flag is to be ignore when sent in the command MAV_CMD_DO_SET_MODE and MAV_CMD_COMPONENT_ARM_DISARM shall be used instead. The flag can still be used to report the armed state.
    /// bit 8
    SafetyArmed = 128,
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
pub enum MavModeFlagDecodePosition {
    /// These values encode the bit positions of the decode position. These values can be used to read the value of a flag bit by combining the base_mode variable with AND with the flag position value. The result will be either 0 or 1, depending on if the flag is set or not.
    /// Not used in MavLink, make protobuf happy.
    Undefined = 0,
    /// Eighth bit: 00000001
    CustomMode = 1,
    /// Seventh bit: 00000010
    Test = 2,
    /// Sixth bit:   00000100
    Auto = 4,
    /// Fifth bit:  00001000
    Guided = 8,
    /// Fourth bit: 00010000
    Stabilize = 16,
    /// Third bit:  00100000
    Hil = 32,
    /// Second bit: 01000000
    Manual = 64,
    /// First bit:  10000000
    Safety = 128,
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
pub enum MavState {
    /// Uninitialized system, state is unknown.
    Uninit = 0,
    /// System is booting up.
    Boot = 1,
    /// System is calibrating and not flight-ready.
    Calibrating = 2,
    /// System is grounded and on standby. It can be launched any time.
    Standby = 3,
    /// System is active and might be already airborne. Motors are engaged.
    Active = 4,
    /// System is in a non-normal flight mode. It can however still navigate.
    Critical = 5,
    /// System is in a non-normal flight mode. It lost control over parts or over the whole airframe. It is in mayday and going down.
    Emergency = 6,
    /// System just initialized its power-down sequence, will shut down now.
    Poweroff = 7,
    /// System is terminating itself.
    FlightTermination = 8,
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
pub enum MavComponent {
    /// Component ids (values) for the different types and instances of onboard hardware/software that might make up a MAVLink system (autopilot, cameras, servos, GPS systems, avoidance systems etc.).
    /// Components must use the appropriate ID in their source address when sending messages. Components can also use IDs to determine if they are the intended recipient of an incoming message. The MAV_COMP_ID_ALL value is used to indicate messages that must be processed by all components.
    /// When creating new entries, components that can have multiple instances (e.g. cameras, servos etc.) should be allocated sequential values. An appropriate number of values should be left free after these components to allow the number of instances to be expanded.
    /// Used to broadcast messages to all components of the receiving system. Components should attempt to process messages with this component ID and forward to components on any other interfaces.
    MavCompIdAll = 0,
    /// System flight controller component ("autopilot"). Only one autopilot is expected in a particular system.
    MavCompIdAutopilot1 = 1,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser1 = 25,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser2 = 26,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser3 = 27,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser4 = 28,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser5 = 29,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser6 = 30,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser7 = 31,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser8 = 32,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser9 = 33,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser10 = 34,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser11 = 35,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser12 = 36,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser13 = 37,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser14 = 38,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser15 = 39,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser16 = 40,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser17 = 41,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser18 = 42,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser19 = 43,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser20 = 44,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser21 = 45,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser22 = 46,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser23 = 47,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser24 = 48,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser25 = 49,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser26 = 50,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser27 = 51,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser28 = 52,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser29 = 53,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser30 = 54,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser31 = 55,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser32 = 56,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser33 = 57,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser34 = 58,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser35 = 59,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser36 = 60,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser37 = 61,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser38 = 62,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser39 = 63,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser40 = 64,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser41 = 65,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser42 = 66,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser43 = 67,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser44 = 68,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser45 = 69,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser46 = 70,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser47 = 71,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser48 = 72,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser49 = 73,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser50 = 74,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser51 = 75,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser52 = 76,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser53 = 77,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser54 = 78,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser55 = 79,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser56 = 80,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser57 = 81,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser58 = 82,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser59 = 83,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser60 = 84,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser61 = 85,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser62 = 86,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser63 = 87,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser64 = 88,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser65 = 89,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser66 = 90,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser67 = 91,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser68 = 92,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser69 = 93,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser70 = 94,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser71 = 95,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser72 = 96,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser73 = 97,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser74 = 98,
    /// Id for a component on privately managed MAVLink network. Can be used for any purpose but may not be published by components outside of the private network.
    MavCompIdUser75 = 99,
    /// Camera #1.
    MavCompIdCamera = 100,
    /// Camera #2.
    MavCompIdCamera2 = 101,
    /// Camera #3.
    MavCompIdCamera3 = 102,
    /// Camera #4.
    MavCompIdCamera4 = 103,
    /// Camera #5.
    MavCompIdCamera5 = 104,
    /// Camera #6.
    MavCompIdCamera6 = 105,
    /// Servo #1.
    MavCompIdServo1 = 140,
    /// Servo #2.
    MavCompIdServo2 = 141,
    /// Servo #3.
    MavCompIdServo3 = 142,
    /// Servo #4.
    MavCompIdServo4 = 143,
    /// Servo #5.
    MavCompIdServo5 = 144,
    /// Servo #6.
    MavCompIdServo6 = 145,
    /// Servo #7.
    MavCompIdServo7 = 146,
    /// Servo #8.
    MavCompIdServo8 = 147,
    /// Servo #9.
    MavCompIdServo9 = 148,
    /// Servo #10.
    MavCompIdServo10 = 149,
    /// Servo #11.
    MavCompIdServo11 = 150,
    /// Servo #12.
    MavCompIdServo12 = 151,
    /// Servo #13.
    MavCompIdServo13 = 152,
    /// Servo #14.
    MavCompIdServo14 = 153,
    /// Gimbal component.
    MavCompIdGimbal = 154,
    /// Logging component.
    MavCompIdLog = 155,
    /// Automatic Dependent Surveillance-Broadcast (ADS-B) component.
    MavCompIdAdsb = 156,
    /// On Screen Display (OSD) devices for video links.
    MavCompIdOsd = 157,
    /// Generic autopilot peripheral component ID. Meant for devices that do not implement the parameter microservice.
    MavCompIdPeripheral = 158,
    /// Gimbal ID for QX1.
    MavCompIdQx1Gimbal = 159,
    /// FLARM collision alert component.
    MavCompIdFlarm = 160,
    /// Component that can generate/supply a mission flight plan (e.g. GCS or developer API).
    MavCompIdMissionplanner = 190,
    /// Component that finds an optimal path between points based on a certain constraint (e.g. minimum snap, shortest path, cost, etc.).
    MavCompIdPathplanner = 195,
    /// Component that plans a collision free path between two points.
    MavCompIdObstacleAvoidance = 196,
    /// Component that provides position estimates using VIO techniques.
    MavCompIdVisualInertialOdometry = 197,
    /// Component that manages pairing of vehicle and GCS.
    MavCompIdPairingManager = 198,
    /// Inertial Measurement Unit (IMU) #1.
    MavCompIdImu = 200,
    /// Inertial Measurement Unit (IMU) #2.
    MavCompIdImu2 = 201,
    /// Inertial Measurement Unit (IMU) #3.
    MavCompIdImu3 = 202,
    /// GPS #1.
    MavCompIdGps = 220,
    /// GPS #2.
    MavCompIdGps2 = 221,
    /// Component to bridge MAVLink to UDP (i.e. from a UART).
    MavCompIdUdpBridge = 240,
    /// Component to bridge to UART (i.e. from UDP).
    MavCompIdUartBridge = 241,
    /// Component for handling system messages (e.g. to ARM, takeoff, etc.).
    MavCompIdSystemControl = 250,
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
pub enum MavCmd {
    /// Commands to be executed by the MAV. They can be executed on user request, or as part of a mission script. If the action is used in a mission, the parameter mapping to the waypoint/mission message is as follows: Param 1, Param 2, Param 3, Param 4, X: Param 5, Y:Param 6, Z:Param 7. This command list is similar what ARINC 424 is for commercial aircraft: A data format how to interpret waypoint/mission data. See <https://mavlink.io/en/guide/xml_schema.html#MAV_CMD> for information about the structure of the MAV_CMD entries
    /// Not used in MavLink, make protobuf happy.
    Undefined = 0,
    /// Request the target system(s) emit a single instance of a specified message (i.e. a "one-shot" version of MAV_CMD_SET_MESSAGE_INTERVAL).
    RequestMessage = 512,
    /// ***** START Params
    /// The MAVLink message ID of the requested message.
    /// Index id (if appropriate). The use of this parameter (if any), must be defined in the requested message.
    /// The use of this parameter (if any), must be defined in the requested message. By default assumed not used (0).
    /// The use of this parameter (if any), must be defined in the requested message. By default assumed not used (0).
    /// The use of this parameter (if any), must be defined in the requested message. By default assumed not used (0).
    /// The use of this parameter (if any), must be defined in the requested message. By default assumed not used (0).
    /// Target address for requested message (if message has target address fields). 0: Flight-stack default, 1: address of requestor, 2: broadcast.
    /// ***** END Params
    /// Request MAVLink protocol version compatibility
    ///
    /// ***** START Params
    /// 1: Request supported protocol versions by all nodes on the network
    /// Reserved (all remaining params)
    /// ***** END Params
    RequestProtocolVersion = 519,
}
