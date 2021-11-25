#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn acosf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn asinf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn atanf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn atan2f(_: libc::c_float, _: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn cosf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn sinf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn tanf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn sqrtf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn fabsf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn Matrix3x4_FromOriginQuat(out: *mut [vec_t; 4],
                                quaternion: *const vec_t,
                                origin: *const vec_t);
    #[no_mangle]
    fn Matrix3x4_AnglesFromMatrix(in_0: *const [vec_t; 4], out: *mut vec_t);
}
pub type byte = libc::c_uchar;
pub type vec_t = libc::c_float;
pub type vec3_t = [vec_t; 3];
pub type vec4_t = [vec_t; 4];
pub type matrix3x4 = [[vec_t; 4]; 3];
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type word = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mplane_s {
    pub normal: vec3_t,
    pub dist: libc::c_float,
    pub type_0: byte,
    pub signbits: byte,
    pub pad: [byte; 2],
}
pub type mplane_t = mplane_s;
#[inline(always)]
unsafe extern "C" fn __tg_acos(mut __x: libc::c_float) -> libc::c_float {
    return acosf(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_asin(mut __x: libc::c_float) -> libc::c_float {
    return asinf(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_atan(mut __x: libc::c_float) -> libc::c_float {
    return atanf(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_cos(mut __x: libc::c_float) -> libc::c_float {
    return cosf(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_sin(mut __x: libc::c_float) -> libc::c_float {
    return sinf(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_tan(mut __x: libc::c_float) -> libc::c_float {
    return tanf(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_atan2(mut __x: libc::c_float,
                                mut __y: libc::c_float) -> libc::c_float {
    return atan2f(__x, __y);
}
#[inline(always)]
unsafe extern "C" fn __tg_fabs(mut __x: libc::c_float) -> libc::c_float {
    return fabsf(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_sqrt(mut __x: libc::c_float) -> libc::c_float {
    return sqrtf(__x);
}
#[no_mangle]
pub static mut vec3_origin: vec3_t =
    [0 as libc::c_int as vec_t, 0 as libc::c_int as vec_t,
     0 as libc::c_int as vec_t];
static mut hull_table: [word; 24] =
    [2 as libc::c_int as word, 4 as libc::c_int as word,
     6 as libc::c_int as word, 8 as libc::c_int as word,
     12 as libc::c_int as word, 16 as libc::c_int as word,
     18 as libc::c_int as word, 24 as libc::c_int as word,
     28 as libc::c_int as word, 32 as libc::c_int as word,
     36 as libc::c_int as word, 40 as libc::c_int as word,
     48 as libc::c_int as word, 54 as libc::c_int as word,
     56 as libc::c_int as word, 60 as libc::c_int as word,
     64 as libc::c_int as word, 72 as libc::c_int as word,
     80 as libc::c_int as word, 112 as libc::c_int as word,
     120 as libc::c_int as word, 128 as libc::c_int as word,
     140 as libc::c_int as word, 176 as libc::c_int as word];
#[no_mangle]
pub static mut boxpnt: [[libc::c_int; 4]; 6] =
    [[0 as libc::c_int, 4 as libc::c_int, 6 as libc::c_int, 2 as libc::c_int],
     [0 as libc::c_int, 1 as libc::c_int, 5 as libc::c_int, 4 as libc::c_int],
     [0 as libc::c_int, 2 as libc::c_int, 3 as libc::c_int, 1 as libc::c_int],
     [7 as libc::c_int, 5 as libc::c_int, 1 as libc::c_int, 3 as libc::c_int],
     [7 as libc::c_int, 3 as libc::c_int, 2 as libc::c_int, 6 as libc::c_int],
     [7 as libc::c_int, 6 as libc::c_int, 4 as libc::c_int,
      5 as libc::c_int]];
// pre-quantized table normals from Quake1
#[no_mangle]
pub static mut m_bytenormals: [[libc::c_float; 3]; 162] =
    [[-0.525731f64 as libc::c_float, 0.000000f64 as libc::c_float,
      0.850651f64 as libc::c_float],
     [-0.442863f64 as libc::c_float, 0.238856f64 as libc::c_float,
      0.864188f64 as libc::c_float],
     [-0.295242f64 as libc::c_float, 0.000000f64 as libc::c_float,
      0.955423f64 as libc::c_float],
     [-0.309017f64 as libc::c_float, 0.500000f64 as libc::c_float,
      0.809017f64 as libc::c_float],
     [-0.162460f64 as libc::c_float, 0.262866f64 as libc::c_float,
      0.951056f64 as libc::c_float],
     [0.000000f64 as libc::c_float, 0.000000f64 as libc::c_float,
      1.000000f64 as libc::c_float],
     [0.000000f64 as libc::c_float, 0.850651f64 as libc::c_float,
      0.525731f64 as libc::c_float],
     [-0.147621f64 as libc::c_float, 0.716567f64 as libc::c_float,
      0.681718f64 as libc::c_float],
     [0.147621f64 as libc::c_float, 0.716567f64 as libc::c_float,
      0.681718f64 as libc::c_float],
     [0.000000f64 as libc::c_float, 0.525731f64 as libc::c_float,
      0.850651f64 as libc::c_float],
     [0.309017f64 as libc::c_float, 0.500000f64 as libc::c_float,
      0.809017f64 as libc::c_float],
     [0.525731f64 as libc::c_float, 0.000000f64 as libc::c_float,
      0.850651f64 as libc::c_float],
     [0.295242f64 as libc::c_float, 0.000000f64 as libc::c_float,
      0.955423f64 as libc::c_float],
     [0.442863f64 as libc::c_float, 0.238856f64 as libc::c_float,
      0.864188f64 as libc::c_float],
     [0.162460f64 as libc::c_float, 0.262866f64 as libc::c_float,
      0.951056f64 as libc::c_float],
     [-0.681718f64 as libc::c_float, 0.147621f64 as libc::c_float,
      0.716567f64 as libc::c_float],
     [-0.809017f64 as libc::c_float, 0.309017f64 as libc::c_float,
      0.500000f64 as libc::c_float],
     [-0.587785f64 as libc::c_float, 0.425325f64 as libc::c_float,
      0.688191f64 as libc::c_float],
     [-0.850651f64 as libc::c_float, 0.525731f64 as libc::c_float,
      0.000000f64 as libc::c_float],
     [-0.864188f64 as libc::c_float, 0.442863f64 as libc::c_float,
      0.238856f64 as libc::c_float],
     [-0.716567f64 as libc::c_float, 0.681718f64 as libc::c_float,
      0.147621f64 as libc::c_float],
     [-0.688191f64 as libc::c_float, 0.587785f64 as libc::c_float,
      0.425325f64 as libc::c_float],
     [-0.500000f64 as libc::c_float, 0.809017f64 as libc::c_float,
      0.309017f64 as libc::c_float],
     [-0.238856f64 as libc::c_float, 0.864188f64 as libc::c_float,
      0.442863f64 as libc::c_float],
     [-0.425325f64 as libc::c_float, 0.688191f64 as libc::c_float,
      0.587785f64 as libc::c_float],
     [-0.716567f64 as libc::c_float, 0.681718f64 as libc::c_float,
      -0.147621f64 as libc::c_float],
     [-0.500000f64 as libc::c_float, 0.809017f64 as libc::c_float,
      -0.309017f64 as libc::c_float],
     [-0.525731f64 as libc::c_float, 0.850651f64 as libc::c_float,
      0.000000f64 as libc::c_float],
     [0.000000f64 as libc::c_float, 0.850651f64 as libc::c_float,
      -0.525731f64 as libc::c_float],
     [-0.238856f64 as libc::c_float, 0.864188f64 as libc::c_float,
      -0.442863f64 as libc::c_float],
     [0.000000f64 as libc::c_float, 0.955423f64 as libc::c_float,
      -0.295242f64 as libc::c_float],
     [-0.262866f64 as libc::c_float, 0.951056f64 as libc::c_float,
      -0.162460f64 as libc::c_float],
     [0.000000f64 as libc::c_float, 1.000000f64 as libc::c_float,
      0.000000f64 as libc::c_float],
     [0.000000f64 as libc::c_float, 0.955423f64 as libc::c_float,
      0.295242f64 as libc::c_float],
     [-0.262866f64 as libc::c_float, 0.951056f64 as libc::c_float,
      0.162460f64 as libc::c_float],
     [0.238856f64 as libc::c_float, 0.864188f64 as libc::c_float,
      0.442863f64 as libc::c_float],
     [0.262866f64 as libc::c_float, 0.951056f64 as libc::c_float,
      0.162460f64 as libc::c_float],
     [0.500000f64 as libc::c_float, 0.809017f64 as libc::c_float,
      0.309017f64 as libc::c_float],
     [0.238856f64 as libc::c_float, 0.864188f64 as libc::c_float,
      -0.442863f64 as libc::c_float],
     [0.262866f64 as libc::c_float, 0.951056f64 as libc::c_float,
      -0.162460f64 as libc::c_float],
     [0.500000f64 as libc::c_float, 0.809017f64 as libc::c_float,
      -0.309017f64 as libc::c_float],
     [0.850651f64 as libc::c_float, 0.525731f64 as libc::c_float,
      0.000000f64 as libc::c_float],
     [0.716567f64 as libc::c_float, 0.681718f64 as libc::c_float,
      0.147621f64 as libc::c_float],
     [0.716567f64 as libc::c_float, 0.681718f64 as libc::c_float,
      -0.147621f64 as libc::c_float],
     [0.525731f64 as libc::c_float, 0.850651f64 as libc::c_float,
      0.000000f64 as libc::c_float],
     [0.425325f64 as libc::c_float, 0.688191f64 as libc::c_float,
      0.587785f64 as libc::c_float],
     [0.864188f64 as libc::c_float, 0.442863f64 as libc::c_float,
      0.238856f64 as libc::c_float],
     [0.688191f64 as libc::c_float, 0.587785f64 as libc::c_float,
      0.425325f64 as libc::c_float],
     [0.809017f64 as libc::c_float, 0.309017f64 as libc::c_float,
      0.500000f64 as libc::c_float],
     [0.681718f64 as libc::c_float, 0.147621f64 as libc::c_float,
      0.716567f64 as libc::c_float],
     [0.587785f64 as libc::c_float, 0.425325f64 as libc::c_float,
      0.688191f64 as libc::c_float],
     [0.955423f64 as libc::c_float, 0.295242f64 as libc::c_float,
      0.000000f64 as libc::c_float],
     [1.000000f64 as libc::c_float, 0.000000f64 as libc::c_float,
      0.000000f64 as libc::c_float],
     [0.951056f64 as libc::c_float, 0.162460f64 as libc::c_float,
      0.262866f64 as libc::c_float],
     [0.850651f64 as libc::c_float, -0.525731f64 as libc::c_float,
      0.000000f64 as libc::c_float],
     [0.955423f64 as libc::c_float, -0.295242f64 as libc::c_float,
      0.000000f64 as libc::c_float],
     [0.864188f64 as libc::c_float, -0.442863f64 as libc::c_float,
      0.238856f64 as libc::c_float],
     [0.951056f64 as libc::c_float, -0.162460f64 as libc::c_float,
      0.262866f64 as libc::c_float],
     [0.809017f64 as libc::c_float, -0.309017f64 as libc::c_float,
      0.500000f64 as libc::c_float],
     [0.681718f64 as libc::c_float, -0.147621f64 as libc::c_float,
      0.716567f64 as libc::c_float],
     [0.850651f64 as libc::c_float, 0.000000f64 as libc::c_float,
      0.525731f64 as libc::c_float],
     [0.864188f64 as libc::c_float, 0.442863f64 as libc::c_float,
      -0.238856f64 as libc::c_float],
     [0.809017f64 as libc::c_float, 0.309017f64 as libc::c_float,
      -0.500000f64 as libc::c_float],
     [0.951056f64 as libc::c_float, 0.162460f64 as libc::c_float,
      -0.262866f64 as libc::c_float],
     [0.525731f64 as libc::c_float, 0.000000f64 as libc::c_float,
      -0.850651f64 as libc::c_float],
     [0.681718f64 as libc::c_float, 0.147621f64 as libc::c_float,
      -0.716567f64 as libc::c_float],
     [0.681718f64 as libc::c_float, -0.147621f64 as libc::c_float,
      -0.716567f64 as libc::c_float],
     [0.850651f64 as libc::c_float, 0.000000f64 as libc::c_float,
      -0.525731f64 as libc::c_float],
     [0.809017f64 as libc::c_float, -0.309017f64 as libc::c_float,
      -0.500000f64 as libc::c_float],
     [0.864188f64 as libc::c_float, -0.442863f64 as libc::c_float,
      -0.238856f64 as libc::c_float],
     [0.951056f64 as libc::c_float, -0.162460f64 as libc::c_float,
      -0.262866f64 as libc::c_float],
     [0.147621f64 as libc::c_float, 0.716567f64 as libc::c_float,
      -0.681718f64 as libc::c_float],
     [0.309017f64 as libc::c_float, 0.500000f64 as libc::c_float,
      -0.809017f64 as libc::c_float],
     [0.425325f64 as libc::c_float, 0.688191f64 as libc::c_float,
      -0.587785f64 as libc::c_float],
     [0.442863f64 as libc::c_float, 0.238856f64 as libc::c_float,
      -0.864188f64 as libc::c_float],
     [0.587785f64 as libc::c_float, 0.425325f64 as libc::c_float,
      -0.688191f64 as libc::c_float],
     [0.688191f64 as libc::c_float, 0.587785f64 as libc::c_float,
      -0.425325f64 as libc::c_float],
     [-0.147621f64 as libc::c_float, 0.716567f64 as libc::c_float,
      -0.681718f64 as libc::c_float],
     [-0.309017f64 as libc::c_float, 0.500000f64 as libc::c_float,
      -0.809017f64 as libc::c_float],
     [0.000000f64 as libc::c_float, 0.525731f64 as libc::c_float,
      -0.850651f64 as libc::c_float],
     [-0.525731f64 as libc::c_float, 0.000000f64 as libc::c_float,
      -0.850651f64 as libc::c_float],
     [-0.442863f64 as libc::c_float, 0.238856f64 as libc::c_float,
      -0.864188f64 as libc::c_float],
     [-0.295242f64 as libc::c_float, 0.000000f64 as libc::c_float,
      -0.955423f64 as libc::c_float],
     [-0.162460f64 as libc::c_float, 0.262866f64 as libc::c_float,
      -0.951056f64 as libc::c_float],
     [0.000000f64 as libc::c_float, 0.000000f64 as libc::c_float,
      -1.000000f64 as libc::c_float],
     [0.295242f64 as libc::c_float, 0.000000f64 as libc::c_float,
      -0.955423f64 as libc::c_float],
     [0.162460f64 as libc::c_float, 0.262866f64 as libc::c_float,
      -0.951056f64 as libc::c_float],
     [-0.442863f64 as libc::c_float, -0.238856f64 as libc::c_float,
      -0.864188f64 as libc::c_float],
     [-0.309017f64 as libc::c_float, -0.500000f64 as libc::c_float,
      -0.809017f64 as libc::c_float],
     [-0.162460f64 as libc::c_float, -0.262866f64 as libc::c_float,
      -0.951056f64 as libc::c_float],
     [0.000000f64 as libc::c_float, -0.850651f64 as libc::c_float,
      -0.525731f64 as libc::c_float],
     [-0.147621f64 as libc::c_float, -0.716567f64 as libc::c_float,
      -0.681718f64 as libc::c_float],
     [0.147621f64 as libc::c_float, -0.716567f64 as libc::c_float,
      -0.681718f64 as libc::c_float],
     [0.000000f64 as libc::c_float, -0.525731f64 as libc::c_float,
      -0.850651f64 as libc::c_float],
     [0.309017f64 as libc::c_float, -0.500000f64 as libc::c_float,
      -0.809017f64 as libc::c_float],
     [0.442863f64 as libc::c_float, -0.238856f64 as libc::c_float,
      -0.864188f64 as libc::c_float],
     [0.162460f64 as libc::c_float, -0.262866f64 as libc::c_float,
      -0.951056f64 as libc::c_float],
     [0.238856f64 as libc::c_float, -0.864188f64 as libc::c_float,
      -0.442863f64 as libc::c_float],
     [0.500000f64 as libc::c_float, -0.809017f64 as libc::c_float,
      -0.309017f64 as libc::c_float],
     [0.425325f64 as libc::c_float, -0.688191f64 as libc::c_float,
      -0.587785f64 as libc::c_float],
     [0.716567f64 as libc::c_float, -0.681718f64 as libc::c_float,
      -0.147621f64 as libc::c_float],
     [0.688191f64 as libc::c_float, -0.587785f64 as libc::c_float,
      -0.425325f64 as libc::c_float],
     [0.587785f64 as libc::c_float, -0.425325f64 as libc::c_float,
      -0.688191f64 as libc::c_float],
     [0.000000f64 as libc::c_float, -0.955423f64 as libc::c_float,
      -0.295242f64 as libc::c_float],
     [0.000000f64 as libc::c_float, -1.000000f64 as libc::c_float,
      0.000000f64 as libc::c_float],
     [0.262866f64 as libc::c_float, -0.951056f64 as libc::c_float,
      -0.162460f64 as libc::c_float],
     [0.000000f64 as libc::c_float, -0.850651f64 as libc::c_float,
      0.525731f64 as libc::c_float],
     [0.000000f64 as libc::c_float, -0.955423f64 as libc::c_float,
      0.295242f64 as libc::c_float],
     [0.238856f64 as libc::c_float, -0.864188f64 as libc::c_float,
      0.442863f64 as libc::c_float],
     [0.262866f64 as libc::c_float, -0.951056f64 as libc::c_float,
      0.162460f64 as libc::c_float],
     [0.500000f64 as libc::c_float, -0.809017f64 as libc::c_float,
      0.309017f64 as libc::c_float],
     [0.716567f64 as libc::c_float, -0.681718f64 as libc::c_float,
      0.147621f64 as libc::c_float],
     [0.525731f64 as libc::c_float, -0.850651f64 as libc::c_float,
      0.000000f64 as libc::c_float],
     [-0.238856f64 as libc::c_float, -0.864188f64 as libc::c_float,
      -0.442863f64 as libc::c_float],
     [-0.500000f64 as libc::c_float, -0.809017f64 as libc::c_float,
      -0.309017f64 as libc::c_float],
     [-0.262866f64 as libc::c_float, -0.951056f64 as libc::c_float,
      -0.162460f64 as libc::c_float],
     [-0.850651f64 as libc::c_float, -0.525731f64 as libc::c_float,
      0.000000f64 as libc::c_float],
     [-0.716567f64 as libc::c_float, -0.681718f64 as libc::c_float,
      -0.147621f64 as libc::c_float],
     [-0.716567f64 as libc::c_float, -0.681718f64 as libc::c_float,
      0.147621f64 as libc::c_float],
     [-0.525731f64 as libc::c_float, -0.850651f64 as libc::c_float,
      0.000000f64 as libc::c_float],
     [-0.500000f64 as libc::c_float, -0.809017f64 as libc::c_float,
      0.309017f64 as libc::c_float],
     [-0.238856f64 as libc::c_float, -0.864188f64 as libc::c_float,
      0.442863f64 as libc::c_float],
     [-0.262866f64 as libc::c_float, -0.951056f64 as libc::c_float,
      0.162460f64 as libc::c_float],
     [-0.864188f64 as libc::c_float, -0.442863f64 as libc::c_float,
      0.238856f64 as libc::c_float],
     [-0.809017f64 as libc::c_float, -0.309017f64 as libc::c_float,
      0.500000f64 as libc::c_float],
     [-0.688191f64 as libc::c_float, -0.587785f64 as libc::c_float,
      0.425325f64 as libc::c_float],
     [-0.681718f64 as libc::c_float, -0.147621f64 as libc::c_float,
      0.716567f64 as libc::c_float],
     [-0.442863f64 as libc::c_float, -0.238856f64 as libc::c_float,
      0.864188f64 as libc::c_float],
     [-0.587785f64 as libc::c_float, -0.425325f64 as libc::c_float,
      0.688191f64 as libc::c_float],
     [-0.309017f64 as libc::c_float, -0.500000f64 as libc::c_float,
      0.809017f64 as libc::c_float],
     [-0.147621f64 as libc::c_float, -0.716567f64 as libc::c_float,
      0.681718f64 as libc::c_float],
     [-0.425325f64 as libc::c_float, -0.688191f64 as libc::c_float,
      0.587785f64 as libc::c_float],
     [-0.162460f64 as libc::c_float, -0.262866f64 as libc::c_float,
      0.951056f64 as libc::c_float],
     [0.442863f64 as libc::c_float, -0.238856f64 as libc::c_float,
      0.864188f64 as libc::c_float],
     [0.162460f64 as libc::c_float, -0.262866f64 as libc::c_float,
      0.951056f64 as libc::c_float],
     [0.309017f64 as libc::c_float, -0.500000f64 as libc::c_float,
      0.809017f64 as libc::c_float],
     [0.147621f64 as libc::c_float, -0.716567f64 as libc::c_float,
      0.681718f64 as libc::c_float],
     [0.000000f64 as libc::c_float, -0.525731f64 as libc::c_float,
      0.850651f64 as libc::c_float],
     [0.425325f64 as libc::c_float, -0.688191f64 as libc::c_float,
      0.587785f64 as libc::c_float],
     [0.587785f64 as libc::c_float, -0.425325f64 as libc::c_float,
      0.688191f64 as libc::c_float],
     [0.688191f64 as libc::c_float, -0.587785f64 as libc::c_float,
      0.425325f64 as libc::c_float],
     [-0.955423f64 as libc::c_float, 0.295242f64 as libc::c_float,
      0.000000f64 as libc::c_float],
     [-0.951056f64 as libc::c_float, 0.162460f64 as libc::c_float,
      0.262866f64 as libc::c_float],
     [-1.000000f64 as libc::c_float, 0.000000f64 as libc::c_float,
      0.000000f64 as libc::c_float],
     [-0.850651f64 as libc::c_float, 0.000000f64 as libc::c_float,
      0.525731f64 as libc::c_float],
     [-0.955423f64 as libc::c_float, -0.295242f64 as libc::c_float,
      0.000000f64 as libc::c_float],
     [-0.951056f64 as libc::c_float, -0.162460f64 as libc::c_float,
      0.262866f64 as libc::c_float],
     [-0.864188f64 as libc::c_float, 0.442863f64 as libc::c_float,
      -0.238856f64 as libc::c_float],
     [-0.951056f64 as libc::c_float, 0.162460f64 as libc::c_float,
      -0.262866f64 as libc::c_float],
     [-0.809017f64 as libc::c_float, 0.309017f64 as libc::c_float,
      -0.500000f64 as libc::c_float],
     [-0.864188f64 as libc::c_float, -0.442863f64 as libc::c_float,
      -0.238856f64 as libc::c_float],
     [-0.951056f64 as libc::c_float, -0.162460f64 as libc::c_float,
      -0.262866f64 as libc::c_float],
     [-0.809017f64 as libc::c_float, -0.309017f64 as libc::c_float,
      -0.500000f64 as libc::c_float],
     [-0.681718f64 as libc::c_float, 0.147621f64 as libc::c_float,
      -0.716567f64 as libc::c_float],
     [-0.681718f64 as libc::c_float, -0.147621f64 as libc::c_float,
      -0.716567f64 as libc::c_float],
     [-0.850651f64 as libc::c_float, 0.000000f64 as libc::c_float,
      -0.525731f64 as libc::c_float],
     [-0.688191f64 as libc::c_float, 0.587785f64 as libc::c_float,
      -0.425325f64 as libc::c_float],
     [-0.587785f64 as libc::c_float, 0.425325f64 as libc::c_float,
      -0.688191f64 as libc::c_float],
     [-0.425325f64 as libc::c_float, 0.688191f64 as libc::c_float,
      -0.587785f64 as libc::c_float],
     [-0.425325f64 as libc::c_float, -0.688191f64 as libc::c_float,
      -0.587785f64 as libc::c_float],
     [-0.587785f64 as libc::c_float, -0.425325f64 as libc::c_float,
      -0.688191f64 as libc::c_float],
     [-0.688191f64 as libc::c_float, -0.587785f64 as libc::c_float,
      -0.425325f64 as libc::c_float]];
/*
=================
anglemod
=================
*/
#[no_mangle]
pub unsafe extern "C" fn anglemod(mut a: libc::c_float) -> libc::c_float {
    a =
        360.0f32 / 65536 as libc::c_int as libc::c_float *
            ((a * (65536 as libc::c_int as libc::c_float / 360.0f32)) as
                 libc::c_int & 65535 as libc::c_int) as libc::c_float;
    return a;
}
/*
=================
SimpleSpline

NOTE: ripped from hl2 source
hermite basis function for smooth interpolation
Similar to Gain() above, but very cheap to call
value should be between 0 & 1 inclusive
=================
*/
#[no_mangle]
pub unsafe extern "C" fn SimpleSpline(mut value: libc::c_float)
 -> libc::c_float {
    let mut valueSquared: libc::c_float = value * value;
    // nice little ease-in, ease-out spline-like curve
    return 3.0f32 * valueSquared - 2.0f32 * valueSquared * value;
}
#[no_mangle]
pub unsafe extern "C" fn FloatToHalf(mut v: libc::c_float) -> word {
    let mut i: libc::c_uint =
        *(&mut v as *mut libc::c_float as *mut libc::c_uint);
    let mut e: libc::c_uint =
        i >> 23 as libc::c_int & 0xff as libc::c_int as libc::c_uint;
    let mut m: libc::c_uint = i & 0x7fffff as libc::c_int as libc::c_uint;
    let mut h: libc::c_ushort = 0;
    if e <= (127 as libc::c_int - 15 as libc::c_int) as libc::c_uint {
        h =
            ((m | 0x800000 as libc::c_int as libc::c_uint) >>
                 ((127 as libc::c_int - 14 as libc::c_int) as
                      libc::c_uint).wrapping_sub(e) >> 13 as libc::c_int) as
                libc::c_ushort
    } else {
        h =
            (i >> 13 as libc::c_int & 0x3fff as libc::c_int as libc::c_uint)
                as libc::c_ushort
    }
    h =
        (h as libc::c_uint |
             i >> 16 as libc::c_int & 0xc000 as libc::c_int as libc::c_uint)
            as libc::c_ushort;
    return h;
}
#[no_mangle]
pub unsafe extern "C" fn HalfToFloat(mut h: word) -> libc::c_float {
    let mut f: libc::c_uint =
        ((h as libc::c_int) << 16 as libc::c_int) as libc::c_uint &
            0x80000000 as libc::c_uint;
    let mut em: libc::c_uint =
        (h as libc::c_int & 0x7fff as libc::c_int) as libc::c_uint;
    if em > 0x3ff as libc::c_int as libc::c_uint {
        f |=
            (em <<
                 13 as
                     libc::c_int).wrapping_add(((127 as libc::c_int -
                                                     15 as libc::c_int) <<
                                                    23 as libc::c_int) as
                                                   libc::c_uint)
    } else {
        let mut m: libc::c_uint = em & 0x3ff as libc::c_int as libc::c_uint;
        if m != 0 as libc::c_int as libc::c_uint {
            let mut e: libc::c_uint =
                em >> 10 as libc::c_int & 0x1f as libc::c_int as libc::c_uint;
            while m & 0x400 as libc::c_int as libc::c_uint ==
                      0 as libc::c_int as libc::c_uint {
                m <<= 1 as libc::c_int;
                e = e.wrapping_sub(1)
            }
            m &= 0x3ff as libc::c_int as libc::c_uint;
            f |=
                e.wrapping_add((127 as libc::c_int - 14 as libc::c_int) as
                                   libc::c_uint) << 23 as libc::c_int |
                    m << 13 as libc::c_int
        }
    }
    return *(&mut f as *mut libc::c_uint as *mut libc::c_float);
}
/*
=================
RoundUpHullSize

round the hullsize to nearest 'right' value
=================
*/
#[no_mangle]
pub unsafe extern "C" fn RoundUpHullSize(mut size: *mut vec_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        let mut negative: qboolean = false_0;
        let mut result: libc::c_float = 0.;
        let mut value: libc::c_float = 0.;
        value = *size.offset(i as isize);
        if value < 0.0f32 { negative = true_0 }
        value =
            (__tg_fabs(value) + 1 as libc::c_int as libc::c_float) as
                libc::c_int as libc::c_float;
        result =
            (*size.offset(i as isize) + 1 as libc::c_int as libc::c_float) as
                libc::c_int as libc::c_float;
        // lookup hull table to find nearest supposed value
        j = 0 as libc::c_int; // ceil only
        while (j as libc::c_ulong) <
                  (::std::mem::size_of::<[word; 24]>() as
                       libc::c_ulong).wrapping_div(::std::mem::size_of::<word>()
                                                       as libc::c_ulong) {
            if !(value >
                     hull_table[j as usize] as libc::c_int as libc::c_float) {
                if negative as u64 != 0 {
                    result =
                        value -
                            hull_table[j as usize] as libc::c_int as
                                libc::c_float;
                    if result <= 4 as libc::c_int as libc::c_float {
                        result =
                            -(hull_table[j as usize] as libc::c_int) as
                                libc::c_float;
                        break ;
                    }
                } else {
                    result =
                        value -
                            hull_table[j as usize] as libc::c_int as
                                libc::c_float;
                    if result <= 4 as libc::c_int as libc::c_float {
                        result = hull_table[j as usize] as libc::c_float;
                        break ;
                    }
                }
            }
            j += 1
        }
        *size.offset(i as isize) = result;
        i += 1
    };
}
/*
=================
SignbitsForPlane

fast box on planeside test
=================
*/
#[no_mangle]
pub unsafe extern "C" fn SignbitsForPlane(mut normal: *const vec_t)
 -> libc::c_int {
    let mut bits: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    bits = i;
    while i < 3 as libc::c_int {
        if *normal.offset(i as isize) < 0.0f32 {
            bits |= (1 as libc::c_int) << i
        }
        i += 1
    }
    return bits;
}
/*
=================
PlaneTypeForNormal
=================
*/
#[no_mangle]
pub unsafe extern "C" fn PlaneTypeForNormal(mut normal: *const vec_t)
 -> libc::c_int {
    if *normal.offset(0 as libc::c_int as isize) == 1.0f32 {
        return 0 as libc::c_int
    }
    if *normal.offset(1 as libc::c_int as isize) == 1.0f32 {
        return 1 as libc::c_int
    }
    if *normal.offset(2 as libc::c_int as isize) == 1.0f32 {
        return 2 as libc::c_int
    }
    return 3 as libc::c_int;
}
/*
=================
PlanesGetIntersectionPoint

=================
*/
#[no_mangle]
pub unsafe extern "C" fn PlanesGetIntersectionPoint(mut plane1:
                                                        *const mplane_t,
                                                    mut plane2:
                                                        *const mplane_t,
                                                    mut plane3:
                                                        *const mplane_t,
                                                    mut out: *mut vec_t)
 -> qboolean {
    let mut n1: vec3_t = [0.; 3];
    let mut n2: vec3_t = [0.; 3];
    let mut n3: vec3_t = [0.; 3];
    let mut n1n2: vec3_t = [0.; 3];
    let mut n2n3: vec3_t = [0.; 3];
    let mut n3n1: vec3_t = [0.; 3];
    let mut denom: libc::c_float = 0.;
    let mut ilength: libc::c_float =
        __tg_sqrt((*plane1).normal[0 as libc::c_int as usize] *
                      (*plane1).normal[0 as libc::c_int as usize] +
                      (*plane1).normal[1 as libc::c_int as usize] *
                          (*plane1).normal[1 as libc::c_int as usize] +
                      (*plane1).normal[2 as libc::c_int as usize] *
                          (*plane1).normal[2 as libc::c_int as usize]);
    if ilength != 0. { ilength = 1.0f32 / ilength }
    n1[0 as libc::c_int as usize] =
        (*plane1).normal[0 as libc::c_int as usize] * ilength;
    n1[1 as libc::c_int as usize] =
        (*plane1).normal[1 as libc::c_int as usize] * ilength;
    n1[2 as libc::c_int as usize] =
        (*plane1).normal[2 as libc::c_int as usize] * ilength;
    let mut ilength_0: libc::c_float =
        __tg_sqrt((*plane2).normal[0 as libc::c_int as usize] *
                      (*plane2).normal[0 as libc::c_int as usize] +
                      (*plane2).normal[1 as libc::c_int as usize] *
                          (*plane2).normal[1 as libc::c_int as usize] +
                      (*plane2).normal[2 as libc::c_int as usize] *
                          (*plane2).normal[2 as libc::c_int as usize]);
    if ilength_0 != 0. { ilength_0 = 1.0f32 / ilength_0 }
    n2[0 as libc::c_int as usize] =
        (*plane2).normal[0 as libc::c_int as usize] * ilength_0;
    n2[1 as libc::c_int as usize] =
        (*plane2).normal[1 as libc::c_int as usize] * ilength_0;
    n2[2 as libc::c_int as usize] =
        (*plane2).normal[2 as libc::c_int as usize] * ilength_0;
    let mut ilength_1: libc::c_float =
        __tg_sqrt((*plane3).normal[0 as libc::c_int as usize] *
                      (*plane3).normal[0 as libc::c_int as usize] +
                      (*plane3).normal[1 as libc::c_int as usize] *
                          (*plane3).normal[1 as libc::c_int as usize] +
                      (*plane3).normal[2 as libc::c_int as usize] *
                          (*plane3).normal[2 as libc::c_int as usize]);
    if ilength_1 != 0. { ilength_1 = 1.0f32 / ilength_1 }
    n3[0 as libc::c_int as usize] =
        (*plane3).normal[0 as libc::c_int as usize] * ilength_1;
    n3[1 as libc::c_int as usize] =
        (*plane3).normal[1 as libc::c_int as usize] * ilength_1;
    n3[2 as libc::c_int as usize] =
        (*plane3).normal[2 as libc::c_int as usize] * ilength_1;
    n1n2[0 as libc::c_int as usize] =
        n1[1 as libc::c_int as usize] * n2[2 as libc::c_int as usize] -
            n1[2 as libc::c_int as usize] * n2[1 as libc::c_int as usize];
    n1n2[1 as libc::c_int as usize] =
        n1[2 as libc::c_int as usize] * n2[0 as libc::c_int as usize] -
            n1[0 as libc::c_int as usize] * n2[2 as libc::c_int as usize];
    n1n2[2 as libc::c_int as usize] =
        n1[0 as libc::c_int as usize] * n2[1 as libc::c_int as usize] -
            n1[1 as libc::c_int as usize] * n2[0 as libc::c_int as usize];
    n2n3[0 as libc::c_int as usize] =
        n2[1 as libc::c_int as usize] * n3[2 as libc::c_int as usize] -
            n2[2 as libc::c_int as usize] * n3[1 as libc::c_int as usize];
    n2n3[1 as libc::c_int as usize] =
        n2[2 as libc::c_int as usize] * n3[0 as libc::c_int as usize] -
            n2[0 as libc::c_int as usize] * n3[2 as libc::c_int as usize];
    n2n3[2 as libc::c_int as usize] =
        n2[0 as libc::c_int as usize] * n3[1 as libc::c_int as usize] -
            n2[1 as libc::c_int as usize] * n3[0 as libc::c_int as usize];
    n3n1[0 as libc::c_int as usize] =
        n3[1 as libc::c_int as usize] * n1[2 as libc::c_int as usize] -
            n3[2 as libc::c_int as usize] * n1[1 as libc::c_int as usize];
    n3n1[1 as libc::c_int as usize] =
        n3[2 as libc::c_int as usize] * n1[0 as libc::c_int as usize] -
            n3[0 as libc::c_int as usize] * n1[2 as libc::c_int as usize];
    n3n1[2 as libc::c_int as usize] =
        n3[0 as libc::c_int as usize] * n1[1 as libc::c_int as usize] -
            n3[1 as libc::c_int as usize] * n1[0 as libc::c_int as usize];
    denom =
        n1[0 as libc::c_int as usize] * n2n3[0 as libc::c_int as usize] +
            n1[1 as libc::c_int as usize] * n2n3[1 as libc::c_int as usize] +
            n1[2 as libc::c_int as usize] * n2n3[2 as libc::c_int as usize];
    let ref mut fresh0 = *out.offset(2 as libc::c_int as isize);
    *fresh0 = 0 as libc::c_int as vec_t;
    let ref mut fresh1 = *out.offset(1 as libc::c_int as isize);
    *fresh1 = *fresh0;
    *out.offset(0 as libc::c_int as isize) = *fresh1;
    // check if the denominator is zero (which would mean that no intersection is to be found
    if denom == 0.0f32 {
        // no intersection could be found, return <0,0,0>
        return false_0
    }
    // compute intersection point
    *out.offset(0 as libc::c_int as isize) =
        *out.offset(0 as libc::c_int as isize) +
            (*plane1).dist * n2n3[0 as libc::c_int as usize];
    *out.offset(1 as libc::c_int as isize) =
        *out.offset(1 as libc::c_int as isize) +
            (*plane1).dist * n2n3[1 as libc::c_int as usize];
    *out.offset(2 as libc::c_int as isize) =
        *out.offset(2 as libc::c_int as isize) +
            (*plane1).dist * n2n3[2 as libc::c_int as usize];
    *out.offset(0 as libc::c_int as isize) =
        *out.offset(0 as libc::c_int as isize) +
            (*plane2).dist * n3n1[0 as libc::c_int as usize];
    *out.offset(1 as libc::c_int as isize) =
        *out.offset(1 as libc::c_int as isize) +
            (*plane2).dist * n3n1[1 as libc::c_int as usize];
    *out.offset(2 as libc::c_int as isize) =
        *out.offset(2 as libc::c_int as isize) +
            (*plane2).dist * n3n1[2 as libc::c_int as usize];
    *out.offset(0 as libc::c_int as isize) =
        *out.offset(0 as libc::c_int as isize) +
            (*plane3).dist * n1n2[0 as libc::c_int as usize];
    *out.offset(1 as libc::c_int as isize) =
        *out.offset(1 as libc::c_int as isize) +
            (*plane3).dist * n1n2[1 as libc::c_int as usize];
    *out.offset(2 as libc::c_int as isize) =
        *out.offset(2 as libc::c_int as isize) +
            (*plane3).dist * n1n2[2 as libc::c_int as usize];
    *out.offset(0 as libc::c_int as isize) =
        *out.offset(0 as libc::c_int as isize) * (1.0f32 / denom);
    *out.offset(1 as libc::c_int as isize) =
        *out.offset(1 as libc::c_int as isize) * (1.0f32 / denom);
    *out.offset(2 as libc::c_int as isize) =
        *out.offset(2 as libc::c_int as isize) * (1.0f32 / denom);
    return true_0;
}
/*
=================
NearestPOW
=================
*/
#[no_mangle]
pub unsafe extern "C" fn NearestPOW(mut value: libc::c_int,
                                    mut roundDown: qboolean) -> libc::c_int {
    let mut n: libc::c_int = 1 as libc::c_int;
    if value <= 0 as libc::c_int { return 1 as libc::c_int }
    while n < value { n <<= 1 as libc::c_int }
    if roundDown as u64 != 0 { if n > value { n >>= 1 as libc::c_int } }
    return n;
}
// remap a value in the range [A,B] to [C,D].
#[no_mangle]
pub unsafe extern "C" fn RemapVal(mut val: libc::c_float,
                                  mut A: libc::c_float, mut B: libc::c_float,
                                  mut C: libc::c_float, mut D: libc::c_float)
 -> libc::c_float {
    return C + (D - C) * (val - A) / (B - A);
}
#[no_mangle]
pub unsafe extern "C" fn ApproachVal(mut target: libc::c_float,
                                     mut value: libc::c_float,
                                     mut speed: libc::c_float)
 -> libc::c_float {
    let mut delta: libc::c_float = target - value;
    if delta > speed {
        value += speed
    } else if delta < -speed { value -= speed } else { value = target }
    return value;
}
/*
=================
rsqrt
=================
*/
#[no_mangle]
pub unsafe extern "C" fn rsqrt(mut number: libc::c_float) -> libc::c_float {
    let mut i: libc::c_int = 0; // evil floating point bit level hacking
    let mut x: libc::c_float = 0.; // what the fuck?
    let mut y: libc::c_float = 0.; // first iteration
    if number == 0.0f32 { return 0.0f32 }
    x = number * 0.5f32;
    i = *(&mut number as *mut libc::c_float as *mut libc::c_int);
    i = 0x5f3759df as libc::c_int - (i >> 1 as libc::c_int);
    y = *(&mut i as *mut libc::c_int as *mut libc::c_float);
    y = y * (1.5f32 - x * y * y);
    return y;
}
/*
=================
SinCos
=================
*/
#[no_mangle]
pub unsafe extern "C" fn SinCos(mut radians: libc::c_float,
                                mut sine: *mut libc::c_float,
                                mut cosine: *mut libc::c_float) {
    *sine = __tg_sin(radians);
    *cosine = __tg_cos(radians);
}
/*
==============
VectorCompareEpsilon

==============
*/
#[no_mangle]
pub unsafe extern "C" fn VectorCompareEpsilon(mut vec1: *const vec_t,
                                              mut vec2: *const vec_t,
                                              mut epsilon: vec_t)
 -> qboolean {
    let mut ax: vec_t = 0.;
    let mut ay: vec_t = 0.;
    let mut az: vec_t = 0.;
    ax =
        __tg_fabs(*vec1.offset(0 as libc::c_int as isize) -
                      *vec2.offset(0 as libc::c_int as isize));
    ay =
        __tg_fabs(*vec1.offset(1 as libc::c_int as isize) -
                      *vec2.offset(1 as libc::c_int as isize));
    az =
        __tg_fabs(*vec1.offset(2 as libc::c_int as isize) -
                      *vec2.offset(2 as libc::c_int as isize));
    if ax <= epsilon && ay <= epsilon && az <= epsilon { return true_0 }
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn VectorNormalizeLength2(mut v: *const vec_t,
                                                mut out: *mut vec_t)
 -> libc::c_float {
    let mut length: libc::c_float = 0.;
    let mut ilength: libc::c_float = 0.;
    length =
        *v.offset(0 as libc::c_int as isize) *
            *v.offset(0 as libc::c_int as isize) +
            *v.offset(1 as libc::c_int as isize) *
                *v.offset(1 as libc::c_int as isize) +
            *v.offset(2 as libc::c_int as isize) *
                *v.offset(2 as libc::c_int as isize);
    length = __tg_sqrt(length);
    if length != 0. {
        ilength = 1.0f32 / length;
        *out.offset(0 as libc::c_int as isize) =
            *v.offset(0 as libc::c_int as isize) * ilength;
        *out.offset(1 as libc::c_int as isize) =
            *v.offset(1 as libc::c_int as isize) * ilength;
        *out.offset(2 as libc::c_int as isize) =
            *v.offset(2 as libc::c_int as isize) * ilength
    }
    return length;
}
#[no_mangle]
pub unsafe extern "C" fn VectorVectors(mut forward: *const vec_t,
                                       mut right: *mut vec_t,
                                       mut up: *mut vec_t) {
    let mut d: libc::c_float = 0.;
    *right.offset(0 as libc::c_int as isize) =
        *forward.offset(2 as libc::c_int as isize);
    *right.offset(1 as libc::c_int as isize) =
        -*forward.offset(0 as libc::c_int as isize);
    *right.offset(2 as libc::c_int as isize) =
        *forward.offset(1 as libc::c_int as isize);
    d =
        *forward.offset(0 as libc::c_int as isize) *
            *right.offset(0 as libc::c_int as isize) +
            *forward.offset(1 as libc::c_int as isize) *
                *right.offset(1 as libc::c_int as isize) +
            *forward.offset(2 as libc::c_int as isize) *
                *right.offset(2 as libc::c_int as isize);
    *right.offset(0 as libc::c_int as isize) =
        *right.offset(0 as libc::c_int as isize) +
            -d * *forward.offset(0 as libc::c_int as isize);
    *right.offset(1 as libc::c_int as isize) =
        *right.offset(1 as libc::c_int as isize) +
            -d * *forward.offset(1 as libc::c_int as isize);
    *right.offset(2 as libc::c_int as isize) =
        *right.offset(2 as libc::c_int as isize) +
            -d * *forward.offset(2 as libc::c_int as isize);
    let mut ilength: libc::c_float =
        __tg_sqrt(*right.offset(0 as libc::c_int as isize) *
                      *right.offset(0 as libc::c_int as isize) +
                      *right.offset(1 as libc::c_int as isize) *
                          *right.offset(1 as libc::c_int as isize) +
                      *right.offset(2 as libc::c_int as isize) *
                          *right.offset(2 as libc::c_int as isize));
    if ilength != 0. { ilength = 1.0f32 / ilength }
    let ref mut fresh2 = *right.offset(0 as libc::c_int as isize);
    *fresh2 *= ilength;
    let ref mut fresh3 = *right.offset(1 as libc::c_int as isize);
    *fresh3 *= ilength;
    let ref mut fresh4 = *right.offset(2 as libc::c_int as isize);
    *fresh4 *= ilength;
    *up.offset(0 as libc::c_int as isize) =
        *right.offset(1 as libc::c_int as isize) *
            *forward.offset(2 as libc::c_int as isize) -
            *right.offset(2 as libc::c_int as isize) *
                *forward.offset(1 as libc::c_int as isize);
    *up.offset(1 as libc::c_int as isize) =
        *right.offset(2 as libc::c_int as isize) *
            *forward.offset(0 as libc::c_int as isize) -
            *right.offset(0 as libc::c_int as isize) *
                *forward.offset(2 as libc::c_int as isize);
    *up.offset(2 as libc::c_int as isize) =
        *right.offset(0 as libc::c_int as isize) *
            *forward.offset(1 as libc::c_int as isize) -
            *right.offset(1 as libc::c_int as isize) *
                *forward.offset(0 as libc::c_int as isize);
    let mut ilength_0: libc::c_float =
        __tg_sqrt(*up.offset(0 as libc::c_int as isize) *
                      *up.offset(0 as libc::c_int as isize) +
                      *up.offset(1 as libc::c_int as isize) *
                          *up.offset(1 as libc::c_int as isize) +
                      *up.offset(2 as libc::c_int as isize) *
                          *up.offset(2 as libc::c_int as isize));
    if ilength_0 != 0. { ilength_0 = 1.0f32 / ilength_0 }
    let ref mut fresh5 = *up.offset(0 as libc::c_int as isize);
    *fresh5 *= ilength_0;
    let ref mut fresh6 = *up.offset(1 as libc::c_int as isize);
    *fresh6 *= ilength_0;
    let ref mut fresh7 = *up.offset(2 as libc::c_int as isize);
    *fresh7 *= ilength_0;
}
/*
=================
AngleVectors

=================
*/
#[no_mangle]
pub unsafe extern "C" fn AngleVectors(mut angles: *const vec_t,
                                      mut forward: *mut vec_t,
                                      mut right: *mut vec_t,
                                      mut up: *mut vec_t) {
    let mut sr: libc::c_float = 0.;
    let mut sp: libc::c_float = 0.;
    let mut sy: libc::c_float = 0.;
    let mut cr: libc::c_float = 0.;
    let mut cp: libc::c_float = 0.;
    let mut cy: libc::c_float = 0.;
    SinCos(*angles.offset(1 as libc::c_int as isize) *
               (3.14159265358979323846f64 as libc::c_float / 180.0f32),
           &mut sy, &mut cy);
    SinCos(*angles.offset(0 as libc::c_int as isize) *
               (3.14159265358979323846f64 as libc::c_float / 180.0f32),
           &mut sp, &mut cp);
    SinCos(*angles.offset(2 as libc::c_int as isize) *
               (3.14159265358979323846f64 as libc::c_float / 180.0f32),
           &mut sr, &mut cr);
    if !forward.is_null() {
        *forward.offset(0 as libc::c_int as isize) = cp * cy;
        *forward.offset(1 as libc::c_int as isize) = cp * sy;
        *forward.offset(2 as libc::c_int as isize) = -sp
    }
    if !right.is_null() {
        *right.offset(0 as libc::c_int as isize) =
            -1.0f32 * sr * sp * cy + -1.0f32 * cr * -sy;
        *right.offset(1 as libc::c_int as isize) =
            -1.0f32 * sr * sp * sy + -1.0f32 * cr * cy;
        *right.offset(2 as libc::c_int as isize) = -1.0f32 * sr * cp
    }
    if !up.is_null() {
        *up.offset(0 as libc::c_int as isize) = cr * sp * cy + -sr * -sy;
        *up.offset(1 as libc::c_int as isize) = cr * sp * sy + -sr * cy;
        *up.offset(2 as libc::c_int as isize) = cr * cp
    };
}
/*
=================
VectorAngles

=================
*/
#[no_mangle]
pub unsafe extern "C" fn VectorAngles(mut forward: *const libc::c_float,
                                      mut angles: *mut libc::c_float) {
    let mut tmp: libc::c_float = 0.;
    let mut yaw: libc::c_float = 0.;
    let mut pitch: libc::c_float = 0.;
    if forward.is_null() || angles.is_null() {
        if !angles.is_null() {
            let ref mut fresh8 = *angles.offset(2 as libc::c_int as isize);
            *fresh8 = 0 as libc::c_int as libc::c_float;
            let ref mut fresh9 = *angles.offset(1 as libc::c_int as isize);
            *fresh9 = *fresh8;
            *angles.offset(0 as libc::c_int as isize) = *fresh9
        }
        return
    }
    if *forward.offset(1 as libc::c_int as isize) ==
           0 as libc::c_int as libc::c_float &&
           *forward.offset(0 as libc::c_int as isize) ==
               0 as libc::c_int as libc::c_float {
        // fast case
        yaw = 0 as libc::c_int as libc::c_float;
        if *forward.offset(2 as libc::c_int as isize) >
               0 as libc::c_int as libc::c_float {
            pitch = 90.0f32
        } else { pitch = 270.0f32 }
    } else {
        yaw =
            __tg_atan2(*forward.offset(1 as libc::c_int as isize),
                       *forward.offset(0 as libc::c_int as isize)) *
                180 as libc::c_int as libc::c_float /
                3.14159265358979323846f64 as libc::c_float;
        if yaw < 0 as libc::c_int as libc::c_float {
            yaw += 360 as libc::c_int as libc::c_float
        }
        tmp =
            __tg_sqrt(*forward.offset(0 as libc::c_int as isize) *
                          *forward.offset(0 as libc::c_int as isize) +
                          *forward.offset(1 as libc::c_int as isize) *
                              *forward.offset(1 as libc::c_int as isize));
        pitch =
            __tg_atan2(*forward.offset(2 as libc::c_int as isize), tmp) *
                180 as libc::c_int as libc::c_float /
                3.14159265358979323846f64 as libc::c_float;
        if pitch < 0 as libc::c_int as libc::c_float {
            pitch += 360 as libc::c_int as libc::c_float
        }
    }
    *angles.offset(0 as libc::c_int as isize) = pitch;
    *angles.offset(1 as libc::c_int as isize) = yaw;
    *angles.offset(2 as libc::c_int as isize) =
        0 as libc::c_int as libc::c_float;
}
/*
=================
VectorsAngles

=================
*/
#[no_mangle]
pub unsafe extern "C" fn VectorsAngles(mut forward: *const vec_t,
                                       mut right: *const vec_t,
                                       mut up: *const vec_t,
                                       mut angles: *mut vec_t) {
    let mut pitch: libc::c_float = 0.;
    let mut cpitch: libc::c_float = 0.;
    let mut yaw: libc::c_float = 0.;
    let mut roll: libc::c_float = 0.;
    pitch = -__tg_asin(*forward.offset(2 as libc::c_int as isize));
    cpitch = __tg_cos(pitch);
    if __tg_fabs(cpitch) > 0.001f32 {
        // gimball lock?
        cpitch = 1.0f32 / cpitch;
        pitch =
            pitch * (180.0f32 / 3.14159265358979323846f64 as libc::c_float);
        yaw =
            __tg_atan2(*forward.offset(1 as libc::c_int as isize) * cpitch,
                       *forward.offset(0 as libc::c_int as isize) * cpitch) *
                (180.0f32 / 3.14159265358979323846f64 as libc::c_float);
        roll =
            __tg_atan2(-*right.offset(2 as libc::c_int as isize) * cpitch,
                       *up.offset(2 as libc::c_int as isize) * cpitch) *
                (180.0f32 / 3.14159265358979323846f64 as libc::c_float)
    } else {
        pitch =
            if *forward.offset(2 as libc::c_int as isize) >
                   0 as libc::c_int as libc::c_float {
                -90.0f32
            } else { 90.0f32 };
        yaw =
            __tg_atan2(*right.offset(0 as libc::c_int as isize),
                       -*right.offset(1 as libc::c_int as isize)) *
                (180.0f32 / 3.14159265358979323846f64 as libc::c_float);
        roll = 180.0f32
    }
    *angles.offset(0 as libc::c_int as isize) = pitch;
    *angles.offset(1 as libc::c_int as isize) = yaw;
    *angles.offset(2 as libc::c_int as isize) = roll;
}
//
// bounds operations
//
/*
=================
ClearBounds
=================
*/
#[no_mangle]
pub unsafe extern "C" fn ClearBounds(mut mins: *mut vec_t,
                                     mut maxs: *mut vec_t) {
    // make bogus range
    let ref mut fresh10 = *mins.offset(2 as libc::c_int as isize);
    *fresh10 = 999999.0f32;
    let ref mut fresh11 = *mins.offset(1 as libc::c_int as isize);
    *fresh11 = *fresh10;
    *mins.offset(0 as libc::c_int as isize) = *fresh11;
    let ref mut fresh12 = *maxs.offset(2 as libc::c_int as isize);
    *fresh12 = -999999.0f32;
    let ref mut fresh13 = *maxs.offset(1 as libc::c_int as isize);
    *fresh13 = *fresh12;
    *maxs.offset(0 as libc::c_int as isize) = *fresh13;
}
/*
=================
AddPointToBounds
=================
*/
#[no_mangle]
pub unsafe extern "C" fn AddPointToBounds(mut v: *const vec_t,
                                          mut mins: *mut vec_t,
                                          mut maxs: *mut vec_t) {
    let mut val: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        val = *v.offset(i as isize);
        if val < *mins.offset(i as isize) { *mins.offset(i as isize) = val }
        if val > *maxs.offset(i as isize) { *maxs.offset(i as isize) = val }
        i += 1
    };
}
/*
=================
ExpandBounds
=================
*/
#[no_mangle]
pub unsafe extern "C" fn ExpandBounds(mut mins: *mut vec_t,
                                      mut maxs: *mut vec_t,
                                      mut offset: libc::c_float) {
    let ref mut fresh14 = *mins.offset(0 as libc::c_int as isize);
    *fresh14 -= offset;
    let ref mut fresh15 = *mins.offset(1 as libc::c_int as isize);
    *fresh15 -= offset;
    let ref mut fresh16 = *mins.offset(2 as libc::c_int as isize);
    *fresh16 -= offset;
    let ref mut fresh17 = *maxs.offset(0 as libc::c_int as isize);
    *fresh17 += offset;
    let ref mut fresh18 = *maxs.offset(1 as libc::c_int as isize);
    *fresh18 += offset;
    let ref mut fresh19 = *maxs.offset(2 as libc::c_int as isize);
    *fresh19 += offset;
}
/*
=================
BoundsIntersect
=================
*/
#[no_mangle]
pub unsafe extern "C" fn BoundsIntersect(mut mins1: *const vec_t,
                                         mut maxs1: *const vec_t,
                                         mut mins2: *const vec_t,
                                         mut maxs2: *const vec_t)
 -> qboolean {
    if *mins1.offset(0 as libc::c_int as isize) >
           *maxs2.offset(0 as libc::c_int as isize) ||
           *mins1.offset(1 as libc::c_int as isize) >
               *maxs2.offset(1 as libc::c_int as isize) ||
           *mins1.offset(2 as libc::c_int as isize) >
               *maxs2.offset(2 as libc::c_int as isize) {
        return false_0
    }
    if *maxs1.offset(0 as libc::c_int as isize) <
           *mins2.offset(0 as libc::c_int as isize) ||
           *maxs1.offset(1 as libc::c_int as isize) <
               *mins2.offset(1 as libc::c_int as isize) ||
           *maxs1.offset(2 as libc::c_int as isize) <
               *mins2.offset(2 as libc::c_int as isize) {
        return false_0
    }
    return true_0;
}
/*
=================
BoundsAndSphereIntersect
=================
*/
#[no_mangle]
pub unsafe extern "C" fn BoundsAndSphereIntersect(mut mins: *const vec_t,
                                                  mut maxs: *const vec_t,
                                                  mut origin: *const vec_t,
                                                  mut radius: libc::c_float)
 -> qboolean {
    if *mins.offset(0 as libc::c_int as isize) >
           *origin.offset(0 as libc::c_int as isize) + radius ||
           *mins.offset(1 as libc::c_int as isize) >
               *origin.offset(1 as libc::c_int as isize) + radius ||
           *mins.offset(2 as libc::c_int as isize) >
               *origin.offset(2 as libc::c_int as isize) + radius {
        return false_0
    }
    if *maxs.offset(0 as libc::c_int as isize) <
           *origin.offset(0 as libc::c_int as isize) - radius ||
           *maxs.offset(1 as libc::c_int as isize) <
               *origin.offset(1 as libc::c_int as isize) - radius ||
           *maxs.offset(2 as libc::c_int as isize) <
               *origin.offset(2 as libc::c_int as isize) - radius {
        return false_0
    }
    return true_0;
}
/*
=================
SphereIntersect
=================
*/
#[no_mangle]
pub unsafe extern "C" fn SphereIntersect(mut vSphereCenter: *const vec_t,
                                         mut fSphereRadiusSquared:
                                             libc::c_float,
                                         mut vLinePt: *const vec_t,
                                         mut vLineDir: *const vec_t)
 -> qboolean {
    let mut a: libc::c_float = 0.;
    let mut b: libc::c_float = 0.;
    let mut c: libc::c_float = 0.;
    let mut insideSqr: libc::c_float = 0.;
    let mut p: vec3_t = [0.; 3];
    // translate sphere to origin.
    p[0 as libc::c_int as usize] =
        *vLinePt.offset(0 as libc::c_int as isize) -
            *vSphereCenter.offset(0 as libc::c_int as isize);
    p[1 as libc::c_int as usize] =
        *vLinePt.offset(1 as libc::c_int as isize) -
            *vSphereCenter.offset(1 as libc::c_int as isize);
    p[2 as libc::c_int as usize] =
        *vLinePt.offset(2 as libc::c_int as isize) -
            *vSphereCenter.offset(2 as libc::c_int as isize);
    a =
        *vLineDir.offset(0 as libc::c_int as isize) *
            *vLineDir.offset(0 as libc::c_int as isize) +
            *vLineDir.offset(1 as libc::c_int as isize) *
                *vLineDir.offset(1 as libc::c_int as isize) +
            *vLineDir.offset(2 as libc::c_int as isize) *
                *vLineDir.offset(2 as libc::c_int as isize);
    b =
        2.0f32 *
            (p[0 as libc::c_int as usize] *
                 *vLineDir.offset(0 as libc::c_int as isize) +
                 p[1 as libc::c_int as usize] *
                     *vLineDir.offset(1 as libc::c_int as isize) +
                 p[2 as libc::c_int as usize] *
                     *vLineDir.offset(2 as libc::c_int as isize));
    c =
        p[0 as libc::c_int as usize] * p[0 as libc::c_int as usize] +
            p[1 as libc::c_int as usize] * p[1 as libc::c_int as usize] +
            p[2 as libc::c_int as usize] * p[2 as libc::c_int as usize] -
            fSphereRadiusSquared;
    insideSqr = b * b - 4.0f32 * a * c;
    if insideSqr <= 0.000001f32 { return false_0 }
    return true_0;
}
/*
=================
PlaneIntersect

find point where ray
was intersect with plane
=================
*/
#[no_mangle]
pub unsafe extern "C" fn PlaneIntersect(mut plane: *const mplane_t,
                                        mut p0: *const vec_t,
                                        mut p1: *const vec_t,
                                        mut out: *mut vec_t) {
    let mut distToPlane: libc::c_float =
        (if ((*plane).type_0 as libc::c_int) < 3 as libc::c_int {
             *p0.offset((*plane).type_0 as isize)
         } else {
             (*p0.offset(0 as libc::c_int as isize) *
                  (*plane).normal[0 as libc::c_int as usize] +
                  *p0.offset(1 as libc::c_int as isize) *
                      (*plane).normal[1 as libc::c_int as usize]) +
                 *p0.offset(2 as libc::c_int as isize) *
                     (*plane).normal[2 as libc::c_int as usize]
         }) - (*plane).dist;
    let mut planeDotRay: libc::c_float =
        (*plane).normal[0 as libc::c_int as usize] *
            *p1.offset(0 as libc::c_int as isize) +
            (*plane).normal[1 as libc::c_int as usize] *
                *p1.offset(1 as libc::c_int as isize) +
            (*plane).normal[2 as libc::c_int as usize] *
                *p1.offset(2 as libc::c_int as isize);
    let mut sect: libc::c_float = -distToPlane / planeDotRay;
    *out.offset(0 as libc::c_int as isize) =
        *p0.offset(0 as libc::c_int as isize) +
            sect * *p1.offset(0 as libc::c_int as isize);
    *out.offset(1 as libc::c_int as isize) =
        *p0.offset(1 as libc::c_int as isize) +
            sect * *p1.offset(1 as libc::c_int as isize);
    *out.offset(2 as libc::c_int as isize) =
        *p0.offset(2 as libc::c_int as isize) +
            sect * *p1.offset(2 as libc::c_int as isize);
}
/*
=================
RadiusFromBounds
=================
*/
#[no_mangle]
pub unsafe extern "C" fn RadiusFromBounds(mut mins: *const vec_t,
                                          mut maxs: *const vec_t)
 -> libc::c_float {
    let mut corner: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        corner[i as usize] =
            if __tg_fabs(*mins.offset(i as isize)) >
                   __tg_fabs(*maxs.offset(i as isize)) {
                __tg_fabs(*mins.offset(i as isize))
            } else { __tg_fabs(*maxs.offset(i as isize)) };
        i += 1
    }
    return __tg_sqrt(corner[0 as libc::c_int as usize] *
                         corner[0 as libc::c_int as usize] +
                         corner[1 as libc::c_int as usize] *
                             corner[1 as libc::c_int as usize] +
                         corner[2 as libc::c_int as usize] *
                             corner[2 as libc::c_int as usize]);
}
//
// studio utils
//
/*
====================
AngleQuaternion

====================
*/
#[no_mangle]
pub unsafe extern "C" fn AngleQuaternion(mut angles: *const vec_t,
                                         mut q: *mut vec_t,
                                         mut studio: qboolean) {
    let mut sr: libc::c_float = 0.; // X
    let mut sp: libc::c_float = 0.; // Y
    let mut sy: libc::c_float = 0.; // Z
    let mut cr: libc::c_float = 0.;
    let mut cp: libc::c_float = 0.;
    let mut cy: libc::c_float = 0.;
    if studio as u64 != 0 {
        SinCos(*angles.offset(2 as libc::c_int as isize) * 0.5f32, &mut sy,
               &mut cy);
        SinCos(*angles.offset(1 as libc::c_int as isize) * 0.5f32, &mut sp,
               &mut cp);
        SinCos(*angles.offset(0 as libc::c_int as isize) * 0.5f32, &mut sr,
               &mut cr);
    } else {
        SinCos(*angles.offset(1 as libc::c_int as isize) *
                   (3.14159265358979323846f64 as libc::c_float / 180.0f32) *
                   0.5f32, &mut sy, &mut cy);
        SinCos(*angles.offset(0 as libc::c_int as isize) *
                   (3.14159265358979323846f64 as libc::c_float / 180.0f32) *
                   0.5f32, &mut sp, &mut cp);
        SinCos(*angles.offset(2 as libc::c_int as isize) *
                   (3.14159265358979323846f64 as libc::c_float / 180.0f32) *
                   0.5f32, &mut sr, &mut cr);
    }
    *q.offset(0 as libc::c_int as isize) = sr * cp * cy - cr * sp * sy;
    *q.offset(1 as libc::c_int as isize) = cr * sp * cy + sr * cp * sy;
    *q.offset(2 as libc::c_int as isize) = cr * cp * sy - sr * sp * cy;
    *q.offset(3 as libc::c_int as isize) = cr * cp * cy + sr * sp * sy;
    // W
}
/*
====================
QuaternionAngle

====================
*/
#[no_mangle]
pub unsafe extern "C" fn QuaternionAngle(mut q: *const vec_t,
                                         mut angles: *mut vec_t) {
    let mut mat: matrix3x4 = [[0.; 4]; 3];
    Matrix3x4_FromOriginQuat(mat.as_mut_ptr(), q,
                             vec3_origin.as_mut_ptr() as *const vec_t);
    Matrix3x4_AnglesFromMatrix(mat.as_mut_ptr() as *const [vec_t; 4], angles);
}
/*
====================
QuaternionAlign

make sure quaternions are within 180 degrees of one another,
if not, reverse q
====================
*/
#[no_mangle]
pub unsafe extern "C" fn QuaternionAlign(mut p: *const vec_t,
                                         mut q: *const vec_t,
                                         mut qt: *mut vec_t) {
    // decide if one of the quaternions is backwards
    let mut a: libc::c_float = 0.0f32;
    let mut b: libc::c_float = 0.0f32;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        a +=
            (*p.offset(i as isize) - *q.offset(i as isize)) *
                (*p.offset(i as isize) - *q.offset(i as isize));
        b +=
            (*p.offset(i as isize) + *q.offset(i as isize)) *
                (*p.offset(i as isize) + *q.offset(i as isize));
        i += 1
    }
    if a > b {
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            *qt.offset(i as isize) = -*q.offset(i as isize);
            i += 1
        }
    } else {
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            *qt.offset(i as isize) = *q.offset(i as isize);
            i += 1
        }
    };
}
/*
====================
QuaternionSlerpNoAlign
====================
*/
#[no_mangle]
pub unsafe extern "C" fn QuaternionSlerpNoAlign(mut p: *const vec_t,
                                                mut q: *const vec_t,
                                                mut t: libc::c_float,
                                                mut qt: *mut vec_t) {
    let mut omega: libc::c_float = 0.;
    let mut cosom: libc::c_float = 0.;
    let mut sinom: libc::c_float = 0.;
    let mut sclp: libc::c_float = 0.;
    let mut sclq: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    // 0.0 returns p, 1.0 return q.
    cosom =
        *p.offset(0 as libc::c_int as isize) *
            *q.offset(0 as libc::c_int as isize) +
            *p.offset(1 as libc::c_int as isize) *
                *q.offset(1 as libc::c_int as isize) +
            *p.offset(2 as libc::c_int as isize) *
                *q.offset(2 as libc::c_int as isize) +
            *p.offset(3 as libc::c_int as isize) *
                *q.offset(3 as libc::c_int as isize);
    if 1.0f32 + cosom > 0.000001f32 {
        if 1.0f32 - cosom > 0.000001f32 {
            omega = __tg_acos(cosom);
            sinom = __tg_sin(omega);
            sclp = __tg_sin((1.0f32 - t) * omega) / sinom;
            sclq = __tg_sin(t * omega) / sinom
        } else { sclp = 1.0f32 - t; sclq = t }
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            *qt.offset(i as isize) =
                sclp * *p.offset(i as isize) + sclq * *q.offset(i as isize);
            i += 1
        }
    } else {
        *qt.offset(0 as libc::c_int as isize) =
            -*q.offset(1 as libc::c_int as isize);
        *qt.offset(1 as libc::c_int as isize) =
            *q.offset(0 as libc::c_int as isize);
        *qt.offset(2 as libc::c_int as isize) =
            -*q.offset(3 as libc::c_int as isize);
        *qt.offset(3 as libc::c_int as isize) =
            *q.offset(2 as libc::c_int as isize);
        sclp =
            __tg_sin((1.0f32 - t) *
                         (0.5f32 *
                              3.14159265358979323846f64 as libc::c_float));
        sclq =
            __tg_sin(t *
                         (0.5f32 *
                              3.14159265358979323846f64 as libc::c_float));
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            *qt.offset(i as isize) =
                sclp * *p.offset(i as isize) + sclq * *qt.offset(i as isize);
            i += 1
        }
    };
}
/*
====================
QuaternionSlerp

Quaternion sphereical linear interpolation
====================
*/
#[no_mangle]
pub unsafe extern "C" fn QuaternionSlerp(mut p: *const vec_t,
                                         mut q: *const vec_t,
                                         mut t: libc::c_float,
                                         mut qt: *mut vec_t) {
    let mut q2: vec4_t = [0.; 4];
    // 0.0 returns p, 1.0 return q.
	// decide if one of the quaternions is backwards
    QuaternionAlign(p, q, q2.as_mut_ptr());
    QuaternionSlerpNoAlign(p, q2.as_mut_ptr() as *const vec_t, t, qt);
}
/*
====================
V_CalcFov
====================
*/
#[no_mangle]
pub unsafe extern "C" fn V_CalcFov(mut fov_x: *mut libc::c_float,
                                   mut width: libc::c_float,
                                   mut height: libc::c_float)
 -> libc::c_float {
    let mut x: libc::c_float = 0.; // default value
    let mut half_fov_y: libc::c_float = 0.;
    if *fov_x < 1.0f32 || *fov_x > 179.0f32 { *fov_x = 90.0f32 }
    x =
        width /
            __tg_tan(*fov_x *
                         (3.14159265358979323846f64 as libc::c_float /
                              180.0f32) * 0.5f32);
    half_fov_y = __tg_atan(height / x);
    return half_fov_y *
               (180.0f32 / 3.14159265358979323846f64 as libc::c_float) *
               2 as libc::c_int as libc::c_float;
}
/*
====================
V_AdjustFov
====================
*/
#[no_mangle]
pub unsafe extern "C" fn V_AdjustFov(mut fov_x: *mut libc::c_float,
                                     mut fov_y: *mut libc::c_float,
                                     mut width: libc::c_float,
                                     mut height: libc::c_float,
                                     mut lock_x: qboolean) {
    let mut x: libc::c_float = 0.;
    let mut y: libc::c_float = 0.;
    if width * 3 as libc::c_int as libc::c_float ==
           4 as libc::c_int as libc::c_float * height ||
           width * 4 as libc::c_int as libc::c_float ==
               height * 5 as libc::c_int as libc::c_float {
        // 4:3 or 5:4 ratio
        return
    }
    if lock_x as u64 != 0 {
        *fov_y =
            2 as libc::c_int as libc::c_float *
                __tg_atan(width * 3 as libc::c_int as libc::c_float /
                              (height * 4 as libc::c_int as libc::c_float) *
                              __tg_tan(*fov_y *
                                           3.14159265358979323846f64 as
                                               libc::c_float / 360.0f32 *
                                           0.5f32)) *
                360 as libc::c_int as libc::c_float /
                3.14159265358979323846f64 as libc::c_float;
        return
    }
    y =
        V_CalcFov(fov_x, 640 as libc::c_int as libc::c_float,
                  480 as libc::c_int as libc::c_float);
    x = *fov_x;
    *fov_x = V_CalcFov(&mut y, height, width);
    if *fov_x < x { *fov_x = x } else { *fov_y = y };
}
/*
==================
BoxOnPlaneSide

Returns 1, 2, or 1 + 2
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BoxOnPlaneSide(mut emins: *const vec_t,
                                        mut emaxs: *const vec_t,
                                        mut p: *const mplane_t)
 -> libc::c_int {
    let mut dist1: libc::c_float = 0.;
    let mut dist2: libc::c_float = 0.;
    let mut sides: libc::c_int = 0 as libc::c_int;
    // general case
    match (*p).signbits as libc::c_int {
        0 => {
            dist1 =
                (*p).normal[0 as libc::c_int as usize] *
                    *emaxs.offset(0 as libc::c_int as isize) +
                    (*p).normal[1 as libc::c_int as usize] *
                        *emaxs.offset(1 as libc::c_int as isize) +
                    (*p).normal[2 as libc::c_int as usize] *
                        *emaxs.offset(2 as libc::c_int as isize);
            dist2 =
                (*p).normal[0 as libc::c_int as usize] *
                    *emins.offset(0 as libc::c_int as isize) +
                    (*p).normal[1 as libc::c_int as usize] *
                        *emins.offset(1 as libc::c_int as isize) +
                    (*p).normal[2 as libc::c_int as usize] *
                        *emins.offset(2 as libc::c_int as isize)
        }
        1 => {
            dist1 =
                (*p).normal[0 as libc::c_int as usize] *
                    *emins.offset(0 as libc::c_int as isize) +
                    (*p).normal[1 as libc::c_int as usize] *
                        *emaxs.offset(1 as libc::c_int as isize) +
                    (*p).normal[2 as libc::c_int as usize] *
                        *emaxs.offset(2 as libc::c_int as isize);
            dist2 =
                (*p).normal[0 as libc::c_int as usize] *
                    *emaxs.offset(0 as libc::c_int as isize) +
                    (*p).normal[1 as libc::c_int as usize] *
                        *emins.offset(1 as libc::c_int as isize) +
                    (*p).normal[2 as libc::c_int as usize] *
                        *emins.offset(2 as libc::c_int as isize)
        }
        2 => {
            dist1 =
                (*p).normal[0 as libc::c_int as usize] *
                    *emaxs.offset(0 as libc::c_int as isize) +
                    (*p).normal[1 as libc::c_int as usize] *
                        *emins.offset(1 as libc::c_int as isize) +
                    (*p).normal[2 as libc::c_int as usize] *
                        *emaxs.offset(2 as libc::c_int as isize);
            dist2 =
                (*p).normal[0 as libc::c_int as usize] *
                    *emins.offset(0 as libc::c_int as isize) +
                    (*p).normal[1 as libc::c_int as usize] *
                        *emaxs.offset(1 as libc::c_int as isize) +
                    (*p).normal[2 as libc::c_int as usize] *
                        *emins.offset(2 as libc::c_int as isize)
        }
        3 => {
            dist1 =
                (*p).normal[0 as libc::c_int as usize] *
                    *emins.offset(0 as libc::c_int as isize) +
                    (*p).normal[1 as libc::c_int as usize] *
                        *emins.offset(1 as libc::c_int as isize) +
                    (*p).normal[2 as libc::c_int as usize] *
                        *emaxs.offset(2 as libc::c_int as isize);
            dist2 =
                (*p).normal[0 as libc::c_int as usize] *
                    *emaxs.offset(0 as libc::c_int as isize) +
                    (*p).normal[1 as libc::c_int as usize] *
                        *emaxs.offset(1 as libc::c_int as isize) +
                    (*p).normal[2 as libc::c_int as usize] *
                        *emins.offset(2 as libc::c_int as isize)
        }
        4 => {
            dist1 =
                (*p).normal[0 as libc::c_int as usize] *
                    *emaxs.offset(0 as libc::c_int as isize) +
                    (*p).normal[1 as libc::c_int as usize] *
                        *emaxs.offset(1 as libc::c_int as isize) +
                    (*p).normal[2 as libc::c_int as usize] *
                        *emins.offset(2 as libc::c_int as isize);
            dist2 =
                (*p).normal[0 as libc::c_int as usize] *
                    *emins.offset(0 as libc::c_int as isize) +
                    (*p).normal[1 as libc::c_int as usize] *
                        *emins.offset(1 as libc::c_int as isize) +
                    (*p).normal[2 as libc::c_int as usize] *
                        *emaxs.offset(2 as libc::c_int as isize)
        }
        5 => {
            dist1 =
                (*p).normal[0 as libc::c_int as usize] *
                    *emins.offset(0 as libc::c_int as isize) +
                    (*p).normal[1 as libc::c_int as usize] *
                        *emaxs.offset(1 as libc::c_int as isize) +
                    (*p).normal[2 as libc::c_int as usize] *
                        *emins.offset(2 as libc::c_int as isize);
            dist2 =
                (*p).normal[0 as libc::c_int as usize] *
                    *emaxs.offset(0 as libc::c_int as isize) +
                    (*p).normal[1 as libc::c_int as usize] *
                        *emins.offset(1 as libc::c_int as isize) +
                    (*p).normal[2 as libc::c_int as usize] *
                        *emaxs.offset(2 as libc::c_int as isize)
        }
        6 => {
            dist1 =
                (*p).normal[0 as libc::c_int as usize] *
                    *emaxs.offset(0 as libc::c_int as isize) +
                    (*p).normal[1 as libc::c_int as usize] *
                        *emins.offset(1 as libc::c_int as isize) +
                    (*p).normal[2 as libc::c_int as usize] *
                        *emins.offset(2 as libc::c_int as isize);
            dist2 =
                (*p).normal[0 as libc::c_int as usize] *
                    *emins.offset(0 as libc::c_int as isize) +
                    (*p).normal[1 as libc::c_int as usize] *
                        *emaxs.offset(1 as libc::c_int as isize) +
                    (*p).normal[2 as libc::c_int as usize] *
                        *emaxs.offset(2 as libc::c_int as isize)
        }
        7 => {
            dist1 =
                (*p).normal[0 as libc::c_int as usize] *
                    *emins.offset(0 as libc::c_int as isize) +
                    (*p).normal[1 as libc::c_int as usize] *
                        *emins.offset(1 as libc::c_int as isize) +
                    (*p).normal[2 as libc::c_int as usize] *
                        *emins.offset(2 as libc::c_int as isize);
            dist2 =
                (*p).normal[0 as libc::c_int as usize] *
                    *emaxs.offset(0 as libc::c_int as isize) +
                    (*p).normal[1 as libc::c_int as usize] *
                        *emaxs.offset(1 as libc::c_int as isize) +
                    (*p).normal[2 as libc::c_int as usize] *
                        *emaxs.offset(2 as libc::c_int as isize)
        }
        _ => {
            // shut up compiler
            dist2 = 0 as libc::c_int as libc::c_float;
            dist1 = dist2
        }
    }
    if dist1 >= (*p).dist { sides = 1 as libc::c_int }
    if dist2 < (*p).dist { sides |= 2 as libc::c_int }
    return sides;
}
