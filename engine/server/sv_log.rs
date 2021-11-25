#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, const_raw_ptr_to_usize_cast, extern_types,
           register_tool)]
extern "C" {
    pub type file_s;
    #[no_mangle]
    fn Info_ValueForKey(s: *const libc::c_char, key: *const libc::c_char)
     -> *const libc::c_char;
    #[no_mangle]
    fn SV_Serverinfo() -> *mut libc::c_char;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn localtime(__timer: *const time_t) -> *mut tm;
    #[no_mangle]
    fn Q_atoi(str: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn Q_strchr(s: *const libc::c_char, c: libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn Q_strnicmp(s1: *const libc::c_char, s2: *const libc::c_char,
                  n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Q_strstr(string: *const libc::c_char, string2: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn Q_vsnprintf(buffer: *mut libc::c_char, buffersize: size_t,
                   format: *const libc::c_char, args: ::std::ffi::VaList)
     -> libc::c_int;
    #[no_mangle]
    fn Q_snprintf(buffer: *mut libc::c_char, buffersize: size_t,
                  format: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn Cvar_LookupVars(checkbit: libc::c_int, buffer: *mut libc::c_void,
                       ptr: *mut libc::c_void, callback: setpair_t);
    #[no_mangle]
    fn Cvar_VariableString(var_name: *const libc::c_char)
     -> *const libc::c_char;
    #[no_mangle]
    fn NET_AdrToString(a: netadr_t) -> *const libc::c_char;
    #[no_mangle]
    fn NET_StringToAdr(string: *const libc::c_char, adr: *mut netadr_t)
     -> qboolean;
    #[no_mangle]
    fn Cmd_Argc() -> libc::c_int;
    #[no_mangle]
    fn Cmd_Argv(arg: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn FS_Open(filepath: *const libc::c_char, mode: *const libc::c_char,
               gamedironly: qboolean) -> *mut file_t;
    #[no_mangle]
    fn FS_Printf(file: *mut file_t, format: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn FS_FileExists(filename: *const libc::c_char, gamedironly: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn FS_Close(file: *mut file_t) -> libc::c_int;
    #[no_mangle]
    fn Q_buildnum() -> libc::c_int;
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Netchan_OutOfBandPrint(net_socket: libc::c_int, adr: netadr_t,
                              format: *const libc::c_char, _: ...);
    #[no_mangle]
    static mut sv_log_singleplayer: convar_t;
    #[no_mangle]
    static mut mp_logecho: convar_t;
    #[no_mangle]
    static mut mp_logfile: convar_t;
    #[no_mangle]
    static mut svs: server_static_t;
    #[no_mangle]
    static mut sv_log_onefile: convar_t;
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
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type time_t = __time_t;
pub type uint = libc::c_uint;
pub type va_list = __builtin_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type byte = libc::c_uchar;
pub type vec_t = libc::c_float;
pub type vec3_t = [vec_t; 3];
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type string = [libc::c_char; 256];
pub type file_t = file_s;
pub type setpair_t
    =
    Option<unsafe extern "C" fn(_: *const libc::c_char,
                                _: *const libc::c_void, _: *mut libc::c_void,
                                _: *mut libc::c_void) -> ()>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct challenge_t {
    pub adr: netadr_t,
    pub time: libc::c_double,
    pub challenge: libc::c_int,
    pub connected: qboolean,
}
pub type sv_client_t = sv_client_s;
/*
sv_log.c - server logging in multiplayer
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
pub unsafe extern "C" fn Log_Open() {
    let mut ltime: time_t = 0;
    let mut today: *mut tm = 0 as *mut tm;
    let mut szFileBase: [libc::c_char; 260] = [0; 260];
    let mut szTestFile: [libc::c_char; 260] = [0; 260];
    let mut fp: *mut file_t = 0 as *mut file_t;
    let mut temp: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    if svs.log.active as u64 == 0 { return }
    if sv_log_onefile.value != 0. && !svs.log.file.is_null() { return }
    if mp_logfile.value == 0. {
        Con_Printf(b"Server logging data to console.\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    Log_Close();
    // Find a new log file slot
    time(&mut ltime);
    today = localtime(&mut ltime);
    temp =
        Cvar_VariableString(b"logsdir\x00" as *const u8 as
                                *const libc::c_char);
    if (if temp.is_null() || *temp == 0 {
            0 as libc::c_int
        } else { 1 as libc::c_int }) != 0 &&
           Q_strchr(temp, ':' as i32 as libc::c_char).is_null() &&
           Q_strstr(temp,
                    b"..\x00" as *const u8 as *const libc::c_char).is_null() {
        Q_snprintf(szFileBase.as_mut_ptr(),
                   ::std::mem::size_of::<[libc::c_char; 260]>() as
                       libc::c_ulong,
                   b"%s/L%02i%02i\x00" as *const u8 as *const libc::c_char,
                   temp, (*today).tm_mon + 1 as libc::c_int,
                   (*today).tm_mday);
    } else {
        Q_snprintf(szFileBase.as_mut_ptr(),
                   ::std::mem::size_of::<[libc::c_char; 260]>() as
                       libc::c_ulong,
                   b"logs/L%02i%02i\x00" as *const u8 as *const libc::c_char,
                   (*today).tm_mon + 1 as libc::c_int, (*today).tm_mday);
    }
    i = 0 as libc::c_int;
    while i < 1000 as libc::c_int {
        Q_snprintf(szTestFile.as_mut_ptr(),
                   ::std::mem::size_of::<[libc::c_char; 260]>() as
                       libc::c_ulong,
                   b"%s%03i.log\x00" as *const u8 as *const libc::c_char,
                   szFileBase.as_mut_ptr(), i);
        if FS_FileExists(szTestFile.as_mut_ptr(), false_0 as libc::c_int) != 0
           {
            i += 1
        } else {
            fp =
                FS_Open(szTestFile.as_mut_ptr(),
                        b"w\x00" as *const u8 as *const libc::c_char, true_0);
            if !fp.is_null() {
                Con_Printf(b"Server logging data to file %s\n\x00" as
                               *const u8 as *const libc::c_char,
                           szTestFile.as_mut_ptr());
            } else { i = 1000 as libc::c_int }
            break ;
        }
    }
    if i == 1000 as libc::c_int {
        Con_Printf(b"Unable to open logfiles under %s\nLogging disabled\n\x00"
                       as *const u8 as *const libc::c_char,
                   szFileBase.as_mut_ptr());
        svs.log.active = false_0;
        return
    }
    if !fp.is_null() { svs.log.file = fp }
    Log_Printf(b"Log file started (file \"%s\") (game \"%s\") (version \"%i/%s/%d\")\n\x00"
                   as *const u8 as *const libc::c_char,
               szTestFile.as_mut_ptr(),
               Info_ValueForKey(SV_Serverinfo(),
                                b"*gamedir\x00" as *const u8 as
                                    *const libc::c_char), 49 as libc::c_int,
               b"0.20\x00" as *const u8 as *const libc::c_char, Q_buildnum());
}
#[no_mangle]
pub unsafe extern "C" fn Log_Close() {
    if !svs.log.file.is_null() {
        Log_Printf(b"Log file closed\n\x00" as *const u8 as
                       *const libc::c_char);
        FS_Close(svs.log.file);
    }
    svs.log.file = 0 as *mut file_t;
}
/*
==================
Log_Printf

Prints a frag log message to the server's frag log file, console, and possible a UDP port.
==================
*/
#[no_mangle]
pub unsafe extern "C" fn Log_Printf(mut fmt: *const libc::c_char,
                                    mut args: ...) {
    let mut argptr: ::std::ffi::VaListImpl;
    static mut string: [libc::c_char; 1024] = [0; 1024];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ltime: time_t = 0;
    let mut today: *mut tm = 0 as *mut tm;
    let mut len: libc::c_int = 0;
    if svs.log.active as u64 == 0 { return }
    time(&mut ltime);
    today = localtime(&mut ltime);
    len =
        Q_snprintf(string.as_mut_ptr(),
                   ::std::mem::size_of::<[libc::c_char; 1024]>() as
                       libc::c_ulong,
                   b"%02i/%02i/%04i - %02i:%02i:%02i: \x00" as *const u8 as
                       *const libc::c_char,
                   (*today).tm_mon + 1 as libc::c_int, (*today).tm_mday,
                   1900 as libc::c_int + (*today).tm_year, (*today).tm_hour,
                   (*today).tm_min, (*today).tm_sec);
    p = string.as_mut_ptr().offset(len as isize);
    argptr = args.clone();
    Q_vsnprintf(p,
                (::std::mem::size_of::<[libc::c_char; 1024]>() as
                     libc::c_ulong).wrapping_sub(len as libc::c_ulong), fmt,
                argptr.as_va_list());
    if svs.log.net_log as u64 != 0 {
        Netchan_OutOfBandPrint(NS_SERVER as libc::c_int, svs.log.net_address,
                               b"log %s\x00" as *const u8 as
                                   *const libc::c_char, string.as_mut_ptr());
    }
    if svs.log.active as libc::c_uint != 0 &&
           (svs.maxclients > 1 as libc::c_int ||
                sv_log_singleplayer.value != 0.0f32) {
        // echo to server console
        if mp_logecho.value != 0. {
            Con_Printf(b"%s\x00" as *const u8 as *const libc::c_char,
                       string.as_mut_ptr());
        }
        // echo to log file
        if !svs.log.file.is_null() && mp_logfile.value != 0. {
            FS_Printf(svs.log.file,
                      b"%s\x00" as *const u8 as *const libc::c_char,
                      string.as_mut_ptr());
        }
    };
}
unsafe extern "C" fn Log_PrintServerCvar(mut var_name: *const libc::c_char,
                                         mut var_value: *const libc::c_char,
                                         mut unused2: *mut libc::c_void,
                                         mut unused3: *mut libc::c_void) {
    Log_Printf(b"Server cvar \"%s\" = \"%s\"\n\x00" as *const u8 as
                   *const libc::c_char, var_name, var_value);
}
/*
==================
Log_PrintServerVars

==================
*/
#[no_mangle]
pub unsafe extern "C" fn Log_PrintServerVars() {
    if svs.log.active as u64 == 0 { return }
    Log_Printf(b"Server cvars start\n\x00" as *const u8 as
                   *const libc::c_char);
    Cvar_LookupVars((1 as libc::c_int) << 2 as libc::c_int,
                    0 as *mut libc::c_void, 0 as *mut libc::c_void,
                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                            *const libc::c_char,
                                                                        _:
                                                                            *const libc::c_char,
                                                                        _:
                                                                            *mut libc::c_void,
                                                                        _:
                                                                            *mut libc::c_void)
                                                       -> ()>,
                                            setpair_t>(Some(Log_PrintServerCvar
                                                                as
                                                                unsafe extern "C" fn(_:
                                                                                         *const libc::c_char,
                                                                                     _:
                                                                                         *const libc::c_char,
                                                                                     _:
                                                                                         *mut libc::c_void,
                                                                                     _:
                                                                                         *mut libc::c_void)
                                                                    -> ())));
    Log_Printf(b"Server cvars end\n\x00" as *const u8 as *const libc::c_char);
}
/*
====================
SV_SetLogAddress_f

====================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_SetLogAddress_f() {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut port: libc::c_int = 0;
    let mut addr: string = [0; 256];
    if Cmd_Argc() != 3 as libc::c_int {
        Con_Printf(b"logaddress: usage\nlogaddress ip port\n\x00" as *const u8
                       as *const libc::c_char);
        if svs.log.active as u64 != 0 {
            Con_Printf(b"current: %s\n\x00" as *const u8 as
                           *const libc::c_char,
                       NET_AdrToString(svs.log.net_address));
        }
        return
    }
    port = Q_atoi(Cmd_Argv(2 as libc::c_int));
    if port == 0 {
        Con_Printf(b"logaddress: must specify a valid port\n\x00" as *const u8
                       as *const libc::c_char);
        return
    }
    s = Cmd_Argv(1 as libc::c_int);
    if if s.is_null() || *s == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        Con_Printf(b"logaddress: unparseable address\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    Q_snprintf(addr.as_mut_ptr(),
               ::std::mem::size_of::<string>() as libc::c_ulong,
               b"%s:%i\x00" as *const u8 as *const libc::c_char, s, port);
    if NET_StringToAdr(addr.as_mut_ptr(), &mut svs.log.net_address) as u64 ==
           0 {
        Con_Printf(b"logaddress: unable to resolve %s\n\x00" as *const u8 as
                       *const libc::c_char, addr.as_mut_ptr());
        return
    }
    svs.log.net_log = true_0;
    Con_Printf(b"logaddress: %s\n\x00" as *const u8 as *const libc::c_char,
               NET_AdrToString(svs.log.net_address));
}
/*
====================
SV_ServerLog_f

====================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_ServerLog_f() {
    if Cmd_Argc() != 2 as libc::c_int {
        Con_Printf(b"usage: log < on|off >\n\x00" as *const u8 as
                       *const libc::c_char);
        if svs.log.active as u64 != 0 {
            Con_Printf(b"currently logging\n\x00" as *const u8 as
                           *const libc::c_char);
        } else {
            Con_Printf(b"not currently logging\n\x00" as *const u8 as
                           *const libc::c_char);
        }
        return
    }
    if Q_strnicmp(Cmd_Argv(1 as libc::c_int),
                  b"off\x00" as *const u8 as *const libc::c_char,
                  99999 as libc::c_int) == 0 {
        if svs.log.active as u64 != 0 { Log_Close(); }
    } else if Q_strnicmp(Cmd_Argv(1 as libc::c_int),
                         b"on\x00" as *const u8 as *const libc::c_char,
                         99999 as libc::c_int) == 0 {
        svs.log.active = true_0;
        Log_Open();
    } else {
        Con_Printf(b"log: unknown parameter %s\n\x00" as *const u8 as
                       *const libc::c_char, Cmd_Argv(1 as libc::c_int));
    };
}
