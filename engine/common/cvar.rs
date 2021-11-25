#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, extern_types,
           register_tool)]
extern "C" {
    pub type file_s;
    pub type decallist_s;
    pub type sv_client_s;
    #[no_mangle]
    fn fabs(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn Q_strlen(string: *const libc::c_char) -> size_t;
    #[no_mangle]
    fn Q_colorstr(string: *const libc::c_char) -> size_t;
    #[no_mangle]
    fn Q_strncat(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Q_atoi(str: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn Q_atof(str: *const libc::c_char) -> libc::c_float;
    #[no_mangle]
    fn Q_strnicmp(s1: *const libc::c_char, s2: *const libc::c_char,
                  n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Q_strncmp(s1: *const libc::c_char, s2: *const libc::c_char,
                 n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Q_stricmpext(s1: *const libc::c_char, s2: *const libc::c_char)
     -> qboolean;
    #[no_mangle]
    fn Q_snprintf(buffer: *mut libc::c_char, buffersize: size_t,
                  format: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn Sys_Error(error: *const libc::c_char, _: ...);
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    fn SV_BroadcastPrintf(ignore: *mut sv_client_s, fmt: *const libc::c_char,
                          _: ...);
    #[no_mangle]
    fn Log_Printf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn CL_LegacyUpdateInfo();
    #[no_mangle]
    fn CL_ServerCommand(reliable: qboolean, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn CL_Userinfo() -> *mut libc::c_char;
    #[no_mangle]
    fn Info_SetValueForKey(s: *mut libc::c_char, key: *const libc::c_char,
                           value: *const libc::c_char, maxsize: libc::c_int)
     -> qboolean;
    #[no_mangle]
    fn SV_Serverinfo() -> *mut libc::c_char;
    #[no_mangle]
    fn SV_BroadcastCommand(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn _copystring(mempool: poolhandle_t, s: *const libc::c_char,
                   filename: *const libc::c_char, fileline: libc::c_int)
     -> *mut libc::c_char;
    #[no_mangle]
    fn Con_DPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Cmd_Exists(cmd_name: *const libc::c_char) -> qboolean;
    #[no_mangle]
    fn _Mem_Free(data: *mut libc::c_void, filename: *const libc::c_char,
                 fileline: libc::c_int);
    #[no_mangle]
    fn _Mem_Alloc(poolptr: poolhandle_t, size: size_t, clear: qboolean,
                  filename: *const libc::c_char, fileline: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    fn Con_Reportf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn CL_Active() -> libc::c_int;
    #[no_mangle]
    fn SV_Active() -> qboolean;
    #[no_mangle]
    fn FS_Printf(file: *mut file_t, format: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn Cmd_Argv(arg: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn CL_GetMaxClients() -> libc::c_int;
    #[no_mangle]
    fn Cmd_Argc() -> libc::c_int;
    #[no_mangle]
    fn Cmd_AddCommand(cmd_name: *const libc::c_char, function: xcommand_t,
                      cmd_desc: *const libc::c_char);
    #[no_mangle]
    fn Cmd_AddRestrictedCommand(cmd_name: *const libc::c_char,
                                function: xcommand_t,
                                cmd_desc: *const libc::c_char);
    #[no_mangle]
    static mut host_developer: convar_t;
    #[no_mangle]
    fn BaseCmd_Find(type_0: base_command_type_e, name: *const libc::c_char)
     -> *mut libc::c_void;
    #[no_mangle]
    fn BaseCmd_Insert(type_0: base_command_type_e, basecmd: *mut libc::c_void,
                      name: *const libc::c_char);
    #[no_mangle]
    fn BaseCmd_Remove(type_0: base_command_type_e, name: *const libc::c_char);
}
pub type __uint32_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type HANDLE = *mut libc::c_void;
pub type HINSTANCE = *mut libc::c_void;
pub type uint = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type uint32_t = __uint32_t;
pub type uintptr_t = libc::c_ulong;
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
pub type byte = libc::c_uchar;
pub type vec_t = libc::c_float;
pub type vec3_t = [vec_t; 3];
pub type poolhandle_t = uint32_t;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type dword = libc::c_uint;
pub type string = [libc::c_char; 256];
pub type file_t = file_s;
pub type setpair_t
    =
    Option<unsafe extern "C" fn(_: *const libc::c_char,
                                _: *const libc::c_void, _: *mut libc::c_void,
                                _: *mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cvar_s {
    pub name: *mut libc::c_char,
    pub string: *mut libc::c_char,
    pub flags: libc::c_int,
    pub value: libc::c_float,
    pub next: *mut cvar_s,
}
pub type cvar_t = cvar_s;
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
pub type convar_t = convar_s;
pub type base_command_t = ();
pub type base_command_type_e = base_command_type;
pub type base_command_type = libc::c_uint;
pub const HM_CMDALIAS: base_command_type = 3;
pub const HM_CMD: base_command_type = 2;
pub const HM_CVAR: base_command_type = 1;
pub const HM_DONTCARE: base_command_type = 0;
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
pub type xcommand_t = Option<unsafe extern "C" fn() -> ()>;
/*
cvar.c - dynamic variable tracking
Copyright (C) 2007 Uncle Mike

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
*/
// fabs...
// ARRAYSIZE
#[no_mangle]
pub static mut cvar_vars: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
// head of list
#[no_mangle]
pub static mut cmd_scripting: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut cl_filterstuffcmd: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"cl_filterstuffcmd\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"1\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags:
                             (1 as libc::c_int) << 0 as libc::c_int |
                                 (1 as libc::c_int) << 10 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"filter commands coming from server\x00" as
                                 *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
/*
============
Cvar_GetList
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cvar_GetList() -> *mut cvar_t {
    return cvar_vars as *mut cvar_t;
}
/*
============
Cvar_FindVar

find the specified variable by name
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cvar_FindVarExt(mut var_name: *const libc::c_char,
                                         mut ignore_group: libc::c_int)
 -> *mut convar_t {
    // TODO: ignore group for cvar
    return BaseCmd_Find(HM_CVAR, var_name) as *mut convar_t;
}
/*
============
Cvar_BuildAutoDescription

build cvar auto description that based on the setup flags
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cvar_BuildAutoDescription(mut flags: libc::c_int)
 -> *const libc::c_char {
    static mut desc: [libc::c_char; 256] = [0; 256];
    desc[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    if flags & (1 as libc::c_int) << 3 as libc::c_int != 0 {
        Q_strncpy(desc.as_mut_ptr(),
                  b"game \x00" as *const u8 as *const libc::c_char,
                  ::std::mem::size_of::<[libc::c_char; 256]>() as
                      libc::c_ulong);
    } else if flags & (1 as libc::c_int) << 4 as libc::c_int != 0 {
        Q_strncpy(desc.as_mut_ptr(),
                  b"client \x00" as *const u8 as *const libc::c_char,
                  ::std::mem::size_of::<[libc::c_char; 256]>() as
                      libc::c_ulong);
    } else if flags & (1 as libc::c_int) << 14 as libc::c_int != 0 {
        Q_strncpy(desc.as_mut_ptr(),
                  b"GameUI \x00" as *const u8 as *const libc::c_char,
                  ::std::mem::size_of::<[libc::c_char; 256]>() as
                      libc::c_ulong);
    }
    if flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        Q_strncat(desc.as_mut_ptr(),
                  b"server \x00" as *const u8 as *const libc::c_char,
                  ::std::mem::size_of::<[libc::c_char; 256]>() as
                      libc::c_ulong);
    }
    if flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        Q_strncat(desc.as_mut_ptr(),
                  b"user \x00" as *const u8 as *const libc::c_char,
                  ::std::mem::size_of::<[libc::c_char; 256]>() as
                      libc::c_ulong);
    }
    if flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        Q_strncat(desc.as_mut_ptr(),
                  b"archived \x00" as *const u8 as *const libc::c_char,
                  ::std::mem::size_of::<[libc::c_char; 256]>() as
                      libc::c_ulong);
    }
    if flags & (1 as libc::c_int) << 5 as libc::c_int != 0 {
        Q_strncat(desc.as_mut_ptr(),
                  b"protected \x00" as *const u8 as *const libc::c_char,
                  ::std::mem::size_of::<[libc::c_char; 256]>() as
                      libc::c_ulong);
    }
    if flags & (1 as libc::c_int) << 10 as libc::c_int != 0 {
        Q_strncat(desc.as_mut_ptr(),
                  b"privileged \x00" as *const u8 as *const libc::c_char,
                  ::std::mem::size_of::<[libc::c_char; 256]>() as
                      libc::c_ulong);
    }
    Q_strncat(desc.as_mut_ptr(),
              b"cvar\x00" as *const u8 as *const libc::c_char,
              ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong);
    return desc.as_mut_ptr();
}
/*
============
Cvar_UpdateInfo

deal with userinfo etc
============
*/
unsafe extern "C" fn Cvar_UpdateInfo(mut var: *mut convar_t,
                                     mut value: *const libc::c_char,
                                     mut notify: qboolean) -> qboolean {
    if (*var).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        if host.type_0 == HOST_DEDICATED as libc::c_int as libc::c_uint {
            // g-cont. this is a very strange behavior...
            Info_SetValueForKey(SV_Serverinfo(), (*var).name, value,
                                512 as libc::c_int); // failed to change value
            SV_BroadcastCommand(b"fullserverinfo \"%s\"\n\x00" as *const u8 as
                                    *const libc::c_char, SV_Serverinfo());
        } else {
            if Info_SetValueForKey(CL_Userinfo(), (*var).name, value,
                                   256 as libc::c_int) as u64 == 0 {
                return false_0
            }
            // time to update server copy of userinfo
            CL_ServerCommand(true_0,
                             b"setinfo \"%s\" \"%s\"\n\x00" as *const u8 as
                                 *const libc::c_char, (*var).name, value);
            CL_LegacyUpdateInfo();
        }
    }
    if (*var).flags & (1 as libc::c_int) << 2 as libc::c_int != 0 &&
           notify as libc::c_uint != 0 {
        if (*var).flags & (1 as libc::c_int) << 8 as libc::c_int == 0 {
            if (*var).flags & (1 as libc::c_int) << 5 as libc::c_int != 0 {
                Log_Printf(b"Server cvar \"%s\" = \"%s\"\n\x00" as *const u8
                               as *const libc::c_char, (*var).name,
                           b"***PROTECTED***\x00" as *const u8 as
                               *const libc::c_char);
                SV_BroadcastPrintf(0 as *mut sv_client_s,
                                   b"\"%s\" changed to \"%s\"\n\x00" as
                                       *const u8 as *const libc::c_char,
                                   (*var).name,
                                   b"***PROTECTED***\x00" as *const u8 as
                                       *const libc::c_char);
            } else {
                Log_Printf(b"Server cvar \"%s\" = \"%s\"\n\x00" as *const u8
                               as *const libc::c_char, (*var).name, value);
                SV_BroadcastPrintf(0 as *mut sv_client_s,
                                   b"\"%s\" changed to \"%s\"\n\x00" as
                                       *const u8 as *const libc::c_char,
                                   (*var).name, value);
            }
        }
    }
    return true_0;
}
/*
============
Cvar_ValidateString

deal with userinfo etc
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cvar_ValidateString(mut var: *mut convar_t,
                                             mut value: *const libc::c_char)
 -> *const libc::c_char {
    let mut pszValue: *const libc::c_char = 0 as *const libc::c_char;
    static mut szNew: [libc::c_char; 256] = [0; 256];
    pszValue = value;
    szNew[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    // this cvar's string must only contain printable characters.
	// strip out any other crap. we'll fill in "empty" if nothing is left
    if (*var).flags & (1 as libc::c_int) << 7 as libc::c_int != 0 {
        let mut szVal: *mut libc::c_char = szNew.as_mut_ptr();
        let mut len: libc::c_int = 0 as libc::c_int;
        // step through the string, only copying back in characters that are printable
        while *pszValue as libc::c_int != 0 && len < 256 as libc::c_int {
            if (*pszValue as byte as libc::c_int) < 32 as libc::c_int {
                pszValue = pszValue.offset(1)
            } else {
                let fresh0 = pszValue;
                pszValue = pszValue.offset(1);
                let fresh1 = szVal;
                szVal = szVal.offset(1);
                *fresh1 = *fresh0;
                len += 1
            }
        }
        *szVal = '\u{0}' as i32 as libc::c_char;
        pszValue = szNew.as_mut_ptr();
        // g-cont. is this even need?
        if if *szNew.as_mut_ptr() == 0 {
               0 as libc::c_int
           } else { 1 as libc::c_int } == 0 {
            Q_strncpy(szNew.as_mut_ptr(),
                      b"empty\x00" as *const u8 as *const libc::c_char,
                      ::std::mem::size_of::<[libc::c_char; 256]>() as
                          libc::c_ulong);
        }
    }
    if (*var).flags & (1 as libc::c_int) << 9 as libc::c_int != 0 {
        let mut szVal_0: *mut libc::c_char = szNew.as_mut_ptr();
        let mut len_0: libc::c_int = 0 as libc::c_int;
        // step through the string, only copying back in characters that are printable
        while *pszValue as libc::c_int != 0 && len_0 < 256 as libc::c_int {
            if *pszValue as libc::c_int == ' ' as i32 {
                pszValue = pszValue.offset(1)
            } else {
                let fresh2 = pszValue;
                pszValue = pszValue.offset(1);
                let fresh3 = szVal_0;
                szVal_0 = szVal_0.offset(1);
                *fresh3 = *fresh2;
                len_0 += 1
            }
        }
        *szVal_0 = '\u{0}' as i32 as libc::c_char;
        pszValue = szNew.as_mut_ptr()
    }
    return pszValue;
}
/*
============
Cvar_UnlinkVar

unlink the variable
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cvar_UnlinkVar(mut var_name: *const libc::c_char,
                                        mut group: libc::c_int)
 -> libc::c_int {
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut prev: *mut *mut convar_t = 0 as *mut *mut convar_t;
    let mut var: *mut convar_t = 0 as *mut convar_t;
    prev = &mut cvar_vars;
    loop  {
        var = *prev;
        if var.is_null() { break ; }
        // do filter by name
        if !var_name.is_null() &&
               Q_strncmp((*var).name, var_name, 99999 as libc::c_int) != 0 {
            prev = &mut (*var).next
        } else if group != 0 && (*var).flags & group == 0 {
            prev = &mut (*var).next
        } else {
            BaseCmd_Remove(HM_CVAR, (*var).name);
            // do filter by specified group
            // unlink variable from list
            if !(*var).string.is_null() {
                _Mem_Free((*var).string as *mut libc::c_void,
                          b"../engine/common/cvar.c\x00" as *const u8 as
                              *const libc::c_char, 262 as libc::c_int);
                (*var).string = 0 as *mut libc::c_char
            }
            *prev = (*var).next;
            // only allocated cvars can throw these fields
            if (*var).flags & (1 as libc::c_int) << 19 as libc::c_int != 0 {
                if !(*var).name.is_null() {
                    _Mem_Free((*var).name as *mut libc::c_void,
                              b"../engine/common/cvar.c\x00" as *const u8 as
                                  *const libc::c_char, 268 as libc::c_int);
                    (*var).name = 0 as *mut libc::c_char
                }
                if !(*var).def_string.is_null() {
                    _Mem_Free((*var).def_string as *mut libc::c_void,
                              b"../engine/common/cvar.c\x00" as *const u8 as
                                  *const libc::c_char, 269 as libc::c_int);
                    (*var).def_string = 0 as *mut libc::c_char
                }
                if !(*var).desc.is_null() {
                    _Mem_Free((*var).desc as *mut libc::c_void,
                              b"../engine/common/cvar.c\x00" as *const u8 as
                                  *const libc::c_char, 270 as libc::c_int);
                    (*var).desc = 0 as *mut libc::c_char
                }
                _Mem_Free(var as *mut libc::c_void,
                          b"../engine/common/cvar.c\x00" as *const u8 as
                              *const libc::c_char, 271 as libc::c_int);
            }
            count += 1
        }
    }
    return count;
}
/*
============
Cvar_Changed

Tell the engine parts about cvar changing
============
*/
unsafe extern "C" fn Cvar_Changed(mut var: *mut convar_t) {
    // tell about changes
    (*var).flags = (*var).flags | (1 as libc::c_int) << 13 as libc::c_int;
    // tell the engine parts with global state
    if (*var).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        host.userinfo_changed = true_0
    }
    if (*var).flags & (1 as libc::c_int) << 22 as libc::c_int != 0 {
        host.movevars_changed = true_0
    }
    if (*var).flags & (1 as libc::c_int) << 20 as libc::c_int != 0 {
        host.renderinfo_changed = true_0
    }
    if Q_strncmp((*var).name,
                 b"sv_cheats\x00" as *const u8 as *const libc::c_char,
                 99999 as libc::c_int) == 0 {
        host.allow_cheats = Q_atoi((*var).string) as qboolean
    };
}
/*
============
Cvar_LookupVars
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cvar_LookupVars(mut checkbit: libc::c_int,
                                         mut buffer: *mut libc::c_void,
                                         mut ptr: *mut libc::c_void,
                                         mut callback: setpair_t) {
    let mut var: *mut convar_t = 0 as *mut convar_t;
    // nothing to process ?
    if callback.is_none() { return }
    // force checkbit to 0 for lookup all cvars
    var = cvar_vars;
    while !var.is_null() {
        if !(checkbit != 0 && (*var).flags & checkbit == 0) {
            if !buffer.is_null() {
                callback.expect("non-null function pointer")((*var).name,
                                                             (*var).string as
                                                                 *const libc::c_void,
                                                             buffer, ptr);
            } else if (*var).flags &
                          ((1 as libc::c_int) << 19 as libc::c_int |
                               (1 as libc::c_int) << 18 as libc::c_int) != 0 {
                callback.expect("non-null function pointer")((*var).name,
                                                             (*var).string as
                                                                 *const libc::c_void,
                                                             (*var).desc as
                                                                 *mut libc::c_void,
                                                             ptr);
            } else {
                callback.expect("non-null function pointer")((*var).name,
                                                             (*var).string as
                                                                 *const libc::c_void,
                                                             b"\x00" as
                                                                 *const u8 as
                                                                 *const libc::c_char
                                                                 as
                                                                 *mut libc::c_void,
                                                             ptr);
            }
        }
        var = (*var).next
    };
}
// NOTE: dlls cvars doesn't have description
/*
============
Cvar_Get

If the variable already exists, the value will not be set
The flags will be or'ed in if the variable exists.
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cvar_Get(mut name: *const libc::c_char,
                                  mut value: *const libc::c_char,
                                  mut flags: libc::c_int,
                                  mut var_desc: *const libc::c_char)
 -> *mut convar_t {
    let mut cur: *mut convar_t = 0 as *mut convar_t;
    let mut find: *mut convar_t = 0 as *mut convar_t;
    let mut var: *mut convar_t = 0 as *mut convar_t;
    if !(!name.is_null() && *name as libc::c_int != 0) {
        Sys_Error(b"assert failed at %s:%i\n\x00" as *const u8 as
                      *const libc::c_char,
                  b"../engine/common/cvar.c\x00" as *const u8 as
                      *const libc::c_char, 351 as libc::c_int);
    }
    // check for command coexisting
    if Cmd_Exists(name) as u64 != 0 {
        Con_DPrintf(b"^1Error:^7 can\'t register variable \'%s\', is already defined as command\n\x00"
                        as *const u8 as *const libc::c_char, name);
        return 0 as *mut convar_t
    }
    var = Cvar_FindVarExt(name, 0 as libc::c_int);
    if !var.is_null() {
        // already existed?
        if flags & (1 as libc::c_int) << 12 as libc::c_int != 0 {
            // NOTE: cvars without description produced by Cvar_FullSet
			// which executed from the config file. So we don't need to
			// change value here: we *already* have actual value from config.
			// in other cases we need to rewrite them
            if Q_strncmp((*var).desc,
                         b"\x00" as *const u8 as *const libc::c_char,
                         99999 as libc::c_int) != 0 {
                // directly set value
                if !(*var).string.is_null() {
                    _Mem_Free((*var).string as *mut libc::c_void,
                              b"../engine/common/cvar.c\x00" as *const u8 as
                                  *const libc::c_char, 374 as libc::c_int);
                    (*var).string = 0 as *mut libc::c_char
                }
                (*var).string =
                    _copystring(host.mempool, value,
                                b"../engine/common/cvar.c\x00" as *const u8 as
                                    *const libc::c_char, 375 as libc::c_int);
                (*var).value = Q_atof((*var).string);
                (*var).flags = (*var).flags | flags;
                // tell engine about changes
                Cvar_Changed(var);
            }
        } else {
            (*var).flags = (*var).flags | flags;
            Cvar_DirectSet(var, value);
        }
        if (*var).flags & (1 as libc::c_int) << 19 as libc::c_int != 0 &&
               Q_strncmp(var_desc, (*var).desc, 99999 as libc::c_int) != 0 {
            if flags & (1 as libc::c_int) << 12 as libc::c_int == 0 {
                Con_Reportf(b"%s change description from %s to %s\n\x00" as
                                *const u8 as *const libc::c_char, (*var).name,
                            (*var).desc, var_desc);
            }
            // update description if needs
            if !(*var).desc.is_null() {
                _Mem_Free((*var).desc as *mut libc::c_void,
                          b"../engine/common/cvar.c\x00" as *const u8 as
                              *const libc::c_char, 394 as libc::c_int);
                (*var).desc = 0 as *mut libc::c_char
            }
            (*var).desc =
                _copystring(host.mempool, var_desc,
                            b"../engine/common/cvar.c\x00" as *const u8 as
                                *const libc::c_char, 395 as libc::c_int)
        }
        return var
    }
    // allocate a new cvar
    var =
        _Mem_Alloc(host.mempool,
                   ::std::mem::size_of::<convar_t>() as libc::c_ulong,
                   false_0,
                   b"../engine/common/cvar.c\x00" as *const u8 as
                       *const libc::c_char, 402 as libc::c_int) as
            *mut convar_t;
    (*var).name =
        _copystring(host.mempool, name,
                    b"../engine/common/cvar.c\x00" as *const u8 as
                        *const libc::c_char, 403 as libc::c_int);
    (*var).string =
        _copystring(host.mempool, value,
                    b"../engine/common/cvar.c\x00" as *const u8 as
                        *const libc::c_char, 404 as libc::c_int);
    (*var).def_string =
        _copystring(host.mempool, value,
                    b"../engine/common/cvar.c\x00" as *const u8 as
                        *const libc::c_char, 405 as libc::c_int);
    (*var).desc =
        _copystring(host.mempool, var_desc,
                    b"../engine/common/cvar.c\x00" as *const u8 as
                        *const libc::c_char, 406 as libc::c_int);
    (*var).value = Q_atof((*var).string);
    (*var).flags = flags | (1 as libc::c_int) << 19 as libc::c_int;
    // link the variable in alphanumerical order
    cur = 0 as *mut convar_t;
    find = cvar_vars;
    while !find.is_null() &&
              Q_strncmp((*find).name, (*var).name, 99999 as libc::c_int) <
                  0 as libc::c_int {
        cur = find;
        find = (*find).next
    }
    if !cur.is_null() { (*cur).next = var } else { cvar_vars = var }
    (*var).next = find;
    // fill it cls.userinfo, svs.serverinfo
    Cvar_UpdateInfo(var, (*var).string, false_0);
    // tell engine about changes
    Cvar_Changed(var);
    // add to map
    BaseCmd_Insert(HM_CVAR, var as *mut libc::c_void, (*var).name);
    return var;
}
/*
============
Cvar_RegisterVariable

Adds a freestanding variable to the variable list.
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cvar_RegisterVariable(mut var: *mut convar_t) {
    let mut cur: *mut convar_t = 0 as *mut convar_t;
    let mut find: *mut convar_t = 0 as *mut convar_t;
    let mut dup: *mut convar_t = 0 as *mut convar_t;
    if var.is_null() {
        Sys_Error(b"assert failed at %s:%i\n\x00" as *const u8 as
                      *const libc::c_char,
                  b"../engine/common/cvar.c\x00" as *const u8 as
                      *const libc::c_char, 442 as libc::c_int);
    }
    // first check to see if it has allready been defined
    dup = Cvar_FindVarExt((*var).name, 0 as libc::c_int);
    if !dup.is_null() {
        if (*dup).flags & (1 as libc::c_int) << 21 as libc::c_int == 0 {
            Con_DPrintf(b"^1Error:^7 can\'t register variable \'%s\', is already defined\n\x00"
                            as *const u8 as *const libc::c_char, (*var).name);
            return
        }
        // time to replace temp variable with real
        Cvar_UnlinkVar((*var).name, (1 as libc::c_int) << 21 as libc::c_int);
    }
    // check for overlap with a command
    if Cmd_Exists((*var).name) as u64 != 0 {
        Con_DPrintf(b"^1Error:^7 can\'t register variable \'%s\', is already defined as command\n\x00"
                        as *const u8 as *const libc::c_char, (*var).name);
        return
    }
    // NOTE: all the 'long' engine cvars have an special setntinel on static declaration
	// (all the engine cvars should be declared through CVAR_DEFINE macros or they shouldn't working properly anyway)
	// so we can determine long version 'convar_t' and short version 'cvar_t' more reliable than by FCVAR_EXTDLL flag
    if (*var).next as uintptr_t == 0xdeadbeefdeadbeef as libc::c_ulong {
        (*var).flags = (*var).flags | (1 as libc::c_int) << 18 as libc::c_int
    }
    // copy the value off, because future sets will free it
    if (*var).flags & (1 as libc::c_int) << 18 as libc::c_int != 0 {
        (*var).def_string = (*var).string
    } // just swap pointers
    (*var).string =
        _copystring(host.mempool, (*var).string,
                    b"../engine/common/cvar.c\x00" as *const u8 as
                        *const libc::c_char, 475 as libc::c_int);
    (*var).value = Q_atof((*var).string);
    // find the supposed position in chain (alphanumerical order)
    cur = 0 as *mut convar_t;
    find = cvar_vars;
    while !find.is_null() &&
              Q_strncmp((*find).name, (*var).name, 99999 as libc::c_int) <
                  0 as libc::c_int {
        cur = find;
        find = (*find).next
    }
    // now link variable
    if !cur.is_null() { (*cur).next = var } else { cvar_vars = var }
    (*var).next = find;
    // fill it cls.userinfo, svs.serverinfo
    Cvar_UpdateInfo(var, (*var).string, false_0);
    // tell engine about changes
    Cvar_Changed(var);
    // add to map
    BaseCmd_Insert(HM_CVAR, var as *mut libc::c_void, (*var).name);
}
/*
============
Cvar_DirectSet

way to change value for many cvars
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cvar_DirectSet(mut var: *mut convar_t,
                                        mut value: *const libc::c_char) {
    let mut pszValue: *const libc::c_char = 0 as *const libc::c_char; // ???
    if var.is_null() { return }
    // lookup for registration
    if (*var).next as uintptr_t == 0xdeadbeefdeadbeef as libc::c_ulong ||
           (*var).next.is_null() &&
               (*var).flags &
                   ((1 as libc::c_int) << 18 as libc::c_int |
                        (1 as libc::c_int) << 19 as libc::c_int) == 0 {
        // need to registering cvar fisrt
        Cvar_RegisterVariable(var);
        // ok, register it
    }
    // lookup for registration again
    if var != Cvar_FindVarExt((*var).name, 0 as libc::c_int) {
        return
    } // how this possible?
    if (*var).flags &
           ((1 as libc::c_int) << 17 as libc::c_int |
                (1 as libc::c_int) << 12 as libc::c_int) != 0 {
        Con_Printf(b"%s is read-only.\n\x00" as *const u8 as
                       *const libc::c_char, (*var).name);
        return
    }
    if (*var).flags & (1 as libc::c_int) << 15 as libc::c_int != 0 &&
           host.allow_cheats as u64 == 0 {
        Con_Printf(b"%s is cheat protected.\n\x00" as *const u8 as
                       *const libc::c_char, (*var).name);
        return
    }
    // just tell user about deferred changes
    if (*var).flags & (1 as libc::c_int) << 30 as libc::c_int != 0 &&
           (SV_Active() as libc::c_uint != 0 || CL_Active() != 0) {
        Con_Printf(b"%s will be changed upon restarting.\n\x00" as *const u8
                       as *const libc::c_char, (*var).name);
    }
    // check value
    if value.is_null() {
        if (*var).flags &
               ((1 as libc::c_int) << 18 as libc::c_int |
                    (1 as libc::c_int) << 19 as libc::c_int) == 0 {
            Con_Printf(b"%s has no default value and can\'t be reset.\n\x00"
                           as *const u8 as *const libc::c_char, (*var).name);
            return
        }
        value = (*var).def_string
        // reset to default value
    }
    pszValue = Cvar_ValidateString(var, value);
    // nothing to change
    if Q_strncmp(pszValue, (*var).string, 99999 as libc::c_int) == 0 {
        return
    }
    // fill it cls.userinfo, svs.serverinfo
    if Cvar_UpdateInfo(var, pszValue, true_0) as u64 == 0 { return }
    // and finally changed the cvar itself
    if !(*var).string.is_null() {
        _Mem_Free((*var).string as *mut libc::c_void,
                  b"../engine/common/cvar.c\x00" as *const u8 as
                      *const libc::c_char, 561 as libc::c_int);
        (*var).string = 0 as *mut libc::c_char
    }
    (*var).string =
        _copystring(host.mempool, pszValue,
                    b"../engine/common/cvar.c\x00" as *const u8 as
                        *const libc::c_char, 562 as libc::c_int);
    (*var).value = Q_atof((*var).string);
    // tell engine about changes
    Cvar_Changed(var);
}
/*
============
Cvar_FullSet

can set any protected cvars
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cvar_FullSet(mut var_name: *const libc::c_char,
                                      mut value: *const libc::c_char,
                                      mut flags: libc::c_int) {
    let mut var: *mut convar_t = Cvar_FindVarExt(var_name, 0 as libc::c_int);
    if var.is_null() {
        Cvar_Get(var_name, value, flags,
                 b"\x00" as *const u8 as *const libc::c_char);
        return
    }
    if !(*var).string.is_null() {
        _Mem_Free((*var).string as *mut libc::c_void,
                  b"../engine/common/cvar.c\x00" as *const u8 as
                      *const libc::c_char, 586 as libc::c_int);
        (*var).string = 0 as *mut libc::c_char
    }
    (*var).string =
        _copystring(host.mempool, value,
                    b"../engine/common/cvar.c\x00" as *const u8 as
                        *const libc::c_char, 587 as libc::c_int);
    (*var).value = Q_atof((*var).string);
    (*var).flags = (*var).flags | flags;
    // tell engine about changes
    Cvar_Changed(var);
}
/*
============
Cvar_Set
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cvar_Set(mut var_name: *const libc::c_char,
                                  mut value: *const libc::c_char) {
    let mut var: *mut convar_t = 0 as *mut convar_t;
    if var_name.is_null() {
        // there is an error in C code if this happens
        Con_Printf(b"Cvar_Set: passed NULL variable name\n\x00" as *const u8
                       as *const libc::c_char);
        return
    }
    var = Cvar_FindVarExt(var_name, 0 as libc::c_int);
    if var.is_null() {
        // there is an error in C code if this happens
        Con_Printf(b"Cvar_Set: variable \'%s\' not found\n\x00" as *const u8
                       as *const libc::c_char, var_name);
        return
    }
    Cvar_DirectSet(var, value);
}
/*
============
Cvar_SetValue
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cvar_SetValue(mut var_name: *const libc::c_char,
                                       mut value: libc::c_float) {
    let mut val: [libc::c_char; 32] = [0; 32];
    if fabs((value - value as libc::c_int as libc::c_float) as libc::c_double)
           < 0.000001f64 {
        Q_snprintf(val.as_mut_ptr(),
                   ::std::mem::size_of::<[libc::c_char; 32]>() as
                       libc::c_ulong,
                   b"%d\x00" as *const u8 as *const libc::c_char,
                   value as libc::c_int);
    } else {
        Q_snprintf(val.as_mut_ptr(),
                   ::std::mem::size_of::<[libc::c_char; 32]>() as
                       libc::c_ulong,
                   b"%f\x00" as *const u8 as *const libc::c_char,
                   value as libc::c_double);
    }
    Cvar_Set(var_name, val.as_mut_ptr());
}
/*
============
Cvar_Reset
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cvar_Reset(mut var_name: *const libc::c_char) {
    Cvar_Set(var_name, 0 as *const libc::c_char);
}
/*
============
Cvar_VariableValue
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cvar_VariableValue(mut var_name: *const libc::c_char)
 -> libc::c_float {
    let mut var: *mut convar_t = 0 as *mut convar_t;
    if var_name.is_null() {
        // there is an error in C code if this happens
        Con_Printf(b"Cvar_VariableValue: passed NULL variable name\n\x00" as
                       *const u8 as *const libc::c_char);
        return 0.0f32
    }
    var = Cvar_FindVarExt(var_name, 0 as libc::c_int);
    if var.is_null() { return 0.0f32 }
    return Q_atof((*var).string);
}
/*
============
Cvar_VariableInteger
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cvar_VariableInteger(mut var_name:
                                                  *const libc::c_char)
 -> libc::c_int {
    let mut var: *mut convar_t = 0 as *mut convar_t;
    var = Cvar_FindVarExt(var_name, 0 as libc::c_int);
    if var.is_null() { return 0 as libc::c_int }
    return Q_atoi((*var).string);
}
/*
============
Cvar_VariableString
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cvar_VariableString(mut var_name:
                                                 *const libc::c_char)
 -> *const libc::c_char {
    let mut var: *mut convar_t = 0 as *mut convar_t;
    if var_name.is_null() {
        // there is an error in C code if this happens
        Con_Printf(b"Cvar_VariableString: passed NULL variable name\n\x00" as
                       *const u8 as *const libc::c_char);
        return b"\x00" as *const u8 as *const libc::c_char
    }
    var = Cvar_FindVarExt(var_name, 0 as libc::c_int);
    if var.is_null() { return b"\x00" as *const u8 as *const libc::c_char }
    return (*var).string;
}
/*
============
Cvar_Exists
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cvar_Exists(mut var_name: *const libc::c_char)
 -> qboolean {
    if !Cvar_FindVarExt(var_name, 0 as libc::c_int).is_null() {
        return true_0
    }
    return false_0;
}
/*
============
Cvar_SetCheatState

Any testing variables will be reset to the safe values
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cvar_SetCheatState() {
    let mut var: *mut convar_t = 0 as *mut convar_t;
    // set all default vars to the safe value
    var = cvar_vars;
    while !var.is_null() {
        // can't process dll cvars - missed def_string
        if !((*var).flags &
                 ((1 as libc::c_int) << 19 as libc::c_int |
                      (1 as libc::c_int) << 18 as libc::c_int) == 0) {
            if (*var).flags & (1 as libc::c_int) << 15 as libc::c_int != 0 {
                if Q_strncmp((*var).def_string, (*var).string,
                             99999 as libc::c_int) != 0 {
                    Cvar_DirectSet(var, (*var).def_string);
                }
            }
        }
        var = (*var).next
    };
}
/*
============
Cvar_SetGL

As Cvar_Set, but also flags it as glconfig
============
*/
unsafe extern "C" fn Cvar_SetGL(mut name: *const libc::c_char,
                                mut value: *const libc::c_char) {
    let mut var: *mut convar_t = Cvar_FindVarExt(name, 0 as libc::c_int);
    if !var.is_null() &&
           (*var).flags & (1 as libc::c_int) << 12 as libc::c_int == 0 {
        Con_Reportf(b"^1Error:^7 Can\'t set non-GL cvar %s to %s\n\x00" as
                        *const u8 as *const libc::c_char, name, value);
        return
    }
    Cvar_FullSet(name, value, (1 as libc::c_int) << 12 as libc::c_int);
}
unsafe extern "C" fn Cvar_ShouldSetCvar(mut v: *mut convar_t,
                                        mut isPrivileged: qboolean)
 -> qboolean {
    let mut prefixes: [*const libc::c_char; 5] =
        [b"cl_\x00" as *const u8 as *const libc::c_char,
         b"gl_\x00" as *const u8 as *const libc::c_char,
         b"m_\x00" as *const u8 as *const libc::c_char,
         b"r_\x00" as *const u8 as *const libc::c_char,
         b"hud_\x00" as *const u8 as *const libc::c_char];
    let mut i: libc::c_int = 0;
    if isPrivileged as u64 != 0 { return true_0 }
    if (*v).flags & (1 as libc::c_int) << 10 as libc::c_int != 0 {
        return false_0
    }
    if cl_filterstuffcmd.value <= 0.0f32 { return true_0 }
    if (*v).flags & (1 as libc::c_int) << 11 as libc::c_int != 0 {
        return false_0
    }
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[*const libc::c_char; 5]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<*const libc::c_char>()
                                                   as libc::c_ulong) {
        if Q_strnicmp((*v).name, prefixes[i as usize],
                      Q_strlen(prefixes[i as usize]) as libc::c_int) == 0 {
            return false_0
        }
        i += 1
    }
    return true_0;
}
/*
============
Cvar_Command

Handles variable inspection and changing from the console
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cvar_CommandWithPrivilegeCheck(mut v: *mut convar_t,
                                                        mut isPrivileged:
                                                            qboolean)
 -> qboolean {
    // special case for setup opengl configuration
    if host.apply_opengl_config as u64 != 0 {
        Cvar_SetGL(Cmd_Argv(0 as libc::c_int), Cmd_Argv(1 as libc::c_int));
        return true_0
    }
    // check variables
    if v.is_null() {
        // already found in basecmd
        v = Cvar_FindVarExt(Cmd_Argv(0 as libc::c_int), 0 as libc::c_int)
    }
    if v.is_null() { return false_0 }
    // perform a variable print or set
    if Cmd_Argc() == 1 as libc::c_int {
        if (*v).flags &
               ((1 as libc::c_int) << 19 as libc::c_int |
                    (1 as libc::c_int) << 18 as libc::c_int) != 0 {
            Con_Printf(b"\"%s\" is \"%s\" ( ^3\"%s\"^7 )\n\x00" as *const u8
                           as *const libc::c_char, (*v).name, (*v).string,
                       (*v).def_string);
        } else {
            Con_Printf(b"\"%s\" is \"%s\"\n\x00" as *const u8 as
                           *const libc::c_char, (*v).name, (*v).string);
        }
        return true_0
    }
    if host.apply_game_config as u64 != 0 {
        if (*v).flags & (1 as libc::c_int) << 3 as libc::c_int == 0 {
            return true_0
        }
        // only game.dll cvars passed
    }
    if (*v).flags & (1 as libc::c_int) << 6 as libc::c_int != 0 &&
           CL_GetMaxClients() > 1 as libc::c_int {
        Con_Printf(b"can\'t set \"%s\" in multiplayer\n\x00" as *const u8 as
                       *const libc::c_char, (*v).name);
        return false_0
    } else if Cvar_ShouldSetCvar(v, isPrivileged) as u64 == 0 {
        Con_Printf(b"%s is a privileged variable\n\x00" as *const u8 as
                       *const libc::c_char, (*v).name);
        return true_0
    } else {
        Cvar_DirectSet(v, Cmd_Argv(1 as libc::c_int));
        if host.apply_game_config as u64 != 0 { host.sv_cvars_restored += 1 }
        return true_0
    };
}
/*
============
Cvar_WriteVariables

Writes lines containing "variable value" for all variables
with the specified flag set to true.
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cvar_WriteVariables(mut f: *mut file_t,
                                             mut group: libc::c_int) {
    let mut var: *mut convar_t = 0 as *mut convar_t;
    var = cvar_vars;
    while !var.is_null() {
        if (*var).flags & group != 0 {
            FS_Printf(f,
                      b"%s \"%s\"\n\x00" as *const u8 as *const libc::c_char,
                      (*var).name, (*var).string);
        }
        var = (*var).next
    };
}
/*
============
Cvar_Toggle_f

Toggles a cvar for easy single key binding
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cvar_Toggle_f() {
    let mut v: libc::c_int = 0;
    if Cmd_Argc() != 2 as libc::c_int {
        Con_Printf(b"Usage: toggle <variable>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    v =
        (Cvar_VariableInteger(Cmd_Argv(1 as libc::c_int)) == 0) as
            libc::c_int;
    Cvar_Set(Cmd_Argv(1 as libc::c_int),
             va(b"%i\x00" as *const u8 as *const libc::c_char, v));
}
/*
============
Cvar_SetGL_f

As Cvar_Set, but also flags it as glconfig
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cvar_SetGL_f() {
    let mut var: *mut convar_t = 0 as *mut convar_t;
    if Cmd_Argc() != 3 as libc::c_int {
        Con_Printf(b"Usage: setgl <variable> <value>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    Cvar_SetGL(Cmd_Argv(1 as libc::c_int), Cmd_Argv(2 as libc::c_int));
}
/*
============
Cvar_Reset_f
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cvar_Reset_f() {
    if Cmd_Argc() != 2 as libc::c_int {
        Con_Printf(b"Usage: reset <variable>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    Cvar_Reset(Cmd_Argv(1 as libc::c_int));
}
/*
============
Cvar_List_f
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cvar_List_f() {
    let mut var: *mut convar_t =
        0 as *mut convar_t; // never shows system cvars
    let mut match_0: *const libc::c_char = 0 as *const libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut count: libc::c_int = 0 as libc::c_int;
    if Cmd_Argc() > 1 as libc::c_int { match_0 = Cmd_Argv(1 as libc::c_int) }
    var = cvar_vars;
    while !var.is_null() {
        if !(*(*var).name.offset(0 as libc::c_int as isize) as libc::c_int ==
                 '@' as i32) {
            if !(!match_0.is_null() &&
                     Q_stricmpext(match_0, (*var).name) as u64 == 0) {
                if Q_colorstr((*var).string) != 0 {
                    value =
                        va(b"\"%s\"\x00" as *const u8 as *const libc::c_char,
                           (*var).string)
                } else {
                    value =
                        va(b"\"^2%s^7\"\x00" as *const u8 as
                               *const libc::c_char, (*var).string)
                }
                if (*var).flags &
                       ((1 as libc::c_int) << 18 as libc::c_int |
                            (1 as libc::c_int) << 19 as libc::c_int) != 0 {
                    Con_Printf(b" %-*s %s ^3%s^7\n\x00" as *const u8 as
                                   *const libc::c_char, 32 as libc::c_int,
                               (*var).name, value, (*var).desc);
                } else {
                    Con_Printf(b" %-*s %s ^3%s^7\n\x00" as *const u8 as
                                   *const libc::c_char, 32 as libc::c_int,
                               (*var).name, value,
                               Cvar_BuildAutoDescription((*var).flags));
                }
                count += 1
            }
        }
        var = (*var).next
    }
    Con_Printf(b"\n%i cvars\n\x00" as *const u8 as *const libc::c_char,
               count);
}
/*
============
Cvar_Unlink

unlink all cvars with specified flag
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cvar_Unlink(mut group: libc::c_int) {
    let mut count: libc::c_int = 0;
    if Cvar_VariableInteger(b"host_gameloaded\x00" as *const u8 as
                                *const libc::c_char) != 0 &&
           group & (1 as libc::c_int) << 3 as libc::c_int != 0 {
        return
    }
    if Cvar_VariableInteger(b"host_clientloaded\x00" as *const u8 as
                                *const libc::c_char) != 0 &&
           group & (1 as libc::c_int) << 4 as libc::c_int != 0 {
        return
    }
    if Cvar_VariableInteger(b"host_gameuiloaded\x00" as *const u8 as
                                *const libc::c_char) != 0 &&
           group & (1 as libc::c_int) << 14 as libc::c_int != 0 {
        return
    }
    count = Cvar_UnlinkVar(0 as *const libc::c_char, group);
    Con_Reportf(b"unlink %i cvars\n\x00" as *const u8 as *const libc::c_char,
                count);
}
/*
============
Cvar_Init

Reads in all archived cvars
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cvar_Init() {
    cvar_vars = 0 as *mut convar_t; // early registering for dev
    cmd_scripting =
        Cvar_Get(b"cmd_scripting\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int |
                     (1 as libc::c_int) << 10 as libc::c_int,
                 b"enable simple condition checking and variable operations\x00"
                     as *const u8 as *const libc::c_char); // OBSOLETE
    Cvar_RegisterVariable(&mut host_developer);
    Cvar_RegisterVariable(&mut cl_filterstuffcmd);
    Cmd_AddRestrictedCommand(b"setgl\x00" as *const u8 as *const libc::c_char,
                             Some(Cvar_SetGL_f as
                                      unsafe extern "C" fn() -> ()),
                             b"change the value of a opengl variable\x00" as
                                 *const u8 as *const libc::c_char);
    Cmd_AddRestrictedCommand(b"toggle\x00" as *const u8 as
                                 *const libc::c_char,
                             Some(Cvar_Toggle_f as
                                      unsafe extern "C" fn() -> ()),
                             b"toggles a console variable\'s values (use for more info)\x00"
                                 as *const u8 as *const libc::c_char);
    Cmd_AddRestrictedCommand(b"reset\x00" as *const u8 as *const libc::c_char,
                             Some(Cvar_Reset_f as
                                      unsafe extern "C" fn() -> ()),
                             b"reset any type variable to initial value\x00"
                                 as *const u8 as *const libc::c_char);
    Cmd_AddCommand(b"cvarlist\x00" as *const u8 as *const libc::c_char,
                   Some(Cvar_List_f as unsafe extern "C" fn() -> ()),
                   b"display all console variables beginning with the specified prefix\x00"
                       as *const u8 as *const libc::c_char);
}
