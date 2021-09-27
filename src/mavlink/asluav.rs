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
impl crate::proto::asluav::CommandIntStamped {
    pub const ENCODED_LEN: usize = 47usize;
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
            _struct.vehicle_timestamp = buf.get_u64_le();
            _struct.utc_time = buf.get_u32_le();
            _struct.param1 = buf.get_f32_le();
            _struct.param2 = buf.get_f32_le();
            _struct.param3 = buf.get_f32_le();
            _struct.param4 = buf.get_f32_le();
            _struct.x = buf.get_i32_le();
            _struct.y = buf.get_i32_le();
            _struct.z = buf.get_f32_le();
            let tmp = buf.get_u16_le();
            _struct.command = FromPrimitive::from_u16(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavCmd".to_string(),
                value: tmp as u32,
            })?;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            let tmp = buf.get_u8();
            _struct.frame = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavFrame".to_string(),
                value: tmp as u32,
            })?;
            _struct.current = buf.get_u8() as u32;
            _struct.autocontinue = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.vehicle_timestamp as u64);
        _tmp.put_u32_le(self.utc_time as u32);
        _tmp.put_f32_le(self.param1 as f32);
        _tmp.put_f32_le(self.param2 as f32);
        _tmp.put_f32_le(self.param3 as f32);
        _tmp.put_f32_le(self.param4 as f32);
        _tmp.put_i32_le(self.x as i32);
        _tmp.put_i32_le(self.y as i32);
        _tmp.put_f32_le(self.z as f32);
        _tmp.put_u16_le(self.command as u16);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp.put_u8(self.frame as u8);
        _tmp.put_u8(self.current as u8);
        _tmp.put_u8(self.autocontinue as u8);
        _tmp
    }
}
impl crate::proto::asluav::CommandLongStamped {
    pub const ENCODED_LEN: usize = 45usize;
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
            _struct.vehicle_timestamp = buf.get_u64_le();
            _struct.utc_time = buf.get_u32_le();
            _struct.param1 = buf.get_f32_le();
            _struct.param2 = buf.get_f32_le();
            _struct.param3 = buf.get_f32_le();
            _struct.param4 = buf.get_f32_le();
            _struct.param5 = buf.get_f32_le();
            _struct.param6 = buf.get_f32_le();
            _struct.param7 = buf.get_f32_le();
            let tmp = buf.get_u16_le();
            _struct.command = FromPrimitive::from_u16(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavCmd".to_string(),
                value: tmp as u32,
            })?;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            _struct.confirmation = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.vehicle_timestamp as u64);
        _tmp.put_u32_le(self.utc_time as u32);
        _tmp.put_f32_le(self.param1 as f32);
        _tmp.put_f32_le(self.param2 as f32);
        _tmp.put_f32_le(self.param3 as f32);
        _tmp.put_f32_le(self.param4 as f32);
        _tmp.put_f32_le(self.param5 as f32);
        _tmp.put_f32_le(self.param6 as f32);
        _tmp.put_f32_le(self.param7 as f32);
        _tmp.put_u16_le(self.command as u16);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp.put_u8(self.confirmation as u8);
        _tmp
    }
}
impl crate::proto::asluav::SensPower {
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
            _struct.adc121_vspb_volt = buf.get_f32_le();
            _struct.adc121_cspb_amp = buf.get_f32_le();
            _struct.adc121_cs1_amp = buf.get_f32_le();
            _struct.adc121_cs2_amp = buf.get_f32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.adc121_vspb_volt as f32);
        _tmp.put_f32_le(self.adc121_cspb_amp as f32);
        _tmp.put_f32_le(self.adc121_cs1_amp as f32);
        _tmp.put_f32_le(self.adc121_cs2_amp as f32);
        _tmp
    }
}
impl crate::proto::asluav::SensMppt {
    pub const ENCODED_LEN: usize = 41usize;
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
            _struct.mppt_timestamp = buf.get_u64_le();
            _struct.mppt1_volt = buf.get_f32_le();
            _struct.mppt1_amp = buf.get_f32_le();
            _struct.mppt2_volt = buf.get_f32_le();
            _struct.mppt2_amp = buf.get_f32_le();
            _struct.mppt3_volt = buf.get_f32_le();
            _struct.mppt3_amp = buf.get_f32_le();
            _struct.mppt1_pwm = buf.get_u16_le() as u32;
            _struct.mppt2_pwm = buf.get_u16_le() as u32;
            _struct.mppt3_pwm = buf.get_u16_le() as u32;
            _struct.mppt1_status = buf.get_u8() as u32;
            _struct.mppt2_status = buf.get_u8() as u32;
            _struct.mppt3_status = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.mppt_timestamp as u64);
        _tmp.put_f32_le(self.mppt1_volt as f32);
        _tmp.put_f32_le(self.mppt1_amp as f32);
        _tmp.put_f32_le(self.mppt2_volt as f32);
        _tmp.put_f32_le(self.mppt2_amp as f32);
        _tmp.put_f32_le(self.mppt3_volt as f32);
        _tmp.put_f32_le(self.mppt3_amp as f32);
        _tmp.put_u16_le(self.mppt1_pwm as u16);
        _tmp.put_u16_le(self.mppt2_pwm as u16);
        _tmp.put_u16_le(self.mppt3_pwm as u16);
        _tmp.put_u8(self.mppt1_status as u8);
        _tmp.put_u8(self.mppt2_status as u8);
        _tmp.put_u8(self.mppt3_status as u8);
        _tmp
    }
}
impl crate::proto::asluav::AslctrlData {
    pub const ENCODED_LEN: usize = 98usize;
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
            _struct.timestamp = buf.get_u64_le();
            _struct.h = buf.get_f32_le();
            _struct.h_ref = buf.get_f32_le();
            _struct.h_ref_t = buf.get_f32_le();
            _struct.pitch_angle = buf.get_f32_le();
            _struct.pitch_angle_ref = buf.get_f32_le();
            _struct.q = buf.get_f32_le();
            _struct.q_ref = buf.get_f32_le();
            _struct.u_elev = buf.get_f32_le();
            _struct.u_throt = buf.get_f32_le();
            _struct.u_throt2 = buf.get_f32_le();
            _struct.n_z = buf.get_f32_le();
            _struct.airspeed_ref = buf.get_f32_le();
            _struct.yaw_angle = buf.get_f32_le();
            _struct.yaw_angle_ref = buf.get_f32_le();
            _struct.roll_angle = buf.get_f32_le();
            _struct.roll_angle_ref = buf.get_f32_le();
            _struct.p = buf.get_f32_le();
            _struct.p_ref = buf.get_f32_le();
            _struct.r = buf.get_f32_le();
            _struct.r_ref = buf.get_f32_le();
            _struct.u_ail = buf.get_f32_le();
            _struct.u_rud = buf.get_f32_le();
            _struct.aslctrl_mode = buf.get_u8() as u32;
            _struct.spoilers_engaged = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.timestamp as u64);
        _tmp.put_f32_le(self.h as f32);
        _tmp.put_f32_le(self.h_ref as f32);
        _tmp.put_f32_le(self.h_ref_t as f32);
        _tmp.put_f32_le(self.pitch_angle as f32);
        _tmp.put_f32_le(self.pitch_angle_ref as f32);
        _tmp.put_f32_le(self.q as f32);
        _tmp.put_f32_le(self.q_ref as f32);
        _tmp.put_f32_le(self.u_elev as f32);
        _tmp.put_f32_le(self.u_throt as f32);
        _tmp.put_f32_le(self.u_throt2 as f32);
        _tmp.put_f32_le(self.n_z as f32);
        _tmp.put_f32_le(self.airspeed_ref as f32);
        _tmp.put_f32_le(self.yaw_angle as f32);
        _tmp.put_f32_le(self.yaw_angle_ref as f32);
        _tmp.put_f32_le(self.roll_angle as f32);
        _tmp.put_f32_le(self.roll_angle_ref as f32);
        _tmp.put_f32_le(self.p as f32);
        _tmp.put_f32_le(self.p_ref as f32);
        _tmp.put_f32_le(self.r as f32);
        _tmp.put_f32_le(self.r_ref as f32);
        _tmp.put_f32_le(self.u_ail as f32);
        _tmp.put_f32_le(self.u_rud as f32);
        _tmp.put_u8(self.aslctrl_mode as u8);
        _tmp.put_u8(self.spoilers_engaged as u8);
        _tmp
    }
}
impl crate::proto::asluav::AslctrlDebug {
    pub const ENCODED_LEN: usize = 38usize;
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
            _struct.i32_1 = buf.get_u32_le();
            _struct.f_1 = buf.get_f32_le();
            _struct.f_2 = buf.get_f32_le();
            _struct.f_3 = buf.get_f32_le();
            _struct.f_4 = buf.get_f32_le();
            _struct.f_5 = buf.get_f32_le();
            _struct.f_6 = buf.get_f32_le();
            _struct.f_7 = buf.get_f32_le();
            _struct.f_8 = buf.get_f32_le();
            _struct.i8_1 = buf.get_u8() as u32;
            _struct.i8_2 = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.i32_1 as u32);
        _tmp.put_f32_le(self.f_1 as f32);
        _tmp.put_f32_le(self.f_2 as f32);
        _tmp.put_f32_le(self.f_3 as f32);
        _tmp.put_f32_le(self.f_4 as f32);
        _tmp.put_f32_le(self.f_5 as f32);
        _tmp.put_f32_le(self.f_6 as f32);
        _tmp.put_f32_le(self.f_7 as f32);
        _tmp.put_f32_le(self.f_8 as f32);
        _tmp.put_u8(self.i8_1 as u8);
        _tmp.put_u8(self.i8_2 as u8);
        _tmp
    }
}
impl crate::proto::asluav::AsluavStatus {
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
            _struct.motor_rpm = buf.get_f32_le();
            _struct.led_status = buf.get_u8() as u32;
            _struct.satcom_status = buf.get_u8() as u32;
            for _ in 0..8usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.servo_status.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.motor_rpm as f32);
        _tmp.put_u8(self.led_status as u8);
        _tmp.put_u8(self.satcom_status as u8);
        for val in &self.servo_status {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
impl crate::proto::asluav::EkfExt {
    pub const ENCODED_LEN: usize = 32usize;
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
            _struct.timestamp = buf.get_u64_le();
            _struct.windspeed = buf.get_f32_le();
            _struct.wind_dir = buf.get_f32_le();
            _struct.wind_z = buf.get_f32_le();
            _struct.airspeed = buf.get_f32_le();
            _struct.beta = buf.get_f32_le();
            _struct.alpha = buf.get_f32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.timestamp as u64);
        _tmp.put_f32_le(self.windspeed as f32);
        _tmp.put_f32_le(self.wind_dir as f32);
        _tmp.put_f32_le(self.wind_z as f32);
        _tmp.put_f32_le(self.airspeed as f32);
        _tmp.put_f32_le(self.beta as f32);
        _tmp.put_f32_le(self.alpha as f32);
        _tmp
    }
}
impl crate::proto::asluav::AslObctrl {
    pub const ENCODED_LEN: usize = 33usize;
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
            _struct.timestamp = buf.get_u64_le();
            _struct.u_elev = buf.get_f32_le();
            _struct.u_throt = buf.get_f32_le();
            _struct.u_throt2 = buf.get_f32_le();
            _struct.u_ail_l = buf.get_f32_le();
            _struct.u_ail_r = buf.get_f32_le();
            _struct.u_rud = buf.get_f32_le();
            _struct.obctrl_status = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.timestamp as u64);
        _tmp.put_f32_le(self.u_elev as f32);
        _tmp.put_f32_le(self.u_throt as f32);
        _tmp.put_f32_le(self.u_throt2 as f32);
        _tmp.put_f32_le(self.u_ail_l as f32);
        _tmp.put_f32_le(self.u_ail_r as f32);
        _tmp.put_f32_le(self.u_rud as f32);
        _tmp.put_u8(self.obctrl_status as u8);
        _tmp
    }
}
impl crate::proto::asluav::SensAtmos {
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
            _struct.timestamp = buf.get_u64_le();
            _struct.temp_ambient = buf.get_f32_le();
            _struct.humidity = buf.get_f32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.timestamp as u64);
        _tmp.put_f32_le(self.temp_ambient as f32);
        _tmp.put_f32_le(self.humidity as f32);
        _tmp
    }
}
impl crate::proto::asluav::SensBatmon {
    pub const ENCODED_LEN: usize = 41usize;
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
            _struct.batmon_timestamp = buf.get_u64_le();
            _struct.temperature = buf.get_f32_le();
            _struct.safetystatus = buf.get_u32_le();
            _struct.operationstatus = buf.get_u32_le();
            _struct.voltage = buf.get_u16_le() as u32;
            _struct.current = buf.get_i16_le() as i32;
            _struct.batterystatus = buf.get_u16_le() as u32;
            _struct.serialnumber = buf.get_u16_le() as u32;
            _struct.cellvoltage1 = buf.get_u16_le() as u32;
            _struct.cellvoltage2 = buf.get_u16_le() as u32;
            _struct.cellvoltage3 = buf.get_u16_le() as u32;
            _struct.cellvoltage4 = buf.get_u16_le() as u32;
            _struct.cellvoltage5 = buf.get_u16_le() as u32;
            _struct.cellvoltage6 = buf.get_u16_le() as u32;
            _struct.so_c = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.batmon_timestamp as u64);
        _tmp.put_f32_le(self.temperature as f32);
        _tmp.put_u32_le(self.safetystatus as u32);
        _tmp.put_u32_le(self.operationstatus as u32);
        _tmp.put_u16_le(self.voltage as u16);
        _tmp.put_i16_le(self.current as i16);
        _tmp.put_u16_le(self.batterystatus as u16);
        _tmp.put_u16_le(self.serialnumber as u16);
        _tmp.put_u16_le(self.cellvoltage1 as u16);
        _tmp.put_u16_le(self.cellvoltage2 as u16);
        _tmp.put_u16_le(self.cellvoltage3 as u16);
        _tmp.put_u16_le(self.cellvoltage4 as u16);
        _tmp.put_u16_le(self.cellvoltage5 as u16);
        _tmp.put_u16_le(self.cellvoltage6 as u16);
        _tmp.put_u8(self.so_c as u8);
        _tmp
    }
}
impl crate::proto::asluav::FwSoaringData {
    pub const ENCODED_LEN: usize = 102usize;
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
            _struct.timestamp = buf.get_u64_le();
            _struct.timestamp_mode_changed = buf.get_u64_le();
            _struct.x_w = buf.get_f32_le();
            _struct.x_r = buf.get_f32_le();
            _struct.x_lat = buf.get_f32_le();
            _struct.x_lon = buf.get_f32_le();
            _struct.var_w = buf.get_f32_le();
            _struct.var_r = buf.get_f32_le();
            _struct.var_lat = buf.get_f32_le();
            _struct.var_lon = buf.get_f32_le();
            _struct.loiter_radius = buf.get_f32_le();
            _struct.loiter_direction = buf.get_f32_le();
            _struct.dist_to_soar_point = buf.get_f32_le();
            _struct.v_sink_exp = buf.get_f32_le();
            _struct.z1_local_updraft_speed = buf.get_f32_le();
            _struct.z2_delta_roll = buf.get_f32_le();
            _struct.z1_exp = buf.get_f32_le();
            _struct.z2_exp = buf.get_f32_le();
            _struct.thermal_gs_north = buf.get_f32_le();
            _struct.thermal_gs_east = buf.get_f32_le();
            _struct.tse_dot = buf.get_f32_le();
            _struct.debug_var1 = buf.get_f32_le();
            _struct.debug_var2 = buf.get_f32_le();
            _struct.control_mode = buf.get_u8() as u32;
            _struct.valid = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.timestamp as u64);
        _tmp.put_u64_le(self.timestamp_mode_changed as u64);
        _tmp.put_f32_le(self.x_w as f32);
        _tmp.put_f32_le(self.x_r as f32);
        _tmp.put_f32_le(self.x_lat as f32);
        _tmp.put_f32_le(self.x_lon as f32);
        _tmp.put_f32_le(self.var_w as f32);
        _tmp.put_f32_le(self.var_r as f32);
        _tmp.put_f32_le(self.var_lat as f32);
        _tmp.put_f32_le(self.var_lon as f32);
        _tmp.put_f32_le(self.loiter_radius as f32);
        _tmp.put_f32_le(self.loiter_direction as f32);
        _tmp.put_f32_le(self.dist_to_soar_point as f32);
        _tmp.put_f32_le(self.v_sink_exp as f32);
        _tmp.put_f32_le(self.z1_local_updraft_speed as f32);
        _tmp.put_f32_le(self.z2_delta_roll as f32);
        _tmp.put_f32_le(self.z1_exp as f32);
        _tmp.put_f32_le(self.z2_exp as f32);
        _tmp.put_f32_le(self.thermal_gs_north as f32);
        _tmp.put_f32_le(self.thermal_gs_east as f32);
        _tmp.put_f32_le(self.tse_dot as f32);
        _tmp.put_f32_le(self.debug_var1 as f32);
        _tmp.put_f32_le(self.debug_var2 as f32);
        _tmp.put_u8(self.control_mode as u8);
        _tmp.put_u8(self.valid as u8);
        _tmp
    }
}
impl crate::proto::asluav::SensorpodStatus {
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
            _struct.timestamp = buf.get_u64_le();
            _struct.free_space = buf.get_u16_le() as u32;
            _struct.visensor_rate_1 = buf.get_u8() as u32;
            _struct.visensor_rate_2 = buf.get_u8() as u32;
            _struct.visensor_rate_3 = buf.get_u8() as u32;
            _struct.visensor_rate_4 = buf.get_u8() as u32;
            _struct.recording_nodes_count = buf.get_u8() as u32;
            _struct.cpu_temp = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.timestamp as u64);
        _tmp.put_u16_le(self.free_space as u16);
        _tmp.put_u8(self.visensor_rate_1 as u8);
        _tmp.put_u8(self.visensor_rate_2 as u8);
        _tmp.put_u8(self.visensor_rate_3 as u8);
        _tmp.put_u8(self.visensor_rate_4 as u8);
        _tmp.put_u8(self.recording_nodes_count as u8);
        _tmp.put_u8(self.cpu_temp as u8);
        _tmp
    }
}
impl crate::proto::asluav::SensPowerBoard {
    pub const ENCODED_LEN: usize = 46usize;
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
            _struct.timestamp = buf.get_u64_le();
            _struct.pwr_brd_system_volt = buf.get_f32_le();
            _struct.pwr_brd_servo_volt = buf.get_f32_le();
            _struct.pwr_brd_digital_volt = buf.get_f32_le();
            _struct.pwr_brd_mot_l_amp = buf.get_f32_le();
            _struct.pwr_brd_mot_r_amp = buf.get_f32_le();
            _struct.pwr_brd_analog_amp = buf.get_f32_le();
            _struct.pwr_brd_digital_amp = buf.get_f32_le();
            _struct.pwr_brd_ext_amp = buf.get_f32_le();
            _struct.pwr_brd_aux_amp = buf.get_f32_le();
            _struct.pwr_brd_status = buf.get_u8() as u32;
            _struct.pwr_brd_led_status = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.timestamp as u64);
        _tmp.put_f32_le(self.pwr_brd_system_volt as f32);
        _tmp.put_f32_le(self.pwr_brd_servo_volt as f32);
        _tmp.put_f32_le(self.pwr_brd_digital_volt as f32);
        _tmp.put_f32_le(self.pwr_brd_mot_l_amp as f32);
        _tmp.put_f32_le(self.pwr_brd_mot_r_amp as f32);
        _tmp.put_f32_le(self.pwr_brd_analog_amp as f32);
        _tmp.put_f32_le(self.pwr_brd_digital_amp as f32);
        _tmp.put_f32_le(self.pwr_brd_ext_amp as f32);
        _tmp.put_f32_le(self.pwr_brd_aux_amp as f32);
        _tmp.put_u8(self.pwr_brd_status as u8);
        _tmp.put_u8(self.pwr_brd_led_status as u8);
        _tmp
    }
}
impl crate::proto::asluav::GsmLinkStatus {
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
            _struct.timestamp = buf.get_u64_le();
            let tmp = buf.get_u8();
            _struct.gsm_modem_type =
                FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "GsmModemType".to_string(),
                    value: tmp as u32,
                })?;
            let tmp = buf.get_u8();
            _struct.gsm_link_type =
                FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "GsmLinkType".to_string(),
                    value: tmp as u32,
                })?;
            _struct.rssi = buf.get_u8() as u32;
            _struct.rsrp_rscp = buf.get_u8() as u32;
            _struct.sinr_ecio = buf.get_u8() as u32;
            _struct.rsrq = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.timestamp as u64);
        _tmp.put_u8(self.gsm_modem_type as u8);
        _tmp.put_u8(self.gsm_link_type as u8);
        _tmp.put_u8(self.rssi as u8);
        _tmp.put_u8(self.rsrp_rscp as u8);
        _tmp.put_u8(self.sinr_ecio as u8);
        _tmp.put_u8(self.rsrq as u8);
        _tmp
    }
}
#[derive(Clone, PartialEq, Debug)]
pub enum MavMessage {
    CommandIntStamped(crate::proto::asluav::CommandIntStamped),
    CommandLongStamped(crate::proto::asluav::CommandLongStamped),
    SensPower(crate::proto::asluav::SensPower),
    SensMppt(crate::proto::asluav::SensMppt),
    AslctrlData(crate::proto::asluav::AslctrlData),
    AslctrlDebug(crate::proto::asluav::AslctrlDebug),
    AsluavStatus(crate::proto::asluav::AsluavStatus),
    EkfExt(crate::proto::asluav::EkfExt),
    AslObctrl(crate::proto::asluav::AslObctrl),
    SensAtmos(crate::proto::asluav::SensAtmos),
    SensBatmon(crate::proto::asluav::SensBatmon),
    FwSoaringData(crate::proto::asluav::FwSoaringData),
    SensorpodStatus(crate::proto::asluav::SensorpodStatus),
    SensPowerBoard(crate::proto::asluav::SensPowerBoard),
    GsmLinkStatus(crate::proto::asluav::GsmLinkStatus),
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
            78 => crate::proto::asluav::CommandIntStamped::mavlink_deser(version, payload)
                .map(MavMessage::CommandIntStamped),
            79 => crate::proto::asluav::CommandLongStamped::mavlink_deser(version, payload)
                .map(MavMessage::CommandLongStamped),
            201 => crate::proto::asluav::SensPower::mavlink_deser(version, payload)
                .map(MavMessage::SensPower),
            202 => crate::proto::asluav::SensMppt::mavlink_deser(version, payload)
                .map(MavMessage::SensMppt),
            203 => crate::proto::asluav::AslctrlData::mavlink_deser(version, payload)
                .map(MavMessage::AslctrlData),
            204 => crate::proto::asluav::AslctrlDebug::mavlink_deser(version, payload)
                .map(MavMessage::AslctrlDebug),
            205 => crate::proto::asluav::AsluavStatus::mavlink_deser(version, payload)
                .map(MavMessage::AsluavStatus),
            206 => crate::proto::asluav::EkfExt::mavlink_deser(version, payload)
                .map(MavMessage::EkfExt),
            207 => crate::proto::asluav::AslObctrl::mavlink_deser(version, payload)
                .map(MavMessage::AslObctrl),
            208 => crate::proto::asluav::SensAtmos::mavlink_deser(version, payload)
                .map(MavMessage::SensAtmos),
            209 => crate::proto::asluav::SensBatmon::mavlink_deser(version, payload)
                .map(MavMessage::SensBatmon),
            210 => crate::proto::asluav::FwSoaringData::mavlink_deser(version, payload)
                .map(MavMessage::FwSoaringData),
            211 => crate::proto::asluav::SensorpodStatus::mavlink_deser(version, payload)
                .map(MavMessage::SensorpodStatus),
            212 => crate::proto::asluav::SensPowerBoard::mavlink_deser(version, payload)
                .map(MavMessage::SensPowerBoard),
            213 => crate::proto::asluav::GsmLinkStatus::mavlink_deser(version, payload)
                .map(MavMessage::GsmLinkStatus),
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
            MavMessage::CommandIntStamped(..) => "CommandIntStamped",
            MavMessage::CommandLongStamped(..) => "CommandLongStamped",
            MavMessage::SensPower(..) => "SensPower",
            MavMessage::SensMppt(..) => "SensMppt",
            MavMessage::AslctrlData(..) => "AslctrlData",
            MavMessage::AslctrlDebug(..) => "AslctrlDebug",
            MavMessage::AsluavStatus(..) => "AsluavStatus",
            MavMessage::EkfExt(..) => "EkfExt",
            MavMessage::AslObctrl(..) => "AslObctrl",
            MavMessage::SensAtmos(..) => "SensAtmos",
            MavMessage::SensBatmon(..) => "SensBatmon",
            MavMessage::FwSoaringData(..) => "FwSoaringData",
            MavMessage::SensorpodStatus(..) => "SensorpodStatus",
            MavMessage::SensPowerBoard(..) => "SensPowerBoard",
            MavMessage::GsmLinkStatus(..) => "GsmLinkStatus",
            MavMessage::Common(msg) => msg.message_name(),
        }
    }
    fn message_id(&self) -> u32 {
        match self {
            MavMessage::CommandIntStamped(..) => 78,
            MavMessage::CommandLongStamped(..) => 79,
            MavMessage::SensPower(..) => 201,
            MavMessage::SensMppt(..) => 202,
            MavMessage::AslctrlData(..) => 203,
            MavMessage::AslctrlDebug(..) => 204,
            MavMessage::AsluavStatus(..) => 205,
            MavMessage::EkfExt(..) => 206,
            MavMessage::AslObctrl(..) => 207,
            MavMessage::SensAtmos(..) => 208,
            MavMessage::SensBatmon(..) => 209,
            MavMessage::FwSoaringData(..) => 210,
            MavMessage::SensorpodStatus(..) => 211,
            MavMessage::SensPowerBoard(..) => 212,
            MavMessage::GsmLinkStatus(..) => 213,
            MavMessage::Common(msg) => msg.message_id(),
        }
    }
    fn message_id_from_name(name: &str) -> Result<u32, &'static str> {
        match name {
            "CommandIntStamped" => Ok(78),
            "CommandLongStamped" => Ok(79),
            "SensPower" => Ok(201),
            "SensMppt" => Ok(202),
            "AslctrlData" => Ok(203),
            "AslctrlDebug" => Ok(204),
            "AsluavStatus" => Ok(205),
            "EkfExt" => Ok(206),
            "AslObctrl" => Ok(207),
            "SensAtmos" => Ok(208),
            "SensBatmon" => Ok(209),
            "FwSoaringData" => Ok(210),
            "SensorpodStatus" => Ok(211),
            "SensPowerBoard" => Ok(212),
            "GsmLinkStatus" => Ok(213),
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
            78 => Ok(MavMessage::CommandIntStamped(
                crate::proto::asluav::CommandIntStamped::default(),
            )),
            79 => Ok(MavMessage::CommandLongStamped(
                crate::proto::asluav::CommandLongStamped::default(),
            )),
            201 => Ok(MavMessage::SensPower(
                crate::proto::asluav::SensPower::default(),
            )),
            202 => Ok(MavMessage::SensMppt(
                crate::proto::asluav::SensMppt::default(),
            )),
            203 => Ok(MavMessage::AslctrlData(
                crate::proto::asluav::AslctrlData::default(),
            )),
            204 => Ok(MavMessage::AslctrlDebug(
                crate::proto::asluav::AslctrlDebug::default(),
            )),
            205 => Ok(MavMessage::AsluavStatus(
                crate::proto::asluav::AsluavStatus::default(),
            )),
            206 => Ok(MavMessage::EkfExt(crate::proto::asluav::EkfExt::default())),
            207 => Ok(MavMessage::AslObctrl(
                crate::proto::asluav::AslObctrl::default(),
            )),
            208 => Ok(MavMessage::SensAtmos(
                crate::proto::asluav::SensAtmos::default(),
            )),
            209 => Ok(MavMessage::SensBatmon(
                crate::proto::asluav::SensBatmon::default(),
            )),
            210 => Ok(MavMessage::FwSoaringData(
                crate::proto::asluav::FwSoaringData::default(),
            )),
            211 => Ok(MavMessage::SensorpodStatus(
                crate::proto::asluav::SensorpodStatus::default(),
            )),
            212 => Ok(MavMessage::SensPowerBoard(
                crate::proto::asluav::SensPowerBoard::default(),
            )),
            213 => Ok(MavMessage::GsmLinkStatus(
                crate::proto::asluav::GsmLinkStatus::default(),
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
            MavMessage::CommandIntStamped(ref body) => body.mavlink_ser(),
            MavMessage::CommandLongStamped(ref body) => body.mavlink_ser(),
            MavMessage::SensPower(ref body) => body.mavlink_ser(),
            MavMessage::SensMppt(ref body) => body.mavlink_ser(),
            MavMessage::AslctrlData(ref body) => body.mavlink_ser(),
            MavMessage::AslctrlDebug(ref body) => body.mavlink_ser(),
            MavMessage::AsluavStatus(ref body) => body.mavlink_ser(),
            MavMessage::EkfExt(ref body) => body.mavlink_ser(),
            MavMessage::AslObctrl(ref body) => body.mavlink_ser(),
            MavMessage::SensAtmos(ref body) => body.mavlink_ser(),
            MavMessage::SensBatmon(ref body) => body.mavlink_ser(),
            MavMessage::FwSoaringData(ref body) => body.mavlink_ser(),
            MavMessage::SensorpodStatus(ref body) => body.mavlink_ser(),
            MavMessage::SensPowerBoard(ref body) => body.mavlink_ser(),
            MavMessage::GsmLinkStatus(ref body) => body.mavlink_ser(),
            MavMessage::Common(ref msg) => msg.mavlink_ser(),
        }
    }
    fn proto_encode(&self) -> Vec<u8> {
        match *self {
            MavMessage::CommandIntStamped(ref body) => body.encode_to_vec(),
            MavMessage::CommandLongStamped(ref body) => body.encode_to_vec(),
            MavMessage::SensPower(ref body) => body.encode_to_vec(),
            MavMessage::SensMppt(ref body) => body.encode_to_vec(),
            MavMessage::AslctrlData(ref body) => body.encode_to_vec(),
            MavMessage::AslctrlDebug(ref body) => body.encode_to_vec(),
            MavMessage::AsluavStatus(ref body) => body.encode_to_vec(),
            MavMessage::EkfExt(ref body) => body.encode_to_vec(),
            MavMessage::AslObctrl(ref body) => body.encode_to_vec(),
            MavMessage::SensAtmos(ref body) => body.encode_to_vec(),
            MavMessage::SensBatmon(ref body) => body.encode_to_vec(),
            MavMessage::FwSoaringData(ref body) => body.encode_to_vec(),
            MavMessage::SensorpodStatus(ref body) => body.encode_to_vec(),
            MavMessage::SensPowerBoard(ref body) => body.encode_to_vec(),
            MavMessage::GsmLinkStatus(ref body) => body.encode_to_vec(),
            MavMessage::Common(ref msg) => msg.proto_encode(),
        }
    }
    fn extra_crc(id: u32) -> u8 {
        match id {
            78 => 119,
            79 => 102,
            201 => 218,
            202 => 231,
            203 => 172,
            204 => 251,
            205 => 97,
            206 => 64,
            207 => 234,
            208 => 144,
            209 => 155,
            210 => 20,
            211 => 54,
            212 => 222,
            213 => 200,
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
