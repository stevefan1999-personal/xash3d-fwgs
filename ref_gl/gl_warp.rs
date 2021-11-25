#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, extern_types,
           register_tool)]
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
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn sqrtf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn fabsf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    static mut pglBegin: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglDepthFunc: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglDepthMask: Option<unsafe extern "C" fn(_: GLboolean) -> ()>;
    #[no_mangle]
    static mut pglDisable: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglEnd: Option<unsafe extern "C" fn() -> ()>;
    #[no_mangle]
    static mut pglFogf:
           Option<unsafe extern "C" fn(_: GLenum, _: GLfloat) -> ()>;
    #[no_mangle]
    static mut pglTexCoord2f:
           Option<unsafe extern "C" fn(_: GLfloat, _: GLfloat) -> ()>;
    #[no_mangle]
    static mut pglTexEnvi:
           Option<unsafe extern "C" fn(_: GLenum, _: GLenum, _: GLint) -> ()>;
    #[no_mangle]
    static mut pglVertex3f:
           Option<unsafe extern "C" fn(_: GLfloat, _: GLfloat, _: GLfloat)
                      -> ()>;
    #[no_mangle]
    static mut pglVertex3fv:
           Option<unsafe extern "C" fn(_: *const GLfloat) -> ()>;
    #[no_mangle]
    static mut r_temppool: poolhandle_t;
    #[no_mangle]
    static mut RI: ref_instance_t;
    #[no_mangle]
    static mut tr: gl_globals_t;
    #[no_mangle]
    fn GL_Bind(tmu: GLint, texnum: GLenum);
    #[no_mangle]
    fn GL_SetRenderMode(mode: libc::c_int);
    #[no_mangle]
    fn GL_LoadTexture(name: *const libc::c_char, buf: *const byte,
                      size: size_t, flags: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn GL_LoadTextureFromBuffer(name: *const libc::c_char,
                                pic: *mut rgbdata_t, flags: texFlags_t,
                                update: qboolean) -> libc::c_int;
    #[no_mangle]
    fn GL_FreeTexture(texnum: GLenum);
    #[no_mangle]
    fn R_LoadIdentity();
    #[no_mangle]
    fn R_AllowFog(allowed: qboolean);
    #[no_mangle]
    fn GL_SetupFogColorForSurfaces();
    #[no_mangle]
    fn GL_ResetFogColor();
    #[no_mangle]
    static mut gEngfuncs: ref_api_t;
    #[no_mangle]
    fn Q_snprintf(buffer: *mut libc::c_char, buffersize: size_t,
                  format: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    static mut glw_state: glwstate_t;
    #[no_mangle]
    static mut gpGlobals: *mut ref_globals_t;
    #[no_mangle]
    fn Q_strnicmp(s1: *const libc::c_char, s2: *const libc::c_char,
                  n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn Q_strlen(string: *const libc::c_char) -> size_t;
    #[no_mangle]
    fn COM_StripExtension(path: *mut libc::c_char);
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
pub type matrix4x4 = [[vec_t; 4]; 4];
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
pub type C2RustUnnamed = libc::c_uint;
pub const kRenderTransAdd: C2RustUnnamed = 5;
pub const kRenderTransAlpha: C2RustUnnamed = 4;
pub const kRenderGlow: C2RustUnnamed = 3;
pub const kRenderTransTexture: C2RustUnnamed = 2;
pub const kRenderTransColor: C2RustUnnamed = 1;
pub const kRenderNormal: C2RustUnnamed = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gl_frustum_s {
    pub planes: [mplane_t; 6],
    pub clipFlags: libc::c_uint,
}
pub type gl_frustum_t = gl_frustum_s;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const PF_TOTALCOUNT: C2RustUnnamed_0 = 12;
pub const PF_ATI2: C2RustUnnamed_0 = 11;
pub const PF_DXT5: C2RustUnnamed_0 = 10;
pub const PF_DXT3: C2RustUnnamed_0 = 9;
pub const PF_DXT1: C2RustUnnamed_0 = 8;
pub const PF_LUMINANCE: C2RustUnnamed_0 = 7;
pub const PF_BGR_24: C2RustUnnamed_0 = 6;
pub const PF_RGB_24: C2RustUnnamed_0 = 5;
pub const PF_BGRA_32: C2RustUnnamed_0 = 4;
pub const PF_RGBA_32: C2RustUnnamed_0 = 3;
pub const PF_INDEXED_32: C2RustUnnamed_0 = 2;
pub const PF_INDEXED_24: C2RustUnnamed_0 = 1;
pub const PF_UNKNOWN: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bpc_desc_s {
    pub format: libc::c_int,
    pub name: [libc::c_char; 16],
    pub glFormat: uint,
    pub bpp: libc::c_int,
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const IMAGE_REMAP: C2RustUnnamed_1 = 134217728;
pub const IMAGE_LIGHTGAMMA: C2RustUnnamed_1 = 67108864;
pub const IMAGE_QUANTIZE: C2RustUnnamed_1 = 33554432;
pub const IMAGE_MAKE_LUMA: C2RustUnnamed_1 = 16777216;
pub const IMAGE_FORCE_RGBA: C2RustUnnamed_1 = 8388608;
pub const IMAGE_RESAMPLE: C2RustUnnamed_1 = 1048576;
pub const IMAGE_EMBOSS: C2RustUnnamed_1 = 524288;
pub const IMAGE_ROT270: C2RustUnnamed_1 = 458752;
pub const IMAGE_ROT180: C2RustUnnamed_1 = 196608;
pub const IMAGE_ROT_90: C2RustUnnamed_1 = 262144;
pub const IMAGE_FLIP_Y: C2RustUnnamed_1 = 131072;
pub const IMAGE_FLIP_X: C2RustUnnamed_1 = 65536;
pub const IMAGE_QUAKEPAL: C2RustUnnamed_1 = 1024;
pub const IMAGE_ONEBIT_ALPHA: C2RustUnnamed_1 = 512;
pub const IMAGE_MULTILAYER: C2RustUnnamed_1 = 256;
pub const IMAGE_DDS_FORMAT: C2RustUnnamed_1 = 128;
pub const IMAGE_QUAKESKY: C2RustUnnamed_1 = 64;
pub const IMAGE_SKYBOX: C2RustUnnamed_1 = 32;
pub const IMAGE_HAS_LUMA: C2RustUnnamed_1 = 16;
pub const IMAGE_COLORINDEX: C2RustUnnamed_1 = 8;
pub const IMAGE_HAS_COLOR: C2RustUnnamed_1 = 4;
pub const IMAGE_HAS_ALPHA: C2RustUnnamed_1 = 2;
pub const IMAGE_CUBEMAP: C2RustUnnamed_1 = 1;
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
pub type C2RustUnnamed_2 = libc::c_int;
pub const MAX_TEXTURE_UNITS: C2RustUnnamed_2 = 32;
pub const XASH_TEXTURE4: C2RustUnnamed_2 = 4;
pub const XASH_TEXTURE3: C2RustUnnamed_2 = 3;
pub const XASH_TEXTURE2: C2RustUnnamed_2 = 2;
pub const XASH_TEXTURE1: C2RustUnnamed_2 = 1;
pub const XASH_TEXTURE0: C2RustUnnamed_2 = 0;
pub const GL_KEEP_UNIT: C2RustUnnamed_2 = -1;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mip_s {
    pub name: [libc::c_char; 16],
    pub width: libc::c_uint,
    pub height: libc::c_uint,
    pub offsets: [libc::c_uint; 4],
}
pub type GLenum = uint;
pub type GLboolean = byte;
pub type GLint = libc::c_int;
pub type GLfloat = libc::c_float;
pub type mip_t = mip_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ref_instance_t {
    pub params: libc::c_int,
    pub drawWorld: qboolean,
    pub isSkyVisible: qboolean,
    pub onlyClientDraw: qboolean,
    pub drawOrtho: qboolean,
    pub fov_x: libc::c_float,
    pub fov_y: libc::c_float,
    pub currententity: *mut cl_entity_t,
    pub currentmodel: *mut model_t,
    pub currentbeam: *mut cl_entity_t,
    pub viewport: [libc::c_int; 4],
    pub frustum: gl_frustum_t,
    pub viewleaf: *mut mleaf_t,
    pub oldviewleaf: *mut mleaf_t,
    pub pvsorigin: vec3_t,
    pub vieworg: vec3_t,
    pub viewangles: vec3_t,
    pub vforward: vec3_t,
    pub vright: vec3_t,
    pub vup: vec3_t,
    pub cullorigin: vec3_t,
    pub cull_vforward: vec3_t,
    pub cull_vright: vec3_t,
    pub cull_vup: vec3_t,
    pub farClip: libc::c_float,
    pub fogCustom: qboolean,
    pub fogEnabled: qboolean,
    pub fogSkybox: qboolean,
    pub fogColor: vec4_t,
    pub fogDensity: libc::c_float,
    pub fogStart: libc::c_float,
    pub fogEnd: libc::c_float,
    pub cached_contents: libc::c_int,
    pub cached_waterlevel: libc::c_int,
    pub skyMins: [[libc::c_float; 6]; 2],
    pub skyMaxs: [[libc::c_float; 6]; 2],
    pub objectMatrix: matrix4x4,
    pub worldviewMatrix: matrix4x4,
    pub modelviewMatrix: matrix4x4,
    pub projectionMatrix: matrix4x4,
    pub worldviewProjectionMatrix: matrix4x4,
    pub visbytes: [byte; 4096],
    pub viewplanedist: libc::c_float,
    pub clipPlane: mplane_t,
}
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
pub struct glwstate_t {
    pub initialized: qboolean,
    pub extended: qboolean,
}
#[inline(always)]
unsafe extern "C" fn __tg_sqrt(mut __x: libc::c_float) -> libc::c_float {
    return sqrtf(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_fabs(mut __x: libc::c_float) -> libc::c_float {
    return fabsf(__x);
}
#[no_mangle]
pub static mut r_skyBoxSuffix: [*const libc::c_char; 6] =
    [b"rt\x00" as *const u8 as *const libc::c_char,
     b"bk\x00" as *const u8 as *const libc::c_char,
     b"lf\x00" as *const u8 as *const libc::c_char,
     b"ft\x00" as *const u8 as *const libc::c_char,
     b"up\x00" as *const u8 as *const libc::c_char,
     b"dn\x00" as *const u8 as *const libc::c_char];
static mut r_skyTexOrder: [libc::c_int; 6] =
    [0 as libc::c_int, 2 as libc::c_int, 1 as libc::c_int, 3 as libc::c_int,
     4 as libc::c_int, 5 as libc::c_int];
static mut skyclip: [vec3_t; 6] =
    [[1 as libc::c_int as vec_t, 1 as libc::c_int as vec_t,
      0 as libc::c_int as vec_t],
     [1 as libc::c_int as vec_t, -(1 as libc::c_int) as vec_t,
      0 as libc::c_int as vec_t],
     [0 as libc::c_int as vec_t, -(1 as libc::c_int) as vec_t,
      1 as libc::c_int as vec_t],
     [0 as libc::c_int as vec_t, 1 as libc::c_int as vec_t,
      1 as libc::c_int as vec_t],
     [1 as libc::c_int as vec_t, 0 as libc::c_int as vec_t,
      1 as libc::c_int as vec_t],
     [-(1 as libc::c_int) as vec_t, 0 as libc::c_int as vec_t,
      1 as libc::c_int as vec_t]];
// 1 = s, 2 = t, 3 = 2048
static mut st_to_vec: [[libc::c_int; 3]; 6] =
    [[3 as libc::c_int, -(1 as libc::c_int), 2 as libc::c_int],
     [-(3 as libc::c_int), 1 as libc::c_int, 2 as libc::c_int],
     [1 as libc::c_int, 3 as libc::c_int, 2 as libc::c_int],
     [-(1 as libc::c_int), -(3 as libc::c_int), 2 as libc::c_int],
     [-(2 as libc::c_int), -(1 as libc::c_int), 3 as libc::c_int],
     [2 as libc::c_int, -(1 as libc::c_int), -(3 as libc::c_int)]];
// s = [0]/[2], t = [1]/[2]
static mut vec_to_st: [[libc::c_int; 3]; 6] =
    [[-(2 as libc::c_int), 3 as libc::c_int, 1 as libc::c_int],
     [2 as libc::c_int, 3 as libc::c_int, -(1 as libc::c_int)],
     [1 as libc::c_int, 3 as libc::c_int, 2 as libc::c_int],
     [-(1 as libc::c_int), 3 as libc::c_int, -(2 as libc::c_int)],
     [-(2 as libc::c_int), -(1 as libc::c_int), 3 as libc::c_int],
     [-(2 as libc::c_int), 1 as libc::c_int, -(3 as libc::c_int)]];
// speed up sin calculations
#[no_mangle]
pub static mut r_turbsin: [libc::c_float; 256] =
    [0.000000f64 as libc::c_float, 0.098165f64 as libc::c_float,
     0.196270f64 as libc::c_float, 0.294259f64 as libc::c_float,
     0.392069f64 as libc::c_float, 0.489643f64 as libc::c_float,
     0.586920f64 as libc::c_float, 0.683850f64 as libc::c_float,
     0.780360f64 as libc::c_float, 0.876405f64 as libc::c_float,
     0.971920f64 as libc::c_float, 1.066850f64 as libc::c_float,
     1.161140f64 as libc::c_float, 1.254725f64 as libc::c_float,
     1.347560f64 as libc::c_float, 1.439580f64 as libc::c_float,
     1.530735f64 as libc::c_float, 1.620965f64 as libc::c_float,
     1.710220f64 as libc::c_float, 1.798445f64 as libc::c_float,
     1.885585f64 as libc::c_float, 1.971595f64 as libc::c_float,
     2.056410f64 as libc::c_float, 2.139990f64 as libc::c_float,
     2.222280f64 as libc::c_float, 2.303235f64 as libc::c_float,
     2.382795f64 as libc::c_float, 2.460925f64 as libc::c_float,
     2.537575f64 as libc::c_float, 2.612690f64 as libc::c_float,
     2.686235f64 as libc::c_float, 2.758160f64 as libc::c_float,
     2.828425f64 as libc::c_float, 2.896990f64 as libc::c_float,
     2.963805f64 as libc::c_float, 3.028835f64 as libc::c_float,
     3.092040f64 as libc::c_float, 3.153385f64 as libc::c_float,
     3.212830f64 as libc::c_float, 3.270340f64 as libc::c_float,
     3.325880f64 as libc::c_float, 3.379415f64 as libc::c_float,
     3.430915f64 as libc::c_float, 3.480350f64 as libc::c_float,
     3.527685f64 as libc::c_float, 3.572895f64 as libc::c_float,
     3.615955f64 as libc::c_float, 3.656840f64 as libc::c_float,
     3.695520f64 as libc::c_float, 3.731970f64 as libc::c_float,
     3.766175f64 as libc::c_float, 3.798115f64 as libc::c_float,
     3.827760f64 as libc::c_float, 3.855105f64 as libc::c_float,
     3.880125f64 as libc::c_float, 3.902810f64 as libc::c_float,
     3.923140f64 as libc::c_float, 3.941110f64 as libc::c_float,
     3.956705f64 as libc::c_float, 3.969920f64 as libc::c_float,
     3.980740f64 as libc::c_float, 3.989160f64 as libc::c_float,
     3.995180f64 as libc::c_float, 3.998795f64 as libc::c_float,
     4.000000f64 as libc::c_float, 3.998795f64 as libc::c_float,
     3.995180f64 as libc::c_float, 3.989160f64 as libc::c_float,
     3.980740f64 as libc::c_float, 3.969920f64 as libc::c_float,
     3.956705f64 as libc::c_float, 3.941110f64 as libc::c_float,
     3.923140f64 as libc::c_float, 3.902810f64 as libc::c_float,
     3.880125f64 as libc::c_float, 3.855105f64 as libc::c_float,
     3.827760f64 as libc::c_float, 3.798115f64 as libc::c_float,
     3.766175f64 as libc::c_float, 3.731970f64 as libc::c_float,
     3.695520f64 as libc::c_float, 3.656840f64 as libc::c_float,
     3.615955f64 as libc::c_float, 3.572895f64 as libc::c_float,
     3.527685f64 as libc::c_float, 3.480350f64 as libc::c_float,
     3.430915f64 as libc::c_float, 3.379415f64 as libc::c_float,
     3.325880f64 as libc::c_float, 3.270340f64 as libc::c_float,
     3.212830f64 as libc::c_float, 3.153385f64 as libc::c_float,
     3.092040f64 as libc::c_float, 3.028835f64 as libc::c_float,
     2.963805f64 as libc::c_float, 2.896990f64 as libc::c_float,
     2.828425f64 as libc::c_float, 2.758160f64 as libc::c_float,
     2.686235f64 as libc::c_float, 2.612690f64 as libc::c_float,
     2.537575f64 as libc::c_float, 2.460925f64 as libc::c_float,
     2.382795f64 as libc::c_float, 2.303235f64 as libc::c_float,
     2.222280f64 as libc::c_float, 2.139990f64 as libc::c_float,
     2.056410f64 as libc::c_float, 1.971595f64 as libc::c_float,
     1.885585f64 as libc::c_float, 1.798445f64 as libc::c_float,
     1.710220f64 as libc::c_float, 1.620965f64 as libc::c_float,
     1.530735f64 as libc::c_float, 1.439580f64 as libc::c_float,
     1.347560f64 as libc::c_float, 1.254725f64 as libc::c_float,
     1.161140f64 as libc::c_float, 1.066850f64 as libc::c_float,
     0.971920f64 as libc::c_float, 0.876405f64 as libc::c_float,
     0.780360f64 as libc::c_float, 0.683850f64 as libc::c_float,
     0.586920f64 as libc::c_float, 0.489643f64 as libc::c_float,
     0.392069f64 as libc::c_float, 0.294259f64 as libc::c_float,
     0.196270f64 as libc::c_float, 0.098165f64 as libc::c_float,
     0.000000f64 as libc::c_float, -0.098165f64 as libc::c_float,
     -0.196270f64 as libc::c_float, -0.294259f64 as libc::c_float,
     -0.392069f64 as libc::c_float, -0.489643f64 as libc::c_float,
     -0.586920f64 as libc::c_float, -0.683850f64 as libc::c_float,
     -0.780360f64 as libc::c_float, -0.876405f64 as libc::c_float,
     -0.971920f64 as libc::c_float, -1.066850f64 as libc::c_float,
     -1.161140f64 as libc::c_float, -1.254725f64 as libc::c_float,
     -1.347560f64 as libc::c_float, -1.439580f64 as libc::c_float,
     -1.530735f64 as libc::c_float, -1.620965f64 as libc::c_float,
     -1.710220f64 as libc::c_float, -1.798445f64 as libc::c_float,
     -1.885585f64 as libc::c_float, -1.971595f64 as libc::c_float,
     -2.056410f64 as libc::c_float, -2.139990f64 as libc::c_float,
     -2.222280f64 as libc::c_float, -2.303235f64 as libc::c_float,
     -2.382795f64 as libc::c_float, -2.460925f64 as libc::c_float,
     -2.537575f64 as libc::c_float, -2.612690f64 as libc::c_float,
     -2.686235f64 as libc::c_float, -2.758160f64 as libc::c_float,
     -2.828425f64 as libc::c_float, -2.896990f64 as libc::c_float,
     -2.963805f64 as libc::c_float, -3.028835f64 as libc::c_float,
     -3.092040f64 as libc::c_float, -3.153385f64 as libc::c_float,
     -3.212830f64 as libc::c_float, -3.270340f64 as libc::c_float,
     -3.325880f64 as libc::c_float, -3.379415f64 as libc::c_float,
     -3.430915f64 as libc::c_float, -3.480350f64 as libc::c_float,
     -3.527685f64 as libc::c_float, -3.572895f64 as libc::c_float,
     -3.615955f64 as libc::c_float, -3.656840f64 as libc::c_float,
     -3.695520f64 as libc::c_float, -3.731970f64 as libc::c_float,
     -3.766175f64 as libc::c_float, -3.798115f64 as libc::c_float,
     -3.827760f64 as libc::c_float, -3.855105f64 as libc::c_float,
     -3.880125f64 as libc::c_float, -3.902810f64 as libc::c_float,
     -3.923140f64 as libc::c_float, -3.941110f64 as libc::c_float,
     -3.956705f64 as libc::c_float, -3.969920f64 as libc::c_float,
     -3.980740f64 as libc::c_float, -3.989160f64 as libc::c_float,
     -3.995180f64 as libc::c_float, -3.998795f64 as libc::c_float,
     -4.000000f64 as libc::c_float, -3.998795f64 as libc::c_float,
     -3.995180f64 as libc::c_float, -3.989160f64 as libc::c_float,
     -3.980740f64 as libc::c_float, -3.969920f64 as libc::c_float,
     -3.956705f64 as libc::c_float, -3.941110f64 as libc::c_float,
     -3.923140f64 as libc::c_float, -3.902810f64 as libc::c_float,
     -3.880125f64 as libc::c_float, -3.855105f64 as libc::c_float,
     -3.827760f64 as libc::c_float, -3.798115f64 as libc::c_float,
     -3.766175f64 as libc::c_float, -3.731970f64 as libc::c_float,
     -3.695520f64 as libc::c_float, -3.656840f64 as libc::c_float,
     -3.615955f64 as libc::c_float, -3.572895f64 as libc::c_float,
     -3.527685f64 as libc::c_float, -3.480350f64 as libc::c_float,
     -3.430915f64 as libc::c_float, -3.379415f64 as libc::c_float,
     -3.325880f64 as libc::c_float, -3.270340f64 as libc::c_float,
     -3.212830f64 as libc::c_float, -3.153385f64 as libc::c_float,
     -3.092040f64 as libc::c_float, -3.028835f64 as libc::c_float,
     -2.963805f64 as libc::c_float, -2.896990f64 as libc::c_float,
     -2.828425f64 as libc::c_float, -2.758160f64 as libc::c_float,
     -2.686235f64 as libc::c_float, -2.612690f64 as libc::c_float,
     -2.537575f64 as libc::c_float, -2.460925f64 as libc::c_float,
     -2.382795f64 as libc::c_float, -2.303235f64 as libc::c_float,
     -2.222280f64 as libc::c_float, -2.139990f64 as libc::c_float,
     -2.056410f64 as libc::c_float, -1.971595f64 as libc::c_float,
     -1.885585f64 as libc::c_float, -1.798445f64 as libc::c_float,
     -1.710220f64 as libc::c_float, -1.620965f64 as libc::c_float,
     -1.530735f64 as libc::c_float, -1.439580f64 as libc::c_float,
     -1.347560f64 as libc::c_float, -1.254725f64 as libc::c_float,
     -1.161140f64 as libc::c_float, -1.066850f64 as libc::c_float,
     -0.971920f64 as libc::c_float, -0.876405f64 as libc::c_float,
     -0.780360f64 as libc::c_float, -0.683850f64 as libc::c_float,
     -0.586920f64 as libc::c_float, -0.489643f64 as libc::c_float,
     -0.392069f64 as libc::c_float, -0.294259f64 as libc::c_float,
     -0.196270f64 as libc::c_float, -0.098165f64 as libc::c_float];
unsafe extern "C" fn CheckSkybox(mut name: *const libc::c_char)
 -> libc::c_int {
    let mut skybox_ext: [*const libc::c_char; 3] =
        [b"dds\x00" as *const u8 as *const libc::c_char,
         b"tga\x00" as *const u8 as *const libc::c_char,
         b"bmp\x00" as *const u8 as *const libc::c_char];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut num_checked_sides: libc::c_int = 0;
    let mut sidename: *const libc::c_char = 0 as *const libc::c_char;
    // search for skybox images
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        num_checked_sides = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < 6 as libc::c_int {
            // images exists
            // build side name
            sidename =
                va(b"%s%s.%s\x00" as *const u8 as *const libc::c_char, name,
                   r_skyBoxSuffix[j as usize],
                   skybox_ext[i as usize]); // image exists
            if gEngfuncs.FS_FileExists.expect("non-null function pointer")(sidename,
                                                                           false_0
                                                                               as
                                                                               libc::c_int)
                   != 0 {
                num_checked_sides += 1
            }
            j += 1
        }
        if num_checked_sides == 6 as libc::c_int { return 1 as libc::c_int }
        j = 0 as libc::c_int;
        while j < 6 as libc::c_int {
            // build side name
            sidename =
                va(b"%s_%s.%s\x00" as *const u8 as *const libc::c_char, name,
                   r_skyBoxSuffix[j as usize], skybox_ext[i as usize]);
            if gEngfuncs.FS_FileExists.expect("non-null function pointer")(sidename,
                                                                           false_0
                                                                               as
                                                                               libc::c_int)
                   != 0 {
                num_checked_sides += 1
            }
            j += 1
        }
        if num_checked_sides == 6 as libc::c_int { return 2 as libc::c_int }
        i += 1
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn DrawSkyPolygon(mut nump: libc::c_int,
                                        mut vecs: *mut vec_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut axis: libc::c_int = 0;
    let mut s: libc::c_float = 0.;
    let mut t: libc::c_float = 0.;
    let mut dv: libc::c_float = 0.;
    let mut vp: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut v: vec3_t = [0.; 3];
    let mut av: vec3_t = [0.; 3];
    // decide which face it maps to
    v[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    v[1 as libc::c_int as usize] = v[2 as libc::c_int as usize];
    v[0 as libc::c_int as usize] = v[1 as libc::c_int as usize];
    i = 0 as libc::c_int;
    vp = vecs;
    while i < nump {
        v[0 as libc::c_int as usize] =
            *vp.offset(0 as libc::c_int as isize) +
                v[0 as libc::c_int as usize];
        v[1 as libc::c_int as usize] =
            *vp.offset(1 as libc::c_int as isize) +
                v[1 as libc::c_int as usize];
        v[2 as libc::c_int as usize] =
            *vp.offset(2 as libc::c_int as isize) +
                v[2 as libc::c_int as usize];
        i += 1;
        vp = vp.offset(3 as libc::c_int as isize)
    }
    av[0 as libc::c_int as usize] = __tg_fabs(v[0 as libc::c_int as usize]);
    av[1 as libc::c_int as usize] = __tg_fabs(v[1 as libc::c_int as usize]);
    av[2 as libc::c_int as usize] = __tg_fabs(v[2 as libc::c_int as usize]);
    if av[0 as libc::c_int as usize] > av[1 as libc::c_int as usize] &&
           av[0 as libc::c_int as usize] > av[2 as libc::c_int as usize] {
        axis =
            if v[0 as libc::c_int as usize] <
                   0 as libc::c_int as libc::c_float {
                1 as libc::c_int
            } else { 0 as libc::c_int }
    } else if av[1 as libc::c_int as usize] > av[2 as libc::c_int as usize] &&
                  av[1 as libc::c_int as usize] >
                      av[0 as libc::c_int as usize] {
        axis =
            if v[1 as libc::c_int as usize] <
                   0 as libc::c_int as libc::c_float {
                3 as libc::c_int
            } else { 2 as libc::c_int }
    } else {
        axis =
            if v[2 as libc::c_int as usize] <
                   0 as libc::c_int as libc::c_float {
                5 as libc::c_int
            } else { 4 as libc::c_int }
    }
    // project new texture coords
    i = 0 as libc::c_int;
    while i < nump {
        j = vec_to_st[axis as usize][2 as libc::c_int as usize];
        dv =
            if j > 0 as libc::c_int {
                *vecs.offset((j - 1 as libc::c_int) as isize)
            } else { -*vecs.offset((-j - 1 as libc::c_int) as isize) };
        if !(dv == 0.0f32) {
            j = vec_to_st[axis as usize][0 as libc::c_int as usize];
            s =
                if j < 0 as libc::c_int {
                    (-*vecs.offset((-j - 1 as libc::c_int) as isize)) / dv
                } else {
                    (*vecs.offset((j - 1 as libc::c_int) as isize)) / dv
                };
            j = vec_to_st[axis as usize][1 as libc::c_int as usize];
            t =
                if j < 0 as libc::c_int {
                    (-*vecs.offset((-j - 1 as libc::c_int) as isize)) / dv
                } else {
                    (*vecs.offset((j - 1 as libc::c_int) as isize)) / dv
                };
            if s < RI.skyMins[0 as libc::c_int as usize][axis as usize] {
                RI.skyMins[0 as libc::c_int as usize][axis as usize] = s
            }
            if t < RI.skyMins[1 as libc::c_int as usize][axis as usize] {
                RI.skyMins[1 as libc::c_int as usize][axis as usize] = t
            }
            if s > RI.skyMaxs[0 as libc::c_int as usize][axis as usize] {
                RI.skyMaxs[0 as libc::c_int as usize][axis as usize] = s
            }
            if t > RI.skyMaxs[1 as libc::c_int as usize][axis as usize] {
                RI.skyMaxs[1 as libc::c_int as usize][axis as usize] = t
            }
        }
        i += 1;
        vecs = vecs.offset(3 as libc::c_int as isize)
    };
}
/*
==============
ClipSkyPolygon
==============
*/
#[no_mangle]
pub unsafe extern "C" fn ClipSkyPolygon(mut nump: libc::c_int,
                                        mut vecs: *mut vec_t,
                                        mut stage: libc::c_int) {
    let mut norm: *const libc::c_float = 0 as *const libc::c_float;
    let mut v: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut d: libc::c_float = 0.;
    let mut e: libc::c_float = 0.;
    let mut front: qboolean = false_0;
    let mut back: qboolean = false_0;
    let mut dists: [libc::c_float; 129] = [0.; 129];
    let mut sides: [libc::c_int; 129] = [0; 129];
    let mut newv: [[vec3_t; 129]; 2] = [[[0.; 3]; 129]; 2];
    let mut newc: [libc::c_int; 2] = [0; 2];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if nump > 128 as libc::c_int {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"ClipSkyPolygon: MAX_CLIP_VERTS\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char);
    }
    loop  {
        if stage == 6 as libc::c_int {
            // fully clipped, so draw it
            DrawSkyPolygon(nump, vecs);
            return
        }
        back = false_0;
        front = back;
        norm = skyclip[stage as usize].as_ptr();
        i = 0 as libc::c_int;
        v = vecs;
        while i < nump {
            d =
                *v.offset(0 as libc::c_int as isize) *
                    *norm.offset(0 as libc::c_int as isize) +
                    *v.offset(1 as libc::c_int as isize) *
                        *norm.offset(1 as libc::c_int as isize) +
                    *v.offset(2 as libc::c_int as isize) *
                        *norm.offset(2 as libc::c_int as isize);
            if d > 0.1f32 {
                front = true_0;
                sides[i as usize] = 0 as libc::c_int
            } else if d < -0.1f32 {
                back = true_0;
                sides[i as usize] = 1 as libc::c_int
            } else { sides[i as usize] = 2 as libc::c_int }
            dists[i as usize] = d;
            i += 1;
            v = v.offset(3 as libc::c_int as isize)
        }
        if !(front as u64 == 0 || back as u64 == 0) { break ; }
        // not clipped
        stage += 1
    }
    // clip it
    sides[i as usize] = sides[0 as libc::c_int as usize];
    dists[i as usize] = dists[0 as libc::c_int as usize];
    *vecs.offset((i * 3 as libc::c_int) as
                     isize).offset(0 as libc::c_int as isize) =
        *vecs.offset(0 as libc::c_int as isize);
    *vecs.offset((i * 3 as libc::c_int) as
                     isize).offset(1 as libc::c_int as isize) =
        *vecs.offset(1 as libc::c_int as isize);
    *vecs.offset((i * 3 as libc::c_int) as
                     isize).offset(2 as libc::c_int as isize) =
        *vecs.offset(2 as libc::c_int as isize);
    newc[1 as libc::c_int as usize] = 0 as libc::c_int;
    newc[0 as libc::c_int as usize] = newc[1 as libc::c_int as usize];
    i = 0 as libc::c_int;
    v = vecs;
    while i < nump {
        match sides[i as usize] {
            0 => {
                newv[0 as libc::c_int as
                         usize][newc[0 as libc::c_int as usize] as
                                    usize][0 as libc::c_int as usize] =
                    *v.offset(0 as libc::c_int as isize);
                newv[0 as libc::c_int as
                         usize][newc[0 as libc::c_int as usize] as
                                    usize][1 as libc::c_int as usize] =
                    *v.offset(1 as libc::c_int as isize);
                newv[0 as libc::c_int as
                         usize][newc[0 as libc::c_int as usize] as
                                    usize][2 as libc::c_int as usize] =
                    *v.offset(2 as libc::c_int as isize);
                newc[0 as libc::c_int as usize] += 1
            }
            1 => {
                newv[1 as libc::c_int as
                         usize][newc[1 as libc::c_int as usize] as
                                    usize][0 as libc::c_int as usize] =
                    *v.offset(0 as libc::c_int as isize);
                newv[1 as libc::c_int as
                         usize][newc[1 as libc::c_int as usize] as
                                    usize][1 as libc::c_int as usize] =
                    *v.offset(1 as libc::c_int as isize);
                newv[1 as libc::c_int as
                         usize][newc[1 as libc::c_int as usize] as
                                    usize][2 as libc::c_int as usize] =
                    *v.offset(2 as libc::c_int as isize);
                newc[1 as libc::c_int as usize] += 1
            }
            2 => {
                newv[0 as libc::c_int as
                         usize][newc[0 as libc::c_int as usize] as
                                    usize][0 as libc::c_int as usize] =
                    *v.offset(0 as libc::c_int as isize);
                newv[0 as libc::c_int as
                         usize][newc[0 as libc::c_int as usize] as
                                    usize][1 as libc::c_int as usize] =
                    *v.offset(1 as libc::c_int as isize);
                newv[0 as libc::c_int as
                         usize][newc[0 as libc::c_int as usize] as
                                    usize][2 as libc::c_int as usize] =
                    *v.offset(2 as libc::c_int as isize);
                newc[0 as libc::c_int as usize] += 1;
                newv[1 as libc::c_int as
                         usize][newc[1 as libc::c_int as usize] as
                                    usize][0 as libc::c_int as usize] =
                    *v.offset(0 as libc::c_int as isize);
                newv[1 as libc::c_int as
                         usize][newc[1 as libc::c_int as usize] as
                                    usize][1 as libc::c_int as usize] =
                    *v.offset(1 as libc::c_int as isize);
                newv[1 as libc::c_int as
                         usize][newc[1 as libc::c_int as usize] as
                                    usize][2 as libc::c_int as usize] =
                    *v.offset(2 as libc::c_int as isize);
                newc[1 as libc::c_int as usize] += 1
            }
            _ => { }
        }
        if !(sides[i as usize] == 2 as libc::c_int ||
                 sides[(i + 1 as libc::c_int) as usize] == 2 as libc::c_int ||
                 sides[(i + 1 as libc::c_int) as usize] == sides[i as usize])
           {
            d =
                dists[i as usize] /
                    (dists[i as usize] -
                         dists[(i + 1 as libc::c_int) as usize]);
            j = 0 as libc::c_int;
            while j < 3 as libc::c_int {
                e =
                    *v.offset(j as isize) +
                        d *
                            (*v.offset((j + 3 as libc::c_int) as isize) -
                                 *v.offset(j as isize));
                newv[0 as libc::c_int as
                         usize][newc[0 as libc::c_int as usize] as
                                    usize][j as usize] = e;
                newv[1 as libc::c_int as
                         usize][newc[1 as libc::c_int as usize] as
                                    usize][j as usize] = e;
                j += 1
            }
            newc[0 as libc::c_int as usize] += 1;
            newc[1 as libc::c_int as usize] += 1
        }
        i += 1;
        v = v.offset(3 as libc::c_int as isize)
    }
    // continue
    ClipSkyPolygon(newc[0 as libc::c_int as usize],
                   newv[0 as libc::c_int as
                            usize][0 as libc::c_int as usize].as_mut_ptr(),
                   stage + 1 as libc::c_int);
    ClipSkyPolygon(newc[1 as libc::c_int as usize],
                   newv[1 as libc::c_int as
                            usize][0 as libc::c_int as usize].as_mut_ptr(),
                   stage + 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn MakeSkyVec(mut s: libc::c_float,
                                    mut t: libc::c_float,
                                    mut axis: libc::c_int) {
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut farclip: libc::c_int = 0;
    let mut v: vec3_t = [0.; 3];
    let mut b: vec3_t = [0.; 3];
    farclip = RI.farClip as libc::c_int;
    b[0 as libc::c_int as usize] =
        s * (farclip >> 1 as libc::c_int) as libc::c_float;
    b[1 as libc::c_int as usize] =
        t * (farclip >> 1 as libc::c_int) as libc::c_float;
    b[2 as libc::c_int as usize] = (farclip >> 1 as libc::c_int) as vec_t;
    j = 0 as libc::c_int;
    while j < 3 as libc::c_int {
        k = st_to_vec[axis as usize][j as usize];
        v[j as usize] =
            if k < 0 as libc::c_int {
                -b[(-k - 1 as libc::c_int) as usize]
            } else { b[(k - 1 as libc::c_int) as usize] };
        v[j as usize] += RI.cullorigin[j as usize];
        j += 1
    }
    // avoid bilerp seam
    s = (s + 1.0f32) * 0.5f32;
    t = (t + 1.0f32) * 0.5f32;
    if s < 1.0f32 / 512.0f32 {
        s = 1.0f32 / 512.0f32
    } else if s > 511.0f32 / 512.0f32 { s = 511.0f32 / 512.0f32 }
    if t < 1.0f32 / 512.0f32 {
        t = 1.0f32 / 512.0f32
    } else if t > 511.0f32 / 512.0f32 { t = 511.0f32 / 512.0f32 }
    t = 1.0f32 - t;
    pglTexCoord2f.expect("non-null function pointer")(s, t);
    pglVertex3fv.expect("non-null function pointer")(v.as_mut_ptr());
}
/*
==============
R_ClearSkyBox
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_ClearSkyBox() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        RI.skyMins[1 as libc::c_int as usize][i as usize] = 9999999.0f32;
        RI.skyMins[0 as libc::c_int as usize][i as usize] =
            RI.skyMins[1 as libc::c_int as usize][i as usize];
        RI.skyMaxs[1 as libc::c_int as usize][i as usize] = -9999999.0f32;
        RI.skyMaxs[0 as libc::c_int as usize][i as usize] =
            RI.skyMaxs[1 as libc::c_int as usize][i as usize];
        i += 1
    };
}
/*
=================
R_AddSkyBoxSurface
=================
*/
#[no_mangle]
pub unsafe extern "C" fn R_AddSkyBoxSurface(mut fa: *mut msurface_t) {
    let mut verts: [vec3_t; 128] = [[0.; 3]; 128];
    let mut p: *mut glpoly_t = 0 as *mut glpoly_t;
    let mut v: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut i: libc::c_int = 0;
    if Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(17
                                                                                                                 as
                                                                                                                 libc::c_int,
                                                                                                             0
                                                                                                                 as
                                                                                                                 libc::c_int)
           != 0 && !(*fa).polys.is_null() && tr.fCustomSkybox as u64 == 0 {
        let mut p_0: *mut glpoly_t = (*fa).polys;
        // draw the sky poly
        pglBegin.expect("non-null function pointer")(0x9 as libc::c_int as
                                                         GLenum);
        i = 0 as libc::c_int;
        v = (*p_0).verts[0 as libc::c_int as usize].as_mut_ptr();
        while i < (*p_0).numverts {
            pglTexCoord2f.expect("non-null function pointer")(*v.offset(3 as
                                                                            libc::c_int
                                                                            as
                                                                            isize),
                                                              *v.offset(4 as
                                                                            libc::c_int
                                                                            as
                                                                            isize));
            pglVertex3fv.expect("non-null function pointer")(v);
            i += 1;
            v = v.offset(7 as libc::c_int as isize)
        }
        pglEnd.expect("non-null function pointer")();
    }
    // calculate vertex values for sky box
    p = (*fa).polys;
    while !p.is_null() {
        i = 0 as libc::c_int;
        while i < (*p).numverts {
            verts[i as usize][0 as libc::c_int as usize] =
                (*p).verts[i as usize][0 as libc::c_int as usize] -
                    RI.cullorigin[0 as libc::c_int as usize];
            verts[i as usize][1 as libc::c_int as usize] =
                (*p).verts[i as usize][1 as libc::c_int as usize] -
                    RI.cullorigin[1 as libc::c_int as usize];
            verts[i as usize][2 as libc::c_int as usize] =
                (*p).verts[i as usize][2 as libc::c_int as usize] -
                    RI.cullorigin[2 as libc::c_int as usize];
            i += 1
        }
        ClipSkyPolygon((*p).numverts,
                       verts[0 as libc::c_int as usize].as_mut_ptr(),
                       0 as libc::c_int);
        p = (*p).next
    };
}
/*
==============
R_UnloadSkybox

Unload previous skybox
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_UnloadSkybox() {
    let mut i: libc::c_int = 0;
    // release old skybox
    i =
        0 as
            libc::c_int; // set skybox base (to let some mods load hi-res skyboxes)
    while i < 6 as libc::c_int {
        if !(tr.skyboxTextures[i as usize] == 0) {
            GL_FreeTexture(tr.skyboxTextures[i as usize] as GLenum);
        }
        i += 1
    }
    tr.skyboxbasenum = 5800 as libc::c_int;
    memset(tr.skyboxTextures.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<[libc::c_int; 6]>() as libc::c_ulong);
    tr.fCustomSkybox = false_0;
}
/*
==============
R_DrawSkybox
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_DrawSkyBox() {
    let mut i: libc::c_int = 0;
    RI.isSkyVisible = true_0;
    // don't fogging skybox (this fix old Half-Life bug)
    if RI.fogSkybox as u64 == 0 {
        R_AllowFog(false_0); // stub
    }
    if RI.fogEnabled as u64 != 0 {
        pglFogf.expect("non-null function pointer")(0xb62 as libc::c_int as
                                                        GLenum,
                                                    RI.fogDensity * 0.5f32);
    }
    pglDisable.expect("non-null function pointer")(0xbe2 as libc::c_int as
                                                       GLenum);
    pglDisable.expect("non-null function pointer")(0xbc0 as libc::c_int as
                                                       GLenum);
    pglTexEnvi.expect("non-null function pointer")(0x2300 as libc::c_int as
                                                       GLenum,
                                                   0x2200 as libc::c_int as
                                                       GLenum,
                                                   0x1e01 as libc::c_int);
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        if !(RI.skyMins[0 as libc::c_int as usize][i as usize] >=
                 RI.skyMaxs[0 as libc::c_int as usize][i as usize] ||
                 RI.skyMins[1 as libc::c_int as usize][i as usize] >=
                     RI.skyMaxs[1 as libc::c_int as usize][i as usize]) {
            if tr.skyboxTextures[r_skyTexOrder[i as usize] as usize] != 0 {
                GL_Bind(XASH_TEXTURE0 as libc::c_int,
                        tr.skyboxTextures[r_skyTexOrder[i as usize] as usize]
                            as GLenum);
            } else {
                GL_Bind(XASH_TEXTURE0 as libc::c_int,
                        tr.grayTexture as GLenum);
            }
            pglBegin.expect("non-null function pointer")(0x7 as libc::c_int as
                                                             GLenum);
            MakeSkyVec(RI.skyMins[0 as libc::c_int as usize][i as usize],
                       RI.skyMins[1 as libc::c_int as usize][i as usize], i);
            MakeSkyVec(RI.skyMins[0 as libc::c_int as usize][i as usize],
                       RI.skyMaxs[1 as libc::c_int as usize][i as usize], i);
            MakeSkyVec(RI.skyMaxs[0 as libc::c_int as usize][i as usize],
                       RI.skyMaxs[1 as libc::c_int as usize][i as usize], i);
            MakeSkyVec(RI.skyMaxs[0 as libc::c_int as usize][i as usize],
                       RI.skyMins[1 as libc::c_int as usize][i as usize], i);
            pglEnd.expect("non-null function pointer")();
        }
        i += 1
    }
    if RI.fogSkybox as u64 == 0 { R_AllowFog(true_0); }
    if RI.fogEnabled as u64 != 0 {
        pglFogf.expect("non-null function pointer")(0xb62 as libc::c_int as
                                                        GLenum,
                                                    RI.fogDensity);
    }
    R_LoadIdentity();
}
/*
===============
R_SetupSky
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_SetupSky(mut skyboxname: *const libc::c_char) {
    let mut loadname: [libc::c_char; 256] = [0; 256];
    let mut sidename: [libc::c_char; 256] = [0; 256];
    let mut i: libc::c_int = 0;
    let mut result: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    if if skyboxname.is_null() || *skyboxname == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        R_UnloadSkybox();
        return
        // clear old skybox
    }
    Q_snprintf(loadname.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
               b"gfx/env/%s\x00" as *const u8 as *const libc::c_char,
               skyboxname);
    COM_StripExtension(loadname.as_mut_ptr());
    // kill the underline suffix to find them manually later
    len = Q_strlen(loadname.as_mut_ptr()) as libc::c_int;
    if loadname[(len - 1 as libc::c_int) as usize] as libc::c_int ==
           '_' as i32 {
        loadname[(len - 1 as libc::c_int) as usize] =
            '\u{0}' as i32 as libc::c_char
    }
    result = CheckSkybox(loadname.as_mut_ptr());
    // to prevent infinite recursion if default skybox was missed
    if result == 0 as libc::c_int &&
           Q_strnicmp(loadname.as_mut_ptr(),
                      b"gfx/env/desert\x00" as *const u8 as
                          *const libc::c_char, 99999 as libc::c_int) != 0 {
        gEngfuncs.Con_Reportf.expect("non-null function pointer")(b"^3Warning:^7 missed or incomplete skybox \'%s\'\n\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char,
                                                                  skyboxname); // force to default
        R_SetupSky(b"desert\x00" as *const u8 as *const libc::c_char);
        return
    }
    // release old skybox
    R_UnloadSkybox();
    gEngfuncs.Con_DPrintf.expect("non-null function pointer")(b"SKY:  \x00" as
                                                                  *const u8 as
                                                                  *const libc::c_char);
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        if result == 1 as libc::c_int {
            Q_snprintf(sidename.as_mut_ptr(),
                       ::std::mem::size_of::<[libc::c_char; 256]>() as
                           libc::c_ulong,
                       b"%s%s\x00" as *const u8 as *const libc::c_char,
                       loadname.as_mut_ptr(), r_skyBoxSuffix[i as usize]);
        } else {
            Q_snprintf(sidename.as_mut_ptr(),
                       ::std::mem::size_of::<[libc::c_char; 256]>() as
                           libc::c_ulong,
                       b"%s_%s\x00" as *const u8 as *const libc::c_char,
                       loadname.as_mut_ptr(), r_skyBoxSuffix[i as usize]);
        }
        tr.skyboxTextures[i as usize] =
            GL_LoadTexture(sidename.as_mut_ptr(), 0 as *const byte,
                           0 as libc::c_int as size_t,
                           TF_CLAMP as libc::c_int |
                               (TF_SKYSIDE as libc::c_int |
                                    TF_NOMIPMAP as libc::c_int));
        if tr.skyboxTextures[i as usize] == 0 { break ; }
        gEngfuncs.Con_DPrintf.expect("non-null function pointer")(b"%s%s%s\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char,
                                                                  skyboxname,
                                                                  r_skyBoxSuffix[i
                                                                                     as
                                                                                     usize],
                                                                  if i !=
                                                                         5 as
                                                                             libc::c_int
                                                                     {
                                                                      b", \x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char
                                                                  } else {
                                                                      b". \x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char
                                                                  });
        i += 1
    }
    if i == 6 as libc::c_int {
        tr.fCustomSkybox = true_0;
        gEngfuncs.Con_DPrintf.expect("non-null function pointer")(b"done\n\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char);
        return
        // loaded
    }
    gEngfuncs.Con_DPrintf.expect("non-null function pointer")(b"^2failed\n\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char);
    R_UnloadSkybox();
}
//==============================================================================
//
//  RENDER CLOUDS
//
//==============================================================================
/*
==============
R_CloudVertex
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_CloudVertex(mut s: libc::c_float,
                                       mut t: libc::c_float,
                                       mut axis: libc::c_int,
                                       mut v: *mut vec_t) {
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut farclip: libc::c_int = 0;
    let mut b: vec3_t = [0.; 3];
    farclip = RI.farClip as libc::c_int;
    b[0 as libc::c_int as usize] =
        s * (farclip >> 1 as libc::c_int) as libc::c_float;
    b[1 as libc::c_int as usize] =
        t * (farclip >> 1 as libc::c_int) as libc::c_float;
    b[2 as libc::c_int as usize] = (farclip >> 1 as libc::c_int) as vec_t;
    j = 0 as libc::c_int;
    while j < 3 as libc::c_int {
        k = st_to_vec[axis as usize][j as usize];
        *v.offset(j as isize) =
            if k < 0 as libc::c_int {
                -b[(-k - 1 as libc::c_int) as usize]
            } else { b[(k - 1 as libc::c_int) as usize] };
        let ref mut fresh0 = *v.offset(j as isize);
        *fresh0 += RI.cullorigin[j as usize];
        j += 1
    };
}
/*
=============
R_CloudTexCoord
=============
*/
#[no_mangle]
pub unsafe extern "C" fn R_CloudTexCoord(mut v: *mut vec_t,
                                         mut speed: libc::c_float,
                                         mut s: *mut libc::c_float,
                                         mut t: *mut libc::c_float) {
    let mut length: libc::c_float = 0.; // flatten the sphere
    let mut speedscale: libc::c_float = 0.;
    let mut dir: vec3_t = [0.; 3];
    speedscale = (*gpGlobals).time * speed;
    speedscale -=
        (speedscale as libc::c_int & !(127 as libc::c_int)) as libc::c_float;
    dir[0 as libc::c_int as usize] =
        *v.offset(0 as libc::c_int as isize) -
            RI.vieworg[0 as libc::c_int as usize];
    dir[1 as libc::c_int as usize] =
        *v.offset(1 as libc::c_int as isize) -
            RI.vieworg[1 as libc::c_int as usize];
    dir[2 as libc::c_int as usize] =
        *v.offset(2 as libc::c_int as isize) -
            RI.vieworg[2 as libc::c_int as usize];
    dir[2 as libc::c_int as usize] *= 3.0f32;
    length =
        __tg_sqrt(dir[0 as libc::c_int as usize] *
                      dir[0 as libc::c_int as usize] +
                      dir[1 as libc::c_int as usize] *
                          dir[1 as libc::c_int as usize] +
                      dir[2 as libc::c_int as usize] *
                          dir[2 as libc::c_int as usize]);
    length = 6.0f32 * 63.0f32 / length;
    *s =
        (speedscale + dir[0 as libc::c_int as usize] * length) *
            (1.0f32 / 128.0f32);
    *t =
        (speedscale + dir[1 as libc::c_int as usize] * length) *
            (1.0f32 / 128.0f32);
}
/*
===============
R_CloudDrawPoly
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_CloudDrawPoly(mut p: *mut glpoly_t) {
    let mut s: libc::c_float = 0.;
    let mut t: libc::c_float = 0.;
    let mut v: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut i: libc::c_int = 0;
    GL_SetRenderMode(kRenderNormal as libc::c_int);
    GL_Bind(XASH_TEXTURE0 as libc::c_int, tr.solidskyTexture as GLenum);
    pglBegin.expect("non-null function pointer")(0x7 as libc::c_int as
                                                     GLenum);
    i = 0 as libc::c_int;
    v = (*p).verts[0 as libc::c_int as usize].as_mut_ptr();
    while i < 4 as libc::c_int {
        R_CloudTexCoord(v, 8.0f32, &mut s, &mut t);
        pglTexCoord2f.expect("non-null function pointer")(s, t);
        pglVertex3fv.expect("non-null function pointer")(v);
        i += 1;
        v = v.offset(7 as libc::c_int as isize)
    }
    pglEnd.expect("non-null function pointer")();
    GL_SetRenderMode(kRenderTransTexture as libc::c_int);
    GL_Bind(XASH_TEXTURE0 as libc::c_int, tr.alphaskyTexture as GLenum);
    pglBegin.expect("non-null function pointer")(0x7 as libc::c_int as
                                                     GLenum);
    i = 0 as libc::c_int;
    v = (*p).verts[0 as libc::c_int as usize].as_mut_ptr();
    while i < 4 as libc::c_int {
        R_CloudTexCoord(v, 16.0f32, &mut s, &mut t);
        pglTexCoord2f.expect("non-null function pointer")(s, t);
        pglVertex3fv.expect("non-null function pointer")(v);
        i += 1;
        v = v.offset(7 as libc::c_int as isize)
    }
    pglEnd.expect("non-null function pointer")();
    pglDisable.expect("non-null function pointer")(0xbe2 as libc::c_int as
                                                       GLenum);
}
/*
==============
R_CloudRenderSide
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_CloudRenderSide(mut axis: libc::c_int) {
    let mut verts: [vec3_t; 4] =
        [[0.; 3];
            4]; //subdivide vertically more than horizontally on skybox sides
    let mut di: libc::c_float = 0.;
    let mut qi: libc::c_float = 0.;
    let mut dj: libc::c_float = 0.;
    let mut qj: libc::c_float = 0.;
    let mut vup: vec3_t = [0.; 3];
    let mut vright: vec3_t = [0.; 3];
    let mut temp: vec3_t = [0.; 3];
    let mut temp2: vec3_t = [0.; 3];
    let mut p: [glpoly_t; 1] =
        [glpoly_t{next: 0 as *mut glpoly_s,
                  chain: 0 as *mut glpoly_s,
                  numverts: 0,
                  flags: 0,
                  verts: [[0.; 7]; 4],}; 1];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    R_CloudVertex(-1.0f32, -1.0f32, axis,
                  verts[0 as libc::c_int as usize].as_mut_ptr());
    R_CloudVertex(-1.0f32, 1.0f32, axis,
                  verts[1 as libc::c_int as usize].as_mut_ptr());
    R_CloudVertex(1.0f32, 1.0f32, axis,
                  verts[2 as libc::c_int as usize].as_mut_ptr());
    R_CloudVertex(1.0f32, -1.0f32, axis,
                  verts[3 as libc::c_int as usize].as_mut_ptr());
    vup[0 as libc::c_int as usize] =
        verts[2 as libc::c_int as usize][0 as libc::c_int as usize] -
            verts[3 as libc::c_int as usize][0 as libc::c_int as usize];
    vup[1 as libc::c_int as usize] =
        verts[2 as libc::c_int as usize][1 as libc::c_int as usize] -
            verts[3 as libc::c_int as usize][1 as libc::c_int as usize];
    vup[2 as libc::c_int as usize] =
        verts[2 as libc::c_int as usize][2 as libc::c_int as usize] -
            verts[3 as libc::c_int as usize][2 as libc::c_int as usize];
    vright[0 as libc::c_int as usize] =
        verts[2 as libc::c_int as usize][0 as libc::c_int as usize] -
            verts[1 as libc::c_int as usize][0 as libc::c_int as usize];
    vright[1 as libc::c_int as usize] =
        verts[2 as libc::c_int as usize][1 as libc::c_int as usize] -
            verts[1 as libc::c_int as usize][1 as libc::c_int as usize];
    vright[2 as libc::c_int as usize] =
        verts[2 as libc::c_int as usize][2 as libc::c_int as usize] -
            verts[1 as libc::c_int as usize][2 as libc::c_int as usize];
    (*p.as_mut_ptr()).numverts = 4 as libc::c_int;
    di = 12 as libc::c_int as libc::c_float;
    qi = 1.0f32 / di;
    dj =
        if axis < 4 as libc::c_int {
            (di) * 2 as libc::c_int as libc::c_float
        } else { di };
    qj = 1.0f32 / dj;
    i = 0 as libc::c_int;
    while (i as libc::c_float) < di {
        j = 0 as libc::c_int;
        while (j as libc::c_float) < dj {
            if !(i as libc::c_float * qi <
                     RI.skyMins[0 as libc::c_int as usize][axis as usize] /
                         2 as libc::c_int as libc::c_float + 0.5f32 - qi ||
                     i as libc::c_float * qi >
                         RI.skyMaxs[0 as libc::c_int as usize][axis as usize]
                             / 2 as libc::c_int as libc::c_float + 0.5f32 ||
                     j as libc::c_float * qj <
                         RI.skyMins[1 as libc::c_int as usize][axis as usize]
                             / 2 as libc::c_int as libc::c_float + 0.5f32 - qj
                     ||
                     j as libc::c_float * qj >
                         RI.skyMaxs[1 as libc::c_int as usize][axis as usize]
                             / 2 as libc::c_int as libc::c_float + 0.5f32) {
                temp[0 as libc::c_int as usize] =
                    vright[0 as libc::c_int as usize] *
                        (qi * i as libc::c_float);
                temp[1 as libc::c_int as usize] =
                    vright[1 as libc::c_int as usize] *
                        (qi * i as libc::c_float);
                temp[2 as libc::c_int as usize] =
                    vright[2 as libc::c_int as usize] *
                        (qi * i as libc::c_float);
                temp2[0 as libc::c_int as usize] =
                    vup[0 as libc::c_int as usize] *
                        (qj * j as libc::c_float);
                temp2[1 as libc::c_int as usize] =
                    vup[1 as libc::c_int as usize] *
                        (qj * j as libc::c_float);
                temp2[2 as libc::c_int as usize] =
                    vup[2 as libc::c_int as usize] *
                        (qj * j as libc::c_float);
                temp[0 as libc::c_int as usize] =
                    temp[0 as libc::c_int as usize] +
                        temp2[0 as libc::c_int as usize];
                temp[1 as libc::c_int as usize] =
                    temp[1 as libc::c_int as usize] +
                        temp2[1 as libc::c_int as usize];
                temp[2 as libc::c_int as usize] =
                    temp[2 as libc::c_int as usize] +
                        temp2[2 as libc::c_int as usize];
                (*p.as_mut_ptr()).verts[0 as libc::c_int as
                                            usize][0 as libc::c_int as usize]
                    =
                    verts[0 as libc::c_int as
                              usize][0 as libc::c_int as usize] +
                        temp[0 as libc::c_int as usize];
                (*p.as_mut_ptr()).verts[0 as libc::c_int as
                                            usize][1 as libc::c_int as usize]
                    =
                    verts[0 as libc::c_int as
                              usize][1 as libc::c_int as usize] +
                        temp[1 as libc::c_int as usize];
                (*p.as_mut_ptr()).verts[0 as libc::c_int as
                                            usize][2 as libc::c_int as usize]
                    =
                    verts[0 as libc::c_int as
                              usize][2 as libc::c_int as usize] +
                        temp[2 as libc::c_int as usize];
                temp[0 as libc::c_int as usize] =
                    vup[0 as libc::c_int as usize] * qj;
                temp[1 as libc::c_int as usize] =
                    vup[1 as libc::c_int as usize] * qj;
                temp[2 as libc::c_int as usize] =
                    vup[2 as libc::c_int as usize] * qj;
                (*p.as_mut_ptr()).verts[1 as libc::c_int as
                                            usize][0 as libc::c_int as usize]
                    =
                    (*p.as_mut_ptr()).verts[0 as libc::c_int as
                                                usize][0 as libc::c_int as
                                                           usize] +
                        temp[0 as libc::c_int as usize];
                (*p.as_mut_ptr()).verts[1 as libc::c_int as
                                            usize][1 as libc::c_int as usize]
                    =
                    (*p.as_mut_ptr()).verts[0 as libc::c_int as
                                                usize][1 as libc::c_int as
                                                           usize] +
                        temp[1 as libc::c_int as usize];
                (*p.as_mut_ptr()).verts[1 as libc::c_int as
                                            usize][2 as libc::c_int as usize]
                    =
                    (*p.as_mut_ptr()).verts[0 as libc::c_int as
                                                usize][2 as libc::c_int as
                                                           usize] +
                        temp[2 as libc::c_int as usize];
                temp[0 as libc::c_int as usize] =
                    vright[0 as libc::c_int as usize] * qi;
                temp[1 as libc::c_int as usize] =
                    vright[1 as libc::c_int as usize] * qi;
                temp[2 as libc::c_int as usize] =
                    vright[2 as libc::c_int as usize] * qi;
                (*p.as_mut_ptr()).verts[2 as libc::c_int as
                                            usize][0 as libc::c_int as usize]
                    =
                    (*p.as_mut_ptr()).verts[1 as libc::c_int as
                                                usize][0 as libc::c_int as
                                                           usize] +
                        temp[0 as libc::c_int as usize];
                (*p.as_mut_ptr()).verts[2 as libc::c_int as
                                            usize][1 as libc::c_int as usize]
                    =
                    (*p.as_mut_ptr()).verts[1 as libc::c_int as
                                                usize][1 as libc::c_int as
                                                           usize] +
                        temp[1 as libc::c_int as usize];
                (*p.as_mut_ptr()).verts[2 as libc::c_int as
                                            usize][2 as libc::c_int as usize]
                    =
                    (*p.as_mut_ptr()).verts[1 as libc::c_int as
                                                usize][2 as libc::c_int as
                                                           usize] +
                        temp[2 as libc::c_int as usize];
                (*p.as_mut_ptr()).verts[3 as libc::c_int as
                                            usize][0 as libc::c_int as usize]
                    =
                    (*p.as_mut_ptr()).verts[0 as libc::c_int as
                                                usize][0 as libc::c_int as
                                                           usize] +
                        temp[0 as libc::c_int as usize];
                (*p.as_mut_ptr()).verts[3 as libc::c_int as
                                            usize][1 as libc::c_int as usize]
                    =
                    (*p.as_mut_ptr()).verts[0 as libc::c_int as
                                                usize][1 as libc::c_int as
                                                           usize] +
                        temp[1 as libc::c_int as usize];
                (*p.as_mut_ptr()).verts[3 as libc::c_int as
                                            usize][2 as libc::c_int as usize]
                    =
                    (*p.as_mut_ptr()).verts[0 as libc::c_int as
                                                usize][2 as libc::c_int as
                                                           usize] +
                        temp[2 as libc::c_int as usize];
                R_CloudDrawPoly(p.as_mut_ptr());
            }
            j += 1
        }
        i += 1
    };
}
/*
==============
R_DrawClouds

Quake-style clouds
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_DrawClouds() {
    let mut i: libc::c_int = 0;
    RI.isSkyVisible = true_0;
    if RI.fogEnabled as u64 != 0 {
        pglFogf.expect("non-null function pointer")(0xb62 as libc::c_int as
                                                        GLenum,
                                                    RI.fogDensity * 0.25f32);
    }
    pglDepthFunc.expect("non-null function pointer")(0x206 as libc::c_int as
                                                         GLenum);
    pglDepthMask.expect("non-null function pointer")(0 as libc::c_int as
                                                         GLboolean);
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        if !(RI.skyMins[0 as libc::c_int as usize][i as usize] >=
                 RI.skyMaxs[0 as libc::c_int as usize][i as usize] ||
                 RI.skyMins[1 as libc::c_int as usize][i as usize] >=
                     RI.skyMaxs[1 as libc::c_int as usize][i as usize]) {
            R_CloudRenderSide(i);
        }
        i += 1
    }
    pglDepthFunc.expect("non-null function pointer")(0x203 as libc::c_int as
                                                         GLenum);
    pglDepthMask.expect("non-null function pointer")(0x1 as libc::c_int as
                                                         GLboolean);
    if RI.fogEnabled as u64 != 0 {
        pglFogf.expect("non-null function pointer")(0xb62 as libc::c_int as
                                                        GLenum,
                                                    RI.fogDensity);
    };
}
/*
=============
R_InitSkyClouds

A sky texture is 256*128, with the right side being a masked overlay
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_InitSkyClouds(mut mt: *mut mip_t,
                                         mut tx: *mut texture_t,
                                         mut custom_palette: qboolean) {
    let mut r_temp: rgbdata_t =
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
    let mut r_sky: *mut rgbdata_t = 0 as *mut rgbdata_t;
    let mut trans: *mut uint = 0 as *mut uint;
    let mut rgba: *mut uint = 0 as *mut uint;
    let mut transpix: uint = 0;
    let mut r: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut texname: [libc::c_char; 32] = [0; 32];
    if glw_state.initialized as u64 == 0 { return }
    Q_snprintf(texname.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
               b"%s%s.mip\x00" as *const u8 as *const libc::c_char,
               if (*mt).offsets[0 as libc::c_int as usize] >
                      0 as libc::c_int as libc::c_uint {
                   b"#\x00" as *const u8 as *const libc::c_char
               } else { b"\x00" as *const u8 as *const libc::c_char },
               (*tx).name.as_mut_ptr());
    if (*mt).offsets[0 as libc::c_int as usize] >
           0 as libc::c_int as libc::c_uint {
        let mut size: libc::c_int =
            (::std::mem::size_of::<mip_t>() as libc::c_ulong as libc::c_int as
                 libc::c_uint).wrapping_add((*mt).width.wrapping_mul((*mt).height).wrapping_mul(85
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_uint)
                                                >> 6 as libc::c_int) as
                libc::c_int;
        if custom_palette as u64 != 0 {
            size =
                (size as
                     libc::c_ulong).wrapping_add((::std::mem::size_of::<libc::c_short>()
                                                      as
                                                      libc::c_ulong).wrapping_add(768
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_ulong))
                    as libc::c_int as libc::c_int
        }
        r_sky =
            gEngfuncs.FS_LoadImage.expect("non-null function pointer")(texname.as_mut_ptr(),
                                                                       mt as
                                                                           *mut byte,
                                                                       size as
                                                                           size_t)
    } else {
        // okay, loading it from wad
        r_sky =
            gEngfuncs.FS_LoadImage.expect("non-null function pointer")(texname.as_mut_ptr(),
                                                                       0 as
                                                                           *const byte,
                                                                       0 as
                                                                           libc::c_int
                                                                           as
                                                                           size_t)
    }
    // make sure what sky image is valid
    if r_sky.is_null() || (*r_sky).palette.is_null() ||
           (*r_sky).type_0 != PF_INDEXED_32 as libc::c_int as libc::c_uint ||
           (*r_sky).height as libc::c_int == 0 as libc::c_int {
        gEngfuncs.Con_Reportf.expect("non-null function pointer")(b"^1Error:^7 R_InitSky: unable to load sky texture %s\n\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char,
                                                                  (*tx).name.as_mut_ptr());
        if !r_sky.is_null() {
            gEngfuncs.FS_FreeImage.expect("non-null function pointer")(r_sky);
        }
        return
    }
    // make an average value for the back to avoid
	// a fringe on the top level
    trans =
        gEngfuncs._Mem_Alloc.expect("non-null function pointer")(r_temppool,
                                                                 (((*r_sky).height
                                                                       as
                                                                       libc::c_int
                                                                       *
                                                                       (*r_sky).height
                                                                           as
                                                                           libc::c_int)
                                                                      as
                                                                      libc::c_ulong).wrapping_mul(::std::mem::size_of::<uint>()
                                                                                                      as
                                                                                                      libc::c_ulong),
                                                                 false_0,
                                                                 b"../ref_gl/gl_warp.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 689 as
                                                                     libc::c_int)
            as *mut uint;
    b = 0 as libc::c_int;
    g = b;
    r = g;
    i = 0 as libc::c_int;
    while i < (*r_sky).width as libc::c_int >> 1 as libc::c_int {
        j = 0 as libc::c_int;
        while j < (*r_sky).height as libc::c_int {
            p =
                *(*r_sky).buffer.offset((i * (*r_sky).width as libc::c_int + j
                                             + (*r_sky).height as libc::c_int)
                                            as isize) as libc::c_int;
            rgba = ((*r_sky).palette as *mut uint).offset(p as isize);
            *trans.offset((i * (*r_sky).height as libc::c_int + j) as isize) =
                *rgba;
            r +=
                *(rgba as *mut byte).offset(0 as libc::c_int as isize) as
                    libc::c_int;
            g +=
                *(rgba as *mut byte).offset(1 as libc::c_int as isize) as
                    libc::c_int;
            b +=
                *(rgba as *mut byte).offset(2 as libc::c_int as isize) as
                    libc::c_int;
            j += 1
        }
        i += 1
    }
    *(&mut transpix as *mut uint as
          *mut byte).offset(0 as libc::c_int as isize) =
        (r /
             ((*r_sky).height as libc::c_int *
                  (*r_sky).height as libc::c_int)) as byte;
    *(&mut transpix as *mut uint as
          *mut byte).offset(1 as libc::c_int as isize) =
        (g /
             ((*r_sky).height as libc::c_int *
                  (*r_sky).height as libc::c_int)) as byte;
    *(&mut transpix as *mut uint as
          *mut byte).offset(2 as libc::c_int as isize) =
        (b /
             ((*r_sky).height as libc::c_int *
                  (*r_sky).height as libc::c_int)) as byte;
    *(&mut transpix as *mut uint as
          *mut byte).offset(3 as libc::c_int as isize) =
        0 as libc::c_int as byte;
    // build a temporary image
    r_temp = *r_sky;
    r_temp.width =
        ((*r_sky).width as libc::c_int >> 1 as libc::c_int) as word;
    r_temp.height = (*r_sky).height;
    r_temp.type_0 = PF_RGBA_32 as libc::c_int as uint;
    r_temp.flags = IMAGE_HAS_COLOR as libc::c_int as uint;
    r_temp.size =
        (r_temp.width as libc::c_int * r_temp.height as libc::c_int *
             4 as libc::c_int) as size_t;
    r_temp.buffer = trans as *mut byte;
    r_temp.palette = 0 as *mut byte;
    // load it in
    tr.solidskyTexture =
        GL_LoadTextureFromBuffer(b"solid_sky\x00" as *const u8 as
                                     *const libc::c_char, &mut r_temp,
                                 TF_NOMIPMAP, false_0);
    i = 0 as libc::c_int;
    while i < (*r_sky).width as libc::c_int >> 1 as libc::c_int {
        j = 0 as libc::c_int;
        while j < (*r_sky).height as libc::c_int {
            p =
                *(*r_sky).buffer.offset((i * (*r_sky).width as libc::c_int +
                                             j) as isize) as libc::c_int;
            if p == 0 as libc::c_int {
                *trans.offset((i * (*r_sky).height as libc::c_int + j) as
                                  isize) = transpix
            } else {
                rgba = ((*r_sky).palette as *mut uint).offset(p as isize);
                *trans.offset((i * (*r_sky).height as libc::c_int + j) as
                                  isize) = *rgba
            }
            j += 1
        }
        i += 1
    }
    r_temp.flags =
        (IMAGE_HAS_COLOR as libc::c_int | IMAGE_HAS_ALPHA as libc::c_int) as
            uint;
    // load it in
    tr.alphaskyTexture =
        GL_LoadTextureFromBuffer(b"alpha_sky\x00" as *const u8 as
                                     *const libc::c_char, &mut r_temp,
                                 TF_NOMIPMAP, false_0);
    // clean up
    gEngfuncs.FS_FreeImage.expect("non-null function pointer")(r_sky);
    gEngfuncs._Mem_Free.expect("non-null function pointer")(trans as
                                                                *mut libc::c_void,
                                                            b"../ref_gl/gl_warp.c\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char,
                                                            748 as
                                                                libc::c_int);
}
/*
=============
EmitWaterPolys

Does a water warp on the pre-fragmented glpoly_t chain
=============
*/
#[no_mangle]
pub unsafe extern "C" fn EmitWaterPolys(mut warp: *mut msurface_t,
                                        mut reverse: qboolean) {
    let mut v: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut nv: libc::c_float = 0.;
    let mut waveHeight: libc::c_float = 0.;
    let mut s: libc::c_float = 0.;
    let mut t: libc::c_float = 0.;
    let mut os: libc::c_float = 0.;
    let mut ot: libc::c_float = 0.;
    let mut p: *mut glpoly_t = 0 as *mut glpoly_t;
    let mut i: libc::c_int = 0;
    let useQuads: qboolean =
        ((*warp).flags as libc::c_uint &
             (1 as libc::c_uint) << 3 as libc::c_int) as qboolean;
    if (*warp).polys.is_null() { return }
    // set the current waveheight
    if (*(*warp).polys).verts[0 as libc::c_int as
                                  usize][2 as libc::c_int as usize] >=
           RI.vieworg[2 as libc::c_int as usize] {
        waveHeight = -(*RI.currententity).curstate.scale
    } else { waveHeight = (*RI.currententity).curstate.scale }
    // reset fog color for nonlightmapped water
    GL_ResetFogColor();
    if useQuads as u64 != 0 {
        pglBegin.expect("non-null function pointer")(0x7 as libc::c_int as
                                                         GLenum);
    }
    p = (*warp).polys;
    while !p.is_null() {
        if reverse as u64 != 0 {
            v =
                (*p).verts[0 as libc::c_int as
                               usize].as_mut_ptr().offset((((*p).numverts -
                                                                1 as
                                                                    libc::c_int)
                                                               *
                                                               7 as
                                                                   libc::c_int)
                                                              as isize)
        } else { v = (*p).verts[0 as libc::c_int as usize].as_mut_ptr() }
        if useQuads as u64 == 0 {
            pglBegin.expect("non-null function pointer")(0x9 as libc::c_int as
                                                             GLenum);
        }
        i = 0 as libc::c_int;
        while i < (*p).numverts {
            if waveHeight != 0. {
                nv =
                    r_turbsin[(((*gpGlobals).time * 160.0f32 +
                                    *v.offset(1 as libc::c_int as isize) +
                                    *v.offset(0 as libc::c_int as isize)) as
                                   libc::c_int & 255 as libc::c_int) as usize]
                        + 8.0f32;
                nv =
                    (r_turbsin[((*v.offset(0 as libc::c_int as isize) * 5.0f32
                                     + (*gpGlobals).time * 171.0f32 -
                                     *v.offset(1 as libc::c_int as isize)) as
                                    libc::c_int & 255 as libc::c_int) as
                                   usize] + 8.0f32) * 0.8f32 + nv;
                nv = nv * waveHeight + *v.offset(2 as libc::c_int as isize)
            } else { nv = *v.offset(2 as libc::c_int as isize) }
            os = *v.offset(3 as libc::c_int as isize);
            ot = *v.offset(4 as libc::c_int as isize);
            s =
                os +
                    r_turbsin[(((ot * 0.125f32 + (*gpGlobals).time) *
                                    (256.0f32 /
                                         (3.14159265358979323846f64 *
                                              2 as libc::c_int as
                                                  libc::c_double) as
                                             libc::c_float)) as libc::c_int &
                                   255 as libc::c_int) as usize];
            s *= 1.0f32 / 64 as libc::c_int as libc::c_float;
            t =
                ot +
                    r_turbsin[(((os * 0.125f32 + (*gpGlobals).time) *
                                    (256.0f32 /
                                         (3.14159265358979323846f64 *
                                              2 as libc::c_int as
                                                  libc::c_double) as
                                             libc::c_float)) as libc::c_int &
                                   255 as libc::c_int) as usize];
            t *= 1.0f32 / 64 as libc::c_int as libc::c_float;
            pglTexCoord2f.expect("non-null function pointer")(s, t);
            pglVertex3f.expect("non-null function pointer")(*v.offset(0 as
                                                                          libc::c_int
                                                                          as
                                                                          isize),
                                                            *v.offset(1 as
                                                                          libc::c_int
                                                                          as
                                                                          isize),
                                                            nv);
            if reverse as u64 != 0 {
                v = v.offset(-(7 as libc::c_int as isize))
            } else { v = v.offset(7 as libc::c_int as isize) }
            i += 1
        }
        if useQuads as u64 == 0 {
            pglEnd.expect("non-null function pointer")();
        }
        p = (*p).next
    }
    if useQuads as u64 != 0 { pglEnd.expect("non-null function pointer")(); }
    GL_SetupFogColorForSurfaces();
}
