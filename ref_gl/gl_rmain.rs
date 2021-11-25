#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
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
    fn GL_FrustumInitProj(out: *mut gl_frustum_t, flZNear: libc::c_float,
                          flZFar: libc::c_float, flFovX: libc::c_float,
                          flFovY: libc::c_float);
    #[no_mangle]
    fn GL_FrustumInitOrtho(out: *mut gl_frustum_t, xLeft: libc::c_float,
                           xRight: libc::c_float, yTop: libc::c_float,
                           yBottom: libc::c_float, flZNear: libc::c_float,
                           flZFar: libc::c_float);
    #[no_mangle]
    fn ceil(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn floor(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn atanf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn sinf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn tanf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn AngleVectors(angles: *const vec_t, forward: *mut vec_t,
                    right: *mut vec_t, up: *mut vec_t);
    #[no_mangle]
    fn Matrix4x4_ConcatTransforms(out: *mut [vec_t; 4],
                                  in1: *const [vec_t; 4],
                                  in2: *const [vec_t; 4]);
    #[no_mangle]
    fn Matrix4x4_CreateFromEntity(out: *mut [vec_t; 4], angles: *const vec_t,
                                  origin: *const vec_t, scale: libc::c_float);
    #[no_mangle]
    fn Matrix4x4_Invert_Full(out: *mut [vec_t; 4], in1: *const [vec_t; 4])
     -> qboolean;
    #[no_mangle]
    static mut vec3_origin: vec3_t;
    #[no_mangle]
    static matrix4x4_identity: matrix4x4;
    #[no_mangle]
    static mut pglClear: Option<unsafe extern "C" fn(_: GLbitfield) -> ()>;
    #[no_mangle]
    static mut pglClearColor:
           Option<unsafe extern "C" fn(_: GLclampf, _: GLclampf, _: GLclampf,
                                       _: GLclampf) -> ()>;
    #[no_mangle]
    static mut pglIsEnabled:
           Option<unsafe extern "C" fn(_: GLenum) -> GLboolean>;
    #[no_mangle]
    static mut pglClipPlane:
           Option<unsafe extern "C" fn(_: GLenum, _: *const GLdouble) -> ()>;
    #[no_mangle]
    static mut pglColor4f:
           Option<unsafe extern "C" fn(_: GLfloat, _: GLfloat, _: GLfloat,
                                       _: GLfloat) -> ()>;
    #[no_mangle]
    static mut pglDepthFunc: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglDepthRange:
           Option<unsafe extern "C" fn(_: GLclampd, _: GLclampd) -> ()>;
    #[no_mangle]
    static mut pglDisable: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglDrawBuffer: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglEnable: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglFinish: Option<unsafe extern "C" fn() -> ()>;
    #[no_mangle]
    static mut pglFogf:
           Option<unsafe extern "C" fn(_: GLenum, _: GLfloat) -> ()>;
    #[no_mangle]
    static mut pglFogfv:
           Option<unsafe extern "C" fn(_: GLenum, _: *const GLfloat) -> ()>;
    #[no_mangle]
    static mut pglFogi:
           Option<unsafe extern "C" fn(_: GLenum, _: GLint) -> ()>;
    #[no_mangle]
    static mut pglGetFloatv:
           Option<unsafe extern "C" fn(_: GLenum, _: *mut GLfloat) -> ()>;
    #[no_mangle]
    static mut pglHint:
           Option<unsafe extern "C" fn(_: GLenum, _: GLenum) -> ()>;
    #[no_mangle]
    static mut pglMatrixMode: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglTexEnvi:
           Option<unsafe extern "C" fn(_: GLenum, _: GLenum, _: GLint) -> ()>;
    #[no_mangle]
    static mut pglViewport:
           Option<unsafe extern "C" fn(_: GLint, _: GLint, _: GLsizei,
                                       _: GLsizei) -> ()>;
    #[no_mangle]
    static mut r_stats: ref_speeds_t;
    #[no_mangle]
    static mut tr: gl_globals_t;
    #[no_mangle]
    fn GL_LoadMatrix(source: *const [vec_t; 4]);
    #[no_mangle]
    fn GL_Cull(cull: GLenum);
    #[no_mangle]
    fn R_Set2DMode(enable: qboolean);
    #[no_mangle]
    fn R_SetTextureParameters();
    #[no_mangle]
    fn R_GetTexture(texnum: GLenum) -> *mut gl_texture_t;
    #[no_mangle]
    fn R_PushDlights();
    #[no_mangle]
    static mut gEngfuncs: ref_api_t;
    #[no_mangle]
    fn R_DrawWaterSurfaces();
    #[no_mangle]
    fn GL_CheckForErrors_(filename: *const libc::c_char,
                          fileline: libc::c_int);
    #[no_mangle]
    fn R_DrawViewModel();
    #[no_mangle]
    static mut glState: glstate_t;
    #[no_mangle]
    fn R_DrawSpriteModel(e: *mut cl_entity_t);
    #[no_mangle]
    fn R_DrawStudioModel(e: *mut cl_entity_t);
    #[no_mangle]
    fn R_DrawAliasModel(e: *mut cl_entity_t);
    #[no_mangle]
    fn R_DrawBrushModel(e: *mut cl_entity_t);
    #[no_mangle]
    static mut gpGlobals: *mut ref_globals_t;
    #[no_mangle]
    fn R_DrawAlphaTextureChains();
    #[no_mangle]
    fn R_DrawWorld();
    #[no_mangle]
    fn R_MarkLeaves();
    #[no_mangle]
    fn Matrix4x4_Concat(out: *mut [vec_t; 4], in1: *const [vec_t; 4],
                        in2: *const [vec_t; 4]);
    #[no_mangle]
    fn Matrix4x4_CreateProjection(out: *mut [vec_t; 4], xMax: libc::c_float,
                                  xMin: libc::c_float, yMax: libc::c_float,
                                  yMin: libc::c_float, zNear: libc::c_float,
                                  zFar: libc::c_float);
    #[no_mangle]
    fn Matrix4x4_CreateOrtho(m: *mut [vec_t; 4], xLeft: libc::c_float,
                             xRight: libc::c_float, yBottom: libc::c_float,
                             yTop: libc::c_float, zNear: libc::c_float,
                             zFar: libc::c_float);
    #[no_mangle]
    fn Matrix4x4_ConcatTranslate(out: *mut [vec_t; 4], x: libc::c_float,
                                 y: libc::c_float, z: libc::c_float);
    #[no_mangle]
    fn Matrix4x4_ConcatRotate(out: *mut [vec_t; 4], angle: libc::c_float,
                              x: libc::c_float, y: libc::c_float,
                              z: libc::c_float);
    #[no_mangle]
    fn Matrix4x4_CreateModelview(out: *mut [vec_t; 4]);
    #[no_mangle]
    fn R_GetEntityRenderMode(ent: *mut cl_entity_t) -> libc::c_int;
    #[no_mangle]
    static mut gl_nosort: *mut cvar_t;
    #[no_mangle]
    static mut r_lockfrustum: *mut cvar_t;
    #[no_mangle]
    fn R_RunViewmodelEvents();
    #[no_mangle]
    fn R_GatherPlayerLight();
    #[no_mangle]
    static mut gl_msaa: *mut cvar_t;
    #[no_mangle]
    static mut glConfig: glconfig_t;
    #[no_mangle]
    static mut gl_finish: *mut cvar_t;
    #[no_mangle]
    static mut r_norefresh: *mut cvar_t;
    #[no_mangle]
    fn GL_RebuildLightmaps();
    #[no_mangle]
    static mut vid_brightness: *mut cvar_t;
    #[no_mangle]
    static mut vid_gamma: *mut cvar_t;
    #[no_mangle]
    static mut gl_texture_lodbias: *mut cvar_t;
    #[no_mangle]
    static mut gl_texture_anisotropy: *mut cvar_t;
    #[no_mangle]
    static mut gl_lightmap_nearest: *mut cvar_t;
    #[no_mangle]
    static mut gl_texture_nearest: *mut cvar_t;
    #[no_mangle]
    static mut gl_clear: *mut cvar_t;
    #[no_mangle]
    static mut r_drawentities: *mut cvar_t;
    #[no_mangle]
    fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t,
             __compar: __compar_fn_t);
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const kRenderFxClampMinScale: C2RustUnnamed_0 = 20;
pub const kRenderFxGlowShell: C2RustUnnamed_0 = 19;
pub const kRenderFxExplode: C2RustUnnamed_0 = 18;
pub const kRenderFxDeadPlayer: C2RustUnnamed_0 = 17;
pub const kRenderFxHologram: C2RustUnnamed_0 = 16;
pub const kRenderFxDistort: C2RustUnnamed_0 = 15;
pub const kRenderFxNoDissipation: C2RustUnnamed_0 = 14;
pub const kRenderFxFlickerFast: C2RustUnnamed_0 = 13;
pub const kRenderFxFlickerSlow: C2RustUnnamed_0 = 12;
pub const kRenderFxStrobeFaster: C2RustUnnamed_0 = 11;
pub const kRenderFxStrobeFast: C2RustUnnamed_0 = 10;
pub const kRenderFxStrobeSlow: C2RustUnnamed_0 = 9;
pub const kRenderFxSolidFast: C2RustUnnamed_0 = 8;
pub const kRenderFxSolidSlow: C2RustUnnamed_0 = 7;
pub const kRenderFxFadeFast: C2RustUnnamed_0 = 6;
pub const kRenderFxFadeSlow: C2RustUnnamed_0 = 5;
pub const kRenderFxPulseFastWide: C2RustUnnamed_0 = 4;
pub const kRenderFxPulseSlowWide: C2RustUnnamed_0 = 3;
pub const kRenderFxPulseFast: C2RustUnnamed_0 = 2;
pub const kRenderFxPulseSlow: C2RustUnnamed_0 = 1;
pub const kRenderFxNone: C2RustUnnamed_0 = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bpc_desc_s {
    pub format: libc::c_int,
    pub name: [libc::c_char; 16],
    pub glFormat: uint,
    pub bpp: libc::c_int,
}
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
pub type C2RustUnnamed_1 = libc::c_int;
pub const PARM_NUMMODELS: C2RustUnnamed_1 = -13;
pub const PARM_NUMENTITIES: C2RustUnnamed_1 = -12;
pub const PARM_LOCAL_GAME: C2RustUnnamed_1 = -11;
pub const PARM_LOCAL_HEALTH: C2RustUnnamed_1 = -10;
pub const PARM_MAX_CLIENTS: C2RustUnnamed_1 = -9;
pub const PARM_WATER_LEVEL: C2RustUnnamed_1 = -8;
pub const PARM_PLAYING_DEMO: C2RustUnnamed_1 = -7;
pub const PARM_CONNSTATE: C2RustUnnamed_1 = -6;
pub const PARM_VIEWENT_INDEX: C2RustUnnamed_1 = -5;
pub const PARM_PLAYER_INDEX: C2RustUnnamed_1 = -4;
pub const PARM_QUAKE_COMPATIBLE: C2RustUnnamed_1 = -3;
pub const PARM_THIRDPERSON: C2RustUnnamed_1 = -2;
pub const PARM_DEV_OVERVIEW: C2RustUnnamed_1 = -1;
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
pub type ref_overview_t = ref_overview_s;
pub type ref_viewpass_t = ref_viewpass_s;
pub type GLenum = uint;
pub type GLboolean = byte;
pub type GLbitfield = uint;
pub type GLint = libc::c_int;
pub type GLuint = uint;
pub type GLsizei = libc::c_int;
pub type GLfloat = libc::c_float;
pub type GLclampf = libc::c_float;
pub type GLdouble = libc::c_double;
pub type GLclampd = libc::c_double;
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
#[inline(always)]
unsafe extern "C" fn __tg_atan(mut __x: libc::c_float) -> libc::c_float {
    return atanf(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_sin(mut __x: libc::c_float) -> libc::c_float {
    return sinf(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_tan(mut __x: libc::c_float) -> libc::c_float {
    return tanf(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_ceil(mut __x: libc::c_double) -> libc::c_double {
    return ceil(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_floor(mut __x: libc::c_double) -> libc::c_double {
    return floor(__x);
}
#[no_mangle]
pub static mut gldepthmin: libc::c_float = 0.;
#[no_mangle]
pub static mut gldepthmax: libc::c_float = 0.;
#[no_mangle]
pub static mut RI: ref_instance_t =
    ref_instance_t{params: 0,
                   drawWorld: false_0,
                   isSkyVisible: false_0,
                   onlyClientDraw: false_0,
                   drawOrtho: false_0,
                   fov_x: 0.,
                   fov_y: 0.,
                   currententity: 0 as *const cl_entity_t as *mut cl_entity_t,
                   currentmodel: 0 as *const model_t as *mut model_t,
                   currentbeam: 0 as *const cl_entity_t as *mut cl_entity_t,
                   viewport: [0; 4],
                   frustum:
                       gl_frustum_t{planes:
                                        [mplane_t{normal: [0.; 3],
                                                  dist: 0.,
                                                  type_0: 0,
                                                  signbits: 0,
                                                  pad: [0; 2],}; 6],
                                    clipFlags: 0,},
                   viewleaf: 0 as *const mleaf_t as *mut mleaf_t,
                   oldviewleaf: 0 as *const mleaf_t as *mut mleaf_t,
                   pvsorigin: [0.; 3],
                   vieworg: [0.; 3],
                   viewangles: [0.; 3],
                   vforward: [0.; 3],
                   vright: [0.; 3],
                   vup: [0.; 3],
                   cullorigin: [0.; 3],
                   cull_vforward: [0.; 3],
                   cull_vright: [0.; 3],
                   cull_vup: [0.; 3],
                   farClip: 0.,
                   fogCustom: false_0,
                   fogEnabled: false_0,
                   fogSkybox: false_0,
                   fogColor: [0.; 4],
                   fogDensity: 0.,
                   fogStart: 0.,
                   fogEnd: 0.,
                   cached_contents: 0,
                   cached_waterlevel: 0,
                   skyMins: [[0.; 6]; 2],
                   skyMaxs: [[0.; 6]; 2],
                   objectMatrix: [[0.; 4]; 4],
                   worldviewMatrix: [[0.; 4]; 4],
                   modelviewMatrix: [[0.; 4]; 4],
                   projectionMatrix: [[0.; 4]; 4],
                   worldviewProjectionMatrix: [[0.; 4]; 4],
                   visbytes: [0; 4096],
                   viewplanedist: 0.,
                   clipPlane:
                       mplane_t{normal: [0.; 3],
                                dist: 0.,
                                type_0: 0,
                                signbits: 0,
                                pad: [0; 2],},};
unsafe extern "C" fn R_RankForRenderMode(mut rendermode: libc::c_int)
 -> libc::c_int {
    match rendermode {
        2 => {
            return 1 as libc::c_int
            // must be last!
        }
        5 => { return 2 as libc::c_int }
        3 => { return 3 as libc::c_int }
        _ => { }
    } // draw third
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn R_AllowFog(mut allowed: qboolean) {
    if allowed as u64 != 0 {
        if glState.isFogEnabled != 0 {
            pglEnable.expect("non-null function pointer")(0xb60 as libc::c_int
                                                              as GLenum);
        }
    } else if glState.isFogEnabled != 0 {
        pglDisable.expect("non-null function pointer")(0xb60 as libc::c_int as
                                                           GLenum);
    };
}
/*
===============
R_OpaqueEntity

Opaque entity can be brush or studio model but sprite
===============
*/
unsafe extern "C" fn R_OpaqueEntity(mut ent: *mut cl_entity_t) -> qboolean {
    if R_GetEntityRenderMode(ent) == kRenderNormal as libc::c_int {
        return true_0
    }
    return false_0;
}
/*
===============
R_TransEntityCompare

Sorting translucent entities by rendermode then by distance
===============
*/
unsafe extern "C" fn R_TransEntityCompare(mut a: *const libc::c_void,
                                          mut b: *const libc::c_void)
 -> libc::c_int {
    let mut ent1: *mut cl_entity_t = 0 as *mut cl_entity_t;
    let mut ent2: *mut cl_entity_t = 0 as *mut cl_entity_t;
    let mut vecLen: vec3_t = [0.; 3];
    let mut org: vec3_t = [0.; 3];
    let mut dist1: libc::c_float = 0.;
    let mut dist2: libc::c_float = 0.;
    let mut rendermode1: libc::c_int = 0;
    let mut rendermode2: libc::c_int = 0;
    ent1 = *(a as *mut *mut cl_entity_t);
    ent2 = *(b as *mut *mut cl_entity_t);
    rendermode1 = R_GetEntityRenderMode(ent1);
    rendermode2 = R_GetEntityRenderMode(ent2);
    // sort by distance
    if (*(*ent1).model).type_0 as libc::c_int != mod_brush as libc::c_int ||
           rendermode1 != kRenderTransAlpha as libc::c_int {
        org[0 as libc::c_int as usize] =
            ((*(*ent1).model).mins[0 as libc::c_int as usize] +
                 (*(*ent1).model).maxs[0 as libc::c_int as usize]) * 0.5f32;
        org[1 as libc::c_int as usize] =
            ((*(*ent1).model).mins[1 as libc::c_int as usize] +
                 (*(*ent1).model).maxs[1 as libc::c_int as usize]) * 0.5f32;
        org[2 as libc::c_int as usize] =
            ((*(*ent1).model).mins[2 as libc::c_int as usize] +
                 (*(*ent1).model).maxs[2 as libc::c_int as usize]) * 0.5f32;
        org[0 as libc::c_int as usize] =
            (*ent1).origin[0 as libc::c_int as usize] +
                org[0 as libc::c_int as usize];
        org[1 as libc::c_int as usize] =
            (*ent1).origin[1 as libc::c_int as usize] +
                org[1 as libc::c_int as usize];
        org[2 as libc::c_int as usize] =
            (*ent1).origin[2 as libc::c_int as usize] +
                org[2 as libc::c_int as usize];
        vecLen[0 as libc::c_int as usize] =
            RI.vieworg[0 as libc::c_int as usize] -
                org[0 as libc::c_int as usize];
        vecLen[1 as libc::c_int as usize] =
            RI.vieworg[1 as libc::c_int as usize] -
                org[1 as libc::c_int as usize];
        vecLen[2 as libc::c_int as usize] =
            RI.vieworg[2 as libc::c_int as usize] -
                org[2 as libc::c_int as usize];
        dist1 =
            vecLen[0 as libc::c_int as usize] *
                vecLen[0 as libc::c_int as usize] +
                vecLen[1 as libc::c_int as usize] *
                    vecLen[1 as libc::c_int as usize] +
                vecLen[2 as libc::c_int as usize] *
                    vecLen[2 as libc::c_int as usize]
    } else { dist1 = 1000000000 as libc::c_int as libc::c_float }
    if (*(*ent2).model).type_0 as libc::c_int != mod_brush as libc::c_int ||
           rendermode2 != kRenderTransAlpha as libc::c_int {
        org[0 as libc::c_int as usize] =
            ((*(*ent2).model).mins[0 as libc::c_int as usize] +
                 (*(*ent2).model).maxs[0 as libc::c_int as usize]) * 0.5f32;
        org[1 as libc::c_int as usize] =
            ((*(*ent2).model).mins[1 as libc::c_int as usize] +
                 (*(*ent2).model).maxs[1 as libc::c_int as usize]) * 0.5f32;
        org[2 as libc::c_int as usize] =
            ((*(*ent2).model).mins[2 as libc::c_int as usize] +
                 (*(*ent2).model).maxs[2 as libc::c_int as usize]) * 0.5f32;
        org[0 as libc::c_int as usize] =
            (*ent2).origin[0 as libc::c_int as usize] +
                org[0 as libc::c_int as usize];
        org[1 as libc::c_int as usize] =
            (*ent2).origin[1 as libc::c_int as usize] +
                org[1 as libc::c_int as usize];
        org[2 as libc::c_int as usize] =
            (*ent2).origin[2 as libc::c_int as usize] +
                org[2 as libc::c_int as usize];
        vecLen[0 as libc::c_int as usize] =
            RI.vieworg[0 as libc::c_int as usize] -
                org[0 as libc::c_int as usize];
        vecLen[1 as libc::c_int as usize] =
            RI.vieworg[1 as libc::c_int as usize] -
                org[1 as libc::c_int as usize];
        vecLen[2 as libc::c_int as usize] =
            RI.vieworg[2 as libc::c_int as usize] -
                org[2 as libc::c_int as usize];
        dist2 =
            vecLen[0 as libc::c_int as usize] *
                vecLen[0 as libc::c_int as usize] +
                vecLen[1 as libc::c_int as usize] *
                    vecLen[1 as libc::c_int as usize] +
                vecLen[2 as libc::c_int as usize] *
                    vecLen[2 as libc::c_int as usize]
    } else { dist2 = 1000000000 as libc::c_int as libc::c_float }
    if dist1 > dist2 { return -(1 as libc::c_int) }
    if dist1 < dist2 { return 1 as libc::c_int }
    // then sort by rendermode
    if R_RankForRenderMode(rendermode1) > R_RankForRenderMode(rendermode2) {
        return 1 as libc::c_int
    }
    if R_RankForRenderMode(rendermode1) < R_RankForRenderMode(rendermode2) {
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
/*
===============
R_WorldToScreen

Convert a given point from world into screen space
Returns true if we behind to screen
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_WorldToScreen(mut point: *const vec_t,
                                         mut screen: *mut vec_t)
 -> libc::c_int {
    let mut worldToScreen: matrix4x4 =
        [[0.; 4]; 4]; // just so we have something valid here
    let mut behind: qboolean = false_0;
    let mut w: libc::c_float = 0.;
    if point.is_null() || screen.is_null() { return true_0 as libc::c_int }
    memcpy(worldToScreen.as_mut_ptr() as *mut libc::c_void,
           RI.worldviewProjectionMatrix.as_mut_ptr() as *const libc::c_void,
           ::std::mem::size_of::<matrix4x4>() as libc::c_ulong);
    *screen.offset(0 as libc::c_int as isize) =
        worldToScreen[0 as libc::c_int as usize][0 as libc::c_int as usize] *
            *point.offset(0 as libc::c_int as isize) +
            worldToScreen[0 as libc::c_int as
                              usize][1 as libc::c_int as usize] *
                *point.offset(1 as libc::c_int as isize) +
            worldToScreen[0 as libc::c_int as
                              usize][2 as libc::c_int as usize] *
                *point.offset(2 as libc::c_int as isize) +
            worldToScreen[0 as libc::c_int as
                              usize][3 as libc::c_int as usize];
    *screen.offset(1 as libc::c_int as isize) =
        worldToScreen[1 as libc::c_int as usize][0 as libc::c_int as usize] *
            *point.offset(0 as libc::c_int as isize) +
            worldToScreen[1 as libc::c_int as
                              usize][1 as libc::c_int as usize] *
                *point.offset(1 as libc::c_int as isize) +
            worldToScreen[1 as libc::c_int as
                              usize][2 as libc::c_int as usize] *
                *point.offset(2 as libc::c_int as isize) +
            worldToScreen[1 as libc::c_int as
                              usize][3 as libc::c_int as usize];
    w =
        worldToScreen[3 as libc::c_int as usize][0 as libc::c_int as usize] *
            *point.offset(0 as libc::c_int as isize) +
            worldToScreen[3 as libc::c_int as
                              usize][1 as libc::c_int as usize] *
                *point.offset(1 as libc::c_int as isize) +
            worldToScreen[3 as libc::c_int as
                              usize][2 as libc::c_int as usize] *
                *point.offset(2 as libc::c_int as isize) +
            worldToScreen[3 as libc::c_int as
                              usize][3 as libc::c_int as usize];
    *screen.offset(2 as libc::c_int as isize) = 0.0f32;
    if w < 0.001f32 {
        let ref mut fresh0 = *screen.offset(0 as libc::c_int as isize);
        *fresh0 *= 100000 as libc::c_int as libc::c_float;
        let ref mut fresh1 = *screen.offset(1 as libc::c_int as isize);
        *fresh1 *= 100000 as libc::c_int as libc::c_float;
        behind = true_0
    } else {
        let mut invw: libc::c_float = 1.0f32 / w;
        let ref mut fresh2 = *screen.offset(0 as libc::c_int as isize);
        *fresh2 *= invw;
        let ref mut fresh3 = *screen.offset(1 as libc::c_int as isize);
        *fresh3 *= invw;
        behind = false_0
    }
    return behind as libc::c_int;
}
/*
===============
R_ScreenToWorld

Convert a given point from screen into world space
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_ScreenToWorld(mut screen: *const vec_t,
                                         mut point: *mut vec_t) {
    let mut screenToWorld: matrix4x4 = [[0.; 4]; 4];
    let mut w: libc::c_float = 0.;
    if point.is_null() || screen.is_null() { return }
    Matrix4x4_Invert_Full(screenToWorld.as_mut_ptr(),
                          RI.worldviewProjectionMatrix.as_mut_ptr() as
                              *const [vec_t; 4]);
    *point.offset(0 as libc::c_int as isize) =
        *screen.offset(0 as libc::c_int as isize) *
            screenToWorld[0 as libc::c_int as
                              usize][0 as libc::c_int as usize] +
            *screen.offset(1 as libc::c_int as isize) *
                screenToWorld[0 as libc::c_int as
                                  usize][1 as libc::c_int as usize] +
            *screen.offset(2 as libc::c_int as isize) *
                screenToWorld[0 as libc::c_int as
                                  usize][2 as libc::c_int as usize] +
            screenToWorld[0 as libc::c_int as
                              usize][3 as libc::c_int as usize];
    *point.offset(1 as libc::c_int as isize) =
        *screen.offset(0 as libc::c_int as isize) *
            screenToWorld[1 as libc::c_int as
                              usize][0 as libc::c_int as usize] +
            *screen.offset(1 as libc::c_int as isize) *
                screenToWorld[1 as libc::c_int as
                                  usize][1 as libc::c_int as usize] +
            *screen.offset(2 as libc::c_int as isize) *
                screenToWorld[1 as libc::c_int as
                                  usize][2 as libc::c_int as usize] +
            screenToWorld[1 as libc::c_int as
                              usize][3 as libc::c_int as usize];
    *point.offset(2 as libc::c_int as isize) =
        *screen.offset(0 as libc::c_int as isize) *
            screenToWorld[2 as libc::c_int as
                              usize][0 as libc::c_int as usize] +
            *screen.offset(1 as libc::c_int as isize) *
                screenToWorld[2 as libc::c_int as
                                  usize][1 as libc::c_int as usize] +
            *screen.offset(2 as libc::c_int as isize) *
                screenToWorld[2 as libc::c_int as
                                  usize][2 as libc::c_int as usize] +
            screenToWorld[2 as libc::c_int as
                              usize][3 as libc::c_int as usize];
    w =
        *screen.offset(0 as libc::c_int as isize) *
            screenToWorld[3 as libc::c_int as
                              usize][0 as libc::c_int as usize] +
            *screen.offset(1 as libc::c_int as isize) *
                screenToWorld[3 as libc::c_int as
                                  usize][1 as libc::c_int as usize] +
            *screen.offset(2 as libc::c_int as isize) *
                screenToWorld[3 as libc::c_int as
                                  usize][2 as libc::c_int as usize] +
            screenToWorld[3 as libc::c_int as
                              usize][3 as libc::c_int as usize];
    if w != 0.0f32 {
        *point.offset(0 as libc::c_int as isize) =
            *point.offset(0 as libc::c_int as isize) * (1.0f32 / w);
        *point.offset(1 as libc::c_int as isize) =
            *point.offset(1 as libc::c_int as isize) * (1.0f32 / w);
        *point.offset(2 as libc::c_int as isize) =
            *point.offset(2 as libc::c_int as isize) * (1.0f32 / w)
    };
}
/*
===============
R_PushScene
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_PushScene() {
    tr.draw_stack_pos += 1;
    if tr.draw_stack_pos >= 2 as libc::c_int {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"draw stack overflow\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char);
    }
    tr.draw_list =
        &mut *tr.draw_stack.as_mut_ptr().offset(tr.draw_stack_pos as isize) as
            *mut draw_list_t;
}
/*
===============
R_PopScene
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_PopScene() {
    tr.draw_stack_pos -= 1;
    if tr.draw_stack_pos < 0 as libc::c_int {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"draw stack underflow\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char);
    }
    tr.draw_list =
        &mut *tr.draw_stack.as_mut_ptr().offset(tr.draw_stack_pos as isize) as
            *mut draw_list_t;
}
/*
===============
R_ClearScene
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_ClearScene() {
    (*tr.draw_list).num_solid_entities = 0 as libc::c_int as uint;
    (*tr.draw_list).num_trans_entities = 0 as libc::c_int as uint;
    (*tr.draw_list).num_beam_entities = 0 as libc::c_int as uint;
    // clear the scene befor start new frame
    if (*gEngfuncs.drawFuncs).R_ClearScene.is_some() {
        (*gEngfuncs.drawFuncs).R_ClearScene.expect("non-null function pointer")();
    };
}
/*
===============
R_AddEntity
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_AddEntity(mut clent: *mut cl_entity_s,
                                     mut type_0: libc::c_int) -> qboolean {
    if (*r_drawentities).value == 0. {
        return false_0
    } // not allow to drawing
    if clent.is_null() || (*clent).model.is_null() {
        return false_0
    } // if set to invisible, skip
    if (*clent).curstate.effects & 128 as libc::c_int != 0 {
        return false_0
    } // done
    if !((*clent).curstate.rendermode == kRenderNormal as libc::c_int) &&
           CL_FxBlend(clent) <= 0 as libc::c_int {
        return true_0
    } // invisible
    match type_0 {
        4 => { r_stats.c_client_ents = r_stats.c_client_ents.wrapping_add(1) }
        2 => {
            r_stats.c_active_tents_count =
                r_stats.c_active_tents_count.wrapping_add(1)
        }
        _ => { }
    }
    if R_OpaqueEntity(clent) as u64 != 0 {
        // opaque
        if (*tr.draw_list).num_solid_entities >=
               ((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint {
            return false_0
        }
        (*tr.draw_list).solid_entities[(*tr.draw_list).num_solid_entities as
                                           usize] = clent;
        (*tr.draw_list).num_solid_entities =
            (*tr.draw_list).num_solid_entities.wrapping_add(1)
    } else {
        // translucent
        if (*tr.draw_list).num_trans_entities >=
               ((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint {
            return false_0
        }
        (*tr.draw_list).trans_entities[(*tr.draw_list).num_trans_entities as
                                           usize] = clent;
        (*tr.draw_list).num_trans_entities =
            (*tr.draw_list).num_trans_entities.wrapping_add(1)
    }
    return true_0;
}
/*
=============
R_Clear
=============
*/
unsafe extern "C" fn R_Clear(mut bitMask: libc::c_int) {
    let mut bits: libc::c_int = 0; // green background (Valve rules)
    if Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(PARM_DEV_OVERVIEW
                                                                                                                 as
                                                                                                                 libc::c_int,
                                                                                                             0
                                                                                                                 as
                                                                                                                 libc::c_int)
           != 0 {
        pglClearColor.expect("non-null function pointer")(0.0f32, 1.0f32,
                                                          0.0f32, 1.0f32);
    } else {
        pglClearColor.expect("non-null function pointer")(0.5f32, 0.5f32,
                                                          0.5f32, 1.0f32);
    }
    bits = 0x100 as libc::c_int;
    if glState.stencilEnabled as u64 != 0 { bits |= 0x400 as libc::c_int }
    bits &= bitMask;
    pglClear.expect("non-null function pointer")(bits as GLbitfield);
    // change ordering for overview
    if RI.drawOrtho as u64 != 0 {
        gldepthmin = 1.0f32;
        gldepthmax = 0.0f32
    } else { gldepthmin = 0.0f32; gldepthmax = 1.0f32 }
    pglDepthFunc.expect("non-null function pointer")(0x203 as libc::c_int as
                                                         GLenum);
    pglDepthRange.expect("non-null function pointer")(gldepthmin as GLclampd,
                                                      gldepthmax as GLclampd);
}
//=============================================================================
/*
===============
R_GetFarClip
===============
*/
unsafe extern "C" fn R_GetFarClip() -> libc::c_float {
    if !gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1 as
                                                                             libc::c_int).is_null()
           && RI.drawWorld as libc::c_uint != 0 {
        return (*gEngfuncs.pfnGetMoveVars.expect("non-null function pointer")()).zmax
                   * 1.73f32
    }
    return 2048.0f32;
}
/*
===============
R_SetupFrustum
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_SetupFrustum() {
    let mut ov: *const ref_overview_t =
        gEngfuncs.GetOverviewParms.expect("non-null function pointer")();
    if RI.params as libc::c_uint & (1 as libc::c_uint) << 0 as libc::c_int ==
           0 as libc::c_int as libc::c_uint &&
           Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(PARM_WATER_LEVEL
                                                                                                                     as
                                                                                                                     libc::c_int,
                                                                                                                 0
                                                                                                                     as
                                                                                                                     libc::c_int)
               >= 3 as libc::c_int {
        RI.fov_x =
            __tg_atan(__tg_tan(RI.fov_x *
                                   (3.14159265358979323846f64 as libc::c_float
                                        / 180.0f32) /
                                   2 as libc::c_int as libc::c_float) *
                          (0.97f32 +
                               __tg_sin((*gpGlobals).time * 1.5f32) *
                                   0.03f32)) *
                2 as libc::c_int as libc::c_float /
                (3.14159265358979323846f64 as libc::c_float / 180.0f32);
        RI.fov_y =
            __tg_atan(__tg_tan(RI.fov_y *
                                   (3.14159265358979323846f64 as libc::c_float
                                        / 180.0f32) /
                                   2 as libc::c_int as libc::c_float) *
                          (1.03f32 -
                               __tg_sin((*gpGlobals).time * 1.5f32) *
                                   0.03f32)) *
                2 as libc::c_int as libc::c_float /
                (3.14159265358979323846f64 as libc::c_float / 180.0f32)
    }
    // build the transformation matrix for the given view angles
    AngleVectors(RI.viewangles.as_mut_ptr() as *const vec_t,
                 RI.vforward.as_mut_ptr(), RI.vright.as_mut_ptr(),
                 RI.vup.as_mut_ptr());
    if (*r_lockfrustum).value == 0. {
        RI.cullorigin[0 as libc::c_int as usize] =
            RI.vieworg[0 as libc::c_int as usize];
        RI.cullorigin[1 as libc::c_int as usize] =
            RI.vieworg[1 as libc::c_int as usize];
        RI.cullorigin[2 as libc::c_int as usize] =
            RI.vieworg[2 as libc::c_int as usize];
        RI.cull_vforward[0 as libc::c_int as usize] =
            RI.vforward[0 as libc::c_int as usize];
        RI.cull_vforward[1 as libc::c_int as usize] =
            RI.vforward[1 as libc::c_int as usize];
        RI.cull_vforward[2 as libc::c_int as usize] =
            RI.vforward[2 as libc::c_int as usize];
        RI.cull_vright[0 as libc::c_int as usize] =
            RI.vright[0 as libc::c_int as usize];
        RI.cull_vright[1 as libc::c_int as usize] =
            RI.vright[1 as libc::c_int as usize];
        RI.cull_vright[2 as libc::c_int as usize] =
            RI.vright[2 as libc::c_int as usize];
        RI.cull_vup[0 as libc::c_int as usize] =
            RI.vup[0 as libc::c_int as usize];
        RI.cull_vup[1 as libc::c_int as usize] =
            RI.vup[1 as libc::c_int as usize];
        RI.cull_vup[2 as libc::c_int as usize] =
            RI.vup[2 as libc::c_int as usize]
    }
    if RI.drawOrtho as u64 != 0 {
        GL_FrustumInitOrtho(&mut RI.frustum, (*ov).xLeft, (*ov).xRight,
                            (*ov).yTop, (*ov).yBottom, (*ov).zNear,
                            (*ov).zFar);
    } else {
        GL_FrustumInitProj(&mut RI.frustum, 0.0f32, R_GetFarClip(), RI.fov_x,
                           RI.fov_y);
    };
    // NOTE: we ignore nearplane here (mirrors only)
}
/*
=============
R_SetupProjectionMatrix
=============
*/
unsafe extern "C" fn R_SetupProjectionMatrix(mut m: *mut [vec_t; 4]) {
    let mut xMin: GLfloat = 0.;
    let mut xMax: GLfloat = 0.;
    let mut yMin: GLfloat = 0.;
    let mut yMax: GLfloat = 0.;
    let mut zNear: GLfloat = 0.;
    let mut zFar: GLfloat = 0.;
    if RI.drawOrtho as u64 != 0 {
        let mut ov: *const ref_overview_t =
            gEngfuncs.GetOverviewParms.expect("non-null function pointer")();
        Matrix4x4_CreateOrtho(m, (*ov).xLeft, (*ov).xRight, (*ov).yTop,
                              (*ov).yBottom, (*ov).zNear, (*ov).zFar);
        return
    }
    RI.farClip = R_GetFarClip();
    zNear = 4.0f32;
    zFar = if 256.0f32 > RI.farClip { 256.0f32 } else { RI.farClip };
    yMax =
        zNear *
            __tg_tan(RI.fov_y * 3.14159265358979323846f64 as libc::c_float /
                         360.0f32);
    yMin = -yMax;
    xMax =
        zNear *
            __tg_tan(RI.fov_x * 3.14159265358979323846f64 as libc::c_float /
                         360.0f32);
    xMin = -xMax;
    Matrix4x4_CreateProjection(m, xMax, xMin, yMax, yMin, zNear, zFar);
}
/*
=============
R_SetupModelviewMatrix
=============
*/
unsafe extern "C" fn R_SetupModelviewMatrix(mut m: *mut [vec_t; 4]) {
    Matrix4x4_CreateModelview(m);
    Matrix4x4_ConcatRotate(m, -RI.viewangles[2 as libc::c_int as usize],
                           1 as libc::c_int as libc::c_float,
                           0 as libc::c_int as libc::c_float,
                           0 as libc::c_int as libc::c_float);
    Matrix4x4_ConcatRotate(m, -RI.viewangles[0 as libc::c_int as usize],
                           0 as libc::c_int as libc::c_float,
                           1 as libc::c_int as libc::c_float,
                           0 as libc::c_int as libc::c_float);
    Matrix4x4_ConcatRotate(m, -RI.viewangles[1 as libc::c_int as usize],
                           0 as libc::c_int as libc::c_float,
                           0 as libc::c_int as libc::c_float,
                           1 as libc::c_int as libc::c_float);
    Matrix4x4_ConcatTranslate(m, -RI.vieworg[0 as libc::c_int as usize],
                              -RI.vieworg[1 as libc::c_int as usize],
                              -RI.vieworg[2 as libc::c_int as usize]);
}
/*
=============
R_LoadIdentity
=============
*/
#[no_mangle]
pub unsafe extern "C" fn R_LoadIdentity() {
    if tr.modelviewIdentity as u64 != 0 { return }
    memcpy(RI.objectMatrix.as_mut_ptr() as *mut libc::c_void,
           matrix4x4_identity.as_ptr() as *const libc::c_void,
           ::std::mem::size_of::<matrix4x4>() as libc::c_ulong);
    memcpy(RI.modelviewMatrix.as_mut_ptr() as *mut libc::c_void,
           RI.worldviewMatrix.as_mut_ptr() as *const libc::c_void,
           ::std::mem::size_of::<matrix4x4>() as libc::c_ulong);
    pglMatrixMode.expect("non-null function pointer")(0x1700 as libc::c_int as
                                                          GLenum);
    GL_LoadMatrix(RI.modelviewMatrix.as_mut_ptr() as *const [vec_t; 4]);
    tr.modelviewIdentity = true_0;
}
/*
=============
R_RotateForEntity
=============
*/
#[no_mangle]
pub unsafe extern "C" fn R_RotateForEntity(mut e: *mut cl_entity_t) {
    let mut scale: libc::c_float = 1.0f32;
    if e ==
           gEngfuncs.GetEntityByIndex.expect("non-null function pointer")(0 as
                                                                              libc::c_int)
       {
        R_LoadIdentity();
        return
    }
    if (*(*e).model).type_0 as libc::c_int != mod_brush as libc::c_int &&
           (*e).curstate.scale > 0.0f32 {
        scale = (*e).curstate.scale
    }
    Matrix4x4_CreateFromEntity(RI.objectMatrix.as_mut_ptr(),
                               (*e).angles.as_mut_ptr() as *const vec_t,
                               (*e).origin.as_mut_ptr() as *const vec_t,
                               scale);
    Matrix4x4_ConcatTransforms(RI.modelviewMatrix.as_mut_ptr(),
                               RI.worldviewMatrix.as_mut_ptr() as
                                   *const [vec_t; 4],
                               RI.objectMatrix.as_mut_ptr() as
                                   *const [vec_t; 4]);
    pglMatrixMode.expect("non-null function pointer")(0x1700 as libc::c_int as
                                                          GLenum);
    GL_LoadMatrix(RI.modelviewMatrix.as_mut_ptr() as *const [vec_t; 4]);
    tr.modelviewIdentity = false_0;
}
/*
=============
R_TranslateForEntity
=============
*/
#[no_mangle]
pub unsafe extern "C" fn R_TranslateForEntity(mut e: *mut cl_entity_t) {
    let mut scale: libc::c_float = 1.0f32;
    if e ==
           gEngfuncs.GetEntityByIndex.expect("non-null function pointer")(0 as
                                                                              libc::c_int)
       {
        R_LoadIdentity();
        return
    }
    if (*(*e).model).type_0 as libc::c_int != mod_brush as libc::c_int &&
           (*e).curstate.scale > 0.0f32 {
        scale = (*e).curstate.scale
    }
    Matrix4x4_CreateFromEntity(RI.objectMatrix.as_mut_ptr(),
                               vec3_origin.as_mut_ptr() as *const vec_t,
                               (*e).origin.as_mut_ptr() as *const vec_t,
                               scale);
    Matrix4x4_ConcatTransforms(RI.modelviewMatrix.as_mut_ptr(),
                               RI.worldviewMatrix.as_mut_ptr() as
                                   *const [vec_t; 4],
                               RI.objectMatrix.as_mut_ptr() as
                                   *const [vec_t; 4]);
    pglMatrixMode.expect("non-null function pointer")(0x1700 as libc::c_int as
                                                          GLenum);
    GL_LoadMatrix(RI.modelviewMatrix.as_mut_ptr() as *const [vec_t; 4]);
    tr.modelviewIdentity = false_0;
}
/*
===============
R_FindViewLeaf
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_FindViewLeaf() {
    RI.oldviewleaf = RI.viewleaf;
    RI.viewleaf =
        gEngfuncs.Mod_PointInLeaf.expect("non-null function pointer")(RI.pvsorigin.as_mut_ptr()
                                                                          as
                                                                          *const vec_t,
                                                                      (*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                                                                             as
                                                                                                                                             libc::c_int)).nodes);
}
/*
===============
R_SetupFrame
===============
*/
unsafe extern "C" fn R_SetupFrame() {
    // setup viewplane dist
    RI.viewplanedist =
        RI.vieworg[0 as libc::c_int as usize] *
            RI.vforward[0 as libc::c_int as usize] +
            RI.vieworg[1 as libc::c_int as usize] *
                RI.vforward[1 as libc::c_int as usize] +
            RI.vieworg[2 as libc::c_int as usize] *
                RI.vforward[2 as libc::c_int as usize];
    // NOTE: this request is the fps-killer on some NVidia drivers
    glState.isFogEnabled =
        pglIsEnabled.expect("non-null function pointer")(0xb60 as libc::c_int
                                                             as GLenum) as
            GLint;
    if (*gl_nosort).value == 0. {
        // sort translucents entities by rendermode and distance
        qsort((*tr.draw_list).trans_entities.as_mut_ptr() as
                  *mut libc::c_void,
              (*tr.draw_list).num_trans_entities as size_t,
              ::std::mem::size_of::<*mut cl_entity_t>() as libc::c_ulong,
              Some(R_TransEntityCompare as
                       unsafe extern "C" fn(_: *const libc::c_void,
                                            _: *const libc::c_void)
                           -> libc::c_int));
    }
    // current viewleaf
    if RI.drawWorld as u64 != 0 {
        RI.isSkyVisible = false_0; // unknown at this moment
        R_FindViewLeaf();
    };
}
/*
=============
R_SetupGL
=============
*/
#[no_mangle]
pub unsafe extern "C" fn R_SetupGL(mut set_gl_state: qboolean) {
    R_SetupModelviewMatrix(RI.worldviewMatrix.as_mut_ptr());
    R_SetupProjectionMatrix(RI.projectionMatrix.as_mut_ptr());
    Matrix4x4_Concat(RI.worldviewProjectionMatrix.as_mut_ptr(),
                     RI.projectionMatrix.as_mut_ptr() as *const [vec_t; 4],
                     RI.worldviewMatrix.as_mut_ptr() as *const [vec_t; 4]);
    if set_gl_state as u64 == 0 { return }
    if RI.params as libc::c_uint & (1 as libc::c_uint) << 0 as libc::c_int ==
           0 as libc::c_int as libc::c_uint {
        let mut x: libc::c_int = 0;
        let mut x2: libc::c_int = 0;
        let mut y: libc::c_int = 0;
        let mut y2: libc::c_int = 0;
        // set up viewport (main, playersetup)
        x =
            __tg_floor((RI.viewport[0 as libc::c_int as usize] *
                            (*gpGlobals).width / (*gpGlobals).width) as
                           libc::c_double) as libc::c_int;
        x2 =
            __tg_ceil(((RI.viewport[0 as libc::c_int as usize] +
                            RI.viewport[2 as libc::c_int as usize]) *
                           (*gpGlobals).width / (*gpGlobals).width) as
                          libc::c_double) as libc::c_int;
        y =
            __tg_floor(((*gpGlobals).height -
                            RI.viewport[1 as libc::c_int as usize] *
                                (*gpGlobals).height / (*gpGlobals).height) as
                           libc::c_double) as libc::c_int;
        y2 =
            __tg_ceil(((*gpGlobals).height -
                           (RI.viewport[1 as libc::c_int as usize] +
                                RI.viewport[3 as libc::c_int as usize]) *
                               (*gpGlobals).height / (*gpGlobals).height) as
                          libc::c_double) as libc::c_int;
        pglViewport.expect("non-null function pointer")(x, y2, x2 - x,
                                                        y - y2);
    } else {
        // envpass, mirrorpass
        pglViewport.expect("non-null function pointer")(RI.viewport[0 as
                                                                        libc::c_int
                                                                        as
                                                                        usize],
                                                        RI.viewport[1 as
                                                                        libc::c_int
                                                                        as
                                                                        usize],
                                                        RI.viewport[2 as
                                                                        libc::c_int
                                                                        as
                                                                        usize],
                                                        RI.viewport[3 as
                                                                        libc::c_int
                                                                        as
                                                                        usize]);
    }
    pglMatrixMode.expect("non-null function pointer")(0x1701 as libc::c_int as
                                                          GLenum);
    GL_LoadMatrix(RI.projectionMatrix.as_mut_ptr() as *const [vec_t; 4]);
    pglMatrixMode.expect("non-null function pointer")(0x1700 as libc::c_int as
                                                          GLenum);
    GL_LoadMatrix(RI.worldviewMatrix.as_mut_ptr() as *const [vec_t; 4]);
    if RI.params as libc::c_uint & (1 as libc::c_uint) << 2 as libc::c_int !=
           0 {
        let mut clip: [GLdouble; 4] = [0.; 4];
        let mut p: *mut mplane_t = &mut RI.clipPlane;
        clip[0 as libc::c_int as usize] =
            (*p).normal[0 as libc::c_int as usize] as GLdouble;
        clip[1 as libc::c_int as usize] =
            (*p).normal[1 as libc::c_int as usize] as GLdouble;
        clip[2 as libc::c_int as usize] =
            (*p).normal[2 as libc::c_int as usize] as GLdouble;
        clip[3 as libc::c_int as usize] = -(*p).dist as GLdouble;
        pglClipPlane.expect("non-null function pointer")(0x3000 as libc::c_int
                                                             as GLenum,
                                                         clip.as_mut_ptr());
        pglEnable.expect("non-null function pointer")(0x3000 as libc::c_int as
                                                          GLenum);
    }
    GL_Cull(0x404 as libc::c_int as GLenum);
    pglDisable.expect("non-null function pointer")(0xbe2 as libc::c_int as
                                                       GLenum);
    pglDisable.expect("non-null function pointer")(0xbc0 as libc::c_int as
                                                       GLenum);
    pglColor4f.expect("non-null function pointer")(1.0f32, 1.0f32, 1.0f32,
                                                   1.0f32);
}
/*
=============
R_EndGL
=============
*/
unsafe extern "C" fn R_EndGL() {
    if RI.params as libc::c_uint & (1 as libc::c_uint) << 2 as libc::c_int !=
           0 {
        pglDisable.expect("non-null function pointer")(0x3000 as libc::c_int
                                                           as GLenum);
    };
}
/*
=============
R_RecursiveFindWaterTexture

using to find source waterleaf with
watertexture to grab fog values from it
=============
*/
unsafe extern "C" fn R_RecursiveFindWaterTexture(mut node: *const mnode_t,
                                                 mut ignore: *const mnode_t,
                                                 mut down: qboolean)
 -> *mut gl_texture_t {
    let mut tex: *mut gl_texture_t = 0 as *mut gl_texture_t;
    // assure the initial node is not null
	// we could check it here, but we would rather check it
	// outside the call to get rid of one additional recursion level
    if node.is_null() {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 b"../ref_gl/gl_rmain.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 599 as
                                                                     libc::c_int);
    }
    // ignore solid nodes
    if (*node).contents == -(2 as libc::c_int) {
        return 0 as *mut gl_texture_t
    }
    if (*node).contents < 0 as libc::c_int {
        let mut pleaf: *mut mleaf_t = 0 as *mut mleaf_t;
        let mut mark: *mut *mut msurface_t = 0 as *mut *mut msurface_t;
        let mut i: libc::c_int = 0;
        let mut c: libc::c_int = 0;
        // ignore non-liquid leaves
        if (*node).contents != -(3 as libc::c_int) &&
               (*node).contents != -(5 as libc::c_int) &&
               (*node).contents != -(4 as libc::c_int) {
            return 0 as *mut gl_texture_t
        }
        // find texture
        pleaf = node as *mut mleaf_t;
        mark = (*pleaf).firstmarksurface;
        c = (*pleaf).nummarksurfaces;
        i = 0 as libc::c_int;
        while i < c {
            if (**mark).flags as libc::c_uint &
                   (1 as libc::c_uint) << 4 as libc::c_int != 0 &&
                   !(**mark).texinfo.is_null() &&
                   !(*(**mark).texinfo).texture.is_null() {
                return R_GetTexture((*(*(**mark).texinfo).texture).gl_texturenum
                                        as GLenum)
            }
            i += 1;
            mark = mark.offset(1)
        }
        // texture not found
        return 0 as *mut gl_texture_t
    }
    // this is a regular node
	// traverse children
    if !(*node).children[0 as libc::c_int as usize].is_null() &&
           (*node).children[0 as libc::c_int as usize] !=
               ignore as *mut mnode_s {
        tex =
            R_RecursiveFindWaterTexture((*node).children[0 as libc::c_int as
                                                             usize], node,
                                        true_0);
        if !tex.is_null() { return tex }
    }
    if !(*node).children[1 as libc::c_int as usize].is_null() &&
           (*node).children[1 as libc::c_int as usize] !=
               ignore as *mut mnode_s {
        tex =
            R_RecursiveFindWaterTexture((*node).children[1 as libc::c_int as
                                                             usize], node,
                                        true_0);
        if !tex.is_null() { return tex }
    }
    // for down recursion, return immediately
    if down as u64 != 0 { return 0 as *mut gl_texture_t }
    // texture not found, step up if any
    if !(*node).parent.is_null() {
        return R_RecursiveFindWaterTexture((*node).parent, node, false_0)
    }
    // top-level node, bail out
    return 0 as *mut gl_texture_t;
}
/*
=============
R_CheckFog

check for underwater fog
Using backward recursion to find waterline leaf
from underwater leaf (idea: XaeroX)
=============
*/
unsafe extern "C" fn R_CheckFog() {
    let mut ent: *mut cl_entity_t = 0 as *mut cl_entity_t;
    let mut tex: *mut gl_texture_t = 0 as *mut gl_texture_t;
    let mut i: libc::c_int = 0;
    let mut cnt: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    // quake global fog
    if Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(PARM_QUAKE_COMPATIBLE
                                                                                                                 as
                                                                                                                 libc::c_int,
                                                                                                             0
                                                                                                                 as
                                                                                                                 libc::c_int)
           != 0 {
        if (*gEngfuncs.pfnGetMoveVars.expect("non-null function pointer")()).fog_settings
               == 0 {
            if pglIsEnabled.expect("non-null function pointer")(0xb60 as
                                                                    libc::c_int
                                                                    as GLenum)
                   != 0 {
                pglDisable.expect("non-null function pointer")(0xb60 as
                                                                   libc::c_int
                                                                   as GLenum);
            }
            RI.fogEnabled = false_0;
            return
        }
        // quake-style global fog
        RI.fogColor[0 as libc::c_int as usize] =
            (((*gEngfuncs.pfnGetMoveVars.expect("non-null function pointer")()).fog_settings
                  as libc::c_uint & 0xff000000 as libc::c_uint) >>
                 24 as libc::c_int) as libc::c_float / 255.0f32;
        RI.fogColor[1 as libc::c_int as usize] =
            (((*gEngfuncs.pfnGetMoveVars.expect("non-null function pointer")()).fog_settings
                  & 0xff0000 as libc::c_int) >> 16 as libc::c_int) as
                libc::c_float / 255.0f32;
        RI.fogColor[2 as libc::c_int as usize] =
            (((*gEngfuncs.pfnGetMoveVars.expect("non-null function pointer")()).fog_settings
                  & 0xff00 as libc::c_int) >> 8 as libc::c_int) as
                libc::c_float / 255.0f32;
        RI.fogDensity =
            ((*gEngfuncs.pfnGetMoveVars.expect("non-null function pointer")()).fog_settings
                 & 0xff as libc::c_int) as libc::c_float / 255.0f32 * 0.01f32;
        RI.fogEnd = 0.0f32;
        RI.fogStart = RI.fogEnd;
        RI.fogColor[3 as libc::c_int as usize] = 1.0f32;
        RI.fogCustom = false_0;
        RI.fogEnabled = true_0;
        RI.fogSkybox = true_0;
        return
    }
    RI.fogEnabled = false_0;
    if RI.onlyClientDraw as libc::c_uint != 0 ||
           Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(PARM_WATER_LEVEL
                                                                                                                     as
                                                                                                                     libc::c_int,
                                                                                                                 0
                                                                                                                     as
                                                                                                                     libc::c_int)
               < 3 as libc::c_int || RI.drawWorld as u64 == 0 ||
           RI.viewleaf.is_null() {
        if RI.cached_waterlevel == 3 as libc::c_int {
            // in some cases waterlevel jumps from 3 to 1. Catch it
            RI.cached_waterlevel =
                Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(PARM_WATER_LEVEL
                                                                                                                          as
                                                                                                                          libc::c_int,
                                                                                                                      0
                                                                                                                          as
                                                                                                                          libc::c_int);
            RI.cached_contents = -(1 as libc::c_int);
            if RI.fogCustom as u64 == 0 {
                glState.isFogEnabled = false_0 as libc::c_int;
                pglDisable.expect("non-null function pointer")(0xb60 as
                                                                   libc::c_int
                                                                   as GLenum);
            }
        }
        return
    }
    ent =
        gEngfuncs.CL_GetWaterEntity.expect("non-null function pointer")(RI.vieworg.as_mut_ptr()
                                                                            as
                                                                            *const vec_t);
    if !ent.is_null() && !(*ent).model.is_null() &&
           (*(*ent).model).type_0 as libc::c_int == mod_brush as libc::c_int
           && ((*ent).curstate.skin as libc::c_int) < 0 as libc::c_int {
        cnt = (*ent).curstate.skin as libc::c_int
    } else { cnt = (*RI.viewleaf).contents }
    RI.cached_waterlevel =
        Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(PARM_WATER_LEVEL
                                                                                                                  as
                                                                                                                  libc::c_int,
                                                                                                              0
                                                                                                                  as
                                                                                                                  libc::c_int);
    if !(RI.cached_contents == -(3 as libc::c_int) ||
             RI.cached_contents == -(4 as libc::c_int) ||
             RI.cached_contents == -(5 as libc::c_int)) &&
           (cnt == -(3 as libc::c_int) || cnt == -(4 as libc::c_int) ||
                cnt == -(5 as libc::c_int)) {
        tex = 0 as *mut gl_texture_t;
        // check for water texture
        if !ent.is_null() && !(*ent).model.is_null() &&
               (*(*ent).model).type_0 as libc::c_int ==
                   mod_brush as libc::c_int {
            let mut surf: *mut msurface_t =
                0 as *mut msurface_t; // no valid fogs
            count = (*(*ent).model).nummodelsurfaces;
            i = 0 as libc::c_int;
            surf =
                &mut *(*(*ent).model).surfaces.offset((*(*ent).model).firstmodelsurface
                                                          as isize) as
                    *mut msurface_t;
            while i < count {
                if (*surf).flags as libc::c_uint &
                       (1 as libc::c_uint) << 4 as libc::c_int != 0 &&
                       !(*surf).texinfo.is_null() &&
                       !(*(*surf).texinfo).texture.is_null() {
                    tex =
                        R_GetTexture((*(*(*surf).texinfo).texture).gl_texturenum
                                         as GLenum);
                    RI.cached_contents = (*ent).curstate.skin as libc::c_int;
                    break ;
                } else { i += 1; surf = surf.offset(1) }
            }
        } else {
            tex =
                R_RecursiveFindWaterTexture((*RI.viewleaf).parent,
                                            0 as *const mnode_t, false_0);
            if !tex.is_null() { RI.cached_contents = (*RI.viewleaf).contents }
        }
        if tex.is_null() { return }
        // copy fog params
        RI.fogColor[0 as libc::c_int as usize] =
            (*tex).fogParams[0 as libc::c_int as usize] as libc::c_int as
                libc::c_float / 255.0f32;
        RI.fogColor[1 as libc::c_int as usize] =
            (*tex).fogParams[1 as libc::c_int as usize] as libc::c_int as
                libc::c_float / 255.0f32;
        RI.fogColor[2 as libc::c_int as usize] =
            (*tex).fogParams[2 as libc::c_int as usize] as libc::c_int as
                libc::c_float / 255.0f32;
        RI.fogDensity =
            (*tex).fogParams[3 as libc::c_int as usize] as libc::c_int as
                libc::c_float * 0.000025f32;
        RI.fogEnd = 0.0f32;
        RI.fogStart = RI.fogEnd;
        RI.fogColor[3 as libc::c_int as usize] = 1.0f32;
        RI.fogCustom = false_0;
        RI.fogEnabled = true_0;
        RI.fogSkybox = true_0
    } else {
        RI.fogCustom = false_0;
        RI.fogEnabled = true_0;
        RI.fogSkybox = true_0
    };
}
/*
=============
R_CheckGLFog

special condition for Spirit 1.9
that used direct calls of glFog-functions
=============
*/
unsafe extern "C" fn R_CheckGLFog() {
    if RI.fogEnabled as u64 == 0 && RI.fogCustom as u64 == 0 &&
           pglIsEnabled.expect("non-null function pointer")(0xb60 as
                                                                libc::c_int as
                                                                GLenum) as
               libc::c_int != 0 &&
           (RI.fogColor[0 as libc::c_int as usize] == 0.0f32 &&
                RI.fogColor[1 as libc::c_int as usize] == 0.0f32 &&
                RI.fogColor[2 as libc::c_int as usize] == 0.0f32) {
        // fill the fog color from GL-state machine
        pglGetFloatv.expect("non-null function pointer")(0xb66 as libc::c_int
                                                             as GLenum,
                                                         RI.fogColor.as_mut_ptr());
        RI.fogSkybox = true_0
    };
}
/*
=============
R_DrawFog

=============
*/
#[no_mangle]
pub unsafe extern "C" fn R_DrawFog() {
    if RI.fogEnabled as u64 == 0 { return }
    pglEnable.expect("non-null function pointer")(0xb60 as libc::c_int as
                                                      GLenum);
    if Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(PARM_QUAKE_COMPATIBLE
                                                                                                                 as
                                                                                                                 libc::c_int,
                                                                                                             0
                                                                                                                 as
                                                                                                                 libc::c_int)
           != 0 {
        pglFogi.expect("non-null function pointer")(0xb65 as libc::c_int as
                                                        GLenum,
                                                    0x801 as libc::c_int);
    } else {
        pglFogi.expect("non-null function pointer")(0xb65 as libc::c_int as
                                                        GLenum,
                                                    0x800 as libc::c_int);
    }
    pglFogf.expect("non-null function pointer")(0xb62 as libc::c_int as
                                                    GLenum, RI.fogDensity);
    pglFogfv.expect("non-null function pointer")(0xb66 as libc::c_int as
                                                     GLenum,
                                                 RI.fogColor.as_mut_ptr());
    pglHint.expect("non-null function pointer")(0xc54 as libc::c_int as
                                                    GLenum,
                                                0x1102 as libc::c_int as
                                                    GLenum);
}
/*
=============
R_DrawEntitiesOnList
=============
*/
#[no_mangle]
pub unsafe extern "C" fn R_DrawEntitiesOnList() {
    let mut i: libc::c_int = 0;
    tr.blend = 1.0f32;
    GL_CheckForErrors_(b"../ref_gl/gl_rmain.c\x00" as *const u8 as
                           *const libc::c_char, 816 as libc::c_int);
    // first draw solid entities
    i = 0 as libc::c_int;
    while (i as libc::c_uint) < (*tr.draw_list).num_solid_entities &&
              RI.onlyClientDraw as u64 == 0 {
        RI.currententity = (*tr.draw_list).solid_entities[i as usize];
        RI.currentmodel = (*RI.currententity).model;
        if RI.currententity.is_null() {
            gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     b"../ref_gl/gl_rmain.c\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     824 as
                                                                         libc::c_int);
        }
        if RI.currentmodel.is_null() {
            gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     b"../ref_gl/gl_rmain.c\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     825 as
                                                                         libc::c_int);
        }
        match (*RI.currentmodel).type_0 as libc::c_int {
            0 => { R_DrawBrushModel(RI.currententity); }
            2 => { R_DrawAliasModel(RI.currententity); }
            3 => { R_DrawStudioModel(RI.currententity); }
            _ => { }
        }
        i += 1
    }
    GL_CheckForErrors_(b"../ref_gl/gl_rmain.c\x00" as *const u8 as
                           *const libc::c_char, 843 as libc::c_int);
    // quake-specific feature
    R_DrawAlphaTextureChains();
    GL_CheckForErrors_(b"../ref_gl/gl_rmain.c\x00" as *const u8 as
                           *const libc::c_char, 848 as libc::c_int);
    // draw sprites seperately, because of alpha blending
    i = 0 as libc::c_int;
    while (i as libc::c_uint) < (*tr.draw_list).num_solid_entities &&
              RI.onlyClientDraw as u64 == 0 {
        RI.currententity = (*tr.draw_list).solid_entities[i as usize];
        RI.currentmodel = (*RI.currententity).model;
        if RI.currententity.is_null() {
            gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     b"../ref_gl/gl_rmain.c\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     856 as
                                                                         libc::c_int);
        }
        if RI.currentmodel.is_null() {
            gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     b"../ref_gl/gl_rmain.c\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     857 as
                                                                         libc::c_int);
        }
        match (*RI.currentmodel).type_0 as libc::c_int {
            1 => { R_DrawSpriteModel(RI.currententity); }
            _ => { }
        }
        i += 1
    }
    GL_CheckForErrors_(b"../ref_gl/gl_rmain.c\x00" as *const u8 as
                           *const libc::c_char, 867 as libc::c_int);
    if RI.onlyClientDraw as u64 == 0 {
        gEngfuncs.CL_DrawEFX.expect("non-null function pointer")(tr.frametime
                                                                     as
                                                                     libc::c_float,
                                                                 false_0);
    }
    GL_CheckForErrors_(b"../ref_gl/gl_rmain.c\x00" as *const u8 as
                           *const libc::c_char, 874 as libc::c_int);
    if RI.drawWorld as u64 != 0 {
        gEngfuncs.pfnDrawNormalTriangles.expect("non-null function pointer")();
    }
    GL_CheckForErrors_(b"../ref_gl/gl_rmain.c\x00" as *const u8 as
                           *const libc::c_char, 879 as libc::c_int);
    // then draw translucent entities
    i = 0 as libc::c_int;
    while (i as libc::c_uint) < (*tr.draw_list).num_trans_entities &&
              RI.onlyClientDraw as u64 == 0 {
        RI.currententity = (*tr.draw_list).trans_entities[i as usize];
        RI.currentmodel = (*RI.currententity).model;
        // handle studiomodels with custom rendermodes on texture
        if (*RI.currententity).curstate.rendermode !=
               kRenderNormal as libc::c_int {
            tr.blend =
                CL_FxBlend(RI.currententity) as libc::c_float / 255.0f32
        } else { tr.blend = 1.0f32 } // draw as solid but sorted by distance
        if !(tr.blend <= 0.0f32) {
            if RI.currententity.is_null() {
                gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char,
                                                                         b"../ref_gl/gl_rmain.c\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char,
                                                                         894
                                                                             as
                                                                             libc::c_int); // Trinity Render issues
            }
            if RI.currentmodel.is_null() {
                gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char,
                                                                         b"../ref_gl/gl_rmain.c\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char,
                                                                         895
                                                                             as
                                                                             libc::c_int);
            }
            match (*RI.currentmodel).type_0 as libc::c_int {
                0 => { R_DrawBrushModel(RI.currententity); }
                2 => { R_DrawAliasModel(RI.currententity); }
                3 => { R_DrawStudioModel(RI.currententity); }
                1 => { R_DrawSpriteModel(RI.currententity); }
                _ => { }
            }
        }
        i += 1
    }
    GL_CheckForErrors_(b"../ref_gl/gl_rmain.c\x00" as *const u8 as
                           *const libc::c_char, 916 as libc::c_int);
    if RI.drawWorld as u64 != 0 {
        pglTexEnvi.expect("non-null function pointer")(0x2300 as libc::c_int
                                                           as GLenum,
                                                       0x2200 as libc::c_int
                                                           as GLenum,
                                                       0x2100 as libc::c_int);
        gEngfuncs.pfnDrawTransparentTriangles.expect("non-null function pointer")();
    }
    GL_CheckForErrors_(b"../ref_gl/gl_rmain.c\x00" as *const u8 as
                           *const libc::c_char, 924 as libc::c_int);
    if RI.onlyClientDraw as u64 == 0 {
        R_AllowFog(false_0);
        gEngfuncs.CL_DrawEFX.expect("non-null function pointer")(tr.frametime
                                                                     as
                                                                     libc::c_float,
                                                                 true_0);
        R_AllowFog(true_0);
    }
    GL_CheckForErrors_(b"../ref_gl/gl_rmain.c\x00" as *const u8 as
                           *const libc::c_char, 933 as libc::c_int);
    pglDisable.expect("non-null function pointer")(0xbe2 as libc::c_int as
                                                       GLenum);
    if RI.onlyClientDraw as u64 == 0 { R_DrawViewModel(); }
    gEngfuncs.CL_ExtraUpdate.expect("non-null function pointer")();
    GL_CheckForErrors_(b"../ref_gl/gl_rmain.c\x00" as *const u8 as
                           *const libc::c_char, 941 as libc::c_int);
}
/*
================
R_RenderScene

R_SetupRefParams must be called right before
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_RenderScene() {
    if gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1 as
                                                                            libc::c_int).is_null()
           && RI.drawWorld as libc::c_uint != 0 {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"R_RenderView: NULL worldmodel\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char);
    }
    // frametime is valid only for normal pass
    if RI.params as libc::c_uint & (1 as libc::c_uint) << 0 as libc::c_int ==
           0 as libc::c_int as libc::c_uint {
        tr.frametime =
            ((*gpGlobals).time - (*gpGlobals).oldtime) as libc::c_double
    } else { tr.frametime = 0.0f64 }
    // begin a new frame
    tr.framecount += 1; // don't let sound get messed up if going slow
    R_PushDlights();
    R_SetupFrustum();
    R_SetupFrame();
    R_SetupGL(true_0);
    R_Clear(!(0 as libc::c_int));
    R_MarkLeaves();
    R_DrawFog();
    R_CheckGLFog();
    R_DrawWorld();
    R_CheckFog();
    gEngfuncs.CL_ExtraUpdate.expect("non-null function pointer")();
    R_DrawEntitiesOnList();
    R_DrawWaterSurfaces();
    R_EndGL();
}
/*
===============
R_CheckGamma
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_CheckGamma() {
    if gEngfuncs.R_DoResetGamma.expect("non-null function pointer")() as u64
           != 0 {
        // paranoia cubemaps uses this
        gEngfuncs.BuildGammaTable.expect("non-null function pointer")(1.8f32,
                                                                      0.0f32);
        // paranoia cubemap rendering
        if (*gEngfuncs.drawFuncs).GL_BuildLightmaps.is_some() {
            (*gEngfuncs.drawFuncs).GL_BuildLightmaps.expect("non-null function pointer")();
        }
    } else if (*vid_gamma).flags & (1 as libc::c_int) << 13 as libc::c_int !=
                  0 ||
                  (*vid_brightness).flags &
                      (1 as libc::c_int) << 13 as libc::c_int != 0 {
        gEngfuncs.BuildGammaTable.expect("non-null function pointer")((*vid_gamma).value,
                                                                      (*vid_brightness).value);
        glConfig.softwareGammaUpdate = true_0;
        GL_RebuildLightmaps();
        glConfig.softwareGammaUpdate = false_0
    };
}
/*
===============
R_BeginFrame
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_BeginFrame(mut clearScene: qboolean) {
    glConfig.softwareGammaUpdate = false_0; // in case of possible fails
    if ((*gl_clear).value != 0. ||
            Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(PARM_DEV_OVERVIEW
                                                                                                                      as
                                                                                                                      libc::c_int,
                                                                                                                  0
                                                                                                                      as
                                                                                                                      libc::c_int)
                != 0) && clearScene as libc::c_uint != 0 &&
           Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(PARM_CONNSTATE
                                                                                                                     as
                                                                                                                     libc::c_int,
                                                                                                                 0
                                                                                                                     as
                                                                                                                     libc::c_int)
               != ca_cinematic as libc::c_int {
        pglClear.expect("non-null function pointer")(0x4000 as libc::c_int as
                                                         GLbitfield);
    }
    R_CheckGamma();
    R_Set2DMode(true_0);
    // draw buffer stuff
    pglDrawBuffer.expect("non-null function pointer")(0x405 as libc::c_int as
                                                          GLenum);
    // update texture parameters
    if ((*gl_texture_nearest).flags | (*gl_lightmap_nearest).flags |
            (*gl_texture_anisotropy).flags | (*gl_texture_lodbias).flags) &
           (1 as libc::c_int) << 13 as libc::c_int != 0 {
        R_SetTextureParameters();
    }
    gEngfuncs.CL_ExtraUpdate.expect("non-null function pointer")();
}
/*
===============
R_SetupRefParams

set initial params for renderer
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_SetupRefParams(mut rvp: *const ref_viewpass_t) {
    RI.params = 0 as libc::c_int;
    RI.drawWorld =
        ((*rvp).flags & (1 as libc::c_int) << 0 as libc::c_int) as qboolean;
    RI.onlyClientDraw =
        ((*rvp).flags & (1 as libc::c_int) << 3 as libc::c_int) as qboolean;
    RI.farClip = 0 as libc::c_int as libc::c_float;
    if (*rvp).flags & (1 as libc::c_int) << 1 as libc::c_int == 0 {
        RI.drawOrtho =
            ((*rvp).flags & (1 as libc::c_int) << 2 as libc::c_int) as
                qboolean
    } else { RI.drawOrtho = false_0 }
    // setup viewport
    RI.viewport[0 as libc::c_int as usize] =
        (*rvp).viewport[0 as libc::c_int as usize];
    RI.viewport[1 as libc::c_int as usize] =
        (*rvp).viewport[1 as libc::c_int as usize];
    RI.viewport[2 as libc::c_int as usize] =
        (*rvp).viewport[2 as libc::c_int as usize];
    RI.viewport[3 as libc::c_int as usize] =
        (*rvp).viewport[3 as libc::c_int as usize];
    // calc FOV
    RI.fov_x = (*rvp).fov_x;
    RI.fov_y = (*rvp).fov_y;
    RI.vieworg[0 as libc::c_int as usize] =
        (*rvp).vieworigin[0 as libc::c_int as usize];
    RI.vieworg[1 as libc::c_int as usize] =
        (*rvp).vieworigin[1 as libc::c_int as usize];
    RI.vieworg[2 as libc::c_int as usize] =
        (*rvp).vieworigin[2 as libc::c_int as usize];
    RI.viewangles[0 as libc::c_int as usize] =
        (*rvp).viewangles[0 as libc::c_int as usize];
    RI.viewangles[1 as libc::c_int as usize] =
        (*rvp).viewangles[1 as libc::c_int as usize];
    RI.viewangles[2 as libc::c_int as usize] =
        (*rvp).viewangles[2 as libc::c_int as usize];
    RI.pvsorigin[0 as libc::c_int as usize] =
        (*rvp).vieworigin[0 as libc::c_int as usize];
    RI.pvsorigin[1 as libc::c_int as usize] =
        (*rvp).vieworigin[1 as libc::c_int as usize];
    RI.pvsorigin[2 as libc::c_int as usize] =
        (*rvp).vieworigin[2 as libc::c_int as usize];
}
/*
===============
R_RenderFrame
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_RenderFrame(mut rvp: *const ref_viewpass_t) {
    if (*r_norefresh).value != 0. { return }
    // setup the initial render params
    R_SetupRefParams(rvp);
    if (*gl_finish).value != 0. && RI.drawWorld as libc::c_uint != 0 {
        pglFinish.expect("non-null function pointer")();
    }
    if glConfig.max_multisamples > 1 as libc::c_int &&
           (*gl_msaa).flags & (1 as libc::c_int) << 13 as libc::c_int != 0 {
        if if !gl_msaa.is_null() && (*gl_msaa).value != 0.0f32 {
               true_0 as libc::c_int
           } else { false_0 as libc::c_int } != 0 {
            pglEnable.expect("non-null function pointer")(0x809d as
                                                              libc::c_int as
                                                              GLenum);
        } else {
            pglDisable.expect("non-null function pointer")(0x809d as
                                                               libc::c_int as
                                                               GLenum);
        }
        (*gl_msaa).flags =
            (*gl_msaa).flags & !((1 as libc::c_int) << 13 as libc::c_int)
    }
    // completely override rendering
    if (*gEngfuncs.drawFuncs).GL_RenderFrame.is_some() {
        tr.fCustomRendering = true_0; // right called after viewmodel events
        if (*gEngfuncs.drawFuncs).GL_RenderFrame.expect("non-null function pointer")(rvp)
               != 0 {
            R_GatherPlayerLight();
            tr.realframecount += 1;
            tr.fResetVis = true_0;
            return
        }
    }
    tr.fCustomRendering = false_0;
    if RI.onlyClientDraw as u64 == 0 { R_RunViewmodelEvents(); }
    tr.realframecount += 1;
    R_RenderScene();
}
/*
===============
R_EndFrame
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_EndFrame() {
    // flush any remaining 2D bits
    R_Set2DMode(false_0);
    gEngfuncs.GL_SwapBuffers.expect("non-null function pointer")();
}
/*
===============
R_DrawCubemapView
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_DrawCubemapView(mut origin: *const vec_t,
                                           mut angles: *const vec_t,
                                           mut size: libc::c_int) {
    let mut rvp: ref_viewpass_t =
        ref_viewpass_t{viewport: [0; 4],
                       vieworigin: [0.; 3],
                       viewangles: [0.; 3],
                       viewentity: 0,
                       fov_x: 0.,
                       fov_y: 0.,
                       flags: 0,};
    // basic params
    rvp.viewentity = 0 as libc::c_int; // this is a final fov value
    rvp.flags = rvp.viewentity;
    rvp.flags = rvp.flags | (1 as libc::c_int) << 0 as libc::c_int;
    rvp.flags = rvp.flags | (1 as libc::c_int) << 1 as libc::c_int;
    rvp.viewport[1 as libc::c_int as usize] = 0 as libc::c_int;
    rvp.viewport[0 as libc::c_int as usize] =
        rvp.viewport[1 as libc::c_int as usize];
    rvp.viewport[3 as libc::c_int as usize] = size;
    rvp.viewport[2 as libc::c_int as usize] =
        rvp.viewport[3 as libc::c_int as usize];
    rvp.fov_y = 90.0f32;
    rvp.fov_x = rvp.fov_y;
    // setup origin & angles
    rvp.vieworigin[0 as libc::c_int as usize] =
        *origin.offset(0 as libc::c_int as isize);
    rvp.vieworigin[1 as libc::c_int as usize] =
        *origin.offset(1 as libc::c_int as isize);
    rvp.vieworigin[2 as libc::c_int as usize] =
        *origin.offset(2 as libc::c_int as isize);
    rvp.viewangles[0 as libc::c_int as usize] =
        *angles.offset(0 as libc::c_int as isize);
    rvp.viewangles[1 as libc::c_int as usize] =
        *angles.offset(1 as libc::c_int as isize);
    rvp.viewangles[2 as libc::c_int as usize] =
        *angles.offset(2 as libc::c_int as isize);
    R_RenderFrame(&mut rvp);
    RI.viewleaf = 0 as *mut mleaf_t;
    // force markleafs next frame
}
/*
===============
CL_FxBlend
===============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_FxBlend(mut e: *mut cl_entity_t) -> libc::c_int {
    let mut blend: libc::c_int =
        0 as libc::c_int; // Use ent index to de-sync these fx
    let mut offset: libc::c_float = 0.;
    let mut dist: libc::c_float = 0.;
    let mut tmp: vec3_t = [0.; 3];
    offset = (*e).index as libc::c_float * 363.0f32;
    match (*e).curstate.renderfx {
        3 => {
            blend =
                ((*e).curstate.renderamt as libc::c_float +
                     0x40 as libc::c_int as libc::c_float *
                         __tg_sin((*gpGlobals).time *
                                      2 as libc::c_int as libc::c_float +
                                      offset)) as libc::c_int
        }
        4 => {
            blend =
                ((*e).curstate.renderamt as libc::c_float +
                     0x40 as libc::c_int as libc::c_float *
                         __tg_sin((*gpGlobals).time *
                                      8 as libc::c_int as libc::c_float +
                                      offset)) as libc::c_int
        }
        1 => {
            blend =
                ((*e).curstate.renderamt as libc::c_float +
                     0x10 as libc::c_int as libc::c_float *
                         __tg_sin((*gpGlobals).time *
                                      2 as libc::c_int as libc::c_float +
                                      offset)) as libc::c_int
        }
        2 => {
            blend =
                ((*e).curstate.renderamt as libc::c_float +
                     0x10 as libc::c_int as libc::c_float *
                         __tg_sin((*gpGlobals).time *
                                      8 as libc::c_int as libc::c_float +
                                      offset)) as libc::c_int
        }
        5 => {
            if RI.params as libc::c_uint &
                   (1 as libc::c_uint) << 0 as libc::c_int ==
                   0 as libc::c_int as libc::c_uint {
                if (*e).curstate.renderamt > 0 as libc::c_int {
                    (*e).curstate.renderamt -= 1 as libc::c_int
                } else { (*e).curstate.renderamt = 0 as libc::c_int }
            }
            blend = (*e).curstate.renderamt
        }
        6 => {
            if RI.params as libc::c_uint &
                   (1 as libc::c_uint) << 0 as libc::c_int ==
                   0 as libc::c_int as libc::c_uint {
                if (*e).curstate.renderamt > 3 as libc::c_int {
                    (*e).curstate.renderamt -= 4 as libc::c_int
                } else { (*e).curstate.renderamt = 0 as libc::c_int }
            }
            blend = (*e).curstate.renderamt
        }
        7 => {
            if RI.params as libc::c_uint &
                   (1 as libc::c_uint) << 0 as libc::c_int ==
                   0 as libc::c_int as libc::c_uint {
                if (*e).curstate.renderamt < 255 as libc::c_int {
                    (*e).curstate.renderamt += 1 as libc::c_int
                } else { (*e).curstate.renderamt = 255 as libc::c_int }
            }
            blend = (*e).curstate.renderamt
        }
        8 => {
            if RI.params as libc::c_uint &
                   (1 as libc::c_uint) << 0 as libc::c_int ==
                   0 as libc::c_int as libc::c_uint {
                if (*e).curstate.renderamt < 252 as libc::c_int {
                    (*e).curstate.renderamt += 4 as libc::c_int
                } else { (*e).curstate.renderamt = 255 as libc::c_int }
            }
            blend = (*e).curstate.renderamt
        }
        9 => {
            blend =
                (20 as libc::c_int as libc::c_float *
                     __tg_sin((*gpGlobals).time *
                                  4 as libc::c_int as libc::c_float + offset))
                    as libc::c_int;
            if blend < 0 as libc::c_int {
                blend = 0 as libc::c_int
            } else { blend = (*e).curstate.renderamt }
        }
        10 => {
            blend =
                (20 as libc::c_int as libc::c_float *
                     __tg_sin((*gpGlobals).time *
                                  16 as libc::c_int as libc::c_float +
                                  offset)) as libc::c_int;
            if blend < 0 as libc::c_int {
                blend = 0 as libc::c_int
            } else { blend = (*e).curstate.renderamt }
        }
        11 => {
            blend =
                (20 as libc::c_int as libc::c_float *
                     __tg_sin((*gpGlobals).time *
                                  36 as libc::c_int as libc::c_float +
                                  offset)) as libc::c_int;
            if blend < 0 as libc::c_int {
                blend = 0 as libc::c_int
            } else { blend = (*e).curstate.renderamt }
        }
        12 => {
            blend =
                (20 as libc::c_int as libc::c_float *
                     (__tg_sin((*gpGlobals).time *
                                   2 as libc::c_int as libc::c_float) +
                          __tg_sin((*gpGlobals).time *
                                       17 as libc::c_int as libc::c_float +
                                       offset))) as libc::c_int;
            if blend < 0 as libc::c_int {
                blend = 0 as libc::c_int
            } else { blend = (*e).curstate.renderamt }
        }
        13 => {
            blend =
                (20 as libc::c_int as libc::c_float *
                     (__tg_sin((*gpGlobals).time *
                                   16 as libc::c_int as libc::c_float) +
                          __tg_sin((*gpGlobals).time *
                                       23 as libc::c_int as libc::c_float +
                                       offset))) as libc::c_int;
            if blend < 0 as libc::c_int {
                blend = 0 as libc::c_int
            } else { blend = (*e).curstate.renderamt }
        }
        16 | 15 => {
            tmp[0 as libc::c_int as usize] =
                (*e).origin[0 as libc::c_int as usize];
            tmp[1 as libc::c_int as usize] =
                (*e).origin[1 as libc::c_int as usize];
            tmp[2 as libc::c_int as usize] =
                (*e).origin[2 as libc::c_int as usize];
            tmp[0 as libc::c_int as usize] =
                tmp[0 as libc::c_int as usize] -
                    RI.vieworg[0 as libc::c_int as usize];
            tmp[1 as libc::c_int as usize] =
                tmp[1 as libc::c_int as usize] -
                    RI.vieworg[1 as libc::c_int as usize];
            tmp[2 as libc::c_int as usize] =
                tmp[2 as libc::c_int as usize] -
                    RI.vieworg[2 as libc::c_int as usize];
            dist =
                tmp[0 as libc::c_int as usize] *
                    RI.vforward[0 as libc::c_int as usize] +
                    tmp[1 as libc::c_int as usize] *
                        RI.vforward[1 as libc::c_int as usize] +
                    tmp[2 as libc::c_int as usize] *
                        RI.vforward[2 as libc::c_int as usize];
            // turn off distance fade
            if (*e).curstate.renderfx == kRenderFxDistort as libc::c_int {
                dist = 1 as libc::c_int as libc::c_float
            }
            if dist <= 0 as libc::c_int as libc::c_float {
                blend = 0 as libc::c_int
            } else {
                (*e).curstate.renderamt = 180 as libc::c_int;
                if dist <= 100 as libc::c_int as libc::c_float {
                    blend = (*e).curstate.renderamt
                } else {
                    blend =
                        ((1.0f32 -
                              (dist - 100 as libc::c_int as libc::c_float) *
                                  (1.0f32 / 400.0f32)) *
                             (*e).curstate.renderamt as libc::c_float) as
                            libc::c_int
                }
                blend +=
                    gEngfuncs.COM_RandomLong.expect("non-null function pointer")(-(32
                                                                                       as
                                                                                       libc::c_int),
                                                                                 31
                                                                                     as
                                                                                     libc::c_int)
            }
        }
        _ => { blend = (*e).curstate.renderamt }
    }
    blend =
        if blend >= 0 as libc::c_int {
            if blend < 255 as libc::c_int {
                blend
            } else { 255 as libc::c_int }
        } else { 0 as libc::c_int };
    return blend;
}
