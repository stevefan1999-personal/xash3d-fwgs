#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type file_s;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn COM_RandomLong(lMin: libc::c_int, lMax: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Q_atof(str: *const libc::c_char) -> libc::c_float;
    #[no_mangle]
    fn Sys_DoubleTime() -> libc::c_double;
    #[no_mangle]
    fn Cvar_Get(var_name: *const libc::c_char, value: *const libc::c_char,
                flags: libc::c_int, description: *const libc::c_char)
     -> *mut convar_t;
    #[no_mangle]
    fn Cvar_SetValue(var_name: *const libc::c_char, value: libc::c_float);
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
    fn Cmd_RemoveCommand(cmd_name: *const libc::c_char);
    #[no_mangle]
    fn _Mem_Alloc(poolptr: poolhandle_t, size: size_t, clear: qboolean,
                  filename: *const libc::c_char, fileline: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    fn _Mem_Free(data: *mut libc::c_void, filename: *const libc::c_char,
                 fileline: libc::c_int);
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    static mut cls: client_static_t;
    #[no_mangle]
    static mut s_listener: listener_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct color24 {
    pub r: byte,
    pub g: byte,
    pub b: byte,
}
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
pub type keydest_t = libc::c_uint;
pub const key_message: keydest_t = 3;
pub const key_menu: keydest_t = 2;
pub const key_game: keydest_t = 1;
pub const key_console: keydest_t = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sizebuf_s {
    pub bOverflow: qboolean,
    pub pDebugName: *const libc::c_char,
    pub pData: *mut byte,
    pub iCurBit: libc::c_int,
    pub nDataBits: libc::c_int,
}
pub type sizebuf_t = sizebuf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wrect_s {
    pub left: libc::c_int,
    pub right: libc::c_int,
    pub top: libc::c_int,
    pub bottom: libc::c_int,
}
pub type wrect_t = wrect_s;
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
pub type clientdata_t = clientdata_s;
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
pub type weapon_data_t = weapon_data_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct local_state_s {
    pub playerstate: entity_state_t,
    pub client: clientdata_t,
    pub weapondata: [weapon_data_t; 64],
}
pub type local_state_t = local_state_s;
pub type connstate_e = libc::c_uint;
pub const ca_cinematic: connstate_e = 5;
pub const ca_active: connstate_e = 4;
pub const ca_validate: connstate_e = 3;
pub const ca_connected: connstate_e = 2;
pub const ca_connecting: connstate_e = 1;
pub const ca_disconnected: connstate_e = 0;
pub type connstate_t = connstate_e;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct netsplit_chain_packet_s {
    pub recieved_v: [uint32_t; 8],
    pub id: uint32_t,
    pub data: [byte; 131072],
    pub received: byte,
    pub count: byte,
}
pub type netsplit_chain_packet_t = netsplit_chain_packet_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct netsplit_s {
    pub packets: [netsplit_chain_packet_t; 8],
    pub total_received: uint64_t,
    pub total_received_uncompressed: uint64_t,
}
pub type netsplit_t = netsplit_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flowstats_t {
    pub size: libc::c_int,
    pub time: libc::c_double,
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
pub type fragbuf_t = fragbuf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fbufqueue_s {
    pub next: *mut fbufqueue_s,
    pub fragbufcount: libc::c_int,
    pub fragbufs: *mut fragbuf_t,
}
pub type fragbufwaiting_t = fbufqueue_s;
pub type fragsize_e = libc::c_uint;
pub const FRAGSIZE_UNRELIABLE: fragsize_e = 2;
pub const FRAGSIZE_SPLIT: fragsize_e = 1;
pub const FRAGSIZE_FRAG: fragsize_e = 0;
pub type fragsize_t = fragsize_e;
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
pub type netchan_t = netchan_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct downloadtime_t {
    pub bUsed: qboolean,
    pub fTime: libc::c_float,
    pub nBytesRemaining: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct incomingtransfer_t {
    pub doneregistering: qboolean,
    pub percent: libc::c_int,
    pub downloadrequested: qboolean,
    pub rgStats: [downloadtime_t; 8],
    pub nCurStat: libc::c_int,
    pub nTotalSize: libc::c_int,
    pub nTotalToTransfer: libc::c_int,
    pub nRemainingToTransfer: libc::c_int,
    pub fLastStatusUpdate: libc::c_float,
    pub custom: qboolean,
}
pub type scrshot_t = libc::c_uint;
pub const scrshot_mapshot: scrshot_t = 7;
pub const scrshot_skyshot: scrshot_t = 6;
pub const scrshot_envshot: scrshot_t = 5;
pub const scrshot_savegame: scrshot_t = 4;
pub const scrshot_plaque: scrshot_t = 3;
pub const scrshot_snapshot: scrshot_t = 2;
pub const scrshot_normal: scrshot_t = 1;
pub const scrshot_inactive: scrshot_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cl_font_t {
    pub hFontTexture: libc::c_int,
    pub fontRc: [wrect_t; 256],
    pub charWidths: [byte; 256],
    pub charHeight: libc::c_int,
    pub type_0: libc::c_int,
    pub valid: qboolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cl_predicted_player_s {
    pub movetype: libc::c_int,
    pub solid: libc::c_int,
    pub usehull: libc::c_int,
    pub active: qboolean,
    pub origin: vec3_t,
    pub angles: vec3_t,
}
pub type predicted_player_t = cl_predicted_player_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client_static_t {
    pub state: connstate_t,
    pub initialized: qboolean,
    pub changelevel: qboolean,
    pub changedemo: qboolean,
    pub timestart: libc::c_double,
    pub disable_screen: libc::c_float,
    pub disable_servercount: libc::c_int,
    pub draw_changelevel: qboolean,
    pub key_dest: keydest_t,
    pub mempool: poolhandle_t,
    pub hltv_listen_address: netadr_t,
    pub signon: libc::c_int,
    pub quakePort: libc::c_int,
    pub servername: [libc::c_char; 64],
    pub connect_time: libc::c_double,
    pub max_fragment_size: libc::c_int,
    pub connect_retry: libc::c_int,
    pub spectator: qboolean,
    pub spectator_state: local_state_t,
    pub userinfo: [libc::c_char; 256],
    pub physinfo: [libc::c_char; 256],
    pub datagram: sizebuf_t,
    pub datagram_buf: [byte; 16384],
    pub netchan: netchan_t,
    pub challenge: libc::c_int,
    pub packet_loss: libc::c_float,
    pub packet_loss_recalc_time: libc::c_double,
    pub starting_count: libc::c_int,
    pub nextcmdtime: libc::c_float,
    pub lastoutgoingcommand: libc::c_int,
    pub lastupdate_sequence: libc::c_int,
    pub td_lastframe: libc::c_int,
    pub td_startframe: libc::c_int,
    pub td_starttime: libc::c_double,
    pub forcetrack: libc::c_int,
    pub pauseIcon: libc::c_int,
    pub tileImage: libc::c_int,
    pub loadingBar: libc::c_int,
    pub creditsFont: cl_font_t,
    pub latency: libc::c_float,
    pub num_client_entities: libc::c_int,
    pub next_client_entities: libc::c_int,
    pub packet_entities: *mut entity_state_t,
    pub predicted_players: [predicted_player_t; 32],
    pub correction_time: libc::c_double,
    pub scrshot_request: scrshot_t,
    pub scrshot_action: scrshot_t,
    pub envshot_vieworg: *const libc::c_float,
    pub envshot_viewsize: libc::c_int,
    pub envshot_disable_vis: qboolean,
    pub shotname: string,
    pub dl: incomingtransfer_t,
    pub demonum: libc::c_int,
    pub olddemonum: libc::c_int,
    pub demos: [[libc::c_char; 64]; 32],
    pub demos_pending: qboolean,
    pub movienum: libc::c_int,
    pub movies: [string; 8],
    pub demorecording: qboolean,
    pub demoplayback: libc::c_int,
    pub demowaiting: qboolean,
    pub timedemo: qboolean,
    pub demoname: string,
    pub demotime: libc::c_double,
    pub set_lastdemo: qboolean,
    pub demofile: *mut file_t,
    pub demoheader: *mut file_t,
    pub internetservers_wait: qboolean,
    pub internetservers_pending: qboolean,
    pub legacymode: qboolean,
    pub legacyserver: netadr_t,
    pub legacyservers: [netadr_t; 256],
    pub legacyservercount: libc::c_int,
    pub extensions: libc::c_int,
    pub serveradr: netadr_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct portable_samplepair_t {
    pub left: libc::c_int,
    pub right: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct listener_t {
    pub origin: vec3_t,
    pub velocity: vec3_t,
    pub forward: vec3_t,
    pub right: vec3_t,
    pub up: vec3_t,
    pub entnum: libc::c_int,
    pub waterlevel: libc::c_int,
    pub frametime: libc::c_float,
    pub active: qboolean,
    pub inmenu: qboolean,
    pub paused: qboolean,
    pub streaming: qboolean,
    pub stream_paused: qboolean,
}
pub type dly_t = dly_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dly_s {
    pub cdelaysamplesmax: size_t,
    pub idelayinput: size_t,
    pub idelayoutput: size_t,
    pub idelayoutputxf: libc::c_int,
    pub xfade: libc::c_int,
    pub delaysamples: libc::c_int,
    pub delayfeedback: libc::c_int,
    pub lp: libc::c_int,
    pub lp0: libc::c_int,
    pub lp1: libc::c_int,
    pub lp2: libc::c_int,
    pub mod_0: libc::c_int,
    pub modcur: libc::c_int,
    pub lpdelayline: *mut libc::c_int,
}
pub type sx_preset_t = sx_preset_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sx_preset_s {
    pub room_lp: libc::c_float,
    pub room_mod: libc::c_float,
    pub room_size: libc::c_float,
    pub room_refl: libc::c_float,
    pub room_rvblp: libc::c_float,
    pub room_delay: libc::c_float,
    pub room_feedback: libc::c_float,
    pub room_dlylp: libc::c_float,
    pub room_left: libc::c_float,
}
#[no_mangle]
pub static mut rgsxpre: [sx_preset_t; 29] =
    [{
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.0f64 as libc::c_float,
                         room_refl: 0.0f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.0f64 as libc::c_float,
                         room_feedback: 0.0f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.0f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.0f64 as libc::c_float,
                         room_refl: 0.0f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.065f64 as libc::c_float,
                         room_feedback: 0.1f64 as libc::c_float,
                         room_dlylp: 0.0f64 as libc::c_float,
                         room_left: 0.01f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.0f64 as libc::c_float,
                         room_refl: 0.0f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.02f64 as libc::c_float,
                         room_feedback: 0.75f64 as libc::c_float,
                         room_dlylp: 0.0f64 as libc::c_float,
                         room_left: 0.01f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.0f64 as libc::c_float,
                         room_refl: 0.0f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.03f64 as libc::c_float,
                         room_feedback: 0.78f64 as libc::c_float,
                         room_dlylp: 0.0f64 as libc::c_float,
                         room_left: 0.02f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.0f64 as libc::c_float,
                         room_refl: 0.0f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.06f64 as libc::c_float,
                         room_feedback: 0.77f64 as libc::c_float,
                         room_dlylp: 0.0f64 as libc::c_float,
                         room_left: 0.03f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.05f64 as libc::c_float,
                         room_refl: 0.85f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.008f64 as libc::c_float,
                         room_feedback: 0.96f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.01f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.05f64 as libc::c_float,
                         room_refl: 0.88f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.01f64 as libc::c_float,
                         room_feedback: 0.98f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.02f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.05f64 as libc::c_float,
                         room_refl: 0.92f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.015f64 as libc::c_float,
                         room_feedback: 0.995f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.04f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.05f64 as libc::c_float,
                         room_refl: 0.84f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.0f64 as libc::c_float,
                         room_feedback: 0.0f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.012f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.05f64 as libc::c_float,
                         room_refl: 0.9f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.0f64 as libc::c_float,
                         room_feedback: 0.0f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.008f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.05f64 as libc::c_float,
                         room_refl: 0.95f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.0f64 as libc::c_float,
                         room_feedback: 0.0f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.004f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.05f64 as libc::c_float,
                         room_refl: 0.7f64 as libc::c_float,
                         room_rvblp: 0.0f64 as libc::c_float,
                         room_delay: 0.0f64 as libc::c_float,
                         room_feedback: 0.0f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.012f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.055f64 as libc::c_float,
                         room_refl: 0.78f64 as libc::c_float,
                         room_rvblp: 0.0f64 as libc::c_float,
                         room_delay: 0.0f64 as libc::c_float,
                         room_feedback: 0.0f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.008f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.05f64 as libc::c_float,
                         room_refl: 0.86f64 as libc::c_float,
                         room_rvblp: 0.0f64 as libc::c_float,
                         room_delay: 0.0f64 as libc::c_float,
                         room_feedback: 0.0f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.002f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 1.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.0f64 as libc::c_float,
                         room_refl: 0.0f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.0f64 as libc::c_float,
                         room_feedback: 0.0f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.01f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 1.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.0f64 as libc::c_float,
                         room_refl: 0.0f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.06f64 as libc::c_float,
                         room_feedback: 0.85f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.02f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 1.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.0f64 as libc::c_float,
                         room_refl: 0.0f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.2f64 as libc::c_float,
                         room_feedback: 0.6f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.05f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.05f64 as libc::c_float,
                         room_refl: 0.8f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.0f64 as libc::c_float,
                         room_feedback: 0.48f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.016f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.06f64 as libc::c_float,
                         room_refl: 0.9f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.0f64 as libc::c_float,
                         room_feedback: 0.52f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.01f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.07f64 as libc::c_float,
                         room_refl: 0.94f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.3f64 as libc::c_float,
                         room_feedback: 0.6f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.008f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.0f64 as libc::c_float,
                         room_refl: 0.0f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.3f64 as libc::c_float,
                         room_feedback: 0.42f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.0f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.0f64 as libc::c_float,
                         room_refl: 0.0f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.35f64 as libc::c_float,
                         room_feedback: 0.48f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.0f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.0f64 as libc::c_float,
                         room_refl: 0.0f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.38f64 as libc::c_float,
                         room_feedback: 0.6f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.0f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.05f64 as libc::c_float,
                         room_refl: 0.9f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.2f64 as libc::c_float,
                         room_feedback: 0.28f64 as libc::c_float,
                         room_dlylp: 0.0f64 as libc::c_float,
                         room_left: 0.0f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.07f64 as libc::c_float,
                         room_refl: 0.9f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.3f64 as libc::c_float,
                         room_feedback: 0.4f64 as libc::c_float,
                         room_dlylp: 0.0f64 as libc::c_float,
                         room_left: 0.0f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.09f64 as libc::c_float,
                         room_refl: 0.9f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.35f64 as libc::c_float,
                         room_feedback: 0.5f64 as libc::c_float,
                         room_dlylp: 0.0f64 as libc::c_float,
                         room_left: 0.0f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 1.0f64 as libc::c_float,
                         room_size: 0.01f64 as libc::c_float,
                         room_refl: 0.9f64 as libc::c_float,
                         room_rvblp: 0.0f64 as libc::c_float,
                         room_delay: 0.0f64 as libc::c_float,
                         room_feedback: 0.0f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.05f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.0f64 as libc::c_float,
                         room_refl: 0.0f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.009f64 as libc::c_float,
                         room_feedback: 0.999f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.04f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.001f64 as libc::c_float,
                         room_refl: 0.999f64 as libc::c_float,
                         room_rvblp: 0.0f64 as libc::c_float,
                         room_delay: 0.2f64 as libc::c_float,
                         room_feedback: 0.8f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.05f64 as libc::c_float,};
         init
     }];
// delay line array size
// delay line pointers
// crossfade
// output pointer
// value
// delay setting
// feedback setting
// lowpass
// is lowpass enabled
// lowpass buffer
// modulation
// delay line
// lowpass
// modulation
// reverb
// delay
// 0x0045dca8 enginegl.exe
// SHA256: 42383d32cd712e59ee2c1bd78b7ba48814e680e7026c4223e730111f34a60d66
#[no_mangle]
pub static mut rgsxpre_hlalpha052: [sx_preset_t; 29] =
    [{
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.0f64 as libc::c_float,
                         room_refl: 0.0f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.0f64 as libc::c_float,
                         room_feedback: 0.0f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.0f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.0f64 as libc::c_float,
                         room_refl: 0.0f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.08f64 as libc::c_float,
                         room_feedback: 0.8f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.0f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.0f64 as libc::c_float,
                         room_refl: 0.0f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.02f64 as libc::c_float,
                         room_feedback: 0.75f64 as libc::c_float,
                         room_dlylp: 0.0f64 as libc::c_float,
                         room_left: 0.001f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.0f64 as libc::c_float,
                         room_refl: 0.0f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.03f64 as libc::c_float,
                         room_feedback: 0.78f64 as libc::c_float,
                         room_dlylp: 0.0f64 as libc::c_float,
                         room_left: 0.002f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.0f64 as libc::c_float,
                         room_refl: 0.0f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.06f64 as libc::c_float,
                         room_feedback: 0.77f64 as libc::c_float,
                         room_dlylp: 0.0f64 as libc::c_float,
                         room_left: 0.003f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.05f64 as libc::c_float,
                         room_refl: 0.85f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.008f64 as libc::c_float,
                         room_feedback: 0.96f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.01f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.05f64 as libc::c_float,
                         room_refl: 0.88f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.01f64 as libc::c_float,
                         room_feedback: 0.98f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.02f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.05f64 as libc::c_float,
                         room_refl: 0.92f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.015f64 as libc::c_float,
                         room_feedback: 0.995f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.04f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.05f64 as libc::c_float,
                         room_refl: 0.84f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.0f64 as libc::c_float,
                         room_feedback: 0.0f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.003f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.05f64 as libc::c_float,
                         room_refl: 0.9f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.0f64 as libc::c_float,
                         room_feedback: 0.0f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.002f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.05f64 as libc::c_float,
                         room_refl: 0.95f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.0f64 as libc::c_float,
                         room_feedback: 0.0f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.001f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.05f64 as libc::c_float,
                         room_refl: 0.7f64 as libc::c_float,
                         room_rvblp: 0.0f64 as libc::c_float,
                         room_delay: 0.0f64 as libc::c_float,
                         room_feedback: 0.0f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.003f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.055f64 as libc::c_float,
                         room_refl: 0.78f64 as libc::c_float,
                         room_rvblp: 0.0f64 as libc::c_float,
                         room_delay: 0.0f64 as libc::c_float,
                         room_feedback: 0.0f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.002f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.05f64 as libc::c_float,
                         room_refl: 0.86f64 as libc::c_float,
                         room_rvblp: 0.0f64 as libc::c_float,
                         room_delay: 0.0f64 as libc::c_float,
                         room_feedback: 0.0f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.001f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 1.0f64 as libc::c_float,
                         room_mod: 1.0f64 as libc::c_float,
                         room_size: 0.0f64 as libc::c_float,
                         room_refl: 0.0f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.0f64 as libc::c_float,
                         room_feedback: 0.0f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.01f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 1.0f64 as libc::c_float,
                         room_mod: 1.0f64 as libc::c_float,
                         room_size: 0.0f64 as libc::c_float,
                         room_refl: 0.0f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.06f64 as libc::c_float,
                         room_feedback: 0.85f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.02f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 1.0f64 as libc::c_float,
                         room_mod: 1.0f64 as libc::c_float,
                         room_size: 0.0f64 as libc::c_float,
                         room_refl: 0.0f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.2f64 as libc::c_float,
                         room_feedback: 0.6f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.05f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.05f64 as libc::c_float,
                         room_refl: 0.8f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.15f64 as libc::c_float,
                         room_feedback: 0.48f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.008f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.06f64 as libc::c_float,
                         room_refl: 0.9f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.22f64 as libc::c_float,
                         room_feedback: 0.52f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.005f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.07f64 as libc::c_float,
                         room_refl: 0.94f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.3f64 as libc::c_float,
                         room_feedback: 0.6f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.001f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.0f64 as libc::c_float,
                         room_refl: 0.0f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.3f64 as libc::c_float,
                         room_feedback: 0.42f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.0f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.0f64 as libc::c_float,
                         room_refl: 0.0f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.35f64 as libc::c_float,
                         room_feedback: 0.48f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.0f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.0f64 as libc::c_float,
                         room_refl: 0.0f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.38f64 as libc::c_float,
                         room_feedback: 0.6f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.0f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.05f64 as libc::c_float,
                         room_refl: 0.9f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.2f64 as libc::c_float,
                         room_feedback: 0.28f64 as libc::c_float,
                         room_dlylp: 0.0f64 as libc::c_float,
                         room_left: 0.0f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.07f64 as libc::c_float,
                         room_refl: 0.9f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.3f64 as libc::c_float,
                         room_feedback: 0.4f64 as libc::c_float,
                         room_dlylp: 0.0f64 as libc::c_float,
                         room_left: 0.0f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.09f64 as libc::c_float,
                         room_refl: 0.9f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.35f64 as libc::c_float,
                         room_feedback: 0.5f64 as libc::c_float,
                         room_dlylp: 0.0f64 as libc::c_float,
                         room_left: 0.0f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 1.0f64 as libc::c_float,
                         room_size: 0.01f64 as libc::c_float,
                         room_refl: 0.9f64 as libc::c_float,
                         room_rvblp: 0.0f64 as libc::c_float,
                         room_delay: 0.0f64 as libc::c_float,
                         room_feedback: 0.0f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.05f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.0f64 as libc::c_float,
                         room_refl: 0.0f64 as libc::c_float,
                         room_rvblp: 1.0f64 as libc::c_float,
                         room_delay: 0.009f64 as libc::c_float,
                         room_feedback: 0.999f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.04f64 as libc::c_float,};
         init
     },
     {
         let mut init =
             sx_preset_s{room_lp: 0.0f64 as libc::c_float,
                         room_mod: 0.0f64 as libc::c_float,
                         room_size: 0.001f64 as libc::c_float,
                         room_refl: 0.999f64 as libc::c_float,
                         room_rvblp: 0.0f64 as libc::c_float,
                         room_delay: 0.2f64 as libc::c_float,
                         room_feedback: 0.8f64 as libc::c_float,
                         room_dlylp: 2.0f64 as libc::c_float,
                         room_left: 0.05f64 as libc::c_float,};
         init
     }];
#[no_mangle]
pub static mut ptable: *const sx_preset_t = unsafe { rgsxpre.as_ptr() };
// cvars
#[no_mangle]
pub static mut dsp_off: *mut convar_t = 0 as *const convar_t as *mut convar_t;
// disable dsp
#[no_mangle]
pub static mut roomwater_type: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
// water room_type
#[no_mangle]
pub static mut room_type: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
// current room type
#[no_mangle]
pub static mut hisound: *mut convar_t = 0 as *const convar_t as *mut convar_t;
// DSP quality
// underwater/special fx modulations
#[no_mangle]
pub static mut sxmod_mod: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut sxmod_lowpass: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
// stereo delay(no feedback)
#[no_mangle]
pub static mut sxste_delay: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
// straight left delay
// mono reverb
#[no_mangle]
pub static mut sxrvb_lp: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
// lowpass
#[no_mangle]
pub static mut sxrvb_feedback: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
// reverb decay. Higher -- longer
#[no_mangle]
pub static mut sxrvb_size: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
// room size. Higher -- larger
// mono delay
#[no_mangle]
pub static mut sxdly_lp: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
// lowpass
#[no_mangle]
pub static mut sxdly_feedback: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
// cycles
#[no_mangle]
pub static mut sxdly_delay: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
// current delay in seconds
#[no_mangle]
pub static mut dsp_room: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
// for compability
#[no_mangle]
pub static mut dsp_coeff_table: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
// use release or 0.52 style
#[no_mangle]
pub static mut idsp_dma_speed: libc::c_int = 0;
#[no_mangle]
pub static mut idsp_room: libc::c_int = 0;
#[no_mangle]
pub static mut room_typeprev: libc::c_int = 0;
// routines
#[no_mangle]
pub static mut sxamodl: libc::c_int = 0;
#[no_mangle]
pub static mut sxamodr: libc::c_int = 0;
// amplitude modulation values
#[no_mangle]
pub static mut sxamodlt: libc::c_int = 0;
#[no_mangle]
pub static mut sxamodrt: libc::c_int = 0;
// modulation targets
#[no_mangle]
pub static mut sxmod1cur: libc::c_int = 0;
#[no_mangle]
pub static mut sxmod2cur: libc::c_int = 0;
#[no_mangle]
pub static mut sxmod1: libc::c_int = 0;
#[no_mangle]
pub static mut sxmod2: libc::c_int = 0;
#[no_mangle]
pub static mut sxhires: libc::c_int = 0;
#[no_mangle]
pub static mut paintto: *mut portable_samplepair_t =
    0 as *const portable_samplepair_t as *mut portable_samplepair_t;
#[no_mangle]
pub static mut rgsxdly: [dly_t; 4] =
    [dly_t{cdelaysamplesmax: 0,
           idelayinput: 0,
           idelayoutput: 0,
           idelayoutputxf: 0,
           xfade: 0,
           delaysamples: 0,
           delayfeedback: 0,
           lp: 0,
           lp0: 0,
           lp1: 0,
           lp2: 0,
           mod_0: 0,
           modcur: 0,
           lpdelayline: 0 as *const libc::c_int as *mut libc::c_int,}; 4];
// stereo is last
#[no_mangle]
pub static mut rgsxlp: [libc::c_int; 10] = [0; 10];
/*
============
SX_ReloadRoomFX

============
*/
#[no_mangle]
pub unsafe extern "C" fn SX_ReloadRoomFX() {
    if dsp_room.is_null() { return } // not initialized
    (*sxste_delay).flags =
        (*sxste_delay).flags | (1 as libc::c_int) << 13 as libc::c_int;
    (*sxrvb_feedback).flags =
        (*sxrvb_feedback).flags | (1 as libc::c_int) << 13 as libc::c_int;
    (*sxdly_delay).flags =
        (*sxdly_delay).flags | (1 as libc::c_int) << 13 as libc::c_int;
    (*room_type).flags =
        (*room_type).flags | (1 as libc::c_int) << 13 as libc::c_int;
}
/*
============
SX_Init()

Starts sound crackling system
============
*/
#[no_mangle]
pub unsafe extern "C" fn SX_Init() {
    memset(rgsxdly.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[dly_t; 4]>() as libc::c_ulong);
    memset(rgsxlp.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[libc::c_int; 10]>() as libc::c_ulong);
    sxamodlt = 255 as libc::c_int;
    sxamodrt = sxamodlt;
    sxamodl = sxamodrt;
    sxamodr = sxamodl;
    idsp_dma_speed = 11025 as libc::c_int;
    hisound =
        Cvar_Get(b"room_hires\x00" as *const u8 as *const libc::c_char,
                 b"2\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int,
                 b"dsp quality. 1 for 22k, 2 for 44k(recommended) and 3 for 96k\x00"
                     as *const u8 as *const libc::c_char);
    sxhires = 2 as libc::c_int;
    sxmod1 = 350 as libc::c_int * (idsp_dma_speed / 11025 as libc::c_int);
    sxmod1cur = sxmod1;
    sxmod2 = 450 as libc::c_int * (idsp_dma_speed / 11025 as libc::c_int);
    sxmod2cur = sxmod2;
    dsp_off =
        Cvar_Get(b"dsp_off\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int,
                 b"disable DSP processing\x00" as *const u8 as
                     *const libc::c_char);
    dsp_coeff_table =
        Cvar_Get(b"dsp_coeff_table\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int,
                 b"select DSP coefficient table: 0 for release or 1 for alpha 0.52\x00"
                     as *const u8 as *const libc::c_char);
    roomwater_type =
        Cvar_Get(b"waterroom_type\x00" as *const u8 as *const libc::c_char,
                 b"14\x00" as *const u8 as *const libc::c_char,
                 0 as libc::c_int,
                 b"water room type\x00" as *const u8 as *const libc::c_char);
    room_type =
        Cvar_Get(b"room_type\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 0 as libc::c_int,
                 b"current room type preset\x00" as *const u8 as
                     *const libc::c_char);
    sxmod_lowpass =
        Cvar_Get(b"room_lp\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 0 as libc::c_int,
                 b"for water fx, lowpass for entire room\x00" as *const u8 as
                     *const libc::c_char);
    sxmod_mod =
        Cvar_Get(b"room_mod\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 0 as libc::c_int,
                 b"stereo amptitude modulation for room\x00" as *const u8 as
                     *const libc::c_char);
    sxrvb_size =
        Cvar_Get(b"room_size\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 0 as libc::c_int,
                 b"reverb: initial reflection size\x00" as *const u8 as
                     *const libc::c_char);
    sxrvb_feedback =
        Cvar_Get(b"room_refl\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 0 as libc::c_int,
                 b"reverb: decay time\x00" as *const u8 as
                     *const libc::c_char);
    sxrvb_lp =
        Cvar_Get(b"room_rvblp\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char,
                 0 as libc::c_int,
                 b"reverb: low pass filtering level\x00" as *const u8 as
                     *const libc::c_char);
    sxdly_delay =
        Cvar_Get(b"room_delay\x00" as *const u8 as *const libc::c_char,
                 b"0.8\x00" as *const u8 as *const libc::c_char,
                 0 as libc::c_int,
                 b"mono delay: delay time\x00" as *const u8 as
                     *const libc::c_char);
    sxdly_feedback =
        Cvar_Get(b"room_feedback\x00" as *const u8 as *const libc::c_char,
                 b"0.2\x00" as *const u8 as *const libc::c_char,
                 0 as libc::c_int,
                 b"mono delay: decay time\x00" as *const u8 as
                     *const libc::c_char);
    sxdly_lp =
        Cvar_Get(b"room_dlylp\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char,
                 0 as libc::c_int,
                 b"mono delay: low pass filtering level\x00" as *const u8 as
                     *const libc::c_char);
    sxste_delay =
        Cvar_Get(b"room_left\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 0 as libc::c_int,
                 b"left channel delay time\x00" as *const u8 as
                     *const libc::c_char);
    Cmd_AddCommand(b"dsp_profile\x00" as *const u8 as *const libc::c_char,
                   Some(SX_Profiling_f as unsafe extern "C" fn() -> ()),
                   b"dsp stress-test, first argument is room_type\x00" as
                       *const u8 as *const libc::c_char);
    // for compability
    dsp_room = room_type;
    SX_ReloadRoomFX();
}
/*
===========
DLY_Free

Free memory allocated for DSP
===========
*/
#[no_mangle]
pub unsafe extern "C" fn DLY_Free(mut idelay: libc::c_int) {
    if !rgsxdly[idelay as usize].lpdelayline.is_null() {
        if !rgsxdly[idelay as usize].lpdelayline.is_null() {
            _Mem_Free(rgsxdly[idelay as usize].lpdelayline as
                          *mut libc::c_void,
                      b"../engine/client/s_dsp.c\x00" as *const u8 as
                          *const libc::c_char, 277 as libc::c_int);
        }
        rgsxdly[idelay as usize].lpdelayline = 0 as *mut libc::c_int
    };
}
/*
==========
SX_Shutdown

Stop DSP processor
==========
*/
#[no_mangle]
pub unsafe extern "C" fn SX_Free() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i <= 3 as libc::c_int { DLY_Free(i); i += 1 }
    Cmd_RemoveCommand(b"dsp_profile\x00" as *const u8 as *const libc::c_char);
}
/*
===========
DLY_Init

Initialize dly
===========
*/
#[no_mangle]
pub unsafe extern "C" fn DLY_Init(mut idelay: libc::c_int,
                                  mut delay: libc::c_float) -> libc::c_int {
    let mut cur: *mut dly_t = 0 as *mut dly_t;
    // DLY_Init called anytime with constants. So valid it in debug builds only.
    DLY_Free(idelay); // free dly if it's allocated
    cur = &mut *rgsxdly.as_mut_ptr().offset(idelay as isize) as *mut dly_t;
    (*cur).cdelaysamplesmax =
        ((((delay * idsp_dma_speed as libc::c_float) as libc::c_int) <<
              sxhires) + 1 as libc::c_int) as size_t;
    (*cur).lpdelayline =
        _Mem_Alloc(host.mempool,
                   (*cur).cdelaysamplesmax.wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                            as libc::c_ulong),
                   true_0,
                   b"../engine/client/s_dsp.c\x00" as *const u8 as
                       *const libc::c_char, 319 as libc::c_int) as
            *mut libc::c_int;
    (*cur).xfade = 0 as libc::c_int;
    // init modulation
    (*cur).modcur = 0 as libc::c_int;
    (*cur).mod_0 = (*cur).modcur;
    // init lowpass
    (*cur).lp = 1 as libc::c_int; // NOTE: delaysamples must be set!!!
    (*cur).lp2 = 0 as libc::c_int;
    (*cur).lp1 = (*cur).lp2;
    (*cur).lp0 = (*cur).lp1;
    (*cur).idelayinput = 0 as libc::c_int as size_t;
    (*cur).idelayoutput =
        (*cur).cdelaysamplesmax.wrapping_sub((*cur).delaysamples as
                                                 libc::c_ulong);
    return 1 as libc::c_int;
}
/*
============
DLY_MovePointer

Checks overflow and moves pointer
============
*/
#[inline]
unsafe extern "C" fn DLY_MovePointer(mut dly: *mut dly_t) {
    (*dly).idelayinput = (*dly).idelayinput.wrapping_add(1);
    if (*dly).idelayinput >= (*dly).cdelaysamplesmax {
        (*dly).idelayinput = 0 as libc::c_int as size_t
    }
    (*dly).idelayoutput = (*dly).idelayoutput.wrapping_add(1);
    if (*dly).idelayoutput >= (*dly).cdelaysamplesmax {
        (*dly).idelayoutput = 0 as libc::c_int as size_t
    };
}
/*
=============
DLY_CheckNewStereoDelayVal

Update stereo processor settings if we are in new room
=============
*/
#[no_mangle]
pub unsafe extern "C" fn DLY_CheckNewStereoDelayVal() {
    let dly: *mut dly_t =
        &mut *rgsxdly.as_mut_ptr().offset(3 as libc::c_int as isize) as
            *mut dly_t;
    let mut delay: libc::c_float = (*sxste_delay).value;
    if (*sxste_delay).flags & (1 as libc::c_int) << 13 as libc::c_int == 0 {
        return
    }
    if delay == 0 as libc::c_int as libc::c_float {
        DLY_Free(3 as libc::c_int);
    } else {
        let mut samples: libc::c_int = 0;
        delay = if delay < 0.1f32 { delay } else { 0.1f32 };
        samples =
            ((delay * idsp_dma_speed as libc::c_float) as libc::c_int) <<
                sxhires;
        // re-init dly
        if (*dly).lpdelayline.is_null() {
            (*dly).delaysamples = samples;
            DLY_Init(3 as libc::c_int, 0.1f32);
        }
        if (*dly).delaysamples != samples {
            (*dly).xfade = 128 as libc::c_int;
            (*dly).idelayoutputxf =
                (*dly).idelayinput.wrapping_sub(samples as libc::c_ulong) as
                    libc::c_int;
            if (*dly).idelayoutputxf < 0 as libc::c_int {
                (*dly).idelayoutputxf =
                    ((*dly).idelayoutputxf as
                         libc::c_ulong).wrapping_add((*dly).cdelaysamplesmax)
                        as libc::c_int as libc::c_int
            }
        }
        (*dly).mod_0 = 0 as libc::c_int;
        (*dly).modcur = (*dly).mod_0;
        if (*dly).delaysamples == 0 as libc::c_int {
            DLY_Free(3 as libc::c_int);
        }
    };
}
/*
=============
DLY_DoStereoDelay

Do stereo processing
=============
*/
#[no_mangle]
pub unsafe extern "C" fn DLY_DoStereoDelay(mut count: libc::c_int) {
    let mut delay: libc::c_int = 0; // inactive
    let mut samplexf: libc::c_int = 0;
    let dly: *mut dly_t =
        &mut *rgsxdly.as_mut_ptr().offset(3 as libc::c_int as isize) as
            *mut dly_t;
    let mut paint: *mut portable_samplepair_t = paintto;
    if (*dly).lpdelayline.is_null() { return }
    while count != 0 {
        if (*dly).mod_0 != 0 &&
               { (*dly).modcur -= 1; ((*dly).modcur) < 0 as libc::c_int } {
            (*dly).modcur = (*dly).mod_0
        }
        delay = *(*dly).lpdelayline.offset((*dly).idelayoutput as isize);
        // process only if crossfading, active left value or delayline
        if delay != 0 || (*paint).left != 0 || (*dly).xfade != 0 {
            // set up new crossfade, if not crossfading, not modulating, but going to
            if (*dly).xfade == 0 && (*dly).modcur == 0 && (*dly).mod_0 != 0 {
                (*dly).idelayoutputxf =
                    (*dly).idelayoutput.wrapping_add((COM_RandomLong(0 as
                                                                         libc::c_int,
                                                                     255 as
                                                                         libc::c_int)
                                                          *
                                                          (*dly).delaysamples
                                                          >> 9 as libc::c_int)
                                                         as libc::c_ulong) as
                        libc::c_int;
                (*dly).xfade = 128 as libc::c_int
            }
            (*dly).idelayoutputxf =
                ((*dly).idelayoutputxf as
                     libc::c_ulong).wrapping_rem((*dly).cdelaysamplesmax) as
                    libc::c_int as libc::c_int;
            // modify delay, if crossfading
            if (*dly).xfade != 0 {
                samplexf =
                    *(*dly).lpdelayline.offset((*dly).idelayoutputxf as isize)
                        * (128 as libc::c_int - (*dly).xfade) >>
                        7 as libc::c_int;
                delay = samplexf + (delay * (*dly).xfade >> 7 as libc::c_int);
                (*dly).idelayoutputxf += 1;
                if (*dly).idelayoutputxf as libc::c_ulong >=
                       (*dly).cdelaysamplesmax {
                    (*dly).idelayoutputxf = 0 as libc::c_int
                }
                (*dly).xfade -= 1;
                if (*dly).xfade == 0 as libc::c_int {
                    (*dly).idelayoutput = (*dly).idelayoutputxf as size_t
                }
            }
            // save left value to delay line
            *(*dly).lpdelayline.offset((*dly).idelayinput as isize) =
                if (*paint).left > 32760 as libc::c_int {
                    32760 as libc::c_int
                } else if (*paint).left < -(32760 as libc::c_int) {
                    -(32760 as libc::c_int)
                } else { (*paint).left };
            // paint new delay value
            (*paint).left = delay
        } else {
            // clear delay line
            *(*dly).lpdelayline.offset((*dly).idelayinput as isize) =
                0 as libc::c_int
        }
        DLY_MovePointer(dly);
        count -= 1;
        paint = paint.offset(1)
    };
}
/*
=============
DLY_CheckNewDelayVal

Update delay processor settings if we are in new room
=============
*/
#[no_mangle]
pub unsafe extern "C" fn DLY_CheckNewDelayVal() {
    let mut delay: libc::c_float = (*sxdly_delay).value;
    let dly: *mut dly_t =
        &mut *rgsxdly.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut dly_t;
    if (*sxdly_delay).flags & (1 as libc::c_int) << 13 as libc::c_int != 0 {
        if delay == 0 as libc::c_int as libc::c_float {
            DLY_Free(0 as libc::c_int);
        } else {
            delay = if delay < 0.4f32 { delay } else { 0.4f32 };
            (*dly).delaysamples =
                ((delay * idsp_dma_speed as libc::c_float) as libc::c_int) <<
                    sxhires;
            // init dly
            if (*dly).lpdelayline.is_null() {
                DLY_Init(0 as libc::c_int, 0.4f32);
            }
            if !(*dly).lpdelayline.is_null() {
                memset((*dly).lpdelayline as *mut libc::c_void,
                       0 as libc::c_int,
                       (*dly).cdelaysamplesmax.wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                                as
                                                                libc::c_ulong));
                (*dly).lp2 = 0 as libc::c_int;
                (*dly).lp1 = (*dly).lp2;
                (*dly).lp0 = (*dly).lp1
            }
            (*dly).idelayinput = 0 as libc::c_int as size_t;
            (*dly).idelayoutput =
                (*dly).cdelaysamplesmax.wrapping_sub((*dly).delaysamples as
                                                         libc::c_ulong);
            if (*dly).delaysamples == 0 { DLY_Free(0 as libc::c_int); }
        }
    }
    (*dly).lp = (*sxdly_lp).value as libc::c_int;
    (*dly).delayfeedback =
        (255 as libc::c_int as libc::c_float * (*sxdly_feedback).value) as
            libc::c_int;
}
/*
=============
DLY_DoDelay

Do delay processing
=============
*/
#[no_mangle]
pub unsafe extern "C" fn DLY_DoDelay(mut count: libc::c_int) {
    let dly: *mut dly_t =
        &mut *rgsxdly.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut dly_t; // inactive
    let mut paint: *mut portable_samplepair_t = paintto;
    let mut delay: libc::c_int = 0;
    if (*dly).lpdelayline.is_null() || count == 0 { return }
    while count != 0 {
        delay = *(*dly).lpdelayline.offset((*dly).idelayoutput as isize);
        // don't process if delay line and left/right samples are zero
        if delay != 0 || (*paint).left != 0 || (*paint).right != 0 {
            // calculate delayed value from average
            let mut val: libc::c_int =
                ((*paint).left + (*paint).right >> 1 as libc::c_int) +
                    ((*dly).delayfeedback * delay >> 8 as libc::c_int);
            val =
                if val > 32760 as libc::c_int {
                    32760 as libc::c_int
                } else if val < -(32760 as libc::c_int) {
                    -(32760 as libc::c_int)
                } else { val };
            if (*dly).lp != 0 {
                // lowpass
                val = ((*dly).lp0 + (*dly).lp1 + val) / 3 as libc::c_int;
                (*dly).lp0 = (*dly).lp1;
                (*dly).lp1 = val
            }
            *(*dly).lpdelayline.offset((*dly).idelayinput as isize) = val;
            val >>= 2 as libc::c_int;
            (*paint).left =
                if (*paint).left + val > 32760 as libc::c_int {
                    32760 as libc::c_int
                } else if (*paint).left + val < -(32760 as libc::c_int) {
                    -(32760 as libc::c_int)
                } else { ((*paint).left) + val };
            (*paint).right =
                if (*paint).right + val > 32760 as libc::c_int {
                    32760 as libc::c_int
                } else if (*paint).right + val < -(32760 as libc::c_int) {
                    -(32760 as libc::c_int)
                } else { ((*paint).right) + val }
        } else {
            *(*dly).lpdelayline.offset((*dly).idelayinput as isize) =
                0 as libc::c_int;
            (*dly).lp2 = 0 as libc::c_int;
            (*dly).lp1 = (*dly).lp2;
            (*dly).lp0 = (*dly).lp1
        }
        DLY_MovePointer(dly);
        count -= 1;
        paint = paint.offset(1)
    };
}
/*
===========
RVB_SetUpDly

Set up dly for reverb
===========
*/
#[no_mangle]
pub unsafe extern "C" fn RVB_SetUpDly(mut pos: libc::c_int,
                                      mut delay: libc::c_float,
                                      mut kmod: libc::c_int) {
    let mut samples: libc::c_int = 0;
    delay = if delay < 0.1f32 { delay } else { 0.1f32 };
    samples =
        ((delay * idsp_dma_speed as libc::c_float) as libc::c_int) << sxhires;
    if rgsxdly[pos as usize].lpdelayline.is_null() {
        rgsxdly[pos as usize].delaysamples = samples;
        DLY_Init(pos, 0.1f32);
    }
    rgsxdly[pos as usize].mod_0 =
        (kmod * idsp_dma_speed / 11025 as libc::c_int) << sxhires;
    rgsxdly[pos as usize].modcur = rgsxdly[pos as usize].mod_0;
    // set up crossfade, if delay has changed
    if rgsxdly[pos as usize].delaysamples != samples {
        rgsxdly[pos as usize].idelayoutputxf =
            rgsxdly[pos as
                        usize].idelayinput.wrapping_sub(samples as
                                                            libc::c_ulong) as
                libc::c_int;
        if rgsxdly[pos as usize].idelayoutputxf < 0 as libc::c_int {
            rgsxdly[pos as usize].idelayoutputxf =
                (rgsxdly[pos as usize].idelayoutputxf as
                     libc::c_ulong).wrapping_add(rgsxdly[pos as
                                                             usize].cdelaysamplesmax)
                    as libc::c_int as libc::c_int
        }
        rgsxdly[pos as usize].xfade = 32 as libc::c_int
    }
    if rgsxdly[pos as usize].delaysamples == 0 { DLY_Free(pos); };
}
/*
===========
RVB_CheckNewReverbVal

Update reverb settings if we are in new room
===========
*/
#[no_mangle]
pub unsafe extern "C" fn RVB_CheckNewReverbVal() {
    let dly1: *mut dly_t =
        &mut *rgsxdly.as_mut_ptr().offset(1 as libc::c_int as isize) as
            *mut dly_t;
    let dly2: *mut dly_t =
        &mut *rgsxdly.as_mut_ptr().offset((1 as libc::c_int +
                                               1 as libc::c_int) as isize) as
            *mut dly_t;
    let mut delay: libc::c_float = (*sxrvb_size).value;
    if (*sxrvb_size).flags & (1 as libc::c_int) << 13 as libc::c_int != 0 {
        if delay == 0.0f32 {
            DLY_Free(1 as libc::c_int);
            DLY_Free(1 as libc::c_int + 1 as libc::c_int);
        } else {
            RVB_SetUpDly(1 as libc::c_int, (*sxrvb_size).value,
                         500 as libc::c_int);
            RVB_SetUpDly(1 as libc::c_int + 1 as libc::c_int,
                         (*sxrvb_size).value * 0.71f32, 700 as libc::c_int);
        }
    }
    (*dly2).lp = (*sxrvb_lp).value as libc::c_int;
    (*dly1).lp = (*dly2).lp;
    (*dly2).delayfeedback =
        (255 as libc::c_int as libc::c_float * (*sxrvb_feedback).value) as
            libc::c_int;
    (*dly1).delayfeedback = (*dly2).delayfeedback;
}
/*
===========
RVB_DoReverbForOneDly

Do reverberation for one dly
===========
*/
#[no_mangle]
pub unsafe extern "C" fn RVB_DoReverbForOneDly(mut dly: *mut dly_t,
                                               vlr: libc::c_int,
                                               mut samplepair:
                                                   *const portable_samplepair_t)
 -> libc::c_int {
    let mut delay: libc::c_int = 0;
    let mut samplexf: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    let mut valt: libc::c_int = 0;
    let mut voutm: libc::c_int = 0 as libc::c_int;
    (*dly).modcur -= 1;
    if (*dly).modcur < 0 as libc::c_int { (*dly).modcur = (*dly).mod_0 }
    delay = *(*dly).lpdelayline.offset((*dly).idelayoutput as isize);
    if (*dly).xfade != 0 || delay != 0 || (*samplepair).left != 0 ||
           (*samplepair).right != 0 {
        // modulate delay rate
        if (*dly).mod_0 == 0 {
            (*dly).idelayoutputxf =
                (*dly).idelayoutput.wrapping_add((COM_RandomLong(0 as
                                                                     libc::c_int,
                                                                 255 as
                                                                     libc::c_int)
                                                      * delay >>
                                                      9 as libc::c_int) as
                                                     libc::c_ulong) as
                    libc::c_int;
            (*dly).idelayoutputxf =
                ((*dly).idelayoutputxf as
                     libc::c_ulong).wrapping_rem((*dly).cdelaysamplesmax) as
                    libc::c_int as libc::c_int;
            (*dly).xfade = 32 as libc::c_int
        }
        if (*dly).xfade != 0 {
            samplexf =
                *(*dly).lpdelayline.offset((*dly).idelayoutputxf as isize) *
                    (32 as libc::c_int - (*dly).xfade) / 32 as libc::c_int;
            delay = delay * (*dly).xfade / 32 as libc::c_int + samplexf;
            (*dly).idelayoutputxf += 1;
            if (*dly).idelayoutputxf as libc::c_ulong >=
                   (*dly).cdelaysamplesmax {
                (*dly).idelayoutputxf = 0 as libc::c_int
            }
            (*dly).xfade -= 1;
            if (*dly).xfade == 0 as libc::c_int {
                (*dly).idelayoutput = (*dly).idelayoutputxf as size_t
            }
        }
        if delay != 0 {
            val = vlr + ((*dly).delayfeedback * delay >> 8 as libc::c_int);
            val =
                if val > 32760 as libc::c_int {
                    32760 as libc::c_int
                } else if val < -(32760 as libc::c_int) {
                    -(32760 as libc::c_int)
                } else { val }
        } else { val = vlr }
        if (*dly).lp != 0 {
            valt = (*dly).lp0 + val >> 1 as libc::c_int;
            (*dly).lp0 = val
        } else { valt = val }
        let ref mut fresh0 =
            *(*dly).lpdelayline.offset((*dly).idelayinput as isize);
        *fresh0 = valt;
        voutm = *fresh0
    } else {
        let ref mut fresh1 =
            *(*dly).lpdelayline.offset((*dly).idelayinput as isize);
        *fresh1 = 0 as libc::c_int;
        voutm = *fresh1;
        (*dly).lp0 = 0 as libc::c_int
    }
    DLY_MovePointer(dly);
    return voutm;
}
/*
===========
RVB_DoReverb

Do reverberation processing
===========
*/
#[no_mangle]
pub unsafe extern "C" fn RVB_DoReverb(mut count: libc::c_int) {
    let dly1: *mut dly_t =
        &mut *rgsxdly.as_mut_ptr().offset(1 as libc::c_int as isize) as
            *mut dly_t; // alpha
    let dly2: *mut dly_t =
        &mut *rgsxdly.as_mut_ptr().offset((1 as libc::c_int +
                                               1 as libc::c_int) as isize) as
            *mut dly_t;
    let mut paint: *mut portable_samplepair_t = paintto;
    let mut vlr: libc::c_int = 0;
    let mut voutm: libc::c_int = 0;
    if (*dly1).lpdelayline.is_null() { return }
    while count != 0 {
        vlr = (*paint).left + (*paint).right >> 1 as libc::c_int;
        voutm = RVB_DoReverbForOneDly(dly1, vlr, paint);
        voutm += RVB_DoReverbForOneDly(dly2, vlr, paint);
        if (*dsp_coeff_table).value == 1.0f32 {
            voutm /= 6 as libc::c_int
        } else { voutm = 11 as libc::c_int * voutm >> 6 as libc::c_int }
        (*paint).left =
            if (*paint).left + voutm > 32760 as libc::c_int {
                32760 as libc::c_int
            } else if (*paint).left + voutm < -(32760 as libc::c_int) {
                -(32760 as libc::c_int)
            } else { ((*paint).left) + voutm };
        (*paint).right =
            if (*paint).right + voutm > 32760 as libc::c_int {
                32760 as libc::c_int
            } else if (*paint).right + voutm < -(32760 as libc::c_int) {
                -(32760 as libc::c_int)
            } else { ((*paint).right) + voutm };
        count -= 1;
        paint = paint.offset(1)
    };
}
/*
===========
RVB_DoAMod

Do amplification modulation processing
===========
*/
#[no_mangle]
pub unsafe extern "C" fn RVB_DoAMod(mut count: libc::c_int) {
    let mut paint: *mut portable_samplepair_t = paintto;
    if (*sxmod_lowpass).value == 0. && (*sxmod_mod).value == 0. { return }
    while count != 0 {
        let mut res: portable_samplepair_t = *paint;
        if (*sxmod_lowpass).value != 0. {
            res.left =
                rgsxlp[0 as libc::c_int as usize] +
                    rgsxlp[1 as libc::c_int as usize] +
                    rgsxlp[2 as libc::c_int as usize] +
                    rgsxlp[3 as libc::c_int as usize] +
                    rgsxlp[4 as libc::c_int as usize] + res.left;
            res.right =
                rgsxlp[5 as libc::c_int as usize] +
                    rgsxlp[6 as libc::c_int as usize] +
                    rgsxlp[7 as libc::c_int as usize] +
                    rgsxlp[8 as libc::c_int as usize] +
                    rgsxlp[9 as libc::c_int as usize] + res.right;
            res.left >>= 2 as libc::c_int;
            res.right >>= 2 as libc::c_int;
            rgsxlp[4 as libc::c_int as usize] = (*paint).left;
            rgsxlp[9 as libc::c_int as usize] = (*paint).right;
            rgsxlp[0 as libc::c_int as usize] =
                rgsxlp[1 as libc::c_int as usize];
            rgsxlp[1 as libc::c_int as usize] =
                rgsxlp[2 as libc::c_int as usize];
            rgsxlp[2 as libc::c_int as usize] =
                rgsxlp[3 as libc::c_int as usize];
            rgsxlp[3 as libc::c_int as usize] =
                rgsxlp[4 as libc::c_int as usize];
            rgsxlp[4 as libc::c_int as usize] =
                rgsxlp[5 as libc::c_int as usize];
            rgsxlp[5 as libc::c_int as usize] =
                rgsxlp[6 as libc::c_int as usize];
            rgsxlp[6 as libc::c_int as usize] =
                rgsxlp[7 as libc::c_int as usize];
            rgsxlp[7 as libc::c_int as usize] =
                rgsxlp[8 as libc::c_int as usize];
            rgsxlp[8 as libc::c_int as usize] =
                rgsxlp[9 as libc::c_int as usize]
        }
        if (*sxmod_mod).value != 0. {
            sxmod1cur -= 1;
            if sxmod1cur < 0 as libc::c_int { sxmod1cur = sxmod1 }
            if sxmod1 == 0 {
                sxamodlt =
                    COM_RandomLong(32 as libc::c_int, 255 as libc::c_int)
            }
            sxmod2cur -= 1;
            if sxmod2cur < 0 as libc::c_int { sxmod2cur = sxmod2 }
            if sxmod2 == 0 {
                sxamodrt =
                    COM_RandomLong(32 as libc::c_int, 255 as libc::c_int)
            }
            res.left = sxamodl * res.left >> 8 as libc::c_int;
            res.right = sxamodr * res.right >> 8 as libc::c_int;
            if sxamodl < sxamodlt {
                sxamodl += 1
            } else if sxamodl > sxamodlt { sxamodl -= 1 }
            if sxamodr < sxamodrt {
                sxamodr += 1
            } else if sxamodr > sxamodrt { sxamodr -= 1 }
        }
        (*paint).left =
            if res.left > 32760 as libc::c_int {
                32760 as libc::c_int
            } else if res.left < -(32760 as libc::c_int) {
                -(32760 as libc::c_int)
            } else { res.left };
        (*paint).right =
            if res.right > 32760 as libc::c_int {
                32760 as libc::c_int
            } else if res.right < -(32760 as libc::c_int) {
                -(32760 as libc::c_int)
            } else { res.right };
        count -= 1;
        paint = paint.offset(1)
    };
}
/*
===========
DSP_Process

(xash dsp interface)
===========
*/
#[no_mangle]
pub unsafe extern "C" fn DSP_Process(mut idsp: libc::c_int,
                                     mut pbfront: *mut portable_samplepair_t,
                                     mut sampleCount: libc::c_int) {
    if (*dsp_off).value != 0. { return }
    // don't process DSP while in menu
    if cls.key_dest as libc::c_uint == key_menu as libc::c_int as libc::c_uint
           || sampleCount == 0 {
        return
    }
    // preset is already installed by CheckNewDspPresets
    paintto = pbfront;
    RVB_DoAMod(sampleCount);
    RVB_DoReverb(sampleCount);
    DLY_DoDelay(sampleCount);
    DLY_DoStereoDelay(sampleCount);
}
/*
===========
DSP_ClearState

(xash dsp interface)
===========
*/
#[no_mangle]
pub unsafe extern "C" fn DSP_ClearState() {
    Cvar_SetValue(b"room_type\x00" as *const u8 as *const libc::c_char,
                  0.0f32);
    SX_ReloadRoomFX();
}
/*
===========
CheckNewDspPresets

(xash dsp interface)
===========
*/
#[no_mangle]
pub unsafe extern "C" fn CheckNewDspPresets() {
    if (*dsp_off).value != 0.0f32 { return }
    if (*dsp_coeff_table).flags & (1 as libc::c_int) << 13 as libc::c_int != 0
       {
        match (*dsp_coeff_table).value as libc::c_int {
            0 => {
                // release
                ptable = rgsxpre.as_ptr()
            }
            1 => {
                // alpha
                ptable = rgsxpre_hlalpha052.as_ptr()
            }
            _ => { ptable = rgsxpre.as_ptr() }
        }
        SX_ReloadRoomFX();
        room_typeprev = -(1 as libc::c_int);
        (*dsp_coeff_table).flags =
            (*dsp_coeff_table).flags &
                !((1 as libc::c_int) << 13 as libc::c_int)
    }
    if s_listener.waterlevel > 2 as libc::c_int {
        idsp_room = (*roomwater_type).value as libc::c_int
    } else { idsp_room = (*room_type).value as libc::c_int }
    // don't pass invalid presets
    idsp_room =
        if idsp_room >= 0 as libc::c_int {
            if idsp_room < 29 as libc::c_int - 1 as libc::c_int {
                idsp_room
            } else { (29 as libc::c_int) - 1 as libc::c_int }
        } else { 0 as libc::c_int };
    if (*hisound).flags & (1 as libc::c_int) << 13 as libc::c_int != 0 {
        sxhires = (*hisound).value as libc::c_int;
        (*hisound).flags =
            (*hisound).flags & !((1 as libc::c_int) << 13 as libc::c_int)
    }
    if idsp_room == room_typeprev && idsp_room == 0 as libc::c_int { return }
    if idsp_room as libc::c_ulong >
           (::std::mem::size_of::<[sx_preset_t; 29]>() as
                libc::c_ulong).wrapping_div(::std::mem::size_of::<sx_preset_t>()
                                                as libc::c_ulong) {
        return
    }
    if idsp_room != room_typeprev {
        let mut cur: *const sx_preset_t = 0 as *const sx_preset_t;
        cur = ptable.offset(idsp_room as isize);
        Cvar_SetValue(b"room_lp\x00" as *const u8 as *const libc::c_char,
                      (*cur).room_lp);
        Cvar_SetValue(b"room_mod\x00" as *const u8 as *const libc::c_char,
                      (*cur).room_mod);
        Cvar_SetValue(b"room_size\x00" as *const u8 as *const libc::c_char,
                      (*cur).room_size);
        Cvar_SetValue(b"room_refl\x00" as *const u8 as *const libc::c_char,
                      (*cur).room_refl);
        Cvar_SetValue(b"room_rvblp\x00" as *const u8 as *const libc::c_char,
                      (*cur).room_rvblp);
        Cvar_SetValue(b"room_delay\x00" as *const u8 as *const libc::c_char,
                      (*cur).room_delay);
        Cvar_SetValue(b"room_feedback\x00" as *const u8 as
                          *const libc::c_char, (*cur).room_feedback);
        Cvar_SetValue(b"room_dlylp\x00" as *const u8 as *const libc::c_char,
                      (*cur).room_dlylp);
        Cvar_SetValue(b"room_left\x00" as *const u8 as *const libc::c_char,
                      (*cur).room_left);
    }
    room_typeprev = idsp_room;
    RVB_CheckNewReverbVal();
    DLY_CheckNewDelayVal();
    DLY_CheckNewStereoDelayVal();
    (*sxrvb_size).flags =
        (*sxrvb_size).flags & !((1 as libc::c_int) << 13 as libc::c_int);
    (*sxdly_delay).flags =
        (*sxdly_delay).flags & !((1 as libc::c_int) << 13 as libc::c_int);
    (*sxste_delay).flags =
        (*sxste_delay).flags & !((1 as libc::c_int) << 13 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn SX_Profiling_f() {
    let mut testbuffer: [portable_samplepair_t; 512] =
        [portable_samplepair_t{left: 0, right: 0,}; 512];
    let mut oldroom: libc::c_float = (*room_type).value;
    let mut start: libc::c_double = 0.;
    let mut end: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut calls: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 512 as libc::c_int {
        testbuffer[i as usize].left =
            COM_RandomLong(0 as libc::c_int, 3000 as libc::c_int);
        testbuffer[i as usize].right =
            COM_RandomLong(0 as libc::c_int, 3000 as libc::c_int);
        i += 1
    }
    if Cmd_Argc() > 1 as libc::c_int {
        Cvar_SetValue(b"room_type\x00" as *const u8 as *const libc::c_char,
                      Q_atof(Cmd_Argv(1 as libc::c_int)));
        SX_ReloadRoomFX();
        CheckNewDspPresets();
        // we just need idsp_room immediately, for message below
    }
    Con_Printf(b"Profiling 10000 calls to DSP. Sample count is 512, room_type is %i\n\x00"
                   as *const u8 as *const libc::c_char, idsp_room);
    start = Sys_DoubleTime();
    calls = 10000 as libc::c_int;
    while calls != 0 {
        DSP_Process(idsp_room, testbuffer.as_mut_ptr(), 512 as libc::c_int);
        calls -= 1
    }
    end = Sys_DoubleTime();
    Con_Printf(b"----------\nTook %g seconds.\n\x00" as *const u8 as
                   *const libc::c_char, end - start);
    if Cmd_Argc() > 1 as libc::c_int {
        Cvar_SetValue(b"room_type\x00" as *const u8 as *const libc::c_char,
                      oldroom);
        SX_ReloadRoomFX();
        CheckNewDspPresets();
    };
}
