#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type file_s;
    pub type grasshdr_s;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Q_strnicmp(s1: *const libc::c_char, s2: *const libc::c_char,
                  n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Q_snprintf(buffer: *mut libc::c_char, buffersize: size_t,
                  format: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn Q_sprintf(buffer: *mut libc::c_char, format: *const libc::c_char,
                 _: ...) -> libc::c_int;
    #[no_mangle]
    fn Q_pretifymem(value: libc::c_float, digitsafterdecimal: libc::c_int)
     -> *mut libc::c_char;
    #[no_mangle]
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn COM_FileBase(in_0: *const libc::c_char, out: *mut libc::c_char);
    #[no_mangle]
    fn COM_ReplaceExtension(path: *mut libc::c_char,
                            extension: *const libc::c_char);
    #[no_mangle]
    fn _COM_ParseFileSafe(data: *mut libc::c_char, token: *mut libc::c_char,
                          size: libc::c_int, flags: libc::c_uint,
                          len: *mut libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn Sys_DoubleTime() -> libc::c_double;
    #[no_mangle]
    fn Cvar_FullSet(var_name: *const libc::c_char, value: *const libc::c_char,
                    flags: libc::c_int);
    #[no_mangle]
    fn Cvar_Set(var_name: *const libc::c_char, value: *const libc::c_char);
    #[no_mangle]
    fn Cvar_SetValue(var_name: *const libc::c_char, value: libc::c_float);
    #[no_mangle]
    fn Cvar_VariableString(var_name: *const libc::c_char)
     -> *const libc::c_char;
    #[no_mangle]
    fn CRC32_Init(pulCRC: *mut dword);
    #[no_mangle]
    fn CRC32_ProcessBuffer(pulCRC: *mut dword, pBuffer: *const libc::c_void,
                           nBuffer: libc::c_int);
    #[no_mangle]
    static mut sv_maxclients: *mut convar_t;
    #[no_mangle]
    fn NET_Config(net_enable: qboolean);
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    static mut SI: sysinfo_t;
    #[no_mangle]
    fn Cbuf_AddText(text: *const libc::c_char);
    #[no_mangle]
    fn _Mem_Realloc(poolptr: poolhandle_t, memptr: *mut libc::c_void,
                    size: size_t, clear: qboolean,
                    filename: *const libc::c_char, fileline: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    fn _Mem_Free(data: *mut libc::c_void, filename: *const libc::c_char,
                 fileline: libc::c_int);
    #[no_mangle]
    fn FS_LoadFile(path: *const libc::c_char, filesizeptr: *mut fs_offset_t,
                   gamedironly: qboolean) -> *mut byte;
    #[no_mangle]
    fn COM_FixSlashes(pname: *mut libc::c_char);
    #[no_mangle]
    fn FS_Open(filepath: *const libc::c_char, mode: *const libc::c_char,
               gamedironly: qboolean) -> *mut file_t;
    #[no_mangle]
    fn FS_Read(file: *mut file_t, buffer: *mut libc::c_void,
               buffersize: size_t) -> fs_offset_t;
    #[no_mangle]
    fn FS_Seek(file: *mut file_t, offset: fs_offset_t, whence: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn FS_FileSize(filename: *const libc::c_char, gamedironly: qboolean)
     -> fs_offset_t;
    #[no_mangle]
    fn FS_FileExists(filename: *const libc::c_char, gamedironly: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn FS_Eof(file: *mut file_t) -> qboolean;
    #[no_mangle]
    fn FS_Close(file: *mut file_t) -> libc::c_int;
    #[no_mangle]
    fn Host_EndGame(abort: qboolean, message: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Host_ShutdownServer();
    #[no_mangle]
    fn Host_Error(error: *const libc::c_char, _: ...);
    #[no_mangle]
    fn S_StopBackgroundTrack();
    #[no_mangle]
    fn S_StopAllSounds(ambient: qboolean);
    #[no_mangle]
    fn CL_StopPlayback();
    #[no_mangle]
    fn Log_Printf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn COM_IsSafeFileToDownload(filename: *const libc::c_char) -> qboolean;
    #[no_mangle]
    fn Con_Reportf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_DPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn HPAK_FlushHostQueue();
    #[no_mangle]
    fn SV_ChangeLevel(loadfromsavedgame: qboolean,
                      mapname: *const libc::c_char,
                      start: *const libc::c_char, background: qboolean);
    #[no_mangle]
    fn SV_UnloadProgs();
    #[no_mangle]
    fn SV_LoadGameState(level: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn SV_LoadProgs(name: *const libc::c_char) -> qboolean;
    #[no_mangle]
    fn Log_Open();
    #[no_mangle]
    fn Log_PrintServerVars();
    #[no_mangle]
    static mut hostname: convar_t;
    #[no_mangle]
    static mut skill: convar_t;
    #[no_mangle]
    static mut deathmatch: convar_t;
    #[no_mangle]
    static mut coop: convar_t;
    #[no_mangle]
    fn Mod_LoadWorld(name: *const libc::c_char, preload: qboolean)
     -> *mut model_t;
    #[no_mangle]
    fn Mod_ForName(name: *const libc::c_char, crash: qboolean,
                   trackCRC: qboolean) -> *mut model_t;
    #[no_mangle]
    fn SV_InitEdict(pEdict: *mut edict_t);
    #[no_mangle]
    fn SV_UpdateMovevars(initialize: qboolean);
    #[no_mangle]
    fn SV_ClearWorld();
    #[no_mangle]
    fn SV_SpawnEntities(mapname: *const libc::c_char);
    #[no_mangle]
    fn MSG_InitExt(sb: *mut sizebuf_t, pDebugName: *const libc::c_char,
                   pData: *mut libc::c_void, nBytes: libc::c_int,
                   nMaxBits: libc::c_int);
    #[no_mangle]
    fn SV_FreeEdict(pEdict: *mut edict_t);
    #[no_mangle]
    fn SV_SetStringArrayMode(dynamic: qboolean);
    #[no_mangle]
    fn SV_Physics();
    #[no_mangle]
    fn SV_SendResource(pResource: *mut resource_t, msg: *mut sizebuf_t);
    #[no_mangle]
    fn MSG_WriteCmdExt(sb: *mut sizebuf_t, cmd: libc::c_int, type_0: netsrc_t,
                       name: *const libc::c_char);
    #[no_mangle]
    fn SV_CheckEdict(e: *const edict_t, file: *const libc::c_char,
                     line: libc::c_int) -> qboolean;
    #[no_mangle]
    fn SV_EdictNum(n: libc::c_int) -> *mut edict_t;
    #[no_mangle]
    fn MSG_WriteUBitLong(sb: *mut sizebuf_t, curData: uint,
                         numbits: libc::c_int);
    #[no_mangle]
    fn SV_TransferConsistencyInfo();
    #[no_mangle]
    fn Netchan_Clear(chan: *mut netchan_t);
    #[no_mangle]
    fn Mod_FreeUnused();
    #[no_mangle]
    static mut public_server: *mut convar_t;
    #[no_mangle]
    fn Master_Add();
    #[no_mangle]
    fn SV_ClearGameState();
    #[no_mangle]
    fn SV_FinalMessage(message: *const libc::c_char, reconnect: qboolean);
    #[no_mangle]
    fn Host_SetServerState(state: libc::c_int);
    #[no_mangle]
    fn SV_FreeEdicts();
    #[no_mangle]
    fn SV_ClearPhysEnts();
    #[no_mangle]
    fn SV_EmptyStringPool();
    #[no_mangle]
    fn MSG_WriteDeltaEntity(from: *mut entity_state_s,
                            to: *mut entity_state_s, msg: *mut sizebuf_t,
                            force: qboolean, type_0: libc::c_int,
                            timebase: libc::c_double, ofs: libc::c_int);
    #[no_mangle]
    fn COM_GetLibraryError() -> *const libc::c_char;
    #[no_mangle]
    fn COM_ResetLibraryError();
    #[no_mangle]
    fn COM_GetCommonLibraryPath(eLibType: ECommonLibraryType,
                                out: *mut libc::c_char, size: size_t);
}
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
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
pub type fs_offset_t = off_t;
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
pub struct dlump_t {
    pub fileofs: libc::c_int,
    pub filelen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dheader_t {
    pub version: libc::c_int,
    pub lumps: [dlump_t; 15],
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
pub const ss_dead: sv_state_t = 0;
pub type sv_state_t = libc::c_uint;
pub const ss_active: sv_state_t = 2;
pub const ss_loading: sv_state_t = 1;
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
pub type CRC32_t = libc::c_uint;
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
pub struct server_log_t {
    pub active: qboolean,
    pub net_log: qboolean,
    pub net_address: netadr_t,
    pub file: *mut file_t,
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
pub const DELTA_ENTITY: C2RustUnnamed_0 = 0;
pub const DELTA_PLAYER: C2RustUnnamed_0 = 1;
pub type ECommonLibraryType = libc::c_uint;
pub const LIBRARY_GAMEUI: ECommonLibraryType = 2;
pub const LIBRARY_SERVER: ECommonLibraryType = 1;
pub const LIBRARY_CLIENT: ECommonLibraryType = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const DELTA_STATIC: C2RustUnnamed_0 = 2;
/*
sv_init.c - server initialize operations
Copyright (C) 2009 Uncle Mike

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
pub static mut SV_UPDATE_BACKUP: libc::c_int = 16 as libc::c_int;
#[no_mangle]
pub static mut sv: server_t =
    server_t{state: ss_dead,
             background: false_0,
             loadgame: false_0,
             time: 0.,
             time_residual: 0.,
             frametime: 0.,
             framecount: 0,
             current_client: 0 as *const sv_client_s as *mut sv_client_s,
             hostflags: 0,
             worldmapCRC: 0,
             progsCRC: 0,
             name: [0; 64],
             startspot: [0; 64],
             lastchecktime: 0.,
             lastcheck: 0,
             model_precache: [[0; 64]; 1024],
             sound_precache: [[0; 64]; 2048],
             files_precache: [[0; 64]; 1024],
             event_precache: [[0; 64]; 1024],
             model_precache_flags: [0; 1024],
             models: [0 as *const model_t as *mut model_t; 1024],
             num_static_entities: 0,
             lightstyles:
                 [lightstyle_t{pattern: [0; 256],
                               map: [0.; 256],
                               length: 0,
                               value: 0.,
                               interp: false_0,
                               time: 0.,}; 64],
             consistency_list:
                 [consistency_t{filename: 0 as *const libc::c_char,
                                orig_index: 0,
                                check_type: 0,
                                issound: false_0,
                                value: 0,
                                mins: [0.; 3],
                                maxs: [0.; 3],}; 1024],
             resources:
                 [resource_t{szFileName: [0; 64],
                             type_0: t_sound,
                             nIndex: 0,
                             nDownloadSize: 0,
                             ucFlags: 0,
                             rgucMD5_hash: [0; 16],
                             playernum: 0,
                             rguc_reserved: [0; 32],
                             pNext: 0 as *const resource_s as *mut resource_s,
                             pPrev:
                                 0 as *const resource_s as *mut resource_s,};
                     5120],
             num_consistency: 0,
             num_resources: 0,
             instanced:
                 [sv_baseline_t{classname: 0 as *const libc::c_char,
                                baseline:
                                    entity_state_t{entityType: 0,
                                                   number: 0,
                                                   msg_time: 0.,
                                                   messagenum: 0,
                                                   origin: [0.; 3],
                                                   angles: [0.; 3],
                                                   modelindex: 0,
                                                   sequence: 0,
                                                   frame: 0.,
                                                   colormap: 0,
                                                   skin: 0,
                                                   solid: 0,
                                                   effects: 0,
                                                   scale: 0.,
                                                   eflags: 0,
                                                   rendermode: 0,
                                                   renderamt: 0,
                                                   rendercolor:
                                                       color24{r: 0,
                                                               g: 0,
                                                               b: 0,},
                                                   renderfx: 0,
                                                   movetype: 0,
                                                   animtime: 0.,
                                                   framerate: 0.,
                                                   body: 0,
                                                   controller: [0; 4],
                                                   blending: [0; 4],
                                                   velocity: [0.; 3],
                                                   mins: [0.; 3],
                                                   maxs: [0.; 3],
                                                   aiment: 0,
                                                   owner: 0,
                                                   friction: 0.,
                                                   gravity: 0.,
                                                   team: 0,
                                                   playerclass: 0,
                                                   health: 0,
                                                   spectator: false_0,
                                                   weaponmodel: 0,
                                                   gaitsequence: 0,
                                                   basevelocity: [0.; 3],
                                                   usehull: 0,
                                                   oldbuttons: 0,
                                                   onground: 0,
                                                   iStepLeft: 0,
                                                   flFallVelocity: 0.,
                                                   fov: 0.,
                                                   weaponanim: 0,
                                                   startpos: [0.; 3],
                                                   endpos: [0.; 3],
                                                   impacttime: 0.,
                                                   starttime: 0.,
                                                   iuser1: 0,
                                                   iuser2: 0,
                                                   iuser3: 0,
                                                   iuser4: 0,
                                                   fuser1: 0.,
                                                   fuser2: 0.,
                                                   fuser3: 0.,
                                                   fuser4: 0.,
                                                   vuser1: [0.; 3],
                                                   vuser2: [0.; 3],
                                                   vuser3: [0.; 3],
                                                   vuser4: [0.; 3],},}; 64],
             last_valid_baseline: 0,
             num_instanced: 0,
             datagram:
                 sizebuf_t{bOverflow: false_0,
                           pDebugName: 0 as *const libc::c_char,
                           pData: 0 as *const byte as *mut byte,
                           iCurBit: 0,
                           nDataBits: 0,},
             datagram_buf: [0; 16384],
             reliable_datagram:
                 sizebuf_t{bOverflow: false_0,
                           pDebugName: 0 as *const libc::c_char,
                           pData: 0 as *const byte as *mut byte,
                           iCurBit: 0,
                           nDataBits: 0,},
             reliable_datagram_buf: [0; 16384],
             multicast:
                 sizebuf_t{bOverflow: false_0,
                           pDebugName: 0 as *const libc::c_char,
                           pData: 0 as *const byte as *mut byte,
                           iCurBit: 0,
                           nDataBits: 0,},
             multicast_buf: [0; 8192],
             signon:
                 sizebuf_t{bOverflow: false_0,
                           pDebugName: 0 as *const libc::c_char,
                           pData: 0 as *const byte as *mut byte,
                           iCurBit: 0,
                           nDataBits: 0,},
             signon_buf: [0; 131072],
             spec_datagram:
                 sizebuf_t{bOverflow: false_0,
                           pDebugName: 0 as *const libc::c_char,
                           pData: 0 as *const byte as *mut byte,
                           iCurBit: 0,
                           nDataBits: 0,},
             spectator_buf: [0; 8192],
             worldmodel: 0 as *const model_t as *mut model_t,
             playersonly: false_0,
             simulating: false_0,
             paused: false_0,
             ignored_static_ents: 0,
             ignored_world_decals: 0,
             static_ents_overflow: 0,};
// local server
#[no_mangle]
pub static mut svs: server_static_t =
    server_static_t{initialized: false_0,
                    game_library_loaded: false_0,
                    timestart: 0.,
                    maxclients: 0,
                    groupmask: 0,
                    groupop: 0,
                    log:
                        server_log_t{active: false_0,
                                     net_log: false_0,
                                     net_address:
                                         netadr_t{type_0: NA_UNUSED,
                                                  ip: [0; 4],
                                                  ipx: [0; 10],
                                                  port: 0,},
                                     file:
                                         0 as *const file_t as *mut file_t,},
                    serverinfo: [0; 512],
                    localinfo: [0; 32768],
                    spawncount: 0,
                    clients: 0 as *const sv_client_t as *mut sv_client_t,
                    num_client_entities: 0,
                    next_client_entities: 0,
                    packet_entities:
                        0 as *const entity_state_t as *mut entity_state_t,
                    baselines:
                        0 as *const entity_state_t as *mut entity_state_t,
                    static_entities:
                        0 as *const entity_state_t as *mut entity_state_t,
                    last_heartbeat: 0.,
                    challenges:
                        [challenge_t{adr:
                                         netadr_t{type_0: NA_UNUSED,
                                                  ip: [0; 4],
                                                  ipx: [0; 10],
                                                  port: 0,},
                                     time: 0.,
                                     challenge: 0,
                                     connected: false_0,}; 1024],};
// persistant server info
#[no_mangle]
pub static mut svgame: svgame_static_t =
    svgame_static_t{msg_name: 0 as *const libc::c_char,
                    msg:
                        [sv_user_message_t{name: [0; 32],
                                           number: 0,
                                           size: 0,}; 197],
                    msg_size_index: 0,
                    msg_realsize: 0,
                    msg_index: 0,
                    msg_dest: 0,
                    msg_started: false_0,
                    msg_ent: 0 as *const edict_t as *mut edict_t,
                    msg_org: [0.; 3],
                    hInstance: 0 as *const libc::c_void as *mut libc::c_void,
                    config_executed: false_0,
                    edicts: 0 as *const edict_t as *mut edict_t,
                    numEntities: 0,
                    movevars:
                        movevars_t{gravity: 0.,
                                   stopspeed: 0.,
                                   maxspeed: 0.,
                                   spectatormaxspeed: 0.,
                                   accelerate: 0.,
                                   airaccelerate: 0.,
                                   wateraccelerate: 0.,
                                   friction: 0.,
                                   edgefriction: 0.,
                                   waterfriction: 0.,
                                   entgravity: 0.,
                                   bounce: 0.,
                                   stepsize: 0.,
                                   maxvelocity: 0.,
                                   zmax: 0.,
                                   waveHeight: 0.,
                                   footsteps: false_0,
                                   skyName: [0; 32],
                                   rollangle: 0.,
                                   rollspeed: 0.,
                                   skycolor_r: 0.,
                                   skycolor_g: 0.,
                                   skycolor_b: 0.,
                                   skyvec_x: 0.,
                                   skyvec_y: 0.,
                                   skyvec_z: 0.,
                                   features: 0,
                                   fog_settings: 0,
                                   wateralpha: 0.,
                                   skydir_x: 0.,
                                   skydir_y: 0.,
                                   skydir_z: 0.,
                                   skyangle: 0.,},
                    oldmovevars:
                        movevars_t{gravity: 0.,
                                   stopspeed: 0.,
                                   maxspeed: 0.,
                                   spectatormaxspeed: 0.,
                                   accelerate: 0.,
                                   airaccelerate: 0.,
                                   wateraccelerate: 0.,
                                   friction: 0.,
                                   edgefriction: 0.,
                                   waterfriction: 0.,
                                   entgravity: 0.,
                                   bounce: 0.,
                                   stepsize: 0.,
                                   maxvelocity: 0.,
                                   zmax: 0.,
                                   waveHeight: 0.,
                                   footsteps: false_0,
                                   skyName: [0; 32],
                                   rollangle: 0.,
                                   rollspeed: 0.,
                                   skycolor_r: 0.,
                                   skycolor_g: 0.,
                                   skycolor_b: 0.,
                                   skyvec_x: 0.,
                                   skyvec_y: 0.,
                                   skyvec_z: 0.,
                                   features: 0,
                                   fog_settings: 0,
                                   wateralpha: 0.,
                                   skydir_x: 0.,
                                   skydir_y: 0.,
                                   skydir_z: 0.,
                                   skyangle: 0.,},
                    pmove: 0 as *const playermove_t as *mut playermove_t,
                    interp:
                        [sv_interp_t{active: false_0,
                                     moving: false_0,
                                     firstframe: false_0,
                                     nointerp: false_0,
                                     mins: [0.; 3],
                                     maxs: [0.; 3],
                                     curpos: [0.; 3],
                                     oldpos: [0.; 3],
                                     newpos: [0.; 3],
                                     finalpos: [0.; 3],}; 32],
                    pushed:
                        [sv_pushed_t{ent: 0 as *const edict_t as *mut edict_t,
                                     origin: [0.; 3],
                                     angles: [0.; 3],
                                     fixangle: 0,}; 256],
                    globals: 0 as *const globalvars_t as *mut globalvars_t,
                    dllFuncs:
                        DLL_FUNCTIONS{pfnGameInit: None,
                                      pfnSpawn: None,
                                      pfnThink: None,
                                      pfnUse: None,
                                      pfnTouch: None,
                                      pfnBlocked: None,
                                      pfnKeyValue: None,
                                      pfnSave: None,
                                      pfnRestore: None,
                                      pfnSetAbsBox: None,
                                      pfnSaveWriteFields: None,
                                      pfnSaveReadFields: None,
                                      pfnSaveGlobalState: None,
                                      pfnRestoreGlobalState: None,
                                      pfnResetGlobalState: None,
                                      pfnClientConnect: None,
                                      pfnClientDisconnect: None,
                                      pfnClientKill: None,
                                      pfnClientPutInServer: None,
                                      pfnClientCommand: None,
                                      pfnClientUserInfoChanged: None,
                                      pfnServerActivate: None,
                                      pfnServerDeactivate: None,
                                      pfnPlayerPreThink: None,
                                      pfnPlayerPostThink: None,
                                      pfnStartFrame: None,
                                      pfnParmsNewLevel: None,
                                      pfnParmsChangeLevel: None,
                                      pfnGetGameDescription: None,
                                      pfnPlayerCustomization: None,
                                      pfnSpectatorConnect: None,
                                      pfnSpectatorDisconnect: None,
                                      pfnSpectatorThink: None,
                                      pfnSys_Error: None,
                                      pfnPM_Move: None,
                                      pfnPM_Init: None,
                                      pfnPM_FindTextureType: None,
                                      pfnSetupVisibility: None,
                                      pfnUpdateClientData: None,
                                      pfnAddToFullPack: None,
                                      pfnCreateBaseline: None,
                                      pfnRegisterEncoders: None,
                                      pfnGetWeaponData: None,
                                      pfnCmdStart: None,
                                      pfnCmdEnd: None,
                                      pfnConnectionlessPacket: None,
                                      pfnGetHullBounds: None,
                                      pfnCreateInstancedBaselines: None,
                                      pfnInconsistentFile: None,
                                      pfnAllowLagCompensation: None,},
                    dllFuncs2:
                        NEW_DLL_FUNCTIONS{pfnOnFreeEntPrivateData: None,
                                          pfnGameShutdown: None,
                                          pfnShouldCollide: None,
                                          pfnCvarValue: None,
                                          pfnCvarValue2: None,},
                    physFuncs:
                        physics_interface_t{version: 0,
                                            SV_CreateEntity: None,
                                            SV_PhysicsEntity: None,
                                            SV_LoadEntities: None,
                                            SV_UpdatePlayerBaseVelocity: None,
                                            SV_AllowSaveGame: None,
                                            SV_TriggerTouch: None,
                                            SV_CheckFeatures: None,
                                            DrawDebugTriangles: None,
                                            DrawNormalTriangles: None,
                                            DrawOrthoTriangles: None,
                                            ClipMoveToEntity: None,
                                            ClipPMoveToEntity: None,
                                            SV_EndFrame: None,
                                            pfnPrepWorldFrame: None,
                                            pfnCreateEntitiesInRestoreList:
                                                None,
                                            pfnAllocString: None,
                                            pfnMakeString: None,
                                            pfnGetString: None,
                                            pfnRestoreDecal: None,
                                            PM_PlayerTouch: None,
                                            Mod_ProcessUserData: None,
                                            SV_HullForBsp: None,
                                            SV_PlayerThink: None,},
                    mempool: 0,
                    stringspool: 0,};
// persistant game info
/*
================
SV_AddResource

generic method to put the resources into array
================
*/
unsafe extern "C" fn SV_AddResource(mut type_0: resourcetype_t,
                                    mut name: *const libc::c_char,
                                    mut size: libc::c_int, mut flags: byte,
                                    mut index: libc::c_int) {
    let mut pResource: *mut resource_t =
        &mut *sv.resources.as_mut_ptr().offset(sv.num_resources as isize) as
            *mut resource_t;
    if sv.num_resources >=
           1024 as libc::c_int + ((1 as libc::c_int) << 11 as libc::c_int) +
               ((1 as libc::c_int) << 10 as libc::c_int) +
               ((1 as libc::c_int) << 10 as libc::c_int) {
        Host_Error(b"MAX_RESOURCES limit exceeded (%d)\n\x00" as *const u8 as
                       *const libc::c_char,
                   1024 as libc::c_int +
                       ((1 as libc::c_int) << 11 as libc::c_int) +
                       ((1 as libc::c_int) << 10 as libc::c_int) +
                       ((1 as libc::c_int) << 10 as libc::c_int));
    }
    sv.num_resources += 1;
    Q_strncpy((*pResource).szFileName.as_mut_ptr(), name,
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    (*pResource).nDownloadSize = size;
    (*pResource).ucFlags = flags;
    (*pResource).nIndex = index;
    (*pResource).type_0 = type_0;
}
/*
================
SV_SendSingleResource

hot precache on a flying
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_SendSingleResource(mut name: *const libc::c_char,
                                               mut type_0: resourcetype_t,
                                               mut index: libc::c_int,
                                               mut flags: byte) {
    let mut pResource: *mut resource_t =
        &mut *sv.resources.as_mut_ptr().offset(sv.num_resources as isize) as
            *mut resource_t;
    let mut nSize: libc::c_int = 0 as libc::c_int;
    if if name.is_null() || *name == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return
    }
    match type_0 as libc::c_uint {
        2 => {
            nSize =
                if *name.offset(0 as libc::c_int as isize) as libc::c_int !=
                       '*' as i32 {
                    FS_FileSize(name, false_0)
                } else { 0 as libc::c_int as libc::c_long } as libc::c_int
        }
        0 => {
            nSize =
                FS_FileSize(va(b"sound/%s\x00" as *const u8 as
                                   *const libc::c_char, name), false_0) as
                    libc::c_int
        }
        _ => { nSize = FS_FileSize(name, false_0) as libc::c_int }
    }
    SV_AddResource(type_0, name, nSize, flags, index);
    MSG_WriteCmdExt(&mut sv.reliable_datagram, 16 as libc::c_int, NS_SERVER,
                    0 as *const libc::c_char);
    SV_SendResource(pResource, &mut sv.reliable_datagram);
}
/*
================
SV_ModelIndex

register unique model for a server and client
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_ModelIndex(mut filename: *const libc::c_char)
 -> libc::c_int {
    let mut name: [libc::c_char; 64] = [0; 64];
    let mut i: libc::c_int = 0;
    if if filename.is_null() || *filename == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return 0 as libc::c_int
    }
    if *filename as libc::c_int == '\\' as i32 ||
           *filename as libc::c_int == '/' as i32 {
        filename = filename.offset(1)
    }
    Q_strncpy(name.as_mut_ptr(), filename,
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    COM_FixSlashes(name.as_mut_ptr());
    i = 1 as libc::c_int;
    while i < 1024 as libc::c_int &&
              sv.model_precache[i as usize][0 as libc::c_int as usize] as
                  libc::c_int != 0 {
        if Q_strnicmp(sv.model_precache[i as usize].as_mut_ptr(),
                      name.as_mut_ptr(), 99999 as libc::c_int) == 0 {
            return i
        }
        i += 1
    }
    if i == 1024 as libc::c_int {
        Host_Error(b"MAX_MODELS limit exceeded (%d)\n\x00" as *const u8 as
                       *const libc::c_char, 1024 as libc::c_int);
        return 0 as libc::c_int
    }
    // register new model
    Q_strncpy(sv.model_precache[i as usize].as_mut_ptr(), name.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    if sv.state as libc::c_uint != ss_loading as libc::c_int as libc::c_uint {
        // send the update to everyone
        SV_SendSingleResource(name.as_mut_ptr(), t_model, i,
                              sv.model_precache_flags[i as usize]);
        Con_Printf(b"^3Warning:^7 late precache of %s\n\x00" as *const u8 as
                       *const libc::c_char, name.as_mut_ptr());
    }
    return i;
}
/*
================
SV_SoundIndex

register unique sound for client
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_SoundIndex(mut filename: *const libc::c_char)
 -> libc::c_int {
    let mut name: [libc::c_char; 64] = [0; 64];
    let mut i: libc::c_int = 0;
    if if filename.is_null() || *filename == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return 0 as libc::c_int
    }
    if *filename.offset(0 as libc::c_int as isize) as libc::c_int ==
           '!' as i32 {
        Con_Printf(b"^3Warning:^7 \'%s\' do not precache sentence names!\n\x00"
                       as *const u8 as *const libc::c_char, filename);
        return 0 as libc::c_int
    }
    if *filename as libc::c_int == '\\' as i32 ||
           *filename as libc::c_int == '/' as i32 {
        filename = filename.offset(1)
    }
    Q_strncpy(name.as_mut_ptr(), filename,
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    COM_FixSlashes(name.as_mut_ptr());
    i = 1 as libc::c_int;
    while i < (1 as libc::c_int) << 11 as libc::c_int &&
              sv.sound_precache[i as usize][0 as libc::c_int as usize] as
                  libc::c_int != 0 {
        if Q_strnicmp(sv.sound_precache[i as usize].as_mut_ptr(),
                      name.as_mut_ptr(), 99999 as libc::c_int) == 0 {
            return i
        }
        i += 1
    }
    if i == (1 as libc::c_int) << 11 as libc::c_int {
        Host_Error(b"MAX_SOUNDS limit exceeded (%d)\n\x00" as *const u8 as
                       *const libc::c_char,
                   (1 as libc::c_int) << 11 as libc::c_int);
        return 0 as libc::c_int
    }
    // register new sound
    Q_strncpy(sv.sound_precache[i as usize].as_mut_ptr(), name.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    if sv.state as libc::c_uint != ss_loading as libc::c_int as libc::c_uint {
        // send the update to everyone
        SV_SendSingleResource(name.as_mut_ptr(), t_sound, i,
                              0 as libc::c_int as byte);
        Con_Printf(b"^3Warning:^7 late precache of %s\n\x00" as *const u8 as
                       *const libc::c_char, name.as_mut_ptr());
    }
    return i;
}
/*
================
SV_EventIndex

register network event for a server and client
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_EventIndex(mut filename: *const libc::c_char)
 -> libc::c_int {
    let mut name: [libc::c_char; 64] = [0; 64];
    let mut i: libc::c_int = 0;
    if if filename.is_null() || *filename == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return 0 as libc::c_int
    }
    Q_strncpy(name.as_mut_ptr(), filename,
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    COM_FixSlashes(name.as_mut_ptr());
    i = 1 as libc::c_int;
    while i < (1 as libc::c_int) << 10 as libc::c_int &&
              sv.event_precache[i as usize][0 as libc::c_int as usize] as
                  libc::c_int != 0 {
        if Q_strnicmp(sv.event_precache[i as usize].as_mut_ptr(),
                      name.as_mut_ptr(), 99999 as libc::c_int) == 0 {
            return i
        }
        i += 1
    }
    if i == (1 as libc::c_int) << 10 as libc::c_int {
        Host_Error(b"MAX_EVENTS limit exceeded (%d)\n\x00" as *const u8 as
                       *const libc::c_char,
                   (1 as libc::c_int) << 10 as libc::c_int);
        return 0 as libc::c_int
    }
    // register new event
    Q_strncpy(sv.event_precache[i as usize].as_mut_ptr(), name.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    if sv.state as libc::c_uint != ss_loading as libc::c_int as libc::c_uint {
        // send the update to everyone
        SV_SendSingleResource(name.as_mut_ptr(), t_eventscript, i,
                              ((1 as libc::c_int) << 0 as libc::c_int) as
                                  byte);
    }
    return i;
}
/*
================
SV_GenericIndex

register generic resourse for a server and client
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_GenericIndex(mut filename: *const libc::c_char)
 -> libc::c_int {
    let mut name: [libc::c_char; 64] = [0; 64];
    let mut i: libc::c_int = 0;
    if if filename.is_null() || *filename == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return 0 as libc::c_int
    }
    Q_strncpy(name.as_mut_ptr(), filename,
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    COM_FixSlashes(name.as_mut_ptr());
    i = 1 as libc::c_int;
    while i < (1 as libc::c_int) << 10 as libc::c_int &&
              sv.files_precache[i as usize][0 as libc::c_int as usize] as
                  libc::c_int != 0 {
        if Q_strnicmp(sv.files_precache[i as usize].as_mut_ptr(),
                      name.as_mut_ptr(), 99999 as libc::c_int) == 0 {
            return i
        }
        i += 1
    }
    if i == (1 as libc::c_int) << 10 as libc::c_int {
        Host_Error(b"MAX_CUSTOM limit exceeded (%d)\n\x00" as *const u8 as
                       *const libc::c_char,
                   (1 as libc::c_int) << 10 as libc::c_int);
        return 0 as libc::c_int
    }
    // register new generic resource
    Q_strncpy(sv.files_precache[i as usize].as_mut_ptr(), name.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    if sv.state as libc::c_uint != ss_loading as libc::c_int as libc::c_uint {
        // send the update to everyone
        SV_SendSingleResource(name.as_mut_ptr(), t_generic, i,
                              ((1 as libc::c_int) << 0 as libc::c_int) as
                                  byte);
    }
    return i;
}
/*
================
SV_ModelHandle

get model by handle
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_ModelHandle(mut modelindex: libc::c_int)
 -> *mut model_t {
    if modelindex < 0 as libc::c_int || modelindex >= 1024 as libc::c_int {
        return 0 as *mut model_t
    }
    return sv.models[modelindex as usize];
}
#[no_mangle]
pub unsafe extern "C" fn SV_ReadResourceList(mut filename:
                                                 *const libc::c_char) {
    let mut token: string = [0; 256];
    let mut afile: *mut byte = 0 as *mut byte;
    let mut pfile: *mut libc::c_char = 0 as *mut libc::c_char;
    afile = FS_LoadFile(filename, 0 as *mut fs_offset_t, false_0);
    if afile.is_null() { return }
    pfile = afile as *mut libc::c_char;
    Con_DPrintf(b"Precaching from %s\n\x00" as *const u8 as
                    *const libc::c_char, filename);
    Con_DPrintf(b"----------------------------------\n\x00" as *const u8 as
                    *const libc::c_char);
    loop  {
        pfile =
            _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                               ::std::mem::size_of::<string>() as
                                   libc::c_ulong as libc::c_int,
                               0 as libc::c_int as libc::c_uint,
                               0 as *mut libc::c_int);
        if pfile.is_null() { break ; }
        if COM_IsSafeFileToDownload(token.as_mut_ptr()) as u64 == 0 {
            continue ;
        }
        Con_DPrintf(b"  %s\n\x00" as *const u8 as *const libc::c_char,
                    token.as_mut_ptr());
        SV_GenericIndex(token.as_mut_ptr());
    }
    Con_DPrintf(b"----------------------------------\n\x00" as *const u8 as
                    *const libc::c_char);
    _Mem_Free(afile as *mut libc::c_void,
              b"../engine/server/sv_init.c\x00" as *const u8 as
                  *const libc::c_char, 300 as libc::c_int);
}
/*
================
SV_CreateGenericResources

loads external resource list
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_CreateGenericResources() {
    let mut filename: string = [0; 256];
    Q_strncpy(filename.as_mut_ptr(),
              sv.model_precache[1 as libc::c_int as usize].as_mut_ptr(),
              ::std::mem::size_of::<string>() as libc::c_ulong);
    COM_ReplaceExtension(filename.as_mut_ptr(),
                         b".res\x00" as *const u8 as *const libc::c_char);
    COM_FixSlashes(filename.as_mut_ptr());
    SV_ReadResourceList(filename.as_mut_ptr());
    SV_ReadResourceList(b"reslist.txt\x00" as *const u8 as
                            *const libc::c_char);
}
/*
================
SV_CreateResourceList

add resources to common list
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_CreateResourceList() {
    let mut ffirstsent: qboolean = false_0; // end of list
    let mut i: libc::c_int = 0; // end of list
    let mut nSize: libc::c_int = 0; // end of list
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    sv.num_resources = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i < (1 as libc::c_int) << 10 as libc::c_int {
        s = sv.files_precache[i as usize].as_mut_ptr();
        if if s.is_null() || *s == 0 {
               0 as libc::c_int
           } else { 1 as libc::c_int } == 0 {
            break ;
        }
        nSize = FS_FileSize(s, false_0) as libc::c_int;
        SV_AddResource(t_generic, s, nSize,
                       ((1 as libc::c_int) << 0 as libc::c_int) as byte, i);
        i += 1
    }
    i = 1 as libc::c_int;
    while i < (1 as libc::c_int) << 11 as libc::c_int {
        s = sv.sound_precache[i as usize].as_mut_ptr();
        if if s.is_null() || *s == 0 {
               0 as libc::c_int
           } else { 1 as libc::c_int } == 0 {
            break ;
        }
        if *s.offset(0 as libc::c_int as isize) as libc::c_int == '!' as i32 {
            if ffirstsent as u64 == 0 {
                SV_AddResource(t_sound,
                               b"!\x00" as *const u8 as *const libc::c_char,
                               0 as libc::c_int,
                               ((1 as libc::c_int) << 0 as libc::c_int) as
                                   byte, i);
                ffirstsent = true_0
            }
        } else {
            nSize =
                FS_FileSize(va(b"sound/%s\x00" as *const u8 as
                                   *const libc::c_char, s), false_0) as
                    libc::c_int;
            SV_AddResource(t_sound, s, nSize, 0 as libc::c_int as byte, i);
        }
        i += 1
    }
    i = 1 as libc::c_int;
    while i < 1024 as libc::c_int {
        s = sv.model_precache[i as usize].as_mut_ptr();
        if if s.is_null() || *s == 0 {
               0 as libc::c_int
           } else { 1 as libc::c_int } == 0 {
            break ;
        }
        nSize =
            if *s.offset(0 as libc::c_int as isize) as libc::c_int !=
                   '*' as i32 {
                FS_FileSize(s, false_0)
            } else { 0 as libc::c_int as libc::c_long } as libc::c_int;
        SV_AddResource(t_model, s, nSize, sv.model_precache_flags[i as usize],
                       i);
        i += 1
    }
    // just send names
    i = 0 as libc::c_int; // end of list
    while i < 512 as libc::c_int &&
              host.draw_decals[i as usize][0 as libc::c_int as usize] as
                  libc::c_int != 0 {
        SV_AddResource(t_decal, host.draw_decals[i as usize].as_mut_ptr(),
                       0 as libc::c_int, 0 as libc::c_int as byte, i);
        i += 1
    }
    i = 1 as libc::c_int;
    while i < (1 as libc::c_int) << 10 as libc::c_int {
        s = sv.event_precache[i as usize].as_mut_ptr();
        if if s.is_null() || *s == 0 {
               0 as libc::c_int
           } else { 1 as libc::c_int } == 0 {
            break ;
        }
        nSize = FS_FileSize(s, false_0) as libc::c_int;
        SV_AddResource(t_eventscript, s, nSize,
                       ((1 as libc::c_int) << 0 as libc::c_int) as byte, i);
        i += 1
    };
}
/*
================
SV_CreateBaseline

Entity baselines are used to compress the update messages
to the clients -- only the fields that differ from the
baseline will be transmitted

INTERNAL RESOURCE
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_CreateBaseline() {
    let mut nullstate: entity_state_t =
        entity_state_t{entityType: 0,
                       number: 0,
                       msg_time: 0.,
                       messagenum: 0,
                       origin: [0.; 3],
                       angles: [0.; 3],
                       modelindex: 0,
                       sequence: 0,
                       frame: 0.,
                       colormap: 0,
                       skin: 0,
                       solid: 0,
                       effects: 0,
                       scale: 0.,
                       eflags: 0,
                       rendermode: 0,
                       renderamt: 0,
                       rendercolor: color24{r: 0, g: 0, b: 0,},
                       renderfx: 0,
                       movetype: 0,
                       animtime: 0.,
                       framerate: 0.,
                       body: 0,
                       controller: [0; 4],
                       blending: [0; 4],
                       velocity: [0.; 3],
                       mins: [0.; 3],
                       maxs: [0.; 3],
                       aiment: 0,
                       owner: 0,
                       friction: 0.,
                       gravity: 0.,
                       team: 0,
                       playerclass: 0,
                       health: 0,
                       spectator: false_0,
                       weaponmodel: 0,
                       gaitsequence: 0,
                       basevelocity: [0.; 3],
                       usehull: 0,
                       oldbuttons: 0,
                       onground: 0,
                       iStepLeft: 0,
                       flFallVelocity: 0.,
                       fov: 0.,
                       weaponanim: 0,
                       startpos: [0.; 3],
                       endpos: [0.; 3],
                       impacttime: 0.,
                       starttime: 0.,
                       iuser1: 0,
                       iuser2: 0,
                       iuser3: 0,
                       iuser4: 0,
                       fuser1: 0.,
                       fuser2: 0.,
                       fuser3: 0.,
                       fuser4: 0.,
                       vuser1: [0.; 3],
                       vuser2: [0.; 3],
                       vuser3: [0.; 3],
                       vuser4: [0.; 3],}; // invisible
    let mut base: *mut entity_state_t = 0 as *mut entity_state_t;
    let mut playermodel: libc::c_int = 0;
    let mut delta_type: libc::c_int = 0;
    let mut entnum: libc::c_int = 0;
    if host.features &
           ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint != 0 {
        playermodel =
            SV_ModelIndex(b"progs/player.mdl\x00" as *const u8 as
                              *const libc::c_char)
    } else {
        playermodel =
            SV_ModelIndex(b"models/player.mdl\x00" as *const u8 as
                              *const libc::c_char)
    }
    memset(&mut nullstate as *mut entity_state_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<entity_state_t>() as libc::c_ulong);
    let mut current_block_14: u64;
    entnum = 0 as libc::c_int;
    while entnum < svgame.numEntities {
        let mut pEdict: *mut edict_t = SV_EdictNum(entnum);
        if !(SV_CheckEdict(pEdict,
                           b"../engine/server/sv_init.c\x00" as *const u8 as
                               *const libc::c_char, 417 as libc::c_int) as u64
                 == 0) {
            if entnum != 0 as libc::c_int && entnum <= svs.maxclients {
                delta_type = DELTA_PLAYER as libc::c_int;
                current_block_14 = 8236137900636309791;
            } else if (*pEdict).v.modelindex == 0 {
                current_block_14 = 2473556513754201174;
            } else {
                delta_type = DELTA_ENTITY as libc::c_int;
                current_block_14 = 8236137900636309791;
            }
            match current_block_14 {
                2473556513754201174 => { }
                _ => {
                    // take current state as baseline
                    base =
                        &mut *svs.baselines.offset(entnum as isize) as
                            *mut entity_state_t;
                    (*base).number = entnum;
                    // set entity type
                    if (*pEdict).v.flags as libc::c_uint &
                           (1 as libc::c_uint) << 29 as libc::c_int != 0 {
                        (*base).entityType =
                            (1 as libc::c_int) << 1 as libc::c_int
                    } else {
                        (*base).entityType =
                            (1 as libc::c_int) << 0 as libc::c_int
                    }
                    svgame.dllFuncs.pfnCreateBaseline.expect("non-null function pointer")(delta_type,
                                                                                          entnum,
                                                                                          base,
                                                                                          pEdict,
                                                                                          playermodel,
                                                                                          host.player_mins[0
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               usize].as_mut_ptr(),
                                                                                          host.player_maxs[0
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               usize].as_mut_ptr());
                    sv.last_valid_baseline = entnum
                }
            }
        }
        entnum += 1
    }
    // create the instanced baselines
    svgame.dllFuncs.pfnCreateInstancedBaselines.expect("non-null function pointer")();
    // now put the baseline into the signon message.
    MSG_WriteCmdExt(&mut sv.signon, 22 as libc::c_int, NS_SERVER,
                    0 as *const libc::c_char); // invisible
    let mut current_block_23: u64;
    entnum = 0 as libc::c_int;
    while entnum < svgame.numEntities {
        let mut pEdict_0: *mut edict_t = SV_EdictNum(entnum);
        if !(SV_CheckEdict(pEdict_0,
                           b"../engine/server/sv_init.c\x00" as *const u8 as
                               *const libc::c_char, 455 as libc::c_int) as u64
                 == 0) {
            if entnum != 0 as libc::c_int && entnum <= svs.maxclients {
                delta_type = DELTA_PLAYER as libc::c_int;
                current_block_23 = 3275366147856559585;
            } else if (*pEdict_0).v.modelindex == 0 {
                current_block_23 = 10043043949733653460;
            } else {
                delta_type = DELTA_ENTITY as libc::c_int;
                current_block_23 = 3275366147856559585;
            }
            match current_block_23 {
                10043043949733653460 => { }
                _ => {
                    // take current state as baseline
                    base =
                        &mut *svs.baselines.offset(entnum as isize) as
                            *mut entity_state_t; // end of baselines
                    MSG_WriteDeltaEntity(&mut nullstate, base, &mut sv.signon,
                                         true_0, delta_type,
                                         1.0f32 as libc::c_double,
                                         0 as libc::c_int);
                }
            }
        }
        entnum += 1
    }
    MSG_WriteUBitLong(&mut sv.signon,
                      (((1 as libc::c_int) << 13 as libc::c_int) -
                           1 as libc::c_int) as uint, 13 as libc::c_int);
    MSG_WriteUBitLong(&mut sv.signon, sv.num_instanced as uint,
                      6 as libc::c_int);
    entnum = 0 as libc::c_int;
    while entnum < sv.num_instanced {
        base =
            &mut (*sv.instanced.as_mut_ptr().offset(entnum as
                                                        isize)).baseline;
        MSG_WriteDeltaEntity(&mut nullstate, base, &mut sv.signon, true_0,
                             DELTA_ENTITY as libc::c_int,
                             1.0f32 as libc::c_double, 0 as libc::c_int);
        entnum += 1
    };
}
/*
================
SV_FreeOldEntities

remove immediate entities
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_FreeOldEntities() {
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    let mut i: libc::c_int = 0;
    // at end of frame kill all entities which supposed to it
    i = svs.maxclients + 1 as libc::c_int;
    while i < svgame.numEntities {
        ent = SV_EdictNum(i);
        if (*ent).free as u64 == 0 &&
               (*ent).v.flags as libc::c_uint &
                   (1 as libc::c_uint) << 30 as libc::c_int != 0 {
            SV_FreeEdict(ent);
        }
        i += 1
    }
    // decrement svgame.numEntities if the highest number entities died
    while (*SV_EdictNum(svgame.numEntities - 1 as libc::c_int)).free as u64 !=
              0 {
        svgame.numEntities -= 1
    };
}
/*
================
SV_ActivateServer

activate server on changed map, run physics
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_ActivateServer(mut runPhysics: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut numFrames: libc::c_int = 0;
    let mut msg_buf: [byte; 131072] = [0; 131072];
    let mut msg: sizebuf_t =
        sizebuf_t{bOverflow: false_0,
                  pDebugName: 0 as *const libc::c_char,
                  pData: 0 as *const byte as *mut byte,
                  iCurBit: 0,
                  nDataBits: 0,};
    let mut cl: *mut sv_client_t = 0 as *mut sv_client_t;
    if svs.initialized as u64 == 0 { return }
    MSG_InitExt(&mut msg,
                b"ActivateServer\x00" as *const u8 as *const libc::c_char,
                msg_buf.as_mut_ptr() as *mut libc::c_void,
                ::std::mem::size_of::<[byte; 131072]>() as libc::c_ulong as
                    libc::c_int, -(1 as libc::c_int));
    // always clearing newunit variable
    Cvar_SetValue(b"sv_newunit\x00" as *const u8 as *const libc::c_char,
                  0 as libc::c_int as libc::c_float);
    // relese all intermediate entities
    SV_FreeOldEntities();
    // Activate the DLL server code
    (*svgame.globals).time = sv.time as libc::c_float;
    svgame.dllFuncs.pfnServerActivate.expect("non-null function pointer")(svgame.edicts,
                                                                          svgame.numEntities,
                                                                          svs.maxclients);
    SV_SetStringArrayMode(true_0);
    // parse user-specified resources
    SV_CreateGenericResources();
    if runPhysics != 0 {
        numFrames =
            if svs.maxclients <= 1 as libc::c_int {
                2 as libc::c_int
            } else { 8 as libc::c_int };
        sv.frametime = 0.1f64 as libc::c_float
    } else {
        sv.frametime = 0.001f64 as libc::c_float;
        numFrames = 1 as libc::c_int
    }
    // run some frames to allow everything to settle
    i = 0 as libc::c_int;
    while i < numFrames { SV_Physics(); i += 1 }
    // create a baseline for more efficient communications
    SV_CreateBaseline();
    // collect all info from precached resources
    SV_CreateResourceList();
    // check and count all files that marked by user as unmodified (typically is a player models etc)
    SV_TransferConsistencyInfo();
    // send serverinfo to all connected clients
    i = 0 as libc::c_int;
    cl = svs.clients;
    while i < svs.maxclients {
        if !(((*cl).state as libc::c_uint) <
                 cs_connected as libc::c_int as libc::c_uint) {
            Netchan_Clear(&mut (*cl).netchan);
            (*cl).delta_sequence = -(1 as libc::c_int)
        }
        i += 1;
        cl = cl.offset(1)
    }
    // invoke to refresh all movevars
    memset(&mut svgame.oldmovevars as *mut movevars_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<movevars_t>() as libc::c_ulong);
    (*svgame.globals).changelevel = false_0 as libc::c_int;
    // setup hostflags
    sv.hostflags = 0 as libc::c_int;
    HPAK_FlushHostQueue();
    // tell what kind of server has been started.
    if svs.maxclients > 1 as libc::c_int {
        Con_Printf(b"%i player server started\n\x00" as *const u8 as
                       *const libc::c_char, svs.maxclients);
    } else {
        Con_Printf(b"Game started\n\x00" as *const u8 as *const libc::c_char);
    }
    Log_Printf(b"Started map \"%s\" (CRC \"%u\")\n\x00" as *const u8 as
                   *const libc::c_char, sv.name.as_mut_ptr(), sv.worldmapCRC);
    // dedicated server purge unused resources here
    if host.type_0 == HOST_DEDICATED as libc::c_int as libc::c_uint {
        Mod_FreeUnused();
    }
    host.movevars_changed = true_0;
    Host_SetServerState(ss_active as libc::c_int);
    Con_DPrintf(b"level loaded at %.2f sec\n\x00" as *const u8 as
                    *const libc::c_char, Sys_DoubleTime() - svs.timestart);
    if sv.ignored_static_ents != 0 {
        Con_Printf(b"^3Warning:^7 %i static entities was rejected due buffer overflow\n\x00"
                       as *const u8 as *const libc::c_char,
                   sv.ignored_static_ents);
    }
    if sv.ignored_world_decals != 0 {
        Con_Printf(b"^3Warning:^7 %i static decals was rejected due buffer overflow\n\x00"
                       as *const u8 as *const libc::c_char,
                   sv.ignored_world_decals);
    }
    if svs.maxclients > 1 as libc::c_int {
        let mut cycle: *const libc::c_char =
            Cvar_VariableString(b"mapchangecfgfile\x00" as *const u8 as
                                    *const libc::c_char);
        if if cycle.is_null() || *cycle == 0 {
               0 as libc::c_int
           } else { 1 as libc::c_int } != 0 {
            Cbuf_AddText(va(b"exec %s\n\x00" as *const u8 as
                                *const libc::c_char, cycle));
        }
        if (*public_server).value != 0. { Master_Add(); }
    };
}
/*
================
SV_DeactivateServer

deactivate server, free edicts, strings etc
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_DeactivateServer() {
    let mut i: libc::c_int = 0;
    if svs.initialized as u64 == 0 ||
           sv.state as libc::c_uint == ss_dead as libc::c_int as libc::c_uint
       {
        return
    }
    (*svgame.globals).time = sv.time as libc::c_float;
    svgame.dllFuncs.pfnServerDeactivate.expect("non-null function pointer")();
    Host_SetServerState(ss_dead as libc::c_int);
    SV_FreeEdicts();
    SV_ClearPhysEnts();
    SV_EmptyStringPool();
    i = 0 as libc::c_int;
    while i < svs.maxclients {
        // release client frames
        if !(*svs.clients.offset(i as isize)).frames.is_null() {
            _Mem_Free((*svs.clients.offset(i as isize)).frames as
                          *mut libc::c_void,
                      b"../engine/server/sv_init.c\x00" as *const u8 as
                          *const libc::c_char,
                      649 as libc::c_int); // clients + world
        }
        let ref mut fresh0 = (*svs.clients.offset(i as isize)).frames;
        *fresh0 = 0 as *mut client_frame_t;
        i += 1
    }
    (*svgame.globals).maxEntities = (*SI.GameInfo).max_edicts;
    (*svgame.globals).maxClients = svs.maxclients;
    svgame.numEntities = svs.maxclients + 1 as libc::c_int;
    (*svgame.globals).startspot = 0 as libc::c_int;
    (*svgame.globals).mapname = 0 as libc::c_int;
}
/*
==============
SV_InitGame

A brand new game has been started
==============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_InitGame() -> qboolean {
    let mut dllpath: string = [0; 256];
    if svs.game_library_loaded as u64 != 0 { return true_0 }
    // first initialize?
    COM_ResetLibraryError();
    COM_GetCommonLibraryPath(LIBRARY_SERVER, dllpath.as_mut_ptr(),
                             ::std::mem::size_of::<string>() as
                                 libc::c_ulong);
    if SV_LoadProgs(dllpath.as_mut_ptr()) as u64 == 0 {
        Con_Printf(b"^1Error:^7 can\'t initialize %s: %s\n\x00" as *const u8
                       as *const libc::c_char, dllpath.as_mut_ptr(),
                   COM_GetLibraryError());
        return false_0
        // failed to loading server.dll
    }
    // client frames will be allocated in SV_ClientConnect
    svs.game_library_loaded = true_0;
    return true_0;
}
/*
==============
SV_ShutdownGame

prepare to close server
==============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_ShutdownGame() {
    if host.game.loadGame as u64 == 0 {
        SV_ClearGameState(); // stop demo too
    }
    SV_FinalMessage(b"\x00" as *const u8 as *const libc::c_char, true_0);
    S_StopBackgroundTrack();
    CL_StopPlayback();
    if host.game.newGame as u64 != 0 {
        Host_EndGame(false_0,
                     b"The End\x00" as *const u8 as *const libc::c_char);
    } else { S_StopAllSounds(true_0); SV_DeactivateServer(); };
}
/*
================
SV_SetupClients

determine the game type and prepare clients
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_SetupClients() {
    let mut changed_maxclients: qboolean = false_0;
    // check if clients count was really changed
    if svs.maxclients != (*sv_maxclients).value as libc::c_int {
        changed_maxclients = true_0
    } // nothing to change
    if changed_maxclients as u64 == 0 { return }
    // if clients count was changed we need to run full shutdown procedure
    if svs.maxclients != 0 { Host_ShutdownServer(); }
    // copy the actual value from cvar
    svs.maxclients = (*sv_maxclients).value as libc::c_int;
    // dedicated servers are can't be single player and are usually DM
    if host.type_0 == HOST_DEDICATED as libc::c_int as libc::c_uint {
        svs.maxclients =
            if svs.maxclients >= 4 as libc::c_int {
                if svs.maxclients < (1 as libc::c_int) << 5 as libc::c_int {
                    svs.maxclients
                } else { ((1 as libc::c_int)) << 5 as libc::c_int }
            } else { 4 as libc::c_int }
    } else {
        svs.maxclients =
            if svs.maxclients >= 1 as libc::c_int {
                if svs.maxclients < (1 as libc::c_int) << 5 as libc::c_int {
                    svs.maxclients
                } else { ((1 as libc::c_int)) << 5 as libc::c_int }
            } else { 1 as libc::c_int }
    }
    if svs.maxclients == 1 as libc::c_int {
        Cvar_SetValue(b"deathmatch\x00" as *const u8 as *const libc::c_char,
                      0.0f32);
    } else {
        Cvar_SetValue(b"deathmatch\x00" as *const u8 as *const libc::c_char,
                      1.0f32);
    }
    // make cvars consistant
    if coop.value != 0. {
        Cvar_SetValue(b"deathmatch\x00" as *const u8 as *const libc::c_char,
                      0.0f32);
    }
    // feedback for cvar
    Cvar_FullSet(b"maxplayers\x00" as *const u8 as *const libc::c_char,
                 va(b"%d\x00" as *const u8 as *const libc::c_char,
                    svs.maxclients), (1 as libc::c_int) << 30 as libc::c_int);
    SV_UPDATE_BACKUP =
        if svs.maxclients == 1 as libc::c_int {
            16 as libc::c_int
        } else { 64 as libc::c_int };
    svs.clients =
        _Mem_Realloc(host.mempool, svs.clients as *mut libc::c_void,
                     (::std::mem::size_of::<sv_client_t>() as
                          libc::c_ulong).wrapping_mul(svs.maxclients as
                                                          libc::c_ulong),
                     true_0,
                     b"../engine/server/sv_init.c\x00" as *const u8 as
                         *const libc::c_char, 758 as libc::c_int) as
            *mut sv_client_t;
    svs.num_client_entities =
        svs.maxclients * SV_UPDATE_BACKUP * 256 as libc::c_int;
    svs.packet_entities =
        _Mem_Realloc(host.mempool, svs.packet_entities as *mut libc::c_void,
                     (::std::mem::size_of::<entity_state_t>() as
                          libc::c_ulong).wrapping_mul(svs.num_client_entities
                                                          as libc::c_ulong),
                     true_0,
                     b"../engine/server/sv_init.c\x00" as *const u8 as
                         *const libc::c_char, 760 as libc::c_int) as
            *mut entity_state_t;
    Con_Reportf(b"%s alloced by server packet entities\n\x00" as *const u8 as
                    *const libc::c_char,
                Q_pretifymem((::std::mem::size_of::<entity_state_t>() as
                                  libc::c_ulong).wrapping_mul(svs.num_client_entities
                                                                  as
                                                                  libc::c_ulong)
                                 as libc::c_float, 2 as libc::c_int));
    // init network stuff
    NET_Config((svs.maxclients > 1 as libc::c_int) as libc::c_int as
                   qboolean); // clients + world
    svgame.numEntities = svs.maxclients + 1 as libc::c_int;
    (*sv_maxclients).flags =
        (*sv_maxclients).flags & !((1 as libc::c_int) << 13 as libc::c_int);
}
unsafe extern "C" fn CRC32_MapFile(mut crcvalue: *mut dword,
                                   mut filename: *const libc::c_char,
                                   mut multiplayer: qboolean) -> qboolean {
    let mut headbuf: [libc::c_char; 1024] = [0; 1024];
    let mut buffer: [libc::c_char; 1024] = [0; 1024];
    let mut i: libc::c_int = 0;
    let mut num_bytes: libc::c_int = 0;
    let mut lumplen: libc::c_int = 0;
    let mut version: libc::c_int = 0;
    let mut hdr_size: libc::c_int = 0;
    let mut header: *mut dheader_t = 0 as *mut dheader_t;
    let mut f: *mut file_t = 0 as *mut file_t;
    if crcvalue.is_null() { return false_0 }
    // always calc same checksum for singleplayer
    if multiplayer as libc::c_uint == false_0 as libc::c_int as libc::c_uint {
        *crcvalue =
            ((('H' as i32) << 24 as libc::c_int) +
                 (('S' as i32) << 16 as libc::c_int) +
                 (('A' as i32) << 8 as libc::c_int) + 'X' as i32) as dword;
        return true_0
    }
    f =
        FS_Open(filename, b"rb\x00" as *const u8 as *const libc::c_char,
                false_0);
    if f.is_null() { return false_0 }
    // read version number
    FS_Read(f, &mut version as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    FS_Seek(f, 0 as libc::c_int as fs_offset_t, 0 as libc::c_int);
    hdr_size =
        (::std::mem::size_of::<libc::c_int>() as
             libc::c_ulong).wrapping_add((::std::mem::size_of::<dlump_t>() as
                                              libc::c_ulong).wrapping_mul(15
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_ulong))
            as libc::c_int;
    num_bytes =
        FS_Read(f, headbuf.as_mut_ptr() as *mut libc::c_void,
                hdr_size as size_t) as libc::c_int;
    // corrupted map ?
    if num_bytes != hdr_size { FS_Close(f); return false_0 }
    header = headbuf.as_mut_ptr() as *mut dheader_t;
    // invalid version ?
    match (*header).version {
        29 | 30 | 844124994 => { }
        _ => { FS_Close(f); return false_0 }
    }
    CRC32_Init(crcvalue);
    i = 1 as libc::c_int;
    while i < 15 as libc::c_int {
        lumplen = (*header).lumps[i as usize].filelen;
        FS_Seek(f, (*header).lumps[i as usize].fileofs as fs_offset_t,
                0 as libc::c_int);
        while lumplen > 0 as libc::c_int {
            if lumplen as libc::c_ulong >=
                   ::std::mem::size_of::<[libc::c_char; 1024]>() as
                       libc::c_ulong {
                num_bytes =
                    FS_Read(f, buffer.as_mut_ptr() as *mut libc::c_void,
                            ::std::mem::size_of::<[libc::c_char; 1024]>() as
                                libc::c_ulong) as libc::c_int
            } else {
                num_bytes =
                    FS_Read(f, buffer.as_mut_ptr() as *mut libc::c_void,
                            lumplen as size_t) as libc::c_int
            }
            if num_bytes > 0 as libc::c_int {
                lumplen -= num_bytes;
                CRC32_ProcessBuffer(crcvalue,
                                    buffer.as_mut_ptr() as
                                        *const libc::c_void, num_bytes);
            }
            // file unexpected end ?
            if FS_Eof(f) as u64 != 0 { break ; }
        }
        i += 1
    }
    FS_Close(f);
    return true_0;
}
/*
================
SV_SpawnServer

Change the server to a new map, taking all connected
clients along with it.
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_SpawnServer(mut mapname: *const libc::c_char,
                                        mut startspot: *const libc::c_char,
                                        mut background: qboolean)
 -> qboolean {
    let mut i: libc::c_int =
        0; // any partially connected client will be restarted
    let mut current_skill: libc::c_int = 0;
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    SV_SetupClients();
    if SV_InitGame() as u64 == 0 { return false_0 }
    svs.initialized = true_0;
    Log_Open();
    Log_Printf(b"Loading map \"%s\"\n\x00" as *const u8 as
                   *const libc::c_char, mapname);
    Log_PrintServerVars();
    svs.timestart = Sys_DoubleTime();
    svs.spawncount += 1;
    // let's not have any servers with no name
    if if hostname.string.is_null() || *hostname.string == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        Cvar_Set(b"hostname\x00" as *const u8 as *const libc::c_char,
                 if svgame.dllFuncs.pfnGetGameDescription.is_some() {
                     svgame.dllFuncs.pfnGetGameDescription.expect("non-null function pointer")()
                 } else {
                     (*SI.GameInfo).title.as_mut_ptr() as *const libc::c_char
                 }); // wipe the entire per-level structure
    } // server spawn time it's always 1.0 second
    if !startspot.is_null() {
        Con_Printf(b"Spawn Server: %s [%s]\n\x00" as *const u8 as
                       *const libc::c_char, mapname, startspot);
    } else {
        Con_DPrintf(b"Spawn Server: %s\n\x00" as *const u8 as
                        *const libc::c_char, mapname);
    }
    memset(&mut sv as *mut server_t as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<server_t>() as libc::c_ulong);
    (*svgame.globals).time = 1.0f32;
    sv.time = (*svgame.globals).time as libc::c_double;
    sv.background = background;
    // initialize buffers
    MSG_InitExt(&mut sv.signon,
                b"Signon\x00" as *const u8 as *const libc::c_char,
                sv.signon_buf.as_mut_ptr() as *mut libc::c_void,
                ::std::mem::size_of::<[byte; 131072]>() as libc::c_ulong as
                    libc::c_int, -(1 as libc::c_int));
    MSG_InitExt(&mut sv.multicast,
                b"Multicast\x00" as *const u8 as *const libc::c_char,
                sv.multicast_buf.as_mut_ptr() as *mut libc::c_void,
                ::std::mem::size_of::<[byte; 8192]>() as libc::c_ulong as
                    libc::c_int, -(1 as libc::c_int));
    MSG_InitExt(&mut sv.datagram,
                b"Datagram\x00" as *const u8 as *const libc::c_char,
                sv.datagram_buf.as_mut_ptr() as *mut libc::c_void,
                ::std::mem::size_of::<[byte; 16384]>() as libc::c_ulong as
                    libc::c_int, -(1 as libc::c_int));
    MSG_InitExt(&mut sv.reliable_datagram,
                b"Reliable Datagram\x00" as *const u8 as *const libc::c_char,
                sv.reliable_datagram_buf.as_mut_ptr() as *mut libc::c_void,
                ::std::mem::size_of::<[byte; 16384]>() as libc::c_ulong as
                    libc::c_int, -(1 as libc::c_int));
    MSG_InitExt(&mut sv.spec_datagram,
                b"Spectator Datagram\x00" as *const u8 as *const libc::c_char,
                sv.spectator_buf.as_mut_ptr() as *mut libc::c_void,
                ::std::mem::size_of::<[byte; 8192]>() as libc::c_ulong as
                    libc::c_int, -(1 as libc::c_int));
    // clearing all the baselines
    memset(svs.static_entities as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<entity_state_t>() as
                libc::c_ulong).wrapping_mul(3096 as libc::c_int as
                                                libc::c_ulong));
    memset(svs.baselines as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<entity_state_t>() as
                libc::c_ulong).wrapping_mul((*SI.GameInfo).max_edicts as
                                                libc::c_ulong));
    // make cvars consistant
    if coop.value != 0. {
        Cvar_SetValue(b"deathmatch\x00" as *const u8 as *const libc::c_char,
                      0 as libc::c_int as libc::c_float);
    }
    current_skill =
        if skill.value < 0.0f32 {
            (skill.value - 0.5f32) as libc::c_int
        } else { (skill.value + 0.5f32) as libc::c_int };
    current_skill =
        if current_skill >= 0 as libc::c_int {
            if current_skill < 3 as libc::c_int {
                current_skill
            } else { 3 as libc::c_int }
        } else { 0 as libc::c_int };
    Cvar_SetValue(b"skill\x00" as *const u8 as *const libc::c_char,
                  current_skill as libc::c_float);
    // force normal player collisions for single player
    if svs.maxclients == 1 as libc::c_int {
        Cvar_SetValue(b"sv_clienttrace\x00" as *const u8 as
                          *const libc::c_char,
                      1 as libc::c_int as libc::c_float);
    }
    // copy gamemode into svgame.globals
    (*svgame.globals).deathmatch = deathmatch.value;
    (*svgame.globals).coop = coop.value;
    (*svgame.globals).maxClients = svs.maxclients;
    if sv.background as u64 != 0 {
        // tell the game parts about background state
        Cvar_FullSet(b"sv_background\x00" as *const u8 as *const libc::c_char,
                     b"1\x00" as *const u8 as *const libc::c_char,
                     (1 as libc::c_int) << 17 as libc::c_int);
        Cvar_FullSet(b"cl_background\x00" as *const u8 as *const libc::c_char,
                     b"1\x00" as *const u8 as *const libc::c_char,
                     (1 as libc::c_int) << 17 as libc::c_int);
    } else {
        Cvar_FullSet(b"sv_background\x00" as *const u8 as *const libc::c_char,
                     b"0\x00" as *const u8 as *const libc::c_char,
                     (1 as libc::c_int) << 17 as libc::c_int);
        Cvar_FullSet(b"cl_background\x00" as *const u8 as *const libc::c_char,
                     b"0\x00" as *const u8 as *const libc::c_char,
                     (1 as libc::c_int) << 17 as libc::c_int);
    }
    // force normal player collisions for single player
    if svs.maxclients == 1 as libc::c_int {
        Cvar_SetValue(b"sv_clienttrace\x00" as *const u8 as
                          *const libc::c_char,
                      1 as libc::c_int as libc::c_float);
    }
    // make sure what server name doesn't contain path and extension
    COM_FileBase(mapname, sv.name.as_mut_ptr());
    // precache and static commands can be issued during map initialization
    Host_SetServerState(ss_loading as libc::c_int);
    if !startspot.is_null() {
        Q_strncpy(sv.startspot.as_mut_ptr(), startspot,
                  ::std::mem::size_of::<[libc::c_char; 64]>() as
                      libc::c_ulong);
    } else {
        sv.startspot[0 as libc::c_int as usize] =
            '\u{0}' as i32 as libc::c_char
    }
    Q_snprintf(sv.model_precache[1 as libc::c_int as usize].as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
               b"maps/%s.bsp\x00" as *const u8 as *const libc::c_char,
               sv.name.as_mut_ptr());
    sv.model_precache_flags[1 as libc::c_int as usize] =
        (sv.model_precache_flags[1 as libc::c_int as usize] as libc::c_int |
             (1 as libc::c_int) << 0 as libc::c_int) as byte;
    sv.models[1 as libc::c_int as usize] =
        Mod_LoadWorld(sv.model_precache[1 as libc::c_int as
                                            usize].as_mut_ptr(), true_0);
    sv.worldmodel = sv.models[1 as libc::c_int as usize];
    CRC32_MapFile(&mut sv.worldmapCRC,
                  sv.model_precache[1 as libc::c_int as usize].as_mut_ptr(),
                  (svs.maxclients > 1 as libc::c_int) as libc::c_int as
                      qboolean);
    if host.features &
           ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint != 0 &&
           FS_FileExists(b"progs.dat\x00" as *const u8 as *const libc::c_char,
                         false_0 as libc::c_int) != 0 {
        let mut f: *mut file_t =
            FS_Open(b"progs.dat\x00" as *const u8 as *const libc::c_char,
                    b"rb\x00" as *const u8 as *const libc::c_char, false_0);
        FS_Seek(f,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                    fs_offset_t, 0 as libc::c_int);
        FS_Read(f, &mut sv.progsCRC as *mut libc::c_int as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
        FS_Close(f);
    }
    i = 1 as libc::c_int;
    while i < (*sv.worldmodel).numsubmodels {
        Q_sprintf(sv.model_precache[(i + 1 as libc::c_int) as
                                        usize].as_mut_ptr(),
                  b"*%i\x00" as *const u8 as *const libc::c_char, i);
        sv.models[(i + 1 as libc::c_int) as usize] =
            Mod_ForName(sv.model_precache[(i + 1 as libc::c_int) as
                                              usize].as_mut_ptr(), false_0,
                        false_0);
        sv.model_precache_flags[(i + 1 as libc::c_int) as usize] =
            (sv.model_precache_flags[(i + 1 as libc::c_int) as usize] as
                 libc::c_int | (1 as libc::c_int) << 0 as libc::c_int) as
                byte;
        i += 1
    }
    // leave slots at start for clients only
    i = 0 as libc::c_int;
    while i < svs.maxclients {
        // needs to reconnect
        if (*svs.clients.offset(i as isize)).state as libc::c_uint >
               cs_connected as libc::c_int as libc::c_uint {
            (*svs.clients.offset(i as isize)).state = cs_connected
        }
        ent = SV_EdictNum(i + 1 as libc::c_int);
        let ref mut fresh1 = (*svs.clients.offset(i as isize)).pViewEntity;
        *fresh1 = 0 as *mut edict_t;
        let ref mut fresh2 = (*svs.clients.offset(i as isize)).edict;
        *fresh2 = ent;
        SV_InitEdict(ent);
        i += 1
    }
    // heartbeats will always be sent to the id master
    svs.last_heartbeat =
        -(99999 as libc::c_int) as libc::c_double; // send immediately
    // get actual movevars
    SV_UpdateMovevars(true_0);
    // clear physics interaction links
    SV_ClearWorld(); // already loaded
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn SV_Active() -> qboolean {
    return (sv.state as libc::c_uint !=
                ss_dead as libc::c_int as libc::c_uint) as libc::c_int as
               qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn SV_Initialized() -> qboolean {
    return svs.initialized;
}
#[no_mangle]
pub unsafe extern "C" fn SV_GetMaxClients() -> libc::c_int {
    return svs.maxclients;
}
#[no_mangle]
pub unsafe extern "C" fn SV_InitGameProgs() {
    let mut dllpath: string = [0; 256];
    if !svgame.hInstance.is_null() { return }
    COM_GetCommonLibraryPath(LIBRARY_SERVER, dllpath.as_mut_ptr(),
                             ::std::mem::size_of::<string>() as
                                 libc::c_ulong);
    // just try to initialize
    SV_LoadProgs(dllpath.as_mut_ptr()); // server is active
}
#[no_mangle]
pub unsafe extern "C" fn SV_FreeGameProgs() {
    if svs.initialized as u64 != 0 { return }
    // unload progs (free cvars and commands)
    SV_UnloadProgs();
}
/*
================
SV_ExecLoadLevel

State machine exec new map
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_ExecLoadLevel() {
    SV_SetStringArrayMode(false_0);
    if SV_SpawnServer(host.game.levelName.as_mut_ptr(),
                      0 as *const libc::c_char, host.game.backgroundMap) as
           u64 != 0 {
        SV_SpawnEntities(host.game.levelName.as_mut_ptr());
        SV_ActivateServer(true_0 as libc::c_int);
    };
}
/*
================
SV_ExecLoadGame

State machine exec load saved game
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_ExecLoadGame() {
    if SV_SpawnServer(host.game.levelName.as_mut_ptr(),
                      0 as *const libc::c_char, false_0) as u64 != 0 {
        if SV_LoadGameState(host.game.levelName.as_mut_ptr()) == 0 {
            SV_SpawnEntities(host.game.levelName.as_mut_ptr());
        }
        SV_ActivateServer(false_0 as libc::c_int);
    };
}
/*
================
SV_ExecChangeLevel

State machine exec changelevel path
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_ExecChangeLevel() {
    SV_ChangeLevel(host.game.loadGame, host.game.levelName.as_mut_ptr(),
                   host.game.landmarkName.as_mut_ptr(),
                   host.game.backgroundMap);
}
