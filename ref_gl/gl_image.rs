#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types,
           ptr_wrapping_offset_from, register_tool)]
extern "C" {
    pub type grasshdr_s;
    pub type pmtrace_s;
    pub type con_nprint_s;
    pub type engine_studio_api_s;
    pub type r_studio_interface_s;
    pub type physent_s;
    pub type client_textmessage_s;
    pub type screenfade_s;
    pub type world_static_s;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn sqrt(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn VectorNormalizeLength2(v: *const vec_t, out: *mut vec_t)
     -> libc::c_float;
    #[no_mangle]
    static mut pglGetError: Option<unsafe extern "C" fn() -> GLenum>;
    #[no_mangle]
    static mut pglBindTexture:
           Option<unsafe extern "C" fn(_: GLenum, _: GLuint) -> ()>;
    #[no_mangle]
    static mut pglDeleteTextures:
           Option<unsafe extern "C" fn(_: GLsizei, _: *const GLuint) -> ()>;
    #[no_mangle]
    static mut pglDisable: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglEnable: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglTexImage1D:
           Option<unsafe extern "C" fn(_: GLenum, _: GLint, _: GLint,
                                       _: GLsizei, _: GLint, _: GLenum,
                                       _: GLenum, _: *const libc::c_void)
                      -> ()>;
    #[no_mangle]
    static mut pglTexImage2D:
           Option<unsafe extern "C" fn(_: GLenum, _: GLint, _: GLint,
                                       _: GLsizei, _: GLsizei, _: GLint,
                                       _: GLenum, _: GLenum,
                                       _: *const libc::c_void) -> ()>;
    #[no_mangle]
    static mut pglTexImage2DMultisample:
           Option<unsafe extern "C" fn(_: GLenum, _: GLsizei, _: GLenum,
                                       _: GLsizei, _: GLsizei, _: GLboolean)
                      -> ()>;
    #[no_mangle]
    static mut pglTexParameterf:
           Option<unsafe extern "C" fn(_: GLenum, _: GLenum, _: GLfloat)
                      -> ()>;
    #[no_mangle]
    static mut pglTexParameterfv:
           Option<unsafe extern "C" fn(_: GLenum, _: GLenum,
                                       _: *const GLfloat) -> ()>;
    #[no_mangle]
    static mut pglTexParameteri:
           Option<unsafe extern "C" fn(_: GLenum, _: GLenum, _: GLint) -> ()>;
    #[no_mangle]
    static mut pglTexSubImage1D:
           Option<unsafe extern "C" fn(_: GLenum, _: GLint, _: GLint,
                                       _: GLsizei, _: GLenum, _: GLenum,
                                       _: *const libc::c_void) -> ()>;
    #[no_mangle]
    static mut pglTexSubImage2D:
           Option<unsafe extern "C" fn(_: GLenum, _: GLint, _: GLint,
                                       _: GLint, _: GLsizei, _: GLsizei,
                                       _: GLenum, _: GLenum,
                                       _: *const libc::c_void) -> ()>;
    #[no_mangle]
    static mut pglCompressedTexImage3DARB:
           Option<unsafe extern "C" fn(_: GLenum, _: GLint, _: GLenum,
                                       _: GLsizei, _: GLsizei, _: GLsizei,
                                       _: GLint, _: GLsizei,
                                       _: *const libc::c_void) -> ()>;
    #[no_mangle]
    static mut pglCompressedTexImage2DARB:
           Option<unsafe extern "C" fn(_: GLenum, _: GLint, _: GLenum,
                                       _: GLsizei, _: GLsizei, _: GLint,
                                       _: GLsizei, _: *const libc::c_void)
                      -> ()>;
    #[no_mangle]
    static mut pglCompressedTexImage1DARB:
           Option<unsafe extern "C" fn(_: GLenum, _: GLint, _: GLenum,
                                       _: GLsizei, _: GLint, _: GLsizei,
                                       _: *const libc::c_void) -> ()>;
    #[no_mangle]
    static mut pglCompressedTexSubImage3DARB:
           Option<unsafe extern "C" fn(_: GLenum, _: GLint, _: GLint,
                                       _: GLint, _: GLint, _: GLsizei,
                                       _: GLsizei, _: GLsizei, _: GLenum,
                                       _: GLsizei, _: *const libc::c_void)
                      -> ()>;
    #[no_mangle]
    static mut pglCompressedTexSubImage2DARB:
           Option<unsafe extern "C" fn(_: GLenum, _: GLint, _: GLint,
                                       _: GLint, _: GLsizei, _: GLsizei,
                                       _: GLenum, _: GLsizei,
                                       _: *const libc::c_void) -> ()>;
    #[no_mangle]
    static mut pglCompressedTexSubImage1DARB:
           Option<unsafe extern "C" fn(_: GLenum, _: GLint, _: GLint,
                                       _: GLsizei, _: GLenum, _: GLsizei,
                                       _: *const libc::c_void) -> ()>;
    #[no_mangle]
    static mut pglTexImage3D:
           Option<unsafe extern "C" fn(_: GLenum, _: GLint, _: GLenum,
                                       _: GLsizei, _: GLsizei, _: GLsizei,
                                       _: GLint, _: GLenum, _: GLenum,
                                       _: *const libc::c_void) -> ()>;
    #[no_mangle]
    static mut pglTexSubImage3D:
           Option<unsafe extern "C" fn(_: GLenum, _: GLint, _: GLint,
                                       _: GLint, _: GLint, _: GLsizei,
                                       _: GLsizei, _: GLsizei, _: GLenum,
                                       _: GLenum, _: *const libc::c_void)
                      -> ()>;
    #[no_mangle]
    static mut r_temppool: poolhandle_t;
    #[no_mangle]
    static mut tr: gl_globals_t;
    #[no_mangle]
    static mut glState: glstate_t;
    #[no_mangle]
    fn GL_SelectTexture(texture: GLint);
    #[no_mangle]
    static mut gEngfuncs: ref_api_t;
    #[no_mangle]
    fn GL_CleanupAllTextureUnits();
    #[no_mangle]
    static mut gl_texture_nearest: *mut cvar_t;
    #[no_mangle]
    static mut gl_lightmap_nearest: *mut cvar_t;
    #[no_mangle]
    static mut gl_texture_lodbias: *mut cvar_t;
    #[no_mangle]
    fn GL_Support(r_ext: libc::c_int) -> qboolean;
    #[no_mangle]
    static mut gl_texture_anisotropy: *mut cvar_t;
    #[no_mangle]
    static mut glConfig: glconfig_t;
    #[no_mangle]
    static mut glw_state: glwstate_t;
    #[no_mangle]
    fn GL_ErrorString(err: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    static mut gl_check_errors: *mut cvar_t;
    #[no_mangle]
    static mut gpGlobals: *mut ref_globals_t;
    #[no_mangle]
    static mut gl_round_down: *mut cvar_t;
    #[no_mangle]
    static mut gl_emboss_scale: *mut cvar_t;
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Q_strnicmp(s1: *const libc::c_char, s2: *const libc::c_char,
                  n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Q_strlen(string: *const libc::c_char) -> size_t;
    #[no_mangle]
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn Q_strncat(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn COM_FileBase(in_0: *const libc::c_char, out: *mut libc::c_char);
    #[no_mangle]
    fn Q_pretifymem(value: libc::c_float, digitsafterdecimal: libc::c_int)
     -> *mut libc::c_char;
    #[no_mangle]
    fn COM_HashKey(string: *const libc::c_char, hashSize: uint) -> uint;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type off_t = __off_t;
pub type uint = libc::c_uint;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type byte = libc::c_uchar;
pub type vec_t = libc::c_float;
pub type vec3_t = [vec_t; 3];
pub type vec4_t = [vec_t; 4];
pub type rgba_t = [byte; 4];
pub type poolhandle_t = uint32_t;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type fs_offset_t = off_t;
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
pub type cl_entity_t = cl_entity_s;
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
pub type dlight_t = dlight_s;
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
pub type gl_context_type_t = libc::c_uint;
pub const CONTEXT_TYPE_GL_CORE: gl_context_type_t = 3;
pub const CONTEXT_TYPE_GLES_2_X: gl_context_type_t = 2;
pub const CONTEXT_TYPE_GLES_1_X: gl_context_type_t = 1;
pub const CONTEXT_TYPE_GL: gl_context_type_t = 0;
pub type gles_wrapper_t = libc::c_uint;
pub const GLES_WRAPPER_GL4ES: gles_wrapper_t = 3;
pub const GLES_WRAPPER_WES: gles_wrapper_t = 2;
pub const GLES_WRAPPER_NANOGL: gles_wrapper_t = 1;
pub const GLES_WRAPPER_NONE: gles_wrapper_t = 0;
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
pub type modelstate_t = modelstate_s;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mstudiotex_s {
    pub name: [libc::c_char; 64],
    pub flags: uint32_t,
    pub width: int32_t,
    pub height: int32_t,
    pub index: int32_t,
}
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
pub type render_interface_t = render_interface_s;
pub type pixformat_t = libc::c_uint;
pub const PF_TOTALCOUNT: pixformat_t = 12;
pub const PF_ATI2: pixformat_t = 11;
pub const PF_DXT5: pixformat_t = 10;
pub const PF_DXT3: pixformat_t = 9;
pub const PF_DXT1: pixformat_t = 8;
pub const PF_LUMINANCE: pixformat_t = 7;
pub const PF_BGR_24: pixformat_t = 6;
pub const PF_RGB_24: pixformat_t = 5;
pub const PF_BGRA_32: pixformat_t = 4;
pub const PF_RGBA_32: pixformat_t = 3;
pub const PF_INDEXED_32: pixformat_t = 2;
pub const PF_INDEXED_24: pixformat_t = 1;
pub const PF_UNKNOWN: pixformat_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bpc_desc_s {
    pub format: libc::c_int,
    pub name: [libc::c_char; 16],
    pub glFormat: uint,
    pub bpp: libc::c_int,
}
pub type C2RustUnnamed = libc::c_uint;
pub const IL_OVERVIEW: C2RustUnnamed = 64;
pub const IL_LOAD_DECAL: C2RustUnnamed = 32;
pub const IL_DDS_HARDWARE: C2RustUnnamed = 16;
pub const IL_DONTFLIP_TGA: C2RustUnnamed = 8;
pub const IL_ALLOW_OVERWRITE: C2RustUnnamed = 4;
pub const IL_KEEP_8BIT: C2RustUnnamed = 2;
pub const IL_USE_LERPING: C2RustUnnamed = 1;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const IMAGE_REMAP: C2RustUnnamed_0 = 134217728;
pub const IMAGE_LIGHTGAMMA: C2RustUnnamed_0 = 67108864;
pub const IMAGE_QUANTIZE: C2RustUnnamed_0 = 33554432;
pub const IMAGE_MAKE_LUMA: C2RustUnnamed_0 = 16777216;
pub const IMAGE_FORCE_RGBA: C2RustUnnamed_0 = 8388608;
pub const IMAGE_RESAMPLE: C2RustUnnamed_0 = 1048576;
pub const IMAGE_EMBOSS: C2RustUnnamed_0 = 524288;
pub const IMAGE_ROT270: C2RustUnnamed_0 = 458752;
pub const IMAGE_ROT180: C2RustUnnamed_0 = 196608;
pub const IMAGE_ROT_90: C2RustUnnamed_0 = 262144;
pub const IMAGE_FLIP_Y: C2RustUnnamed_0 = 131072;
pub const IMAGE_FLIP_X: C2RustUnnamed_0 = 65536;
pub const IMAGE_QUAKEPAL: C2RustUnnamed_0 = 1024;
pub const IMAGE_ONEBIT_ALPHA: C2RustUnnamed_0 = 512;
pub const IMAGE_MULTILAYER: C2RustUnnamed_0 = 256;
pub const IMAGE_DDS_FORMAT: C2RustUnnamed_0 = 128;
pub const IMAGE_QUAKESKY: C2RustUnnamed_0 = 64;
pub const IMAGE_SKYBOX: C2RustUnnamed_0 = 32;
pub const IMAGE_HAS_LUMA: C2RustUnnamed_0 = 16;
pub const IMAGE_COLORINDEX: C2RustUnnamed_0 = 8;
pub const IMAGE_HAS_COLOR: C2RustUnnamed_0 = 4;
pub const IMAGE_HAS_ALPHA: C2RustUnnamed_0 = 2;
pub const IMAGE_CUBEMAP: C2RustUnnamed_0 = 1;
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
pub struct studiohdr_s {
    pub ident: int32_t,
    pub version: int32_t,
    pub name: [libc::c_char; 64],
    pub length: int32_t,
    pub eyeposition: vec3_t,
    pub min: vec3_t,
    pub max: vec3_t,
    pub bbmin: vec3_t,
    pub bbmax: vec3_t,
    pub flags: int32_t,
    pub numbones: int32_t,
    pub boneindex: int32_t,
    pub numbonecontrollers: int32_t,
    pub bonecontrollerindex: int32_t,
    pub numhitboxes: int32_t,
    pub hitboxindex: int32_t,
    pub numseq: int32_t,
    pub seqindex: int32_t,
    pub numseqgroups: int32_t,
    pub seqgroupindex: int32_t,
    pub numtextures: int32_t,
    pub textureindex: int32_t,
    pub texturedataindex: int32_t,
    pub numskinref: int32_t,
    pub numskinfamilies: int32_t,
    pub skinindex: int32_t,
    pub numbodyparts: int32_t,
    pub bodypartindex: int32_t,
    pub numattachments: int32_t,
    pub attachmentindex: int32_t,
    pub studiohdr2index: int32_t,
    pub unused: int32_t,
    pub unused2: int32_t,
    pub unused3: int32_t,
    pub numtransitions: int32_t,
    pub transitionindex: int32_t,
}
pub type studiohdr_t = studiohdr_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mstudiobone_s {
    pub name: [libc::c_char; 32],
    pub parent: int32_t,
    pub unused: int32_t,
    pub bonecontroller: [int32_t; 6],
    pub value: [vec_t; 6],
    pub scale: [vec_t; 6],
}
pub type mstudiobone_t = mstudiobone_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mstudioevent_s {
    pub frame: int32_t,
    pub event: int32_t,
    pub unused: int32_t,
    pub options: [libc::c_char; 64],
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mstudioanim_s {
    pub offset: [uint16_t; 6],
}
pub type mstudioanim_t = mstudioanim_s;
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
pub struct sortedface_t {
    pub surf: *mut msurface_t,
    pub cull: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ref_globals_s {
    pub developer: qboolean,
    pub time: libc::c_float,
    pub oldtime: libc::c_float,
    pub realtime: libc::c_double,
    pub frametime: libc::c_double,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub fullScreen: qboolean,
    pub wideScreen: qboolean,
    pub vieworg: vec3_t,
    pub viewangles: vec3_t,
    pub vforward: vec3_t,
    pub vright: vec3_t,
    pub vup: vec3_t,
    pub draw_surfaces: *mut sortedface_t,
    pub max_surfaces: libc::c_int,
    pub visbytes: size_t,
    pub desktopBitsPixel: libc::c_int,
}
pub type ref_globals_t = ref_globals_s;
pub type C2RustUnnamed_1 = libc::c_int;
pub const MAX_TEXTURE_UNITS: C2RustUnnamed_1 = 32;
pub const XASH_TEXTURE4: C2RustUnnamed_1 = 4;
pub const XASH_TEXTURE3: C2RustUnnamed_1 = 3;
pub const XASH_TEXTURE2: C2RustUnnamed_1 = 2;
pub const XASH_TEXTURE1: C2RustUnnamed_1 = 1;
pub const XASH_TEXTURE0: C2RustUnnamed_1 = 0;
pub const GL_KEEP_UNIT: C2RustUnnamed_1 = -1;
pub type ref_defaultsprite_e = libc::c_uint;
pub const REF_CHROME_SPRITE: ref_defaultsprite_e = 1;
pub const REF_DOT_SPRITE: ref_defaultsprite_e = 0;
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
pub type C2RustUnnamed_2 = libc::c_int;
pub const PARM_NUMMODELS: C2RustUnnamed_2 = -13;
pub const PARM_NUMENTITIES: C2RustUnnamed_2 = -12;
pub const PARM_LOCAL_GAME: C2RustUnnamed_2 = -11;
pub const PARM_LOCAL_HEALTH: C2RustUnnamed_2 = -10;
pub const PARM_MAX_CLIENTS: C2RustUnnamed_2 = -9;
pub const PARM_WATER_LEVEL: C2RustUnnamed_2 = -8;
pub const PARM_PLAYING_DEMO: C2RustUnnamed_2 = -7;
pub const PARM_CONNSTATE: C2RustUnnamed_2 = -6;
pub const PARM_VIEWENT_INDEX: C2RustUnnamed_2 = -5;
pub const PARM_PLAYER_INDEX: C2RustUnnamed_2 = -4;
pub const PARM_QUAKE_COMPATIBLE: C2RustUnnamed_2 = -3;
pub const PARM_THIRDPERSON: C2RustUnnamed_2 = -2;
pub const PARM_DEV_OVERVIEW: C2RustUnnamed_2 = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ref_api_s {
    pub EngineGetParm: Option<unsafe extern "C" fn(_: libc::c_int,
                                                   _: libc::c_int)
                                  -> libc::c_int>,
    pub Cvar_Get: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                              _: *const libc::c_char,
                                              _: libc::c_int,
                                              _: *const libc::c_char)
                             -> *mut cvar_t>,
    pub pfnGetCvarPointer: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                       _: libc::c_int)
                                      -> *mut cvar_t>,
    pub pfnGetCvarFloat: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                    -> libc::c_float>,
    pub pfnGetCvarString: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                     -> *const libc::c_char>,
    pub Cvar_SetValue: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                   _: libc::c_float) -> ()>,
    pub Cvar_Set: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                              _: *const libc::c_char) -> ()>,
    pub Cvar_RegisterVariable: Option<unsafe extern "C" fn(_: *mut cvar_t)
                                          -> ()>,
    pub Cvar_FullSet: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                  _: *const libc::c_char,
                                                  _: libc::c_int) -> ()>,
    pub Cmd_AddCommand: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                    _:
                                                        Option<unsafe extern "C" fn()
                                                                   -> ()>,
                                                    _: *const libc::c_char)
                                   -> libc::c_int>,
    pub Cmd_RemoveCommand: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                      -> ()>,
    pub Cmd_Argc: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub Cmd_Argv: Option<unsafe extern "C" fn(_: libc::c_int)
                             -> *const libc::c_char>,
    pub Cmd_Args: Option<unsafe extern "C" fn() -> *const libc::c_char>,
    pub Cbuf_AddText: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                 -> ()>,
    pub Cbuf_InsertText: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                    -> ()>,
    pub Cbuf_Execute: Option<unsafe extern "C" fn() -> ()>,
    pub Con_Printf: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                _: ...) -> ()>,
    pub Con_DPrintf: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                 _: ...) -> ()>,
    pub Con_Reportf: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                 _: ...) -> ()>,
    pub Con_NPrintf: Option<unsafe extern "C" fn(_: libc::c_int,
                                                 _: *const libc::c_char,
                                                 _: ...) -> ()>,
    pub Con_NXPrintf: Option<unsafe extern "C" fn(_: *mut con_nprint_s,
                                                  _: *const libc::c_char,
                                                  _: ...) -> ()>,
    pub CL_CenterPrint: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                    _: libc::c_float) -> ()>,
    pub Con_DrawStringLen: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                       _: *mut libc::c_int,
                                                       _: *mut libc::c_int)
                                      -> ()>,
    pub Con_DrawString: Option<unsafe extern "C" fn(_: libc::c_int,
                                                    _: libc::c_int,
                                                    _: *const libc::c_char,
                                                    _: *mut byte)
                                   -> libc::c_int>,
    pub CL_DrawCenterPrint: Option<unsafe extern "C" fn() -> ()>,
    pub GetLocalPlayer: Option<unsafe extern "C" fn() -> *mut cl_entity_s>,
    pub GetViewModel: Option<unsafe extern "C" fn() -> *mut cl_entity_s>,
    pub GetEntityByIndex: Option<unsafe extern "C" fn(_: libc::c_int)
                                     -> *mut cl_entity_s>,
    pub R_BeamGetEntity: Option<unsafe extern "C" fn(_: libc::c_int)
                                    -> *mut cl_entity_s>,
    pub CL_GetWaterEntity: Option<unsafe extern "C" fn(_: *const vec_t)
                                      -> *mut cl_entity_s>,
    pub CL_AddVisibleEntity: Option<unsafe extern "C" fn(_: *mut cl_entity_t,
                                                         _: libc::c_int)
                                        -> qboolean>,
    pub Mod_SampleSizeForFace: Option<unsafe extern "C" fn(_: *mut msurface_s)
                                          -> libc::c_int>,
    pub Mod_BoxVisible: Option<unsafe extern "C" fn(_: *const vec_t,
                                                    _: *const vec_t,
                                                    _: *const byte)
                                   -> qboolean>,
    pub GetWorld: Option<unsafe extern "C" fn() -> *mut world_static_s>,
    pub Mod_PointInLeaf: Option<unsafe extern "C" fn(_: *const vec_t,
                                                     _: *mut mnode_t)
                                    -> *mut mleaf_t>,
    pub Mod_CreatePolygonsForHull: Option<unsafe extern "C" fn(_: libc::c_int)
                                              -> ()>,
    pub R_StudioSlerpBones: Option<unsafe extern "C" fn(_: libc::c_int,
                                                        _: *mut vec4_t,
                                                        _:
                                                            *mut [libc::c_float; 3],
                                                        _: *mut vec4_t,
                                                        _:
                                                            *mut [libc::c_float; 3],
                                                        _: libc::c_float)
                                       -> ()>,
    pub R_StudioCalcBoneQuaternion: Option<unsafe extern "C" fn(_:
                                                                    libc::c_int,
                                                                _:
                                                                    libc::c_float,
                                                                _:
                                                                    *mut mstudiobone_t,
                                                                _:
                                                                    *mut mstudioanim_t,
                                                                _:
                                                                    *mut libc::c_float,
                                                                _: *mut vec_t)
                                               -> ()>,
    pub R_StudioCalcBonePosition: Option<unsafe extern "C" fn(_: libc::c_int,
                                                              _:
                                                                  libc::c_float,
                                                              _:
                                                                  *mut mstudiobone_t,
                                                              _:
                                                                  *mut mstudioanim_t,
                                                              _: *mut vec_t,
                                                              _: *mut vec_t)
                                             -> ()>,
    pub R_StudioGetAnim: Option<unsafe extern "C" fn(_: *mut studiohdr_t,
                                                     _: *mut model_t,
                                                     _: *mut mstudioseqdesc_t)
                                    -> *mut libc::c_void>,
    pub pfnStudioEvent: Option<unsafe extern "C" fn(_: *const mstudioevent_s,
                                                    _: *const cl_entity_t)
                                   -> ()>,
    pub CL_DrawEFX: Option<unsafe extern "C" fn(_: libc::c_float, _: qboolean)
                               -> ()>,
    pub CL_ThinkParticle: Option<unsafe extern "C" fn(_: libc::c_double,
                                                      _: *mut particle_t)
                                     -> ()>,
    pub R_FreeDeadParticles: Option<unsafe extern "C" fn(_:
                                                             *mut *mut particle_t)
                                        -> ()>,
    pub CL_AllocParticleFast: Option<unsafe extern "C" fn()
                                         -> *mut particle_t>,
    pub CL_AllocElight: Option<unsafe extern "C" fn(_: libc::c_int)
                                   -> *mut dlight_s>,
    pub GetDefaultSprite: Option<unsafe extern "C" fn(_: ref_defaultsprite_e)
                                     -> *mut model_s>,
    pub R_StoreEfrags: Option<unsafe extern "C" fn(_: *mut *mut efrag_s,
                                                   _: libc::c_int) -> ()>,
    pub Mod_ForName: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                 _: qboolean, _: qboolean)
                                -> *mut model_t>,
    pub Mod_Extradata: Option<unsafe extern "C" fn(_: libc::c_int,
                                                   _: *mut model_t)
                                  -> *mut libc::c_void>,
    pub pfnGetModelByIndex: Option<unsafe extern "C" fn(_: libc::c_int)
                                       -> *mut model_s>,
    pub Mod_GetCurrentLoadingModel: Option<unsafe extern "C" fn()
                                               -> *mut model_s>,
    pub Mod_SetCurrentLoadingModel: Option<unsafe extern "C" fn(_:
                                                                    *mut model_s)
                                               -> ()>,
    pub CL_GetRemapInfoForEntity: Option<unsafe extern "C" fn(_:
                                                                  *mut cl_entity_t)
                                             -> *mut remap_info_s>,
    pub CL_AllocRemapInfo: Option<unsafe extern "C" fn(_: *mut cl_entity_t,
                                                       _: *mut model_t,
                                                       _: libc::c_int,
                                                       _: libc::c_int) -> ()>,
    pub CL_FreeRemapInfo: Option<unsafe extern "C" fn(_: *mut remap_info_s)
                                     -> ()>,
    pub CL_UpdateRemapInfo: Option<unsafe extern "C" fn(_: *mut cl_entity_t,
                                                        _: libc::c_int,
                                                        _: libc::c_int)
                                       -> ()>,
    pub CL_ExtraUpdate: Option<unsafe extern "C" fn() -> ()>,
    pub Host_Error: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                _: ...) -> ()>,
    pub COM_SetRandomSeed: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub COM_RandomFloat: Option<unsafe extern "C" fn(_: libc::c_float,
                                                     _: libc::c_float)
                                    -> libc::c_float>,
    pub COM_RandomLong: Option<unsafe extern "C" fn(_: libc::c_int,
                                                    _: libc::c_int)
                                   -> libc::c_int>,
    pub GetScreenFade: Option<unsafe extern "C" fn() -> *mut screenfade_s>,
    pub pfnTextMessageGet: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                      -> *mut client_textmessage_s>,
    pub GetPredictedOrigin: Option<unsafe extern "C" fn(_: *mut vec_t) -> ()>,
    pub CL_GetPaletteColor: Option<unsafe extern "C" fn(_: libc::c_int)
                                       -> *mut color24>,
    pub CL_GetScreenInfo: Option<unsafe extern "C" fn(_: *mut libc::c_int,
                                                      _: *mut libc::c_int)
                                     -> ()>,
    pub SetLocalLightLevel: Option<unsafe extern "C" fn(_: libc::c_int)
                                       -> ()>,
    pub Sys_CheckParm: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                  -> libc::c_int>,
    pub pfnPlayerInfo: Option<unsafe extern "C" fn(_: libc::c_int)
                                  -> *mut player_info_t>,
    pub pfnGetPlayerState: Option<unsafe extern "C" fn(_: libc::c_int)
                                      -> *mut entity_state_t>,
    pub Mod_CacheCheck: Option<unsafe extern "C" fn(_: *mut cache_user_s)
                                   -> *mut libc::c_void>,
    pub Mod_LoadCacheFile: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                       _: *mut cache_user_s)
                                      -> ()>,
    pub Mod_Calloc: Option<unsafe extern "C" fn(_: libc::c_int, _: size_t)
                               -> *mut libc::c_void>,
    pub pfnGetStudioModelInterface: Option<unsafe extern "C" fn(_:
                                                                    libc::c_int,
                                                                _:
                                                                    *mut *mut r_studio_interface_s,
                                                                _:
                                                                    *mut engine_studio_api_s)
                                               -> libc::c_int>,
    pub _Mem_AllocPool: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                    _: *const libc::c_char,
                                                    _: libc::c_int)
                                   -> poolhandle_t>,
    pub _Mem_FreePool: Option<unsafe extern "C" fn(_: *mut poolhandle_t,
                                                   _: *const libc::c_char,
                                                   _: libc::c_int) -> ()>,
    pub _Mem_Alloc: Option<unsafe extern "C" fn(_: poolhandle_t, _: size_t,
                                                _: qboolean,
                                                _: *const libc::c_char,
                                                _: libc::c_int)
                               -> *mut libc::c_void>,
    pub _Mem_Realloc: Option<unsafe extern "C" fn(_: poolhandle_t,
                                                  _: *mut libc::c_void,
                                                  _: size_t, _: qboolean,
                                                  _: *const libc::c_char,
                                                  _: libc::c_int)
                                 -> *mut libc::c_void>,
    pub _Mem_Free: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                               _: *const libc::c_char,
                                               _: libc::c_int) -> ()>,
    pub COM_LoadLibrary: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                     _: libc::c_int,
                                                     _: qboolean)
                                    -> *mut libc::c_void>,
    pub COM_FreeLibrary: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                    -> ()>,
    pub COM_GetProcAddress: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                        _:
                                                            *const libc::c_char)
                                       -> *mut libc::c_void>,
    pub COM_LoadFile: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                  _: *mut fs_offset_t,
                                                  _: qboolean) -> *mut byte>,
    pub FS_FileExists: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                   _: libc::c_int)
                                  -> libc::c_int>,
    pub FS_AllowDirectPaths: Option<unsafe extern "C" fn(_: qboolean) -> ()>,
    pub R_Init_Video: Option<unsafe extern "C" fn(_: libc::c_int)
                                 -> qboolean>,
    pub R_Free_Video: Option<unsafe extern "C" fn() -> ()>,
    pub GL_SetAttribute: Option<unsafe extern "C" fn(_: libc::c_int,
                                                     _: libc::c_int)
                                    -> libc::c_int>,
    pub GL_GetAttribute: Option<unsafe extern "C" fn(_: libc::c_int,
                                                     _: *mut libc::c_int)
                                    -> libc::c_int>,
    pub GL_GetProcAddress: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                      -> *mut libc::c_void>,
    pub GL_SwapBuffers: Option<unsafe extern "C" fn() -> ()>,
    pub SW_CreateBuffer: Option<unsafe extern "C" fn(_: libc::c_int,
                                                     _: libc::c_int,
                                                     _: *mut uint,
                                                     _: *mut uint,
                                                     _: *mut uint,
                                                     _: *mut uint,
                                                     _: *mut uint)
                                    -> qboolean>,
    pub SW_LockBuffer: Option<unsafe extern "C" fn() -> *mut libc::c_void>,
    pub SW_UnlockBuffer: Option<unsafe extern "C" fn() -> ()>,
    pub BuildGammaTable: Option<unsafe extern "C" fn(_: libc::c_float,
                                                     _: libc::c_float) -> ()>,
    pub LightToTexGamma: Option<unsafe extern "C" fn(_: byte) -> byte>,
    pub R_DoResetGamma: Option<unsafe extern "C" fn() -> qboolean>,
    pub GetLightStyle: Option<unsafe extern "C" fn(_: libc::c_int)
                                  -> *mut lightstyle_t>,
    pub GetDynamicLight: Option<unsafe extern "C" fn(_: libc::c_int)
                                    -> *mut dlight_t>,
    pub GetEntityLight: Option<unsafe extern "C" fn(_: libc::c_int)
                                   -> *mut dlight_t>,
    pub R_FatPVS: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                              _: libc::c_float, _: *mut byte,
                                              _: qboolean, _: qboolean)
                             -> libc::c_int>,
    pub GetOverviewParms: Option<unsafe extern "C" fn()
                                     -> *const ref_overview_s>,
    pub pfnTime: Option<unsafe extern "C" fn() -> libc::c_double>,
    pub EV_GetPhysent: Option<unsafe extern "C" fn(_: libc::c_int)
                                  -> *mut physent_s>,
    pub EV_TraceSurface: Option<unsafe extern "C" fn(_: libc::c_int,
                                                     _: *mut libc::c_float,
                                                     _: *mut libc::c_float)
                                    -> *mut msurface_s>,
    pub PM_TraceLine: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                  _: *mut libc::c_float,
                                                  _: libc::c_int,
                                                  _: libc::c_int,
                                                  _: libc::c_int)
                                 -> *mut pmtrace_s>,
    pub EV_VisTraceLine: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                     _: *mut libc::c_float,
                                                     _: libc::c_int)
                                    -> *mut pmtrace_s>,
    pub CL_TraceLine: Option<unsafe extern "C" fn(_: *mut vec_t,
                                                  _: *mut vec_t,
                                                  _: libc::c_int)
                                 -> pmtrace_s>,
    pub pfnGetMoveVars: Option<unsafe extern "C" fn() -> *mut movevars_s>,
    pub Image_AddCmdFlags: Option<unsafe extern "C" fn(_: uint) -> ()>,
    pub Image_SetForceFlags: Option<unsafe extern "C" fn(_: uint) -> ()>,
    pub Image_ClearForceFlags: Option<unsafe extern "C" fn() -> ()>,
    pub Image_CustomPalette: Option<unsafe extern "C" fn() -> qboolean>,
    pub Image_Process: Option<unsafe extern "C" fn(_: *mut *mut rgbdata_t,
                                                   _: libc::c_int,
                                                   _: libc::c_int, _: uint,
                                                   _: libc::c_float)
                                  -> qboolean>,
    pub FS_LoadImage: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                  _: *const byte, _: size_t)
                                 -> *mut rgbdata_t>,
    pub FS_SaveImage: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                  _: *mut rgbdata_t)
                                 -> qboolean>,
    pub FS_CopyImage: Option<unsafe extern "C" fn(_: *mut rgbdata_t)
                                 -> *mut rgbdata_t>,
    pub FS_FreeImage: Option<unsafe extern "C" fn(_: *mut rgbdata_t) -> ()>,
    pub Image_SetMDLPointer: Option<unsafe extern "C" fn(_: *mut byte) -> ()>,
    pub Image_GetPool: Option<unsafe extern "C" fn() -> poolhandle_t>,
    pub Image_GetPFDesc: Option<unsafe extern "C" fn(_: libc::c_int)
                                    -> *const bpc_desc_s>,
    pub pfnDrawNormalTriangles: Option<unsafe extern "C" fn() -> ()>,
    pub pfnDrawTransparentTriangles: Option<unsafe extern "C" fn() -> ()>,
    pub drawFuncs: *mut render_interface_t,
}
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
pub type ref_api_t = ref_api_s;
pub type GLenum = uint;
pub type GLboolean = byte;
pub type GLvoid = ();
pub type GLint = libc::c_int;
pub type GLuint = uint;
pub type GLsizei = libc::c_int;
pub type GLfloat = libc::c_float;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gltexture_s {
    pub name: [libc::c_char; 256],
    pub srcWidth: word,
    pub srcHeight: word,
    pub width: word,
    pub height: word,
    pub depth: word,
    pub numMips: byte,
    pub target: GLuint,
    pub texnum: GLuint,
    pub format: GLint,
    pub encode: GLint,
    pub flags: texFlags_t,
    pub fogParams: rgba_t,
    pub original: *mut rgbdata_t,
    pub size: size_t,
    pub xscale: libc::c_float,
    pub yscale: libc::c_float,
    pub servercount: libc::c_int,
    pub hashValue: uint,
    pub nextHash: *mut gltexture_s,
}
pub type gl_texture_t = gltexture_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct draw_list_t {
    pub solid_entities: [*mut cl_entity_t; 2048],
    pub trans_entities: [*mut cl_entity_t; 2048],
    pub beam_entities: [*mut cl_entity_t; 2048],
    pub num_solid_entities: uint,
    pub num_trans_entities: uint,
    pub num_beam_entities: uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gl_globals_t {
    pub defaultTexture: libc::c_int,
    pub particleTexture: libc::c_int,
    pub whiteTexture: libc::c_int,
    pub grayTexture: libc::c_int,
    pub blackTexture: libc::c_int,
    pub solidskyTexture: libc::c_int,
    pub alphaskyTexture: libc::c_int,
    pub lightmapTextures: [libc::c_int; 256],
    pub dlightTexture: libc::c_int,
    pub skyboxTextures: [libc::c_int; 6],
    pub cinTexture: libc::c_int,
    pub skytexturenum: libc::c_int,
    pub skyboxbasenum: libc::c_int,
    pub draw_stack: [draw_list_t; 2],
    pub draw_stack_pos: libc::c_int,
    pub draw_list: *mut draw_list_t,
    pub draw_decals: [*mut msurface_t; 4096],
    pub num_draw_decals: libc::c_int,
    pub modelviewIdentity: qboolean,
    pub visframecount: libc::c_int,
    pub dlightframecount: libc::c_int,
    pub realframecount: libc::c_int,
    pub framecount: libc::c_int,
    pub ignore_lightgamma: qboolean,
    pub fCustomRendering: qboolean,
    pub fResetVis: qboolean,
    pub fFlipViewModel: qboolean,
    pub visbytes: [byte; 4096],
    pub lightstylevalue: [libc::c_int; 64],
    pub block_size: libc::c_int,
    pub frametime: libc::c_double,
    pub blend: libc::c_float,
    pub modelorg: vec3_t,
    pub fCustomSkybox: qboolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glstate_t {
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub activeTMU: libc::c_int,
    pub currentTextures: [GLint; 32],
    pub currentTextureTargets: [GLuint; 32],
    pub texIdentityMatrix: [GLboolean; 32],
    pub genSTEnabled: [GLint; 32],
    pub texCoordArrayMode: [GLint; 32],
    pub isFogEnabled: GLint,
    pub faceCull: libc::c_int,
    pub stencilEnabled: qboolean,
    pub in2DMode: qboolean,
}
pub const GL_TEXTURE_LOD_BIAS: C2RustUnnamed_3 = 4;
pub const GL_ANISOTROPY_EXT: C2RustUnnamed_3 = 3;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glconfig_t {
    pub renderer_string: *const libc::c_char,
    pub vendor_string: *const libc::c_char,
    pub version_string: *const libc::c_char,
    pub hardware_type: glHWType_t,
    pub extensions_string: *const libc::c_char,
    pub extension: [byte; 22],
    pub max_texture_units: libc::c_int,
    pub max_texture_coords: libc::c_int,
    pub max_teximage_units: libc::c_int,
    pub max_2d_texture_size: GLint,
    pub max_2d_rectangle_size: GLint,
    pub max_2d_texture_layers: GLint,
    pub max_3d_texture_size: GLint,
    pub max_cubemap_size: GLint,
    pub max_texture_anisotropy: GLfloat,
    pub max_texture_lod_bias: GLfloat,
    pub max_vertex_uniforms: GLint,
    pub max_vertex_attribs: GLint,
    pub max_multisamples: GLint,
    pub color_bits: libc::c_int,
    pub alpha_bits: libc::c_int,
    pub depth_bits: libc::c_int,
    pub stencil_bits: libc::c_int,
    pub msaasamples: libc::c_int,
    pub context: gl_context_type_t,
    pub wrapper: gles_wrapper_t,
    pub softwareGammaUpdate: qboolean,
    pub fCustomRenderer: qboolean,
    pub prev_width: libc::c_int,
    pub prev_height: libc::c_int,
}
pub type glHWType_t = libc::c_uint;
pub const GLHW_INTEL: glHWType_t = 3;
pub const GLHW_NVIDIA: glHWType_t = 2;
pub const GLHW_RADEON: glHWType_t = 1;
pub const GLHW_GENERIC: glHWType_t = 0;
pub const GL_CLAMPTOEDGE_EXT: C2RustUnnamed_3 = 10;
pub const GL_ARB_SEAMLESS_CUBEMAP: C2RustUnnamed_3 = 15;
pub const GL_CLAMP_TEXBORDER_EXT: C2RustUnnamed_3 = 12;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glwstate_t {
    pub initialized: qboolean,
    pub extended: qboolean,
}
pub const GL_ARB_TEXTURE_FLOAT_EXT: C2RustUnnamed_3 = 13;
pub const GL_ARB_DEPTH_FLOAT_EXT: C2RustUnnamed_3 = 14;
pub const GL_ARB_TEXTURE_NPOT_EXT: C2RustUnnamed_3 = 11;
pub const GL_TEXTURE_MULTISAMPLE: C2RustUnnamed_3 = 21;
pub const GL_EXT_GPU_SHADER4: C2RustUnnamed_3 = 16;
pub const GL_DEPTH_TEXTURE: C2RustUnnamed_3 = 17;
pub const GL_TEXTURE_3D_EXT: C2RustUnnamed_3 = 9;
pub const GL_TEXTURE_ARRAY_EXT: C2RustUnnamed_3 = 8;
pub const GL_TEXTURE_2D_RECT_EXT: C2RustUnnamed_3 = 7;
pub const GL_TEXTURE_CUBEMAP_EXT: C2RustUnnamed_3 = 2;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const GL_EXTCOUNT: C2RustUnnamed_3 = 22;
pub const GL_DRAW_RANGEELEMENTS_EXT: C2RustUnnamed_3 = 20;
pub const GL_ARB_VERTEX_BUFFER_OBJECT_EXT: C2RustUnnamed_3 = 19;
pub const GL_DEBUG_OUTPUT: C2RustUnnamed_3 = 18;
pub const GL_SHADER_GLSL100_EXT: C2RustUnnamed_3 = 6;
pub const GL_TEXTURE_COMPRESSION_EXT: C2RustUnnamed_3 = 5;
pub const GL_ARB_MULTITEXTURE: C2RustUnnamed_3 = 1;
pub const GL_OPENGL_110: C2RustUnnamed_3 = 0;
#[inline(always)]
unsafe extern "C" fn __tg_sqrt(mut __x: libc::c_double) -> libc::c_double {
    return sqrt(__x);
}
static mut gl_textures: [gl_texture_t; 4096] =
    [gl_texture_t{name: [0; 256],
                  srcWidth: 0,
                  srcHeight: 0,
                  width: 0,
                  height: 0,
                  depth: 0,
                  numMips: 0,
                  target: 0,
                  texnum: 0,
                  format: 0,
                  encode: 0,
                  flags: TF_COLORMAP,
                  fogParams: [0; 4],
                  original: 0 as *const rgbdata_t as *mut rgbdata_t,
                  size: 0,
                  xscale: 0.,
                  yscale: 0.,
                  servercount: 0,
                  hashValue: 0,
                  nextHash: 0 as *const gltexture_s as *mut gltexture_s,};
        4096];
static mut gl_texturesHashTable: [*mut gl_texture_t; 1024] =
    [0 as *const gl_texture_t as *mut gl_texture_t; 1024];
static mut gl_numTextures: uint = 0;
/*
=================
R_GetTexture

acess to array elem
=================
*/
#[no_mangle]
pub unsafe extern "C" fn R_GetTexture(mut texnum: GLenum)
 -> *mut gl_texture_t {
    if !(texnum >= 0 as libc::c_int as libc::c_uint &&
             texnum < 4096 as libc::c_int as libc::c_uint) {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 b"../ref_gl/gl_image.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 35 as
                                                                     libc::c_int);
    }
    return &mut *gl_textures.as_mut_ptr().offset(texnum as isize) as
               *mut gl_texture_t;
}
/*
=================
GL_TargetToString
=================
*/
unsafe extern "C" fn GL_TargetToString(mut target: GLenum)
 -> *const libc::c_char {
    match target {
        3552 => { return b"1D\x00" as *const u8 as *const libc::c_char }
        3553 => { return b"2D\x00" as *const u8 as *const libc::c_char }
        37120 => {
            return b"2D Multisample\x00" as *const u8 as *const libc::c_char
        }
        32879 => { return b"3D\x00" as *const u8 as *const libc::c_char }
        34067 => { return b"Cube\x00" as *const u8 as *const libc::c_char }
        35866 => { return b"Array\x00" as *const u8 as *const libc::c_char }
        34037 => { return b"Rect\x00" as *const u8 as *const libc::c_char }
        _ => { }
    }
    return b"??\x00" as *const u8 as *const libc::c_char;
}
/*
=================
GL_Bind
=================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_Bind(mut tmu: GLint, mut texnum: GLenum) {
    let mut texture: *mut gl_texture_t = 0 as *mut gl_texture_t;
    let mut glTarget: GLuint = 0;
    // missed or invalid texture?
    if texnum <= 0 as libc::c_int as libc::c_uint ||
           texnum >= 4096 as libc::c_int as libc::c_uint {
        if texnum != 0 as libc::c_int as libc::c_uint {
            gEngfuncs.Con_DPrintf.expect("non-null function pointer")(b"^1Error:^7 GL_Bind: invalid texturenum %d\n\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char,
                                                                      texnum);
        }
        texnum = tr.defaultTexture as GLenum
    }
    if tmu != GL_KEEP_UNIT as libc::c_int {
        GL_SelectTexture(tmu);
    } else { tmu = glState.activeTMU }
    texture =
        &mut *gl_textures.as_mut_ptr().offset(texnum as isize) as
            *mut gl_texture_t;
    glTarget = (*texture).target;
    if glTarget == 0x8c1a as libc::c_int as libc::c_uint {
        glTarget = 0xde1 as libc::c_int as GLuint
    }
    if glState.currentTextureTargets[tmu as usize] != glTarget {
        if glState.currentTextureTargets[tmu as usize] !=
               0 as libc::c_int as libc::c_uint {
            pglDisable.expect("non-null function pointer")(glState.currentTextureTargets[tmu
                                                                                             as
                                                                                             usize]);
        }
        glState.currentTextureTargets[tmu as usize] = glTarget;
        pglEnable.expect("non-null function pointer")(glState.currentTextureTargets[tmu
                                                                                        as
                                                                                        usize]);
    }
    if glState.currentTextures[tmu as usize] as libc::c_uint ==
           (*texture).texnum {
        return
    }
    pglBindTexture.expect("non-null function pointer")((*texture).target,
                                                       (*texture).texnum);
    glState.currentTextures[tmu as usize] = (*texture).texnum as GLint;
}
/*
=================
GL_ApplyTextureParams
=================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_ApplyTextureParams(mut tex: *mut gl_texture_t) {
    let mut border: vec4_t = [0.0f32, 0.0f32, 0.0f32, 1.0f32];
    if glw_state.initialized as u64 == 0 { return }
    if tex.is_null() {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 b"../ref_gl/gl_image.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 120 as
                                                                     libc::c_int);
    }
    // multisample textures does not support any sampling state changing
    if (*tex).flags as libc::c_uint &
           TF_MULTISAMPLE as libc::c_int as libc::c_uint != 0 {
        return
    }
    // set texture filter
    if (*tex).flags as libc::c_uint &
           TF_DEPTHMAP as libc::c_int as libc::c_uint != 0 {
        if (*tex).flags as libc::c_uint &
               TF_NOCOMPARE as libc::c_int as libc::c_uint == 0 {
            pglTexParameteri.expect("non-null function pointer")((*tex).target,
                                                                 0x884c as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 0x884e as
                                                                     libc::c_int);
            pglTexParameteri.expect("non-null function pointer")((*tex).target,
                                                                 0x884d as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 0x203 as
                                                                     libc::c_int);
        }
        if (*tex).flags as libc::c_uint &
               TF_LUMINANCE as libc::c_int as libc::c_uint != 0 {
            pglTexParameteri.expect("non-null function pointer")((*tex).target,
                                                                 0x884b as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 0x1909 as
                                                                     libc::c_int);
        } else {
            pglTexParameteri.expect("non-null function pointer")((*tex).target,
                                                                 0x884b as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 0x8049 as
                                                                     libc::c_int);
        }
        if (*tex).flags as libc::c_uint &
               TF_NEAREST as libc::c_int as libc::c_uint != 0 {
            pglTexParameteri.expect("non-null function pointer")((*tex).target,
                                                                 0x2801 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 0x2600 as
                                                                     libc::c_int);
            pglTexParameteri.expect("non-null function pointer")((*tex).target,
                                                                 0x2800 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 0x2600 as
                                                                     libc::c_int);
        } else {
            pglTexParameteri.expect("non-null function pointer")((*tex).target,
                                                                 0x2801 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 0x2601 as
                                                                     libc::c_int);
            pglTexParameteri.expect("non-null function pointer")((*tex).target,
                                                                 0x2800 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 0x2601 as
                                                                     libc::c_int);
        }
        // allow max anisotropy as 1.0f on depth textures
        if GL_Support(GL_ANISOTROPY_EXT as libc::c_int) as u64 != 0 {
            pglTexParameterf.expect("non-null function pointer")((*tex).target,
                                                                 0x84fe as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 1.0f32);
        }
    } else if (*tex).flags as libc::c_uint &
                  TF_NOMIPMAP as libc::c_int as libc::c_uint != 0 ||
                  (*tex).numMips as libc::c_int <= 1 as libc::c_int {
        if (*tex).flags as libc::c_uint &
               TF_NEAREST as libc::c_int as libc::c_uint != 0 ||
               (*tex).flags as libc::c_uint &
                   TF_ATLAS_PAGE as libc::c_int as libc::c_uint != 0 &&
                   (*gl_lightmap_nearest).value != 0. {
            pglTexParameteri.expect("non-null function pointer")((*tex).target,
                                                                 0x2801 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 0x2600 as
                                                                     libc::c_int);
            pglTexParameteri.expect("non-null function pointer")((*tex).target,
                                                                 0x2800 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 0x2600 as
                                                                     libc::c_int);
        } else {
            pglTexParameteri.expect("non-null function pointer")((*tex).target,
                                                                 0x2801 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 0x2601 as
                                                                     libc::c_int);
            pglTexParameteri.expect("non-null function pointer")((*tex).target,
                                                                 0x2800 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 0x2601 as
                                                                     libc::c_int);
        }
    } else {
        if (*tex).flags as libc::c_uint &
               TF_NEAREST as libc::c_int as libc::c_uint != 0 ||
               (*gl_texture_nearest).value != 0. {
            pglTexParameteri.expect("non-null function pointer")((*tex).target,
                                                                 0x2801 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 0x2700 as
                                                                     libc::c_int);
            pglTexParameteri.expect("non-null function pointer")((*tex).target,
                                                                 0x2800 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 0x2600 as
                                                                     libc::c_int);
        } else {
            pglTexParameteri.expect("non-null function pointer")((*tex).target,
                                                                 0x2801 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 0x2703 as
                                                                     libc::c_int);
            pglTexParameteri.expect("non-null function pointer")((*tex).target,
                                                                 0x2800 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 0x2601 as
                                                                     libc::c_int);
        }
        // set texture anisotropy if available
        if GL_Support(GL_ANISOTROPY_EXT as libc::c_int) as libc::c_uint != 0
               && (*tex).numMips as libc::c_int > 1 as libc::c_int &&
               (*tex).flags as libc::c_uint &
                   TF_ALPHACONTRAST as libc::c_int as libc::c_uint == 0 {
            pglTexParameterf.expect("non-null function pointer")((*tex).target,
                                                                 0x84fe as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 (*gl_texture_anisotropy).value);
        }
        // set texture LOD bias if available
        if GL_Support(GL_TEXTURE_LOD_BIAS as libc::c_int) as libc::c_uint != 0
               && (*tex).numMips as libc::c_int > 1 as libc::c_int {
            pglTexParameterf.expect("non-null function pointer")((*tex).target,
                                                                 0x8501 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 (*gl_texture_lodbias).value);
        }
    }
    // check if border is not supported
    if (*tex).flags as libc::c_uint & TF_BORDER as libc::c_int as libc::c_uint
           != 0 &&
           GL_Support(GL_CLAMP_TEXBORDER_EXT as libc::c_int) as u64 == 0 {
        (*tex).flags =
            ((*tex).flags as libc::c_uint &
                 !(TF_BORDER as libc::c_int) as libc::c_uint) as texFlags_t;
        (*tex).flags =
            ((*tex).flags as libc::c_uint |
                 TF_CLAMP as libc::c_int as libc::c_uint) as texFlags_t
    }
    // only seamless cubemaps allows wrap 'clamp_to_border"
    if (*tex).target == 0x8513 as libc::c_int as libc::c_uint &&
           GL_Support(GL_ARB_SEAMLESS_CUBEMAP as libc::c_int) as u64 == 0 &&
           (*tex).flags as libc::c_uint &
               TF_BORDER as libc::c_int as libc::c_uint != 0 {
        (*tex).flags =
            ((*tex).flags as libc::c_uint &
                 !(TF_BORDER as libc::c_int) as libc::c_uint) as texFlags_t
    }
    // set texture wrap
    if (*tex).flags as libc::c_uint & TF_BORDER as libc::c_int as libc::c_uint
           != 0 {
        pglTexParameteri.expect("non-null function pointer")((*tex).target,
                                                             0x2802 as
                                                                 libc::c_int
                                                                 as GLenum,
                                                             0x812d as
                                                                 libc::c_int);
        if (*tex).target != 0xde0 as libc::c_int as libc::c_uint {
            pglTexParameteri.expect("non-null function pointer")((*tex).target,
                                                                 0x2803 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 0x812d as
                                                                     libc::c_int);
        }
        if (*tex).target == 0x806f as libc::c_int as libc::c_uint ||
               (*tex).target == 0x8513 as libc::c_int as libc::c_uint {
            pglTexParameteri.expect("non-null function pointer")((*tex).target,
                                                                 0x8072 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 0x812d as
                                                                     libc::c_int);
        }
        pglTexParameterfv.expect("non-null function pointer")((*tex).target,
                                                              0x1004 as
                                                                  libc::c_int
                                                                  as GLenum,
                                                              border.as_mut_ptr());
    } else if (*tex).flags as libc::c_uint &
                  TF_CLAMP as libc::c_int as libc::c_uint != 0 {
        if GL_Support(GL_CLAMPTOEDGE_EXT as libc::c_int) as u64 != 0 {
            pglTexParameteri.expect("non-null function pointer")((*tex).target,
                                                                 0x2802 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 0x812f as
                                                                     libc::c_int);
            if (*tex).target != 0xde0 as libc::c_int as libc::c_uint {
                pglTexParameteri.expect("non-null function pointer")((*tex).target,
                                                                     0x2803 as
                                                                         libc::c_int
                                                                         as
                                                                         GLenum,
                                                                     0x812f as
                                                                         libc::c_int);
            }
            if (*tex).target == 0x806f as libc::c_int as libc::c_uint ||
                   (*tex).target == 0x8513 as libc::c_int as libc::c_uint {
                pglTexParameteri.expect("non-null function pointer")((*tex).target,
                                                                     0x8072 as
                                                                         libc::c_int
                                                                         as
                                                                         GLenum,
                                                                     0x812f as
                                                                         libc::c_int);
            }
        } else {
            pglTexParameteri.expect("non-null function pointer")((*tex).target,
                                                                 0x2802 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 0x2900 as
                                                                     libc::c_int);
            if (*tex).target != 0xde0 as libc::c_int as libc::c_uint {
                pglTexParameteri.expect("non-null function pointer")((*tex).target,
                                                                     0x2803 as
                                                                         libc::c_int
                                                                         as
                                                                         GLenum,
                                                                     0x2900 as
                                                                         libc::c_int);
            }
            if (*tex).target == 0x806f as libc::c_int as libc::c_uint ||
                   (*tex).target == 0x8513 as libc::c_int as libc::c_uint {
                pglTexParameteri.expect("non-null function pointer")((*tex).target,
                                                                     0x8072 as
                                                                         libc::c_int
                                                                         as
                                                                         GLenum,
                                                                     0x2900 as
                                                                         libc::c_int);
            }
        }
    } else {
        pglTexParameteri.expect("non-null function pointer")((*tex).target,
                                                             0x2802 as
                                                                 libc::c_int
                                                                 as GLenum,
                                                             0x2901 as
                                                                 libc::c_int);
        if (*tex).target != 0xde0 as libc::c_int as libc::c_uint {
            pglTexParameteri.expect("non-null function pointer")((*tex).target,
                                                                 0x2803 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 0x2901 as
                                                                     libc::c_int);
        }
        if (*tex).target == 0x806f as libc::c_int as libc::c_uint ||
               (*tex).target == 0x8513 as libc::c_int as libc::c_uint {
            pglTexParameteri.expect("non-null function pointer")((*tex).target,
                                                                 0x8072 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 0x2901 as
                                                                     libc::c_int);
        }
    };
}
/*
=================
GL_UpdateTextureParams
=================
*/
unsafe extern "C" fn GL_UpdateTextureParams(mut iTexture: libc::c_int) {
    let mut tex: *mut gl_texture_t =
        &mut *gl_textures.as_mut_ptr().offset(iTexture as isize) as
            *mut gl_texture_t; // free slot
    if tex.is_null() {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 b"../ref_gl/gl_image.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 257 as
                                                                     libc::c_int);
    }
    if (*tex).texnum == 0 { return }
    GL_Bind(XASH_TEXTURE0 as libc::c_int, iTexture as GLenum);
    // set texture anisotropy if available
    if GL_Support(GL_ANISOTROPY_EXT as libc::c_int) as libc::c_uint != 0 &&
           (*tex).numMips as libc::c_int > 1 as libc::c_int &&
           (*tex).flags as libc::c_uint &
               (TF_DEPTHMAP as libc::c_int | TF_ALPHACONTRAST as libc::c_int)
                   as libc::c_uint == 0 {
        pglTexParameterf.expect("non-null function pointer")((*tex).target,
                                                             0x84fe as
                                                                 libc::c_int
                                                                 as GLenum,
                                                             (*gl_texture_anisotropy).value);
    }
    // set texture LOD bias if available
    if GL_Support(GL_TEXTURE_LOD_BIAS as libc::c_int) as libc::c_uint != 0 &&
           (*tex).numMips as libc::c_int > 1 as libc::c_int &&
           (*tex).flags as libc::c_uint &
               TF_DEPTHMAP as libc::c_int as libc::c_uint == 0 {
        pglTexParameterf.expect("non-null function pointer")((*tex).target,
                                                             0x8501 as
                                                                 libc::c_int
                                                                 as GLenum,
                                                             (*gl_texture_lodbias).value);
    }
    if (*tex).flags as libc::c_uint &
           TF_ATLAS_PAGE as libc::c_int as libc::c_uint != 0 {
        if (*gl_lightmap_nearest).value != 0. {
            pglTexParameteri.expect("non-null function pointer")((*tex).target,
                                                                 0x2801 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 0x2600 as
                                                                     libc::c_int);
            pglTexParameteri.expect("non-null function pointer")((*tex).target,
                                                                 0x2800 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 0x2600 as
                                                                     libc::c_int);
        } else {
            pglTexParameteri.expect("non-null function pointer")((*tex).target,
                                                                 0x2801 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 0x2601 as
                                                                     libc::c_int);
            pglTexParameteri.expect("non-null function pointer")((*tex).target,
                                                                 0x2800 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 0x2601 as
                                                                     libc::c_int);
        }
    }
    if (*tex).numMips as libc::c_int <= 1 as libc::c_int { return }
    if (*tex).flags as libc::c_uint &
           TF_NEAREST as libc::c_int as libc::c_uint != 0 ||
           (*gl_texture_nearest).value != 0. {
        pglTexParameteri.expect("non-null function pointer")((*tex).target,
                                                             0x2801 as
                                                                 libc::c_int
                                                                 as GLenum,
                                                             0x2700 as
                                                                 libc::c_int);
        pglTexParameteri.expect("non-null function pointer")((*tex).target,
                                                             0x2800 as
                                                                 libc::c_int
                                                                 as GLenum,
                                                             0x2600 as
                                                                 libc::c_int);
    } else {
        pglTexParameteri.expect("non-null function pointer")((*tex).target,
                                                             0x2801 as
                                                                 libc::c_int
                                                                 as GLenum,
                                                             0x2703 as
                                                                 libc::c_int);
        pglTexParameteri.expect("non-null function pointer")((*tex).target,
                                                             0x2800 as
                                                                 libc::c_int
                                                                 as GLenum,
                                                             0x2601 as
                                                                 libc::c_int);
    };
}
/*
=================
R_SetTextureParameters
=================
*/
#[no_mangle]
pub unsafe extern "C" fn R_SetTextureParameters() {
    let mut i: libc::c_int = 0;
    if GL_Support(GL_ANISOTROPY_EXT as libc::c_int) as u64 != 0 {
        if (*gl_texture_anisotropy).value > glConfig.max_texture_anisotropy {
            gEngfuncs.Cvar_SetValue.expect("non-null function pointer")(b"gl_anisotropy\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char,
                                                                        glConfig.max_texture_anisotropy);
        } else if (*gl_texture_anisotropy).value < 1.0f32 {
            gEngfuncs.Cvar_SetValue.expect("non-null function pointer")(b"gl_anisotropy\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char,
                                                                        1.0f32);
        }
    }
    if GL_Support(GL_TEXTURE_LOD_BIAS as libc::c_int) as u64 != 0 {
        if (*gl_texture_lodbias).value < -glConfig.max_texture_lod_bias {
            gEngfuncs.Cvar_SetValue.expect("non-null function pointer")(b"gl_texture_lodbias\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char,
                                                                        -glConfig.max_texture_lod_bias);
        } else if (*gl_texture_lodbias).value > glConfig.max_texture_lod_bias
         {
            gEngfuncs.Cvar_SetValue.expect("non-null function pointer")(b"gl_texture_lodbias\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char,
                                                                        glConfig.max_texture_lod_bias);
        }
    }
    (*gl_texture_anisotropy).flags =
        (*gl_texture_anisotropy).flags &
            !((1 as libc::c_int) << 13 as libc::c_int);
    (*gl_texture_lodbias).flags =
        (*gl_texture_lodbias).flags &
            !((1 as libc::c_int) << 13 as libc::c_int);
    (*gl_texture_nearest).flags =
        (*gl_texture_nearest).flags &
            !((1 as libc::c_int) << 13 as libc::c_int);
    (*gl_lightmap_nearest).flags =
        (*gl_lightmap_nearest).flags &
            !((1 as libc::c_int) << 13 as libc::c_int);
    // change all the existing mipmapped texture objects
    i = 0 as libc::c_int;
    while (i as libc::c_uint) < gl_numTextures {
        GL_UpdateTextureParams(i);
        i += 1
    };
}
/*
================
GL_CalcTextureSamples
================
*/
unsafe extern "C" fn GL_CalcTextureSamples(mut flags: libc::c_int)
 -> libc::c_int {
    if flags & IMAGE_HAS_COLOR as libc::c_int != 0 {
        return if flags & IMAGE_HAS_ALPHA as libc::c_int != 0 {
                   4 as libc::c_int
               } else { 3 as libc::c_int }
    }
    return if flags & IMAGE_HAS_ALPHA as libc::c_int != 0 {
               2 as libc::c_int
           } else { 1 as libc::c_int };
}
/*
==================
GL_CalcImageSize
==================
*/
unsafe extern "C" fn GL_CalcImageSize(mut format: pixformat_t,
                                      mut width: libc::c_int,
                                      mut height: libc::c_int,
                                      mut depth: libc::c_int) -> size_t {
    let mut size: size_t = 0 as libc::c_int as size_t;
    // check the depth error
    depth = if 1 as libc::c_int > depth { 1 as libc::c_int } else { depth };
    match format as libc::c_uint {
        7 => { size = (width * height * depth) as size_t }
        5 | 6 => {
            size = (width * height * depth * 3 as libc::c_int) as size_t
        }
        4 | 3 => {
            size = (width * height * depth * 4 as libc::c_int) as size_t
        }
        8 => {
            size =
                ((width + 3 as libc::c_int >> 2 as libc::c_int) *
                     (height + 3 as libc::c_int >> 2 as libc::c_int) *
                     8 as libc::c_int * depth) as size_t
        }
        9 | 10 | 11 => {
            size =
                ((width + 3 as libc::c_int >> 2 as libc::c_int) *
                     (height + 3 as libc::c_int >> 2 as libc::c_int) *
                     16 as libc::c_int * depth) as size_t
        }
        _ => { }
    }
    return size;
}
/*
==================
GL_CalcTextureSize
==================
*/
unsafe extern "C" fn GL_CalcTextureSize(mut format: GLenum,
                                        mut width: libc::c_int,
                                        mut height: libc::c_int,
                                        mut depth: libc::c_int) -> size_t {
    let mut size: size_t = 0 as libc::c_int as size_t;
    // check the depth error
    depth =
        if 1 as libc::c_int > depth {
            1 as libc::c_int
        } else { depth }; // half-floats
    match format {
        33776 | 33777 => {
            size =
                ((width + 3 as libc::c_int >> 2 as libc::c_int) *
                     (height + 3 as libc::c_int >> 2 as libc::c_int) *
                     8 as libc::c_int * depth) as size_t
        }
        33778 | 33779 | 36285 | 34027 | 34871 => {
            size =
                ((width + 3 as libc::c_int >> 2 as libc::c_int) *
                     (height + 3 as libc::c_int >> 2 as libc::c_int) *
                     16 as libc::c_int * depth) as size_t
        }
        32856 | 6408 => {
            size = (width * height * depth * 4 as libc::c_int) as size_t
        }
        32849 | 6407 => {
            size = (width * height * depth * 3 as libc::c_int) as size_t
        }
        32848 => {
            size =
                (width * height * depth * 3 as libc::c_int / 2 as libc::c_int)
                    as size_t
        }
        32854 => {
            size =
                (width * height * depth * 4 as libc::c_int / 2 as libc::c_int)
                    as size_t
        }
        32841 | 6409 | 32843 | 32832 => {
            size = (width * height * depth) as size_t
        }
        6410 | 32837 => {
            size = (width * height * depth * 2 as libc::c_int) as size_t
        }
        33321 => { size = (width * height * depth) as size_t }
        33323 => {
            size = (width * height * depth * 2 as libc::c_int) as size_t
        }
        33322 => {
            size = (width * height * depth * 2 as libc::c_int) as size_t
        }
        33324 => {
            size = (width * height * depth * 4 as libc::c_int) as size_t
        }
        33325 | 34846 => {
            size = (width * height * depth * 2 as libc::c_int) as size_t
        }
        33326 | 34840 => {
            size = (width * height * depth * 4 as libc::c_int) as size_t
        }
        33327 | 34847 => {
            size = (width * height * depth * 4 as libc::c_int) as size_t
        }
        33328 | 34841 => {
            size = (width * height * depth * 8 as libc::c_int) as size_t
        }
        34843 => {
            size = (width * height * depth * 6 as libc::c_int) as size_t
        }
        34842 => {
            size = (width * height * depth * 8 as libc::c_int) as size_t
        }
        34837 => {
            size = (width * height * depth * 12 as libc::c_int) as size_t
        }
        34836 => {
            size = (width * height * depth * 16 as libc::c_int) as size_t
        }
        33189 => {
            size = (width * height * depth * 2 as libc::c_int) as size_t
        }
        33190 => {
            size = (width * height * depth * 3 as libc::c_int) as size_t
        }
        36012 => {
            size = (width * height * depth * 4 as libc::c_int) as size_t
        }
        _ => {
            gEngfuncs.Host_Error.expect("non-null function pointer")(b"GL_CalcTextureSize: bad texture internal format (%u)\n\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     format);
        }
    }
    return size;
}
unsafe extern "C" fn GL_CalcMipmapCount(mut tex: *mut gl_texture_t,
                                        mut haveBuffer: qboolean)
 -> libc::c_int {
    let mut width: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    let mut mipcount: libc::c_int = 0;
    if tex.is_null() {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 b"../ref_gl/gl_image.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 495 as
                                                                     libc::c_int);
    }
    if haveBuffer as u64 == 0 ||
           (*tex).target == 0x806f as libc::c_int as libc::c_uint {
        return 1 as libc::c_int
    }
    // generate mip-levels by user request
    if (*tex).flags as libc::c_uint &
           TF_NOMIPMAP as libc::c_int as libc::c_uint != 0 {
        return 1 as libc::c_int
    }
    // mip-maps can't exceeds 16
    mipcount = 0 as libc::c_int;
    while mipcount < 16 as libc::c_int {
        width =
            if 1 as libc::c_int > (*tex).width as libc::c_int >> mipcount {
                1 as libc::c_int
            } else { ((*tex).width as libc::c_int) >> mipcount };
        height =
            if 1 as libc::c_int > (*tex).height as libc::c_int >> mipcount {
                1 as libc::c_int
            } else { ((*tex).height as libc::c_int) >> mipcount };
        if width == 1 as libc::c_int && height == 1 as libc::c_int { break ; }
        mipcount += 1
    }
    return mipcount + 1 as libc::c_int;
}
/*
================
GL_SetTextureDimensions
================
*/
unsafe extern "C" fn GL_SetTextureDimensions(mut tex: *mut gl_texture_t,
                                             mut width: libc::c_int,
                                             mut height: libc::c_int,
                                             mut depth: libc::c_int) {
    let mut maxTextureSize: libc::c_int = 0 as libc::c_int;
    let mut maxDepthSize: libc::c_int = 1 as libc::c_int;
    if tex.is_null() {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 b"../ref_gl/gl_image.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 526 as
                                                                     libc::c_int);
    }
    match (*tex).target {
        3552 | 3553 | 37120 => {
            maxTextureSize = glConfig.max_2d_texture_size
        }
        35866 => {
            maxDepthSize = glConfig.max_2d_texture_layers;
            maxTextureSize = glConfig.max_2d_texture_size
        }
        34037 => { maxTextureSize = glConfig.max_2d_rectangle_size }
        34067 => { maxTextureSize = glConfig.max_cubemap_size }
        32879 => {
            maxDepthSize = glConfig.max_3d_texture_size;
            maxTextureSize = glConfig.max_3d_texture_size
        }
        _ => {
            if false_0 as libc::c_int == 0 {
                gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char,
                                                                         b"../ref_gl/gl_image.c\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char,
                                                                         550
                                                                             as
                                                                             libc::c_int);
            }
        }
    }
    // store original sizes
    (*tex).srcWidth = width as word;
    (*tex).srcHeight = height as word;
    if GL_Support(GL_ARB_TEXTURE_NPOT_EXT as libc::c_int) as u64 == 0 {
        let mut step: libc::c_int = (*gl_round_down).value as libc::c_int;
        let mut scaled_width: libc::c_int = 0;
        let mut scaled_height: libc::c_int = 0;
        scaled_width = 1 as libc::c_int;
        while scaled_width < width { scaled_width <<= 1 as libc::c_int }
        if step > 0 as libc::c_int && width < scaled_width &&
               (step == 1 as libc::c_int ||
                    scaled_width - width > scaled_width >> step) {
            scaled_width >>= 1 as libc::c_int
        }
        scaled_height = 1 as libc::c_int;
        while scaled_height < height { scaled_height <<= 1 as libc::c_int }
        if step > 0 as libc::c_int && height < scaled_height &&
               (step == 1 as libc::c_int ||
                    scaled_height - height > scaled_height >> step) {
            scaled_height >>= 1 as libc::c_int
        }
        width = scaled_width;
        height = scaled_height
    }
    if width > maxTextureSize || height > maxTextureSize ||
           depth > maxDepthSize {
        if (*tex).target == 0xde0 as libc::c_int as libc::c_uint {
            while width > maxTextureSize { width >>= 1 as libc::c_int }
        } else if (*tex).target == 0x806f as libc::c_int as libc::c_uint ||
                      (*tex).target == 0x8c1a as libc::c_int as libc::c_uint {
            while width > maxTextureSize || height > maxTextureSize ||
                      depth > maxDepthSize {
                width >>= 1 as libc::c_int;
                height >>= 1 as libc::c_int;
                depth >>= 1 as libc::c_int
            }
        } else {
            // all remaining cases
            while width > maxTextureSize || height > maxTextureSize {
                width >>= 1 as libc::c_int;
                height >>= 1 as libc::c_int
            }
        }
    }
    // set the texture dimensions
    (*tex).width =
        if 1 as libc::c_int > width { 1 as libc::c_int } else { width } as
            word;
    (*tex).height =
        if 1 as libc::c_int > height { 1 as libc::c_int } else { height } as
            word;
    (*tex).depth =
        if 1 as libc::c_int > depth { 1 as libc::c_int } else { depth } as
            word;
}
/*
===============
GL_SetTextureTarget
===============
*/
unsafe extern "C" fn GL_SetTextureTarget(mut tex: *mut gl_texture_t,
                                         mut pic: *mut rgbdata_t) {
    if pic.is_null() {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 b"../ref_gl/gl_image.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 615 as
                                                                     libc::c_int);
    }
    if tex.is_null() {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 b"../ref_gl/gl_image.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 616 as
                                                                     libc::c_int);
    }
    // correct depth size
    (*pic).depth =
        if 1 as libc::c_int > (*pic).depth as libc::c_int {
            1 as libc::c_int
        } else { (*pic).depth as libc::c_int } as word; // begin counting
    (*tex).numMips = 0 as libc::c_int as byte;
    // correct mip count
    (*pic).numMips =
        if 1 as libc::c_int > (*pic).numMips as libc::c_int {
            1 as libc::c_int
        } else { (*pic).numMips as libc::c_int } as byte;
    // trying to determine texture type
    if (*pic).width as libc::c_int > 1 as libc::c_int &&
           (*pic).height as libc::c_int <= 1 as libc::c_int {
        (*tex).target = 0xde0 as libc::c_int as GLuint
    } else if (*pic).flags & IMAGE_CUBEMAP as libc::c_int as libc::c_uint != 0
     {
        (*tex).target = 0x8513 as libc::c_int as GLuint
    } else if (*pic).flags & IMAGE_MULTILAYER as libc::c_int as libc::c_uint
                  != 0 && (*pic).depth as libc::c_int >= 1 as libc::c_int {
        (*tex).target = 0x8c1a as libc::c_int as GLuint
    } else if (*pic).width as libc::c_int > 1 as libc::c_int &&
                  (*pic).height as libc::c_int > 1 as libc::c_int &&
                  (*pic).depth as libc::c_int > 1 as libc::c_int {
        (*tex).target = 0x806f as libc::c_int as GLuint
    } else if (*tex).flags as libc::c_uint &
                  TF_RECTANGLE as libc::c_int as libc::c_uint != 0 {
        (*tex).target = 0x84f5 as libc::c_int as GLuint
    } else if (*tex).flags as libc::c_uint &
                  TF_MULTISAMPLE as libc::c_int as libc::c_uint != 0 {
        (*tex).target = 0x9100 as libc::c_int as GLuint
    } else { (*tex).target = 0xde1 as libc::c_int as GLuint } // default case
    // check for hardware support
    if (*tex).target == 0x8513 as libc::c_int as libc::c_uint &&
           GL_Support(GL_TEXTURE_CUBEMAP_EXT as libc::c_int) as u64 == 0 {
        (*tex).target = 0 as libc::c_int as GLuint
    } // fallback
    if (*tex).target == 0x84f5 as libc::c_int as libc::c_uint &&
           GL_Support(GL_TEXTURE_2D_RECT_EXT as libc::c_int) as u64 == 0 {
        (*tex).target = 0xde1 as libc::c_int as GLuint
    }
    if (*tex).target == 0x8c1a as libc::c_int as libc::c_uint &&
           GL_Support(GL_TEXTURE_ARRAY_EXT as libc::c_int) as u64 == 0 {
        (*tex).target = 0 as libc::c_int as GLuint
    }
    if (*tex).target == 0x806f as libc::c_int as libc::c_uint &&
           GL_Support(GL_TEXTURE_3D_EXT as libc::c_int) as u64 == 0 {
        (*tex).target = 0 as libc::c_int as GLuint
    }
    // check if depth textures are not supported
    if (*tex).flags as libc::c_uint &
           TF_DEPTHMAP as libc::c_int as libc::c_uint != 0 &&
           GL_Support(GL_DEPTH_TEXTURE as libc::c_int) as u64 == 0 {
        (*tex).target = 0 as libc::c_int as GLuint
    }
    // depth cubemaps only allowed when GL_EXT_gpu_shader4 is supported
    if (*tex).target == 0x8513 as libc::c_int as libc::c_uint &&
           GL_Support(GL_EXT_GPU_SHADER4 as libc::c_int) as u64 == 0 &&
           (*tex).flags as libc::c_uint &
               TF_DEPTHMAP as libc::c_int as libc::c_uint != 0 {
        (*tex).target = 0 as libc::c_int as GLuint
    }
    if (*tex).target == 0x9100 as libc::c_int as libc::c_uint &&
           GL_Support(GL_TEXTURE_MULTISAMPLE as libc::c_int) as u64 == 0 {
        (*tex).target = 0 as libc::c_int as GLuint
    };
}
/*
===============
GL_SetTextureFormat
===============
*/
unsafe extern "C" fn GL_SetTextureFormat(mut tex: *mut gl_texture_t,
                                         mut format: pixformat_t,
                                         mut channelMask: libc::c_int) {
    let mut haveColor: qboolean =
        (channelMask & IMAGE_HAS_COLOR as libc::c_int) as qboolean;
    let mut haveAlpha: qboolean =
        (channelMask & IMAGE_HAS_ALPHA as libc::c_int) as qboolean;
    if tex.is_null() {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 b"../ref_gl/gl_image.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 675 as
                                                                     libc::c_int);
    }
    if format as libc::c_uint == PF_DXT1 as libc::c_int as libc::c_uint ||
           format as libc::c_uint == PF_DXT3 as libc::c_int as libc::c_uint ||
           format as libc::c_uint == PF_DXT5 as libc::c_int as libc::c_uint ||
           format as libc::c_uint == PF_ATI2 as libc::c_int as libc::c_uint {
        match format as libc::c_uint {
            8 => { (*tex).format = 0x83f0 as libc::c_int }
            9 => { (*tex).format = 0x83f2 as libc::c_int }
            10 => { (*tex).format = 0x83f3 as libc::c_int }
            11 => {
                if glConfig.hardware_type as libc::c_uint ==
                       GLHW_RADEON as libc::c_int as libc::c_uint {
                    (*tex).format = 0x8837 as libc::c_int
                } else { (*tex).format = 0x8dbd as libc::c_int }
            }
            _ => { }
        }
        return
    } else {
        if (*tex).flags as libc::c_uint &
               TF_DEPTHMAP as libc::c_int as libc::c_uint != 0 {
            if (*tex).flags as libc::c_uint &
                   TF_ARB_16BIT as libc::c_int as libc::c_uint != 0 {
                (*tex).format = 0x81a5 as libc::c_int
            } else if (*tex).flags as libc::c_uint &
                          TF_ARB_FLOAT as libc::c_int as libc::c_uint != 0 &&
                          GL_Support(GL_ARB_DEPTH_FLOAT_EXT as libc::c_int) as
                              libc::c_uint != 0 {
                (*tex).format = 0x8cac as libc::c_int
            } else { (*tex).format = 0x81a6 as libc::c_int }
        } else if (*tex).flags as libc::c_uint &
                      (TF_ARB_FLOAT as libc::c_int |
                           TF_ARB_16BIT as libc::c_int) as libc::c_uint != 0
                      &&
                      GL_Support(GL_ARB_TEXTURE_FLOAT_EXT as libc::c_int) as
                          libc::c_uint != 0 {
            if haveColor as libc::c_uint != 0 &&
                   haveAlpha as libc::c_uint != 0 {
                if (*tex).flags as libc::c_uint &
                       TF_ARB_16BIT as libc::c_int as libc::c_uint != 0 ||
                       (*gpGlobals).desktopBitsPixel == 16 as libc::c_int {
                    (*tex).format = 0x881a as libc::c_int
                } else { (*tex).format = 0x8814 as libc::c_int }
            } else if haveColor as u64 != 0 {
                if (*tex).flags as libc::c_uint &
                       TF_ARB_16BIT as libc::c_int as libc::c_uint != 0 ||
                       (*gpGlobals).desktopBitsPixel == 16 as libc::c_int {
                    (*tex).format = 0x881b as libc::c_int
                } else { (*tex).format = 0x8815 as libc::c_int }
            } else if haveAlpha as u64 != 0 {
                if (*tex).flags as libc::c_uint &
                       TF_ARB_16BIT as libc::c_int as libc::c_uint != 0 ||
                       (*gpGlobals).desktopBitsPixel == 16 as libc::c_int {
                    (*tex).format = 0x822f as libc::c_int
                } else { (*tex).format = 0x8230 as libc::c_int }
            } else if (*tex).flags as libc::c_uint &
                          TF_ARB_16BIT as libc::c_int as libc::c_uint != 0 ||
                          (*gpGlobals).desktopBitsPixel == 16 as libc::c_int {
                (*tex).format = 0x881e as libc::c_int
            } else { (*tex).format = 0x8818 as libc::c_int }
        } else {
            // NOTE: not all the types will be compressed
            let mut bits: libc::c_int = (*gpGlobals).desktopBitsPixel;
            match GL_CalcTextureSamples(channelMask) {
                1 => {
                    if (*tex).flags as libc::c_uint &
                           TF_ALPHACONTRAST as libc::c_int as libc::c_uint !=
                           0 {
                        (*tex).format = 0x804b as libc::c_int
                    } else { (*tex).format = 0x8040 as libc::c_int }
                }
                2 => { (*tex).format = 0x8045 as libc::c_int }
                3 => {
                    match bits {
                        16 => { (*tex).format = 0x8050 as libc::c_int }
                        32 => { (*tex).format = 0x8051 as libc::c_int }
                        _ => { (*tex).format = 0x1907 as libc::c_int }
                    }
                }
                4 | _ => {
                    match bits {
                        16 => { (*tex).format = 0x8056 as libc::c_int }
                        32 => { (*tex).format = 0x8058 as libc::c_int }
                        _ => { (*tex).format = 0x1908 as libc::c_int }
                    }
                }
            }
        }
    };
}
/*
=================
GL_ResampleTexture

Assume input buffer is RGBA
=================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_ResampleTexture(mut source: *const byte,
                                            mut inWidth: libc::c_int,
                                            mut inHeight: libc::c_int,
                                            mut outWidth: libc::c_int,
                                            mut outHeight: libc::c_int,
                                            mut isNormalMap: qboolean)
 -> *mut byte {
    let mut frac: uint = 0; // pointer to a scaled image
    let mut fracStep: uint = 0;
    let mut in_0: *mut uint = source as *mut uint;
    let mut p1: [uint; 4096] = [0; 4096];
    let mut p2: [uint; 4096] = [0; 4096];
    let mut pix1: *mut byte = 0 as *mut byte;
    let mut pix2: *mut byte = 0 as *mut byte;
    let mut pix3: *mut byte = 0 as *mut byte;
    let mut pix4: *mut byte = 0 as *mut byte;
    let mut out: *mut uint = 0 as *mut uint;
    let mut inRow1: *mut uint = 0 as *mut uint;
    let mut inRow2: *mut uint = 0 as *mut uint;
    static mut scaledImage: *mut byte = 0 as *const byte as *mut byte;
    let mut normal: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    if source.is_null() { return 0 as *mut byte }
    scaledImage =
        gEngfuncs._Mem_Realloc.expect("non-null function pointer")(r_temppool,
                                                                   scaledImage
                                                                       as
                                                                       *mut libc::c_void,
                                                                   (outWidth *
                                                                        outHeight
                                                                        *
                                                                        4 as
                                                                            libc::c_int)
                                                                       as
                                                                       size_t,
                                                                   true_0,
                                                                   b"../ref_gl/gl_image.c\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char,
                                                                   781 as
                                                                       libc::c_int)
            as *mut byte;
    fracStep = (inWidth * 0x10000 as libc::c_int / outWidth) as uint;
    out = scaledImage as *mut uint;
    frac = fracStep >> 2 as libc::c_int;
    i = 0 as libc::c_int;
    while i < outWidth {
        p1[i as usize] =
            (4 as libc::c_int as
                 libc::c_uint).wrapping_mul(frac >> 16 as libc::c_int);
        frac = (frac as libc::c_uint).wrapping_add(fracStep) as uint as uint;
        i += 1
    }
    frac =
        (fracStep >>
             2 as libc::c_int).wrapping_mul(3 as libc::c_int as libc::c_uint);
    i = 0 as libc::c_int;
    while i < outWidth {
        p2[i as usize] =
            (4 as libc::c_int as
                 libc::c_uint).wrapping_mul(frac >> 16 as libc::c_int);
        frac = (frac as libc::c_uint).wrapping_add(fracStep) as uint as uint;
        i += 1
    }
    if isNormalMap as u64 != 0 {
        y = 0 as libc::c_int;
        while y < outHeight {
            inRow1 =
                in_0.offset((inWidth *
                                 ((y as libc::c_float + 0.25f32) *
                                      inHeight as libc::c_float /
                                      outHeight as libc::c_float) as
                                     libc::c_int) as isize);
            inRow2 =
                in_0.offset((inWidth *
                                 ((y as libc::c_float + 0.75f32) *
                                      inHeight as libc::c_float /
                                      outHeight as libc::c_float) as
                                     libc::c_int) as isize);
            x = 0 as libc::c_int;
            while x < outWidth {
                pix1 = (inRow1 as *mut byte).offset(p1[x as usize] as isize);
                pix2 = (inRow1 as *mut byte).offset(p2[x as usize] as isize);
                pix3 = (inRow2 as *mut byte).offset(p1[x as usize] as isize);
                pix4 = (inRow2 as *mut byte).offset(p2[x as usize] as isize);
                normal[0 as libc::c_int as usize] =
                    *pix1.offset(0 as libc::c_int as isize) as libc::c_int as
                        libc::c_float * (1.0f32 / 127.0f32) - 1.0f32 +
                        (*pix2.offset(0 as libc::c_int as isize) as
                             libc::c_int as libc::c_float *
                             (1.0f32 / 127.0f32) - 1.0f32) +
                        (*pix3.offset(0 as libc::c_int as isize) as
                             libc::c_int as libc::c_float *
                             (1.0f32 / 127.0f32) - 1.0f32) +
                        (*pix4.offset(0 as libc::c_int as isize) as
                             libc::c_int as libc::c_float *
                             (1.0f32 / 127.0f32) - 1.0f32);
                normal[1 as libc::c_int as usize] =
                    *pix1.offset(1 as libc::c_int as isize) as libc::c_int as
                        libc::c_float * (1.0f32 / 127.0f32) - 1.0f32 +
                        (*pix2.offset(1 as libc::c_int as isize) as
                             libc::c_int as libc::c_float *
                             (1.0f32 / 127.0f32) - 1.0f32) +
                        (*pix3.offset(1 as libc::c_int as isize) as
                             libc::c_int as libc::c_float *
                             (1.0f32 / 127.0f32) - 1.0f32) +
                        (*pix4.offset(1 as libc::c_int as isize) as
                             libc::c_int as libc::c_float *
                             (1.0f32 / 127.0f32) - 1.0f32);
                normal[2 as libc::c_int as usize] =
                    *pix1.offset(2 as libc::c_int as isize) as libc::c_int as
                        libc::c_float * (1.0f32 / 127.0f32) - 1.0f32 +
                        (*pix2.offset(2 as libc::c_int as isize) as
                             libc::c_int as libc::c_float *
                             (1.0f32 / 127.0f32) - 1.0f32) +
                        (*pix3.offset(2 as libc::c_int as isize) as
                             libc::c_int as libc::c_float *
                             (1.0f32 / 127.0f32) - 1.0f32) +
                        (*pix4.offset(2 as libc::c_int as isize) as
                             libc::c_int as libc::c_float *
                             (1.0f32 / 127.0f32) - 1.0f32);
                if VectorNormalizeLength2(normal.as_mut_ptr() as *const vec_t,
                                          normal.as_mut_ptr()) == 0. {
                    normal[0 as libc::c_int as usize] = 0.5f32;
                    normal[1 as libc::c_int as usize] = 0.5f32;
                    normal[2 as libc::c_int as usize] = 1.0f32
                }
                *(out.offset(x as isize) as
                      *mut byte).offset(0 as libc::c_int as isize) =
                    (128 as libc::c_int +
                         (127.0f32 * normal[0 as libc::c_int as usize]) as
                             byte as libc::c_int) as byte;
                *(out.offset(x as isize) as
                      *mut byte).offset(1 as libc::c_int as isize) =
                    (128 as libc::c_int +
                         (127.0f32 * normal[1 as libc::c_int as usize]) as
                             byte as libc::c_int) as byte;
                *(out.offset(x as isize) as
                      *mut byte).offset(2 as libc::c_int as isize) =
                    (128 as libc::c_int +
                         (127.0f32 * normal[2 as libc::c_int as usize]) as
                             byte as libc::c_int) as byte;
                *(out.offset(x as isize) as
                      *mut byte).offset(3 as libc::c_int as isize) =
                    255 as libc::c_int as byte;
                x += 1
            }
            y += 1;
            out = out.offset(outWidth as isize)
        }
    } else {
        y = 0 as libc::c_int;
        while y < outHeight {
            inRow1 =
                in_0.offset((inWidth *
                                 ((y as libc::c_float + 0.25f32) *
                                      inHeight as libc::c_float /
                                      outHeight as libc::c_float) as
                                     libc::c_int) as isize);
            inRow2 =
                in_0.offset((inWidth *
                                 ((y as libc::c_float + 0.75f32) *
                                      inHeight as libc::c_float /
                                      outHeight as libc::c_float) as
                                     libc::c_int) as isize);
            x = 0 as libc::c_int;
            while x < outWidth {
                pix1 = (inRow1 as *mut byte).offset(p1[x as usize] as isize);
                pix2 = (inRow1 as *mut byte).offset(p2[x as usize] as isize);
                pix3 = (inRow2 as *mut byte).offset(p1[x as usize] as isize);
                pix4 = (inRow2 as *mut byte).offset(p2[x as usize] as isize);
                *(out.offset(x as isize) as
                      *mut byte).offset(0 as libc::c_int as isize) =
                    (*pix1.offset(0 as libc::c_int as isize) as libc::c_int +
                         *pix2.offset(0 as libc::c_int as isize) as
                             libc::c_int +
                         *pix3.offset(0 as libc::c_int as isize) as
                             libc::c_int +
                         *pix4.offset(0 as libc::c_int as isize) as
                             libc::c_int >> 2 as libc::c_int) as byte;
                *(out.offset(x as isize) as
                      *mut byte).offset(1 as libc::c_int as isize) =
                    (*pix1.offset(1 as libc::c_int as isize) as libc::c_int +
                         *pix2.offset(1 as libc::c_int as isize) as
                             libc::c_int +
                         *pix3.offset(1 as libc::c_int as isize) as
                             libc::c_int +
                         *pix4.offset(1 as libc::c_int as isize) as
                             libc::c_int >> 2 as libc::c_int) as byte;
                *(out.offset(x as isize) as
                      *mut byte).offset(2 as libc::c_int as isize) =
                    (*pix1.offset(2 as libc::c_int as isize) as libc::c_int +
                         *pix2.offset(2 as libc::c_int as isize) as
                             libc::c_int +
                         *pix3.offset(2 as libc::c_int as isize) as
                             libc::c_int +
                         *pix4.offset(2 as libc::c_int as isize) as
                             libc::c_int >> 2 as libc::c_int) as byte;
                *(out.offset(x as isize) as
                      *mut byte).offset(3 as libc::c_int as isize) =
                    (*pix1.offset(3 as libc::c_int as isize) as libc::c_int +
                         *pix2.offset(3 as libc::c_int as isize) as
                             libc::c_int +
                         *pix3.offset(3 as libc::c_int as isize) as
                             libc::c_int +
                         *pix4.offset(3 as libc::c_int as isize) as
                             libc::c_int >> 2 as libc::c_int) as byte;
                x += 1
            }
            y += 1;
            out = out.offset(outWidth as isize)
        }
    }
    return scaledImage;
}
/*
=================
GL_BoxFilter3x3

box filter 3x3
=================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_BoxFilter3x3(mut out: *mut byte,
                                         mut in_0: *const byte,
                                         mut w: libc::c_int,
                                         mut h: libc::c_int,
                                         mut x: libc::c_int,
                                         mut y: libc::c_int) {
    let mut r: libc::c_int = 0 as libc::c_int;
    let mut g: libc::c_int = 0 as libc::c_int;
    let mut b: libc::c_int = 0 as libc::c_int;
    let mut a: libc::c_int = 0 as libc::c_int;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut acount: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut u: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut pixel: *const byte = 0 as *const byte;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        u = i - 1 as libc::c_int + x;
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            v = j - 1 as libc::c_int + y;
            if u >= 0 as libc::c_int && u < w && v >= 0 as libc::c_int &&
                   v < h {
                pixel =
                    &*in_0.offset(((u + v * w) * 4 as libc::c_int) as isize)
                        as *const byte;
                if *pixel.offset(3 as libc::c_int as isize) as libc::c_int !=
                       0 as libc::c_int {
                    r +=
                        *pixel.offset(0 as libc::c_int as isize) as
                            libc::c_int;
                    g +=
                        *pixel.offset(1 as libc::c_int as isize) as
                            libc::c_int;
                    b +=
                        *pixel.offset(2 as libc::c_int as isize) as
                            libc::c_int;
                    a +=
                        *pixel.offset(3 as libc::c_int as isize) as
                            libc::c_int;
                    acount += 1
                }
            }
            j += 1
        }
        i += 1
    }
    if acount == 0 as libc::c_int { acount = 1 as libc::c_int }
    *out.offset(0 as libc::c_int as isize) = (r / acount) as byte;
    *out.offset(1 as libc::c_int as isize) = (g / acount) as byte;
    *out.offset(2 as libc::c_int as isize) = (b / acount) as byte;
    //	out[3] = (int)( SimpleSpline( ( a / 12.0f ) / 255.0f ) * 255 );
}
/*
=================
GL_ApplyFilter

Apply box-filter to 1-bit alpha
=================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_ApplyFilter(mut source: *const byte,
                                        mut width: libc::c_int,
                                        mut height: libc::c_int)
 -> *mut byte {
    let mut in_0: *mut byte = source as *mut byte;
    let mut out: *mut byte = source as *mut byte;
    let mut i: libc::c_int = 0;
    if Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(PARM_QUAKE_COMPATIBLE
                                                                                                                 as
                                                                                                                 libc::c_int,
                                                                                                             0
                                                                                                                 as
                                                                                                                 libc::c_int)
           != 0 || glConfig.max_multisamples > 1 as libc::c_int {
        return in_0
    }
    i = 0 as libc::c_int;
    while !source.is_null() && i < width * height {
        if *in_0.offset(0 as libc::c_int as isize) as libc::c_int ==
               0 as libc::c_int &&
               *in_0.offset(1 as libc::c_int as isize) as libc::c_int ==
                   0 as libc::c_int &&
               *in_0.offset(2 as libc::c_int as isize) as libc::c_int ==
                   0 as libc::c_int &&
               *in_0.offset(3 as libc::c_int as isize) as libc::c_int ==
                   0 as libc::c_int {
            GL_BoxFilter3x3(in_0, source, width, height, i % width,
                            i / width);
        }
        i += 1;
        in_0 = in_0.offset(4 as libc::c_int as isize)
    }
    return out;
}
/*
=================
GL_BuildMipMap

Operates in place, quartering the size of the texture
=================
*/
unsafe extern "C" fn GL_BuildMipMap(mut in_0: *mut byte,
                                    mut srcWidth: libc::c_int,
                                    mut srcHeight: libc::c_int,
                                    mut srcDepth: libc::c_int,
                                    mut flags: libc::c_int) {
    let mut out: *mut byte = in_0;
    let mut instride: libc::c_int =
        (((srcWidth * 4 as libc::c_int) as
              libc::c_ulong).wrapping_add((1 as libc::c_int as
                                               size_t).wrapping_sub(1 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong))
             &
             !(1 as libc::c_int as
                   size_t).wrapping_sub(1 as libc::c_int as libc::c_ulong)) as
            libc::c_int;
    let mut mipWidth: libc::c_int = 0;
    let mut mipHeight: libc::c_int = 0;
    let mut outpadding: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    let mut normal: vec3_t = [0.; 3];
    if in_0.is_null() { return }
    mipWidth =
        if 1 as libc::c_int > srcWidth >> 1 as libc::c_int {
            1 as libc::c_int
        } else { (srcWidth) >> 1 as libc::c_int };
    mipHeight =
        if 1 as libc::c_int > srcHeight >> 1 as libc::c_int {
            1 as libc::c_int
        } else { (srcHeight) >> 1 as libc::c_int };
    outpadding =
        (((mipWidth * 4 as libc::c_int) as
              libc::c_ulong).wrapping_add((1 as libc::c_int as
                                               size_t).wrapping_sub(1 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong))
             &
             !(1 as libc::c_int as
                   size_t).wrapping_sub(1 as libc::c_int as
                                            libc::c_ulong)).wrapping_sub((mipWidth
                                                                              *
                                                                              4
                                                                                  as
                                                                                  libc::c_int)
                                                                             as
                                                                             libc::c_ulong)
            as libc::c_int;
    row = srcWidth << 2 as libc::c_int;
    if flags & TF_ALPHACONTRAST as libc::c_int != 0 {
        memset(in_0 as *mut libc::c_void, mipWidth,
               (mipWidth * mipHeight * 4 as libc::c_int) as libc::c_ulong);
        return
    }
    // move through all layers
    z = 0 as libc::c_int;
    while z < srcDepth {
        if flags & TF_NORMALMAP as libc::c_int != 0 {
            y = 0 as libc::c_int;
            while y < mipHeight {
                let mut next: *mut byte =
                    if ((y << 1 as libc::c_int) + 1 as libc::c_int) <
                           srcHeight {
                        in_0.offset(instride as isize)
                    } else { in_0 };
                x = 0 as libc::c_int;
                row = 0 as libc::c_int;
                while x < mipWidth {
                    if ((x << 1 as libc::c_int) + 1 as libc::c_int) < srcWidth
                       {
                        normal[0 as libc::c_int as usize] =
                            *in_0.offset((row + 0 as libc::c_int) as isize) as
                                libc::c_int as libc::c_float *
                                (1.0f32 / 127.0f32) - 1.0f32 +
                                (*in_0.offset((row + 4 as libc::c_int) as
                                                  isize) as libc::c_int as
                                     libc::c_float * (1.0f32 / 127.0f32) -
                                     1.0f32) +
                                (*next.offset((row + 0 as libc::c_int) as
                                                  isize) as libc::c_int as
                                     libc::c_float * (1.0f32 / 127.0f32) -
                                     1.0f32) +
                                (*next.offset((row + 4 as libc::c_int) as
                                                  isize) as libc::c_int as
                                     libc::c_float * (1.0f32 / 127.0f32) -
                                     1.0f32);
                        normal[1 as libc::c_int as usize] =
                            *in_0.offset((row + 1 as libc::c_int) as isize) as
                                libc::c_int as libc::c_float *
                                (1.0f32 / 127.0f32) - 1.0f32 +
                                (*in_0.offset((row + 5 as libc::c_int) as
                                                  isize) as libc::c_int as
                                     libc::c_float * (1.0f32 / 127.0f32) -
                                     1.0f32) +
                                (*next.offset((row + 1 as libc::c_int) as
                                                  isize) as libc::c_int as
                                     libc::c_float * (1.0f32 / 127.0f32) -
                                     1.0f32) +
                                (*next.offset((row + 5 as libc::c_int) as
                                                  isize) as libc::c_int as
                                     libc::c_float * (1.0f32 / 127.0f32) -
                                     1.0f32);
                        normal[2 as libc::c_int as usize] =
                            *in_0.offset((row + 2 as libc::c_int) as isize) as
                                libc::c_int as libc::c_float *
                                (1.0f32 / 127.0f32) - 1.0f32 +
                                (*in_0.offset((row + 6 as libc::c_int) as
                                                  isize) as libc::c_int as
                                     libc::c_float * (1.0f32 / 127.0f32) -
                                     1.0f32) +
                                (*next.offset((row + 2 as libc::c_int) as
                                                  isize) as libc::c_int as
                                     libc::c_float * (1.0f32 / 127.0f32) -
                                     1.0f32) +
                                (*next.offset((row + 6 as libc::c_int) as
                                                  isize) as libc::c_int as
                                     libc::c_float * (1.0f32 / 127.0f32) -
                                     1.0f32)
                    } else {
                        normal[0 as libc::c_int as usize] =
                            *in_0.offset((row + 0 as libc::c_int) as isize) as
                                libc::c_int as libc::c_float *
                                (1.0f32 / 127.0f32) - 1.0f32 +
                                (*next.offset((row + 0 as libc::c_int) as
                                                  isize) as libc::c_int as
                                     libc::c_float * (1.0f32 / 127.0f32) -
                                     1.0f32);
                        normal[1 as libc::c_int as usize] =
                            *in_0.offset((row + 1 as libc::c_int) as isize) as
                                libc::c_int as libc::c_float *
                                (1.0f32 / 127.0f32) - 1.0f32 +
                                (*next.offset((row + 1 as libc::c_int) as
                                                  isize) as libc::c_int as
                                     libc::c_float * (1.0f32 / 127.0f32) -
                                     1.0f32);
                        normal[2 as libc::c_int as usize] =
                            *in_0.offset((row + 2 as libc::c_int) as isize) as
                                libc::c_int as libc::c_float *
                                (1.0f32 / 127.0f32) - 1.0f32 +
                                (*next.offset((row + 2 as libc::c_int) as
                                                  isize) as libc::c_int as
                                     libc::c_float * (1.0f32 / 127.0f32) -
                                     1.0f32)
                    }
                    if VectorNormalizeLength2(normal.as_mut_ptr() as
                                                  *const vec_t,
                                              normal.as_mut_ptr()) == 0. {
                        normal[0 as libc::c_int as usize] = 0.5f32;
                        normal[1 as libc::c_int as usize] = 0.5f32;
                        normal[2 as libc::c_int as usize] = 1.0f32
                    }
                    *out.offset(0 as libc::c_int as isize) =
                        (128 as libc::c_int +
                             (127.0f32 * normal[0 as libc::c_int as usize]) as
                                 byte as libc::c_int) as byte;
                    *out.offset(1 as libc::c_int as isize) =
                        (128 as libc::c_int +
                             (127.0f32 * normal[1 as libc::c_int as usize]) as
                                 byte as libc::c_int) as byte;
                    *out.offset(2 as libc::c_int as isize) =
                        (128 as libc::c_int +
                             (127.0f32 * normal[2 as libc::c_int as usize]) as
                                 byte as libc::c_int) as byte;
                    *out.offset(3 as libc::c_int as isize) =
                        255 as libc::c_int as byte;
                    x += 1;
                    row += 8 as libc::c_int;
                    out = out.offset(4 as libc::c_int as isize)
                }
                y += 1;
                in_0 = in_0.offset((instride * 2 as libc::c_int) as isize);
                out = out.offset(outpadding as isize)
            }
        } else {
            y = 0 as libc::c_int;
            while y < mipHeight {
                let mut next_0: *mut byte =
                    if ((y << 1 as libc::c_int) + 1 as libc::c_int) <
                           srcHeight {
                        in_0.offset(instride as isize)
                    } else { in_0 };
                x = 0 as libc::c_int;
                row = 0 as libc::c_int;
                while x < mipWidth {
                    if ((x << 1 as libc::c_int) + 1 as libc::c_int) < srcWidth
                       {
                        *out.offset(0 as libc::c_int as isize) =
                            (*in_0.offset((row + 0 as libc::c_int) as isize)
                                 as libc::c_int +
                                 *in_0.offset((row + 4 as libc::c_int) as
                                                  isize) as libc::c_int +
                                 *next_0.offset((row + 0 as libc::c_int) as
                                                    isize) as libc::c_int +
                                 *next_0.offset((row + 4 as libc::c_int) as
                                                    isize) as libc::c_int >>
                                 2 as libc::c_int) as byte;
                        *out.offset(1 as libc::c_int as isize) =
                            (*in_0.offset((row + 1 as libc::c_int) as isize)
                                 as libc::c_int +
                                 *in_0.offset((row + 5 as libc::c_int) as
                                                  isize) as libc::c_int +
                                 *next_0.offset((row + 1 as libc::c_int) as
                                                    isize) as libc::c_int +
                                 *next_0.offset((row + 5 as libc::c_int) as
                                                    isize) as libc::c_int >>
                                 2 as libc::c_int) as byte;
                        *out.offset(2 as libc::c_int as isize) =
                            (*in_0.offset((row + 2 as libc::c_int) as isize)
                                 as libc::c_int +
                                 *in_0.offset((row + 6 as libc::c_int) as
                                                  isize) as libc::c_int +
                                 *next_0.offset((row + 2 as libc::c_int) as
                                                    isize) as libc::c_int +
                                 *next_0.offset((row + 6 as libc::c_int) as
                                                    isize) as libc::c_int >>
                                 2 as libc::c_int) as byte;
                        *out.offset(3 as libc::c_int as isize) =
                            (*in_0.offset((row + 3 as libc::c_int) as isize)
                                 as libc::c_int +
                                 *in_0.offset((row + 7 as libc::c_int) as
                                                  isize) as libc::c_int +
                                 *next_0.offset((row + 3 as libc::c_int) as
                                                    isize) as libc::c_int +
                                 *next_0.offset((row + 7 as libc::c_int) as
                                                    isize) as libc::c_int >>
                                 2 as libc::c_int) as byte
                    } else {
                        *out.offset(0 as libc::c_int as isize) =
                            (*in_0.offset((row + 0 as libc::c_int) as isize)
                                 as libc::c_int +
                                 *next_0.offset((row + 0 as libc::c_int) as
                                                    isize) as libc::c_int >>
                                 1 as libc::c_int) as byte;
                        *out.offset(1 as libc::c_int as isize) =
                            (*in_0.offset((row + 1 as libc::c_int) as isize)
                                 as libc::c_int +
                                 *next_0.offset((row + 1 as libc::c_int) as
                                                    isize) as libc::c_int >>
                                 1 as libc::c_int) as byte;
                        *out.offset(2 as libc::c_int as isize) =
                            (*in_0.offset((row + 2 as libc::c_int) as isize)
                                 as libc::c_int +
                                 *next_0.offset((row + 2 as libc::c_int) as
                                                    isize) as libc::c_int >>
                                 1 as libc::c_int) as byte;
                        *out.offset(3 as libc::c_int as isize) =
                            (*in_0.offset((row + 3 as libc::c_int) as isize)
                                 as libc::c_int +
                                 *next_0.offset((row + 3 as libc::c_int) as
                                                    isize) as libc::c_int >>
                                 1 as libc::c_int) as byte
                    }
                    x += 1;
                    row += 8 as libc::c_int;
                    out = out.offset(4 as libc::c_int as isize)
                }
                y += 1;
                in_0 = in_0.offset((instride * 2 as libc::c_int) as isize);
                out = out.offset(outpadding as isize)
            }
        }
        z += 1
    };
}
unsafe extern "C" fn GL_TextureImageRAW(mut tex: *mut gl_texture_t,
                                        mut side: GLint, mut level: GLint,
                                        mut width: GLint, mut height: GLint,
                                        mut depth: GLint, mut type_0: GLint,
                                        mut data: *const libc::c_void) {
    let mut cubeTarget: GLuint = 0x8515 as libc::c_int as GLuint;
    let mut subImage: qboolean =
        ((*tex).flags as libc::c_uint &
             TF_IMG_UPLOADED as libc::c_int as libc::c_uint) as qboolean;
    let mut inFormat: GLenum =
        (*gEngfuncs.Image_GetPFDesc.expect("non-null function pointer")(type_0)).glFormat;
    let mut dataType: GLint = 0x1401 as libc::c_int;
    let mut samplesCount: GLsizei = 0 as libc::c_int;
    if tex.is_null() {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 b"../ref_gl/gl_image.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 1023 as
                                                                     libc::c_int);
    }
    if (*tex).flags as libc::c_uint &
           TF_DEPTHMAP as libc::c_int as libc::c_uint != 0 {
        inFormat = 0x1902 as libc::c_int as GLenum
    }
    if (*tex).flags as libc::c_uint &
           TF_ARB_16BIT as libc::c_int as libc::c_uint != 0 {
        dataType = 0x140b as libc::c_int
    } else if (*tex).flags as libc::c_uint &
                  TF_ARB_FLOAT as libc::c_int as libc::c_uint != 0 {
        dataType = 0x1406 as libc::c_int
    }
    if (*tex).target == 0xde0 as libc::c_int as libc::c_uint {
        if subImage as u64 != 0 {
            pglTexSubImage1D.expect("non-null function pointer")((*tex).target,
                                                                 level,
                                                                 0 as
                                                                     libc::c_int,
                                                                 width,
                                                                 inFormat,
                                                                 dataType as
                                                                     GLenum,
                                                                 data);
        } else {
            pglTexImage1D.expect("non-null function pointer")((*tex).target,
                                                              level,
                                                              (*tex).format,
                                                              width,
                                                              0 as
                                                                  libc::c_int,
                                                              inFormat,
                                                              dataType as
                                                                  GLenum,
                                                              data);
        }
    } else if (*tex).target == 0x8513 as libc::c_int as libc::c_uint {
        if subImage as u64 != 0 {
            pglTexSubImage2D.expect("non-null function pointer")(cubeTarget.wrapping_add(side
                                                                                             as
                                                                                             libc::c_uint),
                                                                 level,
                                                                 0 as
                                                                     libc::c_int,
                                                                 0 as
                                                                     libc::c_int,
                                                                 width,
                                                                 height,
                                                                 inFormat,
                                                                 dataType as
                                                                     GLenum,
                                                                 data);
        } else {
            pglTexImage2D.expect("non-null function pointer")(cubeTarget.wrapping_add(side
                                                                                          as
                                                                                          libc::c_uint),
                                                              level,
                                                              (*tex).format,
                                                              width, height,
                                                              0 as
                                                                  libc::c_int,
                                                              inFormat,
                                                              dataType as
                                                                  GLenum,
                                                              data);
        }
    } else if (*tex).target == 0x806f as libc::c_int as libc::c_uint ||
                  (*tex).target == 0x8c1a as libc::c_int as libc::c_uint {
        if subImage as u64 != 0 {
            pglTexSubImage3D.expect("non-null function pointer")((*tex).target,
                                                                 level,
                                                                 0 as
                                                                     libc::c_int,
                                                                 0 as
                                                                     libc::c_int,
                                                                 0 as
                                                                     libc::c_int,
                                                                 width,
                                                                 height,
                                                                 depth,
                                                                 inFormat,
                                                                 dataType as
                                                                     GLenum,
                                                                 data);
        } else {
            pglTexImage3D.expect("non-null function pointer")((*tex).target,
                                                              level,
                                                              (*tex).format as
                                                                  GLenum,
                                                              width, height,
                                                              depth,
                                                              0 as
                                                                  libc::c_int,
                                                              inFormat,
                                                              dataType as
                                                                  GLenum,
                                                              data);
        }
    } else if (*tex).target == 0x9100 as libc::c_int as libc::c_uint {
        samplesCount =
            gEngfuncs.pfnGetCvarFloat.expect("non-null function pointer")(b"gl_msaa_samples\x00"
                                                                              as
                                                                              *const u8
                                                                              as
                                                                              *const libc::c_char)
                as GLsizei;
        match samplesCount {
            2 | 4 | 8 | 16 => { }
            _ => { samplesCount = 1 as libc::c_int }
        }
        pglTexImage2DMultisample.expect("non-null function pointer")((*tex).target,
                                                                     samplesCount,
                                                                     (*tex).format
                                                                         as
                                                                         GLenum,
                                                                     width,
                                                                     height,
                                                                     0x1 as
                                                                         libc::c_int
                                                                         as
                                                                         GLboolean);
    } else if subImage as u64 != 0 {
        pglTexSubImage2D.expect("non-null function pointer")((*tex).target,
                                                             level,
                                                             0 as libc::c_int,
                                                             0 as libc::c_int,
                                                             width, height,
                                                             inFormat,
                                                             dataType as
                                                                 GLenum,
                                                             data);
    } else {
        pglTexImage2D.expect("non-null function pointer")((*tex).target,
                                                          level,
                                                          (*tex).format,
                                                          width, height,
                                                          0 as libc::c_int,
                                                          inFormat,
                                                          dataType as GLenum,
                                                          data);
    };
}
unsafe extern "C" fn GL_TextureImageDXT(mut tex: *mut gl_texture_t,
                                        mut side: GLint, mut level: GLint,
                                        mut width: GLint, mut height: GLint,
                                        mut depth: GLint, mut size: size_t,
                                        mut data: *const libc::c_void) {
    let mut cubeTarget: GLuint = 0x8515 as libc::c_int as GLuint;
    let mut subImage: qboolean =
        ((*tex).flags as libc::c_uint &
             TF_IMG_UPLOADED as libc::c_int as libc::c_uint) as qboolean;
    if tex.is_null() {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 b"../ref_gl/gl_image.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 1075 as
                                                                     libc::c_int);
    }
    if (*tex).target == 0xde0 as libc::c_int as libc::c_uint {
        if subImage as u64 != 0 {
            pglCompressedTexSubImage1DARB.expect("non-null function pointer")((*tex).target,
                                                                              level,
                                                                              0
                                                                                  as
                                                                                  libc::c_int,
                                                                              width,
                                                                              (*tex).format
                                                                                  as
                                                                                  GLenum,
                                                                              size
                                                                                  as
                                                                                  GLsizei,
                                                                              data);
        } else {
            pglCompressedTexImage1DARB.expect("non-null function pointer")((*tex).target,
                                                                           level,
                                                                           (*tex).format
                                                                               as
                                                                               GLenum,
                                                                           width,
                                                                           0
                                                                               as
                                                                               libc::c_int,
                                                                           size
                                                                               as
                                                                               GLsizei,
                                                                           data);
        }
    } else if (*tex).target == 0x8513 as libc::c_int as libc::c_uint {
        if subImage as u64 != 0 {
            pglCompressedTexSubImage2DARB.expect("non-null function pointer")(cubeTarget.wrapping_add(side
                                                                                                          as
                                                                                                          libc::c_uint),
                                                                              level,
                                                                              0
                                                                                  as
                                                                                  libc::c_int,
                                                                              0
                                                                                  as
                                                                                  libc::c_int,
                                                                              width,
                                                                              height,
                                                                              (*tex).format
                                                                                  as
                                                                                  GLenum,
                                                                              size
                                                                                  as
                                                                                  GLsizei,
                                                                              data);
        } else {
            pglCompressedTexImage2DARB.expect("non-null function pointer")(cubeTarget.wrapping_add(side
                                                                                                       as
                                                                                                       libc::c_uint),
                                                                           level,
                                                                           (*tex).format
                                                                               as
                                                                               GLenum,
                                                                           width,
                                                                           height,
                                                                           0
                                                                               as
                                                                               libc::c_int,
                                                                           size
                                                                               as
                                                                               GLsizei,
                                                                           data);
        }
    } else if (*tex).target == 0x806f as libc::c_int as libc::c_uint ||
                  (*tex).target == 0x8c1a as libc::c_int as libc::c_uint {
        if subImage as u64 != 0 {
            pglCompressedTexSubImage3DARB.expect("non-null function pointer")((*tex).target,
                                                                              level,
                                                                              0
                                                                                  as
                                                                                  libc::c_int,
                                                                              0
                                                                                  as
                                                                                  libc::c_int,
                                                                              0
                                                                                  as
                                                                                  libc::c_int,
                                                                              width,
                                                                              height,
                                                                              depth,
                                                                              (*tex).format
                                                                                  as
                                                                                  GLenum,
                                                                              size
                                                                                  as
                                                                                  GLsizei,
                                                                              data);
        } else {
            pglCompressedTexImage3DARB.expect("non-null function pointer")((*tex).target,
                                                                           level,
                                                                           (*tex).format
                                                                               as
                                                                               GLenum,
                                                                           width,
                                                                           height,
                                                                           depth,
                                                                           0
                                                                               as
                                                                               libc::c_int,
                                                                           size
                                                                               as
                                                                               GLsizei,
                                                                           data);
        }
    } else if subImage as u64 != 0 {
        pglCompressedTexSubImage2DARB.expect("non-null function pointer")((*tex).target,
                                                                          level,
                                                                          0 as
                                                                              libc::c_int,
                                                                          0 as
                                                                              libc::c_int,
                                                                          width,
                                                                          height,
                                                                          (*tex).format
                                                                              as
                                                                              GLenum,
                                                                          size
                                                                              as
                                                                              GLsizei,
                                                                          data);
    } else {
        pglCompressedTexImage2DARB.expect("non-null function pointer")((*tex).target,
                                                                       level,
                                                                       (*tex).format
                                                                           as
                                                                           GLenum,
                                                                       width,
                                                                       height,
                                                                       0 as
                                                                           libc::c_int,
                                                                       size as
                                                                           GLsizei,
                                                                       data);
    };
}
// 2D or RECT
// 2D or RECT
/*
===============
GL_CheckTexImageError

show GL-errors on load images
===============
*/
unsafe extern "C" fn GL_CheckTexImageError(mut tex: *mut gl_texture_t) {
    let mut err: libc::c_int = 0;
    if tex.is_null() {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 b"../ref_gl/gl_image.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 1112 as
                                                                     libc::c_int);
    }
    // catch possible errors
    if (if !gl_check_errors.is_null() && (*gl_check_errors).value != 0.0f32 {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int }) != 0 &&
           {
               err =
                   pglGetError.expect("non-null function pointer")() as
                       libc::c_int;
               (err) != 0 as libc::c_int
           } {
        gEngfuncs.Con_Printf.expect("non-null function pointer")(b"^3OpenGL Error:^7 %s while uploading %s [%s]\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 GL_ErrorString(err),
                                                                 (*tex).name.as_mut_ptr(),
                                                                 GL_TargetToString((*tex).target));
    };
}
/*
===============
GL_UploadTexture

upload texture into video memory
===============
*/
unsafe extern "C" fn GL_UploadTexture(mut tex: *mut gl_texture_t,
                                      mut pic: *mut rgbdata_t) -> qboolean {
    let mut buf: *mut byte = 0 as *mut byte;
    let mut data: *mut byte = 0 as *mut byte;
    let mut texsize: size_t = 0;
    let mut size: size_t = 0;
    let mut width: uint = 0;
    let mut height: uint = 0;
    let mut i: uint = 0;
    let mut j: uint = 0;
    let mut numSides: uint = 0;
    let mut offset: uint = 0 as libc::c_int as uint;
    let mut normalMap: qboolean = false_0;
    let mut bufend: *const byte = 0 as *const byte;
    // dedicated server
    if glw_state.initialized as u64 == 0 { return true_0 } // must be first
    if pic.is_null() {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 b"../ref_gl/gl_image.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 1140 as
                                                                     libc::c_int);
    }
    if tex.is_null() {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 b"../ref_gl/gl_image.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 1141 as
                                                                     libc::c_int);
    }
    GL_SetTextureTarget(tex, pic);
    // make sure what target is correct
    if (*tex).target == 0 as libc::c_int as libc::c_uint {
        gEngfuncs.Con_DPrintf.expect("non-null function pointer")(b"^1Error:^7 GL_UploadTexture: %s is not supported by your hardware\n\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char,
                                                                  (*tex).name.as_mut_ptr());
        return false_0
    }
    GL_SetTextureDimensions(tex, (*pic).width as libc::c_int,
                            (*pic).height as libc::c_int,
                            (*pic).depth as libc::c_int);
    GL_SetTextureFormat(tex, (*pic).type_0 as pixformat_t,
                        (*pic).flags as libc::c_int);
    (*tex).fogParams[0 as libc::c_int as usize] =
        (*pic).fogParams[0 as libc::c_int as usize];
    (*tex).fogParams[1 as libc::c_int as usize] =
        (*pic).fogParams[1 as libc::c_int as usize];
    (*tex).fogParams[2 as libc::c_int as usize] =
        (*pic).fogParams[2 as libc::c_int as usize];
    (*tex).fogParams[3 as libc::c_int as usize] =
        (*pic).fogParams[3 as libc::c_int as usize];
    if (*pic).width as libc::c_int * (*pic).height as libc::c_int &
           3 as libc::c_int != 0 {
        // will be resampled, just tell me for debug targets
        gEngfuncs.Con_Reportf.expect("non-null function pointer")(b"GL_UploadTexture: %s s&3 [%d x %d]\n\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char,
                                                                  (*tex).name.as_mut_ptr(),
                                                                  (*pic).width
                                                                      as
                                                                      libc::c_int,
                                                                  (*pic).height
                                                                      as
                                                                      libc::c_int); // total image size include all the layers, cube sides, mipmaps
    }
    buf = (*pic).buffer;
    bufend = (*pic).buffer.offset((*pic).size as isize);
    offset =
        GL_CalcImageSize((*pic).type_0 as pixformat_t,
                         (*pic).width as libc::c_int,
                         (*pic).height as libc::c_int,
                         (*pic).depth as libc::c_int) as uint;
    texsize =
        GL_CalcTextureSize((*tex).format as GLenum,
                           (*tex).width as libc::c_int,
                           (*tex).height as libc::c_int,
                           (*tex).depth as libc::c_int);
    normalMap =
        if (*tex).flags as libc::c_uint &
               TF_NORMALMAP as libc::c_int as libc::c_uint != 0 {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int } as qboolean;
    numSides =
        if (*pic).flags & IMAGE_CUBEMAP as libc::c_int as libc::c_uint != 0 {
            6 as libc::c_int
        } else { 1 as libc::c_int } as uint;
    // uploading texture into video memory, change the binding
    glState.currentTextures[glState.activeTMU as usize] =
        (*tex).texnum as GLint;
    pglBindTexture.expect("non-null function pointer")((*tex).target,
                                                       (*tex).texnum);
    i = 0 as libc::c_int as uint;
    while i < numSides {
        // track the buffer bounds
        if !buf.is_null() && buf >= bufend as *mut byte {
            gEngfuncs.Host_Error.expect("non-null function pointer")(b"GL_UploadTexture: %s image buffer overflow\n\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     (*tex).name.as_mut_ptr()); // move pointer
        }
        if (*pic).type_0 == PF_DXT1 as libc::c_int as libc::c_uint ||
               (*pic).type_0 == PF_DXT3 as libc::c_int as libc::c_uint ||
               (*pic).type_0 == PF_DXT5 as libc::c_int as libc::c_uint ||
               (*pic).type_0 == PF_ATI2 as libc::c_int as libc::c_uint {
            j = 0 as libc::c_int as uint;
            while j <
                      (if 1 as libc::c_int > (*pic).numMips as libc::c_int {
                           1 as libc::c_int
                       } else { (*pic).numMips as libc::c_int }) as
                          libc::c_uint {
                width =
                    if 1 as libc::c_int > (*tex).width as libc::c_int >> j {
                        1 as libc::c_int
                    } else { ((*tex).width as libc::c_int) >> j } as uint;
                height =
                    if 1 as libc::c_int > (*tex).height as libc::c_int >> j {
                        1 as libc::c_int
                    } else { ((*tex).height as libc::c_int) >> j } as uint;
                texsize =
                    GL_CalcTextureSize((*tex).format as GLenum,
                                       width as libc::c_int,
                                       height as libc::c_int,
                                       (*tex).depth as libc::c_int);
                size =
                    GL_CalcImageSize((*pic).type_0 as pixformat_t,
                                     width as libc::c_int,
                                     height as libc::c_int,
                                     (*tex).depth as libc::c_int);
                GL_TextureImageDXT(tex, i as GLint, j as GLint,
                                   width as GLint, height as GLint,
                                   (*tex).depth as GLint, size,
                                   buf as *const libc::c_void);
                (*tex).size =
                    ((*tex).size as libc::c_ulong).wrapping_add(texsize) as
                        size_t as size_t;
                buf = buf.offset(size as isize);
                (*tex).numMips = (*tex).numMips.wrapping_add(1);
                GL_CheckTexImageError(tex);
                j = j.wrapping_add(1)
            }
        } else if (if 1 as libc::c_int > (*pic).numMips as libc::c_int {
                       1 as libc::c_int
                   } else { (*pic).numMips as libc::c_int }) >
                      1 as libc::c_int {
            // not-compressed DDS
            j = 0 as libc::c_int as uint; // move pointer
            while j <
                      (if 1 as libc::c_int > (*pic).numMips as libc::c_int {
                           1 as libc::c_int
                       } else { (*pic).numMips as libc::c_int }) as
                          libc::c_uint {
                width =
                    if 1 as libc::c_int > (*tex).width as libc::c_int >> j {
                        1 as libc::c_int
                    } else { ((*tex).width as libc::c_int) >> j } as uint;
                height =
                    if 1 as libc::c_int > (*tex).height as libc::c_int >> j {
                        1 as libc::c_int
                    } else { ((*tex).height as libc::c_int) >> j } as uint;
                texsize =
                    GL_CalcTextureSize((*tex).format as GLenum,
                                       width as libc::c_int,
                                       height as libc::c_int,
                                       (*tex).depth as libc::c_int);
                size =
                    GL_CalcImageSize((*pic).type_0 as pixformat_t,
                                     width as libc::c_int,
                                     height as libc::c_int,
                                     (*tex).depth as libc::c_int);
                GL_TextureImageRAW(tex, i as GLint, j as GLint,
                                   width as GLint, height as GLint,
                                   (*tex).depth as GLint,
                                   (*pic).type_0 as GLint,
                                   buf as *const libc::c_void);
                (*tex).size =
                    ((*tex).size as libc::c_ulong).wrapping_add(texsize) as
                        size_t as size_t;
                buf = buf.offset(size as isize);
                (*tex).numMips = (*tex).numMips.wrapping_add(1);
                GL_CheckTexImageError(tex);
                j = j.wrapping_add(1)
            }
        } else {
            // RGBA32
            let mut mipCount: libc::c_int =
                GL_CalcMipmapCount(tex,
                                   (buf !=
                                        0 as *mut libc::c_void as *mut byte)
                                       as libc::c_int as qboolean);
            // NOTE: only single uncompressed textures can be resamples, no mips, no layers, no sides
            if (*tex).depth as libc::c_int == 1 as libc::c_int &&
                   ((*pic).width as libc::c_int != (*tex).width as libc::c_int
                        ||
                        (*pic).height as libc::c_int !=
                            (*tex).height as libc::c_int) {
                data =
                    GL_ResampleTexture(buf, (*pic).width as libc::c_int,
                                       (*pic).height as libc::c_int,
                                       (*tex).width as libc::c_int,
                                       (*tex).height as libc::c_int,
                                       normalMap)
            } else { data = buf }
            if !((*pic).type_0 == PF_DXT1 as libc::c_int as libc::c_uint ||
                     (*pic).type_0 == PF_DXT3 as libc::c_int as libc::c_uint
                     ||
                     (*pic).type_0 == PF_DXT5 as libc::c_int as libc::c_uint
                     ||
                     (*pic).type_0 == PF_ATI2 as libc::c_int as libc::c_uint)
                   &&
                   (*tex).flags as libc::c_uint &
                       TF_NOMIPMAP as libc::c_int as libc::c_uint == 0 &&
                   (*pic).flags &
                       IMAGE_ONEBIT_ALPHA as libc::c_int as libc::c_uint != 0
               {
                data =
                    GL_ApplyFilter(data, (*tex).width as libc::c_int,
                                   (*tex).height as libc::c_int)
            }
            // mips will be auto-generated if desired
            j = 0 as libc::c_int as uint;
            while j < mipCount as libc::c_uint {
                width =
                    if 1 as libc::c_int > (*tex).width as libc::c_int >> j {
                        1 as libc::c_int
                    } else { ((*tex).width as libc::c_int) >> j } as uint;
                height =
                    if 1 as libc::c_int > (*tex).height as libc::c_int >> j {
                        1 as libc::c_int
                    } else { ((*tex).height as libc::c_int) >> j } as uint;
                texsize =
                    GL_CalcTextureSize((*tex).format as GLenum,
                                       width as libc::c_int,
                                       height as libc::c_int,
                                       (*tex).depth as libc::c_int);
                size =
                    GL_CalcImageSize((*pic).type_0 as pixformat_t,
                                     width as libc::c_int,
                                     height as libc::c_int,
                                     (*tex).depth as libc::c_int);
                GL_TextureImageRAW(tex, i as GLint, j as GLint,
                                   width as GLint, height as GLint,
                                   (*tex).depth as GLint,
                                   (*pic).type_0 as GLint,
                                   data as *const libc::c_void);
                if mipCount > 1 as libc::c_int {
                    GL_BuildMipMap(data, width as libc::c_int,
                                   height as libc::c_int,
                                   (*tex).depth as libc::c_int,
                                   (*tex).flags as libc::c_int);
                }
                (*tex).size =
                    ((*tex).size as libc::c_ulong).wrapping_add(texsize) as
                        size_t as size_t;
                (*tex).numMips = (*tex).numMips.wrapping_add(1);
                GL_CheckTexImageError(tex);
                j = j.wrapping_add(1)
            }
            // move to next side
            if numSides > 1 as libc::c_int as libc::c_uint && !buf.is_null() {
                buf =
                    buf.offset(GL_CalcImageSize((*pic).type_0 as pixformat_t,
                                                (*pic).width as libc::c_int,
                                                (*pic).height as libc::c_int,
                                                1 as libc::c_int) as isize)
            }
        } // done
        i = i.wrapping_add(1)
    }
    (*tex).flags =
        ((*tex).flags as libc::c_uint |
             TF_IMG_UPLOADED as libc::c_int as libc::c_uint) as texFlags_t;
    (*tex).numMips =
        ((*tex).numMips as libc::c_uint).wrapping_div(numSides) as byte as
            byte;
    return true_0;
}
/*
===============
GL_ProcessImage

do specified actions on pixels
===============
*/
unsafe extern "C" fn GL_ProcessImage(mut tex: *mut gl_texture_t,
                                     mut pic: *mut rgbdata_t) {
    let mut emboss_scale: libc::c_float = 0.0f32;
    let mut img_flags: uint = 0 as libc::c_int as uint;
    // force upload texture as RGB or RGBA (detail textures requires this)
    if (*tex).flags as libc::c_uint &
           TF_FORCE_COLOR as libc::c_int as libc::c_uint != 0 {
        (*pic).flags |= IMAGE_HAS_COLOR as libc::c_int as libc::c_uint
    } // share encode method
    if (*pic).flags & IMAGE_HAS_ALPHA as libc::c_int as libc::c_uint != 0 {
        (*tex).flags =
            ::std::mem::transmute::<libc::c_uint,
                                    texFlags_t>((*tex).flags as libc::c_uint |
                                                    TF_HAS_ALPHA as
                                                        libc::c_int as
                                                        libc::c_uint)
    } // disable mipmapping by user request
    (*tex).encode = (*pic).encode as GLint;
    if (*pic).type_0 == PF_DXT1 as libc::c_int as libc::c_uint ||
           (*pic).type_0 == PF_DXT3 as libc::c_int as libc::c_uint ||
           (*pic).type_0 == PF_DXT5 as libc::c_int as libc::c_uint ||
           (*pic).type_0 == PF_ATI2 as libc::c_int as libc::c_uint {
        if (*pic).numMips == 0 {
            (*tex).flags =
                ::std::mem::transmute::<libc::c_uint,
                                        texFlags_t>((*tex).flags as
                                                        libc::c_uint |
                                                        TF_NOMIPMAP as
                                                            libc::c_int as
                                                            libc::c_uint)
        }
        // clear all the unsupported flags
        (*tex).flags =
            ::std::mem::transmute::<libc::c_uint,
                                    texFlags_t>((*tex).flags as libc::c_uint &
                                                    !(TF_KEEP_SOURCE as
                                                          libc::c_int) as
                                                        libc::c_uint)
    } else {
        // copy flag about luma pixels
        if (*pic).flags & IMAGE_HAS_LUMA as libc::c_int as libc::c_uint != 0 {
            (*tex).flags =
                ::std::mem::transmute::<libc::c_uint,
                                        texFlags_t>((*tex).flags as
                                                        libc::c_uint |
                                                        TF_HAS_LUMA as
                                                            libc::c_int as
                                                            libc::c_uint)
        }
        if (*pic).flags & IMAGE_QUAKEPAL as libc::c_int as libc::c_uint != 0 {
            (*tex).flags =
                ::std::mem::transmute::<libc::c_uint,
                                        texFlags_t>((*tex).flags as
                                                        libc::c_uint |
                                                        TF_QUAKEPAL as
                                                            libc::c_int as
                                                            libc::c_uint)
        }
        // create luma texture from quake texture
        if (*tex).flags as libc::c_uint &
               TF_MAKELUMA as libc::c_int as libc::c_uint != 0 {
            img_flags |=
                IMAGE_MAKE_LUMA as libc::c_int as
                    libc::c_uint; // because current pic will be expanded to rgba
            (*tex).flags =
                ::std::mem::transmute::<libc::c_uint,
                                        texFlags_t>((*tex).flags as
                                                        libc::c_uint &
                                                        !(TF_MAKELUMA as
                                                              libc::c_int) as
                                                            libc::c_uint)
        }
        if (*tex).flags as libc::c_uint &
               TF_ALLOW_EMBOSS as libc::c_int as libc::c_uint != 0 {
            img_flags |= IMAGE_EMBOSS as libc::c_int as libc::c_uint;
            (*tex).flags =
                ::std::mem::transmute::<libc::c_uint,
                                        texFlags_t>((*tex).flags as
                                                        libc::c_uint &
                                                        !(TF_ALLOW_EMBOSS as
                                                              libc::c_int) as
                                                            libc::c_uint)
        }
        if (*tex).flags as libc::c_uint &
               TF_IMG_UPLOADED as libc::c_int as libc::c_uint == 0 &&
               (*tex).flags as libc::c_uint &
                   TF_KEEP_SOURCE as libc::c_int as libc::c_uint != 0 {
            (*tex).original =
                gEngfuncs.FS_CopyImage.expect("non-null function pointer")(pic)
        }
        // we need to expand image into RGBA buffer
        if (*pic).type_0 == PF_INDEXED_24 as libc::c_int as libc::c_uint ||
               (*pic).type_0 == PF_INDEXED_32 as libc::c_int as libc::c_uint {
            img_flags |= IMAGE_FORCE_RGBA as libc::c_int as libc::c_uint
        }
        // dedicated server doesn't register this variable
        if !gl_emboss_scale.is_null() {
            emboss_scale = (*gl_emboss_scale).value
        }
        // processing image before uploading (force to rgba, make luma etc)
        if !(*pic).buffer.is_null() {
            gEngfuncs.Image_Process.expect("non-null function pointer")(&mut pic,
                                                                        0 as
                                                                            libc::c_int,
                                                                        0 as
                                                                            libc::c_int,
                                                                        img_flags,
                                                                        emboss_scale);
        }
        if (*tex).flags as libc::c_uint &
               TF_LUMINANCE as libc::c_int as libc::c_uint != 0 {
            (*pic).flags =
                (*pic).flags &
                    !(IMAGE_HAS_COLOR as libc::c_int) as libc::c_uint
        }
    };
}
/*
================
GL_CheckTexName
================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_CheckTexName(mut name: *const libc::c_char)
 -> qboolean {
    let mut len: libc::c_int = 0;
    if if name.is_null() || *name == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return false_0
    }
    len = Q_strlen(name) as libc::c_int;
    // because multi-layered textures can exceed name string
    if len as libc::c_ulong >=
           ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong {
        gEngfuncs.Con_Printf.expect("non-null function pointer")(b"^1Error:^7 LoadTexture: too long name %s (%d)\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 name, len);
        return false_0
    }
    return true_0;
}
/*
================
GL_TextureForName
================
*/
unsafe extern "C" fn GL_TextureForName(mut name: *const libc::c_char)
 -> *mut gl_texture_t {
    let mut tex: *mut gl_texture_t = 0 as *mut gl_texture_t;
    let mut hash: uint = 0;
    // find the texture in array
    hash =
        COM_HashKey(name, (4096 as libc::c_int >> 2 as libc::c_int) as uint);
    tex = gl_texturesHashTable[hash as usize];
    while !tex.is_null() {
        if Q_strnicmp((*tex).name.as_mut_ptr(), name, 99999 as libc::c_int) ==
               0 {
            return tex
        }
        tex = (*tex).nextHash
    }
    return 0 as *mut gl_texture_t;
}
/*
================
GL_AllocTexture
================
*/
unsafe extern "C" fn GL_AllocTexture(mut name: *const libc::c_char,
                                     mut flags: texFlags_t)
 -> *mut gl_texture_t {
    let mut tex: *mut gl_texture_t = 0 as *mut gl_texture_t;
    let mut i: uint = 0;
    // find a free texture_t slot
    i = 0 as libc::c_int as uint;
    tex = gl_textures.as_mut_ptr();
    while i < gl_numTextures {
        if (*tex).name[0 as libc::c_int as usize] == 0 { break ; }
        i = i.wrapping_add(1);
        tex = tex.offset(1)
    }
    if i == gl_numTextures {
        if gl_numTextures == 4096 as libc::c_int as libc::c_uint {
            gEngfuncs.Host_Error.expect("non-null function pointer")(b"GL_AllocTexture: MAX_TEXTURES limit exceeds\n\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char);
        }
        gl_numTextures = gl_numTextures.wrapping_add(1)
    }
    tex =
        &mut *gl_textures.as_mut_ptr().offset(i as isize) as
            *mut gl_texture_t;
    // copy initial params
    Q_strncpy((*tex).name.as_mut_ptr(), name,
              ::std::mem::size_of::<[libc::c_char; 256]>() as
                  libc::c_ulong); // texnum is used for fast acess into gl_textures array too
    if flags as libc::c_uint & TF_SKYSIDE as libc::c_int as libc::c_uint != 0
       {
        let fresh0 = tr.skyboxbasenum;
        tr.skyboxbasenum = tr.skyboxbasenum + 1;
        (*tex).texnum = fresh0 as GLuint
    } else { (*tex).texnum = i }
    (*tex).flags = flags;
    // add to hash table
    (*tex).hashValue =
        COM_HashKey(name, (4096 as libc::c_int >> 2 as libc::c_int) as uint);
    (*tex).nextHash = gl_texturesHashTable[(*tex).hashValue as usize];
    gl_texturesHashTable[(*tex).hashValue as usize] = tex;
    return tex;
}
/*
================
GL_DeleteTexture
================
*/
unsafe extern "C" fn GL_DeleteTexture(mut tex: *mut gl_texture_t) {
    let mut prev: *mut *mut gl_texture_t = 0 as *mut *mut gl_texture_t;
    let mut cur: *mut gl_texture_t = 0 as *mut gl_texture_t;
    if tex.is_null() {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 b"../ref_gl/gl_image.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 1417 as
                                                                     libc::c_int);
    }
    // already freed?
    if (*tex).texnum == 0 { return }
    // debug
    if (*tex).name[0 as libc::c_int as usize] == 0 {
        gEngfuncs.Con_Printf.expect("non-null function pointer")(b"^1Error:^7 GL_DeleteTexture: trying to free unnamed texture with texnum %i\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 (*tex).texnum);
        return
    }
    // remove from hash table
    prev =
        &mut *gl_texturesHashTable.as_mut_ptr().offset((*tex).hashValue as
                                                           isize) as
            *mut *mut gl_texture_t;
    loop  {
        cur = *prev;
        if cur.is_null() { break ; }
        if cur == tex {
            *prev = (*cur).nextHash;
            break ;
        } else { prev = &mut (*cur).nextHash }
    }
    // release source
    if !(*tex).original.is_null() {
        gEngfuncs.FS_FreeImage.expect("non-null function pointer")((*tex).original);
    }
    if glw_state.initialized as u64 != 0 {
        pglDeleteTextures.expect("non-null function pointer")(1 as
                                                                  libc::c_int,
                                                              &mut (*tex).texnum);
    }
    memset(tex as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<gl_texture_t>() as libc::c_ulong);
}
/*
================
GL_UpdateTexSize

recalc image room
================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_UpdateTexSize(mut texnum: libc::c_int,
                                          mut width: libc::c_int,
                                          mut height: libc::c_int,
                                          mut depth: libc::c_int) {
    let mut i: libc::c_int = 0; // recompute now
    let mut j: libc::c_int = 0;
    let mut texsize: libc::c_int = 0;
    let mut numSides: libc::c_int = 0;
    let mut tex: *mut gl_texture_t = 0 as *mut gl_texture_t;
    if texnum <= 0 as libc::c_int || texnum >= 4096 as libc::c_int { return }
    tex =
        &mut *gl_textures.as_mut_ptr().offset(texnum as isize) as
            *mut gl_texture_t;
    numSides =
        if (*tex).flags as libc::c_uint &
               TF_CUBEMAP as libc::c_int as libc::c_uint != 0 {
            6 as libc::c_int
        } else { 1 as libc::c_int };
    GL_SetTextureDimensions(tex, width, height, depth);
    (*tex).size = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int;
    while i < numSides {
        j = 0 as libc::c_int;
        while j <
                  (if 1 as libc::c_int > (*tex).numMips as libc::c_int {
                       1 as libc::c_int
                   } else { (*tex).numMips as libc::c_int }) {
            width =
                if 1 as libc::c_int > (*tex).width as libc::c_int >> j {
                    1 as libc::c_int
                } else { ((*tex).width as libc::c_int) >> j };
            height =
                if 1 as libc::c_int > (*tex).height as libc::c_int >> j {
                    1 as libc::c_int
                } else { ((*tex).height as libc::c_int) >> j };
            texsize =
                GL_CalcTextureSize((*tex).format as GLenum, width, height,
                                   (*tex).depth as libc::c_int) as
                    libc::c_int;
            (*tex).size =
                ((*tex).size as
                     libc::c_ulong).wrapping_add(texsize as libc::c_ulong) as
                    size_t as size_t;
            j += 1
        }
        i += 1
    };
}
/*
================
GL_LoadTexture
================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_LoadTexture(mut name: *const libc::c_char,
                                        mut buf: *const byte,
                                        mut size: size_t,
                                        mut flags: libc::c_int)
 -> libc::c_int {
    let mut tex: *mut gl_texture_t = 0 as *mut gl_texture_t;
    let mut pic: *mut rgbdata_t = 0 as *mut rgbdata_t;
    let mut picFlags: uint = 0 as libc::c_int as uint;
    if GL_CheckTexName(name) as u64 == 0 { return 0 as libc::c_int }
    // see if already loaded
    tex = GL_TextureForName(name);
    if !tex.is_null() {
        return tex.wrapping_offset_from(gl_textures.as_mut_ptr()) as
                   libc::c_long as libc::c_int
    }
    if flags & TF_NOFLIP_TGA as libc::c_int != 0 {
        picFlags = picFlags | IL_DONTFLIP_TGA as libc::c_int as libc::c_uint
    }
    if flags & TF_KEEP_SOURCE as libc::c_int != 0 &&
           flags & TF_EXPAND_SOURCE as libc::c_int == 0 {
        picFlags = picFlags | IL_KEEP_8BIT as libc::c_int as libc::c_uint
    }
    // set some image flags
    gEngfuncs.Image_SetForceFlags.expect("non-null function pointer")(picFlags); // couldn't loading image
    pic =
        gEngfuncs.FS_LoadImage.expect("non-null function pointer")(name, buf,
                                                                   size);
    if pic.is_null() { return 0 as libc::c_int }
    // allocate the new one
    tex =
        GL_AllocTexture(name, flags as texFlags_t); // release source texture
    GL_ProcessImage(tex, pic); // update texture filter, wrap etc
    if GL_UploadTexture(tex, pic) as u64 == 0 {
        memset(tex as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<gl_texture_t>() as
                   libc::c_ulong); // release source texture
        gEngfuncs.FS_FreeImage.expect("non-null function pointer")(pic);
        return 0 as libc::c_int
    }
    GL_ApplyTextureParams(tex);
    gEngfuncs.FS_FreeImage.expect("non-null function pointer")(pic);
    // NOTE: always return texnum as index in array or engine will stop work !!!
    return tex.wrapping_offset_from(gl_textures.as_mut_ptr()) as libc::c_long
               as libc::c_int;
}
/*
================
GL_LoadTextureArray
================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_LoadTextureArray(mut names:
                                                 *mut *const libc::c_char,
                                             mut flags: libc::c_int)
 -> libc::c_int {
    let mut pic: *mut rgbdata_t = 0 as *mut rgbdata_t;
    let mut src: *mut rgbdata_t = 0 as *mut rgbdata_t;
    let mut basename: [libc::c_char; 256] = [0; 256];
    let mut numLayers: uint = 0 as libc::c_int as uint;
    let mut picFlags: uint = 0 as libc::c_int as uint;
    let mut name: [libc::c_char; 256] = [0; 256];
    let mut tex: *mut gl_texture_t = 0 as *mut gl_texture_t;
    let mut i: uint = 0;
    let mut j: uint = 0;
    if names.is_null() || (*names.offset(0 as libc::c_int as isize)).is_null()
           || glw_state.initialized as u64 == 0 {
        return 0 as libc::c_int
    }
    // count layers (g-cont. this is pontentially unsafe loop)
    i = 0 as libc::c_int as uint;
    while i < glConfig.max_2d_texture_layers as libc::c_uint &&
              **names.offset(i as isize) as libc::c_int != '\u{0}' as i32 {
        numLayers = numLayers.wrapping_add(1);
        i = i.wrapping_add(1)
    }
    name[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    if numLayers <= 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    }
    // create complexname from layer names
    i = 0 as libc::c_int as uint;
    while i < numLayers {
        COM_FileBase(*names.offset(i as isize), basename.as_mut_ptr());
        Q_strncat(name.as_mut_ptr(), basename.as_mut_ptr(),
                  ::std::mem::size_of::<[libc::c_char; 256]>() as
                      libc::c_ulong);
        if i != numLayers.wrapping_sub(1 as libc::c_int as libc::c_uint) {
            Q_strncat(name.as_mut_ptr(),
                      b"|\x00" as *const u8 as *const libc::c_char,
                      ::std::mem::size_of::<[libc::c_char; 256]>() as
                          libc::c_ulong);
        }
        i = i.wrapping_add(1)
    }
    Q_strncat(name.as_mut_ptr(),
              va(b"[%i]\x00" as *const u8 as *const libc::c_char, numLayers),
              ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong);
    if GL_CheckTexName(name.as_mut_ptr()) as u64 == 0 {
        return 0 as libc::c_int
    }
    // see if already loaded
    tex = GL_TextureForName(name.as_mut_ptr());
    if !tex.is_null() {
        return tex.wrapping_offset_from(gl_textures.as_mut_ptr()) as
                   libc::c_long as libc::c_int
    }
    // load all the images and pack it into single image
    i = 0 as libc::c_int as uint; // coldn't find layer
    pic = 0 as *mut rgbdata_t;
    while i < numLayers {
        let mut srcsize: size_t = 0;
        let mut dstsize: size_t = 0;
        let mut mipsize: size_t = 0;
        src =
            gEngfuncs.FS_LoadImage.expect("non-null function pointer")(*names.offset(i
                                                                                         as
                                                                                         isize),
                                                                       0 as
                                                                           *const byte,
                                                                       0 as
                                                                           libc::c_int
                                                                           as
                                                                           size_t);
        if src.is_null() { break ; }
        if !pic.is_null() {
            // mixed mode: DXT + RGB
            if (*pic).type_0 != (*src).type_0 {
                gEngfuncs.Con_Printf.expect("non-null function pointer")(b"^1Error:^7 GL_LoadTextureArray: mismatch image format for %s and %s\n\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char,
                                                                         *names.offset(0
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           isize),
                                                                         *names.offset(i
                                                                                           as
                                                                                           isize));
                break ;
            } else if (*pic).numMips as libc::c_int !=
                          (*src).numMips as libc::c_int {
                gEngfuncs.Con_Printf.expect("non-null function pointer")(b"^1Error:^7 GL_LoadTextureArray: mismatch mip count for %s and %s\n\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char,
                                                                         *names.offset(0
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           isize),
                                                                         *names.offset(i
                                                                                           as
                                                                                           isize));
                break ;
            } else if (*pic).encode as libc::c_int !=
                          (*src).encode as libc::c_int {
                gEngfuncs.Con_Printf.expect("non-null function pointer")(b"^1Error:^7 GL_LoadTextureArray: mismatch custom encoding for %s and %s\n\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char,
                                                                         *names.offset(0
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           isize),
                                                                         *names.offset(i
                                                                                           as
                                                                                           isize));
                break ;
            } else {
                // different mipcount
                // but allow to rescale raw images
                if ((*pic).type_0 == PF_RGBA_32 as libc::c_int as libc::c_uint
                        ||
                        (*pic).type_0 ==
                            PF_BGRA_32 as libc::c_int as libc::c_uint ||
                        (*pic).type_0 ==
                            PF_RGB_24 as libc::c_int as libc::c_uint ||
                        (*pic).type_0 ==
                            PF_BGR_24 as libc::c_int as libc::c_uint ||
                        (*pic).type_0 ==
                            PF_LUMINANCE as libc::c_int as libc::c_uint) &&
                       ((*src).type_0 ==
                            PF_RGBA_32 as libc::c_int as libc::c_uint ||
                            (*src).type_0 ==
                                PF_BGRA_32 as libc::c_int as libc::c_uint ||
                            (*src).type_0 ==
                                PF_RGB_24 as libc::c_int as libc::c_uint ||
                            (*src).type_0 ==
                                PF_BGR_24 as libc::c_int as libc::c_uint ||
                            (*src).type_0 ==
                                PF_LUMINANCE as libc::c_int as libc::c_uint)
                       &&
                       ((*pic).width as libc::c_int !=
                            (*src).width as libc::c_int ||
                            (*pic).height as libc::c_int !=
                                (*src).height as libc::c_int) {
                    gEngfuncs.Image_Process.expect("non-null function pointer")(&mut src,
                                                                                (*pic).width
                                                                                    as
                                                                                    libc::c_int,
                                                                                (*pic).height
                                                                                    as
                                                                                    libc::c_int,
                                                                                IMAGE_RESAMPLE
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    uint,
                                                                                0.0f32);
                }
                if (*pic).size != (*src).size {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"^1Error:^7 GL_LoadTextureArray: mismatch image size for %s and %s\n\x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char,
                                                                             *names.offset(0
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               isize),
                                                                             *names.offset(i
                                                                                               as
                                                                                               isize));
                    break ;
                }
            }
        } else {
            // create new image
            pic =
                gEngfuncs._Mem_Alloc.expect("non-null function pointer")(gEngfuncs.Image_GetPool.expect("non-null function pointer")(),
                                                                         ::std::mem::size_of::<rgbdata_t>()
                                                                             as
                                                                             libc::c_ulong,
                                                                         false_0,
                                                                         b"../ref_gl/gl_image.c\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char,
                                                                         1620
                                                                             as
                                                                             libc::c_int)
                    as *mut rgbdata_t;
            memcpy(pic as *mut libc::c_void, src as *const libc::c_void,
                   ::std::mem::size_of::<rgbdata_t>() as libc::c_ulong);
            // expand pic buffer for all layers
            (*pic).buffer =
                gEngfuncs._Mem_Alloc.expect("non-null function pointer")(gEngfuncs.Image_GetPool.expect("non-null function pointer")(),
                                                                         (*pic).size.wrapping_mul(numLayers
                                                                                                      as
                                                                                                      libc::c_ulong),
                                                                         false_0,
                                                                         b"../ref_gl/gl_image.c\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char,
                                                                         1624
                                                                             as
                                                                             libc::c_int)
                    as *mut byte;
            (*pic).depth = 0 as libc::c_int as word
        }
        dstsize = 0 as libc::c_int as size_t;
        srcsize = dstsize;
        mipsize = srcsize;
        j = 0 as libc::c_int as uint;
        while j <
                  (if 1 as libc::c_int > (*pic).numMips as libc::c_int {
                       1 as libc::c_int
                   } else { (*pic).numMips as libc::c_int }) as libc::c_uint {
            let mut width: libc::c_int =
                if 1 as libc::c_int > (*pic).width as libc::c_int >> j {
                    1 as libc::c_int
                } else { ((*pic).width as libc::c_int) >> j };
            let mut height: libc::c_int =
                if 1 as libc::c_int > (*pic).height as libc::c_int >> j {
                    1 as libc::c_int
                } else { ((*pic).height as libc::c_int) >> j };
            mipsize =
                GL_CalcImageSize((*pic).type_0 as pixformat_t, width, height,
                                 1 as libc::c_int);
            memcpy((*pic).buffer.offset(dstsize as
                                            isize).offset(mipsize.wrapping_mul(i
                                                                                   as
                                                                                   libc::c_ulong)
                                                              as isize) as
                       *mut libc::c_void,
                   (*src).buffer.offset(srcsize as isize) as
                       *const libc::c_void, mipsize);
            dstsize =
                (dstsize as
                     libc::c_ulong).wrapping_add(mipsize.wrapping_mul(numLayers
                                                                          as
                                                                          libc::c_ulong))
                    as size_t as size_t;
            srcsize =
                (srcsize as libc::c_ulong).wrapping_add(mipsize) as size_t as
                    size_t;
            j = j.wrapping_add(1)
        }
        gEngfuncs.FS_FreeImage.expect("non-null function pointer")(src);
        // increase layers
        (*pic).depth = (*pic).depth.wrapping_add(1);
        i = i.wrapping_add(1)
    }
    // there were errors
    if pic.is_null() || (*pic).depth as libc::c_uint != numLayers {
        gEngfuncs.Con_Printf.expect("non-null function pointer")(b"^1Error:^7 GL_LoadTextureArray: not all layers were loaded. Texture array is not created\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char);
        if !pic.is_null() {
            gEngfuncs.FS_FreeImage.expect("non-null function pointer")(pic);
        }
        return 0 as libc::c_int
    }
    // it's multilayer image!
    (*pic).flags =
        (*pic).flags | IMAGE_MULTILAYER as libc::c_int as libc::c_uint;
    (*pic).size =
        ((*pic).size as
             libc::c_ulong).wrapping_mul(numLayers as libc::c_ulong) as size_t
            as size_t;
    // allocate the new one
    tex =
        GL_AllocTexture(name.as_mut_ptr(),
                        flags as texFlags_t); // release source texture
    GL_ProcessImage(tex, pic); // update texture filter, wrap etc
    if GL_UploadTexture(tex, pic) as u64 == 0 {
        memset(tex as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<gl_texture_t>() as
                   libc::c_ulong); // release source texture
        gEngfuncs.FS_FreeImage.expect("non-null function pointer")(pic);
        return 0 as libc::c_int
    }
    GL_ApplyTextureParams(tex);
    gEngfuncs.FS_FreeImage.expect("non-null function pointer")(pic);
    // NOTE: always return texnum as index in array or engine will stop work !!!
    return tex.wrapping_offset_from(gl_textures.as_mut_ptr()) as libc::c_long
               as libc::c_int;
}
/*
================
GL_LoadTextureFromBuffer
================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_LoadTextureFromBuffer(mut name:
                                                      *const libc::c_char,
                                                  mut pic: *mut rgbdata_t,
                                                  mut flags: texFlags_t,
                                                  mut update: qboolean)
 -> libc::c_int {
    let mut tex: *mut gl_texture_t = 0 as *mut gl_texture_t;
    if GL_CheckTexName(name) as u64 == 0 { return 0 as libc::c_int }
    // see if already loaded
    tex = GL_TextureForName(name);
    if !tex.is_null() && update as u64 == 0 {
        return tex.wrapping_offset_from(gl_textures.as_mut_ptr()) as
                   libc::c_long as libc::c_int
    }
    // couldn't loading image
    if pic.is_null() { return 0 as libc::c_int }
    if update as u64 != 0 {
        if tex.is_null() {
            gEngfuncs.Host_Error.expect("non-null function pointer")(b"GL_LoadTextureFromBuffer: couldn\'t find texture %s for update\n\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     name);
        }
        (*tex).flags =
            ((*tex).flags as libc::c_uint | flags as libc::c_uint) as
                texFlags_t
    } else {
        // allocate the new one
        tex = GL_AllocTexture(name, flags)
    } // update texture filter, wrap etc
    GL_ProcessImage(tex, pic);
    if GL_UploadTexture(tex, pic) as u64 == 0 {
        memset(tex as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<gl_texture_t>() as libc::c_ulong);
        return 0 as libc::c_int
    }
    GL_ApplyTextureParams(tex);
    return tex.wrapping_offset_from(gl_textures.as_mut_ptr()) as libc::c_long
               as libc::c_int;
}
/*
================
GL_CreateTexture

creates texture from buffer
================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_CreateTexture(mut name: *const libc::c_char,
                                          mut width: libc::c_int,
                                          mut height: libc::c_int,
                                          mut buffer: *const libc::c_void,
                                          mut flags: texFlags_t)
 -> libc::c_int {
    let mut update: qboolean =
        if flags as libc::c_uint & TF_UPDATE as libc::c_int as libc::c_uint !=
               0 {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int } as qboolean;
    let mut datasize: libc::c_int = 1 as libc::c_int;
    let mut r_empty: rgbdata_t =
        rgbdata_t{width: 0,
                  height: 0,
                  depth: 0,
                  type_0: 0,
                  flags: 0,
                  encode: 0,
                  numMips: 0,
                  palette: 0 as *mut byte,
                  buffer: 0 as *mut byte,
                  fogParams: [0; 4],
                  size: 0,};
    if flags as libc::c_uint & TF_ARB_16BIT as libc::c_int as libc::c_uint !=
           0 {
        datasize = 2 as libc::c_int
    } else if flags as libc::c_uint &
                  TF_ARB_FLOAT as libc::c_int as libc::c_uint != 0 {
        datasize = 4 as libc::c_int
    }
    flags =
        (flags as libc::c_uint & !(TF_UPDATE as libc::c_int) as libc::c_uint)
            as texFlags_t;
    memset(&mut r_empty as *mut rgbdata_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<rgbdata_t>() as libc::c_ulong);
    r_empty.width = width as word;
    r_empty.height = height as word;
    r_empty.type_0 = PF_RGBA_32 as libc::c_int as uint;
    r_empty.size =
        (r_empty.width as libc::c_int * r_empty.height as libc::c_int *
             datasize * 4 as libc::c_int) as size_t;
    r_empty.buffer = buffer as *mut byte;
    // clear invalid combinations
    flags =
        (flags as libc::c_uint &
             !(TF_TEXTURE_3D as libc::c_int) as libc::c_uint) as texFlags_t;
    // if image not luminance and not alphacontrast it will have color
    if flags as libc::c_uint & TF_LUMINANCE as libc::c_int as libc::c_uint ==
           0 &&
           flags as libc::c_uint &
               TF_ALPHACONTRAST as libc::c_int as libc::c_uint == 0 {
        r_empty.flags =
            r_empty.flags | IMAGE_HAS_COLOR as libc::c_int as libc::c_uint
    }
    if flags as libc::c_uint & TF_HAS_ALPHA as libc::c_int as libc::c_uint !=
           0 {
        r_empty.flags =
            r_empty.flags | IMAGE_HAS_ALPHA as libc::c_int as libc::c_uint
    }
    if flags as libc::c_uint & TF_CUBEMAP as libc::c_int as libc::c_uint != 0
       {
        if GL_Support(GL_TEXTURE_CUBEMAP_EXT as libc::c_int) as u64 == 0 {
            return 0 as libc::c_int
        }
        r_empty.flags =
            r_empty.flags | IMAGE_CUBEMAP as libc::c_int as libc::c_uint;
        r_empty.size =
            (r_empty.size as
                 libc::c_ulong).wrapping_mul(6 as libc::c_int as
                                                 libc::c_ulong) as size_t as
                size_t
    }
    return GL_LoadTextureFromBuffer(name, &mut r_empty, flags, update);
}
/*
================
GL_CreateTextureArray

creates texture array from buffer
================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_CreateTextureArray(mut name: *const libc::c_char,
                                               mut width: libc::c_int,
                                               mut height: libc::c_int,
                                               mut depth: libc::c_int,
                                               mut buffer:
                                                   *const libc::c_void,
                                               mut flags: texFlags_t)
 -> libc::c_int {
    let mut r_empty: rgbdata_t =
        rgbdata_t{width: 0,
                  height: 0,
                  depth: 0,
                  type_0: 0,
                  flags: 0,
                  encode: 0,
                  numMips: 0,
                  palette: 0 as *mut byte,
                  buffer: 0 as *mut byte,
                  fogParams: [0; 4],
                  size: 0,};
    memset(&mut r_empty as *mut rgbdata_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<rgbdata_t>() as libc::c_ulong);
    r_empty.width =
        if width > 1 as libc::c_int { width } else { 1 as libc::c_int } as
            word;
    r_empty.height =
        if height > 1 as libc::c_int { height } else { 1 as libc::c_int } as
            word;
    r_empty.depth =
        if depth > 1 as libc::c_int { depth } else { 1 as libc::c_int } as
            word;
    r_empty.type_0 = PF_RGBA_32 as libc::c_int as uint;
    r_empty.size =
        (r_empty.width as libc::c_int * r_empty.height as libc::c_int *
             r_empty.depth as libc::c_int * 4 as libc::c_int) as size_t;
    r_empty.buffer = buffer as *mut byte;
    // clear invalid combinations
    flags =
        (flags as libc::c_uint &
             !(TF_CUBEMAP as libc::c_int | TF_SKYSIDE as libc::c_int |
                   TF_HAS_LUMA as libc::c_int | TF_MAKELUMA as libc::c_int |
                   TF_ALPHACONTRAST as libc::c_int) as libc::c_uint) as
            texFlags_t;
    // if image not luminance it will have color
    if flags as libc::c_uint & TF_LUMINANCE as libc::c_int as libc::c_uint ==
           0 {
        r_empty.flags =
            r_empty.flags | IMAGE_HAS_COLOR as libc::c_int as libc::c_uint
    }
    if flags as libc::c_uint & TF_HAS_ALPHA as libc::c_int as libc::c_uint !=
           0 {
        r_empty.flags =
            r_empty.flags | IMAGE_HAS_ALPHA as libc::c_int as libc::c_uint
    }
    if flags as libc::c_uint & TF_TEXTURE_3D as libc::c_int as libc::c_uint !=
           0 {
        if GL_Support(GL_TEXTURE_3D_EXT as libc::c_int) as u64 == 0 {
            return 0 as libc::c_int
        }
    } else {
        if GL_Support(GL_TEXTURE_ARRAY_EXT as libc::c_int) as u64 == 0 {
            return 0 as libc::c_int
        }
        r_empty.flags =
            r_empty.flags | IMAGE_MULTILAYER as libc::c_int as libc::c_uint
    }
    return GL_LoadTextureFromBuffer(name, &mut r_empty, flags, false_0);
}
/*
================
GL_FindTexture
================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_FindTexture(mut name: *const libc::c_char)
 -> libc::c_int {
    let mut tex: *mut gl_texture_t = 0 as *mut gl_texture_t;
    if GL_CheckTexName(name) as u64 == 0 { return 0 as libc::c_int }
    // see if already loaded
    tex = GL_TextureForName(name);
    if !tex.is_null() {
        return tex.wrapping_offset_from(gl_textures.as_mut_ptr()) as
                   libc::c_long as libc::c_int
    }
    return 0 as libc::c_int;
}
/*
================
GL_FreeTexture
================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_FreeTexture(mut texnum: GLenum) {
    // number 0 it's already freed
    if texnum <= 0 as libc::c_int as libc::c_uint { return }
    GL_DeleteTexture(&mut *gl_textures.as_mut_ptr().offset(texnum as isize));
}
/*
================
GL_ProcessTexture
================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_ProcessTexture(mut texnum: libc::c_int,
                                           mut gamma: libc::c_float,
                                           mut topColor: libc::c_int,
                                           mut bottomColor: libc::c_int) {
    let mut image: *mut gl_texture_t = 0 as *mut gl_texture_t; // missed image
    let mut pic: *mut rgbdata_t = 0 as *mut rgbdata_t;
    let mut flags: libc::c_int = 0 as libc::c_int;
    if texnum <= 0 as libc::c_int || texnum >= 4096 as libc::c_int { return }
    image =
        &mut *gl_textures.as_mut_ptr().offset(texnum as isize) as
            *mut gl_texture_t;
    // select mode
    if gamma != -1.0f32 {
        flags = IMAGE_LIGHTGAMMA as libc::c_int
    } else if topColor != -(1 as libc::c_int) &&
                  bottomColor != -(1 as libc::c_int) {
        flags = IMAGE_REMAP as libc::c_int
    } else {
        gEngfuncs.Con_Printf.expect("non-null function pointer")(b"^1Error:^7 GL_ProcessTexture: bad operation for %s\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 (*image).name.as_mut_ptr());
        return
    }
    if (*image).original.is_null() {
        gEngfuncs.Con_Printf.expect("non-null function pointer")(b"^1Error:^7 GL_ProcessTexture: no input data for %s\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 (*image).name.as_mut_ptr());
        return
    }
    if (*(*image).original).type_0 == PF_DXT1 as libc::c_int as libc::c_uint
           ||
           (*(*image).original).type_0 ==
               PF_DXT3 as libc::c_int as libc::c_uint ||
           (*(*image).original).type_0 ==
               PF_DXT5 as libc::c_int as libc::c_uint ||
           (*(*image).original).type_0 ==
               PF_ATI2 as libc::c_int as libc::c_uint {
        gEngfuncs.Con_Printf.expect("non-null function pointer")(b"^1Error:^7 GL_ProcessTexture: can\'t process compressed texture %s\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 (*image).name.as_mut_ptr());
        return
    }
    // all the operations makes over the image copy not an original
    pic =
        gEngfuncs.FS_CopyImage.expect("non-null function pointer")((*image).original); // update texture filter, wrap etc
    gEngfuncs.Image_Process.expect("non-null function pointer")(&mut pic,
                                                                topColor,
                                                                bottomColor,
                                                                flags as uint,
                                                                0.0f32);
    GL_UploadTexture(image, pic);
    GL_ApplyTextureParams(image);
    gEngfuncs.FS_FreeImage.expect("non-null function pointer")(pic);
}
/*
================
GL_TexMemory

return size of all uploaded textures
================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_TexMemory() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut total: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while (i as libc::c_uint) < gl_numTextures {
        total =
            (total as
                 libc::c_ulong).wrapping_add(gl_textures[i as usize].size) as
                libc::c_int as libc::c_int;
        i += 1
    }
    return total;
}
/*
==============================================================================

INTERNAL TEXTURES

==============================================================================
*/
/*
==================
GL_FakeImage
==================
*/
unsafe extern "C" fn GL_FakeImage(mut width: libc::c_int,
                                  mut height: libc::c_int,
                                  mut depth: libc::c_int,
                                  mut flags: libc::c_int) -> *mut rgbdata_t {
    static mut data2D: [byte; 1024] = [0; 1024]; // 16x16x4
    static mut r_image: rgbdata_t =
        rgbdata_t{width: 0,
                  height: 0,
                  depth: 0,
                  type_0: 0,
                  flags: 0,
                  encode: 0,
                  numMips: 0,
                  palette: 0 as *mut byte,
                  buffer: 0 as *mut byte,
                  fogParams: [0; 4],
                  size: 0,};
    // also use this for bad textures, but without alpha
    r_image.width =
        if 1 as libc::c_int > width { 1 as libc::c_int } else { width } as
            word;
    r_image.height =
        if 1 as libc::c_int > height { 1 as libc::c_int } else { height } as
            word;
    r_image.depth =
        if 1 as libc::c_int > depth { 1 as libc::c_int } else { depth } as
            word;
    r_image.flags = flags as uint;
    r_image.type_0 = PF_RGBA_32 as libc::c_int as uint;
    r_image.size =
        (r_image.width as libc::c_int * r_image.height as libc::c_int *
             r_image.depth as libc::c_int * 4 as libc::c_int) as size_t;
    r_image.buffer =
        if r_image.size >
               ::std::mem::size_of::<[byte; 1024]>() as libc::c_ulong {
            0 as *mut byte
        } else { data2D.as_mut_ptr() };
    r_image.palette = 0 as *mut byte;
    r_image.numMips = 1 as libc::c_int as byte;
    r_image.encode = 0 as libc::c_int as word;
    if r_image.flags & IMAGE_CUBEMAP as libc::c_int as libc::c_uint != 0 {
        r_image.size =
            (r_image.size as
                 libc::c_ulong).wrapping_mul(6 as libc::c_int as
                                                 libc::c_ulong) as size_t as
                size_t
    }
    memset(data2D.as_mut_ptr() as *mut libc::c_void, 0xff as libc::c_int,
           ::std::mem::size_of::<[byte; 1024]>() as libc::c_ulong);
    return &mut r_image;
}
/*
==================
R_InitDlightTexture
==================
*/
#[no_mangle]
pub unsafe extern "C" fn R_InitDlightTexture() {
    let mut r_image: rgbdata_t =
        rgbdata_t{width: 0,
                  height: 0,
                  depth: 0,
                  type_0: 0,
                  flags: 0,
                  encode: 0,
                  numMips: 0,
                  palette: 0 as *mut byte,
                  buffer: 0 as *mut byte,
                  fogParams: [0; 4],
                  size: 0,}; // already initialized
    if tr.dlightTexture != 0 as libc::c_int { return }
    memset(&mut r_image as *mut rgbdata_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<rgbdata_t>() as libc::c_ulong);
    r_image.width = tr.block_size as word;
    r_image.height = tr.block_size as word;
    r_image.flags = IMAGE_HAS_COLOR as libc::c_int as uint;
    r_image.type_0 = PF_RGBA_32 as libc::c_int as uint;
    r_image.size =
        (r_image.width as libc::c_int * r_image.height as libc::c_int *
             4 as libc::c_int) as size_t;
    tr.dlightTexture =
        GL_LoadTextureFromBuffer(b"*dlight\x00" as *const u8 as
                                     *const libc::c_char, &mut r_image,
                                 (TF_NOMIPMAP as libc::c_int |
                                      TF_CLAMP as libc::c_int |
                                      TF_ATLAS_PAGE as libc::c_int) as
                                     texFlags_t, false_0);
}
/*
==================
GL_CreateInternalTextures
==================
*/
unsafe extern "C" fn GL_CreateInternalTextures() {
    let mut dx2: libc::c_int = 0;
    let mut dy: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut pic: *mut rgbdata_t = 0 as *mut rgbdata_t;
    // emo-texture from quake1
    pic =
        GL_FakeImage(16 as libc::c_int, 16 as libc::c_int, 1 as libc::c_int,
                     IMAGE_HAS_COLOR as libc::c_int);
    y = 0 as libc::c_int;
    while y < 16 as libc::c_int {
        x = 0 as libc::c_int;
        while x < 16 as libc::c_int {
            if (y < 8 as libc::c_int) as libc::c_int ^
                   (x < 8 as libc::c_int) as libc::c_int != 0 {
                *((*pic).buffer as
                      *mut uint).offset((y * 16 as libc::c_int + x) as isize)
                    = 0xffff00ff as libc::c_uint
            } else {
                *((*pic).buffer as
                      *mut uint).offset((y * 16 as libc::c_int + x) as isize)
                    = 0xff000000 as libc::c_uint
            }
            x += 1
        }
        y += 1
    }
    tr.defaultTexture =
        GL_LoadTextureFromBuffer(b"*default\x00" as *const u8 as
                                     *const libc::c_char, pic, TF_COLORMAP,
                                 false_0);
    // particle texture from quake1
    pic =
        GL_FakeImage(16 as libc::c_int, 16 as libc::c_int, 1 as libc::c_int,
                     IMAGE_HAS_COLOR as libc::c_int |
                         IMAGE_HAS_ALPHA as libc::c_int);
    x = 0 as libc::c_int;
    while x < 16 as libc::c_int {
        dx2 = x - 8 as libc::c_int;
        dx2 = dx2 * dx2;
        y = 0 as libc::c_int;
        while y < 16 as libc::c_int {
            dy = y - 8 as libc::c_int;
            d =
                (255 as libc::c_int as libc::c_double -
                     35 as libc::c_int as libc::c_double *
                         __tg_sqrt((dx2 + dy * dy) as libc::c_double)) as
                    libc::c_int;
            *(*pic).buffer.offset(((y * 16 as libc::c_int + x) *
                                       4 as libc::c_int + 3 as libc::c_int) as
                                      isize) =
                if d >= 0 as libc::c_int {
                    if d < 255 as libc::c_int {
                        d
                    } else { 255 as libc::c_int }
                } else { 0 as libc::c_int } as byte;
            y += 1
        }
        x += 1
    }
    tr.particleTexture =
        GL_LoadTextureFromBuffer(b"*particle\x00" as *const u8 as
                                     *const libc::c_char, pic, TF_CLAMP,
                                 false_0);
    // white texture
    pic =
        GL_FakeImage(4 as libc::c_int, 4 as libc::c_int, 1 as libc::c_int,
                     IMAGE_HAS_COLOR as libc::c_int);
    x = 0 as libc::c_int;
    while x < 16 as libc::c_int {
        *((*pic).buffer as *mut uint).offset(x as isize) =
            0xffffffff as libc::c_uint;
        x += 1
    }
    tr.whiteTexture =
        GL_LoadTextureFromBuffer(b"*white\x00" as *const u8 as
                                     *const libc::c_char, pic, TF_COLORMAP,
                                 false_0);
    // gray texture
    pic =
        GL_FakeImage(4 as libc::c_int, 4 as libc::c_int, 1 as libc::c_int,
                     IMAGE_HAS_COLOR as libc::c_int);
    x = 0 as libc::c_int;
    while x < 16 as libc::c_int {
        *((*pic).buffer as *mut uint).offset(x as isize) =
            0xff7f7f7f as libc::c_uint;
        x += 1
    }
    tr.grayTexture =
        GL_LoadTextureFromBuffer(b"*gray\x00" as *const u8 as
                                     *const libc::c_char, pic, TF_COLORMAP,
                                 false_0);
    // black texture
    pic =
        GL_FakeImage(4 as libc::c_int, 4 as libc::c_int, 1 as libc::c_int,
                     IMAGE_HAS_COLOR as libc::c_int);
    x = 0 as libc::c_int;
    while x < 16 as libc::c_int {
        *((*pic).buffer as *mut uint).offset(x as isize) =
            0xff000000 as libc::c_uint;
        x += 1
    }
    tr.blackTexture =
        GL_LoadTextureFromBuffer(b"*black\x00" as *const u8 as
                                     *const libc::c_char, pic, TF_COLORMAP,
                                 false_0);
    // cinematic dummy
    pic =
        GL_FakeImage(640 as libc::c_int, 100 as libc::c_int, 1 as libc::c_int,
                     IMAGE_HAS_COLOR as libc::c_int);
    tr.cinTexture =
        GL_LoadTextureFromBuffer(b"*cintexture\x00" as *const u8 as
                                     *const libc::c_char, pic,
                                 (TF_NOMIPMAP as libc::c_int |
                                      TF_CLAMP as libc::c_int) as texFlags_t,
                                 false_0);
}
/*
===============
R_TextureList_f
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_TextureList_f() {
    let mut image: *mut gl_texture_t = 0 as *mut gl_texture_t;
    let mut i: libc::c_int = 0;
    let mut texCount: libc::c_int = 0;
    let mut bytes: libc::c_int = 0 as libc::c_int;
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"\n\x00" as
                                                                 *const u8 as
                                                                 *const libc::c_char);
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b" -id-   -w-  -h-     -size- -fmt- -type- -data-  -encode- -wrap- -depth- -name--------\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char);
    texCount = 0 as libc::c_int;
    i = texCount;
    image = gl_textures.as_mut_ptr();
    while (i as libc::c_uint) < gl_numTextures {
        if !((*image).texnum == 0) {
            bytes =
                (bytes as libc::c_ulong).wrapping_add((*image).size) as
                    libc::c_int as libc::c_int;
            texCount += 1;
            gEngfuncs.Con_Printf.expect("non-null function pointer")(b"%4i: \x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     i);
            gEngfuncs.Con_Printf.expect("non-null function pointer")(b"%4i %4i \x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     (*image).width
                                                                         as
                                                                         libc::c_int,
                                                                     (*image).height
                                                                         as
                                                                         libc::c_int);
            gEngfuncs.Con_Printf.expect("non-null function pointer")(b"%12s \x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     Q_pretifymem((*image).size
                                                                                      as
                                                                                      libc::c_float,
                                                                                  2
                                                                                      as
                                                                                      libc::c_int));
            match (*image).format {
                34030 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"CRGBA \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                34029 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"CRGB  \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                34027 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"CLA   \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                34026 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"CL    \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                34025 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"CA    \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                34028 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"CI    \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                33776 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"DXT1c \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                33777 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"DXT1a \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                33778 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"DXT3  \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                33779 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"DXT5  \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                36285 | 34871 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"ATI2  \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                6408 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"RGBA  \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                32856 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"RGBA8 \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                32854 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"RGBA4 \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                6407 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"RGB   \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                32849 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"RGB8  \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                32848 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"RGB5  \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                32835 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"L4A4  \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                6410 | 32837 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"L8A8  \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                32831 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"L4    \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                6409 | 32832 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"L8    \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                32828 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"A8    \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                32843 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"I8    \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                6402 | 33190 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"DPTH24\x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                36012 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"DPTH32\x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                34846 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"L16F  \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                34840 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"L32F  \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                34847 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"LA16F \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                34841 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"LA32F \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                33327 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"RG16F \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                33328 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"RG32F \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                34843 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"RGB16F\x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                34837 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"RGB32F\x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                34842 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"RGBA16F\x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                34836 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"RGBA32F\x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                _ => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b" ^1ERROR^7 \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
            }
            match (*image).target {
                3552 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b" 1D   \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                3553 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b" 2D   \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                32879 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b" 3D   \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                34067 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"CUBE  \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                34037 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"RECT  \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                35866 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"ARRAY \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                37120 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"MSAA  \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                _ => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"????  \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
            }
            if (*image).flags as libc::c_uint &
                   TF_NORMALMAP as libc::c_int as libc::c_uint != 0 {
                gEngfuncs.Con_Printf.expect("non-null function pointer")(b"normal  \x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char);
            } else {
                gEngfuncs.Con_Printf.expect("non-null function pointer")(b"diffuse \x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char);
            }
            match (*image).encode {
                6657 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"YCoCg     \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                6661 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"ortho     \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                6662 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"stereo    \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                6663 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"parabolic \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                6664 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"quartic   \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                6665 => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"azimuthal \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                _ => {
                    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"default   \x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
            }
            if (*image).flags as libc::c_uint &
                   TF_CLAMP as libc::c_int as libc::c_uint != 0 {
                gEngfuncs.Con_Printf.expect("non-null function pointer")(b"clamp  \x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char);
            } else if (*image).flags as libc::c_uint &
                          TF_BORDER as libc::c_int as libc::c_uint != 0 {
                gEngfuncs.Con_Printf.expect("non-null function pointer")(b"border \x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char);
            } else {
                gEngfuncs.Con_Printf.expect("non-null function pointer")(b"repeat \x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char);
            }
            gEngfuncs.Con_Printf.expect("non-null function pointer")(b"   %d  \x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     (*image).depth
                                                                         as
                                                                         libc::c_int);
            gEngfuncs.Con_Printf.expect("non-null function pointer")(b"  %s\n\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     (*image).name.as_mut_ptr());
        }
        i += 1;
        image = image.offset(1)
    }
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"---------------------------------------------------------\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char);
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"%i total textures\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             texCount);
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"%s total memory used\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             Q_pretifymem(bytes
                                                                              as
                                                                              libc::c_float,
                                                                          2 as
                                                                              libc::c_int));
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"\n\x00" as
                                                                 *const u8 as
                                                                 *const libc::c_char);
}
/*
===============
R_InitImages
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_InitImages() {
    memset(gl_textures.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[gl_texture_t; 4096]>() as libc::c_ulong);
    memset(gl_texturesHashTable.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<[*mut gl_texture_t; 1024]>() as
               libc::c_ulong);
    gl_numTextures = 0 as libc::c_int as uint;
    // create unused 0-entry
    Q_strncpy((*gl_textures.as_mut_ptr()).name.as_mut_ptr(),
              b"*unused*\x00" as *const u8 as *const libc::c_char,
              ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong);
    (*gl_textures.as_mut_ptr()).hashValue =
        COM_HashKey((*gl_textures.as_mut_ptr()).name.as_mut_ptr(),
                    (4096 as libc::c_int >> 2 as libc::c_int) as uint);
    let ref mut fresh1 = (*gl_textures.as_mut_ptr()).nextHash;
    *fresh1 =
        gl_texturesHashTable[(*gl_textures.as_mut_ptr()).hashValue as usize];
    gl_texturesHashTable[(*gl_textures.as_mut_ptr()).hashValue as usize] =
        gl_textures.as_mut_ptr();
    gl_numTextures = 1 as libc::c_int as uint;
    // validate cvars
    R_SetTextureParameters();
    GL_CreateInternalTextures();
    gEngfuncs.Cmd_AddCommand.expect("non-null function pointer")(b"texturelist\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 Some(R_TextureList_f
                                                                          as
                                                                          unsafe extern "C" fn()
                                                                              ->
                                                                                  ()),
                                                                 b"display loaded textures list\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char);
}
/*
===============
R_ShutdownImages
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_ShutdownImages() {
    let mut tex: *mut gl_texture_t = 0 as *mut gl_texture_t;
    let mut i: libc::c_int = 0;
    gEngfuncs.Cmd_RemoveCommand.expect("non-null function pointer")(b"texturelist\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char);
    GL_CleanupAllTextureUnits();
    i = 0 as libc::c_int;
    tex = gl_textures.as_mut_ptr();
    while (i as libc::c_uint) < gl_numTextures {
        GL_DeleteTexture(tex);
        i += 1;
        tex = tex.offset(1)
    }
    memset(tr.lightmapTextures.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<[libc::c_int; 256]>() as libc::c_ulong);
    memset(gl_texturesHashTable.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<[*mut gl_texture_t; 1024]>() as
               libc::c_ulong);
    memset(gl_textures.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[gl_texture_t; 4096]>() as libc::c_ulong);
    gl_numTextures = 0 as libc::c_int as uint;
}
