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
impl crate::proto::slugs::CpuLoad {
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
            _struct.bat_volt = buf.get_u16_le() as u32;
            _struct.sens_load = buf.get_u8() as u32;
            _struct.ctrl_load = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.bat_volt as u16);
        _tmp.put_u8(self.sens_load as u8);
        _tmp.put_u8(self.ctrl_load as u8);
        _tmp
    }
}
impl crate::proto::slugs::SensorBias {
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
            _struct.ax_bias = buf.get_f32_le();
            _struct.ay_bias = buf.get_f32_le();
            _struct.az_bias = buf.get_f32_le();
            _struct.gx_bias = buf.get_f32_le();
            _struct.gy_bias = buf.get_f32_le();
            _struct.gz_bias = buf.get_f32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.ax_bias as f32);
        _tmp.put_f32_le(self.ay_bias as f32);
        _tmp.put_f32_le(self.az_bias as f32);
        _tmp.put_f32_le(self.gx_bias as f32);
        _tmp.put_f32_le(self.gy_bias as f32);
        _tmp.put_f32_le(self.gz_bias as f32);
        _tmp
    }
}
impl crate::proto::slugs::Diagnostic {
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
            _struct.diag_fl1 = buf.get_f32_le();
            _struct.diag_fl2 = buf.get_f32_le();
            _struct.diag_fl3 = buf.get_f32_le();
            _struct.diag_sh1 = buf.get_i16_le() as i32;
            _struct.diag_sh2 = buf.get_i16_le() as i32;
            _struct.diag_sh3 = buf.get_i16_le() as i32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.diag_fl1 as f32);
        _tmp.put_f32_le(self.diag_fl2 as f32);
        _tmp.put_f32_le(self.diag_fl3 as f32);
        _tmp.put_i16_le(self.diag_sh1 as i16);
        _tmp.put_i16_le(self.diag_sh2 as i16);
        _tmp.put_i16_le(self.diag_sh3 as i16);
        _tmp
    }
}
impl crate::proto::slugs::SlugsNavigation {
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
            _struct.u_m = buf.get_f32_le();
            _struct.phi_c = buf.get_f32_le();
            _struct.theta_c = buf.get_f32_le();
            _struct.psi_dot_c = buf.get_f32_le();
            _struct.ay_body = buf.get_f32_le();
            _struct.total_dist = buf.get_f32_le();
            _struct.dist2_go = buf.get_f32_le();
            _struct.h_c = buf.get_u16_le() as u32;
            _struct.from_wp = buf.get_u8() as u32;
            _struct.to_wp = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.u_m as f32);
        _tmp.put_f32_le(self.phi_c as f32);
        _tmp.put_f32_le(self.theta_c as f32);
        _tmp.put_f32_le(self.psi_dot_c as f32);
        _tmp.put_f32_le(self.ay_body as f32);
        _tmp.put_f32_le(self.total_dist as f32);
        _tmp.put_f32_le(self.dist2_go as f32);
        _tmp.put_u16_le(self.h_c as u16);
        _tmp.put_u8(self.from_wp as u8);
        _tmp.put_u8(self.to_wp as u8);
        _tmp
    }
}
impl crate::proto::slugs::DataLog {
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
            _struct.fl_1 = buf.get_f32_le();
            _struct.fl_2 = buf.get_f32_le();
            _struct.fl_3 = buf.get_f32_le();
            _struct.fl_4 = buf.get_f32_le();
            _struct.fl_5 = buf.get_f32_le();
            _struct.fl_6 = buf.get_f32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.fl_1 as f32);
        _tmp.put_f32_le(self.fl_2 as f32);
        _tmp.put_f32_le(self.fl_3 as f32);
        _tmp.put_f32_le(self.fl_4 as f32);
        _tmp.put_f32_le(self.fl_5 as f32);
        _tmp.put_f32_le(self.fl_6 as f32);
        _tmp
    }
}
impl crate::proto::slugs::GpsDateTime {
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
            _struct.year = buf.get_u8() as u32;
            _struct.month = buf.get_u8() as u32;
            _struct.day = buf.get_u8() as u32;
            _struct.hour = buf.get_u8() as u32;
            _struct.min = buf.get_u8() as u32;
            _struct.sec = buf.get_u8() as u32;
            _struct.clock_stat = buf.get_u8() as u32;
            _struct.vis_sat = buf.get_u8() as u32;
            _struct.use_sat = buf.get_u8() as u32;
            _struct.gpp_gl = buf.get_u8() as u32;
            _struct.sig_used_mask = buf.get_u8() as u32;
            _struct.percent_used = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.year as u8);
        _tmp.put_u8(self.month as u8);
        _tmp.put_u8(self.day as u8);
        _tmp.put_u8(self.hour as u8);
        _tmp.put_u8(self.min as u8);
        _tmp.put_u8(self.sec as u8);
        _tmp.put_u8(self.clock_stat as u8);
        _tmp.put_u8(self.vis_sat as u8);
        _tmp.put_u8(self.use_sat as u8);
        _tmp.put_u8(self.gpp_gl as u8);
        _tmp.put_u8(self.sig_used_mask as u8);
        _tmp.put_u8(self.percent_used as u8);
        _tmp
    }
}
impl crate::proto::slugs::MidLvlCmds {
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
            _struct.h_command = buf.get_f32_le();
            _struct.u_command = buf.get_f32_le();
            _struct.r_command = buf.get_f32_le();
            _struct.target = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.h_command as f32);
        _tmp.put_f32_le(self.u_command as f32);
        _tmp.put_f32_le(self.r_command as f32);
        _tmp.put_u8(self.target as u8);
        _tmp
    }
}
impl crate::proto::slugs::CtrlSrfcPt {
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
            let tmp = buf.get_u16_le();
            _struct.bitfield_pt = FromPrimitive::from_u16(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "ControlSurfaceFlag".to_string(),
                value: tmp as u32,
            })?;
            _struct.target = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.bitfield_pt as u16);
        _tmp.put_u8(self.target as u8);
        _tmp
    }
}
impl crate::proto::slugs::SlugsCameraOrder {
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
            _struct.target = buf.get_u8() as u32;
            _struct.pan = buf.get_i8() as i32;
            _struct.tilt = buf.get_i8() as i32;
            _struct.zoom = buf.get_i8() as i32;
            _struct.move_home = buf.get_i8() as i32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.target as u8);
        _tmp.put_i8(self.pan as i8);
        _tmp.put_i8(self.tilt as i8);
        _tmp.put_i8(self.zoom as i8);
        _tmp.put_i8(self.move_home as i8);
        _tmp
    }
}
impl crate::proto::slugs::ControlSurface {
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
            _struct.m_control = buf.get_f32_le();
            _struct.b_control = buf.get_f32_le();
            _struct.target = buf.get_u8() as u32;
            _struct.id_surface = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.m_control as f32);
        _tmp.put_f32_le(self.b_control as f32);
        _tmp.put_u8(self.target as u8);
        _tmp.put_u8(self.id_surface as u8);
        _tmp
    }
}
impl crate::proto::slugs::SlugsMobileLocation {
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
            _struct.latitude = buf.get_f32_le();
            _struct.longitude = buf.get_f32_le();
            _struct.target = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.latitude as f32);
        _tmp.put_f32_le(self.longitude as f32);
        _tmp.put_u8(self.target as u8);
        _tmp
    }
}
impl crate::proto::slugs::SlugsConfigurationCamera {
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
            _struct.target = buf.get_u8() as u32;
            _struct.id_order = buf.get_u8() as u32;
            _struct.order = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.target as u8);
        _tmp.put_u8(self.id_order as u8);
        _tmp.put_u8(self.order as u8);
        _tmp
    }
}
impl crate::proto::slugs::IsrLocation {
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
            _struct.latitude = buf.get_f32_le();
            _struct.longitude = buf.get_f32_le();
            _struct.height = buf.get_f32_le();
            _struct.target = buf.get_u8() as u32;
            _struct.option1 = buf.get_u8() as u32;
            _struct.option2 = buf.get_u8() as u32;
            _struct.option3 = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.latitude as f32);
        _tmp.put_f32_le(self.longitude as f32);
        _tmp.put_f32_le(self.height as f32);
        _tmp.put_u8(self.target as u8);
        _tmp.put_u8(self.option1 as u8);
        _tmp.put_u8(self.option2 as u8);
        _tmp.put_u8(self.option3 as u8);
        _tmp
    }
}
impl crate::proto::slugs::VoltSensor {
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
            _struct.voltage = buf.get_u16_le() as u32;
            _struct.reading2 = buf.get_u16_le() as u32;
            _struct.r2_type = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.voltage as u16);
        _tmp.put_u16_le(self.reading2 as u16);
        _tmp.put_u8(self.r2_type as u8);
        _tmp
    }
}
impl crate::proto::slugs::PtzStatus {
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
            _struct.pan = buf.get_i16_le() as i32;
            _struct.tilt = buf.get_i16_le() as i32;
            _struct.zoom = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i16_le(self.pan as i16);
        _tmp.put_i16_le(self.tilt as i16);
        _tmp.put_u8(self.zoom as u8);
        _tmp
    }
}
impl crate::proto::slugs::UavStatus {
    pub const ENCODED_LEN: usize = 21usize;
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
            _struct.latitude = buf.get_f32_le();
            _struct.longitude = buf.get_f32_le();
            _struct.altitude = buf.get_f32_le();
            _struct.speed = buf.get_f32_le();
            _struct.course = buf.get_f32_le();
            _struct.target = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.latitude as f32);
        _tmp.put_f32_le(self.longitude as f32);
        _tmp.put_f32_le(self.altitude as f32);
        _tmp.put_f32_le(self.speed as f32);
        _tmp.put_f32_le(self.course as f32);
        _tmp.put_u8(self.target as u8);
        _tmp
    }
}
impl crate::proto::slugs::StatusGps {
    pub const ENCODED_LEN: usize = 11usize;
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
            _struct.mag_var = buf.get_f32_le();
            _struct.cs_fails = buf.get_u16_le() as u32;
            _struct.gps_quality = buf.get_u8() as u32;
            _struct.msgs_type = buf.get_u8() as u32;
            _struct.pos_status = buf.get_u8() as u32;
            _struct.mag_dir = buf.get_i8() as i32;
            _struct.mode_ind = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.mag_var as f32);
        _tmp.put_u16_le(self.cs_fails as u16);
        _tmp.put_u8(self.gps_quality as u8);
        _tmp.put_u8(self.msgs_type as u8);
        _tmp.put_u8(self.pos_status as u8);
        _tmp.put_i8(self.mag_dir as i8);
        _tmp.put_u8(self.mode_ind as u8);
        _tmp
    }
}
impl crate::proto::slugs::NovatelDiag {
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
            _struct.receiver_status = buf.get_u32_le();
            _struct.pos_sol_age = buf.get_f32_le();
            _struct.cs_fails = buf.get_u16_le() as u32;
            _struct.time_status = buf.get_u8() as u32;
            _struct.sol_status = buf.get_u8() as u32;
            _struct.pos_type = buf.get_u8() as u32;
            _struct.vel_type = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.receiver_status as u32);
        _tmp.put_f32_le(self.pos_sol_age as f32);
        _tmp.put_u16_le(self.cs_fails as u16);
        _tmp.put_u8(self.time_status as u8);
        _tmp.put_u8(self.sol_status as u8);
        _tmp.put_u8(self.pos_type as u8);
        _tmp.put_u8(self.vel_type as u8);
        _tmp
    }
}
impl crate::proto::slugs::SensorDiag {
    pub const ENCODED_LEN: usize = 11usize;
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
            _struct.float1 = buf.get_f32_le();
            _struct.float2 = buf.get_f32_le();
            _struct.int1 = buf.get_i16_le() as i32;
            _struct.char1 = buf.get_i8() as i32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.float1 as f32);
        _tmp.put_f32_le(self.float2 as f32);
        _tmp.put_i16_le(self.int1 as i16);
        _tmp.put_i8(self.char1 as i8);
        _tmp
    }
}
impl crate::proto::slugs::Boot {
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
            _struct.version = buf.get_u32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.version as u32);
        _tmp
    }
}
#[derive(Clone, PartialEq, Debug)]
pub enum MavMessage {
    CpuLoad(crate::proto::slugs::CpuLoad),
    SensorBias(crate::proto::slugs::SensorBias),
    Diagnostic(crate::proto::slugs::Diagnostic),
    SlugsNavigation(crate::proto::slugs::SlugsNavigation),
    DataLog(crate::proto::slugs::DataLog),
    GpsDateTime(crate::proto::slugs::GpsDateTime),
    MidLvlCmds(crate::proto::slugs::MidLvlCmds),
    CtrlSrfcPt(crate::proto::slugs::CtrlSrfcPt),
    SlugsCameraOrder(crate::proto::slugs::SlugsCameraOrder),
    ControlSurface(crate::proto::slugs::ControlSurface),
    SlugsMobileLocation(crate::proto::slugs::SlugsMobileLocation),
    SlugsConfigurationCamera(crate::proto::slugs::SlugsConfigurationCamera),
    IsrLocation(crate::proto::slugs::IsrLocation),
    VoltSensor(crate::proto::slugs::VoltSensor),
    PtzStatus(crate::proto::slugs::PtzStatus),
    UavStatus(crate::proto::slugs::UavStatus),
    StatusGps(crate::proto::slugs::StatusGps),
    NovatelDiag(crate::proto::slugs::NovatelDiag),
    SensorDiag(crate::proto::slugs::SensorDiag),
    Boot(crate::proto::slugs::Boot),
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
            170 => crate::proto::slugs::CpuLoad::mavlink_deser(version, payload)
                .map(MavMessage::CpuLoad),
            172 => crate::proto::slugs::SensorBias::mavlink_deser(version, payload)
                .map(MavMessage::SensorBias),
            173 => crate::proto::slugs::Diagnostic::mavlink_deser(version, payload)
                .map(MavMessage::Diagnostic),
            176 => crate::proto::slugs::SlugsNavigation::mavlink_deser(version, payload)
                .map(MavMessage::SlugsNavigation),
            177 => crate::proto::slugs::DataLog::mavlink_deser(version, payload)
                .map(MavMessage::DataLog),
            179 => crate::proto::slugs::GpsDateTime::mavlink_deser(version, payload)
                .map(MavMessage::GpsDateTime),
            180 => crate::proto::slugs::MidLvlCmds::mavlink_deser(version, payload)
                .map(MavMessage::MidLvlCmds),
            181 => crate::proto::slugs::CtrlSrfcPt::mavlink_deser(version, payload)
                .map(MavMessage::CtrlSrfcPt),
            184 => crate::proto::slugs::SlugsCameraOrder::mavlink_deser(version, payload)
                .map(MavMessage::SlugsCameraOrder),
            185 => crate::proto::slugs::ControlSurface::mavlink_deser(version, payload)
                .map(MavMessage::ControlSurface),
            186 => crate::proto::slugs::SlugsMobileLocation::mavlink_deser(version, payload)
                .map(MavMessage::SlugsMobileLocation),
            188 => crate::proto::slugs::SlugsConfigurationCamera::mavlink_deser(version, payload)
                .map(MavMessage::SlugsConfigurationCamera),
            189 => crate::proto::slugs::IsrLocation::mavlink_deser(version, payload)
                .map(MavMessage::IsrLocation),
            191 => crate::proto::slugs::VoltSensor::mavlink_deser(version, payload)
                .map(MavMessage::VoltSensor),
            192 => crate::proto::slugs::PtzStatus::mavlink_deser(version, payload)
                .map(MavMessage::PtzStatus),
            193 => crate::proto::slugs::UavStatus::mavlink_deser(version, payload)
                .map(MavMessage::UavStatus),
            194 => crate::proto::slugs::StatusGps::mavlink_deser(version, payload)
                .map(MavMessage::StatusGps),
            195 => crate::proto::slugs::NovatelDiag::mavlink_deser(version, payload)
                .map(MavMessage::NovatelDiag),
            196 => crate::proto::slugs::SensorDiag::mavlink_deser(version, payload)
                .map(MavMessage::SensorDiag),
            197 => crate::proto::slugs::Boot::mavlink_deser(version, payload).map(MavMessage::Boot),
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
            MavMessage::CpuLoad(..) => "CpuLoad",
            MavMessage::SensorBias(..) => "SensorBias",
            MavMessage::Diagnostic(..) => "Diagnostic",
            MavMessage::SlugsNavigation(..) => "SlugsNavigation",
            MavMessage::DataLog(..) => "DataLog",
            MavMessage::GpsDateTime(..) => "GpsDateTime",
            MavMessage::MidLvlCmds(..) => "MidLvlCmds",
            MavMessage::CtrlSrfcPt(..) => "CtrlSrfcPt",
            MavMessage::SlugsCameraOrder(..) => "SlugsCameraOrder",
            MavMessage::ControlSurface(..) => "ControlSurface",
            MavMessage::SlugsMobileLocation(..) => "SlugsMobileLocation",
            MavMessage::SlugsConfigurationCamera(..) => "SlugsConfigurationCamera",
            MavMessage::IsrLocation(..) => "IsrLocation",
            MavMessage::VoltSensor(..) => "VoltSensor",
            MavMessage::PtzStatus(..) => "PtzStatus",
            MavMessage::UavStatus(..) => "UavStatus",
            MavMessage::StatusGps(..) => "StatusGps",
            MavMessage::NovatelDiag(..) => "NovatelDiag",
            MavMessage::SensorDiag(..) => "SensorDiag",
            MavMessage::Boot(..) => "Boot",
            MavMessage::Common(msg) => msg.message_name(),
        }
    }
    fn message_id(&self) -> u32 {
        match self {
            MavMessage::CpuLoad(..) => 170,
            MavMessage::SensorBias(..) => 172,
            MavMessage::Diagnostic(..) => 173,
            MavMessage::SlugsNavigation(..) => 176,
            MavMessage::DataLog(..) => 177,
            MavMessage::GpsDateTime(..) => 179,
            MavMessage::MidLvlCmds(..) => 180,
            MavMessage::CtrlSrfcPt(..) => 181,
            MavMessage::SlugsCameraOrder(..) => 184,
            MavMessage::ControlSurface(..) => 185,
            MavMessage::SlugsMobileLocation(..) => 186,
            MavMessage::SlugsConfigurationCamera(..) => 188,
            MavMessage::IsrLocation(..) => 189,
            MavMessage::VoltSensor(..) => 191,
            MavMessage::PtzStatus(..) => 192,
            MavMessage::UavStatus(..) => 193,
            MavMessage::StatusGps(..) => 194,
            MavMessage::NovatelDiag(..) => 195,
            MavMessage::SensorDiag(..) => 196,
            MavMessage::Boot(..) => 197,
            MavMessage::Common(msg) => msg.message_id(),
        }
    }
    fn message_id_from_name(name: &str) -> Result<u32, &'static str> {
        match name {
            "CpuLoad" => Ok(170),
            "SensorBias" => Ok(172),
            "Diagnostic" => Ok(173),
            "SlugsNavigation" => Ok(176),
            "DataLog" => Ok(177),
            "GpsDateTime" => Ok(179),
            "MidLvlCmds" => Ok(180),
            "CtrlSrfcPt" => Ok(181),
            "SlugsCameraOrder" => Ok(184),
            "ControlSurface" => Ok(185),
            "SlugsMobileLocation" => Ok(186),
            "SlugsConfigurationCamera" => Ok(188),
            "IsrLocation" => Ok(189),
            "VoltSensor" => Ok(191),
            "PtzStatus" => Ok(192),
            "UavStatus" => Ok(193),
            "StatusGps" => Ok(194),
            "NovatelDiag" => Ok(195),
            "SensorDiag" => Ok(196),
            "Boot" => Ok(197),
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
            170 => Ok(MavMessage::CpuLoad(crate::proto::slugs::CpuLoad::default())),
            172 => Ok(MavMessage::SensorBias(
                crate::proto::slugs::SensorBias::default(),
            )),
            173 => Ok(MavMessage::Diagnostic(
                crate::proto::slugs::Diagnostic::default(),
            )),
            176 => Ok(MavMessage::SlugsNavigation(
                crate::proto::slugs::SlugsNavigation::default(),
            )),
            177 => Ok(MavMessage::DataLog(crate::proto::slugs::DataLog::default())),
            179 => Ok(MavMessage::GpsDateTime(
                crate::proto::slugs::GpsDateTime::default(),
            )),
            180 => Ok(MavMessage::MidLvlCmds(
                crate::proto::slugs::MidLvlCmds::default(),
            )),
            181 => Ok(MavMessage::CtrlSrfcPt(
                crate::proto::slugs::CtrlSrfcPt::default(),
            )),
            184 => Ok(MavMessage::SlugsCameraOrder(
                crate::proto::slugs::SlugsCameraOrder::default(),
            )),
            185 => Ok(MavMessage::ControlSurface(
                crate::proto::slugs::ControlSurface::default(),
            )),
            186 => Ok(MavMessage::SlugsMobileLocation(
                crate::proto::slugs::SlugsMobileLocation::default(),
            )),
            188 => Ok(MavMessage::SlugsConfigurationCamera(
                crate::proto::slugs::SlugsConfigurationCamera::default(),
            )),
            189 => Ok(MavMessage::IsrLocation(
                crate::proto::slugs::IsrLocation::default(),
            )),
            191 => Ok(MavMessage::VoltSensor(
                crate::proto::slugs::VoltSensor::default(),
            )),
            192 => Ok(MavMessage::PtzStatus(
                crate::proto::slugs::PtzStatus::default(),
            )),
            193 => Ok(MavMessage::UavStatus(
                crate::proto::slugs::UavStatus::default(),
            )),
            194 => Ok(MavMessage::StatusGps(
                crate::proto::slugs::StatusGps::default(),
            )),
            195 => Ok(MavMessage::NovatelDiag(
                crate::proto::slugs::NovatelDiag::default(),
            )),
            196 => Ok(MavMessage::SensorDiag(
                crate::proto::slugs::SensorDiag::default(),
            )),
            197 => Ok(MavMessage::Boot(crate::proto::slugs::Boot::default())),
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
            MavMessage::CpuLoad(ref body) => body.mavlink_ser(),
            MavMessage::SensorBias(ref body) => body.mavlink_ser(),
            MavMessage::Diagnostic(ref body) => body.mavlink_ser(),
            MavMessage::SlugsNavigation(ref body) => body.mavlink_ser(),
            MavMessage::DataLog(ref body) => body.mavlink_ser(),
            MavMessage::GpsDateTime(ref body) => body.mavlink_ser(),
            MavMessage::MidLvlCmds(ref body) => body.mavlink_ser(),
            MavMessage::CtrlSrfcPt(ref body) => body.mavlink_ser(),
            MavMessage::SlugsCameraOrder(ref body) => body.mavlink_ser(),
            MavMessage::ControlSurface(ref body) => body.mavlink_ser(),
            MavMessage::SlugsMobileLocation(ref body) => body.mavlink_ser(),
            MavMessage::SlugsConfigurationCamera(ref body) => body.mavlink_ser(),
            MavMessage::IsrLocation(ref body) => body.mavlink_ser(),
            MavMessage::VoltSensor(ref body) => body.mavlink_ser(),
            MavMessage::PtzStatus(ref body) => body.mavlink_ser(),
            MavMessage::UavStatus(ref body) => body.mavlink_ser(),
            MavMessage::StatusGps(ref body) => body.mavlink_ser(),
            MavMessage::NovatelDiag(ref body) => body.mavlink_ser(),
            MavMessage::SensorDiag(ref body) => body.mavlink_ser(),
            MavMessage::Boot(ref body) => body.mavlink_ser(),
            MavMessage::Common(ref msg) => msg.mavlink_ser(),
        }
    }
    fn proto_encode(&self) -> Vec<u8> {
        match *self {
            MavMessage::CpuLoad(ref body) => body.encode_to_vec(),
            MavMessage::SensorBias(ref body) => body.encode_to_vec(),
            MavMessage::Diagnostic(ref body) => body.encode_to_vec(),
            MavMessage::SlugsNavigation(ref body) => body.encode_to_vec(),
            MavMessage::DataLog(ref body) => body.encode_to_vec(),
            MavMessage::GpsDateTime(ref body) => body.encode_to_vec(),
            MavMessage::MidLvlCmds(ref body) => body.encode_to_vec(),
            MavMessage::CtrlSrfcPt(ref body) => body.encode_to_vec(),
            MavMessage::SlugsCameraOrder(ref body) => body.encode_to_vec(),
            MavMessage::ControlSurface(ref body) => body.encode_to_vec(),
            MavMessage::SlugsMobileLocation(ref body) => body.encode_to_vec(),
            MavMessage::SlugsConfigurationCamera(ref body) => body.encode_to_vec(),
            MavMessage::IsrLocation(ref body) => body.encode_to_vec(),
            MavMessage::VoltSensor(ref body) => body.encode_to_vec(),
            MavMessage::PtzStatus(ref body) => body.encode_to_vec(),
            MavMessage::UavStatus(ref body) => body.encode_to_vec(),
            MavMessage::StatusGps(ref body) => body.encode_to_vec(),
            MavMessage::NovatelDiag(ref body) => body.encode_to_vec(),
            MavMessage::SensorDiag(ref body) => body.encode_to_vec(),
            MavMessage::Boot(ref body) => body.encode_to_vec(),
            MavMessage::Common(ref msg) => msg.proto_encode(),
        }
    }
    fn extra_crc(id: u32) -> u8 {
        match id {
            170 => 75,
            172 => 168,
            173 => 2,
            176 => 228,
            177 => 167,
            179 => 132,
            180 => 146,
            181 => 104,
            184 => 45,
            185 => 113,
            186 => 101,
            188 => 5,
            189 => 246,
            191 => 17,
            192 => 187,
            193 => 160,
            194 => 51,
            195 => 59,
            196 => 129,
            197 => 39,
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
