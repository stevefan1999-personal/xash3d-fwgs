#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type file_s;
    #[no_mangle]
    fn Info_ValueForKey(s: *const libc::c_char, key: *const libc::c_char)
     -> *const libc::c_char;
    #[no_mangle]
    fn Con_DPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_Reportf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Q_strlen(string: *const libc::c_char) -> size_t;
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
    fn Q_timestamp(format: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn Cvar_VariableString(var_name: *const libc::c_char)
     -> *const libc::c_char;
    #[no_mangle]
    static mut sv_maxclients: *mut convar_t;
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    fn Cbuf_AddText(text: *const libc::c_char);
    #[no_mangle]
    fn Cmd_Argc() -> libc::c_int;
    #[no_mangle]
    fn Cmd_Argv(arg: libc::c_int) -> *const libc::c_char;
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
    fn FS_Open(filepath: *const libc::c_char, mode: *const libc::c_char,
               gamedironly: qboolean) -> *mut file_t;
    #[no_mangle]
    fn FS_Printf(file: *mut file_t, format: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn FS_Close(file: *mut file_t) -> libc::c_int;
    #[no_mangle]
    static mut svs: server_static_t;
    #[no_mangle]
    fn SV_ClientById(id: libc::c_int) -> *mut sv_client_t;
}
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
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
pub type uint64_t = __uint64_t;
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
pub type string_t = libc::c_int;
pub type word = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct color24 {
    pub r: byte,
    pub g: byte,
    pub b: byte,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct link_s {
    pub prev: *mut link_s,
    pub next: *mut link_s,
}
pub type link_t = link_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct edict_s {
    pub free: qboolean,
    pub serialnumber: libc::c_int,
    pub area: link_t,
    pub headnode: libc::c_int,
    pub num_leafs: libc::c_int,
    pub leafnums: [libc::c_short; 48],
    pub freetime: libc::c_float,
    pub pvPrivateData: *mut libc::c_void,
    pub v: entvars_t,
}
pub type entvars_t = entvars_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct entvars_s {
    pub classname: string_t,
    pub globalname: string_t,
    pub origin: vec3_t,
    pub oldorigin: vec3_t,
    pub velocity: vec3_t,
    pub basevelocity: vec3_t,
    pub clbasevelocity: vec3_t,
    pub movedir: vec3_t,
    pub angles: vec3_t,
    pub avelocity: vec3_t,
    pub punchangle: vec3_t,
    pub v_angle: vec3_t,
    pub endpos: vec3_t,
    pub startpos: vec3_t,
    pub impacttime: libc::c_float,
    pub starttime: libc::c_float,
    pub fixangle: libc::c_int,
    pub idealpitch: libc::c_float,
    pub pitch_speed: libc::c_float,
    pub ideal_yaw: libc::c_float,
    pub yaw_speed: libc::c_float,
    pub modelindex: libc::c_int,
    pub model: string_t,
    pub viewmodel: libc::c_int,
    pub weaponmodel: libc::c_int,
    pub absmin: vec3_t,
    pub absmax: vec3_t,
    pub mins: vec3_t,
    pub maxs: vec3_t,
    pub size: vec3_t,
    pub ltime: libc::c_float,
    pub nextthink: libc::c_float,
    pub movetype: libc::c_int,
    pub solid: libc::c_int,
    pub skin: libc::c_int,
    pub body: libc::c_int,
    pub effects: libc::c_int,
    pub gravity: libc::c_float,
    pub friction: libc::c_float,
    pub light_level: libc::c_int,
    pub sequence: libc::c_int,
    pub gaitsequence: libc::c_int,
    pub frame: libc::c_float,
    pub animtime: libc::c_float,
    pub framerate: libc::c_float,
    pub controller: [byte; 4],
    pub blending: [byte; 2],
    pub scale: libc::c_float,
    pub rendermode: libc::c_int,
    pub renderamt: libc::c_float,
    pub rendercolor: vec3_t,
    pub renderfx: libc::c_int,
    pub health: libc::c_float,
    pub frags: libc::c_float,
    pub weapons: libc::c_int,
    pub takedamage: libc::c_float,
    pub deadflag: libc::c_int,
    pub view_ofs: vec3_t,
    pub button: libc::c_int,
    pub impulse: libc::c_int,
    pub chain: *mut edict_t,
    pub dmg_inflictor: *mut edict_t,
    pub enemy: *mut edict_t,
    pub aiment: *mut edict_t,
    pub owner: *mut edict_t,
    pub groundentity: *mut edict_t,
    pub spawnflags: libc::c_int,
    pub flags: libc::c_int,
    pub colormap: libc::c_int,
    pub team: libc::c_int,
    pub max_health: libc::c_float,
    pub teleport_time: libc::c_float,
    pub armortype: libc::c_float,
    pub armorvalue: libc::c_float,
    pub waterlevel: libc::c_int,
    pub watertype: libc::c_int,
    pub target: string_t,
    pub targetname: string_t,
    pub netname: string_t,
    pub message: string_t,
    pub dmg_take: libc::c_float,
    pub dmg_save: libc::c_float,
    pub dmg: libc::c_float,
    pub dmgtime: libc::c_float,
    pub noise: string_t,
    pub noise1: string_t,
    pub noise2: string_t,
    pub noise3: string_t,
    pub speed: libc::c_float,
    pub air_finished: libc::c_float,
    pub pain_finished: libc::c_float,
    pub radsuit_finished: libc::c_float,
    pub pContainingEntity: *mut edict_t,
    pub playerclass: libc::c_int,
    pub maxspeed: libc::c_float,
    pub fov: libc::c_float,
    pub weaponanim: libc::c_int,
    pub pushmsec: libc::c_int,
    pub bInDuck: libc::c_int,
    pub flTimeStepSound: libc::c_int,
    pub flSwimTime: libc::c_int,
    pub flDuckTime: libc::c_int,
    pub iStepLeft: libc::c_int,
    pub flFallVelocity: libc::c_float,
    pub gamestate: libc::c_int,
    pub oldbuttons: libc::c_int,
    pub groupinfo: libc::c_int,
    pub iuser1: libc::c_int,
    pub iuser2: libc::c_int,
    pub iuser3: libc::c_int,
    pub iuser4: libc::c_int,
    pub fuser1: libc::c_float,
    pub fuser2: libc::c_float,
    pub fuser3: libc::c_float,
    pub fuser4: libc::c_float,
    pub vuser1: vec3_t,
    pub vuser2: vec3_t,
    pub vuser3: vec3_t,
    pub vuser4: vec3_t,
    pub euser1: *mut edict_t,
    pub euser2: *mut edict_t,
    pub euser3: *mut edict_t,
    pub euser4: *mut edict_t,
}
pub type edict_t = edict_s;
pub type C2RustUnnamed = libc::c_uint;
pub const TIME_FILENAME: C2RustUnnamed = 5;
pub const TIME_YEAR_ONLY: C2RustUnnamed = 4;
pub const TIME_NO_SECONDS: C2RustUnnamed = 3;
pub const TIME_TIME_ONLY: C2RustUnnamed = 2;
pub const TIME_DATE_ONLY: C2RustUnnamed = 1;
pub const TIME_FULL: C2RustUnnamed = 0;
pub type entity_state_t = entity_state_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct entity_state_s {
    pub entityType: libc::c_int,
    pub number: libc::c_int,
    pub msg_time: libc::c_float,
    pub messagenum: libc::c_int,
    pub origin: vec3_t,
    pub angles: vec3_t,
    pub modelindex: libc::c_int,
    pub sequence: libc::c_int,
    pub frame: libc::c_float,
    pub colormap: libc::c_int,
    pub skin: libc::c_short,
    pub solid: libc::c_short,
    pub effects: libc::c_int,
    pub scale: libc::c_float,
    pub eflags: byte,
    pub rendermode: libc::c_int,
    pub renderamt: libc::c_int,
    pub rendercolor: color24,
    pub renderfx: libc::c_int,
    pub movetype: libc::c_int,
    pub animtime: libc::c_float,
    pub framerate: libc::c_float,
    pub body: libc::c_int,
    pub controller: [byte; 4],
    pub blending: [byte; 4],
    pub velocity: vec3_t,
    pub mins: vec3_t,
    pub maxs: vec3_t,
    pub aiment: libc::c_int,
    pub owner: libc::c_int,
    pub friction: libc::c_float,
    pub gravity: libc::c_float,
    pub team: libc::c_int,
    pub playerclass: libc::c_int,
    pub health: libc::c_int,
    pub spectator: qboolean,
    pub weaponmodel: libc::c_int,
    pub gaitsequence: libc::c_int,
    pub basevelocity: vec3_t,
    pub usehull: libc::c_int,
    pub oldbuttons: libc::c_int,
    pub onground: libc::c_int,
    pub iStepLeft: libc::c_int,
    pub flFallVelocity: libc::c_float,
    pub fov: libc::c_float,
    pub weaponanim: libc::c_int,
    pub startpos: vec3_t,
    pub endpos: vec3_t,
    pub impacttime: libc::c_float,
    pub starttime: libc::c_float,
    pub iuser1: libc::c_int,
    pub iuser2: libc::c_int,
    pub iuser3: libc::c_int,
    pub iuser4: libc::c_int,
    pub fuser1: libc::c_float,
    pub fuser2: libc::c_float,
    pub fuser3: libc::c_float,
    pub fuser4: libc::c_float,
    pub vuser1: vec3_t,
    pub vuser2: vec3_t,
    pub vuser3: vec3_t,
    pub vuser4: vec3_t,
}
pub type resourcetype_t = libc::c_uint;
pub const t_world: resourcetype_t = 6;
pub const t_eventscript: resourcetype_t = 5;
pub const t_generic: resourcetype_t = 4;
pub const t_decal: resourcetype_t = 3;
pub const t_model: resourcetype_t = 2;
pub const t_skin: resourcetype_t = 1;
pub const t_sound: resourcetype_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct resource_s {
    pub szFileName: [libc::c_char; 64],
    pub type_0: resourcetype_t,
    pub nIndex: libc::c_int,
    pub nDownloadSize: libc::c_int,
    pub ucFlags: libc::c_uchar,
    pub rgucMD5_hash: [libc::c_uchar; 16],
    pub playernum: libc::c_uchar,
    pub rguc_reserved: [libc::c_uchar; 32],
    pub pNext: *mut resource_s,
    pub pPrev: *mut resource_s,
}
pub type resource_t = resource_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct customization_s {
    pub bInUse: qboolean,
    pub resource: resource_t,
    pub bTranslated: qboolean,
    pub nUserData1: libc::c_int,
    pub nUserData2: libc::c_int,
    pub pInfo: *mut libc::c_void,
    pub pBuffer: *mut libc::c_void,
    pub pNext: *mut customization_s,
}
pub type customization_t = customization_s;
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
pub type host_parm_t = host_parm_s;
pub type xcommand_t = Option<unsafe extern "C" fn() -> ()>;
/*
sv_filter.c - server ID/IP filter
Copyright (C) 2017 a1batross

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
*/
pub type ipfilter_t = ipfilter_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ipfilter_s {
    pub time: libc::c_float,
    pub endTime: libc::c_float,
    pub next: *mut ipfilter_s,
    pub mask: uint,
    pub ip: uint,
}
// -1 for permanent ban
// TODO: Is IP filter really needed?
// TODO: Make it IPv6 compatible, for future expansion
pub type cidfilter_t = cidfilter_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cidfilter_s {
    pub endTime: libc::c_float,
    pub next: *mut cidfilter_s,
    pub id: string,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sv_client_s {
    pub state: cl_state_t,
    pub upstate: cl_upload_t,
    pub name: [libc::c_char; 32],
    pub flags: uint,
    pub crcValue: CRC32_t,
    pub userinfo: [libc::c_char; 256],
    pub physinfo: [libc::c_char; 256],
    pub netchan: netchan_t,
    pub chokecount: libc::c_int,
    pub delta_sequence: libc::c_int,
    pub next_messagetime: libc::c_double,
    pub next_checkpingtime: libc::c_double,
    pub next_sendinfotime: libc::c_double,
    pub cl_updaterate: libc::c_double,
    pub timebase: libc::c_double,
    pub connection_started: libc::c_double,
    pub hashedcdkey: [libc::c_char; 34],
    pub customdata: customization_t,
    pub resourcesonhand: resource_t,
    pub resourcesneeded: resource_t,
    pub lastcmd: usercmd_t,
    pub connecttime: libc::c_double,
    pub cmdtime: libc::c_double,
    pub ignorecmdtime: libc::c_double,
    pub packet_loss: libc::c_int,
    pub latency: libc::c_float,
    pub ignored_ents: libc::c_int,
    pub edict: *mut edict_t,
    pub pViewEntity: *mut edict_t,
    pub viewentity: [*mut edict_t; 128],
    pub num_viewents: libc::c_int,
    pub m_bLoopback: qboolean,
    pub listeners: uint,
    pub datagram: sizebuf_t,
    pub datagram_buf: [byte; 16384],
    pub frames: *mut client_frame_t,
    pub events: event_state_t,
    pub challenge: libc::c_int,
    pub userid: libc::c_int,
    pub extensions: libc::c_int,
    pub useragent: [libc::c_char; 256],
}
pub type event_state_t = event_state_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_state_s {
    pub ei: [event_info_t; 64],
}
pub type event_info_t = event_info_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_info_s {
    pub index: word,
    pub packet_index: libc::c_short,
    pub entity_index: libc::c_short,
    pub fire_time: libc::c_float,
    pub args: event_args_t,
    pub flags: libc::c_int,
}
pub type event_args_t = event_args_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_args_s {
    pub flags: libc::c_int,
    pub entindex: libc::c_int,
    pub origin: [libc::c_float; 3],
    pub angles: [libc::c_float; 3],
    pub velocity: [libc::c_float; 3],
    pub ducking: libc::c_int,
    pub fparam1: libc::c_float,
    pub fparam2: libc::c_float,
    pub iparam1: libc::c_int,
    pub iparam2: libc::c_int,
    pub bparam1: libc::c_int,
    pub bparam2: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client_frame_t {
    pub senttime: libc::c_double,
    pub ping_time: libc::c_float,
    pub clientdata: clientdata_t,
    pub weapondata: [weapon_data_t; 64],
    pub num_entities: libc::c_int,
    pub first_entity: libc::c_int,
}
pub type weapon_data_t = weapon_data_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct weapon_data_s {
    pub m_iId: libc::c_int,
    pub m_iClip: libc::c_int,
    pub m_flNextPrimaryAttack: libc::c_float,
    pub m_flNextSecondaryAttack: libc::c_float,
    pub m_flTimeWeaponIdle: libc::c_float,
    pub m_fInReload: libc::c_int,
    pub m_fInSpecialReload: libc::c_int,
    pub m_flNextReload: libc::c_float,
    pub m_flPumpTime: libc::c_float,
    pub m_fReloadTime: libc::c_float,
    pub m_fAimedDamage: libc::c_float,
    pub m_fNextAimBonus: libc::c_float,
    pub m_fInZoom: libc::c_int,
    pub m_iWeaponState: libc::c_int,
    pub iuser1: libc::c_int,
    pub iuser2: libc::c_int,
    pub iuser3: libc::c_int,
    pub iuser4: libc::c_int,
    pub fuser1: libc::c_float,
    pub fuser2: libc::c_float,
    pub fuser3: libc::c_float,
    pub fuser4: libc::c_float,
}
pub type clientdata_t = clientdata_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clientdata_s {
    pub origin: vec3_t,
    pub velocity: vec3_t,
    pub viewmodel: libc::c_int,
    pub punchangle: vec3_t,
    pub flags: libc::c_int,
    pub waterlevel: libc::c_int,
    pub watertype: libc::c_int,
    pub view_ofs: vec3_t,
    pub health: libc::c_float,
    pub bInDuck: libc::c_int,
    pub weapons: libc::c_int,
    pub flTimeStepSound: libc::c_int,
    pub flDuckTime: libc::c_int,
    pub flSwimTime: libc::c_int,
    pub waterjumptime: libc::c_int,
    pub maxspeed: libc::c_float,
    pub fov: libc::c_float,
    pub weaponanim: libc::c_int,
    pub m_iId: libc::c_int,
    pub ammo_shells: libc::c_int,
    pub ammo_nails: libc::c_int,
    pub ammo_cells: libc::c_int,
    pub ammo_rockets: libc::c_int,
    pub m_flNextAttack: libc::c_float,
    pub tfstate: libc::c_int,
    pub pushmsec: libc::c_int,
    pub deadflag: libc::c_int,
    pub physinfo: [libc::c_char; 256],
    pub iuser1: libc::c_int,
    pub iuser2: libc::c_int,
    pub iuser3: libc::c_int,
    pub iuser4: libc::c_int,
    pub fuser1: libc::c_float,
    pub fuser2: libc::c_float,
    pub fuser3: libc::c_float,
    pub fuser4: libc::c_float,
    pub vuser1: vec3_t,
    pub vuser2: vec3_t,
    pub vuser3: vec3_t,
    pub vuser4: vec3_t,
}
pub type sizebuf_t = sizebuf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sizebuf_s {
    pub bOverflow: qboolean,
    pub pDebugName: *const libc::c_char,
    pub pData: *mut byte,
    pub iCurBit: libc::c_int,
    pub nDataBits: libc::c_int,
}
pub type usercmd_t = usercmd_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct usercmd_s {
    pub lerp_msec: libc::c_short,
    pub msec: byte,
    pub viewangles: vec3_t,
    pub forwardmove: libc::c_float,
    pub sidemove: libc::c_float,
    pub upmove: libc::c_float,
    pub lightlevel: byte,
    pub buttons: libc::c_ushort,
    pub impulse: byte,
    pub weaponselect: byte,
    pub impact_index: libc::c_int,
    pub impact_position: vec3_t,
}
pub type netchan_t = netchan_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct netchan_s {
    pub sock: netsrc_t,
    pub remote_address: netadr_t,
    pub qport: libc::c_int,
    pub last_received: libc::c_double,
    pub connect_time: libc::c_double,
    pub rate: libc::c_double,
    pub cleartime: libc::c_double,
    pub incoming_sequence: libc::c_uint,
    pub incoming_acknowledged: libc::c_uint,
    pub incoming_reliable_acknowledged: libc::c_uint,
    pub incoming_reliable_sequence: libc::c_uint,
    pub outgoing_sequence: libc::c_uint,
    pub reliable_sequence: libc::c_uint,
    pub last_reliable_sequence: libc::c_uint,
    pub client: *mut libc::c_void,
    pub pfnBlockSize: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                  _: fragsize_t)
                                 -> libc::c_int>,
    pub message: sizebuf_t,
    pub message_buf: [byte; 131120],
    pub reliable_length: libc::c_int,
    pub reliable_buf: [byte; 131120],
    pub waitlist: [*mut fragbufwaiting_t; 2],
    pub reliable_fragment: [libc::c_int; 2],
    pub reliable_fragid: [uint; 2],
    pub fragbufs: [*mut fragbuf_t; 2],
    pub fragbufcount: [libc::c_int; 2],
    pub frag_startpos: [libc::c_int; 2],
    pub frag_length: [libc::c_int; 2],
    pub incomingbufs: [*mut fragbuf_t; 2],
    pub incomingready: [qboolean; 2],
    pub incomingfilename: [libc::c_char; 260],
    pub tempbuffer: *mut libc::c_void,
    pub tempbuffersize: libc::c_int,
    pub flow: [flow_t; 2],
    pub total_sended: size_t,
    pub total_received: size_t,
    pub split: qboolean,
    pub maxpacket: libc::c_uint,
    pub splitid: libc::c_uint,
    pub netsplit: netsplit_t,
}
pub type netsplit_t = netsplit_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct netsplit_s {
    pub packets: [netsplit_chain_packet_t; 8],
    pub total_received: uint64_t,
    pub total_received_uncompressed: uint64_t,
}
pub type netsplit_chain_packet_t = netsplit_chain_packet_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct netsplit_chain_packet_s {
    pub recieved_v: [uint32_t; 8],
    pub id: uint32_t,
    pub data: [byte; 131072],
    pub received: byte,
    pub count: byte,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flow_t {
    pub stats: [flowstats_t; 32],
    pub current: libc::c_int,
    pub nextcompute: libc::c_double,
    pub kbytespersec: libc::c_float,
    pub avgkbytespersec: libc::c_float,
    pub totalbytes: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flowstats_t {
    pub size: libc::c_int,
    pub time: libc::c_double,
}
pub type fragbuf_t = fragbuf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fragbuf_s {
    pub next: *mut fragbuf_s,
    pub bufferid: libc::c_int,
    pub frag_message: sizebuf_t,
    pub frag_message_buf: [byte; 65535],
    pub isfile: qboolean,
    pub isbuffer: qboolean,
    pub iscompressed: qboolean,
    pub filename: [libc::c_char; 260],
    pub foffset: libc::c_int,
    pub size: libc::c_int,
}
pub type fragbufwaiting_t = fbufqueue_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fbufqueue_s {
    pub next: *mut fbufqueue_s,
    pub fragbufcount: libc::c_int,
    pub fragbufs: *mut fragbuf_t,
}
pub type fragsize_t = fragsize_e;
pub type fragsize_e = libc::c_uint;
pub const FRAGSIZE_UNRELIABLE: fragsize_e = 2;
pub const FRAGSIZE_SPLIT: fragsize_e = 1;
pub const FRAGSIZE_FRAG: fragsize_e = 0;
pub type CRC32_t = libc::c_uint;
pub type cl_upload_t = libc::c_uint;
pub const us_complete: cl_upload_t = 2;
pub const us_processing: cl_upload_t = 1;
pub const us_inactive: cl_upload_t = 0;
pub type cl_state_t = libc::c_uint;
pub const cs_spawned: cl_state_t = 3;
pub const cs_connected: cl_state_t = 2;
pub const cs_zombie: cl_state_t = 1;
pub const cs_free: cl_state_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct server_log_t {
    pub active: qboolean,
    pub net_log: qboolean,
    pub net_address: netadr_t,
    pub file: *mut file_t,
}
pub type sv_client_t = sv_client_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct challenge_t {
    pub adr: netadr_t,
    pub time: libc::c_double,
    pub challenge: libc::c_int,
    pub connected: qboolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct server_static_t {
    pub initialized: qboolean,
    pub game_library_loaded: qboolean,
    pub timestart: libc::c_double,
    pub maxclients: libc::c_int,
    pub groupmask: libc::c_int,
    pub groupop: libc::c_int,
    pub log: server_log_t,
    pub serverinfo: [libc::c_char; 512],
    pub localinfo: [libc::c_char; 32768],
    pub spawncount: libc::c_int,
    pub clients: *mut sv_client_t,
    pub num_client_entities: libc::c_int,
    pub next_client_entities: libc::c_int,
    pub packet_entities: *mut entity_state_t,
    pub baselines: *mut entity_state_t,
    pub static_entities: *mut entity_state_t,
    pub last_heartbeat: libc::c_double,
    pub challenges: [challenge_t; 1024],
}
static mut ipfilter: *mut ipfilter_t =
    0 as *const ipfilter_t as *mut ipfilter_t;
static mut cidfilter: *mut cidfilter_t =
    0 as *const cidfilter_t as *mut cidfilter_t;
unsafe extern "C" fn SV_RemoveID(mut id: *const libc::c_char) {
    let mut filter: *mut cidfilter_t =
        0 as *mut cidfilter_t; // no negative time
    let mut prevfilter: *mut cidfilter_t = 0 as *mut cidfilter_t;
    filter = cidfilter;
    while !filter.is_null() {
        if Q_strncmp((*filter).id.as_mut_ptr(), id, 99999 as libc::c_int) != 0
           {
            prevfilter = filter;
            filter = (*filter).next
        } else {
            if filter == cidfilter {
                cidfilter = (*cidfilter).next;
                _Mem_Free(filter as *mut libc::c_void,
                          b"../engine/server/sv_filter.c\x00" as *const u8 as
                              *const libc::c_char, 59 as libc::c_int);
                return
            }
            if !prevfilter.is_null() { (*prevfilter).next = (*filter).next }
            _Mem_Free(filter as *mut libc::c_void,
                      b"../engine/server/sv_filter.c\x00" as *const u8 as
                          *const libc::c_char, 65 as libc::c_int);
            return
        }
    };
}
unsafe extern "C" fn SV_RemoveIP(mut ip: uint, mut mask: uint) {
    let mut filter: *mut ipfilter_t = 0 as *mut ipfilter_t;
    let mut prevfilter: *mut ipfilter_t = 0 as *mut ipfilter_t;
    filter = ipfilter;
    while !filter.is_null() {
        if (*filter).ip != ip || mask != (*filter).mask {
            prevfilter = filter;
            filter = (*filter).next
        } else {
            if filter == ipfilter {
                ipfilter = (*ipfilter).next;
                _Mem_Free(filter as *mut libc::c_void,
                          b"../engine/server/sv_filter.c\x00" as *const u8 as
                              *const libc::c_char, 85 as libc::c_int);
                return
            }
            if !prevfilter.is_null() { (*prevfilter).next = (*filter).next }
            _Mem_Free(filter as *mut libc::c_void,
                      b"../engine/server/sv_filter.c\x00" as *const u8 as
                          *const libc::c_char, 91 as libc::c_int);
            return
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn SV_CheckID(mut id: *const libc::c_char) -> qboolean {
    let mut ret: qboolean = false_0;
    let mut filter: *mut cidfilter_t = 0 as *mut cidfilter_t;
    filter = cidfilter;
    while !filter.is_null() {
        let mut len1: libc::c_int = Q_strlen(id) as libc::c_int;
        let mut len2: libc::c_int =
            Q_strlen((*filter).id.as_mut_ptr()) as libc::c_int;
        let mut len: libc::c_int = if len1 < len2 { len1 } else { len2 };
        while (*filter).endTime != 0. &&
                  host.realtime > (*filter).endTime as libc::c_double {
            let mut fid: *mut libc::c_char = (*filter).id.as_mut_ptr();
            filter = (*filter).next;
            SV_RemoveID(fid);
            if filter.is_null() { return false_0 }
        }
        if Q_strncmp(id, (*filter).id.as_mut_ptr(), len) == 0 {
            ret = true_0;
            break ;
        } else { filter = (*filter).next }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn SV_CheckIP(mut addr: *mut netadr_t) -> qboolean {
    let mut ip: uint =
        (((*addr).ip[0 as libc::c_int as usize] as libc::c_int) <<
             24 as libc::c_int |
             ((*addr).ip[1 as libc::c_int as usize] as libc::c_int) <<
                 16 as libc::c_int |
             ((*addr).ip[2 as libc::c_int as usize] as libc::c_int) <<
                 8 as libc::c_int |
             (*addr).ip[3 as libc::c_int as usize] as libc::c_int) as uint;
    let mut ret: qboolean = false_0;
    let mut filter: *mut ipfilter_t = 0 as *mut ipfilter_t;
    filter = ipfilter;
    while !filter.is_null() {
        while (*filter).endTime != 0. &&
                  host.realtime > (*filter).endTime as libc::c_double {
            let mut rip: uint = (*filter).ip;
            let mut rmask: uint = (*filter).mask;
            SV_RemoveIP(rip, rmask);
            filter = (*filter).next;
            if filter.is_null() { return false_0 }
        }
        if ip & (*filter).mask == (*filter).ip & (*filter).mask {
            ret = true_0;
            break ;
        } else { filter = (*filter).next }
    }
    return ret;
}
unsafe extern "C" fn SV_BanID_f() {
    let mut time: libc::c_float = Q_atof(Cmd_Argv(1 as libc::c_int));
    let mut id: *const libc::c_char = Cmd_Argv(2 as libc::c_int);
    let mut cl: *mut sv_client_t = 0 as *mut sv_client_t;
    let mut filter: *mut cidfilter_t = 0 as *mut cidfilter_t;
    if time != 0. {
        time =
            (host.realtime + (time * 60.0f32) as libc::c_double) as
                libc::c_float
    }
    if *id.offset(0 as libc::c_int as isize) == 0 {
        Con_Reportf(b"Usage: banid <minutes> <#userid or unique id>\n0 minutes for permanent ban\n\x00"
                        as *const u8 as *const libc::c_char);
        return
    }
    if Q_strnicmp(id, b"STEAM_\x00" as *const u8 as *const libc::c_char,
                  6 as libc::c_int) == 0 ||
           Q_strnicmp(id, b"VALVE_\x00" as *const u8 as *const libc::c_char,
                      6 as libc::c_int) == 0 {
        id = id.offset(6 as libc::c_int as isize)
    }
    if Q_strnicmp(id, b"XASH_\x00" as *const u8 as *const libc::c_char,
                  5 as libc::c_int) == 0 {
        id = id.offset(5 as libc::c_int as isize)
    }
    if !svs.clients.is_null() {
        if *id.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32
           {
            cl = SV_ClientById(Q_atoi(id.offset(1 as libc::c_int as isize)))
        }
        if cl.is_null() {
            let mut i: libc::c_int = 0;
            let mut cl1: *mut sv_client_t = 0 as *mut sv_client_t;
            let mut len: libc::c_int = Q_strlen(id) as libc::c_int;
            i = 0 as libc::c_int;
            cl1 = svs.clients;
            while (i as libc::c_float) < (*sv_maxclients).value {
                if Q_strncmp(id,
                             Info_ValueForKey((*cl1).useragent.as_mut_ptr(),
                                              b"uuid\x00" as *const u8 as
                                                  *const libc::c_char), len)
                       == 0 {
                    cl = cl1;
                    break ;
                } else { i += 1; cl1 = cl1.offset(1) }
            }
        }
        if cl.is_null() {
            Con_DPrintf(b"^3Warning:^7 banid: no such player\n\x00" as
                            *const u8 as *const libc::c_char);
        } else {
            id =
                Info_ValueForKey((*cl).useragent.as_mut_ptr(),
                                 b"uuid\x00" as *const u8 as
                                     *const libc::c_char)
        }
        if *id.offset(0 as libc::c_int as isize) == 0 {
            Con_DPrintf(b"^1Error:^7 Could not ban, not implemented yet\n\x00"
                            as *const u8 as *const libc::c_char);
            return
        }
    }
    if *id.offset(0 as libc::c_int as isize) == 0 ||
           *id.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32
       {
        Con_DPrintf(b"^1Error:^7 banid: bad id\n\x00" as *const u8 as
                        *const libc::c_char);
        return
    }
    SV_RemoveID(id);
    filter =
        _Mem_Alloc(host.mempool,
                   ::std::mem::size_of::<cidfilter_t>() as libc::c_ulong,
                   false_0,
                   b"../engine/server/sv_filter.c\x00" as *const u8 as
                       *const libc::c_char, 217 as libc::c_int) as
            *mut cidfilter_t;
    (*filter).endTime = time;
    (*filter).next = cidfilter;
    Q_strncpy((*filter).id.as_mut_ptr(), id,
              ::std::mem::size_of::<string>() as libc::c_ulong);
    cidfilter = filter;
    if !cl.is_null() &&
           Q_strnicmp(Cmd_Argv(Cmd_Argc() - 1 as libc::c_int),
                      b"kick\x00" as *const u8 as *const libc::c_char,
                      99999 as libc::c_int) == 0 {
        Cbuf_AddText(va(b"kick #%d \"Kicked and banned\"\n\x00" as *const u8
                            as *const libc::c_char, (*cl).userid));
    };
}
unsafe extern "C" fn SV_ListID_f() {
    let mut filter: *mut cidfilter_t = 0 as *mut cidfilter_t;
    Con_Reportf(b"id ban list\n\x00" as *const u8 as *const libc::c_char);
    Con_Reportf(b"-----------\n\x00" as *const u8 as *const libc::c_char);
    filter = cidfilter;
    while !filter.is_null() {
        if !((*filter).endTime != 0. &&
                 host.realtime > (*filter).endTime as libc::c_double) {
            if (*filter).endTime != 0. {
                Con_Reportf(b"%s expries in %f minutes\n\x00" as *const u8 as
                                *const libc::c_char,
                            (*filter).id.as_mut_ptr(),
                            ((*filter).endTime as libc::c_double -
                                 host.realtime) / 60.0f32 as libc::c_double);
            } else {
                Con_Reportf(b"%s permanent\n\x00" as *const u8 as
                                *const libc::c_char,
                            (*filter).id.as_mut_ptr());
            }
        }
        filter = (*filter).next
    };
}
unsafe extern "C" fn SV_RemoveID_f() {
    let mut id: *const libc::c_char = Cmd_Argv(1 as libc::c_int);
    if *id.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32 &&
           !svs.clients.is_null() {
        let mut num: libc::c_int =
            Q_atoi(id.offset(1 as libc::c_int as isize));
        if num as libc::c_float >= (*sv_maxclients).value ||
               num < 0 as libc::c_int {
            return
        }
        id =
            Info_ValueForKey((*svs.clients.offset(num as
                                                      isize)).useragent.as_mut_ptr(),
                             b"uuid\x00" as *const u8 as *const libc::c_char)
    }
    if *id.offset(0 as libc::c_int as isize) == 0 {
        Con_Reportf(b"Usage: removeid <#slotnumber or uniqueid>\n\x00" as
                        *const u8 as *const libc::c_char);
        return
    }
    SV_RemoveID(id);
}
unsafe extern "C" fn SV_WriteID_f() {
    let mut f: *mut file_t =
        FS_Open(Cvar_VariableString(b"bannedcfgfile\x00" as *const u8 as
                                        *const libc::c_char),
                b"w\x00" as *const u8 as *const libc::c_char, false_0);
    let mut filter: *mut cidfilter_t = 0 as *mut cidfilter_t;
    if f.is_null() {
        Con_DPrintf(b"^1Error:^7 Could not write %s\n\x00" as *const u8 as
                        *const libc::c_char,
                    Cvar_VariableString(b"bannedcfgfile\x00" as *const u8 as
                                            *const libc::c_char));
        return
    }
    FS_Printf(f,
              b"//=======================================================================\n\x00"
                  as *const u8 as *const libc::c_char);
    FS_Printf(f,
              b"//\t\tCopyright Flying With Gauss Team %s \xc2\xa9\n\x00" as
                  *const u8 as *const libc::c_char,
              Q_timestamp(TIME_YEAR_ONLY as libc::c_int));
    FS_Printf(f,
              b"//\t\t    %s - archive of id blacklist\n\x00" as *const u8 as
                  *const libc::c_char,
              Cvar_VariableString(b"bannedcfgfile\x00" as *const u8 as
                                      *const libc::c_char));
    FS_Printf(f,
              b"//=======================================================================\n\x00"
                  as *const u8 as *const libc::c_char);
    filter = cidfilter;
    while !filter.is_null() {
        if (*filter).endTime == 0. {
            // only permanent
            FS_Printf(f,
                      b"banid 0 %s\n\x00" as *const u8 as *const libc::c_char,
                      (*filter).id.as_mut_ptr()); // no negative time
        }
        filter = (*filter).next
    }
    FS_Close(f);
}
unsafe extern "C" fn StringToIP(mut str: *const libc::c_char,
                                mut maskstr: *const libc::c_char,
                                mut outip: *mut uint, mut outmask: *mut uint)
 -> qboolean {
    let mut ip: [byte; 4] = [0 as libc::c_int as byte, 0, 0, 0];
    let mut mask: [byte; 4] = [0 as libc::c_int as byte, 0, 0, 0];
    let mut i: libc::c_int = 0 as libc::c_int;
    if *str as libc::c_int > '9' as i32 || (*str as libc::c_int) < '0' as i32
       {
        return false_0
    }
    loop  {
        while *str as libc::c_int <= '9' as i32 &&
                  *str as libc::c_int >= '0' as i32 {
            ip[i as usize] =
                (ip[i as usize] as libc::c_int * 10 as libc::c_int) as byte;
            ip[i as usize] =
                (ip[i as usize] as libc::c_int +
                     (*str as libc::c_int - '0' as i32)) as byte;
            str = str.offset(1)
        }
        mask[i as usize] = 255 as libc::c_int as byte;
        i += 1;
        if *str as libc::c_int != '.' as i32 { break ; }
        str = str.offset(1);
        if !(i < 4 as libc::c_int) { break ; }
    }
    i = 0 as libc::c_int;
    if !(maskstr.is_null() || *maskstr as libc::c_int > '9' as i32 ||
             (*maskstr as libc::c_int) < '0' as i32) {
        loop  {
            let mut mask1: byte = 0 as libc::c_int as byte;
            while *maskstr as libc::c_int <= '9' as i32 &&
                      *maskstr as libc::c_int >= '0' as i32 {
                mask1 = (mask1 as libc::c_int * 10 as libc::c_int) as byte;
                mask1 =
                    (mask1 as libc::c_int +
                         (*maskstr as libc::c_int - '0' as i32)) as byte;
                maskstr = maskstr.offset(1)
            }
            mask[i as usize] =
                (mask[i as usize] as libc::c_int & mask1 as libc::c_int) as
                    byte;
            i += 1;
            if *maskstr as libc::c_int != '.' as i32 { break ; }
            maskstr = maskstr.offset(1);
            if !(i < 4 as libc::c_int) { break ; }
        }
    }
    *outip =
        ((ip[0 as libc::c_int as usize] as libc::c_int) << 24 as libc::c_int |
             (ip[1 as libc::c_int as usize] as libc::c_int) <<
                 16 as libc::c_int |
             (ip[2 as libc::c_int as usize] as libc::c_int) <<
                 8 as libc::c_int |
             ip[3 as libc::c_int as usize] as libc::c_int) as uint;
    if !outmask.is_null() {
        *outmask =
            ((mask[0 as libc::c_int as usize] as libc::c_int) <<
                 24 as libc::c_int |
                 (mask[1 as libc::c_int as usize] as libc::c_int) <<
                     16 as libc::c_int |
                 (mask[2 as libc::c_int as usize] as libc::c_int) <<
                     8 as libc::c_int |
                 mask[3 as libc::c_int as usize] as libc::c_int) as uint
    }
    return true_0;
}
unsafe extern "C" fn SV_AddIP_f() {
    let mut time: libc::c_float = Q_atof(Cmd_Argv(1 as libc::c_int));
    let mut ipstr: *const libc::c_char = Cmd_Argv(2 as libc::c_int);
    let mut maskstr: *const libc::c_char = Cmd_Argv(3 as libc::c_int);
    let mut ip: uint = 0;
    let mut mask: uint = 0;
    let mut filter: *mut ipfilter_t = 0 as *mut ipfilter_t;
    if time != 0. {
        time =
            (host.realtime + (time * 60.0f32) as libc::c_double) as
                libc::c_float
    }
    if StringToIP(ipstr, maskstr, &mut ip, &mut mask) as u64 == 0 {
        Con_Reportf(b"Usage: addip <minutes> <ip> [mask]\n0 minutes for permanent ban\n\x00"
                        as *const u8 as *const libc::c_char);
        return
    }
    SV_RemoveIP(ip, mask);
    filter =
        _Mem_Alloc(host.mempool,
                   ::std::mem::size_of::<ipfilter_t>() as libc::c_ulong,
                   false_0,
                   b"../engine/server/sv_filter.c\x00" as *const u8 as
                       *const libc::c_char, 363 as libc::c_int) as
            *mut ipfilter_t;
    (*filter).endTime = time;
    (*filter).ip = ip;
    (*filter).mask = mask;
    (*filter).next = ipfilter;
    ipfilter = filter;
}
unsafe extern "C" fn SV_ListIP_f() {
    let mut filter: *mut ipfilter_t = 0 as *mut ipfilter_t;
    Con_Reportf(b"ip ban list\n\x00" as *const u8 as *const libc::c_char);
    Con_Reportf(b"-----------\n\x00" as *const u8 as *const libc::c_char);
    filter = ipfilter;
    while !filter.is_null() {
        if !((*filter).endTime != 0. &&
                 host.realtime > (*filter).endTime as libc::c_double) {
            if (*filter).endTime != 0. {
                Con_Reportf(b"%d.%d.%d.%d %d.%d.%d.%d expries in %f minutes\n\x00"
                                as *const u8 as *const libc::c_char,
                            (*filter).ip >> 24 as libc::c_int &
                                0xff as libc::c_int as libc::c_uint,
                            (*filter).ip >> 16 as libc::c_int &
                                0xff as libc::c_int as libc::c_uint,
                            (*filter).ip >> 8 as libc::c_int &
                                0xff as libc::c_int as libc::c_uint,
                            (*filter).ip &
                                0xff as libc::c_int as libc::c_uint,
                            (*filter).mask >> 24 as libc::c_int &
                                0xff as libc::c_int as libc::c_uint,
                            (*filter).mask >> 16 as libc::c_int &
                                0xff as libc::c_int as libc::c_uint,
                            (*filter).mask >> 8 as libc::c_int &
                                0xff as libc::c_int as libc::c_uint,
                            (*filter).mask &
                                0xff as libc::c_int as libc::c_uint,
                            ((*filter).endTime as libc::c_double -
                                 host.realtime) / 60.0f32 as libc::c_double);
            } else {
                Con_Reportf(b"%d.%d.%d.%d %d.%d.%d.%d permanent\n\x00" as
                                *const u8 as *const libc::c_char,
                            (*filter).ip >> 24 as libc::c_int &
                                0xff as libc::c_int as libc::c_uint,
                            (*filter).ip >> 16 as libc::c_int &
                                0xff as libc::c_int as libc::c_uint,
                            (*filter).ip >> 8 as libc::c_int &
                                0xff as libc::c_int as libc::c_uint,
                            (*filter).ip &
                                0xff as libc::c_int as libc::c_uint,
                            (*filter).mask >> 24 as libc::c_int &
                                0xff as libc::c_int as libc::c_uint,
                            (*filter).mask >> 16 as libc::c_int &
                                0xff as libc::c_int as libc::c_uint,
                            (*filter).mask >> 8 as libc::c_int &
                                0xff as libc::c_int as libc::c_uint,
                            (*filter).mask &
                                0xff as libc::c_int as libc::c_uint);
            }
        }
        filter = (*filter).next
    };
}
unsafe extern "C" fn SV_RemoveIP_f() {
    let mut ip: uint = 0;
    let mut mask: uint = 0;
    if StringToIP(Cmd_Argv(1 as libc::c_int), Cmd_Argv(2 as libc::c_int),
                  &mut ip, &mut mask) as u64 == 0 {
        Con_Reportf(b"Usage: removeip <ip> [mask]\n\x00" as *const u8 as
                        *const libc::c_char);
        return
    }
    SV_RemoveIP(ip, mask);
}
unsafe extern "C" fn SV_WriteIP_f() {
    let mut f: *mut file_t =
        FS_Open(Cvar_VariableString(b"listipcfgfile\x00" as *const u8 as
                                        *const libc::c_char),
                b"w\x00" as *const u8 as *const libc::c_char, false_0);
    let mut filter: *mut ipfilter_t = 0 as *mut ipfilter_t;
    if f.is_null() {
        Con_DPrintf(b"^1Error:^7 Could not write %s\n\x00" as *const u8 as
                        *const libc::c_char,
                    Cvar_VariableString(b"listipcfgfile\x00" as *const u8 as
                                            *const libc::c_char));
        return
    }
    FS_Printf(f,
              b"//=======================================================================\n\x00"
                  as *const u8 as *const libc::c_char);
    FS_Printf(f,
              b"//\t\tCopyright Flying With Gauss Team %s \xc2\xa9\n\x00" as
                  *const u8 as *const libc::c_char,
              Q_timestamp(TIME_YEAR_ONLY as libc::c_int));
    FS_Printf(f,
              b"//\t\t    %s - archive of IP blacklist\n\x00" as *const u8 as
                  *const libc::c_char,
              Cvar_VariableString(b"listipcfgfile\x00" as *const u8 as
                                      *const libc::c_char));
    FS_Printf(f,
              b"//=======================================================================\n\x00"
                  as *const u8 as *const libc::c_char);
    filter = ipfilter;
    while !filter.is_null() {
        if (*filter).endTime == 0. {
            // only permanent
            FS_Printf(f,
                      b"addip 0 %d.%d.%d.%d %d.%d.%d.%d\n\x00" as *const u8 as
                          *const libc::c_char,
                      (*filter).ip >> 24 as libc::c_int &
                          0xff as libc::c_int as libc::c_uint,
                      (*filter).ip >> 16 as libc::c_int &
                          0xff as libc::c_int as libc::c_uint,
                      (*filter).ip >> 8 as libc::c_int &
                          0xff as libc::c_int as libc::c_uint,
                      (*filter).ip & 0xff as libc::c_int as libc::c_uint,
                      (*filter).mask >> 24 as libc::c_int &
                          0xff as libc::c_int as libc::c_uint,
                      (*filter).mask >> 16 as libc::c_int &
                          0xff as libc::c_int as libc::c_uint,
                      (*filter).mask >> 8 as libc::c_int &
                          0xff as libc::c_int as libc::c_uint,
                      (*filter).mask & 0xff as libc::c_int as libc::c_uint);
        }
        filter = (*filter).next
    }
    FS_Close(f);
}
#[no_mangle]
pub unsafe extern "C" fn SV_InitFilter() {
    Cmd_AddRestrictedCommand(b"banid\x00" as *const u8 as *const libc::c_char,
                             Some(SV_BanID_f as unsafe extern "C" fn() -> ()),
                             b"ban player by ID\x00" as *const u8 as
                                 *const libc::c_char);
    Cmd_AddRestrictedCommand(b"listid\x00" as *const u8 as
                                 *const libc::c_char,
                             Some(SV_ListID_f as
                                      unsafe extern "C" fn() -> ()),
                             b"list banned players\x00" as *const u8 as
                                 *const libc::c_char);
    Cmd_AddRestrictedCommand(b"removeid\x00" as *const u8 as
                                 *const libc::c_char,
                             Some(SV_RemoveID_f as
                                      unsafe extern "C" fn() -> ()),
                             b"remove player from banned list\x00" as
                                 *const u8 as *const libc::c_char);
    Cmd_AddRestrictedCommand(b"writeid\x00" as *const u8 as
                                 *const libc::c_char,
                             Some(SV_WriteID_f as
                                      unsafe extern "C" fn() -> ()),
                             b"write banned.cfg\x00" as *const u8 as
                                 *const libc::c_char);
    Cmd_AddRestrictedCommand(b"addip\x00" as *const u8 as *const libc::c_char,
                             Some(SV_AddIP_f as unsafe extern "C" fn() -> ()),
                             b"add entry to IP filter\x00" as *const u8 as
                                 *const libc::c_char);
    Cmd_AddRestrictedCommand(b"listip\x00" as *const u8 as
                                 *const libc::c_char,
                             Some(SV_ListIP_f as
                                      unsafe extern "C" fn() -> ()),
                             b"list current IP filter\x00" as *const u8 as
                                 *const libc::c_char);
    Cmd_AddRestrictedCommand(b"removeip\x00" as *const u8 as
                                 *const libc::c_char,
                             Some(SV_RemoveIP_f as
                                      unsafe extern "C" fn() -> ()),
                             b"remove IP filter\x00" as *const u8 as
                                 *const libc::c_char);
    Cmd_AddRestrictedCommand(b"writeip\x00" as *const u8 as
                                 *const libc::c_char,
                             Some(SV_WriteIP_f as
                                      unsafe extern "C" fn() -> ()),
                             b"write listip.cfg\x00" as *const u8 as
                                 *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn SV_ShutdownFilter() {
    let mut ipList: *mut ipfilter_t = 0 as *mut ipfilter_t;
    let mut ipNext: *mut ipfilter_t = 0 as *mut ipfilter_t;
    let mut cidList: *mut cidfilter_t = 0 as *mut cidfilter_t;
    let mut cidNext: *mut cidfilter_t = 0 as *mut cidfilter_t;
    // should be called manually because banned.cfg is not executed by engine
	//SV_WriteIP_f();
	//SV_WriteID_f();
    ipList = ipfilter;
    while !ipList.is_null() {
        ipNext = (*ipList).next;
        _Mem_Free(ipList as *mut libc::c_void,
                  b"../engine/server/sv_filter.c\x00" as *const u8 as
                      *const libc::c_char, 451 as libc::c_int);
        ipList = ipNext
    }
    cidList = cidfilter;
    while !cidList.is_null() {
        cidNext = (*cidList).next;
        _Mem_Free(cidList as *mut libc::c_void,
                  b"../engine/server/sv_filter.c\x00" as *const u8 as
                      *const libc::c_char, 457 as libc::c_int);
        cidList = cidNext
    }
    cidfilter = 0 as *mut cidfilter_t;
    ipfilter = 0 as *mut ipfilter_t;
}
