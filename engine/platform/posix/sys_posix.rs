#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types,
           ptr_wrapping_offset_from, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type decallist_s;
    #[no_mangle]
    fn access(__name: *const libc::c_char, __type: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn execv(__path: *const libc::c_char, __argv: *const *mut libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn _exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn setsid() -> __pid_t;
    #[no_mangle]
    fn fork() -> __pid_t;
    #[no_mangle]
    fn umask(__mask: __mode_t) -> __mode_t;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn Q_strlen(string: *const libc::c_char) -> size_t;
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Q_strchr(s: *const libc::c_char, c: libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn Q_strncmp(s1: *const libc::c_char, s2: *const libc::c_char,
                 n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Sys_CheckParm(parm: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    fn Host_Error(error: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_Reportf(szFmt: *const libc::c_char, _: ...);
}
pub type __uint32_t = libc::c_uint;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type size_t = libc::c_ulong;
pub type pid_t = __pid_t;
pub type uint = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type HANDLE = *mut libc::c_void;
pub type HINSTANCE = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type uint32_t = __uint32_t;
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
pub type vec_t = libc::c_float;
pub type vec3_t = [vec_t; 3];
pub type poolhandle_t = uint32_t;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type dword = libc::c_uint;
pub type string = [libc::c_char; 256];
pub type host_status_t = libc::c_uint;
pub const HOST_CRASHED: host_status_t = 6;
pub const HOST_NOFOCUS: host_status_t = 5;
pub const HOST_SLEEP: host_status_t = 4;
pub const HOST_ERR_FATAL: host_status_t = 3;
pub const HOST_SHUTDOWN: host_status_t = 2;
pub const HOST_FRAME: host_status_t = 1;
pub const HOST_INIT: host_status_t = 0;
pub type host_state_t = libc::c_uint;
pub const STATE_GAME_SHUTDOWN: host_state_t = 4;
pub const STATE_CHANGELEVEL: host_state_t = 3;
pub const STATE_LOAD_GAME: host_state_t = 2;
pub const STATE_LOAD_LEVEL: host_state_t = 1;
pub const STATE_RUNFRAME: host_state_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct game_status_t {
    pub curstate: host_state_t,
    pub nextstate: host_state_t,
    pub levelName: [libc::c_char; 64],
    pub landmarkName: [libc::c_char; 64],
    pub backgroundMap: qboolean,
    pub loadGame: qboolean,
    pub newGame: qboolean,
}
pub type rdtype_t = libc::c_uint;
pub const RD_PACKET: rdtype_t = 2;
pub const RD_CLIENT: rdtype_t = 1;
pub const RD_NONE: rdtype_t = 0;
pub type netadrtype_t = libc::c_uint;
pub const NA_BROADCAST_IPX: netadrtype_t = 5;
pub const NA_IPX: netadrtype_t = 4;
pub const NA_IP: netadrtype_t = 3;
pub const NA_BROADCAST: netadrtype_t = 2;
pub const NA_LOOPBACK: netadrtype_t = 1;
pub const NA_UNUSED: netadrtype_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct netadr_s {
    pub type_0: netadrtype_t,
    pub ip: [libc::c_uchar; 4],
    pub ipx: [libc::c_uchar; 10],
    pub port: libc::c_ushort,
}
pub type netadr_t = netadr_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct host_redirect_s {
    pub target: rdtype_t,
    pub buffer: *mut libc::c_char,
    pub buffersize: size_t,
    pub address: netadr_t,
    pub flush: Option<unsafe extern "C" fn(_: netadr_t, _: rdtype_t,
                                           _: *mut libc::c_char) -> ()>,
    pub lines: libc::c_int,
}
pub type host_redirect_t = host_redirect_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct host_parm_s {
    pub hInst: HINSTANCE,
    pub hMutex: HANDLE,
    pub status: host_status_t,
    pub game: game_status_t,
    pub type_0: uint,
    pub abortframe: jmp_buf,
    pub errorframe: dword,
    pub mempool: poolhandle_t,
    pub finalmsg: string,
    pub downloadfile: string,
    pub downloadcount: libc::c_int,
    pub deferred_cmd: [libc::c_char; 128],
    pub rd: host_redirect_t,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
    pub realtime: libc::c_double,
    pub frametime: libc::c_double,
    pub realframetime: libc::c_double,
    pub framecount: uint,
    pub draw_decals: [[libc::c_char; 64]; 512],
    pub player_mins: [vec3_t; 4],
    pub player_maxs: [vec3_t; 4],
    pub hWnd: *mut libc::c_void,
    pub allow_console: qboolean,
    pub allow_console_init: qboolean,
    pub key_overstrike: qboolean,
    pub stuffcmds_pending: qboolean,
    pub allow_cheats: qboolean,
    pub con_showalways: qboolean,
    pub change_game: qboolean,
    pub mouse_visible: qboolean,
    pub shutdown_issued: qboolean,
    pub force_draw_version: qboolean,
    pub force_draw_version_time: libc::c_float,
    pub apply_game_config: qboolean,
    pub apply_opengl_config: qboolean,
    pub config_executed: qboolean,
    pub sv_cvars_restored: libc::c_int,
    pub crashed: qboolean,
    pub daemonized: qboolean,
    pub enabledll: qboolean,
    pub textmode: qboolean,
    pub userinfo_changed: qboolean,
    pub movevars_changed: qboolean,
    pub renderinfo_changed: qboolean,
    pub rootdir: [libc::c_char; 260],
    pub rodir: [libc::c_char; 260],
    pub gamefolder: [libc::c_char; 64],
    pub imagepool: poolhandle_t,
    pub soundpool: poolhandle_t,
    pub features: uint,
    pub window_center_x: libc::c_int,
    pub window_center_y: libc::c_int,
    pub decalList: *mut decallist_s,
    pub numdecals: libc::c_int,
}
pub type host_parm_t = host_parm_s;
/*
sys_win.c - posix system utils
Copyright (C) 2019 a1batross

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
*/
// fork
unsafe extern "C" fn Sys_FindExecutable(mut baseName: *const libc::c_char,
                                        mut buf: *mut libc::c_char,
                                        mut size: size_t) -> qboolean {
    let mut envPath: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut part: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut length: size_t = 0;
    let mut baseNameLength: size_t = 0;
    let mut needTrailingSlash: size_t = 0;
    if baseName.is_null() || *baseName.offset(0 as libc::c_int as isize) == 0
       {
        return false_0
    }
    envPath = getenv(b"PATH\x00" as *const u8 as *const libc::c_char);
    if if envPath.is_null() || *envPath == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return false_0
    }
    baseNameLength = Q_strlen(baseName);
    while *envPath != 0 {
        part = Q_strchr(envPath, ':' as i32 as libc::c_char);
        if !part.is_null() {
            length =
                part.wrapping_offset_from(envPath) as libc::c_long as size_t
        } else { length = Q_strlen(envPath) }
        if length > 0 as libc::c_int as libc::c_ulong {
            needTrailingSlash =
                if *envPath.offset(length.wrapping_sub(1 as libc::c_int as
                                                           libc::c_ulong) as
                                       isize) as libc::c_int == '/' as i32 {
                    0 as libc::c_int
                } else { 1 as libc::c_int } as size_t;
            if length.wrapping_add(baseNameLength).wrapping_add(needTrailingSlash)
                   < size {
                Q_strncpy(buf, envPath,
                          length.wrapping_add(1 as libc::c_int as
                                                  libc::c_ulong));
                if needTrailingSlash != 0 {
                    Q_strncpy(buf.offset(length as isize),
                              b"/\x00" as *const u8 as *const libc::c_char,
                              99999 as libc::c_int as size_t);
                }
                Q_strncpy(buf.offset(length as
                                         isize).offset(needTrailingSlash as
                                                           isize), baseName,
                          99999 as libc::c_int as size_t);
                *buf.offset(length.wrapping_add(needTrailingSlash).wrapping_add(baseNameLength)
                                as isize) = '\u{0}' as i32 as libc::c_char;
                if access(buf, 1 as libc::c_int) == 0 as libc::c_int {
                    return true_0
                }
            }
        }
        envPath = envPath.offset(length as isize);
        if *envPath as libc::c_int == ':' as i32 {
            envPath = envPath.offset(1)
        }
    }
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn Platform_ShellExecute(mut path: *const libc::c_char,
                                               mut parms:
                                                   *const libc::c_char) {
    let mut xdgOpen: [libc::c_char; 128] = [0; 128];
    if Q_strncmp(path,
                 b"GenericUpdatePage\x00" as *const u8 as *const libc::c_char,
                 99999 as libc::c_int) == 0 ||
           Q_strncmp(path,
                     b"PlatformUpdatePage\x00" as *const u8 as
                         *const libc::c_char, 99999 as libc::c_int) == 0 {
        path =
            b"https://github.com/FWGS/xash3d-fwgs/releases/latest\x00" as
                *const u8 as *const libc::c_char
    }
    if Sys_FindExecutable(b"xdg-open\x00" as *const u8 as *const libc::c_char,
                          xdgOpen.as_mut_ptr(),
                          ::std::mem::size_of::<[libc::c_char; 128]>() as
                              libc::c_ulong) as u64 != 0 {
        let mut argv: [*const libc::c_char; 3] =
            [xdgOpen.as_mut_ptr() as *const libc::c_char, path,
             0 as *const libc::c_char];
        let mut id: pid_t = fork();
        if id == 0 as libc::c_int {
            execv(xdgOpen.as_mut_ptr(),
                  argv.as_mut_ptr() as *mut *mut libc::c_char as
                      *const *mut libc::c_char);
            fprintf(stderr,
                    b"error opening %s %s\x00" as *const u8 as
                        *const libc::c_char, xdgOpen.as_mut_ptr(), path);
            _exit(1 as libc::c_int);
        }
    } else {
        Con_Reportf(b"^3Warning:^7 Could not find xdg-open utility\n\x00" as
                        *const u8 as *const libc::c_char);
    };
}
// XASH_ANDROID
#[no_mangle]
pub unsafe extern "C" fn Posix_Daemonize() {
    // to be accessed later
    host.daemonized =
        Sys_CheckParm(b"-daemonize\x00" as *const u8 as *const libc::c_char)
            as qboolean;
    if host.daemonized as u64 != 0 {
        let mut daemon: pid_t = 0;
        daemon = fork();
        if daemon < 0 as libc::c_int {
            Host_Error(b"fork() failed: %s\n\x00" as *const u8 as
                           *const libc::c_char,
                       strerror(*__errno_location()));
        }
        if daemon > 0 as libc::c_int {
            // parent
            Con_Reportf(b"Child pid: %i\n\x00" as *const u8 as
                            *const libc::c_char, daemon);
            exit(0 as libc::c_int);
        } else {
            // don't be closed by parent
            if setsid() < 0 as libc::c_int {
                Host_Error(b"setsid() failed: %s\n\x00" as *const u8 as
                               *const libc::c_char,
                           strerror(*__errno_location()));
            }
            // stderr
            // fallthrough
            umask(0 as libc::c_int as __mode_t);
            close(0 as libc::c_int);
            close(1 as libc::c_int);
            close(2 as libc::c_int);
            open(b"/dev/null\x00" as *const u8 as *const libc::c_char,
                 0 as libc::c_int);
            open(b"/dev/null\x00" as *const u8 as *const libc::c_char,
                 0o2 as libc::c_int);
            open(b"/dev/null\x00" as *const u8 as *const libc::c_char,
                 0o2 as libc::c_int);
        }
    };
}
// set permissions
// engine will still use stdin/stdout,
			// so just redirect them to /dev/null
// becomes stdin
// stdout
// XASH_TIMER == TIMER_POSIX
