#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, extern_types,
           register_tool)]
extern "C" {
    pub type file_s;
    pub type grasshdr_s;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn Info_SetValueForKey(s: *mut libc::c_char, key: *const libc::c_char,
                           value: *const libc::c_char, maxsize: libc::c_int)
     -> qboolean;
    #[no_mangle]
    fn Info_ValueForKey(s: *const libc::c_char, key: *const libc::c_char)
     -> *const libc::c_char;
    #[no_mangle]
    fn Q_strlen(string: *const libc::c_char) -> size_t;
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Q_atoi(str: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn Q_strncmp(s1: *const libc::c_char, s2: *const libc::c_char,
                 n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Q_snprintf(buffer: *mut libc::c_char, buffersize: size_t,
                  format: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn Cvar_RegisterVariable(var: *mut convar_t);
    #[no_mangle]
    fn Cvar_Get(var_name: *const libc::c_char, value: *const libc::c_char,
                flags: libc::c_int, description: *const libc::c_char)
     -> *mut convar_t;
    #[no_mangle]
    fn Cvar_FullSet(var_name: *const libc::c_char, value: *const libc::c_char,
                    flags: libc::c_int);
    #[no_mangle]
    fn Cvar_SetValue(var_name: *const libc::c_char, value: libc::c_float);
    #[no_mangle]
    static mut host_developer: convar_t;
    #[no_mangle]
    static mut host_limitlocal: *mut convar_t;
    #[no_mangle]
    static mut net_clockwindow: *mut convar_t;
    #[no_mangle]
    fn NET_Config(net_enable: qboolean);
    #[no_mangle]
    fn NET_IsLocalAddress(adr: netadr_t) -> qboolean;
    #[no_mangle]
    fn NET_StringToAdr(string: *const libc::c_char, adr: *mut netadr_t)
     -> qboolean;
    #[no_mangle]
    fn NET_CompareBaseAdr(a: netadr_t, b: netadr_t) -> qboolean;
    #[no_mangle]
    fn NET_GetPacket(sock: netsrc_t, from: *mut netadr_t, data: *mut byte,
                     length: *mut size_t) -> qboolean;
    #[no_mangle]
    fn NET_SendPacket(sock: netsrc_t, length: size_t,
                      data: *const libc::c_void, to: netadr_t);
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    static mut SI: sysinfo_t;
    #[no_mangle]
    fn Cmd_Argv(arg: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn Cmd_TokenizeString(text: *const libc::c_char);
    #[no_mangle]
    fn _Mem_Free(data: *mut libc::c_void, filename: *const libc::c_char,
                 fileline: libc::c_int);
    #[no_mangle]
    fn Q_buildnum() -> libc::c_int;
    #[no_mangle]
    fn Q_buildos() -> *const libc::c_char;
    #[no_mangle]
    fn Q_buildarch() -> *const libc::c_char;
    #[no_mangle]
    fn Q_buildcommit() -> *const libc::c_char;
    #[no_mangle]
    fn Host_IsLocalGame() -> qboolean;
    #[no_mangle]
    fn CL_Active() -> libc::c_int;
    #[no_mangle]
    fn COM_HexConvert(pszInput: *const libc::c_char,
                      nInputLength: libc::c_int, pOutput: *mut byte);
    #[no_mangle]
    fn HPAK_AddLump(queue: qboolean, filename: *const libc::c_char,
                    pRes: *mut resource_s, data: *mut byte, f: *mut file_t);
    #[no_mangle]
    fn COM_CreateCustomization(pHead: *mut customization_t,
                               pRes: *mut resource_t, playernum: libc::c_int,
                               flags: libc::c_int,
                               pCust: *mut *mut customization_t,
                               nLumps: *mut libc::c_int) -> qboolean;
    #[no_mangle]
    fn Con_DPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn SV_BroadcastPrintf(ignore: *mut sv_client_s, fmt: *const libc::c_char,
                          _: ...);
    #[no_mangle]
    fn SV_Active() -> qboolean;
    #[no_mangle]
    fn CL_IsInConsole() -> qboolean;
    #[no_mangle]
    fn CL_IsInGame() -> qboolean;
    #[no_mangle]
    fn SV_Initialized() -> qboolean;
    #[no_mangle]
    fn CL_IsPlaybackDemo() -> qboolean;
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn CL_Drop();
    #[no_mangle]
    fn HPAK_FlushHostQueue();
    #[no_mangle]
    fn Log_Printf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Mod_FreeAll();
    #[no_mangle]
    fn Netchan_CopyFileFragments(chan: *mut netchan_t, msg: *mut sizebuf_t)
     -> qboolean;
    #[no_mangle]
    fn Netchan_CopyNormalFragments(chan: *mut netchan_t, msg: *mut sizebuf_t,
                                   length: *mut size_t) -> qboolean;
    #[no_mangle]
    fn Netchan_IncomingReady(chan: *mut netchan_t) -> qboolean;
    #[no_mangle]
    fn Netchan_Process(chan: *mut netchan_t, msg: *mut sizebuf_t) -> qboolean;
    #[no_mangle]
    static mut net_from: netadr_t;
    #[no_mangle]
    fn MSG_ReadShort(sb: *mut sizebuf_t) -> libc::c_int;
    #[no_mangle]
    fn MSG_ReadLong(sb: *mut sizebuf_t) -> libc::c_int;
    #[no_mangle]
    fn MSG_Clear(sb: *mut sizebuf_t);
    #[no_mangle]
    fn SV_RemoteCommand(from: netadr_t, msg: *mut sizebuf_t);
    #[no_mangle]
    fn MSG_ReadStringExt(sb: *mut sizebuf_t, bLine: qboolean)
     -> *mut libc::c_char;
    #[no_mangle]
    static mut svs: server_static_t;
    #[no_mangle]
    fn MSG_ReadUBitLong(sb: *mut sizebuf_t, numbits: libc::c_int) -> uint;
    #[no_mangle]
    fn MSG_InitExt(sb: *mut sizebuf_t, pDebugName: *const libc::c_char,
                   pData: *mut libc::c_void, nBytes: libc::c_int,
                   nMaxBits: libc::c_int);
    #[no_mangle]
    static mut net_message: sizebuf_t;
    #[no_mangle]
    static mut net_message_buffer: [byte; 131120];
    #[no_mangle]
    fn SV_InitGame() -> qboolean;
    #[no_mangle]
    fn SV_DropClient(cl: *mut sv_client_t, crash: qboolean);
    #[no_mangle]
    fn SV_Physics();
    #[no_mangle]
    static mut svgame: svgame_static_t;
    #[no_mangle]
    fn MSG_WriteString(sb: *mut sizebuf_t, pStr: *const libc::c_char)
     -> qboolean;
    #[no_mangle]
    fn MSG_WriteOneBit(sb: *mut sizebuf_t, nValue: libc::c_int);
    #[no_mangle]
    fn MSG_WriteCmdExt(sb: *mut sizebuf_t, cmd: libc::c_int, type_0: netsrc_t,
                       name: *const libc::c_char);
    #[no_mangle]
    fn Netchan_TransmitBits(chan: *mut netchan_t, lengthInBits: libc::c_int,
                            data: *mut byte);
    #[no_mangle]
    static mut sv: server_t;
    #[no_mangle]
    fn SV_UnloadProgs();
    #[no_mangle]
    fn SV_RequestMissingResources();
    #[no_mangle]
    fn SV_BuildReconnect(msg: *mut sizebuf_t);
    #[no_mangle]
    fn SV_EndRedirect();
    #[no_mangle]
    fn SV_MoveToOnHandList(cl: *mut sv_client_t, pResource: *mut resource_t);
    #[no_mangle]
    fn SV_ExecuteClientMessage(cl: *mut sv_client_t, msg: *mut sizebuf_t);
    #[no_mangle]
    fn SV_ConnectionlessPacket(from: netadr_t, msg: *mut sizebuf_t);
    #[no_mangle]
    fn SV_SendClientMessages();
    #[no_mangle]
    fn SV_InitHostCommands();
    #[no_mangle]
    fn SV_TogglePause(msg: *const libc::c_char);
    #[no_mangle]
    fn SV_InitFilter();
    #[no_mangle]
    fn SV_CheckID(id: *const libc::c_char) -> qboolean;
    #[no_mangle]
    fn SV_RejectConnection(from: netadr_t, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Log_Close();
    #[no_mangle]
    fn SV_EdictNum(n: libc::c_int) -> *mut edict_t;
    #[no_mangle]
    fn SV_ClearGameState();
    #[no_mangle]
    fn MSG_WriteDeltaMovevars(msg: *mut sizebuf_t, from: *mut movevars_s,
                              to: *mut movevars_s) -> qboolean;
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
pub struct colorVec {
    pub r: libc::c_uint,
    pub g: libc::c_uint,
    pub b: libc::c_uint,
    pub a: libc::c_uint,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plane_t {
    pub normal: vec3_t,
    pub dist: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct trace_t {
    pub allsolid: qboolean,
    pub startsolid: qboolean,
    pub inopen: qboolean,
    pub inwater: qboolean,
    pub fraction: libc::c_float,
    pub endpos: vec3_t,
    pub plane: plane_t,
    pub ent: *mut edict_t,
    pub hitgroup: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dmodel_t {
    pub mins: vec3_t,
    pub maxs: vec3_t,
    pub origin: vec3_t,
    pub headnode: [libc::c_int; 4],
    pub visleafs: libc::c_int,
    pub firstface: libc::c_int,
    pub numfaces: libc::c_int,
}
pub type modtype_t = libc::c_int;
pub const mod_studio: modtype_t = 3;
pub const mod_alias: modtype_t = 2;
pub const mod_sprite: modtype_t = 1;
pub const mod_brush: modtype_t = 0;
pub const mod_bad: modtype_t = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mplane_s {
    pub normal: vec3_t,
    pub dist: libc::c_float,
    pub type_0: byte,
    pub signbits: byte,
    pub pad: [byte; 2],
}
pub type mplane_t = mplane_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mvertex_t {
    pub position: vec3_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mclipnode_t {
    pub planenum: libc::c_int,
    pub children: [libc::c_short; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct medge_t {
    pub v: [libc::c_ushort; 2],
    pub cachededgeoffset: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct texture_s {
    pub name: [libc::c_char; 16],
    pub width: libc::c_uint,
    pub height: libc::c_uint,
    pub gl_texturenum: libc::c_int,
    pub texturechain: *mut msurface_s,
    pub anim_total: libc::c_int,
    pub anim_min: libc::c_int,
    pub anim_max: libc::c_int,
    pub anim_next: *mut texture_s,
    pub alternate_anims: *mut texture_s,
    pub fb_texturenum: libc::c_ushort,
    pub dt_texturenum: libc::c_ushort,
    pub unused: [libc::c_uint; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msurface_s {
    pub visframe: libc::c_int,
    pub plane: *mut mplane_t,
    pub flags: libc::c_int,
    pub firstedge: libc::c_int,
    pub numedges: libc::c_int,
    pub texturemins: [libc::c_short; 2],
    pub extents: [libc::c_short; 2],
    pub light_s: libc::c_int,
    pub light_t: libc::c_int,
    pub polys: *mut glpoly_t,
    pub texturechain: *mut msurface_s,
    pub texinfo: *mut mtexinfo_t,
    pub dlightframe: libc::c_int,
    pub dlightbits: libc::c_int,
    pub lightmaptexturenum: libc::c_int,
    pub styles: [byte; 4],
    pub cached_light: [libc::c_int; 4],
    pub info: *mut mextrasurf_t,
    pub samples: *mut color24,
    pub pdecals: *mut decal_t,
}
pub type decal_t = decal_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct decal_s {
    pub pnext: *mut decal_t,
    pub psurface: *mut msurface_t,
    pub dx: libc::c_float,
    pub dy: libc::c_float,
    pub scale: libc::c_float,
    pub texture: libc::c_short,
    pub flags: libc::c_short,
    pub entityIndex: libc::c_short,
    pub position: vec3_t,
    pub polys: *mut glpoly_t,
    pub reserved: [libc::c_int; 4],
}
pub type glpoly_t = glpoly_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glpoly_s {
    pub next: *mut glpoly_s,
    pub chain: *mut glpoly_s,
    pub numverts: libc::c_int,
    pub flags: libc::c_int,
    pub verts: [[libc::c_float; 7]; 4],
}
pub type msurface_t = msurface_s;
pub type mextrasurf_t = mextrasurf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mextrasurf_s {
    pub mins: vec3_t,
    pub maxs: vec3_t,
    pub origin: vec3_t,
    pub surf: *mut msurface_s,
    pub dlight_s: libc::c_int,
    pub dlight_t: libc::c_int,
    pub lightmapmins: [libc::c_short; 2],
    pub lightextents: [libc::c_short; 2],
    pub lmvecs: [[libc::c_float; 4]; 2],
    pub deluxemap: *mut color24,
    pub shadowmap: *mut byte,
    pub lightmapchain: *mut msurface_s,
    pub detailchain: *mut mextrasurf_s,
    pub bevel: *mut mfacebevel_t,
    pub lumachain: *mut mextrasurf_s,
    pub parent: *mut cl_entity_s,
    pub mirrortexturenum: libc::c_int,
    pub mirrormatrix: [[libc::c_float; 4]; 4],
    pub grass: *mut grasshdr_s,
    pub grasscount: libc::c_ushort,
    pub numverts: libc::c_ushort,
    pub firstvertex: libc::c_int,
    pub reserved: [libc::c_int; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cl_entity_s {
    pub index: libc::c_int,
    pub player: qboolean,
    pub baseline: entity_state_t,
    pub prevstate: entity_state_t,
    pub curstate: entity_state_t,
    pub current_position: libc::c_int,
    pub ph: [position_history_t; 64],
    pub mouth: mouth_t,
    pub latched: latchedvars_t,
    pub lastmove: libc::c_float,
    pub origin: vec3_t,
    pub angles: vec3_t,
    pub attachment: [vec3_t; 4],
    pub trivial_accept: libc::c_int,
    pub model: *mut model_s,
    pub efrag: *mut efrag_s,
    pub topnode: *mut mnode_s,
    pub syncbase: libc::c_float,
    pub visframe: libc::c_int,
    pub cvFloorColor: colorVec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mnode_s {
    pub contents: libc::c_int,
    pub visframe: libc::c_int,
    pub minmaxs: [libc::c_float; 6],
    pub parent: *mut mnode_s,
    pub plane: *mut mplane_t,
    pub children: [*mut mnode_s; 2],
    pub firstsurface: libc::c_ushort,
    pub numsurfaces: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct efrag_s {
    pub leaf: *mut mleaf_s,
    pub leafnext: *mut efrag_s,
    pub entity: *mut cl_entity_s,
    pub entnext: *mut efrag_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mleaf_s {
    pub contents: libc::c_int,
    pub visframe: libc::c_int,
    pub minmaxs: [libc::c_float; 6],
    pub parent: *mut mnode_s,
    pub compressed_vis: *mut byte,
    pub efrags: *mut efrag_s,
    pub firstmarksurface: *mut *mut msurface_t,
    pub nummarksurfaces: libc::c_int,
    pub cluster: libc::c_int,
    pub ambient_sound_level: [byte; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct model_s {
    pub name: [libc::c_char; 64],
    pub needload: qboolean,
    pub type_0: modtype_t,
    pub numframes: libc::c_int,
    pub mempool: poolhandle_t,
    pub flags: libc::c_int,
    pub mins: vec3_t,
    pub maxs: vec3_t,
    pub radius: libc::c_float,
    pub firstmodelsurface: libc::c_int,
    pub nummodelsurfaces: libc::c_int,
    pub numsubmodels: libc::c_int,
    pub submodels: *mut dmodel_t,
    pub numplanes: libc::c_int,
    pub planes: *mut mplane_t,
    pub numleafs: libc::c_int,
    pub leafs: *mut mleaf_t,
    pub numvertexes: libc::c_int,
    pub vertexes: *mut mvertex_t,
    pub numedges: libc::c_int,
    pub edges: *mut medge_t,
    pub numnodes: libc::c_int,
    pub nodes: *mut mnode_t,
    pub numtexinfo: libc::c_int,
    pub texinfo: *mut mtexinfo_t,
    pub numsurfaces: libc::c_int,
    pub surfaces: *mut msurface_t,
    pub numsurfedges: libc::c_int,
    pub surfedges: *mut libc::c_int,
    pub numclipnodes: libc::c_int,
    pub clipnodes: *mut mclipnode_t,
    pub nummarksurfaces: libc::c_int,
    pub marksurfaces: *mut *mut msurface_t,
    pub hulls: [hull_t; 4],
    pub numtextures: libc::c_int,
    pub textures: *mut *mut texture_t,
    pub visdata: *mut byte,
    pub lightdata: *mut color24,
    pub entities: *mut libc::c_char,
    pub cache: cache_user_t,
}
pub type cache_user_t = cache_user_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cache_user_s {
    pub data: *mut libc::c_void,
}
pub type texture_t = texture_s;
pub type hull_t = hull_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hull_s {
    pub clipnodes: *mut mclipnode_t,
    pub planes: *mut mplane_t,
    pub firstclipnode: libc::c_int,
    pub lastclipnode: libc::c_int,
    pub clip_mins: vec3_t,
    pub clip_maxs: vec3_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mtexinfo_t {
    pub vecs: [[libc::c_float; 4]; 2],
    pub faceinfo: *mut mfaceinfo_t,
    pub texture: *mut texture_t,
    pub flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mfaceinfo_t {
    pub landname: [libc::c_char; 16],
    pub texture_step: libc::c_ushort,
    pub max_extent: libc::c_ushort,
    pub groupid: libc::c_short,
    pub mins: vec3_t,
    pub maxs: vec3_t,
    pub reserved: [libc::c_int; 32],
}
pub type mnode_t = mnode_s;
pub type mleaf_t = mleaf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct latchedvars_t {
    pub prevanimtime: libc::c_float,
    pub sequencetime: libc::c_float,
    pub prevseqblending: [byte; 2],
    pub prevorigin: vec3_t,
    pub prevangles: vec3_t,
    pub prevsequence: libc::c_int,
    pub prevframe: libc::c_float,
    pub prevcontroller: [byte; 4],
    pub prevblending: [byte; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mouth_t {
    pub mouthopen: byte,
    pub sndcount: byte,
    pub sndavg: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct position_history_t {
    pub animtime: libc::c_float,
    pub origin: vec3_t,
    pub angles: vec3_t,
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
pub struct mfacebevel_t {
    pub edges: *mut mplane_t,
    pub numedges: libc::c_int,
    pub origin: vec3_t,
    pub radius: vec_t,
    pub contents: libc::c_int,
}
pub type model_t = model_s;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gameinfo_s {
    pub gamefolder: [libc::c_char; 64],
    pub basedir: [libc::c_char; 64],
    pub falldir: [libc::c_char; 64],
    pub startmap: [libc::c_char; 64],
    pub trainmap: [libc::c_char; 64],
    pub title: [libc::c_char; 64],
    pub version: libc::c_float,
    pub dll_path: [libc::c_char; 64],
    pub game_dll: [libc::c_char; 64],
    pub iconpath: [libc::c_char; 64],
    pub game_url: string,
    pub update_url: string,
    pub type_0: [libc::c_char; 64],
    pub date: [libc::c_char; 64],
    pub size: size_t,
    pub gamemode: libc::c_int,
    pub secure: qboolean,
    pub nomodels: qboolean,
    pub noskills: qboolean,
    pub sp_entity: [libc::c_char; 32],
    pub mp_entity: [libc::c_char; 32],
    pub mp_filter: [libc::c_char; 32],
    pub ambientsound: [[libc::c_char; 64]; 4],
    pub max_edicts: libc::c_int,
    pub max_tents: libc::c_int,
    pub max_beams: libc::c_int,
    pub max_particles: libc::c_int,
    pub game_dll_linux: [libc::c_char; 64],
    pub game_dll_osx: [libc::c_char; 64],
    pub added: qboolean,
}
pub type gameinfo_t = gameinfo_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sysinfo_s {
    pub exeName: string,
    pub rcName: string,
    pub basedirName: string,
    pub gamedll: string,
    pub clientlib: string,
    pub GameInfo: *mut gameinfo_t,
    pub games: [*mut gameinfo_t; 512],
    pub numgames: libc::c_int,
}
pub type sysinfo_t = sysinfo_s;
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
pub type server_t = server_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct server_s {
    pub state: sv_state_t,
    pub background: qboolean,
    pub loadgame: qboolean,
    pub time: libc::c_double,
    pub time_residual: libc::c_double,
    pub frametime: libc::c_float,
    pub framecount: libc::c_int,
    pub current_client: *mut sv_client_s,
    pub hostflags: libc::c_int,
    pub worldmapCRC: CRC32_t,
    pub progsCRC: libc::c_int,
    pub name: [libc::c_char; 64],
    pub startspot: [libc::c_char; 64],
    pub lastchecktime: libc::c_double,
    pub lastcheck: libc::c_int,
    pub model_precache: [[libc::c_char; 64]; 1024],
    pub sound_precache: [[libc::c_char; 64]; 2048],
    pub files_precache: [[libc::c_char; 64]; 1024],
    pub event_precache: [[libc::c_char; 64]; 1024],
    pub model_precache_flags: [byte; 1024],
    pub models: [*mut model_t; 1024],
    pub num_static_entities: libc::c_int,
    pub lightstyles: [lightstyle_t; 64],
    pub consistency_list: [consistency_t; 1024],
    pub resources: [resource_t; 5120],
    pub num_consistency: libc::c_int,
    pub num_resources: libc::c_int,
    pub instanced: [sv_baseline_t; 64],
    pub last_valid_baseline: libc::c_int,
    pub num_instanced: libc::c_int,
    pub datagram: sizebuf_t,
    pub datagram_buf: [byte; 16384],
    pub reliable_datagram: sizebuf_t,
    pub reliable_datagram_buf: [byte; 16384],
    pub multicast: sizebuf_t,
    pub multicast_buf: [byte; 8192],
    pub signon: sizebuf_t,
    pub signon_buf: [byte; 131072],
    pub spec_datagram: sizebuf_t,
    pub spectator_buf: [byte; 8192],
    pub worldmodel: *mut model_t,
    pub playersonly: qboolean,
    pub simulating: qboolean,
    pub paused: qboolean,
    pub ignored_static_ents: libc::c_int,
    pub ignored_world_decals: libc::c_int,
    pub static_ents_overflow: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sv_baseline_t {
    pub classname: *const libc::c_char,
    pub baseline: entity_state_t,
}
pub type consistency_t = consistency_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct consistency_s {
    pub filename: *const libc::c_char,
    pub orig_index: libc::c_int,
    pub check_type: libc::c_int,
    pub issound: qboolean,
    pub value: libc::c_int,
    pub mins: vec3_t,
    pub maxs: vec3_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lightstyle_t {
    pub pattern: [libc::c_char; 256],
    pub map: [libc::c_float; 256],
    pub length: libc::c_int,
    pub value: libc::c_float,
    pub interp: qboolean,
    pub time: libc::c_float,
}
pub type sv_state_t = libc::c_uint;
pub const ss_active: sv_state_t = 2;
pub const ss_loading: sv_state_t = 1;
pub const ss_dead: sv_state_t = 0;
pub type physics_interface_t = physics_interface_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct physics_interface_s {
    pub version: libc::c_int,
    pub SV_CreateEntity: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                     _: *const libc::c_char)
                                    -> libc::c_int>,
    pub SV_PhysicsEntity: Option<unsafe extern "C" fn(_: *mut edict_t)
                                     -> libc::c_int>,
    pub SV_LoadEntities: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                     _: *mut libc::c_char)
                                    -> libc::c_int>,
    pub SV_UpdatePlayerBaseVelocity: Option<unsafe extern "C" fn(_:
                                                                     *mut edict_t)
                                                -> ()>,
    pub SV_AllowSaveGame: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub SV_TriggerTouch: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                     _: *mut edict_t)
                                    -> libc::c_int>,
    pub SV_CheckFeatures: Option<unsafe extern "C" fn() -> libc::c_uint>,
    pub DrawDebugTriangles: Option<unsafe extern "C" fn() -> ()>,
    pub DrawNormalTriangles: Option<unsafe extern "C" fn() -> ()>,
    pub DrawOrthoTriangles: Option<unsafe extern "C" fn() -> ()>,
    pub ClipMoveToEntity: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                      _: *const libc::c_float,
                                                      _: *mut libc::c_float,
                                                      _: *mut libc::c_float,
                                                      _: *const libc::c_float,
                                                      _: *mut trace_t) -> ()>,
    pub ClipPMoveToEntity: Option<unsafe extern "C" fn(_: *mut physent_s,
                                                       _:
                                                           *const libc::c_float,
                                                       _: *mut libc::c_float,
                                                       _: *mut libc::c_float,
                                                       _:
                                                           *const libc::c_float,
                                                       _: *mut pmtrace_s)
                                      -> ()>,
    pub SV_EndFrame: Option<unsafe extern "C" fn() -> ()>,
    pub pfnPrepWorldFrame: Option<unsafe extern "C" fn() -> ()>,
    pub pfnCreateEntitiesInRestoreList: Option<unsafe extern "C" fn(_:
                                                                        *mut SAVERESTOREDATA,
                                                                    _:
                                                                        libc::c_int,
                                                                    _:
                                                                        qboolean)
                                                   -> ()>,
    pub pfnAllocString: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                   -> string_t>,
    pub pfnMakeString: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                  -> string_t>,
    pub pfnGetString: Option<unsafe extern "C" fn(_: string_t)
                                 -> *const libc::c_char>,
    pub pfnRestoreDecal: Option<unsafe extern "C" fn(_: *mut decallist_s,
                                                     _: *mut edict_t,
                                                     _: qboolean)
                                    -> libc::c_int>,
    pub PM_PlayerTouch: Option<unsafe extern "C" fn(_: *mut playermove_s,
                                                    _: *mut edict_t) -> ()>,
    pub Mod_ProcessUserData: Option<unsafe extern "C" fn(_: *mut model_s,
                                                         _: qboolean,
                                                         _: *const byte)
                                        -> ()>,
    pub SV_HullForBsp: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                   _: *const libc::c_float,
                                                   _: *const libc::c_float,
                                                   _: *mut libc::c_float)
                                  -> *mut libc::c_void>,
    pub SV_PlayerThink: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                    _: libc::c_float,
                                                    _: libc::c_double)
                                   -> libc::c_int>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct playermove_s {
    pub player_index: libc::c_int,
    pub server: qboolean,
    pub multiplayer: qboolean,
    pub time: libc::c_float,
    pub frametime: libc::c_float,
    pub forward: vec3_t,
    pub right: vec3_t,
    pub up: vec3_t,
    pub origin: vec3_t,
    pub angles: vec3_t,
    pub oldangles: vec3_t,
    pub velocity: vec3_t,
    pub movedir: vec3_t,
    pub basevelocity: vec3_t,
    pub view_ofs: vec3_t,
    pub flDuckTime: libc::c_float,
    pub bInDuck: qboolean,
    pub flTimeStepSound: libc::c_int,
    pub iStepLeft: libc::c_int,
    pub flFallVelocity: libc::c_float,
    pub punchangle: vec3_t,
    pub flSwimTime: libc::c_float,
    pub flNextPrimaryAttack: libc::c_float,
    pub effects: libc::c_int,
    pub flags: libc::c_int,
    pub usehull: libc::c_int,
    pub gravity: libc::c_float,
    pub friction: libc::c_float,
    pub oldbuttons: libc::c_int,
    pub waterjumptime: libc::c_float,
    pub dead: qboolean,
    pub deadflag: libc::c_int,
    pub spectator: libc::c_int,
    pub movetype: libc::c_int,
    pub onground: libc::c_int,
    pub waterlevel: libc::c_int,
    pub watertype: libc::c_int,
    pub oldwaterlevel: libc::c_int,
    pub sztexturename: [libc::c_char; 256],
    pub chtexturetype: libc::c_char,
    pub maxspeed: libc::c_float,
    pub clientmaxspeed: libc::c_float,
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
    pub numphysent: libc::c_int,
    pub physents: [physent_t; 600],
    pub nummoveent: libc::c_int,
    pub moveents: [physent_t; 64],
    pub numvisent: libc::c_int,
    pub visents: [physent_t; 600],
    pub cmd: usercmd_t,
    pub numtouch: libc::c_int,
    pub touchindex: [pmtrace_t; 600],
    pub physinfo: [libc::c_char; 256],
    pub movevars: *mut movevars_s,
    pub player_mins: [vec3_t; 4],
    pub player_maxs: [vec3_t; 4],
    pub PM_Info_ValueForKey: Option<unsafe extern "C" fn(_:
                                                             *const libc::c_char,
                                                         _:
                                                             *const libc::c_char)
                                        -> *const libc::c_char>,
    pub PM_Particle: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                 _: libc::c_int,
                                                 _: libc::c_float,
                                                 _: libc::c_int,
                                                 _: libc::c_int) -> ()>,
    pub PM_TestPlayerPosition: Option<unsafe extern "C" fn(_:
                                                               *mut libc::c_float,
                                                           _: *mut pmtrace_t)
                                          -> libc::c_int>,
    pub Con_NPrintf: Option<unsafe extern "C" fn(_: libc::c_int,
                                                 _: *const libc::c_char,
                                                 _: ...) -> ()>,
    pub Con_DPrintf: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                 _: ...) -> ()>,
    pub Con_Printf: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                _: ...) -> ()>,
    pub Sys_FloatTime: Option<unsafe extern "C" fn() -> libc::c_double>,
    pub PM_StuckTouch: Option<unsafe extern "C" fn(_: libc::c_int,
                                                   _: *mut pmtrace_t) -> ()>,
    pub PM_PointContents: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                      _: *mut libc::c_int)
                                     -> libc::c_int>,
    pub PM_TruePointContents: Option<unsafe extern "C" fn(_:
                                                              *mut libc::c_float)
                                         -> libc::c_int>,
    pub PM_HullPointContents: Option<unsafe extern "C" fn(_: *mut hull_s,
                                                          _: libc::c_int,
                                                          _:
                                                              *mut libc::c_float)
                                         -> libc::c_int>,
    pub PM_PlayerTrace: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                    _: *mut libc::c_float,
                                                    _: libc::c_int,
                                                    _: libc::c_int)
                                   -> pmtrace_t>,
    pub PM_TraceLine: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                  _: *mut libc::c_float,
                                                  _: libc::c_int,
                                                  _: libc::c_int,
                                                  _: libc::c_int)
                                 -> *mut pmtrace_s>,
    pub RandomLong: Option<unsafe extern "C" fn(_: libc::c_int,
                                                _: libc::c_int)
                               -> libc::c_int>,
    pub RandomFloat: Option<unsafe extern "C" fn(_: libc::c_float,
                                                 _: libc::c_float)
                                -> libc::c_float>,
    pub PM_GetModelType: Option<unsafe extern "C" fn(_: *mut model_s)
                                    -> libc::c_int>,
    pub PM_GetModelBounds: Option<unsafe extern "C" fn(_: *mut model_s,
                                                       _: *mut libc::c_float,
                                                       _: *mut libc::c_float)
                                      -> ()>,
    pub PM_HullForBsp: Option<unsafe extern "C" fn(_: *mut physent_t,
                                                   _: *mut libc::c_float)
                                  -> *mut libc::c_void>,
    pub PM_TraceModel: Option<unsafe extern "C" fn(_: *mut physent_t,
                                                   _: *mut libc::c_float,
                                                   _: *mut libc::c_float,
                                                   _: *mut trace_t)
                                  -> libc::c_float>,
    pub COM_FileSize: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                 -> libc::c_int>,
    pub COM_LoadFile: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                  _: libc::c_int,
                                                  _: *mut libc::c_int)
                                 -> *mut byte>,
    pub COM_FreeFile: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                 -> ()>,
    pub memfgets: Option<unsafe extern "C" fn(_: *mut byte, _: libc::c_int,
                                              _: *mut libc::c_int,
                                              _: *mut libc::c_char,
                                              _: libc::c_int)
                             -> *mut libc::c_char>,
    pub runfuncs: qboolean,
    pub PM_PlaySound: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _: *const libc::c_char,
                                                  _: libc::c_float,
                                                  _: libc::c_float,
                                                  _: libc::c_int,
                                                  _: libc::c_int) -> ()>,
    pub PM_TraceTexture: Option<unsafe extern "C" fn(_: libc::c_int,
                                                     _: *mut libc::c_float,
                                                     _: *mut libc::c_float)
                                    -> *const libc::c_char>,
    pub PM_PlaybackEventFull: Option<unsafe extern "C" fn(_: libc::c_int,
                                                          _: libc::c_int,
                                                          _: libc::c_ushort,
                                                          _: libc::c_float,
                                                          _:
                                                              *mut libc::c_float,
                                                          _:
                                                              *mut libc::c_float,
                                                          _: libc::c_float,
                                                          _: libc::c_float,
                                                          _: libc::c_int,
                                                          _: libc::c_int,
                                                          _: libc::c_int,
                                                          _: libc::c_int)
                                         -> ()>,
    pub PM_PlayerTraceEx: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                      _: *mut libc::c_float,
                                                      _: libc::c_int,
                                                      _:
                                                          Option<unsafe extern "C" fn(_:
                                                                                          *mut physent_t)
                                                                     ->
                                                                         libc::c_int>)
                                     -> pmtrace_t>,
    pub PM_TestPlayerPositionEx: Option<unsafe extern "C" fn(_:
                                                                 *mut libc::c_float,
                                                             _:
                                                                 *mut pmtrace_t,
                                                             _:
                                                                 Option<unsafe extern "C" fn(_:
                                                                                                 *mut physent_t)
                                                                            ->
                                                                                libc::c_int>)
                                            -> libc::c_int>,
    pub PM_TraceLineEx: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                    _: *mut libc::c_float,
                                                    _: libc::c_int,
                                                    _: libc::c_int,
                                                    _:
                                                        Option<unsafe extern "C" fn(_:
                                                                                        *mut physent_t)
                                                                   ->
                                                                       libc::c_int>)
                                   -> *mut pmtrace_s>,
    pub PM_TraceSurface: Option<unsafe extern "C" fn(_: libc::c_int,
                                                     _: *mut libc::c_float,
                                                     _: *mut libc::c_float)
                                    -> *mut msurface_s>,
}
pub type physent_t = physent_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct physent_s {
    pub name: [libc::c_char; 32],
    pub player: libc::c_int,
    pub origin: vec3_t,
    pub model: *mut model_s,
    pub studiomodel: *mut model_s,
    pub mins: vec3_t,
    pub maxs: vec3_t,
    pub info: libc::c_int,
    pub angles: vec3_t,
    pub solid: libc::c_int,
    pub skin: libc::c_int,
    pub rendermode: libc::c_int,
    pub frame: libc::c_float,
    pub sequence: libc::c_int,
    pub controller: [byte; 4],
    pub blending: [byte; 2],
    pub movetype: libc::c_int,
    pub takedamage: libc::c_int,
    pub blooddecal: libc::c_int,
    pub team: libc::c_int,
    pub classnumber: libc::c_int,
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
pub struct pmtrace_s {
    pub allsolid: qboolean,
    pub startsolid: qboolean,
    pub inopen: qboolean,
    pub inwater: qboolean,
    pub fraction: libc::c_float,
    pub endpos: vec3_t,
    pub plane: pmplane_t,
    pub ent: libc::c_int,
    pub deltavelocity: vec3_t,
    pub hitgroup: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pmplane_t {
    pub normal: vec3_t,
    pub dist: libc::c_float,
}
pub type pmtrace_t = pmtrace_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct movevars_s {
    pub gravity: libc::c_float,
    pub stopspeed: libc::c_float,
    pub maxspeed: libc::c_float,
    pub spectatormaxspeed: libc::c_float,
    pub accelerate: libc::c_float,
    pub airaccelerate: libc::c_float,
    pub wateraccelerate: libc::c_float,
    pub friction: libc::c_float,
    pub edgefriction: libc::c_float,
    pub waterfriction: libc::c_float,
    pub entgravity: libc::c_float,
    pub bounce: libc::c_float,
    pub stepsize: libc::c_float,
    pub maxvelocity: libc::c_float,
    pub zmax: libc::c_float,
    pub waveHeight: libc::c_float,
    pub footsteps: qboolean,
    pub skyName: [libc::c_char; 32],
    pub rollangle: libc::c_float,
    pub rollspeed: libc::c_float,
    pub skycolor_r: libc::c_float,
    pub skycolor_g: libc::c_float,
    pub skycolor_b: libc::c_float,
    pub skyvec_x: libc::c_float,
    pub skyvec_y: libc::c_float,
    pub skyvec_z: libc::c_float,
    pub features: libc::c_int,
    pub fog_settings: libc::c_int,
    pub wateralpha: libc::c_float,
    pub skydir_x: libc::c_float,
    pub skydir_y: libc::c_float,
    pub skydir_z: libc::c_float,
    pub skyangle: libc::c_float,
}
pub type SAVERESTOREDATA = saverestore_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct saverestore_s {
    pub pBaseData: *mut libc::c_char,
    pub pCurrentData: *mut libc::c_char,
    pub size: libc::c_int,
    pub bufferSize: libc::c_int,
    pub tokenSize: libc::c_int,
    pub tokenCount: libc::c_int,
    pub pTokens: *mut *mut libc::c_char,
    pub currentIndex: libc::c_int,
    pub tableCount: libc::c_int,
    pub connectionCount: libc::c_int,
    pub pTable: *mut ENTITYTABLE,
    pub levelList: [LEVELLIST; 16],
    pub fUseLandmark: libc::c_int,
    pub szLandmarkName: [libc::c_char; 20],
    pub vecLandmarkOffset: vec3_t,
    pub time: libc::c_float,
    pub szCurrentMapName: [libc::c_char; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LEVELLIST {
    pub mapName: [libc::c_char; 32],
    pub landmarkName: [libc::c_char; 32],
    pub pentLandmark: *mut edict_t,
    pub vecLandmarkOrigin: vec3_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ENTITYTABLE {
    pub id: libc::c_int,
    pub pent: *mut edict_t,
    pub location: libc::c_int,
    pub size: libc::c_int,
    pub flags: libc::c_int,
    pub classname: string_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct svgame_static_t {
    pub msg_name: *const libc::c_char,
    pub msg: [sv_user_message_t; 197],
    pub msg_size_index: libc::c_int,
    pub msg_realsize: libc::c_int,
    pub msg_index: libc::c_int,
    pub msg_dest: libc::c_int,
    pub msg_started: qboolean,
    pub msg_ent: *mut edict_t,
    pub msg_org: vec3_t,
    pub hInstance: *mut libc::c_void,
    pub config_executed: qboolean,
    pub edicts: *mut edict_t,
    pub numEntities: libc::c_int,
    pub movevars: movevars_t,
    pub oldmovevars: movevars_t,
    pub pmove: *mut playermove_t,
    pub interp: [sv_interp_t; 32],
    pub pushed: [sv_pushed_t; 256],
    pub globals: *mut globalvars_t,
    pub dllFuncs: DLL_FUNCTIONS,
    pub dllFuncs2: NEW_DLL_FUNCTIONS,
    pub physFuncs: physics_interface_t,
    pub mempool: poolhandle_t,
    pub stringspool: poolhandle_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NEW_DLL_FUNCTIONS {
    pub pfnOnFreeEntPrivateData: Option<unsafe extern "C" fn(_: *mut edict_t)
                                            -> ()>,
    pub pfnGameShutdown: Option<unsafe extern "C" fn() -> ()>,
    pub pfnShouldCollide: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                      _: *mut edict_t)
                                     -> libc::c_int>,
    pub pfnCvarValue: Option<unsafe extern "C" fn(_: *const edict_t,
                                                  _: *const libc::c_char)
                                 -> ()>,
    pub pfnCvarValue2: Option<unsafe extern "C" fn(_: *const edict_t,
                                                   _: libc::c_int,
                                                   _: *const libc::c_char,
                                                   _: *const libc::c_char)
                                  -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DLL_FUNCTIONS {
    pub pfnGameInit: Option<unsafe extern "C" fn() -> ()>,
    pub pfnSpawn: Option<unsafe extern "C" fn(_: *mut edict_t)
                             -> libc::c_int>,
    pub pfnThink: Option<unsafe extern "C" fn(_: *mut edict_t) -> ()>,
    pub pfnUse: Option<unsafe extern "C" fn(_: *mut edict_t, _: *mut edict_t)
                           -> ()>,
    pub pfnTouch: Option<unsafe extern "C" fn(_: *mut edict_t,
                                              _: *mut edict_t) -> ()>,
    pub pfnBlocked: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                _: *mut edict_t) -> ()>,
    pub pfnKeyValue: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                 _: *mut KeyValueData) -> ()>,
    pub pfnSave: Option<unsafe extern "C" fn(_: *mut edict_t,
                                             _: *mut SAVERESTOREDATA) -> ()>,
    pub pfnRestore: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                _: *mut SAVERESTOREDATA,
                                                _: libc::c_int)
                               -> libc::c_int>,
    pub pfnSetAbsBox: Option<unsafe extern "C" fn(_: *mut edict_t) -> ()>,
    pub pfnSaveWriteFields: Option<unsafe extern "C" fn(_:
                                                            *mut SAVERESTOREDATA,
                                                        _:
                                                            *const libc::c_char,
                                                        _: *mut libc::c_void,
                                                        _:
                                                            *mut TYPEDESCRIPTION,
                                                        _: libc::c_int)
                                       -> ()>,
    pub pfnSaveReadFields: Option<unsafe extern "C" fn(_:
                                                           *mut SAVERESTOREDATA,
                                                       _: *const libc::c_char,
                                                       _: *mut libc::c_void,
                                                       _:
                                                           *mut TYPEDESCRIPTION,
                                                       _: libc::c_int) -> ()>,
    pub pfnSaveGlobalState: Option<unsafe extern "C" fn(_:
                                                            *mut SAVERESTOREDATA)
                                       -> ()>,
    pub pfnRestoreGlobalState: Option<unsafe extern "C" fn(_:
                                                               *mut SAVERESTOREDATA)
                                          -> ()>,
    pub pfnResetGlobalState: Option<unsafe extern "C" fn() -> ()>,
    pub pfnClientConnect: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                      _: *const libc::c_char,
                                                      _: *const libc::c_char,
                                                      _: *mut libc::c_char)
                                     -> qboolean>,
    pub pfnClientDisconnect: Option<unsafe extern "C" fn(_: *mut edict_t)
                                        -> ()>,
    pub pfnClientKill: Option<unsafe extern "C" fn(_: *mut edict_t) -> ()>,
    pub pfnClientPutInServer: Option<unsafe extern "C" fn(_: *mut edict_t)
                                         -> ()>,
    pub pfnClientCommand: Option<unsafe extern "C" fn(_: *mut edict_t) -> ()>,
    pub pfnClientUserInfoChanged: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                              _:
                                                                  *mut libc::c_char)
                                             -> ()>,
    pub pfnServerActivate: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                       _: libc::c_int,
                                                       _: libc::c_int) -> ()>,
    pub pfnServerDeactivate: Option<unsafe extern "C" fn() -> ()>,
    pub pfnPlayerPreThink: Option<unsafe extern "C" fn(_: *mut edict_t)
                                      -> ()>,
    pub pfnPlayerPostThink: Option<unsafe extern "C" fn(_: *mut edict_t)
                                       -> ()>,
    pub pfnStartFrame: Option<unsafe extern "C" fn() -> ()>,
    pub pfnParmsNewLevel: Option<unsafe extern "C" fn() -> ()>,
    pub pfnParmsChangeLevel: Option<unsafe extern "C" fn() -> ()>,
    pub pfnGetGameDescription: Option<unsafe extern "C" fn()
                                          -> *const libc::c_char>,
    pub pfnPlayerCustomization: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                            _:
                                                                *mut customization_t)
                                           -> ()>,
    pub pfnSpectatorConnect: Option<unsafe extern "C" fn(_: *mut edict_t)
                                        -> ()>,
    pub pfnSpectatorDisconnect: Option<unsafe extern "C" fn(_: *mut edict_t)
                                           -> ()>,
    pub pfnSpectatorThink: Option<unsafe extern "C" fn(_: *mut edict_t)
                                      -> ()>,
    pub pfnSys_Error: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                 -> ()>,
    pub pfnPM_Move: Option<unsafe extern "C" fn(_: *mut playermove_s,
                                                _: qboolean) -> ()>,
    pub pfnPM_Init: Option<unsafe extern "C" fn(_: *mut playermove_s) -> ()>,
    pub pfnPM_FindTextureType: Option<unsafe extern "C" fn(_:
                                                               *mut libc::c_char)
                                          -> libc::c_char>,
    pub pfnSetupVisibility: Option<unsafe extern "C" fn(_: *mut edict_s,
                                                        _: *mut edict_s,
                                                        _:
                                                            *mut *mut libc::c_uchar,
                                                        _:
                                                            *mut *mut libc::c_uchar)
                                       -> ()>,
    pub pfnUpdateClientData: Option<unsafe extern "C" fn(_: *const edict_s,
                                                         _: libc::c_int,
                                                         _: *mut clientdata_s)
                                        -> ()>,
    pub pfnAddToFullPack: Option<unsafe extern "C" fn(_: *mut entity_state_s,
                                                      _: libc::c_int,
                                                      _: *mut edict_t,
                                                      _: *mut edict_t,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: *mut libc::c_uchar)
                                     -> libc::c_int>,
    pub pfnCreateBaseline: Option<unsafe extern "C" fn(_: libc::c_int,
                                                       _: libc::c_int,
                                                       _: *mut entity_state_s,
                                                       _: *mut edict_s,
                                                       _: libc::c_int,
                                                       _: *mut vec_t,
                                                       _: *mut vec_t) -> ()>,
    pub pfnRegisterEncoders: Option<unsafe extern "C" fn() -> ()>,
    pub pfnGetWeaponData: Option<unsafe extern "C" fn(_: *mut edict_s,
                                                      _: *mut weapon_data_s)
                                     -> libc::c_int>,
    pub pfnCmdStart: Option<unsafe extern "C" fn(_: *const edict_t,
                                                 _: *const usercmd_s,
                                                 _: libc::c_uint) -> ()>,
    pub pfnCmdEnd: Option<unsafe extern "C" fn(_: *const edict_t) -> ()>,
    pub pfnConnectionlessPacket: Option<unsafe extern "C" fn(_:
                                                                 *const netadr_s,
                                                             _:
                                                                 *const libc::c_char,
                                                             _:
                                                                 *mut libc::c_char,
                                                             _:
                                                                 *mut libc::c_int)
                                            -> libc::c_int>,
    pub pfnGetHullBounds: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: *mut libc::c_float,
                                                      _: *mut libc::c_float)
                                     -> libc::c_int>,
    pub pfnCreateInstancedBaselines: Option<unsafe extern "C" fn() -> ()>,
    pub pfnInconsistentFile: Option<unsafe extern "C" fn(_: *const edict_s,
                                                         _:
                                                             *const libc::c_char,
                                                         _: *mut libc::c_char)
                                        -> libc::c_int>,
    pub pfnAllowLagCompensation: Option<unsafe extern "C" fn()
                                            -> libc::c_int>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TYPEDESCRIPTION {
    pub fieldType: FIELDTYPE,
    pub fieldName: *const libc::c_char,
    pub fieldOffset: libc::c_int,
    pub fieldSize: libc::c_short,
    pub flags: libc::c_short,
}
pub type FIELDTYPE = _fieldtypes;
pub type _fieldtypes = libc::c_uint;
pub const FIELD_TYPECOUNT: _fieldtypes = 18;
pub const FIELD_SOUNDNAME: _fieldtypes = 17;
pub const FIELD_MODELNAME: _fieldtypes = 16;
pub const FIELD_TIME: _fieldtypes = 15;
pub const FIELD_CHARACTER: _fieldtypes = 14;
pub const FIELD_SHORT: _fieldtypes = 13;
pub const FIELD_BOOLEAN: _fieldtypes = 12;
pub const FIELD_FUNCTION: _fieldtypes = 11;
pub const FIELD_INTEGER: _fieldtypes = 10;
pub const FIELD_POINTER: _fieldtypes = 9;
pub const FIELD_POSITION_VECTOR: _fieldtypes = 8;
pub const FIELD_VECTOR: _fieldtypes = 7;
pub const FIELD_EDICT: _fieldtypes = 6;
pub const FIELD_EVARS: _fieldtypes = 5;
pub const FIELD_EHANDLE: _fieldtypes = 4;
pub const FIELD_CLASSPTR: _fieldtypes = 3;
pub const FIELD_ENTITY: _fieldtypes = 2;
pub const FIELD_STRING: _fieldtypes = 1;
pub const FIELD_FLOAT: _fieldtypes = 0;
pub type KeyValueData = KeyValueData_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct KeyValueData_s {
    pub szClassName: *mut libc::c_char,
    pub szKeyName: *mut libc::c_char,
    pub szValue: *mut libc::c_char,
    pub fHandled: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct globalvars_t {
    pub time: libc::c_float,
    pub frametime: libc::c_float,
    pub force_retouch: libc::c_float,
    pub mapname: string_t,
    pub startspot: string_t,
    pub deathmatch: libc::c_float,
    pub coop: libc::c_float,
    pub teamplay: libc::c_float,
    pub serverflags: libc::c_float,
    pub found_secrets: libc::c_float,
    pub v_forward: vec3_t,
    pub v_up: vec3_t,
    pub v_right: vec3_t,
    pub trace_allsolid: libc::c_float,
    pub trace_startsolid: libc::c_float,
    pub trace_fraction: libc::c_float,
    pub trace_endpos: vec3_t,
    pub trace_plane_normal: vec3_t,
    pub trace_plane_dist: libc::c_float,
    pub trace_ent: *mut edict_t,
    pub trace_inopen: libc::c_float,
    pub trace_inwater: libc::c_float,
    pub trace_hitgroup: libc::c_int,
    pub trace_flags: libc::c_int,
    pub changelevel: libc::c_int,
    pub cdAudioTrack: libc::c_int,
    pub maxClients: libc::c_int,
    pub maxEntities: libc::c_int,
    pub pStringBase: *const libc::c_char,
    pub pSaveData: *mut libc::c_void,
    pub vecLandmarkOffset: vec3_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sv_pushed_t {
    pub ent: *mut edict_t,
    pub origin: vec3_t,
    pub angles: vec3_t,
    pub fixangle: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sv_interp_t {
    pub active: qboolean,
    pub moving: qboolean,
    pub firstframe: qboolean,
    pub nointerp: qboolean,
    pub mins: vec3_t,
    pub maxs: vec3_t,
    pub curpos: vec3_t,
    pub oldpos: vec3_t,
    pub newpos: vec3_t,
    pub finalpos: vec3_t,
}
pub type playermove_t = playermove_s;
pub type movevars_t = movevars_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sv_user_message_t {
    pub name: [libc::c_char; 32],
    pub number: libc::c_int,
    pub size: libc::c_int,
}
#[inline]
unsafe extern "C" fn MSG_GetMaxBytes(mut sb: *mut sizebuf_t) -> libc::c_int {
    return (*sb).nDataBits >> 3 as libc::c_int;
}
#[inline]
unsafe extern "C" fn MSG_GetNumBitsWritten(mut sb: *mut sizebuf_t)
 -> libc::c_int {
    return (*sb).iCurBit;
}
#[inline]
unsafe extern "C" fn MSG_GetData(mut sb: *mut sizebuf_t) -> *mut byte {
    return (*sb).pData;
}
// 300 seconds
// server cvars
#[no_mangle]
pub static mut sv_lan: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_lan\x00" as *const u8 as *const libc::c_char
                                 as *mut libc::c_char,
                         string:
                             b"0\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"server is a lan server ( no heartbeat, no authentication, no non-class C addresses, 9999.0 rate, etc.\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_lan_rate: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_lan_rate\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"20000.0\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         flags: 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"rate for lan server\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_aim: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_aim\x00" as *const u8 as *const libc::c_char
                                 as *mut libc::c_char,
                         string:
                             b"1\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags:
                             (1 as libc::c_int) << 0 as libc::c_int |
                                 (1 as libc::c_int) << 2 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"auto aiming option\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_unlag: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_unlag\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"1\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"allow lag compensation on server-side\x00" as
                                 *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_maxunlag: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_maxunlag\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"0.5\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"max latency value which can be interpolated (by default ping should not exceed 500 units)\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_unlagpush: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_unlagpush\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"0.0\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"interpolation bias for unlag time\x00" as
                                 *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_unlagsamples: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_unlagsamples\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"1\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"max samples to interpolate\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut rcon_password: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"rcon_password\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"remote connect password\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_filterban: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_filterban\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"1\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"filter banned users\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_cheats: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_cheats\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"0\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: (1 as libc::c_int) << 2 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"allow cheats on server\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_instancedbaseline: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_instancedbaseline\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"1\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"allow to use instanced baselines to saves network overhead\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_contact: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_contact\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags:
                             (1 as libc::c_int) << 0 as libc::c_int |
                                 (1 as libc::c_int) << 2 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"server techincal support contact address or web-page\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_minupdaterate: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_minupdaterate\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"25.0\x00" as *const u8 as *const libc::c_char
                                 as *mut libc::c_char,
                         flags: (1 as libc::c_int) << 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"minimal value for \'cl_updaterate\' window\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_maxupdaterate: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_maxupdaterate\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"60.0\x00" as *const u8 as *const libc::c_char
                                 as *mut libc::c_char,
                         flags: (1 as libc::c_int) << 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"maximal value for \'cl_updaterate\' window\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_minrate: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_minrate\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"5000\x00" as *const u8 as *const libc::c_char
                                 as *mut libc::c_char,
                         flags: (1 as libc::c_int) << 2 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"min bandwidth rate allowed on server, 0 == unlimited\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_maxrate: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_maxrate\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"50000\x00" as *const u8 as *const libc::c_char
                                 as *mut libc::c_char,
                         flags: (1 as libc::c_int) << 2 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"max bandwidth rate allowed on server, 0 == unlimited\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_logrelay: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_logrelay\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"0\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: (1 as libc::c_int) << 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"allow log messages from remote machines to be logged on this server\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_newunit: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_newunit\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"0\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"clear level-saves from previous SP game chapter to help keep .sav file size as minimum\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_clienttrace: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_clienttrace\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"1\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: (1 as libc::c_int) << 2 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"0 = big box(Quake), 0.5 = halfsize, 1 = normal (100%), otherwise it\'s a scaling factor\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_timeout: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_timeout\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"65\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"after this many seconds without a message from a client, the client is dropped\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_failuretime: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_failuretime\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"0.5\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"after this long without a packet from client, don\'t send any more until client starts sending again\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_password: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_password\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags:
                             (1 as libc::c_int) << 2 as libc::c_int |
                                 (1 as libc::c_int) << 5 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"server password for entry into multiplayer games\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_proxies: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_proxies\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"1\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: (1 as libc::c_int) << 2 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"maximum count of allowed proxies for HLTV spectating\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_send_logos: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_send_logos\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"1\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"send custom decal logo to other players so they can view his too\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_send_resources: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_send_resources\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"1\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"allow to download missed resources for players\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_logbans: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_logbans\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"0\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"print into the server log info about player bans\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_allow_upload: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_allow_upload\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"1\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: (1 as libc::c_int) << 2 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"allow uploading custom resources on a server\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_allow_download: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_allow_download\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"1\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: (1 as libc::c_int) << 2 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"allow downloading custom resources to the client\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_uploadmax: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_uploadmax\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"0.5\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: (1 as libc::c_int) << 2 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"max size to upload custom resources (500 kB as default)\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_downloadurl: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_downloadurl\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: (1 as libc::c_int) << 5 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"location from which clients can download missing files\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_consistency: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"mp_consistency\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"1\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: (1 as libc::c_int) << 2 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"enbale consistency check in multiplayer\x00" as
                                 *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut mp_logecho: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"mp_logecho\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"1\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"log multiplayer frags to server logfile\x00" as
                                 *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut mp_logfile: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"mp_logfile\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"1\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"log multiplayer frags to console\x00" as
                                 *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_log_singleplayer: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_log_singleplayer\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"0\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: (1 as libc::c_int) << 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"allows logging in singleplayer games\x00" as
                                 *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_log_onefile: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_log_onefile\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"0\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: (1 as libc::c_int) << 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"logs server information to only one file\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
// game-related cvars
#[no_mangle]
pub static mut mapcyclefile: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"mapcyclefile\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"mapcycle.txt\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         flags: 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"name of multiplayer map cycle configuration file\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut motdfile: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"motdfile\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"motd.txt\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         flags: 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"name of \'message of the day\' file\x00" as
                                 *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut logsdir: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"logsdir\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"logs\x00" as *const u8 as *const libc::c_char
                                 as *mut libc::c_char,
                         flags: 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"place to store multiplayer logs\x00" as
                                 *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut bannedcfgfile: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"bannedcfgfile\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"banned.cfg\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         flags: 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"name of list of banned users\x00" as *const u8
                                 as *const libc::c_char as *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut deathmatch: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"deathmatch\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"0\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"deathmatch mode in multiplayer game\x00" as
                                 *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut coop: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"coop\x00" as *const u8 as *const libc::c_char
                                 as *mut libc::c_char,
                         string:
                             b"0\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"cooperative mode in multiplayer game\x00" as
                                 *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut teamplay: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"teamplay\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"0\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"team mode in multiplayer game\x00" as *const u8
                                 as *const libc::c_char as *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut skill: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"skill\x00" as *const u8 as *const libc::c_char
                                 as *mut libc::c_char,
                         string:
                             b"1\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"skill level in singleplayer game\x00" as
                                 *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut temp1: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"temp1\x00" as *const u8 as *const libc::c_char
                                 as *mut libc::c_char,
                         string:
                             b"0\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"temporary cvar that used by some mods\x00" as
                                 *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut listipcfgfile: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"listipcfgfile\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"listip.cfg\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         flags: 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"name of listip.cfg file\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut mapchangecfgfile: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"mapchangecfgfile\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"name of map change configuration file\x00" as
                                 *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
// physic-related variables
#[no_mangle]
pub static mut sv_gravity: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_gravity\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"800\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: (1 as libc::c_int) << 22 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"world gravity value\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_stopspeed: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_stopspeed\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"100\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: (1 as libc::c_int) << 22 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"how fast you come to a complete stop\x00" as
                                 *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_maxspeed: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_maxspeed\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"320\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: (1 as libc::c_int) << 22 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"maximum speed a player can accelerate to when on ground\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_spectatormaxspeed: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_spectatormaxspeed\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"500\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags:
                             (1 as libc::c_int) << 22 as libc::c_int |
                                 (1 as libc::c_int) << 8 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"maximum speed a spectator can accelerate in air\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_accelerate: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_accelerate\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"10\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: (1 as libc::c_int) << 22 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"rate at which a player accelerates to sv_maxspeed\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_airaccelerate: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_airaccelerate\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"10\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: (1 as libc::c_int) << 22 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"rate at which a player accelerates to sv_maxspeed while in the air\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_wateraccelerate: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_wateraccelerate\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"10\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: (1 as libc::c_int) << 22 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"rate at which a player accelerates to sv_maxspeed while in the water\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_friction: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_friction\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"4\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: (1 as libc::c_int) << 22 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"how fast you slow down\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_edgefriction: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"edgefriction\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"2\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: (1 as libc::c_int) << 22 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"how much you slow down when nearing a ledge you might fall off\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_waterfriction: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_waterfriction\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"1\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: (1 as libc::c_int) << 22 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"how fast you slow down in water\x00" as
                                 *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_bounce: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_bounce\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"1\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: (1 as libc::c_int) << 22 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"bounce factor for entities with MOVETYPE_BOUNCE\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_stepsize: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_stepsize\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"18\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: (1 as libc::c_int) << 22 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"how high you and NPS\'s can step up\x00" as
                                 *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_maxvelocity: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_maxvelocity\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"2000\x00" as *const u8 as *const libc::c_char
                                 as *mut libc::c_char,
                         flags:
                             (1 as libc::c_int) << 22 as libc::c_int |
                                 (1 as libc::c_int) << 8 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"max velocity for all things in the world\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_zmax: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_zmax\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"4096\x00" as *const u8 as *const libc::c_char
                                 as *mut libc::c_char,
                         flags:
                             (1 as libc::c_int) << 22 as libc::c_int |
                                 (1 as libc::c_int) << 6 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"maximum viewable distance\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_wateramp: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_wateramp\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"0\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags:
                             (1 as libc::c_int) << 22 as libc::c_int |
                                 (1 as libc::c_int) << 8 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"world waveheight factor\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_footsteps: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"mp_footsteps\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"1\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: (1 as libc::c_int) << 22 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"world gravity value\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_skyname: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_skyname\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"desert\x00" as *const u8 as *const libc::c_char
                                 as *mut libc::c_char,
                         flags:
                             (1 as libc::c_int) << 22 as libc::c_int |
                                 (1 as libc::c_int) << 8 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"skybox name (can be dynamically changed in-game)\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_rollangle: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_rollangle\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"0\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags:
                             (1 as libc::c_int) << 22 as libc::c_int |
                                 (1 as libc::c_int) << 8 as libc::c_int |
                                 (1 as libc::c_int) << 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"how much to tilt the view when strafing\x00" as
                                 *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_rollspeed: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_rollspeed\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"200\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags:
                             (1 as libc::c_int) << 22 as libc::c_int |
                                 (1 as libc::c_int) << 8 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"how much strafing is necessary to tilt the view\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_skycolor_r: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_skycolor_r\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"0\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags:
                             (1 as libc::c_int) << 22 as libc::c_int |
                                 (1 as libc::c_int) << 8 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"skylight red component value\x00" as *const u8
                                 as *const libc::c_char as *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_skycolor_g: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_skycolor_g\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"0\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags:
                             (1 as libc::c_int) << 22 as libc::c_int |
                                 (1 as libc::c_int) << 8 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"skylight green component value\x00" as
                                 *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_skycolor_b: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_skycolor_b\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"0\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags:
                             (1 as libc::c_int) << 22 as libc::c_int |
                                 (1 as libc::c_int) << 8 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"skylight blue component value\x00" as *const u8
                                 as *const libc::c_char as *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_skyvec_x: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_skyvec_x\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"0\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags:
                             (1 as libc::c_int) << 22 as libc::c_int |
                                 (1 as libc::c_int) << 8 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"skylight direction by x-axis\x00" as *const u8
                                 as *const libc::c_char as *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_skyvec_y: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_skyvec_y\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"0\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags:
                             (1 as libc::c_int) << 22 as libc::c_int |
                                 (1 as libc::c_int) << 8 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"skylight direction by y-axis\x00" as *const u8
                                 as *const libc::c_char as *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_skyvec_z: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_skyvec_z\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"0\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags:
                             (1 as libc::c_int) << 22 as libc::c_int |
                                 (1 as libc::c_int) << 8 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"skylight direction by z-axis\x00" as *const u8
                                 as *const libc::c_char as *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_wateralpha: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_wateralpha\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"1\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags:
                             (1 as libc::c_int) << 22 as libc::c_int |
                                 (1 as libc::c_int) << 8 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"world surfaces water transparency factor. 1.0 - solid, 0.0 - fully transparent\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_background_freeze: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_background_freeze\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"1\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: (1 as libc::c_int) << 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"freeze player movement on background maps (e.g. to prevent falling)\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut showtriggers: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"showtriggers\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"0\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: (1 as libc::c_int) << 30 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"debug cvar shows triggers\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_airmove: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_airmove\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"1\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: (1 as libc::c_int) << 2 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"obsolete, compatibility issues\x00" as
                                 *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_version: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_version\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: (1 as libc::c_int) << 17 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"engine version string\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut hostname: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"hostname\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags:
                             (1 as libc::c_int) << 2 as libc::c_int |
                                 (1 as libc::c_int) << 7 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"name of current host\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_fps: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sv_fps\x00" as *const u8 as *const libc::c_char
                                 as *mut libc::c_char,
                         string:
                             b"0.0\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: (1 as libc::c_int) << 2 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"server framerate\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
// gore-related cvars
#[no_mangle]
pub static mut violence_hblood: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"violence_hblood\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"1\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"draw human blood\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut violence_ablood: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"violence_ablood\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"1\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"draw alien blood\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut violence_hgibs: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"violence_hgibs\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"1\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"show human gib entities\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut violence_agibs: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"violence_agibs\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"1\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"show alien gib entities\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sv_novis: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
// disable server culling entities by vis
#[no_mangle]
pub static mut sv_pausable: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut timeout: *mut convar_t = 0 as *const convar_t as *mut convar_t;
// seconds without any message
#[no_mangle]
pub static mut sv_lighting_modulate: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut sv_maxclients: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut sv_check_errors: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut public_server: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
// should heartbeats be sent
#[no_mangle]
pub static mut sv_reconnect_limit: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
// minimum seconds between connect messages
#[no_mangle]
pub static mut sv_validate_changelevel: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut sv_sendvelocity: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut sv_hostmap: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut sv_allow_noinputdevices: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut sv_allow_touch: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut sv_allow_mouse: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut sv_allow_joystick: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut sv_allow_vr: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
//============================================================================
/*
================
SV_HasActivePlayers

returns true if server have spawned players
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_HasActivePlayers() -> qboolean {
    let mut i: libc::c_int = 0;
    // server inactive
    if svs.clients.is_null() { return false_0 }
    i = 0 as libc::c_int;
    while i < svs.maxclients {
        if (*svs.clients.offset(i as isize)).state as libc::c_uint ==
               cs_spawned as libc::c_int as libc::c_uint {
            return true_0
        }
        i += 1
    }
    return false_0;
}
/*
===================
SV_UpdateMovevars

check movevars for changes every frame
send updates to client if changed
===================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_UpdateMovevars(mut initialize: qboolean) {
    if sv.state as libc::c_uint == ss_dead as libc::c_int as libc::c_uint {
        return
    }
    if initialize as u64 == 0 && host.movevars_changed as u64 == 0 { return }
    // check range
    if sv_zmax.value < 256.0f32 {
        Cvar_SetValue(b"sv_zmax\x00" as *const u8 as *const libc::c_char,
                      256.0f32);
    }
    // clamp it right
    if host.features &
           ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint != 0 {
        if sv_zmax.value > 131070.0f32 {
            Cvar_SetValue(b"sv_zmax\x00" as *const u8 as *const libc::c_char,
                          131070.0f32); // just in case. not really need
        }
    } else if sv_zmax.value > 32767.0f32 {
        Cvar_SetValue(b"sv_zmax\x00" as *const u8 as *const libc::c_char,
                      32767.0f32); // too early
    } // oldstate changed
    svgame.movevars.gravity = sv_gravity.value;
    svgame.movevars.stopspeed = sv_stopspeed.value;
    svgame.movevars.maxspeed = sv_maxspeed.value;
    svgame.movevars.spectatormaxspeed = sv_spectatormaxspeed.value;
    svgame.movevars.accelerate = sv_accelerate.value;
    svgame.movevars.airaccelerate = sv_airaccelerate.value;
    svgame.movevars.wateraccelerate = sv_wateraccelerate.value;
    svgame.movevars.friction = sv_friction.value;
    svgame.movevars.edgefriction = sv_edgefriction.value;
    svgame.movevars.waterfriction = sv_waterfriction.value;
    svgame.movevars.bounce = sv_bounce.value;
    svgame.movevars.stepsize = sv_stepsize.value;
    svgame.movevars.maxvelocity = sv_maxvelocity.value;
    svgame.movevars.zmax = sv_zmax.value;
    svgame.movevars.waveHeight = sv_wateramp.value;
    Q_strncpy(svgame.movevars.skyName.as_mut_ptr(), sv_skyname.string,
              ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong);
    svgame.movevars.footsteps = sv_footsteps.value as qboolean;
    svgame.movevars.rollangle = sv_rollangle.value;
    svgame.movevars.rollspeed = sv_rollspeed.value;
    svgame.movevars.skycolor_r = sv_skycolor_r.value;
    svgame.movevars.skycolor_g = sv_skycolor_g.value;
    svgame.movevars.skycolor_b = sv_skycolor_b.value;
    svgame.movevars.skyvec_x = sv_skyvec_x.value;
    svgame.movevars.skyvec_y = sv_skyvec_y.value;
    svgame.movevars.skyvec_z = sv_skyvec_z.value;
    svgame.movevars.wateralpha = sv_wateralpha.value;
    svgame.movevars.features = host.features as libc::c_int;
    svgame.movevars.entgravity = 1.0f32;
    if initialize as u64 != 0 { return }
    if MSG_WriteDeltaMovevars(&mut sv.reliable_datagram,
                              &mut svgame.oldmovevars, &mut svgame.movevars)
           as u64 != 0 {
        memcpy(&mut svgame.oldmovevars as *mut movevars_t as
                   *mut libc::c_void,
               &mut svgame.movevars as *mut movevars_t as *const libc::c_void,
               ::std::mem::size_of::<movevars_t>() as libc::c_ulong);
    }
    host.movevars_changed = false_0;
}
/*
=================
SV_CheckCmdTimes
=================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_CheckCmdTimes() {
    let mut cl: *mut sv_client_t = 0 as *mut sv_client_t;
    static mut lastreset: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut diff: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    if sv_fps.value != 0.0f32 {
        if sv_fps.value < 20.0f32 {
            Cvar_SetValue(b"sv_fps\x00" as *const u8 as *const libc::c_char,
                          20.0f32);
        }
        if sv_fps.value > 200.0f32 {
            Cvar_SetValue(b"sv_fps\x00" as *const u8 as *const libc::c_char,
                          200.0f32);
        }
    }
    if Host_IsLocalGame() as u64 != 0 { return }
    if host.realtime - lastreset < 1.0f64 { return }
    lastreset = host.realtime;
    i = 0 as libc::c_int;
    cl = svs.clients;
    while i < svs.maxclients {
        if !((*cl).state as libc::c_uint !=
                 cs_spawned as libc::c_int as libc::c_uint) {
            if (*cl).connecttime == 0.0f64 {
                (*cl).connecttime = host.realtime
            }
            diff =
                ((*cl).connecttime + (*cl).cmdtime - host.realtime) as
                    libc::c_float;
            if diff > (*net_clockwindow).value {
                (*cl).ignorecmdtime =
                    (*net_clockwindow).value as libc::c_double +
                        host.realtime;
                (*cl).cmdtime = host.realtime - (*cl).connecttime
            } else if diff < -(*net_clockwindow).value {
                (*cl).cmdtime = host.realtime - (*cl).connecttime
            }
        }
        i += 1;
        cl = cl.offset(1)
    };
}
/*
=================
SV_ProcessFile

process incoming file (customization)
=================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_ProcessFile(mut cl: *mut sv_client_t,
                                        mut filename: *const libc::c_char) {
    let mut pList: *mut customization_t = 0 as *mut customization_t;
    let mut resource: *mut resource_t = 0 as *mut resource_t;
    let mut next: *mut resource_t = 0 as *mut resource_t;
    let mut md5: [byte; 16] = [0; 16];
    let mut bFound: qboolean = false_0;
    let mut bError: qboolean = false_0;
    if *filename.offset(0 as libc::c_int as isize) as libc::c_int !=
           '!' as i32 {
        Con_Printf(b"Ignoring non-customization file upload of %s\n\x00" as
                       *const u8 as *const libc::c_char, filename);
        return
    }
    COM_HexConvert(filename.offset(4 as libc::c_int as isize),
                   32 as libc::c_int, md5.as_mut_ptr());
    resource = (*cl).resourcesneeded.pNext;
    while resource != &mut (*cl).resourcesneeded as *mut resource_t {
        next = (*resource).pNext;
        if memcmp((*resource).rgucMD5_hash.as_mut_ptr() as
                      *const libc::c_void,
                  md5.as_mut_ptr() as *const libc::c_void,
                  16 as libc::c_int as libc::c_ulong) == 0 {
            break ;
        }
        resource = next
    }
    if resource == &mut (*cl).resourcesneeded as *mut resource_t {
        Con_Printf(b"SV_ProcessFile:  Unrequested decal\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    if (*resource).nDownloadSize != (*cl).netchan.tempbuffersize {
        Con_Printf(b"Downloaded %i bytes for purported %i byte file\n\x00" as
                       *const u8 as *const libc::c_char,
                   (*cl).netchan.tempbuffersize, (*resource).nDownloadSize);
        return
    }
    HPAK_AddLump(true_0,
                 b"custom.hpk\x00" as *const u8 as *const libc::c_char,
                 resource, (*cl).netchan.tempbuffer as *mut byte,
                 0 as *mut file_t);
    (*resource).ucFlags =
        ((*resource).ucFlags as libc::c_int &
             !((1 as libc::c_int) << 1 as libc::c_int)) as libc::c_uchar;
    SV_MoveToOnHandList(cl, resource);
    bError = false_0;
    bFound = false_0;
    pList = (*cl).customdata.pNext;
    while !pList.is_null() {
        if memcmp((*pList).resource.rgucMD5_hash.as_mut_ptr() as
                      *const libc::c_void,
                  (*resource).rgucMD5_hash.as_mut_ptr() as
                      *const libc::c_void, 16 as libc::c_int as libc::c_ulong)
               == 0 {
            bFound = true_0;
            break ;
        } else { pList = (*pList).pNext }
    }
    if bFound as u64 == 0 {
        if COM_CreateCustomization(&mut (*cl).customdata, resource,
                                   -(1 as libc::c_int),
                                   (1 as libc::c_int) << 0 as libc::c_int |
                                       (1 as libc::c_int) << 1 as libc::c_int
                                       |
                                       (1 as libc::c_int) << 2 as libc::c_int,
                                   0 as *mut *mut customization_t,
                                   0 as *mut libc::c_int) as u64 == 0 {
            bError = true_0
        }
    } else {
        Con_DPrintf(b"Duplicate resource received and ignored.\n\x00" as
                        *const u8 as *const libc::c_char);
    }
    if bError as u64 != 0 {
        Con_Printf(b"^1Error:^7 parsing custom decal from %s\n\x00" as
                       *const u8 as *const libc::c_char,
                   (*cl).name.as_mut_ptr());
    };
}
/*
=================
SV_ReadPackets
=================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_ReadPackets() {
    let mut cl: *mut sv_client_t = 0 as *mut sv_client_t;
    let mut i: libc::c_int = 0;
    let mut qport: libc::c_int = 0;
    let mut curSize: size_t = 0;
    while NET_GetPacket(NS_SERVER, &mut net_from,
                        net_message_buffer.as_mut_ptr(), &mut curSize) as u64
              != 0 {
        MSG_InitExt(&mut net_message,
                    b"ClientPacket\x00" as *const u8 as *const libc::c_char,
                    net_message_buffer.as_mut_ptr() as *mut libc::c_void,
                    curSize as libc::c_int, -(1 as libc::c_int));
        // check for connectionless packet (0xffffffff) first
        if MSG_GetMaxBytes(&mut net_message) >= 4 as libc::c_int &&
               *(net_message.pData as *mut libc::c_int) == -(1 as libc::c_int)
           {
            if svs.initialized as u64 == 0 {
                let mut args: *mut libc::c_char =
                    0 as *mut libc::c_char; // skip the -1 marker
                let mut c: *const libc::c_char = 0 as *const libc::c_char;
                MSG_Clear(&mut net_message);
                MSG_ReadLong(&mut net_message);
                args = MSG_ReadStringExt(&mut net_message, true_0);
                Cmd_TokenizeString(args);
                c = Cmd_Argv(0 as libc::c_int);
                if Q_strncmp(c,
                             b"rcon\x00" as *const u8 as *const libc::c_char,
                             99999 as libc::c_int) == 0 {
                    SV_RemoteCommand(net_from, &mut net_message);
                }
            } else { SV_ConnectionlessPacket(net_from, &mut net_message); }
        } else {
            // read the qport out of the message so we can fix up
		// stupid address translating routers
            MSG_Clear(&mut net_message); // sequence number
            MSG_ReadLong(&mut net_message); // sequence number
            MSG_ReadLong(&mut net_message);
            qport = MSG_ReadShort(&mut net_message) & 0xffff as libc::c_int;
            // check for packets from connected clients
            i = 0 as libc::c_int; // reply at end of frame
            sv.current_client = svs.clients;
            while i < svs.maxclients {
                cl = sv.current_client;
                if !((*cl).state as libc::c_uint ==
                         cs_free as libc::c_int as libc::c_uint ||
                         (*cl).flags & (1 as libc::c_uint) << 7 as libc::c_int
                             != 0) {
                    if !(NET_CompareBaseAdr(net_from,
                                            (*cl).netchan.remote_address) as
                             u64 == 0) {
                        if !((*cl).netchan.qport != qport) {
                            if (*cl).netchan.remote_address.port as
                                   libc::c_int != net_from.port as libc::c_int
                               {
                                (*cl).netchan.remote_address.port =
                                    net_from.port
                            }
                            if Netchan_Process(&mut (*cl).netchan,
                                               &mut net_message) as u64 != 0 {
                                if svs.maxclients == 1 as libc::c_int &&
                                       (*host_limitlocal).value == 0. ||
                                       (*cl).state as libc::c_uint !=
                                           cs_spawned as libc::c_int as
                                               libc::c_uint {
                                    (*cl).flags =
                                        (*cl).flags |
                                            (1 as libc::c_uint) <<
                                                3 as libc::c_int
                                }
                                // this is a valid, sequenced packet, so process it
                                if !(*cl).frames.is_null() &&
                                       (*cl).state as libc::c_uint !=
                                           cs_zombie as libc::c_int as
                                               libc::c_uint {
                                    SV_ExecuteClientMessage(cl,
                                                            &mut net_message);
                                    (*svgame.globals).frametime =
                                        sv.frametime;
                                    (*svgame.globals).time =
                                        sv.time as libc::c_float
                                }
                            }
                            // fragmentation/reassembly sending takes priority over all game messages, want this in the future?
                            if Netchan_IncomingReady(&mut (*cl).netchan) as
                                   u64 != 0 {
                                if Netchan_CopyNormalFragments(&mut (*cl).netchan,
                                                               &mut net_message,
                                                               &mut curSize)
                                       as u64 != 0 {
                                    MSG_InitExt(&mut net_message,
                                                b"ClientPacket\x00" as
                                                    *const u8 as
                                                    *const libc::c_char,
                                                net_message_buffer.as_mut_ptr()
                                                    as *mut libc::c_void,
                                                curSize as libc::c_int,
                                                -(1 as
                                                      libc::c_int)); // reply at end of frame
                                    if svs.maxclients == 1 as libc::c_int &&
                                           (*host_limitlocal).value == 0. ||
                                           (*cl).state as libc::c_uint !=
                                               cs_spawned as libc::c_int as
                                                   libc::c_uint {
                                        (*cl).flags =
                                            (*cl).flags |
                                                (1 as libc::c_uint) <<
                                                    3 as libc::c_int
                                    }
                                    // this is a valid, sequenced packet, so process it
                                    if !(*cl).frames.is_null() &&
                                           (*cl).state as libc::c_uint !=
                                               cs_zombie as libc::c_int as
                                                   libc::c_uint {
                                        SV_ExecuteClientMessage(cl,
                                                                &mut net_message);
                                        (*svgame.globals).frametime =
                                            sv.frametime;
                                        (*svgame.globals).time =
                                            sv.time as libc::c_float
                                    }
                                }
                                if Netchan_CopyFileFragments(&mut (*cl).netchan,
                                                             &mut net_message)
                                       as u64 != 0 {
                                    SV_ProcessFile(cl,
                                                   (*cl).netchan.incomingfilename.as_mut_ptr());
                                }
                            }
                            break ;
                        }
                    }
                }
                i += 1;
                sv.current_client = sv.current_client.offset(1)
            }
            (i) != svs.maxclients;
        }
    }
    sv.current_client = 0 as *mut sv_client_s;
}
/*
==================
SV_CheckTimeouts

If a packet has not been received from a client for timeout->value
seconds, drop the conneciton.  Server frames are used instead of
realtime to avoid dropping the local client while debugging.

When a client is normally dropped, the sv_client_t goes into a zombie state
for a few seconds to make sure any final reliable message gets resent
if necessary
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_CheckTimeouts() {
    let mut cl: *mut sv_client_t = 0 as *mut sv_client_t;
    let mut droppoint: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut numclients: libc::c_int = 0 as libc::c_int;
    droppoint = host.realtime - (*timeout).value as libc::c_double;
    i = 0 as libc::c_int;
    cl = svs.clients;
    while i < svs.maxclients {
        if (*cl).state as libc::c_uint >=
               cs_connected as libc::c_int as libc::c_uint {
            if !(*cl).edict.is_null() &&
                   (*(*cl).edict).v.flags as libc::c_uint &
                       ((1 as libc::c_uint) << 26 as libc::c_int |
                            (1 as libc::c_uint) << 13 as libc::c_int) == 0 {
                numclients += 1
            }
        }
        // fake clients do not timeout
        if !((*cl).flags & (1 as libc::c_uint) << 7 as libc::c_int != 0) {
            // FIXME: get rid of the zombie state
            if (*cl).state as libc::c_uint ==
                   cs_zombie as libc::c_int as libc::c_uint {
                (*cl).state = cs_free
            } else if ((*cl).state as libc::c_uint ==
                           cs_connected as libc::c_int as libc::c_uint ||
                           (*cl).state as libc::c_uint ==
                               cs_spawned as libc::c_int as libc::c_uint) &&
                          (*cl).netchan.last_received < droppoint {
                if NET_IsLocalAddress((*cl).netchan.remote_address) as u64 ==
                       0 {
                    SV_BroadcastPrintf(0 as *mut sv_client_s,
                                       b"%s timed out\n\x00" as *const u8 as
                                           *const libc::c_char,
                                       (*cl).name.as_mut_ptr());
                    SV_DropClient(cl, false_0);
                    (*cl).state = cs_free // can now be reused
                    // don't bother with zombie state
                }
            }
        }
        i += 1;
        cl = cl.offset(1)
    }
    if svs.maxclients > 1 as libc::c_int && sv.paused as libc::c_uint != 0 &&
           numclients == 0 {
        // nobody left, unpause the server
        SV_TogglePause(b"Pause released since no players are left.\x00" as
                           *const u8 as *const libc::c_char);
    };
}
/*
================
SV_PrepWorldFrame

This has to be done before the world logic, because
player processing happens outside RunWorldFrame
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_PrepWorldFrame() {
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < svgame.numEntities {
        ent = SV_EdictNum(i);
        if !((*ent).free as u64 != 0) {
            (*ent).v.effects =
                (*ent).v.effects & !(2 as libc::c_int | 32 as libc::c_int)
        }
        i += 1
    }
    if svgame.physFuncs.pfnPrepWorldFrame.is_some() {
        svgame.physFuncs.pfnPrepWorldFrame.expect("non-null function pointer")();
    };
}
/*
=================
SV_IsSimulating
=================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_IsSimulating() -> qboolean {
    if sv.background as libc::c_uint != 0 && SV_Active() as libc::c_uint != 0
           && CL_Active() != 0 {
        if CL_IsInConsole() as u64 != 0 { return false_0 }
        return true_0
        // force simulating for background map
    } // always active for dedicated servers
    if host.type_0 == HOST_DEDICATED as libc::c_int as libc::c_uint {
        return true_0
    }
    if SV_HasActivePlayers() as u64 == 0 { return false_0 }
    // allow to freeze everything in singleplayer
    if svs.maxclients <= 1 as libc::c_int &&
           sv.playersonly as libc::c_uint != 0 {
        return false_0
    }
    if sv.paused as u64 == 0 && CL_IsInGame() as libc::c_uint != 0 {
        return true_0
    }
    return false_0;
}
/*
=================
SV_RunGameFrame
=================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_RunGameFrame() -> qboolean {
    sv.simulating = SV_IsSimulating(); // FP issues
    if sv.simulating as u64 == 0 { return true_0 }
    if sv_fps.value != 0.0f32 {
        let mut fps: libc::c_double =
            1.0f64 / (sv_fps.value - 0.01f32) as libc::c_double;
        let mut numFrames: libc::c_int = 0 as libc::c_int;
        while sv.time_residual >= fps {
            sv.frametime = fps as libc::c_float;
            SV_Physics();
            sv.time_residual -= fps;
            sv.time += fps;
            numFrames += 1
        }
        return (numFrames != 0 as libc::c_int) as libc::c_int as qboolean
    } else {
        SV_Physics();
        sv.time += sv.frametime as libc::c_double;
        return true_0
    };
}
/*
==================
Host_ServerFrame

==================
*/
#[no_mangle]
pub unsafe extern "C" fn Host_ServerFrame() {
    // if server is not active, do nothing
    if svs.initialized as u64 == 0 { return }
    if sv_fps.value != 0.0f32 &&
           (sv.simulating as libc::c_uint != 0 ||
                sv.state as libc::c_uint !=
                    ss_active as libc::c_int as libc::c_uint) {
        sv.time_residual += host.frametime
    }
    if sv_fps.value == 0.0f32 {
        sv.frametime = host.frametime as libc::c_float
    }
    (*svgame.globals).frametime = sv.frametime;
    // check clients timewindow
    SV_CheckCmdTimes();
    // read packets from clients
    SV_ReadPackets();
    // refresh physic movevars on the client side
    SV_UpdateMovevars(false_0);
    // request missing resources for clients
    SV_RequestMissingResources();
    // check timeouts
    SV_CheckTimeouts();
    // let everything in the world think and move
    if SV_RunGameFrame() as u64 == 0 { return }
    // send messages back to the clients that had packets read this frame
    SV_SendClientMessages();
    // clear edict flags for next frame
    SV_PrepWorldFrame();
    // send a heartbeat to the master if needed
    Master_Heartbeat();
}
/*
==================
Host_SetServerState
==================
*/
#[no_mangle]
pub unsafe extern "C" fn Host_SetServerState(mut state: libc::c_int) {
    Cvar_FullSet(b"host_serverstate\x00" as *const u8 as *const libc::c_char,
                 va(b"%i\x00" as *const u8 as *const libc::c_char, state),
                 (1 as libc::c_int) << 17 as libc::c_int);
    sv.state = state as sv_state_t;
}
//============================================================================
/*
=================
Master_Add
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Master_Add() {
    let mut adr: netadr_t =
        netadr_t{type_0: NA_UNUSED,
                 ip: [0; 4],
                 ipx: [0; 10],
                 port: 0,}; // allow remote
    NET_Config(true_0);
    if NET_StringToAdr(b"ms.xash.su:27010\x00" as *const u8 as
                           *const libc::c_char, &mut adr) as u64 == 0 {
        Con_Printf(b"can\'t resolve adr: %s\n\x00" as *const u8 as
                       *const libc::c_char,
                   b"ms.xash.su:27010\x00" as *const u8 as
                       *const libc::c_char);
    } else {
        NET_SendPacket(NS_SERVER, 2 as libc::c_int as size_t,
                       b"q\xff\x00" as *const u8 as *const libc::c_char as
                           *const libc::c_void, adr);
    };
}
/*
================
Master_Heartbeat

Send a message to the master every few minutes to
let it know we are alive, and log information
================
*/
#[no_mangle]
pub unsafe extern "C" fn Master_Heartbeat() {
    if (*public_server).value == 0. || svs.maxclients == 1 as libc::c_int {
        return
    } // only public servers send heartbeats
    // check for time wraparound
    if svs.last_heartbeat > host.realtime {
        svs.last_heartbeat = host.realtime
    } // not time to send yet
    if host.realtime - svs.last_heartbeat < 300.0f32 as libc::c_double {
        return
    }
    svs.last_heartbeat = host.realtime;
    Master_Add();
}
/*
=================
Master_Shutdown

Informs all masters that this server is going down
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Master_Shutdown() {
    let mut adr: netadr_t =
        netadr_t{type_0: NA_UNUSED,
                 ip: [0; 4],
                 ipx: [0; 10],
                 port: 0,}; // allow remote
    NET_Config(true_0);
    if NET_StringToAdr(b"ms.xash.su:27010\x00" as *const u8 as
                           *const libc::c_char, &mut adr) as u64 == 0 {
        Con_Printf(b"can\'t resolve addr: %s\n\x00" as *const u8 as
                       *const libc::c_char,
                   b"ms.xash.su:27010\x00" as *const u8 as
                       *const libc::c_char);
    } else {
        NET_SendPacket(NS_SERVER, 2 as libc::c_int as size_t,
                       b"b\n\x00" as *const u8 as *const libc::c_char as
                           *const libc::c_void, adr);
    };
}
/*
=================
SV_AddToMaster

A server info answer to master server.
Master will validate challenge and this server to public list
=================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_AddToMaster(mut from: netadr_t,
                                        mut msg: *mut sizebuf_t) {
    let mut challenge: uint = 0; // skip 2 bytes of header
    let mut s: [libc::c_char; 256] =
        *::std::mem::transmute::<&[u8; 256],
                                 &mut [libc::c_char; 256]>(b"0\n\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"); // protocol version
    let mut clients: libc::c_int = 0 as libc::c_int; // challenge number
    let mut bots: libc::c_int =
        0 as libc::c_int; // current player number, without bots
    let mut index: libc::c_int = 0; // max_players
    let mut len: libc::c_int =
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as
            libc::c_int; // bot count
    if !svs.clients.is_null() {
        index = 0 as libc::c_int; // gamedir
        while index < svs.maxclients {
            if (*svs.clients.offset(index as isize)).state as libc::c_uint >=
                   cs_connected as libc::c_int as libc::c_uint {
                if (*svs.clients.offset(index as isize)).flags &
                       (1 as libc::c_uint) << 7 as libc::c_int != 0 {
                    bots += 1
                } else { clients += 1 }
            } // current map
            index += 1
        }
    } // dedicated or local
    challenge =
        MSG_ReadUBitLong(msg,
                         ((::std::mem::size_of::<uint>() as libc::c_ulong) <<
                              3 as libc::c_int) as
                             libc::c_int); // is password set
    Info_SetValueForKey(s.as_mut_ptr(),
                        b"protocol\x00" as *const u8 as *const libc::c_char,
                        va(b"%d\x00" as *const u8 as *const libc::c_char,
                           49 as libc::c_int), len); // Windows
    Info_SetValueForKey(s.as_mut_ptr(),
                        b"challenge\x00" as *const u8 as *const libc::c_char,
                        va(b"%u\x00" as *const u8 as *const libc::c_char,
                           challenge), len); // server anti-cheat
    Info_SetValueForKey(s.as_mut_ptr(),
                        b"players\x00" as *const u8 as *const libc::c_char,
                        va(b"%d\x00" as *const u8 as *const libc::c_char,
                           clients),
                        len); // LAN servers doesn't send info to master
    Info_SetValueForKey(s.as_mut_ptr(),
                        b"max\x00" as *const u8 as *const libc::c_char,
                        va(b"%d\x00" as *const u8 as *const libc::c_char,
                           svs.maxclients),
                        len); // server region. 255 -- all regions
    Info_SetValueForKey(s.as_mut_ptr(),
                        b"bots\x00" as *const u8 as *const libc::c_char,
                        va(b"%d\x00" as *const u8 as *const libc::c_char,
                           bots), len); // server region. 255 -- all regions
    Info_SetValueForKey(s.as_mut_ptr(),
                        b"gamedir\x00" as *const u8 as *const libc::c_char,
                        (*SI.GameInfo).gamefolder.as_mut_ptr(),
                        len); // product? Where is the difference with gamedir?
    Info_SetValueForKey(s.as_mut_ptr(),
                        b"map\x00" as *const u8 as *const libc::c_char,
                        sv.name.as_mut_ptr(), len);
    Info_SetValueForKey(s.as_mut_ptr(),
                        b"type\x00" as *const u8 as *const libc::c_char,
                        if host.type_0 ==
                               HOST_DEDICATED as libc::c_int as libc::c_uint {
                            b"d\x00" as *const u8 as *const libc::c_char
                        } else {
                            b"l\x00" as *const u8 as *const libc::c_char
                        }, len);
    Info_SetValueForKey(s.as_mut_ptr(),
                        b"password\x00" as *const u8 as *const libc::c_char,
                        b"0\x00" as *const u8 as *const libc::c_char, len);
    Info_SetValueForKey(s.as_mut_ptr(),
                        b"os\x00" as *const u8 as *const libc::c_char,
                        b"w\x00" as *const u8 as *const libc::c_char, len);
    Info_SetValueForKey(s.as_mut_ptr(),
                        b"secure\x00" as *const u8 as *const libc::c_char,
                        b"0\x00" as *const u8 as *const libc::c_char, len);
    Info_SetValueForKey(s.as_mut_ptr(),
                        b"lan\x00" as *const u8 as *const libc::c_char,
                        b"0\x00" as *const u8 as *const libc::c_char, len);
    Info_SetValueForKey(s.as_mut_ptr(),
                        b"version\x00" as *const u8 as *const libc::c_char,
                        va(b"%s\x00" as *const u8 as *const libc::c_char,
                           b"0.20\x00" as *const u8 as *const libc::c_char),
                        len);
    Info_SetValueForKey(s.as_mut_ptr(),
                        b"region\x00" as *const u8 as *const libc::c_char,
                        b"255\x00" as *const u8 as *const libc::c_char, len);
    Info_SetValueForKey(s.as_mut_ptr(),
                        b"product\x00" as *const u8 as *const libc::c_char,
                        (*SI.GameInfo).gamefolder.as_mut_ptr(), len);
    NET_SendPacket(NS_SERVER, Q_strlen(s.as_mut_ptr()),
                   s.as_mut_ptr() as *const libc::c_void, from);
}
/*
====================
SV_ProcessUserAgent

send error message and return false on wrong input devices
====================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_ProcessUserAgent(mut from: netadr_t,
                                             mut useragent:
                                                 *const libc::c_char)
 -> qboolean {
    let mut input_devices_str: *const libc::c_char =
        Info_ValueForKey(useragent,
                         b"d\x00" as *const u8 as *const libc::c_char);
    let mut id: *const libc::c_char =
        Info_ValueForKey(useragent,
                         b"uuid\x00" as *const u8 as *const libc::c_char);
    if (*sv_allow_noinputdevices).value == 0. &&
           (input_devices_str.is_null() ||
                *input_devices_str.offset(0 as libc::c_int as isize) == 0) {
        SV_RejectConnection(from,
                            b"This server does not allow\nconnect without input devices list.\nPlease update your engine.\n\x00"
                                as *const u8 as *const libc::c_char);
        return false_0
    }
    if !input_devices_str.is_null() {
        let mut input_devices: libc::c_int = Q_atoi(input_devices_str);
        if (*sv_allow_touch).value == 0. &&
               input_devices & (1 as libc::c_int) << 1 as libc::c_int != 0 {
            SV_RejectConnection(from,
                                b"This server does not allow touch\nDisable it (touch_enable 0)\nto play on this server\n\x00"
                                    as *const u8 as *const libc::c_char);
            return false_0
        }
        if (*sv_allow_mouse).value == 0. &&
               input_devices & (1 as libc::c_int) << 0 as libc::c_int != 0 {
            SV_RejectConnection(from,
                                b"This server does not allow mouse\nDisable it(m_ignore 1)\nto play on this server\n\x00"
                                    as *const u8 as *const libc::c_char);
            return false_0
        }
        if (*sv_allow_joystick).value == 0. &&
               input_devices & (1 as libc::c_int) << 2 as libc::c_int != 0 {
            SV_RejectConnection(from,
                                b"This server does not allow joystick\nDisable it(joy_enable 0)\nto play on this server\n\x00"
                                    as *const u8 as *const libc::c_char);
            return false_0
        }
        if (*sv_allow_vr).value == 0. &&
               input_devices & (1 as libc::c_int) << 3 as libc::c_int != 0 {
            SV_RejectConnection(from,
                                b"This server does not allow VR\n\x00" as
                                    *const u8 as *const libc::c_char);
            return false_0
        }
    }
    if !id.is_null() {
        let mut banned: qboolean = SV_CheckID(id);
        if banned as u64 != 0 {
            SV_RejectConnection(from,
                                b"You are banned!\n\x00" as *const u8 as
                                    *const libc::c_char);
            return false_0
        }
    }
    return true_0;
}
//============================================================================
/*
===============
SV_Init

Only called at startup, not for each game
===============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_Init() {
    let mut versionString: string = [0; 256]; // XashXT cvar
    SV_InitHostCommands(); // Paranoia 2 cvar
    Cvar_Get(b"protocol\x00" as *const u8 as *const libc::c_char,
             va(b"%i\x00" as *const u8 as *const libc::c_char,
                49 as libc::c_int), (1 as libc::c_int) << 17 as libc::c_int,
             b"displays server protocol version\x00" as *const u8 as
                 *const libc::c_char);
    Cvar_Get(b"suitvolume\x00" as *const u8 as *const libc::c_char,
             b"0.25\x00" as *const u8 as *const libc::c_char,
             (1 as libc::c_int) << 0 as libc::c_int,
             b"HEV suit volume\x00" as *const u8 as *const libc::c_char);
    Cvar_Get(b"sv_background\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char,
             (1 as libc::c_int) << 17 as libc::c_int,
             b"indicate what background map is running\x00" as *const u8 as
                 *const libc::c_char);
    Cvar_Get(b"gamedir\x00" as *const u8 as *const libc::c_char,
             (*SI.GameInfo).gamefolder.as_mut_ptr(),
             (1 as libc::c_int) << 17 as libc::c_int,
             b"game folder\x00" as *const u8 as *const libc::c_char);
    Cvar_Get(b"sv_alltalk\x00" as *const u8 as *const libc::c_char,
             b"1\x00" as *const u8 as *const libc::c_char, 0 as libc::c_int,
             b"allow to talking for all players (legacy, unused)\x00" as
                 *const u8 as *const libc::c_char);
    Cvar_Get(b"sv_allow_PhysX\x00" as *const u8 as *const libc::c_char,
             b"1\x00" as *const u8 as *const libc::c_char,
             (1 as libc::c_int) << 0 as libc::c_int,
             b"allow XashXT to usage PhysX engine\x00" as *const u8 as
                 *const libc::c_char);
    Cvar_Get(b"sv_precache_meshes\x00" as *const u8 as *const libc::c_char,
             b"1\x00" as *const u8 as *const libc::c_char,
             (1 as libc::c_int) << 0 as libc::c_int,
             b"cache SOLID_CUSTOM meshes before level loading\x00" as
                 *const u8 as *const libc::c_char);
    Cvar_Get(b"servercfgfile\x00" as *const u8 as *const libc::c_char,
             b"server.cfg\x00" as *const u8 as *const libc::c_char,
             0 as libc::c_int,
             b"name of dedicated server configuration file\x00" as *const u8
                 as *const libc::c_char);
    Cvar_Get(b"lservercfgfile\x00" as *const u8 as *const libc::c_char,
             b"listenserver.cfg\x00" as *const u8 as *const libc::c_char,
             0 as libc::c_int,
             b"name of listen server configuration file\x00" as *const u8 as
                 *const libc::c_char);
    Cvar_RegisterVariable(&mut sv_zmax);
    Cvar_RegisterVariable(&mut sv_wateramp);
    Cvar_RegisterVariable(&mut sv_skycolor_r);
    Cvar_RegisterVariable(&mut sv_skycolor_g);
    Cvar_RegisterVariable(&mut sv_skycolor_b);
    Cvar_RegisterVariable(&mut sv_skyvec_x);
    Cvar_RegisterVariable(&mut sv_skyvec_y);
    Cvar_RegisterVariable(&mut sv_skyvec_z);
    Cvar_RegisterVariable(&mut sv_skyname);
    Cvar_RegisterVariable(&mut sv_footsteps);
    Cvar_RegisterVariable(&mut sv_wateralpha);
    Cvar_RegisterVariable(&mut sv_minupdaterate);
    Cvar_RegisterVariable(&mut sv_maxupdaterate);
    Cvar_RegisterVariable(&mut sv_minrate);
    Cvar_RegisterVariable(&mut sv_maxrate);
    Cvar_RegisterVariable(&mut sv_cheats);
    Cvar_RegisterVariable(&mut sv_airmove);
    Cvar_RegisterVariable(&mut sv_fps);
    Cvar_RegisterVariable(&mut showtriggers);
    Cvar_RegisterVariable(&mut sv_aim);
    Cvar_RegisterVariable(&mut deathmatch);
    Cvar_RegisterVariable(&mut coop);
    Cvar_RegisterVariable(&mut teamplay);
    Cvar_RegisterVariable(&mut skill);
    Cvar_RegisterVariable(&mut temp1);
    Cvar_RegisterVariable(&mut rcon_password);
    Cvar_RegisterVariable(&mut sv_stepsize);
    Cvar_RegisterVariable(&mut sv_newunit);
    Cvar_RegisterVariable(&mut hostname);
    timeout =
        Cvar_Get(b"timeout\x00" as *const u8 as *const libc::c_char,
                 b"125\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 2 as libc::c_int,
                 b"connection timeout\x00" as *const u8 as
                     *const libc::c_char);
    sv_pausable =
        Cvar_Get(b"pausable\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 2 as libc::c_int,
                 b"allow players to pause or not\x00" as *const u8 as
                     *const libc::c_char);
    sv_validate_changelevel =
        Cvar_Get(b"sv_validate_changelevel\x00" as *const u8 as
                     *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 0 as libc::c_int,
                 b"test change level for level-designer errors\x00" as
                     *const u8 as *const libc::c_char);
    Cvar_RegisterVariable(&mut sv_clienttrace);
    Cvar_RegisterVariable(&mut sv_bounce);
    Cvar_RegisterVariable(&mut sv_spectatormaxspeed);
    Cvar_RegisterVariable(&mut sv_waterfriction);
    Cvar_RegisterVariable(&mut sv_wateraccelerate);
    Cvar_RegisterVariable(&mut sv_rollangle);
    Cvar_RegisterVariable(&mut sv_rollspeed);
    Cvar_RegisterVariable(&mut sv_airaccelerate);
    Cvar_RegisterVariable(&mut sv_maxvelocity);
    Cvar_RegisterVariable(&mut sv_gravity);
    Cvar_RegisterVariable(&mut sv_maxspeed);
    Cvar_RegisterVariable(&mut sv_accelerate);
    Cvar_RegisterVariable(&mut sv_friction);
    Cvar_RegisterVariable(&mut sv_edgefriction);
    Cvar_RegisterVariable(&mut sv_stopspeed);
    sv_maxclients =
        Cvar_Get(b"maxplayers\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 30 as libc::c_int,
                 b"server max capacity\x00" as *const u8 as
                     *const libc::c_char);
    sv_check_errors =
        Cvar_Get(b"sv_check_errors\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int,
                 b"check edicts for errors\x00" as *const u8 as
                     *const libc::c_char);
    public_server =
        Cvar_Get(b"public\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 0 as libc::c_int,
                 b"change server type from private to public\x00" as *const u8
                     as *const libc::c_char);
    sv_lighting_modulate =
        Cvar_Get(b"r_lighting_modulate\x00" as *const u8 as
                     *const libc::c_char,
                 b"0.6\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int,
                 b"lightstyles modulate scale\x00" as *const u8 as
                     *const libc::c_char);
    sv_reconnect_limit =
        Cvar_Get(b"sv_reconnect_limit\x00" as *const u8 as
                     *const libc::c_char,
                 b"3\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int,
                 b"max reconnect attempts\x00" as *const u8 as
                     *const libc::c_char);
    Cvar_RegisterVariable(&mut sv_failuretime);
    Cvar_RegisterVariable(&mut sv_unlag);
    Cvar_RegisterVariable(&mut sv_maxunlag);
    Cvar_RegisterVariable(&mut sv_unlagpush);
    Cvar_RegisterVariable(&mut sv_unlagsamples);
    Cvar_RegisterVariable(&mut sv_allow_upload);
    Cvar_RegisterVariable(&mut sv_allow_download);
    Cvar_RegisterVariable(&mut sv_send_logos);
    Cvar_RegisterVariable(&mut sv_send_resources);
    Cvar_RegisterVariable(&mut sv_uploadmax);
    Cvar_RegisterVariable(&mut sv_version);
    Cvar_RegisterVariable(&mut sv_instancedbaseline);
    Cvar_RegisterVariable(&mut sv_consistency);
    Cvar_RegisterVariable(&mut sv_downloadurl);
    sv_novis =
        Cvar_Get(b"sv_novis\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 0 as libc::c_int,
                 b"force to ignore server visibility\x00" as *const u8 as
                     *const libc::c_char);
    sv_hostmap =
        Cvar_Get(b"hostmap\x00" as *const u8 as *const libc::c_char,
                 (*SI.GameInfo).startmap.as_mut_ptr(), 0 as libc::c_int,
                 b"keep name of last entered map\x00" as *const u8 as
                     *const libc::c_char);
    Cvar_RegisterVariable(&mut sv_password);
    Cvar_RegisterVariable(&mut sv_lan);
    Cvar_RegisterVariable(&mut violence_ablood);
    Cvar_RegisterVariable(&mut violence_hblood);
    Cvar_RegisterVariable(&mut violence_agibs);
    Cvar_RegisterVariable(&mut violence_hgibs);
    Cvar_RegisterVariable(&mut mp_logecho);
    Cvar_RegisterVariable(&mut mp_logfile);
    Cvar_RegisterVariable(&mut sv_log_onefile);
    Cvar_RegisterVariable(&mut sv_log_singleplayer);
    Cvar_RegisterVariable(&mut sv_background_freeze);
    Cvar_RegisterVariable(&mut mapcyclefile);
    Cvar_RegisterVariable(&mut motdfile);
    Cvar_RegisterVariable(&mut logsdir);
    Cvar_RegisterVariable(&mut bannedcfgfile);
    Cvar_RegisterVariable(&mut listipcfgfile);
    Cvar_RegisterVariable(&mut mapchangecfgfile);
    sv_allow_joystick =
        Cvar_Get(b"sv_allow_joystick\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int,
                 b"allow connect with joystick enabled\x00" as *const u8 as
                     *const libc::c_char);
    sv_allow_mouse =
        Cvar_Get(b"sv_allow_mouse\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int,
                 b"allow connect with mouse\x00" as *const u8 as
                     *const libc::c_char);
    sv_allow_touch =
        Cvar_Get(b"sv_allow_touch\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int,
                 b"allow connect with touch controls\x00" as *const u8 as
                     *const libc::c_char);
    sv_allow_vr =
        Cvar_Get(b"sv_allow_vr\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int,
                 b"allow connect from vr version\x00" as *const u8 as
                     *const libc::c_char);
    sv_allow_noinputdevices =
        Cvar_Get(b"sv_allow_noinputdevices\x00" as *const u8 as
                     *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int,
                 b"allow connect from old versions without useragent\x00" as
                     *const u8 as *const libc::c_char);
    // when we in developer-mode automatically turn cheats on
    if host_developer.value != 0. {
        Cvar_SetValue(b"sv_cheats\x00" as *const u8 as *const libc::c_char,
                      1.0f32); // delete all temporary *.hl files
    }
    MSG_InitExt(&mut net_message,
                b"NetMessage\x00" as *const u8 as *const libc::c_char,
                net_message_buffer.as_mut_ptr() as *mut libc::c_void,
                ::std::mem::size_of::<[byte; 131120]>() as libc::c_ulong as
                    libc::c_int, -(1 as libc::c_int));
    Q_snprintf(versionString.as_mut_ptr(),
               ::std::mem::size_of::<string>() as libc::c_ulong,
               b"%s: %s-%s(%s-%s),%i,%i\x00" as *const u8 as
                   *const libc::c_char,
               b"Xash3D FWGS\x00" as *const u8 as *const libc::c_char,
               b"0.20\x00" as *const u8 as *const libc::c_char,
               Q_buildcommit(), Q_buildos(), Q_buildarch(), 49 as libc::c_int,
               Q_buildnum());
    Cvar_FullSet(b"sv_version\x00" as *const u8 as *const libc::c_char,
                 versionString.as_mut_ptr(),
                 (1 as libc::c_int) << 17 as libc::c_int);
    SV_InitFilter();
    SV_ClearGameState();
    SV_InitGame();
}
/*
==================
SV_FinalMessage

Used by SV_Shutdown to send a final message to all
connected clients before the server goes down.  The messages are sent immediately,
not just stuck on the outgoing message list, because the server is going
to totally exit after returning from this function.
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_FinalMessage(mut message: *const libc::c_char,
                                         mut reconnect: qboolean) {
    let mut msg_buf: [byte; 1024] = [0; 1024];
    let mut cl: *mut sv_client_t = 0 as *mut sv_client_t;
    let mut msg: sizebuf_t =
        sizebuf_t{bOverflow: false_0,
                  pDebugName: 0 as *const libc::c_char,
                  pData: 0 as *const byte as *mut byte,
                  iCurBit: 0,
                  nDataBits: 0,};
    let mut i: libc::c_int = 0;
    MSG_InitExt(&mut msg,
                b"FinalMessage\x00" as *const u8 as *const libc::c_char,
                msg_buf.as_mut_ptr() as *mut libc::c_void,
                ::std::mem::size_of::<[byte; 1024]>() as libc::c_ulong as
                    libc::c_int, -(1 as libc::c_int));
    if if message.is_null() || *message == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } != 0 {
        MSG_WriteCmdExt(&mut msg, 8 as libc::c_int, NS_SERVER,
                        0 as *const libc::c_char);
        MSG_WriteString(&mut msg, message);
    }
    if reconnect as u64 != 0 {
        if svs.maxclients <= 1 as libc::c_int {
            MSG_WriteCmdExt(&mut msg, 4 as libc::c_int, NS_SERVER,
                            0 as *const libc::c_char);
            MSG_WriteOneBit(&mut msg, host.game.loadGame as libc::c_int);
        } else { SV_BuildReconnect(&mut msg); }
    } else {
        MSG_WriteCmdExt(&mut msg, 2 as libc::c_int, NS_SERVER,
                        0 as *const libc::c_char);
    }
    // send it twice
	// stagger the packets to crutch operating system limited buffers
    i = 0 as libc::c_int;
    cl = svs.clients;
    while i < svs.maxclients {
        if (*cl).state as libc::c_uint >=
               cs_connected as libc::c_int as libc::c_uint &&
               (*cl).flags & (1 as libc::c_uint) << 7 as libc::c_int == 0 {
            Netchan_TransmitBits(&mut (*cl).netchan,
                                 MSG_GetNumBitsWritten(&mut msg),
                                 MSG_GetData(&mut msg));
        }
        i += 1;
        cl = cl.offset(1)
    }
    i = 0 as libc::c_int;
    cl = svs.clients;
    while i < svs.maxclients {
        if (*cl).state as libc::c_uint >=
               cs_connected as libc::c_int as libc::c_uint &&
               (*cl).flags & (1 as libc::c_uint) << 7 as libc::c_int == 0 {
            Netchan_TransmitBits(&mut (*cl).netchan,
                                 MSG_GetNumBitsWritten(&mut msg),
                                 MSG_GetData(&mut msg));
        }
        i += 1;
        cl = cl.offset(1)
    };
}
/*
================
SV_FreeClients

release server clients
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_FreeClients() {
    if svs.maxclients != 0 as libc::c_int {
        // free server static data
        if !svs.clients.is_null() {
            if !svs.clients.is_null() {
                _Mem_Free(svs.clients as *mut libc::c_void,
                          b"../engine/server/sv_main.c\x00" as *const u8 as
                              *const libc::c_char, 1034 as libc::c_int);
            }
            svs.clients = 0 as *mut sv_client_t
        }
        if !svs.packet_entities.is_null() {
            if !svs.packet_entities.is_null() {
                _Mem_Free(svs.packet_entities as *mut libc::c_void,
                          b"../engine/server/sv_main.c\x00" as *const u8 as
                              *const libc::c_char, 1040 as libc::c_int);
            }
            svs.packet_entities = 0 as *mut entity_state_t;
            svs.num_client_entities = 0 as libc::c_int;
            svs.next_client_entities = 0 as libc::c_int
        }
    };
}
/*
================
SV_Shutdown

Called when each game quits,
before Sys_Quit or Sys_Error
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_Shutdown(mut finalmsg: *const libc::c_char) {
    // already freed
    if SV_Initialized() as u64 == 0 {
        // drop the client if want to load a new map
        if CL_IsPlaybackDemo() as u64 != 0 { CL_Drop(); }
        return
    }
    if if finalmsg.is_null() || *finalmsg == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } != 0 {
        Con_Printf(b"%s\x00" as *const u8 as *const libc::c_char, finalmsg);
    }
    // rcon will be disconnected
    SV_EndRedirect();
    if !svs.clients.is_null() { SV_FinalMessage(finalmsg, false_0); }
    if (*public_server).value != 0. && svs.maxclients != 1 as libc::c_int {
        Master_Shutdown();
    }
    NET_Config(false_0);
    SV_UnloadProgs();
    CL_Drop();
    // free current level
    memset(&mut sv as *mut server_t as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<server_t>() as libc::c_ulong);
    SV_FreeClients();
    svs.maxclients = 0 as libc::c_int;
    // release all models
    Mod_FreeAll();
    HPAK_FlushHostQueue();
    Log_Printf(b"Server shutdown\n\x00" as *const u8 as *const libc::c_char);
    Log_Close();
    svs.initialized = false_0;
}
