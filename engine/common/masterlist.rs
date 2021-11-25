#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type file_s;
    pub type decallist_s;
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Q_strncmp(s1: *const libc::c_char, s2: *const libc::c_char,
                 n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn _COM_ParseFileSafe(data: *mut libc::c_char, token: *mut libc::c_char,
                          size: libc::c_int, flags: libc::c_uint,
                          len: *mut libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn NET_StringToAdrNB(string: *const libc::c_char, adr: *mut netadr_t)
     -> libc::c_int;
    #[no_mangle]
    fn NET_SendPacket(sock: netsrc_t, length: size_t,
                      data: *const libc::c_void, to: netadr_t);
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    fn Cmd_Argc() -> libc::c_int;
    #[no_mangle]
    fn Cmd_Argv(arg: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn Cmd_AddCommand(cmd_name: *const libc::c_char, function: xcommand_t,
                      cmd_desc: *const libc::c_char);
    #[no_mangle]
    fn Cmd_AddRestrictedCommand(cmd_name: *const libc::c_char,
                                function: xcommand_t,
                                cmd_desc: *const libc::c_char);
    #[no_mangle]
    fn _Mem_Alloc(poolptr: poolhandle_t, size: size_t, clear: qboolean,
                  filename: *const libc::c_char, fileline: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    fn _Mem_Free(data: *mut libc::c_void, filename: *const libc::c_char,
                 fileline: libc::c_int);
    #[no_mangle]
    fn FS_LoadFile(path: *const libc::c_char, filesizeptr: *mut fs_offset_t,
                   gamedironly: qboolean) -> *mut byte;
    #[no_mangle]
    fn FS_Open(filepath: *const libc::c_char, mode: *const libc::c_char,
               gamedironly: qboolean) -> *mut file_t;
    #[no_mangle]
    fn FS_Printf(file: *mut file_t, format: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn FS_Close(file: *mut file_t) -> libc::c_int;
    #[no_mangle]
    fn Con_Reportf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
}
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type off_t = __off_t;
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
pub type fs_offset_t = off_t;
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
pub type netsrc_t = libc::c_uint;
pub const NS_COUNT: netsrc_t = 2;
pub const NS_SERVER: netsrc_t = 1;
pub const NS_CLIENT: netsrc_t = 0;
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
pub type xcommand_t = Option<unsafe extern "C" fn() -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct masterlist_s {
    pub list: *mut master_t,
    pub modified: qboolean,
}
/*
masterlist.c - multi-master list
Copyright (C) 2018 mittorn

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
*/
pub type master_t = master_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct master_s {
    pub next: *mut master_s,
    pub sent: qboolean,
    pub save: qboolean,
    pub address: string,
}
#[no_mangle]
pub static mut ml: masterlist_s =
    masterlist_s{list: 0 as *const master_t as *mut master_t,
                 modified: false_0,};
/*
========================
NET_SendToMasters

Send request to all masterservers list
return true if would block
========================
*/
#[no_mangle]
pub unsafe extern "C" fn NET_SendToMasters(mut sock: netsrc_t,
                                           mut len: size_t,
                                           mut data: *const libc::c_void)
 -> qboolean {
    let mut list: *mut master_t = 0 as *mut master_t;
    let mut wait: qboolean = false_0;
    list = ml.list;
    while !list.is_null() {
        let mut adr: netadr_t =
            netadr_t{type_0: NA_UNUSED, ip: [0; 4], ipx: [0; 10], port: 0,};
        let mut res: libc::c_int = 0;
        if !((*list).sent as u64 != 0) {
            res = NET_StringToAdrNB((*list).address.as_mut_ptr(), &mut adr);
            if res == 0 {
                Con_Reportf(b"Can\'t resolve adr: %s\n\x00" as *const u8 as
                                *const libc::c_char,
                            (*list).address.as_mut_ptr());
                (*list).sent = true_0
            } else if res == 2 as libc::c_int {
                (*list).sent = false_0;
                wait = true_0
            } else {
                (*list).sent = true_0;
                NET_SendPacket(sock, len, data, adr);
            }
        }
        list = (*list).next
    }
    if wait as u64 == 0 {
        list = ml.list;
        while !list.is_null() { (*list).sent = false_0; list = (*list).next }
    }
    return wait;
}
/*
========================
NET_AddMaster

Add master to the list
========================
*/
unsafe extern "C" fn NET_AddMaster(mut addr: *const libc::c_char,
                                   mut save: qboolean) {
    let mut master: *mut master_t = 0 as *mut master_t;
    let mut last: *mut master_t = 0 as *mut master_t;
    last = ml.list;
    while !last.is_null() && !(*last).next.is_null() {
        if Q_strncmp((*last).address.as_mut_ptr(), addr, 99999 as libc::c_int)
               == 0 {
            // already exists
            return
        }
        last = (*last).next
    }
    master =
        _Mem_Alloc(host.mempool,
                   ::std::mem::size_of::<master_t>() as libc::c_ulong,
                   false_0,
                   b"../engine/common/masterlist.c\x00" as *const u8 as
                       *const libc::c_char, 105 as libc::c_int) as
            *mut master_t;
    Q_strncpy((*master).address.as_mut_ptr(), addr,
              256 as libc::c_int as size_t);
    (*master).sent = false_0;
    (*master).save = save;
    (*master).next = 0 as *mut master_s;
    // link in
    if !last.is_null() {
        (*last).next = master
    } else { ml.list = master }; // save them into config
}
unsafe extern "C" fn NET_AddMaster_f() {
    if Cmd_Argc() != 2 as libc::c_int {
        Con_Printf(b"Usage: addmaster <address>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    NET_AddMaster(Cmd_Argv(1 as libc::c_int), true_0);
    ml.modified = true_0;
    // save config
}
/*
========================
NET_ClearMasters

Clear master list
========================
*/
unsafe extern "C" fn NET_ClearMasters_f() {
    while !ml.list.is_null() {
        let mut prev: *mut master_t = ml.list;
        ml.list = (*ml.list).next;
        _Mem_Free(prev as *mut libc::c_void,
                  b"../engine/common/masterlist.c\x00" as *const u8 as
                      *const libc::c_char, 143 as libc::c_int);
    };
}
/*
========================
NET_ListMasters_f

Display current master linked list
========================
*/
unsafe extern "C" fn NET_ListMasters_f() {
    let mut list: *mut master_t = 0 as *mut master_t;
    let mut i: libc::c_int = 0;
    Con_Printf(b"Master servers\n=============\n\x00" as *const u8 as
                   *const libc::c_char);
    i = 1 as libc::c_int;
    list = ml.list;
    while !list.is_null() {
        Con_Printf(b"%d\t%s\n\x00" as *const u8 as *const libc::c_char, i,
                   (*list).address.as_mut_ptr());
        i += 1;
        list = (*list).next
    };
}
/*
========================
NET_LoadMasters

Load master server list from xashcomm.lst
========================
*/
unsafe extern "C" fn NET_LoadMasters() {
    let mut afile: *mut byte = 0 as *mut byte;
    let mut pfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut token: [libc::c_char; 2048] = [0; 2048];
    afile =
        FS_LoadFile(b"xashcomm.lst\x00" as *const u8 as *const libc::c_char,
                    0 as *mut fs_offset_t, true_0);
    if afile.is_null() {
        // file doesn't exist yet
        Con_Reportf(b"Cannot load xashcomm.lst\n\x00" as *const u8 as
                        *const libc::c_char);
        return
    }
    pfile = afile as *mut libc::c_char;
    loop 
         // format: master <addr>\n
         {
        pfile =
            _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                               ::std::mem::size_of::<[libc::c_char; 2048]>()
                                   as libc::c_ulong as libc::c_int,
                               0 as libc::c_int as libc::c_uint,
                               0 as *mut libc::c_int);
        if pfile.is_null() { break ; }
        if Q_strncmp(token.as_mut_ptr(),
                     b"master\x00" as *const u8 as *const libc::c_char,
                     99999 as libc::c_int) == 0 {
            // load addr
            pfile =
                _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                                   ::std::mem::size_of::<[libc::c_char; 2048]>()
                                       as libc::c_ulong as libc::c_int,
                                   0 as libc::c_int as libc::c_uint,
                                   0 as *mut libc::c_int);
            NET_AddMaster(token.as_mut_ptr(), true_0);
        }
    }
    _Mem_Free(afile as *mut libc::c_void,
              b"../engine/common/masterlist.c\x00" as *const u8 as
                  *const libc::c_char, 202 as libc::c_int);
    ml.modified = false_0;
}
/*
========================
NET_SaveMasters

Save master server list to xashcomm.lst, except for default
========================
*/
#[no_mangle]
pub unsafe extern "C" fn NET_SaveMasters() {
    let mut f: *mut file_t = 0 as *mut file_t;
    let mut m: *mut master_t = 0 as *mut master_t;
    if ml.modified as u64 == 0 {
        Con_Reportf(b"Master server list not changed\n\x00" as *const u8 as
                        *const libc::c_char);
        return
    }
    f =
        FS_Open(b"xashcomm.lst\x00" as *const u8 as *const libc::c_char,
                b"w\x00" as *const u8 as *const libc::c_char, true_0);
    if f.is_null() {
        Con_Reportf(b"^1Error:^7 Couldn\'t write xashcomm.lst\n\x00" as
                        *const u8 as *const libc::c_char);
        return
    }
    m = ml.list;
    while !m.is_null() {
        if (*m).save as u64 != 0 {
            FS_Printf(f,
                      b"master %s\n\x00" as *const u8 as *const libc::c_char,
                      (*m).address.as_mut_ptr());
        }
        m = (*m).next
    }
    FS_Close(f);
}
/*
========================
NET_InitMasters

Initialize master server list
========================
*/
#[no_mangle]
pub unsafe extern "C" fn NET_InitMasters() {
    Cmd_AddRestrictedCommand(b"addmaster\x00" as *const u8 as
                                 *const libc::c_char,
                             Some(NET_AddMaster_f as
                                      unsafe extern "C" fn() -> ()),
                             b"add address to masterserver list\x00" as
                                 *const u8 as *const libc::c_char);
    Cmd_AddRestrictedCommand(b"clearmasters\x00" as *const u8 as
                                 *const libc::c_char,
                             Some(NET_ClearMasters_f as
                                      unsafe extern "C" fn() -> ()),
                             b"clear masterserver list\x00" as *const u8 as
                                 *const libc::c_char);
    Cmd_AddCommand(b"listmasters\x00" as *const u8 as *const libc::c_char,
                   Some(NET_ListMasters_f as unsafe extern "C" fn() -> ()),
                   b"list masterservers\x00" as *const u8 as
                       *const libc::c_char);
    // keep main master always there
    NET_AddMaster(b"ms.xash.su:27010\x00" as *const u8 as *const libc::c_char,
                  false_0);
    NET_AddMaster(b"ms2.xash.su:27010\x00" as *const u8 as
                      *const libc::c_char, false_0);
    NET_LoadMasters();
}
