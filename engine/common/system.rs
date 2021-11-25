#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, const_raw_ptr_to_usize_cast, extern_types,
           register_tool)]
extern "C" {
    pub type SDL_Window;
    pub type decallist_s;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn dlsym(__handle: *mut libc::c_void, __name: *const libc::c_char)
     -> *mut libc::c_void;
    #[no_mangle]
    fn dlclose(__handle: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn dlopen(__file: *const libc::c_char, __mode: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    fn geteuid() -> __uid_t;
    #[no_mangle]
    fn SDL_HideWindow(window: *mut SDL_Window);
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Q_atoi(str: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn Q_strnicmp(s1: *const libc::c_char, s2: *const libc::c_char,
                  n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Q_vsnprintf(buffer: *mut libc::c_char, buffersize: size_t,
                   format: *const libc::c_char, args: ::std::ffi::VaList)
     -> libc::c_int;
    #[no_mangle]
    fn Q_snprintf(buffer: *mut libc::c_char, buffersize: size_t,
                  format: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn Platform_DoubleTime() -> libc::c_double;
    #[no_mangle]
    fn Platform_Sleep(msec: libc::c_int);
    #[no_mangle]
    fn Platform_MessageBox(title: *const libc::c_char,
                           message: *const libc::c_char,
                           parentMainWindow: qboolean);
    #[no_mangle]
    fn Platform_GetClipboardText(buffer: *mut libc::c_char, size: size_t);
    #[no_mangle]
    fn Platform_SetClipboardText(buffer: *const libc::c_char, size: size_t);
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Host_Shutdown();
    #[no_mangle]
    fn Rcon_Print(pMsg: *const libc::c_char);
    #[no_mangle]
    fn Sys_PrintLog(pMsg: *const libc::c_char);
    #[no_mangle]
    fn Con_Print(txt: *const libc::c_char);
    #[no_mangle]
    static mut host_developer: convar_t;
    #[no_mangle]
    fn SV_SysError(error_string: *const libc::c_char);
    #[no_mangle]
    fn Con_Reportf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn raise(__sig: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    #[no_mangle]
    fn Sys_DebuggerPresent() -> qboolean;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type uid_t = __uid_t;
pub type HANDLE = *mut libc::c_void;
pub type HINSTANCE = *mut libc::c_void;
pub type uint = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type va_list = __builtin_va_list;
pub type uint32_t = __uint32_t;
pub type C2RustUnnamed = libc::c_uint;
pub const HOST_DEDICATED: C2RustUnnamed = 1;
pub const HOST_NORMAL: C2RustUnnamed = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dllfunc_s {
    pub name: *const libc::c_char,
    pub func: *mut *mut libc::c_void,
}
pub type dllfunc_t = dllfunc_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dll_info_s {
    pub name: *const libc::c_char,
    pub fcts: *const dllfunc_t,
    pub crash: qboolean,
    pub link: *mut libc::c_void,
}
pub type dll_info_t = dll_info_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
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
pub type convar_t = convar_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct convar_s {
    pub name: *mut libc::c_char,
    pub string: *mut libc::c_char,
    pub flags: libc::c_int,
    pub value: libc::c_float,
    pub next: *mut convar_s,
    pub desc: *mut libc::c_char,
    pub def_string: *mut libc::c_char,
}
/*
sys_win.c - platform dependent code
Copyright (C) 2011 Uncle Mike

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
*/
// _UPDATE_PAGE macro
#[no_mangle]
pub static mut error_on_exit: qboolean = false_0;
// arg for exit();
/*
================
Sys_DoubleTime
================
*/
#[no_mangle]
pub unsafe extern "C" fn Sys_DoubleTime() -> libc::c_double {
    return Platform_DoubleTime();
}
// see sys_linux.c
/*
================
Sys_GetClipboardData

create buffer, that contain clipboard
================
*/
#[no_mangle]
pub unsafe extern "C" fn Sys_GetClipboardData() -> *mut libc::c_char {
    static mut data: [libc::c_char; 1024] = [0; 1024];
    data[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    Platform_GetClipboardText(data.as_mut_ptr(),
                              ::std::mem::size_of::<[libc::c_char; 1024]>() as
                                  libc::c_ulong);
    return data.as_mut_ptr();
}
/*
================
Sys_SetClipboardData

write screenshot into clipboard
================
*/
#[no_mangle]
pub unsafe extern "C" fn Sys_SetClipboardData(mut buffer: *const libc::c_char,
                                              mut size: size_t) {
    Platform_SetClipboardText(buffer, size);
}
// XASH_DEDICATED
/*
================
Sys_Sleep

freeze application for some time
================
*/
#[no_mangle]
pub unsafe extern "C" fn Sys_Sleep(mut msec: libc::c_int) {
    if msec == 0 { return }
    msec =
        if msec < 1000 as libc::c_int { msec } else { 1000 as libc::c_int };
    Platform_Sleep(msec);
}
/*
================
Sys_GetCurrentUser

returns username for current profile
================
*/
#[no_mangle]
pub unsafe extern "C" fn Sys_GetCurrentUser() -> *const libc::c_char {
    let mut uid: uid_t = geteuid();
    let mut pw: *mut passwd = getpwuid(uid);
    if !pw.is_null() { return (*pw).pw_name }
    return b"Player\x00" as *const u8 as *const libc::c_char;
}
/*
==================
Sys_ParseCommandLine

==================
*/
#[no_mangle]
pub unsafe extern "C" fn Sys_ParseCommandLine(mut argc: libc::c_int,
                                              mut argv:
                                                  *mut *mut libc::c_char) {
    let mut blank: *const libc::c_char =
        b"censored\x00" as *const u8 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    host.argc = argc;
    host.argv = argv;
    if host.change_game as u64 == 0 { return }
    i = 0 as libc::c_int;
    while i < host.argc {
        // we don't want to return to first game
        if Q_strnicmp(b"-game\x00" as *const u8 as *const libc::c_char,
                      *host.argv.offset(i as isize), 99999 as libc::c_int) ==
               0 {
            let ref mut fresh0 = *host.argv.offset(i as isize);
            *fresh0 = blank as *mut libc::c_char
        } else if Q_strnicmp(b"+game\x00" as *const u8 as *const libc::c_char,
                             *host.argv.offset(i as isize),
                             99999 as libc::c_int) == 0 {
            let ref mut fresh1 = *host.argv.offset(i as isize);
            *fresh1 = blank as *mut libc::c_char
        } else if Q_strnicmp(b"+map\x00" as *const u8 as *const libc::c_char,
                             *host.argv.offset(i as isize),
                             99999 as libc::c_int) == 0 {
            let ref mut fresh2 = *host.argv.offset(i as isize);
            *fresh2 = blank as *mut libc::c_char
        } else if Q_strnicmp(b"+load\x00" as *const u8 as *const libc::c_char,
                             *host.argv.offset(i as isize),
                             99999 as libc::c_int) == 0 {
            let ref mut fresh3 = *host.argv.offset(i as isize);
            *fresh3 = blank as *mut libc::c_char
        } else if Q_strnicmp(b"+changelevel\x00" as *const u8 as
                                 *const libc::c_char,
                             *host.argv.offset(i as isize),
                             99999 as libc::c_int) == 0 {
            let ref mut fresh4 = *host.argv.offset(i as isize);
            *fresh4 = blank as *mut libc::c_char
        }
        i += 1
    };
}
// probably it's timewaster, because engine rejected second change
// you sure that map exists in new game?
// just stupid action
// changelevel beetwen games? wow it's great idea!
/*
==================
Sys_MergeCommandLine

==================
*/
#[no_mangle]
pub unsafe extern "C" fn Sys_MergeCommandLine() {
    let mut blank: *const libc::c_char =
        b"censored\x00" as *const u8 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    if host.change_game as u64 == 0 { return }
    i = 0 as libc::c_int;
    while i < host.argc {
        // second call
        if host.type_0 == HOST_DEDICATED as libc::c_int as libc::c_uint &&
               Q_strnicmp(b"+menu_\x00" as *const u8 as *const libc::c_char,
                          *host.argv.offset(i as isize), 6 as libc::c_int) ==
                   0 {
            let ref mut fresh5 = *host.argv.offset(i as isize);
            *fresh5 = blank as *mut libc::c_char
        }
        i += 1
    };
}
/*
================
Sys_CheckParm

Returns the position (1 to argc-1) in the program's argument list
where the given parameter apears, or 0 if not present
================
*/
#[no_mangle]
pub unsafe extern "C" fn Sys_CheckParm(mut parm: *const libc::c_char)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < host.argc {
        if !(*host.argv.offset(i as isize)).is_null() {
            if Q_strnicmp(parm, *host.argv.offset(i as isize),
                          99999 as libc::c_int) == 0 {
                return i
            }
        }
        i += 1
    }
    return 0 as libc::c_int;
}
/*
================
Sys_GetParmFromCmdLine

Returns the argument for specified parm
================
*/
#[no_mangle]
pub unsafe extern "C" fn _Sys_GetParmFromCmdLine(mut parm:
                                                     *const libc::c_char,
                                                 mut out: *mut libc::c_char,
                                                 mut size: size_t)
 -> qboolean {
    let mut argc: libc::c_int = Sys_CheckParm(parm);
    if argc == 0 || out.is_null() ||
           (*host.argv.offset((argc + 1 as libc::c_int) as isize)).is_null() {
        return false_0
    }
    Q_strncpy(out, *host.argv.offset((argc + 1 as libc::c_int) as isize),
              size);
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn Sys_GetIntFromCmdLine(mut argName:
                                                   *const libc::c_char,
                                               mut out: *mut libc::c_int)
 -> qboolean {
    let mut argIndex: libc::c_int = Sys_CheckParm(argName);
    if argIndex < 1 as libc::c_int || argIndex + 1 as libc::c_int >= host.argc
           ||
           (*host.argv.offset((argIndex + 1 as libc::c_int) as
                                  isize)).is_null() {
        *out = 0 as libc::c_int;
        return false_0
    }
    *out = Q_atoi(*host.argv.offset((argIndex + 1 as libc::c_int) as isize));
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn Sys_SendKeyEvents() { }
//=======================================================================
//			DLL'S MANAGER SYSTEM
//=======================================================================
#[no_mangle]
pub unsafe extern "C" fn Sys_LoadLibrary(mut dll: *mut dll_info_t)
 -> qboolean {
    let mut current_block: u64;
    let mut func: *const dllfunc_t = 0 as *const dllfunc_t;
    let mut errorstring: string = [0; 256];
    // check errors
    if dll.is_null() { return false_0 } // invalid desc
    if !(*dll).link.is_null() { return true_0 } // already loaded
    if (*dll).name.is_null() || *(*dll).name == 0 {
        return false_0
    } // nothing to load
    Con_Reportf(b"Sys_LoadLibrary: Loading %s\x00" as *const u8 as
                    *const libc::c_char, (*dll).name);
    if !(*dll).fcts.is_null() {
        // lookup export table
        func = (*dll).fcts; // environment pathes
        while !func.is_null() && !(*func).name.is_null() {
            *(*func).func = 0 as *mut libc::c_void;
            func = func.offset(1)
        }
    }
    if (*dll).link.is_null() {
        (*dll).link = dlopen((*dll).name, 0x2 as libc::c_int)
    }
    // no DLL found
    if (*dll).link.is_null() {
        Q_snprintf(errorstring.as_mut_ptr(),
                   ::std::mem::size_of::<string>() as libc::c_ulong,
                   b"Sys_LoadLibrary: couldn\'t load %s\n\x00" as *const u8 as
                       *const libc::c_char, (*dll).name);
    } else {
        // Get the function adresses
        func = (*dll).fcts; // trying to free
        loop  {
            if !(!func.is_null() && !(*func).name.is_null()) {
                current_block = 4495394744059808450;
                break ;
            }
            *(*func).func = Sys_GetProcAddress(dll, (*func).name);
            if (*(*func).func).is_null() {
                Q_snprintf(errorstring.as_mut_ptr(),
                           ::std::mem::size_of::<string>() as libc::c_ulong,
                           b"Sys_LoadLibrary: %s missing or invalid function (%s)\n\x00"
                               as *const u8 as *const libc::c_char,
                           (*dll).name, (*func).name);
                current_block = 8177123644368231091;
                break ;
            } else { func = func.offset(1) }
        }
        match current_block {
            8177123644368231091 => { }
            _ => {
                Con_Reportf(b" - ok\n\x00" as *const u8 as
                                *const libc::c_char);
                return true_0
            }
        }
    }
    Con_Reportf(b" - failed\n\x00" as *const u8 as *const libc::c_char);
    Sys_FreeLibrary(dll);
    if (*dll).crash as u64 != 0 {
        Sys_Error(b"%s\x00" as *const u8 as *const libc::c_char,
                  errorstring.as_mut_ptr());
    } else {
        Con_Reportf(b"^1Error:^7 %s\x00" as *const u8 as *const libc::c_char,
                    errorstring.as_mut_ptr());
    }
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn Sys_GetProcAddress(mut dll: *mut dll_info_t,
                                            mut name: *const libc::c_char)
 -> *mut libc::c_void {
    if dll.is_null() || (*dll).link.is_null() {
        // invalid desc
        return 0 as *mut libc::c_void
    }
    return dlsym((*dll).link, name);
}
#[no_mangle]
pub unsafe extern "C" fn Sys_FreeLibrary(mut dll: *mut dll_info_t)
 -> qboolean {
    // invalid desc or alredy freed
    if dll.is_null() || (*dll).link.is_null() { return false_0 }
    if host.status as libc::c_uint ==
           HOST_CRASHED as libc::c_int as libc::c_uint {
        // we need to hold down all modules, while MSVC can find error
        Con_Reportf(b"Sys_FreeLibrary: hold %s for debugging\n\x00" as
                        *const u8 as *const libc::c_char, (*dll).name);
        return false_0
    } else {
        Con_Reportf(b"Sys_FreeLibrary: Unloading %s\n\x00" as *const u8 as
                        *const libc::c_char, (*dll).name);
    }
    dlclose((*dll).link);
    (*dll).link = 0 as *mut libc::c_void;
    return true_0;
}
/*
================
Sys_WaitForQuit

wait for 'Esc' key will be hit
================
*/
#[no_mangle]
pub unsafe extern "C" fn Sys_WaitForQuit() { }
/*
================
Sys_Warn

Just messagebox
================
*/
#[no_mangle]
pub unsafe extern "C" fn Sys_Warn(mut format: *const libc::c_char,
                                  mut args: ...) {
    let mut argptr: ::std::ffi::VaListImpl;
    let mut text: [libc::c_char; 8192] = [0; 8192];
    if Sys_DebuggerPresent() as u64 != 0 { raise(2 as libc::c_int); }
    argptr = args.clone();
    Q_vsnprintf(text.as_mut_ptr(), 8192 as libc::c_int as size_t, format,
                argptr.as_va_list());
    Con_Printf(b"Sys_Warn: %s\n\x00" as *const u8 as *const libc::c_char,
               text.as_mut_ptr());
    if !(host.type_0 == HOST_DEDICATED as libc::c_int as libc::c_uint) {
        // dedicated server should not hang on messagebox
        Platform_MessageBox(b"Xash Error\x00" as *const u8 as
                                *const libc::c_char, text.as_mut_ptr(),
                            false_0);
    };
}
/*
================
Sys_Error

NOTE: we must prepare engine to shutdown
before call this
================
*/
#[no_mangle]
pub unsafe extern "C" fn Sys_Error(mut error: *const libc::c_char,
                                   mut args: ...) {
    let mut argptr: ::std::ffi::VaListImpl; // don't multiple executes
    let mut text: [libc::c_char; 8192] = [0; 8192];
    if Sys_DebuggerPresent() as u64 != 0 { raise(2 as libc::c_int); }
    if host.status as libc::c_uint ==
           HOST_ERR_FATAL as libc::c_int as libc::c_uint {
        return
    }
    // make sure what console received last message
    if host.change_game as u64 != 0 {
        Sys_Sleep(200 as libc::c_int); // print error message
    }
    error_on_exit = true_0;
    host.status = HOST_ERR_FATAL;
    argptr = args.clone();
    Q_vsnprintf(text.as_mut_ptr(), 8192 as libc::c_int as size_t, error,
                argptr.as_va_list());
    SV_SysError(text.as_mut_ptr());
    if !(host.type_0 == HOST_DEDICATED as libc::c_int as libc::c_uint) {
        if !host.hWnd.is_null() {
            SDL_HideWindow(host.hWnd as *mut SDL_Window);
        }
    }
    if host_developer.value != 0. {
        Sys_Print(text.as_mut_ptr());
        Sys_WaitForQuit();
    } else {
        Platform_MessageBox(b"Xash Error\x00" as *const u8 as
                                *const libc::c_char, text.as_mut_ptr(),
                            false_0);
    }
    Sys_Quit();
}
/*
================
Sys_Quit
================
*/
#[no_mangle]
pub unsafe extern "C" fn Sys_Quit() -> ! {
    Host_Shutdown();
    exit(error_on_exit as libc::c_int);
}
/*
================
Sys_Print

print into window console
================
*/
#[no_mangle]
pub unsafe extern "C" fn Sys_Print(mut pMsg: *const libc::c_char) {
    if !(host.type_0 == HOST_DEDICATED as libc::c_int as libc::c_uint) {
        Con_Print(pMsg);
    }
    Sys_PrintLog(pMsg);
    Rcon_Print(pMsg);
}
