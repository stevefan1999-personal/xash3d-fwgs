#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types,
           ptr_wrapping_offset_from, register_tool)]
extern "C" {
    pub type file_s;
    pub type grasshdr_s;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn Q_strlen(string: *const libc::c_char) -> size_t;
    #[no_mangle]
    fn Q_strncat(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Q_atof(str: *const libc::c_char) -> libc::c_float;
    #[no_mangle]
    fn Q_strchr(s: *const libc::c_char, c: libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn Q_strnicmp(s1: *const libc::c_char, s2: *const libc::c_char,
                  n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Q_strncmp(s1: *const libc::c_char, s2: *const libc::c_char,
                 n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Q_stricmpext(s1: *const libc::c_char, s2: *const libc::c_char)
     -> qboolean;
    #[no_mangle]
    fn Q_strpbrk(s: *const libc::c_char, accept: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn _COM_ParseFileSafe(data: *mut libc::c_char, token: *mut libc::c_char,
                          size: libc::c_int, flags: libc::c_uint,
                          len: *mut libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn matchpattern_with_separator(in_0: *const libc::c_char,
                                   pattern: *const libc::c_char,
                                   caseinsensitive: qboolean,
                                   separators: *const libc::c_char,
                                   wildcard_least_one: qboolean)
     -> libc::c_int;
    #[no_mangle]
    fn Cvar_GetList() -> *mut cvar_t;
    #[no_mangle]
    fn Cvar_FindVarExt(var_name: *const libc::c_char,
                       ignore_group: libc::c_int) -> *mut convar_t;
    #[no_mangle]
    fn Cvar_VariableInteger(var_name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn Cvar_VariableString(var_name: *const libc::c_char)
     -> *const libc::c_char;
    #[no_mangle]
    fn Cvar_CommandWithPrivilegeCheck(v: *mut convar_t,
                                      isPrivileged: qboolean) -> qboolean;
    #[no_mangle]
    static mut cmd_scripting: *mut convar_t;
    #[no_mangle]
    static mut cl_filterstuffcmd: convar_t;
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    fn Host_Error(error: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_Reportf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn _Mem_Free(data: *mut libc::c_void, filename: *const libc::c_char,
                 fileline: libc::c_int);
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn _copystring(mempool: poolhandle_t, s: *const libc::c_char,
                   filename: *const libc::c_char, fileline: libc::c_int)
     -> *mut libc::c_char;
    #[no_mangle]
    fn Con_DPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn _Mem_Alloc(poolptr: poolhandle_t, size: size_t, clear: qboolean,
                  filename: *const libc::c_char, fileline: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    static mut cl: client_t;
    #[no_mangle]
    static mut cls: client_static_t;
    #[no_mangle]
    fn MSG_WriteString(sb: *mut sizebuf_t, pStr: *const libc::c_char)
     -> qboolean;
    #[no_mangle]
    fn MSG_WriteCmdExt(sb: *mut sizebuf_t, cmd: libc::c_int, type_0: netsrc_t,
                       name: *const libc::c_char);
    #[no_mangle]
    fn BaseCmd_Find(type_0: base_command_type_e, name: *const libc::c_char)
     -> *mut libc::c_void;
    #[no_mangle]
    fn BaseCmd_FindAll(name: *const libc::c_char, cmd: *mut *mut libc::c_void,
                       alias: *mut *mut libc::c_void,
                       cvar: *mut *mut libc::c_void);
    #[no_mangle]
    fn BaseCmd_Insert(type_0: base_command_type_e, basecmd: *mut libc::c_void,
                      name: *const libc::c_char);
    #[no_mangle]
    fn BaseCmd_Remove(type_0: base_command_type_e, name: *const libc::c_char);
    #[no_mangle]
    fn BaseCmd_Stats_f();
    #[no_mangle]
    fn BaseCmd_Test_f();
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
pub type setpair_t
    =
    Option<unsafe extern "C" fn(_: *const libc::c_char,
                                _: *const libc::c_void, _: *mut libc::c_void,
                                _: *mut libc::c_void) -> ()>;
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
pub struct cmdbuf_t {
    pub data: *mut byte,
    pub cursize: libc::c_int,
    pub maxsize: libc::c_int,
}
/*
=============================================================================

			COMMAND EXECUTION

=============================================================================
*/
pub type cmd_t = cmd_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmd_s {
    pub next: *mut cmd_s,
    pub name: *mut libc::c_char,
    pub function: xcommand_t,
    pub desc: *mut libc::c_char,
    pub flags: libc::c_int,
}
pub type base_command_type_e = base_command_type;
pub type base_command_type = libc::c_uint;
pub const HM_CMDALIAS: base_command_type = 3;
pub const HM_CMD: base_command_type = 2;
pub const HM_CVAR: base_command_type = 1;
pub const HM_DONTCARE: base_command_type = 0;
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
pub type cmdalias_t = cmdalias_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmdalias_s {
    pub next: *mut cmdalias_s,
    pub name: [libc::c_char; 32],
    pub value: *mut libc::c_char,
}
pub type base_command_t = ();
#[no_mangle]
pub static mut cmd_wait: qboolean = false_0;
#[no_mangle]
pub static mut cmd_text: cmdbuf_t =
    cmdbuf_t{data: 0 as *const byte as *mut byte, cursize: 0, maxsize: 0,};
#[no_mangle]
pub static mut filteredcmd_text: cmdbuf_t =
    cmdbuf_t{data: 0 as *const byte as *mut byte, cursize: 0, maxsize: 0,};
#[no_mangle]
pub static mut cmd_text_buf: [byte; 32768] = [0; 32768];
#[no_mangle]
pub static mut filteredcmd_text_buf: [byte; 32768] = [0; 32768];
#[no_mangle]
pub static mut cmd_alias: *mut cmdalias_t =
    0 as *const cmdalias_t as *mut cmdalias_t;
#[no_mangle]
pub static mut cmd_condition: uint = 0;
#[no_mangle]
pub static mut cmd_condlevel: libc::c_int = 0;
static mut cmd_currentCommandIsPrivileged: qboolean = false_0;
/*
=============================================================================

			COMMAND BUFFER

=============================================================================
*/
/*
============
Cbuf_Init
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cbuf_Init() {
    cmd_text.data = cmd_text_buf.as_mut_ptr();
    filteredcmd_text.data = filteredcmd_text_buf.as_mut_ptr();
    cmd_text.maxsize = 32768 as libc::c_int;
    filteredcmd_text.maxsize = cmd_text.maxsize;
    cmd_text.cursize = 0 as libc::c_int;
    filteredcmd_text.cursize = cmd_text.cursize;
}
/*
============
Cbuf_Clear
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cbuf_Clear() {
    memset(cmd_text.data as *mut libc::c_void, 0 as libc::c_int,
           cmd_text.maxsize as libc::c_ulong);
    memset(filteredcmd_text.data as *mut libc::c_void, 0 as libc::c_int,
           filteredcmd_text.maxsize as libc::c_ulong);
    filteredcmd_text.cursize = 0 as libc::c_int;
    cmd_text.cursize = filteredcmd_text.cursize;
}
/*
============
Cbuf_GetSpace
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cbuf_GetSpace(mut buf: *mut cmdbuf_t,
                                       mut length: libc::c_int)
 -> *mut libc::c_void {
    let mut data: *mut libc::c_void = 0 as *mut libc::c_void;
    if (*buf).cursize + length > (*buf).maxsize {
        (*buf).cursize = 0 as libc::c_int;
        Host_Error(b"Cbuf_GetSpace: overflow\n\x00" as *const u8 as
                       *const libc::c_char);
    }
    data = (*buf).data.offset((*buf).cursize as isize) as *mut libc::c_void;
    (*buf).cursize += length;
    return data;
}
unsafe extern "C" fn Cbuf_AddTextToBuffer(mut buf: *mut cmdbuf_t,
                                          mut text: *const libc::c_char) {
    let mut l: libc::c_int = Q_strlen(text) as libc::c_int;
    if (*buf).cursize + l >= (*buf).maxsize {
        Con_Reportf(b"^3Warning:^7 %s: overflow\n\x00" as *const u8 as
                        *const libc::c_char,
                    (*::std::mem::transmute::<&[u8; 21],
                                              &[libc::c_char; 21]>(b"Cbuf_AddTextToBuffer\x00")).as_ptr());
        return
    }
    memcpy(Cbuf_GetSpace(buf, l), text as *const libc::c_void,
           l as libc::c_ulong);
}
/*
============
Cbuf_AddText

Adds command text at the end of the buffer
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cbuf_AddText(mut text: *const libc::c_char) {
    Cbuf_AddTextToBuffer(&mut cmd_text, text);
}
/*
============
Cbuf_AddFilteredText
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cbuf_AddFilteredText(mut text: *const libc::c_char) {
    Cbuf_AddTextToBuffer(&mut filteredcmd_text, text);
}
/*
============
Cbuf_InsertText

Adds command text immediately after the current command
Adds a \n to the text
============
*/
unsafe extern "C" fn Cbuf_InsertTextToBuffer(mut buf: *mut cmdbuf_t,
                                             mut text: *const libc::c_char) {
    let mut l: libc::c_int = Q_strlen(text) as libc::c_int;
    if (*buf).cursize + l >= (*buf).maxsize {
        Con_Reportf(b"^3Warning:^7 Cbuf_InsertText: overflow\n\x00" as
                        *const u8 as *const libc::c_char);
    } else {
        memmove((*buf).data.offset(l as isize) as *mut libc::c_void,
                (*buf).data as *const libc::c_void,
                (*buf).cursize as libc::c_ulong);
        memcpy((*buf).data as *mut libc::c_void, text as *const libc::c_void,
               l as libc::c_ulong);
        (*buf).cursize += l
    };
}
#[no_mangle]
pub unsafe extern "C" fn Cbuf_InsertText(mut text: *const libc::c_char) {
    Cbuf_InsertTextToBuffer(&mut cmd_text, text);
}
/*
============
Cbuf_Execute
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cbuf_ExecuteCommandsFromBuffer(mut buf:
                                                            *mut cmdbuf_t,
                                                        mut isPrivileged:
                                                            qboolean,
                                                        mut cmdsToExecute:
                                                            libc::c_int) {
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line: [libc::c_char; 2048] = [0; 2048];
    let mut i: libc::c_int = 0;
    let mut quotes: libc::c_int = 0;
    let mut comment: *mut libc::c_char = 0 as *mut libc::c_char;
    while (*buf).cursize != 0 {
        // limit amount of commands that can be issued
        if cmdsToExecute >= 0 as libc::c_int {
            let fresh0 = cmdsToExecute;
            cmdsToExecute = cmdsToExecute - 1;
            if fresh0 == 0 { break ; }
        }
        // find a \n or ; line break
        text = (*buf).data as *mut libc::c_char;
        quotes = false_0 as libc::c_int;
        comment = 0 as *mut libc::c_char;
        i = 0 as libc::c_int;
        while i < (*buf).cursize {
            if comment.is_null() {
                if *text.offset(i as isize) as libc::c_int == '\"' as i32 {
                    quotes = (quotes == 0) as libc::c_int
                }
                if quotes != 0 {
                    // make sure i doesn't get > cursize which causes a negative size in memmove, which is fatal --blub
                    if i < (*buf).cursize - 1 as libc::c_int &&
                           (*text.offset((i + 0 as libc::c_int) as isize) as
                                libc::c_int == '\\' as i32 &&
                                (*text.offset((i + 1 as libc::c_int) as isize)
                                     as libc::c_int == '\"' as i32 ||
                                     *text.offset((i + 1 as libc::c_int) as
                                                      isize) as libc::c_int ==
                                         '\\' as i32)) {
                        i += 1
                    }
                } else {
                    if *text.offset((i + 0 as libc::c_int) as isize) as
                           libc::c_int == '/' as i32 &&
                           *text.offset((i + 1 as libc::c_int) as isize) as
                               libc::c_int == '/' as i32 &&
                           (i == 0 as libc::c_int ||
                                *text.offset((i - 1 as libc::c_int) as isize)
                                    as byte as libc::c_int <= ' ' as i32) {
                        comment =
                            &mut *text.offset(i as isize) as *mut libc::c_char
                    }
                    if *text.offset(i as isize) as libc::c_int == ';' as i32 {
                        break ;
                    }
                    // don't break if inside a quoted string or comment
                }
            }
            if *text.offset(i as isize) as libc::c_int == '\n' as i32 ||
                   *text.offset(i as isize) as libc::c_int == '\r' as i32 {
                break ;
            }
            i += 1
        }
        if i >= 2048 as libc::c_int - 1 as libc::c_int {
            Con_DPrintf(b"^1Error:^7 Cbuf_Execute: command string owerflow\n\x00"
                            as *const u8 as *const libc::c_char);
            line[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char
        } else {
            memcpy(line.as_mut_ptr() as *mut libc::c_void,
                   text as *const libc::c_void,
                   if !comment.is_null() {
                       comment.wrapping_offset_from(text) as libc::c_long
                   } else { i as libc::c_long } as libc::c_ulong);
            line[if !comment.is_null() {
                     comment.wrapping_offset_from(text) as libc::c_long
                 } else { i as libc::c_long } as usize] =
                0 as libc::c_int as libc::c_char
        }
        // delete the text from the command buffer and move remaining commands down
		// this is necessary because commands (exec) can insert data at the
		// beginning of the text buffer
        if i == (*buf).cursize {
            (*buf).cursize = 0 as libc::c_int
        } else {
            i += 1;
            (*buf).cursize -= i;
            memmove((*buf).data as *mut libc::c_void,
                    text.offset(i as isize) as *const libc::c_void,
                    (*buf).cursize as libc::c_ulong);
        }
        // execute the command line
        Cmd_ExecuteStringWithPrivilegeCheck(line.as_mut_ptr(), isPrivileged);
        if !(cmd_wait as u64 != 0) { continue ; }
        // skip out while text still remains in buffer,
			// leaving it for next frame
        cmd_wait = false_0;
        break ;
    };
}
/*
============
Cbuf_Execute
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cbuf_Execute() {
    Cbuf_ExecuteCommandsFromBuffer(&mut cmd_text, true_0,
                                   -(1 as libc::c_int));
    // a1ba: unlimited commands for filtered buffer per frame
	// I don't see any sense in restricting that at this moment
	// but in future we may limit this
    Cbuf_ExecuteCommandsFromBuffer(&mut filteredcmd_text, false_0,
                                   -(1 as libc::c_int));
}
/*
===============
Cbuf_ExecStuffCmds

execute commandline
===============
*/
#[no_mangle]
pub unsafe extern "C" fn Cbuf_ExecStuffCmds() {
    let mut build: [libc::c_char; 2048] =
        [0;
            2048]; // this is for all commandline options combined (and is bounds checked)
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut l: libc::c_int = 0 as libc::c_int;
    // no reason to run the commandline arguments twice
    if host.stuffcmds_pending as u64 == 0 { return }
    build[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    i = 0 as libc::c_int;
    while i < host.argc {
        if !(*host.argv.offset(i as isize)).is_null() &&
               *(*host.argv.offset(i as
                                       isize)).offset(0 as libc::c_int as
                                                          isize) as
                   libc::c_int == '+' as i32 &&
               ((*(*host.argv.offset(i as
                                         isize)).offset(1 as libc::c_int as
                                                            isize) as
                     libc::c_int) < '0' as i32 ||
                    *(*host.argv.offset(i as
                                            isize)).offset(1 as libc::c_int as
                                                               isize) as
                        libc::c_int > '9' as i32) &&
               (l as
                    libc::c_ulong).wrapping_add(Q_strlen(*host.argv.offset(i
                                                                               as
                                                                               isize))).wrapping_sub(1
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_ulong)
                   <=
                   (::std::mem::size_of::<[libc::c_char; 2048]>() as
                        libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                        libc::c_ulong) {
            j = 1 as libc::c_int;
            while *(*host.argv.offset(i as isize)).offset(j as isize) != 0 {
                let fresh1 = j;
                j = j + 1;
                let fresh2 = l;
                l = l + 1;
                build[fresh2 as usize] =
                    *(*host.argv.offset(i as isize)).offset(fresh1 as isize)
            }
            i += 1;
            while i < host.argc {
                if !(*host.argv.offset(i as isize)).is_null() {
                    if (*(*host.argv.offset(i as
                                                isize)).offset(0 as
                                                                   libc::c_int
                                                                   as isize)
                            as libc::c_int == '+' as i32 ||
                            *(*host.argv.offset(i as
                                                    isize)).offset(0 as
                                                                       libc::c_int
                                                                       as
                                                                       isize)
                                as libc::c_int == '-' as i32) &&
                           ((*(*host.argv.offset(i as
                                                     isize)).offset(1 as
                                                                        libc::c_int
                                                                        as
                                                                        isize)
                                 as libc::c_int) < '0' as i32 ||
                                *(*host.argv.offset(i as
                                                        isize)).offset(1 as
                                                                           libc::c_int
                                                                           as
                                                                           isize)
                                    as libc::c_int > '9' as i32) {
                        break ;
                    }
                    if (l as
                            libc::c_ulong).wrapping_add(Q_strlen(*host.argv.offset(i
                                                                                       as
                                                                                       isize))).wrapping_add(4
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 as
                                                                                                                 libc::c_ulong)
                           >
                           (::std::mem::size_of::<[libc::c_char; 2048]>() as
                                libc::c_ulong).wrapping_sub(1 as libc::c_int
                                                                as
                                                                libc::c_ulong)
                       {
                        break ;
                    }
                    let fresh3 = l;
                    l = l + 1;
                    build[fresh3 as usize] = ' ' as i32 as libc::c_char;
                    if !Q_strchr(*host.argv.offset(i as isize),
                                 ' ' as i32 as libc::c_char).is_null() {
                        let fresh4 = l;
                        l = l + 1;
                        build[fresh4 as usize] = '\"' as i32 as libc::c_char
                    }
                    j = 0 as libc::c_int;
                    while *(*host.argv.offset(i as isize)).offset(j as isize)
                              != 0 {
                        let fresh5 = l;
                        l = l + 1;
                        build[fresh5 as usize] =
                            *(*host.argv.offset(i as
                                                    isize)).offset(j as
                                                                       isize);
                        j += 1
                    }
                    if !Q_strchr(*host.argv.offset(i as isize),
                                 ' ' as i32 as libc::c_char).is_null() {
                        let fresh6 = l;
                        l = l + 1;
                        build[fresh6 as usize] = '\"' as i32 as libc::c_char
                    }
                }
                i += 1
            }
            let fresh7 = l;
            l = l + 1;
            build[fresh7 as usize] = '\n' as i32 as libc::c_char;
            i -= 1
        }
        i += 1
    }
    // now terminate the combined string and prepend it to the command buffer
	// we already reserved space for the terminator
    let fresh8 = l; // apply now
    l = l + 1;
    build[fresh8 as usize] = 0 as libc::c_int as libc::c_char;
    Cbuf_InsertText(build.as_mut_ptr());
    Cbuf_Execute();
    // this command can be called only from .rc
    Cmd_RemoveCommand(b"stuffcmds\x00" as *const u8 as *const libc::c_char);
    host.stuffcmds_pending = false_0;
}
/*
==============================================================================

			SCRIPT COMMANDS

==============================================================================
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_CurrentCommandIsPrivileged() -> qboolean {
    return cmd_currentCommandIsPrivileged;
}
/*
===============
Cmd_StuffCmds_f

Adds command line parameters as script statements
Commands lead with a +, and continue until a - or another +
hl.exe -dev 3 +map c1a0d
hl.exe -nosound -game bshift
===============
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_StuffCmds_f() {
    host.stuffcmds_pending = true_0;
}
/*
============
Cmd_Wait_f

Causes execution of the remainder of the command buffer to be delayed until
next frame.  This allows commands like:
bind g "cmd use rocket ; +attack ; wait ; -attack ; cmd use blaster"
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_Wait_f() { cmd_wait = true_0; }
/*
===============
Cmd_Echo_f

Just prints the rest of the line to the console
===============
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_Echo_f() {
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < Cmd_Argc() {
        Con_Printf(b"%s\x00" as *const u8 as *const libc::c_char,
                   Cmd_Argv(i));
        i += 1
    }
    Con_Printf(b"\n\x00" as *const u8 as *const libc::c_char);
}
/*
===============
Cmd_Alias_f

Creates a new command that executes a command string (possibly ; seperated)
===============
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_Alias_f() {
    let mut a: *mut cmdalias_t = 0 as *mut cmdalias_t;
    let mut cmd: [libc::c_char; 2048] = [0; 2048];
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    if Cmd_Argc() == 1 as libc::c_int {
        Con_Printf(b"Current alias commands:\n\x00" as *const u8 as
                       *const libc::c_char);
        a = cmd_alias;
        while !a.is_null() {
            Con_Printf(b"^2%s^7 : ^3%s^7\n\x00" as *const u8 as
                           *const libc::c_char, (*a).name.as_mut_ptr(),
                       (*a).value);
            a = (*a).next
        }
        return
    }
    s = Cmd_Argv(1 as libc::c_int);
    if Q_strlen(s) >= 32 as libc::c_int as libc::c_ulong {
        Con_Printf(b"Alias name is too long\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    // if the alias already exists, reuse it
    a = cmd_alias;
    while !a.is_null() {
        if Q_strncmp(s, (*a).name.as_mut_ptr(), 99999 as libc::c_int) == 0 {
            if !(*a).value.is_null() {
                _Mem_Free((*a).value as *mut libc::c_void,
                          b"../engine/common/cmd.c\x00" as *const u8 as
                              *const libc::c_char, 419 as libc::c_int);
            }
            break ;
        } else { a = (*a).next }
    }
    if a.is_null() {
        let mut cur: *mut cmdalias_t = 0 as *mut cmdalias_t;
        let mut prev: *mut cmdalias_t = 0 as *mut cmdalias_t;
        a =
            _Mem_Alloc(host.mempool,
                       ::std::mem::size_of::<cmdalias_t>() as libc::c_ulong,
                       false_0,
                       b"../engine/common/cmd.c\x00" as *const u8 as
                           *const libc::c_char, 428 as libc::c_int) as
                *mut cmdalias_t;
        Q_strncpy((*a).name.as_mut_ptr(), s,
                  ::std::mem::size_of::<[libc::c_char; 32]>() as
                      libc::c_ulong);
        // insert it at the right alphanumeric position
        prev = 0 as *mut cmdalias_t;
        cur = cmd_alias;
        while !cur.is_null() &&
                  Q_strncmp((*cur).name.as_mut_ptr(), (*a).name.as_mut_ptr(),
                            99999 as libc::c_int) < 0 as libc::c_int {
            prev = cur;
            cur = (*cur).next
        }
        if !prev.is_null() { (*prev).next = a } else { cmd_alias = a }
        (*a).next = cur;
        BaseCmd_Insert(HM_CMDALIAS, a as *mut libc::c_void,
                       (*a).name.as_mut_ptr());
    }
    // copy the rest of the command line
    cmd[0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_char; // start out with a null string
    c = Cmd_Argc();
    i = 2 as libc::c_int;
    while i < c {
        if i != 2 as libc::c_int {
            Q_strncat(cmd.as_mut_ptr(),
                      b" \x00" as *const u8 as *const libc::c_char,
                      ::std::mem::size_of::<[libc::c_char; 2048]>() as
                          libc::c_ulong);
        }
        Q_strncat(cmd.as_mut_ptr(), Cmd_Argv(i),
                  ::std::mem::size_of::<[libc::c_char; 2048]>() as
                      libc::c_ulong);
        i += 1
    }
    Q_strncat(cmd.as_mut_ptr(), b"\n\x00" as *const u8 as *const libc::c_char,
              ::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong);
    (*a).value =
        _copystring(host.mempool, cmd.as_mut_ptr(),
                    b"../engine/common/cmd.c\x00" as *const u8 as
                        *const libc::c_char, 456 as libc::c_int);
}
/*
===============
Cmd_UnAlias_f

Remove existing aliases.
===============
*/
unsafe extern "C" fn Cmd_UnAlias_f() {
    let mut a: *mut cmdalias_t = 0 as *mut cmdalias_t;
    let mut p: *mut cmdalias_t = 0 as *mut cmdalias_t;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    if Cmd_Argc() == 1 as libc::c_int {
        Con_Printf(b"Usage: unalias alias1 [alias2 ...]\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    i = 1 as libc::c_int;
    while i < Cmd_Argc() {
        s = Cmd_Argv(i);
        p = 0 as *mut cmdalias_t;
        a = cmd_alias;
        while !a.is_null() {
            if Q_strncmp(s, (*a).name.as_mut_ptr(), 99999 as libc::c_int) == 0
               {
                BaseCmd_Remove(HM_CMDALIAS, (*a).name.as_mut_ptr());
                if a == cmd_alias { cmd_alias = (*a).next }
                if !p.is_null() { (*p).next = (*a).next }
                _Mem_Free((*a).value as *mut libc::c_void,
                          b"../engine/common/cmd.c\x00" as *const u8 as
                              *const libc::c_char, 493 as libc::c_int);
                _Mem_Free(a as *mut libc::c_void,
                          b"../engine/common/cmd.c\x00" as *const u8 as
                              *const libc::c_char, 494 as libc::c_int);
                break ;
            } else { p = a; a = (*a).next }
        }
        if a.is_null() {
            Con_Printf(b"%s not found\n\x00" as *const u8 as
                           *const libc::c_char, s);
        }
        i += 1
    };
}
static mut cmd_argc: libc::c_int = 0;
static mut cmd_args: *const libc::c_char = 0 as *const libc::c_char;
static mut cmd_argv: [*mut libc::c_char; 80] =
    [0 as *const libc::c_char as *mut libc::c_char; 80];
// will have 0 bytes inserted
static mut cmd_functions: *mut cmd_t = 0 as *const cmd_t as *mut cmd_t;
// possible commands to execute
/*
============
Cmd_Argc
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_Argc() -> libc::c_int { return cmd_argc; }
/*
============
Cmd_Argv
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_Argv(mut arg: libc::c_int)
 -> *const libc::c_char {
    if arg as uint >= cmd_argc as libc::c_uint {
        return b"\x00" as *const u8 as *const libc::c_char
    }
    return cmd_argv[arg as usize];
}
/*
============
Cmd_Args
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_Args() -> *const libc::c_char {
    return cmd_args;
}
/*
===========================

Client exports

===========================
*/
/*
============
Cmd_AliasGetList
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_AliasGetList() -> *mut cmdalias_s {
    return cmd_alias;
}
/*
============
Cmd_GetList
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_GetFirstFunctionHandle() -> *mut cmd_s {
    return cmd_functions;
}
/*
============
Cmd_GetNext
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_GetNextFunctionHandle(mut cmd: *mut cmd_t)
 -> *mut cmd_s {
    return if !cmd.is_null() { (*cmd).next } else { 0 as *mut cmd_s };
}
/*
============
Cmd_GetName
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_GetName(mut cmd: *mut cmd_t)
 -> *const libc::c_char {
    return (*cmd).name;
}
/*
============
Cmd_TokenizeString

Parses the given string into command line tokens.
The text is copied to a seperate buffer and 0 characters
are inserted in the apropriate place, The argv array
will point into this temporary buffer.
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_TokenizeString(mut text: *const libc::c_char) {
    let mut cmd_token: [libc::c_char; 32768] = [0; 32768];
    let mut i: libc::c_int = 0;
    // clear the args from the last string
    i = 0 as libc::c_int; // clear previous args
    while i < cmd_argc {
        if !cmd_argv[i as usize].is_null() {
            _Mem_Free(cmd_argv[i as usize] as *mut libc::c_void,
                      b"../engine/common/cmd.c\x00" as *const u8 as
                          *const libc::c_char, 624 as libc::c_int);
        }
        i += 1
    }
    cmd_argc = 0 as libc::c_int;
    cmd_args = 0 as *const libc::c_char;
    if text.is_null() { return }
    loop  {
        // skip whitespace up to a /n
        while *text as libc::c_int != 0 && *text as libc::c_int <= ' ' as i32
                  && *text as libc::c_int != '\r' as i32 &&
                  *text as libc::c_int != '\n' as i32 {
            text = text.offset(1)
        }
        if *text as libc::c_int == '\n' as i32 ||
               *text as libc::c_int == '\r' as i32 {
            // a newline seperates commands in the buffer
            if *text as libc::c_int == '\r' as i32 &&
                   *text.offset(1 as libc::c_int as isize) as libc::c_int ==
                       '\n' as i32 {
                text = text.offset(1)
            }
            text = text.offset(1);
            break ;
        } else {
            if *text == 0 { return }
            if cmd_argc == 1 as libc::c_int { cmd_args = text }
            text =
                _COM_ParseFileSafe(text as *mut libc::c_char,
                                   cmd_token.as_mut_ptr(),
                                   ::std::mem::size_of::<[libc::c_char; 32768]>()
                                       as libc::c_ulong as libc::c_int,
                                   ((1 as libc::c_int) << 0 as libc::c_int) as
                                       libc::c_uint, 0 as *mut libc::c_int);
            if text.is_null() { return }
            if cmd_argc < 80 as libc::c_int {
                cmd_argv[cmd_argc as usize] =
                    _copystring(host.mempool, cmd_token.as_mut_ptr(),
                                b"../engine/common/cmd.c\x00" as *const u8 as
                                    *const libc::c_char, 658 as libc::c_int);
                cmd_argc += 1
            }
        }
    };
}
/*
============
Cmd_AddCommandEx
============
*/
unsafe extern "C" fn Cmd_AddCommandEx(mut funcname: *const libc::c_char,
                                      mut cmd_name: *const libc::c_char,
                                      mut function: xcommand_t,
                                      mut cmd_desc: *const libc::c_char,
                                      mut iFlags: libc::c_int)
 -> libc::c_int {
    let mut cmd: *mut cmd_t = 0 as *mut cmd_t;
    let mut cur: *mut cmd_t = 0 as *mut cmd_t;
    let mut prev: *mut cmd_t = 0 as *mut cmd_t;
    if if cmd_name.is_null() || *cmd_name == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        Con_Reportf(b"^1Error:^7 Cmd_AddCommand: NULL name\n\x00" as *const u8
                        as *const libc::c_char);
        return 0 as libc::c_int
    }
    // fail if the command is a variable name
    if !Cvar_FindVarExt(cmd_name, 0 as libc::c_int).is_null() {
        Con_DPrintf(b"^1Error:^7 Cmd_AddServerCommand: %s already defined as a var\n\x00"
                        as *const u8 as *const libc::c_char, cmd_name);
        return 0 as libc::c_int
    }
    // fail if the command already exists
    if Cmd_Exists(cmd_name) as u64 != 0 {
        Con_DPrintf(b"^1Error:^7 Cmd_AddServerCommand: %s already defined\n\x00"
                        as *const u8 as *const libc::c_char, cmd_name);
        return 0 as libc::c_int
    }
    // use a small malloc to avoid zone fragmentation
    cmd =
        _Mem_Alloc(host.mempool,
                   ::std::mem::size_of::<cmd_t>() as libc::c_ulong, false_0,
                   b"../engine/common/cmd.c\x00" as *const u8 as
                       *const libc::c_char, 695 as libc::c_int) as *mut cmd_t;
    (*cmd).name =
        _copystring(host.mempool, cmd_name,
                    b"../engine/common/cmd.c\x00" as *const u8 as
                        *const libc::c_char, 696 as libc::c_int);
    (*cmd).desc =
        _copystring(host.mempool, cmd_desc,
                    b"../engine/common/cmd.c\x00" as *const u8 as
                        *const libc::c_char, 697 as libc::c_int);
    (*cmd).function = function;
    (*cmd).flags = iFlags;
    // insert it at the right alphanumeric position
    prev = 0 as *mut cmd_t;
    cur = cmd_functions;
    while !cur.is_null() &&
              Q_strncmp((*cur).name, cmd_name, 99999 as libc::c_int) <
                  0 as libc::c_int {
        prev = cur;
        cur = (*cur).next
    }
    if !prev.is_null() { (*prev).next = cmd } else { cmd_functions = cmd }
    (*cmd).next = cur;
    BaseCmd_Insert(HM_CMD, cmd as *mut libc::c_void, (*cmd).name);
    return 1 as libc::c_int;
}
/*
============
Cmd_AddCommand
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_AddCommand(mut cmd_name: *const libc::c_char,
                                        mut function: xcommand_t,
                                        mut cmd_desc: *const libc::c_char) {
    Cmd_AddCommandEx((*::std::mem::transmute::<&[u8; 15],
                                               &[libc::c_char; 15]>(b"Cmd_AddCommand\x00")).as_ptr(),
                     cmd_name, function, cmd_desc, 0 as libc::c_int);
}
/*
============
Cmd_AddRestrictedCommand
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_AddRestrictedCommand(mut cmd_name:
                                                      *const libc::c_char,
                                                  mut function: xcommand_t,
                                                  mut cmd_desc:
                                                      *const libc::c_char) {
    Cmd_AddCommandEx((*::std::mem::transmute::<&[u8; 25],
                                               &[libc::c_char; 25]>(b"Cmd_AddRestrictedCommand\x00")).as_ptr(),
                     cmd_name, function, cmd_desc,
                     ((1 as libc::c_uint) << 3 as libc::c_int) as
                         libc::c_int);
}
/*
============
Cmd_AddServerCommand
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_AddServerCommand(mut cmd_name:
                                                  *const libc::c_char,
                                              mut function: xcommand_t) {
    Cmd_AddCommandEx((*::std::mem::transmute::<&[u8; 21],
                                               &[libc::c_char; 21]>(b"Cmd_AddServerCommand\x00")).as_ptr(),
                     cmd_name, function,
                     b"server command\x00" as *const u8 as
                         *const libc::c_char,
                     ((1 as libc::c_uint) << 0 as libc::c_int) as
                         libc::c_int);
}
/*
============
Cmd_AddClientCommand
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_AddClientCommand(mut cmd_name:
                                                  *const libc::c_char,
                                              mut function: xcommand_t)
 -> libc::c_int {
    let mut flags: libc::c_int =
        ((1 as libc::c_uint) << 1 as libc::c_int) as libc::c_int;
    // a1ba: try to mitigate outdated client.dll vulnerabilities
    if Q_strnicmp(cmd_name,
                  b"motd_write\x00" as *const u8 as *const libc::c_char,
                  99999 as libc::c_int) == 0 {
        flags =
            (flags as libc::c_uint | (1 as libc::c_uint) << 3 as libc::c_int)
                as libc::c_int
    }
    return Cmd_AddCommandEx((*::std::mem::transmute::<&[u8; 21],
                                                      &[libc::c_char; 21]>(b"Cmd_AddClientCommand\x00")).as_ptr(),
                            cmd_name, function,
                            b"client command\x00" as *const u8 as
                                *const libc::c_char, flags);
}
/*
============
Cmd_AddGameUICommand
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_AddGameUICommand(mut cmd_name:
                                                  *const libc::c_char,
                                              mut function: xcommand_t)
 -> libc::c_int {
    return Cmd_AddCommandEx((*::std::mem::transmute::<&[u8; 21],
                                                      &[libc::c_char; 21]>(b"Cmd_AddGameUICommand\x00")).as_ptr(),
                            cmd_name, function,
                            b"gameui command\x00" as *const u8 as
                                *const libc::c_char,
                            ((1 as libc::c_uint) << 2 as libc::c_int) as
                                libc::c_int);
}
/*
============
Cmd_AddRefCommand
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_AddRefCommand(mut cmd_name: *const libc::c_char,
                                           mut function: xcommand_t,
                                           mut description:
                                               *const libc::c_char)
 -> libc::c_int {
    return Cmd_AddCommandEx((*::std::mem::transmute::<&[u8; 18],
                                                      &[libc::c_char; 18]>(b"Cmd_AddRefCommand\x00")).as_ptr(),
                            cmd_name, function, description,
                            ((1 as libc::c_uint) << 5 as libc::c_int) as
                                libc::c_int);
}
/*
============
Cmd_RemoveCommand
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_RemoveCommand(mut cmd_name:
                                               *const libc::c_char) {
    let mut cmd: *mut cmd_t = 0 as *mut cmd_t;
    let mut back: *mut *mut cmd_t = 0 as *mut *mut cmd_t;
    if cmd_name.is_null() || *cmd_name == 0 { return }
    back = &mut cmd_functions;
    loop  {
        cmd = *back;
        if cmd.is_null() { return }
        if Q_strncmp(cmd_name, (*cmd).name, 99999 as libc::c_int) == 0 {
            BaseCmd_Remove(HM_CMD, (*cmd).name);
            *back = (*cmd).next;
            if !(*cmd).name.is_null() {
                _Mem_Free((*cmd).name as *mut libc::c_void,
                          b"../engine/common/cmd.c\x00" as *const u8 as
                              *const libc::c_char, 811 as libc::c_int);
            }
            if !(*cmd).desc.is_null() {
                _Mem_Free((*cmd).desc as *mut libc::c_void,
                          b"../engine/common/cmd.c\x00" as *const u8 as
                              *const libc::c_char, 814 as libc::c_int);
            }
            _Mem_Free(cmd as *mut libc::c_void,
                      b"../engine/common/cmd.c\x00" as *const u8 as
                          *const libc::c_char, 816 as libc::c_int);
            return
        }
        back = &mut (*cmd).next
    };
}
/*
============
Cmd_LookupCmds
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_LookupCmds(mut buffer: *mut libc::c_void,
                                        mut ptr: *mut libc::c_void,
                                        mut callback: setpair_t) {
    let mut cmd: *mut cmd_t = 0 as *mut cmd_t;
    let mut alias: *mut cmdalias_t = 0 as *mut cmdalias_t;
    // nothing to process ?
    if callback.is_none() { return }
    cmd = cmd_functions;
    while !cmd.is_null() {
        if buffer.is_null() {
            callback.expect("non-null function pointer")((*cmd).name,
                                                         ::std::mem::transmute::<xcommand_t,
                                                                                 *mut libc::c_char>((*cmd).function)
                                                             as
                                                             *const libc::c_void,
                                                         (*cmd).desc as
                                                             *mut libc::c_void,
                                                         ptr);
        } else {
            callback.expect("non-null function pointer")((*cmd).name,
                                                         ::std::mem::transmute::<xcommand_t,
                                                                                 *mut libc::c_char>((*cmd).function)
                                                             as
                                                             *const libc::c_void,
                                                         buffer, ptr);
        }
        cmd = (*cmd).next
    }
    // lookup an aliases too
    alias = cmd_alias;
    while !alias.is_null() {
        callback.expect("non-null function pointer")((*alias).name.as_mut_ptr(),
                                                     (*alias).value as
                                                         *const libc::c_void,
                                                     buffer, ptr);
        alias = (*alias).next
    };
}
/*
============
Cmd_Exists
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_Exists(mut cmd_name: *const libc::c_char)
 -> qboolean {
    return (BaseCmd_Find(HM_CMD, cmd_name) != 0 as *mut libc::c_void) as
               libc::c_int as qboolean;
}
/*
============
Cmd_If_f

Compare and et condition bit if true
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_If_f() {
    // reset bit first
    cmd_condition &= !((1 as libc::c_uint) << cmd_condlevel);
    // usage
    if cmd_argc == 1 as libc::c_int {
        Con_Printf(b"Usage: if <op1> [ <operator> <op2> ]\n\x00" as *const u8
                       as *const libc::c_char);
        Con_Printf(b":<action1>\n\x00" as *const u8 as *const libc::c_char);
        Con_Printf(b":<action2>\n\x00" as *const u8 as *const libc::c_char);
        Con_Printf(b"else\n\x00" as *const u8 as *const libc::c_char);
        Con_Printf(b":<action3>\n\x00" as *const u8 as *const libc::c_char);
        Con_Printf(b"operands are string or float values\n\x00" as *const u8
                       as *const libc::c_char);
        Con_Printf(b"and substituted cvars like \'$cl_lw\'\n\x00" as *const u8
                       as *const libc::c_char);
        Con_Printf(b"operator is \'=\'\', \'==\', \'>\', \'<\', \'>=\', \'<=\' or \'!=\'\n\x00"
                       as *const u8 as *const libc::c_char);
        return
    }
    // one argument - check if nonzero
    if cmd_argc == 2 as libc::c_int {
        if Q_atof(cmd_argv[1 as libc::c_int as usize]) != 0. {
            cmd_condition |= (1 as libc::c_uint) << cmd_condlevel
        }
    } else if cmd_argc == 4 as libc::c_int {
        // simple compare
        let mut f1: libc::c_float =
            Q_atof(cmd_argv[1 as libc::c_int as usize]);
        let mut f2: libc::c_float =
            Q_atof(cmd_argv[3 as libc::c_int as usize]);
        if *cmd_argv[2 as libc::c_int as
                         usize].offset(0 as libc::c_int as isize) == 0 {
            // this is wrong
            return
        }
        if *cmd_argv[2 as libc::c_int as
                         usize].offset(0 as libc::c_int as isize) as
               libc::c_int == '=' as i32 ||
               *cmd_argv[2 as libc::c_int as
                             usize].offset(1 as libc::c_int as isize) as
                   libc::c_int == '=' as i32 {
            // =, ==, >=, <=
            if Q_strncmp(cmd_argv[1 as libc::c_int as usize],
                         cmd_argv[3 as libc::c_int as usize],
                         99999 as libc::c_int) == 0 ||
                   (f1 != 0. || f2 != 0.) && f1 == f2 {
                cmd_condition |= (1 as libc::c_uint) << cmd_condlevel
            }
        }
        if *cmd_argv[2 as libc::c_int as
                         usize].offset(0 as libc::c_int as isize) as
               libc::c_int == '!' as i32 {
            // !=
            cmd_condition ^= (1 as libc::c_uint) << cmd_condlevel;
            return
        }
        if *cmd_argv[2 as libc::c_int as
                         usize].offset(0 as libc::c_int as isize) as
               libc::c_int == '>' as i32 && f1 > f2 {
            // >, >=
            cmd_condition |= (1 as libc::c_uint) << cmd_condlevel
        }
        if *cmd_argv[2 as libc::c_int as
                         usize].offset(0 as libc::c_int as isize) as
               libc::c_int == '<' as i32 && f1 < f2 {
            // <, <=
            cmd_condition |= (1 as libc::c_uint) << cmd_condlevel
        }
    };
}
/*
============
Cmd_Else_f

Invert condition bit
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_Else_f() {
    cmd_condition ^= (1 as libc::c_uint) << cmd_condlevel;
}
unsafe extern "C" fn Cmd_ShouldAllowCommand(mut cmd: *mut cmd_t,
                                            mut isPrivileged: qboolean)
 -> qboolean {
    let mut prefixes: [*const libc::c_char; 5] =
        [b"cl_\x00" as *const u8 as *const libc::c_char,
         b"gl_\x00" as *const u8 as *const libc::c_char,
         b"r_\x00" as *const u8 as *const libc::c_char,
         b"m_\x00" as *const u8 as *const libc::c_char,
         b"hud_\x00" as *const u8 as *const libc::c_char];
    let mut i: libc::c_int = 0;
    // always allow local commands
    if isPrivileged as u64 != 0 { return true_0 }
    // never allow local only commands from remote
    if (*cmd).flags as libc::c_uint & (1 as libc::c_uint) << 3 as libc::c_int
           != 0 {
        return false_0
    }
    // allow engine commands if user don't mind
    if cl_filterstuffcmd.value <= 0.0f32 { return true_0 }
    if (*cmd).flags as libc::c_uint & (1 as libc::c_uint) << 4 as libc::c_int
           != 0 {
        return false_0
    }
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[*const libc::c_char; 5]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<*const libc::c_char>()
                                                   as libc::c_ulong) {
        if Q_strnicmp((*cmd).name, prefixes[i as usize],
                      Q_strlen(prefixes[i as usize]) as libc::c_int) == 0 {
            return false_0
        }
        i += 1
    }
    return true_0;
}
/*
============
Cmd_ExecuteString

A complete command line has been parsed, so try to execute it
============
*/
unsafe extern "C" fn Cmd_ExecuteStringWithPrivilegeCheck(mut text:
                                                             *const libc::c_char,
                                                         mut isPrivileged:
                                                             qboolean) {
    let mut cmd: *mut cmd_t = 0 as *mut cmd_t;
    let mut a: *mut cmdalias_t = 0 as *mut cmdalias_t;
    let mut cvar: *mut convar_t = 0 as *mut convar_t;
    let mut command: [libc::c_char; 2048] = [0; 2048];
    let mut pcmd: *mut libc::c_char = command.as_mut_ptr();
    let mut len: libc::c_int = 0 as libc::c_int;
    cmd_condlevel = 0 as libc::c_int;
    // cvar value substitution
    if if !cmd_scripting.is_null() && (*cmd_scripting).value != 0.0f32 {
           true_0 as libc::c_int
       } else { false_0 as libc::c_int } != 0 {
        while *text != 0 {
            // check for escape
            if (*text as libc::c_int == '\\' as i32 ||
                    *text as libc::c_int == '$' as i32) &&
                   *text.offset(1 as libc::c_int as isize) as libc::c_int ==
                       '$' as i32 {
                text = text.offset(1)
            } else if *text as libc::c_int == '$' as i32 {
                let mut token: [libc::c_char; 2048] = [0; 2048];
                let mut ptoken: *mut libc::c_char = token.as_mut_ptr();
                // check for correct cvar name
                text = text.offset(1);
                while *text as libc::c_int >= '0' as i32 &&
                          *text as libc::c_int <= '9' as i32 ||
                          *text as libc::c_int >= 'A' as i32 &&
                              *text as libc::c_int <= 'Z' as i32 ||
                          *text as libc::c_int >= 'a' as i32 &&
                              *text as libc::c_int <= 'z' as i32 ||
                          *text as libc::c_int == '_' as i32 {
                    let fresh9 = text;
                    text = text.offset(1);
                    let fresh10 = ptoken;
                    ptoken = ptoken.offset(1);
                    *fresh10 = *fresh9
                }
                *ptoken = 0 as libc::c_int as libc::c_char;
                len =
                    (len as
                         libc::c_ulong).wrapping_add(Q_strncpy(pcmd,
                                                               Cvar_VariableString(token.as_mut_ptr()),
                                                               (2048 as
                                                                    libc::c_int
                                                                    - len) as
                                                                   size_t)) as
                        libc::c_int as libc::c_int;
                pcmd = command.as_mut_ptr().offset(len as isize);
                if *text == 0 { break ; }
            }
            let fresh11 = text;
            text = text.offset(1);
            let fresh12 = pcmd;
            pcmd = pcmd.offset(1);
            *fresh12 = *fresh11;
            len += 1
        }
        *pcmd = 0 as libc::c_int as libc::c_char;
        text = command.as_mut_ptr();
        while *text as libc::c_int == ':' as i32 {
            if cmd_condition & (1 as libc::c_uint) << cmd_condlevel == 0 {
                return
            }
            cmd_condlevel += 1;
            text = text.offset(1)
        }
    }
    // execute the command line
    Cmd_TokenizeString(text); // no tokens
    if Cmd_Argc() == 0 { return }
    BaseCmd_FindAll(cmd_argv[0 as libc::c_int as usize],
                    &mut cmd as *mut *mut cmd_t as *mut *mut libc::c_void,
                    &mut a as *mut *mut cmdalias_t as *mut *mut libc::c_void,
                    &mut cvar as *mut *mut convar_t as
                        *mut *mut libc::c_void);
    if host.apply_game_config as u64 == 0 {
        // check aliases
        if a.is_null() {
            // if not found in basecmd
            a = cmd_alias;
            while !a.is_null() {
                if Q_strnicmp(cmd_argv[0 as libc::c_int as usize],
                              (*a).name.as_mut_ptr(), 99999 as libc::c_int) ==
                       0 {
                    break ;
                }
                a = (*a).next
            }
        }
        if !a.is_null() {
            Cbuf_InsertTextToBuffer(if isPrivileged as libc::c_uint != 0 {
                                        &mut cmd_text
                                    } else { &mut filteredcmd_text },
                                    (*a).value);
            return
        }
    }
    // special mode for restore game.dll archived cvars
    if host.apply_game_config as u64 == 0 ||
           Q_strncmp(cmd_argv[0 as libc::c_int as usize],
                     b"exec\x00" as *const u8 as *const libc::c_char,
                     99999 as libc::c_int) == 0 {
        if cmd.is_null() || (*cmd).function.is_none() {
            // if not found in basecmd
            cmd = cmd_functions;
            while !cmd.is_null() {
                if Q_strnicmp(cmd_argv[0 as libc::c_int as usize],
                              (*cmd).name, 99999 as libc::c_int) == 0 &&
                       (*cmd).function.is_some() {
                    break ;
                }
                cmd = (*cmd).next
            }
        }
        // check functions
        if !cmd.is_null() && (*cmd).function.is_some() {
            if Cmd_ShouldAllowCommand(cmd, isPrivileged) as u64 != 0 {
                cmd_currentCommandIsPrivileged = isPrivileged;
                (*cmd).function.expect("non-null function pointer")();
                cmd_currentCommandIsPrivileged = true_0
            } else {
                Con_Printf(b"^3Warning:^7 Could not execute privileged command %s\n\x00"
                               as *const u8 as *const libc::c_char,
                           (*cmd).name);
            }
            return
        }
    }
    // check cvars
    if Cvar_CommandWithPrivilegeCheck(cvar, isPrivileged) as u64 != 0 {
        return
    } // don't send nothing to server: we is a server!
    if host.apply_game_config as u64 != 0 { return }
    // forward the command line to the server, so the entity DLL can parse it
    if host.type_0 == HOST_NORMAL as libc::c_int as libc::c_uint {
        if cls.state as libc::c_uint >=
               ca_connected as libc::c_int as libc::c_uint {
            Cmd_ForwardToServer();
        } else if Cvar_VariableInteger(b"host_gameloaded\x00" as *const u8 as
                                           *const libc::c_char) != 0 {
            Con_Printf(b"^3Warning:^7 Unknown command \"%s\"\n\x00" as
                           *const u8 as *const libc::c_char, text);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_ExecuteString(mut text: *const libc::c_char) {
    Cmd_ExecuteStringWithPrivilegeCheck(text, true_0);
}
// XASH_DEDICATED
/*
===================
Cmd_ForwardToServer

adds the current command line as a clc_stringcmd to the client message.
things like godmode, noclip, etc, are commands directed to the server,
so when they are typed in at the console, they will need to be forwarded.
===================
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_ForwardToServer() {
    let mut str: [libc::c_char; 32768] = [0; 32768];
    if cls.demoplayback != 0 {
        if Q_strnicmp(Cmd_Argv(0 as libc::c_int),
                      b"pause\x00" as *const u8 as *const libc::c_char,
                      99999 as libc::c_int) == 0 {
            cl.paused =
                ::std::mem::transmute::<libc::c_uint,
                                        qboolean>(cl.paused as libc::c_uint ^
                                                      1 as libc::c_int as
                                                          libc::c_uint)
        }
        return
    }
    if (cls.state as libc::c_uint) <
           ca_connected as libc::c_int as libc::c_uint ||
           cls.state as libc::c_uint >
               ca_active as libc::c_int as libc::c_uint {
        if Q_strnicmp(Cmd_Argv(0 as libc::c_int),
                      b"setinfo\x00" as *const u8 as *const libc::c_char,
                      99999 as libc::c_int) != 0 {
            Con_Printf(b"Can\'t \"%s\", not connected\n\x00" as *const u8 as
                           *const libc::c_char, Cmd_Argv(0 as libc::c_int));
        }
        return
        // not connected
    }
    MSG_WriteCmdExt(&mut cls.netchan.message, 3 as libc::c_int, NS_CLIENT,
                    0 as *const libc::c_char);
    str[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    if Q_strnicmp(Cmd_Argv(0 as libc::c_int),
                  b"cmd\x00" as *const u8 as *const libc::c_char,
                  99999 as libc::c_int) != 0 {
        Q_strncat(str.as_mut_ptr(), Cmd_Argv(0 as libc::c_int),
                  99999 as libc::c_int as size_t);
        Q_strncat(str.as_mut_ptr(),
                  b" \x00" as *const u8 as *const libc::c_char,
                  99999 as libc::c_int as size_t);
    }
    if Cmd_Argc() > 1 as libc::c_int {
        Q_strncat(str.as_mut_ptr(), Cmd_Args(),
                  99999 as libc::c_int as size_t);
    } else {
        Q_strncat(str.as_mut_ptr(),
                  b"\n\x00" as *const u8 as *const libc::c_char,
                  99999 as libc::c_int as size_t);
    }
    MSG_WriteString(&mut cls.netchan.message, str.as_mut_ptr());
}
// XASH_DEDICATED
/*
============
Cmd_List_f
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_List_f() {
    let mut cmd: *mut cmd_t = 0 as *mut cmd_t; // never show system cmds
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut match_0: *const libc::c_char = 0 as *const libc::c_char;
    if Cmd_Argc() > 1 as libc::c_int {
        match_0 = Cmd_Argv(1 as libc::c_int)
    } else { match_0 = 0 as *const libc::c_char }
    cmd = cmd_functions;
    while !cmd.is_null() {
        if !(*(*cmd).name.offset(0 as libc::c_int as isize) as libc::c_int ==
                 '@' as i32) {
            if !(!match_0.is_null() &&
                     Q_stricmpext(match_0, (*cmd).name) as u64 == 0) {
                Con_Printf(b" %-*s ^3%s^7\n\x00" as *const u8 as
                               *const libc::c_char, 32 as libc::c_int,
                           (*cmd).name, (*cmd).desc);
                i += 1
            }
        }
        cmd = (*cmd).next
    }
    Con_Printf(b"%i commands\n\x00" as *const u8 as *const libc::c_char, i);
}
/*
============
Cmd_Unlink

unlink all commands with specified flag
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_Unlink(mut group: libc::c_int) {
    let mut cmd: *mut cmd_t = 0 as *mut cmd_t;
    let mut prev: *mut *mut cmd_t = 0 as *mut *mut cmd_t;
    let mut count: libc::c_int = 0 as libc::c_int;
    if Cvar_VariableInteger(b"host_gameloaded\x00" as *const u8 as
                                *const libc::c_char) != 0 &&
           group as libc::c_uint & (1 as libc::c_uint) << 0 as libc::c_int !=
               0 {
        return
    }
    if Cvar_VariableInteger(b"host_clientloaded\x00" as *const u8 as
                                *const libc::c_char) != 0 &&
           group as libc::c_uint & (1 as libc::c_uint) << 1 as libc::c_int !=
               0 {
        return
    }
    if Cvar_VariableInteger(b"host_gameuiloaded\x00" as *const u8 as
                                *const libc::c_char) != 0 &&
           group as libc::c_uint & (1 as libc::c_uint) << 2 as libc::c_int !=
               0 {
        return
    }
    prev = &mut cmd_functions;
    loop  {
        cmd = *prev;
        if cmd.is_null() { break ; }
        // do filter by specified group
        if group != 0 && (*cmd).flags & group == 0 {
            prev = &mut (*cmd).next
        } else {
            BaseCmd_Remove(HM_CMD, (*cmd).name);
            *prev = (*cmd).next;
            if !(*cmd).name.is_null() {
                _Mem_Free((*cmd).name as *mut libc::c_void,
                          b"../engine/common/cmd.c\x00" as *const u8 as
                              *const libc::c_char, 1238 as libc::c_int);
            }
            if !(*cmd).desc.is_null() {
                _Mem_Free((*cmd).desc as *mut libc::c_void,
                          b"../engine/common/cmd.c\x00" as *const u8 as
                              *const libc::c_char, 1239 as libc::c_int);
            }
            _Mem_Free(cmd as *mut libc::c_void,
                      b"../engine/common/cmd.c\x00" as *const u8 as
                          *const libc::c_char, 1241 as libc::c_int);
            count += 1
        }
    }
    Con_Reportf(b"unlink %i commands\n\x00" as *const u8 as
                    *const libc::c_char, count);
}
unsafe extern "C" fn Cmd_Apropos_f() {
    let mut cmd: *mut cmd_t = 0 as *mut cmd_t;
    let mut var: *mut convar_t = 0 as *mut convar_t;
    let mut alias: *mut cmdalias_t = 0 as *mut cmdalias_t;
    let mut partial: *const libc::c_char = 0 as *const libc::c_char;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut ispattern: qboolean = false_0;
    if Cmd_Argc() > 1 as libc::c_int {
        partial = Cmd_Args()
    } else {
        Con_Printf(b"apropos what?\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    ispattern =
        (!partial.is_null() &&
             !Q_strpbrk(partial,
                        b"*?\x00" as *const u8 as
                            *const libc::c_char).is_null()) as libc::c_int as
            qboolean;
    if ispattern as u64 == 0 {
        partial = va(b"*%s*\x00" as *const u8 as *const libc::c_char, partial)
    }
    let mut current_block_16: u64;
    var = Cvar_GetList() as *mut convar_t;
    while !var.is_null() {
        if matchpattern_with_separator((*var).name, partial, true_0,
                                       b"\x00" as *const u8 as
                                           *const libc::c_char, false_0) == 0
           {
            let mut desc: *const libc::c_char = 0 as *const libc::c_char;
            if (*var).flags & (1 as libc::c_int) << 18 as libc::c_int != 0 {
                desc = (*var).desc
            } else {
                desc = b"game cvar\x00" as *const u8 as *const libc::c_char
            }
            if desc.is_null() {
                desc = b"user cvar\x00" as *const u8 as *const libc::c_char
            }
            if matchpattern_with_separator(desc, partial, true_0,
                                           b"\x00" as *const u8 as
                                               *const libc::c_char, false_0)
                   == 0 {
                current_block_16 = 7651349459974463963;
            } else { current_block_16 = 12147880666119273379; }
        } else { current_block_16 = 12147880666119273379; }
        match current_block_16 {
            12147880666119273379 => {
                // TODO: maybe add flags output like cvarlist, also
		// fix inconsistencies in output from different commands
                Con_Printf(b"cvar ^3%s^7 is \"%s\" [\"%s\"] %s\n\x00" as
                               *const u8 as *const libc::c_char, (*var).name,
                           (*var).string,
                           if (*var).flags &
                                  (1 as libc::c_int) << 18 as libc::c_int != 0
                              {
                               (*var).def_string as *const libc::c_char
                           } else {
                               b"\x00" as *const u8 as *const libc::c_char
                           },
                           if (*var).flags &
                                  (1 as libc::c_int) << 18 as libc::c_int != 0
                              {
                               (*var).desc as *const libc::c_char
                           } else {
                               b"game cvar\x00" as *const u8 as
                                   *const libc::c_char
                           }); // never show system cmds
                count += 1
            }
            _ => { }
        }
        var = (*var).next
    }
    cmd = Cmd_GetFirstFunctionHandle();
    while !cmd.is_null() {
        if !(*(*cmd).name.offset(0 as libc::c_int as isize) as libc::c_int ==
                 '@' as i32) {
            if !(matchpattern_with_separator((*cmd).name, partial, true_0,
                                             b"\x00" as *const u8 as
                                                 *const libc::c_char, false_0)
                     == 0 &&
                     matchpattern_with_separator((*cmd).desc, partial, true_0,
                                                 b"\x00" as *const u8 as
                                                     *const libc::c_char,
                                                 false_0) == 0) {
                Con_Printf(b"command ^2%s^7: %s\n\x00" as *const u8 as
                               *const libc::c_char, (*cmd).name, (*cmd).desc);
                count += 1
            }
        }
        cmd = Cmd_GetNextFunctionHandle(cmd)
    }
    alias = Cmd_AliasGetList();
    while !alias.is_null() {
        // proceed a bit differently here as an alias value always got a final \n
        if !(matchpattern_with_separator((*alias).name.as_mut_ptr(), partial,
                                         true_0,
                                         b"\x00" as *const u8 as
                                             *const libc::c_char, false_0) ==
                 0 &&
                 matchpattern_with_separator((*alias).value, partial, true_0,
                                             b"\n\x00" as *const u8 as
                                                 *const libc::c_char, false_0)
                     == 0) {
            Con_Printf(b"alias ^5%s^7: %s\x00" as *const u8 as
                           *const libc::c_char, (*alias).name.as_mut_ptr(),
                       (*alias).value); // do not print an extra \n
            count += 1
        }
        // when \n is a separator, wildcards don't match it //-V666
        alias = (*alias).next
    }
    Con_Printf(b"\n%i result%s\n\n\x00" as *const u8 as *const libc::c_char,
               count,
               if count > 1 as libc::c_int {
                   b"s\x00" as *const u8 as *const libc::c_char
               } else { b"\x00" as *const u8 as *const libc::c_char });
}
/*
============
Cmd_Null_f

null function for some cmd stubs
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_Null_f() { }
/*
==========
Cmd_Escape

inserts escape sequences
==========
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_Escape(mut newCommand: *mut libc::c_char,
                                    mut oldCommand: *const libc::c_char,
                                    mut len: libc::c_int) {
    let mut c: libc::c_int = 0;
    let mut scripting: libc::c_int =
        if !cmd_scripting.is_null() && (*cmd_scripting).value != 0.0f32 {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int };
    loop  {
        let fresh13 = oldCommand;
        oldCommand = oldCommand.offset(1);
        c = *fresh13 as libc::c_int;
        if !(c != 0 && len > 1 as libc::c_int) { break ; }
        if c == '\"' as i32 {
            let fresh14 = newCommand;
            newCommand = newCommand.offset(1);
            *fresh14 = '\\' as i32 as libc::c_char;
            len -= 1
        }
        if scripting != 0 && c == '$' as i32 {
            let fresh15 = newCommand;
            newCommand = newCommand.offset(1);
            *fresh15 = '$' as i32 as libc::c_char;
            len -= 1
        }
        let fresh16 = newCommand;
        newCommand = newCommand.offset(1);
        *fresh16 = c as libc::c_char;
        len -= 1
    }
    let fresh17 = newCommand;
    newCommand = newCommand.offset(1);
    *fresh17 = 0 as libc::c_int as libc::c_char;
}
/*
============
Cmd_Init

============
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_Init() {
    Cbuf_Init();
    cmd_functions = 0 as *mut cmd_t;
    cmd_condition = 0 as libc::c_int as uint;
    cmd_alias = 0 as *mut cmdalias_t;
    cmd_args = 0 as *const libc::c_char;
    cmd_argc = 0 as libc::c_int;
    // register our commands
    Cmd_AddCommand(b"echo\x00" as *const u8 as *const libc::c_char,
                   Some(Cmd_Echo_f as unsafe extern "C" fn() -> ()),
                   b"print a message to the console (useful in scripts)\x00"
                       as *const u8 as *const libc::c_char);
    Cmd_AddCommand(b"wait\x00" as *const u8 as *const libc::c_char,
                   Some(Cmd_Wait_f as unsafe extern "C" fn() -> ()),
                   b"make script execution wait for some rendered frames\x00"
                       as *const u8 as *const libc::c_char);
    Cmd_AddCommand(b"cmdlist\x00" as *const u8 as *const libc::c_char,
                   Some(Cmd_List_f as unsafe extern "C" fn() -> ()),
                   b"display all console commands beginning with the specified prefix\x00"
                       as *const u8 as *const libc::c_char);
    Cmd_AddCommand(b"stuffcmds\x00" as *const u8 as *const libc::c_char,
                   Some(Cmd_StuffCmds_f as unsafe extern "C" fn() -> ()),
                   b"execute commandline parameters (must be present in .rc script)\x00"
                       as *const u8 as *const libc::c_char);
    Cmd_AddCommand(b"apropos\x00" as *const u8 as *const libc::c_char,
                   Some(Cmd_Apropos_f as unsafe extern "C" fn() -> ()),
                   b"lists all console variables/commands/aliases containing the specified string in the name or description\x00"
                       as *const u8 as *const libc::c_char);
    Cmd_AddCommand(b"cmd\x00" as *const u8 as *const libc::c_char,
                   Some(Cmd_ForwardToServer as unsafe extern "C" fn() -> ()),
                   b"send a console commandline to the server\x00" as
                       *const u8 as *const libc::c_char);
    // XASH_DEDICATED
    Cmd_AddCommand(b"alias\x00" as *const u8 as *const libc::c_char,
                   Some(Cmd_Alias_f as unsafe extern "C" fn() -> ()),
                   b"create a script function. Without arguments show the list of all alias\x00"
                       as *const u8 as *const libc::c_char);
    Cmd_AddCommand(b"unalias\x00" as *const u8 as *const libc::c_char,
                   Some(Cmd_UnAlias_f as unsafe extern "C" fn() -> ()),
                   b"remove a script function\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"if\x00" as *const u8 as *const libc::c_char,
                   Some(Cmd_If_f as unsafe extern "C" fn() -> ()),
                   b"compare and set condition bits\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"else\x00" as *const u8 as *const libc::c_char,
                   Some(Cmd_Else_f as unsafe extern "C" fn() -> ()),
                   b"invert condition bit\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"basecmd_stats\x00" as *const u8 as *const libc::c_char,
                   Some(BaseCmd_Stats_f as unsafe extern "C" fn() -> ()),
                   b"print info about basecmd usage\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"basecmd_test\x00" as *const u8 as *const libc::c_char,
                   Some(BaseCmd_Test_f as unsafe extern "C" fn() -> ()),
                   b"test basecmd\x00" as *const u8 as *const libc::c_char);
}
