#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type decallist_s;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn SV_ExecLoadLevel();
    #[no_mangle]
    fn SV_ExecLoadGame();
    #[no_mangle]
    fn SV_ExecChangeLevel();
    #[no_mangle]
    fn SCR_BeginLoadingPlaque(is_background: qboolean);
    #[no_mangle]
    fn SV_ShutdownGame();
    #[no_mangle]
    fn UI_CreditsActive() -> qboolean;
    #[no_mangle]
    fn Host_Frame(time: libc::c_float);
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    fn Sys_Error(error: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Platform_RunEvents();
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
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
host_cmd.c - dedicated and normal host
Copyright (C) 2017 Uncle Mike

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
pub unsafe extern "C" fn COM_InitHostState() {
    memset(&mut host.game as *mut game_status_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<game_status_t>() as libc::c_ulong);
}
unsafe extern "C" fn Host_SetState(mut newState: host_state_t,
                                   mut clearNext: qboolean) {
    if clearNext as u64 != 0 { host.game.nextstate = newState }
    host.game.curstate = newState;
    if clearNext as libc::c_uint != 0 &&
           newState as libc::c_uint ==
               STATE_RUNFRAME as libc::c_int as libc::c_uint {
        // states finished here
        host.game.backgroundMap = false_0;
        host.game.loadGame = false_0;
        host.game.newGame = false_0
    };
}
unsafe extern "C" fn Host_SetNextState(mut nextState: host_state_t) {
    if !(host.game.curstate as libc::c_uint ==
             STATE_RUNFRAME as libc::c_int as libc::c_uint) {
        Sys_Error(b"assert failed at %s:%i\n\x00" as *const u8 as
                      *const libc::c_char,
                  b"../engine/common/host_state.c\x00" as *const u8 as
                      *const libc::c_char, 41 as libc::c_int);
    }
    host.game.nextstate = nextState;
}
#[no_mangle]
pub unsafe extern "C" fn COM_NewGame(mut pMapName: *const libc::c_char) {
    if host.game.nextstate as libc::c_uint !=
           STATE_RUNFRAME as libc::c_int as libc::c_uint {
        return
    }
    if UI_CreditsActive() as u64 != 0 { return }
    Q_strncpy(host.game.levelName.as_mut_ptr(), pMapName,
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    Host_SetNextState(STATE_LOAD_LEVEL);
    host.game.backgroundMap = false_0;
    host.game.landmarkName[0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_char;
    host.game.loadGame = false_0;
    host.game.newGame = true_0;
}
#[no_mangle]
pub unsafe extern "C" fn COM_LoadLevel(mut pMapName: *const libc::c_char,
                                       mut background: qboolean) {
    if host.game.nextstate as libc::c_uint !=
           STATE_RUNFRAME as libc::c_int as libc::c_uint {
        return
    }
    if UI_CreditsActive() as u64 != 0 { return }
    Q_strncpy(host.game.levelName.as_mut_ptr(), pMapName,
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    Host_SetNextState(STATE_LOAD_LEVEL);
    host.game.backgroundMap = background;
    host.game.landmarkName[0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_char;
    host.game.loadGame = false_0;
    host.game.newGame = false_0;
}
#[no_mangle]
pub unsafe extern "C" fn COM_LoadGame(mut pMapName: *const libc::c_char) {
    if host.game.nextstate as libc::c_uint !=
           STATE_RUNFRAME as libc::c_int as libc::c_uint {
        return
    }
    if UI_CreditsActive() as u64 != 0 { return }
    Q_strncpy(host.game.levelName.as_mut_ptr(), pMapName,
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    Host_SetNextState(STATE_LOAD_GAME);
    host.game.backgroundMap = false_0;
    host.game.newGame = false_0;
    host.game.loadGame = true_0;
}
#[no_mangle]
pub unsafe extern "C" fn COM_ChangeLevel(mut pNewLevel: *const libc::c_char,
                                         mut pLandmarkName:
                                             *const libc::c_char,
                                         mut background: qboolean) {
    if host.game.nextstate as libc::c_uint !=
           STATE_RUNFRAME as libc::c_int as libc::c_uint {
        return
    }
    if UI_CreditsActive() as u64 != 0 { return }
    Q_strncpy(host.game.levelName.as_mut_ptr(), pNewLevel,
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    host.game.backgroundMap = background;
    if if pLandmarkName.is_null() || *pLandmarkName == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } != 0 {
        Q_strncpy(host.game.landmarkName.as_mut_ptr(), pLandmarkName,
                  ::std::mem::size_of::<[libc::c_char; 64]>() as
                      libc::c_ulong);
        host.game.loadGame = true_0
    } else {
        host.game.landmarkName[0 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_char;
        host.game.loadGame = false_0
    }
    Host_SetNextState(STATE_CHANGELEVEL);
    host.game.newGame = false_0;
}
#[no_mangle]
pub unsafe extern "C" fn Host_ShutdownGame() {
    SV_ShutdownGame();
    match host.game.nextstate as libc::c_uint {
        2 | 1 => { Host_SetState(host.game.nextstate, true_0); }
        _ => { Host_SetState(STATE_RUNFRAME, true_0); }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Host_RunFrame(mut time: libc::c_float) {
    // at this time, we don't need to get events from OS on dedicated
    Platform_RunEvents();
    // XASH_DEDICATED
    // engine main frame
    Host_Frame(time);
    let mut current_block_6: u64;
    match host.game.nextstate as libc::c_uint {
        0 => { current_block_6 = 13109137661213826276; }
        2 | 1 => {
            SCR_BeginLoadingPlaque(host.game.backgroundMap);
            current_block_6 = 854019367872935040;
        }
        4 => { current_block_6 = 854019367872935040; }
        3 => {
            SCR_BeginLoadingPlaque(host.game.backgroundMap);
            Host_SetState(host.game.nextstate, true_0);
            current_block_6 = 13109137661213826276;
        }
        _ => {
            Host_SetState(STATE_RUNFRAME, true_0);
            current_block_6 = 13109137661213826276;
        }
    }
    match current_block_6 {
        854019367872935040 =>
        // intentionally fallthrough
        {
            Host_SetState(STATE_GAME_SHUTDOWN, false_0);
        }
        _ => { }
    };
}
#[no_mangle]
pub unsafe extern "C" fn COM_Frame(mut time: libc::c_float) {
    let mut loopCount: libc::c_int = 0 as libc::c_int;
    if _setjmp(host.abortframe.as_mut_ptr()) != 0 { return }
    loop  {
        let mut oldState: libc::c_int = host.game.curstate as libc::c_int;
        // execute the current state (and transition to the next state if not in STATE_RUNFRAME)
        match host.game.curstate as libc::c_uint {
            1 => {
                SV_ExecLoadLevel();
                Host_SetState(STATE_RUNFRAME, true_0);
            }
            2 => { SV_ExecLoadGame(); Host_SetState(STATE_RUNFRAME, true_0); }
            3 => {
                SV_ExecChangeLevel();
                Host_SetState(STATE_RUNFRAME, true_0);
            }
            0 => { Host_RunFrame(time); }
            4 => { Host_ShutdownGame(); }
            _ => { }
        }
        if oldState == STATE_RUNFRAME as libc::c_int { break ; }
        if host.game.curstate as libc::c_uint == oldState as libc::c_uint ||
               { loopCount += 1; (loopCount) > 8 as libc::c_int } {
            Sys_Error(b"state infinity loop!\n\x00" as *const u8 as
                          *const libc::c_char);
        }
    };
}
