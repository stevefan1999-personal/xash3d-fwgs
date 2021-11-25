#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn atan2f(_: libc::c_float, _: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn sqrtf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn fabsf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn SinCos(radians: libc::c_float, sine: *mut libc::c_float,
              cosine: *mut libc::c_float);
}
pub type vec_t = libc::c_float;
pub type vec3_t = [vec_t; 3];
pub type matrix3x4 = [[vec_t; 4]; 3];
pub type matrix4x4 = [[vec_t; 4]; 4];
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
#[inline(always)]
unsafe extern "C" fn __tg_sqrt(mut __x: libc::c_float) -> libc::c_float {
    return sqrtf(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_fabs(mut __x: libc::c_float) -> libc::c_float {
    return fabsf(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_atan2(mut __x: libc::c_float,
                                mut __y: libc::c_float) -> libc::c_float {
    return atan2f(__x, __y);
}
/*
matrixlib.c - internal matrixlib
Copyright (C) 2010 Uncle Mike

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
*/
#[no_mangle]
pub static mut matrix3x4_identity: matrix3x4 =
    [[1 as libc::c_int as vec_t, 0 as libc::c_int as vec_t,
      0 as libc::c_int as vec_t, 0 as libc::c_int as vec_t],
     [0 as libc::c_int as vec_t, 1 as libc::c_int as vec_t,
      0 as libc::c_int as vec_t, 0 as libc::c_int as vec_t],
     [0 as libc::c_int as vec_t, 0 as libc::c_int as vec_t,
      1 as libc::c_int as vec_t, 0 as libc::c_int as vec_t]];
/*
========================================================================

		Matrix3x4 operations

========================================================================
*/
#[no_mangle]
pub unsafe extern "C" fn Matrix3x4_VectorTransform(mut in_0:
                                                       *const [vec_t; 4],
                                                   mut v:
                                                       *const libc::c_float,
                                                   mut out:
                                                       *mut libc::c_float) {
    *out.offset(0 as libc::c_int as isize) =
        *v.offset(0 as libc::c_int as isize) *
            (*in_0.offset(0 as libc::c_int as
                              isize))[0 as libc::c_int as usize] +
            *v.offset(1 as libc::c_int as isize) *
                (*in_0.offset(0 as libc::c_int as
                                  isize))[1 as libc::c_int as usize] +
            *v.offset(2 as libc::c_int as isize) *
                (*in_0.offset(0 as libc::c_int as
                                  isize))[2 as libc::c_int as usize] +
            (*in_0.offset(0 as libc::c_int as
                              isize))[3 as libc::c_int as usize];
    *out.offset(1 as libc::c_int as isize) =
        *v.offset(0 as libc::c_int as isize) *
            (*in_0.offset(1 as libc::c_int as
                              isize))[0 as libc::c_int as usize] +
            *v.offset(1 as libc::c_int as isize) *
                (*in_0.offset(1 as libc::c_int as
                                  isize))[1 as libc::c_int as usize] +
            *v.offset(2 as libc::c_int as isize) *
                (*in_0.offset(1 as libc::c_int as
                                  isize))[2 as libc::c_int as usize] +
            (*in_0.offset(1 as libc::c_int as
                              isize))[3 as libc::c_int as usize];
    *out.offset(2 as libc::c_int as isize) =
        *v.offset(0 as libc::c_int as isize) *
            (*in_0.offset(2 as libc::c_int as
                              isize))[0 as libc::c_int as usize] +
            *v.offset(1 as libc::c_int as isize) *
                (*in_0.offset(2 as libc::c_int as
                                  isize))[1 as libc::c_int as usize] +
            *v.offset(2 as libc::c_int as isize) *
                (*in_0.offset(2 as libc::c_int as
                                  isize))[2 as libc::c_int as usize] +
            (*in_0.offset(2 as libc::c_int as
                              isize))[3 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn Matrix3x4_VectorITransform(mut in_0:
                                                        *const [vec_t; 4],
                                                    mut v:
                                                        *const libc::c_float,
                                                    mut out:
                                                        *mut libc::c_float) {
    let mut dir: vec3_t = [0.; 3];
    dir[0 as libc::c_int as usize] =
        *v.offset(0 as libc::c_int as isize) -
            (*in_0.offset(0 as libc::c_int as
                              isize))[3 as libc::c_int as usize];
    dir[1 as libc::c_int as usize] =
        *v.offset(1 as libc::c_int as isize) -
            (*in_0.offset(1 as libc::c_int as
                              isize))[3 as libc::c_int as usize];
    dir[2 as libc::c_int as usize] =
        *v.offset(2 as libc::c_int as isize) -
            (*in_0.offset(2 as libc::c_int as
                              isize))[3 as libc::c_int as usize];
    *out.offset(0 as libc::c_int as isize) =
        dir[0 as libc::c_int as usize] *
            (*in_0.offset(0 as libc::c_int as
                              isize))[0 as libc::c_int as usize] +
            dir[1 as libc::c_int as usize] *
                (*in_0.offset(1 as libc::c_int as
                                  isize))[0 as libc::c_int as usize] +
            dir[2 as libc::c_int as usize] *
                (*in_0.offset(2 as libc::c_int as
                                  isize))[0 as libc::c_int as usize];
    *out.offset(1 as libc::c_int as isize) =
        dir[0 as libc::c_int as usize] *
            (*in_0.offset(0 as libc::c_int as
                              isize))[1 as libc::c_int as usize] +
            dir[1 as libc::c_int as usize] *
                (*in_0.offset(1 as libc::c_int as
                                  isize))[1 as libc::c_int as usize] +
            dir[2 as libc::c_int as usize] *
                (*in_0.offset(2 as libc::c_int as
                                  isize))[1 as libc::c_int as usize];
    *out.offset(2 as libc::c_int as isize) =
        dir[0 as libc::c_int as usize] *
            (*in_0.offset(0 as libc::c_int as
                              isize))[2 as libc::c_int as usize] +
            dir[1 as libc::c_int as usize] *
                (*in_0.offset(1 as libc::c_int as
                                  isize))[2 as libc::c_int as usize] +
            dir[2 as libc::c_int as usize] *
                (*in_0.offset(2 as libc::c_int as
                                  isize))[2 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn Matrix3x4_VectorRotate(mut in_0: *const [vec_t; 4],
                                                mut v: *const libc::c_float,
                                                mut out: *mut libc::c_float) {
    *out.offset(0 as libc::c_int as isize) =
        *v.offset(0 as libc::c_int as isize) *
            (*in_0.offset(0 as libc::c_int as
                              isize))[0 as libc::c_int as usize] +
            *v.offset(1 as libc::c_int as isize) *
                (*in_0.offset(0 as libc::c_int as
                                  isize))[1 as libc::c_int as usize] +
            *v.offset(2 as libc::c_int as isize) *
                (*in_0.offset(0 as libc::c_int as
                                  isize))[2 as libc::c_int as usize];
    *out.offset(1 as libc::c_int as isize) =
        *v.offset(0 as libc::c_int as isize) *
            (*in_0.offset(1 as libc::c_int as
                              isize))[0 as libc::c_int as usize] +
            *v.offset(1 as libc::c_int as isize) *
                (*in_0.offset(1 as libc::c_int as
                                  isize))[1 as libc::c_int as usize] +
            *v.offset(2 as libc::c_int as isize) *
                (*in_0.offset(1 as libc::c_int as
                                  isize))[2 as libc::c_int as usize];
    *out.offset(2 as libc::c_int as isize) =
        *v.offset(0 as libc::c_int as isize) *
            (*in_0.offset(2 as libc::c_int as
                              isize))[0 as libc::c_int as usize] +
            *v.offset(1 as libc::c_int as isize) *
                (*in_0.offset(2 as libc::c_int as
                                  isize))[1 as libc::c_int as usize] +
            *v.offset(2 as libc::c_int as isize) *
                (*in_0.offset(2 as libc::c_int as
                                  isize))[2 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn Matrix3x4_VectorIRotate(mut in_0: *const [vec_t; 4],
                                                 mut v: *const libc::c_float,
                                                 mut out:
                                                     *mut libc::c_float) {
    *out.offset(0 as libc::c_int as isize) =
        *v.offset(0 as libc::c_int as isize) *
            (*in_0.offset(0 as libc::c_int as
                              isize))[0 as libc::c_int as usize] +
            *v.offset(1 as libc::c_int as isize) *
                (*in_0.offset(1 as libc::c_int as
                                  isize))[0 as libc::c_int as usize] +
            *v.offset(2 as libc::c_int as isize) *
                (*in_0.offset(2 as libc::c_int as
                                  isize))[0 as libc::c_int as usize];
    *out.offset(1 as libc::c_int as isize) =
        *v.offset(0 as libc::c_int as isize) *
            (*in_0.offset(0 as libc::c_int as
                              isize))[1 as libc::c_int as usize] +
            *v.offset(1 as libc::c_int as isize) *
                (*in_0.offset(1 as libc::c_int as
                                  isize))[1 as libc::c_int as usize] +
            *v.offset(2 as libc::c_int as isize) *
                (*in_0.offset(2 as libc::c_int as
                                  isize))[1 as libc::c_int as usize];
    *out.offset(2 as libc::c_int as isize) =
        *v.offset(0 as libc::c_int as isize) *
            (*in_0.offset(0 as libc::c_int as
                              isize))[2 as libc::c_int as usize] +
            *v.offset(1 as libc::c_int as isize) *
                (*in_0.offset(1 as libc::c_int as
                                  isize))[2 as libc::c_int as usize] +
            *v.offset(2 as libc::c_int as isize) *
                (*in_0.offset(2 as libc::c_int as
                                  isize))[2 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn Matrix3x4_ConcatTransforms(mut out: *mut [vec_t; 4],
                                                    mut in1:
                                                        *const [vec_t; 4],
                                                    mut in2:
                                                        *const [vec_t; 4]) {
    (*out.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (*in1.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] *
            (*in2.offset(0 as libc::c_int as
                             isize))[0 as libc::c_int as usize] +
            (*in1.offset(0 as libc::c_int as
                             isize))[1 as libc::c_int as usize] *
                (*in2.offset(1 as libc::c_int as
                                 isize))[0 as libc::c_int as usize] +
            (*in1.offset(0 as libc::c_int as
                             isize))[2 as libc::c_int as usize] *
                (*in2.offset(2 as libc::c_int as
                                 isize))[0 as libc::c_int as usize];
    (*out.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] =
        (*in1.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] *
            (*in2.offset(0 as libc::c_int as
                             isize))[1 as libc::c_int as usize] +
            (*in1.offset(0 as libc::c_int as
                             isize))[1 as libc::c_int as usize] *
                (*in2.offset(1 as libc::c_int as
                                 isize))[1 as libc::c_int as usize] +
            (*in1.offset(0 as libc::c_int as
                             isize))[2 as libc::c_int as usize] *
                (*in2.offset(2 as libc::c_int as
                                 isize))[1 as libc::c_int as usize];
    (*out.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize] =
        (*in1.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] *
            (*in2.offset(0 as libc::c_int as
                             isize))[2 as libc::c_int as usize] +
            (*in1.offset(0 as libc::c_int as
                             isize))[1 as libc::c_int as usize] *
                (*in2.offset(1 as libc::c_int as
                                 isize))[2 as libc::c_int as usize] +
            (*in1.offset(0 as libc::c_int as
                             isize))[2 as libc::c_int as usize] *
                (*in2.offset(2 as libc::c_int as
                                 isize))[2 as libc::c_int as usize];
    (*out.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize] =
        (*in1.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] *
            (*in2.offset(0 as libc::c_int as
                             isize))[3 as libc::c_int as usize] +
            (*in1.offset(0 as libc::c_int as
                             isize))[1 as libc::c_int as usize] *
                (*in2.offset(1 as libc::c_int as
                                 isize))[3 as libc::c_int as usize] +
            (*in1.offset(0 as libc::c_int as
                             isize))[2 as libc::c_int as usize] *
                (*in2.offset(2 as libc::c_int as
                                 isize))[3 as libc::c_int as usize] +
            (*in1.offset(0 as libc::c_int as
                             isize))[3 as libc::c_int as usize];
    (*out.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (*in1.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] *
            (*in2.offset(0 as libc::c_int as
                             isize))[0 as libc::c_int as usize] +
            (*in1.offset(1 as libc::c_int as
                             isize))[1 as libc::c_int as usize] *
                (*in2.offset(1 as libc::c_int as
                                 isize))[0 as libc::c_int as usize] +
            (*in1.offset(1 as libc::c_int as
                             isize))[2 as libc::c_int as usize] *
                (*in2.offset(2 as libc::c_int as
                                 isize))[0 as libc::c_int as usize];
    (*out.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] =
        (*in1.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] *
            (*in2.offset(0 as libc::c_int as
                             isize))[1 as libc::c_int as usize] +
            (*in1.offset(1 as libc::c_int as
                             isize))[1 as libc::c_int as usize] *
                (*in2.offset(1 as libc::c_int as
                                 isize))[1 as libc::c_int as usize] +
            (*in1.offset(1 as libc::c_int as
                             isize))[2 as libc::c_int as usize] *
                (*in2.offset(2 as libc::c_int as
                                 isize))[1 as libc::c_int as usize];
    (*out.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize] =
        (*in1.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] *
            (*in2.offset(0 as libc::c_int as
                             isize))[2 as libc::c_int as usize] +
            (*in1.offset(1 as libc::c_int as
                             isize))[1 as libc::c_int as usize] *
                (*in2.offset(1 as libc::c_int as
                                 isize))[2 as libc::c_int as usize] +
            (*in1.offset(1 as libc::c_int as
                             isize))[2 as libc::c_int as usize] *
                (*in2.offset(2 as libc::c_int as
                                 isize))[2 as libc::c_int as usize];
    (*out.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize] =
        (*in1.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] *
            (*in2.offset(0 as libc::c_int as
                             isize))[3 as libc::c_int as usize] +
            (*in1.offset(1 as libc::c_int as
                             isize))[1 as libc::c_int as usize] *
                (*in2.offset(1 as libc::c_int as
                                 isize))[3 as libc::c_int as usize] +
            (*in1.offset(1 as libc::c_int as
                             isize))[2 as libc::c_int as usize] *
                (*in2.offset(2 as libc::c_int as
                                 isize))[3 as libc::c_int as usize] +
            (*in1.offset(1 as libc::c_int as
                             isize))[3 as libc::c_int as usize];
    (*out.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (*in1.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] *
            (*in2.offset(0 as libc::c_int as
                             isize))[0 as libc::c_int as usize] +
            (*in1.offset(2 as libc::c_int as
                             isize))[1 as libc::c_int as usize] *
                (*in2.offset(1 as libc::c_int as
                                 isize))[0 as libc::c_int as usize] +
            (*in1.offset(2 as libc::c_int as
                             isize))[2 as libc::c_int as usize] *
                (*in2.offset(2 as libc::c_int as
                                 isize))[0 as libc::c_int as usize];
    (*out.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize] =
        (*in1.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] *
            (*in2.offset(0 as libc::c_int as
                             isize))[1 as libc::c_int as usize] +
            (*in1.offset(2 as libc::c_int as
                             isize))[1 as libc::c_int as usize] *
                (*in2.offset(1 as libc::c_int as
                                 isize))[1 as libc::c_int as usize] +
            (*in1.offset(2 as libc::c_int as
                             isize))[2 as libc::c_int as usize] *
                (*in2.offset(2 as libc::c_int as
                                 isize))[1 as libc::c_int as usize];
    (*out.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize] =
        (*in1.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] *
            (*in2.offset(0 as libc::c_int as
                             isize))[2 as libc::c_int as usize] +
            (*in1.offset(2 as libc::c_int as
                             isize))[1 as libc::c_int as usize] *
                (*in2.offset(1 as libc::c_int as
                                 isize))[2 as libc::c_int as usize] +
            (*in1.offset(2 as libc::c_int as
                             isize))[2 as libc::c_int as usize] *
                (*in2.offset(2 as libc::c_int as
                                 isize))[2 as libc::c_int as usize];
    (*out.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize] =
        (*in1.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] *
            (*in2.offset(0 as libc::c_int as
                             isize))[3 as libc::c_int as usize] +
            (*in1.offset(2 as libc::c_int as
                             isize))[1 as libc::c_int as usize] *
                (*in2.offset(1 as libc::c_int as
                                 isize))[3 as libc::c_int as usize] +
            (*in1.offset(2 as libc::c_int as
                             isize))[2 as libc::c_int as usize] *
                (*in2.offset(2 as libc::c_int as
                                 isize))[3 as libc::c_int as usize] +
            (*in1.offset(2 as libc::c_int as
                             isize))[3 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn Matrix3x4_SetOrigin(mut out: *mut [vec_t; 4],
                                             mut x: libc::c_float,
                                             mut y: libc::c_float,
                                             mut z: libc::c_float) {
    (*out.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize] = x;
    (*out.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize] = y;
    (*out.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize] = z;
}
#[no_mangle]
pub unsafe extern "C" fn Matrix3x4_OriginFromMatrix(mut in_0:
                                                        *const [vec_t; 4],
                                                    mut out:
                                                        *mut libc::c_float) {
    *out.offset(0 as libc::c_int as isize) =
        (*in_0.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize];
    *out.offset(1 as libc::c_int as isize) =
        (*in_0.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize];
    *out.offset(2 as libc::c_int as isize) =
        (*in_0.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn Matrix3x4_AnglesFromMatrix(mut in_0:
                                                        *const [vec_t; 4],
                                                    mut out: *mut vec_t) {
    let mut xyDist: libc::c_float =
        __tg_sqrt((*in_0.offset(0 as libc::c_int as
                                    isize))[0 as libc::c_int as usize] *
                      (*in_0.offset(0 as libc::c_int as
                                        isize))[0 as libc::c_int as usize] +
                      (*in_0.offset(1 as libc::c_int as
                                        isize))[0 as libc::c_int as usize] *
                          (*in_0.offset(1 as libc::c_int as
                                            isize))[0 as libc::c_int as
                                                        usize]);
    if xyDist > 0.001f32 {
        // enough here to get angles?
        *out.offset(0 as libc::c_int as isize) =
            __tg_atan2(-(*in_0.offset(2 as libc::c_int as
                                          isize))[0 as libc::c_int as usize],
                       xyDist) *
                (180.0f32 / 3.14159265358979323846f64 as libc::c_float);
        *out.offset(1 as libc::c_int as isize) =
            __tg_atan2((*in_0.offset(1 as libc::c_int as
                                         isize))[0 as libc::c_int as usize],
                       (*in_0.offset(0 as libc::c_int as
                                         isize))[0 as libc::c_int as usize]) *
                (180.0f32 / 3.14159265358979323846f64 as libc::c_float);
        *out.offset(2 as libc::c_int as isize) =
            __tg_atan2((*in_0.offset(2 as libc::c_int as
                                         isize))[1 as libc::c_int as usize],
                       (*in_0.offset(2 as libc::c_int as
                                         isize))[2 as libc::c_int as usize]) *
                (180.0f32 / 3.14159265358979323846f64 as libc::c_float)
    } else {
        // forward is mostly Z, gimbal lock
        *out.offset(0 as libc::c_int as isize) =
            __tg_atan2(-(*in_0.offset(2 as libc::c_int as
                                          isize))[0 as libc::c_int as usize],
                       xyDist) *
                (180.0f32 / 3.14159265358979323846f64 as libc::c_float);
        *out.offset(1 as libc::c_int as isize) =
            __tg_atan2(-(*in_0.offset(0 as libc::c_int as
                                          isize))[1 as libc::c_int as usize],
                       (*in_0.offset(1 as libc::c_int as
                                         isize))[1 as libc::c_int as usize]) *
                (180.0f32 / 3.14159265358979323846f64 as libc::c_float);
        *out.offset(2 as libc::c_int as isize) = 0.0f32
    };
}
#[no_mangle]
pub unsafe extern "C" fn Matrix3x4_FromOriginQuat(mut out: *mut [vec_t; 4],
                                                  mut quaternion:
                                                      *const vec_t,
                                                  mut origin: *const vec_t) {
    (*out.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] =
        1.0f32 -
            2.0f32 * *quaternion.offset(1 as libc::c_int as isize) *
                *quaternion.offset(1 as libc::c_int as isize) -
            2.0f32 * *quaternion.offset(2 as libc::c_int as isize) *
                *quaternion.offset(2 as libc::c_int as isize);
    (*out.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] =
        2.0f32 * *quaternion.offset(0 as libc::c_int as isize) *
            *quaternion.offset(1 as libc::c_int as isize) +
            2.0f32 * *quaternion.offset(3 as libc::c_int as isize) *
                *quaternion.offset(2 as libc::c_int as isize);
    (*out.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] =
        2.0f32 * *quaternion.offset(0 as libc::c_int as isize) *
            *quaternion.offset(2 as libc::c_int as isize) -
            2.0f32 * *quaternion.offset(3 as libc::c_int as isize) *
                *quaternion.offset(1 as libc::c_int as isize);
    (*out.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] =
        2.0f32 * *quaternion.offset(0 as libc::c_int as isize) *
            *quaternion.offset(1 as libc::c_int as isize) -
            2.0f32 * *quaternion.offset(3 as libc::c_int as isize) *
                *quaternion.offset(2 as libc::c_int as isize);
    (*out.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] =
        1.0f32 -
            2.0f32 * *quaternion.offset(0 as libc::c_int as isize) *
                *quaternion.offset(0 as libc::c_int as isize) -
            2.0f32 * *quaternion.offset(2 as libc::c_int as isize) *
                *quaternion.offset(2 as libc::c_int as isize);
    (*out.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize] =
        2.0f32 * *quaternion.offset(1 as libc::c_int as isize) *
            *quaternion.offset(2 as libc::c_int as isize) +
            2.0f32 * *quaternion.offset(3 as libc::c_int as isize) *
                *quaternion.offset(0 as libc::c_int as isize);
    (*out.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize] =
        2.0f32 * *quaternion.offset(0 as libc::c_int as isize) *
            *quaternion.offset(2 as libc::c_int as isize) +
            2.0f32 * *quaternion.offset(3 as libc::c_int as isize) *
                *quaternion.offset(1 as libc::c_int as isize);
    (*out.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize] =
        2.0f32 * *quaternion.offset(1 as libc::c_int as isize) *
            *quaternion.offset(2 as libc::c_int as isize) -
            2.0f32 * *quaternion.offset(3 as libc::c_int as isize) *
                *quaternion.offset(0 as libc::c_int as isize);
    (*out.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize] =
        1.0f32 -
            2.0f32 * *quaternion.offset(0 as libc::c_int as isize) *
                *quaternion.offset(0 as libc::c_int as isize) -
            2.0f32 * *quaternion.offset(1 as libc::c_int as isize) *
                *quaternion.offset(1 as libc::c_int as isize);
    (*out.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize] =
        *origin.offset(0 as libc::c_int as isize);
    (*out.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize] =
        *origin.offset(1 as libc::c_int as isize);
    (*out.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize] =
        *origin.offset(2 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn Matrix3x4_CreateFromEntity(mut out: *mut [vec_t; 4],
                                                    mut angles: *const vec_t,
                                                    mut origin: *const vec_t,
                                                    mut scale:
                                                        libc::c_float) {
    let mut angle: libc::c_float = 0.;
    let mut sr: libc::c_float = 0.;
    let mut sp: libc::c_float = 0.;
    let mut sy: libc::c_float = 0.;
    let mut cr: libc::c_float = 0.;
    let mut cp: libc::c_float = 0.;
    let mut cy: libc::c_float = 0.;
    if *angles.offset(2 as libc::c_int as isize) != 0. {
        angle =
            *angles.offset(1 as libc::c_int as isize) *
                ((3.14159265358979323846f64 *
                      2 as libc::c_int as libc::c_double) as libc::c_float /
                     360.0f32);
        SinCos(angle, &mut sy, &mut cy);
        angle =
            *angles.offset(0 as libc::c_int as isize) *
                ((3.14159265358979323846f64 *
                      2 as libc::c_int as libc::c_double) as libc::c_float /
                     360.0f32);
        SinCos(angle, &mut sp, &mut cp);
        angle =
            *angles.offset(2 as libc::c_int as isize) *
                ((3.14159265358979323846f64 *
                      2 as libc::c_int as libc::c_double) as libc::c_float /
                     360.0f32);
        SinCos(angle, &mut sr, &mut cr);
        (*out.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] =
            cp * cy * scale;
        (*out.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] =
            (sr * sp * cy + cr * -sy) * scale;
        (*out.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize] =
            (cr * sp * cy + -sr * -sy) * scale;
        (*out.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize] =
            *origin.offset(0 as libc::c_int as isize);
        (*out.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] =
            cp * sy * scale;
        (*out.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] =
            (sr * sp * sy + cr * cy) * scale;
        (*out.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize] =
            (cr * sp * sy + -sr * cy) * scale;
        (*out.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize] =
            *origin.offset(1 as libc::c_int as isize);
        (*out.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] =
            -sp * scale;
        (*out.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize] =
            sr * cp * scale;
        (*out.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize] =
            cr * cp * scale;
        (*out.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize] =
            *origin.offset(2 as libc::c_int as isize)
    } else if *angles.offset(0 as libc::c_int as isize) != 0. {
        angle =
            *angles.offset(1 as libc::c_int as isize) *
                ((3.14159265358979323846f64 *
                      2 as libc::c_int as libc::c_double) as libc::c_float /
                     360.0f32);
        SinCos(angle, &mut sy, &mut cy);
        angle =
            *angles.offset(0 as libc::c_int as isize) *
                ((3.14159265358979323846f64 *
                      2 as libc::c_int as libc::c_double) as libc::c_float /
                     360.0f32);
        SinCos(angle, &mut sp, &mut cp);
        (*out.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] =
            cp * cy * scale;
        (*out.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] =
            -sy * scale;
        (*out.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize] =
            sp * cy * scale;
        (*out.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize] =
            *origin.offset(0 as libc::c_int as isize);
        (*out.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] =
            cp * sy * scale;
        (*out.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] =
            cy * scale;
        (*out.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize] =
            sp * sy * scale;
        (*out.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize] =
            *origin.offset(1 as libc::c_int as isize);
        (*out.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] =
            -sp * scale;
        (*out.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize] =
            0.0f32;
        (*out.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize] =
            cp * scale;
        (*out.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize] =
            *origin.offset(2 as libc::c_int as isize)
    } else if *angles.offset(1 as libc::c_int as isize) != 0. {
        angle =
            *angles.offset(1 as libc::c_int as isize) *
                ((3.14159265358979323846f64 *
                      2 as libc::c_int as libc::c_double) as libc::c_float /
                     360.0f32);
        SinCos(angle, &mut sy, &mut cy);
        (*out.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] =
            cy * scale;
        (*out.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] =
            -sy * scale;
        (*out.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize] =
            0.0f32;
        (*out.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize] =
            *origin.offset(0 as libc::c_int as isize);
        (*out.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] =
            sy * scale;
        (*out.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] =
            cy * scale;
        (*out.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize] =
            0.0f32;
        (*out.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize] =
            *origin.offset(1 as libc::c_int as isize);
        (*out.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] =
            0.0f32;
        (*out.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize] =
            0.0f32;
        (*out.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize] =
            scale;
        (*out.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize] =
            *origin.offset(2 as libc::c_int as isize)
    } else {
        (*out.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] =
            scale;
        (*out.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] =
            0.0f32;
        (*out.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize] =
            0.0f32;
        (*out.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize] =
            *origin.offset(0 as libc::c_int as isize);
        (*out.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] =
            0.0f32;
        (*out.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] =
            scale;
        (*out.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize] =
            0.0f32;
        (*out.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize] =
            *origin.offset(1 as libc::c_int as isize);
        (*out.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] =
            0.0f32;
        (*out.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize] =
            0.0f32;
        (*out.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize] =
            scale;
        (*out.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize] =
            *origin.offset(2 as libc::c_int as isize)
    };
}
#[no_mangle]
pub unsafe extern "C" fn Matrix3x4_TransformPositivePlane(mut in_0:
                                                              *const [vec_t; 4],
                                                          mut normal:
                                                              *const vec_t,
                                                          mut d:
                                                              libc::c_float,
                                                          mut out: *mut vec_t,
                                                          mut dist:
                                                              *mut libc::c_float) {
    let mut scale: libc::c_float =
        __tg_sqrt((*in_0.offset(0 as libc::c_int as
                                    isize))[0 as libc::c_int as usize] *
                      (*in_0.offset(0 as libc::c_int as
                                        isize))[0 as libc::c_int as usize] +
                      (*in_0.offset(0 as libc::c_int as
                                        isize))[1 as libc::c_int as usize] *
                          (*in_0.offset(0 as libc::c_int as
                                            isize))[1 as libc::c_int as usize]
                      +
                      (*in_0.offset(0 as libc::c_int as
                                        isize))[2 as libc::c_int as usize] *
                          (*in_0.offset(0 as libc::c_int as
                                            isize))[2 as libc::c_int as
                                                        usize]);
    let mut iscale: libc::c_float = 1.0f32 / scale;
    *out.offset(0 as libc::c_int as isize) =
        (*normal.offset(0 as libc::c_int as isize) *
             (*in_0.offset(0 as libc::c_int as
                               isize))[0 as libc::c_int as usize] +
             *normal.offset(1 as libc::c_int as isize) *
                 (*in_0.offset(0 as libc::c_int as
                                   isize))[1 as libc::c_int as usize] +
             *normal.offset(2 as libc::c_int as isize) *
                 (*in_0.offset(0 as libc::c_int as
                                   isize))[2 as libc::c_int as usize]) *
            iscale;
    *out.offset(1 as libc::c_int as isize) =
        (*normal.offset(0 as libc::c_int as isize) *
             (*in_0.offset(1 as libc::c_int as
                               isize))[0 as libc::c_int as usize] +
             *normal.offset(1 as libc::c_int as isize) *
                 (*in_0.offset(1 as libc::c_int as
                                   isize))[1 as libc::c_int as usize] +
             *normal.offset(2 as libc::c_int as isize) *
                 (*in_0.offset(1 as libc::c_int as
                                   isize))[2 as libc::c_int as usize]) *
            iscale;
    *out.offset(2 as libc::c_int as isize) =
        (*normal.offset(0 as libc::c_int as isize) *
             (*in_0.offset(2 as libc::c_int as
                               isize))[0 as libc::c_int as usize] +
             *normal.offset(1 as libc::c_int as isize) *
                 (*in_0.offset(2 as libc::c_int as
                                   isize))[1 as libc::c_int as usize] +
             *normal.offset(2 as libc::c_int as isize) *
                 (*in_0.offset(2 as libc::c_int as
                                   isize))[2 as libc::c_int as usize]) *
            iscale;
    *dist =
        d * scale +
            (*out.offset(0 as libc::c_int as isize) *
                 (*in_0.offset(0 as libc::c_int as
                                   isize))[3 as libc::c_int as usize] +
                 *out.offset(1 as libc::c_int as isize) *
                     (*in_0.offset(1 as libc::c_int as
                                       isize))[3 as libc::c_int as usize] +
                 *out.offset(2 as libc::c_int as isize) *
                     (*in_0.offset(2 as libc::c_int as
                                       isize))[3 as libc::c_int as usize]);
}
#[no_mangle]
pub unsafe extern "C" fn Matrix3x4_Invert_Simple(mut out: *mut [vec_t; 4],
                                                 mut in1: *const [vec_t; 4]) {
    // we only support uniform scaling, so assume the first row is enough
	// (note the lack of sqrt here, because we're trying to undo the scaling,
	// this means multiplying by the inverse scale twice - squaring it, which
	// makes the sqrt a waste of time)
    let mut scale: libc::c_float =
        1.0f32 /
            ((*in1.offset(0 as libc::c_int as
                              isize))[0 as libc::c_int as usize] *
                 (*in1.offset(0 as libc::c_int as
                                  isize))[0 as libc::c_int as usize] +
                 (*in1.offset(0 as libc::c_int as
                                  isize))[1 as libc::c_int as usize] *
                     (*in1.offset(0 as libc::c_int as
                                      isize))[1 as libc::c_int as usize] +
                 (*in1.offset(0 as libc::c_int as
                                  isize))[2 as libc::c_int as usize] *
                     (*in1.offset(0 as libc::c_int as
                                      isize))[2 as libc::c_int as usize]);
    // invert the rotation by transposing and multiplying by the squared
	// recipricol of the input matrix scale as described above
    (*out.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (*in1.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] *
            scale;
    (*out.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] =
        (*in1.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] *
            scale;
    (*out.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize] =
        (*in1.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] *
            scale;
    (*out.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (*in1.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] *
            scale;
    (*out.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] =
        (*in1.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] *
            scale;
    (*out.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize] =
        (*in1.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize] *
            scale;
    (*out.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (*in1.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize] *
            scale;
    (*out.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize] =
        (*in1.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize] *
            scale;
    (*out.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize] =
        (*in1.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize] *
            scale;
    // invert the translate
    (*out.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize] =
        -((*in1.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize]
              *
              (*out.offset(0 as libc::c_int as
                               isize))[0 as libc::c_int as usize] +
              (*in1.offset(1 as libc::c_int as
                               isize))[3 as libc::c_int as usize] *
                  (*out.offset(0 as libc::c_int as
                                   isize))[1 as libc::c_int as usize] +
              (*in1.offset(2 as libc::c_int as
                               isize))[3 as libc::c_int as usize] *
                  (*out.offset(0 as libc::c_int as
                                   isize))[2 as libc::c_int as usize]);
    (*out.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize] =
        -((*in1.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize]
              *
              (*out.offset(1 as libc::c_int as
                               isize))[0 as libc::c_int as usize] +
              (*in1.offset(1 as libc::c_int as
                               isize))[3 as libc::c_int as usize] *
                  (*out.offset(1 as libc::c_int as
                                   isize))[1 as libc::c_int as usize] +
              (*in1.offset(2 as libc::c_int as
                               isize))[3 as libc::c_int as usize] *
                  (*out.offset(1 as libc::c_int as
                                   isize))[2 as libc::c_int as usize]);
    (*out.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize] =
        -((*in1.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize]
              *
              (*out.offset(2 as libc::c_int as
                               isize))[0 as libc::c_int as usize] +
              (*in1.offset(1 as libc::c_int as
                               isize))[3 as libc::c_int as usize] *
                  (*out.offset(2 as libc::c_int as
                                   isize))[1 as libc::c_int as usize] +
              (*in1.offset(2 as libc::c_int as
                               isize))[3 as libc::c_int as usize] *
                  (*out.offset(2 as libc::c_int as
                                   isize))[2 as libc::c_int as usize]);
}
#[no_mangle]
pub unsafe extern "C" fn Matrix3x4_Transpose(mut out: *mut [vec_t; 4],
                                             mut in1: *const [vec_t; 4]) {
    // transpose only rotational component
    (*out.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (*in1.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize];
    (*out.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] =
        (*in1.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize];
    (*out.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize] =
        (*in1.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize];
    (*out.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (*in1.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize];
    (*out.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] =
        (*in1.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize];
    (*out.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize] =
        (*in1.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize];
    (*out.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (*in1.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize];
    (*out.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize] =
        (*in1.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize];
    (*out.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize] =
        (*in1.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize];
    // copy origin
    (*out.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize] =
        (*in1.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize];
    (*out.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize] =
        (*in1.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize];
    (*out.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize] =
        (*in1.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize];
}
/*
==================
Matrix3x4_TransformAABB
==================
*/
#[no_mangle]
pub unsafe extern "C" fn Matrix3x4_TransformAABB(mut world: *const [vec_t; 4],
                                                 mut mins: *const vec_t,
                                                 mut maxs: *const vec_t,
                                                 mut absmin: *mut vec_t,
                                                 mut absmax: *mut vec_t) {
    let mut localCenter: vec3_t = [0.; 3]; // auto-transposed!
    let mut localExtents: vec3_t = [0.; 3];
    let mut worldCenter: vec3_t = [0.; 3];
    let mut worldExtents: vec3_t = [0.; 3];
    localCenter[0 as libc::c_int as usize] =
        (*mins.offset(0 as libc::c_int as isize) +
             *maxs.offset(0 as libc::c_int as isize)) * 0.5f32;
    localCenter[1 as libc::c_int as usize] =
        (*mins.offset(1 as libc::c_int as isize) +
             *maxs.offset(1 as libc::c_int as isize)) * 0.5f32;
    localCenter[2 as libc::c_int as usize] =
        (*mins.offset(2 as libc::c_int as isize) +
             *maxs.offset(2 as libc::c_int as isize)) * 0.5f32;
    localExtents[0 as libc::c_int as usize] =
        *maxs.offset(0 as libc::c_int as isize) -
            localCenter[0 as libc::c_int as usize];
    localExtents[1 as libc::c_int as usize] =
        *maxs.offset(1 as libc::c_int as isize) -
            localCenter[1 as libc::c_int as usize];
    localExtents[2 as libc::c_int as usize] =
        *maxs.offset(2 as libc::c_int as isize) -
            localCenter[2 as libc::c_int as usize];
    Matrix3x4_VectorTransform(world,
                              localCenter.as_mut_ptr() as
                                  *const libc::c_float,
                              worldCenter.as_mut_ptr());
    worldExtents[0 as libc::c_int as usize] =
        (abs((localExtents[0 as libc::c_int as usize] *
                  (*world.offset(0 as libc::c_int as
                                     isize))[0 as libc::c_int as usize]) as
                 libc::c_int) +
             abs((localExtents[1 as libc::c_int as usize] *
                      (*world.offset(0 as libc::c_int as
                                         isize))[1 as libc::c_int as usize])
                     as libc::c_int) +
             abs((localExtents[2 as libc::c_int as usize] *
                      (*world.offset(0 as libc::c_int as
                                         isize))[2 as libc::c_int as usize])
                     as libc::c_int)) as vec_t;
    worldExtents[1 as libc::c_int as usize] =
        (abs((localExtents[0 as libc::c_int as usize] *
                  (*world.offset(1 as libc::c_int as
                                     isize))[0 as libc::c_int as usize]) as
                 libc::c_int) +
             abs((localExtents[1 as libc::c_int as usize] *
                      (*world.offset(1 as libc::c_int as
                                         isize))[1 as libc::c_int as usize])
                     as libc::c_int) +
             abs((localExtents[2 as libc::c_int as usize] *
                      (*world.offset(1 as libc::c_int as
                                         isize))[2 as libc::c_int as usize])
                     as libc::c_int)) as vec_t;
    worldExtents[2 as libc::c_int as usize] =
        (abs((localExtents[0 as libc::c_int as usize] *
                  (*world.offset(2 as libc::c_int as
                                     isize))[0 as libc::c_int as usize]) as
                 libc::c_int) +
             abs((localExtents[1 as libc::c_int as usize] *
                      (*world.offset(2 as libc::c_int as
                                         isize))[1 as libc::c_int as usize])
                     as libc::c_int) +
             abs((localExtents[2 as libc::c_int as usize] *
                      (*world.offset(2 as libc::c_int as
                                         isize))[2 as libc::c_int as usize])
                     as libc::c_int)) as vec_t;
    *absmin.offset(0 as libc::c_int as isize) =
        worldCenter[0 as libc::c_int as usize] -
            worldExtents[0 as libc::c_int as usize];
    *absmin.offset(1 as libc::c_int as isize) =
        worldCenter[1 as libc::c_int as usize] -
            worldExtents[1 as libc::c_int as usize];
    *absmin.offset(2 as libc::c_int as isize) =
        worldCenter[2 as libc::c_int as usize] -
            worldExtents[2 as libc::c_int as usize];
    *absmax.offset(0 as libc::c_int as isize) =
        worldCenter[0 as libc::c_int as usize] +
            worldExtents[0 as libc::c_int as usize];
    *absmax.offset(1 as libc::c_int as isize) =
        worldCenter[1 as libc::c_int as usize] +
            worldExtents[1 as libc::c_int as usize];
    *absmax.offset(2 as libc::c_int as isize) =
        worldCenter[2 as libc::c_int as usize] +
            worldExtents[2 as libc::c_int as usize];
}
#[no_mangle]
pub static mut matrix4x4_identity: matrix4x4 =
    [[1 as libc::c_int as vec_t, 0 as libc::c_int as vec_t,
      0 as libc::c_int as vec_t, 0 as libc::c_int as vec_t],
     [0 as libc::c_int as vec_t, 1 as libc::c_int as vec_t,
      0 as libc::c_int as vec_t, 0 as libc::c_int as vec_t],
     [0 as libc::c_int as vec_t, 0 as libc::c_int as vec_t,
      1 as libc::c_int as vec_t, 0 as libc::c_int as vec_t],
     [0 as libc::c_int as vec_t, 0 as libc::c_int as vec_t,
      0 as libc::c_int as vec_t, 1 as libc::c_int as vec_t]];
/*
========================================================================

		Matrix4x4 operations

========================================================================
*/
#[no_mangle]
pub unsafe extern "C" fn Matrix4x4_VectorTransform(mut in_0:
                                                       *const [vec_t; 4],
                                                   mut v:
                                                       *const libc::c_float,
                                                   mut out:
                                                       *mut libc::c_float) {
    *out.offset(0 as libc::c_int as isize) =
        *v.offset(0 as libc::c_int as isize) *
            (*in_0.offset(0 as libc::c_int as
                              isize))[0 as libc::c_int as usize] +
            *v.offset(1 as libc::c_int as isize) *
                (*in_0.offset(0 as libc::c_int as
                                  isize))[1 as libc::c_int as usize] +
            *v.offset(2 as libc::c_int as isize) *
                (*in_0.offset(0 as libc::c_int as
                                  isize))[2 as libc::c_int as usize] +
            (*in_0.offset(0 as libc::c_int as
                              isize))[3 as libc::c_int as usize];
    *out.offset(1 as libc::c_int as isize) =
        *v.offset(0 as libc::c_int as isize) *
            (*in_0.offset(1 as libc::c_int as
                              isize))[0 as libc::c_int as usize] +
            *v.offset(1 as libc::c_int as isize) *
                (*in_0.offset(1 as libc::c_int as
                                  isize))[1 as libc::c_int as usize] +
            *v.offset(2 as libc::c_int as isize) *
                (*in_0.offset(1 as libc::c_int as
                                  isize))[2 as libc::c_int as usize] +
            (*in_0.offset(1 as libc::c_int as
                              isize))[3 as libc::c_int as usize];
    *out.offset(2 as libc::c_int as isize) =
        *v.offset(0 as libc::c_int as isize) *
            (*in_0.offset(2 as libc::c_int as
                              isize))[0 as libc::c_int as usize] +
            *v.offset(1 as libc::c_int as isize) *
                (*in_0.offset(2 as libc::c_int as
                                  isize))[1 as libc::c_int as usize] +
            *v.offset(2 as libc::c_int as isize) *
                (*in_0.offset(2 as libc::c_int as
                                  isize))[2 as libc::c_int as usize] +
            (*in_0.offset(2 as libc::c_int as
                              isize))[3 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn Matrix4x4_VectorITransform(mut in_0:
                                                        *const [vec_t; 4],
                                                    mut v:
                                                        *const libc::c_float,
                                                    mut out:
                                                        *mut libc::c_float) {
    let mut dir: vec3_t = [0.; 3];
    dir[0 as libc::c_int as usize] =
        *v.offset(0 as libc::c_int as isize) -
            (*in_0.offset(0 as libc::c_int as
                              isize))[3 as libc::c_int as usize];
    dir[1 as libc::c_int as usize] =
        *v.offset(1 as libc::c_int as isize) -
            (*in_0.offset(1 as libc::c_int as
                              isize))[3 as libc::c_int as usize];
    dir[2 as libc::c_int as usize] =
        *v.offset(2 as libc::c_int as isize) -
            (*in_0.offset(2 as libc::c_int as
                              isize))[3 as libc::c_int as usize];
    *out.offset(0 as libc::c_int as isize) =
        dir[0 as libc::c_int as usize] *
            (*in_0.offset(0 as libc::c_int as
                              isize))[0 as libc::c_int as usize] +
            dir[1 as libc::c_int as usize] *
                (*in_0.offset(1 as libc::c_int as
                                  isize))[0 as libc::c_int as usize] +
            dir[2 as libc::c_int as usize] *
                (*in_0.offset(2 as libc::c_int as
                                  isize))[0 as libc::c_int as usize];
    *out.offset(1 as libc::c_int as isize) =
        dir[0 as libc::c_int as usize] *
            (*in_0.offset(0 as libc::c_int as
                              isize))[1 as libc::c_int as usize] +
            dir[1 as libc::c_int as usize] *
                (*in_0.offset(1 as libc::c_int as
                                  isize))[1 as libc::c_int as usize] +
            dir[2 as libc::c_int as usize] *
                (*in_0.offset(2 as libc::c_int as
                                  isize))[1 as libc::c_int as usize];
    *out.offset(2 as libc::c_int as isize) =
        dir[0 as libc::c_int as usize] *
            (*in_0.offset(0 as libc::c_int as
                              isize))[2 as libc::c_int as usize] +
            dir[1 as libc::c_int as usize] *
                (*in_0.offset(1 as libc::c_int as
                                  isize))[2 as libc::c_int as usize] +
            dir[2 as libc::c_int as usize] *
                (*in_0.offset(2 as libc::c_int as
                                  isize))[2 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn Matrix4x4_VectorRotate(mut in_0: *const [vec_t; 4],
                                                mut v: *const libc::c_float,
                                                mut out: *mut libc::c_float) {
    *out.offset(0 as libc::c_int as isize) =
        *v.offset(0 as libc::c_int as isize) *
            (*in_0.offset(0 as libc::c_int as
                              isize))[0 as libc::c_int as usize] +
            *v.offset(1 as libc::c_int as isize) *
                (*in_0.offset(0 as libc::c_int as
                                  isize))[1 as libc::c_int as usize] +
            *v.offset(2 as libc::c_int as isize) *
                (*in_0.offset(0 as libc::c_int as
                                  isize))[2 as libc::c_int as usize];
    *out.offset(1 as libc::c_int as isize) =
        *v.offset(0 as libc::c_int as isize) *
            (*in_0.offset(1 as libc::c_int as
                              isize))[0 as libc::c_int as usize] +
            *v.offset(1 as libc::c_int as isize) *
                (*in_0.offset(1 as libc::c_int as
                                  isize))[1 as libc::c_int as usize] +
            *v.offset(2 as libc::c_int as isize) *
                (*in_0.offset(1 as libc::c_int as
                                  isize))[2 as libc::c_int as usize];
    *out.offset(2 as libc::c_int as isize) =
        *v.offset(0 as libc::c_int as isize) *
            (*in_0.offset(2 as libc::c_int as
                              isize))[0 as libc::c_int as usize] +
            *v.offset(1 as libc::c_int as isize) *
                (*in_0.offset(2 as libc::c_int as
                                  isize))[1 as libc::c_int as usize] +
            *v.offset(2 as libc::c_int as isize) *
                (*in_0.offset(2 as libc::c_int as
                                  isize))[2 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn Matrix4x4_VectorIRotate(mut in_0: *const [vec_t; 4],
                                                 mut v: *const libc::c_float,
                                                 mut out:
                                                     *mut libc::c_float) {
    *out.offset(0 as libc::c_int as isize) =
        *v.offset(0 as libc::c_int as isize) *
            (*in_0.offset(0 as libc::c_int as
                              isize))[0 as libc::c_int as usize] +
            *v.offset(1 as libc::c_int as isize) *
                (*in_0.offset(1 as libc::c_int as
                                  isize))[0 as libc::c_int as usize] +
            *v.offset(2 as libc::c_int as isize) *
                (*in_0.offset(2 as libc::c_int as
                                  isize))[0 as libc::c_int as usize];
    *out.offset(1 as libc::c_int as isize) =
        *v.offset(0 as libc::c_int as isize) *
            (*in_0.offset(0 as libc::c_int as
                              isize))[1 as libc::c_int as usize] +
            *v.offset(1 as libc::c_int as isize) *
                (*in_0.offset(1 as libc::c_int as
                                  isize))[1 as libc::c_int as usize] +
            *v.offset(2 as libc::c_int as isize) *
                (*in_0.offset(2 as libc::c_int as
                                  isize))[1 as libc::c_int as usize];
    *out.offset(2 as libc::c_int as isize) =
        *v.offset(0 as libc::c_int as isize) *
            (*in_0.offset(0 as libc::c_int as
                              isize))[2 as libc::c_int as usize] +
            *v.offset(1 as libc::c_int as isize) *
                (*in_0.offset(1 as libc::c_int as
                                  isize))[2 as libc::c_int as usize] +
            *v.offset(2 as libc::c_int as isize) *
                (*in_0.offset(2 as libc::c_int as
                                  isize))[2 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn Matrix4x4_ConcatTransforms(mut out: *mut [vec_t; 4],
                                                    mut in1:
                                                        *const [vec_t; 4],
                                                    mut in2:
                                                        *const [vec_t; 4]) {
    (*out.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (*in1.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] *
            (*in2.offset(0 as libc::c_int as
                             isize))[0 as libc::c_int as usize] +
            (*in1.offset(0 as libc::c_int as
                             isize))[1 as libc::c_int as usize] *
                (*in2.offset(1 as libc::c_int as
                                 isize))[0 as libc::c_int as usize] +
            (*in1.offset(0 as libc::c_int as
                             isize))[2 as libc::c_int as usize] *
                (*in2.offset(2 as libc::c_int as
                                 isize))[0 as libc::c_int as usize];
    (*out.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] =
        (*in1.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] *
            (*in2.offset(0 as libc::c_int as
                             isize))[1 as libc::c_int as usize] +
            (*in1.offset(0 as libc::c_int as
                             isize))[1 as libc::c_int as usize] *
                (*in2.offset(1 as libc::c_int as
                                 isize))[1 as libc::c_int as usize] +
            (*in1.offset(0 as libc::c_int as
                             isize))[2 as libc::c_int as usize] *
                (*in2.offset(2 as libc::c_int as
                                 isize))[1 as libc::c_int as usize];
    (*out.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize] =
        (*in1.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] *
            (*in2.offset(0 as libc::c_int as
                             isize))[2 as libc::c_int as usize] +
            (*in1.offset(0 as libc::c_int as
                             isize))[1 as libc::c_int as usize] *
                (*in2.offset(1 as libc::c_int as
                                 isize))[2 as libc::c_int as usize] +
            (*in1.offset(0 as libc::c_int as
                             isize))[2 as libc::c_int as usize] *
                (*in2.offset(2 as libc::c_int as
                                 isize))[2 as libc::c_int as usize];
    (*out.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize] =
        (*in1.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] *
            (*in2.offset(0 as libc::c_int as
                             isize))[3 as libc::c_int as usize] +
            (*in1.offset(0 as libc::c_int as
                             isize))[1 as libc::c_int as usize] *
                (*in2.offset(1 as libc::c_int as
                                 isize))[3 as libc::c_int as usize] +
            (*in1.offset(0 as libc::c_int as
                             isize))[2 as libc::c_int as usize] *
                (*in2.offset(2 as libc::c_int as
                                 isize))[3 as libc::c_int as usize] +
            (*in1.offset(0 as libc::c_int as
                             isize))[3 as libc::c_int as usize];
    (*out.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (*in1.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] *
            (*in2.offset(0 as libc::c_int as
                             isize))[0 as libc::c_int as usize] +
            (*in1.offset(1 as libc::c_int as
                             isize))[1 as libc::c_int as usize] *
                (*in2.offset(1 as libc::c_int as
                                 isize))[0 as libc::c_int as usize] +
            (*in1.offset(1 as libc::c_int as
                             isize))[2 as libc::c_int as usize] *
                (*in2.offset(2 as libc::c_int as
                                 isize))[0 as libc::c_int as usize];
    (*out.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] =
        (*in1.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] *
            (*in2.offset(0 as libc::c_int as
                             isize))[1 as libc::c_int as usize] +
            (*in1.offset(1 as libc::c_int as
                             isize))[1 as libc::c_int as usize] *
                (*in2.offset(1 as libc::c_int as
                                 isize))[1 as libc::c_int as usize] +
            (*in1.offset(1 as libc::c_int as
                             isize))[2 as libc::c_int as usize] *
                (*in2.offset(2 as libc::c_int as
                                 isize))[1 as libc::c_int as usize];
    (*out.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize] =
        (*in1.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] *
            (*in2.offset(0 as libc::c_int as
                             isize))[2 as libc::c_int as usize] +
            (*in1.offset(1 as libc::c_int as
                             isize))[1 as libc::c_int as usize] *
                (*in2.offset(1 as libc::c_int as
                                 isize))[2 as libc::c_int as usize] +
            (*in1.offset(1 as libc::c_int as
                             isize))[2 as libc::c_int as usize] *
                (*in2.offset(2 as libc::c_int as
                                 isize))[2 as libc::c_int as usize];
    (*out.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize] =
        (*in1.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] *
            (*in2.offset(0 as libc::c_int as
                             isize))[3 as libc::c_int as usize] +
            (*in1.offset(1 as libc::c_int as
                             isize))[1 as libc::c_int as usize] *
                (*in2.offset(1 as libc::c_int as
                                 isize))[3 as libc::c_int as usize] +
            (*in1.offset(1 as libc::c_int as
                             isize))[2 as libc::c_int as usize] *
                (*in2.offset(2 as libc::c_int as
                                 isize))[3 as libc::c_int as usize] +
            (*in1.offset(1 as libc::c_int as
                             isize))[3 as libc::c_int as usize];
    (*out.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (*in1.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] *
            (*in2.offset(0 as libc::c_int as
                             isize))[0 as libc::c_int as usize] +
            (*in1.offset(2 as libc::c_int as
                             isize))[1 as libc::c_int as usize] *
                (*in2.offset(1 as libc::c_int as
                                 isize))[0 as libc::c_int as usize] +
            (*in1.offset(2 as libc::c_int as
                             isize))[2 as libc::c_int as usize] *
                (*in2.offset(2 as libc::c_int as
                                 isize))[0 as libc::c_int as usize];
    (*out.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize] =
        (*in1.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] *
            (*in2.offset(0 as libc::c_int as
                             isize))[1 as libc::c_int as usize] +
            (*in1.offset(2 as libc::c_int as
                             isize))[1 as libc::c_int as usize] *
                (*in2.offset(1 as libc::c_int as
                                 isize))[1 as libc::c_int as usize] +
            (*in1.offset(2 as libc::c_int as
                             isize))[2 as libc::c_int as usize] *
                (*in2.offset(2 as libc::c_int as
                                 isize))[1 as libc::c_int as usize];
    (*out.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize] =
        (*in1.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] *
            (*in2.offset(0 as libc::c_int as
                             isize))[2 as libc::c_int as usize] +
            (*in1.offset(2 as libc::c_int as
                             isize))[1 as libc::c_int as usize] *
                (*in2.offset(1 as libc::c_int as
                                 isize))[2 as libc::c_int as usize] +
            (*in1.offset(2 as libc::c_int as
                             isize))[2 as libc::c_int as usize] *
                (*in2.offset(2 as libc::c_int as
                                 isize))[2 as libc::c_int as usize];
    (*out.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize] =
        (*in1.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] *
            (*in2.offset(0 as libc::c_int as
                             isize))[3 as libc::c_int as usize] +
            (*in1.offset(2 as libc::c_int as
                             isize))[1 as libc::c_int as usize] *
                (*in2.offset(1 as libc::c_int as
                                 isize))[3 as libc::c_int as usize] +
            (*in1.offset(2 as libc::c_int as
                             isize))[2 as libc::c_int as usize] *
                (*in2.offset(2 as libc::c_int as
                                 isize))[3 as libc::c_int as usize] +
            (*in1.offset(2 as libc::c_int as
                             isize))[3 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn Matrix4x4_SetOrigin(mut out: *mut [vec_t; 4],
                                             mut x: libc::c_float,
                                             mut y: libc::c_float,
                                             mut z: libc::c_float) {
    (*out.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize] = x;
    (*out.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize] = y;
    (*out.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize] = z;
}
#[no_mangle]
pub unsafe extern "C" fn Matrix4x4_OriginFromMatrix(mut in_0:
                                                        *const [vec_t; 4],
                                                    mut out:
                                                        *mut libc::c_float) {
    *out.offset(0 as libc::c_int as isize) =
        (*in_0.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize];
    *out.offset(1 as libc::c_int as isize) =
        (*in_0.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize];
    *out.offset(2 as libc::c_int as isize) =
        (*in_0.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn Matrix4x4_FromOriginQuat(mut out: *mut [vec_t; 4],
                                                  mut quaternion:
                                                      *const vec_t,
                                                  mut origin: *const vec_t) {
    (*out.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] =
        1.0f32 -
            2.0f32 * *quaternion.offset(1 as libc::c_int as isize) *
                *quaternion.offset(1 as libc::c_int as isize) -
            2.0f32 * *quaternion.offset(2 as libc::c_int as isize) *
                *quaternion.offset(2 as libc::c_int as isize);
    (*out.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] =
        2.0f32 * *quaternion.offset(0 as libc::c_int as isize) *
            *quaternion.offset(1 as libc::c_int as isize) +
            2.0f32 * *quaternion.offset(3 as libc::c_int as isize) *
                *quaternion.offset(2 as libc::c_int as isize);
    (*out.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] =
        2.0f32 * *quaternion.offset(0 as libc::c_int as isize) *
            *quaternion.offset(2 as libc::c_int as isize) -
            2.0f32 * *quaternion.offset(3 as libc::c_int as isize) *
                *quaternion.offset(1 as libc::c_int as isize);
    (*out.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize] =
        *origin.offset(0 as libc::c_int as isize);
    (*out.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] =
        2.0f32 * *quaternion.offset(0 as libc::c_int as isize) *
            *quaternion.offset(1 as libc::c_int as isize) -
            2.0f32 * *quaternion.offset(3 as libc::c_int as isize) *
                *quaternion.offset(2 as libc::c_int as isize);
    (*out.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] =
        1.0f32 -
            2.0f32 * *quaternion.offset(0 as libc::c_int as isize) *
                *quaternion.offset(0 as libc::c_int as isize) -
            2.0f32 * *quaternion.offset(2 as libc::c_int as isize) *
                *quaternion.offset(2 as libc::c_int as isize);
    (*out.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize] =
        2.0f32 * *quaternion.offset(1 as libc::c_int as isize) *
            *quaternion.offset(2 as libc::c_int as isize) +
            2.0f32 * *quaternion.offset(3 as libc::c_int as isize) *
                *quaternion.offset(0 as libc::c_int as isize);
    (*out.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize] =
        *origin.offset(1 as libc::c_int as isize);
    (*out.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize] =
        2.0f32 * *quaternion.offset(0 as libc::c_int as isize) *
            *quaternion.offset(2 as libc::c_int as isize) +
            2.0f32 * *quaternion.offset(3 as libc::c_int as isize) *
                *quaternion.offset(1 as libc::c_int as isize);
    (*out.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize] =
        2.0f32 * *quaternion.offset(1 as libc::c_int as isize) *
            *quaternion.offset(2 as libc::c_int as isize) -
            2.0f32 * *quaternion.offset(3 as libc::c_int as isize) *
                *quaternion.offset(0 as libc::c_int as isize);
    (*out.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize] =
        1.0f32 -
            2.0f32 * *quaternion.offset(0 as libc::c_int as isize) *
                *quaternion.offset(0 as libc::c_int as isize) -
            2.0f32 * *quaternion.offset(1 as libc::c_int as isize) *
                *quaternion.offset(1 as libc::c_int as isize);
    (*out.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize] =
        *origin.offset(2 as libc::c_int as isize);
    (*out.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize] =
        0.0f32;
    (*out.offset(3 as libc::c_int as isize))[1 as libc::c_int as usize] =
        0.0f32;
    (*out.offset(3 as libc::c_int as isize))[2 as libc::c_int as usize] =
        0.0f32;
    (*out.offset(3 as libc::c_int as isize))[3 as libc::c_int as usize] =
        1.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn Matrix4x4_CreateFromEntity(mut out: *mut [vec_t; 4],
                                                    mut angles: *const vec_t,
                                                    mut origin: *const vec_t,
                                                    mut scale:
                                                        libc::c_float) {
    let mut angle: libc::c_float = 0.;
    let mut sr: libc::c_float = 0.;
    let mut sp: libc::c_float = 0.;
    let mut sy: libc::c_float = 0.;
    let mut cr: libc::c_float = 0.;
    let mut cp: libc::c_float = 0.;
    let mut cy: libc::c_float = 0.;
    if *angles.offset(2 as libc::c_int as isize) != 0. {
        angle =
            *angles.offset(1 as libc::c_int as isize) *
                ((3.14159265358979323846f64 *
                      2 as libc::c_int as libc::c_double) as libc::c_float /
                     360.0f32);
        SinCos(angle, &mut sy, &mut cy);
        angle =
            *angles.offset(0 as libc::c_int as isize) *
                ((3.14159265358979323846f64 *
                      2 as libc::c_int as libc::c_double) as libc::c_float /
                     360.0f32);
        SinCos(angle, &mut sp, &mut cp);
        angle =
            *angles.offset(2 as libc::c_int as isize) *
                ((3.14159265358979323846f64 *
                      2 as libc::c_int as libc::c_double) as libc::c_float /
                     360.0f32);
        SinCos(angle, &mut sr, &mut cr);
        (*out.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] =
            cp * cy * scale;
        (*out.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] =
            (sr * sp * cy + cr * -sy) * scale;
        (*out.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize] =
            (cr * sp * cy + -sr * -sy) * scale;
        (*out.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize] =
            *origin.offset(0 as libc::c_int as isize);
        (*out.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] =
            cp * sy * scale;
        (*out.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] =
            (sr * sp * sy + cr * cy) * scale;
        (*out.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize] =
            (cr * sp * sy + -sr * cy) * scale;
        (*out.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize] =
            *origin.offset(1 as libc::c_int as isize);
        (*out.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] =
            -sp * scale;
        (*out.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize] =
            sr * cp * scale;
        (*out.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize] =
            cr * cp * scale;
        (*out.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize] =
            *origin.offset(2 as libc::c_int as isize);
        (*out.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize] =
            0.0f32;
        (*out.offset(3 as libc::c_int as isize))[1 as libc::c_int as usize] =
            0.0f32;
        (*out.offset(3 as libc::c_int as isize))[2 as libc::c_int as usize] =
            0.0f32;
        (*out.offset(3 as libc::c_int as isize))[3 as libc::c_int as usize] =
            1.0f32
    } else if *angles.offset(0 as libc::c_int as isize) != 0. {
        angle =
            *angles.offset(1 as libc::c_int as isize) *
                ((3.14159265358979323846f64 *
                      2 as libc::c_int as libc::c_double) as libc::c_float /
                     360.0f32);
        SinCos(angle, &mut sy, &mut cy);
        angle =
            *angles.offset(0 as libc::c_int as isize) *
                ((3.14159265358979323846f64 *
                      2 as libc::c_int as libc::c_double) as libc::c_float /
                     360.0f32);
        SinCos(angle, &mut sp, &mut cp);
        (*out.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] =
            cp * cy * scale;
        (*out.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] =
            -sy * scale;
        (*out.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize] =
            sp * cy * scale;
        (*out.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize] =
            *origin.offset(0 as libc::c_int as isize);
        (*out.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] =
            cp * sy * scale;
        (*out.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] =
            cy * scale;
        (*out.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize] =
            sp * sy * scale;
        (*out.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize] =
            *origin.offset(1 as libc::c_int as isize);
        (*out.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] =
            -sp * scale;
        (*out.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize] =
            0.0f32;
        (*out.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize] =
            cp * scale;
        (*out.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize] =
            *origin.offset(2 as libc::c_int as isize);
        (*out.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize] =
            0.0f32;
        (*out.offset(3 as libc::c_int as isize))[1 as libc::c_int as usize] =
            0.0f32;
        (*out.offset(3 as libc::c_int as isize))[2 as libc::c_int as usize] =
            0.0f32;
        (*out.offset(3 as libc::c_int as isize))[3 as libc::c_int as usize] =
            1.0f32
    } else if *angles.offset(1 as libc::c_int as isize) != 0. {
        angle =
            *angles.offset(1 as libc::c_int as isize) *
                ((3.14159265358979323846f64 *
                      2 as libc::c_int as libc::c_double) as libc::c_float /
                     360.0f32);
        SinCos(angle, &mut sy, &mut cy);
        (*out.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] =
            cy * scale;
        (*out.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] =
            -sy * scale;
        (*out.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize] =
            0.0f32;
        (*out.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize] =
            *origin.offset(0 as libc::c_int as isize);
        (*out.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] =
            sy * scale;
        (*out.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] =
            cy * scale;
        (*out.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize] =
            0.0f32;
        (*out.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize] =
            *origin.offset(1 as libc::c_int as isize);
        (*out.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] =
            0.0f32;
        (*out.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize] =
            0.0f32;
        (*out.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize] =
            scale;
        (*out.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize] =
            *origin.offset(2 as libc::c_int as isize);
        (*out.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize] =
            0.0f32;
        (*out.offset(3 as libc::c_int as isize))[1 as libc::c_int as usize] =
            0.0f32;
        (*out.offset(3 as libc::c_int as isize))[2 as libc::c_int as usize] =
            0.0f32;
        (*out.offset(3 as libc::c_int as isize))[3 as libc::c_int as usize] =
            1.0f32
    } else {
        (*out.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] =
            scale;
        (*out.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] =
            0.0f32;
        (*out.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize] =
            0.0f32;
        (*out.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize] =
            *origin.offset(0 as libc::c_int as isize);
        (*out.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] =
            0.0f32;
        (*out.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] =
            scale;
        (*out.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize] =
            0.0f32;
        (*out.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize] =
            *origin.offset(1 as libc::c_int as isize);
        (*out.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] =
            0.0f32;
        (*out.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize] =
            0.0f32;
        (*out.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize] =
            scale;
        (*out.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize] =
            *origin.offset(2 as libc::c_int as isize);
        (*out.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize] =
            0.0f32;
        (*out.offset(3 as libc::c_int as isize))[1 as libc::c_int as usize] =
            0.0f32;
        (*out.offset(3 as libc::c_int as isize))[2 as libc::c_int as usize] =
            0.0f32;
        (*out.offset(3 as libc::c_int as isize))[3 as libc::c_int as usize] =
            1.0f32
    };
}
#[no_mangle]
pub unsafe extern "C" fn Matrix4x4_ConvertToEntity(mut in_0:
                                                       *const [vec_t; 4],
                                                   mut angles: *mut vec_t,
                                                   mut origin: *mut vec_t) {
    let mut xyDist: libc::c_float =
        __tg_sqrt((*in_0.offset(0 as libc::c_int as
                                    isize))[0 as libc::c_int as usize] *
                      (*in_0.offset(0 as libc::c_int as
                                        isize))[0 as libc::c_int as usize] +
                      (*in_0.offset(1 as libc::c_int as
                                        isize))[0 as libc::c_int as usize] *
                          (*in_0.offset(1 as libc::c_int as
                                            isize))[0 as libc::c_int as
                                                        usize]);
    // enough here to get angles?
    if xyDist > 0.001f32 {
        *angles.offset(0 as libc::c_int as isize) =
            __tg_atan2(-(*in_0.offset(2 as libc::c_int as
                                          isize))[0 as libc::c_int as usize],
                       xyDist) *
                (180.0f32 / 3.14159265358979323846f64 as libc::c_float);
        *angles.offset(1 as libc::c_int as isize) =
            __tg_atan2((*in_0.offset(1 as libc::c_int as
                                         isize))[0 as libc::c_int as usize],
                       (*in_0.offset(0 as libc::c_int as
                                         isize))[0 as libc::c_int as usize]) *
                (180.0f32 / 3.14159265358979323846f64 as libc::c_float);
        *angles.offset(2 as libc::c_int as isize) =
            __tg_atan2((*in_0.offset(2 as libc::c_int as
                                         isize))[1 as libc::c_int as usize],
                       (*in_0.offset(2 as libc::c_int as
                                         isize))[2 as libc::c_int as usize]) *
                (180.0f32 / 3.14159265358979323846f64 as libc::c_float)
    } else {
        // forward is mostly Z, gimbal lock
        *angles.offset(0 as libc::c_int as isize) =
            __tg_atan2(-(*in_0.offset(2 as libc::c_int as
                                          isize))[0 as libc::c_int as usize],
                       xyDist) *
                (180.0f32 / 3.14159265358979323846f64 as libc::c_float);
        *angles.offset(1 as libc::c_int as isize) =
            __tg_atan2(-(*in_0.offset(0 as libc::c_int as
                                          isize))[1 as libc::c_int as usize],
                       (*in_0.offset(1 as libc::c_int as
                                         isize))[1 as libc::c_int as usize]) *
                (180.0f32 / 3.14159265358979323846f64 as libc::c_float);
        *angles.offset(2 as libc::c_int as isize) = 0.0f32
    }
    *origin.offset(0 as libc::c_int as isize) =
        (*in_0.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize];
    *origin.offset(1 as libc::c_int as isize) =
        (*in_0.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize];
    *origin.offset(2 as libc::c_int as isize) =
        (*in_0.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn Matrix4x4_TransformPositivePlane(mut in_0:
                                                              *const [vec_t; 4],
                                                          mut normal:
                                                              *const vec_t,
                                                          mut d:
                                                              libc::c_float,
                                                          mut out: *mut vec_t,
                                                          mut dist:
                                                              *mut libc::c_float) {
    let mut scale: libc::c_float =
        __tg_sqrt((*in_0.offset(0 as libc::c_int as
                                    isize))[0 as libc::c_int as usize] *
                      (*in_0.offset(0 as libc::c_int as
                                        isize))[0 as libc::c_int as usize] +
                      (*in_0.offset(0 as libc::c_int as
                                        isize))[1 as libc::c_int as usize] *
                          (*in_0.offset(0 as libc::c_int as
                                            isize))[1 as libc::c_int as usize]
                      +
                      (*in_0.offset(0 as libc::c_int as
                                        isize))[2 as libc::c_int as usize] *
                          (*in_0.offset(0 as libc::c_int as
                                            isize))[2 as libc::c_int as
                                                        usize]);
    let mut iscale: libc::c_float = 1.0f32 / scale;
    *out.offset(0 as libc::c_int as isize) =
        (*normal.offset(0 as libc::c_int as isize) *
             (*in_0.offset(0 as libc::c_int as
                               isize))[0 as libc::c_int as usize] +
             *normal.offset(1 as libc::c_int as isize) *
                 (*in_0.offset(0 as libc::c_int as
                                   isize))[1 as libc::c_int as usize] +
             *normal.offset(2 as libc::c_int as isize) *
                 (*in_0.offset(0 as libc::c_int as
                                   isize))[2 as libc::c_int as usize]) *
            iscale;
    *out.offset(1 as libc::c_int as isize) =
        (*normal.offset(0 as libc::c_int as isize) *
             (*in_0.offset(1 as libc::c_int as
                               isize))[0 as libc::c_int as usize] +
             *normal.offset(1 as libc::c_int as isize) *
                 (*in_0.offset(1 as libc::c_int as
                                   isize))[1 as libc::c_int as usize] +
             *normal.offset(2 as libc::c_int as isize) *
                 (*in_0.offset(1 as libc::c_int as
                                   isize))[2 as libc::c_int as usize]) *
            iscale;
    *out.offset(2 as libc::c_int as isize) =
        (*normal.offset(0 as libc::c_int as isize) *
             (*in_0.offset(2 as libc::c_int as
                               isize))[0 as libc::c_int as usize] +
             *normal.offset(1 as libc::c_int as isize) *
                 (*in_0.offset(2 as libc::c_int as
                                   isize))[1 as libc::c_int as usize] +
             *normal.offset(2 as libc::c_int as isize) *
                 (*in_0.offset(2 as libc::c_int as
                                   isize))[2 as libc::c_int as usize]) *
            iscale;
    *dist =
        d * scale +
            (*out.offset(0 as libc::c_int as isize) *
                 (*in_0.offset(0 as libc::c_int as
                                   isize))[3 as libc::c_int as usize] +
                 *out.offset(1 as libc::c_int as isize) *
                     (*in_0.offset(1 as libc::c_int as
                                       isize))[3 as libc::c_int as usize] +
                 *out.offset(2 as libc::c_int as isize) *
                     (*in_0.offset(2 as libc::c_int as
                                       isize))[3 as libc::c_int as usize]);
}
#[no_mangle]
pub unsafe extern "C" fn Matrix4x4_TransformStandardPlane(mut in_0:
                                                              *const [vec_t; 4],
                                                          mut normal:
                                                              *const vec_t,
                                                          mut d:
                                                              libc::c_float,
                                                          mut out: *mut vec_t,
                                                          mut dist:
                                                              *mut libc::c_float) {
    let mut scale: libc::c_float =
        __tg_sqrt((*in_0.offset(0 as libc::c_int as
                                    isize))[0 as libc::c_int as usize] *
                      (*in_0.offset(0 as libc::c_int as
                                        isize))[0 as libc::c_int as usize] +
                      (*in_0.offset(0 as libc::c_int as
                                        isize))[1 as libc::c_int as usize] *
                          (*in_0.offset(0 as libc::c_int as
                                            isize))[1 as libc::c_int as usize]
                      +
                      (*in_0.offset(0 as libc::c_int as
                                        isize))[2 as libc::c_int as usize] *
                          (*in_0.offset(0 as libc::c_int as
                                            isize))[2 as libc::c_int as
                                                        usize]);
    let mut iscale: libc::c_float = 1.0f32 / scale;
    *out.offset(0 as libc::c_int as isize) =
        (*normal.offset(0 as libc::c_int as isize) *
             (*in_0.offset(0 as libc::c_int as
                               isize))[0 as libc::c_int as usize] +
             *normal.offset(1 as libc::c_int as isize) *
                 (*in_0.offset(0 as libc::c_int as
                                   isize))[1 as libc::c_int as usize] +
             *normal.offset(2 as libc::c_int as isize) *
                 (*in_0.offset(0 as libc::c_int as
                                   isize))[2 as libc::c_int as usize]) *
            iscale;
    *out.offset(1 as libc::c_int as isize) =
        (*normal.offset(0 as libc::c_int as isize) *
             (*in_0.offset(1 as libc::c_int as
                               isize))[0 as libc::c_int as usize] +
             *normal.offset(1 as libc::c_int as isize) *
                 (*in_0.offset(1 as libc::c_int as
                                   isize))[1 as libc::c_int as usize] +
             *normal.offset(2 as libc::c_int as isize) *
                 (*in_0.offset(1 as libc::c_int as
                                   isize))[2 as libc::c_int as usize]) *
            iscale;
    *out.offset(2 as libc::c_int as isize) =
        (*normal.offset(0 as libc::c_int as isize) *
             (*in_0.offset(2 as libc::c_int as
                               isize))[0 as libc::c_int as usize] +
             *normal.offset(1 as libc::c_int as isize) *
                 (*in_0.offset(2 as libc::c_int as
                                   isize))[1 as libc::c_int as usize] +
             *normal.offset(2 as libc::c_int as isize) *
                 (*in_0.offset(2 as libc::c_int as
                                   isize))[2 as libc::c_int as usize]) *
            iscale;
    *dist =
        d * scale -
            (*out.offset(0 as libc::c_int as isize) *
                 (*in_0.offset(0 as libc::c_int as
                                   isize))[3 as libc::c_int as usize] +
                 *out.offset(1 as libc::c_int as isize) *
                     (*in_0.offset(1 as libc::c_int as
                                       isize))[3 as libc::c_int as usize] +
                 *out.offset(2 as libc::c_int as isize) *
                     (*in_0.offset(2 as libc::c_int as
                                       isize))[3 as libc::c_int as usize]);
}
#[no_mangle]
pub unsafe extern "C" fn Matrix4x4_Invert_Simple(mut out: *mut [vec_t; 4],
                                                 mut in1: *const [vec_t; 4]) {
    // we only support uniform scaling, so assume the first row is enough
	// (note the lack of sqrt here, because we're trying to undo the scaling,
	// this means multiplying by the inverse scale twice - squaring it, which
	// makes the sqrt a waste of time)
    let mut scale: libc::c_float =
        1.0f32 /
            ((*in1.offset(0 as libc::c_int as
                              isize))[0 as libc::c_int as usize] *
                 (*in1.offset(0 as libc::c_int as
                                  isize))[0 as libc::c_int as usize] +
                 (*in1.offset(0 as libc::c_int as
                                  isize))[1 as libc::c_int as usize] *
                     (*in1.offset(0 as libc::c_int as
                                      isize))[1 as libc::c_int as usize] +
                 (*in1.offset(0 as libc::c_int as
                                  isize))[2 as libc::c_int as usize] *
                     (*in1.offset(0 as libc::c_int as
                                      isize))[2 as libc::c_int as usize]);
    // invert the rotation by transposing and multiplying by the squared
	// recipricol of the input matrix scale as described above
    (*out.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (*in1.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] *
            scale;
    (*out.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] =
        (*in1.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] *
            scale;
    (*out.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize] =
        (*in1.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] *
            scale;
    (*out.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (*in1.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] *
            scale;
    (*out.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] =
        (*in1.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] *
            scale;
    (*out.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize] =
        (*in1.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize] *
            scale;
    (*out.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (*in1.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize] *
            scale;
    (*out.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize] =
        (*in1.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize] *
            scale;
    (*out.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize] =
        (*in1.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize] *
            scale;
    // invert the translate
    (*out.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize] =
        -((*in1.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize]
              *
              (*out.offset(0 as libc::c_int as
                               isize))[0 as libc::c_int as usize] +
              (*in1.offset(1 as libc::c_int as
                               isize))[3 as libc::c_int as usize] *
                  (*out.offset(0 as libc::c_int as
                                   isize))[1 as libc::c_int as usize] +
              (*in1.offset(2 as libc::c_int as
                               isize))[3 as libc::c_int as usize] *
                  (*out.offset(0 as libc::c_int as
                                   isize))[2 as libc::c_int as usize]);
    (*out.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize] =
        -((*in1.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize]
              *
              (*out.offset(1 as libc::c_int as
                               isize))[0 as libc::c_int as usize] +
              (*in1.offset(1 as libc::c_int as
                               isize))[3 as libc::c_int as usize] *
                  (*out.offset(1 as libc::c_int as
                                   isize))[1 as libc::c_int as usize] +
              (*in1.offset(2 as libc::c_int as
                               isize))[3 as libc::c_int as usize] *
                  (*out.offset(1 as libc::c_int as
                                   isize))[2 as libc::c_int as usize]);
    (*out.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize] =
        -((*in1.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize]
              *
              (*out.offset(2 as libc::c_int as
                               isize))[0 as libc::c_int as usize] +
              (*in1.offset(1 as libc::c_int as
                               isize))[3 as libc::c_int as usize] *
                  (*out.offset(2 as libc::c_int as
                                   isize))[1 as libc::c_int as usize] +
              (*in1.offset(2 as libc::c_int as
                               isize))[3 as libc::c_int as usize] *
                  (*out.offset(2 as libc::c_int as
                                   isize))[2 as libc::c_int as usize]);
    // don't know if there's anything worth doing here
    (*out.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize] =
        0.0f32;
    (*out.offset(3 as libc::c_int as isize))[1 as libc::c_int as usize] =
        0.0f32;
    (*out.offset(3 as libc::c_int as isize))[2 as libc::c_int as usize] =
        0.0f32;
    (*out.offset(3 as libc::c_int as isize))[3 as libc::c_int as usize] =
        1.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn Matrix4x4_Transpose(mut out: *mut [vec_t; 4],
                                             mut in1: *const [vec_t; 4]) {
    (*out.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (*in1.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize];
    (*out.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] =
        (*in1.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize];
    (*out.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize] =
        (*in1.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize];
    (*out.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize] =
        (*in1.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize];
    (*out.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (*in1.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize];
    (*out.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] =
        (*in1.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize];
    (*out.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize] =
        (*in1.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize];
    (*out.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize] =
        (*in1.offset(3 as libc::c_int as isize))[1 as libc::c_int as usize];
    (*out.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (*in1.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize];
    (*out.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize] =
        (*in1.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize];
    (*out.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize] =
        (*in1.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize];
    (*out.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize] =
        (*in1.offset(3 as libc::c_int as isize))[2 as libc::c_int as usize];
    (*out.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (*in1.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize];
    (*out.offset(3 as libc::c_int as isize))[1 as libc::c_int as usize] =
        (*in1.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize];
    (*out.offset(3 as libc::c_int as isize))[2 as libc::c_int as usize] =
        (*in1.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize];
    (*out.offset(3 as libc::c_int as isize))[3 as libc::c_int as usize] =
        (*in1.offset(3 as libc::c_int as isize))[3 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn Matrix4x4_Invert_Full(mut out: *mut [vec_t; 4],
                                               mut in1: *const [vec_t; 4])
 -> qboolean {
    let mut temp: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut r: [*mut libc::c_float; 4] = [0 as *mut libc::c_float; 4];
    let mut rtemp: [[libc::c_float; 8]; 4] = [[0.; 8]; 4];
    let mut m: [libc::c_float; 4] = [0.; 4];
    let mut s: libc::c_float = 0.;
    r[0 as libc::c_int as usize] =
        rtemp[0 as libc::c_int as usize].as_mut_ptr();
    r[1 as libc::c_int as usize] =
        rtemp[1 as libc::c_int as usize].as_mut_ptr();
    r[2 as libc::c_int as usize] =
        rtemp[2 as libc::c_int as usize].as_mut_ptr();
    r[3 as libc::c_int as usize] =
        rtemp[3 as libc::c_int as usize].as_mut_ptr();
    *r[0 as libc::c_int as usize].offset(0 as libc::c_int as isize) =
        (*in1.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize];
    *r[0 as libc::c_int as usize].offset(1 as libc::c_int as isize) =
        (*in1.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize];
    *r[0 as libc::c_int as usize].offset(2 as libc::c_int as isize) =
        (*in1.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize];
    *r[0 as libc::c_int as usize].offset(3 as libc::c_int as isize) =
        (*in1.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize];
    *r[0 as libc::c_int as usize].offset(4 as libc::c_int as isize) = 1.0f32;
    *r[0 as libc::c_int as usize].offset(5 as libc::c_int as isize) = 0.0f32;
    *r[0 as libc::c_int as usize].offset(6 as libc::c_int as isize) = 0.0f32;
    *r[0 as libc::c_int as usize].offset(7 as libc::c_int as isize) = 0.0f32;
    *r[1 as libc::c_int as usize].offset(0 as libc::c_int as isize) =
        (*in1.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize];
    *r[1 as libc::c_int as usize].offset(1 as libc::c_int as isize) =
        (*in1.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize];
    *r[1 as libc::c_int as usize].offset(2 as libc::c_int as isize) =
        (*in1.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize];
    *r[1 as libc::c_int as usize].offset(3 as libc::c_int as isize) =
        (*in1.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize];
    *r[1 as libc::c_int as usize].offset(5 as libc::c_int as isize) = 1.0f32;
    *r[1 as libc::c_int as usize].offset(4 as libc::c_int as isize) = 0.0f32;
    *r[1 as libc::c_int as usize].offset(6 as libc::c_int as isize) = 0.0f32;
    *r[1 as libc::c_int as usize].offset(7 as libc::c_int as isize) = 0.0f32;
    *r[2 as libc::c_int as usize].offset(0 as libc::c_int as isize) =
        (*in1.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize];
    *r[2 as libc::c_int as usize].offset(1 as libc::c_int as isize) =
        (*in1.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize];
    *r[2 as libc::c_int as usize].offset(2 as libc::c_int as isize) =
        (*in1.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize];
    *r[2 as libc::c_int as usize].offset(3 as libc::c_int as isize) =
        (*in1.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize];
    *r[2 as libc::c_int as usize].offset(6 as libc::c_int as isize) = 1.0f32;
    *r[2 as libc::c_int as usize].offset(4 as libc::c_int as isize) = 0.0f32;
    *r[2 as libc::c_int as usize].offset(5 as libc::c_int as isize) = 0.0f32;
    *r[2 as libc::c_int as usize].offset(7 as libc::c_int as isize) = 0.0f32;
    *r[3 as libc::c_int as usize].offset(0 as libc::c_int as isize) =
        (*in1.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize];
    *r[3 as libc::c_int as usize].offset(1 as libc::c_int as isize) =
        (*in1.offset(3 as libc::c_int as isize))[1 as libc::c_int as usize];
    *r[3 as libc::c_int as usize].offset(2 as libc::c_int as isize) =
        (*in1.offset(3 as libc::c_int as isize))[2 as libc::c_int as usize];
    *r[3 as libc::c_int as usize].offset(3 as libc::c_int as isize) =
        (*in1.offset(3 as libc::c_int as isize))[3 as libc::c_int as usize];
    *r[3 as libc::c_int as usize].offset(4 as libc::c_int as isize) = 0.0f32;
    *r[3 as libc::c_int as usize].offset(5 as libc::c_int as isize) = 0.0f32;
    *r[3 as libc::c_int as usize].offset(6 as libc::c_int as isize) = 0.0f32;
    *r[3 as libc::c_int as usize].offset(7 as libc::c_int as isize) = 1.0f32;
    if __tg_fabs(*r[3 as libc::c_int as
                        usize].offset(0 as libc::c_int as isize)) >
           __tg_fabs(*r[2 as libc::c_int as
                            usize].offset(0 as libc::c_int as isize)) {
        temp = r[3 as libc::c_int as usize];
        r[3 as libc::c_int as usize] = r[2 as libc::c_int as usize];
        r[2 as libc::c_int as usize] = temp
    }
    if __tg_fabs(*r[2 as libc::c_int as
                        usize].offset(0 as libc::c_int as isize)) >
           __tg_fabs(*r[1 as libc::c_int as
                            usize].offset(0 as libc::c_int as isize)) {
        temp = r[2 as libc::c_int as usize];
        r[2 as libc::c_int as usize] = r[1 as libc::c_int as usize];
        r[1 as libc::c_int as usize] = temp
    }
    if __tg_fabs(*r[1 as libc::c_int as
                        usize].offset(0 as libc::c_int as isize)) >
           __tg_fabs(*r[0 as libc::c_int as
                            usize].offset(0 as libc::c_int as isize)) {
        temp = r[1 as libc::c_int as usize];
        r[1 as libc::c_int as usize] = r[0 as libc::c_int as usize];
        r[0 as libc::c_int as usize] = temp
    }
    if *r[0 as libc::c_int as usize].offset(0 as libc::c_int as isize) != 0. {
        m[1 as libc::c_int as usize] =
            *r[1 as libc::c_int as usize].offset(0 as libc::c_int as isize) /
                *r[0 as libc::c_int as
                       usize].offset(0 as libc::c_int as isize);
        m[2 as libc::c_int as usize] =
            *r[2 as libc::c_int as usize].offset(0 as libc::c_int as isize) /
                *r[0 as libc::c_int as
                       usize].offset(0 as libc::c_int as isize);
        m[3 as libc::c_int as usize] =
            *r[3 as libc::c_int as usize].offset(0 as libc::c_int as isize) /
                *r[0 as libc::c_int as
                       usize].offset(0 as libc::c_int as isize);
        s = *r[0 as libc::c_int as usize].offset(1 as libc::c_int as isize);
        *r[1 as libc::c_int as usize].offset(1 as libc::c_int as isize) -=
            m[1 as libc::c_int as usize] * s;
        *r[2 as libc::c_int as usize].offset(1 as libc::c_int as isize) -=
            m[2 as libc::c_int as usize] * s;
        *r[3 as libc::c_int as usize].offset(1 as libc::c_int as isize) -=
            m[3 as libc::c_int as usize] * s;
        s = *r[0 as libc::c_int as usize].offset(2 as libc::c_int as isize);
        *r[1 as libc::c_int as usize].offset(2 as libc::c_int as isize) -=
            m[1 as libc::c_int as usize] * s;
        *r[2 as libc::c_int as usize].offset(2 as libc::c_int as isize) -=
            m[2 as libc::c_int as usize] * s;
        *r[3 as libc::c_int as usize].offset(2 as libc::c_int as isize) -=
            m[3 as libc::c_int as usize] * s;
        s = *r[0 as libc::c_int as usize].offset(3 as libc::c_int as isize);
        *r[1 as libc::c_int as usize].offset(3 as libc::c_int as isize) -=
            m[1 as libc::c_int as usize] * s;
        *r[2 as libc::c_int as usize].offset(3 as libc::c_int as isize) -=
            m[2 as libc::c_int as usize] * s;
        *r[3 as libc::c_int as usize].offset(3 as libc::c_int as isize) -=
            m[3 as libc::c_int as usize] * s;
        s = *r[0 as libc::c_int as usize].offset(4 as libc::c_int as isize);
        if s != 0. {
            *r[1 as libc::c_int as usize].offset(4 as libc::c_int as isize) -=
                m[1 as libc::c_int as usize] * s;
            *r[2 as libc::c_int as usize].offset(4 as libc::c_int as isize) -=
                m[2 as libc::c_int as usize] * s;
            *r[3 as libc::c_int as usize].offset(4 as libc::c_int as isize) -=
                m[3 as libc::c_int as usize] * s
        }
        s = *r[0 as libc::c_int as usize].offset(5 as libc::c_int as isize);
        if s != 0. {
            *r[1 as libc::c_int as usize].offset(5 as libc::c_int as isize) -=
                m[1 as libc::c_int as usize] * s;
            *r[2 as libc::c_int as usize].offset(5 as libc::c_int as isize) -=
                m[2 as libc::c_int as usize] * s;
            *r[3 as libc::c_int as usize].offset(5 as libc::c_int as isize) -=
                m[3 as libc::c_int as usize] * s
        }
        s = *r[0 as libc::c_int as usize].offset(6 as libc::c_int as isize);
        if s != 0. {
            *r[1 as libc::c_int as usize].offset(6 as libc::c_int as isize) -=
                m[1 as libc::c_int as usize] * s;
            *r[2 as libc::c_int as usize].offset(6 as libc::c_int as isize) -=
                m[2 as libc::c_int as usize] * s;
            *r[3 as libc::c_int as usize].offset(6 as libc::c_int as isize) -=
                m[3 as libc::c_int as usize] * s
        }
        s = *r[0 as libc::c_int as usize].offset(7 as libc::c_int as isize);
        if s != 0. {
            *r[1 as libc::c_int as usize].offset(7 as libc::c_int as isize) -=
                m[1 as libc::c_int as usize] * s;
            *r[2 as libc::c_int as usize].offset(7 as libc::c_int as isize) -=
                m[2 as libc::c_int as usize] * s;
            *r[3 as libc::c_int as usize].offset(7 as libc::c_int as isize) -=
                m[3 as libc::c_int as usize] * s
        }
        if __tg_fabs(*r[3 as libc::c_int as
                            usize].offset(1 as libc::c_int as isize)) >
               __tg_fabs(*r[2 as libc::c_int as
                                usize].offset(1 as libc::c_int as isize)) {
            temp = r[3 as libc::c_int as usize];
            r[3 as libc::c_int as usize] = r[2 as libc::c_int as usize];
            r[2 as libc::c_int as usize] = temp
        }
        if __tg_fabs(*r[2 as libc::c_int as
                            usize].offset(1 as libc::c_int as isize)) >
               __tg_fabs(*r[1 as libc::c_int as
                                usize].offset(1 as libc::c_int as isize)) {
            temp = r[2 as libc::c_int as usize];
            r[2 as libc::c_int as usize] = r[1 as libc::c_int as usize];
            r[1 as libc::c_int as usize] = temp
        }
        if *r[1 as libc::c_int as usize].offset(1 as libc::c_int as isize) !=
               0. {
            m[2 as libc::c_int as usize] =
                *r[2 as libc::c_int as
                       usize].offset(1 as libc::c_int as isize) /
                    *r[1 as libc::c_int as
                           usize].offset(1 as libc::c_int as isize);
            m[3 as libc::c_int as usize] =
                *r[3 as libc::c_int as
                       usize].offset(1 as libc::c_int as isize) /
                    *r[1 as libc::c_int as
                           usize].offset(1 as libc::c_int as isize);
            *r[2 as libc::c_int as usize].offset(2 as libc::c_int as isize) -=
                m[2 as libc::c_int as usize] *
                    *r[1 as libc::c_int as
                           usize].offset(2 as libc::c_int as isize);
            *r[3 as libc::c_int as usize].offset(2 as libc::c_int as isize) -=
                m[3 as libc::c_int as usize] *
                    *r[1 as libc::c_int as
                           usize].offset(2 as libc::c_int as isize);
            *r[2 as libc::c_int as usize].offset(3 as libc::c_int as isize) -=
                m[2 as libc::c_int as usize] *
                    *r[1 as libc::c_int as
                           usize].offset(3 as libc::c_int as isize);
            *r[3 as libc::c_int as usize].offset(3 as libc::c_int as isize) -=
                m[3 as libc::c_int as usize] *
                    *r[1 as libc::c_int as
                           usize].offset(3 as libc::c_int as isize);
            s =
                *r[1 as libc::c_int as
                       usize].offset(4 as libc::c_int as isize);
            if s != 0. {
                *r[2 as libc::c_int as
                       usize].offset(4 as libc::c_int as isize) -=
                    m[2 as libc::c_int as usize] * s;
                *r[3 as libc::c_int as
                       usize].offset(4 as libc::c_int as isize) -=
                    m[3 as libc::c_int as usize] * s
            }
            s =
                *r[1 as libc::c_int as
                       usize].offset(5 as libc::c_int as isize);
            if s != 0. {
                *r[2 as libc::c_int as
                       usize].offset(5 as libc::c_int as isize) -=
                    m[2 as libc::c_int as usize] * s;
                *r[3 as libc::c_int as
                       usize].offset(5 as libc::c_int as isize) -=
                    m[3 as libc::c_int as usize] * s
            }
            s =
                *r[1 as libc::c_int as
                       usize].offset(6 as libc::c_int as isize);
            if s != 0. {
                *r[2 as libc::c_int as
                       usize].offset(6 as libc::c_int as isize) -=
                    m[2 as libc::c_int as usize] * s;
                *r[3 as libc::c_int as
                       usize].offset(6 as libc::c_int as isize) -=
                    m[3 as libc::c_int as usize] * s
            }
            s =
                *r[1 as libc::c_int as
                       usize].offset(7 as libc::c_int as isize);
            if s != 0. {
                *r[2 as libc::c_int as
                       usize].offset(7 as libc::c_int as isize) -=
                    m[2 as libc::c_int as usize] * s;
                *r[3 as libc::c_int as
                       usize].offset(7 as libc::c_int as isize) -=
                    m[3 as libc::c_int as usize] * s
            }
            if __tg_fabs(*r[3 as libc::c_int as
                                usize].offset(2 as libc::c_int as isize)) >
                   __tg_fabs(*r[2 as libc::c_int as
                                    usize].offset(2 as libc::c_int as isize))
               {
                temp = r[3 as libc::c_int as usize];
                r[3 as libc::c_int as usize] = r[2 as libc::c_int as usize];
                r[2 as libc::c_int as usize] = temp
            }
            if *r[2 as libc::c_int as usize].offset(2 as libc::c_int as isize)
                   != 0. {
                m[3 as libc::c_int as usize] =
                    *r[3 as libc::c_int as
                           usize].offset(2 as libc::c_int as isize) /
                        *r[2 as libc::c_int as
                               usize].offset(2 as libc::c_int as isize);
                *r[3 as libc::c_int as
                       usize].offset(3 as libc::c_int as isize) -=
                    m[3 as libc::c_int as usize] *
                        *r[2 as libc::c_int as
                               usize].offset(3 as libc::c_int as isize);
                *r[3 as libc::c_int as
                       usize].offset(4 as libc::c_int as isize) -=
                    m[3 as libc::c_int as usize] *
                        *r[2 as libc::c_int as
                               usize].offset(4 as libc::c_int as isize);
                *r[3 as libc::c_int as
                       usize].offset(5 as libc::c_int as isize) -=
                    m[3 as libc::c_int as usize] *
                        *r[2 as libc::c_int as
                               usize].offset(5 as libc::c_int as isize);
                *r[3 as libc::c_int as
                       usize].offset(6 as libc::c_int as isize) -=
                    m[3 as libc::c_int as usize] *
                        *r[2 as libc::c_int as
                               usize].offset(6 as libc::c_int as isize);
                *r[3 as libc::c_int as
                       usize].offset(7 as libc::c_int as isize) -=
                    m[3 as libc::c_int as usize] *
                        *r[2 as libc::c_int as
                               usize].offset(7 as libc::c_int as isize);
                if *r[3 as libc::c_int as
                          usize].offset(3 as libc::c_int as isize) != 0. {
                    s =
                        1.0f32 /
                            *r[3 as libc::c_int as
                                   usize].offset(3 as libc::c_int as isize);
                    *r[3 as libc::c_int as
                           usize].offset(4 as libc::c_int as isize) *= s;
                    *r[3 as libc::c_int as
                           usize].offset(5 as libc::c_int as isize) *= s;
                    *r[3 as libc::c_int as
                           usize].offset(6 as libc::c_int as isize) *= s;
                    *r[3 as libc::c_int as
                           usize].offset(7 as libc::c_int as isize) *= s;
                    m[2 as libc::c_int as usize] =
                        *r[2 as libc::c_int as
                               usize].offset(3 as libc::c_int as isize);
                    s =
                        1.0f32 /
                            *r[2 as libc::c_int as
                                   usize].offset(2 as libc::c_int as isize);
                    *r[2 as libc::c_int as
                           usize].offset(4 as libc::c_int as isize) =
                        s *
                            (*r[2 as libc::c_int as
                                    usize].offset(4 as libc::c_int as isize) -
                                 *r[3 as libc::c_int as
                                        usize].offset(4 as libc::c_int as
                                                          isize) *
                                     m[2 as libc::c_int as usize]);
                    *r[2 as libc::c_int as
                           usize].offset(5 as libc::c_int as isize) =
                        s *
                            (*r[2 as libc::c_int as
                                    usize].offset(5 as libc::c_int as isize) -
                                 *r[3 as libc::c_int as
                                        usize].offset(5 as libc::c_int as
                                                          isize) *
                                     m[2 as libc::c_int as usize]);
                    *r[2 as libc::c_int as
                           usize].offset(6 as libc::c_int as isize) =
                        s *
                            (*r[2 as libc::c_int as
                                    usize].offset(6 as libc::c_int as isize) -
                                 *r[3 as libc::c_int as
                                        usize].offset(6 as libc::c_int as
                                                          isize) *
                                     m[2 as libc::c_int as usize]);
                    *r[2 as libc::c_int as
                           usize].offset(7 as libc::c_int as isize) =
                        s *
                            (*r[2 as libc::c_int as
                                    usize].offset(7 as libc::c_int as isize) -
                                 *r[3 as libc::c_int as
                                        usize].offset(7 as libc::c_int as
                                                          isize) *
                                     m[2 as libc::c_int as usize]);
                    m[1 as libc::c_int as usize] =
                        *r[1 as libc::c_int as
                               usize].offset(3 as libc::c_int as isize);
                    *r[1 as libc::c_int as
                           usize].offset(4 as libc::c_int as isize) -=
                        *r[3 as libc::c_int as
                               usize].offset(4 as libc::c_int as isize) *
                            m[1 as libc::c_int as usize];
                    *r[1 as libc::c_int as
                           usize].offset(5 as libc::c_int as isize) -=
                        *r[3 as libc::c_int as
                               usize].offset(5 as libc::c_int as isize) *
                            m[1 as libc::c_int as usize];
                    *r[1 as libc::c_int as
                           usize].offset(6 as libc::c_int as isize) -=
                        *r[3 as libc::c_int as
                               usize].offset(6 as libc::c_int as isize) *
                            m[1 as libc::c_int as usize];
                    *r[1 as libc::c_int as
                           usize].offset(7 as libc::c_int as isize) -=
                        *r[3 as libc::c_int as
                               usize].offset(7 as libc::c_int as isize) *
                            m[1 as libc::c_int as usize];
                    m[0 as libc::c_int as usize] =
                        *r[0 as libc::c_int as
                               usize].offset(3 as libc::c_int as isize);
                    *r[0 as libc::c_int as
                           usize].offset(4 as libc::c_int as isize) -=
                        *r[3 as libc::c_int as
                               usize].offset(4 as libc::c_int as isize) *
                            m[0 as libc::c_int as usize];
                    *r[0 as libc::c_int as
                           usize].offset(5 as libc::c_int as isize) -=
                        *r[3 as libc::c_int as
                               usize].offset(5 as libc::c_int as isize) *
                            m[0 as libc::c_int as usize];
                    *r[0 as libc::c_int as
                           usize].offset(6 as libc::c_int as isize) -=
                        *r[3 as libc::c_int as
                               usize].offset(6 as libc::c_int as isize) *
                            m[0 as libc::c_int as usize];
                    *r[0 as libc::c_int as
                           usize].offset(7 as libc::c_int as isize) -=
                        *r[3 as libc::c_int as
                               usize].offset(7 as libc::c_int as isize) *
                            m[0 as libc::c_int as usize];
                    m[1 as libc::c_int as usize] =
                        *r[1 as libc::c_int as
                               usize].offset(2 as libc::c_int as isize);
                    s =
                        1.0f32 /
                            *r[1 as libc::c_int as
                                   usize].offset(1 as libc::c_int as isize);
                    *r[1 as libc::c_int as
                           usize].offset(4 as libc::c_int as isize) =
                        s *
                            (*r[1 as libc::c_int as
                                    usize].offset(4 as libc::c_int as isize) -
                                 *r[2 as libc::c_int as
                                        usize].offset(4 as libc::c_int as
                                                          isize) *
                                     m[1 as libc::c_int as usize]);
                    *r[1 as libc::c_int as
                           usize].offset(5 as libc::c_int as isize) =
                        s *
                            (*r[1 as libc::c_int as
                                    usize].offset(5 as libc::c_int as isize) -
                                 *r[2 as libc::c_int as
                                        usize].offset(5 as libc::c_int as
                                                          isize) *
                                     m[1 as libc::c_int as usize]);
                    *r[1 as libc::c_int as
                           usize].offset(6 as libc::c_int as isize) =
                        s *
                            (*r[1 as libc::c_int as
                                    usize].offset(6 as libc::c_int as isize) -
                                 *r[2 as libc::c_int as
                                        usize].offset(6 as libc::c_int as
                                                          isize) *
                                     m[1 as libc::c_int as usize]);
                    *r[1 as libc::c_int as
                           usize].offset(7 as libc::c_int as isize) =
                        s *
                            (*r[1 as libc::c_int as
                                    usize].offset(7 as libc::c_int as isize) -
                                 *r[2 as libc::c_int as
                                        usize].offset(7 as libc::c_int as
                                                          isize) *
                                     m[1 as libc::c_int as usize]);
                    m[0 as libc::c_int as usize] =
                        *r[0 as libc::c_int as
                               usize].offset(2 as libc::c_int as isize);
                    *r[0 as libc::c_int as
                           usize].offset(4 as libc::c_int as isize) -=
                        *r[2 as libc::c_int as
                               usize].offset(4 as libc::c_int as isize) *
                            m[0 as libc::c_int as usize];
                    *r[0 as libc::c_int as
                           usize].offset(5 as libc::c_int as isize) -=
                        *r[2 as libc::c_int as
                               usize].offset(5 as libc::c_int as isize) *
                            m[0 as libc::c_int as usize];
                    *r[0 as libc::c_int as
                           usize].offset(6 as libc::c_int as isize) -=
                        *r[2 as libc::c_int as
                               usize].offset(6 as libc::c_int as isize) *
                            m[0 as libc::c_int as usize];
                    *r[0 as libc::c_int as
                           usize].offset(7 as libc::c_int as isize) -=
                        *r[2 as libc::c_int as
                               usize].offset(7 as libc::c_int as isize) *
                            m[0 as libc::c_int as usize];
                    m[0 as libc::c_int as usize] =
                        *r[0 as libc::c_int as
                               usize].offset(1 as libc::c_int as isize);
                    s =
                        1.0f32 /
                            *r[0 as libc::c_int as
                                   usize].offset(0 as libc::c_int as isize);
                    *r[0 as libc::c_int as
                           usize].offset(4 as libc::c_int as isize) =
                        s *
                            (*r[0 as libc::c_int as
                                    usize].offset(4 as libc::c_int as isize) -
                                 *r[1 as libc::c_int as
                                        usize].offset(4 as libc::c_int as
                                                          isize) *
                                     m[0 as libc::c_int as usize]);
                    *r[0 as libc::c_int as
                           usize].offset(5 as libc::c_int as isize) =
                        s *
                            (*r[0 as libc::c_int as
                                    usize].offset(5 as libc::c_int as isize) -
                                 *r[1 as libc::c_int as
                                        usize].offset(5 as libc::c_int as
                                                          isize) *
                                     m[0 as libc::c_int as usize]);
                    *r[0 as libc::c_int as
                           usize].offset(6 as libc::c_int as isize) =
                        s *
                            (*r[0 as libc::c_int as
                                    usize].offset(6 as libc::c_int as isize) -
                                 *r[1 as libc::c_int as
                                        usize].offset(6 as libc::c_int as
                                                          isize) *
                                     m[0 as libc::c_int as usize]);
                    *r[0 as libc::c_int as
                           usize].offset(7 as libc::c_int as isize) =
                        s *
                            (*r[0 as libc::c_int as
                                    usize].offset(7 as libc::c_int as isize) -
                                 *r[1 as libc::c_int as
                                        usize].offset(7 as libc::c_int as
                                                          isize) *
                                     m[0 as libc::c_int as usize]);
                    (*out.offset(0 as libc::c_int as
                                     isize))[0 as libc::c_int as usize] =
                        *r[0 as libc::c_int as
                               usize].offset(4 as libc::c_int as isize);
                    (*out.offset(0 as libc::c_int as
                                     isize))[1 as libc::c_int as usize] =
                        *r[0 as libc::c_int as
                               usize].offset(5 as libc::c_int as isize);
                    (*out.offset(0 as libc::c_int as
                                     isize))[2 as libc::c_int as usize] =
                        *r[0 as libc::c_int as
                               usize].offset(6 as libc::c_int as isize);
                    (*out.offset(0 as libc::c_int as
                                     isize))[3 as libc::c_int as usize] =
                        *r[0 as libc::c_int as
                               usize].offset(7 as libc::c_int as isize);
                    (*out.offset(1 as libc::c_int as
                                     isize))[0 as libc::c_int as usize] =
                        *r[1 as libc::c_int as
                               usize].offset(4 as libc::c_int as isize);
                    (*out.offset(1 as libc::c_int as
                                     isize))[1 as libc::c_int as usize] =
                        *r[1 as libc::c_int as
                               usize].offset(5 as libc::c_int as isize);
                    (*out.offset(1 as libc::c_int as
                                     isize))[2 as libc::c_int as usize] =
                        *r[1 as libc::c_int as
                               usize].offset(6 as libc::c_int as isize);
                    (*out.offset(1 as libc::c_int as
                                     isize))[3 as libc::c_int as usize] =
                        *r[1 as libc::c_int as
                               usize].offset(7 as libc::c_int as isize);
                    (*out.offset(2 as libc::c_int as
                                     isize))[0 as libc::c_int as usize] =
                        *r[2 as libc::c_int as
                               usize].offset(4 as libc::c_int as isize);
                    (*out.offset(2 as libc::c_int as
                                     isize))[1 as libc::c_int as usize] =
                        *r[2 as libc::c_int as
                               usize].offset(5 as libc::c_int as isize);
                    (*out.offset(2 as libc::c_int as
                                     isize))[2 as libc::c_int as usize] =
                        *r[2 as libc::c_int as
                               usize].offset(6 as libc::c_int as isize);
                    (*out.offset(2 as libc::c_int as
                                     isize))[3 as libc::c_int as usize] =
                        *r[2 as libc::c_int as
                               usize].offset(7 as libc::c_int as isize);
                    (*out.offset(3 as libc::c_int as
                                     isize))[0 as libc::c_int as usize] =
                        *r[3 as libc::c_int as
                               usize].offset(4 as libc::c_int as isize);
                    (*out.offset(3 as libc::c_int as
                                     isize))[1 as libc::c_int as usize] =
                        *r[3 as libc::c_int as
                               usize].offset(5 as libc::c_int as isize);
                    (*out.offset(3 as libc::c_int as
                                     isize))[2 as libc::c_int as usize] =
                        *r[3 as libc::c_int as
                               usize].offset(6 as libc::c_int as isize);
                    (*out.offset(3 as libc::c_int as
                                     isize))[3 as libc::c_int as usize] =
                        *r[3 as libc::c_int as
                               usize].offset(7 as libc::c_int as isize);
                    return true_0
                }
            }
        }
    }
    return false_0;
}
