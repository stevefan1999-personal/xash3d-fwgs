#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type movie_state_s;
}
pub type size_t = libc::c_ulong;
pub type uint = libc::c_uint;
pub type byte = libc::c_uchar;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type word = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wavdata_t {
    pub rate: word,
    pub width: byte,
    pub channels: byte,
    pub loopStart: libc::c_int,
    pub samples: libc::c_int,
    pub type_0: uint,
    pub flags: uint,
    pub buffer: *mut byte,
    pub size: size_t,
}
pub type movie_state_t = movie_state_s;
/*
avi_stub.c - playing AVI files (stub)
Copyright (C) 2018 a1batross

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
pub unsafe extern "C" fn AVI_GetVideoFrameNumber(mut Avi: *mut movie_state_t,
                                                 mut time: libc::c_float)
 -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn AVI_GetVideoFrame(mut Avi: *mut movie_state_t,
                                           mut frame: libc::c_int)
 -> *mut byte {
    return 0 as *mut byte;
}
#[no_mangle]
pub unsafe extern "C" fn AVI_GetVideoInfo(mut Avi: *mut movie_state_t,
                                          mut xres: *mut libc::c_int,
                                          mut yres: *mut libc::c_int,
                                          mut duration: *mut libc::c_float)
 -> qboolean {
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn AVI_GetAudioInfo(mut Avi: *mut movie_state_t,
                                          mut snd_info: *mut wavdata_t)
 -> qboolean {
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn AVI_GetAudioChunk(mut Avi: *mut movie_state_t,
                                           mut audiodata: *mut libc::c_char,
                                           mut offset: libc::c_int,
                                           mut length: libc::c_int)
 -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn AVI_OpenVideo(mut Avi: *mut movie_state_t,
                                       mut filename: *const libc::c_char,
                                       mut load_audio: qboolean,
                                       mut quiet: libc::c_int) {
}
#[no_mangle]
pub unsafe extern "C" fn AVI_LoadVideo(mut filename: *const libc::c_char,
                                       mut load_audio: qboolean)
 -> *mut movie_state_t {
    return 0 as *mut movie_state_t;
}
#[no_mangle]
pub unsafe extern "C" fn AVI_TimeToSoundPosition(mut Avi: *mut movie_state_t,
                                                 mut time: libc::c_int)
 -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn AVI_GetVideoFrameCount(mut Avi: *mut movie_state_t)
 -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn AVI_CloseVideo(mut Avi: *mut movie_state_t) { }
#[no_mangle]
pub unsafe extern "C" fn AVI_IsActive(mut Avi: *mut movie_state_t)
 -> qboolean {
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn AVI_FreeVideo(mut Avi: *mut movie_state_t) { }
#[no_mangle]
pub unsafe extern "C" fn AVI_GetState(mut num: libc::c_int)
 -> *mut movie_state_t {
    return 0 as *mut movie_state_t;
}
#[no_mangle]
pub unsafe extern "C" fn AVI_Initailize() -> qboolean { return false_0; }
#[no_mangle]
pub unsafe extern "C" fn AVI_Shutdown() { }
// WIN32
