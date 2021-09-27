/// Offsets and calibrations values for hardware sensors. This makes it easier to debug the calibration process.
///
/// MavLink id: 150
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SensorOffsets {
    /// Magnetic declination.
    #[prost(float, tag = "1")]
    pub mag_declination: f32,
    /// Raw pressure from barometer.
    #[prost(int32, tag = "2")]
    pub raw_press: i32,
    /// Raw temperature from barometer.
    #[prost(int32, tag = "3")]
    pub raw_temp: i32,
    /// Gyro X calibration.
    #[prost(float, tag = "4")]
    pub gyro_cal_x: f32,
    /// Gyro Y calibration.
    #[prost(float, tag = "5")]
    pub gyro_cal_y: f32,
    /// Gyro Z calibration.
    #[prost(float, tag = "6")]
    pub gyro_cal_z: f32,
    /// Accel X calibration.
    #[prost(float, tag = "7")]
    pub accel_cal_x: f32,
    /// Accel Y calibration.
    #[prost(float, tag = "8")]
    pub accel_cal_y: f32,
    /// Accel Z calibration.
    #[prost(float, tag = "9")]
    pub accel_cal_z: f32,
    /// Magnetometer X offset.
    #[prost(int32, tag = "10")]
    pub mag_ofs_x: i32,
    /// Magnetometer Y offset.
    #[prost(int32, tag = "11")]
    pub mag_ofs_y: i32,
    /// Magnetometer Z offset.
    #[prost(int32, tag = "12")]
    pub mag_ofs_z: i32,
}
/// Set the magnetometer offsets
///
/// MavLink id: 151
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SetMagOffsets {
    /// Magnetometer X offset.
    #[prost(int32, tag = "1")]
    pub mag_ofs_x: i32,
    /// Magnetometer Y offset.
    #[prost(int32, tag = "2")]
    pub mag_ofs_y: i32,
    /// Magnetometer Z offset.
    #[prost(int32, tag = "3")]
    pub mag_ofs_z: i32,
    /// System ID.
    #[prost(uint32, tag = "4")]
    pub target_system: u32,
    /// Component ID.
    #[prost(uint32, tag = "5")]
    pub target_component: u32,
}
/// State of APM memory.
///
/// MavLink id: 152
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Meminfo {
    /// Heap top.
    #[prost(uint32, tag = "1")]
    pub brkval: u32,
    /// Free memory.
    #[prost(uint32, tag = "2")]
    pub freemem: u32,
}
/// Raw ADC output.
///
/// MavLink id: 153
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ApAdc {
    /// ADC output 1.
    #[prost(uint32, tag = "1")]
    pub adc1: u32,
    /// ADC output 2.
    #[prost(uint32, tag = "2")]
    pub adc2: u32,
    /// ADC output 3.
    #[prost(uint32, tag = "3")]
    pub adc3: u32,
    /// ADC output 4.
    #[prost(uint32, tag = "4")]
    pub adc4: u32,
    /// ADC output 5.
    #[prost(uint32, tag = "5")]
    pub adc5: u32,
    /// ADC output 6.
    #[prost(uint32, tag = "6")]
    pub adc6: u32,
}
/// Configure on-board Camera Control System.
///
/// MavLink id: 154
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct DigicamConfigure {
    /// Correspondent value to given extra_param.
    #[prost(float, tag = "1")]
    pub extra_value: f32,
    /// Divisor number //e.g. 1000 means 1/1000 (0 means ignore).
    #[prost(uint32, tag = "2")]
    pub shutter_speed: u32,
    /// System ID.
    #[prost(uint32, tag = "3")]
    pub target_system: u32,
    /// Component ID.
    #[prost(uint32, tag = "4")]
    pub target_component: u32,
    /// Mode enumeration from 1 to N //P, TV, AV, M, etc. (0 means ignore).
    #[prost(uint32, tag = "5")]
    pub mode: u32,
    /// F stop number x 10 //e.g. 28 means 2.8 (0 means ignore).
    #[prost(uint32, tag = "6")]
    pub aperture: u32,
    /// ISO enumeration from 1 to N //e.g. 80, 100, 200, Etc (0 means ignore).
    #[prost(uint32, tag = "7")]
    pub iso: u32,
    /// Exposure type enumeration from 1 to N (0 means ignore).
    #[prost(uint32, tag = "8")]
    pub exposure_type: u32,
    /// Command Identity (incremental loop: 0 to 255). //A command sent multiple times will be executed or pooled just once.
    #[prost(uint32, tag = "9")]
    pub command_id: u32,
    /// Main engine cut-off time before camera trigger (0 means no cut-off).
    #[prost(uint32, tag = "10")]
    pub engine_cut_off: u32,
    /// Extra parameters enumeration (0 means ignore).
    #[prost(uint32, tag = "11")]
    pub extra_param: u32,
}
/// Control on-board Camera Control System to take shots.
///
/// MavLink id: 155
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct DigicamControl {
    /// Correspondent value to given extra_param.
    #[prost(float, tag = "1")]
    pub extra_value: f32,
    /// System ID.
    #[prost(uint32, tag = "2")]
    pub target_system: u32,
    /// Component ID.
    #[prost(uint32, tag = "3")]
    pub target_component: u32,
    /// 0: stop, 1: start or keep it up //Session control e.g. show/hide lens.
    #[prost(uint32, tag = "4")]
    pub session: u32,
    /// 1 to N //Zoom's absolute position (0 means ignore).
    #[prost(uint32, tag = "5")]
    pub zoom_pos: u32,
    /// -100 to 100 //Zooming step value to offset zoom from the current position.
    #[prost(int32, tag = "6")]
    pub zoom_step: i32,
    /// 0: unlock focus or keep unlocked, 1: lock focus or keep locked, 3: re-lock focus.
    #[prost(uint32, tag = "7")]
    pub focus_lock: u32,
    /// 0: ignore, 1: shot or start filming.
    #[prost(uint32, tag = "8")]
    pub shot: u32,
    /// Command Identity (incremental loop: 0 to 255)//A command sent multiple times will be executed or pooled just once.
    #[prost(uint32, tag = "9")]
    pub command_id: u32,
    /// Extra parameters enumeration (0 means ignore).
    #[prost(uint32, tag = "10")]
    pub extra_param: u32,
}
/// Message to configure a camera mount, directional antenna, etc.
///
/// MavLink id: 156
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MountConfigure {
    /// System ID.
    #[prost(uint32, tag = "1")]
    pub target_system: u32,
    /// Component ID.
    #[prost(uint32, tag = "2")]
    pub target_component: u32,
    /// Mount operating mode.
    #[prost(enumeration = "super::common::MavMountMode", tag = "3")]
    pub mount_mode: i32,
    /// (1 = yes, 0 = no).
    #[prost(uint32, tag = "4")]
    pub stab_roll: u32,
    /// (1 = yes, 0 = no).
    #[prost(uint32, tag = "5")]
    pub stab_pitch: u32,
    /// (1 = yes, 0 = no).
    #[prost(uint32, tag = "6")]
    pub stab_yaw: u32,
}
/// Message to control a camera mount, directional antenna, etc.
///
/// MavLink id: 157
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MountControl {
    /// Pitch (centi-degrees) or lat (degE7), depending on mount mode.
    #[prost(int32, tag = "1")]
    pub input_a: i32,
    /// Roll (centi-degrees) or lon (degE7) depending on mount mode.
    #[prost(int32, tag = "2")]
    pub input_b: i32,
    /// Yaw (centi-degrees) or alt (cm) depending on mount mode.
    #[prost(int32, tag = "3")]
    pub input_c: i32,
    /// System ID.
    #[prost(uint32, tag = "4")]
    pub target_system: u32,
    /// Component ID.
    #[prost(uint32, tag = "5")]
    pub target_component: u32,
    /// If "1" it will save current trimmed position on EEPROM (just valid for NEUTRAL and LANDING).
    #[prost(uint32, tag = "6")]
    pub save_position: u32,
}
/// Message with some status from APM to GCS about camera or antenna mount.
///
/// MavLink id: 158
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MountStatus {
    /// Pitch.
    #[prost(int32, tag = "1")]
    pub pointing_a: i32,
    /// Roll.
    #[prost(int32, tag = "2")]
    pub pointing_b: i32,
    /// Yaw.
    #[prost(int32, tag = "3")]
    pub pointing_c: i32,
    /// System ID.
    #[prost(uint32, tag = "4")]
    pub target_system: u32,
    /// Component ID.
    #[prost(uint32, tag = "5")]
    pub target_component: u32,
}
/// A fence point. Used to set a point when from GCS -> MAV. Also used to return a point from MAV -> GCS.
///
/// MavLink id: 160
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct FencePoint {
    /// Latitude of point.
    #[prost(float, tag = "1")]
    pub lat: f32,
    /// Longitude of point.
    #[prost(float, tag = "2")]
    pub lng: f32,
    /// System ID.
    #[prost(uint32, tag = "3")]
    pub target_system: u32,
    /// Component ID.
    #[prost(uint32, tag = "4")]
    pub target_component: u32,
    /// Point index (first point is 1, 0 is for return point).
    #[prost(uint32, tag = "5")]
    pub idx: u32,
    /// Total number of points (for sanity checking).
    #[prost(uint32, tag = "6")]
    pub count: u32,
}
/// Request a current fence point from MAV.
///
/// MavLink id: 161
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct FenceFetchPoint {
    /// System ID.
    #[prost(uint32, tag = "1")]
    pub target_system: u32,
    /// Component ID.
    #[prost(uint32, tag = "2")]
    pub target_component: u32,
    /// Point index (first point is 1, 0 is for return point).
    #[prost(uint32, tag = "3")]
    pub idx: u32,
}
/// Status of DCM attitude estimator.
///
/// MavLink id: 163
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Ahrs {
    /// X gyro drift estimate.
    #[prost(float, tag = "1")]
    pub omega_ix: f32,
    /// Y gyro drift estimate.
    #[prost(float, tag = "2")]
    pub omega_iy: f32,
    /// Z gyro drift estimate.
    #[prost(float, tag = "3")]
    pub omega_iz: f32,
    /// Average accel_weight.
    #[prost(float, tag = "4")]
    pub accel_weight: f32,
    /// Average renormalisation value.
    #[prost(float, tag = "5")]
    pub renorm_val: f32,
    /// Average error_roll_pitch value.
    #[prost(float, tag = "6")]
    pub error_rp: f32,
    /// Average error_yaw value.
    #[prost(float, tag = "7")]
    pub error_yaw: f32,
}
/// Status of simulation environment, if used.
///
/// MavLink id: 164
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Simstate {
    /// Roll angle.
    #[prost(float, tag = "1")]
    pub roll: f32,
    /// Pitch angle.
    #[prost(float, tag = "2")]
    pub pitch: f32,
    /// Yaw angle.
    #[prost(float, tag = "3")]
    pub yaw: f32,
    /// X acceleration.
    #[prost(float, tag = "4")]
    pub xacc: f32,
    /// Y acceleration.
    #[prost(float, tag = "5")]
    pub yacc: f32,
    /// Z acceleration.
    #[prost(float, tag = "6")]
    pub zacc: f32,
    /// Angular speed around X axis.
    #[prost(float, tag = "7")]
    pub xgyro: f32,
    /// Angular speed around Y axis.
    #[prost(float, tag = "8")]
    pub ygyro: f32,
    /// Angular speed around Z axis.
    #[prost(float, tag = "9")]
    pub zgyro: f32,
    /// Latitude.
    #[prost(int32, tag = "10")]
    pub lat: i32,
    /// Longitude.
    #[prost(int32, tag = "11")]
    pub lng: i32,
}
/// Status of key hardware.
///
/// MavLink id: 165
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Hwstatus {
    /// Board voltage.
    #[prost(uint32, tag = "1")]
    pub vcc: u32,
    /// I2C error count.
    #[prost(uint32, tag = "2")]
    pub i2_cerr: u32,
}
/// Status generated by radio.
///
/// MavLink id: 166
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Radio {
    /// Receive errors.
    #[prost(uint32, tag = "1")]
    pub rxerrors: u32,
    /// Count of error corrected packets.
    #[prost(uint32, tag = "2")]
    pub fixed: u32,
    /// Local signal strength.
    #[prost(uint32, tag = "3")]
    pub rssi: u32,
    /// Remote signal strength.
    #[prost(uint32, tag = "4")]
    pub remrssi: u32,
    /// How full the tx buffer is.
    #[prost(uint32, tag = "5")]
    pub txbuf: u32,
    /// Background noise level.
    #[prost(uint32, tag = "6")]
    pub noise: u32,
    /// Remote background noise level.
    #[prost(uint32, tag = "7")]
    pub remnoise: u32,
}
/// Status of AP_Limits. Sent in extended status stream when AP_Limits is enabled.
///
/// MavLink id: 167
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct LimitsStatus {
    /// Time (since boot) of last breach.
    #[prost(uint32, tag = "1")]
    pub last_trigger: u32,
    /// Time (since boot) of last recovery action.
    #[prost(uint32, tag = "2")]
    pub last_action: u32,
    /// Time (since boot) of last successful recovery.
    #[prost(uint32, tag = "3")]
    pub last_recovery: u32,
    /// Time (since boot) of last all-clear.
    #[prost(uint32, tag = "4")]
    pub last_clear: u32,
    /// Number of fence breaches.
    #[prost(uint32, tag = "5")]
    pub breach_count: u32,
    /// State of AP_Limits.
    #[prost(enumeration = "LimitsState", tag = "6")]
    pub limits_state: i32,
    /// AP_Limit_Module bitfield of enabled modules.
    /// bitfield defined by enum LIMIT_MODULE
    #[prost(uint32, tag = "7")]
    pub mods_enabled: u32,
    /// AP_Limit_Module bitfield of required modules.
    /// bitfield defined by enum LIMIT_MODULE
    #[prost(uint32, tag = "8")]
    pub mods_required: u32,
    /// AP_Limit_Module bitfield of triggered modules.
    /// bitfield defined by enum LIMIT_MODULE
    #[prost(uint32, tag = "9")]
    pub mods_triggered: u32,
}
/// Wind estimation.
///
/// MavLink id: 168
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Wind {
    /// Wind direction (that wind is coming from).
    #[prost(float, tag = "1")]
    pub direction: f32,
    /// Wind speed in ground plane.
    #[prost(float, tag = "2")]
    pub speed: f32,
    /// Vertical wind speed.
    #[prost(float, tag = "3")]
    pub speed_z: f32,
}
/// Data packet, size 16.
///
/// MavLink id: 169
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Data16 {
    /// Data type.
    #[prost(uint32, tag = "1")]
    pub r#type: u32,
    /// Data length.
    #[prost(uint32, tag = "2")]
    pub len: u32,
    /// Raw data.
    #[prost(uint32, repeated, packed = "false", tag = "3")]
    pub data: ::prost::alloc::vec::Vec<u32>,
}
/// Data packet, size 32.
///
/// MavLink id: 170
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Data32 {
    /// Data type.
    #[prost(uint32, tag = "1")]
    pub r#type: u32,
    /// Data length.
    #[prost(uint32, tag = "2")]
    pub len: u32,
    /// Raw data.
    #[prost(uint32, repeated, packed = "false", tag = "3")]
    pub data: ::prost::alloc::vec::Vec<u32>,
}
/// Data packet, size 64.
///
/// MavLink id: 171
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Data64 {
    /// Data type.
    #[prost(uint32, tag = "1")]
    pub r#type: u32,
    /// Data length.
    #[prost(uint32, tag = "2")]
    pub len: u32,
    /// Raw data.
    #[prost(uint32, repeated, packed = "false", tag = "3")]
    pub data: ::prost::alloc::vec::Vec<u32>,
}
/// Data packet, size 96.
///
/// MavLink id: 172
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Data96 {
    /// Data type.
    #[prost(uint32, tag = "1")]
    pub r#type: u32,
    /// Data length.
    #[prost(uint32, tag = "2")]
    pub len: u32,
    /// Raw data.
    #[prost(uint32, repeated, packed = "false", tag = "3")]
    pub data: ::prost::alloc::vec::Vec<u32>,
}
/// Rangefinder reporting.
///
/// MavLink id: 173
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Rangefinder {
    /// Distance.
    #[prost(float, tag = "1")]
    pub distance: f32,
    /// Raw voltage if available, zero otherwise.
    #[prost(float, tag = "2")]
    pub voltage: f32,
}
/// Airspeed auto-calibration.
///
/// MavLink id: 174
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct AirspeedAutocal {
    /// GPS velocity north.
    #[prost(float, tag = "1")]
    pub vx: f32,
    /// GPS velocity east.
    #[prost(float, tag = "2")]
    pub vy: f32,
    /// GPS velocity down.
    #[prost(float, tag = "3")]
    pub vz: f32,
    /// Differential pressure.
    #[prost(float, tag = "4")]
    pub diff_pressure: f32,
    /// Estimated to true airspeed ratio.
    #[prost(float, tag = "5")]
    pub eas2tas: f32,
    /// Airspeed ratio.
    #[prost(float, tag = "6")]
    pub ratio: f32,
    /// EKF state x.
    #[prost(float, tag = "7")]
    pub state_x: f32,
    /// EKF state y.
    #[prost(float, tag = "8")]
    pub state_y: f32,
    /// EKF state z.
    #[prost(float, tag = "9")]
    pub state_z: f32,
    /// EKF Pax.
    #[prost(float, tag = "10")]
    pub pax: f32,
    /// EKF Pby.
    #[prost(float, tag = "11")]
    pub pby: f32,
    /// EKF Pcz.
    #[prost(float, tag = "12")]
    pub pcz: f32,
}
/// A rally point. Used to set a point when from GCS -> MAV. Also used to return a point from MAV -> GCS.
///
/// MavLink id: 175
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct RallyPoint {
    /// Latitude of point.
    #[prost(int32, tag = "1")]
    pub lat: i32,
    /// Longitude of point.
    #[prost(int32, tag = "2")]
    pub lng: i32,
    /// Transit / loiter altitude relative to home.
    #[prost(int32, tag = "3")]
    pub alt: i32,
    /// Break altitude relative to home.
    #[prost(int32, tag = "4")]
    pub break_alt: i32,
    /// Heading to aim for when landing.
    #[prost(uint32, tag = "5")]
    pub land_dir: u32,
    /// System ID.
    #[prost(uint32, tag = "6")]
    pub target_system: u32,
    /// Component ID.
    #[prost(uint32, tag = "7")]
    pub target_component: u32,
    /// Point index (first point is 0).
    #[prost(uint32, tag = "8")]
    pub idx: u32,
    /// Total number of points (for sanity checking).
    #[prost(uint32, tag = "9")]
    pub count: u32,
    /// Configuration flags.
    /// bitfield defined by enum RALLY_FLAGS
    #[prost(uint32, tag = "10")]
    pub flags: u32,
}
/// Request a current rally point from MAV. MAV should respond with a RALLY_POINT message. MAV should not respond if the request is invalid.
///
/// MavLink id: 176
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct RallyFetchPoint {
    /// System ID.
    #[prost(uint32, tag = "1")]
    pub target_system: u32,
    /// Component ID.
    #[prost(uint32, tag = "2")]
    pub target_component: u32,
    /// Point index (first point is 0).
    #[prost(uint32, tag = "3")]
    pub idx: u32,
}
/// Status of compassmot calibration.
///
/// MavLink id: 177
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CompassmotStatus {
    /// Current.
    #[prost(float, tag = "1")]
    pub current: f32,
    /// Motor Compensation X.
    #[prost(float, tag = "2")]
    pub compensation_x: f32,
    /// Motor Compensation Y.
    #[prost(float, tag = "3")]
    pub compensation_y: f32,
    /// Motor Compensation Z.
    #[prost(float, tag = "4")]
    pub compensation_z: f32,
    /// Throttle.
    #[prost(uint32, tag = "5")]
    pub throttle: u32,
    /// Interference.
    #[prost(uint32, tag = "6")]
    pub interference: u32,
}
/// Status of secondary AHRS filter if available.
///
/// MavLink id: 178
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Ahrs2 {
    /// Roll angle.
    #[prost(float, tag = "1")]
    pub roll: f32,
    /// Pitch angle.
    #[prost(float, tag = "2")]
    pub pitch: f32,
    /// Yaw angle.
    #[prost(float, tag = "3")]
    pub yaw: f32,
    /// Altitude (MSL).
    #[prost(float, tag = "4")]
    pub altitude: f32,
    /// Latitude.
    #[prost(int32, tag = "5")]
    pub lat: i32,
    /// Longitude.
    #[prost(int32, tag = "6")]
    pub lng: i32,
}
/// Camera Event.
///
/// MavLink id: 179
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CameraStatus {
    /// Image timestamp (since UNIX epoch, according to camera clock).
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// Parameter 1 (meaning depends on event_id, see CAMERA_STATUS_TYPES enum).
    #[prost(float, tag = "2")]
    pub p1: f32,
    /// Parameter 2 (meaning depends on event_id, see CAMERA_STATUS_TYPES enum).
    #[prost(float, tag = "3")]
    pub p2: f32,
    /// Parameter 3 (meaning depends on event_id, see CAMERA_STATUS_TYPES enum).
    #[prost(float, tag = "4")]
    pub p3: f32,
    /// Parameter 4 (meaning depends on event_id, see CAMERA_STATUS_TYPES enum).
    #[prost(float, tag = "5")]
    pub p4: f32,
    /// Image index.
    #[prost(uint32, tag = "6")]
    pub img_idx: u32,
    /// System ID.
    #[prost(uint32, tag = "7")]
    pub target_system: u32,
    /// Camera ID.
    #[prost(uint32, tag = "8")]
    pub cam_idx: u32,
    /// Event type.
    #[prost(enumeration = "CameraStatusTypes", tag = "9")]
    pub event_id: i32,
}
/// Camera Capture Feedback.
///
/// MavLink id: 180
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CameraFeedback {
    /// Image timestamp (since UNIX epoch), as passed in by CAMERA_STATUS message (or autopilot if no CCB).
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// Latitude.
    #[prost(int32, tag = "2")]
    pub lat: i32,
    /// Longitude.
    #[prost(int32, tag = "3")]
    pub lng: i32,
    /// Altitude (MSL).
    #[prost(float, tag = "4")]
    pub alt_msl: f32,
    /// Altitude (Relative to HOME location).
    #[prost(float, tag = "5")]
    pub alt_rel: f32,
    /// Camera Roll angle (earth frame, +-180).
    #[prost(float, tag = "6")]
    pub roll: f32,
    /// Camera Pitch angle (earth frame, +-180).
    #[prost(float, tag = "7")]
    pub pitch: f32,
    /// Camera Yaw (earth frame, 0-360, true).
    #[prost(float, tag = "8")]
    pub yaw: f32,
    /// Focal Length.
    #[prost(float, tag = "9")]
    pub foc_len: f32,
    /// Image index.
    #[prost(uint32, tag = "10")]
    pub img_idx: u32,
    /// System ID.
    #[prost(uint32, tag = "11")]
    pub target_system: u32,
    /// Camera ID.
    #[prost(uint32, tag = "12")]
    pub cam_idx: u32,
    /// Feedback flags.
    #[prost(enumeration = "CameraFeedbackFlags", tag = "13")]
    pub flags: i32,
}
/// 2nd Battery status
///
/// MavLink id: 181
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Battery2 {
    /// Voltage.
    #[prost(uint32, tag = "1")]
    pub voltage: u32,
    /// Battery current, -1: autopilot does not measure the current.
    #[prost(int32, tag = "2")]
    pub current_battery: i32,
}
/// Status of third AHRS filter if available. This is for ANU research group (Ali and Sean).
///
/// MavLink id: 182
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Ahrs3 {
    /// Roll angle.
    #[prost(float, tag = "1")]
    pub roll: f32,
    /// Pitch angle.
    #[prost(float, tag = "2")]
    pub pitch: f32,
    /// Yaw angle.
    #[prost(float, tag = "3")]
    pub yaw: f32,
    /// Altitude (MSL).
    #[prost(float, tag = "4")]
    pub altitude: f32,
    /// Latitude.
    #[prost(int32, tag = "5")]
    pub lat: i32,
    /// Longitude.
    #[prost(int32, tag = "6")]
    pub lng: i32,
    /// Test variable1.
    #[prost(float, tag = "7")]
    pub v1: f32,
    /// Test variable2.
    #[prost(float, tag = "8")]
    pub v2: f32,
    /// Test variable3.
    #[prost(float, tag = "9")]
    pub v3: f32,
    /// Test variable4.
    #[prost(float, tag = "10")]
    pub v4: f32,
}
/// Request the autopilot version from the system/component.
///
/// MavLink id: 183
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct AutopilotVersionRequest {
    /// System ID.
    #[prost(uint32, tag = "1")]
    pub target_system: u32,
    /// Component ID.
    #[prost(uint32, tag = "2")]
    pub target_component: u32,
}
/// Send a block of log data to remote location.
///
/// MavLink id: 184
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct RemoteLogDataBlock {
    /// Log data block sequence number.
    #[prost(enumeration = "MavRemoteLogDataBlockCommands", tag = "1")]
    pub seqno: i32,
    /// System ID.
    #[prost(uint32, tag = "2")]
    pub target_system: u32,
    /// Component ID.
    #[prost(uint32, tag = "3")]
    pub target_component: u32,
    /// Log data block.
    #[prost(uint32, repeated, packed = "false", tag = "4")]
    pub data: ::prost::alloc::vec::Vec<u32>,
}
/// Send Status of each log block that autopilot board might have sent.
///
/// MavLink id: 185
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct RemoteLogBlockStatus {
    /// Log data block sequence number.
    #[prost(uint32, tag = "1")]
    pub seqno: u32,
    /// System ID.
    #[prost(uint32, tag = "2")]
    pub target_system: u32,
    /// Component ID.
    #[prost(uint32, tag = "3")]
    pub target_component: u32,
    /// Log data block status.
    #[prost(enumeration = "MavRemoteLogDataBlockStatuses", tag = "4")]
    pub status: i32,
}
/// Control vehicle LEDs.
///
/// MavLink id: 186
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct LedControl {
    /// System ID.
    #[prost(uint32, tag = "1")]
    pub target_system: u32,
    /// Component ID.
    #[prost(uint32, tag = "2")]
    pub target_component: u32,
    /// Instance (LED instance to control or 255 for all LEDs).
    #[prost(uint32, tag = "3")]
    pub instance: u32,
    /// Pattern (see LED_PATTERN_ENUM).
    #[prost(uint32, tag = "4")]
    pub pattern: u32,
    /// Custom Byte Length.
    #[prost(uint32, tag = "5")]
    pub custom_len: u32,
    /// Custom Bytes.
    #[prost(uint32, repeated, packed = "false", tag = "6")]
    pub custom_bytes: ::prost::alloc::vec::Vec<u32>,
}
/// Reports progress of compass calibration.
///
/// MavLink id: 191
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MagCalProgress {
    /// Body frame direction vector for display.
    #[prost(float, tag = "1")]
    pub direction_x: f32,
    /// Body frame direction vector for display.
    #[prost(float, tag = "2")]
    pub direction_y: f32,
    /// Body frame direction vector for display.
    #[prost(float, tag = "3")]
    pub direction_z: f32,
    /// Compass being calibrated.
    #[prost(uint32, tag = "4")]
    pub compass_id: u32,
    /// Bitmask of compasses being calibrated.
    #[prost(uint32, tag = "5")]
    pub cal_mask: u32,
    /// Calibration Status.
    #[prost(enumeration = "MagCalStatus", tag = "6")]
    pub cal_status: i32,
    /// Attempt number.
    #[prost(uint32, tag = "7")]
    pub attempt: u32,
    /// Completion percentage.
    #[prost(uint32, tag = "8")]
    pub completion_pct: u32,
    /// Bitmask of sphere sections (see http://en.wikipedia.org/wiki/Geodesic_grid).
    #[prost(uint32, repeated, packed = "false", tag = "9")]
    pub completion_mask: ::prost::alloc::vec::Vec<u32>,
}
/// Reports results of completed compass calibration. Sent until MAG_CAL_ACK received.
///
/// MavLink id: 192
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MagCalReport {
    /// RMS milligauss residuals.
    #[prost(float, tag = "1")]
    pub fitness: f32,
    /// X offset.
    #[prost(float, tag = "2")]
    pub ofs_x: f32,
    /// Y offset.
    #[prost(float, tag = "3")]
    pub ofs_y: f32,
    /// Z offset.
    #[prost(float, tag = "4")]
    pub ofs_z: f32,
    /// X diagonal (matrix 11).
    #[prost(float, tag = "5")]
    pub diag_x: f32,
    /// Y diagonal (matrix 22).
    #[prost(float, tag = "6")]
    pub diag_y: f32,
    /// Z diagonal (matrix 33).
    #[prost(float, tag = "7")]
    pub diag_z: f32,
    /// X off-diagonal (matrix 12 and 21).
    #[prost(float, tag = "8")]
    pub offdiag_x: f32,
    /// Y off-diagonal (matrix 13 and 31).
    #[prost(float, tag = "9")]
    pub offdiag_y: f32,
    /// Z off-diagonal (matrix 32 and 23).
    #[prost(float, tag = "10")]
    pub offdiag_z: f32,
    /// Compass being calibrated.
    #[prost(uint32, tag = "11")]
    pub compass_id: u32,
    /// Bitmask of compasses being calibrated.
    #[prost(uint32, tag = "12")]
    pub cal_mask: u32,
    /// Calibration Status.
    #[prost(enumeration = "MagCalStatus", tag = "13")]
    pub cal_status: i32,
    /// 0=requires a MAV_CMD_DO_ACCEPT_MAG_CAL, 1=saved to parameters.
    #[prost(uint32, tag = "14")]
    pub autosaved: u32,
}
/// EKF Status message including flags and variances.
///
/// MavLink id: 193
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct EkfStatusReport {
    /// Velocity variance.
    #[prost(float, tag = "1")]
    pub velocity_variance: f32,
    /// Horizontal Position variance.
    #[prost(float, tag = "2")]
    pub pos_horiz_variance: f32,
    /// Vertical Position variance.
    #[prost(float, tag = "3")]
    pub pos_vert_variance: f32,
    /// Compass variance.
    #[prost(float, tag = "4")]
    pub compass_variance: f32,
    /// Terrain Altitude variance.
    #[prost(float, tag = "5")]
    pub terrain_alt_variance: f32,
    /// Flags.
    /// bitfield defined by enum EKF_STATUS_FLAGS
    #[prost(uint32, tag = "6")]
    pub flags: u32,
}
/// PID tuning information.
///
/// MavLink id: 194
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PidTuning {
    /// Desired rate.
    #[prost(float, tag = "1")]
    pub desired: f32,
    /// Achieved rate.
    #[prost(float, tag = "2")]
    pub achieved: f32,
    /// FF component.
    #[prost(float, tag = "3")]
    pub ff: f32,
    /// P component.
    #[prost(float, tag = "4")]
    pub p: f32,
    /// I component.
    #[prost(float, tag = "5")]
    pub i: f32,
    /// D component.
    #[prost(float, tag = "6")]
    pub d: f32,
    /// Axis.
    #[prost(enumeration = "PidTuningAxis", tag = "7")]
    pub axis: i32,
}
/// Deepstall path planning.
///
/// MavLink id: 195
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Deepstall {
    /// Landing latitude.
    #[prost(int32, tag = "1")]
    pub landing_lat: i32,
    /// Landing longitude.
    #[prost(int32, tag = "2")]
    pub landing_lon: i32,
    /// Final heading start point, latitude.
    #[prost(int32, tag = "3")]
    pub path_lat: i32,
    /// Final heading start point, longitude.
    #[prost(int32, tag = "4")]
    pub path_lon: i32,
    /// Arc entry point, latitude.
    #[prost(int32, tag = "5")]
    pub arc_entry_lat: i32,
    /// Arc entry point, longitude.
    #[prost(int32, tag = "6")]
    pub arc_entry_lon: i32,
    /// Altitude.
    #[prost(float, tag = "7")]
    pub altitude: f32,
    /// Distance the aircraft expects to travel during the deepstall.
    #[prost(float, tag = "8")]
    pub expected_travel_distance: f32,
    /// Deepstall cross track error (only valid when in DEEPSTALL_STAGE_LAND).
    #[prost(float, tag = "9")]
    pub cross_track_error: f32,
    /// Deepstall stage.
    #[prost(enumeration = "DeepstallStage", tag = "10")]
    pub stage: i32,
}
/// 3 axis gimbal measurements.
///
/// MavLink id: 200
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GimbalReport {
    /// Time since last update.
    #[prost(float, tag = "1")]
    pub delta_time: f32,
    /// Delta angle X.
    #[prost(float, tag = "2")]
    pub delta_angle_x: f32,
    /// Delta angle Y.
    #[prost(float, tag = "3")]
    pub delta_angle_y: f32,
    /// Delta angle X.
    #[prost(float, tag = "4")]
    pub delta_angle_z: f32,
    /// Delta velocity X.
    #[prost(float, tag = "5")]
    pub delta_velocity_x: f32,
    /// Delta velocity Y.
    #[prost(float, tag = "6")]
    pub delta_velocity_y: f32,
    /// Delta velocity Z.
    #[prost(float, tag = "7")]
    pub delta_velocity_z: f32,
    /// Joint ROLL.
    #[prost(float, tag = "8")]
    pub joint_roll: f32,
    /// Joint EL.
    #[prost(float, tag = "9")]
    pub joint_el: f32,
    /// Joint AZ.
    #[prost(float, tag = "10")]
    pub joint_az: f32,
    /// System ID.
    #[prost(uint32, tag = "11")]
    pub target_system: u32,
    /// Component ID.
    #[prost(uint32, tag = "12")]
    pub target_component: u32,
}
/// Control message for rate gimbal.
///
/// MavLink id: 201
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GimbalControl {
    /// Demanded angular rate X.
    #[prost(float, tag = "1")]
    pub demanded_rate_x: f32,
    /// Demanded angular rate Y.
    #[prost(float, tag = "2")]
    pub demanded_rate_y: f32,
    /// Demanded angular rate Z.
    #[prost(float, tag = "3")]
    pub demanded_rate_z: f32,
    /// System ID.
    #[prost(uint32, tag = "4")]
    pub target_system: u32,
    /// Component ID.
    #[prost(uint32, tag = "5")]
    pub target_component: u32,
}
/// 100 Hz gimbal torque command telemetry.
///
/// MavLink id: 214
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GimbalTorqueCmdReport {
    /// Roll Torque Command.
    #[prost(int32, tag = "1")]
    pub rl_torque_cmd: i32,
    /// Elevation Torque Command.
    #[prost(int32, tag = "2")]
    pub el_torque_cmd: i32,
    /// Azimuth Torque Command.
    #[prost(int32, tag = "3")]
    pub az_torque_cmd: i32,
    /// System ID.
    #[prost(uint32, tag = "4")]
    pub target_system: u32,
    /// Component ID.
    #[prost(uint32, tag = "5")]
    pub target_component: u32,
}
/// Heartbeat from a HeroBus attached GoPro.
///
/// MavLink id: 215
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GoproHeartbeat {
    /// Status.
    #[prost(enumeration = "GoproHeartbeatStatus", tag = "1")]
    pub status: i32,
    /// Current capture mode.
    #[prost(enumeration = "GoproCaptureMode", tag = "2")]
    pub capture_mode: i32,
    /// Additional status bits.
    /// bitfield defined by enum GOPRO_HEARTBEAT_FLAGS
    #[prost(uint32, tag = "3")]
    pub flags: u32,
}
/// Request a GOPRO_COMMAND response from the GoPro.
///
/// MavLink id: 216
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GoproGetRequest {
    /// System ID.
    #[prost(uint32, tag = "1")]
    pub target_system: u32,
    /// Component ID.
    #[prost(uint32, tag = "2")]
    pub target_component: u32,
    /// Command ID.
    #[prost(enumeration = "GoproCommand", tag = "3")]
    pub cmd_id: i32,
}
/// Response from a GOPRO_COMMAND get request.
///
/// MavLink id: 217
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GoproGetResponse {
    /// Command ID.
    #[prost(enumeration = "GoproCommand", tag = "1")]
    pub cmd_id: i32,
    /// Status.
    #[prost(enumeration = "GoproRequestStatus", tag = "2")]
    pub status: i32,
    /// Value.
    #[prost(uint32, repeated, packed = "false", tag = "3")]
    pub value: ::prost::alloc::vec::Vec<u32>,
}
/// Request to set a GOPRO_COMMAND with a desired.
///
/// MavLink id: 218
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GoproSetRequest {
    /// System ID.
    #[prost(uint32, tag = "1")]
    pub target_system: u32,
    /// Component ID.
    #[prost(uint32, tag = "2")]
    pub target_component: u32,
    /// Command ID.
    #[prost(enumeration = "GoproCommand", tag = "3")]
    pub cmd_id: i32,
    /// Value.
    #[prost(uint32, repeated, packed = "false", tag = "4")]
    pub value: ::prost::alloc::vec::Vec<u32>,
}
/// Response from a GOPRO_COMMAND set request.
///
/// MavLink id: 219
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GoproSetResponse {
    /// Command ID.
    #[prost(enumeration = "GoproCommand", tag = "1")]
    pub cmd_id: i32,
    /// Status.
    #[prost(enumeration = "GoproRequestStatus", tag = "2")]
    pub status: i32,
}
/// EFI status output
///
/// MavLink id: 225
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct EfiStatus {
    /// ECU index
    #[prost(float, tag = "1")]
    pub ecu_index: f32,
    /// RPM
    #[prost(float, tag = "2")]
    pub rpm: f32,
    /// Fuel consumed
    #[prost(float, tag = "3")]
    pub fuel_consumed: f32,
    /// Fuel flow rate
    #[prost(float, tag = "4")]
    pub fuel_flow: f32,
    /// Engine load
    #[prost(float, tag = "5")]
    pub engine_load: f32,
    /// Throttle position
    #[prost(float, tag = "6")]
    pub throttle_position: f32,
    /// Spark dwell time
    #[prost(float, tag = "7")]
    pub spark_dwell_time: f32,
    /// Barometric pressure
    #[prost(float, tag = "8")]
    pub barometric_pressure: f32,
    /// Intake manifold pressure(
    #[prost(float, tag = "9")]
    pub intake_manifold_pressure: f32,
    /// Intake manifold temperature
    #[prost(float, tag = "10")]
    pub intake_manifold_temperature: f32,
    /// Cylinder head temperature
    #[prost(float, tag = "11")]
    pub cylinder_head_temperature: f32,
    /// Ignition timing (Crank angle degrees)
    #[prost(float, tag = "12")]
    pub ignition_timing: f32,
    /// Injection time
    #[prost(float, tag = "13")]
    pub injection_time: f32,
    /// Exhaust gas temperature
    #[prost(float, tag = "14")]
    pub exhaust_gas_temperature: f32,
    /// Output throttle
    #[prost(float, tag = "15")]
    pub throttle_out: f32,
    /// Pressure/temperature compensation
    #[prost(float, tag = "16")]
    pub pt_compensation: f32,
    /// EFI health status
    #[prost(uint32, tag = "17")]
    pub health: u32,
}
/// RPM sensor output.
///
/// MavLink id: 226
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Rpm {
    /// RPM Sensor1.
    #[prost(float, tag = "1")]
    pub rpm1: f32,
    /// RPM Sensor2.
    #[prost(float, tag = "2")]
    pub rpm2: f32,
}
/// Read registers for a device.
///
/// MavLink id: 11000
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct DeviceOpRead {
    /// Request ID - copied to reply.
    #[prost(uint32, tag = "1")]
    pub request_id: u32,
    /// System ID.
    #[prost(uint32, tag = "2")]
    pub target_system: u32,
    /// Component ID.
    #[prost(uint32, tag = "3")]
    pub target_component: u32,
    /// The bus type.
    #[prost(enumeration = "DeviceOpBustype", tag = "4")]
    pub bustype: i32,
    /// Bus number.
    #[prost(uint32, tag = "5")]
    pub bus: u32,
    /// Bus address.
    #[prost(uint32, tag = "6")]
    pub address: u32,
    /// Name of device on bus (for SPI).
    #[prost(string, tag = "7")]
    pub busname: ::prost::alloc::string::String,
    /// First register to read.
    #[prost(uint32, tag = "8")]
    pub regstart: u32,
    /// Count of registers to read.
    #[prost(uint32, tag = "9")]
    pub count: u32,
}
/// Read registers reply.
///
/// MavLink id: 11001
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct DeviceOpReadReply {
    /// Request ID - copied from request.
    #[prost(uint32, tag = "1")]
    pub request_id: u32,
    /// 0 for success, anything else is failure code.
    #[prost(uint32, tag = "2")]
    pub result: u32,
    /// Starting register.
    #[prost(uint32, tag = "3")]
    pub regstart: u32,
    /// Count of bytes read.
    #[prost(uint32, tag = "4")]
    pub count: u32,
    /// Reply data.
    #[prost(uint32, repeated, packed = "false", tag = "5")]
    pub data: ::prost::alloc::vec::Vec<u32>,
}
/// Write registers for a device.
///
/// MavLink id: 11002
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct DeviceOpWrite {
    /// Request ID - copied to reply.
    #[prost(uint32, tag = "1")]
    pub request_id: u32,
    /// System ID.
    #[prost(uint32, tag = "2")]
    pub target_system: u32,
    /// Component ID.
    #[prost(uint32, tag = "3")]
    pub target_component: u32,
    /// The bus type.
    #[prost(enumeration = "DeviceOpBustype", tag = "4")]
    pub bustype: i32,
    /// Bus number.
    #[prost(uint32, tag = "5")]
    pub bus: u32,
    /// Bus address.
    #[prost(uint32, tag = "6")]
    pub address: u32,
    /// Name of device on bus (for SPI).
    #[prost(string, tag = "7")]
    pub busname: ::prost::alloc::string::String,
    /// First register to write.
    #[prost(uint32, tag = "8")]
    pub regstart: u32,
    /// Count of registers to write.
    #[prost(uint32, tag = "9")]
    pub count: u32,
    /// Write data.
    #[prost(uint32, repeated, packed = "false", tag = "10")]
    pub data: ::prost::alloc::vec::Vec<u32>,
}
/// Write registers reply.
///
/// MavLink id: 11003
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct DeviceOpWriteReply {
    /// Request ID - copied from request.
    #[prost(uint32, tag = "1")]
    pub request_id: u32,
    /// 0 for success, anything else is failure code.
    #[prost(uint32, tag = "2")]
    pub result: u32,
}
/// Adaptive Controller tuning information.
///
/// MavLink id: 11010
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct AdapTuning {
    /// Desired rate.
    #[prost(float, tag = "1")]
    pub desired: f32,
    /// Achieved rate.
    #[prost(float, tag = "2")]
    pub achieved: f32,
    /// Error between model and vehicle.
    #[prost(float, tag = "3")]
    pub error: f32,
    /// Theta estimated state predictor.
    #[prost(float, tag = "4")]
    pub theta: f32,
    /// Omega estimated state predictor.
    #[prost(float, tag = "5")]
    pub omega: f32,
    /// Sigma estimated state predictor.
    #[prost(float, tag = "6")]
    pub sigma: f32,
    /// Theta derivative.
    #[prost(float, tag = "7")]
    pub theta_dot: f32,
    /// Omega derivative.
    #[prost(float, tag = "8")]
    pub omega_dot: f32,
    /// Sigma derivative.
    #[prost(float, tag = "9")]
    pub sigma_dot: f32,
    /// Projection operator value.
    #[prost(float, tag = "10")]
    pub f: f32,
    /// Projection operator derivative.
    #[prost(float, tag = "11")]
    pub f_dot: f32,
    /// u adaptive controlled output command.
    #[prost(float, tag = "12")]
    pub u: f32,
    /// Axis.
    #[prost(enumeration = "PidTuningAxis", tag = "13")]
    pub axis: i32,
}
/// Camera vision based attitude and position deltas.
///
/// MavLink id: 11011
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct VisionPositionDelta {
    /// Timestamp (synced to UNIX time or since system boot).
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// Time since the last reported camera frame.
    #[prost(uint64, tag = "2")]
    pub time_delta_usec: u64,
    /// Defines a rotation vector in body frame that rotates the vehicle from the previous to the current orientation.
    #[prost(float, repeated, packed = "false", tag = "3")]
    pub angle_delta: ::prost::alloc::vec::Vec<f32>,
    /// Change in position from previous to current frame rotated into body frame (0=forward, 1=right, 2=down).
    #[prost(float, repeated, packed = "false", tag = "4")]
    pub position_delta: ::prost::alloc::vec::Vec<f32>,
    /// Normalised confidence value from 0 to 100.
    #[prost(float, tag = "5")]
    pub confidence: f32,
}
/// Angle of Attack and Side Slip Angle.
///
/// MavLink id: 11020
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct AoaSsa {
    /// Timestamp (since boot or Unix epoch).
    #[prost(uint64, tag = "1")]
    pub time_usec: u64,
    /// Angle of Attack.
    #[prost(float, tag = "2")]
    pub aoa: f32,
    /// Side Slip Angle.
    #[prost(float, tag = "3")]
    pub ssa: f32,
}
/// ESC Telemetry Data for ESCs 1 to 4, matching data sent by BLHeli ESCs.
///
/// MavLink id: 11030
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct EscTelemetry1To4 {
    /// Voltage.
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub voltage: ::prost::alloc::vec::Vec<u32>,
    /// Current.
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub current: ::prost::alloc::vec::Vec<u32>,
    /// Total current.
    #[prost(uint32, repeated, packed = "false", tag = "3")]
    pub totalcurrent: ::prost::alloc::vec::Vec<u32>,
    /// RPM (eRPM).
    #[prost(uint32, repeated, packed = "false", tag = "4")]
    pub rpm: ::prost::alloc::vec::Vec<u32>,
    /// count of telemetry packets received (wraps at 65535).
    #[prost(uint32, repeated, packed = "false", tag = "5")]
    pub count: ::prost::alloc::vec::Vec<u32>,
    /// Temperature.
    #[prost(uint32, repeated, packed = "false", tag = "6")]
    pub temperature: ::prost::alloc::vec::Vec<u32>,
}
/// ESC Telemetry Data for ESCs 5 to 8, matching data sent by BLHeli ESCs.
///
/// MavLink id: 11031
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct EscTelemetry5To8 {
    /// Voltage.
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub voltage: ::prost::alloc::vec::Vec<u32>,
    /// Current.
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub current: ::prost::alloc::vec::Vec<u32>,
    /// Total current.
    #[prost(uint32, repeated, packed = "false", tag = "3")]
    pub totalcurrent: ::prost::alloc::vec::Vec<u32>,
    /// RPM (eRPM).
    #[prost(uint32, repeated, packed = "false", tag = "4")]
    pub rpm: ::prost::alloc::vec::Vec<u32>,
    /// count of telemetry packets received (wraps at 65535).
    #[prost(uint32, repeated, packed = "false", tag = "5")]
    pub count: ::prost::alloc::vec::Vec<u32>,
    /// Temperature.
    #[prost(uint32, repeated, packed = "false", tag = "6")]
    pub temperature: ::prost::alloc::vec::Vec<u32>,
}
/// ESC Telemetry Data for ESCs 9 to 12, matching data sent by BLHeli ESCs.
///
/// MavLink id: 11032
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct EscTelemetry9To12 {
    /// Voltage.
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub voltage: ::prost::alloc::vec::Vec<u32>,
    /// Current.
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub current: ::prost::alloc::vec::Vec<u32>,
    /// Total current.
    #[prost(uint32, repeated, packed = "false", tag = "3")]
    pub totalcurrent: ::prost::alloc::vec::Vec<u32>,
    /// RPM (eRPM).
    #[prost(uint32, repeated, packed = "false", tag = "4")]
    pub rpm: ::prost::alloc::vec::Vec<u32>,
    /// count of telemetry packets received (wraps at 65535).
    #[prost(uint32, repeated, packed = "false", tag = "5")]
    pub count: ::prost::alloc::vec::Vec<u32>,
    /// Temperature.
    #[prost(uint32, repeated, packed = "false", tag = "6")]
    pub temperature: ::prost::alloc::vec::Vec<u32>,
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
pub enum AccelcalVehiclePos {
    /// Not used in MavLink, make protobuf happy.
    Undefined = 0,
    Level = 1,
    Left = 2,
    Right = 3,
    Nosedown = 4,
    Noseup = 5,
    Back = 6,
    Success = 16777215,
    Failed = 16777216,
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
    /// Mission command to wait for an altitude or downwards vertical speed. This is meant for high altitude balloon launches, allowing the aircraft to be idle until either an altitude is reached or a negative vertical speed is reached (indicating early balloon burst). The wiggle time is how often to wiggle the control surfaces to prevent them seizing up.
    NavAltitudeWait = 83,
    /// ***** START Params
    /// Altitude (m).
    /// Descent speed (m/s).
    /// Wiggle Time (s).
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
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
    /// Mission command to operate EPM gripper.
    DoGripper = 211,
    /// ***** START Params
    /// Gripper number (a number from 1 to max number of grippers on the vehicle).
    /// Gripper action (0=release, 1=grab. See GRIPPER_ACTIONS enum).
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// ***** END Params
    /// Enable/disable autotune.
    DoAutotuneEnable = 212,
    /// ***** START Params
    /// Enable (1: enable, 0:disable).
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
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
    /// Set the distance to be repeated on mission resume
    DoSetResumeRepeatDist = 215,
    /// ***** START Params
    /// Distance.
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
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
    User5 = 31014,
    /// ***** START Params
    /// User defined
    /// User defined
    /// User defined
    /// User defined
    /// User defined
    /// User defined
    /// User defined
    /// ***** END Params
    /// A system wide power-off event has been initiated.
    PowerOffInitiated = 42000,
    /// ***** START Params
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// ***** END Params
    /// FLY button has been clicked.
    SoloBtnFlyClick = 42001,
    /// ***** START Params
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// ***** END Params
    /// FLY button has been held for 1.5 seconds.
    SoloBtnFlyHold = 42002,
    /// ***** START Params
    /// Takeoff altitude.
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// ***** END Params
    /// PAUSE button has been clicked.
    SoloBtnPauseClick = 42003,
    /// ***** START Params
    /// 1 if Solo is in a shot mode, 0 otherwise.
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// ***** END Params
    /// Magnetometer calibration based on fixed position
    ///         in earth field given by inclination, declination and intensity.
    FixedMagCal = 42004,
    /// ***** START Params
    /// MagDeclinationDegrees.
    /// MagInclinationDegrees.
    /// MagIntensityMilliGauss.
    /// YawDegrees.
    /// Empty.
    /// Empty.
    /// Empty.
    /// ***** END Params
    /// Magnetometer calibration based on fixed expected field values in milliGauss.
    FixedMagCalField = 42005,
    /// ***** START Params
    /// FieldX.
    /// FieldY.
    /// FieldZ.
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// ***** END Params
    /// Magnetometer calibration based on provided known yaw. This allows for fast calibration using WMM field tables in the vehicle, given only the known yaw of the vehicle. If Latitude and longitude are both zero then use the current vehicle location.
    FixedMagCalYaw = 42006,
    /// ***** START Params
    /// Yaw of vehicle in earth frame.
    /// CompassMask, 0 for all.
    /// Latitude.
    /// Longitude.
    /// Empty.
    /// Empty.
    /// Empty.
    /// ***** END Params
    /// Initiate a magnetometer calibration.
    DoStartMagCal = 42424,
    /// ***** START Params
    /// uint8_t bitmask of magnetometers (0 means all).
    /// Automatically retry on failure (0=no retry, 1=retry).
    /// Save without user input (0=require input, 1=autosave).
    /// Delay (seconds).
    /// Autoreboot (0=user reboot, 1=autoreboot).
    /// Empty.
    /// Empty.
    /// ***** END Params
    /// Initiate a magnetometer calibration.
    DoAcceptMagCal = 42425,
    /// ***** START Params
    /// uint8_t bitmask of magnetometers (0 means all).
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// ***** END Params
    /// Cancel a running magnetometer calibration.
    DoCancelMagCal = 42426,
    /// ***** START Params
    /// uint8_t bitmask of magnetometers (0 means all).
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// ***** END Params
    /// Command autopilot to get into factory test/diagnostic mode.
    SetFactoryTestMode = 42427,
    /// ***** START Params
    /// 0 means get out of test mode, 1 means get into test mode.
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// ***** END Params
    /// Reply with the version banner.
    DoSendBanner = 42428,
    /// ***** START Params
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// ***** END Params
    /// Used when doing accelerometer calibration. When sent to the GCS tells it what position to put the vehicle in. When sent to the vehicle says what position the vehicle is in.
    AccelcalVehiclePos = 42429,
    /// ***** START Params
    /// Position, one of the ACCELCAL_VEHICLE_POS enum values.
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// ***** END Params
    /// Causes the gimbal to reset and boot as if it was just powered on.
    GimbalReset = 42501,
    /// ***** START Params
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// ***** END Params
    /// Reports progress and success or failure of gimbal axis calibration procedure.
    GimbalAxisCalibrationStatus = 42502,
    /// ***** START Params
    /// Gimbal axis we're reporting calibration progress for.
    /// Current calibration progress for this axis, 0x64=100%.
    /// Status of the calibration.
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// ***** END Params
    /// Starts commutation calibration on the gimbal.
    GimbalRequestAxisCalibration = 42503,
    /// ***** START Params
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// ***** END Params
    /// Erases gimbal application and parameters.
    GimbalFullReset = 42505,
    /// ***** START Params
    /// Magic number.
    /// Magic number.
    /// Magic number.
    /// Magic number.
    /// Magic number.
    /// Magic number.
    /// Magic number.
    /// ***** END Params
    /// Command to operate winch.
    DoWinch = 42600,
    /// ***** START Params
    /// Winch number (0 for the default winch, otherwise a number from 1 to max number of winches on the vehicle).
    /// Action (0=relax, 1=relative length control, 2=rate control. See WINCH_ACTIONS enum.).
    /// Release length (cable distance to unwind in meters, negative numbers to wind in cable).
    /// Release rate (meters/second).
    /// Empty.
    /// Empty.
    /// Empty.
    /// ***** END Params
    /// Update the bootloader
    FlashBootloader = 42650,
    /// ***** START Params
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// Magic number - set to 290876 to actually flash
    /// Empty
    /// Empty
    /// ***** END Params
    /// Reset battery capacity for batteries that accumulate consumed battery via integration.
    BatteryReset = 42651,
    /// ***** START Params
    /// Bitmask of batteries to reset. Least significant bit is for the first battery.
    /// Battery percentage remaining to set.
    /// ***** END Params
    /// Issue a trap signal to the autopilot process, presumably to enter the debugger.
    DebugTrap = 42700,
    /// ***** START Params
    /// Magic number - set to 32451 to actually trap.
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// Empty.
    /// ***** END Params
    /// Control onboard scripting.
    ///
    /// ***** START Params
    /// Scripting command to execute
    /// ***** END Params
    Scripting = 42701,
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
pub enum ScriptingCmd {
    /// Start a REPL session.
    ReplStart = 0,
    /// End a REPL session.
    ReplStop = 1,
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
pub enum LimitsState {
    /// Pre-initialization.
    LimitsInit = 0,
    /// Disabled.
    LimitsDisabled = 1,
    /// Checking limits.
    LimitsEnabled = 2,
    /// A limit has been breached.
    LimitsTriggered = 3,
    /// Taking action e.g. Return/RTL.
    LimitsRecovering = 4,
    /// We're no longer in breach of a limit.
    LimitsRecovered = 5,
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
pub enum LimitModule {
    /// This enum is used to define bitmasks (work around protobuf limitations).
    /// Not used in MavLink, make protobuf happy.
    Undefined = 0,
    /// Pre-initialization.
    /// bit 1
    LimitGpslock = 1,
    /// Disabled.
    /// bit 2
    LimitGeofence = 2,
    /// Checking limits.
    /// bit 3
    LimitAltitude = 4,
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
pub enum RallyFlags {
    /// Flags in RALLY_POINT message.
    /// This enum is used to define bitmasks (work around protobuf limitations).
    /// Not used in MavLink, make protobuf happy.
    Undefined = 0,
    /// Flag set when requiring favorable winds for landing.
    /// bit 1
    FavorableWind = 1,
    /// Flag set when plane is to immediately descend to break altitude and land without GCS intervention. Flag not set when plane is to loiter at Rally point until commanded to land.
    /// bit 2
    LandImmediately = 2,
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
pub enum GripperActions {
    /// Gripper actions.
    /// Gripper release cargo.
    GripperActionRelease = 0,
    /// Gripper grab onto cargo.
    GripperActionGrab = 1,
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
pub enum WinchActions {
    /// Winch actions.
    /// Relax winch.
    WinchRelaxed = 0,
    /// Winch unwinds or winds specified length of cable optionally using specified rate.
    WinchRelativeLengthControl = 1,
    /// Winch unwinds or winds cable at specified rate in meters/seconds.
    WinchRateControl = 2,
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
pub enum CameraStatusTypes {
    /// Camera heartbeat, announce camera component ID at 1Hz.
    CameraStatusTypeHeartbeat = 0,
    /// Camera image triggered.
    CameraStatusTypeTrigger = 1,
    /// Camera connection lost.
    CameraStatusTypeDisconnect = 2,
    /// Camera unknown error.
    CameraStatusTypeError = 3,
    /// Camera battery low. Parameter p1 shows reported voltage.
    CameraStatusTypeLowbatt = 4,
    /// Camera storage low. Parameter p1 shows reported shots remaining.
    CameraStatusTypeLowstore = 5,
    /// Camera storage low. Parameter p1 shows reported video minutes remaining.
    CameraStatusTypeLowstorev = 6,
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
pub enum CameraFeedbackFlags {
    /// Shooting photos, not video.
    CameraFeedbackPhoto = 0,
    /// Shooting video, not stills.
    CameraFeedbackVideo = 1,
    /// Unable to achieve requested exposure (e.g. shutter speed too low).
    CameraFeedbackBadexposure = 2,
    /// Closed loop feedback from camera, we know for sure it has successfully taken a picture.
    CameraFeedbackClosedloop = 3,
    /// Open loop camera, an image trigger has been requested but we can't know for sure it has successfully taken a picture.
    CameraFeedbackOpenloop = 4,
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
pub enum MavModeGimbal {
    /// Gimbal is powered on but has not started initializing yet.
    Uninitialized = 0,
    /// Gimbal is currently running calibration on the pitch axis.
    CalibratingPitch = 1,
    /// Gimbal is currently running calibration on the roll axis.
    CalibratingRoll = 2,
    /// Gimbal is currently running calibration on the yaw axis.
    CalibratingYaw = 3,
    /// Gimbal has finished calibrating and initializing, but is relaxed pending reception of first rate command from copter.
    Initialized = 4,
    /// Gimbal is actively stabilizing.
    Active = 5,
    /// Gimbal is relaxed because it missed more than 10 expected rate command messages in a row. Gimbal will move back to active mode when it receives a new rate command.
    RateCmdTimeout = 6,
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
pub enum GimbalAxis {
    /// Gimbal yaw axis.
    Yaw = 0,
    /// Gimbal pitch axis.
    Pitch = 1,
    /// Gimbal roll axis.
    Roll = 2,
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
pub enum GimbalAxisCalibrationStatus {
    /// Axis calibration is in progress.
    InProgress = 0,
    /// Axis calibration succeeded.
    Succeeded = 1,
    /// Axis calibration failed.
    Failed = 2,
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
pub enum GimbalAxisCalibrationRequired {
    /// Whether or not this axis requires calibration is unknown at this time.
    Unknown = 0,
    /// This axis requires calibration.
    True = 1,
    /// This axis does not require calibration.
    False = 2,
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
pub enum GoproHeartbeatStatus {
    /// No GoPro connected.
    Disconnected = 0,
    /// The detected GoPro is not HeroBus compatible.
    Incompatible = 1,
    /// A HeroBus compatible GoPro is connected.
    Connected = 2,
    /// An unrecoverable error was encountered with the connected GoPro, it may require a power cycle.
    Error = 3,
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
pub enum GoproHeartbeatFlags {
    /// This enum is used to define bitmasks (work around protobuf limitations).
    /// Not used in MavLink, make protobuf happy.
    Undefined = 0,
    /// GoPro is currently recording.
    /// bit 1
    GoproFlagRecording = 1,
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
pub enum GoproRequestStatus {
    /// The write message with ID indicated succeeded.
    GoproRequestSuccess = 0,
    /// The write message with ID indicated failed.
    GoproRequestFailed = 1,
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
pub enum GoproCommand {
    /// (Get/Set).
    Power = 0,
    /// (Get/Set).
    CaptureMode = 1,
    /// (___/Set).
    Shutter = 2,
    /// (Get/___).
    Battery = 3,
    /// (Get/___).
    Model = 4,
    /// (Get/Set).
    VideoSettings = 5,
    /// (Get/Set).
    LowLight = 6,
    /// (Get/Set).
    PhotoResolution = 7,
    /// (Get/Set).
    PhotoBurstRate = 8,
    /// (Get/Set).
    Protune = 9,
    /// (Get/Set) Hero 3+ Only.
    ProtuneWhiteBalance = 10,
    /// (Get/Set) Hero 3+ Only.
    ProtuneColour = 11,
    /// (Get/Set) Hero 3+ Only.
    ProtuneGain = 12,
    /// (Get/Set) Hero 3+ Only.
    ProtuneSharpness = 13,
    /// (Get/Set) Hero 3+ Only.
    ProtuneExposure = 14,
    /// (Get/Set).
    Time = 15,
    /// (Get/Set).
    Charging = 16,
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
pub enum GoproCaptureMode {
    /// Video mode.
    Video = 0,
    /// Photo mode.
    Photo = 1,
    /// Burst mode, Hero 3+ only.
    Burst = 2,
    /// Time lapse mode, Hero 3+ only.
    TimeLapse = 3,
    /// Multi shot mode, Hero 4 only.
    MultiShot = 4,
    /// Playback mode, Hero 4 only, silver only except when LCD or HDMI is connected to black.
    Playback = 5,
    /// Playback mode, Hero 4 only.
    Setup = 6,
    /// Mode not yet known.
    Unknown = 255,
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
pub enum GoproResolution {
    /// 848 x 480 (480p).
    GoproResolution480p = 0,
    /// 1280 x 720 (720p).
    GoproResolution720p = 1,
    /// 1280 x 960 (960p).
    GoproResolution960p = 2,
    /// 1920 x 1080 (1080p).
    GoproResolution1080p = 3,
    /// 1920 x 1440 (1440p).
    GoproResolution1440p = 4,
    /// 2704 x 1440 (2.7k-17:9).
    GoproResolution27k179 = 5,
    /// 2704 x 1524 (2.7k-16:9).
    GoproResolution27k169 = 6,
    /// 2704 x 2028 (2.7k-4:3).
    GoproResolution27k43 = 7,
    /// 3840 x 2160 (4k-16:9).
    GoproResolution4k169 = 8,
    /// 4096 x 2160 (4k-17:9).
    GoproResolution4k179 = 9,
    /// 1280 x 720 (720p-SuperView).
    GoproResolution720pSuperview = 10,
    /// 1920 x 1080 (1080p-SuperView).
    GoproResolution1080pSuperview = 11,
    /// 2704 x 1520 (2.7k-SuperView).
    GoproResolution27kSuperview = 12,
    /// 3840 x 2160 (4k-SuperView).
    GoproResolution4kSuperview = 13,
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
pub enum GoproFrameRate {
    /// 12 FPS.
    GoproFrameRate12 = 0,
    /// 15 FPS.
    GoproFrameRate15 = 1,
    /// 24 FPS.
    GoproFrameRate24 = 2,
    /// 25 FPS.
    GoproFrameRate25 = 3,
    /// 30 FPS.
    GoproFrameRate30 = 4,
    /// 48 FPS.
    GoproFrameRate48 = 5,
    /// 50 FPS.
    GoproFrameRate50 = 6,
    /// 60 FPS.
    GoproFrameRate60 = 7,
    /// 80 FPS.
    GoproFrameRate80 = 8,
    /// 90 FPS.
    GoproFrameRate90 = 9,
    /// 100 FPS.
    GoproFrameRate100 = 10,
    /// 120 FPS.
    GoproFrameRate120 = 11,
    /// 240 FPS.
    GoproFrameRate240 = 12,
    /// 12.5 FPS.
    GoproFrameRate125 = 13,
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
pub enum GoproFieldOfView {
    /// 0x00: Wide.
    Wide = 0,
    /// 0x01: Medium.
    Medium = 1,
    /// 0x02: Narrow.
    Narrow = 2,
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
pub enum GoproVideoSettingsFlags {
    /// Not used in MavLink, make protobuf happy.
    Undefined = 0,
    /// 0=NTSC, 1=PAL.
    GoproVideoSettingsTvMode = 1,
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
pub enum GoproPhotoResolution {
    /// 5MP Medium.
    GoproPhotoResolution5mpMedium = 0,
    /// 7MP Medium.
    GoproPhotoResolution7mpMedium = 1,
    /// 7MP Wide.
    GoproPhotoResolution7mpWide = 2,
    /// 10MP Wide.
    GoproPhotoResolution10mpWide = 3,
    /// 12MP Wide.
    GoproPhotoResolution12mpWide = 4,
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
pub enum GoproProtuneWhiteBalance {
    /// Auto.
    Auto = 0,
    /// 3000K.
    GoproProtuneWhiteBalance3000k = 1,
    /// 5500K.
    GoproProtuneWhiteBalance5500k = 2,
    /// 6500K.
    GoproProtuneWhiteBalance6500k = 3,
    /// Camera Raw.
    Raw = 4,
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
pub enum GoproProtuneColour {
    /// Auto.
    Standard = 0,
    /// Neutral.
    Neutral = 1,
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
pub enum GoproProtuneGain {
    /// ISO 400.
    GoproProtuneGain400 = 0,
    /// ISO 800 (Only Hero 4).
    GoproProtuneGain800 = 1,
    /// ISO 1600.
    GoproProtuneGain1600 = 2,
    /// ISO 3200 (Only Hero 4).
    GoproProtuneGain3200 = 3,
    /// ISO 6400.
    GoproProtuneGain6400 = 4,
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
pub enum GoproProtuneSharpness {
    /// Low Sharpness.
    Low = 0,
    /// Medium Sharpness.
    Medium = 1,
    /// High Sharpness.
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
pub enum GoproProtuneExposure {
    /// -5.0 EV (Hero 3+ Only).
    Neg50 = 0,
    /// -4.5 EV (Hero 3+ Only).
    Neg45 = 1,
    /// -4.0 EV (Hero 3+ Only).
    Neg40 = 2,
    /// -3.5 EV (Hero 3+ Only).
    Neg35 = 3,
    /// -3.0 EV (Hero 3+ Only).
    Neg30 = 4,
    /// -2.5 EV (Hero 3+ Only).
    Neg25 = 5,
    /// -2.0 EV.
    Neg20 = 6,
    /// -1.5 EV.
    Neg15 = 7,
    /// -1.0 EV.
    Neg10 = 8,
    /// -0.5 EV.
    Neg05 = 9,
    /// 0.0 EV.
    Zero = 10,
    /// +0.5 EV.
    Pos05 = 11,
    /// +1.0 EV.
    Pos10 = 12,
    /// +1.5 EV.
    Pos15 = 13,
    /// +2.0 EV.
    Pos20 = 14,
    /// +2.5 EV (Hero 3+ Only).
    Pos25 = 15,
    /// +3.0 EV (Hero 3+ Only).
    Pos30 = 16,
    /// +3.5 EV (Hero 3+ Only).
    Pos35 = 17,
    /// +4.0 EV (Hero 3+ Only).
    Pos40 = 18,
    /// +4.5 EV (Hero 3+ Only).
    Pos45 = 19,
    /// +5.0 EV (Hero 3+ Only).
    Pos50 = 20,
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
pub enum GoproCharging {
    /// Charging disabled.
    Disabled = 0,
    /// Charging enabled.
    Enabled = 1,
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
pub enum GoproModel {
    /// Unknown gopro model.
    Unknown = 0,
    /// Hero 3+ Silver (HeroBus not supported by GoPro).
    Hero3PlusSilver = 1,
    /// Hero 3+ Black.
    Hero3PlusBlack = 2,
    /// Hero 4 Silver.
    Hero4Silver = 3,
    /// Hero 4 Black.
    Hero4Black = 4,
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
pub enum GoproBurstRate {
    /// 3 Shots / 1 Second.
    GoproBurstRate3In1Second = 0,
    /// 5 Shots / 1 Second.
    GoproBurstRate5In1Second = 1,
    /// 10 Shots / 1 Second.
    GoproBurstRate10In1Second = 2,
    /// 10 Shots / 2 Second.
    GoproBurstRate10In2Second = 3,
    /// 10 Shots / 3 Second (Hero 4 Only).
    GoproBurstRate10In3Second = 4,
    /// 30 Shots / 1 Second.
    GoproBurstRate30In1Second = 5,
    /// 30 Shots / 2 Second.
    GoproBurstRate30In2Second = 6,
    /// 30 Shots / 3 Second.
    GoproBurstRate30In3Second = 7,
    /// 30 Shots / 6 Second.
    GoproBurstRate30In6Second = 8,
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
pub enum LedControlPattern {
    /// LED patterns off (return control to regular vehicle control).
    Off = 0,
    /// LEDs show pattern during firmware update.
    Firmwareupdate = 1,
    /// Custom Pattern using custom bytes fields.
    Custom = 255,
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
pub enum EkfStatusFlags {
    /// Flags in EKF_STATUS message.
    /// This enum is used to define bitmasks (work around protobuf limitations).
    /// Not used in MavLink, make protobuf happy.
    Undefined = 0,
    /// Set if EKF's attitude estimate is good.
    /// bit 1
    EkfAttitude = 1,
    /// Set if EKF's horizontal velocity estimate is good.
    /// bit 2
    EkfVelocityHoriz = 2,
    /// Set if EKF's vertical velocity estimate is good.
    /// bit 3
    EkfVelocityVert = 4,
    /// Set if EKF's horizontal position (relative) estimate is good.
    /// bit 4
    EkfPosHorizRel = 8,
    /// Set if EKF's horizontal position (absolute) estimate is good.
    /// bit 5
    EkfPosHorizAbs = 16,
    /// Set if EKF's vertical position (absolute) estimate is good.
    /// bit 6
    EkfPosVertAbs = 32,
    /// Set if EKF's vertical position (above ground) estimate is good.
    /// bit 7
    EkfPosVertAgl = 64,
    /// EKF is in constant position mode and does not know it's absolute or relative position.
    /// bit 8
    EkfConstPosMode = 128,
    /// Set if EKF's predicted horizontal position (relative) estimate is good.
    /// bit 9
    EkfPredPosHorizRel = 256,
    /// Set if EKF's predicted horizontal position (absolute) estimate is good.
    /// bit 10
    EkfPredPosHorizAbs = 512,
    /// Set if EKF has never been healthy.
    /// bit 11
    EkfUninitialized = 1024,
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
pub enum PidTuningAxis {
    /// Not used in MavLink, make protobuf happy.
    Undefined = 0,
    PidTuningRoll = 1,
    PidTuningPitch = 2,
    PidTuningYaw = 3,
    PidTuningAccz = 4,
    PidTuningSteer = 5,
    PidTuningLanding = 6,
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
pub enum MagCalStatus {
    MagCalNotStarted = 0,
    MagCalWaitingToStart = 1,
    MagCalRunningStepOne = 2,
    MagCalRunningStepTwo = 3,
    MagCalSuccess = 4,
    MagCalFailed = 5,
    MagCalBadOrientation = 6,
    MagCalBadRadius = 7,
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
pub enum MavRemoteLogDataBlockCommands {
    /// Special ACK block numbers control activation of dataflash log streaming.
    /// Not used in MavLink, make protobuf happy.
    Undefined = 0,
    /// UAV to stop sending DataFlash blocks.
    MavRemoteLogDataBlockStop = 2147483645,
    /// UAV to start sending DataFlash blocks.
    MavRemoteLogDataBlockStart = 2147483646,
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
pub enum MavRemoteLogDataBlockStatuses {
    /// Possible remote log data block statuses.
    /// This block has NOT been received.
    MavRemoteLogDataBlockNack = 0,
    /// This block has been received.
    MavRemoteLogDataBlockAck = 1,
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
pub enum DeviceOpBustype {
    /// Bus types for device operations.
    /// I2C Device operation.
    I2c = 0,
    /// SPI Device operation.
    Spi = 1,
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
pub enum DeepstallStage {
    /// Deepstall flight stage.
    /// Flying to the landing point.
    FlyToLanding = 0,
    /// Building an estimate of the wind.
    EstimateWind = 1,
    /// Waiting to breakout of the loiter to fly the approach.
    WaitForBreakout = 2,
    /// Flying to the first arc point to turn around to the landing point.
    FlyToArc = 3,
    /// Turning around back to the deepstall landing point.
    Arc = 4,
    /// Approaching the landing point.
    Approach = 5,
    /// Stalling and steering towards the land point.
    Land = 6,
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
pub enum PlaneMode {
    /// A mapping of plane flight modes for custom_mode field of heartbeat.
    Manual = 0,
    Circle = 1,
    Stabilize = 2,
    Training = 3,
    Acro = 4,
    FlyByWireA = 5,
    FlyByWireB = 6,
    Cruise = 7,
    Autotune = 8,
    Auto = 10,
    Rtl = 11,
    Loiter = 12,
    Takeoff = 13,
    AvoidAdsb = 14,
    Guided = 15,
    Initializing = 16,
    Qstabilize = 17,
    Qhover = 18,
    Qloiter = 19,
    Qland = 20,
    Qrtl = 21,
    Qautotune = 22,
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
pub enum CopterMode {
    /// A mapping of copter flight modes for custom_mode field of heartbeat.
    Stabilize = 0,
    Acro = 1,
    AltHold = 2,
    Auto = 3,
    Guided = 4,
    Loiter = 5,
    Rtl = 6,
    Circle = 7,
    Land = 9,
    Drift = 11,
    Sport = 13,
    Flip = 14,
    Autotune = 15,
    Poshold = 16,
    Brake = 17,
    Throw = 18,
    AvoidAdsb = 19,
    GuidedNogps = 20,
    SmartRtl = 21,
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
pub enum SubMode {
    /// A mapping of sub flight modes for custom_mode field of heartbeat.
    Stabilize = 0,
    Acro = 1,
    AltHold = 2,
    Auto = 3,
    Guided = 4,
    Circle = 7,
    Surface = 9,
    Poshold = 16,
    Manual = 19,
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
pub enum RoverMode {
    /// A mapping of rover flight modes for custom_mode field of heartbeat.
    Manual = 0,
    Acro = 1,
    Steering = 3,
    Hold = 4,
    Loiter = 5,
    Auto = 10,
    Rtl = 11,
    SmartRtl = 12,
    Guided = 15,
    Initializing = 16,
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
pub enum TrackerMode {
    /// A mapping of antenna tracker flight modes for custom_mode field of heartbeat.
    Manual = 0,
    Stop = 1,
    Scan = 2,
    ServoTest = 3,
    Auto = 10,
    Initializing = 16,
}
