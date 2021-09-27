/// The heartbeat message shows that a system or component is present and responding. The type and autopilot fields (along with the message component id), allow the receiving system to treat further messages from this system appropriately (e.g. by laying out the user interface based on the autopilot). This microservice is documented at https://mavlink.io/en/services/heartbeat.html
///
/// MavLink id: 0
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Heartbeat {
    /// A bitfield for use for autopilot-specific flags
    #[prost(uint32, tag = "1")]
    pub custom_mode: u32,
    /// Vehicle or component type. For a flight controller component the vehicle type (quadrotor, helicopter, etc.). For other components the component type (e.g. camera, gimbal, etc.). This should be used in preference to component id for identifying the component type.
    #[prost(enumeration = "MavType", tag = "2")]
    pub r#type: i32,
    /// Autopilot type / class. Use MAV_AUTOPILOT_INVALID for components that are not flight controllers.
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
/// The general system state. If the system is following the MAVLink standard, the system state is mainly defined by three orthogonal states/modes: The system mode, which is either LOCKED (motors shut down and locked), MANUAL (system under RC control), GUIDED (system with autonomous position control, position setpoint controlled manually) or AUTO (system guided by path/waypoint planner). The NAV_MODE defined the current flight state: LIFTOFF (often an open-loop maneuver), LANDING, WAYPOINTS or VECTOR. This represents the internal navigation state machine. The system status shows whether the system is currently active or not and if an emergency occurred. During the CRITICAL and EMERGENCY states the MAV is still considered to be active, but should start emergency procedures autonomously. After a failure occurred it should first move from active to critical to allow manual intervention and then move to emergency after a certain timeout.
///
/// MavLink id: 1
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SysStatus {
    /// Bitmap showing which onboard controllers and sensors are present. Value of 0: not present. Value of 1: present.
    /// bitfield defined by enum MAV_SYS_STATUS_SENSOR
    #[prost(uint32, tag = "1")]
    pub onboard_control_sensors_present: u32,
    /// Bitmap showing which onboard controllers and sensors are enabled:  Value of 0: not enabled. Value of 1: enabled.
    /// bitfield defined by enum MAV_SYS_STATUS_SENSOR
    #[prost(uint32, tag = "2")]
    pub onboard_control_sensors_enabled: u32,
    /// Bitmap showing which onboard controllers and sensors have an error (or are operational). Value of 0: error. Value of 1: healthy.
    /// bitfield defined by enum MAV_SYS_STATUS_SENSOR
    #[prost(uint32, tag = "3")]
    pub onboard_control_sensors_health: u32,
    /// Maximum usage in percent of the mainloop time. Values: [0-1000] - should always be below 1000
    #[prost(uint32, tag = "4")]
    pub load: u32,
    /// Battery voltage, UINT16_MAX: Voltage not sent by autopilot
    #[prost(uint32, tag = "5")]
    pub voltage_battery: u32,
    /// Battery current, -1: Current not sent by autopilot
    #[prost(int32, tag = "6")]
    pub current_battery: i32,
    /// Communication drop rate, (UART, I2C, SPI, CAN), dropped packets on all links (packets that were corrupted on reception on the MAV)
    #[prost(uint32, tag = "7")]
    pub drop_rate_comm: u32,
    /// Communication errors (UART, I2C, SPI, CAN), dropped packets on all links (packets that were corrupted on reception on the MAV)
    #[prost(uint32, tag = "8")]
    pub errors_comm: u32,
    /// Autopilot-specific errors
    #[prost(uint32, tag = "9")]
    pub errors_count1: u32,
    /// Autopilot-specific errors
    #[prost(uint32, tag = "10")]
    pub errors_count2: u32,
    /// Autopilot-specific errors
    #[prost(uint32, tag = "11")]
    pub errors_count3: u32,
    /// Autopilot-specific errors
    #[prost(uint32, tag = "12")]
    pub errors_count4: u32,
    /// Battery energy remaining, -1: Battery remaining energy not sent by autopilot
    #[prost(int32, tag = "13")]
    pub battery_remaining: i32,
}
/// The system time is the time of the master clock, typically the computer clock of the main onboard computer.
///
/// MavLink id: 2
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SystemTime {
    /// Timestamp (UNIX epoch time).
    #[prost(uint64, tag = "1")]
    pub time_unix_usec: u64,
    /// Timestamp (time since system boot).
    #[prost(uint32, tag = "2")]
    pub time_boot_ms: u32,
}
/// A ping message either requesting or responding to a ping. This allows to measure the system latencies, including serial port, radio modem and UDP connections. The ping microservice is documented at https://mavlink.io/en/services/ping.html
///
/// MavLink id: 4
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Ping {
    /// Timestamp (UNIX Epoch time or time since system boot). The receiving end can infer timestamp format (since 1.1.1970 or since system boot) by checking for the magnitude of the number.
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// PING sequence
    #[prost(uint32, tag = "2")]
    pub seq: u32,
    /// 0: request ping from all receiving systems. If greater than 0: message is a ping response and number is the system id of the requesting system
    #[prost(uint32, tag = "3")]
    pub target_system: u32,
    /// 0: request ping from all receiving components. If greater than 0: message is a ping response and number is the component id of the requesting component.
    #[prost(uint32, tag = "4")]
    pub target_component: u32,
}
/// Request to control this MAV
///
/// MavLink id: 5
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ChangeOperatorControl {
    /// System the GCS requests control for
    #[prost(uint32, tag = "1")]
    pub target_system: u32,
    /// 0: request control of this MAV, 1: Release control of this MAV
    #[prost(uint32, tag = "2")]
    pub control_request: u32,
    /// 0: key as plaintext, 1-255: future, different hashing/encryption variants. The GCS should in general use the safest mode possible initially and then gradually move down the encryption level if it gets a NACK message indicating an encryption mismatch.
    #[prost(uint32, tag = "3")]
    pub version: u32,
    /// Password / Key, depending on version plaintext or encrypted. 25 or less characters, NULL terminated. The characters may involve A-Z, a-z, 0-9, and "!?,.-"
    #[prost(string, tag = "4")]
    pub passkey: ::prost::alloc::string::String,
}
/// Accept / deny control of this MAV
///
/// MavLink id: 6
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ChangeOperatorControlAck {
    /// ID of the GCS this message
    #[prost(uint32, tag = "1")]
    pub gcs_system_id: u32,
    /// 0: request control of this MAV, 1: Release control of this MAV
    #[prost(uint32, tag = "2")]
    pub control_request: u32,
    /// 0: ACK, 1: NACK: Wrong passkey, 2: NACK: Unsupported passkey encryption method, 3: NACK: Already under control
    #[prost(uint32, tag = "3")]
    pub ack: u32,
}
/// Emit an encrypted signature / key identifying this system. PLEASE NOTE: This protocol has been kept simple, so transmitting the key requires an encrypted channel for true safety.
///
/// MavLink id: 7
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct AuthKey {
    /// key
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
}
/// Status generated in each node in the communication chain and injected into MAVLink stream.
///
/// MavLink id: 8
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct LinkNodeStatus {
    /// Timestamp (time since system boot).
    #[prost(uint64, tag = "1")]
    pub timestamp: u64,
    /// Transmit rate
    #[prost(uint32, tag = "2")]
    pub tx_rate: u32,
    /// Receive rate
    #[prost(uint32, tag = "3")]
    pub rx_rate: u32,
    /// Messages sent
    #[prost(uint32, tag = "4")]
    pub messages_sent: u32,
    /// Messages received (estimated from counting seq)
    #[prost(uint32, tag = "5")]
    pub messages_received: u32,
    /// Messages lost (estimated from counting seq)
    #[prost(uint32, tag = "6")]
    pub messages_lost: u32,
    /// Number of bytes that could not be parsed correctly.
    #[prost(uint32, tag = "7")]
    pub rx_parse_err: u32,
    /// Transmit buffer overflows. This number wraps around as it reaches UINT16_MAX
    #[prost(uint32, tag = "8")]
    pub tx_overflows: u32,
    /// Receive buffer overflows. This number wraps around as it reaches UINT16_MAX
    #[prost(uint32, tag = "9")]
    pub rx_overflows: u32,
    /// Remaining free transmit buffer space
    #[prost(uint32, tag = "10")]
    pub tx_buf: u32,
    /// Remaining free receive buffer space
    #[prost(uint32, tag = "11")]
    pub rx_buf: u32,
}
/// Set the system mode, as defined by enum MAV_MODE. There is no target component id as the mode is by definition for the overall aircraft, not only for one component.
///
/// MavLink id: 11
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetMode {
    /// The new autopilot-specific mode. This field can be ignored by an autopilot.
    #[prost(uint32, tag = "1")]
    pub custom_mode: u32,
    /// The system setting the mode
    #[prost(uint32, tag = "2")]
    pub target_system: u32,
    /// The new base mode.
    #[prost(enumeration = "MavMode", tag = "3")]
    pub base_mode: i32,
}
/// Request to read the onboard parameter with the param_id string id. Onboard parameters are stored as key[const char*] -> value[float]. This allows to send a parameter to any other component (such as the GCS) without the need of previous knowledge of possible parameter names. Thus the same GCS can store different parameters for different autopilots. See also https://mavlink.io/en/services/parameter.html for a full documentation of QGroundControl and IMU code.
///
/// MavLink id: 20
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ParamRequestRead {
    /// Parameter index. Send -1 to use the param ID field as identifier (else the param id will be ignored)
    #[prost(int32, tag = "1")]
    pub param_index: i32,
    /// System ID
    #[prost(uint32, tag = "2")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "3")]
    pub target_component: u32,
    /// Onboard parameter id, terminated by NULL if the length is less than 16 human-readable chars and WITHOUT null termination (NULL) byte if the length is exactly 16 chars - applications have to provide 16+1 bytes storage if the ID is stored as string
    #[prost(string, tag = "4")]
    pub param_id: ::prost::alloc::string::String,
}
/// Request all parameters of this component. After this request, all parameters are emitted. The parameter microservice is documented at https://mavlink.io/en/services/parameter.html
///
/// MavLink id: 21
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ParamRequestList {
    /// System ID
    #[prost(uint32, tag = "1")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "2")]
    pub target_component: u32,
}
/// Emit the value of a onboard parameter. The inclusion of param_count and param_index in the message allows the recipient to keep track of received parameters and allows him to re-request missing parameters after a loss or timeout. The parameter microservice is documented at https://mavlink.io/en/services/parameter.html
///
/// MavLink id: 22
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ParamValue {
    /// Onboard parameter value
    #[prost(float, tag = "1")]
    pub param_value: f32,
    /// Total number of onboard parameters
    #[prost(uint32, tag = "2")]
    pub param_count: u32,
    /// Index of this onboard parameter
    #[prost(uint32, tag = "3")]
    pub param_index: u32,
    /// Onboard parameter id, terminated by NULL if the length is less than 16 human-readable chars and WITHOUT null termination (NULL) byte if the length is exactly 16 chars - applications have to provide 16+1 bytes storage if the ID is stored as string
    #[prost(string, tag = "4")]
    pub param_id: ::prost::alloc::string::String,
    /// Onboard parameter type.
    #[prost(enumeration = "MavParamType", tag = "5")]
    pub param_type: i32,
}
/// Set a parameter value (write new value to permanent storage). IMPORTANT: The receiving component should acknowledge the new parameter value by sending a PARAM_VALUE message to all communication partners. This will also ensure that multiple GCS all have an up-to-date list of all parameters. If the sending GCS did not receive a PARAM_VALUE message within its timeout time, it should re-send the PARAM_SET message. The parameter microservice is documented at https://mavlink.io/en/services/parameter.html
///
/// MavLink id: 23
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ParamSet {
    /// Onboard parameter value
    #[prost(float, tag = "1")]
    pub param_value: f32,
    /// System ID
    #[prost(uint32, tag = "2")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "3")]
    pub target_component: u32,
    /// Onboard parameter id, terminated by NULL if the length is less than 16 human-readable chars and WITHOUT null termination (NULL) byte if the length is exactly 16 chars - applications have to provide 16+1 bytes storage if the ID is stored as string
    #[prost(string, tag = "4")]
    pub param_id: ::prost::alloc::string::String,
    /// Onboard parameter type.
    #[prost(enumeration = "MavParamType", tag = "5")]
    pub param_type: i32,
}
/// The global position, as returned by the Global Positioning System (GPS). This is
/// NOT the global position estimate of the system, but rather a RAW sensor value. See message GLOBAL_POSITION for the global position estimate.
///
/// MavLink id: 24
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GpsRawInt {
    /// Timestamp (UNIX Epoch time or time since system boot). The receiving end can infer timestamp format (since 1.1.1970 or since system boot) by checking for the magnitude of the number.
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// Latitude (WGS84, EGM96 ellipsoid)
    #[prost(int32, tag = "2")]
    pub lat: i32,
    /// Longitude (WGS84, EGM96 ellipsoid)
    #[prost(int32, tag = "3")]
    pub lon: i32,
    /// Altitude (MSL). Positive for up. Note that virtually all GPS modules provide the MSL altitude in addition to the WGS84 altitude.
    #[prost(int32, tag = "4")]
    pub alt: i32,
    /// GPS HDOP horizontal dilution of position (unitless). If unknown, set to: UINT16_MAX
    #[prost(uint32, tag = "5")]
    pub eph: u32,
    /// GPS VDOP vertical dilution of position (unitless). If unknown, set to: UINT16_MAX
    #[prost(uint32, tag = "6")]
    pub epv: u32,
    /// GPS ground speed. If unknown, set to: UINT16_MAX
    #[prost(uint32, tag = "7")]
    pub vel: u32,
    /// Course over ground (NOT heading, but direction of movement) in degrees * 100, 0.0..359.99 degrees. If unknown, set to: UINT16_MAX
    #[prost(uint32, tag = "8")]
    pub cog: u32,
    /// GPS fix type.
    #[prost(enumeration = "GpsFixType", tag = "9")]
    pub fix_type: i32,
    /// Number of satellites visible. If unknown, set to 255
    #[prost(uint32, tag = "10")]
    pub satellites_visible: u32,
}
/// The positioning status, as reported by GPS. This message is intended to display status information about each satellite visible to the receiver. See message GLOBAL_POSITION for the global position estimate. This message can contain information for up to 20 satellites.
///
/// MavLink id: 25
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GpsStatus {
    /// Number of satellites visible
    #[prost(uint32, tag = "1")]
    pub satellites_visible: u32,
    /// Global satellite ID
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub satellite_prn: ::prost::alloc::vec::Vec<u32>,
    /// 0: Satellite not used, 1: used for localization
    #[prost(uint32, repeated, packed = "false", tag = "3")]
    pub satellite_used: ::prost::alloc::vec::Vec<u32>,
    /// Elevation (0: right on top of receiver, 90: on the horizon) of satellite
    #[prost(uint32, repeated, packed = "false", tag = "4")]
    pub satellite_elevation: ::prost::alloc::vec::Vec<u32>,
    /// Direction of satellite, 0: 0 deg, 255: 360 deg.
    #[prost(uint32, repeated, packed = "false", tag = "5")]
    pub satellite_azimuth: ::prost::alloc::vec::Vec<u32>,
    /// Signal to noise ratio of satellite
    #[prost(uint32, repeated, packed = "false", tag = "6")]
    pub satellite_snr: ::prost::alloc::vec::Vec<u32>,
}
/// The RAW IMU readings for the usual 9DOF sensor setup. This message should contain the scaled values to the described units
///
/// MavLink id: 26
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ScaledImu {
    /// Timestamp (time since system boot).
    #[prost(uint32, tag = "1")]
    pub time_boot_ms: u32,
    /// X acceleration
    #[prost(int32, tag = "2")]
    pub xacc: i32,
    /// Y acceleration
    #[prost(int32, tag = "3")]
    pub yacc: i32,
    /// Z acceleration
    #[prost(int32, tag = "4")]
    pub zacc: i32,
    /// Angular speed around X axis
    #[prost(int32, tag = "5")]
    pub xgyro: i32,
    /// Angular speed around Y axis
    #[prost(int32, tag = "6")]
    pub ygyro: i32,
    /// Angular speed around Z axis
    #[prost(int32, tag = "7")]
    pub zgyro: i32,
    /// X Magnetic field
    #[prost(int32, tag = "8")]
    pub xmag: i32,
    /// Y Magnetic field
    #[prost(int32, tag = "9")]
    pub ymag: i32,
    /// Z Magnetic field
    #[prost(int32, tag = "10")]
    pub zmag: i32,
}
/// The RAW IMU readings for a 9DOF sensor, which is identified by the id (default IMU1). This message should always contain the true raw values without any scaling to allow data capture and system debugging.
///
/// MavLink id: 27
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct RawImu {
    /// Timestamp (UNIX Epoch time or time since system boot). The receiving end can infer timestamp format (since 1.1.1970 or since system boot) by checking for the magnitude of the number.
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// X acceleration (raw)
    #[prost(int32, tag = "2")]
    pub xacc: i32,
    /// Y acceleration (raw)
    #[prost(int32, tag = "3")]
    pub yacc: i32,
    /// Z acceleration (raw)
    #[prost(int32, tag = "4")]
    pub zacc: i32,
    /// Angular speed around X axis (raw)
    #[prost(int32, tag = "5")]
    pub xgyro: i32,
    /// Angular speed around Y axis (raw)
    #[prost(int32, tag = "6")]
    pub ygyro: i32,
    /// Angular speed around Z axis (raw)
    #[prost(int32, tag = "7")]
    pub zgyro: i32,
    /// X Magnetic field (raw)
    #[prost(int32, tag = "8")]
    pub xmag: i32,
    /// Y Magnetic field (raw)
    #[prost(int32, tag = "9")]
    pub ymag: i32,
    /// Z Magnetic field (raw)
    #[prost(int32, tag = "10")]
    pub zmag: i32,
}
/// The RAW pressure readings for the typical setup of one absolute pressure and one differential pressure sensor. The sensor values should be the raw, UNSCALED ADC values.
///
/// MavLink id: 28
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct RawPressure {
    /// Timestamp (UNIX Epoch time or time since system boot). The receiving end can infer timestamp format (since 1.1.1970 or since system boot) by checking for the magnitude of the number.
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// Absolute pressure (raw)
    #[prost(int32, tag = "2")]
    pub press_abs: i32,
    /// Differential pressure 1 (raw, 0 if nonexistent)
    #[prost(int32, tag = "3")]
    pub press_diff1: i32,
    /// Differential pressure 2 (raw, 0 if nonexistent)
    #[prost(int32, tag = "4")]
    pub press_diff2: i32,
    /// Raw Temperature measurement (raw)
    #[prost(int32, tag = "5")]
    pub temperature: i32,
}
/// The pressure readings for the typical setup of one absolute and differential pressure sensor. The units are as specified in each field.
///
/// MavLink id: 29
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ScaledPressure {
    /// Timestamp (time since system boot).
    #[prost(uint32, tag = "1")]
    pub time_boot_ms: u32,
    /// Absolute pressure
    #[prost(float, tag = "2")]
    pub press_abs: f32,
    /// Differential pressure 1
    #[prost(float, tag = "3")]
    pub press_diff: f32,
    /// Temperature
    #[prost(int32, tag = "4")]
    pub temperature: i32,
}
/// The attitude in the aeronautical frame (right-handed, Z-down, X-front, Y-right).
///
/// MavLink id: 30
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Attitude {
    /// Timestamp (time since system boot).
    #[prost(uint32, tag = "1")]
    pub time_boot_ms: u32,
    /// Roll angle (-pi..+pi)
    #[prost(float, tag = "2")]
    pub roll: f32,
    /// Pitch angle (-pi..+pi)
    #[prost(float, tag = "3")]
    pub pitch: f32,
    /// Yaw angle (-pi..+pi)
    #[prost(float, tag = "4")]
    pub yaw: f32,
    /// Roll angular speed
    #[prost(float, tag = "5")]
    pub rollspeed: f32,
    /// Pitch angular speed
    #[prost(float, tag = "6")]
    pub pitchspeed: f32,
    /// Yaw angular speed
    #[prost(float, tag = "7")]
    pub yawspeed: f32,
}
/// The attitude in the aeronautical frame (right-handed, Z-down, X-front, Y-right), expressed as quaternion. Quaternion order is w, x, y, z and a zero rotation would be expressed as (1 0 0 0).
///
/// MavLink id: 31
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct AttitudeQuaternion {
    /// Timestamp (time since system boot).
    #[prost(uint32, tag = "1")]
    pub time_boot_ms: u32,
    /// Quaternion component 1, w (1 in null-rotation)
    #[prost(float, tag = "2")]
    pub q1: f32,
    /// Quaternion component 2, x (0 in null-rotation)
    #[prost(float, tag = "3")]
    pub q2: f32,
    /// Quaternion component 3, y (0 in null-rotation)
    #[prost(float, tag = "4")]
    pub q3: f32,
    /// Quaternion component 4, z (0 in null-rotation)
    #[prost(float, tag = "5")]
    pub q4: f32,
    /// Roll angular speed
    #[prost(float, tag = "6")]
    pub rollspeed: f32,
    /// Pitch angular speed
    #[prost(float, tag = "7")]
    pub pitchspeed: f32,
    /// Yaw angular speed
    #[prost(float, tag = "8")]
    pub yawspeed: f32,
}
/// The filtered local position (e.g. fused computer vision and accelerometers). Coordinate frame is right-handed, Z-axis down (aeronautical frame, NED / north-east-down convention)
///
/// MavLink id: 32
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct LocalPositionNed {
    /// Timestamp (time since system boot).
    #[prost(uint32, tag = "1")]
    pub time_boot_ms: u32,
    /// X Position
    #[prost(float, tag = "2")]
    pub x: f32,
    /// Y Position
    #[prost(float, tag = "3")]
    pub y: f32,
    /// Z Position
    #[prost(float, tag = "4")]
    pub z: f32,
    /// X Speed
    #[prost(float, tag = "5")]
    pub vx: f32,
    /// Y Speed
    #[prost(float, tag = "6")]
    pub vy: f32,
    /// Z Speed
    #[prost(float, tag = "7")]
    pub vz: f32,
}
/// The filtered global position (e.g. fused GPS and accelerometers). The position is in GPS-frame (right-handed, Z-up). It
/// is designed as scaled integer message since the resolution of float is not sufficient.
///
/// MavLink id: 33
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GlobalPositionInt {
    /// Timestamp (time since system boot).
    #[prost(uint32, tag = "1")]
    pub time_boot_ms: u32,
    /// Latitude, expressed
    #[prost(int32, tag = "2")]
    pub lat: i32,
    /// Longitude, expressed
    #[prost(int32, tag = "3")]
    pub lon: i32,
    /// Altitude (MSL). Note that virtually all GPS modules provide both WGS84 and MSL.
    #[prost(int32, tag = "4")]
    pub alt: i32,
    /// Altitude above ground
    #[prost(int32, tag = "5")]
    pub relative_alt: i32,
    /// Ground X Speed (Latitude, positive north)
    #[prost(int32, tag = "6")]
    pub vx: i32,
    /// Ground Y Speed (Longitude, positive east)
    #[prost(int32, tag = "7")]
    pub vy: i32,
    /// Ground Z Speed (Altitude, positive down)
    #[prost(int32, tag = "8")]
    pub vz: i32,
    /// Vehicle heading (yaw angle), 0.0..359.99 degrees. If unknown, set to: UINT16_MAX
    #[prost(uint32, tag = "9")]
    pub hdg: u32,
}
/// The scaled values of the RC channels received: (-100%) -10000, (0%) 0, (100%) 10000. Channels that are inactive should be set to UINT16_MAX.
///
/// MavLink id: 34
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct RcChannelsScaled {
    /// Timestamp (time since system boot).
    #[prost(uint32, tag = "1")]
    pub time_boot_ms: u32,
    /// RC channel 1 value scaled.
    #[prost(int32, tag = "2")]
    pub chan1_scaled: i32,
    /// RC channel 2 value scaled.
    #[prost(int32, tag = "3")]
    pub chan2_scaled: i32,
    /// RC channel 3 value scaled.
    #[prost(int32, tag = "4")]
    pub chan3_scaled: i32,
    /// RC channel 4 value scaled.
    #[prost(int32, tag = "5")]
    pub chan4_scaled: i32,
    /// RC channel 5 value scaled.
    #[prost(int32, tag = "6")]
    pub chan5_scaled: i32,
    /// RC channel 6 value scaled.
    #[prost(int32, tag = "7")]
    pub chan6_scaled: i32,
    /// RC channel 7 value scaled.
    #[prost(int32, tag = "8")]
    pub chan7_scaled: i32,
    /// RC channel 8 value scaled.
    #[prost(int32, tag = "9")]
    pub chan8_scaled: i32,
    /// Servo output port (set of 8 outputs = 1 port). Flight stacks running on Pixhawk should use: 0 = MAIN, 1 = AUX.
    #[prost(uint32, tag = "10")]
    pub port: u32,
    /// Receive signal strength indicator in device-dependent units/scale. Values: [0-254], 255: invalid/unknown.
    #[prost(uint32, tag = "11")]
    pub rssi: u32,
}
/// The RAW values of the RC channels received. The standard PPM modulation is as follows: 1000 microseconds: 0%, 2000 microseconds: 100%. A value of UINT16_MAX implies the channel is unused. Individual receivers/transmitters might violate this specification.
///
/// MavLink id: 35
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct RcChannelsRaw {
    /// Timestamp (time since system boot).
    #[prost(uint32, tag = "1")]
    pub time_boot_ms: u32,
    /// RC channel 1 value.
    #[prost(uint32, tag = "2")]
    pub chan1_raw: u32,
    /// RC channel 2 value.
    #[prost(uint32, tag = "3")]
    pub chan2_raw: u32,
    /// RC channel 3 value.
    #[prost(uint32, tag = "4")]
    pub chan3_raw: u32,
    /// RC channel 4 value.
    #[prost(uint32, tag = "5")]
    pub chan4_raw: u32,
    /// RC channel 5 value.
    #[prost(uint32, tag = "6")]
    pub chan5_raw: u32,
    /// RC channel 6 value.
    #[prost(uint32, tag = "7")]
    pub chan6_raw: u32,
    /// RC channel 7 value.
    #[prost(uint32, tag = "8")]
    pub chan7_raw: u32,
    /// RC channel 8 value.
    #[prost(uint32, tag = "9")]
    pub chan8_raw: u32,
    /// Servo output port (set of 8 outputs = 1 port). Flight stacks running on Pixhawk should use: 0 = MAIN, 1 = AUX.
    #[prost(uint32, tag = "10")]
    pub port: u32,
    /// Receive signal strength indicator in device-dependent units/scale. Values: [0-254], 255: invalid/unknown.
    #[prost(uint32, tag = "11")]
    pub rssi: u32,
}
/// Superseded by ACTUATOR_OUTPUT_STATUS. The RAW values of the servo outputs (for RC input from the remote, use the RC_CHANNELS messages). The standard PPM modulation is as follows: 1000 microseconds: 0%, 2000 microseconds: 100%.
///
/// MavLink id: 36
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ServoOutputRaw {
    /// Timestamp (UNIX Epoch time or time since system boot). The receiving end can infer timestamp format (since 1.1.1970 or since system boot) by checking for the magnitude of the number.
    #[prost(uint32, tag = "1")]
    pub time_usec: u32,
    /// Servo output 1 value
    #[prost(uint32, tag = "2")]
    pub servo1_raw: u32,
    /// Servo output 2 value
    #[prost(uint32, tag = "3")]
    pub servo2_raw: u32,
    /// Servo output 3 value
    #[prost(uint32, tag = "4")]
    pub servo3_raw: u32,
    /// Servo output 4 value
    #[prost(uint32, tag = "5")]
    pub servo4_raw: u32,
    /// Servo output 5 value
    #[prost(uint32, tag = "6")]
    pub servo5_raw: u32,
    /// Servo output 6 value
    #[prost(uint32, tag = "7")]
    pub servo6_raw: u32,
    /// Servo output 7 value
    #[prost(uint32, tag = "8")]
    pub servo7_raw: u32,
    /// Servo output 8 value
    #[prost(uint32, tag = "9")]
    pub servo8_raw: u32,
    /// Servo output port (set of 8 outputs = 1 port). Flight stacks running on Pixhawk should use: 0 = MAIN, 1 = AUX.
    #[prost(uint32, tag = "10")]
    pub port: u32,
}
/// Request a partial list of mission items from the system/component. https://mavlink.io/en/services/mission.html. If start and end index are the same, just send one waypoint.
///
/// MavLink id: 37
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MissionRequestPartialList {
    /// Start index
    #[prost(int32, tag = "1")]
    pub start_index: i32,
    /// End index, -1 by default (-1: send list to end). Else a valid index of the list
    #[prost(int32, tag = "2")]
    pub end_index: i32,
    /// System ID
    #[prost(uint32, tag = "3")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "4")]
    pub target_component: u32,
}
/// This message is sent to the MAV to write a partial list. If start index == end index, only one item will be transmitted / updated. If the start index is NOT 0 and above the current list size, this request should be REJECTED!
///
/// MavLink id: 38
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MissionWritePartialList {
    /// Start index. Must be smaller / equal to the largest index of the current onboard list.
    #[prost(int32, tag = "1")]
    pub start_index: i32,
    /// End index, equal or greater than start index.
    #[prost(int32, tag = "2")]
    pub end_index: i32,
    /// System ID
    #[prost(uint32, tag = "3")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "4")]
    pub target_component: u32,
}
/// Message encoding a mission item. This message is emitted to announce
/// the presence of a mission item and to set a mission item on the system. The mission item can be either in x, y, z meters (type: LOCAL) or x:lat, y:lon, z:altitude. Local frame is Z-down, right handed (NED), global frame is Z-up, right handed (ENU). NaN may be used to indicate an optional/default value (e.g. to use the system's current latitude or yaw rather than a specific value). See also https://mavlink.io/en/services/mission.html.
///
/// MavLink id: 39
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MissionItem {
    /// PARAM1, see MAV_CMD enum
    #[prost(float, tag = "1")]
    pub param1: f32,
    /// PARAM2, see MAV_CMD enum
    #[prost(float, tag = "2")]
    pub param2: f32,
    /// PARAM3, see MAV_CMD enum
    #[prost(float, tag = "3")]
    pub param3: f32,
    /// PARAM4, see MAV_CMD enum
    #[prost(float, tag = "4")]
    pub param4: f32,
    /// PARAM5 / local: X coordinate, global: latitude
    #[prost(float, tag = "5")]
    pub x: f32,
    /// PARAM6 / local: Y coordinate, global: longitude
    #[prost(float, tag = "6")]
    pub y: f32,
    /// PARAM7 / local: Z coordinate, global: altitude (relative or absolute, depending on frame).
    #[prost(float, tag = "7")]
    pub z: f32,
    /// Sequence
    #[prost(uint32, tag = "8")]
    pub seq: u32,
    /// The scheduled action for the waypoint.
    #[prost(enumeration = "MavCmd", tag = "9")]
    pub command: i32,
    /// System ID
    #[prost(uint32, tag = "10")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "11")]
    pub target_component: u32,
    /// The coordinate system of the waypoint.
    #[prost(enumeration = "MavFrame", tag = "12")]
    pub frame: i32,
    /// false:0, true:1
    #[prost(uint32, tag = "13")]
    pub current: u32,
    /// Autocontinue to next waypoint
    #[prost(uint32, tag = "14")]
    pub autocontinue: u32,
}
/// Request the information of the mission item with the sequence number seq. The response of the system to this message should be a MISSION_ITEM message. https://mavlink.io/en/services/mission.html
///
/// MavLink id: 40
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MissionRequest {
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
/// Set the mission item with sequence number seq as current item. This means that the MAV will continue to this mission item on the shortest path (not following the mission items in-between).
///
/// MavLink id: 41
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MissionSetCurrent {
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
/// Message that announces the sequence number of the current active mission item. The MAV will fly towards this mission item.
///
/// MavLink id: 42
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MissionCurrent {
    /// Sequence
    #[prost(uint32, tag = "1")]
    pub seq: u32,
}
/// Request the overall list of mission items from the system/component.
///
/// MavLink id: 43
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MissionRequestList {
    /// System ID
    #[prost(uint32, tag = "1")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "2")]
    pub target_component: u32,
}
/// This message is emitted as response to MISSION_REQUEST_LIST by the MAV and to initiate a write transaction. The GCS can then request the individual mission item based on the knowledge of the total number of waypoints.
///
/// MavLink id: 44
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MissionCount {
    /// Number of mission items in the sequence
    #[prost(uint32, tag = "1")]
    pub count: u32,
    /// System ID
    #[prost(uint32, tag = "2")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "3")]
    pub target_component: u32,
}
/// Delete all mission items at once.
///
/// MavLink id: 45
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MissionClearAll {
    /// System ID
    #[prost(uint32, tag = "1")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "2")]
    pub target_component: u32,
}
/// A certain mission item has been reached. The system will either hold this position (or circle on the orbit) or (if the autocontinue on the WP was set) continue to the next waypoint.
///
/// MavLink id: 46
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MissionItemReached {
    /// Sequence
    #[prost(uint32, tag = "1")]
    pub seq: u32,
}
/// Acknowledgment message during waypoint handling. The type field states if this message is a positive ack (type=0) or if an error happened (type=non-zero).
///
/// MavLink id: 47
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MissionAck {
    /// System ID
    #[prost(uint32, tag = "1")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "2")]
    pub target_component: u32,
    /// Mission result.
    #[prost(enumeration = "MavMissionResult", tag = "3")]
    pub r#type: i32,
}
/// Sets the GPS co-ordinates of the vehicle local origin (0,0,0) position. Vehicle should emit GPS_GLOBAL_ORIGIN irrespective of whether the origin is changed. This enables transform between the local coordinate frame and the global (GPS) coordinate frame, which may be necessary when (for example) indoor and outdoor settings are connected and the MAV should move from in- to outdoor.
///
/// MavLink id: 48
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetGpsGlobalOrigin {
    /// Latitude (WGS84)
    #[prost(int32, tag = "1")]
    pub latitude: i32,
    /// Longitude (WGS84)
    #[prost(int32, tag = "2")]
    pub longitude: i32,
    /// Altitude (MSL). Positive for up.
    #[prost(int32, tag = "3")]
    pub altitude: i32,
    /// System ID
    #[prost(uint32, tag = "4")]
    pub target_system: u32,
}
/// Publishes the GPS co-ordinates of the vehicle local origin (0,0,0) position. Emitted whenever a new GPS-Local position mapping is requested or set - e.g. following SET_GPS_GLOBAL_ORIGIN message.
///
/// MavLink id: 49
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GpsGlobalOrigin {
    /// Latitude (WGS84)
    #[prost(int32, tag = "1")]
    pub latitude: i32,
    /// Longitude (WGS84)
    #[prost(int32, tag = "2")]
    pub longitude: i32,
    /// Altitude (MSL). Positive for up.
    #[prost(int32, tag = "3")]
    pub altitude: i32,
}
/// Bind a RC channel to a parameter. The parameter should change according to the RC channel value.
///
/// MavLink id: 50
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ParamMapRc {
    /// Initial parameter value
    #[prost(float, tag = "1")]
    pub param_value0: f32,
    /// Scale, maps the RC range [-1, 1] to a parameter value
    #[prost(float, tag = "2")]
    pub scale: f32,
    /// Minimum param value. The protocol does not define if this overwrites an onboard minimum value. (Depends on implementation)
    #[prost(float, tag = "3")]
    pub param_value_min: f32,
    /// Maximum param value. The protocol does not define if this overwrites an onboard maximum value. (Depends on implementation)
    #[prost(float, tag = "4")]
    pub param_value_max: f32,
    /// Parameter index. Send -1 to use the param ID field as identifier (else the param id will be ignored), send -2 to disable any existing map for this rc_channel_index.
    #[prost(int32, tag = "5")]
    pub param_index: i32,
    /// System ID
    #[prost(uint32, tag = "6")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "7")]
    pub target_component: u32,
    /// Onboard parameter id, terminated by NULL if the length is less than 16 human-readable chars and WITHOUT null termination (NULL) byte if the length is exactly 16 chars - applications have to provide 16+1 bytes storage if the ID is stored as string
    #[prost(string, tag = "8")]
    pub param_id: ::prost::alloc::string::String,
    /// Index of parameter RC channel. Not equal to the RC channel id. Typically corresponds to a potentiometer-knob on the RC.
    #[prost(uint32, tag = "9")]
    pub parameter_rc_channel_index: u32,
}
/// Request the information of the mission item with the sequence number seq. The response of the system to this message should be a MISSION_ITEM_INT message. https://mavlink.io/en/services/mission.html
///
/// MavLink id: 51
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MissionRequestInt {
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
/// A broadcast message to notify any ground station or SDK if a mission, geofence or safe points have changed on the vehicle.
///
/// MavLink id: 52
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MissionChanged {
    /// Start index for partial mission change (-1 for all items).
    #[prost(int32, tag = "1")]
    pub start_index: i32,
    /// End index of a partial mission change. -1 is a synonym for the last mission item (i.e. selects all items from start_index). Ignore field if start_index=-1.
    #[prost(int32, tag = "2")]
    pub end_index: i32,
    /// System ID of the author of the new mission.
    #[prost(uint32, tag = "3")]
    pub origin_sysid: u32,
    /// Compnent ID of the author of the new mission.
    #[prost(enumeration = "MavComponent", tag = "4")]
    pub origin_compid: i32,
    /// Mission type.
    #[prost(enumeration = "MavMissionType", tag = "5")]
    pub mission_type: i32,
}
/// Set a safety zone (volume), which is defined by two corners of a cube. This message can be used to tell the MAV which setpoints/waypoints to accept and which to reject. Safety areas are often enforced by national or competition regulations.
///
/// MavLink id: 54
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SafetySetAllowedArea {
    /// x position 1 / Latitude 1
    #[prost(float, tag = "1")]
    pub p1x: f32,
    /// y position 1 / Longitude 1
    #[prost(float, tag = "2")]
    pub p1y: f32,
    /// z position 1 / Altitude 1
    #[prost(float, tag = "3")]
    pub p1z: f32,
    /// x position 2 / Latitude 2
    #[prost(float, tag = "4")]
    pub p2x: f32,
    /// y position 2 / Longitude 2
    #[prost(float, tag = "5")]
    pub p2y: f32,
    /// z position 2 / Altitude 2
    #[prost(float, tag = "6")]
    pub p2z: f32,
    /// System ID
    #[prost(uint32, tag = "7")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "8")]
    pub target_component: u32,
    /// Coordinate frame. Can be either global, GPS, right-handed with Z axis up or local, right handed, Z axis down.
    #[prost(enumeration = "MavFrame", tag = "9")]
    pub frame: i32,
}
/// Read out the safety zone the MAV currently assumes.
///
/// MavLink id: 55
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SafetyAllowedArea {
    /// x position 1 / Latitude 1
    #[prost(float, tag = "1")]
    pub p1x: f32,
    /// y position 1 / Longitude 1
    #[prost(float, tag = "2")]
    pub p1y: f32,
    /// z position 1 / Altitude 1
    #[prost(float, tag = "3")]
    pub p1z: f32,
    /// x position 2 / Latitude 2
    #[prost(float, tag = "4")]
    pub p2x: f32,
    /// y position 2 / Longitude 2
    #[prost(float, tag = "5")]
    pub p2y: f32,
    /// z position 2 / Altitude 2
    #[prost(float, tag = "6")]
    pub p2z: f32,
    /// Coordinate frame. Can be either global, GPS, right-handed with Z axis up or local, right handed, Z axis down.
    #[prost(enumeration = "MavFrame", tag = "7")]
    pub frame: i32,
}
/// The attitude in the aeronautical frame (right-handed, Z-down, X-front, Y-right), expressed as quaternion. Quaternion order is w, x, y, z and a zero rotation would be expressed as (1 0 0 0).
///
/// MavLink id: 61
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct AttitudeQuaternionCov {
    /// Timestamp (UNIX Epoch time or time since system boot). The receiving end can infer timestamp format (since 1.1.1970 or since system boot) by checking for the magnitude of the number.
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// Quaternion components, w, x, y, z (1 0 0 0 is the null-rotation)
    #[prost(float, repeated, packed = "false", tag = "2")]
    pub q: ::prost::alloc::vec::Vec<f32>,
    /// Roll angular speed
    #[prost(float, tag = "3")]
    pub rollspeed: f32,
    /// Pitch angular speed
    #[prost(float, tag = "4")]
    pub pitchspeed: f32,
    /// Yaw angular speed
    #[prost(float, tag = "5")]
    pub yawspeed: f32,
    /// Row-major representation of a 3x3 attitude covariance matrix (states: roll, pitch, yaw; first three entries are the first ROW, next three entries are the second row, etc.). If unknown, assign NaN value to first element in the array.
    #[prost(float, repeated, packed = "false", tag = "6")]
    pub covariance: ::prost::alloc::vec::Vec<f32>,
}
/// The state of the fixed wing navigation and position controller.
///
/// MavLink id: 62
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct NavControllerOutput {
    /// Current desired roll
    #[prost(float, tag = "1")]
    pub nav_roll: f32,
    /// Current desired pitch
    #[prost(float, tag = "2")]
    pub nav_pitch: f32,
    /// Current altitude error
    #[prost(float, tag = "3")]
    pub alt_error: f32,
    /// Current airspeed error
    #[prost(float, tag = "4")]
    pub aspd_error: f32,
    /// Current crosstrack error on x-y plane
    #[prost(float, tag = "5")]
    pub xtrack_error: f32,
    /// Current desired heading
    #[prost(int32, tag = "6")]
    pub nav_bearing: i32,
    /// Bearing to current waypoint/target
    #[prost(int32, tag = "7")]
    pub target_bearing: i32,
    /// Distance to active waypoint
    #[prost(uint32, tag = "8")]
    pub wp_dist: u32,
}
/// The filtered global position (e.g. fused GPS and accelerometers). The position is in GPS-frame (right-handed, Z-up). It  is designed as scaled integer message since the resolution of float is not sufficient. NOTE: This message is intended for onboard networks / companion computers and higher-bandwidth links and optimized for accuracy and completeness. Please use the GLOBAL_POSITION_INT message for a minimal subset.
///
/// MavLink id: 63
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GlobalPositionIntCov {
    /// Timestamp (UNIX Epoch time or time since system boot). The receiving end can infer timestamp format (since 1.1.1970 or since system boot) by checking for the magnitude of the number.
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// Latitude
    #[prost(int32, tag = "2")]
    pub lat: i32,
    /// Longitude
    #[prost(int32, tag = "3")]
    pub lon: i32,
    /// Altitude in meters above MSL
    #[prost(int32, tag = "4")]
    pub alt: i32,
    /// Altitude above ground
    #[prost(int32, tag = "5")]
    pub relative_alt: i32,
    /// Ground X Speed (Latitude)
    #[prost(float, tag = "6")]
    pub vx: f32,
    /// Ground Y Speed (Longitude)
    #[prost(float, tag = "7")]
    pub vy: f32,
    /// Ground Z Speed (Altitude)
    #[prost(float, tag = "8")]
    pub vz: f32,
    /// Row-major representation of a 6x6 position and velocity 6x6 cross-covariance matrix (states: lat, lon, alt, vx, vy, vz; first six entries are the first ROW, next six entries are the second row, etc.). If unknown, assign NaN value to first element in the array.
    #[prost(float, repeated, packed = "false", tag = "9")]
    pub covariance: ::prost::alloc::vec::Vec<f32>,
    /// Class id of the estimator this estimate originated from.
    #[prost(enumeration = "MavEstimatorType", tag = "10")]
    pub estimator_type: i32,
}
/// The filtered local position (e.g. fused computer vision and accelerometers). Coordinate frame is right-handed, Z-axis down (aeronautical frame, NED / north-east-down convention)
///
/// MavLink id: 64
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct LocalPositionNedCov {
    /// Timestamp (UNIX Epoch time or time since system boot). The receiving end can infer timestamp format (since 1.1.1970 or since system boot) by checking for the magnitude of the number.
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// X Position
    #[prost(float, tag = "2")]
    pub x: f32,
    /// Y Position
    #[prost(float, tag = "3")]
    pub y: f32,
    /// Z Position
    #[prost(float, tag = "4")]
    pub z: f32,
    /// X Speed
    #[prost(float, tag = "5")]
    pub vx: f32,
    /// Y Speed
    #[prost(float, tag = "6")]
    pub vy: f32,
    /// Z Speed
    #[prost(float, tag = "7")]
    pub vz: f32,
    /// X Acceleration
    #[prost(float, tag = "8")]
    pub ax: f32,
    /// Y Acceleration
    #[prost(float, tag = "9")]
    pub ay: f32,
    /// Z Acceleration
    #[prost(float, tag = "10")]
    pub az: f32,
    /// Row-major representation of position, velocity and acceleration 9x9 cross-covariance matrix upper right triangle (states: x, y, z, vx, vy, vz, ax, ay, az; first nine entries are the first ROW, next eight entries are the second row, etc.). If unknown, assign NaN value to first element in the array.
    #[prost(float, repeated, packed = "false", tag = "11")]
    pub covariance: ::prost::alloc::vec::Vec<f32>,
    /// Class id of the estimator this estimate originated from.
    #[prost(enumeration = "MavEstimatorType", tag = "12")]
    pub estimator_type: i32,
}
/// The PPM values of the RC channels received. The standard PPM modulation is as follows: 1000 microseconds: 0%, 2000 microseconds: 100%.  A value of UINT16_MAX implies the channel is unused. Individual receivers/transmitters might violate this specification.
///
/// MavLink id: 65
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct RcChannels {
    /// Timestamp (time since system boot).
    #[prost(uint32, tag = "1")]
    pub time_boot_ms: u32,
    /// RC channel 1 value.
    #[prost(uint32, tag = "2")]
    pub chan1_raw: u32,
    /// RC channel 2 value.
    #[prost(uint32, tag = "3")]
    pub chan2_raw: u32,
    /// RC channel 3 value.
    #[prost(uint32, tag = "4")]
    pub chan3_raw: u32,
    /// RC channel 4 value.
    #[prost(uint32, tag = "5")]
    pub chan4_raw: u32,
    /// RC channel 5 value.
    #[prost(uint32, tag = "6")]
    pub chan5_raw: u32,
    /// RC channel 6 value.
    #[prost(uint32, tag = "7")]
    pub chan6_raw: u32,
    /// RC channel 7 value.
    #[prost(uint32, tag = "8")]
    pub chan7_raw: u32,
    /// RC channel 8 value.
    #[prost(uint32, tag = "9")]
    pub chan8_raw: u32,
    /// RC channel 9 value.
    #[prost(uint32, tag = "10")]
    pub chan9_raw: u32,
    /// RC channel 10 value.
    #[prost(uint32, tag = "11")]
    pub chan10_raw: u32,
    /// RC channel 11 value.
    #[prost(uint32, tag = "12")]
    pub chan11_raw: u32,
    /// RC channel 12 value.
    #[prost(uint32, tag = "13")]
    pub chan12_raw: u32,
    /// RC channel 13 value.
    #[prost(uint32, tag = "14")]
    pub chan13_raw: u32,
    /// RC channel 14 value.
    #[prost(uint32, tag = "15")]
    pub chan14_raw: u32,
    /// RC channel 15 value.
    #[prost(uint32, tag = "16")]
    pub chan15_raw: u32,
    /// RC channel 16 value.
    #[prost(uint32, tag = "17")]
    pub chan16_raw: u32,
    /// RC channel 17 value.
    #[prost(uint32, tag = "18")]
    pub chan17_raw: u32,
    /// RC channel 18 value.
    #[prost(uint32, tag = "19")]
    pub chan18_raw: u32,
    /// Total number of RC channels being received. This can be larger than 18, indicating that more channels are available but not given in this message. This value should be 0 when no RC channels are available.
    #[prost(uint32, tag = "20")]
    pub chancount: u32,
    /// Receive signal strength indicator in device-dependent units/scale. Values: [0-254], 255: invalid/unknown.
    #[prost(uint32, tag = "21")]
    pub rssi: u32,
}
/// Request a data stream.
///
/// MavLink id: 66
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct RequestDataStream {
    /// The requested message rate
    #[prost(uint32, tag = "1")]
    pub req_message_rate: u32,
    /// The target requested to send the message stream.
    #[prost(uint32, tag = "2")]
    pub target_system: u32,
    /// The target requested to send the message stream.
    #[prost(uint32, tag = "3")]
    pub target_component: u32,
    /// The ID of the requested data stream
    #[prost(uint32, tag = "4")]
    pub req_stream_id: u32,
    /// 1 to start sending, 0 to stop sending.
    #[prost(uint32, tag = "5")]
    pub start_stop: u32,
}
/// Data stream status information.
///
/// MavLink id: 67
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct DataStream {
    /// The message rate
    #[prost(uint32, tag = "1")]
    pub message_rate: u32,
    /// The ID of the requested data stream
    #[prost(uint32, tag = "2")]
    pub stream_id: u32,
    /// 1 stream is enabled, 0 stream is stopped.
    #[prost(uint32, tag = "3")]
    pub on_off: u32,
}
/// This message provides an API for manually controlling the vehicle using standard joystick axes nomenclature, along with a joystick-like input device. Unused axes can be disabled an buttons are also transmit as boolean values of their
///
/// MavLink id: 69
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ManualControl {
    /// X-axis, normalized to the range [-1000,1000]. A value of INT16_MAX indicates that this axis is invalid. Generally corresponds to forward(1000)-backward(-1000) movement on a joystick and the pitch of a vehicle.
    #[prost(int32, tag = "1")]
    pub x: i32,
    /// Y-axis, normalized to the range [-1000,1000]. A value of INT16_MAX indicates that this axis is invalid. Generally corresponds to left(-1000)-right(1000) movement on a joystick and the roll of a vehicle.
    #[prost(int32, tag = "2")]
    pub y: i32,
    /// Z-axis, normalized to the range [-1000,1000]. A value of INT16_MAX indicates that this axis is invalid. Generally corresponds to a separate slider movement with maximum being 1000 and minimum being -1000 on a joystick and the thrust of a vehicle. Positive values are positive thrust, negative values are negative thrust.
    #[prost(int32, tag = "3")]
    pub z: i32,
    /// R-axis, normalized to the range [-1000,1000]. A value of INT16_MAX indicates that this axis is invalid. Generally corresponds to a twisting of the joystick, with counter-clockwise being 1000 and clockwise being -1000, and the yaw of a vehicle.
    #[prost(int32, tag = "4")]
    pub r: i32,
    /// A bitfield corresponding to the joystick buttons' current state, 1 for pressed, 0 for released. The lowest bit corresponds to Button 1.
    #[prost(uint32, tag = "5")]
    pub buttons: u32,
    /// The system to be controlled.
    #[prost(uint32, tag = "6")]
    pub target: u32,
}
/// The RAW values of the RC channels sent to the MAV to override info received from the RC radio. A value of UINT16_MAX means no change to that channel. A value of 0 means control of that channel should be released back to the RC radio. The standard PPM modulation is as follows: 1000 microseconds: 0%, 2000 microseconds: 100%. Individual receivers/transmitters might violate this specification.
///
/// MavLink id: 70
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct RcChannelsOverride {
    /// RC channel 1 value. A value of UINT16_MAX means to ignore this field.
    #[prost(uint32, tag = "1")]
    pub chan1_raw: u32,
    /// RC channel 2 value. A value of UINT16_MAX means to ignore this field.
    #[prost(uint32, tag = "2")]
    pub chan2_raw: u32,
    /// RC channel 3 value. A value of UINT16_MAX means to ignore this field.
    #[prost(uint32, tag = "3")]
    pub chan3_raw: u32,
    /// RC channel 4 value. A value of UINT16_MAX means to ignore this field.
    #[prost(uint32, tag = "4")]
    pub chan4_raw: u32,
    /// RC channel 5 value. A value of UINT16_MAX means to ignore this field.
    #[prost(uint32, tag = "5")]
    pub chan5_raw: u32,
    /// RC channel 6 value. A value of UINT16_MAX means to ignore this field.
    #[prost(uint32, tag = "6")]
    pub chan6_raw: u32,
    /// RC channel 7 value. A value of UINT16_MAX means to ignore this field.
    #[prost(uint32, tag = "7")]
    pub chan7_raw: u32,
    /// RC channel 8 value. A value of UINT16_MAX means to ignore this field.
    #[prost(uint32, tag = "8")]
    pub chan8_raw: u32,
    /// System ID
    #[prost(uint32, tag = "9")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "10")]
    pub target_component: u32,
}
/// Message encoding a mission item. This message is emitted to announce
/// the presence of a mission item and to set a mission item on the system. The mission item can be either in x, y, z meters (type: LOCAL) or x:lat, y:lon, z:altitude. Local frame is Z-down, right handed (NED), global frame is Z-up, right handed (ENU). NaN or INT32_MAX may be used in float/integer params (respectively) to indicate optional/default values (e.g. to use the component's current latitude, yaw rather than a specific value). See also https://mavlink.io/en/services/mission.html.
///
/// MavLink id: 73
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MissionItemInt {
    /// PARAM1, see MAV_CMD enum
    #[prost(float, tag = "1")]
    pub param1: f32,
    /// PARAM2, see MAV_CMD enum
    #[prost(float, tag = "2")]
    pub param2: f32,
    /// PARAM3, see MAV_CMD enum
    #[prost(float, tag = "3")]
    pub param3: f32,
    /// PARAM4, see MAV_CMD enum
    #[prost(float, tag = "4")]
    pub param4: f32,
    /// PARAM5 / local: x position in meters * 1e4, global: latitude in degrees * 10^7
    #[prost(int32, tag = "5")]
    pub x: i32,
    /// PARAM6 / y position: local: x position in meters * 1e4, global: longitude in degrees *10^7
    #[prost(int32, tag = "6")]
    pub y: i32,
    /// PARAM7 / z position: global: altitude in meters (relative or absolute, depending on frame.
    #[prost(float, tag = "7")]
    pub z: f32,
    /// Waypoint ID (sequence number). Starts at zero. Increases monotonically for each waypoint, no gaps in the sequence (0,1,2,3,4).
    #[prost(uint32, tag = "8")]
    pub seq: u32,
    /// The scheduled action for the waypoint.
    #[prost(enumeration = "MavCmd", tag = "9")]
    pub command: i32,
    /// System ID
    #[prost(uint32, tag = "10")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "11")]
    pub target_component: u32,
    /// The coordinate system of the waypoint.
    #[prost(enumeration = "MavFrame", tag = "12")]
    pub frame: i32,
    /// false:0, true:1
    #[prost(uint32, tag = "13")]
    pub current: u32,
    /// Autocontinue to next waypoint
    #[prost(uint32, tag = "14")]
    pub autocontinue: u32,
}
/// Metrics typically displayed on a HUD for fixed wing aircraft.
///
/// MavLink id: 74
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct VfrHud {
    /// Vehicle speed in form appropriate for vehicle type. For standard aircraft this is typically calibrated airspeed (CAS) or indicated airspeed (IAS) - either of which can be used by a pilot to estimate stall speed.
    #[prost(float, tag = "1")]
    pub airspeed: f32,
    /// Current ground speed.
    #[prost(float, tag = "2")]
    pub groundspeed: f32,
    /// Current altitude (MSL).
    #[prost(float, tag = "3")]
    pub alt: f32,
    /// Current climb rate.
    #[prost(float, tag = "4")]
    pub climb: f32,
    /// Current heading in compass units (0-360, 0=north).
    #[prost(int32, tag = "5")]
    pub heading: i32,
    /// Current throttle setting (0 to 100).
    #[prost(uint32, tag = "6")]
    pub throttle: u32,
}
/// Message encoding a command with parameters as scaled integers. Scaling depends on the actual command value. The command microservice is documented at https://mavlink.io/en/services/command.html
///
/// MavLink id: 75
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CommandInt {
    /// PARAM1, see MAV_CMD enum
    #[prost(float, tag = "1")]
    pub param1: f32,
    /// PARAM2, see MAV_CMD enum
    #[prost(float, tag = "2")]
    pub param2: f32,
    /// PARAM3, see MAV_CMD enum
    #[prost(float, tag = "3")]
    pub param3: f32,
    /// PARAM4, see MAV_CMD enum
    #[prost(float, tag = "4")]
    pub param4: f32,
    /// PARAM5 / local: x position in meters * 1e4, global: latitude in degrees * 10^7
    #[prost(int32, tag = "5")]
    pub x: i32,
    /// PARAM6 / local: y position in meters * 1e4, global: longitude in degrees * 10^7
    #[prost(int32, tag = "6")]
    pub y: i32,
    /// PARAM7 / z position: global: altitude in meters (relative or absolute, depending on frame).
    #[prost(float, tag = "7")]
    pub z: f32,
    /// The scheduled action for the mission item.
    #[prost(enumeration = "MavCmd", tag = "8")]
    pub command: i32,
    /// System ID
    #[prost(uint32, tag = "9")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "10")]
    pub target_component: u32,
    /// The coordinate system of the COMMAND.
    #[prost(enumeration = "MavFrame", tag = "11")]
    pub frame: i32,
    /// false:0, true:1
    #[prost(uint32, tag = "12")]
    pub current: u32,
    /// autocontinue to next wp
    #[prost(uint32, tag = "13")]
    pub autocontinue: u32,
}
/// Send a command with up to seven parameters to the MAV. The command microservice is documented at https://mavlink.io/en/services/command.html
///
/// MavLink id: 76
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CommandLong {
    /// Parameter 1 (for the specific command).
    #[prost(float, tag = "1")]
    pub param1: f32,
    /// Parameter 2 (for the specific command).
    #[prost(float, tag = "2")]
    pub param2: f32,
    /// Parameter 3 (for the specific command).
    #[prost(float, tag = "3")]
    pub param3: f32,
    /// Parameter 4 (for the specific command).
    #[prost(float, tag = "4")]
    pub param4: f32,
    /// Parameter 5 (for the specific command).
    #[prost(float, tag = "5")]
    pub param5: f32,
    /// Parameter 6 (for the specific command).
    #[prost(float, tag = "6")]
    pub param6: f32,
    /// Parameter 7 (for the specific command).
    #[prost(float, tag = "7")]
    pub param7: f32,
    /// Command ID (of command to send).
    #[prost(enumeration = "MavCmd", tag = "8")]
    pub command: i32,
    /// System which should execute the command
    #[prost(uint32, tag = "9")]
    pub target_system: u32,
    /// Component which should execute the command, 0 for all components
    #[prost(uint32, tag = "10")]
    pub target_component: u32,
    /// 0: First transmission of this command. 1-255: Confirmation transmissions (e.g. for kill command)
    #[prost(uint32, tag = "11")]
    pub confirmation: u32,
}
/// Report status of a command. Includes feedback whether the command was executed. The command microservice is documented at https://mavlink.io/en/services/command.html
///
/// MavLink id: 77
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CommandAck {
    /// Command ID (of acknowledged command).
    #[prost(enumeration = "MavCmd", tag = "1")]
    pub command: i32,
    /// Result of command.
    #[prost(enumeration = "MavResult", tag = "2")]
    pub result: i32,
}
/// Cancel a long running command. The target system should respond with a COMMAND_ACK to the original command with result=MAV_RESULT_CANCELLED if the long running process was cancelled. If it has already completed, the cancel action can be ignored. The cancel action can be retried until some sort of acknowledgement to the original command has been received. The command microservice is documented at https://mavlink.io/en/services/command.html
///
/// MavLink id: 80
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CommandCancel {
    /// Command ID (of command to cancel).
    #[prost(enumeration = "MavCmd", tag = "1")]
    pub command: i32,
    /// System executing long running command. Should not be broadcast (0).
    #[prost(uint32, tag = "2")]
    pub target_system: u32,
    /// Component executing long running command.
    #[prost(uint32, tag = "3")]
    pub target_component: u32,
}
/// Setpoint in roll, pitch, yaw and thrust from the operator
///
/// MavLink id: 81
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ManualSetpoint {
    /// Timestamp (time since system boot).
    #[prost(uint32, tag = "1")]
    pub time_boot_ms: u32,
    /// Desired roll rate
    #[prost(float, tag = "2")]
    pub roll: f32,
    /// Desired pitch rate
    #[prost(float, tag = "3")]
    pub pitch: f32,
    /// Desired yaw rate
    #[prost(float, tag = "4")]
    pub yaw: f32,
    /// Collective thrust, normalized to 0 .. 1
    #[prost(float, tag = "5")]
    pub thrust: f32,
    /// Flight mode switch position, 0.. 255
    #[prost(uint32, tag = "6")]
    pub mode_switch: u32,
    /// Override mode switch position, 0.. 255
    #[prost(uint32, tag = "7")]
    pub manual_override_switch: u32,
}
/// Sets a desired vehicle attitude. Used by an external controller to command the vehicle (manual controller or other system).
///
/// MavLink id: 82
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetAttitudeTarget {
    /// Timestamp (time since system boot).
    #[prost(uint32, tag = "1")]
    pub time_boot_ms: u32,
    /// Attitude quaternion (w, x, y, z order, zero-rotation is 1, 0, 0, 0)
    #[prost(float, repeated, packed = "false", tag = "2")]
    pub q: ::prost::alloc::vec::Vec<f32>,
    /// Body roll rate
    #[prost(float, tag = "3")]
    pub body_roll_rate: f32,
    /// Body pitch rate
    #[prost(float, tag = "4")]
    pub body_pitch_rate: f32,
    /// Body yaw rate
    #[prost(float, tag = "5")]
    pub body_yaw_rate: f32,
    /// Collective thrust, normalized to 0 .. 1 (-1 .. 1 for vehicles capable of reverse trust)
    #[prost(float, tag = "6")]
    pub thrust: f32,
    /// System ID
    #[prost(uint32, tag = "7")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "8")]
    pub target_component: u32,
    /// Mappings: If any of these bits are set, the corresponding input should be ignored: bit 1: body roll rate, bit 2: body pitch rate, bit 3: body yaw rate. bit 4-bit 6: reserved, bit 7: throttle, bit 8: attitude
    #[prost(uint32, tag = "9")]
    pub type_mask: u32,
}
/// Reports the current commanded attitude of the vehicle as specified by the autopilot. This should match the commands sent in a SET_ATTITUDE_TARGET message if the vehicle is being controlled this way.
///
/// MavLink id: 83
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct AttitudeTarget {
    /// Timestamp (time since system boot).
    #[prost(uint32, tag = "1")]
    pub time_boot_ms: u32,
    /// Attitude quaternion (w, x, y, z order, zero-rotation is 1, 0, 0, 0)
    #[prost(float, repeated, packed = "false", tag = "2")]
    pub q: ::prost::alloc::vec::Vec<f32>,
    /// Body roll rate
    #[prost(float, tag = "3")]
    pub body_roll_rate: f32,
    /// Body pitch rate
    #[prost(float, tag = "4")]
    pub body_pitch_rate: f32,
    /// Body yaw rate
    #[prost(float, tag = "5")]
    pub body_yaw_rate: f32,
    /// Collective thrust, normalized to 0 .. 1 (-1 .. 1 for vehicles capable of reverse trust)
    #[prost(float, tag = "6")]
    pub thrust: f32,
    /// Mappings: If any of these bits are set, the corresponding input should be ignored: bit 1: body roll rate, bit 2: body pitch rate, bit 3: body yaw rate. bit 4-bit 7: reserved, bit 8: attitude
    #[prost(uint32, tag = "7")]
    pub type_mask: u32,
}
/// Sets a desired vehicle position in a local north-east-down coordinate frame. Used by an external controller to command the vehicle (manual controller or other system).
///
/// MavLink id: 84
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetPositionTargetLocalNed {
    /// Timestamp (time since system boot).
    #[prost(uint32, tag = "1")]
    pub time_boot_ms: u32,
    /// X Position in NED frame
    #[prost(float, tag = "2")]
    pub x: f32,
    /// Y Position in NED frame
    #[prost(float, tag = "3")]
    pub y: f32,
    /// Z Position in NED frame (note, altitude is negative in NED)
    #[prost(float, tag = "4")]
    pub z: f32,
    /// X velocity in NED frame
    #[prost(float, tag = "5")]
    pub vx: f32,
    /// Y velocity in NED frame
    #[prost(float, tag = "6")]
    pub vy: f32,
    /// Z velocity in NED frame
    #[prost(float, tag = "7")]
    pub vz: f32,
    /// X acceleration or force (if bit 10 of type_mask is set) in NED frame in meter / s^2 or N
    #[prost(float, tag = "8")]
    pub afx: f32,
    /// Y acceleration or force (if bit 10 of type_mask is set) in NED frame in meter / s^2 or N
    #[prost(float, tag = "9")]
    pub afy: f32,
    /// Z acceleration or force (if bit 10 of type_mask is set) in NED frame in meter / s^2 or N
    #[prost(float, tag = "10")]
    pub afz: f32,
    /// yaw setpoint
    #[prost(float, tag = "11")]
    pub yaw: f32,
    /// yaw rate setpoint
    #[prost(float, tag = "12")]
    pub yaw_rate: f32,
    /// Bitmap to indicate which dimensions should be ignored by the vehicle.
    /// bitfield defined by enum POSITION_TARGET_TYPEMASK
    #[prost(uint32, tag = "13")]
    pub type_mask: u32,
    /// System ID
    #[prost(uint32, tag = "14")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "15")]
    pub target_component: u32,
    /// Valid options are: MAV_FRAME_LOCAL_NED = 1, MAV_FRAME_LOCAL_OFFSET_NED = 7, MAV_FRAME_BODY_NED = 8, MAV_FRAME_BODY_OFFSET_NED = 9
    #[prost(enumeration = "MavFrame", tag = "16")]
    pub coordinate_frame: i32,
}
/// Reports the current commanded vehicle position, velocity, and acceleration as specified by the autopilot. This should match the commands sent in SET_POSITION_TARGET_LOCAL_NED if the vehicle is being controlled this way.
///
/// MavLink id: 85
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PositionTargetLocalNed {
    /// Timestamp (time since system boot).
    #[prost(uint32, tag = "1")]
    pub time_boot_ms: u32,
    /// X Position in NED frame
    #[prost(float, tag = "2")]
    pub x: f32,
    /// Y Position in NED frame
    #[prost(float, tag = "3")]
    pub y: f32,
    /// Z Position in NED frame (note, altitude is negative in NED)
    #[prost(float, tag = "4")]
    pub z: f32,
    /// X velocity in NED frame
    #[prost(float, tag = "5")]
    pub vx: f32,
    /// Y velocity in NED frame
    #[prost(float, tag = "6")]
    pub vy: f32,
    /// Z velocity in NED frame
    #[prost(float, tag = "7")]
    pub vz: f32,
    /// X acceleration or force (if bit 10 of type_mask is set) in NED frame in meter / s^2 or N
    #[prost(float, tag = "8")]
    pub afx: f32,
    /// Y acceleration or force (if bit 10 of type_mask is set) in NED frame in meter / s^2 or N
    #[prost(float, tag = "9")]
    pub afy: f32,
    /// Z acceleration or force (if bit 10 of type_mask is set) in NED frame in meter / s^2 or N
    #[prost(float, tag = "10")]
    pub afz: f32,
    /// yaw setpoint
    #[prost(float, tag = "11")]
    pub yaw: f32,
    /// yaw rate setpoint
    #[prost(float, tag = "12")]
    pub yaw_rate: f32,
    /// Bitmap to indicate which dimensions should be ignored by the vehicle.
    /// bitfield defined by enum POSITION_TARGET_TYPEMASK
    #[prost(uint32, tag = "13")]
    pub type_mask: u32,
    /// Valid options are: MAV_FRAME_LOCAL_NED = 1, MAV_FRAME_LOCAL_OFFSET_NED = 7, MAV_FRAME_BODY_NED = 8, MAV_FRAME_BODY_OFFSET_NED = 9
    #[prost(enumeration = "MavFrame", tag = "14")]
    pub coordinate_frame: i32,
}
/// Sets a desired vehicle position, velocity, and/or acceleration in a global coordinate system (WGS84). Used by an external controller to command the vehicle (manual controller or other system).
///
/// MavLink id: 86
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetPositionTargetGlobalInt {
    /// Timestamp (time since system boot). The rationale for the timestamp in the setpoint is to allow the system to compensate for the transport delay of the setpoint. This allows the system to compensate processing latency.
    #[prost(uint32, tag = "1")]
    pub time_boot_ms: u32,
    /// X Position in WGS84 frame
    #[prost(int32, tag = "2")]
    pub lat_int: i32,
    /// Y Position in WGS84 frame
    #[prost(int32, tag = "3")]
    pub lon_int: i32,
    /// Altitude (MSL, Relative to home, or AGL - depending on frame)
    #[prost(float, tag = "4")]
    pub alt: f32,
    /// X velocity in NED frame
    #[prost(float, tag = "5")]
    pub vx: f32,
    /// Y velocity in NED frame
    #[prost(float, tag = "6")]
    pub vy: f32,
    /// Z velocity in NED frame
    #[prost(float, tag = "7")]
    pub vz: f32,
    /// X acceleration or force (if bit 10 of type_mask is set) in NED frame in meter / s^2 or N
    #[prost(float, tag = "8")]
    pub afx: f32,
    /// Y acceleration or force (if bit 10 of type_mask is set) in NED frame in meter / s^2 or N
    #[prost(float, tag = "9")]
    pub afy: f32,
    /// Z acceleration or force (if bit 10 of type_mask is set) in NED frame in meter / s^2 or N
    #[prost(float, tag = "10")]
    pub afz: f32,
    /// yaw setpoint
    #[prost(float, tag = "11")]
    pub yaw: f32,
    /// yaw rate setpoint
    #[prost(float, tag = "12")]
    pub yaw_rate: f32,
    /// Bitmap to indicate which dimensions should be ignored by the vehicle.
    /// bitfield defined by enum POSITION_TARGET_TYPEMASK
    #[prost(uint32, tag = "13")]
    pub type_mask: u32,
    /// System ID
    #[prost(uint32, tag = "14")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "15")]
    pub target_component: u32,
    /// Valid options are: MAV_FRAME_GLOBAL_INT = 5, MAV_FRAME_GLOBAL_RELATIVE_ALT_INT = 6, MAV_FRAME_GLOBAL_TERRAIN_ALT_INT = 11
    #[prost(enumeration = "MavFrame", tag = "16")]
    pub coordinate_frame: i32,
}
/// Reports the current commanded vehicle position, velocity, and acceleration as specified by the autopilot. This should match the commands sent in SET_POSITION_TARGET_GLOBAL_INT if the vehicle is being controlled this way.
///
/// MavLink id: 87
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PositionTargetGlobalInt {
    /// Timestamp (time since system boot). The rationale for the timestamp in the setpoint is to allow the system to compensate for the transport delay of the setpoint. This allows the system to compensate processing latency.
    #[prost(uint32, tag = "1")]
    pub time_boot_ms: u32,
    /// X Position in WGS84 frame
    #[prost(int32, tag = "2")]
    pub lat_int: i32,
    /// Y Position in WGS84 frame
    #[prost(int32, tag = "3")]
    pub lon_int: i32,
    /// Altitude (MSL, AGL or relative to home altitude, depending on frame)
    #[prost(float, tag = "4")]
    pub alt: f32,
    /// X velocity in NED frame
    #[prost(float, tag = "5")]
    pub vx: f32,
    /// Y velocity in NED frame
    #[prost(float, tag = "6")]
    pub vy: f32,
    /// Z velocity in NED frame
    #[prost(float, tag = "7")]
    pub vz: f32,
    /// X acceleration or force (if bit 10 of type_mask is set) in NED frame in meter / s^2 or N
    #[prost(float, tag = "8")]
    pub afx: f32,
    /// Y acceleration or force (if bit 10 of type_mask is set) in NED frame in meter / s^2 or N
    #[prost(float, tag = "9")]
    pub afy: f32,
    /// Z acceleration or force (if bit 10 of type_mask is set) in NED frame in meter / s^2 or N
    #[prost(float, tag = "10")]
    pub afz: f32,
    /// yaw setpoint
    #[prost(float, tag = "11")]
    pub yaw: f32,
    /// yaw rate setpoint
    #[prost(float, tag = "12")]
    pub yaw_rate: f32,
    /// Bitmap to indicate which dimensions should be ignored by the vehicle.
    /// bitfield defined by enum POSITION_TARGET_TYPEMASK
    #[prost(uint32, tag = "13")]
    pub type_mask: u32,
    /// Valid options are: MAV_FRAME_GLOBAL_INT = 5, MAV_FRAME_GLOBAL_RELATIVE_ALT_INT = 6, MAV_FRAME_GLOBAL_TERRAIN_ALT_INT = 11
    #[prost(enumeration = "MavFrame", tag = "14")]
    pub coordinate_frame: i32,
}
/// The offset in X, Y, Z and yaw between the LOCAL_POSITION_NED messages of MAV X and the global coordinate frame in NED coordinates. Coordinate frame is right-handed, Z-axis down (aeronautical frame, NED / north-east-down convention)
///
/// MavLink id: 89
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct LocalPositionNedSystemGlobalOffset {
    /// Timestamp (time since system boot).
    #[prost(uint32, tag = "1")]
    pub time_boot_ms: u32,
    /// X Position
    #[prost(float, tag = "2")]
    pub x: f32,
    /// Y Position
    #[prost(float, tag = "3")]
    pub y: f32,
    /// Z Position
    #[prost(float, tag = "4")]
    pub z: f32,
    /// Roll
    #[prost(float, tag = "5")]
    pub roll: f32,
    /// Pitch
    #[prost(float, tag = "6")]
    pub pitch: f32,
    /// Yaw
    #[prost(float, tag = "7")]
    pub yaw: f32,
}
/// Sent from simulation to autopilot. This packet is useful for high throughput applications such as hardware in the loop simulations.
///
/// MavLink id: 90
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct HilState {
    /// Timestamp (UNIX Epoch time or time since system boot). The receiving end can infer timestamp format (since 1.1.1970 or since system boot) by checking for the magnitude of the number.
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// Roll angle
    #[prost(float, tag = "2")]
    pub roll: f32,
    /// Pitch angle
    #[prost(float, tag = "3")]
    pub pitch: f32,
    /// Yaw angle
    #[prost(float, tag = "4")]
    pub yaw: f32,
    /// Body frame roll / phi angular speed
    #[prost(float, tag = "5")]
    pub rollspeed: f32,
    /// Body frame pitch / theta angular speed
    #[prost(float, tag = "6")]
    pub pitchspeed: f32,
    /// Body frame yaw / psi angular speed
    #[prost(float, tag = "7")]
    pub yawspeed: f32,
    /// Latitude
    #[prost(int32, tag = "8")]
    pub lat: i32,
    /// Longitude
    #[prost(int32, tag = "9")]
    pub lon: i32,
    /// Altitude
    #[prost(int32, tag = "10")]
    pub alt: i32,
    /// Ground X Speed (Latitude)
    #[prost(int32, tag = "11")]
    pub vx: i32,
    /// Ground Y Speed (Longitude)
    #[prost(int32, tag = "12")]
    pub vy: i32,
    /// Ground Z Speed (Altitude)
    #[prost(int32, tag = "13")]
    pub vz: i32,
    /// X acceleration
    #[prost(int32, tag = "14")]
    pub xacc: i32,
    /// Y acceleration
    #[prost(int32, tag = "15")]
    pub yacc: i32,
    /// Z acceleration
    #[prost(int32, tag = "16")]
    pub zacc: i32,
}
/// Sent from autopilot to simulation. Hardware in the loop control outputs
///
/// MavLink id: 91
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct HilControls {
    /// Timestamp (UNIX Epoch time or time since system boot). The receiving end can infer timestamp format (since 1.1.1970 or since system boot) by checking for the magnitude of the number.
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// Control output -1 .. 1
    #[prost(float, tag = "2")]
    pub roll_ailerons: f32,
    /// Control output -1 .. 1
    #[prost(float, tag = "3")]
    pub pitch_elevator: f32,
    /// Control output -1 .. 1
    #[prost(float, tag = "4")]
    pub yaw_rudder: f32,
    /// Throttle 0 .. 1
    #[prost(float, tag = "5")]
    pub throttle: f32,
    /// Aux 1, -1 .. 1
    #[prost(float, tag = "6")]
    pub aux1: f32,
    /// Aux 2, -1 .. 1
    #[prost(float, tag = "7")]
    pub aux2: f32,
    /// Aux 3, -1 .. 1
    #[prost(float, tag = "8")]
    pub aux3: f32,
    /// Aux 4, -1 .. 1
    #[prost(float, tag = "9")]
    pub aux4: f32,
    /// System mode.
    #[prost(enumeration = "MavMode", tag = "10")]
    pub mode: i32,
    /// Navigation mode (MAV_NAV_MODE)
    #[prost(uint32, tag = "11")]
    pub nav_mode: u32,
}
/// Sent from simulation to autopilot. The RAW values of the RC channels received. The standard PPM modulation is as follows: 1000 microseconds: 0%, 2000 microseconds: 100%. Individual receivers/transmitters might violate this specification.
///
/// MavLink id: 92
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct HilRcInputsRaw {
    /// Timestamp (UNIX Epoch time or time since system boot). The receiving end can infer timestamp format (since 1.1.1970 or since system boot) by checking for the magnitude of the number.
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// RC channel 1 value
    #[prost(uint32, tag = "2")]
    pub chan1_raw: u32,
    /// RC channel 2 value
    #[prost(uint32, tag = "3")]
    pub chan2_raw: u32,
    /// RC channel 3 value
    #[prost(uint32, tag = "4")]
    pub chan3_raw: u32,
    /// RC channel 4 value
    #[prost(uint32, tag = "5")]
    pub chan4_raw: u32,
    /// RC channel 5 value
    #[prost(uint32, tag = "6")]
    pub chan5_raw: u32,
    /// RC channel 6 value
    #[prost(uint32, tag = "7")]
    pub chan6_raw: u32,
    /// RC channel 7 value
    #[prost(uint32, tag = "8")]
    pub chan7_raw: u32,
    /// RC channel 8 value
    #[prost(uint32, tag = "9")]
    pub chan8_raw: u32,
    /// RC channel 9 value
    #[prost(uint32, tag = "10")]
    pub chan9_raw: u32,
    /// RC channel 10 value
    #[prost(uint32, tag = "11")]
    pub chan10_raw: u32,
    /// RC channel 11 value
    #[prost(uint32, tag = "12")]
    pub chan11_raw: u32,
    /// RC channel 12 value
    #[prost(uint32, tag = "13")]
    pub chan12_raw: u32,
    /// Receive signal strength indicator in device-dependent units/scale. Values: [0-254], 255: invalid/unknown.
    #[prost(uint32, tag = "14")]
    pub rssi: u32,
}
/// Sent from autopilot to simulation. Hardware in the loop control outputs (replacement for HIL_CONTROLS)
///
/// MavLink id: 93
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct HilActuatorControls {
    /// Timestamp (UNIX Epoch time or time since system boot). The receiving end can infer timestamp format (since 1.1.1970 or since system boot) by checking for the magnitude of the number.
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// Flags as bitfield, 1: indicate simulation using lockstep.
    #[prost(uint64, tag = "2")]
    pub flags: u64,
    /// Control outputs -1 .. 1. Channel assignment depends on the simulated hardware.
    #[prost(float, repeated, packed = "false", tag = "3")]
    pub controls: ::prost::alloc::vec::Vec<f32>,
    /// System mode. Includes arming state.
    /// bitfield defined by enum MAV_MODE_FLAG
    #[prost(uint32, tag = "4")]
    pub mode: u32,
}
/// Optical flow from a flow sensor (e.g. optical mouse sensor)
///
/// MavLink id: 100
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct OpticalFlow {
    /// Timestamp (UNIX Epoch time or time since system boot). The receiving end can infer timestamp format (since 1.1.1970 or since system boot) by checking for the magnitude of the number.
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// Flow in x-sensor direction, angular-speed compensated
    #[prost(float, tag = "2")]
    pub flow_comp_m_x: f32,
    /// Flow in y-sensor direction, angular-speed compensated
    #[prost(float, tag = "3")]
    pub flow_comp_m_y: f32,
    /// Ground distance. Positive value: distance known. Negative value: Unknown distance
    #[prost(float, tag = "4")]
    pub ground_distance: f32,
    /// Flow in x-sensor direction
    #[prost(int32, tag = "5")]
    pub flow_x: i32,
    /// Flow in y-sensor direction
    #[prost(int32, tag = "6")]
    pub flow_y: i32,
    /// Sensor ID
    #[prost(uint32, tag = "7")]
    pub sensor_id: u32,
    /// Optical flow quality / confidence. 0: bad, 255: maximum quality
    #[prost(uint32, tag = "8")]
    pub quality: u32,
}
/// Global position/attitude estimate from a vision source.
///
/// MavLink id: 101
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GlobalVisionPositionEstimate {
    /// Timestamp (UNIX time or since system boot)
    #[prost(uint64, tag = "1")]
    pub usec: u64,
    /// Global X position
    #[prost(float, tag = "2")]
    pub x: f32,
    /// Global Y position
    #[prost(float, tag = "3")]
    pub y: f32,
    /// Global Z position
    #[prost(float, tag = "4")]
    pub z: f32,
    /// Roll angle
    #[prost(float, tag = "5")]
    pub roll: f32,
    /// Pitch angle
    #[prost(float, tag = "6")]
    pub pitch: f32,
    /// Yaw angle
    #[prost(float, tag = "7")]
    pub yaw: f32,
}
/// Local position/attitude estimate from a vision source.
///
/// MavLink id: 102
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct VisionPositionEstimate {
    /// Timestamp (UNIX time or time since system boot)
    #[prost(uint64, tag = "1")]
    pub usec: u64,
    /// Local X position
    #[prost(float, tag = "2")]
    pub x: f32,
    /// Local Y position
    #[prost(float, tag = "3")]
    pub y: f32,
    /// Local Z position
    #[prost(float, tag = "4")]
    pub z: f32,
    /// Roll angle
    #[prost(float, tag = "5")]
    pub roll: f32,
    /// Pitch angle
    #[prost(float, tag = "6")]
    pub pitch: f32,
    /// Yaw angle
    #[prost(float, tag = "7")]
    pub yaw: f32,
}
/// Speed estimate from a vision source.
///
/// MavLink id: 103
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct VisionSpeedEstimate {
    /// Timestamp (UNIX time or time since system boot)
    #[prost(uint64, tag = "1")]
    pub usec: u64,
    /// Global X speed
    #[prost(float, tag = "2")]
    pub x: f32,
    /// Global Y speed
    #[prost(float, tag = "3")]
    pub y: f32,
    /// Global Z speed
    #[prost(float, tag = "4")]
    pub z: f32,
}
/// Global position estimate from a Vicon motion system source.
///
/// MavLink id: 104
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ViconPositionEstimate {
    /// Timestamp (UNIX time or time since system boot)
    #[prost(uint64, tag = "1")]
    pub usec: u64,
    /// Global X position
    #[prost(float, tag = "2")]
    pub x: f32,
    /// Global Y position
    #[prost(float, tag = "3")]
    pub y: f32,
    /// Global Z position
    #[prost(float, tag = "4")]
    pub z: f32,
    /// Roll angle
    #[prost(float, tag = "5")]
    pub roll: f32,
    /// Pitch angle
    #[prost(float, tag = "6")]
    pub pitch: f32,
    /// Yaw angle
    #[prost(float, tag = "7")]
    pub yaw: f32,
}
/// The IMU readings in SI units in NED body frame
///
/// MavLink id: 105
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct HighresImu {
    /// Timestamp (UNIX Epoch time or time since system boot). The receiving end can infer timestamp format (since 1.1.1970 or since system boot) by checking for the magnitude of the number.
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// X acceleration
    #[prost(float, tag = "2")]
    pub xacc: f32,
    /// Y acceleration
    #[prost(float, tag = "3")]
    pub yacc: f32,
    /// Z acceleration
    #[prost(float, tag = "4")]
    pub zacc: f32,
    /// Angular speed around X axis
    #[prost(float, tag = "5")]
    pub xgyro: f32,
    /// Angular speed around Y axis
    #[prost(float, tag = "6")]
    pub ygyro: f32,
    /// Angular speed around Z axis
    #[prost(float, tag = "7")]
    pub zgyro: f32,
    /// X Magnetic field
    #[prost(float, tag = "8")]
    pub xmag: f32,
    /// Y Magnetic field
    #[prost(float, tag = "9")]
    pub ymag: f32,
    /// Z Magnetic field
    #[prost(float, tag = "10")]
    pub zmag: f32,
    /// Absolute pressure
    #[prost(float, tag = "11")]
    pub abs_pressure: f32,
    /// Differential pressure
    #[prost(float, tag = "12")]
    pub diff_pressure: f32,
    /// Altitude calculated from pressure
    #[prost(float, tag = "13")]
    pub pressure_alt: f32,
    /// Temperature
    #[prost(float, tag = "14")]
    pub temperature: f32,
    /// Bitmap for fields that have updated since last message, bit 0 = xacc, bit 12: temperature
    #[prost(uint32, tag = "15")]
    pub fields_updated: u32,
}
/// Optical flow from an angular rate flow sensor (e.g. PX4FLOW or mouse sensor)
///
/// MavLink id: 106
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct OpticalFlowRad {
    /// Timestamp (UNIX Epoch time or time since system boot). The receiving end can infer timestamp format (since 1.1.1970 or since system boot) by checking for the magnitude of the number.
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// Integration time. Divide integrated_x and integrated_y by the integration time to obtain average flow. The integration time also indicates the.
    #[prost(uint32, tag = "2")]
    pub integration_time_us: u32,
    /// Flow around X axis (Sensor RH rotation about the X axis induces a positive flow. Sensor linear motion along the positive Y axis induces a negative flow.)
    #[prost(float, tag = "3")]
    pub integrated_x: f32,
    /// Flow around Y axis (Sensor RH rotation about the Y axis induces a positive flow. Sensor linear motion along the positive X axis induces a positive flow.)
    #[prost(float, tag = "4")]
    pub integrated_y: f32,
    /// RH rotation around X axis
    #[prost(float, tag = "5")]
    pub integrated_xgyro: f32,
    /// RH rotation around Y axis
    #[prost(float, tag = "6")]
    pub integrated_ygyro: f32,
    /// RH rotation around Z axis
    #[prost(float, tag = "7")]
    pub integrated_zgyro: f32,
    /// Time since the distance was sampled.
    #[prost(uint32, tag = "8")]
    pub time_delta_distance_us: u32,
    /// Distance to the center of the flow field. Positive value (including zero): distance known. Negative value: Unknown distance.
    #[prost(float, tag = "9")]
    pub distance: f32,
    /// Temperature
    #[prost(int32, tag = "10")]
    pub temperature: i32,
    /// Sensor ID
    #[prost(uint32, tag = "11")]
    pub sensor_id: u32,
    /// Optical flow quality / confidence. 0: no valid flow, 255: maximum quality
    #[prost(uint32, tag = "12")]
    pub quality: u32,
}
/// The IMU readings in SI units in NED body frame
///
/// MavLink id: 107
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct HilSensor {
    /// Timestamp (UNIX Epoch time or time since system boot). The receiving end can infer timestamp format (since 1.1.1970 or since system boot) by checking for the magnitude of the number.
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// X acceleration
    #[prost(float, tag = "2")]
    pub xacc: f32,
    /// Y acceleration
    #[prost(float, tag = "3")]
    pub yacc: f32,
    /// Z acceleration
    #[prost(float, tag = "4")]
    pub zacc: f32,
    /// Angular speed around X axis in body frame
    #[prost(float, tag = "5")]
    pub xgyro: f32,
    /// Angular speed around Y axis in body frame
    #[prost(float, tag = "6")]
    pub ygyro: f32,
    /// Angular speed around Z axis in body frame
    #[prost(float, tag = "7")]
    pub zgyro: f32,
    /// X Magnetic field
    #[prost(float, tag = "8")]
    pub xmag: f32,
    /// Y Magnetic field
    #[prost(float, tag = "9")]
    pub ymag: f32,
    /// Z Magnetic field
    #[prost(float, tag = "10")]
    pub zmag: f32,
    /// Absolute pressure
    #[prost(float, tag = "11")]
    pub abs_pressure: f32,
    /// Differential pressure (airspeed)
    #[prost(float, tag = "12")]
    pub diff_pressure: f32,
    /// Altitude calculated from pressure
    #[prost(float, tag = "13")]
    pub pressure_alt: f32,
    /// Temperature
    #[prost(float, tag = "14")]
    pub temperature: f32,
    /// Bitmap for fields that have updated since last message, bit 0 = xacc, bit 12: temperature, bit 31: full reset of attitude/position/velocities/etc was performed in sim.
    #[prost(uint32, tag = "15")]
    pub fields_updated: u32,
}
/// Status of simulation environment, if used
///
/// MavLink id: 108
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SimState {
    /// True attitude quaternion component 1, w (1 in null-rotation)
    #[prost(float, tag = "1")]
    pub q1: f32,
    /// True attitude quaternion component 2, x (0 in null-rotation)
    #[prost(float, tag = "2")]
    pub q2: f32,
    /// True attitude quaternion component 3, y (0 in null-rotation)
    #[prost(float, tag = "3")]
    pub q3: f32,
    /// True attitude quaternion component 4, z (0 in null-rotation)
    #[prost(float, tag = "4")]
    pub q4: f32,
    /// Attitude roll expressed as Euler angles, not recommended except for human-readable outputs
    #[prost(float, tag = "5")]
    pub roll: f32,
    /// Attitude pitch expressed as Euler angles, not recommended except for human-readable outputs
    #[prost(float, tag = "6")]
    pub pitch: f32,
    /// Attitude yaw expressed as Euler angles, not recommended except for human-readable outputs
    #[prost(float, tag = "7")]
    pub yaw: f32,
    /// X acceleration
    #[prost(float, tag = "8")]
    pub xacc: f32,
    /// Y acceleration
    #[prost(float, tag = "9")]
    pub yacc: f32,
    /// Z acceleration
    #[prost(float, tag = "10")]
    pub zacc: f32,
    /// Angular speed around X axis
    #[prost(float, tag = "11")]
    pub xgyro: f32,
    /// Angular speed around Y axis
    #[prost(float, tag = "12")]
    pub ygyro: f32,
    /// Angular speed around Z axis
    #[prost(float, tag = "13")]
    pub zgyro: f32,
    /// Latitude
    #[prost(float, tag = "14")]
    pub lat: f32,
    /// Longitude
    #[prost(float, tag = "15")]
    pub lon: f32,
    /// Altitude
    #[prost(float, tag = "16")]
    pub alt: f32,
    /// Horizontal position standard deviation
    #[prost(float, tag = "17")]
    pub std_dev_horz: f32,
    /// Vertical position standard deviation
    #[prost(float, tag = "18")]
    pub std_dev_vert: f32,
    /// True velocity in north direction in earth-fixed NED frame
    #[prost(float, tag = "19")]
    pub vn: f32,
    /// True velocity in east direction in earth-fixed NED frame
    #[prost(float, tag = "20")]
    pub ve: f32,
    /// True velocity in down direction in earth-fixed NED frame
    #[prost(float, tag = "21")]
    pub vd: f32,
}
/// Status generated by radio and injected into MAVLink stream.
///
/// MavLink id: 109
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct RadioStatus {
    /// Count of radio packet receive errors (since boot).
    #[prost(uint32, tag = "1")]
    pub rxerrors: u32,
    /// Count of error corrected radio packets (since boot).
    #[prost(uint32, tag = "2")]
    pub fixed: u32,
    /// Local (message sender) recieved signal strength indication in device-dependent units/scale. Values: [0-254], 255: invalid/unknown.
    #[prost(uint32, tag = "3")]
    pub rssi: u32,
    /// Remote (message receiver) signal strength indication in device-dependent units/scale. Values: [0-254], 255: invalid/unknown.
    #[prost(uint32, tag = "4")]
    pub remrssi: u32,
    /// Remaining free transmitter buffer space.
    #[prost(uint32, tag = "5")]
    pub txbuf: u32,
    /// Local background noise level. These are device dependent RSSI values (scale as approx 2x dB on SiK radios). Values: [0-254], 255: invalid/unknown.
    #[prost(uint32, tag = "6")]
    pub noise: u32,
    /// Remote background noise level. These are device dependent RSSI values (scale as approx 2x dB on SiK radios). Values: [0-254], 255: invalid/unknown.
    #[prost(uint32, tag = "7")]
    pub remnoise: u32,
}
/// File transfer message
///
/// MavLink id: 110
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct FileTransferProtocol {
    /// Network ID (0 for broadcast)
    #[prost(uint32, tag = "1")]
    pub target_network: u32,
    /// System ID (0 for broadcast)
    #[prost(uint32, tag = "2")]
    pub target_system: u32,
    /// Component ID (0 for broadcast)
    #[prost(uint32, tag = "3")]
    pub target_component: u32,
    /// Variable length payload. The length is defined by the remaining message length when subtracting the header and other fields.  The entire content of this block is opaque unless you understand any the encoding message_type.  The particular encoding used can be extension specific and might not always be documented as part of the mavlink specification.
    #[prost(uint32, repeated, packed = "false", tag = "4")]
    pub payload: ::prost::alloc::vec::Vec<u32>,
}
/// Time synchronization message.
///
/// MavLink id: 111
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Timesync {
    /// Time sync timestamp 1
    #[prost(int64, tag = "1")]
    pub tc1: i64,
    /// Time sync timestamp 2
    #[prost(int64, tag = "2")]
    pub ts1: i64,
}
/// Camera-IMU triggering and synchronisation message.
///
/// MavLink id: 112
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CameraTrigger {
    /// Timestamp for image frame (UNIX Epoch time or time since system boot). The receiving end can infer timestamp format (since 1.1.1970 or since system boot) by checking for the magnitude of the number.
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// Image frame sequence
    #[prost(uint32, tag = "2")]
    pub seq: u32,
}
/// The global position, as returned by the Global Positioning System (GPS). This is
/// NOT the global position estimate of the sytem, but rather a RAW sensor value. See message GLOBAL_POSITION for the global position estimate.
///
/// MavLink id: 113
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct HilGps {
    /// Timestamp (UNIX Epoch time or time since system boot). The receiving end can infer timestamp format (since 1.1.1970 or since system boot) by checking for the magnitude of the number.
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// Latitude (WGS84)
    #[prost(int32, tag = "2")]
    pub lat: i32,
    /// Longitude (WGS84)
    #[prost(int32, tag = "3")]
    pub lon: i32,
    /// Altitude (MSL). Positive for up.
    #[prost(int32, tag = "4")]
    pub alt: i32,
    /// GPS HDOP horizontal dilution of position. If unknown, set to: 65535
    #[prost(uint32, tag = "5")]
    pub eph: u32,
    /// GPS VDOP vertical dilution of position. If unknown, set to: 65535
    #[prost(uint32, tag = "6")]
    pub epv: u32,
    /// GPS ground speed. If unknown, set to: 65535
    #[prost(uint32, tag = "7")]
    pub vel: u32,
    /// GPS velocity in north direction in earth-fixed NED frame
    #[prost(int32, tag = "8")]
    pub vn: i32,
    /// GPS velocity in east direction in earth-fixed NED frame
    #[prost(int32, tag = "9")]
    pub ve: i32,
    /// GPS velocity in down direction in earth-fixed NED frame
    #[prost(int32, tag = "10")]
    pub vd: i32,
    /// Course over ground (NOT heading, but direction of movement), 0.0..359.99 degrees. If unknown, set to: 65535
    #[prost(uint32, tag = "11")]
    pub cog: u32,
    /// 0-1: no fix, 2: 2D fix, 3: 3D fix. Some applications will not use the value of this field unless it is at least two, so always correctly fill in the fix.
    #[prost(uint32, tag = "12")]
    pub fix_type: u32,
    /// Number of satellites visible. If unknown, set to 255
    #[prost(uint32, tag = "13")]
    pub satellites_visible: u32,
}
/// Simulated optical flow from a flow sensor (e.g. PX4FLOW or optical mouse sensor)
///
/// MavLink id: 114
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct HilOpticalFlow {
    /// Timestamp (UNIX Epoch time or time since system boot). The receiving end can infer timestamp format (since 1.1.1970 or since system boot) by checking for the magnitude of the number.
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// Integration time. Divide integrated_x and integrated_y by the integration time to obtain average flow. The integration time also indicates the.
    #[prost(uint32, tag = "2")]
    pub integration_time_us: u32,
    /// Flow in radians around X axis (Sensor RH rotation about the X axis induces a positive flow. Sensor linear motion along the positive Y axis induces a negative flow.)
    #[prost(float, tag = "3")]
    pub integrated_x: f32,
    /// Flow in radians around Y axis (Sensor RH rotation about the Y axis induces a positive flow. Sensor linear motion along the positive X axis induces a positive flow.)
    #[prost(float, tag = "4")]
    pub integrated_y: f32,
    /// RH rotation around X axis
    #[prost(float, tag = "5")]
    pub integrated_xgyro: f32,
    /// RH rotation around Y axis
    #[prost(float, tag = "6")]
    pub integrated_ygyro: f32,
    /// RH rotation around Z axis
    #[prost(float, tag = "7")]
    pub integrated_zgyro: f32,
    /// Time since the distance was sampled.
    #[prost(uint32, tag = "8")]
    pub time_delta_distance_us: u32,
    /// Distance to the center of the flow field. Positive value (including zero): distance known. Negative value: Unknown distance.
    #[prost(float, tag = "9")]
    pub distance: f32,
    /// Temperature
    #[prost(int32, tag = "10")]
    pub temperature: i32,
    /// Sensor ID
    #[prost(uint32, tag = "11")]
    pub sensor_id: u32,
    /// Optical flow quality / confidence. 0: no valid flow, 255: maximum quality
    #[prost(uint32, tag = "12")]
    pub quality: u32,
}
/// Sent from simulation to autopilot, avoids in contrast to HIL_STATE singularities. This packet is useful for high throughput applications such as hardware in the loop simulations.
///
/// MavLink id: 115
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct HilStateQuaternion {
    /// Timestamp (UNIX Epoch time or time since system boot). The receiving end can infer timestamp format (since 1.1.1970 or since system boot) by checking for the magnitude of the number.
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// Vehicle attitude expressed as normalized quaternion in w, x, y, z order (with 1 0 0 0 being the null-rotation)
    #[prost(float, repeated, packed = "false", tag = "2")]
    pub attitude_quaternion: ::prost::alloc::vec::Vec<f32>,
    /// Body frame roll / phi angular speed
    #[prost(float, tag = "3")]
    pub rollspeed: f32,
    /// Body frame pitch / theta angular speed
    #[prost(float, tag = "4")]
    pub pitchspeed: f32,
    /// Body frame yaw / psi angular speed
    #[prost(float, tag = "5")]
    pub yawspeed: f32,
    /// Latitude
    #[prost(int32, tag = "6")]
    pub lat: i32,
    /// Longitude
    #[prost(int32, tag = "7")]
    pub lon: i32,
    /// Altitude
    #[prost(int32, tag = "8")]
    pub alt: i32,
    /// Ground X Speed (Latitude)
    #[prost(int32, tag = "9")]
    pub vx: i32,
    /// Ground Y Speed (Longitude)
    #[prost(int32, tag = "10")]
    pub vy: i32,
    /// Ground Z Speed (Altitude)
    #[prost(int32, tag = "11")]
    pub vz: i32,
    /// Indicated airspeed
    #[prost(uint32, tag = "12")]
    pub ind_airspeed: u32,
    /// True airspeed
    #[prost(uint32, tag = "13")]
    pub true_airspeed: u32,
    /// X acceleration
    #[prost(int32, tag = "14")]
    pub xacc: i32,
    /// Y acceleration
    #[prost(int32, tag = "15")]
    pub yacc: i32,
    /// Z acceleration
    #[prost(int32, tag = "16")]
    pub zacc: i32,
}
/// The RAW IMU readings for secondary 9DOF sensor setup. This message should contain the scaled values to the described units
///
/// MavLink id: 116
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ScaledImu2 {
    /// Timestamp (time since system boot).
    #[prost(uint32, tag = "1")]
    pub time_boot_ms: u32,
    /// X acceleration
    #[prost(int32, tag = "2")]
    pub xacc: i32,
    /// Y acceleration
    #[prost(int32, tag = "3")]
    pub yacc: i32,
    /// Z acceleration
    #[prost(int32, tag = "4")]
    pub zacc: i32,
    /// Angular speed around X axis
    #[prost(int32, tag = "5")]
    pub xgyro: i32,
    /// Angular speed around Y axis
    #[prost(int32, tag = "6")]
    pub ygyro: i32,
    /// Angular speed around Z axis
    #[prost(int32, tag = "7")]
    pub zgyro: i32,
    /// X Magnetic field
    #[prost(int32, tag = "8")]
    pub xmag: i32,
    /// Y Magnetic field
    #[prost(int32, tag = "9")]
    pub ymag: i32,
    /// Z Magnetic field
    #[prost(int32, tag = "10")]
    pub zmag: i32,
}
/// Request a list of available logs. On some systems calling this may stop on-board logging until LOG_REQUEST_END is called. If there are no log files available this request shall be answered with one LOG_ENTRY message with id = 0 and num_logs = 0.
///
/// MavLink id: 117
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct LogRequestList {
    /// First log id (0 for first available)
    #[prost(uint32, tag = "1")]
    pub start: u32,
    /// Last log id (0xffff for last available)
    #[prost(uint32, tag = "2")]
    pub end: u32,
    /// System ID
    #[prost(uint32, tag = "3")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "4")]
    pub target_component: u32,
}
/// Reply to LOG_REQUEST_LIST
///
/// MavLink id: 118
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct LogEntry {
    /// UTC timestamp of log since 1970, or 0 if not available
    #[prost(uint32, tag = "1")]
    pub time_utc: u32,
    /// Size of the log (may be approximate)
    #[prost(uint32, tag = "2")]
    pub size: u32,
    /// Log id
    #[prost(uint32, tag = "3")]
    pub id: u32,
    /// Total number of logs
    #[prost(uint32, tag = "4")]
    pub num_logs: u32,
    /// High log number
    #[prost(uint32, tag = "5")]
    pub last_log_num: u32,
}
/// Request a chunk of a log
///
/// MavLink id: 119
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct LogRequestData {
    /// Offset into the log
    #[prost(uint32, tag = "1")]
    pub ofs: u32,
    /// Number of bytes
    #[prost(uint32, tag = "2")]
    pub count: u32,
    /// Log id (from LOG_ENTRY reply)
    #[prost(uint32, tag = "3")]
    pub id: u32,
    /// System ID
    #[prost(uint32, tag = "4")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "5")]
    pub target_component: u32,
}
/// Reply to LOG_REQUEST_DATA
///
/// MavLink id: 120
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct LogData {
    /// Offset into the log
    #[prost(uint32, tag = "1")]
    pub ofs: u32,
    /// Log id (from LOG_ENTRY reply)
    #[prost(uint32, tag = "2")]
    pub id: u32,
    /// Number of bytes (zero for end of log)
    #[prost(uint32, tag = "3")]
    pub count: u32,
    /// log data
    #[prost(uint32, repeated, packed = "false", tag = "4")]
    pub data: ::prost::alloc::vec::Vec<u32>,
}
/// Erase all logs
///
/// MavLink id: 121
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct LogErase {
    /// System ID
    #[prost(uint32, tag = "1")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "2")]
    pub target_component: u32,
}
/// Stop log transfer and resume normal logging
///
/// MavLink id: 122
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct LogRequestEnd {
    /// System ID
    #[prost(uint32, tag = "1")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "2")]
    pub target_component: u32,
}
/// Data for injecting into the onboard GPS (used for DGPS)
///
/// MavLink id: 123
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GpsInjectData {
    /// System ID
    #[prost(uint32, tag = "1")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "2")]
    pub target_component: u32,
    /// Data length
    #[prost(uint32, tag = "3")]
    pub len: u32,
    /// Raw data (110 is enough for 12 satellites of RTCMv2)
    #[prost(uint32, repeated, packed = "false", tag = "4")]
    pub data: ::prost::alloc::vec::Vec<u32>,
}
/// Second GPS data.
///
/// MavLink id: 124
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Gps2Raw {
    /// Timestamp (UNIX Epoch time or time since system boot). The receiving end can infer timestamp format (since 1.1.1970 or since system boot) by checking for the magnitude of the number.
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// Latitude (WGS84)
    #[prost(int32, tag = "2")]
    pub lat: i32,
    /// Longitude (WGS84)
    #[prost(int32, tag = "3")]
    pub lon: i32,
    /// Altitude (MSL). Positive for up.
    #[prost(int32, tag = "4")]
    pub alt: i32,
    /// Age of DGPS info
    #[prost(uint32, tag = "5")]
    pub dgps_age: u32,
    /// GPS HDOP horizontal dilution of position. If unknown, set to: UINT16_MAX
    #[prost(uint32, tag = "6")]
    pub eph: u32,
    /// GPS VDOP vertical dilution of position. If unknown, set to: UINT16_MAX
    #[prost(uint32, tag = "7")]
    pub epv: u32,
    /// GPS ground speed. If unknown, set to: UINT16_MAX
    #[prost(uint32, tag = "8")]
    pub vel: u32,
    /// Course over ground (NOT heading, but direction of movement): 0.0..359.99 degrees. If unknown, set to: UINT16_MAX
    #[prost(uint32, tag = "9")]
    pub cog: u32,
    /// GPS fix type.
    #[prost(enumeration = "GpsFixType", tag = "10")]
    pub fix_type: i32,
    /// Number of satellites visible. If unknown, set to 255
    #[prost(uint32, tag = "11")]
    pub satellites_visible: u32,
    /// Number of DGPS satellites
    #[prost(uint32, tag = "12")]
    pub dgps_numch: u32,
}
/// Power supply status
///
/// MavLink id: 125
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PowerStatus {
    /// 5V rail voltage.
    #[prost(uint32, tag = "1")]
    pub vcc: u32,
    /// Servo rail voltage.
    #[prost(uint32, tag = "2")]
    pub vservo: u32,
    /// Bitmap of power supply status flags.
    /// bitfield defined by enum MAV_POWER_STATUS
    #[prost(uint32, tag = "3")]
    pub flags: u32,
}
/// Control a serial port. This can be used for raw access to an onboard serial peripheral such as a GPS or telemetry radio. It is designed to make it possible to update the devices firmware via MAVLink messages or change the devices settings. A message with zero bytes can be used to change just the baudrate.
///
/// MavLink id: 126
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SerialControl {
    /// Baudrate of transfer. Zero means no change.
    #[prost(uint32, tag = "1")]
    pub baudrate: u32,
    /// Timeout for reply data
    #[prost(uint32, tag = "2")]
    pub timeout: u32,
    /// Serial control device type.
    #[prost(enumeration = "SerialControlDev", tag = "3")]
    pub device: i32,
    /// Bitmap of serial control flags.
    /// bitfield defined by enum SERIAL_CONTROL_FLAG
    #[prost(uint32, tag = "4")]
    pub flags: u32,
    /// how many bytes in this transfer
    #[prost(uint32, tag = "5")]
    pub count: u32,
    /// serial data
    #[prost(uint32, repeated, packed = "false", tag = "6")]
    pub data: ::prost::alloc::vec::Vec<u32>,
}
/// RTK GPS data. Gives information on the relative baseline calculation the GPS is reporting
///
/// MavLink id: 127
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GpsRtk {
    /// Time since boot of last baseline message received.
    #[prost(uint32, tag = "1")]
    pub time_last_baseline_ms: u32,
    /// GPS Time of Week of last baseline
    #[prost(uint32, tag = "2")]
    pub tow: u32,
    /// Current baseline in ECEF x or NED north component.
    #[prost(int32, tag = "3")]
    pub baseline_a_mm: i32,
    /// Current baseline in ECEF y or NED east component.
    #[prost(int32, tag = "4")]
    pub baseline_b_mm: i32,
    /// Current baseline in ECEF z or NED down component.
    #[prost(int32, tag = "5")]
    pub baseline_c_mm: i32,
    /// Current estimate of baseline accuracy.
    #[prost(uint32, tag = "6")]
    pub accuracy: u32,
    /// Current number of integer ambiguity hypotheses.
    #[prost(int32, tag = "7")]
    pub iar_num_hypotheses: i32,
    /// GPS Week Number of last baseline
    #[prost(uint32, tag = "8")]
    pub wn: u32,
    /// Identification of connected RTK receiver.
    #[prost(uint32, tag = "9")]
    pub rtk_receiver_id: u32,
    /// GPS-specific health report for RTK data.
    #[prost(uint32, tag = "10")]
    pub rtk_health: u32,
    /// Rate of baseline messages being received by GPS
    #[prost(uint32, tag = "11")]
    pub rtk_rate: u32,
    /// Current number of sats used for RTK calculation.
    #[prost(uint32, tag = "12")]
    pub nsats: u32,
    /// Coordinate system of baseline
    #[prost(enumeration = "RtkBaselineCoordinateSystem", tag = "13")]
    pub baseline_coords_type: i32,
}
/// RTK GPS data. Gives information on the relative baseline calculation the GPS is reporting
///
/// MavLink id: 128
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Gps2Rtk {
    /// Time since boot of last baseline message received.
    #[prost(uint32, tag = "1")]
    pub time_last_baseline_ms: u32,
    /// GPS Time of Week of last baseline
    #[prost(uint32, tag = "2")]
    pub tow: u32,
    /// Current baseline in ECEF x or NED north component.
    #[prost(int32, tag = "3")]
    pub baseline_a_mm: i32,
    /// Current baseline in ECEF y or NED east component.
    #[prost(int32, tag = "4")]
    pub baseline_b_mm: i32,
    /// Current baseline in ECEF z or NED down component.
    #[prost(int32, tag = "5")]
    pub baseline_c_mm: i32,
    /// Current estimate of baseline accuracy.
    #[prost(uint32, tag = "6")]
    pub accuracy: u32,
    /// Current number of integer ambiguity hypotheses.
    #[prost(int32, tag = "7")]
    pub iar_num_hypotheses: i32,
    /// GPS Week Number of last baseline
    #[prost(uint32, tag = "8")]
    pub wn: u32,
    /// Identification of connected RTK receiver.
    #[prost(uint32, tag = "9")]
    pub rtk_receiver_id: u32,
    /// GPS-specific health report for RTK data.
    #[prost(uint32, tag = "10")]
    pub rtk_health: u32,
    /// Rate of baseline messages being received by GPS
    #[prost(uint32, tag = "11")]
    pub rtk_rate: u32,
    /// Current number of sats used for RTK calculation.
    #[prost(uint32, tag = "12")]
    pub nsats: u32,
    /// Coordinate system of baseline
    #[prost(enumeration = "RtkBaselineCoordinateSystem", tag = "13")]
    pub baseline_coords_type: i32,
}
/// The RAW IMU readings for 3rd 9DOF sensor setup. This message should contain the scaled values to the described units
///
/// MavLink id: 129
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ScaledImu3 {
    /// Timestamp (time since system boot).
    #[prost(uint32, tag = "1")]
    pub time_boot_ms: u32,
    /// X acceleration
    #[prost(int32, tag = "2")]
    pub xacc: i32,
    /// Y acceleration
    #[prost(int32, tag = "3")]
    pub yacc: i32,
    /// Z acceleration
    #[prost(int32, tag = "4")]
    pub zacc: i32,
    /// Angular speed around X axis
    #[prost(int32, tag = "5")]
    pub xgyro: i32,
    /// Angular speed around Y axis
    #[prost(int32, tag = "6")]
    pub ygyro: i32,
    /// Angular speed around Z axis
    #[prost(int32, tag = "7")]
    pub zgyro: i32,
    /// X Magnetic field
    #[prost(int32, tag = "8")]
    pub xmag: i32,
    /// Y Magnetic field
    #[prost(int32, tag = "9")]
    pub ymag: i32,
    /// Z Magnetic field
    #[prost(int32, tag = "10")]
    pub zmag: i32,
}
/// Handshake message to initiate, control and stop image streaming when using the Image Transmission Protocol: https://mavlink.io/en/services/image_transmission.html.
///
/// MavLink id: 130
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct DataTransmissionHandshake {
    /// total data size (set on ACK only).
    #[prost(uint32, tag = "1")]
    pub size: u32,
    /// Width of a matrix or image.
    #[prost(uint32, tag = "2")]
    pub width: u32,
    /// Height of a matrix or image.
    #[prost(uint32, tag = "3")]
    pub height: u32,
    /// Number of packets being sent (set on ACK only).
    #[prost(uint32, tag = "4")]
    pub packets: u32,
    /// Type of requested/acknowledged data.
    #[prost(enumeration = "MavlinkDataStreamType", tag = "5")]
    pub r#type: i32,
    /// Payload size per packet (normally 253 byte, see DATA field size in message ENCAPSULATED_DATA) (set on ACK only).
    #[prost(uint32, tag = "6")]
    pub payload: u32,
    /// JPEG quality. Values: [1-100].
    #[prost(uint32, tag = "7")]
    pub jpg_quality: u32,
}
/// Data packet for images sent using the Image Transmission Protocol: https://mavlink.io/en/services/image_transmission.html.
///
/// MavLink id: 131
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct EncapsulatedData {
    /// sequence number (starting with 0 on every transmission)
    #[prost(uint32, tag = "1")]
    pub seqnr: u32,
    /// image data bytes
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u32>,
}
/// Distance sensor information for an onboard rangefinder.
///
/// MavLink id: 132
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct DistanceSensor {
    /// Timestamp (time since system boot).
    #[prost(uint32, tag = "1")]
    pub time_boot_ms: u32,
    /// Minimum distance the sensor can measure
    #[prost(uint32, tag = "2")]
    pub min_distance: u32,
    /// Maximum distance the sensor can measure
    #[prost(uint32, tag = "3")]
    pub max_distance: u32,
    /// Current distance reading
    #[prost(uint32, tag = "4")]
    pub current_distance: u32,
    /// Type of distance sensor.
    #[prost(enumeration = "MavDistanceSensor", tag = "5")]
    pub r#type: i32,
    /// Onboard ID of the sensor
    #[prost(uint32, tag = "6")]
    pub id: u32,
    /// Direction the sensor faces. downward-facing: ROTATION_PITCH_270, upward-facing: ROTATION_PITCH_90, backward-facing: ROTATION_PITCH_180, forward-facing: ROTATION_NONE, left-facing: ROTATION_YAW_90, right-facing: ROTATION_YAW_270
    #[prost(enumeration = "MavSensorOrientation", tag = "7")]
    pub orientation: i32,
    /// Measurement variance. Max standard deviation is 6cm. 255 if unknown.
    #[prost(uint32, tag = "8")]
    pub covariance: u32,
}
/// Request for terrain data and terrain status
///
/// MavLink id: 133
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TerrainRequest {
    /// Bitmask of requested 4x4 grids (row major 8x7 array of grids, 56 bits)
    #[prost(uint64, tag = "1")]
    pub mask: u64,
    /// Latitude of SW corner of first grid
    #[prost(int32, tag = "2")]
    pub lat: i32,
    /// Longitude of SW corner of first grid
    #[prost(int32, tag = "3")]
    pub lon: i32,
    /// Grid spacing
    #[prost(uint32, tag = "4")]
    pub grid_spacing: u32,
}
/// Terrain data sent from GCS. The lat/lon and grid_spacing must be the same as a lat/lon from a TERRAIN_REQUEST
///
/// MavLink id: 134
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TerrainData {
    /// Latitude of SW corner of first grid
    #[prost(int32, tag = "1")]
    pub lat: i32,
    /// Longitude of SW corner of first grid
    #[prost(int32, tag = "2")]
    pub lon: i32,
    /// Grid spacing
    #[prost(uint32, tag = "3")]
    pub grid_spacing: u32,
    /// Terrain data MSL
    #[prost(int32, repeated, packed = "false", tag = "4")]
    pub data: ::prost::alloc::vec::Vec<i32>,
    /// bit within the terrain request mask
    #[prost(uint32, tag = "5")]
    pub gridbit: u32,
}
/// Request that the vehicle report terrain height at the given location. Used by GCS to check if vehicle has all terrain data needed for a mission.
///
/// MavLink id: 135
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TerrainCheck {
    /// Latitude
    #[prost(int32, tag = "1")]
    pub lat: i32,
    /// Longitude
    #[prost(int32, tag = "2")]
    pub lon: i32,
}
/// Response from a TERRAIN_CHECK request
///
/// MavLink id: 136
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TerrainReport {
    /// Latitude
    #[prost(int32, tag = "1")]
    pub lat: i32,
    /// Longitude
    #[prost(int32, tag = "2")]
    pub lon: i32,
    /// Terrain height MSL
    #[prost(float, tag = "3")]
    pub terrain_height: f32,
    /// Current vehicle height above lat/lon terrain height
    #[prost(float, tag = "4")]
    pub current_height: f32,
    /// grid spacing (zero if terrain at this location unavailable)
    #[prost(uint32, tag = "5")]
    pub spacing: u32,
    /// Number of 4x4 terrain blocks waiting to be received or read from disk
    #[prost(uint32, tag = "6")]
    pub pending: u32,
    /// Number of 4x4 terrain blocks in memory
    #[prost(uint32, tag = "7")]
    pub loaded: u32,
}
/// Barometer readings for 2nd barometer
///
/// MavLink id: 137
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ScaledPressure2 {
    /// Timestamp (time since system boot).
    #[prost(uint32, tag = "1")]
    pub time_boot_ms: u32,
    /// Absolute pressure
    #[prost(float, tag = "2")]
    pub press_abs: f32,
    /// Differential pressure
    #[prost(float, tag = "3")]
    pub press_diff: f32,
    /// Temperature measurement
    #[prost(int32, tag = "4")]
    pub temperature: i32,
}
/// Motion capture attitude and position
///
/// MavLink id: 138
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct AttPosMocap {
    /// Timestamp (UNIX Epoch time or time since system boot). The receiving end can infer timestamp format (since 1.1.1970 or since system boot) by checking for the magnitude of the number.
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// Attitude quaternion (w, x, y, z order, zero-rotation is 1, 0, 0, 0)
    #[prost(float, repeated, packed = "false", tag = "2")]
    pub q: ::prost::alloc::vec::Vec<f32>,
    /// X position (NED)
    #[prost(float, tag = "3")]
    pub x: f32,
    /// Y position (NED)
    #[prost(float, tag = "4")]
    pub y: f32,
    /// Z position (NED)
    #[prost(float, tag = "5")]
    pub z: f32,
}
/// Set the vehicle attitude and body angular rates.
///
/// MavLink id: 139
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetActuatorControlTarget {
    /// Timestamp (UNIX Epoch time or time since system boot). The receiving end can infer timestamp format (since 1.1.1970 or since system boot) by checking for the magnitude of the number.
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// Actuator controls. Normed to -1..+1 where 0 is neutral position. Throttle for single rotation direction motors is 0..1, negative range for reverse direction. Standard mapping for attitude controls (group 0): (index 0-7): roll, pitch, yaw, throttle, flaps, spoilers, airbrakes, landing gear. Load a pass-through mixer to repurpose them as generic outputs.
    #[prost(float, repeated, packed = "false", tag = "2")]
    pub controls: ::prost::alloc::vec::Vec<f32>,
    /// Actuator group. The "_mlx" indicates this is a multi-instance message and a MAVLink parser should use this field to difference between instances.
    #[prost(uint32, tag = "3")]
    pub group_mlx: u32,
    /// System ID
    #[prost(uint32, tag = "4")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "5")]
    pub target_component: u32,
}
/// Set the vehicle attitude and body angular rates.
///
/// MavLink id: 140
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ActuatorControlTarget {
    /// Timestamp (UNIX Epoch time or time since system boot). The receiving end can infer timestamp format (since 1.1.1970 or since system boot) by checking for the magnitude of the number.
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// Actuator controls. Normed to -1..+1 where 0 is neutral position. Throttle for single rotation direction motors is 0..1, negative range for reverse direction. Standard mapping for attitude controls (group 0): (index 0-7): roll, pitch, yaw, throttle, flaps, spoilers, airbrakes, landing gear. Load a pass-through mixer to repurpose them as generic outputs.
    #[prost(float, repeated, packed = "false", tag = "2")]
    pub controls: ::prost::alloc::vec::Vec<f32>,
    /// Actuator group. The "_mlx" indicates this is a multi-instance message and a MAVLink parser should use this field to difference between instances.
    #[prost(uint32, tag = "3")]
    pub group_mlx: u32,
}
/// The current system altitude.
///
/// MavLink id: 141
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Altitude {
    /// Timestamp (UNIX Epoch time or time since system boot). The receiving end can infer timestamp format (since 1.1.1970 or since system boot) by checking for the magnitude of the number.
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// This altitude measure is initialized on system boot and monotonic (it is never reset, but represents the local altitude change). The only guarantee on this field is that it will never be reset and is consistent within a flight. The recommended value for this field is the uncorrected barometric altitude at boot time. This altitude will also drift and vary between flights.
    #[prost(float, tag = "2")]
    pub altitude_monotonic: f32,
    /// This altitude measure is strictly above mean sea level and might be non-monotonic (it might reset on events like GPS lock or when a new QNH value is set). It should be the altitude to which global altitude waypoints are compared to. Note that it is *not* the GPS altitude, however, most GPS modules already output MSL by default and not the WGS84 altitude.
    #[prost(float, tag = "3")]
    pub altitude_amsl: f32,
    /// This is the local altitude in the local coordinate frame. It is not the altitude above home, but in reference to the coordinate origin (0, 0, 0). It is up-positive.
    #[prost(float, tag = "4")]
    pub altitude_local: f32,
    /// This is the altitude above the home position. It resets on each change of the current home position.
    #[prost(float, tag = "5")]
    pub altitude_relative: f32,
    /// This is the altitude above terrain. It might be fed by a terrain database or an altimeter. Values smaller than -1000 should be interpreted as unknown.
    #[prost(float, tag = "6")]
    pub altitude_terrain: f32,
    /// This is not the altitude, but the clear space below the system according to the fused clearance estimate. It generally should max out at the maximum range of e.g. the laser altimeter. It is generally a moving target. A negative value indicates no measurement available.
    #[prost(float, tag = "7")]
    pub bottom_clearance: f32,
}
/// The autopilot is requesting a resource (file, binary, other type of data)
///
/// MavLink id: 142
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ResourceRequest {
    /// Request ID. This ID should be re-used when sending back URI contents
    #[prost(uint32, tag = "1")]
    pub request_id: u32,
    /// The type of requested URI. 0 = a file via URL. 1 = a UAVCAN binary
    #[prost(uint32, tag = "2")]
    pub uri_type: u32,
    /// The requested unique resource identifier (URI). It is not necessarily a straight domain name (depends on the URI type enum)
    #[prost(uint32, repeated, packed = "false", tag = "3")]
    pub uri: ::prost::alloc::vec::Vec<u32>,
    /// The way the autopilot wants to receive the URI. 0 = MAVLink FTP. 1 = binary stream.
    #[prost(uint32, tag = "4")]
    pub transfer_type: u32,
    /// The storage path the autopilot wants the URI to be stored in. Will only be valid if the transfer_type has a storage associated (e.g. MAVLink FTP).
    #[prost(uint32, repeated, packed = "false", tag = "5")]
    pub storage: ::prost::alloc::vec::Vec<u32>,
}
/// Barometer readings for 3rd barometer
///
/// MavLink id: 143
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ScaledPressure3 {
    /// Timestamp (time since system boot).
    #[prost(uint32, tag = "1")]
    pub time_boot_ms: u32,
    /// Absolute pressure
    #[prost(float, tag = "2")]
    pub press_abs: f32,
    /// Differential pressure
    #[prost(float, tag = "3")]
    pub press_diff: f32,
    /// Temperature measurement
    #[prost(int32, tag = "4")]
    pub temperature: i32,
}
/// Current motion information from a designated system
///
/// MavLink id: 144
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct FollowTarget {
    /// Timestamp (time since system boot).
    #[prost(uint64, tag = "1")]
    pub timestamp: u64,
    /// button states or switches of a tracker device
    #[prost(uint64, tag = "2")]
    pub custom_state: u64,
    /// Latitude (WGS84)
    #[prost(int32, tag = "3")]
    pub lat: i32,
    /// Longitude (WGS84)
    #[prost(int32, tag = "4")]
    pub lon: i32,
    /// Altitude (MSL)
    #[prost(float, tag = "5")]
    pub alt: f32,
    /// target velocity (0,0,0) for unknown
    #[prost(float, repeated, packed = "false", tag = "6")]
    pub vel: ::prost::alloc::vec::Vec<f32>,
    /// linear target acceleration (0,0,0) for unknown
    #[prost(float, repeated, packed = "false", tag = "7")]
    pub acc: ::prost::alloc::vec::Vec<f32>,
    /// (1 0 0 0 for unknown)
    #[prost(float, repeated, packed = "false", tag = "8")]
    pub attitude_q: ::prost::alloc::vec::Vec<f32>,
    /// (0 0 0 for unknown)
    #[prost(float, repeated, packed = "false", tag = "9")]
    pub rates: ::prost::alloc::vec::Vec<f32>,
    /// eph epv
    #[prost(float, repeated, packed = "false", tag = "10")]
    pub position_cov: ::prost::alloc::vec::Vec<f32>,
    /// bit positions for tracker reporting capabilities (POS = 0, VEL = 1, ACCEL = 2, ATT + RATES = 3)
    #[prost(uint32, tag = "11")]
    pub est_capabilities: u32,
}
/// The smoothed, monotonic system state used to feed the control loops of the system.
///
/// MavLink id: 146
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ControlSystemState {
    /// Timestamp (UNIX Epoch time or time since system boot). The receiving end can infer timestamp format (since 1.1.1970 or since system boot) by checking for the magnitude of the number.
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// X acceleration in body frame
    #[prost(float, tag = "2")]
    pub x_acc: f32,
    /// Y acceleration in body frame
    #[prost(float, tag = "3")]
    pub y_acc: f32,
    /// Z acceleration in body frame
    #[prost(float, tag = "4")]
    pub z_acc: f32,
    /// X velocity in body frame
    #[prost(float, tag = "5")]
    pub x_vel: f32,
    /// Y velocity in body frame
    #[prost(float, tag = "6")]
    pub y_vel: f32,
    /// Z velocity in body frame
    #[prost(float, tag = "7")]
    pub z_vel: f32,
    /// X position in local frame
    #[prost(float, tag = "8")]
    pub x_pos: f32,
    /// Y position in local frame
    #[prost(float, tag = "9")]
    pub y_pos: f32,
    /// Z position in local frame
    #[prost(float, tag = "10")]
    pub z_pos: f32,
    /// Airspeed, set to -1 if unknown
    #[prost(float, tag = "11")]
    pub airspeed: f32,
    /// Variance of body velocity estimate
    #[prost(float, repeated, packed = "false", tag = "12")]
    pub vel_variance: ::prost::alloc::vec::Vec<f32>,
    /// Variance in local position
    #[prost(float, repeated, packed = "false", tag = "13")]
    pub pos_variance: ::prost::alloc::vec::Vec<f32>,
    /// The attitude, represented as Quaternion
    #[prost(float, repeated, packed = "false", tag = "14")]
    pub q: ::prost::alloc::vec::Vec<f32>,
    /// Angular rate in roll axis
    #[prost(float, tag = "15")]
    pub roll_rate: f32,
    /// Angular rate in pitch axis
    #[prost(float, tag = "16")]
    pub pitch_rate: f32,
    /// Angular rate in yaw axis
    #[prost(float, tag = "17")]
    pub yaw_rate: f32,
}
/// Battery information. Updates GCS with flight controller battery status. Use SMART_BATTERY_* messages instead for smart batteries.
///
/// MavLink id: 147
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct BatteryStatus {
    /// Consumed charge, -1: autopilot does not provide consumption estimate
    #[prost(int32, tag = "1")]
    pub current_consumed: i32,
    /// Consumed energy, -1: autopilot does not provide energy consumption estimate
    #[prost(int32, tag = "2")]
    pub energy_consumed: i32,
    /// Temperature of the battery. INT16_MAX for unknown temperature.
    #[prost(int32, tag = "3")]
    pub temperature: i32,
    /// Battery voltage of cells. Cells above the valid cell count for this battery should have the UINT16_MAX value. If individual cell voltages are unknown or not measured for this battery, then the overall battery voltage should be filled in cell 0, with all others set to UINT16_MAX. If the voltage of the battery is greater than (UINT16_MAX - 1), then cell 0 should be set to (UINT16_MAX - 1), and cell 1 to the remaining voltage. This can be extended to multiple cells if the total voltage is greater than 2 * (UINT16_MAX - 1).
    #[prost(uint32, repeated, packed = "false", tag = "4")]
    pub voltages: ::prost::alloc::vec::Vec<u32>,
    /// Battery current, -1: autopilot does not measure the current
    #[prost(int32, tag = "5")]
    pub current_battery: i32,
    /// Battery ID
    #[prost(uint32, tag = "6")]
    pub id: u32,
    /// Function of the battery
    #[prost(enumeration = "MavBatteryFunction", tag = "7")]
    pub battery_function: i32,
    /// Type (chemistry) of the battery
    #[prost(enumeration = "MavBatteryType", tag = "8")]
    pub r#type: i32,
    /// Remaining battery energy. Values: [0-100], -1: autopilot does not estimate the remaining battery.
    #[prost(int32, tag = "9")]
    pub battery_remaining: i32,
}
/// Version and capability of autopilot software. This should be emitted in response to a request with MAV_CMD_REQUEST_MESSAGE.
///
/// MavLink id: 148
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct AutopilotVersion {
    /// Bitmap of capabilities
    /// bitfield defined by enum MAV_PROTOCOL_CAPABILITY
    #[prost(uint64, tag = "1")]
    pub capabilities: u64,
    /// UID if provided by hardware (see uid2)
    #[prost(uint64, tag = "2")]
    pub uid: u64,
    /// Firmware version number
    #[prost(uint32, tag = "3")]
    pub flight_sw_version: u32,
    /// Middleware version number
    #[prost(uint32, tag = "4")]
    pub middleware_sw_version: u32,
    /// Operating system version number
    #[prost(uint32, tag = "5")]
    pub os_sw_version: u32,
    /// HW / board version (last 8 bytes should be silicon ID, if any)
    #[prost(uint32, tag = "6")]
    pub board_version: u32,
    /// ID of the board vendor
    #[prost(uint32, tag = "7")]
    pub vendor_id: u32,
    /// ID of the product
    #[prost(uint32, tag = "8")]
    pub product_id: u32,
    /// Custom version field, commonly the first 8 bytes of the git hash. This is not an unique identifier, but should allow to identify the commit using the main version number even for very large code bases.
    #[prost(uint32, repeated, packed = "false", tag = "9")]
    pub flight_custom_version: ::prost::alloc::vec::Vec<u32>,
    /// Custom version field, commonly the first 8 bytes of the git hash. This is not an unique identifier, but should allow to identify the commit using the main version number even for very large code bases.
    #[prost(uint32, repeated, packed = "false", tag = "10")]
    pub middleware_custom_version: ::prost::alloc::vec::Vec<u32>,
    /// Custom version field, commonly the first 8 bytes of the git hash. This is not an unique identifier, but should allow to identify the commit using the main version number even for very large code bases.
    #[prost(uint32, repeated, packed = "false", tag = "11")]
    pub os_custom_version: ::prost::alloc::vec::Vec<u32>,
}
/// The location of a landing target. See: https://mavlink.io/en/services/landing_target.html
///
/// MavLink id: 149
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct LandingTarget {
    /// Timestamp (UNIX Epoch time or time since system boot). The receiving end can infer timestamp format (since 1.1.1970 or since system boot) by checking for the magnitude of the number.
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// X-axis angular offset of the target from the center of the image
    #[prost(float, tag = "2")]
    pub angle_x: f32,
    /// Y-axis angular offset of the target from the center of the image
    #[prost(float, tag = "3")]
    pub angle_y: f32,
    /// Distance to the target from the vehicle
    #[prost(float, tag = "4")]
    pub distance: f32,
    /// Size of target along x-axis
    #[prost(float, tag = "5")]
    pub size_x: f32,
    /// Size of target along y-axis
    #[prost(float, tag = "6")]
    pub size_y: f32,
    /// The ID of the target if multiple targets are present
    #[prost(uint32, tag = "7")]
    pub target_num: u32,
    /// Coordinate frame used for following fields.
    #[prost(enumeration = "MavFrame", tag = "8")]
    pub frame: i32,
}
/// Status of geo-fencing. Sent in extended status stream when fencing enabled.
///
/// MavLink id: 162
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct FenceStatus {
    /// Time (since boot) of last breach.
    #[prost(uint32, tag = "1")]
    pub breach_time: u32,
    /// Number of fence breaches.
    #[prost(uint32, tag = "2")]
    pub breach_count: u32,
    /// Breach status (0 if currently inside fence, 1 if outside).
    #[prost(uint32, tag = "3")]
    pub breach_status: u32,
    /// Last breach type.
    #[prost(enumeration = "FenceBreach", tag = "4")]
    pub breach_type: i32,
}
/// Estimator status message including flags, innovation test ratios and estimated accuracies. The flags message is an integer bitmask containing information on which EKF outputs are valid. See the ESTIMATOR_STATUS_FLAGS enum definition for further information. The innovation test ratios show the magnitude of the sensor innovation divided by the innovation check threshold. Under normal operation the innovation test ratios should be below 0.5 with occasional values up to 1.0. Values greater than 1.0 should be rare under normal operation and indicate that a measurement has been rejected by the filter. The user should be notified if an innovation test ratio greater than 1.0 is recorded. Notifications for values in the range between 0.5 and 1.0 should be optional and controllable by the user.
///
/// MavLink id: 230
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct EstimatorStatus {
    /// Timestamp (UNIX Epoch time or time since system boot). The receiving end can infer timestamp format (since 1.1.1970 or since system boot) by checking for the magnitude of the number.
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// Velocity innovation test ratio
    #[prost(float, tag = "2")]
    pub vel_ratio: f32,
    /// Horizontal position innovation test ratio
    #[prost(float, tag = "3")]
    pub pos_horiz_ratio: f32,
    /// Vertical position innovation test ratio
    #[prost(float, tag = "4")]
    pub pos_vert_ratio: f32,
    /// Magnetometer innovation test ratio
    #[prost(float, tag = "5")]
    pub mag_ratio: f32,
    /// Height above terrain innovation test ratio
    #[prost(float, tag = "6")]
    pub hagl_ratio: f32,
    /// True airspeed innovation test ratio
    #[prost(float, tag = "7")]
    pub tas_ratio: f32,
    /// Horizontal position 1-STD accuracy relative to the EKF local origin
    #[prost(float, tag = "8")]
    pub pos_horiz_accuracy: f32,
    /// Vertical position 1-STD accuracy relative to the EKF local origin
    #[prost(float, tag = "9")]
    pub pos_vert_accuracy: f32,
    /// Bitmap indicating which EKF outputs are valid.
    /// bitfield defined by enum ESTIMATOR_STATUS_FLAGS
    #[prost(uint32, tag = "10")]
    pub flags: u32,
}
/// Wind covariance estimate from vehicle.
///
/// MavLink id: 231
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct WindCov {
    /// Timestamp (UNIX Epoch time or time since system boot). The receiving end can infer timestamp format (since 1.1.1970 or since system boot) by checking for the magnitude of the number.
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// Wind in X (NED) direction
    #[prost(float, tag = "2")]
    pub wind_x: f32,
    /// Wind in Y (NED) direction
    #[prost(float, tag = "3")]
    pub wind_y: f32,
    /// Wind in Z (NED) direction
    #[prost(float, tag = "4")]
    pub wind_z: f32,
    /// Variability of the wind in XY. RMS of a 1 Hz lowpassed wind estimate.
    #[prost(float, tag = "5")]
    pub var_horiz: f32,
    /// Variability of the wind in Z. RMS of a 1 Hz lowpassed wind estimate.
    #[prost(float, tag = "6")]
    pub var_vert: f32,
    /// Altitude (MSL) that this measurement was taken at
    #[prost(float, tag = "7")]
    pub wind_alt: f32,
    /// Horizontal speed 1-STD accuracy
    #[prost(float, tag = "8")]
    pub horiz_accuracy: f32,
    /// Vertical speed 1-STD accuracy
    #[prost(float, tag = "9")]
    pub vert_accuracy: f32,
}
/// GPS sensor input message.  This is a raw sensor value sent by the GPS. This is NOT the global position estimate of the system.
///
/// MavLink id: 232
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GpsInput {
    /// Timestamp (UNIX Epoch time or time since system boot). The receiving end can infer timestamp format (since 1.1.1970 or since system boot) by checking for the magnitude of the number.
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// GPS time (from start of GPS week)
    #[prost(uint32, tag = "2")]
    pub time_week_ms: u32,
    /// Latitude (WGS84)
    #[prost(int32, tag = "3")]
    pub lat: i32,
    /// Longitude (WGS84)
    #[prost(int32, tag = "4")]
    pub lon: i32,
    /// Altitude (MSL). Positive for up.
    #[prost(float, tag = "5")]
    pub alt: f32,
    /// GPS HDOP horizontal dilution of position
    #[prost(float, tag = "6")]
    pub hdop: f32,
    /// GPS VDOP vertical dilution of position
    #[prost(float, tag = "7")]
    pub vdop: f32,
    /// GPS velocity in north direction in earth-fixed NED frame
    #[prost(float, tag = "8")]
    pub vn: f32,
    /// GPS velocity in east direction in earth-fixed NED frame
    #[prost(float, tag = "9")]
    pub ve: f32,
    /// GPS velocity in down direction in earth-fixed NED frame
    #[prost(float, tag = "10")]
    pub vd: f32,
    /// GPS speed accuracy
    #[prost(float, tag = "11")]
    pub speed_accuracy: f32,
    /// GPS horizontal accuracy
    #[prost(float, tag = "12")]
    pub horiz_accuracy: f32,
    /// GPS vertical accuracy
    #[prost(float, tag = "13")]
    pub vert_accuracy: f32,
    /// Bitmap indicating which GPS input flags fields to ignore.  All other fields must be provided.
    /// bitfield defined by enum GPS_INPUT_IGNORE_FLAGS
    #[prost(uint32, tag = "14")]
    pub ignore_flags: u32,
    /// GPS week number
    #[prost(uint32, tag = "15")]
    pub time_week: u32,
    /// ID of the GPS for multiple GPS inputs
    #[prost(uint32, tag = "16")]
    pub gps_id: u32,
    /// 0-1: no fix, 2: 2D fix, 3: 3D fix. 4: 3D with DGPS. 5: 3D with RTK
    #[prost(uint32, tag = "17")]
    pub fix_type: u32,
    /// Number of satellites visible.
    #[prost(uint32, tag = "18")]
    pub satellites_visible: u32,
}
/// RTCM message for injecting into the onboard GPS (used for DGPS)
///
/// MavLink id: 233
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GpsRtcmData {
    /// LSB: 1 means message is fragmented, next 2 bits are the fragment ID, the remaining 5 bits are used for the sequence ID. Messages are only to be flushed to the GPS when the entire message has been reconstructed on the autopilot. The fragment ID specifies which order the fragments should be assembled into a buffer, while the sequence ID is used to detect a mismatch between different buffers. The buffer is considered fully reconstructed when either all 4 fragments are present, or all the fragments before the first fragment with a non full payload is received. This management is used to ensure that normal GPS operation doesn't corrupt RTCM data, and to recover from a unreliable transport delivery order.
    #[prost(uint32, tag = "1")]
    pub flags: u32,
    /// data length
    #[prost(uint32, tag = "2")]
    pub len: u32,
    /// RTCM message (may be fragmented)
    #[prost(uint32, repeated, packed = "false", tag = "3")]
    pub data: ::prost::alloc::vec::Vec<u32>,
}
/// Message appropriate for high latency connections like Iridium
///
/// MavLink id: 234
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct HighLatency {
    /// A bitfield for use for autopilot-specific flags.
    #[prost(uint32, tag = "1")]
    pub custom_mode: u32,
    /// Latitude
    #[prost(int32, tag = "2")]
    pub latitude: i32,
    /// Longitude
    #[prost(int32, tag = "3")]
    pub longitude: i32,
    /// roll
    #[prost(int32, tag = "4")]
    pub roll: i32,
    /// pitch
    #[prost(int32, tag = "5")]
    pub pitch: i32,
    /// heading
    #[prost(uint32, tag = "6")]
    pub heading: u32,
    /// heading setpoint
    #[prost(int32, tag = "7")]
    pub heading_sp: i32,
    /// Altitude above mean sea level
    #[prost(int32, tag = "8")]
    pub altitude_amsl: i32,
    /// Altitude setpoint relative to the home position
    #[prost(int32, tag = "9")]
    pub altitude_sp: i32,
    /// distance to target
    #[prost(uint32, tag = "10")]
    pub wp_distance: u32,
    /// Bitmap of enabled system modes.
    /// bitfield defined by enum MAV_MODE_FLAG
    #[prost(uint32, tag = "11")]
    pub base_mode: u32,
    /// The landed state. Is set to MAV_LANDED_STATE_UNDEFINED if landed state is unknown.
    #[prost(enumeration = "MavLandedState", tag = "12")]
    pub landed_state: i32,
    /// throttle (percentage)
    #[prost(int32, tag = "13")]
    pub throttle: i32,
    /// airspeed
    #[prost(uint32, tag = "14")]
    pub airspeed: u32,
    /// airspeed setpoint
    #[prost(uint32, tag = "15")]
    pub airspeed_sp: u32,
    /// groundspeed
    #[prost(uint32, tag = "16")]
    pub groundspeed: u32,
    /// climb rate
    #[prost(int32, tag = "17")]
    pub climb_rate: i32,
    /// Number of satellites visible. If unknown, set to 255
    #[prost(uint32, tag = "18")]
    pub gps_nsat: u32,
    /// GPS Fix type.
    #[prost(enumeration = "GpsFixType", tag = "19")]
    pub gps_fix_type: i32,
    /// Remaining battery (percentage)
    #[prost(uint32, tag = "20")]
    pub battery_remaining: u32,
    /// Autopilot temperature (degrees C)
    #[prost(int32, tag = "21")]
    pub temperature: i32,
    /// Air temperature (degrees C) from airspeed sensor
    #[prost(int32, tag = "22")]
    pub temperature_air: i32,
    /// failsafe (each bit represents a failsafe where 0=ok, 1=failsafe active (bit0:RC, bit1:batt, bit2:GPS, bit3:GCS, bit4:fence)
    #[prost(uint32, tag = "23")]
    pub failsafe: u32,
    /// current waypoint number
    #[prost(uint32, tag = "24")]
    pub wp_num: u32,
}
/// Message appropriate for high latency connections like Iridium (version 2)
///
/// MavLink id: 235
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct HighLatency2 {
    /// Timestamp (milliseconds since boot or Unix epoch)
    #[prost(uint32, tag = "1")]
    pub timestamp: u32,
    /// Latitude
    #[prost(int32, tag = "2")]
    pub latitude: i32,
    /// Longitude
    #[prost(int32, tag = "3")]
    pub longitude: i32,
    /// A bitfield for use for autopilot-specific flags (2 byte version).
    #[prost(uint32, tag = "4")]
    pub custom_mode: u32,
    /// Altitude above mean sea level
    #[prost(int32, tag = "5")]
    pub altitude: i32,
    /// Altitude setpoint
    #[prost(int32, tag = "6")]
    pub target_altitude: i32,
    /// Distance to target waypoint or position
    #[prost(uint32, tag = "7")]
    pub target_distance: u32,
    /// Current waypoint number
    #[prost(uint32, tag = "8")]
    pub wp_num: u32,
    /// Bitmap of failure flags.
    /// bitfield defined by enum HL_FAILURE_FLAG
    #[prost(uint32, tag = "9")]
    pub failure_flags: u32,
    /// Type of the MAV (quadrotor, helicopter, etc.)
    #[prost(enumeration = "MavType", tag = "10")]
    pub r#type: i32,
    /// Autopilot type / class. Use MAV_AUTOPILOT_INVALID for components that are not flight controllers.
    #[prost(enumeration = "MavAutopilot", tag = "11")]
    pub autopilot: i32,
    /// Heading
    #[prost(uint32, tag = "12")]
    pub heading: u32,
    /// Heading setpoint
    #[prost(uint32, tag = "13")]
    pub target_heading: u32,
    /// Throttle
    #[prost(uint32, tag = "14")]
    pub throttle: u32,
    /// Airspeed
    #[prost(uint32, tag = "15")]
    pub airspeed: u32,
    /// Airspeed setpoint
    #[prost(uint32, tag = "16")]
    pub airspeed_sp: u32,
    /// Groundspeed
    #[prost(uint32, tag = "17")]
    pub groundspeed: u32,
    /// Windspeed
    #[prost(uint32, tag = "18")]
    pub windspeed: u32,
    /// Wind heading
    #[prost(uint32, tag = "19")]
    pub wind_heading: u32,
    /// Maximum error horizontal position since last message
    #[prost(uint32, tag = "20")]
    pub eph: u32,
    /// Maximum error vertical position since last message
    #[prost(uint32, tag = "21")]
    pub epv: u32,
    /// Air temperature from airspeed sensor
    #[prost(int32, tag = "22")]
    pub temperature_air: i32,
    /// Maximum climb rate magnitude since last message
    #[prost(int32, tag = "23")]
    pub climb_rate: i32,
    /// Battery level (-1 if field not provided).
    #[prost(int32, tag = "24")]
    pub battery: i32,
    /// Field for custom payload.
    #[prost(int32, tag = "25")]
    pub custom0: i32,
    /// Field for custom payload.
    #[prost(int32, tag = "26")]
    pub custom1: i32,
    /// Field for custom payload.
    #[prost(int32, tag = "27")]
    pub custom2: i32,
}
/// Vibration levels and accelerometer clipping
///
/// MavLink id: 241
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Vibration {
    /// Timestamp (UNIX Epoch time or time since system boot). The receiving end can infer timestamp format (since 1.1.1970 or since system boot) by checking for the magnitude of the number.
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// Vibration levels on X-axis
    #[prost(float, tag = "2")]
    pub vibration_x: f32,
    /// Vibration levels on Y-axis
    #[prost(float, tag = "3")]
    pub vibration_y: f32,
    /// Vibration levels on Z-axis
    #[prost(float, tag = "4")]
    pub vibration_z: f32,
    /// first accelerometer clipping count
    #[prost(uint32, tag = "5")]
    pub clipping_0: u32,
    /// second accelerometer clipping count
    #[prost(uint32, tag = "6")]
    pub clipping_1: u32,
    /// third accelerometer clipping count
    #[prost(uint32, tag = "7")]
    pub clipping_2: u32,
}
/// This message can be requested by sending the MAV_CMD_GET_HOME_POSITION command. The position the system will return to and land on. The position is set automatically by the system during the takeoff in case it was not explicitly set by the operator before or after. The global and local positions encode the position in the respective coordinate frames, while the q parameter encodes the orientation of the surface. Under normal conditions it describes the heading and terrain slope, which can be used by the aircraft to adjust the approach. The approach 3D vector describes the point to which the system should fly in normal flight mode and then perform a landing sequence along the vector.
///
/// MavLink id: 242
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct HomePosition {
    /// Latitude (WGS84)
    #[prost(int32, tag = "1")]
    pub latitude: i32,
    /// Longitude (WGS84)
    #[prost(int32, tag = "2")]
    pub longitude: i32,
    /// Altitude (MSL). Positive for up.
    #[prost(int32, tag = "3")]
    pub altitude: i32,
    /// Local X position of this position in the local coordinate frame
    #[prost(float, tag = "4")]
    pub x: f32,
    /// Local Y position of this position in the local coordinate frame
    #[prost(float, tag = "5")]
    pub y: f32,
    /// Local Z position of this position in the local coordinate frame
    #[prost(float, tag = "6")]
    pub z: f32,
    /// World to surface normal and heading transformation of the takeoff position. Used to indicate the heading and slope of the ground
    #[prost(float, repeated, packed = "false", tag = "7")]
    pub q: ::prost::alloc::vec::Vec<f32>,
    /// Local X position of the end of the approach vector. Multicopters should set this position based on their takeoff path. Grass-landing fixed wing aircraft should set it the same way as multicopters. Runway-landing fixed wing aircraft should set it to the opposite direction of the takeoff, assuming the takeoff happened from the threshold / touchdown zone.
    #[prost(float, tag = "8")]
    pub approach_x: f32,
    /// Local Y position of the end of the approach vector. Multicopters should set this position based on their takeoff path. Grass-landing fixed wing aircraft should set it the same way as multicopters. Runway-landing fixed wing aircraft should set it to the opposite direction of the takeoff, assuming the takeoff happened from the threshold / touchdown zone.
    #[prost(float, tag = "9")]
    pub approach_y: f32,
    /// Local Z position of the end of the approach vector. Multicopters should set this position based on their takeoff path. Grass-landing fixed wing aircraft should set it the same way as multicopters. Runway-landing fixed wing aircraft should set it to the opposite direction of the takeoff, assuming the takeoff happened from the threshold / touchdown zone.
    #[prost(float, tag = "10")]
    pub approach_z: f32,
}
/// The position the system will return to and land on. The position is set automatically by the system during the takeoff in case it was not explicitly set by the operator before or after. The global and local positions encode the position in the respective coordinate frames, while the q parameter encodes the orientation of the surface. Under normal conditions it describes the heading and terrain slope, which can be used by the aircraft to adjust the approach. The approach 3D vector describes the point to which the system should fly in normal flight mode and then perform a landing sequence along the vector.
///
/// MavLink id: 243
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetHomePosition {
    /// Latitude (WGS84)
    #[prost(int32, tag = "1")]
    pub latitude: i32,
    /// Longitude (WGS84)
    #[prost(int32, tag = "2")]
    pub longitude: i32,
    /// Altitude (MSL). Positive for up.
    #[prost(int32, tag = "3")]
    pub altitude: i32,
    /// Local X position of this position in the local coordinate frame
    #[prost(float, tag = "4")]
    pub x: f32,
    /// Local Y position of this position in the local coordinate frame
    #[prost(float, tag = "5")]
    pub y: f32,
    /// Local Z position of this position in the local coordinate frame
    #[prost(float, tag = "6")]
    pub z: f32,
    /// World to surface normal and heading transformation of the takeoff position. Used to indicate the heading and slope of the ground
    #[prost(float, repeated, packed = "false", tag = "7")]
    pub q: ::prost::alloc::vec::Vec<f32>,
    /// Local X position of the end of the approach vector. Multicopters should set this position based on their takeoff path. Grass-landing fixed wing aircraft should set it the same way as multicopters. Runway-landing fixed wing aircraft should set it to the opposite direction of the takeoff, assuming the takeoff happened from the threshold / touchdown zone.
    #[prost(float, tag = "8")]
    pub approach_x: f32,
    /// Local Y position of the end of the approach vector. Multicopters should set this position based on their takeoff path. Grass-landing fixed wing aircraft should set it the same way as multicopters. Runway-landing fixed wing aircraft should set it to the opposite direction of the takeoff, assuming the takeoff happened from the threshold / touchdown zone.
    #[prost(float, tag = "9")]
    pub approach_y: f32,
    /// Local Z position of the end of the approach vector. Multicopters should set this position based on their takeoff path. Grass-landing fixed wing aircraft should set it the same way as multicopters. Runway-landing fixed wing aircraft should set it to the opposite direction of the takeoff, assuming the takeoff happened from the threshold / touchdown zone.
    #[prost(float, tag = "10")]
    pub approach_z: f32,
    /// System ID.
    #[prost(uint32, tag = "11")]
    pub target_system: u32,
}
/// The interval between messages for a particular MAVLink message ID. This message is the response to the MAV_CMD_GET_MESSAGE_INTERVAL command. This interface replaces DATA_STREAM.
///
/// MavLink id: 244
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MessageInterval {
    /// The interval between two messages. A value of -1 indicates this stream is disabled, 0 indicates it is not available, > 0 indicates the interval at which it is sent.
    #[prost(int32, tag = "1")]
    pub interval_us: i32,
    /// The ID of the requested MAVLink message. v1.0 is limited to 254 messages.
    #[prost(uint32, tag = "2")]
    pub message_id: u32,
}
/// Provides state for additional features
///
/// MavLink id: 245
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ExtendedSysState {
    /// The VTOL state if applicable. Is set to MAV_VTOL_STATE_UNDEFINED if UAV is not in VTOL configuration.
    #[prost(enumeration = "MavVtolState", tag = "1")]
    pub vtol_state: i32,
    /// The landed state. Is set to MAV_LANDED_STATE_UNDEFINED if landed state is unknown.
    #[prost(enumeration = "MavLandedState", tag = "2")]
    pub landed_state: i32,
}
/// The location and information of an ADSB vehicle
///
/// MavLink id: 246
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct AdsbVehicle {
    /// ICAO address
    #[prost(uint32, tag = "1")]
    pub icao_address: u32,
    /// Latitude
    #[prost(int32, tag = "2")]
    pub lat: i32,
    /// Longitude
    #[prost(int32, tag = "3")]
    pub lon: i32,
    /// Altitude(ASL)
    #[prost(int32, tag = "4")]
    pub altitude: i32,
    /// Course over ground
    #[prost(uint32, tag = "5")]
    pub heading: u32,
    /// The horizontal velocity
    #[prost(uint32, tag = "6")]
    pub hor_velocity: u32,
    /// The vertical velocity. Positive is up
    #[prost(int32, tag = "7")]
    pub ver_velocity: i32,
    /// Bitmap to indicate various statuses including valid data fields
    /// bitfield defined by enum ADSB_FLAGS
    #[prost(uint32, tag = "8")]
    pub flags: u32,
    /// Squawk code
    #[prost(uint32, tag = "9")]
    pub squawk: u32,
    /// ADSB altitude type.
    #[prost(enumeration = "AdsbAltitudeType", tag = "10")]
    pub altitude_type: i32,
    /// The callsign, 8+null
    #[prost(string, tag = "11")]
    pub callsign: ::prost::alloc::string::String,
    /// ADSB emitter type.
    #[prost(enumeration = "AdsbEmitterType", tag = "12")]
    pub emitter_type: i32,
    /// Time since last communication in seconds
    #[prost(uint32, tag = "13")]
    pub tslc: u32,
}
/// Information about a potential collision
///
/// MavLink id: 247
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Collision {
    /// Unique identifier, domain based on src field
    #[prost(uint32, tag = "1")]
    pub id: u32,
    /// Estimated time until collision occurs
    #[prost(float, tag = "2")]
    pub time_to_minimum_delta: f32,
    /// Closest vertical distance between vehicle and object
    #[prost(float, tag = "3")]
    pub altitude_minimum_delta: f32,
    /// Closest horizontal distance between vehicle and object
    #[prost(float, tag = "4")]
    pub horizontal_minimum_delta: f32,
    /// Collision data source
    #[prost(enumeration = "MavCollisionSrc", tag = "5")]
    pub src: i32,
    /// Action that is being taken to avoid this collision
    #[prost(enumeration = "MavCollisionAction", tag = "6")]
    pub action: i32,
    /// How concerned the aircraft is about this collision
    #[prost(enumeration = "MavCollisionThreatLevel", tag = "7")]
    pub threat_level: i32,
}
/// Message implementing parts of the V2 payload specs in V1 frames for transitional support.
///
/// MavLink id: 248
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct V2Extension {
    /// A code that identifies the software component that understands this message (analogous to USB device classes or mime type strings). If this code is less than 32768, it is considered a 'registered' protocol extension and the corresponding entry should be added to https://github.com/mavlink/mavlink/definition_files/extension_message_ids.xml. Software creators can register blocks of message IDs as needed (useful for GCS specific metadata, etc...). Message_types greater than 32767 are considered local experiments and should not be checked in to any widely distributed codebase.
    #[prost(uint32, tag = "1")]
    pub message_type: u32,
    /// Network ID (0 for broadcast)
    #[prost(uint32, tag = "2")]
    pub target_network: u32,
    /// System ID (0 for broadcast)
    #[prost(uint32, tag = "3")]
    pub target_system: u32,
    /// Component ID (0 for broadcast)
    #[prost(uint32, tag = "4")]
    pub target_component: u32,
    /// Variable length payload. The length must be encoded in the payload as part of the message_type protocol, e.g. by including the length as payload data, or by terminating the payload data with a non-zero marker. This is required in order to reconstruct zero-terminated payloads that are (or otherwise would be) trimmed by MAVLink 2 empty-byte truncation. The entire content of the payload block is opaque unless you understand the encoding message_type. The particular encoding used can be extension specific and might not always be documented as part of the MAVLink specification.
    #[prost(uint32, repeated, packed = "false", tag = "5")]
    pub payload: ::prost::alloc::vec::Vec<u32>,
}
/// Send raw controller memory. The use of this message is discouraged for normal packets, but a quite efficient way for testing new messages and getting experimental debug output.
///
/// MavLink id: 249
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MemoryVect {
    /// Starting address of the debug variables
    #[prost(uint32, tag = "1")]
    pub address: u32,
    /// Version code of the type variable. 0=unknown, type ignored and assumed int16_t. 1=as below
    #[prost(uint32, tag = "2")]
    pub ver: u32,
    /// Type code of the memory variables. for ver = 1: 0=16 x int16_t, 1=16 x uint16_t, 2=16 x Q15, 3=16 x 1Q14
    #[prost(uint32, tag = "3")]
    pub r#type: u32,
    /// Memory contents at specified address
    #[prost(int32, repeated, packed = "false", tag = "4")]
    pub value: ::prost::alloc::vec::Vec<i32>,
}
/// To debug something using a named 3D vector.
///
/// MavLink id: 250
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct DebugVect {
    /// Timestamp (UNIX Epoch time or time since system boot). The receiving end can infer timestamp format (since 1.1.1970 or since system boot) by checking for the magnitude of the number.
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// x
    #[prost(float, tag = "2")]
    pub x: f32,
    /// y
    #[prost(float, tag = "3")]
    pub y: f32,
    /// z
    #[prost(float, tag = "4")]
    pub z: f32,
    /// Name
    #[prost(string, tag = "5")]
    pub name: ::prost::alloc::string::String,
}
/// Send a key-value pair as float. The use of this message is discouraged for normal packets, but a quite efficient way for testing new messages and getting experimental debug output.
///
/// MavLink id: 251
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct NamedValueFloat {
    /// Timestamp (time since system boot).
    #[prost(uint32, tag = "1")]
    pub time_boot_ms: u32,
    /// Floating point value
    #[prost(float, tag = "2")]
    pub value: f32,
    /// Name of the debug variable
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
}
/// Send a key-value pair as integer. The use of this message is discouraged for normal packets, but a quite efficient way for testing new messages and getting experimental debug output.
///
/// MavLink id: 252
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct NamedValueInt {
    /// Timestamp (time since system boot).
    #[prost(uint32, tag = "1")]
    pub time_boot_ms: u32,
    /// Signed integer value
    #[prost(int32, tag = "2")]
    pub value: i32,
    /// Name of the debug variable
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
}
/// Status text message. These messages are printed in yellow in the COMM console of QGroundControl. WARNING: They consume quite some bandwidth, so use only for important status and error messages. If implemented wisely, these messages are buffered on the MCU and sent only at a limited rate (e.g. 10 Hz).
///
/// MavLink id: 253
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Statustext {
    /// Severity of status. Relies on the definitions within RFC-5424.
    #[prost(enumeration = "MavSeverity", tag = "1")]
    pub severity: i32,
    /// Status text message, without null termination character
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
}
/// Send a debug value. The index is used to discriminate between values. These values show up in the plot of QGroundControl as DEBUG N.
///
/// MavLink id: 254
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Debug {
    /// Timestamp (time since system boot).
    #[prost(uint32, tag = "1")]
    pub time_boot_ms: u32,
    /// DEBUG value
    #[prost(float, tag = "2")]
    pub value: f32,
    /// index of debug variable
    #[prost(uint32, tag = "3")]
    pub ind: u32,
}
/// Setup a MAVLink2 signing key. If called with secret_key of all zero and zero initial_timestamp will disable signing
///
/// MavLink id: 256
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetupSigning {
    /// initial timestamp
    #[prost(uint64, tag = "1")]
    pub initial_timestamp: u64,
    /// system id of the target
    #[prost(uint32, tag = "2")]
    pub target_system: u32,
    /// component ID of the target
    #[prost(uint32, tag = "3")]
    pub target_component: u32,
    /// signing key
    #[prost(uint32, repeated, packed = "false", tag = "4")]
    pub secret_key: ::prost::alloc::vec::Vec<u32>,
}
/// Report button state change.
///
/// MavLink id: 257
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ButtonChange {
    /// Timestamp (time since system boot).
    #[prost(uint32, tag = "1")]
    pub time_boot_ms: u32,
    /// Time of last change of button state.
    #[prost(uint32, tag = "2")]
    pub last_change_ms: u32,
    /// Bitmap for state of buttons.
    #[prost(uint32, tag = "3")]
    pub state: u32,
}
/// Control vehicle tone generation (buzzer).
///
/// MavLink id: 258
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PlayTune {
    /// System ID
    #[prost(uint32, tag = "1")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "2")]
    pub target_component: u32,
    /// tune in board specific format
    #[prost(string, tag = "3")]
    pub tune: ::prost::alloc::string::String,
}
/// Information about a camera. Can be requested with a MAV_CMD_REQUEST_MESSAGE command.
///
/// MavLink id: 259
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CameraInformation {
    /// Timestamp (time since system boot).
    #[prost(uint32, tag = "1")]
    pub time_boot_ms: u32,
    /// Version of the camera firmware, encoded as: (Dev & 0xff) << 24 | (Patch & 0xff) << 16 | (Minor & 0xff) << 8 | (Major & 0xff)
    #[prost(uint32, tag = "2")]
    pub firmware_version: u32,
    /// Focal length
    #[prost(float, tag = "3")]
    pub focal_length: f32,
    /// Image sensor size horizontal
    #[prost(float, tag = "4")]
    pub sensor_size_h: f32,
    /// Image sensor size vertical
    #[prost(float, tag = "5")]
    pub sensor_size_v: f32,
    /// Bitmap of camera capability flags.
    /// bitfield defined by enum CAMERA_CAP_FLAGS
    #[prost(uint32, tag = "6")]
    pub flags: u32,
    /// Horizontal image resolution
    #[prost(uint32, tag = "7")]
    pub resolution_h: u32,
    /// Vertical image resolution
    #[prost(uint32, tag = "8")]
    pub resolution_v: u32,
    /// Camera definition version (iteration)
    #[prost(uint32, tag = "9")]
    pub cam_definition_version: u32,
    /// Name of the camera vendor
    #[prost(uint32, repeated, packed = "false", tag = "10")]
    pub vendor_name: ::prost::alloc::vec::Vec<u32>,
    /// Name of the camera model
    #[prost(uint32, repeated, packed = "false", tag = "11")]
    pub model_name: ::prost::alloc::vec::Vec<u32>,
    /// Reserved for a lens ID
    #[prost(uint32, tag = "12")]
    pub lens_id: u32,
    /// Camera definition URI (if any, otherwise only basic functions will be available). HTTP- (http://) and MAVLink FTP- (mavlinkftp://) formatted URIs are allowed (and both must be supported by any GCS that implements the Camera Protocol).
    #[prost(string, tag = "13")]
    pub cam_definition_uri: ::prost::alloc::string::String,
}
/// Settings of a camera. Can be requested with a MAV_CMD_REQUEST_MESSAGE command.
///
/// MavLink id: 260
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CameraSettings {
    /// Timestamp (time since system boot).
    #[prost(uint32, tag = "1")]
    pub time_boot_ms: u32,
    /// Camera mode
    #[prost(enumeration = "CameraMode", tag = "2")]
    pub mode_id: i32,
}
/// Information about a storage medium. This message is sent in response to a request with MAV_CMD_REQUEST_MESSAGE and whenever the status of the storage changes (STORAGE_STATUS). Use MAV_CMD_REQUEST_MESSAGE.param2 to indicate the index/id of requested storage: 0 for all, 1 for first, 2 for second, etc.
///
/// MavLink id: 261
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct StorageInformation {
    /// Timestamp (time since system boot).
    #[prost(uint32, tag = "1")]
    pub time_boot_ms: u32,
    /// Total capacity. If storage is not ready (STORAGE_STATUS_READY) value will be ignored.
    #[prost(float, tag = "2")]
    pub total_capacity: f32,
    /// Used capacity. If storage is not ready (STORAGE_STATUS_READY) value will be ignored.
    #[prost(float, tag = "3")]
    pub used_capacity: f32,
    /// Available storage capacity. If storage is not ready (STORAGE_STATUS_READY) value will be ignored.
    #[prost(float, tag = "4")]
    pub available_capacity: f32,
    /// Read speed.
    #[prost(float, tag = "5")]
    pub read_speed: f32,
    /// Write speed.
    #[prost(float, tag = "6")]
    pub write_speed: f32,
    /// Storage ID (1 for first, 2 for second, etc.)
    #[prost(uint32, tag = "7")]
    pub storage_id: u32,
    /// Number of storage devices
    #[prost(uint32, tag = "8")]
    pub storage_count: u32,
    /// Status of storage
    #[prost(enumeration = "StorageStatus", tag = "9")]
    pub status: i32,
}
/// Information about the status of a capture. Can be requested with a MAV_CMD_REQUEST_MESSAGE command.
///
/// MavLink id: 262
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CameraCaptureStatus {
    /// Timestamp (time since system boot).
    #[prost(uint32, tag = "1")]
    pub time_boot_ms: u32,
    /// Image capture interval
    #[prost(float, tag = "2")]
    pub image_interval: f32,
    /// Time since recording started
    #[prost(uint32, tag = "3")]
    pub recording_time_ms: u32,
    /// Available storage capacity.
    #[prost(float, tag = "4")]
    pub available_capacity: f32,
    /// Current status of image capturing (0: idle, 1: capture in progress, 2: interval set but idle, 3: interval set and capture in progress)
    #[prost(uint32, tag = "5")]
    pub image_status: u32,
    /// Current status of video capturing (0: idle, 1: capture in progress)
    #[prost(uint32, tag = "6")]
    pub video_status: u32,
}
/// Information about a captured image. This is emitted every time a message is captured. It may be re-requested using MAV_CMD_REQUEST_MESSAGE, using param2 to indicate the sequence number for the missing image.
///
/// MavLink id: 263
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CameraImageCaptured {
    /// Timestamp (time since UNIX epoch) in UTC. 0 for unknown.
    #[prost(uint64, tag = "1")]
    pub time_utc: u64,
    /// Timestamp (time since system boot).
    #[prost(uint32, tag = "2")]
    pub time_boot_ms: u32,
    /// Latitude where image was taken
    #[prost(int32, tag = "3")]
    pub lat: i32,
    /// Longitude where capture was taken
    #[prost(int32, tag = "4")]
    pub lon: i32,
    /// Altitude (MSL) where image was taken
    #[prost(int32, tag = "5")]
    pub alt: i32,
    /// Altitude above ground
    #[prost(int32, tag = "6")]
    pub relative_alt: i32,
    /// Quaternion of camera orientation (w, x, y, z order, zero-rotation is 0, 0, 0, 0)
    #[prost(float, repeated, packed = "false", tag = "7")]
    pub q: ::prost::alloc::vec::Vec<f32>,
    /// Zero based index of this image (i.e. a new image will have index CAMERA_CAPTURE_STATUS.image count -1)
    #[prost(int32, tag = "8")]
    pub image_index: i32,
    /// Camera ID (1 for first, 2 for second, etc.)
    #[prost(uint32, tag = "9")]
    pub camera_id: u32,
    /// Boolean indicating success (1) or failure (0) while capturing this image.
    #[prost(int32, tag = "10")]
    pub capture_result: i32,
    /// URL of image taken. Either local storage or http://foo.jpg if camera provides an HTTP interface.
    #[prost(string, tag = "11")]
    pub file_url: ::prost::alloc::string::String,
}
/// Information about flight since last arming.
///
/// MavLink id: 264
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct FlightInformation {
    /// Timestamp at arming (time since UNIX epoch) in UTC, 0 for unknown
    #[prost(uint64, tag = "1")]
    pub arming_time_utc: u64,
    /// Timestamp at takeoff (time since UNIX epoch) in UTC, 0 for unknown
    #[prost(uint64, tag = "2")]
    pub takeoff_time_utc: u64,
    /// Universally unique identifier (UUID) of flight, should correspond to name of log files
    #[prost(uint64, tag = "3")]
    pub flight_uuid: u64,
    /// Timestamp (time since system boot).
    #[prost(uint32, tag = "4")]
    pub time_boot_ms: u32,
}
/// Orientation of a mount
///
/// MavLink id: 265
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MountOrientation {
    /// Timestamp (time since system boot).
    #[prost(uint32, tag = "1")]
    pub time_boot_ms: u32,
    /// Roll in global frame (set to NaN for invalid).
    #[prost(float, tag = "2")]
    pub roll: f32,
    /// Pitch in global frame (set to NaN for invalid).
    #[prost(float, tag = "3")]
    pub pitch: f32,
    /// Yaw relative to vehicle (set to NaN for invalid).
    #[prost(float, tag = "4")]
    pub yaw: f32,
}
/// A message containing logged data (see also MAV_CMD_LOGGING_START)
///
/// MavLink id: 266
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct LoggingData {
    /// sequence number (can wrap)
    #[prost(uint32, tag = "1")]
    pub sequence: u32,
    /// system ID of the target
    #[prost(uint32, tag = "2")]
    pub target_system: u32,
    /// component ID of the target
    #[prost(uint32, tag = "3")]
    pub target_component: u32,
    /// data length
    #[prost(uint32, tag = "4")]
    pub length: u32,
    /// offset into data where first message starts. This can be used for recovery, when a previous message got lost (set to 255 if no start exists).
    #[prost(uint32, tag = "5")]
    pub first_message_offset: u32,
    /// logged data
    #[prost(uint32, repeated, packed = "false", tag = "6")]
    pub data: ::prost::alloc::vec::Vec<u32>,
}
/// A message containing logged data which requires a LOGGING_ACK to be sent back
///
/// MavLink id: 267
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct LoggingDataAcked {
    /// sequence number (can wrap)
    #[prost(uint32, tag = "1")]
    pub sequence: u32,
    /// system ID of the target
    #[prost(uint32, tag = "2")]
    pub target_system: u32,
    /// component ID of the target
    #[prost(uint32, tag = "3")]
    pub target_component: u32,
    /// data length
    #[prost(uint32, tag = "4")]
    pub length: u32,
    /// offset into data where first message starts. This can be used for recovery, when a previous message got lost (set to 255 if no start exists).
    #[prost(uint32, tag = "5")]
    pub first_message_offset: u32,
    /// logged data
    #[prost(uint32, repeated, packed = "false", tag = "6")]
    pub data: ::prost::alloc::vec::Vec<u32>,
}
/// An ack for a LOGGING_DATA_ACKED message
///
/// MavLink id: 268
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct LoggingAck {
    /// sequence number (must match the one in LOGGING_DATA_ACKED)
    #[prost(uint32, tag = "1")]
    pub sequence: u32,
    /// system ID of the target
    #[prost(uint32, tag = "2")]
    pub target_system: u32,
    /// component ID of the target
    #[prost(uint32, tag = "3")]
    pub target_component: u32,
}
/// Information about video stream. It may be requested using MAV_CMD_REQUEST_MESSAGE, where param2 indicates the video stream id: 0 for all streams, 1 for first, 2 for second, etc.
///
/// MavLink id: 269
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct VideoStreamInformation {
    /// Frame rate.
    #[prost(float, tag = "1")]
    pub framerate: f32,
    /// Bit rate.
    #[prost(uint32, tag = "2")]
    pub bitrate: u32,
    /// Bitmap of stream status flags.
    /// bitfield defined by enum VIDEO_STREAM_STATUS_FLAGS
    #[prost(uint32, tag = "3")]
    pub flags: u32,
    /// Horizontal resolution.
    #[prost(uint32, tag = "4")]
    pub resolution_h: u32,
    /// Vertical resolution.
    #[prost(uint32, tag = "5")]
    pub resolution_v: u32,
    /// Video image rotation clockwise.
    #[prost(uint32, tag = "6")]
    pub rotation: u32,
    /// Horizontal Field of view.
    #[prost(uint32, tag = "7")]
    pub hfov: u32,
    /// Video Stream ID (1 for first, 2 for second, etc.)
    #[prost(uint32, tag = "8")]
    pub stream_id: u32,
    /// Number of streams available.
    #[prost(uint32, tag = "9")]
    pub count: u32,
    /// Type of stream.
    #[prost(enumeration = "VideoStreamType", tag = "10")]
    pub r#type: i32,
    /// Stream name.
    #[prost(string, tag = "11")]
    pub name: ::prost::alloc::string::String,
    /// Video stream URI (TCP or RTSP URI ground station should connect to) or port number (UDP port ground station should listen to).
    #[prost(string, tag = "12")]
    pub uri: ::prost::alloc::string::String,
}
/// Information about the status of a video stream. It may be requested using MAV_CMD_REQUEST_MESSAGE.
///
/// MavLink id: 270
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct VideoStreamStatus {
    /// Frame rate
    #[prost(float, tag = "1")]
    pub framerate: f32,
    /// Bit rate
    #[prost(uint32, tag = "2")]
    pub bitrate: u32,
    /// Bitmap of stream status flags
    /// bitfield defined by enum VIDEO_STREAM_STATUS_FLAGS
    #[prost(uint32, tag = "3")]
    pub flags: u32,
    /// Horizontal resolution
    #[prost(uint32, tag = "4")]
    pub resolution_h: u32,
    /// Vertical resolution
    #[prost(uint32, tag = "5")]
    pub resolution_v: u32,
    /// Video image rotation clockwise
    #[prost(uint32, tag = "6")]
    pub rotation: u32,
    /// Horizontal Field of view
    #[prost(uint32, tag = "7")]
    pub hfov: u32,
    /// Video Stream ID (1 for first, 2 for second, etc.)
    #[prost(uint32, tag = "8")]
    pub stream_id: u32,
}
/// Information about a high level gimbal manager. This message should be requested by a ground station using MAV_CMD_REQUEST_MESSAGE.
///
/// MavLink id: 280
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GimbalManagerInformation {
    /// Timestamp (time since system boot).
    #[prost(uint32, tag = "1")]
    pub time_boot_ms: u32,
    /// Bitmap of gimbal capability flags.
    /// bitfield defined by enum GIMBAL_MANAGER_CAP_FLAGS
    #[prost(uint32, tag = "2")]
    pub cap_flags: u32,
    /// Maximum tilt/pitch angle (positive: up, negative: down)
    #[prost(float, tag = "3")]
    pub tilt_max: f32,
    /// Minimum tilt/pitch angle (positive: up, negative: down)
    #[prost(float, tag = "4")]
    pub tilt_min: f32,
    /// Maximum tilt/pitch angular rate (positive: up, negative: down)
    #[prost(float, tag = "5")]
    pub tilt_rate_max: f32,
    /// Maximum pan/yaw angle (positive: to the right, negative: to the left)
    #[prost(float, tag = "6")]
    pub pan_max: f32,
    /// Minimum pan/yaw angle (positive: to the right, negative: to the left)
    #[prost(float, tag = "7")]
    pub pan_min: f32,
    /// Minimum pan/yaw angular rate (positive: to the right, negative: to the left)
    #[prost(float, tag = "8")]
    pub pan_rate_max: f32,
    /// Gimbal device ID that this gimbal manager is responsible for.
    #[prost(uint32, tag = "9")]
    pub gimbal_device_id: u32,
}
/// Current status about a high level gimbal manager. This message should be broadcast at a low regular rate (e.g. 5Hz).
///
/// MavLink id: 281
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GimbalManagerStatus {
    /// Timestamp (time since system boot).
    #[prost(uint32, tag = "1")]
    pub time_boot_ms: u32,
    /// High level gimbal manager flags currently applied.
    /// bitfield defined by enum GIMBAL_MANAGER_FLAGS
    #[prost(uint32, tag = "2")]
    pub flags: u32,
    /// Gimbal device ID that this gimbal manager is responsible for.
    #[prost(uint32, tag = "3")]
    pub gimbal_device_id: u32,
}
/// High level message to control a gimbal's attitude. This message is to be sent to the gimbal manager (e.g. from a ground station). Angles and rates can be set to NaN according to use case.
///
/// MavLink id: 282
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GimbalManagerSetAttitude {
    /// High level gimbal manager flags to use.
    /// bitfield defined by enum GIMBAL_MANAGER_FLAGS
    #[prost(uint32, tag = "1")]
    pub flags: u32,
    /// Quaternion components, w, x, y, z (1 0 0 0 is the null-rotation, the frame is depends on whether the flag GIMBAL_MANAGER_FLAGS_YAW_LOCK is set)
    #[prost(float, repeated, packed = "false", tag = "2")]
    pub q: ::prost::alloc::vec::Vec<f32>,
    /// X component of angular velocity, positive is banking to the right, NaN to be ignored.
    #[prost(float, tag = "3")]
    pub angular_velocity_x: f32,
    /// Y component of angular velocity, positive is tilting up, NaN to be ignored.
    #[prost(float, tag = "4")]
    pub angular_velocity_y: f32,
    /// Z component of angular velocity, positive is panning to the right, NaN to be ignored.
    #[prost(float, tag = "5")]
    pub angular_velocity_z: f32,
    /// System ID
    #[prost(uint32, tag = "6")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "7")]
    pub target_component: u32,
    /// Component ID of gimbal device to address (or 1-6 for non-MAVLink gimbal), 0 for all gimbal device components. (Send command multiple times for more than one but not all gimbals.)
    #[prost(uint32, tag = "8")]
    pub gimbal_device_id: u32,
}
/// Information about a low level gimbal. This message should be requested by the gimbal manager or a ground station using MAV_CMD_REQUEST_MESSAGE.
///
/// MavLink id: 283
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GimbalDeviceInformation {
    /// Timestamp (time since system boot).
    #[prost(uint32, tag = "1")]
    pub time_boot_ms: u32,
    /// Version of the gimbal firmware, encoded as: (Dev & 0xff) << 24 | (Patch & 0xff) << 16 | (Minor & 0xff) << 8 | (Major & 0xff)
    #[prost(uint32, tag = "2")]
    pub firmware_version: u32,
    /// Maximum tilt/pitch angle (positive: up, negative: down)
    #[prost(float, tag = "3")]
    pub tilt_max: f32,
    /// Minimum tilt/pitch angle (positive: up, negative: down)
    #[prost(float, tag = "4")]
    pub tilt_min: f32,
    /// Maximum tilt/pitch angular rate (positive: up, negative: down)
    #[prost(float, tag = "5")]
    pub tilt_rate_max: f32,
    /// Maximum pan/yaw angle (positive: to the right, negative: to the left)
    #[prost(float, tag = "6")]
    pub pan_max: f32,
    /// Minimum pan/yaw angle (positive: to the right, negative: to the left)
    #[prost(float, tag = "7")]
    pub pan_min: f32,
    /// Minimum pan/yaw angular rate (positive: to the right, negative: to the left)
    #[prost(float, tag = "8")]
    pub pan_rate_max: f32,
    /// Bitmap of gimbal capability flags.
    /// bitfield defined by enum GIMBAL_DEVICE_CAP_FLAGS
    #[prost(uint32, tag = "9")]
    pub cap_flags: u32,
    /// Name of the gimbal vendor
    #[prost(uint32, repeated, packed = "false", tag = "10")]
    pub vendor_name: ::prost::alloc::vec::Vec<u32>,
    /// Name of the gimbal model
    #[prost(uint32, repeated, packed = "false", tag = "11")]
    pub model_name: ::prost::alloc::vec::Vec<u32>,
}
/// Low level message to control a gimbal device's attitude. This message is to be sent from the gimbal manager to the gimbal device component. Angles and rates can be set to NaN according to use case.
///
/// MavLink id: 284
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GimbalDeviceSetAttitude {
    /// Quaternion components, w, x, y, z (1 0 0 0 is the null-rotation, the frame is depends on whether the flag GIMBAL_DEVICE_FLAGS_YAW_LOCK is set, set all fields to NaN if only angular velocity should be used)
    #[prost(float, repeated, packed = "false", tag = "1")]
    pub q: ::prost::alloc::vec::Vec<f32>,
    /// X component of angular velocity, positive is banking to the right, NaN to be ignored.
    #[prost(float, tag = "2")]
    pub angular_velocity_x: f32,
    /// Y component of angular velocity, positive is tilting up, NaN to be ignored.
    #[prost(float, tag = "3")]
    pub angular_velocity_y: f32,
    /// Z component of angular velocity, positive is panning to the right, NaN to be ignored.
    #[prost(float, tag = "4")]
    pub angular_velocity_z: f32,
    /// Low level gimbal flags.
    /// bitfield defined by enum GIMBAL_DEVICE_FLAGS
    #[prost(uint32, tag = "5")]
    pub flags: u32,
    /// System ID
    #[prost(uint32, tag = "6")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "7")]
    pub target_component: u32,
}
/// Message reporting the status of a gimbal device. This message should be broadcasted by a gimbal device component. The angles encoded in the quaternion are in the global frame (roll: positive is tilt to the right, pitch: positive is tilting up, yaw is turn to the right). This message should be broadcast at a low regular rate (e.g. 10Hz).
///
/// MavLink id: 285
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GimbalDeviceAttitudeStatus {
    /// Timestamp (time since system boot).
    #[prost(uint32, tag = "1")]
    pub time_boot_ms: u32,
    /// Quaternion components, w, x, y, z (1 0 0 0 is the null-rotation, the frame is depends on whether the flag GIMBAL_DEVICE_FLAGS_YAW_LOCK is set)
    #[prost(float, repeated, packed = "false", tag = "2")]
    pub q: ::prost::alloc::vec::Vec<f32>,
    /// X component of angular velocity (NaN if unknown)
    #[prost(float, tag = "3")]
    pub angular_velocity_x: f32,
    /// Y component of angular velocity (NaN if unknown)
    #[prost(float, tag = "4")]
    pub angular_velocity_y: f32,
    /// Z component of angular velocity (NaN if unknown)
    #[prost(float, tag = "5")]
    pub angular_velocity_z: f32,
    /// Failure flags (0 for no failure)
    /// bitfield defined by enum GIMBAL_DEVICE_ERROR_FLAGS
    #[prost(uint32, tag = "6")]
    pub failure_flags: u32,
    /// Current gimbal flags set.
    /// bitfield defined by enum GIMBAL_DEVICE_FLAGS
    #[prost(uint32, tag = "7")]
    pub flags: u32,
    /// System ID
    #[prost(uint32, tag = "8")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "9")]
    pub target_component: u32,
}
/// Low level message containing autopilot state relevant for a gimbal device. This message is to be sent from the gimbal manager to the gimbal device component. The data of this message server for the gimbal's estimator corrections in particular horizon compensation, as well as the autopilot's control intention e.g. feed forward angular control in z-axis.
///
/// MavLink id: 286
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct AutopilotStateForGimbalDevice {
    /// Timestamp (time since system boot).
    #[prost(uint64, tag = "1")]
    pub time_boot_us: u64,
    /// Quaternion components of autopilot attitude: w, x, y, z (1 0 0 0 is the null-rotation, Hamiltonian convention).
    #[prost(float, repeated, packed = "false", tag = "2")]
    pub q: ::prost::alloc::vec::Vec<f32>,
    /// Estimated delay of the attitude data.
    #[prost(uint32, tag = "3")]
    pub q_estimated_delay_us: u32,
    /// X Speed in NED (North, East, Down).
    #[prost(float, tag = "4")]
    pub vx: f32,
    /// Y Speed in NED (North, East, Down).
    #[prost(float, tag = "5")]
    pub vy: f32,
    /// Z Speed in NED (North, East, Down).
    #[prost(float, tag = "6")]
    pub vz: f32,
    /// Estimated delay of the speed data.
    #[prost(uint32, tag = "7")]
    pub v_estimated_delay_us: u32,
    /// Feed forward Z component of angular velocity, positive is yawing to the right, NaN to be ignored. This is to indicate if the autopilot is actively yawing.
    #[prost(float, tag = "8")]
    pub feed_forward_angular_velocity_z: f32,
    /// Bitmap indicating which estimator outputs are valid.
    /// bitfield defined by enum ESTIMATOR_STATUS_FLAGS
    #[prost(uint32, tag = "9")]
    pub estimator_status: u32,
    /// System ID
    #[prost(uint32, tag = "10")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "11")]
    pub target_component: u32,
    /// The landed state. Is set to MAV_LANDED_STATE_UNDEFINED if landed state is unknown.
    #[prost(enumeration = "MavLandedState", tag = "12")]
    pub landed_state: i32,
}
/// High level message to control a gimbal's tilt and pan angles. This message is to be sent to the gimbal manager (e.g. from a ground station). Angles and rates can be set to NaN according to use case.
///
/// MavLink id: 287
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GimbalManagerSetTiltpan {
    /// High level gimbal manager flags to use.
    /// bitfield defined by enum GIMBAL_MANAGER_FLAGS
    #[prost(uint32, tag = "1")]
    pub flags: u32,
    /// Tilt/pitch angle (positive: up, negative: down, NaN to be ignored).
    #[prost(float, tag = "2")]
    pub tilt: f32,
    /// Pan/yaw angle (positive: to the right, negative: to the left, NaN to be ignored).
    #[prost(float, tag = "3")]
    pub pan: f32,
    /// Tilt/pitch angular rate (positive: up, negative: down, NaN to be ignored).
    #[prost(float, tag = "4")]
    pub tilt_rate: f32,
    /// Pan/yaw angular rate (positive: to the right, negative: to the left, NaN to be ignored).
    #[prost(float, tag = "5")]
    pub pan_rate: f32,
    /// System ID
    #[prost(uint32, tag = "6")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "7")]
    pub target_component: u32,
    /// Component ID of gimbal device to address (or 1-6 for non-MAVLink gimbal), 0 for all gimbal device components. (Send command multiple times for more than one but not all gimbals.)
    #[prost(uint32, tag = "8")]
    pub gimbal_device_id: u32,
}
/// Configure WiFi AP SSID, password, and mode. This message is re-emitted as an acknowledgement by the AP. The message may also be explicitly requested using MAV_CMD_REQUEST_MESSAGE
///
/// MavLink id: 299
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct WifiConfigAp {
    /// Name of Wi-Fi network (SSID). Blank to leave it unchanged when setting. Current SSID when sent back as a response.
    #[prost(string, tag = "1")]
    pub ssid: ::prost::alloc::string::String,
    /// Password. Blank for an open AP. MD5 hash when message is sent back as a response.
    #[prost(string, tag = "2")]
    pub password: ::prost::alloc::string::String,
}
/// Version and capability of protocol version. This message can be requested with MAV_CMD_REQUEST_MESSAGE and is used as part of the handshaking to establish which MAVLink version should be used on the network. Every node should respond to a request for PROTOCOL_VERSION to enable the handshaking. Library implementers should consider adding this into the default decoding state machine to allow the protocol core to respond directly.
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
/// The location and information of an AIS vessel
///
/// MavLink id: 301
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct AisVessel {
    /// Mobile Marine Service Identifier, 9 decimal digits
    #[prost(uint32, tag = "1")]
    pub mmsi: u32,
    /// Latitude
    #[prost(int32, tag = "2")]
    pub lat: i32,
    /// Longitude
    #[prost(int32, tag = "3")]
    pub lon: i32,
    /// Course over ground
    #[prost(uint32, tag = "4")]
    pub cog: u32,
    /// True heading
    #[prost(uint32, tag = "5")]
    pub heading: u32,
    /// Speed over ground
    #[prost(uint32, tag = "6")]
    pub velocity: u32,
    /// Distance from lat/lon location to bow
    #[prost(uint32, tag = "7")]
    pub dimension_bow: u32,
    /// Distance from lat/lon location to stern
    #[prost(uint32, tag = "8")]
    pub dimension_stern: u32,
    /// Time since last communication in seconds
    #[prost(uint32, tag = "9")]
    pub tslc: u32,
    /// Bitmask to indicate various statuses including valid data fields
    /// bitfield defined by enum AIS_FLAGS
    #[prost(uint32, tag = "10")]
    pub flags: u32,
    /// Turn rate
    #[prost(int32, tag = "11")]
    pub turn_rate: i32,
    /// Navigational status
    #[prost(enumeration = "AisNavStatus", tag = "12")]
    pub navigational_status: i32,
    /// Type of vessels
    #[prost(enumeration = "AisType", tag = "13")]
    pub r#type: i32,
    /// Distance from lat/lon location to port side
    #[prost(uint32, tag = "14")]
    pub dimension_port: u32,
    /// Distance from lat/lon location to starboard side
    #[prost(uint32, tag = "15")]
    pub dimension_starboard: u32,
    /// The vessel callsign
    #[prost(string, tag = "16")]
    pub callsign: ::prost::alloc::string::String,
    /// The vessel name
    #[prost(string, tag = "17")]
    pub name: ::prost::alloc::string::String,
}
/// General status information of an UAVCAN node. Please refer to the definition of the UAVCAN message "uavcan.protocol.NodeStatus" for the background information. The UAVCAN specification is available at http://uavcan.org.
///
/// MavLink id: 310
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct UavcanNodeStatus {
    /// Timestamp (UNIX Epoch time or time since system boot). The receiving end can infer timestamp format (since 1.1.1970 or since system boot) by checking for the magnitude of the number.
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// Time since the start-up of the node.
    #[prost(uint32, tag = "2")]
    pub uptime_sec: u32,
    /// Vendor-specific status information.
    #[prost(uint32, tag = "3")]
    pub vendor_specific_status_code: u32,
    /// Generalized node health status.
    #[prost(enumeration = "UavcanNodeHealth", tag = "4")]
    pub health: i32,
    /// Generalized operating mode.
    #[prost(enumeration = "UavcanNodeMode", tag = "5")]
    pub mode: i32,
    /// Not used currently.
    #[prost(uint32, tag = "6")]
    pub sub_mode: u32,
}
/// General information describing a particular UAVCAN node. Please refer to the definition of the UAVCAN service "uavcan.protocol.GetNodeInfo" for the background information. This message should be emitted by the system whenever a new node appears online, or an existing node reboots. Additionally, it can be emitted upon request from the other end of the MAVLink channel (see MAV_CMD_UAVCAN_GET_NODE_INFO). It is also not prohibited to emit this message unconditionally at a low frequency. The UAVCAN specification is available at http://uavcan.org.
///
/// MavLink id: 311
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct UavcanNodeInfo {
    /// Timestamp (UNIX Epoch time or time since system boot). The receiving end can infer timestamp format (since 1.1.1970 or since system boot) by checking for the magnitude of the number.
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// Time since the start-up of the node.
    #[prost(uint32, tag = "2")]
    pub uptime_sec: u32,
    /// Version control system (VCS) revision identifier (e.g. git short commit hash). Zero if unknown.
    #[prost(uint32, tag = "3")]
    pub sw_vcs_commit: u32,
    /// Node name string. For example, "sapog.px4.io".
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    /// Hardware major version number.
    #[prost(uint32, tag = "5")]
    pub hw_version_major: u32,
    /// Hardware minor version number.
    #[prost(uint32, tag = "6")]
    pub hw_version_minor: u32,
    /// Hardware unique 128-bit ID.
    #[prost(uint32, repeated, packed = "false", tag = "7")]
    pub hw_unique_id: ::prost::alloc::vec::Vec<u32>,
    /// Software major version number.
    #[prost(uint32, tag = "8")]
    pub sw_version_major: u32,
    /// Software minor version number.
    #[prost(uint32, tag = "9")]
    pub sw_version_minor: u32,
}
/// Request to read the value of a parameter with the either the param_id string id or param_index.
///
/// MavLink id: 320
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ParamExtRequestRead {
    /// Parameter index. Set to -1 to use the Parameter ID field as identifier (else param_id will be ignored)
    #[prost(int32, tag = "1")]
    pub param_index: i32,
    /// System ID
    #[prost(uint32, tag = "2")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "3")]
    pub target_component: u32,
    /// Parameter id, terminated by NULL if the length is less than 16 human-readable chars and WITHOUT null termination (NULL) byte if the length is exactly 16 chars - applications have to provide 16+1 bytes storage if the ID is stored as string
    #[prost(string, tag = "4")]
    pub param_id: ::prost::alloc::string::String,
}
/// Request all parameters of this component. After this request, all parameters are emitted.
///
/// MavLink id: 321
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ParamExtRequestList {
    /// System ID
    #[prost(uint32, tag = "1")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "2")]
    pub target_component: u32,
}
/// Emit the value of a parameter. The inclusion of param_count and param_index in the message allows the recipient to keep track of received parameters and allows them to re-request missing parameters after a loss or timeout.
///
/// MavLink id: 322
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ParamExtValue {
    /// Total number of parameters
    #[prost(uint32, tag = "1")]
    pub param_count: u32,
    /// Index of this parameter
    #[prost(uint32, tag = "2")]
    pub param_index: u32,
    /// Parameter id, terminated by NULL if the length is less than 16 human-readable chars and WITHOUT null termination (NULL) byte if the length is exactly 16 chars - applications have to provide 16+1 bytes storage if the ID is stored as string
    #[prost(string, tag = "3")]
    pub param_id: ::prost::alloc::string::String,
    /// Parameter value
    #[prost(string, tag = "4")]
    pub param_value: ::prost::alloc::string::String,
    /// Parameter type.
    #[prost(enumeration = "MavParamExtType", tag = "5")]
    pub param_type: i32,
}
/// Set a parameter value. In order to deal with message loss (and retransmission of PARAM_EXT_SET), when setting a parameter value and the new value is the same as the current value, you will immediately get a PARAM_ACK_ACCEPTED response. If the current state is PARAM_ACK_IN_PROGRESS, you will accordingly receive a PARAM_ACK_IN_PROGRESS in response.
///
/// MavLink id: 323
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ParamExtSet {
    /// System ID
    #[prost(uint32, tag = "1")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "2")]
    pub target_component: u32,
    /// Parameter id, terminated by NULL if the length is less than 16 human-readable chars and WITHOUT null termination (NULL) byte if the length is exactly 16 chars - applications have to provide 16+1 bytes storage if the ID is stored as string
    #[prost(string, tag = "3")]
    pub param_id: ::prost::alloc::string::String,
    /// Parameter value
    #[prost(string, tag = "4")]
    pub param_value: ::prost::alloc::string::String,
    /// Parameter type.
    #[prost(enumeration = "MavParamExtType", tag = "5")]
    pub param_type: i32,
}
/// Response from a PARAM_EXT_SET message.
///
/// MavLink id: 324
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ParamExtAck {
    /// Parameter id, terminated by NULL if the length is less than 16 human-readable chars and WITHOUT null termination (NULL) byte if the length is exactly 16 chars - applications have to provide 16+1 bytes storage if the ID is stored as string
    #[prost(string, tag = "1")]
    pub param_id: ::prost::alloc::string::String,
    /// Parameter value (new value if PARAM_ACK_ACCEPTED, current value otherwise)
    #[prost(string, tag = "2")]
    pub param_value: ::prost::alloc::string::String,
    /// Parameter type.
    #[prost(enumeration = "MavParamExtType", tag = "3")]
    pub param_type: i32,
    /// Result code.
    #[prost(enumeration = "ParamAck", tag = "4")]
    pub param_result: i32,
}
/// Obstacle distances in front of the sensor, starting from the left in increment degrees to the right
///
/// MavLink id: 330
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ObstacleDistance {
    /// Timestamp (UNIX Epoch time or time since system boot). The receiving end can infer timestamp format (since 1.1.1970 or since system boot) by checking for the magnitude of the number.
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// Distance of obstacles around the vehicle with index 0 corresponding to north + angle_offset, unless otherwise specified in the frame. A value of 0 is valid and means that the obstacle is practically touching the sensor. A value of max_distance +1 means no obstacle is present. A value of UINT16_MAX for unknown/not used. In a array element, one unit corresponds to 1cm.
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub distances: ::prost::alloc::vec::Vec<u32>,
    /// Minimum distance the sensor can measure.
    #[prost(uint32, tag = "3")]
    pub min_distance: u32,
    /// Maximum distance the sensor can measure.
    #[prost(uint32, tag = "4")]
    pub max_distance: u32,
    /// Class id of the distance sensor type.
    #[prost(enumeration = "MavDistanceSensor", tag = "5")]
    pub sensor_type: i32,
    /// Angular width in degrees of each array element. Increment direction is clockwise. This field is ignored if increment_f is non-zero.
    #[prost(uint32, tag = "6")]
    pub increment: u32,
}
/// Odometry message to communicate odometry information with an external interface. Fits ROS REP 147 standard for aerial vehicles (http://www.ros.org/reps/rep-0147.html).
///
/// MavLink id: 331
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Odometry {
    /// Timestamp (UNIX Epoch time or time since system boot). The receiving end can infer timestamp format (since 1.1.1970 or since system boot) by checking for the magnitude of the number.
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// X Position
    #[prost(float, tag = "2")]
    pub x: f32,
    /// Y Position
    #[prost(float, tag = "3")]
    pub y: f32,
    /// Z Position
    #[prost(float, tag = "4")]
    pub z: f32,
    /// Quaternion components, w, x, y, z (1 0 0 0 is the null-rotation)
    #[prost(float, repeated, packed = "false", tag = "5")]
    pub q: ::prost::alloc::vec::Vec<f32>,
    /// X linear speed
    #[prost(float, tag = "6")]
    pub vx: f32,
    /// Y linear speed
    #[prost(float, tag = "7")]
    pub vy: f32,
    /// Z linear speed
    #[prost(float, tag = "8")]
    pub vz: f32,
    /// Roll angular speed
    #[prost(float, tag = "9")]
    pub rollspeed: f32,
    /// Pitch angular speed
    #[prost(float, tag = "10")]
    pub pitchspeed: f32,
    /// Yaw angular speed
    #[prost(float, tag = "11")]
    pub yawspeed: f32,
    /// Row-major representation of a 6x6 pose cross-covariance matrix upper right triangle (states: x, y, z, roll, pitch, yaw; first six entries are the first ROW, next five entries are the second ROW, etc.). If unknown, assign NaN value to first element in the array.
    #[prost(float, repeated, packed = "false", tag = "12")]
    pub pose_covariance: ::prost::alloc::vec::Vec<f32>,
    /// Row-major representation of a 6x6 velocity cross-covariance matrix upper right triangle (states: vx, vy, vz, rollspeed, pitchspeed, yawspeed; first six entries are the first ROW, next five entries are the second ROW, etc.). If unknown, assign NaN value to first element in the array.
    #[prost(float, repeated, packed = "false", tag = "13")]
    pub velocity_covariance: ::prost::alloc::vec::Vec<f32>,
    /// Coordinate frame of reference for the pose data.
    #[prost(enumeration = "MavFrame", tag = "14")]
    pub frame_id: i32,
    /// Coordinate frame of reference for the velocity in free space (twist) data.
    #[prost(enumeration = "MavFrame", tag = "15")]
    pub child_frame_id: i32,
}
/// Describe a trajectory using an array of up-to 5 waypoints in the local frame (MAV_FRAME_LOCAL_NED).
///
/// MavLink id: 332
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TrajectoryRepresentationWaypoints {
    /// Timestamp (UNIX Epoch time or time since system boot). The receiving end can infer timestamp format (since 1.1.1970 or since system boot) by checking for the magnitude of the number.
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// X-coordinate of waypoint, set to NaN if not being used
    #[prost(float, repeated, packed = "false", tag = "2")]
    pub pos_x: ::prost::alloc::vec::Vec<f32>,
    /// Y-coordinate of waypoint, set to NaN if not being used
    #[prost(float, repeated, packed = "false", tag = "3")]
    pub pos_y: ::prost::alloc::vec::Vec<f32>,
    /// Z-coordinate of waypoint, set to NaN if not being used
    #[prost(float, repeated, packed = "false", tag = "4")]
    pub pos_z: ::prost::alloc::vec::Vec<f32>,
    /// X-velocity of waypoint, set to NaN if not being used
    #[prost(float, repeated, packed = "false", tag = "5")]
    pub vel_x: ::prost::alloc::vec::Vec<f32>,
    /// Y-velocity of waypoint, set to NaN if not being used
    #[prost(float, repeated, packed = "false", tag = "6")]
    pub vel_y: ::prost::alloc::vec::Vec<f32>,
    /// Z-velocity of waypoint, set to NaN if not being used
    #[prost(float, repeated, packed = "false", tag = "7")]
    pub vel_z: ::prost::alloc::vec::Vec<f32>,
    /// X-acceleration of waypoint, set to NaN if not being used
    #[prost(float, repeated, packed = "false", tag = "8")]
    pub acc_x: ::prost::alloc::vec::Vec<f32>,
    /// Y-acceleration of waypoint, set to NaN if not being used
    #[prost(float, repeated, packed = "false", tag = "9")]
    pub acc_y: ::prost::alloc::vec::Vec<f32>,
    /// Z-acceleration of waypoint, set to NaN if not being used
    #[prost(float, repeated, packed = "false", tag = "10")]
    pub acc_z: ::prost::alloc::vec::Vec<f32>,
    /// Yaw angle, set to NaN if not being used
    #[prost(float, repeated, packed = "false", tag = "11")]
    pub pos_yaw: ::prost::alloc::vec::Vec<f32>,
    /// Yaw rate, set to NaN if not being used
    #[prost(float, repeated, packed = "false", tag = "12")]
    pub vel_yaw: ::prost::alloc::vec::Vec<f32>,
    /// Scheduled action for each waypoint, UINT16_MAX if not being used.
    #[prost(enumeration = "MavCmd", repeated, packed = "false", tag = "13")]
    pub command: ::prost::alloc::vec::Vec<i32>,
    /// Number of valid points (up-to 5 waypoints are possible)
    #[prost(uint32, tag = "14")]
    pub valid_points: u32,
}
/// Describe a trajectory using an array of up-to 5 bezier control points in the local frame (MAV_FRAME_LOCAL_NED).
///
/// MavLink id: 333
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TrajectoryRepresentationBezier {
    /// Timestamp (UNIX Epoch time or time since system boot). The receiving end can infer timestamp format (since 1.1.1970 or since system boot) by checking for the magnitude of the number.
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// X-coordinate of bezier control points. Set to NaN if not being used
    #[prost(float, repeated, packed = "false", tag = "2")]
    pub pos_x: ::prost::alloc::vec::Vec<f32>,
    /// Y-coordinate of bezier control points. Set to NaN if not being used
    #[prost(float, repeated, packed = "false", tag = "3")]
    pub pos_y: ::prost::alloc::vec::Vec<f32>,
    /// Z-coordinate of bezier control points. Set to NaN if not being used
    #[prost(float, repeated, packed = "false", tag = "4")]
    pub pos_z: ::prost::alloc::vec::Vec<f32>,
    /// Bezier time horizon. Set to NaN if velocity/acceleration should not be incorporated
    #[prost(float, repeated, packed = "false", tag = "5")]
    pub delta: ::prost::alloc::vec::Vec<f32>,
    /// Yaw. Set to NaN for unchanged
    #[prost(float, repeated, packed = "false", tag = "6")]
    pub pos_yaw: ::prost::alloc::vec::Vec<f32>,
    /// Number of valid control points (up-to 5 points are possible)
    #[prost(uint32, tag = "7")]
    pub valid_points: u32,
}
/// Report current used cellular network status
///
/// MavLink id: 334
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CellularStatus {
    /// Mobile country code. If unknown, set to UINT16_MAX
    #[prost(uint32, tag = "1")]
    pub mcc: u32,
    /// Mobile network code. If unknown, set to UINT16_MAX
    #[prost(uint32, tag = "2")]
    pub mnc: u32,
    /// Location area code. If unknown, set to 0
    #[prost(uint32, tag = "3")]
    pub lac: u32,
    /// Cellular modem status
    #[prost(enumeration = "CellularStatusFlag", tag = "4")]
    pub status: i32,
    /// Failure reason when status in in CELLUAR_STATUS_FAILED
    #[prost(enumeration = "CellularNetworkFailedReason", tag = "5")]
    pub failure_reason: i32,
    /// Cellular network radio type: gsm, cdma, lte...
    #[prost(enumeration = "CellularNetworkRadioType", tag = "6")]
    pub r#type: i32,
    /// Signal quality in percent. If unknown, set to UINT8_MAX
    #[prost(uint32, tag = "7")]
    pub quality: u32,
}
/// Status of the Iridium SBD link.
///
/// MavLink id: 335
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct IsbdLinkStatus {
    /// Timestamp (UNIX Epoch time or time since system boot). The receiving end can infer timestamp format (since 1.1.1970 or since system boot) by checking for the magnitude of the number.
    #[prost(uint64, tag = "1")]
    pub timestamp: u64,
    /// Timestamp of the last successful sbd session. The receiving end can infer timestamp format (since 1.1.1970 or since system boot) by checking for the magnitude of the number.
    #[prost(uint64, tag = "2")]
    pub last_heartbeat: u64,
    /// Number of failed SBD sessions.
    #[prost(uint32, tag = "3")]
    pub failed_sessions: u32,
    /// Number of successful SBD sessions.
    #[prost(uint32, tag = "4")]
    pub successful_sessions: u32,
    /// Signal quality equal to the number of bars displayed on the ISU signal strength indicator. Range is 0 to 5, where 0 indicates no signal and 5 indicates maximum signal strength.
    #[prost(uint32, tag = "5")]
    pub signal_quality: u32,
    /// 1: Ring call pending, 0: No call pending.
    #[prost(uint32, tag = "6")]
    pub ring_pending: u32,
    /// 1: Transmission session pending, 0: No transmission session pending.
    #[prost(uint32, tag = "7")]
    pub tx_session_pending: u32,
    /// 1: Receiving session pending, 0: No receiving session pending.
    #[prost(uint32, tag = "8")]
    pub rx_session_pending: u32,
}
/// Configure cellular modems. This message is re-emitted as an acknowledgement by the modem. The message may also be explicitly requested using MAV_CMD_REQUEST_MESSAGE.
///
/// MavLink id: 336
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CellularConfig {
    /// Enable / disable PIN on the SIM card. 0: Unchange setttings 1: PIN disabled, 2: PIN enabled.
    #[prost(uint32, tag = "1")]
    pub enable_pin: u32,
    /// PIN sent to the simcard. Blank when PIN is disabled. Empty when message is sent back as a response.
    #[prost(string, tag = "2")]
    pub pin: ::prost::alloc::string::String,
    /// Name of the cellular APN. Blank to leave it unchanged when setting. Current APN when sent back as a response.
    #[prost(string, tag = "3")]
    pub apn: ::prost::alloc::string::String,
    /// Required PUK code in case the user failed to authenticate 3 times with the PIN.
    #[prost(string, tag = "4")]
    pub puk: ::prost::alloc::string::String,
    /// Configure whether roaming is allowed, 0: settings not changed, 1: roaming disabled, 2: roaming enabled.
    #[prost(uint32, tag = "5")]
    pub roaming: u32,
    /// Message acceptance response (sent back to GS).
    #[prost(enumeration = "CellularConfigResponse", tag = "6")]
    pub response: i32,
}
/// RPM sensor data message.
///
/// MavLink id: 339
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct RawRpm {
    /// Indicated rate
    #[prost(float, tag = "1")]
    pub frequency: f32,
    /// Index of this RPM sensor (0-indexed)
    #[prost(uint32, tag = "2")]
    pub index: u32,
}
/// The global position resulting from GPS and sensor fusion.
///
/// MavLink id: 340
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct UtmGlobalPosition {
    /// Time of applicability of position (microseconds since UNIX epoch).
    #[prost(uint64, tag = "1")]
    pub time: u64,
    /// Latitude (WGS84)
    #[prost(int32, tag = "2")]
    pub lat: i32,
    /// Longitude (WGS84)
    #[prost(int32, tag = "3")]
    pub lon: i32,
    /// Altitude (WGS84)
    #[prost(int32, tag = "4")]
    pub alt: i32,
    /// Altitude above ground
    #[prost(int32, tag = "5")]
    pub relative_alt: i32,
    /// Next waypoint, latitude (WGS84)
    #[prost(int32, tag = "6")]
    pub next_lat: i32,
    /// Next waypoint, longitude (WGS84)
    #[prost(int32, tag = "7")]
    pub next_lon: i32,
    /// Next waypoint, altitude (WGS84)
    #[prost(int32, tag = "8")]
    pub next_alt: i32,
    /// Ground X speed (latitude, positive north)
    #[prost(int32, tag = "9")]
    pub vx: i32,
    /// Ground Y speed (longitude, positive east)
    #[prost(int32, tag = "10")]
    pub vy: i32,
    /// Ground Z speed (altitude, positive down)
    #[prost(int32, tag = "11")]
    pub vz: i32,
    /// Horizontal position uncertainty (standard deviation)
    #[prost(uint32, tag = "12")]
    pub h_acc: u32,
    /// Altitude uncertainty (standard deviation)
    #[prost(uint32, tag = "13")]
    pub v_acc: u32,
    /// Speed uncertainty (standard deviation)
    #[prost(uint32, tag = "14")]
    pub vel_acc: u32,
    /// Time until next update. Set to 0 if unknown or in data driven mode.
    #[prost(uint32, tag = "15")]
    pub update_rate: u32,
    /// Unique UAS ID.
    #[prost(uint32, repeated, packed = "false", tag = "16")]
    pub uas_id: ::prost::alloc::vec::Vec<u32>,
    /// Flight state
    #[prost(enumeration = "UtmFlightState", tag = "17")]
    pub flight_state: i32,
    /// Bitwise OR combination of the data available flags.
    /// bitfield defined by enum UTM_DATA_AVAIL_FLAGS
    #[prost(uint32, tag = "18")]
    pub flags: u32,
}
/// Large debug/prototyping array. The message uses the maximum available payload for data. The array_id and name fields are used to discriminate between messages in code and in user interfaces (respectively). Do not use in production code.
///
/// MavLink id: 350
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct DebugFloatArray {
    /// Timestamp (UNIX Epoch time or time since system boot). The receiving end can infer timestamp format (since 1.1.1970 or since system boot) by checking for the magnitude of the number.
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// Unique ID used to discriminate between arrays
    #[prost(uint32, tag = "2")]
    pub array_id: u32,
    /// Name, for human-friendly display in a Ground Control Station
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
}
/// Vehicle status report that is sent out while orbit execution is in progress (see MAV_CMD_DO_ORBIT).
///
/// MavLink id: 360
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct OrbitExecutionStatus {
    /// Timestamp (UNIX Epoch time or time since system boot). The receiving end can infer timestamp format (since 1.1.1970 or since system boot) by checking for the magnitude of the number.
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// Radius of the orbit circle. Positive values orbit clockwise, negative values orbit counter-clockwise.
    #[prost(float, tag = "2")]
    pub radius: f32,
    /// X coordinate of center point. Coordinate system depends on frame field: local = x position in meters * 1e4, global = latitude in degrees * 1e7.
    #[prost(int32, tag = "3")]
    pub x: i32,
    /// Y coordinate of center point.  Coordinate system depends on frame field: local = x position in meters * 1e4, global = latitude in degrees * 1e7.
    #[prost(int32, tag = "4")]
    pub y: i32,
    /// Altitude of center point. Coordinate system depends on frame field.
    #[prost(float, tag = "5")]
    pub z: f32,
    /// The coordinate system of the fields: x, y, z.
    #[prost(enumeration = "MavFrame", tag = "6")]
    pub frame: i32,
}
/// Smart Battery information (static/infrequent update). Use for updates from: smart battery to flight stack, flight stack to GCS. Use instead of BATTERY_STATUS for smart batteries.
///
/// MavLink id: 370
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SmartBatteryInfo {
    /// Capacity when full according to manufacturer, -1: field not provided.
    #[prost(int32, tag = "1")]
    pub capacity_full_specification: i32,
    /// Capacity when full (accounting for battery degradation), -1: field not provided.
    #[prost(int32, tag = "2")]
    pub capacity_full: i32,
    /// Serial number. -1: field not provided.
    #[prost(int32, tag = "3")]
    pub serial_number: i32,
    /// Charge/discharge cycle count. -1: field not provided.
    #[prost(uint32, tag = "4")]
    pub cycle_count: u32,
    /// Battery weight. 0: field not provided.
    #[prost(uint32, tag = "5")]
    pub weight: u32,
    /// Minimum per-cell voltage when discharging. If not supplied set to UINT16_MAX value.
    #[prost(uint32, tag = "6")]
    pub discharge_minimum_voltage: u32,
    /// Minimum per-cell voltage when charging. If not supplied set to UINT16_MAX value.
    #[prost(uint32, tag = "7")]
    pub charging_minimum_voltage: u32,
    /// Minimum per-cell voltage when resting. If not supplied set to UINT16_MAX value.
    #[prost(uint32, tag = "8")]
    pub resting_minimum_voltage: u32,
    /// Battery ID
    #[prost(uint32, tag = "9")]
    pub id: u32,
    /// Static device name. Encode as manufacturer and product names separated using an underscore.
    #[prost(string, tag = "10")]
    pub device_name: ::prost::alloc::string::String,
}
/// Smart Battery information (dynamic). Use for updates from: smart battery to flight stack, flight stack to GCS. Use instead of BATTERY_STATUS for smart batteries.
///
/// MavLink id: 371
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SmartBatteryStatus {
    /// Fault/health indications.
    /// bitfield defined by enum MAV_SMART_BATTERY_FAULT
    #[prost(int32, tag = "1")]
    pub fault_bitmask: i32,
    /// Estimated remaining battery time. -1: field not provided.
    #[prost(int32, tag = "2")]
    pub time_remaining: i32,
    /// Battery ID
    #[prost(uint32, tag = "3")]
    pub id: u32,
    /// Remaining battery energy. Values: [0-100], -1: field not provided.
    #[prost(int32, tag = "4")]
    pub capacity_remaining: i32,
    /// Battery current (through all cells/loads). Positive if discharging, negative if charging. UINT16_MAX: field not provided.
    #[prost(int32, tag = "5")]
    pub current: i32,
    /// Battery temperature. -1: field not provided.
    #[prost(int32, tag = "6")]
    pub temperature: i32,
    /// The cell number of the first index in the 'voltages' array field. Using this field allows you to specify cell voltages for batteries with more than 16 cells.
    #[prost(uint32, tag = "7")]
    pub cell_offset: u32,
    /// Individual cell voltages. Batteries with more 16 cells can use the cell_offset field to specify the cell offset for the array specified in the current message . Index values above the valid cell count for this battery should have the UINT16_MAX value.
    #[prost(uint32, repeated, packed = "false", tag = "8")]
    pub voltages: ::prost::alloc::vec::Vec<u32>,
}
/// Telemetry of power generation system. Alternator or mechanical generator.
///
/// MavLink id: 373
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GeneratorStatus {
    /// Status flags.
    /// bitfield defined by enum MAV_GENERATOR_STATUS_FLAG
    #[prost(uint64, tag = "1")]
    pub status: u64,
    /// Current into/out of battery. Positive for out. Negative for in. NaN: field not provided.
    #[prost(float, tag = "2")]
    pub battery_current: f32,
    /// Current going to the UAV. If battery current not available this is the DC current from the generator. Positive for out. Negative for in. NaN: field not provided
    #[prost(float, tag = "3")]
    pub load_current: f32,
    /// The power being generated. NaN: field not provided
    #[prost(float, tag = "4")]
    pub power_generated: f32,
    /// Voltage of the bus seen at the generator, or battery bus if battery bus is controlled by generator and at a different voltage to main bus.
    #[prost(float, tag = "5")]
    pub bus_voltage: f32,
    /// The target battery current. Positive for out. Negative for in. NaN: field not provided
    #[prost(float, tag = "6")]
    pub bat_current_setpoint: f32,
    /// Speed of electrical generator or alternator. UINT16_MAX: field not provided.
    #[prost(uint32, tag = "7")]
    pub generator_speed: u32,
    /// The temperature of the rectifier or power converter. INT16_MAX: field not provided.
    #[prost(int32, tag = "8")]
    pub rectifier_temperature: i32,
    /// The temperature of the mechanical motor, fuel cell core or generator. INT16_MAX: field not provided.
    #[prost(int32, tag = "9")]
    pub generator_temperature: i32,
}
/// The raw values of the actuator outputs (e.g. on Pixhawk, from MAIN, AUX ports). This message supersedes SERVO_OUTPUT_RAW.
///
/// MavLink id: 375
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ActuatorOutputStatus {
    /// Timestamp (since system boot).
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// Active outputs
    #[prost(uint32, tag = "2")]
    pub active: u32,
    /// Servo / motor output array values. Zero values indicate unused channels.
    #[prost(float, repeated, packed = "false", tag = "3")]
    pub actuator: ::prost::alloc::vec::Vec<f32>,
}
/// Time/duration estimates for various events and actions given the current vehicle state and position.
///
/// MavLink id: 380
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TimeEstimateToTarget {
    /// Estimated time to complete the vehicle's configured "safe return" action from its current position (e.g. RTL, Smart RTL, etc.). -1 indicates that the vehicle is landed, or that no time estimate available.
    #[prost(int32, tag = "1")]
    pub safe_return: i32,
    /// Estimated time for vehicle to complete the LAND action from its current position. -1 indicates that the vehicle is landed, or that no time estimate available.
    #[prost(int32, tag = "2")]
    pub land: i32,
    /// Estimated time for reaching/completing the currently active mission item. -1 means no time estimate available.
    #[prost(int32, tag = "3")]
    pub mission_next_item: i32,
    /// Estimated time for completing the current mission. -1 means no mission active and/or no estimate available.
    #[prost(int32, tag = "4")]
    pub mission_end: i32,
    /// Estimated time for completing the current commanded action (i.e. Go To, Takeoff, Land, etc.). -1 means no action active and/or no estimate available.
    #[prost(int32, tag = "5")]
    pub commanded_action: i32,
}
/// Message for transporting "arbitrary" variable-length data from one component to another (broadcast is not forbidden, but discouraged). The encoding of the data is usually extension specific, i.e. determined by the source, and is usually not documented as part of the MAVLink specification.
///
/// MavLink id: 385
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Tunnel {
    /// A code that identifies the content of the payload (0 for unknown, which is the default). If this code is less than 32768, it is a 'registered' payload type and the corresponding code should be added to the MAV_TUNNEL_PAYLOAD_TYPE enum. Software creators can register blocks of types as needed. Codes greater than 32767 are considered local experiments and should not be checked in to any widely distributed codebase.
    #[prost(enumeration = "MavTunnelPayloadType", tag = "1")]
    pub payload_type: i32,
    /// System ID (can be 0 for broadcast, but this is discouraged)
    #[prost(uint32, tag = "2")]
    pub target_system: u32,
    /// Component ID (can be 0 for broadcast, but this is discouraged)
    #[prost(uint32, tag = "3")]
    pub target_component: u32,
    /// Length of the data transported in payload
    #[prost(uint32, tag = "4")]
    pub payload_length: u32,
    /// Variable length payload. The payload length is defined by payload_length. The entire content of this block is opaque unless you understand the encoding specified by payload_type.
    #[prost(uint32, repeated, packed = "false", tag = "5")]
    pub payload: ::prost::alloc::vec::Vec<u32>,
}
/// Hardware status sent by an onboard computer.
///
/// MavLink id: 390
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct OnboardComputerStatus {
    /// Timestamp (UNIX Epoch time or time since system boot). The receiving end can infer timestamp format (since 1.1.1970 or since system boot) by checking for the magnitude of the number.
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// Time since system boot.
    #[prost(uint32, tag = "2")]
    pub uptime: u32,
    /// Amount of used RAM on the component system. A value of UINT32_MAX implies the field is unused.
    #[prost(uint32, tag = "3")]
    pub ram_usage: u32,
    /// Total amount of RAM on the component system. A value of UINT32_MAX implies the field is unused.
    #[prost(uint32, tag = "4")]
    pub ram_total: u32,
    /// Storage type: 0: HDD, 1: SSD, 2: EMMC, 3: SD card (non-removable), 4: SD card (removable). A value of UINT32_MAX implies the field is unused.
    #[prost(uint32, repeated, packed = "false", tag = "5")]
    pub storage_type: ::prost::alloc::vec::Vec<u32>,
    /// Amount of used storage space on the component system. A value of UINT32_MAX implies the field is unused.
    #[prost(uint32, repeated, packed = "false", tag = "6")]
    pub storage_usage: ::prost::alloc::vec::Vec<u32>,
    /// Total amount of storage space on the component system. A value of UINT32_MAX implies the field is unused.
    #[prost(uint32, repeated, packed = "false", tag = "7")]
    pub storage_total: ::prost::alloc::vec::Vec<u32>,
    /// Link type: 0-9: UART, 10-19: Wired network, 20-29: Wifi, 30-39: Point-to-point proprietary, 40-49: Mesh proprietary
    #[prost(uint32, repeated, packed = "false", tag = "8")]
    pub link_type: ::prost::alloc::vec::Vec<u32>,
    /// Network traffic from the component system. A value of UINT32_MAX implies the field is unused.
    #[prost(uint32, repeated, packed = "false", tag = "9")]
    pub link_tx_rate: ::prost::alloc::vec::Vec<u32>,
    /// Network traffic to the component system. A value of UINT32_MAX implies the field is unused.
    #[prost(uint32, repeated, packed = "false", tag = "10")]
    pub link_rx_rate: ::prost::alloc::vec::Vec<u32>,
    /// Network capacity from the component system. A value of UINT32_MAX implies the field is unused.
    #[prost(uint32, repeated, packed = "false", tag = "11")]
    pub link_tx_max: ::prost::alloc::vec::Vec<u32>,
    /// Network capacity to the component system. A value of UINT32_MAX implies the field is unused.
    #[prost(uint32, repeated, packed = "false", tag = "12")]
    pub link_rx_max: ::prost::alloc::vec::Vec<u32>,
    /// Fan speeds. A value of INT16_MAX implies the field is unused.
    #[prost(int32, repeated, packed = "false", tag = "13")]
    pub fan_speed: ::prost::alloc::vec::Vec<i32>,
    /// Type of the onboard computer: 0: Mission computer primary, 1: Mission computer backup 1, 2: Mission computer backup 2, 3: Compute node, 4-5: Compute spares, 6-9: Payload computers.
    #[prost(uint32, tag = "14")]
    pub r#type: u32,
    /// CPU usage on the component in percent (100 - idle). A value of UINT8_MAX implies the field is unused.
    #[prost(uint32, repeated, packed = "false", tag = "15")]
    pub cpu_cores: ::prost::alloc::vec::Vec<u32>,
    /// Combined CPU usage as the last 10 slices of 100 MS (a histogram). This allows to identify spikes in load that max out the system, but only for a short amount of time. A value of UINT8_MAX implies the field is unused.
    #[prost(uint32, repeated, packed = "false", tag = "16")]
    pub cpu_combined: ::prost::alloc::vec::Vec<u32>,
    /// GPU usage on the component in percent (100 - idle). A value of UINT8_MAX implies the field is unused.
    #[prost(uint32, repeated, packed = "false", tag = "17")]
    pub gpu_cores: ::prost::alloc::vec::Vec<u32>,
    /// Combined GPU usage as the last 10 slices of 100 MS (a histogram). This allows to identify spikes in load that max out the system, but only for a short amount of time. A value of UINT8_MAX implies the field is unused.
    #[prost(uint32, repeated, packed = "false", tag = "18")]
    pub gpu_combined: ::prost::alloc::vec::Vec<u32>,
    /// Temperature of the board. A value of INT8_MAX implies the field is unused.
    #[prost(int32, tag = "19")]
    pub temperature_board: i32,
    /// Temperature of the CPU core. A value of INT8_MAX implies the field is unused.
    #[prost(int32, repeated, packed = "false", tag = "20")]
    pub temperature_core: ::prost::alloc::vec::Vec<i32>,
}
/// Information about a component. For camera components instead use CAMERA_INFORMATION, and for autopilots use AUTOPILOT_VERSION. Components including GCSes should consider supporting requests of this message via MAV_CMD_REQUEST_MESSAGE.
///
/// MavLink id: 395
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ComponentInformation {
    /// Timestamp (time since system boot).
    #[prost(uint32, tag = "1")]
    pub time_boot_ms: u32,
    /// The type of metadata being requested.
    #[prost(enumeration = "CompMetadataType", tag = "2")]
    pub metadata_type: i32,
    /// Unique uid for this metadata which a gcs can use for created cached metadata and understanding whether it's cache it up to date or whether it needs to download new data.
    #[prost(uint32, tag = "3")]
    pub metadata_uid: u32,
    /// Unique uid for the translation file associated with the metadata.
    #[prost(uint32, tag = "4")]
    pub translation_uid: u32,
    /// Component definition URI. If prefix mavlinkftp:// file is downloaded from vehicle over mavlink ftp protocol. If prefix http[s]:// file is downloaded over internet. Files are json format. They can end in .gz to indicate file is in gzip format.
    #[prost(string, tag = "5")]
    pub metadata_uri: ::prost::alloc::string::String,
    /// The translations for strings within the metadata file. If null the either do not exist or may be included in the metadata file itself. The translations specified here supercede any which may be in the metadata file itself. The uri format is the same as component_metadata_uri . Files are in Json Translation spec format. Empty string indicates no tranlsation file.
    #[prost(string, tag = "6")]
    pub translation_uri: ::prost::alloc::string::String,
}
/// Play vehicle tone/tune (buzzer). Supersedes message PLAY_TUNE.
///
/// MavLink id: 400
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PlayTuneV2 {
    /// Tune format
    /// bitfield defined by enum TUNE_FORMAT
    #[prost(uint32, tag = "1")]
    pub format: u32,
    /// System ID
    #[prost(uint32, tag = "2")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "3")]
    pub target_component: u32,
    /// Tune definition as a NULL-terminated string.
    #[prost(string, tag = "4")]
    pub tune: ::prost::alloc::string::String,
}
/// Tune formats supported by vehicle. This should be emitted as response to MAV_CMD_REQUEST_MESSAGE.
///
/// MavLink id: 401
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SupportedTunes {
    /// Bitfield of supported tune formats.
    /// bitfield defined by enum TUNE_FORMAT
    #[prost(uint32, tag = "1")]
    pub format: u32,
    /// System ID
    #[prost(uint32, tag = "2")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "3")]
    pub target_component: u32,
}
/// Cumulative distance traveled for each reported wheel.
///
/// MavLink id: 9000
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct WheelDistance {
    /// Timestamp (synced to UNIX time or since system boot).
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// Distance reported by individual wheel encoders. Forward rotations increase values, reverse rotations decrease them. Not all wheels will necessarily have wheel encoders; the mapping of encoders to wheel positions must be agreed/understood by the endpoints.
    #[prost(double, repeated, packed = "false", tag = "2")]
    pub distance: ::prost::alloc::vec::Vec<f64>,
    /// Number of wheels reported.
    #[prost(uint32, tag = "3")]
    pub count: u32,
}
/// Data for filling the OpenDroneID Basic ID message. This and the below messages are primarily meant for feeding data to/from an OpenDroneID implementation. E.g. https://github.com/opendroneid/opendroneid-core-c. These messages are compatible with the ASTM Remote ID standard at https://www.astm.org/Standards/F3411.htm and the ASD-STAN Direct Remote ID standard. The usage of these messages is documented at https://mavlink.io/en/services/opendroneid.html.
///
/// MavLink id: 12900
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct OpenDroneIdBasicId {
    /// System ID (0 for broadcast).
    #[prost(uint32, tag = "1")]
    pub target_system: u32,
    /// Component ID (0 for broadcast).
    #[prost(uint32, tag = "2")]
    pub target_component: u32,
    /// Only used for drone ID data received from other UAs. See detailed description at https://mavlink.io/en/services/opendroneid.html.
    #[prost(uint32, repeated, packed = "false", tag = "3")]
    pub id_or_mac: ::prost::alloc::vec::Vec<u32>,
    /// Indicates the format for the uas_id field of this message.
    #[prost(enumeration = "MavOdidIdType", tag = "4")]
    pub id_type: i32,
    /// Indicates the type of UA (Unmanned Aircraft).
    #[prost(enumeration = "MavOdidUaType", tag = "5")]
    pub ua_type: i32,
    /// UAS (Unmanned Aircraft System) ID following the format specified by id_type. Shall be filled with nulls in the unused portion of the field.
    #[prost(uint32, repeated, packed = "false", tag = "6")]
    pub uas_id: ::prost::alloc::vec::Vec<u32>,
}
/// Data for filling the OpenDroneID Location message. The float data types are 32-bit IEEE 754. The Location message provides the location, altitude, direction and speed of the aircraft.
///
/// MavLink id: 12901
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct OpenDroneIdLocation {
    /// Current latitude of the unmanned aircraft. If unknown: 0 (both Lat/Lon).
    #[prost(int32, tag = "1")]
    pub latitude: i32,
    /// Current longitude of the unmanned aircraft. If unknown: 0 (both Lat/Lon).
    #[prost(int32, tag = "2")]
    pub longitude: i32,
    /// The altitude calculated from the barometric pressue. Reference is against 29.92inHg or 1013.2mb. If unknown: -1000 m.
    #[prost(float, tag = "3")]
    pub altitude_barometric: f32,
    /// The geodetic altitude as defined by WGS84. If unknown: -1000 m.
    #[prost(float, tag = "4")]
    pub altitude_geodetic: f32,
    /// The current height of the unmanned aircraft above the take-off location or the ground as indicated by height_reference. If unknown: -1000 m.
    #[prost(float, tag = "5")]
    pub height: f32,
    /// Seconds after the full hour with reference to UTC time. Typically the GPS outputs a time-of-week value in milliseconds. First convert that to UTC and then convert for this field using ((float) (time_week_ms % (60*60*1000))) / 1000.
    #[prost(float, tag = "6")]
    pub timestamp: f32,
    /// Direction over ground (not heading, but direction of movement) measured clockwise from true North: 0 - 35999 centi-degrees. If unknown: 36100 centi-degrees.
    #[prost(uint32, tag = "7")]
    pub direction: u32,
    /// Ground speed. Positive only. If unknown: 25500 cm/s. If speed is larger than 25425 cm/s, use 25425 cm/s.
    #[prost(uint32, tag = "8")]
    pub speed_horizontal: u32,
    /// The vertical speed. Up is positive. If unknown: 6300 cm/s. If speed is larger than 6200 cm/s, use 6200 cm/s. If lower than -6200 cm/s, use -6200 cm/s.
    #[prost(int32, tag = "9")]
    pub speed_vertical: i32,
    /// System ID (0 for broadcast).
    #[prost(uint32, tag = "10")]
    pub target_system: u32,
    /// Component ID (0 for broadcast).
    #[prost(uint32, tag = "11")]
    pub target_component: u32,
    /// Only used for drone ID data received from other UAs. See detailed description at https://mavlink.io/en/services/opendroneid.html.
    #[prost(uint32, repeated, packed = "false", tag = "12")]
    pub id_or_mac: ::prost::alloc::vec::Vec<u32>,
    /// Indicates whether the unmanned aircraft is on the ground or in the air.
    #[prost(enumeration = "MavOdidStatus", tag = "13")]
    pub status: i32,
    /// Indicates the reference point for the height field.
    #[prost(enumeration = "MavOdidHeightRef", tag = "14")]
    pub height_reference: i32,
    /// The accuracy of the horizontal position.
    #[prost(enumeration = "MavOdidHorAcc", tag = "15")]
    pub horizontal_accuracy: i32,
    /// The accuracy of the vertical position.
    #[prost(enumeration = "MavOdidVerAcc", tag = "16")]
    pub vertical_accuracy: i32,
    /// The accuracy of the barometric altitude.
    #[prost(enumeration = "MavOdidVerAcc", tag = "17")]
    pub barometer_accuracy: i32,
    /// The accuracy of the horizontal and vertical speed.
    #[prost(enumeration = "MavOdidSpeedAcc", tag = "18")]
    pub speed_accuracy: i32,
    /// The accuracy of the timestamps.
    #[prost(enumeration = "MavOdidTimeAcc", tag = "19")]
    pub timestamp_accuracy: i32,
}
/// Data for filling the OpenDroneID Authentication message. The Authentication Message defines a field that can provide a means of authenticity for the identity of the UAS (Unmanned Aircraft System). The Authentication message can have two different formats. Five data pages are supported. For data page 0, the fields PageCount, Length and TimeStamp are present and AuthData is only 17 bytes. For data page 1 through 4, PageCount, Length and TimeStamp are not present and the size of AuthData is 23 bytes.
///
/// MavLink id: 12902
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct OpenDroneIdAuthentication {
    /// This field is only present for page 0. 32 bit Unix Timestamp in seconds since 00:00:00 01/01/2019.
    #[prost(uint32, tag = "1")]
    pub timestamp: u32,
    /// System ID (0 for broadcast).
    #[prost(uint32, tag = "2")]
    pub target_system: u32,
    /// Component ID (0 for broadcast).
    #[prost(uint32, tag = "3")]
    pub target_component: u32,
    /// Only used for drone ID data received from other UAs. See detailed description at https://mavlink.io/en/services/opendroneid.html.
    #[prost(uint32, repeated, packed = "false", tag = "4")]
    pub id_or_mac: ::prost::alloc::vec::Vec<u32>,
    /// Indicates the type of authentication.
    #[prost(enumeration = "MavOdidAuthType", tag = "5")]
    pub authentication_type: i32,
    /// Allowed range is 0 - 4.
    #[prost(uint32, tag = "6")]
    pub data_page: u32,
    /// This field is only present for page 0. Allowed range is 0 - 5.
    #[prost(uint32, tag = "7")]
    pub page_count: u32,
    /// This field is only present for page 0. Total bytes of authentication_data from all data pages. Allowed range is 0 - 109 (17 + 23*4).
    #[prost(uint32, tag = "8")]
    pub length: u32,
    /// Opaque authentication data. For page 0, the size is only 17 bytes. For other pages, the size is 23 bytes. Shall be filled with nulls in the unused portion of the field.
    #[prost(uint32, repeated, packed = "false", tag = "9")]
    pub authentication_data: ::prost::alloc::vec::Vec<u32>,
}
/// Data for filling the OpenDroneID Self ID message. The Self ID Message is an opportunity for the operator to (optionally) declare their identity and purpose of the flight. This message can provide additional information that could reduce the threat profile of a UA (Unmanned Aircraft) flying in a particular area or manner.
///
/// MavLink id: 12903
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct OpenDroneIdSelfId {
    /// System ID (0 for broadcast).
    #[prost(uint32, tag = "1")]
    pub target_system: u32,
    /// Component ID (0 for broadcast).
    #[prost(uint32, tag = "2")]
    pub target_component: u32,
    /// Only used for drone ID data received from other UAs. See detailed description at https://mavlink.io/en/services/opendroneid.html.
    #[prost(uint32, repeated, packed = "false", tag = "3")]
    pub id_or_mac: ::prost::alloc::vec::Vec<u32>,
    /// Indicates the type of the description field.
    #[prost(enumeration = "MavOdidDescType", tag = "4")]
    pub description_type: i32,
    /// Text description or numeric value expressed as ASCII characters. Shall be filled with nulls in the unused portion of the field.
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
}
/// Data for filling the OpenDroneID System message. The System Message contains general system information including the operator location and possible aircraft group information.
///
/// MavLink id: 12904
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct OpenDroneIdSystem {
    /// Latitude of the operator. If unknown: 0 (both Lat/Lon).
    #[prost(int32, tag = "1")]
    pub operator_latitude: i32,
    /// Longitude of the operator. If unknown: 0 (both Lat/Lon).
    #[prost(int32, tag = "2")]
    pub operator_longitude: i32,
    /// Area Operations Ceiling relative to WGS84. If unknown: -1000 m.
    #[prost(float, tag = "3")]
    pub area_ceiling: f32,
    /// Area Operations Floor relative to WGS84. If unknown: -1000 m.
    #[prost(float, tag = "4")]
    pub area_floor: f32,
    /// Number of aircraft in the area, group or formation (default 1).
    #[prost(uint32, tag = "5")]
    pub area_count: u32,
    /// Radius of the cylindrical area of the group or formation (default 0).
    #[prost(uint32, tag = "6")]
    pub area_radius: u32,
    /// System ID (0 for broadcast).
    #[prost(uint32, tag = "7")]
    pub target_system: u32,
    /// Component ID (0 for broadcast).
    #[prost(uint32, tag = "8")]
    pub target_component: u32,
    /// Only used for drone ID data received from other UAs. See detailed description at https://mavlink.io/en/services/opendroneid.html.
    #[prost(uint32, repeated, packed = "false", tag = "9")]
    pub id_or_mac: ::prost::alloc::vec::Vec<u32>,
    /// Specifies the operator location type.
    #[prost(enumeration = "MavOdidOperatorLocationType", tag = "10")]
    pub operator_location_type: i32,
    /// Specifies the classification type of the UA.
    #[prost(enumeration = "MavOdidClassificationType", tag = "11")]
    pub classification_type: i32,
    /// When classification_type is MAV_ODID_CLASSIFICATION_TYPE_EU, specifies the category of the UA.
    #[prost(enumeration = "MavOdidCategoryEu", tag = "12")]
    pub category_eu: i32,
    /// When classification_type is MAV_ODID_CLASSIFICATION_TYPE_EU, specifies the class of the UA.
    #[prost(enumeration = "MavOdidClassEu", tag = "13")]
    pub class_eu: i32,
}
/// Data for filling the OpenDroneID Operator ID message, which contains the CAA (Civil Aviation Authority) issued operator ID.
///
/// MavLink id: 12905
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct OpenDroneIdOperatorId {
    /// System ID (0 for broadcast).
    #[prost(uint32, tag = "1")]
    pub target_system: u32,
    /// Component ID (0 for broadcast).
    #[prost(uint32, tag = "2")]
    pub target_component: u32,
    /// Only used for drone ID data received from other UAs. See detailed description at https://mavlink.io/en/services/opendroneid.html.
    #[prost(uint32, repeated, packed = "false", tag = "3")]
    pub id_or_mac: ::prost::alloc::vec::Vec<u32>,
    /// Indicates the type of the operator_id field.
    #[prost(enumeration = "MavOdidOperatorIdType", tag = "4")]
    pub operator_id_type: i32,
    /// Text description or numeric value expressed as ASCII characters. Shall be filled with nulls in the unused portion of the field.
    #[prost(string, tag = "5")]
    pub operator_id: ::prost::alloc::string::String,
}
/// An OpenDroneID message pack is a container for multiple encoded OpenDroneID messages (i.e. not in the format given for the above messages descriptions but after encoding into the compressed OpenDroneID byte format). Used e.g. when transmitting on Bluetooth 5.0 Long Range/Extended Advertising or on WiFi Neighbor Aware Networking.
///
/// MavLink id: 12915
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct OpenDroneIdMessagePack {
    /// System ID (0 for broadcast).
    #[prost(uint32, tag = "1")]
    pub target_system: u32,
    /// Component ID (0 for broadcast).
    #[prost(uint32, tag = "2")]
    pub target_component: u32,
    /// This field must currently always be equal to 25 (bytes), since all encoded OpenDroneID messages are specificed to have this length.
    #[prost(uint32, tag = "3")]
    pub single_message_size: u32,
    /// Number of encoded messages in the pack (not the number of bytes). Allowed range is 1 - 10.
    #[prost(uint32, tag = "4")]
    pub msg_pack_size: u32,
    /// Concatenation of encoded OpenDroneID messages. Shall be filled with nulls in the unused portion of the field.
    #[prost(uint32, repeated, packed = "false", tag = "5")]
    pub messages: ::prost::alloc::vec::Vec<u32>,
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
    /// SLUGS autopilot, http://slugsuav.soe.ucsc.edu
    Slugs = 2,
    /// ArduPilot - Plane/Copter/Rover/Sub/Tracker, https://ardupilot.org
    Ardupilotmega = 3,
    /// OpenPilot, http://openpilot.org
    Openpilot = 4,
    /// Generic autopilot only supporting simple waypoints
    GenericWaypointsOnly = 5,
    /// Generic autopilot supporting waypoints and other simple navigation commands
    GenericWaypointsAndSimpleNavigationOnly = 6,
    /// Generic autopilot supporting the full mission command set
    GenericMissionFull = 7,
    /// No valid autopilot, e.g. a GCS or other MAVLink component
    Invalid = 8,
    /// PPZ UAV - http://nongnu.org/paparazzi
    Ppz = 9,
    /// UAV Dev Board
    Udb = 10,
    /// FlexiPilot
    Fp = 11,
    /// PX4 Autopilot - http://px4.io/
    Px4 = 12,
    /// SMACCMPilot - http://smaccmpilot.org
    Smaccmpilot = 13,
    /// AutoQuad -- http://autoquad.org
    Autoquad = 14,
    /// Armazila -- http://armazila.com
    Armazila = 15,
    /// Aerob -- http://aerob.ru
    Aerob = 16,
    /// ASLUAV autopilot -- http://www.asl.ethz.ch
    Asluav = 17,
    /// SmartAP Autopilot - http://sky-drones.com
    Smartap = 18,
    /// AirRails - http://uaventure.com
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
    /// Open Drone ID. See https://mavlink.io/en/services/opendroneid.html.
    Odid = 34,
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
pub enum FirmwareVersionType {
    /// These values define the type of firmware release.  These values indicate the first version or release of this type.  For example the first alpha release would be 64, the second would be 65.
    /// development release
    Dev = 0,
    /// alpha release
    Alpha = 64,
    /// beta release
    Beta = 128,
    /// release candidate
    Rc = 192,
    /// official stable release
    Official = 255,
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
pub enum HlFailureFlag {
    /// Flags to report failure cases over the high latency telemtry.
    /// This enum is used to define bitmasks (work around protobuf limitations).
    /// Not used in MavLink, make protobuf happy.
    Undefined = 0,
    /// GPS failure.
    /// bit 1
    Gps = 1,
    /// Differential pressure sensor failure.
    /// bit 2
    DifferentialPressure = 2,
    /// Absolute pressure sensor failure.
    /// bit 3
    AbsolutePressure = 4,
    /// Accelerometer sensor failure.
    /// bit 4
    HlFailureFlag3dAccel = 8,
    /// Gyroscope sensor failure.
    /// bit 5
    HlFailureFlag3dGyro = 16,
    /// Magnetometer sensor failure.
    /// bit 6
    HlFailureFlag3dMag = 32,
    /// Terrain subsystem failure.
    /// bit 7
    Terrain = 64,
    /// Battery failure/critical low battery.
    /// bit 8
    Battery = 128,
    /// RC receiver failure/no rc connection.
    /// bit 9
    RcReceiver = 256,
    /// Offboard link failure.
    /// bit 10
    OffboardLink = 512,
    /// Engine failure.
    /// bit 11
    Engine = 1024,
    /// Geofence violation.
    /// bit 12
    Geofence = 2048,
    /// Estimator failure, for example measurement rejection or large variances.
    /// bit 13
    Estimator = 4096,
    /// Mission failure.
    /// bit 14
    Mission = 8192,
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
    /// This enum is used to define bitmasks (work around protobuf limitations).
    /// Not used in MavLink, make protobuf happy.
    Undefined = 0,
    /// Eighth bit: 00000001
    /// bit 1
    CustomMode = 1,
    /// Seventh bit: 00000010
    /// bit 2
    Test = 2,
    /// Sixth bit:   00000100
    /// bit 3
    Auto = 4,
    /// Fifth bit:  00001000
    /// bit 4
    Guided = 8,
    /// Fourth bit: 00010000
    /// bit 5
    Stabilize = 16,
    /// Third bit:  00100000
    /// bit 6
    Hil = 32,
    /// Second bit: 01000000
    /// bit 7
    Manual = 64,
    /// First bit:  10000000
    /// bit 8
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
pub enum MavGoto {
    /// Actions that may be specified in MAV_CMD_OVERRIDE_GOTO to override mission execution.
    /// Hold at the current position.
    DoHold = 0,
    /// Continue with the next item in mission execution.
    DoContinue = 1,
    /// Hold at the current position of the system
    HoldAtCurrentPosition = 2,
    /// Hold at the position specified in the parameters of the DO_HOLD action
    HoldAtSpecifiedPosition = 3,
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
pub enum MavMode {
    /// These defines are predefined OR-combined mode flags. There is no need to use values from this enum, but it
    /// simplifies the use of the mode flags. Note that manual input is enabled in all modes as a safety override.
    /// System is not ready to fly, booting, calibrating, etc. No flag is set.
    Preflight = 0,
    /// System is allowed to be active, under manual (RC) control, no stabilization
    ManualDisarmed = 64,
    /// UNDEFINED mode. This solely depends on the autopilot - use with caution, intended for developers only.
    TestDisarmed = 66,
    /// System is allowed to be active, under assisted RC control.
    StabilizeDisarmed = 80,
    /// System is allowed to be active, under autonomous control, manual setpoint
    GuidedDisarmed = 88,
    /// System is allowed to be active, under autonomous control and navigation (the trajectory is decided onboard and not pre-programmed by waypoints)
    AutoDisarmed = 92,
    /// System is allowed to be active, under manual (RC) control, no stabilization
    ManualArmed = 192,
    /// UNDEFINED mode. This solely depends on the autopilot - use with caution, intended for developers only.
    TestArmed = 194,
    /// System is allowed to be active, under assisted RC control.
    StabilizeArmed = 208,
    /// System is allowed to be active, under autonomous control, manual setpoint
    GuidedArmed = 216,
    /// System is allowed to be active, under autonomous control and navigation (the trajectory is decided onboard and not pre-programmed by waypoints)
    AutoArmed = 220,
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
    /// Target id (target_component) used to broadcast messages to all components of the receiving system. Components should attempt to process messages with this component ID and forward to components on any other interfaces. Note: This is not a valid *source* component id for a message.
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
    /// Telemetry radio (e.g. SiK radio, or other component that emits RADIO_STATUS messages).
    MavCompIdTelemetryRadio = 68,
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
    /// Gimbal #1.
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
    /// Gimbal #2.
    MavCompIdGimbal2 = 171,
    /// Gimbal #3.
    MavCompIdGimbal3 = 172,
    /// Gimbal #4
    MavCompIdGimbal4 = 173,
    /// Gimbal #5.
    MavCompIdGimbal5 = 174,
    /// Gimbal #6.
    MavCompIdGimbal6 = 175,
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
    /// Open Drone ID transmitter/receiver (Bluetooth/WiFi/Internet).
    MavCompIdOdidTxrx1 = 236,
    /// Open Drone ID transmitter/receiver (Bluetooth/WiFi/Internet).
    MavCompIdOdidTxrx2 = 237,
    /// Open Drone ID transmitter/receiver (Bluetooth/WiFi/Internet).
    MavCompIdOdidTxrx3 = 238,
    /// Component to bridge MAVLink to UDP (i.e. from a UART).
    MavCompIdUdpBridge = 240,
    /// Component to bridge to UART (i.e. from UDP).
    MavCompIdUartBridge = 241,
    /// Component handling TUNNEL messages (e.g. vendor specific GUI of a component).
    MavCompIdTunnelNode = 242,
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
pub enum MavSysStatusSensor {
    /// These encode the sensors whose status is sent as part of the SYS_STATUS message.
    /// This enum is used to define bitmasks (work around protobuf limitations).
    /// Not used in MavLink, make protobuf happy.
    Undefined = 0,
    /// 0x01 3D gyro
    /// bit 1
    MavSysStatusSensor3dGyro = 1,
    /// 0x02 3D accelerometer
    /// bit 2
    MavSysStatusSensor3dAccel = 2,
    /// 0x04 3D magnetometer
    /// bit 3
    MavSysStatusSensor3dMag = 4,
    /// 0x08 absolute pressure
    /// bit 4
    AbsolutePressure = 8,
    /// 0x10 differential pressure
    /// bit 5
    DifferentialPressure = 16,
    /// 0x20 GPS
    /// bit 6
    Gps = 32,
    /// 0x40 optical flow
    /// bit 7
    OpticalFlow = 64,
    /// 0x80 computer vision position
    /// bit 8
    VisionPosition = 128,
    /// 0x100 laser based position
    /// bit 9
    LaserPosition = 256,
    /// 0x200 external ground truth (Vicon or Leica)
    /// bit 10
    ExternalGroundTruth = 512,
    /// 0x400 3D angular rate control
    /// bit 11
    AngularRateControl = 1024,
    /// 0x800 attitude stabilization
    /// bit 12
    AttitudeStabilization = 2048,
    /// 0x1000 yaw position
    /// bit 13
    YawPosition = 4096,
    /// 0x2000 z/altitude control
    /// bit 14
    ZAltitudeControl = 8192,
    /// 0x4000 x/y position control
    /// bit 15
    XyPositionControl = 16384,
    /// 0x8000 motor outputs / control
    /// bit 16
    MotorOutputs = 32768,
    /// 0x10000 rc receiver
    /// bit 17
    RcReceiver = 65536,
    /// 0x20000 2nd 3D gyro
    /// bit 18
    MavSysStatusSensor3dGyro2 = 131072,
    /// 0x40000 2nd 3D accelerometer
    /// bit 19
    MavSysStatusSensor3dAccel2 = 262144,
    /// 0x80000 2nd 3D magnetometer
    /// bit 20
    MavSysStatusSensor3dMag2 = 524288,
    /// 0x100000 geofence
    /// bit 21
    MavSysStatusGeofence = 1048576,
    /// 0x200000 AHRS subsystem health
    /// bit 22
    MavSysStatusAhrs = 2097152,
    /// 0x400000 Terrain subsystem health
    /// bit 23
    MavSysStatusTerrain = 4194304,
    /// 0x800000 Motors are reversed
    /// bit 24
    MavSysStatusReverseMotor = 8388608,
    /// 0x1000000 Logging
    /// bit 25
    MavSysStatusLogging = 16777216,
    /// 0x2000000 Battery
    /// bit 26
    Battery = 33554432,
    /// 0x4000000 Proximity
    /// bit 27
    Proximity = 67108864,
    /// 0x8000000 Satellite Communication
    /// bit 28
    Satcom = 134217728,
    /// 0x10000000 pre-arm check status. Always healthy when armed
    /// bit 29
    MavSysStatusPrearmCheck = 268435456,
    /// 0x20000000 Avoidance/collision prevention
    /// bit 30
    MavSysStatusObstacleAvoidance = 536870912,
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
pub enum MavFrame {
    /// Global (WGS84) coordinate frame + MSL altitude. First value / x: latitude, second value / y: longitude, third value / z: positive altitude over mean sea level (MSL).
    Global = 0,
    /// Local coordinate frame, Z-down (x: North, y: East, z: Down).
    LocalNed = 1,
    /// NOT a coordinate frame, indicates a mission command.
    Mission = 2,
    /// Global (WGS84) coordinate frame + altitude relative to the home position. First value / x: latitude, second value / y: longitude, third value / z: positive altitude with 0 being at the altitude of the home location.
    GlobalRelativeAlt = 3,
    /// Local coordinate frame, Z-up (x: East, y: North, z: Up).
    LocalEnu = 4,
    /// Global (WGS84) coordinate frame (scaled) + MSL altitude. First value / x: latitude in degrees*1.0e-7, second value / y: longitude in degrees*1.0e-7, third value / z: positive altitude over mean sea level (MSL).
    GlobalInt = 5,
    /// Global (WGS84) coordinate frame (scaled) + altitude relative to the home position. First value / x: latitude in degrees*10e-7, second value / y: longitude in degrees*10e-7, third value / z: positive altitude with 0 being at the altitude of the home location.
    GlobalRelativeAltInt = 6,
    /// Offset to the current local frame. Anything expressed in this frame should be added to the current local frame position.
    LocalOffsetNed = 7,
    /// Setpoint in body NED frame. This makes sense if all position control is externalized - e.g. useful to command 2 m/s^2 acceleration to the right.
    BodyNed = 8,
    /// Offset in body NED frame. This makes sense if adding setpoints to the current flight path, to avoid an obstacle - e.g. useful to command 2 m/s^2 acceleration to the east.
    BodyOffsetNed = 9,
    /// Global (WGS84) coordinate frame with AGL altitude (at the waypoint coordinate). First value / x: latitude in degrees, second value / y: longitude in degrees, third value / z: positive altitude in meters with 0 being at ground level in terrain model.
    GlobalTerrainAlt = 10,
    /// Global (WGS84) coordinate frame (scaled) with AGL altitude (at the waypoint coordinate). First value / x: latitude in degrees*10e-7, second value / y: longitude in degrees*10e-7, third value / z: positive altitude in meters with 0 being at ground level in terrain model.
    GlobalTerrainAltInt = 11,
    /// Body fixed frame of reference, Z-down (x: Forward, y: Right, z: Down).
    BodyFrd = 12,
    /// MAV_FRAME_BODY_FLU - Body fixed frame of reference, Z-up (x: Forward, y: Left, z: Up).
    Reserved13 = 13,
    /// MAV_FRAME_MOCAP_NED - Odometry local coordinate frame of data given by a motion capture system, Z-down (x: North, y: East, z: Down).
    Reserved14 = 14,
    /// MAV_FRAME_MOCAP_ENU - Odometry local coordinate frame of data given by a motion capture system, Z-up (x: East, y: North, z: Up).
    Reserved15 = 15,
    /// MAV_FRAME_VISION_NED - Odometry local coordinate frame of data given by a vision estimation system, Z-down (x: North, y: East, z: Down).
    Reserved16 = 16,
    /// MAV_FRAME_VISION_ENU - Odometry local coordinate frame of data given by a vision estimation system, Z-up (x: East, y: North, z: Up).
    Reserved17 = 17,
    /// MAV_FRAME_ESTIM_NED - Odometry local coordinate frame of data given by an estimator running onboard the vehicle, Z-down (x: North, y: East, z: Down).
    Reserved18 = 18,
    /// MAV_FRAME_ESTIM_ENU - Odometry local coordinate frame of data given by an estimator running onboard the vehicle, Z-up (x: East, y: North, z: Up).
    Reserved19 = 19,
    /// Forward, Right, Down coordinate frame. This is a local frame with Z-down and arbitrary F/R alignment (i.e. not aligned with NED/earth frame).
    LocalFrd = 20,
    /// Forward, Left, Up coordinate frame. This is a local frame with Z-up and arbitrary F/L alignment (i.e. not aligned with ENU/earth frame).
    LocalFlu = 21,
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
pub enum MavlinkDataStreamType {
    MavlinkDataStreamImgJpeg = 0,
    MavlinkDataStreamImgBmp = 1,
    MavlinkDataStreamImgRaw8u = 2,
    MavlinkDataStreamImgRaw32u = 3,
    MavlinkDataStreamImgPgm = 4,
    MavlinkDataStreamImgPng = 5,
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
pub enum FenceAction {
    /// Disable fenced mode
    None = 0,
    /// Switched to guided mode to return point (fence point 0)
    Guided = 1,
    /// Report fence breach, but don't take action
    Report = 2,
    /// Switched to guided mode to return point (fence point 0) with manual throttle control
    GuidedThrPass = 3,
    /// Switch to RTL (return to launch) mode and head for the return point.
    Rtl = 4,
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
pub enum FenceBreach {
    /// No last fence breach
    None = 0,
    /// Breached minimum altitude
    Minalt = 1,
    /// Breached maximum altitude
    Maxalt = 2,
    /// Breached fence boundary
    Boundary = 3,
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
pub enum FenceMitigate {
    /// Actions being taken to mitigate/prevent fence breach
    /// Unknown
    Unknown = 0,
    /// No actions being taken
    None = 1,
    /// Velocity limiting active to prevent breach
    VelLimit = 2,
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
pub enum MavMountMode {
    /// Enumeration of possible mount operation modes. This message is used by obsolete/deprecated gimbal messages.
    /// Load and keep safe position (Roll,Pitch,Yaw) from permant memory and stop stabilization
    Retract = 0,
    /// Load and keep neutral position (Roll,Pitch,Yaw) from permanent memory.
    Neutral = 1,
    /// Load neutral position and start MAVLink Roll,Pitch,Yaw control with stabilization
    MavlinkTargeting = 2,
    /// Load neutral position and start RC Roll,Pitch,Yaw control with stabilization
    RcTargeting = 3,
    /// Load neutral position and start to point to Lat,Lon,Alt
    GpsPoint = 4,
    /// Gimbal tracks system with specified system ID
    SysidTarget = 5,
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
pub enum GimbalDeviceCapFlags {
    /// Gimbal device (low level) capability flags (bitmap)
    /// This enum is used to define bitmasks (work around protobuf limitations).
    /// Not used in MavLink, make protobuf happy.
    Undefined = 0,
    /// Gimbal device supports a retracted position
    /// bit 1
    HasRetract = 1,
    /// Gimbal device supports a horizontal, forward looking position, stabilized
    /// bit 2
    HasNeutral = 2,
    /// Gimbal device supports rotating around roll axis.
    /// bit 3
    HasRollAxis = 4,
    /// Gimbal device supports to follow a roll angle relative to the vehicle
    /// bit 4
    HasRollFollow = 8,
    /// Gimbal device supports locking to an roll angle (generally that's the default with roll stabilized)
    /// bit 5
    HasRollLock = 16,
    /// Gimbal device supports rotating around pitch axis.
    /// bit 6
    HasPitchAxis = 32,
    /// Gimbal device supports to follow a pitch angle relative to the vehicle
    /// bit 7
    HasPitchFollow = 64,
    /// Gimbal device supports locking to an pitch angle (generally that's the default with pitch stabilized)
    /// bit 8
    HasPitchLock = 128,
    /// Gimbal device supports rotating around yaw axis.
    /// bit 9
    HasYawAxis = 256,
    /// Gimbal device supports to follow a yaw angle relative to the vehicle (generally that's the default)
    /// bit 10
    HasYawFollow = 512,
    /// Gimbal device supports locking to an absolute heading (often this is an option available)
    /// bit 11
    HasYawLock = 1024,
    /// Gimbal device supports yawing/panning infinetely (e.g. using slip disk).
    /// bit 12
    SupportsInfiniteYaw = 2048,
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
pub enum GimbalManagerCapFlags {
    /// Gimbal manager high level capability flags (bitmap). The first 16 bits are identical to the GIMBAL_DEVICE_CAP_FLAGS which are identical with GIMBAL_DEVICE_FLAGS. However, the gimbal manager does not need to copy the flags from the gimbal but can also enhance the capabilities and thus add flags.
    /// This enum is used to define bitmasks (work around protobuf limitations).
    /// Not used in MavLink, make protobuf happy.
    Undefined = 0,
    /// Based on GIMBAL_DEVICE_CAP_FLAGS_HAS_RETRACT.
    /// bit 1
    HasRetract = 1,
    /// Based on GIMBAL_DEVICE_CAP_FLAGS_HAS_NEUTRAL.
    /// bit 2
    HasNeutral = 2,
    /// Based on GIMBAL_DEVICE_CAP_FLAGS_HAS_ROLL_AXIS.
    /// bit 3
    HasRollAxis = 4,
    /// Based on GIMBAL_DEVICE_CAP_FLAGS_HAS_ROLL_FOLLOW.
    /// bit 4
    HasRollFollow = 8,
    /// Based on GIMBAL_DEVICE_CAP_FLAGS_HAS_ROLL_LOCK.
    /// bit 5
    HasRollLock = 16,
    /// Based on GIMBAL_DEVICE_CAP_FLAGS_HAS_PITCH_AXIS.
    /// bit 6
    HasPitchAxis = 32,
    /// Based on GIMBAL_DEVICE_CAP_FLAGS_HAS_PITCH_FOLLOW.
    /// bit 7
    HasPitchFollow = 64,
    /// Based on GIMBAL_DEVICE_CAP_FLAGS_HAS_PITCH_LOCK.
    /// bit 8
    HasPitchLock = 128,
    /// Based on GIMBAL_DEVICE_CAP_FLAGS_HAS_YAW_AXIS.
    /// bit 9
    HasYawAxis = 256,
    /// Based on GIMBAL_DEVICE_CAP_FLAGS_HAS_YAW_FOLLOW.
    /// bit 10
    HasYawFollow = 512,
    /// Based on GIMBAL_DEVICE_CAP_FLAGS_HAS_YAW_LOCK.
    /// bit 11
    HasYawLock = 1024,
    /// Based on GIMBAL_DEVICE_CAP_FLAGS_SUPPORTS_INFINITE_YAW.
    /// bit 12
    SupportsInfiniteYaw = 2048,
    /// Gimbal manager supports to point to a local position.
    /// bit 17
    CanPointLocationLocal = 65536,
    /// Gimbal manager supports to point to a global latitude, longitude, altitude position.
    /// bit 18
    CanPointLocationGlobal = 131072,
    /// Gimbal manager supports tracking of a point on the camera.
    /// bit 19
    HasTrackingPoint = 262144,
    /// Gimbal manager supports tracking of a point on the camera.
    /// bit 20
    HasTrackingRectangle = 524288,
    /// Gimbal manager supports pitching and yawing at an angular velocity scaled by focal length (the more zoomed in, the slower the movement).
    /// bit 21
    SupportsFocalLengthScale = 1048576,
    /// Gimbal manager supports nudging when pointing to a location or tracking.
    /// bit 22
    SupportsNudging = 2097152,
    /// Gimbal manager supports overriding when pointing to a location or tracking.
    /// bit 23
    SupportsOverride = 4194304,
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
pub enum GimbalDeviceFlags {
    /// Flags for gimbal device (lower level) operation.
    /// This enum is used to define bitmasks (work around protobuf limitations).
    /// Not used in MavLink, make protobuf happy.
    Undefined = 0,
    /// Set to retracted safe position (no stabilization), takes presedence over all other flags.
    /// bit 1
    Retract = 1,
    /// Set to neutral position (horizontal, forward looking, with stabiliziation), takes presedence over all other flags except RETRACT.
    /// bit 2
    Neutral = 2,
    /// Lock roll angle to absolute angle relative to horizon (not relative to drone). This is generally the default with a stabilizing gimbal.
    /// bit 3
    RollLock = 4,
    /// Lock pitch angle to absolute angle relative to horizon (not relative to drone). This is generally the default.
    /// bit 4
    PitchLock = 8,
    /// Lock yaw angle to absolute angle relative to North (not relative to drone). If this flag is set, the quaternion is in the Earth frame with the x-axis pointing North (yaw absolute). If this flag is not set, the quaternion frame is in the Earth frame rotated so that the x-axis is pointing forward (yaw relative to vehicle).
    /// bit 5
    YawLock = 16,
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
pub enum GimbalManagerFlags {
    /// Flags for high level gimbal manager operation The first 16 bytes are identical to the GIMBAL_DEVICE_FLAGS.
    /// This enum is used to define bitmasks (work around protobuf limitations).
    /// Not used in MavLink, make protobuf happy.
    Undefined = 0,
    /// Based on GIMBAL_DEVICE_FLAGS_RETRACT
    /// bit 1
    Retract = 1,
    /// Based on GIMBAL_DEVICE_FLAGS_NEUTRAL
    /// bit 2
    Neutral = 2,
    /// Based on GIMBAL_DEVICE_FLAGS_ROLL_LOCK
    /// bit 3
    RollLock = 4,
    /// Based on GIMBAL_DEVICE_FLAGS_PITCH_LOCK
    /// bit 4
    PitchLock = 8,
    /// Based on GIMBAL_DEVICE_FLAGS_YAW_LOCK
    /// bit 5
    YawLock = 16,
    /// Scale angular velocity relative to focal length. This means the gimbal moves slower if it is zoomed in.
    /// bit 21
    AngularVelocityRelativeToFocalLength = 1048576,
    /// Interpret attitude control on top of pointing to a location or tracking. If this flag is set, the quaternion is relative to the existing tracking angle.
    /// bit 22
    Nudge = 2097152,
    /// Completely override pointing to a location or tracking. If this flag is set, the quaternion is (as usual) according to GIMBAL_MANAGER_FLAGS_YAW_LOCK.
    /// bit 23
    Override = 4194304,
    /// This flag can be set to give up control previously set using MAV_CMD_DO_GIMBAL_MANAGER_ATTITUDE. This flag must not be combined with other flags.
    /// bit 24
    None = 8388608,
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
pub enum GimbalDeviceErrorFlags {
    /// Gimbal device (low level) error flags (bitmap, 0 means no error)
    /// This enum is used to define bitmasks (work around protobuf limitations).
    /// Not used in MavLink, make protobuf happy.
    Undefined = 0,
    /// Gimbal device is limited by hardware roll limit.
    /// bit 1
    AtRollLimit = 1,
    /// Gimbal device is limited by hardware pitch limit.
    /// bit 2
    AtPitchLimit = 2,
    /// Gimbal device is limited by hardware yaw limit.
    /// bit 3
    AtYawLimit = 4,
    /// There is an error with the gimbal encoders.
    /// bit 4
    EncoderError = 8,
    /// There is an error with the gimbal power source.
    /// bit 5
    PowerError = 16,
    /// There is an error with the gimbal motor's.
    /// bit 6
    MotorError = 32,
    /// There is an error with the gimbal's software.
    /// bit 7
    SoftwareError = 64,
    /// There is an error with the gimbal's communication.
    /// bit 8
    CommsError = 128,
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
pub enum UavcanNodeHealth {
    /// Generalized UAVCAN node health
    /// The node is functioning properly.
    Ok = 0,
    /// A critical parameter went out of range or the node has encountered a minor failure.
    Warning = 1,
    /// The node has encountered a major failure.
    Error = 2,
    /// The node has suffered a fatal malfunction.
    Critical = 3,
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
pub enum UavcanNodeMode {
    /// Generalized UAVCAN node mode
    /// The node is performing its primary functions.
    Operational = 0,
    /// The node is initializing; this mode is entered immediately after startup.
    Initialization = 1,
    /// The node is under maintenance.
    Maintenance = 2,
    /// The node is in the process of updating its software.
    SoftwareUpdate = 3,
    /// The node is no longer available online.
    Offline = 7,
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
pub enum StorageStatus {
    /// Flags to indicate the status of camera storage.
    /// Storage is missing (no microSD card loaded for example.)
    Empty = 0,
    /// Storage present but unformatted.
    Unformatted = 1,
    /// Storage present and ready.
    Ready = 2,
    /// Camera does not supply storage status information. Capacity information in STORAGE_INFORMATION fields will be ignored.
    NotSupported = 3,
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
pub enum OrbitYawBehaviour {
    /// Yaw behaviour during orbit flight.
    /// Vehicle front points to the center (default).
    HoldFrontToCircleCenter = 0,
    /// Vehicle front holds heading when message received.
    HoldInitialHeading = 1,
    /// Yaw uncontrolled.
    Uncontrolled = 2,
    /// Vehicle front follows flight path (tangential to circle).
    HoldFrontTangentToCircle = 3,
    /// Yaw controlled by RC input.
    RcControlled = 4,
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
pub enum WifiConfigApResponse {
    /// Possible responses from a WIFI_CONFIG_AP message.
    /// Undefined response. Likely an indicative of a system that doesn't support this request.
    Undefined = 0,
    /// Changes accepted.
    Accepted = 1,
    /// Changes rejected.
    Rejected = 2,
    /// Invalid Mode.
    ModeError = 3,
    /// Invalid SSID.
    SsidError = 4,
    /// Invalid Password.
    PasswordError = 5,
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
pub enum CellularConfigResponse {
    /// Possible responses from a CELLULAR_CONFIG message.
    /// Changes accepted.
    Accepted = 0,
    /// Invalid APN.
    ApnError = 1,
    /// Invalid PIN.
    PinError = 2,
    /// Changes rejected.
    Rejected = 3,
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
pub enum WifiConfigApMode {
    /// WiFi Mode.
    /// WiFi mode is undefined.
    Undefined = 0,
    /// WiFi configured as an access point.
    Ap = 1,
    /// WiFi configured as a station connected to an existing local WiFi network.
    Station = 2,
    /// WiFi disabled.
    Disabled = 3,
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
pub enum CompMetadataType {
    /// Possible values for COMPONENT_INFORMATION.comp_metadata_type.
    /// Version information which also includes information on other optional supported COMP_METADATA_TYPE's. Must be supported. Only downloadable from vehicle.
    Version = 0,
    /// Parameter meta data.
    Parameter = 1,
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
    /// Commands to be executed by the MAV. They can be executed on user request, or as part of a mission script. If the action is used in a mission, the parameter mapping to the waypoint/mission message is as follows: Param 1, Param 2, Param 3, Param 4, X: Param 5, Y:Param 6, Z:Param 7. This command list is similar what ARINC 424 is for commercial aircraft: A data format how to interpret waypoint/mission data. NaN and INT32_MAX may be used in float/integer params (respectively) to indicate optional/default values (e.g. to use the component's current yaw or latitude rather than a specific value). See https://mavlink.io/en/guide/xml_schema.html#MAV_CMD for information about the structure of the MAV_CMD entries
    /// Not used in MavLink, make protobuf happy.
    Undefined = 0,
    /// Navigate to waypoint.
    NavWaypoint = 16,
    /// ***** START Params
    /// Hold time. (ignored by fixed wing, time to stay at waypoint for rotary wing)
    /// Acceptance radius (if the sphere with this radius is hit, the waypoint counts as reached)
    /// 0 to pass through the WP, if > 0 radius to pass by WP. Positive value for clockwise orbit, negative value for counter-clockwise orbit. Allows trajectory control.
    /// Desired yaw angle at waypoint (rotary wing). NaN to use the current system yaw heading mode (e.g. yaw towards next waypoint, yaw to home, etc.).
    /// Latitude
    /// Longitude
    /// Altitude
    /// ***** END Params
    /// Loiter around this waypoint an unlimited amount of time
    NavLoiterUnlim = 17,
    /// ***** START Params
    /// Empty
    /// Empty
    /// Loiter radius around waypoint for forward-only moving vehicles (not multicopters). If positive loiter clockwise, else counter-clockwise
    /// Desired yaw angle. NaN to use the current system yaw heading mode (e.g. yaw towards next waypoint, yaw to home, etc.).
    /// Latitude
    /// Longitude
    /// Altitude
    /// ***** END Params
    /// Loiter around this waypoint for X turns
    NavLoiterTurns = 18,
    /// ***** START Params
    /// Number of turns.
    /// Leave loiter circle only once heading towards the next waypoint (0 = False)
    /// Loiter radius around waypoint for forward-only moving vehicles (not multicopters). If positive loiter clockwise, else counter-clockwise
    /// Loiter circle exit location and/or path to next waypoint ("xtrack") for forward-only moving vehicles (not multicopters). 0 for the vehicle to converge towards the center xtrack when it leaves the loiter (the line between the centers of the current and next waypoint), 1 to converge to the direct line between the location that the vehicle exits the loiter radius and the next waypoint. Otherwise the angle (in degrees) between the tangent of the loiter circle and the center xtrack at which the vehicle must leave the loiter (and converge to the center xtrack). NaN to use the current system default xtrack behaviour.
    /// Latitude
    /// Longitude
    /// Altitude
    /// ***** END Params
    /// Loiter at the specified latitude, longitude and altitude for a certain amount of time. Multicopter vehicles stop at the point (within a vehicle-specific acceptance radius). Forward-only moving vehicles (e.g. fixed-wing) circle the point with the specified radius/direction. If the Heading Required parameter (2) is non-zero forward moving aircraft will only leave the loiter circle once heading towards the next waypoint.
    NavLoiterTime = 19,
    /// ***** START Params
    /// Loiter time (only starts once Lat, Lon and Alt is reached).
    /// Leave loiter circle only once heading towards the next waypoint (0 = False)
    /// Loiter radius around waypoint for forward-only moving vehicles (not multicopters). If positive loiter clockwise, else counter-clockwise.
    /// Loiter circle exit location and/or path to next waypoint ("xtrack") for forward-only moving vehicles (not multicopters). 0 for the vehicle to converge towards the center xtrack when it leaves the loiter (the line between the centers of the current and next waypoint), 1 to converge to the direct line between the location that the vehicle exits the loiter radius and the next waypoint. Otherwise the angle (in degrees) between the tangent of the loiter circle and the center xtrack at which the vehicle must leave the loiter (and converge to the center xtrack). NaN to use the current system default xtrack behaviour.
    /// Latitude
    /// Longitude
    /// Altitude
    /// ***** END Params
    /// Return to launch location
    NavReturnToLaunch = 20,
    /// ***** START Params
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// ***** END Params
    /// Land at location.
    NavLand = 21,
    /// ***** START Params
    /// Minimum target altitude if landing is aborted (0 = undefined/use system default).
    /// Precision land mode.
    /// Empty
    /// Desired yaw angle. NaN to use the current system yaw heading mode (e.g. yaw towards next waypoint, yaw to home, etc.).
    /// Latitude.
    /// Longitude.
    /// Landing altitude (ground level in current frame).
    /// ***** END Params
    /// Takeoff from ground / hand. Vehicles that support multiple takeoff modes (e.g. VTOL quadplane) should take off using the currently configured mode.
    NavTakeoff = 22,
    /// ***** START Params
    /// Minimum pitch (if airspeed sensor present), desired pitch without sensor
    /// Empty
    /// Empty
    /// Yaw angle (if magnetometer present), ignored without magnetometer. NaN to use the current system yaw heading mode (e.g. yaw towards next waypoint, yaw to home, etc.).
    /// Latitude
    /// Longitude
    /// Altitude
    /// ***** END Params
    /// Land at local position (local frame only)
    NavLandLocal = 23,
    /// ***** START Params
    /// Landing target number (if available)
    /// Maximum accepted offset from desired landing position - computed magnitude from spherical coordinates: d = sqrt(x^2 + y^2 + z^2), which gives the maximum accepted distance between the desired landing position and the position where the vehicle is about to land
    /// Landing descend rate
    /// Desired yaw angle
    /// Y-axis position
    /// X-axis position
    /// Z-axis / ground level position
    /// ***** END Params
    /// Takeoff from local position (local frame only)
    NavTakeoffLocal = 24,
    /// ***** START Params
    /// Minimum pitch (if airspeed sensor present), desired pitch without sensor
    /// Empty
    /// Takeoff ascend rate
    /// Yaw angle (if magnetometer or another yaw estimation source present), ignored without one of these
    /// Y-axis position
    /// X-axis position
    /// Z-axis position
    /// ***** END Params
    /// Vehicle following, i.e. this waypoint represents the position of a moving vehicle
    NavFollow = 25,
    /// ***** START Params
    /// Following logic to use (e.g. loitering or sinusoidal following) - depends on specific autopilot implementation
    /// Ground speed of vehicle to be followed
    /// Radius around waypoint. If positive loiter clockwise, else counter-clockwise
    /// Desired yaw angle.
    /// Latitude
    /// Longitude
    /// Altitude
    /// ***** END Params
    /// Continue on the current course and climb/descend to specified altitude.  When the altitude is reached continue to the next command (i.e., don't proceed to the next command until the desired altitude is reached.
    NavContinueAndChangeAlt = 30,
    /// ***** START Params
    /// Climb or Descend (0 = Neutral, command completes when within 5m of this command's altitude, 1 = Climbing, command completes when at or above this command's altitude, 2 = Descending, command completes when at or below this command's altitude.
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Desired altitude
    /// ***** END Params
    /// Begin loiter at the specified Latitude and Longitude.  If Lat=Lon=0, then loiter at the current position.  Don't consider the navigation command complete (don't leave loiter) until the altitude has been reached. Additionally, if the Heading Required parameter is non-zero the aircraft will not leave the loiter until heading toward the next waypoint.
    NavLoiterToAlt = 31,
    /// ***** START Params
    /// Leave loiter circle only once heading towards the next waypoint (0 = False)
    /// Loiter radius around waypoint for forward-only moving vehicles (not multicopters). If positive loiter clockwise, negative counter-clockwise, 0 means no change to standard loiter.
    /// Empty
    /// Loiter circle exit location and/or path to next waypoint ("xtrack") for forward-only moving vehicles (not multicopters). 0 for the vehicle to converge towards the center xtrack when it leaves the loiter (the line between the centers of the current and next waypoint), 1 to converge to the direct line between the location that the vehicle exits the loiter radius and the next waypoint. Otherwise the angle (in degrees) between the tangent of the loiter circle and the center xtrack at which the vehicle must leave the loiter (and converge to the center xtrack). NaN to use the current system default xtrack behaviour.
    /// Latitude
    /// Longitude
    /// Altitude
    /// ***** END Params
    /// Begin following a target
    DoFollow = 32,
    /// ***** START Params
    /// System ID (of the FOLLOW_TARGET beacon). Send 0 to disable follow-me and return to the default position hold mode.
    /// Reserved
    /// Reserved
    /// Altitude mode: 0: Keep current altitude, 1: keep altitude difference to target, 2: go to a fixed altitude above home.
    /// Altitude above home. (used if mode=2)
    /// Reserved
    /// Time to land in which the MAV should go to the default position hold mode after a message RX timeout.
    /// ***** END Params
    /// Reposition the MAV after a follow target command has been sent
    DoFollowReposition = 33,
    /// ***** START Params
    /// Camera q1 (where 0 is on the ray from the camera to the tracking device)
    /// Camera q2
    /// Camera q3
    /// Camera q4
    /// altitude offset from target
    /// X offset from target
    /// Y offset from target
    /// ***** END Params
    /// Start orbiting on the circumference of a circle defined by the parameters. Setting any value NaN results in using defaults.
    DoOrbit = 34,
    /// ***** START Params
    /// Radius of the circle. positive: Orbit clockwise. negative: Orbit counter-clockwise.
    /// Tangential Velocity. NaN: Vehicle configuration default.
    /// Yaw behavior of the vehicle.
    /// Reserved (e.g. for dynamic center beacon options)
    /// Center point latitude (if no MAV_FRAME specified) / X coordinate according to MAV_FRAME. NaN: Use current vehicle position or current center if already orbiting.
    /// Center point longitude (if no MAV_FRAME specified) / Y coordinate according to MAV_FRAME. NaN: Use current vehicle position or current center if already orbiting.
    /// Center point altitude (MSL) (if no MAV_FRAME specified) / Z coordinate according to MAV_FRAME. NaN: Use current vehicle position or current center if already orbiting.
    /// ***** END Params
    /// Sets the region of interest (ROI) for a sensor set or the vehicle itself. This can then be used by the vehicle's control system to control the vehicle attitude and the attitude of various sensors such as cameras.
    NavRoi = 80,
    /// ***** START Params
    /// Region of interest mode.
    /// Waypoint index/ target ID. (see MAV_ROI enum)
    /// ROI index (allows a vehicle to manage multiple ROI's)
    /// Empty
    /// x the location of the fixed ROI (see MAV_FRAME)
    /// y
    /// z
    /// ***** END Params
    /// Control autonomous path planning on the MAV.
    NavPathplanning = 81,
    /// ***** START Params
    /// 0: Disable local obstacle avoidance / local path planning (without resetting map), 1: Enable local path planning, 2: Enable and reset local path planning
    /// 0: Disable full path planning (without resetting map), 1: Enable, 2: Enable and reset map/occupancy grid, 3: Enable and reset planned route, but not occupancy grid
    /// Empty
    /// Yaw angle at goal
    /// Latitude/X of goal
    /// Longitude/Y of goal
    /// Altitude/Z of goal
    /// ***** END Params
    /// Navigate to waypoint using a spline path.
    NavSplineWaypoint = 82,
    /// ***** START Params
    /// Hold time. (ignored by fixed wing, time to stay at waypoint for rotary wing)
    /// Empty
    /// Empty
    /// Empty
    /// Latitude/X of goal
    /// Longitude/Y of goal
    /// Altitude/Z of goal
    /// ***** END Params
    /// Takeoff from ground using VTOL mode, and transition to forward flight with specified heading. The command should be ignored by vehicles that dont support both VTOL and fixed-wing flight (multicopters, boats,etc.).
    NavVtolTakeoff = 84,
    /// ***** START Params
    /// Empty
    /// Front transition heading.
    /// Empty
    /// Yaw angle. NaN to use the current system yaw heading mode (e.g. yaw towards next waypoint, yaw to home, etc.).
    /// Latitude
    /// Longitude
    /// Altitude
    /// ***** END Params
    /// Land using VTOL mode
    NavVtolLand = 85,
    /// ***** START Params
    /// Empty
    /// Empty
    /// Approach altitude (with the same reference as the Altitude field). NaN if unspecified.
    /// Yaw angle. NaN to use the current system yaw heading mode (e.g. yaw towards next waypoint, yaw to home, etc.).
    /// Latitude
    /// Longitude
    /// Altitude (ground level)
    /// ***** END Params
    /// hand control over to an external controller
    NavGuidedEnable = 92,
    /// ***** START Params
    /// On / Off (> 0.5f on)
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// ***** END Params
    /// Delay the next navigation command a number of seconds or until a specified time
    NavDelay = 93,
    /// ***** START Params
    /// Delay (-1 to enable time-of-day fields)
    /// hour (24h format, UTC, -1 to ignore)
    /// minute (24h format, UTC, -1 to ignore)
    /// second (24h format, UTC)
    /// Empty
    /// Empty
    /// Empty
    /// ***** END Params
    /// Descend and place payload. Vehicle moves to specified location, descends until it detects a hanging payload has reached the ground, and then releases the payload. If ground is not detected before the reaching the maximum descent value (param1), the command will complete without releasing the payload.
    NavPayloadPlace = 94,
    /// ***** START Params
    /// Maximum distance to descend.
    /// Empty
    /// Empty
    /// Empty
    /// Latitude
    /// Longitude
    /// Altitude
    /// ***** END Params
    /// NOP - This command is only used to mark the upper limit of the NAV/ACTION commands in the enumeration
    NavLast = 95,
    /// ***** START Params
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// ***** END Params
    /// Delay mission state machine.
    ConditionDelay = 112,
    /// ***** START Params
    /// Delay
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// ***** END Params
    /// Ascend/descend to target altitude at specified rate. Delay mission state machine until desired altitude reached.
    ConditionChangeAlt = 113,
    /// ***** START Params
    /// Descent / Ascend rate.
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Target Altitude
    /// ***** END Params
    /// Delay mission state machine until within desired distance of next NAV point.
    ConditionDistance = 114,
    /// ***** START Params
    /// Distance.
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// ***** END Params
    /// Reach a certain target angle.
    ConditionYaw = 115,
    /// ***** START Params
    /// target angle, 0 is north
    /// angular speed
    /// direction: -1: counter clockwise, 1: clockwise
    /// 0: absolute angle, 1: relative offset
    /// Empty
    /// Empty
    /// Empty
    /// ***** END Params
    /// NOP - This command is only used to mark the upper limit of the CONDITION commands in the enumeration
    ConditionLast = 159,
    /// ***** START Params
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// ***** END Params
    /// Set system mode.
    DoSetMode = 176,
    /// ***** START Params
    /// Mode
    /// Custom mode - this is system specific, please refer to the individual autopilot specifications for details.
    /// Custom sub mode - this is system specific, please refer to the individual autopilot specifications for details.
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// ***** END Params
    /// Jump to the desired command in the mission list.  Repeat this action only the specified number of times
    DoJump = 177,
    /// ***** START Params
    /// Sequence number
    /// Repeat count
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// ***** END Params
    /// Change speed and/or throttle set points.
    DoChangeSpeed = 178,
    /// ***** START Params
    /// Speed type (0=Airspeed, 1=Ground Speed, 2=Climb Speed, 3=Descent Speed)
    /// Speed (-1 indicates no change)
    /// Throttle (-1 indicates no change)
    /// 0: absolute, 1: relative
    /// Empty
    /// Empty
    /// Empty
    /// ***** END Params
    /// Changes the home location either to the current location or a specified location.
    DoSetHome = 179,
    /// ***** START Params
    /// Use current (1=use current location, 0=use specified location)
    /// Empty
    /// Empty
    /// Yaw angle. NaN to use default heading
    /// Latitude
    /// Longitude
    /// Altitude
    /// ***** END Params
    /// Set a system parameter.  Caution!  Use of this command requires knowledge of the numeric enumeration value of the parameter.
    DoSetParameter = 180,
    /// ***** START Params
    /// Parameter number
    /// Parameter value
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// ***** END Params
    /// Set a relay to a condition.
    DoSetRelay = 181,
    /// ***** START Params
    /// Relay instance number.
    /// Setting. (1=on, 0=off, others possible depending on system hardware)
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// ***** END Params
    /// Cycle a relay on and off for a desired number of cycles with a desired period.
    DoRepeatRelay = 182,
    /// ***** START Params
    /// Relay instance number.
    /// Cycle count.
    /// Cycle time.
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// ***** END Params
    /// Set a servo to a desired PWM value.
    DoSetServo = 183,
    /// ***** START Params
    /// Servo instance number.
    /// Pulse Width Modulation.
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// ***** END Params
    /// Cycle a between its nominal setting and a desired PWM for a desired number of cycles with a desired period.
    DoRepeatServo = 184,
    /// ***** START Params
    /// Servo instance number.
    /// Pulse Width Modulation.
    /// Cycle count.
    /// Cycle time.
    /// Empty
    /// Empty
    /// Empty
    /// ***** END Params
    /// Terminate flight immediately
    DoFlighttermination = 185,
    /// ***** START Params
    /// Flight termination activated if > 0.5
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// ***** END Params
    /// Change altitude set point.
    DoChangeAltitude = 186,
    /// ***** START Params
    /// Altitude
    /// Frame of new altitude.
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// ***** END Params
    /// Sets actuators (e.g. servos) to a desired value. The actuator numbers are mapped to specific outputs (e.g. on any MAIN or AUX PWM or UAVCAN) using a flight-stack specific mechanism (i.e. a parameter).
    DoSetActuator = 187,
    /// ***** START Params
    /// Actuator 1 value, scaled from [-1 to 1]. NaN to ignore.
    /// Actuator 2 value, scaled from [-1 to 1]. NaN to ignore.
    /// Actuator 3 value, scaled from [-1 to 1]. NaN to ignore.
    /// Actuator 4 value, scaled from [-1 to 1]. NaN to ignore.
    /// Actuator 5 value, scaled from [-1 to 1]. NaN to ignore.
    /// Actuator 6 value, scaled from [-1 to 1]. NaN to ignore.
    /// Index of actuator set (i.e if set to 1, Actuator 1 becomes Actuator 7)
    /// ***** END Params
    /// Mission command to perform a landing. This is used as a marker in a mission to tell the autopilot where a sequence of mission items that represents a landing starts. It may also be sent via a COMMAND_LONG to trigger a landing, in which case the nearest (geographically) landing sequence in the mission will be used. The Latitude/Longitude is optional, and may be set to 0 if not needed. If specified then it will be used to help find the closest landing sequence.
    DoLandStart = 189,
    /// ***** START Params
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Latitude
    /// Longitude
    /// Empty
    /// ***** END Params
    /// Mission command to perform a landing from a rally point.
    DoRallyLand = 190,
    /// ***** START Params
    /// Break altitude
    /// Landing speed
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// ***** END Params
    /// Mission command to safely abort an autonomous landing.
    DoGoAround = 191,
    /// ***** START Params
    /// Altitude
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// ***** END Params
    /// Reposition the vehicle to a specific WGS84 global position.
    DoReposition = 192,
    /// ***** START Params
    /// Ground speed, less than 0 (-1) for default
    /// Bitmask of option flags.
    /// Reserved
    /// Yaw heading. NaN to use the current system yaw heading mode (e.g. yaw towards next waypoint, yaw to home, etc.). For planes indicates loiter direction (0: clockwise, 1: counter clockwise)
    /// Latitude
    /// Longitude
    /// Altitude
    /// ***** END Params
    /// If in a GPS controlled position mode, hold the current position or continue.
    DoPauseContinue = 193,
    /// ***** START Params
    /// 0: Pause current mission or reposition command, hold current position. 1: Continue mission. A VTOL capable vehicle should enter hover mode (multicopter and VTOL planes). A plane should loiter with the default loiter radius.
    /// Reserved
    /// Reserved
    /// Reserved
    /// Reserved
    /// Reserved
    /// Reserved
    /// ***** END Params
    /// Set moving direction to forward or reverse.
    DoSetReverse = 194,
    /// ***** START Params
    /// Direction (0=Forward, 1=Reverse)
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// ***** END Params
    /// Sets the region of interest (ROI) to a location. This can then be used by the vehicle's control system to control the vehicle attitude and the attitude of various sensors such as cameras. This command can be sent to a gimbal manager but not to a gimbal device. A gimbal is not to react to this message.
    DoSetRoiLocation = 195,
    /// ***** START Params
    /// Component ID of gimbal device to address (or 1-6 for non-MAVLink gimbal), 0 for all gimbal device components. (Send command multiple times for more than one but not all gimbals.)
    /// Empty
    /// Empty
    /// Empty
    /// Latitude of ROI location
    /// Longitude of ROI location
    /// Altitude of ROI location
    /// ***** END Params
    /// Sets the region of interest (ROI) to be toward next waypoint, with optional pitch/roll/yaw offset. This can then be used by the vehicle's control system to control the vehicle attitude and the attitude of various sensors such as cameras. This command can be sent to a gimbal manager but not to a gimbal device. A gimbal device is not to react to this message.
    DoSetRoiWpnextOffset = 196,
    /// ***** START Params
    /// Component ID of gimbal device to address (or 1-6 for non-MAVLink gimbal), 0 for all gimbal device components. (Send command multiple times for more than one but not all gimbals.)
    /// Empty
    /// Empty
    /// Empty
    /// Pitch offset from next waypoint, positive tilting up
    /// roll offset from next waypoint, positive banking to the right
    /// yaw offset from next waypoint, positive panning to the right
    /// ***** END Params
    /// Cancels any previous ROI command returning the vehicle/sensors to default flight characteristics. This can then be used by the vehicle's control system to control the vehicle attitude and the attitude of various sensors such as cameras. This command can be sent to a gimbal manager but not to a gimbal device. A gimbal device is not to react to this message. After this command the gimbal manager should go back to manual input if available, and otherwise assume a neutral position.
    DoSetRoiNone = 197,
    /// ***** START Params
    /// Component ID of gimbal device to address (or 1-6 for non-MAVLink gimbal), 0 for all gimbal device components. (Send command multiple times for more than one but not all gimbals.)
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// ***** END Params
    /// Mount tracks system with specified system ID. Determination of target vehicle position may be done with GLOBAL_POSITION_INT or any other means. This command can be sent to a gimbal manager but not to a gimbal device. A gimbal device is not to react to this message.
    DoSetRoiSysid = 198,
    /// ***** START Params
    /// System ID
    /// Component ID of gimbal device to address (or 1-6 for non-MAVLink gimbal), 0 for all gimbal device components. (Send command multiple times for more than one but not all gimbals.)
    /// ***** END Params
    /// Control onboard camera system.
    DoControlVideo = 200,
    /// ***** START Params
    /// Camera ID (-1 for all)
    /// Transmission: 0: disabled, 1: enabled compressed, 2: enabled raw
    /// Transmission mode: 0: video stream, >0: single images every n seconds
    /// Recording: 0: disabled, 1: enabled compressed, 2: enabled raw
    /// Empty
    /// Empty
    /// Empty
    /// ***** END Params
    /// Sets the region of interest (ROI) for a sensor set or the vehicle itself. This can then be used by the vehicle's control system to control the vehicle attitude and the attitude of various sensors such as cameras.
    DoSetRoi = 201,
    /// ***** START Params
    /// Region of interest mode.
    /// Waypoint index/ target ID (depends on param 1).
    /// Region of interest index. (allows a vehicle to manage multiple ROI's)
    /// Empty
    /// MAV_ROI_WPNEXT: pitch offset from next waypoint, MAV_ROI_LOCATION: latitude
    /// MAV_ROI_WPNEXT: roll offset from next waypoint, MAV_ROI_LOCATION: longitude
    /// MAV_ROI_WPNEXT: yaw offset from next waypoint, MAV_ROI_LOCATION: altitude
    /// ***** END Params
    /// Configure digital camera. This is a fallback message for systems that have not yet implemented PARAM_EXT_XXX messages and camera definition files (see https://mavlink.io/en/services/camera_def.html ).
    DoDigicamConfigure = 202,
    /// ***** START Params
    /// Modes: P, TV, AV, M, Etc.
    /// Shutter speed: Divisor number for one second.
    /// Aperture: F stop number.
    /// ISO number e.g. 80, 100, 200, Etc.
    /// Exposure type enumerator.
    /// Command Identity.
    /// Main engine cut-off time before camera trigger. (0 means no cut-off)
    /// ***** END Params
    /// Control digital camera. This is a fallback message for systems that have not yet implemented PARAM_EXT_XXX messages and camera definition files (see https://mavlink.io/en/services/camera_def.html ).
    DoDigicamControl = 203,
    /// ***** START Params
    /// Session control e.g. show/hide lens
    /// Zoom's absolute position
    /// Zooming step value to offset zoom from the current position
    /// Focus Locking, Unlocking or Re-locking
    /// Shooting Command
    /// Command Identity
    /// Test shot identifier. If set to 1, image will only be captured, but not counted towards internal frame count.
    /// ***** END Params
    /// Mission command to configure a camera or antenna mount
    DoMountConfigure = 204,
    /// ***** START Params
    /// Mount operation mode
    /// stabilize roll? (1 = yes, 0 = no)
    /// stabilize pitch? (1 = yes, 0 = no)
    /// stabilize yaw? (1 = yes, 0 = no)
    /// roll input (0 = angle body frame, 1 = angular rate, 2 = angle absolute frame)
    /// pitch input (0 = angle body frame, 1 = angular rate, 2 = angle absolute frame)
    /// yaw input (0 = angle body frame, 1 = angular rate, 2 = angle absolute frame)
    /// ***** END Params
    /// Mission command to control a camera or antenna mount
    DoMountControl = 205,
    /// ***** START Params
    /// pitch depending on mount mode (degrees or degrees/second depending on pitch input).
    /// roll depending on mount mode (degrees or degrees/second depending on roll input).
    /// yaw depending on mount mode (degrees or degrees/second depending on yaw input).
    /// altitude depending on mount mode.
    /// latitude, set if appropriate mount mode.
    /// longitude, set if appropriate mount mode.
    /// Mount mode.
    /// ***** END Params
    /// Mission command to set camera trigger distance for this flight. The camera is triggered each time this distance is exceeded. This command can also be used to set the shutter integration time for the camera.
    DoSetCamTriggDist = 206,
    /// ***** START Params
    /// Camera trigger distance. 0 to stop triggering.
    /// Camera shutter integration time. -1 or 0 to ignore
    /// Trigger camera once immediately. (0 = no trigger, 1 = trigger)
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// ***** END Params
    /// Mission command to enable the geofence
    DoFenceEnable = 207,
    /// ***** START Params
    /// enable? (0=disable, 1=enable, 2=disable_floor_only)
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// ***** END Params
    /// Mission item/command to release a parachute or enable/disable auto release.
    DoParachute = 208,
    /// ***** START Params
    /// Action
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// ***** END Params
    /// Mission command to perform motor test.
    DoMotorTest = 209,
    /// ***** START Params
    /// Motor instance number. (from 1 to max number of motors on the vehicle)
    /// Throttle type.
    /// Throttle.
    /// Timeout.
    /// Motor count. (number of motors to test to test in sequence, waiting for the timeout above between them; 0=1 motor, 1=1 motor, 2=2 motors...)
    /// Motor test order.
    /// Empty
    /// ***** END Params
    /// Change to/from inverted flight.
    DoInvertedFlight = 210,
    /// ***** START Params
    /// Inverted flight. (0=normal, 1=inverted)
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// ***** END Params
    /// Sets a desired vehicle turn angle and speed change.
    NavSetYawSpeed = 213,
    /// ***** START Params
    /// Yaw angle to adjust steering by.
    /// Speed.
    /// Final angle. (0=absolute, 1=relative)
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// ***** END Params
    /// Mission command to set camera trigger interval for this flight. If triggering is enabled, the camera is triggered each time this interval expires. This command can also be used to set the shutter integration time for the camera.
    DoSetCamTriggInterval = 214,
    /// ***** START Params
    /// Camera trigger cycle time. -1 or 0 to ignore.
    /// Camera shutter integration time. Should be less than trigger cycle time. -1 or 0 to ignore.
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// ***** END Params
    /// Mission command to control a camera or antenna mount, using a quaternion as reference.
    DoMountControlQuat = 220,
    /// ***** START Params
    /// quaternion param q1, w (1 in null-rotation)
    /// quaternion param q2, x (0 in null-rotation)
    /// quaternion param q3, y (0 in null-rotation)
    /// quaternion param q4, z (0 in null-rotation)
    /// Empty
    /// Empty
    /// Empty
    /// ***** END Params
    /// set id of master controller
    DoGuidedMaster = 221,
    /// ***** START Params
    /// System ID
    /// Component ID
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// ***** END Params
    /// Set limits for external control
    DoGuidedLimits = 222,
    /// ***** START Params
    /// Timeout - maximum time that external controller will be allowed to control vehicle. 0 means no timeout.
    /// Altitude (MSL) min - if vehicle moves below this alt, the command will be aborted and the mission will continue. 0 means no lower altitude limit.
    /// Altitude (MSL) max - if vehicle moves above this alt, the command will be aborted and the mission will continue. 0 means no upper altitude limit.
    /// Horizontal move limit - if vehicle moves more than this distance from its location at the moment the command was executed, the command will be aborted and the mission will continue. 0 means no horizontal move limit.
    /// Empty
    /// Empty
    /// Empty
    /// ***** END Params
    /// Control vehicle engine. This is interpreted by the vehicles engine controller to change the target engine state. It is intended for vehicles with internal combustion engines
    DoEngineControl = 223,
    /// ***** START Params
    /// 0: Stop engine, 1:Start Engine
    /// 0: Warm start, 1:Cold start. Controls use of choke where applicable
    /// Height delay. This is for commanding engine start only after the vehicle has gained the specified height. Used in VTOL vehicles during takeoff to start engine after the aircraft is off the ground. Zero for no delay.
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// ***** END Params
    /// Set the mission item with sequence number seq as current item. This means that the MAV will continue to this mission item on the shortest path (not following the mission items in-between).
    DoSetMissionCurrent = 224,
    /// ***** START Params
    /// Mission sequence value to set
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// ***** END Params
    /// NOP - This command is only used to mark the upper limit of the DO commands in the enumeration
    DoLast = 240,
    /// ***** START Params
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// ***** END Params
    /// Trigger calibration. This command will be only accepted if in pre-flight mode. Except for Temperature Calibration, only one sensor should be set in a single message and all others should be zero.
    PreflightCalibration = 241,
    /// ***** START Params
    /// 1: gyro calibration, 3: gyro temperature calibration
    /// 1: magnetometer calibration
    /// 1: ground pressure calibration
    /// 1: radio RC calibration, 2: RC trim calibration
    /// 1: accelerometer calibration, 2: board level calibration, 3: accelerometer temperature calibration, 4: simple accelerometer calibration
    /// 1: APM: compass/motor interference calibration (PX4: airspeed calibration, deprecated), 2: airspeed calibration
    /// 1: ESC calibration, 3: barometer temperature calibration
    /// ***** END Params
    /// Set sensor offsets. This command will be only accepted if in pre-flight mode.
    PreflightSetSensorOffsets = 242,
    /// ***** START Params
    /// Sensor to adjust the offsets for: 0: gyros, 1: accelerometer, 2: magnetometer, 3: barometer, 4: optical flow, 5: second magnetometer, 6: third magnetometer
    /// X axis offset (or generic dimension 1), in the sensor's raw units
    /// Y axis offset (or generic dimension 2), in the sensor's raw units
    /// Z axis offset (or generic dimension 3), in the sensor's raw units
    /// Generic dimension 4, in the sensor's raw units
    /// Generic dimension 5, in the sensor's raw units
    /// Generic dimension 6, in the sensor's raw units
    /// ***** END Params
    /// Trigger UAVCAN config. This command will be only accepted if in pre-flight mode.
    PreflightUavcan = 243,
    /// ***** START Params
    /// 1: Trigger actuator ID assignment and direction mapping.
    /// Reserved
    /// Reserved
    /// Reserved
    /// Reserved
    /// Reserved
    /// Reserved
    /// ***** END Params
    /// Request storage of different parameter values and logs. This command will be only accepted if in pre-flight mode.
    PreflightStorage = 245,
    /// ***** START Params
    /// Parameter storage: 0: READ FROM FLASH/EEPROM, 1: WRITE CURRENT TO FLASH/EEPROM, 2: Reset to defaults
    /// Mission storage: 0: READ FROM FLASH/EEPROM, 1: WRITE CURRENT TO FLASH/EEPROM, 2: Reset to defaults
    /// Onboard logging: 0: Ignore, 1: Start default rate logging, -1: Stop logging, > 1: logging rate (e.g. set to 1000 for 1000 Hz logging)
    /// Reserved
    /// Empty
    /// Empty
    /// Empty
    /// ***** END Params
    /// Request the reboot or shutdown of system components.
    PreflightRebootShutdown = 246,
    /// ***** START Params
    /// 0: Do nothing for autopilot, 1: Reboot autopilot, 2: Shutdown autopilot, 3: Reboot autopilot and keep it in the bootloader until upgraded.
    /// 0: Do nothing for onboard computer, 1: Reboot onboard computer, 2: Shutdown onboard computer, 3: Reboot onboard computer and keep it in the bootloader until upgraded.
    /// WIP: 0: Do nothing for camera, 1: Reboot onboard camera, 2: Shutdown onboard camera, 3: Reboot onboard camera and keep it in the bootloader until upgraded
    /// WIP: 0: Do nothing for mount (e.g. gimbal), 1: Reboot mount, 2: Shutdown mount, 3: Reboot mount and keep it in the bootloader until upgraded
    /// Reserved (set to 0)
    /// Reserved (set to 0)
    /// WIP: ID (e.g. camera ID -1 for all IDs)
    /// ***** END Params
    /// Request a target system to start an upgrade of one (or all) of its components. For example, the command might be sent to a companion computer to cause it to upgrade a connected flight controller. The system doing the upgrade will report progress using the normal command protocol sequence for a long running operation. Command protocol information: https://mavlink.io/en/services/command.html.
    DoUpgrade = 247,
    /// ***** START Params
    /// Component id of the component to be upgraded. If set to 0, all components should be upgraded.
    /// 0: Do not reboot component after the action is executed, 1: Reboot component after the action is executed.
    /// Reserved
    /// Reserved
    /// Reserved
    /// Reserved
    /// WIP: upgrade progress report rate (can be used for more granular control).
    /// ***** END Params
    /// Override current mission with command to pause mission, pause mission and move to position, continue/resume mission. When param 1 indicates that the mission is paused (MAV_GOTO_DO_HOLD), param 2 defines whether it holds in place or moves to another position.
    OverrideGoto = 252,
    /// ***** START Params
    /// MAV_GOTO_DO_HOLD: pause mission and either hold or move to specified position (depending on param2), MAV_GOTO_DO_CONTINUE: resume mission.
    /// MAV_GOTO_HOLD_AT_CURRENT_POSITION: hold at current position, MAV_GOTO_HOLD_AT_SPECIFIED_POSITION: hold at specified position.
    /// Coordinate frame of hold point.
    /// Desired yaw angle.
    /// Latitude/X position.
    /// Longitude/Y position.
    /// Altitude/Z position.
    /// ***** END Params
    /// start running a mission
    MissionStart = 300,
    /// ***** START Params
    /// first_item: the first mission item to run
    /// last_item:  the last mission item to run (after this item is run, the mission ends)
    /// ***** END Params
    /// Arms / Disarms a component
    ComponentArmDisarm = 400,
    /// ***** START Params
    /// 0: disarm, 1: arm
    /// 0: arm-disarm unless prevented by safety checks (i.e. when landed), 21196: force arming/disarming (e.g. allow arming to override preflight checks and disarming in flight)
    /// ***** END Params
    /// Turns illuminators ON/OFF. An illuminator is a light source that is used for lighting up dark areas external to the sytstem: e.g. a torch or searchlight (as opposed to a light source for illuminating the system itself, e.g. an indicator light).
    IlluminatorOnOff = 405,
    /// ***** START Params
    /// 0: Illuminators OFF, 1: Illuminators ON
    /// ***** END Params
    /// Request the home position from the vehicle.
    GetHomePosition = 410,
    /// ***** START Params
    /// Reserved
    /// Reserved
    /// Reserved
    /// Reserved
    /// Reserved
    /// Reserved
    /// Reserved
    /// ***** END Params
    /// Inject artificial failure for testing purposes. Note that autopilots should implement an additional protection before accepting this command such as a specific param setting.
    InjectFailure = 420,
    /// ***** START Params
    /// The unit which is affected by the failure.
    /// The type how the failure manifests itself.
    /// Instance affected by failure (0 to signal all).
    /// ***** END Params
    /// Starts receiver pairing.
    StartRxPair = 500,
    /// ***** START Params
    /// 0:Spektrum.
    /// RC type.
    /// ***** END Params
    /// Request the interval between messages for a particular MAVLink message ID. The receiver should ACK the command and then emit its response in a MESSAGE_INTERVAL message.
    GetMessageInterval = 510,
    /// ***** START Params
    /// The MAVLink message ID
    /// ***** END Params
    /// Set the interval between messages for a particular MAVLink message ID. This interface replaces REQUEST_DATA_STREAM.
    SetMessageInterval = 511,
    /// ***** START Params
    /// The MAVLink message ID
    /// The interval between two messages. Set to -1 to disable and 0 to request default rate.
    /// The use of this parameter (if any), must be defined in the requested message. By default assumed not used (0).
    /// The use of this parameter (if any), must be defined in the requested message. By default assumed not used (0).
    /// The use of this parameter (if any), must be defined in the requested message. By default assumed not used (0).
    /// The use of this parameter (if any), must be defined in the requested message. By default assumed not used (0).
    /// Target address of message stream (if message has target address fields). 0: Flight-stack default (recommended), 1: address of requestor, 2: broadcast.
    /// ***** END Params
    /// Request the target system(s) emit a single instance of a specified message (i.e. a "one-shot" version of MAV_CMD_SET_MESSAGE_INTERVAL).
    RequestMessage = 512,
    /// ***** START Params
    /// The MAVLink message ID of the requested message.
    /// Use for index ID, if required. Otherwise, the use of this parameter (if any) must be defined in the requested message. By default assumed not used (0).
    /// The use of this parameter (if any), must be defined in the requested message. By default assumed not used (0).
    /// The use of this parameter (if any), must be defined in the requested message. By default assumed not used (0).
    /// The use of this parameter (if any), must be defined in the requested message. By default assumed not used (0).
    /// The use of this parameter (if any), must be defined in the requested message. By default assumed not used (0).
    /// Target address for requested message (if message has target address fields). 0: Flight-stack default, 1: address of requestor, 2: broadcast.
    /// ***** END Params
    /// Request MAVLink protocol version compatibility. All receivers should ACK the command and then emit their capabilities in an PROTOCOL_VERSION message
    RequestProtocolVersion = 519,
    /// ***** START Params
    /// 1: Request supported protocol versions by all nodes on the network
    /// Reserved (all remaining params)
    /// ***** END Params
    /// Request autopilot capabilities. The receiver should ACK the command and then emit its capabilities in an AUTOPILOT_VERSION message
    RequestAutopilotCapabilities = 520,
    /// ***** START Params
    /// 1: Request autopilot version
    /// Reserved (all remaining params)
    /// ***** END Params
    /// Request camera information (CAMERA_INFORMATION).
    RequestCameraInformation = 521,
    /// ***** START Params
    /// 0: No action 1: Request camera capabilities
    /// Reserved (all remaining params)
    /// ***** END Params
    /// Request camera settings (CAMERA_SETTINGS).
    RequestCameraSettings = 522,
    /// ***** START Params
    /// 0: No Action 1: Request camera settings
    /// Reserved (all remaining params)
    /// ***** END Params
    /// Request storage information (STORAGE_INFORMATION). Use the command's target_component to target a specific component's storage.
    RequestStorageInformation = 525,
    /// ***** START Params
    /// Storage ID (0 for all, 1 for first, 2 for second, etc.)
    /// 0: No Action 1: Request storage information
    /// Reserved (all remaining params)
    /// ***** END Params
    /// Format a storage medium. Once format is complete, a STORAGE_INFORMATION message is sent. Use the command's target_component to target a specific component's storage.
    StorageFormat = 526,
    /// ***** START Params
    /// Storage ID (1 for first, 2 for second, etc.)
    /// Format storage (and reset image log). 0: No action 1: Format storage
    /// Reset Image Log (without formatting storage medium). This will reset CAMERA_CAPTURE_STATUS.image_count and CAMERA_IMAGE_CAPTURED.image_index. 0: No action 1: Reset Image Log
    /// Reserved (all remaining params)
    /// ***** END Params
    /// Request camera capture status (CAMERA_CAPTURE_STATUS)
    RequestCameraCaptureStatus = 527,
    /// ***** START Params
    /// 0: No Action 1: Request camera capture status
    /// Reserved (all remaining params)
    /// ***** END Params
    /// Request flight information (FLIGHT_INFORMATION)
    RequestFlightInformation = 528,
    /// ***** START Params
    /// 1: Request flight information
    /// Reserved (all remaining params)
    /// ***** END Params
    /// Reset all camera settings to Factory Default
    ResetCameraSettings = 529,
    /// ***** START Params
    /// 0: No Action 1: Reset all settings
    /// Reserved (all remaining params)
    /// ***** END Params
    /// Set camera running mode. Use NaN for reserved values. GCS will send a MAV_CMD_REQUEST_VIDEO_STREAM_STATUS command after a mode change if the camera supports video streaming.
    SetCameraMode = 530,
    /// ***** START Params
    /// Reserved (Set to 0)
    /// Camera mode
    /// ***** END Params
    /// Set camera zoom. Camera must respond with a CAMERA_SETTINGS message (on success).
    SetCameraZoom = 531,
    /// ***** START Params
    /// Zoom type
    /// Zoom value. The range of valid values depend on the zoom type.
    /// ***** END Params
    /// Set camera focus. Camera must respond with a CAMERA_SETTINGS message (on success).
    SetCameraFocus = 532,
    /// ***** START Params
    /// Focus type
    /// Focus value
    /// ***** END Params
    /// Tagged jump target. Can be jumped to with MAV_CMD_DO_JUMP_TAG.
    JumpTag = 600,
    /// ***** START Params
    /// Tag.
    /// ***** END Params
    /// Jump to the matching tag in the mission list. Repeat this action for the specified number of times. A mission should contain a single matching tag for each jump. If this is not the case then a jump to a missing tag should complete the mission, and a jump where there are multiple matching tags should always select the one with the lowest mission sequence number.
    DoJumpTag = 601,
    /// ***** START Params
    /// Target tag to jump to.
    /// Repeat count.
    /// ***** END Params
    /// High level setpoint to be sent to a gimbal manager to set a gimbal attitude. It is possible to set combinations of the values below. E.g. an angle as well as a desired angular rate can be used to get to this angle at a certain angular rate, or an angular rate only will result in continuous turning. NaN is to be used to signal unset. Note: a gimbal is never to react to this command but only the gimbal manager.
    DoGimbalManagerTiltpan = 1000,
    /// ***** START Params
    /// Tilt/pitch rate (positive to tilt up).
    /// Pan/yaw rate (positive to pan to the right).
    /// Tilt/pitch angle (positive to tilt up, relative to vehicle for PAN mode, relative to world horizon for HOLD mode).
    /// Pan/yaw angle (positive to pan to the right, relative to vehicle for PAN mode, absolute to North for HOLD mode).
    /// Gimbal manager flags to use.
    /// The use of this parameter (if any), must be defined in the requested message. By default assumed not used (0).
    /// Component ID of gimbal device to address (or 1-6 for non-MAVLink gimbal), 0 for all gimbal device components. (Send command multiple times for more than one but not all gimbals.)
    /// ***** END Params
    /// If the gimbal manager supports visual tracking (GIMBAL_MANAGER_CAP_FLAGS_HAS_TRACKING_POINT is set), this command allows to initiate the tracking. Such a tracking gimbal manager would usually be an integrated camera/gimbal, or alternatively a companion computer connected to a camera.
    DoGimbalManagerTrackPoint = 1001,
    /// ***** START Params
    /// Point to track x value.
    /// Point to track y value.
    /// The use of this parameter (if any), must be defined in the requested message. By default assumed not used (0).
    /// The use of this parameter (if any), must be defined in the requested message. By default assumed not used (0).
    /// The use of this parameter (if any), must be defined in the requested message. By default assumed not used (0).
    /// The use of this parameter (if any), must be defined in the requested message. By default assumed not used (0).
    /// Component ID of gimbal device to address (or 1-6 for non-MAVLink gimbal), 0 for all gimbal device components. (Send command multiple times for more than one but not all gimbals.)
    /// ***** END Params
    /// If the gimbal supports visual tracking (GIMBAL_MANAGER_CAP_FLAGS_HAS_TRACKING_RECTANGLE is set), this command allows to initiate the tracking. Such a tracking gimbal manager would usually be an integrated camera/gimbal, or alternatively a companion computer connected to a camera.
    DoGimbalManagerTrackRectangle = 1002,
    /// ***** START Params
    /// Top left corner of rectangle x value (normalized 0..1, 0 is left, 1 is right).
    /// Top left corner of rectangle y value (normalized 0..1, 0 is top, 1 is bottom).
    /// Bottom right corner of rectangle x value (normalized 0..1, 0 is left, 1 is right).
    /// Bottom right corner of rectangle y value (normalized 0..1, 0 is top, 1 is bottom).
    /// The use of this parameter (if any), must be defined in the requested message. By default assumed not used (0).
    /// The use of this parameter (if any), must be defined in the requested message. By default assumed not used (0).
    /// Component ID of gimbal device to address (or 1-6 for non-MAVLink gimbal), 0 for all gimbal device components. (Send command multiple times for more than one but not all gimbals.)
    /// ***** END Params
    /// Start image capture sequence. Sends CAMERA_IMAGE_CAPTURED after each capture. Use NaN for reserved values.
    ImageStartCapture = 2000,
    /// ***** START Params
    /// Reserved (Set to 0)
    /// Desired elapsed time between two consecutive pictures (in seconds). Minimum values depend on hardware (typically greater than 2 seconds).
    /// Total number of images to capture. 0 to capture forever/until MAV_CMD_IMAGE_STOP_CAPTURE.
    /// Capture sequence number starting from 1. This is only valid for single-capture (param3 == 1). Increment the capture ID for each capture command to prevent double captures when a command is re-transmitted. Use 0 to ignore it.
    /// Reserved (all remaining params)
    /// ***** END Params
    /// Stop image capture sequence Use NaN for reserved values.
    ImageStopCapture = 2001,
    /// ***** START Params
    /// Reserved (Set to 0)
    /// ***** END Params
    /// Re-request a CAMERA_IMAGE_CAPTURED message.
    RequestCameraImageCapture = 2002,
    /// ***** START Params
    /// Sequence number for missing CAMERA_IMAGE_CAPTURED message
    /// ***** END Params
    /// Enable or disable on-board camera triggering system.
    DoTriggerControl = 2003,
    /// ***** START Params
    /// Trigger enable/disable (0 for disable, 1 for start), -1 to ignore
    /// 1 to reset the trigger sequence, -1 or 0 to ignore
    /// 1 to pause triggering, but without switching the camera off or retracting it. -1 to ignore
    /// ***** END Params
    /// Starts video capture (recording).
    VideoStartCapture = 2500,
    /// ***** START Params
    /// Video Stream ID (0 for all streams)
    /// Frequency CAMERA_CAPTURE_STATUS messages should be sent while recording (0 for no messages, otherwise frequency)
    /// ***** END Params
    /// Stop the current video capture (recording).
    VideoStopCapture = 2501,
    /// ***** START Params
    /// Video Stream ID (0 for all streams)
    /// ***** END Params
    /// Start video streaming
    VideoStartStreaming = 2502,
    /// ***** START Params
    /// Video Stream ID (0 for all streams, 1 for first, 2 for second, etc.)
    /// ***** END Params
    /// Stop the given video stream
    VideoStopStreaming = 2503,
    /// ***** START Params
    /// Video Stream ID (0 for all streams, 1 for first, 2 for second, etc.)
    /// ***** END Params
    /// Request video stream information (VIDEO_STREAM_INFORMATION)
    RequestVideoStreamInformation = 2504,
    /// ***** START Params
    /// Video Stream ID (0 for all streams, 1 for first, 2 for second, etc.)
    /// ***** END Params
    /// Request video stream status (VIDEO_STREAM_STATUS)
    RequestVideoStreamStatus = 2505,
    /// ***** START Params
    /// Video Stream ID (0 for all streams, 1 for first, 2 for second, etc.)
    /// ***** END Params
    /// Request to start streaming logging data over MAVLink (see also LOGGING_DATA message)
    LoggingStart = 2510,
    /// ***** START Params
    /// Format: 0: ULog
    /// Reserved (set to 0)
    /// Reserved (set to 0)
    /// Reserved (set to 0)
    /// Reserved (set to 0)
    /// Reserved (set to 0)
    /// Reserved (set to 0)
    /// ***** END Params
    /// Request to stop streaming log data over MAVLink
    LoggingStop = 2511,
    /// ***** START Params
    /// Reserved (set to 0)
    /// Reserved (set to 0)
    /// Reserved (set to 0)
    /// Reserved (set to 0)
    /// Reserved (set to 0)
    /// Reserved (set to 0)
    /// Reserved (set to 0)
    /// ***** END Params
    AirframeConfiguration = 2520,
    /// ***** START Params
    /// Landing gear ID (default: 0, -1 for all)
    /// Landing gear position (Down: 0, Up: 1, NaN for no change)
    /// ***** END Params
    /// Request to start/stop transmitting over the high latency telemetry
    ControlHighLatency = 2600,
    /// ***** START Params
    /// Control transmission over high latency telemetry (0: stop, 1: start)
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// ***** END Params
    /// Create a panorama at the current position
    PanoramaCreate = 2800,
    /// ***** START Params
    /// Viewing angle horizontal of the panorama (+- 0.5 the total angle)
    /// Viewing angle vertical of panorama.
    /// Speed of the horizontal rotation.
    /// Speed of the vertical rotation.
    /// ***** END Params
    /// Request VTOL transition
    DoVtolTransition = 3000,
    /// ***** START Params
    /// The target VTOL state. Only MAV_VTOL_STATE_MC and MAV_VTOL_STATE_FW can be used.
    /// ***** END Params
    /// Request authorization to arm the vehicle to a external entity, the arm authorizer is responsible to request all data that is needs from the vehicle before authorize or deny the request. If approved the progress of command_ack message should be set with period of time that this authorization is valid in seconds or in case it was denied it should be set with one of the reasons in ARM_AUTH_DENIED_REASON.
    ///         
    ArmAuthorizationRequest = 3001,
    /// ***** START Params
    /// Vehicle system id, this way ground station can request arm authorization on behalf of any vehicle
    /// ***** END Params
    /// This command sets the submode to standard guided when vehicle is in guided mode. The vehicle holds position and altitude and the user can input the desired velocities along all three axes.
    ///                   
    SetGuidedSubmodeStandard = 4000,
    /// This command sets submode circle when vehicle is in guided mode. Vehicle flies along a circle facing the center of the circle. The user can input the velocity along the circle and change the radius. If no input is given the vehicle will hold position.
    ///                   
    SetGuidedSubmodeCircle = 4001,
    /// ***** START Params
    /// Radius of desired circle in CIRCLE_MODE
    /// User defined
    /// User defined
    /// User defined
    /// Target latitude of center of circle in CIRCLE_MODE
    /// Target longitude of center of circle in CIRCLE_MODE
    /// ***** END Params
    /// Delay mission state machine until gate has been reached.
    ConditionGate = 4501,
    /// ***** START Params
    /// Geometry: 0: orthogonal to path between previous and next waypoint.
    /// Altitude: 0: ignore altitude
    /// Empty
    /// Empty
    /// Latitude
    /// Longitude
    /// Altitude
    /// ***** END Params
    /// Fence return point. There can only be one fence return point.
    ///         
    NavFenceReturnPoint = 5000,
    /// ***** START Params
    /// Reserved
    /// Reserved
    /// Reserved
    /// Reserved
    /// Latitude
    /// Longitude
    /// Altitude
    /// ***** END Params
    /// Fence vertex for an inclusion polygon (the polygon must not be self-intersecting). The vehicle must stay within this area. Minimum of 3 vertices required.
    ///         
    NavFencePolygonVertexInclusion = 5001,
    /// ***** START Params
    /// Polygon vertex count
    /// Reserved
    /// Reserved
    /// Reserved
    /// Latitude
    /// Longitude
    /// Reserved
    /// ***** END Params
    /// Fence vertex for an exclusion polygon (the polygon must not be self-intersecting). The vehicle must stay outside this area. Minimum of 3 vertices required.
    ///         
    NavFencePolygonVertexExclusion = 5002,
    /// ***** START Params
    /// Polygon vertex count
    /// Reserved
    /// Reserved
    /// Reserved
    /// Latitude
    /// Longitude
    /// Reserved
    /// ***** END Params
    /// Circular fence area. The vehicle must stay inside this area.
    ///         
    NavFenceCircleInclusion = 5003,
    /// ***** START Params
    /// Radius.
    /// Reserved
    /// Reserved
    /// Reserved
    /// Latitude
    /// Longitude
    /// Reserved
    /// ***** END Params
    /// Circular fence area. The vehicle must stay outside this area.
    ///         
    NavFenceCircleExclusion = 5004,
    /// ***** START Params
    /// Radius.
    /// Reserved
    /// Reserved
    /// Reserved
    /// Latitude
    /// Longitude
    /// Reserved
    /// ***** END Params
    /// Rally point. You can have multiple rally points defined.
    ///         
    NavRallyPoint = 5100,
    /// ***** START Params
    /// Reserved
    /// Reserved
    /// Reserved
    /// Reserved
    /// Latitude
    /// Longitude
    /// Altitude
    /// ***** END Params
    /// Commands the vehicle to respond with a sequence of messages UAVCAN_NODE_INFO, one message per every UAVCAN node that is online. Note that some of the response messages can be lost, which the receiver can detect easily by checking whether every received UAVCAN_NODE_STATUS has a matching message UAVCAN_NODE_INFO received earlier; if not, this command should be sent again in order to request re-transmission of the node information messages.
    UavcanGetNodeInfo = 5200,
    /// ***** START Params
    /// Reserved (set to 0)
    /// Reserved (set to 0)
    /// Reserved (set to 0)
    /// Reserved (set to 0)
    /// Reserved (set to 0)
    /// Reserved (set to 0)
    /// Reserved (set to 0)
    /// ***** END Params
    /// Deploy payload on a Lat / Lon / Alt position. This includes the navigation to reach the required release position and velocity.
    PayloadPrepareDeploy = 30001,
    /// ***** START Params
    /// Operation mode. 0: prepare single payload deploy (overwriting previous requests), but do not execute it. 1: execute payload deploy immediately (rejecting further deploy commands during execution, but allowing abort). 2: add payload deploy to existing deployment list.
    /// Desired approach vector in compass heading. A negative value indicates the system can define the approach vector at will.
    /// Desired ground speed at release time. This can be overridden by the airframe in case it needs to meet minimum airspeed. A negative value indicates the system can define the ground speed at will.
    /// Minimum altitude clearance to the release position. A negative value indicates the system can define the clearance at will.
    /// Latitude. Note, if used in MISSION_ITEM (deprecated) the units are degrees (unscaled)
    /// Longitude. Note, if used in MISSION_ITEM (deprecated) the units are degrees (unscaled)
    /// Altitude (MSL)
    /// ***** END Params
    /// Control the payload deployment.
    PayloadControlDeploy = 30002,
    /// ***** START Params
    /// Operation mode. 0: Abort deployment, continue normal mission. 1: switch to payload deployment mode. 100: delete first payload deployment request. 101: delete all payload deployment requests.
    /// Reserved
    /// Reserved
    /// Reserved
    /// Reserved
    /// Reserved
    /// Reserved
    /// ***** END Params
    /// User defined waypoint item. Ground Station will show the Vehicle as flying through this item.
    WaypointUser1 = 31000,
    /// ***** START Params
    /// User defined
    /// User defined
    /// User defined
    /// User defined
    /// Latitude unscaled
    /// Longitude unscaled
    /// Altitude (MSL)
    /// ***** END Params
    /// User defined waypoint item. Ground Station will show the Vehicle as flying through this item.
    WaypointUser2 = 31001,
    /// ***** START Params
    /// User defined
    /// User defined
    /// User defined
    /// User defined
    /// Latitude unscaled
    /// Longitude unscaled
    /// Altitude (MSL)
    /// ***** END Params
    /// User defined waypoint item. Ground Station will show the Vehicle as flying through this item.
    WaypointUser3 = 31002,
    /// ***** START Params
    /// User defined
    /// User defined
    /// User defined
    /// User defined
    /// Latitude unscaled
    /// Longitude unscaled
    /// Altitude (MSL)
    /// ***** END Params
    /// User defined waypoint item. Ground Station will show the Vehicle as flying through this item.
    WaypointUser4 = 31003,
    /// ***** START Params
    /// User defined
    /// User defined
    /// User defined
    /// User defined
    /// Latitude unscaled
    /// Longitude unscaled
    /// Altitude (MSL)
    /// ***** END Params
    /// User defined waypoint item. Ground Station will show the Vehicle as flying through this item.
    WaypointUser5 = 31004,
    /// ***** START Params
    /// User defined
    /// User defined
    /// User defined
    /// User defined
    /// Latitude unscaled
    /// Longitude unscaled
    /// Altitude (MSL)
    /// ***** END Params
    /// User defined spatial item. Ground Station will not show the Vehicle as flying through this item. Example: ROI item.
    SpatialUser1 = 31005,
    /// ***** START Params
    /// User defined
    /// User defined
    /// User defined
    /// User defined
    /// Latitude unscaled
    /// Longitude unscaled
    /// Altitude (MSL)
    /// ***** END Params
    /// User defined spatial item. Ground Station will not show the Vehicle as flying through this item. Example: ROI item.
    SpatialUser2 = 31006,
    /// ***** START Params
    /// User defined
    /// User defined
    /// User defined
    /// User defined
    /// Latitude unscaled
    /// Longitude unscaled
    /// Altitude (MSL)
    /// ***** END Params
    /// User defined spatial item. Ground Station will not show the Vehicle as flying through this item. Example: ROI item.
    SpatialUser3 = 31007,
    /// ***** START Params
    /// User defined
    /// User defined
    /// User defined
    /// User defined
    /// Latitude unscaled
    /// Longitude unscaled
    /// Altitude (MSL)
    /// ***** END Params
    /// User defined spatial item. Ground Station will not show the Vehicle as flying through this item. Example: ROI item.
    SpatialUser4 = 31008,
    /// ***** START Params
    /// User defined
    /// User defined
    /// User defined
    /// User defined
    /// Latitude unscaled
    /// Longitude unscaled
    /// Altitude (MSL)
    /// ***** END Params
    /// User defined spatial item. Ground Station will not show the Vehicle as flying through this item. Example: ROI item.
    SpatialUser5 = 31009,
    /// ***** START Params
    /// User defined
    /// User defined
    /// User defined
    /// User defined
    /// Latitude unscaled
    /// Longitude unscaled
    /// Altitude (MSL)
    /// ***** END Params
    /// User defined command. Ground Station will not show the Vehicle as flying through this item. Example: MAV_CMD_DO_SET_PARAMETER item.
    User1 = 31010,
    /// ***** START Params
    /// User defined
    /// User defined
    /// User defined
    /// User defined
    /// User defined
    /// User defined
    /// User defined
    /// ***** END Params
    /// User defined command. Ground Station will not show the Vehicle as flying through this item. Example: MAV_CMD_DO_SET_PARAMETER item.
    User2 = 31011,
    /// ***** START Params
    /// User defined
    /// User defined
    /// User defined
    /// User defined
    /// User defined
    /// User defined
    /// User defined
    /// ***** END Params
    /// User defined command. Ground Station will not show the Vehicle as flying through this item. Example: MAV_CMD_DO_SET_PARAMETER item.
    User3 = 31012,
    /// ***** START Params
    /// User defined
    /// User defined
    /// User defined
    /// User defined
    /// User defined
    /// User defined
    /// User defined
    /// ***** END Params
    /// User defined command. Ground Station will not show the Vehicle as flying through this item. Example: MAV_CMD_DO_SET_PARAMETER item.
    User4 = 31013,
    /// ***** START Params
    /// User defined
    /// User defined
    /// User defined
    /// User defined
    /// User defined
    /// User defined
    /// User defined
    /// ***** END Params
    /// User defined command. Ground Station will not show the Vehicle as flying through this item. Example: MAV_CMD_DO_SET_PARAMETER item.
    ///
    /// ***** START Params
    /// User defined
    /// User defined
    /// User defined
    /// User defined
    /// User defined
    /// User defined
    /// User defined
    /// ***** END Params
    User5 = 31014,
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
pub enum MavDataStream {
    /// A data stream is not a fixed set of messages, but rather a
    /// recommendation to the autopilot software. Individual autopilots may or may not obey
    /// the recommended messages.
    /// Enable all data streams
    All = 0,
    /// Enable IMU_RAW, GPS_RAW, GPS_STATUS packets.
    RawSensors = 1,
    /// Enable GPS_STATUS, CONTROL_STATUS, AUX_STATUS
    ExtendedStatus = 2,
    /// Enable RC_CHANNELS_SCALED, RC_CHANNELS_RAW, SERVO_OUTPUT_RAW
    RcChannels = 3,
    /// Enable ATTITUDE_CONTROLLER_OUTPUT, POSITION_CONTROLLER_OUTPUT, NAV_CONTROLLER_OUTPUT.
    RawController = 4,
    /// Enable LOCAL_POSITION, GLOBAL_POSITION/GLOBAL_POSITION_INT messages.
    Position = 6,
    /// Dependent on the autopilot
    Extra1 = 10,
    /// Dependent on the autopilot
    Extra2 = 11,
    /// Dependent on the autopilot
    Extra3 = 12,
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
pub enum MavRoi {
    /// The ROI (region of interest) for the vehicle. This can be
    /// be used by the vehicle for camera/vehicle attitude alignment (see
    /// MAV_CMD_NAV_ROI).
    /// No region of interest.
    None = 0,
    /// Point toward next waypoint, with optional pitch/roll/yaw offset.
    Wpnext = 1,
    /// Point toward given waypoint.
    Wpindex = 2,
    /// Point toward fixed location.
    Location = 3,
    /// Point toward of given id.
    Target = 4,
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
pub enum MavCmdAck {
    /// ACK / NACK / ERROR values as a result of MAV_CMDs and for mission item transmission.
    /// Command / mission item is ok.
    Ok = 0,
    /// Generic error message if none of the other reasons fails or if no detailed error reporting is implemented.
    ErrFail = 1,
    /// The system is refusing to accept this command from this source / communication partner.
    ErrAccessDenied = 2,
    /// Command or mission item is not supported, other commands would be accepted.
    ErrNotSupported = 3,
    /// The coordinate frame of this command / mission item is not supported.
    ErrCoordinateFrameNotSupported = 4,
    /// The coordinate frame of this command is ok, but he coordinate values exceed the safety limits of this system. This is a generic error, please use the more specific error messages below if possible.
    ErrCoordinatesOutOfRange = 5,
    /// The X or latitude value is out of range.
    ErrXLatOutOfRange = 6,
    /// The Y or longitude value is out of range.
    ErrYLonOutOfRange = 7,
    /// The Z or altitude value is out of range.
    ErrZAltOutOfRange = 8,
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
pub enum MavParamType {
    /// Specifies the datatype of a MAVLink parameter.
    /// Not used in MavLink, make protobuf happy.
    Undefined = 0,
    /// 8-bit unsigned integer
    Uint8 = 1,
    /// 8-bit signed integer
    Int8 = 2,
    /// 16-bit unsigned integer
    Uint16 = 3,
    /// 16-bit signed integer
    Int16 = 4,
    /// 32-bit unsigned integer
    Uint32 = 5,
    /// 32-bit signed integer
    Int32 = 6,
    /// 64-bit unsigned integer
    Uint64 = 7,
    /// 64-bit signed integer
    Int64 = 8,
    /// 32-bit floating-point
    Real32 = 9,
    /// 64-bit floating-point
    Real64 = 10,
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
pub enum MavParamExtType {
    /// Specifies the datatype of a MAVLink extended parameter.
    /// Not used in MavLink, make protobuf happy.
    Undefined = 0,
    /// 8-bit unsigned integer
    Uint8 = 1,
    /// 8-bit signed integer
    Int8 = 2,
    /// 16-bit unsigned integer
    Uint16 = 3,
    /// 16-bit signed integer
    Int16 = 4,
    /// 32-bit unsigned integer
    Uint32 = 5,
    /// 32-bit signed integer
    Int32 = 6,
    /// 64-bit unsigned integer
    Uint64 = 7,
    /// 64-bit signed integer
    Int64 = 8,
    /// 32-bit floating-point
    Real32 = 9,
    /// 64-bit floating-point
    Real64 = 10,
    /// Custom Type
    Custom = 11,
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
pub enum MavResult {
    /// Result from a MAVLink command (MAV_CMD)
    /// Command is valid (is supported and has valid parameters), and was executed.
    Accepted = 0,
    /// Command is valid, but cannot be executed at this time. This is used to indicate a problem that should be fixed just by waiting (e.g. a state machine is busy, can't arm because have not got GPS lock, etc.). Retrying later should work.
    TemporarilyRejected = 1,
    /// Command is invalid (is supported but has invalid parameters). Retrying same command and parameters will not work.
    Denied = 2,
    /// Command is not supported (unknown).
    Unsupported = 3,
    /// Command is valid, but execution has failed. This is used to indicate any non-temporary or unexpected problem, i.e. any problem that must be fixed before the command can succeed/be retried. For example, attempting to write a file when out of memory, attempting to arm when sensors are not calibrated, etc.
    Failed = 4,
    /// Command is valid and is being executed. This will be followed by further progress updates, i.e. the component may send further COMMAND_ACK messages with result MAV_RESULT_IN_PROGRESS (at a rate decided by the implementation), and must terminate by sending a COMMAND_ACK message with final result of the operation. The COMMAND_ACK.progress field can be used to indicate the progress of the operation.
    InProgress = 5,
    /// Command has been cancelled (as a result of receiving a COMMAND_CANCEL message).
    Cancelled = 6,
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
pub enum MavMissionResult {
    /// Result of mission operation (in a MISSION_ACK message).
    /// mission accepted OK
    MavMissionAccepted = 0,
    /// Generic error / not accepting mission commands at all right now.
    MavMissionError = 1,
    /// Coordinate frame is not supported.
    MavMissionUnsupportedFrame = 2,
    /// Command is not supported.
    MavMissionUnsupported = 3,
    /// Mission items exceed storage space.
    MavMissionNoSpace = 4,
    /// One of the parameters has an invalid value.
    MavMissionInvalid = 5,
    /// param1 has an invalid value.
    MavMissionInvalidParam1 = 6,
    /// param2 has an invalid value.
    MavMissionInvalidParam2 = 7,
    /// param3 has an invalid value.
    MavMissionInvalidParam3 = 8,
    /// param4 has an invalid value.
    MavMissionInvalidParam4 = 9,
    /// x / param5 has an invalid value.
    MavMissionInvalidParam5X = 10,
    /// y / param6 has an invalid value.
    MavMissionInvalidParam6Y = 11,
    /// z / param7 has an invalid value.
    MavMissionInvalidParam7 = 12,
    /// Mission item received out of sequence
    MavMissionInvalidSequence = 13,
    /// Not accepting any mission commands from this communication partner.
    MavMissionDenied = 14,
    /// Current mission operation cancelled (e.g. mission upload, mission download).
    MavMissionOperationCancelled = 15,
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
pub enum MavSeverity {
    /// Indicates the severity level, generally used for status messages to indicate their relative urgency. Based on RFC-5424 using expanded definitions at: http://www.kiwisyslog.com/kb/info:-syslog-message-levels/.
    /// System is unusable. This is a "panic" condition.
    Emergency = 0,
    /// Action should be taken immediately. Indicates error in non-critical systems.
    Alert = 1,
    /// Action must be taken immediately. Indicates failure in a primary system.
    Critical = 2,
    /// Indicates an error in secondary/redundant systems.
    Error = 3,
    /// Indicates about a possible future error if this is not resolved within a given timeframe. Example would be a low battery warning.
    Warning = 4,
    /// An unusual event has occurred, though not an error condition. This should be investigated for the root cause.
    Notice = 5,
    /// Normal operational messages. Useful for logging. No action is required for these messages.
    Info = 6,
    /// Useful non-operational messages that can assist in debugging. These should not occur during normal operation.
    Debug = 7,
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
pub enum MavPowerStatus {
    /// Power supply status flags (bitmask)
    /// This enum is used to define bitmasks (work around protobuf limitations).
    /// Not used in MavLink, make protobuf happy.
    Undefined = 0,
    /// main brick power supply valid
    /// bit 1
    BrickValid = 1,
    /// main servo power supply valid for FMU
    /// bit 2
    ServoValid = 2,
    /// USB power is connected
    /// bit 3
    UsbConnected = 4,
    /// peripheral supply is in over-current state
    /// bit 4
    PeriphOvercurrent = 8,
    /// hi-power peripheral supply is in over-current state
    /// bit 5
    PeriphHipowerOvercurrent = 16,
    /// Power status has changed since boot
    /// bit 6
    Changed = 32,
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
pub enum SerialControlDev {
    /// SERIAL_CONTROL device types
    /// First telemetry port
    Telem1 = 0,
    /// Second telemetry port
    Telem2 = 1,
    /// First GPS port
    Gps1 = 2,
    /// Second GPS port
    Gps2 = 3,
    /// system shell
    Shell = 10,
    /// SERIAL0
    SerialControlSerial0 = 100,
    /// SERIAL1
    SerialControlSerial1 = 101,
    /// SERIAL2
    SerialControlSerial2 = 102,
    /// SERIAL3
    SerialControlSerial3 = 103,
    /// SERIAL4
    SerialControlSerial4 = 104,
    /// SERIAL5
    SerialControlSerial5 = 105,
    /// SERIAL6
    SerialControlSerial6 = 106,
    /// SERIAL7
    SerialControlSerial7 = 107,
    /// SERIAL8
    SerialControlSerial8 = 108,
    /// SERIAL9
    SerialControlSerial9 = 109,
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
pub enum SerialControlFlag {
    /// SERIAL_CONTROL flags (bitmask)
    /// This enum is used to define bitmasks (work around protobuf limitations).
    /// Not used in MavLink, make protobuf happy.
    Undefined = 0,
    /// Set if this is a reply
    /// bit 1
    Reply = 1,
    /// Set if the sender wants the receiver to send a response as another SERIAL_CONTROL message
    /// bit 2
    Respond = 2,
    /// Set if access to the serial port should be removed from whatever driver is currently using it, giving exclusive access to the SERIAL_CONTROL protocol. The port can be handed back by sending a request without this flag set
    /// bit 3
    Exclusive = 4,
    /// Block on writes to the serial port
    /// bit 4
    Blocking = 8,
    /// Send multiple replies until port is drained
    /// bit 5
    Multi = 16,
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
pub enum MavDistanceSensor {
    /// Enumeration of distance sensor types
    /// Laser rangefinder, e.g. LightWare SF02/F or PulsedLight units
    Laser = 0,
    /// Ultrasound rangefinder, e.g. MaxBotix units
    Ultrasound = 1,
    /// Infrared rangefinder, e.g. Sharp units
    Infrared = 2,
    /// Radar type, e.g. uLanding units
    Radar = 3,
    /// Broken or unknown type, e.g. analog units
    Unknown = 4,
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
pub enum MavSensorOrientation {
    /// Enumeration of sensor orientation, according to its rotations
    /// Roll: 0, Pitch: 0, Yaw: 0
    MavSensorRotationNone = 0,
    /// Roll: 0, Pitch: 0, Yaw: 45
    MavSensorRotationYaw45 = 1,
    /// Roll: 0, Pitch: 0, Yaw: 90
    MavSensorRotationYaw90 = 2,
    /// Roll: 0, Pitch: 0, Yaw: 135
    MavSensorRotationYaw135 = 3,
    /// Roll: 0, Pitch: 0, Yaw: 180
    MavSensorRotationYaw180 = 4,
    /// Roll: 0, Pitch: 0, Yaw: 225
    MavSensorRotationYaw225 = 5,
    /// Roll: 0, Pitch: 0, Yaw: 270
    MavSensorRotationYaw270 = 6,
    /// Roll: 0, Pitch: 0, Yaw: 315
    MavSensorRotationYaw315 = 7,
    /// Roll: 180, Pitch: 0, Yaw: 0
    MavSensorRotationRoll180 = 8,
    /// Roll: 180, Pitch: 0, Yaw: 45
    MavSensorRotationRoll180Yaw45 = 9,
    /// Roll: 180, Pitch: 0, Yaw: 90
    MavSensorRotationRoll180Yaw90 = 10,
    /// Roll: 180, Pitch: 0, Yaw: 135
    MavSensorRotationRoll180Yaw135 = 11,
    /// Roll: 0, Pitch: 180, Yaw: 0
    MavSensorRotationPitch180 = 12,
    /// Roll: 180, Pitch: 0, Yaw: 225
    MavSensorRotationRoll180Yaw225 = 13,
    /// Roll: 180, Pitch: 0, Yaw: 270
    MavSensorRotationRoll180Yaw270 = 14,
    /// Roll: 180, Pitch: 0, Yaw: 315
    MavSensorRotationRoll180Yaw315 = 15,
    /// Roll: 90, Pitch: 0, Yaw: 0
    MavSensorRotationRoll90 = 16,
    /// Roll: 90, Pitch: 0, Yaw: 45
    MavSensorRotationRoll90Yaw45 = 17,
    /// Roll: 90, Pitch: 0, Yaw: 90
    MavSensorRotationRoll90Yaw90 = 18,
    /// Roll: 90, Pitch: 0, Yaw: 135
    MavSensorRotationRoll90Yaw135 = 19,
    /// Roll: 270, Pitch: 0, Yaw: 0
    MavSensorRotationRoll270 = 20,
    /// Roll: 270, Pitch: 0, Yaw: 45
    MavSensorRotationRoll270Yaw45 = 21,
    /// Roll: 270, Pitch: 0, Yaw: 90
    MavSensorRotationRoll270Yaw90 = 22,
    /// Roll: 270, Pitch: 0, Yaw: 135
    MavSensorRotationRoll270Yaw135 = 23,
    /// Roll: 0, Pitch: 90, Yaw: 0
    MavSensorRotationPitch90 = 24,
    /// Roll: 0, Pitch: 270, Yaw: 0
    MavSensorRotationPitch270 = 25,
    /// Roll: 0, Pitch: 180, Yaw: 90
    MavSensorRotationPitch180Yaw90 = 26,
    /// Roll: 0, Pitch: 180, Yaw: 270
    MavSensorRotationPitch180Yaw270 = 27,
    /// Roll: 90, Pitch: 90, Yaw: 0
    MavSensorRotationRoll90Pitch90 = 28,
    /// Roll: 180, Pitch: 90, Yaw: 0
    MavSensorRotationRoll180Pitch90 = 29,
    /// Roll: 270, Pitch: 90, Yaw: 0
    MavSensorRotationRoll270Pitch90 = 30,
    /// Roll: 90, Pitch: 180, Yaw: 0
    MavSensorRotationRoll90Pitch180 = 31,
    /// Roll: 270, Pitch: 180, Yaw: 0
    MavSensorRotationRoll270Pitch180 = 32,
    /// Roll: 90, Pitch: 270, Yaw: 0
    MavSensorRotationRoll90Pitch270 = 33,
    /// Roll: 180, Pitch: 270, Yaw: 0
    MavSensorRotationRoll180Pitch270 = 34,
    /// Roll: 270, Pitch: 270, Yaw: 0
    MavSensorRotationRoll270Pitch270 = 35,
    /// Roll: 90, Pitch: 180, Yaw: 90
    MavSensorRotationRoll90Pitch180Yaw90 = 36,
    /// Roll: 90, Pitch: 0, Yaw: 270
    MavSensorRotationRoll90Yaw270 = 37,
    /// Roll: 90, Pitch: 68, Yaw: 293
    MavSensorRotationRoll90Pitch68Yaw293 = 38,
    /// Pitch: 315
    MavSensorRotationPitch315 = 39,
    /// Roll: 90, Pitch: 315
    MavSensorRotationRoll90Pitch315 = 40,
    /// Roll: 270, Yaw: 180
    MavSensorRotationRoll270Yaw180 = 41,
    /// Custom orientation
    MavSensorRotationCustom = 100,
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
pub enum MavProtocolCapability {
    /// Bitmask of (optional) autopilot capabilities (64 bit). If a bit is set, the autopilot supports this capability.
    /// This enum is used to define bitmasks (work around protobuf limitations).
    /// Not used in MavLink, make protobuf happy.
    Undefined = 0,
    /// Autopilot supports MISSION float message type.
    /// bit 1
    MissionFloat = 1,
    /// Autopilot supports the new param float message type.
    /// bit 2
    ParamFloat = 2,
    /// Autopilot supports MISSION_ITEM_INT scaled integer message type.
    /// bit 3
    MissionInt = 4,
    /// Autopilot supports COMMAND_INT scaled integer message type.
    /// bit 4
    CommandInt = 8,
    /// Autopilot supports the new param union message type.
    /// bit 5
    ParamUnion = 16,
    /// Autopilot supports the new FILE_TRANSFER_PROTOCOL message type.
    /// bit 6
    Ftp = 32,
    /// Autopilot supports commanding attitude offboard.
    /// bit 7
    SetAttitudeTarget = 64,
    /// Autopilot supports commanding position and velocity targets in local NED frame.
    /// bit 8
    SetPositionTargetLocalNed = 128,
    /// Autopilot supports commanding position and velocity targets in global scaled integers.
    /// bit 9
    SetPositionTargetGlobalInt = 256,
    /// Autopilot supports terrain protocol / data handling.
    /// bit 10
    Terrain = 512,
    /// Autopilot supports direct actuator control.
    /// bit 11
    SetActuatorTarget = 1024,
    /// Autopilot supports the flight termination command.
    /// bit 12
    FlightTermination = 2048,
    /// Autopilot supports onboard compass calibration.
    /// bit 13
    CompassCalibration = 4096,
    /// Autopilot supports MAVLink version 2.
    /// bit 14
    Mavlink2 = 8192,
    /// Autopilot supports mission fence protocol.
    /// bit 15
    MissionFence = 16384,
    /// Autopilot supports mission rally point protocol.
    /// bit 16
    MissionRally = 32768,
    /// Autopilot supports the flight information protocol.
    /// bit 17
    FlightInformation = 65536,
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
pub enum MavMissionType {
    /// Type of mission items being requested/sent in mission protocol.
    /// Items are mission commands for main mission.
    Mission = 0,
    /// Specifies GeoFence area(s). Items are MAV_CMD_NAV_FENCE_ GeoFence items.
    Fence = 1,
    /// Specifies the rally points for the vehicle. Rally points are alternative RTL points. Items are MAV_CMD_NAV_RALLY_POINT rally point items.
    Rally = 2,
    /// Only used in MISSION_CLEAR_ALL to clear all mission types.
    All = 255,
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
pub enum MavEstimatorType {
    /// Enumeration of estimator types
    /// Unknown type of the estimator.
    Unknown = 0,
    /// This is a naive estimator without any real covariance feedback.
    Naive = 1,
    /// Computer vision based estimate. Might be up to scale.
    Vision = 2,
    /// Visual-inertial estimate.
    Vio = 3,
    /// Plain GPS estimate.
    Gps = 4,
    /// Estimator integrating GPS and inertial sensing.
    GpsIns = 5,
    /// Estimate from external motion capturing system.
    Mocap = 6,
    /// Estimator based on lidar sensor input.
    Lidar = 7,
    /// Estimator on autopilot.
    Autopilot = 8,
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
pub enum MavBatteryType {
    /// Enumeration of battery types
    /// Not specified.
    Unknown = 0,
    /// Lithium polymer battery
    Lipo = 1,
    /// Lithium-iron-phosphate battery
    Life = 2,
    /// Lithium-ION battery
    Lion = 3,
    /// Nickel metal hydride battery
    Nimh = 4,
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
pub enum MavBatteryFunction {
    /// Enumeration of battery functions
    /// Battery function is unknown
    Unknown = 0,
    /// Battery supports all flight systems
    All = 1,
    /// Battery for the propulsion system
    Propulsion = 2,
    /// Avionics battery
    Avionics = 3,
    /// Payload battery
    MavBatteryTypePayload = 4,
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
pub enum MavBatteryChargeState {
    /// Enumeration for battery charge states.
    /// Low battery state is not provided
    Undefined = 0,
    /// Battery is not in low state. Normal operation.
    Ok = 1,
    /// Battery state is low, warn and monitor close.
    Low = 2,
    /// Battery state is critical, return or abort immediately.
    Critical = 3,
    /// Battery state is too low for ordinary abort sequence. Perform fastest possible emergency stop to prevent damage.
    Emergency = 4,
    /// Battery failed, damage unavoidable.
    Failed = 5,
    /// Battery is diagnosed to be defective or an error occurred, usage is discouraged / prohibited.
    Unhealthy = 6,
    /// Battery is charging.
    Charging = 7,
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
pub enum MavSmartBatteryFault {
    /// Smart battery supply status/fault flags (bitmask) for health indication.
    /// This enum is used to define bitmasks (work around protobuf limitations).
    /// Not used in MavLink, make protobuf happy.
    Undefined = 0,
    /// Battery has deep discharged.
    /// bit 1
    DeepDischarge = 1,
    /// Voltage spikes.
    /// bit 2
    Spikes = 2,
    /// Single cell has failed.
    /// bit 3
    SingleCellFail = 4,
    /// Over-current fault.
    /// bit 4
    OverCurrent = 8,
    /// Over-temperature fault.
    /// bit 5
    OverTemperature = 16,
    /// Under-temperature fault.
    /// bit 6
    UnderTemperature = 32,
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
pub enum MavGeneratorStatusFlag {
    /// Flags to report status/failure cases for a power generator (used in GENERATOR_STATUS). Note that FAULTS are conditions that cause the generator to fail. Warnings are conditions that require attention before the next use (they indicate the system is not operating properly).
    /// This enum is used to define bitmasks (work around protobuf limitations).
    /// Not used in MavLink, make protobuf happy.
    Undefined = 0,
    /// Generator is off.
    /// bit 1
    Off = 1,
    /// Generator is ready to start generating power.
    /// bit 2
    Ready = 2,
    /// Generator is generating power.
    /// bit 3
    Generating = 4,
    /// Generator is charging the batteries (generating enough power to charge and provide the load).
    /// bit 4
    Charging = 8,
    /// Generator is operating at a reduced maximum power.
    /// bit 5
    ReducedPower = 16,
    /// Generator is providing the maximum output.
    /// bit 6
    Maxpower = 32,
    /// Generator is near the maximum operating temperature, cooling is insufficient.
    /// bit 7
    OvertempWarning = 64,
    /// Generator hit the maximum operating temperature and shutdown.
    /// bit 8
    OvertempFault = 128,
    /// Power electronics are near the maximum operating temperature, cooling is insufficient.
    /// bit 9
    ElectronicsOvertempWarning = 256,
    /// Power electronics hit the maximum operating temperature and shutdown.
    /// bit 10
    ElectronicsOvertempFault = 512,
    /// Power electronics experienced a fault and shutdown.
    /// bit 11
    ElectronicsFault = 1024,
    /// The power source supplying the generator failed e.g. mechanical generator stopped, tether is no longer providing power, solar cell is in shade, hydrogen reaction no longer happening.
    /// bit 12
    PowersourceFault = 2048,
    /// Generator controller having communication problems.
    /// bit 13
    CommunicationWarning = 4096,
    /// Power electronic or generator cooling system error.
    /// bit 14
    CoolingWarning = 8192,
    /// Generator controller power rail experienced a fault.
    /// bit 15
    PowerRailFault = 16384,
    /// Generator controller exceeded the overcurrent threshold and shutdown to prevent damage.
    /// bit 16
    OvercurrentFault = 32768,
    /// Generator controller detected a high current going into the batteries and shutdown to prevent battery damage.
    /// bit 17
    BatteryOverchargeCurrentFault = 65536,
    /// Generator controller exceeded it's overvoltage threshold and shutdown to prevent it exceeding the voltage rating.
    /// bit 18
    OvervoltageFault = 131072,
    /// Batteries are under voltage (generator will not start).
    /// bit 19
    BatteryUndervoltFault = 262144,
    /// Generator start is inhibited by e.g. a safety switch.
    /// bit 20
    StartInhibited = 524288,
    /// Generator requires maintenance.
    /// bit 21
    MaintenanceRequired = 1048576,
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
pub enum MavVtolState {
    /// Enumeration of VTOL states
    /// MAV is not configured as VTOL
    Undefined = 0,
    /// VTOL is in transition from multicopter to fixed-wing
    TransitionToFw = 1,
    /// VTOL is in transition from fixed-wing to multicopter
    TransitionToMc = 2,
    /// VTOL is in multicopter state
    Mc = 3,
    /// VTOL is in fixed-wing state
    Fw = 4,
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
pub enum MavLandedState {
    /// Enumeration of landed detector states
    /// MAV landed state is unknown
    Undefined = 0,
    /// MAV is landed (on ground)
    OnGround = 1,
    /// MAV is in air
    InAir = 2,
    /// MAV currently taking off
    Takeoff = 3,
    /// MAV currently landing
    Landing = 4,
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
pub enum AdsbAltitudeType {
    /// Enumeration of the ADSB altimeter types
    /// Altitude reported from a Baro source using QNH reference
    PressureQnh = 0,
    /// Altitude reported from a GNSS source
    Geometric = 1,
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
pub enum AdsbEmitterType {
    /// ADSB classification for the type of vehicle emitting the transponder signal
    NoInfo = 0,
    Light = 1,
    Small = 2,
    Large = 3,
    HighVortexLarge = 4,
    Heavy = 5,
    HighlyManuv = 6,
    Rotocraft = 7,
    Unassigned = 8,
    Glider = 9,
    LighterAir = 10,
    Parachute = 11,
    UltraLight = 12,
    Unassigned2 = 13,
    Uav = 14,
    Space = 15,
    Unassgined3 = 16,
    EmergencySurface = 17,
    ServiceSurface = 18,
    PointObstacle = 19,
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
pub enum AdsbFlags {
    /// These flags indicate status such as data validity of each data source. Set = data valid
    /// This enum is used to define bitmasks (work around protobuf limitations).
    /// Not used in MavLink, make protobuf happy.
    Undefined = 0,
    /// bit 1
    ValidCoords = 1,
    /// bit 2
    ValidAltitude = 2,
    /// bit 3
    ValidHeading = 4,
    /// bit 4
    ValidVelocity = 8,
    /// bit 5
    ValidCallsign = 16,
    /// bit 6
    ValidSquawk = 32,
    /// bit 7
    Simulated = 64,
    /// bit 8
    VerticalVelocityValid = 128,
    /// bit 9
    BaroValid = 256,
    /// bit 16
    SourceUat = 32768,
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
pub enum MavDoRepositionFlags {
    /// Bitmap of options for the MAV_CMD_DO_REPOSITION
    /// This enum is used to define bitmasks (work around protobuf limitations).
    /// Not used in MavLink, make protobuf happy.
    Undefined = 0,
    /// The aircraft should immediately transition into guided. This should not be set for follow me applications
    /// bit 1
    ChangeMode = 1,
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
pub enum EstimatorStatusFlags {
    /// Flags in ESTIMATOR_STATUS message
    /// This enum is used to define bitmasks (work around protobuf limitations).
    /// Not used in MavLink, make protobuf happy.
    Undefined = 0,
    /// True if the attitude estimate is good
    /// bit 1
    EstimatorAttitude = 1,
    /// True if the horizontal velocity estimate is good
    /// bit 2
    EstimatorVelocityHoriz = 2,
    /// True if the  vertical velocity estimate is good
    /// bit 3
    EstimatorVelocityVert = 4,
    /// True if the horizontal position (relative) estimate is good
    /// bit 4
    EstimatorPosHorizRel = 8,
    /// True if the horizontal position (absolute) estimate is good
    /// bit 5
    EstimatorPosHorizAbs = 16,
    /// True if the vertical position (absolute) estimate is good
    /// bit 6
    EstimatorPosVertAbs = 32,
    /// True if the vertical position (above ground) estimate is good
    /// bit 7
    EstimatorPosVertAgl = 64,
    /// True if the EKF is in a constant position mode and is not using external measurements (eg GPS or optical flow)
    /// bit 8
    EstimatorConstPosMode = 128,
    /// True if the EKF has sufficient data to enter a mode that will provide a (relative) position estimate
    /// bit 9
    EstimatorPredPosHorizRel = 256,
    /// True if the EKF has sufficient data to enter a mode that will provide a (absolute) position estimate
    /// bit 10
    EstimatorPredPosHorizAbs = 512,
    /// True if the EKF has detected a GPS glitch
    /// bit 11
    EstimatorGpsGlitch = 1024,
    /// True if the EKF has detected bad accelerometer data
    /// bit 12
    EstimatorAccelError = 2048,
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
pub enum MotorTestOrder {
    /// default autopilot motor test method
    Default = 0,
    /// motor numbers are specified as their index in a predefined vehicle-specific sequence
    Sequence = 1,
    /// motor numbers are specified as the output as labeled on the board
    Board = 2,
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
pub enum MotorTestThrottleType {
    /// throttle as a percentage from 0 ~ 100
    MotorTestThrottlePercent = 0,
    /// throttle as an absolute PWM value (normally in range of 1000~2000)
    MotorTestThrottlePwm = 1,
    /// throttle pass-through from pilot's transmitter
    MotorTestThrottlePilot = 2,
    /// per-motor compass calibration test
    MotorTestCompassCal = 3,
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
pub enum GpsInputIgnoreFlags {
    /// This enum is used to define bitmasks (work around protobuf limitations).
    /// Not used in MavLink, make protobuf happy.
    Undefined = 0,
    /// ignore altitude field
    /// bit 1
    GpsInputIgnoreFlagAlt = 1,
    /// ignore hdop field
    /// bit 2
    GpsInputIgnoreFlagHdop = 2,
    /// ignore vdop field
    /// bit 3
    GpsInputIgnoreFlagVdop = 4,
    /// ignore horizontal velocity field (vn and ve)
    /// bit 4
    GpsInputIgnoreFlagVelHoriz = 8,
    /// ignore vertical velocity field (vd)
    /// bit 5
    GpsInputIgnoreFlagVelVert = 16,
    /// ignore speed accuracy field
    /// bit 6
    GpsInputIgnoreFlagSpeedAccuracy = 32,
    /// ignore horizontal accuracy field
    /// bit 7
    GpsInputIgnoreFlagHorizontalAccuracy = 64,
    /// ignore vertical accuracy field
    /// bit 8
    GpsInputIgnoreFlagVerticalAccuracy = 128,
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
pub enum MavCollisionAction {
    /// Possible actions an aircraft can take to avoid a collision.
    /// Ignore any potential collisions
    None = 0,
    /// Report potential collision
    Report = 1,
    /// Ascend or Descend to avoid threat
    AscendOrDescend = 2,
    /// Move horizontally to avoid threat
    MoveHorizontally = 3,
    /// Aircraft to move perpendicular to the collision's velocity vector
    MovePerpendicular = 4,
    /// Aircraft to fly directly back to its launch point
    Rtl = 5,
    /// Aircraft to stop in place
    Hover = 6,
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
pub enum MavCollisionThreatLevel {
    /// Aircraft-rated danger from this threat.
    /// Not a threat
    None = 0,
    /// Craft is mildly concerned about this threat
    Low = 1,
    /// Craft is panicking, and may take actions to avoid threat
    High = 2,
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
pub enum MavCollisionSrc {
    /// Source of information about this collision.
    /// ID field references ADSB_VEHICLE packets
    Adsb = 0,
    /// ID field references MAVLink SRC ID
    MavlinkGpsGlobalInt = 1,
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
pub enum GpsFixType {
    /// Type of GPS fix
    /// No GPS connected
    NoGps = 0,
    /// No position information, GPS is connected
    NoFix = 1,
    /// 2D position
    GpsFixType2dFix = 2,
    /// 3D position
    GpsFixType3dFix = 3,
    /// DGPS/SBAS aided 3D position
    Dgps = 4,
    /// RTK float, 3D position
    RtkFloat = 5,
    /// RTK Fixed, 3D position
    RtkFixed = 6,
    /// Static fixed, typically used for base stations
    Static = 7,
    /// PPP, 3D position.
    Ppp = 8,
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
pub enum RtkBaselineCoordinateSystem {
    /// RTK GPS baseline coordinate system, used for RTK corrections
    /// Earth-centered, Earth-fixed
    Ecef = 0,
    /// RTK basestation centered, north, east, down
    Ned = 1,
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
pub enum LandingTargetType {
    /// Type of landing target
    /// Landing target signaled by light beacon (ex: IR-LOCK)
    LightBeacon = 0,
    /// Landing target signaled by radio beacon (ex: ILS, NDB)
    RadioBeacon = 1,
    /// Landing target represented by a fiducial marker (ex: ARTag)
    VisionFiducial = 2,
    /// Landing target represented by a pre-defined visual shape/feature (ex: X-marker, H-marker, square)
    VisionOther = 3,
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
pub enum VtolTransitionHeading {
    /// Direction of VTOL transition
    /// Respect the heading configuration of the vehicle.
    VehicleDefault = 0,
    /// Use the heading pointing towards the next waypoint.
    NextWaypoint = 1,
    /// Use the heading on takeoff (while sitting on the ground).
    Takeoff = 2,
    /// Use the specified heading in parameter 4.
    Specified = 3,
    /// Use the current heading when reaching takeoff altitude (potentially facing the wind when weather-vaning is active).
    Any = 4,
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
pub enum CameraCapFlags {
    /// Camera capability flags (Bitmap)
    /// This enum is used to define bitmasks (work around protobuf limitations).
    /// Not used in MavLink, make protobuf happy.
    Undefined = 0,
    /// Camera is able to record video
    /// bit 1
    CaptureVideo = 1,
    /// Camera is able to capture images
    /// bit 2
    CaptureImage = 2,
    /// Camera has separate Video and Image/Photo modes (MAV_CMD_SET_CAMERA_MODE)
    /// bit 3
    HasModes = 4,
    /// Camera can capture images while in video mode
    /// bit 4
    CanCaptureImageInVideoMode = 8,
    /// Camera can capture videos while in Photo/Image mode
    /// bit 5
    CanCaptureVideoInImageMode = 16,
    /// Camera has image survey mode (MAV_CMD_SET_CAMERA_MODE)
    /// bit 6
    HasImageSurveyMode = 32,
    /// Camera has basic zoom control (MAV_CMD_SET_CAMERA_ZOOM)
    /// bit 7
    HasBasicZoom = 64,
    /// Camera has basic focus control (MAV_CMD_SET_CAMERA_FOCUS)
    /// bit 8
    HasBasicFocus = 128,
    /// Camera has video streaming capabilities (request VIDEO_STREAM_INFORMATION with MAV_CMD_REQUEST_MESSAGE for video streaming info)
    /// bit 9
    HasVideoStream = 256,
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
pub enum VideoStreamStatusFlags {
    /// Stream status flags (Bitmap)
    /// This enum is used to define bitmasks (work around protobuf limitations).
    /// Not used in MavLink, make protobuf happy.
    Undefined = 0,
    /// Stream is active (running)
    /// bit 1
    Running = 1,
    /// Stream is thermal imaging
    /// bit 2
    Thermal = 2,
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
pub enum VideoStreamType {
    /// Video stream types
    /// Stream is RTSP
    Rtsp = 0,
    /// Stream is RTP UDP (URI gives the port number)
    Rtpudp = 1,
    /// Stream is MPEG on TCP
    TcpMpeg = 2,
    /// Stream is h.264 on MPEG TS (URI gives the port number)
    MpegTsH264 = 3,
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
pub enum CameraZoomType {
    /// Zoom types for MAV_CMD_SET_CAMERA_ZOOM
    /// Zoom one step increment (-1 for wide, 1 for tele)
    ZoomTypeStep = 0,
    /// Continuous zoom up/down until stopped (-1 for wide, 1 for tele, 0 to stop zooming)
    ZoomTypeContinuous = 1,
    /// Zoom value as proportion of full camera range (a value between 0.0 and 100.0)
    ZoomTypeRange = 2,
    /// Zoom value/variable focal length in milimetres. Note that there is no message to get the valid zoom range of the camera, so this can type can only be used for cameras where the zoom range is known (implying that this cannot reliably be used in a GCS for an arbitrary camera)
    ZoomTypeFocalLength = 3,
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
pub enum SetFocusType {
    /// Focus types for MAV_CMD_SET_CAMERA_FOCUS
    /// Focus one step increment (-1 for focusing in, 1 for focusing out towards infinity).
    FocusTypeStep = 0,
    /// Continuous focus up/down until stopped (-1 for focusing in, 1 for focusing out towards infinity, 0 to stop focusing)
    FocusTypeContinuous = 1,
    /// Focus value as proportion of full camera focus range (a value between 0.0 and 100.0)
    FocusTypeRange = 2,
    /// Focus value in metres. Note that there is no message to get the valid focus range of the camera, so this can type can only be used for cameras where the range is known (implying that this cannot reliably be used in a GCS for an arbitrary camera).
    FocusTypeMeters = 3,
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
pub enum ParamAck {
    /// Result from a PARAM_EXT_SET message.
    /// Parameter value ACCEPTED and SET
    Accepted = 0,
    /// Parameter value UNKNOWN/UNSUPPORTED
    ValueUnsupported = 1,
    /// Parameter failed to set
    Failed = 2,
    /// Parameter value received but not yet validated or set. A subsequent PARAM_EXT_ACK will follow once operation is completed with the actual result. These are for parameters that may take longer to set. Instead of waiting for an ACK and potentially timing out, you will immediately receive this response to let you know it was received.
    InProgress = 3,
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
pub enum CameraMode {
    /// Camera Modes.
    /// Camera is in image/photo capture mode.
    Image = 0,
    /// Camera is in video capture mode.
    Video = 1,
    /// Camera is in image survey capture mode. It allows for camera controller to do specific settings for surveys.
    ImageSurvey = 2,
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
pub enum MavArmAuthDeniedReason {
    /// Not a specific reason
    Generic = 0,
    /// Authorizer will send the error as string to GCS
    None = 1,
    /// At least one waypoint have a invalid value
    InvalidWaypoint = 2,
    /// Timeout in the authorizer process(in case it depends on network)
    Timeout = 3,
    /// Airspace of the mission in use by another vehicle, second result parameter can have the waypoint id that caused it to be denied.
    AirspaceInUse = 4,
    /// Weather is not good to fly
    BadWeather = 5,
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
pub enum RcType {
    /// RC type
    /// Spektrum DSM2
    SpektrumDsm2 = 0,
    /// Spektrum DSMX
    SpektrumDsmx = 1,
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
pub enum PositionTargetTypemask {
    /// Bitmap to indicate which dimensions should be ignored by the vehicle: a value of 0b0000000000000000 or 0b0000001000000000 indicates that none of the setpoint dimensions should be ignored. If bit 9 is set the floats afx afy afz should be interpreted as force instead of acceleration.
    /// This enum is used to define bitmasks (work around protobuf limitations).
    /// Not used in MavLink, make protobuf happy.
    Undefined = 0,
    /// Ignore position x
    /// bit 1
    XIgnore = 1,
    /// Ignore position y
    /// bit 2
    YIgnore = 2,
    /// Ignore position z
    /// bit 3
    ZIgnore = 4,
    /// Ignore velocity x
    /// bit 4
    VxIgnore = 8,
    /// Ignore velocity y
    /// bit 5
    VyIgnore = 16,
    /// Ignore velocity z
    /// bit 6
    VzIgnore = 32,
    /// Ignore acceleration x
    /// bit 7
    AxIgnore = 64,
    /// Ignore acceleration y
    /// bit 8
    AyIgnore = 128,
    /// Ignore acceleration z
    /// bit 9
    AzIgnore = 256,
    /// Use force instead of acceleration
    /// bit 10
    ForceSet = 512,
    /// Ignore yaw
    /// bit 11
    YawIgnore = 1024,
    /// Ignore yaw rate
    /// bit 12
    YawRateIgnore = 2048,
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
pub enum UtmFlightState {
    /// Airborne status of UAS.
    /// Not used in MavLink, make protobuf happy.
    Undefined = 0,
    /// The flight state can't be determined.
    Unknown = 1,
    /// UAS on ground.
    Ground = 2,
    /// UAS airborne.
    Airborne = 3,
    /// UAS is in an emergency flight state.
    Emergency = 16,
    /// UAS has no active controls.
    Noctrl = 32,
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
pub enum UtmDataAvailFlags {
    /// Flags for the global position report.
    /// This enum is used to define bitmasks (work around protobuf limitations).
    /// Not used in MavLink, make protobuf happy.
    Undefined = 0,
    /// The field time contains valid data.
    /// bit 1
    TimeValid = 1,
    /// The field uas_id contains valid data.
    /// bit 2
    UasIdAvailable = 2,
    /// The fields lat, lon and h_acc contain valid data.
    /// bit 3
    PositionAvailable = 4,
    /// The fields alt and v_acc contain valid data.
    /// bit 4
    AltitudeAvailable = 8,
    /// The field relative_alt contains valid data.
    /// bit 5
    RelativeAltitudeAvailable = 16,
    /// The fields vx and vy contain valid data.
    /// bit 6
    HorizontalVeloAvailable = 32,
    /// The field vz contains valid data.
    /// bit 7
    VerticalVeloAvailable = 64,
    /// The fields next_lat, next_lon and next_alt contain valid data.
    /// bit 8
    NextWaypointAvailable = 128,
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
pub enum CellularNetworkRadioType {
    /// Cellular network radio type
    None = 0,
    Gsm = 1,
    Cdma = 2,
    Wcdma = 3,
    Lte = 4,
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
pub enum CellularStatusFlag {
    /// These flags encode the cellular network status
    /// State unknown or not reportable.
    Unknown = 0,
    /// Modem is unusable
    Failed = 1,
    /// Modem is being initialized
    Initializing = 2,
    /// Modem is locked
    Locked = 3,
    /// Modem is not enabled and is powered down
    Disabled = 4,
    /// Modem is currently transitioning to the CELLULAR_STATUS_FLAG_DISABLED state
    Disabling = 5,
    /// Modem is currently transitioning to the CELLULAR_STATUS_FLAG_ENABLED state
    Enabling = 6,
    /// Modem is enabled and powered on but not registered with a network provider and not available for data connections
    Enabled = 7,
    /// Modem is searching for a network provider to register
    Searching = 8,
    /// Modem is registered with a network provider, and data connections and messaging may be available for use
    Registered = 9,
    /// Modem is disconnecting and deactivating the last active packet data bearer. This state will not be entered if more than one packet data bearer is active and one of the active bearers is deactivated
    Disconnecting = 10,
    /// Modem is activating and connecting the first packet data bearer. Subsequent bearer activations when another bearer is already active do not cause this state to be entered
    Connecting = 11,
    /// One or more packet data bearers is active and connected
    Connected = 12,
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
pub enum CellularNetworkFailedReason {
    /// These flags are used to diagnose the failure state of CELLULAR_STATUS
    /// No error
    None = 0,
    /// Error state is unknown
    Unknown = 1,
    /// SIM is required for the modem but missing
    SimMissing = 2,
    /// SIM is available, but not usuable for connection
    SimError = 3,
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
pub enum PrecisionLandMode {
    /// Precision land modes (used in MAV_CMD_NAV_LAND).
    /// Normal (non-precision) landing.
    Disabled = 0,
    /// Use precision landing if beacon detected when land command accepted, otherwise land normally.
    Opportunistic = 1,
    /// Use precision landing, searching for beacon if not found when land command accepted (land normally if beacon cannot be found).
    Required = 2,
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
pub enum ParachuteAction {
    /// Parachute actions. Trigger release and enable/disable auto-release.
    /// Disable auto-release of parachute (i.e. release triggered by crash detectors).
    ParachuteDisable = 0,
    /// Enable auto-release of parachute.
    ParachuteEnable = 1,
    /// Release parachute and kill motors.
    ParachuteRelease = 2,
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
pub enum MavTunnelPayloadType {
    /// Encoding of payload unknown.
    Unknown = 0,
    /// Registered for STorM32 gimbal controller.
    Storm32Reserved0 = 200,
    /// Registered for STorM32 gimbal controller.
    Storm32Reserved1 = 201,
    /// Registered for STorM32 gimbal controller.
    Storm32Reserved2 = 202,
    /// Registered for STorM32 gimbal controller.
    Storm32Reserved3 = 203,
    /// Registered for STorM32 gimbal controller.
    Storm32Reserved4 = 204,
    /// Registered for STorM32 gimbal controller.
    Storm32Reserved5 = 205,
    /// Registered for STorM32 gimbal controller.
    Storm32Reserved6 = 206,
    /// Registered for STorM32 gimbal controller.
    Storm32Reserved7 = 207,
    /// Registered for STorM32 gimbal controller.
    Storm32Reserved8 = 208,
    /// Registered for STorM32 gimbal controller.
    Storm32Reserved9 = 209,
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
pub enum MavOdidIdType {
    /// No type defined.
    None = 0,
    /// Manufacturer Serial Number (ANSI/CTA-2063 format).
    SerialNumber = 1,
    /// CAA (Civil Aviation Authority) registered ID. Format: [ICAO Country Code].[CAA Assigned ID].
    CaaRegistrationId = 2,
    /// UTM (Unmanned Traffic Management) assigned UUID (RFC4122).
    UtmAssignedUuid = 3,
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
pub enum MavOdidUaType {
    /// No UA (Unmanned Aircraft) type defined.
    None = 0,
    /// Aeroplane/Airplane. Fixed wing.
    Aeroplane = 1,
    /// Helicopter or multirotor.
    HelicopterOrMultirotor = 2,
    /// Gyroplane.
    Gyroplane = 3,
    /// VTOL (Vertical Take-Off and Landing). Fixed wing aircraft that can take off vertically.
    HybridLift = 4,
    /// Ornithopter.
    Ornithopter = 5,
    /// Glider.
    Glider = 6,
    /// Kite.
    Kite = 7,
    /// Free Balloon.
    FreeBalloon = 8,
    /// Captive Balloon.
    CaptiveBalloon = 9,
    /// Airship. E.g. a blimp.
    Airship = 10,
    /// Free Fall/Parachute (unpowered).
    FreeFallParachute = 11,
    /// Rocket.
    Rocket = 12,
    /// Tethered powered aircraft.
    TetheredPoweredAircraft = 13,
    /// Ground Obstacle.
    GroundObstacle = 14,
    /// Other type of aircraft not listed earlier.
    Other = 15,
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
pub enum MavOdidStatus {
    /// The status of the (UA) Unmanned Aircraft is undefined.
    Undeclared = 0,
    /// The UA is on the ground.
    Ground = 1,
    /// The UA is in the air.
    Airborne = 2,
    /// The UA is having an emergency.
    Emergency = 3,
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
pub enum MavOdidHeightRef {
    /// The height field is relative to the take-off location.
    OverTakeoff = 0,
    /// The height field is relative to ground.
    OverGround = 1,
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
pub enum MavOdidHorAcc {
    /// The horizontal accuracy is unknown.
    Unknown = 0,
    /// The horizontal accuracy is smaller than 10 Nautical Miles. 18.52 km.
    MavOdidHorAcc10nm = 1,
    /// The horizontal accuracy is smaller than 4 Nautical Miles. 7.408 km.
    MavOdidHorAcc4nm = 2,
    /// The horizontal accuracy is smaller than 2 Nautical Miles. 3.704 km.
    MavOdidHorAcc2nm = 3,
    /// The horizontal accuracy is smaller than 1 Nautical Miles. 1.852 km.
    MavOdidHorAcc1nm = 4,
    /// The horizontal accuracy is smaller than 0.5 Nautical Miles. 926 m.
    MavOdidHorAcc05nm = 5,
    /// The horizontal accuracy is smaller than 0.3 Nautical Miles. 555.6 m.
    MavOdidHorAcc03nm = 6,
    /// The horizontal accuracy is smaller than 0.1 Nautical Miles. 185.2 m.
    MavOdidHorAcc01nm = 7,
    /// The horizontal accuracy is smaller than 0.05 Nautical Miles. 92.6 m.
    MavOdidHorAcc005nm = 8,
    /// The horizontal accuracy is smaller than 30 meter.
    MavOdidHorAcc30Meter = 9,
    /// The horizontal accuracy is smaller than 10 meter.
    MavOdidHorAcc10Meter = 10,
    /// The horizontal accuracy is smaller than 3 meter.
    MavOdidHorAcc3Meter = 11,
    /// The horizontal accuracy is smaller than 1 meter.
    MavOdidHorAcc1Meter = 12,
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
pub enum MavOdidVerAcc {
    /// The vertical accuracy is unknown.
    Unknown = 0,
    /// The vertical accuracy is smaller than 150 meter.
    MavOdidVerAcc150Meter = 1,
    /// The vertical accuracy is smaller than 45 meter.
    MavOdidVerAcc45Meter = 2,
    /// The vertical accuracy is smaller than 25 meter.
    MavOdidVerAcc25Meter = 3,
    /// The vertical accuracy is smaller than 10 meter.
    MavOdidVerAcc10Meter = 4,
    /// The vertical accuracy is smaller than 3 meter.
    MavOdidVerAcc3Meter = 5,
    /// The vertical accuracy is smaller than 1 meter.
    MavOdidVerAcc1Meter = 6,
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
pub enum MavOdidSpeedAcc {
    /// The speed accuracy is unknown.
    Unknown = 0,
    /// The speed accuracy is smaller than 10 meters per second.
    MavOdidSpeedAcc10MetersPerSecond = 1,
    /// The speed accuracy is smaller than 3 meters per second.
    MavOdidSpeedAcc3MetersPerSecond = 2,
    /// The speed accuracy is smaller than 1 meters per second.
    MavOdidSpeedAcc1MetersPerSecond = 3,
    /// The speed accuracy is smaller than 0.3 meters per second.
    MavOdidSpeedAcc03MetersPerSecond = 4,
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
pub enum MavOdidTimeAcc {
    /// The timestamp accuracy is unknown.
    Unknown = 0,
    /// The timestamp accuracy is smaller than or equal to 0.1 second.
    MavOdidTimeAcc01Second = 1,
    /// The timestamp accuracy is smaller than or equal to 0.2 second.
    MavOdidTimeAcc02Second = 2,
    /// The timestamp accuracy is smaller than or equal to 0.3 second.
    MavOdidTimeAcc03Second = 3,
    /// The timestamp accuracy is smaller than or equal to 0.4 second.
    MavOdidTimeAcc04Second = 4,
    /// The timestamp accuracy is smaller than or equal to 0.5 second.
    MavOdidTimeAcc05Second = 5,
    /// The timestamp accuracy is smaller than or equal to 0.6 second.
    MavOdidTimeAcc06Second = 6,
    /// The timestamp accuracy is smaller than or equal to 0.7 second.
    MavOdidTimeAcc07Second = 7,
    /// The timestamp accuracy is smaller than or equal to 0.8 second.
    MavOdidTimeAcc08Second = 8,
    /// The timestamp accuracy is smaller than or equal to 0.9 second.
    MavOdidTimeAcc09Second = 9,
    /// The timestamp accuracy is smaller than or equal to 1.0 second.
    MavOdidTimeAcc10Second = 10,
    /// The timestamp accuracy is smaller than or equal to 1.1 second.
    MavOdidTimeAcc11Second = 11,
    /// The timestamp accuracy is smaller than or equal to 1.2 second.
    MavOdidTimeAcc12Second = 12,
    /// The timestamp accuracy is smaller than or equal to 1.3 second.
    MavOdidTimeAcc13Second = 13,
    /// The timestamp accuracy is smaller than or equal to 1.4 second.
    MavOdidTimeAcc14Second = 14,
    /// The timestamp accuracy is smaller than or equal to 1.5 second.
    MavOdidTimeAcc15Second = 15,
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
pub enum MavOdidAuthType {
    /// No authentication type is specified.
    None = 0,
    /// Signature for the UAS (Unmanned Aircraft System) ID.
    UasIdSignature = 1,
    /// Signature for the Operator ID.
    OperatorIdSignature = 2,
    /// Signature for the entire message set.
    MessageSetSignature = 3,
    /// Authentication is provided by Network Remote ID.
    NetworkRemoteId = 4,
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
pub enum MavOdidDescType {
    /// Free-form text description of the purpose of the flight.
    Text = 0,
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
pub enum MavOdidOperatorLocationType {
    /// The location of the operator is the same as the take-off location.
    Takeoff = 0,
    /// The location of the operator is based on live GNSS data.
    LiveGnss = 1,
    /// The location of the operator is a fixed location.
    Fixed = 2,
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
pub enum MavOdidClassificationType {
    /// The classification type for the UA is undeclared.
    Undeclared = 0,
    /// The classification type for the UA follows EU (European Union) specifications.
    Eu = 1,
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
pub enum MavOdidCategoryEu {
    /// The category for the UA, according to the EU specification, is undeclared.
    Undeclared = 0,
    /// The category for the UA, according to the EU specification, is the Open category.
    Open = 1,
    /// The category for the UA, according to the EU specification, is the Specific category.
    Specific = 2,
    /// The category for the UA, according to the EU specification, is the Certified category.
    Certified = 3,
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
pub enum MavOdidClassEu {
    /// The class for the UA, according to the EU specification, is undeclared.
    Undeclared = 0,
    /// The class for the UA, according to the EU specification, is Class 0.
    Class0 = 1,
    /// The class for the UA, according to the EU specification, is Class 1.
    Class1 = 2,
    /// The class for the UA, according to the EU specification, is Class 2.
    Class2 = 3,
    /// The class for the UA, according to the EU specification, is Class 3.
    Class3 = 4,
    /// The class for the UA, according to the EU specification, is Class 4.
    Class4 = 5,
    /// The class for the UA, according to the EU specification, is Class 5.
    Class5 = 6,
    /// The class for the UA, according to the EU specification, is Class 6.
    Class6 = 7,
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
pub enum MavOdidOperatorIdType {
    /// CAA (Civil Aviation Authority) registered operator ID.
    Caa = 0,
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
pub enum TuneFormat {
    /// Tune formats (used for vehicle buzzer/tone generation).
    /// This enum is used to define bitmasks (work around protobuf limitations).
    /// Not used in MavLink, make protobuf happy.
    Undefined = 0,
    /// Format is QBasic 1.1 Play: https://www.qbasic.net/en/reference/qb11/Statement/PLAY-006.htm.
    /// bit 1
    Qbasic11 = 1,
    /// Format is Modern Music Markup Language (MML): https://en.wikipedia.org/wiki/Music_Macro_Language#Modern_MML.
    /// bit 2
    MmlModern = 2,
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
pub enum ComponentCapFlags {
    /// Component capability flags (Bitmap)
    /// This enum is used to define bitmasks (work around protobuf limitations).
    /// Not used in MavLink, make protobuf happy.
    Undefined = 0,
    /// Component has parameters, and supports the parameter protocol (PARAM messages).
    /// bit 1
    Param = 1,
    /// Component has parameters, and supports the extended parameter protocol (PARAM_EXT messages).
    /// bit 2
    ParamExt = 2,
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
pub enum AisType {
    /// Type of AIS vessel, enum duplicated from AIS standard, https://gpsd.gitlab.io/gpsd/AIVDM.html
    /// Not available (default).
    Unknown = 0,
    Reserved1 = 1,
    Reserved2 = 2,
    Reserved3 = 3,
    Reserved4 = 4,
    Reserved5 = 5,
    Reserved6 = 6,
    Reserved7 = 7,
    Reserved8 = 8,
    Reserved9 = 9,
    Reserved10 = 10,
    Reserved11 = 11,
    Reserved12 = 12,
    Reserved13 = 13,
    Reserved14 = 14,
    Reserved15 = 15,
    Reserved16 = 16,
    Reserved17 = 17,
    Reserved18 = 18,
    Reserved19 = 19,
    /// Wing In Ground effect.
    Wig = 20,
    WigHazardousA = 21,
    WigHazardousB = 22,
    WigHazardousC = 23,
    WigHazardousD = 24,
    WigReserved1 = 25,
    WigReserved2 = 26,
    WigReserved3 = 27,
    WigReserved4 = 28,
    WigReserved5 = 29,
    Fishing = 30,
    Towing = 31,
    /// Towing: length exceeds 200m or breadth exceeds 25m.
    TowingLarge = 32,
    /// Dredging or other underwater ops.
    Dredging = 33,
    Diving = 34,
    Military = 35,
    Sailing = 36,
    Pleasure = 37,
    Reserved20 = 38,
    Reserved21 = 39,
    /// High Speed Craft.
    Hsc = 40,
    HscHazardousA = 41,
    HscHazardousB = 42,
    HscHazardousC = 43,
    HscHazardousD = 44,
    HscReserved1 = 45,
    HscReserved2 = 46,
    HscReserved3 = 47,
    HscReserved4 = 48,
    HscUnknown = 49,
    Pilot = 50,
    /// Search And Rescue vessel.
    Sar = 51,
    Tug = 52,
    PortTender = 53,
    /// Anti-pollution equipment.
    AntiPollution = 54,
    LawEnforcement = 55,
    SpareLocal1 = 56,
    SpareLocal2 = 57,
    MedicalTransport = 58,
    /// Noncombatant ship according to RR Resolution No. 18.
    Nonecombatant = 59,
    Passenger = 60,
    PassengerHazardousA = 61,
    PassengerHazardousB = 62,
    AisTypePassengerHazardousC = 63,
    PassengerHazardousD = 64,
    PassengerReserved1 = 65,
    PassengerReserved2 = 66,
    PassengerReserved3 = 67,
    AisTypePassengerReserved4 = 68,
    PassengerUnknown = 69,
    Cargo = 70,
    CargoHazardousA = 71,
    CargoHazardousB = 72,
    CargoHazardousC = 73,
    CargoHazardousD = 74,
    CargoReserved1 = 75,
    CargoReserved2 = 76,
    CargoReserved3 = 77,
    CargoReserved4 = 78,
    CargoUnknown = 79,
    Tanker = 80,
    TankerHazardousA = 81,
    TankerHazardousB = 82,
    TankerHazardousC = 83,
    TankerHazardousD = 84,
    TankerReserved1 = 85,
    TankerReserved2 = 86,
    TankerReserved3 = 87,
    TankerReserved4 = 88,
    TankerUnknown = 89,
    Other = 90,
    OtherHazardousA = 91,
    OtherHazardousB = 92,
    OtherHazardousC = 93,
    OtherHazardousD = 94,
    OtherReserved1 = 95,
    OtherReserved2 = 96,
    OtherReserved3 = 97,
    OtherReserved4 = 98,
    OtherUnknown = 99,
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
pub enum AisNavStatus {
    /// Navigational status of AIS vessel, enum duplicated from AIS standard, https://gpsd.gitlab.io/gpsd/AIVDM.html
    /// Under way using engine.
    UnderWay = 0,
    AisNavAnchored = 1,
    AisNavUnCommanded = 2,
    AisNavRestrictedManoeuverability = 3,
    AisNavDraughtConstrained = 4,
    AisNavMoored = 5,
    AisNavAground = 6,
    AisNavFishing = 7,
    AisNavSailing = 8,
    AisNavReservedHsc = 9,
    AisNavReservedWig = 10,
    AisNavReserved1 = 11,
    AisNavReserved2 = 12,
    AisNavReserved3 = 13,
    /// Search And Rescue Transponder.
    AisNavAisSart = 14,
    /// Not available (default).
    AisNavUnknown = 15,
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
pub enum AisFlags {
    /// These flags are used in the AIS_VESSEL.fields bitmask to indicate validity of data in the other message fields. When set, the data is valid.
    /// This enum is used to define bitmasks (work around protobuf limitations).
    /// Not used in MavLink, make protobuf happy.
    Undefined = 0,
    /// 1 = Position accuracy less than 10m, 0 = position accuracy greater than 10m.
    /// bit 1
    PositionAccuracy = 1,
    /// bit 2
    ValidCog = 2,
    /// bit 3
    ValidVelocity = 4,
    /// 1 = Velocity over 52.5765m/s (102.2 knots)
    /// bit 4
    HighVelocity = 8,
    /// bit 5
    ValidTurnRate = 16,
    /// Only the sign of the returned turn rate value is valid, either greater than 5deg/30s or less than -5deg/30s
    /// bit 6
    TurnRateSignOnly = 32,
    /// bit 7
    ValidDimensions = 64,
    /// Distance to bow is larger than 511m
    /// bit 8
    LargeBowDimension = 128,
    /// Distance to stern is larger than 511m
    /// bit 9
    LargeSternDimension = 256,
    /// Distance to port side is larger than 63m
    /// bit 10
    LargePortDimension = 512,
    /// Distance to starboard side is larger than 63m
    /// bit 11
    LargeStarboardDimension = 1024,
    /// bit 12
    ValidCallsign = 2048,
    /// bit 13
    ValidName = 4096,
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
pub enum FailureUnit {
    /// List of possible units where failures can be injected.
    SensorGyro = 0,
    SensorAccel = 1,
    SensorMag = 2,
    SensorBaro = 3,
    SensorGps = 4,
    SensorOpticalFlow = 5,
    SensorVio = 6,
    SensorDistanceSensor = 7,
    SensorAirspeed = 8,
    SystemBattery = 100,
    SystemMotor = 101,
    SystemServo = 102,
    SystemAvoidance = 103,
    SystemRcSignal = 104,
    SystemMavlinkSignal = 105,
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
pub enum FailureType {
    /// List of possible failure type to inject.
    /// No failure injected, used to reset a previous failure.
    Ok = 0,
    /// Sets unit off, so completely non-responsive.
    Off = 1,
    /// Unit is stuck e.g. keeps reporting the same value.
    Stuck = 2,
    /// Unit is reporting complete garbage.
    Garbage = 3,
    /// Unit is consistently wrong.
    Wrong = 4,
    /// Unit is slow, so e.g. reporting at slower than expected rate.
    Slow = 5,
    /// Data of unit is delayed in time.
    Delayed = 6,
    /// Unit is sometimes working, sometimes not.
    Intermittent = 7,
}
