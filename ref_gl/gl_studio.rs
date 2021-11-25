#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, extern_types,
           register_tool)]
extern "C" {
    pub type grasshdr_s;
    pub type con_nprint_s;
    pub type client_textmessage_s;
    pub type screenfade_s;
    pub type world_static_s;
    #[no_mangle]
    fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t,
             __compar: __compar_fn_t);
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn cos(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn sin(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn atan2f(_: libc::c_float, _: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn sqrtf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn HalfToFloat(h: word) -> libc::c_float;
    #[no_mangle]
    fn VectorCompareEpsilon(vec1: *const vec_t, vec2: *const vec_t,
                            epsilon: vec_t) -> qboolean;
    #[no_mangle]
    fn ClearBounds(mins: *mut vec_t, maxs: *mut vec_t);
    #[no_mangle]
    fn AddPointToBounds(v: *const vec_t, mins: *mut vec_t, maxs: *mut vec_t);
    #[no_mangle]
    fn AngleQuaternion(angles: *const vec_t, q: *mut vec_t, studio: qboolean);
    #[no_mangle]
    fn QuaternionAngle(q: *const vec_t, angles: *mut vec_t);
    #[no_mangle]
    fn QuaternionSlerp(p: *const vec_t, q: *const vec_t, t: libc::c_float,
                       qt: *mut vec_t);
    #[no_mangle]
    fn Matrix3x4_VectorTransform(in_0: *const [vec_t; 4],
                                 v: *const libc::c_float,
                                 out: *mut libc::c_float);
    #[no_mangle]
    fn Matrix3x4_VectorITransform(in_0: *const [vec_t; 4],
                                  v: *const libc::c_float,
                                  out: *mut libc::c_float);
    #[no_mangle]
    fn Matrix3x4_VectorRotate(in_0: *const [vec_t; 4],
                              v: *const libc::c_float,
                              out: *mut libc::c_float);
    #[no_mangle]
    fn Matrix3x4_VectorIRotate(in_0: *const [vec_t; 4],
                               v: *const libc::c_float,
                               out: *mut libc::c_float);
    #[no_mangle]
    fn Matrix3x4_ConcatTransforms(out: *mut [vec_t; 4],
                                  in1: *const [vec_t; 4],
                                  in2: *const [vec_t; 4]);
    #[no_mangle]
    fn Matrix3x4_FromOriginQuat(out: *mut [vec_t; 4],
                                quaternion: *const vec_t,
                                origin: *const vec_t);
    #[no_mangle]
    fn Matrix3x4_CreateFromEntity(out: *mut [vec_t; 4], angles: *const vec_t,
                                  origin: *const vec_t, scale: libc::c_float);
    #[no_mangle]
    fn Matrix3x4_OriginFromMatrix(in_0: *const [vec_t; 4],
                                  out: *mut libc::c_float);
    #[no_mangle]
    static mut vec3_origin: vec3_t;
    #[no_mangle]
    static mut boxpnt: [[libc::c_int; 4]; 6];
    #[no_mangle]
    static matrix3x4_identity: matrix3x4;
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
    static mut pglColor4ubv:
           Option<unsafe extern "C" fn(_: *const GLubyte) -> ()>;
    #[no_mangle]
    static mut pglColorPointer:
           Option<unsafe extern "C" fn(_: GLint, _: GLenum, _: GLsizei,
                                       _: *const libc::c_void) -> ()>;
    #[no_mangle]
    static mut pglDepthFunc: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglDepthMask: Option<unsafe extern "C" fn(_: GLboolean) -> ()>;
    #[no_mangle]
    static mut pglDepthRange:
           Option<unsafe extern "C" fn(_: GLclampd, _: GLclampd) -> ()>;
    #[no_mangle]
    static mut pglDisable: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglDisableClientState:
           Option<unsafe extern "C" fn(_: GLenum) -> ()>;
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
    static mut pglFrontFace: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglPointSize: Option<unsafe extern "C" fn(_: GLfloat) -> ()>;
    #[no_mangle]
    static mut pglShadeModel: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
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
    static mut r_stats: ref_speeds_t;
    #[no_mangle]
    static mut RI: ref_instance_t;
    #[no_mangle]
    static mut tr: gl_globals_t;
    #[no_mangle]
    static mut gldepthmin: libc::c_float;
    #[no_mangle]
    static mut gldepthmax: libc::c_float;
    #[no_mangle]
    fn GL_Bind(tmu: GLint, texnum: GLenum);
    #[no_mangle]
    fn R_CullModel(e: *mut cl_entity_t, absmin: *const vec_t,
                   absmax: *const vec_t) -> libc::c_int;
    #[no_mangle]
    fn GL_LoadTexture(name: *const libc::c_char, buf: *const byte,
                      size: size_t, flags: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn GL_FreeTexture(texnum: GLenum);
    #[no_mangle]
    fn R_LightVec(start: *const vec_t, end: *const vec_t,
                  lightspot: *mut vec_t, lightvec: *mut vec_t) -> colorVec;
    #[no_mangle]
    fn R_LightPoint(p0: *const vec_t) -> colorVec;
    #[no_mangle]
    fn R_AllowFog(allowed: qboolean);
    #[no_mangle]
    fn CL_FxBlend(e: *mut cl_entity_t) -> libc::c_int;
    #[no_mangle]
    static mut gEngfuncs: ref_api_t;
    #[no_mangle]
    fn Q_snprintf(buffer: *mut libc::c_char, buffersize: size_t,
                  format: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Q_strncmp(s1: *const libc::c_char, s2: *const libc::c_char,
                 n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    static mut gpGlobals: *mut ref_globals_t;
    #[no_mangle]
    static mut r_drawentities: *mut cvar_t;
    #[no_mangle]
    fn TriRenderMode(mode: libc::c_int);
    #[no_mangle]
    fn TriEnd();
    #[no_mangle]
    fn TriVertex3fv(v: *const libc::c_float);
    #[no_mangle]
    fn TriBrightness(brightness: libc::c_float);
    #[no_mangle]
    fn TriBegin(mode: libc::c_int);
    #[no_mangle]
    fn TriColor4f(r: libc::c_float, g: libc::c_float, b: libc::c_float,
                  a: libc::c_float);
    #[no_mangle]
    static mut glState: glstate_t;
    #[no_mangle]
    static mut r_fullbright: *mut cvar_t;
    #[no_mangle]
    static mut r_lightmap: *mut cvar_t;
    #[no_mangle]
    fn TriColor4ub(r: byte, g: byte, b: byte, a: byte);
    #[no_mangle]
    fn TriSpriteTexture(pSpriteModel: *mut model_t, frame: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    static mut r_dynamic: *mut cvar_t;
    #[no_mangle]
    fn Q_strnicmp(s1: *const libc::c_char, s2: *const libc::c_char,
                  n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn COM_StripExtension(path: *mut libc::c_char);
    #[no_mangle]
    fn COM_FileBase(in_0: *const libc::c_char, out: *mut libc::c_char);
    #[no_mangle]
    fn Q_atoi(str: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn R_DrawAliasModel(e: *mut cl_entity_t);
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type off_t = __off_t;
pub type uint = libc::c_uint;
pub type int8_t = __int8_t;
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
pub type matrix3x4 = [[vec_t; 4]; 3];
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct alight_s {
    pub ambientlight: libc::c_int,
    pub shadelight: libc::c_int,
    pub color: vec3_t,
    pub plightvec: *mut libc::c_float,
}
pub type alight_t = alight_s;
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
pub type C2RustUnnamed_1 = libc::c_uint;
pub const TF_MULTISAMPLE: C2RustUnnamed_1 = 536870912;
pub const TF_ARB_16BIT: C2RustUnnamed_1 = 268435456;
pub const TF_NOCOMPARE: C2RustUnnamed_1 = 134217728;
pub const TF_ARB_FLOAT: C2RustUnnamed_1 = 67108864;
pub const TF_IMG_UPLOADED: C2RustUnnamed_1 = 33554432;
pub const TF_ALPHACONTRAST: C2RustUnnamed_1 = 4194304;
pub const TF_ATLAS_PAGE: C2RustUnnamed_1 = 2097152;
pub const TF_TEXTURE_3D: C2RustUnnamed_1 = 1048576;
pub const TF_BORDER: C2RustUnnamed_1 = 524288;
pub const TF_UPDATE: C2RustUnnamed_1 = 262144;
pub const TF_FORCE_COLOR: C2RustUnnamed_1 = 131072;
pub const TF_HAS_ALPHA: C2RustUnnamed_1 = 65536;
pub const TF_NORMALMAP: C2RustUnnamed_1 = 32768;
pub const TF_MAKELUMA: C2RustUnnamed_1 = 16384;
pub const TF_HAS_LUMA: C2RustUnnamed_1 = 8192;
pub const TF_NOMIPMAP: C2RustUnnamed_1 = 4096;
pub const TF_CLAMP: C2RustUnnamed_1 = 2048;
pub const TF_SKYSIDE: C2RustUnnamed_1 = 1024;
pub const TF_LUMINANCE: C2RustUnnamed_1 = 512;
pub const TF_QUAKEPAL: C2RustUnnamed_1 = 256;
pub const TF_DEPTHMAP: C2RustUnnamed_1 = 128;
pub const TF_CUBEMAP: C2RustUnnamed_1 = 64;
pub const TF_RECTANGLE: C2RustUnnamed_1 = 32;
pub const TF_ALLOW_EMBOSS: C2RustUnnamed_1 = 16;
pub const TF_EXPAND_SOURCE: C2RustUnnamed_1 = 8;
pub const TF_NOFLIP_TGA: C2RustUnnamed_1 = 4;
pub const TF_KEEP_SOURCE: C2RustUnnamed_1 = 2;
pub const TF_NEAREST: C2RustUnnamed_1 = 1;
pub const TF_COLORMAP: C2RustUnnamed_1 = 0;
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
pub struct mstudioboneinfo_t {
    pub poseToBone: [[vec_t; 4]; 3],
    pub qAlignment: vec4_t,
    pub proctype: int32_t,
    pub procindex: int32_t,
    pub quat: vec4_t,
    pub reserved: [int32_t; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mstudiobonecontroller_t {
    pub bone: int32_t,
    pub type_0: int32_t,
    pub start: vec_t,
    pub end: vec_t,
    pub unused: int32_t,
    pub index: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mstudiobbox_t {
    pub bone: int32_t,
    pub group: int32_t,
    pub bbmin: vec3_t,
    pub bbmax: vec3_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mstudioevent_s {
    pub frame: int32_t,
    pub event: int32_t,
    pub unused: int32_t,
    pub options: [libc::c_char; 64],
}
pub type mstudioevent_t = mstudioevent_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mstudioattachment_t {
    pub unused: [libc::c_char; 32],
    pub flags: int32_t,
    pub bone: int32_t,
    pub org: vec3_t,
    pub vectors: [vec3_t; 3],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mstudiobodyparts_t {
    pub name: [libc::c_char; 64],
    pub nummodels: int32_t,
    pub base: int32_t,
    pub modelindex: int32_t,
}
pub type mstudiotexture_t = mstudiotex_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mstudioboneweight_t {
    pub weight: [uint8_t; 4],
    pub bone: [int8_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mstudiomodel_t {
    pub name: [libc::c_char; 64],
    pub unused: int32_t,
    pub unused2: vec_t,
    pub nummesh: int32_t,
    pub meshindex: int32_t,
    pub numverts: int32_t,
    pub vertinfoindex: int32_t,
    pub vertindex: int32_t,
    pub numnorms: int32_t,
    pub norminfoindex: int32_t,
    pub normindex: int32_t,
    pub blendvertinfoindex: int32_t,
    pub blendnorminfoindex: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mstudiomesh_t {
    pub numtris: int32_t,
    pub triindex: int32_t,
    pub skinref: int32_t,
    pub numnorms: int32_t,
    pub unused: int32_t,
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
pub struct engine_studio_api_s {
    pub Mem_Calloc: Option<unsafe extern "C" fn(_: libc::c_int, _: size_t)
                               -> *mut libc::c_void>,
    pub Cache_Check: Option<unsafe extern "C" fn(_: *mut cache_user_s)
                                -> *mut libc::c_void>,
    pub LoadCacheFile: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                   _: *mut cache_user_s)
                                  -> ()>,
    pub Mod_ForName: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                 _: libc::c_int)
                                -> *mut model_s>,
    pub Mod_Extradata: Option<unsafe extern "C" fn(_: *mut model_s)
                                  -> *mut libc::c_void>,
    pub GetModelByIndex: Option<unsafe extern "C" fn(_: libc::c_int)
                                    -> *mut model_s>,
    pub GetCurrentEntity: Option<unsafe extern "C" fn() -> *mut cl_entity_s>,
    pub PlayerInfo: Option<unsafe extern "C" fn(_: libc::c_int)
                               -> *mut player_info_s>,
    pub GetPlayerState: Option<unsafe extern "C" fn(_: libc::c_int)
                                   -> *mut entity_state_s>,
    pub GetViewEntity: Option<unsafe extern "C" fn() -> *mut cl_entity_s>,
    pub GetTimes: Option<unsafe extern "C" fn(_: *mut libc::c_int,
                                              _: *mut libc::c_double,
                                              _: *mut libc::c_double) -> ()>,
    pub GetCvar: Option<unsafe extern "C" fn(_: *const libc::c_char)
                            -> *mut cvar_s>,
    pub GetViewInfo: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                 _: *mut libc::c_float,
                                                 _: *mut libc::c_float,
                                                 _: *mut libc::c_float)
                                -> ()>,
    pub GetChromeSprite: Option<unsafe extern "C" fn() -> *mut model_s>,
    pub GetModelCounters: Option<unsafe extern "C" fn(_:
                                                          *mut *mut libc::c_int,
                                                      _:
                                                          *mut *mut libc::c_int)
                                     -> ()>,
    pub GetAliasScale: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                   _: *mut libc::c_float)
                                  -> ()>,
    pub StudioGetBoneTransform: Option<unsafe extern "C" fn()
                                           ->
                                               *mut *mut *mut *mut libc::c_float>,
    pub StudioGetLightTransform: Option<unsafe extern "C" fn()
                                            ->
                                                *mut *mut *mut *mut libc::c_float>,
    pub StudioGetAliasTransform: Option<unsafe extern "C" fn()
                                            -> *mut *mut *mut libc::c_float>,
    pub StudioGetRotationMatrix: Option<unsafe extern "C" fn()
                                            -> *mut *mut *mut libc::c_float>,
    pub StudioSetupModel: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _:
                                                          *mut *mut libc::c_void,
                                                      _:
                                                          *mut *mut libc::c_void)
                                     -> ()>,
    pub StudioCheckBBox: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub StudioDynamicLight: Option<unsafe extern "C" fn(_: *mut cl_entity_s,
                                                        _: *mut alight_s)
                                       -> ()>,
    pub StudioEntityLight: Option<unsafe extern "C" fn(_: *mut alight_s)
                                      -> ()>,
    pub StudioSetupLighting: Option<unsafe extern "C" fn(_: *mut alight_s)
                                        -> ()>,
    pub StudioDrawPoints: Option<unsafe extern "C" fn() -> ()>,
    pub StudioDrawHulls: Option<unsafe extern "C" fn() -> ()>,
    pub StudioDrawAbsBBox: Option<unsafe extern "C" fn() -> ()>,
    pub StudioDrawBones: Option<unsafe extern "C" fn() -> ()>,
    pub StudioSetupSkin: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                     _: libc::c_int) -> ()>,
    pub StudioSetRemapColors: Option<unsafe extern "C" fn(_: libc::c_int,
                                                          _: libc::c_int)
                                         -> ()>,
    pub SetupPlayerModel: Option<unsafe extern "C" fn(_: libc::c_int)
                                     -> *mut model_s>,
    pub StudioClientEvents: Option<unsafe extern "C" fn() -> ()>,
    pub GetForceFaceFlags: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub SetForceFaceFlags: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub StudioSetHeader: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                    -> ()>,
    pub SetRenderModel: Option<unsafe extern "C" fn(_: *mut model_s) -> ()>,
    pub SetupRenderer: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub RestoreRenderer: Option<unsafe extern "C" fn() -> ()>,
    pub SetChromeOrigin: Option<unsafe extern "C" fn() -> ()>,
    pub IsHardware: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub GL_StudioDrawShadow: Option<unsafe extern "C" fn() -> ()>,
    pub GL_SetRenderMode: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub StudioSetRenderamt: Option<unsafe extern "C" fn(_: libc::c_int)
                                       -> ()>,
    pub StudioSetCullState: Option<unsafe extern "C" fn(_: libc::c_int)
                                       -> ()>,
    pub StudioRenderShadow: Option<unsafe extern "C" fn(_: libc::c_int,
                                                        _: *mut libc::c_float,
                                                        _: *mut libc::c_float,
                                                        _: *mut libc::c_float,
                                                        _: *mut libc::c_float)
                                       -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct r_studio_interface_s {
    pub version: libc::c_int,
    pub StudioDrawModel: Option<unsafe extern "C" fn(_: libc::c_int)
                                    -> libc::c_int>,
    pub StudioDrawPlayer: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: *mut entity_state_s)
                                     -> libc::c_int>,
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
pub type ref_api_t = ref_api_s;
pub type movevars_t = movevars_s;
pub type GLenum = uint;
pub type GLboolean = byte;
pub type GLvoid = ();
pub type GLint = libc::c_int;
pub type GLubyte = byte;
pub type GLuint = uint;
pub type GLsizei = libc::c_int;
pub type GLfloat = libc::c_float;
pub type GLclampf = libc::c_float;
pub type GLclampd = libc::c_double;
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
pub struct studio_draw_state_t {
    pub time: libc::c_double,
    pub frametime: libc::c_double,
    pub framecount: libc::c_int,
    pub interpolate: qboolean,
    pub rendermode: libc::c_int,
    pub blend: libc::c_float,
    pub rotationmatrix: matrix3x4,
    pub bonestransform: [matrix3x4; 128],
    pub lighttransform: [matrix3x4; 128],
    pub worldtransform: [matrix3x4; 128],
    pub cached_bonestransform: [matrix3x4; 128],
    pub cached_lighttransform: [matrix3x4; 128],
    pub cached_bonenames: [[libc::c_char; 32]; 128],
    pub cached_numbones: libc::c_int,
    pub meshes: [sortedmesh_t; 256],
    pub verts: [vec3_t; 16384],
    pub norms: [vec3_t; 16384],
    pub ambientlight: libc::c_float,
    pub shadelight: libc::c_float,
    pub lightvec: vec3_t,
    pub lightspot: vec3_t,
    pub lightcolor: vec3_t,
    pub blightvec: [vec3_t; 128],
    pub lightvalues: [vec3_t; 16384],
    pub chrome_origin: vec3_t,
    pub chrome: [vec2_t; 16384],
    pub chromeright: [vec3_t; 128],
    pub chromeup: [vec3_t; 128],
    pub chromeage: [libc::c_int; 128],
    pub normaltable: [libc::c_int; 16384],
    pub numlocallights: libc::c_int,
    pub lightage: [libc::c_int; 128],
    pub locallight: [*mut dlight_t; 4],
    pub locallightcolor: [color24; 4],
    pub lightpos: [[vec4_t; 4]; 16384],
    pub lightbonepos: [[vec3_t; 4]; 128],
    pub locallightR2: [libc::c_float; 4],
    pub player_models: [player_model_t; 32],
    pub arrayverts: [vec3_t; 16384],
    pub arraycoord: [vec2_t; 16384],
    pub arrayelems: [libc::c_ushort; 98304],
    pub arraycolor: [[GLubyte; 4]; 16384],
    pub numverts: uint,
    pub numelems: uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct player_model_t {
    pub name: [libc::c_char; 260],
    pub modelname: [libc::c_char; 260],
    pub model: *mut model_t,
}
pub type sortedmesh_t = sortedmesh_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sortedmesh_s {
    pub mesh: *mut mstudiomesh_t,
    pub flags: libc::c_int,
}
pub type r_studio_interface_t = r_studio_interface_s;
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
pub type pmtrace_t = pmtrace_s;
pub type engine_studio_api_t = engine_studio_api_s;
#[inline(always)]
unsafe extern "C" fn __tg_cos(mut __x: libc::c_double) -> libc::c_double {
    return cos(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_sin(mut __x: libc::c_double) -> libc::c_double {
    return sin(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_sqrt(mut __x: libc::c_float) -> libc::c_float {
    return sqrtf(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_atan2(mut __x: libc::c_float,
                                mut __y: libc::c_float) -> libc::c_float {
    return atan2f(__x, __y);
}
#[no_mangle]
pub static mut r_glowshellfreq: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_shadows: cvar_t =
    {
        let mut init =
            cvar_s{name:
                       b"r_shadows\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   string:
                       b"0\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   flags: 0 as libc::c_int,
                   value: 0.,
                   next: 0 as *const cvar_s as *mut cvar_s,};
        init
    };
static mut hullcolor: [vec3_t; 8] =
    [[1.0f32, 1.0f32, 1.0f32], [1.0f32, 0.5f32, 0.5f32],
     [0.5f32, 1.0f32, 0.5f32], [1.0f32, 1.0f32, 0.5f32],
     [0.5f32, 0.5f32, 1.0f32], [1.0f32, 0.5f32, 1.0f32],
     [0.5f32, 1.0f32, 1.0f32], [1.0f32, 1.0f32, 1.0f32]];
// face flags
// studio-related cvars
static mut r_studio_sort_textures: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
static mut r_drawviewmodel: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_righthand: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
static mut cl_himodels: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
static mut r_studio_drawelements: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
static mut pStudioDraw: *mut r_studio_interface_t =
    0 as *const r_studio_interface_t as *mut r_studio_interface_t;
static mut g_studio: studio_draw_state_t =
    studio_draw_state_t{time: 0.,
                        frametime: 0.,
                        framecount: 0,
                        interpolate: false_0,
                        rendermode: 0,
                        blend: 0.,
                        rotationmatrix: [[0.; 4]; 3],
                        bonestransform: [[[0.; 4]; 3]; 128],
                        lighttransform: [[[0.; 4]; 3]; 128],
                        worldtransform: [[[0.; 4]; 3]; 128],
                        cached_bonestransform: [[[0.; 4]; 3]; 128],
                        cached_lighttransform: [[[0.; 4]; 3]; 128],
                        cached_bonenames: [[0; 32]; 128],
                        cached_numbones: 0,
                        meshes:
                            [sortedmesh_t{mesh:
                                              0 as *const mstudiomesh_t as
                                                  *mut mstudiomesh_t,
                                          flags: 0,}; 256],
                        verts: [[0.; 3]; 16384],
                        norms: [[0.; 3]; 16384],
                        ambientlight: 0.,
                        shadelight: 0.,
                        lightvec: [0.; 3],
                        lightspot: [0.; 3],
                        lightcolor: [0.; 3],
                        blightvec: [[0.; 3]; 128],
                        lightvalues: [[0.; 3]; 16384],
                        chrome_origin: [0.; 3],
                        chrome: [[0.; 2]; 16384],
                        chromeright: [[0.; 3]; 128],
                        chromeup: [[0.; 3]; 128],
                        chromeage: [0; 128],
                        normaltable: [0; 16384],
                        numlocallights: 0,
                        lightage: [0; 128],
                        locallight:
                            [0 as *const dlight_t as *mut dlight_t; 4],
                        locallightcolor: [color24{r: 0, g: 0, b: 0,}; 4],
                        lightpos: [[[0.; 4]; 4]; 16384],
                        lightbonepos: [[[0.; 3]; 4]; 128],
                        locallightR2: [0.; 4],
                        player_models:
                            [player_model_t{name: [0; 260],
                                            modelname: [0; 260],
                                            model:
                                                0 as *const model_t as
                                                    *mut model_t,}; 32],
                        arrayverts: [[0.; 3]; 16384],
                        arraycoord: [[0.; 2]; 16384],
                        arrayelems: [0; 98304],
                        arraycolor: [[0; 4]; 16384],
                        numverts: 0,
                        numelems: 0,};
// global studio state
// global variables
static mut m_fDoRemap: qboolean = false_0;
#[no_mangle]
pub static mut m_pSubModel: *mut mstudiomodel_t =
    0 as *const mstudiomodel_t as *mut mstudiomodel_t;
#[no_mangle]
pub static mut m_pBodyPart: *mut mstudiobodyparts_t =
    0 as *const mstudiobodyparts_t as *mut mstudiobodyparts_t;
#[no_mangle]
pub static mut m_pPlayerInfo: *mut player_info_t =
    0 as *const player_info_t as *mut player_info_t;
#[no_mangle]
pub static mut m_pStudioHeader: *mut studiohdr_t =
    0 as *const studiohdr_t as *mut studiohdr_t;
#[no_mangle]
pub static mut m_flGaitMovement: libc::c_float = 0.;
#[no_mangle]
pub static mut g_iBackFaceCull: libc::c_int = 0;
#[no_mangle]
pub static mut g_nTopColor: libc::c_int = 0;
#[no_mangle]
pub static mut g_nBottomColor: libc::c_int = 0;
// remap colors
#[no_mangle]
pub static mut g_nFaceFlags: libc::c_int = 0;
#[no_mangle]
pub static mut g_nForceFaceFlags: libc::c_int = 0;
/*
====================
R_StudioInit

====================
*/
#[no_mangle]
pub unsafe extern "C" fn R_StudioInit() {
    // guaranteed to exist by engine
    cl_himodels =
        gEngfuncs.pfnGetCvarPointer.expect("non-null function pointer")(b"cl_himodels\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char,
                                                                        0 as
                                                                            libc::c_int);
    r_studio_sort_textures =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"r_studio_sort_textures\x00"
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
                                                                   0 as
                                                                       libc::c_int,
                                                               b"change draw order for additive meshes\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    r_drawviewmodel =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"r_drawviewmodel\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"1\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               0 as
                                                                   libc::c_int,
                                                               b"draw firstperson weapon model\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    r_studio_drawelements =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"r_studio_drawelements\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"1\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   0 as
                                                                       libc::c_int,
                                                               b"use glDrawElements for studiomodels\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    memcpy(g_studio.rotationmatrix.as_mut_ptr() as *mut libc::c_void,
           matrix3x4_identity.as_ptr() as *const libc::c_void,
           ::std::mem::size_of::<matrix3x4>() as libc::c_ulong);
    r_glowshellfreq =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"r_glowshellfreq\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"2.2\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               0 as
                                                                   libc::c_int,
                                                               b"glowing shell frequency update\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    // g-cont. cvar disabled by Valve
//	gEngfuncs.Cvar_RegisterVariable( &r_shadows );
    g_studio.interpolate = true_0;
    g_studio.framecount = 0 as libc::c_int;
    m_fDoRemap = false_0;
}
/*
================
R_StudioSetupTimings

init current time for a given model
================
*/
unsafe extern "C" fn R_StudioSetupTimings() {
    if RI.drawWorld as u64 != 0 {
        // synchronize with server time
        g_studio.time = (*gpGlobals).time as libc::c_double;
        g_studio.frametime =
            ((*gpGlobals).time - (*gpGlobals).oldtime) as libc::c_double
    } else {
        // menu stuff
        g_studio.time = (*gpGlobals).realtime;
        g_studio.frametime = (*gpGlobals).frametime
    };
}
/*
================
R_AllowFlipViewModel

should a flip the viewmodel if cl_righthand is set to 1
================
*/
unsafe extern "C" fn R_AllowFlipViewModel(mut e: *mut cl_entity_t)
 -> qboolean {
    if !cl_righthand.is_null() &&
           (*cl_righthand).value > 0 as libc::c_int as libc::c_float {
        if e == gEngfuncs.GetViewModel.expect("non-null function pointer")() {
            return true_0
        }
    }
    return false_0;
}
/*
================
R_StudioComputeBBox

Compute a full bounding box for current sequence
================
*/
unsafe extern "C" fn R_StudioComputeBBox(mut bbox: *mut vec3_t) -> qboolean {
    let mut studio_mins: vec3_t = [0.; 3];
    let mut studio_maxs: vec3_t = [0.; 3];
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut p1: vec3_t = [0.; 3];
    let mut p2: vec3_t = [0.; 3];
    let mut e: *mut cl_entity_t = RI.currententity;
    let mut pseqdesc: *mut mstudioseqdesc_t = 0 as *mut mstudioseqdesc_t;
    let mut i: libc::c_int = 0;
    if m_pStudioHeader.is_null() { return false_0 }
    // check if we have valid mins\maxs
    if !(vec3_origin[0 as libc::c_int as usize] ==
             (*RI.currentmodel).mins[0 as libc::c_int as usize] &&
             vec3_origin[1 as libc::c_int as usize] ==
                 (*RI.currentmodel).mins[1 as libc::c_int as usize] &&
             vec3_origin[2 as libc::c_int as usize] ==
                 (*RI.currentmodel).mins[2 as libc::c_int as usize]) {
        // clipping bounding box
        mins[0 as libc::c_int as usize] =
            (*RI.currentmodel).mins[0 as libc::c_int as usize];
        mins[1 as libc::c_int as usize] =
            (*RI.currentmodel).mins[1 as libc::c_int as usize];
        mins[2 as libc::c_int as usize] =
            (*RI.currentmodel).mins[2 as libc::c_int as usize];
        maxs[0 as libc::c_int as usize] =
            (*RI.currentmodel).maxs[0 as libc::c_int as usize];
        maxs[1 as libc::c_int as usize] =
            (*RI.currentmodel).maxs[1 as libc::c_int as usize];
        maxs[2 as libc::c_int as usize] =
            (*RI.currentmodel).maxs[2 as libc::c_int as usize]
    } else { ClearBounds(mins.as_mut_ptr(), maxs.as_mut_ptr()); }
    // check sequence range
    if (*e).curstate.sequence < 0 as libc::c_int ||
           (*e).curstate.sequence >= (*m_pStudioHeader).numseq {
        (*e).curstate.sequence = 0 as libc::c_int
    }
    pseqdesc =
        ((m_pStudioHeader as
              *mut byte).offset((*m_pStudioHeader).seqindex as isize) as
             *mut mstudioseqdesc_t).offset((*e).curstate.sequence as isize);
    // add sequence box to the model box
    AddPointToBounds((*pseqdesc).bbmin.as_mut_ptr() as *const vec_t,
                     mins.as_mut_ptr(), maxs.as_mut_ptr());
    AddPointToBounds((*pseqdesc).bbmax.as_mut_ptr() as *const vec_t,
                     mins.as_mut_ptr(), maxs.as_mut_ptr());
    ClearBounds(studio_mins.as_mut_ptr(), studio_maxs.as_mut_ptr());
    // compute a full bounding box
    i = 0 as libc::c_int; // model culled
    while i < 8 as libc::c_int {
        p1[0 as libc::c_int as usize] =
            if i & 1 as libc::c_int != 0 {
                mins[0 as libc::c_int as usize]
            } else { maxs[0 as libc::c_int as usize] };
        p1[1 as libc::c_int as usize] =
            if i & 2 as libc::c_int != 0 {
                mins[1 as libc::c_int as usize]
            } else { maxs[1 as libc::c_int as usize] };
        p1[2 as libc::c_int as usize] =
            if i & 4 as libc::c_int != 0 {
                mins[2 as libc::c_int as usize]
            } else { maxs[2 as libc::c_int as usize] };
        Matrix3x4_VectorTransform(g_studio.rotationmatrix.as_mut_ptr() as
                                      *const [vec_t; 4],
                                  p1.as_mut_ptr() as *const libc::c_float,
                                  p2.as_mut_ptr());
        AddPointToBounds(p2.as_mut_ptr() as *const vec_t,
                         studio_mins.as_mut_ptr(), studio_maxs.as_mut_ptr());
        if !bbox.is_null() {
            (*bbox.offset(i as isize))[0 as libc::c_int as usize] =
                p2[0 as libc::c_int as usize];
            (*bbox.offset(i as isize))[1 as libc::c_int as usize] =
                p2[1 as libc::c_int as usize];
            (*bbox.offset(i as isize))[2 as libc::c_int as usize] =
                p2[2 as libc::c_int as usize]
        }
        i += 1
    }
    if bbox.is_null() &&
           R_CullModel(e, studio_mins.as_mut_ptr() as *const vec_t,
                       studio_maxs.as_mut_ptr() as *const vec_t) != 0 {
        return false_0
    }
    return true_0;
    // visible
}
#[no_mangle]
pub unsafe extern "C" fn R_StudioComputeSkinMatrix(mut boneweights:
                                                       *mut mstudioboneweight_t,
                                                   mut result:
                                                       *mut [vec_t; 4]) {
    let mut flWeight0: libc::c_float = 0.; // compensate rounding error
    let mut flWeight1: libc::c_float = 0.; // compensate rounding error
    let mut flWeight2: libc::c_float = 0.; // compensate rounding error
    let mut flWeight3: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    let mut numbones: libc::c_int = 0 as libc::c_int;
    let mut flTotal: libc::c_float = 0.;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if (*boneweights).bone[i as usize] as libc::c_int !=
               -(1 as libc::c_int) {
            numbones += 1
        }
        i += 1
    }
    if numbones == 4 as libc::c_int {
        let mut boneMat0: *mut vec4_t =
            g_studio.worldtransform[(*boneweights).bone[0 as libc::c_int as
                                                            usize] as
                                        usize].as_mut_ptr() as *mut vec4_t;
        let mut boneMat1: *mut vec4_t =
            g_studio.worldtransform[(*boneweights).bone[1 as libc::c_int as
                                                            usize] as
                                        usize].as_mut_ptr() as *mut vec4_t;
        let mut boneMat2: *mut vec4_t =
            g_studio.worldtransform[(*boneweights).bone[2 as libc::c_int as
                                                            usize] as
                                        usize].as_mut_ptr() as *mut vec4_t;
        let mut boneMat3: *mut vec4_t =
            g_studio.worldtransform[(*boneweights).bone[3 as libc::c_int as
                                                            usize] as
                                        usize].as_mut_ptr() as *mut vec4_t;
        flWeight0 =
            (*boneweights).weight[0 as libc::c_int as usize] as libc::c_int as
                libc::c_float / 255.0f32;
        flWeight1 =
            (*boneweights).weight[1 as libc::c_int as usize] as libc::c_int as
                libc::c_float / 255.0f32;
        flWeight2 =
            (*boneweights).weight[2 as libc::c_int as usize] as libc::c_int as
                libc::c_float / 255.0f32;
        flWeight3 =
            (*boneweights).weight[3 as libc::c_int as usize] as libc::c_int as
                libc::c_float / 255.0f32;
        flTotal = flWeight0 + flWeight1 + flWeight2 + flWeight3;
        if flTotal < 1.0f32 { flWeight0 += 1.0f32 - flTotal }
        (*result.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
            =
            (*boneMat0.offset(0 as libc::c_int as
                                  isize))[0 as libc::c_int as usize] *
                flWeight0 +
                (*boneMat1.offset(0 as libc::c_int as
                                      isize))[0 as libc::c_int as usize] *
                    flWeight1 +
                (*boneMat2.offset(0 as libc::c_int as
                                      isize))[0 as libc::c_int as usize] *
                    flWeight2 +
                (*boneMat3.offset(0 as libc::c_int as
                                      isize))[0 as libc::c_int as usize] *
                    flWeight3;
        (*result.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
            =
            (*boneMat0.offset(0 as libc::c_int as
                                  isize))[1 as libc::c_int as usize] *
                flWeight0 +
                (*boneMat1.offset(0 as libc::c_int as
                                      isize))[1 as libc::c_int as usize] *
                    flWeight1 +
                (*boneMat2.offset(0 as libc::c_int as
                                      isize))[1 as libc::c_int as usize] *
                    flWeight2 +
                (*boneMat3.offset(0 as libc::c_int as
                                      isize))[1 as libc::c_int as usize] *
                    flWeight3;
        (*result.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]
            =
            (*boneMat0.offset(0 as libc::c_int as
                                  isize))[2 as libc::c_int as usize] *
                flWeight0 +
                (*boneMat1.offset(0 as libc::c_int as
                                      isize))[2 as libc::c_int as usize] *
                    flWeight1 +
                (*boneMat2.offset(0 as libc::c_int as
                                      isize))[2 as libc::c_int as usize] *
                    flWeight2 +
                (*boneMat3.offset(0 as libc::c_int as
                                      isize))[2 as libc::c_int as usize] *
                    flWeight3;
        (*result.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize]
            =
            (*boneMat0.offset(0 as libc::c_int as
                                  isize))[3 as libc::c_int as usize] *
                flWeight0 +
                (*boneMat1.offset(0 as libc::c_int as
                                      isize))[3 as libc::c_int as usize] *
                    flWeight1 +
                (*boneMat2.offset(0 as libc::c_int as
                                      isize))[3 as libc::c_int as usize] *
                    flWeight2 +
                (*boneMat3.offset(0 as libc::c_int as
                                      isize))[3 as libc::c_int as usize] *
                    flWeight3;
        (*result.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize]
            =
            (*boneMat0.offset(1 as libc::c_int as
                                  isize))[0 as libc::c_int as usize] *
                flWeight0 +
                (*boneMat1.offset(1 as libc::c_int as
                                      isize))[0 as libc::c_int as usize] *
                    flWeight1 +
                (*boneMat2.offset(1 as libc::c_int as
                                      isize))[0 as libc::c_int as usize] *
                    flWeight2 +
                (*boneMat3.offset(1 as libc::c_int as
                                      isize))[0 as libc::c_int as usize] *
                    flWeight3;
        (*result.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize]
            =
            (*boneMat0.offset(1 as libc::c_int as
                                  isize))[1 as libc::c_int as usize] *
                flWeight0 +
                (*boneMat1.offset(1 as libc::c_int as
                                      isize))[1 as libc::c_int as usize] *
                    flWeight1 +
                (*boneMat2.offset(1 as libc::c_int as
                                      isize))[1 as libc::c_int as usize] *
                    flWeight2 +
                (*boneMat3.offset(1 as libc::c_int as
                                      isize))[1 as libc::c_int as usize] *
                    flWeight3;
        (*result.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize]
            =
            (*boneMat0.offset(1 as libc::c_int as
                                  isize))[2 as libc::c_int as usize] *
                flWeight0 +
                (*boneMat1.offset(1 as libc::c_int as
                                      isize))[2 as libc::c_int as usize] *
                    flWeight1 +
                (*boneMat2.offset(1 as libc::c_int as
                                      isize))[2 as libc::c_int as usize] *
                    flWeight2 +
                (*boneMat3.offset(1 as libc::c_int as
                                      isize))[2 as libc::c_int as usize] *
                    flWeight3;
        (*result.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize]
            =
            (*boneMat0.offset(1 as libc::c_int as
                                  isize))[3 as libc::c_int as usize] *
                flWeight0 +
                (*boneMat1.offset(1 as libc::c_int as
                                      isize))[3 as libc::c_int as usize] *
                    flWeight1 +
                (*boneMat2.offset(1 as libc::c_int as
                                      isize))[3 as libc::c_int as usize] *
                    flWeight2 +
                (*boneMat3.offset(1 as libc::c_int as
                                      isize))[3 as libc::c_int as usize] *
                    flWeight3;
        (*result.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize]
            =
            (*boneMat0.offset(2 as libc::c_int as
                                  isize))[0 as libc::c_int as usize] *
                flWeight0 +
                (*boneMat1.offset(2 as libc::c_int as
                                      isize))[0 as libc::c_int as usize] *
                    flWeight1 +
                (*boneMat2.offset(2 as libc::c_int as
                                      isize))[0 as libc::c_int as usize] *
                    flWeight2 +
                (*boneMat3.offset(2 as libc::c_int as
                                      isize))[0 as libc::c_int as usize] *
                    flWeight3;
        (*result.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize]
            =
            (*boneMat0.offset(2 as libc::c_int as
                                  isize))[1 as libc::c_int as usize] *
                flWeight0 +
                (*boneMat1.offset(2 as libc::c_int as
                                      isize))[1 as libc::c_int as usize] *
                    flWeight1 +
                (*boneMat2.offset(2 as libc::c_int as
                                      isize))[1 as libc::c_int as usize] *
                    flWeight2 +
                (*boneMat3.offset(2 as libc::c_int as
                                      isize))[1 as libc::c_int as usize] *
                    flWeight3;
        (*result.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize]
            =
            (*boneMat0.offset(2 as libc::c_int as
                                  isize))[2 as libc::c_int as usize] *
                flWeight0 +
                (*boneMat1.offset(2 as libc::c_int as
                                      isize))[2 as libc::c_int as usize] *
                    flWeight1 +
                (*boneMat2.offset(2 as libc::c_int as
                                      isize))[2 as libc::c_int as usize] *
                    flWeight2 +
                (*boneMat3.offset(2 as libc::c_int as
                                      isize))[2 as libc::c_int as usize] *
                    flWeight3;
        (*result.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize]
            =
            (*boneMat0.offset(2 as libc::c_int as
                                  isize))[3 as libc::c_int as usize] *
                flWeight0 +
                (*boneMat1.offset(2 as libc::c_int as
                                      isize))[3 as libc::c_int as usize] *
                    flWeight1 +
                (*boneMat2.offset(2 as libc::c_int as
                                      isize))[3 as libc::c_int as usize] *
                    flWeight2 +
                (*boneMat3.offset(2 as libc::c_int as
                                      isize))[3 as libc::c_int as usize] *
                    flWeight3
    } else if numbones == 3 as libc::c_int {
        let mut boneMat0_0: *mut vec4_t =
            g_studio.worldtransform[(*boneweights).bone[0 as libc::c_int as
                                                            usize] as
                                        usize].as_mut_ptr() as *mut vec4_t;
        let mut boneMat1_0: *mut vec4_t =
            g_studio.worldtransform[(*boneweights).bone[1 as libc::c_int as
                                                            usize] as
                                        usize].as_mut_ptr() as *mut vec4_t;
        let mut boneMat2_0: *mut vec4_t =
            g_studio.worldtransform[(*boneweights).bone[2 as libc::c_int as
                                                            usize] as
                                        usize].as_mut_ptr() as *mut vec4_t;
        flWeight0 =
            (*boneweights).weight[0 as libc::c_int as usize] as libc::c_int as
                libc::c_float / 255.0f32;
        flWeight1 =
            (*boneweights).weight[1 as libc::c_int as usize] as libc::c_int as
                libc::c_float / 255.0f32;
        flWeight2 =
            (*boneweights).weight[2 as libc::c_int as usize] as libc::c_int as
                libc::c_float / 255.0f32;
        flTotal = flWeight0 + flWeight1 + flWeight2;
        if flTotal < 1.0f32 { flWeight0 += 1.0f32 - flTotal }
        (*result.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
            =
            (*boneMat0_0.offset(0 as libc::c_int as
                                    isize))[0 as libc::c_int as usize] *
                flWeight0 +
                (*boneMat1_0.offset(0 as libc::c_int as
                                        isize))[0 as libc::c_int as usize] *
                    flWeight1 +
                (*boneMat2_0.offset(0 as libc::c_int as
                                        isize))[0 as libc::c_int as usize] *
                    flWeight2;
        (*result.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
            =
            (*boneMat0_0.offset(0 as libc::c_int as
                                    isize))[1 as libc::c_int as usize] *
                flWeight0 +
                (*boneMat1_0.offset(0 as libc::c_int as
                                        isize))[1 as libc::c_int as usize] *
                    flWeight1 +
                (*boneMat2_0.offset(0 as libc::c_int as
                                        isize))[1 as libc::c_int as usize] *
                    flWeight2;
        (*result.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]
            =
            (*boneMat0_0.offset(0 as libc::c_int as
                                    isize))[2 as libc::c_int as usize] *
                flWeight0 +
                (*boneMat1_0.offset(0 as libc::c_int as
                                        isize))[2 as libc::c_int as usize] *
                    flWeight1 +
                (*boneMat2_0.offset(0 as libc::c_int as
                                        isize))[2 as libc::c_int as usize] *
                    flWeight2;
        (*result.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize]
            =
            (*boneMat0_0.offset(0 as libc::c_int as
                                    isize))[3 as libc::c_int as usize] *
                flWeight0 +
                (*boneMat1_0.offset(0 as libc::c_int as
                                        isize))[3 as libc::c_int as usize] *
                    flWeight1 +
                (*boneMat2_0.offset(0 as libc::c_int as
                                        isize))[3 as libc::c_int as usize] *
                    flWeight2;
        (*result.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize]
            =
            (*boneMat0_0.offset(1 as libc::c_int as
                                    isize))[0 as libc::c_int as usize] *
                flWeight0 +
                (*boneMat1_0.offset(1 as libc::c_int as
                                        isize))[0 as libc::c_int as usize] *
                    flWeight1 +
                (*boneMat2_0.offset(1 as libc::c_int as
                                        isize))[0 as libc::c_int as usize] *
                    flWeight2;
        (*result.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize]
            =
            (*boneMat0_0.offset(1 as libc::c_int as
                                    isize))[1 as libc::c_int as usize] *
                flWeight0 +
                (*boneMat1_0.offset(1 as libc::c_int as
                                        isize))[1 as libc::c_int as usize] *
                    flWeight1 +
                (*boneMat2_0.offset(1 as libc::c_int as
                                        isize))[1 as libc::c_int as usize] *
                    flWeight2;
        (*result.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize]
            =
            (*boneMat0_0.offset(1 as libc::c_int as
                                    isize))[2 as libc::c_int as usize] *
                flWeight0 +
                (*boneMat1_0.offset(1 as libc::c_int as
                                        isize))[2 as libc::c_int as usize] *
                    flWeight1 +
                (*boneMat2_0.offset(1 as libc::c_int as
                                        isize))[2 as libc::c_int as usize] *
                    flWeight2;
        (*result.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize]
            =
            (*boneMat0_0.offset(1 as libc::c_int as
                                    isize))[3 as libc::c_int as usize] *
                flWeight0 +
                (*boneMat1_0.offset(1 as libc::c_int as
                                        isize))[3 as libc::c_int as usize] *
                    flWeight1 +
                (*boneMat2_0.offset(1 as libc::c_int as
                                        isize))[3 as libc::c_int as usize] *
                    flWeight2;
        (*result.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize]
            =
            (*boneMat0_0.offset(2 as libc::c_int as
                                    isize))[0 as libc::c_int as usize] *
                flWeight0 +
                (*boneMat1_0.offset(2 as libc::c_int as
                                        isize))[0 as libc::c_int as usize] *
                    flWeight1 +
                (*boneMat2_0.offset(2 as libc::c_int as
                                        isize))[0 as libc::c_int as usize] *
                    flWeight2;
        (*result.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize]
            =
            (*boneMat0_0.offset(2 as libc::c_int as
                                    isize))[1 as libc::c_int as usize] *
                flWeight0 +
                (*boneMat1_0.offset(2 as libc::c_int as
                                        isize))[1 as libc::c_int as usize] *
                    flWeight1 +
                (*boneMat2_0.offset(2 as libc::c_int as
                                        isize))[1 as libc::c_int as usize] *
                    flWeight2;
        (*result.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize]
            =
            (*boneMat0_0.offset(2 as libc::c_int as
                                    isize))[2 as libc::c_int as usize] *
                flWeight0 +
                (*boneMat1_0.offset(2 as libc::c_int as
                                        isize))[2 as libc::c_int as usize] *
                    flWeight1 +
                (*boneMat2_0.offset(2 as libc::c_int as
                                        isize))[2 as libc::c_int as usize] *
                    flWeight2;
        (*result.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize]
            =
            (*boneMat0_0.offset(2 as libc::c_int as
                                    isize))[3 as libc::c_int as usize] *
                flWeight0 +
                (*boneMat1_0.offset(2 as libc::c_int as
                                        isize))[3 as libc::c_int as usize] *
                    flWeight1 +
                (*boneMat2_0.offset(2 as libc::c_int as
                                        isize))[3 as libc::c_int as usize] *
                    flWeight2
    } else if numbones == 2 as libc::c_int {
        let mut boneMat0_1: *mut vec4_t =
            g_studio.worldtransform[(*boneweights).bone[0 as libc::c_int as
                                                            usize] as
                                        usize].as_mut_ptr() as *mut vec4_t;
        let mut boneMat1_1: *mut vec4_t =
            g_studio.worldtransform[(*boneweights).bone[1 as libc::c_int as
                                                            usize] as
                                        usize].as_mut_ptr() as *mut vec4_t;
        flWeight0 =
            (*boneweights).weight[0 as libc::c_int as usize] as libc::c_int as
                libc::c_float / 255.0f32;
        flWeight1 =
            (*boneweights).weight[1 as libc::c_int as usize] as libc::c_int as
                libc::c_float / 255.0f32;
        flTotal = flWeight0 + flWeight1;
        if flTotal < 1.0f32 { flWeight0 += 1.0f32 - flTotal }
        (*result.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
            =
            (*boneMat0_1.offset(0 as libc::c_int as
                                    isize))[0 as libc::c_int as usize] *
                flWeight0 +
                (*boneMat1_1.offset(0 as libc::c_int as
                                        isize))[0 as libc::c_int as usize] *
                    flWeight1;
        (*result.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
            =
            (*boneMat0_1.offset(0 as libc::c_int as
                                    isize))[1 as libc::c_int as usize] *
                flWeight0 +
                (*boneMat1_1.offset(0 as libc::c_int as
                                        isize))[1 as libc::c_int as usize] *
                    flWeight1;
        (*result.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]
            =
            (*boneMat0_1.offset(0 as libc::c_int as
                                    isize))[2 as libc::c_int as usize] *
                flWeight0 +
                (*boneMat1_1.offset(0 as libc::c_int as
                                        isize))[2 as libc::c_int as usize] *
                    flWeight1;
        (*result.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize]
            =
            (*boneMat0_1.offset(0 as libc::c_int as
                                    isize))[3 as libc::c_int as usize] *
                flWeight0 +
                (*boneMat1_1.offset(0 as libc::c_int as
                                        isize))[3 as libc::c_int as usize] *
                    flWeight1;
        (*result.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize]
            =
            (*boneMat0_1.offset(1 as libc::c_int as
                                    isize))[0 as libc::c_int as usize] *
                flWeight0 +
                (*boneMat1_1.offset(1 as libc::c_int as
                                        isize))[0 as libc::c_int as usize] *
                    flWeight1;
        (*result.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize]
            =
            (*boneMat0_1.offset(1 as libc::c_int as
                                    isize))[1 as libc::c_int as usize] *
                flWeight0 +
                (*boneMat1_1.offset(1 as libc::c_int as
                                        isize))[1 as libc::c_int as usize] *
                    flWeight1;
        (*result.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize]
            =
            (*boneMat0_1.offset(1 as libc::c_int as
                                    isize))[2 as libc::c_int as usize] *
                flWeight0 +
                (*boneMat1_1.offset(1 as libc::c_int as
                                        isize))[2 as libc::c_int as usize] *
                    flWeight1;
        (*result.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize]
            =
            (*boneMat0_1.offset(1 as libc::c_int as
                                    isize))[3 as libc::c_int as usize] *
                flWeight0 +
                (*boneMat1_1.offset(1 as libc::c_int as
                                        isize))[3 as libc::c_int as usize] *
                    flWeight1;
        (*result.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize]
            =
            (*boneMat0_1.offset(2 as libc::c_int as
                                    isize))[0 as libc::c_int as usize] *
                flWeight0 +
                (*boneMat1_1.offset(2 as libc::c_int as
                                        isize))[0 as libc::c_int as usize] *
                    flWeight1;
        (*result.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize]
            =
            (*boneMat0_1.offset(2 as libc::c_int as
                                    isize))[1 as libc::c_int as usize] *
                flWeight0 +
                (*boneMat1_1.offset(2 as libc::c_int as
                                        isize))[1 as libc::c_int as usize] *
                    flWeight1;
        (*result.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize]
            =
            (*boneMat0_1.offset(2 as libc::c_int as
                                    isize))[2 as libc::c_int as usize] *
                flWeight0 +
                (*boneMat1_1.offset(2 as libc::c_int as
                                        isize))[2 as libc::c_int as usize] *
                    flWeight1;
        (*result.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize]
            =
            (*boneMat0_1.offset(2 as libc::c_int as
                                    isize))[3 as libc::c_int as usize] *
                flWeight0 +
                (*boneMat1_1.offset(2 as libc::c_int as
                                        isize))[3 as libc::c_int as usize] *
                    flWeight1
    } else {
        memcpy(result as *mut libc::c_void,
               g_studio.worldtransform[(*boneweights).bone[0 as libc::c_int as
                                                               usize] as
                                           usize].as_mut_ptr() as
                   *const libc::c_void,
               ::std::mem::size_of::<matrix3x4>() as libc::c_ulong);
    };
}
/*
===============
pfnGetCurrentEntity

===============
*/
unsafe extern "C" fn pfnGetCurrentEntity() -> *mut cl_entity_t {
    return RI.currententity;
}
/*
===============
pfnPlayerInfo

===============
*/
#[no_mangle]
pub unsafe extern "C" fn pfnPlayerInfo(mut index: libc::c_int)
 -> *mut player_info_t {
    if RI.drawWorld as u64 == 0 { index = -(1 as libc::c_int) }
    return gEngfuncs.pfnPlayerInfo.expect("non-null function pointer")(index);
}
/*
===============
pfnMod_ForName

===============
*/
unsafe extern "C" fn pfnMod_ForName(mut model: *const libc::c_char,
                                    mut crash: libc::c_int) -> *mut model_t {
    return gEngfuncs.Mod_ForName.expect("non-null function pointer")(model,
                                                                     crash as
                                                                         qboolean,
                                                                     false_0);
}
/*
===============
pfnGetPlayerState

===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_StudioGetPlayerState(mut index: libc::c_int)
 -> *mut entity_state_t {
    if RI.drawWorld as u64 == 0 { return &mut (*RI.currententity).curstate }
    return gEngfuncs.pfnGetPlayerState.expect("non-null function pointer")(index);
}
/*
===============
pfnGetViewEntity

===============
*/
unsafe extern "C" fn pfnGetViewEntity() -> *mut cl_entity_t {
    return gEngfuncs.GetViewModel.expect("non-null function pointer")();
}
/*
===============
pfnGetEngineTimes

===============
*/
unsafe extern "C" fn pfnGetEngineTimes(mut framecount: *mut libc::c_int,
                                       mut current: *mut libc::c_double,
                                       mut old: *mut libc::c_double) {
    if !framecount.is_null() { *framecount = tr.realframecount }
    if !current.is_null() { *current = (*gpGlobals).time as libc::c_double }
    if !old.is_null() { *old = (*gpGlobals).oldtime as libc::c_double };
}
/*
===============
pfnGetViewInfo

===============
*/
unsafe extern "C" fn pfnGetViewInfo(mut origin: *mut libc::c_float,
                                    mut upv: *mut libc::c_float,
                                    mut rightv: *mut libc::c_float,
                                    mut forwardv: *mut libc::c_float) {
    if !origin.is_null() {
        *origin.offset(0 as libc::c_int as isize) =
            RI.vieworg[0 as libc::c_int as usize];
        *origin.offset(1 as libc::c_int as isize) =
            RI.vieworg[1 as libc::c_int as usize];
        *origin.offset(2 as libc::c_int as isize) =
            RI.vieworg[2 as libc::c_int as usize]
    }
    if !forwardv.is_null() {
        *forwardv.offset(0 as libc::c_int as isize) =
            RI.vforward[0 as libc::c_int as usize];
        *forwardv.offset(1 as libc::c_int as isize) =
            RI.vforward[1 as libc::c_int as usize];
        *forwardv.offset(2 as libc::c_int as isize) =
            RI.vforward[2 as libc::c_int as usize]
    }
    if !rightv.is_null() {
        *rightv.offset(0 as libc::c_int as isize) =
            RI.vright[0 as libc::c_int as usize];
        *rightv.offset(1 as libc::c_int as isize) =
            RI.vright[1 as libc::c_int as usize];
        *rightv.offset(2 as libc::c_int as isize) =
            RI.vright[2 as libc::c_int as usize]
    }
    if !upv.is_null() {
        *upv.offset(0 as libc::c_int as isize) =
            RI.vup[0 as libc::c_int as usize];
        *upv.offset(1 as libc::c_int as isize) =
            RI.vup[1 as libc::c_int as usize];
        *upv.offset(2 as libc::c_int as isize) =
            RI.vup[2 as libc::c_int as usize]
    };
}
/*
===============
R_GetChromeSprite

===============
*/
unsafe extern "C" fn R_GetChromeSprite() -> *mut model_t {
    return gEngfuncs.GetDefaultSprite.expect("non-null function pointer")(REF_CHROME_SPRITE);
}
/*
===============
pfnGetModelCounters

===============
*/
unsafe extern "C" fn pfnGetModelCounters(mut s: *mut *mut libc::c_int,
                                         mut a: *mut *mut libc::c_int) {
    *s = &mut g_studio.framecount;
    *a = &mut r_stats.c_studio_models_drawn as *mut uint as *mut libc::c_int;
}
/*
===============
pfnGetAliasScale

===============
*/
unsafe extern "C" fn pfnGetAliasScale(mut x: *mut libc::c_float,
                                      mut y: *mut libc::c_float) {
    if !x.is_null() { *x = 1.0f32 }
    if !y.is_null() { *y = 1.0f32 };
}
/*
===============
pfnStudioGetBoneTransform

===============
*/
unsafe extern "C" fn pfnStudioGetBoneTransform()
 -> *mut *mut *mut *mut libc::c_float {
    return g_studio.bonestransform.as_mut_ptr() as
               *mut *mut *mut *mut libc::c_float;
}
/*
===============
pfnStudioGetLightTransform

===============
*/
unsafe extern "C" fn pfnStudioGetLightTransform()
 -> *mut *mut *mut *mut libc::c_float {
    return g_studio.lighttransform.as_mut_ptr() as
               *mut *mut *mut *mut libc::c_float;
}
/*
===============
pfnStudioGetAliasTransform

===============
*/
unsafe extern "C" fn pfnStudioGetAliasTransform()
 -> *mut *mut *mut libc::c_float {
    return 0 as *mut *mut *mut libc::c_float;
}
/*
===============
pfnStudioGetRotationMatrix

===============
*/
unsafe extern "C" fn pfnStudioGetRotationMatrix()
 -> *mut *mut *mut libc::c_float {
    return g_studio.rotationmatrix.as_mut_ptr() as
               *mut *mut *mut libc::c_float;
}
/*
====================
StudioPlayerBlend

====================
*/
#[no_mangle]
pub unsafe extern "C" fn R_StudioPlayerBlend(mut pseqdesc:
                                                 *mut mstudioseqdesc_t,
                                             mut pBlend: *mut libc::c_int,
                                             mut pPitch: *mut libc::c_float) {
    // calc up/down pointing
    *pBlend = (*pPitch * 3.0f32) as libc::c_int;
    if (*pBlend as libc::c_float) <
           (*pseqdesc).blendstart[0 as libc::c_int as usize] {
        *pPitch -= (*pseqdesc).blendstart[0 as libc::c_int as usize] / 3.0f32;
        *pBlend = 0 as libc::c_int
    } else if *pBlend as libc::c_float >
                  (*pseqdesc).blendend[0 as libc::c_int as usize] {
        *pPitch -= (*pseqdesc).blendend[0 as libc::c_int as usize] / 3.0f32;
        *pBlend = 255 as libc::c_int
    } else {
        if (*pseqdesc).blendend[0 as libc::c_int as usize] -
               (*pseqdesc).blendstart[0 as libc::c_int as usize] < 0.1f32 {
            // catch qc error
            *pBlend = 127 as libc::c_int
        } else {
            *pBlend =
                (255 as libc::c_int as libc::c_float *
                     (*pBlend as libc::c_float -
                          (*pseqdesc).blendstart[0 as libc::c_int as usize]) /
                     ((*pseqdesc).blendend[0 as libc::c_int as usize] -
                          (*pseqdesc).blendstart[0 as libc::c_int as usize]))
                    as libc::c_int
        }
        *pPitch = 0.0f32
    };
}
/*
====================
R_StudioLerpMovement

====================
*/
#[no_mangle]
pub unsafe extern "C" fn R_StudioLerpMovement(mut e: *mut cl_entity_t,
                                              mut time: libc::c_double,
                                              mut origin: *mut vec_t,
                                              mut angles: *mut vec_t) {
    let mut f: libc::c_float = 1.0f32;
    // don't do it if the goalstarttime hasn't updated in a while.
	// NOTE: Because we need to interpolate multiplayer characters, the interpolation time limit
	// was increased to 1.0 s., which is 2x the max lag we are accounting for.
    if g_studio.interpolate as libc::c_uint != 0 &&
           time < ((*e).curstate.animtime + 1.0f32) as libc::c_double &&
           (*e).curstate.animtime != (*e).latched.prevanimtime {
        f =
            ((time - (*e).curstate.animtime as libc::c_double) /
                 ((*e).curstate.animtime - (*e).latched.prevanimtime) as
                     libc::c_double) as libc::c_float
    }
    // Con_Printf( "%4.2f %.2f %.2f\n", f, e->curstate.animtime, g_studio.time );
    *origin.offset(0 as libc::c_int as isize) =
        (*e).latched.prevorigin[0 as libc::c_int as usize] +
            f *
                ((*e).curstate.origin[0 as libc::c_int as usize] -
                     (*e).latched.prevorigin[0 as libc::c_int as usize]);
    *origin.offset(1 as libc::c_int as isize) =
        (*e).latched.prevorigin[1 as libc::c_int as usize] +
            f *
                ((*e).curstate.origin[1 as libc::c_int as usize] -
                     (*e).latched.prevorigin[1 as libc::c_int as usize]);
    *origin.offset(2 as libc::c_int as isize) =
        (*e).latched.prevorigin[2 as libc::c_int as usize] +
            f *
                ((*e).curstate.origin[2 as libc::c_int as usize] -
                     (*e).latched.prevorigin[2 as libc::c_int as usize]);
    if VectorCompareEpsilon((*e).curstate.angles.as_mut_ptr() as *const vec_t,
                            (*e).latched.prevangles.as_mut_ptr() as
                                *const vec_t, 0.1f32) as u64 == 0 {
        let mut q: vec4_t = [0.; 4];
        let mut q1: vec4_t = [0.; 4];
        let mut q2: vec4_t = [0.; 4];
        AngleQuaternion((*e).curstate.angles.as_mut_ptr() as *const vec_t,
                        q1.as_mut_ptr(), false_0);
        AngleQuaternion((*e).latched.prevangles.as_mut_ptr() as *const vec_t,
                        q2.as_mut_ptr(), false_0);
        QuaternionSlerp(q2.as_mut_ptr() as *const vec_t,
                        q1.as_mut_ptr() as *const vec_t, f, q.as_mut_ptr());
        QuaternionAngle(q.as_mut_ptr() as *const vec_t, angles);
    } else {
        *angles.offset(0 as libc::c_int as isize) =
            (*e).curstate.angles[0 as libc::c_int as usize];
        *angles.offset(1 as libc::c_int as isize) =
            (*e).curstate.angles[1 as libc::c_int as usize];
        *angles.offset(2 as libc::c_int as isize) =
            (*e).curstate.angles[2 as libc::c_int as usize]
    };
}
/*
====================
StudioSetUpTransform

====================
*/
#[no_mangle]
pub unsafe extern "C" fn R_StudioSetUpTransform(mut e: *mut cl_entity_t) {
    let mut origin: vec3_t = [0.; 3];
    let mut angles: vec3_t = [0.; 3];
    origin[0 as libc::c_int as usize] =
        (*e).origin[0 as libc::c_int as usize];
    origin[1 as libc::c_int as usize] =
        (*e).origin[1 as libc::c_int as usize];
    origin[2 as libc::c_int as usize] =
        (*e).origin[2 as libc::c_int as usize];
    angles[0 as libc::c_int as usize] =
        (*e).angles[0 as libc::c_int as usize];
    angles[1 as libc::c_int as usize] =
        (*e).angles[1 as libc::c_int as usize];
    angles[2 as libc::c_int as usize] =
        (*e).angles[2 as libc::c_int as usize];
    // interpolate monsters position (moved into UpdateEntityFields by user request)
    if (*e).curstate.movetype == 4 as libc::c_int &&
           Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(26
                                                                                                                     as
                                                                                                                     libc::c_int,
                                                                                                                 0
                                                                                                                     as
                                                                                                                     libc::c_int)
               & (1 as libc::c_int) << 7 as libc::c_int == 0 {
        R_StudioLerpMovement(e, g_studio.time, origin.as_mut_ptr(),
                             angles.as_mut_ptr()); // stupid quake bug
    }
    if Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(26
                                                                                                                 as
                                                                                                                 libc::c_int,
                                                                                                             0
                                                                                                                 as
                                                                                                                 libc::c_int)
           & (1 as libc::c_int) << 5 as libc::c_int == 0 {
        angles[0 as libc::c_int as usize] = -angles[0 as libc::c_int as usize]
    }
    // don't rotate clients, only aim
    if (*e).player as u64 != 0 { angles[0 as libc::c_int as usize] = 0.0f32 }
    Matrix3x4_CreateFromEntity(g_studio.rotationmatrix.as_mut_ptr(),
                               angles.as_mut_ptr() as *const vec_t,
                               origin.as_mut_ptr() as *const vec_t, 1.0f32);
    if tr.fFlipViewModel as u64 != 0 {
        g_studio.rotationmatrix[0 as libc::c_int as
                                    usize][1 as libc::c_int as usize] =
            -g_studio.rotationmatrix[0 as libc::c_int as
                                         usize][1 as libc::c_int as usize];
        g_studio.rotationmatrix[1 as libc::c_int as
                                    usize][1 as libc::c_int as usize] =
            -g_studio.rotationmatrix[1 as libc::c_int as
                                         usize][1 as libc::c_int as usize];
        g_studio.rotationmatrix[2 as libc::c_int as
                                    usize][1 as libc::c_int as usize] =
            -g_studio.rotationmatrix[2 as libc::c_int as
                                         usize][1 as libc::c_int as usize]
    };
}
/*
====================
StudioEstimateFrame

====================
*/
#[no_mangle]
pub unsafe extern "C" fn R_StudioEstimateFrame(mut e: *mut cl_entity_t,
                                               mut pseqdesc:
                                                   *mut mstudioseqdesc_t)
 -> libc::c_float {
    let mut dfdt: libc::c_double = 0.;
    let mut f: libc::c_double = 0.;
    if g_studio.interpolate as u64 != 0 {
        if g_studio.time < (*e).curstate.animtime as libc::c_double {
            dfdt = 0.0f64
        } else {
            dfdt =
                (g_studio.time - (*e).curstate.animtime as libc::c_double) *
                    (*e).curstate.framerate as libc::c_double *
                    (*pseqdesc).fps as libc::c_double
        }
    } else { dfdt = 0 as libc::c_int as libc::c_double }
    if (*pseqdesc).numframes <= 1 as libc::c_int {
        f = 0.0f64
    } else {
        f =
            ((*e).curstate.frame *
                 ((*pseqdesc).numframes - 1 as libc::c_int) as libc::c_float /
                 256.0f32) as libc::c_double
    }
    f += dfdt;
    if (*pseqdesc).flags & 0x1 as libc::c_int != 0 {
        if (*pseqdesc).numframes > 1 as libc::c_int {
            f -=
                ((f /
                      ((*pseqdesc).numframes - 1 as libc::c_int) as
                          libc::c_double) as libc::c_int *
                     ((*pseqdesc).numframes - 1 as libc::c_int)) as
                    libc::c_double
        }
        if f < 0 as libc::c_int as libc::c_double {
            f += ((*pseqdesc).numframes - 1 as libc::c_int) as libc::c_double
        }
    } else {
        if f >= (*pseqdesc).numframes as libc::c_double - 1.001f64 {
            f = (*pseqdesc).numframes as libc::c_double - 1.001f64
        }
        if f < 0.0f64 { f = 0.0f64 }
    }
    return f as libc::c_float;
}
/*
====================
StudioEstimateInterpolant

====================
*/
#[no_mangle]
pub unsafe extern "C" fn R_StudioEstimateInterpolant(mut e: *mut cl_entity_t)
 -> libc::c_float {
    let mut dadt: libc::c_float = 1.0f32;
    if g_studio.interpolate as libc::c_uint != 0 &&
           (*e).curstate.animtime >= (*e).latched.prevanimtime + 0.01f32 {
        dadt =
            ((g_studio.time - (*e).curstate.animtime as libc::c_double) /
                 0.1f32 as libc::c_double) as libc::c_float;
        if dadt > 2.0f32 { dadt = 2.0f32 }
    }
    return dadt;
}
/*
====================
CL_GetSequenceDuration

====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_GetSequenceDuration(mut ent: *mut cl_entity_t,
                                                mut sequence: libc::c_int)
 -> libc::c_float {
    let mut pstudiohdr: *mut studiohdr_t = 0 as *mut studiohdr_t;
    let mut pseqdesc: *mut mstudioseqdesc_t = 0 as *mut mstudioseqdesc_t;
    if !(*ent).model.is_null() &&
           (*(*ent).model).type_0 as libc::c_int == mod_studio as libc::c_int
       {
        pstudiohdr =
            gEngfuncs.Mod_Extradata.expect("non-null function pointer")(mod_studio
                                                                            as
                                                                            libc::c_int,
                                                                        (*ent).model)
                as *mut studiohdr_t;
        if !pstudiohdr.is_null() {
            sequence =
                if sequence >= 0 as libc::c_int {
                    if sequence < (*pstudiohdr).numseq - 1 as libc::c_int {
                        sequence
                    } else { ((*pstudiohdr).numseq) - 1 as libc::c_int }
                } else { 0 as libc::c_int };
            pseqdesc =
                ((pstudiohdr as
                      *mut byte).offset((*pstudiohdr).seqindex as isize) as
                     *mut mstudioseqdesc_t).offset(sequence as isize);
            if (*pseqdesc).numframes > 1 as libc::c_int &&
                   (*pseqdesc).fps > 0 as libc::c_int as libc::c_float {
                return (*pseqdesc).numframes as libc::c_float /
                           (*pseqdesc).fps
            }
        }
    }
    return 0.1f32;
}
/*
====================
StudioFxTransform

====================
*/
#[no_mangle]
pub unsafe extern "C" fn R_StudioFxTransform(mut ent: *mut cl_entity_t,
                                             mut transform: *mut [vec_t; 4]) {
    match (*ent).curstate.renderfx {
        15 | 16 => {
            if gEngfuncs.COM_RandomLong.expect("non-null function pointer")(0
                                                                                as
                                                                                libc::c_int,
                                                                            49
                                                                                as
                                                                                libc::c_int)
                   == 0 {
                let mut axis: libc::c_int =
                    gEngfuncs.COM_RandomLong.expect("non-null function pointer")(0
                                                                                     as
                                                                                     libc::c_int,
                                                                                 1
                                                                                     as
                                                                                     libc::c_int); // choose between x & z
                if axis == 1 as libc::c_int {
                    axis = 2 as libc::c_int
                } // choose between x & z
                (*transform.offset(axis as isize))[0 as libc::c_int as usize]
                    =
                    (*transform.offset(axis as
                                           isize))[0 as libc::c_int as usize]
                        *
                        gEngfuncs.COM_RandomFloat.expect("non-null function pointer")(1.0f32,
                                                                                      1.484f32); // don't blow up more than 200%
                (*transform.offset(axis as isize))[1 as libc::c_int as usize]
                    =
                    (*transform.offset(axis as
                                           isize))[1 as libc::c_int as usize]
                        *
                        gEngfuncs.COM_RandomFloat.expect("non-null function pointer")(1.0f32,
                                                                                      1.484f32);
                (*transform.offset(axis as isize))[2 as libc::c_int as usize]
                    =
                    (*transform.offset(axis as
                                           isize))[2 as libc::c_int as usize]
                        *
                        gEngfuncs.COM_RandomFloat.expect("non-null function pointer")(1.0f32,
                                                                                      1.484f32)
            } else if gEngfuncs.COM_RandomLong.expect("non-null function pointer")(0
                                                                                       as
                                                                                       libc::c_int,
                                                                                   49
                                                                                       as
                                                                                       libc::c_int)
                          == 0 {
                let mut offset: libc::c_float = 0.;
                let mut axis_0: libc::c_int =
                    gEngfuncs.COM_RandomLong.expect("non-null function pointer")(0
                                                                                     as
                                                                                     libc::c_int,
                                                                                 1
                                                                                     as
                                                                                     libc::c_int);
                if axis_0 == 1 as libc::c_int { axis_0 = 2 as libc::c_int }
                offset =
                    gEngfuncs.COM_RandomFloat.expect("non-null function pointer")(-10.0f32,
                                                                                  10.0f32);
                let ref mut fresh0 =
                    (*transform.offset(gEngfuncs.COM_RandomLong.expect("non-null function pointer")(0
                                                                                                        as
                                                                                                        libc::c_int,
                                                                                                    2
                                                                                                        as
                                                                                                        libc::c_int)
                                           as
                                           isize))[3 as libc::c_int as usize];
                *fresh0 += offset
            }
        }
        18 => {
            let mut scale: libc::c_float = 0.;
            scale =
                (1.0f32 as libc::c_double +
                     (g_studio.time -
                          (*ent).curstate.animtime as libc::c_double) *
                         10.0f32 as libc::c_double) as libc::c_float;
            if scale > 2.0f32 { scale = 2.0f32 }
            let ref mut fresh1 =
                (*transform.offset(0 as libc::c_int as
                                       isize))[1 as libc::c_int as usize];
            *fresh1 *= scale;
            let ref mut fresh2 =
                (*transform.offset(1 as libc::c_int as
                                       isize))[1 as libc::c_int as usize];
            *fresh2 *= scale;
            let ref mut fresh3 =
                (*transform.offset(2 as libc::c_int as
                                       isize))[1 as libc::c_int as usize];
            *fresh3 *= scale
        }
        _ => { }
    };
}
/*
====================
StudioCalcBoneAdj

====================
*/
#[no_mangle]
pub unsafe extern "C" fn R_StudioCalcBoneAdj(mut dadt: libc::c_float,
                                             mut adj: *mut libc::c_float,
                                             mut pcontroller1: *const byte,
                                             mut pcontroller2: *const byte,
                                             mut mouthopen: byte) {
    let mut pbonecontroller: *mut mstudiobonecontroller_t =
        0 as *mut mstudiobonecontroller_t;
    let mut value: libc::c_float = 0.0f32;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    pbonecontroller =
        (m_pStudioHeader as
             *mut byte).offset((*m_pStudioHeader).bonecontrollerindex as
                                   isize) as *mut mstudiobonecontroller_t;
    j = 0 as libc::c_int;
    while j < (*m_pStudioHeader).numbonecontrollers {
        i = (*pbonecontroller.offset(j as isize)).index;
        if i == 4 as libc::c_int {
            // mouth hardcoded at controller 4
            value = mouthopen as libc::c_float / 64.0f32;
            value =
                if value >= 0.0f32 {
                    if value < 1.0f32 { value } else { 1.0f32 }
                } else { 0.0f32 };
            value =
                (1.0f32 - value) * (*pbonecontroller.offset(j as isize)).start
                    + value * (*pbonecontroller.offset(j as isize)).end
        } else if i < 4 as libc::c_int {
            // check for 360% wrapping
            if (*pbonecontroller.offset(j as isize)).type_0 &
                   0x8000 as libc::c_int != 0 {
                if abs(*pcontroller1.offset(i as isize) as libc::c_int -
                           *pcontroller2.offset(i as isize) as libc::c_int) >
                       128 as libc::c_int {
                    let mut a: libc::c_int =
                        (*pcontroller1.offset(i as isize) as libc::c_int +
                             128 as libc::c_int) % 256 as libc::c_int;
                    let mut b: libc::c_int =
                        (*pcontroller2.offset(i as isize) as libc::c_int +
                             128 as libc::c_int) % 256 as libc::c_int;
                    value =
                        (a as libc::c_float * dadt +
                             b as libc::c_float * (1.0f32 - dadt) -
                             128 as libc::c_int as libc::c_float) *
                            (360.0f32 / 256.0f32) +
                            (*pbonecontroller.offset(j as isize)).start
                } else {
                    value =
                        (*pcontroller1.offset(i as isize) as libc::c_int as
                             libc::c_float * dadt +
                             *pcontroller2.offset(i as isize) as libc::c_int
                                 as libc::c_float * (1.0f32 - dadt)) *
                            (360.0f32 / 256.0f32) +
                            (*pbonecontroller.offset(j as isize)).start
                }
            } else {
                value =
                    (*pcontroller1.offset(i as isize) as libc::c_int as
                         libc::c_float * dadt +
                         *pcontroller2.offset(i as isize) as libc::c_int as
                             libc::c_float * (1.0f32 - dadt)) / 255.0f32;
                value =
                    if value >= 0.0f32 {
                        if value < 1.0f32 { value } else { 1.0f32 }
                    } else { 0.0f32 };
                value =
                    (1.0f32 - value) *
                        (*pbonecontroller.offset(j as isize)).start +
                        value * (*pbonecontroller.offset(j as isize)).end
            }
        }
        match (*pbonecontroller.offset(j as isize)).type_0 &
                  0x7fff as libc::c_int {
            8 | 16 | 32 => {
                *adj.offset(j as isize) =
                    value *
                        (3.14159265358979323846f64 as libc::c_float /
                             180.0f32)
            }
            1 | 2 | 4 => { *adj.offset(j as isize) = value }
            _ => { }
        }
        j += 1
    };
}
/*
====================
StudioCalcRotations

====================
*/
#[no_mangle]
pub unsafe extern "C" fn R_StudioCalcRotations(mut e: *mut cl_entity_t,
                                               mut pos:
                                                   *mut [libc::c_float; 3],
                                               mut q: *mut vec4_t,
                                               mut pseqdesc:
                                                   *mut mstudioseqdesc_t,
                                               mut panim: *mut mstudioanim_t,
                                               mut f: libc::c_float) {
    let mut i: libc::c_int = 0;
    let mut frame: libc::c_int = 0;
    let mut adj: [libc::c_float; 32] = [0.; 32];
    let mut s: libc::c_float = 0.;
    let mut dadt: libc::c_float = 0.;
    let mut pbone: *mut mstudiobone_t = 0 as *mut mstudiobone_t;
    // bah, fix this bug with changing sequences too fast
    if f > ((*pseqdesc).numframes - 1 as libc::c_int) as libc::c_float {
        f = 0.0f32
    } else if f < -0.01f32 {
        // BUG ( somewhere else ) but this code should validate this data.
		// This could cause a crash if the frame # is negative, so we'll go ahead
		// and clamp it here
        f = -0.01f32
    }
    frame = f as libc::c_int;
    dadt = R_StudioEstimateInterpolant(e);
    s = f - frame as libc::c_float;
    // add in programtic controllers
    pbone =
        (m_pStudioHeader as
             *mut byte).offset((*m_pStudioHeader).boneindex as isize) as
            *mut mstudiobone_t;
    R_StudioCalcBoneAdj(dadt, adj.as_mut_ptr(),
                        (*e).curstate.controller.as_mut_ptr(),
                        (*e).latched.prevcontroller.as_mut_ptr(),
                        (*e).mouth.mouthopen);
    i = 0 as libc::c_int;
    while i < (*m_pStudioHeader).numbones {
        gEngfuncs.R_StudioCalcBoneQuaternion.expect("non-null function pointer")(frame,
                                                                                 s,
                                                                                 pbone,
                                                                                 panim,
                                                                                 adj.as_mut_ptr(),
                                                                                 (*q.offset(i
                                                                                                as
                                                                                                isize)).as_mut_ptr());
        gEngfuncs.R_StudioCalcBonePosition.expect("non-null function pointer")(frame,
                                                                               s,
                                                                               pbone,
                                                                               panim,
                                                                               adj.as_mut_ptr(),
                                                                               (*pos.offset(i
                                                                                                as
                                                                                                isize)).as_mut_ptr());
        i += 1;
        pbone = pbone.offset(1);
        panim = panim.offset(1)
    }
    if (*pseqdesc).motiontype & 0x1 as libc::c_int != 0 {
        (*pos.offset((*pseqdesc).motionbone as
                         isize))[0 as libc::c_int as usize] = 0.0f32
    }
    if (*pseqdesc).motiontype & 0x2 as libc::c_int != 0 {
        (*pos.offset((*pseqdesc).motionbone as
                         isize))[1 as libc::c_int as usize] = 0.0f32
    }
    if (*pseqdesc).motiontype & 0x4 as libc::c_int != 0 {
        (*pos.offset((*pseqdesc).motionbone as
                         isize))[2 as libc::c_int as usize] = 0.0f32
    };
}
/*
====================
StudioMergeBones

====================
*/
#[no_mangle]
pub unsafe extern "C" fn R_StudioMergeBones(mut e: *mut cl_entity_t,
                                            mut m_pSubModel_0: *mut model_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut pbones: *mut mstudiobone_t = 0 as *mut mstudiobone_t;
    let mut pseqdesc: *mut mstudioseqdesc_t = 0 as *mut mstudioseqdesc_t;
    let mut panim: *mut mstudioanim_t = 0 as *mut mstudioanim_t;
    let mut bonematrix: matrix3x4 = [[0.; 4]; 3];
    static mut q: [vec4_t; 128] = [[0.; 4]; 128];
    static mut pos: [[libc::c_float; 3]; 128] = [[0.; 3]; 128];
    let mut f: libc::c_float = 0.;
    if (*e).curstate.sequence >= (*m_pStudioHeader).numseq {
        (*e).curstate.sequence = 0 as libc::c_int
    }
    pseqdesc =
        ((m_pStudioHeader as
              *mut byte).offset((*m_pStudioHeader).seqindex as isize) as
             *mut mstudioseqdesc_t).offset((*e).curstate.sequence as isize);
    f = R_StudioEstimateFrame(e, pseqdesc);
    panim =
        gEngfuncs.R_StudioGetAnim.expect("non-null function pointer")(m_pStudioHeader,
                                                                      m_pSubModel_0,
                                                                      pseqdesc)
            as *mut mstudioanim_t;
    R_StudioCalcRotations(e, pos.as_mut_ptr(), q.as_mut_ptr(), pseqdesc,
                          panim, f);
    pbones =
        (m_pStudioHeader as
             *mut byte).offset((*m_pStudioHeader).boneindex as isize) as
            *mut mstudiobone_t;
    i = 0 as libc::c_int;
    while i < (*m_pStudioHeader).numbones {
        j = 0 as libc::c_int;
        while j < g_studio.cached_numbones {
            if Q_strnicmp((*pbones.offset(i as isize)).name.as_mut_ptr(),
                          g_studio.cached_bonenames[j as usize].as_mut_ptr(),
                          99999 as libc::c_int) == 0 {
                memcpy(g_studio.bonestransform[i as usize].as_mut_ptr() as
                           *mut libc::c_void,
                       g_studio.cached_bonestransform[j as usize].as_mut_ptr()
                           as *const libc::c_void,
                       ::std::mem::size_of::<matrix3x4>() as libc::c_ulong);
                memcpy(g_studio.lighttransform[i as usize].as_mut_ptr() as
                           *mut libc::c_void,
                       g_studio.cached_lighttransform[j as usize].as_mut_ptr()
                           as *const libc::c_void,
                       ::std::mem::size_of::<matrix3x4>() as libc::c_ulong);
                break ;
            } else { j += 1 }
        }
        if j >= g_studio.cached_numbones {
            Matrix3x4_FromOriginQuat(bonematrix.as_mut_ptr(),
                                     q[i as usize].as_mut_ptr() as
                                         *const vec_t,
                                     pos[i as usize].as_mut_ptr() as
                                         *const vec_t);
            if (*pbones.offset(i as isize)).parent == -(1 as libc::c_int) {
                Matrix3x4_ConcatTransforms(g_studio.bonestransform[i as
                                                                       usize].as_mut_ptr(),
                                           g_studio.rotationmatrix.as_mut_ptr()
                                               as *const [vec_t; 4],
                                           bonematrix.as_mut_ptr() as
                                               *const [vec_t; 4]);
                memcpy(g_studio.lighttransform[i as usize].as_mut_ptr() as
                           *mut libc::c_void,
                       g_studio.bonestransform[i as usize].as_mut_ptr() as
                           *const libc::c_void,
                       ::std::mem::size_of::<matrix3x4>() as libc::c_ulong);
                // apply client-side effects to the transformation matrix
                R_StudioFxTransform(e,
                                    g_studio.bonestransform[i as
                                                                usize].as_mut_ptr());
            } else {
                Matrix3x4_ConcatTransforms(g_studio.bonestransform[i as
                                                                       usize].as_mut_ptr(),
                                           g_studio.bonestransform[(*pbones.offset(i
                                                                                       as
                                                                                       isize)).parent
                                                                       as
                                                                       usize].as_mut_ptr()
                                               as *const [vec_t; 4],
                                           bonematrix.as_mut_ptr() as
                                               *const [vec_t; 4]);
                Matrix3x4_ConcatTransforms(g_studio.lighttransform[i as
                                                                       usize].as_mut_ptr(),
                                           g_studio.lighttransform[(*pbones.offset(i
                                                                                       as
                                                                                       isize)).parent
                                                                       as
                                                                       usize].as_mut_ptr()
                                               as *const [vec_t; 4],
                                           bonematrix.as_mut_ptr() as
                                               *const [vec_t; 4]);
            }
        }
        i += 1
    };
}
/*
====================
StudioSetupBones

====================
*/
#[no_mangle]
pub unsafe extern "C" fn R_StudioSetupBones(mut e: *mut cl_entity_t) {
    let mut f: libc::c_float = 0.;
    let mut pbones: *mut mstudiobone_t = 0 as *mut mstudiobone_t;
    let mut pseqdesc: *mut mstudioseqdesc_t = 0 as *mut mstudioseqdesc_t;
    let mut panim: *mut mstudioanim_t = 0 as *mut mstudioanim_t;
    let mut bonematrix: matrix3x4 = [[0.; 4]; 3];
    static mut pos: [vec3_t; 128] = [[0.; 3]; 128];
    static mut q: [vec4_t; 128] = [[0.; 4]; 128];
    static mut pos2: [vec3_t; 128] = [[0.; 3]; 128];
    static mut q2: [vec4_t; 128] = [[0.; 4]; 128];
    static mut pos3: [vec3_t; 128] = [[0.; 3]; 128];
    static mut q3: [vec4_t; 128] = [[0.; 4]; 128];
    static mut pos4: [vec3_t; 128] = [[0.; 3]; 128];
    static mut q4: [vec4_t; 128] = [[0.; 4]; 128];
    let mut i: libc::c_int = 0;
    if (*e).curstate.sequence >= (*m_pStudioHeader).numseq {
        (*e).curstate.sequence = 0 as libc::c_int
    }
    pseqdesc =
        ((m_pStudioHeader as
              *mut byte).offset((*m_pStudioHeader).seqindex as isize) as
             *mut mstudioseqdesc_t).offset((*e).curstate.sequence as isize);
    f = R_StudioEstimateFrame(e, pseqdesc);
    panim =
        gEngfuncs.R_StudioGetAnim.expect("non-null function pointer")(m_pStudioHeader,
                                                                      RI.currentmodel,
                                                                      pseqdesc)
            as *mut mstudioanim_t;
    R_StudioCalcRotations(e, pos.as_mut_ptr(), q.as_mut_ptr(), pseqdesc,
                          panim, f);
    if (*pseqdesc).numblends > 1 as libc::c_int {
        let mut s: libc::c_float = 0.;
        let mut dadt: libc::c_float = 0.;
        panim = panim.offset((*m_pStudioHeader).numbones as isize);
        R_StudioCalcRotations(e, pos2.as_mut_ptr(), q2.as_mut_ptr(), pseqdesc,
                              panim, f);
        dadt = R_StudioEstimateInterpolant(e);
        s =
            ((*e).curstate.blending[0 as libc::c_int as usize] as libc::c_int
                 as libc::c_float * dadt +
                 (*e).latched.prevblending[0 as libc::c_int as usize] as
                     libc::c_int as libc::c_float * (1.0f32 - dadt)) /
                255.0f32;
        gEngfuncs.R_StudioSlerpBones.expect("non-null function pointer")((*m_pStudioHeader).numbones,
                                                                         q.as_mut_ptr(),
                                                                         pos.as_mut_ptr(),
                                                                         q2.as_mut_ptr(),
                                                                         pos2.as_mut_ptr(),
                                                                         s);
        if (*pseqdesc).numblends == 4 as libc::c_int {
            panim = panim.offset((*m_pStudioHeader).numbones as isize);
            R_StudioCalcRotations(e, pos3.as_mut_ptr(), q3.as_mut_ptr(),
                                  pseqdesc, panim, f);
            panim = panim.offset((*m_pStudioHeader).numbones as isize);
            R_StudioCalcRotations(e, pos4.as_mut_ptr(), q4.as_mut_ptr(),
                                  pseqdesc, panim, f);
            s =
                ((*e).curstate.blending[0 as libc::c_int as usize] as
                     libc::c_int as libc::c_float * dadt +
                     (*e).latched.prevblending[0 as libc::c_int as usize] as
                         libc::c_int as libc::c_float * (1.0f32 - dadt)) /
                    255.0f32;
            gEngfuncs.R_StudioSlerpBones.expect("non-null function pointer")((*m_pStudioHeader).numbones,
                                                                             q3.as_mut_ptr(),
                                                                             pos3.as_mut_ptr(),
                                                                             q4.as_mut_ptr(),
                                                                             pos4.as_mut_ptr(),
                                                                             s);
            s =
                ((*e).curstate.blending[1 as libc::c_int as usize] as
                     libc::c_int as libc::c_float * dadt +
                     (*e).latched.prevblending[1 as libc::c_int as usize] as
                         libc::c_int as libc::c_float * (1.0f32 - dadt)) /
                    255.0f32;
            gEngfuncs.R_StudioSlerpBones.expect("non-null function pointer")((*m_pStudioHeader).numbones,
                                                                             q.as_mut_ptr(),
                                                                             pos.as_mut_ptr(),
                                                                             q3.as_mut_ptr(),
                                                                             pos3.as_mut_ptr(),
                                                                             s);
        }
    }
    if g_studio.interpolate as libc::c_uint != 0 &&
           (*e).latched.sequencetime != 0. &&
           ((*e).latched.sequencetime + 0.2f32) as libc::c_double >
               g_studio.time &&
           (*e).latched.prevsequence < (*m_pStudioHeader).numseq {
        // blend from last sequence
        static mut pos1b: [vec3_t; 128] = [[0.; 3]; 128];
        static mut q1b: [vec4_t; 128] = [[0.; 4]; 128];
        let mut s_0: libc::c_float = 0.;
        pseqdesc =
            ((m_pStudioHeader as
                  *mut byte).offset((*m_pStudioHeader).seqindex as isize) as
                 *mut mstudioseqdesc_t).offset((*e).latched.prevsequence as
                                                   isize);
        panim =
            gEngfuncs.R_StudioGetAnim.expect("non-null function pointer")(m_pStudioHeader,
                                                                          RI.currentmodel,
                                                                          pseqdesc)
                as *mut mstudioanim_t;
        // clip prevframe
        R_StudioCalcRotations(e, pos1b.as_mut_ptr(), q1b.as_mut_ptr(),
                              pseqdesc, panim, (*e).latched.prevframe);
        if (*pseqdesc).numblends > 1 as libc::c_int {
            panim = panim.offset((*m_pStudioHeader).numbones as isize);
            R_StudioCalcRotations(e, pos2.as_mut_ptr(), q2.as_mut_ptr(),
                                  pseqdesc, panim, (*e).latched.prevframe);
            s_0 =
                (*e).latched.prevseqblending[0 as libc::c_int as usize] as
                    libc::c_int as libc::c_float / 255.0f32;
            gEngfuncs.R_StudioSlerpBones.expect("non-null function pointer")((*m_pStudioHeader).numbones,
                                                                             q1b.as_mut_ptr(),
                                                                             pos1b.as_mut_ptr(),
                                                                             q2.as_mut_ptr(),
                                                                             pos2.as_mut_ptr(),
                                                                             s_0);
            if (*pseqdesc).numblends == 4 as libc::c_int {
                panim = panim.offset((*m_pStudioHeader).numbones as isize);
                R_StudioCalcRotations(e, pos3.as_mut_ptr(), q3.as_mut_ptr(),
                                      pseqdesc, panim,
                                      (*e).latched.prevframe);
                panim = panim.offset((*m_pStudioHeader).numbones as isize);
                R_StudioCalcRotations(e, pos4.as_mut_ptr(), q4.as_mut_ptr(),
                                      pseqdesc, panim,
                                      (*e).latched.prevframe);
                s_0 =
                    (*e).latched.prevseqblending[0 as libc::c_int as usize] as
                        libc::c_int as libc::c_float / 255.0f32;
                gEngfuncs.R_StudioSlerpBones.expect("non-null function pointer")((*m_pStudioHeader).numbones,
                                                                                 q3.as_mut_ptr(),
                                                                                 pos3.as_mut_ptr(),
                                                                                 q4.as_mut_ptr(),
                                                                                 pos4.as_mut_ptr(),
                                                                                 s_0);
                s_0 =
                    (*e).latched.prevseqblending[1 as libc::c_int as usize] as
                        libc::c_int as libc::c_float / 255.0f32;
                gEngfuncs.R_StudioSlerpBones.expect("non-null function pointer")((*m_pStudioHeader).numbones,
                                                                                 q1b.as_mut_ptr(),
                                                                                 pos1b.as_mut_ptr(),
                                                                                 q3.as_mut_ptr(),
                                                                                 pos3.as_mut_ptr(),
                                                                                 s_0);
            }
        }
        s_0 =
            (1.0f32 as libc::c_double -
                 (g_studio.time - (*e).latched.sequencetime as libc::c_double)
                     / 0.2f32 as libc::c_double) as libc::c_float;
        gEngfuncs.R_StudioSlerpBones.expect("non-null function pointer")((*m_pStudioHeader).numbones,
                                                                         q.as_mut_ptr(),
                                                                         pos.as_mut_ptr(),
                                                                         q1b.as_mut_ptr(),
                                                                         pos1b.as_mut_ptr(),
                                                                         s_0);
    } else {
        // store prevframe otherwise
        (*e).latched.prevframe = f
    }
    pbones =
        (m_pStudioHeader as
             *mut byte).offset((*m_pStudioHeader).boneindex as isize) as
            *mut mstudiobone_t;
    // calc gait animation
    if !m_pPlayerInfo.is_null() &&
           (*m_pPlayerInfo).gaitsequence != 0 as libc::c_int {
        let mut copy_bones: qboolean = true_0;
        if (*m_pPlayerInfo).gaitsequence >= (*m_pStudioHeader).numseq {
            (*m_pPlayerInfo).gaitsequence = 0 as libc::c_int
        }
        pseqdesc =
            ((m_pStudioHeader as
                  *mut byte).offset((*m_pStudioHeader).seqindex as isize) as
                 *mut mstudioseqdesc_t).offset((*m_pPlayerInfo).gaitsequence
                                                   as isize);
        panim =
            gEngfuncs.R_StudioGetAnim.expect("non-null function pointer")(m_pStudioHeader,
                                                                          RI.currentmodel,
                                                                          pseqdesc)
                as *mut mstudioanim_t;
        R_StudioCalcRotations(e, pos2.as_mut_ptr(), q2.as_mut_ptr(), pseqdesc,
                              panim, (*m_pPlayerInfo).gaitframe);
        i = 0 as libc::c_int;
        while i < (*m_pStudioHeader).numbones {
            if Q_strncmp((*pbones.offset(i as isize)).name.as_mut_ptr(),
                         b"Bip01 Spine\x00" as *const u8 as
                             *const libc::c_char, 99999 as libc::c_int) == 0 {
                copy_bones = false_0
            } else if Q_strncmp((*pbones.offset((*pbones.offset(i as
                                                                    isize)).parent
                                                    as
                                                    isize)).name.as_mut_ptr(),
                                b"Bip01 Pelvis\x00" as *const u8 as
                                    *const libc::c_char, 99999 as libc::c_int)
                          == 0 {
                copy_bones = true_0
            }
            if !(copy_bones as u64 == 0) {
                pos[i as usize][0 as libc::c_int as usize] =
                    pos2[i as usize][0 as libc::c_int as usize];
                pos[i as usize][1 as libc::c_int as usize] =
                    pos2[i as usize][1 as libc::c_int as usize];
                pos[i as usize][2 as libc::c_int as usize] =
                    pos2[i as usize][2 as libc::c_int as usize];
                q[i as usize][0 as libc::c_int as usize] =
                    q2[i as usize][0 as libc::c_int as usize];
                q[i as usize][1 as libc::c_int as usize] =
                    q2[i as usize][1 as libc::c_int as usize];
                q[i as usize][2 as libc::c_int as usize] =
                    q2[i as usize][2 as libc::c_int as usize];
                q[i as usize][3 as libc::c_int as usize] =
                    q2[i as usize][3 as libc::c_int as usize]
            }
            i += 1
        }
    }
    i = 0 as libc::c_int;
    while i < (*m_pStudioHeader).numbones {
        Matrix3x4_FromOriginQuat(bonematrix.as_mut_ptr(),
                                 q[i as usize].as_mut_ptr() as *const vec_t,
                                 pos[i as usize].as_mut_ptr() as
                                     *const vec_t);
        if (*pbones.offset(i as isize)).parent == -(1 as libc::c_int) {
            Matrix3x4_ConcatTransforms(g_studio.bonestransform[i as
                                                                   usize].as_mut_ptr(),
                                       g_studio.rotationmatrix.as_mut_ptr() as
                                           *const [vec_t; 4],
                                       bonematrix.as_mut_ptr() as
                                           *const [vec_t; 4]);
            memcpy(g_studio.lighttransform[i as usize].as_mut_ptr() as
                       *mut libc::c_void,
                   g_studio.bonestransform[i as usize].as_mut_ptr() as
                       *const libc::c_void,
                   ::std::mem::size_of::<matrix3x4>() as libc::c_ulong);
            // apply client-side effects to the transformation matrix
            R_StudioFxTransform(e,
                                g_studio.bonestransform[i as
                                                            usize].as_mut_ptr());
        } else {
            Matrix3x4_ConcatTransforms(g_studio.bonestransform[i as
                                                                   usize].as_mut_ptr(),
                                       g_studio.bonestransform[(*pbones.offset(i
                                                                                   as
                                                                                   isize)).parent
                                                                   as
                                                                   usize].as_mut_ptr()
                                           as *const [vec_t; 4],
                                       bonematrix.as_mut_ptr() as
                                           *const [vec_t; 4]);
            Matrix3x4_ConcatTransforms(g_studio.lighttransform[i as
                                                                   usize].as_mut_ptr(),
                                       g_studio.lighttransform[(*pbones.offset(i
                                                                                   as
                                                                                   isize)).parent
                                                                   as
                                                                   usize].as_mut_ptr()
                                           as *const [vec_t; 4],
                                       bonematrix.as_mut_ptr() as
                                           *const [vec_t; 4]);
        }
        i += 1
    };
}
/*
====================
StudioSaveBones

====================
*/
unsafe extern "C" fn R_StudioSaveBones() {
    let mut pbones: *mut mstudiobone_t = 0 as *mut mstudiobone_t;
    let mut i: libc::c_int = 0;
    pbones =
        (m_pStudioHeader as
             *mut byte).offset((*m_pStudioHeader).boneindex as isize) as
            *mut mstudiobone_t;
    g_studio.cached_numbones = (*m_pStudioHeader).numbones;
    i = 0 as libc::c_int;
    while i < (*m_pStudioHeader).numbones {
        memcpy(g_studio.cached_bonestransform[i as usize].as_mut_ptr() as
                   *mut libc::c_void,
               g_studio.bonestransform[i as usize].as_mut_ptr() as
                   *const libc::c_void,
               ::std::mem::size_of::<matrix3x4>() as libc::c_ulong);
        memcpy(g_studio.cached_lighttransform[i as usize].as_mut_ptr() as
                   *mut libc::c_void,
               g_studio.lighttransform[i as usize].as_mut_ptr() as
                   *const libc::c_void,
               ::std::mem::size_of::<matrix3x4>() as libc::c_ulong);
        Q_strncpy(g_studio.cached_bonenames[i as usize].as_mut_ptr(),
                  (*pbones.offset(i as isize)).name.as_mut_ptr(),
                  32 as libc::c_int as size_t);
        i += 1
    };
}
/*
====================
StudioBuildNormalTable

NOTE: m_pSubModel must be set
====================
*/
#[no_mangle]
pub unsafe extern "C" fn R_StudioBuildNormalTable() {
    let mut e: *mut cl_entity_t = RI.currententity;
    let mut pmesh: *mut mstudiomesh_t = 0 as *mut mstudiomesh_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if m_pSubModel.is_null() {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 b"../ref_gl/gl_studio.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 1121 as
                                                                     libc::c_int);
    }
    // reset chrome cache
    i = 0 as libc::c_int;
    while i < (*m_pStudioHeader).numbones {
        g_studio.chromeage[i as usize] = 0 as libc::c_int;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < (*m_pSubModel).numverts {
        g_studio.normaltable[i as usize] = -(1 as libc::c_int);
        i += 1
    }
    j = 0 as libc::c_int;
    while j < (*m_pSubModel).nummesh {
        let mut ptricmds: *mut libc::c_short = 0 as *mut libc::c_short;
        pmesh =
            ((m_pStudioHeader as
                  *mut byte).offset((*m_pSubModel).meshindex as isize) as
                 *mut mstudiomesh_t).offset(j as isize);
        ptricmds =
            (m_pStudioHeader as *mut byte).offset((*pmesh).triindex as isize)
                as *mut libc::c_short;
        loop  {
            let fresh4 = ptricmds;
            ptricmds = ptricmds.offset(1);
            i = *fresh4 as libc::c_int;
            if !(i != 0) { break ; }
            if i < 0 as libc::c_int { i = -i }
            while i > 0 as libc::c_int {
                if g_studio.normaltable[*ptricmds.offset(0 as libc::c_int as
                                                             isize) as usize]
                       < 0 as libc::c_int {
                    g_studio.normaltable[*ptricmds.offset(0 as libc::c_int as
                                                              isize) as usize]
                        =
                        *ptricmds.offset(1 as libc::c_int as isize) as
                            libc::c_int
                }
                i -= 1;
                ptricmds = ptricmds.offset(4 as libc::c_int as isize)
            }
        }
        j += 1
    }
    g_studio.chrome_origin[0 as libc::c_int as usize] =
        (__tg_cos((*r_glowshellfreq).value as libc::c_double * g_studio.time)
             * 4000.0f32 as libc::c_double) as vec_t;
    g_studio.chrome_origin[1 as libc::c_int as usize] =
        (__tg_sin((*r_glowshellfreq).value as libc::c_double * g_studio.time)
             * 4000.0f32 as libc::c_double) as vec_t;
    g_studio.chrome_origin[2 as libc::c_int as usize] =
        (__tg_cos((*r_glowshellfreq).value as libc::c_double * g_studio.time *
                      0.33f32 as libc::c_double) *
             4000.0f32 as libc::c_double) as vec_t;
    if (*e).curstate.rendercolor.r as libc::c_int != 0 ||
           (*e).curstate.rendercolor.g as libc::c_int != 0 ||
           (*e).curstate.rendercolor.b as libc::c_int != 0 {
        TriColor4ub((*e).curstate.rendercolor.r, (*e).curstate.rendercolor.g,
                    (*e).curstate.rendercolor.b, 255 as libc::c_int as byte);
    } else {
        TriColor4ub(255 as libc::c_int as byte, 255 as libc::c_int as byte,
                    255 as libc::c_int as byte, 255 as libc::c_int as byte);
    };
}
/*
====================
StudioGenerateNormals

NOTE: m_pSubModel must be set
g_studio.verts must be computed
====================
*/
#[no_mangle]
pub unsafe extern "C" fn R_StudioGenerateNormals() {
    let mut v0: libc::c_int = 0;
    let mut v1: libc::c_int = 0;
    let mut v2: libc::c_int = 0;
    let mut e0: vec3_t = [0.; 3];
    let mut e1: vec3_t = [0.; 3];
    let mut norm: vec3_t = [0.; 3];
    let mut pmesh: *mut mstudiomesh_t = 0 as *mut mstudiomesh_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if m_pSubModel.is_null() {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 b"../ref_gl/gl_studio.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 1173 as
                                                                     libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < (*m_pSubModel).numverts {
        g_studio.norms[i as usize][2 as libc::c_int as usize] =
            0 as libc::c_int as vec_t;
        g_studio.norms[i as usize][1 as libc::c_int as usize] =
            g_studio.norms[i as usize][2 as libc::c_int as usize];
        g_studio.norms[i as usize][0 as libc::c_int as usize] =
            g_studio.norms[i as usize][1 as libc::c_int as usize];
        i += 1
    }
    j = 0 as libc::c_int;
    while j < (*m_pSubModel).nummesh {
        let mut ptricmds: *mut libc::c_short = 0 as *mut libc::c_short;
        pmesh =
            ((m_pStudioHeader as
                  *mut byte).offset((*m_pSubModel).meshindex as isize) as
                 *mut mstudiomesh_t).offset(j as isize);
        ptricmds =
            (m_pStudioHeader as *mut byte).offset((*pmesh).triindex as isize)
                as *mut libc::c_short;
        loop  {
            let fresh5 = ptricmds;
            ptricmds = ptricmds.offset(1);
            i = *fresh5 as libc::c_int;
            if !(i != 0) { break ; }
            if i < 0 as libc::c_int {
                i = -i;
                if i > 2 as libc::c_int {
                    v0 =
                        *ptricmds.offset(0 as libc::c_int as isize) as
                            libc::c_int;
                    ptricmds = ptricmds.offset(4 as libc::c_int as isize);
                    v1 =
                        *ptricmds.offset(0 as libc::c_int as isize) as
                            libc::c_int;
                    ptricmds = ptricmds.offset(4 as libc::c_int as isize);
                    i -= 2 as libc::c_int;
                    while i > 0 as libc::c_int {
                        v2 =
                            *ptricmds.offset(0 as libc::c_int as isize) as
                                libc::c_int;
                        e0[0 as libc::c_int as usize] =
                            g_studio.verts[v1 as
                                               usize][0 as libc::c_int as
                                                          usize] -
                                g_studio.verts[v0 as
                                                   usize][0 as libc::c_int as
                                                              usize];
                        e0[1 as libc::c_int as usize] =
                            g_studio.verts[v1 as
                                               usize][1 as libc::c_int as
                                                          usize] -
                                g_studio.verts[v0 as
                                                   usize][1 as libc::c_int as
                                                              usize];
                        e0[2 as libc::c_int as usize] =
                            g_studio.verts[v1 as
                                               usize][2 as libc::c_int as
                                                          usize] -
                                g_studio.verts[v0 as
                                                   usize][2 as libc::c_int as
                                                              usize];
                        e1[0 as libc::c_int as usize] =
                            g_studio.verts[v2 as
                                               usize][0 as libc::c_int as
                                                          usize] -
                                g_studio.verts[v0 as
                                                   usize][0 as libc::c_int as
                                                              usize];
                        e1[1 as libc::c_int as usize] =
                            g_studio.verts[v2 as
                                               usize][1 as libc::c_int as
                                                          usize] -
                                g_studio.verts[v0 as
                                                   usize][1 as libc::c_int as
                                                              usize];
                        e1[2 as libc::c_int as usize] =
                            g_studio.verts[v2 as
                                               usize][2 as libc::c_int as
                                                          usize] -
                                g_studio.verts[v0 as
                                                   usize][2 as libc::c_int as
                                                              usize];
                        norm[0 as libc::c_int as usize] =
                            e1[1 as libc::c_int as usize] *
                                e0[2 as libc::c_int as usize] -
                                e1[2 as libc::c_int as usize] *
                                    e0[1 as libc::c_int as usize];
                        norm[1 as libc::c_int as usize] =
                            e1[2 as libc::c_int as usize] *
                                e0[0 as libc::c_int as usize] -
                                e1[0 as libc::c_int as usize] *
                                    e0[2 as libc::c_int as usize];
                        norm[2 as libc::c_int as usize] =
                            e1[0 as libc::c_int as usize] *
                                e0[1 as libc::c_int as usize] -
                                e1[1 as libc::c_int as usize] *
                                    e0[0 as libc::c_int as usize];
                        g_studio.norms[v0 as usize][0 as libc::c_int as usize]
                            =
                            g_studio.norms[v0 as
                                               usize][0 as libc::c_int as
                                                          usize] +
                                norm[0 as libc::c_int as usize];
                        g_studio.norms[v0 as usize][1 as libc::c_int as usize]
                            =
                            g_studio.norms[v0 as
                                               usize][1 as libc::c_int as
                                                          usize] +
                                norm[1 as libc::c_int as usize];
                        g_studio.norms[v0 as usize][2 as libc::c_int as usize]
                            =
                            g_studio.norms[v0 as
                                               usize][2 as libc::c_int as
                                                          usize] +
                                norm[2 as libc::c_int as usize];
                        g_studio.norms[v1 as usize][0 as libc::c_int as usize]
                            =
                            g_studio.norms[v1 as
                                               usize][0 as libc::c_int as
                                                          usize] +
                                norm[0 as libc::c_int as usize];
                        g_studio.norms[v1 as usize][1 as libc::c_int as usize]
                            =
                            g_studio.norms[v1 as
                                               usize][1 as libc::c_int as
                                                          usize] +
                                norm[1 as libc::c_int as usize];
                        g_studio.norms[v1 as usize][2 as libc::c_int as usize]
                            =
                            g_studio.norms[v1 as
                                               usize][2 as libc::c_int as
                                                          usize] +
                                norm[2 as libc::c_int as usize];
                        g_studio.norms[v2 as usize][0 as libc::c_int as usize]
                            =
                            g_studio.norms[v2 as
                                               usize][0 as libc::c_int as
                                                          usize] +
                                norm[0 as libc::c_int as usize];
                        g_studio.norms[v2 as usize][1 as libc::c_int as usize]
                            =
                            g_studio.norms[v2 as
                                               usize][1 as libc::c_int as
                                                          usize] +
                                norm[1 as libc::c_int as usize];
                        g_studio.norms[v2 as usize][2 as libc::c_int as usize]
                            =
                            g_studio.norms[v2 as
                                               usize][2 as libc::c_int as
                                                          usize] +
                                norm[2 as libc::c_int as usize];
                        v1 = v2;
                        i -= 1;
                        ptricmds = ptricmds.offset(4 as libc::c_int as isize)
                    }
                } else { ptricmds = ptricmds.offset(i as isize) }
            } else if i > 2 as libc::c_int {
                let mut odd: qboolean = false_0;
                v0 =
                    *ptricmds.offset(0 as libc::c_int as isize) as
                        libc::c_int;
                ptricmds = ptricmds.offset(4 as libc::c_int as isize);
                v1 =
                    *ptricmds.offset(0 as libc::c_int as isize) as
                        libc::c_int;
                ptricmds = ptricmds.offset(4 as libc::c_int as isize);
                i -= 2 as libc::c_int;
                while i > 0 as libc::c_int {
                    v2 =
                        *ptricmds.offset(0 as libc::c_int as isize) as
                            libc::c_int;
                    e0[0 as libc::c_int as usize] =
                        g_studio.verts[v1 as usize][0 as libc::c_int as usize]
                            -
                            g_studio.verts[v0 as
                                               usize][0 as libc::c_int as
                                                          usize];
                    e0[1 as libc::c_int as usize] =
                        g_studio.verts[v1 as usize][1 as libc::c_int as usize]
                            -
                            g_studio.verts[v0 as
                                               usize][1 as libc::c_int as
                                                          usize];
                    e0[2 as libc::c_int as usize] =
                        g_studio.verts[v1 as usize][2 as libc::c_int as usize]
                            -
                            g_studio.verts[v0 as
                                               usize][2 as libc::c_int as
                                                          usize];
                    e1[0 as libc::c_int as usize] =
                        g_studio.verts[v2 as usize][0 as libc::c_int as usize]
                            -
                            g_studio.verts[v0 as
                                               usize][0 as libc::c_int as
                                                          usize];
                    e1[1 as libc::c_int as usize] =
                        g_studio.verts[v2 as usize][1 as libc::c_int as usize]
                            -
                            g_studio.verts[v0 as
                                               usize][1 as libc::c_int as
                                                          usize];
                    e1[2 as libc::c_int as usize] =
                        g_studio.verts[v2 as usize][2 as libc::c_int as usize]
                            -
                            g_studio.verts[v0 as
                                               usize][2 as libc::c_int as
                                                          usize];
                    norm[0 as libc::c_int as usize] =
                        e1[1 as libc::c_int as usize] *
                            e0[2 as libc::c_int as usize] -
                            e1[2 as libc::c_int as usize] *
                                e0[1 as libc::c_int as usize];
                    norm[1 as libc::c_int as usize] =
                        e1[2 as libc::c_int as usize] *
                            e0[0 as libc::c_int as usize] -
                            e1[0 as libc::c_int as usize] *
                                e0[2 as libc::c_int as usize];
                    norm[2 as libc::c_int as usize] =
                        e1[0 as libc::c_int as usize] *
                            e0[1 as libc::c_int as usize] -
                            e1[1 as libc::c_int as usize] *
                                e0[0 as libc::c_int as usize];
                    g_studio.norms[v0 as usize][0 as libc::c_int as usize] =
                        g_studio.norms[v0 as usize][0 as libc::c_int as usize]
                            + norm[0 as libc::c_int as usize];
                    g_studio.norms[v0 as usize][1 as libc::c_int as usize] =
                        g_studio.norms[v0 as usize][1 as libc::c_int as usize]
                            + norm[1 as libc::c_int as usize];
                    g_studio.norms[v0 as usize][2 as libc::c_int as usize] =
                        g_studio.norms[v0 as usize][2 as libc::c_int as usize]
                            + norm[2 as libc::c_int as usize];
                    g_studio.norms[v1 as usize][0 as libc::c_int as usize] =
                        g_studio.norms[v1 as usize][0 as libc::c_int as usize]
                            + norm[0 as libc::c_int as usize];
                    g_studio.norms[v1 as usize][1 as libc::c_int as usize] =
                        g_studio.norms[v1 as usize][1 as libc::c_int as usize]
                            + norm[1 as libc::c_int as usize];
                    g_studio.norms[v1 as usize][2 as libc::c_int as usize] =
                        g_studio.norms[v1 as usize][2 as libc::c_int as usize]
                            + norm[2 as libc::c_int as usize];
                    g_studio.norms[v2 as usize][0 as libc::c_int as usize] =
                        g_studio.norms[v2 as usize][0 as libc::c_int as usize]
                            + norm[0 as libc::c_int as usize];
                    g_studio.norms[v2 as usize][1 as libc::c_int as usize] =
                        g_studio.norms[v2 as usize][1 as libc::c_int as usize]
                            + norm[1 as libc::c_int as usize];
                    g_studio.norms[v2 as usize][2 as libc::c_int as usize] =
                        g_studio.norms[v2 as usize][2 as libc::c_int as usize]
                            + norm[2 as libc::c_int as usize];
                    if odd as u64 != 0 { v1 = v2 } else { v0 = v2 }
                    odd = (odd as u64 == 0) as libc::c_int as qboolean;
                    i -= 1;
                    ptricmds = ptricmds.offset(4 as libc::c_int as isize)
                }
            } else { ptricmds = ptricmds.offset(i as isize) }
        }
        j += 1
    }
    i = 0 as libc::c_int;
    while i < (*m_pSubModel).numverts {
        let mut ilength: libc::c_float =
            __tg_sqrt(g_studio.norms[i as usize][0 as libc::c_int as usize] *
                          g_studio.norms[i as
                                             usize][0 as libc::c_int as usize]
                          +
                          g_studio.norms[i as
                                             usize][1 as libc::c_int as usize]
                              *
                              g_studio.norms[i as
                                                 usize][1 as libc::c_int as
                                                            usize] +
                          g_studio.norms[i as
                                             usize][2 as libc::c_int as usize]
                              *
                              g_studio.norms[i as
                                                 usize][2 as libc::c_int as
                                                            usize]);
        if ilength != 0. { ilength = 1.0f32 / ilength }
        g_studio.norms[i as usize][0 as libc::c_int as usize] *= ilength;
        g_studio.norms[i as usize][1 as libc::c_int as usize] *= ilength;
        g_studio.norms[i as usize][2 as libc::c_int as usize] *= ilength;
        i += 1
    };
}
/*
====================
StudioSetupChrome

====================
*/
#[no_mangle]
pub unsafe extern "C" fn R_StudioSetupChrome(mut pchrome: *mut libc::c_float,
                                             mut bone: libc::c_int,
                                             mut normal: *mut vec_t) {
    let mut n: libc::c_float = 0.;
    if g_studio.chromeage[bone as usize] != g_studio.framecount {
        // calculate vectors from the viewer to the bone. This roughly adjusts for position
        let mut chromeupvec: vec3_t =
            [0.; 3]; // g_studio.chrome t vector in world reference frame
        let mut chromerightvec: vec3_t =
            [0.; 3]; // g_studio.chrome s vector in world reference frame
        let mut tmp: vec3_t =
            [0.; 3]; // vector pointing at bone in world reference frame
        tmp[0 as libc::c_int as usize] =
            -g_studio.chrome_origin[0 as libc::c_int as usize];
        tmp[1 as libc::c_int as usize] =
            -g_studio.chrome_origin[1 as libc::c_int as usize];
        tmp[2 as libc::c_int as usize] =
            -g_studio.chrome_origin[2 as libc::c_int as usize];
        tmp[0 as libc::c_int as usize] +=
            g_studio.bonestransform[bone as
                                        usize][0 as libc::c_int as
                                                   usize][3 as libc::c_int as
                                                              usize];
        tmp[1 as libc::c_int as usize] +=
            g_studio.bonestransform[bone as
                                        usize][1 as libc::c_int as
                                                   usize][3 as libc::c_int as
                                                              usize];
        tmp[2 as libc::c_int as usize] +=
            g_studio.bonestransform[bone as
                                        usize][2 as libc::c_int as
                                                   usize][3 as libc::c_int as
                                                              usize];
        let mut ilength: libc::c_float =
            __tg_sqrt(tmp[0 as libc::c_int as usize] *
                          tmp[0 as libc::c_int as usize] +
                          tmp[1 as libc::c_int as usize] *
                              tmp[1 as libc::c_int as usize] +
                          tmp[2 as libc::c_int as usize] *
                              tmp[2 as libc::c_int as usize]);
        if ilength != 0. { ilength = 1.0f32 / ilength }
        tmp[0 as libc::c_int as usize] *= ilength;
        tmp[1 as libc::c_int as usize] *= ilength;
        tmp[2 as libc::c_int as usize] *= ilength;
        chromeupvec[0 as libc::c_int as usize] =
            tmp[1 as libc::c_int as usize] *
                RI.vright[2 as libc::c_int as usize] -
                tmp[2 as libc::c_int as usize] *
                    RI.vright[1 as libc::c_int as usize];
        chromeupvec[1 as libc::c_int as usize] =
            tmp[2 as libc::c_int as usize] *
                RI.vright[0 as libc::c_int as usize] -
                tmp[0 as libc::c_int as usize] *
                    RI.vright[2 as libc::c_int as usize];
        chromeupvec[2 as libc::c_int as usize] =
            tmp[0 as libc::c_int as usize] *
                RI.vright[1 as libc::c_int as usize] -
                tmp[1 as libc::c_int as usize] *
                    RI.vright[0 as libc::c_int as usize];
        let mut ilength_0: libc::c_float =
            __tg_sqrt(chromeupvec[0 as libc::c_int as usize] *
                          chromeupvec[0 as libc::c_int as usize] +
                          chromeupvec[1 as libc::c_int as usize] *
                              chromeupvec[1 as libc::c_int as usize] +
                          chromeupvec[2 as libc::c_int as usize] *
                              chromeupvec[2 as libc::c_int as usize]);
        if ilength_0 != 0. { ilength_0 = 1.0f32 / ilength_0 }
        chromeupvec[0 as libc::c_int as usize] *= ilength_0;
        chromeupvec[1 as libc::c_int as usize] *= ilength_0;
        chromeupvec[2 as libc::c_int as usize] *= ilength_0;
        chromerightvec[0 as libc::c_int as usize] =
            tmp[1 as libc::c_int as usize] *
                chromeupvec[2 as libc::c_int as usize] -
                tmp[2 as libc::c_int as usize] *
                    chromeupvec[1 as libc::c_int as usize];
        chromerightvec[1 as libc::c_int as usize] =
            tmp[2 as libc::c_int as usize] *
                chromeupvec[0 as libc::c_int as usize] -
                tmp[0 as libc::c_int as usize] *
                    chromeupvec[2 as libc::c_int as usize];
        chromerightvec[2 as libc::c_int as usize] =
            tmp[0 as libc::c_int as usize] *
                chromeupvec[1 as libc::c_int as usize] -
                tmp[1 as libc::c_int as usize] *
                    chromeupvec[0 as libc::c_int as usize];
        let mut ilength_1: libc::c_float =
            __tg_sqrt(chromerightvec[0 as libc::c_int as usize] *
                          chromerightvec[0 as libc::c_int as usize] +
                          chromerightvec[1 as libc::c_int as usize] *
                              chromerightvec[1 as libc::c_int as usize] +
                          chromerightvec[2 as libc::c_int as usize] *
                              chromerightvec[2 as libc::c_int as usize]);
        if ilength_1 != 0. { ilength_1 = 1.0f32 / ilength_1 }
        chromerightvec[0 as libc::c_int as usize] *= ilength_1;
        chromerightvec[1 as libc::c_int as usize] *= ilength_1;
        chromerightvec[2 as libc::c_int as usize] *= ilength_1;
        Matrix3x4_VectorIRotate(g_studio.bonestransform[bone as
                                                            usize].as_mut_ptr()
                                    as *const [vec_t; 4],
                                chromeupvec.as_mut_ptr() as
                                    *const libc::c_float,
                                g_studio.chromeup[bone as
                                                      usize].as_mut_ptr());
        Matrix3x4_VectorIRotate(g_studio.bonestransform[bone as
                                                            usize].as_mut_ptr()
                                    as *const [vec_t; 4],
                                chromerightvec.as_mut_ptr() as
                                    *const libc::c_float,
                                g_studio.chromeright[bone as
                                                         usize].as_mut_ptr());
        g_studio.chromeage[bone as usize] = g_studio.framecount
    }
    // calc s coord
    n =
        *normal.offset(0 as libc::c_int as isize) *
            g_studio.chromeright[bone as usize][0 as libc::c_int as usize] +
            *normal.offset(1 as libc::c_int as isize) *
                g_studio.chromeright[bone as usize][1 as libc::c_int as usize]
            +
            *normal.offset(2 as libc::c_int as isize) *
                g_studio.chromeright[bone as
                                         usize][2 as libc::c_int as usize];
    *pchrome.offset(0 as libc::c_int as isize) = (n + 1.0f32) * 32.0f32;
    // calc t coord
    n =
        *normal.offset(0 as libc::c_int as isize) *
            g_studio.chromeup[bone as usize][0 as libc::c_int as usize] +
            *normal.offset(1 as libc::c_int as isize) *
                g_studio.chromeup[bone as usize][1 as libc::c_int as usize] +
            *normal.offset(2 as libc::c_int as isize) *
                g_studio.chromeup[bone as usize][2 as libc::c_int as usize];
    *pchrome.offset(1 as libc::c_int as isize) = (n + 1.0f32) * 32.0f32;
}
/*
====================
StudioCalcAttachments

====================
*/
unsafe extern "C" fn R_StudioCalcAttachments() {
    let mut pAtt: *mut mstudioattachment_t = 0 as *mut mstudioattachment_t;
    let mut i: libc::c_int = 0;
    // calculate attachment points
    pAtt =
        (m_pStudioHeader as
             *mut byte).offset((*m_pStudioHeader).attachmentindex as isize) as
            *mut mstudioattachment_t;
    i = 0 as libc::c_int;
    while i <
              (if (64 as libc::c_int) < (*m_pStudioHeader).numattachments {
                   64 as libc::c_int
               } else { (*m_pStudioHeader).numattachments }) {
        Matrix3x4_VectorTransform(g_studio.lighttransform[(*pAtt.offset(i as
                                                                            isize)).bone
                                                              as
                                                              usize].as_mut_ptr()
                                      as *const [vec_t; 4],
                                  (*pAtt.offset(i as isize)).org.as_mut_ptr()
                                      as *const libc::c_float,
                                  (*RI.currententity).attachment[i as
                                                                     usize].as_mut_ptr());
        i += 1
    };
}
/*
===============
pfnStudioSetupModel

===============
*/
unsafe extern "C" fn R_StudioSetupModel(mut bodypart: libc::c_int,
                                        mut ppbodypart:
                                            *mut *mut libc::c_void,
                                        mut ppsubmodel:
                                            *mut *mut libc::c_void) {
    let mut index: libc::c_int = 0;
    if bodypart > (*m_pStudioHeader).numbodyparts {
        bodypart = 0 as libc::c_int
    }
    m_pBodyPart =
        ((m_pStudioHeader as
              *mut byte).offset((*m_pStudioHeader).bodypartindex as isize) as
             *mut mstudiobodyparts_t).offset(bodypart as isize);
    index = (*RI.currententity).curstate.body / (*m_pBodyPart).base;
    index = index % (*m_pBodyPart).nummodels;
    m_pSubModel =
        ((m_pStudioHeader as
              *mut byte).offset((*m_pBodyPart).modelindex as isize) as
             *mut mstudiomodel_t).offset(index as isize);
    if !ppbodypart.is_null() {
        *ppbodypart = m_pBodyPart as *mut libc::c_void
    }
    if !ppsubmodel.is_null() {
        *ppsubmodel = m_pSubModel as *mut libc::c_void
    };
}
/*
===============
R_StudioCheckBBox

===============
*/
unsafe extern "C" fn R_StudioCheckBBox() -> libc::c_int {
    if RI.currententity.is_null() || RI.currentmodel.is_null() {
        return false_0 as libc::c_int
    }
    return R_StudioComputeBBox(0 as *mut vec3_t) as libc::c_int;
}
/*
===============
R_StudioDynamicLight

===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_StudioDynamicLight(mut ent: *mut cl_entity_t,
                                              mut plight: *mut alight_t) {
    let mut mv: *mut movevars_t =
        gEngfuncs.pfnGetMoveVars.expect("non-null function pointer")();
    let mut lightDir: vec3_t = [0.; 3];
    let mut vecSrc: vec3_t = [0.; 3];
    let mut vecEnd: vec3_t = [0.; 3];
    let mut origin: vec3_t = [0.; 3];
    let mut dist: vec3_t = [0.; 3];
    let mut finalLight: vec3_t = [0.; 3];
    let mut add: libc::c_float = 0.;
    let mut radius: libc::c_float = 0.;
    let mut total: libc::c_float = 0.;
    let mut light: colorVec = colorVec{r: 0, g: 0, b: 0, a: 0,};
    let mut lnum: uint = 0;
    let mut dl: *mut dlight_t = 0 as *mut dlight_t;
    if plight.is_null() || ent.is_null() || (*ent).model.is_null() { return }
    if RI.drawWorld as u64 == 0 || (*r_fullbright).value != 0. ||
           (*ent).curstate.effects as libc::c_uint &
               (1 as libc::c_uint) << 27 as libc::c_int != 0 {
        (*plight).shadelight = 0 as libc::c_int;
        (*plight).ambientlight = 192 as libc::c_int;
        *(*plight).plightvec.offset(0 as libc::c_int as isize) = 0.0f32;
        *(*plight).plightvec.offset(1 as libc::c_int as isize) = 0.0f32;
        *(*plight).plightvec.offset(2 as libc::c_int as isize) = -1.0f32;
        (*plight).color[0 as libc::c_int as usize] = 1.0f32;
        (*plight).color[1 as libc::c_int as usize] = 1.0f32;
        (*plight).color[2 as libc::c_int as usize] = 1.0f32;
        return
    }
    // determine plane to get lightvalues from: ceil or floor
    if (*ent).curstate.effects & 16 as libc::c_int != 0 {
        lightDir[0 as libc::c_int as usize] = 0.0f32;
        lightDir[1 as libc::c_int as usize] = 0.0f32;
        lightDir[2 as libc::c_int as usize] = 1.0f32
    } else {
        lightDir[0 as libc::c_int as usize] = 0.0f32;
        lightDir[1 as libc::c_int as usize] = 0.0f32;
        lightDir[2 as libc::c_int as usize] = -1.0f32
    }
    origin[0 as libc::c_int as usize] =
        (*ent).origin[0 as libc::c_int as usize];
    origin[1 as libc::c_int as usize] =
        (*ent).origin[1 as libc::c_int as usize];
    origin[2 as libc::c_int as usize] =
        (*ent).origin[2 as libc::c_int as usize];
    vecSrc[0 as libc::c_int as usize] = origin[0 as libc::c_int as usize];
    vecSrc[1 as libc::c_int as usize] = origin[1 as libc::c_int as usize];
    vecSrc[2 as libc::c_int as usize] =
        origin[2 as libc::c_int as usize] -
            lightDir[2 as libc::c_int as usize] * 8.0f32;
    light.a = 0 as libc::c_int as libc::c_uint;
    light.b = light.a;
    light.g = light.b;
    light.r = light.g;
    if (*mv).skycolor_r + (*mv).skycolor_g + (*mv).skycolor_b !=
           0 as libc::c_int as libc::c_float {
        let mut psurf: *mut msurface_t = 0 as *mut msurface_t;
        let mut trace: pmtrace_t =
            pmtrace_t{allsolid: false_0,
                      startsolid: false_0,
                      inopen: false_0,
                      inwater: false_0,
                      fraction: 0.,
                      endpos: [0.; 3],
                      plane: pmplane_t{normal: [0.; 3], dist: 0.,},
                      ent: 0,
                      deltavelocity: [0.; 3],
                      hitgroup: 0,};
        if Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(26
                                                                                                                     as
                                                                                                                     libc::c_int,
                                                                                                                 0
                                                                                                                     as
                                                                                                                     libc::c_int)
               & (1 as libc::c_int) << 0 as libc::c_int != 0 {
            vecEnd[0 as libc::c_int as usize] =
                origin[0 as libc::c_int as usize] -
                    (*mv).skyvec_x * 65536.0f32;
            vecEnd[1 as libc::c_int as usize] =
                origin[1 as libc::c_int as usize] -
                    (*mv).skyvec_y * 65536.0f32;
            vecEnd[2 as libc::c_int as usize] =
                origin[2 as libc::c_int as usize] -
                    (*mv).skyvec_z * 65536.0f32
        } else {
            vecEnd[0 as libc::c_int as usize] =
                origin[0 as libc::c_int as usize] -
                    (*mv).skyvec_x * 8192.0f32;
            vecEnd[1 as libc::c_int as usize] =
                origin[1 as libc::c_int as usize] -
                    (*mv).skyvec_y * 8192.0f32;
            vecEnd[2 as libc::c_int as usize] =
                origin[2 as libc::c_int as usize] - (*mv).skyvec_z * 8192.0f32
        }
        trace =
            gEngfuncs.CL_TraceLine.expect("non-null function pointer")(vecSrc.as_mut_ptr(),
                                                                       vecEnd.as_mut_ptr(),
                                                                       0x8 as
                                                                           libc::c_int);
        if trace.ent > 0 as libc::c_int {
            psurf =
                gEngfuncs.EV_TraceSurface.expect("non-null function pointer")(trace.ent,
                                                                              vecSrc.as_mut_ptr(),
                                                                              vecEnd.as_mut_ptr())
        } else {
            psurf =
                gEngfuncs.EV_TraceSurface.expect("non-null function pointer")(0
                                                                                  as
                                                                                  libc::c_int,
                                                                              vecSrc.as_mut_ptr(),
                                                                              vecEnd.as_mut_ptr())
        }
        if (*(*ent).model).flags as libc::c_uint &
               (1 as libc::c_uint) << 10 as libc::c_int != 0 ||
               !psurf.is_null() &&
                   (*psurf).flags as libc::c_uint &
                       (1 as libc::c_uint) << 2 as libc::c_int != 0 {
            lightDir[0 as libc::c_int as usize] = (*mv).skyvec_x;
            lightDir[1 as libc::c_int as usize] = (*mv).skyvec_y;
            lightDir[2 as libc::c_int as usize] = (*mv).skyvec_z;
            light.r =
                gEngfuncs.LightToTexGamma.expect("non-null function pointer")(if (*mv).skycolor_r
                                                                                     >=
                                                                                     0
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_float
                                                                                 {
                                                                                  if (*mv).skycolor_r
                                                                                         <
                                                                                         255
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_float
                                                                                     {
                                                                                      (*mv).skycolor_r
                                                                                  } else {
                                                                                      255
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_float
                                                                                  }
                                                                              } else {
                                                                                  0
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_float
                                                                              }
                                                                                  as
                                                                                  byte)
                    as libc::c_uint;
            light.g =
                gEngfuncs.LightToTexGamma.expect("non-null function pointer")(if (*mv).skycolor_g
                                                                                     >=
                                                                                     0
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_float
                                                                                 {
                                                                                  if (*mv).skycolor_g
                                                                                         <
                                                                                         255
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_float
                                                                                     {
                                                                                      (*mv).skycolor_g
                                                                                  } else {
                                                                                      255
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_float
                                                                                  }
                                                                              } else {
                                                                                  0
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_float
                                                                              }
                                                                                  as
                                                                                  byte)
                    as libc::c_uint;
            light.b =
                gEngfuncs.LightToTexGamma.expect("non-null function pointer")(if (*mv).skycolor_b
                                                                                     >=
                                                                                     0
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_float
                                                                                 {
                                                                                  if (*mv).skycolor_b
                                                                                         <
                                                                                         255
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_float
                                                                                     {
                                                                                      (*mv).skycolor_b
                                                                                  } else {
                                                                                      255
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_float
                                                                                  }
                                                                              } else {
                                                                                  0
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_float
                                                                              }
                                                                                  as
                                                                                  byte)
                    as libc::c_uint
        }
    }
    if light.r.wrapping_add(light.g).wrapping_add(light.b) <
           16 as libc::c_int as libc::c_uint {
        // TESTTEST
        let mut gcolor: colorVec = colorVec{r: 0, g: 0, b: 0, a: 0,};
        let mut grad: [libc::c_float; 4] = [0.; 4];
        vecEnd[0 as libc::c_int as usize] =
            lightDir[0 as libc::c_int as usize] * 2048.0f32;
        vecEnd[1 as libc::c_int as usize] =
            lightDir[1 as libc::c_int as usize] * 2048.0f32;
        vecEnd[2 as libc::c_int as usize] =
            lightDir[2 as libc::c_int as usize] * 2048.0f32;
        vecEnd[0 as libc::c_int as usize] =
            vecEnd[0 as libc::c_int as usize] +
                vecSrc[0 as libc::c_int as usize];
        vecEnd[1 as libc::c_int as usize] =
            vecEnd[1 as libc::c_int as usize] +
                vecSrc[1 as libc::c_int as usize];
        vecEnd[2 as libc::c_int as usize] =
            vecEnd[2 as libc::c_int as usize] +
                vecSrc[2 as libc::c_int as usize];
        light =
            R_LightVec(vecSrc.as_mut_ptr() as *const vec_t,
                       vecEnd.as_mut_ptr() as *const vec_t,
                       g_studio.lightspot.as_mut_ptr(),
                       g_studio.lightvec.as_mut_ptr());
        if g_studio.lightvec[0 as libc::c_int as usize] == 0.0f32 &&
               g_studio.lightvec[1 as libc::c_int as usize] == 0.0f32 &&
               g_studio.lightvec[2 as libc::c_int as usize] == 0.0f32 {
            vecSrc[0 as libc::c_int as usize] -= 16.0f32;
            vecSrc[1 as libc::c_int as usize] -= 16.0f32;
            vecEnd[0 as libc::c_int as usize] -= 16.0f32;
            vecEnd[1 as libc::c_int as usize] -= 16.0f32;
            gcolor =
                R_LightVec(vecSrc.as_mut_ptr() as *const vec_t,
                           vecEnd.as_mut_ptr() as *const vec_t,
                           0 as *mut vec_t, 0 as *mut vec_t);
            grad[0 as libc::c_int as usize] =
                gcolor.r.wrapping_add(gcolor.g).wrapping_add(gcolor.b) as
                    libc::c_float / 768.0f32;
            vecSrc[0 as libc::c_int as usize] += 32.0f32;
            vecEnd[0 as libc::c_int as usize] += 32.0f32;
            gcolor =
                R_LightVec(vecSrc.as_mut_ptr() as *const vec_t,
                           vecEnd.as_mut_ptr() as *const vec_t,
                           0 as *mut vec_t, 0 as *mut vec_t);
            grad[1 as libc::c_int as usize] =
                gcolor.r.wrapping_add(gcolor.g).wrapping_add(gcolor.b) as
                    libc::c_float / 768.0f32;
            vecSrc[1 as libc::c_int as usize] += 32.0f32;
            vecEnd[1 as libc::c_int as usize] += 32.0f32;
            gcolor =
                R_LightVec(vecSrc.as_mut_ptr() as *const vec_t,
                           vecEnd.as_mut_ptr() as *const vec_t,
                           0 as *mut vec_t, 0 as *mut vec_t);
            grad[2 as libc::c_int as usize] =
                gcolor.r.wrapping_add(gcolor.g).wrapping_add(gcolor.b) as
                    libc::c_float / 768.0f32;
            vecSrc[0 as libc::c_int as usize] -= 32.0f32;
            vecEnd[0 as libc::c_int as usize] -= 32.0f32;
            gcolor =
                R_LightVec(vecSrc.as_mut_ptr() as *const vec_t,
                           vecEnd.as_mut_ptr() as *const vec_t,
                           0 as *mut vec_t, 0 as *mut vec_t);
            grad[3 as libc::c_int as usize] =
                gcolor.r.wrapping_add(gcolor.g).wrapping_add(gcolor.b) as
                    libc::c_float / 768.0f32;
            lightDir[0 as libc::c_int as usize] =
                grad[0 as libc::c_int as usize] -
                    grad[1 as libc::c_int as usize] -
                    grad[2 as libc::c_int as usize] +
                    grad[3 as libc::c_int as usize];
            lightDir[1 as libc::c_int as usize] =
                grad[1 as libc::c_int as usize] +
                    grad[0 as libc::c_int as usize] -
                    grad[2 as libc::c_int as usize] -
                    grad[3 as libc::c_int as usize];
            let mut ilength: libc::c_float =
                __tg_sqrt(lightDir[0 as libc::c_int as usize] *
                              lightDir[0 as libc::c_int as usize] +
                              lightDir[1 as libc::c_int as usize] *
                                  lightDir[1 as libc::c_int as usize] +
                              lightDir[2 as libc::c_int as usize] *
                                  lightDir[2 as libc::c_int as usize]);
            if ilength != 0. { ilength = 1.0f32 / ilength }
            lightDir[0 as libc::c_int as usize] *= ilength;
            lightDir[1 as libc::c_int as usize] *= ilength;
            lightDir[2 as libc::c_int as usize] *= ilength
        } else {
            lightDir[0 as libc::c_int as usize] =
                g_studio.lightvec[0 as libc::c_int as usize];
            lightDir[1 as libc::c_int as usize] =
                g_studio.lightvec[1 as libc::c_int as usize];
            lightDir[2 as libc::c_int as usize] =
                g_studio.lightvec[2 as libc::c_int as usize]
        }
    }
    finalLight[0 as libc::c_int as usize] = light.r as vec_t;
    finalLight[1 as libc::c_int as usize] = light.g as vec_t;
    finalLight[2 as libc::c_int as usize] = light.b as vec_t;
    (*ent).cvFloorColor = light;
    total =
        if (if light.r > light.g { light.r } else { light.g }) > light.b {
            if light.r > light.g { light.r } else { light.g }
        } else { light.b } as libc::c_float;
    if total == 0.0f32 { total = 1.0f32 }
    // scale lightdir by light intentsity
    lightDir[0 as libc::c_int as usize] =
        lightDir[0 as libc::c_int as usize] * total;
    lightDir[1 as libc::c_int as usize] =
        lightDir[1 as libc::c_int as usize] * total;
    lightDir[2 as libc::c_int as usize] =
        lightDir[2 as libc::c_int as usize] * total;
    lnum = 0 as libc::c_int as uint;
    while lnum < 32 as libc::c_int as libc::c_uint {
        dl =
            gEngfuncs.GetDynamicLight.expect("non-null function pointer")(lnum
                                                                              as
                                                                              libc::c_int);
        if !(((*dl).die as libc::c_double) < g_studio.time ||
                 (*r_dynamic).value == 0.) {
            dist[0 as libc::c_int as usize] =
                (*ent).origin[0 as libc::c_int as usize] -
                    (*dl).origin[0 as libc::c_int as usize];
            dist[1 as libc::c_int as usize] =
                (*ent).origin[1 as libc::c_int as usize] -
                    (*dl).origin[1 as libc::c_int as usize];
            dist[2 as libc::c_int as usize] =
                (*ent).origin[2 as libc::c_int as usize] -
                    (*dl).origin[2 as libc::c_int as usize];
            radius =
                __tg_sqrt(dist[0 as libc::c_int as usize] *
                              dist[0 as libc::c_int as usize] +
                              dist[1 as libc::c_int as usize] *
                                  dist[1 as libc::c_int as usize] +
                              dist[2 as libc::c_int as usize] *
                                  dist[2 as libc::c_int as usize]);
            add = (*dl).radius - radius;
            if add > 0.0f32 {
                total += add;
                if radius > 1.0f32 {
                    dist[0 as libc::c_int as usize] =
                        dist[0 as libc::c_int as usize] * (add / radius);
                    dist[1 as libc::c_int as usize] =
                        dist[1 as libc::c_int as usize] * (add / radius);
                    dist[2 as libc::c_int as usize] =
                        dist[2 as libc::c_int as usize] * (add / radius)
                } else {
                    dist[0 as libc::c_int as usize] =
                        dist[0 as libc::c_int as usize] * add;
                    dist[1 as libc::c_int as usize] =
                        dist[1 as libc::c_int as usize] * add;
                    dist[2 as libc::c_int as usize] =
                        dist[2 as libc::c_int as usize] * add
                }
                lightDir[0 as libc::c_int as usize] =
                    lightDir[0 as libc::c_int as usize] +
                        dist[0 as libc::c_int as usize];
                lightDir[1 as libc::c_int as usize] =
                    lightDir[1 as libc::c_int as usize] +
                        dist[1 as libc::c_int as usize];
                lightDir[2 as libc::c_int as usize] =
                    lightDir[2 as libc::c_int as usize] +
                        dist[2 as libc::c_int as usize];
                finalLight[0 as libc::c_int as usize] +=
                    gEngfuncs.LightToTexGamma.expect("non-null function pointer")((*dl).color.r)
                        as libc::c_int as libc::c_float * (add / 256.0f32) *
                        2.0f32;
                finalLight[1 as libc::c_int as usize] +=
                    gEngfuncs.LightToTexGamma.expect("non-null function pointer")((*dl).color.g)
                        as libc::c_int as libc::c_float * (add / 256.0f32) *
                        2.0f32;
                finalLight[2 as libc::c_int as usize] +=
                    gEngfuncs.LightToTexGamma.expect("non-null function pointer")((*dl).color.b)
                        as libc::c_int as libc::c_float * (add / 256.0f32) *
                        2.0f32
            }
        }
        lnum = lnum.wrapping_add(1)
    }
    if (*(*ent).model).flags as libc::c_uint &
           (1 as libc::c_uint) << 8 as libc::c_int != 0 {
        add = 0.6f32
    } else { add = 0.9f32 }
    lightDir[0 as libc::c_int as usize] =
        lightDir[0 as libc::c_int as usize] * add;
    lightDir[1 as libc::c_int as usize] =
        lightDir[1 as libc::c_int as usize] * add;
    lightDir[2 as libc::c_int as usize] =
        lightDir[2 as libc::c_int as usize] * add;
    (*plight).shadelight =
        __tg_sqrt(lightDir[0 as libc::c_int as usize] *
                      lightDir[0 as libc::c_int as usize] +
                      lightDir[1 as libc::c_int as usize] *
                          lightDir[1 as libc::c_int as usize] +
                      lightDir[2 as libc::c_int as usize] *
                          lightDir[2 as libc::c_int as usize]) as libc::c_int;
    (*plight).ambientlight =
        (total - (*plight).shadelight as libc::c_float) as libc::c_int;
    total =
        if (if finalLight[0 as libc::c_int as usize] >
                   finalLight[1 as libc::c_int as usize] {
                finalLight[0 as libc::c_int as usize]
            } else { finalLight[1 as libc::c_int as usize] }) >
               finalLight[2 as libc::c_int as usize] {
            if finalLight[0 as libc::c_int as usize] >
                   finalLight[1 as libc::c_int as usize] {
                finalLight[0 as libc::c_int as usize]
            } else { finalLight[1 as libc::c_int as usize] }
        } else { finalLight[2 as libc::c_int as usize] };
    if total > 0.0f32 {
        (*plight).color[0 as libc::c_int as usize] =
            finalLight[0 as libc::c_int as usize] * (1.0f32 / total);
        (*plight).color[1 as libc::c_int as usize] =
            finalLight[1 as libc::c_int as usize] * (1.0f32 / total);
        (*plight).color[2 as libc::c_int as usize] =
            finalLight[2 as libc::c_int as usize] * (1.0f32 / total)
    } else {
        (*plight).color[0 as libc::c_int as usize] = 1.0f32;
        (*plight).color[1 as libc::c_int as usize] = 1.0f32;
        (*plight).color[2 as libc::c_int as usize] = 1.0f32
    }
    if (*plight).ambientlight > 128 as libc::c_int {
        (*plight).ambientlight = 128 as libc::c_int
    }
    if (*plight).ambientlight + (*plight).shadelight > 255 as libc::c_int {
        (*plight).shadelight = 255 as libc::c_int - (*plight).ambientlight
    }
    let mut ilength_0: libc::c_float =
        __tg_sqrt(lightDir[0 as libc::c_int as usize] *
                      lightDir[0 as libc::c_int as usize] +
                      lightDir[1 as libc::c_int as usize] *
                          lightDir[1 as libc::c_int as usize] +
                      lightDir[2 as libc::c_int as usize] *
                          lightDir[2 as libc::c_int as usize]);
    if ilength_0 != 0. { ilength_0 = 1.0f32 / ilength_0 }
    *(*plight).plightvec.offset(0 as libc::c_int as isize) =
        lightDir[0 as libc::c_int as usize] * ilength_0;
    *(*plight).plightvec.offset(1 as libc::c_int as isize) =
        lightDir[1 as libc::c_int as usize] * ilength_0;
    *(*plight).plightvec.offset(2 as libc::c_int as isize) =
        lightDir[2 as libc::c_int as usize] * ilength_0;
}
/*
===============
pfnStudioEntityLight

===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_StudioEntityLight(mut lightinfo: *mut alight_t) {
    let mut lnum: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut minstrength: libc::c_float = 0.;
    let mut dist2: libc::c_float = 0.;
    let mut f: libc::c_float = 0.;
    let mut r2: libc::c_float = 0.;
    let mut lstrength: [libc::c_float; 4] = [0.; 4];
    let mut ent: *mut cl_entity_t = RI.currententity;
    let mut mid: vec3_t = [0.; 3];
    let mut origin: vec3_t = [0.; 3];
    let mut pos: vec3_t = [0.; 3];
    let mut el: *mut dlight_t = 0 as *mut dlight_t;
    g_studio.numlocallights = 0 as libc::c_int;
    if ent.is_null() || (*r_dynamic).value == 0. { return }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        lstrength[i as usize] = 0 as libc::c_int as libc::c_float;
        i += 1
    }
    Matrix3x4_OriginFromMatrix(g_studio.rotationmatrix.as_mut_ptr() as
                                   *const [vec_t; 4], origin.as_mut_ptr());
    dist2 = 1000000.0f32;
    k = 0 as libc::c_int;
    lnum = 0 as libc::c_int;
    while lnum < 64 as libc::c_int {
        el =
            gEngfuncs.GetEntityLight.expect("non-null function pointer")(lnum);
        if !(((*el).die as libc::c_double) < g_studio.time ||
                 (*el).radius <= 0.0f32) {
            if (*el).key & 0xfff as libc::c_int == (*ent).index {
                let mut att: libc::c_int =
                    (*el).key >> 12 as libc::c_int & 0xf as libc::c_int;
                if att != 0 {
                    (*el).origin[0 as libc::c_int as usize] =
                        (*ent).attachment[att as
                                              usize][0 as libc::c_int as
                                                         usize];
                    (*el).origin[1 as libc::c_int as usize] =
                        (*ent).attachment[att as
                                              usize][1 as libc::c_int as
                                                         usize];
                    (*el).origin[2 as libc::c_int as usize] =
                        (*ent).attachment[att as
                                              usize][2 as libc::c_int as
                                                         usize]
                } else {
                    (*el).origin[0 as libc::c_int as usize] =
                        (*ent).origin[0 as libc::c_int as usize];
                    (*el).origin[1 as libc::c_int as usize] =
                        (*ent).origin[1 as libc::c_int as usize];
                    (*el).origin[2 as libc::c_int as usize] =
                        (*ent).origin[2 as libc::c_int as usize]
                }
            }
            pos[0 as libc::c_int as usize] =
                (*el).origin[0 as libc::c_int as usize];
            pos[1 as libc::c_int as usize] =
                (*el).origin[1 as libc::c_int as usize];
            pos[2 as libc::c_int as usize] =
                (*el).origin[2 as libc::c_int as usize];
            mid[0 as libc::c_int as usize] =
                origin[0 as libc::c_int as usize] -
                    (*el).origin[0 as libc::c_int as usize];
            mid[1 as libc::c_int as usize] =
                origin[1 as libc::c_int as usize] -
                    (*el).origin[1 as libc::c_int as usize];
            mid[2 as libc::c_int as usize] =
                origin[2 as libc::c_int as usize] -
                    (*el).origin[2 as libc::c_int as usize];
            f =
                mid[0 as libc::c_int as usize] *
                    mid[0 as libc::c_int as usize] +
                    mid[1 as libc::c_int as usize] *
                        mid[1 as libc::c_int as usize] +
                    mid[2 as libc::c_int as usize] *
                        mid[2 as libc::c_int as usize];
            r2 = (*el).radius * (*el).radius;
            if f > r2 { minstrength = r2 / f } else { minstrength = 1.0f32 }
            if minstrength > 0.05f32 {
                if g_studio.numlocallights >= 4 as libc::c_int {
                    j = 0 as libc::c_int;
                    k = -(1 as libc::c_int);
                    while j < g_studio.numlocallights {
                        if lstrength[j as usize] < dist2 &&
                               lstrength[j as usize] < minstrength {
                            dist2 = lstrength[j as usize];
                            k = j
                        }
                        j += 1
                    }
                } else { k = g_studio.numlocallights }
                if k != -(1 as libc::c_int) {
                    g_studio.locallightcolor[k as usize].r =
                        gEngfuncs.LightToTexGamma.expect("non-null function pointer")((*el).color.r);
                    g_studio.locallightcolor[k as usize].g =
                        gEngfuncs.LightToTexGamma.expect("non-null function pointer")((*el).color.g);
                    g_studio.locallightcolor[k as usize].b =
                        gEngfuncs.LightToTexGamma.expect("non-null function pointer")((*el).color.b);
                    g_studio.locallightR2[k as usize] = r2;
                    g_studio.locallight[k as usize] = el;
                    lstrength[k as usize] = minstrength;
                    if k >= g_studio.numlocallights {
                        g_studio.numlocallights = k + 1 as libc::c_int
                    }
                }
            }
        }
        lnum += 1
    };
}
/*
===============
R_StudioSetupLighting

===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_StudioSetupLighting(mut plight: *mut alight_t) {
    let mut scale: libc::c_float = 1.0f32;
    let mut i: libc::c_int = 0;
    if m_pStudioHeader.is_null() || plight.is_null() { return }
    if !RI.currententity.is_null() {
        scale = (*RI.currententity).curstate.scale
    }
    g_studio.ambientlight = (*plight).ambientlight as libc::c_float;
    g_studio.shadelight = (*plight).shadelight as libc::c_float;
    g_studio.lightvec[0 as libc::c_int as usize] =
        *(*plight).plightvec.offset(0 as libc::c_int as isize);
    g_studio.lightvec[1 as libc::c_int as usize] =
        *(*plight).plightvec.offset(1 as libc::c_int as isize);
    g_studio.lightvec[2 as libc::c_int as usize] =
        *(*plight).plightvec.offset(2 as libc::c_int as isize);
    i = 0 as libc::c_int;
    while i < (*m_pStudioHeader).numbones {
        Matrix3x4_VectorIRotate(g_studio.lighttransform[i as
                                                            usize].as_mut_ptr()
                                    as *const [vec_t; 4],
                                (*plight).plightvec as *const libc::c_float,
                                g_studio.blightvec[i as usize].as_mut_ptr());
        if scale > 1.0f32 {
            let mut ilength: libc::c_float =
                __tg_sqrt(g_studio.blightvec[i as
                                                 usize][0 as libc::c_int as
                                                            usize] *
                              g_studio.blightvec[i as
                                                     usize][0 as libc::c_int
                                                                as usize] +
                              g_studio.blightvec[i as
                                                     usize][1 as libc::c_int
                                                                as usize] *
                                  g_studio.blightvec[i as
                                                         usize][1 as
                                                                    libc::c_int
                                                                    as usize]
                              +
                              g_studio.blightvec[i as
                                                     usize][2 as libc::c_int
                                                                as usize] *
                                  g_studio.blightvec[i as
                                                         usize][2 as
                                                                    libc::c_int
                                                                    as
                                                                    usize]);
            if ilength != 0. { ilength = 1.0f32 / ilength }
            g_studio.blightvec[i as usize][0 as libc::c_int as usize] *=
                ilength;
            g_studio.blightvec[i as usize][1 as libc::c_int as usize] *=
                ilength;
            g_studio.blightvec[i as usize][2 as libc::c_int as usize] *=
                ilength
        }
        i += 1
        // in case model may be scaled
    }
    g_studio.lightcolor[0 as libc::c_int as usize] =
        (*plight).color[0 as libc::c_int as usize];
    g_studio.lightcolor[1 as libc::c_int as usize] =
        (*plight).color[1 as libc::c_int as usize];
    g_studio.lightcolor[2 as libc::c_int as usize] =
        (*plight).color[2 as libc::c_int as usize];
}
/*
===============
R_StudioLighting

===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_StudioLighting(mut lv: *mut libc::c_float,
                                          mut bone: libc::c_int,
                                          mut flags: libc::c_int,
                                          mut normal: *mut vec_t) {
    let mut illum: libc::c_float = 0.; // -1 colinear, 1 opposite
    if flags & 0x4 as libc::c_int != 0 { *lv = 1.0f32; return }
    illum = g_studio.ambientlight;
    if flags & 0x1 as libc::c_int != 0 {
        illum += g_studio.shadelight * 0.8f32
    } else {
        let mut r: libc::c_float = 0.;
        let mut lightcos: libc::c_float = 0.;
        if bone != -(1 as libc::c_int) {
            lightcos =
                *normal.offset(0 as libc::c_int as isize) *
                    g_studio.blightvec[bone as
                                           usize][0 as libc::c_int as usize] +
                    *normal.offset(1 as libc::c_int as isize) *
                        g_studio.blightvec[bone as
                                               usize][1 as libc::c_int as
                                                          usize] +
                    *normal.offset(2 as libc::c_int as isize) *
                        g_studio.blightvec[bone as
                                               usize][2 as libc::c_int as
                                                          usize]
        } else {
            lightcos =
                *normal.offset(0 as libc::c_int as isize) *
                    g_studio.lightvec[0 as libc::c_int as usize] +
                    *normal.offset(1 as libc::c_int as isize) *
                        g_studio.lightvec[1 as libc::c_int as usize] +
                    *normal.offset(2 as libc::c_int as isize) *
                        g_studio.lightvec[2 as libc::c_int as usize]
        }
        if lightcos > 1.0f32 { lightcos = 1.0f32 }
        illum += g_studio.shadelight;
        r = 1.495f32;
        // do modified hemispherical lighting
        if r <= 1.0f32 {
            r += 1.0f32;
            lightcos = (r - 1.0f32 - lightcos) / r;
            if lightcos > 0.0f32 { illum += g_studio.shadelight * lightcos }
        } else {
            lightcos = (lightcos + (r - 1.0f32)) / r;
            if lightcos > 0.0f32 { illum -= g_studio.shadelight * lightcos }
        }
        illum = if illum > 0.0f32 { illum } else { 0.0f32 }
    }
    illum = if illum < 255.0f32 { illum } else { 255.0f32 };
    *lv = illum * (1.0f32 / 255.0f32);
}
/*
====================
R_LightLambert

====================
*/
#[no_mangle]
pub unsafe extern "C" fn R_LightLambert(mut light: *mut vec4_t,
                                        mut normal: *mut vec_t,
                                        mut color: *mut vec_t,
                                        mut out: *mut byte) {
    let mut finalLight: vec3_t = [0.; 3];
    let mut localLight: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    finalLight[0 as libc::c_int as usize] =
        *color.offset(0 as libc::c_int as isize);
    finalLight[1 as libc::c_int as usize] =
        *color.offset(1 as libc::c_int as isize);
    finalLight[2 as libc::c_int as usize] =
        *color.offset(2 as libc::c_int as isize);
    i = 0 as libc::c_int;
    while i < g_studio.numlocallights {
        let mut r: libc::c_float = 0.;
        let mut r2: libc::c_float = 0.;
        if tr.fFlipViewModel as u64 != 0 {
            r =
                *normal.offset(0 as libc::c_int as isize) *
                    (*light.offset(i as isize))[0 as libc::c_int as usize] +
                    *normal.offset(1 as libc::c_int as isize) *
                        (*light.offset(i as isize))[1 as libc::c_int as usize]
                    +
                    *normal.offset(2 as libc::c_int as isize) *
                        (*light.offset(i as isize))[2 as libc::c_int as usize]
        } else {
            r =
                -(*normal.offset(0 as libc::c_int as isize) *
                      (*light.offset(i as isize))[0 as libc::c_int as usize] +
                      *normal.offset(1 as libc::c_int as isize) *
                          (*light.offset(i as
                                             isize))[1 as libc::c_int as
                                                         usize] +
                      *normal.offset(2 as libc::c_int as isize) *
                          (*light.offset(i as
                                             isize))[2 as libc::c_int as
                                                         usize])
        }
        if r > 0.0f32 {
            if (*light.offset(i as isize))[3 as libc::c_int as usize] ==
                   0.0f32 {
                r2 =
                    (*light.offset(i as isize))[0 as libc::c_int as usize] *
                        (*light.offset(i as isize))[0 as libc::c_int as usize]
                        +
                        (*light.offset(i as isize))[1 as libc::c_int as usize]
                            *
                            (*light.offset(i as
                                               isize))[1 as libc::c_int as
                                                           usize] +
                        (*light.offset(i as isize))[2 as libc::c_int as usize]
                            *
                            (*light.offset(i as
                                               isize))[2 as libc::c_int as
                                                           usize];
                if r2 > 0.0f32 {
                    (*light.offset(i as isize))[3 as libc::c_int as usize] =
                        g_studio.locallightR2[i as usize] /
                            (r2 * __tg_sqrt(r2))
                } else {
                    (*light.offset(i as isize))[3 as libc::c_int as usize] =
                        0.0001f32
                }
            }
            localLight[0 as libc::c_int as usize] =
                if g_studio.locallightcolor[i as usize].r as libc::c_int as
                       libc::c_float * r *
                       (*light.offset(i as isize))[3 as libc::c_int as usize]
                       < 255.0f32 {
                    (g_studio.locallightcolor[i as usize].r as libc::c_int as
                         libc::c_float * r) *
                        (*light.offset(i as isize))[3 as libc::c_int as usize]
                } else { 255.0f32 };
            localLight[1 as libc::c_int as usize] =
                if g_studio.locallightcolor[i as usize].g as libc::c_int as
                       libc::c_float * r *
                       (*light.offset(i as isize))[3 as libc::c_int as usize]
                       < 255.0f32 {
                    (g_studio.locallightcolor[i as usize].g as libc::c_int as
                         libc::c_float * r) *
                        (*light.offset(i as isize))[3 as libc::c_int as usize]
                } else { 255.0f32 };
            localLight[2 as libc::c_int as usize] =
                if g_studio.locallightcolor[i as usize].b as libc::c_int as
                       libc::c_float * r *
                       (*light.offset(i as isize))[3 as libc::c_int as usize]
                       < 255.0f32 {
                    (g_studio.locallightcolor[i as usize].b as libc::c_int as
                         libc::c_float * r) *
                        (*light.offset(i as isize))[3 as libc::c_int as usize]
                } else { 255.0f32 };
            localLight[0 as libc::c_int as usize] =
                localLight[0 as libc::c_int as usize] * (1.0f32 / 255.0f32);
            localLight[1 as libc::c_int as usize] =
                localLight[1 as libc::c_int as usize] * (1.0f32 / 255.0f32);
            localLight[2 as libc::c_int as usize] =
                localLight[2 as libc::c_int as usize] * (1.0f32 / 255.0f32);
            finalLight[0 as libc::c_int as usize] =
                if finalLight[0 as libc::c_int as usize] +
                       localLight[0 as libc::c_int as usize] < 1.0f32 {
                    (finalLight[0 as libc::c_int as usize]) +
                        localLight[0 as libc::c_int as usize]
                } else { 1.0f32 };
            finalLight[1 as libc::c_int as usize] =
                if finalLight[1 as libc::c_int as usize] +
                       localLight[1 as libc::c_int as usize] < 1.0f32 {
                    (finalLight[1 as libc::c_int as usize]) +
                        localLight[1 as libc::c_int as usize]
                } else { 1.0f32 };
            finalLight[2 as libc::c_int as usize] =
                if finalLight[2 as libc::c_int as usize] +
                       localLight[2 as libc::c_int as usize] < 1.0f32 {
                    (finalLight[2 as libc::c_int as usize]) +
                        localLight[2 as libc::c_int as usize]
                } else { 1.0f32 }
        }
        i += 1
    }
    *out.offset(0 as libc::c_int as isize) =
        (finalLight[0 as libc::c_int as usize] *
             255 as libc::c_int as libc::c_float) as byte;
    *out.offset(1 as libc::c_int as isize) =
        (finalLight[1 as libc::c_int as usize] *
             255 as libc::c_int as libc::c_float) as byte;
    *out.offset(2 as libc::c_int as isize) =
        (finalLight[2 as libc::c_int as usize] *
             255 as libc::c_int as libc::c_float) as byte;
}
unsafe extern "C" fn R_StudioSetColorBegin(mut ptricmds: *mut libc::c_short,
                                           mut pstudionorms: *mut vec3_t) {
    let mut lv: *mut libc::c_float =
        g_studio.lightvalues[*ptricmds.offset(1 as libc::c_int as isize) as
                                 usize].as_mut_ptr() as *mut libc::c_float;
    let mut color: rgba_t = [0; 4];
    if g_studio.numlocallights != 0 {
        color[3 as libc::c_int as usize] =
            (tr.blend * 255 as libc::c_int as libc::c_float) as byte;
        R_LightLambert(g_studio.lightpos[*ptricmds.offset(0 as libc::c_int as
                                                              isize) as
                                             usize].as_mut_ptr(),
                       (*pstudionorms.offset(*ptricmds.offset(1 as libc::c_int
                                                                  as isize) as
                                                 isize)).as_mut_ptr(), lv,
                       color.as_mut_ptr());
        pglColor4ubv.expect("non-null function pointer")(color.as_mut_ptr());
    } else if (*RI.currententity).curstate.rendermode ==
                  kRenderTransColor as libc::c_int {
        color[3 as libc::c_int as usize] =
            (tr.blend * 255 as libc::c_int as libc::c_float) as byte;
        color[0 as libc::c_int as usize] =
            *(&mut (*RI.currententity).curstate.rendercolor as *mut color24 as
                  *mut byte).offset(0 as libc::c_int as isize);
        color[1 as libc::c_int as usize] =
            *(&mut (*RI.currententity).curstate.rendercolor as *mut color24 as
                  *mut byte).offset(1 as libc::c_int as isize);
        color[2 as libc::c_int as usize] =
            *(&mut (*RI.currententity).curstate.rendercolor as *mut color24 as
                  *mut byte).offset(2 as libc::c_int as isize);
        pglColor4ubv.expect("non-null function pointer")(color.as_mut_ptr());
    } else {
        pglColor4f.expect("non-null function pointer")(*lv.offset(0 as
                                                                      libc::c_int
                                                                      as
                                                                      isize),
                                                       *lv.offset(1 as
                                                                      libc::c_int
                                                                      as
                                                                      isize),
                                                       *lv.offset(2 as
                                                                      libc::c_int
                                                                      as
                                                                      isize),
                                                       tr.blend);
    };
}
unsafe extern "C" fn R_StudioSetColorArray(mut ptricmds: *mut libc::c_short,
                                           mut pstudionorms: *mut vec3_t,
                                           mut color: *mut byte) {
    let mut lv: *mut libc::c_float =
        g_studio.lightvalues[*ptricmds.offset(1 as libc::c_int as isize) as
                                 usize].as_mut_ptr() as *mut libc::c_float;
    *color.offset(3 as libc::c_int as isize) =
        (tr.blend * 255 as libc::c_int as libc::c_float) as byte;
    if g_studio.numlocallights != 0 {
        R_LightLambert(g_studio.lightpos[*ptricmds.offset(0 as libc::c_int as
                                                              isize) as
                                             usize].as_mut_ptr(),
                       (*pstudionorms.offset(*ptricmds.offset(1 as libc::c_int
                                                                  as isize) as
                                                 isize)).as_mut_ptr(), lv,
                       color);
    } else if (*RI.currententity).curstate.rendermode ==
                  kRenderTransColor as libc::c_int {
        *color.offset(0 as libc::c_int as isize) =
            *(&mut (*RI.currententity).curstate.rendercolor as *mut color24 as
                  *mut byte).offset(0 as libc::c_int as isize);
        *color.offset(1 as libc::c_int as isize) =
            *(&mut (*RI.currententity).curstate.rendercolor as *mut color24 as
                  *mut byte).offset(1 as libc::c_int as isize);
        *color.offset(2 as libc::c_int as isize) =
            *(&mut (*RI.currententity).curstate.rendercolor as *mut color24 as
                  *mut byte).offset(2 as libc::c_int as isize)
    } else {
        *color.offset(0 as libc::c_int as isize) =
            (*lv.offset(0 as libc::c_int as isize) *
                 255 as libc::c_int as libc::c_float) as byte;
        *color.offset(1 as libc::c_int as isize) =
            (*lv.offset(1 as libc::c_int as isize) *
                 255 as libc::c_int as libc::c_float) as byte;
        *color.offset(2 as libc::c_int as isize) =
            (*lv.offset(2 as libc::c_int as isize) *
                 255 as libc::c_int as libc::c_float) as byte
    };
}
/*
====================
R_LightStrength

====================
*/
#[no_mangle]
pub unsafe extern "C" fn R_LightStrength(mut bone: libc::c_int,
                                         mut localpos: *mut vec_t,
                                         mut light: *mut vec4_t) {
    let mut i: libc::c_int = 0;
    if g_studio.lightage[bone as usize] != g_studio.framecount {
        i = 0 as libc::c_int;
        while i < g_studio.numlocallights {
            let mut el: *mut dlight_t = g_studio.locallight[i as usize];
            Matrix3x4_VectorITransform(g_studio.lighttransform[bone as
                                                                   usize].as_mut_ptr()
                                           as *const [vec_t; 4],
                                       (*el).origin.as_mut_ptr() as
                                           *const libc::c_float,
                                       g_studio.lightbonepos[bone as
                                                                 usize][i as
                                                                            usize].as_mut_ptr());
            i += 1
        }
        g_studio.lightage[bone as usize] = g_studio.framecount
    }
    i = 0 as libc::c_int;
    while i < g_studio.numlocallights {
        (*light.offset(i as isize))[0 as libc::c_int as usize] =
            *localpos.offset(0 as libc::c_int as isize) -
                g_studio.lightbonepos[bone as
                                          usize][i as
                                                     usize][0 as libc::c_int
                                                                as usize];
        (*light.offset(i as isize))[1 as libc::c_int as usize] =
            *localpos.offset(1 as libc::c_int as isize) -
                g_studio.lightbonepos[bone as
                                          usize][i as
                                                     usize][1 as libc::c_int
                                                                as usize];
        (*light.offset(i as isize))[2 as libc::c_int as usize] =
            *localpos.offset(2 as libc::c_int as isize) -
                g_studio.lightbonepos[bone as
                                          usize][i as
                                                     usize][2 as libc::c_int
                                                                as usize];
        (*light.offset(i as isize))[3 as libc::c_int as usize] = 0.0f32;
        i += 1
    };
}
/*
===============
R_StudioSetupSkin

===============
*/
unsafe extern "C" fn R_StudioSetupSkin(mut ptexturehdr: *mut studiohdr_t,
                                       mut index: libc::c_int) {
    let mut ptexture: *mut mstudiotexture_t = 0 as *mut mstudiotexture_t;
    if g_nForceFaceFlags & 0x2 as libc::c_int != 0 { return }
    if ptexturehdr.is_null() { return }
    // NOTE: user may ignore to call StudioRemapColors and remap_info will be unavailable
    if m_fDoRemap as u64 != 0 {
        ptexture =
            (*gEngfuncs.CL_GetRemapInfoForEntity.expect("non-null function pointer")(RI.currententity)).ptexture
    } // fallback
    if ptexture.is_null() {
        ptexture =
            (ptexturehdr as
                 *mut byte).offset((*ptexturehdr).textureindex as isize) as
                *mut mstudiotexture_t
    }
    if (*r_lightmap).value != 0. && (*r_fullbright).value == 0. {
        GL_Bind(XASH_TEXTURE0 as libc::c_int, tr.whiteTexture as GLenum);
    } else {
        GL_Bind(XASH_TEXTURE0 as libc::c_int,
                (*ptexture.offset(index as isize)).index as GLenum);
    };
}
/*
===============
R_StudioGetTexture

Doesn't changes studio global state at all
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_StudioGetTexture(mut e: *mut cl_entity_t)
 -> *mut mstudiotex_s {
    let mut ptexture: *mut mstudiotexture_t = 0 as *mut mstudiotexture_t;
    let mut phdr: *mut studiohdr_t = 0 as *mut studiohdr_t;
    let mut thdr: *mut studiohdr_t = 0 as *mut studiohdr_t;
    phdr =
        gEngfuncs.Mod_Extradata.expect("non-null function pointer")(mod_studio
                                                                        as
                                                                        libc::c_int,
                                                                    (*e).model)
            as *mut studiohdr_t;
    if phdr.is_null() { return 0 as *mut mstudiotex_s }
    thdr = m_pStudioHeader;
    if thdr.is_null() { return 0 as *mut mstudiotex_s }
    if m_fDoRemap as u64 != 0 {
        ptexture =
            (*gEngfuncs.CL_GetRemapInfoForEntity.expect("non-null function pointer")(e)).ptexture
    } else {
        ptexture =
            (thdr as *mut byte).offset((*thdr).textureindex as isize) as
                *mut mstudiotexture_t
    }
    return ptexture;
}
#[no_mangle]
pub unsafe extern "C" fn R_StudioSetRenderamt(mut iRenderamt: libc::c_int) {
    if RI.currententity.is_null() { return }
    (*RI.currententity).curstate.renderamt = iRenderamt;
    tr.blend = CL_FxBlend(RI.currententity) as libc::c_float / 255.0f32;
}
/*
===============
R_StudioSetCullState

sets true for enable backculling (for left-hand viewmodel)
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_StudioSetCullState(mut iCull: libc::c_int) {
    g_iBackFaceCull = iCull;
}
/*
===============
R_StudioRenderShadow

just a prefab for render shadow
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_StudioRenderShadow(mut iSprite: libc::c_int,
                                              mut p1: *mut libc::c_float,
                                              mut p2: *mut libc::c_float,
                                              mut p3: *mut libc::c_float,
                                              mut p4: *mut libc::c_float) {
    if p1.is_null() || p2.is_null() || p3.is_null() || p4.is_null() { return }
    if TriSpriteTexture(gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(iSprite),
                        0 as libc::c_int) != 0 {
        TriRenderMode(kRenderTransAlpha as libc::c_int);
        TriColor4f(0.0f32, 0.0f32, 0.0f32, 1.0f32);
        pglBegin.expect("non-null function pointer")(0x7 as libc::c_int as
                                                         GLenum);
        pglTexCoord2f.expect("non-null function pointer")(0.0f32, 0.0f32);
        pglVertex3fv.expect("non-null function pointer")(p1);
        pglTexCoord2f.expect("non-null function pointer")(0.0f32, 1.0f32);
        pglVertex3fv.expect("non-null function pointer")(p2);
        pglTexCoord2f.expect("non-null function pointer")(1.0f32, 1.0f32);
        pglVertex3fv.expect("non-null function pointer")(p3);
        pglTexCoord2f.expect("non-null function pointer")(1.0f32, 0.0f32);
        pglVertex3fv.expect("non-null function pointer")(p4);
        pglEnd.expect("non-null function pointer")();
        TriRenderMode(kRenderNormal as libc::c_int);
    };
}
/*
===============
R_StudioMeshCompare

Sorting opaque entities by model type
===============
*/
unsafe extern "C" fn R_StudioMeshCompare(mut a: *const libc::c_void,
                                         mut b: *const libc::c_void)
 -> libc::c_int {
    if (*(a as *const sortedmesh_t)).flags & 0x20 as libc::c_int != 0 {
        return 1 as libc::c_int
    }
    if (*(a as *const sortedmesh_t)).flags & 0x40 as libc::c_int != 0 {
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
/*
===============
R_StudioDrawNormalMesh

generic path
===============
*/
#[inline]
unsafe extern "C" fn R_StudioDrawNormalMesh(mut ptricmds: *mut libc::c_short,
                                            mut pstudionorms: *mut vec3_t,
                                            mut s: libc::c_float,
                                            mut t: libc::c_float) {
    let mut lv: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut i: libc::c_int = 0;
    loop  {
        let fresh6 = ptricmds;
        ptricmds = ptricmds.offset(1);
        i = *fresh6 as libc::c_int;
        if !(i != 0) { break ; }
        if i < 0 as libc::c_int {
            pglBegin.expect("non-null function pointer")(0x6 as libc::c_int as
                                                             GLenum);
            i = -i
        } else {
            pglBegin.expect("non-null function pointer")(0x5 as libc::c_int as
                                                             GLenum);
        }
        while i > 0 as libc::c_int {
            R_StudioSetColorBegin(ptricmds, pstudionorms);
            pglTexCoord2f.expect("non-null function pointer")(*ptricmds.offset(2
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)
                                                                  as
                                                                  libc::c_int
                                                                  as
                                                                  libc::c_float
                                                                  * s,
                                                              *ptricmds.offset(3
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)
                                                                  as
                                                                  libc::c_int
                                                                  as
                                                                  libc::c_float
                                                                  * t);
            pglVertex3fv.expect("non-null function pointer")(g_studio.verts[*ptricmds.offset(0
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 isize)
                                                                                as
                                                                                usize].as_mut_ptr());
            i -= 1;
            ptricmds = ptricmds.offset(4 as libc::c_int as isize)
        }
        pglEnd.expect("non-null function pointer")();
    };
}
/*
===============
R_StudioDrawNormalMesh

generic path
===============
*/
#[inline]
unsafe extern "C" fn R_StudioDrawFloatMesh(mut ptricmds: *mut libc::c_short,
                                           mut pstudionorms: *mut vec3_t) {
    let mut lv: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut i: libc::c_int = 0;
    loop  {
        let fresh7 = ptricmds;
        ptricmds = ptricmds.offset(1);
        i = *fresh7 as libc::c_int;
        if !(i != 0) { break ; }
        if i < 0 as libc::c_int {
            pglBegin.expect("non-null function pointer")(0x6 as libc::c_int as
                                                             GLenum);
            i = -i
        } else {
            pglBegin.expect("non-null function pointer")(0x5 as libc::c_int as
                                                             GLenum);
        }
        while i > 0 as libc::c_int {
            R_StudioSetColorBegin(ptricmds, pstudionorms);
            pglTexCoord2f.expect("non-null function pointer")(HalfToFloat(*ptricmds.offset(2
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               isize)
                                                                              as
                                                                              word),
                                                              HalfToFloat(*ptricmds.offset(3
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               isize)
                                                                              as
                                                                              word));
            pglVertex3fv.expect("non-null function pointer")(g_studio.verts[*ptricmds.offset(0
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 isize)
                                                                                as
                                                                                usize].as_mut_ptr());
            i -= 1;
            ptricmds = ptricmds.offset(4 as libc::c_int as isize)
        }
        pglEnd.expect("non-null function pointer")();
    };
}
/*
===============
R_StudioDrawNormalMesh

generic path
===============
*/
#[inline]
unsafe extern "C" fn R_StudioDrawChromeMesh(mut ptricmds: *mut libc::c_short,
                                            mut pstudionorms: *mut vec3_t,
                                            mut s: libc::c_float,
                                            mut t: libc::c_float,
                                            mut scale: libc::c_float) {
    let mut lv: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut av: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut i: libc::c_int = 0;
    let mut idx: libc::c_int = 0;
    let mut glowShell: qboolean =
        if scale > 0.0f32 {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int } as qboolean;
    let mut vert: vec3_t = [0.; 3];
    loop  {
        let fresh8 = ptricmds;
        ptricmds = ptricmds.offset(1);
        i = *fresh8 as libc::c_int;
        if !(i != 0) { break ; }
        if i < 0 as libc::c_int {
            pglBegin.expect("non-null function pointer")(0x6 as libc::c_int as
                                                             GLenum);
            i = -i
        } else {
            pglBegin.expect("non-null function pointer")(0x5 as libc::c_int as
                                                             GLenum);
        }
        while i > 0 as libc::c_int {
            if glowShell as u64 != 0 {
                let mut clr: *mut color24 =
                    &mut (*RI.currententity).curstate.rendercolor;
                idx =
                    g_studio.normaltable[*ptricmds.offset(0 as libc::c_int as
                                                              isize) as
                                             usize];
                av =
                    g_studio.verts[*ptricmds.offset(0 as libc::c_int as isize)
                                       as usize].as_mut_ptr();
                lv =
                    g_studio.norms[*ptricmds.offset(0 as libc::c_int as isize)
                                       as usize].as_mut_ptr();
                vert[0 as libc::c_int as usize] =
                    *av.offset(0 as libc::c_int as isize) +
                        scale * *lv.offset(0 as libc::c_int as isize);
                vert[1 as libc::c_int as usize] =
                    *av.offset(1 as libc::c_int as isize) +
                        scale * *lv.offset(1 as libc::c_int as isize);
                vert[2 as libc::c_int as usize] =
                    *av.offset(2 as libc::c_int as isize) +
                        scale * *lv.offset(2 as libc::c_int as isize);
                pglColor4ub.expect("non-null function pointer")((*clr).r,
                                                                (*clr).g,
                                                                (*clr).b,
                                                                255 as
                                                                    libc::c_int
                                                                    as
                                                                    GLubyte);
                pglTexCoord2f.expect("non-null function pointer")(g_studio.chrome[idx
                                                                                      as
                                                                                      usize][0
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 usize]
                                                                      * s,
                                                                  g_studio.chrome[idx
                                                                                      as
                                                                                      usize][1
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 usize]
                                                                      * t);
                pglVertex3fv.expect("non-null function pointer")(vert.as_mut_ptr());
            } else {
                idx =
                    *ptricmds.offset(1 as libc::c_int as isize) as
                        libc::c_int;
                lv =
                    g_studio.lightvalues[*ptricmds.offset(1 as libc::c_int as
                                                              isize) as
                                             usize].as_mut_ptr() as
                        *mut libc::c_float;
                R_StudioSetColorBegin(ptricmds, pstudionorms);
                pglTexCoord2f.expect("non-null function pointer")(g_studio.chrome[idx
                                                                                      as
                                                                                      usize][0
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 usize]
                                                                      * s,
                                                                  g_studio.chrome[idx
                                                                                      as
                                                                                      usize][1
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 usize]
                                                                      * t);
                pglVertex3fv.expect("non-null function pointer")(g_studio.verts[*ptricmds.offset(0
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     isize)
                                                                                    as
                                                                                    usize].as_mut_ptr());
            }
            i -= 1;
            ptricmds = ptricmds.offset(4 as libc::c_int as isize)
        }
        pglEnd.expect("non-null function pointer")();
    };
}
#[inline]
unsafe extern "C" fn R_StudioBuildIndices(mut tri_strip: qboolean,
                                          mut vertexState: libc::c_int)
 -> libc::c_int {
    // build in indices
    let fresh9 = vertexState;
    vertexState = vertexState + 1;
    if fresh9 < 3 as libc::c_int {
        let fresh10 = g_studio.numelems;
        g_studio.numelems = g_studio.numelems.wrapping_add(1);
        g_studio.arrayelems[fresh10 as usize] =
            g_studio.numverts as libc::c_ushort
    } else if tri_strip as u64 != 0 {
        // flip triangles between clockwise and counter clockwise
        if vertexState & 1 as libc::c_int != 0 {
            // draw triangle [n-2 n-1 n]
            let fresh11 = g_studio.numelems;
            g_studio.numelems = g_studio.numelems.wrapping_add(1);
            g_studio.arrayelems[fresh11 as usize] =
                g_studio.numverts.wrapping_sub(2 as libc::c_int as
                                                   libc::c_uint) as
                    libc::c_ushort;
            let fresh12 = g_studio.numelems;
            g_studio.numelems = g_studio.numelems.wrapping_add(1);
            g_studio.arrayelems[fresh12 as usize] =
                g_studio.numverts.wrapping_sub(1 as libc::c_int as
                                                   libc::c_uint) as
                    libc::c_ushort;
            let fresh13 = g_studio.numelems;
            g_studio.numelems = g_studio.numelems.wrapping_add(1);
            g_studio.arrayelems[fresh13 as usize] =
                g_studio.numverts as libc::c_ushort
        } else {
            // draw triangle [n-1 n-2 n]
            let fresh14 = g_studio.numelems;
            g_studio.numelems = g_studio.numelems.wrapping_add(1);
            g_studio.arrayelems[fresh14 as usize] =
                g_studio.numverts.wrapping_sub(1 as libc::c_int as
                                                   libc::c_uint) as
                    libc::c_ushort;
            let fresh15 = g_studio.numelems;
            g_studio.numelems = g_studio.numelems.wrapping_add(1);
            g_studio.arrayelems[fresh15 as usize] =
                g_studio.numverts.wrapping_sub(2 as libc::c_int as
                                                   libc::c_uint) as
                    libc::c_ushort;
            let fresh16 = g_studio.numelems;
            g_studio.numelems = g_studio.numelems.wrapping_add(1);
            g_studio.arrayelems[fresh16 as usize] =
                g_studio.numverts as libc::c_ushort
        }
    } else {
        // draw triangle fan [0 n-1 n]
        let fresh17 = g_studio.numelems;
        g_studio.numelems = g_studio.numelems.wrapping_add(1);
        g_studio.arrayelems[fresh17 as usize] =
            g_studio.numverts.wrapping_sub((vertexState - 1 as libc::c_int) as
                                               libc::c_uint) as
                libc::c_ushort;
        let fresh18 = g_studio.numelems;
        g_studio.numelems = g_studio.numelems.wrapping_add(1);
        g_studio.arrayelems[fresh18 as usize] =
            g_studio.numverts.wrapping_sub(1 as libc::c_int as libc::c_uint)
                as libc::c_ushort;
        let fresh19 = g_studio.numelems;
        g_studio.numelems = g_studio.numelems.wrapping_add(1);
        g_studio.arrayelems[fresh19 as usize] =
            g_studio.numverts as libc::c_ushort
    }
    return vertexState;
}
/*
===============
R_StudioDrawNormalMesh

generic path
===============
*/
#[inline]
unsafe extern "C" fn R_StudioBuildArrayNormalMesh(mut ptricmds:
                                                      *mut libc::c_short,
                                                  mut pstudionorms:
                                                      *mut vec3_t,
                                                  mut s: libc::c_float,
                                                  mut t: libc::c_float) {
    let mut lv: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut i: libc::c_int = 0;
    let mut alpha: libc::c_float = tr.blend;
    loop  {
        let fresh20 = ptricmds;
        ptricmds = ptricmds.offset(1);
        i = *fresh20 as libc::c_int;
        if !(i != 0) { break ; }
        let mut vertexState: libc::c_int = 0 as libc::c_int;
        let mut tri_strip: qboolean = true_0;
        if i < 0 as libc::c_int { tri_strip = false_0; i = -i }
        while i > 0 as libc::c_int {
            let mut cl: *mut GLubyte = 0 as *mut GLubyte;
            cl = g_studio.arraycolor[g_studio.numverts as usize].as_mut_ptr();
            lv =
                g_studio.lightvalues[*ptricmds.offset(1 as libc::c_int as
                                                          isize) as
                                         usize].as_mut_ptr() as
                    *mut libc::c_float;
            vertexState = R_StudioBuildIndices(tri_strip, vertexState);
            R_StudioSetColorArray(ptricmds, pstudionorms, cl);
            g_studio.arraycoord[g_studio.numverts as
                                    usize][0 as libc::c_int as usize] =
                *ptricmds.offset(2 as libc::c_int as isize) as libc::c_int as
                    libc::c_float * s;
            g_studio.arraycoord[g_studio.numverts as
                                    usize][1 as libc::c_int as usize] =
                *ptricmds.offset(3 as libc::c_int as isize) as libc::c_int as
                    libc::c_float * t;
            g_studio.arrayverts[g_studio.numverts as
                                    usize][0 as libc::c_int as usize] =
                g_studio.verts[*ptricmds.offset(0 as libc::c_int as isize) as
                                   usize][0 as libc::c_int as usize];
            g_studio.arrayverts[g_studio.numverts as
                                    usize][1 as libc::c_int as usize] =
                g_studio.verts[*ptricmds.offset(0 as libc::c_int as isize) as
                                   usize][1 as libc::c_int as usize];
            g_studio.arrayverts[g_studio.numverts as
                                    usize][2 as libc::c_int as usize] =
                g_studio.verts[*ptricmds.offset(0 as libc::c_int as isize) as
                                   usize][2 as libc::c_int as usize];
            g_studio.numverts = g_studio.numverts.wrapping_add(1);
            i -= 1;
            ptricmds = ptricmds.offset(4 as libc::c_int as isize)
        }
    };
}
/*
===============
R_StudioDrawNormalMesh

generic path
===============
*/
#[inline]
unsafe extern "C" fn R_StudioBuildArrayFloatMesh(mut ptricmds:
                                                     *mut libc::c_short,
                                                 mut pstudionorms:
                                                     *mut vec3_t) {
    let mut lv: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut i: libc::c_int = 0;
    let mut alpha: libc::c_float = tr.blend;
    loop  {
        let fresh21 = ptricmds;
        ptricmds = ptricmds.offset(1);
        i = *fresh21 as libc::c_int;
        if !(i != 0) { break ; }
        let mut vertexState: libc::c_int = 0 as libc::c_int;
        let mut tri_strip: qboolean = true_0;
        if i < 0 as libc::c_int { tri_strip = false_0; i = -i }
        while i > 0 as libc::c_int {
            let mut cl: *mut GLubyte = 0 as *mut GLubyte;
            cl = g_studio.arraycolor[g_studio.numverts as usize].as_mut_ptr();
            lv =
                g_studio.lightvalues[*ptricmds.offset(1 as libc::c_int as
                                                          isize) as
                                         usize].as_mut_ptr() as
                    *mut libc::c_float;
            vertexState = R_StudioBuildIndices(tri_strip, vertexState);
            R_StudioSetColorArray(ptricmds, pstudionorms, cl);
            g_studio.arraycoord[g_studio.numverts as
                                    usize][0 as libc::c_int as usize] =
                HalfToFloat(*ptricmds.offset(2 as libc::c_int as isize) as
                                word);
            g_studio.arraycoord[g_studio.numverts as
                                    usize][1 as libc::c_int as usize] =
                HalfToFloat(*ptricmds.offset(3 as libc::c_int as isize) as
                                word);
            g_studio.arrayverts[g_studio.numverts as
                                    usize][0 as libc::c_int as usize] =
                g_studio.verts[*ptricmds.offset(0 as libc::c_int as isize) as
                                   usize][0 as libc::c_int as usize];
            g_studio.arrayverts[g_studio.numverts as
                                    usize][1 as libc::c_int as usize] =
                g_studio.verts[*ptricmds.offset(0 as libc::c_int as isize) as
                                   usize][1 as libc::c_int as usize];
            g_studio.arrayverts[g_studio.numverts as
                                    usize][2 as libc::c_int as usize] =
                g_studio.verts[*ptricmds.offset(0 as libc::c_int as isize) as
                                   usize][2 as libc::c_int as usize];
            g_studio.numverts = g_studio.numverts.wrapping_add(1);
            i -= 1;
            ptricmds = ptricmds.offset(4 as libc::c_int as isize)
        }
    };
}
/*
===============
R_StudioDrawNormalMesh

generic path
===============
*/
#[inline]
unsafe extern "C" fn R_StudioBuildArrayChromeMesh(mut ptricmds:
                                                      *mut libc::c_short,
                                                  mut pstudionorms:
                                                      *mut vec3_t,
                                                  mut s: libc::c_float,
                                                  mut t: libc::c_float,
                                                  mut scale: libc::c_float) {
    let mut lv: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut av: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut i: libc::c_int = 0;
    let mut idx: libc::c_int = 0;
    let mut glowShell: qboolean =
        if scale > 0.0f32 {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int } as qboolean;
    let mut vert: vec3_t = [0.; 3];
    let mut alpha: libc::c_float = tr.blend;
    loop  {
        let fresh22 = ptricmds;
        ptricmds = ptricmds.offset(1);
        i = *fresh22 as libc::c_int;
        if !(i != 0) { break ; }
        let mut vertexState: libc::c_int = 0 as libc::c_int;
        let mut tri_strip: qboolean = true_0;
        if i < 0 as libc::c_int { tri_strip = false_0; i = -i }
        while i > 0 as libc::c_int {
            let mut cl: *mut GLubyte = 0 as *mut GLubyte;
            cl = g_studio.arraycolor[g_studio.numverts as usize].as_mut_ptr();
            lv =
                g_studio.lightvalues[*ptricmds.offset(1 as libc::c_int as
                                                          isize) as
                                         usize].as_mut_ptr() as
                    *mut libc::c_float;
            vertexState = R_StudioBuildIndices(tri_strip, vertexState);
            if glowShell as u64 != 0 {
                idx =
                    g_studio.normaltable[*ptricmds.offset(0 as libc::c_int as
                                                              isize) as
                                             usize];
                av =
                    g_studio.verts[*ptricmds.offset(0 as libc::c_int as isize)
                                       as usize].as_mut_ptr();
                lv =
                    g_studio.norms[*ptricmds.offset(0 as libc::c_int as isize)
                                       as usize].as_mut_ptr();
                *cl.offset(0 as libc::c_int as isize) =
                    (*RI.currententity).curstate.rendercolor.r;
                *cl.offset(1 as libc::c_int as isize) =
                    (*RI.currententity).curstate.rendercolor.g;
                *cl.offset(2 as libc::c_int as isize) =
                    (*RI.currententity).curstate.rendercolor.b;
                *cl.offset(3 as libc::c_int as isize) =
                    255 as libc::c_int as GLubyte;
                vert[0 as libc::c_int as usize] =
                    *av.offset(0 as libc::c_int as isize) +
                        scale * *lv.offset(0 as libc::c_int as isize);
                vert[1 as libc::c_int as usize] =
                    *av.offset(1 as libc::c_int as isize) +
                        scale * *lv.offset(1 as libc::c_int as isize);
                vert[2 as libc::c_int as usize] =
                    *av.offset(2 as libc::c_int as isize) +
                        scale * *lv.offset(2 as libc::c_int as isize);
                g_studio.arrayverts[g_studio.numverts as
                                        usize][0 as libc::c_int as usize] =
                    vert[0 as libc::c_int as usize];
                g_studio.arrayverts[g_studio.numverts as
                                        usize][1 as libc::c_int as usize] =
                    vert[1 as libc::c_int as usize];
                g_studio.arrayverts[g_studio.numverts as
                                        usize][2 as libc::c_int as usize] =
                    vert[2 as libc::c_int as usize]
            } else {
                idx =
                    *ptricmds.offset(1 as libc::c_int as isize) as
                        libc::c_int;
                R_StudioSetColorArray(ptricmds, pstudionorms, cl);
                g_studio.arrayverts[g_studio.numverts as
                                        usize][0 as libc::c_int as usize] =
                    g_studio.verts[*ptricmds.offset(0 as libc::c_int as isize)
                                       as usize][0 as libc::c_int as usize];
                g_studio.arrayverts[g_studio.numverts as
                                        usize][1 as libc::c_int as usize] =
                    g_studio.verts[*ptricmds.offset(0 as libc::c_int as isize)
                                       as usize][1 as libc::c_int as usize];
                g_studio.arrayverts[g_studio.numverts as
                                        usize][2 as libc::c_int as usize] =
                    g_studio.verts[*ptricmds.offset(0 as libc::c_int as isize)
                                       as usize][2 as libc::c_int as usize]
            }
            g_studio.arraycoord[g_studio.numverts as
                                    usize][0 as libc::c_int as usize] =
                g_studio.chrome[idx as usize][0 as libc::c_int as usize] * s;
            g_studio.arraycoord[g_studio.numverts as
                                    usize][1 as libc::c_int as usize] =
                g_studio.chrome[idx as usize][1 as libc::c_int as usize] * t;
            g_studio.numverts = g_studio.numverts.wrapping_add(1);
            i -= 1;
            ptricmds = ptricmds.offset(4 as libc::c_int as isize)
        }
    };
}
#[inline]
unsafe extern "C" fn R_StudioDrawArrays(mut startverts: uint,
                                        mut startelems: uint) {
    pglEnableClientState.expect("non-null function pointer")(0x8074 as
                                                                 libc::c_int
                                                                 as GLenum);
    pglVertexPointer.expect("non-null function pointer")(3 as libc::c_int,
                                                         0x1406 as libc::c_int
                                                             as GLenum,
                                                         12 as libc::c_int,
                                                         g_studio.arrayverts.as_mut_ptr()
                                                             as
                                                             *const libc::c_void);
    pglEnableClientState.expect("non-null function pointer")(0x8078 as
                                                                 libc::c_int
                                                                 as GLenum);
    pglTexCoordPointer.expect("non-null function pointer")(2 as libc::c_int,
                                                           0x1406 as
                                                               libc::c_int as
                                                               GLenum,
                                                           0 as libc::c_int,
                                                           g_studio.arraycoord.as_mut_ptr()
                                                               as
                                                               *const libc::c_void);
    if g_nForceFaceFlags & 0x2 as libc::c_int == 0 {
        pglEnableClientState.expect("non-null function pointer")(0x8076 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum);
        pglColorPointer.expect("non-null function pointer")(4 as libc::c_int,
                                                            0x1401 as
                                                                libc::c_int as
                                                                GLenum,
                                                            0 as libc::c_int,
                                                            g_studio.arraycolor.as_mut_ptr()
                                                                as
                                                                *const libc::c_void);
    }
    // WebGL need to know array sizes
    if pglDrawRangeElements.is_some() {
        pglDrawRangeElements.expect("non-null function pointer")(0x4 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 startverts,
                                                                 g_studio.numverts,
                                                                 g_studio.numelems.wrapping_sub(startelems)
                                                                     as
                                                                     GLsizei,
                                                                 0x1403 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 &mut *g_studio.arrayelems.as_mut_ptr().offset(startelems
                                                                                                                   as
                                                                                                                   isize)
                                                                     as
                                                                     *mut libc::c_ushort
                                                                     as
                                                                     *const libc::c_void);
    } else {
        pglDrawElements.expect("non-null function pointer")(0x4 as libc::c_int
                                                                as GLenum,
                                                            g_studio.numelems.wrapping_sub(startelems)
                                                                as GLsizei,
                                                            0x1403 as
                                                                libc::c_int as
                                                                GLenum,
                                                            &mut *g_studio.arrayelems.as_mut_ptr().offset(startelems
                                                                                                              as
                                                                                                              isize)
                                                                as
                                                                *mut libc::c_ushort
                                                                as
                                                                *const libc::c_void);
    }
    pglDisableClientState.expect("non-null function pointer")(0x8074 as
                                                                  libc::c_int
                                                                  as GLenum);
    pglDisableClientState.expect("non-null function pointer")(0x8078 as
                                                                  libc::c_int
                                                                  as GLenum);
    if g_nForceFaceFlags & 0x2 as libc::c_int == 0 {
        pglDisableClientState.expect("non-null function pointer")(0x8076 as
                                                                      libc::c_int
                                                                      as
                                                                      GLenum);
    };
}
/*
===============
R_StudioDrawPoints

===============
*/
unsafe extern "C" fn R_StudioDrawPoints() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut m_skinnum: libc::c_int = 0;
    let mut shellscale: libc::c_float = 0.0f32;
    let mut need_sort: qboolean = false_0;
    let mut pvertbone: *mut byte = 0 as *mut byte;
    let mut pnormbone: *mut byte = 0 as *mut byte;
    let mut pstudioverts: *mut vec3_t = 0 as *mut vec3_t;
    let mut pstudionorms: *mut vec3_t = 0 as *mut vec3_t;
    let mut ptexture: *mut mstudiotexture_t = 0 as *mut mstudiotexture_t;
    let mut pmesh: *mut mstudiomesh_t = 0 as *mut mstudiomesh_t;
    let mut pskinref: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut lv_tmp: libc::c_float = 0.;
    if m_pStudioHeader.is_null() { return }
    g_studio.numelems = 0 as libc::c_int as uint;
    g_studio.numverts = g_studio.numelems;
    // safety bounding the skinnum
    m_skinnum =
        if (*RI.currententity).curstate.skin as libc::c_int >=
               0 as libc::c_int {
            if ((*RI.currententity).curstate.skin as libc::c_int) <
                   (*m_pStudioHeader).numskinfamilies - 1 as libc::c_int {
                (*RI.currententity).curstate.skin as libc::c_int
            } else { ((*m_pStudioHeader).numskinfamilies) - 1 as libc::c_int }
        } else { 0 as libc::c_int };
    ptexture =
        (m_pStudioHeader as
             *mut byte).offset((*m_pStudioHeader).textureindex as isize) as
            *mut mstudiotexture_t;
    pvertbone =
        (m_pStudioHeader as
             *mut byte).offset((*m_pSubModel).vertinfoindex as isize);
    pnormbone =
        (m_pStudioHeader as
             *mut byte).offset((*m_pSubModel).norminfoindex as isize);
    pmesh =
        (m_pStudioHeader as
             *mut byte).offset((*m_pSubModel).meshindex as isize) as
            *mut mstudiomesh_t;
    pstudioverts =
        (m_pStudioHeader as
             *mut byte).offset((*m_pSubModel).vertindex as isize) as
            *mut vec3_t;
    pstudionorms =
        (m_pStudioHeader as
             *mut byte).offset((*m_pSubModel).normindex as isize) as
            *mut vec3_t;
    pskinref =
        (m_pStudioHeader as
             *mut byte).offset((*m_pStudioHeader).skinindex as isize) as
            *mut libc::c_short;
    if m_skinnum != 0 as libc::c_int {
        pskinref =
            pskinref.offset((m_skinnum * (*m_pStudioHeader).numskinref) as
                                isize)
    }
    if (*m_pStudioHeader).flags as libc::c_uint &
           (1 as libc::c_uint) << 31 as libc::c_int != 0 &&
           (*m_pSubModel).blendvertinfoindex != 0 as libc::c_int &&
           (*m_pSubModel).blendnorminfoindex != 0 as libc::c_int {
        let mut pvertweight: *mut mstudioboneweight_t =
            (m_pStudioHeader as
                 *mut byte).offset((*m_pSubModel).blendvertinfoindex as isize)
                as *mut mstudioboneweight_t;
        let mut pnormweight: *mut mstudioboneweight_t =
            (m_pStudioHeader as
                 *mut byte).offset((*m_pSubModel).blendnorminfoindex as isize)
                as *mut mstudioboneweight_t;
        let mut skinMat: matrix3x4 = [[0.; 4]; 3];
        i = 0 as libc::c_int;
        while i < (*m_pSubModel).numverts {
            R_StudioComputeSkinMatrix(&mut *pvertweight.offset(i as isize),
                                      skinMat.as_mut_ptr());
            Matrix3x4_VectorTransform(skinMat.as_mut_ptr() as
                                          *const [vec_t; 4],
                                      (*pstudioverts.offset(i as
                                                                isize)).as_mut_ptr()
                                          as *const libc::c_float,
                                      g_studio.verts[i as
                                                         usize].as_mut_ptr());
            R_LightStrength(*pvertbone.offset(i as isize) as libc::c_int,
                            (*pstudioverts.offset(i as isize)).as_mut_ptr(),
                            g_studio.lightpos[i as usize].as_mut_ptr());
            i += 1
        }
        i = 0 as libc::c_int;
        while i < (*m_pSubModel).numnorms {
            R_StudioComputeSkinMatrix(&mut *pnormweight.offset(i as isize),
                                      skinMat.as_mut_ptr());
            Matrix3x4_VectorRotate(skinMat.as_mut_ptr() as *const [vec_t; 4],
                                   (*pstudionorms.offset(i as
                                                             isize)).as_mut_ptr()
                                       as *const libc::c_float,
                                   g_studio.norms[i as usize].as_mut_ptr());
            i += 1
        }
    } else {
        i = 0 as libc::c_int;
        while i < (*m_pSubModel).numverts {
            Matrix3x4_VectorTransform(g_studio.bonestransform[*pvertbone.offset(i
                                                                                    as
                                                                                    isize)
                                                                  as
                                                                  usize].as_mut_ptr()
                                          as *const [vec_t; 4],
                                      (*pstudioverts.offset(i as
                                                                isize)).as_mut_ptr()
                                          as *const libc::c_float,
                                      g_studio.verts[i as
                                                         usize].as_mut_ptr());
            R_LightStrength(*pvertbone.offset(i as isize) as libc::c_int,
                            (*pstudioverts.offset(i as isize)).as_mut_ptr(),
                            g_studio.lightpos[i as usize].as_mut_ptr());
            i += 1
        }
    }
    // generate shared normals for properly scaling glowing shell
    if (*RI.currententity).curstate.renderfx ==
           kRenderFxGlowShell as libc::c_int {
        let mut factor: libc::c_float = 1.0f32 / 128.0f32;
        shellscale =
            if factor >
                   (*RI.currententity).curstate.renderamt as libc::c_float *
                       factor {
                factor
            } else {
                ((*RI.currententity).curstate.renderamt as libc::c_float) *
                    factor
            };
        R_StudioBuildNormalTable();
        R_StudioGenerateNormals();
    }
    k = 0 as libc::c_int;
    j = k;
    while j < (*m_pSubModel).nummesh {
        g_nFaceFlags =
            ((*ptexture.offset(*pskinref.offset((*pmesh.offset(j as
                                                                   isize)).skinref
                                                    as isize) as isize)).flags
                 | g_nForceFaceFlags as libc::c_uint) as libc::c_int;
        // fill in sortedmesh info
        g_studio.meshes[j as usize].flags = g_nFaceFlags;
        g_studio.meshes[j as usize].mesh =
            &mut *pmesh.offset(j as isize) as *mut mstudiomesh_t;
        if g_nFaceFlags & (0x40 as libc::c_int | 0x20 as libc::c_int) != 0 {
            need_sort = true_0
        }
        if (*RI.currententity).curstate.rendermode ==
               kRenderTransAdd as libc::c_int {
            i = 0 as libc::c_int;
            while i < (*pmesh.offset(j as isize)).numnorms {
                if g_nFaceFlags & 0x2 as libc::c_int != 0 {
                    R_StudioSetupChrome(g_studio.chrome[k as
                                                            usize].as_mut_ptr(),
                                        *pnormbone as libc::c_int,
                                        pstudionorms as *mut libc::c_float);
                }
                g_studio.lightvalues[k as usize][0 as libc::c_int as usize] =
                    tr.blend;
                g_studio.lightvalues[k as usize][1 as libc::c_int as usize] =
                    tr.blend;
                g_studio.lightvalues[k as usize][2 as libc::c_int as usize] =
                    tr.blend;
                i += 1;
                k += 1;
                pstudionorms = pstudionorms.offset(1);
                pnormbone = pnormbone.offset(1)
            }
        } else {
            i = 0 as libc::c_int;
            while i < (*pmesh.offset(j as isize)).numnorms {
                if (*m_pStudioHeader).flags as libc::c_uint &
                       (1 as libc::c_uint) << 31 as libc::c_int != 0 {
                    R_StudioLighting(&mut lv_tmp, -(1 as libc::c_int),
                                     g_nFaceFlags,
                                     g_studio.norms[k as usize].as_mut_ptr());
                } else {
                    R_StudioLighting(&mut lv_tmp, *pnormbone as libc::c_int,
                                     g_nFaceFlags,
                                     pstudionorms as *mut libc::c_float);
                }
                if g_nFaceFlags & 0x2 as libc::c_int != 0 {
                    R_StudioSetupChrome(g_studio.chrome[k as
                                                            usize].as_mut_ptr(),
                                        *pnormbone as libc::c_int,
                                        pstudionorms as *mut libc::c_float);
                }
                g_studio.lightvalues[k as usize][0 as libc::c_int as usize] =
                    g_studio.lightcolor[0 as libc::c_int as usize] * lv_tmp;
                g_studio.lightvalues[k as usize][1 as libc::c_int as usize] =
                    g_studio.lightcolor[1 as libc::c_int as usize] * lv_tmp;
                g_studio.lightvalues[k as usize][2 as libc::c_int as usize] =
                    g_studio.lightcolor[2 as libc::c_int as usize] * lv_tmp;
                i += 1;
                k += 1;
                pstudionorms = pstudionorms.offset(1);
                pnormbone = pnormbone.offset(1)
            }
        }
        j += 1
    }
    if (*r_studio_sort_textures).value != 0. && need_sort as libc::c_uint != 0
       {
        // resort opaque and translucent meshes draw order
        qsort(g_studio.meshes.as_mut_ptr() as *mut libc::c_void,
              (*m_pSubModel).nummesh as size_t,
              ::std::mem::size_of::<sortedmesh_t>() as libc::c_ulong,
              Some(R_StudioMeshCompare as
                       unsafe extern "C" fn(_: *const libc::c_void,
                                            _: *const libc::c_void)
                           -> libc::c_int));
    }
    // NOTE: rewind normals at start
    pstudionorms =
        (m_pStudioHeader as
             *mut byte).offset((*m_pSubModel).normindex as isize) as
            *mut vec3_t;
    j = 0 as libc::c_int;
    while j < (*m_pSubModel).nummesh {
        let mut oldblend: libc::c_float = tr.blend;
        let mut startArrayVerts: uint = g_studio.numverts;
        let mut startArrayElems: uint = g_studio.numelems;
        let mut ptricmds: *mut libc::c_short = 0 as *mut libc::c_short;
        let mut s: libc::c_float = 0.;
        let mut t: libc::c_float = 0.;
        pmesh = g_studio.meshes[j as usize].mesh;
        ptricmds =
            (m_pStudioHeader as *mut byte).offset((*pmesh).triindex as isize)
                as *mut libc::c_short;
        g_nFaceFlags =
            ((*ptexture.offset(*pskinref.offset((*pmesh).skinref as isize) as
                                   isize)).flags |
                 g_nForceFaceFlags as libc::c_uint) as libc::c_int;
        s =
            1.0f32 /
                (*ptexture.offset(*pskinref.offset((*pmesh).skinref as isize)
                                      as isize)).width as libc::c_float;
        t =
            1.0f32 /
                (*ptexture.offset(*pskinref.offset((*pmesh).skinref as isize)
                                      as isize)).height as libc::c_float;
        if g_nFaceFlags & 0x40 as libc::c_int != 0 {
            pglEnable.expect("non-null function pointer")(0xbc0 as libc::c_int
                                                              as GLenum);
            pglAlphaFunc.expect("non-null function pointer")(0x204 as
                                                                 libc::c_int
                                                                 as GLenum,
                                                             0.5f32);
            pglDepthMask.expect("non-null function pointer")(0x1 as
                                                                 libc::c_int
                                                                 as
                                                                 GLboolean);
            if (*RI.currententity).curstate.rendermode ==
                   kRenderNormal as libc::c_int {
                tr.blend = 1.0f32
            }
        } else if g_nFaceFlags & 0x20 as libc::c_int != 0 {
            if (*RI.currententity).curstate.rendermode ==
                   kRenderNormal as libc::c_int {
                pglBlendFunc.expect("non-null function pointer")(0x1 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 0x1 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum);
                pglDepthMask.expect("non-null function pointer")(0 as
                                                                     libc::c_int
                                                                     as
                                                                     GLboolean);
                pglEnable.expect("non-null function pointer")(0xbe2 as
                                                                  libc::c_int
                                                                  as GLenum);
                R_AllowFog(false_0);
            } else {
                pglBlendFunc.expect("non-null function pointer")(0x302 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum,
                                                                 0x1 as
                                                                     libc::c_int
                                                                     as
                                                                     GLenum);
            }
        }
        R_StudioSetupSkin(m_pStudioHeader,
                          *pskinref.offset((*pmesh).skinref as isize) as
                              libc::c_int);
        if if !r_studio_drawelements.is_null() &&
                  (*r_studio_drawelements).value != 0.0f32 {
               true_0 as libc::c_int
           } else { false_0 as libc::c_int } != 0 {
            if g_nFaceFlags & 0x2 as libc::c_int != 0 {
                R_StudioBuildArrayChromeMesh(ptricmds, pstudionorms, s, t,
                                             shellscale);
            } else if g_nFaceFlags as libc::c_uint &
                          (1 as libc::c_uint) << 31 as libc::c_int != 0 {
                R_StudioBuildArrayFloatMesh(ptricmds, pstudionorms);
            } else {
                R_StudioBuildArrayNormalMesh(ptricmds, pstudionorms, s, t);
            }
            R_StudioDrawArrays(startArrayVerts, startArrayElems);
        } else if g_nFaceFlags & 0x2 as libc::c_int != 0 {
            R_StudioDrawChromeMesh(ptricmds, pstudionorms, s, t, shellscale);
        } else if g_nFaceFlags as libc::c_uint &
                      (1 as libc::c_uint) << 31 as libc::c_int != 0 {
            R_StudioDrawFloatMesh(ptricmds, pstudionorms);
        } else { R_StudioDrawNormalMesh(ptricmds, pstudionorms, s, t); }
        if g_nFaceFlags & 0x40 as libc::c_int != 0 {
            pglAlphaFunc.expect("non-null function pointer")(0x204 as
                                                                 libc::c_int
                                                                 as GLenum,
                                                             0.0f32);
            pglDisable.expect("non-null function pointer")(0xbc0 as
                                                               libc::c_int as
                                                               GLenum);
        } else if g_nFaceFlags & 0x20 as libc::c_int != 0 &&
                      (*RI.currententity).curstate.rendermode ==
                          kRenderNormal as libc::c_int {
            pglDepthMask.expect("non-null function pointer")(0x1 as
                                                                 libc::c_int
                                                                 as
                                                                 GLboolean);
            pglDisable.expect("non-null function pointer")(0xbe2 as
                                                               libc::c_int as
                                                               GLenum);
            R_AllowFog(true_0);
        }
        r_stats.c_studio_polys =
            (r_stats.c_studio_polys as
                 libc::c_uint).wrapping_add((*pmesh).numtris as libc::c_uint)
                as uint as uint;
        tr.blend = oldblend;
        j += 1
    };
}
/*
===============
R_StudioDrawHulls

===============
*/
unsafe extern "C" fn R_StudioDrawHulls() {
    let mut alpha: libc::c_float = 0.;
    let mut lv: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if (*r_drawentities).value == 4 as libc::c_int as libc::c_float {
        alpha = 0.5f32
    } else { alpha = 1.0f32 }
    GL_Bind(XASH_TEXTURE0 as libc::c_int, tr.whiteTexture as GLenum);
    pglTexEnvf.expect("non-null function pointer")(0x2300 as libc::c_int as
                                                       GLenum,
                                                   0x2200 as libc::c_int as
                                                       GLenum,
                                                   0x2100 as libc::c_int as
                                                       GLfloat);
    i = 0 as libc::c_int;
    while i < (*m_pStudioHeader).numhitboxes {
        let mut pbbox: *mut mstudiobbox_t =
            (m_pStudioHeader as
                 *mut byte).offset((*m_pStudioHeader).hitboxindex as isize) as
                *mut mstudiobbox_t;
        let mut tmp: vec3_t = [0.; 3];
        let mut p: [vec3_t; 8] = [[0.; 3]; 8];
        j = 0 as libc::c_int;
        while j < 8 as libc::c_int {
            tmp[0 as libc::c_int as usize] =
                if j & 1 as libc::c_int != 0 {
                    (*pbbox.offset(i as
                                       isize)).bbmin[0 as libc::c_int as
                                                         usize]
                } else {
                    (*pbbox.offset(i as
                                       isize)).bbmax[0 as libc::c_int as
                                                         usize]
                };
            tmp[1 as libc::c_int as usize] =
                if j & 2 as libc::c_int != 0 {
                    (*pbbox.offset(i as
                                       isize)).bbmin[1 as libc::c_int as
                                                         usize]
                } else {
                    (*pbbox.offset(i as
                                       isize)).bbmax[1 as libc::c_int as
                                                         usize]
                };
            tmp[2 as libc::c_int as usize] =
                if j & 4 as libc::c_int != 0 {
                    (*pbbox.offset(i as
                                       isize)).bbmin[2 as libc::c_int as
                                                         usize]
                } else {
                    (*pbbox.offset(i as
                                       isize)).bbmax[2 as libc::c_int as
                                                         usize]
                };
            Matrix3x4_VectorTransform(g_studio.bonestransform[(*pbbox.offset(i
                                                                                 as
                                                                                 isize)).bone
                                                                  as
                                                                  usize].as_mut_ptr()
                                          as *const [vec_t; 4],
                                      tmp.as_mut_ptr() as
                                          *const libc::c_float,
                                      p[j as usize].as_mut_ptr());
            j += 1
        }
        j = (*pbbox.offset(i as isize)).group % 8 as libc::c_int;
        TriBegin(2 as libc::c_int);
        TriColor4f(hullcolor[j as usize][0 as libc::c_int as usize],
                   hullcolor[j as usize][1 as libc::c_int as usize],
                   hullcolor[j as usize][2 as libc::c_int as usize], alpha);
        j = 0 as libc::c_int;
        while j < 6 as libc::c_int {
            tmp[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            tmp[1 as libc::c_int as usize] = tmp[2 as libc::c_int as usize];
            tmp[0 as libc::c_int as usize] = tmp[1 as libc::c_int as usize];
            tmp[(j % 3 as libc::c_int) as usize] =
                if j < 3 as libc::c_int { 1.0f32 } else { -1.0f32 };
            R_StudioLighting(&mut lv, (*pbbox.offset(i as isize)).bone,
                             0 as libc::c_int, tmp.as_mut_ptr());
            TriBrightness(lv);
            TriVertex3fv(p[boxpnt[j as usize][0 as libc::c_int as usize] as
                               usize].as_mut_ptr());
            TriVertex3fv(p[boxpnt[j as usize][1 as libc::c_int as usize] as
                               usize].as_mut_ptr());
            TriVertex3fv(p[boxpnt[j as usize][2 as libc::c_int as usize] as
                               usize].as_mut_ptr());
            TriVertex3fv(p[boxpnt[j as usize][3 as libc::c_int as usize] as
                               usize].as_mut_ptr());
            j += 1
        }
        TriEnd();
        i += 1
    };
}
/*
===============
R_StudioDrawAbsBBox

===============
*/
unsafe extern "C" fn R_StudioDrawAbsBBox() {
    let mut p: [vec3_t; 8] = [[0.; 3]; 8];
    let mut tmp: vec3_t = [0.; 3];
    let mut lv: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    // looks ugly, skip
    if RI.currententity ==
           gEngfuncs.GetViewModel.expect("non-null function pointer")() {
        return
    }
    if R_StudioComputeBBox(p.as_mut_ptr()) as u64 == 0 { return }
    GL_Bind(XASH_TEXTURE0 as libc::c_int, tr.whiteTexture as GLenum);
    TriColor4f(0.5f32, 0.5f32, 1.0f32, 0.5f32);
    TriRenderMode(kRenderTransAdd as libc::c_int);
    TriBegin(2 as libc::c_int);
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        tmp[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
        tmp[1 as libc::c_int as usize] = tmp[2 as libc::c_int as usize];
        tmp[0 as libc::c_int as usize] = tmp[1 as libc::c_int as usize];
        tmp[(i % 3 as libc::c_int) as usize] =
            if i < 3 as libc::c_int { 1.0f32 } else { -1.0f32 };
        R_StudioLighting(&mut lv, -(1 as libc::c_int), 0 as libc::c_int,
                         tmp.as_mut_ptr());
        TriBrightness(lv);
        TriVertex3fv(p[boxpnt[i as usize][0 as libc::c_int as usize] as
                           usize].as_mut_ptr());
        TriVertex3fv(p[boxpnt[i as usize][1 as libc::c_int as usize] as
                           usize].as_mut_ptr());
        TriVertex3fv(p[boxpnt[i as usize][2 as libc::c_int as usize] as
                           usize].as_mut_ptr());
        TriVertex3fv(p[boxpnt[i as usize][3 as libc::c_int as usize] as
                           usize].as_mut_ptr());
        i += 1
    }
    TriEnd();
    TriRenderMode(kRenderNormal as libc::c_int);
}
/*
===============
R_StudioDrawBones

===============
*/
unsafe extern "C" fn R_StudioDrawBones() {
    let mut pbones: *mut mstudiobone_t =
        (m_pStudioHeader as
             *mut byte).offset((*m_pStudioHeader).boneindex as isize) as
            *mut mstudiobone_t;
    let mut point: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    pglDisable.expect("non-null function pointer")(0xde1 as libc::c_int as
                                                       GLenum);
    i = 0 as libc::c_int;
    while i < (*m_pStudioHeader).numbones {
        if (*pbones.offset(i as isize)).parent >= 0 as libc::c_int {
            pglPointSize.expect("non-null function pointer")(3.0f32);
            pglColor3f.expect("non-null function pointer")(1 as libc::c_int as
                                                               GLfloat,
                                                           0.7f32,
                                                           0 as libc::c_int as
                                                               GLfloat);
            pglBegin.expect("non-null function pointer")(0x1 as libc::c_int as
                                                             GLenum);
            Matrix3x4_OriginFromMatrix(g_studio.bonestransform[(*pbones.offset(i
                                                                                   as
                                                                                   isize)).parent
                                                                   as
                                                                   usize].as_mut_ptr()
                                           as *const [vec_t; 4],
                                       point.as_mut_ptr());
            pglVertex3fv.expect("non-null function pointer")(point.as_mut_ptr());
            Matrix3x4_OriginFromMatrix(g_studio.bonestransform[i as
                                                                   usize].as_mut_ptr()
                                           as *const [vec_t; 4],
                                       point.as_mut_ptr());
            pglVertex3fv.expect("non-null function pointer")(point.as_mut_ptr());
            pglEnd.expect("non-null function pointer")();
            pglColor3f.expect("non-null function pointer")(0 as libc::c_int as
                                                               GLfloat,
                                                           0 as libc::c_int as
                                                               GLfloat,
                                                           0.8f32);
            pglBegin.expect("non-null function pointer")(0 as libc::c_int as
                                                             GLenum);
            if (*pbones.offset((*pbones.offset(i as isize)).parent as
                                   isize)).parent != -(1 as libc::c_int) {
                Matrix3x4_OriginFromMatrix(g_studio.bonestransform[(*pbones.offset(i
                                                                                       as
                                                                                       isize)).parent
                                                                       as
                                                                       usize].as_mut_ptr()
                                               as *const [vec_t; 4],
                                           point.as_mut_ptr());
                pglVertex3fv.expect("non-null function pointer")(point.as_mut_ptr());
            }
            Matrix3x4_OriginFromMatrix(g_studio.bonestransform[i as
                                                                   usize].as_mut_ptr()
                                           as *const [vec_t; 4],
                                       point.as_mut_ptr());
            pglVertex3fv.expect("non-null function pointer")(point.as_mut_ptr());
            pglEnd.expect("non-null function pointer")();
        } else {
            // draw parent bone node
            pglPointSize.expect("non-null function pointer")(5.0f32);
            pglColor3f.expect("non-null function pointer")(0.8f32,
                                                           0 as libc::c_int as
                                                               GLfloat,
                                                           0 as libc::c_int as
                                                               GLfloat);
            pglBegin.expect("non-null function pointer")(0 as libc::c_int as
                                                             GLenum);
            Matrix3x4_OriginFromMatrix(g_studio.bonestransform[i as
                                                                   usize].as_mut_ptr()
                                           as *const [vec_t; 4],
                                       point.as_mut_ptr());
            pglVertex3fv.expect("non-null function pointer")(point.as_mut_ptr());
            pglEnd.expect("non-null function pointer")();
        }
        i += 1
    }
    pglPointSize.expect("non-null function pointer")(1.0f32);
    pglEnable.expect("non-null function pointer")(0xde1 as libc::c_int as
                                                      GLenum);
}
unsafe extern "C" fn R_StudioDrawAttachments() {
    let mut i: libc::c_int = 0;
    pglDisable.expect("non-null function pointer")(0xde1 as libc::c_int as
                                                       GLenum);
    pglDisable.expect("non-null function pointer")(0xb71 as libc::c_int as
                                                       GLenum);
    i = 0 as libc::c_int;
    while i < (*m_pStudioHeader).numattachments {
        let mut pattachments: *mut mstudioattachment_t =
            0 as *mut mstudioattachment_t;
        let mut v: [vec3_t; 4] = [[0.; 3]; 4];
        pattachments =
            (m_pStudioHeader as
                 *mut byte).offset((*m_pStudioHeader).attachmentindex as
                                       isize) as *mut mstudioattachment_t;
        Matrix3x4_VectorTransform(g_studio.bonestransform[(*pattachments.offset(i
                                                                                    as
                                                                                    isize)).bone
                                                              as
                                                              usize].as_mut_ptr()
                                      as *const [vec_t; 4],
                                  (*pattachments.offset(i as
                                                            isize)).org.as_mut_ptr()
                                      as *const libc::c_float,
                                  v[0 as libc::c_int as usize].as_mut_ptr());
        Matrix3x4_VectorTransform(g_studio.bonestransform[(*pattachments.offset(i
                                                                                    as
                                                                                    isize)).bone
                                                              as
                                                              usize].as_mut_ptr()
                                      as *const [vec_t; 4],
                                  (*pattachments.offset(i as
                                                            isize)).vectors[0
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize].as_mut_ptr()
                                      as *const libc::c_float,
                                  v[1 as libc::c_int as usize].as_mut_ptr());
        Matrix3x4_VectorTransform(g_studio.bonestransform[(*pattachments.offset(i
                                                                                    as
                                                                                    isize)).bone
                                                              as
                                                              usize].as_mut_ptr()
                                      as *const [vec_t; 4],
                                  (*pattachments.offset(i as
                                                            isize)).vectors[1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize].as_mut_ptr()
                                      as *const libc::c_float,
                                  v[2 as libc::c_int as usize].as_mut_ptr());
        Matrix3x4_VectorTransform(g_studio.bonestransform[(*pattachments.offset(i
                                                                                    as
                                                                                    isize)).bone
                                                              as
                                                              usize].as_mut_ptr()
                                      as *const [vec_t; 4],
                                  (*pattachments.offset(i as
                                                            isize)).vectors[2
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize].as_mut_ptr()
                                      as *const libc::c_float,
                                  v[3 as libc::c_int as usize].as_mut_ptr());
        pglBegin.expect("non-null function pointer")(0x1 as libc::c_int as
                                                         GLenum);
        pglColor3f.expect("non-null function pointer")(1 as libc::c_int as
                                                           GLfloat,
                                                       0 as libc::c_int as
                                                           GLfloat,
                                                       0 as libc::c_int as
                                                           GLfloat);
        pglVertex3fv.expect("non-null function pointer")(v[0 as libc::c_int as
                                                               usize].as_mut_ptr());
        pglColor3f.expect("non-null function pointer")(1 as libc::c_int as
                                                           GLfloat,
                                                       1 as libc::c_int as
                                                           GLfloat,
                                                       1 as libc::c_int as
                                                           GLfloat);
        pglVertex3fv.expect("non-null function pointer")(v[1 as libc::c_int as
                                                               usize].as_mut_ptr());
        pglColor3f.expect("non-null function pointer")(1 as libc::c_int as
                                                           GLfloat,
                                                       0 as libc::c_int as
                                                           GLfloat,
                                                       0 as libc::c_int as
                                                           GLfloat);
        pglVertex3fv.expect("non-null function pointer")(v[0 as libc::c_int as
                                                               usize].as_mut_ptr());
        pglColor3f.expect("non-null function pointer")(1 as libc::c_int as
                                                           GLfloat,
                                                       1 as libc::c_int as
                                                           GLfloat,
                                                       1 as libc::c_int as
                                                           GLfloat);
        pglVertex3fv.expect("non-null function pointer")(v[2 as libc::c_int as
                                                               usize].as_mut_ptr());
        pglColor3f.expect("non-null function pointer")(1 as libc::c_int as
                                                           GLfloat,
                                                       0 as libc::c_int as
                                                           GLfloat,
                                                       0 as libc::c_int as
                                                           GLfloat);
        pglVertex3fv.expect("non-null function pointer")(v[0 as libc::c_int as
                                                               usize].as_mut_ptr());
        pglColor3f.expect("non-null function pointer")(1 as libc::c_int as
                                                           GLfloat,
                                                       1 as libc::c_int as
                                                           GLfloat,
                                                       1 as libc::c_int as
                                                           GLfloat);
        pglVertex3fv.expect("non-null function pointer")(v[3 as libc::c_int as
                                                               usize].as_mut_ptr());
        pglEnd.expect("non-null function pointer")();
        pglPointSize.expect("non-null function pointer")(5.0f32);
        pglColor3f.expect("non-null function pointer")(0 as libc::c_int as
                                                           GLfloat,
                                                       1 as libc::c_int as
                                                           GLfloat,
                                                       0 as libc::c_int as
                                                           GLfloat);
        pglBegin.expect("non-null function pointer")(0 as libc::c_int as
                                                         GLenum);
        pglVertex3fv.expect("non-null function pointer")(v[0 as libc::c_int as
                                                               usize].as_mut_ptr());
        pglEnd.expect("non-null function pointer")();
        pglPointSize.expect("non-null function pointer")(1.0f32);
        i += 1
    }
    pglEnable.expect("non-null function pointer")(0xde1 as libc::c_int as
                                                      GLenum);
    pglEnable.expect("non-null function pointer")(0xb71 as libc::c_int as
                                                      GLenum);
}
/*
===============
R_StudioSetRemapColors

===============
*/
unsafe extern "C" fn R_StudioSetRemapColors(mut newTop: libc::c_int,
                                            mut newBottom: libc::c_int) {
    gEngfuncs.CL_AllocRemapInfo.expect("non-null function pointer")(RI.currententity,
                                                                    RI.currentmodel,
                                                                    newTop,
                                                                    newBottom);
    if !gEngfuncs.CL_GetRemapInfoForEntity.expect("non-null function pointer")(RI.currententity).is_null()
       {
        gEngfuncs.CL_UpdateRemapInfo.expect("non-null function pointer")(RI.currententity,
                                                                         newTop,
                                                                         newBottom);
        m_fDoRemap = true_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_StudioResetPlayerModels() {
    memset(g_studio.player_models.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<[player_model_t; 32]>() as libc::c_ulong);
}
/*
===============
R_StudioSetupPlayerModel

===============
*/
unsafe extern "C" fn R_StudioSetupPlayerModel(mut index: libc::c_int)
 -> *mut model_t {
    let mut info: *mut player_info_t =
        gEngfuncs.pfnPlayerInfo.expect("non-null function pointer")(index);
    let mut state: *mut player_model_t = 0 as *mut player_model_t;
    state =
        &mut *g_studio.player_models.as_mut_ptr().offset(index as isize) as
            *mut player_model_t;
    // g-cont: force for "dev-mode", non-local games and menu preview
    if ((*gpGlobals).developer as libc::c_uint != 0 ||
            Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(PARM_LOCAL_GAME
                                                                                                                      as
                                                                                                                      libc::c_int,
                                                                                                                  0
                                                                                                                      as
                                                                                                                      libc::c_int)
                == 0 || RI.drawWorld as u64 == 0) &&
           (*info).model[0 as libc::c_int as usize] as libc::c_int != 0 {
        if Q_strncmp((*state).name.as_mut_ptr(), (*info).model.as_mut_ptr(),
                     99999 as libc::c_int) != 0 {
            Q_strncpy((*state).name.as_mut_ptr(), (*info).model.as_mut_ptr(),
                      ::std::mem::size_of::<[libc::c_char; 260]>() as
                          libc::c_ulong);
            (*state).name[(::std::mem::size_of::<[libc::c_char; 260]>() as
                               libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                               libc::c_ulong)
                              as usize] = 0 as libc::c_int as libc::c_char;
            Q_snprintf((*state).modelname.as_mut_ptr(),
                       ::std::mem::size_of::<[libc::c_char; 260]>() as
                           libc::c_ulong,
                       b"models/player/%s/%s.mdl\x00" as *const u8 as
                           *const libc::c_char, (*info).model.as_mut_ptr(),
                       (*info).model.as_mut_ptr());
            if gEngfuncs.FS_FileExists.expect("non-null function pointer")((*state).modelname.as_mut_ptr(),
                                                                           false_0
                                                                               as
                                                                               libc::c_int)
                   != 0 {
                (*state).model =
                    gEngfuncs.Mod_ForName.expect("non-null function pointer")((*state).modelname.as_mut_ptr(),
                                                                              false_0,
                                                                              true_0)
            } else { (*state).model = 0 as *mut model_t }
            if (*state).model.is_null() {
                (*state).model = (*RI.currententity).model
            }
        }
    } else {
        if (*state).model != (*RI.currententity).model {
            (*state).model = (*RI.currententity).model
        }
        (*state).name[0 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_char
    }
    return (*state).model;
}
/*
================
R_GetEntityRenderMode

check for texture flags
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_GetEntityRenderMode(mut ent: *mut cl_entity_t)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut opaque: libc::c_int = 0;
    let mut trans: libc::c_int = 0;
    let mut ptexture: *mut mstudiotexture_t = 0 as *mut mstudiotexture_t;
    let mut oldent: *mut cl_entity_t = 0 as *mut cl_entity_t;
    let mut model: *mut model_t = 0 as *mut model_t;
    let mut phdr: *mut studiohdr_t = 0 as *mut studiohdr_t;
    oldent = RI.currententity;
    RI.currententity = ent;
    if (*ent).player as u64 != 0 {
        // check it for real playermodel
        model =
            R_StudioSetupPlayerModel((*ent).curstate.number -
                                         1 as libc::c_int)
    } else { model = (*ent).model }
    RI.currententity = oldent;
    phdr =
        gEngfuncs.Mod_Extradata.expect("non-null function pointer")(mod_studio
                                                                        as
                                                                        libc::c_int,
                                                                    model) as
            *mut studiohdr_t;
    if phdr.is_null() {
        if (*ent).curstate.rendermode == kRenderNormal as libc::c_int {
            // forcing to choose right sorting type
            if !model.is_null() &&
                   (*model).type_0 as libc::c_int == mod_brush as libc::c_int
                   &&
                   (*model).flags as libc::c_uint &
                       (1 as libc::c_uint) << 3 as libc::c_int != 0 {
                return kRenderTransAlpha as libc::c_int
            }
        }
        return (*ent).curstate.rendermode
    }
    ptexture =
        (phdr as *mut byte).offset((*phdr).textureindex as isize) as
            *mut mstudiotexture_t;
    i = 0 as libc::c_int;
    trans = i;
    opaque = trans;
    while i < (*phdr).numtextures {
        // ignore chrome & additive it's just a specular-like effect
        if (*ptexture).flags & 0x20 as libc::c_int as libc::c_uint != 0 &&
               (*ptexture).flags & 0x2 as libc::c_int as libc::c_uint == 0 {
            trans += 1
        } else { opaque += 1 }
        i += 1;
        ptexture = ptexture.offset(1)
    }
    // if model is more additive than opaque
    if trans > opaque { return kRenderTransAdd as libc::c_int }
    return (*ent).curstate.rendermode;
}
/*
===============
R_StudioClientEvents

===============
*/
unsafe extern "C" fn R_StudioClientEvents() {
    let mut pseqdesc: *mut mstudioseqdesc_t =
        0 as *mut mstudioseqdesc_t; // gamepaused
    let mut pevent: *mut mstudioevent_t = 0 as *mut mstudioevent_t;
    let mut e: *mut cl_entity_t = RI.currententity;
    let mut i: libc::c_int = 0;
    let mut sequence: libc::c_int = 0;
    let mut end: libc::c_float = 0.;
    let mut start: libc::c_float = 0.;
    if g_studio.frametime == 0.0f64 { return }
    // fill attachments with interpolated origin
    if (*m_pStudioHeader).numattachments <= 0 as libc::c_int {
        Matrix3x4_OriginFromMatrix(g_studio.rotationmatrix.as_mut_ptr() as
                                       *const [vec_t; 4],
                                   (*e).attachment[0 as libc::c_int as
                                                       usize].as_mut_ptr());
        Matrix3x4_OriginFromMatrix(g_studio.rotationmatrix.as_mut_ptr() as
                                       *const [vec_t; 4],
                                   (*e).attachment[1 as libc::c_int as
                                                       usize].as_mut_ptr());
        Matrix3x4_OriginFromMatrix(g_studio.rotationmatrix.as_mut_ptr() as
                                       *const [vec_t; 4],
                                   (*e).attachment[2 as libc::c_int as
                                                       usize].as_mut_ptr());
        Matrix3x4_OriginFromMatrix(g_studio.rotationmatrix.as_mut_ptr() as
                                       *const [vec_t; 4],
                                   (*e).attachment[3 as libc::c_int as
                                                       usize].as_mut_ptr());
    }
    if (*e).curstate.effects & 2 as libc::c_int != 0 {
        let mut el: *mut dlight_t =
            gEngfuncs.CL_AllocElight.expect("non-null function pointer")(0 as
                                                                             libc::c_int);
        (*e).curstate.effects = (*e).curstate.effects & !(2 as libc::c_int);
        (*el).origin[0 as libc::c_int as usize] =
            (*e).attachment[0 as libc::c_int as
                                usize][0 as libc::c_int as usize];
        (*el).origin[1 as libc::c_int as usize] =
            (*e).attachment[0 as libc::c_int as
                                usize][1 as libc::c_int as usize];
        (*el).origin[2 as libc::c_int as usize] =
            (*e).attachment[0 as libc::c_int as
                                usize][2 as libc::c_int as usize];
        (*el).die = (*gpGlobals).time + 0.05f32;
        (*el).color.r = 255 as libc::c_int as byte;
        (*el).color.g = 192 as libc::c_int as byte;
        (*el).color.b = 64 as libc::c_int as byte;
        (*el).decay = 320 as libc::c_int as libc::c_float;
        (*el).radius = 24 as libc::c_int as libc::c_float
    }
    sequence =
        if (*e).curstate.sequence >= 0 as libc::c_int {
            if (*e).curstate.sequence <
                   (*m_pStudioHeader).numseq - 1 as libc::c_int {
                (*e).curstate.sequence
            } else { ((*m_pStudioHeader).numseq) - 1 as libc::c_int }
        } else { 0 as libc::c_int };
    pseqdesc =
        ((m_pStudioHeader as
              *mut byte).offset((*m_pStudioHeader).seqindex as isize) as
             *mut mstudioseqdesc_t).offset(sequence as isize);
    // no events for this animation
    if (*pseqdesc).numevents == 0 as libc::c_int { return }
    end = R_StudioEstimateFrame(e, pseqdesc);
    start =
        (end as libc::c_double -
             (*e).curstate.framerate as libc::c_double *
                 (*gpGlobals).frametime * (*pseqdesc).fps as libc::c_double)
            as libc::c_float;
    pevent =
        (m_pStudioHeader as *mut byte).offset((*pseqdesc).eventindex as isize)
            as *mut mstudioevent_t;
    if (*e).latched.sequencetime == (*e).curstate.animtime {
        if (*pseqdesc).flags & 0x1 as libc::c_int == 0 { start = -0.01f32 }
    }
    i = 0 as libc::c_int;
    while i < (*pseqdesc).numevents {
        // ignore all non-client-side events
        if !((*pevent.offset(i as isize)).event < 5000 as libc::c_int) {
            if (*pevent.offset(i as isize)).frame as libc::c_float > start &&
                   (*pevent.offset(i as isize)).frame as libc::c_float <= end
               {
                gEngfuncs.pfnStudioEvent.expect("non-null function pointer")(&mut *pevent.offset(i
                                                                                                     as
                                                                                                     isize),
                                                                             e);
            }
        }
        i += 1
    };
}
/*
===============
R_StudioGetForceFaceFlags

===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_StudioGetForceFaceFlags() -> libc::c_int {
    return g_nForceFaceFlags;
}
/*
===============
R_StudioSetForceFaceFlags

===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_StudioSetForceFaceFlags(mut flags: libc::c_int) {
    g_nForceFaceFlags = flags;
}
/*
===============
pfnStudioSetHeader

===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_StudioSetHeader(mut pheader: *mut studiohdr_t) {
    m_pStudioHeader = pheader;
    m_fDoRemap = false_0;
}
/*
===============
R_StudioSetRenderModel

===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_StudioSetRenderModel(mut model: *mut model_t) {
    RI.currentmodel = model;
}
/*
===============
R_StudioSetupRenderer

===============
*/
unsafe extern "C" fn R_StudioSetupRenderer(mut rendermode: libc::c_int) {
    let mut phdr: *mut studiohdr_t = m_pStudioHeader;
    let mut i: libc::c_int = 0;
    if rendermode > kRenderTransAdd as libc::c_int {
        rendermode = 0 as libc::c_int
    }
    g_studio.rendermode =
        if rendermode >= 0 as libc::c_int {
            if rendermode < kRenderTransAdd as libc::c_int {
                rendermode
            } else { kRenderTransAdd as libc::c_int }
        } else { 0 as libc::c_int };
    pglTexEnvf.expect("non-null function pointer")(0x2300 as libc::c_int as
                                                       GLenum,
                                                   0x2200 as libc::c_int as
                                                       GLenum,
                                                   0x2100 as libc::c_int as
                                                       GLfloat);
    pglDisable.expect("non-null function pointer")(0xbc0 as libc::c_int as
                                                       GLenum);
    pglShadeModel.expect("non-null function pointer")(0x1d01 as libc::c_int as
                                                          GLenum);
    // a point to setup local to world transform for boneweighted models
    if !phdr.is_null() &&
           (*phdr).flags as libc::c_uint &
               (1 as libc::c_uint) << 30 as libc::c_int != 0 {
        // NOTE: extended boneinfo goes immediately after bones
        let mut boneinfo: *mut mstudioboneinfo_t =
            (phdr as
                 *mut byte).offset((*phdr).boneindex as
                                       isize).offset(((*phdr).numbones as
                                                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<mstudiobone_t>()
                                                                                          as
                                                                                          libc::c_ulong)
                                                         as isize) as
                *mut mstudioboneinfo_t;
        i = 0 as libc::c_int;
        while i < (*phdr).numbones {
            Matrix3x4_ConcatTransforms(g_studio.worldtransform[i as
                                                                   usize].as_mut_ptr(),
                                       g_studio.bonestransform[i as
                                                                   usize].as_mut_ptr()
                                           as *const [vec_t; 4],
                                       (*boneinfo.offset(i as
                                                             isize)).poseToBone.as_mut_ptr()
                                           as *const [vec_t; 4]);
            i += 1
        }
    };
}
/*
===============
R_StudioRestoreRenderer

===============
*/
unsafe extern "C" fn R_StudioRestoreRenderer() {
    if g_studio.rendermode != kRenderNormal as libc::c_int {
        pglDisable.expect("non-null function pointer")(0xbe2 as libc::c_int as
                                                           GLenum);
    }
    pglTexEnvf.expect("non-null function pointer")(0x2300 as libc::c_int as
                                                       GLenum,
                                                   0x2200 as libc::c_int as
                                                       GLenum,
                                                   0x1e01 as libc::c_int as
                                                       GLfloat);
    pglShadeModel.expect("non-null function pointer")(0x1d00 as libc::c_int as
                                                          GLenum);
    m_fDoRemap = false_0;
}
/*
===============
R_StudioSetChromeOrigin

===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_StudioSetChromeOrigin() {
    g_studio.chrome_origin[0 as libc::c_int as usize] =
        RI.vieworg[0 as libc::c_int as usize];
    g_studio.chrome_origin[1 as libc::c_int as usize] =
        RI.vieworg[1 as libc::c_int as usize];
    g_studio.chrome_origin[2 as libc::c_int as usize] =
        RI.vieworg[2 as libc::c_int as usize];
}
/*
===============
pfnIsHardware

Xash3D is always works in hardware mode
===============
*/
unsafe extern "C" fn pfnIsHardware() -> libc::c_int {
    return 1 as libc::c_int;
    // 0 is Software, 1 is OpenGL, 2 is Direct3D
}
/*
===============
R_StudioDrawPointsShadow

===============
*/
unsafe extern "C" fn R_StudioDrawPointsShadow() {
    let mut av: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut height: libc::c_float = 0.;
    let mut vec_x: libc::c_float = 0.;
    let mut vec_y: libc::c_float = 0.;
    let mut pmesh: *mut mstudiomesh_t = 0 as *mut mstudiomesh_t;
    let mut point: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    if (*RI.currententity).curstate.effects as libc::c_uint &
           (1 as libc::c_uint) << 28 as libc::c_int != 0 {
        return
    }
    if glState.stencilEnabled as u64 != 0 {
        pglEnable.expect("non-null function pointer")(0xb90 as libc::c_int as
                                                          GLenum);
    }
    height = g_studio.lightspot[2 as libc::c_int as usize] + 1.0f32;
    vec_x = -g_studio.lightvec[0 as libc::c_int as usize] * 8.0f32;
    vec_y = -g_studio.lightvec[1 as libc::c_int as usize] * 8.0f32;
    k = 0 as libc::c_int;
    while k < (*m_pSubModel).nummesh {
        let mut ptricmds: *mut libc::c_short = 0 as *mut libc::c_short;
        pmesh =
            ((m_pStudioHeader as
                  *mut byte).offset((*m_pSubModel).meshindex as isize) as
                 *mut mstudiomesh_t).offset(k as isize);
        ptricmds =
            (m_pStudioHeader as *mut byte).offset((*pmesh).triindex as isize)
                as *mut libc::c_short;
        r_stats.c_studio_polys =
            (r_stats.c_studio_polys as
                 libc::c_uint).wrapping_add((*pmesh).numtris as libc::c_uint)
                as uint as uint;
        loop  {
            let fresh23 = ptricmds;
            ptricmds = ptricmds.offset(1);
            i = *fresh23 as libc::c_int;
            if !(i != 0) { break ; }
            if i < 0 as libc::c_int {
                pglBegin.expect("non-null function pointer")(0x6 as
                                                                 libc::c_int
                                                                 as GLenum);
                i = -i
            } else {
                pglBegin.expect("non-null function pointer")(0x5 as
                                                                 libc::c_int
                                                                 as GLenum);
            }
            while i > 0 as libc::c_int {
                av =
                    g_studio.verts[*ptricmds.offset(0 as libc::c_int as isize)
                                       as usize].as_mut_ptr();
                point[0 as libc::c_int as usize] =
                    *av.offset(0 as libc::c_int as isize) -
                        vec_x *
                            (*av.offset(2 as libc::c_int as isize) -
                                 g_studio.lightspot[2 as libc::c_int as
                                                        usize]);
                point[1 as libc::c_int as usize] =
                    *av.offset(1 as libc::c_int as isize) -
                        vec_y *
                            (*av.offset(2 as libc::c_int as isize) -
                                 g_studio.lightspot[2 as libc::c_int as
                                                        usize]);
                point[2 as libc::c_int as usize] =
                    g_studio.lightspot[2 as libc::c_int as usize] + 1.0f32;
                pglVertex3fv.expect("non-null function pointer")(point.as_mut_ptr());
                i -= 1;
                ptricmds = ptricmds.offset(4 as libc::c_int as isize)
            }
            pglEnd.expect("non-null function pointer")();
        }
        k += 1
    }
    if glState.stencilEnabled as u64 != 0 {
        pglDisable.expect("non-null function pointer")(0xb90 as libc::c_int as
                                                           GLenum);
    };
}
/*
===============
GL_StudioSetRenderMode

set rendermode for studiomodel
===============
*/
#[no_mangle]
pub unsafe extern "C" fn GL_StudioSetRenderMode(mut rendermode: libc::c_int) {
    match rendermode {
        0 => { }
        1 => {
            pglBlendFunc.expect("non-null function pointer")(0x302 as
                                                                 libc::c_int
                                                                 as GLenum,
                                                             0x303 as
                                                                 libc::c_int
                                                                 as GLenum);
            pglTexEnvi.expect("non-null function pointer")(0x2300 as
                                                               libc::c_int as
                                                               GLenum,
                                                           0x2200 as
                                                               libc::c_int as
                                                               GLenum,
                                                           0x2100 as
                                                               libc::c_int);
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
            pglDepthMask.expect("non-null function pointer")(0x1 as
                                                                 libc::c_int
                                                                 as
                                                                 GLboolean);
            pglEnable.expect("non-null function pointer")(0xbe2 as libc::c_int
                                                              as GLenum);
        }
    };
}
/*
===============
GL_StudioDrawShadow

g-cont: don't modify this code it's 100% matched with
original GoldSrc code and used in some mods to enable
studio shadows with some asm tricks
===============
*/
unsafe extern "C" fn GL_StudioDrawShadow() {
    pglDepthMask.expect("non-null function pointer")(0x1 as libc::c_int as
                                                         GLboolean);
    if r_shadows.value != 0. &&
           g_studio.rendermode != kRenderTransAdd as libc::c_int &&
           (*RI.currentmodel).flags as libc::c_uint &
               (1 as libc::c_uint) << 8 as libc::c_int == 0 {
        let mut color: libc::c_float = 1.0f32 - tr.blend * 0.5f32;
        pglDisable.expect("non-null function pointer")(0xde1 as libc::c_int as
                                                           GLenum);
        pglBlendFunc.expect("non-null function pointer")(0x302 as libc::c_int
                                                             as GLenum,
                                                         0x303 as libc::c_int
                                                             as GLenum);
        pglEnable.expect("non-null function pointer")(0xbe2 as libc::c_int as
                                                          GLenum);
        pglColor4f.expect("non-null function pointer")(0.0f32, 0.0f32, 0.0f32,
                                                       1.0f32 - color);
        pglDepthFunc.expect("non-null function pointer")(0x201 as libc::c_int
                                                             as GLenum);
        R_StudioDrawPointsShadow();
        pglDepthFunc.expect("non-null function pointer")(0x203 as libc::c_int
                                                             as GLenum);
        pglEnable.expect("non-null function pointer")(0xde1 as libc::c_int as
                                                          GLenum);
        pglDisable.expect("non-null function pointer")(0xbe2 as libc::c_int as
                                                           GLenum);
        pglColor4f.expect("non-null function pointer")(1.0f32, 1.0f32, 1.0f32,
                                                       1.0f32);
        pglShadeModel.expect("non-null function pointer")(0x1d01 as
                                                              libc::c_int as
                                                              GLenum);
    };
}
/*
====================
StudioRenderFinal

====================
*/
#[no_mangle]
pub unsafe extern "C" fn R_StudioRenderFinal() {
    let mut i: libc::c_int = 0;
    let mut rendermode: libc::c_int = 0;
    rendermode =
        if R_StudioGetForceFaceFlags() != 0 {
            kRenderTransAdd as libc::c_int
        } else { (*RI.currententity).curstate.rendermode };
    R_StudioSetupRenderer(rendermode);
    if (*r_drawentities).value == 2 as libc::c_int as libc::c_float {
        R_StudioDrawBones();
    } else if (*r_drawentities).value == 3 as libc::c_int as libc::c_float {
        R_StudioDrawHulls();
    } else {
        i = 0 as libc::c_int;
        while i < (*m_pStudioHeader).numbodyparts {
            R_StudioSetupModel(i,
                               &mut m_pBodyPart as
                                   *mut *mut mstudiobodyparts_t as
                                   *mut *mut libc::c_void,
                               &mut m_pSubModel as *mut *mut mstudiomodel_t as
                                   *mut *mut libc::c_void);
            GL_StudioSetRenderMode(rendermode);
            R_StudioDrawPoints();
            GL_StudioDrawShadow();
            i += 1
        }
    }
    if (*r_drawentities).value == 4 as libc::c_int as libc::c_float {
        TriRenderMode(kRenderTransAdd as libc::c_int);
        R_StudioDrawHulls();
        TriRenderMode(kRenderNormal as libc::c_int);
    }
    if (*r_drawentities).value == 5 as libc::c_int as libc::c_float {
        R_StudioDrawAbsBBox();
    }
    if (*r_drawentities).value == 6 as libc::c_int as libc::c_float {
        R_StudioDrawAttachments();
    }
    if (*r_drawentities).value == 7 as libc::c_int as libc::c_float {
        let mut origin: vec3_t = [0.; 3];
        pglDisable.expect("non-null function pointer")(0xde1 as libc::c_int as
                                                           GLenum);
        pglDisable.expect("non-null function pointer")(0xb71 as libc::c_int as
                                                           GLenum);
        Matrix3x4_OriginFromMatrix(g_studio.rotationmatrix.as_mut_ptr() as
                                       *const [vec_t; 4],
                                   origin.as_mut_ptr());
        pglBegin.expect("non-null function pointer")(0x1 as libc::c_int as
                                                         GLenum);
        pglColor3f.expect("non-null function pointer")(1 as libc::c_int as
                                                           GLfloat,
                                                       0.5f64 as GLfloat,
                                                       0 as libc::c_int as
                                                           GLfloat);
        pglVertex3fv.expect("non-null function pointer")(origin.as_mut_ptr());
        pglVertex3fv.expect("non-null function pointer")(g_studio.lightspot.as_mut_ptr());
        pglEnd.expect("non-null function pointer")();
        pglBegin.expect("non-null function pointer")(0x1 as libc::c_int as
                                                         GLenum);
        pglColor3f.expect("non-null function pointer")(0 as libc::c_int as
                                                           GLfloat,
                                                       0.5f64 as GLfloat,
                                                       1 as libc::c_int as
                                                           GLfloat);
        origin[0 as libc::c_int as usize] =
            g_studio.lightspot[0 as libc::c_int as usize] +
                -64.0f32 * g_studio.lightvec[0 as libc::c_int as usize];
        origin[1 as libc::c_int as usize] =
            g_studio.lightspot[1 as libc::c_int as usize] +
                -64.0f32 * g_studio.lightvec[1 as libc::c_int as usize];
        origin[2 as libc::c_int as usize] =
            g_studio.lightspot[2 as libc::c_int as usize] +
                -64.0f32 * g_studio.lightvec[2 as libc::c_int as usize];
        pglVertex3fv.expect("non-null function pointer")(g_studio.lightspot.as_mut_ptr());
        pglVertex3fv.expect("non-null function pointer")(origin.as_mut_ptr());
        pglEnd.expect("non-null function pointer")();
        pglPointSize.expect("non-null function pointer")(5.0f32);
        pglColor3f.expect("non-null function pointer")(1 as libc::c_int as
                                                           GLfloat,
                                                       0 as libc::c_int as
                                                           GLfloat,
                                                       0 as libc::c_int as
                                                           GLfloat);
        pglBegin.expect("non-null function pointer")(0 as libc::c_int as
                                                         GLenum);
        pglVertex3fv.expect("non-null function pointer")(g_studio.lightspot.as_mut_ptr());
        pglEnd.expect("non-null function pointer")();
        pglPointSize.expect("non-null function pointer")(1.0f32);
        pglEnable.expect("non-null function pointer")(0xb71 as libc::c_int as
                                                          GLenum);
        pglEnable.expect("non-null function pointer")(0xde1 as libc::c_int as
                                                          GLenum);
    }
    R_StudioRestoreRenderer();
}
/*
====================
StudioRenderModel

====================
*/
#[no_mangle]
pub unsafe extern "C" fn R_StudioRenderModel() {
    R_StudioSetChromeOrigin();
    R_StudioSetForceFaceFlags(0 as libc::c_int);
    if (*RI.currententity).curstate.renderfx ==
           kRenderFxGlowShell as libc::c_int {
        (*RI.currententity).curstate.renderfx = kRenderFxNone as libc::c_int;
        R_StudioRenderFinal();
        R_StudioSetForceFaceFlags(0x2 as libc::c_int);
        TriSpriteTexture(R_GetChromeSprite(), 0 as libc::c_int);
        (*RI.currententity).curstate.renderfx =
            kRenderFxGlowShell as libc::c_int;
        R_StudioRenderFinal();
    } else { R_StudioRenderFinal(); };
}
/*
====================
StudioEstimateGait

====================
*/
#[no_mangle]
pub unsafe extern "C" fn R_StudioEstimateGait(mut pplayer:
                                                  *mut entity_state_t) {
    let mut est_velocity: vec3_t = [0.; 3];
    let mut dt: libc::c_float = 0.;
    dt =
        if g_studio.frametime >= 0.0f32 as libc::c_double {
            if g_studio.frametime < 1.0f32 as libc::c_double {
                g_studio.frametime
            } else { 1.0f32 as libc::c_double }
        } else { 0.0f32 as libc::c_double } as libc::c_float;
    if dt == 0.0f32 || (*m_pPlayerInfo).renderframe == tr.realframecount {
        m_flGaitMovement = 0 as libc::c_int as libc::c_float;
        return
    }
    est_velocity[0 as libc::c_int as usize] =
        (*RI.currententity).origin[0 as libc::c_int as usize] -
            (*m_pPlayerInfo).prevgaitorigin[0 as libc::c_int as usize];
    est_velocity[1 as libc::c_int as usize] =
        (*RI.currententity).origin[1 as libc::c_int as usize] -
            (*m_pPlayerInfo).prevgaitorigin[1 as libc::c_int as usize];
    est_velocity[2 as libc::c_int as usize] =
        (*RI.currententity).origin[2 as libc::c_int as usize] -
            (*m_pPlayerInfo).prevgaitorigin[2 as libc::c_int as usize];
    (*m_pPlayerInfo).prevgaitorigin[0 as libc::c_int as usize] =
        (*RI.currententity).origin[0 as libc::c_int as usize];
    (*m_pPlayerInfo).prevgaitorigin[1 as libc::c_int as usize] =
        (*RI.currententity).origin[1 as libc::c_int as usize];
    (*m_pPlayerInfo).prevgaitorigin[2 as libc::c_int as usize] =
        (*RI.currententity).origin[2 as libc::c_int as usize];
    m_flGaitMovement =
        __tg_sqrt(est_velocity[0 as libc::c_int as usize] *
                      est_velocity[0 as libc::c_int as usize] +
                      est_velocity[1 as libc::c_int as usize] *
                          est_velocity[1 as libc::c_int as usize] +
                      est_velocity[2 as libc::c_int as usize] *
                          est_velocity[2 as libc::c_int as usize]);
    if dt <= 0.0f32 || m_flGaitMovement / dt < 5.0f32 {
        m_flGaitMovement = 0.0f32;
        est_velocity[0 as libc::c_int as usize] = 0.0f32;
        est_velocity[1 as libc::c_int as usize] = 0.0f32
    }
    if est_velocity[1 as libc::c_int as usize] == 0.0f32 &&
           est_velocity[0 as libc::c_int as usize] == 0.0f32 {
        let mut flYawDiff: libc::c_float =
            (*RI.currententity).angles[1 as libc::c_int as usize] -
                (*m_pPlayerInfo).gaityaw;
        flYawDiff =
            flYawDiff -
                ((flYawDiff / 360 as libc::c_int as libc::c_float) as
                     libc::c_int * 360 as libc::c_int) as libc::c_float;
        if flYawDiff > 180.0f32 { flYawDiff -= 360.0f32 }
        if flYawDiff < -180.0f32 { flYawDiff += 360.0f32 }
        if dt < 0.25f32 { flYawDiff *= dt * 4.0f32 } else { flYawDiff *= dt }
        (*m_pPlayerInfo).gaityaw += flYawDiff;
        (*m_pPlayerInfo).gaityaw =
            (*m_pPlayerInfo).gaityaw -
                (((*m_pPlayerInfo).gaityaw /
                      360 as libc::c_int as libc::c_float) as libc::c_int *
                     360 as libc::c_int) as libc::c_float;
        m_flGaitMovement = 0.0f32
    } else {
        (*m_pPlayerInfo).gaityaw =
            __tg_atan2(est_velocity[1 as libc::c_int as usize],
                       est_velocity[0 as libc::c_int as usize]) *
                180 as libc::c_int as libc::c_float /
                3.14159265358979323846f64 as libc::c_float;
        if (*m_pPlayerInfo).gaityaw > 180.0f32 {
            (*m_pPlayerInfo).gaityaw = 180.0f32
        }
        if (*m_pPlayerInfo).gaityaw < -180.0f32 {
            (*m_pPlayerInfo).gaityaw = -180.0f32
        }
    };
}
/*
====================
StudioProcessGait

====================
*/
#[no_mangle]
pub unsafe extern "C" fn R_StudioProcessGait(mut pplayer:
                                                 *mut entity_state_t) {
    let mut pseqdesc: *mut mstudioseqdesc_t =
        0 as *mut mstudioseqdesc_t; // view direction relative to movement
    let mut iBlend: libc::c_int = 0;
    let mut dt: libc::c_float = 0.;
    let mut flYaw: libc::c_float = 0.;
    if (*RI.currententity).curstate.sequence >= (*m_pStudioHeader).numseq {
        (*RI.currententity).curstate.sequence = 0 as libc::c_int
    }
    dt =
        if g_studio.frametime >= 0.0f32 as libc::c_double {
            if g_studio.frametime < 1.0f32 as libc::c_double {
                g_studio.frametime
            } else { 1.0f32 as libc::c_double }
        } else { 0.0f32 as libc::c_double } as libc::c_float;
    pseqdesc =
        ((m_pStudioHeader as
              *mut byte).offset((*m_pStudioHeader).seqindex as isize) as
             *mut mstudioseqdesc_t).offset((*RI.currententity).curstate.sequence
                                               as isize);
    R_StudioPlayerBlend(pseqdesc, &mut iBlend,
                        &mut *(*RI.currententity).angles.as_mut_ptr().offset(0
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 isize));
    (*RI.currententity).latched.prevangles[0 as libc::c_int as usize] =
        (*RI.currententity).angles[0 as libc::c_int as usize];
    (*RI.currententity).curstate.blending[0 as libc::c_int as usize] =
        iBlend as byte;
    (*RI.currententity).latched.prevblending[0 as libc::c_int as usize] =
        (*RI.currententity).curstate.blending[0 as libc::c_int as usize];
    (*RI.currententity).latched.prevseqblending[0 as libc::c_int as usize] =
        (*RI.currententity).curstate.blending[0 as libc::c_int as usize];
    R_StudioEstimateGait(pplayer);
    // calc side to side turning
    flYaw =
        (*RI.currententity).angles[1 as libc::c_int as usize] -
            (*m_pPlayerInfo).gaityaw;
    flYaw =
        flYaw -
            ((flYaw / 360 as libc::c_int as libc::c_float) as libc::c_int *
                 360 as libc::c_int) as libc::c_float;
    if flYaw < -180.0f32 { flYaw = flYaw + 360.0f32 }
    if flYaw > 180.0f32 { flYaw = flYaw - 360.0f32 }
    if flYaw > 120.0f32 {
        (*m_pPlayerInfo).gaityaw = (*m_pPlayerInfo).gaityaw - 180.0f32;
        m_flGaitMovement = -m_flGaitMovement;
        flYaw = flYaw - 180.0f32
    } else if flYaw < -120.0f32 {
        (*m_pPlayerInfo).gaityaw = (*m_pPlayerInfo).gaityaw + 180.0f32;
        m_flGaitMovement = -m_flGaitMovement;
        flYaw = flYaw + 180.0f32
    }
    // adjust torso
    (*RI.currententity).curstate.controller[0 as libc::c_int as usize] =
        ((flYaw / 4.0f32 + 30.0f32) / (60.0f32 / 255.0f32)) as byte;
    (*RI.currententity).curstate.controller[1 as libc::c_int as usize] =
        ((flYaw / 4.0f32 + 30.0f32) / (60.0f32 / 255.0f32)) as byte;
    (*RI.currententity).curstate.controller[2 as libc::c_int as usize] =
        ((flYaw / 4.0f32 + 30.0f32) / (60.0f32 / 255.0f32)) as byte;
    (*RI.currententity).curstate.controller[3 as libc::c_int as usize] =
        ((flYaw / 4.0f32 + 30.0f32) / (60.0f32 / 255.0f32)) as byte;
    (*RI.currententity).latched.prevcontroller[0 as libc::c_int as usize] =
        (*RI.currententity).curstate.controller[0 as libc::c_int as usize];
    (*RI.currententity).latched.prevcontroller[1 as libc::c_int as usize] =
        (*RI.currententity).curstate.controller[1 as libc::c_int as usize];
    (*RI.currententity).latched.prevcontroller[2 as libc::c_int as usize] =
        (*RI.currententity).curstate.controller[2 as libc::c_int as usize];
    (*RI.currententity).latched.prevcontroller[3 as libc::c_int as usize] =
        (*RI.currententity).curstate.controller[3 as libc::c_int as usize];
    (*RI.currententity).angles[1 as libc::c_int as usize] =
        (*m_pPlayerInfo).gaityaw;
    if (*RI.currententity).angles[1 as libc::c_int as usize] <
           -(0 as libc::c_int) as libc::c_float {
        (*RI.currententity).angles[1 as libc::c_int as usize] += 360.0f32
    }
    (*RI.currententity).latched.prevangles[1 as libc::c_int as usize] =
        (*RI.currententity).angles[1 as libc::c_int as usize];
    if (*pplayer).gaitsequence >= (*m_pStudioHeader).numseq {
        (*pplayer).gaitsequence = 0 as libc::c_int
    }
    pseqdesc =
        ((m_pStudioHeader as
              *mut byte).offset((*m_pStudioHeader).seqindex as isize) as
             *mut mstudioseqdesc_t).offset((*pplayer).gaitsequence as isize);
    // calc gait frame
    if (*pseqdesc).linearmovement[0 as libc::c_int as usize] >
           0 as libc::c_int as libc::c_float {
        (*m_pPlayerInfo).gaitframe +=
            m_flGaitMovement /
                (*pseqdesc).linearmovement[0 as libc::c_int as usize] *
                (*pseqdesc).numframes as libc::c_float
    } else { (*m_pPlayerInfo).gaitframe += (*pseqdesc).fps * dt }
    // do modulo
    (*m_pPlayerInfo).gaitframe =
        (*m_pPlayerInfo).gaitframe -
            (((*m_pPlayerInfo).gaitframe /
                  (*pseqdesc).numframes as libc::c_float) as libc::c_int *
                 (*pseqdesc).numframes) as libc::c_float;
    if (*m_pPlayerInfo).gaitframe < 0 as libc::c_int as libc::c_float {
        (*m_pPlayerInfo).gaitframe += (*pseqdesc).numframes as libc::c_float
    };
}
/*
===============
R_StudioDrawPlayer

===============
*/
unsafe extern "C" fn R_StudioDrawPlayer(mut flags: libc::c_int,
                                        mut pplayer: *mut entity_state_t)
 -> libc::c_int {
    let mut m_nPlayerIndex: libc::c_int = 0;
    let mut lighting: alight_t =
        alight_t{ambientlight: 0,
                 shadelight: 0,
                 color: [0.; 3],
                 plightvec: 0 as *mut libc::c_float,};
    let mut dir: vec3_t = [0.; 3];
    m_nPlayerIndex = (*pplayer).number - 1 as libc::c_int;
    if m_nPlayerIndex < 0 as libc::c_int ||
           m_nPlayerIndex >=
               Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(PARM_MAX_CLIENTS
                                                                                                                         as
                                                                                                                         libc::c_int,
                                                                                                                     0
                                                                                                                         as
                                                                                                                         libc::c_int)
       {
        return 0 as libc::c_int
    }
    RI.currentmodel = R_StudioSetupPlayerModel(m_nPlayerIndex);
    if RI.currentmodel.is_null() { return 0 as libc::c_int }
    R_StudioSetHeader(gEngfuncs.Mod_Extradata.expect("non-null function pointer")(mod_studio
                                                                                      as
                                                                                      libc::c_int,
                                                                                  RI.currentmodel)
                          as *mut studiohdr_t);
    if (*pplayer).gaitsequence != 0 {
        let mut orig_angles: vec3_t = [0.; 3];
        m_pPlayerInfo = pfnPlayerInfo(m_nPlayerIndex);
        orig_angles[0 as libc::c_int as usize] =
            (*RI.currententity).angles[0 as libc::c_int as usize];
        orig_angles[1 as libc::c_int as usize] =
            (*RI.currententity).angles[1 as libc::c_int as usize];
        orig_angles[2 as libc::c_int as usize] =
            (*RI.currententity).angles[2 as libc::c_int as usize];
        R_StudioProcessGait(pplayer);
        (*m_pPlayerInfo).gaitsequence = (*pplayer).gaitsequence;
        m_pPlayerInfo = 0 as *mut player_info_t;
        R_StudioSetUpTransform(RI.currententity);
        (*RI.currententity).angles[0 as libc::c_int as usize] =
            orig_angles[0 as libc::c_int as usize];
        (*RI.currententity).angles[1 as libc::c_int as usize] =
            orig_angles[1 as libc::c_int as usize];
        (*RI.currententity).angles[2 as libc::c_int as usize] =
            orig_angles[2 as libc::c_int as usize]
    } else {
        (*RI.currententity).curstate.controller[0 as libc::c_int as usize] =
            127 as libc::c_int as byte;
        (*RI.currententity).curstate.controller[1 as libc::c_int as usize] =
            127 as libc::c_int as byte;
        (*RI.currententity).curstate.controller[2 as libc::c_int as usize] =
            127 as libc::c_int as byte;
        (*RI.currententity).curstate.controller[3 as libc::c_int as usize] =
            127 as libc::c_int as byte;
        (*RI.currententity).latched.prevcontroller[0 as libc::c_int as usize]
            =
            (*RI.currententity).curstate.controller[0 as libc::c_int as
                                                        usize];
        (*RI.currententity).latched.prevcontroller[1 as libc::c_int as usize]
            =
            (*RI.currententity).curstate.controller[1 as libc::c_int as
                                                        usize];
        (*RI.currententity).latched.prevcontroller[2 as libc::c_int as usize]
            =
            (*RI.currententity).curstate.controller[2 as libc::c_int as
                                                        usize];
        (*RI.currententity).latched.prevcontroller[3 as libc::c_int as usize]
            =
            (*RI.currententity).curstate.controller[3 as libc::c_int as
                                                        usize];
        m_pPlayerInfo = pfnPlayerInfo(m_nPlayerIndex);
        (*m_pPlayerInfo).gaitsequence = 0 as libc::c_int;
        R_StudioSetUpTransform(RI.currententity);
    }
    if flags & 1 as libc::c_int != 0 {
        // see if the bounding box lets us trivially reject, also sets
        if R_StudioCheckBBox() == 0 {
            return 0 as libc::c_int
        } // render data cache cookie
        r_stats.c_studio_models_drawn =
            r_stats.c_studio_models_drawn.wrapping_add(1);
        g_studio.framecount += 1;
        if (*m_pStudioHeader).numbodyparts == 0 as libc::c_int {
            return 1 as libc::c_int
        }
    }
    m_pPlayerInfo = pfnPlayerInfo(m_nPlayerIndex);
    R_StudioSetupBones(RI.currententity);
    R_StudioSaveBones();
    (*m_pPlayerInfo).renderframe = tr.realframecount;
    m_pPlayerInfo = 0 as *mut player_info_t;
    if flags & 2 as libc::c_int != 0 {
        R_StudioCalcAttachments();
        R_StudioClientEvents();
        // copy attachments into global entity array
        if (*RI.currententity).index > 0 as libc::c_int {
            let mut ent: *mut cl_entity_t =
                gEngfuncs.GetEntityByIndex.expect("non-null function pointer")((*RI.currententity).index);
            memcpy((*ent).attachment.as_mut_ptr() as *mut libc::c_void,
                   (*RI.currententity).attachment.as_mut_ptr() as
                       *const libc::c_void,
                   (::std::mem::size_of::<vec3_t>() as
                        libc::c_ulong).wrapping_mul(4 as libc::c_int as
                                                        libc::c_ulong));
        }
    }
    if flags & 1 as libc::c_int != 0 {
        if (*cl_himodels).value != 0. &&
               RI.currentmodel != (*RI.currententity).model {
            // show highest resolution multiplayer model
            (*RI.currententity).curstate.body = 255 as libc::c_int
        } // force helmet
        if !((*gpGlobals).developer as u64 == 0 &&
                 Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(PARM_MAX_CLIENTS
                                                                                                                           as
                                                                                                                           libc::c_int,
                                                                                                                       0
                                                                                                                           as
                                                                                                                           libc::c_int)
                     == 1 as libc::c_int) &&
               RI.currentmodel == (*RI.currententity).model {
            (*RI.currententity).curstate.body = 1 as libc::c_int
        }
        lighting.plightvec = dir.as_mut_ptr();
        R_StudioDynamicLight(RI.currententity, &mut lighting);
        R_StudioEntityLight(&mut lighting);
        // model and frame independant
        R_StudioSetupLighting(&mut lighting);
        m_pPlayerInfo = pfnPlayerInfo(m_nPlayerIndex);
        // get remap colors
        g_nTopColor = (*m_pPlayerInfo).topcolor;
        g_nBottomColor = (*m_pPlayerInfo).bottomcolor;
        if g_nTopColor < 0 as libc::c_int { g_nTopColor = 0 as libc::c_int }
        if g_nTopColor > 360 as libc::c_int {
            g_nTopColor = 360 as libc::c_int
        }
        if g_nBottomColor < 0 as libc::c_int {
            g_nBottomColor = 0 as libc::c_int
        }
        if g_nBottomColor > 360 as libc::c_int {
            g_nBottomColor = 360 as libc::c_int
        }
        R_StudioSetRemapColors(g_nTopColor, g_nBottomColor);
        R_StudioRenderModel();
        m_pPlayerInfo = 0 as *mut player_info_t;
        if (*pplayer).weaponmodel != 0 {
            let mut saveent: cl_entity_t = *RI.currententity;
            let mut pweaponmodel: *mut model_t =
                gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")((*pplayer).weaponmodel);
            m_pStudioHeader =
                gEngfuncs.Mod_Extradata.expect("non-null function pointer")(mod_studio
                                                                                as
                                                                                libc::c_int,
                                                                            pweaponmodel)
                    as *mut studiohdr_t;
            R_StudioMergeBones(RI.currententity, pweaponmodel);
            R_StudioSetupLighting(&mut lighting);
            R_StudioRenderModel();
            R_StudioCalcAttachments();
            *RI.currententity = saveent
        }
    }
    return 1 as libc::c_int;
}
/*
===============
R_StudioDrawModel

===============
*/
unsafe extern "C" fn R_StudioDrawModel(mut flags: libc::c_int)
 -> libc::c_int {
    let mut lighting: alight_t =
        alight_t{ambientlight: 0,
                 shadelight: 0,
                 color: [0.; 3],
                 plightvec: 0 as *mut libc::c_float,};
    let mut dir: vec3_t = [0.; 3];
    if (*RI.currententity).curstate.renderfx ==
           kRenderFxDeadPlayer as libc::c_int {
        let mut deadplayer: entity_state_t =
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
                           vuser4: [0.; 3],};
        let mut result: libc::c_int = 0;
        if (*RI.currententity).curstate.renderamt <= 0 as libc::c_int ||
               (*RI.currententity).curstate.renderamt >
                   Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(PARM_MAX_CLIENTS
                                                                                                                             as
                                                                                                                             libc::c_int,
                                                                                                                         0
                                                                                                                             as
                                                                                                                             libc::c_int)
           {
            return 0 as libc::c_int
        }
        // get copy of player
        deadplayer =
            *R_StudioGetPlayerState((*RI.currententity).curstate.renderamt -
                                        1 as libc::c_int);
        // clear weapon, movement state
        deadplayer.number =
            (*RI.currententity).curstate.renderamt; // draw as though it were a player
        deadplayer.weaponmodel = 0 as libc::c_int;
        deadplayer.gaitsequence = 0 as libc::c_int;
        deadplayer.movetype = 0 as libc::c_int;
        deadplayer.angles[0 as libc::c_int as usize] =
            (*RI.currententity).curstate.angles[0 as libc::c_int as usize];
        deadplayer.angles[1 as libc::c_int as usize] =
            (*RI.currententity).curstate.angles[1 as libc::c_int as usize];
        deadplayer.angles[2 as libc::c_int as usize] =
            (*RI.currententity).curstate.angles[2 as libc::c_int as usize];
        deadplayer.origin[0 as libc::c_int as usize] =
            (*RI.currententity).curstate.origin[0 as libc::c_int as usize];
        deadplayer.origin[1 as libc::c_int as usize] =
            (*RI.currententity).curstate.origin[1 as libc::c_int as usize];
        deadplayer.origin[2 as libc::c_int as usize] =
            (*RI.currententity).curstate.origin[2 as libc::c_int as usize];
        g_studio.interpolate = false_0;
        result = R_StudioDrawPlayer(flags, &mut deadplayer);
        g_studio.interpolate = true_0;
        return result
    }
    R_StudioSetHeader(gEngfuncs.Mod_Extradata.expect("non-null function pointer")(mod_studio
                                                                                      as
                                                                                      libc::c_int,
                                                                                  RI.currentmodel)
                          as *mut studiohdr_t);
    R_StudioSetUpTransform(RI.currententity);
    if flags & 1 as libc::c_int != 0 {
        // see if the bounding box lets us trivially reject, also sets
        if R_StudioCheckBBox() == 0 {
            return 0 as libc::c_int
        } // render data cache cookie
        r_stats.c_studio_models_drawn =
            r_stats.c_studio_models_drawn.wrapping_add(1);
        g_studio.framecount += 1;
        if (*m_pStudioHeader).numbodyparts == 0 as libc::c_int {
            return 1 as libc::c_int
        }
    }
    if (*RI.currententity).curstate.movetype == 12 as libc::c_int {
        R_StudioMergeBones(RI.currententity, RI.currentmodel);
    } else { R_StudioSetupBones(RI.currententity); }
    R_StudioSaveBones();
    if flags & 2 as libc::c_int != 0 {
        R_StudioCalcAttachments();
        R_StudioClientEvents();
        // copy attachments into global entity array
        if (*RI.currententity).index > 0 as libc::c_int {
            let mut ent: *mut cl_entity_t =
                gEngfuncs.GetEntityByIndex.expect("non-null function pointer")((*RI.currententity).index);
            memcpy((*ent).attachment.as_mut_ptr() as *mut libc::c_void,
                   (*RI.currententity).attachment.as_mut_ptr() as
                       *const libc::c_void,
                   (::std::mem::size_of::<vec3_t>() as
                        libc::c_ulong).wrapping_mul(4 as libc::c_int as
                                                        libc::c_ulong));
        }
    }
    if flags & 1 as libc::c_int != 0 {
        lighting.plightvec = dir.as_mut_ptr();
        R_StudioDynamicLight(RI.currententity, &mut lighting);
        R_StudioEntityLight(&mut lighting);
        // model and frame independant
        R_StudioSetupLighting(&mut lighting);
        // get remap colors
        g_nTopColor =
            (*RI.currententity).curstate.colormap & 0xff as libc::c_int;
        g_nBottomColor =
            ((*RI.currententity).curstate.colormap & 0xff00 as libc::c_int) >>
                8 as libc::c_int;
        R_StudioSetRemapColors(g_nTopColor, g_nBottomColor);
        R_StudioRenderModel();
    }
    return 1 as libc::c_int;
}
/*
=================
R_StudioDrawModelInternal
=================
*/
#[no_mangle]
pub unsafe extern "C" fn R_StudioDrawModelInternal(mut e: *mut cl_entity_t,
                                                   mut flags: libc::c_int) {
    if RI.drawWorld as u64 == 0 {
        if (*e).player as u64 != 0 {
            R_StudioDrawPlayer(flags, &mut (*e).curstate);
        } else { R_StudioDrawModel(flags); }
    } else if (*e).player as u64 != 0 {
        (*pStudioDraw).StudioDrawPlayer.expect("non-null function pointer")(flags,
                                                                            R_StudioGetPlayerState((*e).index
                                                                                                       -
                                                                                                       1
                                                                                                           as
                                                                                                           libc::c_int));
    } else {
        (*pStudioDraw).StudioDrawModel.expect("non-null function pointer")(flags);
    };
}
// select the properly method
/*
=================
R_DrawStudioModel
=================
*/
#[no_mangle]
pub unsafe extern "C" fn R_DrawStudioModel(mut e: *mut cl_entity_t) {
    if RI.params as libc::c_uint & (1 as libc::c_uint) << 0 as libc::c_int !=
           0 {
        return
    }
    R_StudioSetupTimings();
    if (*e).player as u64 != 0 {
        R_StudioDrawModelInternal(e, 1 as libc::c_int | 2 as libc::c_int);
    } else {
        if (*e).curstate.movetype == 12 as libc::c_int &&
               (*e).curstate.aiment > 0 as libc::c_int {
            let mut parent: *mut cl_entity_t =
                gEngfuncs.GetEntityByIndex.expect("non-null function pointer")((*e).curstate.aiment);
            if !parent.is_null() && !(*parent).model.is_null() &&
                   (*(*parent).model).type_0 as libc::c_int ==
                       mod_studio as libc::c_int {
                RI.currententity = parent;
                R_StudioDrawModelInternal(RI.currententity, 0 as libc::c_int);
                (*e).curstate.origin[0 as libc::c_int as usize] =
                    (*parent).curstate.origin[0 as libc::c_int as usize];
                (*e).curstate.origin[1 as libc::c_int as usize] =
                    (*parent).curstate.origin[1 as libc::c_int as usize];
                (*e).curstate.origin[2 as libc::c_int as usize] =
                    (*parent).curstate.origin[2 as libc::c_int as usize];
                (*e).origin[0 as libc::c_int as usize] =
                    (*parent).origin[0 as libc::c_int as usize];
                (*e).origin[1 as libc::c_int as usize] =
                    (*parent).origin[1 as libc::c_int as usize];
                (*e).origin[2 as libc::c_int as usize] =
                    (*parent).origin[2 as libc::c_int as usize];
                RI.currententity = e
            }
        }
        R_StudioDrawModelInternal(e, 1 as libc::c_int | 2 as libc::c_int);
    };
}
/*
=================
R_RunViewmodelEvents
=================
*/
#[no_mangle]
pub unsafe extern "C" fn R_RunViewmodelEvents() {
    let mut i: libc::c_int = 0;
    let mut simorg: vec3_t = [0.; 3];
    if (*r_drawviewmodel).value == 0 as libc::c_int as libc::c_float {
        return
    }
    if Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(PARM_THIRDPERSON
                                                                                                                 as
                                                                                                                 libc::c_int,
                                                                                                             0
                                                                                                                 as
                                                                                                                 libc::c_int)
           != 0 {
        return
    }
    // ignore in thirdperson, camera view or client is died
    if !(RI.params as libc::c_uint & (1 as libc::c_uint) << 0 as libc::c_int
             == 0 as libc::c_int as libc::c_uint) ||
           Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(PARM_LOCAL_HEALTH
                                                                                                                     as
                                                                                                                     libc::c_int,
                                                                                                                 0
                                                                                                                     as
                                                                                                                     libc::c_int)
               <= 0 as libc::c_int ||
           !(Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(PARM_VIEWENT_INDEX
                                                                                                                       as
                                                                                                                       libc::c_int,
                                                                                                                   0
                                                                                                                       as
                                                                                                                       libc::c_int)
                 ==
                 Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(PARM_PLAYER_INDEX
                                                                                                                           as
                                                                                                                           libc::c_int,
                                                                                                                       0
                                                                                                                           as
                                                                                                                           libc::c_int))
       {
        return
    }
    RI.currententity =
        gEngfuncs.GetViewModel.expect("non-null function pointer")();
    if (*RI.currententity).model.is_null() ||
           (*(*RI.currententity).model).type_0 as libc::c_int !=
               mod_studio as libc::c_int {
        return
    }
    R_StudioSetupTimings();
    gEngfuncs.GetPredictedOrigin.expect("non-null function pointer")(simorg.as_mut_ptr());
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        (*RI.currententity).attachment[i as usize][0 as libc::c_int as usize]
            = simorg[0 as libc::c_int as usize];
        (*RI.currententity).attachment[i as usize][1 as libc::c_int as usize]
            = simorg[1 as libc::c_int as usize];
        (*RI.currententity).attachment[i as usize][2 as libc::c_int as usize]
            = simorg[2 as libc::c_int as usize];
        i += 1
    }
    RI.currentmodel = (*RI.currententity).model;
    R_StudioDrawModelInternal(RI.currententity, 2 as libc::c_int);
}
/*
=================
R_GatherPlayerLight
=================
*/
#[no_mangle]
pub unsafe extern "C" fn R_GatherPlayerLight() {
    let mut view: *mut cl_entity_t =
        gEngfuncs.GetViewModel.expect("non-null function pointer")();
    let mut c: colorVec = colorVec{r: 0, g: 0, b: 0, a: 0,};
    tr.ignore_lightgamma = true_0;
    c = R_LightPoint((*view).origin.as_mut_ptr() as *const vec_t);
    tr.ignore_lightgamma = false_0;
    gEngfuncs.SetLocalLightLevel.expect("non-null function pointer")(c.r.wrapping_add(c.g).wrapping_add(c.b).wrapping_div(3
                                                                                                                              as
                                                                                                                              libc::c_int
                                                                                                                              as
                                                                                                                              libc::c_uint)
                                                                         as
                                                                         libc::c_int);
}
/*
=================
R_DrawViewModel
=================
*/
#[no_mangle]
pub unsafe extern "C" fn R_DrawViewModel() {
    let mut view: *mut cl_entity_t =
        gEngfuncs.GetViewModel.expect("non-null function pointer")();
    R_GatherPlayerLight();
    if (*r_drawviewmodel).value == 0 as libc::c_int as libc::c_float {
        return
    }
    if Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(PARM_THIRDPERSON
                                                                                                                 as
                                                                                                                 libc::c_int,
                                                                                                             0
                                                                                                                 as
                                                                                                                 libc::c_int)
           != 0 {
        return
    }
    // ignore in thirdperson, camera view or client is died
    if !(RI.params as libc::c_uint & (1 as libc::c_uint) << 0 as libc::c_int
             == 0 as libc::c_int as libc::c_uint) ||
           Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(PARM_LOCAL_HEALTH
                                                                                                                     as
                                                                                                                     libc::c_int,
                                                                                                                 0
                                                                                                                     as
                                                                                                                     libc::c_int)
               <= 0 as libc::c_int ||
           !(Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(PARM_VIEWENT_INDEX
                                                                                                                       as
                                                                                                                       libc::c_int,
                                                                                                                   0
                                                                                                                       as
                                                                                                                       libc::c_int)
                 ==
                 Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(PARM_PLAYER_INDEX
                                                                                                                           as
                                                                                                                           libc::c_int,
                                                                                                                       0
                                                                                                                           as
                                                                                                                           libc::c_int))
       {
        return
    } // invisible ?
    tr.blend = CL_FxBlend(view) as libc::c_float / 255.0f32;
    if !((*view).curstate.rendermode == kRenderNormal as libc::c_int) &&
           tr.blend <= 0.0f32 {
        return
    }
    RI.currententity = view;
    if (*RI.currententity).model.is_null() { return }
    // adjust the depth range to prevent view model from poking into walls
    pglDepthRange.expect("non-null function pointer")(gldepthmin as GLclampd,
                                                      (gldepthmin +
                                                           0.3f32 *
                                                               (gldepthmax -
                                                                    gldepthmin))
                                                          as GLclampd);
    RI.currentmodel = (*RI.currententity).model;
    // backface culling for left-handed weapons
    if R_AllowFlipViewModel(RI.currententity) as libc::c_uint != 0 ||
           g_iBackFaceCull != 0 {
        tr.fFlipViewModel = true_0;
        pglFrontFace.expect("non-null function pointer")(0x900 as libc::c_int
                                                             as GLenum);
    }
    match (*(*RI.currententity).model).type_0 as libc::c_int {
        2 => { R_DrawAliasModel(RI.currententity); }
        3 => {
            R_StudioSetupTimings();
            R_StudioDrawModelInternal(RI.currententity, 1 as libc::c_int);
        }
        _ => { }
    }
    // restore depth range
    pglDepthRange.expect("non-null function pointer")(gldepthmin as GLclampd,
                                                      gldepthmax as GLclampd);
    // backface culling for left-handed weapons
    if R_AllowFlipViewModel(RI.currententity) as libc::c_uint != 0 ||
           g_iBackFaceCull != 0 {
        tr.fFlipViewModel = false_0;
        pglFrontFace.expect("non-null function pointer")(0x901 as libc::c_int
                                                             as GLenum);
    };
}
/*
====================
R_StudioLoadTexture

load model texture with unique name
====================
*/
unsafe extern "C" fn R_StudioLoadTexture(mut mod_0: *mut model_t,
                                         mut phdr: *mut studiohdr_t,
                                         mut ptexture:
                                             *mut mstudiotexture_t) {
    let mut size: size_t = 0;
    let mut flags: libc::c_int = 0 as libc::c_int;
    let mut texname: [libc::c_char; 128] = [0; 128];
    let mut name: [libc::c_char; 128] = [0; 128];
    let mut mdlname: [libc::c_char; 128] = [0; 128];
    let mut tx: *mut texture_t = 0 as *mut texture_t;
    if (*ptexture).flags & 0x80 as libc::c_int as libc::c_uint != 0 {
        flags |= TF_NORMALMAP as libc::c_int
    }
    // store some textures for remapping
    if Q_strnicmp((*ptexture).name.as_mut_ptr(),
                  b"DM_Base\x00" as *const u8 as *const libc::c_char,
                  7 as libc::c_int) == 0 ||
           Q_strnicmp((*ptexture).name.as_mut_ptr(),
                      b"remap\x00" as *const u8 as *const libc::c_char,
                      5 as libc::c_int) == 0 {
        let mut i: libc::c_int = 0;
        let mut size_0: libc::c_int = 0;
        let mut val: [libc::c_char; 6] = [0; 6];
        let mut pixels: *mut byte = 0 as *mut byte;
        i = (*mod_0).numtextures;
        (*mod_0).textures =
            gEngfuncs._Mem_Realloc.expect("non-null function pointer")((*mod_0).mempool,
                                                                       (*mod_0).textures
                                                                           as
                                                                           *mut libc::c_void,
                                                                       ((i +
                                                                             1
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut texture_t>()
                                                                                                            as
                                                                                                            libc::c_ulong),
                                                                       true_0,
                                                                       b"../ref_gl/gl_studio.c\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const libc::c_char,
                                                                       3773 as
                                                                           libc::c_int)
                as *mut *mut texture_t;
        size_0 = (*ptexture).width * (*ptexture).height + 768 as libc::c_int;
        tx =
            gEngfuncs._Mem_Alloc.expect("non-null function pointer")((*mod_0).mempool,
                                                                     (::std::mem::size_of::<texture_t>()
                                                                          as
                                                                          libc::c_ulong).wrapping_add(size_0
                                                                                                          as
                                                                                                          libc::c_ulong),
                                                                     true_0,
                                                                     b"../ref_gl/gl_studio.c\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     3775 as
                                                                         libc::c_int)
                as *mut texture_t;
        let ref mut fresh24 = *(*mod_0).textures.offset(i as isize);
        *fresh24 = tx;
        // done
        if Q_strnicmp((*ptexture).name.as_mut_ptr(),
                      b"DM_Base\x00" as *const u8 as *const libc::c_char,
                      7 as libc::c_int) == 0 {
            Q_strncpy((*tx).name.as_mut_ptr(),
                      b"DM_Base\x00" as *const u8 as *const libc::c_char,
                      ::std::mem::size_of::<[libc::c_char; 16]>() as
                          libc::c_ulong);
            // store ranges into anim_min, anim_max etc
            // bottomcolor end
            (*tx).anim_min = 160 as libc::c_int; // topcolor start
            (*tx).anim_max = 191 as libc::c_int; // topcolor end
            (*tx).anim_total = 223 as libc::c_int
        } else {
            // bottomcolor start always equal is (topcolor end + 1)
            Q_strncpy((*tx).name.as_mut_ptr(),
                      b"DM_User\x00" as *const u8 as *const libc::c_char,
                      ::std::mem::size_of::<[libc::c_char; 16]>() as
                          libc::c_ulong);
            Q_strncpy(val.as_mut_ptr(),
                      (*ptexture).name.as_mut_ptr().offset(7 as libc::c_int as
                                                               isize),
                      4 as libc::c_int as size_t); // custom remapped
            // bottomcolor end
            (*tx).anim_min =
                if Q_atoi(val.as_mut_ptr()) >= 0 as libc::c_int {
                    if Q_atoi(val.as_mut_ptr()) < 255 as libc::c_int {
                        Q_atoi(val.as_mut_ptr())
                    } else { 255 as libc::c_int }
                } else { 0 as libc::c_int }; // topcolor start
            Q_strncpy(val.as_mut_ptr(),
                      (*ptexture).name.as_mut_ptr().offset(11 as libc::c_int
                                                               as isize),
                      4 as libc::c_int as size_t); // topcolor end
            (*tx).anim_max =
                if Q_atoi(val.as_mut_ptr()) >= 0 as libc::c_int {
                    if Q_atoi(val.as_mut_ptr()) < 255 as libc::c_int {
                        Q_atoi(val.as_mut_ptr())
                    } else { 255 as libc::c_int }
                } else { 0 as libc::c_int };
            Q_strncpy(val.as_mut_ptr(),
                      (*ptexture).name.as_mut_ptr().offset(15 as libc::c_int
                                                               as isize),
                      4 as libc::c_int as size_t);
            (*tx).anim_total =
                if Q_atoi(val.as_mut_ptr()) >= 0 as libc::c_int {
                    if Q_atoi(val.as_mut_ptr()) < 255 as libc::c_int {
                        Q_atoi(val.as_mut_ptr())
                    } else { 255 as libc::c_int }
                } else { 0 as libc::c_int }
        }
        (*tx).width = (*ptexture).width as libc::c_uint;
        (*tx).height = (*ptexture).height as libc::c_uint;
        pixels = (phdr as *mut byte).offset((*ptexture).index as isize);
        memcpy(tx.offset(1 as libc::c_int as isize) as *mut libc::c_void,
               pixels as *const libc::c_void, size_0 as libc::c_ulong);
        (*ptexture).flags |= (1 as libc::c_uint) << 30 as libc::c_int;
        flags |= TF_FORCE_COLOR as libc::c_int;
        (*mod_0).numtextures += 1
    }
    Q_strncpy(mdlname.as_mut_ptr(), (*mod_0).name.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong);
    COM_FileBase((*ptexture).name.as_mut_ptr(), name.as_mut_ptr());
    COM_StripExtension(mdlname.as_mut_ptr());
    if (*ptexture).flags & 0x8 as libc::c_int as libc::c_uint != 0 {
        flags = flags | TF_NOMIPMAP as libc::c_int
    }
    // bottomcolor start always equal is (topcolor end + 1)
    // the pixels immediately follow the structures
    // yes, this is colormap image
    // NOTE: replace index with pointer to start of imagebuffer, ImageLib expected it
	//ptexture->index = (int)((byte *)phdr) + ptexture->index;
    gEngfuncs.Image_SetMDLPointer.expect("non-null function pointer")((phdr as
                                                                           *mut byte).offset((*ptexture).index
                                                                                                 as
                                                                                                 isize)); // Paranoia2 texture alpha-tracing
    size =
        (::std::mem::size_of::<mstudiotexture_t>() as
             libc::c_ulong).wrapping_add(((*ptexture).width *
                                              (*ptexture).height) as
                                             libc::c_ulong).wrapping_add(768
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_ulong);
    if Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(26
                                                                                                                 as
                                                                                                                 libc::c_int,
                                                                                                             0
                                                                                                                 as
                                                                                                                 libc::c_int)
           & (1 as libc::c_int) << 6 as libc::c_int != 0 &&
           (*ptexture).flags & 0x40 as libc::c_int as libc::c_uint != 0 {
        flags |= TF_KEEP_SOURCE as libc::c_int
    }
    // build the texname
    Q_snprintf(texname.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
               b"#%s/%s.mdl\x00" as *const u8 as *const libc::c_char,
               mdlname.as_mut_ptr(), name.as_mut_ptr());
    (*ptexture).index =
        GL_LoadTexture(texname.as_mut_ptr(), ptexture as *mut byte, size,
                       flags);
    if (*ptexture).index == 0 {
        (*ptexture).index = tr.defaultTexture
    } else if !tx.is_null() {
        // duplicate texnum for easy acess
        (*tx).gl_texturenum = (*ptexture).index
    };
}
/*
=================
Mod_StudioLoadTextures
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Mod_StudioLoadTextures(mut mod_0: *mut model_t,
                                                mut data: *mut libc::c_void) {
    let mut phdr: *mut studiohdr_t = data as *mut studiohdr_t;
    let mut ptexture: *mut mstudiotexture_t = 0 as *mut mstudiotexture_t;
    let mut i: libc::c_int = 0;
    if phdr.is_null() { return }
    ptexture =
        (phdr as *mut byte).offset((*phdr).textureindex as isize) as
            *mut mstudiotexture_t;
    if (*phdr).textureindex > 0 as libc::c_int &&
           (*phdr).numtextures <= 256 as libc::c_int {
        i = 0 as libc::c_int;
        while i < (*phdr).numtextures {
            R_StudioLoadTexture(mod_0, phdr,
                                &mut *ptexture.offset(i as isize));
            i += 1
        }
    };
}
/*
=================
Mod_StudioUnloadTextures
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Mod_StudioUnloadTextures(mut data:
                                                      *mut libc::c_void) {
    let mut phdr: *mut studiohdr_t = data as *mut studiohdr_t;
    let mut ptexture: *mut mstudiotexture_t = 0 as *mut mstudiotexture_t;
    let mut i: libc::c_int = 0;
    if phdr.is_null() { return }
    ptexture =
        (phdr as *mut byte).offset((*phdr).textureindex as isize) as
            *mut mstudiotexture_t;
    // release all textures
    i = 0 as libc::c_int;
    while i < (*phdr).numtextures {
        if !((*ptexture.offset(i as isize)).index == tr.defaultTexture) {
            GL_FreeTexture((*ptexture.offset(i as isize)).index as GLenum);
        }
        i += 1
    };
}
unsafe extern "C" fn pfnModelHandle(mut modelindex: libc::c_int)
 -> *mut model_t {
    return gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(modelindex);
}
unsafe extern "C" fn pfnMod_CacheCheck(mut c: *mut cache_user_s)
 -> *mut libc::c_void {
    return gEngfuncs.Mod_CacheCheck.expect("non-null function pointer")(c);
}
unsafe extern "C" fn pfnMod_StudioExtradata(mut mod_0: *mut model_t)
 -> *mut libc::c_void {
    return gEngfuncs.Mod_Extradata.expect("non-null function pointer")(mod_studio
                                                                           as
                                                                           libc::c_int,
                                                                       mod_0);
}
unsafe extern "C" fn pfnMod_LoadCacheFile(mut path: *const libc::c_char,
                                          mut cu: *mut cache_user_s) {
    gEngfuncs.Mod_LoadCacheFile.expect("non-null function pointer")(path, cu);
}
unsafe extern "C" fn pfnGetCvarPointer(mut name: *const libc::c_char)
 -> *mut cvar_t {
    return gEngfuncs.pfnGetCvarPointer.expect("non-null function pointer")(name,
                                                                           0
                                                                               as
                                                                               libc::c_int);
}
unsafe extern "C" fn pfnMod_Calloc(mut number: libc::c_int, mut size: size_t)
 -> *mut libc::c_void {
    return gEngfuncs.Mod_Calloc.expect("non-null function pointer")(number,
                                                                    size);
}
static mut gStudioAPI: engine_studio_api_t =
    unsafe {
        {
            let mut init =
                engine_studio_api_s{Mem_Calloc:
                                        Some(pfnMod_Calloc as
                                                 unsafe extern "C" fn(_:
                                                                          libc::c_int,
                                                                      _:
                                                                          size_t)
                                                     -> *mut libc::c_void),
                                    Cache_Check:
                                        Some(pfnMod_CacheCheck as
                                                 unsafe extern "C" fn(_:
                                                                          *mut cache_user_s)
                                                     -> *mut libc::c_void),
                                    LoadCacheFile:
                                        Some(pfnMod_LoadCacheFile as
                                                 unsafe extern "C" fn(_:
                                                                          *const libc::c_char,
                                                                      _:
                                                                          *mut cache_user_s)
                                                     -> ()),
                                    Mod_ForName:
                                        Some(pfnMod_ForName as
                                                 unsafe extern "C" fn(_:
                                                                          *const libc::c_char,
                                                                      _:
                                                                          libc::c_int)
                                                     -> *mut model_t),
                                    Mod_Extradata:
                                        Some(pfnMod_StudioExtradata as
                                                 unsafe extern "C" fn(_:
                                                                          *mut model_t)
                                                     -> *mut libc::c_void),
                                    GetModelByIndex:
                                        Some(pfnModelHandle as
                                                 unsafe extern "C" fn(_:
                                                                          libc::c_int)
                                                     -> *mut model_t),
                                    GetCurrentEntity:
                                        Some(pfnGetCurrentEntity as
                                                 unsafe extern "C" fn()
                                                     -> *mut cl_entity_t),
                                    PlayerInfo:
                                        Some(pfnPlayerInfo as
                                                 unsafe extern "C" fn(_:
                                                                          libc::c_int)
                                                     -> *mut player_info_t),
                                    GetPlayerState:
                                        Some(R_StudioGetPlayerState as
                                                 unsafe extern "C" fn(_:
                                                                          libc::c_int)
                                                     -> *mut entity_state_t),
                                    GetViewEntity:
                                        Some(pfnGetViewEntity as
                                                 unsafe extern "C" fn()
                                                     -> *mut cl_entity_t),
                                    GetTimes:
                                        Some(pfnGetEngineTimes as
                                                 unsafe extern "C" fn(_:
                                                                          *mut libc::c_int,
                                                                      _:
                                                                          *mut libc::c_double,
                                                                      _:
                                                                          *mut libc::c_double)
                                                     -> ()),
                                    GetCvar:
                                        Some(pfnGetCvarPointer as
                                                 unsafe extern "C" fn(_:
                                                                          *const libc::c_char)
                                                     -> *mut cvar_t),
                                    GetViewInfo:
                                        Some(pfnGetViewInfo as
                                                 unsafe extern "C" fn(_:
                                                                          *mut libc::c_float,
                                                                      _:
                                                                          *mut libc::c_float,
                                                                      _:
                                                                          *mut libc::c_float,
                                                                      _:
                                                                          *mut libc::c_float)
                                                     -> ()),
                                    GetChromeSprite:
                                        Some(R_GetChromeSprite as
                                                 unsafe extern "C" fn()
                                                     -> *mut model_t),
                                    GetModelCounters:
                                        Some(pfnGetModelCounters as
                                                 unsafe extern "C" fn(_:
                                                                          *mut *mut libc::c_int,
                                                                      _:
                                                                          *mut *mut libc::c_int)
                                                     -> ()),
                                    GetAliasScale:
                                        Some(pfnGetAliasScale as
                                                 unsafe extern "C" fn(_:
                                                                          *mut libc::c_float,
                                                                      _:
                                                                          *mut libc::c_float)
                                                     -> ()),
                                    StudioGetBoneTransform:
                                        Some(pfnStudioGetBoneTransform as
                                                 unsafe extern "C" fn()
                                                     ->
                                                         *mut *mut *mut *mut libc::c_float),
                                    StudioGetLightTransform:
                                        Some(pfnStudioGetLightTransform as
                                                 unsafe extern "C" fn()
                                                     ->
                                                         *mut *mut *mut *mut libc::c_float),
                                    StudioGetAliasTransform:
                                        Some(pfnStudioGetAliasTransform as
                                                 unsafe extern "C" fn()
                                                     ->
                                                         *mut *mut *mut libc::c_float),
                                    StudioGetRotationMatrix:
                                        Some(pfnStudioGetRotationMatrix as
                                                 unsafe extern "C" fn()
                                                     ->
                                                         *mut *mut *mut libc::c_float),
                                    StudioSetupModel:
                                        Some(R_StudioSetupModel as
                                                 unsafe extern "C" fn(_:
                                                                          libc::c_int,
                                                                      _:
                                                                          *mut *mut libc::c_void,
                                                                      _:
                                                                          *mut *mut libc::c_void)
                                                     -> ()),
                                    StudioCheckBBox:
                                        Some(R_StudioCheckBBox as
                                                 unsafe extern "C" fn()
                                                     -> libc::c_int),
                                    StudioDynamicLight:
                                        Some(R_StudioDynamicLight as
                                                 unsafe extern "C" fn(_:
                                                                          *mut cl_entity_t,
                                                                      _:
                                                                          *mut alight_t)
                                                     -> ()),
                                    StudioEntityLight:
                                        Some(R_StudioEntityLight as
                                                 unsafe extern "C" fn(_:
                                                                          *mut alight_t)
                                                     -> ()),
                                    StudioSetupLighting:
                                        Some(R_StudioSetupLighting as
                                                 unsafe extern "C" fn(_:
                                                                          *mut alight_t)
                                                     -> ()),
                                    StudioDrawPoints:
                                        Some(R_StudioDrawPoints as
                                                 unsafe extern "C" fn()
                                                     -> ()),
                                    StudioDrawHulls:
                                        Some(R_StudioDrawHulls as
                                                 unsafe extern "C" fn()
                                                     -> ()),
                                    StudioDrawAbsBBox:
                                        Some(R_StudioDrawAbsBBox as
                                                 unsafe extern "C" fn()
                                                     -> ()),
                                    StudioDrawBones:
                                        Some(R_StudioDrawBones as
                                                 unsafe extern "C" fn()
                                                     -> ()),
                                    StudioSetupSkin:
                                        ::std::mem::transmute::<*mut libc::c_void,
                                                                Option<unsafe extern "C" fn(_:
                                                                                                *mut libc::c_void,
                                                                                            _:
                                                                                                libc::c_int)
                                                                           ->
                                                                               ()>>(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                                                            *mut studiohdr_t,
                                                                                                                                        _:
                                                                                                                                            libc::c_int)
                                                                                                                       ->
                                                                                                                           ()>,
                                                                                                            *mut libc::c_void>(Some(R_StudioSetupSkin
                                                                                                                                        as
                                                                                                                                        unsafe extern "C" fn(_:
                                                                                                                                                                 *mut studiohdr_t,
                                                                                                                                                             _:
                                                                                                                                                                 libc::c_int)
                                                                                                                                            ->
                                                                                                                                                ()))),
                                    StudioSetRemapColors:
                                        Some(R_StudioSetRemapColors as
                                                 unsafe extern "C" fn(_:
                                                                          libc::c_int,
                                                                      _:
                                                                          libc::c_int)
                                                     -> ()),
                                    SetupPlayerModel:
                                        Some(R_StudioSetupPlayerModel as
                                                 unsafe extern "C" fn(_:
                                                                          libc::c_int)
                                                     -> *mut model_t),
                                    StudioClientEvents:
                                        Some(R_StudioClientEvents as
                                                 unsafe extern "C" fn()
                                                     -> ()),
                                    GetForceFaceFlags:
                                        Some(R_StudioGetForceFaceFlags as
                                                 unsafe extern "C" fn()
                                                     -> libc::c_int),
                                    SetForceFaceFlags:
                                        Some(R_StudioSetForceFaceFlags as
                                                 unsafe extern "C" fn(_:
                                                                          libc::c_int)
                                                     -> ()),
                                    StudioSetHeader:
                                        ::std::mem::transmute::<*mut libc::c_void,
                                                                Option<unsafe extern "C" fn(_:
                                                                                                *mut libc::c_void)
                                                                           ->
                                                                               ()>>(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                                                            *mut studiohdr_t)
                                                                                                                       ->
                                                                                                                           ()>,
                                                                                                            *mut libc::c_void>(Some(R_StudioSetHeader
                                                                                                                                        as
                                                                                                                                        unsafe extern "C" fn(_:
                                                                                                                                                                 *mut studiohdr_t)
                                                                                                                                            ->
                                                                                                                                                ()))),
                                    SetRenderModel:
                                        Some(R_StudioSetRenderModel as
                                                 unsafe extern "C" fn(_:
                                                                          *mut model_t)
                                                     -> ()),
                                    SetupRenderer:
                                        Some(R_StudioSetupRenderer as
                                                 unsafe extern "C" fn(_:
                                                                          libc::c_int)
                                                     -> ()),
                                    RestoreRenderer:
                                        Some(R_StudioRestoreRenderer as
                                                 unsafe extern "C" fn()
                                                     -> ()),
                                    SetChromeOrigin:
                                        Some(R_StudioSetChromeOrigin as
                                                 unsafe extern "C" fn()
                                                     -> ()),
                                    IsHardware:
                                        Some(pfnIsHardware as
                                                 unsafe extern "C" fn()
                                                     -> libc::c_int),
                                    GL_StudioDrawShadow:
                                        Some(GL_StudioDrawShadow as
                                                 unsafe extern "C" fn()
                                                     -> ()),
                                    GL_SetRenderMode:
                                        Some(GL_StudioSetRenderMode as
                                                 unsafe extern "C" fn(_:
                                                                          libc::c_int)
                                                     -> ()),
                                    StudioSetRenderamt:
                                        Some(R_StudioSetRenderamt as
                                                 unsafe extern "C" fn(_:
                                                                          libc::c_int)
                                                     -> ()),
                                    StudioSetCullState:
                                        Some(R_StudioSetCullState as
                                                 unsafe extern "C" fn(_:
                                                                          libc::c_int)
                                                     -> ()),
                                    StudioRenderShadow:
                                        Some(R_StudioRenderShadow as
                                                 unsafe extern "C" fn(_:
                                                                          libc::c_int,
                                                                      _:
                                                                          *mut libc::c_float,
                                                                      _:
                                                                          *mut libc::c_float,
                                                                      _:
                                                                          *mut libc::c_float,
                                                                      _:
                                                                          *mut libc::c_float)
                                                     -> ()),};
            init
        }
    };
static mut gStudioDraw: r_studio_interface_t =
    unsafe {
        {
            let mut init =
                r_studio_interface_s{version: 1 as libc::c_int,
                                     StudioDrawModel:
                                         Some(R_StudioDrawModel as
                                                  unsafe extern "C" fn(_:
                                                                           libc::c_int)
                                                      -> libc::c_int),
                                     StudioDrawPlayer:
                                         Some(R_StudioDrawPlayer as
                                                  unsafe extern "C" fn(_:
                                                                           libc::c_int,
                                                                       _:
                                                                           *mut entity_state_t)
                                                      -> libc::c_int),};
            init
        }
    };
/*
===============
CL_InitStudioAPI

Initialize client studio
===============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_InitStudioAPI() {
    pStudioDraw = &mut gStudioDraw;
    // trying to grab them from client.dll
    cl_righthand =
        gEngfuncs.pfnGetCvarPointer.expect("non-null function pointer")(b"cl_righthand\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char,
                                                                        0 as
                                                                            libc::c_int);
    if cl_righthand.is_null() {
        cl_righthand =
            gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"cl_righthand\x00"
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
                                                                       0 as
                                                                           libc::c_int,
                                                                   b"flip viewmodel (left to right)\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char)
    }
    // Xash will be used internal StudioModelRenderer
    if gEngfuncs.pfnGetStudioModelInterface.expect("non-null function pointer")(1
                                                                                    as
                                                                                    libc::c_int,
                                                                                &mut pStudioDraw,
                                                                                &mut gStudioAPI)
           != 0 {
        return
    }
    // NOTE: we always return true even if game interface was not correct
	// because we need Draw our StudioModels
	// just restore pointer to builtin function
    pStudioDraw = &mut gStudioDraw;
}
