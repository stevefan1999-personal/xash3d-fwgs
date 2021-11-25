#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type grasshdr_s;
    pub type con_nprint_s;
    pub type engine_studio_api_s;
    pub type r_studio_interface_s;
    pub type client_textmessage_s;
    pub type screenfade_s;
    pub type world_static_s;
    #[no_mangle]
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn cosf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn sinf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn sqrtf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn fmodf(_: libc::c_float, _: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn rsqrt(number: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn SinCos(radians: libc::c_float, sine: *mut libc::c_float,
              cosine: *mut libc::c_float);
    #[no_mangle]
    static mut pglDepthMask: Option<unsafe extern "C" fn(_: GLboolean) -> ()>;
    #[no_mangle]
    static mut pglNormal3fv:
           Option<unsafe extern "C" fn(_: *const GLfloat) -> ()>;
    #[no_mangle]
    static mut pglShadeModel: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglTexCoord2f:
           Option<unsafe extern "C" fn(_: GLfloat, _: GLfloat) -> ()>;
    #[no_mangle]
    static mut pglVertex3fv:
           Option<unsafe extern "C" fn(_: *const GLfloat) -> ()>;
    #[no_mangle]
    static mut r_stats: ref_speeds_t;
    #[no_mangle]
    static mut RI: ref_instance_t;
    #[no_mangle]
    static mut tr: gl_globals_t;
    #[no_mangle]
    fn GL_Cull(cull: GLenum);
    #[no_mangle]
    static mut gpGlobals: *mut ref_globals_t;
    #[no_mangle]
    fn TriEnd();
    #[no_mangle]
    static mut gEngfuncs: ref_api_t;
    #[no_mangle]
    fn TriVertex3fv(v: *const libc::c_float);
    #[no_mangle]
    fn TriTexCoord2f(u: libc::c_float, v: libc::c_float);
    #[no_mangle]
    fn TriWorldToScreen(world: *const libc::c_float,
                        screen: *mut libc::c_float) -> libc::c_int;
    #[no_mangle]
    fn R_CullBox(mins: *const vec_t, maxs: *const vec_t) -> qboolean;
    #[no_mangle]
    fn Mod_GetCurrentVis() -> *mut byte;
    #[no_mangle]
    fn TriBegin(mode: libc::c_int);
    #[no_mangle]
    fn TriBrightness(brightness: libc::c_float);
    #[no_mangle]
    fn TriColor4f(r: libc::c_float, g: libc::c_float, b: libc::c_float,
                  a: libc::c_float);
    #[no_mangle]
    fn CL_FxBlend(e: *mut cl_entity_t) -> libc::c_int;
    #[no_mangle]
    fn TriSpriteTexture(pSpriteModel: *mut model_t, frame: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn TriRenderMode(mode: libc::c_int);
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
pub type C2RustUnnamed_0 = libc::c_int;
pub const PARM_NUMMODELS: C2RustUnnamed_0 = -13;
pub const PARM_NUMENTITIES: C2RustUnnamed_0 = -12;
pub const PARM_LOCAL_GAME: C2RustUnnamed_0 = -11;
pub const PARM_LOCAL_HEALTH: C2RustUnnamed_0 = -10;
pub const PARM_MAX_CLIENTS: C2RustUnnamed_0 = -9;
pub const PARM_WATER_LEVEL: C2RustUnnamed_0 = -8;
pub const PARM_PLAYING_DEMO: C2RustUnnamed_0 = -7;
pub const PARM_CONNSTATE: C2RustUnnamed_0 = -6;
pub const PARM_VIEWENT_INDEX: C2RustUnnamed_0 = -5;
pub const PARM_PLAYER_INDEX: C2RustUnnamed_0 = -4;
pub const PARM_QUAKE_COMPATIBLE: C2RustUnnamed_0 = -3;
pub const PARM_THIRDPERSON: C2RustUnnamed_0 = -2;
pub const PARM_DEV_OVERVIEW: C2RustUnnamed_0 = -1;
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
pub type GLenum = uint;
pub type GLboolean = byte;
pub type GLfloat = libc::c_float;
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
pub struct beamseg_t {
    pub pos: vec3_t,
    pub texcoord: libc::c_float,
    pub width: libc::c_float,
}
pub const BEAM_POINTS: C2RustUnnamed_1 = 0;
pub const BEAM_HOSE: C2RustUnnamed_1 = 3;
pub const BEAM_ENTS: C2RustUnnamed_1 = 2;
pub const BEAM_ENTPOINT: C2RustUnnamed_1 = 1;
pub type C2RustUnnamed_1 = libc::c_uint;
#[inline(always)]
unsafe extern "C" fn __tg_cos(mut __x: libc::c_float) -> libc::c_float {
    return cosf(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_sin(mut __x: libc::c_float) -> libc::c_float {
    return sinf(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_pow(mut __x: libc::c_double,
                              mut __y: libc::c_double) -> libc::c_double {
    return pow(__x, __y);
}
#[inline(always)]
unsafe extern "C" fn __tg_sqrt(mut __x: libc::c_float) -> libc::c_float {
    return sqrtf(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_fmod(mut __x: libc::c_float, mut __y: libc::c_float)
 -> libc::c_float {
    return fmodf(__x, __y);
}
#[inline(always)]
unsafe extern "C" fn __tg_fmod_0(mut __x: libc::c_double,
                                 mut __y: libc::c_double) -> libc::c_double {
    return fmod(__x, __y);
}
/*
==============================================================

FRACTAL NOISE

==============================================================
*/
static mut rgNoise: [libc::c_float; 65] = [0.; 65];
// global noise array
// freq2 += step * 0.1;
// Fractal noise generator, power of 2 wavelength
unsafe extern "C" fn FracNoise(mut noise: *mut libc::c_float,
                               mut divs: libc::c_int) {
    let mut div2: libc::c_int = 0;
    div2 = divs >> 1 as libc::c_int;
    if divs < 2 as libc::c_int { return }
    // noise is normalized to +/- scale
    *noise.offset(div2 as isize) =
        (*noise.offset(0 as libc::c_int as isize) +
             *noise.offset(divs as isize)) * 0.5f32 +
            divs as libc::c_float *
                gEngfuncs.COM_RandomFloat.expect("non-null function pointer")(-0.125f32,
                                                                              0.125f32);
    if div2 > 1 as libc::c_int {
        FracNoise(&mut *noise.offset(div2 as isize), div2);
        FracNoise(noise, div2);
    };
}
unsafe extern "C" fn SineNoise(mut noise: *mut libc::c_float,
                               mut divs: libc::c_int) {
    let mut freq: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut step: libc::c_float =
        3.14159265358979323846f64 as libc::c_float / divs as libc::c_float;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < divs {
        *noise.offset(i as isize) = __tg_sin(freq);
        freq += step;
        i += 1
    };
}
/*
==============================================================

BEAM MATHLIB

==============================================================
*/
unsafe extern "C" fn R_BeamComputePerpendicular(mut vecBeamDelta:
                                                    *const vec_t,
                                                mut pPerp: *mut vec_t) {
    // direction in worldspace of the center of the beam
    let mut vecBeamCenter: vec3_t = [0.; 3];
    let mut ilength: libc::c_float =
        __tg_sqrt(*vecBeamDelta.offset(0 as libc::c_int as isize) *
                      *vecBeamDelta.offset(0 as libc::c_int as isize) +
                      *vecBeamDelta.offset(1 as libc::c_int as isize) *
                          *vecBeamDelta.offset(1 as libc::c_int as isize) +
                      *vecBeamDelta.offset(2 as libc::c_int as isize) *
                          *vecBeamDelta.offset(2 as libc::c_int as isize));
    if ilength != 0. { ilength = 1.0f32 / ilength }
    vecBeamCenter[0 as libc::c_int as usize] =
        *vecBeamDelta.offset(0 as libc::c_int as isize) * ilength;
    vecBeamCenter[1 as libc::c_int as usize] =
        *vecBeamDelta.offset(1 as libc::c_int as isize) * ilength;
    vecBeamCenter[2 as libc::c_int as usize] =
        *vecBeamDelta.offset(2 as libc::c_int as isize) * ilength;
    *pPerp.offset(0 as libc::c_int as isize) =
        RI.vforward[1 as libc::c_int as usize] *
            vecBeamCenter[2 as libc::c_int as usize] -
            RI.vforward[2 as libc::c_int as usize] *
                vecBeamCenter[1 as libc::c_int as usize];
    *pPerp.offset(1 as libc::c_int as isize) =
        RI.vforward[2 as libc::c_int as usize] *
            vecBeamCenter[0 as libc::c_int as usize] -
            RI.vforward[0 as libc::c_int as usize] *
                vecBeamCenter[2 as libc::c_int as usize];
    *pPerp.offset(2 as libc::c_int as isize) =
        RI.vforward[0 as libc::c_int as usize] *
            vecBeamCenter[1 as libc::c_int as usize] -
            RI.vforward[1 as libc::c_int as usize] *
                vecBeamCenter[0 as libc::c_int as usize];
    let mut ilength_0: libc::c_float =
        __tg_sqrt(*pPerp.offset(0 as libc::c_int as isize) *
                      *pPerp.offset(0 as libc::c_int as isize) +
                      *pPerp.offset(1 as libc::c_int as isize) *
                          *pPerp.offset(1 as libc::c_int as isize) +
                      *pPerp.offset(2 as libc::c_int as isize) *
                          *pPerp.offset(2 as libc::c_int as isize));
    if ilength_0 != 0. { ilength_0 = 1.0f32 / ilength_0 }
    let ref mut fresh0 = *pPerp.offset(0 as libc::c_int as isize);
    *fresh0 *= ilength_0;
    let ref mut fresh1 = *pPerp.offset(1 as libc::c_int as isize);
    *fresh1 *= ilength_0;
    let ref mut fresh2 = *pPerp.offset(2 as libc::c_int as isize);
    *fresh2 *= ilength_0;
}
unsafe extern "C" fn R_BeamComputeNormal(mut vStartPos: *const vec_t,
                                         mut vNextPos: *const vec_t,
                                         mut pNormal: *mut vec_t) {
    // vTangentY = line vector for beam
    let mut vTangentY: vec3_t = [0.; 3];
    let mut vDirToBeam: vec3_t = [0.; 3];
    vTangentY[0 as libc::c_int as usize] =
        *vStartPos.offset(0 as libc::c_int as isize) -
            *vNextPos.offset(0 as libc::c_int as isize);
    vTangentY[1 as libc::c_int as usize] =
        *vStartPos.offset(1 as libc::c_int as isize) -
            *vNextPos.offset(1 as libc::c_int as isize);
    vTangentY[2 as libc::c_int as usize] =
        *vStartPos.offset(2 as libc::c_int as isize) -
            *vNextPos.offset(2 as libc::c_int as isize);
    // vDirToBeam = vector from viewer origin to beam
    vDirToBeam[0 as libc::c_int as usize] =
        *vStartPos.offset(0 as libc::c_int as isize) -
            RI.vieworg[0 as libc::c_int as usize];
    vDirToBeam[1 as libc::c_int as usize] =
        *vStartPos.offset(1 as libc::c_int as isize) -
            RI.vieworg[1 as libc::c_int as usize];
    vDirToBeam[2 as libc::c_int as usize] =
        *vStartPos.offset(2 as libc::c_int as isize) -
            RI.vieworg[2 as libc::c_int as usize];
    // get a vector that is perpendicular to us and perpendicular to the beam.
	// this is used to fatten the beam.
    *pNormal.offset(0 as libc::c_int as isize) =
        vTangentY[1 as libc::c_int as usize] *
            vDirToBeam[2 as libc::c_int as usize] -
            vTangentY[2 as libc::c_int as usize] *
                vDirToBeam[1 as libc::c_int as usize];
    *pNormal.offset(1 as libc::c_int as isize) =
        vTangentY[2 as libc::c_int as usize] *
            vDirToBeam[0 as libc::c_int as usize] -
            vTangentY[0 as libc::c_int as usize] *
                vDirToBeam[2 as libc::c_int as usize];
    *pNormal.offset(2 as libc::c_int as isize) =
        vTangentY[0 as libc::c_int as usize] *
            vDirToBeam[1 as libc::c_int as usize] -
            vTangentY[1 as libc::c_int as usize] *
                vDirToBeam[0 as libc::c_int as usize];
    let mut ilength: libc::c_float =
        rsqrt(*pNormal.offset(0 as libc::c_int as isize) *
                  *pNormal.offset(0 as libc::c_int as isize) +
                  *pNormal.offset(1 as libc::c_int as isize) *
                      *pNormal.offset(1 as libc::c_int as isize) +
                  *pNormal.offset(2 as libc::c_int as isize) *
                      *pNormal.offset(2 as libc::c_int as isize));
    let ref mut fresh3 = *pNormal.offset(0 as libc::c_int as isize);
    *fresh3 *= ilength;
    let ref mut fresh4 = *pNormal.offset(1 as libc::c_int as isize);
    *fresh4 *= ilength;
    let ref mut fresh5 = *pNormal.offset(2 as libc::c_int as isize);
    *fresh5 *= ilength;
}
/*
==============
R_BeamCull

Cull the beam by bbox
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_BeamCull(mut start: *const vec_t,
                                    mut end: *const vec_t,
                                    mut pvsOnly: qboolean) -> qboolean {
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if *start.offset(i as isize) < *end.offset(i as isize) {
            mins[i as usize] = *start.offset(i as isize);
            maxs[i as usize] = *end.offset(i as isize)
        } else {
            mins[i as usize] = *end.offset(i as isize);
            maxs[i as usize] = *start.offset(i as isize)
        }
        // don't let it be zero sized
        if mins[i as usize] == maxs[i as usize] { maxs[i as usize] += 1.0f32 }
        i += 1
    }
    // check bbox
    if gEngfuncs.Mod_BoxVisible.expect("non-null function pointer")(mins.as_mut_ptr()
                                                                        as
                                                                        *const vec_t,
                                                                    maxs.as_mut_ptr()
                                                                        as
                                                                        *const vec_t,
                                                                    Mod_GetCurrentVis())
           as u64 != 0 {
        if pvsOnly as libc::c_uint != 0 ||
               R_CullBox(mins.as_mut_ptr() as *const vec_t,
                         maxs.as_mut_ptr() as *const vec_t) as u64 == 0 {
            // beam is visible
            return false_0
        }
    }
    // beam is culled
    return true_0;
}
/*
================
CL_AddCustomBeam

Add the beam that encoded as custom entity
================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_AddCustomBeam(mut pEnvBeam: *mut cl_entity_t) {
    if (*tr.draw_list).num_beam_entities >=
           ((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint {
        gEngfuncs.Con_Printf.expect("non-null function pointer")(b"^1Error:^7 Too many beams %d!\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 (*tr.draw_list).num_beam_entities);
        return
    }
    if !pEnvBeam.is_null() {
        (*tr.draw_list).beam_entities[(*tr.draw_list).num_beam_entities as
                                          usize] = pEnvBeam;
        (*tr.draw_list).num_beam_entities =
            (*tr.draw_list).num_beam_entities.wrapping_add(1)
    };
}
/*
==============================================================

BEAM DRAW METHODS

==============================================================
*/
/*
================
R_DrawSegs

general code for drawing beams
================
*/
unsafe extern "C" fn R_DrawSegs(mut source: *mut vec_t, mut delta: *mut vec_t,
                                mut width: libc::c_float,
                                mut scale: libc::c_float,
                                mut freq: libc::c_float,
                                mut speed: libc::c_float,
                                mut segments: libc::c_int,
                                mut flags: libc::c_int) {
    let mut noiseIndex: libc::c_int = 0;
    let mut noiseStep: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut total_segs: libc::c_int = 0;
    let mut segs_drawn: libc::c_int = 0;
    let mut div: libc::c_float = 0.;
    let mut length: libc::c_float = 0.;
    let mut fraction: libc::c_float = 0.;
    let mut factor: libc::c_float = 0.;
    let mut flMaxWidth: libc::c_float = 0.;
    let mut vLast: libc::c_float = 0.;
    let mut vStep: libc::c_float = 0.;
    let mut brightness: libc::c_float = 0.;
    let mut perp1: vec3_t = [0.; 3];
    let mut vLastNormal: vec3_t = [0.; 3];
    let mut curSeg: beamseg_t =
        beamseg_t{pos: [0.; 3], texcoord: 0., width: 0.,};
    if segments < 2 as libc::c_int { return }
    length =
        __tg_sqrt(*delta.offset(0 as libc::c_int as isize) *
                      *delta.offset(0 as libc::c_int as isize) +
                      *delta.offset(1 as libc::c_int as isize) *
                          *delta.offset(1 as libc::c_int as isize) +
                      *delta.offset(2 as libc::c_int as isize) *
                          *delta.offset(2 as libc::c_int as isize));
    flMaxWidth = width * 0.5f32;
    div = 1.0f32 / (segments - 1 as libc::c_int) as libc::c_float;
    if length * div < flMaxWidth * 1.414f32 {
        // here, we have too many segments; we could get overlap... so lets have less segments
        segments =
            ((length / (flMaxWidth * 1.414f32)) as libc::c_int as
                 libc::c_float + 1.0f32) as
                libc::c_int; // Texture length texels per space pixel
        if segments < 2 as libc::c_int { segments = 2 as libc::c_int }
    }
    if segments > 64 as libc::c_int { segments = 64 as libc::c_int }
    div = 1.0f32 / (segments - 1 as libc::c_int) as libc::c_float;
    length *= 0.01f32;
    vStep = length * div;
    // Scroll speed 3.5 -- initial texture position, scrolls 3.5/sec (1.0 is entire texture)
    vLast =
        __tg_fmod_0((freq * speed) as libc::c_double,
                    1 as libc::c_int as libc::c_double) as libc::c_float;
    if flags & 0x10 as libc::c_int != 0 {
        if segments < 16 as libc::c_int {
            segments = 16 as libc::c_int;
            div = 1.0f32 / (segments - 1 as libc::c_int) as libc::c_float
        }
        scale *= 100.0f32;
        length = segments as libc::c_float * 0.1f32
    } else { scale *= length * 2.0f32 }
    // Iterator to resample noise waveform (it needs to be generated in powers of 2)
    noiseStep =
        ((64 as libc::c_int - 1 as libc::c_int) as libc::c_float * div *
             65536.0f32) as libc::c_int;
    brightness = 1.0f32;
    noiseIndex = 0 as libc::c_int;
    if flags & 0x40 as libc::c_int != 0 {
        brightness = 0 as libc::c_int as libc::c_float
    }
    // Choose two vectors that are perpendicular to the beam
    R_BeamComputePerpendicular(delta as *const vec_t, perp1.as_mut_ptr());
    total_segs = segments;
    segs_drawn = 0 as libc::c_int;
    // specify all the segments.
    i = 0 as libc::c_int;
    while i < segments {
        let mut nextSeg: beamseg_t =
            beamseg_t{pos: [0.; 3], texcoord: 0., width: 0.,};
        let mut vPoint1: vec3_t = [0.; 3];
        let mut vPoint2: vec3_t = [0.; 3];
        if !(noiseIndex < (64 as libc::c_int) << 16 as libc::c_int) {
            gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     b"../ref_gl/gl_beams.c\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     259 as
                                                                         libc::c_int);
        }
        fraction = i as libc::c_float * div;
        nextSeg.pos[0 as libc::c_int as usize] =
            *source.offset(0 as libc::c_int as isize) +
                fraction * *delta.offset(0 as libc::c_int as isize);
        nextSeg.pos[1 as libc::c_int as usize] =
            *source.offset(1 as libc::c_int as isize) +
                fraction * *delta.offset(1 as libc::c_int as isize);
        nextSeg.pos[2 as libc::c_int as usize] =
            *source.offset(2 as libc::c_int as isize) +
                fraction * *delta.offset(2 as libc::c_int as isize);
        // distort using noise
        if scale != 0 as libc::c_int as libc::c_float {
            factor =
                rgNoise[(noiseIndex >> 16 as libc::c_int) as usize] * scale;
            if flags & 0x10 as libc::c_int != 0 {
                let mut s: libc::c_float = 0.;
                let mut c: libc::c_float = 0.;
                SinCos(fraction * 3.14159265358979323846f64 as libc::c_float *
                           length + freq, &mut s, &mut c);
                nextSeg.pos[0 as libc::c_int as usize] =
                    nextSeg.pos[0 as libc::c_int as usize] +
                        factor * s * RI.vup[0 as libc::c_int as usize];
                nextSeg.pos[1 as libc::c_int as usize] =
                    nextSeg.pos[1 as libc::c_int as usize] +
                        factor * s * RI.vup[1 as libc::c_int as usize];
                nextSeg.pos[2 as libc::c_int as usize] =
                    nextSeg.pos[2 as libc::c_int as usize] +
                        factor * s * RI.vup[2 as libc::c_int as usize];
                // rotate the noise along the perpendicluar axis a bit to keep the bolt from looking diagonal
                nextSeg.pos[0 as libc::c_int as usize] =
                    nextSeg.pos[0 as libc::c_int as usize] +
                        factor * c * RI.vright[0 as libc::c_int as usize];
                nextSeg.pos[1 as libc::c_int as usize] =
                    nextSeg.pos[1 as libc::c_int as usize] +
                        factor * c * RI.vright[1 as libc::c_int as usize];
                nextSeg.pos[2 as libc::c_int as usize] =
                    nextSeg.pos[2 as libc::c_int as usize] +
                        factor * c * RI.vright[2 as libc::c_int as usize]
            } else {
                nextSeg.pos[0 as libc::c_int as usize] =
                    nextSeg.pos[0 as libc::c_int as usize] +
                        factor * perp1[0 as libc::c_int as usize];
                nextSeg.pos[1 as libc::c_int as usize] =
                    nextSeg.pos[1 as libc::c_int as usize] +
                        factor * perp1[1 as libc::c_int as usize];
                nextSeg.pos[2 as libc::c_int as usize] =
                    nextSeg.pos[2 as libc::c_int as usize] +
                        factor * perp1[2 as libc::c_int as usize]
            }
        }
        // specify the next segment.
        nextSeg.width = width * 2.0f32;
        nextSeg.texcoord = vLast;
        if segs_drawn > 0 as libc::c_int {
            // Get a vector that is perpendicular to us and perpendicular to the beam.
			// This is used to fatten the beam.
            let mut vNormal: vec3_t = [0.; 3];
            let mut vAveNormal: vec3_t = [0.; 3];
            R_BeamComputeNormal(curSeg.pos.as_mut_ptr() as *const vec_t,
                                nextSeg.pos.as_mut_ptr() as *const vec_t,
                                vNormal.as_mut_ptr());
            if segs_drawn > 1 as libc::c_int {
                // Average this with the previous normal
                vAveNormal[0 as libc::c_int as usize] =
                    vNormal[0 as libc::c_int as usize] +
                        vLastNormal[0 as libc::c_int as usize];
                vAveNormal[1 as libc::c_int as usize] =
                    vNormal[1 as libc::c_int as usize] +
                        vLastNormal[1 as libc::c_int as usize];
                vAveNormal[2 as libc::c_int as usize] =
                    vNormal[2 as libc::c_int as usize] +
                        vLastNormal[2 as libc::c_int as usize];
                vAveNormal[0 as libc::c_int as usize] =
                    vAveNormal[0 as libc::c_int as usize] * 0.5f32;
                vAveNormal[1 as libc::c_int as usize] =
                    vAveNormal[1 as libc::c_int as usize] * 0.5f32;
                vAveNormal[2 as libc::c_int as usize] =
                    vAveNormal[2 as libc::c_int as usize] * 0.5f32;
                let mut ilength: libc::c_float =
                    rsqrt(vAveNormal[0 as libc::c_int as usize] *
                              vAveNormal[0 as libc::c_int as usize] +
                              vAveNormal[1 as libc::c_int as usize] *
                                  vAveNormal[1 as libc::c_int as usize] +
                              vAveNormal[2 as libc::c_int as usize] *
                                  vAveNormal[2 as libc::c_int as usize]);
                vAveNormal[0 as libc::c_int as usize] *= ilength;
                vAveNormal[1 as libc::c_int as usize] *= ilength;
                vAveNormal[2 as libc::c_int as usize] *= ilength
            } else {
                vAveNormal[0 as libc::c_int as usize] =
                    vNormal[0 as libc::c_int as usize];
                vAveNormal[1 as libc::c_int as usize] =
                    vNormal[1 as libc::c_int as usize];
                vAveNormal[2 as libc::c_int as usize] =
                    vNormal[2 as libc::c_int as usize]
            }
            vLastNormal[0 as libc::c_int as usize] =
                vNormal[0 as libc::c_int as usize];
            vLastNormal[1 as libc::c_int as usize] =
                vNormal[1 as libc::c_int as usize];
            vLastNormal[2 as libc::c_int as usize] =
                vNormal[2 as libc::c_int as usize];
            // draw regular segment
            vPoint1[0 as libc::c_int as usize] =
                curSeg.pos[0 as libc::c_int as usize] +
                    curSeg.width * 0.5f32 *
                        vAveNormal[0 as libc::c_int as usize];
            vPoint1[1 as libc::c_int as usize] =
                curSeg.pos[1 as libc::c_int as usize] +
                    curSeg.width * 0.5f32 *
                        vAveNormal[1 as libc::c_int as usize];
            vPoint1[2 as libc::c_int as usize] =
                curSeg.pos[2 as libc::c_int as usize] +
                    curSeg.width * 0.5f32 *
                        vAveNormal[2 as libc::c_int as usize];
            vPoint2[0 as libc::c_int as usize] =
                curSeg.pos[0 as libc::c_int as usize] +
                    -curSeg.width * 0.5f32 *
                        vAveNormal[0 as libc::c_int as usize];
            vPoint2[1 as libc::c_int as usize] =
                curSeg.pos[1 as libc::c_int as usize] +
                    -curSeg.width * 0.5f32 *
                        vAveNormal[1 as libc::c_int as usize];
            vPoint2[2 as libc::c_int as usize] =
                curSeg.pos[2 as libc::c_int as usize] +
                    -curSeg.width * 0.5f32 *
                        vAveNormal[2 as libc::c_int as usize];
            pglTexCoord2f.expect("non-null function pointer")(0.0f32,
                                                              curSeg.texcoord);
            TriBrightness(brightness);
            pglNormal3fv.expect("non-null function pointer")(vAveNormal.as_mut_ptr());
            pglVertex3fv.expect("non-null function pointer")(vPoint1.as_mut_ptr());
            pglTexCoord2f.expect("non-null function pointer")(1.0f32,
                                                              curSeg.texcoord);
            TriBrightness(brightness);
            pglNormal3fv.expect("non-null function pointer")(vAveNormal.as_mut_ptr());
            pglVertex3fv.expect("non-null function pointer")(vPoint2.as_mut_ptr());
        }
        curSeg = nextSeg;
        segs_drawn += 1;
        if flags & 0x40 as libc::c_int != 0 &&
               flags & 0x80 as libc::c_int != 0 {
            if fraction < 0.5f32 {
                brightness = fraction
            } else { brightness = 1.0f32 - fraction }
        } else if flags & 0x40 as libc::c_int != 0 {
            brightness = fraction
        } else if flags & 0x80 as libc::c_int != 0 {
            brightness = 1.0f32 - fraction
        }
        if segs_drawn == total_segs {
            // draw the last segment
            vPoint1[0 as libc::c_int as usize] =
                curSeg.pos[0 as libc::c_int as usize] +
                    curSeg.width * 0.5f32 *
                        vLastNormal[0 as libc::c_int as usize];
            vPoint1[1 as libc::c_int as usize] =
                curSeg.pos[1 as libc::c_int as usize] +
                    curSeg.width * 0.5f32 *
                        vLastNormal[1 as libc::c_int as usize];
            vPoint1[2 as libc::c_int as usize] =
                curSeg.pos[2 as libc::c_int as usize] +
                    curSeg.width * 0.5f32 *
                        vLastNormal[2 as libc::c_int as usize];
            vPoint2[0 as libc::c_int as usize] =
                curSeg.pos[0 as libc::c_int as usize] +
                    -curSeg.width * 0.5f32 *
                        vLastNormal[0 as libc::c_int as usize];
            vPoint2[1 as libc::c_int as usize] =
                curSeg.pos[1 as libc::c_int as usize] +
                    -curSeg.width * 0.5f32 *
                        vLastNormal[1 as libc::c_int as usize];
            vPoint2[2 as libc::c_int as usize] =
                curSeg.pos[2 as libc::c_int as usize] +
                    -curSeg.width * 0.5f32 *
                        vLastNormal[2 as libc::c_int as usize];
            // specify the points.
            pglTexCoord2f.expect("non-null function pointer")(0.0f32,
                                                              curSeg.texcoord); // Advance texture scroll (v axis only)
            TriBrightness(brightness);
            pglNormal3fv.expect("non-null function pointer")(vLastNormal.as_mut_ptr());
            pglVertex3fv.expect("non-null function pointer")(vPoint1.as_mut_ptr());
            pglTexCoord2f.expect("non-null function pointer")(1.0f32,
                                                              curSeg.texcoord);
            TriBrightness(brightness);
            pglNormal3fv.expect("non-null function pointer")(vLastNormal.as_mut_ptr());
            pglVertex3fv.expect("non-null function pointer")(vPoint2.as_mut_ptr());
        }
        vLast += vStep;
        noiseIndex += noiseStep;
        i += 1
    };
}
/*
================
R_DrawTorus

Draw beamtours
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_DrawTorus(mut source: *mut vec_t,
                                     mut delta: *mut vec_t,
                                     mut width: libc::c_float,
                                     mut scale: libc::c_float,
                                     mut freq: libc::c_float,
                                     mut speed: libc::c_float,
                                     mut segments: libc::c_int) {
    let mut i: libc::c_int =
        0; // don't lose all of the noise/texture on short beams
    let mut noiseIndex: libc::c_int =
        0; // Texture length texels per space pixel
    let mut noiseStep: libc::c_int = 0;
    let mut div: libc::c_float = 0.;
    let mut length: libc::c_float = 0.;
    let mut fraction: libc::c_float = 0.;
    let mut factor: libc::c_float = 0.;
    let mut vLast: libc::c_float = 0.;
    let mut vStep: libc::c_float = 0.;
    let mut last1: vec3_t = [0.; 3];
    let mut last2: vec3_t = [0.; 3];
    let mut point: vec3_t = [0.; 3];
    let mut screen: vec3_t = [0.; 3];
    let mut screenLast: vec3_t = [0.; 3];
    let mut tmp: vec3_t = [0.; 3];
    let mut normal: vec3_t = [0.; 3];
    if segments < 2 as libc::c_int { return }
    if segments > 64 as libc::c_int { segments = 64 as libc::c_int }
    length =
        __tg_sqrt(*delta.offset(0 as libc::c_int as isize) *
                      *delta.offset(0 as libc::c_int as isize) +
                      *delta.offset(1 as libc::c_int as isize) *
                          *delta.offset(1 as libc::c_int as isize) +
                      *delta.offset(2 as libc::c_int as isize) *
                          *delta.offset(2 as libc::c_int as isize)) * 0.01f32;
    if length < 0.5f32 { length = 0.5f32 }
    div = 1.0f32 / (segments - 1 as libc::c_int) as libc::c_float;
    vStep = length * div;
    // Scroll speed 3.5 -- initial texture position, scrolls 3.5/sec (1.0 is entire texture)
    vLast =
        __tg_fmod_0((freq * speed) as libc::c_double,
                    1 as libc::c_int as libc::c_double) as libc::c_float;
    scale = scale * length;
    // Iterator to resample noise waveform (it needs to be generated in powers of 2)
    noiseStep =
        ((64 as libc::c_int - 1 as libc::c_int) as libc::c_float * div *
             65536.0f32) as libc::c_int;
    noiseIndex = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < segments {
        let mut s: libc::c_float = 0.;
        let mut c: libc::c_float = 0.;
        fraction = i as libc::c_float * div;
        SinCos(fraction *
                   (3.14159265358979323846f64 *
                        2 as libc::c_int as libc::c_double) as libc::c_float,
               &mut s, &mut c);
        point[0 as libc::c_int as usize] =
            s * freq * *delta.offset(2 as libc::c_int as isize) +
                *source.offset(0 as libc::c_int as isize);
        point[1 as libc::c_int as usize] =
            c * freq * *delta.offset(2 as libc::c_int as isize) +
                *source.offset(1 as libc::c_int as isize);
        point[2 as libc::c_int as usize] =
            *source.offset(2 as libc::c_int as isize);
        // distort using noise
        if scale != 0 as libc::c_int as libc::c_float {
            if (noiseIndex >> 16 as libc::c_int) < 64 as libc::c_int {
                factor =
                    rgNoise[(noiseIndex >> 16 as libc::c_int) as usize] *
                        scale;
                point[0 as libc::c_int as usize] =
                    point[0 as libc::c_int as usize] +
                        factor * RI.vup[0 as libc::c_int as usize];
                point[1 as libc::c_int as usize] =
                    point[1 as libc::c_int as usize] +
                        factor * RI.vup[1 as libc::c_int as usize];
                point[2 as libc::c_int as usize] =
                    point[2 as libc::c_int as usize] +
                        factor * RI.vup[2 as libc::c_int as usize];
                // rotate the noise along the perpendicluar axis a bit to keep the bolt from looking diagonal
                factor =
                    rgNoise[(noiseIndex >> 16 as libc::c_int) as usize] *
                        scale *
                        __tg_cos(fraction *
                                     3.14159265358979323846f64 as
                                         libc::c_float *
                                     3 as libc::c_int as libc::c_float +
                                     freq);
                point[0 as libc::c_int as usize] =
                    point[0 as libc::c_int as usize] +
                        factor * RI.vright[0 as libc::c_int as usize];
                point[1 as libc::c_int as usize] =
                    point[1 as libc::c_int as usize] +
                        factor * RI.vright[1 as libc::c_int as usize];
                point[2 as libc::c_int as usize] =
                    point[2 as libc::c_int as usize] +
                        factor * RI.vright[2 as libc::c_int as usize]
            }
        }
        // Transform point into screen space
        TriWorldToScreen(point.as_mut_ptr(), screen.as_mut_ptr());
        if i != 0 as libc::c_int {
            // build world-space normal to screen-space direction vector
            tmp[0 as libc::c_int as usize] =
                screen[0 as libc::c_int as usize] -
                    screenLast[0 as libc::c_int as usize];
            tmp[1 as libc::c_int as usize] =
                screen[1 as libc::c_int as usize] -
                    screenLast[1 as libc::c_int as usize];
            tmp[2 as libc::c_int as usize] =
                screen[2 as libc::c_int as usize] -
                    screenLast[2 as libc::c_int as usize];
            // we don't need Z, we're in screen space
            tmp[2 as libc::c_int as usize] =
                0 as libc::c_int as
                    vec_t; // Build point along noraml line (normal is -y, x)
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
            normal[0 as libc::c_int as usize] =
                RI.vup[0 as libc::c_int as usize] *
                    -tmp[0 as libc::c_int as usize];
            normal[1 as libc::c_int as usize] =
                RI.vup[1 as libc::c_int as usize] *
                    -tmp[0 as libc::c_int as usize];
            normal[2 as libc::c_int as usize] =
                RI.vup[2 as libc::c_int as usize] *
                    -tmp[0 as libc::c_int as usize];
            normal[0 as libc::c_int as usize] =
                normal[0 as libc::c_int as usize] +
                    tmp[1 as libc::c_int as usize] *
                        RI.vright[0 as libc::c_int as usize];
            normal[1 as libc::c_int as usize] =
                normal[1 as libc::c_int as usize] +
                    tmp[1 as libc::c_int as usize] *
                        RI.vright[1 as libc::c_int as usize];
            normal[2 as libc::c_int as usize] =
                normal[2 as libc::c_int as usize] +
                    tmp[1 as libc::c_int as usize] *
                        RI.vright[2 as libc::c_int as usize];
            // Make a wide line
            last1[0 as libc::c_int as usize] =
                point[0 as libc::c_int as usize] +
                    width *
                        normal[0 as libc::c_int as
                                   usize]; // advance texture scroll (v axis only)
            last1[1 as libc::c_int as usize] =
                point[1 as libc::c_int as usize] +
                    width * normal[1 as libc::c_int as usize];
            last1[2 as libc::c_int as usize] =
                point[2 as libc::c_int as usize] +
                    width * normal[2 as libc::c_int as usize];
            last2[0 as libc::c_int as usize] =
                point[0 as libc::c_int as usize] +
                    -width * normal[0 as libc::c_int as usize];
            last2[1 as libc::c_int as usize] =
                point[1 as libc::c_int as usize] +
                    -width * normal[1 as libc::c_int as usize];
            last2[2 as libc::c_int as usize] =
                point[2 as libc::c_int as usize] +
                    -width * normal[2 as libc::c_int as usize];
            vLast += vStep;
            TriTexCoord2f(1 as libc::c_int as libc::c_float, vLast);
            TriVertex3fv(last2.as_mut_ptr());
            TriTexCoord2f(0 as libc::c_int as libc::c_float, vLast);
            TriVertex3fv(last1.as_mut_ptr());
        }
        screenLast[0 as libc::c_int as usize] =
            screen[0 as libc::c_int as usize];
        screenLast[1 as libc::c_int as usize] =
            screen[1 as libc::c_int as usize];
        screenLast[2 as libc::c_int as usize] =
            screen[2 as libc::c_int as usize];
        noiseIndex += noiseStep;
        i += 1
    };
}
/*
================
R_DrawDisk

Draw beamdisk
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_DrawDisk(mut source: *mut vec_t,
                                    mut delta: *mut vec_t,
                                    mut width: libc::c_float,
                                    mut scale: libc::c_float,
                                    mut freq: libc::c_float,
                                    mut speed: libc::c_float,
                                    mut segments: libc::c_int) {
    let mut div: libc::c_float = 0.;
    let mut length: libc::c_float = 0.;
    let mut fraction: libc::c_float = 0.;
    let mut w: libc::c_float = 0.;
    let mut vLast: libc::c_float = 0.;
    let mut vStep: libc::c_float = 0.;
    let mut point: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    if segments < 2 as libc::c_int { return }
    if segments > 64 as libc::c_int {
        // UNDONE: Allow more segments?
        segments = 64 as libc::c_int
    } // don't lose all of the noise/texture on short beams
    length =
        __tg_sqrt(*delta.offset(0 as libc::c_int as isize) *
                      *delta.offset(0 as libc::c_int as isize) +
                      *delta.offset(1 as libc::c_int as isize) *
                          *delta.offset(1 as libc::c_int as isize) +
                      *delta.offset(2 as libc::c_int as isize) *
                          *delta.offset(2 as libc::c_int as isize)) *
            0.01f32; // Texture length texels per space pixel
    if length < 0.5f32 { length = 0.5f32 }
    div = 1.0f32 / (segments - 1 as libc::c_int) as libc::c_float;
    vStep = length * div;
    // scroll speed 3.5 -- initial texture position, scrolls 3.5/sec (1.0 is entire texture)
    vLast =
        __tg_fmod_0((freq * speed) as libc::c_double,
                    1 as libc::c_int as libc::c_double) as libc::c_float;
    scale = scale * length;
    // clamp the beam width
    w =
        __tg_fmod(freq, width * 0.1f32) *
            *delta.offset(2 as libc::c_int as isize);
    // NOTE: we must force the degenerate triangles to be on the edge
    i = 0 as libc::c_int;
    while i < segments {
        let mut s: libc::c_float = 0.;
        let mut c: libc::c_float = 0.;
        fraction = i as libc::c_float * div;
        point[0 as libc::c_int as usize] =
            *source.offset(0 as libc::c_int as isize);
        point[1 as libc::c_int as usize] =
            *source.offset(1 as libc::c_int as isize);
        point[2 as libc::c_int as usize] =
            *source.offset(2 as libc::c_int as isize);
        TriBrightness(1.0f32);
        TriTexCoord2f(1.0f32, vLast);
        TriVertex3fv(point.as_mut_ptr());
        SinCos(fraction *
                   (3.14159265358979323846f64 *
                        2 as libc::c_int as libc::c_double) as libc::c_float,
               &mut s, &mut c);
        point[0 as libc::c_int as usize] =
            s * w + *source.offset(0 as libc::c_int as isize);
        point[1 as libc::c_int as usize] =
            c * w + *source.offset(1 as libc::c_int as isize);
        point[2 as libc::c_int as usize] =
            *source.offset(2 as libc::c_int as isize);
        TriBrightness(1.0f32);
        TriTexCoord2f(0.0f32, vLast);
        TriVertex3fv(point.as_mut_ptr());
        vLast += vStep;
        i += 1
        // advance texture scroll (v axis only)
    };
}
/*
================
R_DrawCylinder

Draw beam cylinder
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_DrawCylinder(mut source: *mut vec_t,
                                        mut delta: *mut vec_t,
                                        mut width: libc::c_float,
                                        mut scale: libc::c_float,
                                        mut freq: libc::c_float,
                                        mut speed: libc::c_float,
                                        mut segments: libc::c_int) {
    let mut div: libc::c_float =
        0.; // don't lose all of the noise/texture on short beams
    let mut length: libc::c_float =
        0.; // texture length texels per space pixel
    let mut fraction: libc::c_float = 0.;
    let mut vLast: libc::c_float = 0.;
    let mut vStep: libc::c_float = 0.;
    let mut point: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    if segments < 2 as libc::c_int { return }
    if segments > 64 as libc::c_int { segments = 64 as libc::c_int }
    length =
        __tg_sqrt(*delta.offset(0 as libc::c_int as isize) *
                      *delta.offset(0 as libc::c_int as isize) +
                      *delta.offset(1 as libc::c_int as isize) *
                          *delta.offset(1 as libc::c_int as isize) +
                      *delta.offset(2 as libc::c_int as isize) *
                          *delta.offset(2 as libc::c_int as isize)) * 0.01f32;
    if length < 0.5f32 { length = 0.5f32 }
    div = 1.0f32 / (segments - 1 as libc::c_int) as libc::c_float;
    vStep = length * div;
    // Scroll speed 3.5 -- initial texture position, scrolls 3.5/sec (1.0 is entire texture)
    vLast =
        __tg_fmod_0((freq * speed) as libc::c_double,
                    1 as libc::c_int as libc::c_double) as libc::c_float;
    scale = scale * length;
    i = 0 as libc::c_int;
    while i < segments {
        let mut s: libc::c_float = 0.;
        let mut c: libc::c_float = 0.;
        fraction = i as libc::c_float * div;
        SinCos(fraction *
                   (3.14159265358979323846f64 *
                        2 as libc::c_int as libc::c_double) as libc::c_float,
               &mut s, &mut c);
        point[0 as libc::c_int as usize] =
            s * freq * *delta.offset(2 as libc::c_int as isize) +
                *source.offset(0 as libc::c_int as isize);
        point[1 as libc::c_int as usize] =
            c * freq * *delta.offset(2 as libc::c_int as isize) +
                *source.offset(1 as libc::c_int as isize);
        point[2 as libc::c_int as usize] =
            *source.offset(2 as libc::c_int as isize) + width;
        TriBrightness(0 as libc::c_int as libc::c_float);
        TriTexCoord2f(1 as libc::c_int as libc::c_float, vLast);
        TriVertex3fv(point.as_mut_ptr());
        point[0 as libc::c_int as usize] =
            s * freq * (*delta.offset(2 as libc::c_int as isize) + width) +
                *source.offset(0 as libc::c_int as isize);
        point[1 as libc::c_int as usize] =
            c * freq * (*delta.offset(2 as libc::c_int as isize) + width) +
                *source.offset(1 as libc::c_int as isize);
        point[2 as libc::c_int as usize] =
            *source.offset(2 as libc::c_int as isize) - width;
        TriBrightness(1 as libc::c_int as libc::c_float);
        TriTexCoord2f(0 as libc::c_int as libc::c_float, vLast);
        TriVertex3fv(point.as_mut_ptr());
        vLast += vStep;
        i += 1
        // Advance texture scroll (v axis only)
    };
}
/*
==============
R_DrawBeamFollow

drawi followed beam
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_DrawBeamFollow(mut pbeam: *mut BEAM,
                                          mut frametime: libc::c_float) {
    let mut pnew: *mut particle_t = 0 as *mut particle_t;
    let mut particles: *mut particle_t = 0 as *mut particle_t;
    let mut fraction: libc::c_float = 0.;
    let mut div: libc::c_float = 0.;
    let mut vLast: libc::c_float = 0.;
    let mut vStep: libc::c_float = 0.;
    let mut last1: vec3_t = [0.; 3];
    let mut last2: vec3_t = [0.; 3];
    let mut tmp: vec3_t = [0.; 3];
    let mut screen: vec3_t = [0.; 3];
    let mut delta: vec3_t = [0.; 3];
    let mut screenLast: vec3_t = [0.; 3];
    let mut normal: vec3_t = [0.; 3];
    gEngfuncs.R_FreeDeadParticles.expect("non-null function pointer")(&mut (*pbeam).particles);
    particles = (*pbeam).particles;
    pnew = 0 as *mut particle_t;
    div = 0 as libc::c_int as libc::c_float;
    if (*pbeam).flags & 0x1 as libc::c_int != 0 {
        if !particles.is_null() {
            delta[0 as libc::c_int as usize] =
                (*particles).org[0 as libc::c_int as usize] -
                    (*pbeam).source[0 as libc::c_int as usize];
            delta[1 as libc::c_int as usize] =
                (*particles).org[1 as libc::c_int as usize] -
                    (*pbeam).source[1 as libc::c_int as usize];
            delta[2 as libc::c_int as usize] =
                (*particles).org[2 as libc::c_int as usize] -
                    (*pbeam).source[2 as libc::c_int as usize];
            div =
                __tg_sqrt(delta[0 as libc::c_int as usize] *
                              delta[0 as libc::c_int as usize] +
                              delta[1 as libc::c_int as usize] *
                                  delta[1 as libc::c_int as usize] +
                              delta[2 as libc::c_int as usize] *
                                  delta[2 as libc::c_int as usize]);
            if div >= 32 as libc::c_int as libc::c_float {
                pnew =
                    gEngfuncs.CL_AllocParticleFast.expect("non-null function pointer")()
            }
        } else {
            pnew =
                gEngfuncs.CL_AllocParticleFast.expect("non-null function pointer")()
        }
    }
    if !pnew.is_null() {
        (*pnew).org[0 as libc::c_int as usize] =
            (*pbeam).source[0 as libc::c_int as usize];
        (*pnew).org[1 as libc::c_int as usize] =
            (*pbeam).source[1 as libc::c_int as usize];
        (*pnew).org[2 as libc::c_int as usize] =
            (*pbeam).source[2 as libc::c_int as usize];
        (*pnew).die = (*gpGlobals).time + (*pbeam).amplitude;
        (*pnew).vel[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
        (*pnew).vel[1 as libc::c_int as usize] =
            (*pnew).vel[2 as libc::c_int as usize];
        (*pnew).vel[0 as libc::c_int as usize] =
            (*pnew).vel[1 as libc::c_int as usize];
        (*pnew).next = particles;
        (*pbeam).particles = pnew;
        particles = pnew
    }
    // nothing to draw
    if particles.is_null() { return }
    if pnew.is_null() && div != 0 as libc::c_int as libc::c_float {
        delta[0 as libc::c_int as usize] =
            (*pbeam).source[0 as libc::c_int as usize];
        delta[1 as libc::c_int as usize] =
            (*pbeam).source[1 as libc::c_int as usize];
        delta[2 as libc::c_int as usize] =
            (*pbeam).source[2 as libc::c_int as usize];
        TriWorldToScreen((*pbeam).source.as_mut_ptr(),
                         screenLast.as_mut_ptr());
        TriWorldToScreen((*particles).org.as_mut_ptr(), screen.as_mut_ptr());
    } else if !particles.is_null() && !(*particles).next.is_null() {
        delta[0 as libc::c_int as usize] =
            (*particles).org[0 as libc::c_int as usize];
        delta[1 as libc::c_int as usize] =
            (*particles).org[1 as libc::c_int as usize];
        delta[2 as libc::c_int as usize] =
            (*particles).org[2 as libc::c_int as usize];
        TriWorldToScreen((*particles).org.as_mut_ptr(),
                         screenLast.as_mut_ptr());
        TriWorldToScreen((*(*particles).next).org.as_mut_ptr(),
                         screen.as_mut_ptr());
        particles = (*particles).next
    } else { return }
    // UNDONE: This won't work, screen and screenLast must be extrapolated here to fix the
	// first beam segment for this trail
    // build world-space normal to screen-space direction vector
    tmp[0 as libc::c_int as usize] =
        screen[0 as libc::c_int as usize] -
            screenLast[0 as libc::c_int as usize];
    tmp[1 as libc::c_int as usize] =
        screen[1 as libc::c_int as usize] -
            screenLast[1 as libc::c_int as usize];
    tmp[2 as libc::c_int as usize] =
        screen[2 as libc::c_int as usize] -
            screenLast[2 as libc::c_int as usize];
    // we don't need Z, we're in screen space
    tmp[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
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
    // Build point along noraml line (normal is -y, x)
    normal[0 as libc::c_int as usize] =
        RI.vup[0 as libc::c_int as usize] *
            tmp[0 as libc::c_int as
                    usize]; // Build point along normal line (normal is -y, x)
    normal[1 as libc::c_int as usize] =
        RI.vup[1 as libc::c_int as usize] * tmp[0 as libc::c_int as usize];
    normal[2 as libc::c_int as usize] =
        RI.vup[2 as libc::c_int as usize] * tmp[0 as libc::c_int as usize];
    normal[0 as libc::c_int as usize] =
        normal[0 as libc::c_int as usize] +
            tmp[1 as libc::c_int as usize] *
                RI.vright[0 as libc::c_int as usize];
    normal[1 as libc::c_int as usize] =
        normal[1 as libc::c_int as usize] +
            tmp[1 as libc::c_int as usize] *
                RI.vright[1 as libc::c_int as usize];
    normal[2 as libc::c_int as usize] =
        normal[2 as libc::c_int as usize] +
            tmp[1 as libc::c_int as usize] *
                RI.vright[2 as libc::c_int as usize];
    // Make a wide line
    last1[0 as libc::c_int as usize] =
        delta[0 as libc::c_int as usize] +
            (*pbeam).width * normal[0 as libc::c_int as usize];
    last1[1 as libc::c_int as usize] =
        delta[1 as libc::c_int as usize] +
            (*pbeam).width * normal[1 as libc::c_int as usize];
    last1[2 as libc::c_int as usize] =
        delta[2 as libc::c_int as usize] +
            (*pbeam).width * normal[2 as libc::c_int as usize];
    last2[0 as libc::c_int as usize] =
        delta[0 as libc::c_int as usize] +
            -(*pbeam).width * normal[0 as libc::c_int as usize];
    last2[1 as libc::c_int as usize] =
        delta[1 as libc::c_int as usize] +
            -(*pbeam).width * normal[1 as libc::c_int as usize];
    last2[2 as libc::c_int as usize] =
        delta[2 as libc::c_int as usize] +
            -(*pbeam).width * normal[2 as libc::c_int as usize];
    div = 1.0f32 / (*pbeam).amplitude;
    fraction = ((*pbeam).die - (*gpGlobals).time) * div;
    vLast = 0.0f32;
    vStep = 1.0f32;
    while !particles.is_null() {
        TriBrightness(fraction);
        TriTexCoord2f(1 as libc::c_int as libc::c_float,
                      1 as libc::c_int as libc::c_float);
        TriVertex3fv(last2.as_mut_ptr());
        TriBrightness(fraction);
        TriTexCoord2f(0 as libc::c_int as libc::c_float,
                      1 as libc::c_int as libc::c_float);
        TriVertex3fv(last1.as_mut_ptr());
        // Transform point into screen space
        TriWorldToScreen((*particles).org.as_mut_ptr(), screen.as_mut_ptr());
        // Build world-space normal to screen-space direction vector
        tmp[0 as libc::c_int as usize] =
            screen[0 as libc::c_int as usize] -
                screenLast[0 as libc::c_int as usize];
        tmp[1 as libc::c_int as usize] =
            screen[1 as libc::c_int as usize] -
                screenLast[1 as libc::c_int as usize];
        tmp[2 as libc::c_int as usize] =
            screen[2 as libc::c_int as usize] -
                screenLast[2 as libc::c_int as usize];
        // we don't need Z, we're in screen space
        tmp[2 as libc::c_int as usize] =
            0 as libc::c_int as
                vec_t; // Build point along noraml line (normal is -y, x)
        let mut ilength_0: libc::c_float =
            __tg_sqrt(tmp[0 as libc::c_int as usize] *
                          tmp[0 as libc::c_int as usize] +
                          tmp[1 as libc::c_int as usize] *
                              tmp[1 as libc::c_int as usize] +
                          tmp[2 as libc::c_int as usize] *
                              tmp[2 as libc::c_int as usize]);
        if ilength_0 != 0. { ilength_0 = 1.0f32 / ilength_0 }
        tmp[0 as libc::c_int as usize] *= ilength_0;
        tmp[1 as libc::c_int as usize] *= ilength_0;
        tmp[2 as libc::c_int as usize] *= ilength_0;
        normal[0 as libc::c_int as usize] =
            RI.vup[0 as libc::c_int as usize] *
                tmp[0 as libc::c_int as usize];
        normal[1 as libc::c_int as usize] =
            RI.vup[1 as libc::c_int as usize] *
                tmp[0 as libc::c_int as usize];
        normal[2 as libc::c_int as usize] =
            RI.vup[2 as libc::c_int as usize] *
                tmp[0 as libc::c_int as usize];
        normal[0 as libc::c_int as usize] =
            normal[0 as libc::c_int as usize] +
                tmp[1 as libc::c_int as usize] *
                    RI.vright[0 as libc::c_int as usize];
        normal[1 as libc::c_int as usize] =
            normal[1 as libc::c_int as usize] +
                tmp[1 as libc::c_int as usize] *
                    RI.vright[1 as libc::c_int as usize];
        normal[2 as libc::c_int as usize] =
            normal[2 as libc::c_int as usize] +
                tmp[1 as libc::c_int as usize] *
                    RI.vright[2 as libc::c_int as usize];
        // Make a wide line
        last1[0 as libc::c_int as usize] =
            (*particles).org[0 as libc::c_int as usize] +
                (*pbeam).width *
                    normal[0 as libc::c_int as
                               usize]; // Advance texture scroll (v axis only)
        last1[1 as libc::c_int as usize] =
            (*particles).org[1 as libc::c_int as usize] +
                (*pbeam).width * normal[1 as libc::c_int as usize];
        last1[2 as libc::c_int as usize] =
            (*particles).org[2 as libc::c_int as usize] +
                (*pbeam).width * normal[2 as libc::c_int as usize];
        last2[0 as libc::c_int as usize] =
            (*particles).org[0 as libc::c_int as usize] +
                -(*pbeam).width * normal[0 as libc::c_int as usize];
        last2[1 as libc::c_int as usize] =
            (*particles).org[1 as libc::c_int as usize] +
                -(*pbeam).width * normal[1 as libc::c_int as usize];
        last2[2 as libc::c_int as usize] =
            (*particles).org[2 as libc::c_int as usize] +
                -(*pbeam).width * normal[2 as libc::c_int as usize];
        vLast += vStep;
        if !(*particles).next.is_null() {
            fraction = ((*particles).die - (*gpGlobals).time) * div
        } else { fraction = 0.0f64 as libc::c_float }
        TriBrightness(fraction);
        TriTexCoord2f(0 as libc::c_int as libc::c_float,
                      0 as libc::c_int as libc::c_float);
        TriVertex3fv(last1.as_mut_ptr());
        TriBrightness(fraction);
        TriTexCoord2f(1 as libc::c_int as libc::c_float,
                      0 as libc::c_int as libc::c_float);
        TriVertex3fv(last2.as_mut_ptr());
        screenLast[0 as libc::c_int as usize] =
            screen[0 as libc::c_int as usize];
        screenLast[1 as libc::c_int as usize] =
            screen[1 as libc::c_int as usize];
        screenLast[2 as libc::c_int as usize] =
            screen[2 as libc::c_int as usize];
        particles = (*particles).next
    }
    // drift popcorn trail if there is a velocity
    particles = (*pbeam).particles;
    while !particles.is_null() {
        (*particles).org[0 as libc::c_int as usize] =
            (*particles).org[0 as libc::c_int as usize] +
                frametime * (*particles).vel[0 as libc::c_int as usize];
        (*particles).org[1 as libc::c_int as usize] =
            (*particles).org[1 as libc::c_int as usize] +
                frametime * (*particles).vel[1 as libc::c_int as usize];
        (*particles).org[2 as libc::c_int as usize] =
            (*particles).org[2 as libc::c_int as usize] +
                frametime * (*particles).vel[2 as libc::c_int as usize];
        particles = (*particles).next
    };
}
/*
================
R_DrawRing

Draw beamring
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_DrawRing(mut source: *mut vec_t,
                                    mut delta: *mut vec_t,
                                    mut width: libc::c_float,
                                    mut amplitude: libc::c_float,
                                    mut freq: libc::c_float,
                                    mut speed: libc::c_float,
                                    mut segments: libc::c_int) {
    let mut i: libc::c_int =
        0; // Don't lose all of the noise/texture on short beams
    let mut j: libc::c_int = 0; // texture length texels per space pixel
    let mut noiseIndex: libc::c_int = 0;
    let mut noiseStep: libc::c_int = 0;
    let mut div: libc::c_float = 0.;
    let mut length: libc::c_float = 0.;
    let mut fraction: libc::c_float = 0.;
    let mut factor: libc::c_float = 0.;
    let mut vLast: libc::c_float = 0.;
    let mut vStep: libc::c_float = 0.;
    let mut last1: vec3_t = [0.; 3];
    let mut last2: vec3_t = [0.; 3];
    let mut point: vec3_t = [0.; 3];
    let mut screen: vec3_t = [0.; 3];
    let mut screenLast: vec3_t = [0.; 3];
    let mut tmp: vec3_t = [0.; 3];
    let mut normal: vec3_t = [0.; 3];
    let mut center: vec3_t = [0.; 3];
    let mut xaxis: vec3_t = [0.; 3];
    let mut yaxis: vec3_t = [0.; 3];
    let mut radius: libc::c_float = 0.;
    let mut x: libc::c_float = 0.;
    let mut y: libc::c_float = 0.;
    let mut scale: libc::c_float = 0.;
    if segments < 2 as libc::c_int { return }
    screenLast[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    screenLast[1 as libc::c_int as usize] =
        screenLast[2 as libc::c_int as usize];
    screenLast[0 as libc::c_int as usize] =
        screenLast[1 as libc::c_int as usize];
    segments =
        (segments as libc::c_float *
             3.14159265358979323846f64 as libc::c_float) as libc::c_int;
    if segments > 64 as libc::c_int * 8 as libc::c_int {
        segments = 64 as libc::c_int * 8 as libc::c_int
    }
    length =
        __tg_sqrt(*delta.offset(0 as libc::c_int as isize) *
                      *delta.offset(0 as libc::c_int as isize) +
                      *delta.offset(1 as libc::c_int as isize) *
                          *delta.offset(1 as libc::c_int as isize) +
                      *delta.offset(2 as libc::c_int as isize) *
                          *delta.offset(2 as libc::c_int as isize)) * 0.01f32
            * 3.14159265358979323846f64 as libc::c_float;
    if length < 0.5f32 { length = 0.5f32 }
    div = 1.0f32 / (segments - 1 as libc::c_int) as libc::c_float;
    vStep = length * div / 8.0f32;
    // Scroll speed 3.5 -- initial texture position, scrolls 3.5/sec (1.0 is entire texture)
    vLast = __tg_fmod(freq * speed, 1.0f32);
    scale = amplitude * length / 8.0f32;
    // Iterator to resample noise waveform (it needs to be generated in powers of 2)
    noiseStep =
        ((64 as libc::c_int - 1 as libc::c_int) as libc::c_float * div *
             65536.0f32) as libc::c_int * 8 as libc::c_int;
    noiseIndex = 0 as libc::c_int;
    *delta.offset(0 as libc::c_int as isize) =
        *delta.offset(0 as libc::c_int as isize) * 0.5f32;
    *delta.offset(1 as libc::c_int as isize) =
        *delta.offset(1 as libc::c_int as isize) * 0.5f32;
    *delta.offset(2 as libc::c_int as isize) =
        *delta.offset(2 as libc::c_int as isize) * 0.5f32;
    center[0 as libc::c_int as usize] =
        *source.offset(0 as libc::c_int as isize) +
            *delta.offset(0 as libc::c_int as isize);
    center[1 as libc::c_int as usize] =
        *source.offset(1 as libc::c_int as isize) +
            *delta.offset(1 as libc::c_int as isize);
    center[2 as libc::c_int as usize] =
        *source.offset(2 as libc::c_int as isize) +
            *delta.offset(2 as libc::c_int as isize);
    xaxis[0 as libc::c_int as usize] =
        *delta.offset(0 as libc::c_int as isize);
    xaxis[1 as libc::c_int as usize] =
        *delta.offset(1 as libc::c_int as isize);
    xaxis[2 as libc::c_int as usize] =
        *delta.offset(2 as libc::c_int as isize);
    radius =
        __tg_sqrt(xaxis[0 as libc::c_int as usize] *
                      xaxis[0 as libc::c_int as usize] +
                      xaxis[1 as libc::c_int as usize] *
                          xaxis[1 as libc::c_int as usize] +
                      xaxis[2 as libc::c_int as usize] *
                          xaxis[2 as libc::c_int as usize]);
    // cull beamring
	// --------------------------------
	// Compute box center +/- radius
    last1[0 as libc::c_int as usize] = radius; // maxs
    last1[1 as libc::c_int as usize] = radius; // mins
    last1[2 as libc::c_int as usize] = scale;
    tmp[0 as libc::c_int as usize] =
        center[0 as libc::c_int as usize] + last1[0 as libc::c_int as usize];
    tmp[1 as libc::c_int as usize] =
        center[1 as libc::c_int as usize] + last1[1 as libc::c_int as usize];
    tmp[2 as libc::c_int as usize] =
        center[2 as libc::c_int as usize] + last1[2 as libc::c_int as usize];
    screen[0 as libc::c_int as usize] =
        center[0 as libc::c_int as usize] - last1[0 as libc::c_int as usize];
    screen[1 as libc::c_int as usize] =
        center[1 as libc::c_int as usize] - last1[1 as libc::c_int as usize];
    screen[2 as libc::c_int as usize] =
        center[2 as libc::c_int as usize] - last1[2 as libc::c_int as usize];
    if gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1 as
                                                                            libc::c_int).is_null()
       {
        return
    }
    // is that box in PVS && frustum?
    if gEngfuncs.Mod_BoxVisible.expect("non-null function pointer")(screen.as_mut_ptr()
                                                                        as
                                                                        *const vec_t,
                                                                    tmp.as_mut_ptr()
                                                                        as
                                                                        *const vec_t,
                                                                    Mod_GetCurrentVis())
           as u64 == 0 ||
           R_CullBox(screen.as_mut_ptr() as *const vec_t,
                     tmp.as_mut_ptr() as *const vec_t) as libc::c_uint != 0 {
        return
    }
    yaxis[0 as libc::c_int as usize] = xaxis[1 as libc::c_int as usize];
    yaxis[1 as libc::c_int as usize] = -xaxis[0 as libc::c_int as usize];
    yaxis[2 as libc::c_int as usize] = 0.0f32;
    let mut ilength: libc::c_float =
        __tg_sqrt(yaxis[0 as libc::c_int as usize] *
                      yaxis[0 as libc::c_int as usize] +
                      yaxis[1 as libc::c_int as usize] *
                          yaxis[1 as libc::c_int as usize] +
                      yaxis[2 as libc::c_int as usize] *
                          yaxis[2 as libc::c_int as usize]);
    if ilength != 0. { ilength = 1.0f32 / ilength }
    yaxis[0 as libc::c_int as usize] *= ilength;
    yaxis[1 as libc::c_int as usize] *= ilength;
    yaxis[2 as libc::c_int as usize] *= ilength;
    yaxis[0 as libc::c_int as usize] =
        yaxis[0 as libc::c_int as usize] * radius;
    yaxis[1 as libc::c_int as usize] =
        yaxis[1 as libc::c_int as usize] * radius;
    yaxis[2 as libc::c_int as usize] =
        yaxis[2 as libc::c_int as usize] * radius;
    j = segments / 8 as libc::c_int;
    i = 0 as libc::c_int;
    while i < segments + 1 as libc::c_int {
        fraction = i as libc::c_float * div;
        SinCos(fraction *
                   (3.14159265358979323846f64 *
                        2 as libc::c_int as libc::c_double) as libc::c_float,
               &mut x, &mut y);
        point[0 as libc::c_int as usize] =
            x * xaxis[0 as libc::c_int as usize] +
                y * yaxis[0 as libc::c_int as usize] +
                1.0f32 * center[0 as libc::c_int as usize];
        point[1 as libc::c_int as usize] =
            x * xaxis[1 as libc::c_int as usize] +
                y * yaxis[1 as libc::c_int as usize] +
                1.0f32 * center[1 as libc::c_int as usize];
        point[2 as libc::c_int as usize] =
            x * xaxis[2 as libc::c_int as usize] +
                y * yaxis[2 as libc::c_int as usize] +
                1.0f32 * center[2 as libc::c_int as usize];
        // distort using noise
        factor =
            rgNoise[(noiseIndex >> 16 as libc::c_int &
                         64 as libc::c_int - 1 as libc::c_int) as usize] *
                scale;
        point[0 as libc::c_int as usize] =
            point[0 as libc::c_int as usize] +
                factor * RI.vup[0 as libc::c_int as usize];
        point[1 as libc::c_int as usize] =
            point[1 as libc::c_int as usize] +
                factor * RI.vup[1 as libc::c_int as usize];
        point[2 as libc::c_int as usize] =
            point[2 as libc::c_int as usize] +
                factor * RI.vup[2 as libc::c_int as usize];
        // Rotate the noise along the perpendicluar axis a bit to keep the bolt from looking diagonal
        factor =
            rgNoise[(noiseIndex >> 16 as libc::c_int &
                         64 as libc::c_int - 1 as libc::c_int) as usize] *
                scale;
        factor *=
            __tg_cos(fraction * 3.14159265358979323846f64 as libc::c_float *
                         24 as libc::c_int as libc::c_float + freq);
        point[0 as libc::c_int as usize] =
            point[0 as libc::c_int as usize] +
                factor * RI.vright[0 as libc::c_int as usize];
        point[1 as libc::c_int as usize] =
            point[1 as libc::c_int as usize] +
                factor * RI.vright[1 as libc::c_int as usize];
        point[2 as libc::c_int as usize] =
            point[2 as libc::c_int as usize] +
                factor * RI.vright[2 as libc::c_int as usize];
        // Transform point into screen space
        TriWorldToScreen(point.as_mut_ptr(), screen.as_mut_ptr());
        if i != 0 as libc::c_int {
            // build world-space normal to screen-space direction vector
            tmp[0 as libc::c_int as usize] =
                screen[0 as libc::c_int as usize] -
                    screenLast[0 as libc::c_int as usize];
            tmp[1 as libc::c_int as usize] =
                screen[1 as libc::c_int as usize] -
                    screenLast[1 as libc::c_int as usize];
            tmp[2 as libc::c_int as usize] =
                screen[2 as libc::c_int as usize] -
                    screenLast[2 as libc::c_int as usize];
            // we don't need Z, we're in screen space
            tmp[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            let mut ilength_0: libc::c_float =
                __tg_sqrt(tmp[0 as libc::c_int as usize] *
                              tmp[0 as libc::c_int as usize] +
                              tmp[1 as libc::c_int as usize] *
                                  tmp[1 as libc::c_int as usize] +
                              tmp[2 as libc::c_int as usize] *
                                  tmp[2 as libc::c_int as usize]);
            if ilength_0 != 0. { ilength_0 = 1.0f32 / ilength_0 }
            tmp[0 as libc::c_int as usize] *= ilength_0;
            tmp[1 as libc::c_int as usize] *= ilength_0;
            tmp[2 as libc::c_int as usize] *= ilength_0;
            // Build point along normal line (normal is -y, x)
            normal[0 as libc::c_int as usize] =
                RI.vup[0 as libc::c_int as usize] *
                    tmp[0 as libc::c_int as usize];
            normal[1 as libc::c_int as usize] =
                RI.vup[1 as libc::c_int as usize] *
                    tmp[0 as libc::c_int as usize];
            normal[2 as libc::c_int as usize] =
                RI.vup[2 as libc::c_int as usize] *
                    tmp[0 as libc::c_int as usize];
            normal[0 as libc::c_int as usize] =
                normal[0 as libc::c_int as usize] +
                    tmp[1 as libc::c_int as usize] *
                        RI.vright[0 as libc::c_int as usize];
            normal[1 as libc::c_int as usize] =
                normal[1 as libc::c_int as usize] +
                    tmp[1 as libc::c_int as usize] *
                        RI.vright[1 as libc::c_int as usize];
            normal[2 as libc::c_int as usize] =
                normal[2 as libc::c_int as usize] +
                    tmp[1 as libc::c_int as usize] *
                        RI.vright[2 as libc::c_int as usize];
            // Make a wide line
            last1[0 as libc::c_int as usize] =
                point[0 as libc::c_int as usize] +
                    width *
                        normal[0 as libc::c_int as
                                   usize]; // Advance texture scroll (v axis only)
            last1[1 as libc::c_int as usize] =
                point[1 as libc::c_int as usize] +
                    width * normal[1 as libc::c_int as usize];
            last1[2 as libc::c_int as usize] =
                point[2 as libc::c_int as usize] +
                    width * normal[2 as libc::c_int as usize];
            last2[0 as libc::c_int as usize] =
                point[0 as libc::c_int as usize] +
                    -width * normal[0 as libc::c_int as usize];
            last2[1 as libc::c_int as usize] =
                point[1 as libc::c_int as usize] +
                    -width * normal[1 as libc::c_int as usize];
            last2[2 as libc::c_int as usize] =
                point[2 as libc::c_int as usize] +
                    -width * normal[2 as libc::c_int as usize];
            vLast += vStep;
            TriTexCoord2f(1.0f32, vLast);
            TriVertex3fv(last2.as_mut_ptr());
            TriTexCoord2f(0.0f32, vLast);
            TriVertex3fv(last1.as_mut_ptr());
        }
        screenLast[0 as libc::c_int as usize] =
            screen[0 as libc::c_int as usize];
        screenLast[1 as libc::c_int as usize] =
            screen[1 as libc::c_int as usize];
        screenLast[2 as libc::c_int as usize] =
            screen[2 as libc::c_int as usize];
        noiseIndex += noiseStep;
        j -= 1;
        if j == 0 as libc::c_int &&
               amplitude != 0 as libc::c_int as libc::c_float {
            j = segments / 8 as libc::c_int;
            FracNoise(rgNoise.as_mut_ptr(), 64 as libc::c_int);
        }
        i += 1
    };
}
/*
==============
R_BeamComputePoint

compute attachment point for beam
==============
*/
unsafe extern "C" fn R_BeamComputePoint(mut beamEnt: libc::c_int,
                                        mut pt: *mut vec_t) -> qboolean {
    let mut ent: *mut cl_entity_t = 0 as *mut cl_entity_t;
    let mut attach: libc::c_int = 0;
    ent =
        gEngfuncs.R_BeamGetEntity.expect("non-null function pointer")(beamEnt);
    if beamEnt < 0 as libc::c_int {
        attach = -beamEnt >> 12 as libc::c_int & 0xf as libc::c_int
    } else { attach = beamEnt >> 12 as libc::c_int & 0xf as libc::c_int }
    if ent.is_null() {
        gEngfuncs.Con_DPrintf.expect("non-null function pointer")(b"^1Error:^7 R_BeamComputePoint: invalid entity %i\n\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char,
                                                                  beamEnt &
                                                                      0xfff as
                                                                          libc::c_int);
        let ref mut fresh6 = *pt.offset(2 as libc::c_int as isize);
        *fresh6 = 0 as libc::c_int as vec_t;
        let ref mut fresh7 = *pt.offset(1 as libc::c_int as isize);
        *fresh7 = *fresh6;
        *pt.offset(0 as libc::c_int as isize) = *fresh7;
        return false_0
    }
    // get attachment
    if attach > 0 as libc::c_int {
        *pt.offset(0 as libc::c_int as isize) =
            (*ent).attachment[(attach - 1 as libc::c_int) as
                                  usize][0 as libc::c_int as usize];
        *pt.offset(1 as libc::c_int as isize) =
            (*ent).attachment[(attach - 1 as libc::c_int) as
                                  usize][1 as libc::c_int as usize];
        *pt.offset(2 as libc::c_int as isize) =
            (*ent).attachment[(attach - 1 as libc::c_int) as
                                  usize][2 as libc::c_int as usize]
    } else if (*ent).index ==
                  Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(PARM_PLAYER_INDEX
                                                                                                                            as
                                                                                                                            libc::c_int,
                                                                                                                        0
                                                                                                                            as
                                                                                                                            libc::c_int)
     {
        let mut simorg: vec3_t = [0.; 3];
        gEngfuncs.GetPredictedOrigin.expect("non-null function pointer")(simorg.as_mut_ptr());
        *pt.offset(0 as libc::c_int as isize) =
            simorg[0 as libc::c_int as usize];
        *pt.offset(1 as libc::c_int as isize) =
            simorg[1 as libc::c_int as usize];
        *pt.offset(2 as libc::c_int as isize) =
            simorg[2 as libc::c_int as usize]
    } else {
        *pt.offset(0 as libc::c_int as isize) =
            (*ent).origin[0 as libc::c_int as usize];
        *pt.offset(1 as libc::c_int as isize) =
            (*ent).origin[1 as libc::c_int as usize];
        *pt.offset(2 as libc::c_int as isize) =
            (*ent).origin[2 as libc::c_int as usize]
    }
    return true_0;
}
/*
==============
R_BeamRecomputeEndpoints

Recomputes beam endpoints..
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_BeamRecomputeEndpoints(mut pbeam: *mut BEAM)
 -> qboolean {
    if (*pbeam).flags & 0x1 as libc::c_int != 0 {
        let mut start: *mut cl_entity_t =
            gEngfuncs.R_BeamGetEntity.expect("non-null function pointer")((*pbeam).startEntity);
        if R_BeamComputePoint((*pbeam).startEntity,
                              (*pbeam).source.as_mut_ptr()) as u64 != 0 {
            if (*pbeam).pFollowModel.is_null() {
                (*pbeam).pFollowModel = (*start).model
            }
            (*pbeam).flags = (*pbeam).flags | 0x10000000 as libc::c_int
        } else if (*pbeam).flags as libc::c_uint & 0x80000000 as libc::c_uint
                      == 0 {
            (*pbeam).flags = (*pbeam).flags & !(0x1 as libc::c_int)
        }
    }
    if (*pbeam).flags & 0x2 as libc::c_int != 0 {
        let mut end: *mut cl_entity_t =
            gEngfuncs.R_BeamGetEntity.expect("non-null function pointer")((*pbeam).endEntity);
        if R_BeamComputePoint((*pbeam).endEntity,
                              (*pbeam).target.as_mut_ptr()) as u64 != 0 {
            if (*pbeam).pFollowModel.is_null() {
                (*pbeam).pFollowModel = (*end).model
            }
            (*pbeam).flags = (*pbeam).flags | 0x20000000 as libc::c_int
        } else if (*pbeam).flags as libc::c_uint & 0x80000000 as libc::c_uint
                      == 0 {
            (*pbeam).flags = (*pbeam).flags & !(0x2 as libc::c_int);
            (*pbeam).die = (*gpGlobals).time;
            return false_0
        } else { return false_0 }
    }
    if (*pbeam).flags & 0x1 as libc::c_int != 0 &&
           (*pbeam).flags & 0x10000000 as libc::c_int == 0 {
        return false_0
    }
    return true_0;
}
/*
==============
R_BeamDraw

Update beam vars and draw it
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_BeamDraw(mut pbeam: *mut BEAM,
                                    mut frametime: libc::c_float) {
    let mut model: *mut model_t = 0 as *mut model_t; // force to ignore
    let mut delta: vec3_t = [0.; 3];
    model =
        gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")((*pbeam).modelIndex);
    (*pbeam).flags = (*pbeam).flags | 0x40000000 as libc::c_int;
    if model.is_null() ||
           (*model).type_0 as libc::c_int != mod_sprite as libc::c_int {
        (*pbeam).flags &= !(0x40000000 as libc::c_int);
        (*pbeam).die = (*gpGlobals).time;
        return
    }
    // update frequency
    (*pbeam).freq += frametime;
    // generate fractal noise
    if frametime != 0.0f32 {
        rgNoise[0 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_float;
        rgNoise[64 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_float
    }
    if (*pbeam).amplitude != 0 as libc::c_int as libc::c_float &&
           frametime != 0.0f32 {
        if (*pbeam).flags & 0x10 as libc::c_int != 0 {
            SineNoise(rgNoise.as_mut_ptr(), 64 as libc::c_int);
        } else { FracNoise(rgNoise.as_mut_ptr(), 64 as libc::c_int); }
    }
    // update end points
    if (*pbeam).flags & (0x1 as libc::c_int | 0x2 as libc::c_int) != 0 {
        // makes sure attachment[0] + attachment[1] are valid
        if R_BeamRecomputeEndpoints(pbeam) as u64 == 0 {
            (*pbeam).flags =
                (*pbeam).flags &
                    !(0x40000000 as libc::c_int); // force to ignore
            return
        }
        // one per 16 pixels
        delta[0 as libc::c_int as usize] =
            (*pbeam).target[0 as libc::c_int as usize] -
                (*pbeam).source[0 as libc::c_int as usize];
        delta[1 as libc::c_int as usize] =
            (*pbeam).target[1 as libc::c_int as usize] -
                (*pbeam).source[1 as libc::c_int as usize];
        delta[2 as libc::c_int as usize] =
            (*pbeam).target[2 as libc::c_int as usize] -
                (*pbeam).source[2 as libc::c_int as usize];
        (*pbeam).delta[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
        (*pbeam).delta[1 as libc::c_int as usize] =
            (*pbeam).delta[2 as libc::c_int as usize];
        (*pbeam).delta[0 as libc::c_int as usize] =
            (*pbeam).delta[1 as libc::c_int as usize];
        if __tg_sqrt(delta[0 as libc::c_int as usize] *
                         delta[0 as libc::c_int as usize] +
                         delta[1 as libc::c_int as usize] *
                             delta[1 as libc::c_int as usize] +
                         delta[2 as libc::c_int as usize] *
                             delta[2 as libc::c_int as usize]) > 0.0000001f32
           {
            (*pbeam).delta[0 as libc::c_int as usize] =
                delta[0 as libc::c_int as usize];
            (*pbeam).delta[1 as libc::c_int as usize] =
                delta[1 as libc::c_int as usize];
            (*pbeam).delta[2 as libc::c_int as usize] =
                delta[2 as libc::c_int as usize]
        }
        if (*pbeam).amplitude >= 0.50f32 {
            // compute segments from the new endpoints
            (*pbeam).segments =
                (__tg_sqrt((*pbeam).delta[0 as libc::c_int as usize] *
                               (*pbeam).delta[0 as libc::c_int as usize] +
                               (*pbeam).delta[1 as libc::c_int as usize] *
                                   (*pbeam).delta[1 as libc::c_int as usize] +
                               (*pbeam).delta[2 as libc::c_int as usize] *
                                   (*pbeam).delta[2 as libc::c_int as usize])
                     * 0.25f32 + 3.0f32) as libc::c_int
        } else {
            (*pbeam).segments =
                (__tg_sqrt((*pbeam).delta[0 as libc::c_int as usize] *
                               (*pbeam).delta[0 as libc::c_int as usize] +
                               (*pbeam).delta[1 as libc::c_int as usize] *
                                   (*pbeam).delta[1 as libc::c_int as usize] +
                               (*pbeam).delta[2 as libc::c_int as usize] *
                                   (*pbeam).delta[2 as libc::c_int as usize])
                     * 0.075f32 + 3.0f32) as libc::c_int
        }
    } // one per 4 pixels
    if (*pbeam).type_0 == 0 as libc::c_int &&
           R_BeamCull((*pbeam).source.as_mut_ptr() as *const vec_t,
                      (*pbeam).target.as_mut_ptr() as *const vec_t, false_0)
               as libc::c_uint != 0 {
        (*pbeam).flags = (*pbeam).flags & !(0x40000000 as libc::c_int);
        return
    }
    // don't draw really short or inactive beams
    if (*pbeam).flags & 0x40000000 as libc::c_int == 0 ||
           __tg_sqrt((*pbeam).delta[0 as libc::c_int as usize] *
                         (*pbeam).delta[0 as libc::c_int as usize] +
                         (*pbeam).delta[1 as libc::c_int as usize] *
                             (*pbeam).delta[1 as libc::c_int as usize] +
                         (*pbeam).delta[2 as libc::c_int as usize] *
                             (*pbeam).delta[2 as libc::c_int as usize]) <
               0.1f32 {
        return
    }
    if (*pbeam).flags & (0x4 as libc::c_int | 0x8 as libc::c_int) != 0 {
        // update life cycle
        (*pbeam).t = (*pbeam).freq + ((*pbeam).die - (*gpGlobals).time);
        if (*pbeam).t != 0.0f32 {
            (*pbeam).t = 1.0f32 - (*pbeam).freq / (*pbeam).t
        }
    }
    if (*pbeam).type_0 == 26 as libc::c_int {
        let mut flDot: libc::c_float = 0.;
        delta[0 as libc::c_int as usize] =
            (*pbeam).target[0 as libc::c_int as usize] -
                (*pbeam).source[0 as libc::c_int as usize];
        delta[1 as libc::c_int as usize] =
            (*pbeam).target[1 as libc::c_int as usize] -
                (*pbeam).source[1 as libc::c_int as usize];
        delta[2 as libc::c_int as usize] =
            (*pbeam).target[2 as libc::c_int as usize] -
                (*pbeam).source[2 as libc::c_int as usize];
        let mut ilength: libc::c_float =
            __tg_sqrt(delta[0 as libc::c_int as usize] *
                          delta[0 as libc::c_int as usize] +
                          delta[1 as libc::c_int as usize] *
                              delta[1 as libc::c_int as usize] +
                          delta[2 as libc::c_int as usize] *
                              delta[2 as libc::c_int as usize]);
        if ilength != 0. { ilength = 1.0f32 / ilength }
        delta[0 as libc::c_int as usize] *= ilength;
        delta[1 as libc::c_int as usize] *= ilength;
        delta[2 as libc::c_int as usize] *= ilength;
        flDot =
            delta[0 as libc::c_int as usize] *
                RI.vforward[0 as libc::c_int as usize] +
                delta[1 as libc::c_int as usize] *
                    RI.vforward[1 as libc::c_int as usize] +
                delta[2 as libc::c_int as usize] *
                    RI.vforward[2 as libc::c_int as usize];
        // abort if the player's looking along it away from the source
        if flDot > 0 as libc::c_int as libc::c_float {
            return
        } else {
            let mut flFade: libc::c_float =
                __tg_pow(flDot as libc::c_double,
                         10 as libc::c_int as libc::c_double) as
                    libc::c_float;
            let mut localDir: vec3_t = [0.; 3];
            let mut vecProjection: vec3_t = [0.; 3];
            let mut tmp: vec3_t = [0.; 3];
            let mut flDistance: libc::c_float = 0.;
            // fade the beam if the player's not looking at the source
            localDir[0 as libc::c_int as usize] =
                RI.vieworg[0 as libc::c_int as usize] -
                    (*pbeam).source[0 as libc::c_int as usize];
            localDir[1 as libc::c_int as usize] =
                RI.vieworg[1 as libc::c_int as usize] -
                    (*pbeam).source[1 as libc::c_int as usize];
            localDir[2 as libc::c_int as usize] =
                RI.vieworg[2 as libc::c_int as usize] -
                    (*pbeam).source[2 as libc::c_int as usize];
            flDot =
                delta[0 as libc::c_int as usize] *
                    localDir[0 as libc::c_int as usize] +
                    delta[1 as libc::c_int as usize] *
                        localDir[1 as libc::c_int as usize] +
                    delta[2 as libc::c_int as usize] *
                        localDir[2 as libc::c_int as usize];
            vecProjection[0 as libc::c_int as usize] =
                delta[0 as libc::c_int as usize] * flDot;
            vecProjection[1 as libc::c_int as usize] =
                delta[1 as libc::c_int as usize] * flDot;
            vecProjection[2 as libc::c_int as usize] =
                delta[2 as libc::c_int as usize] * flDot;
            tmp[0 as libc::c_int as usize] =
                localDir[0 as libc::c_int as usize] -
                    vecProjection[0 as libc::c_int as usize];
            tmp[1 as libc::c_int as usize] =
                localDir[1 as libc::c_int as usize] -
                    vecProjection[1 as libc::c_int as usize];
            tmp[2 as libc::c_int as usize] =
                localDir[2 as libc::c_int as usize] -
                    vecProjection[2 as libc::c_int as usize];
            flDistance =
                __tg_sqrt(tmp[0 as libc::c_int as usize] *
                              tmp[0 as libc::c_int as usize] +
                              tmp[1 as libc::c_int as usize] *
                                  tmp[1 as libc::c_int as usize] +
                              tmp[2 as libc::c_int as usize] *
                                  tmp[2 as libc::c_int as usize]);
            if flDistance > 30 as libc::c_int as libc::c_float {
                flDistance = 1.0f32 - (flDistance - 30.0f32) / 64.0f32;
                if flDistance <= 0 as libc::c_int as libc::c_float {
                    flFade = 0 as libc::c_int as libc::c_float
                } else {
                    flFade =
                        (flFade as libc::c_double *
                             __tg_pow(flDistance as libc::c_double,
                                      3 as libc::c_int as libc::c_double)) as
                            libc::c_float
                }
            }
            if flFade < 1.0f32 / 255.0f32 { return }
            // FIXME: needs to be testing
            (*pbeam).brightness *= flFade
        }
    }
    TriRenderMode(if (*pbeam).flags & 0x20 as libc::c_int != 0 {
                      kRenderNormal as libc::c_int
                  } else { kRenderTransAdd as libc::c_int });
    if TriSpriteTexture(model,
                        ((*pbeam).frame +
                             (*pbeam).frameRate * (*gpGlobals).time) as
                            libc::c_int % (*pbeam).frameCount) == 0 {
        (*pbeam).flags = (*pbeam).flags & !(0x40000000 as libc::c_int);
        return
    }
    if (*pbeam).type_0 == 22 as libc::c_int {
        let mut pStart: *mut cl_entity_t = 0 as *mut cl_entity_t;
        // XASH SPECIFIC: get brightness from head entity
        pStart =
            gEngfuncs.R_BeamGetEntity.expect("non-null function pointer")((*pbeam).startEntity);
        if !pStart.is_null() &&
               (*pStart).curstate.rendermode != kRenderNormal as libc::c_int {
            (*pbeam).brightness =
                CL_FxBlend(pStart) as libc::c_float / 255.0f32
        }
    }
    if (*pbeam).flags & 0x4 as libc::c_int != 0 {
        TriColor4f((*pbeam).r, (*pbeam).g, (*pbeam).b,
                   (*pbeam).t * (*pbeam).brightness);
    } else if (*pbeam).flags & 0x8 as libc::c_int != 0 {
        TriColor4f((*pbeam).r, (*pbeam).g, (*pbeam).b,
                   (1.0f32 - (*pbeam).t) * (*pbeam).brightness);
    } else {
        TriColor4f((*pbeam).r, (*pbeam).g, (*pbeam).b, (*pbeam).brightness);
    }
    match (*pbeam).type_0 {
        19 => {
            GL_Cull(0 as libc::c_int as GLenum);
            TriBegin(5 as libc::c_int);
            R_DrawTorus((*pbeam).source.as_mut_ptr(),
                        (*pbeam).delta.as_mut_ptr(), (*pbeam).width,
                        (*pbeam).amplitude, (*pbeam).freq, (*pbeam).speed,
                        (*pbeam).segments);
            TriEnd();
        }
        20 => {
            GL_Cull(0 as libc::c_int as GLenum);
            TriBegin(5 as libc::c_int);
            R_DrawDisk((*pbeam).source.as_mut_ptr(),
                       (*pbeam).delta.as_mut_ptr(), (*pbeam).width,
                       (*pbeam).amplitude, (*pbeam).freq, (*pbeam).speed,
                       (*pbeam).segments);
            TriEnd();
        }
        21 => {
            GL_Cull(0 as libc::c_int as GLenum);
            TriBegin(5 as libc::c_int);
            R_DrawCylinder((*pbeam).source.as_mut_ptr(),
                           (*pbeam).delta.as_mut_ptr(), (*pbeam).width,
                           (*pbeam).amplitude, (*pbeam).freq, (*pbeam).speed,
                           (*pbeam).segments);
            TriEnd();
        }
        0 | 26 => {
            TriBegin(5 as libc::c_int);
            R_DrawSegs((*pbeam).source.as_mut_ptr(),
                       (*pbeam).delta.as_mut_ptr(), (*pbeam).width,
                       (*pbeam).amplitude, (*pbeam).freq, (*pbeam).speed,
                       (*pbeam).segments, (*pbeam).flags);
            TriEnd();
        }
        22 => {
            TriBegin(2 as libc::c_int);
            R_DrawBeamFollow(pbeam, frametime);
            TriEnd();
        }
        24 => {
            GL_Cull(0 as libc::c_int as GLenum);
            TriBegin(5 as libc::c_int);
            R_DrawRing((*pbeam).source.as_mut_ptr(),
                       (*pbeam).delta.as_mut_ptr(), (*pbeam).width,
                       (*pbeam).amplitude, (*pbeam).freq, (*pbeam).speed,
                       (*pbeam).segments);
            TriEnd();
        }
        _ => { }
    }
    GL_Cull(0x404 as libc::c_int as GLenum);
    r_stats.c_view_beams_count = r_stats.c_view_beams_count.wrapping_add(1);
}
/*
==============
R_BeamSetAttributes

set beam attributes
==============
*/
unsafe extern "C" fn R_BeamSetAttributes(mut pbeam: *mut BEAM,
                                         mut r: libc::c_float,
                                         mut g: libc::c_float,
                                         mut b: libc::c_float,
                                         mut framerate: libc::c_float,
                                         mut startFrame: libc::c_int) {
    (*pbeam).frame = startFrame as libc::c_float;
    (*pbeam).frameRate = framerate;
    (*pbeam).r = r;
    (*pbeam).g = g;
    (*pbeam).b = b;
}
/*
==============
R_BeamSetup

generic function. all beams must be
passed through this
==============
*/
unsafe extern "C" fn R_BeamSetup(mut pbeam: *mut BEAM, mut start: *mut vec_t,
                                 mut end: *mut vec_t,
                                 mut modelIndex: libc::c_int,
                                 mut life: libc::c_float,
                                 mut width: libc::c_float,
                                 mut amplitude: libc::c_float,
                                 mut brightness: libc::c_float,
                                 mut speed: libc::c_float) {
    let mut sprite: *mut model_t =
        gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(modelIndex); // one per 16 pixels
    if sprite.is_null() { return } // one per 4 pixels
    (*pbeam).type_0 = BEAM_POINTS as libc::c_int;
    (*pbeam).modelIndex = modelIndex;
    (*pbeam).frame = 0 as libc::c_int as libc::c_float;
    (*pbeam).frameRate = 0 as libc::c_int as libc::c_float;
    (*pbeam).frameCount = (*sprite).numframes;
    (*pbeam).source[0 as libc::c_int as usize] =
        *start.offset(0 as libc::c_int as isize);
    (*pbeam).source[1 as libc::c_int as usize] =
        *start.offset(1 as libc::c_int as isize);
    (*pbeam).source[2 as libc::c_int as usize] =
        *start.offset(2 as libc::c_int as isize);
    (*pbeam).target[0 as libc::c_int as usize] =
        *end.offset(0 as libc::c_int as isize);
    (*pbeam).target[1 as libc::c_int as usize] =
        *end.offset(1 as libc::c_int as isize);
    (*pbeam).target[2 as libc::c_int as usize] =
        *end.offset(2 as libc::c_int as isize);
    (*pbeam).delta[0 as libc::c_int as usize] =
        *end.offset(0 as libc::c_int as isize) -
            *start.offset(0 as libc::c_int as isize);
    (*pbeam).delta[1 as libc::c_int as usize] =
        *end.offset(1 as libc::c_int as isize) -
            *start.offset(1 as libc::c_int as isize);
    (*pbeam).delta[2 as libc::c_int as usize] =
        *end.offset(2 as libc::c_int as isize) -
            *start.offset(2 as libc::c_int as isize);
    (*pbeam).freq = speed * (*gpGlobals).time;
    (*pbeam).die = life + (*gpGlobals).time;
    (*pbeam).amplitude = amplitude;
    (*pbeam).brightness = brightness;
    (*pbeam).width = width;
    (*pbeam).speed = speed;
    if amplitude >= 0.50f32 {
        (*pbeam).segments =
            (__tg_sqrt((*pbeam).delta[0 as libc::c_int as usize] *
                           (*pbeam).delta[0 as libc::c_int as usize] +
                           (*pbeam).delta[1 as libc::c_int as usize] *
                               (*pbeam).delta[1 as libc::c_int as usize] +
                           (*pbeam).delta[2 as libc::c_int as usize] *
                               (*pbeam).delta[2 as libc::c_int as usize]) *
                 0.25f32 + 3.0f32) as libc::c_int
    } else {
        (*pbeam).segments =
            (__tg_sqrt((*pbeam).delta[0 as libc::c_int as usize] *
                           (*pbeam).delta[0 as libc::c_int as usize] +
                           (*pbeam).delta[1 as libc::c_int as usize] *
                               (*pbeam).delta[1 as libc::c_int as usize] +
                           (*pbeam).delta[2 as libc::c_int as usize] *
                               (*pbeam).delta[2 as libc::c_int as usize]) *
                 0.075f32 + 3.0f32) as libc::c_int
    }
    (*pbeam).pFollowModel = 0 as *mut model_s;
    (*pbeam).flags = 0 as libc::c_int;
}
/*
==============
R_BeamDrawCustomEntity

initialize beam from server entity
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_BeamDrawCustomEntity(mut ent: *mut cl_entity_t) {
    let mut beam: BEAM =
        BEAM{next: 0 as *mut BEAM,
             type_0: 0,
             flags: 0,
             source: [0.; 3],
             target: [0.; 3],
             delta: [0.; 3],
             t: 0.,
             freq: 0.,
             die: 0.,
             width: 0.,
             amplitude: 0.,
             r: 0.,
             g: 0.,
             b: 0.,
             brightness: 0.,
             speed: 0.,
             frameRate: 0.,
             frame: 0.,
             segments: 0,
             startEntity: 0,
             endEntity: 0,
             modelIndex: 0,
             frameCount: 0,
             pFollowModel: 0 as *mut model_s,
             particles: 0 as *mut particle_s,};
    let mut amp: libc::c_float =
        (*ent).curstate.body as libc::c_float / 100.0f32;
    let mut blend: libc::c_float =
        CL_FxBlend(ent) as libc::c_float / 255.0f32;
    let mut r: libc::c_float = 0.;
    let mut g: libc::c_float = 0.;
    let mut b: libc::c_float = 0.;
    let mut beamFlags: libc::c_int = 0;
    r =
        (*ent).curstate.rendercolor.r as libc::c_int as libc::c_float /
            255.0f32;
    g =
        (*ent).curstate.rendercolor.g as libc::c_int as libc::c_float /
            255.0f32;
    b =
        (*ent).curstate.rendercolor.b as libc::c_int as libc::c_float /
            255.0f32;
    R_BeamSetup(&mut beam, (*ent).origin.as_mut_ptr(),
                (*ent).angles.as_mut_ptr(), (*ent).curstate.modelindex,
                0 as libc::c_int as libc::c_float, (*ent).curstate.scale, amp,
                blend, (*ent).curstate.animtime);
    R_BeamSetAttributes(&mut beam, r, g, b, (*ent).curstate.framerate,
                        (*ent).curstate.frame as libc::c_int);
    beam.pFollowModel = 0 as *mut model_s;
    match (*ent).curstate.rendermode & 0xf as libc::c_int {
        1 => {
            beam.type_0 = 0 as libc::c_int;
            if (*ent).curstate.sequence != 0 {
                beam.flags = beam.flags | 0x1 as libc::c_int;
                beam.startEntity = (*ent).curstate.sequence
            }
            if (*ent).curstate.skin != 0 {
                beam.flags = beam.flags | 0x2 as libc::c_int;
                beam.endEntity = (*ent).curstate.skin as libc::c_int
            }
        }
        2 => {
            beam.type_0 = 0 as libc::c_int;
            beam.flags =
                beam.flags | (0x1 as libc::c_int | 0x2 as libc::c_int);
            beam.startEntity = (*ent).curstate.sequence;
            beam.endEntity = (*ent).curstate.skin as libc::c_int
        }
        3 => { beam.type_0 = 26 as libc::c_int }
        0 | _ => { }
    }
    beamFlags = (*ent).curstate.rendermode & 0xf0 as libc::c_int;
    if beamFlags & 0x10 as libc::c_int != 0 {
        beam.flags = beam.flags | 0x10 as libc::c_int
    }
    if beamFlags & 0x20 as libc::c_int != 0 {
        beam.flags = beam.flags | 0x20 as libc::c_int
    }
    if beamFlags & 0x40 as libc::c_int != 0 {
        beam.flags = beam.flags | 0x40 as libc::c_int
    }
    if beamFlags & 0x80 as libc::c_int != 0 {
        beam.flags = beam.flags | 0x80 as libc::c_int
    }
    // draw it
    R_BeamDraw(&mut beam, tr.frametime as libc::c_float);
}
/*
==============
CL_DrawBeams

draw beam loop
==============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_DrawBeams(mut fTrans: libc::c_int,
                                      mut active_beams: *mut BEAM) {
    let mut pBeam: *mut BEAM = 0 as *mut BEAM;
    let mut i: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    pglShadeModel.expect("non-null function pointer")(0x1d01 as libc::c_int as
                                                          GLenum);
    pglDepthMask.expect("non-null function pointer")(if fTrans != 0 {
                                                         0 as libc::c_int
                                                     } else {
                                                         0x1 as libc::c_int
                                                     } as GLboolean);
    // server beams don't allocate beam chains
	// all params are stored in cl_entity_t
    i = 0 as libc::c_int;
    while (i as libc::c_uint) < (*tr.draw_list).num_beam_entities {
        RI.currentbeam = (*tr.draw_list).beam_entities[i as usize];
        flags = (*RI.currentbeam).curstate.rendermode & 0xf0 as libc::c_int;
        if !(fTrans != 0 && flags & 0x20 as libc::c_int != 0) {
            if !(fTrans == 0 && flags & 0x20 as libc::c_int == 0) {
                R_BeamDrawCustomEntity(RI.currentbeam);
                r_stats.c_view_beams_count =
                    r_stats.c_view_beams_count.wrapping_add(1)
            }
        }
        i += 1
    }
    RI.currentbeam = 0 as *mut cl_entity_t;
    // draw temporary entity beams
    pBeam = active_beams;
    while !pBeam.is_null() {
        if !(fTrans != 0 && (*pBeam).flags & 0x20 as libc::c_int != 0) {
            if !(fTrans == 0 && (*pBeam).flags & 0x20 as libc::c_int == 0) {
                R_BeamDraw(pBeam, (*gpGlobals).time - (*gpGlobals).oldtime);
            }
        }
        pBeam = (*pBeam).next
    }
    pglShadeModel.expect("non-null function pointer")(0x1d00 as libc::c_int as
                                                          GLenum);
    pglDepthMask.expect("non-null function pointer")(0x1 as libc::c_int as
                                                         GLboolean);
}
