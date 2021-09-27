// This file was automatically generated, do not edit
#[allow(unused_imports)]
use crate::{mavlink::common::*, mavlink::icarous::*, mavlink::uavionix::*};
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
impl crate::proto::ardupilotmega::SensorOffsets {
    pub const ENCODED_LEN: usize = 42usize;
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
            _struct.mag_declination = buf.get_f32_le();
            _struct.raw_press = buf.get_i32_le();
            _struct.raw_temp = buf.get_i32_le();
            _struct.gyro_cal_x = buf.get_f32_le();
            _struct.gyro_cal_y = buf.get_f32_le();
            _struct.gyro_cal_z = buf.get_f32_le();
            _struct.accel_cal_x = buf.get_f32_le();
            _struct.accel_cal_y = buf.get_f32_le();
            _struct.accel_cal_z = buf.get_f32_le();
            _struct.mag_ofs_x = buf.get_i16_le() as i32;
            _struct.mag_ofs_y = buf.get_i16_le() as i32;
            _struct.mag_ofs_z = buf.get_i16_le() as i32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.mag_declination as f32);
        _tmp.put_i32_le(self.raw_press as i32);
        _tmp.put_i32_le(self.raw_temp as i32);
        _tmp.put_f32_le(self.gyro_cal_x as f32);
        _tmp.put_f32_le(self.gyro_cal_y as f32);
        _tmp.put_f32_le(self.gyro_cal_z as f32);
        _tmp.put_f32_le(self.accel_cal_x as f32);
        _tmp.put_f32_le(self.accel_cal_y as f32);
        _tmp.put_f32_le(self.accel_cal_z as f32);
        _tmp.put_i16_le(self.mag_ofs_x as i16);
        _tmp.put_i16_le(self.mag_ofs_y as i16);
        _tmp.put_i16_le(self.mag_ofs_z as i16);
        _tmp
    }
}
impl crate::proto::ardupilotmega::SetMagOffsets {
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
            _struct.mag_ofs_x = buf.get_i16_le() as i32;
            _struct.mag_ofs_y = buf.get_i16_le() as i32;
            _struct.mag_ofs_z = buf.get_i16_le() as i32;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i16_le(self.mag_ofs_x as i16);
        _tmp.put_i16_le(self.mag_ofs_y as i16);
        _tmp.put_i16_le(self.mag_ofs_z as i16);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp
    }
}
impl crate::proto::ardupilotmega::Meminfo {
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
            _struct.brkval = buf.get_u16_le() as u32;
            _struct.freemem = buf.get_u16_le() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.brkval as u16);
        _tmp.put_u16_le(self.freemem as u16);
        _tmp
    }
}
impl crate::proto::ardupilotmega::ApAdc {
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
            _struct.adc1 = buf.get_u16_le() as u32;
            _struct.adc2 = buf.get_u16_le() as u32;
            _struct.adc3 = buf.get_u16_le() as u32;
            _struct.adc4 = buf.get_u16_le() as u32;
            _struct.adc5 = buf.get_u16_le() as u32;
            _struct.adc6 = buf.get_u16_le() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.adc1 as u16);
        _tmp.put_u16_le(self.adc2 as u16);
        _tmp.put_u16_le(self.adc3 as u16);
        _tmp.put_u16_le(self.adc4 as u16);
        _tmp.put_u16_le(self.adc5 as u16);
        _tmp.put_u16_le(self.adc6 as u16);
        _tmp
    }
}
impl crate::proto::ardupilotmega::DigicamConfigure {
    pub const ENCODED_LEN: usize = 15usize;
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
            _struct.extra_value = buf.get_f32_le();
            _struct.shutter_speed = buf.get_u16_le() as u32;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            _struct.mode = buf.get_u8() as u32;
            _struct.aperture = buf.get_u8() as u32;
            _struct.iso = buf.get_u8() as u32;
            _struct.exposure_type = buf.get_u8() as u32;
            _struct.command_id = buf.get_u8() as u32;
            _struct.engine_cut_off = buf.get_u8() as u32;
            _struct.extra_param = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.extra_value as f32);
        _tmp.put_u16_le(self.shutter_speed as u16);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp.put_u8(self.mode as u8);
        _tmp.put_u8(self.aperture as u8);
        _tmp.put_u8(self.iso as u8);
        _tmp.put_u8(self.exposure_type as u8);
        _tmp.put_u8(self.command_id as u8);
        _tmp.put_u8(self.engine_cut_off as u8);
        _tmp.put_u8(self.extra_param as u8);
        _tmp
    }
}
impl crate::proto::ardupilotmega::DigicamControl {
    pub const ENCODED_LEN: usize = 13usize;
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
            _struct.extra_value = buf.get_f32_le();
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            _struct.session = buf.get_u8() as u32;
            _struct.zoom_pos = buf.get_u8() as u32;
            _struct.zoom_step = buf.get_i8() as i32;
            _struct.focus_lock = buf.get_u8() as u32;
            _struct.shot = buf.get_u8() as u32;
            _struct.command_id = buf.get_u8() as u32;
            _struct.extra_param = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.extra_value as f32);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp.put_u8(self.session as u8);
        _tmp.put_u8(self.zoom_pos as u8);
        _tmp.put_i8(self.zoom_step as i8);
        _tmp.put_u8(self.focus_lock as u8);
        _tmp.put_u8(self.shot as u8);
        _tmp.put_u8(self.command_id as u8);
        _tmp.put_u8(self.extra_param as u8);
        _tmp
    }
}
impl crate::proto::ardupilotmega::MountConfigure {
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
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            let tmp = buf.get_u8();
            _struct.mount_mode = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavMountMode".to_string(),
                value: tmp as u32,
            })?;
            _struct.stab_roll = buf.get_u8() as u32;
            _struct.stab_pitch = buf.get_u8() as u32;
            _struct.stab_yaw = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp.put_u8(self.mount_mode as u8);
        _tmp.put_u8(self.stab_roll as u8);
        _tmp.put_u8(self.stab_pitch as u8);
        _tmp.put_u8(self.stab_yaw as u8);
        _tmp
    }
}
impl crate::proto::ardupilotmega::MountControl {
    pub const ENCODED_LEN: usize = 15usize;
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
            _struct.input_a = buf.get_i32_le();
            _struct.input_b = buf.get_i32_le();
            _struct.input_c = buf.get_i32_le();
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            _struct.save_position = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i32_le(self.input_a as i32);
        _tmp.put_i32_le(self.input_b as i32);
        _tmp.put_i32_le(self.input_c as i32);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp.put_u8(self.save_position as u8);
        _tmp
    }
}
impl crate::proto::ardupilotmega::MountStatus {
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
            _struct.pointing_a = buf.get_i32_le();
            _struct.pointing_b = buf.get_i32_le();
            _struct.pointing_c = buf.get_i32_le();
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i32_le(self.pointing_a as i32);
        _tmp.put_i32_le(self.pointing_b as i32);
        _tmp.put_i32_le(self.pointing_c as i32);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp
    }
}
impl crate::proto::ardupilotmega::FencePoint {
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
            _struct.lat = buf.get_f32_le();
            _struct.lng = buf.get_f32_le();
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            _struct.idx = buf.get_u8() as u32;
            _struct.count = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.lat as f32);
        _tmp.put_f32_le(self.lng as f32);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp.put_u8(self.idx as u8);
        _tmp.put_u8(self.count as u8);
        _tmp
    }
}
impl crate::proto::ardupilotmega::FenceFetchPoint {
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
            _struct.idx = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp.put_u8(self.idx as u8);
        _tmp
    }
}
impl crate::proto::ardupilotmega::Ahrs {
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
            _struct.omega_ix = buf.get_f32_le();
            _struct.omega_iy = buf.get_f32_le();
            _struct.omega_iz = buf.get_f32_le();
            _struct.accel_weight = buf.get_f32_le();
            _struct.renorm_val = buf.get_f32_le();
            _struct.error_rp = buf.get_f32_le();
            _struct.error_yaw = buf.get_f32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.omega_ix as f32);
        _tmp.put_f32_le(self.omega_iy as f32);
        _tmp.put_f32_le(self.omega_iz as f32);
        _tmp.put_f32_le(self.accel_weight as f32);
        _tmp.put_f32_le(self.renorm_val as f32);
        _tmp.put_f32_le(self.error_rp as f32);
        _tmp.put_f32_le(self.error_yaw as f32);
        _tmp
    }
}
impl crate::proto::ardupilotmega::Simstate {
    pub const ENCODED_LEN: usize = 44usize;
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
            _struct.roll = buf.get_f32_le();
            _struct.pitch = buf.get_f32_le();
            _struct.yaw = buf.get_f32_le();
            _struct.xacc = buf.get_f32_le();
            _struct.yacc = buf.get_f32_le();
            _struct.zacc = buf.get_f32_le();
            _struct.xgyro = buf.get_f32_le();
            _struct.ygyro = buf.get_f32_le();
            _struct.zgyro = buf.get_f32_le();
            _struct.lat = buf.get_i32_le();
            _struct.lng = buf.get_i32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.roll as f32);
        _tmp.put_f32_le(self.pitch as f32);
        _tmp.put_f32_le(self.yaw as f32);
        _tmp.put_f32_le(self.xacc as f32);
        _tmp.put_f32_le(self.yacc as f32);
        _tmp.put_f32_le(self.zacc as f32);
        _tmp.put_f32_le(self.xgyro as f32);
        _tmp.put_f32_le(self.ygyro as f32);
        _tmp.put_f32_le(self.zgyro as f32);
        _tmp.put_i32_le(self.lat as i32);
        _tmp.put_i32_le(self.lng as i32);
        _tmp
    }
}
impl crate::proto::ardupilotmega::Hwstatus {
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
            _struct.vcc = buf.get_u16_le() as u32;
            _struct.i2_cerr = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.vcc as u16);
        _tmp.put_u8(self.i2_cerr as u8);
        _tmp
    }
}
impl crate::proto::ardupilotmega::Radio {
    pub const ENCODED_LEN: usize = 9usize;
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
            _struct.rxerrors = buf.get_u16_le() as u32;
            _struct.fixed = buf.get_u16_le() as u32;
            _struct.rssi = buf.get_u8() as u32;
            _struct.remrssi = buf.get_u8() as u32;
            _struct.txbuf = buf.get_u8() as u32;
            _struct.noise = buf.get_u8() as u32;
            _struct.remnoise = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.rxerrors as u16);
        _tmp.put_u16_le(self.fixed as u16);
        _tmp.put_u8(self.rssi as u8);
        _tmp.put_u8(self.remrssi as u8);
        _tmp.put_u8(self.txbuf as u8);
        _tmp.put_u8(self.noise as u8);
        _tmp.put_u8(self.remnoise as u8);
        _tmp
    }
}
impl crate::proto::ardupilotmega::LimitsStatus {
    pub const ENCODED_LEN: usize = 22usize;
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
            _struct.last_trigger = buf.get_u32_le();
            _struct.last_action = buf.get_u32_le();
            _struct.last_recovery = buf.get_u32_le();
            _struct.last_clear = buf.get_u32_le();
            _struct.breach_count = buf.get_u16_le() as u32;
            let tmp = buf.get_u8();
            _struct.limits_state = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "LimitsState".to_string(),
                value: tmp as u32,
            })?;
            let tmp = buf.get_u8();
            _struct.mods_enabled = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "LimitModule".to_string(),
                value: tmp as u32,
            })?;
            let tmp = buf.get_u8();
            _struct.mods_required =
                FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "LimitModule".to_string(),
                    value: tmp as u32,
                })?;
            let tmp = buf.get_u8();
            _struct.mods_triggered =
                FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "LimitModule".to_string(),
                    value: tmp as u32,
                })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.last_trigger as u32);
        _tmp.put_u32_le(self.last_action as u32);
        _tmp.put_u32_le(self.last_recovery as u32);
        _tmp.put_u32_le(self.last_clear as u32);
        _tmp.put_u16_le(self.breach_count as u16);
        _tmp.put_u8(self.limits_state as u8);
        _tmp.put_u8(self.mods_enabled as u8);
        _tmp.put_u8(self.mods_required as u8);
        _tmp.put_u8(self.mods_triggered as u8);
        _tmp
    }
}
impl crate::proto::ardupilotmega::Wind {
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
            _struct.direction = buf.get_f32_le();
            _struct.speed = buf.get_f32_le();
            _struct.speed_z = buf.get_f32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.direction as f32);
        _tmp.put_f32_le(self.speed as f32);
        _tmp.put_f32_le(self.speed_z as f32);
        _tmp
    }
}
impl crate::proto::ardupilotmega::Data16 {
    pub const ENCODED_LEN: usize = 18usize;
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
            _struct.r#type = buf.get_u8() as u32;
            _struct.len = buf.get_u8() as u32;
            for _ in 0..16usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.data.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.r#type as u8);
        _tmp.put_u8(self.len as u8);
        for val in &self.data {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
impl crate::proto::ardupilotmega::Data32 {
    pub const ENCODED_LEN: usize = 34usize;
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
            _struct.r#type = buf.get_u8() as u32;
            _struct.len = buf.get_u8() as u32;
            for _ in 0..32usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.data.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.r#type as u8);
        _tmp.put_u8(self.len as u8);
        for val in &self.data {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
impl crate::proto::ardupilotmega::Data64 {
    pub const ENCODED_LEN: usize = 66usize;
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
            _struct.r#type = buf.get_u8() as u32;
            _struct.len = buf.get_u8() as u32;
            for _ in 0..64usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.data.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.r#type as u8);
        _tmp.put_u8(self.len as u8);
        for val in &self.data {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
impl crate::proto::ardupilotmega::Data96 {
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
            _struct.r#type = buf.get_u8() as u32;
            _struct.len = buf.get_u8() as u32;
            for _ in 0..96usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.data.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.r#type as u8);
        _tmp.put_u8(self.len as u8);
        for val in &self.data {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
impl crate::proto::ardupilotmega::Rangefinder {
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
            _struct.distance = buf.get_f32_le();
            _struct.voltage = buf.get_f32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.distance as f32);
        _tmp.put_f32_le(self.voltage as f32);
        _tmp
    }
}
impl crate::proto::ardupilotmega::AirspeedAutocal {
    pub const ENCODED_LEN: usize = 48usize;
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
            _struct.vx = buf.get_f32_le();
            _struct.vy = buf.get_f32_le();
            _struct.vz = buf.get_f32_le();
            _struct.diff_pressure = buf.get_f32_le();
            _struct.eas2tas = buf.get_f32_le();
            _struct.ratio = buf.get_f32_le();
            _struct.state_x = buf.get_f32_le();
            _struct.state_y = buf.get_f32_le();
            _struct.state_z = buf.get_f32_le();
            _struct.pax = buf.get_f32_le();
            _struct.pby = buf.get_f32_le();
            _struct.pcz = buf.get_f32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.vx as f32);
        _tmp.put_f32_le(self.vy as f32);
        _tmp.put_f32_le(self.vz as f32);
        _tmp.put_f32_le(self.diff_pressure as f32);
        _tmp.put_f32_le(self.eas2tas as f32);
        _tmp.put_f32_le(self.ratio as f32);
        _tmp.put_f32_le(self.state_x as f32);
        _tmp.put_f32_le(self.state_y as f32);
        _tmp.put_f32_le(self.state_z as f32);
        _tmp.put_f32_le(self.pax as f32);
        _tmp.put_f32_le(self.pby as f32);
        _tmp.put_f32_le(self.pcz as f32);
        _tmp
    }
}
impl crate::proto::ardupilotmega::RallyPoint {
    pub const ENCODED_LEN: usize = 19usize;
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
            _struct.lat = buf.get_i32_le();
            _struct.lng = buf.get_i32_le();
            _struct.alt = buf.get_i16_le() as i32;
            _struct.break_alt = buf.get_i16_le() as i32;
            _struct.land_dir = buf.get_u16_le() as u32;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            _struct.idx = buf.get_u8() as u32;
            _struct.count = buf.get_u8() as u32;
            let tmp = buf.get_u8();
            _struct.flags = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "RallyFlags".to_string(),
                value: tmp as u32,
            })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i32_le(self.lat as i32);
        _tmp.put_i32_le(self.lng as i32);
        _tmp.put_i16_le(self.alt as i16);
        _tmp.put_i16_le(self.break_alt as i16);
        _tmp.put_u16_le(self.land_dir as u16);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp.put_u8(self.idx as u8);
        _tmp.put_u8(self.count as u8);
        _tmp.put_u8(self.flags as u8);
        _tmp
    }
}
impl crate::proto::ardupilotmega::RallyFetchPoint {
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
            _struct.idx = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp.put_u8(self.idx as u8);
        _tmp
    }
}
impl crate::proto::ardupilotmega::CompassmotStatus {
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
            _struct.current = buf.get_f32_le();
            _struct.compensation_x = buf.get_f32_le();
            _struct.compensation_y = buf.get_f32_le();
            _struct.compensation_z = buf.get_f32_le();
            _struct.throttle = buf.get_u16_le() as u32;
            _struct.interference = buf.get_u16_le() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.current as f32);
        _tmp.put_f32_le(self.compensation_x as f32);
        _tmp.put_f32_le(self.compensation_y as f32);
        _tmp.put_f32_le(self.compensation_z as f32);
        _tmp.put_u16_le(self.throttle as u16);
        _tmp.put_u16_le(self.interference as u16);
        _tmp
    }
}
impl crate::proto::ardupilotmega::Ahrs2 {
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
            _struct.roll = buf.get_f32_le();
            _struct.pitch = buf.get_f32_le();
            _struct.yaw = buf.get_f32_le();
            _struct.altitude = buf.get_f32_le();
            _struct.lat = buf.get_i32_le();
            _struct.lng = buf.get_i32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.roll as f32);
        _tmp.put_f32_le(self.pitch as f32);
        _tmp.put_f32_le(self.yaw as f32);
        _tmp.put_f32_le(self.altitude as f32);
        _tmp.put_i32_le(self.lat as i32);
        _tmp.put_i32_le(self.lng as i32);
        _tmp
    }
}
impl crate::proto::ardupilotmega::CameraStatus {
    pub const ENCODED_LEN: usize = 29usize;
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
            _struct.time_usec = buf.get_u64_le();
            _struct.p1 = buf.get_f32_le();
            _struct.p2 = buf.get_f32_le();
            _struct.p3 = buf.get_f32_le();
            _struct.p4 = buf.get_f32_le();
            _struct.img_idx = buf.get_u16_le() as u32;
            _struct.target_system = buf.get_u8() as u32;
            _struct.cam_idx = buf.get_u8() as u32;
            let tmp = buf.get_u8();
            _struct.event_id = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "CameraStatusTypes".to_string(),
                value: tmp as u32,
            })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        _tmp.put_f32_le(self.p1 as f32);
        _tmp.put_f32_le(self.p2 as f32);
        _tmp.put_f32_le(self.p3 as f32);
        _tmp.put_f32_le(self.p4 as f32);
        _tmp.put_u16_le(self.img_idx as u16);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.cam_idx as u8);
        _tmp.put_u8(self.event_id as u8);
        _tmp
    }
}
impl crate::proto::ardupilotmega::CameraFeedback {
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
            _struct.time_usec = buf.get_u64_le();
            _struct.lat = buf.get_i32_le();
            _struct.lng = buf.get_i32_le();
            _struct.alt_msl = buf.get_f32_le();
            _struct.alt_rel = buf.get_f32_le();
            _struct.roll = buf.get_f32_le();
            _struct.pitch = buf.get_f32_le();
            _struct.yaw = buf.get_f32_le();
            _struct.foc_len = buf.get_f32_le();
            _struct.img_idx = buf.get_u16_le() as u32;
            _struct.target_system = buf.get_u8() as u32;
            _struct.cam_idx = buf.get_u8() as u32;
            let tmp = buf.get_u8();
            _struct.flags = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "CameraFeedbackFlags".to_string(),
                value: tmp as u32,
            })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        _tmp.put_i32_le(self.lat as i32);
        _tmp.put_i32_le(self.lng as i32);
        _tmp.put_f32_le(self.alt_msl as f32);
        _tmp.put_f32_le(self.alt_rel as f32);
        _tmp.put_f32_le(self.roll as f32);
        _tmp.put_f32_le(self.pitch as f32);
        _tmp.put_f32_le(self.yaw as f32);
        _tmp.put_f32_le(self.foc_len as f32);
        _tmp.put_u16_le(self.img_idx as u16);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.cam_idx as u8);
        _tmp.put_u8(self.flags as u8);
        _tmp
    }
}
impl crate::proto::ardupilotmega::Battery2 {
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
            _struct.voltage = buf.get_u16_le() as u32;
            _struct.current_battery = buf.get_i16_le() as i32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.voltage as u16);
        _tmp.put_i16_le(self.current_battery as i16);
        _tmp
    }
}
impl crate::proto::ardupilotmega::Ahrs3 {
    pub const ENCODED_LEN: usize = 40usize;
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
            _struct.roll = buf.get_f32_le();
            _struct.pitch = buf.get_f32_le();
            _struct.yaw = buf.get_f32_le();
            _struct.altitude = buf.get_f32_le();
            _struct.lat = buf.get_i32_le();
            _struct.lng = buf.get_i32_le();
            _struct.v1 = buf.get_f32_le();
            _struct.v2 = buf.get_f32_le();
            _struct.v3 = buf.get_f32_le();
            _struct.v4 = buf.get_f32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.roll as f32);
        _tmp.put_f32_le(self.pitch as f32);
        _tmp.put_f32_le(self.yaw as f32);
        _tmp.put_f32_le(self.altitude as f32);
        _tmp.put_i32_le(self.lat as i32);
        _tmp.put_i32_le(self.lng as i32);
        _tmp.put_f32_le(self.v1 as f32);
        _tmp.put_f32_le(self.v2 as f32);
        _tmp.put_f32_le(self.v3 as f32);
        _tmp.put_f32_le(self.v4 as f32);
        _tmp
    }
}
impl crate::proto::ardupilotmega::AutopilotVersionRequest {
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
impl crate::proto::ardupilotmega::RemoteLogDataBlock {
    pub const ENCODED_LEN: usize = 206usize;
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
            let tmp = buf.get_u32_le();
            _struct.seqno = FromPrimitive::from_u32(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavRemoteLogDataBlockCommands".to_string(),
                value: tmp as u32,
            })?;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            for _ in 0..200usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.data.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.seqno as u32);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        for val in &self.data {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
impl crate::proto::ardupilotmega::RemoteLogBlockStatus {
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
            _struct.seqno = buf.get_u32_le();
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            let tmp = buf.get_u8();
            _struct.status = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavRemoteLogDataBlockStatuses".to_string(),
                value: tmp as u32,
            })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.seqno as u32);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp.put_u8(self.status as u8);
        _tmp
    }
}
impl crate::proto::ardupilotmega::LedControl {
    pub const ENCODED_LEN: usize = 29usize;
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
            _struct.instance = buf.get_u8() as u32;
            _struct.pattern = buf.get_u8() as u32;
            _struct.custom_len = buf.get_u8() as u32;
            for _ in 0..24usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.custom_bytes.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp.put_u8(self.instance as u8);
        _tmp.put_u8(self.pattern as u8);
        _tmp.put_u8(self.custom_len as u8);
        for val in &self.custom_bytes {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
impl crate::proto::ardupilotmega::MagCalProgress {
    pub const ENCODED_LEN: usize = 27usize;
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
            _struct.direction_x = buf.get_f32_le();
            _struct.direction_y = buf.get_f32_le();
            _struct.direction_z = buf.get_f32_le();
            _struct.compass_id = buf.get_u8() as u32;
            _struct.cal_mask = buf.get_u8() as u32;
            let tmp = buf.get_u8();
            _struct.cal_status = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MagCalStatus".to_string(),
                value: tmp as u32,
            })?;
            _struct.attempt = buf.get_u8() as u32;
            _struct.completion_pct = buf.get_u8() as u32;
            for _ in 0..10usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.completion_mask.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.direction_x as f32);
        _tmp.put_f32_le(self.direction_y as f32);
        _tmp.put_f32_le(self.direction_z as f32);
        _tmp.put_u8(self.compass_id as u8);
        _tmp.put_u8(self.cal_mask as u8);
        _tmp.put_u8(self.cal_status as u8);
        _tmp.put_u8(self.attempt as u8);
        _tmp.put_u8(self.completion_pct as u8);
        for val in &self.completion_mask {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
impl crate::proto::ardupilotmega::MagCalReport {
    pub const ENCODED_LEN: usize = 44usize;
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
            _struct.fitness = buf.get_f32_le();
            _struct.ofs_x = buf.get_f32_le();
            _struct.ofs_y = buf.get_f32_le();
            _struct.ofs_z = buf.get_f32_le();
            _struct.diag_x = buf.get_f32_le();
            _struct.diag_y = buf.get_f32_le();
            _struct.diag_z = buf.get_f32_le();
            _struct.offdiag_x = buf.get_f32_le();
            _struct.offdiag_y = buf.get_f32_le();
            _struct.offdiag_z = buf.get_f32_le();
            _struct.compass_id = buf.get_u8() as u32;
            _struct.cal_mask = buf.get_u8() as u32;
            let tmp = buf.get_u8();
            _struct.cal_status = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MagCalStatus".to_string(),
                value: tmp as u32,
            })?;
            _struct.autosaved = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.fitness as f32);
        _tmp.put_f32_le(self.ofs_x as f32);
        _tmp.put_f32_le(self.ofs_y as f32);
        _tmp.put_f32_le(self.ofs_z as f32);
        _tmp.put_f32_le(self.diag_x as f32);
        _tmp.put_f32_le(self.diag_y as f32);
        _tmp.put_f32_le(self.diag_z as f32);
        _tmp.put_f32_le(self.offdiag_x as f32);
        _tmp.put_f32_le(self.offdiag_y as f32);
        _tmp.put_f32_le(self.offdiag_z as f32);
        _tmp.put_u8(self.compass_id as u8);
        _tmp.put_u8(self.cal_mask as u8);
        _tmp.put_u8(self.cal_status as u8);
        _tmp.put_u8(self.autosaved as u8);
        _tmp
    }
}
impl crate::proto::ardupilotmega::EkfStatusReport {
    pub const ENCODED_LEN: usize = 22usize;
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
            _struct.velocity_variance = buf.get_f32_le();
            _struct.pos_horiz_variance = buf.get_f32_le();
            _struct.pos_vert_variance = buf.get_f32_le();
            _struct.compass_variance = buf.get_f32_le();
            _struct.terrain_alt_variance = buf.get_f32_le();
            let tmp = buf.get_u16_le();
            _struct.flags = FromPrimitive::from_u16(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "EkfStatusFlags".to_string(),
                value: tmp as u32,
            })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.velocity_variance as f32);
        _tmp.put_f32_le(self.pos_horiz_variance as f32);
        _tmp.put_f32_le(self.pos_vert_variance as f32);
        _tmp.put_f32_le(self.compass_variance as f32);
        _tmp.put_f32_le(self.terrain_alt_variance as f32);
        _tmp.put_u16_le(self.flags as u16);
        _tmp
    }
}
impl crate::proto::ardupilotmega::PidTuning {
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
            _struct.desired = buf.get_f32_le();
            _struct.achieved = buf.get_f32_le();
            _struct.ff = buf.get_f32_le();
            _struct.p = buf.get_f32_le();
            _struct.i = buf.get_f32_le();
            _struct.d = buf.get_f32_le();
            let tmp = buf.get_u8();
            _struct.axis = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "PidTuningAxis".to_string(),
                value: tmp as u32,
            })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.desired as f32);
        _tmp.put_f32_le(self.achieved as f32);
        _tmp.put_f32_le(self.ff as f32);
        _tmp.put_f32_le(self.p as f32);
        _tmp.put_f32_le(self.i as f32);
        _tmp.put_f32_le(self.d as f32);
        _tmp.put_u8(self.axis as u8);
        _tmp
    }
}
impl crate::proto::ardupilotmega::Deepstall {
    pub const ENCODED_LEN: usize = 37usize;
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
            _struct.landing_lat = buf.get_i32_le();
            _struct.landing_lon = buf.get_i32_le();
            _struct.path_lat = buf.get_i32_le();
            _struct.path_lon = buf.get_i32_le();
            _struct.arc_entry_lat = buf.get_i32_le();
            _struct.arc_entry_lon = buf.get_i32_le();
            _struct.altitude = buf.get_f32_le();
            _struct.expected_travel_distance = buf.get_f32_le();
            _struct.cross_track_error = buf.get_f32_le();
            let tmp = buf.get_u8();
            _struct.stage = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "DeepstallStage".to_string(),
                value: tmp as u32,
            })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i32_le(self.landing_lat as i32);
        _tmp.put_i32_le(self.landing_lon as i32);
        _tmp.put_i32_le(self.path_lat as i32);
        _tmp.put_i32_le(self.path_lon as i32);
        _tmp.put_i32_le(self.arc_entry_lat as i32);
        _tmp.put_i32_le(self.arc_entry_lon as i32);
        _tmp.put_f32_le(self.altitude as f32);
        _tmp.put_f32_le(self.expected_travel_distance as f32);
        _tmp.put_f32_le(self.cross_track_error as f32);
        _tmp.put_u8(self.stage as u8);
        _tmp
    }
}
impl crate::proto::ardupilotmega::GimbalReport {
    pub const ENCODED_LEN: usize = 42usize;
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
            _struct.delta_time = buf.get_f32_le();
            _struct.delta_angle_x = buf.get_f32_le();
            _struct.delta_angle_y = buf.get_f32_le();
            _struct.delta_angle_z = buf.get_f32_le();
            _struct.delta_velocity_x = buf.get_f32_le();
            _struct.delta_velocity_y = buf.get_f32_le();
            _struct.delta_velocity_z = buf.get_f32_le();
            _struct.joint_roll = buf.get_f32_le();
            _struct.joint_el = buf.get_f32_le();
            _struct.joint_az = buf.get_f32_le();
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.delta_time as f32);
        _tmp.put_f32_le(self.delta_angle_x as f32);
        _tmp.put_f32_le(self.delta_angle_y as f32);
        _tmp.put_f32_le(self.delta_angle_z as f32);
        _tmp.put_f32_le(self.delta_velocity_x as f32);
        _tmp.put_f32_le(self.delta_velocity_y as f32);
        _tmp.put_f32_le(self.delta_velocity_z as f32);
        _tmp.put_f32_le(self.joint_roll as f32);
        _tmp.put_f32_le(self.joint_el as f32);
        _tmp.put_f32_le(self.joint_az as f32);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp
    }
}
impl crate::proto::ardupilotmega::GimbalControl {
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
            _struct.demanded_rate_x = buf.get_f32_le();
            _struct.demanded_rate_y = buf.get_f32_le();
            _struct.demanded_rate_z = buf.get_f32_le();
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.demanded_rate_x as f32);
        _tmp.put_f32_le(self.demanded_rate_y as f32);
        _tmp.put_f32_le(self.demanded_rate_z as f32);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp
    }
}
impl crate::proto::ardupilotmega::GimbalTorqueCmdReport {
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
            _struct.rl_torque_cmd = buf.get_i16_le() as i32;
            _struct.el_torque_cmd = buf.get_i16_le() as i32;
            _struct.az_torque_cmd = buf.get_i16_le() as i32;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i16_le(self.rl_torque_cmd as i16);
        _tmp.put_i16_le(self.el_torque_cmd as i16);
        _tmp.put_i16_le(self.az_torque_cmd as i16);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp
    }
}
impl crate::proto::ardupilotmega::GoproHeartbeat {
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
            let tmp = buf.get_u8();
            _struct.status = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "GoproHeartbeatStatus".to_string(),
                value: tmp as u32,
            })?;
            let tmp = buf.get_u8();
            _struct.capture_mode = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "GoproCaptureMode".to_string(),
                value: tmp as u32,
            })?;
            let tmp = buf.get_u8();
            _struct.flags = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "GoproHeartbeatFlags".to_string(),
                value: tmp as u32,
            })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.status as u8);
        _tmp.put_u8(self.capture_mode as u8);
        _tmp.put_u8(self.flags as u8);
        _tmp
    }
}
impl crate::proto::ardupilotmega::GoproGetRequest {
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
            let tmp = buf.get_u8();
            _struct.cmd_id = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "GoproCommand".to_string(),
                value: tmp as u32,
            })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp.put_u8(self.cmd_id as u8);
        _tmp
    }
}
impl crate::proto::ardupilotmega::GoproGetResponse {
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
            let tmp = buf.get_u8();
            _struct.cmd_id = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "GoproCommand".to_string(),
                value: tmp as u32,
            })?;
            let tmp = buf.get_u8();
            _struct.status = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "GoproRequestStatus".to_string(),
                value: tmp as u32,
            })?;
            for _ in 0..4usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.value.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.cmd_id as u8);
        _tmp.put_u8(self.status as u8);
        for val in &self.value {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
impl crate::proto::ardupilotmega::GoproSetRequest {
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
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            let tmp = buf.get_u8();
            _struct.cmd_id = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "GoproCommand".to_string(),
                value: tmp as u32,
            })?;
            for _ in 0..4usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.value.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp.put_u8(self.cmd_id as u8);
        for val in &self.value {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
impl crate::proto::ardupilotmega::GoproSetResponse {
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
            let tmp = buf.get_u8();
            _struct.cmd_id = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "GoproCommand".to_string(),
                value: tmp as u32,
            })?;
            let tmp = buf.get_u8();
            _struct.status = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "GoproRequestStatus".to_string(),
                value: tmp as u32,
            })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.cmd_id as u8);
        _tmp.put_u8(self.status as u8);
        _tmp
    }
}
impl crate::proto::ardupilotmega::EfiStatus {
    pub const ENCODED_LEN: usize = 65usize;
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
            _struct.ecu_index = buf.get_f32_le();
            _struct.rpm = buf.get_f32_le();
            _struct.fuel_consumed = buf.get_f32_le();
            _struct.fuel_flow = buf.get_f32_le();
            _struct.engine_load = buf.get_f32_le();
            _struct.throttle_position = buf.get_f32_le();
            _struct.spark_dwell_time = buf.get_f32_le();
            _struct.barometric_pressure = buf.get_f32_le();
            _struct.intake_manifold_pressure = buf.get_f32_le();
            _struct.intake_manifold_temperature = buf.get_f32_le();
            _struct.cylinder_head_temperature = buf.get_f32_le();
            _struct.ignition_timing = buf.get_f32_le();
            _struct.injection_time = buf.get_f32_le();
            _struct.exhaust_gas_temperature = buf.get_f32_le();
            _struct.throttle_out = buf.get_f32_le();
            _struct.pt_compensation = buf.get_f32_le();
            _struct.health = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.ecu_index as f32);
        _tmp.put_f32_le(self.rpm as f32);
        _tmp.put_f32_le(self.fuel_consumed as f32);
        _tmp.put_f32_le(self.fuel_flow as f32);
        _tmp.put_f32_le(self.engine_load as f32);
        _tmp.put_f32_le(self.throttle_position as f32);
        _tmp.put_f32_le(self.spark_dwell_time as f32);
        _tmp.put_f32_le(self.barometric_pressure as f32);
        _tmp.put_f32_le(self.intake_manifold_pressure as f32);
        _tmp.put_f32_le(self.intake_manifold_temperature as f32);
        _tmp.put_f32_le(self.cylinder_head_temperature as f32);
        _tmp.put_f32_le(self.ignition_timing as f32);
        _tmp.put_f32_le(self.injection_time as f32);
        _tmp.put_f32_le(self.exhaust_gas_temperature as f32);
        _tmp.put_f32_le(self.throttle_out as f32);
        _tmp.put_f32_le(self.pt_compensation as f32);
        _tmp.put_u8(self.health as u8);
        _tmp
    }
}
impl crate::proto::ardupilotmega::Rpm {
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
            _struct.rpm1 = buf.get_f32_le();
            _struct.rpm2 = buf.get_f32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.rpm1 as f32);
        _tmp.put_f32_le(self.rpm2 as f32);
        _tmp
    }
}
impl crate::proto::ardupilotmega::DeviceOpRead {
    pub const ENCODED_LEN: usize = 51usize;
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
            _struct.request_id = buf.get_u32_le();
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            let tmp = buf.get_u8();
            _struct.bustype = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "DeviceOpBustype".to_string(),
                value: tmp as u32,
            })?;
            _struct.bus = buf.get_u8() as u32;
            _struct.address = buf.get_u8() as u32;
            let mut s = Vec::with_capacity(40usize);
            for _ in 0..40usize {
                s.push(buf.get_u8());
            }
            _struct.busname = String::from_utf8_lossy(&s).into();
            _struct.regstart = buf.get_u8() as u32;
            _struct.count = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.request_id as u32);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp.put_u8(self.bustype as u8);
        _tmp.put_u8(self.bus as u8);
        _tmp.put_u8(self.address as u8);
        for val in self.busname.bytes() {
            _tmp.put_u8(val);
        }
        _tmp.put_u8(self.regstart as u8);
        _tmp.put_u8(self.count as u8);
        _tmp
    }
}
impl crate::proto::ardupilotmega::DeviceOpReadReply {
    pub const ENCODED_LEN: usize = 135usize;
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
            _struct.request_id = buf.get_u32_le();
            _struct.result = buf.get_u8() as u32;
            _struct.regstart = buf.get_u8() as u32;
            _struct.count = buf.get_u8() as u32;
            for _ in 0..128usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.data.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.request_id as u32);
        _tmp.put_u8(self.result as u8);
        _tmp.put_u8(self.regstart as u8);
        _tmp.put_u8(self.count as u8);
        for val in &self.data {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
impl crate::proto::ardupilotmega::DeviceOpWrite {
    pub const ENCODED_LEN: usize = 179usize;
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
            _struct.request_id = buf.get_u32_le();
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            let tmp = buf.get_u8();
            _struct.bustype = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "DeviceOpBustype".to_string(),
                value: tmp as u32,
            })?;
            _struct.bus = buf.get_u8() as u32;
            _struct.address = buf.get_u8() as u32;
            let mut s = Vec::with_capacity(40usize);
            for _ in 0..40usize {
                s.push(buf.get_u8());
            }
            _struct.busname = String::from_utf8_lossy(&s).into();
            _struct.regstart = buf.get_u8() as u32;
            _struct.count = buf.get_u8() as u32;
            for _ in 0..128usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.data.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.request_id as u32);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp.put_u8(self.bustype as u8);
        _tmp.put_u8(self.bus as u8);
        _tmp.put_u8(self.address as u8);
        for val in self.busname.bytes() {
            _tmp.put_u8(val);
        }
        _tmp.put_u8(self.regstart as u8);
        _tmp.put_u8(self.count as u8);
        for val in &self.data {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
impl crate::proto::ardupilotmega::DeviceOpWriteReply {
    pub const ENCODED_LEN: usize = 5usize;
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
            _struct.request_id = buf.get_u32_le();
            _struct.result = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.request_id as u32);
        _tmp.put_u8(self.result as u8);
        _tmp
    }
}
impl crate::proto::ardupilotmega::AdapTuning {
    pub const ENCODED_LEN: usize = 49usize;
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
            _struct.desired = buf.get_f32_le();
            _struct.achieved = buf.get_f32_le();
            _struct.error = buf.get_f32_le();
            _struct.theta = buf.get_f32_le();
            _struct.omega = buf.get_f32_le();
            _struct.sigma = buf.get_f32_le();
            _struct.theta_dot = buf.get_f32_le();
            _struct.omega_dot = buf.get_f32_le();
            _struct.sigma_dot = buf.get_f32_le();
            _struct.f = buf.get_f32_le();
            _struct.f_dot = buf.get_f32_le();
            _struct.u = buf.get_f32_le();
            let tmp = buf.get_u8();
            _struct.axis = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "PidTuningAxis".to_string(),
                value: tmp as u32,
            })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.desired as f32);
        _tmp.put_f32_le(self.achieved as f32);
        _tmp.put_f32_le(self.error as f32);
        _tmp.put_f32_le(self.theta as f32);
        _tmp.put_f32_le(self.omega as f32);
        _tmp.put_f32_le(self.sigma as f32);
        _tmp.put_f32_le(self.theta_dot as f32);
        _tmp.put_f32_le(self.omega_dot as f32);
        _tmp.put_f32_le(self.sigma_dot as f32);
        _tmp.put_f32_le(self.f as f32);
        _tmp.put_f32_le(self.f_dot as f32);
        _tmp.put_f32_le(self.u as f32);
        _tmp.put_u8(self.axis as u8);
        _tmp
    }
}
impl crate::proto::ardupilotmega::VisionPositionDelta {
    pub const ENCODED_LEN: usize = 44usize;
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
            _struct.time_usec = buf.get_u64_le();
            _struct.time_delta_usec = buf.get_u64_le();
            for _ in 0..3usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.angle_delta.push(val.into());
            }
            for _ in 0..3usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.position_delta.push(val.into());
            }
            _struct.confidence = buf.get_f32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        _tmp.put_u64_le(self.time_delta_usec as u64);
        for val in &self.angle_delta {
            _tmp.put_f32_le(*val as f32);
        }
        for val in &self.position_delta {
            _tmp.put_f32_le(*val as f32);
        }
        _tmp.put_f32_le(self.confidence as f32);
        _tmp
    }
}
impl crate::proto::ardupilotmega::AoaSsa {
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
            _struct.time_usec = buf.get_u64_le();
            _struct.aoa = buf.get_f32_le();
            _struct.ssa = buf.get_f32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        _tmp.put_f32_le(self.aoa as f32);
        _tmp.put_f32_le(self.ssa as f32);
        _tmp
    }
}
impl crate::proto::ardupilotmega::EscTelemetry1To4 {
    pub const ENCODED_LEN: usize = 44usize;
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
            for _ in 0..4usize {
                let val = buf.get_u16_le() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.voltage.push(val.into());
            }
            for _ in 0..4usize {
                let val = buf.get_u16_le() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.current.push(val.into());
            }
            for _ in 0..4usize {
                let val = buf.get_u16_le() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.totalcurrent.push(val.into());
            }
            for _ in 0..4usize {
                let val = buf.get_u16_le() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.rpm.push(val.into());
            }
            for _ in 0..4usize {
                let val = buf.get_u16_le() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.count.push(val.into());
            }
            for _ in 0..4usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.temperature.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        for val in &self.voltage {
            _tmp.put_u16_le(*val as u16);
        }
        for val in &self.current {
            _tmp.put_u16_le(*val as u16);
        }
        for val in &self.totalcurrent {
            _tmp.put_u16_le(*val as u16);
        }
        for val in &self.rpm {
            _tmp.put_u16_le(*val as u16);
        }
        for val in &self.count {
            _tmp.put_u16_le(*val as u16);
        }
        for val in &self.temperature {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
impl crate::proto::ardupilotmega::EscTelemetry5To8 {
    pub const ENCODED_LEN: usize = 44usize;
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
            for _ in 0..4usize {
                let val = buf.get_u16_le() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.voltage.push(val.into());
            }
            for _ in 0..4usize {
                let val = buf.get_u16_le() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.current.push(val.into());
            }
            for _ in 0..4usize {
                let val = buf.get_u16_le() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.totalcurrent.push(val.into());
            }
            for _ in 0..4usize {
                let val = buf.get_u16_le() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.rpm.push(val.into());
            }
            for _ in 0..4usize {
                let val = buf.get_u16_le() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.count.push(val.into());
            }
            for _ in 0..4usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.temperature.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        for val in &self.voltage {
            _tmp.put_u16_le(*val as u16);
        }
        for val in &self.current {
            _tmp.put_u16_le(*val as u16);
        }
        for val in &self.totalcurrent {
            _tmp.put_u16_le(*val as u16);
        }
        for val in &self.rpm {
            _tmp.put_u16_le(*val as u16);
        }
        for val in &self.count {
            _tmp.put_u16_le(*val as u16);
        }
        for val in &self.temperature {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
impl crate::proto::ardupilotmega::EscTelemetry9To12 {
    pub const ENCODED_LEN: usize = 44usize;
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
            for _ in 0..4usize {
                let val = buf.get_u16_le() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.voltage.push(val.into());
            }
            for _ in 0..4usize {
                let val = buf.get_u16_le() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.current.push(val.into());
            }
            for _ in 0..4usize {
                let val = buf.get_u16_le() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.totalcurrent.push(val.into());
            }
            for _ in 0..4usize {
                let val = buf.get_u16_le() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.rpm.push(val.into());
            }
            for _ in 0..4usize {
                let val = buf.get_u16_le() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.count.push(val.into());
            }
            for _ in 0..4usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.temperature.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        for val in &self.voltage {
            _tmp.put_u16_le(*val as u16);
        }
        for val in &self.current {
            _tmp.put_u16_le(*val as u16);
        }
        for val in &self.totalcurrent {
            _tmp.put_u16_le(*val as u16);
        }
        for val in &self.rpm {
            _tmp.put_u16_le(*val as u16);
        }
        for val in &self.count {
            _tmp.put_u16_le(*val as u16);
        }
        for val in &self.temperature {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
#[derive(Clone, PartialEq, Debug)]
pub enum MavMessage {
    SensorOffsets(crate::proto::ardupilotmega::SensorOffsets),
    SetMagOffsets(crate::proto::ardupilotmega::SetMagOffsets),
    Meminfo(crate::proto::ardupilotmega::Meminfo),
    ApAdc(crate::proto::ardupilotmega::ApAdc),
    DigicamConfigure(crate::proto::ardupilotmega::DigicamConfigure),
    DigicamControl(crate::proto::ardupilotmega::DigicamControl),
    MountConfigure(crate::proto::ardupilotmega::MountConfigure),
    MountControl(crate::proto::ardupilotmega::MountControl),
    MountStatus(crate::proto::ardupilotmega::MountStatus),
    FencePoint(crate::proto::ardupilotmega::FencePoint),
    FenceFetchPoint(crate::proto::ardupilotmega::FenceFetchPoint),
    Ahrs(crate::proto::ardupilotmega::Ahrs),
    Simstate(crate::proto::ardupilotmega::Simstate),
    Hwstatus(crate::proto::ardupilotmega::Hwstatus),
    Radio(crate::proto::ardupilotmega::Radio),
    LimitsStatus(crate::proto::ardupilotmega::LimitsStatus),
    Wind(crate::proto::ardupilotmega::Wind),
    Data16(crate::proto::ardupilotmega::Data16),
    Data32(crate::proto::ardupilotmega::Data32),
    Data64(crate::proto::ardupilotmega::Data64),
    Data96(crate::proto::ardupilotmega::Data96),
    Rangefinder(crate::proto::ardupilotmega::Rangefinder),
    AirspeedAutocal(crate::proto::ardupilotmega::AirspeedAutocal),
    RallyPoint(crate::proto::ardupilotmega::RallyPoint),
    RallyFetchPoint(crate::proto::ardupilotmega::RallyFetchPoint),
    CompassmotStatus(crate::proto::ardupilotmega::CompassmotStatus),
    Ahrs2(crate::proto::ardupilotmega::Ahrs2),
    CameraStatus(crate::proto::ardupilotmega::CameraStatus),
    CameraFeedback(crate::proto::ardupilotmega::CameraFeedback),
    Battery2(crate::proto::ardupilotmega::Battery2),
    Ahrs3(crate::proto::ardupilotmega::Ahrs3),
    AutopilotVersionRequest(crate::proto::ardupilotmega::AutopilotVersionRequest),
    RemoteLogDataBlock(crate::proto::ardupilotmega::RemoteLogDataBlock),
    RemoteLogBlockStatus(crate::proto::ardupilotmega::RemoteLogBlockStatus),
    LedControl(crate::proto::ardupilotmega::LedControl),
    MagCalProgress(crate::proto::ardupilotmega::MagCalProgress),
    MagCalReport(crate::proto::ardupilotmega::MagCalReport),
    EkfStatusReport(crate::proto::ardupilotmega::EkfStatusReport),
    PidTuning(crate::proto::ardupilotmega::PidTuning),
    Deepstall(crate::proto::ardupilotmega::Deepstall),
    GimbalReport(crate::proto::ardupilotmega::GimbalReport),
    GimbalControl(crate::proto::ardupilotmega::GimbalControl),
    GimbalTorqueCmdReport(crate::proto::ardupilotmega::GimbalTorqueCmdReport),
    GoproHeartbeat(crate::proto::ardupilotmega::GoproHeartbeat),
    GoproGetRequest(crate::proto::ardupilotmega::GoproGetRequest),
    GoproGetResponse(crate::proto::ardupilotmega::GoproGetResponse),
    GoproSetRequest(crate::proto::ardupilotmega::GoproSetRequest),
    GoproSetResponse(crate::proto::ardupilotmega::GoproSetResponse),
    EfiStatus(crate::proto::ardupilotmega::EfiStatus),
    Rpm(crate::proto::ardupilotmega::Rpm),
    DeviceOpRead(crate::proto::ardupilotmega::DeviceOpRead),
    DeviceOpReadReply(crate::proto::ardupilotmega::DeviceOpReadReply),
    DeviceOpWrite(crate::proto::ardupilotmega::DeviceOpWrite),
    DeviceOpWriteReply(crate::proto::ardupilotmega::DeviceOpWriteReply),
    AdapTuning(crate::proto::ardupilotmega::AdapTuning),
    VisionPositionDelta(crate::proto::ardupilotmega::VisionPositionDelta),
    AoaSsa(crate::proto::ardupilotmega::AoaSsa),
    EscTelemetry1To4(crate::proto::ardupilotmega::EscTelemetry1To4),
    EscTelemetry5To8(crate::proto::ardupilotmega::EscTelemetry5To8),
    EscTelemetry9To12(crate::proto::ardupilotmega::EscTelemetry9To12),
    Common(crate::mavlink::common::MavMessage),
    Uavionix(crate::mavlink::uavionix::MavMessage),
    Icarous(crate::mavlink::icarous::MavMessage),
}
impl From<crate::mavlink::common::MavMessage> for MavMessage {
    fn from(message: crate::mavlink::common::MavMessage) -> Self {
        MavMessage::Common(message)
    }
}
impl From<crate::mavlink::uavionix::MavMessage> for MavMessage {
    fn from(message: crate::mavlink::uavionix::MavMessage) -> Self {
        MavMessage::Uavionix(message)
    }
}
impl From<crate::mavlink::icarous::MavMessage> for MavMessage {
    fn from(message: crate::mavlink::icarous::MavMessage) -> Self {
        MavMessage::Icarous(message)
    }
}
impl Message for MavMessage {
    fn parse(version: MavlinkVersion, id: u32, payload: &[u8]) -> Result<MavMessage, ParserError> {
        match id {
            150 => crate::proto::ardupilotmega::SensorOffsets::mavlink_deser(version, payload)
                .map(MavMessage::SensorOffsets),
            151 => crate::proto::ardupilotmega::SetMagOffsets::mavlink_deser(version, payload)
                .map(MavMessage::SetMagOffsets),
            152 => crate::proto::ardupilotmega::Meminfo::mavlink_deser(version, payload)
                .map(MavMessage::Meminfo),
            153 => crate::proto::ardupilotmega::ApAdc::mavlink_deser(version, payload)
                .map(MavMessage::ApAdc),
            154 => crate::proto::ardupilotmega::DigicamConfigure::mavlink_deser(version, payload)
                .map(MavMessage::DigicamConfigure),
            155 => crate::proto::ardupilotmega::DigicamControl::mavlink_deser(version, payload)
                .map(MavMessage::DigicamControl),
            156 => crate::proto::ardupilotmega::MountConfigure::mavlink_deser(version, payload)
                .map(MavMessage::MountConfigure),
            157 => crate::proto::ardupilotmega::MountControl::mavlink_deser(version, payload)
                .map(MavMessage::MountControl),
            158 => crate::proto::ardupilotmega::MountStatus::mavlink_deser(version, payload)
                .map(MavMessage::MountStatus),
            160 => crate::proto::ardupilotmega::FencePoint::mavlink_deser(version, payload)
                .map(MavMessage::FencePoint),
            161 => crate::proto::ardupilotmega::FenceFetchPoint::mavlink_deser(version, payload)
                .map(MavMessage::FenceFetchPoint),
            163 => crate::proto::ardupilotmega::Ahrs::mavlink_deser(version, payload)
                .map(MavMessage::Ahrs),
            164 => crate::proto::ardupilotmega::Simstate::mavlink_deser(version, payload)
                .map(MavMessage::Simstate),
            165 => crate::proto::ardupilotmega::Hwstatus::mavlink_deser(version, payload)
                .map(MavMessage::Hwstatus),
            166 => crate::proto::ardupilotmega::Radio::mavlink_deser(version, payload)
                .map(MavMessage::Radio),
            167 => crate::proto::ardupilotmega::LimitsStatus::mavlink_deser(version, payload)
                .map(MavMessage::LimitsStatus),
            168 => crate::proto::ardupilotmega::Wind::mavlink_deser(version, payload)
                .map(MavMessage::Wind),
            169 => crate::proto::ardupilotmega::Data16::mavlink_deser(version, payload)
                .map(MavMessage::Data16),
            170 => crate::proto::ardupilotmega::Data32::mavlink_deser(version, payload)
                .map(MavMessage::Data32),
            171 => crate::proto::ardupilotmega::Data64::mavlink_deser(version, payload)
                .map(MavMessage::Data64),
            172 => crate::proto::ardupilotmega::Data96::mavlink_deser(version, payload)
                .map(MavMessage::Data96),
            173 => crate::proto::ardupilotmega::Rangefinder::mavlink_deser(version, payload)
                .map(MavMessage::Rangefinder),
            174 => crate::proto::ardupilotmega::AirspeedAutocal::mavlink_deser(version, payload)
                .map(MavMessage::AirspeedAutocal),
            175 => crate::proto::ardupilotmega::RallyPoint::mavlink_deser(version, payload)
                .map(MavMessage::RallyPoint),
            176 => crate::proto::ardupilotmega::RallyFetchPoint::mavlink_deser(version, payload)
                .map(MavMessage::RallyFetchPoint),
            177 => crate::proto::ardupilotmega::CompassmotStatus::mavlink_deser(version, payload)
                .map(MavMessage::CompassmotStatus),
            178 => crate::proto::ardupilotmega::Ahrs2::mavlink_deser(version, payload)
                .map(MavMessage::Ahrs2),
            179 => crate::proto::ardupilotmega::CameraStatus::mavlink_deser(version, payload)
                .map(MavMessage::CameraStatus),
            180 => crate::proto::ardupilotmega::CameraFeedback::mavlink_deser(version, payload)
                .map(MavMessage::CameraFeedback),
            181 => crate::proto::ardupilotmega::Battery2::mavlink_deser(version, payload)
                .map(MavMessage::Battery2),
            182 => crate::proto::ardupilotmega::Ahrs3::mavlink_deser(version, payload)
                .map(MavMessage::Ahrs3),
            183 => crate::proto::ardupilotmega::AutopilotVersionRequest::mavlink_deser(
                version, payload,
            )
            .map(MavMessage::AutopilotVersionRequest),
            184 => crate::proto::ardupilotmega::RemoteLogDataBlock::mavlink_deser(version, payload)
                .map(MavMessage::RemoteLogDataBlock),
            185 => {
                crate::proto::ardupilotmega::RemoteLogBlockStatus::mavlink_deser(version, payload)
                    .map(MavMessage::RemoteLogBlockStatus)
            }
            186 => crate::proto::ardupilotmega::LedControl::mavlink_deser(version, payload)
                .map(MavMessage::LedControl),
            191 => crate::proto::ardupilotmega::MagCalProgress::mavlink_deser(version, payload)
                .map(MavMessage::MagCalProgress),
            192 => crate::proto::ardupilotmega::MagCalReport::mavlink_deser(version, payload)
                .map(MavMessage::MagCalReport),
            193 => crate::proto::ardupilotmega::EkfStatusReport::mavlink_deser(version, payload)
                .map(MavMessage::EkfStatusReport),
            194 => crate::proto::ardupilotmega::PidTuning::mavlink_deser(version, payload)
                .map(MavMessage::PidTuning),
            195 => crate::proto::ardupilotmega::Deepstall::mavlink_deser(version, payload)
                .map(MavMessage::Deepstall),
            200 => crate::proto::ardupilotmega::GimbalReport::mavlink_deser(version, payload)
                .map(MavMessage::GimbalReport),
            201 => crate::proto::ardupilotmega::GimbalControl::mavlink_deser(version, payload)
                .map(MavMessage::GimbalControl),
            214 => {
                crate::proto::ardupilotmega::GimbalTorqueCmdReport::mavlink_deser(version, payload)
                    .map(MavMessage::GimbalTorqueCmdReport)
            }
            215 => crate::proto::ardupilotmega::GoproHeartbeat::mavlink_deser(version, payload)
                .map(MavMessage::GoproHeartbeat),
            216 => crate::proto::ardupilotmega::GoproGetRequest::mavlink_deser(version, payload)
                .map(MavMessage::GoproGetRequest),
            217 => crate::proto::ardupilotmega::GoproGetResponse::mavlink_deser(version, payload)
                .map(MavMessage::GoproGetResponse),
            218 => crate::proto::ardupilotmega::GoproSetRequest::mavlink_deser(version, payload)
                .map(MavMessage::GoproSetRequest),
            219 => crate::proto::ardupilotmega::GoproSetResponse::mavlink_deser(version, payload)
                .map(MavMessage::GoproSetResponse),
            225 => crate::proto::ardupilotmega::EfiStatus::mavlink_deser(version, payload)
                .map(MavMessage::EfiStatus),
            226 => crate::proto::ardupilotmega::Rpm::mavlink_deser(version, payload)
                .map(MavMessage::Rpm),
            11000 => crate::proto::ardupilotmega::DeviceOpRead::mavlink_deser(version, payload)
                .map(MavMessage::DeviceOpRead),
            11001 => {
                crate::proto::ardupilotmega::DeviceOpReadReply::mavlink_deser(version, payload)
                    .map(MavMessage::DeviceOpReadReply)
            }
            11002 => crate::proto::ardupilotmega::DeviceOpWrite::mavlink_deser(version, payload)
                .map(MavMessage::DeviceOpWrite),
            11003 => {
                crate::proto::ardupilotmega::DeviceOpWriteReply::mavlink_deser(version, payload)
                    .map(MavMessage::DeviceOpWriteReply)
            }
            11010 => crate::proto::ardupilotmega::AdapTuning::mavlink_deser(version, payload)
                .map(MavMessage::AdapTuning),
            11011 => {
                crate::proto::ardupilotmega::VisionPositionDelta::mavlink_deser(version, payload)
                    .map(MavMessage::VisionPositionDelta)
            }
            11020 => crate::proto::ardupilotmega::AoaSsa::mavlink_deser(version, payload)
                .map(MavMessage::AoaSsa),
            11030 => crate::proto::ardupilotmega::EscTelemetry1To4::mavlink_deser(version, payload)
                .map(MavMessage::EscTelemetry1To4),
            11031 => crate::proto::ardupilotmega::EscTelemetry5To8::mavlink_deser(version, payload)
                .map(MavMessage::EscTelemetry5To8),
            11032 => {
                crate::proto::ardupilotmega::EscTelemetry9To12::mavlink_deser(version, payload)
                    .map(MavMessage::EscTelemetry9To12)
            }
            _ => {
                if let Ok(msg) = crate::mavlink::common::MavMessage::parse(version, id, payload) {
                    return Ok(MavMessage::Common(msg));
                }
                if let Ok(msg) = crate::mavlink::uavionix::MavMessage::parse(version, id, payload) {
                    return Ok(MavMessage::Uavionix(msg));
                }
                if let Ok(msg) = crate::mavlink::icarous::MavMessage::parse(version, id, payload) {
                    return Ok(MavMessage::Icarous(msg));
                }
                Err(ParserError::UnknownMessage { id })
            }
        }
    }
    fn message_name(&self) -> &'static str {
        match self {
            MavMessage::SensorOffsets(..) => "SensorOffsets",
            MavMessage::SetMagOffsets(..) => "SetMagOffsets",
            MavMessage::Meminfo(..) => "Meminfo",
            MavMessage::ApAdc(..) => "ApAdc",
            MavMessage::DigicamConfigure(..) => "DigicamConfigure",
            MavMessage::DigicamControl(..) => "DigicamControl",
            MavMessage::MountConfigure(..) => "MountConfigure",
            MavMessage::MountControl(..) => "MountControl",
            MavMessage::MountStatus(..) => "MountStatus",
            MavMessage::FencePoint(..) => "FencePoint",
            MavMessage::FenceFetchPoint(..) => "FenceFetchPoint",
            MavMessage::Ahrs(..) => "Ahrs",
            MavMessage::Simstate(..) => "Simstate",
            MavMessage::Hwstatus(..) => "Hwstatus",
            MavMessage::Radio(..) => "Radio",
            MavMessage::LimitsStatus(..) => "LimitsStatus",
            MavMessage::Wind(..) => "Wind",
            MavMessage::Data16(..) => "Data16",
            MavMessage::Data32(..) => "Data32",
            MavMessage::Data64(..) => "Data64",
            MavMessage::Data96(..) => "Data96",
            MavMessage::Rangefinder(..) => "Rangefinder",
            MavMessage::AirspeedAutocal(..) => "AirspeedAutocal",
            MavMessage::RallyPoint(..) => "RallyPoint",
            MavMessage::RallyFetchPoint(..) => "RallyFetchPoint",
            MavMessage::CompassmotStatus(..) => "CompassmotStatus",
            MavMessage::Ahrs2(..) => "Ahrs2",
            MavMessage::CameraStatus(..) => "CameraStatus",
            MavMessage::CameraFeedback(..) => "CameraFeedback",
            MavMessage::Battery2(..) => "Battery2",
            MavMessage::Ahrs3(..) => "Ahrs3",
            MavMessage::AutopilotVersionRequest(..) => "AutopilotVersionRequest",
            MavMessage::RemoteLogDataBlock(..) => "RemoteLogDataBlock",
            MavMessage::RemoteLogBlockStatus(..) => "RemoteLogBlockStatus",
            MavMessage::LedControl(..) => "LedControl",
            MavMessage::MagCalProgress(..) => "MagCalProgress",
            MavMessage::MagCalReport(..) => "MagCalReport",
            MavMessage::EkfStatusReport(..) => "EkfStatusReport",
            MavMessage::PidTuning(..) => "PidTuning",
            MavMessage::Deepstall(..) => "Deepstall",
            MavMessage::GimbalReport(..) => "GimbalReport",
            MavMessage::GimbalControl(..) => "GimbalControl",
            MavMessage::GimbalTorqueCmdReport(..) => "GimbalTorqueCmdReport",
            MavMessage::GoproHeartbeat(..) => "GoproHeartbeat",
            MavMessage::GoproGetRequest(..) => "GoproGetRequest",
            MavMessage::GoproGetResponse(..) => "GoproGetResponse",
            MavMessage::GoproSetRequest(..) => "GoproSetRequest",
            MavMessage::GoproSetResponse(..) => "GoproSetResponse",
            MavMessage::EfiStatus(..) => "EfiStatus",
            MavMessage::Rpm(..) => "Rpm",
            MavMessage::DeviceOpRead(..) => "DeviceOpRead",
            MavMessage::DeviceOpReadReply(..) => "DeviceOpReadReply",
            MavMessage::DeviceOpWrite(..) => "DeviceOpWrite",
            MavMessage::DeviceOpWriteReply(..) => "DeviceOpWriteReply",
            MavMessage::AdapTuning(..) => "AdapTuning",
            MavMessage::VisionPositionDelta(..) => "VisionPositionDelta",
            MavMessage::AoaSsa(..) => "AoaSsa",
            MavMessage::EscTelemetry1To4(..) => "EscTelemetry1To4",
            MavMessage::EscTelemetry5To8(..) => "EscTelemetry5To8",
            MavMessage::EscTelemetry9To12(..) => "EscTelemetry9To12",
            MavMessage::Common(msg) => msg.message_name(),
            MavMessage::Uavionix(msg) => msg.message_name(),
            MavMessage::Icarous(msg) => msg.message_name(),
        }
    }
    fn message_id(&self) -> u32 {
        match self {
            MavMessage::SensorOffsets(..) => 150,
            MavMessage::SetMagOffsets(..) => 151,
            MavMessage::Meminfo(..) => 152,
            MavMessage::ApAdc(..) => 153,
            MavMessage::DigicamConfigure(..) => 154,
            MavMessage::DigicamControl(..) => 155,
            MavMessage::MountConfigure(..) => 156,
            MavMessage::MountControl(..) => 157,
            MavMessage::MountStatus(..) => 158,
            MavMessage::FencePoint(..) => 160,
            MavMessage::FenceFetchPoint(..) => 161,
            MavMessage::Ahrs(..) => 163,
            MavMessage::Simstate(..) => 164,
            MavMessage::Hwstatus(..) => 165,
            MavMessage::Radio(..) => 166,
            MavMessage::LimitsStatus(..) => 167,
            MavMessage::Wind(..) => 168,
            MavMessage::Data16(..) => 169,
            MavMessage::Data32(..) => 170,
            MavMessage::Data64(..) => 171,
            MavMessage::Data96(..) => 172,
            MavMessage::Rangefinder(..) => 173,
            MavMessage::AirspeedAutocal(..) => 174,
            MavMessage::RallyPoint(..) => 175,
            MavMessage::RallyFetchPoint(..) => 176,
            MavMessage::CompassmotStatus(..) => 177,
            MavMessage::Ahrs2(..) => 178,
            MavMessage::CameraStatus(..) => 179,
            MavMessage::CameraFeedback(..) => 180,
            MavMessage::Battery2(..) => 181,
            MavMessage::Ahrs3(..) => 182,
            MavMessage::AutopilotVersionRequest(..) => 183,
            MavMessage::RemoteLogDataBlock(..) => 184,
            MavMessage::RemoteLogBlockStatus(..) => 185,
            MavMessage::LedControl(..) => 186,
            MavMessage::MagCalProgress(..) => 191,
            MavMessage::MagCalReport(..) => 192,
            MavMessage::EkfStatusReport(..) => 193,
            MavMessage::PidTuning(..) => 194,
            MavMessage::Deepstall(..) => 195,
            MavMessage::GimbalReport(..) => 200,
            MavMessage::GimbalControl(..) => 201,
            MavMessage::GimbalTorqueCmdReport(..) => 214,
            MavMessage::GoproHeartbeat(..) => 215,
            MavMessage::GoproGetRequest(..) => 216,
            MavMessage::GoproGetResponse(..) => 217,
            MavMessage::GoproSetRequest(..) => 218,
            MavMessage::GoproSetResponse(..) => 219,
            MavMessage::EfiStatus(..) => 225,
            MavMessage::Rpm(..) => 226,
            MavMessage::DeviceOpRead(..) => 11000,
            MavMessage::DeviceOpReadReply(..) => 11001,
            MavMessage::DeviceOpWrite(..) => 11002,
            MavMessage::DeviceOpWriteReply(..) => 11003,
            MavMessage::AdapTuning(..) => 11010,
            MavMessage::VisionPositionDelta(..) => 11011,
            MavMessage::AoaSsa(..) => 11020,
            MavMessage::EscTelemetry1To4(..) => 11030,
            MavMessage::EscTelemetry5To8(..) => 11031,
            MavMessage::EscTelemetry9To12(..) => 11032,
            MavMessage::Common(msg) => msg.message_id(),
            MavMessage::Uavionix(msg) => msg.message_id(),
            MavMessage::Icarous(msg) => msg.message_id(),
        }
    }
    fn message_id_from_name(name: &str) -> Result<u32, &'static str> {
        match name {
            "SensorOffsets" => Ok(150),
            "SetMagOffsets" => Ok(151),
            "Meminfo" => Ok(152),
            "ApAdc" => Ok(153),
            "DigicamConfigure" => Ok(154),
            "DigicamControl" => Ok(155),
            "MountConfigure" => Ok(156),
            "MountControl" => Ok(157),
            "MountStatus" => Ok(158),
            "FencePoint" => Ok(160),
            "FenceFetchPoint" => Ok(161),
            "Ahrs" => Ok(163),
            "Simstate" => Ok(164),
            "Hwstatus" => Ok(165),
            "Radio" => Ok(166),
            "LimitsStatus" => Ok(167),
            "Wind" => Ok(168),
            "Data16" => Ok(169),
            "Data32" => Ok(170),
            "Data64" => Ok(171),
            "Data96" => Ok(172),
            "Rangefinder" => Ok(173),
            "AirspeedAutocal" => Ok(174),
            "RallyPoint" => Ok(175),
            "RallyFetchPoint" => Ok(176),
            "CompassmotStatus" => Ok(177),
            "Ahrs2" => Ok(178),
            "CameraStatus" => Ok(179),
            "CameraFeedback" => Ok(180),
            "Battery2" => Ok(181),
            "Ahrs3" => Ok(182),
            "AutopilotVersionRequest" => Ok(183),
            "RemoteLogDataBlock" => Ok(184),
            "RemoteLogBlockStatus" => Ok(185),
            "LedControl" => Ok(186),
            "MagCalProgress" => Ok(191),
            "MagCalReport" => Ok(192),
            "EkfStatusReport" => Ok(193),
            "PidTuning" => Ok(194),
            "Deepstall" => Ok(195),
            "GimbalReport" => Ok(200),
            "GimbalControl" => Ok(201),
            "GimbalTorqueCmdReport" => Ok(214),
            "GoproHeartbeat" => Ok(215),
            "GoproGetRequest" => Ok(216),
            "GoproGetResponse" => Ok(217),
            "GoproSetRequest" => Ok(218),
            "GoproSetResponse" => Ok(219),
            "EfiStatus" => Ok(225),
            "Rpm" => Ok(226),
            "DeviceOpRead" => Ok(11000),
            "DeviceOpReadReply" => Ok(11001),
            "DeviceOpWrite" => Ok(11002),
            "DeviceOpWriteReply" => Ok(11003),
            "AdapTuning" => Ok(11010),
            "VisionPositionDelta" => Ok(11011),
            "AoaSsa" => Ok(11020),
            "EscTelemetry1To4" => Ok(11030),
            "EscTelemetry5To8" => Ok(11031),
            "EscTelemetry9To12" => Ok(11032),
            _ => {
                match crate::mavlink::common::MavMessage::message_id_from_name(name) {
                    Ok(name) => return Ok(name),
                    Err(..) => {}
                }
                match crate::mavlink::uavionix::MavMessage::message_id_from_name(name) {
                    Ok(name) => return Ok(name),
                    Err(..) => {}
                }
                match crate::mavlink::icarous::MavMessage::message_id_from_name(name) {
                    Ok(name) => return Ok(name),
                    Err(..) => {}
                }
                Err("Invalid message name.")
            }
        }
    }
    fn default_message_from_id(id: u32) -> Result<MavMessage, &'static str> {
        match id {
            150 => Ok(MavMessage::SensorOffsets(
                crate::proto::ardupilotmega::SensorOffsets::default(),
            )),
            151 => Ok(MavMessage::SetMagOffsets(
                crate::proto::ardupilotmega::SetMagOffsets::default(),
            )),
            152 => Ok(MavMessage::Meminfo(
                crate::proto::ardupilotmega::Meminfo::default(),
            )),
            153 => Ok(MavMessage::ApAdc(
                crate::proto::ardupilotmega::ApAdc::default(),
            )),
            154 => Ok(MavMessage::DigicamConfigure(
                crate::proto::ardupilotmega::DigicamConfigure::default(),
            )),
            155 => Ok(MavMessage::DigicamControl(
                crate::proto::ardupilotmega::DigicamControl::default(),
            )),
            156 => Ok(MavMessage::MountConfigure(
                crate::proto::ardupilotmega::MountConfigure::default(),
            )),
            157 => Ok(MavMessage::MountControl(
                crate::proto::ardupilotmega::MountControl::default(),
            )),
            158 => Ok(MavMessage::MountStatus(
                crate::proto::ardupilotmega::MountStatus::default(),
            )),
            160 => Ok(MavMessage::FencePoint(
                crate::proto::ardupilotmega::FencePoint::default(),
            )),
            161 => Ok(MavMessage::FenceFetchPoint(
                crate::proto::ardupilotmega::FenceFetchPoint::default(),
            )),
            163 => Ok(MavMessage::Ahrs(
                crate::proto::ardupilotmega::Ahrs::default(),
            )),
            164 => Ok(MavMessage::Simstate(
                crate::proto::ardupilotmega::Simstate::default(),
            )),
            165 => Ok(MavMessage::Hwstatus(
                crate::proto::ardupilotmega::Hwstatus::default(),
            )),
            166 => Ok(MavMessage::Radio(
                crate::proto::ardupilotmega::Radio::default(),
            )),
            167 => Ok(MavMessage::LimitsStatus(
                crate::proto::ardupilotmega::LimitsStatus::default(),
            )),
            168 => Ok(MavMessage::Wind(
                crate::proto::ardupilotmega::Wind::default(),
            )),
            169 => Ok(MavMessage::Data16(
                crate::proto::ardupilotmega::Data16::default(),
            )),
            170 => Ok(MavMessage::Data32(
                crate::proto::ardupilotmega::Data32::default(),
            )),
            171 => Ok(MavMessage::Data64(
                crate::proto::ardupilotmega::Data64::default(),
            )),
            172 => Ok(MavMessage::Data96(
                crate::proto::ardupilotmega::Data96::default(),
            )),
            173 => Ok(MavMessage::Rangefinder(
                crate::proto::ardupilotmega::Rangefinder::default(),
            )),
            174 => Ok(MavMessage::AirspeedAutocal(
                crate::proto::ardupilotmega::AirspeedAutocal::default(),
            )),
            175 => Ok(MavMessage::RallyPoint(
                crate::proto::ardupilotmega::RallyPoint::default(),
            )),
            176 => Ok(MavMessage::RallyFetchPoint(
                crate::proto::ardupilotmega::RallyFetchPoint::default(),
            )),
            177 => Ok(MavMessage::CompassmotStatus(
                crate::proto::ardupilotmega::CompassmotStatus::default(),
            )),
            178 => Ok(MavMessage::Ahrs2(
                crate::proto::ardupilotmega::Ahrs2::default(),
            )),
            179 => Ok(MavMessage::CameraStatus(
                crate::proto::ardupilotmega::CameraStatus::default(),
            )),
            180 => Ok(MavMessage::CameraFeedback(
                crate::proto::ardupilotmega::CameraFeedback::default(),
            )),
            181 => Ok(MavMessage::Battery2(
                crate::proto::ardupilotmega::Battery2::default(),
            )),
            182 => Ok(MavMessage::Ahrs3(
                crate::proto::ardupilotmega::Ahrs3::default(),
            )),
            183 => Ok(MavMessage::AutopilotVersionRequest(
                crate::proto::ardupilotmega::AutopilotVersionRequest::default(),
            )),
            184 => Ok(MavMessage::RemoteLogDataBlock(
                crate::proto::ardupilotmega::RemoteLogDataBlock::default(),
            )),
            185 => Ok(MavMessage::RemoteLogBlockStatus(
                crate::proto::ardupilotmega::RemoteLogBlockStatus::default(),
            )),
            186 => Ok(MavMessage::LedControl(
                crate::proto::ardupilotmega::LedControl::default(),
            )),
            191 => Ok(MavMessage::MagCalProgress(
                crate::proto::ardupilotmega::MagCalProgress::default(),
            )),
            192 => Ok(MavMessage::MagCalReport(
                crate::proto::ardupilotmega::MagCalReport::default(),
            )),
            193 => Ok(MavMessage::EkfStatusReport(
                crate::proto::ardupilotmega::EkfStatusReport::default(),
            )),
            194 => Ok(MavMessage::PidTuning(
                crate::proto::ardupilotmega::PidTuning::default(),
            )),
            195 => Ok(MavMessage::Deepstall(
                crate::proto::ardupilotmega::Deepstall::default(),
            )),
            200 => Ok(MavMessage::GimbalReport(
                crate::proto::ardupilotmega::GimbalReport::default(),
            )),
            201 => Ok(MavMessage::GimbalControl(
                crate::proto::ardupilotmega::GimbalControl::default(),
            )),
            214 => Ok(MavMessage::GimbalTorqueCmdReport(
                crate::proto::ardupilotmega::GimbalTorqueCmdReport::default(),
            )),
            215 => Ok(MavMessage::GoproHeartbeat(
                crate::proto::ardupilotmega::GoproHeartbeat::default(),
            )),
            216 => Ok(MavMessage::GoproGetRequest(
                crate::proto::ardupilotmega::GoproGetRequest::default(),
            )),
            217 => Ok(MavMessage::GoproGetResponse(
                crate::proto::ardupilotmega::GoproGetResponse::default(),
            )),
            218 => Ok(MavMessage::GoproSetRequest(
                crate::proto::ardupilotmega::GoproSetRequest::default(),
            )),
            219 => Ok(MavMessage::GoproSetResponse(
                crate::proto::ardupilotmega::GoproSetResponse::default(),
            )),
            225 => Ok(MavMessage::EfiStatus(
                crate::proto::ardupilotmega::EfiStatus::default(),
            )),
            226 => Ok(MavMessage::Rpm(crate::proto::ardupilotmega::Rpm::default())),
            11000 => Ok(MavMessage::DeviceOpRead(
                crate::proto::ardupilotmega::DeviceOpRead::default(),
            )),
            11001 => Ok(MavMessage::DeviceOpReadReply(
                crate::proto::ardupilotmega::DeviceOpReadReply::default(),
            )),
            11002 => Ok(MavMessage::DeviceOpWrite(
                crate::proto::ardupilotmega::DeviceOpWrite::default(),
            )),
            11003 => Ok(MavMessage::DeviceOpWriteReply(
                crate::proto::ardupilotmega::DeviceOpWriteReply::default(),
            )),
            11010 => Ok(MavMessage::AdapTuning(
                crate::proto::ardupilotmega::AdapTuning::default(),
            )),
            11011 => Ok(MavMessage::VisionPositionDelta(
                crate::proto::ardupilotmega::VisionPositionDelta::default(),
            )),
            11020 => Ok(MavMessage::AoaSsa(
                crate::proto::ardupilotmega::AoaSsa::default(),
            )),
            11030 => Ok(MavMessage::EscTelemetry1To4(
                crate::proto::ardupilotmega::EscTelemetry1To4::default(),
            )),
            11031 => Ok(MavMessage::EscTelemetry5To8(
                crate::proto::ardupilotmega::EscTelemetry5To8::default(),
            )),
            11032 => Ok(MavMessage::EscTelemetry9To12(
                crate::proto::ardupilotmega::EscTelemetry9To12::default(),
            )),
            _ => {
                if let Ok(msg) = crate::mavlink::common::MavMessage::default_message_from_id(id) {
                    return Ok(MavMessage::Common(msg));
                }
                if let Ok(msg) = crate::mavlink::uavionix::MavMessage::default_message_from_id(id) {
                    return Ok(MavMessage::Uavionix(msg));
                }
                if let Ok(msg) = crate::mavlink::icarous::MavMessage::default_message_from_id(id) {
                    return Ok(MavMessage::Icarous(msg));
                }
                Err("Invalid message id.")
            }
        }
    }
    fn mavlink_ser(&self) -> Vec<u8> {
        match *self {
            MavMessage::SensorOffsets(ref body) => body.mavlink_ser(),
            MavMessage::SetMagOffsets(ref body) => body.mavlink_ser(),
            MavMessage::Meminfo(ref body) => body.mavlink_ser(),
            MavMessage::ApAdc(ref body) => body.mavlink_ser(),
            MavMessage::DigicamConfigure(ref body) => body.mavlink_ser(),
            MavMessage::DigicamControl(ref body) => body.mavlink_ser(),
            MavMessage::MountConfigure(ref body) => body.mavlink_ser(),
            MavMessage::MountControl(ref body) => body.mavlink_ser(),
            MavMessage::MountStatus(ref body) => body.mavlink_ser(),
            MavMessage::FencePoint(ref body) => body.mavlink_ser(),
            MavMessage::FenceFetchPoint(ref body) => body.mavlink_ser(),
            MavMessage::Ahrs(ref body) => body.mavlink_ser(),
            MavMessage::Simstate(ref body) => body.mavlink_ser(),
            MavMessage::Hwstatus(ref body) => body.mavlink_ser(),
            MavMessage::Radio(ref body) => body.mavlink_ser(),
            MavMessage::LimitsStatus(ref body) => body.mavlink_ser(),
            MavMessage::Wind(ref body) => body.mavlink_ser(),
            MavMessage::Data16(ref body) => body.mavlink_ser(),
            MavMessage::Data32(ref body) => body.mavlink_ser(),
            MavMessage::Data64(ref body) => body.mavlink_ser(),
            MavMessage::Data96(ref body) => body.mavlink_ser(),
            MavMessage::Rangefinder(ref body) => body.mavlink_ser(),
            MavMessage::AirspeedAutocal(ref body) => body.mavlink_ser(),
            MavMessage::RallyPoint(ref body) => body.mavlink_ser(),
            MavMessage::RallyFetchPoint(ref body) => body.mavlink_ser(),
            MavMessage::CompassmotStatus(ref body) => body.mavlink_ser(),
            MavMessage::Ahrs2(ref body) => body.mavlink_ser(),
            MavMessage::CameraStatus(ref body) => body.mavlink_ser(),
            MavMessage::CameraFeedback(ref body) => body.mavlink_ser(),
            MavMessage::Battery2(ref body) => body.mavlink_ser(),
            MavMessage::Ahrs3(ref body) => body.mavlink_ser(),
            MavMessage::AutopilotVersionRequest(ref body) => body.mavlink_ser(),
            MavMessage::RemoteLogDataBlock(ref body) => body.mavlink_ser(),
            MavMessage::RemoteLogBlockStatus(ref body) => body.mavlink_ser(),
            MavMessage::LedControl(ref body) => body.mavlink_ser(),
            MavMessage::MagCalProgress(ref body) => body.mavlink_ser(),
            MavMessage::MagCalReport(ref body) => body.mavlink_ser(),
            MavMessage::EkfStatusReport(ref body) => body.mavlink_ser(),
            MavMessage::PidTuning(ref body) => body.mavlink_ser(),
            MavMessage::Deepstall(ref body) => body.mavlink_ser(),
            MavMessage::GimbalReport(ref body) => body.mavlink_ser(),
            MavMessage::GimbalControl(ref body) => body.mavlink_ser(),
            MavMessage::GimbalTorqueCmdReport(ref body) => body.mavlink_ser(),
            MavMessage::GoproHeartbeat(ref body) => body.mavlink_ser(),
            MavMessage::GoproGetRequest(ref body) => body.mavlink_ser(),
            MavMessage::GoproGetResponse(ref body) => body.mavlink_ser(),
            MavMessage::GoproSetRequest(ref body) => body.mavlink_ser(),
            MavMessage::GoproSetResponse(ref body) => body.mavlink_ser(),
            MavMessage::EfiStatus(ref body) => body.mavlink_ser(),
            MavMessage::Rpm(ref body) => body.mavlink_ser(),
            MavMessage::DeviceOpRead(ref body) => body.mavlink_ser(),
            MavMessage::DeviceOpReadReply(ref body) => body.mavlink_ser(),
            MavMessage::DeviceOpWrite(ref body) => body.mavlink_ser(),
            MavMessage::DeviceOpWriteReply(ref body) => body.mavlink_ser(),
            MavMessage::AdapTuning(ref body) => body.mavlink_ser(),
            MavMessage::VisionPositionDelta(ref body) => body.mavlink_ser(),
            MavMessage::AoaSsa(ref body) => body.mavlink_ser(),
            MavMessage::EscTelemetry1To4(ref body) => body.mavlink_ser(),
            MavMessage::EscTelemetry5To8(ref body) => body.mavlink_ser(),
            MavMessage::EscTelemetry9To12(ref body) => body.mavlink_ser(),
            MavMessage::Common(ref msg) => msg.mavlink_ser(),
            MavMessage::Uavionix(ref msg) => msg.mavlink_ser(),
            MavMessage::Icarous(ref msg) => msg.mavlink_ser(),
        }
    }
    fn proto_encode(&self) -> Vec<u8> {
        match *self {
            MavMessage::SensorOffsets(ref body) => body.encode_to_vec(),
            MavMessage::SetMagOffsets(ref body) => body.encode_to_vec(),
            MavMessage::Meminfo(ref body) => body.encode_to_vec(),
            MavMessage::ApAdc(ref body) => body.encode_to_vec(),
            MavMessage::DigicamConfigure(ref body) => body.encode_to_vec(),
            MavMessage::DigicamControl(ref body) => body.encode_to_vec(),
            MavMessage::MountConfigure(ref body) => body.encode_to_vec(),
            MavMessage::MountControl(ref body) => body.encode_to_vec(),
            MavMessage::MountStatus(ref body) => body.encode_to_vec(),
            MavMessage::FencePoint(ref body) => body.encode_to_vec(),
            MavMessage::FenceFetchPoint(ref body) => body.encode_to_vec(),
            MavMessage::Ahrs(ref body) => body.encode_to_vec(),
            MavMessage::Simstate(ref body) => body.encode_to_vec(),
            MavMessage::Hwstatus(ref body) => body.encode_to_vec(),
            MavMessage::Radio(ref body) => body.encode_to_vec(),
            MavMessage::LimitsStatus(ref body) => body.encode_to_vec(),
            MavMessage::Wind(ref body) => body.encode_to_vec(),
            MavMessage::Data16(ref body) => body.encode_to_vec(),
            MavMessage::Data32(ref body) => body.encode_to_vec(),
            MavMessage::Data64(ref body) => body.encode_to_vec(),
            MavMessage::Data96(ref body) => body.encode_to_vec(),
            MavMessage::Rangefinder(ref body) => body.encode_to_vec(),
            MavMessage::AirspeedAutocal(ref body) => body.encode_to_vec(),
            MavMessage::RallyPoint(ref body) => body.encode_to_vec(),
            MavMessage::RallyFetchPoint(ref body) => body.encode_to_vec(),
            MavMessage::CompassmotStatus(ref body) => body.encode_to_vec(),
            MavMessage::Ahrs2(ref body) => body.encode_to_vec(),
            MavMessage::CameraStatus(ref body) => body.encode_to_vec(),
            MavMessage::CameraFeedback(ref body) => body.encode_to_vec(),
            MavMessage::Battery2(ref body) => body.encode_to_vec(),
            MavMessage::Ahrs3(ref body) => body.encode_to_vec(),
            MavMessage::AutopilotVersionRequest(ref body) => body.encode_to_vec(),
            MavMessage::RemoteLogDataBlock(ref body) => body.encode_to_vec(),
            MavMessage::RemoteLogBlockStatus(ref body) => body.encode_to_vec(),
            MavMessage::LedControl(ref body) => body.encode_to_vec(),
            MavMessage::MagCalProgress(ref body) => body.encode_to_vec(),
            MavMessage::MagCalReport(ref body) => body.encode_to_vec(),
            MavMessage::EkfStatusReport(ref body) => body.encode_to_vec(),
            MavMessage::PidTuning(ref body) => body.encode_to_vec(),
            MavMessage::Deepstall(ref body) => body.encode_to_vec(),
            MavMessage::GimbalReport(ref body) => body.encode_to_vec(),
            MavMessage::GimbalControl(ref body) => body.encode_to_vec(),
            MavMessage::GimbalTorqueCmdReport(ref body) => body.encode_to_vec(),
            MavMessage::GoproHeartbeat(ref body) => body.encode_to_vec(),
            MavMessage::GoproGetRequest(ref body) => body.encode_to_vec(),
            MavMessage::GoproGetResponse(ref body) => body.encode_to_vec(),
            MavMessage::GoproSetRequest(ref body) => body.encode_to_vec(),
            MavMessage::GoproSetResponse(ref body) => body.encode_to_vec(),
            MavMessage::EfiStatus(ref body) => body.encode_to_vec(),
            MavMessage::Rpm(ref body) => body.encode_to_vec(),
            MavMessage::DeviceOpRead(ref body) => body.encode_to_vec(),
            MavMessage::DeviceOpReadReply(ref body) => body.encode_to_vec(),
            MavMessage::DeviceOpWrite(ref body) => body.encode_to_vec(),
            MavMessage::DeviceOpWriteReply(ref body) => body.encode_to_vec(),
            MavMessage::AdapTuning(ref body) => body.encode_to_vec(),
            MavMessage::VisionPositionDelta(ref body) => body.encode_to_vec(),
            MavMessage::AoaSsa(ref body) => body.encode_to_vec(),
            MavMessage::EscTelemetry1To4(ref body) => body.encode_to_vec(),
            MavMessage::EscTelemetry5To8(ref body) => body.encode_to_vec(),
            MavMessage::EscTelemetry9To12(ref body) => body.encode_to_vec(),
            MavMessage::Common(ref msg) => msg.proto_encode(),
            MavMessage::Uavionix(ref msg) => msg.proto_encode(),
            MavMessage::Icarous(ref msg) => msg.proto_encode(),
        }
    }
    fn extra_crc(id: u32) -> u8 {
        match id {
            150 => 134,
            151 => 219,
            152 => 208,
            153 => 188,
            154 => 84,
            155 => 22,
            156 => 19,
            157 => 21,
            158 => 134,
            160 => 78,
            161 => 68,
            163 => 127,
            164 => 154,
            165 => 21,
            166 => 21,
            167 => 144,
            168 => 1,
            169 => 234,
            170 => 73,
            171 => 181,
            172 => 22,
            173 => 83,
            174 => 167,
            175 => 138,
            176 => 234,
            177 => 240,
            178 => 47,
            179 => 189,
            180 => 52,
            181 => 174,
            182 => 229,
            183 => 85,
            184 => 159,
            185 => 186,
            186 => 72,
            191 => 92,
            192 => 36,
            193 => 71,
            194 => 98,
            195 => 120,
            200 => 134,
            201 => 205,
            214 => 69,
            215 => 101,
            216 => 50,
            217 => 202,
            218 => 17,
            219 => 162,
            225 => 208,
            226 => 207,
            11000 => 134,
            11001 => 15,
            11002 => 234,
            11003 => 64,
            11010 => 46,
            11011 => 106,
            11020 => 205,
            11030 => 144,
            11031 => 133,
            11032 => 85,
            _ => {
                match crate::mavlink::common::MavMessage::extra_crc(id) {
                    0 => {}
                    any => return any,
                }
                match crate::mavlink::uavionix::MavMessage::extra_crc(id) {
                    0 => {}
                    any => return any,
                }
                match crate::mavlink::icarous::MavMessage::extra_crc(id) {
                    0 => {}
                    any => return any,
                }
                0
            }
        }
    }
}
