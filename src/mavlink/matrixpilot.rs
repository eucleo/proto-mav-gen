// This file was automatically generated, do not edit
#[allow(unused_imports)]
use crate::mavlink::common::*;
#[allow(unused_imports)]
use bitflags::bitflags;
#[allow(unused_imports)]
use bytes::{Buf, BufMut, Bytes, BytesMut};
#[allow(unused_imports)]
use num_derive::FromPrimitive;
#[allow(unused_imports)]
use num_derive::ToPrimitive;
#[allow(unused_imports)]
use num_traits::FromPrimitive;
#[allow(unused_imports)]
use num_traits::ToPrimitive;
#[allow(unused_imports)]
use prost::Message as ProstMessage;
use proto_mav_comm::MavlinkVersion;
use proto_mav_comm::{error::*, Message};
impl crate::proto::matrixpilot::FlexifunctionSet {
    pub const ENCODED_LEN: usize = 2usize;
    pub fn mavlink_deser(_version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < Self::ENCODED_LEN {
            let mut payload_buf = [0; Self::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        #[allow(clippy::field_reassign_with_default)]
        {
            let mut _struct = Self::default();
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp
    }
}
impl crate::proto::matrixpilot::FlexifunctionReadReq {
    pub const ENCODED_LEN: usize = 6usize;
    pub fn mavlink_deser(_version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < Self::ENCODED_LEN {
            let mut payload_buf = [0; Self::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        #[allow(clippy::field_reassign_with_default)]
        {
            let mut _struct = Self::default();
            _struct.read_req_type = buf.get_i16_le() as i32;
            _struct.data_index = buf.get_i16_le() as i32;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i16_le(self.read_req_type as i16);
        _tmp.put_i16_le(self.data_index as i16);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp
    }
}
impl crate::proto::matrixpilot::FlexifunctionBufferFunction {
    pub const ENCODED_LEN: usize = 58usize;
    pub fn mavlink_deser(_version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < Self::ENCODED_LEN {
            let mut payload_buf = [0; Self::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        #[allow(clippy::field_reassign_with_default)]
        {
            let mut _struct = Self::default();
            _struct.func_index = buf.get_u16_le() as u32;
            _struct.func_count = buf.get_u16_le() as u32;
            _struct.data_address = buf.get_u16_le() as u32;
            _struct.data_size = buf.get_u16_le() as u32;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            for _ in 0..48usize {
                let val = buf.get_i8() as i32;
                #[allow(clippy::useless_conversion)]
                _struct.data.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.func_index as u16);
        _tmp.put_u16_le(self.func_count as u16);
        _tmp.put_u16_le(self.data_address as u16);
        _tmp.put_u16_le(self.data_size as u16);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        for val in &self.data {
            _tmp.put_i8(*val as i8);
        }
        _tmp
    }
}
impl crate::proto::matrixpilot::FlexifunctionBufferFunctionAck {
    pub const ENCODED_LEN: usize = 6usize;
    pub fn mavlink_deser(_version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < Self::ENCODED_LEN {
            let mut payload_buf = [0; Self::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        #[allow(clippy::field_reassign_with_default)]
        {
            let mut _struct = Self::default();
            _struct.func_index = buf.get_u16_le() as u32;
            _struct.result = buf.get_u16_le() as u32;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.func_index as u16);
        _tmp.put_u16_le(self.result as u16);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp
    }
}
impl crate::proto::matrixpilot::FlexifunctionDirectory {
    pub const ENCODED_LEN: usize = 53usize;
    pub fn mavlink_deser(_version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < Self::ENCODED_LEN {
            let mut payload_buf = [0; Self::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        #[allow(clippy::field_reassign_with_default)]
        {
            let mut _struct = Self::default();
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            _struct.directory_type = buf.get_u8() as u32;
            _struct.start_index = buf.get_u8() as u32;
            _struct.count = buf.get_u8() as u32;
            for _ in 0..48usize {
                let val = buf.get_i8() as i32;
                #[allow(clippy::useless_conversion)]
                _struct.directory_data.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp.put_u8(self.directory_type as u8);
        _tmp.put_u8(self.start_index as u8);
        _tmp.put_u8(self.count as u8);
        for val in &self.directory_data {
            _tmp.put_i8(*val as i8);
        }
        _tmp
    }
}
impl crate::proto::matrixpilot::FlexifunctionDirectoryAck {
    pub const ENCODED_LEN: usize = 7usize;
    pub fn mavlink_deser(_version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < Self::ENCODED_LEN {
            let mut payload_buf = [0; Self::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        #[allow(clippy::field_reassign_with_default)]
        {
            let mut _struct = Self::default();
            _struct.result = buf.get_u16_le() as u32;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            _struct.directory_type = buf.get_u8() as u32;
            _struct.start_index = buf.get_u8() as u32;
            _struct.count = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.result as u16);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp.put_u8(self.directory_type as u8);
        _tmp.put_u8(self.start_index as u8);
        _tmp.put_u8(self.count as u8);
        _tmp
    }
}
impl crate::proto::matrixpilot::FlexifunctionCommand {
    pub const ENCODED_LEN: usize = 3usize;
    pub fn mavlink_deser(_version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < Self::ENCODED_LEN {
            let mut payload_buf = [0; Self::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        #[allow(clippy::field_reassign_with_default)]
        {
            let mut _struct = Self::default();
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            _struct.command_type = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp.put_u8(self.command_type as u8);
        _tmp
    }
}
impl crate::proto::matrixpilot::FlexifunctionCommandAck {
    pub const ENCODED_LEN: usize = 4usize;
    pub fn mavlink_deser(_version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < Self::ENCODED_LEN {
            let mut payload_buf = [0; Self::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        #[allow(clippy::field_reassign_with_default)]
        {
            let mut _struct = Self::default();
            _struct.command_type = buf.get_u16_le() as u32;
            _struct.result = buf.get_u16_le() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.command_type as u16);
        _tmp.put_u16_le(self.result as u16);
        _tmp
    }
}
impl crate::proto::matrixpilot::SerialUdbExtraF2A {
    pub const ENCODED_LEN: usize = 61usize;
    pub fn mavlink_deser(_version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < Self::ENCODED_LEN {
            let mut payload_buf = [0; Self::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        #[allow(clippy::field_reassign_with_default)]
        {
            let mut _struct = Self::default();
            _struct.sue_time = buf.get_u32_le();
            _struct.sue_latitude = buf.get_i32_le();
            _struct.sue_longitude = buf.get_i32_le();
            _struct.sue_altitude = buf.get_i32_le();
            _struct.sue_waypoint_index = buf.get_u16_le() as u32;
            _struct.sue_rmat0 = buf.get_i16_le() as i32;
            _struct.sue_rmat1 = buf.get_i16_le() as i32;
            _struct.sue_rmat2 = buf.get_i16_le() as i32;
            _struct.sue_rmat3 = buf.get_i16_le() as i32;
            _struct.sue_rmat4 = buf.get_i16_le() as i32;
            _struct.sue_rmat5 = buf.get_i16_le() as i32;
            _struct.sue_rmat6 = buf.get_i16_le() as i32;
            _struct.sue_rmat7 = buf.get_i16_le() as i32;
            _struct.sue_rmat8 = buf.get_i16_le() as i32;
            _struct.sue_cog = buf.get_u16_le() as u32;
            _struct.sue_sog = buf.get_i16_le() as i32;
            _struct.sue_cpu_load = buf.get_u16_le() as u32;
            _struct.sue_air_speed_3dimu = buf.get_u16_le() as u32;
            _struct.sue_estimated_wind_0 = buf.get_i16_le() as i32;
            _struct.sue_estimated_wind_1 = buf.get_i16_le() as i32;
            _struct.sue_estimated_wind_2 = buf.get_i16_le() as i32;
            _struct.sue_mag_field_earth0 = buf.get_i16_le() as i32;
            _struct.sue_mag_field_earth1 = buf.get_i16_le() as i32;
            _struct.sue_mag_field_earth2 = buf.get_i16_le() as i32;
            _struct.sue_svs = buf.get_i16_le() as i32;
            _struct.sue_hdop = buf.get_i16_le() as i32;
            _struct.sue_status = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.sue_time as u32);
        _tmp.put_i32_le(self.sue_latitude as i32);
        _tmp.put_i32_le(self.sue_longitude as i32);
        _tmp.put_i32_le(self.sue_altitude as i32);
        _tmp.put_u16_le(self.sue_waypoint_index as u16);
        _tmp.put_i16_le(self.sue_rmat0 as i16);
        _tmp.put_i16_le(self.sue_rmat1 as i16);
        _tmp.put_i16_le(self.sue_rmat2 as i16);
        _tmp.put_i16_le(self.sue_rmat3 as i16);
        _tmp.put_i16_le(self.sue_rmat4 as i16);
        _tmp.put_i16_le(self.sue_rmat5 as i16);
        _tmp.put_i16_le(self.sue_rmat6 as i16);
        _tmp.put_i16_le(self.sue_rmat7 as i16);
        _tmp.put_i16_le(self.sue_rmat8 as i16);
        _tmp.put_u16_le(self.sue_cog as u16);
        _tmp.put_i16_le(self.sue_sog as i16);
        _tmp.put_u16_le(self.sue_cpu_load as u16);
        _tmp.put_u16_le(self.sue_air_speed_3dimu as u16);
        _tmp.put_i16_le(self.sue_estimated_wind_0 as i16);
        _tmp.put_i16_le(self.sue_estimated_wind_1 as i16);
        _tmp.put_i16_le(self.sue_estimated_wind_2 as i16);
        _tmp.put_i16_le(self.sue_mag_field_earth0 as i16);
        _tmp.put_i16_le(self.sue_mag_field_earth1 as i16);
        _tmp.put_i16_le(self.sue_mag_field_earth2 as i16);
        _tmp.put_i16_le(self.sue_svs as i16);
        _tmp.put_i16_le(self.sue_hdop as i16);
        _tmp.put_u8(self.sue_status as u8);
        _tmp
    }
}
impl crate::proto::matrixpilot::SerialUdbExtraF2B {
    pub const ENCODED_LEN: usize = 108usize;
    pub fn mavlink_deser(_version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < Self::ENCODED_LEN {
            let mut payload_buf = [0; Self::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        #[allow(clippy::field_reassign_with_default)]
        {
            let mut _struct = Self::default();
            _struct.sue_time = buf.get_u32_le();
            _struct.sue_flags = buf.get_u32_le();
            _struct.sue_barom_press = buf.get_i32_le();
            _struct.sue_barom_alt = buf.get_i32_le();
            _struct.sue_pwm_input_1 = buf.get_i16_le() as i32;
            _struct.sue_pwm_input_2 = buf.get_i16_le() as i32;
            _struct.sue_pwm_input_3 = buf.get_i16_le() as i32;
            _struct.sue_pwm_input_4 = buf.get_i16_le() as i32;
            _struct.sue_pwm_input_5 = buf.get_i16_le() as i32;
            _struct.sue_pwm_input_6 = buf.get_i16_le() as i32;
            _struct.sue_pwm_input_7 = buf.get_i16_le() as i32;
            _struct.sue_pwm_input_8 = buf.get_i16_le() as i32;
            _struct.sue_pwm_input_9 = buf.get_i16_le() as i32;
            _struct.sue_pwm_input_10 = buf.get_i16_le() as i32;
            _struct.sue_pwm_input_11 = buf.get_i16_le() as i32;
            _struct.sue_pwm_input_12 = buf.get_i16_le() as i32;
            _struct.sue_pwm_output_1 = buf.get_i16_le() as i32;
            _struct.sue_pwm_output_2 = buf.get_i16_le() as i32;
            _struct.sue_pwm_output_3 = buf.get_i16_le() as i32;
            _struct.sue_pwm_output_4 = buf.get_i16_le() as i32;
            _struct.sue_pwm_output_5 = buf.get_i16_le() as i32;
            _struct.sue_pwm_output_6 = buf.get_i16_le() as i32;
            _struct.sue_pwm_output_7 = buf.get_i16_le() as i32;
            _struct.sue_pwm_output_8 = buf.get_i16_le() as i32;
            _struct.sue_pwm_output_9 = buf.get_i16_le() as i32;
            _struct.sue_pwm_output_10 = buf.get_i16_le() as i32;
            _struct.sue_pwm_output_11 = buf.get_i16_le() as i32;
            _struct.sue_pwm_output_12 = buf.get_i16_le() as i32;
            _struct.sue_imu_location_x = buf.get_i16_le() as i32;
            _struct.sue_imu_location_y = buf.get_i16_le() as i32;
            _struct.sue_imu_location_z = buf.get_i16_le() as i32;
            _struct.sue_location_error_earth_x = buf.get_i16_le() as i32;
            _struct.sue_location_error_earth_y = buf.get_i16_le() as i32;
            _struct.sue_location_error_earth_z = buf.get_i16_le() as i32;
            _struct.sue_osc_fails = buf.get_i16_le() as i32;
            _struct.sue_imu_velocity_x = buf.get_i16_le() as i32;
            _struct.sue_imu_velocity_y = buf.get_i16_le() as i32;
            _struct.sue_imu_velocity_z = buf.get_i16_le() as i32;
            _struct.sue_waypoint_goal_x = buf.get_i16_le() as i32;
            _struct.sue_waypoint_goal_y = buf.get_i16_le() as i32;
            _struct.sue_waypoint_goal_z = buf.get_i16_le() as i32;
            _struct.sue_aero_x = buf.get_i16_le() as i32;
            _struct.sue_aero_y = buf.get_i16_le() as i32;
            _struct.sue_aero_z = buf.get_i16_le() as i32;
            _struct.sue_barom_temp = buf.get_i16_le() as i32;
            _struct.sue_bat_volt = buf.get_i16_le() as i32;
            _struct.sue_bat_amp = buf.get_i16_le() as i32;
            _struct.sue_bat_amp_hours = buf.get_i16_le() as i32;
            _struct.sue_desired_height = buf.get_i16_le() as i32;
            _struct.sue_memory_stack_free = buf.get_i16_le() as i32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.sue_time as u32);
        _tmp.put_u32_le(self.sue_flags as u32);
        _tmp.put_i32_le(self.sue_barom_press as i32);
        _tmp.put_i32_le(self.sue_barom_alt as i32);
        _tmp.put_i16_le(self.sue_pwm_input_1 as i16);
        _tmp.put_i16_le(self.sue_pwm_input_2 as i16);
        _tmp.put_i16_le(self.sue_pwm_input_3 as i16);
        _tmp.put_i16_le(self.sue_pwm_input_4 as i16);
        _tmp.put_i16_le(self.sue_pwm_input_5 as i16);
        _tmp.put_i16_le(self.sue_pwm_input_6 as i16);
        _tmp.put_i16_le(self.sue_pwm_input_7 as i16);
        _tmp.put_i16_le(self.sue_pwm_input_8 as i16);
        _tmp.put_i16_le(self.sue_pwm_input_9 as i16);
        _tmp.put_i16_le(self.sue_pwm_input_10 as i16);
        _tmp.put_i16_le(self.sue_pwm_input_11 as i16);
        _tmp.put_i16_le(self.sue_pwm_input_12 as i16);
        _tmp.put_i16_le(self.sue_pwm_output_1 as i16);
        _tmp.put_i16_le(self.sue_pwm_output_2 as i16);
        _tmp.put_i16_le(self.sue_pwm_output_3 as i16);
        _tmp.put_i16_le(self.sue_pwm_output_4 as i16);
        _tmp.put_i16_le(self.sue_pwm_output_5 as i16);
        _tmp.put_i16_le(self.sue_pwm_output_6 as i16);
        _tmp.put_i16_le(self.sue_pwm_output_7 as i16);
        _tmp.put_i16_le(self.sue_pwm_output_8 as i16);
        _tmp.put_i16_le(self.sue_pwm_output_9 as i16);
        _tmp.put_i16_le(self.sue_pwm_output_10 as i16);
        _tmp.put_i16_le(self.sue_pwm_output_11 as i16);
        _tmp.put_i16_le(self.sue_pwm_output_12 as i16);
        _tmp.put_i16_le(self.sue_imu_location_x as i16);
        _tmp.put_i16_le(self.sue_imu_location_y as i16);
        _tmp.put_i16_le(self.sue_imu_location_z as i16);
        _tmp.put_i16_le(self.sue_location_error_earth_x as i16);
        _tmp.put_i16_le(self.sue_location_error_earth_y as i16);
        _tmp.put_i16_le(self.sue_location_error_earth_z as i16);
        _tmp.put_i16_le(self.sue_osc_fails as i16);
        _tmp.put_i16_le(self.sue_imu_velocity_x as i16);
        _tmp.put_i16_le(self.sue_imu_velocity_y as i16);
        _tmp.put_i16_le(self.sue_imu_velocity_z as i16);
        _tmp.put_i16_le(self.sue_waypoint_goal_x as i16);
        _tmp.put_i16_le(self.sue_waypoint_goal_y as i16);
        _tmp.put_i16_le(self.sue_waypoint_goal_z as i16);
        _tmp.put_i16_le(self.sue_aero_x as i16);
        _tmp.put_i16_le(self.sue_aero_y as i16);
        _tmp.put_i16_le(self.sue_aero_z as i16);
        _tmp.put_i16_le(self.sue_barom_temp as i16);
        _tmp.put_i16_le(self.sue_bat_volt as i16);
        _tmp.put_i16_le(self.sue_bat_amp as i16);
        _tmp.put_i16_le(self.sue_bat_amp_hours as i16);
        _tmp.put_i16_le(self.sue_desired_height as i16);
        _tmp.put_i16_le(self.sue_memory_stack_free as i16);
        _tmp
    }
}
impl crate::proto::matrixpilot::SerialUdbExtraF4 {
    pub const ENCODED_LEN: usize = 10usize;
    pub fn mavlink_deser(_version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < Self::ENCODED_LEN {
            let mut payload_buf = [0; Self::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        #[allow(clippy::field_reassign_with_default)]
        {
            let mut _struct = Self::default();
            _struct.sue_roll_stabilization_ailerons = buf.get_u8() as u32;
            _struct.sue_roll_stabilization_rudder = buf.get_u8() as u32;
            _struct.sue_pitch_stabilization = buf.get_u8() as u32;
            _struct.sue_yaw_stabilization_rudder = buf.get_u8() as u32;
            _struct.sue_yaw_stabilization_aileron = buf.get_u8() as u32;
            _struct.sue_aileron_navigation = buf.get_u8() as u32;
            _struct.sue_rudder_navigation = buf.get_u8() as u32;
            _struct.sue_altitudehold_stabilized = buf.get_u8() as u32;
            _struct.sue_altitudehold_waypoint = buf.get_u8() as u32;
            _struct.sue_racing_mode = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.sue_roll_stabilization_ailerons as u8);
        _tmp.put_u8(self.sue_roll_stabilization_rudder as u8);
        _tmp.put_u8(self.sue_pitch_stabilization as u8);
        _tmp.put_u8(self.sue_yaw_stabilization_rudder as u8);
        _tmp.put_u8(self.sue_yaw_stabilization_aileron as u8);
        _tmp.put_u8(self.sue_aileron_navigation as u8);
        _tmp.put_u8(self.sue_rudder_navigation as u8);
        _tmp.put_u8(self.sue_altitudehold_stabilized as u8);
        _tmp.put_u8(self.sue_altitudehold_waypoint as u8);
        _tmp.put_u8(self.sue_racing_mode as u8);
        _tmp
    }
}
impl crate::proto::matrixpilot::SerialUdbExtraF5 {
    pub const ENCODED_LEN: usize = 16usize;
    pub fn mavlink_deser(_version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < Self::ENCODED_LEN {
            let mut payload_buf = [0; Self::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        #[allow(clippy::field_reassign_with_default)]
        {
            let mut _struct = Self::default();
            _struct.sue_yawkp_aileron = buf.get_f32_le();
            _struct.sue_yawkd_aileron = buf.get_f32_le();
            _struct.sue_rollkp = buf.get_f32_le();
            _struct.sue_rollkd = buf.get_f32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.sue_yawkp_aileron as f32);
        _tmp.put_f32_le(self.sue_yawkd_aileron as f32);
        _tmp.put_f32_le(self.sue_rollkp as f32);
        _tmp.put_f32_le(self.sue_rollkd as f32);
        _tmp
    }
}
impl crate::proto::matrixpilot::SerialUdbExtraF6 {
    pub const ENCODED_LEN: usize = 20usize;
    pub fn mavlink_deser(_version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < Self::ENCODED_LEN {
            let mut payload_buf = [0; Self::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        #[allow(clippy::field_reassign_with_default)]
        {
            let mut _struct = Self::default();
            _struct.sue_pitchgain = buf.get_f32_le();
            _struct.sue_pitchkd = buf.get_f32_le();
            _struct.sue_rudder_elev_mix = buf.get_f32_le();
            _struct.sue_roll_elev_mix = buf.get_f32_le();
            _struct.sue_elevator_boost = buf.get_f32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.sue_pitchgain as f32);
        _tmp.put_f32_le(self.sue_pitchkd as f32);
        _tmp.put_f32_le(self.sue_rudder_elev_mix as f32);
        _tmp.put_f32_le(self.sue_roll_elev_mix as f32);
        _tmp.put_f32_le(self.sue_elevator_boost as f32);
        _tmp
    }
}
impl crate::proto::matrixpilot::SerialUdbExtraF7 {
    pub const ENCODED_LEN: usize = 24usize;
    pub fn mavlink_deser(_version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < Self::ENCODED_LEN {
            let mut payload_buf = [0; Self::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        #[allow(clippy::field_reassign_with_default)]
        {
            let mut _struct = Self::default();
            _struct.sue_yawkp_rudder = buf.get_f32_le();
            _struct.sue_yawkd_rudder = buf.get_f32_le();
            _struct.sue_rollkp_rudder = buf.get_f32_le();
            _struct.sue_rollkd_rudder = buf.get_f32_le();
            _struct.sue_rudder_boost = buf.get_f32_le();
            _struct.sue_rtl_pitch_down = buf.get_f32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.sue_yawkp_rudder as f32);
        _tmp.put_f32_le(self.sue_yawkd_rudder as f32);
        _tmp.put_f32_le(self.sue_rollkp_rudder as f32);
        _tmp.put_f32_le(self.sue_rollkd_rudder as f32);
        _tmp.put_f32_le(self.sue_rudder_boost as f32);
        _tmp.put_f32_le(self.sue_rtl_pitch_down as f32);
        _tmp
    }
}
impl crate::proto::matrixpilot::SerialUdbExtraF8 {
    pub const ENCODED_LEN: usize = 28usize;
    pub fn mavlink_deser(_version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < Self::ENCODED_LEN {
            let mut payload_buf = [0; Self::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        #[allow(clippy::field_reassign_with_default)]
        {
            let mut _struct = Self::default();
            _struct.sue_height_target_max = buf.get_f32_le();
            _struct.sue_height_target_min = buf.get_f32_le();
            _struct.sue_alt_hold_throttle_min = buf.get_f32_le();
            _struct.sue_alt_hold_throttle_max = buf.get_f32_le();
            _struct.sue_alt_hold_pitch_min = buf.get_f32_le();
            _struct.sue_alt_hold_pitch_max = buf.get_f32_le();
            _struct.sue_alt_hold_pitch_high = buf.get_f32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.sue_height_target_max as f32);
        _tmp.put_f32_le(self.sue_height_target_min as f32);
        _tmp.put_f32_le(self.sue_alt_hold_throttle_min as f32);
        _tmp.put_f32_le(self.sue_alt_hold_throttle_max as f32);
        _tmp.put_f32_le(self.sue_alt_hold_pitch_min as f32);
        _tmp.put_f32_le(self.sue_alt_hold_pitch_max as f32);
        _tmp.put_f32_le(self.sue_alt_hold_pitch_high as f32);
        _tmp
    }
}
impl crate::proto::matrixpilot::SerialUdbExtraF13 {
    pub const ENCODED_LEN: usize = 14usize;
    pub fn mavlink_deser(_version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < Self::ENCODED_LEN {
            let mut payload_buf = [0; Self::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        #[allow(clippy::field_reassign_with_default)]
        {
            let mut _struct = Self::default();
            _struct.sue_lat_origin = buf.get_i32_le();
            _struct.sue_lon_origin = buf.get_i32_le();
            _struct.sue_alt_origin = buf.get_i32_le();
            _struct.sue_week_no = buf.get_i16_le() as i32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i32_le(self.sue_lat_origin as i32);
        _tmp.put_i32_le(self.sue_lon_origin as i32);
        _tmp.put_i32_le(self.sue_alt_origin as i32);
        _tmp.put_i16_le(self.sue_week_no as i16);
        _tmp
    }
}
impl crate::proto::matrixpilot::SerialUdbExtraF14 {
    pub const ENCODED_LEN: usize = 17usize;
    pub fn mavlink_deser(_version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < Self::ENCODED_LEN {
            let mut payload_buf = [0; Self::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        #[allow(clippy::field_reassign_with_default)]
        {
            let mut _struct = Self::default();
            _struct.sue_trap_source = buf.get_u32_le();
            _struct.sue_rcon = buf.get_i16_le() as i32;
            _struct.sue_trap_flags = buf.get_i16_le() as i32;
            _struct.sue_osc_fail_count = buf.get_i16_le() as i32;
            _struct.sue_wind_estimation = buf.get_u8() as u32;
            _struct.sue_gps_type = buf.get_u8() as u32;
            _struct.sue_dr = buf.get_u8() as u32;
            _struct.sue_board_type = buf.get_u8() as u32;
            _struct.sue_airframe = buf.get_u8() as u32;
            _struct.sue_clock_config = buf.get_u8() as u32;
            _struct.sue_flight_plan_type = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.sue_trap_source as u32);
        _tmp.put_i16_le(self.sue_rcon as i16);
        _tmp.put_i16_le(self.sue_trap_flags as i16);
        _tmp.put_i16_le(self.sue_osc_fail_count as i16);
        _tmp.put_u8(self.sue_wind_estimation as u8);
        _tmp.put_u8(self.sue_gps_type as u8);
        _tmp.put_u8(self.sue_dr as u8);
        _tmp.put_u8(self.sue_board_type as u8);
        _tmp.put_u8(self.sue_airframe as u8);
        _tmp.put_u8(self.sue_clock_config as u8);
        _tmp.put_u8(self.sue_flight_plan_type as u8);
        _tmp
    }
}
impl crate::proto::matrixpilot::SerialUdbExtraF15 {
    pub const ENCODED_LEN: usize = 60usize;
    pub fn mavlink_deser(_version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < Self::ENCODED_LEN {
            let mut payload_buf = [0; Self::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        #[allow(clippy::field_reassign_with_default)]
        {
            let mut _struct = Self::default();
            for _ in 0..40usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.sue_id_vehicle_model_name.push(val.into());
            }
            for _ in 0..20usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.sue_id_vehicle_registration.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        for val in &self.sue_id_vehicle_model_name {
            _tmp.put_u8(*val as u8);
        }
        for val in &self.sue_id_vehicle_registration {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
impl crate::proto::matrixpilot::SerialUdbExtraF16 {
    pub const ENCODED_LEN: usize = 110usize;
    pub fn mavlink_deser(_version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < Self::ENCODED_LEN {
            let mut payload_buf = [0; Self::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        #[allow(clippy::field_reassign_with_default)]
        {
            let mut _struct = Self::default();
            for _ in 0..40usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.sue_id_lead_pilot.push(val.into());
            }
            for _ in 0..70usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.sue_id_diy_drones_url.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        for val in &self.sue_id_lead_pilot {
            _tmp.put_u8(*val as u8);
        }
        for val in &self.sue_id_diy_drones_url {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
impl crate::proto::matrixpilot::Altitudes {
    pub const ENCODED_LEN: usize = 28usize;
    pub fn mavlink_deser(_version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < Self::ENCODED_LEN {
            let mut payload_buf = [0; Self::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        #[allow(clippy::field_reassign_with_default)]
        {
            let mut _struct = Self::default();
            _struct.time_boot_ms = buf.get_u32_le();
            _struct.alt_gps = buf.get_i32_le();
            _struct.alt_imu = buf.get_i32_le();
            _struct.alt_barometric = buf.get_i32_le();
            _struct.alt_optical_flow = buf.get_i32_le();
            _struct.alt_range_finder = buf.get_i32_le();
            _struct.alt_extra = buf.get_i32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms as u32);
        _tmp.put_i32_le(self.alt_gps as i32);
        _tmp.put_i32_le(self.alt_imu as i32);
        _tmp.put_i32_le(self.alt_barometric as i32);
        _tmp.put_i32_le(self.alt_optical_flow as i32);
        _tmp.put_i32_le(self.alt_range_finder as i32);
        _tmp.put_i32_le(self.alt_extra as i32);
        _tmp
    }
}
impl crate::proto::matrixpilot::Airspeeds {
    pub const ENCODED_LEN: usize = 16usize;
    pub fn mavlink_deser(_version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < Self::ENCODED_LEN {
            let mut payload_buf = [0; Self::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        #[allow(clippy::field_reassign_with_default)]
        {
            let mut _struct = Self::default();
            _struct.time_boot_ms = buf.get_u32_le();
            _struct.airspeed_imu = buf.get_i16_le() as i32;
            _struct.airspeed_pitot = buf.get_i16_le() as i32;
            _struct.airspeed_hot_wire = buf.get_i16_le() as i32;
            _struct.airspeed_ultrasonic = buf.get_i16_le() as i32;
            _struct.aoa = buf.get_i16_le() as i32;
            _struct.aoy = buf.get_i16_le() as i32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms as u32);
        _tmp.put_i16_le(self.airspeed_imu as i16);
        _tmp.put_i16_le(self.airspeed_pitot as i16);
        _tmp.put_i16_le(self.airspeed_hot_wire as i16);
        _tmp.put_i16_le(self.airspeed_ultrasonic as i16);
        _tmp.put_i16_le(self.aoa as i16);
        _tmp.put_i16_le(self.aoy as i16);
        _tmp
    }
}
impl crate::proto::matrixpilot::SerialUdbExtraF17 {
    pub const ENCODED_LEN: usize = 12usize;
    pub fn mavlink_deser(_version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < Self::ENCODED_LEN {
            let mut payload_buf = [0; Self::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        #[allow(clippy::field_reassign_with_default)]
        {
            let mut _struct = Self::default();
            _struct.sue_feed_forward = buf.get_f32_le();
            _struct.sue_turn_rate_nav = buf.get_f32_le();
            _struct.sue_turn_rate_fbw = buf.get_f32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.sue_feed_forward as f32);
        _tmp.put_f32_le(self.sue_turn_rate_nav as f32);
        _tmp.put_f32_le(self.sue_turn_rate_fbw as f32);
        _tmp
    }
}
impl crate::proto::matrixpilot::SerialUdbExtraF18 {
    pub const ENCODED_LEN: usize = 20usize;
    pub fn mavlink_deser(_version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < Self::ENCODED_LEN {
            let mut payload_buf = [0; Self::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        #[allow(clippy::field_reassign_with_default)]
        {
            let mut _struct = Self::default();
            _struct.angle_of_attack_normal = buf.get_f32_le();
            _struct.angle_of_attack_inverted = buf.get_f32_le();
            _struct.elevator_trim_normal = buf.get_f32_le();
            _struct.elevator_trim_inverted = buf.get_f32_le();
            _struct.reference_speed = buf.get_f32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.angle_of_attack_normal as f32);
        _tmp.put_f32_le(self.angle_of_attack_inverted as f32);
        _tmp.put_f32_le(self.elevator_trim_normal as f32);
        _tmp.put_f32_le(self.elevator_trim_inverted as f32);
        _tmp.put_f32_le(self.reference_speed as f32);
        _tmp
    }
}
impl crate::proto::matrixpilot::SerialUdbExtraF19 {
    pub const ENCODED_LEN: usize = 8usize;
    pub fn mavlink_deser(_version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < Self::ENCODED_LEN {
            let mut payload_buf = [0; Self::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        #[allow(clippy::field_reassign_with_default)]
        {
            let mut _struct = Self::default();
            _struct.sue_aileron_output_channel = buf.get_u8() as u32;
            _struct.sue_aileron_reversed = buf.get_u8() as u32;
            _struct.sue_elevator_output_channel = buf.get_u8() as u32;
            _struct.sue_elevator_reversed = buf.get_u8() as u32;
            _struct.sue_throttle_output_channel = buf.get_u8() as u32;
            _struct.sue_throttle_reversed = buf.get_u8() as u32;
            _struct.sue_rudder_output_channel = buf.get_u8() as u32;
            _struct.sue_rudder_reversed = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.sue_aileron_output_channel as u8);
        _tmp.put_u8(self.sue_aileron_reversed as u8);
        _tmp.put_u8(self.sue_elevator_output_channel as u8);
        _tmp.put_u8(self.sue_elevator_reversed as u8);
        _tmp.put_u8(self.sue_throttle_output_channel as u8);
        _tmp.put_u8(self.sue_throttle_reversed as u8);
        _tmp.put_u8(self.sue_rudder_output_channel as u8);
        _tmp.put_u8(self.sue_rudder_reversed as u8);
        _tmp
    }
}
impl crate::proto::matrixpilot::SerialUdbExtraF20 {
    pub const ENCODED_LEN: usize = 25usize;
    pub fn mavlink_deser(_version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < Self::ENCODED_LEN {
            let mut payload_buf = [0; Self::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        #[allow(clippy::field_reassign_with_default)]
        {
            let mut _struct = Self::default();
            _struct.sue_trim_value_input_1 = buf.get_i16_le() as i32;
            _struct.sue_trim_value_input_2 = buf.get_i16_le() as i32;
            _struct.sue_trim_value_input_3 = buf.get_i16_le() as i32;
            _struct.sue_trim_value_input_4 = buf.get_i16_le() as i32;
            _struct.sue_trim_value_input_5 = buf.get_i16_le() as i32;
            _struct.sue_trim_value_input_6 = buf.get_i16_le() as i32;
            _struct.sue_trim_value_input_7 = buf.get_i16_le() as i32;
            _struct.sue_trim_value_input_8 = buf.get_i16_le() as i32;
            _struct.sue_trim_value_input_9 = buf.get_i16_le() as i32;
            _struct.sue_trim_value_input_10 = buf.get_i16_le() as i32;
            _struct.sue_trim_value_input_11 = buf.get_i16_le() as i32;
            _struct.sue_trim_value_input_12 = buf.get_i16_le() as i32;
            _struct.sue_number_of_inputs = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i16_le(self.sue_trim_value_input_1 as i16);
        _tmp.put_i16_le(self.sue_trim_value_input_2 as i16);
        _tmp.put_i16_le(self.sue_trim_value_input_3 as i16);
        _tmp.put_i16_le(self.sue_trim_value_input_4 as i16);
        _tmp.put_i16_le(self.sue_trim_value_input_5 as i16);
        _tmp.put_i16_le(self.sue_trim_value_input_6 as i16);
        _tmp.put_i16_le(self.sue_trim_value_input_7 as i16);
        _tmp.put_i16_le(self.sue_trim_value_input_8 as i16);
        _tmp.put_i16_le(self.sue_trim_value_input_9 as i16);
        _tmp.put_i16_le(self.sue_trim_value_input_10 as i16);
        _tmp.put_i16_le(self.sue_trim_value_input_11 as i16);
        _tmp.put_i16_le(self.sue_trim_value_input_12 as i16);
        _tmp.put_u8(self.sue_number_of_inputs as u8);
        _tmp
    }
}
impl crate::proto::matrixpilot::SerialUdbExtraF21 {
    pub const ENCODED_LEN: usize = 12usize;
    pub fn mavlink_deser(_version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < Self::ENCODED_LEN {
            let mut payload_buf = [0; Self::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        #[allow(clippy::field_reassign_with_default)]
        {
            let mut _struct = Self::default();
            _struct.sue_accel_x_offset = buf.get_i16_le() as i32;
            _struct.sue_accel_y_offset = buf.get_i16_le() as i32;
            _struct.sue_accel_z_offset = buf.get_i16_le() as i32;
            _struct.sue_gyro_x_offset = buf.get_i16_le() as i32;
            _struct.sue_gyro_y_offset = buf.get_i16_le() as i32;
            _struct.sue_gyro_z_offset = buf.get_i16_le() as i32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i16_le(self.sue_accel_x_offset as i16);
        _tmp.put_i16_le(self.sue_accel_y_offset as i16);
        _tmp.put_i16_le(self.sue_accel_z_offset as i16);
        _tmp.put_i16_le(self.sue_gyro_x_offset as i16);
        _tmp.put_i16_le(self.sue_gyro_y_offset as i16);
        _tmp.put_i16_le(self.sue_gyro_z_offset as i16);
        _tmp
    }
}
impl crate::proto::matrixpilot::SerialUdbExtraF22 {
    pub const ENCODED_LEN: usize = 12usize;
    pub fn mavlink_deser(_version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < Self::ENCODED_LEN {
            let mut payload_buf = [0; Self::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        #[allow(clippy::field_reassign_with_default)]
        {
            let mut _struct = Self::default();
            _struct.sue_accel_x_at_calibration = buf.get_i16_le() as i32;
            _struct.sue_accel_y_at_calibration = buf.get_i16_le() as i32;
            _struct.sue_accel_z_at_calibration = buf.get_i16_le() as i32;
            _struct.sue_gyro_x_at_calibration = buf.get_i16_le() as i32;
            _struct.sue_gyro_y_at_calibration = buf.get_i16_le() as i32;
            _struct.sue_gyro_z_at_calibration = buf.get_i16_le() as i32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i16_le(self.sue_accel_x_at_calibration as i16);
        _tmp.put_i16_le(self.sue_accel_y_at_calibration as i16);
        _tmp.put_i16_le(self.sue_accel_z_at_calibration as i16);
        _tmp.put_i16_le(self.sue_gyro_x_at_calibration as i16);
        _tmp.put_i16_le(self.sue_gyro_y_at_calibration as i16);
        _tmp.put_i16_le(self.sue_gyro_z_at_calibration as i16);
        _tmp
    }
}
#[derive(Clone, PartialEq, Debug)]
pub enum MavMessage {
    FlexifunctionSet(crate::proto::matrixpilot::FlexifunctionSet),
    FlexifunctionReadReq(crate::proto::matrixpilot::FlexifunctionReadReq),
    FlexifunctionBufferFunction(crate::proto::matrixpilot::FlexifunctionBufferFunction),
    FlexifunctionBufferFunctionAck(crate::proto::matrixpilot::FlexifunctionBufferFunctionAck),
    FlexifunctionDirectory(crate::proto::matrixpilot::FlexifunctionDirectory),
    FlexifunctionDirectoryAck(crate::proto::matrixpilot::FlexifunctionDirectoryAck),
    FlexifunctionCommand(crate::proto::matrixpilot::FlexifunctionCommand),
    FlexifunctionCommandAck(crate::proto::matrixpilot::FlexifunctionCommandAck),
    SerialUdbExtraF2A(crate::proto::matrixpilot::SerialUdbExtraF2A),
    SerialUdbExtraF2B(crate::proto::matrixpilot::SerialUdbExtraF2B),
    SerialUdbExtraF4(crate::proto::matrixpilot::SerialUdbExtraF4),
    SerialUdbExtraF5(crate::proto::matrixpilot::SerialUdbExtraF5),
    SerialUdbExtraF6(crate::proto::matrixpilot::SerialUdbExtraF6),
    SerialUdbExtraF7(crate::proto::matrixpilot::SerialUdbExtraF7),
    SerialUdbExtraF8(crate::proto::matrixpilot::SerialUdbExtraF8),
    SerialUdbExtraF13(crate::proto::matrixpilot::SerialUdbExtraF13),
    SerialUdbExtraF14(crate::proto::matrixpilot::SerialUdbExtraF14),
    SerialUdbExtraF15(crate::proto::matrixpilot::SerialUdbExtraF15),
    SerialUdbExtraF16(crate::proto::matrixpilot::SerialUdbExtraF16),
    Altitudes(crate::proto::matrixpilot::Altitudes),
    Airspeeds(crate::proto::matrixpilot::Airspeeds),
    SerialUdbExtraF17(crate::proto::matrixpilot::SerialUdbExtraF17),
    SerialUdbExtraF18(crate::proto::matrixpilot::SerialUdbExtraF18),
    SerialUdbExtraF19(crate::proto::matrixpilot::SerialUdbExtraF19),
    SerialUdbExtraF20(crate::proto::matrixpilot::SerialUdbExtraF20),
    SerialUdbExtraF21(crate::proto::matrixpilot::SerialUdbExtraF21),
    SerialUdbExtraF22(crate::proto::matrixpilot::SerialUdbExtraF22),
    Common(crate::mavlink::common::MavMessage),
}
impl From<crate::mavlink::common::MavMessage> for MavMessage {
    fn from(message: crate::mavlink::common::MavMessage) -> Self {
        MavMessage::Common(message)
    }
}
impl Message for MavMessage {
    fn parse(version: MavlinkVersion, id: u32, payload: &[u8]) -> Result<MavMessage, ParserError> {
        match id {
            150 => crate::proto::matrixpilot::FlexifunctionSet::mavlink_deser(version, payload)
                .map(MavMessage::FlexifunctionSet),
            151 => crate::proto::matrixpilot::FlexifunctionReadReq::mavlink_deser(version, payload)
                .map(MavMessage::FlexifunctionReadReq),
            152 => crate::proto::matrixpilot::FlexifunctionBufferFunction::mavlink_deser(
                version, payload,
            )
            .map(MavMessage::FlexifunctionBufferFunction),
            153 => crate::proto::matrixpilot::FlexifunctionBufferFunctionAck::mavlink_deser(
                version, payload,
            )
            .map(MavMessage::FlexifunctionBufferFunctionAck),
            155 => {
                crate::proto::matrixpilot::FlexifunctionDirectory::mavlink_deser(version, payload)
                    .map(MavMessage::FlexifunctionDirectory)
            }
            156 => crate::proto::matrixpilot::FlexifunctionDirectoryAck::mavlink_deser(
                version, payload,
            )
            .map(MavMessage::FlexifunctionDirectoryAck),
            157 => crate::proto::matrixpilot::FlexifunctionCommand::mavlink_deser(version, payload)
                .map(MavMessage::FlexifunctionCommand),
            158 => {
                crate::proto::matrixpilot::FlexifunctionCommandAck::mavlink_deser(version, payload)
                    .map(MavMessage::FlexifunctionCommandAck)
            }
            170 => crate::proto::matrixpilot::SerialUdbExtraF2A::mavlink_deser(version, payload)
                .map(MavMessage::SerialUdbExtraF2A),
            171 => crate::proto::matrixpilot::SerialUdbExtraF2B::mavlink_deser(version, payload)
                .map(MavMessage::SerialUdbExtraF2B),
            172 => crate::proto::matrixpilot::SerialUdbExtraF4::mavlink_deser(version, payload)
                .map(MavMessage::SerialUdbExtraF4),
            173 => crate::proto::matrixpilot::SerialUdbExtraF5::mavlink_deser(version, payload)
                .map(MavMessage::SerialUdbExtraF5),
            174 => crate::proto::matrixpilot::SerialUdbExtraF6::mavlink_deser(version, payload)
                .map(MavMessage::SerialUdbExtraF6),
            175 => crate::proto::matrixpilot::SerialUdbExtraF7::mavlink_deser(version, payload)
                .map(MavMessage::SerialUdbExtraF7),
            176 => crate::proto::matrixpilot::SerialUdbExtraF8::mavlink_deser(version, payload)
                .map(MavMessage::SerialUdbExtraF8),
            177 => crate::proto::matrixpilot::SerialUdbExtraF13::mavlink_deser(version, payload)
                .map(MavMessage::SerialUdbExtraF13),
            178 => crate::proto::matrixpilot::SerialUdbExtraF14::mavlink_deser(version, payload)
                .map(MavMessage::SerialUdbExtraF14),
            179 => crate::proto::matrixpilot::SerialUdbExtraF15::mavlink_deser(version, payload)
                .map(MavMessage::SerialUdbExtraF15),
            180 => crate::proto::matrixpilot::SerialUdbExtraF16::mavlink_deser(version, payload)
                .map(MavMessage::SerialUdbExtraF16),
            181 => crate::proto::matrixpilot::Altitudes::mavlink_deser(version, payload)
                .map(MavMessage::Altitudes),
            182 => crate::proto::matrixpilot::Airspeeds::mavlink_deser(version, payload)
                .map(MavMessage::Airspeeds),
            183 => crate::proto::matrixpilot::SerialUdbExtraF17::mavlink_deser(version, payload)
                .map(MavMessage::SerialUdbExtraF17),
            184 => crate::proto::matrixpilot::SerialUdbExtraF18::mavlink_deser(version, payload)
                .map(MavMessage::SerialUdbExtraF18),
            185 => crate::proto::matrixpilot::SerialUdbExtraF19::mavlink_deser(version, payload)
                .map(MavMessage::SerialUdbExtraF19),
            186 => crate::proto::matrixpilot::SerialUdbExtraF20::mavlink_deser(version, payload)
                .map(MavMessage::SerialUdbExtraF20),
            187 => crate::proto::matrixpilot::SerialUdbExtraF21::mavlink_deser(version, payload)
                .map(MavMessage::SerialUdbExtraF21),
            188 => crate::proto::matrixpilot::SerialUdbExtraF22::mavlink_deser(version, payload)
                .map(MavMessage::SerialUdbExtraF22),
            _ => {
                if let Ok(msg) = crate::mavlink::common::MavMessage::parse(version, id, payload) {
                    return Ok(MavMessage::Common(msg));
                }
                Err(ParserError::UnknownMessage { id })
            }
        }
    }
    fn message_name(&self) -> &'static str {
        match self {
            MavMessage::FlexifunctionSet(..) => "FlexifunctionSet",
            MavMessage::FlexifunctionReadReq(..) => "FlexifunctionReadReq",
            MavMessage::FlexifunctionBufferFunction(..) => "FlexifunctionBufferFunction",
            MavMessage::FlexifunctionBufferFunctionAck(..) => "FlexifunctionBufferFunctionAck",
            MavMessage::FlexifunctionDirectory(..) => "FlexifunctionDirectory",
            MavMessage::FlexifunctionDirectoryAck(..) => "FlexifunctionDirectoryAck",
            MavMessage::FlexifunctionCommand(..) => "FlexifunctionCommand",
            MavMessage::FlexifunctionCommandAck(..) => "FlexifunctionCommandAck",
            MavMessage::SerialUdbExtraF2A(..) => "SerialUdbExtraF2A",
            MavMessage::SerialUdbExtraF2B(..) => "SerialUdbExtraF2B",
            MavMessage::SerialUdbExtraF4(..) => "SerialUdbExtraF4",
            MavMessage::SerialUdbExtraF5(..) => "SerialUdbExtraF5",
            MavMessage::SerialUdbExtraF6(..) => "SerialUdbExtraF6",
            MavMessage::SerialUdbExtraF7(..) => "SerialUdbExtraF7",
            MavMessage::SerialUdbExtraF8(..) => "SerialUdbExtraF8",
            MavMessage::SerialUdbExtraF13(..) => "SerialUdbExtraF13",
            MavMessage::SerialUdbExtraF14(..) => "SerialUdbExtraF14",
            MavMessage::SerialUdbExtraF15(..) => "SerialUdbExtraF15",
            MavMessage::SerialUdbExtraF16(..) => "SerialUdbExtraF16",
            MavMessage::Altitudes(..) => "Altitudes",
            MavMessage::Airspeeds(..) => "Airspeeds",
            MavMessage::SerialUdbExtraF17(..) => "SerialUdbExtraF17",
            MavMessage::SerialUdbExtraF18(..) => "SerialUdbExtraF18",
            MavMessage::SerialUdbExtraF19(..) => "SerialUdbExtraF19",
            MavMessage::SerialUdbExtraF20(..) => "SerialUdbExtraF20",
            MavMessage::SerialUdbExtraF21(..) => "SerialUdbExtraF21",
            MavMessage::SerialUdbExtraF22(..) => "SerialUdbExtraF22",
            MavMessage::Common(msg) => msg.message_name(),
        }
    }
    fn message_id(&self) -> u32 {
        match self {
            MavMessage::FlexifunctionSet(..) => 150,
            MavMessage::FlexifunctionReadReq(..) => 151,
            MavMessage::FlexifunctionBufferFunction(..) => 152,
            MavMessage::FlexifunctionBufferFunctionAck(..) => 153,
            MavMessage::FlexifunctionDirectory(..) => 155,
            MavMessage::FlexifunctionDirectoryAck(..) => 156,
            MavMessage::FlexifunctionCommand(..) => 157,
            MavMessage::FlexifunctionCommandAck(..) => 158,
            MavMessage::SerialUdbExtraF2A(..) => 170,
            MavMessage::SerialUdbExtraF2B(..) => 171,
            MavMessage::SerialUdbExtraF4(..) => 172,
            MavMessage::SerialUdbExtraF5(..) => 173,
            MavMessage::SerialUdbExtraF6(..) => 174,
            MavMessage::SerialUdbExtraF7(..) => 175,
            MavMessage::SerialUdbExtraF8(..) => 176,
            MavMessage::SerialUdbExtraF13(..) => 177,
            MavMessage::SerialUdbExtraF14(..) => 178,
            MavMessage::SerialUdbExtraF15(..) => 179,
            MavMessage::SerialUdbExtraF16(..) => 180,
            MavMessage::Altitudes(..) => 181,
            MavMessage::Airspeeds(..) => 182,
            MavMessage::SerialUdbExtraF17(..) => 183,
            MavMessage::SerialUdbExtraF18(..) => 184,
            MavMessage::SerialUdbExtraF19(..) => 185,
            MavMessage::SerialUdbExtraF20(..) => 186,
            MavMessage::SerialUdbExtraF21(..) => 187,
            MavMessage::SerialUdbExtraF22(..) => 188,
            MavMessage::Common(msg) => msg.message_id(),
        }
    }
    fn message_id_from_name(name: &str) -> Result<u32, &'static str> {
        match name {
            "FlexifunctionSet" => Ok(150),
            "FlexifunctionReadReq" => Ok(151),
            "FlexifunctionBufferFunction" => Ok(152),
            "FlexifunctionBufferFunctionAck" => Ok(153),
            "FlexifunctionDirectory" => Ok(155),
            "FlexifunctionDirectoryAck" => Ok(156),
            "FlexifunctionCommand" => Ok(157),
            "FlexifunctionCommandAck" => Ok(158),
            "SerialUdbExtraF2A" => Ok(170),
            "SerialUdbExtraF2B" => Ok(171),
            "SerialUdbExtraF4" => Ok(172),
            "SerialUdbExtraF5" => Ok(173),
            "SerialUdbExtraF6" => Ok(174),
            "SerialUdbExtraF7" => Ok(175),
            "SerialUdbExtraF8" => Ok(176),
            "SerialUdbExtraF13" => Ok(177),
            "SerialUdbExtraF14" => Ok(178),
            "SerialUdbExtraF15" => Ok(179),
            "SerialUdbExtraF16" => Ok(180),
            "Altitudes" => Ok(181),
            "Airspeeds" => Ok(182),
            "SerialUdbExtraF17" => Ok(183),
            "SerialUdbExtraF18" => Ok(184),
            "SerialUdbExtraF19" => Ok(185),
            "SerialUdbExtraF20" => Ok(186),
            "SerialUdbExtraF21" => Ok(187),
            "SerialUdbExtraF22" => Ok(188),
            _ => {
                match crate::mavlink::common::MavMessage::message_id_from_name(name) {
                    Ok(name) => return Ok(name),
                    Err(..) => {}
                }
                Err("Invalid message name.")
            }
        }
    }
    fn default_message_from_id(id: u32) -> Result<MavMessage, &'static str> {
        match id {
            150 => Ok(MavMessage::FlexifunctionSet(
                crate::proto::matrixpilot::FlexifunctionSet::default(),
            )),
            151 => Ok(MavMessage::FlexifunctionReadReq(
                crate::proto::matrixpilot::FlexifunctionReadReq::default(),
            )),
            152 => Ok(MavMessage::FlexifunctionBufferFunction(
                crate::proto::matrixpilot::FlexifunctionBufferFunction::default(),
            )),
            153 => Ok(MavMessage::FlexifunctionBufferFunctionAck(
                crate::proto::matrixpilot::FlexifunctionBufferFunctionAck::default(),
            )),
            155 => Ok(MavMessage::FlexifunctionDirectory(
                crate::proto::matrixpilot::FlexifunctionDirectory::default(),
            )),
            156 => Ok(MavMessage::FlexifunctionDirectoryAck(
                crate::proto::matrixpilot::FlexifunctionDirectoryAck::default(),
            )),
            157 => Ok(MavMessage::FlexifunctionCommand(
                crate::proto::matrixpilot::FlexifunctionCommand::default(),
            )),
            158 => Ok(MavMessage::FlexifunctionCommandAck(
                crate::proto::matrixpilot::FlexifunctionCommandAck::default(),
            )),
            170 => Ok(MavMessage::SerialUdbExtraF2A(
                crate::proto::matrixpilot::SerialUdbExtraF2A::default(),
            )),
            171 => Ok(MavMessage::SerialUdbExtraF2B(
                crate::proto::matrixpilot::SerialUdbExtraF2B::default(),
            )),
            172 => Ok(MavMessage::SerialUdbExtraF4(
                crate::proto::matrixpilot::SerialUdbExtraF4::default(),
            )),
            173 => Ok(MavMessage::SerialUdbExtraF5(
                crate::proto::matrixpilot::SerialUdbExtraF5::default(),
            )),
            174 => Ok(MavMessage::SerialUdbExtraF6(
                crate::proto::matrixpilot::SerialUdbExtraF6::default(),
            )),
            175 => Ok(MavMessage::SerialUdbExtraF7(
                crate::proto::matrixpilot::SerialUdbExtraF7::default(),
            )),
            176 => Ok(MavMessage::SerialUdbExtraF8(
                crate::proto::matrixpilot::SerialUdbExtraF8::default(),
            )),
            177 => Ok(MavMessage::SerialUdbExtraF13(
                crate::proto::matrixpilot::SerialUdbExtraF13::default(),
            )),
            178 => Ok(MavMessage::SerialUdbExtraF14(
                crate::proto::matrixpilot::SerialUdbExtraF14::default(),
            )),
            179 => Ok(MavMessage::SerialUdbExtraF15(
                crate::proto::matrixpilot::SerialUdbExtraF15::default(),
            )),
            180 => Ok(MavMessage::SerialUdbExtraF16(
                crate::proto::matrixpilot::SerialUdbExtraF16::default(),
            )),
            181 => Ok(MavMessage::Altitudes(
                crate::proto::matrixpilot::Altitudes::default(),
            )),
            182 => Ok(MavMessage::Airspeeds(
                crate::proto::matrixpilot::Airspeeds::default(),
            )),
            183 => Ok(MavMessage::SerialUdbExtraF17(
                crate::proto::matrixpilot::SerialUdbExtraF17::default(),
            )),
            184 => Ok(MavMessage::SerialUdbExtraF18(
                crate::proto::matrixpilot::SerialUdbExtraF18::default(),
            )),
            185 => Ok(MavMessage::SerialUdbExtraF19(
                crate::proto::matrixpilot::SerialUdbExtraF19::default(),
            )),
            186 => Ok(MavMessage::SerialUdbExtraF20(
                crate::proto::matrixpilot::SerialUdbExtraF20::default(),
            )),
            187 => Ok(MavMessage::SerialUdbExtraF21(
                crate::proto::matrixpilot::SerialUdbExtraF21::default(),
            )),
            188 => Ok(MavMessage::SerialUdbExtraF22(
                crate::proto::matrixpilot::SerialUdbExtraF22::default(),
            )),
            _ => {
                if let Ok(msg) = crate::mavlink::common::MavMessage::default_message_from_id(id) {
                    return Ok(MavMessage::Common(msg));
                }
                Err("Invalid message id.")
            }
        }
    }
    fn mavlink_ser(&self) -> Vec<u8> {
        match *self {
            MavMessage::FlexifunctionSet(ref body) => body.mavlink_ser(),
            MavMessage::FlexifunctionReadReq(ref body) => body.mavlink_ser(),
            MavMessage::FlexifunctionBufferFunction(ref body) => body.mavlink_ser(),
            MavMessage::FlexifunctionBufferFunctionAck(ref body) => body.mavlink_ser(),
            MavMessage::FlexifunctionDirectory(ref body) => body.mavlink_ser(),
            MavMessage::FlexifunctionDirectoryAck(ref body) => body.mavlink_ser(),
            MavMessage::FlexifunctionCommand(ref body) => body.mavlink_ser(),
            MavMessage::FlexifunctionCommandAck(ref body) => body.mavlink_ser(),
            MavMessage::SerialUdbExtraF2A(ref body) => body.mavlink_ser(),
            MavMessage::SerialUdbExtraF2B(ref body) => body.mavlink_ser(),
            MavMessage::SerialUdbExtraF4(ref body) => body.mavlink_ser(),
            MavMessage::SerialUdbExtraF5(ref body) => body.mavlink_ser(),
            MavMessage::SerialUdbExtraF6(ref body) => body.mavlink_ser(),
            MavMessage::SerialUdbExtraF7(ref body) => body.mavlink_ser(),
            MavMessage::SerialUdbExtraF8(ref body) => body.mavlink_ser(),
            MavMessage::SerialUdbExtraF13(ref body) => body.mavlink_ser(),
            MavMessage::SerialUdbExtraF14(ref body) => body.mavlink_ser(),
            MavMessage::SerialUdbExtraF15(ref body) => body.mavlink_ser(),
            MavMessage::SerialUdbExtraF16(ref body) => body.mavlink_ser(),
            MavMessage::Altitudes(ref body) => body.mavlink_ser(),
            MavMessage::Airspeeds(ref body) => body.mavlink_ser(),
            MavMessage::SerialUdbExtraF17(ref body) => body.mavlink_ser(),
            MavMessage::SerialUdbExtraF18(ref body) => body.mavlink_ser(),
            MavMessage::SerialUdbExtraF19(ref body) => body.mavlink_ser(),
            MavMessage::SerialUdbExtraF20(ref body) => body.mavlink_ser(),
            MavMessage::SerialUdbExtraF21(ref body) => body.mavlink_ser(),
            MavMessage::SerialUdbExtraF22(ref body) => body.mavlink_ser(),
            MavMessage::Common(ref msg) => msg.mavlink_ser(),
        }
    }
    fn proto_encode(&self) -> Vec<u8> {
        match *self {
            MavMessage::FlexifunctionSet(ref body) => body.encode_to_vec(),
            MavMessage::FlexifunctionReadReq(ref body) => body.encode_to_vec(),
            MavMessage::FlexifunctionBufferFunction(ref body) => body.encode_to_vec(),
            MavMessage::FlexifunctionBufferFunctionAck(ref body) => body.encode_to_vec(),
            MavMessage::FlexifunctionDirectory(ref body) => body.encode_to_vec(),
            MavMessage::FlexifunctionDirectoryAck(ref body) => body.encode_to_vec(),
            MavMessage::FlexifunctionCommand(ref body) => body.encode_to_vec(),
            MavMessage::FlexifunctionCommandAck(ref body) => body.encode_to_vec(),
            MavMessage::SerialUdbExtraF2A(ref body) => body.encode_to_vec(),
            MavMessage::SerialUdbExtraF2B(ref body) => body.encode_to_vec(),
            MavMessage::SerialUdbExtraF4(ref body) => body.encode_to_vec(),
            MavMessage::SerialUdbExtraF5(ref body) => body.encode_to_vec(),
            MavMessage::SerialUdbExtraF6(ref body) => body.encode_to_vec(),
            MavMessage::SerialUdbExtraF7(ref body) => body.encode_to_vec(),
            MavMessage::SerialUdbExtraF8(ref body) => body.encode_to_vec(),
            MavMessage::SerialUdbExtraF13(ref body) => body.encode_to_vec(),
            MavMessage::SerialUdbExtraF14(ref body) => body.encode_to_vec(),
            MavMessage::SerialUdbExtraF15(ref body) => body.encode_to_vec(),
            MavMessage::SerialUdbExtraF16(ref body) => body.encode_to_vec(),
            MavMessage::Altitudes(ref body) => body.encode_to_vec(),
            MavMessage::Airspeeds(ref body) => body.encode_to_vec(),
            MavMessage::SerialUdbExtraF17(ref body) => body.encode_to_vec(),
            MavMessage::SerialUdbExtraF18(ref body) => body.encode_to_vec(),
            MavMessage::SerialUdbExtraF19(ref body) => body.encode_to_vec(),
            MavMessage::SerialUdbExtraF20(ref body) => body.encode_to_vec(),
            MavMessage::SerialUdbExtraF21(ref body) => body.encode_to_vec(),
            MavMessage::SerialUdbExtraF22(ref body) => body.encode_to_vec(),
            MavMessage::Common(ref msg) => msg.proto_encode(),
        }
    }
    fn extra_crc(id: u32) -> u8 {
        match id {
            150 => 181,
            151 => 26,
            152 => 101,
            153 => 109,
            155 => 12,
            156 => 218,
            157 => 133,
            158 => 208,
            170 => 103,
            171 => 245,
            172 => 191,
            173 => 54,
            174 => 54,
            175 => 171,
            176 => 142,
            177 => 249,
            178 => 123,
            179 => 7,
            180 => 222,
            181 => 55,
            182 => 154,
            183 => 175,
            184 => 41,
            185 => 87,
            186 => 144,
            187 => 134,
            188 => 91,
            _ => {
                match crate::mavlink::common::MavMessage::extra_crc(id) {
                    0 => {}
                    any => return any,
                }
                0
            }
        }
    }
}
