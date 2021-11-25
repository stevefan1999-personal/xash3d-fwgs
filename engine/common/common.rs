#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, extern_types,
           ptr_wrapping_offset_from, register_tool)]
extern "C" {
    pub type grasshdr_s;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn Q_strnlwr(in_0: *const libc::c_char, out: *mut libc::c_char,
                 size_out: size_t);
    #[no_mangle]
    fn Q_strlen(string: *const libc::c_char) -> size_t;
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Q_atof(str: *const libc::c_char) -> libc::c_float;
    #[no_mangle]
    fn Q_strchr(s: *const libc::c_char, c: libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn Q_strrchr(s: *const libc::c_char, c: libc::c_char)
     -> *mut libc::c_char;
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
    fn Q_sprintf(buffer: *mut libc::c_char, format: *const libc::c_char,
                 _: ...) -> libc::c_int;
    #[no_mangle]
    fn Q_strpbrk(s: *const libc::c_char, accept: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn COM_FileExtension(in_0: *const libc::c_char) -> *const libc::c_char;
    #[no_mangle]
    fn _COM_ParseFileSafe(data: *mut libc::c_char, token: *mut libc::c_char,
                          size: libc::c_int, flags: libc::c_uint,
                          len: *mut libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn Sys_DoubleTime() -> libc::c_double;
    #[no_mangle]
    fn Sys_CheckParm(parm: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn Cvar_FindVarExt(var_name: *const libc::c_char,
                       ignore_group: libc::c_int) -> *mut convar_t;
    #[no_mangle]
    fn Cvar_RegisterVariable(var: *mut convar_t);
    #[no_mangle]
    fn Cvar_Get(var_name: *const libc::c_char, value: *const libc::c_char,
                flags: libc::c_int, description: *const libc::c_char)
     -> *mut convar_t;
    #[no_mangle]
    fn Cvar_DirectSet(var: *mut convar_t, value: *const libc::c_char);
    #[no_mangle]
    fn Cvar_BuildAutoDescription(flags: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    static mut SI: sysinfo_t;
    #[no_mangle]
    fn _Mem_Alloc(poolptr: poolhandle_t, size: size_t, clear: qboolean,
                  filename: *const libc::c_char, fileline: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    fn _Mem_Free(data: *mut libc::c_void, filename: *const libc::c_char,
                 fileline: libc::c_int);
    #[no_mangle]
    fn Mem_IsAllocatedExt(poolptr: poolhandle_t, data: *mut libc::c_void)
     -> qboolean;
    #[no_mangle]
    fn FS_AddGameHierarchy(dir: *const libc::c_char, flags: uint);
    #[no_mangle]
    fn FS_GetDiskPath(name: *const libc::c_char, gamedironly: qboolean)
     -> *const libc::c_char;
    #[no_mangle]
    fn FS_LoadFile(path: *const libc::c_char, filesizeptr: *mut fs_offset_t,
                   gamedironly: qboolean) -> *mut byte;
    #[no_mangle]
    fn FS_WriteFile(filename: *const libc::c_char, data: *const libc::c_void,
                    len: fs_offset_t) -> qboolean;
    #[no_mangle]
    fn FS_FileSize(filename: *const libc::c_char, gamedironly: qboolean)
     -> fs_offset_t;
    #[no_mangle]
    fn FS_FileTime(filename: *const libc::c_char, gamedironly: qboolean)
     -> libc::c_int;
    #[no_mangle]
    fn Host_CompareFileTime(ft1: libc::c_int, ft2: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn Sequence_Get(fileName: *const libc::c_char,
                    entryName: *const libc::c_char) -> *mut sequenceEntry_s;
    #[no_mangle]
    fn Sequence_PickSentence(groupName: *const libc::c_char,
                             pickMethod: libc::c_int,
                             picked: *mut libc::c_int)
     -> *mut sentenceEntry_s;
}
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type off_t = __off_t;
pub type HANDLE = *mut libc::c_void;
pub type HINSTANCE = *mut libc::c_void;
pub type time_t = __time_t;
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
pub type fs_offset_t = off_t;
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
pub type sequenceEntry_s = sequenceEntry_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sequenceEntry_ {
    pub fileName: *mut libc::c_char,
    pub entryName: *mut libc::c_char,
    pub firstCommand: *mut sequenceCommandLine_s,
    pub nextEntry: *mut sequenceEntry_s,
    pub isGlobal: qboolean,
}
pub type sequenceCommandLine_s = sequenceCommandLine_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sequenceCommandLine_ {
    pub commandType: libc::c_int,
    pub clientMessage: client_textmessage_t,
    pub speakerName: *mut libc::c_char,
    pub listenerName: *mut libc::c_char,
    pub soundFileName: *mut libc::c_char,
    pub sentenceName: *mut libc::c_char,
    pub fireTargetNames: *mut libc::c_char,
    pub killTargetNames: *mut libc::c_char,
    pub delay: libc::c_float,
    pub repeatCount: libc::c_int,
    pub textChannel: libc::c_int,
    pub modifierBitField: libc::c_int,
    pub nextCommandLine: *mut sequenceCommandLine_s,
}
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
pub type sentenceEntry_s = sentenceEntry_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sentenceEntry_ {
    pub data: *mut libc::c_char,
    pub nextEntry: *mut sentenceEntry_s,
    pub isGlobal: qboolean,
    pub index: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzss_header_t {
    pub id: libc::c_uint,
    pub size: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzss_state_t {
    pub hash_table: *mut lzss_list_t,
    pub hash_node: *mut lzss_node_t,
    pub window_size: libc::c_int,
}
// expected to be sixteen bytes
pub type lzss_node_t = lzss_node_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzss_node_s {
    pub data: *const byte,
    pub prev: *mut lzss_node_s,
    pub next: *mut lzss_node_s,
    pub pad: [libc::c_char; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzss_list_t {
    pub start: *mut lzss_node_t,
    pub end: *mut lzss_node_t,
}
/*
common.c - misc functions used by dlls'
Copyright (C) 2008 Uncle Mike

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
*/
static mut file_exts: [*const libc::c_char; 10] =
    [b"cfg\x00" as *const u8 as *const libc::c_char,
     b"lst\x00" as *const u8 as *const libc::c_char,
     b"exe\x00" as *const u8 as *const libc::c_char,
     b"vbs\x00" as *const u8 as *const libc::c_char,
     b"com\x00" as *const u8 as *const libc::c_char,
     b"bat\x00" as *const u8 as *const libc::c_char,
     b"dll\x00" as *const u8 as *const libc::c_char,
     b"ini\x00" as *const u8 as *const libc::c_char,
     b"log\x00" as *const u8 as *const libc::c_char,
     b"sys\x00" as *const u8 as *const libc::c_char];
// DEBUG
static mut idum: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn lran1() -> libc::c_int {
    static mut iy: libc::c_int = 0 as libc::c_int;
    static mut iv: [libc::c_int; 32] = [0; 32];
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    if idum <= 0 as libc::c_int || iy == 0 {
        if -idum < 1 as libc::c_int {
            idum = 1 as libc::c_int
        } else { idum = -idum }
        j = 32 as libc::c_int + 7 as libc::c_int;
        while j >= 0 as libc::c_int {
            k = idum / 127773 as libc::c_int;
            idum =
                16807 as libc::c_int * (idum - k * 127773 as libc::c_int) -
                    2836 as libc::c_int * k;
            if idum < 0 as libc::c_int { idum += 2147483647 as libc::c_int }
            if j < 32 as libc::c_int { iv[j as usize] = idum }
            j -= 1
        }
        iy = iv[0 as libc::c_int as usize]
    }
    k = idum / 127773 as libc::c_int;
    idum =
        16807 as libc::c_int * (idum - k * 127773 as libc::c_int) -
            2836 as libc::c_int * k;
    if idum < 0 as libc::c_int { idum += 2147483647 as libc::c_int }
    j =
        iy /
            (1 as libc::c_int +
                 (2147483647 as libc::c_int - 1 as libc::c_int) /
                     32 as libc::c_int);
    iy = iv[j as usize];
    iv[j as usize] = idum;
    return iy;
}
// fran1 -- return a random floating-point number on the interval [0,1]
unsafe extern "C" fn fran1() -> libc::c_float {
    let mut temp: libc::c_float =
        (1.0f64 / 2147483647 as libc::c_int as libc::c_double) as
            libc::c_float * lran1() as libc::c_float; // float in [0,1]
    if temp as libc::c_double > 1.0f64 - 1.2e-7f64 {
        return (1.0f64 - 1.2e-7f64) as libc::c_float
    }
    return temp;
}
#[no_mangle]
pub unsafe extern "C" fn COM_SetRandomSeed(mut lSeed: libc::c_int) {
    if lSeed != 0 {
        idum = lSeed
    } else { idum = -time(0 as *mut time_t) as libc::c_int }
    if (1000 as libc::c_int) < idum {
        idum = -idum
    } else if (-(1000 as libc::c_int)) < idum {
        idum -= 22261048 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn COM_RandomFloat(mut flLow: libc::c_float,
                                         mut flHigh: libc::c_float)
 -> libc::c_float {
    let mut fl: libc::c_float = 0.;
    if idum == 0 as libc::c_int { COM_SetRandomSeed(0 as libc::c_int); }
    fl = fran1();
    return fl * (flHigh - flLow) + flLow;
    // float in [low, high)
}
#[no_mangle]
pub unsafe extern "C" fn COM_RandomLong(mut lLow: libc::c_int,
                                        mut lHigh: libc::c_int)
 -> libc::c_int {
    let mut maxAcceptable: dword = 0;
    let mut n: dword = 0;
    let mut x: dword = (lHigh - lLow + 1 as libc::c_int) as dword;
    if idum == 0 as libc::c_int { COM_SetRandomSeed(0 as libc::c_int); }
    if x <= 0 as libc::c_int as libc::c_uint ||
           (0x7fffffff as libc::c_ulong) <
               x.wrapping_sub(1 as libc::c_int as libc::c_uint) as
                   libc::c_ulong {
        return lLow
    }
    // The following maps a uniform distribution on the interval [0, MAX_RANDOM_RANGE]
	// to a smaller, client-specified range of [0,x-1] in a way that doesn't bias
	// the uniform distribution unfavorably. Even for a worst case x, the loop is
	// guaranteed to be taken no more than half the time, so for that worst case x,
	// the average number of times through the loop is 2. For cases where x is
	// much smaller than MAX_RANDOM_RANGE, the average number of times through the
	// loop is very close to 1.
    maxAcceptable =
        (0x7fffffff as
             libc::c_ulong).wrapping_sub((0x7fffffff as
                                              libc::c_ulong).wrapping_add(1 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_ulong).wrapping_rem(x
                                                                                                              as
                                                                                                              libc::c_ulong))
            as
            dword; // allocate the output buffer, compressed buffer is expected to be less, caller will free
    loop  {
        n = lran1() as dword; // prevent compression failure
        if !(n > maxAcceptable) { break ; }
    }
    return (lLow as libc::c_uint).wrapping_add(n.wrapping_rem(x)) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn LZSS_IsCompressed(mut source: *const byte)
 -> qboolean {
    let mut phdr: *mut lzss_header_t = source as *mut lzss_header_t;
    if !phdr.is_null() &&
           (*phdr).id ==
               (('S' as i32) << 24 as libc::c_int |
                    ('S' as i32) << 16 as libc::c_int |
                    ('Z' as i32) << 8 as libc::c_int | 'L' as i32) as
                   libc::c_uint {
        return true_0
    }
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn LZSS_GetActualSize(mut source: *const byte) -> uint {
    let mut phdr: *mut lzss_header_t = source as *mut lzss_header_t;
    if !phdr.is_null() &&
           (*phdr).id ==
               (('S' as i32) << 24 as libc::c_int |
                    ('S' as i32) << 16 as libc::c_int |
                    ('Z' as i32) << 8 as libc::c_int | 'L' as i32) as
                   libc::c_uint {
        return (*phdr).size
    }
    return 0 as libc::c_int as uint;
}
unsafe extern "C" fn LZSS_BuildHash(mut state: *mut lzss_state_t,
                                    mut source: *const byte) {
    let mut list: *mut lzss_list_t = 0 as *mut lzss_list_t;
    let mut node: *mut lzss_node_t = 0 as *mut lzss_node_t;
    let mut targetindex: libc::c_uint =
        source as uint &
            ((*state).window_size - 1 as libc::c_int) as libc::c_uint;
    node =
        &mut *(*state).hash_node.offset(targetindex as isize) as
            *mut lzss_node_t;
    if !(*node).data.is_null() {
        list =
            &mut *(*state).hash_table.offset(*(*node).data as isize) as
                *mut lzss_list_t;
        if !(*node).prev.is_null() {
            (*list).end = (*node).prev;
            (*(*node).prev).next = 0 as *mut lzss_node_s
        } else {
            (*list).start = 0 as *mut lzss_node_t;
            (*list).end = 0 as *mut lzss_node_t
        }
    }
    list =
        &mut *(*state).hash_table.offset(*source as isize) as
            *mut lzss_list_t;
    (*node).data = source;
    (*node).prev = 0 as *mut lzss_node_s;
    (*node).next = (*list).start;
    if !(*list).start.is_null() {
        (*(*list).start).prev = node
    } else { (*list).end = node }
    (*list).start = node;
}
#[no_mangle]
pub unsafe extern "C" fn LZSS_CompressNoAlloc(mut state: *mut lzss_state_t,
                                              mut pInput: *mut byte,
                                              mut input_length: libc::c_int,
                                              mut pOutputBuf: *mut byte,
                                              mut pOutputSize: *mut uint)
 -> *mut byte {
    let mut pStart: *mut byte = pOutputBuf;
    let mut pEnd: *mut byte =
        pStart.offset(input_length as
                          isize).offset(-(::std::mem::size_of::<lzss_header_t>()
                                              as libc::c_ulong as
                                              isize)).offset(-(8 as
                                                                   libc::c_int
                                                                   as isize));
    let mut header: *mut lzss_header_t = pStart as *mut lzss_header_t;
    let mut pOutput: *mut byte =
        pStart.offset(::std::mem::size_of::<lzss_header_t>() as libc::c_ulong
                          as isize);
    let mut pEncodedPosition: *const byte = 0 as *const byte;
    let mut pLookAhead: *mut byte = pInput;
    let mut pWindow: *mut byte = pInput;
    let mut i: libc::c_int = 0;
    let mut putCmdByte: libc::c_int = 0 as libc::c_int;
    let mut pCmdByte: *mut byte = 0 as *mut byte;
    if input_length as libc::c_ulong <=
           (::std::mem::size_of::<lzss_header_t>() as
                libc::c_ulong).wrapping_add(8 as libc::c_int as libc::c_ulong)
       {
        return 0 as *mut byte
    }
    // set LZSS header
    (*header).id =
        (('S' as i32) << 24 as libc::c_int | ('S' as i32) << 16 as libc::c_int
             | ('Z' as i32) << 8 as libc::c_int | 'L' as i32) as libc::c_uint;
    (*header).size = input_length as libc::c_uint;
    // create the compression work buffers, small enough (~64K) for stack
    let mut fresh0 =
        ::std::vec::from_elem(0,
                              (256 as libc::c_int as
                                   libc::c_ulong).wrapping_mul(::std::mem::size_of::<lzss_list_t>()
                                                                   as
                                                                   libc::c_ulong)
                                  as usize);
    (*state).hash_table = fresh0.as_mut_ptr() as *mut lzss_list_t;
    memset((*state).hash_table as *mut libc::c_void, 0 as libc::c_int,
           (256 as libc::c_int as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<lzss_list_t>()
                                                as libc::c_ulong));
    let mut fresh1 =
        ::std::vec::from_elem(0,
                              ((*state).window_size as
                                   libc::c_ulong).wrapping_mul(::std::mem::size_of::<lzss_node_t>()
                                                                   as
                                                                   libc::c_ulong)
                                  as usize);
    (*state).hash_node = fresh1.as_mut_ptr() as *mut lzss_node_t;
    memset((*state).hash_node as *mut libc::c_void, 0 as libc::c_int,
           ((*state).window_size as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<lzss_node_t>()
                                                as libc::c_ulong));
    while input_length > 0 as libc::c_int {
        let mut lookAheadLength: libc::c_int =
            if (input_length as libc::c_uint) <
                   (1 as libc::c_uint) << 4 as libc::c_int {
                input_length as libc::c_uint
            } else { ((1 as libc::c_uint)) << 4 as libc::c_int } as
                libc::c_int;
        let mut hash: *mut lzss_node_t =
            (*(*state).hash_table.offset(*pLookAhead.offset(0 as libc::c_int
                                                                as isize) as
                                             isize)).start;
        let mut encoded_length: libc::c_int = 0 as libc::c_int;
        pWindow = pLookAhead.offset(-((*state).window_size as isize));
        if pWindow < pInput { pWindow = pInput }
        if putCmdByte == 0 {
            let fresh2 = pOutput;
            pOutput = pOutput.offset(1);
            pCmdByte = fresh2;
            *pCmdByte = 0 as libc::c_int as byte
        }
        putCmdByte = putCmdByte + 1 as libc::c_int & 0x7 as libc::c_int;
        while !hash.is_null() {
            let mut length: libc::c_int = lookAheadLength;
            let mut match_length: libc::c_int = 0 as libc::c_int;
            loop  {
                let fresh3 = length;
                length = length - 1;
                if !(fresh3 != 0 &&
                         *(*hash).data.offset(match_length as isize) as
                             libc::c_int ==
                             *pLookAhead.offset(match_length as isize) as
                                 libc::c_int) {
                    break ;
                }
                match_length += 1
            }
            if match_length > encoded_length {
                encoded_length = match_length;
                pEncodedPosition = (*hash).data
            }
            if match_length == lookAheadLength { break ; }
            hash = (*hash).next
        }
        if encoded_length >= 3 as libc::c_int {
            *pCmdByte =
                (*pCmdByte as libc::c_int >> 1 as libc::c_int |
                     0x80 as libc::c_int) as byte;
            let fresh4 = pOutput;
            pOutput = pOutput.offset(1);
            *fresh4 =
                (pLookAhead.wrapping_offset_from(pEncodedPosition) as
                     libc::c_long - 1 as libc::c_int as libc::c_long >>
                     4 as libc::c_int) as byte;
            let fresh5 = pOutput;
            pOutput = pOutput.offset(1);
            *fresh5 =
                ((pLookAhead.wrapping_offset_from(pEncodedPosition) as
                      libc::c_long - 1 as libc::c_int as libc::c_long) <<
                     4 as libc::c_int |
                     (encoded_length - 1 as libc::c_int) as libc::c_long) as
                    byte
        } else {
            *pCmdByte =
                (*pCmdByte as libc::c_int >> 1 as libc::c_int) as byte;
            let fresh6 = pOutput;
            pOutput = pOutput.offset(1);
            *fresh6 = *pLookAhead;
            encoded_length = 1 as libc::c_int
        }
        i = 0 as libc::c_int;
        while i < encoded_length {
            let fresh7 = pLookAhead;
            pLookAhead = pLookAhead.offset(1);
            LZSS_BuildHash(state, fresh7);
            i += 1
        }
        input_length -= encoded_length;
        if pOutput >= pEnd {
            // compression is worse, abandon
            return 0 as *mut byte
        }
    }
    if input_length != 0 as libc::c_int {
        // unexpected failure
        return 0 as *mut byte
    }
    if putCmdByte == 0 {
        let fresh8 = pOutput;
        pOutput = pOutput.offset(1);
        pCmdByte = fresh8;
        *pCmdByte = 0x1 as libc::c_int as byte
    } else {
        *pCmdByte =
            ((*pCmdByte as libc::c_int >> 1 as libc::c_int |
                  0x80 as libc::c_int) >> 7 as libc::c_int - putCmdByte) as
                byte
    }
    // put two ints at end of buffer
    let fresh9 = pOutput;
    pOutput = pOutput.offset(1);
    *fresh9 = 0 as libc::c_int as byte;
    let fresh10 = pOutput;
    pOutput = pOutput.offset(1);
    *fresh10 = 0 as libc::c_int as byte;
    if !pOutputSize.is_null() {
        *pOutputSize =
            pOutput.wrapping_offset_from(pStart) as libc::c_long as uint
    }
    return pStart;
}
#[no_mangle]
pub unsafe extern "C" fn LZSS_Compress(mut pInput: *mut byte,
                                       mut inputLength: libc::c_int,
                                       mut pOutputSize: *mut uint)
 -> *mut byte {
    let mut pStart: *mut byte =
        malloc(inputLength as libc::c_ulong) as *mut byte;
    let mut pFinal: *mut byte = 0 as *mut byte;
    let mut state: lzss_state_t =
        lzss_state_t{hash_table: 0 as *mut lzss_list_t,
                     hash_node: 0 as *mut lzss_node_t,
                     window_size: 0,};
    memset(&mut state as *mut lzss_state_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<lzss_state_t>() as libc::c_ulong);
    state.window_size = 4096 as libc::c_int;
    pFinal =
        LZSS_CompressNoAlloc(&mut state, pInput, inputLength, pStart,
                             pOutputSize);
    if pFinal.is_null() {
        free(pStart as *mut libc::c_void);
        return 0 as *mut byte
    }
    return pStart;
}
#[no_mangle]
pub unsafe extern "C" fn LZSS_Decompress(mut pInput: *const byte,
                                         mut pOutput: *mut byte) -> uint {
    let mut totalBytes: uint = 0 as libc::c_int as uint;
    let mut getCmdByte: libc::c_int = 0 as libc::c_int;
    let mut cmdByte: libc::c_int = 0 as libc::c_int;
    let mut actualSize: uint = LZSS_GetActualSize(pInput);
    if actualSize == 0 { return 0 as libc::c_int as uint }
    pInput =
        pInput.offset(::std::mem::size_of::<lzss_header_t>() as libc::c_ulong
                          as isize);
    loop  {
        if getCmdByte == 0 {
            let fresh11 = pInput;
            pInput = pInput.offset(1);
            cmdByte = *fresh11 as libc::c_int
        }
        getCmdByte = getCmdByte + 1 as libc::c_int & 0x7 as libc::c_int;
        if cmdByte & 0x1 as libc::c_int != 0 {
            let fresh12 = pInput;
            pInput = pInput.offset(1);
            let mut position: libc::c_int =
                (*fresh12 as libc::c_int) << 4 as libc::c_int;
            let mut i: libc::c_int = 0;
            let mut count: libc::c_int = 0;
            let mut pSource: *mut byte = 0 as *mut byte;
            position |= *pInput as libc::c_int >> 4 as libc::c_int;
            let fresh13 = pInput;
            pInput = pInput.offset(1);
            count =
                (*fresh13 as libc::c_int & 0xf as libc::c_int) +
                    1 as libc::c_int;
            if count == 1 as libc::c_int { break ; }
            pSource =
                pOutput.offset(-(position as
                                     isize)).offset(-(1 as libc::c_int as
                                                          isize));
            i = 0 as libc::c_int;
            while i < count {
                let fresh14 = pSource;
                pSource = pSource.offset(1);
                let fresh15 = pOutput;
                pOutput = pOutput.offset(1);
                *fresh15 = *fresh14;
                i += 1
            }
            totalBytes =
                (totalBytes as
                     libc::c_uint).wrapping_add(count as libc::c_uint) as uint
                    as uint
        } else {
            let fresh16 = pInput;
            pInput = pInput.offset(1);
            let fresh17 = pOutput;
            pOutput = pOutput.offset(1);
            *fresh17 = *fresh16;
            totalBytes = totalBytes.wrapping_add(1)
        }
        cmdByte = cmdByte >> 1 as libc::c_int
    }
    if totalBytes != actualSize { return 0 as libc::c_int as uint }
    return totalBytes;
}
/*
==============
COM_IsWhiteSpace

interpret symbol as whitespace
==============
*/
unsafe extern "C" fn COM_IsWhiteSpace(mut space: libc::c_char)
 -> libc::c_int {
    if space as libc::c_int == ' ' as i32 ||
           space as libc::c_int == '\t' as i32 ||
           space as libc::c_int == '\r' as i32 ||
           space as libc::c_int == '\n' as i32 {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/*
================
COM_ParseVector

================
*/
#[no_mangle]
pub unsafe extern "C" fn COM_ParseVector(mut pfile: *mut *mut libc::c_char,
                                         mut v: *mut libc::c_float,
                                         mut size: size_t) -> qboolean {
    let mut token: string = [0; 256]; // restore token to right get it again
    let mut bracket: qboolean = false_0; // done
    let mut saved: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: uint = 0;
    if v.is_null() || size == 0 as libc::c_int as libc::c_ulong {
        return false_0
    }
    memset(v as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<libc::c_float>() as
                libc::c_ulong).wrapping_mul(size));
    if size == 1 as libc::c_int as libc::c_ulong {
        *pfile =
            _COM_ParseFileSafe(*pfile, token.as_mut_ptr(),
                               ::std::mem::size_of::<string>() as
                                   libc::c_ulong as libc::c_int,
                               0 as libc::c_int as libc::c_uint,
                               0 as *mut libc::c_int);
        *v.offset(0 as libc::c_int as isize) = Q_atof(token.as_mut_ptr());
        return true_0
    }
    saved = *pfile;
    *pfile =
        _COM_ParseFileSafe(*pfile, token.as_mut_ptr(),
                           ::std::mem::size_of::<string>() as libc::c_ulong as
                               libc::c_int, 0 as libc::c_int as libc::c_uint,
                           0 as *mut libc::c_int);
    if (*pfile).is_null() { return false_0 }
    if token[0 as libc::c_int as usize] as libc::c_int == '(' as i32 {
        bracket = true_0
    } else { *pfile = saved }
    i = 0 as libc::c_int as uint;
    while (i as libc::c_ulong) < size {
        *pfile =
            _COM_ParseFileSafe(*pfile, token.as_mut_ptr(),
                               ::std::mem::size_of::<string>() as
                                   libc::c_ulong as libc::c_int,
                               0 as libc::c_int as libc::c_uint,
                               0 as *mut libc::c_int);
        *v.offset(i as isize) = Q_atof(token.as_mut_ptr());
        i = i.wrapping_add(1)
    }
    if bracket as u64 == 0 { return true_0 }
    *pfile =
        _COM_ParseFileSafe(*pfile, token.as_mut_ptr(),
                           ::std::mem::size_of::<string>() as libc::c_ulong as
                               libc::c_int, 0 as libc::c_int as libc::c_uint,
                           0 as *mut libc::c_int);
    if (*pfile).is_null() { return false_0 }
    if token[0 as libc::c_int as usize] as libc::c_int == ')' as i32 {
        return true_0
    }
    return false_0;
}
/*
=============
COM_FileSize

=============
*/
#[no_mangle]
pub unsafe extern "C" fn COM_FileSize(mut filename: *const libc::c_char)
 -> libc::c_int {
    return FS_FileSize(filename, false_0) as libc::c_int;
}
/*
=============
COM_AddAppDirectoryToSearchPath

=============
*/
#[no_mangle]
pub unsafe extern "C" fn COM_AddAppDirectoryToSearchPath(mut pszBaseDir:
                                                             *const libc::c_char,
                                                         mut appName:
                                                             *const libc::c_char) {
    FS_AddGameHierarchy(pszBaseDir, (1 as libc::c_uint) << 1 as libc::c_int);
}
/*
===========
COM_ExpandFilename

Finds the file in the search path, copies over the name with the full path name.
This doesn't search in the pak file.
===========
*/
#[no_mangle]
pub unsafe extern "C" fn COM_ExpandFilename(mut fileName: *const libc::c_char,
                                            mut nameOutBuffer:
                                                *mut libc::c_char,
                                            mut nameOutBufferSize:
                                                libc::c_int) -> libc::c_int {
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    let mut result: [libc::c_char; 1024] = [0; 1024];
    if (if fileName.is_null() || *fileName == 0 {
            0 as libc::c_int
        } else { 1 as libc::c_int }) == 0 || nameOutBuffer.is_null() ||
           nameOutBufferSize <= 0 as libc::c_int {
        return 0 as libc::c_int
    }
    // filename examples:
	// media\sierra.avi - D:\Xash3D\valve\media\sierra.avi
	// models\barney.mdl - D:\Xash3D\bshift\models\barney.mdl
    path = FS_GetDiskPath(fileName, false_0);
    if !path.is_null() {
        Q_sprintf(result.as_mut_ptr(),
                  b"%s/%s\x00" as *const u8 as *const libc::c_char,
                  host.rootdir.as_mut_ptr(), path);
        // check for enough room
        if Q_strlen(result.as_mut_ptr()) > nameOutBufferSize as libc::c_ulong
           {
            return 0 as libc::c_int
        }
        Q_strncpy(nameOutBuffer, result.as_mut_ptr(),
                  nameOutBufferSize as size_t);
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/*
=============
COM_TrimSpace

trims all whitespace from the front
and end of a string
=============
*/
#[no_mangle]
pub unsafe extern "C" fn COM_TrimSpace(mut source: *const libc::c_char,
                                       mut dest: *mut libc::c_char) {
    let mut start: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    start = 0 as libc::c_int;
    end = Q_strlen(source) as libc::c_int;
    while *source.offset(start as isize) as libc::c_int != 0 &&
              COM_IsWhiteSpace(*source.offset(start as isize)) != 0 {
        start += 1
    }
    end -= 1;
    while end > 0 as libc::c_int &&
              COM_IsWhiteSpace(*source.offset(end as isize)) != 0 {
        end -= 1
    }
    end += 1;
    length = end - start;
    if length > 0 as libc::c_int {
        memcpy(dest as *mut libc::c_void,
               source.offset(start as isize) as *const libc::c_void,
               length as libc::c_ulong);
    } else { length = 0 as libc::c_int }
    // terminate the dest string
    *dest.offset(length as isize) = 0 as libc::c_int as libc::c_char;
}
/*
============
COM_FixSlashes

Changes all '/' characters into '\' characters, in place.
============
*/
#[no_mangle]
pub unsafe extern "C" fn COM_FixSlashes(mut pname: *mut libc::c_char) {
    while *pname != 0 {
        if *pname as libc::c_int == '\\' as i32 {
            *pname = '/' as i32 as libc::c_char
        }
        pname = pname.offset(1)
    };
}
/*
==================
COM_Nibble

Returns the 4 bit nibble for a hex character
==================
*/
#[no_mangle]
pub unsafe extern "C" fn COM_Nibble(mut c: libc::c_char) -> byte {
    if c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32 {
        return (c as libc::c_int - '0' as i32) as byte
    }
    if c as libc::c_int >= 'A' as i32 && c as libc::c_int <= 'F' as i32 {
        return (c as libc::c_int - 'A' as i32 + 0xa as libc::c_int) as byte
    }
    if c as libc::c_int >= 'a' as i32 && c as libc::c_int <= 'f' as i32 {
        return (c as libc::c_int - 'a' as i32 + 0xa as libc::c_int) as byte
    }
    return '0' as i32 as byte;
}
/*
==================
COM_HexConvert

Converts pszInput Hex string to nInputLength/2 binary
==================
*/
#[no_mangle]
pub unsafe extern "C" fn COM_HexConvert(mut pszInput: *const libc::c_char,
                                        mut nInputLength: libc::c_int,
                                        mut pOutput: *mut byte) {
    let mut pIn: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *mut byte = pOutput;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < nInputLength {
        pIn = &*pszInput.offset(i as isize) as *const libc::c_char;
        *p =
            ((COM_Nibble(*pIn.offset(0 as libc::c_int as isize)) as
                  libc::c_int) << 4 as libc::c_int |
                 COM_Nibble(*pIn.offset(1 as libc::c_int as isize)) as
                     libc::c_int) as byte;
        p = p.offset(1);
        i += 2 as libc::c_int
    };
}
/*
=============
COM_MemFgets

=============
*/
#[no_mangle]
pub unsafe extern "C" fn COM_MemFgets(mut pMemFile: *mut byte,
                                      mut fileSize: libc::c_int,
                                      mut filePos: *mut libc::c_int,
                                      mut pBuffer: *mut libc::c_char,
                                      mut bufferSize: libc::c_int)
 -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    let mut last: libc::c_int = 0;
    let mut stop: libc::c_int = 0;
    if pMemFile.is_null() || pBuffer.is_null() || filePos.is_null() {
        return 0 as *mut libc::c_char
    }
    if *filePos >= fileSize { return 0 as *mut libc::c_char }
    i = *filePos;
    last = fileSize;
    // fgets always NULL terminates, so only read bufferSize-1 characters
    if last - *filePos > bufferSize - 1 as libc::c_int {
        last = *filePos + (bufferSize - 1 as libc::c_int)
    }
    stop = 0 as libc::c_int;
    // stop at the next newline (inclusive) or end of buffer
    while i < last && stop == 0 {
        if *pMemFile.offset(i as isize) as libc::c_int == '\n' as i32 {
            stop = 1 as libc::c_int
        }
        i += 1
    }
    // if we actually advanced the pointer, copy it over
    if i != *filePos {
        // we read in size bytes
        let mut size: libc::c_int = i - *filePos;
        // copy it out
        memcpy(pBuffer as *mut libc::c_void,
               pMemFile.offset(*filePos as isize) as *const libc::c_void,
               size as libc::c_ulong);
        // If the buffer isn't full, terminate (this is always true)
        if size < bufferSize {
            *pBuffer.offset(size as isize) = 0 as libc::c_int as libc::c_char
        }
        // update file pointer
        *filePos = i;
        return pBuffer
    }
    return 0 as *mut libc::c_char;
}
/*
====================
Cache_Check

consistency check
====================
*/
#[no_mangle]
pub unsafe extern "C" fn Cache_Check(mut mempool: poolhandle_t,
                                     mut c: *mut cache_user_t)
 -> *mut libc::c_void {
    if (*c).data.is_null() { return 0 as *mut libc::c_void }
    if Mem_IsAllocatedExt(mempool, (*c).data) as u64 == 0 {
        return 0 as *mut libc::c_void
    }
    return (*c).data;
}
/*
=============
COM_LoadFileForMe

=============
*/
#[no_mangle]
pub unsafe extern "C" fn COM_LoadFileForMe(mut filename: *const libc::c_char,
                                           mut pLength: *mut libc::c_int)
 -> *mut byte {
    let mut name: string = [0; 256];
    let mut file: *mut byte = 0 as *mut byte;
    let mut pfile: *mut byte = 0 as *mut byte;
    let mut iLength: fs_offset_t = 0;
    if if filename.is_null() || *filename == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        if !pLength.is_null() { *pLength = 0 as libc::c_int }
        return 0 as *mut byte
    }
    Q_strncpy(name.as_mut_ptr(), filename,
              ::std::mem::size_of::<string>() as libc::c_ulong);
    COM_FixSlashes(name.as_mut_ptr());
    pfile = FS_LoadFile(name.as_mut_ptr(), &mut iLength, false_0);
    if !pLength.is_null() { *pLength = iLength as libc::c_int }
    if !pfile.is_null() {
        file =
            malloc((iLength + 1 as libc::c_int as libc::c_long) as
                       libc::c_ulong) as *mut byte;
        if !file.is_null() {
            memcpy(file as *mut libc::c_void, pfile as *const libc::c_void,
                   iLength as libc::c_ulong);
            *file.offset(iLength as isize) = '\u{0}' as i32 as byte
        }
        _Mem_Free(pfile as *mut libc::c_void,
                  b"../engine/common/common.c\x00" as *const u8 as
                      *const libc::c_char, 755 as libc::c_int);
        pfile = file
    }
    return pfile;
}
/*
=============
COM_LoadFile

=============
*/
#[no_mangle]
pub unsafe extern "C" fn COM_LoadFile(mut filename: *const libc::c_char,
                                      mut usehunk: libc::c_int,
                                      mut pLength: *mut libc::c_int)
 -> *mut byte {
    return COM_LoadFileForMe(filename, pLength);
}
/*
=============
COM_SaveFile

=============
*/
#[no_mangle]
pub unsafe extern "C" fn COM_SaveFile(mut filename: *const libc::c_char,
                                      mut data: *const libc::c_void,
                                      mut len: libc::c_int) -> libc::c_int {
    // check for empty filename
    if if filename.is_null() || *filename == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return false_0 as libc::c_int
    }
    // check for null data
    if data.is_null() || len <= 0 as libc::c_int {
        return false_0 as libc::c_int
    }
    return FS_WriteFile(filename, data, len as fs_offset_t) as libc::c_int;
}
/*
=============
COM_FreeFile

=============
*/
#[no_mangle]
pub unsafe extern "C" fn COM_FreeFile(mut buffer: *mut libc::c_void) {
    free(buffer);
}
/*
=============
COM_NormalizeAngles

=============
*/
#[no_mangle]
pub unsafe extern "C" fn COM_NormalizeAngles(mut angles: *mut vec_t) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if *angles.offset(i as isize) > 180.0f32 {
            let ref mut fresh18 = *angles.offset(i as isize);
            *fresh18 -= 360.0f32
        } else if *angles.offset(i as isize) < -180.0f32 {
            let ref mut fresh19 = *angles.offset(i as isize);
            *fresh19 += 360.0f32
        }
        i += 1
    };
}
/*
=============
pfnGetModelType

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnGetModelType(mut mod_0: *mut model_t)
 -> libc::c_int {
    if mod_0.is_null() { return mod_bad as libc::c_int }
    return (*mod_0).type_0 as libc::c_int;
}
/*
=============
pfnGetModelBounds

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnGetModelBounds(mut mod_0: *mut model_t,
                                           mut mins: *mut libc::c_float,
                                           mut maxs: *mut libc::c_float) {
    if !mod_0.is_null() {
        if !mins.is_null() {
            *mins.offset(0 as libc::c_int as isize) =
                (*mod_0).mins[0 as libc::c_int as usize];
            *mins.offset(1 as libc::c_int as isize) =
                (*mod_0).mins[1 as libc::c_int as usize];
            *mins.offset(2 as libc::c_int as isize) =
                (*mod_0).mins[2 as libc::c_int as usize]
        }
        if !maxs.is_null() {
            *maxs.offset(0 as libc::c_int as isize) =
                (*mod_0).maxs[0 as libc::c_int as usize];
            *maxs.offset(1 as libc::c_int as isize) =
                (*mod_0).maxs[1 as libc::c_int as usize];
            *maxs.offset(2 as libc::c_int as isize) =
                (*mod_0).maxs[2 as libc::c_int as usize]
        }
    } else {
        if !mins.is_null() {
            let ref mut fresh20 = *mins.offset(2 as libc::c_int as isize);
            *fresh20 = 0 as libc::c_int as libc::c_float;
            let ref mut fresh21 = *mins.offset(1 as libc::c_int as isize);
            *fresh21 = *fresh20;
            *mins.offset(0 as libc::c_int as isize) = *fresh21
        }
        if !maxs.is_null() {
            let ref mut fresh22 = *maxs.offset(2 as libc::c_int as isize);
            *fresh22 = 0 as libc::c_int as libc::c_float;
            let ref mut fresh23 = *maxs.offset(1 as libc::c_int as isize);
            *fresh23 = *fresh22;
            *maxs.offset(0 as libc::c_int as isize) = *fresh23
        }
    };
}
/*
=============
pfnCvar_RegisterServerVariable

standard path to register game variable
=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnCvar_RegisterServerVariable(mut variable:
                                                            *mut cvar_t) {
    if !variable.is_null() {
        (*variable).flags =
            (*variable).flags | (1 as libc::c_int) << 3 as libc::c_int
    }
    Cvar_RegisterVariable(variable as *mut convar_t);
}
/*
=============
pfnCvar_RegisterEngineVariable

use with precaution: this cvar will NOT unlinked
after game.dll is unloaded
=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnCvar_RegisterEngineVariable(mut variable:
                                                            *mut cvar_t) {
    Cvar_RegisterVariable(variable as *mut convar_t);
}
/*
=============
pfnCvar_RegisterVariable

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnCvar_RegisterClientVariable(mut szName:
                                                            *const libc::c_char,
                                                        mut szValue:
                                                            *const libc::c_char,
                                                        mut flags:
                                                            libc::c_int)
 -> *mut cvar_t {
    // a1ba: try to mitigate outdated client.dll vulnerabilities
    if Q_strnicmp(szName, b"motdfile\x00" as *const u8 as *const libc::c_char,
                  99999 as libc::c_int) == 0 {
        flags |= (1 as libc::c_int) << 10 as libc::c_int
    }
    if flags & (1 as libc::c_int) << 12 as libc::c_int != 0 {
        return Cvar_Get(szName, szValue, flags,
                        va(b"enable or disable %s\x00" as *const u8 as
                               *const libc::c_char, szName)) as *mut cvar_t
    }
    return Cvar_Get(szName, szValue,
                    flags | (1 as libc::c_int) << 4 as libc::c_int,
                    Cvar_BuildAutoDescription(flags |
                                                  (1 as libc::c_int) <<
                                                      4 as libc::c_int)) as
               *mut cvar_t;
}
/*
=============
pfnCvar_RegisterVariable

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnCvar_RegisterGameUIVariable(mut szName:
                                                            *const libc::c_char,
                                                        mut szValue:
                                                            *const libc::c_char,
                                                        mut flags:
                                                            libc::c_int)
 -> *mut cvar_t {
    if flags & (1 as libc::c_int) << 12 as libc::c_int != 0 {
        return Cvar_Get(szName, szValue, flags,
                        va(b"enable or disable %s\x00" as *const u8 as
                               *const libc::c_char, szName)) as *mut cvar_t
    }
    return Cvar_Get(szName, szValue,
                    flags | (1 as libc::c_int) << 14 as libc::c_int,
                    Cvar_BuildAutoDescription(flags |
                                                  (1 as libc::c_int) <<
                                                      14 as libc::c_int)) as
               *mut cvar_t;
}
/*
=============
pfnCVarGetPointer

can return NULL
=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnCVarGetPointer(mut szVarName: *const libc::c_char)
 -> *mut cvar_t {
    return Cvar_FindVarExt(szVarName, 0 as libc::c_int) as *mut cvar_t;
}
/*
=============
pfnCVarDirectSet

allow to set cvar directly
=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnCVarDirectSet(mut var: *mut cvar_t,
                                          mut szValue: *const libc::c_char) {
    Cvar_DirectSet(var as *mut convar_t, szValue);
}
/*
=============
COM_CompareFileTime

=============
*/
#[no_mangle]
pub unsafe extern "C" fn COM_CompareFileTime(mut filename1:
                                                 *const libc::c_char,
                                             mut filename2:
                                                 *const libc::c_char,
                                             mut iCompare: *mut libc::c_int)
 -> libc::c_int {
    let mut bRet: libc::c_int = 0 as libc::c_int;
    *iCompare = 0 as libc::c_int;
    if !filename1.is_null() && !filename2.is_null() {
        let mut ft1: libc::c_int = FS_FileTime(filename1, false_0);
        let mut ft2: libc::c_int = FS_FileTime(filename2, false_0);
        // one of files is missing
        if ft1 == -(1 as libc::c_int) || ft2 == -(1 as libc::c_int) {
            return bRet
        }
        *iCompare = Host_CompareFileTime(ft1, ft2);
        bRet = 1 as libc::c_int
    }
    return bRet;
}
/*
=============
COM_CheckParm

=============
*/
#[no_mangle]
pub unsafe extern "C" fn COM_CheckParm(mut parm: *mut libc::c_char,
                                       mut ppnext: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut i: libc::c_int = Sys_CheckParm(parm);
    if !ppnext.is_null() {
        if i != 0 as libc::c_int && i < host.argc - 1 as libc::c_int {
            *ppnext = *host.argv.offset((i + 1 as libc::c_int) as isize)
        } else { *ppnext = 0 as *mut libc::c_char }
    }
    return i;
}
/*
=============
pfnTime

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnTime() -> libc::c_float {
    return Sys_DoubleTime() as libc::c_float;
}
/*
=============
pfnGetGameDir

=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnGetGameDir(mut szGetGameDir: *mut libc::c_char) {
    if szGetGameDir.is_null() { return }
    Q_strncpy(szGetGameDir, (*SI.GameInfo).gamefolder.as_mut_ptr(),
              99999 as libc::c_int as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn COM_IsSafeFileToDownload(mut filename:
                                                      *const libc::c_char)
 -> qboolean {
    let mut lwrfilename: [libc::c_char; 4096] = [0; 4096];
    let mut first: *const libc::c_char = 0 as *const libc::c_char;
    let mut last: *const libc::c_char = 0 as *const libc::c_char;
    let mut ext: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    if if filename.is_null() || *filename == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return false_0
    }
    if Q_strncmp(filename, b"!MD5\x00" as *const u8 as *const libc::c_char,
                 4 as libc::c_int) == 0 {
        return true_0
    }
    Q_strnlwr(filename, lwrfilename.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong);
    if !Q_strpbrk(lwrfilename.as_mut_ptr(),
                  b"\\:~\x00" as *const u8 as *const libc::c_char).is_null()
           ||
           !Q_strstr(lwrfilename.as_mut_ptr(),
                     b"..\x00" as *const u8 as *const libc::c_char).is_null()
       {
        return false_0
    }
    if lwrfilename[0 as libc::c_int as usize] as libc::c_int == '/' as i32 {
        return false_0
    }
    first = Q_strchr(lwrfilename.as_mut_ptr(), '.' as i32 as libc::c_char);
    last = Q_strrchr(lwrfilename.as_mut_ptr(), '.' as i32 as libc::c_char);
    if first.is_null() || last.is_null() { return false_0 }
    if first != last { return false_0 }
    if Q_strlen(first) != 4 as libc::c_int as libc::c_ulong { return false_0 }
    ext = COM_FileExtension(lwrfilename.as_mut_ptr());
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[*const libc::c_char; 10]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<*const libc::c_char>()
                                                   as libc::c_ulong) {
        if Q_strnicmp(ext, file_exts[i as usize], 99999 as libc::c_int) == 0 {
            return false_0
        }
        i += 1
    }
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn _copystring(mut mempool: poolhandle_t,
                                     mut s: *const libc::c_char,
                                     mut filename: *const libc::c_char,
                                     mut fileline: libc::c_int)
 -> *mut libc::c_char {
    let mut b: *mut libc::c_char = 0 as *mut libc::c_char;
    if s.is_null() { return 0 as *mut libc::c_char }
    if mempool == 0 { mempool = host.mempool }
    b =
        _Mem_Alloc(mempool,
                   Q_strlen(s).wrapping_add(1 as libc::c_int as
                                                libc::c_ulong), false_0,
                   filename, fileline) as *mut libc::c_char;
    Q_strncpy(b, s, 99999 as libc::c_int as size_t);
    return b;
}
/*
======================

COMMON EXPORT STUBS

======================
*/
/*
=============
pfnSequenceGet

used by CS:CZ
=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnSequenceGet(mut fileName: *const libc::c_char,
                                        mut entryName: *const libc::c_char)
 -> *mut libc::c_void {
    Con_Printf(b"Sequence_Get: file %s, entry %s\n\x00" as *const u8 as
                   *const libc::c_char, fileName, entryName);
    return Sequence_Get(fileName, entryName) as *mut libc::c_void;
}
/*
=============
pfnSequencePickSentence

used by CS:CZ
=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnSequencePickSentence(mut groupName:
                                                     *const libc::c_char,
                                                 mut pickMethod: libc::c_int,
                                                 mut picked: *mut libc::c_int)
 -> *mut libc::c_void {
    Con_Printf(b"Sequence_PickSentence: group %s, pickMethod %i\n\x00" as
                   *const u8 as *const libc::c_char, groupName, pickMethod);
    return Sequence_PickSentence(groupName, pickMethod, picked) as
               *mut libc::c_void;
}
/*
=============
pfnIsCareerMatch

used by CS:CZ (client stub)
=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnIsCareerMatch() -> libc::c_int {
    return 0 as libc::c_int;
}
/*
=============
pfnRegisterTutorMessageShown

only exists in PlayStation version
=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnRegisterTutorMessageShown(mut mid: libc::c_int) {
}
/*
=============
pfnGetTimesTutorMessageShown

only exists in PlayStation version
=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnGetTimesTutorMessageShown(mut mid: libc::c_int)
 -> libc::c_int {
    return 0 as libc::c_int;
}
/*
=============
pfnProcessTutorMessageDecayBuffer

only exists in PlayStation version
=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnProcessTutorMessageDecayBuffer(mut buffer:
                                                               *mut libc::c_int,
                                                           mut bufferLength:
                                                               libc::c_int) {
}
/*
=============
pfnConstructTutorMessageDecayBuffer

only exists in PlayStation version
=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnConstructTutorMessageDecayBuffer(mut buffer:
                                                                 *mut libc::c_int,
                                                             mut bufferLength:
                                                                 libc::c_int) {
}
/*
=============
pfnResetTutorMessageDecayData

only exists in PlayStation version
=============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnResetTutorMessageDecayData() { }
