#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_transmute, register_tool)]
extern "C" {
    #[no_mangle]
    fn Q_atoi(str: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn Q_strnicmp(s1: *const libc::c_char, s2: *const libc::c_char,
                  n: libc::c_int) -> libc::c_int;
}
/*
build.c - returns a engine build number
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
static mut date: *const libc::c_char =
    b"Nov 26 2021\x00" as *const u8 as *const libc::c_char;
static mut mon: [*const libc::c_char; 12] =
    [b"Jan\x00" as *const u8 as *const libc::c_char,
     b"Feb\x00" as *const u8 as *const libc::c_char,
     b"Mar\x00" as *const u8 as *const libc::c_char,
     b"Apr\x00" as *const u8 as *const libc::c_char,
     b"May\x00" as *const u8 as *const libc::c_char,
     b"Jun\x00" as *const u8 as *const libc::c_char,
     b"Jul\x00" as *const u8 as *const libc::c_char,
     b"Aug\x00" as *const u8 as *const libc::c_char,
     b"Sep\x00" as *const u8 as *const libc::c_char,
     b"Oct\x00" as *const u8 as *const libc::c_char,
     b"Nov\x00" as *const u8 as *const libc::c_char,
     b"Dec\x00" as *const u8 as *const libc::c_char];
static mut mond: [libc::c_char; 12] =
    [31 as libc::c_int as libc::c_char, 28 as libc::c_int as libc::c_char,
     31 as libc::c_int as libc::c_char, 30 as libc::c_int as libc::c_char,
     31 as libc::c_int as libc::c_char, 30 as libc::c_int as libc::c_char,
     31 as libc::c_int as libc::c_char, 31 as libc::c_int as libc::c_char,
     30 as libc::c_int as libc::c_char, 31 as libc::c_int as libc::c_char,
     30 as libc::c_int as libc::c_char, 31 as libc::c_int as libc::c_char];
// returns days since Apr 1 2015
#[no_mangle]
pub unsafe extern "C" fn Q_buildnum() -> libc::c_int {
    let mut m: libc::c_int = 0 as libc::c_int; // Apr 1 2015
    let mut d: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    static mut b: libc::c_int = 0 as libc::c_int;
    if b != 0 as libc::c_int { return b }
    m = 0 as libc::c_int;
    while m < 11 as libc::c_int {
        if Q_strnicmp(&*date.offset(0 as libc::c_int as isize),
                      mon[m as usize], 3 as libc::c_int) == 0 {
            break ;
        }
        d += mond[m as usize] as libc::c_int;
        m += 1
    }
    d += Q_atoi(&*date.offset(4 as libc::c_int as isize)) - 1 as libc::c_int;
    y =
        Q_atoi(&*date.offset(7 as libc::c_int as isize)) -
            1900 as libc::c_int;
    b =
        d +
            ((y - 1 as libc::c_int) as libc::c_float * 365.25f32) as
                libc::c_int;
    if y % 4 as libc::c_int == 0 as libc::c_int && m > 1 as libc::c_int {
        b += 1 as libc::c_int
    }
    b -= 41728 as libc::c_int;
    return b;
}
/*
=============
Q_buildnum_compat

Returns a Xash3D build number. This is left for compability with original Xash3D.
IMPORTANT: this value must be changed ONLY after updating to newer Xash3D
IMPORTANT: this value must be acquired through "build" cvar.
=============
*/
#[no_mangle]
pub unsafe extern "C" fn Q_buildnum_compat() -> libc::c_int {
    // do not touch this! Only author of Xash3D can increase buildnumbers!
    return 4529 as libc::c_int;
}
/*
============
Q_buildos

Returns current name of operating system. Without any spaces.
============
*/
#[no_mangle]
pub unsafe extern "C" fn Q_buildos() -> *const libc::c_char {
    let mut osname: *const libc::c_char = 0 as *const libc::c_char;
    osname = b"linux\x00" as *const u8 as *const libc::c_char;
    return osname;
}
/*
============
Q_buildarch

Returns current name of the architecture. Without any spaces.
============
*/
#[no_mangle]
pub unsafe extern "C" fn Q_buildarch() -> *const libc::c_char {
    let mut archname: *const libc::c_char = 0 as *const libc::c_char;
    archname = b"amd64\x00" as *const u8 as *const libc::c_char;
    return archname;
}
/*
=============
Q_buildcommit

Returns a short hash of current commit in VCS as string.
XASH_BUILD_COMMIT must be passed in quotes

if XASH_BUILD_COMMIT is not defined,
Q_buildcommit will identify this build as "notset"
=============
*/
#[no_mangle]
pub unsafe extern "C" fn Q_buildcommit() -> *const libc::c_char {
    return b"576f0a7\x00" as *const u8 as *const libc::c_char;
}
