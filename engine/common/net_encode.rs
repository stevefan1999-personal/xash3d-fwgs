#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type file_s;
    pub type grasshdr_s;
    pub type engine_studio_api_s;
    pub type r_studio_interface_s;
    pub type IVoiceTweak_s;
    pub type demo_api_s;
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
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
    fn _COM_ParseFileSafe(data: *mut libc::c_char, token: *mut libc::c_char,
                          size: libc::c_int, flags: libc::c_uint,
                          len: *mut libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn Sys_Error(error: *const libc::c_char, _: ...);
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    static mut SI: sysinfo_t;
    #[no_mangle]
    fn _Mem_Realloc(poolptr: poolhandle_t, memptr: *mut libc::c_void,
                    size: size_t, clear: qboolean,
                    filename: *const libc::c_char, fileline: libc::c_int)
     -> *mut libc::c_void;
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
    fn COM_NormalizeAngles(angles: *mut vec_t);
    #[no_mangle]
    fn Host_Error(error: *const libc::c_char, _: ...);
    #[no_mangle]
    fn MSG_SeekToBit(sb: *mut sizebuf_t, bitPos: libc::c_int,
                     whence: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn MSG_WriteOneBit(sb: *mut sizebuf_t, nValue: libc::c_int);
    #[no_mangle]
    fn MSG_WriteUBitLong(sb: *mut sizebuf_t, curData: uint,
                         numbits: libc::c_int);
    #[no_mangle]
    fn MSG_WriteSBitLong(sb: *mut sizebuf_t, data: libc::c_int,
                         numbits: libc::c_int);
    #[no_mangle]
    fn MSG_WriteBitLong(sb: *mut sizebuf_t, data: uint, numbits: libc::c_int,
                        bSigned: qboolean);
    #[no_mangle]
    fn MSG_WriteBitAngle(sb: *mut sizebuf_t, fAngle: libc::c_float,
                         numbits: libc::c_int);
    #[no_mangle]
    fn MSG_WriteCmdExt(sb: *mut sizebuf_t, cmd: libc::c_int, type_0: netsrc_t,
                       name: *const libc::c_char);
    #[no_mangle]
    fn MSG_WriteFloat(sb: *mut sizebuf_t, val: libc::c_float);
    #[no_mangle]
    fn MSG_WriteString(sb: *mut sizebuf_t, pStr: *const libc::c_char)
     -> qboolean;
    #[no_mangle]
    fn MSG_ReadOneBit(sb: *mut sizebuf_t) -> libc::c_int;
    #[no_mangle]
    fn MSG_ReadBitAngle(sb: *mut sizebuf_t, numbits: libc::c_int)
     -> libc::c_float;
    #[no_mangle]
    fn MSG_ReadSBitLong(sb: *mut sizebuf_t, numbits: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn MSG_ReadUBitLong(sb: *mut sizebuf_t, numbits: libc::c_int) -> uint;
    #[no_mangle]
    fn MSG_ReadBitLong(sb: *mut sizebuf_t, numbits: libc::c_int,
                       bSigned: qboolean) -> uint;
    #[no_mangle]
    fn MSG_ReadFloat(sb: *mut sizebuf_t) -> libc::c_float;
    #[no_mangle]
    fn MSG_ReadStringExt(sb: *mut sizebuf_t, bLine: qboolean)
     -> *mut libc::c_char;
    #[no_mangle]
    static mut cl: client_t;
    #[no_mangle]
    static mut cls: client_static_t;
    #[no_mangle]
    static mut clgame: clgame_static_t;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type off_t = __off_t;
pub type HANDLE = *mut libc::c_void;
pub type HINSTANCE = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagPOINT {
    pub x: libc::c_int,
    pub y: libc::c_int,
}
pub type uint = libc::c_uint;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
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
pub type vec4_t = [vec_t; 4];
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
pub struct player_info_s {
    pub userid: libc::c_int,
    pub userinfo: [libc::c_char; 256],
    pub name: [libc::c_char; 32],
    pub spectator: libc::c_int,
    pub ping: libc::c_int,
    pub packet_loss: libc::c_int,
    pub model: [libc::c_char; 64],
    pub topcolor: libc::c_int,
    pub bottomcolor: libc::c_int,
    pub renderframe: libc::c_int,
    pub gaitsequence: libc::c_int,
    pub gaitframe: libc::c_float,
    pub gaityaw: libc::c_float,
    pub prevgaitorigin: vec3_t,
    pub customdata: customization_t,
    pub hashedcdkey: [libc::c_char; 16],
}
pub type player_info_t = player_info_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cvar_s {
    pub name: *mut libc::c_char,
    pub string: *mut libc::c_char,
    pub flags: libc::c_int,
    pub value: libc::c_float,
    pub next: *mut cvar_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct con_nprint_s {
    pub index: libc::c_int,
    pub time_to_live: libc::c_float,
    pub color: [libc::c_float; 3],
}
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
pub struct cmdalias_s {
    pub next: *mut cmdalias_s,
    pub name: [libc::c_char; 32],
    pub value: *mut libc::c_char,
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
pub struct delta_s {
    pub name: *const libc::c_char,
    pub offset: libc::c_int,
    pub size: libc::c_int,
    pub flags: libc::c_int,
    pub multiplier: libc::c_float,
    pub post_multiplier: libc::c_float,
    pub bits: libc::c_int,
    pub bInactive: qboolean,
}
pub type delta_t = delta_s;
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
pub type C2RustUnnamed = libc::c_uint;
pub const CUSTOM_CLIENT_ENCODE: C2RustUnnamed = 2;
pub const CUSTOM_SERVER_ENCODE: C2RustUnnamed = 1;
pub const CUSTOM_NONE: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const DELTA_STATIC: C2RustUnnamed_0 = 2;
pub const DELTA_PLAYER: C2RustUnnamed_0 = 1;
pub const DELTA_ENTITY: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct delta_field_t {
    pub name: *const libc::c_char,
    pub offset: libc::c_int,
    pub size: libc::c_int,
}
pub type pfnDeltaEncode
    =
    Option<unsafe extern "C" fn(_: *mut delta_s, _: *const byte,
                                _: *const byte) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct delta_info_t {
    pub pName: *const libc::c_char,
    pub pInfo: *const delta_field_t,
    pub maxFields: libc::c_int,
    pub numFields: libc::c_int,
    pub pFields: *mut delta_t,
    pub customEncode: libc::c_int,
    pub funcName: [libc::c_char; 32],
    pub userCallback: pfnDeltaEncode,
    pub bInitialized: qboolean,
}
pub type movevars_t = movevars_s;
pub type weapon_data_t = weapon_data_s;
pub type clientdata_t = clientdata_s;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct downloadtime_t {
    pub bUsed: qboolean,
    pub fTime: libc::c_float,
    pub nBytesRemaining: libc::c_int,
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
pub type predicted_player_t = cl_predicted_player_s;
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
pub type wrect_t = wrect_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wrect_s {
    pub left: libc::c_int,
    pub right: libc::c_int,
    pub top: libc::c_int,
    pub bottom: libc::c_int,
}
pub type local_state_t = local_state_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct local_state_s {
    pub playerstate: entity_state_t,
    pub client: clientdata_t,
    pub weapondata: [weapon_data_t; 64],
}
pub type connstate_t = connstate_e;
pub type connstate_e = libc::c_uint;
pub const ca_cinematic: connstate_e = 5;
pub const ca_active: connstate_e = 4;
pub const ca_validate: connstate_e = 3;
pub const ca_connected: connstate_e = 2;
pub const ca_connecting: connstate_e = 1;
pub const ca_disconnected: connstate_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client_t {
    pub servercount: libc::c_int,
    pub validsequence: libc::c_int,
    pub parsecount: libc::c_int,
    pub parsecountmod: libc::c_int,
    pub video_prepped: qboolean,
    pub audio_prepped: qboolean,
    pub paused: qboolean,
    pub delta_sequence: libc::c_int,
    pub mtime: [libc::c_double; 2],
    pub lerpFrac: libc::c_float,
    pub last_command_ack: libc::c_int,
    pub last_incoming_sequence: libc::c_int,
    pub send_reply: qboolean,
    pub background: qboolean,
    pub first_frame: qboolean,
    pub proxy_redirect: qboolean,
    pub skip_interp: qboolean,
    pub checksum: uint,
    pub frames: [frame_t; 64],
    pub commands: [runcmd_t; 64],
    pub predicted_frames: [local_state_t; 64],
    pub time: libc::c_double,
    pub oldtime: libc::c_double,
    pub timedelta: libc::c_float,
    pub serverinfo: [libc::c_char; 512],
    pub players: [player_info_t; 32],
    pub lastresourcecheck: libc::c_double,
    pub downloadUrl: string,
    pub events: event_state_t,
    pub local: cl_local_data_t,
    pub cmd: *mut usercmd_t,
    pub viewentity: libc::c_int,
    pub viewangles: vec3_t,
    pub viewheight: vec3_t,
    pub punchangle: vec3_t,
    pub intermission: libc::c_int,
    pub crosshairangle: vec3_t,
    pub predicted_angle: [pred_viewangle_t; 16],
    pub angle_position: libc::c_int,
    pub addangletotal: libc::c_float,
    pub prevaddangletotal: libc::c_float,
    pub simorg: vec3_t,
    pub simvel: vec3_t,
    pub playernum: libc::c_int,
    pub maxclients: libc::c_int,
    pub instanced_baseline: [entity_state_t; 64],
    pub instanced_baseline_count: libc::c_int,
    pub sound_precache: [[libc::c_char; 64]; 2048],
    pub event_precache: [[libc::c_char; 64]; 1024],
    pub files_precache: [[libc::c_char; 64]; 1024],
    pub lightstyles: [lightstyle_t; 64],
    pub models: [*mut model_t; 1025],
    pub nummodels: libc::c_int,
    pub numfiles: libc::c_int,
    pub consistency_list: [consistency_t; 1024],
    pub num_consistency: libc::c_int,
    pub need_force_consistency_response: qboolean,
    pub resourcesonhand: resource_t,
    pub resourcesneeded: resource_t,
    pub resourcelist: [resource_t; 5120],
    pub num_resources: libc::c_int,
    pub sound_index: [libc::c_short; 2048],
    pub decal_index: [libc::c_short; 512],
    pub worldmodel: *mut model_t,
    pub lostpackets: libc::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pred_viewangle_t {
    pub starttime: libc::c_float,
    pub total: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cl_local_data_t {
    pub predicted_origins: [vec3_t; 64],
    pub prediction_error: vec3_t,
    pub lastorigin: vec3_t,
    pub lastground: libc::c_int,
    pub interp_amount: libc::c_float,
    pub repredicting: qboolean,
    pub thirdperson: qboolean,
    pub apply_effects: qboolean,
    pub idealpitch: libc::c_float,
    pub viewmodel: libc::c_int,
    pub health: libc::c_int,
    pub onground: libc::c_int,
    pub light_level: libc::c_int,
    pub waterlevel: libc::c_int,
    pub usehull: libc::c_int,
    pub moving: libc::c_int,
    pub pushmsec: libc::c_int,
    pub weapons: libc::c_int,
    pub maxspeed: libc::c_float,
    pub scr_fov: libc::c_float,
    pub weaponsequence: libc::c_int,
    pub weaponstarttime: libc::c_float,
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
pub type runcmd_t = runcmd_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct runcmd_s {
    pub senttime: libc::c_double,
    pub receivedtime: libc::c_double,
    pub frame_lerp: libc::c_float,
    pub cmd: usercmd_t,
    pub processedfuncs: qboolean,
    pub heldback: qboolean,
    pub sendsize: libc::c_int,
}
pub type frame_t = frame_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct frame_s {
    pub receivedtime: libc::c_double,
    pub latency: libc::c_double,
    pub time: libc::c_double,
    pub valid: qboolean,
    pub choked: qboolean,
    pub clientdata: clientdata_t,
    pub playerstate: [entity_state_t; 32],
    pub weapondata: [weapon_data_t; 64],
    pub graphdata: netbandwidthgraph_t,
    pub flags: [byte; 256],
    pub num_entities: libc::c_int,
    pub first_entity: libc::c_int,
}
pub type netbandwidthgraph_t = netbandwithgraph_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct netbandwithgraph_s {
    pub client: word,
    pub players: word,
    pub entities: word,
    pub tentities: word,
    pub sound: word,
    pub event: word,
    pub usr: word,
    pub msgbytes: word,
    pub voicebytes: word,
}
pub type cl_entity_t = cl_entity_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clgame_static_t {
    pub hInstance: *mut libc::c_void,
    pub dllFuncs: cldll_func_t,
    pub drawFuncs: render_interface_t,
    pub mempool: poolhandle_t,
    pub mapname: string,
    pub maptitle: string,
    pub itemspath: string,
    pub entities: *mut cl_entity_t,
    pub static_entities: *mut cl_entity_t,
    pub remap_info: *mut *mut remap_info_t,
    pub maxEntities: libc::c_int,
    pub maxRemapInfos: libc::c_int,
    pub numStatics: libc::c_int,
    pub maxModels: libc::c_int,
    pub movevars: movevars_t,
    pub oldmovevars: movevars_t,
    pub pmove: *mut playermove_t,
    pub pushed: qboolean,
    pub oldviscount: libc::c_int,
    pub oldphyscount: libc::c_int,
    pub msg: [cl_user_message_t; 197],
    pub events: [*mut cl_user_event_t; 1024],
    pub cdtracks: [string; 32],
    pub sprites: [model_t; 256],
    pub viewport: [libc::c_int; 4],
    pub ds: client_draw_t,
    pub fade: screenfade_t,
    pub shake: screen_shake_t,
    pub centerPrint: center_print_t,
    pub scrInfo: SCREENINFO,
    pub overView: ref_overview_t,
    pub palette: [color24; 256],
    pub sprlist: [cached_spritelist_t; 256],
    pub titles: *mut client_textmessage_t,
    pub numTitles: libc::c_int,
    pub request_type: net_request_type_t,
    pub net_requests: [net_request_t; 64],
    pub master_request: *mut net_request_t,
    pub free_efrags: *mut efrag_t,
    pub viewent: cl_entity_t,
    pub client_dll_uses_sdl: qboolean,
}
pub type efrag_t = efrag_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct net_request_t {
    pub resp: net_response_t,
    pub pfnFunc: net_api_response_func_t,
    pub timeout: libc::c_double,
    pub timesend: libc::c_double,
    pub flags: libc::c_int,
}
pub type net_api_response_func_t
    =
    Option<unsafe extern "C" fn(_: *mut net_response_s) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct net_response_s {
    pub error: libc::c_int,
    pub context: libc::c_int,
    pub type_0: libc::c_int,
    pub remote_address: netadr_t,
    pub ping: libc::c_double,
    pub response: *mut libc::c_void,
}
pub type net_response_t = net_response_s;
pub type net_request_type_t = libc::c_uint;
pub const NET_REQUEST_CLIENT: net_request_type_t = 2;
pub const NET_REQUEST_GAMEUI: net_request_type_t = 1;
pub const NET_REQUEST_CANCEL: net_request_type_t = 0;
pub type client_textmessage_t = client_textmessage_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client_textmessage_s {
    pub effect: libc::c_int,
    pub r1: byte,
    pub g1: byte,
    pub b1: byte,
    pub a1: byte,
    pub r2: byte,
    pub g2: byte,
    pub b2: byte,
    pub a2: byte,
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub fadein: libc::c_float,
    pub fadeout: libc::c_float,
    pub holdtime: libc::c_float,
    pub fxtime: libc::c_float,
    pub pName: *const libc::c_char,
    pub pMessage: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cached_spritelist_t {
    pub szListName: [libc::c_char; 64],
    pub pList: *mut client_sprite_t,
    pub count: libc::c_int,
}
pub type client_sprite_t = client_sprite_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client_sprite_s {
    pub szName: [libc::c_char; 64],
    pub szSprite: [libc::c_char; 64],
    pub hspr: libc::c_int,
    pub iRes: libc::c_int,
    pub rc: wrect_t,
}
pub type ref_overview_t = ref_overview_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ref_overview_s {
    pub origin: vec3_t,
    pub rotated: qboolean,
    pub xLeft: libc::c_float,
    pub xRight: libc::c_float,
    pub yTop: libc::c_float,
    pub yBottom: libc::c_float,
    pub zFar: libc::c_float,
    pub zNear: libc::c_float,
    pub flZoom: libc::c_float,
}
pub type SCREENINFO = SCREENINFO_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCREENINFO_s {
    pub iSize: libc::c_int,
    pub iWidth: libc::c_int,
    pub iHeight: libc::c_int,
    pub iFlags: libc::c_int,
    pub iCharHeight: libc::c_int,
    pub charWidths: [libc::c_short; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct center_print_t {
    pub time: libc::c_float,
    pub y: libc::c_int,
    pub lines: libc::c_int,
    pub message: [libc::c_char; 2048],
    pub totalWidth: libc::c_int,
    pub totalHeight: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct screen_shake_t {
    pub time: libc::c_float,
    pub duration: libc::c_float,
    pub amplitude: libc::c_float,
    pub frequency: libc::c_float,
    pub next_shake: libc::c_float,
    pub offset: vec3_t,
    pub angle: libc::c_float,
    pub applied_offset: vec3_t,
    pub applied_angle: libc::c_float,
}
pub type screenfade_t = screenfade_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct screenfade_s {
    pub fadeSpeed: libc::c_float,
    pub fadeEnd: libc::c_float,
    pub fadeTotalEnd: libc::c_float,
    pub fadeReset: libc::c_float,
    pub fader: byte,
    pub fadeg: byte,
    pub fadeb: byte,
    pub fadealpha: byte,
    pub fadeFlags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client_draw_t {
    pub pSprite: *const model_t,
    pub scissor_x: libc::c_int,
    pub scissor_y: libc::c_int,
    pub scissor_width: libc::c_int,
    pub scissor_height: libc::c_int,
    pub scissor_test: qboolean,
    pub adjust_size: qboolean,
    pub renderMode: libc::c_int,
    pub cullMode: TRICULLSTYLE,
    pub textColor: rgba_t,
    pub spriteColor: rgba_t,
    pub triRGBA: vec4_t,
    pub pCrosshair: *const model_t,
    pub rcCrosshair: wrect_t,
    pub rgbaCrosshair: rgba_t,
}
pub type TRICULLSTYLE = libc::c_uint;
pub const TRI_NONE: TRICULLSTYLE = 1;
pub const TRI_FRONT: TRICULLSTYLE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cl_user_event_t {
    pub name: [libc::c_char; 64],
    pub index: word,
    pub func: pfnEventHook,
}
pub type pfnEventHook
    =
    Option<unsafe extern "C" fn(_: *mut event_args_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cl_user_message_t {
    pub name: [libc::c_char; 32],
    pub number: libc::c_int,
    pub size: libc::c_int,
    pub func: pfnUserMsgHook,
}
pub type pfnUserMsgHook
    =
    Option<unsafe extern "C" fn(_: *const libc::c_char, _: libc::c_int,
                                _: *mut libc::c_void) -> libc::c_int>;
pub type playermove_t = playermove_s;
pub type remap_info_t = remap_info_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct remap_info_s {
    pub textures: [libc::c_ushort; 32],
    pub ptexture: *mut mstudiotex_s,
    pub numtextures: libc::c_short,
    pub topcolor: libc::c_short,
    pub bottomcolor: libc::c_short,
    pub model: *mut model_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mstudiotex_s {
    pub name: [libc::c_char; 64],
    pub flags: uint32_t,
    pub width: int32_t,
    pub height: int32_t,
    pub index: int32_t,
}
pub type render_interface_t = render_interface_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct render_interface_s {
    pub version: libc::c_int,
    pub GL_RenderFrame: Option<unsafe extern "C" fn(_: *const ref_viewpass_s)
                                   -> libc::c_int>,
    pub GL_BuildLightmaps: Option<unsafe extern "C" fn() -> ()>,
    pub GL_OrthoBounds: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                    _: *const libc::c_float)
                                   -> ()>,
    pub R_CreateStudioDecalList: Option<unsafe extern "C" fn(_:
                                                                 *mut decallist_t,
                                                             _: libc::c_int)
                                            -> libc::c_int>,
    pub R_ClearStudioDecals: Option<unsafe extern "C" fn() -> ()>,
    pub R_SpeedsMessage: Option<unsafe extern "C" fn(_: *mut libc::c_char,
                                                     _: size_t) -> qboolean>,
    pub Mod_ProcessUserData: Option<unsafe extern "C" fn(_: *mut model_s,
                                                         _: qboolean,
                                                         _: *const byte)
                                        -> ()>,
    pub R_ProcessEntData: Option<unsafe extern "C" fn(_: qboolean) -> ()>,
    pub Mod_GetCurrentVis: Option<unsafe extern "C" fn() -> *mut byte>,
    pub R_NewMap: Option<unsafe extern "C" fn() -> ()>,
    pub R_ClearScene: Option<unsafe extern "C" fn() -> ()>,
    pub CL_UpdateLatchedVars: Option<unsafe extern "C" fn(_: *mut cl_entity_s,
                                                          _: qboolean) -> ()>,
}
pub type decallist_t = decallist_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ref_viewpass_s {
    pub viewport: [libc::c_int; 4],
    pub vieworigin: vec3_t,
    pub viewangles: vec3_t,
    pub viewentity: libc::c_int,
    pub fov_x: libc::c_float,
    pub fov_y: libc::c_float,
    pub flags: libc::c_int,
}
pub type cldll_func_t = cldll_func_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cldll_func_s {
    pub pfnInitialize: Option<unsafe extern "C" fn(_: *mut cl_enginefunc_t,
                                                   _: libc::c_int)
                                  -> libc::c_int>,
    pub pfnInit: Option<unsafe extern "C" fn() -> ()>,
    pub pfnVidInit: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub pfnRedraw: Option<unsafe extern "C" fn(_: libc::c_float,
                                               _: libc::c_int)
                              -> libc::c_int>,
    pub pfnUpdateClientData: Option<unsafe extern "C" fn(_:
                                                             *mut client_data_t,
                                                         _: libc::c_float)
                                        -> libc::c_int>,
    pub pfnReset: Option<unsafe extern "C" fn() -> ()>,
    pub pfnPlayerMove: Option<unsafe extern "C" fn(_: *mut playermove_s,
                                                   _: libc::c_int) -> ()>,
    pub pfnPlayerMoveInit: Option<unsafe extern "C" fn(_: *mut playermove_s)
                                      -> ()>,
    pub pfnPlayerMoveTexture: Option<unsafe extern "C" fn(_:
                                                              *mut libc::c_char)
                                         -> libc::c_char>,
    pub IN_ActivateMouse: Option<unsafe extern "C" fn() -> ()>,
    pub IN_DeactivateMouse: Option<unsafe extern "C" fn() -> ()>,
    pub IN_MouseEvent: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub IN_ClearStates: Option<unsafe extern "C" fn() -> ()>,
    pub IN_Accumulate: Option<unsafe extern "C" fn() -> ()>,
    pub CL_CreateMove: Option<unsafe extern "C" fn(_: libc::c_float,
                                                   _: *mut usercmd_s,
                                                   _: libc::c_int) -> ()>,
    pub CL_IsThirdPerson: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub CL_CameraOffset: Option<unsafe extern "C" fn(_: *mut libc::c_float)
                                    -> ()>,
    pub KB_Find: Option<unsafe extern "C" fn(_: *const libc::c_char)
                            -> *mut libc::c_void>,
    pub CAM_Think: Option<unsafe extern "C" fn() -> ()>,
    pub pfnCalcRefdef: Option<unsafe extern "C" fn(_: *mut ref_params_t)
                                  -> ()>,
    pub pfnAddEntity: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _: *mut cl_entity_t,
                                                  _: *const libc::c_char)
                                 -> libc::c_int>,
    pub pfnCreateEntities: Option<unsafe extern "C" fn() -> ()>,
    pub pfnDrawNormalTriangles: Option<unsafe extern "C" fn() -> ()>,
    pub pfnDrawTransparentTriangles: Option<unsafe extern "C" fn() -> ()>,
    pub pfnStudioEvent: Option<unsafe extern "C" fn(_: *const mstudioevent_s,
                                                    _: *const cl_entity_t)
                                   -> ()>,
    pub pfnPostRunCmd: Option<unsafe extern "C" fn(_: *mut local_state_s,
                                                   _: *mut local_state_s,
                                                   _: *mut usercmd_t,
                                                   _: libc::c_int,
                                                   _: libc::c_double,
                                                   _: libc::c_uint) -> ()>,
    pub pfnShutdown: Option<unsafe extern "C" fn() -> ()>,
    pub pfnTxferLocalOverrides: Option<unsafe extern "C" fn(_:
                                                                *mut entity_state_t,
                                                            _:
                                                                *const clientdata_t)
                                           -> ()>,
    pub pfnProcessPlayerState: Option<unsafe extern "C" fn(_:
                                                               *mut entity_state_t,
                                                           _:
                                                               *const entity_state_t)
                                          -> ()>,
    pub pfnTxferPredictionData: Option<unsafe extern "C" fn(_:
                                                                *mut entity_state_t,
                                                            _:
                                                                *const entity_state_t,
                                                            _:
                                                                *mut clientdata_t,
                                                            _:
                                                                *const clientdata_t,
                                                            _:
                                                                *mut weapon_data_t,
                                                            _:
                                                                *const weapon_data_t)
                                           -> ()>,
    pub pfnDemo_ReadBuffer: Option<unsafe extern "C" fn(_: libc::c_int,
                                                        _: *mut byte) -> ()>,
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
    pub pfnFrame: Option<unsafe extern "C" fn(_: libc::c_double) -> ()>,
    pub pfnKey_Event: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _: libc::c_int,
                                                  _: *const libc::c_char)
                                 -> libc::c_int>,
    pub pfnTempEntUpdate: Option<unsafe extern "C" fn(_: libc::c_double,
                                                      _: libc::c_double,
                                                      _: libc::c_double,
                                                      _: *mut *mut tempent_s,
                                                      _: *mut *mut tempent_s,
                                                      _:
                                                          Option<unsafe extern "C" fn(_:
                                                                                          *mut cl_entity_t)
                                                                     ->
                                                                         libc::c_int>,
                                                      _:
                                                          Option<unsafe extern "C" fn(_:
                                                                                          *mut tempent_s,
                                                                                      _:
                                                                                          libc::c_float)
                                                                     -> ()>)
                                     -> ()>,
    pub pfnGetUserEntity: Option<unsafe extern "C" fn(_: libc::c_int)
                                     -> *mut cl_entity_t>,
    pub pfnVoiceStatus: Option<unsafe extern "C" fn(_: libc::c_int,
                                                    _: qboolean) -> ()>,
    pub pfnDirectorMessage: Option<unsafe extern "C" fn(_: libc::c_int,
                                                        _: *mut libc::c_void)
                                       -> ()>,
    pub pfnGetStudioModelInterface: Option<unsafe extern "C" fn(_:
                                                                    libc::c_int,
                                                                _:
                                                                    *mut *mut r_studio_interface_s,
                                                                _:
                                                                    *mut engine_studio_api_s)
                                               -> libc::c_int>,
    pub pfnChatInputPosition: Option<unsafe extern "C" fn(_: *mut libc::c_int,
                                                          _: *mut libc::c_int)
                                         -> ()>,
    pub pfnGetRenderInterface: Option<unsafe extern "C" fn(_: libc::c_int,
                                                           _:
                                                               *mut render_api_t,
                                                           _:
                                                               *mut render_interface_t)
                                          -> libc::c_int>,
    pub pfnClipMoveToEntity: Option<unsafe extern "C" fn(_: *mut physent_s,
                                                         _: *const vec_t,
                                                         _: *mut vec_t,
                                                         _: *mut vec_t,
                                                         _: *const vec_t,
                                                         _: *mut pmtrace_s)
                                        -> ()>,
    pub pfnTouchEvent: Option<unsafe extern "C" fn(_: libc::c_int,
                                                   _: libc::c_int,
                                                   _: libc::c_float,
                                                   _: libc::c_float,
                                                   _: libc::c_float,
                                                   _: libc::c_float)
                                  -> libc::c_int>,
    pub pfnMoveEvent: Option<unsafe extern "C" fn(_: libc::c_float,
                                                  _: libc::c_float) -> ()>,
    pub pfnLookEvent: Option<unsafe extern "C" fn(_: libc::c_float,
                                                  _: libc::c_float) -> ()>,
}
pub type render_api_t = render_api_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct render_api_s {
    pub RenderGetParm: Option<unsafe extern "C" fn(_: libc::c_int,
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
    pub GetLightStyle: Option<unsafe extern "C" fn(_: libc::c_int)
                                  -> *mut lightstyle_t>,
    pub GetDynamicLight: Option<unsafe extern "C" fn(_: libc::c_int)
                                    -> *mut dlight_t>,
    pub GetEntityLight: Option<unsafe extern "C" fn(_: libc::c_int)
                                   -> *mut dlight_t>,
    pub LightToTexGamma: Option<unsafe extern "C" fn(_: byte) -> byte>,
    pub GetFrameTime: Option<unsafe extern "C" fn() -> libc::c_float>,
    pub R_SetCurrentEntity: Option<unsafe extern "C" fn(_: *mut cl_entity_s)
                                       -> ()>,
    pub R_SetCurrentModel: Option<unsafe extern "C" fn(_: *mut model_s)
                                      -> ()>,
    pub R_FatPVS: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                              _: libc::c_float, _: *mut byte,
                                              _: qboolean, _: qboolean)
                             -> libc::c_int>,
    pub R_StoreEfrags: Option<unsafe extern "C" fn(_: *mut *mut efrag_s,
                                                   _: libc::c_int) -> ()>,
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
    pub AVI_LoadVideo: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                   _: qboolean)
                                  -> *mut libc::c_void>,
    pub AVI_GetVideoInfo: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                      _: *mut libc::c_int,
                                                      _: *mut libc::c_int,
                                                      _: *mut libc::c_float)
                                     -> libc::c_int>,
    pub AVI_GetVideoFrameNumber: Option<unsafe extern "C" fn(_:
                                                                 *mut libc::c_void,
                                                             _: libc::c_float)
                                            -> libc::c_int>,
    pub AVI_GetVideoFrame: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                       _: libc::c_int)
                                      -> *mut byte>,
    pub AVI_UploadRawFrame: Option<unsafe extern "C" fn(_: libc::c_int,
                                                        _: libc::c_int,
                                                        _: libc::c_int,
                                                        _: libc::c_int,
                                                        _: libc::c_int,
                                                        _: *const byte)
                                       -> ()>,
    pub AVI_FreeVideo: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                  -> ()>,
    pub AVI_IsActive: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                 -> libc::c_int>,
    pub AVI_StreamSound: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                     _: libc::c_int,
                                                     _: libc::c_float,
                                                     _: libc::c_float,
                                                     _: libc::c_float) -> ()>,
    pub AVI_Reserved0: Option<unsafe extern "C" fn() -> ()>,
    pub AVI_Reserved1: Option<unsafe extern "C" fn() -> ()>,
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
    pub GL_GetProcAddress: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                      -> *mut libc::c_void>,
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
    pub EnvShot: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                             _: *const libc::c_char,
                                             _: qboolean, _: libc::c_int)
                            -> ()>,
    pub SPR_LoadExt: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                 _: libc::c_uint)
                                -> libc::c_int>,
    pub LightVec: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                              _: *const libc::c_float,
                                              _: *mut libc::c_float,
                                              _: *mut libc::c_float)
                             -> colorVec>,
    pub StudioGetTexture: Option<unsafe extern "C" fn(_: *mut cl_entity_s)
                                     -> *mut mstudiotex_s>,
    pub GetOverviewParms: Option<unsafe extern "C" fn()
                                     -> *const ref_overview_s>,
    pub GetFileByIndex: Option<unsafe extern "C" fn(_: libc::c_int)
                                   -> *const libc::c_char>,
    pub pfnSaveFile: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                 _: *const libc::c_void,
                                                 _: libc::c_int)
                                -> libc::c_int>,
    pub R_Reserved0: Option<unsafe extern "C" fn() -> ()>,
    pub pfnMemAlloc: Option<unsafe extern "C" fn(_: size_t,
                                                 _: *const libc::c_char,
                                                 _: libc::c_int)
                                -> *mut libc::c_void>,
    pub pfnMemFree: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                _: *const libc::c_char,
                                                _: libc::c_int) -> ()>,
    pub pfnGetFilesList: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                     _: *mut libc::c_int,
                                                     _: libc::c_int)
                                    -> *mut *mut libc::c_char>,
    pub pfnFileBufferCRC32: Option<unsafe extern "C" fn(_:
                                                            *const libc::c_void,
                                                        _: libc::c_int)
                                       -> libc::c_uint>,
    pub COM_CompareFileTime: Option<unsafe extern "C" fn(_:
                                                             *const libc::c_char,
                                                         _:
                                                             *const libc::c_char,
                                                         _: *mut libc::c_int)
                                        -> libc::c_int>,
    pub Host_Error: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                _: ...) -> ()>,
    pub pfnGetModel: Option<unsafe extern "C" fn(_: libc::c_int)
                                -> *mut libc::c_void>,
    pub pfnTime: Option<unsafe extern "C" fn() -> libc::c_float>,
    pub Cvar_Set: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                              _: *const libc::c_char) -> ()>,
    pub S_FadeMusicVolume: Option<unsafe extern "C" fn(_: libc::c_float)
                                      -> ()>,
    pub SetRandomSeed: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
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
pub type dlight_t = dlight_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dlight_s {
    pub origin: vec3_t,
    pub radius: libc::c_float,
    pub color: color24,
    pub die: libc::c_float,
    pub decay: libc::c_float,
    pub minlight: libc::c_float,
    pub key: libc::c_int,
    pub dark: qboolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tempent_s {
    pub flags: libc::c_int,
    pub die: libc::c_float,
    pub frameMax: libc::c_float,
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub z: libc::c_float,
    pub fadeSpeed: libc::c_float,
    pub bounceFactor: libc::c_float,
    pub hitSound: libc::c_int,
    pub hitcallback: Option<unsafe extern "C" fn(_: *mut tempent_s,
                                                 _: *mut pmtrace_s) -> ()>,
    pub callback: Option<unsafe extern "C" fn(_: *mut tempent_s,
                                              _: libc::c_float,
                                              _: libc::c_float) -> ()>,
    pub next: *mut tempent_s,
    pub priority: libc::c_int,
    pub clientIndex: libc::c_short,
    pub tentOffset: vec3_t,
    pub entity: cl_entity_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mstudioevent_s {
    pub frame: int32_t,
    pub event: int32_t,
    pub unused: int32_t,
    pub options: [libc::c_char; 64],
}
pub type ref_params_t = ref_params_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ref_params_s {
    pub vieworg: vec3_t,
    pub viewangles: vec3_t,
    pub forward: vec3_t,
    pub right: vec3_t,
    pub up: vec3_t,
    pub frametime: libc::c_float,
    pub time: libc::c_float,
    pub intermission: libc::c_int,
    pub paused: libc::c_int,
    pub spectator: libc::c_int,
    pub onground: libc::c_int,
    pub waterlevel: libc::c_int,
    pub simvel: vec3_t,
    pub simorg: vec3_t,
    pub viewheight: vec3_t,
    pub idealpitch: libc::c_float,
    pub cl_viewangles: vec3_t,
    pub health: libc::c_int,
    pub crosshairangle: vec3_t,
    pub viewsize: libc::c_float,
    pub punchangle: vec3_t,
    pub maxclients: libc::c_int,
    pub viewentity: libc::c_int,
    pub playernum: libc::c_int,
    pub max_entities: libc::c_int,
    pub demoplayback: libc::c_int,
    pub hardware: libc::c_int,
    pub smoothing: libc::c_int,
    pub cmd: *mut usercmd_s,
    pub movevars: *mut movevars_s,
    pub viewport: [libc::c_int; 4],
    pub nextView: libc::c_int,
    pub onlyClientDraw: libc::c_int,
}
pub type client_data_t = client_data_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client_data_s {
    pub origin: vec3_t,
    pub viewangles: vec3_t,
    pub iWeaponBits: libc::c_int,
    pub fov: libc::c_float,
}
pub type cl_enginefunc_t = cl_enginefuncs_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cl_enginefuncs_s {
    pub pfnSPR_Load: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                -> HSPRITE>,
    pub pfnSPR_Frames: Option<unsafe extern "C" fn(_: HSPRITE)
                                  -> libc::c_int>,
    pub pfnSPR_Height: Option<unsafe extern "C" fn(_: HSPRITE, _: libc::c_int)
                                  -> libc::c_int>,
    pub pfnSPR_Width: Option<unsafe extern "C" fn(_: HSPRITE, _: libc::c_int)
                                 -> libc::c_int>,
    pub pfnSPR_Set: Option<unsafe extern "C" fn(_: HSPRITE, _: libc::c_int,
                                                _: libc::c_int,
                                                _: libc::c_int) -> ()>,
    pub pfnSPR_Draw: Option<unsafe extern "C" fn(_: libc::c_int,
                                                 _: libc::c_int,
                                                 _: libc::c_int,
                                                 _: *const wrect_t) -> ()>,
    pub pfnSPR_DrawHoles: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: *const wrect_t)
                                     -> ()>,
    pub pfnSPR_DrawAdditive: Option<unsafe extern "C" fn(_: libc::c_int,
                                                         _: libc::c_int,
                                                         _: libc::c_int,
                                                         _: *const wrect_t)
                                        -> ()>,
    pub pfnSPR_EnableScissor: Option<unsafe extern "C" fn(_: libc::c_int,
                                                          _: libc::c_int,
                                                          _: libc::c_int,
                                                          _: libc::c_int)
                                         -> ()>,
    pub pfnSPR_DisableScissor: Option<unsafe extern "C" fn() -> ()>,
    pub pfnSPR_GetList: Option<unsafe extern "C" fn(_: *mut libc::c_char,
                                                    _: *mut libc::c_int)
                                   -> *mut client_sprite_t>,
    pub pfnFillRGBA: Option<unsafe extern "C" fn(_: libc::c_int,
                                                 _: libc::c_int,
                                                 _: libc::c_int,
                                                 _: libc::c_int,
                                                 _: libc::c_int,
                                                 _: libc::c_int,
                                                 _: libc::c_int,
                                                 _: libc::c_int) -> ()>,
    pub pfnGetScreenInfo: Option<unsafe extern "C" fn(_: *mut SCREENINFO)
                                     -> libc::c_int>,
    pub pfnSetCrosshair: Option<unsafe extern "C" fn(_: HSPRITE, _: wrect_t,
                                                     _: libc::c_int,
                                                     _: libc::c_int,
                                                     _: libc::c_int) -> ()>,
    pub pfnRegisterVariable: Option<unsafe extern "C" fn(_:
                                                             *const libc::c_char,
                                                         _:
                                                             *const libc::c_char,
                                                         _: libc::c_int)
                                        -> *mut cvar_s>,
    pub pfnGetCvarFloat: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                    -> libc::c_float>,
    pub pfnGetCvarString: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                     -> *const libc::c_char>,
    pub pfnAddCommand: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                   _:
                                                       Option<unsafe extern "C" fn()
                                                                  -> ()>)
                                  -> libc::c_int>,
    pub pfnHookUserMsg: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                    _: pfnUserMsgHook)
                                   -> libc::c_int>,
    pub pfnServerCmd: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                 -> libc::c_int>,
    pub pfnClientCmd: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                 -> libc::c_int>,
    pub pfnGetPlayerInfo: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _:
                                                          *mut hud_player_info_t)
                                     -> ()>,
    pub pfnPlaySoundByName: Option<unsafe extern "C" fn(_:
                                                            *const libc::c_char,
                                                        _: libc::c_float)
                                       -> ()>,
    pub pfnPlaySoundByIndex: Option<unsafe extern "C" fn(_: libc::c_int,
                                                         _: libc::c_float)
                                        -> ()>,
    pub pfnAngleVectors: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                     _: *mut libc::c_float,
                                                     _: *mut libc::c_float,
                                                     _: *mut libc::c_float)
                                    -> ()>,
    pub pfnTextMessageGet: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                      -> *mut client_textmessage_t>,
    pub pfnDrawCharacter: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int)
                                     -> libc::c_int>,
    pub pfnDrawConsoleString: Option<unsafe extern "C" fn(_: libc::c_int,
                                                          _: libc::c_int,
                                                          _:
                                                              *mut libc::c_char)
                                         -> libc::c_int>,
    pub pfnDrawSetTextColor: Option<unsafe extern "C" fn(_: libc::c_float,
                                                         _: libc::c_float,
                                                         _: libc::c_float)
                                        -> ()>,
    pub pfnDrawConsoleStringLen: Option<unsafe extern "C" fn(_:
                                                                 *const libc::c_char,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _:
                                                                 *mut libc::c_int)
                                            -> ()>,
    pub pfnConsolePrint: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                    -> ()>,
    pub pfnCenterPrint: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                   -> ()>,
    pub GetWindowCenterX: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub GetWindowCenterY: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub GetViewAngles: Option<unsafe extern "C" fn(_: *mut libc::c_float)
                                  -> ()>,
    pub SetViewAngles: Option<unsafe extern "C" fn(_: *mut libc::c_float)
                                  -> ()>,
    pub GetMaxClients: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub Cvar_SetValue: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                   _: libc::c_float) -> ()>,
    pub Cmd_Argc: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub Cmd_Argv: Option<unsafe extern "C" fn(_: libc::c_int)
                             -> *const libc::c_char>,
    pub Con_Printf: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                _: ...) -> ()>,
    pub Con_DPrintf: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                 _: ...) -> ()>,
    pub Con_NPrintf: Option<unsafe extern "C" fn(_: libc::c_int,
                                                 _: *const libc::c_char,
                                                 _: ...) -> ()>,
    pub Con_NXPrintf: Option<unsafe extern "C" fn(_: *mut con_nprint_s,
                                                  _: *const libc::c_char,
                                                  _: ...) -> ()>,
    pub PhysInfo_ValueForKey: Option<unsafe extern "C" fn(_:
                                                              *const libc::c_char)
                                         -> *const libc::c_char>,
    pub ServerInfo_ValueForKey: Option<unsafe extern "C" fn(_:
                                                                *const libc::c_char)
                                           -> *const libc::c_char>,
    pub GetClientMaxspeed: Option<unsafe extern "C" fn() -> libc::c_float>,
    pub CheckParm: Option<unsafe extern "C" fn(_: *mut libc::c_char,
                                               _: *mut *mut libc::c_char)
                              -> libc::c_int>,
    pub Key_Event: Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int)
                              -> ()>,
    pub GetMousePosition: Option<unsafe extern "C" fn(_: *mut libc::c_int,
                                                      _: *mut libc::c_int)
                                     -> ()>,
    pub IsNoClipping: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub GetLocalPlayer: Option<unsafe extern "C" fn() -> *mut cl_entity_s>,
    pub GetViewModel: Option<unsafe extern "C" fn() -> *mut cl_entity_s>,
    pub GetEntityByIndex: Option<unsafe extern "C" fn(_: libc::c_int)
                                     -> *mut cl_entity_s>,
    pub GetClientTime: Option<unsafe extern "C" fn() -> libc::c_float>,
    pub V_CalcShake: Option<unsafe extern "C" fn() -> ()>,
    pub V_ApplyShake: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                  _: *mut libc::c_float,
                                                  _: libc::c_float) -> ()>,
    pub PM_PointContents: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                      _: *mut libc::c_int)
                                     -> libc::c_int>,
    pub PM_WaterEntity: Option<unsafe extern "C" fn(_: *const libc::c_float)
                                   -> libc::c_int>,
    pub PM_TraceLine: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                  _: *mut libc::c_float,
                                                  _: libc::c_int,
                                                  _: libc::c_int,
                                                  _: libc::c_int)
                                 -> *mut pmtrace_s>,
    pub CL_LoadModel: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                  _: *mut libc::c_int)
                                 -> *mut model_s>,
    pub CL_CreateVisibleEntity: Option<unsafe extern "C" fn(_: libc::c_int,
                                                            _:
                                                                *mut cl_entity_s)
                                           -> libc::c_int>,
    pub GetSpritePointer: Option<unsafe extern "C" fn(_: HSPRITE)
                                     -> *const model_s>,
    pub pfnPlaySoundByNameAtLocation: Option<unsafe extern "C" fn(_:
                                                                      *mut libc::c_char,
                                                                  _:
                                                                      libc::c_float,
                                                                  _:
                                                                      *mut libc::c_float)
                                                 -> ()>,
    pub pfnPrecacheEvent: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: *const libc::c_char)
                                     -> libc::c_ushort>,
    pub pfnPlaybackEvent: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: *const edict_s,
                                                      _: libc::c_ushort,
                                                      _: libc::c_float,
                                                      _: *mut libc::c_float,
                                                      _: *mut libc::c_float,
                                                      _: libc::c_float,
                                                      _: libc::c_float,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int) -> ()>,
    pub pfnWeaponAnim: Option<unsafe extern "C" fn(_: libc::c_int,
                                                   _: libc::c_int) -> ()>,
    pub pfnRandomFloat: Option<unsafe extern "C" fn(_: libc::c_float,
                                                    _: libc::c_float)
                                   -> libc::c_float>,
    pub pfnRandomLong: Option<unsafe extern "C" fn(_: libc::c_int,
                                                   _: libc::c_int)
                                  -> libc::c_int>,
    pub pfnHookEvent: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                  _:
                                                      Option<unsafe extern "C" fn(_:
                                                                                      *mut event_args_s)
                                                                 -> ()>)
                                 -> ()>,
    pub Con_IsVisible: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub pfnGetGameDirectory: Option<unsafe extern "C" fn()
                                        -> *const libc::c_char>,
    pub pfnGetCvarPointer: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                      -> *mut cvar_s>,
    pub Key_LookupBinding: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                      -> *const libc::c_char>,
    pub pfnGetLevelName: Option<unsafe extern "C" fn()
                                    -> *const libc::c_char>,
    pub pfnGetScreenFade: Option<unsafe extern "C" fn(_: *mut screenfade_s)
                                     -> ()>,
    pub pfnSetScreenFade: Option<unsafe extern "C" fn(_: *mut screenfade_s)
                                     -> ()>,
    pub VGui_GetPanel: Option<unsafe extern "C" fn() -> *mut libc::c_void>,
    pub VGui_ViewportPaintBackground: Option<unsafe extern "C" fn(_:
                                                                      *mut libc::c_int)
                                                 -> ()>,
    pub COM_LoadFile: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                  _: libc::c_int,
                                                  _: *mut libc::c_int)
                                 -> *mut byte>,
    pub COM_ParseFile: Option<unsafe extern "C" fn(_: *mut libc::c_char,
                                                   _: *mut libc::c_char)
                                  -> *mut libc::c_char>,
    pub COM_FreeFile: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                 -> ()>,
    pub pTriAPI: *mut triangleapi_s,
    pub pEfxAPI: *mut efx_api_s,
    pub pEventAPI: *mut event_api_s,
    pub pDemoAPI: *mut demo_api_s,
    pub pNetAPI: *mut net_api_s,
    pub pVoiceTweak: *mut IVoiceTweak_s,
    pub IsSpectateOnly: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub LoadMapSprite: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                  -> *mut model_s>,
    pub COM_AddAppDirectoryToSearchPath: Option<unsafe extern "C" fn(_:
                                                                         *const libc::c_char,
                                                                     _:
                                                                         *const libc::c_char)
                                                    -> ()>,
    pub COM_ExpandFilename: Option<unsafe extern "C" fn(_:
                                                            *const libc::c_char,
                                                        _: *mut libc::c_char,
                                                        _: libc::c_int)
                                       -> libc::c_int>,
    pub PlayerInfo_ValueForKey: Option<unsafe extern "C" fn(_: libc::c_int,
                                                            _:
                                                                *const libc::c_char)
                                           -> *const libc::c_char>,
    pub PlayerInfo_SetValueForKey: Option<unsafe extern "C" fn(_:
                                                                   *const libc::c_char,
                                                               _:
                                                                   *const libc::c_char)
                                              -> ()>,
    pub GetPlayerUniqueID: Option<unsafe extern "C" fn(_: libc::c_int,
                                                       _: *mut libc::c_char)
                                      -> qboolean>,
    pub GetTrackerIDForPlayer: Option<unsafe extern "C" fn(_: libc::c_int)
                                          -> libc::c_int>,
    pub GetPlayerForTrackerID: Option<unsafe extern "C" fn(_: libc::c_int)
                                          -> libc::c_int>,
    pub pfnServerCmdUnreliable: Option<unsafe extern "C" fn(_:
                                                                *mut libc::c_char)
                                           -> libc::c_int>,
    pub pfnGetMousePos: Option<unsafe extern "C" fn(_: *mut tagPOINT) -> ()>,
    pub pfnSetMousePos: Option<unsafe extern "C" fn(_: libc::c_int,
                                                    _: libc::c_int) -> ()>,
    pub pfnSetMouseEnable: Option<unsafe extern "C" fn(_: qboolean) -> ()>,
    pub pfnGetFirstCvarPtr: Option<unsafe extern "C" fn() -> *mut cvar_s>,
    pub pfnGetFirstCmdFunctionHandle: Option<unsafe extern "C" fn()
                                                 -> *mut libc::c_void>,
    pub pfnGetNextCmdFunctionHandle: Option<unsafe extern "C" fn(_:
                                                                     *mut libc::c_void)
                                                -> *mut libc::c_void>,
    pub pfnGetCmdFunctionName: Option<unsafe extern "C" fn(_:
                                                               *mut libc::c_void)
                                          -> *const libc::c_char>,
    pub pfnGetClientOldTime: Option<unsafe extern "C" fn() -> libc::c_float>,
    pub pfnGetGravity: Option<unsafe extern "C" fn() -> libc::c_float>,
    pub pfnGetModelByIndex: Option<unsafe extern "C" fn(_: libc::c_int)
                                       -> *mut model_s>,
    pub pfnSetFilterMode: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub pfnSetFilterColor: Option<unsafe extern "C" fn(_: libc::c_float,
                                                       _: libc::c_float,
                                                       _: libc::c_float)
                                      -> ()>,
    pub pfnSetFilterBrightness: Option<unsafe extern "C" fn(_: libc::c_float)
                                           -> ()>,
    pub pfnSequenceGet: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                    _: *const libc::c_char)
                                   -> *mut libc::c_void>,
    pub pfnSPR_DrawGeneric: Option<unsafe extern "C" fn(_: libc::c_int,
                                                        _: libc::c_int,
                                                        _: libc::c_int,
                                                        _: *const wrect_t,
                                                        _: libc::c_int,
                                                        _: libc::c_int,
                                                        _: libc::c_int,
                                                        _: libc::c_int)
                                       -> ()>,
    pub pfnSequencePickSentence: Option<unsafe extern "C" fn(_:
                                                                 *const libc::c_char,
                                                             _: libc::c_int,
                                                             _:
                                                                 *mut libc::c_int)
                                            -> *mut libc::c_void>,
    pub pfnDrawString: Option<unsafe extern "C" fn(_: libc::c_int,
                                                   _: libc::c_int,
                                                   _: *const libc::c_char,
                                                   _: libc::c_int,
                                                   _: libc::c_int,
                                                   _: libc::c_int)
                                  -> libc::c_int>,
    pub pfnDrawStringReverse: Option<unsafe extern "C" fn(_: libc::c_int,
                                                          _: libc::c_int,
                                                          _:
                                                              *const libc::c_char,
                                                          _: libc::c_int,
                                                          _: libc::c_int,
                                                          _: libc::c_int)
                                         -> libc::c_int>,
    pub LocalPlayerInfo_ValueForKey: Option<unsafe extern "C" fn(_:
                                                                     *const libc::c_char)
                                                -> *const libc::c_char>,
    pub pfnVGUI2DrawCharacter: Option<unsafe extern "C" fn(_: libc::c_int,
                                                           _: libc::c_int,
                                                           _: libc::c_int,
                                                           _: libc::c_uint)
                                          -> libc::c_int>,
    pub pfnVGUI2DrawCharacterAdditive: Option<unsafe extern "C" fn(_:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_uint)
                                                  -> libc::c_int>,
    pub pfnGetApproxWavePlayLen: Option<unsafe extern "C" fn(_:
                                                                 *const libc::c_char)
                                            -> libc::c_uint>,
    pub GetCareerGameUI: Option<unsafe extern "C" fn() -> *mut libc::c_void>,
    pub Cvar_Set: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                              _: *const libc::c_char) -> ()>,
    pub pfnIsPlayingCareerMatch: Option<unsafe extern "C" fn()
                                            -> libc::c_int>,
    pub pfnPlaySoundVoiceByName: Option<unsafe extern "C" fn(_:
                                                                 *mut libc::c_char,
                                                             _: libc::c_float,
                                                             _: libc::c_int)
                                            -> ()>,
    pub pfnPrimeMusicStream: Option<unsafe extern "C" fn(_: *mut libc::c_char,
                                                         _: libc::c_int)
                                        -> ()>,
    pub pfnSys_FloatTime: Option<unsafe extern "C" fn() -> libc::c_double>,
    pub pfnProcessTutorMessageDecayBuffer: Option<unsafe extern "C" fn(_:
                                                                           *mut libc::c_int,
                                                                       _:
                                                                           libc::c_int)
                                                      -> ()>,
    pub pfnConstructTutorMessageDecayBuffer: Option<unsafe extern "C" fn(_:
                                                                             *mut libc::c_int,
                                                                         _:
                                                                             libc::c_int)
                                                        -> ()>,
    pub pfnResetTutorMessageDecayData: Option<unsafe extern "C" fn() -> ()>,
    pub pfnPlaySoundByNameAtPitch: Option<unsafe extern "C" fn(_:
                                                                   *mut libc::c_char,
                                                               _:
                                                                   libc::c_float,
                                                               _: libc::c_int)
                                              -> ()>,
    pub pfnFillRGBABlend: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int) -> ()>,
    pub pfnGetAppID: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub pfnGetAliases: Option<unsafe extern "C" fn() -> *mut cmdalias_t>,
    pub pfnVguiWrap2_GetMouseDelta: Option<unsafe extern "C" fn(_:
                                                                    *mut libc::c_int,
                                                                _:
                                                                    *mut libc::c_int)
                                               -> ()>,
    pub pfnFilteredClientCmd: Option<unsafe extern "C" fn(_:
                                                              *const libc::c_char)
                                         -> libc::c_int>,
}
pub type cmdalias_t = cmdalias_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct net_api_s {
    pub InitNetworking: Option<unsafe extern "C" fn() -> ()>,
    pub Status: Option<unsafe extern "C" fn(_: *mut net_status_s) -> ()>,
    pub SendRequest: Option<unsafe extern "C" fn(_: libc::c_int,
                                                 _: libc::c_int,
                                                 _: libc::c_int,
                                                 _: libc::c_double,
                                                 _: *mut netadr_s,
                                                 _: net_api_response_func_t)
                                -> ()>,
    pub CancelRequest: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub CancelAllRequests: Option<unsafe extern "C" fn() -> ()>,
    pub AdrToString: Option<unsafe extern "C" fn(_: *mut netadr_s)
                                -> *const libc::c_char>,
    pub CompareAdr: Option<unsafe extern "C" fn(_: *mut netadr_s,
                                                _: *mut netadr_s)
                               -> libc::c_int>,
    pub StringToAdr: Option<unsafe extern "C" fn(_: *mut libc::c_char,
                                                 _: *mut netadr_s)
                                -> libc::c_int>,
    pub ValueForKey: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                 _: *const libc::c_char)
                                -> *const libc::c_char>,
    pub RemoveKey: Option<unsafe extern "C" fn(_: *mut libc::c_char,
                                               _: *const libc::c_char) -> ()>,
    pub SetValueForKey: Option<unsafe extern "C" fn(_: *mut libc::c_char,
                                                    _: *const libc::c_char,
                                                    _: *const libc::c_char,
                                                    _: libc::c_int) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct net_status_s {
    pub connected: libc::c_int,
    pub local_address: netadr_t,
    pub remote_address: netadr_t,
    pub packet_loss: libc::c_int,
    pub latency: libc::c_double,
    pub connection_time: libc::c_double,
    pub rate: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_api_s {
    pub version: libc::c_int,
    pub EV_PlaySound: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _: *mut libc::c_float,
                                                  _: libc::c_int,
                                                  _: *const libc::c_char,
                                                  _: libc::c_float,
                                                  _: libc::c_float,
                                                  _: libc::c_int,
                                                  _: libc::c_int) -> ()>,
    pub EV_StopSound: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _: libc::c_int,
                                                  _: *const libc::c_char)
                                 -> ()>,
    pub EV_FindModelIndex: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                      -> libc::c_int>,
    pub EV_IsLocal: Option<unsafe extern "C" fn(_: libc::c_int)
                               -> libc::c_int>,
    pub EV_LocalPlayerDucking: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub EV_LocalPlayerViewheight: Option<unsafe extern "C" fn(_:
                                                                  *mut libc::c_float)
                                             -> ()>,
    pub EV_LocalPlayerBounds: Option<unsafe extern "C" fn(_: libc::c_int,
                                                          _:
                                                              *mut libc::c_float,
                                                          _:
                                                              *mut libc::c_float)
                                         -> ()>,
    pub EV_IndexFromTrace: Option<unsafe extern "C" fn(_: *mut pmtrace_s)
                                      -> libc::c_int>,
    pub EV_GetPhysent: Option<unsafe extern "C" fn(_: libc::c_int)
                                  -> *mut physent_s>,
    pub EV_SetUpPlayerPrediction: Option<unsafe extern "C" fn(_: libc::c_int,
                                                              _: libc::c_int)
                                             -> ()>,
    pub EV_PushPMStates: Option<unsafe extern "C" fn() -> ()>,
    pub EV_PopPMStates: Option<unsafe extern "C" fn() -> ()>,
    pub EV_SetSolidPlayers: Option<unsafe extern "C" fn(_: libc::c_int)
                                       -> ()>,
    pub EV_SetTraceHull: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub EV_PlayerTrace: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                    _: *mut libc::c_float,
                                                    _: libc::c_int,
                                                    _: libc::c_int,
                                                    _: *mut pmtrace_s) -> ()>,
    pub EV_WeaponAnimation: Option<unsafe extern "C" fn(_: libc::c_int,
                                                        _: libc::c_int)
                                       -> ()>,
    pub EV_PrecacheEvent: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: *const libc::c_char)
                                     -> libc::c_ushort>,
    pub EV_PlaybackEvent: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: *const edict_s,
                                                      _: libc::c_ushort,
                                                      _: libc::c_float,
                                                      _: *mut libc::c_float,
                                                      _: *mut libc::c_float,
                                                      _: libc::c_float,
                                                      _: libc::c_float,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int) -> ()>,
    pub EV_TraceTexture: Option<unsafe extern "C" fn(_: libc::c_int,
                                                     _: *mut libc::c_float,
                                                     _: *mut libc::c_float)
                                    -> *const libc::c_char>,
    pub EV_StopAllSounds: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: libc::c_int) -> ()>,
    pub EV_KillEvents: Option<unsafe extern "C" fn(_: libc::c_int,
                                                   _: *const libc::c_char)
                                  -> ()>,
    pub EV_PlayerTraceExt: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                       _: *mut libc::c_float,
                                                       _: libc::c_int,
                                                       _:
                                                           Option<unsafe extern "C" fn(_:
                                                                                           *mut physent_s)
                                                                      ->
                                                                          libc::c_int>,
                                                       _: *mut pmtrace_s)
                                      -> ()>,
    pub EV_SoundForIndex: Option<unsafe extern "C" fn(_: libc::c_int)
                                     -> *const libc::c_char>,
    pub EV_TraceSurface: Option<unsafe extern "C" fn(_: libc::c_int,
                                                     _: *mut libc::c_float,
                                                     _: *mut libc::c_float)
                                    -> *mut msurface_s>,
    pub EV_GetMovevars: Option<unsafe extern "C" fn() -> *mut movevars_s>,
    pub EV_VisTraceLine: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                     _: *mut libc::c_float,
                                                     _: libc::c_int)
                                    -> *mut pmtrace_s>,
    pub EV_GetVisent: Option<unsafe extern "C" fn(_: libc::c_int)
                                 -> *mut physent_s>,
    pub EV_TestLine: Option<unsafe extern "C" fn(_: *const vec_t,
                                                 _: *const vec_t,
                                                 _: libc::c_int)
                                -> libc::c_int>,
    pub EV_PushTraceBounds: Option<unsafe extern "C" fn(_: libc::c_int,
                                                        _:
                                                            *const libc::c_float,
                                                        _:
                                                            *const libc::c_float)
                                       -> ()>,
    pub EV_PopTraceBounds: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct efx_api_s {
    pub R_AllocParticle: Option<unsafe extern "C" fn(_:
                                                         Option<unsafe extern "C" fn(_:
                                                                                         *mut particle_s,
                                                                                     _:
                                                                                         libc::c_float)
                                                                    -> ()>)
                                    -> *mut particle_t>,
    pub R_BlobExplosion: Option<unsafe extern "C" fn(_: *const libc::c_float)
                                    -> ()>,
    pub R_Blood: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                             _: *const libc::c_float,
                                             _: libc::c_int, _: libc::c_int)
                            -> ()>,
    pub R_BloodSprite: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                   _: libc::c_int,
                                                   _: libc::c_int,
                                                   _: libc::c_int,
                                                   _: libc::c_float) -> ()>,
    pub R_BloodStream: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                   _: *const libc::c_float,
                                                   _: libc::c_int,
                                                   _: libc::c_int) -> ()>,
    pub R_BreakModel: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                  _: *const libc::c_float,
                                                  _: *const libc::c_float,
                                                  _: libc::c_float,
                                                  _: libc::c_float,
                                                  _: libc::c_int,
                                                  _: libc::c_int,
                                                  _: libc::c_char) -> ()>,
    pub R_Bubbles: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                               _: *const libc::c_float,
                                               _: libc::c_float,
                                               _: libc::c_int, _: libc::c_int,
                                               _: libc::c_float) -> ()>,
    pub R_BubbleTrail: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                   _: *const libc::c_float,
                                                   _: libc::c_float,
                                                   _: libc::c_int,
                                                   _: libc::c_int,
                                                   _: libc::c_float) -> ()>,
    pub R_BulletImpactParticles: Option<unsafe extern "C" fn(_:
                                                                 *const libc::c_float)
                                            -> ()>,
    pub R_EntityParticles: Option<unsafe extern "C" fn(_: *mut cl_entity_s)
                                      -> ()>,
    pub R_Explosion: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                 _: libc::c_int,
                                                 _: libc::c_float,
                                                 _: libc::c_float,
                                                 _: libc::c_int) -> ()>,
    pub R_FizzEffect: Option<unsafe extern "C" fn(_: *mut cl_entity_s,
                                                  _: libc::c_int,
                                                  _: libc::c_int) -> ()>,
    pub R_FireField: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                 _: libc::c_int,
                                                 _: libc::c_int,
                                                 _: libc::c_int,
                                                 _: libc::c_int,
                                                 _: libc::c_float) -> ()>,
    pub R_FlickerParticles: Option<unsafe extern "C" fn(_:
                                                            *const libc::c_float)
                                       -> ()>,
    pub R_FunnelSprite: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                    _: libc::c_int,
                                                    _: libc::c_int) -> ()>,
    pub R_Implosion: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                 _: libc::c_float,
                                                 _: libc::c_int,
                                                 _: libc::c_float) -> ()>,
    pub R_LargeFunnel: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                   _: libc::c_int) -> ()>,
    pub R_LavaSplash: Option<unsafe extern "C" fn(_: *const libc::c_float)
                                 -> ()>,
    pub R_MultiGunshot: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                    _: *const libc::c_float,
                                                    _: *const libc::c_float,
                                                    _: libc::c_int,
                                                    _: libc::c_int,
                                                    _: *mut libc::c_int)
                                   -> ()>,
    pub R_MuzzleFlash: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                   _: libc::c_int) -> ()>,
    pub R_ParticleBox: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                   _: *const libc::c_float,
                                                   _: libc::c_uchar,
                                                   _: libc::c_uchar,
                                                   _: libc::c_uchar,
                                                   _: libc::c_float) -> ()>,
    pub R_ParticleBurst: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                     _: libc::c_int,
                                                     _: libc::c_int,
                                                     _: libc::c_float) -> ()>,
    pub R_ParticleExplosion: Option<unsafe extern "C" fn(_:
                                                             *const libc::c_float)
                                        -> ()>,
    pub R_ParticleExplosion2: Option<unsafe extern "C" fn(_:
                                                              *const libc::c_float,
                                                          _: libc::c_int,
                                                          _: libc::c_int)
                                         -> ()>,
    pub R_ParticleLine: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                    _: *const libc::c_float,
                                                    _: libc::c_uchar,
                                                    _: libc::c_uchar,
                                                    _: libc::c_uchar,
                                                    _: libc::c_float) -> ()>,
    pub R_PlayerSprites: Option<unsafe extern "C" fn(_: libc::c_int,
                                                     _: libc::c_int,
                                                     _: libc::c_int,
                                                     _: libc::c_int) -> ()>,
    pub R_Projectile: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                  _: *const libc::c_float,
                                                  _: libc::c_int,
                                                  _: libc::c_int,
                                                  _: libc::c_int,
                                                  _:
                                                      Option<unsafe extern "C" fn(_:
                                                                                      *mut tempent_s,
                                                                                  _:
                                                                                      *mut pmtrace_s)
                                                                 -> ()>)
                                 -> ()>,
    pub R_RicochetSound: Option<unsafe extern "C" fn(_: *const libc::c_float)
                                    -> ()>,
    pub R_RicochetSprite: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                      _: *mut model_s,
                                                      _: libc::c_float,
                                                      _: libc::c_float)
                                     -> ()>,
    pub R_RocketFlare: Option<unsafe extern "C" fn(_: *const libc::c_float)
                                  -> ()>,
    pub R_RocketTrail: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                   _: *mut libc::c_float,
                                                   _: libc::c_int) -> ()>,
    pub R_RunParticleEffect: Option<unsafe extern "C" fn(_:
                                                             *const libc::c_float,
                                                         _:
                                                             *const libc::c_float,
                                                         _: libc::c_int,
                                                         _: libc::c_int)
                                        -> ()>,
    pub R_ShowLine: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                _: *const libc::c_float)
                               -> ()>,
    pub R_SparkEffect: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                   _: libc::c_int,
                                                   _: libc::c_int,
                                                   _: libc::c_int) -> ()>,
    pub R_SparkShower: Option<unsafe extern "C" fn(_: *const libc::c_float)
                                  -> ()>,
    pub R_SparkStreaks: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                    _: libc::c_int,
                                                    _: libc::c_int,
                                                    _: libc::c_int) -> ()>,
    pub R_Spray: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                             _: *const libc::c_float,
                                             _: libc::c_int, _: libc::c_int,
                                             _: libc::c_int, _: libc::c_int,
                                             _: libc::c_int) -> ()>,
    pub R_Sprite_Explode: Option<unsafe extern "C" fn(_: *mut TEMPENTITY,
                                                      _: libc::c_float,
                                                      _: libc::c_int) -> ()>,
    pub R_Sprite_Smoke: Option<unsafe extern "C" fn(_: *mut TEMPENTITY,
                                                    _: libc::c_float) -> ()>,
    pub R_Sprite_Spray: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                    _: *const libc::c_float,
                                                    _: libc::c_int,
                                                    _: libc::c_int,
                                                    _: libc::c_int,
                                                    _: libc::c_int) -> ()>,
    pub R_Sprite_Trail: Option<unsafe extern "C" fn(_: libc::c_int,
                                                    _: *mut libc::c_float,
                                                    _: *mut libc::c_float,
                                                    _: libc::c_int,
                                                    _: libc::c_int,
                                                    _: libc::c_float,
                                                    _: libc::c_float,
                                                    _: libc::c_float,
                                                    _: libc::c_int,
                                                    _: libc::c_float) -> ()>,
    pub R_Sprite_WallPuff: Option<unsafe extern "C" fn(_: *mut TEMPENTITY,
                                                       _: libc::c_float)
                                      -> ()>,
    pub R_StreakSplash: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                    _: *const libc::c_float,
                                                    _: libc::c_int,
                                                    _: libc::c_int,
                                                    _: libc::c_float,
                                                    _: libc::c_int,
                                                    _: libc::c_int) -> ()>,
    pub R_TracerEffect: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                    _: *const libc::c_float)
                                   -> ()>,
    pub R_UserTracerParticle: Option<unsafe extern "C" fn(_:
                                                              *mut libc::c_float,
                                                          _:
                                                              *mut libc::c_float,
                                                          _: libc::c_float,
                                                          _: libc::c_int,
                                                          _: libc::c_float,
                                                          _: libc::c_uchar,
                                                          _:
                                                              Option<unsafe extern "C" fn(_:
                                                                                              *mut particle_s)
                                                                         ->
                                                                             ()>)
                                         -> ()>,
    pub R_TracerParticles: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                       _: *mut libc::c_float,
                                                       _: libc::c_float)
                                      -> *mut particle_t>,
    pub R_TeleportSplash: Option<unsafe extern "C" fn(_: *const libc::c_float)
                                     -> ()>,
    pub R_TempSphereModel: Option<unsafe extern "C" fn(_:
                                                           *const libc::c_float,
                                                       _: libc::c_float,
                                                       _: libc::c_float,
                                                       _: libc::c_int,
                                                       _: libc::c_int) -> ()>,
    pub R_TempModel: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                 _: *const libc::c_float,
                                                 _: *const libc::c_float,
                                                 _: libc::c_float,
                                                 _: libc::c_int,
                                                 _: libc::c_int)
                                -> *mut TEMPENTITY>,
    pub R_DefaultSprite: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                     _: libc::c_int,
                                                     _: libc::c_float)
                                    -> *mut TEMPENTITY>,
    pub R_TempSprite: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                  _: *const libc::c_float,
                                                  _: libc::c_float,
                                                  _: libc::c_int,
                                                  _: libc::c_int,
                                                  _: libc::c_int,
                                                  _: libc::c_float,
                                                  _: libc::c_float,
                                                  _: libc::c_int)
                                 -> *mut TEMPENTITY>,
    pub Draw_DecalIndex: Option<unsafe extern "C" fn(_: libc::c_int)
                                    -> libc::c_int>,
    pub Draw_DecalIndexFromName: Option<unsafe extern "C" fn(_:
                                                                 *const libc::c_char)
                                            -> libc::c_int>,
    pub R_DecalShoot: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _: libc::c_int,
                                                  _: libc::c_int,
                                                  _: *mut libc::c_float,
                                                  _: libc::c_int) -> ()>,
    pub R_AttachTentToPlayer: Option<unsafe extern "C" fn(_: libc::c_int,
                                                          _: libc::c_int,
                                                          _: libc::c_float,
                                                          _: libc::c_float)
                                         -> ()>,
    pub R_KillAttachedTents: Option<unsafe extern "C" fn(_: libc::c_int)
                                        -> ()>,
    pub R_BeamCirclePoints: Option<unsafe extern "C" fn(_: libc::c_int,
                                                        _: *mut libc::c_float,
                                                        _: *mut libc::c_float,
                                                        _: libc::c_int,
                                                        _: libc::c_float,
                                                        _: libc::c_float,
                                                        _: libc::c_float,
                                                        _: libc::c_float,
                                                        _: libc::c_float,
                                                        _: libc::c_int,
                                                        _: libc::c_float,
                                                        _: libc::c_float,
                                                        _: libc::c_float,
                                                        _: libc::c_float)
                                       -> *mut BEAM>,
    pub R_BeamEntPoint: Option<unsafe extern "C" fn(_: libc::c_int,
                                                    _: *mut libc::c_float,
                                                    _: libc::c_int,
                                                    _: libc::c_float,
                                                    _: libc::c_float,
                                                    _: libc::c_float,
                                                    _: libc::c_float,
                                                    _: libc::c_float,
                                                    _: libc::c_int,
                                                    _: libc::c_float,
                                                    _: libc::c_float,
                                                    _: libc::c_float,
                                                    _: libc::c_float)
                                   -> *mut BEAM>,
    pub R_BeamEnts: Option<unsafe extern "C" fn(_: libc::c_int,
                                                _: libc::c_int,
                                                _: libc::c_int,
                                                _: libc::c_float,
                                                _: libc::c_float,
                                                _: libc::c_float,
                                                _: libc::c_float,
                                                _: libc::c_float,
                                                _: libc::c_int,
                                                _: libc::c_float,
                                                _: libc::c_float,
                                                _: libc::c_float,
                                                _: libc::c_float)
                               -> *mut BEAM>,
    pub R_BeamFollow: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _: libc::c_int,
                                                  _: libc::c_float,
                                                  _: libc::c_float,
                                                  _: libc::c_float,
                                                  _: libc::c_float,
                                                  _: libc::c_float,
                                                  _: libc::c_float)
                                 -> *mut BEAM>,
    pub R_BeamKill: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub R_BeamLightning: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                     _: *mut libc::c_float,
                                                     _: libc::c_int,
                                                     _: libc::c_float,
                                                     _: libc::c_float,
                                                     _: libc::c_float,
                                                     _: libc::c_float,
                                                     _: libc::c_float)
                                    -> *mut BEAM>,
    pub R_BeamPoints: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                  _: *mut libc::c_float,
                                                  _: libc::c_int,
                                                  _: libc::c_float,
                                                  _: libc::c_float,
                                                  _: libc::c_float,
                                                  _: libc::c_float,
                                                  _: libc::c_float,
                                                  _: libc::c_int,
                                                  _: libc::c_float,
                                                  _: libc::c_float,
                                                  _: libc::c_float,
                                                  _: libc::c_float)
                                 -> *mut BEAM>,
    pub R_BeamRing: Option<unsafe extern "C" fn(_: libc::c_int,
                                                _: libc::c_int,
                                                _: libc::c_int,
                                                _: libc::c_float,
                                                _: libc::c_float,
                                                _: libc::c_float,
                                                _: libc::c_float,
                                                _: libc::c_float,
                                                _: libc::c_int,
                                                _: libc::c_float,
                                                _: libc::c_float,
                                                _: libc::c_float,
                                                _: libc::c_float)
                               -> *mut BEAM>,
    pub CL_AllocDlight: Option<unsafe extern "C" fn(_: libc::c_int)
                                   -> *mut dlight_t>,
    pub CL_AllocElight: Option<unsafe extern "C" fn(_: libc::c_int)
                                   -> *mut dlight_t>,
    pub CL_TempEntAlloc: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                     _: *mut model_s)
                                    -> *mut TEMPENTITY>,
    pub CL_TempEntAllocNoModel: Option<unsafe extern "C" fn(_:
                                                                *const libc::c_float)
                                           -> *mut TEMPENTITY>,
    pub CL_TempEntAllocHigh: Option<unsafe extern "C" fn(_:
                                                             *const libc::c_float,
                                                         _: *mut model_s)
                                        -> *mut TEMPENTITY>,
    pub CL_TentEntAllocCustom: Option<unsafe extern "C" fn(_:
                                                               *const libc::c_float,
                                                           _: *mut model_s,
                                                           _: libc::c_int,
                                                           _:
                                                               Option<unsafe extern "C" fn(_:
                                                                                               *mut tempent_s,
                                                                                           _:
                                                                                               libc::c_float,
                                                                                           _:
                                                                                               libc::c_float)
                                                                          ->
                                                                              ()>)
                                          -> *mut TEMPENTITY>,
    pub R_GetPackedColor: Option<unsafe extern "C" fn(_: *mut libc::c_short,
                                                      _: libc::c_short)
                                     -> ()>,
    pub R_LookupColor: Option<unsafe extern "C" fn(_: libc::c_uchar,
                                                   _: libc::c_uchar,
                                                   _: libc::c_uchar)
                                  -> libc::c_short>,
    pub R_DecalRemoveAll: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub R_FireCustomDecal: Option<unsafe extern "C" fn(_: libc::c_int,
                                                       _: libc::c_int,
                                                       _: libc::c_int,
                                                       _: *mut libc::c_float,
                                                       _: libc::c_int,
                                                       _: libc::c_float)
                                      -> ()>,
}
pub type TEMPENTITY = tempent_s;
pub type BEAM = beam_s;
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
pub type particle_t = particle_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct triangleapi_s {
    pub version: libc::c_int,
    pub RenderMode: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
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
    pub Brightness: Option<unsafe extern "C" fn(_: libc::c_float) -> ()>,
    pub CullFace: Option<unsafe extern "C" fn(_: TRICULLSTYLE) -> ()>,
    pub SpriteTexture: Option<unsafe extern "C" fn(_: *mut model_s,
                                                   _: libc::c_int)
                                  -> libc::c_int>,
    pub WorldToScreen: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                   _: *mut libc::c_float)
                                  -> libc::c_int>,
    pub Fog: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                         _: libc::c_float, _: libc::c_float,
                                         _: libc::c_int) -> ()>,
    pub ScreenToWorld: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                   _: *mut libc::c_float)
                                  -> ()>,
    pub GetMatrix: Option<unsafe extern "C" fn(_: libc::c_int,
                                               _: *mut libc::c_float) -> ()>,
    pub BoxInPVS: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                              _: *mut libc::c_float)
                             -> libc::c_int>,
    pub LightAtPoint: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                  _: *mut libc::c_float)
                                 -> ()>,
    pub Color4fRendermode: Option<unsafe extern "C" fn(_: libc::c_float,
                                                       _: libc::c_float,
                                                       _: libc::c_float,
                                                       _: libc::c_float,
                                                       _: libc::c_int) -> ()>,
    pub FogParams: Option<unsafe extern "C" fn(_: libc::c_float,
                                               _: libc::c_int) -> ()>,
}
pub type HSPRITE = libc::c_int;
pub type hud_player_info_t = hud_player_info_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hud_player_info_s {
    pub name: *mut libc::c_char,
    pub ping: libc::c_short,
    pub thisplayer: byte,
    pub spectator: byte,
    pub packetloss: byte,
    pub model: *mut libc::c_char,
    pub topcolor: libc::c_short,
    pub bottomcolor: libc::c_short,
    pub m_nSteamID: uint64_t,
}
static mut delta_init: qboolean = false_0;
// list of all the struct names
// Initialized in run_static_initializers
static mut cmd_fields: [delta_field_t; 17] =
    [delta_field_t{name: 0 as *const libc::c_char, offset: 0, size: 0,}; 17];
// Initialized in run_static_initializers
static mut pm_fields: [delta_field_t; 32] =
    [delta_field_t{name: 0 as *const libc::c_char, offset: 0, size: 0,}; 32];
// Initialized in run_static_initializers
static mut ev_fields: [delta_field_t; 19] =
    [delta_field_t{name: 0 as *const libc::c_char, offset: 0, size: 0,}; 19];
// Initialized in run_static_initializers
static mut wd_fields: [delta_field_t; 23] =
    [delta_field_t{name: 0 as *const libc::c_char, offset: 0, size: 0,}; 23];
// Initialized in run_static_initializers
static mut cd_fields: [delta_field_t; 57] =
    [delta_field_t{name: 0 as *const libc::c_char, offset: 0, size: 0,}; 57];
// Initialized in run_static_initializers
static mut ent_fields: [delta_field_t; 92] =
    [delta_field_t{name: 0 as *const libc::c_char, offset: 0, size: 0,}; 92];
// Initialized in run_static_initializers
static mut dt_info: [delta_info_t; 9] =
    [delta_info_t{pName: 0 as *const libc::c_char,
                  pInfo: 0 as *const delta_field_t,
                  maxFields: 0,
                  numFields: 0,
                  pFields: 0 as *mut delta_t,
                  customEncode: 0,
                  funcName: [0; 32],
                  userCallback: None,
                  bInitialized: false_0,}; 9];
#[no_mangle]
pub unsafe extern "C" fn Delta_FindStruct(mut name: *const libc::c_char)
 -> *mut delta_info_t {
    let mut i: libc::c_int = 0;
    if if name.is_null() || *name == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return 0 as *mut delta_info_t
    }
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[delta_info_t; 9]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<delta_info_t>()
                                                   as
                                                   libc::c_ulong).wrapping_sub(1
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_ulong)
          {
        if Q_strnicmp(dt_info[i as usize].pName, name, 99999 as libc::c_int)
               == 0 {
            return &mut *dt_info.as_mut_ptr().offset(i as isize) as
                       *mut delta_info_t
        }
        i += 1
    }
    Con_DPrintf(b"^3Warning:^7 Struct %s not found in delta_info\n\x00" as
                    *const u8 as *const libc::c_char, name);
    // found nothing
    return 0 as *mut delta_info_t;
}
#[no_mangle]
pub unsafe extern "C" fn Delta_NumTables() -> libc::c_int {
    return (::std::mem::size_of::<[delta_info_t; 9]>() as
                libc::c_ulong).wrapping_div(::std::mem::size_of::<delta_info_t>()
                                                as
                                                libc::c_ulong).wrapping_sub(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong)
               as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Delta_FindStructByIndex(mut index: libc::c_int)
 -> *mut delta_info_t {
    if index < 0 as libc::c_int ||
           index as libc::c_ulong >=
               (::std::mem::size_of::<[delta_info_t; 9]>() as
                    libc::c_ulong).wrapping_div(::std::mem::size_of::<delta_info_t>()
                                                    as
                                                    libc::c_ulong).wrapping_sub(1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong)
       {
        return 0 as *mut delta_info_t
    }
    return &mut *dt_info.as_mut_ptr().offset(index as isize) as
               *mut delta_info_t;
}
#[no_mangle]
pub unsafe extern "C" fn Delta_FindStructByEncoder(mut encoderName:
                                                       *const libc::c_char)
 -> *mut delta_info_t {
    let mut i: libc::c_int = 0;
    if if encoderName.is_null() || *encoderName == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return 0 as *mut delta_info_t
    }
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[delta_info_t; 9]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<delta_info_t>()
                                                   as
                                                   libc::c_ulong).wrapping_sub(1
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_ulong)
          {
        if Q_strnicmp(dt_info[i as usize].funcName.as_mut_ptr(), encoderName,
                      99999 as libc::c_int) == 0 {
            return &mut *dt_info.as_mut_ptr().offset(i as isize) as
                       *mut delta_info_t
        }
        i += 1
    }
    // found nothing
    return 0 as *mut delta_info_t;
}
#[no_mangle]
pub unsafe extern "C" fn Delta_FindStructByDelta(mut pFields: *const delta_t)
 -> *mut delta_info_t {
    let mut i: libc::c_int = 0;
    if pFields.is_null() { return 0 as *mut delta_info_t }
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[delta_info_t; 9]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<delta_info_t>()
                                                   as
                                                   libc::c_ulong).wrapping_sub(1
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_ulong)
          {
        if dt_info[i as usize].pFields == pFields as *mut delta_t {
            return &mut *dt_info.as_mut_ptr().offset(i as isize) as
                       *mut delta_info_t
        }
        i += 1
    }
    // found nothing
    return 0 as *mut delta_info_t;
}
#[no_mangle]
pub unsafe extern "C" fn Delta_CustomEncode(mut dt: *mut delta_info_t,
                                            mut from: *const libc::c_void,
                                            mut to: *const libc::c_void) {
    let mut i: libc::c_int = 0;
    // set all fields is active by default
    i = 0 as libc::c_int;
    while i < (*dt).numFields {
        (*(*dt).pFields.offset(i as isize)).bInactive = false_0;
        i += 1
    }
    if (*dt).userCallback.is_some() {
        (*dt).userCallback.expect("non-null function pointer")((*dt).pFields,
                                                               from as
                                                                   *const byte,
                                                               to as
                                                                   *const byte);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Delta_FindFieldInfo(mut pInfo: *const delta_field_t,
                                             mut fieldName:
                                                 *const libc::c_char)
 -> *mut delta_field_t {
    if fieldName.is_null() || *fieldName == 0 {
        return 0 as *mut delta_field_t
    }
    while !(*pInfo).name.is_null() {
        if Q_strncmp((*pInfo).name, fieldName, 99999 as libc::c_int) == 0 {
            return pInfo as *mut delta_field_t
        }
        pInfo = pInfo.offset(1)
    }
    return 0 as *mut delta_field_t;
}
#[no_mangle]
pub unsafe extern "C" fn Delta_IndexForFieldInfo(mut pInfo:
                                                     *const delta_field_t,
                                                 mut fieldName:
                                                     *const libc::c_char)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    if fieldName.is_null() || *fieldName == 0 { return -(1 as libc::c_int) }
    i = 0 as libc::c_int;
    while !(*pInfo).name.is_null() {
        if Q_strncmp((*pInfo).name, fieldName, 99999 as libc::c_int) == 0 {
            return i
        }
        i += 1;
        pInfo = pInfo.offset(1)
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Delta_AddField(mut pStructName: *const libc::c_char,
                                        mut pName: *const libc::c_char,
                                        mut flags: libc::c_int,
                                        mut bits: libc::c_int,
                                        mut mul: libc::c_float,
                                        mut post_mul: libc::c_float)
 -> qboolean {
    let mut dt: *mut delta_info_t = 0 as *mut delta_info_t;
    let mut pFieldInfo: *mut delta_field_t = 0 as *mut delta_field_t;
    let mut pField: *mut delta_t = 0 as *mut delta_t;
    let mut i: libc::c_int = 0;
    // get the delta struct
    dt = Delta_FindStruct(pStructName);
    // check for coexisting field
    i = 0 as libc::c_int;
    pField = (*dt).pFields;
    while i < (*dt).numFields {
        if Q_strncmp((*pField).name, pName, 99999 as libc::c_int) == 0 {
            // update existed field
            (*pField).flags = flags;
            (*pField).bits = bits;
            (*pField).multiplier = mul;
            (*pField).post_multiplier = post_mul;
            return true_0
        }
        i += 1;
        pField = pField.offset(1)
    }
    // find field description
    pFieldInfo = Delta_FindFieldInfo((*dt).pInfo, pName);
    if pFieldInfo.is_null() {
        Con_DPrintf(b"^1Error:^7 Delta_Add: couldn\'t find description for %s->%s\n\x00"
                        as *const u8 as *const libc::c_char, pStructName,
                    pName);
        return false_0
    }
    if (*dt).numFields + 1 as libc::c_int > (*dt).maxFields {
        Con_DPrintf(b"^3Warning:^7 Delta_Add: can\'t add %s->%s encoder list is full\n\x00"
                        as *const u8 as *const libc::c_char, pStructName,
                    pName);
        return false_0
        // too many fields specified (duplicated ?)
    }
    // allocate a new one
    (*dt).pFields =
        _Mem_Realloc(host.mempool, (*dt).pFields as *mut libc::c_void,
                     (((*dt).numFields + 1 as libc::c_int) as
                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<delta_t>()
                                                          as libc::c_ulong),
                     true_0,
                     b"../engine/common/net_encode.c\x00" as *const u8 as
                         *const libc::c_char, 458 as libc::c_int) as
            *mut delta_t;
    i = 0 as libc::c_int;
    pField = (*dt).pFields;
    while i < (*dt).numFields { i += 1; pField = pField.offset(1) }
    // copy info to new field
    (*pField).name = (*pFieldInfo).name; // not initialized ?
    (*pField).offset =
        (*pFieldInfo).offset; // assume we support 16 network tables
    (*pField).size =
        (*pFieldInfo).size; // 255 fields by struct should be enough
    (*pField).flags = flags; // flags are indicated various input types
    (*pField).bits = bits; // max received value is 32 (32 bit)
    (*pField).multiplier = mul;
    (*pField).post_multiplier = post_mul;
    (*dt).numFields += 1;
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn Delta_WriteTableField(mut msg: *mut sizebuf_t,
                                               mut tableIndex: libc::c_int,
                                               mut pField: *const delta_t) {
    let mut nameIndex: libc::c_int = 0;
    let mut dt: *mut delta_info_t = 0 as *mut delta_info_t;
    if if (*pField).name.is_null() || *(*pField).name == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return
    }
    dt = Delta_FindStructByIndex(tableIndex);
    nameIndex = Delta_IndexForFieldInfo((*dt).pInfo, (*pField).name);
    MSG_WriteCmdExt(msg, 14 as libc::c_int, NS_SERVER,
                    0 as *const libc::c_char);
    MSG_WriteUBitLong(msg, tableIndex as uint, 4 as libc::c_int);
    MSG_WriteUBitLong(msg, nameIndex as uint, 8 as libc::c_int);
    MSG_WriteUBitLong(msg, (*pField).flags as uint, 10 as libc::c_int);
    MSG_WriteUBitLong(msg, ((*pField).bits - 1 as libc::c_int) as uint,
                      5 as libc::c_int);
    // multipliers is null-compressed
    if !((*pField).multiplier > 1.0f32 - 0.001f32 &&
             (*pField).multiplier < 1.0f32 + 0.001f32) {
        MSG_WriteOneBit(msg, 1 as libc::c_int); // read field name index
        MSG_WriteFloat(msg, (*pField).multiplier);
    } else { MSG_WriteOneBit(msg, 0 as libc::c_int); }
    if !((*pField).post_multiplier > 1.0f32 - 0.001f32 &&
             (*pField).post_multiplier < 1.0f32 + 0.001f32) {
        MSG_WriteOneBit(msg, 1 as libc::c_int);
        MSG_WriteFloat(msg, (*pField).post_multiplier);
    } else { MSG_WriteOneBit(msg, 0 as libc::c_int); };
}
#[no_mangle]
pub unsafe extern "C" fn Delta_ParseTableField(mut msg: *mut sizebuf_t) {
    let mut tableIndex: libc::c_int = 0;
    let mut nameIndex: libc::c_int = 0;
    let mut mul: libc::c_float = 1.0f32;
    let mut post_mul: libc::c_float = 1.0f32;
    let mut flags: libc::c_int = 0;
    let mut bits: libc::c_int = 0;
    let mut pName: *const libc::c_char = 0 as *const libc::c_char;
    let mut ignore: qboolean = false_0;
    let mut dt: *mut delta_info_t = 0 as *mut delta_info_t;
    tableIndex = MSG_ReadUBitLong(msg, 4 as libc::c_int) as libc::c_int;
    dt = Delta_FindStructByIndex(tableIndex);
    if dt.is_null() {
        Host_Error(b"Delta_ParseTableField: not initialized\x00" as *const u8
                       as *const libc::c_char);
    }
    nameIndex = MSG_ReadUBitLong(msg, 8 as libc::c_int) as libc::c_int;
    if nameIndex >= 0 as libc::c_int && nameIndex < (*dt).maxFields {
        pName = (*(*dt).pInfo.offset(nameIndex as isize)).name
    } else {
        ignore = true_0;
        Con_Reportf(b"Delta_ParseTableField: wrong nameIndex %d for table %s, ignoring\n\x00"
                        as *const u8 as *const libc::c_char, nameIndex,
                    (*dt).pName);
    }
    flags = MSG_ReadUBitLong(msg, 10 as libc::c_int) as libc::c_int;
    bits =
        MSG_ReadUBitLong(msg,
                         5 as
                             libc::c_int).wrapping_add(1 as libc::c_int as
                                                           libc::c_uint) as
            libc::c_int;
    // read the multipliers
    if MSG_ReadOneBit(msg) != 0 { mul = MSG_ReadFloat(msg) }
    if MSG_ReadOneBit(msg) != 0 { post_mul = MSG_ReadFloat(msg) }
    if ignore as u64 != 0 { return }
    // delta encoders it's already initialized on this machine (local game)
    if delta_init as u64 != 0 { Delta_Shutdown(); }
    // add field to table
    Delta_AddField((*dt).pName, pName, flags, bits, mul, post_mul);
}
#[no_mangle]
pub unsafe extern "C" fn Delta_ParseField(mut delta_script:
                                              *mut *mut libc::c_char,
                                          mut pInfo: *const delta_field_t,
                                          mut pField: *mut delta_t,
                                          mut bPost: qboolean) -> qboolean {
    let mut token: string = [0; 256];
    let mut pFieldInfo: *mut delta_field_t = 0 as *mut delta_field_t;
    let mut oldpos: *mut libc::c_char = 0 as *mut libc::c_char;
    *delta_script =
        _COM_ParseFileSafe(*delta_script, token.as_mut_ptr(),
                           ::std::mem::size_of::<string>() as libc::c_ulong as
                               libc::c_int, 0 as libc::c_int as libc::c_uint,
                           0 as *mut libc::c_int);
    if Q_strncmp(token.as_mut_ptr(),
                 b"(\x00" as *const u8 as *const libc::c_char,
                 99999 as libc::c_int) != 0 {
        Con_DPrintf(b"^1Error:^7 Delta_ParseField: expected \'(\', found \'%s\' instead\n\x00"
                        as *const u8 as *const libc::c_char,
                    token.as_mut_ptr());
        return false_0
    }
    // read the variable name
    *delta_script =
        _COM_ParseFileSafe(*delta_script, token.as_mut_ptr(),
                           ::std::mem::size_of::<string>() as libc::c_ulong as
                               libc::c_int, 0 as libc::c_int as libc::c_uint,
                           0 as *mut libc::c_int);
    if (*delta_script).is_null() {
        Con_DPrintf(b"^1Error:^7 Delta_ParseField: missing field name\n\x00"
                        as *const u8 as *const libc::c_char);
        return false_0
    }
    pFieldInfo = Delta_FindFieldInfo(pInfo, token.as_mut_ptr());
    if pFieldInfo.is_null() {
        Con_DPrintf(b"^1Error:^7 Delta_ParseField: unable to find field %s\n\x00"
                        as *const u8 as *const libc::c_char,
                    token.as_mut_ptr());
        return false_0
    }
    *delta_script =
        _COM_ParseFileSafe(*delta_script, token.as_mut_ptr(),
                           ::std::mem::size_of::<string>() as libc::c_ulong as
                               libc::c_int, 0 as libc::c_int as libc::c_uint,
                           0 as *mut libc::c_int);
    if Q_strncmp(token.as_mut_ptr(),
                 b",\x00" as *const u8 as *const libc::c_char,
                 99999 as libc::c_int) != 0 {
        Con_DPrintf(b"^1Error:^7 Delta_ParseField: expected \',\', found \'%s\' instead\n\x00"
                        as *const u8 as *const libc::c_char,
                    token.as_mut_ptr());
        return false_0
    }
    // copy base info to new field
    (*pField).name = (*pFieldInfo).name;
    (*pField).offset = (*pFieldInfo).offset;
    (*pField).size = (*pFieldInfo).size;
    (*pField).flags = 0 as libc::c_int;
    loop 
         // read delta-flags
         {
        *delta_script =
            _COM_ParseFileSafe(*delta_script, token.as_mut_ptr(),
                               ::std::mem::size_of::<string>() as
                                   libc::c_ulong as libc::c_int,
                               0 as libc::c_int as libc::c_uint,
                               0 as
                                   *mut libc::c_int); // end of flags argument
        if (*delta_script).is_null() { break ; }
        if Q_strncmp(token.as_mut_ptr(),
                     b",\x00" as *const u8 as *const libc::c_char,
                     99999 as libc::c_int) == 0 {
            break ;
        }
        if Q_strncmp(token.as_mut_ptr(),
                     b"|\x00" as *const u8 as *const libc::c_char,
                     99999 as libc::c_int) == 0 {
            continue ;
        }
        if Q_strncmp(token.as_mut_ptr(),
                     b"DT_BYTE\x00" as *const u8 as *const libc::c_char,
                     99999 as libc::c_int) == 0 {
            (*pField).flags =
                ((*pField).flags as libc::c_uint |
                     (1 as libc::c_uint) << 0 as libc::c_int) as libc::c_int
        } else if Q_strncmp(token.as_mut_ptr(),
                            b"DT_SHORT\x00" as *const u8 as
                                *const libc::c_char, 99999 as libc::c_int) ==
                      0 {
            (*pField).flags =
                ((*pField).flags as libc::c_uint |
                     (1 as libc::c_uint) << 1 as libc::c_int) as libc::c_int
        } else if Q_strncmp(token.as_mut_ptr(),
                            b"DT_FLOAT\x00" as *const u8 as
                                *const libc::c_char, 99999 as libc::c_int) ==
                      0 {
            (*pField).flags =
                ((*pField).flags as libc::c_uint |
                     (1 as libc::c_uint) << 2 as libc::c_int) as libc::c_int
        } else if Q_strncmp(token.as_mut_ptr(),
                            b"DT_INTEGER\x00" as *const u8 as
                                *const libc::c_char, 99999 as libc::c_int) ==
                      0 {
            (*pField).flags =
                ((*pField).flags as libc::c_uint |
                     (1 as libc::c_uint) << 3 as libc::c_int) as libc::c_int
        } else if Q_strncmp(token.as_mut_ptr(),
                            b"DT_ANGLE\x00" as *const u8 as
                                *const libc::c_char, 99999 as libc::c_int) ==
                      0 {
            (*pField).flags =
                ((*pField).flags as libc::c_uint |
                     (1 as libc::c_uint) << 4 as libc::c_int) as libc::c_int
        } else if Q_strncmp(token.as_mut_ptr(),
                            b"DT_TIMEWINDOW_8\x00" as *const u8 as
                                *const libc::c_char, 99999 as libc::c_int) ==
                      0 {
            (*pField).flags =
                ((*pField).flags as libc::c_uint |
                     (1 as libc::c_uint) << 5 as libc::c_int) as libc::c_int
        } else if Q_strncmp(token.as_mut_ptr(),
                            b"DT_TIMEWINDOW_BIG\x00" as *const u8 as
                                *const libc::c_char, 99999 as libc::c_int) ==
                      0 {
            (*pField).flags =
                ((*pField).flags as libc::c_uint |
                     (1 as libc::c_uint) << 6 as libc::c_int) as libc::c_int
        } else if Q_strncmp(token.as_mut_ptr(),
                            b"DT_STRING\x00" as *const u8 as
                                *const libc::c_char, 99999 as libc::c_int) ==
                      0 {
            (*pField).flags =
                ((*pField).flags as libc::c_uint |
                     (1 as libc::c_uint) << 7 as libc::c_int) as libc::c_int
        } else if Q_strncmp(token.as_mut_ptr(),
                            b"DT_SIGNED\x00" as *const u8 as
                                *const libc::c_char, 99999 as libc::c_int) ==
                      0 {
            (*pField).flags =
                ((*pField).flags as libc::c_uint |
                     (1 as libc::c_uint) << 8 as libc::c_int) as libc::c_int
        }
    }
    if Q_strncmp(token.as_mut_ptr(),
                 b",\x00" as *const u8 as *const libc::c_char,
                 99999 as libc::c_int) != 0 {
        Con_DPrintf(b"^1Error:^7 Delta_ParseField: expected \',\', found \'%s\' instead\n\x00"
                        as *const u8 as *const libc::c_char,
                    token.as_mut_ptr());
        return false_0
    }
    // read delta-bits
    *delta_script =
        _COM_ParseFileSafe(*delta_script, token.as_mut_ptr(),
                           ::std::mem::size_of::<string>() as libc::c_ulong as
                               libc::c_int, 0 as libc::c_int as libc::c_uint,
                           0 as *mut libc::c_int);
    if (*delta_script).is_null() {
        Con_DPrintf(b"^1Error:^7 Delta_ReadField: %s field bits argument is missing\n\x00"
                        as *const u8 as *const libc::c_char, (*pField).name);
        return false_0
    }
    (*pField).bits = Q_atoi(token.as_mut_ptr());
    *delta_script =
        _COM_ParseFileSafe(*delta_script, token.as_mut_ptr(),
                           ::std::mem::size_of::<string>() as libc::c_ulong as
                               libc::c_int, 0 as libc::c_int as libc::c_uint,
                           0 as *mut libc::c_int);
    if Q_strncmp(token.as_mut_ptr(),
                 b",\x00" as *const u8 as *const libc::c_char,
                 99999 as libc::c_int) != 0 {
        Con_DPrintf(b"^1Error:^7 Delta_ReadField: expected \',\', found \'%s\' instead\n\x00"
                        as *const u8 as *const libc::c_char,
                    token.as_mut_ptr());
        return false_0
    }
    // read delta-multiplier
    *delta_script =
        _COM_ParseFileSafe(*delta_script, token.as_mut_ptr(),
                           ::std::mem::size_of::<string>() as libc::c_ulong as
                               libc::c_int, 0 as libc::c_int as libc::c_uint,
                           0 as *mut libc::c_int);
    if (*delta_script).is_null() {
        Con_DPrintf(b"^1Error:^7 Delta_ReadField: %s missing \'multiplier\' argument\n\x00"
                        as *const u8 as *const libc::c_char, (*pField).name);
        return false_0
    }
    (*pField).multiplier = Q_atof(token.as_mut_ptr());
    if bPost as u64 != 0 {
        *delta_script =
            _COM_ParseFileSafe(*delta_script, token.as_mut_ptr(),
                               ::std::mem::size_of::<string>() as
                                   libc::c_ulong as libc::c_int,
                               0 as libc::c_int as libc::c_uint,
                               0 as *mut libc::c_int);
        if Q_strncmp(token.as_mut_ptr(),
                     b",\x00" as *const u8 as *const libc::c_char,
                     99999 as libc::c_int) != 0 {
            Con_DPrintf(b"^1Error:^7 Delta_ReadField: expected \',\', found \'%s\' instead\n\x00"
                            as *const u8 as *const libc::c_char,
                        token.as_mut_ptr());
            return false_0
        }
        // read delta-postmultiplier
        *delta_script =
            _COM_ParseFileSafe(*delta_script, token.as_mut_ptr(),
                               ::std::mem::size_of::<string>() as
                                   libc::c_ulong as libc::c_int,
                               0 as libc::c_int as libc::c_uint,
                               0 as *mut libc::c_int);
        if (*delta_script).is_null() {
            Con_DPrintf(b"^1Error:^7 Delta_ReadField: %s missing \'post_multiply\' argument\n\x00"
                            as *const u8 as *const libc::c_char,
                        (*pField).name);
            return false_0
        }
        (*pField).post_multiplier = Q_atof(token.as_mut_ptr())
    } else {
        // to avoid division by zero
        (*pField).post_multiplier = 1.0f32
    }
    // closing brace...
    *delta_script =
        _COM_ParseFileSafe(*delta_script, token.as_mut_ptr(),
                           ::std::mem::size_of::<string>() as libc::c_ulong as
                               libc::c_int, 0 as libc::c_int as libc::c_uint,
                           0 as *mut libc::c_int);
    if Q_strncmp(token.as_mut_ptr(),
                 b")\x00" as *const u8 as *const libc::c_char,
                 99999 as libc::c_int) != 0 {
        Con_DPrintf(b"^1Error:^7 Delta_ParseField: expected \')\', found \'%s\' instead\n\x00"
                        as *const u8 as *const libc::c_char,
                    token.as_mut_ptr());
        return false_0
    }
    // ... and trying to parse optional ',' post-symbol
    oldpos = *delta_script; // not a ','
    *delta_script =
        _COM_ParseFileSafe(*delta_script, token.as_mut_ptr(),
                           ::std::mem::size_of::<string>() as libc::c_ulong as
                               libc::c_int, 0 as libc::c_int as libc::c_uint,
                           0 as *mut libc::c_int);
    if token[0 as libc::c_int as usize] as libc::c_int != ',' as i32 {
        *delta_script = oldpos
    }
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn Delta_ParseTable(mut delta_script:
                                              *mut *mut libc::c_char,
                                          mut dt: *mut delta_info_t,
                                          mut encodeDll: *const libc::c_char,
                                          mut encodeFunc:
                                              *const libc::c_char) {
    let mut token: string = [0; 256];
    let mut pField: *mut delta_t = 0 as *mut delta_t;
    let mut pInfo: *const delta_field_t = 0 as *const delta_field_t;
    // allocate the delta-structures
    if (*dt).pFields.is_null() {
        (*dt).pFields =
            _Mem_Alloc(host.mempool,
                       ((*dt).maxFields as
                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<delta_t>()
                                                            as libc::c_ulong),
                       true_0,
                       b"../engine/common/net_encode.c\x00" as *const u8 as
                           *const libc::c_char, 705 as libc::c_int) as
                *mut delta_t
    }
    pField = (*dt).pFields;
    pInfo = (*dt).pInfo;
    (*dt).numFields = 0 as libc::c_int;
    loop 
         // assume we have handled '{'
         {
        *delta_script =
            _COM_ParseFileSafe(*delta_script, token.as_mut_ptr(),
                               ::std::mem::size_of::<string>() as
                                   libc::c_ulong as libc::c_int,
                               0 as libc::c_int as libc::c_uint,
                               0 as *mut libc::c_int);
        if (*delta_script).is_null() { break ; }
        if Q_strncmp(token.as_mut_ptr(),
                     b"DEFINE_DELTA\x00" as *const u8 as *const libc::c_char,
                     99999 as libc::c_int) == 0 {
            if Delta_ParseField(delta_script, pInfo,
                                &mut *pField.offset((*dt).numFields as isize),
                                false_0) as u64 != 0 {
                (*dt).numFields += 1
            }
        } else if Q_strncmp(token.as_mut_ptr(),
                            b"DEFINE_DELTA_POST\x00" as *const u8 as
                                *const libc::c_char, 99999 as libc::c_int) ==
                      0 {
            if Delta_ParseField(delta_script, pInfo,
                                &mut *pField.offset((*dt).numFields as isize),
                                true_0) as u64 != 0 {
                (*dt).numFields += 1
            }
        } else if token[0 as libc::c_int as usize] as libc::c_int ==
                      '}' as i32 {
            break ;
        }
    }
    // copy function name
    Q_strncpy((*dt).funcName.as_mut_ptr(), encodeFunc,
              ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong);
    if Q_strnicmp(encodeDll, b"none\x00" as *const u8 as *const libc::c_char,
                  99999 as libc::c_int) == 0 {
        (*dt).customEncode = CUSTOM_NONE as libc::c_int
    } else if Q_strnicmp(encodeDll,
                         b"gamedll\x00" as *const u8 as *const libc::c_char,
                         99999 as libc::c_int) == 0 {
        (*dt).customEncode = CUSTOM_SERVER_ENCODE as libc::c_int
    } else if Q_strnicmp(encodeDll,
                         b"clientdll\x00" as *const u8 as *const libc::c_char,
                         99999 as libc::c_int) == 0 {
        (*dt).customEncode = CUSTOM_CLIENT_ENCODE as libc::c_int
    }
    // adjust to fit memory size
    if (*dt).numFields < (*dt).maxFields {
        (*dt).pFields =
            _Mem_Realloc(host.mempool, (*dt).pFields as *mut libc::c_void,
                         ((*dt).numFields as
                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<delta_t>()
                                                              as
                                                              libc::c_ulong),
                         true_0,
                         b"../engine/common/net_encode.c\x00" as *const u8 as
                             *const libc::c_char, 746 as libc::c_int) as
                *mut delta_t
    }
    (*dt).bInitialized = true_0;
    // table is ok
}
#[no_mangle]
pub unsafe extern "C" fn Delta_InitFields() {
    let mut afile: *mut byte = 0 as *mut byte;
    let mut pfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut encodeDll: string = [0; 256];
    let mut encodeFunc: string = [0; 256];
    let mut token: string = [0; 256];
    let mut dt: *mut delta_info_t = 0 as *mut delta_info_t;
    afile =
        FS_LoadFile(b"delta.lst\x00" as *const u8 as *const libc::c_char,
                    0 as *mut fs_offset_t, false_0);
    if afile.is_null() {
        Sys_Error(b"DELTA_Load: couldn\'t load file %s\n\x00" as *const u8 as
                      *const libc::c_char,
                  b"delta.lst\x00" as *const u8 as *const libc::c_char);
    }
    pfile = afile as *mut libc::c_char;
    loop  {
        pfile =
            _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                               ::std::mem::size_of::<string>() as
                                   libc::c_ulong as libc::c_int,
                               0 as libc::c_int as libc::c_uint,
                               0 as *mut libc::c_int);
        if pfile.is_null() { break ; }
        dt = Delta_FindStruct(token.as_mut_ptr());
        if dt.is_null() {
            Sys_Error(b"%s: unknown struct %s\n\x00" as *const u8 as
                          *const libc::c_char,
                      b"delta.lst\x00" as *const u8 as *const libc::c_char,
                      token.as_mut_ptr());
        }
        pfile =
            _COM_ParseFileSafe(pfile, encodeDll.as_mut_ptr(),
                               ::std::mem::size_of::<string>() as
                                   libc::c_ulong as libc::c_int,
                               0 as libc::c_int as libc::c_uint,
                               0 as *mut libc::c_int);
        if Q_strnicmp(encodeDll.as_mut_ptr(),
                      b"none\x00" as *const u8 as *const libc::c_char,
                      99999 as libc::c_int) == 0 {
            Q_strncpy(encodeFunc.as_mut_ptr(),
                      b"null\x00" as *const u8 as *const libc::c_char,
                      99999 as libc::c_int as size_t);
        } else {
            pfile =
                _COM_ParseFileSafe(pfile, encodeFunc.as_mut_ptr(),
                                   ::std::mem::size_of::<string>() as
                                       libc::c_ulong as libc::c_int,
                                   0 as libc::c_int as libc::c_uint,
                                   0 as *mut libc::c_int)
        }
        // jump to '{'
        pfile =
            _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                               ::std::mem::size_of::<string>() as
                                   libc::c_ulong as libc::c_int,
                               0 as libc::c_int as libc::c_uint,
                               0 as *mut libc::c_int);
        if token[0 as libc::c_int as usize] as libc::c_int != '{' as i32 {
            Sys_Error(b"%s: missing \'{\' in section %s\n\x00" as *const u8 as
                          *const libc::c_char,
                      b"delta.lst\x00" as *const u8 as *const libc::c_char,
                      (*dt).pName);
        }
        Delta_ParseTable(&mut pfile, dt, encodeDll.as_mut_ptr(),
                         encodeFunc.as_mut_ptr());
    }
    _Mem_Free(afile as *mut libc::c_void,
              b"../engine/common/net_encode.c\x00" as *const u8 as
                  *const libc::c_char, 790 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Delta_Init() {
    let mut dt: *mut delta_info_t = 0 as *mut delta_info_t;
    // shutdown it first
    if delta_init as u64 != 0 {
        Delta_Shutdown(); // initialize fields
    } // "movevars_t" already specified by user
    Delta_InitFields();
    delta_init = true_0;
    dt =
        Delta_FindStruct(b"movevars_t\x00" as *const u8 as
                             *const libc::c_char);
    if (*dt).bInitialized as u64 != 0 { return }
    // create movevars_t delta internal
    Delta_AddField(b"movevars_t\x00" as *const u8 as *const libc::c_char,
                   b"gravity\x00" as *const u8 as *const libc::c_char,
                   ((1 as libc::c_uint) << 2 as libc::c_int |
                        (1 as libc::c_uint) << 8 as libc::c_int) as
                       libc::c_int, 16 as libc::c_int, 8.0f32,
                   1.0f32); // 0 - 264
    Delta_AddField(b"movevars_t\x00" as *const u8 as *const libc::c_char,
                   b"stopspeed\x00" as *const u8 as *const libc::c_char,
                   ((1 as libc::c_uint) << 2 as libc::c_int |
                        (1 as libc::c_uint) << 8 as libc::c_int) as
                       libc::c_int, 16 as libc::c_int, 8.0f32,
                   1.0f32); // 0 - 1
    Delta_AddField(b"movevars_t\x00" as *const u8 as *const libc::c_char,
                   b"maxspeed\x00" as *const u8 as *const libc::c_char,
                   ((1 as libc::c_uint) << 2 as libc::c_int |
                        (1 as libc::c_uint) << 8 as libc::c_int) as
                       libc::c_int, 16 as libc::c_int, 8.0f32, 1.0f32);
    Delta_AddField(b"movevars_t\x00" as *const u8 as *const libc::c_char,
                   b"spectatormaxspeed\x00" as *const u8 as
                       *const libc::c_char,
                   ((1 as libc::c_uint) << 2 as libc::c_int |
                        (1 as libc::c_uint) << 8 as libc::c_int) as
                       libc::c_int, 16 as libc::c_int, 8.0f32, 1.0f32);
    Delta_AddField(b"movevars_t\x00" as *const u8 as *const libc::c_char,
                   b"accelerate\x00" as *const u8 as *const libc::c_char,
                   ((1 as libc::c_uint) << 2 as libc::c_int |
                        (1 as libc::c_uint) << 8 as libc::c_int) as
                       libc::c_int, 16 as libc::c_int, 8.0f32, 1.0f32);
    Delta_AddField(b"movevars_t\x00" as *const u8 as *const libc::c_char,
                   b"airaccelerate\x00" as *const u8 as *const libc::c_char,
                   ((1 as libc::c_uint) << 2 as libc::c_int |
                        (1 as libc::c_uint) << 8 as libc::c_int) as
                       libc::c_int, 16 as libc::c_int, 8.0f32, 1.0f32);
    Delta_AddField(b"movevars_t\x00" as *const u8 as *const libc::c_char,
                   b"wateraccelerate\x00" as *const u8 as *const libc::c_char,
                   ((1 as libc::c_uint) << 2 as libc::c_int |
                        (1 as libc::c_uint) << 8 as libc::c_int) as
                       libc::c_int, 16 as libc::c_int, 8.0f32, 1.0f32);
    Delta_AddField(b"movevars_t\x00" as *const u8 as *const libc::c_char,
                   b"friction\x00" as *const u8 as *const libc::c_char,
                   ((1 as libc::c_uint) << 2 as libc::c_int |
                        (1 as libc::c_uint) << 8 as libc::c_int) as
                       libc::c_int, 16 as libc::c_int, 8.0f32, 1.0f32);
    Delta_AddField(b"movevars_t\x00" as *const u8 as *const libc::c_char,
                   b"edgefriction\x00" as *const u8 as *const libc::c_char,
                   ((1 as libc::c_uint) << 2 as libc::c_int |
                        (1 as libc::c_uint) << 8 as libc::c_int) as
                       libc::c_int, 16 as libc::c_int, 8.0f32, 1.0f32);
    Delta_AddField(b"movevars_t\x00" as *const u8 as *const libc::c_char,
                   b"waterfriction\x00" as *const u8 as *const libc::c_char,
                   ((1 as libc::c_uint) << 2 as libc::c_int |
                        (1 as libc::c_uint) << 8 as libc::c_int) as
                       libc::c_int, 16 as libc::c_int, 8.0f32, 1.0f32);
    Delta_AddField(b"movevars_t\x00" as *const u8 as *const libc::c_char,
                   b"bounce\x00" as *const u8 as *const libc::c_char,
                   ((1 as libc::c_uint) << 2 as libc::c_int |
                        (1 as libc::c_uint) << 8 as libc::c_int) as
                       libc::c_int, 16 as libc::c_int, 8.0f32, 1.0f32);
    Delta_AddField(b"movevars_t\x00" as *const u8 as *const libc::c_char,
                   b"stepsize\x00" as *const u8 as *const libc::c_char,
                   ((1 as libc::c_uint) << 2 as libc::c_int |
                        (1 as libc::c_uint) << 8 as libc::c_int) as
                       libc::c_int, 16 as libc::c_int, 16.0f32, 1.0f32);
    Delta_AddField(b"movevars_t\x00" as *const u8 as *const libc::c_char,
                   b"maxvelocity\x00" as *const u8 as *const libc::c_char,
                   ((1 as libc::c_uint) << 2 as libc::c_int |
                        (1 as libc::c_uint) << 8 as libc::c_int) as
                       libc::c_int, 16 as libc::c_int, 8.0f32, 1.0f32);
    if host.features &
           ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint != 0 {
        Delta_AddField(b"movevars_t\x00" as *const u8 as *const libc::c_char,
                       b"zmax\x00" as *const u8 as *const libc::c_char,
                       ((1 as libc::c_uint) << 2 as libc::c_int |
                            (1 as libc::c_uint) << 8 as libc::c_int) as
                           libc::c_int, 18 as libc::c_int, 1.0f32, 1.0f32);
    } else {
        Delta_AddField(b"movevars_t\x00" as *const u8 as *const libc::c_char,
                       b"zmax\x00" as *const u8 as *const libc::c_char,
                       ((1 as libc::c_uint) << 2 as libc::c_int |
                            (1 as libc::c_uint) << 8 as libc::c_int) as
                           libc::c_int, 16 as libc::c_int, 1.0f32, 1.0f32);
    }
    Delta_AddField(b"movevars_t\x00" as *const u8 as *const libc::c_char,
                   b"waveHeight\x00" as *const u8 as *const libc::c_char,
                   ((1 as libc::c_uint) << 2 as libc::c_int |
                        (1 as libc::c_uint) << 8 as libc::c_int) as
                       libc::c_int, 16 as libc::c_int, 16.0f32, 1.0f32);
    Delta_AddField(b"movevars_t\x00" as *const u8 as *const libc::c_char,
                   b"skyName\x00" as *const u8 as *const libc::c_char,
                   ((1 as libc::c_uint) << 7 as libc::c_int) as libc::c_int,
                   1 as libc::c_int, 1.0f32, 1.0f32);
    Delta_AddField(b"movevars_t\x00" as *const u8 as *const libc::c_char,
                   b"footsteps\x00" as *const u8 as *const libc::c_char,
                   ((1 as libc::c_uint) << 3 as libc::c_int) as libc::c_int,
                   1 as libc::c_int, 1.0f32, 1.0f32);
    Delta_AddField(b"movevars_t\x00" as *const u8 as *const libc::c_char,
                   b"rollangle\x00" as *const u8 as *const libc::c_char,
                   ((1 as libc::c_uint) << 2 as libc::c_int |
                        (1 as libc::c_uint) << 8 as libc::c_int) as
                       libc::c_int, 16 as libc::c_int, 32.0f32, 1.0f32);
    Delta_AddField(b"movevars_t\x00" as *const u8 as *const libc::c_char,
                   b"rollspeed\x00" as *const u8 as *const libc::c_char,
                   ((1 as libc::c_uint) << 2 as libc::c_int |
                        (1 as libc::c_uint) << 8 as libc::c_int) as
                       libc::c_int, 16 as libc::c_int, 8.0f32, 1.0f32);
    Delta_AddField(b"movevars_t\x00" as *const u8 as *const libc::c_char,
                   b"skycolor_r\x00" as *const u8 as *const libc::c_char,
                   ((1 as libc::c_uint) << 2 as libc::c_int |
                        (1 as libc::c_uint) << 8 as libc::c_int) as
                       libc::c_int, 16 as libc::c_int, 1.0f32, 1.0f32);
    Delta_AddField(b"movevars_t\x00" as *const u8 as *const libc::c_char,
                   b"skycolor_g\x00" as *const u8 as *const libc::c_char,
                   ((1 as libc::c_uint) << 2 as libc::c_int |
                        (1 as libc::c_uint) << 8 as libc::c_int) as
                       libc::c_int, 16 as libc::c_int, 1.0f32, 1.0f32);
    Delta_AddField(b"movevars_t\x00" as *const u8 as *const libc::c_char,
                   b"skycolor_b\x00" as *const u8 as *const libc::c_char,
                   ((1 as libc::c_uint) << 2 as libc::c_int |
                        (1 as libc::c_uint) << 8 as libc::c_int) as
                       libc::c_int, 16 as libc::c_int, 1.0f32, 1.0f32);
    Delta_AddField(b"movevars_t\x00" as *const u8 as *const libc::c_char,
                   b"skyvec_x\x00" as *const u8 as *const libc::c_char,
                   ((1 as libc::c_uint) << 2 as libc::c_int |
                        (1 as libc::c_uint) << 8 as libc::c_int) as
                       libc::c_int, 16 as libc::c_int, 32.0f32, 1.0f32);
    Delta_AddField(b"movevars_t\x00" as *const u8 as *const libc::c_char,
                   b"skyvec_y\x00" as *const u8 as *const libc::c_char,
                   ((1 as libc::c_uint) << 2 as libc::c_int |
                        (1 as libc::c_uint) << 8 as libc::c_int) as
                       libc::c_int, 16 as libc::c_int, 32.0f32, 1.0f32);
    Delta_AddField(b"movevars_t\x00" as *const u8 as *const libc::c_char,
                   b"skyvec_z\x00" as *const u8 as *const libc::c_char,
                   ((1 as libc::c_uint) << 2 as libc::c_int |
                        (1 as libc::c_uint) << 8 as libc::c_int) as
                       libc::c_int, 16 as libc::c_int, 32.0f32, 1.0f32);
    Delta_AddField(b"movevars_t\x00" as *const u8 as *const libc::c_char,
                   b"wateralpha\x00" as *const u8 as *const libc::c_char,
                   ((1 as libc::c_uint) << 2 as libc::c_int |
                        (1 as libc::c_uint) << 8 as libc::c_int) as
                       libc::c_int, 16 as libc::c_int, 32.0f32, 1.0f32);
    Delta_AddField(b"movevars_t\x00" as *const u8 as *const libc::c_char,
                   b"fog_settings\x00" as *const u8 as *const libc::c_char,
                   ((1 as libc::c_uint) << 3 as libc::c_int) as libc::c_int,
                   32 as libc::c_int, 1.0f32, 1.0f32);
    (*dt).numFields =
        (::std::mem::size_of::<[delta_field_t; 32]>() as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<delta_field_t>()
                                             as
                                             libc::c_ulong).wrapping_sub(1 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_ulong).wrapping_sub(4
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_ulong)
            as libc::c_int;
    // now done
    (*dt).bInitialized = true_0;
}
#[no_mangle]
pub unsafe extern "C" fn Delta_InitClient() {
    let mut i: libc::c_int = 0;
    let mut numActive: libc::c_int = 0 as libc::c_int;
    // already initalized
    if delta_init as u64 != 0 { return }
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[delta_info_t; 9]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<delta_info_t>()
                                                   as
                                                   libc::c_ulong).wrapping_sub(1
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_ulong)
          {
        if dt_info[i as usize].numFields > 0 as libc::c_int {
            dt_info[i as usize].bInitialized = true_0;
            numActive += 1
        }
        i += 1
    }
    if numActive != 0 { delta_init = true_0 };
}
#[no_mangle]
pub unsafe extern "C" fn Delta_Shutdown() {
    let mut i: libc::c_int = 0;
    if delta_init as u64 == 0 { return }
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[delta_info_t; 9]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<delta_info_t>()
                                                   as
                                                   libc::c_ulong).wrapping_sub(1
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_ulong)
          {
        dt_info[i as usize].numFields = 0 as libc::c_int;
        dt_info[i as usize].customEncode = CUSTOM_NONE as libc::c_int;
        dt_info[i as usize].userCallback = None;
        dt_info[i as usize].funcName[0 as libc::c_int as usize] =
            '\u{0}' as i32 as libc::c_char;
        if !dt_info[i as usize].pFields.is_null() {
            if !dt_info[i as usize].pFields.is_null() {
                _Mem_Free(dt_info[i as usize].pFields as *mut libc::c_void,
                          b"../engine/common/net_encode.c\x00" as *const u8 as
                              *const libc::c_char, 880 as libc::c_int);
            }
            dt_info[i as usize].pFields = 0 as *mut delta_t
        }
        dt_info[i as usize].bInitialized = false_0;
        i += 1
    }
    delta_init = false_0;
}
/*
=====================
Delta_ClampIntegerField

prevent data to out of range
=====================
*/
#[no_mangle]
pub unsafe extern "C" fn Delta_ClampIntegerField(mut pField: *mut delta_t,
                                                 mut iValue: libc::c_int,
                                                 mut bSigned: qboolean,
                                                 mut numbits: libc::c_int)
 -> libc::c_int {
    if numbits < 32 as libc::c_int {
        let mut signbits: libc::c_int =
            if bSigned as libc::c_uint != 0 {
                (numbits) - 1 as libc::c_int
            } else { numbits };
        let mut maxnum: libc::c_int =
            ((1 as libc::c_uint) <<
                 signbits).wrapping_sub(1 as libc::c_int as libc::c_uint) as
                libc::c_int;
        let mut minnum: libc::c_int =
            if bSigned as libc::c_uint != 0 {
                (-maxnum) - 1 as libc::c_int
            } else { 0 as libc::c_int };
        iValue =
            if iValue >= minnum {
                if iValue < maxnum { iValue } else { maxnum }
            } else { minnum }
    }
    return iValue;
    // clamped;
}
/*
=====================
Delta_CompareField

compare fields by offsets
assume from and to is valid
=====================
*/
#[no_mangle]
pub unsafe extern "C" fn Delta_CompareField(mut pField: *mut delta_t,
                                            mut from: *mut libc::c_void,
                                            mut to: *mut libc::c_void,
                                            mut timebase: libc::c_double)
 -> qboolean {
    let mut bSigned: qboolean =
        if (*pField).flags as libc::c_uint &
               (1 as libc::c_uint) << 8 as libc::c_int != 0 {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int } as qboolean;
    let mut val_a: libc::c_float = 0.;
    let mut val_b: libc::c_float = 0.;
    let mut fromF: libc::c_int = 0;
    let mut toF: libc::c_int = 0;
    if (*pField).bInactive as u64 != 0 { return true_0 }
    toF = 0 as libc::c_int;
    fromF = toF;
    if (*pField).flags as libc::c_uint &
           (1 as libc::c_uint) << 0 as libc::c_int != 0 {
        if (*pField).flags as libc::c_uint &
               (1 as libc::c_uint) << 8 as libc::c_int != 0 {
            fromF =
                *((from as *mut byte).offset((*pField).offset as isize) as
                      *mut libc::c_schar) as libc::c_int;
            toF =
                *((to as *mut byte).offset((*pField).offset as isize) as
                      *mut libc::c_schar) as libc::c_int
        } else {
            fromF =
                *(from as *mut byte).offset((*pField).offset as isize) as
                    libc::c_int;
            toF =
                *(to as *mut byte).offset((*pField).offset as isize) as
                    libc::c_int
        }
        fromF =
            Delta_ClampIntegerField(pField, fromF, bSigned, (*pField).bits);
        toF = Delta_ClampIntegerField(pField, toF, bSigned, (*pField).bits);
        if !((*pField).multiplier as libc::c_double >
                 1.0f64 - 0.001f32 as libc::c_double &&
                 ((*pField).multiplier as libc::c_double) <
                     1.0f64 + 0.001f32 as libc::c_double) {
            fromF =
                (fromF as libc::c_float * (*pField).multiplier) as libc::c_int
        }
        if !((*pField).multiplier as libc::c_double >
                 1.0f64 - 0.001f32 as libc::c_double &&
                 ((*pField).multiplier as libc::c_double) <
                     1.0f64 + 0.001f32 as libc::c_double) {
            toF = (toF as libc::c_float * (*pField).multiplier) as libc::c_int
        }
    } else if (*pField).flags as libc::c_uint &
                  (1 as libc::c_uint) << 1 as libc::c_int != 0 {
        if (*pField).flags as libc::c_uint &
               (1 as libc::c_uint) << 8 as libc::c_int != 0 {
            fromF =
                *((from as *mut byte).offset((*pField).offset as isize) as
                      *mut libc::c_short) as libc::c_int;
            toF =
                *((to as *mut byte).offset((*pField).offset as isize) as
                      *mut libc::c_short) as libc::c_int
        } else {
            fromF =
                *((from as *mut byte).offset((*pField).offset as isize) as
                      *mut word) as libc::c_int;
            toF =
                *((to as *mut byte).offset((*pField).offset as isize) as
                      *mut word) as libc::c_int
        }
        fromF =
            Delta_ClampIntegerField(pField, fromF, bSigned, (*pField).bits);
        toF = Delta_ClampIntegerField(pField, toF, bSigned, (*pField).bits);
        if !((*pField).multiplier as libc::c_double >
                 1.0f64 - 0.001f32 as libc::c_double &&
                 ((*pField).multiplier as libc::c_double) <
                     1.0f64 + 0.001f32 as libc::c_double) {
            fromF =
                (fromF as libc::c_float * (*pField).multiplier) as libc::c_int
        }
        if !((*pField).multiplier as libc::c_double >
                 1.0f64 - 0.001f32 as libc::c_double &&
                 ((*pField).multiplier as libc::c_double) <
                     1.0f64 + 0.001f32 as libc::c_double) {
            toF = (toF as libc::c_float * (*pField).multiplier) as libc::c_int
        }
    } else if (*pField).flags as libc::c_uint &
                  (1 as libc::c_uint) << 3 as libc::c_int != 0 {
        if (*pField).flags as libc::c_uint &
               (1 as libc::c_uint) << 8 as libc::c_int != 0 {
            fromF =
                *((from as *mut byte).offset((*pField).offset as isize) as
                      *mut libc::c_int);
            toF =
                *((to as *mut byte).offset((*pField).offset as isize) as
                      *mut libc::c_int)
        } else {
            fromF =
                *((from as *mut byte).offset((*pField).offset as isize) as
                      *mut uint) as libc::c_int;
            toF =
                *((to as *mut byte).offset((*pField).offset as isize) as
                      *mut uint) as libc::c_int
        }
        fromF =
            Delta_ClampIntegerField(pField, fromF, bSigned, (*pField).bits);
        toF = Delta_ClampIntegerField(pField, toF, bSigned, (*pField).bits);
        if !((*pField).multiplier as libc::c_double >
                 1.0f64 - 0.001f32 as libc::c_double &&
                 ((*pField).multiplier as libc::c_double) <
                     1.0f64 + 0.001f32 as libc::c_double) {
            fromF =
                (fromF as libc::c_float * (*pField).multiplier) as libc::c_int
        }
        if !((*pField).multiplier as libc::c_double >
                 1.0f64 - 0.001f32 as libc::c_double &&
                 ((*pField).multiplier as libc::c_double) <
                     1.0f64 + 0.001f32 as libc::c_double) {
            toF = (toF as libc::c_float * (*pField).multiplier) as libc::c_int
        }
    } else if (*pField).flags as libc::c_uint &
                  ((1 as libc::c_uint) << 4 as libc::c_int |
                       (1 as libc::c_uint) << 2 as libc::c_int) != 0 {
        // don't convert floats to integers
        fromF =
            *((from as *mut byte).offset((*pField).offset as isize) as
                  *mut libc::c_int);
        toF =
            *((to as *mut byte).offset((*pField).offset as isize) as
                  *mut libc::c_int)
    } else if (*pField).flags as libc::c_uint &
                  (1 as libc::c_uint) << 5 as libc::c_int != 0 {
        val_a =
            if *((from as *mut byte).offset((*pField).offset as isize) as
                     *mut libc::c_float) as libc::c_double * 100.0f64 <
                   0.0f32 as libc::c_double {
                (*((from as *mut byte).offset((*pField).offset as isize) as
                       *mut libc::c_float) as libc::c_double * 100.0f64 -
                     0.5f32 as libc::c_double) as libc::c_int
            } else {
                (*((from as *mut byte).offset((*pField).offset as isize) as
                       *mut libc::c_float) as libc::c_double * 100.0f64 +
                     0.5f32 as libc::c_double) as libc::c_int
            } as libc::c_float;
        val_b =
            if *((to as *mut byte).offset((*pField).offset as isize) as
                     *mut libc::c_float) as libc::c_double * 100.0f64 <
                   0.0f32 as libc::c_double {
                (*((to as *mut byte).offset((*pField).offset as isize) as
                       *mut libc::c_float) as libc::c_double * 100.0f64 -
                     0.5f32 as libc::c_double) as libc::c_int
            } else {
                (*((to as *mut byte).offset((*pField).offset as isize) as
                       *mut libc::c_float) as libc::c_double * 100.0f64 +
                     0.5f32 as libc::c_double) as libc::c_int
            } as libc::c_float;
        val_a -=
            if timebase * 100.0f64 < 0.0f32 as libc::c_double {
                (timebase * 100.0f64 - 0.5f32 as libc::c_double) as
                    libc::c_int
            } else {
                (timebase * 100.0f64 + 0.5f32 as libc::c_double) as
                    libc::c_int
            } as libc::c_float;
        val_b -=
            if timebase * 100.0f64 < 0.0f32 as libc::c_double {
                (timebase * 100.0f64 - 0.5f32 as libc::c_double) as
                    libc::c_int
            } else {
                (timebase * 100.0f64 + 0.5f32 as libc::c_double) as
                    libc::c_int
            } as libc::c_float;
        fromF = *(&mut val_a as *mut libc::c_float as *mut libc::c_int);
        toF = *(&mut val_b as *mut libc::c_float as *mut libc::c_int)
    } else if (*pField).flags as libc::c_uint &
                  (1 as libc::c_uint) << 6 as libc::c_int != 0 {
        val_a =
            *((from as *mut byte).offset((*pField).offset as isize) as
                  *mut libc::c_float);
        val_b =
            *((to as *mut byte).offset((*pField).offset as isize) as
                  *mut libc::c_float);
        if !((*pField).multiplier as libc::c_double >
                 1.0f64 - 0.001f32 as libc::c_double &&
                 ((*pField).multiplier as libc::c_double) <
                     1.0f64 + 0.001f32 as libc::c_double) {
            val_a *= (*pField).multiplier;
            val_b *= (*pField).multiplier;
            val_a =
                (timebase * (*pField).multiplier as libc::c_double -
                     val_a as libc::c_double) as libc::c_float;
            val_b =
                (timebase * (*pField).multiplier as libc::c_double -
                     val_b as libc::c_double) as libc::c_float
        } else {
            val_a = (timebase - val_a as libc::c_double) as libc::c_float;
            val_b = (timebase - val_b as libc::c_double) as libc::c_float
        }
        fromF = *(&mut val_a as *mut libc::c_float as *mut libc::c_int);
        toF = *(&mut val_b as *mut libc::c_float as *mut libc::c_int)
    } else if (*pField).flags as libc::c_uint &
                  (1 as libc::c_uint) << 7 as libc::c_int != 0 {
        // compare strings
        let mut s1: *mut libc::c_char =
            (from as *mut byte).offset((*pField).offset as isize) as
                *mut libc::c_char;
        let mut s2: *mut libc::c_char =
            (to as *mut byte).offset((*pField).offset as isize) as
                *mut libc::c_char;
        // 0 is equal, otherwise not equal
        toF = Q_strncmp(s1, s2, 99999 as libc::c_int)
    }
    return if fromF == toF {
               true_0 as libc::c_int
           } else { false_0 as libc::c_int } as qboolean;
}
/*
=====================
Delta_TestBaseline

compare baselines to find optimal
=====================
*/
#[no_mangle]
pub unsafe extern "C" fn Delta_TestBaseline(mut from: *mut entity_state_t,
                                            mut to: *mut entity_state_t,
                                            mut player: qboolean,
                                            mut timebase: libc::c_double)
 -> libc::c_int {
    let mut dt: *mut delta_info_t = 0 as *mut delta_info_t; // entityType flag
    let mut pField: *mut delta_t = 0 as *mut delta_t;
    let mut i: libc::c_int = 0;
    let mut countBits: libc::c_int = 0;
    let mut numChanges: libc::c_int = 0 as libc::c_int;
    countBits = 13 as libc::c_int + 2 as libc::c_int;
    if to.is_null() {
        if from.is_null() { return 0 as libc::c_int }
        return countBits
    }
    if (*to).entityType & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        dt =
            Delta_FindStruct(b"custom_entity_state_t\x00" as *const u8 as
                                 *const libc::c_char)
    } else if player as u64 != 0 {
        dt =
            Delta_FindStruct(b"entity_state_player_t\x00" as *const u8 as
                                 *const libc::c_char)
    } else {
        dt =
            Delta_FindStruct(b"entity_state_t\x00" as *const u8 as
                                 *const libc::c_char)
    }
    countBits += 1;
    pField = (*dt).pFields;
    // activate fields and call custom encode func
    Delta_CustomEncode(dt, from as *const libc::c_void,
                       to as *const libc::c_void);
    // process fields
    i = 0 as libc::c_int;
    while i < (*dt).numFields {
        // flag about field change (sets always)
        countBits += 1;
        if Delta_CompareField(pField, from as *mut libc::c_void,
                              to as *mut libc::c_void, timebase) as u64 == 0 {
            // strings are handled difference
            if (*pField).flags as libc::c_uint &
                   (1 as libc::c_uint) << 7 as libc::c_int != 0 {
                countBits =
                    (countBits as
                         libc::c_ulong).wrapping_add(Q_strlen((to as
                                                                   *mut byte).offset((*pField).offset
                                                                                         as
                                                                                         isize)
                                                                  as
                                                                  *mut libc::c_char).wrapping_mul(8
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      libc::c_ulong))
                        as libc::c_int as libc::c_int
            } else { countBits += (*pField).bits }
        }
        i += 1;
        pField = pField.offset(1)
    }
    // g-cont. compare bitcount directly no reason to call BitByte here
    return countBits;
}
/*
=====================
Delta_WriteField

write fields by offsets
assume from and to is valid
=====================
*/
#[no_mangle]
pub unsafe extern "C" fn Delta_WriteField(mut msg: *mut sizebuf_t,
                                          mut pField: *mut delta_t,
                                          mut from: *mut libc::c_void,
                                          mut to: *mut libc::c_void,
                                          mut timebase: libc::c_double)
 -> qboolean {
    let mut bSigned: qboolean =
        if (*pField).flags as libc::c_uint &
               (1 as libc::c_uint) << 8 as libc::c_int != 0 {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int } as qboolean; // unchanged
    let mut flValue: libc::c_float = 0.; // changed
    let mut flAngle: libc::c_float = 0.;
    let mut flTime: libc::c_float = 0.;
    let mut iValue: uint = 0;
    let mut pStr: *const libc::c_char = 0 as *const libc::c_char;
    if Delta_CompareField(pField, from, to, timebase) as u64 != 0 {
        MSG_WriteOneBit(msg, 0 as libc::c_int);
        return false_0
    }
    MSG_WriteOneBit(msg, 1 as libc::c_int);
    if (*pField).flags as libc::c_uint &
           (1 as libc::c_uint) << 0 as libc::c_int != 0 {
        if bSigned as u64 != 0 {
            iValue =
                *(to as *mut int8_t).offset((*pField).offset as isize) as uint
        } else {
            iValue =
                *((to as *mut int8_t).offset((*pField).offset as isize) as
                      *mut uint8_t) as uint
        }
        iValue =
            Delta_ClampIntegerField(pField, iValue as libc::c_int, bSigned,
                                    (*pField).bits) as uint;
        if !((*pField).multiplier as libc::c_double >
                 1.0f64 - 0.001f32 as libc::c_double &&
                 ((*pField).multiplier as libc::c_double) <
                     1.0f64 + 0.001f32 as libc::c_double) {
            iValue = (iValue as libc::c_float * (*pField).multiplier) as uint
        }
        MSG_WriteBitLong(msg, iValue, (*pField).bits, bSigned);
    } else if (*pField).flags as libc::c_uint &
                  (1 as libc::c_uint) << 1 as libc::c_int != 0 {
        if bSigned as u64 != 0 {
            iValue =
                *((to as *mut int8_t).offset((*pField).offset as isize) as
                      *mut int16_t) as uint
        } else {
            iValue =
                *((to as *mut int8_t).offset((*pField).offset as isize) as
                      *mut uint16_t) as uint
        }
        iValue =
            Delta_ClampIntegerField(pField, iValue as libc::c_int, bSigned,
                                    (*pField).bits) as uint;
        if !((*pField).multiplier as libc::c_double >
                 1.0f64 - 0.001f32 as libc::c_double &&
                 ((*pField).multiplier as libc::c_double) <
                     1.0f64 + 0.001f32 as libc::c_double) {
            iValue = (iValue as libc::c_float * (*pField).multiplier) as uint
        }
        MSG_WriteBitLong(msg, iValue, (*pField).bits, bSigned);
    } else if (*pField).flags as libc::c_uint &
                  (1 as libc::c_uint) << 3 as libc::c_int != 0 {
        if bSigned as u64 != 0 {
            iValue =
                *((to as *mut int8_t).offset((*pField).offset as isize) as
                      *mut int32_t) as uint
        } else {
            iValue =
                *((to as *mut int8_t).offset((*pField).offset as isize) as
                      *mut uint32_t)
        }
        iValue =
            Delta_ClampIntegerField(pField, iValue as libc::c_int, bSigned,
                                    (*pField).bits) as uint;
        if !((*pField).multiplier as libc::c_double >
                 1.0f64 - 0.001f32 as libc::c_double &&
                 ((*pField).multiplier as libc::c_double) <
                     1.0f64 + 0.001f32 as libc::c_double) {
            iValue = (iValue as libc::c_float * (*pField).multiplier) as uint
        }
        MSG_WriteBitLong(msg, iValue, (*pField).bits, bSigned);
    } else if (*pField).flags as libc::c_uint &
                  (1 as libc::c_uint) << 2 as libc::c_int != 0 {
        flValue =
            *((to as *mut byte).offset((*pField).offset as isize) as
                  *mut libc::c_float);
        iValue =
            (flValue as libc::c_double *
                 (*pField).multiplier as libc::c_double) as libc::c_int as
                uint;
        iValue =
            Delta_ClampIntegerField(pField, iValue as libc::c_int, bSigned,
                                    (*pField).bits) as uint;
        MSG_WriteBitLong(msg, iValue, (*pField).bits, bSigned);
    } else if (*pField).flags as libc::c_uint &
                  (1 as libc::c_uint) << 4 as libc::c_int != 0 {
        flAngle =
            *((to as *mut byte).offset((*pField).offset as isize) as
                  *mut libc::c_float);
        // NOTE: never applies multipliers to angle because
		// result may be wrong on client-side
        MSG_WriteBitAngle(msg, flAngle,
                          (*pField).bits); // timewindow is always signed
    } else if (*pField).flags as libc::c_uint &
                  (1 as libc::c_uint) << 5 as libc::c_int != 0 {
        bSigned = true_0; // timewindow is always signed
        flValue =
            *((to as *mut byte).offset((*pField).offset as isize) as
                  *mut libc::c_float);
        iValue =
            ((if timebase * 100.0f64 < 0.0f32 as libc::c_double {
                  (timebase * 100.0f64 - 0.5f32 as libc::c_double) as
                      libc::c_int
              } else {
                  (timebase * 100.0f64 + 0.5f32 as libc::c_double) as
                      libc::c_int
              }) -
                 (if flValue as libc::c_double * 100.0f64 <
                         0.0f32 as libc::c_double {
                      (flValue as libc::c_double * 100.0f64 -
                           0.5f32 as libc::c_double) as libc::c_int
                  } else {
                      (flValue as libc::c_double * 100.0f64 +
                           0.5f32 as libc::c_double) as libc::c_int
                  })) as uint;
        iValue =
            Delta_ClampIntegerField(pField, iValue as libc::c_int, bSigned,
                                    (*pField).bits) as uint;
        MSG_WriteBitLong(msg, iValue, (*pField).bits, bSigned);
    } else if (*pField).flags as libc::c_uint &
                  (1 as libc::c_uint) << 6 as libc::c_int != 0 {
        bSigned = true_0;
        flValue =
            *((to as *mut byte).offset((*pField).offset as isize) as
                  *mut libc::c_float);
        iValue =
            ((if (timebase * (*pField).multiplier as libc::c_double) <
                     0.0f32 as libc::c_double {
                  (timebase * (*pField).multiplier as libc::c_double -
                       0.5f32 as libc::c_double) as libc::c_int
              } else {
                  (timebase * (*pField).multiplier as libc::c_double +
                       0.5f32 as libc::c_double) as libc::c_int
              }) -
                 (if flValue * (*pField).multiplier < 0.0f32 {
                      (flValue * (*pField).multiplier - 0.5f32) as libc::c_int
                  } else {
                      (flValue * (*pField).multiplier + 0.5f32) as libc::c_int
                  })) as uint;
        iValue =
            Delta_ClampIntegerField(pField, iValue as libc::c_int, bSigned,
                                    (*pField).bits) as uint;
        MSG_WriteBitLong(msg, iValue, (*pField).bits, bSigned);
    } else if (*pField).flags as libc::c_uint &
                  (1 as libc::c_uint) << 7 as libc::c_int != 0 {
        pStr =
            (to as *mut byte).offset((*pField).offset as isize) as
                *mut libc::c_char;
        MSG_WriteString(msg, pStr);
    }
    return true_0;
}
/*
=====================
Delta_ReadField

read fields by offsets
assume 'from' and 'to' is valid
=====================
*/
#[no_mangle]
pub unsafe extern "C" fn Delta_ReadField(mut msg: *mut sizebuf_t,
                                         mut pField: *mut delta_t,
                                         mut from: *mut libc::c_void,
                                         mut to: *mut libc::c_void,
                                         mut timebase: libc::c_double)
 -> qboolean {
    let mut bSigned: qboolean =
        if (*pField).flags as libc::c_uint &
               (1 as libc::c_uint) << 8 as libc::c_int != 0 {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int } as
            qboolean; // timewindow is always signed
    let mut flValue: libc::c_float = 0.; // timewindow is always signed
    let mut flAngle: libc::c_float = 0.;
    let mut flTime: libc::c_float = 0.;
    let mut bChanged: qboolean = false_0;
    let mut iValue: uint = 0;
    let mut pStr: *const libc::c_char = 0 as *const libc::c_char;
    let mut pOut: *mut libc::c_char = 0 as *mut libc::c_char;
    bChanged = MSG_ReadOneBit(msg) as qboolean;
    if (*pField).flags as libc::c_uint &
           (1 as libc::c_uint) << 0 as libc::c_int != 0 {
        if bChanged as u64 != 0 {
            iValue = MSG_ReadBitLong(msg, (*pField).bits, bSigned);
            if !((*pField).multiplier as libc::c_double >
                     1.0f64 - 0.001f32 as libc::c_double &&
                     ((*pField).multiplier as libc::c_double) <
                         1.0f64 + 0.001f32 as libc::c_double) {
                iValue =
                    (iValue as libc::c_float / (*pField).multiplier) as uint
            }
        } else if bSigned as u64 != 0 {
            iValue =
                *((from as *mut uint8_t).offset((*pField).offset as isize) as
                      *mut int8_t) as uint
        } else {
            iValue =
                *(from as *mut uint8_t).offset((*pField).offset as isize) as
                    uint
        }
        if bSigned as u64 != 0 {
            *((to as *mut uint8_t).offset((*pField).offset as isize) as
                  *mut int8_t) = iValue as int8_t
        } else {
            *(to as *mut uint8_t).offset((*pField).offset as isize) =
                iValue as uint8_t
        }
    } else if (*pField).flags as libc::c_uint &
                  (1 as libc::c_uint) << 1 as libc::c_int != 0 {
        if bChanged as u64 != 0 {
            iValue = MSG_ReadBitLong(msg, (*pField).bits, bSigned);
            if !((*pField).multiplier as libc::c_double >
                     1.0f64 - 0.001f32 as libc::c_double &&
                     ((*pField).multiplier as libc::c_double) <
                         1.0f64 + 0.001f32 as libc::c_double) {
                iValue =
                    (iValue as libc::c_float / (*pField).multiplier) as uint
            }
        } else if bSigned as u64 != 0 {
            iValue =
                *((from as *mut uint8_t).offset((*pField).offset as isize) as
                      *mut int16_t) as uint
        } else {
            iValue =
                *((from as *mut uint8_t).offset((*pField).offset as isize) as
                      *mut uint16_t) as uint
        }
        if bSigned as u64 != 0 {
            *((to as *mut uint8_t).offset((*pField).offset as isize) as
                  *mut int16_t) = iValue as int16_t
        } else {
            *((to as *mut uint8_t).offset((*pField).offset as isize) as
                  *mut uint16_t) = iValue as uint16_t
        }
    } else if (*pField).flags as libc::c_uint &
                  (1 as libc::c_uint) << 3 as libc::c_int != 0 {
        if bChanged as u64 != 0 {
            iValue = MSG_ReadBitLong(msg, (*pField).bits, bSigned);
            if !((*pField).multiplier as libc::c_double >
                     1.0f64 - 0.001f32 as libc::c_double &&
                     ((*pField).multiplier as libc::c_double) <
                         1.0f64 + 0.001f32 as libc::c_double) {
                iValue =
                    (iValue as libc::c_float / (*pField).multiplier) as uint
            }
        } else if bSigned as u64 != 0 {
            iValue =
                *((from as *mut uint8_t).offset((*pField).offset as isize) as
                      *mut int32_t) as uint
        } else {
            iValue =
                *((from as *mut uint8_t).offset((*pField).offset as isize) as
                      *mut uint32_t)
        }
        if bSigned as u64 != 0 {
            *((to as *mut uint8_t).offset((*pField).offset as isize) as
                  *mut int32_t) = iValue as int32_t
        } else {
            *((to as *mut uint8_t).offset((*pField).offset as isize) as
                  *mut uint32_t) = iValue
        }
    } else if (*pField).flags as libc::c_uint &
                  (1 as libc::c_uint) << 2 as libc::c_int != 0 {
        if bChanged as u64 != 0 {
            iValue = MSG_ReadBitLong(msg, (*pField).bits, bSigned);
            if bSigned as u64 != 0 {
                flValue = iValue as libc::c_int as libc::c_float
            } else { flValue = iValue as libc::c_float }
            if !((*pField).multiplier as libc::c_double >
                     1.0f64 - 0.001f32 as libc::c_double &&
                     ((*pField).multiplier as libc::c_double) <
                         1.0f64 + 0.001f32 as libc::c_double) {
                flValue = flValue / (*pField).multiplier
            }
            if !((*pField).post_multiplier as libc::c_double >
                     1.0f64 - 0.001f32 as libc::c_double &&
                     ((*pField).post_multiplier as libc::c_double) <
                         1.0f64 + 0.001f32 as libc::c_double) {
                flValue = flValue * (*pField).post_multiplier
            }
        } else {
            flValue =
                *((from as *mut byte).offset((*pField).offset as isize) as
                      *mut libc::c_float)
        }
        *((to as *mut byte).offset((*pField).offset as isize) as
              *mut libc::c_float) = flValue
    } else if (*pField).flags as libc::c_uint &
                  (1 as libc::c_uint) << 4 as libc::c_int != 0 {
        if bChanged as u64 != 0 {
            flAngle = MSG_ReadBitAngle(msg, (*pField).bits)
        } else {
            flAngle =
                *((from as *mut byte).offset((*pField).offset as isize) as
                      *mut libc::c_float)
        }
        *((to as *mut byte).offset((*pField).offset as isize) as
              *mut libc::c_float) = flAngle
    } else if (*pField).flags as libc::c_uint &
                  (1 as libc::c_uint) << 5 as libc::c_int != 0 {
        if bChanged as u64 != 0 {
            bSigned = true_0;
            iValue = MSG_ReadBitLong(msg, (*pField).bits, bSigned);
            flTime =
                ((timebase * 100.0f64 - iValue as libc::c_double) / 100.0f64)
                    as libc::c_float
        } else {
            flTime =
                *((from as *mut byte).offset((*pField).offset as isize) as
                      *mut libc::c_float)
        }
        *((to as *mut byte).offset((*pField).offset as isize) as
              *mut libc::c_float) = flTime
    } else if (*pField).flags as libc::c_uint &
                  (1 as libc::c_uint) << 6 as libc::c_int != 0 {
        if bChanged as u64 != 0 {
            bSigned = true_0;
            iValue = MSG_ReadBitLong(msg, (*pField).bits, bSigned);
            if !((*pField).multiplier as libc::c_double >
                     1.0f64 - 0.001f32 as libc::c_double &&
                     ((*pField).multiplier as libc::c_double) <
                         1.0f64 + 0.001f32 as libc::c_double) {
                flTime =
                    ((timebase * (*pField).multiplier as libc::c_double -
                          iValue as libc::c_double) /
                         (*pField).multiplier as libc::c_double) as
                        libc::c_float
            } else {
                flTime =
                    (timebase - iValue as libc::c_double) as libc::c_float
            }
        } else {
            flTime =
                *((from as *mut byte).offset((*pField).offset as isize) as
                      *mut libc::c_float)
        }
        *((to as *mut byte).offset((*pField).offset as isize) as
              *mut libc::c_float) = flTime
    } else if (*pField).flags as libc::c_uint &
                  (1 as libc::c_uint) << 7 as libc::c_int != 0 {
        if bChanged as u64 != 0 {
            pStr = MSG_ReadStringExt(msg, false_0)
        } else {
            pStr =
                (from as *mut byte).offset((*pField).offset as isize) as
                    *mut libc::c_char
        }
        pOut =
            (to as *mut byte).offset((*pField).offset as isize) as
                *mut libc::c_char;
        Q_strncpy(pOut, pStr, (*pField).size as size_t);
    }
    return bChanged;
}
/*
=============================================================================

usercmd_t communication

=============================================================================
*/
/*
=====================
MSG_WriteDeltaUsercmd
=====================
*/
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteDeltaUsercmd(mut msg: *mut sizebuf_t,
                                               mut from: *mut usercmd_t,
                                               mut to: *mut usercmd_t) {
    let mut pField: *mut delta_t = 0 as *mut delta_t;
    let mut dt: *mut delta_info_t = 0 as *mut delta_info_t;
    let mut i: libc::c_int = 0;
    dt =
        Delta_FindStruct(b"usercmd_t\x00" as *const u8 as
                             *const libc::c_char);
    pField = (*dt).pFields;
    // activate fields and call custom encode func
    Delta_CustomEncode(dt, from as *const libc::c_void,
                       to as *const libc::c_void);
    // process fields
    i = 0 as libc::c_int;
    while i < (*dt).numFields {
        Delta_WriteField(msg, pField, from as *mut libc::c_void,
                         to as *mut libc::c_void, 0.0f32 as libc::c_double);
        i += 1;
        pField = pField.offset(1)
    };
}
/*
=====================
MSG_ReadDeltaUsercmd
=====================
*/
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadDeltaUsercmd(mut msg: *mut sizebuf_t,
                                              mut from: *mut usercmd_t,
                                              mut to: *mut usercmd_t) {
    let mut pField: *mut delta_t = 0 as *mut delta_t;
    let mut dt: *mut delta_info_t = 0 as *mut delta_info_t;
    let mut i: libc::c_int = 0;
    dt =
        Delta_FindStruct(b"usercmd_t\x00" as *const u8 as
                             *const libc::c_char);
    pField = (*dt).pFields;
    *to = *from;
    // process fields
    i = 0 as libc::c_int;
    while i < (*dt).numFields {
        Delta_ReadField(msg, pField, from as *mut libc::c_void,
                        to as *mut libc::c_void, 0.0f32 as libc::c_double);
        i += 1;
        pField = pField.offset(1)
    }
    COM_NormalizeAngles((*to).viewangles.as_mut_ptr());
}
/*
============================================================================

event_args_t communication

============================================================================
*/
/*
=====================
MSG_WriteDeltaEvent
=====================
*/
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteDeltaEvent(mut msg: *mut sizebuf_t,
                                             mut from: *mut event_args_t,
                                             mut to: *mut event_args_t) {
    let mut pField: *mut delta_t = 0 as *mut delta_t;
    let mut dt: *mut delta_info_t = 0 as *mut delta_info_t;
    let mut i: libc::c_int = 0;
    dt = Delta_FindStruct(b"event_t\x00" as *const u8 as *const libc::c_char);
    pField = (*dt).pFields;
    // activate fields and call custom encode func
    Delta_CustomEncode(dt, from as *const libc::c_void,
                       to as *const libc::c_void);
    // process fields
    i = 0 as libc::c_int;
    while i < (*dt).numFields {
        Delta_WriteField(msg, pField, from as *mut libc::c_void,
                         to as *mut libc::c_void, 0.0f32 as libc::c_double);
        i += 1;
        pField = pField.offset(1)
    };
}
/*
=====================
MSG_ReadDeltaEvent
=====================
*/
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadDeltaEvent(mut msg: *mut sizebuf_t,
                                            mut from: *mut event_args_t,
                                            mut to: *mut event_args_t) {
    let mut pField: *mut delta_t = 0 as *mut delta_t;
    let mut dt: *mut delta_info_t = 0 as *mut delta_info_t;
    let mut i: libc::c_int = 0;
    dt = Delta_FindStruct(b"event_t\x00" as *const u8 as *const libc::c_char);
    pField = (*dt).pFields;
    *to = *from;
    // process fields
    i = 0 as libc::c_int;
    while i < (*dt).numFields {
        Delta_ReadField(msg, pField, from as *mut libc::c_void,
                        to as *mut libc::c_void, 0.0f32 as libc::c_double);
        i += 1;
        pField = pField.offset(1)
    };
}
/*
=============================================================================

movevars_t communication

=============================================================================
*/
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteDeltaMovevars(mut msg: *mut sizebuf_t,
                                                mut from: *mut movevars_t,
                                                mut to: *mut movevars_t)
 -> qboolean {
    let mut pField: *mut delta_t = 0 as *mut delta_t;
    let mut dt: *mut delta_info_t = 0 as *mut delta_info_t;
    let mut i: libc::c_int = 0;
    let mut startBit: libc::c_int = 0;
    let mut numChanges: libc::c_int = 0 as libc::c_int;
    dt =
        Delta_FindStruct(b"movevars_t\x00" as *const u8 as
                             *const libc::c_char);
    pField = (*dt).pFields;
    startBit = (*msg).iCurBit;
    // activate fields and call custom encode func
    Delta_CustomEncode(dt, from as *const libc::c_void,
                       to as *const libc::c_void);
    MSG_WriteCmdExt(msg, 44 as libc::c_int, NS_SERVER,
                    0 as *const libc::c_char);
    // process fields
    i = 0 as libc::c_int;
    while i < (*dt).numFields {
        if Delta_WriteField(msg, pField, from as *mut libc::c_void,
                            to as *mut libc::c_void, 0.0f32 as libc::c_double)
               as u64 != 0 {
            numChanges += 1
        }
        i += 1;
        pField = pField.offset(1)
    }
    // if we have no changes - kill the message
    if numChanges == 0 {
        MSG_SeekToBit(msg, startBit, 0 as libc::c_int);
        return false_0
    }
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadDeltaMovevars(mut msg: *mut sizebuf_t,
                                               mut from: *mut movevars_t,
                                               mut to: *mut movevars_t) {
    let mut pField: *mut delta_t = 0 as *mut delta_t;
    let mut dt: *mut delta_info_t = 0 as *mut delta_info_t;
    let mut i: libc::c_int = 0;
    dt =
        Delta_FindStruct(b"movevars_t\x00" as *const u8 as
                             *const libc::c_char);
    pField = (*dt).pFields;
    *to = *from;
    // process fields
    i = 0 as libc::c_int;
    while i < (*dt).numFields {
        Delta_ReadField(msg, pField, from as *mut libc::c_void,
                        to as *mut libc::c_void, 0.0f32 as libc::c_double);
        i += 1;
        pField = pField.offset(1)
    };
}
/*
=============================================================================

clientdata_t communication

=============================================================================
*/
/*
==================
MSG_WriteClientData

Writes current client data only for local client
Other clients can grab the client state from entity_state_t
==================
*/
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteClientData(mut msg: *mut sizebuf_t,
                                             mut from: *mut clientdata_t,
                                             mut to: *mut clientdata_t,
                                             mut timebase: libc::c_double) {
    let mut pField: *mut delta_t = 0 as *mut delta_t; // have clientdata
    let mut dt: *mut delta_info_t = 0 as *mut delta_info_t;
    let mut i: libc::c_int = 0;
    let mut startBit: libc::c_int = 0;
    let mut numChanges: libc::c_int = 0 as libc::c_int;
    dt =
        Delta_FindStruct(b"clientdata_t\x00" as *const u8 as
                             *const libc::c_char);
    pField = (*dt).pFields;
    startBit = (*msg).iCurBit;
    MSG_WriteOneBit(msg, 1 as libc::c_int);
    // activate fields and call custom encode func
    Delta_CustomEncode(dt, from as *const libc::c_void,
                       to as *const libc::c_void);
    // process fields
    i = 0 as libc::c_int; // we have updates
    while i < (*dt).numFields {
        if Delta_WriteField(msg, pField, from as *mut libc::c_void,
                            to as *mut libc::c_void, timebase) as u64 != 0 {
            numChanges += 1
        }
        i += 1;
        pField = pField.offset(1)
    }
    if numChanges != 0 { return }
    MSG_SeekToBit(msg, startBit, 0 as libc::c_int);
    MSG_WriteOneBit(msg, 0 as libc::c_int);
    // no changes
}
/*
==================
MSG_ReadClientData

Read the clientdata
==================
*/
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadClientData(mut msg: *mut sizebuf_t,
                                            mut from: *mut clientdata_t,
                                            mut to: *mut clientdata_t,
                                            mut timebase: libc::c_double) {
    let mut pField: *mut delta_t = 0 as *mut delta_t; // we have no changes
    let mut dt: *mut delta_info_t = 0 as *mut delta_info_t;
    let mut i: libc::c_int = 0;
    dt =
        Delta_FindStruct(b"clientdata_t\x00" as *const u8 as
                             *const libc::c_char);
    pField = (*dt).pFields;
    *to = *from;
    if cls.legacymode as u64 == 0 && MSG_ReadOneBit(msg) == 0 { return }
    // process fields
    i = 0 as libc::c_int;
    while i < (*dt).numFields {
        Delta_ReadField(msg, pField, from as *mut libc::c_void,
                        to as *mut libc::c_void, timebase);
        i += 1;
        pField = pField.offset(1)
    };
}
/*
=============================================================================

weapon_data_t communication

=============================================================================
*/
/*
==================
MSG_WriteWeaponData

Writes current client data only for local client
Other clients can grab the client state from entity_state_t
==================
*/
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteWeaponData(mut msg: *mut sizebuf_t,
                                             mut from: *mut weapon_data_t,
                                             mut to: *mut weapon_data_t,
                                             mut timebase: libc::c_double,
                                             mut index: libc::c_int) {
    let mut pField: *mut delta_t = 0 as *mut delta_t;
    let mut dt: *mut delta_info_t = 0 as *mut delta_info_t;
    let mut i: libc::c_int = 0;
    let mut startBit: libc::c_int = 0;
    let mut numChanges: libc::c_int = 0 as libc::c_int;
    dt =
        Delta_FindStruct(b"weapon_data_t\x00" as *const u8 as
                             *const libc::c_char);
    pField = (*dt).pFields;
    // activate fields and call custom encode func
    Delta_CustomEncode(dt, from as *const libc::c_void,
                       to as *const libc::c_void);
    startBit = (*msg).iCurBit;
    MSG_WriteOneBit(msg, 1 as libc::c_int);
    MSG_WriteUBitLong(msg, index as uint, 6 as libc::c_int);
    // process fields
    i = 0 as libc::c_int;
    while i < (*dt).numFields {
        if Delta_WriteField(msg, pField, from as *mut libc::c_void,
                            to as *mut libc::c_void, timebase) as u64 != 0 {
            numChanges += 1
        }
        i += 1;
        pField = pField.offset(1)
    }
    // if we have no changes - kill the message
    if numChanges == 0 { MSG_SeekToBit(msg, startBit, 0 as libc::c_int); };
}
/*
==================
MSG_ReadWeaponData

Read the clientdata
==================
*/
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadWeaponData(mut msg: *mut sizebuf_t,
                                            mut from: *mut weapon_data_t,
                                            mut to: *mut weapon_data_t,
                                            mut timebase: libc::c_double) {
    let mut pField: *mut delta_t = 0 as *mut delta_t;
    let mut dt: *mut delta_info_t = 0 as *mut delta_info_t;
    let mut i: libc::c_int = 0;
    dt =
        Delta_FindStruct(b"weapon_data_t\x00" as *const u8 as
                             *const libc::c_char);
    pField = (*dt).pFields;
    *to = *from;
    // process fields
    i = 0 as libc::c_int;
    while i < (*dt).numFields {
        Delta_ReadField(msg, pField, from as *mut libc::c_void,
                        to as *mut libc::c_void, timebase);
        i += 1;
        pField = pField.offset(1)
    };
}
/*
=============================================================================

entity_state_t communication

=============================================================================
*/
/*
==================
MSG_WriteDeltaEntity

Writes part of a packetentities message, including the entity number.
Can delta from either a baseline or a previous packet_entity
If to is NULL, a remove entity update will be sent
If force is not set, then nothing at all will be generated if the entity is
identical, under the assumption that the in-order delta code will catch it.
==================
*/
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteDeltaEntity(mut from: *mut entity_state_t,
                                              mut to: *mut entity_state_t,
                                              mut msg: *mut sizebuf_t,
                                              mut force: qboolean,
                                              mut delta_type: libc::c_int,
                                              mut timebase: libc::c_double,
                                              mut baseline: libc::c_int) {
    let mut dt: *mut delta_info_t = 0 as *mut delta_info_t;
    let mut pField: *mut delta_t = 0 as *mut delta_t;
    let mut i: libc::c_int = 0;
    let mut startBit: libc::c_int = 0;
    let mut numChanges: libc::c_int = 0 as libc::c_int;
    if to.is_null() {
        let mut fRemoveType: libc::c_int = 0;
        if from.is_null() { return }
        // a NULL to is a delta remove message
        MSG_WriteUBitLong(msg, (*from).number as uint, 13 as libc::c_int);
        // fRemoveType:
		// 0 - keep alive, has delta-update
		// 1 - remove from delta message (but keep states)
		// 2 - completely remove from server
        if force as u64 != 0 {
            fRemoveType = 2 as libc::c_int
        } else { fRemoveType = 1 as libc::c_int } // alive
        MSG_WriteUBitLong(msg, fRemoveType as uint, 2 as libc::c_int);
        return
    }
    startBit = (*msg).iCurBit;
    if (*to).number < 0 as libc::c_int ||
           (*to).number >= (*SI.GameInfo).max_edicts {
        Host_Error(b"MSG_WriteDeltaEntity: Bad entity number: %i\n\x00" as
                       *const u8 as *const libc::c_char, (*to).number);
    }
    MSG_WriteUBitLong(msg, (*to).number as uint, 13 as libc::c_int);
    MSG_WriteUBitLong(msg, 0 as libc::c_int as uint, 2 as libc::c_int);
    if baseline != 0 as libc::c_int {
        MSG_WriteOneBit(msg, 1 as libc::c_int);
        MSG_WriteSBitLong(msg, baseline, 7 as libc::c_int);
    } else { MSG_WriteOneBit(msg, 0 as libc::c_int); }
    if force as libc::c_uint != 0 || (*to).entityType != (*from).entityType {
        MSG_WriteOneBit(msg, 1 as libc::c_int);
        MSG_WriteUBitLong(msg, (*to).entityType as uint, 2 as libc::c_int);
        numChanges += 1
    } else { MSG_WriteOneBit(msg, 0 as libc::c_int); }
    if (*to).entityType & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        dt =
            Delta_FindStruct(b"custom_entity_state_t\x00" as *const u8 as
                                 *const libc::c_char)
    } else if delta_type == DELTA_PLAYER as libc::c_int {
        dt =
            Delta_FindStruct(b"entity_state_player_t\x00" as *const u8 as
                                 *const libc::c_char)
    } else {
        dt =
            Delta_FindStruct(b"entity_state_t\x00" as *const u8 as
                                 *const libc::c_char)
    }
    pField = (*dt).pFields;
    if delta_type == DELTA_STATIC as libc::c_int {
        // static entities won't to be custom encoded
        i = 0 as libc::c_int;
        while i < (*dt).numFields {
            (*(*dt).pFields.offset(i as isize)).bInactive = false_0;
            i += 1
        }
    } else {
        // activate fields and call custom encode func
        Delta_CustomEncode(dt, from as *const libc::c_void,
                           to as *const libc::c_void);
    }
    // process fields
    i = 0 as libc::c_int;
    while i < (*dt).numFields {
        if Delta_WriteField(msg, pField, from as *mut libc::c_void,
                            to as *mut libc::c_void, timebase) as u64 != 0 {
            numChanges += 1
        }
        i += 1;
        pField = pField.offset(1)
    }
    // if we have no changes - kill the message
    if numChanges == 0 && force as u64 == 0 {
        MSG_SeekToBit(msg, startBit, 0 as libc::c_int);
    };
}
/*
==================
MSG_ReadDeltaEntity

The entity number has already been read from the message, which
is how the from state is identified.

If the delta removes the entity, entity_state_t->number will be set to MAX_EDICTS
Can go from either a baseline or a previous packet_entity
==================
*/
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadDeltaEntity(mut msg: *mut sizebuf_t,
                                             mut from: *mut entity_state_t,
                                             mut to: *mut entity_state_t,
                                             mut number: libc::c_int,
                                             mut delta_type: libc::c_int,
                                             mut timebase: libc::c_double)
 -> qboolean {
    let mut dt: *mut delta_info_t = 0 as *mut delta_info_t;
    let mut pField: *mut delta_t = 0 as *mut delta_t;
    let mut i: libc::c_int = 0;
    let mut fRemoveType: libc::c_int = 0;
    let mut baseline_offset: libc::c_int = 0 as libc::c_int;
    if number < 0 as libc::c_int || number >= clgame.maxEntities {
        Host_Error(b"MSG_ReadDeltaEntity: bad delta entity number: %i\n\x00"
                       as *const u8 as *const libc::c_char, number);
    }
    fRemoveType = MSG_ReadUBitLong(msg, 2 as libc::c_int) as libc::c_int;
    if fRemoveType != 0 {
        // check for a remove
        memset(to as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<entity_state_t>() as libc::c_ulong);
        if fRemoveType & 1 as libc::c_int != 0 {
            // removed from delta-message
            return false_0
        }
        if fRemoveType & 2 as libc::c_int != 0 {
            // entity was removed from server
            (*to).number = -(1 as libc::c_int);
            return false_0
        }
        Host_Error(b"MSG_ReadDeltaEntity: unknown update type %i\n\x00" as
                       *const u8 as *const libc::c_char, fRemoveType);
    }
    if cls.legacymode as u64 == 0 {
        if MSG_ReadOneBit(msg) != 0 {
            baseline_offset = MSG_ReadSBitLong(msg, 7 as libc::c_int)
        }
        if baseline_offset != 0 as libc::c_int {
            if delta_type == DELTA_STATIC as libc::c_int {
                let mut backup: libc::c_int =
                    if 0 as libc::c_int >
                           clgame.numStatics - abs(baseline_offset) {
                        0 as libc::c_int
                    } else { (clgame.numStatics) - abs(baseline_offset) };
                from =
                    &mut (*clgame.static_entities.offset(backup as
                                                             isize)).baseline
            } else if baseline_offset > 0 as libc::c_int {
                let mut backup_0: libc::c_int =
                    cls.next_client_entities - baseline_offset;
                from =
                    &mut *cls.packet_entities.offset((backup_0 %
                                                          cls.num_client_entities)
                                                         as isize) as
                        *mut entity_state_t
            } else {
                baseline_offset = abs(baseline_offset + 1 as libc::c_int);
                if baseline_offset < cl.instanced_baseline_count {
                    from =
                        &mut *cl.instanced_baseline.as_mut_ptr().offset(baseline_offset
                                                                            as
                                                                            isize)
                            as *mut entity_state_t
                }
            }
        }
    }
    // g-cont. probably is redundant
    *to = *from;
    if MSG_ReadOneBit(msg) != 0 {
        (*to).entityType =
            MSG_ReadUBitLong(msg, 2 as libc::c_int) as libc::c_int
    }
    (*to).number = number;
    if if cls.legacymode as libc::c_uint != 0 {
           ((*to).entityType == (1 as libc::c_int) << 1 as libc::c_int) as
               libc::c_int
       } else { ((*to).entityType) & (1 as libc::c_int) << 1 as libc::c_int }
           != 0 {
        dt =
            Delta_FindStruct(b"custom_entity_state_t\x00" as *const u8 as
                                 *const libc::c_char)
    } else if delta_type == DELTA_PLAYER as libc::c_int {
        dt =
            Delta_FindStruct(b"entity_state_player_t\x00" as *const u8 as
                                 *const libc::c_char)
    } else {
        dt =
            Delta_FindStruct(b"entity_state_t\x00" as *const u8 as
                                 *const libc::c_char)
    }
    pField = (*dt).pFields;
    // process fields
    i = 0 as libc::c_int;
    while i < (*dt).numFields {
        Delta_ReadField(msg, pField, from as *mut libc::c_void,
                        to as *mut libc::c_void, timebase);
        i += 1;
        pField = pField.offset(1)
    }
    // XASH_DEDICATED
    // message parsed
    return true_0;
}
/*
=============================================================================

	game.dll interface

=============================================================================
*/
#[no_mangle]
pub unsafe extern "C" fn Delta_AddEncoder(mut name: *mut libc::c_char,
                                          mut encodeFunc: pfnDeltaEncode) {
    let mut dt: *mut delta_info_t = 0 as *mut delta_info_t;
    dt = Delta_FindStructByEncoder(name);
    if dt.is_null() || (*dt).bInitialized as u64 == 0 {
        Con_DPrintf(b"^1Error:^7 Delta_AddEncoder: couldn\'t find delta with specified custom encode %s\n\x00"
                        as *const u8 as *const libc::c_char, name);
        return
    }
    if (*dt).customEncode == CUSTOM_NONE as libc::c_int {
        Con_DPrintf(b"^1Error:^7 Delta_AddEncoder: %s not supposed for custom encoding\n\x00"
                        as *const u8 as *const libc::c_char, (*dt).pName);
        return
    }
    // register new encode func
    (*dt).userCallback = encodeFunc;
}
#[no_mangle]
pub unsafe extern "C" fn Delta_FindField(mut pFields: *mut delta_t,
                                         mut fieldname: *const libc::c_char)
 -> libc::c_int {
    let mut dt: *mut delta_info_t = 0 as *mut delta_info_t;
    let mut pField: *mut delta_t = 0 as *mut delta_t;
    let mut i: libc::c_int = 0;
    dt = Delta_FindStructByDelta(pFields);
    if dt.is_null() || fieldname.is_null() ||
           *fieldname.offset(0 as libc::c_int as isize) == 0 {
        return -(1 as libc::c_int)
    }
    i = 0 as libc::c_int;
    pField = (*dt).pFields;
    while i < (*dt).numFields {
        if Q_strncmp((*pField).name, fieldname, 99999 as libc::c_int) == 0 {
            return i
        }
        i += 1;
        pField = pField.offset(1)
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Delta_SetField(mut pFields: *mut delta_t,
                                        mut fieldname: *const libc::c_char) {
    let mut dt: *mut delta_info_t = 0 as *mut delta_info_t;
    let mut pField: *mut delta_t = 0 as *mut delta_t;
    let mut i: libc::c_int = 0;
    dt = Delta_FindStructByDelta(pFields);
    if dt.is_null() || fieldname.is_null() ||
           *fieldname.offset(0 as libc::c_int as isize) == 0 {
        return
    }
    i = 0 as libc::c_int;
    pField = (*dt).pFields;
    while i < (*dt).numFields {
        if Q_strncmp((*pField).name, fieldname, 99999 as libc::c_int) == 0 {
            (*pField).bInactive = false_0;
            return
        }
        i += 1;
        pField = pField.offset(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn Delta_UnsetField(mut pFields: *mut delta_t,
                                          mut fieldname:
                                              *const libc::c_char) {
    let mut dt: *mut delta_info_t = 0 as *mut delta_info_t;
    let mut pField: *mut delta_t = 0 as *mut delta_t;
    let mut i: libc::c_int = 0;
    dt = Delta_FindStructByDelta(pFields);
    if dt.is_null() || fieldname.is_null() ||
           *fieldname.offset(0 as libc::c_int as isize) == 0 {
        return
    }
    i = 0 as libc::c_int;
    pField = (*dt).pFields;
    while i < (*dt).numFields {
        if Q_strncmp((*pField).name, fieldname, 99999 as libc::c_int) == 0 {
            (*pField).bInactive = true_0;
            return
        }
        i += 1;
        pField = pField.offset(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn Delta_SetFieldByIndex(mut pFields: *mut delta_t,
                                               mut fieldNumber: libc::c_int) {
    let mut dt: *mut delta_info_t = 0 as *mut delta_info_t;
    dt = Delta_FindStructByDelta(pFields);
    if dt.is_null() || fieldNumber < 0 as libc::c_int ||
           fieldNumber >= (*dt).numFields {
        return
    }
    (*(*dt).pFields.offset(fieldNumber as isize)).bInactive = false_0;
}
#[no_mangle]
pub unsafe extern "C" fn Delta_UnsetFieldByIndex(mut pFields: *mut delta_t,
                                                 mut fieldNumber:
                                                     libc::c_int) {
    let mut dt: *mut delta_info_t = 0 as *mut delta_info_t;
    dt = Delta_FindStructByDelta(pFields);
    if dt.is_null() || fieldNumber < 0 as libc::c_int ||
           fieldNumber >= (*dt).numFields {
        return
    }
    (*(*dt).pFields.offset(fieldNumber as isize)).bInactive = true_0;
}
unsafe extern "C" fn run_static_initializers() {
    cmd_fields =
        [{
             let mut init =
                 delta_field_t{name:
                                   b"lerp_msec\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 0 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_short>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"msec\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 2 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<byte>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"viewangles[0]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 4 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"viewangles[1]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 8 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"viewangles[2]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 12 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"forwardmove\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 16 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"sidemove\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 20 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"upmove\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 24 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"lightlevel\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 28 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<byte>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"buttons\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 30 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_ushort>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"impulse\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 32 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<byte>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"weaponselect\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 33 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<byte>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"impact_index\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 36 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"impact_position[0]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 40 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"impact_position[1]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 44 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"impact_position[2]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 48 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name: 0 as *const libc::c_char,
                               offset: 0,
                               size: 0,};
             init
         }];
    pm_fields =
        [{
             let mut init =
                 delta_field_t{name:
                                   b"gravity\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 0 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"stopspeed\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 4 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"maxspeed\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 8 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"spectatormaxspeed\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 12 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"accelerate\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 16 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"airaccelerate\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 20 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"wateraccelerate\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 24 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"friction\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 28 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"edgefriction\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 32 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"waterfriction\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 36 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"bounce\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 44 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"stepsize\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 48 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"maxvelocity\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 52 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"zmax\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 56 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"waveHeight\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 60 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"footsteps\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 64 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<qboolean>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"skyName\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 68 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<[libc::c_char; 32]>()
                                       as libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"rollangle\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 100 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"rollspeed\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 104 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"skycolor_r\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 108 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"skycolor_g\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 112 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"skycolor_b\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 116 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"skyvec_x\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 120 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"skyvec_y\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 124 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"skyvec_z\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 128 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"fog_settings\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 136 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"wateralpha\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 140 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"skydir_x\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 144 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"skydir_y\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 148 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"skydir_z\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 152 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"skyangle\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 156 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name: 0 as *const libc::c_char,
                               offset: 0,
                               size: 0,};
             init
         }];
    ev_fields =
        [{
             let mut init =
                 delta_field_t{name:
                                   b"flags\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 0 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"entindex\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 4 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"origin[0]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 8 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"origin[1]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 12 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"origin[2]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 16 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"angles[0]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 20 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"angles[1]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 24 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"angles[2]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 28 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"velocity[0]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 32 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"velocity[1]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 36 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"velocity[2]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 40 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"ducking\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 44 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"fparam1\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 48 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"fparam2\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 52 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"iparam1\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 56 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"iparam2\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 60 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"bparam1\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 64 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"bparam2\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 68 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name: 0 as *const libc::c_char,
                               offset: 0,
                               size: 0,};
             init
         }];
    wd_fields =
        [{
             let mut init =
                 delta_field_t{name:
                                   b"m_iId\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 0 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"m_iClip\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 4 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"m_flNextPrimaryAttack\x00" as *const u8
                                       as *const libc::c_char,
                               offset: 8 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"m_flNextSecondaryAttack\x00" as *const u8
                                       as *const libc::c_char,
                               offset: 12 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"m_flTimeWeaponIdle\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 16 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"m_fInReload\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 20 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"m_fInSpecialReload\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 24 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"m_flNextReload\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 28 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"m_flPumpTime\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 32 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"m_fReloadTime\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 36 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"m_fAimedDamage\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 40 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"m_fNextAimBonus\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 44 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"m_fInZoom\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 48 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"m_iWeaponState\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 52 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"iuser1\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 56 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"iuser2\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 60 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"iuser3\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 64 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"iuser4\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 68 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"fuser1\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 72 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"fuser2\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 76 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"fuser3\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 80 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"fuser4\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 84 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name: 0 as *const libc::c_char,
                               offset: 0,
                               size: 0,};
             init
         }];
    cd_fields =
        [{
             let mut init =
                 delta_field_t{name:
                                   b"origin[0]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 0 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"origin[1]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 4 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"origin[2]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 8 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"velocity[0]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 12 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"velocity[1]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 16 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"velocity[2]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 20 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"viewmodel\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 24 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"punchangle[0]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 28 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"punchangle[1]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 32 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"punchangle[2]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 36 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"flags\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 40 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"waterlevel\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 44 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"watertype\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 48 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"view_ofs[0]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 52 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"view_ofs[1]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 56 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"view_ofs[2]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 60 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"health\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 64 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"bInDuck\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 68 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"weapons\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 72 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"flTimeStepSound\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 76 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"flDuckTime\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 80 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"flSwimTime\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 84 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"waterjumptime\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 88 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"maxspeed\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 92 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"fov\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 96 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"weaponanim\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 100 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"m_iId\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 104 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"ammo_shells\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 108 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"ammo_nails\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 112 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"ammo_cells\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 116 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"ammo_rockets\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 120 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"m_flNextAttack\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 124 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"tfstate\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 128 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"pushmsec\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 132 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"deadflag\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 136 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"physinfo\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 140 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<[libc::c_char; 256]>()
                                       as libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"iuser1\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 396 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"iuser2\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 400 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"iuser3\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 404 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"iuser4\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 408 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"fuser1\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 412 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"fuser2\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 416 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"fuser3\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 420 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"fuser4\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 424 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"vuser1[0]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 428 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"vuser1[1]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 432 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"vuser1[2]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 436 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"vuser2[0]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 440 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"vuser2[1]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 444 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"vuser2[2]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 448 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"vuser3[0]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 452 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"vuser3[1]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 456 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"vuser3[2]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 460 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"vuser4[0]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 464 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"vuser4[1]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 468 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"vuser4[2]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 472 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name: 0 as *const libc::c_char,
                               offset: 0,
                               size: 0,};
             init
         }];
    ent_fields =
        [{
             let mut init =
                 delta_field_t{name:
                                   b"entityType\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 0 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"origin[0]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 16 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"origin[1]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 20 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"origin[2]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 24 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"angles[0]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 28 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"angles[1]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 32 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"angles[2]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 36 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"modelindex\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 40 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"sequence\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 44 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"frame\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 48 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"colormap\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 52 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"skin\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 56 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_short>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"solid\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 58 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_short>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"effects\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 60 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"scale\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 64 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"eflags\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 68 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<byte>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"rendermode\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 72 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"renderamt\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 76 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"rendercolor.r\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 80 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<byte>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"rendercolor.g\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 81 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<byte>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"rendercolor.b\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 82 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<byte>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"renderfx\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 84 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"movetype\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 88 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"animtime\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 92 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"framerate\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 96 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"body\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 100 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"controller[0]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 104 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<byte>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"controller[1]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 105 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<byte>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"controller[2]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 106 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<byte>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"controller[3]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 107 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<byte>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"blending[0]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 108 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<byte>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"blending[1]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 109 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<byte>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"blending[2]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 110 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<byte>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"blending[3]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 111 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<byte>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"velocity[0]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 112 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"velocity[1]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 116 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"velocity[2]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 120 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"mins[0]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 124 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"mins[1]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 128 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"mins[2]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 132 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"maxs[0]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 136 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"maxs[1]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 140 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"maxs[2]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 144 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"aiment\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 148 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"owner\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 152 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"friction\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 156 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"gravity\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 160 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"team\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 164 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"playerclass\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 168 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"health\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 172 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"spectator\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 176 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<qboolean>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"weaponmodel\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 180 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"gaitsequence\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 184 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"basevelocity[0]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 188 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"basevelocity[1]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 192 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"basevelocity[2]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 196 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"usehull\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 200 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"oldbuttons\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 204 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"onground\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 208 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"iStepLeft\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 212 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"flFallVelocity\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 216 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"fov\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 220 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"weaponanim\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 224 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"startpos[0]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 228 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"startpos[1]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 232 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"startpos[2]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 236 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"endpos[0]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 240 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"endpos[1]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 244 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"endpos[2]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 248 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"impacttime\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 252 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"starttime\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 256 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"iuser1\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 260 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"iuser2\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 264 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"iuser3\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 268 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"iuser4\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 272 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_int>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"fuser1\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 276 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"fuser2\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 280 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"fuser3\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 284 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"fuser4\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 288 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<libc::c_float>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"vuser1[0]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 292 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"vuser1[1]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 296 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"vuser1[2]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 300 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"vuser2[0]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 304 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"vuser2[1]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 308 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"vuser2[2]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 312 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"vuser3[0]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 316 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"vuser3[1]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 320 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"vuser3[2]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 324 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"vuser4[0]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 328 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"vuser4[1]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 332 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name:
                                   b"vuser4[2]\x00" as *const u8 as
                                       *const libc::c_char,
                               offset: 336 as libc::c_ulong as libc::c_int,
                               size:
                                   ::std::mem::size_of::<vec_t>() as
                                       libc::c_ulong as libc::c_int,};
             init
         },
         {
             let mut init =
                 delta_field_t{name: 0 as *const libc::c_char,
                               offset: 0,
                               size: 0,};
             init
         }];
    dt_info =
        [{
             let mut init =
                 delta_info_t{pName:
                                  b"event_t\x00" as *const u8 as
                                      *const libc::c_char,
                              pInfo: ev_fields.as_ptr(),
                              maxFields:
                                  (::std::mem::size_of::<[delta_field_t; 19]>()
                                       as
                                       libc::c_ulong).wrapping_div(::std::mem::size_of::<delta_field_t>()
                                                                       as
                                                                       libc::c_ulong).wrapping_sub(1
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong)
                                      as libc::c_int,
                              numFields: 0,
                              pFields: 0 as *mut delta_t,
                              customEncode: 0,
                              funcName: [0; 32],
                              userCallback: None,
                              bInitialized: false_0,};
             init
         },
         {
             let mut init =
                 delta_info_t{pName:
                                  b"movevars_t\x00" as *const u8 as
                                      *const libc::c_char,
                              pInfo: pm_fields.as_ptr(),
                              maxFields:
                                  (::std::mem::size_of::<[delta_field_t; 32]>()
                                       as
                                       libc::c_ulong).wrapping_div(::std::mem::size_of::<delta_field_t>()
                                                                       as
                                                                       libc::c_ulong).wrapping_sub(1
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong)
                                      as libc::c_int,
                              numFields: 0,
                              pFields: 0 as *mut delta_t,
                              customEncode: 0,
                              funcName: [0; 32],
                              userCallback: None,
                              bInitialized: false_0,};
             init
         },
         {
             let mut init =
                 delta_info_t{pName:
                                  b"usercmd_t\x00" as *const u8 as
                                      *const libc::c_char,
                              pInfo: cmd_fields.as_ptr(),
                              maxFields:
                                  (::std::mem::size_of::<[delta_field_t; 17]>()
                                       as
                                       libc::c_ulong).wrapping_div(::std::mem::size_of::<delta_field_t>()
                                                                       as
                                                                       libc::c_ulong).wrapping_sub(1
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong)
                                      as libc::c_int,
                              numFields: 0,
                              pFields: 0 as *mut delta_t,
                              customEncode: 0,
                              funcName: [0; 32],
                              userCallback: None,
                              bInitialized: false_0,};
             init
         },
         {
             let mut init =
                 delta_info_t{pName:
                                  b"clientdata_t\x00" as *const u8 as
                                      *const libc::c_char,
                              pInfo: cd_fields.as_ptr(),
                              maxFields:
                                  (::std::mem::size_of::<[delta_field_t; 57]>()
                                       as
                                       libc::c_ulong).wrapping_div(::std::mem::size_of::<delta_field_t>()
                                                                       as
                                                                       libc::c_ulong).wrapping_sub(1
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong)
                                      as libc::c_int,
                              numFields: 0,
                              pFields: 0 as *mut delta_t,
                              customEncode: 0,
                              funcName: [0; 32],
                              userCallback: None,
                              bInitialized: false_0,};
             init
         },
         {
             let mut init =
                 delta_info_t{pName:
                                  b"weapon_data_t\x00" as *const u8 as
                                      *const libc::c_char,
                              pInfo: wd_fields.as_ptr(),
                              maxFields:
                                  (::std::mem::size_of::<[delta_field_t; 23]>()
                                       as
                                       libc::c_ulong).wrapping_div(::std::mem::size_of::<delta_field_t>()
                                                                       as
                                                                       libc::c_ulong).wrapping_sub(1
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong)
                                      as libc::c_int,
                              numFields: 0,
                              pFields: 0 as *mut delta_t,
                              customEncode: 0,
                              funcName: [0; 32],
                              userCallback: None,
                              bInitialized: false_0,};
             init
         },
         {
             let mut init =
                 delta_info_t{pName:
                                  b"entity_state_t\x00" as *const u8 as
                                      *const libc::c_char,
                              pInfo: ent_fields.as_ptr(),
                              maxFields:
                                  (::std::mem::size_of::<[delta_field_t; 92]>()
                                       as
                                       libc::c_ulong).wrapping_div(::std::mem::size_of::<delta_field_t>()
                                                                       as
                                                                       libc::c_ulong).wrapping_sub(1
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong)
                                      as libc::c_int,
                              numFields: 0,
                              pFields: 0 as *mut delta_t,
                              customEncode: 0,
                              funcName: [0; 32],
                              userCallback: None,
                              bInitialized: false_0,};
             init
         },
         {
             let mut init =
                 delta_info_t{pName:
                                  b"entity_state_player_t\x00" as *const u8 as
                                      *const libc::c_char,
                              pInfo: ent_fields.as_ptr(),
                              maxFields:
                                  (::std::mem::size_of::<[delta_field_t; 92]>()
                                       as
                                       libc::c_ulong).wrapping_div(::std::mem::size_of::<delta_field_t>()
                                                                       as
                                                                       libc::c_ulong).wrapping_sub(1
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong)
                                      as libc::c_int,
                              numFields: 0,
                              pFields: 0 as *mut delta_t,
                              customEncode: 0,
                              funcName: [0; 32],
                              userCallback: None,
                              bInitialized: false_0,};
             init
         },
         {
             let mut init =
                 delta_info_t{pName:
                                  b"custom_entity_state_t\x00" as *const u8 as
                                      *const libc::c_char,
                              pInfo: ent_fields.as_ptr(),
                              maxFields:
                                  (::std::mem::size_of::<[delta_field_t; 92]>()
                                       as
                                       libc::c_ulong).wrapping_div(::std::mem::size_of::<delta_field_t>()
                                                                       as
                                                                       libc::c_ulong).wrapping_sub(1
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong)
                                      as libc::c_int,
                              numFields: 0,
                              pFields: 0 as *mut delta_t,
                              customEncode: 0,
                              funcName: [0; 32],
                              userCallback: None,
                              bInitialized: false_0,};
             init
         },
         {
             let mut init =
                 delta_info_t{pName: 0 as *const libc::c_char,
                              pInfo: 0 as *const delta_field_t,
                              maxFields: 0,
                              numFields: 0,
                              pFields: 0 as *mut delta_t,
                              customEncode: 0,
                              funcName: [0; 32],
                              userCallback: None,
                              bInitialized: false_0,};
             init
         }]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
