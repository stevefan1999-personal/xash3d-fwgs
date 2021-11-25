#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn sqrtf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn SinCos(radians: libc::c_float, sine: *mut libc::c_float,
              cosine: *mut libc::c_float);
}
pub type vec_t = libc::c_float;
pub type matrix4x4 = [[vec_t; 4]; 4];
#[inline(always)]
unsafe extern "C" fn __tg_sqrt(mut __x: libc::c_float) -> libc::c_float {
    return sqrtf(__x);
}
/*
gl_rmath.c - renderer mathlib
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
/*
========================================================================

	       Matrix4x4 operations (private to renderer)

========================================================================
*/
#[no_mangle]
pub unsafe extern "C" fn Matrix4x4_Concat(mut out: *mut [vec_t; 4],
                                          mut in1: *const [vec_t; 4],
                                          mut in2: *const [vec_t; 4]) {
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
                                 isize))[0 as libc::c_int as usize] +
            (*in1.offset(0 as libc::c_int as
                             isize))[3 as libc::c_int as usize] *
                (*in2.offset(3 as libc::c_int as
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
                                 isize))[1 as libc::c_int as usize] +
            (*in1.offset(0 as libc::c_int as
                             isize))[3 as libc::c_int as usize] *
                (*in2.offset(3 as libc::c_int as
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
                                 isize))[2 as libc::c_int as usize] +
            (*in1.offset(0 as libc::c_int as
                             isize))[3 as libc::c_int as usize] *
                (*in2.offset(3 as libc::c_int as
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
                             isize))[3 as libc::c_int as usize] *
                (*in2.offset(3 as libc::c_int as
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
                                 isize))[0 as libc::c_int as usize] +
            (*in1.offset(1 as libc::c_int as
                             isize))[3 as libc::c_int as usize] *
                (*in2.offset(3 as libc::c_int as
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
                                 isize))[1 as libc::c_int as usize] +
            (*in1.offset(1 as libc::c_int as
                             isize))[3 as libc::c_int as usize] *
                (*in2.offset(3 as libc::c_int as
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
                                 isize))[2 as libc::c_int as usize] +
            (*in1.offset(1 as libc::c_int as
                             isize))[3 as libc::c_int as usize] *
                (*in2.offset(3 as libc::c_int as
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
                             isize))[3 as libc::c_int as usize] *
                (*in2.offset(3 as libc::c_int as
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
                                 isize))[0 as libc::c_int as usize] +
            (*in1.offset(2 as libc::c_int as
                             isize))[3 as libc::c_int as usize] *
                (*in2.offset(3 as libc::c_int as
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
                                 isize))[1 as libc::c_int as usize] +
            (*in1.offset(2 as libc::c_int as
                             isize))[3 as libc::c_int as usize] *
                (*in2.offset(3 as libc::c_int as
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
                                 isize))[2 as libc::c_int as usize] +
            (*in1.offset(2 as libc::c_int as
                             isize))[3 as libc::c_int as usize] *
                (*in2.offset(3 as libc::c_int as
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
                             isize))[3 as libc::c_int as usize] *
                (*in2.offset(3 as libc::c_int as
                                 isize))[3 as libc::c_int as usize];
    (*out.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (*in1.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize] *
            (*in2.offset(0 as libc::c_int as
                             isize))[0 as libc::c_int as usize] +
            (*in1.offset(3 as libc::c_int as
                             isize))[1 as libc::c_int as usize] *
                (*in2.offset(1 as libc::c_int as
                                 isize))[0 as libc::c_int as usize] +
            (*in1.offset(3 as libc::c_int as
                             isize))[2 as libc::c_int as usize] *
                (*in2.offset(2 as libc::c_int as
                                 isize))[0 as libc::c_int as usize] +
            (*in1.offset(3 as libc::c_int as
                             isize))[3 as libc::c_int as usize] *
                (*in2.offset(3 as libc::c_int as
                                 isize))[0 as libc::c_int as usize];
    (*out.offset(3 as libc::c_int as isize))[1 as libc::c_int as usize] =
        (*in1.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize] *
            (*in2.offset(0 as libc::c_int as
                             isize))[1 as libc::c_int as usize] +
            (*in1.offset(3 as libc::c_int as
                             isize))[1 as libc::c_int as usize] *
                (*in2.offset(1 as libc::c_int as
                                 isize))[1 as libc::c_int as usize] +
            (*in1.offset(3 as libc::c_int as
                             isize))[2 as libc::c_int as usize] *
                (*in2.offset(2 as libc::c_int as
                                 isize))[1 as libc::c_int as usize] +
            (*in1.offset(3 as libc::c_int as
                             isize))[3 as libc::c_int as usize] *
                (*in2.offset(3 as libc::c_int as
                                 isize))[1 as libc::c_int as usize];
    (*out.offset(3 as libc::c_int as isize))[2 as libc::c_int as usize] =
        (*in1.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize] *
            (*in2.offset(0 as libc::c_int as
                             isize))[2 as libc::c_int as usize] +
            (*in1.offset(3 as libc::c_int as
                             isize))[1 as libc::c_int as usize] *
                (*in2.offset(1 as libc::c_int as
                                 isize))[2 as libc::c_int as usize] +
            (*in1.offset(3 as libc::c_int as
                             isize))[2 as libc::c_int as usize] *
                (*in2.offset(2 as libc::c_int as
                                 isize))[2 as libc::c_int as usize] +
            (*in1.offset(3 as libc::c_int as
                             isize))[3 as libc::c_int as usize] *
                (*in2.offset(3 as libc::c_int as
                                 isize))[2 as libc::c_int as usize];
    (*out.offset(3 as libc::c_int as isize))[3 as libc::c_int as usize] =
        (*in1.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize] *
            (*in2.offset(0 as libc::c_int as
                             isize))[3 as libc::c_int as usize] +
            (*in1.offset(3 as libc::c_int as
                             isize))[1 as libc::c_int as usize] *
                (*in2.offset(1 as libc::c_int as
                                 isize))[3 as libc::c_int as usize] +
            (*in1.offset(3 as libc::c_int as
                             isize))[2 as libc::c_int as usize] *
                (*in2.offset(2 as libc::c_int as
                                 isize))[3 as libc::c_int as usize] +
            (*in1.offset(3 as libc::c_int as
                             isize))[3 as libc::c_int as usize] *
                (*in2.offset(3 as libc::c_int as
                                 isize))[3 as libc::c_int as usize];
}
/*
================
Matrix4x4_CreateProjection

NOTE: produce quake style world orientation
================
*/
#[no_mangle]
pub unsafe extern "C" fn Matrix4x4_CreateProjection(mut out: *mut [vec_t; 4],
                                                    mut xMax: libc::c_float,
                                                    mut xMin: libc::c_float,
                                                    mut yMax: libc::c_float,
                                                    mut yMin: libc::c_float,
                                                    mut zNear: libc::c_float,
                                                    mut zFar: libc::c_float) {
    (*out.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] =
        2.0f32 * zNear / (xMax - xMin);
    (*out.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] =
        2.0f32 * zNear / (yMax - yMin);
    (*out.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize] =
        -(zFar + zNear) / (zFar - zNear);
    let ref mut fresh0 =
        (*out.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize];
    *fresh0 = 0.0f32;
    let ref mut fresh1 =
        (*out.offset(3 as libc::c_int as isize))[1 as libc::c_int as usize];
    *fresh1 = *fresh0;
    let ref mut fresh2 =
        (*out.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize];
    *fresh2 = *fresh1;
    let ref mut fresh3 =
        (*out.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize];
    *fresh3 = *fresh2;
    let ref mut fresh4 =
        (*out.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize];
    *fresh4 = *fresh3;
    let ref mut fresh5 =
        (*out.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize];
    *fresh5 = *fresh4;
    (*out.offset(3 as libc::c_int as isize))[3 as libc::c_int as usize] =
        *fresh5;
    (*out.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] =
        0.0f32;
    (*out.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize] =
        0.0f32;
    (*out.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize] =
        (xMax + xMin) / (xMax - xMin);
    (*out.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize] =
        (yMax + yMin) / (yMax - yMin);
    (*out.offset(3 as libc::c_int as isize))[2 as libc::c_int as usize] =
        -1.0f32;
    (*out.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize] =
        -(2.0f32 * zFar * zNear) / (zFar - zNear);
}
#[no_mangle]
pub unsafe extern "C" fn Matrix4x4_CreateOrtho(mut out: *mut [vec_t; 4],
                                               mut xLeft: libc::c_float,
                                               mut xRight: libc::c_float,
                                               mut yBottom: libc::c_float,
                                               mut yTop: libc::c_float,
                                               mut zNear: libc::c_float,
                                               mut zFar: libc::c_float) {
    (*out.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] =
        2.0f32 / (xRight - xLeft);
    (*out.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] =
        2.0f32 / (yTop - yBottom);
    (*out.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize] =
        -2.0f32 / (zFar - zNear);
    (*out.offset(3 as libc::c_int as isize))[3 as libc::c_int as usize] =
        1.0f32;
    let ref mut fresh6 =
        (*out.offset(3 as libc::c_int as isize))[2 as libc::c_int as usize];
    *fresh6 = 0.0f32;
    let ref mut fresh7 =
        (*out.offset(3 as libc::c_int as isize))[1 as libc::c_int as usize];
    *fresh7 = *fresh6;
    let ref mut fresh8 =
        (*out.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize];
    *fresh8 = *fresh7;
    let ref mut fresh9 =
        (*out.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize];
    *fresh9 = *fresh8;
    let ref mut fresh10 =
        (*out.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize];
    *fresh10 = *fresh9;
    let ref mut fresh11 =
        (*out.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize];
    *fresh11 = *fresh10;
    (*out.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] =
        *fresh11;
    (*out.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] =
        0.0f32;
    (*out.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize] =
        0.0f32;
    (*out.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize] =
        -(xRight + xLeft) / (xRight - xLeft);
    (*out.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize] =
        -(yTop + yBottom) / (yTop - yBottom);
    (*out.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize] =
        -(zFar + zNear) / (zFar - zNear);
}
/*
================
Matrix4x4_CreateModelview

NOTE: produce quake style world orientation
================
*/
#[no_mangle]
pub unsafe extern "C" fn Matrix4x4_CreateModelview(mut out: *mut [vec_t; 4]) {
    let ref mut fresh12 =
        (*out.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize];
    *fresh12 = 0.0f32;
    let ref mut fresh13 =
        (*out.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize];
    *fresh13 = *fresh12;
    (*out.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] =
        *fresh13;
    let ref mut fresh14 =
        (*out.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize];
    *fresh14 = 0.0f32;
    (*out.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize] =
        *fresh14;
    let ref mut fresh15 =
        (*out.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize];
    *fresh15 = 0.0f32;
    (*out.offset(3 as libc::c_int as isize))[1 as libc::c_int as usize] =
        *fresh15;
    let ref mut fresh16 =
        (*out.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize];
    *fresh16 = 0.0f32;
    (*out.offset(3 as libc::c_int as isize))[2 as libc::c_int as usize] =
        *fresh16;
    (*out.offset(3 as libc::c_int as isize))[3 as libc::c_int as usize] =
        1.0f32;
    let ref mut fresh17 =
        (*out.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize];
    *fresh17 = 0.0f32;
    let ref mut fresh18 =
        (*out.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize];
    *fresh18 = *fresh17;
    (*out.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] =
        *fresh18;
    let ref mut fresh19 =
        (*out.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize];
    *fresh19 = -1.0f32;
    (*out.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] =
        *fresh19;
    (*out.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize] =
        1.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn Matrix4x4_ToArrayFloatGL(mut in_0: *const [vec_t; 4],
                                                  mut out:
                                                      *mut libc::c_float) {
    *out.offset(0 as libc::c_int as isize) =
        (*in_0.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize];
    *out.offset(1 as libc::c_int as isize) =
        (*in_0.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize];
    *out.offset(2 as libc::c_int as isize) =
        (*in_0.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize];
    *out.offset(3 as libc::c_int as isize) =
        (*in_0.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize];
    *out.offset(4 as libc::c_int as isize) =
        (*in_0.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize];
    *out.offset(5 as libc::c_int as isize) =
        (*in_0.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize];
    *out.offset(6 as libc::c_int as isize) =
        (*in_0.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize];
    *out.offset(7 as libc::c_int as isize) =
        (*in_0.offset(3 as libc::c_int as isize))[1 as libc::c_int as usize];
    *out.offset(8 as libc::c_int as isize) =
        (*in_0.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize];
    *out.offset(9 as libc::c_int as isize) =
        (*in_0.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize];
    *out.offset(10 as libc::c_int as isize) =
        (*in_0.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize];
    *out.offset(11 as libc::c_int as isize) =
        (*in_0.offset(3 as libc::c_int as isize))[2 as libc::c_int as usize];
    *out.offset(12 as libc::c_int as isize) =
        (*in_0.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize];
    *out.offset(13 as libc::c_int as isize) =
        (*in_0.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize];
    *out.offset(14 as libc::c_int as isize) =
        (*in_0.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize];
    *out.offset(15 as libc::c_int as isize) =
        (*in_0.offset(3 as libc::c_int as isize))[3 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn Matrix4x4_FromArrayFloatGL(mut out: *mut [vec_t; 4],
                                                    mut in_0:
                                                        *const libc::c_float) {
    (*out.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] =
        *in_0.offset(0 as libc::c_int as isize);
    (*out.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] =
        *in_0.offset(1 as libc::c_int as isize);
    (*out.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] =
        *in_0.offset(2 as libc::c_int as isize);
    (*out.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize] =
        *in_0.offset(3 as libc::c_int as isize);
    (*out.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] =
        *in_0.offset(4 as libc::c_int as isize);
    (*out.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] =
        *in_0.offset(5 as libc::c_int as isize);
    (*out.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize] =
        *in_0.offset(6 as libc::c_int as isize);
    (*out.offset(3 as libc::c_int as isize))[1 as libc::c_int as usize] =
        *in_0.offset(7 as libc::c_int as isize);
    (*out.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize] =
        *in_0.offset(8 as libc::c_int as isize);
    (*out.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize] =
        *in_0.offset(9 as libc::c_int as isize);
    (*out.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize] =
        *in_0.offset(10 as libc::c_int as isize);
    (*out.offset(3 as libc::c_int as isize))[2 as libc::c_int as usize] =
        *in_0.offset(11 as libc::c_int as isize);
    (*out.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize] =
        *in_0.offset(12 as libc::c_int as isize);
    (*out.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize] =
        *in_0.offset(13 as libc::c_int as isize);
    (*out.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize] =
        *in_0.offset(14 as libc::c_int as isize);
    (*out.offset(3 as libc::c_int as isize))[3 as libc::c_int as usize] =
        *in_0.offset(15 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn Matrix4x4_CreateTranslate(mut out: *mut [vec_t; 4],
                                                   mut x: libc::c_float,
                                                   mut y: libc::c_float,
                                                   mut z: libc::c_float) {
    (*out.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] =
        1.0f32;
    (*out.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] =
        0.0f32;
    (*out.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize] =
        0.0f32;
    (*out.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize] = x;
    (*out.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] =
        0.0f32;
    (*out.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] =
        1.0f32;
    (*out.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize] =
        0.0f32;
    (*out.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize] = y;
    (*out.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] =
        0.0f32;
    (*out.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize] =
        0.0f32;
    (*out.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize] =
        1.0f32;
    (*out.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize] = z;
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
pub unsafe extern "C" fn Matrix4x4_CreateRotate(mut out: *mut [vec_t; 4],
                                                mut angle: libc::c_float,
                                                mut x: libc::c_float,
                                                mut y: libc::c_float,
                                                mut z: libc::c_float) {
    let mut len: libc::c_float = 0.;
    let mut c: libc::c_float = 0.;
    let mut s: libc::c_float = 0.;
    len = x * x + y * y + z * z;
    if len != 0.0f32 { len = 1.0f32 / __tg_sqrt(len) }
    x *= len;
    y *= len;
    z *= len;
    angle *= -(3.14159265358979323846f64 as libc::c_float) / 180.0f32;
    SinCos(angle, &mut s, &mut c);
    (*out.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] =
        x * x + c * (1 as libc::c_int as libc::c_float - x * x);
    (*out.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] =
        x * y * (1 as libc::c_int as libc::c_float - c) + z * s;
    (*out.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize] =
        z * x * (1 as libc::c_int as libc::c_float - c) - y * s;
    (*out.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize] =
        0.0f32;
    (*out.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] =
        x * y * (1 as libc::c_int as libc::c_float - c) - z * s;
    (*out.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] =
        y * y + c * (1 as libc::c_int as libc::c_float - y * y);
    (*out.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize] =
        y * z * (1 as libc::c_int as libc::c_float - c) + x * s;
    (*out.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize] =
        0.0f32;
    (*out.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] =
        z * x * (1 as libc::c_int as libc::c_float - c) + y * s;
    (*out.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize] =
        y * z * (1 as libc::c_int as libc::c_float - c) - x * s;
    (*out.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize] =
        z * z + c * (1 as libc::c_int as libc::c_float - z * z);
    (*out.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize] =
        0.0f32;
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
pub unsafe extern "C" fn Matrix4x4_CreateScale(mut out: *mut [vec_t; 4],
                                               mut x: libc::c_float) {
    (*out.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] = x;
    (*out.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] =
        0.0f32;
    (*out.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize] =
        0.0f32;
    (*out.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize] =
        0.0f32;
    (*out.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] =
        0.0f32;
    (*out.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] = x;
    (*out.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize] =
        0.0f32;
    (*out.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize] =
        0.0f32;
    (*out.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] =
        0.0f32;
    (*out.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize] =
        0.0f32;
    (*out.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize] = x;
    (*out.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize] =
        0.0f32;
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
pub unsafe extern "C" fn Matrix4x4_CreateScale3(mut out: *mut [vec_t; 4],
                                                mut x: libc::c_float,
                                                mut y: libc::c_float,
                                                mut z: libc::c_float) {
    (*out.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] = x;
    (*out.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] =
        0.0f32;
    (*out.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize] =
        0.0f32;
    (*out.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize] =
        0.0f32;
    (*out.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] =
        0.0f32;
    (*out.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] = y;
    (*out.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize] =
        0.0f32;
    (*out.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize] =
        0.0f32;
    (*out.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] =
        0.0f32;
    (*out.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize] =
        0.0f32;
    (*out.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize] = z;
    (*out.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize] =
        0.0f32;
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
pub unsafe extern "C" fn Matrix4x4_ConcatTranslate(mut out: *mut [vec_t; 4],
                                                   mut x: libc::c_float,
                                                   mut y: libc::c_float,
                                                   mut z: libc::c_float) {
    let mut base: matrix4x4 = [[0.; 4]; 4];
    let mut temp: matrix4x4 = [[0.; 4]; 4];
    memcpy(base.as_mut_ptr() as *mut libc::c_void, out as *const libc::c_void,
           ::std::mem::size_of::<matrix4x4>() as libc::c_ulong);
    Matrix4x4_CreateTranslate(temp.as_mut_ptr(), x, y, z);
    Matrix4x4_Concat(out, base.as_mut_ptr() as *const [vec_t; 4],
                     temp.as_mut_ptr() as *const [vec_t; 4]);
}
#[no_mangle]
pub unsafe extern "C" fn Matrix4x4_ConcatRotate(mut out: *mut [vec_t; 4],
                                                mut angle: libc::c_float,
                                                mut x: libc::c_float,
                                                mut y: libc::c_float,
                                                mut z: libc::c_float) {
    let mut base: matrix4x4 = [[0.; 4]; 4];
    let mut temp: matrix4x4 = [[0.; 4]; 4];
    memcpy(base.as_mut_ptr() as *mut libc::c_void, out as *const libc::c_void,
           ::std::mem::size_of::<matrix4x4>() as libc::c_ulong);
    Matrix4x4_CreateRotate(temp.as_mut_ptr(), angle, x, y, z);
    Matrix4x4_Concat(out, base.as_mut_ptr() as *const [vec_t; 4],
                     temp.as_mut_ptr() as *const [vec_t; 4]);
}
#[no_mangle]
pub unsafe extern "C" fn Matrix4x4_ConcatScale(mut out: *mut [vec_t; 4],
                                               mut x: libc::c_float) {
    let mut base: matrix4x4 = [[0.; 4]; 4];
    let mut temp: matrix4x4 = [[0.; 4]; 4];
    memcpy(base.as_mut_ptr() as *mut libc::c_void, out as *const libc::c_void,
           ::std::mem::size_of::<matrix4x4>() as libc::c_ulong);
    Matrix4x4_CreateScale(temp.as_mut_ptr(), x);
    Matrix4x4_Concat(out, base.as_mut_ptr() as *const [vec_t; 4],
                     temp.as_mut_ptr() as *const [vec_t; 4]);
}
#[no_mangle]
pub unsafe extern "C" fn Matrix4x4_ConcatScale3(mut out: *mut [vec_t; 4],
                                                mut x: libc::c_float,
                                                mut y: libc::c_float,
                                                mut z: libc::c_float) {
    let mut base: matrix4x4 = [[0.; 4]; 4];
    let mut temp: matrix4x4 = [[0.; 4]; 4];
    memcpy(base.as_mut_ptr() as *mut libc::c_void, out as *const libc::c_void,
           ::std::mem::size_of::<matrix4x4>() as libc::c_ulong);
    Matrix4x4_CreateScale3(temp.as_mut_ptr(), x, y, z);
    Matrix4x4_Concat(out, base.as_mut_ptr() as *const [vec_t; 4],
                     temp.as_mut_ptr() as *const [vec_t; 4]);
}
