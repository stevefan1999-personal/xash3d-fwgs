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
    #[no_mangle]
    fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t,
             __compar: __compar_fn_t);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn powf(_: libc::c_float, _: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn sqrtf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn fabsf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn floorf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn SinCos(radians: libc::c_float, sine: *mut libc::c_float,
              cosine: *mut libc::c_float);
    #[no_mangle]
    fn ClearBounds(mins: *mut vec_t, maxs: *mut vec_t);
    #[no_mangle]
    fn Matrix4x4_VectorITransform(in_0: *const [vec_t; 4],
                                  v: *const libc::c_float,
                                  out: *mut libc::c_float);
    #[no_mangle]
    fn BoxOnPlaneSide(emins: *const vec_t, emaxs: *const vec_t,
                      p: *const mplane_t) -> libc::c_int;
    #[no_mangle]
    static mut pglAlphaFunc:
           Option<unsafe extern "C" fn(_: GLenum, _: GLclampf) -> ()>;
    #[no_mangle]
    static mut pglBegin: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglBlendFunc:
           Option<unsafe extern "C" fn(_: GLenum, _: GLenum) -> ()>;
    #[no_mangle]
    static mut pglColor3f:
           Option<unsafe extern "C" fn(_: GLfloat, _: GLfloat, _: GLfloat)
                      -> ()>;
    #[no_mangle]
    static mut pglColor4f:
           Option<unsafe extern "C" fn(_: GLfloat, _: GLfloat, _: GLfloat,
                                       _: GLfloat) -> ()>;
    #[no_mangle]
    static mut pglColor4ub:
           Option<unsafe extern "C" fn(_: GLubyte, _: GLubyte, _: GLubyte,
                                       _: GLubyte) -> ()>;
    #[no_mangle]
    static mut pglDepthFunc: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglDepthMask: Option<unsafe extern "C" fn(_: GLboolean) -> ()>;
    #[no_mangle]
    static mut pglDisable: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglDisableClientState:
           Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglDrawArrays:
           Option<unsafe extern "C" fn(_: GLenum, _: GLint, _: GLsizei)
                      -> ()>;
    #[no_mangle]
    static mut pglDrawElements:
           Option<unsafe extern "C" fn(_: GLenum, _: GLsizei, _: GLenum,
                                       _: *const libc::c_void) -> ()>;
    #[no_mangle]
    static mut pglEnable: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglEnableClientState:
           Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglEnd: Option<unsafe extern "C" fn() -> ()>;
    #[no_mangle]
    static mut pglFogfv:
           Option<unsafe extern "C" fn(_: GLenum, _: *const GLfloat) -> ()>;
    #[no_mangle]
    static mut pglLoadIdentity: Option<unsafe extern "C" fn() -> ()>;
    #[no_mangle]
    static mut pglMatrixMode: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglPolygonMode:
           Option<unsafe extern "C" fn(_: GLenum, _: GLenum) -> ()>;
    #[no_mangle]
    static mut pglScalef:
           Option<unsafe extern "C" fn(_: GLfloat, _: GLfloat, _: GLfloat)
                      -> ()>;
    #[no_mangle]
    static mut pglTexCoord2f:
           Option<unsafe extern "C" fn(_: GLfloat, _: GLfloat) -> ()>;
    #[no_mangle]
    static mut pglTexCoordPointer:
           Option<unsafe extern "C" fn(_: GLint, _: GLenum, _: GLsizei,
                                       _: *const libc::c_void) -> ()>;
    #[no_mangle]
    static mut pglTexEnvf:
           Option<unsafe extern "C" fn(_: GLenum, _: GLenum, _: GLfloat)
                      -> ()>;
    #[no_mangle]
    static mut pglTexEnvi:
           Option<unsafe extern "C" fn(_: GLenum, _: GLenum, _: GLint) -> ()>;
    #[no_mangle]
    static mut pglTexSubImage2D:
           Option<unsafe extern "C" fn(_: GLenum, _: GLint, _: GLint,
                                       _: GLint, _: GLsizei, _: GLsizei,
                                       _: GLenum, _: GLenum,
                                       _: *const libc::c_void) -> ()>;
    #[no_mangle]
    static mut pglVertex3fv:
           Option<unsafe extern "C" fn(_: *const GLfloat) -> ()>;
    #[no_mangle]
    static mut pglVertexPointer:
           Option<unsafe extern "C" fn(_: GLint, _: GLenum, _: GLsizei,
                                       _: *const libc::c_void) -> ()>;
    #[no_mangle]
    static mut pglDrawRangeElements:
           Option<unsafe extern "C" fn(_: GLenum, _: GLuint, _: GLuint,
                                       _: GLsizei, _: GLenum,
                                       _: *const libc::c_void) -> ()>;
    #[no_mangle]
    static mut pglBindBufferARB:
           Option<unsafe extern "C" fn(_: GLenum, _: GLuint) -> ()>;
    #[no_mangle]
    static mut pglDeleteBuffersARB:
           Option<unsafe extern "C" fn(_: GLsizei, _: *const GLuint) -> ()>;
    #[no_mangle]
    static mut pglGenBuffersARB:
           Option<unsafe extern "C" fn(_: GLsizei, _: *mut GLuint) -> ()>;
    #[no_mangle]
    static mut pglBufferDataARB:
           Option<unsafe extern "C" fn(_: GLenum, _: GLsizeiptrARB,
                                       _: *const libc::c_void, _: GLenum)
                      -> ()>;
    #[no_mangle]
    static mut pglBufferSubDataARB:
           Option<unsafe extern "C" fn(_: GLenum, _: GLintptrARB,
                                       _: GLsizeiptrARB,
                                       _: *const libc::c_void) -> ()>;
    #[no_mangle]
    static mut r_stats: ref_speeds_t;
    #[no_mangle]
    static mut RI: ref_instance_t;
    #[no_mangle]
    static mut tr: gl_globals_t;
    #[no_mangle]
    fn GL_Bind(tmu: GLint, texnum: GLenum);
    #[no_mangle]
    fn GL_SelectTexture(texture: GLint);
    #[no_mangle]
    fn R_CullBox(mins: *const vec_t, maxs: *const vec_t) -> qboolean;
    #[no_mangle]
    fn R_CullSurface(surf: *mut msurface_t, frustum: *mut gl_frustum_t,
                     clipflags: uint) -> libc::c_int;
    #[no_mangle]
    fn DrawSurfaceDecals(fa: *mut msurface_t, single: qboolean,
                         reverse: qboolean);
    #[no_mangle]
    fn R_DecalSetupVerts(pDecal: *mut decal_t, surf: *mut msurface_t,
                         texture: libc::c_int, outCount: *mut libc::c_int)
     -> *mut libc::c_float;
    #[no_mangle]
    fn DrawDecalsBatch();
    #[no_mangle]
    fn R_DrawWorldHull();
    #[no_mangle]
    fn R_DrawModelHull();
    #[no_mangle]
    fn R_GetTexture(texnum: GLenum) -> *mut gl_texture_t;
    #[no_mangle]
    fn GL_LoadTextureFromBuffer(name: *const libc::c_char,
                                pic: *mut rgbdata_t, flags: texFlags_t,
                                update: qboolean) -> libc::c_int;
    #[no_mangle]
    fn GL_FreeTexture(texnum: GLenum);
    #[no_mangle]
    fn R_InitDlightTexture();
    #[no_mangle]
    fn CL_RunLightStyles();
    #[no_mangle]
    fn R_MarkLights(light: *mut dlight_t, bit: libc::c_int,
                    node: *mut mnode_t);
    #[no_mangle]
    fn R_CountSurfaceDlights(surf: *mut msurface_t) -> libc::c_int;
    #[no_mangle]
    fn R_LoadIdentity();
    #[no_mangle]
    fn R_TranslateForEntity(e: *mut cl_entity_t);
    #[no_mangle]
    fn R_RotateForEntity(e: *mut cl_entity_t);
    #[no_mangle]
    fn R_AllowFog(allowed: qboolean);
    #[no_mangle]
    fn R_DrawSkyBox();
    #[no_mangle]
    fn Q_snprintf(buffer: *mut libc::c_char, buffersize: size_t,
                  format: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn EmitWaterPolys(warp: *mut msurface_t, reverse: qboolean);
    #[no_mangle]
    fn R_DrawClouds();
    #[no_mangle]
    fn R_AddSkyBoxSurface(fa: *mut msurface_t);
    #[no_mangle]
    fn R_ClearSkyBox();
    #[no_mangle]
    static mut gEngfuncs: ref_api_t;
    #[no_mangle]
    fn GL_Support(r_ext: libc::c_int) -> qboolean;
    #[no_mangle]
    static mut glState: glstate_t;
    #[no_mangle]
    static mut gpGlobals: *mut ref_globals_t;
    #[no_mangle]
    static mut glConfig: glconfig_t;
    #[no_mangle]
    static mut gl_keeptjunctions: *mut cvar_t;
    #[no_mangle]
    static mut gl_wireframe: *mut cvar_t;
    #[no_mangle]
    static mut gl_nosort: *mut cvar_t;
    #[no_mangle]
    static mut r_detailtextures: *mut cvar_t;
    #[no_mangle]
    static mut r_fullbright: *mut cvar_t;
    #[no_mangle]
    static mut vid_gamma: *mut cvar_t;
    #[no_mangle]
    static mut r_vbo_dlightmode: *mut cvar_t;
    #[no_mangle]
    static mut r_nocull: *mut cvar_t;
    #[no_mangle]
    static mut r_lightmap: *mut cvar_t;
    #[no_mangle]
    static mut r_dynamic: *mut cvar_t;
    #[no_mangle]
    static mut r_vbo: *mut cvar_t;
    #[no_mangle]
    static mut r_lockpvs: *mut cvar_t;
    #[no_mangle]
    static mut vid_brightness: *mut cvar_t;
    #[no_mangle]
    static mut r_novis: *mut cvar_t;
    // gl_decals.c
    #[no_mangle]
    static mut gDecalPool: [decal_t; 4096];
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
pub type __compar_fn_t
    =
    Option<unsafe extern "C" fn(_: *const libc::c_void,
                                _: *const libc::c_void) -> libc::c_int>;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type byte = libc::c_uchar;
pub type vec_t = libc::c_float;
pub type vec2_t = [vec_t; 2];
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct world_static_s {
    pub loading: qboolean,
    pub flags: libc::c_int,
    pub message: [libc::c_char; 2048],
    pub compiler: [libc::c_char; 256],
    pub generator: [libc::c_char; 256],
    pub hull_models: *mut hull_model_t,
    pub num_hull_models: libc::c_int,
    pub deluxedata: *mut color24,
    pub shadowdata: *mut byte,
    pub visbytes: size_t,
    pub fatbytes: size_t,
    pub mins: vec3_t,
    pub maxs: vec3_t,
    pub size: vec3_t,
    pub recursion_level: libc::c_int,
    pub max_recursion: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hull_model_t {
    pub polys: hullnode_t,
    pub num_polys: uint,
}
pub type hullnode_t = hullnode_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hullnode_s {
    pub next: *mut hullnode_s,
    pub prev: *mut hullnode_s,
}
pub type ref_api_t = ref_api_s;
pub type GLenum = uint;
pub type GLboolean = byte;
pub type GLvoid = ();
pub type GLint = libc::c_int;
pub type GLubyte = byte;
pub type GLuint = uint;
pub type GLsizei = libc::c_int;
pub type GLfloat = libc::c_float;
pub type GLclampf = libc::c_float;
pub type GLintptrARB = libc::c_int;
pub type GLsizeiptrARB = libc::c_int;
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
pub struct gllightmapstate_t {
    pub allocated: [libc::c_int; 1024],
    pub current_lightmap_texture: libc::c_int,
    pub dynamic_surfaces: *mut msurface_t,
    pub lightmap_surfaces: [*mut msurface_t; 256],
    pub lightmap_buffer: [byte; 4194304],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vbo_static_s {
    pub mempool: poolhandle_t,
    pub decaldata: *mut vbodecaldata_t,
    pub textures: *mut vbotexture_t,
    pub surfdata: *mut vbosurfdata_t,
    pub arraylist: *mut vboarray_t,
    pub dlight_index: *mut libc::c_ushort,
    pub dlight_tc: *mut vec2_t,
    pub dlight_vbo: libc::c_uint,
    pub decal_dlight: [vbovertex_t; 131072],
    pub decal_dlight_vbo: libc::c_uint,
    pub decal_numverts: [libc::c_int; 131072],
    pub minlightmap: libc::c_int,
    pub maxlightmap: libc::c_int,
    pub mintexture: libc::c_int,
    pub maxtexture: libc::c_int,
    pub minarraysplit_tex: libc::c_int,
    pub maxarraysplit_tex: libc::c_int,
    pub minarraysplit_lm: libc::c_int,
    pub maxarraysplit_lm: libc::c_int,
}
/*
==============================

VBO

==============================
*/
/*
Bulld arrays (vboarray_t) for all map geometry on map load.
Store index base for every surface (vbosurfdata_t) to build index arrays
For each texture build index arrays (vbotexture_t) every frame.
*/
// vertex attribs
//#define NO_TEXTURE_MATRIX // need debug
pub type vbovertex_t = vbovertex_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vbovertex_s {
    pub pos: vec3_t,
    pub gl_tc: vec2_t,
    pub lm_tc: vec2_t,
}
// array list
pub type vboarray_t = vboarray_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vboarray_s {
    pub glindex: uint,
    pub array_len: libc::c_int,
    pub array: *mut vbovertex_t,
    pub next: *mut vboarray_s,
}
// glGenBuffers
// allocation length
// vertex attrib array
// split by 65536 vertices
// every surface is linked to vbo texture
pub type vbosurfdata_t = vbosurfdata_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vbosurfdata_s {
    pub vbotexture: *mut vbotexture_t,
    pub texturenum: uint,
    pub startindex: uint,
}
// store indexes for each texture
pub type vbotexture_t = vbotexture_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vbotexture_s {
    pub indexarray: *mut libc::c_ushort,
    pub curindex: uint,
    pub len: uint,
    pub next: *mut vbotexture_s,
    pub dlightchain: *mut msurface_t,
    pub vboarray: *mut vboarray_s,
    pub lightmaptexturenum: uint,
}
pub type vbodecaldata_t = vbodecaldata_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vbodecaldata_s {
    pub decals: [vbodecal_t; 4096],
    pub decalarray: [vbovertex_t; 32768],
    pub decalvbo: uint,
    pub lm: *mut *mut msurface_t,
}
pub type vbodecal_t = vbodecal_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vbodecal_s {
    pub numVerts: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct multitexturestate_s {
    pub tmu_gl: libc::c_int,
    pub tmu_dt: libc::c_int,
    pub tmu_lm: libc::c_int,
    pub details_enabled: qboolean,
    pub lm: libc::c_int,
    pub skiptexture: qboolean,
    pub glt: *mut gl_texture_t,
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
pub const GL_ARB_MULTITEXTURE: C2RustUnnamed_4 = 1;
pub const GL_ARB_VERTEX_BUFFER_OBJECT_EXT: C2RustUnnamed_4 = 19;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const GL_EXTCOUNT: C2RustUnnamed_4 = 22;
pub const GL_TEXTURE_MULTISAMPLE: C2RustUnnamed_4 = 21;
pub const GL_DRAW_RANGEELEMENTS_EXT: C2RustUnnamed_4 = 20;
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
#[inline(always)]
unsafe extern "C" fn __tg_pow(mut __x: libc::c_float, mut __y: libc::c_float)
 -> libc::c_float {
    return powf(__x, __y);
}
#[inline(always)]
unsafe extern "C" fn __tg_sqrt(mut __x: libc::c_float) -> libc::c_float {
    return sqrtf(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_fabs(mut __x: libc::c_float) -> libc::c_float {
    return fabsf(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_floor(mut __x: libc::c_float) -> libc::c_float {
    return floorf(__x);
}
static mut nColinElim: libc::c_int = 0;
// index array (generated instead of texture chains)
// counter for index array
// maximum index array length
// if cannot fit into one array, allocate new one, as every array has own index space
// list of dlight surfaces
// debug
// stats
static mut world_orthocenter: vec2_t = [0.; 2];
static mut world_orthohalf: vec2_t = [0.; 2];
static mut r_blocklights: [uint; 3145728] = [0; 3145728];
static mut fullbright_surfaces: [*mut mextrasurf_t; 4096] =
    [0 as *const mextrasurf_t as *mut mextrasurf_t; 4096];
static mut detail_surfaces: [*mut mextrasurf_t; 4096] =
    [0 as *const mextrasurf_t as *mut mextrasurf_t; 4096];
static mut rtable: [[libc::c_int; 20]; 20] = [[0; 20]; 20];
static mut draw_alpha_surfaces: qboolean = false_0;
static mut draw_fullbrights: qboolean = false_0;
static mut draw_details: qboolean = false_0;
static mut skychain: *mut msurface_t =
    0 as *const msurface_t as *mut msurface_t;
static mut gl_lms: gllightmapstate_t =
    gllightmapstate_t{allocated: [0; 1024],
                      current_lightmap_texture: 0,
                      dynamic_surfaces:
                          0 as *const msurface_t as *mut msurface_t,
                      lightmap_surfaces:
                          [0 as *const msurface_t as *mut msurface_t; 256],
                      lightmap_buffer: [0; 4194304],};
#[no_mangle]
pub unsafe extern "C" fn Mod_GetCurrentVis() -> *mut byte {
    if (*gEngfuncs.drawFuncs).Mod_GetCurrentVis.is_some() &&
           tr.fCustomRendering as libc::c_uint != 0 {
        return (*gEngfuncs.drawFuncs).Mod_GetCurrentVis.expect("non-null function pointer")()
    }
    return RI.visbytes.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn Mod_SetOrthoBounds(mut mins: *const libc::c_float,
                                            mut maxs: *const libc::c_float) {
    if (*gEngfuncs.drawFuncs).GL_OrthoBounds.is_some() {
        (*gEngfuncs.drawFuncs).GL_OrthoBounds.expect("non-null function pointer")(mins,
                                                                                  maxs);
    }
    world_orthocenter[0 as libc::c_int as usize] =
        (*maxs.offset(0 as libc::c_int as isize) +
             *mins.offset(0 as libc::c_int as isize)) * 0.5f32;
    world_orthocenter[1 as libc::c_int as usize] =
        (*maxs.offset(1 as libc::c_int as isize) +
             *mins.offset(1 as libc::c_int as isize)) * 0.5f32;
    world_orthohalf[0 as libc::c_int as usize] =
        *maxs.offset(0 as libc::c_int as isize) -
            world_orthocenter[0 as libc::c_int as usize];
    world_orthohalf[1 as libc::c_int as usize] =
        *maxs.offset(1 as libc::c_int as isize) -
            world_orthocenter[1 as libc::c_int as usize];
}
unsafe extern "C" fn BoundPoly(mut numverts: libc::c_int,
                               mut verts: *mut libc::c_float,
                               mut mins: *mut vec_t, mut maxs: *mut vec_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut v: *mut libc::c_float = 0 as *mut libc::c_float;
    ClearBounds(mins, maxs);
    i = 0 as libc::c_int;
    v = verts;
    while i < numverts {
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            if *v < *mins.offset(j as isize) { *mins.offset(j as isize) = *v }
            if *v > *maxs.offset(j as isize) { *maxs.offset(j as isize) = *v }
            j += 1;
            v = v.offset(1)
        }
        i += 1
    };
}
unsafe extern "C" fn SubdividePolygon_r(mut warpface: *mut msurface_t,
                                        mut numverts: libc::c_int,
                                        mut verts: *mut libc::c_float) {
    let mut front: [vec3_t; 64] = [[0.; 3]; 64];
    let mut back: [vec3_t; 64] = [[0.; 3]; 64];
    let mut warpinfo: *mut mextrasurf_t = (*warpface).info;
    let mut dist: [libc::c_float; 64] = [0.; 64];
    let mut m: libc::c_float = 0.;
    let mut frac: libc::c_float = 0.;
    let mut s: libc::c_float = 0.;
    let mut t: libc::c_float = 0.;
    let mut v: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut f: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut sample_size: libc::c_float = 0.;
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut poly: *mut glpoly_t = 0 as *mut glpoly_t;
    let mut loadmodel: *mut model_t =
        gEngfuncs.Mod_GetCurrentLoadingModel.expect("non-null function pointer")();
    if numverts > 64 as libc::c_int - 4 as libc::c_int {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"Mod_SubdividePolygon: too many vertexes on face ( %i )\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 numverts);
    }
    sample_size =
        gEngfuncs.Mod_SampleSizeForFace.expect("non-null function pointer")(warpface)
            as libc::c_float;
    BoundPoly(numverts, verts, mins.as_mut_ptr(), maxs.as_mut_ptr());
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        m = (mins[i as usize] + maxs[i as usize]) * 0.5f32;
        m =
            64 as libc::c_int as libc::c_float *
                __tg_floor(m / 64 as libc::c_int as libc::c_float + 0.5f32);
        if !(maxs[i as usize] - m < 8 as libc::c_int as libc::c_float) {
            if !(m - mins[i as usize] < 8 as libc::c_int as libc::c_float) {
                // cut it
                v = verts.offset(i as isize);
                j = 0 as libc::c_int;
                while j < numverts {
                    dist[j as usize] = *v - m;
                    j += 1;
                    v = v.offset(3 as libc::c_int as isize)
                }
                // wrap cases
                dist[j as usize] = dist[0 as libc::c_int as usize];
                v = v.offset(-(i as isize));
                *v.offset(0 as libc::c_int as isize) =
                    *verts.offset(0 as libc::c_int as isize);
                *v.offset(1 as libc::c_int as isize) =
                    *verts.offset(1 as libc::c_int as isize);
                *v.offset(2 as libc::c_int as isize) =
                    *verts.offset(2 as libc::c_int as isize);
                b = 0 as libc::c_int;
                f = b;
                v = verts;
                j = 0 as libc::c_int;
                while j < numverts {
                    if dist[j as usize] >= 0 as libc::c_int as libc::c_float {
                        front[f as usize][0 as libc::c_int as usize] =
                            *v.offset(0 as libc::c_int as isize);
                        front[f as usize][1 as libc::c_int as usize] =
                            *v.offset(1 as libc::c_int as isize);
                        front[f as usize][2 as libc::c_int as usize] =
                            *v.offset(2 as libc::c_int as isize);
                        f += 1
                    }
                    if dist[j as usize] <= 0 as libc::c_int as libc::c_float {
                        back[b as usize][0 as libc::c_int as usize] =
                            *v.offset(0 as libc::c_int as isize);
                        back[b as usize][1 as libc::c_int as usize] =
                            *v.offset(1 as libc::c_int as isize);
                        back[b as usize][2 as libc::c_int as usize] =
                            *v.offset(2 as libc::c_int as isize);
                        b += 1
                    }
                    if !(dist[j as usize] == 0 as libc::c_int as libc::c_float
                             ||
                             dist[(j + 1 as libc::c_int) as usize] ==
                                 0 as libc::c_int as libc::c_float) {
                        if (dist[j as usize] >
                                0 as libc::c_int as libc::c_float) as
                               libc::c_int !=
                               (dist[(j + 1 as libc::c_int) as usize] >
                                    0 as libc::c_int as libc::c_float) as
                                   libc::c_int {
                            // clip point
                            frac =
                                dist[j as usize] /
                                    (dist[j as usize] -
                                         dist[(j + 1 as libc::c_int) as
                                                  usize]);
                            k = 0 as libc::c_int;
                            while k < 3 as libc::c_int {
                                back[b as usize][k as usize] =
                                    *v.offset(k as isize) +
                                        frac *
                                            (*v.offset((3 as libc::c_int + k)
                                                           as isize) -
                                                 *v.offset(k as isize));
                                front[f as usize][k as usize] =
                                    back[b as usize][k as usize];
                                k += 1
                            }
                            f += 1;
                            b += 1
                        }
                    }
                    j += 1;
                    v = v.offset(3 as libc::c_int as isize)
                }
                SubdividePolygon_r(warpface, f,
                                   front[0 as libc::c_int as
                                             usize].as_mut_ptr());
                SubdividePolygon_r(warpface, b,
                                   back[0 as libc::c_int as
                                            usize].as_mut_ptr());
                return
            }
        }
        i += 1
    }
    if numverts != 4 as libc::c_int {
        (*warpface).flags =
            ((*warpface).flags as libc::c_uint &
                 !((1 as libc::c_uint) << 3 as libc::c_int)) as libc::c_int
    }
    // add a point in the center to help keep warp valid
    poly =
        gEngfuncs._Mem_Alloc.expect("non-null function pointer")((*loadmodel).mempool,
                                                                 (::std::mem::size_of::<glpoly_t>()
                                                                      as
                                                                      libc::c_ulong).wrapping_add((((numverts
                                                                                                         -
                                                                                                         4
                                                                                                             as
                                                                                                             libc::c_int)
                                                                                                        *
                                                                                                        7
                                                                                                            as
                                                                                                            libc::c_int)
                                                                                                       as
                                                                                                       libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_float>()
                                                                                                                                       as
                                                                                                                                       libc::c_ulong)),
                                                                 true_0,
                                                                 b"../ref_gl/gl_rsurf.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 155 as
                                                                     libc::c_int)
            as *mut glpoly_t;
    (*poly).next = (*warpface).polys;
    (*poly).flags = (*warpface).flags;
    (*warpface).polys = poly;
    (*poly).numverts = numverts;
    i = 0 as libc::c_int;
    while i < numverts {
        (*poly).verts[i as usize][0 as libc::c_int as usize] =
            *verts.offset(0 as libc::c_int as isize);
        (*poly).verts[i as usize][1 as libc::c_int as usize] =
            *verts.offset(1 as libc::c_int as isize);
        (*poly).verts[i as usize][2 as libc::c_int as usize] =
            *verts.offset(2 as libc::c_int as isize);
        if (*warpface).flags as libc::c_uint &
               (1 as libc::c_uint) << 4 as libc::c_int != 0 {
            s =
                *verts.offset(0 as libc::c_int as isize) *
                    (*(*warpface).texinfo).vecs[0 as libc::c_int as
                                                    usize][0 as libc::c_int as
                                                               usize] +
                    *verts.offset(1 as libc::c_int as isize) *
                        (*(*warpface).texinfo).vecs[0 as libc::c_int as
                                                        usize][1 as
                                                                   libc::c_int
                                                                   as usize] +
                    *verts.offset(2 as libc::c_int as isize) *
                        (*(*warpface).texinfo).vecs[0 as libc::c_int as
                                                        usize][2 as
                                                                   libc::c_int
                                                                   as usize];
            t =
                *verts.offset(0 as libc::c_int as isize) *
                    (*(*warpface).texinfo).vecs[1 as libc::c_int as
                                                    usize][0 as libc::c_int as
                                                               usize] +
                    *verts.offset(1 as libc::c_int as isize) *
                        (*(*warpface).texinfo).vecs[1 as libc::c_int as
                                                        usize][1 as
                                                                   libc::c_int
                                                                   as usize] +
                    *verts.offset(2 as libc::c_int as isize) *
                        (*(*warpface).texinfo).vecs[1 as libc::c_int as
                                                        usize][2 as
                                                                   libc::c_int
                                                                   as usize]
        } else {
            s =
                *verts.offset(0 as libc::c_int as isize) *
                    (*(*warpface).texinfo).vecs[0 as libc::c_int as
                                                    usize][0 as libc::c_int as
                                                               usize] +
                    *verts.offset(1 as libc::c_int as isize) *
                        (*(*warpface).texinfo).vecs[0 as libc::c_int as
                                                        usize][1 as
                                                                   libc::c_int
                                                                   as usize] +
                    *verts.offset(2 as libc::c_int as isize) *
                        (*(*warpface).texinfo).vecs[0 as libc::c_int as
                                                        usize][2 as
                                                                   libc::c_int
                                                                   as usize] +
                    (*(*warpface).texinfo).vecs[0 as libc::c_int as
                                                    usize][3 as libc::c_int as
                                                               usize];
            t =
                *verts.offset(0 as libc::c_int as isize) *
                    (*(*warpface).texinfo).vecs[1 as libc::c_int as
                                                    usize][0 as libc::c_int as
                                                               usize] +
                    *verts.offset(1 as libc::c_int as isize) *
                        (*(*warpface).texinfo).vecs[1 as libc::c_int as
                                                        usize][1 as
                                                                   libc::c_int
                                                                   as usize] +
                    *verts.offset(2 as libc::c_int as isize) *
                        (*(*warpface).texinfo).vecs[1 as libc::c_int as
                                                        usize][2 as
                                                                   libc::c_int
                                                                   as usize] +
                    (*(*warpface).texinfo).vecs[1 as libc::c_int as
                                                    usize][3 as libc::c_int as
                                                               usize];
            s /= (*(*(*warpface).texinfo).texture).width as libc::c_float;
            t /= (*(*(*warpface).texinfo).texture).height as libc::c_float
        }
        (*poly).verts[i as usize][3 as libc::c_int as usize] = s;
        (*poly).verts[i as usize][4 as libc::c_int as usize] = t;
        // for speed reasons
        if (*warpface).flags as libc::c_uint &
               (1 as libc::c_uint) << 4 as libc::c_int == 0 {
            // lightmap texture coordinates
            s =
                *verts.offset(0 as libc::c_int as isize) *
                    (*warpinfo).lmvecs[0 as libc::c_int as
                                           usize][0 as libc::c_int as usize] +
                    *verts.offset(1 as libc::c_int as isize) *
                        (*warpinfo).lmvecs[0 as libc::c_int as
                                               usize][1 as libc::c_int as
                                                          usize] +
                    *verts.offset(2 as libc::c_int as isize) *
                        (*warpinfo).lmvecs[0 as libc::c_int as
                                               usize][2 as libc::c_int as
                                                          usize] +
                    (*warpinfo).lmvecs[0 as libc::c_int as
                                           usize][3 as libc::c_int as
                                                      usize]; //fa->texinfo->texture->width;
            s -=
                (*warpinfo).lightmapmins[0 as libc::c_int as usize] as
                    libc::c_int as
                    libc::c_float; //fa->texinfo->texture->height;
            s += (*warpface).light_s as libc::c_float * sample_size;
            s += sample_size * 0.5f32;
            s /= tr.block_size as libc::c_float * sample_size;
            t =
                *verts.offset(0 as libc::c_int as isize) *
                    (*warpinfo).lmvecs[1 as libc::c_int as
                                           usize][0 as libc::c_int as usize] +
                    *verts.offset(1 as libc::c_int as isize) *
                        (*warpinfo).lmvecs[1 as libc::c_int as
                                               usize][1 as libc::c_int as
                                                          usize] +
                    *verts.offset(2 as libc::c_int as isize) *
                        (*warpinfo).lmvecs[1 as libc::c_int as
                                               usize][2 as libc::c_int as
                                                          usize] +
                    (*warpinfo).lmvecs[1 as libc::c_int as
                                           usize][3 as libc::c_int as usize];
            t -=
                (*warpinfo).lightmapmins[1 as libc::c_int as usize] as
                    libc::c_int as libc::c_float;
            t += (*warpface).light_t as libc::c_float * sample_size;
            t += sample_size * 0.5f32;
            t /= tr.block_size as libc::c_float * sample_size;
            (*poly).verts[i as usize][5 as libc::c_int as usize] = s;
            (*poly).verts[i as usize][6 as libc::c_int as usize] = t
        }
        i += 1;
        verts = verts.offset(3 as libc::c_int as isize)
    };
}
#[no_mangle]
pub unsafe extern "C" fn GL_SetupFogColorForSurfaces() {
    let mut fogColor: vec3_t = [0.; 3];
    let mut factor: libc::c_float = 0.;
    let mut div: libc::c_float = 0.;
    if glState.isFogEnabled == 0 { return }
    if !RI.currententity.is_null() &&
           (*RI.currententity).curstate.rendermode ==
               kRenderTransTexture as libc::c_int {
        pglFogfv.expect("non-null function pointer")(0xb66 as libc::c_int as
                                                         GLenum,
                                                     RI.fogColor.as_mut_ptr());
        return
    }
    div = if (*r_detailtextures).value != 0. { 2.0f32 } else { 1.0f32 };
    factor = if (*r_detailtextures).value != 0. { 3.0f32 } else { 2.0f32 };
    fogColor[0 as libc::c_int as usize] =
        __tg_pow(RI.fogColor[0 as libc::c_int as usize] / div,
                 1.0f32 / factor);
    fogColor[1 as libc::c_int as usize] =
        __tg_pow(RI.fogColor[1 as libc::c_int as usize] / div,
                 1.0f32 / factor);
    fogColor[2 as libc::c_int as usize] =
        __tg_pow(RI.fogColor[2 as libc::c_int as usize] / div,
                 1.0f32 / factor);
    pglFogfv.expect("non-null function pointer")(0xb66 as libc::c_int as
                                                     GLenum,
                                                 fogColor.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn GL_ResetFogColor() {
    // restore fog here
    if glState.isFogEnabled != 0 {
        pglFogfv.expect("non-null function pointer")(0xb66 as libc::c_int as
                                                         GLenum,
                                                     RI.fogColor.as_mut_ptr());
    };
}
/*
================
GL_SubdivideSurface

Breaks a polygon up along axial 64 unit
boundaries so that turbulent and sky warps
can be done reasonably.
================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_SubdivideSurface(mut fa: *mut msurface_t) {
    let mut verts: [vec3_t; 64] = [[0.; 3]; 64];
    let mut numverts: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut lindex: libc::c_int = 0;
    let mut vec: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut loadmodel: *mut model_t =
        gEngfuncs.Mod_GetCurrentLoadingModel.expect("non-null function pointer")();
    // convert edges back to a normal polygon
    numverts = 0 as libc::c_int; // predict state
    i = 0 as libc::c_int;
    while i < (*fa).numedges {
        lindex =
            *(*loadmodel).surfedges.offset(((*fa).firstedge + i) as isize);
        if lindex > 0 as libc::c_int {
            vec =
                (*(*loadmodel).vertexes.offset((*(*loadmodel).edges.offset(lindex
                                                                               as
                                                                               isize)).v[0
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             usize]
                                                   as
                                                   isize)).position.as_mut_ptr()
        } else {
            vec =
                (*(*loadmodel).vertexes.offset((*(*loadmodel).edges.offset(-lindex
                                                                               as
                                                                               isize)).v[1
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             usize]
                                                   as
                                                   isize)).position.as_mut_ptr()
        }
        verts[numverts as usize][0 as libc::c_int as usize] =
            *vec.offset(0 as libc::c_int as isize);
        verts[numverts as usize][1 as libc::c_int as usize] =
            *vec.offset(1 as libc::c_int as isize);
        verts[numverts as usize][2 as libc::c_int as usize] =
            *vec.offset(2 as libc::c_int as isize);
        numverts += 1;
        i += 1
    }
    (*fa).flags =
        ((*fa).flags as libc::c_uint |
             (1 as libc::c_uint) << 3 as libc::c_int) as libc::c_int;
    // do subdivide
    SubdividePolygon_r(fa, numverts,
                       verts[0 as libc::c_int as usize].as_mut_ptr());
}
/*
================
GL_BuildPolygonFromSurface
================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_BuildPolygonFromSurface(mut mod_0: *mut model_t,
                                                    mut fa: *mut msurface_t) {
    let mut i: libc::c_int = 0; // bad polygon ?
    let mut lindex: libc::c_int = 0;
    let mut lnumverts: libc::c_int = 0;
    let mut pedges: *mut medge_t = 0 as *mut medge_t;
    let mut r_pedge: *mut medge_t = 0 as *mut medge_t;
    let mut info: *mut mextrasurf_t = (*fa).info;
    let mut sample_size: libc::c_float = 0.;
    let mut tex: *mut texture_t = 0 as *mut texture_t;
    let mut glt: *mut gl_texture_t = 0 as *mut gl_texture_t;
    let mut vec: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut s: libc::c_float = 0.;
    let mut t: libc::c_float = 0.;
    let mut poly: *mut glpoly_t = 0 as *mut glpoly_t;
    if mod_0.is_null() || (*fa).texinfo.is_null() ||
           (*(*fa).texinfo).texture.is_null() {
        return
    }
    if (*fa).flags as libc::c_uint & (1 as libc::c_uint) << 6 as libc::c_int
           != 0 &&
           (*(*(*fa).texinfo).texture).gl_texturenum != 0 as libc::c_int {
        glt =
            R_GetTexture((*(*(*fa).texinfo).texture).gl_texturenum as GLenum);
        tex = (*(*fa).texinfo).texture;
        if !(!glt.is_null() && !tex.is_null()) {
            gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     b"../ref_gl/gl_rsurf.c\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     291 as
                                                                         libc::c_int);
        }
        // update conveyor widths for keep properly speed of scrolling
        (*glt).srcWidth = (*tex).width as word;
        (*glt).srcHeight = (*tex).height as word
    }
    sample_size =
        gEngfuncs.Mod_SampleSizeForFace.expect("non-null function pointer")(fa)
            as libc::c_float;
    // reconstruct the polygon
    pedges = (*mod_0).edges;
    lnumverts = (*fa).numedges;
    // detach if already created, reconstruct again
    poly = (*fa).polys;
    (*fa).polys = 0 as *mut glpoly_t;
    // quake simple models (healthkits etc) need to be reconstructed their polys because LM coords has changed after the map change
    poly =
        gEngfuncs._Mem_Realloc.expect("non-null function pointer")((*mod_0).mempool,
                                                                   poly as
                                                                       *mut libc::c_void,
                                                                   (::std::mem::size_of::<glpoly_t>()
                                                                        as
                                                                        libc::c_ulong).wrapping_add((((lnumverts
                                                                                                           -
                                                                                                           4
                                                                                                               as
                                                                                                               libc::c_int)
                                                                                                          *
                                                                                                          7
                                                                                                              as
                                                                                                              libc::c_int)
                                                                                                         as
                                                                                                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_float>()
                                                                                                                                         as
                                                                                                                                         libc::c_ulong)),
                                                                   true_0,
                                                                   b"../ref_gl/gl_rsurf.c\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char,
                                                                   309 as
                                                                       libc::c_int)
            as *mut glpoly_t;
    (*poly).next = (*fa).polys;
    (*poly).flags = (*fa).flags;
    (*fa).polys = poly;
    (*poly).numverts = lnumverts;
    i = 0 as libc::c_int;
    while i < lnumverts {
        lindex = *(*mod_0).surfedges.offset(((*fa).firstedge + i) as isize);
        if lindex > 0 as libc::c_int {
            r_pedge = &mut *pedges.offset(lindex as isize) as *mut medge_t;
            vec =
                (*(*mod_0).vertexes.offset((*r_pedge).v[0 as libc::c_int as
                                                            usize] as
                                               isize)).position.as_mut_ptr()
        } else {
            r_pedge = &mut *pedges.offset(-lindex as isize) as *mut medge_t;
            vec =
                (*(*mod_0).vertexes.offset((*r_pedge).v[1 as libc::c_int as
                                                            usize] as
                                               isize)).position.as_mut_ptr()
        }
        s =
            *vec.offset(0 as libc::c_int as isize) *
                (*(*fa).texinfo).vecs[0 as libc::c_int as
                                          usize][0 as libc::c_int as usize] +
                *vec.offset(1 as libc::c_int as isize) *
                    (*(*fa).texinfo).vecs[0 as libc::c_int as
                                              usize][1 as libc::c_int as
                                                         usize] +
                *vec.offset(2 as libc::c_int as isize) *
                    (*(*fa).texinfo).vecs[0 as libc::c_int as
                                              usize][2 as libc::c_int as
                                                         usize] +
                (*(*fa).texinfo).vecs[0 as libc::c_int as
                                          usize][3 as libc::c_int as usize];
        s /= (*(*(*fa).texinfo).texture).width as libc::c_float;
        t =
            *vec.offset(0 as libc::c_int as isize) *
                (*(*fa).texinfo).vecs[1 as libc::c_int as
                                          usize][0 as libc::c_int as usize] +
                *vec.offset(1 as libc::c_int as isize) *
                    (*(*fa).texinfo).vecs[1 as libc::c_int as
                                              usize][1 as libc::c_int as
                                                         usize] +
                *vec.offset(2 as libc::c_int as isize) *
                    (*(*fa).texinfo).vecs[1 as libc::c_int as
                                              usize][2 as libc::c_int as
                                                         usize] +
                (*(*fa).texinfo).vecs[1 as libc::c_int as
                                          usize][3 as libc::c_int as usize];
        t /= (*(*(*fa).texinfo).texture).height as libc::c_float;
        (*poly).verts[i as usize][0 as libc::c_int as usize] =
            *vec.offset(0 as libc::c_int as isize);
        (*poly).verts[i as usize][1 as libc::c_int as usize] =
            *vec.offset(1 as libc::c_int as isize);
        (*poly).verts[i as usize][2 as libc::c_int as usize] =
            *vec.offset(2 as libc::c_int as isize);
        (*poly).verts[i as usize][3 as libc::c_int as usize] = s;
        (*poly).verts[i as usize][4 as libc::c_int as usize] = t;
        // lightmap texture coordinates
        s =
            *vec.offset(0 as libc::c_int as isize) *
                (*info).lmvecs[0 as libc::c_int as
                                   usize][0 as libc::c_int as usize] +
                *vec.offset(1 as libc::c_int as isize) *
                    (*info).lmvecs[0 as libc::c_int as
                                       usize][1 as libc::c_int as usize] +
                *vec.offset(2 as libc::c_int as isize) *
                    (*info).lmvecs[0 as libc::c_int as
                                       usize][2 as libc::c_int as usize] +
                (*info).lmvecs[0 as libc::c_int as
                                   usize][3 as libc::c_int as
                                              usize]; //fa->texinfo->texture->width;
        s -=
            (*info).lightmapmins[0 as libc::c_int as usize] as libc::c_int as
                libc::c_float; //fa->texinfo->texture->height;
        s += (*fa).light_s as libc::c_float * sample_size;
        s += sample_size * 0.5f32;
        s /= tr.block_size as libc::c_float * sample_size;
        t =
            *vec.offset(0 as libc::c_int as isize) *
                (*info).lmvecs[1 as libc::c_int as
                                   usize][0 as libc::c_int as usize] +
                *vec.offset(1 as libc::c_int as isize) *
                    (*info).lmvecs[1 as libc::c_int as
                                       usize][1 as libc::c_int as usize] +
                *vec.offset(2 as libc::c_int as isize) *
                    (*info).lmvecs[1 as libc::c_int as
                                       usize][2 as libc::c_int as usize] +
                (*info).lmvecs[1 as libc::c_int as
                                   usize][3 as libc::c_int as usize];
        t -=
            (*info).lightmapmins[1 as libc::c_int as usize] as libc::c_int as
                libc::c_float;
        t += (*fa).light_t as libc::c_float * sample_size;
        t += sample_size * 0.5f32;
        t /= tr.block_size as libc::c_float * sample_size;
        (*poly).verts[i as usize][5 as libc::c_int as usize] = s;
        (*poly).verts[i as usize][6 as libc::c_int as usize] = t;
        i += 1
    }
    // remove co-linear points - Ed
    if (if !gl_keeptjunctions.is_null() &&
               (*gl_keeptjunctions).value != 0.0f32 {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int }) == 0 &&
           (*fa).flags as libc::c_uint &
               (1 as libc::c_uint) << 7 as libc::c_int == 0 {
        i = 0 as libc::c_int;
        while i < lnumverts {
            let mut v1: vec3_t = [0.; 3];
            let mut v2: vec3_t = [0.; 3];
            let mut prev: *mut libc::c_float = 0 as *mut libc::c_float;
            let mut this: *mut libc::c_float = 0 as *mut libc::c_float;
            let mut next: *mut libc::c_float = 0 as *mut libc::c_float;
            prev =
                (*poly).verts[((i + lnumverts - 1 as libc::c_int) % lnumverts)
                                  as usize].as_mut_ptr();
            next =
                (*poly).verts[((i + 1 as libc::c_int) % lnumverts) as
                                  usize].as_mut_ptr();
            this = (*poly).verts[i as usize].as_mut_ptr();
            v1[0 as libc::c_int as usize] =
                *this.offset(0 as libc::c_int as isize) -
                    *prev.offset(0 as libc::c_int as isize);
            v1[1 as libc::c_int as usize] =
                *this.offset(1 as libc::c_int as isize) -
                    *prev.offset(1 as libc::c_int as isize);
            v1[2 as libc::c_int as usize] =
                *this.offset(2 as libc::c_int as isize) -
                    *prev.offset(2 as libc::c_int as isize);
            let mut ilength: libc::c_float =
                __tg_sqrt(v1[0 as libc::c_int as usize] *
                              v1[0 as libc::c_int as usize] +
                              v1[1 as libc::c_int as usize] *
                                  v1[1 as libc::c_int as usize] +
                              v1[2 as libc::c_int as usize] *
                                  v1[2 as libc::c_int as usize]);
            if ilength != 0. { ilength = 1.0f32 / ilength }
            v1[0 as libc::c_int as usize] *= ilength;
            v1[1 as libc::c_int as usize] *= ilength;
            v1[2 as libc::c_int as usize] *= ilength;
            v2[0 as libc::c_int as usize] =
                *next.offset(0 as libc::c_int as isize) -
                    *prev.offset(0 as libc::c_int as isize);
            v2[1 as libc::c_int as usize] =
                *next.offset(1 as libc::c_int as isize) -
                    *prev.offset(1 as libc::c_int as isize);
            v2[2 as libc::c_int as usize] =
                *next.offset(2 as libc::c_int as isize) -
                    *prev.offset(2 as libc::c_int as isize);
            let mut ilength_0: libc::c_float =
                __tg_sqrt(v2[0 as libc::c_int as usize] *
                              v2[0 as libc::c_int as usize] +
                              v2[1 as libc::c_int as usize] *
                                  v2[1 as libc::c_int as usize] +
                              v2[2 as libc::c_int as usize] *
                                  v2[2 as libc::c_int as usize]);
            if ilength_0 != 0. { ilength_0 = 1.0f32 / ilength_0 }
            v2[0 as libc::c_int as usize] *= ilength_0;
            v2[1 as libc::c_int as usize] *= ilength_0;
            v2[2 as libc::c_int as usize] *= ilength_0;
            // skip co-linear points
            if __tg_fabs(v1[0 as libc::c_int as usize] -
                             v2[0 as libc::c_int as usize]) <= 0.001f32 &&
                   __tg_fabs(v1[1 as libc::c_int as usize] -
                                 v2[1 as libc::c_int as usize]) <= 0.001f32 &&
                   __tg_fabs(v1[2 as libc::c_int as usize] -
                                 v2[2 as libc::c_int as usize]) <= 0.001f32 {
                let mut j: libc::c_int = 0;
                let mut k: libc::c_int = 0;
                j = i + 1 as libc::c_int;
                while j < lnumverts {
                    k = 0 as libc::c_int;
                    while k < 7 as libc::c_int {
                        (*poly).verts[(j - 1 as libc::c_int) as
                                          usize][k as usize] =
                            (*poly).verts[j as usize][k as usize];
                        k += 1
                    }
                    j += 1
                }
                // retry next vertex next time, which is now current vertex
                lnumverts -= 1;
                nColinElim += 1;
                i -= 1
            }
            i += 1
        }
    }
    (*poly).numverts = lnumverts;
}
/*
===============
R_TextureAnim

Returns the proper texture for a given time and base texture, do not process random tiling
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_TextureAnim(mut b: *mut texture_t)
 -> *mut texture_t {
    let mut base: *mut texture_t = b;
    let mut count: libc::c_int = 0;
    let mut reletive: libc::c_int = 0;
    if (*RI.currententity).curstate.frame != 0. {
        if !(*base).alternate_anims.is_null() {
            base = (*base).alternate_anims
        }
    }
    if (*base).anim_total == 0 { return base }
    if (*base).name[0 as libc::c_int as usize] as libc::c_int == '-' as i32 {
        return b
        // already tiled
    } else {
        let mut speed: libc::c_int = 0;
        // Quake1 textures uses 10 frames per second
        if (*R_GetTexture((*base).gl_texturenum as GLenum)).flags as
               libc::c_uint & TF_QUAKEPAL as libc::c_int as libc::c_uint != 0
           {
            speed = 10 as libc::c_int
        } else { speed = 20 as libc::c_int }
        reletive =
            ((*gpGlobals).time * speed as libc::c_float) as libc::c_int %
                (*base).anim_total
    }
    count = 0 as libc::c_int;
    while (*base).anim_min > reletive || (*base).anim_max <= reletive {
        base = (*base).anim_next;
        if base.is_null() || { count += 1; (count) > 20 as libc::c_int } {
            return b
        }
    }
    return base;
}
/*
===============
R_TextureAnimation

Returns the proper texture for a given time and surface
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_TextureAnimation(mut s: *mut msurface_t)
 -> *mut texture_t {
    let mut base: *mut texture_t = (*(*s).texinfo).texture;
    let mut count: libc::c_int = 0;
    let mut reletive: libc::c_int = 0;
    if !RI.currententity.is_null() && (*RI.currententity).curstate.frame != 0.
       {
        if !(*base).alternate_anims.is_null() {
            base = (*base).alternate_anims
        }
    }
    if (*base).anim_total == 0 { return base }
    if (*base).name[0 as libc::c_int as usize] as libc::c_int == '-' as i32 {
        let mut tx: libc::c_int =
            ((*s).texturemins[0 as libc::c_int as usize] as
                 libc::c_uint).wrapping_add((*base).width <<
                                                16 as
                                                    libc::c_int).wrapping_div((*base).width)
                as libc::c_int % 20 as libc::c_int;
        let mut ty: libc::c_int =
            ((*s).texturemins[1 as libc::c_int as usize] as
                 libc::c_uint).wrapping_add((*base).height <<
                                                16 as
                                                    libc::c_int).wrapping_div((*base).height)
                as libc::c_int % 20 as libc::c_int;
        reletive = rtable[tx as usize][ty as usize] % (*base).anim_total
    } else {
        let mut speed: libc::c_int = 0;
        // Quake1 textures uses 10 frames per second
        if (*R_GetTexture((*base).gl_texturenum as GLenum)).flags as
               libc::c_uint & TF_QUAKEPAL as libc::c_int as libc::c_uint != 0
           {
            speed = 10 as libc::c_int
        } else { speed = 20 as libc::c_int }
        reletive =
            ((*gpGlobals).time * speed as libc::c_float) as libc::c_int %
                (*base).anim_total
    }
    count = 0 as libc::c_int;
    while (*base).anim_min > reletive || (*base).anim_max <= reletive {
        base = (*base).anim_next;
        if base.is_null() || { count += 1; (count) > 20 as libc::c_int } {
            return (*(*s).texinfo).texture
        }
    }
    return base;
}
/*
===============
R_AddDynamicLights
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_AddDynamicLights(mut surf: *mut msurface_t) {
    let mut dist: libc::c_float = 0.;
    let mut rad: libc::c_float = 0.;
    let mut minlight: libc::c_float = 0.;
    let mut lnum: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut sd: libc::c_int = 0;
    let mut td: libc::c_int = 0;
    let mut smax: libc::c_int = 0;
    let mut tmax: libc::c_int = 0;
    let mut sl: libc::c_float = 0.;
    let mut tl: libc::c_float = 0.;
    let mut sacc: libc::c_float = 0.;
    let mut tacc: libc::c_float = 0.;
    let mut impact: vec3_t = [0.; 3];
    let mut origin_l: vec3_t = [0.; 3];
    let mut info: *mut mextrasurf_t = (*surf).info;
    let mut sample_frac: libc::c_int = 1.0f64 as libc::c_int;
    let mut sample_size: libc::c_float = 0.;
    let mut tex: *mut mtexinfo_t = 0 as *mut mtexinfo_t;
    let mut dl: *mut dlight_t = 0 as *mut dlight_t;
    let mut bl: *mut uint = 0 as *mut uint;
    // no dlighted surfaces here
    if R_CountSurfaceDlights(surf) == 0 { return } // not lit by this light
    sample_size =
        gEngfuncs.Mod_SampleSizeForFace.expect("non-null function pointer")(surf)
            as libc::c_float;
    smax =
        ((*info).lightextents[0 as libc::c_int as usize] as libc::c_int as
             libc::c_float / sample_size + 1 as libc::c_int as libc::c_float)
            as libc::c_int;
    tmax =
        ((*info).lightextents[1 as libc::c_int as usize] as libc::c_int as
             libc::c_float / sample_size + 1 as libc::c_int as libc::c_float)
            as libc::c_int;
    tex = (*surf).texinfo;
    if (*tex).flags as libc::c_uint & (1 as libc::c_uint) << 1 as libc::c_int
           != 0 {
        if !(*(*surf).texinfo).faceinfo.is_null() {
            sample_frac =
                (*(*(*surf).texinfo).faceinfo).texture_step as libc::c_int
        } else if (*(*surf).texinfo).flags as libc::c_uint &
                      (1 as libc::c_uint) << 3 as libc::c_int != 0 {
            sample_frac = 8 as libc::c_int
        } else { sample_frac = 16 as libc::c_int }
    }
    lnum = 0 as libc::c_int;
    while lnum < 32 as libc::c_int {
        if !((*surf).dlightbits as libc::c_uint & (1 as libc::c_uint) << lnum
                 == 0) {
            dl =
                gEngfuncs.GetDynamicLight.expect("non-null function pointer")(lnum);
            // transform light origin to local bmodel space
            if tr.modelviewIdentity as u64 == 0 {
                Matrix4x4_VectorITransform(RI.objectMatrix.as_mut_ptr() as
                                               *const [vec_t; 4],
                                           (*dl).origin.as_mut_ptr() as
                                               *const libc::c_float,
                                           origin_l.as_mut_ptr());
            } else {
                origin_l[0 as libc::c_int as usize] =
                    (*dl).origin[0 as libc::c_int as usize];
                origin_l[1 as libc::c_int as usize] =
                    (*dl).origin[1 as libc::c_int as usize];
                origin_l[2 as libc::c_int as usize] =
                    (*dl).origin[2 as libc::c_int as usize]
            }
            rad = (*dl).radius;
            dist =
                (if ((*(*surf).plane).type_0 as libc::c_int) <
                        3 as libc::c_int {
                     origin_l[(*(*surf).plane).type_0 as usize]
                 } else {
                     (origin_l[0 as libc::c_int as usize] *
                          (*(*surf).plane).normal[0 as libc::c_int as usize] +
                          origin_l[1 as libc::c_int as usize] *
                              (*(*surf).plane).normal[1 as libc::c_int as
                                                          usize]) +
                         origin_l[2 as libc::c_int as usize] *
                             (*(*surf).plane).normal[2 as libc::c_int as
                                                         usize]
                 }) - (*(*surf).plane).dist;
            rad -= __tg_fabs(dist);
            // rad is now the highest intensity on the plane
            minlight = (*dl).minlight;
            if !(rad < minlight) {
                minlight = rad - minlight;
                if ((*(*surf).plane).type_0 as libc::c_int) < 3 as libc::c_int
                   {
                    impact[0 as libc::c_int as usize] =
                        origin_l[0 as libc::c_int as usize];
                    impact[1 as libc::c_int as usize] =
                        origin_l[1 as libc::c_int as usize];
                    impact[2 as libc::c_int as usize] =
                        origin_l[2 as libc::c_int as usize];
                    impact[(*(*surf).plane).type_0 as usize] -= dist
                } else {
                    impact[0 as libc::c_int as usize] =
                        origin_l[0 as libc::c_int as usize] +
                            -dist *
                                (*(*surf).plane).normal[0 as libc::c_int as
                                                            usize];
                    impact[1 as libc::c_int as usize] =
                        origin_l[1 as libc::c_int as usize] +
                            -dist *
                                (*(*surf).plane).normal[1 as libc::c_int as
                                                            usize];
                    impact[2 as libc::c_int as usize] =
                        origin_l[2 as libc::c_int as usize] +
                            -dist *
                                (*(*surf).plane).normal[2 as libc::c_int as
                                                            usize]
                }
                sl =
                    impact[0 as libc::c_int as usize] *
                        (*info).lmvecs[0 as libc::c_int as
                                           usize][0 as libc::c_int as usize] +
                        impact[1 as libc::c_int as usize] *
                            (*info).lmvecs[0 as libc::c_int as
                                               usize][1 as libc::c_int as
                                                          usize] +
                        impact[2 as libc::c_int as usize] *
                            (*info).lmvecs[0 as libc::c_int as
                                               usize][2 as libc::c_int as
                                                          usize] +
                        (*info).lmvecs[0 as libc::c_int as
                                           usize][3 as libc::c_int as usize] -
                        (*info).lightmapmins[0 as libc::c_int as usize] as
                            libc::c_int as libc::c_float;
                tl =
                    impact[0 as libc::c_int as usize] *
                        (*info).lmvecs[1 as libc::c_int as
                                           usize][0 as libc::c_int as usize] +
                        impact[1 as libc::c_int as usize] *
                            (*info).lmvecs[1 as libc::c_int as
                                               usize][1 as libc::c_int as
                                                          usize] +
                        impact[2 as libc::c_int as usize] *
                            (*info).lmvecs[1 as libc::c_int as
                                               usize][2 as libc::c_int as
                                                          usize] +
                        (*info).lmvecs[1 as libc::c_int as
                                           usize][3 as libc::c_int as usize] -
                        (*info).lightmapmins[1 as libc::c_int as usize] as
                            libc::c_int as libc::c_float;
                bl = r_blocklights.as_mut_ptr();
                t = 0 as libc::c_int;
                tacc = 0 as libc::c_int as libc::c_float;
                while t < tmax {
                    td =
                        ((tl - tacc) * sample_frac as libc::c_float) as
                            libc::c_int;
                    if td < 0 as libc::c_int { td = -td }
                    s = 0 as libc::c_int;
                    sacc = 0 as libc::c_int as libc::c_float;
                    while s < smax {
                        sd =
                            ((sl - sacc) * sample_frac as libc::c_float) as
                                libc::c_int;
                        if sd < 0 as libc::c_int { sd = -sd }
                        if sd > td {
                            dist =
                                (sd + (td >> 1 as libc::c_int)) as
                                    libc::c_float
                        } else {
                            dist =
                                (td + (sd >> 1 as libc::c_int)) as
                                    libc::c_float
                        }
                        if dist < minlight {
                            let ref mut fresh0 =
                                *bl.offset(0 as libc::c_int as isize);
                            *fresh0 =
                                (*fresh0 as
                                     libc::c_uint).wrapping_add((((rad - dist)
                                                                      *
                                                                      256 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_float)
                                                                     as
                                                                     libc::c_int
                                                                     *
                                                                     gEngfuncs.LightToTexGamma.expect("non-null function pointer")((*dl).color.r)
                                                                         as
                                                                         libc::c_int
                                                                     /
                                                                     256 as
                                                                         libc::c_int)
                                                                    as
                                                                    libc::c_uint)
                                    as uint as uint;
                            let ref mut fresh1 =
                                *bl.offset(1 as libc::c_int as isize);
                            *fresh1 =
                                (*fresh1 as
                                     libc::c_uint).wrapping_add((((rad - dist)
                                                                      *
                                                                      256 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_float)
                                                                     as
                                                                     libc::c_int
                                                                     *
                                                                     gEngfuncs.LightToTexGamma.expect("non-null function pointer")((*dl).color.g)
                                                                         as
                                                                         libc::c_int
                                                                     /
                                                                     256 as
                                                                         libc::c_int)
                                                                    as
                                                                    libc::c_uint)
                                    as uint as uint;
                            let ref mut fresh2 =
                                *bl.offset(2 as libc::c_int as isize);
                            *fresh2 =
                                (*fresh2 as
                                     libc::c_uint).wrapping_add((((rad - dist)
                                                                      *
                                                                      256 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_float)
                                                                     as
                                                                     libc::c_int
                                                                     *
                                                                     gEngfuncs.LightToTexGamma.expect("non-null function pointer")((*dl).color.b)
                                                                         as
                                                                         libc::c_int
                                                                     /
                                                                     256 as
                                                                         libc::c_int)
                                                                    as
                                                                    libc::c_uint)
                                    as uint as uint
                        }
                        s += 1;
                        sacc += sample_size;
                        bl = bl.offset(3 as libc::c_int as isize)
                    }
                    t += 1;
                    tacc += sample_size
                }
            }
        }
        lnum += 1
    };
}
/*
================
R_SetCacheState
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_SetCacheState(mut surf: *mut msurface_t) {
    let mut maps: libc::c_int = 0;
    maps = 0 as libc::c_int;
    while maps < 4 as libc::c_int &&
              (*surf).styles[maps as usize] as libc::c_int !=
                  255 as libc::c_int {
        (*surf).cached_light[maps as usize] =
            tr.lightstylevalue[(*surf).styles[maps as usize] as usize];
        maps += 1
    };
}
/*
=============================================================================

  LIGHTMAP ALLOCATION

=============================================================================
*/
unsafe extern "C" fn LM_InitBlock() {
    memset(gl_lms.allocated.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<[libc::c_int; 1024]>() as libc::c_ulong);
}
unsafe extern "C" fn LM_AllocBlock(mut w: libc::c_int, mut h: libc::c_int,
                                   mut x: *mut libc::c_int,
                                   mut y: *mut libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut best: libc::c_int = 0;
    let mut best2: libc::c_int = 0;
    best = tr.block_size;
    i = 0 as libc::c_int;
    while i < tr.block_size - w {
        best2 = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < w {
            if gl_lms.allocated[(i + j) as usize] >= best { break ; }
            if gl_lms.allocated[(i + j) as usize] > best2 {
                best2 = gl_lms.allocated[(i + j) as usize]
            }
            j += 1
        }
        if j == w {
            // this is a valid spot
            *x = i;
            best = best2;
            *y = best
        }
        i += 1
    }
    if best + h > tr.block_size { return false_0 as libc::c_int }
    i = 0 as libc::c_int;
    while i < w { gl_lms.allocated[(*x + i) as usize] = best + h; i += 1 }
    return true_0 as libc::c_int;
}
unsafe extern "C" fn LM_UploadDynamicBlock() {
    let mut height: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < tr.block_size {
        if gl_lms.allocated[i as usize] > height {
            height = gl_lms.allocated[i as usize]
        }
        i += 1
    }
    pglTexSubImage2D.expect("non-null function pointer")(0xde1 as libc::c_int
                                                             as GLenum,
                                                         0 as libc::c_int,
                                                         0 as libc::c_int,
                                                         0 as libc::c_int,
                                                         tr.block_size,
                                                         height,
                                                         0x1908 as libc::c_int
                                                             as GLenum,
                                                         0x1401 as libc::c_int
                                                             as GLenum,
                                                         gl_lms.lightmap_buffer.as_mut_ptr()
                                                             as
                                                             *const libc::c_void);
}
unsafe extern "C" fn LM_UploadBlock(mut dynamic: qboolean) {
    let mut i: libc::c_int = 0;
    if dynamic as u64 != 0 {
        let mut height: libc::c_int = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < tr.block_size {
            if gl_lms.allocated[i as usize] > height {
                height = gl_lms.allocated[i as usize]
            }
            i += 1
        }
        GL_Bind(XASH_TEXTURE0 as libc::c_int, tr.dlightTexture as GLenum);
        pglTexSubImage2D.expect("non-null function pointer")(0xde1 as
                                                                 libc::c_int
                                                                 as GLenum,
                                                             0 as libc::c_int,
                                                             0 as libc::c_int,
                                                             0 as libc::c_int,
                                                             tr.block_size,
                                                             height,
                                                             0x1908 as
                                                                 libc::c_int
                                                                 as GLenum,
                                                             0x1401 as
                                                                 libc::c_int
                                                                 as GLenum,
                                                             gl_lms.lightmap_buffer.as_mut_ptr()
                                                                 as
                                                                 *const libc::c_void);
    } else {
        let mut r_lightmap_0: rgbdata_t =
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
        let mut lmName: [libc::c_char; 16] = [0; 16];
        i = gl_lms.current_lightmap_texture;
        // upload static lightmaps only during loading
        memset(&mut r_lightmap_0 as *mut rgbdata_t as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<rgbdata_t>() as libc::c_ulong);
        Q_snprintf(lmName.as_mut_ptr(),
                   ::std::mem::size_of::<[libc::c_char; 16]>() as
                       libc::c_ulong,
                   b"*lightmap%i\x00" as *const u8 as *const libc::c_char, i);
        r_lightmap_0.width = tr.block_size as word;
        r_lightmap_0.height = tr.block_size as word;
        r_lightmap_0.type_0 = PF_RGBA_32 as libc::c_int as uint;
        r_lightmap_0.size =
            (r_lightmap_0.width as libc::c_int *
                 r_lightmap_0.height as libc::c_int * 4 as libc::c_int) as
                size_t;
        r_lightmap_0.flags = IMAGE_HAS_COLOR as libc::c_int as uint;
        r_lightmap_0.buffer = gl_lms.lightmap_buffer.as_mut_ptr();
        tr.lightmapTextures[i as usize] =
            GL_LoadTextureFromBuffer(lmName.as_mut_ptr(), &mut r_lightmap_0,
                                     (TF_NOMIPMAP as libc::c_int |
                                          TF_CLAMP as libc::c_int |
                                          TF_ATLAS_PAGE as libc::c_int) as
                                         texFlags_t, false_0);
        gl_lms.current_lightmap_texture += 1;
        if gl_lms.current_lightmap_texture == 256 as libc::c_int {
            gEngfuncs.Host_Error.expect("non-null function pointer")(b"AllocBlock: full\n\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char);
        }
    };
}
/*
=================
R_BuildLightmap

Combine and scale multiple lightmaps into the floating
format in r_blocklights
=================
*/
unsafe extern "C" fn R_BuildLightMap(mut surf: *mut msurface_t,
                                     mut dest: *mut byte,
                                     mut stride: libc::c_int,
                                     mut dynamic: qboolean) {
    let mut smax: libc::c_int = 0;
    let mut tmax: libc::c_int = 0;
    let mut bl: *mut uint = 0 as *mut uint;
    let mut scale: uint = 0;
    let mut i: libc::c_int = 0;
    let mut map: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut sample_size: libc::c_int = 0;
    let mut info: *mut mextrasurf_t = (*surf).info;
    let mut lm: *mut color24 = 0 as *mut color24;
    sample_size =
        gEngfuncs.Mod_SampleSizeForFace.expect("non-null function pointer")(surf);
    smax =
        (*info).lightextents[0 as libc::c_int as usize] as libc::c_int /
            sample_size + 1 as libc::c_int;
    tmax =
        (*info).lightextents[1 as libc::c_int as usize] as libc::c_int /
            sample_size + 1 as libc::c_int;
    size = smax * tmax;
    lm = (*surf).samples;
    memset(r_blocklights.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<uint>() as
                libc::c_ulong).wrapping_mul(size as
                                                libc::c_ulong).wrapping_mul(3
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong));
    // add all the lightmaps
    map = 0 as libc::c_int;
    while map < 4 as libc::c_int &&
              (*surf).styles[map as usize] as libc::c_int !=
                  255 as libc::c_int && !lm.is_null() {
        scale =
            tr.lightstylevalue[(*surf).styles[map as usize] as usize] as uint;
        i = 0 as libc::c_int;
        bl = r_blocklights.as_mut_ptr();
        while i < size {
            let ref mut fresh3 = *bl.offset(0 as libc::c_int as isize);
            *fresh3 =
                (*fresh3 as
                     libc::c_uint).wrapping_add((gEngfuncs.LightToTexGamma.expect("non-null function pointer")((*lm).r)
                                                     as
                                                     libc::c_uint).wrapping_mul(scale))
                    as uint as uint;
            let ref mut fresh4 = *bl.offset(1 as libc::c_int as isize);
            *fresh4 =
                (*fresh4 as
                     libc::c_uint).wrapping_add((gEngfuncs.LightToTexGamma.expect("non-null function pointer")((*lm).g)
                                                     as
                                                     libc::c_uint).wrapping_mul(scale))
                    as uint as uint;
            let ref mut fresh5 = *bl.offset(2 as libc::c_int as isize);
            *fresh5 =
                (*fresh5 as
                     libc::c_uint).wrapping_add((gEngfuncs.LightToTexGamma.expect("non-null function pointer")((*lm).b)
                                                     as
                                                     libc::c_uint).wrapping_mul(scale))
                    as uint as uint;
            i += 1;
            bl = bl.offset(3 as libc::c_int as isize);
            lm = lm.offset(1)
        }
        map += 1
    }
    // add all the dynamic lights
    if (*surf).dlightframe == tr.framecount && dynamic as libc::c_uint != 0 {
        R_AddDynamicLights(surf);
    }
    // Put into texture format
    stride -= smax << 2 as libc::c_int;
    bl = r_blocklights.as_mut_ptr();
    t = 0 as libc::c_int;
    while t < tmax {
        s = 0 as libc::c_int;
        while s < smax {
            *dest.offset(0 as libc::c_int as isize) =
                if (*bl.offset(0 as libc::c_int as isize) >> 7 as libc::c_int)
                       < 255 as libc::c_int as libc::c_uint {
                    (*bl.offset(0 as libc::c_int as isize)) >>
                        7 as libc::c_int
                } else { 255 as libc::c_int as libc::c_uint } as byte;
            *dest.offset(1 as libc::c_int as isize) =
                if (*bl.offset(1 as libc::c_int as isize) >> 7 as libc::c_int)
                       < 255 as libc::c_int as libc::c_uint {
                    (*bl.offset(1 as libc::c_int as isize)) >>
                        7 as libc::c_int
                } else { 255 as libc::c_int as libc::c_uint } as byte;
            *dest.offset(2 as libc::c_int as isize) =
                if (*bl.offset(2 as libc::c_int as isize) >> 7 as libc::c_int)
                       < 255 as libc::c_int as libc::c_uint {
                    (*bl.offset(2 as libc::c_int as isize)) >>
                        7 as libc::c_int
                } else { 255 as libc::c_int as libc::c_uint } as byte;
            *dest.offset(3 as libc::c_int as isize) =
                255 as libc::c_int as byte;
            bl = bl.offset(3 as libc::c_int as isize);
            dest = dest.offset(4 as libc::c_int as isize);
            s += 1
        }
        t += 1;
        dest = dest.offset(stride as isize)
    };
}
/*
================
DrawGLPoly
================
*/
#[no_mangle]
pub unsafe extern "C" fn DrawGLPoly(mut p: *mut glpoly_t,
                                    mut xScale: libc::c_float,
                                    mut yScale: libc::c_float) {
    let mut v: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut sOffset: libc::c_float = 0.;
    let mut sy: libc::c_float = 0.;
    let mut tOffset: libc::c_float = 0.;
    let mut cy: libc::c_float = 0.;
    let mut e: *mut cl_entity_t = RI.currententity;
    let mut i: libc::c_int = 0;
    let mut hasScale: libc::c_int = false_0 as libc::c_int;
    if p.is_null() { return }
    if (*p).flags as libc::c_uint & (1 as libc::c_uint) << 5 as libc::c_int !=
           0 {
        GL_ResetFogColor();
    }
    if (*p).flags as libc::c_uint & (1 as libc::c_uint) << 6 as libc::c_int !=
           0 {
        let mut flConveyorSpeed: libc::c_float = 0.0f32;
        let mut flRate: libc::c_float = 0.;
        let mut flAngle: libc::c_float = 0.;
        let mut texture: *mut gl_texture_t = 0 as *mut gl_texture_t;
        if Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(PARM_QUAKE_COMPATIBLE
                                                                                                                     as
                                                                                                                     libc::c_int,
                                                                                                                 0
                                                                                                                     as
                                                                                                                     libc::c_int)
               != 0 &&
               RI.currententity ==
                   gEngfuncs.GetEntityByIndex.expect("non-null function pointer")(0
                                                                                      as
                                                                                      libc::c_int)
           {
            // same as doom speed
            flConveyorSpeed = -35.0f32
        } else {
            flConveyorSpeed =
                (((*e).curstate.rendercolor.g as libc::c_int) <<
                     8 as libc::c_int |
                     (*e).curstate.rendercolor.b as libc::c_int) as
                    libc::c_float / 16.0f32;
            if (*e).curstate.rendercolor.r != 0 {
                flConveyorSpeed = -flConveyorSpeed
            }
        }
        texture =
            R_GetTexture(glState.currentTextures[glState.activeTMU as usize]
                             as GLenum);
        flRate =
            __tg_fabs(flConveyorSpeed) / (*texture).srcWidth as libc::c_float;
        flAngle =
            if flConveyorSpeed >= 0 as libc::c_int as libc::c_float {
                180 as libc::c_int
            } else { 0 as libc::c_int } as libc::c_float;
        SinCos(flAngle *
                   (3.14159265358979323846f64 as libc::c_float / 180.0f32),
               &mut sy, &mut cy);
        sOffset = (*gpGlobals).time * cy * flRate;
        tOffset = (*gpGlobals).time * sy * flRate;
        // make sure that we are positive
        if sOffset < 0.0f32 {
            sOffset += 1.0f32 + -(sOffset as libc::c_int) as libc::c_float
        }
        if tOffset < 0.0f32 {
            tOffset += 1.0f32 + -(tOffset as libc::c_int) as libc::c_float
        }
        // make sure that we are in a [0,1] range
        sOffset = sOffset - sOffset as libc::c_int as libc::c_float;
        tOffset = tOffset - tOffset as libc::c_int as libc::c_float
    } else { tOffset = 0.0f32; sOffset = tOffset }
    if xScale != 0.0f32 && yScale != 0.0f32 {
        hasScale = true_0 as libc::c_int
    }
    pglBegin.expect("non-null function pointer")(0x9 as libc::c_int as
                                                     GLenum);
    i = 0 as libc::c_int;
    v = (*p).verts[0 as libc::c_int as usize].as_mut_ptr();
    while i < (*p).numverts {
        if hasScale != 0 {
            pglTexCoord2f.expect("non-null function pointer")((*v.offset(3 as
                                                                             libc::c_int
                                                                             as
                                                                             isize)
                                                                   + sOffset)
                                                                  * xScale,
                                                              (*v.offset(4 as
                                                                             libc::c_int
                                                                             as
                                                                             isize)
                                                                   + tOffset)
                                                                  * yScale);
        } else {
            pglTexCoord2f.expect("non-null function pointer")(*v.offset(3 as
                                                                            libc::c_int
                                                                            as
                                                                            isize)
                                                                  + sOffset,
                                                              *v.offset(4 as
                                                                            libc::c_int
                                                                            as
                                                                            isize)
                                                                  + tOffset);
        }
        pglVertex3fv.expect("non-null function pointer")(v);
        i += 1;
        v = v.offset(7 as libc::c_int as isize)
    }
    pglEnd.expect("non-null function pointer")();
    if (*p).flags as libc::c_uint & (1 as libc::c_uint) << 5 as libc::c_int !=
           0 {
        GL_SetupFogColorForSurfaces();
    };
}
/*
================
DrawGLPolyChain

Render lightmaps
================
*/
#[no_mangle]
pub unsafe extern "C" fn DrawGLPolyChain(mut p: *mut glpoly_t,
                                         mut soffset: libc::c_float,
                                         mut toffset: libc::c_float) {
    let mut dynamic: qboolean = true_0; // disabled by user
    if soffset == 0.0f32 && toffset == 0.0f32 { dynamic = false_0 }
    while !p.is_null() {
        let mut v: *mut libc::c_float = 0 as *mut libc::c_float;
        let mut i: libc::c_int = 0;
        pglBegin.expect("non-null function pointer")(0x9 as libc::c_int as
                                                         GLenum);
        v = (*p).verts[0 as libc::c_int as usize].as_mut_ptr();
        i = 0 as libc::c_int;
        while i < (*p).numverts {
            if dynamic as u64 == 0 {
                pglTexCoord2f.expect("non-null function pointer")(*v.offset(5
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize),
                                                                  *v.offset(6
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize));
            } else {
                pglTexCoord2f.expect("non-null function pointer")(*v.offset(5
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize)
                                                                      -
                                                                      soffset,
                                                                  *v.offset(6
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize)
                                                                      -
                                                                      toffset);
            }
            pglVertex3fv.expect("non-null function pointer")(v);
            i += 1;
            v = v.offset(7 as libc::c_int as isize)
        }
        pglEnd.expect("non-null function pointer")();
        p = (*p).chain
    };
}
#[inline]
unsafe extern "C" fn R_HasLightmap() -> qboolean {
    if (if !r_fullbright.is_null() && (*r_fullbright).value != 0.0f32 {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int }) != 0 ||
           (*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                  as
                                                                                  libc::c_int)).lightdata.is_null()
       {
        return false_0
    }
    if !RI.currententity.is_null() {
        if (*RI.currententity).curstate.effects as libc::c_uint &
               (1 as libc::c_uint) << 27 as libc::c_int != 0 {
            return false_0
        }
        // check for rendermode
        match (*RI.currententity).curstate.rendermode {
            2 | 1 | 5 | 3 => {
                return false_0
                // no lightmaps
            }
            _ => { }
        }
    }
    return true_0;
}
/*
================
R_BlendLightmaps
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_BlendLightmaps() {
    let mut surf: *mut msurface_t = 0 as *mut msurface_t;
    let mut newsurf: *mut msurface_t = 0 as *mut msurface_t;
    let mut i: libc::c_int = 0;
    if R_HasLightmap() as u64 == 0 { return }
    GL_SetupFogColorForSurfaces();
    if if !r_lightmap.is_null() && (*r_lightmap).value != 0.0f32 {
           true_0 as libc::c_int
       } else { false_0 as libc::c_int } == 0 {
        pglEnable.expect("non-null function pointer")(0xbe2 as libc::c_int as
                                                          GLenum);
    } else {
        pglDisable.expect("non-null function pointer")(0xbe2 as libc::c_int as
                                                           GLenum);
    }
    // lightmapped solid surfaces
    pglDepthMask.expect("non-null function pointer")(0 as libc::c_int as
                                                         GLboolean);
    pglDepthFunc.expect("non-null function pointer")(0x202 as libc::c_int as
                                                         GLenum);
    pglDisable.expect("non-null function pointer")(0xbc0 as libc::c_int as
                                                       GLenum);
    pglBlendFunc.expect("non-null function pointer")(0 as libc::c_int as
                                                         GLenum,
                                                     0x300 as libc::c_int as
                                                         GLenum);
    pglTexEnvi.expect("non-null function pointer")(0x2300 as libc::c_int as
                                                       GLenum,
                                                   0x2200 as libc::c_int as
                                                       GLenum,
                                                   0x2100 as libc::c_int);
    // render static lightmaps first
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        if !gl_lms.lightmap_surfaces[i as usize].is_null() {
            GL_Bind(XASH_TEXTURE0 as libc::c_int,
                    tr.lightmapTextures[i as usize] as GLenum);
            surf = gl_lms.lightmap_surfaces[i as usize];
            while !surf.is_null() {
                if !(*surf).polys.is_null() {
                    DrawGLPolyChain((*surf).polys, 0.0f32, 0.0f32);
                }
                surf = (*(*surf).info).lightmapchain
            }
        }
        i += 1
    }
    // render dynamic lightmaps
    if if !r_dynamic.is_null() && (*r_dynamic).value != 0.0f32 {
           true_0 as libc::c_int
       } else { false_0 as libc::c_int } != 0 {
        LM_InitBlock();
        GL_Bind(XASH_TEXTURE0 as libc::c_int, tr.dlightTexture as GLenum);
        newsurf = gl_lms.dynamic_surfaces;
        surf = gl_lms.dynamic_surfaces;
        while !surf.is_null() {
            let mut smax: libc::c_int = 0;
            let mut tmax: libc::c_int = 0;
            let mut sample_size: libc::c_int = 0;
            let mut info: *mut mextrasurf_t = (*surf).info;
            let mut base: *mut byte = 0 as *mut byte;
            sample_size =
                gEngfuncs.Mod_SampleSizeForFace.expect("non-null function pointer")(surf);
            smax =
                (*info).lightextents[0 as libc::c_int as usize] as libc::c_int
                    / sample_size + 1 as libc::c_int;
            tmax =
                (*info).lightextents[1 as libc::c_int as usize] as libc::c_int
                    / sample_size + 1 as libc::c_int;
            if LM_AllocBlock(smax, tmax, &mut (*(*surf).info).dlight_s,
                             &mut (*(*surf).info).dlight_t) != 0 {
                base = gl_lms.lightmap_buffer.as_mut_ptr();
                base =
                    base.offset((((*(*surf).info).dlight_t * tr.block_size +
                                      (*(*surf).info).dlight_s) *
                                     4 as libc::c_int) as isize);
                R_BuildLightMap(surf, base, tr.block_size * 4 as libc::c_int,
                                true_0);
            } else {
                let mut drawsurf: *mut msurface_t = 0 as *mut msurface_t;
                // upload what we have so far
                LM_UploadBlock(true_0);
                // draw all surfaces that use this lightmap
                drawsurf = newsurf;
                while drawsurf != surf {
                    if !(*drawsurf).polys.is_null() {
                        DrawGLPolyChain((*drawsurf).polys,
                                        ((*drawsurf).light_s -
                                             (*(*drawsurf).info).dlight_s) as
                                            libc::c_float *
                                            (1.0f32 /
                                                 tr.block_size as
                                                     libc::c_float),
                                        ((*drawsurf).light_t -
                                             (*(*drawsurf).info).dlight_t) as
                                            libc::c_float *
                                            (1.0f32 /
                                                 tr.block_size as
                                                     libc::c_float));
                    }
                    drawsurf = (*(*drawsurf).info).lightmapchain
                }
                newsurf = drawsurf;
                // clear the block
                LM_InitBlock();
                // try uploading the block now
                if LM_AllocBlock(smax, tmax, &mut (*(*surf).info).dlight_s,
                                 &mut (*(*surf).info).dlight_t) == 0 {
                    gEngfuncs.Host_Error.expect("non-null function pointer")(b"AllocBlock: full\n\x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                base = gl_lms.lightmap_buffer.as_mut_ptr();
                base =
                    base.offset((((*(*surf).info).dlight_t * tr.block_size +
                                      (*(*surf).info).dlight_s) *
                                     4 as libc::c_int) as isize);
                R_BuildLightMap(surf, base, tr.block_size * 4 as libc::c_int,
                                true_0);
            }
            surf = (*(*surf).info).lightmapchain
        }
        // draw remainder of dynamic lightmaps that haven't been uploaded yet
        if !newsurf.is_null() { LM_UploadBlock(true_0); }
        surf = newsurf;
        while !surf.is_null() {
            if !(*surf).polys.is_null() {
                DrawGLPolyChain((*surf).polys,
                                ((*surf).light_s - (*(*surf).info).dlight_s)
                                    as libc::c_float *
                                    (1.0f32 / tr.block_size as libc::c_float),
                                ((*surf).light_t - (*(*surf).info).dlight_t)
                                    as libc::c_float *
                                    (1.0f32 /
                                         tr.block_size as libc::c_float));
            }
            surf = (*(*surf).info).lightmapchain
        }
    }
    pglDisable.expect("non-null function pointer")(0xbe2 as libc::c_int as
                                                       GLenum);
    pglDepthMask.expect("non-null function pointer")(0x1 as libc::c_int as
                                                         GLboolean);
    pglDepthFunc.expect("non-null function pointer")(0x203 as libc::c_int as
                                                         GLenum);
    pglTexEnvi.expect("non-null function pointer")(0x2300 as libc::c_int as
                                                       GLenum,
                                                   0x2200 as libc::c_int as
                                                       GLenum,
                                                   0x1e01 as libc::c_int);
    pglColor4f.expect("non-null function pointer")(1.0f32, 1.0f32, 1.0f32,
                                                   1.0f32);
    // restore fog here
    GL_ResetFogColor();
}
/*
================
R_RenderFullbrights
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_RenderFullbrights() {
    let mut es: *mut mextrasurf_t = 0 as *mut mextrasurf_t;
    let mut p: *mut mextrasurf_t = 0 as *mut mextrasurf_t;
    let mut i: libc::c_int = 0;
    if draw_fullbrights as u64 == 0 { return }
    R_AllowFog(false_0);
    pglEnable.expect("non-null function pointer")(0xbe2 as libc::c_int as
                                                      GLenum);
    pglDepthMask.expect("non-null function pointer")(0 as libc::c_int as
                                                         GLboolean);
    pglDisable.expect("non-null function pointer")(0xbc0 as libc::c_int as
                                                       GLenum);
    pglBlendFunc.expect("non-null function pointer")(0x1 as libc::c_int as
                                                         GLenum,
                                                     0x1 as libc::c_int as
                                                         GLenum);
    pglTexEnvi.expect("non-null function pointer")(0x2300 as libc::c_int as
                                                       GLenum,
                                                   0x2200 as libc::c_int as
                                                       GLenum,
                                                   0x2100 as libc::c_int);
    i = 1 as libc::c_int;
    while i < 4096 as libc::c_int {
        es = fullbright_surfaces[i as usize];
        if !es.is_null() {
            GL_Bind(XASH_TEXTURE0 as libc::c_int, i as GLenum);
            p = es;
            while !p.is_null() {
                DrawGLPoly((*(*p).surf).polys, 0.0f32, 0.0f32);
                p = (*p).lumachain
            }
            fullbright_surfaces[i as usize] = 0 as *mut mextrasurf_t;
            (*es).lumachain = 0 as *mut mextrasurf_s
        }
        i += 1
    }
    pglDisable.expect("non-null function pointer")(0xbe2 as libc::c_int as
                                                       GLenum);
    pglDepthMask.expect("non-null function pointer")(0x1 as libc::c_int as
                                                         GLboolean);
    pglDisable.expect("non-null function pointer")(0xbc0 as libc::c_int as
                                                       GLenum);
    pglTexEnvi.expect("non-null function pointer")(0x2300 as libc::c_int as
                                                       GLenum,
                                                   0x2200 as libc::c_int as
                                                       GLenum,
                                                   0x1e01 as libc::c_int);
    draw_fullbrights = false_0;
    R_AllowFog(true_0);
}
/*
================
R_RenderDetails
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_RenderDetails() {
    let mut glt: *mut gl_texture_t =
        0 as *mut gl_texture_t; // get texture scale
    let mut es: *mut mextrasurf_t = 0 as *mut mextrasurf_t;
    let mut p: *mut mextrasurf_t = 0 as *mut mextrasurf_t;
    let mut fa: *mut msurface_t = 0 as *mut msurface_t;
    let mut i: libc::c_int = 0;
    if draw_details as u64 == 0 { return }
    GL_SetupFogColorForSurfaces();
    pglEnable.expect("non-null function pointer")(0xbe2 as libc::c_int as
                                                      GLenum);
    pglBlendFunc.expect("non-null function pointer")(0x306 as libc::c_int as
                                                         GLenum,
                                                     0x300 as libc::c_int as
                                                         GLenum);
    pglTexEnvi.expect("non-null function pointer")(0x2300 as libc::c_int as
                                                       GLenum,
                                                   0x2200 as libc::c_int as
                                                       GLenum,
                                                   0x2101 as libc::c_int);
    pglDepthFunc.expect("non-null function pointer")(0x202 as libc::c_int as
                                                         GLenum);
    i = 1 as libc::c_int;
    while i < 4096 as libc::c_int {
        es = detail_surfaces[i as usize];
        if !es.is_null() {
            GL_Bind(XASH_TEXTURE0 as libc::c_int, i as GLenum);
            p = es;
            while !p.is_null() {
                fa = (*p).surf;
                glt =
                    R_GetTexture((*(*(*fa).texinfo).texture).gl_texturenum as
                                     GLenum);
                DrawGLPoly((*fa).polys, (*glt).xscale, (*glt).yscale);
                p = (*p).detailchain
            }
            detail_surfaces[i as usize] = 0 as *mut mextrasurf_t;
            (*es).detailchain = 0 as *mut mextrasurf_s
        }
        i += 1
    }
    pglDisable.expect("non-null function pointer")(0xbe2 as libc::c_int as
                                                       GLenum);
    pglTexEnvi.expect("non-null function pointer")(0x2300 as libc::c_int as
                                                       GLenum,
                                                   0x2200 as libc::c_int as
                                                       GLenum,
                                                   0x1e01 as libc::c_int);
    pglDepthFunc.expect("non-null function pointer")(0x203 as libc::c_int as
                                                         GLenum);
    draw_details = false_0;
    // restore fog here
    GL_ResetFogColor();
}
/*
================
R_RenderBrushPoly
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_RenderBrushPoly(mut fa: *mut msurface_t,
                                           mut cull_type: libc::c_int) {
    let mut current_block: u64; // already handled
    let mut is_dynamic: qboolean = false_0;
    let mut maps: libc::c_int = 0;
    let mut t: *mut texture_t = 0 as *mut texture_t;
    r_stats.c_world_polys = r_stats.c_world_polys.wrapping_add(1);
    if (*fa).flags as libc::c_uint & (1 as libc::c_uint) << 2 as libc::c_int
           != 0 {
        return
    }
    t = R_TextureAnimation(fa);
    GL_Bind(XASH_TEXTURE0 as libc::c_int, (*t).gl_texturenum as GLenum);
    if (*fa).flags as libc::c_uint & (1 as libc::c_uint) << 4 as libc::c_int
           != 0 {
        // warp texture, no lightmaps
        EmitWaterPolys(fa,
                       (cull_type == 1 as libc::c_int) as libc::c_int as
                           qboolean);
        return
    }
    if (*t).fb_texturenum != 0 {
        (*(*fa).info).lumachain =
            fullbright_surfaces[(*t).fb_texturenum as usize];
        fullbright_surfaces[(*t).fb_texturenum as usize] = (*fa).info;
        draw_fullbrights = true_0
    }
    if if !r_detailtextures.is_null() && (*r_detailtextures).value != 0.0f32 {
           true_0 as libc::c_int
       } else { false_0 as libc::c_int } != 0 {
        if glState.isFogEnabled != 0 {
            // don't apply detail textures for windows in the fog
            if (*RI.currententity).curstate.rendermode !=
                   kRenderTransTexture as libc::c_int {
                if (*t).dt_texturenum != 0 {
                    (*(*fa).info).detailchain =
                        detail_surfaces[(*t).dt_texturenum as usize];
                    detail_surfaces[(*t).dt_texturenum as usize] = (*fa).info
                } else {
                    // draw stub detail texture for underwater surfaces
                    (*(*fa).info).detailchain =
                        detail_surfaces[tr.grayTexture as usize];
                    detail_surfaces[tr.grayTexture as usize] = (*fa).info
                }
                draw_details = true_0
            }
        } else if (*t).dt_texturenum != 0 {
            (*(*fa).info).detailchain =
                detail_surfaces[(*t).dt_texturenum as usize];
            detail_surfaces[(*t).dt_texturenum as usize] = (*fa).info;
            draw_details = true_0
        }
    }
    DrawGLPoly((*fa).polys, 0.0f32, 0.0f32);
    if (*RI.currententity).curstate.rendermode == kRenderNormal as libc::c_int
       {
        // batch decals to draw later
        if tr.num_draw_decals < 4096 as libc::c_int &&
               !(*fa).pdecals.is_null() {
            let fresh6 = tr.num_draw_decals;
            tr.num_draw_decals = tr.num_draw_decals + 1;
            tr.draw_decals[fresh6 as usize] = fa
        }
    } else {
        // if rendermode != kRenderNormal draw decals sequentially
        DrawSurfaceDecals(fa, true_0,
                          (cull_type == 1 as libc::c_int) as libc::c_int as
                              qboolean); // no lightmaps anyway
    }
    if (*fa).flags as libc::c_uint & (1 as libc::c_uint) << 5 as libc::c_int
           != 0 {
        return
    }
    // check for lightmap modification
    maps = 0 as libc::c_int;
    loop  {
        if !(maps < 4 as libc::c_int &&
                 (*fa).styles[maps as usize] as libc::c_int !=
                     255 as libc::c_int) {
            current_block = 15512526488502093901;
            break ;
        }
        if tr.lightstylevalue[(*fa).styles[maps as usize] as usize] !=
               (*fa).cached_light[maps as usize] {
            current_block = 6904835840647375537;
            break ;
        }
        maps += 1
    }
    match current_block {
        15512526488502093901 =>
        // dynamic this frame or dynamic previously
        {
            if (*fa).dlightframe == tr.framecount {
                current_block = 6904835840647375537;
            } else { current_block = 1434579379687443766; }
        }
        _ => { }
    }
    match current_block {
        6904835840647375537 => {
            // NOTE: at this point we have only valid textures
            if (*r_dynamic).value != 0. { is_dynamic = true_0 }
        }
        _ => { }
    }
    if is_dynamic as u64 != 0 {
        if ((*fa).styles[maps as usize] as libc::c_int >= 32 as libc::c_int ||
                (*fa).styles[maps as usize] as libc::c_int == 0 as libc::c_int
                ||
                (*fa).styles[maps as usize] as libc::c_int ==
                    20 as libc::c_int) && (*fa).dlightframe != tr.framecount {
            let mut temp: [byte; 69696] = [0; 69696];
            let mut info: *mut mextrasurf_t = (*fa).info;
            let mut sample_size: libc::c_int = 0;
            let mut smax: libc::c_int = 0;
            let mut tmax: libc::c_int = 0;
            sample_size =
                gEngfuncs.Mod_SampleSizeForFace.expect("non-null function pointer")(fa);
            smax =
                (*info).lightextents[0 as libc::c_int as usize] as libc::c_int
                    / sample_size + 1 as libc::c_int;
            tmax =
                (*info).lightextents[1 as libc::c_int as usize] as libc::c_int
                    / sample_size + 1 as libc::c_int;
            R_BuildLightMap(fa, temp.as_mut_ptr(), smax * 4 as libc::c_int,
                            true_0);
            R_SetCacheState(fa);
            GL_Bind(XASH_TEXTURE0 as libc::c_int,
                    tr.lightmapTextures[(*fa).lightmaptexturenum as usize] as
                        GLenum);
            pglTexSubImage2D.expect("non-null function pointer")(0xde1 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 0 as
                                                                     libc::c_int,
                                                                 (*fa).light_s,
                                                                 (*fa).light_t,
                                                                 smax, tmax,
                                                                 0x1908 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 0x1401 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 temp.as_mut_ptr()
                                                                     as
                                                                     *const libc::c_void);
            (*(*fa).info).lightmapchain =
                gl_lms.lightmap_surfaces[(*fa).lightmaptexturenum as usize];
            gl_lms.lightmap_surfaces[(*fa).lightmaptexturenum as usize] = fa
        } else {
            (*(*fa).info).lightmapchain = gl_lms.dynamic_surfaces;
            gl_lms.dynamic_surfaces = fa
        }
    } else {
        (*(*fa).info).lightmapchain =
            gl_lms.lightmap_surfaces[(*fa).lightmaptexturenum as usize];
        gl_lms.lightmap_surfaces[(*fa).lightmaptexturenum as usize] = fa
    };
}
/*
================
R_DrawTextureChains
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_DrawTextureChains() {
    let mut i: libc::c_int = 0;
    let mut s: *mut msurface_t = 0 as *mut msurface_t;
    let mut t: *mut texture_t = 0 as *mut texture_t;
    // make sure what color is reset
    pglColor4ub.expect("non-null function pointer")(255 as libc::c_int as
                                                        GLubyte,
                                                    255 as libc::c_int as
                                                        GLubyte,
                                                    255 as libc::c_int as
                                                        GLubyte,
                                                    255 as libc::c_int as
                                                        GLubyte); // set identity matrix
    R_LoadIdentity();
    GL_SetupFogColorForSurfaces();
    // restore worldmodel
    RI.currententity =
        gEngfuncs.GetEntityByIndex.expect("non-null function pointer")(0 as
                                                                           libc::c_int);
    RI.currentmodel = (*RI.currententity).model;
    if Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(17
                                                                                                                 as
                                                                                                                 libc::c_int,
                                                                                                             0
                                                                                                                 as
                                                                                                                 libc::c_int)
           != 0 {
        pglDisable.expect("non-null function pointer")(0xde1 as libc::c_int as
                                                           GLenum);
        pglColor3f.expect("non-null function pointer")(1.0f32, 1.0f32,
                                                       1.0f32);
    }
    // clip skybox surfaces
    s = skychain; // draw translucent water later
    while !s.is_null() { R_AddSkyBoxSurface(s); s = (*s).texturechain }
    if Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(17
                                                                                                                 as
                                                                                                                 libc::c_int,
                                                                                                             0
                                                                                                                 as
                                                                                                                 libc::c_int)
           != 0 {
        pglEnable.expect("non-null function pointer")(0xde1 as libc::c_int as
                                                          GLenum);
        if !skychain.is_null() { R_DrawClouds(); }
        skychain = 0 as *mut msurface_t
    }
    i = 0 as libc::c_int;
    while i <
              (*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                     as
                                                                                     libc::c_int)).numtextures
          {
        t =
            *(*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                    as
                                                                                    libc::c_int)).textures.offset(i
                                                                                                                      as
                                                                                                                      isize);
        if !t.is_null() {
            s = (*t).texturechain;
            if !(s.is_null() || i == tr.skytexturenum) {
                if !((*s).flags as libc::c_uint &
                         (1 as libc::c_uint) << 4 as libc::c_int != 0 &&
                         (*gEngfuncs.pfnGetMoveVars.expect("non-null function pointer")()).wateralpha
                             < 1.0f32) {
                    if Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(PARM_QUAKE_COMPATIBLE
                                                                                                                                 as
                                                                                                                                 libc::c_int,
                                                                                                                             0
                                                                                                                                 as
                                                                                                                                 libc::c_int)
                           != 0 &&
                           (*s).flags as libc::c_uint &
                               (1 as libc::c_uint) << 8 as libc::c_int != 0 {
                        draw_alpha_surfaces = true_0
                        // draw transparent surfaces later
                    } else {
                        while !s.is_null() {
                            R_RenderBrushPoly(s, 0 as libc::c_int);
                            s = (*s).texturechain
                        }
                        (*t).texturechain = 0 as *mut msurface_s
                    }
                }
            }
        }
        i += 1
    };
}
/*
================
R_DrawAlphaTextureChains
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_DrawAlphaTextureChains() {
    let mut i: libc::c_int = 0;
    let mut s: *mut msurface_t = 0 as *mut msurface_t;
    let mut t: *mut texture_t = 0 as *mut texture_t;
    if draw_alpha_surfaces as u64 == 0 { return }
    memset(gl_lms.lightmap_surfaces.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<[*mut msurface_t; 256]>() as libc::c_ulong);
    gl_lms.dynamic_surfaces = 0 as *mut msurface_t;
    // make sure what color is reset
    pglColor4ub.expect("non-null function pointer")(255 as libc::c_int as
                                                        GLubyte,
                                                    255 as libc::c_int as
                                                        GLubyte,
                                                    255 as libc::c_int as
                                                        GLubyte,
                                                    255 as libc::c_int as
                                                        GLubyte); // set identity matrix
    R_LoadIdentity();
    pglDisable.expect("non-null function pointer")(0xbe2 as libc::c_int as
                                                       GLenum);
    pglEnable.expect("non-null function pointer")(0xbc0 as libc::c_int as
                                                      GLenum);
    pglAlphaFunc.expect("non-null function pointer")(0x204 as libc::c_int as
                                                         GLenum, 0.25f32);
    GL_SetupFogColorForSurfaces();
    // restore worldmodel
    RI.currententity =
        gEngfuncs.GetEntityByIndex.expect("non-null function pointer")(0 as
                                                                           libc::c_int); // restore world rendermode
    RI.currentmodel = (*RI.currententity).model;
    (*RI.currententity).curstate.rendermode =
        kRenderTransAlpha as libc::c_int;
    draw_alpha_surfaces = false_0;
    i = 0 as libc::c_int;
    while i <
              (*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                     as
                                                                                     libc::c_int)).numtextures
          {
        t =
            *(*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                    as
                                                                                    libc::c_int)).textures.offset(i
                                                                                                                      as
                                                                                                                      isize);
        if !t.is_null() {
            s = (*t).texturechain;
            if !(s.is_null() ||
                     (*s).flags as libc::c_uint &
                         (1 as libc::c_uint) << 8 as libc::c_int == 0) {
                while !s.is_null() {
                    R_RenderBrushPoly(s, 0 as libc::c_int);
                    s = (*s).texturechain
                }
                (*t).texturechain = 0 as *mut msurface_s
            }
        }
        i += 1
    }
    GL_ResetFogColor();
    R_BlendLightmaps();
    (*RI.currententity).curstate.rendermode = kRenderNormal as libc::c_int;
    pglAlphaFunc.expect("non-null function pointer")(0x204 as libc::c_int as
                                                         GLenum, 0.0f32);
}
/*
================
R_DrawWaterSurfaces
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_DrawWaterSurfaces() {
    let mut i: libc::c_int = 0;
    let mut s: *mut msurface_t = 0 as *mut msurface_t;
    let mut t: *mut texture_t = 0 as *mut texture_t;
    if RI.drawWorld as u64 == 0 || RI.onlyClientDraw as libc::c_uint != 0 {
        return
    }
    // non-transparent water is already drawed
    if (*gEngfuncs.pfnGetMoveVars.expect("non-null function pointer")()).wateralpha
           >= 1.0f32 {
        return
    }
    // restore worldmodel
    RI.currententity =
        gEngfuncs.GetEntityByIndex.expect("non-null function pointer")(0 as
                                                                           libc::c_int);
    RI.currentmodel = (*RI.currententity).model;
    // go back to the world matrix
    R_LoadIdentity();
    pglEnable.expect("non-null function pointer")(0xbe2 as libc::c_int as
                                                      GLenum);
    pglDepthMask.expect("non-null function pointer")(0 as libc::c_int as
                                                         GLboolean);
    pglDisable.expect("non-null function pointer")(0xbc0 as libc::c_int as
                                                       GLenum);
    pglBlendFunc.expect("non-null function pointer")(0x302 as libc::c_int as
                                                         GLenum,
                                                     0x303 as libc::c_int as
                                                         GLenum);
    pglTexEnvi.expect("non-null function pointer")(0x2300 as libc::c_int as
                                                       GLenum,
                                                   0x2200 as libc::c_int as
                                                       GLenum,
                                                   0x2100 as libc::c_int);
    pglColor4f.expect("non-null function pointer")(1.0f32, 1.0f32, 1.0f32,
                                                   (*gEngfuncs.pfnGetMoveVars.expect("non-null function pointer")()).wateralpha);
    i = 0 as libc::c_int;
    while i <
              (*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                     as
                                                                                     libc::c_int)).numtextures
          {
        t =
            *(*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                    as
                                                                                    libc::c_int)).textures.offset(i
                                                                                                                      as
                                                                                                                      isize);
        if !t.is_null() {
            s = (*t).texturechain;
            if !s.is_null() {
                if !((*s).flags as libc::c_uint &
                         (1 as libc::c_uint) << 4 as libc::c_int == 0) {
                    // set modulate mode explicitly
                    GL_Bind(XASH_TEXTURE0 as libc::c_int,
                            (*t).gl_texturenum as GLenum);
                    while !s.is_null() {
                        EmitWaterPolys(s, false_0);
                        s = (*s).texturechain
                    }
                    (*t).texturechain = 0 as *mut msurface_s
                }
            }
        }
        i += 1
    }
    pglDisable.expect("non-null function pointer")(0xbe2 as libc::c_int as
                                                       GLenum);
    pglDepthMask.expect("non-null function pointer")(0x1 as libc::c_int as
                                                         GLboolean);
    pglDisable.expect("non-null function pointer")(0xbc0 as libc::c_int as
                                                       GLenum);
    pglTexEnvi.expect("non-null function pointer")(0x2300 as libc::c_int as
                                                       GLenum,
                                                   0x2200 as libc::c_int as
                                                       GLenum,
                                                   0x1e01 as libc::c_int);
    pglColor4ub.expect("non-null function pointer")(255 as libc::c_int as
                                                        GLubyte,
                                                    255 as libc::c_int as
                                                        GLubyte,
                                                    255 as libc::c_int as
                                                        GLubyte,
                                                    255 as libc::c_int as
                                                        GLubyte);
}
/*
=================
R_SurfaceCompare

compare translucent surfaces
=================
*/
unsafe extern "C" fn R_SurfaceCompare(mut a: *const libc::c_void,
                                      mut b: *const libc::c_void)
 -> libc::c_int {
    let mut surf1: *mut msurface_t = 0 as *mut msurface_t;
    let mut surf2: *mut msurface_t = 0 as *mut msurface_t;
    let mut org1: vec3_t = [0.; 3];
    let mut org2: vec3_t = [0.; 3];
    let mut len1: libc::c_float = 0.;
    let mut len2: libc::c_float = 0.;
    surf1 = (*(a as *mut sortedface_t)).surf;
    surf2 = (*(b as *mut sortedface_t)).surf;
    org1[0 as libc::c_int as usize] =
        (*RI.currententity).origin[0 as libc::c_int as usize] +
            (*(*surf1).info).origin[0 as libc::c_int as usize];
    org1[1 as libc::c_int as usize] =
        (*RI.currententity).origin[1 as libc::c_int as usize] +
            (*(*surf1).info).origin[1 as libc::c_int as usize];
    org1[2 as libc::c_int as usize] =
        (*RI.currententity).origin[2 as libc::c_int as usize] +
            (*(*surf1).info).origin[2 as libc::c_int as usize];
    org2[0 as libc::c_int as usize] =
        (*RI.currententity).origin[0 as libc::c_int as usize] +
            (*(*surf2).info).origin[0 as libc::c_int as usize];
    org2[1 as libc::c_int as usize] =
        (*RI.currententity).origin[1 as libc::c_int as usize] +
            (*(*surf2).info).origin[1 as libc::c_int as usize];
    org2[2 as libc::c_int as usize] =
        (*RI.currententity).origin[2 as libc::c_int as usize] +
            (*(*surf2).info).origin[2 as libc::c_int as usize];
    // compare by plane dists
    len1 =
        org1[0 as libc::c_int as usize] *
            RI.vforward[0 as libc::c_int as usize] +
            org1[1 as libc::c_int as usize] *
                RI.vforward[1 as libc::c_int as usize] +
            org1[2 as libc::c_int as usize] *
                RI.vforward[2 as libc::c_int as usize] - RI.viewplanedist;
    len2 =
        org2[0 as libc::c_int as usize] *
            RI.vforward[0 as libc::c_int as usize] +
            org2[1 as libc::c_int as usize] *
                RI.vforward[1 as libc::c_int as usize] +
            org2[2 as libc::c_int as usize] *
                RI.vforward[2 as libc::c_int as usize] - RI.viewplanedist;
    if len1 > len2 { return -(1 as libc::c_int) }
    if len1 < len2 { return 1 as libc::c_int }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn R_SetRenderMode(mut e: *mut cl_entity_t) {
    match (*e).curstate.rendermode {
        0 => {
            pglColor4f.expect("non-null function pointer")(1.0f32, 1.0f32,
                                                           1.0f32, 1.0f32);
        }
        1 => {
            pglBlendFunc.expect("non-null function pointer")(0x302 as
                                                                 libc::c_int
                                                                 as GLenum,
                                                             0x303 as
                                                                 libc::c_int
                                                                 as GLenum);
            pglColor4ub.expect("non-null function pointer")((*e).curstate.rendercolor.r,
                                                            (*e).curstate.rendercolor.g,
                                                            (*e).curstate.rendercolor.b,
                                                            (*e).curstate.renderamt
                                                                as GLubyte);
            pglTexEnvf.expect("non-null function pointer")(0x2300 as
                                                               libc::c_int as
                                                               GLenum,
                                                           0x2200 as
                                                               libc::c_int as
                                                               GLenum,
                                                           0x2100 as
                                                               libc::c_int as
                                                               GLfloat);
            pglDisable.expect("non-null function pointer")(0xde1 as
                                                               libc::c_int as
                                                               GLenum);
            pglEnable.expect("non-null function pointer")(0xbe2 as libc::c_int
                                                              as GLenum);
        }
        5 => {
            pglTexEnvf.expect("non-null function pointer")(0x2300 as
                                                               libc::c_int as
                                                               GLenum,
                                                           0x2200 as
                                                               libc::c_int as
                                                               GLenum,
                                                           0x2100 as
                                                               libc::c_int as
                                                               GLfloat);
            pglColor4f.expect("non-null function pointer")(tr.blend, tr.blend,
                                                           tr.blend, 1.0f32);
            pglBlendFunc.expect("non-null function pointer")(0x1 as
                                                                 libc::c_int
                                                                 as GLenum,
                                                             0x1 as
                                                                 libc::c_int
                                                                 as GLenum);
            pglDepthMask.expect("non-null function pointer")(0 as libc::c_int
                                                                 as
                                                                 GLboolean);
            pglEnable.expect("non-null function pointer")(0xbe2 as libc::c_int
                                                              as GLenum);
        }
        4 => {
            pglEnable.expect("non-null function pointer")(0xbc0 as libc::c_int
                                                              as GLenum);
            pglTexEnvf.expect("non-null function pointer")(0x2300 as
                                                               libc::c_int as
                                                               GLenum,
                                                           0x2200 as
                                                               libc::c_int as
                                                               GLenum,
                                                           0x2100 as
                                                               libc::c_int as
                                                               GLfloat);
            if Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(PARM_QUAKE_COMPATIBLE
                                                                                                                         as
                                                                                                                         libc::c_int,
                                                                                                                     0
                                                                                                                         as
                                                                                                                         libc::c_int)
                   != 0 {
                pglBlendFunc.expect("non-null function pointer")(0x302 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 0x303 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum);
                pglColor4f.expect("non-null function pointer")(1.0f32, 1.0f32,
                                                               1.0f32,
                                                               tr.blend);
                pglEnable.expect("non-null function pointer")(0xbe2 as
                                                                  libc::c_int
                                                                  as GLenum);
            } else {
                pglColor4f.expect("non-null function pointer")(1.0f32, 1.0f32,
                                                               1.0f32,
                                                               1.0f32);
                pglDisable.expect("non-null function pointer")(0xbe2 as
                                                                   libc::c_int
                                                                   as GLenum);
            }
            pglAlphaFunc.expect("non-null function pointer")(0x204 as
                                                                 libc::c_int
                                                                 as GLenum,
                                                             0.25f32);
        }
        _ => {
            pglTexEnvf.expect("non-null function pointer")(0x2300 as
                                                               libc::c_int as
                                                               GLenum,
                                                           0x2200 as
                                                               libc::c_int as
                                                               GLenum,
                                                           0x2100 as
                                                               libc::c_int as
                                                               GLfloat);
            pglBlendFunc.expect("non-null function pointer")(0x302 as
                                                                 libc::c_int
                                                                 as GLenum,
                                                             0x303 as
                                                                 libc::c_int
                                                                 as GLenum);
            pglColor4f.expect("non-null function pointer")(1.0f32, 1.0f32,
                                                           1.0f32, tr.blend);
            pglDepthMask.expect("non-null function pointer")(0 as libc::c_int
                                                                 as
                                                                 GLboolean);
            pglEnable.expect("non-null function pointer")(0xbe2 as libc::c_int
                                                              as GLenum);
        }
    };
}
/*
=================
R_DrawBrushModel
=================
*/
#[no_mangle]
pub unsafe extern "C" fn R_DrawBrushModel(mut e: *mut cl_entity_t) {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut num_sorted: libc::c_int = 0;
    let mut origin_l: vec3_t = [0.; 3];
    let mut oldorigin: vec3_t = [0.; 3];
    let mut old_rendermode: libc::c_int = 0;
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut cull_type: libc::c_int = 0;
    let mut psurf: *mut msurface_t = 0 as *mut msurface_t;
    let mut clmodel: *mut model_t = 0 as *mut model_t;
    let mut rotated: qboolean = false_0;
    let mut l: *mut dlight_t = 0 as *mut dlight_t;
    let mut allow_vbo: qboolean =
        if !r_vbo.is_null() && (*r_vbo).value != 0.0f32 {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int } as qboolean;
    if RI.drawWorld as u64 == 0 { return }
    clmodel = (*e).model;
    // external models not loaded to VBO
    if (*clmodel).surfaces !=
           (*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                  as
                                                                                  libc::c_int)).surfaces
       {
        allow_vbo = false_0
    } // visible
    if !((*e).angles[0 as libc::c_int as usize] == 0.0f32 &&
             (*e).angles[1 as libc::c_int as usize] == 0.0f32 &&
             (*e).angles[2 as libc::c_int as usize] == 0.0f32) {
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            mins[i as usize] = (*e).origin[i as usize] - (*clmodel).radius;
            maxs[i as usize] = (*e).origin[i as usize] + (*clmodel).radius;
            i += 1
        }
        rotated = true_0
    } else {
        mins[0 as libc::c_int as usize] =
            (*e).origin[0 as libc::c_int as usize] +
                (*clmodel).mins[0 as libc::c_int as usize];
        mins[1 as libc::c_int as usize] =
            (*e).origin[1 as libc::c_int as usize] +
                (*clmodel).mins[1 as libc::c_int as usize];
        mins[2 as libc::c_int as usize] =
            (*e).origin[2 as libc::c_int as usize] +
                (*clmodel).mins[2 as libc::c_int as usize];
        maxs[0 as libc::c_int as usize] =
            (*e).origin[0 as libc::c_int as usize] +
                (*clmodel).maxs[0 as libc::c_int as usize];
        maxs[1 as libc::c_int as usize] =
            (*e).origin[1 as libc::c_int as usize] +
                (*clmodel).maxs[1 as libc::c_int as usize];
        maxs[2 as libc::c_int as usize] =
            (*e).origin[2 as libc::c_int as usize] +
                (*clmodel).maxs[2 as libc::c_int as usize];
        rotated = false_0
    }
    if R_CullBox(mins.as_mut_ptr() as *const vec_t,
                 maxs.as_mut_ptr() as *const vec_t) as u64 != 0 {
        return
    }
    memset(gl_lms.lightmap_surfaces.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<[*mut msurface_t; 256]>() as libc::c_ulong);
    old_rendermode = (*e).curstate.rendermode;
    gl_lms.dynamic_surfaces = 0 as *mut msurface_t;
    if rotated as u64 != 0 {
        R_RotateForEntity(e);
    } else { R_TranslateForEntity(e); }
    if Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(PARM_QUAKE_COMPATIBLE
                                                                                                                 as
                                                                                                                 libc::c_int,
                                                                                                             0
                                                                                                                 as
                                                                                                                 libc::c_int)
           != 0 &&
           (*clmodel).flags as libc::c_uint &
               (1 as libc::c_uint) << 3 as libc::c_int != 0 {
        (*e).curstate.rendermode = kRenderTransAlpha as libc::c_int
    }
    (*e).visframe = tr.realframecount;
    if rotated as u64 != 0 {
        Matrix4x4_VectorITransform(RI.objectMatrix.as_mut_ptr() as
                                       *const [vec_t; 4],
                                   RI.cullorigin.as_mut_ptr() as
                                       *const libc::c_float,
                                   tr.modelorg.as_mut_ptr());
    } else {
        tr.modelorg[0 as libc::c_int as usize] =
            RI.cullorigin[0 as libc::c_int as usize] -
                (*e).origin[0 as libc::c_int as usize];
        tr.modelorg[1 as libc::c_int as usize] =
            RI.cullorigin[1 as libc::c_int as usize] -
                (*e).origin[1 as libc::c_int as usize];
        tr.modelorg[2 as libc::c_int as usize] =
            RI.cullorigin[2 as libc::c_int as usize] -
                (*e).origin[2 as libc::c_int as usize]
    }
    // calculate dynamic lighting for bmodel
    k = 0 as libc::c_int;
    while k < 32 as libc::c_int {
        l = gEngfuncs.GetDynamicLight.expect("non-null function pointer")(k);
        if !((*l).die < (*gpGlobals).time || (*l).radius == 0.) {
            // restore lightorigin
            oldorigin[0 as libc::c_int as usize] =
                (*l).origin[0 as libc::c_int as usize]; // save lightorigin
            oldorigin[1 as libc::c_int as usize] =
                (*l).origin[1 as libc::c_int as
                                usize]; // move light in bmodel space
            oldorigin[2 as libc::c_int as usize] =
                (*l).origin[2 as libc::c_int as usize];
            Matrix4x4_VectorITransform(RI.objectMatrix.as_mut_ptr() as
                                           *const [vec_t; 4],
                                       (*l).origin.as_mut_ptr() as
                                           *const libc::c_float,
                                       origin_l.as_mut_ptr());
            (*l).origin[0 as libc::c_int as usize] =
                origin_l[0 as libc::c_int as usize];
            (*l).origin[1 as libc::c_int as usize] =
                origin_l[1 as libc::c_int as usize];
            (*l).origin[2 as libc::c_int as usize] =
                origin_l[2 as libc::c_int as usize];
            R_MarkLights(l, (1 as libc::c_int) << k,
                         (*clmodel).nodes.offset((*clmodel).hulls[0 as
                                                                      libc::c_int
                                                                      as
                                                                      usize].firstclipnode
                                                     as isize));
            (*l).origin[0 as libc::c_int as usize] =
                oldorigin[0 as libc::c_int as usize];
            (*l).origin[1 as libc::c_int as usize] =
                oldorigin[1 as libc::c_int as usize];
            (*l).origin[2 as libc::c_int as usize] =
                oldorigin[2 as libc::c_int as usize]
        }
        k += 1
    }
    // setup the rendermode
    R_SetRenderMode(e);
    GL_SetupFogColorForSurfaces();
    if (*e).curstate.rendermode == kRenderTransAdd as libc::c_int {
        R_AllowFog(false_0);
        allow_vbo = false_0
    }
    if (*e).curstate.rendermode == kRenderTransColor as libc::c_int ||
           (*e).curstate.rendermode == kRenderTransTexture as libc::c_int {
        allow_vbo = false_0
    }
    psurf =
        &mut *(*clmodel).surfaces.offset((*clmodel).firstmodelsurface as
                                             isize) as *mut msurface_t;
    num_sorted = 0 as libc::c_int;
    let mut current_block_55: u64;
    i = 0 as libc::c_int;
    while i < (*clmodel).nummodelsurfaces {
        if (*psurf).flags as libc::c_uint &
               (1 as libc::c_uint) << 4 as libc::c_int != 0 &&
               Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(PARM_QUAKE_COMPATIBLE
                                                                                                                         as
                                                                                                                         libc::c_int,
                                                                                                                     0
                                                                                                                         as
                                                                                                                         libc::c_int)
                   == 0 {
            if (*(*psurf).plane).type_0 as libc::c_int != 2 as libc::c_int &&
                   (*e).curstate.effects as libc::c_uint &
                       (1 as libc::c_uint) << 26 as libc::c_int == 0 {
                current_block_55 = 7990025728955927862;
            } else if mins[2 as libc::c_int as usize] + 1.0f32 >=
                          (*(*psurf).plane).dist {
                current_block_55 = 7990025728955927862;
            } else { current_block_55 = 10380409671385728102; }
        } else { current_block_55 = 10380409671385728102; }
        match current_block_55 {
            10380409671385728102 => {
                cull_type =
                    R_CullSurface(psurf, &mut RI.frustum,
                                  RI.frustum.clipFlags);
                if !(cull_type >= 2 as libc::c_int) {
                    if cull_type == 1 as libc::c_int {
                        if (*psurf).flags as libc::c_uint &
                               (1 as libc::c_uint) << 4 as libc::c_int == 0 &&
                               !(!(*psurf).pdecals.is_null() &&
                                     (*e).curstate.rendermode ==
                                         kRenderTransTexture as libc::c_int) {
                            current_block_55 = 7990025728955927862;
                        } else { current_block_55 = 3546145585875536353; }
                    } else { current_block_55 = 3546145585875536353; }
                    match current_block_55 {
                        7990025728955927862 => { }
                        _ => {
                            if num_sorted < (*gpGlobals).max_surfaces {
                                let ref mut fresh7 =
                                    (*(*gpGlobals).draw_surfaces.offset(num_sorted
                                                                            as
                                                                            isize)).surf;
                                *fresh7 = psurf;
                                (*(*gpGlobals).draw_surfaces.offset(num_sorted
                                                                        as
                                                                        isize)).cull
                                    = cull_type;
                                num_sorted += 1
                            }
                        }
                    }
                }
            }
            _ => { }
        }
        i += 1;
        psurf = psurf.offset(1)
    }
    // sort faces if needs
    if (*clmodel).flags as libc::c_uint &
           (1 as libc::c_uint) << 2 as libc::c_int == 0 &&
           (*e).curstate.rendermode == kRenderTransTexture as libc::c_int &&
           (if !gl_nosort.is_null() && (*gl_nosort).value != 0.0f32 {
                true_0 as libc::c_int
            } else { false_0 as libc::c_int }) == 0 {
        qsort((*gpGlobals).draw_surfaces as *mut libc::c_void,
              num_sorted as size_t,
              ::std::mem::size_of::<sortedface_t>() as libc::c_ulong,
              Some(R_SurfaceCompare as
                       unsafe extern "C" fn(_: *const libc::c_void,
                                            _: *const libc::c_void)
                           -> libc::c_int));
    }
    // draw sorted translucent surfaces
    i = 0 as libc::c_int;
    while i < num_sorted {
        if allow_vbo as u64 == 0 ||
               R_AddSurfToVBO((*(*gpGlobals).draw_surfaces.offset(i as
                                                                      isize)).surf,
                              true_0) as u64 == 0 {
            R_RenderBrushPoly((*(*gpGlobals).draw_surfaces.offset(i as
                                                                      isize)).surf,
                              (*(*gpGlobals).draw_surfaces.offset(i as
                                                                      isize)).cull);
        }
        i += 1
    }
    R_DrawVBO(R_HasLightmap(), true_0);
    if (*e).curstate.rendermode == kRenderTransColor as libc::c_int {
        pglEnable.expect("non-null function pointer")(0xde1 as libc::c_int as
                                                          GLenum);
    }
    DrawDecalsBatch();
    GL_ResetFogColor();
    R_BlendLightmaps();
    R_RenderFullbrights();
    R_RenderDetails();
    // restore fog here
    if (*e).curstate.rendermode == kRenderTransAdd as libc::c_int {
        R_AllowFog(true_0); // draw before restore
    }
    (*e).curstate.rendermode = old_rendermode;
    pglDisable.expect("non-null function pointer")(0xbc0 as libc::c_int as
                                                       GLenum);
    pglAlphaFunc.expect("non-null function pointer")(0x204 as libc::c_int as
                                                         GLenum, 0.0f32);
    pglDisable.expect("non-null function pointer")(0xbe2 as libc::c_int as
                                                       GLenum);
    pglDepthMask.expect("non-null function pointer")(0x1 as libc::c_int as
                                                         GLboolean);
    R_DrawModelHull();
    R_LoadIdentity();
    // restore worldmatrix
}
#[no_mangle]
pub static mut vbos: vbo_static_s =
    vbo_static_s{mempool: 0,
                 decaldata: 0 as *const vbodecaldata_t as *mut vbodecaldata_t,
                 textures: 0 as *const vbotexture_t as *mut vbotexture_t,
                 surfdata: 0 as *const vbosurfdata_t as *mut vbosurfdata_t,
                 arraylist: 0 as *const vboarray_t as *mut vboarray_t,
                 dlight_index:
                     0 as *const libc::c_ushort as *mut libc::c_ushort,
                 dlight_tc: 0 as *const vec2_t as *mut vec2_t,
                 dlight_vbo: 0,
                 decal_dlight:
                     [vbovertex_t{pos: [0.; 3],
                                  gl_tc: [0.; 2],
                                  lm_tc: [0.; 2],}; 131072],
                 decal_dlight_vbo: 0,
                 decal_numverts: [0; 131072],
                 minlightmap: 0,
                 maxlightmap: 0,
                 mintexture: 0,
                 maxtexture: 0,
                 minarraysplit_tex: 0,
                 maxarraysplit_tex: 0,
                 minarraysplit_lm: 0,
                 maxarraysplit_lm: 0,};
// details scale
#[no_mangle]
pub static mut mtst: multitexturestate_s =
    multitexturestate_s{tmu_gl: 0,
                        tmu_dt: 0,
                        tmu_lm: 0,
                        details_enabled: false_0,
                        lm: 0,
                        skiptexture: false_0,
                        glt: 0 as *const gl_texture_t as *mut gl_texture_t,};
/*
===================
R_GenerateVBO

Allocate memory for arrays, fill it with vertex attribs and upload to GPU
===================
*/
#[no_mangle]
pub unsafe extern "C" fn R_GenerateVBO() {
    let mut numtextures: libc::c_int =
        (*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                               as
                                                                               libc::c_int)).numtextures;
    let mut numlightmaps: libc::c_int = gl_lms.current_lightmap_texture;
    let mut k: libc::c_int = 0;
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut vbo: *mut vboarray_t = 0 as *mut vboarray_t;
    let mut maxindex: uint = 0 as libc::c_int as uint;
    R_ClearVBO();
    // we do not want to write vbo code that does not use multitexture
    if GL_Support(GL_ARB_VERTEX_BUFFER_OBJECT_EXT as libc::c_int) as u64 == 0
           || GL_Support(GL_ARB_MULTITEXTURE as libc::c_int) as u64 == 0 ||
           glConfig.max_texture_units < 2 as libc::c_int {
        gEngfuncs.Cvar_FullSet.expect("non-null function pointer")(b"r_vbo\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char,
                                                                   b"0\x00" as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char,
                                                                   (1 as
                                                                        libc::c_int)
                                                                       <<
                                                                       17 as
                                                                           libc::c_int);
        return
    }
    // save in config if enabled manually
    if if !r_vbo.is_null() && (*r_vbo).value != 0.0f32 {
           true_0 as libc::c_int
       } else { false_0 as libc::c_int } != 0 {
        (*r_vbo).flags |= (1 as libc::c_int) << 0 as libc::c_int
    }
    vbos.mempool =
        gEngfuncs._Mem_AllocPool.expect("non-null function pointer")(b"Render VBO Zone\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     b"../ref_gl/gl_rsurf.c\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     1805 as
                                                                         libc::c_int);
    vbos.minarraysplit_tex = 2147483647 as libc::c_int;
    vbos.maxarraysplit_tex = 0 as libc::c_int;
    vbos.minarraysplit_lm = 4 as libc::c_int;
    vbos.maxarraysplit_lm = 0 as libc::c_int;
    vbos.minlightmap = 256 as libc::c_int;
    vbos.maxlightmap = 0 as libc::c_int;
    vbos.mintexture = 2147483647 as libc::c_int;
    vbos.maxtexture = 0 as libc::c_int;
    vbos.textures =
        gEngfuncs._Mem_Alloc.expect("non-null function pointer")(vbos.mempool,
                                                                 ((numtextures
                                                                       *
                                                                       numlightmaps)
                                                                      as
                                                                      libc::c_ulong).wrapping_mul(::std::mem::size_of::<vbotexture_t>()
                                                                                                      as
                                                                                                      libc::c_ulong),
                                                                 true_0,
                                                                 b"../ref_gl/gl_rsurf.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 1816 as
                                                                     libc::c_int)
            as *mut vbotexture_t;
    vbos.surfdata =
        gEngfuncs._Mem_Alloc.expect("non-null function pointer")(vbos.mempool,
                                                                 ((*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                                                                         as
                                                                                                                                         libc::c_int)).numsurfaces
                                                                      as
                                                                      libc::c_ulong).wrapping_mul(::std::mem::size_of::<vbosurfdata_t>()
                                                                                                      as
                                                                                                      libc::c_ulong),
                                                                 true_0,
                                                                 b"../ref_gl/gl_rsurf.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 1817 as
                                                                     libc::c_int)
            as *mut vbosurfdata_t;
    vbo =
        gEngfuncs._Mem_Alloc.expect("non-null function pointer")(vbos.mempool,
                                                                 ::std::mem::size_of::<vboarray_t>()
                                                                     as
                                                                     libc::c_ulong,
                                                                 true_0,
                                                                 b"../ref_gl/gl_rsurf.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 1818 as
                                                                     libc::c_int)
            as *mut vboarray_t;
    vbos.arraylist = vbo;
    vbos.decaldata =
        gEngfuncs._Mem_Alloc.expect("non-null function pointer")(vbos.mempool,
                                                                 ::std::mem::size_of::<vbodecaldata_t>()
                                                                     as
                                                                     libc::c_ulong,
                                                                 true_0,
                                                                 b"../ref_gl/gl_rsurf.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 1819 as
                                                                     libc::c_int)
            as *mut vbodecaldata_t;
    (*vbos.decaldata).lm =
        gEngfuncs._Mem_Alloc.expect("non-null function pointer")(vbos.mempool,
                                                                 (::std::mem::size_of::<*mut msurface_t>()
                                                                      as
                                                                      libc::c_ulong).wrapping_mul(numlightmaps
                                                                                                      as
                                                                                                      libc::c_ulong),
                                                                 true_0,
                                                                 b"../ref_gl/gl_rsurf.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 1820 as
                                                                     libc::c_int)
            as *mut *mut msurface_t;
    // count array lengths
    k = 0 as libc::c_int;
    while k < numlightmaps {
        let mut j: libc::c_int = 0;
        j = 0 as libc::c_int;
        while j < numtextures {
            let mut i: libc::c_int = 0;
            let mut vbotex: *mut vbotexture_t =
                &mut *vbos.textures.offset((k * numtextures + j) as isize) as
                    *mut vbotexture_t;
            i = 0 as libc::c_int;
            while i <
                      (*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                             as
                                                                                             libc::c_int)).numsurfaces
                  {
                let mut surf: *mut msurface_t =
                    &mut *(*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                                 as
                                                                                                 libc::c_int)).surfaces.offset(i
                                                                                                                                   as
                                                                                                                                   isize)
                        as *mut msurface_t;
                if !((*surf).flags as libc::c_uint &
                         ((1 as libc::c_uint) << 2 as libc::c_int |
                              (1 as libc::c_uint) << 4 as libc::c_int |
                              (1 as libc::c_uint) << 6 as libc::c_int |
                              (1 as libc::c_uint) << 3 as libc::c_int) != 0) {
                    if !((*surf).lightmaptexturenum != k) {
                        if !(R_TextureAnimation(surf) !=
                                 *(*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                                         as
                                                                                                         libc::c_int)).textures.offset(j
                                                                                                                                           as
                                                                                                                                           isize))
                           {
                            if (*vbo).array_len + (*(*surf).polys).numverts >
                                   32767 as libc::c_int * 2 as libc::c_int +
                                       1 as libc::c_int {
                                // generate new array and new vbotexture node
                                (*vbo).array =
                                    gEngfuncs._Mem_Alloc.expect("non-null function pointer")(vbos.mempool,
                                                                                             (::std::mem::size_of::<vbovertex_t>()
                                                                                                  as
                                                                                                  libc::c_ulong).wrapping_mul((*vbo).array_len
                                                                                                                                  as
                                                                                                                                  libc::c_ulong),
                                                                                             true_0,
                                                                                             b"../ref_gl/gl_rsurf.c\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             1848
                                                                                                 as
                                                                                                 libc::c_int)
                                        as *mut vbovertex_t;
                                gEngfuncs.Con_Printf.expect("non-null function pointer")(b"R_GenerateVBOs: allocated array of %d verts, texture %d\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         (*vbo).array_len,
                                                                                         j);
                                (*vbo).next =
                                    gEngfuncs._Mem_Alloc.expect("non-null function pointer")(vbos.mempool,
                                                                                             ::std::mem::size_of::<vboarray_t>()
                                                                                                 as
                                                                                                 libc::c_ulong,
                                                                                             true_0,
                                                                                             b"../ref_gl/gl_rsurf.c\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             1850
                                                                                                 as
                                                                                                 libc::c_int)
                                        as *mut vboarray_s;
                                vbo = (*vbo).next;
                                (*vbotex).next =
                                    gEngfuncs._Mem_Alloc.expect("non-null function pointer")(vbos.mempool,
                                                                                             ::std::mem::size_of::<vbotexture_t>()
                                                                                                 as
                                                                                                 libc::c_ulong,
                                                                                             true_0,
                                                                                             b"../ref_gl/gl_rsurf.c\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             1852
                                                                                                 as
                                                                                                 libc::c_int)
                                        as *mut vbotexture_s;
                                vbotex = (*vbotex).next;
                                // never skip this textures and lightmaps
                                if vbos.minarraysplit_tex > j {
                                    vbos.minarraysplit_tex = j
                                }
                                if vbos.minarraysplit_lm > k {
                                    vbos.minarraysplit_lm = k
                                }
                                if vbos.maxarraysplit_tex <
                                       j + 1 as libc::c_int {
                                    vbos.maxarraysplit_tex =
                                        j + 1 as libc::c_int
                                }
                                if vbos.maxarraysplit_lm <
                                       k + 1 as libc::c_int {
                                    vbos.maxarraysplit_lm =
                                        k + 1 as libc::c_int
                                }
                            }
                            let ref mut fresh8 =
                                (*vbos.surfdata.offset(i as
                                                           isize)).vbotexture;
                            *fresh8 = vbotex;
                            (*vbos.surfdata.offset(i as isize)).startindex =
                                (*vbo).array_len as uint;
                            (*vbos.surfdata.offset(i as isize)).texturenum =
                                j as uint;
                            (*vbo).array_len += (*(*surf).polys).numverts;
                            (*vbotex).len =
                                ((*vbotex).len as
                                     libc::c_uint).wrapping_add((*(*surf).polys).numverts
                                                                    as
                                                                    libc::c_uint)
                                    as uint as uint;
                            (*vbotex).vboarray = vbo
                        }
                    }
                }
                i += 1
            }
            j += 1
        }
        k += 1
    }
    // allocate last array
    (*vbo).array =
        gEngfuncs._Mem_Alloc.expect("non-null function pointer")(vbos.mempool,
                                                                 (::std::mem::size_of::<vbovertex_t>()
                                                                      as
                                                                      libc::c_ulong).wrapping_mul((*vbo).array_len
                                                                                                      as
                                                                                                      libc::c_ulong),
                                                                 true_0,
                                                                 b"../ref_gl/gl_rsurf.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 1876 as
                                                                     libc::c_int)
            as *mut vbovertex_t;
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"R_GenerateVBOs: allocated array of %d verts\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             (*vbo).array_len);
    // switch to list begin
    vbo = vbos.arraylist;
    // fill and upload
    k = 0 as libc::c_int;
    while k < numlightmaps {
        let mut j_0: libc::c_int = 0;
        j_0 = 0 as libc::c_int;
        while j_0 < numtextures {
            let mut i_0: libc::c_int = 0;
            let mut vbotex_0: *mut vbotexture_t =
                &mut *vbos.textures.offset((k * numtextures + j_0) as isize)
                    as *mut vbotexture_t;
            // preallocate index arrays
            (*vbotex_0).indexarray =
                gEngfuncs._Mem_Alloc.expect("non-null function pointer")(vbos.mempool,
                                                                         (::std::mem::size_of::<libc::c_ushort>()
                                                                              as
                                                                              libc::c_ulong).wrapping_mul(6
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              libc::c_ulong).wrapping_mul((*vbotex_0).len
                                                                                                                                              as
                                                                                                                                              libc::c_ulong),
                                                                         true_0,
                                                                         b"../ref_gl/gl_rsurf.c\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char,
                                                                         1893
                                                                             as
                                                                             libc::c_int)
                    as *mut libc::c_ushort;
            (*vbotex_0).lightmaptexturenum = k as uint;
            if maxindex < (*vbotex_0).len { maxindex = (*vbotex_0).len }
            i_0 = 0 as libc::c_int;
            while i_0 <
                      (*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                             as
                                                                                             libc::c_int)).numsurfaces
                  {
                let mut surf_0: *mut msurface_t =
                    &mut *(*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                                 as
                                                                                                 libc::c_int)).surfaces.offset(i_0
                                                                                                                                   as
                                                                                                                                   isize)
                        as *mut msurface_t;
                let mut l: libc::c_int = 0;
                if !((*surf_0).flags as libc::c_uint &
                         ((1 as libc::c_uint) << 2 as libc::c_int |
                              (1 as libc::c_uint) << 4 as libc::c_int |
                              (1 as libc::c_uint) << 6 as libc::c_int |
                              (1 as libc::c_uint) << 3 as libc::c_int) != 0) {
                    if !((*surf_0).lightmaptexturenum != k) {
                        if !(R_TextureAnimation(surf_0) !=
                                 *(*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                                         as
                                                                                                         libc::c_int)).textures.offset(j_0
                                                                                                                                           as
                                                                                                                                           isize))
                           {
                            // switch to next array
                            if len + (*(*surf_0).polys).numverts >
                                   32767 as libc::c_int * 2 as libc::c_int +
                                       1 as libc::c_int {
                                // upload last generated array
                                pglGenBuffersARB.expect("non-null function pointer")(1
                                                                                         as
                                                                                         libc::c_int,
                                                                                     &mut (*vbo).glindex);
                                pglBindBufferARB.expect("non-null function pointer")(0x8892
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         GLenum,
                                                                                     (*vbo).glindex);
                                pglBufferDataARB.expect("non-null function pointer")(0x8892
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         GLenum,
                                                                                     ((*vbo).array_len
                                                                                          as
                                                                                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<vbovertex_t>()
                                                                                                                          as
                                                                                                                          libc::c_ulong)
                                                                                         as
                                                                                         GLsizeiptrARB,
                                                                                     (*vbo).array
                                                                                         as
                                                                                         *const libc::c_void,
                                                                                     0x88e4
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         GLenum);
                                if !(len == (*vbo).array_len) {
                                    gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"../ref_gl/gl_rsurf.c\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             1921
                                                                                                 as
                                                                                                 libc::c_int);
                                }
                                vbo = (*vbo).next;
                                vbotex_0 = (*vbotex_0).next;
                                (*vbotex_0).indexarray =
                                    gEngfuncs._Mem_Alloc.expect("non-null function pointer")(vbos.mempool,
                                                                                             (::std::mem::size_of::<libc::c_ushort>()
                                                                                                  as
                                                                                                  libc::c_ulong).wrapping_mul(6
                                                                                                                                  as
                                                                                                                                  libc::c_int
                                                                                                                                  as
                                                                                                                                  libc::c_ulong).wrapping_mul((*vbotex_0).len
                                                                                                                                                                  as
                                                                                                                                                                  libc::c_ulong),
                                                                                             true_0,
                                                                                             b"../ref_gl/gl_rsurf.c\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             1925
                                                                                                 as
                                                                                                 libc::c_int)
                                        as *mut libc::c_ushort;
                                (*vbotex_0).lightmaptexturenum = k as uint;
                                // calculate limits for dlights
                                if maxindex < (*vbotex_0).len {
                                    maxindex = (*vbotex_0).len
                                }
                                len = 0 as libc::c_int
                            }
                            // fill vbovertex_t
                            l = 0 as libc::c_int;
                            while l < (*(*surf_0).polys).numverts {
                                let mut v: *mut libc::c_float =
                                    (*(*surf_0).polys).verts[l as
                                                                 usize].as_mut_ptr();
                                (*(*vbo).array.offset((len + l) as
                                                          isize)).pos[0 as
                                                                          libc::c_int
                                                                          as
                                                                          usize]
                                    = *v.offset(0 as libc::c_int as isize);
                                (*(*vbo).array.offset((len + l) as
                                                          isize)).pos[1 as
                                                                          libc::c_int
                                                                          as
                                                                          usize]
                                    = *v.offset(1 as libc::c_int as isize);
                                (*(*vbo).array.offset((len + l) as
                                                          isize)).pos[2 as
                                                                          libc::c_int
                                                                          as
                                                                          usize]
                                    = *v.offset(2 as libc::c_int as isize);
                                (*(*vbo).array.offset((len + l) as
                                                          isize)).gl_tc[0 as
                                                                            libc::c_int
                                                                            as
                                                                            usize]
                                    = *v.offset(3 as libc::c_int as isize);
                                (*(*vbo).array.offset((len + l) as
                                                          isize)).gl_tc[1 as
                                                                            libc::c_int
                                                                            as
                                                                            usize]
                                    = *v.offset(4 as libc::c_int as isize);
                                (*(*vbo).array.offset((len + l) as
                                                          isize)).lm_tc[0 as
                                                                            libc::c_int
                                                                            as
                                                                            usize]
                                    = *v.offset(5 as libc::c_int as isize);
                                (*(*vbo).array.offset((len + l) as
                                                          isize)).lm_tc[1 as
                                                                            libc::c_int
                                                                            as
                                                                            usize]
                                    = *v.offset(6 as libc::c_int as isize);
                                l += 1
                            }
                            len += (*(*surf_0).polys).numverts
                        }
                    }
                }
                i_0 += 1
            }
            j_0 += 1
        }
        k += 1
    }
    if !(len == (*vbo).array_len) {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 b"../ref_gl/gl_rsurf.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 1960 as
                                                                     libc::c_int);
    }
    // upload last array
    pglGenBuffersARB.expect("non-null function pointer")(1 as libc::c_int,
                                                         &mut (*vbo).glindex);
    pglBindBufferARB.expect("non-null function pointer")(0x8892 as libc::c_int
                                                             as GLenum,
                                                         (*vbo).glindex);
    pglBufferDataARB.expect("non-null function pointer")(0x8892 as libc::c_int
                                                             as GLenum,
                                                         ((*vbo).array_len as
                                                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<vbovertex_t>()
                                                                                              as
                                                                                              libc::c_ulong)
                                                             as GLsizeiptrARB,
                                                         (*vbo).array as
                                                             *const libc::c_void,
                                                         0x88e4 as libc::c_int
                                                             as GLenum);
    // prepare decal array
    pglGenBuffersARB.expect("non-null function pointer")(1 as libc::c_int,
                                                         &mut (*vbos.decaldata).decalvbo);
    pglBindBufferARB.expect("non-null function pointer")(0x8892 as libc::c_int
                                                             as GLenum,
                                                         (*vbos.decaldata).decalvbo);
    pglBufferDataARB.expect("non-null function pointer")(0x8892 as libc::c_int
                                                             as GLenum,
                                                         (::std::mem::size_of::<vbovertex_t>()
                                                              as
                                                              libc::c_ulong).wrapping_mul(8
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_ulong).wrapping_mul(4096
                                                                                                                              as
                                                                                                                              libc::c_int
                                                                                                                              as
                                                                                                                              libc::c_ulong)
                                                             as GLsizeiptrARB,
                                                         (*vbos.decaldata).decalarray.as_mut_ptr()
                                                             as
                                                             *const libc::c_void,
                                                         0x88e8 as libc::c_int
                                                             as GLenum);
    // preallocate dlight arrays
    vbos.dlight_index =
        gEngfuncs._Mem_Alloc.expect("non-null function pointer")(vbos.mempool,
                                                                 (maxindex as
                                                                      libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_ushort>()
                                                                                                      as
                                                                                                      libc::c_ulong).wrapping_mul(6
                                                                                                                                      as
                                                                                                                                      libc::c_int
                                                                                                                                      as
                                                                                                                                      libc::c_ulong),
                                                                 true_0,
                                                                 b"../ref_gl/gl_rsurf.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 1973 as
                                                                     libc::c_int)
            as *mut libc::c_ushort;
    // select maximum possible length for dlight
    vbos.dlight_tc =
        gEngfuncs._Mem_Alloc.expect("non-null function pointer")(vbos.mempool,
                                                                 (::std::mem::size_of::<vec2_t>()
                                                                      as
                                                                      libc::c_ulong).wrapping_mul((if !(*vbos.arraylist).next.is_null()
                                                                                                      {
                                                                                                       (32767
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            *
                                                                                                            2
                                                                                                                as
                                                                                                                libc::c_int
                                                                                                            +
                                                                                                            1
                                                                                                                as
                                                                                                                libc::c_int)
                                                                                                           +
                                                                                                           1
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                   } else {
                                                                                                       ((*vbos.arraylist).array_len)
                                                                                                           +
                                                                                                           1
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                   })
                                                                                                      as
                                                                                                      libc::c_ulong),
                                                                 true_0,
                                                                 b"../ref_gl/gl_rsurf.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 1976 as
                                                                     libc::c_int)
            as *mut vec2_t;
    if if !r_vbo_dlightmode.is_null() && (*r_vbo_dlightmode).value != 0.0f32 {
           true_0 as libc::c_int
       } else { false_0 as libc::c_int } != 0 {
        pglGenBuffersARB.expect("non-null function pointer")(1 as libc::c_int,
                                                             &mut vbos.dlight_vbo);
        pglBindBufferARB.expect("non-null function pointer")(0x8892 as
                                                                 libc::c_int
                                                                 as GLenum,
                                                             vbos.dlight_vbo);
        pglBufferDataARB.expect("non-null function pointer")(0x8892 as
                                                                 libc::c_int
                                                                 as GLenum,
                                                             (::std::mem::size_of::<vec2_t>()
                                                                  as
                                                                  libc::c_ulong).wrapping_mul((if !(*vbos.arraylist).next.is_null()
                                                                                                  {
                                                                                                   (32767
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        *
                                                                                                        2
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                        +
                                                                                                        1
                                                                                                            as
                                                                                                            libc::c_int)
                                                                                                       +
                                                                                                       1
                                                                                                           as
                                                                                                           libc::c_int
                                                                                               } else {
                                                                                                   ((*vbos.arraylist).array_len)
                                                                                                       +
                                                                                                       1
                                                                                                           as
                                                                                                           libc::c_int
                                                                                               })
                                                                                                  as
                                                                                                  libc::c_ulong)
                                                                 as
                                                                 GLsizeiptrARB,
                                                             vbos.dlight_tc as
                                                                 *const libc::c_void,
                                                             0x88e0 as
                                                                 libc::c_int
                                                                 as GLenum);
        pglGenBuffersARB.expect("non-null function pointer")(1 as libc::c_int,
                                                             &mut vbos.decal_dlight_vbo);
        pglBindBufferARB.expect("non-null function pointer")(0x8892 as
                                                                 libc::c_int
                                                                 as GLenum,
                                                             vbos.decal_dlight_vbo);
        pglBufferDataARB.expect("non-null function pointer")(0x8892 as
                                                                 libc::c_int
                                                                 as GLenum,
                                                             ::std::mem::size_of::<[vbovertex_t; 131072]>()
                                                                 as
                                                                 libc::c_ulong
                                                                 as
                                                                 GLsizeiptrARB,
                                                             vbos.decal_dlight.as_mut_ptr()
                                                                 as
                                                                 *const libc::c_void,
                                                             0x88e0 as
                                                                 libc::c_int
                                                                 as GLenum);
    }
    // reset state
    pglBindBufferARB.expect("non-null function pointer")(0x8892 as libc::c_int
                                                             as GLenum,
                                                         0 as libc::c_int as
                                                             GLuint);
    mtst.tmu_gl = XASH_TEXTURE0 as libc::c_int;
}
/*
==============
R_AddDecalVBO

generate decal mesh and put it to array
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_AddDecalVBO(mut pdecal: *mut decal_t,
                                       mut surf: *mut msurface_t) {
    let mut numVerts: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut v: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut decalindex: libc::c_int =
        pdecal.wrapping_offset_from(&mut *gDecalPool.as_mut_ptr().offset(0 as
                                                                             libc::c_int
                                                                             as
                                                                             isize)
                                        as *mut decal_t) as libc::c_long as
            libc::c_int;
    if vbos.decaldata.is_null() { return }
    v =
        R_DecalSetupVerts(pdecal, surf, (*pdecal).texture as libc::c_int,
                          &mut numVerts);
    if numVerts > 8 as libc::c_int {
        // use client arrays
        (*vbos.decaldata).decals[decalindex as usize].numVerts =
            -(1 as libc::c_int);
        return
    }
    i = 0 as libc::c_int;
    while i < numVerts {
        memcpy(&mut *(*vbos.decaldata).decalarray.as_mut_ptr().offset((decalindex
                                                                           *
                                                                           8
                                                                               as
                                                                               libc::c_int
                                                                           +
                                                                           i)
                                                                          as
                                                                          isize)
                   as *mut vbovertex_t as *mut libc::c_void,
               v.offset((i * 7 as libc::c_int) as isize) as
                   *const libc::c_void,
               (7 as libc::c_int * 4 as libc::c_int) as libc::c_ulong);
        i += 1
    }
    pglBindBufferARB.expect("non-null function pointer")(0x8892 as libc::c_int
                                                             as GLenum,
                                                         (*vbos.decaldata).decalvbo);
    pglBufferSubDataARB.expect("non-null function pointer")(0x8892 as
                                                                libc::c_int as
                                                                GLenum,
                                                            (decalindex as
                                                                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<vbovertex_t>()
                                                                                                 as
                                                                                                 libc::c_ulong).wrapping_mul(8
                                                                                                                                 as
                                                                                                                                 libc::c_int
                                                                                                                                 as
                                                                                                                                 libc::c_ulong)
                                                                as
                                                                GLintptrARB,
                                                            (::std::mem::size_of::<vbovertex_t>()
                                                                 as
                                                                 libc::c_ulong).wrapping_mul(numVerts
                                                                                                 as
                                                                                                 libc::c_ulong)
                                                                as
                                                                GLsizeiptrARB,
                                                            &mut *(*vbos.decaldata).decalarray.as_mut_ptr().offset((decalindex
                                                                                                                        *
                                                                                                                        8
                                                                                                                            as
                                                                                                                            libc::c_int)
                                                                                                                       as
                                                                                                                       isize)
                                                                as
                                                                *mut vbovertex_t
                                                                as
                                                                *const libc::c_void);
    pglBindBufferARB.expect("non-null function pointer")(0x8892 as libc::c_int
                                                             as GLenum,
                                                         0 as libc::c_int as
                                                             GLuint);
    (*vbos.decaldata).decals[decalindex as usize].numVerts = numVerts;
}
/*
=============
R_ClearVBO

free all vbo data
=============
*/
#[no_mangle]
pub unsafe extern "C" fn R_ClearVBO() {
    let mut vbo: *mut vboarray_t = 0 as *mut vboarray_t;
    vbo = vbos.arraylist;
    while !vbo.is_null() {
        pglDeleteBuffersARB.expect("non-null function pointer")(1 as
                                                                    libc::c_int,
                                                                &mut (*vbo).glindex);
        vbo = (*vbo).next
    }
    vbos.arraylist = 0 as *mut vboarray_t;
    if !vbos.decaldata.is_null() {
        pglDeleteBuffersARB.expect("non-null function pointer")(1 as
                                                                    libc::c_int,
                                                                &mut (*vbos.decaldata).decalvbo);
    }
    if vbos.dlight_vbo != 0 {
        pglDeleteBuffersARB.expect("non-null function pointer")(1 as
                                                                    libc::c_int,
                                                                &mut vbos.dlight_vbo);
    }
    if vbos.decal_dlight_vbo != 0 {
        pglDeleteBuffersARB.expect("non-null function pointer")(1 as
                                                                    libc::c_int,
                                                                &mut vbos.decal_dlight_vbo);
    }
    vbos.dlight_vbo = 0 as libc::c_int as libc::c_uint;
    vbos.decal_dlight_vbo = vbos.dlight_vbo;
    vbos.decaldata = 0 as *mut vbodecaldata_t;
    gEngfuncs._Mem_FreePool.expect("non-null function pointer")(&mut vbos.mempool,
                                                                b"../ref_gl/gl_rsurf.c\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char,
                                                                2056 as
                                                                    libc::c_int);
}
/*
===================
R_DisableDetail

disable detail tmu
===================
*/
unsafe extern "C" fn R_DisableDetail() {
    if mtst.details_enabled as libc::c_uint != 0 &&
           mtst.tmu_dt != -(1 as libc::c_int) {
        GL_SelectTexture(mtst.tmu_dt);
        pglDisableClientState.expect("non-null function pointer")(0x8078 as
                                                                      libc::c_int
                                                                      as
                                                                      GLenum);
        pglDisable.expect("non-null function pointer")(0xde1 as libc::c_int as
                                                           GLenum);
        pglLoadIdentity.expect("non-null function pointer")();
    };
}
/*
===================
R_EnableDetail

enable detail tmu if availiable
===================
*/
unsafe extern "C" fn R_EnableDetail() {
    if mtst.details_enabled as libc::c_uint != 0 &&
           mtst.tmu_dt != -(1 as libc::c_int) {
        GL_SelectTexture(mtst.tmu_dt);
        pglEnableClientState.expect("non-null function pointer")(0x8078 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum);
        pglEnable.expect("non-null function pointer")(0xde1 as libc::c_int as
                                                          GLenum);
        pglTexEnvi.expect("non-null function pointer")(0x2300 as libc::c_int
                                                           as GLenum,
                                                       0x2200 as libc::c_int
                                                           as GLenum,
                                                       0x8570 as libc::c_int);
        pglTexEnvi.expect("non-null function pointer")(0x2300 as libc::c_int
                                                           as GLenum,
                                                       0x8571 as libc::c_int
                                                           as GLenum,
                                                       0x2100 as libc::c_int);
        pglTexEnvi.expect("non-null function pointer")(0x2300 as libc::c_int
                                                           as GLenum,
                                                       0x8580 as libc::c_int
                                                           as GLenum,
                                                       0x8578 as libc::c_int);
        pglTexEnvi.expect("non-null function pointer")(0x2300 as libc::c_int
                                                           as GLenum,
                                                       0x8581 as libc::c_int
                                                           as GLenum,
                                                       0x1702 as libc::c_int);
        pglTexEnvi.expect("non-null function pointer")(0x2300 as libc::c_int
                                                           as GLenum,
                                                       0x8573 as libc::c_int
                                                           as GLenum,
                                                       2 as libc::c_int);
        // use transform matrix for details (undone)
        pglTexCoordPointer.expect("non-null function pointer")(2 as
                                                                   libc::c_int,
                                                               0x1406 as
                                                                   libc::c_int
                                                                   as GLenum,
                                                               ::std::mem::size_of::<vbovertex_t>()
                                                                   as
                                                                   libc::c_ulong
                                                                   as GLsizei,
                                                               12 as
                                                                   libc::c_ulong
                                                                   as
                                                                   *mut libc::c_void);
        pglMatrixMode.expect("non-null function pointer")(0x1702 as
                                                              libc::c_int as
                                                              GLenum);
        pglLoadIdentity.expect("non-null function pointer")();
        pglScalef.expect("non-null function pointer")((*mtst.glt).xscale,
                                                      (*mtst.glt).yscale,
                                                      1 as libc::c_int as
                                                          GLfloat);
    };
}
/*
==============
R_SetLightmap

enable lightmap on current tmu
==============
*/
unsafe extern "C" fn R_SetLightmap() {
    if mtst.skiptexture as u64 != 0 { return }
    /*if( gl_overbright->integer )
	{
		pglTexEnvi( GL_TEXTURE_ENV, GL_TEXTURE_ENV_MODE, GL_COMBINE_ARB );
		pglTexEnvi( GL_TEXTURE_ENV, GL_COMBINE_RGB_ARB, GL_MODULATE );
		pglTexEnvi( GL_TEXTURE_ENV, GL_SOURCE0_RGB_ARB, GL_PREVIOUS_ARB );
		pglTexEnvi( GL_TEXTURE_ENV, GL_SOURCE1_RGB_ARB, GL_TEXTURE );
		pglTexEnvi( GL_TEXTURE_ENV, GL_RGB_SCALE_ARB, 2 );

	}
	else*/
    pglTexEnvi.expect("non-null function pointer")(0x2300 as libc::c_int as
                                                       GLenum,
                                                   0x2200 as libc::c_int as
                                                       GLenum,
                                                   0x2100 as libc::c_int);
    pglTexCoordPointer.expect("non-null function pointer")(2 as libc::c_int,
                                                           0x1406 as
                                                               libc::c_int as
                                                               GLenum,
                                                           ::std::mem::size_of::<vbovertex_t>()
                                                               as
                                                               libc::c_ulong
                                                               as GLsizei,
                                                           20 as libc::c_ulong
                                                               as
                                                               *mut libc::c_void);
}
/*
==============
R_SetDecalMode

When drawing decal, disable or restore bump and details
set tmu to lightmap when enabled
==============
*/
unsafe extern "C" fn R_SetDecalMode(mut enable: qboolean) {
    // order is important to correctly rearrange TMUs
    if enable as u64 != 0 {
        // disable detail texture if enabled
        R_DisableDetail();
    } else { R_EnableDetail(); };
}
/*
==============
R_SetupVBOTexture

setup multitexture mode before drawing VBOs
if tex is NULL, load texture by number
==============
*/
unsafe extern "C" fn R_SetupVBOTexture(mut tex: *mut texture_t,
                                       mut number: libc::c_int)
 -> *mut texture_t {
    if mtst.skiptexture as u64 != 0 { return tex }
    if tex.is_null() {
        tex =
            R_TextureAnim(*(*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                                  as
                                                                                                  libc::c_int)).textures.offset(number
                                                                                                                                    as
                                                                                                                                    isize))
    }
    if (if !r_detailtextures.is_null() && (*r_detailtextures).value != 0.0f32
           {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int }) != 0 &&
           (*tex).dt_texturenum as libc::c_int != 0 &&
           mtst.tmu_dt != -(1 as libc::c_int) {
        mtst.details_enabled = true_0;
        GL_Bind(mtst.tmu_dt, (*tex).dt_texturenum as GLenum);
        mtst.glt = R_GetTexture((*tex).gl_texturenum as GLenum);
        R_EnableDetail();
    } else { R_DisableDetail(); }
    GL_Bind(mtst.tmu_gl,
            if if !r_lightmap.is_null() && (*r_lightmap).value != 0.0f32 {
                   true_0 as libc::c_int
               } else { false_0 as libc::c_int } != 0 {
                tr.whiteTexture
            } else { (*tex).gl_texturenum } as GLenum);
    return tex;
}
/*
===================
R_AdditionalPasses

draw details when not enough tmus
===================
*/
unsafe extern "C" fn R_AdditionalPasses(mut vbo: *mut vboarray_t,
                                        mut indexlen: libc::c_int,
                                        mut indexarray: *mut libc::c_void,
                                        mut tex: *mut texture_t,
                                        mut resetvbo: qboolean) {
    // draw details in additional pass
    if (*r_detailtextures).value != 0. && mtst.tmu_dt == -(1 as libc::c_int)
           && (*tex).dt_texturenum as libc::c_int != 0 {
        let mut glt: *mut gl_texture_t =
            R_GetTexture((*tex).gl_texturenum as GLenum);
        GL_SelectTexture(XASH_TEXTURE1 as libc::c_int);
        pglDisable.expect("non-null function pointer")(0xde1 as libc::c_int as
                                                           GLenum);
        // setup detail
        GL_Bind(XASH_TEXTURE0 as libc::c_int, (*tex).dt_texturenum as GLenum);
        pglEnable.expect("non-null function pointer")(0xbe2 as libc::c_int as
                                                          GLenum);
        pglBlendFunc.expect("non-null function pointer")(0x306 as libc::c_int
                                                             as GLenum,
                                                         0x300 as libc::c_int
                                                             as GLenum);
        pglTexEnvi.expect("non-null function pointer")(0x2300 as libc::c_int
                                                           as GLenum,
                                                       0x2200 as libc::c_int
                                                           as GLenum,
                                                       0x2101 as libc::c_int);
        // when drawing dlights, we need to bind array and unbind it again
        if resetvbo as u64 != 0 {
            pglBindBufferARB.expect("non-null function pointer")(0x8892 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 (*vbo).glindex);
        }
        pglTexCoordPointer.expect("non-null function pointer")(2 as
                                                                   libc::c_int,
                                                               0x1406 as
                                                                   libc::c_int
                                                                   as GLenum,
                                                               ::std::mem::size_of::<vbovertex_t>()
                                                                   as
                                                                   libc::c_ulong
                                                                   as GLsizei,
                                                               12 as
                                                                   libc::c_ulong
                                                                   as
                                                                   *mut libc::c_void);
        // apply scale
        pglMatrixMode.expect("non-null function pointer")(0x1702 as
                                                              libc::c_int as
                                                              GLenum);
        pglLoadIdentity.expect("non-null function pointer")();
        pglScalef.expect("non-null function pointer")((*glt).xscale,
                                                      (*glt).yscale,
                                                      1 as libc::c_int as
                                                          GLfloat);
        // draw
        // WebGL need to know array sizes
        if pglDrawRangeElements.is_some() {
            pglDrawRangeElements.expect("non-null function pointer")(0x4 as
                                                                         libc::c_int
                                                                         as
                                                                         GLenum,
                                                                     0 as
                                                                         libc::c_int
                                                                         as
                                                                         GLuint,
                                                                     (*vbo).array_len
                                                                         as
                                                                         GLuint,
                                                                     indexlen,
                                                                     0x1403 as
                                                                         libc::c_int
                                                                         as
                                                                         GLenum,
                                                                     indexarray);
        } else {
            pglDrawElements.expect("non-null function pointer")(0x4 as
                                                                    libc::c_int
                                                                    as GLenum,
                                                                indexlen,
                                                                0x1403 as
                                                                    libc::c_int
                                                                    as GLenum,
                                                                indexarray);
        }
        // restore state
        pglLoadIdentity.expect("non-null function pointer")();
        pglTexEnvi.expect("non-null function pointer")(0x2300 as libc::c_int
                                                           as GLenum,
                                                       0x2200 as libc::c_int
                                                           as GLenum,
                                                       0x1e01 as libc::c_int);
        pglDisable.expect("non-null function pointer")(0xbe2 as libc::c_int as
                                                           GLenum);
        GL_Bind(XASH_TEXTURE1 as libc::c_int, mtst.lm as GLenum);
        pglTexCoordPointer.expect("non-null function pointer")(2 as
                                                                   libc::c_int,
                                                               0x1406 as
                                                                   libc::c_int
                                                                   as GLenum,
                                                               ::std::mem::size_of::<vbovertex_t>()
                                                                   as
                                                                   libc::c_ulong
                                                                   as GLsizei,
                                                               20 as
                                                                   libc::c_ulong
                                                                   as
                                                                   *mut libc::c_void);
        GL_SelectTexture(XASH_TEXTURE1 as libc::c_int);
        pglEnable.expect("non-null function pointer")(0xde1 as libc::c_int as
                                                          GLenum);
        if resetvbo as u64 != 0 {
            pglBindBufferARB.expect("non-null function pointer")(0x8892 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     GLuint);
        }
    };
}
/*
=====================
R_DrawLightmappedVBO

Draw array for given vbotexture_t. build and draw dynamic lightmaps if present
=====================
*/
unsafe extern "C" fn R_DrawLightmappedVBO(mut vbo: *mut vboarray_t,
                                          mut vbotex: *mut vbotexture_t,
                                          mut texture: *mut texture_t,
                                          mut lightmap: libc::c_int,
                                          mut skiplighting: qboolean) {
    // WebGL need to know array sizes
    if pglDrawRangeElements.is_some() {
        pglDrawRangeElements.expect("non-null function pointer")(0x4 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     GLuint,
                                                                 (*vbo).array_len
                                                                     as
                                                                     GLuint,
                                                                 (*vbotex).curindex
                                                                     as
                                                                     GLsizei,
                                                                 0x1403 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 (*vbotex).indexarray
                                                                     as
                                                                     *const libc::c_void);
    } else {
        pglDrawElements.expect("non-null function pointer")(0x4 as libc::c_int
                                                                as GLenum,
                                                            (*vbotex).curindex
                                                                as GLsizei,
                                                            0x1403 as
                                                                libc::c_int as
                                                                GLenum,
                                                            (*vbotex).indexarray
                                                                as
                                                                *const libc::c_void);
    }
    R_AdditionalPasses(vbo, (*vbotex).curindex as libc::c_int,
                       (*vbotex).indexarray as *mut libc::c_void, texture,
                       false_0);
    // draw debug lines
    if (if !gl_wireframe.is_null() && (*gl_wireframe).value != 0.0f32 {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int }) != 0 && skiplighting as u64 == 0 {
        R_SetDecalMode(true_0);
        pglDisable.expect("non-null function pointer")(0xde1 as libc::c_int as
                                                           GLenum);
        GL_SelectTexture(XASH_TEXTURE0 as libc::c_int);
        pglDisable.expect("non-null function pointer")(0xde1 as libc::c_int as
                                                           GLenum);
        pglDisable.expect("non-null function pointer")(0xb71 as libc::c_int as
                                                           GLenum);
        // WebGL need to know array sizes
        if pglDrawRangeElements.is_some() {
            pglDrawRangeElements.expect("non-null function pointer")(0x1 as
                                                                         libc::c_int
                                                                         as
                                                                         GLenum,
                                                                     0 as
                                                                         libc::c_int
                                                                         as
                                                                         GLuint,
                                                                     (*vbo).array_len
                                                                         as
                                                                         GLuint,
                                                                     (*vbotex).curindex
                                                                         as
                                                                         GLsizei,
                                                                     0x1403 as
                                                                         libc::c_int
                                                                         as
                                                                         GLenum,
                                                                     (*vbotex).indexarray
                                                                         as
                                                                         *const libc::c_void);
        } else {
            pglDrawElements.expect("non-null function pointer")(0x1 as
                                                                    libc::c_int
                                                                    as GLenum,
                                                                (*vbotex).curindex
                                                                    as
                                                                    GLsizei,
                                                                0x1403 as
                                                                    libc::c_int
                                                                    as GLenum,
                                                                (*vbotex).indexarray
                                                                    as
                                                                    *const libc::c_void);
        }
        pglEnable.expect("non-null function pointer")(0xb71 as libc::c_int as
                                                          GLenum);
        pglEnable.expect("non-null function pointer")(0xde1 as libc::c_int as
                                                          GLenum);
        GL_SelectTexture(XASH_TEXTURE1 as libc::c_int);
        pglEnable.expect("non-null function pointer")(0xde1 as libc::c_int as
                                                          GLenum);
        R_SetDecalMode(false_0);
    }
    //Msg( "%d %d %d\n", vbo->array_len, vbotex->len, lightmap );
    if skiplighting as u64 != 0 {
        (*vbotex).curindex = 0 as libc::c_int as uint;
        (*vbotex).dlightchain = 0 as *mut msurface_t;
        return
    }
    // draw dlights and dlighted decals
    if !(*vbotex).dlightchain.is_null() {
        let mut dlightarray: *mut libc::c_ushort =
            vbos.dlight_index; // preallocated array
        let mut dlightindex: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        let mut surf: *mut msurface_t = 0 as *mut msurface_t;
        let mut newsurf: *mut msurface_t = 0 as *mut msurface_t;
        let mut decalcount: libc::c_int = 0 as libc::c_int;
        GL_Bind(mtst.tmu_lm, tr.dlightTexture as GLenum);
        // replace lightmap texcoord array by dlight array
        pglBindBufferARB.expect("non-null function pointer")(0x8892 as
                                                                 libc::c_int
                                                                 as GLenum,
                                                             vbos.dlight_vbo);
        if vbos.dlight_vbo != 0 {
            pglTexCoordPointer.expect("non-null function pointer")(2 as
                                                                       libc::c_int,
                                                                   0x1406 as
                                                                       libc::c_int
                                                                       as
                                                                       GLenum,
                                                                   (::std::mem::size_of::<libc::c_float>()
                                                                        as
                                                                        libc::c_ulong).wrapping_mul(2
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_ulong)
                                                                       as
                                                                       GLsizei,
                                                                   0 as
                                                                       *const libc::c_void);
        } else {
            pglTexCoordPointer.expect("non-null function pointer")(2 as
                                                                       libc::c_int,
                                                                   0x1406 as
                                                                       libc::c_int
                                                                       as
                                                                       GLenum,
                                                                   (::std::mem::size_of::<libc::c_float>()
                                                                        as
                                                                        libc::c_ulong).wrapping_mul(2
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_ulong)
                                                                       as
                                                                       GLsizei,
                                                                   vbos.dlight_tc
                                                                       as
                                                                       *const libc::c_void);
        }
        // clear the block
        LM_InitBlock();
        // accumulate indexes for every dlighted surface until dlight block full
        newsurf = (*vbotex).dlightchain;
        surf = newsurf;
        while !surf.is_null() {
            let mut smax: libc::c_int = 0;
            let mut tmax: libc::c_int = 0;
            let mut base: *mut byte = 0 as *mut byte;
            let mut indexbase: uint =
                (*vbos.surfdata.offset(((surf as
                                             *mut libc::c_char).wrapping_offset_from((*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                                                                                            as
                                                                                                                                                            libc::c_int)).surfaces
                                                                                         as
                                                                                         *mut libc::c_char)
                                            as libc::c_long as
                                            libc::c_ulong).wrapping_div(::std::mem::size_of::<msurface_t>()
                                                                            as
                                                                            libc::c_ulong)
                                           as isize)).startindex;
            let mut index: uint = 0;
            //info->dlight_s = info->dlight_t = 0;
            let mut info: *mut mextrasurf_t =
                0 as *mut mextrasurf_t; // this stores current dlight offset
            let mut pdecal: *mut decal_t = 0 as *mut decal_t;
            let mut sample_size: libc::c_int = 0;
            info = (*surf).info;
            sample_size =
                gEngfuncs.Mod_SampleSizeForFace.expect("non-null function pointer")(surf);
            smax =
                (*info).lightextents[0 as libc::c_int as usize] as libc::c_int
                    / sample_size + 1 as libc::c_int;
            tmax =
                (*info).lightextents[1 as libc::c_int as usize] as libc::c_int
                    / sample_size + 1 as libc::c_int;
            if LM_AllocBlock(smax, tmax, &mut (*info).dlight_s,
                             &mut (*info).dlight_t) != 0 {
                base = gl_lms.lightmap_buffer.as_mut_ptr();
                base =
                    base.offset((((*info).dlight_t * tr.block_size +
                                      (*info).dlight_s) * 4 as libc::c_int) as
                                    isize);
                R_BuildLightMap(surf, base, tr.block_size * 4 as libc::c_int,
                                true_0);
            } else {
                // find space for this surface and get offsets
                // out of free block space. Draw all generated index array and clear it
				// upload already generated block
                LM_UploadDynamicBlock();
                // WebGL need to know array sizes
                if pglDrawRangeElements.is_some() {
                    pglDrawRangeElements.expect("non-null function pointer")(0x4
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 GLenum,
                                                                             0
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 GLuint,
                                                                             (*vbo).array_len
                                                                                 as
                                                                                 GLuint,
                                                                             dlightindex
                                                                                 as
                                                                                 GLsizei,
                                                                             0x1403
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 GLenum,
                                                                             dlightarray
                                                                                 as
                                                                                 *const libc::c_void);
                } else {
                    pglDrawElements.expect("non-null function pointer")(0x4 as
                                                                            libc::c_int
                                                                            as
                                                                            GLenum,
                                                                        dlightindex
                                                                            as
                                                                            GLsizei,
                                                                        0x1403
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            GLenum,
                                                                        dlightarray
                                                                            as
                                                                            *const libc::c_void);
                }
                // draw decals that lighted with this lightmap
                if decalcount != 0 {
                    let mut decalsurf: *mut msurface_t = 0 as *mut msurface_t;
                    let mut decali: libc::c_int = 0 as libc::c_int;
                    pglDepthMask.expect("non-null function pointer")(0 as
                                                                         libc::c_int
                                                                         as
                                                                         GLboolean);
                    pglEnable.expect("non-null function pointer")(0xbe2 as
                                                                      libc::c_int
                                                                      as
                                                                      GLenum);
                    pglEnable.expect("non-null function pointer")(0x8037 as
                                                                      libc::c_int
                                                                      as
                                                                      GLenum);
                    if (*RI.currententity).curstate.rendermode ==
                           kRenderTransAlpha as libc::c_int {
                        pglDisable.expect("non-null function pointer")(0xbc0
                                                                           as
                                                                           libc::c_int
                                                                           as
                                                                           GLenum);
                    }
                    pglBindBufferARB.expect("non-null function pointer")(0x8892
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             GLenum,
                                                                         vbos.decal_dlight_vbo);
                    R_SetDecalMode(true_0);
                    if vbos.decal_dlight_vbo != 0 {
                        pglTexCoordPointer.expect("non-null function pointer")(2
                                                                                   as
                                                                                   libc::c_int,
                                                                               0x1406
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   GLenum,
                                                                               ::std::mem::size_of::<vbovertex_t>()
                                                                                   as
                                                                                   libc::c_ulong
                                                                                   as
                                                                                   GLsizei,
                                                                               20
                                                                                   as
                                                                                   libc::c_ulong
                                                                                   as
                                                                                   *mut libc::c_void);
                        GL_SelectTexture(mtst.tmu_gl);
                        pglTexCoordPointer.expect("non-null function pointer")(2
                                                                                   as
                                                                                   libc::c_int,
                                                                               0x1406
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   GLenum,
                                                                               ::std::mem::size_of::<vbovertex_t>()
                                                                                   as
                                                                                   libc::c_ulong
                                                                                   as
                                                                                   GLsizei,
                                                                               12
                                                                                   as
                                                                                   libc::c_ulong
                                                                                   as
                                                                                   *mut libc::c_void);
                        pglVertexPointer.expect("non-null function pointer")(3
                                                                                 as
                                                                                 libc::c_int,
                                                                             0x1406
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 GLenum,
                                                                             ::std::mem::size_of::<vbovertex_t>()
                                                                                 as
                                                                                 libc::c_ulong
                                                                                 as
                                                                                 GLsizei,
                                                                             0
                                                                                 as
                                                                                 *const libc::c_void);
                    } else {
                        pglTexCoordPointer.expect("non-null function pointer")(2
                                                                                   as
                                                                                   libc::c_int,
                                                                               0x1406
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   GLenum,
                                                                               ::std::mem::size_of::<vbovertex_t>()
                                                                                   as
                                                                                   libc::c_ulong
                                                                                   as
                                                                                   GLsizei,
                                                                               &mut (*vbos.decal_dlight.as_mut_ptr().offset(0
                                                                                                                                as
                                                                                                                                libc::c_int
                                                                                                                                as
                                                                                                                                isize)).lm_tc
                                                                                   as
                                                                                   *mut vec2_t
                                                                                   as
                                                                                   *const libc::c_void);
                        GL_SelectTexture(mtst.tmu_gl);
                        pglTexCoordPointer.expect("non-null function pointer")(2
                                                                                   as
                                                                                   libc::c_int,
                                                                               0x1406
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   GLenum,
                                                                               ::std::mem::size_of::<vbovertex_t>()
                                                                                   as
                                                                                   libc::c_ulong
                                                                                   as
                                                                                   GLsizei,
                                                                               &mut (*vbos.decal_dlight.as_mut_ptr().offset(0
                                                                                                                                as
                                                                                                                                libc::c_int
                                                                                                                                as
                                                                                                                                isize)).gl_tc
                                                                                   as
                                                                                   *mut vec2_t
                                                                                   as
                                                                                   *const libc::c_void);
                        pglVertexPointer.expect("non-null function pointer")(3
                                                                                 as
                                                                                 libc::c_int,
                                                                             0x1406
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 GLenum,
                                                                             ::std::mem::size_of::<vbovertex_t>()
                                                                                 as
                                                                                 libc::c_ulong
                                                                                 as
                                                                                 GLsizei,
                                                                             &mut (*vbos.decal_dlight.as_mut_ptr().offset(0
                                                                                                                              as
                                                                                                                              libc::c_int
                                                                                                                              as
                                                                                                                              isize)).pos
                                                                                 as
                                                                                 *mut vec3_t
                                                                                 as
                                                                                 *const libc::c_void);
                    }
                    decalsurf = newsurf;
                    while decali < decalcount && decalsurf != surf {
                        pdecal = (*decalsurf).pdecals;
                        while !pdecal.is_null() {
                            let mut glt: *mut gl_texture_t =
                                0 as *mut gl_texture_t;
                            if !((*pdecal).texture == 0) {
                                glt =
                                    R_GetTexture((*pdecal).texture as GLenum);
                                GL_Bind(mtst.tmu_gl,
                                        (*pdecal).texture as GLenum);
                                // normal HL decal with alpha-channel
                                if (*glt).flags as libc::c_uint &
                                       TF_HAS_ALPHA as libc::c_int as
                                           libc::c_uint != 0 {
                                    // draw transparent decals with GL_MODULATE
                                    if (*glt).fogParams[3 as libc::c_int as
                                                            usize] as
                                           libc::c_int > 230 as libc::c_int {
                                        pglTexEnvi.expect("non-null function pointer")(0x2300
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           GLenum,
                                                                                       0x2200
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           GLenum,
                                                                                       0x2100
                                                                                           as
                                                                                           libc::c_int);
                                    } else {
                                        pglTexEnvi.expect("non-null function pointer")(0x2300
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           GLenum,
                                                                                       0x2200
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           GLenum,
                                                                                       0x1e01
                                                                                           as
                                                                                           libc::c_int);
                                    }
                                    pglBlendFunc.expect("non-null function pointer")(0x302
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         GLenum,
                                                                                     0x303
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         GLenum);
                                } else {
                                    // color decal like detail texture. Base color is 127 127 127
                                    pglTexEnvi.expect("non-null function pointer")(0x2300
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       GLenum,
                                                                                   0x2200
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       GLenum,
                                                                                   0x1e01
                                                                                       as
                                                                                       libc::c_int);
                                    pglBlendFunc.expect("non-null function pointer")(0x306
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         GLenum,
                                                                                     0x300
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         GLenum);
                                }
                                pglDrawArrays.expect("non-null function pointer")(0x6
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      GLenum,
                                                                                  decali
                                                                                      *
                                                                                      32
                                                                                          as
                                                                                          libc::c_int,
                                                                                  vbos.decal_numverts[decali
                                                                                                          as
                                                                                                          usize]);
                                decali += 1
                            }
                            pdecal = (*pdecal).pnext
                        }
                        newsurf = surf;
                        decalsurf = (*(*decalsurf).info).lightmapchain
                    }
                    // restore states pointers for next dynamic lightmap
                    pglTexEnvi.expect("non-null function pointer")(0x2300 as
                                                                       libc::c_int
                                                                       as
                                                                       GLenum,
                                                                   0x2200 as
                                                                       libc::c_int
                                                                       as
                                                                       GLenum,
                                                                   0x1e01 as
                                                                       libc::c_int);
                    pglDepthMask.expect("non-null function pointer")(0x1 as
                                                                         libc::c_int
                                                                         as
                                                                         GLboolean);
                    pglDisable.expect("non-null function pointer")(0xbe2 as
                                                                       libc::c_int
                                                                       as
                                                                       GLenum);
                    pglDisable.expect("non-null function pointer")(0x8037 as
                                                                       libc::c_int
                                                                       as
                                                                       GLenum);
                    if (*RI.currententity).curstate.rendermode ==
                           kRenderTransAlpha as libc::c_int {
                        pglEnable.expect("non-null function pointer")(0xbc0 as
                                                                          libc::c_int
                                                                          as
                                                                          GLenum);
                    }
                    pglBindBufferARB.expect("non-null function pointer")(0x8892
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             GLenum,
                                                                         vbos.dlight_vbo);
                    R_SetDecalMode(false_0);
                    GL_SelectTexture(mtst.tmu_lm);
                    if vbos.dlight_vbo != 0 {
                        pglTexCoordPointer.expect("non-null function pointer")(2
                                                                                   as
                                                                                   libc::c_int,
                                                                               0x1406
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   GLenum,
                                                                               (::std::mem::size_of::<libc::c_float>()
                                                                                    as
                                                                                    libc::c_ulong).wrapping_mul(2
                                                                                                                    as
                                                                                                                    libc::c_int
                                                                                                                    as
                                                                                                                    libc::c_ulong)
                                                                                   as
                                                                                   GLsizei,
                                                                               0
                                                                                   as
                                                                                   *const libc::c_void);
                    } else {
                        pglTexCoordPointer.expect("non-null function pointer")(2
                                                                                   as
                                                                                   libc::c_int,
                                                                               0x1406
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   GLenum,
                                                                               (::std::mem::size_of::<libc::c_float>()
                                                                                    as
                                                                                    libc::c_ulong).wrapping_mul(2
                                                                                                                    as
                                                                                                                    libc::c_int
                                                                                                                    as
                                                                                                                    libc::c_ulong)
                                                                                   as
                                                                                   GLsizei,
                                                                               vbos.dlight_tc
                                                                                   as
                                                                                   *const libc::c_void);
                    }
                    pglBindBufferARB.expect("non-null function pointer")(0x8892
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             GLenum,
                                                                         (*vbo).glindex);
                    pglVertexPointer.expect("non-null function pointer")(3 as
                                                                             libc::c_int,
                                                                         0x1406
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             GLenum,
                                                                         ::std::mem::size_of::<vbovertex_t>()
                                                                             as
                                                                             libc::c_ulong
                                                                             as
                                                                             GLsizei,
                                                                         0 as
                                                                             *const libc::c_void);
                    R_SetupVBOTexture(texture, 0 as libc::c_int);
                    pglTexCoordPointer.expect("non-null function pointer")(2
                                                                               as
                                                                               libc::c_int,
                                                                           0x1406
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               GLenum,
                                                                           ::std::mem::size_of::<vbovertex_t>()
                                                                               as
                                                                               libc::c_ulong
                                                                               as
                                                                               GLsizei,
                                                                           12
                                                                               as
                                                                               libc::c_ulong
                                                                               as
                                                                               *mut libc::c_void);
                    decalcount = 0 as libc::c_int
                }
                // clear the block
                LM_InitBlock();
                dlightindex = 0 as libc::c_int as libc::c_uint;
                // try upload the block now
                if LM_AllocBlock(smax, tmax, &mut (*info).dlight_s,
                                 &mut (*info).dlight_t) == 0 {
                    gEngfuncs.Host_Error.expect("non-null function pointer")(b"AllocBlock: full\n\x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                }
                base = gl_lms.lightmap_buffer.as_mut_ptr();
                base =
                    base.offset((((*info).dlight_t * tr.block_size +
                                      (*info).dlight_s) * 4 as libc::c_int) as
                                    isize);
                R_BuildLightMap(surf, base, tr.block_size * 4 as libc::c_int,
                                true_0);
            }
            (*vbos.dlight_tc.offset(indexbase as
                                        isize))[0 as libc::c_int as usize] =
                (*(*surf).polys).verts[0 as libc::c_int as
                                           usize][5 as libc::c_int as usize] -
                    ((*surf).light_s - (*info).dlight_s) as libc::c_float *
                        (1.0f32 / tr.block_size as libc::c_float);
            (*vbos.dlight_tc.offset(indexbase as
                                        isize))[1 as libc::c_int as usize] =
                (*(*surf).polys).verts[0 as libc::c_int as
                                           usize][6 as libc::c_int as usize] -
                    ((*surf).light_t - (*info).dlight_t) as libc::c_float *
                        (1.0f32 / tr.block_size as libc::c_float);
            (*vbos.dlight_tc.offset(indexbase.wrapping_add(1 as libc::c_int as
                                                               libc::c_uint)
                                        as isize))[0 as libc::c_int as usize]
                =
                (*(*surf).polys).verts[1 as libc::c_int as
                                           usize][5 as libc::c_int as usize] -
                    ((*surf).light_s - (*info).dlight_s) as libc::c_float *
                        (1.0f32 / tr.block_size as libc::c_float);
            (*vbos.dlight_tc.offset(indexbase.wrapping_add(1 as libc::c_int as
                                                               libc::c_uint)
                                        as isize))[1 as libc::c_int as usize]
                =
                (*(*surf).polys).verts[1 as libc::c_int as
                                           usize][6 as libc::c_int as usize] -
                    ((*surf).light_t - (*info).dlight_t) as libc::c_float *
                        (1.0f32 / tr.block_size as libc::c_float);
            index = indexbase.wrapping_add(2 as libc::c_int as libc::c_uint);
            while index <
                      indexbase.wrapping_add((*(*surf).polys).numverts as
                                                 libc::c_uint) {
                let fresh9 = dlightindex;
                dlightindex = dlightindex.wrapping_add(1);
                *dlightarray.offset(fresh9 as isize) =
                    indexbase as libc::c_ushort;
                let fresh10 = dlightindex;
                dlightindex = dlightindex.wrapping_add(1);
                *dlightarray.offset(fresh10 as isize) =
                    index.wrapping_sub(1 as libc::c_int as libc::c_uint) as
                        libc::c_ushort;
                let fresh11 = dlightindex;
                dlightindex = dlightindex.wrapping_add(1);
                *dlightarray.offset(fresh11 as isize) =
                    index as libc::c_ushort;
                (*vbos.dlight_tc.offset(index as
                                            isize))[0 as libc::c_int as usize]
                    =
                    (*(*surf).polys).verts[index.wrapping_sub(indexbase) as
                                               usize][5 as libc::c_int as
                                                          usize] -
                        ((*surf).light_s - (*info).dlight_s) as libc::c_float
                            * (1.0f32 / tr.block_size as libc::c_float);
                (*vbos.dlight_tc.offset(index as
                                            isize))[1 as libc::c_int as usize]
                    =
                    (*(*surf).polys).verts[index.wrapping_sub(indexbase) as
                                               usize][6 as libc::c_int as
                                                          usize] -
                        ((*surf).light_t - (*info).dlight_t) as libc::c_float
                            * (1.0f32 / tr.block_size as libc::c_float);
                index = index.wrapping_add(1)
            }
            if vbos.dlight_vbo != 0 {
                pglBindBufferARB.expect("non-null function pointer")(0x8892 as
                                                                         libc::c_int
                                                                         as
                                                                         GLenum,
                                                                     vbos.dlight_vbo);
                pglBufferSubDataARB.expect("non-null function pointer")(0x8892
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            GLenum,
                                                                        (::std::mem::size_of::<vec2_t>()
                                                                             as
                                                                             libc::c_ulong).wrapping_mul(indexbase
                                                                                                             as
                                                                                                             libc::c_ulong)
                                                                            as
                                                                            GLintptrARB,
                                                                        (::std::mem::size_of::<vec2_t>()
                                                                             as
                                                                             libc::c_ulong).wrapping_mul((*(*surf).polys).numverts
                                                                                                             as
                                                                                                             libc::c_ulong)
                                                                            as
                                                                            GLsizeiptrARB,
                                                                        vbos.dlight_tc.offset(indexbase
                                                                                                  as
                                                                                                  isize)
                                                                            as
                                                                            *const libc::c_void);
            }
            pdecal = (*surf).pdecals;
            while !pdecal.is_null() {
                let mut decalindex: libc::c_int =
                    pdecal.wrapping_offset_from(&mut *gDecalPool.as_mut_ptr().offset(0
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         isize)
                                                    as *mut decal_t) as
                        libc::c_long as libc::c_int;
                let mut numVerts: libc::c_int =
                    (*vbos.decaldata).decals[decalindex as usize].numVerts;
                let mut i: libc::c_int = 0;
                if numVerts == -(1 as libc::c_int) {
                    // build index and texcoords arrays
                    // if surface has decals, build decal array
                    // build decal array
                    let mut v: *mut libc::c_float =
                        R_DecalSetupVerts(pdecal, surf,
                                          (*pdecal).texture as libc::c_int,
                                          &mut numVerts);
                    i = 0 as libc::c_int;
                    while i < numVerts {
                        vbos.decal_dlight[(decalcount * 32 as libc::c_int + i)
                                              as
                                              usize].pos[0 as libc::c_int as
                                                             usize] =
                            *v.offset(0 as libc::c_int as isize);
                        vbos.decal_dlight[(decalcount * 32 as libc::c_int + i)
                                              as
                                              usize].pos[1 as libc::c_int as
                                                             usize] =
                            *v.offset(1 as libc::c_int as isize);
                        vbos.decal_dlight[(decalcount * 32 as libc::c_int + i)
                                              as
                                              usize].pos[2 as libc::c_int as
                                                             usize] =
                            *v.offset(2 as libc::c_int as isize);
                        vbos.decal_dlight[(decalcount * 32 as libc::c_int + i)
                                              as
                                              usize].gl_tc[0 as libc::c_int as
                                                               usize] =
                            *v.offset(3 as libc::c_int as isize);
                        vbos.decal_dlight[(decalcount * 32 as libc::c_int + i)
                                              as
                                              usize].gl_tc[1 as libc::c_int as
                                                               usize] =
                            *v.offset(4 as libc::c_int as isize);
                        vbos.decal_dlight[(decalcount * 32 as libc::c_int + i)
                                              as
                                              usize].lm_tc[0 as libc::c_int as
                                                               usize] =
                            *v.offset(5 as libc::c_int as isize) -
                                ((*surf).light_s - (*info).dlight_s) as
                                    libc::c_float *
                                    (1.0f32 / tr.block_size as libc::c_float);
                        vbos.decal_dlight[(decalcount * 32 as libc::c_int + i)
                                              as
                                              usize].lm_tc[1 as libc::c_int as
                                                               usize] =
                            *v.offset(6 as libc::c_int as isize) -
                                ((*surf).light_t - (*info).dlight_t) as
                                    libc::c_float *
                                    (1.0f32 / tr.block_size as libc::c_float);
                        i += 1;
                        v = v.offset(7 as libc::c_int as isize)
                    }
                } else {
                    // copy from vbo
                    i = 0 as libc::c_int;
                    while i < numVerts {
                        vbos.decal_dlight[(decalcount * 32 as libc::c_int + i)
                                              as
                                              usize].pos[0 as libc::c_int as
                                                             usize] =
                            (*vbos.decaldata).decalarray[(decalindex *
                                                              8 as libc::c_int
                                                              + i) as
                                                             usize].pos[0 as
                                                                            libc::c_int
                                                                            as
                                                                            usize];
                        vbos.decal_dlight[(decalcount * 32 as libc::c_int + i)
                                              as
                                              usize].pos[1 as libc::c_int as
                                                             usize] =
                            (*vbos.decaldata).decalarray[(decalindex *
                                                              8 as libc::c_int
                                                              + i) as
                                                             usize].pos[1 as
                                                                            libc::c_int
                                                                            as
                                                                            usize];
                        vbos.decal_dlight[(decalcount * 32 as libc::c_int + i)
                                              as
                                              usize].pos[2 as libc::c_int as
                                                             usize] =
                            (*vbos.decaldata).decalarray[(decalindex *
                                                              8 as libc::c_int
                                                              + i) as
                                                             usize].pos[2 as
                                                                            libc::c_int
                                                                            as
                                                                            usize];
                        vbos.decal_dlight[(decalcount * 32 as libc::c_int + i)
                                              as
                                              usize].gl_tc[0 as libc::c_int as
                                                               usize] =
                            (*vbos.decaldata).decalarray[(decalindex *
                                                              8 as libc::c_int
                                                              + i) as
                                                             usize].gl_tc[0 as
                                                                              libc::c_int
                                                                              as
                                                                              usize];
                        vbos.decal_dlight[(decalcount * 32 as libc::c_int + i)
                                              as
                                              usize].gl_tc[1 as libc::c_int as
                                                               usize] =
                            (*vbos.decaldata).decalarray[(decalindex *
                                                              8 as libc::c_int
                                                              + i) as
                                                             usize].gl_tc[1 as
                                                                              libc::c_int
                                                                              as
                                                                              usize];
                        vbos.decal_dlight[(decalcount * 32 as libc::c_int + i)
                                              as
                                              usize].lm_tc[0 as libc::c_int as
                                                               usize] =
                            (*vbos.decaldata).decalarray[(decalindex *
                                                              8 as libc::c_int
                                                              + i) as
                                                             usize].lm_tc[0 as
                                                                              libc::c_int
                                                                              as
                                                                              usize]
                                -
                                ((*surf).light_s - (*info).dlight_s) as
                                    libc::c_float *
                                    (1.0f32 / tr.block_size as libc::c_float);
                        vbos.decal_dlight[(decalcount * 32 as libc::c_int + i)
                                              as
                                              usize].lm_tc[1 as libc::c_int as
                                                               usize] =
                            (*vbos.decaldata).decalarray[(decalindex *
                                                              8 as libc::c_int
                                                              + i) as
                                                             usize].lm_tc[1 as
                                                                              libc::c_int
                                                                              as
                                                                              usize]
                                -
                                ((*surf).light_t - (*info).dlight_t) as
                                    libc::c_float *
                                    (1.0f32 / tr.block_size as libc::c_float);
                        i += 1
                    }
                }
                if vbos.dlight_vbo != 0 {
                    pglBindBufferARB.expect("non-null function pointer")(0x8892
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             GLenum,
                                                                         vbos.decal_dlight_vbo);
                    pglBufferSubDataARB.expect("non-null function pointer")(0x8892
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                GLenum,
                                                                            (::std::mem::size_of::<vbovertex_t>()
                                                                                 as
                                                                                 libc::c_ulong).wrapping_mul(decalcount
                                                                                                                 as
                                                                                                                 libc::c_ulong).wrapping_mul(32
                                                                                                                                                 as
                                                                                                                                                 libc::c_int
                                                                                                                                                 as
                                                                                                                                                 libc::c_ulong)
                                                                                as
                                                                                GLintptrARB,
                                                                            (::std::mem::size_of::<vbovertex_t>()
                                                                                 as
                                                                                 libc::c_ulong).wrapping_mul(numVerts
                                                                                                                 as
                                                                                                                 libc::c_ulong)
                                                                                as
                                                                                GLsizeiptrARB,
                                                                            vbos.decal_dlight.as_mut_ptr().offset((decalcount
                                                                                                                       *
                                                                                                                       32
                                                                                                                           as
                                                                                                                           libc::c_int)
                                                                                                                      as
                                                                                                                      isize)
                                                                                as
                                                                                *const libc::c_void);
                }
                vbos.decal_numverts[decalcount as usize] = numVerts;
                decalcount += 1;
                pdecal = (*pdecal).pnext
            }
            surf = (*(*surf).info).lightmapchain
        }
        if dlightindex != 0 {
            // update block
            LM_UploadDynamicBlock();
            // draw remaining array
            // WebGL need to know array sizes
            if pglDrawRangeElements.is_some() {
                pglDrawRangeElements.expect("non-null function pointer")(0x4
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             GLenum,
                                                                         0 as
                                                                             libc::c_int
                                                                             as
                                                                             GLuint,
                                                                         (*vbo).array_len
                                                                             as
                                                                             GLuint,
                                                                         dlightindex
                                                                             as
                                                                             GLsizei,
                                                                         0x1403
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             GLenum,
                                                                         dlightarray
                                                                             as
                                                                             *const libc::c_void);
            } else {
                pglDrawElements.expect("non-null function pointer")(0x4 as
                                                                        libc::c_int
                                                                        as
                                                                        GLenum,
                                                                    dlightindex
                                                                        as
                                                                        GLsizei,
                                                                    0x1403 as
                                                                        libc::c_int
                                                                        as
                                                                        GLenum,
                                                                    dlightarray
                                                                        as
                                                                        *const libc::c_void);
            }
            R_AdditionalPasses(vbo, dlightindex as libc::c_int,
                               dlightarray as *mut libc::c_void, texture,
                               true_0);
            // draw remaining decals
            if decalcount != 0 {
                let mut decalsurf_0: *mut msurface_t = 0 as *mut msurface_t;
                let mut decali_0: libc::c_int = 0 as libc::c_int;
                pglDepthMask.expect("non-null function pointer")(0 as
                                                                     libc::c_int
                                                                     as
                                                                     GLboolean);
                pglEnable.expect("non-null function pointer")(0xbe2 as
                                                                  libc::c_int
                                                                  as GLenum);
                pglEnable.expect("non-null function pointer")(0x8037 as
                                                                  libc::c_int
                                                                  as GLenum);
                if (*RI.currententity).curstate.rendermode ==
                       kRenderTransAlpha as libc::c_int {
                    pglDisable.expect("non-null function pointer")(0xbc0 as
                                                                       libc::c_int
                                                                       as
                                                                       GLenum);
                }
                pglBindBufferARB.expect("non-null function pointer")(0x8892 as
                                                                         libc::c_int
                                                                         as
                                                                         GLenum,
                                                                     vbos.decal_dlight_vbo);
                R_SetDecalMode(true_0);
                if vbos.decal_dlight_vbo != 0 {
                    pglTexCoordPointer.expect("non-null function pointer")(2
                                                                               as
                                                                               libc::c_int,
                                                                           0x1406
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               GLenum,
                                                                           ::std::mem::size_of::<vbovertex_t>()
                                                                               as
                                                                               libc::c_ulong
                                                                               as
                                                                               GLsizei,
                                                                           20
                                                                               as
                                                                               libc::c_ulong
                                                                               as
                                                                               *mut libc::c_void);
                    GL_SelectTexture(mtst.tmu_gl);
                    pglTexCoordPointer.expect("non-null function pointer")(2
                                                                               as
                                                                               libc::c_int,
                                                                           0x1406
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               GLenum,
                                                                           ::std::mem::size_of::<vbovertex_t>()
                                                                               as
                                                                               libc::c_ulong
                                                                               as
                                                                               GLsizei,
                                                                           12
                                                                               as
                                                                               libc::c_ulong
                                                                               as
                                                                               *mut libc::c_void);
                    pglVertexPointer.expect("non-null function pointer")(3 as
                                                                             libc::c_int,
                                                                         0x1406
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             GLenum,
                                                                         ::std::mem::size_of::<vbovertex_t>()
                                                                             as
                                                                             libc::c_ulong
                                                                             as
                                                                             GLsizei,
                                                                         0 as
                                                                             *const libc::c_void);
                } else {
                    pglTexCoordPointer.expect("non-null function pointer")(2
                                                                               as
                                                                               libc::c_int,
                                                                           0x1406
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               GLenum,
                                                                           ::std::mem::size_of::<vbovertex_t>()
                                                                               as
                                                                               libc::c_ulong
                                                                               as
                                                                               GLsizei,
                                                                           &mut (*vbos.decal_dlight.as_mut_ptr().offset(0
                                                                                                                            as
                                                                                                                            libc::c_int
                                                                                                                            as
                                                                                                                            isize)).lm_tc
                                                                               as
                                                                               *mut vec2_t
                                                                               as
                                                                               *const libc::c_void);
                    GL_SelectTexture(mtst.tmu_gl);
                    pglTexCoordPointer.expect("non-null function pointer")(2
                                                                               as
                                                                               libc::c_int,
                                                                           0x1406
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               GLenum,
                                                                           ::std::mem::size_of::<vbovertex_t>()
                                                                               as
                                                                               libc::c_ulong
                                                                               as
                                                                               GLsizei,
                                                                           &mut (*vbos.decal_dlight.as_mut_ptr().offset(0
                                                                                                                            as
                                                                                                                            libc::c_int
                                                                                                                            as
                                                                                                                            isize)).gl_tc
                                                                               as
                                                                               *mut vec2_t
                                                                               as
                                                                               *const libc::c_void);
                    pglVertexPointer.expect("non-null function pointer")(3 as
                                                                             libc::c_int,
                                                                         0x1406
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             GLenum,
                                                                         ::std::mem::size_of::<vbovertex_t>()
                                                                             as
                                                                             libc::c_ulong
                                                                             as
                                                                             GLsizei,
                                                                         &mut (*vbos.decal_dlight.as_mut_ptr().offset(0
                                                                                                                          as
                                                                                                                          libc::c_int
                                                                                                                          as
                                                                                                                          isize)).pos
                                                                             as
                                                                             *mut vec3_t
                                                                             as
                                                                             *const libc::c_void);
                }
                decalsurf_0 = newsurf;
                while decali_0 < decalcount && !decalsurf_0.is_null() {
                    let mut pdecal_0: *mut decal_t = 0 as *mut decal_t;
                    pdecal_0 = (*decalsurf_0).pdecals;
                    while !pdecal_0.is_null() {
                        let mut glt_0: *mut gl_texture_t =
                            0 as *mut gl_texture_t;
                        if !((*pdecal_0).texture == 0) {
                            glt_0 =
                                R_GetTexture((*pdecal_0).texture as GLenum);
                            GL_Bind(mtst.tmu_gl,
                                    (*pdecal_0).texture as GLenum);
                            // normal HL decal with alpha-channel
                            if (*glt_0).flags as libc::c_uint &
                                   TF_HAS_ALPHA as libc::c_int as libc::c_uint
                                   != 0 {
                                // draw transparent decals with GL_MODULATE
                                if (*glt_0).fogParams[3 as libc::c_int as
                                                          usize] as
                                       libc::c_int > 230 as libc::c_int {
                                    pglTexEnvi.expect("non-null function pointer")(0x2300
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       GLenum,
                                                                                   0x2200
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       GLenum,
                                                                                   0x2100
                                                                                       as
                                                                                       libc::c_int);
                                } else {
                                    pglTexEnvi.expect("non-null function pointer")(0x2300
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       GLenum,
                                                                                   0x2200
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       GLenum,
                                                                                   0x1e01
                                                                                       as
                                                                                       libc::c_int);
                                }
                                pglBlendFunc.expect("non-null function pointer")(0x302
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     GLenum,
                                                                                 0x303
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     GLenum);
                            } else {
                                // color decal like detail texture. Base color is 127 127 127
                                pglTexEnvi.expect("non-null function pointer")(0x2300
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   GLenum,
                                                                               0x2200
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   GLenum,
                                                                               0x1e01
                                                                                   as
                                                                                   libc::c_int);
                                pglBlendFunc.expect("non-null function pointer")(0x306
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     GLenum,
                                                                                 0x300
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     GLenum);
                            }
                            pglDrawArrays.expect("non-null function pointer")(0x6
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  GLenum,
                                                                              decali_0
                                                                                  *
                                                                                  32
                                                                                      as
                                                                                      libc::c_int,
                                                                              vbos.decal_numverts[decali_0
                                                                                                      as
                                                                                                      usize]);
                            decali_0 += 1
                        }
                        pdecal_0 = (*pdecal_0).pnext
                    }
                    decalsurf_0 = (*(*decalsurf_0).info).lightmapchain
                }
                // reset states
                pglTexEnvi.expect("non-null function pointer")(0x2300 as
                                                                   libc::c_int
                                                                   as GLenum,
                                                               0x2200 as
                                                                   libc::c_int
                                                                   as GLenum,
                                                               0x1e01 as
                                                                   libc::c_int);
                pglDepthMask.expect("non-null function pointer")(0x1 as
                                                                     libc::c_int
                                                                     as
                                                                     GLboolean);
                pglDisable.expect("non-null function pointer")(0xbe2 as
                                                                   libc::c_int
                                                                   as GLenum);
                pglDisable.expect("non-null function pointer")(0x8037 as
                                                                   libc::c_int
                                                                   as GLenum);
                if (*RI.currententity).curstate.rendermode ==
                       kRenderTransAlpha as libc::c_int {
                    pglEnable.expect("non-null function pointer")(0xbc0 as
                                                                      libc::c_int
                                                                      as
                                                                      GLenum);
                }
                pglBindBufferARB.expect("non-null function pointer")(0x8892 as
                                                                         libc::c_int
                                                                         as
                                                                         GLenum,
                                                                     (*vbo).glindex);
                R_SetDecalMode(false_0);
                pglVertexPointer.expect("non-null function pointer")(3 as
                                                                         libc::c_int,
                                                                     0x1406 as
                                                                         libc::c_int
                                                                         as
                                                                         GLenum,
                                                                     ::std::mem::size_of::<vbovertex_t>()
                                                                         as
                                                                         libc::c_ulong
                                                                         as
                                                                         GLsizei,
                                                                     0 as
                                                                         *const libc::c_void);
                R_SetupVBOTexture(texture, 0 as libc::c_int);
                pglTexCoordPointer.expect("non-null function pointer")(2 as
                                                                           libc::c_int,
                                                                       0x1406
                                                                           as
                                                                           libc::c_int
                                                                           as
                                                                           GLenum,
                                                                       ::std::mem::size_of::<vbovertex_t>()
                                                                           as
                                                                           libc::c_ulong
                                                                           as
                                                                           GLsizei,
                                                                       12 as
                                                                           libc::c_ulong
                                                                           as
                                                                           *mut libc::c_void);
            }
        }
        // restore static lightmap
        pglBindBufferARB.expect("non-null function pointer")(0x8892 as
                                                                 libc::c_int
                                                                 as GLenum,
                                                             (*vbo).glindex);
        GL_Bind(mtst.tmu_lm,
                tr.lightmapTextures[lightmap as usize] as GLenum);
        pglTexCoordPointer.expect("non-null function pointer")(2 as
                                                                   libc::c_int,
                                                               0x1406 as
                                                                   libc::c_int
                                                                   as GLenum,
                                                               ::std::mem::size_of::<vbovertex_t>()
                                                                   as
                                                                   libc::c_ulong
                                                                   as GLsizei,
                                                               20 as
                                                                   libc::c_ulong
                                                                   as
                                                                   *mut libc::c_void);
        // prepare to next frame
        (*vbotex).dlightchain = 0 as *mut msurface_t
    }
    // prepare to next frame
    (*vbotex).curindex = 0 as libc::c_int as uint;
}
/*
=====================
R_DrawVBO

Draw generated index arrays
=====================
*/
unsafe extern "C" fn R_DrawVBO(mut drawlightmap: qboolean,
                               mut drawtextures: qboolean) {
    let mut numtextures: libc::c_int =
        (*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                               as
                                                                               libc::c_int)).numtextures;
    let mut numlightmaps: libc::c_int = gl_lms.current_lightmap_texture;
    let mut k: libc::c_int = 0;
    let mut vbo: *mut vboarray_t = vbos.arraylist;
    if if !r_vbo.is_null() && (*r_vbo).value != 0.0f32 {
           true_0 as libc::c_int
       } else { false_0 as libc::c_int } == 0 {
        return
    }
    // bind array
    pglBindBufferARB.expect("non-null function pointer")(0x8892 as libc::c_int
                                                             as GLenum,
                                                         (*vbo).glindex);
    pglEnableClientState.expect("non-null function pointer")(0x8074 as
                                                                 libc::c_int
                                                                 as GLenum);
    pglVertexPointer.expect("non-null function pointer")(3 as libc::c_int,
                                                         0x1406 as libc::c_int
                                                             as GLenum,
                                                         ::std::mem::size_of::<vbovertex_t>()
                                                             as libc::c_ulong
                                                             as GLsizei,
                                                         0 as
                                                             *const libc::c_void);
    // setup multitexture
    if drawtextures as u64 != 0 {
        mtst.tmu_gl = XASH_TEXTURE0 as libc::c_int;
        GL_SelectTexture(mtst.tmu_gl);
        pglEnable.expect("non-null function pointer")(0xde1 as libc::c_int as
                                                          GLenum);
        pglEnableClientState.expect("non-null function pointer")(0x8078 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum);
        pglTexEnvf.expect("non-null function pointer")(0x2300 as libc::c_int
                                                           as GLenum,
                                                       0x2200 as libc::c_int
                                                           as GLenum,
                                                       0x1e01 as libc::c_int
                                                           as GLfloat);
        pglTexCoordPointer.expect("non-null function pointer")(2 as
                                                                   libc::c_int,
                                                               0x1406 as
                                                                   libc::c_int
                                                                   as GLenum,
                                                               ::std::mem::size_of::<vbovertex_t>()
                                                                   as
                                                                   libc::c_ulong
                                                                   as GLsizei,
                                                               12 as
                                                                   libc::c_ulong
                                                                   as
                                                                   *mut libc::c_void);
    }
    if drawlightmap as u64 != 0 {
        // set lightmap texenv
        mtst.tmu_lm = XASH_TEXTURE1 as libc::c_int;
        GL_SelectTexture(mtst.tmu_lm);
        pglEnable.expect("non-null function pointer")(0xde1 as libc::c_int as
                                                          GLenum);
        pglEnableClientState.expect("non-null function pointer")(0x8078 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum);
        R_SetLightmap();
    }
    mtst.skiptexture = (drawtextures as u64 == 0) as libc::c_int as qboolean;
    mtst.tmu_dt =
        if glConfig.max_texture_units > 2 as libc::c_int {
            XASH_TEXTURE2 as libc::c_int
        } else { -(1 as libc::c_int) };
    // setup limits
    if vbos.minlightmap > vbos.minarraysplit_lm {
        vbos.minlightmap = vbos.minarraysplit_lm
    }
    if vbos.maxlightmap < vbos.maxarraysplit_lm {
        vbos.maxlightmap = vbos.maxarraysplit_lm
    }
    if vbos.maxlightmap > numlightmaps { vbos.maxlightmap = numlightmaps }
    if vbos.mintexture > vbos.minarraysplit_tex {
        vbos.mintexture = vbos.minarraysplit_tex
    }
    if vbos.maxtexture < vbos.maxarraysplit_tex {
        vbos.maxtexture = vbos.maxarraysplit_tex
    }
    if vbos.maxtexture > numtextures { vbos.maxtexture = numtextures }
    k = vbos.minlightmap;
    while k < vbos.maxlightmap {
        let mut j: libc::c_int = 0;
        let mut lightmapchain: *mut msurface_t = 0 as *mut msurface_t;
        if drawlightmap as u64 != 0 {
            mtst.lm = tr.lightmapTextures[k as usize];
            GL_Bind(mtst.tmu_lm, mtst.lm as GLenum);
        }
        j = vbos.mintexture;
        while j < vbos.maxtexture {
            let mut vbotex: *mut vbotexture_t =
                &mut *vbos.textures.offset((k * numtextures + j) as isize) as
                    *mut vbotexture_t;
            let mut tex: *mut texture_t = 0 as *mut texture_t;
            if !(*vbotex).vboarray.is_null() {
                // ASSERT( vbotex->vboarray == vbo );
                if !((*vbotex).vboarray != vbo) {
                    if (*vbotex).curindex != 0 ||
                           !(*vbotex).dlightchain.is_null() {
                        // draw textures static lightmap first
                        if drawtextures as u64 != 0 {
                            tex = R_SetupVBOTexture(0 as *mut texture_t, j)
                        }
                        R_DrawLightmappedVBO(vbo, vbotex, tex, k,
                                             (drawlightmap as u64 == 0) as
                                                 libc::c_int as qboolean);
                    }
                    // if we need to switch to next array (only if map has >65536 vertices)
                    while !(*vbotex).next.is_null() {
                        vbotex = (*vbotex).next;
                        vbo = (*vbo).next;
                        // bind new vertex and index arrays
                        pglBindBufferARB.expect("non-null function pointer")(0x8892
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 GLenum,
                                                                             (*vbo).glindex);
                        pglVertexPointer.expect("non-null function pointer")(3
                                                                                 as
                                                                                 libc::c_int,
                                                                             0x1406
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 GLenum,
                                                                             ::std::mem::size_of::<vbovertex_t>()
                                                                                 as
                                                                                 libc::c_ulong
                                                                                 as
                                                                                 GLsizei,
                                                                             0
                                                                                 as
                                                                                 *const libc::c_void);
                        // update texcoord pointers
                        if drawtextures as u64 != 0 {
                            tex = R_SetupVBOTexture(tex, 0 as libc::c_int);
                            pglTexCoordPointer.expect("non-null function pointer")(2
                                                                                       as
                                                                                       libc::c_int,
                                                                                   0x1406
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       GLenum,
                                                                                   ::std::mem::size_of::<vbovertex_t>()
                                                                                       as
                                                                                       libc::c_ulong
                                                                                       as
                                                                                       GLsizei,
                                                                                   12
                                                                                       as
                                                                                       libc::c_ulong
                                                                                       as
                                                                                       *mut libc::c_void);
                        }
                        if drawlightmap as u64 != 0 {
                            GL_Bind(mtst.tmu_lm,
                                    tr.lightmapTextures[k as usize] as
                                        GLenum);
                            pglTexCoordPointer.expect("non-null function pointer")(2
                                                                                       as
                                                                                       libc::c_int,
                                                                                   0x1406
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       GLenum,
                                                                                   ::std::mem::size_of::<vbovertex_t>()
                                                                                       as
                                                                                       libc::c_ulong
                                                                                       as
                                                                                       GLsizei,
                                                                                   20
                                                                                       as
                                                                                       libc::c_ulong
                                                                                       as
                                                                                       *mut libc::c_void);
                        }
                        // draw new array
                        if (*vbotex).curindex != 0 ||
                               !(*vbotex).dlightchain.is_null() {
                            R_DrawLightmappedVBO(vbo, vbotex, tex, k,
                                                 (drawlightmap as u64 == 0) as
                                                     libc::c_int as qboolean);
                        }
                    }
                }
            }
            j += 1
        }
        if drawtextures as libc::c_uint != 0 &&
               drawlightmap as libc::c_uint != 0 &&
               !(*(*vbos.decaldata).lm.offset(k as isize)).is_null() {
            // prepare for decal draw
            pglBindBufferARB.expect("non-null function pointer")(0x8892 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 (*vbos.decaldata).decalvbo);
            pglDepthMask.expect("non-null function pointer")(0 as libc::c_int
                                                                 as
                                                                 GLboolean);
            pglEnable.expect("non-null function pointer")(0xbe2 as libc::c_int
                                                              as GLenum);
            pglEnable.expect("non-null function pointer")(0x8037 as
                                                              libc::c_int as
                                                              GLenum);
            if (*RI.currententity).curstate.rendermode ==
                   kRenderTransAlpha as libc::c_int {
                pglDisable.expect("non-null function pointer")(0xbc0 as
                                                                   libc::c_int
                                                                   as GLenum);
            }
            R_SetDecalMode(true_0);
            // Set pointers to vbodecaldata->decalvbo
            if drawlightmap as u64 != 0 {
                GL_SelectTexture(mtst.tmu_lm);
                pglTexCoordPointer.expect("non-null function pointer")(2 as
                                                                           libc::c_int,
                                                                       0x1406
                                                                           as
                                                                           libc::c_int
                                                                           as
                                                                           GLenum,
                                                                       ::std::mem::size_of::<vbovertex_t>()
                                                                           as
                                                                           libc::c_ulong
                                                                           as
                                                                           GLsizei,
                                                                       20 as
                                                                           libc::c_ulong
                                                                           as
                                                                           *mut libc::c_void);
                GL_SelectTexture(mtst.tmu_gl);
                pglTexCoordPointer.expect("non-null function pointer")(2 as
                                                                           libc::c_int,
                                                                       0x1406
                                                                           as
                                                                           libc::c_int
                                                                           as
                                                                           GLenum,
                                                                       ::std::mem::size_of::<vbovertex_t>()
                                                                           as
                                                                           libc::c_ulong
                                                                           as
                                                                           GLsizei,
                                                                       12 as
                                                                           libc::c_ulong
                                                                           as
                                                                           *mut libc::c_void);
                pglVertexPointer.expect("non-null function pointer")(3 as
                                                                         libc::c_int,
                                                                     0x1406 as
                                                                         libc::c_int
                                                                         as
                                                                         GLenum,
                                                                     ::std::mem::size_of::<vbovertex_t>()
                                                                         as
                                                                         libc::c_ulong
                                                                         as
                                                                         GLsizei,
                                                                     0 as
                                                                         *const libc::c_void);
            }
            // all surfaces having decals and this lightmap
            lightmapchain = *(*vbos.decaldata).lm.offset(k as isize);
            while !lightmapchain.is_null() {
                let mut pdecal: *mut decal_t = 0 as *mut decal_t;
                // all decals of surface
                pdecal = (*lightmapchain).pdecals;
                while !pdecal.is_null() {
                    let mut glt: *mut gl_texture_t = 0 as *mut gl_texture_t;
                    let mut decalindex: libc::c_int =
                        pdecal.wrapping_offset_from(&mut *gDecalPool.as_mut_ptr().offset(0
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             isize)
                                                        as *mut decal_t) as
                            libc::c_long as libc::c_int;
                    if !((*pdecal).texture == 0) {
                        glt = R_GetTexture((*pdecal).texture as GLenum);
                        GL_Bind(mtst.tmu_gl, (*pdecal).texture as GLenum);
                        // normal HL decal with alpha-channel
                        if (*glt).flags as libc::c_uint &
                               TF_HAS_ALPHA as libc::c_int as libc::c_uint !=
                               0 {
                            // draw transparent decals with GL_MODULATE
                            if (*glt).fogParams[3 as libc::c_int as usize] as
                                   libc::c_int > 230 as libc::c_int {
                                pglTexEnvi.expect("non-null function pointer")(0x2300
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   GLenum,
                                                                               0x2200
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   GLenum,
                                                                               0x2100
                                                                                   as
                                                                                   libc::c_int);
                            } else {
                                pglTexEnvi.expect("non-null function pointer")(0x2300
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   GLenum,
                                                                               0x2200
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   GLenum,
                                                                               0x1e01
                                                                                   as
                                                                                   libc::c_int);
                            }
                            pglBlendFunc.expect("non-null function pointer")(0x302
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 GLenum,
                                                                             0x303
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 GLenum);
                        } else {
                            // color decal like detail texture. Base color is 127 127 127
                            pglTexEnvi.expect("non-null function pointer")(0x2300
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               GLenum,
                                                                           0x2200
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               GLenum,
                                                                           0x1e01
                                                                               as
                                                                               libc::c_int);
                            pglBlendFunc.expect("non-null function pointer")(0x306
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 GLenum,
                                                                             0x300
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 GLenum);
                        }
                        if (*vbos.decaldata).decals[decalindex as
                                                        usize].numVerts ==
                               -(1 as libc::c_int) {
                            let mut numVerts: libc::c_int = 0;
                            let mut v: *mut libc::c_float =
                                0 as *mut libc::c_float;
                            v =
                                R_DecalSetupVerts(pdecal, lightmapchain,
                                                  (*pdecal).texture as
                                                      libc::c_int,
                                                  &mut numVerts);
                            // to many verts to keep in sparse array, so build it now
                            pglBindBufferARB.expect("non-null function pointer")(0x8892
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     GLenum,
                                                                                 0
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     GLuint);
                            pglVertexPointer.expect("non-null function pointer")(3
                                                                                     as
                                                                                     libc::c_int,
                                                                                 0x1406
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     GLenum,
                                                                                 7
                                                                                     as
                                                                                     libc::c_int
                                                                                     *
                                                                                     4
                                                                                         as
                                                                                         libc::c_int,
                                                                                 v
                                                                                     as
                                                                                     *const libc::c_void);
                            pglTexCoordPointer.expect("non-null function pointer")(2
                                                                                       as
                                                                                       libc::c_int,
                                                                                   0x1406
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       GLenum,
                                                                                   7
                                                                                       as
                                                                                       libc::c_int
                                                                                       *
                                                                                       4
                                                                                           as
                                                                                           libc::c_int,
                                                                                   v.offset(3
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                isize)
                                                                                       as
                                                                                       *const libc::c_void);
                            if drawlightmap as u64 != 0 {
                                GL_SelectTexture(mtst.tmu_lm);
                                pglTexCoordPointer.expect("non-null function pointer")(2
                                                                                           as
                                                                                           libc::c_int,
                                                                                       0x1406
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           GLenum,
                                                                                       7
                                                                                           as
                                                                                           libc::c_int
                                                                                           *
                                                                                           4
                                                                                               as
                                                                                               libc::c_int,
                                                                                       v.offset(5
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    isize)
                                                                                           as
                                                                                           *const libc::c_void);
                            }
                            pglDrawArrays.expect("non-null function pointer")(0x6
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  GLenum,
                                                                              0
                                                                                  as
                                                                                  libc::c_int,
                                                                              numVerts);
                            pglBindBufferARB.expect("non-null function pointer")(0x8892
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     GLenum,
                                                                                 (*vbos.decaldata).decalvbo);
                            pglTexCoordPointer.expect("non-null function pointer")(2
                                                                                       as
                                                                                       libc::c_int,
                                                                                   0x1406
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       GLenum,
                                                                                   ::std::mem::size_of::<vbovertex_t>()
                                                                                       as
                                                                                       libc::c_ulong
                                                                                       as
                                                                                       GLsizei,
                                                                                   20
                                                                                       as
                                                                                       libc::c_ulong
                                                                                       as
                                                                                       *mut libc::c_void);
                            GL_SelectTexture(mtst.tmu_gl);
                            pglTexCoordPointer.expect("non-null function pointer")(2
                                                                                       as
                                                                                       libc::c_int,
                                                                                   0x1406
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       GLenum,
                                                                                   ::std::mem::size_of::<vbovertex_t>()
                                                                                       as
                                                                                       libc::c_ulong
                                                                                       as
                                                                                       GLsizei,
                                                                                   12
                                                                                       as
                                                                                       libc::c_ulong
                                                                                       as
                                                                                       *mut libc::c_void);
                            pglVertexPointer.expect("non-null function pointer")(3
                                                                                     as
                                                                                     libc::c_int,
                                                                                 0x1406
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     GLenum,
                                                                                 ::std::mem::size_of::<vbovertex_t>()
                                                                                     as
                                                                                     libc::c_ulong
                                                                                     as
                                                                                     GLsizei,
                                                                                 0
                                                                                     as
                                                                                     *const libc::c_void);
                        } else {
                            // just draw VBO
                            pglDrawArrays.expect("non-null function pointer")(0x6
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  GLenum,
                                                                              decalindex
                                                                                  *
                                                                                  8
                                                                                      as
                                                                                      libc::c_int,
                                                                              (*vbos.decaldata).decals[decalindex
                                                                                                           as
                                                                                                           usize].numVerts);
                        }
                    }
                    pdecal = (*pdecal).pnext
                }
                lightmapchain = (*(*lightmapchain).info).lightmapchain
            }
            // prepare for next frame
            let ref mut fresh12 = *(*vbos.decaldata).lm.offset(k as isize);
            *fresh12 = 0 as *mut msurface_t;
            // prepare for next texture
            pglDepthMask.expect("non-null function pointer")(0x1 as
                                                                 libc::c_int
                                                                 as
                                                                 GLboolean);
            pglDisable.expect("non-null function pointer")(0xbe2 as
                                                               libc::c_int as
                                                               GLenum);
            pglDisable.expect("non-null function pointer")(0x8037 as
                                                               libc::c_int as
                                                               GLenum);
            // restore vbo
            pglBindBufferARB.expect("non-null function pointer")(0x8892 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 (*vbo).glindex);
            pglVertexPointer.expect("non-null function pointer")(3 as
                                                                     libc::c_int,
                                                                 0x1406 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 ::std::mem::size_of::<vbovertex_t>()
                                                                     as
                                                                     libc::c_ulong
                                                                     as
                                                                     GLsizei,
                                                                 0 as
                                                                     *const libc::c_void);
            // restore bump if needed
            R_SetDecalMode(false_0);
            // restore texture
            GL_SelectTexture(mtst.tmu_gl);
            pglEnableClientState.expect("non-null function pointer")(0x8078 as
                                                                         libc::c_int
                                                                         as
                                                                         GLenum);
            pglTexCoordPointer.expect("non-null function pointer")(2 as
                                                                       libc::c_int,
                                                                   0x1406 as
                                                                       libc::c_int
                                                                       as
                                                                       GLenum,
                                                                   ::std::mem::size_of::<vbovertex_t>()
                                                                       as
                                                                       libc::c_ulong
                                                                       as
                                                                       GLsizei,
                                                                   12 as
                                                                       libc::c_ulong
                                                                       as
                                                                       *mut libc::c_void);
            // restore lightmap
            GL_SelectTexture(mtst.tmu_lm);
            pglTexCoordPointer.expect("non-null function pointer")(2 as
                                                                       libc::c_int,
                                                                   0x1406 as
                                                                       libc::c_int
                                                                       as
                                                                       GLenum,
                                                                   ::std::mem::size_of::<vbovertex_t>()
                                                                       as
                                                                       libc::c_ulong
                                                                       as
                                                                       GLsizei,
                                                                   20 as
                                                                       libc::c_ulong
                                                                       as
                                                                       *mut libc::c_void);
            if (*RI.currententity).curstate.rendermode ==
                   kRenderTransAlpha as libc::c_int {
                pglEnable.expect("non-null function pointer")(0xbc0 as
                                                                  libc::c_int
                                                                  as GLenum);
            }
        }
        if drawtextures as u64 == 0 || drawlightmap as u64 == 0 {
            let ref mut fresh13 = *(*vbos.decaldata).lm.offset(k as isize);
            *fresh13 = 0 as *mut msurface_t
        }
        k += 1
    }
    // ASSERT( !vbo->next );
    // restore states
    R_DisableDetail();
    if drawlightmap as u64 != 0 {
        // reset states
        GL_SelectTexture(XASH_TEXTURE1 as libc::c_int);
        pglDisableClientState.expect("non-null function pointer")(0x8078 as
                                                                      libc::c_int
                                                                      as
                                                                      GLenum);
        pglDisable.expect("non-null function pointer")(0xde1 as libc::c_int as
                                                           GLenum);
        if drawtextures as u64 != 0 {
            GL_SelectTexture(XASH_TEXTURE0 as libc::c_int);
            pglEnable.expect("non-null function pointer")(0xde1 as libc::c_int
                                                              as GLenum);
        }
    }
    if drawtextures as u64 != 0 {
        pglDisableClientState.expect("non-null function pointer")(0x8078 as
                                                                      libc::c_int
                                                                      as
                                                                      GLenum);
    }
    mtst.details_enabled = false_0;
    vbos.minlightmap = 256 as libc::c_int;
    vbos.maxlightmap = 0 as libc::c_int;
    vbos.mintexture = 2147483647 as libc::c_int;
    vbos.maxtexture = 0 as libc::c_int;
    pglDisableClientState.expect("non-null function pointer")(0x8074 as
                                                                  libc::c_int
                                                                  as GLenum);
    pglBindBufferARB.expect("non-null function pointer")(0x8892 as libc::c_int
                                                             as GLenum,
                                                         0 as libc::c_int as
                                                             GLuint);
}
/*
================
R_CheckLightMap

update surface's lightmap if needed and return true if it is dynamic
================
*/
unsafe extern "C" fn R_CheckLightMap(mut fa: *mut msurface_t) -> qboolean {
    let mut maps: libc::c_int = 0;
    let mut is_dynamic: qboolean = false_0;
    // check for lightmap modification
    maps = 0 as libc::c_int;
    while maps < 4 as libc::c_int &&
              (*fa).styles[maps as usize] as libc::c_int != 255 as libc::c_int
          {
        if tr.lightstylevalue[(*fa).styles[maps as usize] as usize] !=
               (*fa).cached_light[maps as usize] {
            is_dynamic = true_0;
            break ;
        } else { maps += 1 }
    }
    // already up to date
    if is_dynamic as u64 == 0 &&
           ((*fa).dlightframe != tr.framecount || maps == 4 as libc::c_int) {
        return false_0
    }
    // build lightmap
    if ((*fa).styles[maps as usize] as libc::c_int >= 32 as libc::c_int ||
            (*fa).styles[maps as usize] as libc::c_int == 0 as libc::c_int) &&
           (*fa).dlightframe != tr.framecount {
        let mut temp: [byte; 69696] = [0; 69696];
        let mut smax: libc::c_int = 0;
        let mut tmax: libc::c_int = 0;
        let mut sample_size: libc::c_int = 0;
        let mut info: *mut mextrasurf_t = 0 as *mut mextrasurf_t;
        info = (*fa).info;
        sample_size =
            gEngfuncs.Mod_SampleSizeForFace.expect("non-null function pointer")(fa);
        smax =
            (*info).lightextents[0 as libc::c_int as usize] as libc::c_int /
                sample_size + 1 as libc::c_int;
        tmax =
            (*info).lightextents[1 as libc::c_int as usize] as libc::c_int /
                sample_size + 1 as libc::c_int;
        if smax < 132 as libc::c_int && tmax < 132 as libc::c_int {
            R_BuildLightMap(fa, temp.as_mut_ptr(), smax * 4 as libc::c_int,
                            true_0);
        } else {
            smax =
                if smax < 132 as libc::c_int {
                    smax
                } else { 132 as libc::c_int };
            tmax =
                if tmax < 132 as libc::c_int {
                    tmax
                } else { 132 as libc::c_int };
            //Host_MapDesignError( "R_RenderBrushPoly: bad surface extents: %d %d", fa->extents[0], fa->extents[1] );
            memset(temp.as_mut_ptr() as *mut libc::c_void, 255 as libc::c_int,
                   ::std::mem::size_of::<[byte; 69696]>() as libc::c_ulong);
        }
        R_SetCacheState(fa);
        GL_Bind(XASH_TEXTURE0 as libc::c_int,
                tr.lightmapTextures[(*fa).lightmaptexturenum as usize] as
                    GLenum);
        pglTexSubImage2D.expect("non-null function pointer")(0xde1 as
                                                                 libc::c_int
                                                                 as GLenum,
                                                             0 as libc::c_int,
                                                             (*fa).light_s,
                                                             (*fa).light_t,
                                                             smax, tmax,
                                                             0x1908 as
                                                                 libc::c_int
                                                                 as GLenum,
                                                             0x1401 as
                                                                 libc::c_int
                                                                 as GLenum,
                                                             temp.as_mut_ptr()
                                                                 as
                                                                 *const libc::c_void);
    } else {
        // add to dynamic chain
        return true_0
    }
    // updated
    return false_0;
}
unsafe extern "C" fn R_AddSurfToVBO(mut surf: *mut msurface_t,
                                    mut buildlightmap: qboolean) -> qboolean {
    if (if !r_vbo.is_null() && (*r_vbo).value != 0.0f32 {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int }) != 0 &&
           !(*vbos.surfdata.offset(surf.wrapping_offset_from((*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                                                                    as
                                                                                                                                    libc::c_int)).surfaces)
                                       as libc::c_long as
                                       isize)).vbotexture.is_null() {
        // find vbotexture_t assotiated with this surface
        let mut idx: libc::c_int =
            surf.wrapping_offset_from((*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                                             as
                                                                                                             libc::c_int)).surfaces)
                as libc::c_long as libc::c_int;
        let mut vbotex: *mut vbotexture_t =
            (*vbos.surfdata.offset(idx as isize)).vbotexture;
        let mut texturenum: libc::c_int =
            (*vbos.surfdata.offset(idx as isize)).texturenum as libc::c_int;
        if (*surf).polys.is_null() { return true_0 }
        if vbos.maxlightmap < (*surf).lightmaptexturenum + 1 as libc::c_int {
            vbos.maxlightmap = (*surf).lightmaptexturenum + 1 as libc::c_int
        }
        if vbos.minlightmap > (*surf).lightmaptexturenum {
            vbos.minlightmap = (*surf).lightmaptexturenum
        }
        if vbos.maxtexture < texturenum + 1 as libc::c_int {
            vbos.maxtexture = texturenum + 1 as libc::c_int
        }
        if vbos.mintexture > texturenum { vbos.mintexture = texturenum }
        buildlightmap =
            ::std::mem::transmute::<libc::c_uint,
                                    qboolean>(buildlightmap as libc::c_uint &
                                                  ((if !r_fullbright.is_null()
                                                           &&
                                                           (*r_fullbright).value
                                                               != 0.0f32 {
                                                        true_0 as libc::c_int
                                                    } else {
                                                        false_0 as libc::c_int
                                                    }) == 0 &&
                                                       !(*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                                                               as
                                                                                                                               libc::c_int)).lightdata.is_null())
                                                      as libc::c_int as
                                                      libc::c_uint);
        if buildlightmap as libc::c_uint != 0 &&
               R_CheckLightMap(surf) as libc::c_uint != 0 {
            // every vbotex has own lightmap chain (as we sorted if by textures to use multitexture)
            (*(*surf).info).lightmapchain = (*vbotex).dlightchain;
            (*vbotex).dlightchain = surf
        } else {
            let mut indexbase: uint =
                (*vbos.surfdata.offset(idx as isize)).startindex;
            let mut index: uint = 0;
            // GL_TRIANGLE_FAN: 0 1 2 0 2 3 0 3 4 ...
            index = indexbase.wrapping_add(2 as libc::c_int as libc::c_uint);
            while index <
                      indexbase.wrapping_add((*(*surf).polys).numverts as
                                                 libc::c_uint) {
                let fresh14 = (*vbotex).curindex;
                (*vbotex).curindex = (*vbotex).curindex.wrapping_add(1);
                *(*vbotex).indexarray.offset(fresh14 as isize) =
                    indexbase as libc::c_ushort;
                let fresh15 = (*vbotex).curindex;
                (*vbotex).curindex = (*vbotex).curindex.wrapping_add(1);
                *(*vbotex).indexarray.offset(fresh15 as isize) =
                    index.wrapping_sub(1 as libc::c_int as libc::c_uint) as
                        libc::c_ushort;
                let fresh16 = (*vbotex).curindex;
                (*vbotex).curindex = (*vbotex).curindex.wrapping_add(1);
                *(*vbotex).indexarray.offset(fresh16 as isize) =
                    index as libc::c_ushort;
                index = index.wrapping_add(1)
            }
            // if surface has decals, add it to decal lightmapchain
            if !(*surf).pdecals.is_null() {
                (*(*surf).info).lightmapchain =
                    *(*vbos.decaldata).lm.offset((*vbotex).lightmaptexturenum
                                                     as isize);
                let ref mut fresh17 =
                    *(*vbos.decaldata).lm.offset((*vbotex).lightmaptexturenum
                                                     as isize);
                *fresh17 = surf
            }
        }
        return true_0
    }
    return false_0;
}
/*
=============================================================

	WORLD MODEL

=============================================================
*/
/*
================
R_RecursiveWorldNode
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_RecursiveWorldNode(mut node: *mut mnode_t,
                                              mut clipflags: uint) {
    let mut i: libc::c_int = 0; // hit a solid leaf
    let mut clipped: libc::c_int = 0;
    let mut surf: *mut msurface_t = 0 as *mut msurface_t;
    let mut mark: *mut *mut msurface_t = 0 as *mut *mut msurface_t;
    let mut pleaf: *mut mleaf_t = 0 as *mut mleaf_t;
    let mut c: libc::c_int = 0;
    let mut side: libc::c_int = 0;
    let mut dot: libc::c_float = 0.;
    loop  {
        if (*node).contents == -(2 as libc::c_int) { return }
        if (*node).visframe != tr.visframecount { return }
        if clipflags != 0 &&
               (if !r_nocull.is_null() && (*r_nocull).value != 0.0f32 {
                    true_0 as libc::c_int
                } else { false_0 as libc::c_int }) == 0 {
            i = 0 as libc::c_int;
            while i < 6 as libc::c_int {
                let mut p: *const mplane_t =
                    &mut *RI.frustum.planes.as_mut_ptr().offset(i as isize) as
                        *mut mplane_t;
                if !(clipflags & (1 as libc::c_uint) << i == 0) {
                    clipped =
                        BoxOnPlaneSide((*node).minmaxs.as_mut_ptr() as
                                           *const vec_t,
                                       (*node).minmaxs.as_mut_ptr().offset(3
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize)
                                           as *const vec_t, p);
                    if clipped == 2 as libc::c_int { return }
                    if clipped == 1 as libc::c_int {
                        clipflags = clipflags & !((1 as libc::c_uint) << i)
                    }
                }
                i += 1
            }
        }
        // if a leaf node, draw stuff
        if (*node).contents < 0 as libc::c_int {
            pleaf = node as *mut mleaf_t;
            mark = (*pleaf).firstmarksurface;
            c = (*pleaf).nummarksurfaces;
            if c != 0 {
                loop  {
                    (**mark).visframe = tr.framecount;
                    mark = mark.offset(1);
                    c -= 1;
                    if !(c != 0) { break ; }
                }
            }
            // deal with model fragments in this leaf
            if !(*pleaf).efrags.is_null() {
                gEngfuncs.R_StoreEfrags.expect("non-null function pointer")(&mut (*pleaf).efrags,
                                                                            tr.realframecount);
            }
            r_stats.c_world_leafs = r_stats.c_world_leafs.wrapping_add(1);
            return
        }
        // node is just a decision point, so go down the apropriate sides
        // find which side of the node we are on
        dot =
            (if ((*(*node).plane).type_0 as libc::c_int) < 3 as libc::c_int {
                 tr.modelorg[(*(*node).plane).type_0 as usize]
             } else {
                 (tr.modelorg[0 as libc::c_int as usize] *
                      (*(*node).plane).normal[0 as libc::c_int as usize] +
                      tr.modelorg[1 as libc::c_int as usize] *
                          (*(*node).plane).normal[1 as libc::c_int as usize])
                     +
                     tr.modelorg[2 as libc::c_int as usize] *
                         (*(*node).plane).normal[2 as libc::c_int as usize]
             }) - (*(*node).plane).dist;
        side =
            if dot >= 0.0f32 { 0 as libc::c_int } else { 1 as libc::c_int };
        // recurse down the children, front side first
        R_RecursiveWorldNode((*node).children[side as usize], clipflags);
        // draw stuff
        c = (*node).numsurfaces as libc::c_int;
        surf =
            (*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                   as
                                                                                   libc::c_int)).surfaces.offset((*node).firstsurface
                                                                                                                     as
                                                                                                                     libc::c_int
                                                                                                                     as
                                                                                                                     isize);
        while c != 0 {
            if !(R_CullSurface(surf, &mut RI.frustum, clipflags) != 0) {
                if (*surf).flags as libc::c_uint &
                       (1 as libc::c_uint) << 2 as libc::c_int != 0 {
                    // make sky chain to right clip the skybox
                    (*surf).texturechain = skychain;
                    skychain = surf
                } else if R_AddSurfToVBO(surf, true_0) as u64 == 0 {
                    (*surf).texturechain =
                        (*(*(*surf).texinfo).texture).texturechain;
                    (*(*(*surf).texinfo).texture).texturechain = surf
                }
            }
            c -= 1;
            surf = surf.offset(1)
        }
        // recurse down the back side
        node = (*node).children[(side == 0) as libc::c_int as usize]
    };
}
/*
================
R_CullNodeTopView

cull node by user rectangle (simple scissor)
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_CullNodeTopView(mut node: *mut mnode_t)
 -> qboolean {
    let mut delta: vec2_t = [0.; 2];
    let mut size: vec2_t = [0.; 2];
    let mut center: vec3_t = [0.; 3];
    let mut half: vec3_t = [0.; 3];
    // build the node center and half-diagonal
    center[0 as libc::c_int as usize] =
        ((*node).minmaxs[0 as libc::c_int as usize] +
             *(*node).minmaxs.as_mut_ptr().offset(3 as libc::c_int as
                                                      isize).offset(0 as
                                                                        libc::c_int
                                                                        as
                                                                        isize))
            * 0.5f32;
    center[1 as libc::c_int as usize] =
        ((*node).minmaxs[1 as libc::c_int as usize] +
             *(*node).minmaxs.as_mut_ptr().offset(3 as libc::c_int as
                                                      isize).offset(1 as
                                                                        libc::c_int
                                                                        as
                                                                        isize))
            * 0.5f32;
    center[2 as libc::c_int as usize] =
        ((*node).minmaxs[2 as libc::c_int as usize] +
             *(*node).minmaxs.as_mut_ptr().offset(3 as libc::c_int as
                                                      isize).offset(2 as
                                                                        libc::c_int
                                                                        as
                                                                        isize))
            * 0.5f32;
    half[0 as libc::c_int as usize] =
        *(*node).minmaxs.as_mut_ptr().offset(3 as libc::c_int as
                                                 isize).offset(0 as
                                                                   libc::c_int
                                                                   as isize) -
            center[0 as libc::c_int as usize];
    half[1 as libc::c_int as usize] =
        *(*node).minmaxs.as_mut_ptr().offset(3 as libc::c_int as
                                                 isize).offset(1 as
                                                                   libc::c_int
                                                                   as isize) -
            center[1 as libc::c_int as usize];
    half[2 as libc::c_int as usize] =
        *(*node).minmaxs.as_mut_ptr().offset(3 as libc::c_int as
                                                 isize).offset(2 as
                                                                   libc::c_int
                                                                   as isize) -
            center[2 as libc::c_int as usize];
    // cull against the screen frustum or the appropriate area's frustum.
    delta[0 as libc::c_int as usize] =
        center[0 as libc::c_int as usize] -
            world_orthocenter[0 as libc::c_int as usize];
    delta[1 as libc::c_int as usize] =
        center[1 as libc::c_int as usize] -
            world_orthocenter[1 as libc::c_int as usize];
    size[0 as libc::c_int as usize] =
        half[0 as libc::c_int as usize] +
            world_orthohalf[0 as libc::c_int as usize];
    size[1 as libc::c_int as usize] =
        half[1 as libc::c_int as usize] +
            world_orthohalf[1 as libc::c_int as usize];
    return (__tg_fabs(delta[0 as libc::c_int as usize]) >
                size[0 as libc::c_int as usize] ||
                __tg_fabs(delta[1 as libc::c_int as usize]) >
                    size[1 as libc::c_int as usize]) as libc::c_int as
               qboolean;
}
/*
================
R_DrawTopViewLeaf
================
*/
unsafe extern "C" fn R_DrawTopViewLeaf(mut pleaf: *mut mleaf_t,
                                       mut clipflags: uint) {
    let mut mark: *mut *mut msurface_t = 0 as *mut *mut msurface_t;
    let mut surf: *mut msurface_t = 0 as *mut msurface_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    mark = (*pleaf).firstmarksurface;
    while i < (*pleaf).nummarksurfaces {
        surf = *mark;
        // don't process the same surface twice
        if !((*surf).visframe == tr.framecount) {
            (*surf).visframe = tr.framecount;
            if !(R_CullSurface(surf, &mut RI.frustum, clipflags) != 0) {
                if (*surf).flags as libc::c_uint &
                       (1 as libc::c_uint) << 2 as libc::c_int == 0 {
                    (*surf).texturechain =
                        (*(*(*surf).texinfo).texture).texturechain;
                    (*(*(*surf).texinfo).texture).texturechain = surf
                }
            }
        }
        i += 1;
        mark = mark.offset(1)
    }
    // deal with model fragments in this leaf
    if !(*pleaf).efrags.is_null() {
        gEngfuncs.R_StoreEfrags.expect("non-null function pointer")(&mut (*pleaf).efrags,
                                                                    tr.realframecount);
    }
    r_stats.c_world_leafs = r_stats.c_world_leafs.wrapping_add(1);
}
/*
================
R_DrawWorldTopView
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_DrawWorldTopView(mut node: *mut mnode_t,
                                            mut clipflags: uint) {
    let mut i: libc::c_int = 0; // hit a solid leaf
    let mut c: libc::c_int = 0;
    let mut clipped: libc::c_int = 0;
    let mut surf: *mut msurface_t = 0 as *mut msurface_t;
    loop  {
        if (*node).contents == -(2 as libc::c_int) { return }
        if (*node).visframe != tr.visframecount { return }
        if clipflags != 0 && (*r_nocull).value == 0. {
            i = 0 as libc::c_int;
            while i < 6 as libc::c_int {
                let mut p: *const mplane_t =
                    &mut *RI.frustum.planes.as_mut_ptr().offset(i as isize) as
                        *mut mplane_t;
                if !(clipflags & (1 as libc::c_uint) << i == 0) {
                    clipped =
                        BoxOnPlaneSide((*node).minmaxs.as_mut_ptr() as
                                           *const vec_t,
                                       (*node).minmaxs.as_mut_ptr().offset(3
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize)
                                           as *const vec_t, p);
                    if clipped == 2 as libc::c_int { return }
                    if clipped == 1 as libc::c_int {
                        clipflags = clipflags & !((1 as libc::c_uint) << i)
                    }
                }
                i += 1
            }
        }
        // cull against the screen frustum or the appropriate area's frustum.
        if R_CullNodeTopView(node) as u64 != 0 { return }
        // if a leaf node, draw stuff
        if (*node).contents < 0 as libc::c_int {
            R_DrawTopViewLeaf(node as *mut mleaf_t, clipflags);
            return
        }
        // draw stuff
        c = (*node).numsurfaces as libc::c_int;
        surf =
            (*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                   as
                                                                                   libc::c_int)).surfaces.offset((*node).firstsurface
                                                                                                                     as
                                                                                                                     libc::c_int
                                                                                                                     as
                                                                                                                     isize);
        while c != 0 {
            // don't process the same surface twice
            if !((*surf).visframe == tr.framecount) {
                (*surf).visframe = tr.framecount;
                if !(R_CullSurface(surf, &mut RI.frustum, clipflags) != 0) {
                    if (*surf).flags as libc::c_uint &
                           (1 as libc::c_uint) << 2 as libc::c_int == 0 {
                        (*surf).texturechain =
                            (*(*(*surf).texinfo).texture).texturechain;
                        (*(*(*surf).texinfo).texture).texturechain = surf
                    }
                }
            }
            c -= 1;
            surf = surf.offset(1)
        }
        // recurse down both children, we don't care the order...
        R_DrawWorldTopView((*node).children[0 as libc::c_int as usize],
                           clipflags);
        node = (*node).children[1 as libc::c_int as usize];
        if node.is_null() { break ; }
    };
}
/*
=============
R_DrawTriangleOutlines
=============
*/
#[no_mangle]
pub unsafe extern "C" fn R_DrawTriangleOutlines() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut surf: *mut msurface_t = 0 as *mut msurface_t;
    let mut p: *mut glpoly_t = 0 as *mut glpoly_t;
    let mut v: *mut libc::c_float = 0 as *mut libc::c_float;
    if (*gl_wireframe).value == 0. { return }
    pglDisable.expect("non-null function pointer")(0xde1 as libc::c_int as
                                                       GLenum);
    pglDisable.expect("non-null function pointer")(0xb71 as libc::c_int as
                                                       GLenum);
    pglColor4f.expect("non-null function pointer")(1.0f32, 1.0f32, 1.0f32,
                                                   1.0f32);
    pglPolygonMode.expect("non-null function pointer")(0x408 as libc::c_int as
                                                           GLenum,
                                                       0x1b01 as libc::c_int
                                                           as GLenum);
    // render static surfaces first
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        surf = gl_lms.lightmap_surfaces[i as usize];
        while !surf.is_null() {
            p = (*surf).polys;
            while !p.is_null() {
                pglBegin.expect("non-null function pointer")(0x9 as
                                                                 libc::c_int
                                                                 as GLenum);
                v = (*p).verts[0 as libc::c_int as usize].as_mut_ptr();
                j = 0 as libc::c_int;
                while j < (*p).numverts {
                    pglVertex3fv.expect("non-null function pointer")(v);
                    j += 1;
                    v = v.offset(7 as libc::c_int as isize)
                }
                pglEnd.expect("non-null function pointer")();
                p = (*p).chain
            }
            surf = (*(*surf).info).lightmapchain
        }
        i += 1
    }
    // render surfaces with dynamic lightmaps
    surf = gl_lms.dynamic_surfaces;
    while !surf.is_null() {
        p = (*surf).polys;
        while !p.is_null() {
            pglBegin.expect("non-null function pointer")(0x9 as libc::c_int as
                                                             GLenum);
            v = (*p).verts[0 as libc::c_int as usize].as_mut_ptr();
            j = 0 as libc::c_int;
            while j < (*p).numverts {
                pglVertex3fv.expect("non-null function pointer")(v);
                j += 1;
                v = v.offset(7 as libc::c_int as isize)
            }
            pglEnd.expect("non-null function pointer")();
            p = (*p).chain
        }
        surf = (*(*surf).info).lightmapchain
    }
    pglPolygonMode.expect("non-null function pointer")(0x408 as libc::c_int as
                                                           GLenum,
                                                       0x1b02 as libc::c_int
                                                           as GLenum);
    pglEnable.expect("non-null function pointer")(0xb71 as libc::c_int as
                                                      GLenum);
    pglEnable.expect("non-null function pointer")(0xde1 as libc::c_int as
                                                      GLenum);
}
/*
=============
R_DrawWorld
=============
*/
#[no_mangle]
pub unsafe extern "C" fn R_DrawWorld() {
    let mut start: libc::c_double = 0.;
    let mut end: libc::c_double = 0.;
    // paranoia issues: when gl_renderer is "0" we need have something valid for currententity
	// to prevent crashing until HeadShield drawing.
    RI.currententity =
        gEngfuncs.GetEntityByIndex.expect("non-null function pointer")(0 as
                                                                           libc::c_int);
    RI.currentmodel = (*RI.currententity).model;
    if RI.drawWorld as u64 == 0 || RI.onlyClientDraw as libc::c_uint != 0 {
        return
    }
    tr.modelorg[0 as libc::c_int as usize] =
        RI.cullorigin[0 as libc::c_int as usize];
    tr.modelorg[1 as libc::c_int as usize] =
        RI.cullorigin[1 as libc::c_int as usize];
    tr.modelorg[2 as libc::c_int as usize] =
        RI.cullorigin[2 as libc::c_int as usize];
    memset(gl_lms.lightmap_surfaces.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<[*mut msurface_t; 256]>() as libc::c_ulong);
    memset(fullbright_surfaces.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<[*mut mextrasurf_t; 4096]>() as
               libc::c_ulong);
    memset(detail_surfaces.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<[*mut mextrasurf_t; 4096]>() as
               libc::c_ulong);
    gl_lms.dynamic_surfaces = 0 as *mut msurface_t;
    pglDisable.expect("non-null function pointer")(0xbc0 as libc::c_int as
                                                       GLenum);
    pglDisable.expect("non-null function pointer")(0xbe2 as libc::c_int as
                                                       GLenum);
    tr.blend = 1.0f32;
    R_ClearSkyBox();
    start = gEngfuncs.pfnTime.expect("non-null function pointer")();
    if RI.drawOrtho as u64 != 0 {
        R_DrawWorldTopView((*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                                  as
                                                                                                  libc::c_int)).nodes,
                           RI.frustum.clipFlags);
    } else {
        R_RecursiveWorldNode((*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                                    as
                                                                                                    libc::c_int)).nodes,
                             RI.frustum.clipFlags);
    }
    end = gEngfuncs.pfnTime.expect("non-null function pointer")();
    r_stats.t_world_node = end - start;
    start = gEngfuncs.pfnTime.expect("non-null function pointer")();
    R_DrawVBO(((if !r_fullbright.is_null() && (*r_fullbright).value != 0.0f32
                   {
                    true_0 as libc::c_int
                } else { false_0 as libc::c_int }) == 0 &&
                   !(*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                           as
                                                                                           libc::c_int)).lightdata.is_null())
                  as libc::c_int as qboolean, true_0);
    R_DrawTextureChains();
    if Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(PARM_DEV_OVERVIEW
                                                                                                                 as
                                                                                                                 libc::c_int,
                                                                                                             0
                                                                                                                 as
                                                                                                                 libc::c_int)
           == 0 {
        DrawDecalsBatch();
        GL_ResetFogColor();
        R_BlendLightmaps();
        R_RenderFullbrights();
        R_RenderDetails();
        if !skychain.is_null() { R_DrawSkyBox(); }
    }
    end = gEngfuncs.pfnTime.expect("non-null function pointer")();
    r_stats.t_world_draw = end - start;
    tr.num_draw_decals = 0 as libc::c_int;
    skychain = 0 as *mut msurface_t;
    R_DrawTriangleOutlines();
    R_DrawWorldHull();
}
/*
===============
R_MarkLeaves

Mark the leaves and nodes that are in the PVS for the current leaf
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_MarkLeaves() {
    let mut novis: qboolean = false_0;
    let mut force: qboolean = false_0;
    let mut leaf: *mut mleaf_t = 0 as *mut mleaf_t;
    let mut node: *mut mnode_t = 0 as *mut mnode_t;
    let mut test: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    if RI.drawWorld as u64 == 0 { return }
    if (*r_novis).flags & (1 as libc::c_int) << 13 as libc::c_int != 0 ||
           tr.fResetVis as libc::c_uint != 0 {
        // force recalc viewleaf
        (*r_novis).flags =
            (*r_novis).flags & !((1 as libc::c_int) << 13 as libc::c_int);
        tr.fResetVis = false_0;
        RI.viewleaf = 0 as *mut mleaf_t
    }
    test[0 as libc::c_int as usize] = RI.pvsorigin[0 as libc::c_int as usize];
    test[1 as libc::c_int as usize] = RI.pvsorigin[1 as libc::c_int as usize];
    test[2 as libc::c_int as usize] = RI.pvsorigin[2 as libc::c_int as usize];
    if !RI.viewleaf.is_null() {
        // merge two leafs that can be a crossed-line contents
        if (*RI.viewleaf).contents == -(1 as libc::c_int) {
            test[0 as libc::c_int as usize] =
                RI.pvsorigin[0 as libc::c_int as usize];
            test[1 as libc::c_int as usize] =
                RI.pvsorigin[1 as libc::c_int as usize];
            test[2 as libc::c_int as usize] =
                RI.pvsorigin[2 as libc::c_int as usize] - 16.0f32;
            leaf =
                gEngfuncs.Mod_PointInLeaf.expect("non-null function pointer")(test.as_mut_ptr()
                                                                                  as
                                                                                  *const vec_t,
                                                                              (*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                                                                                     as
                                                                                                                                                     libc::c_int)).nodes)
        } else {
            test[0 as libc::c_int as usize] =
                RI.pvsorigin[0 as libc::c_int as usize];
            test[1 as libc::c_int as usize] =
                RI.pvsorigin[1 as libc::c_int as usize];
            test[2 as libc::c_int as usize] =
                RI.pvsorigin[2 as libc::c_int as usize] + 16.0f32;
            leaf =
                gEngfuncs.Mod_PointInLeaf.expect("non-null function pointer")(test.as_mut_ptr()
                                                                                  as
                                                                                  *const vec_t,
                                                                              (*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                                                                                     as
                                                                                                                                                     libc::c_int)).nodes)
        }
        if (*leaf).contents != -(2 as libc::c_int) && RI.viewleaf != leaf {
            force = true_0
        }
    }
    if RI.viewleaf == RI.oldviewleaf && !RI.viewleaf.is_null() &&
           force as u64 == 0 {
        return
    }
    // development aid to let you run around
	// and see exactly where the pvs ends
    if (*r_lockpvs).value != 0. { return }
    RI.oldviewleaf = RI.viewleaf;
    tr.visframecount += 1;
    if (*r_novis).value != 0. || RI.drawOrtho as libc::c_uint != 0 ||
           RI.viewleaf.is_null() ||
           (*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                  as
                                                                                  libc::c_int)).visdata.is_null()
       {
        novis = true_0
    }
    gEngfuncs.R_FatPVS.expect("non-null function pointer")(RI.pvsorigin.as_mut_ptr(),
                                                           2.0f32,
                                                           RI.visbytes.as_mut_ptr(),
                                                           (RI.params as
                                                                libc::c_uint &
                                                                (1 as
                                                                     libc::c_uint)
                                                                    <<
                                                                    1 as
                                                                        libc::c_int)
                                                               as qboolean,
                                                           novis);
    if force as libc::c_uint != 0 && novis as u64 == 0 {
        gEngfuncs.R_FatPVS.expect("non-null function pointer")(test.as_mut_ptr(),
                                                               2.0f32,
                                                               RI.visbytes.as_mut_ptr(),
                                                               true_0, novis);
    }
    i = 0 as libc::c_int;
    while i <
              (*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                     as
                                                                                     libc::c_int)).numleafs
          {
        if if i >= 0 as libc::c_int {
               (RI.visbytes[(i >> 3 as libc::c_int) as usize] as libc::c_int &
                    (1 as libc::c_int) << (i & 7 as libc::c_int)) as byte as
                   libc::c_int
           } else { false_0 as libc::c_int as byte as libc::c_int } != 0 {
            node =
                &mut *(*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                             as
                                                                                             libc::c_int)).leafs.offset((i
                                                                                                                             +
                                                                                                                             1
                                                                                                                                 as
                                                                                                                                 libc::c_int)
                                                                                                                            as
                                                                                                                            isize)
                    as *mut mleaf_t as *mut mnode_t;
            while !((*node).visframe == tr.visframecount) {
                (*node).visframe = tr.visframecount;
                node = (*node).parent;
                if node.is_null() { break ; }
            }
        }
        i += 1
    };
}
/*
========================
GL_CreateSurfaceLightmap
========================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_CreateSurfaceLightmap(mut surf: *mut msurface_t,
                                                  mut loadmodel:
                                                      *mut model_t) {
    let mut smax: libc::c_int = 0;
    let mut tmax: libc::c_int = 0;
    let mut sample_size: libc::c_int = 0;
    let mut info: *mut mextrasurf_t = (*surf).info;
    let mut base: *mut byte = 0 as *mut byte;
    if (*loadmodel).lightdata.is_null() { return }
    if (*surf).flags as libc::c_uint & (1 as libc::c_uint) << 5 as libc::c_int
           != 0 {
        return
    }
    sample_size =
        gEngfuncs.Mod_SampleSizeForFace.expect("non-null function pointer")(surf);
    smax =
        (*info).lightextents[0 as libc::c_int as usize] as libc::c_int /
            sample_size + 1 as libc::c_int;
    tmax =
        (*info).lightextents[1 as libc::c_int as usize] as libc::c_int /
            sample_size + 1 as libc::c_int;
    if LM_AllocBlock(smax, tmax, &mut (*surf).light_s, &mut (*surf).light_t)
           == 0 {
        LM_UploadBlock(false_0);
        LM_InitBlock();
        if LM_AllocBlock(smax, tmax, &mut (*surf).light_s,
                         &mut (*surf).light_t) == 0 {
            gEngfuncs.Host_Error.expect("non-null function pointer")(b"AllocBlock: full\n\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char);
        }
    }
    (*surf).lightmaptexturenum = gl_lms.current_lightmap_texture;
    base = gl_lms.lightmap_buffer.as_mut_ptr();
    base =
        base.offset((((*surf).light_t * tr.block_size + (*surf).light_s) *
                         4 as libc::c_int) as isize);
    R_SetCacheState(surf);
    R_BuildLightMap(surf, base, tr.block_size * 4 as libc::c_int, false_0);
}
/*
==================
GL_RebuildLightmaps

Rebuilds the lightmap texture
when gamma is changed
==================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_RebuildLightmaps() {
    let mut i: libc::c_int = 0; // wait for worldmodel
    let mut j: libc::c_int = 0;
    let mut m: *mut model_t = 0 as *mut model_t;
    if Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(30
                                                                                                                 as
                                                                                                                 libc::c_int,
                                                                                                             0
                                                                                                                 as
                                                                                                                 libc::c_int)
           == 0 {
        return
    }
    (*vid_brightness).flags =
        (*vid_brightness).flags & !((1 as libc::c_int) << 13 as libc::c_int);
    (*vid_gamma).flags =
        (*vid_gamma).flags & !((1 as libc::c_int) << 13 as libc::c_int);
    // release old lightmaps
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        if tr.lightmapTextures[i as usize] == 0 { break ; }
        GL_FreeTexture(tr.lightmapTextures[i as usize] as GLenum);
        i += 1
    }
    memset(tr.lightmapTextures.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<[libc::c_int; 256]>() as libc::c_ulong);
    gl_lms.current_lightmap_texture = 0 as libc::c_int;
    // setup all the lightstyles
    CL_RunLightStyles();
    LM_InitBlock();
    i = 0 as libc::c_int;
    while i <
              Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(PARM_NUMMODELS
                                                                                                                        as
                                                                                                                        libc::c_int,
                                                                                                                    0
                                                                                                                        as
                                                                                                                        libc::c_int)
          {
        m =
            gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(i
                                                                                 +
                                                                                 1
                                                                                     as
                                                                                     libc::c_int);
        if !m.is_null() {
            if !((*m).name[0 as libc::c_int as usize] as libc::c_int ==
                     '*' as i32 ||
                     (*m).type_0 as libc::c_int != mod_brush as libc::c_int) {
                j = 0 as libc::c_int;
                while j < (*m).numsurfaces {
                    GL_CreateSurfaceLightmap((*m).surfaces.offset(j as isize),
                                             m);
                    j += 1
                }
            }
        }
        i += 1
    }
    LM_UploadBlock(false_0);
    if (*gEngfuncs.drawFuncs).GL_BuildLightmaps.is_some() {
        // build lightmaps on the client-side
        (*gEngfuncs.drawFuncs).GL_BuildLightmaps.expect("non-null function pointer")();
    };
}
/*
==================
GL_BuildLightmaps

Builds the lightmap texture
with all the surfaces from all brush models
==================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_BuildLightmaps() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut m: *mut model_t = 0 as *mut model_t;
    // release old lightmaps
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        if tr.lightmapTextures[i as usize] == 0 { break ; }
        GL_FreeTexture(tr.lightmapTextures[i as usize] as GLenum);
        i += 1
    }
    memset(tr.lightmapTextures.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<[libc::c_int; 256]>() as libc::c_ulong);
    memset(&mut RI as *mut ref_instance_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<ref_instance_t>() as libc::c_ulong);
    // update the lightmap blocksize
    if Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(26
                                                                                                                 as
                                                                                                                 libc::c_int,
                                                                                                             0
                                                                                                                 as
                                                                                                                 libc::c_int)
           & (1 as libc::c_int) << 4 as libc::c_int != 0 {
        tr.block_size = 1024 as libc::c_int
    } else { tr.block_size = 128 as libc::c_int } // no dlight cache
    skychain = 0 as *mut msurface_t;
    tr.visframecount = 1 as libc::c_int;
    tr.framecount = tr.visframecount;
    gl_lms.current_lightmap_texture = 0 as libc::c_int;
    tr.modelviewIdentity = false_0;
    tr.realframecount = 1 as libc::c_int;
    nColinElim = 0 as libc::c_int;
    // setup the texture for dlights
    R_InitDlightTexture();
    // setup all the lightstyles
    CL_RunLightStyles();
    LM_InitBlock();
    i = 0 as libc::c_int;
    while i <
              Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(PARM_NUMMODELS
                                                                                                                        as
                                                                                                                        libc::c_int,
                                                                                                                    0
                                                                                                                        as
                                                                                                                        libc::c_int)
          {
        m =
            gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(i
                                                                                 +
                                                                                 1
                                                                                     as
                                                                                     libc::c_int);
        if !m.is_null() {
            if !((*m).name[0 as libc::c_int as usize] as libc::c_int ==
                     '*' as i32 ||
                     (*m).type_0 as libc::c_int != mod_brush as libc::c_int) {
                j = 0 as libc::c_int;
                while j < (*m).numsurfaces {
                    // clearing all decal chains
                    let ref mut fresh18 =
                        (*(*m).surfaces.offset(j as isize)).pdecals;
                    *fresh18 = 0 as *mut decal_t;
                    (*(*m).surfaces.offset(j as isize)).visframe =
                        0 as libc::c_int;
                    GL_CreateSurfaceLightmap((*m).surfaces.offset(j as isize),
                                             m);
                    if !((*(*m).surfaces.offset(j as isize)).flags as
                             libc::c_uint &
                             (1 as libc::c_uint) << 4 as libc::c_int != 0) {
                        GL_BuildPolygonFromSurface(m,
                                                   (*m).surfaces.offset(j as
                                                                            isize));
                    }
                    j += 1
                }
                // clearing visframe
                j = 0 as libc::c_int;
                while j < (*m).numleafs {
                    (*(*m).leafs.offset((j + 1 as libc::c_int) as
                                            isize)).visframe =
                        0 as libc::c_int;
                    j += 1
                }
                j = 0 as libc::c_int;
                while j < (*m).numnodes {
                    (*(*m).nodes.offset(j as isize)).visframe =
                        0 as libc::c_int;
                    j += 1
                }
            }
        }
        i += 1
    }
    LM_UploadBlock(false_0);
    if (*gEngfuncs.drawFuncs).GL_BuildLightmaps.is_some() {
        // build lightmaps on the client-side
        (*gEngfuncs.drawFuncs).GL_BuildLightmaps.expect("non-null function pointer")();
    }
    // now gamma and brightness are valid
    (*vid_brightness).flags =
        (*vid_brightness).flags & !((1 as libc::c_int) << 13 as libc::c_int);
    (*vid_gamma).flags =
        (*vid_gamma).flags & !((1 as libc::c_int) << 13 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn GL_InitRandomTable() {
    let mut tu: libc::c_int = 0;
    let mut tv: libc::c_int = 0;
    // make random predictable
    gEngfuncs.COM_SetRandomSeed.expect("non-null function pointer")(255 as
                                                                        libc::c_int);
    tu = 0 as libc::c_int;
    while tu < 20 as libc::c_int {
        tv = 0 as libc::c_int;
        while tv < 20 as libc::c_int {
            rtable[tu as usize][tv as usize] =
                gEngfuncs.COM_RandomLong.expect("non-null function pointer")(0
                                                                                 as
                                                                                 libc::c_int,
                                                                             0x7fff
                                                                                 as
                                                                                 libc::c_int);
            tv += 1
        }
        tu += 1
    }
    gEngfuncs.COM_SetRandomSeed.expect("non-null function pointer")(0 as
                                                                        libc::c_int);
}
