#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, extern_types,
           ptr_wrapping_offset_from, register_tool)]
extern "C" {
    pub type file_s;
    pub type grasshdr_s;
    pub type ref_viewpass_s;
    pub type mip_s;
    #[no_mangle]
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strftime(__s: *mut libc::c_char, __maxsize: size_t,
                __format: *const libc::c_char, __tp: *const tm) -> size_t;
    #[no_mangle]
    fn localtime(__timer: *const time_t) -> *mut tm;
    #[no_mangle]
    fn Q_strncat(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Q_strnicmp(s1: *const libc::c_char, s2: *const libc::c_char,
                  n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Q_strncmp(s1: *const libc::c_char, s2: *const libc::c_char,
                 n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Q_strstr(string: *const libc::c_char, string2: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn Q_snprintf(buffer: *mut libc::c_char, buffersize: size_t,
                  format: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn COM_FileWithoutPath(in_0: *const libc::c_char) -> *const libc::c_char;
    #[no_mangle]
    fn Sys_Error(error: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Cvar_FullSet(var_name: *const libc::c_char, value: *const libc::c_char,
                    flags: libc::c_int);
    #[no_mangle]
    fn Cvar_Set(var_name: *const libc::c_char, value: *const libc::c_char);
    #[no_mangle]
    fn Cvar_SetValue(var_name: *const libc::c_char, value: libc::c_float);
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    static mut SI: sysinfo_t;
    #[no_mangle]
    fn Cbuf_AddText(text: *const libc::c_char);
    #[no_mangle]
    fn _Mem_Alloc(poolptr: poolhandle_t, size: size_t, clear: qboolean,
                  filename: *const libc::c_char, fileline: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    fn _Mem_Free(data: *mut libc::c_void, filename: *const libc::c_char,
                 fileline: libc::c_int);
    #[no_mangle]
    fn COM_FixSlashes(pname: *mut libc::c_char);
    #[no_mangle]
    fn FS_Search(pattern: *const libc::c_char, caseinsensitive: libc::c_int,
                 gamedironly: libc::c_int) -> *mut search_t;
    #[no_mangle]
    fn FS_Open(filepath: *const libc::c_char, mode: *const libc::c_char,
               gamedironly: qboolean) -> *mut file_t;
    #[no_mangle]
    fn FS_Write(file: *mut file_t, data: *const libc::c_void,
                datasize: size_t) -> fs_offset_t;
    #[no_mangle]
    fn FS_Read(file: *mut file_t, buffer: *mut libc::c_void,
               buffersize: size_t) -> fs_offset_t;
    #[no_mangle]
    fn FS_FileTime(filename: *const libc::c_char, gamedironly: qboolean)
     -> libc::c_int;
    #[no_mangle]
    fn FS_Rename(oldname: *const libc::c_char, newname: *const libc::c_char)
     -> qboolean;
    #[no_mangle]
    fn FS_FileExists(filename: *const libc::c_char, gamedironly: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn FS_FileCopy(pOutput: *mut file_t, pInput: *mut file_t,
                   fileSize: libc::c_int) -> qboolean;
    #[no_mangle]
    fn FS_Delete(path: *const libc::c_char) -> qboolean;
    #[no_mangle]
    fn FS_Close(file: *mut file_t) -> libc::c_int;
    #[no_mangle]
    fn FS_FileLength(f: *mut file_t) -> fs_offset_t;
    #[no_mangle]
    fn Host_CompareFileTime(ft1: libc::c_int, ft2: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn Host_Error(error: *const libc::c_char, _: ...);
    #[no_mangle]
    fn COM_LoadGame(pSaveFileName: *const libc::c_char);
    #[no_mangle]
    fn CL_Active() -> libc::c_int;
    #[no_mangle]
    fn pfnDecalIndex(m: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn Con_Reportf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_DPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn GL_FreeImage(name: *const libc::c_char);
    #[no_mangle]
    fn CL_IsIntermission() -> qboolean;
    #[no_mangle]
    fn SV_CreateDecal(msg: *mut sizebuf_t, origin: *const libc::c_float,
                      decalIndex: libc::c_int, entityIndex: libc::c_int,
                      modelIndex: libc::c_int, flags: libc::c_int,
                      scale: libc::c_float);
    #[no_mangle]
    fn SV_RestoreCustomDecal(entry: *mut decallist_s, pEdict: *mut edict_t,
                             adjacent: qboolean) -> qboolean;
    #[no_mangle]
    fn S_StreamGetCurrentState(currentTrack: *mut libc::c_char,
                               loopTrack: *mut libc::c_char,
                               position: *mut libc::c_int) -> qboolean;
    #[no_mangle]
    fn CL_HudMessage(pMessage: *const libc::c_char);
    #[no_mangle]
    fn UI_CreditsActive() -> qboolean;
    #[no_mangle]
    fn SV_PointContents(p: *const vec_t) -> libc::c_int;
    #[no_mangle]
    fn SV_MapIsValid(filename: *const libc::c_char,
                     spawn_entity: *const libc::c_char,
                     landmark_name: *const libc::c_char) -> uint;
    #[no_mangle]
    fn SV_SetLightStyle(style: libc::c_int, s: *const libc::c_char,
                        f: libc::c_float);
    #[no_mangle]
    fn SV_Move(start: *const vec_t, mins: *mut vec_t, maxs: *mut vec_t,
               end: *const vec_t, type_0: libc::c_int, e: *mut edict_t,
               monsterclip: qboolean) -> trace_t;
    #[no_mangle]
    fn SV_EdictNum(n: libc::c_int) -> *mut edict_t;
    #[no_mangle]
    fn pfnIndexOfEdict(pEdict: *const edict_t) -> libc::c_int;
    #[no_mangle]
    fn SV_CreateStaticEntity(msg: *mut sizebuf_s, index: libc::c_int)
     -> qboolean;
    #[no_mangle]
    fn SV_FindGlobalEntity(classname: string_t, globalname: string_t)
     -> *mut edict_t;
    #[no_mangle]
    fn SV_GetString(iString: string_t) -> *const libc::c_char;
    #[no_mangle]
    fn SV_MakeString(szValue: *const libc::c_char) -> string_t;
    #[no_mangle]
    fn SV_CreateNamedEntity(ent: *mut edict_t, className: string_t)
     -> *mut edict_t;
    #[no_mangle]
    fn SV_SpawnEntities(mapname: *const libc::c_char);
    #[no_mangle]
    fn SV_BuildSoundMsg(msg: *mut sizebuf_t, ent: *mut edict_t,
                        chan: libc::c_int, sample: *const libc::c_char,
                        vol: libc::c_int, attn: libc::c_float,
                        flags: libc::c_int, pitch: libc::c_int,
                        pos: *const vec_t) -> libc::c_int;
    #[no_mangle]
    fn SV_CheckEdict(e: *const edict_t, file: *const libc::c_char,
                     line: libc::c_int) -> qboolean;
    #[no_mangle]
    static mut vec3_origin: vec3_t;
    #[no_mangle]
    fn MSG_WriteCmdExt(sb: *mut sizebuf_t, cmd: libc::c_int, type_0: netsrc_t,
                       name: *const libc::c_char);
    #[no_mangle]
    fn MSG_WriteByte(sb: *mut sizebuf_t, val: libc::c_int);
    #[no_mangle]
    fn MSG_WriteBytes(sb: *mut sizebuf_t, pBuf: *const libc::c_void,
                      nBytes: libc::c_int) -> qboolean;
    #[no_mangle]
    fn MSG_WriteString(sb: *mut sizebuf_t, pStr: *const libc::c_char)
     -> qboolean;
    #[no_mangle]
    static mut svs: server_static_t;
    #[no_mangle]
    static mut sv: server_t;
    #[no_mangle]
    static mut svgame: svgame_static_t;
    #[no_mangle]
    static mut sv_newunit: convar_t;
    #[no_mangle]
    static mut sv_wateralpha: convar_t;
    #[no_mangle]
    static mut sv_wateramp: convar_t;
    #[no_mangle]
    static mut sv_skyname: convar_t;
    #[no_mangle]
    static mut sv_skycolor_r: convar_t;
    #[no_mangle]
    static mut sv_skycolor_g: convar_t;
    #[no_mangle]
    static mut sv_skycolor_b: convar_t;
    #[no_mangle]
    static mut sv_skyvec_x: convar_t;
    #[no_mangle]
    static mut sv_skyvec_y: convar_t;
    #[no_mangle]
    static mut sv_skyvec_z: convar_t;
    #[no_mangle]
    static mut skill: convar_t;
    #[no_mangle]
    fn SV_FinalMessage(message: *const libc::c_char, reconnect: qboolean);
    #[no_mangle]
    fn SV_InitGame() -> qboolean;
    #[no_mangle]
    fn SV_ActivateServer(runPhysics: libc::c_int);
    #[no_mangle]
    fn SV_SpawnServer(server: *const libc::c_char,
                      startspot: *const libc::c_char, background: qboolean)
     -> qboolean;
    #[no_mangle]
    fn SV_DeactivateServer();
    #[no_mangle]
    fn SV_FreeOldEntities();
    #[no_mangle]
    fn SV_InactivateClients();
    #[no_mangle]
    fn SV_InitEdict(pEdict: *mut edict_t);
    #[no_mangle]
    fn COM_GetProcAddress(hInstance: *mut libc::c_void,
                          name: *const libc::c_char) -> *mut libc::c_void;
    #[no_mangle]
    fn S_GetCurrentDynamicSounds(pout: *mut soundlist_t, size: libc::c_int)
     -> libc::c_int;
    #[link_name = "ref"]
    static mut ref_0: ref_state_s;
}
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type off_t = __off_t;
pub type HANDLE = *mut libc::c_void;
pub type HINSTANCE = *mut libc::c_void;
pub type time_t = __time_t;
pub type uint = libc::c_uint;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
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
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct search_t {
    pub numfilenames: libc::c_int,
    pub filenames: *mut *mut libc::c_char,
    pub filenamesbuffer: *mut libc::c_char,
}
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
pub type vec2_t = [vec_t; 2];
pub type vec3_t = [vec_t; 3];
pub type rgba_t = [byte; 4];
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
pub struct soundlist_t {
    pub name: [libc::c_char; 64],
    pub entnum: libc::c_short,
    pub origin: vec3_t,
    pub volume: libc::c_float,
    pub attenuation: libc::c_float,
    pub looping: qboolean,
    pub channel: byte,
    pub pitch: byte,
    pub wordIndex: byte,
    pub samplePos: libc::c_double,
    pub forcedEnd: libc::c_double,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rgbdata_s {
    pub width: word,
    pub height: word,
    pub depth: word,
    pub type_0: uint,
    pub flags: uint,
    pub encode: word,
    pub numMips: byte,
    pub palette: *mut byte,
    pub buffer: *mut byte,
    pub fogParams: rgba_t,
    pub size: size_t,
}
pub type rgbdata_t = rgbdata_s;
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
pub struct KeyValueData_s {
    pub szClassName: *mut libc::c_char,
    pub szKeyName: *mut libc::c_char,
    pub szValue: *mut libc::c_char,
    pub fHandled: libc::c_int,
}
pub type KeyValueData = KeyValueData_s;
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
pub type SAVERESTOREDATA = saverestore_s;
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
pub type FIELDTYPE = _fieldtypes;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TYPEDESCRIPTION {
    pub fieldType: FIELDTYPE,
    pub fieldName: *const libc::c_char,
    pub fieldOffset: libc::c_int,
    pub fieldSize: libc::c_short,
    pub flags: libc::c_short,
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
pub type TRICULLSTYLE = libc::c_uint;
pub const TRI_NONE: TRICULLSTYLE = 1;
pub const TRI_FRONT: TRICULLSTYLE = 0;
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
pub type physics_interface_t = physics_interface_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpoint_t {
    pub point: vec2_t,
    pub coord: vec2_t,
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
pub type texFlags_t = libc::c_uint;
pub const TF_MULTISAMPLE: texFlags_t = 536870912;
pub const TF_ARB_16BIT: texFlags_t = 268435456;
pub const TF_NOCOMPARE: texFlags_t = 134217728;
pub const TF_ARB_FLOAT: texFlags_t = 67108864;
pub const TF_IMG_UPLOADED: texFlags_t = 33554432;
pub const TF_ALPHACONTRAST: texFlags_t = 4194304;
pub const TF_ATLAS_PAGE: texFlags_t = 2097152;
pub const TF_TEXTURE_3D: texFlags_t = 1048576;
pub const TF_BORDER: texFlags_t = 524288;
pub const TF_UPDATE: texFlags_t = 262144;
pub const TF_FORCE_COLOR: texFlags_t = 131072;
pub const TF_HAS_ALPHA: texFlags_t = 65536;
pub const TF_NORMALMAP: texFlags_t = 32768;
pub const TF_MAKELUMA: texFlags_t = 16384;
pub const TF_HAS_LUMA: texFlags_t = 8192;
pub const TF_NOMIPMAP: texFlags_t = 4096;
pub const TF_CLAMP: texFlags_t = 2048;
pub const TF_SKYSIDE: texFlags_t = 1024;
pub const TF_LUMINANCE: texFlags_t = 512;
pub const TF_QUAKEPAL: texFlags_t = 256;
pub const TF_DEPTHMAP: texFlags_t = 128;
pub const TF_CUBEMAP: texFlags_t = 64;
pub const TF_RECTANGLE: texFlags_t = 32;
pub const TF_ALLOW_EMBOSS: texFlags_t = 16;
pub const TF_EXPAND_SOURCE: texFlags_t = 8;
pub const TF_NOFLIP_TGA: texFlags_t = 4;
pub const TF_KEEP_SOURCE: texFlags_t = 2;
pub const TF_NEAREST: texFlags_t = 1;
pub const TF_COLORMAP: texFlags_t = 0;
pub type decallist_t = decallist_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mstudiotex_s {
    pub name: [libc::c_char; 64],
    pub flags: uint32_t,
    pub width: int32_t,
    pub height: int32_t,
    pub index: int32_t,
}
pub type cl_entity_t = cl_entity_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mstudioseqdesc_s {
    pub label: [libc::c_char; 32],
    pub fps: vec_t,
    pub flags: int32_t,
    pub activity: int32_t,
    pub actweight: int32_t,
    pub numevents: int32_t,
    pub eventindex: int32_t,
    pub numframes: int32_t,
    pub weightlistindex: int32_t,
    pub iklockindex: int32_t,
    pub motiontype: int32_t,
    pub motionbone: int32_t,
    pub linearmovement: vec3_t,
    pub autolayerindex: int32_t,
    pub keyvalueindex: int32_t,
    pub bbmin: vec3_t,
    pub bbmax: vec3_t,
    pub numblends: int32_t,
    pub animindex: int32_t,
    pub blendtype: [int32_t; 2],
    pub blendstart: [vec_t; 2],
    pub blendend: [vec_t; 2],
    pub groupsize: [uint8_t; 2],
    pub numautolayers: uint8_t,
    pub numiklocks: uint8_t,
    pub seqgroup: int32_t,
    pub entrynode: int32_t,
    pub exitnode: int32_t,
    pub nodeflags: uint8_t,
    pub cycleposeindex: uint8_t,
    pub fadeintime: uint8_t,
    pub fadeouttime: uint8_t,
    pub animdescindex: int32_t,
}
pub type mstudioseqdesc_t = mstudioseqdesc_s;
pub type ptype_t = libc::c_uint;
pub const pt_clientcustom: ptype_t = 10;
pub const pt_vox_grav: ptype_t = 9;
pub const pt_vox_slowgrav: ptype_t = 8;
pub const pt_blob2: ptype_t = 7;
pub const pt_blob: ptype_t = 6;
pub const pt_explode2: ptype_t = 5;
pub const pt_explode: ptype_t = 4;
pub const pt_fire: ptype_t = 3;
pub const pt_slowgrav: ptype_t = 2;
pub const pt_grav: ptype_t = 1;
pub const pt_static: ptype_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct particle_s {
    pub org: vec3_t,
    pub color: libc::c_short,
    pub packedColor: libc::c_short,
    pub next: *mut particle_s,
    pub vel: vec3_t,
    pub ramp: libc::c_float,
    pub die: libc::c_float,
    pub type_0: ptype_t,
    pub deathfunc: Option<unsafe extern "C" fn(_: *mut particle_s) -> ()>,
    pub callback: Option<unsafe extern "C" fn(_: *mut particle_s,
                                              _: libc::c_float) -> ()>,
    pub context: libc::c_uchar,
}
pub type particle_t = particle_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct beam_s {
    pub next: *mut BEAM,
    pub type_0: libc::c_int,
    pub flags: libc::c_int,
    pub source: vec3_t,
    pub target: vec3_t,
    pub delta: vec3_t,
    pub t: libc::c_float,
    pub freq: libc::c_float,
    pub die: libc::c_float,
    pub width: libc::c_float,
    pub amplitude: libc::c_float,
    pub r: libc::c_float,
    pub g: libc::c_float,
    pub b: libc::c_float,
    pub brightness: libc::c_float,
    pub speed: libc::c_float,
    pub frameRate: libc::c_float,
    pub frame: libc::c_float,
    pub segments: libc::c_int,
    pub startEntity: libc::c_int,
    pub endEntity: libc::c_int,
    pub modelIndex: libc::c_int,
    pub frameCount: libc::c_int,
    pub pFollowModel: *mut model_s,
    pub particles: *mut particle_s,
}
pub type BEAM = beam_s;
pub type ref_screen_rotation_e = libc::c_uint;
pub const REF_ROTATE_CCW: ref_screen_rotation_e = 3;
pub const REF_ROTATE_UD: ref_screen_rotation_e = 2;
pub const REF_ROTATE_CW: ref_screen_rotation_e = 1;
pub const REF_ROTATE_NONE: ref_screen_rotation_e = 0;
pub type ref_screen_rotation_t = ref_screen_rotation_e;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ref_interface_s {
    pub R_Init: Option<unsafe extern "C" fn() -> qboolean>,
    pub R_Shutdown: Option<unsafe extern "C" fn() -> ()>,
    pub R_GetConfigName: Option<unsafe extern "C" fn()
                                    -> *const libc::c_char>,
    pub R_SetDisplayTransform: Option<unsafe extern "C" fn(_:
                                                               ref_screen_rotation_t,
                                                           _: libc::c_int,
                                                           _: libc::c_int,
                                                           _: libc::c_float,
                                                           _: libc::c_float)
                                          -> qboolean>,
    pub GL_SetupAttributes: Option<unsafe extern "C" fn(_: libc::c_int)
                                       -> ()>,
    pub GL_InitExtensions: Option<unsafe extern "C" fn() -> ()>,
    pub GL_ClearExtensions: Option<unsafe extern "C" fn() -> ()>,
    pub R_BeginFrame: Option<unsafe extern "C" fn(_: qboolean) -> ()>,
    pub R_RenderScene: Option<unsafe extern "C" fn() -> ()>,
    pub R_EndFrame: Option<unsafe extern "C" fn() -> ()>,
    pub R_PushScene: Option<unsafe extern "C" fn() -> ()>,
    pub R_PopScene: Option<unsafe extern "C" fn() -> ()>,
    pub GL_BackendStartFrame: Option<unsafe extern "C" fn() -> ()>,
    pub GL_BackendEndFrame: Option<unsafe extern "C" fn() -> ()>,
    pub R_ClearScreen: Option<unsafe extern "C" fn() -> ()>,
    pub R_AllowFog: Option<unsafe extern "C" fn(_: qboolean) -> ()>,
    pub GL_SetRenderMode: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub R_AddEntity: Option<unsafe extern "C" fn(_: *mut cl_entity_s,
                                                 _: libc::c_int) -> qboolean>,
    pub CL_AddCustomBeam: Option<unsafe extern "C" fn(_: *mut cl_entity_t)
                                     -> ()>,
    pub R_ProcessEntData: Option<unsafe extern "C" fn(_: qboolean) -> ()>,
    pub R_ShowTextures: Option<unsafe extern "C" fn() -> ()>,
    pub R_GetTextureOriginalBuffer: Option<unsafe extern "C" fn(_:
                                                                    libc::c_uint)
                                               -> *const byte>,
    pub GL_LoadTextureFromBuffer: Option<unsafe extern "C" fn(_:
                                                                  *const libc::c_char,
                                                              _:
                                                                  *mut rgbdata_t,
                                                              _: texFlags_t,
                                                              _: qboolean)
                                             -> libc::c_int>,
    pub GL_ProcessTexture: Option<unsafe extern "C" fn(_: libc::c_int,
                                                       _: libc::c_float,
                                                       _: libc::c_int,
                                                       _: libc::c_int) -> ()>,
    pub R_SetupSky: Option<unsafe extern "C" fn(_: *const libc::c_char)
                               -> ()>,
    pub R_Set2DMode: Option<unsafe extern "C" fn(_: qboolean) -> ()>,
    pub R_DrawStretchRaw: Option<unsafe extern "C" fn(_: libc::c_float,
                                                      _: libc::c_float,
                                                      _: libc::c_float,
                                                      _: libc::c_float,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: *const byte,
                                                      _: qboolean) -> ()>,
    pub R_DrawStretchPic: Option<unsafe extern "C" fn(_: libc::c_float,
                                                      _: libc::c_float,
                                                      _: libc::c_float,
                                                      _: libc::c_float,
                                                      _: libc::c_float,
                                                      _: libc::c_float,
                                                      _: libc::c_float,
                                                      _: libc::c_float,
                                                      _: libc::c_int) -> ()>,
    pub R_DrawTileClear: Option<unsafe extern "C" fn(_: libc::c_int,
                                                     _: libc::c_int,
                                                     _: libc::c_int,
                                                     _: libc::c_int,
                                                     _: libc::c_int) -> ()>,
    pub FillRGBA: Option<unsafe extern "C" fn(_: libc::c_float,
                                              _: libc::c_float,
                                              _: libc::c_float,
                                              _: libc::c_float,
                                              _: libc::c_int, _: libc::c_int,
                                              _: libc::c_int, _: libc::c_int)
                             -> ()>,
    pub FillRGBABlend: Option<unsafe extern "C" fn(_: libc::c_float,
                                                   _: libc::c_float,
                                                   _: libc::c_float,
                                                   _: libc::c_float,
                                                   _: libc::c_int,
                                                   _: libc::c_int,
                                                   _: libc::c_int,
                                                   _: libc::c_int) -> ()>,
    pub WorldToScreen: Option<unsafe extern "C" fn(_: *const vec_t,
                                                   _: *mut vec_t)
                                  -> libc::c_int>,
    pub VID_ScreenShot: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                    _: libc::c_int)
                                   -> qboolean>,
    pub VID_CubemapShot: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                     _: uint,
                                                     _: *const libc::c_float,
                                                     _: qboolean)
                                    -> qboolean>,
    pub R_LightPoint: Option<unsafe extern "C" fn(_: *const libc::c_float)
                                 -> colorVec>,
    pub R_DecalShoot: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _: libc::c_int,
                                                  _: libc::c_int,
                                                  _: *mut vec_t,
                                                  _: libc::c_int,
                                                  _: libc::c_float) -> ()>,
    pub R_DecalRemoveAll: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub R_CreateDecalList: Option<unsafe extern "C" fn(_: *mut decallist_s)
                                      -> libc::c_int>,
    pub R_ClearAllDecals: Option<unsafe extern "C" fn() -> ()>,
    pub R_StudioEstimateFrame: Option<unsafe extern "C" fn(_:
                                                               *mut cl_entity_t,
                                                           _:
                                                               *mut mstudioseqdesc_t)
                                          -> libc::c_float>,
    pub R_StudioLerpMovement: Option<unsafe extern "C" fn(_: *mut cl_entity_t,
                                                          _: libc::c_double,
                                                          _: *mut vec_t,
                                                          _: *mut vec_t)
                                         -> ()>,
    pub CL_InitStudioAPI: Option<unsafe extern "C" fn() -> ()>,
    pub R_InitSkyClouds: Option<unsafe extern "C" fn(_: *mut mip_s,
                                                     _: *mut texture_s,
                                                     _: qboolean) -> ()>,
    pub GL_SubdivideSurface: Option<unsafe extern "C" fn(_: *mut msurface_t)
                                        -> ()>,
    pub CL_RunLightStyles: Option<unsafe extern "C" fn() -> ()>,
    pub R_GetSpriteParms: Option<unsafe extern "C" fn(_: *mut libc::c_int,
                                                      _: *mut libc::c_int,
                                                      _: *mut libc::c_int,
                                                      _: libc::c_int,
                                                      _: *const model_t)
                                     -> ()>,
    pub R_GetSpriteTexture: Option<unsafe extern "C" fn(_: *const model_t,
                                                        _: libc::c_int)
                                       -> libc::c_int>,
    pub Mod_LoadMapSprite: Option<unsafe extern "C" fn(_: *mut model_s,
                                                       _: *const libc::c_void,
                                                       _: size_t,
                                                       _: *mut qboolean)
                                      -> ()>,
    pub Mod_ProcessRenderData: Option<unsafe extern "C" fn(_: *mut model_t,
                                                           _: qboolean,
                                                           _: *const byte)
                                          -> qboolean>,
    pub Mod_StudioLoadTextures: Option<unsafe extern "C" fn(_: *mut model_t,
                                                            _:
                                                                *mut libc::c_void)
                                           -> ()>,
    pub CL_DrawParticles: Option<unsafe extern "C" fn(_: libc::c_double,
                                                      _: *mut particle_t,
                                                      _: libc::c_float)
                                     -> ()>,
    pub CL_DrawTracers: Option<unsafe extern "C" fn(_: libc::c_double,
                                                    _: *mut particle_t)
                                   -> ()>,
    pub CL_DrawBeams: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _: *mut BEAM) -> ()>,
    pub R_BeamCull: Option<unsafe extern "C" fn(_: *const vec_t,
                                                _: *const vec_t, _: qboolean)
                               -> qboolean>,
    pub RefGetParm: Option<unsafe extern "C" fn(_: libc::c_int,
                                                _: libc::c_int)
                               -> libc::c_int>,
    pub GetDetailScaleForTexture: Option<unsafe extern "C" fn(_: libc::c_int,
                                                              _:
                                                                  *mut libc::c_float,
                                                              _:
                                                                  *mut libc::c_float)
                                             -> ()>,
    pub GetExtraParmsForTexture: Option<unsafe extern "C" fn(_: libc::c_int,
                                                             _: *mut byte,
                                                             _: *mut byte,
                                                             _: *mut byte,
                                                             _: *mut byte)
                                            -> ()>,
    pub GetFrameTime: Option<unsafe extern "C" fn() -> libc::c_float>,
    pub R_SetCurrentEntity: Option<unsafe extern "C" fn(_: *mut cl_entity_s)
                                       -> ()>,
    pub R_SetCurrentModel: Option<unsafe extern "C" fn(_: *mut model_s)
                                      -> ()>,
    pub GL_FindTexture: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                   -> libc::c_int>,
    pub GL_TextureName: Option<unsafe extern "C" fn(_: libc::c_uint)
                                   -> *const libc::c_char>,
    pub GL_TextureData: Option<unsafe extern "C" fn(_: libc::c_uint)
                                   -> *const byte>,
    pub GL_LoadTexture: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                    _: *const byte, _: size_t,
                                                    _: libc::c_int)
                                   -> libc::c_int>,
    pub GL_CreateTexture: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: *const libc::c_void,
                                                      _: texFlags_t)
                                     -> libc::c_int>,
    pub GL_LoadTextureArray: Option<unsafe extern "C" fn(_:
                                                             *mut *const libc::c_char,
                                                         _: libc::c_int)
                                        -> libc::c_int>,
    pub GL_CreateTextureArray: Option<unsafe extern "C" fn(_:
                                                               *const libc::c_char,
                                                           _: libc::c_int,
                                                           _: libc::c_int,
                                                           _: libc::c_int,
                                                           _:
                                                               *const libc::c_void,
                                                           _: texFlags_t)
                                          -> libc::c_int>,
    pub GL_FreeTexture: Option<unsafe extern "C" fn(_: libc::c_uint) -> ()>,
    pub DrawSingleDecal: Option<unsafe extern "C" fn(_: *mut decal_s,
                                                     _: *mut msurface_s)
                                    -> ()>,
    pub R_DecalSetupVerts: Option<unsafe extern "C" fn(_: *mut decal_s,
                                                       _: *mut msurface_s,
                                                       _: libc::c_int,
                                                       _: *mut libc::c_int)
                                      -> *mut libc::c_float>,
    pub R_EntityRemoveDecals: Option<unsafe extern "C" fn(_: *mut model_s)
                                         -> ()>,
    pub AVI_UploadRawFrame: Option<unsafe extern "C" fn(_: libc::c_int,
                                                        _: libc::c_int,
                                                        _: libc::c_int,
                                                        _: libc::c_int,
                                                        _: libc::c_int,
                                                        _: *const byte)
                                       -> ()>,
    pub GL_Bind: Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_uint)
                            -> ()>,
    pub GL_SelectTexture: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub GL_LoadTextureMatrix: Option<unsafe extern "C" fn(_:
                                                              *const libc::c_float)
                                         -> ()>,
    pub GL_TexMatrixIdentity: Option<unsafe extern "C" fn() -> ()>,
    pub GL_CleanUpTextureUnits: Option<unsafe extern "C" fn(_: libc::c_int)
                                           -> ()>,
    pub GL_TexGen: Option<unsafe extern "C" fn(_: libc::c_uint,
                                               _: libc::c_uint) -> ()>,
    pub GL_TextureTarget: Option<unsafe extern "C" fn(_: libc::c_uint) -> ()>,
    pub GL_TexCoordArrayMode: Option<unsafe extern "C" fn(_: libc::c_uint)
                                         -> ()>,
    pub GL_UpdateTexSize: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int) -> ()>,
    pub GL_Reserved0: Option<unsafe extern "C" fn() -> ()>,
    pub GL_Reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub GL_DrawParticles: Option<unsafe extern "C" fn(_:
                                                          *const ref_viewpass_s,
                                                      _: qboolean,
                                                      _: libc::c_float)
                                     -> ()>,
    pub LightVec: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                              _: *const libc::c_float,
                                              _: *mut libc::c_float,
                                              _: *mut libc::c_float)
                             -> colorVec>,
    pub StudioGetTexture: Option<unsafe extern "C" fn(_: *mut cl_entity_s)
                                     -> *mut mstudiotex_s>,
    pub GL_RenderFrame: Option<unsafe extern "C" fn(_: *const ref_viewpass_s)
                                   -> ()>,
    pub GL_OrthoBounds: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                    _: *const libc::c_float)
                                   -> ()>,
    pub R_SpeedsMessage: Option<unsafe extern "C" fn(_: *mut libc::c_char,
                                                     _: size_t) -> qboolean>,
    pub Mod_GetCurrentVis: Option<unsafe extern "C" fn() -> *mut byte>,
    pub R_NewMap: Option<unsafe extern "C" fn() -> ()>,
    pub R_ClearScene: Option<unsafe extern "C" fn() -> ()>,
    pub R_GetProcAddress: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                     -> *mut libc::c_void>,
    pub TriRenderMode: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub Begin: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub End: Option<unsafe extern "C" fn() -> ()>,
    pub Color4f: Option<unsafe extern "C" fn(_: libc::c_float,
                                             _: libc::c_float,
                                             _: libc::c_float,
                                             _: libc::c_float) -> ()>,
    pub Color4ub: Option<unsafe extern "C" fn(_: libc::c_uchar,
                                              _: libc::c_uchar,
                                              _: libc::c_uchar,
                                              _: libc::c_uchar) -> ()>,
    pub TexCoord2f: Option<unsafe extern "C" fn(_: libc::c_float,
                                                _: libc::c_float) -> ()>,
    pub Vertex3fv: Option<unsafe extern "C" fn(_: *const libc::c_float)
                              -> ()>,
    pub Vertex3f: Option<unsafe extern "C" fn(_: libc::c_float,
                                              _: libc::c_float,
                                              _: libc::c_float) -> ()>,
    pub Fog: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                         _: libc::c_float, _: libc::c_float,
                                         _: libc::c_int) -> ()>,
    pub ScreenToWorld: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                   _: *mut libc::c_float)
                                  -> ()>,
    pub GetMatrix: Option<unsafe extern "C" fn(_: libc::c_int,
                                               _: *mut libc::c_float) -> ()>,
    pub FogParams: Option<unsafe extern "C" fn(_: libc::c_float,
                                               _: libc::c_int) -> ()>,
    pub CullFace: Option<unsafe extern "C" fn(_: TRICULLSTYLE) -> ()>,
    pub VGUI_DrawInit: Option<unsafe extern "C" fn() -> ()>,
    pub VGUI_DrawShutdown: Option<unsafe extern "C" fn() -> ()>,
    pub VGUI_SetupDrawingText: Option<unsafe extern "C" fn(_:
                                                               *mut libc::c_int)
                                          -> ()>,
    pub VGUI_SetupDrawingRect: Option<unsafe extern "C" fn(_:
                                                               *mut libc::c_int)
                                          -> ()>,
    pub VGUI_SetupDrawingImage: Option<unsafe extern "C" fn(_:
                                                                *mut libc::c_int)
                                           -> ()>,
    pub VGUI_BindTexture: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub VGUI_EnableTexture: Option<unsafe extern "C" fn(_: qboolean) -> ()>,
    pub VGUI_CreateTexture: Option<unsafe extern "C" fn(_: libc::c_int,
                                                        _: libc::c_int,
                                                        _: libc::c_int)
                                       -> ()>,
    pub VGUI_UploadTexture: Option<unsafe extern "C" fn(_: libc::c_int,
                                                        _:
                                                            *const libc::c_char,
                                                        _: libc::c_int,
                                                        _: libc::c_int)
                                       -> ()>,
    pub VGUI_UploadTextureBlock: Option<unsafe extern "C" fn(_: libc::c_int,
                                                             _: libc::c_int,
                                                             _: libc::c_int,
                                                             _: *const byte,
                                                             _: libc::c_int,
                                                             _: libc::c_int)
                                            -> ()>,
    pub VGUI_DrawQuad: Option<unsafe extern "C" fn(_: *const vpoint_t,
                                                   _: *const vpoint_t) -> ()>,
    pub VGUI_GetTextureSizes: Option<unsafe extern "C" fn(_: *mut libc::c_int,
                                                          _: *mut libc::c_int)
                                         -> ()>,
    pub VGUI_GenerateTexture: Option<unsafe extern "C" fn() -> libc::c_int>,
}
pub type ref_interface_t = ref_interface_s;
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
pub type consistency_t = consistency_s;
pub type playermove_t = playermove_s;
pub type movevars_t = movevars_s;
pub type sv_state_t = libc::c_uint;
pub const ss_active: sv_state_t = 2;
pub const ss_loading: sv_state_t = 1;
pub const ss_dead: sv_state_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sv_baseline_t {
    pub classname: *const libc::c_char,
    pub baseline: entity_state_t,
}
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
pub type server_t = server_s;
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
pub struct sv_user_message_t {
    pub name: [libc::c_char; 32],
    pub number: libc::c_int,
    pub size: libc::c_int,
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
pub struct GAME_HEADER {
    pub mapName: [libc::c_char; 32],
    pub comment: [libc::c_char; 80],
    pub mapCount: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SAVE_CLIENT {
    pub decalCount: libc::c_int,
    pub entityCount: libc::c_int,
    pub soundCount: libc::c_int,
    pub tempEntsCount: libc::c_int,
    pub introTrack: [libc::c_char; 64],
    pub mainTrack: [libc::c_char; 64],
    pub trackPosition: libc::c_int,
    pub viewentity: libc::c_short,
    pub wateralpha: libc::c_float,
    pub wateramp: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ref_state_s {
    pub initialized: qboolean,
    pub hInstance: HINSTANCE,
    pub dllFuncs: ref_interface_t,
    pub numRenderers: libc::c_int,
    pub shortNames: [string; 5],
    pub readableNames: [string; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SAVE_LIGHTSTYLE {
    pub index: libc::c_int,
    pub style: [libc::c_char; 256],
    pub time: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SAVE_HEADER {
    pub skillLevel: libc::c_int,
    pub entityCount: libc::c_int,
    pub connectionCount: libc::c_int,
    pub lightStyleCount: libc::c_int,
    pub time: libc::c_float,
    pub mapName: [libc::c_char; 32],
    pub skyName: [libc::c_char; 32],
    pub skyColor_r: libc::c_int,
    pub skyColor_g: libc::c_int,
    pub skyColor_b: libc::c_int,
    pub skyVec_x: libc::c_float,
    pub skyVec_y: libc::c_float,
    pub skyVec_z: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub mapname: *const libc::c_char,
    pub titlename: *const libc::c_char,
}
#[inline(always)]
unsafe extern "C" fn __tg_fmod(mut __x: libc::c_double,
                               mut __y: libc::c_double) -> libc::c_double {
    return fmod(__x, __y);
}
#[inline]
unsafe extern "C" fn MSG_GetNumBitsLeft(mut sb: *mut sizebuf_t)
 -> libc::c_int {
    return (*sb).nDataBits - (*sb).iCurBit;
}
#[inline]
unsafe extern "C" fn MSG_GetNumBytesLeft(mut sb: *mut sizebuf_t)
 -> libc::c_int {
    return MSG_GetNumBitsLeft(sb) >> 3 as libc::c_int;
}
#[no_mangle]
pub static mut pfnSaveGameComment:
           Option<unsafe extern "C" fn(_: *mut libc::c_char, _: libc::c_int)
                      -> ()> =
    None;
static mut gGameHeader: [TYPEDESCRIPTION; 3] =
    [{
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_CHARACTER,
                             fieldName:
                                 b"mapName\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 0 as libc::c_ulong as libc::c_int,
                             fieldSize: 32 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_CHARACTER,
                             fieldName:
                                 b"comment\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 32 as libc::c_ulong as libc::c_int,
                             fieldSize: 80 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_INTEGER,
                             fieldName:
                                 b"mapCount\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 112 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     }];
static mut gSaveHeader: [TYPEDESCRIPTION; 13] =
    [{
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_INTEGER,
                             fieldName:
                                 b"skillLevel\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 0 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_INTEGER,
                             fieldName:
                                 b"entityCount\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 4 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_INTEGER,
                             fieldName:
                                 b"connectionCount\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 8 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_INTEGER,
                             fieldName:
                                 b"lightStyleCount\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 12 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_TIME,
                             fieldName:
                                 b"time\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 16 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_CHARACTER,
                             fieldName:
                                 b"mapName\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 20 as libc::c_ulong as libc::c_int,
                             fieldSize: 32 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_CHARACTER,
                             fieldName:
                                 b"skyName\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 52 as libc::c_ulong as libc::c_int,
                             fieldSize: 32 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_INTEGER,
                             fieldName:
                                 b"skyColor_r\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 84 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_INTEGER,
                             fieldName:
                                 b"skyColor_g\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 88 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_INTEGER,
                             fieldName:
                                 b"skyColor_b\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 92 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_FLOAT,
                             fieldName:
                                 b"skyVec_x\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 96 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_FLOAT,
                             fieldName:
                                 b"skyVec_y\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 100 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_FLOAT,
                             fieldName:
                                 b"skyVec_z\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 104 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     }];
static mut gAdjacency: [TYPEDESCRIPTION; 4] =
    [{
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_CHARACTER,
                             fieldName:
                                 b"mapName\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 0 as libc::c_ulong as libc::c_int,
                             fieldSize: 32 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_CHARACTER,
                             fieldName:
                                 b"landmarkName\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 32 as libc::c_ulong as libc::c_int,
                             fieldSize: 32 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_EDICT,
                             fieldName:
                                 b"pentLandmark\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 64 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_VECTOR,
                             fieldName:
                                 b"vecLandmarkOrigin\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 72 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     }];
static mut gLightStyle: [TYPEDESCRIPTION; 3] =
    [{
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_INTEGER,
                             fieldName:
                                 b"index\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 0 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_CHARACTER,
                             fieldName:
                                 b"style\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 4 as libc::c_ulong as libc::c_int,
                             fieldSize: 256 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_FLOAT,
                             fieldName:
                                 b"time\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 260 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     }];
static mut gEntityTable: [TYPEDESCRIPTION; 5] =
    [{
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_INTEGER,
                             fieldName:
                                 b"id\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 0 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_INTEGER,
                             fieldName:
                                 b"location\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 16 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_INTEGER,
                             fieldName:
                                 b"size\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 20 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_INTEGER,
                             fieldName:
                                 b"flags\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 24 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_STRING,
                             fieldName:
                                 b"classname\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 28 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     }];
static mut gSaveClient: [TYPEDESCRIPTION; 10] =
    [{
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_INTEGER,
                             fieldName:
                                 b"decalCount\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 0 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_INTEGER,
                             fieldName:
                                 b"entityCount\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 4 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_INTEGER,
                             fieldName:
                                 b"soundCount\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 8 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_INTEGER,
                             fieldName:
                                 b"tempEntsCount\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 12 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_CHARACTER,
                             fieldName:
                                 b"introTrack\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 16 as libc::c_ulong as libc::c_int,
                             fieldSize: 64 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_CHARACTER,
                             fieldName:
                                 b"mainTrack\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 80 as libc::c_ulong as libc::c_int,
                             fieldSize: 64 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_INTEGER,
                             fieldName:
                                 b"trackPosition\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 144 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_SHORT,
                             fieldName:
                                 b"viewentity\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 148 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_FLOAT,
                             fieldName:
                                 b"wateralpha\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 152 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_FLOAT,
                             fieldName:
                                 b"wateramp\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 156 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     }];
static mut gDecalEntry: [TYPEDESCRIPTION; 8] =
    [{
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_VECTOR,
                             fieldName:
                                 b"position\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 0 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_CHARACTER,
                             fieldName:
                                 b"name\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 12 as libc::c_ulong as libc::c_int,
                             fieldSize: 64 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_SHORT,
                             fieldName:
                                 b"entityIndex\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 76 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_CHARACTER,
                             fieldName:
                                 b"depth\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 78 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_CHARACTER,
                             fieldName:
                                 b"flags\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 79 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_FLOAT,
                             fieldName:
                                 b"scale\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 80 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_VECTOR,
                             fieldName:
                                 b"impactPlaneNormal\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 84 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_CHARACTER,
                             fieldName:
                                 b"studio_state\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 96 as libc::c_ulong as libc::c_int,
                             fieldSize:
                                 ::std::mem::size_of::<modelstate_t>() as
                                     libc::c_ulong as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     }];
static mut gStaticEntry: [TYPEDESCRIPTION; 34] =
    [{
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_MODELNAME,
                             fieldName:
                                 b"messagenum\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 12 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_VECTOR,
                             fieldName:
                                 b"origin\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 16 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_VECTOR,
                             fieldName:
                                 b"angles\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 28 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_INTEGER,
                             fieldName:
                                 b"sequence\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 44 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_FLOAT,
                             fieldName:
                                 b"frame\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 48 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_INTEGER,
                             fieldName:
                                 b"colormap\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 52 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_SHORT,
                             fieldName:
                                 b"skin\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 56 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_INTEGER,
                             fieldName:
                                 b"body\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 100 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_FLOAT,
                             fieldName:
                                 b"scale\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 64 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_INTEGER,
                             fieldName:
                                 b"effects\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 60 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_FLOAT,
                             fieldName:
                                 b"framerate\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 96 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_VECTOR,
                             fieldName:
                                 b"mins\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 124 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_VECTOR,
                             fieldName:
                                 b"maxs\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 136 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_INTEGER,
                             fieldName:
                                 b"rendermode\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 72 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_FLOAT,
                             fieldName:
                                 b"renderamt\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 76 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_CHARACTER,
                             fieldName:
                                 b"rendercolor\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 80 as libc::c_ulong as libc::c_int,
                             fieldSize:
                                 ::std::mem::size_of::<color24>() as
                                     libc::c_ulong as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_INTEGER,
                             fieldName:
                                 b"renderfx\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 84 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_INTEGER,
                             fieldName:
                                 b"controller\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 104 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_INTEGER,
                             fieldName:
                                 b"blending\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 108 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_SHORT,
                             fieldName:
                                 b"solid\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 58 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_TIME,
                             fieldName:
                                 b"animtime\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 92 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_INTEGER,
                             fieldName:
                                 b"movetype\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 88 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_VECTOR,
                             fieldName:
                                 b"vuser1\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 292 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_VECTOR,
                             fieldName:
                                 b"vuser2\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 304 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_VECTOR,
                             fieldName:
                                 b"vuser3\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 316 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_VECTOR,
                             fieldName:
                                 b"vuser4\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 328 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_INTEGER,
                             fieldName:
                                 b"iuser1\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 260 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_INTEGER,
                             fieldName:
                                 b"iuser2\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 264 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_INTEGER,
                             fieldName:
                                 b"iuser3\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 268 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_INTEGER,
                             fieldName:
                                 b"iuser4\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 272 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_FLOAT,
                             fieldName:
                                 b"fuser1\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 276 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_FLOAT,
                             fieldName:
                                 b"fuser2\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 280 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_FLOAT,
                             fieldName:
                                 b"fuser3\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 284 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_FLOAT,
                             fieldName:
                                 b"fuser4\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 288 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     }];
static mut gSoundEntry: [TYPEDESCRIPTION; 11] =
    [{
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_CHARACTER,
                             fieldName:
                                 b"name\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 0 as libc::c_ulong as libc::c_int,
                             fieldSize: 64 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_SHORT,
                             fieldName:
                                 b"entnum\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 64 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_VECTOR,
                             fieldName:
                                 b"origin\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 68 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_FLOAT,
                             fieldName:
                                 b"volume\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 80 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_FLOAT,
                             fieldName:
                                 b"attenuation\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 84 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_BOOLEAN,
                             fieldName:
                                 b"looping\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 88 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_CHARACTER,
                             fieldName:
                                 b"channel\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 92 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_CHARACTER,
                             fieldName:
                                 b"pitch\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 93 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_CHARACTER,
                             fieldName:
                                 b"wordIndex\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 94 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_CHARACTER,
                             fieldName:
                                 b"samplePos\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 96 as libc::c_ulong as libc::c_int,
                             fieldSize:
                                 ::std::mem::size_of::<libc::c_double>() as
                                     libc::c_ulong as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_CHARACTER,
                             fieldName:
                                 b"forcedEnd\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 104 as libc::c_ulong as libc::c_int,
                             fieldSize:
                                 ::std::mem::size_of::<libc::c_double>() as
                                     libc::c_ulong as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     }];
static mut gTempEntvars: [TYPEDESCRIPTION; 2] =
    [{
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_STRING,
                             fieldName:
                                 b"classname\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 0 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0 as libc::c_int as libc::c_short,};
         init
     },
     {
         let mut init =
             TYPEDESCRIPTION{fieldType: FIELD_STRING,
                             fieldName:
                                 b"globalname\x00" as *const u8 as
                                     *const libc::c_char,
                             fieldOffset: 4 as libc::c_ulong as libc::c_int,
                             fieldSize: 1 as libc::c_int as libc::c_short,
                             flags: 0x1 as libc::c_int as libc::c_short,};
         init
     }];
#[no_mangle]
pub static mut gTitleComments: [C2RustUnnamed_0; 66] =
    [{
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"T0A0\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#T0A0TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"C0A0\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#C0A0TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"C1A0\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#C0A1TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"C1A1\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#C1A1TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"C1A2\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#C1A2TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"C1A3\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#C1A3TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"C1A4\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#C1A4TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"C2A1\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#C2A1TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"C2A2\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#C2A2TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"C2A3\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#C2A3TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"C2A4D\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#C2A4TITLE2\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"C2A4E\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#C2A4TITLE2\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"C2A4F\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#C2A4TITLE2\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"C2A4G\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#C2A4TITLE2\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"C2A4\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#C2A4TITLE1\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"C2A5\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#C2A5TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"C3A1\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#C3A1TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"C3A2\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#C3A2TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"C4A1A\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#C4A1ATITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"C4A1B\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#C4A1ATITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"C4A1C\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#C4A1ATITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"C4A1D\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#C4A1ATITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"C4A1E\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#C4A1ATITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"C4A1\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#C4A1TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"C4A2\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#C4A2TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"C4A3\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#C4A3TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"C5A1\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#C5TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"OFBOOT\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#OF_BOOT0TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"OF0A\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#OF1A1TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"OF1A1\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#OF1A3TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"OF1A2\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#OF1A3TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"OF1A3\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#OF1A3TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"OF1A4\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#OF1A3TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"OF1A\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#OF1A5TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"OF2A1\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#OF2A1TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"OF2A2\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#OF2A1TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"OF2A3\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#OF2A1TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"OF2A\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#OF2A4TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"OF3A1\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#OF3A1TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"OF3A2\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#OF3A1TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"OF3A\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#OF3A3TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"OF4A1\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#OF4A1TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"OF4A2\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#OF4A1TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"OF4A3\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#OF4A1TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"OF4A\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#OF4A4TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"OF5A\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#OF5A1TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"OF6A1\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#OF6A1TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"OF6A2\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#OF6A1TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"OF6A3\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#OF6A1TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"OF6A4b\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#OF6A4TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"OF6A4\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#OF6A4TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"OF6A5\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#OF6A4TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"OF6A\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#OF6A4TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"OF7A\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#OF7A0TITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"ba_tram\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#BA_TRAMTITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"ba_security\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#BA_SECURITYTITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"ba_main\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#BA_SECURITYTITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"ba_elevator\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#BA_SECURITYTITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"ba_canal\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#BA_CANALSTITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"ba_yard\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#BA_YARDTITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"ba_xen\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#BA_XENTITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"ba_hazard\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#BA_HAZARD\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"ba_power\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#BA_POWERTITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"ba_teleport1\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#BA_POWERTITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"ba_teleport\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#BA_TELEPORTTITLE\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_0{mapname:
                                 b"ba_outro\x00" as *const u8 as
                                     *const libc::c_char,
                             titlename:
                                 b"#BA_OUTRO\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     }];
/*
=============
SaveBuildComment

build commentary for each savegame
typically it writes world message and level time
=============
*/
unsafe extern "C" fn SaveBuildComment(mut text: *mut libc::c_char,
                                      mut maxlength: libc::c_int) {
    let mut comment: string = [0; 256]; // clear
    let mut pName: *const libc::c_char = 0 as *const libc::c_char;
    *text.offset(0 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
    if pfnSaveGameComment.is_some() {
        // get save comment from gamedll
        pfnSaveGameComment.expect("non-null function pointer")(comment.as_mut_ptr(),
                                                               256 as
                                                                   libc::c_int);
        pName = comment.as_mut_ptr()
    } else {
        let mut i: size_t = 0;
        let mut mapname: *const libc::c_char =
            SV_GetString((*svgame.globals).mapname);
        i = 0 as libc::c_int as size_t;
        while i <
                  (::std::mem::size_of::<[C2RustUnnamed_0; 66]>() as
                       libc::c_ulong).wrapping_div(::std::mem::size_of::<C2RustUnnamed_0>()
                                                       as libc::c_ulong) {
            // compare if strings are equal at beginning
            let mut len: size_t = strlen(gTitleComments[i as usize].mapname);
            if Q_strnicmp(mapname, gTitleComments[i as usize].mapname,
                          len as libc::c_int) == 0 {
                pName = gTitleComments[i as usize].titlename;
                break ;
            } else { i = i.wrapping_add(1) }
        }
        if pName.is_null() {
            if (*svgame.edicts).v.message != 0 as libc::c_int {
                // trying to extract message from the world
                pName = SV_GetString((*svgame.edicts).v.message)
            } else {
                // or use mapname
                pName = SV_GetString((*svgame.globals).mapname)
            }
        }
    }
    Q_snprintf(text, maxlength as size_t,
               b"%-64.64s %02d:%02d\x00" as *const u8 as *const libc::c_char,
               pName, (sv.time / 60.0f64) as libc::c_int,
               __tg_fmod(sv.time, 60.0f64) as libc::c_int);
}
/*
=============
DirectoryCount

counting all the files with HL1-HL3 extension
in save folder
=============
*/
unsafe extern "C" fn DirectoryCount(mut pPath: *const libc::c_char)
 -> libc::c_int {
    let mut count: libc::c_int = 0; // lookup only in gamedir
    let mut t: *mut search_t = 0 as *mut search_t; // empty
    t = FS_Search(pPath, true_0 as libc::c_int, true_0 as libc::c_int);
    if t.is_null() { return 0 as libc::c_int }
    count = (*t).numfilenames;
    _Mem_Free(t as *mut libc::c_void,
              b"../engine/server/sv_save.c\x00" as *const u8 as
                  *const libc::c_char, 370 as libc::c_int);
    return count;
}
/*
=============
InitEntityTable

reserve space for ETABLE's
=============
*/
unsafe extern "C" fn InitEntityTable(mut pSaveData: *mut SAVERESTOREDATA,
                                     mut entityCount: libc::c_int) {
    let mut pTable: *mut ENTITYTABLE = 0 as *mut ENTITYTABLE;
    let mut i: libc::c_int = 0;
    (*pSaveData).pTable =
        _Mem_Alloc(host.mempool,
                   (::std::mem::size_of::<ENTITYTABLE>() as
                        libc::c_ulong).wrapping_mul(entityCount as
                                                        libc::c_ulong),
                   true_0,
                   b"../engine/server/sv_save.c\x00" as *const u8 as
                       *const libc::c_char, 387 as libc::c_int) as
            *mut ENTITYTABLE;
    (*pSaveData).tableCount = entityCount;
    // setup entitytable
    i = 0 as libc::c_int;
    while i < entityCount {
        pTable =
            &mut *(*pSaveData).pTable.offset(i as isize) as *mut ENTITYTABLE;
        (*pTable).pent = SV_EdictNum(i);
        (*pTable).id = i;
        i += 1
    };
}
/*
=============
EntryInTable

check level in transition list
=============
*/
unsafe extern "C" fn EntryInTable(mut pSaveData: *mut SAVERESTOREDATA,
                                  mut pMapName: *const libc::c_char,
                                  mut index: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = index + 1 as libc::c_int;
    while i < (*pSaveData).connectionCount {
        if Q_strnicmp((*pSaveData).levelList[i as usize].mapName.as_mut_ptr(),
                      pMapName, 99999 as libc::c_int) == 0 {
            return i
        }
        i += 1
    }
    return -(1 as libc::c_int);
}
/*
=============
EdictFromTable

get edict from table
=============
*/
unsafe extern "C" fn EdictFromTable(mut pSaveData: *mut SAVERESTOREDATA,
                                    mut entityIndex: libc::c_int)
 -> *mut edict_t {
    if !pSaveData.is_null() && !(*pSaveData).pTable.is_null() {
        entityIndex =
            if entityIndex >= 0 as libc::c_int {
                if entityIndex < (*pSaveData).tableCount - 1 as libc::c_int {
                    entityIndex
                } else { ((*pSaveData).tableCount) - 1 as libc::c_int }
            } else { 0 as libc::c_int };
        return (*(*pSaveData).pTable.offset(entityIndex as isize)).pent
    }
    return 0 as *mut edict_t;
}
/*
=============
LandmarkOrigin

find global offset for a given landmark
=============
*/
unsafe extern "C" fn LandmarkOrigin(mut pSaveData: *mut SAVERESTOREDATA,
                                    mut output: *mut vec_t,
                                    mut pLandmarkName: *const libc::c_char) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*pSaveData).connectionCount {
        if Q_strncmp((*pSaveData).levelList[i as
                                                usize].landmarkName.as_mut_ptr(),
                     pLandmarkName, 99999 as libc::c_int) == 0 {
            *output.offset(0 as libc::c_int as isize) =
                (*pSaveData).levelList[i as
                                           usize].vecLandmarkOrigin[0 as
                                                                        libc::c_int
                                                                        as
                                                                        usize];
            *output.offset(1 as libc::c_int as isize) =
                (*pSaveData).levelList[i as
                                           usize].vecLandmarkOrigin[1 as
                                                                        libc::c_int
                                                                        as
                                                                        usize];
            *output.offset(2 as libc::c_int as isize) =
                (*pSaveData).levelList[i as
                                           usize].vecLandmarkOrigin[2 as
                                                                        libc::c_int
                                                                        as
                                                                        usize];
            return
        }
        i += 1
    }
    let ref mut fresh0 = *output.offset(2 as libc::c_int as isize);
    *fresh0 = 0 as libc::c_int as vec_t;
    let ref mut fresh1 = *output.offset(1 as libc::c_int as isize);
    *fresh1 = *fresh0;
    *output.offset(0 as libc::c_int as isize) = *fresh1;
}
/*
=============
EntityInSolid

some moved edicts on a next level cause stuck
outside of world. Find them and remove
=============
*/
unsafe extern "C" fn EntityInSolid(mut pent: *mut edict_t) -> libc::c_int {
    let mut aiment: *mut edict_t = (*pent).v.aiment;
    let mut point: vec3_t = [0.; 3];
    // if you're attached to a client, always go through
    if (*pent).v.movetype == 12 as libc::c_int &&
           SV_CheckEdict(aiment,
                         b"../engine/server/sv_save.c\x00" as *const u8 as
                             *const libc::c_char, 474 as libc::c_int) as
               libc::c_uint != 0 &&
           (*aiment).v.flags as libc::c_uint &
               (1 as libc::c_uint) << 3 as libc::c_int != 0 {
        return 0 as libc::c_int
    }
    point[0 as libc::c_int as usize] =
        ((*pent).v.absmin[0 as libc::c_int as usize] +
             (*pent).v.absmax[0 as libc::c_int as usize]) * 0.5f32;
    point[1 as libc::c_int as usize] =
        ((*pent).v.absmin[1 as libc::c_int as usize] +
             (*pent).v.absmax[1 as libc::c_int as usize]) * 0.5f32;
    point[2 as libc::c_int as usize] =
        ((*pent).v.absmin[2 as libc::c_int as usize] +
             (*pent).v.absmax[2 as libc::c_int as usize]) * 0.5f32;
    svs.groupmask = (*pent).v.groupinfo;
    return (SV_PointContents(point.as_mut_ptr() as *const vec_t) ==
                -(2 as libc::c_int)) as libc::c_int;
}
/*
=============
ClearSaveDir

remove all the temp files HL1-HL3
(it will be extracted again from another .sav file)
=============
*/
unsafe extern "C" fn ClearSaveDir() {
    let mut t: *mut search_t = 0 as *mut search_t;
    let mut i: libc::c_int = 0;
    // just delete all HL? files
    t =
        FS_Search(b"save/*.HL?\x00" as *const u8 as *const libc::c_char,
                  true_0 as libc::c_int,
                  true_0 as libc::c_int); // already empty
    if t.is_null() { return }
    i = 0 as libc::c_int;
    while i < (*t).numfilenames {
        FS_Delete(*(*t).filenames.offset(i as isize));
        i += 1
    }
    _Mem_Free(t as *mut libc::c_void,
              b"../engine/server/sv_save.c\x00" as *const u8 as
                  *const libc::c_char, 503 as libc::c_int);
}
/*
=============
IsValidSave

savegame is allowed?
=============
*/
unsafe extern "C" fn IsValidSave() -> libc::c_int {
    if svs.initialized as u64 == 0 ||
           sv.state as libc::c_uint !=
               ss_active as libc::c_int as libc::c_uint {
        Con_Printf(b"Not playing a local game.\n\x00" as *const u8 as
                       *const libc::c_char);
        return 0 as libc::c_int
    }
    // ignore autosave during background
    if sv.background as libc::c_uint != 0 ||
           UI_CreditsActive() as libc::c_uint != 0 {
        return 0 as libc::c_int
    }
    if svgame.physFuncs.SV_AllowSaveGame.is_some() {
        if svgame.physFuncs.SV_AllowSaveGame.expect("non-null function pointer")()
               == 0 {
            Con_Printf(b"Savegame is not allowed.\n\x00" as *const u8 as
                           *const libc::c_char);
            return 0 as libc::c_int
        }
    }
    if CL_Active() == 0 {
        Con_Printf(b"Can\'t save if not active.\n\x00" as *const u8 as
                       *const libc::c_char);
        return 0 as libc::c_int
    }
    if CL_IsIntermission() as u64 != 0 {
        Con_Printf(b"Can\'t save during intermission.\n\x00" as *const u8 as
                       *const libc::c_char);
        return 0 as libc::c_int
    }
    if svs.maxclients != 1 as libc::c_int {
        Con_Printf(b"Can\'t save multiplayer games.\n\x00" as *const u8 as
                       *const libc::c_char);
        return 0 as libc::c_int
    }
    if !svs.clients.is_null() &&
           (*svs.clients.offset(0 as libc::c_int as isize)).state as
               libc::c_uint == cs_spawned as libc::c_int as libc::c_uint {
        let mut pl: *mut edict_t =
            (*svs.clients.offset(0 as libc::c_int as isize)).edict;
        if pl.is_null() {
            Con_Printf(b"Can\'t savegame without a player!\n\x00" as *const u8
                           as *const libc::c_char);
            return 0 as libc::c_int
        }
        if (*pl).v.deadflag != 0 || (*pl).v.health <= 0.0f32 {
            Con_Printf(b"Can\'t savegame with a dead player\n\x00" as
                           *const u8 as *const libc::c_char);
            return 0 as libc::c_int
        }
        // Passed all checks, it's ok to save
        return 1 as libc::c_int
    }
    Con_Printf(b"Can\'t savegame without a client!\n\x00" as *const u8 as
                   *const libc::c_char);
    return 0 as libc::c_int;
}
/*
=============
AgeSaveList

scroll the name list down
=============
*/
unsafe extern "C" fn AgeSaveList(mut pName: *const libc::c_char,
                                 mut count: libc::c_int) {
    let mut newName: [libc::c_char; 260] = [0; 260];
    let mut oldName: [libc::c_char; 260] = [0; 260];
    let mut newShot: [libc::c_char; 260] = [0; 260];
    let mut oldShot: [libc::c_char; 260] = [0; 260];
    // delete last quick/autosave (e.g. quick05.sav)
    Q_snprintf(newName.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 260]>() as libc::c_ulong,
               b"save/%s%02d.sav\x00" as *const u8 as *const libc::c_char,
               pName, count);
    Q_snprintf(newShot.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 260]>() as libc::c_ulong,
               b"save/%s%02d.bmp\x00" as *const u8 as *const libc::c_char,
               pName, count);
    // only delete from game directory, basedir is read-only
    FS_Delete(newName.as_mut_ptr());
    FS_Delete(newShot.as_mut_ptr());
    // unloading the shot footprint
    GL_FreeImage(newShot.as_mut_ptr());
    // XASH_DEDICATED
    while count > 0 as libc::c_int {
        if count == 1 as libc::c_int {
            // quick.sav
            Q_snprintf(oldName.as_mut_ptr(),
                       ::std::mem::size_of::<[libc::c_char; 260]>() as
                           libc::c_ulong,
                       b"save/%s.sav\x00" as *const u8 as *const libc::c_char,
                       pName);
            Q_snprintf(oldShot.as_mut_ptr(),
                       ::std::mem::size_of::<[libc::c_char; 260]>() as
                           libc::c_ulong,
                       b"save/%s.bmp\x00" as *const u8 as *const libc::c_char,
                       pName);
        } else {
            // quick04.sav, etc.
            Q_snprintf(oldName.as_mut_ptr(),
                       ::std::mem::size_of::<[libc::c_char; 260]>() as
                           libc::c_ulong,
                       b"save/%s%02d.sav\x00" as *const u8 as
                           *const libc::c_char, pName,
                       count - 1 as libc::c_int);
            Q_snprintf(oldShot.as_mut_ptr(),
                       ::std::mem::size_of::<[libc::c_char; 260]>() as
                           libc::c_ulong,
                       b"save/%s%02d.bmp\x00" as *const u8 as
                           *const libc::c_char, pName,
                       count - 1 as libc::c_int);
        }
        Q_snprintf(newName.as_mut_ptr(),
                   ::std::mem::size_of::<[libc::c_char; 260]>() as
                       libc::c_ulong,
                   b"save/%s%02d.sav\x00" as *const u8 as *const libc::c_char,
                   pName, count);
        Q_snprintf(newShot.as_mut_ptr(),
                   ::std::mem::size_of::<[libc::c_char; 260]>() as
                       libc::c_ulong,
                   b"save/%s%02d.bmp\x00" as *const u8 as *const libc::c_char,
                   pName, count);
        // unloading the oldshot footprint too
        GL_FreeImage(oldShot.as_mut_ptr());
        // XASH_DEDICATED
        // scroll the name list down (e.g. rename quick04.sav to quick05.sav)
        FS_Rename(oldName.as_mut_ptr(), newName.as_mut_ptr());
        FS_Rename(oldShot.as_mut_ptr(), newShot.as_mut_ptr());
        count -= 1
    };
}
/*
=============
DirectoryCopy

put the HL1-HL3 files into .sav file
=============
*/
unsafe extern "C" fn DirectoryCopy(mut pPath: *const libc::c_char,
                                   mut pFile: *mut file_t) {
    let mut szName: [libc::c_char; 260] = [0; 260]; // nothing to copy ?
    let mut i: libc::c_int =
        0; // clearing the string to prevent garbage in output file
    let mut fileSize: libc::c_int = 0;
    let mut pCopy: *mut file_t = 0 as *mut file_t;
    let mut t: *mut search_t = 0 as *mut search_t;
    t = FS_Search(pPath, true_0 as libc::c_int, true_0 as libc::c_int);
    if t.is_null() { return }
    i = 0 as libc::c_int;
    while i < (*t).numfilenames {
        pCopy =
            FS_Open(*(*t).filenames.offset(i as isize),
                    b"rb\x00" as *const u8 as *const libc::c_char, true_0);
        fileSize = FS_FileLength(pCopy) as libc::c_int;
        memset(szName.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<[libc::c_char; 260]>() as libc::c_ulong);
        Q_strncpy(szName.as_mut_ptr(),
                  COM_FileWithoutPath(*(*t).filenames.offset(i as isize)),
                  260 as libc::c_int as size_t);
        FS_Write(pFile, szName.as_mut_ptr() as *const libc::c_void,
                 260 as libc::c_int as size_t);
        FS_Write(pFile,
                 &mut fileSize as *mut libc::c_int as *const libc::c_void,
                 ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
        FS_FileCopy(pFile, pCopy, fileSize);
        FS_Close(pCopy);
        i += 1
    }
    _Mem_Free(t as *mut libc::c_void,
              b"../engine/server/sv_save.c\x00" as *const u8 as
                  *const libc::c_char, 661 as libc::c_int);
}
/*
=============
DirectoryExtract

extract the HL1-HL3 files from the .sav file
=============
*/
unsafe extern "C" fn DirectoryExtract(mut pFile: *mut file_t,
                                      mut fileCount: libc::c_int) {
    let mut szName: [libc::c_char; 260] = [0; 260];
    let mut fileName: [libc::c_char; 260] = [0; 260];
    let mut i: libc::c_int = 0;
    let mut fileSize: libc::c_int = 0;
    let mut pCopy: *mut file_t = 0 as *mut file_t;
    i = 0 as libc::c_int;
    while i < fileCount {
        // filename can only be as long as a map name + extension
        FS_Read(pFile, szName.as_mut_ptr() as *mut libc::c_void,
                260 as libc::c_int as size_t);
        FS_Read(pFile, &mut fileSize as *mut libc::c_int as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
        Q_snprintf(fileName.as_mut_ptr(),
                   ::std::mem::size_of::<[libc::c_char; 260]>() as
                       libc::c_ulong,
                   b"save/%s\x00" as *const u8 as *const libc::c_char,
                   szName.as_mut_ptr());
        COM_FixSlashes(fileName.as_mut_ptr());
        pCopy =
            FS_Open(fileName.as_mut_ptr(),
                    b"wb\x00" as *const u8 as *const libc::c_char, true_0);
        FS_FileCopy(pCopy, pFile, fileSize);
        FS_Close(pCopy);
        i += 1
    };
}
/*
=============
SaveInit

initialize global save-restore buffer
=============
*/
unsafe extern "C" fn SaveInit(mut size: libc::c_int,
                              mut tokenCount: libc::c_int)
 -> *mut SAVERESTOREDATA {
    let mut pSaveData: *mut SAVERESTOREDATA =
        0 as *mut SAVERESTOREDATA; // skip the save structure);
    pSaveData =
        _Mem_Alloc(host.mempool,
                   (::std::mem::size_of::<SAVERESTOREDATA>() as
                        libc::c_ulong).wrapping_add(size as libc::c_ulong),
                   true_0,
                   b"../engine/server/sv_save.c\x00" as *const u8 as
                       *const libc::c_char, 703 as libc::c_int) as
            *mut SAVERESTOREDATA; // reset the pointer
    (*pSaveData).pTokens =
        _Mem_Alloc(host.mempool,
                   (tokenCount as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                        as libc::c_ulong),
                   true_0,
                   b"../engine/server/sv_save.c\x00" as *const u8 as
                       *const libc::c_char, 704 as libc::c_int) as
            *mut *mut libc::c_char; // Use DLL time
    (*pSaveData).tokenCount = tokenCount;
    (*pSaveData).pBaseData =
        pSaveData.offset(1 as libc::c_int as isize) as *mut libc::c_char;
    (*pSaveData).pCurrentData = (*pSaveData).pBaseData;
    (*pSaveData).bufferSize = size;
    (*pSaveData).time = (*svgame.globals).time;
    // shared with dlls
    (*svgame.globals).pSaveData = pSaveData as *mut libc::c_void;
    return pSaveData;
}
/*
=============
SaveClear

clearing buffer for reuse
=============
*/
unsafe extern "C" fn SaveClear(mut pSaveData: *mut SAVERESTOREDATA) {
    memset((*pSaveData).pTokens as *mut libc::c_void, 0 as libc::c_int,
           ((*pSaveData).tokenCount as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                as
                                                libc::c_ulong)); // skip the save structure);
    (*pSaveData).pBaseData =
        pSaveData.offset(1 as libc::c_int as isize) as
            *mut libc::c_char; // reset the pointer
    (*pSaveData).pCurrentData = (*pSaveData).pBaseData; // Use DLL time
    (*pSaveData).time = (*svgame.globals).time; // reset the hashtable
    (*pSaveData).tokenSize = 0 as libc::c_int; // reset the pointer
    (*pSaveData).size = 0 as libc::c_int;
    // shared with dlls
    (*svgame.globals).pSaveData = pSaveData as *mut libc::c_void;
}
/*
=============
SaveFinish

release global save-restore buffer
=============
*/
unsafe extern "C" fn SaveFinish(mut pSaveData: *mut SAVERESTOREDATA) {
    if pSaveData.is_null() { return }
    if !(*pSaveData).pTokens.is_null() {
        _Mem_Free((*pSaveData).pTokens as *mut libc::c_void,
                  b"../engine/server/sv_save.c\x00" as *const u8 as
                      *const libc::c_char, 753 as libc::c_int);
        (*pSaveData).pTokens = 0 as *mut *mut libc::c_char;
        (*pSaveData).tokenCount = 0 as libc::c_int
    }
    if !(*pSaveData).pTable.is_null() {
        _Mem_Free((*pSaveData).pTable as *mut libc::c_void,
                  b"../engine/server/sv_save.c\x00" as *const u8 as
                      *const libc::c_char, 760 as libc::c_int);
        (*pSaveData).pTable = 0 as *mut ENTITYTABLE;
        (*pSaveData).tableCount = 0 as libc::c_int
    }
    (*svgame.globals).pSaveData = 0 as *mut libc::c_void;
    _Mem_Free(pSaveData as *mut libc::c_void,
              b"../engine/server/sv_save.c\x00" as *const u8 as
                  *const libc::c_char, 766 as libc::c_int);
}
/*
=============
StoreHashTable

write the stringtable into file
=============
*/
unsafe extern "C" fn StoreHashTable(mut pSaveData: *mut SAVERESTOREDATA)
 -> *mut libc::c_char {
    let mut pTokenData: *mut libc::c_char = (*pSaveData).pCurrentData;
    let mut i: libc::c_int = 0;
    // Write entity string token table
    if !(*pSaveData).pTokens.is_null() {
        i = 0 as libc::c_int;
        while i < (*pSaveData).tokenCount {
            let mut pszToken: *const libc::c_char =
                if !(*(*pSaveData).pTokens.offset(i as isize)).is_null() {
                    *(*pSaveData).pTokens.offset(i as isize) as
                        *const libc::c_char
                } else { b"\x00" as *const u8 as *const libc::c_char };
            // Write the term
            while *pszToken != 0
                  // just copy the token byte-by-byte
                  {
                let fresh2 = pszToken;
                pszToken = pszToken.offset(1);
                let fresh3 = (*pSaveData).pCurrentData;
                (*pSaveData).pCurrentData =
                    (*pSaveData).pCurrentData.offset(1);
                *fresh3 = *fresh2
            }
            let fresh4 = (*pSaveData).pCurrentData;
            (*pSaveData).pCurrentData = (*pSaveData).pCurrentData.offset(1);
            *fresh4 = 0 as libc::c_int as libc::c_char;
            i += 1
        }
    }
    (*pSaveData).tokenSize =
        (*pSaveData).pCurrentData.wrapping_offset_from(pTokenData) as
            libc::c_long as libc::c_int;
    return pTokenData;
}
/*
=============
BuildHashTable

build the stringtable from buffer
=============
*/
unsafe extern "C" fn BuildHashTable(mut pSaveData: *mut SAVERESTOREDATA,
                                    mut pFile: *mut file_t) {
    let mut pszTokenList: *mut libc::c_char = (*pSaveData).pBaseData;
    let mut i: libc::c_int = 0;
    // Parse the symbol table
    if (*pSaveData).tokenSize > 0 as libc::c_int {
        FS_Read(pFile, pszTokenList as *mut libc::c_void,
                (*pSaveData).tokenSize as size_t);
        // make sure the token strings pointed to by the pToken hashtable.
        i = 0 as libc::c_int;
        while i < (*pSaveData).tokenCount {
            let ref mut fresh5 = *(*pSaveData).pTokens.offset(i as isize);
            *fresh5 =
                if *pszTokenList as libc::c_int != 0 {
                    pszTokenList
                } else { 0 as *mut libc::c_char };
            loop  {
                let fresh6 = pszTokenList;
                pszTokenList = pszTokenList.offset(1);
                if !(*fresh6 != 0) { break ; }
            }
            i += 1
            // Find next token (after next null)
        }
    }
    // rebase the data pointer
    (*pSaveData).pBaseData =
        pszTokenList; // pszTokenList now points after token data
    (*pSaveData).pCurrentData = (*pSaveData).pBaseData;
}
/*
=============
GetClientDataSize

g-cont: this routine is redundant
i'm write it just for more readable code
=============
*/
unsafe extern "C" fn GetClientDataSize(mut level: *const libc::c_char)
 -> libc::c_int {
    let mut tokenCount: libc::c_int = 0;
    let mut tokenSize: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut id: libc::c_int = 0;
    let mut version: libc::c_int = 0;
    let mut name: [libc::c_char; 64] = [0; 64];
    let mut pFile: *mut file_t = 0 as *mut file_t;
    Q_snprintf(name.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
               b"save/%s.HL2\x00" as *const u8 as *const libc::c_char, level);
    pFile =
        FS_Open(name.as_mut_ptr(),
                b"rb\x00" as *const u8 as *const libc::c_char, true_0);
    if pFile.is_null() { return 0 as libc::c_int }
    FS_Read(pFile, &mut id as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    if id !=
           (('V' as i32) << 24 as libc::c_int) +
               (('A' as i32) << 16 as libc::c_int) +
               (('S' as i32) << 8 as libc::c_int) + 'J' as i32 {
        FS_Close(pFile);
        return 0 as libc::c_int
    }
    FS_Read(pFile, &mut version as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    if version != 0x67 as libc::c_int {
        FS_Close(pFile);
        return 0 as libc::c_int
    }
    FS_Read(pFile, &mut size as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    FS_Read(pFile, &mut tokenCount as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    FS_Read(pFile, &mut tokenSize as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    FS_Close(pFile);
    return size + tokenSize;
}
/*
=============
LoadSaveData

fill the save resore buffer
parse hash strings
=============
*/
unsafe extern "C" fn LoadSaveData(mut level: *const libc::c_char)
 -> *mut SAVERESTOREDATA {
    let mut tokenSize: libc::c_int = 0;
    let mut tableCount: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut tokenCount: libc::c_int = 0;
    let mut name: [libc::c_char; 260] = [0; 260];
    let mut id: libc::c_int = 0;
    let mut version: libc::c_int = 0;
    let mut clientSize: libc::c_int = 0;
    let mut pSaveData: *mut SAVERESTOREDATA = 0 as *mut SAVERESTOREDATA;
    let mut totalSize: libc::c_int = 0;
    let mut pFile: *mut file_t = 0 as *mut file_t;
    Q_snprintf(name.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 260]>() as libc::c_ulong,
               b"save/%s.HL1\x00" as *const u8 as *const libc::c_char, level);
    Con_Printf(b"Loading game from %s...\n\x00" as *const u8 as
                   *const libc::c_char, name.as_mut_ptr());
    pFile =
        FS_Open(name.as_mut_ptr(),
                b"rb\x00" as *const u8 as *const libc::c_char, true_0);
    if pFile.is_null() {
        Con_Printf(b"^1Error:^7 Couldn\'t open save data file %s.\n\x00" as
                       *const u8 as *const libc::c_char, name.as_mut_ptr());
        return 0 as *mut SAVERESTOREDATA
    }
    // Read the header
    FS_Read(pFile, &mut id as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    FS_Read(pFile, &mut version as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    // is this a valid save?
    if id !=
           (('V' as i32) << 24 as libc::c_int) +
               (('L' as i32) << 16 as libc::c_int) +
               (('A' as i32) << 8 as libc::c_int) + 'V' as i32 ||
           version != 0x71 as libc::c_int {
        FS_Close(pFile);
        return 0 as *mut SAVERESTOREDATA
    }
    // Read the sections info and the data
    FS_Read(pFile, &mut size as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as
                libc::c_ulong); // total size of all data to initialize read buffer
    FS_Read(pFile, &mut tableCount as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as
                libc::c_ulong); // entities count to right initialize entity table
    FS_Read(pFile, &mut tokenCount as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as
                libc::c_ulong); // num hash tokens to prepare token table
    FS_Read(pFile, &mut tokenSize as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as
                libc::c_ulong); // total size of hash tokens
    // determine highest size of seve-restore buffer
	// because it's used twice: for HL1 and HL2 restore
    clientSize = GetClientDataSize(level);
    totalSize =
        if clientSize > size + tokenSize {
            clientSize
        } else { (size) + tokenSize };
    // init the read buffer
    pSaveData = SaveInit(totalSize, tokenCount); // count ETABLE entries
    Q_strncpy((*pSaveData).szCurrentMapName.as_mut_ptr(), level,
              ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong);
    (*pSaveData).tableCount = tableCount;
    (*pSaveData).tokenCount = tokenCount;
    (*pSaveData).tokenSize = tokenSize;
    // Parse the symbol table
    BuildHashTable(pSaveData, pFile);
    // Set up the restore basis
    (*pSaveData).fUseLandmark = true_0 as libc::c_int;
    (*pSaveData).time = 0.0f32;
    // now reading all the rest of data
    FS_Read(pFile, (*pSaveData).pBaseData as *mut libc::c_void,
            size as
                size_t); // data is sucessfully moved into SaveRestore buffer (ETABLE will be init later)
    FS_Close(pFile);
    return pSaveData;
}
/*
=============
ParseSaveTables

reading global data, setup ETABLE's
=============
*/
unsafe extern "C" fn ParseSaveTables(mut pSaveData: *mut SAVERESTOREDATA,
                                     mut pHeader: *mut SAVE_HEADER,
                                     mut updateGlobals: libc::c_int) {
    let mut light: SAVE_LIGHTSTYLE =
        SAVE_LIGHTSTYLE{index: 0, style: [0; 256], time: 0.,};
    let mut i: libc::c_int = 0;
    // Re-base the savedata since we re-ordered the entity/table / restore fields
    InitEntityTable(pSaveData, (*pSaveData).tableCount);
    i = 0 as libc::c_int;
    while i < (*pSaveData).tableCount {
        svgame.dllFuncs.pfnSaveReadFields.expect("non-null function pointer")(pSaveData,
                                                                              b"ETABLE\x00"
                                                                                  as
                                                                                  *const u8
                                                                                  as
                                                                                  *const libc::c_char,
                                                                              &mut *(*pSaveData).pTable.offset(i
                                                                                                                   as
                                                                                                                   isize)
                                                                                  as
                                                                                  *mut ENTITYTABLE
                                                                                  as
                                                                                  *mut libc::c_void,
                                                                              gEntityTable.as_mut_ptr(),
                                                                              (::std::mem::size_of::<[TYPEDESCRIPTION; 5]>()
                                                                                   as
                                                                                   libc::c_ulong).wrapping_div(::std::mem::size_of::<TYPEDESCRIPTION>()
                                                                                                                   as
                                                                                                                   libc::c_ulong)
                                                                                  as
                                                                                  libc::c_int);
        i += 1
    }
    (*pSaveData).pBaseData = (*pSaveData).pCurrentData;
    (*pSaveData).size = 0 as libc::c_int;
    // process SAVE_HEADER
    svgame.dllFuncs.pfnSaveReadFields.expect("non-null function pointer")(pSaveData,
                                                                          b"Save Header\x00"
                                                                              as
                                                                              *const u8
                                                                              as
                                                                              *const libc::c_char,
                                                                          pHeader
                                                                              as
                                                                              *mut libc::c_void,
                                                                          gSaveHeader.as_mut_ptr(),
                                                                          (::std::mem::size_of::<[TYPEDESCRIPTION; 13]>()
                                                                               as
                                                                               libc::c_ulong).wrapping_div(::std::mem::size_of::<TYPEDESCRIPTION>()
                                                                                                               as
                                                                                                               libc::c_ulong)
                                                                              as
                                                                              libc::c_int);
    (*pSaveData).connectionCount = (*pHeader).connectionCount;
    (*pSaveData).vecLandmarkOffset[2 as libc::c_int as usize] =
        0 as libc::c_int as vec_t;
    (*pSaveData).vecLandmarkOffset[1 as libc::c_int as usize] =
        (*pSaveData).vecLandmarkOffset[2 as libc::c_int as usize];
    (*pSaveData).vecLandmarkOffset[0 as libc::c_int as usize] =
        (*pSaveData).vecLandmarkOffset[1 as libc::c_int as usize];
    (*pSaveData).time = (*pHeader).time;
    (*pSaveData).fUseLandmark = true_0 as libc::c_int;
    // read adjacency list
    i = 0 as libc::c_int;
    while i < (*pSaveData).connectionCount {
        svgame.dllFuncs.pfnSaveReadFields.expect("non-null function pointer")(pSaveData,
                                                                              b"ADJACENCY\x00"
                                                                                  as
                                                                                  *const u8
                                                                                  as
                                                                                  *const libc::c_char,
                                                                              &mut *(*pSaveData).levelList.as_mut_ptr().offset(i
                                                                                                                                   as
                                                                                                                                   isize)
                                                                                  as
                                                                                  *mut LEVELLIST
                                                                                  as
                                                                                  *mut libc::c_void,
                                                                              gAdjacency.as_mut_ptr(),
                                                                              (::std::mem::size_of::<[TYPEDESCRIPTION; 4]>()
                                                                                   as
                                                                                   libc::c_ulong).wrapping_div(::std::mem::size_of::<TYPEDESCRIPTION>()
                                                                                                                   as
                                                                                                                   libc::c_ulong)
                                                                                  as
                                                                                  libc::c_int);
        i += 1
    }
    if updateGlobals != 0 {
        memset(sv.lightstyles.as_mut_ptr() as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<[lightstyle_t; 64]>() as libc::c_ulong);
    }
    i = 0 as libc::c_int;
    while i < (*pHeader).lightStyleCount {
        svgame.dllFuncs.pfnSaveReadFields.expect("non-null function pointer")(pSaveData,
                                                                              b"LIGHTSTYLE\x00"
                                                                                  as
                                                                                  *const u8
                                                                                  as
                                                                                  *const libc::c_char,
                                                                              &mut light
                                                                                  as
                                                                                  *mut SAVE_LIGHTSTYLE
                                                                                  as
                                                                                  *mut libc::c_void,
                                                                              gLightStyle.as_mut_ptr(),
                                                                              (::std::mem::size_of::<[TYPEDESCRIPTION; 3]>()
                                                                                   as
                                                                                   libc::c_ulong).wrapping_div(::std::mem::size_of::<TYPEDESCRIPTION>()
                                                                                                                   as
                                                                                                                   libc::c_ulong)
                                                                                  as
                                                                                  libc::c_int);
        if updateGlobals != 0 {
            SV_SetLightStyle(light.index, light.style.as_mut_ptr(),
                             light.time);
        }
        i += 1
    };
}
/*
=============
EntityPatchWrite

write out the list of entities that are no longer in the save file for this level
(they've been moved to another level)
=============
*/
unsafe extern "C" fn EntityPatchWrite(mut pSaveData: *mut SAVERESTOREDATA,
                                      mut level: *const libc::c_char) {
    let mut name: [libc::c_char; 64] = [0; 64];
    let mut i: libc::c_int = 0;
    let mut size: libc::c_int = 0 as libc::c_int;
    let mut pFile: *mut file_t = 0 as *mut file_t;
    Q_snprintf(name.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
               b"save/%s.HL3\x00" as *const u8 as *const libc::c_char, level);
    pFile =
        FS_Open(name.as_mut_ptr(),
                b"wb\x00" as *const u8 as *const libc::c_char, true_0);
    if pFile.is_null() { return }
    i = 0 as libc::c_int;
    while i < (*pSaveData).tableCount {
        if (*(*pSaveData).pTable.offset(i as isize)).flags &
               0x40000000 as libc::c_int != 0 {
            size += 1
        }
        i += 1
    }
    // patch count
    FS_Write(pFile, &mut size as *mut libc::c_int as *const libc::c_void,
             ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    i = 0 as libc::c_int;
    while i < (*pSaveData).tableCount {
        if (*(*pSaveData).pTable.offset(i as isize)).flags &
               0x40000000 as libc::c_int != 0 {
            FS_Write(pFile, &mut i as *mut libc::c_int as *const libc::c_void,
                     ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
        }
        i += 1
    }
    FS_Close(pFile);
}
/*
=============
EntityPatchRead

read the list of entities that are no longer in the save file for this level
(they've been moved to another level)
=============
*/
unsafe extern "C" fn EntityPatchRead(mut pSaveData: *mut SAVERESTOREDATA,
                                     mut level: *const libc::c_char) {
    let mut name: [libc::c_char; 64] = [0; 64];
    let mut i: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut entityId: libc::c_int = 0;
    let mut pFile: *mut file_t = 0 as *mut file_t;
    Q_snprintf(name.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
               b"save/%s.HL3\x00" as *const u8 as *const libc::c_char, level);
    pFile =
        FS_Open(name.as_mut_ptr(),
                b"rb\x00" as *const u8 as *const libc::c_char, true_0);
    if pFile.is_null() { return }
    // patch count
    FS_Read(pFile, &mut size as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    i = 0 as libc::c_int;
    while i < size {
        FS_Read(pFile, &mut entityId as *mut libc::c_int as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
        (*(*pSaveData).pTable.offset(entityId as isize)).flags =
            0x40000000 as libc::c_int;
        i += 1
    }
    FS_Close(pFile);
}
/*
=============
RestoreDecal

restore decal\move across transition
=============
*/
unsafe extern "C" fn RestoreDecal(mut pSaveData: *mut SAVERESTOREDATA,
                                  mut entry: *mut decallist_t,
                                  mut adjacent: qboolean) {
    let mut decalIndex: libc::c_int = 0;
    let mut entityIndex: libc::c_int = 0 as libc::c_int;
    let mut flags: libc::c_int = (*entry).flags as libc::c_int;
    let mut modelIndex: libc::c_int = 0 as libc::c_int;
    let mut pEdict: *mut edict_t = 0 as *mut edict_t;
    // never move permanent decals
    if adjacent as libc::c_uint != 0 && flags & 0x1 as libc::c_int != 0 {
        return
    }
    // restore entity and model index
    pEdict =
        EdictFromTable(pSaveData,
                       (*entry).entityIndex as
                           libc::c_int); // decal was sucessfully restored at the game-side
    if SV_RestoreCustomDecal(entry, pEdict, adjacent) as u64 != 0 { return }
    // studio decals are handled at game-side
    if flags & 0x40 as libc::c_int != 0 { return }
    if SV_CheckEdict(pEdict,
                     b"../engine/server/sv_save.c\x00" as *const u8 as
                         *const libc::c_char, 1110 as libc::c_int) as u64 != 0
       {
        modelIndex = (*pEdict).v.modelindex
    }
    if SV_CheckEdict(pEdict,
                     b"../engine/server/sv_save.c\x00" as *const u8 as
                         *const libc::c_char, 1113 as libc::c_int) as u64 != 0
       {
        entityIndex =
            pEdict.wrapping_offset_from(svgame.edicts) as libc::c_long as
                libc::c_int
    }
    decalIndex = pfnDecalIndex((*entry).name.as_mut_ptr());
    // this can happens if brush entity from previous level was turned into world geometry
    if adjacent as libc::c_uint != 0 &&
           (*entry).entityIndex as libc::c_int != 0 as libc::c_int &&
           SV_CheckEdict(pEdict,
                         b"../engine/server/sv_save.c\x00" as *const u8 as
                             *const libc::c_char, 1119 as libc::c_int) as u64
               == 0 {
        let mut testspot: vec3_t = [0.; 3];
        let mut testend: vec3_t = [0.; 3];
        let mut tr: trace_t =
            trace_t{allsolid: false_0,
                    startsolid: false_0,
                    inopen: false_0,
                    inwater: false_0,
                    fraction: 0.,
                    endpos: [0.; 3],
                    plane: plane_t{normal: [0.; 3], dist: 0.,},
                    ent: 0 as *mut edict_t,
                    hitgroup: 0,};
        Con_Printf(b"^1Error:^7 RestoreDecal: couldn\'t restore entity index %i\n\x00"
                       as *const u8 as *const libc::c_char,
                   (*entry).entityIndex as libc::c_int);
        testspot[0 as libc::c_int as usize] =
            (*entry).position[0 as libc::c_int as usize];
        testspot[1 as libc::c_int as usize] =
            (*entry).position[1 as libc::c_int as usize];
        testspot[2 as libc::c_int as usize] =
            (*entry).position[2 as libc::c_int as usize];
        testspot[0 as libc::c_int as usize] =
            testspot[0 as libc::c_int as usize] +
                5.0f32 *
                    (*entry).impactPlaneNormal[0 as libc::c_int as usize];
        testspot[1 as libc::c_int as usize] =
            testspot[1 as libc::c_int as usize] +
                5.0f32 *
                    (*entry).impactPlaneNormal[1 as libc::c_int as usize];
        testspot[2 as libc::c_int as usize] =
            testspot[2 as libc::c_int as usize] +
                5.0f32 *
                    (*entry).impactPlaneNormal[2 as libc::c_int as usize];
        testend[0 as libc::c_int as usize] =
            (*entry).position[0 as libc::c_int as usize];
        testend[1 as libc::c_int as usize] =
            (*entry).position[1 as libc::c_int as usize];
        testend[2 as libc::c_int as usize] =
            (*entry).position[2 as libc::c_int as usize];
        testend[0 as libc::c_int as usize] =
            testend[0 as libc::c_int as usize] +
                -5.0f32 *
                    (*entry).impactPlaneNormal[0 as libc::c_int as usize];
        testend[1 as libc::c_int as usize] =
            testend[1 as libc::c_int as usize] +
                -5.0f32 *
                    (*entry).impactPlaneNormal[1 as libc::c_int as usize];
        testend[2 as libc::c_int as usize] =
            testend[2 as libc::c_int as usize] +
                -5.0f32 *
                    (*entry).impactPlaneNormal[2 as libc::c_int as usize];
        tr =
            SV_Move(testspot.as_mut_ptr() as *const vec_t,
                    vec3_origin.as_mut_ptr(), vec3_origin.as_mut_ptr(),
                    testend.as_mut_ptr() as *const vec_t, 1 as libc::c_int,
                    0 as *mut edict_t, false_0);
        // NOTE: this code may does wrong result on moving brushes e.g. func_tracktrain
        if tr.fraction != 1.0f32 && tr.allsolid as u64 == 0 {
            // check impact plane normal
            let mut dot: libc::c_float =
                (*entry).impactPlaneNormal[0 as libc::c_int as usize] *
                    tr.plane.normal[0 as libc::c_int as usize] +
                    (*entry).impactPlaneNormal[1 as libc::c_int as usize] *
                        tr.plane.normal[1 as libc::c_int as usize] +
                    (*entry).impactPlaneNormal[2 as libc::c_int as usize] *
                        tr.plane.normal[2 as libc::c_int as usize];
            if dot >= 0.95f32 {
                entityIndex = pfnIndexOfEdict(tr.ent);
                if entityIndex > 0 as libc::c_int {
                    modelIndex = (*tr.ent).v.modelindex
                }
                SV_CreateDecal(&mut sv.signon, tr.endpos.as_mut_ptr(),
                               decalIndex, entityIndex, modelIndex, flags,
                               (*entry).scale);
            }
        }
    } else {
        // global entity is exist on new level so we can apply decal in local space
        SV_CreateDecal(&mut sv.signon, (*entry).position.as_mut_ptr(),
                       decalIndex, entityIndex, modelIndex, flags,
                       (*entry).scale);
    };
}
/*
=============
RestoreSound

continue playing sound from saved position
=============
*/
unsafe extern "C" fn RestoreSound(mut pSaveData: *mut SAVERESTOREDATA,
                                  mut snd: *mut soundlist_t) {
    let mut ent: *mut edict_t =
        EdictFromTable(pSaveData, (*snd).entnum as libc::c_int);
    let mut flags: libc::c_int = (1 as libc::c_int) << 12 as libc::c_int;
    // this can happens if serialized map contain 4096 static decals...
    if MSG_GetNumBytesLeft(&mut sv.signon) < 36 as libc::c_int { return }
    if (*snd).looping as u64 == 0 {
        flags = flags | (1 as libc::c_int) << 10 as libc::c_int
    }
    if SV_BuildSoundMsg(&mut sv.signon, ent, (*snd).channel as libc::c_int,
                        (*snd).name.as_mut_ptr(),
                        ((*snd).volume * 255 as libc::c_int as libc::c_float)
                            as libc::c_int, (*snd).attenuation, flags,
                        (*snd).pitch as libc::c_int,
                        (*snd).origin.as_mut_ptr() as *const vec_t) != 0 {
        // write extradata for svc_restoresound
        MSG_WriteByte(&mut sv.signon, (*snd).wordIndex as libc::c_int);
        MSG_WriteBytes(&mut sv.signon,
                       &mut (*snd).samplePos as *mut libc::c_double as
                           *const libc::c_void,
                       ::std::mem::size_of::<libc::c_double>() as
                           libc::c_ulong as libc::c_int);
        MSG_WriteBytes(&mut sv.signon,
                       &mut (*snd).forcedEnd as *mut libc::c_double as
                           *const libc::c_void,
                       ::std::mem::size_of::<libc::c_double>() as
                           libc::c_ulong as libc::c_int);
    };
}
/*
=============
SaveClientState

write out the list of premanent decals for this level
=============
*/
unsafe extern "C" fn SaveClientState(mut pSaveData: *mut SAVERESTOREDATA,
                                     mut level: *const libc::c_char,
                                     mut changelevel: libc::c_int) {
    let mut soundInfo: [soundlist_t; 320] =
        [soundlist_t{name: [0; 64],
                     entnum: 0,
                     origin: [0.; 3],
                     volume: 0.,
                     attenuation: 0.,
                     looping: false_0,
                     channel: 0,
                     pitch: 0,
                     wordIndex: 0,
                     samplePos: 0.,
                     forcedEnd: 0.,}; 320];
    let mut cl: *mut sv_client_t = svs.clients;
    let mut name: [libc::c_char; 64] = [0; 64];
    let mut i: libc::c_int = 0;
    let mut id: libc::c_int = 0;
    let mut version: libc::c_int = 0;
    let mut pTokenData: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut decalList: *mut decallist_t = 0 as *mut decallist_t;
    let mut header: SAVE_CLIENT =
        SAVE_CLIENT{decalCount: 0,
                    entityCount: 0,
                    soundCount: 0,
                    tempEntsCount: 0,
                    introTrack: [0; 64],
                    mainTrack: [0; 64],
                    trackPosition: 0,
                    viewentity: 0,
                    wateralpha: 0.,
                    wateramp: 0.,};
    let mut pFile: *mut file_t = 0 as *mut file_t;
    // clearing the saving buffer to reuse
    SaveClear(pSaveData);
    memset(&mut header as *mut SAVE_CLIENT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<SAVE_CLIENT>() as libc::c_ulong);
    // g-cont. add space for studiodecals if present
    decalList =
        _Mem_Alloc(host.mempool,
                   (::std::mem::size_of::<decallist_t>() as
                        libc::c_ulong).wrapping_mul(4096 as libc::c_int as
                                                        libc::c_ulong).wrapping_mul(2
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_ulong),
                   true_0,
                   b"../engine/server/sv_save.c\x00" as *const u8 as
                       *const libc::c_char, 1207 as libc::c_int) as
            *mut decallist_t;
    // initialize client header
    if !(host.type_0 == HOST_DEDICATED as libc::c_int as libc::c_uint) {
        header.decalCount =
            ref_0.dllFuncs.R_CreateDecalList.expect("non-null function pointer")(decalList)
    } else {
        // XASH_DEDICATED
        // we probably running a dedicated server
        header.decalCount = 0 as libc::c_int
    }
    header.entityCount = sv.num_static_entities;
    if changelevel == 0 {
        // sounds won't going across transition
        header.soundCount =
            S_GetCurrentDynamicSounds(soundInfo.as_mut_ptr(),
                                      256 as libc::c_int +
                                          (60 as libc::c_int +
                                               4 as libc::c_int));
        // music not reqiured to save position: it's just continue playing on a next level
        S_StreamGetCurrentState(header.introTrack.as_mut_ptr(),
                                header.mainTrack.as_mut_ptr(),
                                &mut header.trackPosition);
    }
    // save viewentity to allow camera works after save\restore
    if SV_CheckEdict((*cl).pViewEntity,
                     b"../engine/server/sv_save.c\x00" as *const u8 as
                         *const libc::c_char, 1234 as libc::c_int) as
           libc::c_uint != 0 && (*cl).pViewEntity != (*cl).edict {
        header.viewentity =
            (*cl).pViewEntity.wrapping_offset_from(svgame.edicts) as
                libc::c_long as libc::c_int as libc::c_short
    }
    header.wateralpha = sv_wateralpha.value;
    header.wateramp = sv_wateramp.value;
    // Store the client header
    svgame.dllFuncs.pfnSaveWriteFields.expect("non-null function pointer")(pSaveData,
                                                                           b"ClientHeader\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const libc::c_char,
                                                                           &mut header
                                                                               as
                                                                               *mut SAVE_CLIENT
                                                                               as
                                                                               *mut libc::c_void,
                                                                           gSaveClient.as_mut_ptr(),
                                                                           (::std::mem::size_of::<[TYPEDESCRIPTION; 10]>()
                                                                                as
                                                                                libc::c_ulong).wrapping_div(::std::mem::size_of::<TYPEDESCRIPTION>()
                                                                                                                as
                                                                                                                libc::c_ulong)
                                                                               as
                                                                               libc::c_int);
    // store decals
    i = 0 as libc::c_int;
    while i < header.decalCount {
        // NOTE: apply landmark offset only for brush entities without origin brushes
        if (*pSaveData).fUseLandmark != 0 &&
               (*decalList.offset(i as isize)).flags as libc::c_int &
                   0x2 as libc::c_int != 0 {
            (*decalList.offset(i as
                                   isize)).position[0 as libc::c_int as usize]
                =
                (*decalList.offset(i as
                                       isize)).position[0 as libc::c_int as
                                                            usize] -
                    (*pSaveData).vecLandmarkOffset[0 as libc::c_int as usize];
            (*decalList.offset(i as
                                   isize)).position[1 as libc::c_int as usize]
                =
                (*decalList.offset(i as
                                       isize)).position[1 as libc::c_int as
                                                            usize] -
                    (*pSaveData).vecLandmarkOffset[1 as libc::c_int as usize];
            (*decalList.offset(i as
                                   isize)).position[2 as libc::c_int as usize]
                =
                (*decalList.offset(i as
                                       isize)).position[2 as libc::c_int as
                                                            usize] -
                    (*pSaveData).vecLandmarkOffset[2 as libc::c_int as usize]
        }
        svgame.dllFuncs.pfnSaveWriteFields.expect("non-null function pointer")(pSaveData,
                                                                               b"DECALLIST\x00"
                                                                                   as
                                                                                   *const u8
                                                                                   as
                                                                                   *const libc::c_char,
                                                                               &mut *decalList.offset(i
                                                                                                          as
                                                                                                          isize)
                                                                                   as
                                                                                   *mut decallist_t
                                                                                   as
                                                                                   *mut libc::c_void,
                                                                               gDecalEntry.as_mut_ptr(),
                                                                               (::std::mem::size_of::<[TYPEDESCRIPTION; 8]>()
                                                                                    as
                                                                                    libc::c_ulong).wrapping_div(::std::mem::size_of::<TYPEDESCRIPTION>()
                                                                                                                    as
                                                                                                                    libc::c_ulong)
                                                                                   as
                                                                                   libc::c_int);
        i += 1
    }
    if !decalList.is_null() {
        _Mem_Free(decalList as *mut libc::c_void,
                  b"../engine/server/sv_save.c\x00" as *const u8 as
                      *const libc::c_char, 1252 as libc::c_int);
    }
    // write client entities
    i = 0 as libc::c_int;
    while i < header.entityCount {
        svgame.dllFuncs.pfnSaveWriteFields.expect("non-null function pointer")(pSaveData,
                                                                               b"STATICENTITY\x00"
                                                                                   as
                                                                                   *const u8
                                                                                   as
                                                                                   *const libc::c_char,
                                                                               &mut *svs.static_entities.offset(i
                                                                                                                    as
                                                                                                                    isize)
                                                                                   as
                                                                                   *mut entity_state_t
                                                                                   as
                                                                                   *mut libc::c_void,
                                                                               gStaticEntry.as_mut_ptr(),
                                                                               (::std::mem::size_of::<[TYPEDESCRIPTION; 34]>()
                                                                                    as
                                                                                    libc::c_ulong).wrapping_div(::std::mem::size_of::<TYPEDESCRIPTION>()
                                                                                                                    as
                                                                                                                    libc::c_ulong)
                                                                                   as
                                                                                   libc::c_int);
        i += 1
    }
    // write sounds
    i = 0 as libc::c_int;
    while i < header.soundCount {
        svgame.dllFuncs.pfnSaveWriteFields.expect("non-null function pointer")(pSaveData,
                                                                               b"SOUNDLIST\x00"
                                                                                   as
                                                                                   *const u8
                                                                                   as
                                                                                   *const libc::c_char,
                                                                               &mut *soundInfo.as_mut_ptr().offset(i
                                                                                                                       as
                                                                                                                       isize)
                                                                                   as
                                                                                   *mut soundlist_t
                                                                                   as
                                                                                   *mut libc::c_void,
                                                                               gSoundEntry.as_mut_ptr(),
                                                                               (::std::mem::size_of::<[TYPEDESCRIPTION; 11]>()
                                                                                    as
                                                                                    libc::c_ulong).wrapping_div(::std::mem::size_of::<TYPEDESCRIPTION>()
                                                                                                                    as
                                                                                                                    libc::c_ulong)
                                                                                   as
                                                                                   libc::c_int);
        i += 1
    }
    // Write entity string token table
    pTokenData = StoreHashTable(pSaveData);
    Q_snprintf(name.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
               b"save/%s.HL2\x00" as *const u8 as *const libc::c_char, level);
    // output to disk
    pFile =
        FS_Open(name.as_mut_ptr(),
                b"wb\x00" as *const u8 as *const libc::c_char,
                true_0); // something bad is happens
    if pFile.is_null() { return } // does not include token table
    version = 0x67 as libc::c_int;
    id =
        (('V' as i32) << 24 as libc::c_int) +
            (('A' as i32) << 16 as libc::c_int) +
            (('S' as i32) << 8 as libc::c_int) + 'J' as i32;
    FS_Write(pFile, &mut id as *mut libc::c_int as *const libc::c_void,
             ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    FS_Write(pFile, &mut version as *mut libc::c_int as *const libc::c_void,
             ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    FS_Write(pFile,
             &mut (*pSaveData).size as *mut libc::c_int as
                 *const libc::c_void,
             ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    // write out the tokens first so we can load them before we load the entities
    FS_Write(pFile,
             &mut (*pSaveData).tokenCount as *mut libc::c_int as
                 *const libc::c_void,
             ::std::mem::size_of::<libc::c_int>() as
                 libc::c_ulong); // header and globals
    FS_Write(pFile,
             &mut (*pSaveData).tokenSize as *mut libc::c_int as
                 *const libc::c_void,
             ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    FS_Write(pFile, pTokenData as *const libc::c_void,
             (*pSaveData).tokenSize as size_t);
    FS_Write(pFile, (*pSaveData).pBaseData as *const libc::c_void,
             (*pSaveData).size as size_t);
    FS_Close(pFile);
}
/*
=============
LoadClientState

read the list of decals and reapply them again
=============
*/
unsafe extern "C" fn LoadClientState(mut pSaveData: *mut SAVERESTOREDATA,
                                     mut level: *const libc::c_char,
                                     mut changelevel: qboolean,
                                     mut adjacent: qboolean) {
    let mut tokenCount: libc::c_int = 0; // something bad is happens
    let mut tokenSize: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut id: libc::c_int = 0;
    let mut version: libc::c_int = 0;
    let mut cl: *mut sv_client_t = svs.clients;
    let mut name: [libc::c_char; 64] = [0; 64];
    let mut soundEntry: soundlist_t =
        soundlist_t{name: [0; 64],
                    entnum: 0,
                    origin: [0.; 3],
                    volume: 0.,
                    attenuation: 0.,
                    looping: false_0,
                    channel: 0,
                    pitch: 0,
                    wordIndex: 0,
                    samplePos: 0.,
                    forcedEnd: 0.,};
    let mut decalEntry: decallist_t =
        decallist_t{position: [0.; 3],
                    name: [0; 64],
                    entityIndex: 0,
                    depth: 0,
                    flags: 0,
                    scale: 0.,
                    impactPlaneNormal: [0.; 3],
                    studio_state:
                        modelstate_t{sequence: 0,
                                     frame: 0,
                                     blending: [0; 2],
                                     controller: [0; 4],
                                     poseparam: [0; 16],
                                     body: 0,
                                     skin: 0,
                                     scale: 0,},};
    let mut header: SAVE_CLIENT =
        SAVE_CLIENT{decalCount: 0,
                    entityCount: 0,
                    soundCount: 0,
                    tempEntsCount: 0,
                    introTrack: [0; 64],
                    mainTrack: [0; 64],
                    trackPosition: 0,
                    viewentity: 0,
                    wateralpha: 0.,
                    wateramp: 0.,};
    let mut pFile: *mut file_t = 0 as *mut file_t;
    Q_snprintf(name.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
               b"save/%s.HL2\x00" as *const u8 as *const libc::c_char, level);
    pFile =
        FS_Open(name.as_mut_ptr(),
                b"rb\x00" as *const u8 as *const libc::c_char, true_0);
    if pFile.is_null() { return }
    FS_Read(pFile, &mut id as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    if id !=
           (('V' as i32) << 24 as libc::c_int) +
               (('A' as i32) << 16 as libc::c_int) +
               (('S' as i32) << 8 as libc::c_int) + 'J' as i32 {
        FS_Close(pFile);
        return
    }
    FS_Read(pFile, &mut version as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    if version != 0x67 as libc::c_int { FS_Close(pFile); return }
    FS_Read(pFile, &mut size as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    FS_Read(pFile, &mut tokenCount as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    FS_Read(pFile, &mut tokenSize as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    // sanity check
    if !((*pSaveData).bufferSize >= size + tokenSize) {
        Sys_Error(b"assert failed at %s:%i\n\x00" as *const u8 as
                      *const libc::c_char,
                  b"../engine/server/sv_save.c\x00" as *const u8 as
                      *const libc::c_char, 1328 as libc::c_int);
    }
    // clearing the restore buffer to reuse
    SaveClear(pSaveData);
    (*pSaveData).tokenCount = tokenCount;
    (*pSaveData).tokenSize = tokenSize;
    // Parse the symbol table
    BuildHashTable(pSaveData, pFile);
    FS_Read(pFile, (*pSaveData).pBaseData as *mut libc::c_void,
            size as size_t);
    FS_Close(pFile);
    // Read the client header
    svgame.dllFuncs.pfnSaveReadFields.expect("non-null function pointer")(pSaveData,
                                                                          b"ClientHeader\x00"
                                                                              as
                                                                              *const u8
                                                                              as
                                                                              *const libc::c_char,
                                                                          &mut header
                                                                              as
                                                                              *mut SAVE_CLIENT
                                                                              as
                                                                              *mut libc::c_void,
                                                                          gSaveClient.as_mut_ptr(),
                                                                          (::std::mem::size_of::<[TYPEDESCRIPTION; 10]>()
                                                                               as
                                                                               libc::c_ulong).wrapping_div(::std::mem::size_of::<TYPEDESCRIPTION>()
                                                                                                               as
                                                                                                               libc::c_ulong)
                                                                              as
                                                                              libc::c_int);
    // restore decals
    i = 0 as libc::c_int;
    while i < header.decalCount {
        svgame.dllFuncs.pfnSaveReadFields.expect("non-null function pointer")(pSaveData,
                                                                              b"DECALLIST\x00"
                                                                                  as
                                                                                  *const u8
                                                                                  as
                                                                                  *const libc::c_char,
                                                                              &mut decalEntry
                                                                                  as
                                                                                  *mut decallist_t
                                                                                  as
                                                                                  *mut libc::c_void,
                                                                              gDecalEntry.as_mut_ptr(),
                                                                              (::std::mem::size_of::<[TYPEDESCRIPTION; 8]>()
                                                                                   as
                                                                                   libc::c_ulong).wrapping_div(::std::mem::size_of::<TYPEDESCRIPTION>()
                                                                                                                   as
                                                                                                                   libc::c_ulong)
                                                                                  as
                                                                                  libc::c_int);
        // NOTE: apply landmark offset only for brush entities without origin brushes
        if (*pSaveData).fUseLandmark != 0 &&
               decalEntry.flags as libc::c_int & 0x2 as libc::c_int != 0 {
            decalEntry.position[0 as libc::c_int as usize] =
                decalEntry.position[0 as libc::c_int as usize] +
                    (*pSaveData).vecLandmarkOffset[0 as libc::c_int as usize];
            decalEntry.position[1 as libc::c_int as usize] =
                decalEntry.position[1 as libc::c_int as usize] +
                    (*pSaveData).vecLandmarkOffset[1 as libc::c_int as usize];
            decalEntry.position[2 as libc::c_int as usize] =
                decalEntry.position[2 as libc::c_int as usize] +
                    (*pSaveData).vecLandmarkOffset[2 as libc::c_int as usize]
        }
        RestoreDecal(pSaveData, &mut decalEntry, adjacent);
        i += 1
    }
    // clear old entities
    if adjacent as u64 == 0 {
        memset(svs.static_entities as *mut libc::c_void, 0 as libc::c_int,
               (::std::mem::size_of::<entity_state_t>() as
                    libc::c_ulong).wrapping_mul(3096 as libc::c_int as
                                                    libc::c_ulong));
        sv.num_static_entities = 0 as libc::c_int
    }
    // restore client entities
    i =
        0 as
            libc::c_int; // static entities won't loading from adjacent levels
    while i < header.entityCount {
        id = sv.num_static_entities;
        svgame.dllFuncs.pfnSaveReadFields.expect("non-null function pointer")(pSaveData,
                                                                              b"STATICENTITY\x00"
                                                                                  as
                                                                                  *const u8
                                                                                  as
                                                                                  *const libc::c_char,
                                                                              &mut *svs.static_entities.offset(id
                                                                                                                   as
                                                                                                                   isize)
                                                                                  as
                                                                                  *mut entity_state_t
                                                                                  as
                                                                                  *mut libc::c_void,
                                                                              gStaticEntry.as_mut_ptr(),
                                                                              (::std::mem::size_of::<[TYPEDESCRIPTION; 34]>()
                                                                                   as
                                                                                   libc::c_ulong).wrapping_div(::std::mem::size_of::<TYPEDESCRIPTION>()
                                                                                                                   as
                                                                                                                   libc::c_ulong)
                                                                                  as
                                                                                  libc::c_int);
        if !(adjacent as u64 != 0) {
            if SV_CreateStaticEntity(&mut sv.signon, id) as u64 != 0 {
                sv.num_static_entities += 1
            }
        }
        i += 1
    }
    // restore sounds
    i = 0 as libc::c_int; // sounds don't going across the levels
    while i < header.soundCount {
        svgame.dllFuncs.pfnSaveReadFields.expect("non-null function pointer")(pSaveData,
                                                                              b"SOUNDLIST\x00"
                                                                                  as
                                                                                  *const u8
                                                                                  as
                                                                                  *const libc::c_char,
                                                                              &mut soundEntry
                                                                                  as
                                                                                  *mut soundlist_t
                                                                                  as
                                                                                  *mut libc::c_void,
                                                                              gSoundEntry.as_mut_ptr(),
                                                                              (::std::mem::size_of::<[TYPEDESCRIPTION; 11]>()
                                                                                   as
                                                                                   libc::c_ulong).wrapping_div(::std::mem::size_of::<TYPEDESCRIPTION>()
                                                                                                                   as
                                                                                                                   libc::c_ulong)
                                                                                  as
                                                                                  libc::c_int);
        if !(adjacent as u64 != 0) {
            RestoreSound(pSaveData, &mut soundEntry);
        }
        i += 1
    }
    if adjacent as u64 == 0 {
        // restore camera view here
        let mut pent: *mut edict_t =
            (*(*pSaveData).pTable.offset(if header.viewentity as word as
                                                libc::c_int >=
                                                0 as libc::c_int {
                                             if (header.viewentity as word as
                                                     libc::c_int) <
                                                    (*pSaveData).tableCount {
                                                 header.viewentity as word as
                                                     libc::c_int
                                             } else {
                                                 (*pSaveData).tableCount
                                             }
                                         } else { 0 as libc::c_int } as
                                             isize)).pent;
        if if *header.introTrack.as_mut_ptr() == 0 {
               0 as libc::c_int
           } else { 1 as libc::c_int } != 0 {
            // NOTE: music is automatically goes across transition, never restore it on changelevel
            MSG_WriteCmdExt(&mut sv.signon, 9 as libc::c_int, NS_SERVER,
                            0 as *const libc::c_char);
            MSG_WriteString(&mut sv.signon,
                            va(b"music \"%s\" \"%s\" %i\n\x00" as *const u8 as
                                   *const libc::c_char,
                               header.introTrack.as_mut_ptr(),
                               header.mainTrack.as_mut_ptr(),
                               header.trackPosition));
        }
        // don't go camera across the levels
        if header.viewentity as libc::c_int > svs.maxclients &&
               changelevel as u64 == 0 {
            (*cl).pViewEntity = pent
        }
        // restore some client cvars
        Cvar_SetValue(b"sv_wateralpha\x00" as *const u8 as
                          *const libc::c_char, header.wateralpha);
        Cvar_SetValue(b"sv_wateramp\x00" as *const u8 as *const libc::c_char,
                      header.wateramp);
    };
}
/*
=============
CreateEntitiesInRestoreList

alloc private data for restored entities
=============
*/
unsafe extern "C" fn CreateEntitiesInRestoreList(mut pSaveData:
                                                     *mut SAVERESTOREDATA,
                                                 mut levelMask: libc::c_int,
                                                 mut create_world: qboolean) {
    let mut i: libc::c_int = 0;
    let mut active: libc::c_int = 0;
    let mut pTable: *mut ENTITYTABLE = 0 as *mut ENTITYTABLE;
    let mut pent: *mut edict_t = 0 as *mut edict_t;
    // create entity list
    if svgame.physFuncs.pfnCreateEntitiesInRestoreList.is_some() {
        svgame.physFuncs.pfnCreateEntitiesInRestoreList.expect("non-null function pointer")(pSaveData,
                                                                                            levelMask,
                                                                                            create_world);
    } else {
        i = 0 as libc::c_int;
        while i < (*pSaveData).tableCount {
            pTable =
                &mut *(*pSaveData).pTable.offset(i as isize) as
                    *mut ENTITYTABLE;
            pent = 0 as *mut edict_t;
            if (*pTable).classname != 0 && (*pTable).size != 0 &&
                   ((*pTable).flags & 0x40000000 as libc::c_int == 0 ||
                        create_world as u64 == 0) {
                if create_world as u64 == 0 {
                    active =
                        if (*pTable).flags & levelMask != 0 {
                            1 as libc::c_int
                        } else { 0 as libc::c_int }
                } else { active = 1 as libc::c_int }
                if (*pTable).id == 0 as libc::c_int &&
                       create_world as libc::c_uint != 0 {
                    // worldspawn
                    pent = SV_EdictNum(0 as libc::c_int);
                    SV_InitEdict(pent);
                    pent = SV_CreateNamedEntity(pent, (*pTable).classname)
                } else if (*pTable).id > 0 as libc::c_int &&
                              (*pTable).id < svs.maxclients + 1 as libc::c_int
                 {
                    let mut ed: *mut edict_t = SV_EdictNum((*pTable).id);
                    if (*pTable).flags as libc::c_uint &
                           0x80000000 as libc::c_uint == 0 {
                        Con_Printf(b"^1Error:^7 ENTITY IS NOT A PLAYER: %d\n\x00"
                                       as *const u8 as *const libc::c_char,
                                   i);
                    }
                    // create the player
                    if active != 0 &&
                           SV_CheckEdict(ed,
                                         b"../engine/server/sv_save.c\x00" as
                                             *const u8 as *const libc::c_char,
                                         1449 as libc::c_int) as libc::c_uint
                               != 0 {
                        pent = SV_CreateNamedEntity(ed, (*pTable).classname)
                    }
                } else if active != 0 {
                    pent =
                        SV_CreateNamedEntity(0 as *mut edict_t,
                                             (*pTable).classname)
                }
            }
            (*pTable).pent = pent;
            i += 1
        }
    };
}
/*
=============
SaveGameState

save current game state
=============
*/
unsafe extern "C" fn SaveGameState(mut changelevel: libc::c_int)
 -> *mut SAVERESTOREDATA {
    let mut name: [libc::c_char; 64] = [0; 64];
    let mut i: libc::c_int = 0;
    let mut id: libc::c_int = 0;
    let mut version: libc::c_int = 0;
    let mut pTableData: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pTokenData: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pSaveData: *mut SAVERESTOREDATA = 0 as *mut SAVERESTOREDATA;
    let mut tableSize: libc::c_int = 0;
    let mut dataSize: libc::c_int = 0;
    let mut pTable: *mut ENTITYTABLE = 0 as *mut ENTITYTABLE;
    let mut header: SAVE_HEADER =
        SAVE_HEADER{skillLevel: 0,
                    entityCount: 0,
                    connectionCount: 0,
                    lightStyleCount: 0,
                    time: 0.,
                    mapName: [0; 32],
                    skyName: [0; 32],
                    skyColor_r: 0,
                    skyColor_g: 0,
                    skyColor_b: 0,
                    skyVec_x: 0.,
                    skyVec_y: 0.,
                    skyVec_z: 0.,};
    let mut light: SAVE_LIGHTSTYLE =
        SAVE_LIGHTSTYLE{index: 0, style: [0; 256], time: 0.,};
    let mut pFile: *mut file_t = 0 as *mut file_t;
    if svgame.dllFuncs.pfnParmsChangeLevel.is_none() {
        return 0 as *mut SAVERESTOREDATA
    }
    pSaveData = SaveInit(0x400000 as libc::c_int, 0xfff as libc::c_int);
    Q_snprintf(name.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
               b"save/%s.HL1\x00" as *const u8 as *const libc::c_char,
               sv.name.as_mut_ptr());
    COM_FixSlashes(name.as_mut_ptr());
    // initialize entity table to count moved entities
    InitEntityTable(pSaveData, svgame.numEntities);
    // Build the adjacent map list
    svgame.dllFuncs.pfnParmsChangeLevel.expect("non-null function pointer")();
    // Write the global data
    header.skillLevel =
        skill.value as
            libc::c_int; // this is created from an int even though it's a float
    header.entityCount = (*pSaveData).tableCount; // use DLL time
    header.connectionCount = (*pSaveData).connectionCount;
    header.time = (*svgame.globals).time;
    Q_strncpy(header.mapName.as_mut_ptr(), sv.name.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong);
    Q_strncpy(header.skyName.as_mut_ptr(), sv_skyname.string,
              ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong);
    header.skyColor_r = sv_skycolor_r.value as libc::c_int;
    header.skyColor_g = sv_skycolor_g.value as libc::c_int;
    header.skyColor_b = sv_skycolor_b.value as libc::c_int;
    header.skyVec_x = sv_skyvec_x.value;
    header.skyVec_y = sv_skyvec_y.value;
    header.skyVec_z = sv_skyvec_z.value;
    header.lightStyleCount = 0 as libc::c_int;
    // counting the lightstyles
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        if sv.lightstyles[i as usize].pattern[0 as libc::c_int as usize] != 0
           {
            header.lightStyleCount += 1
        }
        i += 1
    }
    // Write the main header
    (*pSaveData).time =
        0.0f32; // prohibits rebase of header.time (keep compatibility with old saves)
    svgame.dllFuncs.pfnSaveWriteFields.expect("non-null function pointer")(pSaveData,
                                                                           b"Save Header\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const libc::c_char,
                                                                           &mut header
                                                                               as
                                                                               *mut SAVE_HEADER
                                                                               as
                                                                               *mut libc::c_void,
                                                                           gSaveHeader.as_mut_ptr(),
                                                                           (::std::mem::size_of::<[TYPEDESCRIPTION; 13]>()
                                                                                as
                                                                                libc::c_ulong).wrapping_div(::std::mem::size_of::<TYPEDESCRIPTION>()
                                                                                                                as
                                                                                                                libc::c_ulong)
                                                                               as
                                                                               libc::c_int);
    (*pSaveData).time = header.time;
    // Write the adjacency list
    i = 0 as libc::c_int;
    while i < (*pSaveData).connectionCount {
        svgame.dllFuncs.pfnSaveWriteFields.expect("non-null function pointer")(pSaveData,
                                                                               b"ADJACENCY\x00"
                                                                                   as
                                                                                   *const u8
                                                                                   as
                                                                                   *const libc::c_char,
                                                                               &mut *(*pSaveData).levelList.as_mut_ptr().offset(i
                                                                                                                                    as
                                                                                                                                    isize)
                                                                                   as
                                                                                   *mut LEVELLIST
                                                                                   as
                                                                                   *mut libc::c_void,
                                                                               gAdjacency.as_mut_ptr(),
                                                                               (::std::mem::size_of::<[TYPEDESCRIPTION; 4]>()
                                                                                    as
                                                                                    libc::c_ulong).wrapping_div(::std::mem::size_of::<TYPEDESCRIPTION>()
                                                                                                                    as
                                                                                                                    libc::c_ulong)
                                                                                   as
                                                                                   libc::c_int);
        i += 1
    }
    // Write the lightstyles
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        if !(sv.lightstyles[i as usize].pattern[0 as libc::c_int as usize] ==
                 0) {
            Q_strncpy(light.style.as_mut_ptr(),
                      sv.lightstyles[i as usize].pattern.as_mut_ptr(),
                      ::std::mem::size_of::<[libc::c_char; 256]>() as
                          libc::c_ulong);
            light.time = sv.lightstyles[i as usize].time;
            light.index = i;
            svgame.dllFuncs.pfnSaveWriteFields.expect("non-null function pointer")(pSaveData,
                                                                                   b"LIGHTSTYLE\x00"
                                                                                       as
                                                                                       *const u8
                                                                                       as
                                                                                       *const libc::c_char,
                                                                                   &mut light
                                                                                       as
                                                                                       *mut SAVE_LIGHTSTYLE
                                                                                       as
                                                                                       *mut libc::c_void,
                                                                                   gLightStyle.as_mut_ptr(),
                                                                                   (::std::mem::size_of::<[TYPEDESCRIPTION; 3]>()
                                                                                        as
                                                                                        libc::c_ulong).wrapping_div(::std::mem::size_of::<TYPEDESCRIPTION>()
                                                                                                                        as
                                                                                                                        libc::c_ulong)
                                                                                       as
                                                                                       libc::c_int);
        }
        i += 1
    }
    // build the table of entities
	// this is used to turn pointers into savable indices
	// build up ID numbers for each entity, for use in pointer conversions
	// if an entity requires a certain edict number upon restore, save that as well
    i = 0 as libc::c_int;
    while i < svgame.numEntities {
        pTable =
            &mut *(*pSaveData).pTable.offset(i as isize) as *mut ENTITYTABLE;
        (*pTable).location = (*pSaveData).size;
        (*pSaveData).currentIndex = i;
        (*pTable).size = 0 as libc::c_int;
        if !(SV_CheckEdict((*pTable).pent,
                           b"../engine/server/sv_save.c\x00" as *const u8 as
                               *const libc::c_char, 1553 as libc::c_int) as
                 u64 == 0) {
            svgame.dllFuncs.pfnSave.expect("non-null function pointer")((*pTable).pent,
                                                                        pSaveData);
            if (*(*pTable).pent).v.flags as libc::c_uint &
                   (1 as libc::c_uint) << 3 as libc::c_int != 0 {
                (*pTable).flags =
                    ((*pTable).flags as libc::c_uint |
                         0x80000000 as libc::c_uint) as libc::c_int
            }
        }
        i += 1
    }
    // total data what includes:
	// 1. save header
	// 2. adjacency list
	// 3. lightstyles
	// 4. all the entity data
    dataSize = (*pSaveData).size;
    // Write entity table
    pTableData = (*pSaveData).pCurrentData;
    i = 0 as libc::c_int;
    while i < (*pSaveData).tableCount {
        svgame.dllFuncs.pfnSaveWriteFields.expect("non-null function pointer")(pSaveData,
                                                                               b"ETABLE\x00"
                                                                                   as
                                                                                   *const u8
                                                                                   as
                                                                                   *const libc::c_char,
                                                                               &mut *(*pSaveData).pTable.offset(i
                                                                                                                    as
                                                                                                                    isize)
                                                                                   as
                                                                                   *mut ENTITYTABLE
                                                                                   as
                                                                                   *mut libc::c_void,
                                                                               gEntityTable.as_mut_ptr(),
                                                                               (::std::mem::size_of::<[TYPEDESCRIPTION; 5]>()
                                                                                    as
                                                                                    libc::c_ulong).wrapping_div(::std::mem::size_of::<TYPEDESCRIPTION>()
                                                                                                                    as
                                                                                                                    libc::c_ulong)
                                                                                   as
                                                                                   libc::c_int);
        i += 1
    }
    tableSize = (*pSaveData).size - dataSize;
    // Write entity string token table
    pTokenData = StoreHashTable(pSaveData);
    // output to disk
    pFile =
        FS_Open(name.as_mut_ptr(),
                b"wb\x00" as *const u8 as *const libc::c_char, true_0);
    if pFile.is_null() {
        // something bad is happens
        SaveFinish(pSaveData);
        return 0 as *mut SAVERESTOREDATA
    }
    // Write the header -- THIS SHOULD NEVER CHANGE STRUCTURE, USE SAVE_HEADER FOR NEW HEADER INFORMATION
	// THIS IS ONLY HERE TO IDENTIFY THE FILE AND GET IT'S SIZE.
    version = 0x71 as libc::c_int;
    id =
        (('V' as i32) << 24 as libc::c_int) +
            (('L' as i32) << 16 as libc::c_int) +
            (('A' as i32) << 8 as libc::c_int) + 'V' as i32;
    // write the header
    FS_Write(pFile, &mut id as *mut libc::c_int as *const libc::c_void,
             ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    FS_Write(pFile, &mut version as *mut libc::c_int as *const libc::c_void,
             ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    // Write out the tokens and table FIRST so they are loaded in the right order, then write out the rest of the data in the file.
    FS_Write(pFile,
             &mut (*pSaveData).size as *mut libc::c_int as
                 *const libc::c_void,
             ::std::mem::size_of::<libc::c_int>() as
                 libc::c_ulong); // total size of all data to initialize read buffer
    FS_Write(pFile,
             &mut (*pSaveData).tableCount as *mut libc::c_int as
                 *const libc::c_void,
             ::std::mem::size_of::<libc::c_int>() as
                 libc::c_ulong); // entities count to right initialize entity table
    FS_Write(pFile,
             &mut (*pSaveData).tokenCount as *mut libc::c_int as
                 *const libc::c_void,
             ::std::mem::size_of::<libc::c_int>() as
                 libc::c_ulong); // num hash tokens to prepare token table
    FS_Write(pFile,
             &mut (*pSaveData).tokenSize as *mut libc::c_int as
                 *const libc::c_void,
             ::std::mem::size_of::<libc::c_int>() as
                 libc::c_ulong); // total size of hash tokens
    FS_Write(pFile, pTokenData as *const libc::c_void,
             (*pSaveData).tokenSize as size_t); // write tokens into the file
    FS_Write(pFile, pTableData as *const libc::c_void,
             tableSize as size_t); // dump ETABLE structures
    FS_Write(pFile, (*pSaveData).pBaseData as *const libc::c_void,
             dataSize as size_t); // and finally store all the other data
    FS_Close(pFile);
    EntityPatchWrite(pSaveData, sv.name.as_mut_ptr());
    SaveClientState(pSaveData, sv.name.as_mut_ptr(), changelevel);
    return pSaveData;
}
/*
=============
LoadGameState

load current game state
=============
*/
unsafe extern "C" fn LoadGameState(mut level: *const libc::c_char,
                                   mut changelevel: qboolean) -> libc::c_int {
    let mut pSaveData: *mut SAVERESTOREDATA =
        0 as *mut SAVERESTOREDATA; // couldn't load the file
    let mut pTable: *mut ENTITYTABLE = 0 as *mut ENTITYTABLE;
    let mut header: SAVE_HEADER =
        SAVE_HEADER{skillLevel: 0,
                    entityCount: 0,
                    connectionCount: 0,
                    lightStyleCount: 0,
                    time: 0.,
                    mapName: [0; 32],
                    skyName: [0; 32],
                    skyColor_r: 0,
                    skyColor_g: 0,
                    skyColor_b: 0,
                    skyVec_x: 0.,
                    skyVec_y: 0.,
                    skyVec_z: 0.,};
    let mut pent: *mut edict_t = 0 as *mut edict_t;
    let mut i: libc::c_int = 0;
    pSaveData = LoadSaveData(level);
    if pSaveData.is_null() { return 0 as libc::c_int }
    ParseSaveTables(pSaveData, &mut header, true_0 as libc::c_int);
    EntityPatchRead(pSaveData, level);
    // pause until all clients connect
    sv.paused = true_0;
    sv.loadgame = sv.paused;
    Cvar_SetValue(b"skill\x00" as *const u8 as *const libc::c_char,
                  header.skillLevel as libc::c_float);
    Q_strncpy(sv.name.as_mut_ptr(), header.mapName.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    (*svgame.globals).mapname = SV_MakeString(sv.name.as_mut_ptr());
    Cvar_Set(b"sv_skyname\x00" as *const u8 as *const libc::c_char,
             header.skyName.as_mut_ptr());
    // restore sky parms
    Cvar_SetValue(b"sv_skycolor_r\x00" as *const u8 as *const libc::c_char,
                  header.skyColor_r as libc::c_float);
    Cvar_SetValue(b"sv_skycolor_g\x00" as *const u8 as *const libc::c_char,
                  header.skyColor_g as libc::c_float);
    Cvar_SetValue(b"sv_skycolor_b\x00" as *const u8 as *const libc::c_char,
                  header.skyColor_b as libc::c_float);
    Cvar_SetValue(b"sv_skyvec_x\x00" as *const u8 as *const libc::c_char,
                  header.skyVec_x);
    Cvar_SetValue(b"sv_skyvec_y\x00" as *const u8 as *const libc::c_char,
                  header.skyVec_y);
    Cvar_SetValue(b"sv_skyvec_z\x00" as *const u8 as *const libc::c_char,
                  header.skyVec_z);
    // create entity list
    CreateEntitiesInRestoreList(pSaveData, 0 as libc::c_int, true_0);
    // now spawn entities
    i = 0 as libc::c_int;
    while i < (*pSaveData).tableCount {
        pTable =
            &mut *(*pSaveData).pTable.offset(i as isize) as *mut ENTITYTABLE;
        (*pSaveData).pCurrentData =
            (*pSaveData).pBaseData.offset((*pTable).location as isize);
        (*pSaveData).size = (*pTable).location;
        (*pSaveData).currentIndex = i;
        pent = (*pTable).pent;
        if !pent.is_null() {
            if svgame.dllFuncs.pfnRestore.expect("non-null function pointer")(pent,
                                                                              pSaveData,
                                                                              0
                                                                                  as
                                                                                  libc::c_int)
                   < 0 as libc::c_int {
                (*pent).v.flags =
                    ((*pent).v.flags as libc::c_uint |
                         (1 as libc::c_uint) << 30 as libc::c_int) as
                        libc::c_int;
                (*pTable).pent = 0 as *mut edict_t
            }
        }
        i += 1
    }
    LoadClientState(pSaveData, level, changelevel, false_0);
    SaveFinish(pSaveData);
    // restore server time
    sv.time = header.time as libc::c_double;
    return 1 as libc::c_int;
}
/*
=============
SaveGameSlot

do a save game
=============
*/
unsafe extern "C" fn SaveGameSlot(mut pSaveName: *const libc::c_char,
                                  mut pSaveComment: *const libc::c_char)
 -> libc::c_int {
    let mut hlPath: [libc::c_char; 64] = [0; 64]; // re-init the buffer
    let mut name: [libc::c_char; 64] =
        [0; 64]; // get the name of level where a player
    let mut id: libc::c_int = 0; // counting all the adjacency maps
    let mut version: libc::c_int = 0;
    let mut pTokenData: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pSaveData: *mut SAVERESTOREDATA = 0 as *mut SAVERESTOREDATA;
    let mut gameHeader: GAME_HEADER =
        GAME_HEADER{mapName: [0; 32], comment: [0; 80], mapCount: 0,};
    let mut pFile: *mut file_t = 0 as *mut file_t;
    pSaveData = SaveGameState(false_0 as libc::c_int);
    if pSaveData.is_null() { return 0 as libc::c_int }
    SaveFinish(pSaveData);
    pSaveData = SaveInit(0x400000 as libc::c_int, 0xfff as libc::c_int);
    Q_strncpy(hlPath.as_mut_ptr(),
              b"save/*.HL?\x00" as *const u8 as *const libc::c_char,
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    Q_strncpy(gameHeader.mapName.as_mut_ptr(), sv.name.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong);
    Q_strncpy(gameHeader.comment.as_mut_ptr(), pSaveComment,
              ::std::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong);
    gameHeader.mapCount = DirectoryCount(hlPath.as_mut_ptr());
    // Store the game header
    svgame.dllFuncs.pfnSaveWriteFields.expect("non-null function pointer")(pSaveData,
                                                                           b"GameHeader\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const libc::c_char,
                                                                           &mut gameHeader
                                                                               as
                                                                               *mut GAME_HEADER
                                                                               as
                                                                               *mut libc::c_void,
                                                                           gGameHeader.as_mut_ptr(),
                                                                           (::std::mem::size_of::<[TYPEDESCRIPTION; 3]>()
                                                                                as
                                                                                libc::c_ulong).wrapping_div(::std::mem::size_of::<TYPEDESCRIPTION>()
                                                                                                                as
                                                                                                                libc::c_ulong)
                                                                               as
                                                                               libc::c_int);
    // Write the game globals
    svgame.dllFuncs.pfnSaveGlobalState.expect("non-null function pointer")(pSaveData);
    // Write entity string token table
    pTokenData = StoreHashTable(pSaveData);
    Q_snprintf(name.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
               b"save/%s.sav\x00" as *const u8 as *const libc::c_char,
               pSaveName);
    COM_FixSlashes(name.as_mut_ptr());
    // output to disk
    if Q_strnicmp(pSaveName, b"quick\x00" as *const u8 as *const libc::c_char,
                  99999 as libc::c_int) == 0 ||
           Q_strnicmp(pSaveName,
                      b"autosave\x00" as *const u8 as *const libc::c_char,
                      99999 as libc::c_int) == 0 {
        AgeSaveList(pSaveName, 2 as libc::c_int);
    }
    // output to disk
    pFile =
        FS_Open(name.as_mut_ptr(),
                b"wb\x00" as *const u8 as *const libc::c_char, true_0);
    if pFile.is_null() {
        // something bad is happens
        SaveFinish(pSaveData);
        return 0 as libc::c_int
    }
    // pending the preview image for savegame
    Cbuf_AddText(va(b"saveshot \"%s\"\n\x00" as *const u8 as
                        *const libc::c_char,
                    pSaveName)); // does not include token table
    Con_Printf(b"Saving game to %s...\n\x00" as *const u8 as
                   *const libc::c_char, name.as_mut_ptr());
    version = 0x71 as libc::c_int;
    id =
        (('V' as i32) << 24 as libc::c_int) +
            (('A' as i32) << 16 as libc::c_int) +
            (('S' as i32) << 8 as libc::c_int) + 'J' as i32;
    FS_Write(pFile, &mut id as *mut libc::c_int as *const libc::c_void,
             ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    FS_Write(pFile, &mut version as *mut libc::c_int as *const libc::c_void,
             ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    FS_Write(pFile,
             &mut (*pSaveData).size as *mut libc::c_int as
                 *const libc::c_void,
             ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    // write out the tokens first so we can load them before we load the entities
    FS_Write(pFile,
             &mut (*pSaveData).tokenCount as *mut libc::c_int as
                 *const libc::c_void,
             ::std::mem::size_of::<libc::c_int>() as
                 libc::c_ulong); // header and globals
    FS_Write(pFile,
             &mut (*pSaveData).tokenSize as *mut libc::c_int as
                 *const libc::c_void,
             ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    FS_Write(pFile, pTokenData as *const libc::c_void,
             (*pSaveData).tokenSize as size_t);
    FS_Write(pFile, (*pSaveData).pBaseData as *const libc::c_void,
             (*pSaveData).size as size_t);
    DirectoryCopy(hlPath.as_mut_ptr(), pFile);
    SaveFinish(pSaveData);
    FS_Close(pFile);
    return 1 as libc::c_int;
}
/*
=============
SaveReadHeader

read header of .sav file
=============
*/
unsafe extern "C" fn SaveReadHeader(mut pFile: *mut file_t,
                                    mut pHeader: *mut GAME_HEADER)
 -> libc::c_int {
    let mut tokenCount: libc::c_int = 0;
    let mut tokenSize: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut id: libc::c_int = 0;
    let mut version: libc::c_int = 0;
    let mut pSaveData: *mut SAVERESTOREDATA = 0 as *mut SAVERESTOREDATA;
    FS_Read(pFile, &mut id as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    if id !=
           (('V' as i32) << 24 as libc::c_int) +
               (('A' as i32) << 16 as libc::c_int) +
               (('S' as i32) << 8 as libc::c_int) + 'J' as i32 {
        FS_Close(pFile);
        return 0 as libc::c_int
    }
    FS_Read(pFile, &mut version as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    if version != 0x71 as libc::c_int {
        FS_Close(pFile);
        return 0 as libc::c_int
    }
    FS_Read(pFile, &mut size as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    FS_Read(pFile, &mut tokenCount as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    FS_Read(pFile, &mut tokenSize as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    pSaveData = SaveInit(size + tokenSize, tokenCount);
    (*pSaveData).tokenCount = tokenCount;
    (*pSaveData).tokenSize = tokenSize;
    // Parse the symbol table
    BuildHashTable(pSaveData, pFile);
    // Set up the restore basis
    (*pSaveData).fUseLandmark = false_0 as libc::c_int;
    (*pSaveData).time = 0.0f32;
    FS_Read(pFile, (*pSaveData).pBaseData as *mut libc::c_void,
            size as size_t);
    svgame.dllFuncs.pfnSaveReadFields.expect("non-null function pointer")(pSaveData,
                                                                          b"GameHeader\x00"
                                                                              as
                                                                              *const u8
                                                                              as
                                                                              *const libc::c_char,
                                                                          pHeader
                                                                              as
                                                                              *mut libc::c_void,
                                                                          gGameHeader.as_mut_ptr(),
                                                                          (::std::mem::size_of::<[TYPEDESCRIPTION; 3]>()
                                                                               as
                                                                               libc::c_ulong).wrapping_div(::std::mem::size_of::<TYPEDESCRIPTION>()
                                                                                                               as
                                                                                                               libc::c_ulong)
                                                                              as
                                                                              libc::c_int);
    svgame.dllFuncs.pfnRestoreGlobalState.expect("non-null function pointer")(pSaveData);
    SaveFinish(pSaveData);
    return 1 as libc::c_int;
}
/*
=============
CreateEntityTransitionList

moving edicts to another level
=============
*/
unsafe extern "C" fn CreateEntityTransitionList(mut pSaveData:
                                                    *mut SAVERESTOREDATA,
                                                mut levelMask: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut movedCount: libc::c_int = 0;
    let mut pTable: *mut ENTITYTABLE = 0 as *mut ENTITYTABLE;
    let mut pent: *mut edict_t = 0 as *mut edict_t;
    movedCount = 0 as libc::c_int;
    // create entity list
    CreateEntitiesInRestoreList(pSaveData, levelMask, false_0);
    // now spawn entities
    i = 0 as libc::c_int;
    while i < (*pSaveData).tableCount {
        pTable =
            &mut *(*pSaveData).pTable.offset(i as isize) as *mut ENTITYTABLE;
        (*pSaveData).pCurrentData =
            (*pSaveData).pBaseData.offset((*pTable).location as isize);
        (*pSaveData).size = (*pTable).location;
        (*pSaveData).currentIndex = i;
        pent = (*pTable).pent;
        if SV_CheckEdict(pent,
                         b"../engine/server/sv_save.c\x00" as *const u8 as
                             *const libc::c_char, 1844 as libc::c_int) as
               libc::c_uint != 0 && (*pTable).flags & levelMask != 0 {
            // screen out the player if he's not to be spawned
            if (*pTable).flags & 0x10000000 as libc::c_int != 0 {
                let mut tmpVars: entvars_t =
                    entvars_t{classname: 0,
                              globalname: 0,
                              origin: [0.; 3],
                              oldorigin: [0.; 3],
                              velocity: [0.; 3],
                              basevelocity: [0.; 3],
                              clbasevelocity: [0.; 3],
                              movedir: [0.; 3],
                              angles: [0.; 3],
                              avelocity: [0.; 3],
                              punchangle: [0.; 3],
                              v_angle: [0.; 3],
                              endpos: [0.; 3],
                              startpos: [0.; 3],
                              impacttime: 0.,
                              starttime: 0.,
                              fixangle: 0,
                              idealpitch: 0.,
                              pitch_speed: 0.,
                              ideal_yaw: 0.,
                              yaw_speed: 0.,
                              modelindex: 0,
                              model: 0,
                              viewmodel: 0,
                              weaponmodel: 0,
                              absmin: [0.; 3],
                              absmax: [0.; 3],
                              mins: [0.; 3],
                              maxs: [0.; 3],
                              size: [0.; 3],
                              ltime: 0.,
                              nextthink: 0.,
                              movetype: 0,
                              solid: 0,
                              skin: 0,
                              body: 0,
                              effects: 0,
                              gravity: 0.,
                              friction: 0.,
                              light_level: 0,
                              sequence: 0,
                              gaitsequence: 0,
                              frame: 0.,
                              animtime: 0.,
                              framerate: 0.,
                              controller: [0; 4],
                              blending: [0; 2],
                              scale: 0.,
                              rendermode: 0,
                              renderamt: 0.,
                              rendercolor: [0.; 3],
                              renderfx: 0,
                              health: 0.,
                              frags: 0.,
                              weapons: 0,
                              takedamage: 0.,
                              deadflag: 0,
                              view_ofs: [0.; 3],
                              button: 0,
                              impulse: 0,
                              chain: 0 as *mut edict_t,
                              dmg_inflictor: 0 as *mut edict_t,
                              enemy: 0 as *mut edict_t,
                              aiment: 0 as *mut edict_t,
                              owner: 0 as *mut edict_t,
                              groundentity: 0 as *mut edict_t,
                              spawnflags: 0,
                              flags: 0,
                              colormap: 0,
                              team: 0,
                              max_health: 0.,
                              teleport_time: 0.,
                              armortype: 0.,
                              armorvalue: 0.,
                              waterlevel: 0,
                              watertype: 0,
                              target: 0,
                              targetname: 0,
                              netname: 0,
                              message: 0,
                              dmg_take: 0.,
                              dmg_save: 0.,
                              dmg: 0.,
                              dmgtime: 0.,
                              noise: 0,
                              noise1: 0,
                              noise2: 0,
                              noise3: 0,
                              speed: 0.,
                              air_finished: 0.,
                              pain_finished: 0.,
                              radsuit_finished: 0.,
                              pContainingEntity: 0 as *mut edict_t,
                              playerclass: 0,
                              maxspeed: 0.,
                              fov: 0.,
                              weaponanim: 0,
                              pushmsec: 0,
                              bInDuck: 0,
                              flTimeStepSound: 0,
                              flSwimTime: 0,
                              flDuckTime: 0,
                              iStepLeft: 0,
                              flFallVelocity: 0.,
                              gamestate: 0,
                              oldbuttons: 0,
                              groupinfo: 0,
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
                              vuser4: [0.; 3],
                              euser1: 0 as *mut edict_t,
                              euser2: 0 as *mut edict_t,
                              euser3: 0 as *mut edict_t,
                              euser4: 0 as *mut edict_t,};
                let mut pNewEnt: *mut edict_t = 0 as *mut edict_t;
                // NOTE: we need to update table pointer so decals on the global entities with brush models can be
				// correctly moved. found the classname and the globalname for our globalentity
                svgame.dllFuncs.pfnSaveReadFields.expect("non-null function pointer")(pSaveData,
                                                                                      b"ENTVARS\x00"
                                                                                          as
                                                                                          *const u8
                                                                                          as
                                                                                          *const libc::c_char,
                                                                                      &mut tmpVars
                                                                                          as
                                                                                          *mut entvars_t
                                                                                          as
                                                                                          *mut libc::c_void,
                                                                                      gTempEntvars.as_mut_ptr(),
                                                                                      (::std::mem::size_of::<[TYPEDESCRIPTION; 2]>()
                                                                                           as
                                                                                           libc::c_ulong).wrapping_div(::std::mem::size_of::<TYPEDESCRIPTION>()
                                                                                                                           as
                                                                                                                           libc::c_ulong)
                                                                                          as
                                                                                          libc::c_int);
                // reset the save pointers, so dll can read this too
                (*pSaveData).pCurrentData =
                    (*pSaveData).pBaseData.offset((*pTable).location as
                                                      isize);
                (*pSaveData).size = (*pTable).location;
                // IMPORTANT: we should find the already spawned or local restored global entity
                pNewEnt =
                    SV_FindGlobalEntity(tmpVars.classname,
                                        tmpVars.globalname);
                Con_DPrintf(b"Merging changes for global: %s\n\x00" as
                                *const u8 as *const libc::c_char,
                            SV_GetString((*pTable).classname));
                // -------------------------------------------------------------------------
				// Pass the "global" flag to the DLL to indicate this entity should only override
				// a matching entity, not be spawned
                if svgame.dllFuncs.pfnRestore.expect("non-null function pointer")(pent,
                                                                                  pSaveData,
                                                                                  1
                                                                                      as
                                                                                      libc::c_int)
                       > 0 as libc::c_int {
                    movedCount += 1
                } else {
                    if SV_CheckEdict(pNewEnt,
                                     b"../engine/server/sv_save.c\x00" as
                                         *const u8 as *const libc::c_char,
                                     1873 as libc::c_int) as u64 != 0 {
                        // update the table so decals can find parent entity
                        (*pTable).pent = pNewEnt
                    }
                    (*pent).v.flags =
                        ((*pent).v.flags as libc::c_uint |
                             (1 as libc::c_uint) << 30 as libc::c_int) as
                            libc::c_int
                }
            } else {
                Con_Reportf(b"Transferring %s (%d)\n\x00" as *const u8 as
                                *const libc::c_char,
                            SV_GetString((*pTable).classname),
                            pent.wrapping_offset_from(svgame.edicts) as
                                libc::c_long as libc::c_int);
                if svgame.dllFuncs.pfnRestore.expect("non-null function pointer")(pent,
                                                                                  pSaveData,
                                                                                  0
                                                                                      as
                                                                                      libc::c_int)
                       < 0 as libc::c_int {
                    (*pent).v.flags =
                        ((*pent).v.flags as libc::c_uint |
                             (1 as libc::c_uint) << 30 as libc::c_int) as
                            libc::c_int
                } else if (*pTable).flags as libc::c_uint &
                              0x80000000 as libc::c_uint == 0 &&
                              EntityInSolid(pent) != 0 {
                    // this can happen during normal processing - PVS is just a guess,
						// some map areas won't exist in the new map
                    Con_Reportf(b"Suppressing %s\n\x00" as *const u8 as
                                    *const libc::c_char,
                                SV_GetString((*pTable).classname));
                    (*pent).v.flags =
                        ((*pent).v.flags as libc::c_uint |
                             (1 as libc::c_uint) << 30 as libc::c_int) as
                            libc::c_int
                } else {
                    (*pTable).flags = 0x40000000 as libc::c_int;
                    movedCount += 1
                }
            }
            // remove any entities that were removed using UTIL_Remove()
			// as a result of the above calls to UTIL_RemoveImmediate()
            SV_FreeOldEntities();
        }
        i += 1
    }
    return movedCount;
}
/*
=============
LoadAdjacentEnts

loading edicts from adjacency levels
=============
*/
unsafe extern "C" fn LoadAdjacentEnts(mut pOldLevel: *const libc::c_char,
                                      mut pLandmarkName:
                                          *const libc::c_char) {
    let mut header: SAVE_HEADER =
        SAVE_HEADER{skillLevel: 0,
                    entityCount: 0,
                    connectionCount: 0,
                    lightStyleCount: 0,
                    time: 0.,
                    mapName: [0; 32],
                    skyName: [0; 32],
                    skyColor_r: 0,
                    skyColor_g: 0,
                    skyColor_b: 0,
                    skyVec_x: 0.,
                    skyVec_y: 0.,
                    skyVec_z: 0.,};
    let mut currentLevelData: SAVERESTOREDATA =
        SAVERESTOREDATA{pBaseData: 0 as *mut libc::c_char,
                        pCurrentData: 0 as *mut libc::c_char,
                        size: 0,
                        bufferSize: 0,
                        tokenSize: 0,
                        tokenCount: 0,
                        pTokens: 0 as *mut *mut libc::c_char,
                        currentIndex: 0,
                        tableCount: 0,
                        connectionCount: 0,
                        pTable: 0 as *mut ENTITYTABLE,
                        levelList:
                            [LEVELLIST{mapName: [0; 32],
                                       landmarkName: [0; 32],
                                       pentLandmark: 0 as *mut edict_t,
                                       vecLandmarkOrigin: [0.; 3],}; 16],
                        fUseLandmark: 0,
                        szLandmarkName: [0; 20],
                        vecLandmarkOffset: [0.; 3],
                        time: 0.,
                        szCurrentMapName: [0; 32],};
    let mut pSaveData: *mut SAVERESTOREDATA = 0 as *mut SAVERESTOREDATA;
    let mut i: libc::c_int = 0;
    let mut test: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut movedCount: libc::c_int = 0 as libc::c_int;
    let mut foundprevious: qboolean = false_0;
    let mut landmarkOrigin: vec3_t = [0.; 3];
    memset(&mut currentLevelData as *mut SAVERESTOREDATA as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<SAVERESTOREDATA>() as libc::c_ulong);
    (*svgame.globals).pSaveData =
        &mut currentLevelData as *mut SAVERESTOREDATA as *mut libc::c_void;
    sv.paused = true_0;
    sv.loadgame = sv.paused;
    // build the adjacent map list
    svgame.dllFuncs.pfnParmsChangeLevel.expect("non-null function pointer")();
    i = 0 as libc::c_int;
    while i < currentLevelData.connectionCount {
        // make sure the previous level is in the connection list so we can
		// bring over the player.
        if Q_strnicmp(currentLevelData.levelList[i as
                                                     usize].mapName.as_mut_ptr(),
                      pOldLevel, 99999 as libc::c_int) == 0 {
            foundprevious = true_0
        }
        test = 0 as libc::c_int;
        while test < i {
            // only do maps once
            if Q_strnicmp(currentLevelData.levelList[i as
                                                         usize].mapName.as_mut_ptr(),
                          currentLevelData.levelList[test as
                                                         usize].mapName.as_mut_ptr(),
                          99999 as libc::c_int) == 0 {
                break ;
            }
            test += 1
        }
        // map was already in the list
        if !(test < i) {
            pSaveData =
                LoadSaveData(currentLevelData.levelList[i as
                                                            usize].mapName.as_mut_ptr()); // - header.time;
            if !pSaveData.is_null() {
                ParseSaveTables(pSaveData, &mut header,
                                false_0 as libc::c_int);
                EntityPatchRead(pSaveData,
                                currentLevelData.levelList[i as
                                                               usize].mapName.as_mut_ptr());
                (*pSaveData).time = sv.time as libc::c_float;
                (*pSaveData).fUseLandmark = true_0 as libc::c_int;
                movedCount = 0 as libc::c_int;
                flags = movedCount;
                index = -(1 as libc::c_int);
                // calculate landmark offset
                LandmarkOrigin(&mut currentLevelData,
                               landmarkOrigin.as_mut_ptr(), pLandmarkName);
                LandmarkOrigin(pSaveData,
                               (*pSaveData).vecLandmarkOffset.as_mut_ptr(),
                               pLandmarkName);
                (*pSaveData).vecLandmarkOffset[0 as libc::c_int as usize] =
                    landmarkOrigin[0 as libc::c_int as usize] -
                        (*pSaveData).vecLandmarkOffset[0 as libc::c_int as
                                                           usize];
                (*pSaveData).vecLandmarkOffset[1 as libc::c_int as usize] =
                    landmarkOrigin[1 as libc::c_int as usize] -
                        (*pSaveData).vecLandmarkOffset[1 as libc::c_int as
                                                           usize];
                (*pSaveData).vecLandmarkOffset[2 as libc::c_int as usize] =
                    landmarkOrigin[2 as libc::c_int as usize] -
                        (*pSaveData).vecLandmarkOffset[2 as libc::c_int as
                                                           usize];
                if Q_strnicmp(currentLevelData.levelList[i as
                                                             usize].mapName.as_mut_ptr(),
                              pOldLevel, 99999 as libc::c_int) == 0 {
                    flags =
                        (flags as libc::c_uint | 0x80000000 as libc::c_uint)
                            as libc::c_int
                }
                loop  {
                    index =
                        EntryInTable(pSaveData, sv.name.as_mut_ptr(), index);
                    if index < 0 as libc::c_int { break ; }
                    flags =
                        (flags as libc::c_uint | (1 as libc::c_uint) << index)
                            as libc::c_int
                }
                if flags != 0 {
                    movedCount = CreateEntityTransitionList(pSaveData, flags)
                }
                // if ents were moved, rewrite entity table to save file
                if movedCount != 0 {
                    EntityPatchWrite(pSaveData,
                                     currentLevelData.levelList[i as
                                                                    usize].mapName.as_mut_ptr());
                }
                // move the decals from another level
                LoadClientState(pSaveData,
                                currentLevelData.levelList[i as
                                                               usize].mapName.as_mut_ptr(),
                                true_0, true_0);
                SaveFinish(pSaveData);
            }
        }
        i += 1
    }
    (*svgame.globals).pSaveData = 0 as *mut libc::c_void;
    if foundprevious as u64 == 0 {
        Host_Error(b"Level transition ERROR\nCan\'t find connection to %s from %s\n\x00"
                       as *const u8 as *const libc::c_char, pOldLevel,
                   sv.name.as_mut_ptr());
    };
}
/*
=============
SV_LoadGameState

loading entities from the savegame
=============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_LoadGameState(mut level: *const libc::c_char)
 -> libc::c_int {
    return LoadGameState(level, false_0);
}
/*
=============
SV_ClearGameState

clear current game state
=============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_ClearGameState() {
    ClearSaveDir();
    if svgame.dllFuncs.pfnResetGlobalState.is_some() {
        svgame.dllFuncs.pfnResetGlobalState.expect("non-null function pointer")();
    };
}
/*
=============
SV_ChangeLevel
=============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_ChangeLevel(mut loadfromsavedgame: qboolean,
                                        mut mapname: *const libc::c_char,
                                        mut start: *const libc::c_char,
                                        mut background: qboolean) {
    let mut level: [libc::c_char; 64] = [0; 64];
    let mut oldlevel: [libc::c_char; 64] = [0; 64];
    let mut _startspot: [libc::c_char; 64] = [0; 64];
    let mut startspot: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pSaveData: *mut SAVERESTOREDATA = 0 as *mut SAVERESTOREDATA;
    if sv.state as libc::c_uint != ss_active as libc::c_int as libc::c_uint {
        Con_Printf(b"^1Error:^7 server not running\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    if !start.is_null() {
        Q_strncpy(_startspot.as_mut_ptr(), start,
                  256 as libc::c_int as size_t);
        startspot = _startspot.as_mut_ptr()
    }
    Q_strncpy(level.as_mut_ptr(), mapname, 256 as libc::c_int as size_t);
    Q_strncpy(oldlevel.as_mut_ptr(), sv.name.as_mut_ptr(),
              256 as libc::c_int as size_t);
    if loadfromsavedgame as u64 != 0 {
        // smooth transition in-progress
        (*svgame.globals).changelevel = true_0 as libc::c_int;
        // save the current level's state
        pSaveData = SaveGameState(true_0 as libc::c_int)
    } // ???
    SV_InactivateClients();
    SV_FinalMessage(b"\x00" as *const u8 as *const libc::c_char, true_0);
    SV_DeactivateServer();
    if SV_SpawnServer(level.as_mut_ptr(), startspot, background) as u64 == 0 {
        return
    }
    if loadfromsavedgame as u64 != 0 {
        // finish saving gamestate
        SaveFinish(pSaveData);
        if LoadGameState(level.as_mut_ptr(), true_0) == 0 {
            SV_SpawnEntities(level.as_mut_ptr());
        }
        LoadAdjacentEnts(oldlevel.as_mut_ptr(), startspot);
        if sv_newunit.value != 0. { ClearSaveDir(); }
        SV_ActivateServer(false_0 as libc::c_int);
    } else {
        // classic quake changelevel
        svgame.dllFuncs.pfnResetGlobalState.expect("non-null function pointer")();
        SV_SpawnEntities(level.as_mut_ptr());
        SV_ActivateServer(true_0 as libc::c_int);
    };
}
/*
=============
SV_LoadGame
=============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_LoadGame(mut pPath: *const libc::c_char)
 -> qboolean {
    let mut validload: qboolean = false_0;
    let mut gameHeader: GAME_HEADER =
        GAME_HEADER{mapName: [0; 32], comment: [0; 80], mapCount: 0,};
    let mut pFile: *mut file_t = 0 as *mut file_t;
    let mut flags: uint = 0;
    if host.type_0 == HOST_DEDICATED as libc::c_int as libc::c_uint {
        return false_0
    }
    if UI_CreditsActive() as u64 != 0 { return false_0 }
    if if pPath.is_null() || *pPath == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return false_0
    }
    // silently ignore if missed
    if FS_FileExists(pPath, true_0 as libc::c_int) == 0 { return false_0 }
    // initialize game if needs
    if SV_InitGame() as u64 == 0 { return false_0 }
    svs.initialized = true_0;
    pFile =
        FS_Open(pPath, b"rb\x00" as *const u8 as *const libc::c_char, true_0);
    if !pFile.is_null() {
        SV_ClearGameState();
        if SaveReadHeader(pFile, &mut gameHeader) != 0 {
            DirectoryExtract(pFile, gameHeader.mapCount);
            validload = true_0
        }
        FS_Close(pFile);
        if validload as u64 != 0 {
            // now check for map problems
            flags =
                SV_MapIsValid(gameHeader.mapName.as_mut_ptr(),
                              (*SI.GameInfo).sp_entity.as_mut_ptr(),
                              0 as *const libc::c_char);
            if flags & (1 as libc::c_uint) << 3 as libc::c_int != 0 {
                Con_Printf(b"^1Error:^7 map %s is invalid or not supported\n\x00"
                               as *const u8 as *const libc::c_char,
                           gameHeader.mapName.as_mut_ptr());
                validload = false_0
            }
            if flags & (1 as libc::c_uint) << 0 as libc::c_int == 0 {
                Con_Printf(b"^1Error:^7 map %s doesn\'t exist\n\x00" as
                               *const u8 as *const libc::c_char,
                           gameHeader.mapName.as_mut_ptr());
                validload = false_0
            }
        }
    }
    if validload as u64 == 0 {
        Con_Printf(b"^1Error:^7 Couldn\'t load %s\n\x00" as *const u8 as
                       *const libc::c_char, pPath);
        return false_0
    }
    Con_Printf(b"Loading game from %s...\n\x00" as *const u8 as
                   *const libc::c_char, pPath);
    Cvar_FullSet(b"maxplayers\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 30 as libc::c_int);
    Cvar_SetValue(b"deathmatch\x00" as *const u8 as *const libc::c_char,
                  0 as libc::c_int as libc::c_float);
    Cvar_SetValue(b"coop\x00" as *const u8 as *const libc::c_char,
                  0 as libc::c_int as libc::c_float);
    COM_LoadGame(gameHeader.mapName.as_mut_ptr());
    return true_0;
}
/*
==================
SV_SaveGame
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_SaveGame(mut pName: *const libc::c_char) {
    let mut comment: [libc::c_char; 80] = [0; 80];
    let mut result: libc::c_int = 0;
    let mut savename: string = [0; 256];
    if if pName.is_null() || *pName == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return
    }
    // can we save at this point?
    if IsValidSave() == 0 { return }
    if Q_strnicmp(pName, b"new\x00" as *const u8 as *const libc::c_char,
                  99999 as libc::c_int) == 0 {
        let mut n: libc::c_int = 0;
        // scan for a free filename
        n = 0 as libc::c_int;
        while n < 1000 as libc::c_int {
            Q_snprintf(savename.as_mut_ptr(),
                       ::std::mem::size_of::<string>() as libc::c_ulong,
                       b"save%03d\x00" as *const u8 as *const libc::c_char,
                       n);
            if FS_FileExists(va(b"save/%s.sav\x00" as *const u8 as
                                    *const libc::c_char,
                                savename.as_mut_ptr()), true_0 as libc::c_int)
                   == 0 {
                break ;
            }
            n += 1
        }
        if n == 1000 as libc::c_int {
            Con_Printf(b"^1Error:^7 no free slots for savegame\n\x00" as
                           *const u8 as *const libc::c_char);
            return
        }
    } else {
        Q_strncpy(savename.as_mut_ptr(), pName,
                  ::std::mem::size_of::<string>() as libc::c_ulong);
    }
    // unload previous image from memory (it's will be overwritten)
    GL_FreeImage(va(b"save/%s.bmp\x00" as *const u8 as *const libc::c_char,
                    savename.as_mut_ptr()));
    // XASH_DEDICATED
    SaveBuildComment(comment.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 80]>() as
                         libc::c_ulong as libc::c_int);
    result = SaveGameSlot(savename.as_mut_ptr(), comment.as_mut_ptr());
    if result != 0 &&
           host.features &
               ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint == 0 {
        CL_HudMessage(b"GAMESAVED\x00" as *const u8 as *const libc::c_char);
    };
    // defined in titles.txt
    // XASH_DEDICATED
}
/*
==================
SV_GetLatestSave

used for reload game after player death
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_GetLatestSave() -> *const libc::c_char {
    static mut savename: [libc::c_char; 64] = [0; 64];
    let mut newest: libc::c_int = 0 as libc::c_int;
    let mut ft: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut found: libc::c_int = 0 as libc::c_int;
    let mut t: *mut search_t = 0 as *mut search_t;
    t =
        FS_Search(b"save/*.sav\x00" as *const u8 as *const libc::c_char,
                  true_0 as libc::c_int, true_0 as libc::c_int);
    if t.is_null() { return 0 as *const libc::c_char }
    i = 0 as libc::c_int;
    while i < (*t).numfilenames {
        ft = FS_FileTime(*(*t).filenames.offset(i as isize), true_0);
        // found a match?
        if ft > 0 as libc::c_int {
            // should we use the matched?
            if found == 0 ||
                   Host_CompareFileTime(newest, ft) < 0 as libc::c_int {
                Q_strncpy(savename.as_mut_ptr(),
                          *(*t).filenames.offset(i as isize),
                          ::std::mem::size_of::<[libc::c_char; 64]>() as
                              libc::c_ulong); // release search
                newest = ft;
                found = 1 as libc::c_int
            }
        }
        i += 1
    }
    _Mem_Free(t as *mut libc::c_void,
              b"../engine/server/sv_save.c\x00" as *const u8 as
                  *const libc::c_char, 2252 as libc::c_int);
    if found != 0 { return savename.as_mut_ptr() }
    return 0 as *const libc::c_char;
}
/*
==================
SV_GetSaveComment

check savegame for valid
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_GetSaveComment(mut savename: *const libc::c_char,
                                           mut comment: *mut libc::c_char)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut tag: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut nNumberOfFields: libc::c_int = 0;
    let mut nFieldSize: libc::c_int = 0;
    let mut tokenSize: libc::c_int = 0;
    let mut tokenCount: libc::c_int = 0;
    let mut pData: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pSaveData: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pFieldName: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pTokenList: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut mapName: string = [0; 256];
    let mut description: string = [0; 256];
    let mut f: *mut file_t = 0 as *mut file_t;
    f =
        FS_Open(savename, b"rb\x00" as *const u8 as *const libc::c_char,
                true_0);
    if f.is_null() {
        // just not exist - clear comment
        *comment.offset(0 as libc::c_int as isize) =
            '\u{0}' as i32 as libc::c_char;
        return 0 as libc::c_int
    }
    FS_Read(f, &mut tag as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    if tag !=
           (('V' as i32) << 24 as libc::c_int) +
               (('A' as i32) << 16 as libc::c_int) +
               (('S' as i32) << 8 as libc::c_int) + 'J' as i32 {
        // invalid header
        Q_strncpy(comment,
                  b"<corrupted>\x00" as *const u8 as *const libc::c_char,
                  256 as libc::c_int as size_t);
        FS_Close(f);
        return 0 as libc::c_int
    }
    FS_Read(f, &mut tag as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    if tag == 0x65 as libc::c_int {
        Q_strncpy(comment,
                  b"<old version Xash3D unsupported>\x00" as *const u8 as
                      *const libc::c_char, 256 as libc::c_int as size_t);
        FS_Close(f);
        return 0 as libc::c_int
    }
    if tag < 0x71 as libc::c_int {
        Q_strncpy(comment,
                  b"<old version>\x00" as *const u8 as *const libc::c_char,
                  256 as libc::c_int as size_t);
        FS_Close(f);
        return 0 as libc::c_int
    }
    if tag > 0x71 as libc::c_int {
        // old xash version ?
        Q_strncpy(comment,
                  b"<invalid version>\x00" as *const u8 as
                      *const libc::c_char,
                  256 as libc::c_int as
                      size_t); // These two ints are the token list
        FS_Close(f);
        return 0 as libc::c_int
    }
    mapName[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    *comment.offset(0 as libc::c_int as isize) =
        '\u{0}' as i32 as libc::c_char;
    FS_Read(f, &mut size as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    FS_Read(f, &mut tokenCount as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    FS_Read(f, &mut tokenSize as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    size += tokenSize;
    // sanity check.
    if tokenCount < 0 as libc::c_int || tokenCount > 0xfff as libc::c_int {
        Q_strncpy(comment,
                  b"<corrupted hashtable>\x00" as *const u8 as
                      *const libc::c_char, 256 as libc::c_int as size_t);
        FS_Close(f);
        return 0 as libc::c_int
    }
    if tokenSize < 0 as libc::c_int || tokenSize > 0x400000 as libc::c_int {
        Q_strncpy(comment,
                  b"<corrupted hashtable>\x00" as *const u8 as
                      *const libc::c_char, 256 as libc::c_int as size_t);
        FS_Close(f);
        return 0 as libc::c_int
    }
    pSaveData =
        _Mem_Alloc(host.mempool, size as size_t, false_0,
                   b"../engine/server/sv_save.c\x00" as *const u8 as
                       *const libc::c_char, 2336 as libc::c_int) as
            *mut libc::c_char;
    FS_Read(f, pSaveData as *mut libc::c_void, size as size_t);
    pData = pSaveData;
    // allocate a table for the strings, and parse the table
    if tokenSize > 0 as libc::c_int {
        pTokenList =
            _Mem_Alloc(host.mempool,
                       (tokenCount as
                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                            as libc::c_ulong),
                       true_0,
                       b"../engine/server/sv_save.c\x00" as *const u8 as
                           *const libc::c_char, 2343 as libc::c_int) as
                *mut *mut libc::c_char;
        // make sure the token strings pointed to by the pToken hashtable.
        i = 0 as libc::c_int; // point to each string in the pToken table
        while i < tokenCount {
            let ref mut fresh7 = *pTokenList.offset(i as isize);
            *fresh7 =
                if *pData as libc::c_int != 0 {
                    pData
                } else { 0 as *mut libc::c_char };
            loop  {
                let fresh8 = pData;
                pData = pData.offset(1);
                if !(*fresh8 != 0) { break ; }
            }
            i += 1
            // find next token (after next null)
        }
    } else { pTokenList = 0 as *mut *mut libc::c_char }
    // short, short (size, index of field name)
    nFieldSize = *(pData as *mut libc::c_short) as libc::c_int;
    pData =
        pData.offset(::std::mem::size_of::<libc::c_short>() as libc::c_ulong
                         as isize);
    pFieldName = *pTokenList.offset(*(pData as *mut libc::c_short) as isize);
    if Q_strnicmp(pFieldName,
                  b"GameHeader\x00" as *const u8 as *const libc::c_char,
                  99999 as libc::c_int) != 0 {
        Q_strncpy(comment,
                  b"<missing GameHeader>\x00" as *const u8 as
                      *const libc::c_char, 256 as libc::c_int as size_t);
        if !pTokenList.is_null() {
            _Mem_Free(pTokenList as *mut libc::c_void,
                      b"../engine/server/sv_save.c\x00" as *const u8 as
                          *const libc::c_char, 2362 as libc::c_int);
        }
        if !pSaveData.is_null() {
            _Mem_Free(pSaveData as *mut libc::c_void,
                      b"../engine/server/sv_save.c\x00" as *const u8 as
                          *const libc::c_char, 2363 as libc::c_int);
        }
        FS_Close(f);
        return 0 as libc::c_int
    }
    // int (fieldcount)
    pData =
        pData.offset(::std::mem::size_of::<libc::c_short>() as libc::c_ulong
                         as isize);
    nNumberOfFields = *pData as libc::c_int;
    pData = pData.offset(nFieldSize as isize);
    // each field is a short (size), short (index of name), binary string of "size" bytes (data)
    i = 0 as libc::c_int;
    while i < nNumberOfFields {
        let mut size_0: size_t = 0;
        // Data order is:
		// Size
		// szName
		// Actual Data
        nFieldSize = *(pData as *mut libc::c_short) as libc::c_int;
        pData =
            pData.offset(::std::mem::size_of::<libc::c_short>() as
                             libc::c_ulong as isize);
        pFieldName =
            *pTokenList.offset(*(pData as *mut libc::c_short) as isize);
        pData =
            pData.offset(::std::mem::size_of::<libc::c_short>() as
                             libc::c_ulong as isize);
        size_0 =
            if nFieldSize < 256 as libc::c_int {
                nFieldSize
            } else { 256 as libc::c_int } as size_t;
        if Q_strnicmp(pFieldName,
                      b"comment\x00" as *const u8 as *const libc::c_char,
                      99999 as libc::c_int) == 0 {
            Q_strncpy(description.as_mut_ptr(), pData, size_0);
        } else if Q_strnicmp(pFieldName,
                             b"mapName\x00" as *const u8 as
                                 *const libc::c_char, 99999 as libc::c_int) ==
                      0 {
            Q_strncpy(mapName.as_mut_ptr(), pData, size_0);
        }
        // move to start of next field.
        pData = pData.offset(nFieldSize as isize);
        i += 1
    }
    // delete the string table we allocated
    if !pTokenList.is_null() {
        _Mem_Free(pTokenList as *mut libc::c_void,
                  b"../engine/server/sv_save.c\x00" as *const u8 as
                      *const libc::c_char, 2403 as libc::c_int);
    }
    if !pSaveData.is_null() {
        _Mem_Free(pSaveData as *mut libc::c_void,
                  b"../engine/server/sv_save.c\x00" as *const u8 as
                      *const libc::c_char, 2404 as libc::c_int);
    }
    FS_Close(f);
    // at least mapname should be filled
    if if *mapName.as_mut_ptr() == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } != 0 {
        let mut fileTime: time_t = 0;
        let mut file_tm: *const tm = 0 as *const tm;
        let mut timestring: string = [0; 256];
        let mut flags: uint = 0;
        // now check for map problems
        flags =
            SV_MapIsValid(mapName.as_mut_ptr(),
                          (*SI.GameInfo).sp_entity.as_mut_ptr(),
                          0 as *const libc::c_char);
        if flags & (1 as libc::c_uint) << 3 as libc::c_int != 0 {
            Q_strncpy(comment,
                      va(b"<map %s has invalid format>\x00" as *const u8 as
                             *const libc::c_char, mapName.as_mut_ptr()),
                      256 as libc::c_int as size_t);
            return 0 as libc::c_int
        }
        if flags & (1 as libc::c_uint) << 0 as libc::c_int == 0 {
            Q_strncpy(comment,
                      va(b"<map %s is missed>\x00" as *const u8 as
                             *const libc::c_char, mapName.as_mut_ptr()),
                      256 as libc::c_int as size_t);
            return 0 as libc::c_int
        }
        fileTime = FS_FileTime(savename, true_0) as time_t;
        file_tm = localtime(&mut fileTime);
        // split comment to sections
        if !Q_strstr(savename,
                     b"quick\x00" as *const u8 as
                         *const libc::c_char).is_null() {
            Q_strncat(comment,
                      b"[quick]\x00" as *const u8 as *const libc::c_char,
                      64 as libc::c_int as size_t);
        } else if !Q_strstr(savename,
                            b"autosave\x00" as *const u8 as
                                *const libc::c_char).is_null() {
            Q_strncat(comment,
                      b"[autosave]\x00" as *const u8 as *const libc::c_char,
                      64 as libc::c_int as size_t);
        }
        Q_strncat(comment, description.as_mut_ptr(),
                  64 as libc::c_int as size_t);
        strftime(timestring.as_mut_ptr(),
                 ::std::mem::size_of::<string>() as libc::c_ulong,
                 b"%b%d %Y\x00" as *const u8 as *const libc::c_char, file_tm);
        Q_strncpy(comment.offset(64 as libc::c_int as isize),
                  timestring.as_mut_ptr(), 16 as libc::c_int as size_t);
        strftime(timestring.as_mut_ptr(),
                 ::std::mem::size_of::<string>() as libc::c_ulong,
                 b"%H:%M\x00" as *const u8 as *const libc::c_char, file_tm);
        Q_strncpy(comment.offset(64 as libc::c_int as
                                     isize).offset(16 as libc::c_int as
                                                       isize),
                  timestring.as_mut_ptr(), 16 as libc::c_int as size_t);
        Q_strncpy(comment.offset(64 as libc::c_int as
                                     isize).offset((16 as libc::c_int *
                                                        2 as libc::c_int) as
                                                       isize),
                  description.as_mut_ptr().offset(64 as libc::c_int as isize),
                  64 as libc::c_int as size_t);
        return 1 as libc::c_int
    }
    Q_strncpy(comment,
              b"<unknown version>\x00" as *const u8 as *const libc::c_char,
              256 as libc::c_int as size_t);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SV_InitSaveRestore() {
    pfnSaveGameComment =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_:
                                                                *mut libc::c_char,
                                                            _: libc::c_int)
                                           ->
                                               ()>>(COM_GetProcAddress(svgame.hInstance,
                                                                       b"SV_SaveGameComment\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const libc::c_char));
}
