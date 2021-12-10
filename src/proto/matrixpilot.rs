/// Depreciated but used as a compiler flag.  Do not remove
///
/// MavLink id: 150
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct FlexifunctionSet {
    /// System ID
    #[prost(uint32, tag = "1")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "2")]
    pub target_component: u32,
}
/// Reqest reading of flexifunction data
///
/// MavLink id: 151
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct FlexifunctionReadReq {
    /// Type of flexifunction data requested
    #[prost(int32, tag = "1")]
    pub read_req_type: i32,
    /// index into data where needed
    #[prost(int32, tag = "2")]
    pub data_index: i32,
    /// System ID
    #[prost(uint32, tag = "3")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "4")]
    pub target_component: u32,
}
/// Flexifunction type and parameters for component at function index from buffer
///
/// MavLink id: 152
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct FlexifunctionBufferFunction {
    /// Function index
    #[prost(uint32, tag = "1")]
    pub func_index: u32,
    /// Total count of functions
    #[prost(uint32, tag = "2")]
    pub func_count: u32,
    /// Address in the flexifunction data, Set to 0xFFFF to use address in target memory
    #[prost(uint32, tag = "3")]
    pub data_address: u32,
    /// Size of the
    #[prost(uint32, tag = "4")]
    pub data_size: u32,
    /// System ID
    #[prost(uint32, tag = "5")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "6")]
    pub target_component: u32,
    /// Settings data
    #[prost(int32, repeated, packed = "false", tag = "7")]
    pub data: ::prost::alloc::vec::Vec<i32>,
}
/// Flexifunction type and parameters for component at function index from buffer
///
/// MavLink id: 153
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct FlexifunctionBufferFunctionAck {
    /// Function index
    #[prost(uint32, tag = "1")]
    pub func_index: u32,
    /// result of acknowledge, 0=fail, 1=good
    #[prost(uint32, tag = "2")]
    pub result: u32,
    /// System ID
    #[prost(uint32, tag = "3")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "4")]
    pub target_component: u32,
}
/// Acknowldge sucess or failure of a flexifunction command
///
/// MavLink id: 155
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct FlexifunctionDirectory {
    /// System ID
    #[prost(uint32, tag = "1")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "2")]
    pub target_component: u32,
    /// 0=inputs, 1=outputs
    #[prost(uint32, tag = "3")]
    pub directory_type: u32,
    /// index of first directory entry to write
    #[prost(uint32, tag = "4")]
    pub start_index: u32,
    /// count of directory entries to write
    #[prost(uint32, tag = "5")]
    pub count: u32,
    /// Settings data
    #[prost(int32, repeated, packed = "false", tag = "6")]
    pub directory_data: ::prost::alloc::vec::Vec<i32>,
}
/// Acknowldge sucess or failure of a flexifunction command
///
/// MavLink id: 156
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct FlexifunctionDirectoryAck {
    /// result of acknowledge, 0=fail, 1=good
    #[prost(uint32, tag = "1")]
    pub result: u32,
    /// System ID
    #[prost(uint32, tag = "2")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "3")]
    pub target_component: u32,
    /// 0=inputs, 1=outputs
    #[prost(uint32, tag = "4")]
    pub directory_type: u32,
    /// index of first directory entry to write
    #[prost(uint32, tag = "5")]
    pub start_index: u32,
    /// count of directory entries to write
    #[prost(uint32, tag = "6")]
    pub count: u32,
}
/// Acknowldge sucess or failure of a flexifunction command
///
/// MavLink id: 157
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct FlexifunctionCommand {
    /// System ID
    #[prost(uint32, tag = "1")]
    pub target_system: u32,
    /// Component ID
    #[prost(uint32, tag = "2")]
    pub target_component: u32,
    /// Flexifunction command type
    #[prost(uint32, tag = "3")]
    pub command_type: u32,
}
/// Acknowldge sucess or failure of a flexifunction command
///
/// MavLink id: 158
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct FlexifunctionCommandAck {
    /// Command acknowledged
    #[prost(uint32, tag = "1")]
    pub command_type: u32,
    /// result of acknowledge
    #[prost(uint32, tag = "2")]
    pub result: u32,
}
/// Backwards compatible MAVLink version of SERIAL_UDB_EXTRA - F2: Format Part A
///
/// MavLink id: 170
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SerialUdbExtraF2A {
    /// Serial UDB Extra Time
    #[prost(uint32, tag = "1")]
    pub sue_time: u32,
    /// Serial UDB Extra Latitude
    #[prost(int32, tag = "2")]
    pub sue_latitude: i32,
    /// Serial UDB Extra Longitude
    #[prost(int32, tag = "3")]
    pub sue_longitude: i32,
    /// Serial UDB Extra Altitude
    #[prost(int32, tag = "4")]
    pub sue_altitude: i32,
    /// Serial UDB Extra Waypoint Index
    #[prost(uint32, tag = "5")]
    pub sue_waypoint_index: u32,
    /// Serial UDB Extra Rmat 0
    #[prost(int32, tag = "6")]
    pub sue_rmat0: i32,
    /// Serial UDB Extra Rmat 1
    #[prost(int32, tag = "7")]
    pub sue_rmat1: i32,
    /// Serial UDB Extra Rmat 2
    #[prost(int32, tag = "8")]
    pub sue_rmat2: i32,
    /// Serial UDB Extra Rmat 3
    #[prost(int32, tag = "9")]
    pub sue_rmat3: i32,
    /// Serial UDB Extra Rmat 4
    #[prost(int32, tag = "10")]
    pub sue_rmat4: i32,
    /// Serial UDB Extra Rmat 5
    #[prost(int32, tag = "11")]
    pub sue_rmat5: i32,
    /// Serial UDB Extra Rmat 6
    #[prost(int32, tag = "12")]
    pub sue_rmat6: i32,
    /// Serial UDB Extra Rmat 7
    #[prost(int32, tag = "13")]
    pub sue_rmat7: i32,
    /// Serial UDB Extra Rmat 8
    #[prost(int32, tag = "14")]
    pub sue_rmat8: i32,
    /// Serial UDB Extra GPS Course Over Ground
    #[prost(uint32, tag = "15")]
    pub sue_cog: u32,
    /// Serial UDB Extra Speed Over Ground
    #[prost(int32, tag = "16")]
    pub sue_sog: i32,
    /// Serial UDB Extra CPU Load
    #[prost(uint32, tag = "17")]
    pub sue_cpu_load: u32,
    /// Serial UDB Extra 3D IMU Air Speed
    #[prost(uint32, tag = "18")]
    pub sue_air_speed_3dimu: u32,
    /// Serial UDB Extra Estimated Wind 0
    #[prost(int32, tag = "19")]
    pub sue_estimated_wind_0: i32,
    /// Serial UDB Extra Estimated Wind 1
    #[prost(int32, tag = "20")]
    pub sue_estimated_wind_1: i32,
    /// Serial UDB Extra Estimated Wind 2
    #[prost(int32, tag = "21")]
    pub sue_estimated_wind_2: i32,
    /// Serial UDB Extra Magnetic Field Earth 0
    #[prost(int32, tag = "22")]
    pub sue_mag_field_earth0: i32,
    /// Serial UDB Extra Magnetic Field Earth 1
    #[prost(int32, tag = "23")]
    pub sue_mag_field_earth1: i32,
    /// Serial UDB Extra Magnetic Field Earth 2
    #[prost(int32, tag = "24")]
    pub sue_mag_field_earth2: i32,
    /// Serial UDB Extra Number of Sattelites in View
    #[prost(int32, tag = "25")]
    pub sue_svs: i32,
    /// Serial UDB Extra GPS Horizontal Dilution of Precision
    #[prost(int32, tag = "26")]
    pub sue_hdop: i32,
    /// Serial UDB Extra Status
    #[prost(uint32, tag = "27")]
    pub sue_status: u32,
}
/// Backwards compatible version of SERIAL_UDB_EXTRA - F2: Part B
///
/// MavLink id: 171
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SerialUdbExtraF2B {
    /// Serial UDB Extra Time
    #[prost(uint32, tag = "1")]
    pub sue_time: u32,
    /// Serial UDB Extra Status Flags
    #[prost(uint32, tag = "2")]
    pub sue_flags: u32,
    /// SUE barometer pressure
    #[prost(int32, tag = "3")]
    pub sue_barom_press: i32,
    /// SUE barometer altitude
    #[prost(int32, tag = "4")]
    pub sue_barom_alt: i32,
    /// Serial UDB Extra PWM Input Channel 1
    #[prost(int32, tag = "5")]
    pub sue_pwm_input_1: i32,
    /// Serial UDB Extra PWM Input Channel 2
    #[prost(int32, tag = "6")]
    pub sue_pwm_input_2: i32,
    /// Serial UDB Extra PWM Input Channel 3
    #[prost(int32, tag = "7")]
    pub sue_pwm_input_3: i32,
    /// Serial UDB Extra PWM Input Channel 4
    #[prost(int32, tag = "8")]
    pub sue_pwm_input_4: i32,
    /// Serial UDB Extra PWM Input Channel 5
    #[prost(int32, tag = "9")]
    pub sue_pwm_input_5: i32,
    /// Serial UDB Extra PWM Input Channel 6
    #[prost(int32, tag = "10")]
    pub sue_pwm_input_6: i32,
    /// Serial UDB Extra PWM Input Channel 7
    #[prost(int32, tag = "11")]
    pub sue_pwm_input_7: i32,
    /// Serial UDB Extra PWM Input Channel 8
    #[prost(int32, tag = "12")]
    pub sue_pwm_input_8: i32,
    /// Serial UDB Extra PWM Input Channel 9
    #[prost(int32, tag = "13")]
    pub sue_pwm_input_9: i32,
    /// Serial UDB Extra PWM Input Channel 10
    #[prost(int32, tag = "14")]
    pub sue_pwm_input_10: i32,
    /// Serial UDB Extra PWM Input Channel 11
    #[prost(int32, tag = "15")]
    pub sue_pwm_input_11: i32,
    /// Serial UDB Extra PWM Input Channel 12
    #[prost(int32, tag = "16")]
    pub sue_pwm_input_12: i32,
    /// Serial UDB Extra PWM Output Channel 1
    #[prost(int32, tag = "17")]
    pub sue_pwm_output_1: i32,
    /// Serial UDB Extra PWM Output Channel 2
    #[prost(int32, tag = "18")]
    pub sue_pwm_output_2: i32,
    /// Serial UDB Extra PWM Output Channel 3
    #[prost(int32, tag = "19")]
    pub sue_pwm_output_3: i32,
    /// Serial UDB Extra PWM Output Channel 4
    #[prost(int32, tag = "20")]
    pub sue_pwm_output_4: i32,
    /// Serial UDB Extra PWM Output Channel 5
    #[prost(int32, tag = "21")]
    pub sue_pwm_output_5: i32,
    /// Serial UDB Extra PWM Output Channel 6
    #[prost(int32, tag = "22")]
    pub sue_pwm_output_6: i32,
    /// Serial UDB Extra PWM Output Channel 7
    #[prost(int32, tag = "23")]
    pub sue_pwm_output_7: i32,
    /// Serial UDB Extra PWM Output Channel 8
    #[prost(int32, tag = "24")]
    pub sue_pwm_output_8: i32,
    /// Serial UDB Extra PWM Output Channel 9
    #[prost(int32, tag = "25")]
    pub sue_pwm_output_9: i32,
    /// Serial UDB Extra PWM Output Channel 10
    #[prost(int32, tag = "26")]
    pub sue_pwm_output_10: i32,
    /// Serial UDB Extra PWM Output Channel 11
    #[prost(int32, tag = "27")]
    pub sue_pwm_output_11: i32,
    /// Serial UDB Extra PWM Output Channel 12
    #[prost(int32, tag = "28")]
    pub sue_pwm_output_12: i32,
    /// Serial UDB Extra IMU Location X
    #[prost(int32, tag = "29")]
    pub sue_imu_location_x: i32,
    /// Serial UDB Extra IMU Location Y
    #[prost(int32, tag = "30")]
    pub sue_imu_location_y: i32,
    /// Serial UDB Extra IMU Location Z
    #[prost(int32, tag = "31")]
    pub sue_imu_location_z: i32,
    /// Serial UDB Location Error Earth X
    #[prost(int32, tag = "32")]
    pub sue_location_error_earth_x: i32,
    /// Serial UDB Location Error Earth Y
    #[prost(int32, tag = "33")]
    pub sue_location_error_earth_y: i32,
    /// Serial UDB Location Error Earth Z
    #[prost(int32, tag = "34")]
    pub sue_location_error_earth_z: i32,
    /// Serial UDB Extra Oscillator Failure Count
    #[prost(int32, tag = "35")]
    pub sue_osc_fails: i32,
    /// Serial UDB Extra IMU Velocity X
    #[prost(int32, tag = "36")]
    pub sue_imu_velocity_x: i32,
    /// Serial UDB Extra IMU Velocity Y
    #[prost(int32, tag = "37")]
    pub sue_imu_velocity_y: i32,
    /// Serial UDB Extra IMU Velocity Z
    #[prost(int32, tag = "38")]
    pub sue_imu_velocity_z: i32,
    /// Serial UDB Extra Current Waypoint Goal X
    #[prost(int32, tag = "39")]
    pub sue_waypoint_goal_x: i32,
    /// Serial UDB Extra Current Waypoint Goal Y
    #[prost(int32, tag = "40")]
    pub sue_waypoint_goal_y: i32,
    /// Serial UDB Extra Current Waypoint Goal Z
    #[prost(int32, tag = "41")]
    pub sue_waypoint_goal_z: i32,
    /// Aeroforce in UDB X Axis
    #[prost(int32, tag = "42")]
    pub sue_aero_x: i32,
    /// Aeroforce in UDB Y Axis
    #[prost(int32, tag = "43")]
    pub sue_aero_y: i32,
    /// Aeroforce in UDB Z axis
    #[prost(int32, tag = "44")]
    pub sue_aero_z: i32,
    /// SUE barometer temperature
    #[prost(int32, tag = "45")]
    pub sue_barom_temp: i32,
    /// SUE battery voltage
    #[prost(int32, tag = "46")]
    pub sue_bat_volt: i32,
    /// SUE battery current
    #[prost(int32, tag = "47")]
    pub sue_bat_amp: i32,
    /// SUE battery milli amp hours used
    #[prost(int32, tag = "48")]
    pub sue_bat_amp_hours: i32,
    /// Sue autopilot desired height
    #[prost(int32, tag = "49")]
    pub sue_desired_height: i32,
    /// Serial UDB Extra Stack Memory Free
    #[prost(int32, tag = "50")]
    pub sue_memory_stack_free: i32,
}
/// Backwards compatible version of SERIAL_UDB_EXTRA F4: format
///
/// MavLink id: 172
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SerialUdbExtraF4 {
    /// Serial UDB Extra Roll Stabilization with Ailerons Enabled
    #[prost(uint32, tag = "1")]
    pub sue_roll_stabilization_ailerons: u32,
    /// Serial UDB Extra Roll Stabilization with Rudder Enabled
    #[prost(uint32, tag = "2")]
    pub sue_roll_stabilization_rudder: u32,
    /// Serial UDB Extra Pitch Stabilization Enabled
    #[prost(uint32, tag = "3")]
    pub sue_pitch_stabilization: u32,
    /// Serial UDB Extra Yaw Stabilization using Rudder Enabled
    #[prost(uint32, tag = "4")]
    pub sue_yaw_stabilization_rudder: u32,
    /// Serial UDB Extra Yaw Stabilization using Ailerons Enabled
    #[prost(uint32, tag = "5")]
    pub sue_yaw_stabilization_aileron: u32,
    /// Serial UDB Extra Navigation with Ailerons Enabled
    #[prost(uint32, tag = "6")]
    pub sue_aileron_navigation: u32,
    /// Serial UDB Extra Navigation with Rudder Enabled
    #[prost(uint32, tag = "7")]
    pub sue_rudder_navigation: u32,
    /// Serial UDB Extra Type of Alitude Hold when in Stabilized Mode
    #[prost(uint32, tag = "8")]
    pub sue_altitudehold_stabilized: u32,
    /// Serial UDB Extra Type of Alitude Hold when in Waypoint Mode
    #[prost(uint32, tag = "9")]
    pub sue_altitudehold_waypoint: u32,
    /// Serial UDB Extra Firmware racing mode enabled
    #[prost(uint32, tag = "10")]
    pub sue_racing_mode: u32,
}
/// Backwards compatible version of SERIAL_UDB_EXTRA F5: format
///
/// MavLink id: 173
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SerialUdbExtraF5 {
    /// Serial UDB YAWKP_AILERON Gain for Proporional control of navigation
    #[prost(float, tag = "1")]
    pub sue_yawkp_aileron: f32,
    /// Serial UDB YAWKD_AILERON Gain for Rate control of navigation
    #[prost(float, tag = "2")]
    pub sue_yawkd_aileron: f32,
    /// Serial UDB Extra ROLLKP Gain for Proportional control of roll stabilization
    #[prost(float, tag = "3")]
    pub sue_rollkp: f32,
    /// Serial UDB Extra ROLLKD Gain for Rate control of roll stabilization
    #[prost(float, tag = "4")]
    pub sue_rollkd: f32,
}
/// Backwards compatible version of SERIAL_UDB_EXTRA F6: format
///
/// MavLink id: 174
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SerialUdbExtraF6 {
    /// Serial UDB Extra PITCHGAIN Proportional Control
    #[prost(float, tag = "1")]
    pub sue_pitchgain: f32,
    /// Serial UDB Extra Pitch Rate Control
    #[prost(float, tag = "2")]
    pub sue_pitchkd: f32,
    /// Serial UDB Extra Rudder to Elevator Mix
    #[prost(float, tag = "3")]
    pub sue_rudder_elev_mix: f32,
    /// Serial UDB Extra Roll to Elevator Mix
    #[prost(float, tag = "4")]
    pub sue_roll_elev_mix: f32,
    /// Gain For Boosting Manual Elevator control When Plane Stabilized
    #[prost(float, tag = "5")]
    pub sue_elevator_boost: f32,
}
/// Backwards compatible version of SERIAL_UDB_EXTRA F7: format
///
/// MavLink id: 175
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SerialUdbExtraF7 {
    /// Serial UDB YAWKP_RUDDER Gain for Proporional control of navigation
    #[prost(float, tag = "1")]
    pub sue_yawkp_rudder: f32,
    /// Serial UDB YAWKD_RUDDER Gain for Rate control of navigation
    #[prost(float, tag = "2")]
    pub sue_yawkd_rudder: f32,
    /// Serial UDB Extra ROLLKP_RUDDER Gain for Proportional control of roll stabilization
    #[prost(float, tag = "3")]
    pub sue_rollkp_rudder: f32,
    /// Serial UDB Extra ROLLKD_RUDDER Gain for Rate control of roll stabilization
    #[prost(float, tag = "4")]
    pub sue_rollkd_rudder: f32,
    /// SERIAL UDB EXTRA Rudder Boost Gain to Manual Control when stabilized
    #[prost(float, tag = "5")]
    pub sue_rudder_boost: f32,
    /// Serial UDB Extra Return To Landing - Angle to Pitch Plane Down
    #[prost(float, tag = "6")]
    pub sue_rtl_pitch_down: f32,
}
/// Backwards compatible version of SERIAL_UDB_EXTRA F8: format
///
/// MavLink id: 176
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SerialUdbExtraF8 {
    /// Serial UDB Extra HEIGHT_TARGET_MAX
    #[prost(float, tag = "1")]
    pub sue_height_target_max: f32,
    /// Serial UDB Extra HEIGHT_TARGET_MIN
    #[prost(float, tag = "2")]
    pub sue_height_target_min: f32,
    /// Serial UDB Extra ALT_HOLD_THROTTLE_MIN
    #[prost(float, tag = "3")]
    pub sue_alt_hold_throttle_min: f32,
    /// Serial UDB Extra ALT_HOLD_THROTTLE_MAX
    #[prost(float, tag = "4")]
    pub sue_alt_hold_throttle_max: f32,
    /// Serial UDB Extra ALT_HOLD_PITCH_MIN
    #[prost(float, tag = "5")]
    pub sue_alt_hold_pitch_min: f32,
    /// Serial UDB Extra ALT_HOLD_PITCH_MAX
    #[prost(float, tag = "6")]
    pub sue_alt_hold_pitch_max: f32,
    /// Serial UDB Extra ALT_HOLD_PITCH_HIGH
    #[prost(float, tag = "7")]
    pub sue_alt_hold_pitch_high: f32,
}
/// Backwards compatible version of SERIAL_UDB_EXTRA F13: format
///
/// MavLink id: 177
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SerialUdbExtraF13 {
    /// Serial UDB Extra MP Origin Latitude
    #[prost(int32, tag = "1")]
    pub sue_lat_origin: i32,
    /// Serial UDB Extra MP Origin Longitude
    #[prost(int32, tag = "2")]
    pub sue_lon_origin: i32,
    /// Serial UDB Extra MP Origin Altitude Above Sea Level
    #[prost(int32, tag = "3")]
    pub sue_alt_origin: i32,
    /// Serial UDB Extra GPS Week Number
    #[prost(int32, tag = "4")]
    pub sue_week_no: i32,
}
/// Backwards compatible version of SERIAL_UDB_EXTRA F14: format
///
/// MavLink id: 178
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SerialUdbExtraF14 {
    /// Serial UDB Extra Type Program Address of Last Trap
    #[prost(uint32, tag = "1")]
    pub sue_trap_source: u32,
    /// Serial UDB Extra Reboot Register of DSPIC
    #[prost(int32, tag = "2")]
    pub sue_rcon: i32,
    /// Serial UDB Extra  Last dspic Trap Flags
    #[prost(int32, tag = "3")]
    pub sue_trap_flags: i32,
    /// Serial UDB Extra Number of Ocillator Failures
    #[prost(int32, tag = "4")]
    pub sue_osc_fail_count: i32,
    /// Serial UDB Extra Wind Estimation Enabled
    #[prost(uint32, tag = "5")]
    pub sue_wind_estimation: u32,
    /// Serial UDB Extra Type of GPS Unit
    #[prost(uint32, tag = "6")]
    pub sue_gps_type: u32,
    /// Serial UDB Extra Dead Reckoning Enabled
    #[prost(uint32, tag = "7")]
    pub sue_dr: u32,
    /// Serial UDB Extra Type of UDB Hardware
    #[prost(uint32, tag = "8")]
    pub sue_board_type: u32,
    /// Serial UDB Extra Type of Airframe
    #[prost(uint32, tag = "9")]
    pub sue_airframe: u32,
    /// Serial UDB Extra UDB Internal Clock Configuration
    #[prost(uint32, tag = "10")]
    pub sue_clock_config: u32,
    /// Serial UDB Extra Type of Flight Plan
    #[prost(uint32, tag = "11")]
    pub sue_flight_plan_type: u32,
}
/// Backwards compatible version of SERIAL_UDB_EXTRA F15 format
///
/// MavLink id: 179
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SerialUdbExtraF15 {
    /// Serial UDB Extra Model Name Of Vehicle
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub sue_id_vehicle_model_name: ::prost::alloc::vec::Vec<u32>,
    /// Serial UDB Extra Registraton Number of Vehicle
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub sue_id_vehicle_registration: ::prost::alloc::vec::Vec<u32>,
}
/// Backwards compatible version of SERIAL_UDB_EXTRA F16 format
///
/// MavLink id: 180
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SerialUdbExtraF16 {
    /// Serial UDB Extra Name of Expected Lead Pilot
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub sue_id_lead_pilot: ::prost::alloc::vec::Vec<u32>,
    /// Serial UDB Extra URL of Lead Pilot or Team
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub sue_id_diy_drones_url: ::prost::alloc::vec::Vec<u32>,
}
/// The altitude measured by sensors and IMU
///
/// MavLink id: 181
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Altitudes {
    /// Timestamp (milliseconds since system boot)
    #[prost(uint32, tag = "1")]
    pub time_boot_ms: u32,
    /// GPS altitude (MSL) in meters, expressed as * 1000 (millimeters)
    #[prost(int32, tag = "2")]
    pub alt_gps: i32,
    /// IMU altitude above ground in meters, expressed as * 1000 (millimeters)
    #[prost(int32, tag = "3")]
    pub alt_imu: i32,
    /// barometeric altitude above ground in meters, expressed as * 1000 (millimeters)
    #[prost(int32, tag = "4")]
    pub alt_barometric: i32,
    /// Optical flow altitude above ground in meters, expressed as * 1000 (millimeters)
    #[prost(int32, tag = "5")]
    pub alt_optical_flow: i32,
    /// Rangefinder Altitude above ground in meters, expressed as * 1000 (millimeters)
    #[prost(int32, tag = "6")]
    pub alt_range_finder: i32,
    /// Extra altitude above ground in meters, expressed as * 1000 (millimeters)
    #[prost(int32, tag = "7")]
    pub alt_extra: i32,
}
/// The airspeed measured by sensors and IMU
///
/// MavLink id: 182
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Airspeeds {
    /// Timestamp (milliseconds since system boot)
    #[prost(uint32, tag = "1")]
    pub time_boot_ms: u32,
    /// Airspeed estimate from IMU, cm/s
    #[prost(int32, tag = "2")]
    pub airspeed_imu: i32,
    /// Pitot measured forward airpseed, cm/s
    #[prost(int32, tag = "3")]
    pub airspeed_pitot: i32,
    /// Hot wire anenometer measured airspeed, cm/s
    #[prost(int32, tag = "4")]
    pub airspeed_hot_wire: i32,
    /// Ultrasonic measured airspeed, cm/s
    #[prost(int32, tag = "5")]
    pub airspeed_ultrasonic: i32,
    /// Angle of attack sensor, degrees * 10
    #[prost(int32, tag = "6")]
    pub aoa: i32,
    /// Yaw angle sensor, degrees * 10
    #[prost(int32, tag = "7")]
    pub aoy: i32,
}
/// Backwards compatible version of SERIAL_UDB_EXTRA F17 format
///
/// MavLink id: 183
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SerialUdbExtraF17 {
    /// SUE Feed Forward Gain
    #[prost(float, tag = "1")]
    pub sue_feed_forward: f32,
    /// SUE Max Turn Rate when Navigating
    #[prost(float, tag = "2")]
    pub sue_turn_rate_nav: f32,
    /// SUE Max Turn Rate in Fly By Wire Mode
    #[prost(float, tag = "3")]
    pub sue_turn_rate_fbw: f32,
}
/// Backwards compatible version of SERIAL_UDB_EXTRA F18 format
///
/// MavLink id: 184
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SerialUdbExtraF18 {
    /// SUE Angle of Attack Normal
    #[prost(float, tag = "1")]
    pub angle_of_attack_normal: f32,
    /// SUE Angle of Attack Inverted
    #[prost(float, tag = "2")]
    pub angle_of_attack_inverted: f32,
    /// SUE Elevator Trim Normal
    #[prost(float, tag = "3")]
    pub elevator_trim_normal: f32,
    /// SUE Elevator Trim Inverted
    #[prost(float, tag = "4")]
    pub elevator_trim_inverted: f32,
    /// SUE reference_speed
    #[prost(float, tag = "5")]
    pub reference_speed: f32,
}
/// Backwards compatible version of SERIAL_UDB_EXTRA F19 format
///
/// MavLink id: 185
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SerialUdbExtraF19 {
    /// SUE aileron output channel
    #[prost(uint32, tag = "1")]
    pub sue_aileron_output_channel: u32,
    /// SUE aileron reversed
    #[prost(uint32, tag = "2")]
    pub sue_aileron_reversed: u32,
    /// SUE elevator output channel
    #[prost(uint32, tag = "3")]
    pub sue_elevator_output_channel: u32,
    /// SUE elevator reversed
    #[prost(uint32, tag = "4")]
    pub sue_elevator_reversed: u32,
    /// SUE throttle output channel
    #[prost(uint32, tag = "5")]
    pub sue_throttle_output_channel: u32,
    /// SUE throttle reversed
    #[prost(uint32, tag = "6")]
    pub sue_throttle_reversed: u32,
    /// SUE rudder output channel
    #[prost(uint32, tag = "7")]
    pub sue_rudder_output_channel: u32,
    /// SUE rudder reversed
    #[prost(uint32, tag = "8")]
    pub sue_rudder_reversed: u32,
}
/// Backwards compatible version of SERIAL_UDB_EXTRA F20 format
///
/// MavLink id: 186
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SerialUdbExtraF20 {
    /// SUE UDB PWM Trim Value on Input 1
    #[prost(int32, tag = "1")]
    pub sue_trim_value_input_1: i32,
    /// SUE UDB PWM Trim Value on Input 2
    #[prost(int32, tag = "2")]
    pub sue_trim_value_input_2: i32,
    /// SUE UDB PWM Trim Value on Input 3
    #[prost(int32, tag = "3")]
    pub sue_trim_value_input_3: i32,
    /// SUE UDB PWM Trim Value on Input 4
    #[prost(int32, tag = "4")]
    pub sue_trim_value_input_4: i32,
    /// SUE UDB PWM Trim Value on Input 5
    #[prost(int32, tag = "5")]
    pub sue_trim_value_input_5: i32,
    /// SUE UDB PWM Trim Value on Input 6
    #[prost(int32, tag = "6")]
    pub sue_trim_value_input_6: i32,
    /// SUE UDB PWM Trim Value on Input 7
    #[prost(int32, tag = "7")]
    pub sue_trim_value_input_7: i32,
    /// SUE UDB PWM Trim Value on Input 8
    #[prost(int32, tag = "8")]
    pub sue_trim_value_input_8: i32,
    /// SUE UDB PWM Trim Value on Input 9
    #[prost(int32, tag = "9")]
    pub sue_trim_value_input_9: i32,
    /// SUE UDB PWM Trim Value on Input 10
    #[prost(int32, tag = "10")]
    pub sue_trim_value_input_10: i32,
    /// SUE UDB PWM Trim Value on Input 11
    #[prost(int32, tag = "11")]
    pub sue_trim_value_input_11: i32,
    /// SUE UDB PWM Trim Value on Input 12
    #[prost(int32, tag = "12")]
    pub sue_trim_value_input_12: i32,
    /// SUE Number of Input Channels
    #[prost(uint32, tag = "13")]
    pub sue_number_of_inputs: u32,
}
/// Backwards compatible version of SERIAL_UDB_EXTRA F21 format
///
/// MavLink id: 187
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SerialUdbExtraF21 {
    /// SUE X accelerometer offset
    #[prost(int32, tag = "1")]
    pub sue_accel_x_offset: i32,
    /// SUE Y accelerometer offset
    #[prost(int32, tag = "2")]
    pub sue_accel_y_offset: i32,
    /// SUE Z accelerometer offset
    #[prost(int32, tag = "3")]
    pub sue_accel_z_offset: i32,
    /// SUE X gyro offset
    #[prost(int32, tag = "4")]
    pub sue_gyro_x_offset: i32,
    /// SUE Y gyro offset
    #[prost(int32, tag = "5")]
    pub sue_gyro_y_offset: i32,
    /// SUE Z gyro offset
    #[prost(int32, tag = "6")]
    pub sue_gyro_z_offset: i32,
}
/// Backwards compatible version of SERIAL_UDB_EXTRA F22 format
///
/// MavLink id: 188
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SerialUdbExtraF22 {
    /// SUE X accelerometer at calibration time
    #[prost(int32, tag = "1")]
    pub sue_accel_x_at_calibration: i32,
    /// SUE Y accelerometer at calibration time
    #[prost(int32, tag = "2")]
    pub sue_accel_y_at_calibration: i32,
    /// SUE Z accelerometer at calibration time
    #[prost(int32, tag = "3")]
    pub sue_accel_z_at_calibration: i32,
    /// SUE X gyro at calibration time
    #[prost(int32, tag = "4")]
    pub sue_gyro_x_at_calibration: i32,
    /// SUE Y gyro at calibration time
    #[prost(int32, tag = "5")]
    pub sue_gyro_y_at_calibration: i32,
    /// SUE Z gyro at calibration time
    #[prost(int32, tag = "6")]
    pub sue_gyro_z_at_calibration: i32,
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
pub enum MavPreflightStorageAction {
    /// Action required when performing CMD_PREFLIGHT_STORAGE
    /// Read all parameters from storage
    MavPfsCmdReadAll = 0,
    /// Write all parameters to storage
    MavPfsCmdWriteAll = 1,
    /// Clear all  parameters in storage
    MavPfsCmdClearAll = 2,
    /// Read specific parameters from storage
    MavPfsCmdReadSpecific = 3,
    /// Write specific parameters to storage
    MavPfsCmdWriteSpecific = 4,
    /// Clear specific parameters in storage
    MavPfsCmdClearSpecific = 5,
    /// do nothing
    MavPfsCmdDoNothing = 6,
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
    /// Request storage of different parameter values and logs. This command will be only accepted if in pre-flight mode.
    PreflightStorageAdvanced = 0,
    /// ***** START Params
    /// Storage action: Action defined by MAV_PREFLIGHT_STORAGE_ACTION_ADVANCED
    /// Storage area as defined by parameter database
    /// Storage flags as defined by parameter database
    /// Empty
    /// Empty
    /// Empty
    /// Empty
    /// ***** END Params
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
    /// Configure digital camera. This is a fallback message for systems that have not yet implemented PARAM_EXT_XXX messages and camera definition files (see <https://mavlink.io/en/services/camera_def.html> ).
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
    /// Control digital camera. This is a fallback message for systems that have not yet implemented PARAM_EXT_XXX messages and camera definition files (see <https://mavlink.io/en/services/camera_def.html> ).
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
    /// Request a target system to start an upgrade of one (or all) of its components. For example, the command might be sent to a companion computer to cause it to upgrade a connected flight controller. The system doing the upgrade will report progress using the normal command protocol sequence for a long running operation. Command protocol information: <https://mavlink.io/en/services/command.html.>
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
