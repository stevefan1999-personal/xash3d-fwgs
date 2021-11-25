#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, const_raw_ptr_to_usize_cast, extern_types,
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
    static mut pglBegin: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglBlendFunc:
           Option<unsafe extern "C" fn(_: GLenum, _: GLenum) -> ()>;
    #[no_mangle]
    static mut pglClear: Option<unsafe extern "C" fn(_: GLbitfield) -> ()>;
    #[no_mangle]
    static mut pglIsTexture:
           Option<unsafe extern "C" fn(_: GLuint) -> GLboolean>;
    #[no_mangle]
    static mut pglColor4f:
           Option<unsafe extern "C" fn(_: GLfloat, _: GLfloat, _: GLfloat,
                                       _: GLfloat) -> ()>;
    #[no_mangle]
    static mut pglCullFace: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglDisable: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglDisableClientState:
           Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglDrawBuffer: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglEnable: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglEnableClientState:
           Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglEnd: Option<unsafe extern "C" fn() -> ()>;
    #[no_mangle]
    static mut pglFinish: Option<unsafe extern "C" fn() -> ()>;
    #[no_mangle]
    static mut pglLoadIdentity: Option<unsafe extern "C" fn() -> ()>;
    #[no_mangle]
    static mut pglLoadMatrixf:
           Option<unsafe extern "C" fn(_: *const GLfloat) -> ()>;
    #[no_mangle]
    static mut pglMatrixMode: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglReadPixels:
           Option<unsafe extern "C" fn(_: GLint, _: GLint, _: GLsizei,
                                       _: GLsizei, _: GLenum, _: GLenum,
                                       _: *mut libc::c_void) -> ()>;
    #[no_mangle]
    static mut pglTexCoord2f:
           Option<unsafe extern "C" fn(_: GLfloat, _: GLfloat) -> ()>;
    #[no_mangle]
    static mut pglTexEnvi:
           Option<unsafe extern "C" fn(_: GLenum, _: GLenum, _: GLint) -> ()>;
    #[no_mangle]
    static mut pglTexGeni:
           Option<unsafe extern "C" fn(_: GLenum, _: GLenum, _: GLint) -> ()>;
    #[no_mangle]
    static mut pglTexParameteri:
           Option<unsafe extern "C" fn(_: GLenum, _: GLenum, _: GLint) -> ()>;
    #[no_mangle]
    static mut pglVertex2f:
           Option<unsafe extern "C" fn(_: GLfloat, _: GLfloat) -> ()>;
    #[no_mangle]
    static mut pglActiveTextureARB:
           Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglClientActiveTextureARB:
           Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglMultiTexCoord2f:
           Option<unsafe extern "C" fn(_: GLenum, _: GLfloat, _: GLfloat)
                      -> ()>;
    #[no_mangle]
    static mut r_temppool: poolhandle_t;
    #[no_mangle]
    static mut RI: ref_instance_t;
    #[no_mangle]
    static mut tr: gl_globals_t;
    #[no_mangle]
    fn Q_snprintf(buffer: *mut libc::c_char, buffersize: size_t,
                  format: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    static mut gEngfuncs: ref_api_t;
    #[no_mangle]
    fn Q_strncat(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Q_vsnprintf(buffer: *mut libc::c_char, buffersize: size_t,
                   format: *const libc::c_char, args: ::std::ffi::VaList)
     -> libc::c_int;
    #[no_mangle]
    static mut r_speeds: *mut cvar_t;
    #[no_mangle]
    static mut glConfig: glconfig_t;
    #[no_mangle]
    static mut glState: glstate_t;
    #[no_mangle]
    fn GL_MaxTextureUnits() -> libc::c_int;
    #[no_mangle]
    fn GL_Support(r_ext: libc::c_int) -> qboolean;
    #[no_mangle]
    fn GL_Bind(tmu: GLint, texnum: GLenum);
    #[no_mangle]
    fn Matrix4x4_ToArrayFloatGL(in_0: *const [vec_t; 4],
                                out: *mut libc::c_float);
    #[no_mangle]
    static mut glw_state: glwstate_t;
    #[no_mangle]
    fn Q_strlen(string: *const libc::c_char) -> size_t;
    #[no_mangle]
    fn COM_FileBase(in_0: *const libc::c_char, out: *mut libc::c_char);
    #[no_mangle]
    fn R_GetTexture(texnum: GLenum) -> *mut gl_texture_t;
    #[no_mangle]
    static mut gl_showtextures: *mut cvar_t;
    #[no_mangle]
    static mut gpGlobals: *mut ref_globals_t;
    #[no_mangle]
    fn R_EndFrame();
    #[no_mangle]
    fn R_RenderScene();
    #[no_mangle]
    fn R_BeginFrame(clearScene: qboolean);
    #[no_mangle]
    fn R_Set2DMode(enable: qboolean);
    #[no_mangle]
    fn R_DrawCubemapView(origin: *const vec_t, angles: *const vec_t,
                         size: libc::c_int);
    #[no_mangle]
    fn R_CheckGamma();
    #[no_mangle]
    fn COM_DefaultExtension(path: *mut libc::c_char,
                            extension: *const libc::c_char);
    #[no_mangle]
    fn COM_StripExtension(path: *mut libc::c_char);
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
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
pub type string = [libc::c_char; 256];
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gl_frustum_s {
    pub planes: [mplane_t; 6],
    pub clipFlags: libc::c_uint,
}
pub type gl_frustum_t = gl_frustum_s;
pub type va_list = __builtin_va_list;
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
pub type connstate_e = libc::c_uint;
pub const ca_cinematic: connstate_e = 5;
pub const ca_active: connstate_e = 4;
pub const ca_validate: connstate_e = 3;
pub const ca_connected: connstate_e = 2;
pub const ca_connecting: connstate_e = 1;
pub const ca_disconnected: connstate_e = 0;
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
pub type C2RustUnnamed_3 = libc::c_int;
pub const PARM_NUMMODELS: C2RustUnnamed_3 = -13;
pub const PARM_NUMENTITIES: C2RustUnnamed_3 = -12;
pub const PARM_LOCAL_GAME: C2RustUnnamed_3 = -11;
pub const PARM_LOCAL_HEALTH: C2RustUnnamed_3 = -10;
pub const PARM_MAX_CLIENTS: C2RustUnnamed_3 = -9;
pub const PARM_WATER_LEVEL: C2RustUnnamed_3 = -8;
pub const PARM_PLAYING_DEMO: C2RustUnnamed_3 = -7;
pub const PARM_CONNSTATE: C2RustUnnamed_3 = -6;
pub const PARM_VIEWENT_INDEX: C2RustUnnamed_3 = -5;
pub const PARM_PLAYER_INDEX: C2RustUnnamed_3 = -4;
pub const PARM_QUAKE_COMPATIBLE: C2RustUnnamed_3 = -3;
pub const PARM_THIRDPERSON: C2RustUnnamed_3 = -2;
pub const PARM_DEV_OVERVIEW: C2RustUnnamed_3 = -1;
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
pub type GLbitfield = uint;
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
pub struct ref_speeds_t {
    pub c_world_polys: uint,
    pub c_studio_polys: uint,
    pub c_sprite_polys: uint,
    pub c_alias_polys: uint,
    pub c_world_leafs: uint,
    pub c_view_beams_count: uint,
    pub c_active_tents_count: uint,
    pub c_alias_models_drawn: uint,
    pub c_studio_models_drawn: uint,
    pub c_sprite_models_drawn: uint,
    pub c_particle_count: uint,
    pub c_client_ents: uint,
    pub t_world_node: libc::c_double,
    pub t_world_draw: libc::c_double,
}
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
pub const GL_ARB_MULTITEXTURE: C2RustUnnamed_4 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glwstate_t {
    pub initialized: qboolean,
    pub extended: qboolean,
}
/*
==============================================================================

SCREEN SHOTS

==============================================================================
*/
// used for 'env' and 'sky' shots
pub type envmap_t = envmap_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct envmap_s {
    pub angles: vec3_t,
    pub flags: libc::c_int,
}
pub type C2RustUnnamed_4 = libc::c_uint;
pub const GL_EXTCOUNT: C2RustUnnamed_4 = 22;
pub const GL_TEXTURE_MULTISAMPLE: C2RustUnnamed_4 = 21;
pub const GL_DRAW_RANGEELEMENTS_EXT: C2RustUnnamed_4 = 20;
pub const GL_ARB_VERTEX_BUFFER_OBJECT_EXT: C2RustUnnamed_4 = 19;
pub const GL_DEBUG_OUTPUT: C2RustUnnamed_4 = 18;
pub const GL_DEPTH_TEXTURE: C2RustUnnamed_4 = 17;
pub const GL_EXT_GPU_SHADER4: C2RustUnnamed_4 = 16;
pub const GL_ARB_SEAMLESS_CUBEMAP: C2RustUnnamed_4 = 15;
pub const GL_ARB_DEPTH_FLOAT_EXT: C2RustUnnamed_4 = 14;
pub const GL_ARB_TEXTURE_FLOAT_EXT: C2RustUnnamed_4 = 13;
pub const GL_CLAMP_TEXBORDER_EXT: C2RustUnnamed_4 = 12;
pub const GL_ARB_TEXTURE_NPOT_EXT: C2RustUnnamed_4 = 11;
pub const GL_CLAMPTOEDGE_EXT: C2RustUnnamed_4 = 10;
pub const GL_TEXTURE_3D_EXT: C2RustUnnamed_4 = 9;
pub const GL_TEXTURE_ARRAY_EXT: C2RustUnnamed_4 = 8;
pub const GL_TEXTURE_2D_RECT_EXT: C2RustUnnamed_4 = 7;
pub const GL_SHADER_GLSL100_EXT: C2RustUnnamed_4 = 6;
pub const GL_TEXTURE_COMPRESSION_EXT: C2RustUnnamed_4 = 5;
pub const GL_TEXTURE_LOD_BIAS: C2RustUnnamed_4 = 4;
pub const GL_ANISOTROPY_EXT: C2RustUnnamed_4 = 3;
pub const GL_TEXTURE_CUBEMAP_EXT: C2RustUnnamed_4 = 2;
pub const GL_OPENGL_110: C2RustUnnamed_4 = 0;
/*
gl_backend.c - rendering backend
Copyright (C) 2010 Uncle Mike

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
pub static mut r_speeds_msg: [libc::c_char; 1024] = [0; 1024];
#[no_mangle]
pub static mut r_stats: ref_speeds_t =
    ref_speeds_t{c_world_polys: 0,
                 c_studio_polys: 0,
                 c_sprite_polys: 0,
                 c_alias_polys: 0,
                 c_world_leafs: 0,
                 c_view_beams_count: 0,
                 c_active_tents_count: 0,
                 c_alias_models_drawn: 0,
                 c_studio_models_drawn: 0,
                 c_sprite_models_drawn: 0,
                 c_particle_count: 0,
                 c_client_ents: 0,
                 t_world_node: 0.,
                 t_world_draw: 0.,};
// r_speeds counters
/*
===============
R_SpeedsMessage
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_SpeedsMessage(mut out: *mut libc::c_char,
                                         mut size: size_t) -> qboolean {
    if (*gEngfuncs.drawFuncs).R_SpeedsMessage.is_some() {
        if (*gEngfuncs.drawFuncs).R_SpeedsMessage.expect("non-null function pointer")(out,
                                                                                      size)
               as u64 != 0 {
            return true_0
        }
        // otherwise pass to default handler
    }
    if (*r_speeds).value <= 0 as libc::c_int as libc::c_float {
        return false_0
    }
    if out.is_null() || size == 0 { return false_0 }
    Q_strncpy(out, r_speeds_msg.as_mut_ptr(), size);
    return true_0;
}
/*
==============
R_Speeds_Printf

helper to print into r_speeds message
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_Speeds_Printf(mut msg: *const libc::c_char,
                                         mut args: ...) {
    let mut argptr: ::std::ffi::VaListImpl;
    let mut text: [libc::c_char; 2048] = [0; 2048];
    argptr = args.clone();
    Q_vsnprintf(text.as_mut_ptr(), 99999 as libc::c_int as size_t, msg,
                argptr.as_va_list());
    Q_strncat(r_speeds_msg.as_mut_ptr(), text.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong);
}
/*
==============
GL_BackendStartFrame
==============
*/
#[no_mangle]
pub unsafe extern "C" fn GL_BackendStartFrame() {
    r_speeds_msg[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
}
/*
==============
GL_BackendEndFrame
==============
*/
#[no_mangle]
pub unsafe extern "C" fn GL_BackendEndFrame() {
    let mut curleaf: *mut mleaf_t = 0 as *mut mleaf_t;
    if (*r_speeds).value <= 0 as libc::c_int as libc::c_float ||
           RI.drawWorld as u64 == 0 {
        return
    }
    if RI.viewleaf.is_null() {
        curleaf =
            (*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                   as
                                                                                   libc::c_int)).leafs
    } else { curleaf = RI.viewleaf }
    R_Speeds_Printf(b"Renderer: ^1Engine^7\n\n\x00" as *const u8 as
                        *const libc::c_char);
    match (*r_speeds).value as libc::c_int {
        1 => {
            Q_snprintf(r_speeds_msg.as_mut_ptr(),
                       ::std::mem::size_of::<[libc::c_char; 1024]>() as
                           libc::c_ulong,
                       b"%3i wpoly, %3i apoly\n%3i epoly, %3i spoly\x00" as
                           *const u8 as *const libc::c_char,
                       r_stats.c_world_polys, r_stats.c_alias_polys,
                       r_stats.c_studio_polys, r_stats.c_sprite_polys);
        }
        2 => {
            R_Speeds_Printf(b"visible leafs:\n%3i leafs\ncurrent leaf %3i\n\x00"
                                as *const u8 as *const libc::c_char,
                            r_stats.c_world_leafs,
                            curleaf.wrapping_offset_from((*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                                                                as
                                                                                                                                libc::c_int)).leafs)
                                as libc::c_long);
            R_Speeds_Printf(b"ReciusiveWorldNode: %3lf secs\nDrawTextureChains %lf\n\x00"
                                as *const u8 as *const libc::c_char,
                            r_stats.t_world_node, r_stats.t_world_draw);
        }
        3 => {
            Q_snprintf(r_speeds_msg.as_mut_ptr(),
                       ::std::mem::size_of::<[libc::c_char; 1024]>() as
                           libc::c_ulong,
                       b"%3i alias models drawn\n%3i studio models drawn\n%3i sprites drawn\x00"
                           as *const u8 as *const libc::c_char,
                       r_stats.c_alias_models_drawn,
                       r_stats.c_studio_models_drawn,
                       r_stats.c_sprite_models_drawn);
        }
        4 => {
            Q_snprintf(r_speeds_msg.as_mut_ptr(),
                       ::std::mem::size_of::<[libc::c_char; 1024]>() as
                           libc::c_ulong,
                       b"%3i static entities\n%3i normal entities\n%3i server entities\x00"
                           as *const u8 as *const libc::c_char,
                       r_stats.c_client_ents,
                       (*tr.draw_list).num_solid_entities.wrapping_add((*tr.draw_list).num_trans_entities).wrapping_sub(r_stats.c_client_ents),
                       Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(PARM_NUMENTITIES
                                                                                                                                 as
                                                                                                                                 libc::c_int,
                                                                                                                             0
                                                                                                                                 as
                                                                                                                                 libc::c_int));
        }
        5 => {
            Q_snprintf(r_speeds_msg.as_mut_ptr(),
                       ::std::mem::size_of::<[libc::c_char; 1024]>() as
                           libc::c_ulong,
                       b"%3i tempents\n%3i viewbeams\n%3i particles\x00" as
                           *const u8 as *const libc::c_char,
                       r_stats.c_active_tents_count,
                       r_stats.c_view_beams_count, r_stats.c_particle_count);
        }
        _ => { }
    }
    memset(&mut r_stats as *mut ref_speeds_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<ref_speeds_t>() as libc::c_ulong);
}
/*
=================
GL_LoadTexMatrix
=================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_LoadTexMatrix(mut m: *const [vec_t; 4]) {
    pglMatrixMode.expect("non-null function pointer")(0x1702 as libc::c_int as
                                                          GLenum);
    GL_LoadMatrix(m);
    glState.texIdentityMatrix[glState.activeTMU as usize] =
        false_0 as libc::c_int as GLboolean;
}
/*
=================
GL_LoadTexMatrixExt
=================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_LoadTexMatrixExt(mut glmatrix:
                                                 *const libc::c_float) {
    if glmatrix.is_null() {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 b"../ref_gl/gl_backend.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 138 as
                                                                     libc::c_int);
    }
    pglMatrixMode.expect("non-null function pointer")(0x1702 as libc::c_int as
                                                          GLenum);
    pglLoadMatrixf.expect("non-null function pointer")(glmatrix);
    glState.texIdentityMatrix[glState.activeTMU as usize] =
        false_0 as libc::c_int as GLboolean;
}
/*
=================
GL_LoadMatrix
=================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_LoadMatrix(mut source: *const [vec_t; 4]) {
    let mut dest: [GLfloat; 16] = [0.; 16];
    Matrix4x4_ToArrayFloatGL(source, dest.as_mut_ptr());
    pglLoadMatrixf.expect("non-null function pointer")(dest.as_mut_ptr());
}
/*
=================
GL_LoadIdentityTexMatrix
=================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_LoadIdentityTexMatrix() {
    if glState.texIdentityMatrix[glState.activeTMU as usize] != 0 { return }
    pglMatrixMode.expect("non-null function pointer")(0x1702 as libc::c_int as
                                                          GLenum);
    pglLoadIdentity.expect("non-null function pointer")();
    glState.texIdentityMatrix[glState.activeTMU as usize] =
        true_0 as libc::c_int as GLboolean;
}
/*
=================
GL_SelectTexture
=================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_SelectTexture(mut tmu: GLint) {
    if GL_Support(GL_ARB_MULTITEXTURE as libc::c_int) as u64 == 0 { return }
    // don't allow negative texture units
    if tmu < 0 as libc::c_int { return }
    if tmu >= GL_MaxTextureUnits() {
        gEngfuncs.Con_Reportf.expect("non-null function pointer")(b"^1Error:^7 GL_SelectTexture: bad tmu state %i\n\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char,
                                                                  tmu);
        return
    }
    if glState.activeTMU == tmu { return }
    glState.activeTMU = tmu;
    if pglActiveTextureARB.is_some() {
        pglActiveTextureARB.expect("non-null function pointer")((tmu +
                                                                     0x84c0 as
                                                                         libc::c_int)
                                                                    as
                                                                    GLenum);
        if tmu < glConfig.max_texture_coords {
            pglClientActiveTextureARB.expect("non-null function pointer")((tmu
                                                                               +
                                                                               0x84c0
                                                                                   as
                                                                                   libc::c_int)
                                                                              as
                                                                              GLenum);
        }
    };
}
/*
==============
GL_DisableAllTexGens
==============
*/
#[no_mangle]
pub unsafe extern "C" fn GL_DisableAllTexGens() {
    GL_TexGen(0x2000 as libc::c_int as GLenum, 0 as libc::c_int as GLenum);
    GL_TexGen(0x2001 as libc::c_int as GLenum, 0 as libc::c_int as GLenum);
    GL_TexGen(0x2002 as libc::c_int as GLenum, 0 as libc::c_int as GLenum);
    GL_TexGen(0x2003 as libc::c_int as GLenum, 0 as libc::c_int as GLenum);
}
/*
==============
GL_CleanUpTextureUnits
==============
*/
#[no_mangle]
pub unsafe extern "C" fn GL_CleanUpTextureUnits(mut last: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = glState.activeTMU;
    while i > last - 1 as libc::c_int {
        // disable upper units
        if glState.currentTextureTargets[i as usize] !=
               0 as libc::c_int as libc::c_uint {
            pglDisable.expect("non-null function pointer")(glState.currentTextureTargets[i
                                                                                             as
                                                                                             usize]);
            glState.currentTextureTargets[i as usize] =
                0 as libc::c_int as GLuint;
            glState.currentTextures[i as usize] = -(1 as libc::c_int)
            // unbind texture
        }
        GL_SetTexCoordArrayMode(0 as libc::c_int as GLenum);
        GL_LoadIdentityTexMatrix();
        GL_DisableAllTexGens();
        GL_SelectTexture(i - 1 as libc::c_int);
        i -= 1
    };
}
/*
==============
GL_CleanupAllTextureUnits
==============
*/
#[no_mangle]
pub unsafe extern "C" fn GL_CleanupAllTextureUnits() {
    if glw_state.initialized as u64 == 0 { return }
    // force to cleanup all the units
    GL_SelectTexture(GL_MaxTextureUnits() - 1 as libc::c_int);
    GL_CleanUpTextureUnits(0 as libc::c_int);
}
/*
=================
GL_MultiTexCoord2f
=================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_MultiTexCoord2f(mut texture: GLenum,
                                            mut s: GLfloat, mut t: GLfloat) {
    if GL_Support(GL_ARB_MULTITEXTURE as libc::c_int) as u64 == 0 { return }
    if pglMultiTexCoord2f.is_some() {
        pglMultiTexCoord2f.expect("non-null function pointer")(texture.wrapping_add(0x84c0
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_uint),
                                                               s, t);
    };
}
/*
=================
GL_TextureTarget
=================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_TextureTarget(mut target: uint) {
    if glState.activeTMU < 0 as libc::c_int ||
           glState.activeTMU >= GL_MaxTextureUnits() {
        gEngfuncs.Con_Reportf.expect("non-null function pointer")(b"^1Error:^7 GL_TextureTarget: bad tmu state %i\n\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char,
                                                                  glState.activeTMU);
        return
    }
    if glState.currentTextureTargets[glState.activeTMU as usize] != target {
        if glState.currentTextureTargets[glState.activeTMU as usize] !=
               0 as libc::c_int as libc::c_uint {
            pglDisable.expect("non-null function pointer")(glState.currentTextureTargets[glState.activeTMU
                                                                                             as
                                                                                             usize]);
        }
        glState.currentTextureTargets[glState.activeTMU as usize] = target;
        if target != 0 as libc::c_int as libc::c_uint {
            pglEnable.expect("non-null function pointer")(glState.currentTextureTargets[glState.activeTMU
                                                                                            as
                                                                                            usize]);
        }
    };
}
/*
=================
GL_TexGen
=================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_TexGen(mut coord: GLenum, mut mode: GLenum) {
    let mut tmu: libc::c_int =
        if glConfig.max_texture_coords < glState.activeTMU {
            glConfig.max_texture_coords
        } else { glState.activeTMU };
    let mut bit: libc::c_int = 0;
    let mut gen: libc::c_int = 0;
    match coord {
        8192 => { bit = 1 as libc::c_int; gen = 0xc60 as libc::c_int }
        8193 => { bit = 2 as libc::c_int; gen = 0xc61 as libc::c_int }
        8194 => { bit = 4 as libc::c_int; gen = 0xc62 as libc::c_int }
        8195 => { bit = 8 as libc::c_int; gen = 0xc63 as libc::c_int }
        _ => { return }
    }
    if mode != 0 {
        if glState.genSTEnabled[tmu as usize] & bit == 0 {
            pglEnable.expect("non-null function pointer")(gen as GLenum);
            glState.genSTEnabled[tmu as usize] |= bit
        }
        pglTexGeni.expect("non-null function pointer")(coord,
                                                       0x2500 as libc::c_int
                                                           as GLenum,
                                                       mode as GLint);
    } else if glState.genSTEnabled[tmu as usize] & bit != 0 {
        pglDisable.expect("non-null function pointer")(gen as GLenum);
        glState.genSTEnabled[tmu as usize] &= !bit
    };
}
/*
=================
GL_SetTexCoordArrayMode
=================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_SetTexCoordArrayMode(mut mode: GLenum) {
    let mut tmu: libc::c_int =
        if glConfig.max_texture_coords < glState.activeTMU {
            glConfig.max_texture_coords
        } else { glState.activeTMU };
    let mut bit: libc::c_int = 0;
    let mut cmode: libc::c_int = glState.texCoordArrayMode[tmu as usize];
    if mode == 0x8078 as libc::c_int as libc::c_uint {
        bit = 1 as libc::c_int
    } else if mode == 0x8513 as libc::c_int as libc::c_uint {
        bit = 2 as libc::c_int
    } else { bit = 0 as libc::c_int }
    if cmode != bit {
        if cmode == 1 as libc::c_int {
            pglDisableClientState.expect("non-null function pointer")(0x8078
                                                                          as
                                                                          libc::c_int
                                                                          as
                                                                          GLenum);
        } else if cmode == 2 as libc::c_int {
            pglDisable.expect("non-null function pointer")(0x8513 as
                                                               libc::c_int as
                                                               GLenum);
        }
        if bit == 1 as libc::c_int {
            pglEnableClientState.expect("non-null function pointer")(0x8078 as
                                                                         libc::c_int
                                                                         as
                                                                         GLenum);
        } else if bit == 2 as libc::c_int {
            pglEnable.expect("non-null function pointer")(0x8513 as
                                                              libc::c_int as
                                                              GLenum);
        }
        glState.texCoordArrayMode[tmu as usize] = bit
    };
}
/*
=================
GL_Cull
=================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_Cull(mut cull: GLenum) {
    if cull == 0 {
        pglDisable.expect("non-null function pointer")(0xb44 as libc::c_int as
                                                           GLenum);
        glState.faceCull = 0 as libc::c_int;
        return
    }
    pglEnable.expect("non-null function pointer")(0xb44 as libc::c_int as
                                                      GLenum);
    pglCullFace.expect("non-null function pointer")(cull);
    glState.faceCull = cull as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn GL_SetRenderMode(mut mode: libc::c_int) {
    pglTexEnvi.expect("non-null function pointer")(0x2300 as libc::c_int as
                                                       GLenum,
                                                   0x2200 as libc::c_int as
                                                       GLenum,
                                                   0x2100 as libc::c_int);
    match mode {
        1 | 2 => {
            pglEnable.expect("non-null function pointer")(0xbe2 as libc::c_int
                                                              as GLenum);
            pglDisable.expect("non-null function pointer")(0xbc0 as
                                                               libc::c_int as
                                                               GLenum);
            pglBlendFunc.expect("non-null function pointer")(0x302 as
                                                                 libc::c_int
                                                                 as GLenum,
                                                             0x303 as
                                                                 libc::c_int
                                                                 as GLenum);
        }
        4 => {
            pglDisable.expect("non-null function pointer")(0xbe2 as
                                                               libc::c_int as
                                                               GLenum);
            pglEnable.expect("non-null function pointer")(0xbc0 as libc::c_int
                                                              as GLenum);
        }
        3 | 5 => {
            pglEnable.expect("non-null function pointer")(0xbe2 as libc::c_int
                                                              as GLenum);
            pglDisable.expect("non-null function pointer")(0xbc0 as
                                                               libc::c_int as
                                                               GLenum);
            pglBlendFunc.expect("non-null function pointer")(0x302 as
                                                                 libc::c_int
                                                                 as GLenum,
                                                             0x1 as
                                                                 libc::c_int
                                                                 as GLenum);
        }
        0 | _ => {
            pglDisable.expect("non-null function pointer")(0xbe2 as
                                                               libc::c_int as
                                                               GLenum);
            pglDisable.expect("non-null function pointer")(0xbc0 as
                                                               libc::c_int as
                                                               GLenum);
        }
    };
}
#[no_mangle]
pub static mut r_skyBoxInfo: [envmap_t; 6] =
    [{
         let mut init =
             envmap_s{angles:
                          [0 as libc::c_int as vec_t,
                           270 as libc::c_int as vec_t,
                           180 as libc::c_int as vec_t],
                      flags: IMAGE_FLIP_X as libc::c_int,};
         init
     },
     {
         let mut init =
             envmap_s{angles:
                          [0 as libc::c_int as vec_t,
                           90 as libc::c_int as vec_t,
                           180 as libc::c_int as vec_t],
                      flags: IMAGE_FLIP_X as libc::c_int,};
         init
     },
     {
         let mut init =
             envmap_s{angles:
                          [-(90 as libc::c_int) as vec_t,
                           0 as libc::c_int as vec_t,
                           180 as libc::c_int as vec_t],
                      flags: IMAGE_FLIP_X as libc::c_int,};
         init
     },
     {
         let mut init =
             envmap_s{angles:
                          [90 as libc::c_int as vec_t,
                           0 as libc::c_int as vec_t,
                           180 as libc::c_int as vec_t],
                      flags: IMAGE_FLIP_X as libc::c_int,};
         init
     },
     {
         let mut init =
             envmap_s{angles:
                          [0 as libc::c_int as vec_t,
                           0 as libc::c_int as vec_t,
                           180 as libc::c_int as vec_t],
                      flags: IMAGE_FLIP_X as libc::c_int,};
         init
     },
     {
         let mut init =
             envmap_s{angles:
                          [0 as libc::c_int as vec_t,
                           180 as libc::c_int as vec_t,
                           180 as libc::c_int as vec_t],
                      flags: IMAGE_FLIP_X as libc::c_int,};
         init
     }];
#[no_mangle]
pub static mut r_envMapInfo: [envmap_t; 6] =
    [{
         let mut init =
             envmap_s{angles:
                          [0 as libc::c_int as vec_t,
                           0 as libc::c_int as vec_t,
                           90 as libc::c_int as vec_t],
                      flags: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             envmap_s{angles:
                          [0 as libc::c_int as vec_t,
                           180 as libc::c_int as vec_t,
                           -(90 as libc::c_int) as vec_t],
                      flags: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             envmap_s{angles:
                          [0 as libc::c_int as vec_t,
                           90 as libc::c_int as vec_t,
                           0 as libc::c_int as vec_t],
                      flags: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             envmap_s{angles:
                          [0 as libc::c_int as vec_t,
                           270 as libc::c_int as vec_t,
                           180 as libc::c_int as vec_t],
                      flags: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             envmap_s{angles:
                          [-(90 as libc::c_int) as vec_t,
                           180 as libc::c_int as vec_t,
                           -(90 as libc::c_int) as vec_t],
                      flags: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             envmap_s{angles:
                          [90 as libc::c_int as vec_t,
                           0 as libc::c_int as vec_t,
                           90 as libc::c_int as vec_t],
                      flags: 0 as libc::c_int,};
         init
     }];
#[no_mangle]
pub unsafe extern "C" fn VID_ScreenShot(mut filename: *const libc::c_char,
                                        mut shot_type: libc::c_int)
 -> qboolean {
    let mut r_shot: *mut rgbdata_t = 0 as *mut rgbdata_t;
    let mut flags: uint = IMAGE_FLIP_Y as libc::c_int as uint;
    let mut width: libc::c_int = 0 as libc::c_int;
    let mut height: libc::c_int = 0 as libc::c_int;
    let mut result: qboolean = false_0;
    r_shot =
        gEngfuncs._Mem_Alloc.expect("non-null function pointer")(r_temppool,
                                                                 ::std::mem::size_of::<rgbdata_t>()
                                                                     as
                                                                     libc::c_ulong,
                                                                 true_0,
                                                                 b"../ref_gl/gl_backend.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 464 as
                                                                     libc::c_int)
            as *mut rgbdata_t;
    (*r_shot).width =
        ((*gpGlobals).width + 3 as libc::c_int & !(3 as libc::c_int)) as word;
    (*r_shot).height =
        ((*gpGlobals).height + 3 as libc::c_int & !(3 as libc::c_int)) as
            word;
    (*r_shot).flags = IMAGE_HAS_COLOR as libc::c_int as uint;
    (*r_shot).type_0 = PF_RGBA_32 as libc::c_int as uint;
    (*r_shot).size =
        ((*r_shot).width as libc::c_int * (*r_shot).height as libc::c_int *
             (*gEngfuncs.Image_GetPFDesc.expect("non-null function pointer")((*r_shot).type_0
                                                                                 as
                                                                                 libc::c_int)).bpp)
            as size_t;
    (*r_shot).palette = 0 as *mut byte;
    (*r_shot).buffer =
        gEngfuncs._Mem_Alloc.expect("non-null function pointer")(r_temppool,
                                                                 (*r_shot).size,
                                                                 false_0,
                                                                 b"../ref_gl/gl_backend.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 471 as
                                                                     libc::c_int)
            as *mut byte;
    // get screen frame
    pglReadPixels.expect("non-null function pointer")(0 as libc::c_int,
                                                      0 as libc::c_int,
                                                      (*r_shot).width as
                                                          GLsizei,
                                                      (*r_shot).height as
                                                          GLsizei,
                                                      0x1908 as libc::c_int as
                                                          GLenum,
                                                      0x1401 as libc::c_int as
                                                          GLenum,
                                                      (*r_shot).buffer as
                                                          *mut libc::c_void); // GoldSrc request overviews in 8-bit format
    match shot_type {
        4 => {
            gEngfuncs.FS_AllowDirectPaths.expect("non-null function pointer")(true_0);
        }
        1 => {
            flags |= IMAGE_RESAMPLE as libc::c_int as libc::c_uint;
            if (*gpGlobals).wideScreen as u64 != 0 {
                height = 480 as libc::c_int;
                width = 800 as libc::c_int
            } else { height = 480 as libc::c_int; width = 640 as libc::c_int }
        }
        2 => {
            flags |= IMAGE_RESAMPLE as libc::c_int as libc::c_uint;
            height = 200 as libc::c_int;
            width = 320 as libc::c_int
        }
        3 => {
            flags |=
                (IMAGE_RESAMPLE as libc::c_int |
                     IMAGE_QUANTIZE as libc::c_int) as libc::c_uint;
            height = 768 as libc::c_int;
            width = 1024 as libc::c_int
        }
        0 | _ => { }
    }
    gEngfuncs.Image_Process.expect("non-null function pointer")(&mut r_shot,
                                                                width, height,
                                                                flags,
                                                                0.0f32);
    // write image
    result =
        gEngfuncs.FS_SaveImage.expect("non-null function pointer")(filename,
                                                                   r_shot); // always reset after store screenshot
    gEngfuncs.FS_AllowDirectPaths.expect("non-null function pointer")(false_0);
    gEngfuncs.FS_FreeImage.expect("non-null function pointer")(r_shot);
    return result;
}
/*
=================
VID_CubemapShot
=================
*/
#[no_mangle]
pub unsafe extern "C" fn VID_CubemapShot(mut base: *const libc::c_char,
                                         mut size: uint,
                                         mut vieworg: *const libc::c_float,
                                         mut skyshot: qboolean) -> qboolean {
    let mut r_shot: *mut rgbdata_t = 0 as *mut rgbdata_t;
    let mut r_side: *mut rgbdata_t = 0 as *mut rgbdata_t;
    let mut temp: *mut byte = 0 as *mut byte;
    let mut buffer: *mut byte = 0 as *mut byte;
    let mut basename: string = [0; 256];
    let mut i: libc::c_int = 1 as libc::c_int;
    let mut flags: libc::c_int = 0;
    let mut result: libc::c_int = 0;
    if RI.drawWorld as u64 == 0 ||
           gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                as
                                                                                libc::c_int).is_null()
       {
        return false_0
    }
    // make sure the specified size is valid
    while (i as libc::c_uint) < size { i <<= 1 as libc::c_int }
    if i as libc::c_uint != size { return false_0 }
    if size > (*gpGlobals).width as libc::c_uint ||
           size > (*gpGlobals).height as libc::c_uint {
        return false_0
    }
    // alloc space
    temp =
        gEngfuncs._Mem_Alloc.expect("non-null function pointer")(r_temppool,
                                                                 size.wrapping_mul(size).wrapping_mul(3
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          as
                                                                                                          libc::c_uint)
                                                                     as
                                                                     size_t,
                                                                 false_0,
                                                                 b"../ref_gl/gl_backend.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 542 as
                                                                     libc::c_int)
            as *mut byte;
    buffer =
        gEngfuncs._Mem_Alloc.expect("non-null function pointer")(r_temppool,
                                                                 size.wrapping_mul(size).wrapping_mul(3
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          as
                                                                                                          libc::c_uint).wrapping_mul(6
                                                                                                                                         as
                                                                                                                                         libc::c_int
                                                                                                                                         as
                                                                                                                                         libc::c_uint)
                                                                     as
                                                                     size_t,
                                                                 false_0,
                                                                 b"../ref_gl/gl_backend.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 543 as
                                                                     libc::c_int)
            as *mut byte;
    r_shot =
        gEngfuncs._Mem_Alloc.expect("non-null function pointer")(r_temppool,
                                                                 ::std::mem::size_of::<rgbdata_t>()
                                                                     as
                                                                     libc::c_ulong,
                                                                 true_0,
                                                                 b"../ref_gl/gl_backend.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 544 as
                                                                     libc::c_int)
            as *mut rgbdata_t;
    r_side =
        gEngfuncs._Mem_Alloc.expect("non-null function pointer")(r_temppool,
                                                                 ::std::mem::size_of::<rgbdata_t>()
                                                                     as
                                                                     libc::c_ulong,
                                                                 true_0,
                                                                 b"../ref_gl/gl_backend.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 545 as
                                                                     libc::c_int)
            as *mut rgbdata_t;
    // use client vieworg
    if vieworg.is_null() { vieworg = RI.vieworg.as_mut_ptr() }
    R_CheckGamma();
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        // go into 3d mode
        R_Set2DMode(false_0);
        if skyshot as u64 != 0 {
            R_DrawCubemapView(vieworg,
                              r_skyBoxInfo[i as usize].angles.as_ptr(),
                              size as libc::c_int);
            flags = r_skyBoxInfo[i as usize].flags
        } else {
            R_DrawCubemapView(vieworg,
                              r_envMapInfo[i as usize].angles.as_ptr(),
                              size as libc::c_int);
            flags = r_envMapInfo[i as usize].flags
        }
        pglReadPixels.expect("non-null function pointer")(0 as libc::c_int,
                                                          0 as libc::c_int,
                                                          size as GLsizei,
                                                          size as GLsizei,
                                                          0x1907 as
                                                              libc::c_int as
                                                              GLenum,
                                                          0x1401 as
                                                              libc::c_int as
                                                              GLenum,
                                                          temp as
                                                              *mut libc::c_void);
        (*r_side).flags = IMAGE_HAS_COLOR as libc::c_int as uint;
        (*r_side).height = size as word;
        (*r_side).width = (*r_side).height;
        (*r_side).type_0 = PF_RGB_24 as libc::c_int as uint;
        (*r_side).size =
            ((*r_side).width as libc::c_int * (*r_side).height as libc::c_int
                 * 3 as libc::c_int) as size_t;
        (*r_side).buffer = temp;
        if flags != 0 {
            gEngfuncs.Image_Process.expect("non-null function pointer")(&mut r_side,
                                                                        0 as
                                                                            libc::c_int,
                                                                        0 as
                                                                            libc::c_int,
                                                                        flags
                                                                            as
                                                                            uint,
                                                                        0.0f32);
        }
        memcpy(buffer.offset(size.wrapping_mul(size).wrapping_mul(3 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint).wrapping_mul(i
                                                                                                     as
                                                                                                     libc::c_uint)
                                 as isize) as *mut libc::c_void,
               (*r_side).buffer as *const libc::c_void,
               size.wrapping_mul(size).wrapping_mul(3 as libc::c_int as
                                                        libc::c_uint) as
                   libc::c_ulong);
        i += 1
    }
    (*r_shot).flags = IMAGE_HAS_COLOR as libc::c_int as uint;
    (*r_shot).flags |=
        if skyshot as libc::c_uint != 0 {
            IMAGE_SKYBOX as libc::c_int
        } else { IMAGE_CUBEMAP as libc::c_int } as libc::c_uint;
    (*r_shot).width = size as word;
    (*r_shot).height = size as word;
    (*r_shot).type_0 = PF_RGB_24 as libc::c_int as uint;
    (*r_shot).size =
        ((*r_shot).width as libc::c_int * (*r_shot).height as libc::c_int *
             3 as libc::c_int * 6 as libc::c_int) as size_t;
    (*r_shot).palette = 0 as *mut byte;
    (*r_shot).buffer = buffer;
    // make sure what we have right extension
    Q_strncpy(basename.as_mut_ptr(), base, 256 as libc::c_int as size_t);
    COM_StripExtension(basename.as_mut_ptr());
    COM_DefaultExtension(basename.as_mut_ptr(),
                         b".tga\x00" as *const u8 as *const libc::c_char);
    // write image as 6 sides
    result =
        gEngfuncs.FS_SaveImage.expect("non-null function pointer")(basename.as_mut_ptr(),
                                                                   r_shot) as
            libc::c_int;
    gEngfuncs.FS_FreeImage.expect("non-null function pointer")(r_shot);
    gEngfuncs.FS_FreeImage.expect("non-null function pointer")(r_side);
    return result as qboolean;
}
//=======================================================
/*
===============
R_ShowTextures

Draw all the images to the screen, on top of whatever
was there.  This is used to test for texture thrashing.
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_ShowTextures() {
    let mut image: *mut gl_texture_t =
        0 as *mut gl_texture_t; // textures view by horizontal
    let mut x: libc::c_float = 0.; // textures view by vertical
    let mut y: libc::c_float = 0.; // found start
    let mut w: libc::c_float = 0.;
    let mut h: libc::c_float = 0.;
    let mut total: libc::c_int = 0;
    let mut start: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut base_w: libc::c_int = 0;
    let mut base_h: libc::c_int = 0;
    let mut color: rgba_t =
        [192 as libc::c_int as byte, 192 as libc::c_int as byte,
         192 as libc::c_int as byte, 255 as libc::c_int as byte];
    let mut charHeight: libc::c_int = 0;
    let mut numTries: libc::c_int = 0 as libc::c_int;
    static mut showHelp: qboolean = true_0;
    let mut shortname: string = [0; 256];
    if if !gl_showtextures.is_null() && (*gl_showtextures).value != 0.0f32 {
           true_0 as libc::c_int
       } else { false_0 as libc::c_int } == 0 {
        return
    }
    if showHelp as u64 != 0 {
        gEngfuncs.CL_CenterPrint.expect("non-null function pointer")(b"use \'<-\' and \'->\' keys to change atlas page, ESC to quit\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     0.25f32);
        showHelp = false_0
    }
    GL_SetRenderMode(kRenderNormal as libc::c_int);
    pglClear.expect("non-null function pointer")(0x4000 as libc::c_int as
                                                     GLbitfield);
    pglFinish.expect("non-null function pointer")();
    base_w = 8 as libc::c_int;
    base_h = 6 as libc::c_int;
    loop  {
        total = base_w * base_h;
        start =
            (total as libc::c_float *
                 ((*gl_showtextures).value -
                      1 as libc::c_int as libc::c_float)) as libc::c_int;
        end =
            (total as libc::c_float * (*gl_showtextures).value) as
                libc::c_int;
        if end > 4096 as libc::c_int { end = 4096 as libc::c_int }
        w = ((*gpGlobals).width / base_w) as libc::c_float;
        h = ((*gpGlobals).height / base_h) as libc::c_float;
        gEngfuncs.Con_DrawStringLen.expect("non-null function pointer")(0 as
                                                                            *const libc::c_char,
                                                                        0 as
                                                                            *mut libc::c_int,
                                                                        &mut charHeight);
        j = 0 as libc::c_int;
        i = j;
        while i < 4096 as libc::c_int {
            image = R_GetTexture(i as GLenum);
            if j == start { break ; }
            if pglIsTexture.expect("non-null function pointer")((*image).texnum)
                   != 0 {
                j += 1
            }
            i += 1
        }
        if !(i == 4096 as libc::c_int &&
                 (*gl_showtextures).value !=
                     1 as libc::c_int as libc::c_float) {
            break ;
        }
        // bad case, rewind to one and try again
        gEngfuncs.Cvar_SetValue.expect("non-null function pointer")(b"r_showtextures\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char,
                                                                    if 1 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_float
                                                                           >
                                                                           (*gl_showtextures).value
                                                                               -
                                                                               1
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_float
                                                                       {
                                                                        1 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_float
                                                                    } else {
                                                                        ((*gl_showtextures).value)
                                                                            -
                                                                            1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_float
                                                                    });
        numTries += 1;
        if !(numTries < 2 as libc::c_int) { break ; }
        // to prevent infinite loop
    } // page is full
    k =
        0 as
            libc::c_int; // NOTE: don't use image->texnum here, because skybox has a 'wrong' indexes
    while i < 4096 as libc::c_int {
        if j == end { break ; }
        image = R_GetTexture(i as GLenum);
        if !(pglIsTexture.expect("non-null function pointer")((*image).texnum)
                 == 0) {
            x = (k % base_w) as libc::c_float * w;
            y = (k / base_w) as libc::c_float * h;
            pglColor4f.expect("non-null function pointer")(1.0f32, 1.0f32,
                                                           1.0f32, 1.0f32);
            GL_Bind(XASH_TEXTURE0 as libc::c_int, i as GLenum);
            if (*image).flags as libc::c_uint &
                   TF_DEPTHMAP as libc::c_int as libc::c_uint != 0 &&
                   (*image).flags as libc::c_uint &
                       TF_NOCOMPARE as libc::c_int as libc::c_uint == 0 {
                pglTexParameteri.expect("non-null function pointer")((*image).target,
                                                                     0x884c as
                                                                         libc::c_int
                                                                         as
                                                                         GLenum,
                                                                     0 as
                                                                         libc::c_int);
            }
            pglBegin.expect("non-null function pointer")(0x7 as libc::c_int as
                                                             GLenum);
            pglTexCoord2f.expect("non-null function pointer")(0 as libc::c_int
                                                                  as GLfloat,
                                                              0 as libc::c_int
                                                                  as GLfloat);
            pglVertex2f.expect("non-null function pointer")(x, y);
            if (*image).target == 0x84f5 as libc::c_int as libc::c_uint {
                pglTexCoord2f.expect("non-null function pointer")((*image).width
                                                                      as
                                                                      GLfloat,
                                                                  0 as
                                                                      libc::c_int
                                                                      as
                                                                      GLfloat);
            } else {
                pglTexCoord2f.expect("non-null function pointer")(1 as
                                                                      libc::c_int
                                                                      as
                                                                      GLfloat,
                                                                  0 as
                                                                      libc::c_int
                                                                      as
                                                                      GLfloat);
            }
            pglVertex2f.expect("non-null function pointer")(x + w, y);
            if (*image).target == 0x84f5 as libc::c_int as libc::c_uint {
                pglTexCoord2f.expect("non-null function pointer")((*image).width
                                                                      as
                                                                      GLfloat,
                                                                  (*image).height
                                                                      as
                                                                      GLfloat);
            } else {
                pglTexCoord2f.expect("non-null function pointer")(1 as
                                                                      libc::c_int
                                                                      as
                                                                      GLfloat,
                                                                  1 as
                                                                      libc::c_int
                                                                      as
                                                                      GLfloat);
            }
            pglVertex2f.expect("non-null function pointer")(x + w, y + h);
            if (*image).target == 0x84f5 as libc::c_int as libc::c_uint {
                pglTexCoord2f.expect("non-null function pointer")(0 as
                                                                      libc::c_int
                                                                      as
                                                                      GLfloat,
                                                                  (*image).height
                                                                      as
                                                                      GLfloat);
            } else {
                pglTexCoord2f.expect("non-null function pointer")(0 as
                                                                      libc::c_int
                                                                      as
                                                                      GLfloat,
                                                                  1 as
                                                                      libc::c_int
                                                                      as
                                                                      GLfloat);
            }
            pglVertex2f.expect("non-null function pointer")(x, y + h);
            pglEnd.expect("non-null function pointer")();
            if (*image).flags as libc::c_uint &
                   TF_DEPTHMAP as libc::c_int as libc::c_uint != 0 &&
                   (*image).flags as libc::c_uint &
                       TF_NOCOMPARE as libc::c_int as libc::c_uint == 0 {
                pglTexParameteri.expect("non-null function pointer")((*image).target,
                                                                     0x884c as
                                                                         libc::c_int
                                                                         as
                                                                         GLenum,
                                                                     0x884e as
                                                                         libc::c_int);
            }
            COM_FileBase((*image).name.as_mut_ptr(), shortname.as_mut_ptr());
            if Q_strlen(shortname.as_mut_ptr()) >
                   18 as libc::c_int as libc::c_ulong {
                // cutoff too long names, it looks ugly
                shortname[16 as libc::c_int as usize] =
                    '.' as i32 as libc::c_char;
                shortname[17 as libc::c_int as usize] =
                    '.' as i32 as libc::c_char;
                shortname[18 as libc::c_int as usize] =
                    '\u{0}' as i32 as libc::c_char
            }
            gEngfuncs.Con_DrawString.expect("non-null function pointer")((x +
                                                                              1
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_float)
                                                                             as
                                                                             libc::c_int,
                                                                         (y +
                                                                              h
                                                                              -
                                                                              charHeight
                                                                                  as
                                                                                  libc::c_float)
                                                                             as
                                                                             libc::c_int,
                                                                         shortname.as_mut_ptr(),
                                                                         color.as_mut_ptr());
            j += 1;
            k += 1
        }
        i += 1
    }
    gEngfuncs.CL_DrawCenterPrint.expect("non-null function pointer")();
    pglFinish.expect("non-null function pointer")();
}
/*
================
SCR_TimeRefresh_f

timerefresh [noflip]
================
*/
#[no_mangle]
pub unsafe extern "C" fn SCR_TimeRefresh_f() {
    let mut i: libc::c_int = 0;
    let mut start: libc::c_double = 0.;
    let mut stop: libc::c_double = 0.;
    let mut time: libc::c_double = 0.;
    if Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(PARM_CONNSTATE
                                                                                                                 as
                                                                                                                 libc::c_int,
                                                                                                             0
                                                                                                                 as
                                                                                                                 libc::c_int)
           != ca_active as libc::c_int {
        return
    }
    start = gEngfuncs.pfnTime.expect("non-null function pointer")();
    // run without page flipping like GoldSrc
    if gEngfuncs.Cmd_Argc.expect("non-null function pointer")() ==
           1 as libc::c_int {
        pglDrawBuffer.expect("non-null function pointer")(0x404 as libc::c_int
                                                              as GLenum);
        i = 0 as libc::c_int;
        while i < 128 as libc::c_int {
            (*gpGlobals).viewangles[1 as libc::c_int as usize] =
                i as libc::c_float / 128.0f32 * 360.0f32;
            R_RenderScene();
            i += 1
        }
        pglFinish.expect("non-null function pointer")();
        R_EndFrame();
    } else {
        i = 0 as libc::c_int;
        while i < 128 as libc::c_int {
            R_BeginFrame(true_0);
            (*gpGlobals).viewangles[1 as libc::c_int as usize] =
                i as libc::c_float / 128.0f32 * 360.0f32;
            R_RenderScene();
            R_EndFrame();
            i += 1
        }
    }
    stop = gEngfuncs.pfnTime.expect("non-null function pointer")();
    time = stop - start;
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"%f seconds (%f fps)\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             time,
                                                             128 as
                                                                 libc::c_int
                                                                 as
                                                                 libc::c_double
                                                                 / time);
}
