// This file was automatically generated, do not edit
#[allow(unused_imports)]
use crate::{};
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
impl crate::proto::common::Heartbeat {
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
            _struct.custom_mode = buf.get_u32_le();
            let tmp = buf.get_u8();
            _struct.r#type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavType".to_string(),
                value: tmp as u32,
            })?;
            let tmp = buf.get_u8();
            _struct.autopilot = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavAutopilot".to_string(),
                value: tmp as u32,
            })?;
            let tmp = buf.get_u8();
            _struct.base_mode = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavModeFlag".to_string(),
                value: tmp as u32,
            })?;
            let tmp = buf.get_u8();
            _struct.system_status =
                FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "MavState".to_string(),
                    value: tmp as u32,
                })?;
            _struct.mavlink_version = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.custom_mode as u32);
        _tmp.put_u8(self.r#type as u8);
        _tmp.put_u8(self.autopilot as u8);
        _tmp.put_u8(self.base_mode as u8);
        _tmp.put_u8(self.system_status as u8);
        _tmp.put_u8(self.mavlink_version as u8);
        _tmp
    }
}
impl crate::proto::common::SysStatus {
    pub const ENCODED_LEN: usize = 31usize;
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
            _struct.onboard_control_sensors_present =
                FromPrimitive::from_u32(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "MavSysStatusSensor".to_string(),
                    value: tmp as u32,
                })?;
            let tmp = buf.get_u32_le();
            _struct.onboard_control_sensors_enabled =
                FromPrimitive::from_u32(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "MavSysStatusSensor".to_string(),
                    value: tmp as u32,
                })?;
            let tmp = buf.get_u32_le();
            _struct.onboard_control_sensors_health =
                FromPrimitive::from_u32(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "MavSysStatusSensor".to_string(),
                    value: tmp as u32,
                })?;
            _struct.load = buf.get_u16_le() as u32;
            _struct.voltage_battery = buf.get_u16_le() as u32;
            _struct.current_battery = buf.get_i16_le() as i32;
            _struct.drop_rate_comm = buf.get_u16_le() as u32;
            _struct.errors_comm = buf.get_u16_le() as u32;
            _struct.errors_count1 = buf.get_u16_le() as u32;
            _struct.errors_count2 = buf.get_u16_le() as u32;
            _struct.errors_count3 = buf.get_u16_le() as u32;
            _struct.errors_count4 = buf.get_u16_le() as u32;
            _struct.battery_remaining = buf.get_i8() as i32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.onboard_control_sensors_present as u32);
        _tmp.put_u32_le(self.onboard_control_sensors_enabled as u32);
        _tmp.put_u32_le(self.onboard_control_sensors_health as u32);
        _tmp.put_u16_le(self.load as u16);
        _tmp.put_u16_le(self.voltage_battery as u16);
        _tmp.put_i16_le(self.current_battery as i16);
        _tmp.put_u16_le(self.drop_rate_comm as u16);
        _tmp.put_u16_le(self.errors_comm as u16);
        _tmp.put_u16_le(self.errors_count1 as u16);
        _tmp.put_u16_le(self.errors_count2 as u16);
        _tmp.put_u16_le(self.errors_count3 as u16);
        _tmp.put_u16_le(self.errors_count4 as u16);
        _tmp.put_i8(self.battery_remaining as i8);
        _tmp
    }
}
impl crate::proto::common::SystemTime {
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
            _struct.time_unix_usec = buf.get_u64_le();
            _struct.time_boot_ms = buf.get_u32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_unix_usec as u64);
        _tmp.put_u32_le(self.time_boot_ms as u32);
        _tmp
    }
}
impl crate::proto::common::Ping {
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
            _struct.time_usec = buf.get_u64_le();
            _struct.seq = buf.get_u32_le();
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        _tmp.put_u32_le(self.seq as u32);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp
    }
}
impl crate::proto::common::ChangeOperatorControl {
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
            _struct.target_system = buf.get_u8() as u32;
            _struct.control_request = buf.get_u8() as u32;
            _struct.version = buf.get_u8() as u32;
            let mut s = Vec::with_capacity(25usize);
            for _ in 0..25usize {
                s.push(buf.get_u8());
            }
            _struct.passkey = String::from_utf8_lossy(&s).into();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.control_request as u8);
        _tmp.put_u8(self.version as u8);
        for val in self.passkey.bytes() {
            _tmp.put_u8(val);
        }
        _tmp
    }
}
impl crate::proto::common::ChangeOperatorControlAck {
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
            _struct.gcs_system_id = buf.get_u8() as u32;
            _struct.control_request = buf.get_u8() as u32;
            _struct.ack = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.gcs_system_id as u8);
        _tmp.put_u8(self.control_request as u8);
        _tmp.put_u8(self.ack as u8);
        _tmp
    }
}
impl crate::proto::common::AuthKey {
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
            let mut s = Vec::with_capacity(32usize);
            for _ in 0..32usize {
                s.push(buf.get_u8());
            }
            _struct.key = String::from_utf8_lossy(&s).into();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        for val in self.key.bytes() {
            _tmp.put_u8(val);
        }
        _tmp
    }
}
impl crate::proto::common::LinkNodeStatus {
    pub const ENCODED_LEN: usize = 36usize;
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
            _struct.tx_rate = buf.get_u32_le();
            _struct.rx_rate = buf.get_u32_le();
            _struct.messages_sent = buf.get_u32_le();
            _struct.messages_received = buf.get_u32_le();
            _struct.messages_lost = buf.get_u32_le();
            _struct.rx_parse_err = buf.get_u16_le() as u32;
            _struct.tx_overflows = buf.get_u16_le() as u32;
            _struct.rx_overflows = buf.get_u16_le() as u32;
            _struct.tx_buf = buf.get_u8() as u32;
            _struct.rx_buf = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.timestamp as u64);
        _tmp.put_u32_le(self.tx_rate as u32);
        _tmp.put_u32_le(self.rx_rate as u32);
        _tmp.put_u32_le(self.messages_sent as u32);
        _tmp.put_u32_le(self.messages_received as u32);
        _tmp.put_u32_le(self.messages_lost as u32);
        _tmp.put_u16_le(self.rx_parse_err as u16);
        _tmp.put_u16_le(self.tx_overflows as u16);
        _tmp.put_u16_le(self.rx_overflows as u16);
        _tmp.put_u8(self.tx_buf as u8);
        _tmp.put_u8(self.rx_buf as u8);
        _tmp
    }
}
impl crate::proto::common::SetMode {
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
            _struct.custom_mode = buf.get_u32_le();
            _struct.target_system = buf.get_u8() as u32;
            let tmp = buf.get_u8();
            _struct.base_mode = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavMode".to_string(),
                value: tmp as u32,
            })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.custom_mode as u32);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.base_mode as u8);
        _tmp
    }
}
impl crate::proto::common::ParamRequestRead {
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
            _struct.param_index = buf.get_i16_le() as i32;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            let mut s = Vec::with_capacity(16usize);
            for _ in 0..16usize {
                s.push(buf.get_u8());
            }
            _struct.param_id = String::from_utf8_lossy(&s).into();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i16_le(self.param_index as i16);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        for val in self.param_id.bytes() {
            _tmp.put_u8(val);
        }
        _tmp
    }
}
impl crate::proto::common::ParamRequestList {
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
impl crate::proto::common::ParamValue {
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
            _struct.param_value = buf.get_f32_le();
            _struct.param_count = buf.get_u16_le() as u32;
            _struct.param_index = buf.get_u16_le() as u32;
            let mut s = Vec::with_capacity(16usize);
            for _ in 0..16usize {
                s.push(buf.get_u8());
            }
            _struct.param_id = String::from_utf8_lossy(&s).into();
            let tmp = buf.get_u8();
            _struct.param_type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavParamType".to_string(),
                value: tmp as u32,
            })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.param_value as f32);
        _tmp.put_u16_le(self.param_count as u16);
        _tmp.put_u16_le(self.param_index as u16);
        for val in self.param_id.bytes() {
            _tmp.put_u8(val);
        }
        _tmp.put_u8(self.param_type as u8);
        _tmp
    }
}
impl crate::proto::common::ParamSet {
    pub const ENCODED_LEN: usize = 23usize;
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
            _struct.param_value = buf.get_f32_le();
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            let mut s = Vec::with_capacity(16usize);
            for _ in 0..16usize {
                s.push(buf.get_u8());
            }
            _struct.param_id = String::from_utf8_lossy(&s).into();
            let tmp = buf.get_u8();
            _struct.param_type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavParamType".to_string(),
                value: tmp as u32,
            })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.param_value as f32);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        for val in self.param_id.bytes() {
            _tmp.put_u8(val);
        }
        _tmp.put_u8(self.param_type as u8);
        _tmp
    }
}
impl crate::proto::common::GpsRawInt {
    pub const ENCODED_LEN: usize = 30usize;
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
            _struct.lon = buf.get_i32_le();
            _struct.alt = buf.get_i32_le();
            _struct.eph = buf.get_u16_le() as u32;
            _struct.epv = buf.get_u16_le() as u32;
            _struct.vel = buf.get_u16_le() as u32;
            _struct.cog = buf.get_u16_le() as u32;
            let tmp = buf.get_u8();
            _struct.fix_type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "GpsFixType".to_string(),
                value: tmp as u32,
            })?;
            _struct.satellites_visible = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        _tmp.put_i32_le(self.lat as i32);
        _tmp.put_i32_le(self.lon as i32);
        _tmp.put_i32_le(self.alt as i32);
        _tmp.put_u16_le(self.eph as u16);
        _tmp.put_u16_le(self.epv as u16);
        _tmp.put_u16_le(self.vel as u16);
        _tmp.put_u16_le(self.cog as u16);
        _tmp.put_u8(self.fix_type as u8);
        _tmp.put_u8(self.satellites_visible as u8);
        _tmp
    }
}
impl crate::proto::common::GpsStatus {
    pub const ENCODED_LEN: usize = 101usize;
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
            _struct.satellites_visible = buf.get_u8() as u32;
            for _ in 0..20usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.satellite_prn.push(val.into());
            }
            for _ in 0..20usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.satellite_used.push(val.into());
            }
            for _ in 0..20usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.satellite_elevation.push(val.into());
            }
            for _ in 0..20usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.satellite_azimuth.push(val.into());
            }
            for _ in 0..20usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.satellite_snr.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.satellites_visible as u8);
        for val in &self.satellite_prn {
            _tmp.put_u8(*val as u8);
        }
        for val in &self.satellite_used {
            _tmp.put_u8(*val as u8);
        }
        for val in &self.satellite_elevation {
            _tmp.put_u8(*val as u8);
        }
        for val in &self.satellite_azimuth {
            _tmp.put_u8(*val as u8);
        }
        for val in &self.satellite_snr {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
impl crate::proto::common::ScaledImu {
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
            _struct.time_boot_ms = buf.get_u32_le();
            _struct.xacc = buf.get_i16_le() as i32;
            _struct.yacc = buf.get_i16_le() as i32;
            _struct.zacc = buf.get_i16_le() as i32;
            _struct.xgyro = buf.get_i16_le() as i32;
            _struct.ygyro = buf.get_i16_le() as i32;
            _struct.zgyro = buf.get_i16_le() as i32;
            _struct.xmag = buf.get_i16_le() as i32;
            _struct.ymag = buf.get_i16_le() as i32;
            _struct.zmag = buf.get_i16_le() as i32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms as u32);
        _tmp.put_i16_le(self.xacc as i16);
        _tmp.put_i16_le(self.yacc as i16);
        _tmp.put_i16_le(self.zacc as i16);
        _tmp.put_i16_le(self.xgyro as i16);
        _tmp.put_i16_le(self.ygyro as i16);
        _tmp.put_i16_le(self.zgyro as i16);
        _tmp.put_i16_le(self.xmag as i16);
        _tmp.put_i16_le(self.ymag as i16);
        _tmp.put_i16_le(self.zmag as i16);
        _tmp
    }
}
impl crate::proto::common::RawImu {
    pub const ENCODED_LEN: usize = 26usize;
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
            _struct.xacc = buf.get_i16_le() as i32;
            _struct.yacc = buf.get_i16_le() as i32;
            _struct.zacc = buf.get_i16_le() as i32;
            _struct.xgyro = buf.get_i16_le() as i32;
            _struct.ygyro = buf.get_i16_le() as i32;
            _struct.zgyro = buf.get_i16_le() as i32;
            _struct.xmag = buf.get_i16_le() as i32;
            _struct.ymag = buf.get_i16_le() as i32;
            _struct.zmag = buf.get_i16_le() as i32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        _tmp.put_i16_le(self.xacc as i16);
        _tmp.put_i16_le(self.yacc as i16);
        _tmp.put_i16_le(self.zacc as i16);
        _tmp.put_i16_le(self.xgyro as i16);
        _tmp.put_i16_le(self.ygyro as i16);
        _tmp.put_i16_le(self.zgyro as i16);
        _tmp.put_i16_le(self.xmag as i16);
        _tmp.put_i16_le(self.ymag as i16);
        _tmp.put_i16_le(self.zmag as i16);
        _tmp
    }
}
impl crate::proto::common::RawPressure {
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
            _struct.press_abs = buf.get_i16_le() as i32;
            _struct.press_diff1 = buf.get_i16_le() as i32;
            _struct.press_diff2 = buf.get_i16_le() as i32;
            _struct.temperature = buf.get_i16_le() as i32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        _tmp.put_i16_le(self.press_abs as i16);
        _tmp.put_i16_le(self.press_diff1 as i16);
        _tmp.put_i16_le(self.press_diff2 as i16);
        _tmp.put_i16_le(self.temperature as i16);
        _tmp
    }
}
impl crate::proto::common::ScaledPressure {
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
            _struct.time_boot_ms = buf.get_u32_le();
            _struct.press_abs = buf.get_f32_le();
            _struct.press_diff = buf.get_f32_le();
            _struct.temperature = buf.get_i16_le() as i32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms as u32);
        _tmp.put_f32_le(self.press_abs as f32);
        _tmp.put_f32_le(self.press_diff as f32);
        _tmp.put_i16_le(self.temperature as i16);
        _tmp
    }
}
impl crate::proto::common::Attitude {
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
            _struct.roll = buf.get_f32_le();
            _struct.pitch = buf.get_f32_le();
            _struct.yaw = buf.get_f32_le();
            _struct.rollspeed = buf.get_f32_le();
            _struct.pitchspeed = buf.get_f32_le();
            _struct.yawspeed = buf.get_f32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms as u32);
        _tmp.put_f32_le(self.roll as f32);
        _tmp.put_f32_le(self.pitch as f32);
        _tmp.put_f32_le(self.yaw as f32);
        _tmp.put_f32_le(self.rollspeed as f32);
        _tmp.put_f32_le(self.pitchspeed as f32);
        _tmp.put_f32_le(self.yawspeed as f32);
        _tmp
    }
}
impl crate::proto::common::AttitudeQuaternion {
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
            _struct.time_boot_ms = buf.get_u32_le();
            _struct.q1 = buf.get_f32_le();
            _struct.q2 = buf.get_f32_le();
            _struct.q3 = buf.get_f32_le();
            _struct.q4 = buf.get_f32_le();
            _struct.rollspeed = buf.get_f32_le();
            _struct.pitchspeed = buf.get_f32_le();
            _struct.yawspeed = buf.get_f32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms as u32);
        _tmp.put_f32_le(self.q1 as f32);
        _tmp.put_f32_le(self.q2 as f32);
        _tmp.put_f32_le(self.q3 as f32);
        _tmp.put_f32_le(self.q4 as f32);
        _tmp.put_f32_le(self.rollspeed as f32);
        _tmp.put_f32_le(self.pitchspeed as f32);
        _tmp.put_f32_le(self.yawspeed as f32);
        _tmp
    }
}
impl crate::proto::common::LocalPositionNed {
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
            _struct.x = buf.get_f32_le();
            _struct.y = buf.get_f32_le();
            _struct.z = buf.get_f32_le();
            _struct.vx = buf.get_f32_le();
            _struct.vy = buf.get_f32_le();
            _struct.vz = buf.get_f32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms as u32);
        _tmp.put_f32_le(self.x as f32);
        _tmp.put_f32_le(self.y as f32);
        _tmp.put_f32_le(self.z as f32);
        _tmp.put_f32_le(self.vx as f32);
        _tmp.put_f32_le(self.vy as f32);
        _tmp.put_f32_le(self.vz as f32);
        _tmp
    }
}
impl crate::proto::common::GlobalPositionInt {
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
            _struct.lat = buf.get_i32_le();
            _struct.lon = buf.get_i32_le();
            _struct.alt = buf.get_i32_le();
            _struct.relative_alt = buf.get_i32_le();
            _struct.vx = buf.get_i16_le() as i32;
            _struct.vy = buf.get_i16_le() as i32;
            _struct.vz = buf.get_i16_le() as i32;
            _struct.hdg = buf.get_u16_le() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms as u32);
        _tmp.put_i32_le(self.lat as i32);
        _tmp.put_i32_le(self.lon as i32);
        _tmp.put_i32_le(self.alt as i32);
        _tmp.put_i32_le(self.relative_alt as i32);
        _tmp.put_i16_le(self.vx as i16);
        _tmp.put_i16_le(self.vy as i16);
        _tmp.put_i16_le(self.vz as i16);
        _tmp.put_u16_le(self.hdg as u16);
        _tmp
    }
}
impl crate::proto::common::RcChannelsScaled {
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
            _struct.time_boot_ms = buf.get_u32_le();
            _struct.chan1_scaled = buf.get_i16_le() as i32;
            _struct.chan2_scaled = buf.get_i16_le() as i32;
            _struct.chan3_scaled = buf.get_i16_le() as i32;
            _struct.chan4_scaled = buf.get_i16_le() as i32;
            _struct.chan5_scaled = buf.get_i16_le() as i32;
            _struct.chan6_scaled = buf.get_i16_le() as i32;
            _struct.chan7_scaled = buf.get_i16_le() as i32;
            _struct.chan8_scaled = buf.get_i16_le() as i32;
            _struct.port = buf.get_u8() as u32;
            _struct.rssi = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms as u32);
        _tmp.put_i16_le(self.chan1_scaled as i16);
        _tmp.put_i16_le(self.chan2_scaled as i16);
        _tmp.put_i16_le(self.chan3_scaled as i16);
        _tmp.put_i16_le(self.chan4_scaled as i16);
        _tmp.put_i16_le(self.chan5_scaled as i16);
        _tmp.put_i16_le(self.chan6_scaled as i16);
        _tmp.put_i16_le(self.chan7_scaled as i16);
        _tmp.put_i16_le(self.chan8_scaled as i16);
        _tmp.put_u8(self.port as u8);
        _tmp.put_u8(self.rssi as u8);
        _tmp
    }
}
impl crate::proto::common::RcChannelsRaw {
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
            _struct.time_boot_ms = buf.get_u32_le();
            _struct.chan1_raw = buf.get_u16_le() as u32;
            _struct.chan2_raw = buf.get_u16_le() as u32;
            _struct.chan3_raw = buf.get_u16_le() as u32;
            _struct.chan4_raw = buf.get_u16_le() as u32;
            _struct.chan5_raw = buf.get_u16_le() as u32;
            _struct.chan6_raw = buf.get_u16_le() as u32;
            _struct.chan7_raw = buf.get_u16_le() as u32;
            _struct.chan8_raw = buf.get_u16_le() as u32;
            _struct.port = buf.get_u8() as u32;
            _struct.rssi = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms as u32);
        _tmp.put_u16_le(self.chan1_raw as u16);
        _tmp.put_u16_le(self.chan2_raw as u16);
        _tmp.put_u16_le(self.chan3_raw as u16);
        _tmp.put_u16_le(self.chan4_raw as u16);
        _tmp.put_u16_le(self.chan5_raw as u16);
        _tmp.put_u16_le(self.chan6_raw as u16);
        _tmp.put_u16_le(self.chan7_raw as u16);
        _tmp.put_u16_le(self.chan8_raw as u16);
        _tmp.put_u8(self.port as u8);
        _tmp.put_u8(self.rssi as u8);
        _tmp
    }
}
impl crate::proto::common::ServoOutputRaw {
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
            _struct.time_usec = buf.get_u32_le();
            _struct.servo1_raw = buf.get_u16_le() as u32;
            _struct.servo2_raw = buf.get_u16_le() as u32;
            _struct.servo3_raw = buf.get_u16_le() as u32;
            _struct.servo4_raw = buf.get_u16_le() as u32;
            _struct.servo5_raw = buf.get_u16_le() as u32;
            _struct.servo6_raw = buf.get_u16_le() as u32;
            _struct.servo7_raw = buf.get_u16_le() as u32;
            _struct.servo8_raw = buf.get_u16_le() as u32;
            _struct.port = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_usec as u32);
        _tmp.put_u16_le(self.servo1_raw as u16);
        _tmp.put_u16_le(self.servo2_raw as u16);
        _tmp.put_u16_le(self.servo3_raw as u16);
        _tmp.put_u16_le(self.servo4_raw as u16);
        _tmp.put_u16_le(self.servo5_raw as u16);
        _tmp.put_u16_le(self.servo6_raw as u16);
        _tmp.put_u16_le(self.servo7_raw as u16);
        _tmp.put_u16_le(self.servo8_raw as u16);
        _tmp.put_u8(self.port as u8);
        _tmp
    }
}
impl crate::proto::common::MissionRequestPartialList {
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
            _struct.start_index = buf.get_i16_le() as i32;
            _struct.end_index = buf.get_i16_le() as i32;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i16_le(self.start_index as i16);
        _tmp.put_i16_le(self.end_index as i16);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp
    }
}
impl crate::proto::common::MissionWritePartialList {
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
            _struct.start_index = buf.get_i16_le() as i32;
            _struct.end_index = buf.get_i16_le() as i32;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i16_le(self.start_index as i16);
        _tmp.put_i16_le(self.end_index as i16);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp
    }
}
impl crate::proto::common::MissionItem {
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
            _struct.param1 = buf.get_f32_le();
            _struct.param2 = buf.get_f32_le();
            _struct.param3 = buf.get_f32_le();
            _struct.param4 = buf.get_f32_le();
            _struct.x = buf.get_f32_le();
            _struct.y = buf.get_f32_le();
            _struct.z = buf.get_f32_le();
            _struct.seq = buf.get_u16_le() as u32;
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
        _tmp.put_f32_le(self.param1 as f32);
        _tmp.put_f32_le(self.param2 as f32);
        _tmp.put_f32_le(self.param3 as f32);
        _tmp.put_f32_le(self.param4 as f32);
        _tmp.put_f32_le(self.x as f32);
        _tmp.put_f32_le(self.y as f32);
        _tmp.put_f32_le(self.z as f32);
        _tmp.put_u16_le(self.seq as u16);
        _tmp.put_u16_le(self.command as u16);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp.put_u8(self.frame as u8);
        _tmp.put_u8(self.current as u8);
        _tmp.put_u8(self.autocontinue as u8);
        _tmp
    }
}
impl crate::proto::common::MissionRequest {
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
            _struct.seq = buf.get_u16_le() as u32;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.seq as u16);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp
    }
}
impl crate::proto::common::MissionSetCurrent {
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
            _struct.seq = buf.get_u16_le() as u32;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.seq as u16);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp
    }
}
impl crate::proto::common::MissionCurrent {
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
            _struct.seq = buf.get_u16_le() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.seq as u16);
        _tmp
    }
}
impl crate::proto::common::MissionRequestList {
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
impl crate::proto::common::MissionCount {
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
            _struct.count = buf.get_u16_le() as u32;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.count as u16);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp
    }
}
impl crate::proto::common::MissionClearAll {
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
impl crate::proto::common::MissionItemReached {
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
            _struct.seq = buf.get_u16_le() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.seq as u16);
        _tmp
    }
}
impl crate::proto::common::MissionAck {
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
            _struct.r#type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavMissionResult".to_string(),
                value: tmp as u32,
            })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp.put_u8(self.r#type as u8);
        _tmp
    }
}
impl crate::proto::common::SetGpsGlobalOrigin {
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
            _struct.latitude = buf.get_i32_le();
            _struct.longitude = buf.get_i32_le();
            _struct.altitude = buf.get_i32_le();
            _struct.target_system = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i32_le(self.latitude as i32);
        _tmp.put_i32_le(self.longitude as i32);
        _tmp.put_i32_le(self.altitude as i32);
        _tmp.put_u8(self.target_system as u8);
        _tmp
    }
}
impl crate::proto::common::GpsGlobalOrigin {
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
            _struct.latitude = buf.get_i32_le();
            _struct.longitude = buf.get_i32_le();
            _struct.altitude = buf.get_i32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i32_le(self.latitude as i32);
        _tmp.put_i32_le(self.longitude as i32);
        _tmp.put_i32_le(self.altitude as i32);
        _tmp
    }
}
impl crate::proto::common::ParamMapRc {
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
            _struct.param_value0 = buf.get_f32_le();
            _struct.scale = buf.get_f32_le();
            _struct.param_value_min = buf.get_f32_le();
            _struct.param_value_max = buf.get_f32_le();
            _struct.param_index = buf.get_i16_le() as i32;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            let mut s = Vec::with_capacity(16usize);
            for _ in 0..16usize {
                s.push(buf.get_u8());
            }
            _struct.param_id = String::from_utf8_lossy(&s).into();
            _struct.parameter_rc_channel_index = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.param_value0 as f32);
        _tmp.put_f32_le(self.scale as f32);
        _tmp.put_f32_le(self.param_value_min as f32);
        _tmp.put_f32_le(self.param_value_max as f32);
        _tmp.put_i16_le(self.param_index as i16);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        for val in self.param_id.bytes() {
            _tmp.put_u8(val);
        }
        _tmp.put_u8(self.parameter_rc_channel_index as u8);
        _tmp
    }
}
impl crate::proto::common::MissionRequestInt {
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
            _struct.seq = buf.get_u16_le() as u32;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.seq as u16);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp
    }
}
impl crate::proto::common::MissionChanged {
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
            _struct.start_index = buf.get_i16_le() as i32;
            _struct.end_index = buf.get_i16_le() as i32;
            _struct.origin_sysid = buf.get_u8() as u32;
            let tmp = buf.get_u8();
            _struct.origin_compid =
                FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "MavComponent".to_string(),
                    value: tmp as u32,
                })?;
            let tmp = buf.get_u8();
            _struct.mission_type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavMissionType".to_string(),
                value: tmp as u32,
            })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i16_le(self.start_index as i16);
        _tmp.put_i16_le(self.end_index as i16);
        _tmp.put_u8(self.origin_sysid as u8);
        _tmp.put_u8(self.origin_compid as u8);
        _tmp.put_u8(self.mission_type as u8);
        _tmp
    }
}
impl crate::proto::common::SafetySetAllowedArea {
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
            _struct.p1x = buf.get_f32_le();
            _struct.p1y = buf.get_f32_le();
            _struct.p1z = buf.get_f32_le();
            _struct.p2x = buf.get_f32_le();
            _struct.p2y = buf.get_f32_le();
            _struct.p2z = buf.get_f32_le();
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            let tmp = buf.get_u8();
            _struct.frame = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavFrame".to_string(),
                value: tmp as u32,
            })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.p1x as f32);
        _tmp.put_f32_le(self.p1y as f32);
        _tmp.put_f32_le(self.p1z as f32);
        _tmp.put_f32_le(self.p2x as f32);
        _tmp.put_f32_le(self.p2y as f32);
        _tmp.put_f32_le(self.p2z as f32);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp.put_u8(self.frame as u8);
        _tmp
    }
}
impl crate::proto::common::SafetyAllowedArea {
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
            _struct.p1x = buf.get_f32_le();
            _struct.p1y = buf.get_f32_le();
            _struct.p1z = buf.get_f32_le();
            _struct.p2x = buf.get_f32_le();
            _struct.p2y = buf.get_f32_le();
            _struct.p2z = buf.get_f32_le();
            let tmp = buf.get_u8();
            _struct.frame = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavFrame".to_string(),
                value: tmp as u32,
            })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.p1x as f32);
        _tmp.put_f32_le(self.p1y as f32);
        _tmp.put_f32_le(self.p1z as f32);
        _tmp.put_f32_le(self.p2x as f32);
        _tmp.put_f32_le(self.p2y as f32);
        _tmp.put_f32_le(self.p2z as f32);
        _tmp.put_u8(self.frame as u8);
        _tmp
    }
}
impl crate::proto::common::AttitudeQuaternionCov {
    pub const ENCODED_LEN: usize = 72usize;
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
            for _ in 0..4usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.q.push(val.into());
            }
            _struct.rollspeed = buf.get_f32_le();
            _struct.pitchspeed = buf.get_f32_le();
            _struct.yawspeed = buf.get_f32_le();
            for _ in 0..9usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.covariance.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        for val in &self.q {
            _tmp.put_f32_le(*val as f32);
        }
        _tmp.put_f32_le(self.rollspeed as f32);
        _tmp.put_f32_le(self.pitchspeed as f32);
        _tmp.put_f32_le(self.yawspeed as f32);
        for val in &self.covariance {
            _tmp.put_f32_le(*val as f32);
        }
        _tmp
    }
}
impl crate::proto::common::NavControllerOutput {
    pub const ENCODED_LEN: usize = 26usize;
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
            _struct.nav_roll = buf.get_f32_le();
            _struct.nav_pitch = buf.get_f32_le();
            _struct.alt_error = buf.get_f32_le();
            _struct.aspd_error = buf.get_f32_le();
            _struct.xtrack_error = buf.get_f32_le();
            _struct.nav_bearing = buf.get_i16_le() as i32;
            _struct.target_bearing = buf.get_i16_le() as i32;
            _struct.wp_dist = buf.get_u16_le() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.nav_roll as f32);
        _tmp.put_f32_le(self.nav_pitch as f32);
        _tmp.put_f32_le(self.alt_error as f32);
        _tmp.put_f32_le(self.aspd_error as f32);
        _tmp.put_f32_le(self.xtrack_error as f32);
        _tmp.put_i16_le(self.nav_bearing as i16);
        _tmp.put_i16_le(self.target_bearing as i16);
        _tmp.put_u16_le(self.wp_dist as u16);
        _tmp
    }
}
impl crate::proto::common::GlobalPositionIntCov {
    pub const ENCODED_LEN: usize = 181usize;
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
            _struct.lon = buf.get_i32_le();
            _struct.alt = buf.get_i32_le();
            _struct.relative_alt = buf.get_i32_le();
            _struct.vx = buf.get_f32_le();
            _struct.vy = buf.get_f32_le();
            _struct.vz = buf.get_f32_le();
            for _ in 0..36usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.covariance.push(val.into());
            }
            let tmp = buf.get_u8();
            _struct.estimator_type =
                FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "MavEstimatorType".to_string(),
                    value: tmp as u32,
                })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        _tmp.put_i32_le(self.lat as i32);
        _tmp.put_i32_le(self.lon as i32);
        _tmp.put_i32_le(self.alt as i32);
        _tmp.put_i32_le(self.relative_alt as i32);
        _tmp.put_f32_le(self.vx as f32);
        _tmp.put_f32_le(self.vy as f32);
        _tmp.put_f32_le(self.vz as f32);
        for val in &self.covariance {
            _tmp.put_f32_le(*val as f32);
        }
        _tmp.put_u8(self.estimator_type as u8);
        _tmp
    }
}
impl crate::proto::common::LocalPositionNedCov {
    pub const ENCODED_LEN: usize = 225usize;
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
            _struct.x = buf.get_f32_le();
            _struct.y = buf.get_f32_le();
            _struct.z = buf.get_f32_le();
            _struct.vx = buf.get_f32_le();
            _struct.vy = buf.get_f32_le();
            _struct.vz = buf.get_f32_le();
            _struct.ax = buf.get_f32_le();
            _struct.ay = buf.get_f32_le();
            _struct.az = buf.get_f32_le();
            for _ in 0..45usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.covariance.push(val.into());
            }
            let tmp = buf.get_u8();
            _struct.estimator_type =
                FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "MavEstimatorType".to_string(),
                    value: tmp as u32,
                })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        _tmp.put_f32_le(self.x as f32);
        _tmp.put_f32_le(self.y as f32);
        _tmp.put_f32_le(self.z as f32);
        _tmp.put_f32_le(self.vx as f32);
        _tmp.put_f32_le(self.vy as f32);
        _tmp.put_f32_le(self.vz as f32);
        _tmp.put_f32_le(self.ax as f32);
        _tmp.put_f32_le(self.ay as f32);
        _tmp.put_f32_le(self.az as f32);
        for val in &self.covariance {
            _tmp.put_f32_le(*val as f32);
        }
        _tmp.put_u8(self.estimator_type as u8);
        _tmp
    }
}
impl crate::proto::common::RcChannels {
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
            _struct.time_boot_ms = buf.get_u32_le();
            _struct.chan1_raw = buf.get_u16_le() as u32;
            _struct.chan2_raw = buf.get_u16_le() as u32;
            _struct.chan3_raw = buf.get_u16_le() as u32;
            _struct.chan4_raw = buf.get_u16_le() as u32;
            _struct.chan5_raw = buf.get_u16_le() as u32;
            _struct.chan6_raw = buf.get_u16_le() as u32;
            _struct.chan7_raw = buf.get_u16_le() as u32;
            _struct.chan8_raw = buf.get_u16_le() as u32;
            _struct.chan9_raw = buf.get_u16_le() as u32;
            _struct.chan10_raw = buf.get_u16_le() as u32;
            _struct.chan11_raw = buf.get_u16_le() as u32;
            _struct.chan12_raw = buf.get_u16_le() as u32;
            _struct.chan13_raw = buf.get_u16_le() as u32;
            _struct.chan14_raw = buf.get_u16_le() as u32;
            _struct.chan15_raw = buf.get_u16_le() as u32;
            _struct.chan16_raw = buf.get_u16_le() as u32;
            _struct.chan17_raw = buf.get_u16_le() as u32;
            _struct.chan18_raw = buf.get_u16_le() as u32;
            _struct.chancount = buf.get_u8() as u32;
            _struct.rssi = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms as u32);
        _tmp.put_u16_le(self.chan1_raw as u16);
        _tmp.put_u16_le(self.chan2_raw as u16);
        _tmp.put_u16_le(self.chan3_raw as u16);
        _tmp.put_u16_le(self.chan4_raw as u16);
        _tmp.put_u16_le(self.chan5_raw as u16);
        _tmp.put_u16_le(self.chan6_raw as u16);
        _tmp.put_u16_le(self.chan7_raw as u16);
        _tmp.put_u16_le(self.chan8_raw as u16);
        _tmp.put_u16_le(self.chan9_raw as u16);
        _tmp.put_u16_le(self.chan10_raw as u16);
        _tmp.put_u16_le(self.chan11_raw as u16);
        _tmp.put_u16_le(self.chan12_raw as u16);
        _tmp.put_u16_le(self.chan13_raw as u16);
        _tmp.put_u16_le(self.chan14_raw as u16);
        _tmp.put_u16_le(self.chan15_raw as u16);
        _tmp.put_u16_le(self.chan16_raw as u16);
        _tmp.put_u16_le(self.chan17_raw as u16);
        _tmp.put_u16_le(self.chan18_raw as u16);
        _tmp.put_u8(self.chancount as u8);
        _tmp.put_u8(self.rssi as u8);
        _tmp
    }
}
impl crate::proto::common::RequestDataStream {
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
            _struct.req_message_rate = buf.get_u16_le() as u32;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            _struct.req_stream_id = buf.get_u8() as u32;
            _struct.start_stop = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.req_message_rate as u16);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp.put_u8(self.req_stream_id as u8);
        _tmp.put_u8(self.start_stop as u8);
        _tmp
    }
}
impl crate::proto::common::DataStream {
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
            _struct.message_rate = buf.get_u16_le() as u32;
            _struct.stream_id = buf.get_u8() as u32;
            _struct.on_off = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.message_rate as u16);
        _tmp.put_u8(self.stream_id as u8);
        _tmp.put_u8(self.on_off as u8);
        _tmp
    }
}
impl crate::proto::common::ManualControl {
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
            _struct.x = buf.get_i16_le() as i32;
            _struct.y = buf.get_i16_le() as i32;
            _struct.z = buf.get_i16_le() as i32;
            _struct.r = buf.get_i16_le() as i32;
            _struct.buttons = buf.get_u16_le() as u32;
            _struct.target = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i16_le(self.x as i16);
        _tmp.put_i16_le(self.y as i16);
        _tmp.put_i16_le(self.z as i16);
        _tmp.put_i16_le(self.r as i16);
        _tmp.put_u16_le(self.buttons as u16);
        _tmp.put_u8(self.target as u8);
        _tmp
    }
}
impl crate::proto::common::RcChannelsOverride {
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
            _struct.chan1_raw = buf.get_u16_le() as u32;
            _struct.chan2_raw = buf.get_u16_le() as u32;
            _struct.chan3_raw = buf.get_u16_le() as u32;
            _struct.chan4_raw = buf.get_u16_le() as u32;
            _struct.chan5_raw = buf.get_u16_le() as u32;
            _struct.chan6_raw = buf.get_u16_le() as u32;
            _struct.chan7_raw = buf.get_u16_le() as u32;
            _struct.chan8_raw = buf.get_u16_le() as u32;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.chan1_raw as u16);
        _tmp.put_u16_le(self.chan2_raw as u16);
        _tmp.put_u16_le(self.chan3_raw as u16);
        _tmp.put_u16_le(self.chan4_raw as u16);
        _tmp.put_u16_le(self.chan5_raw as u16);
        _tmp.put_u16_le(self.chan6_raw as u16);
        _tmp.put_u16_le(self.chan7_raw as u16);
        _tmp.put_u16_le(self.chan8_raw as u16);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp
    }
}
impl crate::proto::common::MissionItemInt {
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
            _struct.param1 = buf.get_f32_le();
            _struct.param2 = buf.get_f32_le();
            _struct.param3 = buf.get_f32_le();
            _struct.param4 = buf.get_f32_le();
            _struct.x = buf.get_i32_le();
            _struct.y = buf.get_i32_le();
            _struct.z = buf.get_f32_le();
            _struct.seq = buf.get_u16_le() as u32;
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
        _tmp.put_f32_le(self.param1 as f32);
        _tmp.put_f32_le(self.param2 as f32);
        _tmp.put_f32_le(self.param3 as f32);
        _tmp.put_f32_le(self.param4 as f32);
        _tmp.put_i32_le(self.x as i32);
        _tmp.put_i32_le(self.y as i32);
        _tmp.put_f32_le(self.z as f32);
        _tmp.put_u16_le(self.seq as u16);
        _tmp.put_u16_le(self.command as u16);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp.put_u8(self.frame as u8);
        _tmp.put_u8(self.current as u8);
        _tmp.put_u8(self.autocontinue as u8);
        _tmp
    }
}
impl crate::proto::common::VfrHud {
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
            _struct.airspeed = buf.get_f32_le();
            _struct.groundspeed = buf.get_f32_le();
            _struct.alt = buf.get_f32_le();
            _struct.climb = buf.get_f32_le();
            _struct.heading = buf.get_i16_le() as i32;
            _struct.throttle = buf.get_u16_le() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.airspeed as f32);
        _tmp.put_f32_le(self.groundspeed as f32);
        _tmp.put_f32_le(self.alt as f32);
        _tmp.put_f32_le(self.climb as f32);
        _tmp.put_i16_le(self.heading as i16);
        _tmp.put_u16_le(self.throttle as u16);
        _tmp
    }
}
impl crate::proto::common::CommandInt {
    pub const ENCODED_LEN: usize = 35usize;
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
impl crate::proto::common::CommandLong {
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
impl crate::proto::common::CommandAck {
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
            _struct.command = FromPrimitive::from_u16(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavCmd".to_string(),
                value: tmp as u32,
            })?;
            let tmp = buf.get_u8();
            _struct.result = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavResult".to_string(),
                value: tmp as u32,
            })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.command as u16);
        _tmp.put_u8(self.result as u8);
        _tmp
    }
}
impl crate::proto::common::CommandCancel {
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
            let tmp = buf.get_u16_le();
            _struct.command = FromPrimitive::from_u16(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavCmd".to_string(),
                value: tmp as u32,
            })?;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.command as u16);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp
    }
}
impl crate::proto::common::ManualSetpoint {
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
            _struct.time_boot_ms = buf.get_u32_le();
            _struct.roll = buf.get_f32_le();
            _struct.pitch = buf.get_f32_le();
            _struct.yaw = buf.get_f32_le();
            _struct.thrust = buf.get_f32_le();
            _struct.mode_switch = buf.get_u8() as u32;
            _struct.manual_override_switch = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms as u32);
        _tmp.put_f32_le(self.roll as f32);
        _tmp.put_f32_le(self.pitch as f32);
        _tmp.put_f32_le(self.yaw as f32);
        _tmp.put_f32_le(self.thrust as f32);
        _tmp.put_u8(self.mode_switch as u8);
        _tmp.put_u8(self.manual_override_switch as u8);
        _tmp
    }
}
impl crate::proto::common::SetAttitudeTarget {
    pub const ENCODED_LEN: usize = 39usize;
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
            for _ in 0..4usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.q.push(val.into());
            }
            _struct.body_roll_rate = buf.get_f32_le();
            _struct.body_pitch_rate = buf.get_f32_le();
            _struct.body_yaw_rate = buf.get_f32_le();
            _struct.thrust = buf.get_f32_le();
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            _struct.type_mask = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms as u32);
        for val in &self.q {
            _tmp.put_f32_le(*val as f32);
        }
        _tmp.put_f32_le(self.body_roll_rate as f32);
        _tmp.put_f32_le(self.body_pitch_rate as f32);
        _tmp.put_f32_le(self.body_yaw_rate as f32);
        _tmp.put_f32_le(self.thrust as f32);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp.put_u8(self.type_mask as u8);
        _tmp
    }
}
impl crate::proto::common::AttitudeTarget {
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
            _struct.time_boot_ms = buf.get_u32_le();
            for _ in 0..4usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.q.push(val.into());
            }
            _struct.body_roll_rate = buf.get_f32_le();
            _struct.body_pitch_rate = buf.get_f32_le();
            _struct.body_yaw_rate = buf.get_f32_le();
            _struct.thrust = buf.get_f32_le();
            _struct.type_mask = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms as u32);
        for val in &self.q {
            _tmp.put_f32_le(*val as f32);
        }
        _tmp.put_f32_le(self.body_roll_rate as f32);
        _tmp.put_f32_le(self.body_pitch_rate as f32);
        _tmp.put_f32_le(self.body_yaw_rate as f32);
        _tmp.put_f32_le(self.thrust as f32);
        _tmp.put_u8(self.type_mask as u8);
        _tmp
    }
}
impl crate::proto::common::SetPositionTargetLocalNed {
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
            _struct.time_boot_ms = buf.get_u32_le();
            _struct.x = buf.get_f32_le();
            _struct.y = buf.get_f32_le();
            _struct.z = buf.get_f32_le();
            _struct.vx = buf.get_f32_le();
            _struct.vy = buf.get_f32_le();
            _struct.vz = buf.get_f32_le();
            _struct.afx = buf.get_f32_le();
            _struct.afy = buf.get_f32_le();
            _struct.afz = buf.get_f32_le();
            _struct.yaw = buf.get_f32_le();
            _struct.yaw_rate = buf.get_f32_le();
            let tmp = buf.get_u16_le();
            _struct.type_mask = FromPrimitive::from_u16(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "PositionTargetTypemask".to_string(),
                value: tmp as u32,
            })?;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            let tmp = buf.get_u8();
            _struct.coordinate_frame =
                FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "MavFrame".to_string(),
                    value: tmp as u32,
                })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms as u32);
        _tmp.put_f32_le(self.x as f32);
        _tmp.put_f32_le(self.y as f32);
        _tmp.put_f32_le(self.z as f32);
        _tmp.put_f32_le(self.vx as f32);
        _tmp.put_f32_le(self.vy as f32);
        _tmp.put_f32_le(self.vz as f32);
        _tmp.put_f32_le(self.afx as f32);
        _tmp.put_f32_le(self.afy as f32);
        _tmp.put_f32_le(self.afz as f32);
        _tmp.put_f32_le(self.yaw as f32);
        _tmp.put_f32_le(self.yaw_rate as f32);
        _tmp.put_u16_le(self.type_mask as u16);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp.put_u8(self.coordinate_frame as u8);
        _tmp
    }
}
impl crate::proto::common::PositionTargetLocalNed {
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
            _struct.time_boot_ms = buf.get_u32_le();
            _struct.x = buf.get_f32_le();
            _struct.y = buf.get_f32_le();
            _struct.z = buf.get_f32_le();
            _struct.vx = buf.get_f32_le();
            _struct.vy = buf.get_f32_le();
            _struct.vz = buf.get_f32_le();
            _struct.afx = buf.get_f32_le();
            _struct.afy = buf.get_f32_le();
            _struct.afz = buf.get_f32_le();
            _struct.yaw = buf.get_f32_le();
            _struct.yaw_rate = buf.get_f32_le();
            let tmp = buf.get_u16_le();
            _struct.type_mask = FromPrimitive::from_u16(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "PositionTargetTypemask".to_string(),
                value: tmp as u32,
            })?;
            let tmp = buf.get_u8();
            _struct.coordinate_frame =
                FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "MavFrame".to_string(),
                    value: tmp as u32,
                })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms as u32);
        _tmp.put_f32_le(self.x as f32);
        _tmp.put_f32_le(self.y as f32);
        _tmp.put_f32_le(self.z as f32);
        _tmp.put_f32_le(self.vx as f32);
        _tmp.put_f32_le(self.vy as f32);
        _tmp.put_f32_le(self.vz as f32);
        _tmp.put_f32_le(self.afx as f32);
        _tmp.put_f32_le(self.afy as f32);
        _tmp.put_f32_le(self.afz as f32);
        _tmp.put_f32_le(self.yaw as f32);
        _tmp.put_f32_le(self.yaw_rate as f32);
        _tmp.put_u16_le(self.type_mask as u16);
        _tmp.put_u8(self.coordinate_frame as u8);
        _tmp
    }
}
impl crate::proto::common::SetPositionTargetGlobalInt {
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
            _struct.time_boot_ms = buf.get_u32_le();
            _struct.lat_int = buf.get_i32_le();
            _struct.lon_int = buf.get_i32_le();
            _struct.alt = buf.get_f32_le();
            _struct.vx = buf.get_f32_le();
            _struct.vy = buf.get_f32_le();
            _struct.vz = buf.get_f32_le();
            _struct.afx = buf.get_f32_le();
            _struct.afy = buf.get_f32_le();
            _struct.afz = buf.get_f32_le();
            _struct.yaw = buf.get_f32_le();
            _struct.yaw_rate = buf.get_f32_le();
            let tmp = buf.get_u16_le();
            _struct.type_mask = FromPrimitive::from_u16(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "PositionTargetTypemask".to_string(),
                value: tmp as u32,
            })?;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            let tmp = buf.get_u8();
            _struct.coordinate_frame =
                FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "MavFrame".to_string(),
                    value: tmp as u32,
                })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms as u32);
        _tmp.put_i32_le(self.lat_int as i32);
        _tmp.put_i32_le(self.lon_int as i32);
        _tmp.put_f32_le(self.alt as f32);
        _tmp.put_f32_le(self.vx as f32);
        _tmp.put_f32_le(self.vy as f32);
        _tmp.put_f32_le(self.vz as f32);
        _tmp.put_f32_le(self.afx as f32);
        _tmp.put_f32_le(self.afy as f32);
        _tmp.put_f32_le(self.afz as f32);
        _tmp.put_f32_le(self.yaw as f32);
        _tmp.put_f32_le(self.yaw_rate as f32);
        _tmp.put_u16_le(self.type_mask as u16);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp.put_u8(self.coordinate_frame as u8);
        _tmp
    }
}
impl crate::proto::common::PositionTargetGlobalInt {
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
            _struct.time_boot_ms = buf.get_u32_le();
            _struct.lat_int = buf.get_i32_le();
            _struct.lon_int = buf.get_i32_le();
            _struct.alt = buf.get_f32_le();
            _struct.vx = buf.get_f32_le();
            _struct.vy = buf.get_f32_le();
            _struct.vz = buf.get_f32_le();
            _struct.afx = buf.get_f32_le();
            _struct.afy = buf.get_f32_le();
            _struct.afz = buf.get_f32_le();
            _struct.yaw = buf.get_f32_le();
            _struct.yaw_rate = buf.get_f32_le();
            let tmp = buf.get_u16_le();
            _struct.type_mask = FromPrimitive::from_u16(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "PositionTargetTypemask".to_string(),
                value: tmp as u32,
            })?;
            let tmp = buf.get_u8();
            _struct.coordinate_frame =
                FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "MavFrame".to_string(),
                    value: tmp as u32,
                })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms as u32);
        _tmp.put_i32_le(self.lat_int as i32);
        _tmp.put_i32_le(self.lon_int as i32);
        _tmp.put_f32_le(self.alt as f32);
        _tmp.put_f32_le(self.vx as f32);
        _tmp.put_f32_le(self.vy as f32);
        _tmp.put_f32_le(self.vz as f32);
        _tmp.put_f32_le(self.afx as f32);
        _tmp.put_f32_le(self.afy as f32);
        _tmp.put_f32_le(self.afz as f32);
        _tmp.put_f32_le(self.yaw as f32);
        _tmp.put_f32_le(self.yaw_rate as f32);
        _tmp.put_u16_le(self.type_mask as u16);
        _tmp.put_u8(self.coordinate_frame as u8);
        _tmp
    }
}
impl crate::proto::common::LocalPositionNedSystemGlobalOffset {
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
            _struct.x = buf.get_f32_le();
            _struct.y = buf.get_f32_le();
            _struct.z = buf.get_f32_le();
            _struct.roll = buf.get_f32_le();
            _struct.pitch = buf.get_f32_le();
            _struct.yaw = buf.get_f32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms as u32);
        _tmp.put_f32_le(self.x as f32);
        _tmp.put_f32_le(self.y as f32);
        _tmp.put_f32_le(self.z as f32);
        _tmp.put_f32_le(self.roll as f32);
        _tmp.put_f32_le(self.pitch as f32);
        _tmp.put_f32_le(self.yaw as f32);
        _tmp
    }
}
impl crate::proto::common::HilState {
    pub const ENCODED_LEN: usize = 56usize;
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
            _struct.roll = buf.get_f32_le();
            _struct.pitch = buf.get_f32_le();
            _struct.yaw = buf.get_f32_le();
            _struct.rollspeed = buf.get_f32_le();
            _struct.pitchspeed = buf.get_f32_le();
            _struct.yawspeed = buf.get_f32_le();
            _struct.lat = buf.get_i32_le();
            _struct.lon = buf.get_i32_le();
            _struct.alt = buf.get_i32_le();
            _struct.vx = buf.get_i16_le() as i32;
            _struct.vy = buf.get_i16_le() as i32;
            _struct.vz = buf.get_i16_le() as i32;
            _struct.xacc = buf.get_i16_le() as i32;
            _struct.yacc = buf.get_i16_le() as i32;
            _struct.zacc = buf.get_i16_le() as i32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        _tmp.put_f32_le(self.roll as f32);
        _tmp.put_f32_le(self.pitch as f32);
        _tmp.put_f32_le(self.yaw as f32);
        _tmp.put_f32_le(self.rollspeed as f32);
        _tmp.put_f32_le(self.pitchspeed as f32);
        _tmp.put_f32_le(self.yawspeed as f32);
        _tmp.put_i32_le(self.lat as i32);
        _tmp.put_i32_le(self.lon as i32);
        _tmp.put_i32_le(self.alt as i32);
        _tmp.put_i16_le(self.vx as i16);
        _tmp.put_i16_le(self.vy as i16);
        _tmp.put_i16_le(self.vz as i16);
        _tmp.put_i16_le(self.xacc as i16);
        _tmp.put_i16_le(self.yacc as i16);
        _tmp.put_i16_le(self.zacc as i16);
        _tmp
    }
}
impl crate::proto::common::HilControls {
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
            _struct.time_usec = buf.get_u64_le();
            _struct.roll_ailerons = buf.get_f32_le();
            _struct.pitch_elevator = buf.get_f32_le();
            _struct.yaw_rudder = buf.get_f32_le();
            _struct.throttle = buf.get_f32_le();
            _struct.aux1 = buf.get_f32_le();
            _struct.aux2 = buf.get_f32_le();
            _struct.aux3 = buf.get_f32_le();
            _struct.aux4 = buf.get_f32_le();
            let tmp = buf.get_u8();
            _struct.mode = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavMode".to_string(),
                value: tmp as u32,
            })?;
            _struct.nav_mode = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        _tmp.put_f32_le(self.roll_ailerons as f32);
        _tmp.put_f32_le(self.pitch_elevator as f32);
        _tmp.put_f32_le(self.yaw_rudder as f32);
        _tmp.put_f32_le(self.throttle as f32);
        _tmp.put_f32_le(self.aux1 as f32);
        _tmp.put_f32_le(self.aux2 as f32);
        _tmp.put_f32_le(self.aux3 as f32);
        _tmp.put_f32_le(self.aux4 as f32);
        _tmp.put_u8(self.mode as u8);
        _tmp.put_u8(self.nav_mode as u8);
        _tmp
    }
}
impl crate::proto::common::HilRcInputsRaw {
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
            _struct.time_usec = buf.get_u64_le();
            _struct.chan1_raw = buf.get_u16_le() as u32;
            _struct.chan2_raw = buf.get_u16_le() as u32;
            _struct.chan3_raw = buf.get_u16_le() as u32;
            _struct.chan4_raw = buf.get_u16_le() as u32;
            _struct.chan5_raw = buf.get_u16_le() as u32;
            _struct.chan6_raw = buf.get_u16_le() as u32;
            _struct.chan7_raw = buf.get_u16_le() as u32;
            _struct.chan8_raw = buf.get_u16_le() as u32;
            _struct.chan9_raw = buf.get_u16_le() as u32;
            _struct.chan10_raw = buf.get_u16_le() as u32;
            _struct.chan11_raw = buf.get_u16_le() as u32;
            _struct.chan12_raw = buf.get_u16_le() as u32;
            _struct.rssi = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        _tmp.put_u16_le(self.chan1_raw as u16);
        _tmp.put_u16_le(self.chan2_raw as u16);
        _tmp.put_u16_le(self.chan3_raw as u16);
        _tmp.put_u16_le(self.chan4_raw as u16);
        _tmp.put_u16_le(self.chan5_raw as u16);
        _tmp.put_u16_le(self.chan6_raw as u16);
        _tmp.put_u16_le(self.chan7_raw as u16);
        _tmp.put_u16_le(self.chan8_raw as u16);
        _tmp.put_u16_le(self.chan9_raw as u16);
        _tmp.put_u16_le(self.chan10_raw as u16);
        _tmp.put_u16_le(self.chan11_raw as u16);
        _tmp.put_u16_le(self.chan12_raw as u16);
        _tmp.put_u8(self.rssi as u8);
        _tmp
    }
}
impl crate::proto::common::HilActuatorControls {
    pub const ENCODED_LEN: usize = 81usize;
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
            _struct.flags = buf.get_u64_le();
            for _ in 0..16usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.controls.push(val.into());
            }
            let tmp = buf.get_u8();
            _struct.mode = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavModeFlag".to_string(),
                value: tmp as u32,
            })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        _tmp.put_u64_le(self.flags as u64);
        for val in &self.controls {
            _tmp.put_f32_le(*val as f32);
        }
        _tmp.put_u8(self.mode as u8);
        _tmp
    }
}
impl crate::proto::common::OpticalFlow {
    pub const ENCODED_LEN: usize = 26usize;
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
            _struct.flow_comp_m_x = buf.get_f32_le();
            _struct.flow_comp_m_y = buf.get_f32_le();
            _struct.ground_distance = buf.get_f32_le();
            _struct.flow_x = buf.get_i16_le() as i32;
            _struct.flow_y = buf.get_i16_le() as i32;
            _struct.sensor_id = buf.get_u8() as u32;
            _struct.quality = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        _tmp.put_f32_le(self.flow_comp_m_x as f32);
        _tmp.put_f32_le(self.flow_comp_m_y as f32);
        _tmp.put_f32_le(self.ground_distance as f32);
        _tmp.put_i16_le(self.flow_x as i16);
        _tmp.put_i16_le(self.flow_y as i16);
        _tmp.put_u8(self.sensor_id as u8);
        _tmp.put_u8(self.quality as u8);
        _tmp
    }
}
impl crate::proto::common::GlobalVisionPositionEstimate {
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
            _struct.usec = buf.get_u64_le();
            _struct.x = buf.get_f32_le();
            _struct.y = buf.get_f32_le();
            _struct.z = buf.get_f32_le();
            _struct.roll = buf.get_f32_le();
            _struct.pitch = buf.get_f32_le();
            _struct.yaw = buf.get_f32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.usec as u64);
        _tmp.put_f32_le(self.x as f32);
        _tmp.put_f32_le(self.y as f32);
        _tmp.put_f32_le(self.z as f32);
        _tmp.put_f32_le(self.roll as f32);
        _tmp.put_f32_le(self.pitch as f32);
        _tmp.put_f32_le(self.yaw as f32);
        _tmp
    }
}
impl crate::proto::common::VisionPositionEstimate {
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
            _struct.usec = buf.get_u64_le();
            _struct.x = buf.get_f32_le();
            _struct.y = buf.get_f32_le();
            _struct.z = buf.get_f32_le();
            _struct.roll = buf.get_f32_le();
            _struct.pitch = buf.get_f32_le();
            _struct.yaw = buf.get_f32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.usec as u64);
        _tmp.put_f32_le(self.x as f32);
        _tmp.put_f32_le(self.y as f32);
        _tmp.put_f32_le(self.z as f32);
        _tmp.put_f32_le(self.roll as f32);
        _tmp.put_f32_le(self.pitch as f32);
        _tmp.put_f32_le(self.yaw as f32);
        _tmp
    }
}
impl crate::proto::common::VisionSpeedEstimate {
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
            _struct.usec = buf.get_u64_le();
            _struct.x = buf.get_f32_le();
            _struct.y = buf.get_f32_le();
            _struct.z = buf.get_f32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.usec as u64);
        _tmp.put_f32_le(self.x as f32);
        _tmp.put_f32_le(self.y as f32);
        _tmp.put_f32_le(self.z as f32);
        _tmp
    }
}
impl crate::proto::common::ViconPositionEstimate {
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
            _struct.usec = buf.get_u64_le();
            _struct.x = buf.get_f32_le();
            _struct.y = buf.get_f32_le();
            _struct.z = buf.get_f32_le();
            _struct.roll = buf.get_f32_le();
            _struct.pitch = buf.get_f32_le();
            _struct.yaw = buf.get_f32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.usec as u64);
        _tmp.put_f32_le(self.x as f32);
        _tmp.put_f32_le(self.y as f32);
        _tmp.put_f32_le(self.z as f32);
        _tmp.put_f32_le(self.roll as f32);
        _tmp.put_f32_le(self.pitch as f32);
        _tmp.put_f32_le(self.yaw as f32);
        _tmp
    }
}
impl crate::proto::common::HighresImu {
    pub const ENCODED_LEN: usize = 62usize;
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
            _struct.xacc = buf.get_f32_le();
            _struct.yacc = buf.get_f32_le();
            _struct.zacc = buf.get_f32_le();
            _struct.xgyro = buf.get_f32_le();
            _struct.ygyro = buf.get_f32_le();
            _struct.zgyro = buf.get_f32_le();
            _struct.xmag = buf.get_f32_le();
            _struct.ymag = buf.get_f32_le();
            _struct.zmag = buf.get_f32_le();
            _struct.abs_pressure = buf.get_f32_le();
            _struct.diff_pressure = buf.get_f32_le();
            _struct.pressure_alt = buf.get_f32_le();
            _struct.temperature = buf.get_f32_le();
            _struct.fields_updated = buf.get_u16_le() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        _tmp.put_f32_le(self.xacc as f32);
        _tmp.put_f32_le(self.yacc as f32);
        _tmp.put_f32_le(self.zacc as f32);
        _tmp.put_f32_le(self.xgyro as f32);
        _tmp.put_f32_le(self.ygyro as f32);
        _tmp.put_f32_le(self.zgyro as f32);
        _tmp.put_f32_le(self.xmag as f32);
        _tmp.put_f32_le(self.ymag as f32);
        _tmp.put_f32_le(self.zmag as f32);
        _tmp.put_f32_le(self.abs_pressure as f32);
        _tmp.put_f32_le(self.diff_pressure as f32);
        _tmp.put_f32_le(self.pressure_alt as f32);
        _tmp.put_f32_le(self.temperature as f32);
        _tmp.put_u16_le(self.fields_updated as u16);
        _tmp
    }
}
impl crate::proto::common::OpticalFlowRad {
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
            _struct.integration_time_us = buf.get_u32_le();
            _struct.integrated_x = buf.get_f32_le();
            _struct.integrated_y = buf.get_f32_le();
            _struct.integrated_xgyro = buf.get_f32_le();
            _struct.integrated_ygyro = buf.get_f32_le();
            _struct.integrated_zgyro = buf.get_f32_le();
            _struct.time_delta_distance_us = buf.get_u32_le();
            _struct.distance = buf.get_f32_le();
            _struct.temperature = buf.get_i16_le() as i32;
            _struct.sensor_id = buf.get_u8() as u32;
            _struct.quality = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        _tmp.put_u32_le(self.integration_time_us as u32);
        _tmp.put_f32_le(self.integrated_x as f32);
        _tmp.put_f32_le(self.integrated_y as f32);
        _tmp.put_f32_le(self.integrated_xgyro as f32);
        _tmp.put_f32_le(self.integrated_ygyro as f32);
        _tmp.put_f32_le(self.integrated_zgyro as f32);
        _tmp.put_u32_le(self.time_delta_distance_us as u32);
        _tmp.put_f32_le(self.distance as f32);
        _tmp.put_i16_le(self.temperature as i16);
        _tmp.put_u8(self.sensor_id as u8);
        _tmp.put_u8(self.quality as u8);
        _tmp
    }
}
impl crate::proto::common::HilSensor {
    pub const ENCODED_LEN: usize = 64usize;
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
            _struct.xacc = buf.get_f32_le();
            _struct.yacc = buf.get_f32_le();
            _struct.zacc = buf.get_f32_le();
            _struct.xgyro = buf.get_f32_le();
            _struct.ygyro = buf.get_f32_le();
            _struct.zgyro = buf.get_f32_le();
            _struct.xmag = buf.get_f32_le();
            _struct.ymag = buf.get_f32_le();
            _struct.zmag = buf.get_f32_le();
            _struct.abs_pressure = buf.get_f32_le();
            _struct.diff_pressure = buf.get_f32_le();
            _struct.pressure_alt = buf.get_f32_le();
            _struct.temperature = buf.get_f32_le();
            _struct.fields_updated = buf.get_u32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        _tmp.put_f32_le(self.xacc as f32);
        _tmp.put_f32_le(self.yacc as f32);
        _tmp.put_f32_le(self.zacc as f32);
        _tmp.put_f32_le(self.xgyro as f32);
        _tmp.put_f32_le(self.ygyro as f32);
        _tmp.put_f32_le(self.zgyro as f32);
        _tmp.put_f32_le(self.xmag as f32);
        _tmp.put_f32_le(self.ymag as f32);
        _tmp.put_f32_le(self.zmag as f32);
        _tmp.put_f32_le(self.abs_pressure as f32);
        _tmp.put_f32_le(self.diff_pressure as f32);
        _tmp.put_f32_le(self.pressure_alt as f32);
        _tmp.put_f32_le(self.temperature as f32);
        _tmp.put_u32_le(self.fields_updated as u32);
        _tmp
    }
}
impl crate::proto::common::SimState {
    pub const ENCODED_LEN: usize = 84usize;
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
            _struct.q1 = buf.get_f32_le();
            _struct.q2 = buf.get_f32_le();
            _struct.q3 = buf.get_f32_le();
            _struct.q4 = buf.get_f32_le();
            _struct.roll = buf.get_f32_le();
            _struct.pitch = buf.get_f32_le();
            _struct.yaw = buf.get_f32_le();
            _struct.xacc = buf.get_f32_le();
            _struct.yacc = buf.get_f32_le();
            _struct.zacc = buf.get_f32_le();
            _struct.xgyro = buf.get_f32_le();
            _struct.ygyro = buf.get_f32_le();
            _struct.zgyro = buf.get_f32_le();
            _struct.lat = buf.get_f32_le();
            _struct.lon = buf.get_f32_le();
            _struct.alt = buf.get_f32_le();
            _struct.std_dev_horz = buf.get_f32_le();
            _struct.std_dev_vert = buf.get_f32_le();
            _struct.vn = buf.get_f32_le();
            _struct.ve = buf.get_f32_le();
            _struct.vd = buf.get_f32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.q1 as f32);
        _tmp.put_f32_le(self.q2 as f32);
        _tmp.put_f32_le(self.q3 as f32);
        _tmp.put_f32_le(self.q4 as f32);
        _tmp.put_f32_le(self.roll as f32);
        _tmp.put_f32_le(self.pitch as f32);
        _tmp.put_f32_le(self.yaw as f32);
        _tmp.put_f32_le(self.xacc as f32);
        _tmp.put_f32_le(self.yacc as f32);
        _tmp.put_f32_le(self.zacc as f32);
        _tmp.put_f32_le(self.xgyro as f32);
        _tmp.put_f32_le(self.ygyro as f32);
        _tmp.put_f32_le(self.zgyro as f32);
        _tmp.put_f32_le(self.lat as f32);
        _tmp.put_f32_le(self.lon as f32);
        _tmp.put_f32_le(self.alt as f32);
        _tmp.put_f32_le(self.std_dev_horz as f32);
        _tmp.put_f32_le(self.std_dev_vert as f32);
        _tmp.put_f32_le(self.vn as f32);
        _tmp.put_f32_le(self.ve as f32);
        _tmp.put_f32_le(self.vd as f32);
        _tmp
    }
}
impl crate::proto::common::RadioStatus {
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
impl crate::proto::common::FileTransferProtocol {
    pub const ENCODED_LEN: usize = 254usize;
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
            _struct.target_network = buf.get_u8() as u32;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            for _ in 0..251usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.payload.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.target_network as u8);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        for val in &self.payload {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
impl crate::proto::common::Timesync {
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
            _struct.tc1 = buf.get_i64_le();
            _struct.ts1 = buf.get_i64_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i64_le(self.tc1 as i64);
        _tmp.put_i64_le(self.ts1 as i64);
        _tmp
    }
}
impl crate::proto::common::CameraTrigger {
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
            _struct.time_usec = buf.get_u64_le();
            _struct.seq = buf.get_u32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        _tmp.put_u32_le(self.seq as u32);
        _tmp
    }
}
impl crate::proto::common::HilGps {
    pub const ENCODED_LEN: usize = 36usize;
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
            _struct.lon = buf.get_i32_le();
            _struct.alt = buf.get_i32_le();
            _struct.eph = buf.get_u16_le() as u32;
            _struct.epv = buf.get_u16_le() as u32;
            _struct.vel = buf.get_u16_le() as u32;
            _struct.vn = buf.get_i16_le() as i32;
            _struct.ve = buf.get_i16_le() as i32;
            _struct.vd = buf.get_i16_le() as i32;
            _struct.cog = buf.get_u16_le() as u32;
            _struct.fix_type = buf.get_u8() as u32;
            _struct.satellites_visible = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        _tmp.put_i32_le(self.lat as i32);
        _tmp.put_i32_le(self.lon as i32);
        _tmp.put_i32_le(self.alt as i32);
        _tmp.put_u16_le(self.eph as u16);
        _tmp.put_u16_le(self.epv as u16);
        _tmp.put_u16_le(self.vel as u16);
        _tmp.put_i16_le(self.vn as i16);
        _tmp.put_i16_le(self.ve as i16);
        _tmp.put_i16_le(self.vd as i16);
        _tmp.put_u16_le(self.cog as u16);
        _tmp.put_u8(self.fix_type as u8);
        _tmp.put_u8(self.satellites_visible as u8);
        _tmp
    }
}
impl crate::proto::common::HilOpticalFlow {
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
            _struct.integration_time_us = buf.get_u32_le();
            _struct.integrated_x = buf.get_f32_le();
            _struct.integrated_y = buf.get_f32_le();
            _struct.integrated_xgyro = buf.get_f32_le();
            _struct.integrated_ygyro = buf.get_f32_le();
            _struct.integrated_zgyro = buf.get_f32_le();
            _struct.time_delta_distance_us = buf.get_u32_le();
            _struct.distance = buf.get_f32_le();
            _struct.temperature = buf.get_i16_le() as i32;
            _struct.sensor_id = buf.get_u8() as u32;
            _struct.quality = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        _tmp.put_u32_le(self.integration_time_us as u32);
        _tmp.put_f32_le(self.integrated_x as f32);
        _tmp.put_f32_le(self.integrated_y as f32);
        _tmp.put_f32_le(self.integrated_xgyro as f32);
        _tmp.put_f32_le(self.integrated_ygyro as f32);
        _tmp.put_f32_le(self.integrated_zgyro as f32);
        _tmp.put_u32_le(self.time_delta_distance_us as u32);
        _tmp.put_f32_le(self.distance as f32);
        _tmp.put_i16_le(self.temperature as i16);
        _tmp.put_u8(self.sensor_id as u8);
        _tmp.put_u8(self.quality as u8);
        _tmp
    }
}
impl crate::proto::common::HilStateQuaternion {
    pub const ENCODED_LEN: usize = 64usize;
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
            for _ in 0..4usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.attitude_quaternion.push(val.into());
            }
            _struct.rollspeed = buf.get_f32_le();
            _struct.pitchspeed = buf.get_f32_le();
            _struct.yawspeed = buf.get_f32_le();
            _struct.lat = buf.get_i32_le();
            _struct.lon = buf.get_i32_le();
            _struct.alt = buf.get_i32_le();
            _struct.vx = buf.get_i16_le() as i32;
            _struct.vy = buf.get_i16_le() as i32;
            _struct.vz = buf.get_i16_le() as i32;
            _struct.ind_airspeed = buf.get_u16_le() as u32;
            _struct.true_airspeed = buf.get_u16_le() as u32;
            _struct.xacc = buf.get_i16_le() as i32;
            _struct.yacc = buf.get_i16_le() as i32;
            _struct.zacc = buf.get_i16_le() as i32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        for val in &self.attitude_quaternion {
            _tmp.put_f32_le(*val as f32);
        }
        _tmp.put_f32_le(self.rollspeed as f32);
        _tmp.put_f32_le(self.pitchspeed as f32);
        _tmp.put_f32_le(self.yawspeed as f32);
        _tmp.put_i32_le(self.lat as i32);
        _tmp.put_i32_le(self.lon as i32);
        _tmp.put_i32_le(self.alt as i32);
        _tmp.put_i16_le(self.vx as i16);
        _tmp.put_i16_le(self.vy as i16);
        _tmp.put_i16_le(self.vz as i16);
        _tmp.put_u16_le(self.ind_airspeed as u16);
        _tmp.put_u16_le(self.true_airspeed as u16);
        _tmp.put_i16_le(self.xacc as i16);
        _tmp.put_i16_le(self.yacc as i16);
        _tmp.put_i16_le(self.zacc as i16);
        _tmp
    }
}
impl crate::proto::common::ScaledImu2 {
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
            _struct.time_boot_ms = buf.get_u32_le();
            _struct.xacc = buf.get_i16_le() as i32;
            _struct.yacc = buf.get_i16_le() as i32;
            _struct.zacc = buf.get_i16_le() as i32;
            _struct.xgyro = buf.get_i16_le() as i32;
            _struct.ygyro = buf.get_i16_le() as i32;
            _struct.zgyro = buf.get_i16_le() as i32;
            _struct.xmag = buf.get_i16_le() as i32;
            _struct.ymag = buf.get_i16_le() as i32;
            _struct.zmag = buf.get_i16_le() as i32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms as u32);
        _tmp.put_i16_le(self.xacc as i16);
        _tmp.put_i16_le(self.yacc as i16);
        _tmp.put_i16_le(self.zacc as i16);
        _tmp.put_i16_le(self.xgyro as i16);
        _tmp.put_i16_le(self.ygyro as i16);
        _tmp.put_i16_le(self.zgyro as i16);
        _tmp.put_i16_le(self.xmag as i16);
        _tmp.put_i16_le(self.ymag as i16);
        _tmp.put_i16_le(self.zmag as i16);
        _tmp
    }
}
impl crate::proto::common::LogRequestList {
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
            _struct.start = buf.get_u16_le() as u32;
            _struct.end = buf.get_u16_le() as u32;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.start as u16);
        _tmp.put_u16_le(self.end as u16);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp
    }
}
impl crate::proto::common::LogEntry {
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
            _struct.time_utc = buf.get_u32_le();
            _struct.size = buf.get_u32_le();
            _struct.id = buf.get_u16_le() as u32;
            _struct.num_logs = buf.get_u16_le() as u32;
            _struct.last_log_num = buf.get_u16_le() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_utc as u32);
        _tmp.put_u32_le(self.size as u32);
        _tmp.put_u16_le(self.id as u16);
        _tmp.put_u16_le(self.num_logs as u16);
        _tmp.put_u16_le(self.last_log_num as u16);
        _tmp
    }
}
impl crate::proto::common::LogRequestData {
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
            _struct.ofs = buf.get_u32_le();
            _struct.count = buf.get_u32_le();
            _struct.id = buf.get_u16_le() as u32;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.ofs as u32);
        _tmp.put_u32_le(self.count as u32);
        _tmp.put_u16_le(self.id as u16);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp
    }
}
impl crate::proto::common::LogData {
    pub const ENCODED_LEN: usize = 97usize;
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
            _struct.ofs = buf.get_u32_le();
            _struct.id = buf.get_u16_le() as u32;
            _struct.count = buf.get_u8() as u32;
            for _ in 0..90usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.data.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.ofs as u32);
        _tmp.put_u16_le(self.id as u16);
        _tmp.put_u8(self.count as u8);
        for val in &self.data {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
impl crate::proto::common::LogErase {
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
impl crate::proto::common::LogRequestEnd {
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
impl crate::proto::common::GpsInjectData {
    pub const ENCODED_LEN: usize = 113usize;
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
            _struct.len = buf.get_u8() as u32;
            for _ in 0..110usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.data.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp.put_u8(self.len as u8);
        for val in &self.data {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
impl crate::proto::common::Gps2Raw {
    pub const ENCODED_LEN: usize = 35usize;
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
            _struct.lon = buf.get_i32_le();
            _struct.alt = buf.get_i32_le();
            _struct.dgps_age = buf.get_u32_le();
            _struct.eph = buf.get_u16_le() as u32;
            _struct.epv = buf.get_u16_le() as u32;
            _struct.vel = buf.get_u16_le() as u32;
            _struct.cog = buf.get_u16_le() as u32;
            let tmp = buf.get_u8();
            _struct.fix_type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "GpsFixType".to_string(),
                value: tmp as u32,
            })?;
            _struct.satellites_visible = buf.get_u8() as u32;
            _struct.dgps_numch = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        _tmp.put_i32_le(self.lat as i32);
        _tmp.put_i32_le(self.lon as i32);
        _tmp.put_i32_le(self.alt as i32);
        _tmp.put_u32_le(self.dgps_age as u32);
        _tmp.put_u16_le(self.eph as u16);
        _tmp.put_u16_le(self.epv as u16);
        _tmp.put_u16_le(self.vel as u16);
        _tmp.put_u16_le(self.cog as u16);
        _tmp.put_u8(self.fix_type as u8);
        _tmp.put_u8(self.satellites_visible as u8);
        _tmp.put_u8(self.dgps_numch as u8);
        _tmp
    }
}
impl crate::proto::common::PowerStatus {
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
            _struct.vcc = buf.get_u16_le() as u32;
            _struct.vservo = buf.get_u16_le() as u32;
            let tmp = buf.get_u16_le();
            _struct.flags = FromPrimitive::from_u16(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavPowerStatus".to_string(),
                value: tmp as u32,
            })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.vcc as u16);
        _tmp.put_u16_le(self.vservo as u16);
        _tmp.put_u16_le(self.flags as u16);
        _tmp
    }
}
impl crate::proto::common::SerialControl {
    pub const ENCODED_LEN: usize = 79usize;
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
            _struct.baudrate = buf.get_u32_le();
            _struct.timeout = buf.get_u16_le() as u32;
            let tmp = buf.get_u8();
            _struct.device = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "SerialControlDev".to_string(),
                value: tmp as u32,
            })?;
            let tmp = buf.get_u8();
            _struct.flags = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "SerialControlFlag".to_string(),
                value: tmp as u32,
            })?;
            _struct.count = buf.get_u8() as u32;
            for _ in 0..70usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.data.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.baudrate as u32);
        _tmp.put_u16_le(self.timeout as u16);
        _tmp.put_u8(self.device as u8);
        _tmp.put_u8(self.flags as u8);
        _tmp.put_u8(self.count as u8);
        for val in &self.data {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
impl crate::proto::common::GpsRtk {
    pub const ENCODED_LEN: usize = 35usize;
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
            _struct.time_last_baseline_ms = buf.get_u32_le();
            _struct.tow = buf.get_u32_le();
            _struct.baseline_a_mm = buf.get_i32_le();
            _struct.baseline_b_mm = buf.get_i32_le();
            _struct.baseline_c_mm = buf.get_i32_le();
            _struct.accuracy = buf.get_u32_le();
            _struct.iar_num_hypotheses = buf.get_i32_le();
            _struct.wn = buf.get_u16_le() as u32;
            _struct.rtk_receiver_id = buf.get_u8() as u32;
            _struct.rtk_health = buf.get_u8() as u32;
            _struct.rtk_rate = buf.get_u8() as u32;
            _struct.nsats = buf.get_u8() as u32;
            let tmp = buf.get_u8();
            _struct.baseline_coords_type =
                FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "RtkBaselineCoordinateSystem".to_string(),
                    value: tmp as u32,
                })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_last_baseline_ms as u32);
        _tmp.put_u32_le(self.tow as u32);
        _tmp.put_i32_le(self.baseline_a_mm as i32);
        _tmp.put_i32_le(self.baseline_b_mm as i32);
        _tmp.put_i32_le(self.baseline_c_mm as i32);
        _tmp.put_u32_le(self.accuracy as u32);
        _tmp.put_i32_le(self.iar_num_hypotheses as i32);
        _tmp.put_u16_le(self.wn as u16);
        _tmp.put_u8(self.rtk_receiver_id as u8);
        _tmp.put_u8(self.rtk_health as u8);
        _tmp.put_u8(self.rtk_rate as u8);
        _tmp.put_u8(self.nsats as u8);
        _tmp.put_u8(self.baseline_coords_type as u8);
        _tmp
    }
}
impl crate::proto::common::Gps2Rtk {
    pub const ENCODED_LEN: usize = 35usize;
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
            _struct.time_last_baseline_ms = buf.get_u32_le();
            _struct.tow = buf.get_u32_le();
            _struct.baseline_a_mm = buf.get_i32_le();
            _struct.baseline_b_mm = buf.get_i32_le();
            _struct.baseline_c_mm = buf.get_i32_le();
            _struct.accuracy = buf.get_u32_le();
            _struct.iar_num_hypotheses = buf.get_i32_le();
            _struct.wn = buf.get_u16_le() as u32;
            _struct.rtk_receiver_id = buf.get_u8() as u32;
            _struct.rtk_health = buf.get_u8() as u32;
            _struct.rtk_rate = buf.get_u8() as u32;
            _struct.nsats = buf.get_u8() as u32;
            let tmp = buf.get_u8();
            _struct.baseline_coords_type =
                FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "RtkBaselineCoordinateSystem".to_string(),
                    value: tmp as u32,
                })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_last_baseline_ms as u32);
        _tmp.put_u32_le(self.tow as u32);
        _tmp.put_i32_le(self.baseline_a_mm as i32);
        _tmp.put_i32_le(self.baseline_b_mm as i32);
        _tmp.put_i32_le(self.baseline_c_mm as i32);
        _tmp.put_u32_le(self.accuracy as u32);
        _tmp.put_i32_le(self.iar_num_hypotheses as i32);
        _tmp.put_u16_le(self.wn as u16);
        _tmp.put_u8(self.rtk_receiver_id as u8);
        _tmp.put_u8(self.rtk_health as u8);
        _tmp.put_u8(self.rtk_rate as u8);
        _tmp.put_u8(self.nsats as u8);
        _tmp.put_u8(self.baseline_coords_type as u8);
        _tmp
    }
}
impl crate::proto::common::ScaledImu3 {
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
            _struct.time_boot_ms = buf.get_u32_le();
            _struct.xacc = buf.get_i16_le() as i32;
            _struct.yacc = buf.get_i16_le() as i32;
            _struct.zacc = buf.get_i16_le() as i32;
            _struct.xgyro = buf.get_i16_le() as i32;
            _struct.ygyro = buf.get_i16_le() as i32;
            _struct.zgyro = buf.get_i16_le() as i32;
            _struct.xmag = buf.get_i16_le() as i32;
            _struct.ymag = buf.get_i16_le() as i32;
            _struct.zmag = buf.get_i16_le() as i32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms as u32);
        _tmp.put_i16_le(self.xacc as i16);
        _tmp.put_i16_le(self.yacc as i16);
        _tmp.put_i16_le(self.zacc as i16);
        _tmp.put_i16_le(self.xgyro as i16);
        _tmp.put_i16_le(self.ygyro as i16);
        _tmp.put_i16_le(self.zgyro as i16);
        _tmp.put_i16_le(self.xmag as i16);
        _tmp.put_i16_le(self.ymag as i16);
        _tmp.put_i16_le(self.zmag as i16);
        _tmp
    }
}
impl crate::proto::common::DataTransmissionHandshake {
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
            _struct.size = buf.get_u32_le();
            _struct.width = buf.get_u16_le() as u32;
            _struct.height = buf.get_u16_le() as u32;
            _struct.packets = buf.get_u16_le() as u32;
            let tmp = buf.get_u8();
            _struct.r#type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavlinkDataStreamType".to_string(),
                value: tmp as u32,
            })?;
            _struct.payload = buf.get_u8() as u32;
            _struct.jpg_quality = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.size as u32);
        _tmp.put_u16_le(self.width as u16);
        _tmp.put_u16_le(self.height as u16);
        _tmp.put_u16_le(self.packets as u16);
        _tmp.put_u8(self.r#type as u8);
        _tmp.put_u8(self.payload as u8);
        _tmp.put_u8(self.jpg_quality as u8);
        _tmp
    }
}
impl crate::proto::common::EncapsulatedData {
    pub const ENCODED_LEN: usize = 255usize;
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
            _struct.seqnr = buf.get_u16_le() as u32;
            for _ in 0..253usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.data.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.seqnr as u16);
        for val in &self.data {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
impl crate::proto::common::DistanceSensor {
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
            _struct.time_boot_ms = buf.get_u32_le();
            _struct.min_distance = buf.get_u16_le() as u32;
            _struct.max_distance = buf.get_u16_le() as u32;
            _struct.current_distance = buf.get_u16_le() as u32;
            let tmp = buf.get_u8();
            _struct.r#type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavDistanceSensor".to_string(),
                value: tmp as u32,
            })?;
            _struct.id = buf.get_u8() as u32;
            let tmp = buf.get_u8();
            _struct.orientation = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavSensorOrientation".to_string(),
                value: tmp as u32,
            })?;
            _struct.covariance = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms as u32);
        _tmp.put_u16_le(self.min_distance as u16);
        _tmp.put_u16_le(self.max_distance as u16);
        _tmp.put_u16_le(self.current_distance as u16);
        _tmp.put_u8(self.r#type as u8);
        _tmp.put_u8(self.id as u8);
        _tmp.put_u8(self.orientation as u8);
        _tmp.put_u8(self.covariance as u8);
        _tmp
    }
}
impl crate::proto::common::TerrainRequest {
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
            _struct.mask = buf.get_u64_le();
            _struct.lat = buf.get_i32_le();
            _struct.lon = buf.get_i32_le();
            _struct.grid_spacing = buf.get_u16_le() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.mask as u64);
        _tmp.put_i32_le(self.lat as i32);
        _tmp.put_i32_le(self.lon as i32);
        _tmp.put_u16_le(self.grid_spacing as u16);
        _tmp
    }
}
impl crate::proto::common::TerrainData {
    pub const ENCODED_LEN: usize = 43usize;
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
            _struct.lon = buf.get_i32_le();
            _struct.grid_spacing = buf.get_u16_le() as u32;
            for _ in 0..16usize {
                let val = buf.get_i16_le() as i32;
                #[allow(clippy::useless_conversion)]
                _struct.data.push(val.into());
            }
            _struct.gridbit = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i32_le(self.lat as i32);
        _tmp.put_i32_le(self.lon as i32);
        _tmp.put_u16_le(self.grid_spacing as u16);
        for val in &self.data {
            _tmp.put_i16_le(*val as i16);
        }
        _tmp.put_u8(self.gridbit as u8);
        _tmp
    }
}
impl crate::proto::common::TerrainCheck {
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
            _struct.lat = buf.get_i32_le();
            _struct.lon = buf.get_i32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i32_le(self.lat as i32);
        _tmp.put_i32_le(self.lon as i32);
        _tmp
    }
}
impl crate::proto::common::TerrainReport {
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
            _struct.lat = buf.get_i32_le();
            _struct.lon = buf.get_i32_le();
            _struct.terrain_height = buf.get_f32_le();
            _struct.current_height = buf.get_f32_le();
            _struct.spacing = buf.get_u16_le() as u32;
            _struct.pending = buf.get_u16_le() as u32;
            _struct.loaded = buf.get_u16_le() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i32_le(self.lat as i32);
        _tmp.put_i32_le(self.lon as i32);
        _tmp.put_f32_le(self.terrain_height as f32);
        _tmp.put_f32_le(self.current_height as f32);
        _tmp.put_u16_le(self.spacing as u16);
        _tmp.put_u16_le(self.pending as u16);
        _tmp.put_u16_le(self.loaded as u16);
        _tmp
    }
}
impl crate::proto::common::ScaledPressure2 {
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
            _struct.time_boot_ms = buf.get_u32_le();
            _struct.press_abs = buf.get_f32_le();
            _struct.press_diff = buf.get_f32_le();
            _struct.temperature = buf.get_i16_le() as i32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms as u32);
        _tmp.put_f32_le(self.press_abs as f32);
        _tmp.put_f32_le(self.press_diff as f32);
        _tmp.put_i16_le(self.temperature as i16);
        _tmp
    }
}
impl crate::proto::common::AttPosMocap {
    pub const ENCODED_LEN: usize = 36usize;
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
            for _ in 0..4usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.q.push(val.into());
            }
            _struct.x = buf.get_f32_le();
            _struct.y = buf.get_f32_le();
            _struct.z = buf.get_f32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        for val in &self.q {
            _tmp.put_f32_le(*val as f32);
        }
        _tmp.put_f32_le(self.x as f32);
        _tmp.put_f32_le(self.y as f32);
        _tmp.put_f32_le(self.z as f32);
        _tmp
    }
}
impl crate::proto::common::SetActuatorControlTarget {
    pub const ENCODED_LEN: usize = 43usize;
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
            for _ in 0..8usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.controls.push(val.into());
            }
            _struct.group_mlx = buf.get_u8() as u32;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        for val in &self.controls {
            _tmp.put_f32_le(*val as f32);
        }
        _tmp.put_u8(self.group_mlx as u8);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp
    }
}
impl crate::proto::common::ActuatorControlTarget {
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
            _struct.time_usec = buf.get_u64_le();
            for _ in 0..8usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.controls.push(val.into());
            }
            _struct.group_mlx = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        for val in &self.controls {
            _tmp.put_f32_le(*val as f32);
        }
        _tmp.put_u8(self.group_mlx as u8);
        _tmp
    }
}
impl crate::proto::common::Altitude {
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
            _struct.time_usec = buf.get_u64_le();
            _struct.altitude_monotonic = buf.get_f32_le();
            _struct.altitude_amsl = buf.get_f32_le();
            _struct.altitude_local = buf.get_f32_le();
            _struct.altitude_relative = buf.get_f32_le();
            _struct.altitude_terrain = buf.get_f32_le();
            _struct.bottom_clearance = buf.get_f32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        _tmp.put_f32_le(self.altitude_monotonic as f32);
        _tmp.put_f32_le(self.altitude_amsl as f32);
        _tmp.put_f32_le(self.altitude_local as f32);
        _tmp.put_f32_le(self.altitude_relative as f32);
        _tmp.put_f32_le(self.altitude_terrain as f32);
        _tmp.put_f32_le(self.bottom_clearance as f32);
        _tmp
    }
}
impl crate::proto::common::ResourceRequest {
    pub const ENCODED_LEN: usize = 243usize;
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
            _struct.request_id = buf.get_u8() as u32;
            _struct.uri_type = buf.get_u8() as u32;
            for _ in 0..120usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.uri.push(val.into());
            }
            _struct.transfer_type = buf.get_u8() as u32;
            for _ in 0..120usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.storage.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.request_id as u8);
        _tmp.put_u8(self.uri_type as u8);
        for val in &self.uri {
            _tmp.put_u8(*val as u8);
        }
        _tmp.put_u8(self.transfer_type as u8);
        for val in &self.storage {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
impl crate::proto::common::ScaledPressure3 {
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
            _struct.time_boot_ms = buf.get_u32_le();
            _struct.press_abs = buf.get_f32_le();
            _struct.press_diff = buf.get_f32_le();
            _struct.temperature = buf.get_i16_le() as i32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms as u32);
        _tmp.put_f32_le(self.press_abs as f32);
        _tmp.put_f32_le(self.press_diff as f32);
        _tmp.put_i16_le(self.temperature as i16);
        _tmp
    }
}
impl crate::proto::common::FollowTarget {
    pub const ENCODED_LEN: usize = 93usize;
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
            _struct.custom_state = buf.get_u64_le();
            _struct.lat = buf.get_i32_le();
            _struct.lon = buf.get_i32_le();
            _struct.alt = buf.get_f32_le();
            for _ in 0..3usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.vel.push(val.into());
            }
            for _ in 0..3usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.acc.push(val.into());
            }
            for _ in 0..4usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.attitude_q.push(val.into());
            }
            for _ in 0..3usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.rates.push(val.into());
            }
            for _ in 0..3usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.position_cov.push(val.into());
            }
            _struct.est_capabilities = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.timestamp as u64);
        _tmp.put_u64_le(self.custom_state as u64);
        _tmp.put_i32_le(self.lat as i32);
        _tmp.put_i32_le(self.lon as i32);
        _tmp.put_f32_le(self.alt as f32);
        for val in &self.vel {
            _tmp.put_f32_le(*val as f32);
        }
        for val in &self.acc {
            _tmp.put_f32_le(*val as f32);
        }
        for val in &self.attitude_q {
            _tmp.put_f32_le(*val as f32);
        }
        for val in &self.rates {
            _tmp.put_f32_le(*val as f32);
        }
        for val in &self.position_cov {
            _tmp.put_f32_le(*val as f32);
        }
        _tmp.put_u8(self.est_capabilities as u8);
        _tmp
    }
}
impl crate::proto::common::ControlSystemState {
    pub const ENCODED_LEN: usize = 100usize;
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
            _struct.x_acc = buf.get_f32_le();
            _struct.y_acc = buf.get_f32_le();
            _struct.z_acc = buf.get_f32_le();
            _struct.x_vel = buf.get_f32_le();
            _struct.y_vel = buf.get_f32_le();
            _struct.z_vel = buf.get_f32_le();
            _struct.x_pos = buf.get_f32_le();
            _struct.y_pos = buf.get_f32_le();
            _struct.z_pos = buf.get_f32_le();
            _struct.airspeed = buf.get_f32_le();
            for _ in 0..3usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.vel_variance.push(val.into());
            }
            for _ in 0..3usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.pos_variance.push(val.into());
            }
            for _ in 0..4usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.q.push(val.into());
            }
            _struct.roll_rate = buf.get_f32_le();
            _struct.pitch_rate = buf.get_f32_le();
            _struct.yaw_rate = buf.get_f32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        _tmp.put_f32_le(self.x_acc as f32);
        _tmp.put_f32_le(self.y_acc as f32);
        _tmp.put_f32_le(self.z_acc as f32);
        _tmp.put_f32_le(self.x_vel as f32);
        _tmp.put_f32_le(self.y_vel as f32);
        _tmp.put_f32_le(self.z_vel as f32);
        _tmp.put_f32_le(self.x_pos as f32);
        _tmp.put_f32_le(self.y_pos as f32);
        _tmp.put_f32_le(self.z_pos as f32);
        _tmp.put_f32_le(self.airspeed as f32);
        for val in &self.vel_variance {
            _tmp.put_f32_le(*val as f32);
        }
        for val in &self.pos_variance {
            _tmp.put_f32_le(*val as f32);
        }
        for val in &self.q {
            _tmp.put_f32_le(*val as f32);
        }
        _tmp.put_f32_le(self.roll_rate as f32);
        _tmp.put_f32_le(self.pitch_rate as f32);
        _tmp.put_f32_le(self.yaw_rate as f32);
        _tmp
    }
}
impl crate::proto::common::BatteryStatus {
    pub const ENCODED_LEN: usize = 36usize;
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
            _struct.current_consumed = buf.get_i32_le();
            _struct.energy_consumed = buf.get_i32_le();
            _struct.temperature = buf.get_i16_le() as i32;
            for _ in 0..10usize {
                let val = buf.get_u16_le() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.voltages.push(val.into());
            }
            _struct.current_battery = buf.get_i16_le() as i32;
            _struct.id = buf.get_u8() as u32;
            let tmp = buf.get_u8();
            _struct.battery_function =
                FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "MavBatteryFunction".to_string(),
                    value: tmp as u32,
                })?;
            let tmp = buf.get_u8();
            _struct.r#type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavBatteryType".to_string(),
                value: tmp as u32,
            })?;
            _struct.battery_remaining = buf.get_i8() as i32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i32_le(self.current_consumed as i32);
        _tmp.put_i32_le(self.energy_consumed as i32);
        _tmp.put_i16_le(self.temperature as i16);
        for val in &self.voltages {
            _tmp.put_u16_le(*val as u16);
        }
        _tmp.put_i16_le(self.current_battery as i16);
        _tmp.put_u8(self.id as u8);
        _tmp.put_u8(self.battery_function as u8);
        _tmp.put_u8(self.r#type as u8);
        _tmp.put_i8(self.battery_remaining as i8);
        _tmp
    }
}
impl crate::proto::common::AutopilotVersion {
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
            let tmp = buf.get_u64_le();
            _struct.capabilities =
                FromPrimitive::from_u64(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "MavProtocolCapability".to_string(),
                    value: tmp as u32,
                })?;
            _struct.uid = buf.get_u64_le();
            _struct.flight_sw_version = buf.get_u32_le();
            _struct.middleware_sw_version = buf.get_u32_le();
            _struct.os_sw_version = buf.get_u32_le();
            _struct.board_version = buf.get_u32_le();
            _struct.vendor_id = buf.get_u16_le() as u32;
            _struct.product_id = buf.get_u16_le() as u32;
            for _ in 0..8usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.flight_custom_version.push(val.into());
            }
            for _ in 0..8usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.middleware_custom_version.push(val.into());
            }
            for _ in 0..8usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.os_custom_version.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.capabilities as u64);
        _tmp.put_u64_le(self.uid as u64);
        _tmp.put_u32_le(self.flight_sw_version as u32);
        _tmp.put_u32_le(self.middleware_sw_version as u32);
        _tmp.put_u32_le(self.os_sw_version as u32);
        _tmp.put_u32_le(self.board_version as u32);
        _tmp.put_u16_le(self.vendor_id as u16);
        _tmp.put_u16_le(self.product_id as u16);
        for val in &self.flight_custom_version {
            _tmp.put_u8(*val as u8);
        }
        for val in &self.middleware_custom_version {
            _tmp.put_u8(*val as u8);
        }
        for val in &self.os_custom_version {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
impl crate::proto::common::LandingTarget {
    pub const ENCODED_LEN: usize = 30usize;
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
            _struct.angle_x = buf.get_f32_le();
            _struct.angle_y = buf.get_f32_le();
            _struct.distance = buf.get_f32_le();
            _struct.size_x = buf.get_f32_le();
            _struct.size_y = buf.get_f32_le();
            _struct.target_num = buf.get_u8() as u32;
            let tmp = buf.get_u8();
            _struct.frame = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavFrame".to_string(),
                value: tmp as u32,
            })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        _tmp.put_f32_le(self.angle_x as f32);
        _tmp.put_f32_le(self.angle_y as f32);
        _tmp.put_f32_le(self.distance as f32);
        _tmp.put_f32_le(self.size_x as f32);
        _tmp.put_f32_le(self.size_y as f32);
        _tmp.put_u8(self.target_num as u8);
        _tmp.put_u8(self.frame as u8);
        _tmp
    }
}
impl crate::proto::common::FenceStatus {
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
            _struct.breach_time = buf.get_u32_le();
            _struct.breach_count = buf.get_u16_le() as u32;
            _struct.breach_status = buf.get_u8() as u32;
            let tmp = buf.get_u8();
            _struct.breach_type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "FenceBreach".to_string(),
                value: tmp as u32,
            })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.breach_time as u32);
        _tmp.put_u16_le(self.breach_count as u16);
        _tmp.put_u8(self.breach_status as u8);
        _tmp.put_u8(self.breach_type as u8);
        _tmp
    }
}
impl crate::proto::common::EstimatorStatus {
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
            _struct.time_usec = buf.get_u64_le();
            _struct.vel_ratio = buf.get_f32_le();
            _struct.pos_horiz_ratio = buf.get_f32_le();
            _struct.pos_vert_ratio = buf.get_f32_le();
            _struct.mag_ratio = buf.get_f32_le();
            _struct.hagl_ratio = buf.get_f32_le();
            _struct.tas_ratio = buf.get_f32_le();
            _struct.pos_horiz_accuracy = buf.get_f32_le();
            _struct.pos_vert_accuracy = buf.get_f32_le();
            let tmp = buf.get_u16_le();
            _struct.flags = FromPrimitive::from_u16(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "EstimatorStatusFlags".to_string(),
                value: tmp as u32,
            })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        _tmp.put_f32_le(self.vel_ratio as f32);
        _tmp.put_f32_le(self.pos_horiz_ratio as f32);
        _tmp.put_f32_le(self.pos_vert_ratio as f32);
        _tmp.put_f32_le(self.mag_ratio as f32);
        _tmp.put_f32_le(self.hagl_ratio as f32);
        _tmp.put_f32_le(self.tas_ratio as f32);
        _tmp.put_f32_le(self.pos_horiz_accuracy as f32);
        _tmp.put_f32_le(self.pos_vert_accuracy as f32);
        _tmp.put_u16_le(self.flags as u16);
        _tmp
    }
}
impl crate::proto::common::WindCov {
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
            _struct.time_usec = buf.get_u64_le();
            _struct.wind_x = buf.get_f32_le();
            _struct.wind_y = buf.get_f32_le();
            _struct.wind_z = buf.get_f32_le();
            _struct.var_horiz = buf.get_f32_le();
            _struct.var_vert = buf.get_f32_le();
            _struct.wind_alt = buf.get_f32_le();
            _struct.horiz_accuracy = buf.get_f32_le();
            _struct.vert_accuracy = buf.get_f32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        _tmp.put_f32_le(self.wind_x as f32);
        _tmp.put_f32_le(self.wind_y as f32);
        _tmp.put_f32_le(self.wind_z as f32);
        _tmp.put_f32_le(self.var_horiz as f32);
        _tmp.put_f32_le(self.var_vert as f32);
        _tmp.put_f32_le(self.wind_alt as f32);
        _tmp.put_f32_le(self.horiz_accuracy as f32);
        _tmp.put_f32_le(self.vert_accuracy as f32);
        _tmp
    }
}
impl crate::proto::common::GpsInput {
    pub const ENCODED_LEN: usize = 63usize;
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
            _struct.time_week_ms = buf.get_u32_le();
            _struct.lat = buf.get_i32_le();
            _struct.lon = buf.get_i32_le();
            _struct.alt = buf.get_f32_le();
            _struct.hdop = buf.get_f32_le();
            _struct.vdop = buf.get_f32_le();
            _struct.vn = buf.get_f32_le();
            _struct.ve = buf.get_f32_le();
            _struct.vd = buf.get_f32_le();
            _struct.speed_accuracy = buf.get_f32_le();
            _struct.horiz_accuracy = buf.get_f32_le();
            _struct.vert_accuracy = buf.get_f32_le();
            let tmp = buf.get_u16_le();
            _struct.ignore_flags =
                FromPrimitive::from_u16(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "GpsInputIgnoreFlags".to_string(),
                    value: tmp as u32,
                })?;
            _struct.time_week = buf.get_u16_le() as u32;
            _struct.gps_id = buf.get_u8() as u32;
            _struct.fix_type = buf.get_u8() as u32;
            _struct.satellites_visible = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        _tmp.put_u32_le(self.time_week_ms as u32);
        _tmp.put_i32_le(self.lat as i32);
        _tmp.put_i32_le(self.lon as i32);
        _tmp.put_f32_le(self.alt as f32);
        _tmp.put_f32_le(self.hdop as f32);
        _tmp.put_f32_le(self.vdop as f32);
        _tmp.put_f32_le(self.vn as f32);
        _tmp.put_f32_le(self.ve as f32);
        _tmp.put_f32_le(self.vd as f32);
        _tmp.put_f32_le(self.speed_accuracy as f32);
        _tmp.put_f32_le(self.horiz_accuracy as f32);
        _tmp.put_f32_le(self.vert_accuracy as f32);
        _tmp.put_u16_le(self.ignore_flags as u16);
        _tmp.put_u16_le(self.time_week as u16);
        _tmp.put_u8(self.gps_id as u8);
        _tmp.put_u8(self.fix_type as u8);
        _tmp.put_u8(self.satellites_visible as u8);
        _tmp
    }
}
impl crate::proto::common::GpsRtcmData {
    pub const ENCODED_LEN: usize = 182usize;
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
            _struct.flags = buf.get_u8() as u32;
            _struct.len = buf.get_u8() as u32;
            for _ in 0..180usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.data.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.flags as u8);
        _tmp.put_u8(self.len as u8);
        for val in &self.data {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
impl crate::proto::common::HighLatency {
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
            _struct.custom_mode = buf.get_u32_le();
            _struct.latitude = buf.get_i32_le();
            _struct.longitude = buf.get_i32_le();
            _struct.roll = buf.get_i16_le() as i32;
            _struct.pitch = buf.get_i16_le() as i32;
            _struct.heading = buf.get_u16_le() as u32;
            _struct.heading_sp = buf.get_i16_le() as i32;
            _struct.altitude_amsl = buf.get_i16_le() as i32;
            _struct.altitude_sp = buf.get_i16_le() as i32;
            _struct.wp_distance = buf.get_u16_le() as u32;
            let tmp = buf.get_u8();
            _struct.base_mode = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavModeFlag".to_string(),
                value: tmp as u32,
            })?;
            let tmp = buf.get_u8();
            _struct.landed_state = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavLandedState".to_string(),
                value: tmp as u32,
            })?;
            _struct.throttle = buf.get_i8() as i32;
            _struct.airspeed = buf.get_u8() as u32;
            _struct.airspeed_sp = buf.get_u8() as u32;
            _struct.groundspeed = buf.get_u8() as u32;
            _struct.climb_rate = buf.get_i8() as i32;
            _struct.gps_nsat = buf.get_u8() as u32;
            let tmp = buf.get_u8();
            _struct.gps_fix_type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "GpsFixType".to_string(),
                value: tmp as u32,
            })?;
            _struct.battery_remaining = buf.get_u8() as u32;
            _struct.temperature = buf.get_i8() as i32;
            _struct.temperature_air = buf.get_i8() as i32;
            _struct.failsafe = buf.get_u8() as u32;
            _struct.wp_num = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.custom_mode as u32);
        _tmp.put_i32_le(self.latitude as i32);
        _tmp.put_i32_le(self.longitude as i32);
        _tmp.put_i16_le(self.roll as i16);
        _tmp.put_i16_le(self.pitch as i16);
        _tmp.put_u16_le(self.heading as u16);
        _tmp.put_i16_le(self.heading_sp as i16);
        _tmp.put_i16_le(self.altitude_amsl as i16);
        _tmp.put_i16_le(self.altitude_sp as i16);
        _tmp.put_u16_le(self.wp_distance as u16);
        _tmp.put_u8(self.base_mode as u8);
        _tmp.put_u8(self.landed_state as u8);
        _tmp.put_i8(self.throttle as i8);
        _tmp.put_u8(self.airspeed as u8);
        _tmp.put_u8(self.airspeed_sp as u8);
        _tmp.put_u8(self.groundspeed as u8);
        _tmp.put_i8(self.climb_rate as i8);
        _tmp.put_u8(self.gps_nsat as u8);
        _tmp.put_u8(self.gps_fix_type as u8);
        _tmp.put_u8(self.battery_remaining as u8);
        _tmp.put_i8(self.temperature as i8);
        _tmp.put_i8(self.temperature_air as i8);
        _tmp.put_u8(self.failsafe as u8);
        _tmp.put_u8(self.wp_num as u8);
        _tmp
    }
}
impl crate::proto::common::HighLatency2 {
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
            _struct.timestamp = buf.get_u32_le();
            _struct.latitude = buf.get_i32_le();
            _struct.longitude = buf.get_i32_le();
            _struct.custom_mode = buf.get_u16_le() as u32;
            _struct.altitude = buf.get_i16_le() as i32;
            _struct.target_altitude = buf.get_i16_le() as i32;
            _struct.target_distance = buf.get_u16_le() as u32;
            _struct.wp_num = buf.get_u16_le() as u32;
            let tmp = buf.get_u16_le();
            _struct.failure_flags =
                FromPrimitive::from_u16(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "HlFailureFlag".to_string(),
                    value: tmp as u32,
                })?;
            let tmp = buf.get_u8();
            _struct.r#type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavType".to_string(),
                value: tmp as u32,
            })?;
            let tmp = buf.get_u8();
            _struct.autopilot = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavAutopilot".to_string(),
                value: tmp as u32,
            })?;
            _struct.heading = buf.get_u8() as u32;
            _struct.target_heading = buf.get_u8() as u32;
            _struct.throttle = buf.get_u8() as u32;
            _struct.airspeed = buf.get_u8() as u32;
            _struct.airspeed_sp = buf.get_u8() as u32;
            _struct.groundspeed = buf.get_u8() as u32;
            _struct.windspeed = buf.get_u8() as u32;
            _struct.wind_heading = buf.get_u8() as u32;
            _struct.eph = buf.get_u8() as u32;
            _struct.epv = buf.get_u8() as u32;
            _struct.temperature_air = buf.get_i8() as i32;
            _struct.climb_rate = buf.get_i8() as i32;
            _struct.battery = buf.get_i8() as i32;
            _struct.custom0 = buf.get_i8() as i32;
            _struct.custom1 = buf.get_i8() as i32;
            _struct.custom2 = buf.get_i8() as i32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.timestamp as u32);
        _tmp.put_i32_le(self.latitude as i32);
        _tmp.put_i32_le(self.longitude as i32);
        _tmp.put_u16_le(self.custom_mode as u16);
        _tmp.put_i16_le(self.altitude as i16);
        _tmp.put_i16_le(self.target_altitude as i16);
        _tmp.put_u16_le(self.target_distance as u16);
        _tmp.put_u16_le(self.wp_num as u16);
        _tmp.put_u16_le(self.failure_flags as u16);
        _tmp.put_u8(self.r#type as u8);
        _tmp.put_u8(self.autopilot as u8);
        _tmp.put_u8(self.heading as u8);
        _tmp.put_u8(self.target_heading as u8);
        _tmp.put_u8(self.throttle as u8);
        _tmp.put_u8(self.airspeed as u8);
        _tmp.put_u8(self.airspeed_sp as u8);
        _tmp.put_u8(self.groundspeed as u8);
        _tmp.put_u8(self.windspeed as u8);
        _tmp.put_u8(self.wind_heading as u8);
        _tmp.put_u8(self.eph as u8);
        _tmp.put_u8(self.epv as u8);
        _tmp.put_i8(self.temperature_air as i8);
        _tmp.put_i8(self.climb_rate as i8);
        _tmp.put_i8(self.battery as i8);
        _tmp.put_i8(self.custom0 as i8);
        _tmp.put_i8(self.custom1 as i8);
        _tmp.put_i8(self.custom2 as i8);
        _tmp
    }
}
impl crate::proto::common::Vibration {
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
            _struct.time_usec = buf.get_u64_le();
            _struct.vibration_x = buf.get_f32_le();
            _struct.vibration_y = buf.get_f32_le();
            _struct.vibration_z = buf.get_f32_le();
            _struct.clipping_0 = buf.get_u32_le();
            _struct.clipping_1 = buf.get_u32_le();
            _struct.clipping_2 = buf.get_u32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        _tmp.put_f32_le(self.vibration_x as f32);
        _tmp.put_f32_le(self.vibration_y as f32);
        _tmp.put_f32_le(self.vibration_z as f32);
        _tmp.put_u32_le(self.clipping_0 as u32);
        _tmp.put_u32_le(self.clipping_1 as u32);
        _tmp.put_u32_le(self.clipping_2 as u32);
        _tmp
    }
}
impl crate::proto::common::HomePosition {
    pub const ENCODED_LEN: usize = 52usize;
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
            _struct.latitude = buf.get_i32_le();
            _struct.longitude = buf.get_i32_le();
            _struct.altitude = buf.get_i32_le();
            _struct.x = buf.get_f32_le();
            _struct.y = buf.get_f32_le();
            _struct.z = buf.get_f32_le();
            for _ in 0..4usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.q.push(val.into());
            }
            _struct.approach_x = buf.get_f32_le();
            _struct.approach_y = buf.get_f32_le();
            _struct.approach_z = buf.get_f32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i32_le(self.latitude as i32);
        _tmp.put_i32_le(self.longitude as i32);
        _tmp.put_i32_le(self.altitude as i32);
        _tmp.put_f32_le(self.x as f32);
        _tmp.put_f32_le(self.y as f32);
        _tmp.put_f32_le(self.z as f32);
        for val in &self.q {
            _tmp.put_f32_le(*val as f32);
        }
        _tmp.put_f32_le(self.approach_x as f32);
        _tmp.put_f32_le(self.approach_y as f32);
        _tmp.put_f32_le(self.approach_z as f32);
        _tmp
    }
}
impl crate::proto::common::SetHomePosition {
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
            _struct.latitude = buf.get_i32_le();
            _struct.longitude = buf.get_i32_le();
            _struct.altitude = buf.get_i32_le();
            _struct.x = buf.get_f32_le();
            _struct.y = buf.get_f32_le();
            _struct.z = buf.get_f32_le();
            for _ in 0..4usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.q.push(val.into());
            }
            _struct.approach_x = buf.get_f32_le();
            _struct.approach_y = buf.get_f32_le();
            _struct.approach_z = buf.get_f32_le();
            _struct.target_system = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i32_le(self.latitude as i32);
        _tmp.put_i32_le(self.longitude as i32);
        _tmp.put_i32_le(self.altitude as i32);
        _tmp.put_f32_le(self.x as f32);
        _tmp.put_f32_le(self.y as f32);
        _tmp.put_f32_le(self.z as f32);
        for val in &self.q {
            _tmp.put_f32_le(*val as f32);
        }
        _tmp.put_f32_le(self.approach_x as f32);
        _tmp.put_f32_le(self.approach_y as f32);
        _tmp.put_f32_le(self.approach_z as f32);
        _tmp.put_u8(self.target_system as u8);
        _tmp
    }
}
impl crate::proto::common::MessageInterval {
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
            _struct.interval_us = buf.get_i32_le();
            _struct.message_id = buf.get_u16_le() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i32_le(self.interval_us as i32);
        _tmp.put_u16_le(self.message_id as u16);
        _tmp
    }
}
impl crate::proto::common::ExtendedSysState {
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
            _struct.vtol_state = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavVtolState".to_string(),
                value: tmp as u32,
            })?;
            let tmp = buf.get_u8();
            _struct.landed_state = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavLandedState".to_string(),
                value: tmp as u32,
            })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.vtol_state as u8);
        _tmp.put_u8(self.landed_state as u8);
        _tmp
    }
}
impl crate::proto::common::AdsbVehicle {
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
            _struct.icao_address = buf.get_u32_le();
            _struct.lat = buf.get_i32_le();
            _struct.lon = buf.get_i32_le();
            _struct.altitude = buf.get_i32_le();
            _struct.heading = buf.get_u16_le() as u32;
            _struct.hor_velocity = buf.get_u16_le() as u32;
            _struct.ver_velocity = buf.get_i16_le() as i32;
            let tmp = buf.get_u16_le();
            _struct.flags = FromPrimitive::from_u16(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "AdsbFlags".to_string(),
                value: tmp as u32,
            })?;
            _struct.squawk = buf.get_u16_le() as u32;
            let tmp = buf.get_u8();
            _struct.altitude_type =
                FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "AdsbAltitudeType".to_string(),
                    value: tmp as u32,
                })?;
            let mut s = Vec::with_capacity(9usize);
            for _ in 0..9usize {
                s.push(buf.get_u8());
            }
            _struct.callsign = String::from_utf8_lossy(&s).into();
            let tmp = buf.get_u8();
            _struct.emitter_type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "AdsbEmitterType".to_string(),
                value: tmp as u32,
            })?;
            _struct.tslc = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.icao_address as u32);
        _tmp.put_i32_le(self.lat as i32);
        _tmp.put_i32_le(self.lon as i32);
        _tmp.put_i32_le(self.altitude as i32);
        _tmp.put_u16_le(self.heading as u16);
        _tmp.put_u16_le(self.hor_velocity as u16);
        _tmp.put_i16_le(self.ver_velocity as i16);
        _tmp.put_u16_le(self.flags as u16);
        _tmp.put_u16_le(self.squawk as u16);
        _tmp.put_u8(self.altitude_type as u8);
        for val in self.callsign.bytes() {
            _tmp.put_u8(val);
        }
        _tmp.put_u8(self.emitter_type as u8);
        _tmp.put_u8(self.tslc as u8);
        _tmp
    }
}
impl crate::proto::common::Collision {
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
            _struct.id = buf.get_u32_le();
            _struct.time_to_minimum_delta = buf.get_f32_le();
            _struct.altitude_minimum_delta = buf.get_f32_le();
            _struct.horizontal_minimum_delta = buf.get_f32_le();
            let tmp = buf.get_u8();
            _struct.src = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavCollisionSrc".to_string(),
                value: tmp as u32,
            })?;
            let tmp = buf.get_u8();
            _struct.action = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavCollisionAction".to_string(),
                value: tmp as u32,
            })?;
            let tmp = buf.get_u8();
            _struct.threat_level = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavCollisionThreatLevel".to_string(),
                value: tmp as u32,
            })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.id as u32);
        _tmp.put_f32_le(self.time_to_minimum_delta as f32);
        _tmp.put_f32_le(self.altitude_minimum_delta as f32);
        _tmp.put_f32_le(self.horizontal_minimum_delta as f32);
        _tmp.put_u8(self.src as u8);
        _tmp.put_u8(self.action as u8);
        _tmp.put_u8(self.threat_level as u8);
        _tmp
    }
}
impl crate::proto::common::V2Extension {
    pub const ENCODED_LEN: usize = 254usize;
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
            _struct.message_type = buf.get_u16_le() as u32;
            _struct.target_network = buf.get_u8() as u32;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            for _ in 0..249usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.payload.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.message_type as u16);
        _tmp.put_u8(self.target_network as u8);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        for val in &self.payload {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
impl crate::proto::common::MemoryVect {
    pub const ENCODED_LEN: usize = 36usize;
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
            _struct.address = buf.get_u16_le() as u32;
            _struct.ver = buf.get_u8() as u32;
            _struct.r#type = buf.get_u8() as u32;
            for _ in 0..32usize {
                let val = buf.get_i8() as i32;
                #[allow(clippy::useless_conversion)]
                _struct.value.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.address as u16);
        _tmp.put_u8(self.ver as u8);
        _tmp.put_u8(self.r#type as u8);
        for val in &self.value {
            _tmp.put_i8(*val as i8);
        }
        _tmp
    }
}
impl crate::proto::common::DebugVect {
    pub const ENCODED_LEN: usize = 30usize;
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
            _struct.x = buf.get_f32_le();
            _struct.y = buf.get_f32_le();
            _struct.z = buf.get_f32_le();
            let mut s = Vec::with_capacity(10usize);
            for _ in 0..10usize {
                s.push(buf.get_u8());
            }
            _struct.name = String::from_utf8_lossy(&s).into();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        _tmp.put_f32_le(self.x as f32);
        _tmp.put_f32_le(self.y as f32);
        _tmp.put_f32_le(self.z as f32);
        for val in self.name.bytes() {
            _tmp.put_u8(val);
        }
        _tmp
    }
}
impl crate::proto::common::NamedValueFloat {
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
            _struct.time_boot_ms = buf.get_u32_le();
            _struct.value = buf.get_f32_le();
            let mut s = Vec::with_capacity(10usize);
            for _ in 0..10usize {
                s.push(buf.get_u8());
            }
            _struct.name = String::from_utf8_lossy(&s).into();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms as u32);
        _tmp.put_f32_le(self.value as f32);
        for val in self.name.bytes() {
            _tmp.put_u8(val);
        }
        _tmp
    }
}
impl crate::proto::common::NamedValueInt {
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
            _struct.time_boot_ms = buf.get_u32_le();
            _struct.value = buf.get_i32_le();
            let mut s = Vec::with_capacity(10usize);
            for _ in 0..10usize {
                s.push(buf.get_u8());
            }
            _struct.name = String::from_utf8_lossy(&s).into();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms as u32);
        _tmp.put_i32_le(self.value as i32);
        for val in self.name.bytes() {
            _tmp.put_u8(val);
        }
        _tmp
    }
}
impl crate::proto::common::Statustext {
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
            let tmp = buf.get_u8();
            _struct.severity = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavSeverity".to_string(),
                value: tmp as u32,
            })?;
            let mut s = Vec::with_capacity(50usize);
            for _ in 0..50usize {
                s.push(buf.get_u8());
            }
            _struct.text = String::from_utf8_lossy(&s).into();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.severity as u8);
        for val in self.text.bytes() {
            _tmp.put_u8(val);
        }
        _tmp
    }
}
impl crate::proto::common::Debug {
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
            _struct.time_boot_ms = buf.get_u32_le();
            _struct.value = buf.get_f32_le();
            _struct.ind = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms as u32);
        _tmp.put_f32_le(self.value as f32);
        _tmp.put_u8(self.ind as u8);
        _tmp
    }
}
impl crate::proto::common::SetupSigning {
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
            _struct.initial_timestamp = buf.get_u64_le();
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            for _ in 0..32usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.secret_key.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.initial_timestamp as u64);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        for val in &self.secret_key {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
impl crate::proto::common::ButtonChange {
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
            _struct.time_boot_ms = buf.get_u32_le();
            _struct.last_change_ms = buf.get_u32_le();
            _struct.state = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms as u32);
        _tmp.put_u32_le(self.last_change_ms as u32);
        _tmp.put_u8(self.state as u8);
        _tmp
    }
}
impl crate::proto::common::PlayTune {
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
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            let mut s = Vec::with_capacity(30usize);
            for _ in 0..30usize {
                s.push(buf.get_u8());
            }
            _struct.tune = String::from_utf8_lossy(&s).into();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        for val in self.tune.bytes() {
            _tmp.put_u8(val);
        }
        _tmp
    }
}
impl crate::proto::common::CameraInformation {
    pub const ENCODED_LEN: usize = 235usize;
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
            _struct.firmware_version = buf.get_u32_le();
            _struct.focal_length = buf.get_f32_le();
            _struct.sensor_size_h = buf.get_f32_le();
            _struct.sensor_size_v = buf.get_f32_le();
            let tmp = buf.get_u32_le();
            _struct.flags = FromPrimitive::from_u32(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "CameraCapFlags".to_string(),
                value: tmp as u32,
            })?;
            _struct.resolution_h = buf.get_u16_le() as u32;
            _struct.resolution_v = buf.get_u16_le() as u32;
            _struct.cam_definition_version = buf.get_u16_le() as u32;
            for _ in 0..32usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.vendor_name.push(val.into());
            }
            for _ in 0..32usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.model_name.push(val.into());
            }
            _struct.lens_id = buf.get_u8() as u32;
            let mut s = Vec::with_capacity(140usize);
            for _ in 0..140usize {
                s.push(buf.get_u8());
            }
            _struct.cam_definition_uri = String::from_utf8_lossy(&s).into();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms as u32);
        _tmp.put_u32_le(self.firmware_version as u32);
        _tmp.put_f32_le(self.focal_length as f32);
        _tmp.put_f32_le(self.sensor_size_h as f32);
        _tmp.put_f32_le(self.sensor_size_v as f32);
        _tmp.put_u32_le(self.flags as u32);
        _tmp.put_u16_le(self.resolution_h as u16);
        _tmp.put_u16_le(self.resolution_v as u16);
        _tmp.put_u16_le(self.cam_definition_version as u16);
        for val in &self.vendor_name {
            _tmp.put_u8(*val as u8);
        }
        for val in &self.model_name {
            _tmp.put_u8(*val as u8);
        }
        _tmp.put_u8(self.lens_id as u8);
        for val in self.cam_definition_uri.bytes() {
            _tmp.put_u8(val);
        }
        _tmp
    }
}
impl crate::proto::common::CameraSettings {
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
            _struct.time_boot_ms = buf.get_u32_le();
            let tmp = buf.get_u8();
            _struct.mode_id = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "CameraMode".to_string(),
                value: tmp as u32,
            })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms as u32);
        _tmp.put_u8(self.mode_id as u8);
        _tmp
    }
}
impl crate::proto::common::StorageInformation {
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
            _struct.time_boot_ms = buf.get_u32_le();
            _struct.total_capacity = buf.get_f32_le();
            _struct.used_capacity = buf.get_f32_le();
            _struct.available_capacity = buf.get_f32_le();
            _struct.read_speed = buf.get_f32_le();
            _struct.write_speed = buf.get_f32_le();
            _struct.storage_id = buf.get_u8() as u32;
            _struct.storage_count = buf.get_u8() as u32;
            let tmp = buf.get_u8();
            _struct.status = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "StorageStatus".to_string(),
                value: tmp as u32,
            })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms as u32);
        _tmp.put_f32_le(self.total_capacity as f32);
        _tmp.put_f32_le(self.used_capacity as f32);
        _tmp.put_f32_le(self.available_capacity as f32);
        _tmp.put_f32_le(self.read_speed as f32);
        _tmp.put_f32_le(self.write_speed as f32);
        _tmp.put_u8(self.storage_id as u8);
        _tmp.put_u8(self.storage_count as u8);
        _tmp.put_u8(self.status as u8);
        _tmp
    }
}
impl crate::proto::common::CameraCaptureStatus {
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
            _struct.time_boot_ms = buf.get_u32_le();
            _struct.image_interval = buf.get_f32_le();
            _struct.recording_time_ms = buf.get_u32_le();
            _struct.available_capacity = buf.get_f32_le();
            _struct.image_status = buf.get_u8() as u32;
            _struct.video_status = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms as u32);
        _tmp.put_f32_le(self.image_interval as f32);
        _tmp.put_u32_le(self.recording_time_ms as u32);
        _tmp.put_f32_le(self.available_capacity as f32);
        _tmp.put_u8(self.image_status as u8);
        _tmp.put_u8(self.video_status as u8);
        _tmp
    }
}
impl crate::proto::common::CameraImageCaptured {
    pub const ENCODED_LEN: usize = 255usize;
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
            _struct.time_utc = buf.get_u64_le();
            _struct.time_boot_ms = buf.get_u32_le();
            _struct.lat = buf.get_i32_le();
            _struct.lon = buf.get_i32_le();
            _struct.alt = buf.get_i32_le();
            _struct.relative_alt = buf.get_i32_le();
            for _ in 0..4usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.q.push(val.into());
            }
            _struct.image_index = buf.get_i32_le();
            _struct.camera_id = buf.get_u8() as u32;
            _struct.capture_result = buf.get_i8() as i32;
            let mut s = Vec::with_capacity(205usize);
            for _ in 0..205usize {
                s.push(buf.get_u8());
            }
            _struct.file_url = String::from_utf8_lossy(&s).into();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_utc as u64);
        _tmp.put_u32_le(self.time_boot_ms as u32);
        _tmp.put_i32_le(self.lat as i32);
        _tmp.put_i32_le(self.lon as i32);
        _tmp.put_i32_le(self.alt as i32);
        _tmp.put_i32_le(self.relative_alt as i32);
        for val in &self.q {
            _tmp.put_f32_le(*val as f32);
        }
        _tmp.put_i32_le(self.image_index as i32);
        _tmp.put_u8(self.camera_id as u8);
        _tmp.put_i8(self.capture_result as i8);
        for val in self.file_url.bytes() {
            _tmp.put_u8(val);
        }
        _tmp
    }
}
impl crate::proto::common::FlightInformation {
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
            _struct.arming_time_utc = buf.get_u64_le();
            _struct.takeoff_time_utc = buf.get_u64_le();
            _struct.flight_uuid = buf.get_u64_le();
            _struct.time_boot_ms = buf.get_u32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.arming_time_utc as u64);
        _tmp.put_u64_le(self.takeoff_time_utc as u64);
        _tmp.put_u64_le(self.flight_uuid as u64);
        _tmp.put_u32_le(self.time_boot_ms as u32);
        _tmp
    }
}
impl crate::proto::common::MountOrientation {
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
            _struct.roll = buf.get_f32_le();
            _struct.pitch = buf.get_f32_le();
            _struct.yaw = buf.get_f32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms as u32);
        _tmp.put_f32_le(self.roll as f32);
        _tmp.put_f32_le(self.pitch as f32);
        _tmp.put_f32_le(self.yaw as f32);
        _tmp
    }
}
impl crate::proto::common::LoggingData {
    pub const ENCODED_LEN: usize = 255usize;
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
            _struct.sequence = buf.get_u16_le() as u32;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            _struct.length = buf.get_u8() as u32;
            _struct.first_message_offset = buf.get_u8() as u32;
            for _ in 0..249usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.data.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.sequence as u16);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp.put_u8(self.length as u8);
        _tmp.put_u8(self.first_message_offset as u8);
        for val in &self.data {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
impl crate::proto::common::LoggingDataAcked {
    pub const ENCODED_LEN: usize = 255usize;
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
            _struct.sequence = buf.get_u16_le() as u32;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            _struct.length = buf.get_u8() as u32;
            _struct.first_message_offset = buf.get_u8() as u32;
            for _ in 0..249usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.data.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.sequence as u16);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp.put_u8(self.length as u8);
        _tmp.put_u8(self.first_message_offset as u8);
        for val in &self.data {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
impl crate::proto::common::LoggingAck {
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
            _struct.sequence = buf.get_u16_le() as u32;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.sequence as u16);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp
    }
}
impl crate::proto::common::VideoStreamInformation {
    pub const ENCODED_LEN: usize = 213usize;
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
            _struct.framerate = buf.get_f32_le();
            _struct.bitrate = buf.get_u32_le();
            let tmp = buf.get_u16_le();
            _struct.flags = FromPrimitive::from_u16(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "VideoStreamStatusFlags".to_string(),
                value: tmp as u32,
            })?;
            _struct.resolution_h = buf.get_u16_le() as u32;
            _struct.resolution_v = buf.get_u16_le() as u32;
            _struct.rotation = buf.get_u16_le() as u32;
            _struct.hfov = buf.get_u16_le() as u32;
            _struct.stream_id = buf.get_u8() as u32;
            _struct.count = buf.get_u8() as u32;
            let tmp = buf.get_u8();
            _struct.r#type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "VideoStreamType".to_string(),
                value: tmp as u32,
            })?;
            let mut s = Vec::with_capacity(32usize);
            for _ in 0..32usize {
                s.push(buf.get_u8());
            }
            _struct.name = String::from_utf8_lossy(&s).into();
            let mut s = Vec::with_capacity(160usize);
            for _ in 0..160usize {
                s.push(buf.get_u8());
            }
            _struct.uri = String::from_utf8_lossy(&s).into();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.framerate as f32);
        _tmp.put_u32_le(self.bitrate as u32);
        _tmp.put_u16_le(self.flags as u16);
        _tmp.put_u16_le(self.resolution_h as u16);
        _tmp.put_u16_le(self.resolution_v as u16);
        _tmp.put_u16_le(self.rotation as u16);
        _tmp.put_u16_le(self.hfov as u16);
        _tmp.put_u8(self.stream_id as u8);
        _tmp.put_u8(self.count as u8);
        _tmp.put_u8(self.r#type as u8);
        for val in self.name.bytes() {
            _tmp.put_u8(val);
        }
        for val in self.uri.bytes() {
            _tmp.put_u8(val);
        }
        _tmp
    }
}
impl crate::proto::common::VideoStreamStatus {
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
            _struct.framerate = buf.get_f32_le();
            _struct.bitrate = buf.get_u32_le();
            let tmp = buf.get_u16_le();
            _struct.flags = FromPrimitive::from_u16(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "VideoStreamStatusFlags".to_string(),
                value: tmp as u32,
            })?;
            _struct.resolution_h = buf.get_u16_le() as u32;
            _struct.resolution_v = buf.get_u16_le() as u32;
            _struct.rotation = buf.get_u16_le() as u32;
            _struct.hfov = buf.get_u16_le() as u32;
            _struct.stream_id = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.framerate as f32);
        _tmp.put_u32_le(self.bitrate as u32);
        _tmp.put_u16_le(self.flags as u16);
        _tmp.put_u16_le(self.resolution_h as u16);
        _tmp.put_u16_le(self.resolution_v as u16);
        _tmp.put_u16_le(self.rotation as u16);
        _tmp.put_u16_le(self.hfov as u16);
        _tmp.put_u8(self.stream_id as u8);
        _tmp
    }
}
impl crate::proto::common::GimbalManagerInformation {
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
            _struct.time_boot_ms = buf.get_u32_le();
            let tmp = buf.get_u32_le();
            _struct.cap_flags = FromPrimitive::from_u32(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "GimbalManagerCapFlags".to_string(),
                value: tmp as u32,
            })?;
            _struct.tilt_max = buf.get_f32_le();
            _struct.tilt_min = buf.get_f32_le();
            _struct.tilt_rate_max = buf.get_f32_le();
            _struct.pan_max = buf.get_f32_le();
            _struct.pan_min = buf.get_f32_le();
            _struct.pan_rate_max = buf.get_f32_le();
            _struct.gimbal_device_id = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms as u32);
        _tmp.put_u32_le(self.cap_flags as u32);
        _tmp.put_f32_le(self.tilt_max as f32);
        _tmp.put_f32_le(self.tilt_min as f32);
        _tmp.put_f32_le(self.tilt_rate_max as f32);
        _tmp.put_f32_le(self.pan_max as f32);
        _tmp.put_f32_le(self.pan_min as f32);
        _tmp.put_f32_le(self.pan_rate_max as f32);
        _tmp.put_u8(self.gimbal_device_id as u8);
        _tmp
    }
}
impl crate::proto::common::GimbalManagerStatus {
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
            _struct.time_boot_ms = buf.get_u32_le();
            let tmp = buf.get_u32_le();
            _struct.flags = FromPrimitive::from_u32(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "GimbalManagerFlags".to_string(),
                value: tmp as u32,
            })?;
            _struct.gimbal_device_id = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms as u32);
        _tmp.put_u32_le(self.flags as u32);
        _tmp.put_u8(self.gimbal_device_id as u8);
        _tmp
    }
}
impl crate::proto::common::GimbalManagerSetAttitude {
    pub const ENCODED_LEN: usize = 35usize;
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
            _struct.flags = FromPrimitive::from_u32(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "GimbalManagerFlags".to_string(),
                value: tmp as u32,
            })?;
            for _ in 0..4usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.q.push(val.into());
            }
            _struct.angular_velocity_x = buf.get_f32_le();
            _struct.angular_velocity_y = buf.get_f32_le();
            _struct.angular_velocity_z = buf.get_f32_le();
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            _struct.gimbal_device_id = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.flags as u32);
        for val in &self.q {
            _tmp.put_f32_le(*val as f32);
        }
        _tmp.put_f32_le(self.angular_velocity_x as f32);
        _tmp.put_f32_le(self.angular_velocity_y as f32);
        _tmp.put_f32_le(self.angular_velocity_z as f32);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp.put_u8(self.gimbal_device_id as u8);
        _tmp
    }
}
impl crate::proto::common::GimbalDeviceInformation {
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
            _struct.time_boot_ms = buf.get_u32_le();
            _struct.firmware_version = buf.get_u32_le();
            _struct.tilt_max = buf.get_f32_le();
            _struct.tilt_min = buf.get_f32_le();
            _struct.tilt_rate_max = buf.get_f32_le();
            _struct.pan_max = buf.get_f32_le();
            _struct.pan_min = buf.get_f32_le();
            _struct.pan_rate_max = buf.get_f32_le();
            let tmp = buf.get_u16_le();
            _struct.cap_flags = FromPrimitive::from_u16(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "GimbalDeviceCapFlags".to_string(),
                value: tmp as u32,
            })?;
            for _ in 0..32usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.vendor_name.push(val.into());
            }
            for _ in 0..32usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.model_name.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms as u32);
        _tmp.put_u32_le(self.firmware_version as u32);
        _tmp.put_f32_le(self.tilt_max as f32);
        _tmp.put_f32_le(self.tilt_min as f32);
        _tmp.put_f32_le(self.tilt_rate_max as f32);
        _tmp.put_f32_le(self.pan_max as f32);
        _tmp.put_f32_le(self.pan_min as f32);
        _tmp.put_f32_le(self.pan_rate_max as f32);
        _tmp.put_u16_le(self.cap_flags as u16);
        for val in &self.vendor_name {
            _tmp.put_u8(*val as u8);
        }
        for val in &self.model_name {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
impl crate::proto::common::GimbalDeviceSetAttitude {
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
            for _ in 0..4usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.q.push(val.into());
            }
            _struct.angular_velocity_x = buf.get_f32_le();
            _struct.angular_velocity_y = buf.get_f32_le();
            _struct.angular_velocity_z = buf.get_f32_le();
            let tmp = buf.get_u16_le();
            _struct.flags = FromPrimitive::from_u16(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "GimbalDeviceFlags".to_string(),
                value: tmp as u32,
            })?;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        for val in &self.q {
            _tmp.put_f32_le(*val as f32);
        }
        _tmp.put_f32_le(self.angular_velocity_x as f32);
        _tmp.put_f32_le(self.angular_velocity_y as f32);
        _tmp.put_f32_le(self.angular_velocity_z as f32);
        _tmp.put_u16_le(self.flags as u16);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp
    }
}
impl crate::proto::common::GimbalDeviceAttitudeStatus {
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
            _struct.time_boot_ms = buf.get_u32_le();
            for _ in 0..4usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.q.push(val.into());
            }
            _struct.angular_velocity_x = buf.get_f32_le();
            _struct.angular_velocity_y = buf.get_f32_le();
            _struct.angular_velocity_z = buf.get_f32_le();
            let tmp = buf.get_u32_le();
            _struct.failure_flags =
                FromPrimitive::from_u32(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "GimbalDeviceErrorFlags".to_string(),
                    value: tmp as u32,
                })?;
            let tmp = buf.get_u16_le();
            _struct.flags = FromPrimitive::from_u16(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "GimbalDeviceFlags".to_string(),
                value: tmp as u32,
            })?;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms as u32);
        for val in &self.q {
            _tmp.put_f32_le(*val as f32);
        }
        _tmp.put_f32_le(self.angular_velocity_x as f32);
        _tmp.put_f32_le(self.angular_velocity_y as f32);
        _tmp.put_f32_le(self.angular_velocity_z as f32);
        _tmp.put_u32_le(self.failure_flags as u32);
        _tmp.put_u16_le(self.flags as u16);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp
    }
}
impl crate::proto::common::AutopilotStateForGimbalDevice {
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
            _struct.time_boot_us = buf.get_u64_le();
            for _ in 0..4usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.q.push(val.into());
            }
            _struct.q_estimated_delay_us = buf.get_u32_le();
            _struct.vx = buf.get_f32_le();
            _struct.vy = buf.get_f32_le();
            _struct.vz = buf.get_f32_le();
            _struct.v_estimated_delay_us = buf.get_u32_le();
            _struct.feed_forward_angular_velocity_z = buf.get_f32_le();
            let tmp = buf.get_u16_le();
            _struct.estimator_status =
                FromPrimitive::from_u16(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "EstimatorStatusFlags".to_string(),
                    value: tmp as u32,
                })?;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            let tmp = buf.get_u8();
            _struct.landed_state = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavLandedState".to_string(),
                value: tmp as u32,
            })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_boot_us as u64);
        for val in &self.q {
            _tmp.put_f32_le(*val as f32);
        }
        _tmp.put_u32_le(self.q_estimated_delay_us as u32);
        _tmp.put_f32_le(self.vx as f32);
        _tmp.put_f32_le(self.vy as f32);
        _tmp.put_f32_le(self.vz as f32);
        _tmp.put_u32_le(self.v_estimated_delay_us as u32);
        _tmp.put_f32_le(self.feed_forward_angular_velocity_z as f32);
        _tmp.put_u16_le(self.estimator_status as u16);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp.put_u8(self.landed_state as u8);
        _tmp
    }
}
impl crate::proto::common::GimbalManagerSetTiltpan {
    pub const ENCODED_LEN: usize = 23usize;
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
            _struct.flags = FromPrimitive::from_u32(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "GimbalManagerFlags".to_string(),
                value: tmp as u32,
            })?;
            _struct.tilt = buf.get_f32_le();
            _struct.pan = buf.get_f32_le();
            _struct.tilt_rate = buf.get_f32_le();
            _struct.pan_rate = buf.get_f32_le();
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            _struct.gimbal_device_id = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.flags as u32);
        _tmp.put_f32_le(self.tilt as f32);
        _tmp.put_f32_le(self.pan as f32);
        _tmp.put_f32_le(self.tilt_rate as f32);
        _tmp.put_f32_le(self.pan_rate as f32);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp.put_u8(self.gimbal_device_id as u8);
        _tmp
    }
}
impl crate::proto::common::WifiConfigAp {
    pub const ENCODED_LEN: usize = 96usize;
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
            let mut s = Vec::with_capacity(32usize);
            for _ in 0..32usize {
                s.push(buf.get_u8());
            }
            _struct.ssid = String::from_utf8_lossy(&s).into();
            let mut s = Vec::with_capacity(64usize);
            for _ in 0..64usize {
                s.push(buf.get_u8());
            }
            _struct.password = String::from_utf8_lossy(&s).into();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        for val in self.ssid.bytes() {
            _tmp.put_u8(val);
        }
        for val in self.password.bytes() {
            _tmp.put_u8(val);
        }
        _tmp
    }
}
impl crate::proto::common::ProtocolVersion {
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
            _struct.version = buf.get_u16_le() as u32;
            _struct.min_version = buf.get_u16_le() as u32;
            _struct.max_version = buf.get_u16_le() as u32;
            for _ in 0..8usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.spec_version_hash.push(val.into());
            }
            for _ in 0..8usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.library_version_hash.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.version as u16);
        _tmp.put_u16_le(self.min_version as u16);
        _tmp.put_u16_le(self.max_version as u16);
        for val in &self.spec_version_hash {
            _tmp.put_u8(*val as u8);
        }
        for val in &self.library_version_hash {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
impl crate::proto::common::AisVessel {
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
            _struct.mmsi = buf.get_u32_le();
            _struct.lat = buf.get_i32_le();
            _struct.lon = buf.get_i32_le();
            _struct.cog = buf.get_u16_le() as u32;
            _struct.heading = buf.get_u16_le() as u32;
            _struct.velocity = buf.get_u16_le() as u32;
            _struct.dimension_bow = buf.get_u16_le() as u32;
            _struct.dimension_stern = buf.get_u16_le() as u32;
            _struct.tslc = buf.get_u16_le() as u32;
            let tmp = buf.get_u16_le();
            _struct.flags = FromPrimitive::from_u16(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "AisFlags".to_string(),
                value: tmp as u32,
            })?;
            _struct.turn_rate = buf.get_i8() as i32;
            let tmp = buf.get_u8();
            _struct.navigational_status =
                FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "AisNavStatus".to_string(),
                    value: tmp as u32,
                })?;
            let tmp = buf.get_u8();
            _struct.r#type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "AisType".to_string(),
                value: tmp as u32,
            })?;
            _struct.dimension_port = buf.get_u8() as u32;
            _struct.dimension_starboard = buf.get_u8() as u32;
            let mut s = Vec::with_capacity(7usize);
            for _ in 0..7usize {
                s.push(buf.get_u8());
            }
            _struct.callsign = String::from_utf8_lossy(&s).into();
            let mut s = Vec::with_capacity(20usize);
            for _ in 0..20usize {
                s.push(buf.get_u8());
            }
            _struct.name = String::from_utf8_lossy(&s).into();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.mmsi as u32);
        _tmp.put_i32_le(self.lat as i32);
        _tmp.put_i32_le(self.lon as i32);
        _tmp.put_u16_le(self.cog as u16);
        _tmp.put_u16_le(self.heading as u16);
        _tmp.put_u16_le(self.velocity as u16);
        _tmp.put_u16_le(self.dimension_bow as u16);
        _tmp.put_u16_le(self.dimension_stern as u16);
        _tmp.put_u16_le(self.tslc as u16);
        _tmp.put_u16_le(self.flags as u16);
        _tmp.put_i8(self.turn_rate as i8);
        _tmp.put_u8(self.navigational_status as u8);
        _tmp.put_u8(self.r#type as u8);
        _tmp.put_u8(self.dimension_port as u8);
        _tmp.put_u8(self.dimension_starboard as u8);
        for val in self.callsign.bytes() {
            _tmp.put_u8(val);
        }
        for val in self.name.bytes() {
            _tmp.put_u8(val);
        }
        _tmp
    }
}
impl crate::proto::common::UavcanNodeStatus {
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
            _struct.time_usec = buf.get_u64_le();
            _struct.uptime_sec = buf.get_u32_le();
            _struct.vendor_specific_status_code = buf.get_u16_le() as u32;
            let tmp = buf.get_u8();
            _struct.health = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "UavcanNodeHealth".to_string(),
                value: tmp as u32,
            })?;
            let tmp = buf.get_u8();
            _struct.mode = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "UavcanNodeMode".to_string(),
                value: tmp as u32,
            })?;
            _struct.sub_mode = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        _tmp.put_u32_le(self.uptime_sec as u32);
        _tmp.put_u16_le(self.vendor_specific_status_code as u16);
        _tmp.put_u8(self.health as u8);
        _tmp.put_u8(self.mode as u8);
        _tmp.put_u8(self.sub_mode as u8);
        _tmp
    }
}
impl crate::proto::common::UavcanNodeInfo {
    pub const ENCODED_LEN: usize = 116usize;
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
            _struct.uptime_sec = buf.get_u32_le();
            _struct.sw_vcs_commit = buf.get_u32_le();
            let mut s = Vec::with_capacity(80usize);
            for _ in 0..80usize {
                s.push(buf.get_u8());
            }
            _struct.name = String::from_utf8_lossy(&s).into();
            _struct.hw_version_major = buf.get_u8() as u32;
            _struct.hw_version_minor = buf.get_u8() as u32;
            for _ in 0..16usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.hw_unique_id.push(val.into());
            }
            _struct.sw_version_major = buf.get_u8() as u32;
            _struct.sw_version_minor = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        _tmp.put_u32_le(self.uptime_sec as u32);
        _tmp.put_u32_le(self.sw_vcs_commit as u32);
        for val in self.name.bytes() {
            _tmp.put_u8(val);
        }
        _tmp.put_u8(self.hw_version_major as u8);
        _tmp.put_u8(self.hw_version_minor as u8);
        for val in &self.hw_unique_id {
            _tmp.put_u8(*val as u8);
        }
        _tmp.put_u8(self.sw_version_major as u8);
        _tmp.put_u8(self.sw_version_minor as u8);
        _tmp
    }
}
impl crate::proto::common::ParamExtRequestRead {
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
            _struct.param_index = buf.get_i16_le() as i32;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            let mut s = Vec::with_capacity(16usize);
            for _ in 0..16usize {
                s.push(buf.get_u8());
            }
            _struct.param_id = String::from_utf8_lossy(&s).into();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i16_le(self.param_index as i16);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        for val in self.param_id.bytes() {
            _tmp.put_u8(val);
        }
        _tmp
    }
}
impl crate::proto::common::ParamExtRequestList {
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
impl crate::proto::common::ParamExtValue {
    pub const ENCODED_LEN: usize = 149usize;
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
            _struct.param_count = buf.get_u16_le() as u32;
            _struct.param_index = buf.get_u16_le() as u32;
            let mut s = Vec::with_capacity(16usize);
            for _ in 0..16usize {
                s.push(buf.get_u8());
            }
            _struct.param_id = String::from_utf8_lossy(&s).into();
            let mut s = Vec::with_capacity(128usize);
            for _ in 0..128usize {
                s.push(buf.get_u8());
            }
            _struct.param_value = String::from_utf8_lossy(&s).into();
            let tmp = buf.get_u8();
            _struct.param_type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavParamExtType".to_string(),
                value: tmp as u32,
            })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.param_count as u16);
        _tmp.put_u16_le(self.param_index as u16);
        for val in self.param_id.bytes() {
            _tmp.put_u8(val);
        }
        for val in self.param_value.bytes() {
            _tmp.put_u8(val);
        }
        _tmp.put_u8(self.param_type as u8);
        _tmp
    }
}
impl crate::proto::common::ParamExtSet {
    pub const ENCODED_LEN: usize = 147usize;
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
            let mut s = Vec::with_capacity(16usize);
            for _ in 0..16usize {
                s.push(buf.get_u8());
            }
            _struct.param_id = String::from_utf8_lossy(&s).into();
            let mut s = Vec::with_capacity(128usize);
            for _ in 0..128usize {
                s.push(buf.get_u8());
            }
            _struct.param_value = String::from_utf8_lossy(&s).into();
            let tmp = buf.get_u8();
            _struct.param_type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavParamExtType".to_string(),
                value: tmp as u32,
            })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        for val in self.param_id.bytes() {
            _tmp.put_u8(val);
        }
        for val in self.param_value.bytes() {
            _tmp.put_u8(val);
        }
        _tmp.put_u8(self.param_type as u8);
        _tmp
    }
}
impl crate::proto::common::ParamExtAck {
    pub const ENCODED_LEN: usize = 146usize;
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
            let mut s = Vec::with_capacity(16usize);
            for _ in 0..16usize {
                s.push(buf.get_u8());
            }
            _struct.param_id = String::from_utf8_lossy(&s).into();
            let mut s = Vec::with_capacity(128usize);
            for _ in 0..128usize {
                s.push(buf.get_u8());
            }
            _struct.param_value = String::from_utf8_lossy(&s).into();
            let tmp = buf.get_u8();
            _struct.param_type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavParamExtType".to_string(),
                value: tmp as u32,
            })?;
            let tmp = buf.get_u8();
            _struct.param_result = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "ParamAck".to_string(),
                value: tmp as u32,
            })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        for val in self.param_id.bytes() {
            _tmp.put_u8(val);
        }
        for val in self.param_value.bytes() {
            _tmp.put_u8(val);
        }
        _tmp.put_u8(self.param_type as u8);
        _tmp.put_u8(self.param_result as u8);
        _tmp
    }
}
impl crate::proto::common::ObstacleDistance {
    pub const ENCODED_LEN: usize = 158usize;
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
            for _ in 0..72usize {
                let val = buf.get_u16_le() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.distances.push(val.into());
            }
            _struct.min_distance = buf.get_u16_le() as u32;
            _struct.max_distance = buf.get_u16_le() as u32;
            let tmp = buf.get_u8();
            _struct.sensor_type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavDistanceSensor".to_string(),
                value: tmp as u32,
            })?;
            _struct.increment = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        for val in &self.distances {
            _tmp.put_u16_le(*val as u16);
        }
        _tmp.put_u16_le(self.min_distance as u16);
        _tmp.put_u16_le(self.max_distance as u16);
        _tmp.put_u8(self.sensor_type as u8);
        _tmp.put_u8(self.increment as u8);
        _tmp
    }
}
impl crate::proto::common::Odometry {
    pub const ENCODED_LEN: usize = 230usize;
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
            _struct.x = buf.get_f32_le();
            _struct.y = buf.get_f32_le();
            _struct.z = buf.get_f32_le();
            for _ in 0..4usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.q.push(val.into());
            }
            _struct.vx = buf.get_f32_le();
            _struct.vy = buf.get_f32_le();
            _struct.vz = buf.get_f32_le();
            _struct.rollspeed = buf.get_f32_le();
            _struct.pitchspeed = buf.get_f32_le();
            _struct.yawspeed = buf.get_f32_le();
            for _ in 0..21usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.pose_covariance.push(val.into());
            }
            for _ in 0..21usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.velocity_covariance.push(val.into());
            }
            let tmp = buf.get_u8();
            _struct.frame_id = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavFrame".to_string(),
                value: tmp as u32,
            })?;
            let tmp = buf.get_u8();
            _struct.child_frame_id =
                FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "MavFrame".to_string(),
                    value: tmp as u32,
                })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        _tmp.put_f32_le(self.x as f32);
        _tmp.put_f32_le(self.y as f32);
        _tmp.put_f32_le(self.z as f32);
        for val in &self.q {
            _tmp.put_f32_le(*val as f32);
        }
        _tmp.put_f32_le(self.vx as f32);
        _tmp.put_f32_le(self.vy as f32);
        _tmp.put_f32_le(self.vz as f32);
        _tmp.put_f32_le(self.rollspeed as f32);
        _tmp.put_f32_le(self.pitchspeed as f32);
        _tmp.put_f32_le(self.yawspeed as f32);
        for val in &self.pose_covariance {
            _tmp.put_f32_le(*val as f32);
        }
        for val in &self.velocity_covariance {
            _tmp.put_f32_le(*val as f32);
        }
        _tmp.put_u8(self.frame_id as u8);
        _tmp.put_u8(self.child_frame_id as u8);
        _tmp
    }
}
impl crate::proto::common::TrajectoryRepresentationWaypoints {
    pub const ENCODED_LEN: usize = 239usize;
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
            for _ in 0..5usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.pos_x.push(val.into());
            }
            for _ in 0..5usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.pos_y.push(val.into());
            }
            for _ in 0..5usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.pos_z.push(val.into());
            }
            for _ in 0..5usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.vel_x.push(val.into());
            }
            for _ in 0..5usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.vel_y.push(val.into());
            }
            for _ in 0..5usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.vel_z.push(val.into());
            }
            for _ in 0..5usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.acc_x.push(val.into());
            }
            for _ in 0..5usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.acc_y.push(val.into());
            }
            for _ in 0..5usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.acc_z.push(val.into());
            }
            for _ in 0..5usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.pos_yaw.push(val.into());
            }
            for _ in 0..5usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.vel_yaw.push(val.into());
            }
            for _ in 0..5usize {
                let val = buf.get_u16_le();
                #[allow(clippy::useless_conversion)]
                _struct.command.push(val.into());
            }
            _struct.valid_points = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        for val in &self.pos_x {
            _tmp.put_f32_le(*val as f32);
        }
        for val in &self.pos_y {
            _tmp.put_f32_le(*val as f32);
        }
        for val in &self.pos_z {
            _tmp.put_f32_le(*val as f32);
        }
        for val in &self.vel_x {
            _tmp.put_f32_le(*val as f32);
        }
        for val in &self.vel_y {
            _tmp.put_f32_le(*val as f32);
        }
        for val in &self.vel_z {
            _tmp.put_f32_le(*val as f32);
        }
        for val in &self.acc_x {
            _tmp.put_f32_le(*val as f32);
        }
        for val in &self.acc_y {
            _tmp.put_f32_le(*val as f32);
        }
        for val in &self.acc_z {
            _tmp.put_f32_le(*val as f32);
        }
        for val in &self.pos_yaw {
            _tmp.put_f32_le(*val as f32);
        }
        for val in &self.vel_yaw {
            _tmp.put_f32_le(*val as f32);
        }
        for val in &self.command {
            _tmp.put_u16_le(*val as u16);
        }
        _tmp.put_u8(self.valid_points as u8);
        _tmp
    }
}
impl crate::proto::common::TrajectoryRepresentationBezier {
    pub const ENCODED_LEN: usize = 109usize;
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
            for _ in 0..5usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.pos_x.push(val.into());
            }
            for _ in 0..5usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.pos_y.push(val.into());
            }
            for _ in 0..5usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.pos_z.push(val.into());
            }
            for _ in 0..5usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.delta.push(val.into());
            }
            for _ in 0..5usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.pos_yaw.push(val.into());
            }
            _struct.valid_points = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        for val in &self.pos_x {
            _tmp.put_f32_le(*val as f32);
        }
        for val in &self.pos_y {
            _tmp.put_f32_le(*val as f32);
        }
        for val in &self.pos_z {
            _tmp.put_f32_le(*val as f32);
        }
        for val in &self.delta {
            _tmp.put_f32_le(*val as f32);
        }
        for val in &self.pos_yaw {
            _tmp.put_f32_le(*val as f32);
        }
        _tmp.put_u8(self.valid_points as u8);
        _tmp
    }
}
impl crate::proto::common::CellularStatus {
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
            _struct.mcc = buf.get_u16_le() as u32;
            _struct.mnc = buf.get_u16_le() as u32;
            _struct.lac = buf.get_u16_le() as u32;
            let tmp = buf.get_u8();
            _struct.status = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "CellularStatusFlag".to_string(),
                value: tmp as u32,
            })?;
            let tmp = buf.get_u8();
            _struct.failure_reason =
                FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "CellularNetworkFailedReason".to_string(),
                    value: tmp as u32,
                })?;
            let tmp = buf.get_u8();
            _struct.r#type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "CellularNetworkRadioType".to_string(),
                value: tmp as u32,
            })?;
            _struct.quality = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.mcc as u16);
        _tmp.put_u16_le(self.mnc as u16);
        _tmp.put_u16_le(self.lac as u16);
        _tmp.put_u8(self.status as u8);
        _tmp.put_u8(self.failure_reason as u8);
        _tmp.put_u8(self.r#type as u8);
        _tmp.put_u8(self.quality as u8);
        _tmp
    }
}
impl crate::proto::common::IsbdLinkStatus {
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
            _struct.timestamp = buf.get_u64_le();
            _struct.last_heartbeat = buf.get_u64_le();
            _struct.failed_sessions = buf.get_u16_le() as u32;
            _struct.successful_sessions = buf.get_u16_le() as u32;
            _struct.signal_quality = buf.get_u8() as u32;
            _struct.ring_pending = buf.get_u8() as u32;
            _struct.tx_session_pending = buf.get_u8() as u32;
            _struct.rx_session_pending = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.timestamp as u64);
        _tmp.put_u64_le(self.last_heartbeat as u64);
        _tmp.put_u16_le(self.failed_sessions as u16);
        _tmp.put_u16_le(self.successful_sessions as u16);
        _tmp.put_u8(self.signal_quality as u8);
        _tmp.put_u8(self.ring_pending as u8);
        _tmp.put_u8(self.tx_session_pending as u8);
        _tmp.put_u8(self.rx_session_pending as u8);
        _tmp
    }
}
impl crate::proto::common::CellularConfig {
    pub const ENCODED_LEN: usize = 99usize;
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
            _struct.enable_pin = buf.get_u8() as u32;
            let mut s = Vec::with_capacity(32usize);
            for _ in 0..32usize {
                s.push(buf.get_u8());
            }
            _struct.pin = String::from_utf8_lossy(&s).into();
            let mut s = Vec::with_capacity(32usize);
            for _ in 0..32usize {
                s.push(buf.get_u8());
            }
            _struct.apn = String::from_utf8_lossy(&s).into();
            let mut s = Vec::with_capacity(32usize);
            for _ in 0..32usize {
                s.push(buf.get_u8());
            }
            _struct.puk = String::from_utf8_lossy(&s).into();
            _struct.roaming = buf.get_u8() as u32;
            let tmp = buf.get_u8();
            _struct.response = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "CellularConfigResponse".to_string(),
                value: tmp as u32,
            })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.enable_pin as u8);
        for val in self.pin.bytes() {
            _tmp.put_u8(val);
        }
        for val in self.apn.bytes() {
            _tmp.put_u8(val);
        }
        for val in self.puk.bytes() {
            _tmp.put_u8(val);
        }
        _tmp.put_u8(self.roaming as u8);
        _tmp.put_u8(self.response as u8);
        _tmp
    }
}
impl crate::proto::common::RawRpm {
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
            _struct.frequency = buf.get_f32_le();
            _struct.index = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.frequency as f32);
        _tmp.put_u8(self.index as u8);
        _tmp
    }
}
impl crate::proto::common::UtmGlobalPosition {
    pub const ENCODED_LEN: usize = 70usize;
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
            _struct.time = buf.get_u64_le();
            _struct.lat = buf.get_i32_le();
            _struct.lon = buf.get_i32_le();
            _struct.alt = buf.get_i32_le();
            _struct.relative_alt = buf.get_i32_le();
            _struct.next_lat = buf.get_i32_le();
            _struct.next_lon = buf.get_i32_le();
            _struct.next_alt = buf.get_i32_le();
            _struct.vx = buf.get_i16_le() as i32;
            _struct.vy = buf.get_i16_le() as i32;
            _struct.vz = buf.get_i16_le() as i32;
            _struct.h_acc = buf.get_u16_le() as u32;
            _struct.v_acc = buf.get_u16_le() as u32;
            _struct.vel_acc = buf.get_u16_le() as u32;
            _struct.update_rate = buf.get_u16_le() as u32;
            for _ in 0..18usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.uas_id.push(val.into());
            }
            let tmp = buf.get_u8();
            _struct.flight_state = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "UtmFlightState".to_string(),
                value: tmp as u32,
            })?;
            let tmp = buf.get_u8();
            _struct.flags = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "UtmDataAvailFlags".to_string(),
                value: tmp as u32,
            })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time as u64);
        _tmp.put_i32_le(self.lat as i32);
        _tmp.put_i32_le(self.lon as i32);
        _tmp.put_i32_le(self.alt as i32);
        _tmp.put_i32_le(self.relative_alt as i32);
        _tmp.put_i32_le(self.next_lat as i32);
        _tmp.put_i32_le(self.next_lon as i32);
        _tmp.put_i32_le(self.next_alt as i32);
        _tmp.put_i16_le(self.vx as i16);
        _tmp.put_i16_le(self.vy as i16);
        _tmp.put_i16_le(self.vz as i16);
        _tmp.put_u16_le(self.h_acc as u16);
        _tmp.put_u16_le(self.v_acc as u16);
        _tmp.put_u16_le(self.vel_acc as u16);
        _tmp.put_u16_le(self.update_rate as u16);
        for val in &self.uas_id {
            _tmp.put_u8(*val as u8);
        }
        _tmp.put_u8(self.flight_state as u8);
        _tmp.put_u8(self.flags as u8);
        _tmp
    }
}
impl crate::proto::common::DebugFloatArray {
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
            _struct.time_usec = buf.get_u64_le();
            _struct.array_id = buf.get_u16_le() as u32;
            let mut s = Vec::with_capacity(10usize);
            for _ in 0..10usize {
                s.push(buf.get_u8());
            }
            _struct.name = String::from_utf8_lossy(&s).into();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        _tmp.put_u16_le(self.array_id as u16);
        for val in self.name.bytes() {
            _tmp.put_u8(val);
        }
        _tmp
    }
}
impl crate::proto::common::OrbitExecutionStatus {
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
            _struct.time_usec = buf.get_u64_le();
            _struct.radius = buf.get_f32_le();
            _struct.x = buf.get_i32_le();
            _struct.y = buf.get_i32_le();
            _struct.z = buf.get_f32_le();
            let tmp = buf.get_u8();
            _struct.frame = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavFrame".to_string(),
                value: tmp as u32,
            })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        _tmp.put_f32_le(self.radius as f32);
        _tmp.put_i32_le(self.x as i32);
        _tmp.put_i32_le(self.y as i32);
        _tmp.put_f32_le(self.z as f32);
        _tmp.put_u8(self.frame as u8);
        _tmp
    }
}
impl crate::proto::common::SmartBatteryInfo {
    pub const ENCODED_LEN: usize = 73usize;
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
            _struct.capacity_full_specification = buf.get_i32_le();
            _struct.capacity_full = buf.get_i32_le();
            _struct.serial_number = buf.get_i32_le();
            _struct.cycle_count = buf.get_u16_le() as u32;
            _struct.weight = buf.get_u16_le() as u32;
            _struct.discharge_minimum_voltage = buf.get_u16_le() as u32;
            _struct.charging_minimum_voltage = buf.get_u16_le() as u32;
            _struct.resting_minimum_voltage = buf.get_u16_le() as u32;
            _struct.id = buf.get_u8() as u32;
            let mut s = Vec::with_capacity(50usize);
            for _ in 0..50usize {
                s.push(buf.get_u8());
            }
            _struct.device_name = String::from_utf8_lossy(&s).into();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i32_le(self.capacity_full_specification as i32);
        _tmp.put_i32_le(self.capacity_full as i32);
        _tmp.put_i32_le(self.serial_number as i32);
        _tmp.put_u16_le(self.cycle_count as u16);
        _tmp.put_u16_le(self.weight as u16);
        _tmp.put_u16_le(self.discharge_minimum_voltage as u16);
        _tmp.put_u16_le(self.charging_minimum_voltage as u16);
        _tmp.put_u16_le(self.resting_minimum_voltage as u16);
        _tmp.put_u8(self.id as u8);
        for val in self.device_name.bytes() {
            _tmp.put_u8(val);
        }
        _tmp
    }
}
impl crate::proto::common::SmartBatteryStatus {
    pub const ENCODED_LEN: usize = 50usize;
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
            let tmp = buf.get_i32_le();
            _struct.fault_bitmask =
                FromPrimitive::from_i32(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "MavSmartBatteryFault".to_string(),
                    value: tmp as u32,
                })?;
            _struct.time_remaining = buf.get_i32_le();
            _struct.id = buf.get_u16_le() as u32;
            _struct.capacity_remaining = buf.get_i16_le() as i32;
            _struct.current = buf.get_i16_le() as i32;
            _struct.temperature = buf.get_i16_le() as i32;
            _struct.cell_offset = buf.get_u16_le() as u32;
            for _ in 0..16usize {
                let val = buf.get_u16_le() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.voltages.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i32_le(self.fault_bitmask as i32);
        _tmp.put_i32_le(self.time_remaining as i32);
        _tmp.put_u16_le(self.id as u16);
        _tmp.put_i16_le(self.capacity_remaining as i16);
        _tmp.put_i16_le(self.current as i16);
        _tmp.put_i16_le(self.temperature as i16);
        _tmp.put_u16_le(self.cell_offset as u16);
        for val in &self.voltages {
            _tmp.put_u16_le(*val as u16);
        }
        _tmp
    }
}
impl crate::proto::common::GeneratorStatus {
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
            let tmp = buf.get_u64_le();
            _struct.status = FromPrimitive::from_u64(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavGeneratorStatusFlag".to_string(),
                value: tmp as u32,
            })?;
            _struct.battery_current = buf.get_f32_le();
            _struct.load_current = buf.get_f32_le();
            _struct.power_generated = buf.get_f32_le();
            _struct.bus_voltage = buf.get_f32_le();
            _struct.bat_current_setpoint = buf.get_f32_le();
            _struct.generator_speed = buf.get_u16_le() as u32;
            _struct.rectifier_temperature = buf.get_i16_le() as i32;
            _struct.generator_temperature = buf.get_i16_le() as i32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.status as u64);
        _tmp.put_f32_le(self.battery_current as f32);
        _tmp.put_f32_le(self.load_current as f32);
        _tmp.put_f32_le(self.power_generated as f32);
        _tmp.put_f32_le(self.bus_voltage as f32);
        _tmp.put_f32_le(self.bat_current_setpoint as f32);
        _tmp.put_u16_le(self.generator_speed as u16);
        _tmp.put_i16_le(self.rectifier_temperature as i16);
        _tmp.put_i16_le(self.generator_temperature as i16);
        _tmp
    }
}
impl crate::proto::common::ActuatorOutputStatus {
    pub const ENCODED_LEN: usize = 140usize;
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
            _struct.active = buf.get_u32_le();
            for _ in 0..32usize {
                let val = buf.get_f32_le();
                #[allow(clippy::useless_conversion)]
                _struct.actuator.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        _tmp.put_u32_le(self.active as u32);
        for val in &self.actuator {
            _tmp.put_f32_le(*val as f32);
        }
        _tmp
    }
}
impl crate::proto::common::TimeEstimateToTarget {
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
            _struct.safe_return = buf.get_i32_le();
            _struct.land = buf.get_i32_le();
            _struct.mission_next_item = buf.get_i32_le();
            _struct.mission_end = buf.get_i32_le();
            _struct.commanded_action = buf.get_i32_le();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i32_le(self.safe_return as i32);
        _tmp.put_i32_le(self.land as i32);
        _tmp.put_i32_le(self.mission_next_item as i32);
        _tmp.put_i32_le(self.mission_end as i32);
        _tmp.put_i32_le(self.commanded_action as i32);
        _tmp
    }
}
impl crate::proto::common::Tunnel {
    pub const ENCODED_LEN: usize = 133usize;
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
            _struct.payload_type =
                FromPrimitive::from_u16(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "MavTunnelPayloadType".to_string(),
                    value: tmp as u32,
                })?;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            _struct.payload_length = buf.get_u8() as u32;
            for _ in 0..128usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.payload.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.payload_type as u16);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp.put_u8(self.payload_length as u8);
        for val in &self.payload {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
impl crate::proto::common::OnboardComputerStatus {
    pub const ENCODED_LEN: usize = 238usize;
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
            _struct.uptime = buf.get_u32_le();
            _struct.ram_usage = buf.get_u32_le();
            _struct.ram_total = buf.get_u32_le();
            for _ in 0..4usize {
                let val = buf.get_u32_le();
                #[allow(clippy::useless_conversion)]
                _struct.storage_type.push(val.into());
            }
            for _ in 0..4usize {
                let val = buf.get_u32_le();
                #[allow(clippy::useless_conversion)]
                _struct.storage_usage.push(val.into());
            }
            for _ in 0..4usize {
                let val = buf.get_u32_le();
                #[allow(clippy::useless_conversion)]
                _struct.storage_total.push(val.into());
            }
            for _ in 0..6usize {
                let val = buf.get_u32_le();
                #[allow(clippy::useless_conversion)]
                _struct.link_type.push(val.into());
            }
            for _ in 0..6usize {
                let val = buf.get_u32_le();
                #[allow(clippy::useless_conversion)]
                _struct.link_tx_rate.push(val.into());
            }
            for _ in 0..6usize {
                let val = buf.get_u32_le();
                #[allow(clippy::useless_conversion)]
                _struct.link_rx_rate.push(val.into());
            }
            for _ in 0..6usize {
                let val = buf.get_u32_le();
                #[allow(clippy::useless_conversion)]
                _struct.link_tx_max.push(val.into());
            }
            for _ in 0..6usize {
                let val = buf.get_u32_le();
                #[allow(clippy::useless_conversion)]
                _struct.link_rx_max.push(val.into());
            }
            for _ in 0..4usize {
                let val = buf.get_i16_le() as i32;
                #[allow(clippy::useless_conversion)]
                _struct.fan_speed.push(val.into());
            }
            _struct.r#type = buf.get_u8() as u32;
            for _ in 0..8usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.cpu_cores.push(val.into());
            }
            for _ in 0..10usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.cpu_combined.push(val.into());
            }
            for _ in 0..4usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.gpu_cores.push(val.into());
            }
            for _ in 0..10usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.gpu_combined.push(val.into());
            }
            _struct.temperature_board = buf.get_i8() as i32;
            for _ in 0..8usize {
                let val = buf.get_i8() as i32;
                #[allow(clippy::useless_conversion)]
                _struct.temperature_core.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        _tmp.put_u32_le(self.uptime as u32);
        _tmp.put_u32_le(self.ram_usage as u32);
        _tmp.put_u32_le(self.ram_total as u32);
        for val in &self.storage_type {
            _tmp.put_u32_le(*val as u32);
        }
        for val in &self.storage_usage {
            _tmp.put_u32_le(*val as u32);
        }
        for val in &self.storage_total {
            _tmp.put_u32_le(*val as u32);
        }
        for val in &self.link_type {
            _tmp.put_u32_le(*val as u32);
        }
        for val in &self.link_tx_rate {
            _tmp.put_u32_le(*val as u32);
        }
        for val in &self.link_rx_rate {
            _tmp.put_u32_le(*val as u32);
        }
        for val in &self.link_tx_max {
            _tmp.put_u32_le(*val as u32);
        }
        for val in &self.link_rx_max {
            _tmp.put_u32_le(*val as u32);
        }
        for val in &self.fan_speed {
            _tmp.put_i16_le(*val as i16);
        }
        _tmp.put_u8(self.r#type as u8);
        for val in &self.cpu_cores {
            _tmp.put_u8(*val as u8);
        }
        for val in &self.cpu_combined {
            _tmp.put_u8(*val as u8);
        }
        for val in &self.gpu_cores {
            _tmp.put_u8(*val as u8);
        }
        for val in &self.gpu_combined {
            _tmp.put_u8(*val as u8);
        }
        _tmp.put_i8(self.temperature_board as i8);
        for val in &self.temperature_core {
            _tmp.put_i8(*val as i8);
        }
        _tmp
    }
}
impl crate::proto::common::ComponentInformation {
    pub const ENCODED_LEN: usize = 156usize;
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
            let tmp = buf.get_u32_le();
            _struct.metadata_type =
                FromPrimitive::from_u32(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "CompMetadataType".to_string(),
                    value: tmp as u32,
                })?;
            _struct.metadata_uid = buf.get_u32_le();
            _struct.translation_uid = buf.get_u32_le();
            let mut s = Vec::with_capacity(70usize);
            for _ in 0..70usize {
                s.push(buf.get_u8());
            }
            _struct.metadata_uri = String::from_utf8_lossy(&s).into();
            let mut s = Vec::with_capacity(70usize);
            for _ in 0..70usize {
                s.push(buf.get_u8());
            }
            _struct.translation_uri = String::from_utf8_lossy(&s).into();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms as u32);
        _tmp.put_u32_le(self.metadata_type as u32);
        _tmp.put_u32_le(self.metadata_uid as u32);
        _tmp.put_u32_le(self.translation_uid as u32);
        for val in self.metadata_uri.bytes() {
            _tmp.put_u8(val);
        }
        for val in self.translation_uri.bytes() {
            _tmp.put_u8(val);
        }
        _tmp
    }
}
impl crate::proto::common::PlayTuneV2 {
    pub const ENCODED_LEN: usize = 254usize;
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
            _struct.format = FromPrimitive::from_u32(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "TuneFormat".to_string(),
                value: tmp as u32,
            })?;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            let mut s = Vec::with_capacity(248usize);
            for _ in 0..248usize {
                s.push(buf.get_u8());
            }
            _struct.tune = String::from_utf8_lossy(&s).into();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.format as u32);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        for val in self.tune.bytes() {
            _tmp.put_u8(val);
        }
        _tmp
    }
}
impl crate::proto::common::SupportedTunes {
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
            let tmp = buf.get_u32_le();
            _struct.format = FromPrimitive::from_u32(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "TuneFormat".to_string(),
                value: tmp as u32,
            })?;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.format as u32);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp
    }
}
impl crate::proto::common::WheelDistance {
    pub const ENCODED_LEN: usize = 137usize;
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
            for _ in 0..16usize {
                let val = buf.get_f64_le();
                #[allow(clippy::useless_conversion)]
                _struct.distance.push(val.into());
            }
            _struct.count = buf.get_u8() as u32;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec as u64);
        for val in &self.distance {
            _tmp.put_f64_le(*val as f64);
        }
        _tmp.put_u8(self.count as u8);
        _tmp
    }
}
impl crate::proto::common::OpenDroneIdBasicId {
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
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            for _ in 0..20usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.id_or_mac.push(val.into());
            }
            let tmp = buf.get_u8();
            _struct.id_type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavOdidIdType".to_string(),
                value: tmp as u32,
            })?;
            let tmp = buf.get_u8();
            _struct.ua_type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavOdidUaType".to_string(),
                value: tmp as u32,
            })?;
            for _ in 0..20usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.uas_id.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        for val in &self.id_or_mac {
            _tmp.put_u8(*val as u8);
        }
        _tmp.put_u8(self.id_type as u8);
        _tmp.put_u8(self.ua_type as u8);
        for val in &self.uas_id {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
impl crate::proto::common::OpenDroneIdLocation {
    pub const ENCODED_LEN: usize = 59usize;
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
            _struct.latitude = buf.get_i32_le();
            _struct.longitude = buf.get_i32_le();
            _struct.altitude_barometric = buf.get_f32_le();
            _struct.altitude_geodetic = buf.get_f32_le();
            _struct.height = buf.get_f32_le();
            _struct.timestamp = buf.get_f32_le();
            _struct.direction = buf.get_u16_le() as u32;
            _struct.speed_horizontal = buf.get_u16_le() as u32;
            _struct.speed_vertical = buf.get_i16_le() as i32;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            for _ in 0..20usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.id_or_mac.push(val.into());
            }
            let tmp = buf.get_u8();
            _struct.status = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavOdidStatus".to_string(),
                value: tmp as u32,
            })?;
            let tmp = buf.get_u8();
            _struct.height_reference =
                FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "MavOdidHeightRef".to_string(),
                    value: tmp as u32,
                })?;
            let tmp = buf.get_u8();
            _struct.horizontal_accuracy =
                FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "MavOdidHorAcc".to_string(),
                    value: tmp as u32,
                })?;
            let tmp = buf.get_u8();
            _struct.vertical_accuracy =
                FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "MavOdidVerAcc".to_string(),
                    value: tmp as u32,
                })?;
            let tmp = buf.get_u8();
            _struct.barometer_accuracy =
                FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "MavOdidVerAcc".to_string(),
                    value: tmp as u32,
                })?;
            let tmp = buf.get_u8();
            _struct.speed_accuracy =
                FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "MavOdidSpeedAcc".to_string(),
                    value: tmp as u32,
                })?;
            let tmp = buf.get_u8();
            _struct.timestamp_accuracy =
                FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "MavOdidTimeAcc".to_string(),
                    value: tmp as u32,
                })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i32_le(self.latitude as i32);
        _tmp.put_i32_le(self.longitude as i32);
        _tmp.put_f32_le(self.altitude_barometric as f32);
        _tmp.put_f32_le(self.altitude_geodetic as f32);
        _tmp.put_f32_le(self.height as f32);
        _tmp.put_f32_le(self.timestamp as f32);
        _tmp.put_u16_le(self.direction as u16);
        _tmp.put_u16_le(self.speed_horizontal as u16);
        _tmp.put_i16_le(self.speed_vertical as i16);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        for val in &self.id_or_mac {
            _tmp.put_u8(*val as u8);
        }
        _tmp.put_u8(self.status as u8);
        _tmp.put_u8(self.height_reference as u8);
        _tmp.put_u8(self.horizontal_accuracy as u8);
        _tmp.put_u8(self.vertical_accuracy as u8);
        _tmp.put_u8(self.barometer_accuracy as u8);
        _tmp.put_u8(self.speed_accuracy as u8);
        _tmp.put_u8(self.timestamp_accuracy as u8);
        _tmp
    }
}
impl crate::proto::common::OpenDroneIdAuthentication {
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
            _struct.timestamp = buf.get_u32_le();
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            for _ in 0..20usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.id_or_mac.push(val.into());
            }
            let tmp = buf.get_u8();
            _struct.authentication_type =
                FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "MavOdidAuthType".to_string(),
                    value: tmp as u32,
                })?;
            _struct.data_page = buf.get_u8() as u32;
            _struct.page_count = buf.get_u8() as u32;
            _struct.length = buf.get_u8() as u32;
            for _ in 0..23usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.authentication_data.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.timestamp as u32);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        for val in &self.id_or_mac {
            _tmp.put_u8(*val as u8);
        }
        _tmp.put_u8(self.authentication_type as u8);
        _tmp.put_u8(self.data_page as u8);
        _tmp.put_u8(self.page_count as u8);
        _tmp.put_u8(self.length as u8);
        for val in &self.authentication_data {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
impl crate::proto::common::OpenDroneIdSelfId {
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
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            for _ in 0..20usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.id_or_mac.push(val.into());
            }
            let tmp = buf.get_u8();
            _struct.description_type =
                FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "MavOdidDescType".to_string(),
                    value: tmp as u32,
                })?;
            let mut s = Vec::with_capacity(23usize);
            for _ in 0..23usize {
                s.push(buf.get_u8());
            }
            _struct.description = String::from_utf8_lossy(&s).into();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        for val in &self.id_or_mac {
            _tmp.put_u8(*val as u8);
        }
        _tmp.put_u8(self.description_type as u8);
        for val in self.description.bytes() {
            _tmp.put_u8(val);
        }
        _tmp
    }
}
impl crate::proto::common::OpenDroneIdSystem {
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
            _struct.operator_latitude = buf.get_i32_le();
            _struct.operator_longitude = buf.get_i32_le();
            _struct.area_ceiling = buf.get_f32_le();
            _struct.area_floor = buf.get_f32_le();
            _struct.area_count = buf.get_u16_le() as u32;
            _struct.area_radius = buf.get_u16_le() as u32;
            _struct.target_system = buf.get_u8() as u32;
            _struct.target_component = buf.get_u8() as u32;
            for _ in 0..20usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.id_or_mac.push(val.into());
            }
            let tmp = buf.get_u8();
            _struct.operator_location_type =
                FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "MavOdidOperatorLocationType".to_string(),
                    value: tmp as u32,
                })?;
            let tmp = buf.get_u8();
            _struct.classification_type =
                FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "MavOdidClassificationType".to_string(),
                    value: tmp as u32,
                })?;
            let tmp = buf.get_u8();
            _struct.category_eu = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavOdidCategoryEu".to_string(),
                value: tmp as u32,
            })?;
            let tmp = buf.get_u8();
            _struct.class_eu = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavOdidClassEu".to_string(),
                value: tmp as u32,
            })?;
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i32_le(self.operator_latitude as i32);
        _tmp.put_i32_le(self.operator_longitude as i32);
        _tmp.put_f32_le(self.area_ceiling as f32);
        _tmp.put_f32_le(self.area_floor as f32);
        _tmp.put_u16_le(self.area_count as u16);
        _tmp.put_u16_le(self.area_radius as u16);
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        for val in &self.id_or_mac {
            _tmp.put_u8(*val as u8);
        }
        _tmp.put_u8(self.operator_location_type as u8);
        _tmp.put_u8(self.classification_type as u8);
        _tmp.put_u8(self.category_eu as u8);
        _tmp.put_u8(self.class_eu as u8);
        _tmp
    }
}
impl crate::proto::common::OpenDroneIdOperatorId {
    pub const ENCODED_LEN: usize = 43usize;
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
            for _ in 0..20usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.id_or_mac.push(val.into());
            }
            let tmp = buf.get_u8();
            _struct.operator_id_type =
                FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                    enum_type: "MavOdidOperatorIdType".to_string(),
                    value: tmp as u32,
                })?;
            let mut s = Vec::with_capacity(20usize);
            for _ in 0..20usize {
                s.push(buf.get_u8());
            }
            _struct.operator_id = String::from_utf8_lossy(&s).into();
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        for val in &self.id_or_mac {
            _tmp.put_u8(*val as u8);
        }
        _tmp.put_u8(self.operator_id_type as u8);
        for val in self.operator_id.bytes() {
            _tmp.put_u8(val);
        }
        _tmp
    }
}
impl crate::proto::common::OpenDroneIdMessagePack {
    pub const ENCODED_LEN: usize = 254usize;
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
            _struct.single_message_size = buf.get_u8() as u32;
            _struct.msg_pack_size = buf.get_u8() as u32;
            for _ in 0..250usize {
                let val = buf.get_u8() as u32;
                #[allow(clippy::useless_conversion)]
                _struct.messages.push(val.into());
            }
            Ok(_struct)
        }
    }
    pub fn mavlink_ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.target_system as u8);
        _tmp.put_u8(self.target_component as u8);
        _tmp.put_u8(self.single_message_size as u8);
        _tmp.put_u8(self.msg_pack_size as u8);
        for val in &self.messages {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
#[derive(Clone, PartialEq, Debug)]
pub enum MavMessage {
    Heartbeat(crate::proto::common::Heartbeat),
    SysStatus(crate::proto::common::SysStatus),
    SystemTime(crate::proto::common::SystemTime),
    Ping(crate::proto::common::Ping),
    ChangeOperatorControl(crate::proto::common::ChangeOperatorControl),
    ChangeOperatorControlAck(crate::proto::common::ChangeOperatorControlAck),
    AuthKey(crate::proto::common::AuthKey),
    LinkNodeStatus(crate::proto::common::LinkNodeStatus),
    SetMode(crate::proto::common::SetMode),
    ParamRequestRead(crate::proto::common::ParamRequestRead),
    ParamRequestList(crate::proto::common::ParamRequestList),
    ParamValue(crate::proto::common::ParamValue),
    ParamSet(crate::proto::common::ParamSet),
    GpsRawInt(crate::proto::common::GpsRawInt),
    GpsStatus(crate::proto::common::GpsStatus),
    ScaledImu(crate::proto::common::ScaledImu),
    RawImu(crate::proto::common::RawImu),
    RawPressure(crate::proto::common::RawPressure),
    ScaledPressure(crate::proto::common::ScaledPressure),
    Attitude(crate::proto::common::Attitude),
    AttitudeQuaternion(crate::proto::common::AttitudeQuaternion),
    LocalPositionNed(crate::proto::common::LocalPositionNed),
    GlobalPositionInt(crate::proto::common::GlobalPositionInt),
    RcChannelsScaled(crate::proto::common::RcChannelsScaled),
    RcChannelsRaw(crate::proto::common::RcChannelsRaw),
    ServoOutputRaw(crate::proto::common::ServoOutputRaw),
    MissionRequestPartialList(crate::proto::common::MissionRequestPartialList),
    MissionWritePartialList(crate::proto::common::MissionWritePartialList),
    MissionItem(crate::proto::common::MissionItem),
    MissionRequest(crate::proto::common::MissionRequest),
    MissionSetCurrent(crate::proto::common::MissionSetCurrent),
    MissionCurrent(crate::proto::common::MissionCurrent),
    MissionRequestList(crate::proto::common::MissionRequestList),
    MissionCount(crate::proto::common::MissionCount),
    MissionClearAll(crate::proto::common::MissionClearAll),
    MissionItemReached(crate::proto::common::MissionItemReached),
    MissionAck(crate::proto::common::MissionAck),
    SetGpsGlobalOrigin(crate::proto::common::SetGpsGlobalOrigin),
    GpsGlobalOrigin(crate::proto::common::GpsGlobalOrigin),
    ParamMapRc(crate::proto::common::ParamMapRc),
    MissionRequestInt(crate::proto::common::MissionRequestInt),
    MissionChanged(crate::proto::common::MissionChanged),
    SafetySetAllowedArea(crate::proto::common::SafetySetAllowedArea),
    SafetyAllowedArea(crate::proto::common::SafetyAllowedArea),
    AttitudeQuaternionCov(crate::proto::common::AttitudeQuaternionCov),
    NavControllerOutput(crate::proto::common::NavControllerOutput),
    GlobalPositionIntCov(crate::proto::common::GlobalPositionIntCov),
    LocalPositionNedCov(crate::proto::common::LocalPositionNedCov),
    RcChannels(crate::proto::common::RcChannels),
    RequestDataStream(crate::proto::common::RequestDataStream),
    DataStream(crate::proto::common::DataStream),
    ManualControl(crate::proto::common::ManualControl),
    RcChannelsOverride(crate::proto::common::RcChannelsOverride),
    MissionItemInt(crate::proto::common::MissionItemInt),
    VfrHud(crate::proto::common::VfrHud),
    CommandInt(crate::proto::common::CommandInt),
    CommandLong(crate::proto::common::CommandLong),
    CommandAck(crate::proto::common::CommandAck),
    CommandCancel(crate::proto::common::CommandCancel),
    ManualSetpoint(crate::proto::common::ManualSetpoint),
    SetAttitudeTarget(crate::proto::common::SetAttitudeTarget),
    AttitudeTarget(crate::proto::common::AttitudeTarget),
    SetPositionTargetLocalNed(crate::proto::common::SetPositionTargetLocalNed),
    PositionTargetLocalNed(crate::proto::common::PositionTargetLocalNed),
    SetPositionTargetGlobalInt(crate::proto::common::SetPositionTargetGlobalInt),
    PositionTargetGlobalInt(crate::proto::common::PositionTargetGlobalInt),
    LocalPositionNedSystemGlobalOffset(crate::proto::common::LocalPositionNedSystemGlobalOffset),
    HilState(crate::proto::common::HilState),
    HilControls(crate::proto::common::HilControls),
    HilRcInputsRaw(crate::proto::common::HilRcInputsRaw),
    HilActuatorControls(crate::proto::common::HilActuatorControls),
    OpticalFlow(crate::proto::common::OpticalFlow),
    GlobalVisionPositionEstimate(crate::proto::common::GlobalVisionPositionEstimate),
    VisionPositionEstimate(crate::proto::common::VisionPositionEstimate),
    VisionSpeedEstimate(crate::proto::common::VisionSpeedEstimate),
    ViconPositionEstimate(crate::proto::common::ViconPositionEstimate),
    HighresImu(crate::proto::common::HighresImu),
    OpticalFlowRad(crate::proto::common::OpticalFlowRad),
    HilSensor(crate::proto::common::HilSensor),
    SimState(crate::proto::common::SimState),
    RadioStatus(crate::proto::common::RadioStatus),
    FileTransferProtocol(crate::proto::common::FileTransferProtocol),
    Timesync(crate::proto::common::Timesync),
    CameraTrigger(crate::proto::common::CameraTrigger),
    HilGps(crate::proto::common::HilGps),
    HilOpticalFlow(crate::proto::common::HilOpticalFlow),
    HilStateQuaternion(crate::proto::common::HilStateQuaternion),
    ScaledImu2(crate::proto::common::ScaledImu2),
    LogRequestList(crate::proto::common::LogRequestList),
    LogEntry(crate::proto::common::LogEntry),
    LogRequestData(crate::proto::common::LogRequestData),
    LogData(crate::proto::common::LogData),
    LogErase(crate::proto::common::LogErase),
    LogRequestEnd(crate::proto::common::LogRequestEnd),
    GpsInjectData(crate::proto::common::GpsInjectData),
    Gps2Raw(crate::proto::common::Gps2Raw),
    PowerStatus(crate::proto::common::PowerStatus),
    SerialControl(crate::proto::common::SerialControl),
    GpsRtk(crate::proto::common::GpsRtk),
    Gps2Rtk(crate::proto::common::Gps2Rtk),
    ScaledImu3(crate::proto::common::ScaledImu3),
    DataTransmissionHandshake(crate::proto::common::DataTransmissionHandshake),
    EncapsulatedData(crate::proto::common::EncapsulatedData),
    DistanceSensor(crate::proto::common::DistanceSensor),
    TerrainRequest(crate::proto::common::TerrainRequest),
    TerrainData(crate::proto::common::TerrainData),
    TerrainCheck(crate::proto::common::TerrainCheck),
    TerrainReport(crate::proto::common::TerrainReport),
    ScaledPressure2(crate::proto::common::ScaledPressure2),
    AttPosMocap(crate::proto::common::AttPosMocap),
    SetActuatorControlTarget(crate::proto::common::SetActuatorControlTarget),
    ActuatorControlTarget(crate::proto::common::ActuatorControlTarget),
    Altitude(crate::proto::common::Altitude),
    ResourceRequest(crate::proto::common::ResourceRequest),
    ScaledPressure3(crate::proto::common::ScaledPressure3),
    FollowTarget(crate::proto::common::FollowTarget),
    ControlSystemState(crate::proto::common::ControlSystemState),
    BatteryStatus(crate::proto::common::BatteryStatus),
    AutopilotVersion(crate::proto::common::AutopilotVersion),
    LandingTarget(crate::proto::common::LandingTarget),
    FenceStatus(crate::proto::common::FenceStatus),
    EstimatorStatus(crate::proto::common::EstimatorStatus),
    WindCov(crate::proto::common::WindCov),
    GpsInput(crate::proto::common::GpsInput),
    GpsRtcmData(crate::proto::common::GpsRtcmData),
    HighLatency(crate::proto::common::HighLatency),
    HighLatency2(crate::proto::common::HighLatency2),
    Vibration(crate::proto::common::Vibration),
    HomePosition(crate::proto::common::HomePosition),
    SetHomePosition(crate::proto::common::SetHomePosition),
    MessageInterval(crate::proto::common::MessageInterval),
    ExtendedSysState(crate::proto::common::ExtendedSysState),
    AdsbVehicle(crate::proto::common::AdsbVehicle),
    Collision(crate::proto::common::Collision),
    V2Extension(crate::proto::common::V2Extension),
    MemoryVect(crate::proto::common::MemoryVect),
    DebugVect(crate::proto::common::DebugVect),
    NamedValueFloat(crate::proto::common::NamedValueFloat),
    NamedValueInt(crate::proto::common::NamedValueInt),
    Statustext(crate::proto::common::Statustext),
    Debug(crate::proto::common::Debug),
    SetupSigning(crate::proto::common::SetupSigning),
    ButtonChange(crate::proto::common::ButtonChange),
    PlayTune(crate::proto::common::PlayTune),
    CameraInformation(crate::proto::common::CameraInformation),
    CameraSettings(crate::proto::common::CameraSettings),
    StorageInformation(crate::proto::common::StorageInformation),
    CameraCaptureStatus(crate::proto::common::CameraCaptureStatus),
    CameraImageCaptured(crate::proto::common::CameraImageCaptured),
    FlightInformation(crate::proto::common::FlightInformation),
    MountOrientation(crate::proto::common::MountOrientation),
    LoggingData(crate::proto::common::LoggingData),
    LoggingDataAcked(crate::proto::common::LoggingDataAcked),
    LoggingAck(crate::proto::common::LoggingAck),
    VideoStreamInformation(crate::proto::common::VideoStreamInformation),
    VideoStreamStatus(crate::proto::common::VideoStreamStatus),
    GimbalManagerInformation(crate::proto::common::GimbalManagerInformation),
    GimbalManagerStatus(crate::proto::common::GimbalManagerStatus),
    GimbalManagerSetAttitude(crate::proto::common::GimbalManagerSetAttitude),
    GimbalDeviceInformation(crate::proto::common::GimbalDeviceInformation),
    GimbalDeviceSetAttitude(crate::proto::common::GimbalDeviceSetAttitude),
    GimbalDeviceAttitudeStatus(crate::proto::common::GimbalDeviceAttitudeStatus),
    AutopilotStateForGimbalDevice(crate::proto::common::AutopilotStateForGimbalDevice),
    GimbalManagerSetTiltpan(crate::proto::common::GimbalManagerSetTiltpan),
    WifiConfigAp(crate::proto::common::WifiConfigAp),
    ProtocolVersion(crate::proto::common::ProtocolVersion),
    AisVessel(crate::proto::common::AisVessel),
    UavcanNodeStatus(crate::proto::common::UavcanNodeStatus),
    UavcanNodeInfo(crate::proto::common::UavcanNodeInfo),
    ParamExtRequestRead(crate::proto::common::ParamExtRequestRead),
    ParamExtRequestList(crate::proto::common::ParamExtRequestList),
    ParamExtValue(crate::proto::common::ParamExtValue),
    ParamExtSet(crate::proto::common::ParamExtSet),
    ParamExtAck(crate::proto::common::ParamExtAck),
    ObstacleDistance(crate::proto::common::ObstacleDistance),
    Odometry(crate::proto::common::Odometry),
    TrajectoryRepresentationWaypoints(crate::proto::common::TrajectoryRepresentationWaypoints),
    TrajectoryRepresentationBezier(crate::proto::common::TrajectoryRepresentationBezier),
    CellularStatus(crate::proto::common::CellularStatus),
    IsbdLinkStatus(crate::proto::common::IsbdLinkStatus),
    CellularConfig(crate::proto::common::CellularConfig),
    RawRpm(crate::proto::common::RawRpm),
    UtmGlobalPosition(crate::proto::common::UtmGlobalPosition),
    DebugFloatArray(crate::proto::common::DebugFloatArray),
    OrbitExecutionStatus(crate::proto::common::OrbitExecutionStatus),
    SmartBatteryInfo(crate::proto::common::SmartBatteryInfo),
    SmartBatteryStatus(crate::proto::common::SmartBatteryStatus),
    GeneratorStatus(crate::proto::common::GeneratorStatus),
    ActuatorOutputStatus(crate::proto::common::ActuatorOutputStatus),
    TimeEstimateToTarget(crate::proto::common::TimeEstimateToTarget),
    Tunnel(crate::proto::common::Tunnel),
    OnboardComputerStatus(crate::proto::common::OnboardComputerStatus),
    ComponentInformation(crate::proto::common::ComponentInformation),
    PlayTuneV2(crate::proto::common::PlayTuneV2),
    SupportedTunes(crate::proto::common::SupportedTunes),
    WheelDistance(crate::proto::common::WheelDistance),
    OpenDroneIdBasicId(crate::proto::common::OpenDroneIdBasicId),
    OpenDroneIdLocation(crate::proto::common::OpenDroneIdLocation),
    OpenDroneIdAuthentication(crate::proto::common::OpenDroneIdAuthentication),
    OpenDroneIdSelfId(crate::proto::common::OpenDroneIdSelfId),
    OpenDroneIdSystem(crate::proto::common::OpenDroneIdSystem),
    OpenDroneIdOperatorId(crate::proto::common::OpenDroneIdOperatorId),
    OpenDroneIdMessagePack(crate::proto::common::OpenDroneIdMessagePack),
}
impl Message for MavMessage {
    fn parse(version: MavlinkVersion, id: u32, payload: &[u8]) -> Result<MavMessage, ParserError> {
        match id {
            0 => crate::proto::common::Heartbeat::mavlink_deser(version, payload)
                .map(MavMessage::Heartbeat),
            1 => crate::proto::common::SysStatus::mavlink_deser(version, payload)
                .map(MavMessage::SysStatus),
            2 => crate::proto::common::SystemTime::mavlink_deser(version, payload)
                .map(MavMessage::SystemTime),
            4 => crate::proto::common::Ping::mavlink_deser(version, payload).map(MavMessage::Ping),
            5 => crate::proto::common::ChangeOperatorControl::mavlink_deser(version, payload)
                .map(MavMessage::ChangeOperatorControl),
            6 => crate::proto::common::ChangeOperatorControlAck::mavlink_deser(version, payload)
                .map(MavMessage::ChangeOperatorControlAck),
            7 => crate::proto::common::AuthKey::mavlink_deser(version, payload)
                .map(MavMessage::AuthKey),
            8 => crate::proto::common::LinkNodeStatus::mavlink_deser(version, payload)
                .map(MavMessage::LinkNodeStatus),
            11 => crate::proto::common::SetMode::mavlink_deser(version, payload)
                .map(MavMessage::SetMode),
            20 => crate::proto::common::ParamRequestRead::mavlink_deser(version, payload)
                .map(MavMessage::ParamRequestRead),
            21 => crate::proto::common::ParamRequestList::mavlink_deser(version, payload)
                .map(MavMessage::ParamRequestList),
            22 => crate::proto::common::ParamValue::mavlink_deser(version, payload)
                .map(MavMessage::ParamValue),
            23 => crate::proto::common::ParamSet::mavlink_deser(version, payload)
                .map(MavMessage::ParamSet),
            24 => crate::proto::common::GpsRawInt::mavlink_deser(version, payload)
                .map(MavMessage::GpsRawInt),
            25 => crate::proto::common::GpsStatus::mavlink_deser(version, payload)
                .map(MavMessage::GpsStatus),
            26 => crate::proto::common::ScaledImu::mavlink_deser(version, payload)
                .map(MavMessage::ScaledImu),
            27 => crate::proto::common::RawImu::mavlink_deser(version, payload)
                .map(MavMessage::RawImu),
            28 => crate::proto::common::RawPressure::mavlink_deser(version, payload)
                .map(MavMessage::RawPressure),
            29 => crate::proto::common::ScaledPressure::mavlink_deser(version, payload)
                .map(MavMessage::ScaledPressure),
            30 => crate::proto::common::Attitude::mavlink_deser(version, payload)
                .map(MavMessage::Attitude),
            31 => crate::proto::common::AttitudeQuaternion::mavlink_deser(version, payload)
                .map(MavMessage::AttitudeQuaternion),
            32 => crate::proto::common::LocalPositionNed::mavlink_deser(version, payload)
                .map(MavMessage::LocalPositionNed),
            33 => crate::proto::common::GlobalPositionInt::mavlink_deser(version, payload)
                .map(MavMessage::GlobalPositionInt),
            34 => crate::proto::common::RcChannelsScaled::mavlink_deser(version, payload)
                .map(MavMessage::RcChannelsScaled),
            35 => crate::proto::common::RcChannelsRaw::mavlink_deser(version, payload)
                .map(MavMessage::RcChannelsRaw),
            36 => crate::proto::common::ServoOutputRaw::mavlink_deser(version, payload)
                .map(MavMessage::ServoOutputRaw),
            37 => crate::proto::common::MissionRequestPartialList::mavlink_deser(version, payload)
                .map(MavMessage::MissionRequestPartialList),
            38 => crate::proto::common::MissionWritePartialList::mavlink_deser(version, payload)
                .map(MavMessage::MissionWritePartialList),
            39 => crate::proto::common::MissionItem::mavlink_deser(version, payload)
                .map(MavMessage::MissionItem),
            40 => crate::proto::common::MissionRequest::mavlink_deser(version, payload)
                .map(MavMessage::MissionRequest),
            41 => crate::proto::common::MissionSetCurrent::mavlink_deser(version, payload)
                .map(MavMessage::MissionSetCurrent),
            42 => crate::proto::common::MissionCurrent::mavlink_deser(version, payload)
                .map(MavMessage::MissionCurrent),
            43 => crate::proto::common::MissionRequestList::mavlink_deser(version, payload)
                .map(MavMessage::MissionRequestList),
            44 => crate::proto::common::MissionCount::mavlink_deser(version, payload)
                .map(MavMessage::MissionCount),
            45 => crate::proto::common::MissionClearAll::mavlink_deser(version, payload)
                .map(MavMessage::MissionClearAll),
            46 => crate::proto::common::MissionItemReached::mavlink_deser(version, payload)
                .map(MavMessage::MissionItemReached),
            47 => crate::proto::common::MissionAck::mavlink_deser(version, payload)
                .map(MavMessage::MissionAck),
            48 => crate::proto::common::SetGpsGlobalOrigin::mavlink_deser(version, payload)
                .map(MavMessage::SetGpsGlobalOrigin),
            49 => crate::proto::common::GpsGlobalOrigin::mavlink_deser(version, payload)
                .map(MavMessage::GpsGlobalOrigin),
            50 => crate::proto::common::ParamMapRc::mavlink_deser(version, payload)
                .map(MavMessage::ParamMapRc),
            51 => crate::proto::common::MissionRequestInt::mavlink_deser(version, payload)
                .map(MavMessage::MissionRequestInt),
            52 => crate::proto::common::MissionChanged::mavlink_deser(version, payload)
                .map(MavMessage::MissionChanged),
            54 => crate::proto::common::SafetySetAllowedArea::mavlink_deser(version, payload)
                .map(MavMessage::SafetySetAllowedArea),
            55 => crate::proto::common::SafetyAllowedArea::mavlink_deser(version, payload)
                .map(MavMessage::SafetyAllowedArea),
            61 => crate::proto::common::AttitudeQuaternionCov::mavlink_deser(version, payload)
                .map(MavMessage::AttitudeQuaternionCov),
            62 => crate::proto::common::NavControllerOutput::mavlink_deser(version, payload)
                .map(MavMessage::NavControllerOutput),
            63 => crate::proto::common::GlobalPositionIntCov::mavlink_deser(version, payload)
                .map(MavMessage::GlobalPositionIntCov),
            64 => crate::proto::common::LocalPositionNedCov::mavlink_deser(version, payload)
                .map(MavMessage::LocalPositionNedCov),
            65 => crate::proto::common::RcChannels::mavlink_deser(version, payload)
                .map(MavMessage::RcChannels),
            66 => crate::proto::common::RequestDataStream::mavlink_deser(version, payload)
                .map(MavMessage::RequestDataStream),
            67 => crate::proto::common::DataStream::mavlink_deser(version, payload)
                .map(MavMessage::DataStream),
            69 => crate::proto::common::ManualControl::mavlink_deser(version, payload)
                .map(MavMessage::ManualControl),
            70 => crate::proto::common::RcChannelsOverride::mavlink_deser(version, payload)
                .map(MavMessage::RcChannelsOverride),
            73 => crate::proto::common::MissionItemInt::mavlink_deser(version, payload)
                .map(MavMessage::MissionItemInt),
            74 => crate::proto::common::VfrHud::mavlink_deser(version, payload)
                .map(MavMessage::VfrHud),
            75 => crate::proto::common::CommandInt::mavlink_deser(version, payload)
                .map(MavMessage::CommandInt),
            76 => crate::proto::common::CommandLong::mavlink_deser(version, payload)
                .map(MavMessage::CommandLong),
            77 => crate::proto::common::CommandAck::mavlink_deser(version, payload)
                .map(MavMessage::CommandAck),
            80 => crate::proto::common::CommandCancel::mavlink_deser(version, payload)
                .map(MavMessage::CommandCancel),
            81 => crate::proto::common::ManualSetpoint::mavlink_deser(version, payload)
                .map(MavMessage::ManualSetpoint),
            82 => crate::proto::common::SetAttitudeTarget::mavlink_deser(version, payload)
                .map(MavMessage::SetAttitudeTarget),
            83 => crate::proto::common::AttitudeTarget::mavlink_deser(version, payload)
                .map(MavMessage::AttitudeTarget),
            84 => crate::proto::common::SetPositionTargetLocalNed::mavlink_deser(version, payload)
                .map(MavMessage::SetPositionTargetLocalNed),
            85 => crate::proto::common::PositionTargetLocalNed::mavlink_deser(version, payload)
                .map(MavMessage::PositionTargetLocalNed),
            86 => crate::proto::common::SetPositionTargetGlobalInt::mavlink_deser(version, payload)
                .map(MavMessage::SetPositionTargetGlobalInt),
            87 => crate::proto::common::PositionTargetGlobalInt::mavlink_deser(version, payload)
                .map(MavMessage::PositionTargetGlobalInt),
            89 => crate::proto::common::LocalPositionNedSystemGlobalOffset::mavlink_deser(
                version, payload,
            )
            .map(MavMessage::LocalPositionNedSystemGlobalOffset),
            90 => crate::proto::common::HilState::mavlink_deser(version, payload)
                .map(MavMessage::HilState),
            91 => crate::proto::common::HilControls::mavlink_deser(version, payload)
                .map(MavMessage::HilControls),
            92 => crate::proto::common::HilRcInputsRaw::mavlink_deser(version, payload)
                .map(MavMessage::HilRcInputsRaw),
            93 => crate::proto::common::HilActuatorControls::mavlink_deser(version, payload)
                .map(MavMessage::HilActuatorControls),
            100 => crate::proto::common::OpticalFlow::mavlink_deser(version, payload)
                .map(MavMessage::OpticalFlow),
            101 => {
                crate::proto::common::GlobalVisionPositionEstimate::mavlink_deser(version, payload)
                    .map(MavMessage::GlobalVisionPositionEstimate)
            }
            102 => crate::proto::common::VisionPositionEstimate::mavlink_deser(version, payload)
                .map(MavMessage::VisionPositionEstimate),
            103 => crate::proto::common::VisionSpeedEstimate::mavlink_deser(version, payload)
                .map(MavMessage::VisionSpeedEstimate),
            104 => crate::proto::common::ViconPositionEstimate::mavlink_deser(version, payload)
                .map(MavMessage::ViconPositionEstimate),
            105 => crate::proto::common::HighresImu::mavlink_deser(version, payload)
                .map(MavMessage::HighresImu),
            106 => crate::proto::common::OpticalFlowRad::mavlink_deser(version, payload)
                .map(MavMessage::OpticalFlowRad),
            107 => crate::proto::common::HilSensor::mavlink_deser(version, payload)
                .map(MavMessage::HilSensor),
            108 => crate::proto::common::SimState::mavlink_deser(version, payload)
                .map(MavMessage::SimState),
            109 => crate::proto::common::RadioStatus::mavlink_deser(version, payload)
                .map(MavMessage::RadioStatus),
            110 => crate::proto::common::FileTransferProtocol::mavlink_deser(version, payload)
                .map(MavMessage::FileTransferProtocol),
            111 => crate::proto::common::Timesync::mavlink_deser(version, payload)
                .map(MavMessage::Timesync),
            112 => crate::proto::common::CameraTrigger::mavlink_deser(version, payload)
                .map(MavMessage::CameraTrigger),
            113 => crate::proto::common::HilGps::mavlink_deser(version, payload)
                .map(MavMessage::HilGps),
            114 => crate::proto::common::HilOpticalFlow::mavlink_deser(version, payload)
                .map(MavMessage::HilOpticalFlow),
            115 => crate::proto::common::HilStateQuaternion::mavlink_deser(version, payload)
                .map(MavMessage::HilStateQuaternion),
            116 => crate::proto::common::ScaledImu2::mavlink_deser(version, payload)
                .map(MavMessage::ScaledImu2),
            117 => crate::proto::common::LogRequestList::mavlink_deser(version, payload)
                .map(MavMessage::LogRequestList),
            118 => crate::proto::common::LogEntry::mavlink_deser(version, payload)
                .map(MavMessage::LogEntry),
            119 => crate::proto::common::LogRequestData::mavlink_deser(version, payload)
                .map(MavMessage::LogRequestData),
            120 => crate::proto::common::LogData::mavlink_deser(version, payload)
                .map(MavMessage::LogData),
            121 => crate::proto::common::LogErase::mavlink_deser(version, payload)
                .map(MavMessage::LogErase),
            122 => crate::proto::common::LogRequestEnd::mavlink_deser(version, payload)
                .map(MavMessage::LogRequestEnd),
            123 => crate::proto::common::GpsInjectData::mavlink_deser(version, payload)
                .map(MavMessage::GpsInjectData),
            124 => crate::proto::common::Gps2Raw::mavlink_deser(version, payload)
                .map(MavMessage::Gps2Raw),
            125 => crate::proto::common::PowerStatus::mavlink_deser(version, payload)
                .map(MavMessage::PowerStatus),
            126 => crate::proto::common::SerialControl::mavlink_deser(version, payload)
                .map(MavMessage::SerialControl),
            127 => crate::proto::common::GpsRtk::mavlink_deser(version, payload)
                .map(MavMessage::GpsRtk),
            128 => crate::proto::common::Gps2Rtk::mavlink_deser(version, payload)
                .map(MavMessage::Gps2Rtk),
            129 => crate::proto::common::ScaledImu3::mavlink_deser(version, payload)
                .map(MavMessage::ScaledImu3),
            130 => crate::proto::common::DataTransmissionHandshake::mavlink_deser(version, payload)
                .map(MavMessage::DataTransmissionHandshake),
            131 => crate::proto::common::EncapsulatedData::mavlink_deser(version, payload)
                .map(MavMessage::EncapsulatedData),
            132 => crate::proto::common::DistanceSensor::mavlink_deser(version, payload)
                .map(MavMessage::DistanceSensor),
            133 => crate::proto::common::TerrainRequest::mavlink_deser(version, payload)
                .map(MavMessage::TerrainRequest),
            134 => crate::proto::common::TerrainData::mavlink_deser(version, payload)
                .map(MavMessage::TerrainData),
            135 => crate::proto::common::TerrainCheck::mavlink_deser(version, payload)
                .map(MavMessage::TerrainCheck),
            136 => crate::proto::common::TerrainReport::mavlink_deser(version, payload)
                .map(MavMessage::TerrainReport),
            137 => crate::proto::common::ScaledPressure2::mavlink_deser(version, payload)
                .map(MavMessage::ScaledPressure2),
            138 => crate::proto::common::AttPosMocap::mavlink_deser(version, payload)
                .map(MavMessage::AttPosMocap),
            139 => crate::proto::common::SetActuatorControlTarget::mavlink_deser(version, payload)
                .map(MavMessage::SetActuatorControlTarget),
            140 => crate::proto::common::ActuatorControlTarget::mavlink_deser(version, payload)
                .map(MavMessage::ActuatorControlTarget),
            141 => crate::proto::common::Altitude::mavlink_deser(version, payload)
                .map(MavMessage::Altitude),
            142 => crate::proto::common::ResourceRequest::mavlink_deser(version, payload)
                .map(MavMessage::ResourceRequest),
            143 => crate::proto::common::ScaledPressure3::mavlink_deser(version, payload)
                .map(MavMessage::ScaledPressure3),
            144 => crate::proto::common::FollowTarget::mavlink_deser(version, payload)
                .map(MavMessage::FollowTarget),
            146 => crate::proto::common::ControlSystemState::mavlink_deser(version, payload)
                .map(MavMessage::ControlSystemState),
            147 => crate::proto::common::BatteryStatus::mavlink_deser(version, payload)
                .map(MavMessage::BatteryStatus),
            148 => crate::proto::common::AutopilotVersion::mavlink_deser(version, payload)
                .map(MavMessage::AutopilotVersion),
            149 => crate::proto::common::LandingTarget::mavlink_deser(version, payload)
                .map(MavMessage::LandingTarget),
            162 => crate::proto::common::FenceStatus::mavlink_deser(version, payload)
                .map(MavMessage::FenceStatus),
            230 => crate::proto::common::EstimatorStatus::mavlink_deser(version, payload)
                .map(MavMessage::EstimatorStatus),
            231 => crate::proto::common::WindCov::mavlink_deser(version, payload)
                .map(MavMessage::WindCov),
            232 => crate::proto::common::GpsInput::mavlink_deser(version, payload)
                .map(MavMessage::GpsInput),
            233 => crate::proto::common::GpsRtcmData::mavlink_deser(version, payload)
                .map(MavMessage::GpsRtcmData),
            234 => crate::proto::common::HighLatency::mavlink_deser(version, payload)
                .map(MavMessage::HighLatency),
            235 => crate::proto::common::HighLatency2::mavlink_deser(version, payload)
                .map(MavMessage::HighLatency2),
            241 => crate::proto::common::Vibration::mavlink_deser(version, payload)
                .map(MavMessage::Vibration),
            242 => crate::proto::common::HomePosition::mavlink_deser(version, payload)
                .map(MavMessage::HomePosition),
            243 => crate::proto::common::SetHomePosition::mavlink_deser(version, payload)
                .map(MavMessage::SetHomePosition),
            244 => crate::proto::common::MessageInterval::mavlink_deser(version, payload)
                .map(MavMessage::MessageInterval),
            245 => crate::proto::common::ExtendedSysState::mavlink_deser(version, payload)
                .map(MavMessage::ExtendedSysState),
            246 => crate::proto::common::AdsbVehicle::mavlink_deser(version, payload)
                .map(MavMessage::AdsbVehicle),
            247 => crate::proto::common::Collision::mavlink_deser(version, payload)
                .map(MavMessage::Collision),
            248 => crate::proto::common::V2Extension::mavlink_deser(version, payload)
                .map(MavMessage::V2Extension),
            249 => crate::proto::common::MemoryVect::mavlink_deser(version, payload)
                .map(MavMessage::MemoryVect),
            250 => crate::proto::common::DebugVect::mavlink_deser(version, payload)
                .map(MavMessage::DebugVect),
            251 => crate::proto::common::NamedValueFloat::mavlink_deser(version, payload)
                .map(MavMessage::NamedValueFloat),
            252 => crate::proto::common::NamedValueInt::mavlink_deser(version, payload)
                .map(MavMessage::NamedValueInt),
            253 => crate::proto::common::Statustext::mavlink_deser(version, payload)
                .map(MavMessage::Statustext),
            254 => {
                crate::proto::common::Debug::mavlink_deser(version, payload).map(MavMessage::Debug)
            }
            256 => crate::proto::common::SetupSigning::mavlink_deser(version, payload)
                .map(MavMessage::SetupSigning),
            257 => crate::proto::common::ButtonChange::mavlink_deser(version, payload)
                .map(MavMessage::ButtonChange),
            258 => crate::proto::common::PlayTune::mavlink_deser(version, payload)
                .map(MavMessage::PlayTune),
            259 => crate::proto::common::CameraInformation::mavlink_deser(version, payload)
                .map(MavMessage::CameraInformation),
            260 => crate::proto::common::CameraSettings::mavlink_deser(version, payload)
                .map(MavMessage::CameraSettings),
            261 => crate::proto::common::StorageInformation::mavlink_deser(version, payload)
                .map(MavMessage::StorageInformation),
            262 => crate::proto::common::CameraCaptureStatus::mavlink_deser(version, payload)
                .map(MavMessage::CameraCaptureStatus),
            263 => crate::proto::common::CameraImageCaptured::mavlink_deser(version, payload)
                .map(MavMessage::CameraImageCaptured),
            264 => crate::proto::common::FlightInformation::mavlink_deser(version, payload)
                .map(MavMessage::FlightInformation),
            265 => crate::proto::common::MountOrientation::mavlink_deser(version, payload)
                .map(MavMessage::MountOrientation),
            266 => crate::proto::common::LoggingData::mavlink_deser(version, payload)
                .map(MavMessage::LoggingData),
            267 => crate::proto::common::LoggingDataAcked::mavlink_deser(version, payload)
                .map(MavMessage::LoggingDataAcked),
            268 => crate::proto::common::LoggingAck::mavlink_deser(version, payload)
                .map(MavMessage::LoggingAck),
            269 => crate::proto::common::VideoStreamInformation::mavlink_deser(version, payload)
                .map(MavMessage::VideoStreamInformation),
            270 => crate::proto::common::VideoStreamStatus::mavlink_deser(version, payload)
                .map(MavMessage::VideoStreamStatus),
            280 => crate::proto::common::GimbalManagerInformation::mavlink_deser(version, payload)
                .map(MavMessage::GimbalManagerInformation),
            281 => crate::proto::common::GimbalManagerStatus::mavlink_deser(version, payload)
                .map(MavMessage::GimbalManagerStatus),
            282 => crate::proto::common::GimbalManagerSetAttitude::mavlink_deser(version, payload)
                .map(MavMessage::GimbalManagerSetAttitude),
            283 => crate::proto::common::GimbalDeviceInformation::mavlink_deser(version, payload)
                .map(MavMessage::GimbalDeviceInformation),
            284 => crate::proto::common::GimbalDeviceSetAttitude::mavlink_deser(version, payload)
                .map(MavMessage::GimbalDeviceSetAttitude),
            285 => {
                crate::proto::common::GimbalDeviceAttitudeStatus::mavlink_deser(version, payload)
                    .map(MavMessage::GimbalDeviceAttitudeStatus)
            }
            286 => {
                crate::proto::common::AutopilotStateForGimbalDevice::mavlink_deser(version, payload)
                    .map(MavMessage::AutopilotStateForGimbalDevice)
            }
            287 => crate::proto::common::GimbalManagerSetTiltpan::mavlink_deser(version, payload)
                .map(MavMessage::GimbalManagerSetTiltpan),
            299 => crate::proto::common::WifiConfigAp::mavlink_deser(version, payload)
                .map(MavMessage::WifiConfigAp),
            300 => crate::proto::common::ProtocolVersion::mavlink_deser(version, payload)
                .map(MavMessage::ProtocolVersion),
            301 => crate::proto::common::AisVessel::mavlink_deser(version, payload)
                .map(MavMessage::AisVessel),
            310 => crate::proto::common::UavcanNodeStatus::mavlink_deser(version, payload)
                .map(MavMessage::UavcanNodeStatus),
            311 => crate::proto::common::UavcanNodeInfo::mavlink_deser(version, payload)
                .map(MavMessage::UavcanNodeInfo),
            320 => crate::proto::common::ParamExtRequestRead::mavlink_deser(version, payload)
                .map(MavMessage::ParamExtRequestRead),
            321 => crate::proto::common::ParamExtRequestList::mavlink_deser(version, payload)
                .map(MavMessage::ParamExtRequestList),
            322 => crate::proto::common::ParamExtValue::mavlink_deser(version, payload)
                .map(MavMessage::ParamExtValue),
            323 => crate::proto::common::ParamExtSet::mavlink_deser(version, payload)
                .map(MavMessage::ParamExtSet),
            324 => crate::proto::common::ParamExtAck::mavlink_deser(version, payload)
                .map(MavMessage::ParamExtAck),
            330 => crate::proto::common::ObstacleDistance::mavlink_deser(version, payload)
                .map(MavMessage::ObstacleDistance),
            331 => crate::proto::common::Odometry::mavlink_deser(version, payload)
                .map(MavMessage::Odometry),
            332 => crate::proto::common::TrajectoryRepresentationWaypoints::mavlink_deser(
                version, payload,
            )
            .map(MavMessage::TrajectoryRepresentationWaypoints),
            333 => crate::proto::common::TrajectoryRepresentationBezier::mavlink_deser(
                version, payload,
            )
            .map(MavMessage::TrajectoryRepresentationBezier),
            334 => crate::proto::common::CellularStatus::mavlink_deser(version, payload)
                .map(MavMessage::CellularStatus),
            335 => crate::proto::common::IsbdLinkStatus::mavlink_deser(version, payload)
                .map(MavMessage::IsbdLinkStatus),
            336 => crate::proto::common::CellularConfig::mavlink_deser(version, payload)
                .map(MavMessage::CellularConfig),
            339 => crate::proto::common::RawRpm::mavlink_deser(version, payload)
                .map(MavMessage::RawRpm),
            340 => crate::proto::common::UtmGlobalPosition::mavlink_deser(version, payload)
                .map(MavMessage::UtmGlobalPosition),
            350 => crate::proto::common::DebugFloatArray::mavlink_deser(version, payload)
                .map(MavMessage::DebugFloatArray),
            360 => crate::proto::common::OrbitExecutionStatus::mavlink_deser(version, payload)
                .map(MavMessage::OrbitExecutionStatus),
            370 => crate::proto::common::SmartBatteryInfo::mavlink_deser(version, payload)
                .map(MavMessage::SmartBatteryInfo),
            371 => crate::proto::common::SmartBatteryStatus::mavlink_deser(version, payload)
                .map(MavMessage::SmartBatteryStatus),
            373 => crate::proto::common::GeneratorStatus::mavlink_deser(version, payload)
                .map(MavMessage::GeneratorStatus),
            375 => crate::proto::common::ActuatorOutputStatus::mavlink_deser(version, payload)
                .map(MavMessage::ActuatorOutputStatus),
            380 => crate::proto::common::TimeEstimateToTarget::mavlink_deser(version, payload)
                .map(MavMessage::TimeEstimateToTarget),
            385 => crate::proto::common::Tunnel::mavlink_deser(version, payload)
                .map(MavMessage::Tunnel),
            390 => crate::proto::common::OnboardComputerStatus::mavlink_deser(version, payload)
                .map(MavMessage::OnboardComputerStatus),
            395 => crate::proto::common::ComponentInformation::mavlink_deser(version, payload)
                .map(MavMessage::ComponentInformation),
            400 => crate::proto::common::PlayTuneV2::mavlink_deser(version, payload)
                .map(MavMessage::PlayTuneV2),
            401 => crate::proto::common::SupportedTunes::mavlink_deser(version, payload)
                .map(MavMessage::SupportedTunes),
            9000 => crate::proto::common::WheelDistance::mavlink_deser(version, payload)
                .map(MavMessage::WheelDistance),
            12900 => crate::proto::common::OpenDroneIdBasicId::mavlink_deser(version, payload)
                .map(MavMessage::OpenDroneIdBasicId),
            12901 => crate::proto::common::OpenDroneIdLocation::mavlink_deser(version, payload)
                .map(MavMessage::OpenDroneIdLocation),
            12902 => {
                crate::proto::common::OpenDroneIdAuthentication::mavlink_deser(version, payload)
                    .map(MavMessage::OpenDroneIdAuthentication)
            }
            12903 => crate::proto::common::OpenDroneIdSelfId::mavlink_deser(version, payload)
                .map(MavMessage::OpenDroneIdSelfId),
            12904 => crate::proto::common::OpenDroneIdSystem::mavlink_deser(version, payload)
                .map(MavMessage::OpenDroneIdSystem),
            12905 => crate::proto::common::OpenDroneIdOperatorId::mavlink_deser(version, payload)
                .map(MavMessage::OpenDroneIdOperatorId),
            12915 => crate::proto::common::OpenDroneIdMessagePack::mavlink_deser(version, payload)
                .map(MavMessage::OpenDroneIdMessagePack),
            _ => Err(ParserError::UnknownMessage { id }),
        }
    }
    fn message_name(&self) -> &'static str {
        match self {
            MavMessage::Heartbeat(..) => "Heartbeat",
            MavMessage::SysStatus(..) => "SysStatus",
            MavMessage::SystemTime(..) => "SystemTime",
            MavMessage::Ping(..) => "Ping",
            MavMessage::ChangeOperatorControl(..) => "ChangeOperatorControl",
            MavMessage::ChangeOperatorControlAck(..) => "ChangeOperatorControlAck",
            MavMessage::AuthKey(..) => "AuthKey",
            MavMessage::LinkNodeStatus(..) => "LinkNodeStatus",
            MavMessage::SetMode(..) => "SetMode",
            MavMessage::ParamRequestRead(..) => "ParamRequestRead",
            MavMessage::ParamRequestList(..) => "ParamRequestList",
            MavMessage::ParamValue(..) => "ParamValue",
            MavMessage::ParamSet(..) => "ParamSet",
            MavMessage::GpsRawInt(..) => "GpsRawInt",
            MavMessage::GpsStatus(..) => "GpsStatus",
            MavMessage::ScaledImu(..) => "ScaledImu",
            MavMessage::RawImu(..) => "RawImu",
            MavMessage::RawPressure(..) => "RawPressure",
            MavMessage::ScaledPressure(..) => "ScaledPressure",
            MavMessage::Attitude(..) => "Attitude",
            MavMessage::AttitudeQuaternion(..) => "AttitudeQuaternion",
            MavMessage::LocalPositionNed(..) => "LocalPositionNed",
            MavMessage::GlobalPositionInt(..) => "GlobalPositionInt",
            MavMessage::RcChannelsScaled(..) => "RcChannelsScaled",
            MavMessage::RcChannelsRaw(..) => "RcChannelsRaw",
            MavMessage::ServoOutputRaw(..) => "ServoOutputRaw",
            MavMessage::MissionRequestPartialList(..) => "MissionRequestPartialList",
            MavMessage::MissionWritePartialList(..) => "MissionWritePartialList",
            MavMessage::MissionItem(..) => "MissionItem",
            MavMessage::MissionRequest(..) => "MissionRequest",
            MavMessage::MissionSetCurrent(..) => "MissionSetCurrent",
            MavMessage::MissionCurrent(..) => "MissionCurrent",
            MavMessage::MissionRequestList(..) => "MissionRequestList",
            MavMessage::MissionCount(..) => "MissionCount",
            MavMessage::MissionClearAll(..) => "MissionClearAll",
            MavMessage::MissionItemReached(..) => "MissionItemReached",
            MavMessage::MissionAck(..) => "MissionAck",
            MavMessage::SetGpsGlobalOrigin(..) => "SetGpsGlobalOrigin",
            MavMessage::GpsGlobalOrigin(..) => "GpsGlobalOrigin",
            MavMessage::ParamMapRc(..) => "ParamMapRc",
            MavMessage::MissionRequestInt(..) => "MissionRequestInt",
            MavMessage::MissionChanged(..) => "MissionChanged",
            MavMessage::SafetySetAllowedArea(..) => "SafetySetAllowedArea",
            MavMessage::SafetyAllowedArea(..) => "SafetyAllowedArea",
            MavMessage::AttitudeQuaternionCov(..) => "AttitudeQuaternionCov",
            MavMessage::NavControllerOutput(..) => "NavControllerOutput",
            MavMessage::GlobalPositionIntCov(..) => "GlobalPositionIntCov",
            MavMessage::LocalPositionNedCov(..) => "LocalPositionNedCov",
            MavMessage::RcChannels(..) => "RcChannels",
            MavMessage::RequestDataStream(..) => "RequestDataStream",
            MavMessage::DataStream(..) => "DataStream",
            MavMessage::ManualControl(..) => "ManualControl",
            MavMessage::RcChannelsOverride(..) => "RcChannelsOverride",
            MavMessage::MissionItemInt(..) => "MissionItemInt",
            MavMessage::VfrHud(..) => "VfrHud",
            MavMessage::CommandInt(..) => "CommandInt",
            MavMessage::CommandLong(..) => "CommandLong",
            MavMessage::CommandAck(..) => "CommandAck",
            MavMessage::CommandCancel(..) => "CommandCancel",
            MavMessage::ManualSetpoint(..) => "ManualSetpoint",
            MavMessage::SetAttitudeTarget(..) => "SetAttitudeTarget",
            MavMessage::AttitudeTarget(..) => "AttitudeTarget",
            MavMessage::SetPositionTargetLocalNed(..) => "SetPositionTargetLocalNed",
            MavMessage::PositionTargetLocalNed(..) => "PositionTargetLocalNed",
            MavMessage::SetPositionTargetGlobalInt(..) => "SetPositionTargetGlobalInt",
            MavMessage::PositionTargetGlobalInt(..) => "PositionTargetGlobalInt",
            MavMessage::LocalPositionNedSystemGlobalOffset(..) => {
                "LocalPositionNedSystemGlobalOffset"
            }
            MavMessage::HilState(..) => "HilState",
            MavMessage::HilControls(..) => "HilControls",
            MavMessage::HilRcInputsRaw(..) => "HilRcInputsRaw",
            MavMessage::HilActuatorControls(..) => "HilActuatorControls",
            MavMessage::OpticalFlow(..) => "OpticalFlow",
            MavMessage::GlobalVisionPositionEstimate(..) => "GlobalVisionPositionEstimate",
            MavMessage::VisionPositionEstimate(..) => "VisionPositionEstimate",
            MavMessage::VisionSpeedEstimate(..) => "VisionSpeedEstimate",
            MavMessage::ViconPositionEstimate(..) => "ViconPositionEstimate",
            MavMessage::HighresImu(..) => "HighresImu",
            MavMessage::OpticalFlowRad(..) => "OpticalFlowRad",
            MavMessage::HilSensor(..) => "HilSensor",
            MavMessage::SimState(..) => "SimState",
            MavMessage::RadioStatus(..) => "RadioStatus",
            MavMessage::FileTransferProtocol(..) => "FileTransferProtocol",
            MavMessage::Timesync(..) => "Timesync",
            MavMessage::CameraTrigger(..) => "CameraTrigger",
            MavMessage::HilGps(..) => "HilGps",
            MavMessage::HilOpticalFlow(..) => "HilOpticalFlow",
            MavMessage::HilStateQuaternion(..) => "HilStateQuaternion",
            MavMessage::ScaledImu2(..) => "ScaledImu2",
            MavMessage::LogRequestList(..) => "LogRequestList",
            MavMessage::LogEntry(..) => "LogEntry",
            MavMessage::LogRequestData(..) => "LogRequestData",
            MavMessage::LogData(..) => "LogData",
            MavMessage::LogErase(..) => "LogErase",
            MavMessage::LogRequestEnd(..) => "LogRequestEnd",
            MavMessage::GpsInjectData(..) => "GpsInjectData",
            MavMessage::Gps2Raw(..) => "Gps2Raw",
            MavMessage::PowerStatus(..) => "PowerStatus",
            MavMessage::SerialControl(..) => "SerialControl",
            MavMessage::GpsRtk(..) => "GpsRtk",
            MavMessage::Gps2Rtk(..) => "Gps2Rtk",
            MavMessage::ScaledImu3(..) => "ScaledImu3",
            MavMessage::DataTransmissionHandshake(..) => "DataTransmissionHandshake",
            MavMessage::EncapsulatedData(..) => "EncapsulatedData",
            MavMessage::DistanceSensor(..) => "DistanceSensor",
            MavMessage::TerrainRequest(..) => "TerrainRequest",
            MavMessage::TerrainData(..) => "TerrainData",
            MavMessage::TerrainCheck(..) => "TerrainCheck",
            MavMessage::TerrainReport(..) => "TerrainReport",
            MavMessage::ScaledPressure2(..) => "ScaledPressure2",
            MavMessage::AttPosMocap(..) => "AttPosMocap",
            MavMessage::SetActuatorControlTarget(..) => "SetActuatorControlTarget",
            MavMessage::ActuatorControlTarget(..) => "ActuatorControlTarget",
            MavMessage::Altitude(..) => "Altitude",
            MavMessage::ResourceRequest(..) => "ResourceRequest",
            MavMessage::ScaledPressure3(..) => "ScaledPressure3",
            MavMessage::FollowTarget(..) => "FollowTarget",
            MavMessage::ControlSystemState(..) => "ControlSystemState",
            MavMessage::BatteryStatus(..) => "BatteryStatus",
            MavMessage::AutopilotVersion(..) => "AutopilotVersion",
            MavMessage::LandingTarget(..) => "LandingTarget",
            MavMessage::FenceStatus(..) => "FenceStatus",
            MavMessage::EstimatorStatus(..) => "EstimatorStatus",
            MavMessage::WindCov(..) => "WindCov",
            MavMessage::GpsInput(..) => "GpsInput",
            MavMessage::GpsRtcmData(..) => "GpsRtcmData",
            MavMessage::HighLatency(..) => "HighLatency",
            MavMessage::HighLatency2(..) => "HighLatency2",
            MavMessage::Vibration(..) => "Vibration",
            MavMessage::HomePosition(..) => "HomePosition",
            MavMessage::SetHomePosition(..) => "SetHomePosition",
            MavMessage::MessageInterval(..) => "MessageInterval",
            MavMessage::ExtendedSysState(..) => "ExtendedSysState",
            MavMessage::AdsbVehicle(..) => "AdsbVehicle",
            MavMessage::Collision(..) => "Collision",
            MavMessage::V2Extension(..) => "V2Extension",
            MavMessage::MemoryVect(..) => "MemoryVect",
            MavMessage::DebugVect(..) => "DebugVect",
            MavMessage::NamedValueFloat(..) => "NamedValueFloat",
            MavMessage::NamedValueInt(..) => "NamedValueInt",
            MavMessage::Statustext(..) => "Statustext",
            MavMessage::Debug(..) => "Debug",
            MavMessage::SetupSigning(..) => "SetupSigning",
            MavMessage::ButtonChange(..) => "ButtonChange",
            MavMessage::PlayTune(..) => "PlayTune",
            MavMessage::CameraInformation(..) => "CameraInformation",
            MavMessage::CameraSettings(..) => "CameraSettings",
            MavMessage::StorageInformation(..) => "StorageInformation",
            MavMessage::CameraCaptureStatus(..) => "CameraCaptureStatus",
            MavMessage::CameraImageCaptured(..) => "CameraImageCaptured",
            MavMessage::FlightInformation(..) => "FlightInformation",
            MavMessage::MountOrientation(..) => "MountOrientation",
            MavMessage::LoggingData(..) => "LoggingData",
            MavMessage::LoggingDataAcked(..) => "LoggingDataAcked",
            MavMessage::LoggingAck(..) => "LoggingAck",
            MavMessage::VideoStreamInformation(..) => "VideoStreamInformation",
            MavMessage::VideoStreamStatus(..) => "VideoStreamStatus",
            MavMessage::GimbalManagerInformation(..) => "GimbalManagerInformation",
            MavMessage::GimbalManagerStatus(..) => "GimbalManagerStatus",
            MavMessage::GimbalManagerSetAttitude(..) => "GimbalManagerSetAttitude",
            MavMessage::GimbalDeviceInformation(..) => "GimbalDeviceInformation",
            MavMessage::GimbalDeviceSetAttitude(..) => "GimbalDeviceSetAttitude",
            MavMessage::GimbalDeviceAttitudeStatus(..) => "GimbalDeviceAttitudeStatus",
            MavMessage::AutopilotStateForGimbalDevice(..) => "AutopilotStateForGimbalDevice",
            MavMessage::GimbalManagerSetTiltpan(..) => "GimbalManagerSetTiltpan",
            MavMessage::WifiConfigAp(..) => "WifiConfigAp",
            MavMessage::ProtocolVersion(..) => "ProtocolVersion",
            MavMessage::AisVessel(..) => "AisVessel",
            MavMessage::UavcanNodeStatus(..) => "UavcanNodeStatus",
            MavMessage::UavcanNodeInfo(..) => "UavcanNodeInfo",
            MavMessage::ParamExtRequestRead(..) => "ParamExtRequestRead",
            MavMessage::ParamExtRequestList(..) => "ParamExtRequestList",
            MavMessage::ParamExtValue(..) => "ParamExtValue",
            MavMessage::ParamExtSet(..) => "ParamExtSet",
            MavMessage::ParamExtAck(..) => "ParamExtAck",
            MavMessage::ObstacleDistance(..) => "ObstacleDistance",
            MavMessage::Odometry(..) => "Odometry",
            MavMessage::TrajectoryRepresentationWaypoints(..) => {
                "TrajectoryRepresentationWaypoints"
            }
            MavMessage::TrajectoryRepresentationBezier(..) => "TrajectoryRepresentationBezier",
            MavMessage::CellularStatus(..) => "CellularStatus",
            MavMessage::IsbdLinkStatus(..) => "IsbdLinkStatus",
            MavMessage::CellularConfig(..) => "CellularConfig",
            MavMessage::RawRpm(..) => "RawRpm",
            MavMessage::UtmGlobalPosition(..) => "UtmGlobalPosition",
            MavMessage::DebugFloatArray(..) => "DebugFloatArray",
            MavMessage::OrbitExecutionStatus(..) => "OrbitExecutionStatus",
            MavMessage::SmartBatteryInfo(..) => "SmartBatteryInfo",
            MavMessage::SmartBatteryStatus(..) => "SmartBatteryStatus",
            MavMessage::GeneratorStatus(..) => "GeneratorStatus",
            MavMessage::ActuatorOutputStatus(..) => "ActuatorOutputStatus",
            MavMessage::TimeEstimateToTarget(..) => "TimeEstimateToTarget",
            MavMessage::Tunnel(..) => "Tunnel",
            MavMessage::OnboardComputerStatus(..) => "OnboardComputerStatus",
            MavMessage::ComponentInformation(..) => "ComponentInformation",
            MavMessage::PlayTuneV2(..) => "PlayTuneV2",
            MavMessage::SupportedTunes(..) => "SupportedTunes",
            MavMessage::WheelDistance(..) => "WheelDistance",
            MavMessage::OpenDroneIdBasicId(..) => "OpenDroneIdBasicId",
            MavMessage::OpenDroneIdLocation(..) => "OpenDroneIdLocation",
            MavMessage::OpenDroneIdAuthentication(..) => "OpenDroneIdAuthentication",
            MavMessage::OpenDroneIdSelfId(..) => "OpenDroneIdSelfId",
            MavMessage::OpenDroneIdSystem(..) => "OpenDroneIdSystem",
            MavMessage::OpenDroneIdOperatorId(..) => "OpenDroneIdOperatorId",
            MavMessage::OpenDroneIdMessagePack(..) => "OpenDroneIdMessagePack",
        }
    }
    fn message_id(&self) -> u32 {
        match self {
            MavMessage::Heartbeat(..) => 0,
            MavMessage::SysStatus(..) => 1,
            MavMessage::SystemTime(..) => 2,
            MavMessage::Ping(..) => 4,
            MavMessage::ChangeOperatorControl(..) => 5,
            MavMessage::ChangeOperatorControlAck(..) => 6,
            MavMessage::AuthKey(..) => 7,
            MavMessage::LinkNodeStatus(..) => 8,
            MavMessage::SetMode(..) => 11,
            MavMessage::ParamRequestRead(..) => 20,
            MavMessage::ParamRequestList(..) => 21,
            MavMessage::ParamValue(..) => 22,
            MavMessage::ParamSet(..) => 23,
            MavMessage::GpsRawInt(..) => 24,
            MavMessage::GpsStatus(..) => 25,
            MavMessage::ScaledImu(..) => 26,
            MavMessage::RawImu(..) => 27,
            MavMessage::RawPressure(..) => 28,
            MavMessage::ScaledPressure(..) => 29,
            MavMessage::Attitude(..) => 30,
            MavMessage::AttitudeQuaternion(..) => 31,
            MavMessage::LocalPositionNed(..) => 32,
            MavMessage::GlobalPositionInt(..) => 33,
            MavMessage::RcChannelsScaled(..) => 34,
            MavMessage::RcChannelsRaw(..) => 35,
            MavMessage::ServoOutputRaw(..) => 36,
            MavMessage::MissionRequestPartialList(..) => 37,
            MavMessage::MissionWritePartialList(..) => 38,
            MavMessage::MissionItem(..) => 39,
            MavMessage::MissionRequest(..) => 40,
            MavMessage::MissionSetCurrent(..) => 41,
            MavMessage::MissionCurrent(..) => 42,
            MavMessage::MissionRequestList(..) => 43,
            MavMessage::MissionCount(..) => 44,
            MavMessage::MissionClearAll(..) => 45,
            MavMessage::MissionItemReached(..) => 46,
            MavMessage::MissionAck(..) => 47,
            MavMessage::SetGpsGlobalOrigin(..) => 48,
            MavMessage::GpsGlobalOrigin(..) => 49,
            MavMessage::ParamMapRc(..) => 50,
            MavMessage::MissionRequestInt(..) => 51,
            MavMessage::MissionChanged(..) => 52,
            MavMessage::SafetySetAllowedArea(..) => 54,
            MavMessage::SafetyAllowedArea(..) => 55,
            MavMessage::AttitudeQuaternionCov(..) => 61,
            MavMessage::NavControllerOutput(..) => 62,
            MavMessage::GlobalPositionIntCov(..) => 63,
            MavMessage::LocalPositionNedCov(..) => 64,
            MavMessage::RcChannels(..) => 65,
            MavMessage::RequestDataStream(..) => 66,
            MavMessage::DataStream(..) => 67,
            MavMessage::ManualControl(..) => 69,
            MavMessage::RcChannelsOverride(..) => 70,
            MavMessage::MissionItemInt(..) => 73,
            MavMessage::VfrHud(..) => 74,
            MavMessage::CommandInt(..) => 75,
            MavMessage::CommandLong(..) => 76,
            MavMessage::CommandAck(..) => 77,
            MavMessage::CommandCancel(..) => 80,
            MavMessage::ManualSetpoint(..) => 81,
            MavMessage::SetAttitudeTarget(..) => 82,
            MavMessage::AttitudeTarget(..) => 83,
            MavMessage::SetPositionTargetLocalNed(..) => 84,
            MavMessage::PositionTargetLocalNed(..) => 85,
            MavMessage::SetPositionTargetGlobalInt(..) => 86,
            MavMessage::PositionTargetGlobalInt(..) => 87,
            MavMessage::LocalPositionNedSystemGlobalOffset(..) => 89,
            MavMessage::HilState(..) => 90,
            MavMessage::HilControls(..) => 91,
            MavMessage::HilRcInputsRaw(..) => 92,
            MavMessage::HilActuatorControls(..) => 93,
            MavMessage::OpticalFlow(..) => 100,
            MavMessage::GlobalVisionPositionEstimate(..) => 101,
            MavMessage::VisionPositionEstimate(..) => 102,
            MavMessage::VisionSpeedEstimate(..) => 103,
            MavMessage::ViconPositionEstimate(..) => 104,
            MavMessage::HighresImu(..) => 105,
            MavMessage::OpticalFlowRad(..) => 106,
            MavMessage::HilSensor(..) => 107,
            MavMessage::SimState(..) => 108,
            MavMessage::RadioStatus(..) => 109,
            MavMessage::FileTransferProtocol(..) => 110,
            MavMessage::Timesync(..) => 111,
            MavMessage::CameraTrigger(..) => 112,
            MavMessage::HilGps(..) => 113,
            MavMessage::HilOpticalFlow(..) => 114,
            MavMessage::HilStateQuaternion(..) => 115,
            MavMessage::ScaledImu2(..) => 116,
            MavMessage::LogRequestList(..) => 117,
            MavMessage::LogEntry(..) => 118,
            MavMessage::LogRequestData(..) => 119,
            MavMessage::LogData(..) => 120,
            MavMessage::LogErase(..) => 121,
            MavMessage::LogRequestEnd(..) => 122,
            MavMessage::GpsInjectData(..) => 123,
            MavMessage::Gps2Raw(..) => 124,
            MavMessage::PowerStatus(..) => 125,
            MavMessage::SerialControl(..) => 126,
            MavMessage::GpsRtk(..) => 127,
            MavMessage::Gps2Rtk(..) => 128,
            MavMessage::ScaledImu3(..) => 129,
            MavMessage::DataTransmissionHandshake(..) => 130,
            MavMessage::EncapsulatedData(..) => 131,
            MavMessage::DistanceSensor(..) => 132,
            MavMessage::TerrainRequest(..) => 133,
            MavMessage::TerrainData(..) => 134,
            MavMessage::TerrainCheck(..) => 135,
            MavMessage::TerrainReport(..) => 136,
            MavMessage::ScaledPressure2(..) => 137,
            MavMessage::AttPosMocap(..) => 138,
            MavMessage::SetActuatorControlTarget(..) => 139,
            MavMessage::ActuatorControlTarget(..) => 140,
            MavMessage::Altitude(..) => 141,
            MavMessage::ResourceRequest(..) => 142,
            MavMessage::ScaledPressure3(..) => 143,
            MavMessage::FollowTarget(..) => 144,
            MavMessage::ControlSystemState(..) => 146,
            MavMessage::BatteryStatus(..) => 147,
            MavMessage::AutopilotVersion(..) => 148,
            MavMessage::LandingTarget(..) => 149,
            MavMessage::FenceStatus(..) => 162,
            MavMessage::EstimatorStatus(..) => 230,
            MavMessage::WindCov(..) => 231,
            MavMessage::GpsInput(..) => 232,
            MavMessage::GpsRtcmData(..) => 233,
            MavMessage::HighLatency(..) => 234,
            MavMessage::HighLatency2(..) => 235,
            MavMessage::Vibration(..) => 241,
            MavMessage::HomePosition(..) => 242,
            MavMessage::SetHomePosition(..) => 243,
            MavMessage::MessageInterval(..) => 244,
            MavMessage::ExtendedSysState(..) => 245,
            MavMessage::AdsbVehicle(..) => 246,
            MavMessage::Collision(..) => 247,
            MavMessage::V2Extension(..) => 248,
            MavMessage::MemoryVect(..) => 249,
            MavMessage::DebugVect(..) => 250,
            MavMessage::NamedValueFloat(..) => 251,
            MavMessage::NamedValueInt(..) => 252,
            MavMessage::Statustext(..) => 253,
            MavMessage::Debug(..) => 254,
            MavMessage::SetupSigning(..) => 256,
            MavMessage::ButtonChange(..) => 257,
            MavMessage::PlayTune(..) => 258,
            MavMessage::CameraInformation(..) => 259,
            MavMessage::CameraSettings(..) => 260,
            MavMessage::StorageInformation(..) => 261,
            MavMessage::CameraCaptureStatus(..) => 262,
            MavMessage::CameraImageCaptured(..) => 263,
            MavMessage::FlightInformation(..) => 264,
            MavMessage::MountOrientation(..) => 265,
            MavMessage::LoggingData(..) => 266,
            MavMessage::LoggingDataAcked(..) => 267,
            MavMessage::LoggingAck(..) => 268,
            MavMessage::VideoStreamInformation(..) => 269,
            MavMessage::VideoStreamStatus(..) => 270,
            MavMessage::GimbalManagerInformation(..) => 280,
            MavMessage::GimbalManagerStatus(..) => 281,
            MavMessage::GimbalManagerSetAttitude(..) => 282,
            MavMessage::GimbalDeviceInformation(..) => 283,
            MavMessage::GimbalDeviceSetAttitude(..) => 284,
            MavMessage::GimbalDeviceAttitudeStatus(..) => 285,
            MavMessage::AutopilotStateForGimbalDevice(..) => 286,
            MavMessage::GimbalManagerSetTiltpan(..) => 287,
            MavMessage::WifiConfigAp(..) => 299,
            MavMessage::ProtocolVersion(..) => 300,
            MavMessage::AisVessel(..) => 301,
            MavMessage::UavcanNodeStatus(..) => 310,
            MavMessage::UavcanNodeInfo(..) => 311,
            MavMessage::ParamExtRequestRead(..) => 320,
            MavMessage::ParamExtRequestList(..) => 321,
            MavMessage::ParamExtValue(..) => 322,
            MavMessage::ParamExtSet(..) => 323,
            MavMessage::ParamExtAck(..) => 324,
            MavMessage::ObstacleDistance(..) => 330,
            MavMessage::Odometry(..) => 331,
            MavMessage::TrajectoryRepresentationWaypoints(..) => 332,
            MavMessage::TrajectoryRepresentationBezier(..) => 333,
            MavMessage::CellularStatus(..) => 334,
            MavMessage::IsbdLinkStatus(..) => 335,
            MavMessage::CellularConfig(..) => 336,
            MavMessage::RawRpm(..) => 339,
            MavMessage::UtmGlobalPosition(..) => 340,
            MavMessage::DebugFloatArray(..) => 350,
            MavMessage::OrbitExecutionStatus(..) => 360,
            MavMessage::SmartBatteryInfo(..) => 370,
            MavMessage::SmartBatteryStatus(..) => 371,
            MavMessage::GeneratorStatus(..) => 373,
            MavMessage::ActuatorOutputStatus(..) => 375,
            MavMessage::TimeEstimateToTarget(..) => 380,
            MavMessage::Tunnel(..) => 385,
            MavMessage::OnboardComputerStatus(..) => 390,
            MavMessage::ComponentInformation(..) => 395,
            MavMessage::PlayTuneV2(..) => 400,
            MavMessage::SupportedTunes(..) => 401,
            MavMessage::WheelDistance(..) => 9000,
            MavMessage::OpenDroneIdBasicId(..) => 12900,
            MavMessage::OpenDroneIdLocation(..) => 12901,
            MavMessage::OpenDroneIdAuthentication(..) => 12902,
            MavMessage::OpenDroneIdSelfId(..) => 12903,
            MavMessage::OpenDroneIdSystem(..) => 12904,
            MavMessage::OpenDroneIdOperatorId(..) => 12905,
            MavMessage::OpenDroneIdMessagePack(..) => 12915,
        }
    }
    fn message_id_from_name(name: &str) -> Result<u32, &'static str> {
        match name {
            "Heartbeat" => Ok(0),
            "SysStatus" => Ok(1),
            "SystemTime" => Ok(2),
            "Ping" => Ok(4),
            "ChangeOperatorControl" => Ok(5),
            "ChangeOperatorControlAck" => Ok(6),
            "AuthKey" => Ok(7),
            "LinkNodeStatus" => Ok(8),
            "SetMode" => Ok(11),
            "ParamRequestRead" => Ok(20),
            "ParamRequestList" => Ok(21),
            "ParamValue" => Ok(22),
            "ParamSet" => Ok(23),
            "GpsRawInt" => Ok(24),
            "GpsStatus" => Ok(25),
            "ScaledImu" => Ok(26),
            "RawImu" => Ok(27),
            "RawPressure" => Ok(28),
            "ScaledPressure" => Ok(29),
            "Attitude" => Ok(30),
            "AttitudeQuaternion" => Ok(31),
            "LocalPositionNed" => Ok(32),
            "GlobalPositionInt" => Ok(33),
            "RcChannelsScaled" => Ok(34),
            "RcChannelsRaw" => Ok(35),
            "ServoOutputRaw" => Ok(36),
            "MissionRequestPartialList" => Ok(37),
            "MissionWritePartialList" => Ok(38),
            "MissionItem" => Ok(39),
            "MissionRequest" => Ok(40),
            "MissionSetCurrent" => Ok(41),
            "MissionCurrent" => Ok(42),
            "MissionRequestList" => Ok(43),
            "MissionCount" => Ok(44),
            "MissionClearAll" => Ok(45),
            "MissionItemReached" => Ok(46),
            "MissionAck" => Ok(47),
            "SetGpsGlobalOrigin" => Ok(48),
            "GpsGlobalOrigin" => Ok(49),
            "ParamMapRc" => Ok(50),
            "MissionRequestInt" => Ok(51),
            "MissionChanged" => Ok(52),
            "SafetySetAllowedArea" => Ok(54),
            "SafetyAllowedArea" => Ok(55),
            "AttitudeQuaternionCov" => Ok(61),
            "NavControllerOutput" => Ok(62),
            "GlobalPositionIntCov" => Ok(63),
            "LocalPositionNedCov" => Ok(64),
            "RcChannels" => Ok(65),
            "RequestDataStream" => Ok(66),
            "DataStream" => Ok(67),
            "ManualControl" => Ok(69),
            "RcChannelsOverride" => Ok(70),
            "MissionItemInt" => Ok(73),
            "VfrHud" => Ok(74),
            "CommandInt" => Ok(75),
            "CommandLong" => Ok(76),
            "CommandAck" => Ok(77),
            "CommandCancel" => Ok(80),
            "ManualSetpoint" => Ok(81),
            "SetAttitudeTarget" => Ok(82),
            "AttitudeTarget" => Ok(83),
            "SetPositionTargetLocalNed" => Ok(84),
            "PositionTargetLocalNed" => Ok(85),
            "SetPositionTargetGlobalInt" => Ok(86),
            "PositionTargetGlobalInt" => Ok(87),
            "LocalPositionNedSystemGlobalOffset" => Ok(89),
            "HilState" => Ok(90),
            "HilControls" => Ok(91),
            "HilRcInputsRaw" => Ok(92),
            "HilActuatorControls" => Ok(93),
            "OpticalFlow" => Ok(100),
            "GlobalVisionPositionEstimate" => Ok(101),
            "VisionPositionEstimate" => Ok(102),
            "VisionSpeedEstimate" => Ok(103),
            "ViconPositionEstimate" => Ok(104),
            "HighresImu" => Ok(105),
            "OpticalFlowRad" => Ok(106),
            "HilSensor" => Ok(107),
            "SimState" => Ok(108),
            "RadioStatus" => Ok(109),
            "FileTransferProtocol" => Ok(110),
            "Timesync" => Ok(111),
            "CameraTrigger" => Ok(112),
            "HilGps" => Ok(113),
            "HilOpticalFlow" => Ok(114),
            "HilStateQuaternion" => Ok(115),
            "ScaledImu2" => Ok(116),
            "LogRequestList" => Ok(117),
            "LogEntry" => Ok(118),
            "LogRequestData" => Ok(119),
            "LogData" => Ok(120),
            "LogErase" => Ok(121),
            "LogRequestEnd" => Ok(122),
            "GpsInjectData" => Ok(123),
            "Gps2Raw" => Ok(124),
            "PowerStatus" => Ok(125),
            "SerialControl" => Ok(126),
            "GpsRtk" => Ok(127),
            "Gps2Rtk" => Ok(128),
            "ScaledImu3" => Ok(129),
            "DataTransmissionHandshake" => Ok(130),
            "EncapsulatedData" => Ok(131),
            "DistanceSensor" => Ok(132),
            "TerrainRequest" => Ok(133),
            "TerrainData" => Ok(134),
            "TerrainCheck" => Ok(135),
            "TerrainReport" => Ok(136),
            "ScaledPressure2" => Ok(137),
            "AttPosMocap" => Ok(138),
            "SetActuatorControlTarget" => Ok(139),
            "ActuatorControlTarget" => Ok(140),
            "Altitude" => Ok(141),
            "ResourceRequest" => Ok(142),
            "ScaledPressure3" => Ok(143),
            "FollowTarget" => Ok(144),
            "ControlSystemState" => Ok(146),
            "BatteryStatus" => Ok(147),
            "AutopilotVersion" => Ok(148),
            "LandingTarget" => Ok(149),
            "FenceStatus" => Ok(162),
            "EstimatorStatus" => Ok(230),
            "WindCov" => Ok(231),
            "GpsInput" => Ok(232),
            "GpsRtcmData" => Ok(233),
            "HighLatency" => Ok(234),
            "HighLatency2" => Ok(235),
            "Vibration" => Ok(241),
            "HomePosition" => Ok(242),
            "SetHomePosition" => Ok(243),
            "MessageInterval" => Ok(244),
            "ExtendedSysState" => Ok(245),
            "AdsbVehicle" => Ok(246),
            "Collision" => Ok(247),
            "V2Extension" => Ok(248),
            "MemoryVect" => Ok(249),
            "DebugVect" => Ok(250),
            "NamedValueFloat" => Ok(251),
            "NamedValueInt" => Ok(252),
            "Statustext" => Ok(253),
            "Debug" => Ok(254),
            "SetupSigning" => Ok(256),
            "ButtonChange" => Ok(257),
            "PlayTune" => Ok(258),
            "CameraInformation" => Ok(259),
            "CameraSettings" => Ok(260),
            "StorageInformation" => Ok(261),
            "CameraCaptureStatus" => Ok(262),
            "CameraImageCaptured" => Ok(263),
            "FlightInformation" => Ok(264),
            "MountOrientation" => Ok(265),
            "LoggingData" => Ok(266),
            "LoggingDataAcked" => Ok(267),
            "LoggingAck" => Ok(268),
            "VideoStreamInformation" => Ok(269),
            "VideoStreamStatus" => Ok(270),
            "GimbalManagerInformation" => Ok(280),
            "GimbalManagerStatus" => Ok(281),
            "GimbalManagerSetAttitude" => Ok(282),
            "GimbalDeviceInformation" => Ok(283),
            "GimbalDeviceSetAttitude" => Ok(284),
            "GimbalDeviceAttitudeStatus" => Ok(285),
            "AutopilotStateForGimbalDevice" => Ok(286),
            "GimbalManagerSetTiltpan" => Ok(287),
            "WifiConfigAp" => Ok(299),
            "ProtocolVersion" => Ok(300),
            "AisVessel" => Ok(301),
            "UavcanNodeStatus" => Ok(310),
            "UavcanNodeInfo" => Ok(311),
            "ParamExtRequestRead" => Ok(320),
            "ParamExtRequestList" => Ok(321),
            "ParamExtValue" => Ok(322),
            "ParamExtSet" => Ok(323),
            "ParamExtAck" => Ok(324),
            "ObstacleDistance" => Ok(330),
            "Odometry" => Ok(331),
            "TrajectoryRepresentationWaypoints" => Ok(332),
            "TrajectoryRepresentationBezier" => Ok(333),
            "CellularStatus" => Ok(334),
            "IsbdLinkStatus" => Ok(335),
            "CellularConfig" => Ok(336),
            "RawRpm" => Ok(339),
            "UtmGlobalPosition" => Ok(340),
            "DebugFloatArray" => Ok(350),
            "OrbitExecutionStatus" => Ok(360),
            "SmartBatteryInfo" => Ok(370),
            "SmartBatteryStatus" => Ok(371),
            "GeneratorStatus" => Ok(373),
            "ActuatorOutputStatus" => Ok(375),
            "TimeEstimateToTarget" => Ok(380),
            "Tunnel" => Ok(385),
            "OnboardComputerStatus" => Ok(390),
            "ComponentInformation" => Ok(395),
            "PlayTuneV2" => Ok(400),
            "SupportedTunes" => Ok(401),
            "WheelDistance" => Ok(9000),
            "OpenDroneIdBasicId" => Ok(12900),
            "OpenDroneIdLocation" => Ok(12901),
            "OpenDroneIdAuthentication" => Ok(12902),
            "OpenDroneIdSelfId" => Ok(12903),
            "OpenDroneIdSystem" => Ok(12904),
            "OpenDroneIdOperatorId" => Ok(12905),
            "OpenDroneIdMessagePack" => Ok(12915),
            _ => Err("Invalid message name."),
        }
    }
    fn default_message_from_id(id: u32) -> Result<MavMessage, &'static str> {
        match id {
            0 => Ok(MavMessage::Heartbeat(
                crate::proto::common::Heartbeat::default(),
            )),
            1 => Ok(MavMessage::SysStatus(
                crate::proto::common::SysStatus::default(),
            )),
            2 => Ok(MavMessage::SystemTime(
                crate::proto::common::SystemTime::default(),
            )),
            4 => Ok(MavMessage::Ping(crate::proto::common::Ping::default())),
            5 => Ok(MavMessage::ChangeOperatorControl(
                crate::proto::common::ChangeOperatorControl::default(),
            )),
            6 => Ok(MavMessage::ChangeOperatorControlAck(
                crate::proto::common::ChangeOperatorControlAck::default(),
            )),
            7 => Ok(MavMessage::AuthKey(crate::proto::common::AuthKey::default())),
            8 => Ok(MavMessage::LinkNodeStatus(
                crate::proto::common::LinkNodeStatus::default(),
            )),
            11 => Ok(MavMessage::SetMode(crate::proto::common::SetMode::default())),
            20 => Ok(MavMessage::ParamRequestRead(
                crate::proto::common::ParamRequestRead::default(),
            )),
            21 => Ok(MavMessage::ParamRequestList(
                crate::proto::common::ParamRequestList::default(),
            )),
            22 => Ok(MavMessage::ParamValue(
                crate::proto::common::ParamValue::default(),
            )),
            23 => Ok(MavMessage::ParamSet(
                crate::proto::common::ParamSet::default(),
            )),
            24 => Ok(MavMessage::GpsRawInt(
                crate::proto::common::GpsRawInt::default(),
            )),
            25 => Ok(MavMessage::GpsStatus(
                crate::proto::common::GpsStatus::default(),
            )),
            26 => Ok(MavMessage::ScaledImu(
                crate::proto::common::ScaledImu::default(),
            )),
            27 => Ok(MavMessage::RawImu(crate::proto::common::RawImu::default())),
            28 => Ok(MavMessage::RawPressure(
                crate::proto::common::RawPressure::default(),
            )),
            29 => Ok(MavMessage::ScaledPressure(
                crate::proto::common::ScaledPressure::default(),
            )),
            30 => Ok(MavMessage::Attitude(
                crate::proto::common::Attitude::default(),
            )),
            31 => Ok(MavMessage::AttitudeQuaternion(
                crate::proto::common::AttitudeQuaternion::default(),
            )),
            32 => Ok(MavMessage::LocalPositionNed(
                crate::proto::common::LocalPositionNed::default(),
            )),
            33 => Ok(MavMessage::GlobalPositionInt(
                crate::proto::common::GlobalPositionInt::default(),
            )),
            34 => Ok(MavMessage::RcChannelsScaled(
                crate::proto::common::RcChannelsScaled::default(),
            )),
            35 => Ok(MavMessage::RcChannelsRaw(
                crate::proto::common::RcChannelsRaw::default(),
            )),
            36 => Ok(MavMessage::ServoOutputRaw(
                crate::proto::common::ServoOutputRaw::default(),
            )),
            37 => Ok(MavMessage::MissionRequestPartialList(
                crate::proto::common::MissionRequestPartialList::default(),
            )),
            38 => Ok(MavMessage::MissionWritePartialList(
                crate::proto::common::MissionWritePartialList::default(),
            )),
            39 => Ok(MavMessage::MissionItem(
                crate::proto::common::MissionItem::default(),
            )),
            40 => Ok(MavMessage::MissionRequest(
                crate::proto::common::MissionRequest::default(),
            )),
            41 => Ok(MavMessage::MissionSetCurrent(
                crate::proto::common::MissionSetCurrent::default(),
            )),
            42 => Ok(MavMessage::MissionCurrent(
                crate::proto::common::MissionCurrent::default(),
            )),
            43 => Ok(MavMessage::MissionRequestList(
                crate::proto::common::MissionRequestList::default(),
            )),
            44 => Ok(MavMessage::MissionCount(
                crate::proto::common::MissionCount::default(),
            )),
            45 => Ok(MavMessage::MissionClearAll(
                crate::proto::common::MissionClearAll::default(),
            )),
            46 => Ok(MavMessage::MissionItemReached(
                crate::proto::common::MissionItemReached::default(),
            )),
            47 => Ok(MavMessage::MissionAck(
                crate::proto::common::MissionAck::default(),
            )),
            48 => Ok(MavMessage::SetGpsGlobalOrigin(
                crate::proto::common::SetGpsGlobalOrigin::default(),
            )),
            49 => Ok(MavMessage::GpsGlobalOrigin(
                crate::proto::common::GpsGlobalOrigin::default(),
            )),
            50 => Ok(MavMessage::ParamMapRc(
                crate::proto::common::ParamMapRc::default(),
            )),
            51 => Ok(MavMessage::MissionRequestInt(
                crate::proto::common::MissionRequestInt::default(),
            )),
            52 => Ok(MavMessage::MissionChanged(
                crate::proto::common::MissionChanged::default(),
            )),
            54 => Ok(MavMessage::SafetySetAllowedArea(
                crate::proto::common::SafetySetAllowedArea::default(),
            )),
            55 => Ok(MavMessage::SafetyAllowedArea(
                crate::proto::common::SafetyAllowedArea::default(),
            )),
            61 => Ok(MavMessage::AttitudeQuaternionCov(
                crate::proto::common::AttitudeQuaternionCov::default(),
            )),
            62 => Ok(MavMessage::NavControllerOutput(
                crate::proto::common::NavControllerOutput::default(),
            )),
            63 => Ok(MavMessage::GlobalPositionIntCov(
                crate::proto::common::GlobalPositionIntCov::default(),
            )),
            64 => Ok(MavMessage::LocalPositionNedCov(
                crate::proto::common::LocalPositionNedCov::default(),
            )),
            65 => Ok(MavMessage::RcChannels(
                crate::proto::common::RcChannels::default(),
            )),
            66 => Ok(MavMessage::RequestDataStream(
                crate::proto::common::RequestDataStream::default(),
            )),
            67 => Ok(MavMessage::DataStream(
                crate::proto::common::DataStream::default(),
            )),
            69 => Ok(MavMessage::ManualControl(
                crate::proto::common::ManualControl::default(),
            )),
            70 => Ok(MavMessage::RcChannelsOverride(
                crate::proto::common::RcChannelsOverride::default(),
            )),
            73 => Ok(MavMessage::MissionItemInt(
                crate::proto::common::MissionItemInt::default(),
            )),
            74 => Ok(MavMessage::VfrHud(crate::proto::common::VfrHud::default())),
            75 => Ok(MavMessage::CommandInt(
                crate::proto::common::CommandInt::default(),
            )),
            76 => Ok(MavMessage::CommandLong(
                crate::proto::common::CommandLong::default(),
            )),
            77 => Ok(MavMessage::CommandAck(
                crate::proto::common::CommandAck::default(),
            )),
            80 => Ok(MavMessage::CommandCancel(
                crate::proto::common::CommandCancel::default(),
            )),
            81 => Ok(MavMessage::ManualSetpoint(
                crate::proto::common::ManualSetpoint::default(),
            )),
            82 => Ok(MavMessage::SetAttitudeTarget(
                crate::proto::common::SetAttitudeTarget::default(),
            )),
            83 => Ok(MavMessage::AttitudeTarget(
                crate::proto::common::AttitudeTarget::default(),
            )),
            84 => Ok(MavMessage::SetPositionTargetLocalNed(
                crate::proto::common::SetPositionTargetLocalNed::default(),
            )),
            85 => Ok(MavMessage::PositionTargetLocalNed(
                crate::proto::common::PositionTargetLocalNed::default(),
            )),
            86 => Ok(MavMessage::SetPositionTargetGlobalInt(
                crate::proto::common::SetPositionTargetGlobalInt::default(),
            )),
            87 => Ok(MavMessage::PositionTargetGlobalInt(
                crate::proto::common::PositionTargetGlobalInt::default(),
            )),
            89 => Ok(MavMessage::LocalPositionNedSystemGlobalOffset(
                crate::proto::common::LocalPositionNedSystemGlobalOffset::default(),
            )),
            90 => Ok(MavMessage::HilState(
                crate::proto::common::HilState::default(),
            )),
            91 => Ok(MavMessage::HilControls(
                crate::proto::common::HilControls::default(),
            )),
            92 => Ok(MavMessage::HilRcInputsRaw(
                crate::proto::common::HilRcInputsRaw::default(),
            )),
            93 => Ok(MavMessage::HilActuatorControls(
                crate::proto::common::HilActuatorControls::default(),
            )),
            100 => Ok(MavMessage::OpticalFlow(
                crate::proto::common::OpticalFlow::default(),
            )),
            101 => Ok(MavMessage::GlobalVisionPositionEstimate(
                crate::proto::common::GlobalVisionPositionEstimate::default(),
            )),
            102 => Ok(MavMessage::VisionPositionEstimate(
                crate::proto::common::VisionPositionEstimate::default(),
            )),
            103 => Ok(MavMessage::VisionSpeedEstimate(
                crate::proto::common::VisionSpeedEstimate::default(),
            )),
            104 => Ok(MavMessage::ViconPositionEstimate(
                crate::proto::common::ViconPositionEstimate::default(),
            )),
            105 => Ok(MavMessage::HighresImu(
                crate::proto::common::HighresImu::default(),
            )),
            106 => Ok(MavMessage::OpticalFlowRad(
                crate::proto::common::OpticalFlowRad::default(),
            )),
            107 => Ok(MavMessage::HilSensor(
                crate::proto::common::HilSensor::default(),
            )),
            108 => Ok(MavMessage::SimState(
                crate::proto::common::SimState::default(),
            )),
            109 => Ok(MavMessage::RadioStatus(
                crate::proto::common::RadioStatus::default(),
            )),
            110 => Ok(MavMessage::FileTransferProtocol(
                crate::proto::common::FileTransferProtocol::default(),
            )),
            111 => Ok(MavMessage::Timesync(
                crate::proto::common::Timesync::default(),
            )),
            112 => Ok(MavMessage::CameraTrigger(
                crate::proto::common::CameraTrigger::default(),
            )),
            113 => Ok(MavMessage::HilGps(crate::proto::common::HilGps::default())),
            114 => Ok(MavMessage::HilOpticalFlow(
                crate::proto::common::HilOpticalFlow::default(),
            )),
            115 => Ok(MavMessage::HilStateQuaternion(
                crate::proto::common::HilStateQuaternion::default(),
            )),
            116 => Ok(MavMessage::ScaledImu2(
                crate::proto::common::ScaledImu2::default(),
            )),
            117 => Ok(MavMessage::LogRequestList(
                crate::proto::common::LogRequestList::default(),
            )),
            118 => Ok(MavMessage::LogEntry(
                crate::proto::common::LogEntry::default(),
            )),
            119 => Ok(MavMessage::LogRequestData(
                crate::proto::common::LogRequestData::default(),
            )),
            120 => Ok(MavMessage::LogData(crate::proto::common::LogData::default())),
            121 => Ok(MavMessage::LogErase(
                crate::proto::common::LogErase::default(),
            )),
            122 => Ok(MavMessage::LogRequestEnd(
                crate::proto::common::LogRequestEnd::default(),
            )),
            123 => Ok(MavMessage::GpsInjectData(
                crate::proto::common::GpsInjectData::default(),
            )),
            124 => Ok(MavMessage::Gps2Raw(crate::proto::common::Gps2Raw::default())),
            125 => Ok(MavMessage::PowerStatus(
                crate::proto::common::PowerStatus::default(),
            )),
            126 => Ok(MavMessage::SerialControl(
                crate::proto::common::SerialControl::default(),
            )),
            127 => Ok(MavMessage::GpsRtk(crate::proto::common::GpsRtk::default())),
            128 => Ok(MavMessage::Gps2Rtk(crate::proto::common::Gps2Rtk::default())),
            129 => Ok(MavMessage::ScaledImu3(
                crate::proto::common::ScaledImu3::default(),
            )),
            130 => Ok(MavMessage::DataTransmissionHandshake(
                crate::proto::common::DataTransmissionHandshake::default(),
            )),
            131 => Ok(MavMessage::EncapsulatedData(
                crate::proto::common::EncapsulatedData::default(),
            )),
            132 => Ok(MavMessage::DistanceSensor(
                crate::proto::common::DistanceSensor::default(),
            )),
            133 => Ok(MavMessage::TerrainRequest(
                crate::proto::common::TerrainRequest::default(),
            )),
            134 => Ok(MavMessage::TerrainData(
                crate::proto::common::TerrainData::default(),
            )),
            135 => Ok(MavMessage::TerrainCheck(
                crate::proto::common::TerrainCheck::default(),
            )),
            136 => Ok(MavMessage::TerrainReport(
                crate::proto::common::TerrainReport::default(),
            )),
            137 => Ok(MavMessage::ScaledPressure2(
                crate::proto::common::ScaledPressure2::default(),
            )),
            138 => Ok(MavMessage::AttPosMocap(
                crate::proto::common::AttPosMocap::default(),
            )),
            139 => Ok(MavMessage::SetActuatorControlTarget(
                crate::proto::common::SetActuatorControlTarget::default(),
            )),
            140 => Ok(MavMessage::ActuatorControlTarget(
                crate::proto::common::ActuatorControlTarget::default(),
            )),
            141 => Ok(MavMessage::Altitude(
                crate::proto::common::Altitude::default(),
            )),
            142 => Ok(MavMessage::ResourceRequest(
                crate::proto::common::ResourceRequest::default(),
            )),
            143 => Ok(MavMessage::ScaledPressure3(
                crate::proto::common::ScaledPressure3::default(),
            )),
            144 => Ok(MavMessage::FollowTarget(
                crate::proto::common::FollowTarget::default(),
            )),
            146 => Ok(MavMessage::ControlSystemState(
                crate::proto::common::ControlSystemState::default(),
            )),
            147 => Ok(MavMessage::BatteryStatus(
                crate::proto::common::BatteryStatus::default(),
            )),
            148 => Ok(MavMessage::AutopilotVersion(
                crate::proto::common::AutopilotVersion::default(),
            )),
            149 => Ok(MavMessage::LandingTarget(
                crate::proto::common::LandingTarget::default(),
            )),
            162 => Ok(MavMessage::FenceStatus(
                crate::proto::common::FenceStatus::default(),
            )),
            230 => Ok(MavMessage::EstimatorStatus(
                crate::proto::common::EstimatorStatus::default(),
            )),
            231 => Ok(MavMessage::WindCov(crate::proto::common::WindCov::default())),
            232 => Ok(MavMessage::GpsInput(
                crate::proto::common::GpsInput::default(),
            )),
            233 => Ok(MavMessage::GpsRtcmData(
                crate::proto::common::GpsRtcmData::default(),
            )),
            234 => Ok(MavMessage::HighLatency(
                crate::proto::common::HighLatency::default(),
            )),
            235 => Ok(MavMessage::HighLatency2(
                crate::proto::common::HighLatency2::default(),
            )),
            241 => Ok(MavMessage::Vibration(
                crate::proto::common::Vibration::default(),
            )),
            242 => Ok(MavMessage::HomePosition(
                crate::proto::common::HomePosition::default(),
            )),
            243 => Ok(MavMessage::SetHomePosition(
                crate::proto::common::SetHomePosition::default(),
            )),
            244 => Ok(MavMessage::MessageInterval(
                crate::proto::common::MessageInterval::default(),
            )),
            245 => Ok(MavMessage::ExtendedSysState(
                crate::proto::common::ExtendedSysState::default(),
            )),
            246 => Ok(MavMessage::AdsbVehicle(
                crate::proto::common::AdsbVehicle::default(),
            )),
            247 => Ok(MavMessage::Collision(
                crate::proto::common::Collision::default(),
            )),
            248 => Ok(MavMessage::V2Extension(
                crate::proto::common::V2Extension::default(),
            )),
            249 => Ok(MavMessage::MemoryVect(
                crate::proto::common::MemoryVect::default(),
            )),
            250 => Ok(MavMessage::DebugVect(
                crate::proto::common::DebugVect::default(),
            )),
            251 => Ok(MavMessage::NamedValueFloat(
                crate::proto::common::NamedValueFloat::default(),
            )),
            252 => Ok(MavMessage::NamedValueInt(
                crate::proto::common::NamedValueInt::default(),
            )),
            253 => Ok(MavMessage::Statustext(
                crate::proto::common::Statustext::default(),
            )),
            254 => Ok(MavMessage::Debug(crate::proto::common::Debug::default())),
            256 => Ok(MavMessage::SetupSigning(
                crate::proto::common::SetupSigning::default(),
            )),
            257 => Ok(MavMessage::ButtonChange(
                crate::proto::common::ButtonChange::default(),
            )),
            258 => Ok(MavMessage::PlayTune(
                crate::proto::common::PlayTune::default(),
            )),
            259 => Ok(MavMessage::CameraInformation(
                crate::proto::common::CameraInformation::default(),
            )),
            260 => Ok(MavMessage::CameraSettings(
                crate::proto::common::CameraSettings::default(),
            )),
            261 => Ok(MavMessage::StorageInformation(
                crate::proto::common::StorageInformation::default(),
            )),
            262 => Ok(MavMessage::CameraCaptureStatus(
                crate::proto::common::CameraCaptureStatus::default(),
            )),
            263 => Ok(MavMessage::CameraImageCaptured(
                crate::proto::common::CameraImageCaptured::default(),
            )),
            264 => Ok(MavMessage::FlightInformation(
                crate::proto::common::FlightInformation::default(),
            )),
            265 => Ok(MavMessage::MountOrientation(
                crate::proto::common::MountOrientation::default(),
            )),
            266 => Ok(MavMessage::LoggingData(
                crate::proto::common::LoggingData::default(),
            )),
            267 => Ok(MavMessage::LoggingDataAcked(
                crate::proto::common::LoggingDataAcked::default(),
            )),
            268 => Ok(MavMessage::LoggingAck(
                crate::proto::common::LoggingAck::default(),
            )),
            269 => Ok(MavMessage::VideoStreamInformation(
                crate::proto::common::VideoStreamInformation::default(),
            )),
            270 => Ok(MavMessage::VideoStreamStatus(
                crate::proto::common::VideoStreamStatus::default(),
            )),
            280 => Ok(MavMessage::GimbalManagerInformation(
                crate::proto::common::GimbalManagerInformation::default(),
            )),
            281 => Ok(MavMessage::GimbalManagerStatus(
                crate::proto::common::GimbalManagerStatus::default(),
            )),
            282 => Ok(MavMessage::GimbalManagerSetAttitude(
                crate::proto::common::GimbalManagerSetAttitude::default(),
            )),
            283 => Ok(MavMessage::GimbalDeviceInformation(
                crate::proto::common::GimbalDeviceInformation::default(),
            )),
            284 => Ok(MavMessage::GimbalDeviceSetAttitude(
                crate::proto::common::GimbalDeviceSetAttitude::default(),
            )),
            285 => Ok(MavMessage::GimbalDeviceAttitudeStatus(
                crate::proto::common::GimbalDeviceAttitudeStatus::default(),
            )),
            286 => Ok(MavMessage::AutopilotStateForGimbalDevice(
                crate::proto::common::AutopilotStateForGimbalDevice::default(),
            )),
            287 => Ok(MavMessage::GimbalManagerSetTiltpan(
                crate::proto::common::GimbalManagerSetTiltpan::default(),
            )),
            299 => Ok(MavMessage::WifiConfigAp(
                crate::proto::common::WifiConfigAp::default(),
            )),
            300 => Ok(MavMessage::ProtocolVersion(
                crate::proto::common::ProtocolVersion::default(),
            )),
            301 => Ok(MavMessage::AisVessel(
                crate::proto::common::AisVessel::default(),
            )),
            310 => Ok(MavMessage::UavcanNodeStatus(
                crate::proto::common::UavcanNodeStatus::default(),
            )),
            311 => Ok(MavMessage::UavcanNodeInfo(
                crate::proto::common::UavcanNodeInfo::default(),
            )),
            320 => Ok(MavMessage::ParamExtRequestRead(
                crate::proto::common::ParamExtRequestRead::default(),
            )),
            321 => Ok(MavMessage::ParamExtRequestList(
                crate::proto::common::ParamExtRequestList::default(),
            )),
            322 => Ok(MavMessage::ParamExtValue(
                crate::proto::common::ParamExtValue::default(),
            )),
            323 => Ok(MavMessage::ParamExtSet(
                crate::proto::common::ParamExtSet::default(),
            )),
            324 => Ok(MavMessage::ParamExtAck(
                crate::proto::common::ParamExtAck::default(),
            )),
            330 => Ok(MavMessage::ObstacleDistance(
                crate::proto::common::ObstacleDistance::default(),
            )),
            331 => Ok(MavMessage::Odometry(
                crate::proto::common::Odometry::default(),
            )),
            332 => Ok(MavMessage::TrajectoryRepresentationWaypoints(
                crate::proto::common::TrajectoryRepresentationWaypoints::default(),
            )),
            333 => Ok(MavMessage::TrajectoryRepresentationBezier(
                crate::proto::common::TrajectoryRepresentationBezier::default(),
            )),
            334 => Ok(MavMessage::CellularStatus(
                crate::proto::common::CellularStatus::default(),
            )),
            335 => Ok(MavMessage::IsbdLinkStatus(
                crate::proto::common::IsbdLinkStatus::default(),
            )),
            336 => Ok(MavMessage::CellularConfig(
                crate::proto::common::CellularConfig::default(),
            )),
            339 => Ok(MavMessage::RawRpm(crate::proto::common::RawRpm::default())),
            340 => Ok(MavMessage::UtmGlobalPosition(
                crate::proto::common::UtmGlobalPosition::default(),
            )),
            350 => Ok(MavMessage::DebugFloatArray(
                crate::proto::common::DebugFloatArray::default(),
            )),
            360 => Ok(MavMessage::OrbitExecutionStatus(
                crate::proto::common::OrbitExecutionStatus::default(),
            )),
            370 => Ok(MavMessage::SmartBatteryInfo(
                crate::proto::common::SmartBatteryInfo::default(),
            )),
            371 => Ok(MavMessage::SmartBatteryStatus(
                crate::proto::common::SmartBatteryStatus::default(),
            )),
            373 => Ok(MavMessage::GeneratorStatus(
                crate::proto::common::GeneratorStatus::default(),
            )),
            375 => Ok(MavMessage::ActuatorOutputStatus(
                crate::proto::common::ActuatorOutputStatus::default(),
            )),
            380 => Ok(MavMessage::TimeEstimateToTarget(
                crate::proto::common::TimeEstimateToTarget::default(),
            )),
            385 => Ok(MavMessage::Tunnel(crate::proto::common::Tunnel::default())),
            390 => Ok(MavMessage::OnboardComputerStatus(
                crate::proto::common::OnboardComputerStatus::default(),
            )),
            395 => Ok(MavMessage::ComponentInformation(
                crate::proto::common::ComponentInformation::default(),
            )),
            400 => Ok(MavMessage::PlayTuneV2(
                crate::proto::common::PlayTuneV2::default(),
            )),
            401 => Ok(MavMessage::SupportedTunes(
                crate::proto::common::SupportedTunes::default(),
            )),
            9000 => Ok(MavMessage::WheelDistance(
                crate::proto::common::WheelDistance::default(),
            )),
            12900 => Ok(MavMessage::OpenDroneIdBasicId(
                crate::proto::common::OpenDroneIdBasicId::default(),
            )),
            12901 => Ok(MavMessage::OpenDroneIdLocation(
                crate::proto::common::OpenDroneIdLocation::default(),
            )),
            12902 => Ok(MavMessage::OpenDroneIdAuthentication(
                crate::proto::common::OpenDroneIdAuthentication::default(),
            )),
            12903 => Ok(MavMessage::OpenDroneIdSelfId(
                crate::proto::common::OpenDroneIdSelfId::default(),
            )),
            12904 => Ok(MavMessage::OpenDroneIdSystem(
                crate::proto::common::OpenDroneIdSystem::default(),
            )),
            12905 => Ok(MavMessage::OpenDroneIdOperatorId(
                crate::proto::common::OpenDroneIdOperatorId::default(),
            )),
            12915 => Ok(MavMessage::OpenDroneIdMessagePack(
                crate::proto::common::OpenDroneIdMessagePack::default(),
            )),
            _ => Err("Invalid message id."),
        }
    }
    fn mavlink_ser(&self) -> Vec<u8> {
        match *self {
            MavMessage::Heartbeat(ref body) => body.mavlink_ser(),
            MavMessage::SysStatus(ref body) => body.mavlink_ser(),
            MavMessage::SystemTime(ref body) => body.mavlink_ser(),
            MavMessage::Ping(ref body) => body.mavlink_ser(),
            MavMessage::ChangeOperatorControl(ref body) => body.mavlink_ser(),
            MavMessage::ChangeOperatorControlAck(ref body) => body.mavlink_ser(),
            MavMessage::AuthKey(ref body) => body.mavlink_ser(),
            MavMessage::LinkNodeStatus(ref body) => body.mavlink_ser(),
            MavMessage::SetMode(ref body) => body.mavlink_ser(),
            MavMessage::ParamRequestRead(ref body) => body.mavlink_ser(),
            MavMessage::ParamRequestList(ref body) => body.mavlink_ser(),
            MavMessage::ParamValue(ref body) => body.mavlink_ser(),
            MavMessage::ParamSet(ref body) => body.mavlink_ser(),
            MavMessage::GpsRawInt(ref body) => body.mavlink_ser(),
            MavMessage::GpsStatus(ref body) => body.mavlink_ser(),
            MavMessage::ScaledImu(ref body) => body.mavlink_ser(),
            MavMessage::RawImu(ref body) => body.mavlink_ser(),
            MavMessage::RawPressure(ref body) => body.mavlink_ser(),
            MavMessage::ScaledPressure(ref body) => body.mavlink_ser(),
            MavMessage::Attitude(ref body) => body.mavlink_ser(),
            MavMessage::AttitudeQuaternion(ref body) => body.mavlink_ser(),
            MavMessage::LocalPositionNed(ref body) => body.mavlink_ser(),
            MavMessage::GlobalPositionInt(ref body) => body.mavlink_ser(),
            MavMessage::RcChannelsScaled(ref body) => body.mavlink_ser(),
            MavMessage::RcChannelsRaw(ref body) => body.mavlink_ser(),
            MavMessage::ServoOutputRaw(ref body) => body.mavlink_ser(),
            MavMessage::MissionRequestPartialList(ref body) => body.mavlink_ser(),
            MavMessage::MissionWritePartialList(ref body) => body.mavlink_ser(),
            MavMessage::MissionItem(ref body) => body.mavlink_ser(),
            MavMessage::MissionRequest(ref body) => body.mavlink_ser(),
            MavMessage::MissionSetCurrent(ref body) => body.mavlink_ser(),
            MavMessage::MissionCurrent(ref body) => body.mavlink_ser(),
            MavMessage::MissionRequestList(ref body) => body.mavlink_ser(),
            MavMessage::MissionCount(ref body) => body.mavlink_ser(),
            MavMessage::MissionClearAll(ref body) => body.mavlink_ser(),
            MavMessage::MissionItemReached(ref body) => body.mavlink_ser(),
            MavMessage::MissionAck(ref body) => body.mavlink_ser(),
            MavMessage::SetGpsGlobalOrigin(ref body) => body.mavlink_ser(),
            MavMessage::GpsGlobalOrigin(ref body) => body.mavlink_ser(),
            MavMessage::ParamMapRc(ref body) => body.mavlink_ser(),
            MavMessage::MissionRequestInt(ref body) => body.mavlink_ser(),
            MavMessage::MissionChanged(ref body) => body.mavlink_ser(),
            MavMessage::SafetySetAllowedArea(ref body) => body.mavlink_ser(),
            MavMessage::SafetyAllowedArea(ref body) => body.mavlink_ser(),
            MavMessage::AttitudeQuaternionCov(ref body) => body.mavlink_ser(),
            MavMessage::NavControllerOutput(ref body) => body.mavlink_ser(),
            MavMessage::GlobalPositionIntCov(ref body) => body.mavlink_ser(),
            MavMessage::LocalPositionNedCov(ref body) => body.mavlink_ser(),
            MavMessage::RcChannels(ref body) => body.mavlink_ser(),
            MavMessage::RequestDataStream(ref body) => body.mavlink_ser(),
            MavMessage::DataStream(ref body) => body.mavlink_ser(),
            MavMessage::ManualControl(ref body) => body.mavlink_ser(),
            MavMessage::RcChannelsOverride(ref body) => body.mavlink_ser(),
            MavMessage::MissionItemInt(ref body) => body.mavlink_ser(),
            MavMessage::VfrHud(ref body) => body.mavlink_ser(),
            MavMessage::CommandInt(ref body) => body.mavlink_ser(),
            MavMessage::CommandLong(ref body) => body.mavlink_ser(),
            MavMessage::CommandAck(ref body) => body.mavlink_ser(),
            MavMessage::CommandCancel(ref body) => body.mavlink_ser(),
            MavMessage::ManualSetpoint(ref body) => body.mavlink_ser(),
            MavMessage::SetAttitudeTarget(ref body) => body.mavlink_ser(),
            MavMessage::AttitudeTarget(ref body) => body.mavlink_ser(),
            MavMessage::SetPositionTargetLocalNed(ref body) => body.mavlink_ser(),
            MavMessage::PositionTargetLocalNed(ref body) => body.mavlink_ser(),
            MavMessage::SetPositionTargetGlobalInt(ref body) => body.mavlink_ser(),
            MavMessage::PositionTargetGlobalInt(ref body) => body.mavlink_ser(),
            MavMessage::LocalPositionNedSystemGlobalOffset(ref body) => body.mavlink_ser(),
            MavMessage::HilState(ref body) => body.mavlink_ser(),
            MavMessage::HilControls(ref body) => body.mavlink_ser(),
            MavMessage::HilRcInputsRaw(ref body) => body.mavlink_ser(),
            MavMessage::HilActuatorControls(ref body) => body.mavlink_ser(),
            MavMessage::OpticalFlow(ref body) => body.mavlink_ser(),
            MavMessage::GlobalVisionPositionEstimate(ref body) => body.mavlink_ser(),
            MavMessage::VisionPositionEstimate(ref body) => body.mavlink_ser(),
            MavMessage::VisionSpeedEstimate(ref body) => body.mavlink_ser(),
            MavMessage::ViconPositionEstimate(ref body) => body.mavlink_ser(),
            MavMessage::HighresImu(ref body) => body.mavlink_ser(),
            MavMessage::OpticalFlowRad(ref body) => body.mavlink_ser(),
            MavMessage::HilSensor(ref body) => body.mavlink_ser(),
            MavMessage::SimState(ref body) => body.mavlink_ser(),
            MavMessage::RadioStatus(ref body) => body.mavlink_ser(),
            MavMessage::FileTransferProtocol(ref body) => body.mavlink_ser(),
            MavMessage::Timesync(ref body) => body.mavlink_ser(),
            MavMessage::CameraTrigger(ref body) => body.mavlink_ser(),
            MavMessage::HilGps(ref body) => body.mavlink_ser(),
            MavMessage::HilOpticalFlow(ref body) => body.mavlink_ser(),
            MavMessage::HilStateQuaternion(ref body) => body.mavlink_ser(),
            MavMessage::ScaledImu2(ref body) => body.mavlink_ser(),
            MavMessage::LogRequestList(ref body) => body.mavlink_ser(),
            MavMessage::LogEntry(ref body) => body.mavlink_ser(),
            MavMessage::LogRequestData(ref body) => body.mavlink_ser(),
            MavMessage::LogData(ref body) => body.mavlink_ser(),
            MavMessage::LogErase(ref body) => body.mavlink_ser(),
            MavMessage::LogRequestEnd(ref body) => body.mavlink_ser(),
            MavMessage::GpsInjectData(ref body) => body.mavlink_ser(),
            MavMessage::Gps2Raw(ref body) => body.mavlink_ser(),
            MavMessage::PowerStatus(ref body) => body.mavlink_ser(),
            MavMessage::SerialControl(ref body) => body.mavlink_ser(),
            MavMessage::GpsRtk(ref body) => body.mavlink_ser(),
            MavMessage::Gps2Rtk(ref body) => body.mavlink_ser(),
            MavMessage::ScaledImu3(ref body) => body.mavlink_ser(),
            MavMessage::DataTransmissionHandshake(ref body) => body.mavlink_ser(),
            MavMessage::EncapsulatedData(ref body) => body.mavlink_ser(),
            MavMessage::DistanceSensor(ref body) => body.mavlink_ser(),
            MavMessage::TerrainRequest(ref body) => body.mavlink_ser(),
            MavMessage::TerrainData(ref body) => body.mavlink_ser(),
            MavMessage::TerrainCheck(ref body) => body.mavlink_ser(),
            MavMessage::TerrainReport(ref body) => body.mavlink_ser(),
            MavMessage::ScaledPressure2(ref body) => body.mavlink_ser(),
            MavMessage::AttPosMocap(ref body) => body.mavlink_ser(),
            MavMessage::SetActuatorControlTarget(ref body) => body.mavlink_ser(),
            MavMessage::ActuatorControlTarget(ref body) => body.mavlink_ser(),
            MavMessage::Altitude(ref body) => body.mavlink_ser(),
            MavMessage::ResourceRequest(ref body) => body.mavlink_ser(),
            MavMessage::ScaledPressure3(ref body) => body.mavlink_ser(),
            MavMessage::FollowTarget(ref body) => body.mavlink_ser(),
            MavMessage::ControlSystemState(ref body) => body.mavlink_ser(),
            MavMessage::BatteryStatus(ref body) => body.mavlink_ser(),
            MavMessage::AutopilotVersion(ref body) => body.mavlink_ser(),
            MavMessage::LandingTarget(ref body) => body.mavlink_ser(),
            MavMessage::FenceStatus(ref body) => body.mavlink_ser(),
            MavMessage::EstimatorStatus(ref body) => body.mavlink_ser(),
            MavMessage::WindCov(ref body) => body.mavlink_ser(),
            MavMessage::GpsInput(ref body) => body.mavlink_ser(),
            MavMessage::GpsRtcmData(ref body) => body.mavlink_ser(),
            MavMessage::HighLatency(ref body) => body.mavlink_ser(),
            MavMessage::HighLatency2(ref body) => body.mavlink_ser(),
            MavMessage::Vibration(ref body) => body.mavlink_ser(),
            MavMessage::HomePosition(ref body) => body.mavlink_ser(),
            MavMessage::SetHomePosition(ref body) => body.mavlink_ser(),
            MavMessage::MessageInterval(ref body) => body.mavlink_ser(),
            MavMessage::ExtendedSysState(ref body) => body.mavlink_ser(),
            MavMessage::AdsbVehicle(ref body) => body.mavlink_ser(),
            MavMessage::Collision(ref body) => body.mavlink_ser(),
            MavMessage::V2Extension(ref body) => body.mavlink_ser(),
            MavMessage::MemoryVect(ref body) => body.mavlink_ser(),
            MavMessage::DebugVect(ref body) => body.mavlink_ser(),
            MavMessage::NamedValueFloat(ref body) => body.mavlink_ser(),
            MavMessage::NamedValueInt(ref body) => body.mavlink_ser(),
            MavMessage::Statustext(ref body) => body.mavlink_ser(),
            MavMessage::Debug(ref body) => body.mavlink_ser(),
            MavMessage::SetupSigning(ref body) => body.mavlink_ser(),
            MavMessage::ButtonChange(ref body) => body.mavlink_ser(),
            MavMessage::PlayTune(ref body) => body.mavlink_ser(),
            MavMessage::CameraInformation(ref body) => body.mavlink_ser(),
            MavMessage::CameraSettings(ref body) => body.mavlink_ser(),
            MavMessage::StorageInformation(ref body) => body.mavlink_ser(),
            MavMessage::CameraCaptureStatus(ref body) => body.mavlink_ser(),
            MavMessage::CameraImageCaptured(ref body) => body.mavlink_ser(),
            MavMessage::FlightInformation(ref body) => body.mavlink_ser(),
            MavMessage::MountOrientation(ref body) => body.mavlink_ser(),
            MavMessage::LoggingData(ref body) => body.mavlink_ser(),
            MavMessage::LoggingDataAcked(ref body) => body.mavlink_ser(),
            MavMessage::LoggingAck(ref body) => body.mavlink_ser(),
            MavMessage::VideoStreamInformation(ref body) => body.mavlink_ser(),
            MavMessage::VideoStreamStatus(ref body) => body.mavlink_ser(),
            MavMessage::GimbalManagerInformation(ref body) => body.mavlink_ser(),
            MavMessage::GimbalManagerStatus(ref body) => body.mavlink_ser(),
            MavMessage::GimbalManagerSetAttitude(ref body) => body.mavlink_ser(),
            MavMessage::GimbalDeviceInformation(ref body) => body.mavlink_ser(),
            MavMessage::GimbalDeviceSetAttitude(ref body) => body.mavlink_ser(),
            MavMessage::GimbalDeviceAttitudeStatus(ref body) => body.mavlink_ser(),
            MavMessage::AutopilotStateForGimbalDevice(ref body) => body.mavlink_ser(),
            MavMessage::GimbalManagerSetTiltpan(ref body) => body.mavlink_ser(),
            MavMessage::WifiConfigAp(ref body) => body.mavlink_ser(),
            MavMessage::ProtocolVersion(ref body) => body.mavlink_ser(),
            MavMessage::AisVessel(ref body) => body.mavlink_ser(),
            MavMessage::UavcanNodeStatus(ref body) => body.mavlink_ser(),
            MavMessage::UavcanNodeInfo(ref body) => body.mavlink_ser(),
            MavMessage::ParamExtRequestRead(ref body) => body.mavlink_ser(),
            MavMessage::ParamExtRequestList(ref body) => body.mavlink_ser(),
            MavMessage::ParamExtValue(ref body) => body.mavlink_ser(),
            MavMessage::ParamExtSet(ref body) => body.mavlink_ser(),
            MavMessage::ParamExtAck(ref body) => body.mavlink_ser(),
            MavMessage::ObstacleDistance(ref body) => body.mavlink_ser(),
            MavMessage::Odometry(ref body) => body.mavlink_ser(),
            MavMessage::TrajectoryRepresentationWaypoints(ref body) => body.mavlink_ser(),
            MavMessage::TrajectoryRepresentationBezier(ref body) => body.mavlink_ser(),
            MavMessage::CellularStatus(ref body) => body.mavlink_ser(),
            MavMessage::IsbdLinkStatus(ref body) => body.mavlink_ser(),
            MavMessage::CellularConfig(ref body) => body.mavlink_ser(),
            MavMessage::RawRpm(ref body) => body.mavlink_ser(),
            MavMessage::UtmGlobalPosition(ref body) => body.mavlink_ser(),
            MavMessage::DebugFloatArray(ref body) => body.mavlink_ser(),
            MavMessage::OrbitExecutionStatus(ref body) => body.mavlink_ser(),
            MavMessage::SmartBatteryInfo(ref body) => body.mavlink_ser(),
            MavMessage::SmartBatteryStatus(ref body) => body.mavlink_ser(),
            MavMessage::GeneratorStatus(ref body) => body.mavlink_ser(),
            MavMessage::ActuatorOutputStatus(ref body) => body.mavlink_ser(),
            MavMessage::TimeEstimateToTarget(ref body) => body.mavlink_ser(),
            MavMessage::Tunnel(ref body) => body.mavlink_ser(),
            MavMessage::OnboardComputerStatus(ref body) => body.mavlink_ser(),
            MavMessage::ComponentInformation(ref body) => body.mavlink_ser(),
            MavMessage::PlayTuneV2(ref body) => body.mavlink_ser(),
            MavMessage::SupportedTunes(ref body) => body.mavlink_ser(),
            MavMessage::WheelDistance(ref body) => body.mavlink_ser(),
            MavMessage::OpenDroneIdBasicId(ref body) => body.mavlink_ser(),
            MavMessage::OpenDroneIdLocation(ref body) => body.mavlink_ser(),
            MavMessage::OpenDroneIdAuthentication(ref body) => body.mavlink_ser(),
            MavMessage::OpenDroneIdSelfId(ref body) => body.mavlink_ser(),
            MavMessage::OpenDroneIdSystem(ref body) => body.mavlink_ser(),
            MavMessage::OpenDroneIdOperatorId(ref body) => body.mavlink_ser(),
            MavMessage::OpenDroneIdMessagePack(ref body) => body.mavlink_ser(),
        }
    }
    fn proto_encode(&self) -> Vec<u8> {
        match *self {
            MavMessage::Heartbeat(ref body) => body.encode_to_vec(),
            MavMessage::SysStatus(ref body) => body.encode_to_vec(),
            MavMessage::SystemTime(ref body) => body.encode_to_vec(),
            MavMessage::Ping(ref body) => body.encode_to_vec(),
            MavMessage::ChangeOperatorControl(ref body) => body.encode_to_vec(),
            MavMessage::ChangeOperatorControlAck(ref body) => body.encode_to_vec(),
            MavMessage::AuthKey(ref body) => body.encode_to_vec(),
            MavMessage::LinkNodeStatus(ref body) => body.encode_to_vec(),
            MavMessage::SetMode(ref body) => body.encode_to_vec(),
            MavMessage::ParamRequestRead(ref body) => body.encode_to_vec(),
            MavMessage::ParamRequestList(ref body) => body.encode_to_vec(),
            MavMessage::ParamValue(ref body) => body.encode_to_vec(),
            MavMessage::ParamSet(ref body) => body.encode_to_vec(),
            MavMessage::GpsRawInt(ref body) => body.encode_to_vec(),
            MavMessage::GpsStatus(ref body) => body.encode_to_vec(),
            MavMessage::ScaledImu(ref body) => body.encode_to_vec(),
            MavMessage::RawImu(ref body) => body.encode_to_vec(),
            MavMessage::RawPressure(ref body) => body.encode_to_vec(),
            MavMessage::ScaledPressure(ref body) => body.encode_to_vec(),
            MavMessage::Attitude(ref body) => body.encode_to_vec(),
            MavMessage::AttitudeQuaternion(ref body) => body.encode_to_vec(),
            MavMessage::LocalPositionNed(ref body) => body.encode_to_vec(),
            MavMessage::GlobalPositionInt(ref body) => body.encode_to_vec(),
            MavMessage::RcChannelsScaled(ref body) => body.encode_to_vec(),
            MavMessage::RcChannelsRaw(ref body) => body.encode_to_vec(),
            MavMessage::ServoOutputRaw(ref body) => body.encode_to_vec(),
            MavMessage::MissionRequestPartialList(ref body) => body.encode_to_vec(),
            MavMessage::MissionWritePartialList(ref body) => body.encode_to_vec(),
            MavMessage::MissionItem(ref body) => body.encode_to_vec(),
            MavMessage::MissionRequest(ref body) => body.encode_to_vec(),
            MavMessage::MissionSetCurrent(ref body) => body.encode_to_vec(),
            MavMessage::MissionCurrent(ref body) => body.encode_to_vec(),
            MavMessage::MissionRequestList(ref body) => body.encode_to_vec(),
            MavMessage::MissionCount(ref body) => body.encode_to_vec(),
            MavMessage::MissionClearAll(ref body) => body.encode_to_vec(),
            MavMessage::MissionItemReached(ref body) => body.encode_to_vec(),
            MavMessage::MissionAck(ref body) => body.encode_to_vec(),
            MavMessage::SetGpsGlobalOrigin(ref body) => body.encode_to_vec(),
            MavMessage::GpsGlobalOrigin(ref body) => body.encode_to_vec(),
            MavMessage::ParamMapRc(ref body) => body.encode_to_vec(),
            MavMessage::MissionRequestInt(ref body) => body.encode_to_vec(),
            MavMessage::MissionChanged(ref body) => body.encode_to_vec(),
            MavMessage::SafetySetAllowedArea(ref body) => body.encode_to_vec(),
            MavMessage::SafetyAllowedArea(ref body) => body.encode_to_vec(),
            MavMessage::AttitudeQuaternionCov(ref body) => body.encode_to_vec(),
            MavMessage::NavControllerOutput(ref body) => body.encode_to_vec(),
            MavMessage::GlobalPositionIntCov(ref body) => body.encode_to_vec(),
            MavMessage::LocalPositionNedCov(ref body) => body.encode_to_vec(),
            MavMessage::RcChannels(ref body) => body.encode_to_vec(),
            MavMessage::RequestDataStream(ref body) => body.encode_to_vec(),
            MavMessage::DataStream(ref body) => body.encode_to_vec(),
            MavMessage::ManualControl(ref body) => body.encode_to_vec(),
            MavMessage::RcChannelsOverride(ref body) => body.encode_to_vec(),
            MavMessage::MissionItemInt(ref body) => body.encode_to_vec(),
            MavMessage::VfrHud(ref body) => body.encode_to_vec(),
            MavMessage::CommandInt(ref body) => body.encode_to_vec(),
            MavMessage::CommandLong(ref body) => body.encode_to_vec(),
            MavMessage::CommandAck(ref body) => body.encode_to_vec(),
            MavMessage::CommandCancel(ref body) => body.encode_to_vec(),
            MavMessage::ManualSetpoint(ref body) => body.encode_to_vec(),
            MavMessage::SetAttitudeTarget(ref body) => body.encode_to_vec(),
            MavMessage::AttitudeTarget(ref body) => body.encode_to_vec(),
            MavMessage::SetPositionTargetLocalNed(ref body) => body.encode_to_vec(),
            MavMessage::PositionTargetLocalNed(ref body) => body.encode_to_vec(),
            MavMessage::SetPositionTargetGlobalInt(ref body) => body.encode_to_vec(),
            MavMessage::PositionTargetGlobalInt(ref body) => body.encode_to_vec(),
            MavMessage::LocalPositionNedSystemGlobalOffset(ref body) => body.encode_to_vec(),
            MavMessage::HilState(ref body) => body.encode_to_vec(),
            MavMessage::HilControls(ref body) => body.encode_to_vec(),
            MavMessage::HilRcInputsRaw(ref body) => body.encode_to_vec(),
            MavMessage::HilActuatorControls(ref body) => body.encode_to_vec(),
            MavMessage::OpticalFlow(ref body) => body.encode_to_vec(),
            MavMessage::GlobalVisionPositionEstimate(ref body) => body.encode_to_vec(),
            MavMessage::VisionPositionEstimate(ref body) => body.encode_to_vec(),
            MavMessage::VisionSpeedEstimate(ref body) => body.encode_to_vec(),
            MavMessage::ViconPositionEstimate(ref body) => body.encode_to_vec(),
            MavMessage::HighresImu(ref body) => body.encode_to_vec(),
            MavMessage::OpticalFlowRad(ref body) => body.encode_to_vec(),
            MavMessage::HilSensor(ref body) => body.encode_to_vec(),
            MavMessage::SimState(ref body) => body.encode_to_vec(),
            MavMessage::RadioStatus(ref body) => body.encode_to_vec(),
            MavMessage::FileTransferProtocol(ref body) => body.encode_to_vec(),
            MavMessage::Timesync(ref body) => body.encode_to_vec(),
            MavMessage::CameraTrigger(ref body) => body.encode_to_vec(),
            MavMessage::HilGps(ref body) => body.encode_to_vec(),
            MavMessage::HilOpticalFlow(ref body) => body.encode_to_vec(),
            MavMessage::HilStateQuaternion(ref body) => body.encode_to_vec(),
            MavMessage::ScaledImu2(ref body) => body.encode_to_vec(),
            MavMessage::LogRequestList(ref body) => body.encode_to_vec(),
            MavMessage::LogEntry(ref body) => body.encode_to_vec(),
            MavMessage::LogRequestData(ref body) => body.encode_to_vec(),
            MavMessage::LogData(ref body) => body.encode_to_vec(),
            MavMessage::LogErase(ref body) => body.encode_to_vec(),
            MavMessage::LogRequestEnd(ref body) => body.encode_to_vec(),
            MavMessage::GpsInjectData(ref body) => body.encode_to_vec(),
            MavMessage::Gps2Raw(ref body) => body.encode_to_vec(),
            MavMessage::PowerStatus(ref body) => body.encode_to_vec(),
            MavMessage::SerialControl(ref body) => body.encode_to_vec(),
            MavMessage::GpsRtk(ref body) => body.encode_to_vec(),
            MavMessage::Gps2Rtk(ref body) => body.encode_to_vec(),
            MavMessage::ScaledImu3(ref body) => body.encode_to_vec(),
            MavMessage::DataTransmissionHandshake(ref body) => body.encode_to_vec(),
            MavMessage::EncapsulatedData(ref body) => body.encode_to_vec(),
            MavMessage::DistanceSensor(ref body) => body.encode_to_vec(),
            MavMessage::TerrainRequest(ref body) => body.encode_to_vec(),
            MavMessage::TerrainData(ref body) => body.encode_to_vec(),
            MavMessage::TerrainCheck(ref body) => body.encode_to_vec(),
            MavMessage::TerrainReport(ref body) => body.encode_to_vec(),
            MavMessage::ScaledPressure2(ref body) => body.encode_to_vec(),
            MavMessage::AttPosMocap(ref body) => body.encode_to_vec(),
            MavMessage::SetActuatorControlTarget(ref body) => body.encode_to_vec(),
            MavMessage::ActuatorControlTarget(ref body) => body.encode_to_vec(),
            MavMessage::Altitude(ref body) => body.encode_to_vec(),
            MavMessage::ResourceRequest(ref body) => body.encode_to_vec(),
            MavMessage::ScaledPressure3(ref body) => body.encode_to_vec(),
            MavMessage::FollowTarget(ref body) => body.encode_to_vec(),
            MavMessage::ControlSystemState(ref body) => body.encode_to_vec(),
            MavMessage::BatteryStatus(ref body) => body.encode_to_vec(),
            MavMessage::AutopilotVersion(ref body) => body.encode_to_vec(),
            MavMessage::LandingTarget(ref body) => body.encode_to_vec(),
            MavMessage::FenceStatus(ref body) => body.encode_to_vec(),
            MavMessage::EstimatorStatus(ref body) => body.encode_to_vec(),
            MavMessage::WindCov(ref body) => body.encode_to_vec(),
            MavMessage::GpsInput(ref body) => body.encode_to_vec(),
            MavMessage::GpsRtcmData(ref body) => body.encode_to_vec(),
            MavMessage::HighLatency(ref body) => body.encode_to_vec(),
            MavMessage::HighLatency2(ref body) => body.encode_to_vec(),
            MavMessage::Vibration(ref body) => body.encode_to_vec(),
            MavMessage::HomePosition(ref body) => body.encode_to_vec(),
            MavMessage::SetHomePosition(ref body) => body.encode_to_vec(),
            MavMessage::MessageInterval(ref body) => body.encode_to_vec(),
            MavMessage::ExtendedSysState(ref body) => body.encode_to_vec(),
            MavMessage::AdsbVehicle(ref body) => body.encode_to_vec(),
            MavMessage::Collision(ref body) => body.encode_to_vec(),
            MavMessage::V2Extension(ref body) => body.encode_to_vec(),
            MavMessage::MemoryVect(ref body) => body.encode_to_vec(),
            MavMessage::DebugVect(ref body) => body.encode_to_vec(),
            MavMessage::NamedValueFloat(ref body) => body.encode_to_vec(),
            MavMessage::NamedValueInt(ref body) => body.encode_to_vec(),
            MavMessage::Statustext(ref body) => body.encode_to_vec(),
            MavMessage::Debug(ref body) => body.encode_to_vec(),
            MavMessage::SetupSigning(ref body) => body.encode_to_vec(),
            MavMessage::ButtonChange(ref body) => body.encode_to_vec(),
            MavMessage::PlayTune(ref body) => body.encode_to_vec(),
            MavMessage::CameraInformation(ref body) => body.encode_to_vec(),
            MavMessage::CameraSettings(ref body) => body.encode_to_vec(),
            MavMessage::StorageInformation(ref body) => body.encode_to_vec(),
            MavMessage::CameraCaptureStatus(ref body) => body.encode_to_vec(),
            MavMessage::CameraImageCaptured(ref body) => body.encode_to_vec(),
            MavMessage::FlightInformation(ref body) => body.encode_to_vec(),
            MavMessage::MountOrientation(ref body) => body.encode_to_vec(),
            MavMessage::LoggingData(ref body) => body.encode_to_vec(),
            MavMessage::LoggingDataAcked(ref body) => body.encode_to_vec(),
            MavMessage::LoggingAck(ref body) => body.encode_to_vec(),
            MavMessage::VideoStreamInformation(ref body) => body.encode_to_vec(),
            MavMessage::VideoStreamStatus(ref body) => body.encode_to_vec(),
            MavMessage::GimbalManagerInformation(ref body) => body.encode_to_vec(),
            MavMessage::GimbalManagerStatus(ref body) => body.encode_to_vec(),
            MavMessage::GimbalManagerSetAttitude(ref body) => body.encode_to_vec(),
            MavMessage::GimbalDeviceInformation(ref body) => body.encode_to_vec(),
            MavMessage::GimbalDeviceSetAttitude(ref body) => body.encode_to_vec(),
            MavMessage::GimbalDeviceAttitudeStatus(ref body) => body.encode_to_vec(),
            MavMessage::AutopilotStateForGimbalDevice(ref body) => body.encode_to_vec(),
            MavMessage::GimbalManagerSetTiltpan(ref body) => body.encode_to_vec(),
            MavMessage::WifiConfigAp(ref body) => body.encode_to_vec(),
            MavMessage::ProtocolVersion(ref body) => body.encode_to_vec(),
            MavMessage::AisVessel(ref body) => body.encode_to_vec(),
            MavMessage::UavcanNodeStatus(ref body) => body.encode_to_vec(),
            MavMessage::UavcanNodeInfo(ref body) => body.encode_to_vec(),
            MavMessage::ParamExtRequestRead(ref body) => body.encode_to_vec(),
            MavMessage::ParamExtRequestList(ref body) => body.encode_to_vec(),
            MavMessage::ParamExtValue(ref body) => body.encode_to_vec(),
            MavMessage::ParamExtSet(ref body) => body.encode_to_vec(),
            MavMessage::ParamExtAck(ref body) => body.encode_to_vec(),
            MavMessage::ObstacleDistance(ref body) => body.encode_to_vec(),
            MavMessage::Odometry(ref body) => body.encode_to_vec(),
            MavMessage::TrajectoryRepresentationWaypoints(ref body) => body.encode_to_vec(),
            MavMessage::TrajectoryRepresentationBezier(ref body) => body.encode_to_vec(),
            MavMessage::CellularStatus(ref body) => body.encode_to_vec(),
            MavMessage::IsbdLinkStatus(ref body) => body.encode_to_vec(),
            MavMessage::CellularConfig(ref body) => body.encode_to_vec(),
            MavMessage::RawRpm(ref body) => body.encode_to_vec(),
            MavMessage::UtmGlobalPosition(ref body) => body.encode_to_vec(),
            MavMessage::DebugFloatArray(ref body) => body.encode_to_vec(),
            MavMessage::OrbitExecutionStatus(ref body) => body.encode_to_vec(),
            MavMessage::SmartBatteryInfo(ref body) => body.encode_to_vec(),
            MavMessage::SmartBatteryStatus(ref body) => body.encode_to_vec(),
            MavMessage::GeneratorStatus(ref body) => body.encode_to_vec(),
            MavMessage::ActuatorOutputStatus(ref body) => body.encode_to_vec(),
            MavMessage::TimeEstimateToTarget(ref body) => body.encode_to_vec(),
            MavMessage::Tunnel(ref body) => body.encode_to_vec(),
            MavMessage::OnboardComputerStatus(ref body) => body.encode_to_vec(),
            MavMessage::ComponentInformation(ref body) => body.encode_to_vec(),
            MavMessage::PlayTuneV2(ref body) => body.encode_to_vec(),
            MavMessage::SupportedTunes(ref body) => body.encode_to_vec(),
            MavMessage::WheelDistance(ref body) => body.encode_to_vec(),
            MavMessage::OpenDroneIdBasicId(ref body) => body.encode_to_vec(),
            MavMessage::OpenDroneIdLocation(ref body) => body.encode_to_vec(),
            MavMessage::OpenDroneIdAuthentication(ref body) => body.encode_to_vec(),
            MavMessage::OpenDroneIdSelfId(ref body) => body.encode_to_vec(),
            MavMessage::OpenDroneIdSystem(ref body) => body.encode_to_vec(),
            MavMessage::OpenDroneIdOperatorId(ref body) => body.encode_to_vec(),
            MavMessage::OpenDroneIdMessagePack(ref body) => body.encode_to_vec(),
        }
    }
    fn extra_crc(id: u32) -> u8 {
        match id {
            0 => 50,
            1 => 124,
            2 => 137,
            4 => 237,
            5 => 217,
            6 => 104,
            7 => 119,
            8 => 117,
            11 => 89,
            20 => 214,
            21 => 159,
            22 => 220,
            23 => 168,
            24 => 24,
            25 => 23,
            26 => 170,
            27 => 144,
            28 => 67,
            29 => 115,
            30 => 39,
            31 => 246,
            32 => 185,
            33 => 104,
            34 => 237,
            35 => 244,
            36 => 222,
            37 => 212,
            38 => 9,
            39 => 254,
            40 => 230,
            41 => 28,
            42 => 28,
            43 => 132,
            44 => 221,
            45 => 232,
            46 => 11,
            47 => 153,
            48 => 41,
            49 => 39,
            50 => 78,
            51 => 196,
            52 => 132,
            54 => 15,
            55 => 3,
            61 => 167,
            62 => 183,
            63 => 119,
            64 => 191,
            65 => 118,
            66 => 148,
            67 => 21,
            69 => 243,
            70 => 124,
            73 => 38,
            74 => 20,
            75 => 158,
            76 => 152,
            77 => 143,
            80 => 14,
            81 => 106,
            82 => 49,
            83 => 22,
            84 => 143,
            85 => 140,
            86 => 5,
            87 => 150,
            89 => 231,
            90 => 183,
            91 => 63,
            92 => 54,
            93 => 47,
            100 => 175,
            101 => 102,
            102 => 158,
            103 => 208,
            104 => 56,
            105 => 93,
            106 => 138,
            107 => 108,
            108 => 32,
            109 => 185,
            110 => 84,
            111 => 34,
            112 => 174,
            113 => 124,
            114 => 237,
            115 => 4,
            116 => 76,
            117 => 128,
            118 => 56,
            119 => 116,
            120 => 134,
            121 => 237,
            122 => 203,
            123 => 250,
            124 => 87,
            125 => 203,
            126 => 220,
            127 => 25,
            128 => 226,
            129 => 46,
            130 => 29,
            131 => 223,
            132 => 85,
            133 => 6,
            134 => 229,
            135 => 203,
            136 => 1,
            137 => 195,
            138 => 109,
            139 => 168,
            140 => 181,
            141 => 47,
            142 => 72,
            143 => 131,
            144 => 127,
            146 => 103,
            147 => 154,
            148 => 178,
            149 => 200,
            162 => 189,
            230 => 163,
            231 => 105,
            232 => 151,
            233 => 35,
            234 => 150,
            235 => 179,
            241 => 90,
            242 => 104,
            243 => 85,
            244 => 95,
            245 => 130,
            246 => 184,
            247 => 81,
            248 => 8,
            249 => 204,
            250 => 49,
            251 => 170,
            252 => 44,
            253 => 83,
            254 => 46,
            256 => 71,
            257 => 131,
            258 => 187,
            259 => 92,
            260 => 146,
            261 => 179,
            262 => 12,
            263 => 133,
            264 => 49,
            265 => 26,
            266 => 193,
            267 => 35,
            268 => 14,
            269 => 109,
            270 => 59,
            280 => 166,
            281 => 0,
            282 => 123,
            283 => 247,
            284 => 99,
            285 => 137,
            286 => 210,
            287 => 74,
            299 => 19,
            300 => 217,
            301 => 243,
            310 => 28,
            311 => 95,
            320 => 243,
            321 => 88,
            322 => 243,
            323 => 78,
            324 => 132,
            330 => 23,
            331 => 91,
            332 => 236,
            333 => 231,
            334 => 72,
            335 => 225,
            336 => 178,
            339 => 199,
            340 => 99,
            350 => 232,
            360 => 11,
            370 => 98,
            371 => 161,
            373 => 192,
            375 => 251,
            380 => 232,
            385 => 147,
            390 => 156,
            395 => 163,
            400 => 110,
            401 => 183,
            9000 => 113,
            12900 => 114,
            12901 => 254,
            12902 => 49,
            12903 => 249,
            12904 => 203,
            12905 => 49,
            12915 => 62,
            _ => 0,
        }
    }
}
