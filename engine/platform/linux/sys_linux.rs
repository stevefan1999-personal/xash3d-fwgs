#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_transmute, register_tool)]
extern "C" {
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t)
     -> ssize_t;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Q_atoi(str: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn Q_strstr(string: *const libc::c_char, string2: *const libc::c_char)
     -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type byte = libc::c_uchar;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
/*
sys_linux.c - Linux system utils
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
pub unsafe extern "C" fn Sys_DebuggerPresent() -> qboolean {
    let mut buf: [libc::c_char; 4096] = [0; 4096];
    let mut num_read: ssize_t = 0;
    let mut status_fd: libc::c_int = 0;
    status_fd =
        open(b"/proc/self/status\x00" as *const u8 as *const libc::c_char,
             0 as libc::c_int);
    if status_fd == -(1 as libc::c_int) { return false_0 }
    num_read =
        read(status_fd, buf.as_mut_ptr() as *mut libc::c_void,
             ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong);
    close(status_fd);
    if num_read > 0 as libc::c_int as libc::c_long {
        static mut TracerPid: [libc::c_char; 11] =
            unsafe {
                *::std::mem::transmute::<&[u8; 11],
                                         &[libc::c_char; 11]>(b"TracerPid:\x00")
            };
        let mut tracer_pid: *const byte = 0 as *const byte;
        buf[num_read as usize] = 0 as libc::c_int as libc::c_char;
        tracer_pid =
            Q_strstr(buf.as_mut_ptr(), TracerPid.as_ptr()) as *const byte;
        if tracer_pid.is_null() { return false_0 }
        //printf( "%s\n", tracer_pid );
        while (*tracer_pid as libc::c_int) < '0' as i32 ||
                  *tracer_pid as libc::c_int > '9' as i32 {
            let fresh0 = tracer_pid;
            tracer_pid = tracer_pid.offset(1);
            if *fresh0 as libc::c_int == '\n' as i32 { return false_0 }
        }
        //printf( "%s\n", tracer_pid );
        return (Q_atoi(tracer_pid as *const libc::c_char) != 0) as libc::c_int
                   as qboolean
    }
    return false_0;
}
