#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type SDL_Window;
    pub type _SDL_Joystick;
    #[no_mangle]
    fn SDL_free(mem: *mut libc::c_void);
    #[no_mangle]
    fn SDL_GetError() -> *const libc::c_char;
    #[no_mangle]
    fn SDL_RWFromFile(file: *const libc::c_char, mode: *const libc::c_char)
     -> *mut SDL_RWops;
    #[no_mangle]
    fn SDL_SetClipboardText(text: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn SDL_GetClipboardText() -> *mut libc::c_char;
    #[no_mangle]
    fn SDL_WasInit(flags: Uint32) -> Uint32;
    #[no_mangle]
    fn SDL_InitSubSystem(flags: Uint32) -> libc::c_int;
    #[no_mangle]
    fn SDL_StartTextInput();
    #[no_mangle]
    fn SDL_StopTextInput();
    #[no_mangle]
    fn SDL_GetMouseState(x: *mut libc::c_int, y: *mut libc::c_int) -> Uint32;
    #[no_mangle]
    fn SDL_GetRelativeMouseState(x: *mut libc::c_int, y: *mut libc::c_int)
     -> Uint32;
    #[no_mangle]
    fn SDL_WarpMouseInWindow(window: *mut SDL_Window, x: libc::c_int,
                             y: libc::c_int);
    #[no_mangle]
    fn SDL_NumJoysticks() -> libc::c_int;
    #[no_mangle]
    fn SDL_JoystickNameForIndex(device_index: libc::c_int)
     -> *const libc::c_char;
    #[no_mangle]
    fn SDL_JoystickOpen(device_index: libc::c_int) -> *mut SDL_Joystick;
    #[no_mangle]
    fn SDL_JoystickName(joystick: *mut SDL_Joystick) -> *const libc::c_char;
    #[no_mangle]
    fn SDL_JoystickNumAxes(joystick: *mut SDL_Joystick) -> libc::c_int;
    #[no_mangle]
    fn SDL_JoystickNumBalls(joystick: *mut SDL_Joystick) -> libc::c_int;
    #[no_mangle]
    fn SDL_JoystickNumHats(joystick: *mut SDL_Joystick) -> libc::c_int;
    #[no_mangle]
    fn SDL_JoystickNumButtons(joystick: *mut SDL_Joystick) -> libc::c_int;
    #[no_mangle]
    fn SDL_JoystickEventState(state: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn SDL_JoystickClose(joystick: *mut SDL_Joystick);
    #[no_mangle]
    fn SDL_GameControllerAddMappingsFromRW(rw: *mut SDL_RWops,
                                           freerw: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn SDL_IsGameController(joystick_index: libc::c_int) -> SDL_bool;
    #[no_mangle]
    fn SDL_GameControllerEventState(state: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Con_Reportf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Sys_CheckParm(parm: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    static mut host: host_parm_t;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type uint = libc::c_uint;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
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
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type SDL_bool = libc::c_uint;
pub const SDL_TRUE: SDL_bool = 1;
pub const SDL_FALSE: SDL_bool = 0;
pub type Uint8 = uint8_t;
pub type Uint32 = uint32_t;
pub type Sint64 = int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_RWops {
    pub size: Option<unsafe extern "C" fn(_: *mut SDL_RWops) -> Sint64>,
    pub seek: Option<unsafe extern "C" fn(_: *mut SDL_RWops, _: Sint64,
                                          _: libc::c_int) -> Sint64>,
    pub read: Option<unsafe extern "C" fn(_: *mut SDL_RWops,
                                          _: *mut libc::c_void, _: size_t,
                                          _: size_t) -> size_t>,
    pub write: Option<unsafe extern "C" fn(_: *mut SDL_RWops,
                                           _: *const libc::c_void, _: size_t,
                                           _: size_t) -> size_t>,
    pub close: Option<unsafe extern "C" fn(_: *mut SDL_RWops) -> libc::c_int>,
    pub type_0: Uint32,
    pub hidden: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub stdio: C2RustUnnamed_2,
    pub mem: C2RustUnnamed_1,
    pub unknown: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub data1: *mut libc::c_void,
    pub data2: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub base: *mut Uint8,
    pub here: *mut Uint8,
    pub stop: *mut Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub autoclose: SDL_bool,
    pub fp: *mut FILE,
}
pub type SDL_Joystick = _SDL_Joystick;
pub type HANDLE = *mut libc::c_void;
pub type HINSTANCE = *mut libc::c_void;
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
pub type byte = libc::c_uchar;
pub type vec_t = libc::c_float;
pub type vec3_t = [vec_t; 3];
pub type poolhandle_t = uint32_t;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type dword = libc::c_uint;
pub type string = [libc::c_char; 256];
pub type host_parm_t = host_parm_s;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct decallist_s {
    pub position: vec3_t,
    pub name: [libc::c_char; 64],
    pub entityIndex: libc::c_short,
    pub depth: byte,
    pub flags: byte,
    pub scale: libc::c_float,
    pub impactPlaneNormal: vec3_t,
    pub studio_state: modelstate_t,
}
pub type modelstate_t = modelstate_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct modelstate_s {
    pub sequence: libc::c_short,
    pub frame: libc::c_short,
    pub blending: [byte; 2],
    pub controller: [byte; 4],
    pub poseparam: [byte; 16],
    pub body: byte,
    pub skin: byte,
    pub scale: libc::c_short,
}
pub type host_redirect_t = host_redirect_s;
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
pub type rdtype_t = libc::c_uint;
pub const RD_PACKET: rdtype_t = 2;
pub const RD_CLIENT: rdtype_t = 1;
pub const RD_NONE: rdtype_t = 0;
pub type netadr_t = netadr_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct netadr_s {
    pub type_0: netadrtype_t,
    pub ip: [libc::c_uchar; 4],
    pub ipx: [libc::c_uchar; 10],
    pub port: libc::c_ushort,
}
pub type netadrtype_t = libc::c_uint;
pub const NA_BROADCAST_IPX: netadrtype_t = 5;
pub const NA_IPX: netadrtype_t = 4;
pub const NA_IP: netadrtype_t = 3;
pub const NA_BROADCAST: netadrtype_t = 2;
pub const NA_LOOPBACK: netadrtype_t = 1;
pub const NA_UNUSED: netadrtype_t = 0;
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
pub type host_state_t = libc::c_uint;
pub const STATE_GAME_SHUTDOWN: host_state_t = 4;
pub const STATE_CHANGELEVEL: host_state_t = 3;
pub const STATE_LOAD_GAME: host_state_t = 2;
pub const STATE_LOAD_LEVEL: host_state_t = 1;
pub const STATE_RUNFRAME: host_state_t = 0;
pub type host_status_t = libc::c_uint;
pub const HOST_CRASHED: host_status_t = 6;
pub const HOST_NOFOCUS: host_status_t = 5;
pub const HOST_SLEEP: host_status_t = 4;
pub const HOST_ERR_FATAL: host_status_t = 3;
pub const HOST_SHUTDOWN: host_status_t = 2;
pub const HOST_FRAME: host_status_t = 1;
pub const HOST_INIT: host_status_t = 0;
/*
vid_sdl.c - SDL input component
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
pub static mut g_joy: *mut SDL_Joystick =
    0 as *const SDL_Joystick as *mut SDL_Joystick;
/*
=============
Platform_GetMousePos

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Platform_GetMousePos(mut x: *mut libc::c_int,
                                              mut y: *mut libc::c_int) {
    SDL_GetMouseState(x, y);
}
/*
=============
Platform_SetMousePos

============
*/
#[no_mangle]
pub unsafe extern "C" fn Platform_SetMousePos(mut x: libc::c_int,
                                              mut y: libc::c_int) {
    SDL_WarpMouseInWindow(host.hWnd as *mut SDL_Window, x, y);
}
/*
========================
Platform_MouseMove

========================
*/
#[no_mangle]
pub unsafe extern "C" fn Platform_MouseMove(mut x: *mut libc::c_float,
                                            mut y: *mut libc::c_float) {
    let mut m_x: libc::c_int = 0;
    let mut m_y: libc::c_int = 0;
    SDL_GetRelativeMouseState(&mut m_x, &mut m_y);
    *x = m_x as libc::c_float;
    *y = m_y as libc::c_float;
}
/*
=============
Platform_GetClipobardText

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Platform_GetClipboardText(mut buffer:
                                                       *mut libc::c_char,
                                                   mut size: size_t) {
    let mut sdlbuffer: *mut libc::c_char = SDL_GetClipboardText();
    if sdlbuffer.is_null() { return }
    Q_strncpy(buffer, sdlbuffer, size);
    SDL_free(sdlbuffer as *mut libc::c_void);
    // SDL_VERSION_ATLEAST( 2, 0, 0 )
    // SDL_VERSION_ATLEAST( 2, 0, 0 )
}
/*
=============
Platform_SetClipobardText

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Platform_SetClipboardText(mut buffer:
                                                       *const libc::c_char,
                                                   mut size: size_t) {
    SDL_SetClipboardText(buffer);
    // SDL_VERSION_ATLEAST( 2, 0, 0 )
}
/*
=============
Platform_Vibrate

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Platform_Vibrate(mut time: libc::c_float,
                                          mut flags: libc::c_char) {
    // stub
}
/*
=============
SDLash_EnableTextInput

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Platform_EnableTextInput(mut enable: qboolean) {
    if enable as libc::c_uint != 0 {
        SDL_StartTextInput();
    } else { SDL_StopTextInput(); };
    // SDL_VERSION_ATLEAST( 2, 0, 0 )
}
/*
=============
SDLash_JoyInit_Old

=============
*/
unsafe extern "C" fn SDLash_JoyInit_Old(mut numjoy: libc::c_int)
 -> libc::c_int {
    let mut num: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    Con_Reportf(b"Joystick: SDL\n\x00" as *const u8 as *const libc::c_char);
    if SDL_WasInit(0x200 as libc::c_uint) != 0x200 as libc::c_uint &&
           SDL_InitSubSystem(0x200 as libc::c_uint) != 0 {
        Con_Reportf(b"Failed to initialize SDL Joysitck: %s\n\x00" as
                        *const u8 as *const libc::c_char, SDL_GetError());
        return 0 as libc::c_int
    }
    if !g_joy.is_null() { SDL_JoystickClose(g_joy); }
    num = SDL_NumJoysticks();
    if num > 0 as libc::c_int {
        Con_Reportf(b"%i joysticks found:\n\x00" as *const u8 as
                        *const libc::c_char, num);
    } else {
        Con_Reportf(b"No joystick found.\n\x00" as *const u8 as
                        *const libc::c_char);
        return 0 as libc::c_int
    }
    i = 0 as libc::c_int;
    while i < num {
        Con_Reportf(b"%i\t: %s\n\x00" as *const u8 as *const libc::c_char, i,
                    SDL_JoystickNameForIndex(i));
        i += 1
    }
    // SDL_VERSION_ATLEAST( 2, 0, 0 )
    Con_Reportf(b"Pass +set joy_index N to command line, where N is number, to select active joystick\n\x00"
                    as *const u8 as *const libc::c_char);
    g_joy = SDL_JoystickOpen(numjoy);
    if g_joy.is_null() {
        Con_Reportf(b"Failed to select joystick: %s\n\x00" as *const u8 as
                        *const libc::c_char, SDL_GetError());
        return 0 as libc::c_int
    }
    Con_Reportf(b"Selected joystick: %s\n\tAxes: %i\n\tHats: %i\n\tButtons: %i\n\tBalls: %i\n\x00"
                    as *const u8 as *const libc::c_char,
                SDL_JoystickName(g_joy), SDL_JoystickNumAxes(g_joy),
                SDL_JoystickNumHats(g_joy), SDL_JoystickNumButtons(g_joy),
                SDL_JoystickNumBalls(g_joy));
    SDL_GameControllerEventState(0 as libc::c_int);
    // SDL_VERSION_ATLEAST( 2, 0, 0 )
    SDL_JoystickEventState(1 as libc::c_int);
    return num;
}
/*
=============
SDLash_JoyInit_New

=============
*/
unsafe extern "C" fn SDLash_JoyInit_New(mut numjoy: libc::c_int)
 -> libc::c_int {
    let mut count: libc::c_int = 0;
    let mut numJoysticks: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    Con_Reportf(b"Joystick: SDL GameController API\n\x00" as *const u8 as
                    *const libc::c_char);
    if SDL_WasInit(0x2000 as libc::c_uint) != 0x2000 as libc::c_uint &&
           SDL_InitSubSystem(0x2000 as libc::c_uint) != 0 {
        Con_Reportf(b"Failed to initialize SDL GameController API: %s\n\x00"
                        as *const u8 as *const libc::c_char, SDL_GetError());
        return 0 as libc::c_int
    }
    SDL_GameControllerAddMappingsFromRW(SDL_RWFromFile(b"controllermappings.txt\x00"
                                                           as *const u8 as
                                                           *const libc::c_char,
                                                       b"rb\x00" as *const u8
                                                           as
                                                           *const libc::c_char),
                                        1 as libc::c_int);
    count = 0 as libc::c_int;
    numJoysticks = SDL_NumJoysticks();
    i = 0 as libc::c_int;
    while i < numJoysticks {
        if SDL_IsGameController(i) as u64 != 0 { count += 1 }
        i += 1
    }
    return count;
}
// SDL_VERSION_ATLEAST( 2, 0, 0 )
/*
=============
Platform_JoyInit

=============
*/
#[no_mangle]
pub unsafe extern "C" fn Platform_JoyInit(mut numjoy: libc::c_int)
 -> libc::c_int {
    // SDL_Joystick is now an old API
	// SDL_GameController is preferred
    if Sys_CheckParm(b"-sdl_joy_old_api\x00" as *const u8 as
                         *const libc::c_char) == 0 {
        return SDLash_JoyInit_New(numjoy)
    }
    // SDL_VERSION_ATLEAST( 2, 0, 0 )
    return SDLash_JoyInit_Old(numjoy);
}
// XASH_DEDICATED
