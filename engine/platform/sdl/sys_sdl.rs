#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type SDL_Window;
    pub type decallist_s;
    #[no_mangle]
    fn SDL_GetError() -> *const libc::c_char;
    #[no_mangle]
    fn SDL_Init(flags: Uint32) -> libc::c_int;
    #[no_mangle]
    fn SDL_StopTextInput();
    #[no_mangle]
    fn SDL_SetHint(name: *const libc::c_char, value: *const libc::c_char)
     -> SDL_bool;
    #[no_mangle]
    fn SDL_ShowSimpleMessageBox(flags: Uint32, title: *const libc::c_char,
                                message: *const libc::c_char,
                                window: *mut SDL_Window) -> libc::c_int;
    #[no_mangle]
    fn SDL_GetPerformanceCounter() -> Uint64;
    #[no_mangle]
    fn SDL_GetPerformanceFrequency() -> Uint64;
    #[no_mangle]
    fn SDL_Delay(ms: Uint32);
    #[no_mangle]
    fn Sys_Warn(format: *const libc::c_char, _: ...);
    #[no_mangle]
    static mut host: host_parm_t;
    // XASH_MESSAGEBOX == MSGBOX_SDL
    #[no_mangle]
    fn Posix_Daemonize();
}
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type size_t = libc::c_ulong;
pub type uint = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type SDL_bool = libc::c_uint;
pub const SDL_TRUE: SDL_bool = 1;
pub const SDL_FALSE: SDL_bool = 0;
pub type Uint32 = uint32_t;
pub type Uint64 = uint64_t;
pub type C2RustUnnamed = libc::c_uint;
pub const SDL_MESSAGEBOX_INFORMATION: C2RustUnnamed = 64;
pub const SDL_MESSAGEBOX_WARNING: C2RustUnnamed = 32;
pub const SDL_MESSAGEBOX_ERROR: C2RustUnnamed = 16;
pub type HANDLE = *mut libc::c_void;
pub type HINSTANCE = *mut libc::c_void;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const HOST_DEDICATED: C2RustUnnamed_0 = 1;
pub const HOST_NORMAL: C2RustUnnamed_0 = 0;
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
pub type longtime_t = uint64_t;
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
sys_sdl.c - SDL2 system utils
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
pub unsafe extern "C" fn Platform_DoubleTime() -> libc::c_double {
    static mut g_PerformanceFrequency: longtime_t = 0;
    static mut g_ClockStart: longtime_t = 0;
    let mut CurrentTime: longtime_t = 0;
    if g_PerformanceFrequency == 0 {
        g_PerformanceFrequency = SDL_GetPerformanceFrequency();
        g_ClockStart = SDL_GetPerformanceCounter()
    }
    CurrentTime = SDL_GetPerformanceCounter();
    return CurrentTime.wrapping_sub(g_ClockStart) as libc::c_double /
               g_PerformanceFrequency as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn Platform_Sleep(mut msec: libc::c_int) {
    SDL_Delay(msec as Uint32);
}
// XASH_TIMER == TIMER_SDL
#[no_mangle]
pub unsafe extern "C" fn Platform_MessageBox(mut title: *const libc::c_char,
                                             mut message: *const libc::c_char,
                                             mut parentMainWindow: qboolean) {
    SDL_ShowSimpleMessageBox(SDL_MESSAGEBOX_ERROR as libc::c_int as Uint32,
                             title, message,
                             if parentMainWindow as libc::c_uint != 0 {
                                 host.hWnd
                             } else { 0 as *mut libc::c_void } as
                                 *mut SDL_Window);
}
#[no_mangle]
pub unsafe extern "C" fn Platform_Init() {
    if SDL_Init(0x1 as libc::c_uint | 0x20 as libc::c_uint |
                    0x4000 as libc::c_uint) != 0 {
        Sys_Warn(b"SDL_Init failed: %s\x00" as *const u8 as
                     *const libc::c_char, SDL_GetError());
        host.type_0 = HOST_DEDICATED as libc::c_int as uint
    }
    SDL_SetHint(b"SDL_ACCELEROMETER_AS_JOYSTICK\x00" as *const u8 as
                    *const libc::c_char,
                b"0\x00" as *const u8 as *const libc::c_char);
    SDL_StopTextInput();
    // XASH_SDL == 2
    Posix_Daemonize();
}
#[no_mangle]
pub unsafe extern "C" fn Platform_Shutdown() { }
