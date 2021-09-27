/// Test all field types
///
/// MavLink id: 0
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TestTypes {
    /// uint64_t
    #[prost(uint64, tag = "1")]
    pub u64: u64,
    /// int64_t
    #[prost(int64, tag = "2")]
    pub s64: i64,
    /// double
    #[prost(double, tag = "3")]
    pub d: f64,
    /// uint64_t_array
    #[prost(uint64, repeated, packed = "false", tag = "4")]
    pub u64_array: ::prost::alloc::vec::Vec<u64>,
    /// int64_t_array
    #[prost(int64, repeated, packed = "false", tag = "5")]
    pub s64_array: ::prost::alloc::vec::Vec<i64>,
    /// double_array
    #[prost(double, repeated, packed = "false", tag = "6")]
    pub d_array: ::prost::alloc::vec::Vec<f64>,
    /// uint32_t
    #[prost(uint32, tag = "7")]
    pub u32: u32,
    /// int32_t
    #[prost(int32, tag = "8")]
    pub s32: i32,
    /// float
    #[prost(float, tag = "9")]
    pub f: f32,
    /// uint32_t_array
    #[prost(uint32, repeated, packed = "false", tag = "10")]
    pub u32_array: ::prost::alloc::vec::Vec<u32>,
    /// int32_t_array
    #[prost(int32, repeated, packed = "false", tag = "11")]
    pub s32_array: ::prost::alloc::vec::Vec<i32>,
    /// float_array
    #[prost(float, repeated, packed = "false", tag = "12")]
    pub f_array: ::prost::alloc::vec::Vec<f32>,
    /// uint16_t
    #[prost(uint32, tag = "13")]
    pub u16: u32,
    /// int16_t
    #[prost(int32, tag = "14")]
    pub s16: i32,
    /// uint16_t_array
    #[prost(uint32, repeated, packed = "false", tag = "15")]
    pub u16_array: ::prost::alloc::vec::Vec<u32>,
    /// int16_t_array
    #[prost(int32, repeated, packed = "false", tag = "16")]
    pub s16_array: ::prost::alloc::vec::Vec<i32>,
    /// char
    #[prost(uint32, tag = "17")]
    pub c: u32,
    /// string
    #[prost(string, tag = "18")]
    pub s: ::prost::alloc::string::String,
    /// uint8_t
    #[prost(uint32, tag = "19")]
    pub u8: u32,
    /// int8_t
    #[prost(int32, tag = "20")]
    pub s8: i32,
    /// uint8_t_array
    #[prost(uint32, repeated, packed = "false", tag = "21")]
    pub u8_array: ::prost::alloc::vec::Vec<u32>,
    /// int8_t_array
    #[prost(int32, repeated, packed = "false", tag = "22")]
    pub s8_array: ::prost::alloc::vec::Vec<i32>,
}
