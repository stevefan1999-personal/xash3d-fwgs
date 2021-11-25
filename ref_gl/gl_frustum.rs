#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type grasshdr_s;
    pub type pmtrace_s;
    pub type physent_s;
    pub type engine_studio_api_s;
    pub type r_studio_interface_s;
    pub type client_textmessage_s;
    pub type screenfade_s;
    pub type world_static_s;
    pub type con_nprint_s;
    #[no_mangle]
    fn sqrtf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    static mut RI: ref_instance_t;
    #[no_mangle]
    fn SignbitsForPlane(normal: *const vec_t) -> libc::c_int;
    #[no_mangle]
    fn PlaneTypeForNormal(normal: *const vec_t) -> libc::c_int;
    #[no_mangle]
    static mut gEngfuncs: ref_api_t;
    #[no_mangle]
    fn SinCos(radians: libc::c_float, sine: *mut libc::c_float,
              cosine: *mut libc::c_float);
    #[no_mangle]
    fn AddPointToBounds(v: *const vec_t, mins: *mut vec_t, maxs: *mut vec_t);
    #[no_mangle]
    fn ClearBounds(mins: *mut vec_t, maxs: *mut vec_t);
    #[no_mangle]
    fn PlanesGetIntersectionPoint(plane1: *const mplane_s,
                                  plane2: *const mplane_s,
                                  plane3: *const mplane_s, out: *mut vec_t)
     -> qboolean;
    #[no_mangle]
    static mut pglEnable: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglEnd: Option<unsafe extern "C" fn() -> ()>;
    #[no_mangle]
    static mut pglVertex3fv:
           Option<unsafe extern "C" fn(_: *const GLfloat) -> ()>;
    #[no_mangle]
    static mut pglBegin: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglDisable: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglColor4f:
           Option<unsafe extern "C" fn(_: GLfloat, _: GLfloat, _: GLfloat,
                                       _: GLfloat) -> ()>;
    #[no_mangle]
    static mut r_nocull: *mut cvar_t;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
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
pub type ref_api_t = ref_api_s;
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
pub struct bpc_desc_s {
    pub format: libc::c_int,
    pub name: [libc::c_char; 16],
    pub glFormat: uint,
    pub bpp: libc::c_int,
}
pub type rgbdata_t = rgbdata_s;
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
pub struct remap_info_s {
    pub textures: [libc::c_ushort; 32],
    pub ptexture: *mut mstudiotex_s,
    pub numtextures: libc::c_short,
    pub topcolor: libc::c_short,
    pub bottomcolor: libc::c_short,
    pub model: *mut model_t,
}
pub type ref_defaultsprite_e = libc::c_uint;
pub const REF_CHROME_SPRITE: ref_defaultsprite_e = 1;
pub const REF_DOT_SPRITE: ref_defaultsprite_e = 0;
pub type particle_t = particle_s;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mstudioevent_s {
    pub frame: int32_t,
    pub event: int32_t,
    pub unused: int32_t,
    pub options: [libc::c_char; 64],
}
pub type mstudioseqdesc_t = mstudioseqdesc_s;
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
pub type studiohdr_t = studiohdr_s;
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
pub type mstudioanim_t = mstudioanim_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mstudioanim_s {
    pub offset: [uint16_t; 6],
}
pub type mstudiobone_t = mstudiobone_s;
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
pub type GLenum = uint;
pub type GLfloat = libc::c_float;
#[inline(always)]
unsafe extern "C" fn __tg_sqrt(mut __x: libc::c_float) -> libc::c_float {
    return sqrtf(__x);
}
/*
gl_frustum.cpp - frustum test implementation
Copyright (C) 2016 Uncle Mike

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
pub unsafe extern "C" fn GL_FrustumEnablePlane(mut out: *mut gl_frustum_t,
                                               mut side: libc::c_int) {
    if !(side >= 0 as libc::c_int && side < 6 as libc::c_int) {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 b"../ref_gl/gl_frustum.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 21 as
                                                                     libc::c_int);
    }
    // make sure what plane is ready
    if !((*out).planes[side as usize].normal[0 as libc::c_int as usize] ==
             0.0f32 &&
             (*out).planes[side as usize].normal[1 as libc::c_int as usize] ==
                 0.0f32 &&
             (*out).planes[side as usize].normal[2 as libc::c_int as usize] ==
                 0.0f32) {
        (*out).clipFlags = (*out).clipFlags | (1 as libc::c_uint) << side
    };
}
#[no_mangle]
pub unsafe extern "C" fn GL_FrustumDisablePlane(mut out: *mut gl_frustum_t,
                                                mut side: libc::c_int) {
    if !(side >= 0 as libc::c_int && side < 6 as libc::c_int) {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 b"../ref_gl/gl_frustum.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 30 as
                                                                     libc::c_int);
    }
    (*out).clipFlags = (*out).clipFlags & !((1 as libc::c_uint) << side);
}
#[no_mangle]
pub unsafe extern "C" fn GL_FrustumSetPlane(mut out: *mut gl_frustum_t,
                                            mut side: libc::c_int,
                                            mut vecNormal: *const vec_t,
                                            mut flDist: libc::c_float) {
    if !(side >= 0 as libc::c_int && side < 6 as libc::c_int) {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 b"../ref_gl/gl_frustum.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 36 as
                                                                     libc::c_int);
    }
    (*out).planes[side as usize].type_0 =
        PlaneTypeForNormal(vecNormal) as byte;
    (*out).planes[side as usize].signbits =
        SignbitsForPlane(vecNormal) as byte;
    (*out).planes[side as usize].normal[0 as libc::c_int as usize] =
        *vecNormal.offset(0 as libc::c_int as isize);
    (*out).planes[side as usize].normal[1 as libc::c_int as usize] =
        *vecNormal.offset(1 as libc::c_int as isize);
    (*out).planes[side as usize].normal[2 as libc::c_int as usize] =
        *vecNormal.offset(2 as libc::c_int as isize);
    (*out).planes[side as usize].dist = flDist;
    (*out).clipFlags = (*out).clipFlags | (1 as libc::c_uint) << side;
}
#[no_mangle]
pub unsafe extern "C" fn GL_FrustumNormalizePlane(mut out: *mut gl_frustum_t,
                                                  mut side: libc::c_int) {
    let mut length: libc::c_float = 0.;
    if !(side >= 0 as libc::c_int && side < 6 as libc::c_int) {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 b"../ref_gl/gl_frustum.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 50 as
                                                                     libc::c_int);
    }
    // normalize
    length =
        __tg_sqrt((*out).planes[side as
                                    usize].normal[0 as libc::c_int as usize] *
                      (*out).planes[side as
                                        usize].normal[0 as libc::c_int as
                                                          usize] +
                      (*out).planes[side as
                                        usize].normal[1 as libc::c_int as
                                                          usize] *
                          (*out).planes[side as
                                            usize].normal[1 as libc::c_int as
                                                              usize] +
                      (*out).planes[side as
                                        usize].normal[2 as libc::c_int as
                                                          usize] *
                          (*out).planes[side as
                                            usize].normal[2 as libc::c_int as
                                                              usize]);
    if length != 0. {
        let mut ilength: libc::c_float = 1.0f32 / length;
        (*out).planes[side as usize].normal[0 as libc::c_int as usize] *=
            ilength;
        (*out).planes[side as usize].normal[1 as libc::c_int as usize] *=
            ilength;
        (*out).planes[side as usize].normal[2 as libc::c_int as usize] *=
            ilength;
        (*out).planes[side as usize].dist *= ilength
    }
    (*out).planes[side as usize].type_0 =
        PlaneTypeForNormal((*out).planes[side as usize].normal.as_mut_ptr() as
                               *const vec_t) as byte;
    (*out).planes[side as usize].signbits =
        SignbitsForPlane((*out).planes[side as usize].normal.as_mut_ptr() as
                             *const vec_t) as byte;
    (*out).clipFlags = (*out).clipFlags | (1 as libc::c_uint) << side;
}
#[no_mangle]
pub unsafe extern "C" fn GL_FrustumInitProj(mut out: *mut gl_frustum_t,
                                            mut flZNear: libc::c_float,
                                            mut flZFar: libc::c_float,
                                            mut flFovX: libc::c_float,
                                            mut flFovY: libc::c_float) {
    let mut xs: libc::c_float = 0.;
    let mut xc: libc::c_float = 0.;
    let mut farpoint: vec3_t = [0.; 3];
    let mut nearpoint: vec3_t = [0.; 3];
    let mut normal: vec3_t = [0.; 3];
    let mut iforward: vec3_t = [0.; 3];
    // horizontal fov used for left and right planes
    SinCos(flFovX * (3.14159265358979323846f64 as libc::c_float / 180.0f32) *
               0.5f32, &mut xs, &mut xc);
    // setup left plane
    normal[0 as libc::c_int as usize] =
        xs * RI.cull_vforward[0 as libc::c_int as usize] +
            -xc * RI.cull_vright[0 as libc::c_int as usize];
    normal[1 as libc::c_int as usize] =
        xs * RI.cull_vforward[1 as libc::c_int as usize] +
            -xc * RI.cull_vright[1 as libc::c_int as usize];
    normal[2 as libc::c_int as usize] =
        xs * RI.cull_vforward[2 as libc::c_int as usize] +
            -xc * RI.cull_vright[2 as libc::c_int as usize];
    GL_FrustumSetPlane(out, 0 as libc::c_int,
                       normal.as_mut_ptr() as *const vec_t,
                       RI.cullorigin[0 as libc::c_int as usize] *
                           normal[0 as libc::c_int as usize] +
                           RI.cullorigin[1 as libc::c_int as usize] *
                               normal[1 as libc::c_int as usize] +
                           RI.cullorigin[2 as libc::c_int as usize] *
                               normal[2 as libc::c_int as usize]);
    // setup right plane
    normal[0 as libc::c_int as usize] =
        xs * RI.cull_vforward[0 as libc::c_int as usize] +
            xc * RI.cull_vright[0 as libc::c_int as usize];
    normal[1 as libc::c_int as usize] =
        xs * RI.cull_vforward[1 as libc::c_int as usize] +
            xc * RI.cull_vright[1 as libc::c_int as usize];
    normal[2 as libc::c_int as usize] =
        xs * RI.cull_vforward[2 as libc::c_int as usize] +
            xc * RI.cull_vright[2 as libc::c_int as usize];
    GL_FrustumSetPlane(out, 1 as libc::c_int,
                       normal.as_mut_ptr() as *const vec_t,
                       RI.cullorigin[0 as libc::c_int as usize] *
                           normal[0 as libc::c_int as usize] +
                           RI.cullorigin[1 as libc::c_int as usize] *
                               normal[1 as libc::c_int as usize] +
                           RI.cullorigin[2 as libc::c_int as usize] *
                               normal[2 as libc::c_int as usize]);
    // vertical fov used for top and bottom planes
    SinCos(flFovY * (3.14159265358979323846f64 as libc::c_float / 180.0f32) *
               0.5f32, &mut xs, &mut xc);
    iforward[0 as libc::c_int as usize] =
        -RI.cull_vforward[0 as libc::c_int as usize];
    iforward[1 as libc::c_int as usize] =
        -RI.cull_vforward[1 as libc::c_int as usize];
    iforward[2 as libc::c_int as usize] =
        -RI.cull_vforward[2 as libc::c_int as usize];
    // setup bottom plane
    normal[0 as libc::c_int as usize] =
        xs * RI.cull_vforward[0 as libc::c_int as usize] +
            -xc * RI.cull_vup[0 as libc::c_int as usize];
    normal[1 as libc::c_int as usize] =
        xs * RI.cull_vforward[1 as libc::c_int as usize] +
            -xc * RI.cull_vup[1 as libc::c_int as usize];
    normal[2 as libc::c_int as usize] =
        xs * RI.cull_vforward[2 as libc::c_int as usize] +
            -xc * RI.cull_vup[2 as libc::c_int as usize];
    GL_FrustumSetPlane(out, 2 as libc::c_int,
                       normal.as_mut_ptr() as *const vec_t,
                       RI.cullorigin[0 as libc::c_int as usize] *
                           normal[0 as libc::c_int as usize] +
                           RI.cullorigin[1 as libc::c_int as usize] *
                               normal[1 as libc::c_int as usize] +
                           RI.cullorigin[2 as libc::c_int as usize] *
                               normal[2 as libc::c_int as usize]);
    // setup top plane
    normal[0 as libc::c_int as usize] =
        xs * RI.cull_vforward[0 as libc::c_int as usize] +
            xc * RI.cull_vup[0 as libc::c_int as usize];
    normal[1 as libc::c_int as usize] =
        xs * RI.cull_vforward[1 as libc::c_int as usize] +
            xc * RI.cull_vup[1 as libc::c_int as usize];
    normal[2 as libc::c_int as usize] =
        xs * RI.cull_vforward[2 as libc::c_int as usize] +
            xc * RI.cull_vup[2 as libc::c_int as usize];
    GL_FrustumSetPlane(out, 3 as libc::c_int,
                       normal.as_mut_ptr() as *const vec_t,
                       RI.cullorigin[0 as libc::c_int as usize] *
                           normal[0 as libc::c_int as usize] +
                           RI.cullorigin[1 as libc::c_int as usize] *
                               normal[1 as libc::c_int as usize] +
                           RI.cullorigin[2 as libc::c_int as usize] *
                               normal[2 as libc::c_int as usize]);
    // setup far plane
    farpoint[0 as libc::c_int as usize] =
        RI.cullorigin[0 as libc::c_int as usize] +
            flZFar * RI.cull_vforward[0 as libc::c_int as usize];
    farpoint[1 as libc::c_int as usize] =
        RI.cullorigin[1 as libc::c_int as usize] +
            flZFar * RI.cull_vforward[1 as libc::c_int as usize];
    farpoint[2 as libc::c_int as usize] =
        RI.cullorigin[2 as libc::c_int as usize] +
            flZFar * RI.cull_vforward[2 as libc::c_int as usize];
    GL_FrustumSetPlane(out, 4 as libc::c_int,
                       iforward.as_mut_ptr() as *const vec_t,
                       iforward[0 as libc::c_int as usize] *
                           farpoint[0 as libc::c_int as usize] +
                           iforward[1 as libc::c_int as usize] *
                               farpoint[1 as libc::c_int as usize] +
                           iforward[2 as libc::c_int as usize] *
                               farpoint[2 as libc::c_int as usize]);
    // no need to setup backplane for general view.
    if flZNear == 0.0f32 { return }
    // setup near plane
    nearpoint[0 as libc::c_int as usize] =
        RI.cullorigin[0 as libc::c_int as usize] +
            flZNear * RI.cull_vforward[0 as libc::c_int as usize];
    nearpoint[1 as libc::c_int as usize] =
        RI.cullorigin[1 as libc::c_int as usize] +
            flZNear * RI.cull_vforward[1 as libc::c_int as usize];
    nearpoint[2 as libc::c_int as usize] =
        RI.cullorigin[2 as libc::c_int as usize] +
            flZNear * RI.cull_vforward[2 as libc::c_int as usize];
    GL_FrustumSetPlane(out, 5 as libc::c_int,
                       RI.cull_vforward.as_mut_ptr() as *const vec_t,
                       RI.cull_vforward[0 as libc::c_int as usize] *
                           nearpoint[0 as libc::c_int as usize] +
                           RI.cull_vforward[1 as libc::c_int as usize] *
                               nearpoint[1 as libc::c_int as usize] +
                           RI.cull_vforward[2 as libc::c_int as usize] *
                               nearpoint[2 as libc::c_int as usize]);
}
#[no_mangle]
pub unsafe extern "C" fn GL_FrustumInitOrtho(mut out: *mut gl_frustum_t,
                                             mut xLeft: libc::c_float,
                                             mut xRight: libc::c_float,
                                             mut yTop: libc::c_float,
                                             mut yBottom: libc::c_float,
                                             mut flZNear: libc::c_float,
                                             mut flZFar: libc::c_float) {
    let mut iforward: vec3_t = [0.; 3];
    let mut iright: vec3_t = [0.; 3];
    let mut iup: vec3_t = [0.; 3];
    // setup the near and far planes
    let mut orgOffset: libc::c_float =
        RI.cullorigin[0 as libc::c_int as usize] *
            RI.cull_vforward[0 as libc::c_int as usize] +
            RI.cullorigin[1 as libc::c_int as usize] *
                RI.cull_vforward[1 as libc::c_int as usize] +
            RI.cullorigin[2 as libc::c_int as usize] *
                RI.cull_vforward[2 as libc::c_int as usize];
    iforward[0 as libc::c_int as usize] =
        -RI.cull_vforward[0 as libc::c_int as usize];
    iforward[1 as libc::c_int as usize] =
        -RI.cull_vforward[1 as libc::c_int as usize];
    iforward[2 as libc::c_int as usize] =
        -RI.cull_vforward[2 as libc::c_int as usize];
    // because quake ortho is inverted and far and near should be swaped
    GL_FrustumSetPlane(out, 4 as libc::c_int,
                       iforward.as_mut_ptr() as *const vec_t,
                       -flZNear - orgOffset);
    GL_FrustumSetPlane(out, 5 as libc::c_int,
                       RI.cull_vforward.as_mut_ptr() as *const vec_t,
                       flZFar + orgOffset);
    // setup left and right planes
    orgOffset =
        RI.cullorigin[0 as libc::c_int as usize] *
            RI.cull_vright[0 as libc::c_int as usize] +
            RI.cullorigin[1 as libc::c_int as usize] *
                RI.cull_vright[1 as libc::c_int as usize] +
            RI.cullorigin[2 as libc::c_int as usize] *
                RI.cull_vright[2 as libc::c_int as usize];
    iright[0 as libc::c_int as usize] =
        -RI.cull_vright[0 as libc::c_int as usize];
    iright[1 as libc::c_int as usize] =
        -RI.cull_vright[1 as libc::c_int as usize];
    iright[2 as libc::c_int as usize] =
        -RI.cull_vright[2 as libc::c_int as usize];
    GL_FrustumSetPlane(out, 0 as libc::c_int,
                       RI.cull_vright.as_mut_ptr() as *const vec_t,
                       xLeft + orgOffset);
    GL_FrustumSetPlane(out, 1 as libc::c_int,
                       iright.as_mut_ptr() as *const vec_t,
                       -xRight - orgOffset);
    // setup top and buttom planes
    orgOffset =
        RI.cullorigin[0 as libc::c_int as usize] *
            RI.cull_vup[0 as libc::c_int as usize] +
            RI.cullorigin[1 as libc::c_int as usize] *
                RI.cull_vup[1 as libc::c_int as usize] +
            RI.cullorigin[2 as libc::c_int as usize] *
                RI.cull_vup[2 as libc::c_int as usize];
    iup[0 as libc::c_int as usize] = -RI.cull_vup[0 as libc::c_int as usize];
    iup[1 as libc::c_int as usize] = -RI.cull_vup[1 as libc::c_int as usize];
    iup[2 as libc::c_int as usize] = -RI.cull_vup[2 as libc::c_int as usize];
    GL_FrustumSetPlane(out, 3 as libc::c_int,
                       RI.cull_vup.as_mut_ptr() as *const vec_t,
                       yTop + orgOffset);
    GL_FrustumSetPlane(out, 2 as libc::c_int,
                       iup.as_mut_ptr() as *const vec_t,
                       -yBottom - orgOffset);
}
#[no_mangle]
pub unsafe extern "C" fn GL_FrustumInitBox(mut out: *mut gl_frustum_t,
                                           mut org: *const vec_t,
                                           mut radius: libc::c_float) {
    let mut normal: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        // setup normal for each direction
        normal[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
        normal[1 as libc::c_int as usize] = normal[2 as libc::c_int as usize];
        normal[0 as libc::c_int as usize] = normal[1 as libc::c_int as usize];
        normal[(((i >> 1 as libc::c_int) + 1 as libc::c_int) %
                    3 as libc::c_int) as usize] =
            if i & 1 as libc::c_int != 0 { 1.0f32 } else { -1.0f32 };
        GL_FrustumSetPlane(out, i, normal.as_mut_ptr() as *const vec_t,
                           *org.offset(0 as libc::c_int as isize) *
                               normal[0 as libc::c_int as usize] +
                               *org.offset(1 as libc::c_int as isize) *
                                   normal[1 as libc::c_int as usize] +
                               *org.offset(2 as libc::c_int as isize) *
                                   normal[2 as libc::c_int as usize] -
                               radius);
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn GL_FrustumInitProjFromMatrix(mut out:
                                                          *mut gl_frustum_t,
                                                      mut projection:
                                                          *const [vec_t; 4]) {
    let mut i: libc::c_int = 0;
    // left
    (*out).planes[0 as libc::c_int as usize].normal[0 as libc::c_int as usize]
        =
        (*projection.offset(0 as libc::c_int as
                                isize))[3 as libc::c_int as usize] +
            (*projection.offset(0 as libc::c_int as
                                    isize))[0 as libc::c_int as usize];
    (*out).planes[0 as libc::c_int as usize].normal[1 as libc::c_int as usize]
        =
        (*projection.offset(1 as libc::c_int as
                                isize))[3 as libc::c_int as usize] +
            (*projection.offset(1 as libc::c_int as
                                    isize))[0 as libc::c_int as usize];
    (*out).planes[0 as libc::c_int as usize].normal[2 as libc::c_int as usize]
        =
        (*projection.offset(2 as libc::c_int as
                                isize))[3 as libc::c_int as usize] +
            (*projection.offset(2 as libc::c_int as
                                    isize))[0 as libc::c_int as usize];
    (*out).planes[0 as libc::c_int as usize].dist =
        -((*projection.offset(3 as libc::c_int as
                                  isize))[3 as libc::c_int as usize] +
              (*projection.offset(3 as libc::c_int as
                                      isize))[0 as libc::c_int as usize]);
    // right
    (*out).planes[1 as libc::c_int as usize].normal[0 as libc::c_int as usize]
        =
        (*projection.offset(0 as libc::c_int as
                                isize))[3 as libc::c_int as usize] -
            (*projection.offset(0 as libc::c_int as
                                    isize))[0 as libc::c_int as usize];
    (*out).planes[1 as libc::c_int as usize].normal[1 as libc::c_int as usize]
        =
        (*projection.offset(1 as libc::c_int as
                                isize))[3 as libc::c_int as usize] -
            (*projection.offset(1 as libc::c_int as
                                    isize))[0 as libc::c_int as usize];
    (*out).planes[1 as libc::c_int as usize].normal[2 as libc::c_int as usize]
        =
        (*projection.offset(2 as libc::c_int as
                                isize))[3 as libc::c_int as usize] -
            (*projection.offset(2 as libc::c_int as
                                    isize))[0 as libc::c_int as usize];
    (*out).planes[1 as libc::c_int as usize].dist =
        -((*projection.offset(3 as libc::c_int as
                                  isize))[3 as libc::c_int as usize] -
              (*projection.offset(3 as libc::c_int as
                                      isize))[0 as libc::c_int as usize]);
    // bottom
    (*out).planes[2 as libc::c_int as usize].normal[0 as libc::c_int as usize]
        =
        (*projection.offset(0 as libc::c_int as
                                isize))[3 as libc::c_int as usize] +
            (*projection.offset(0 as libc::c_int as
                                    isize))[1 as libc::c_int as usize];
    (*out).planes[2 as libc::c_int as usize].normal[1 as libc::c_int as usize]
        =
        (*projection.offset(1 as libc::c_int as
                                isize))[3 as libc::c_int as usize] +
            (*projection.offset(1 as libc::c_int as
                                    isize))[1 as libc::c_int as usize];
    (*out).planes[2 as libc::c_int as usize].normal[2 as libc::c_int as usize]
        =
        (*projection.offset(2 as libc::c_int as
                                isize))[3 as libc::c_int as usize] +
            (*projection.offset(2 as libc::c_int as
                                    isize))[1 as libc::c_int as usize];
    (*out).planes[2 as libc::c_int as usize].dist =
        -((*projection.offset(3 as libc::c_int as
                                  isize))[3 as libc::c_int as usize] +
              (*projection.offset(3 as libc::c_int as
                                      isize))[1 as libc::c_int as usize]);
    // top
    (*out).planes[3 as libc::c_int as usize].normal[0 as libc::c_int as usize]
        =
        (*projection.offset(0 as libc::c_int as
                                isize))[3 as libc::c_int as usize] -
            (*projection.offset(0 as libc::c_int as
                                    isize))[1 as libc::c_int as usize];
    (*out).planes[3 as libc::c_int as usize].normal[1 as libc::c_int as usize]
        =
        (*projection.offset(1 as libc::c_int as
                                isize))[3 as libc::c_int as usize] -
            (*projection.offset(1 as libc::c_int as
                                    isize))[1 as libc::c_int as usize];
    (*out).planes[3 as libc::c_int as usize].normal[2 as libc::c_int as usize]
        =
        (*projection.offset(2 as libc::c_int as
                                isize))[3 as libc::c_int as usize] -
            (*projection.offset(2 as libc::c_int as
                                    isize))[1 as libc::c_int as usize];
    (*out).planes[3 as libc::c_int as usize].dist =
        -((*projection.offset(3 as libc::c_int as
                                  isize))[3 as libc::c_int as usize] -
              (*projection.offset(3 as libc::c_int as
                                      isize))[1 as libc::c_int as usize]);
    // near
    (*out).planes[5 as libc::c_int as usize].normal[0 as libc::c_int as usize]
        =
        (*projection.offset(0 as libc::c_int as
                                isize))[3 as libc::c_int as usize] +
            (*projection.offset(0 as libc::c_int as
                                    isize))[2 as libc::c_int as usize];
    (*out).planes[5 as libc::c_int as usize].normal[1 as libc::c_int as usize]
        =
        (*projection.offset(1 as libc::c_int as
                                isize))[3 as libc::c_int as usize] +
            (*projection.offset(1 as libc::c_int as
                                    isize))[2 as libc::c_int as usize];
    (*out).planes[5 as libc::c_int as usize].normal[2 as libc::c_int as usize]
        =
        (*projection.offset(2 as libc::c_int as
                                isize))[3 as libc::c_int as usize] +
            (*projection.offset(2 as libc::c_int as
                                    isize))[2 as libc::c_int as usize];
    (*out).planes[5 as libc::c_int as usize].dist =
        -((*projection.offset(3 as libc::c_int as
                                  isize))[3 as libc::c_int as usize] +
              (*projection.offset(3 as libc::c_int as
                                      isize))[2 as libc::c_int as usize]);
    // far
    (*out).planes[4 as libc::c_int as usize].normal[0 as libc::c_int as usize]
        =
        (*projection.offset(0 as libc::c_int as
                                isize))[3 as libc::c_int as usize] -
            (*projection.offset(0 as libc::c_int as
                                    isize))[2 as libc::c_int as usize];
    (*out).planes[4 as libc::c_int as usize].normal[1 as libc::c_int as usize]
        =
        (*projection.offset(1 as libc::c_int as
                                isize))[3 as libc::c_int as usize] -
            (*projection.offset(1 as libc::c_int as
                                    isize))[2 as libc::c_int as usize];
    (*out).planes[4 as libc::c_int as usize].normal[2 as libc::c_int as usize]
        =
        (*projection.offset(2 as libc::c_int as
                                isize))[3 as libc::c_int as usize] -
            (*projection.offset(2 as libc::c_int as
                                    isize))[2 as libc::c_int as usize];
    (*out).planes[4 as libc::c_int as usize].dist =
        -((*projection.offset(3 as libc::c_int as
                                  isize))[3 as libc::c_int as usize] -
              (*projection.offset(3 as libc::c_int as
                                      isize))[2 as libc::c_int as usize]);
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int { GL_FrustumNormalizePlane(out, i); i += 1 };
}
#[no_mangle]
pub unsafe extern "C" fn GL_FrustumComputeCorners(mut out: *mut gl_frustum_t,
                                                  mut corners: *mut vec3_t) {
    memset(corners as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<vec3_t>() as
                libc::c_ulong).wrapping_mul(8 as libc::c_int as
                                                libc::c_ulong));
    PlanesGetIntersectionPoint(&mut *(*out).planes.as_mut_ptr().offset(0 as
                                                                           libc::c_int
                                                                           as
                                                                           isize),
                               &mut *(*out).planes.as_mut_ptr().offset(3 as
                                                                           libc::c_int
                                                                           as
                                                                           isize),
                               &mut *(*out).planes.as_mut_ptr().offset(4 as
                                                                           libc::c_int
                                                                           as
                                                                           isize),
                               (*corners.offset(0 as libc::c_int as
                                                    isize)).as_mut_ptr());
    PlanesGetIntersectionPoint(&mut *(*out).planes.as_mut_ptr().offset(1 as
                                                                           libc::c_int
                                                                           as
                                                                           isize),
                               &mut *(*out).planes.as_mut_ptr().offset(3 as
                                                                           libc::c_int
                                                                           as
                                                                           isize),
                               &mut *(*out).planes.as_mut_ptr().offset(4 as
                                                                           libc::c_int
                                                                           as
                                                                           isize),
                               (*corners.offset(1 as libc::c_int as
                                                    isize)).as_mut_ptr());
    PlanesGetIntersectionPoint(&mut *(*out).planes.as_mut_ptr().offset(0 as
                                                                           libc::c_int
                                                                           as
                                                                           isize),
                               &mut *(*out).planes.as_mut_ptr().offset(2 as
                                                                           libc::c_int
                                                                           as
                                                                           isize),
                               &mut *(*out).planes.as_mut_ptr().offset(4 as
                                                                           libc::c_int
                                                                           as
                                                                           isize),
                               (*corners.offset(2 as libc::c_int as
                                                    isize)).as_mut_ptr());
    PlanesGetIntersectionPoint(&mut *(*out).planes.as_mut_ptr().offset(1 as
                                                                           libc::c_int
                                                                           as
                                                                           isize),
                               &mut *(*out).planes.as_mut_ptr().offset(2 as
                                                                           libc::c_int
                                                                           as
                                                                           isize),
                               &mut *(*out).planes.as_mut_ptr().offset(4 as
                                                                           libc::c_int
                                                                           as
                                                                           isize),
                               (*corners.offset(3 as libc::c_int as
                                                    isize)).as_mut_ptr());
    if (*out).clipFlags & (1 as libc::c_uint) << 5 as libc::c_int != 0 {
        PlanesGetIntersectionPoint(&mut *(*out).planes.as_mut_ptr().offset(0
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                   &mut *(*out).planes.as_mut_ptr().offset(3
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                   &mut *(*out).planes.as_mut_ptr().offset(5
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                   (*corners.offset(4 as libc::c_int as
                                                        isize)).as_mut_ptr());
        PlanesGetIntersectionPoint(&mut *(*out).planes.as_mut_ptr().offset(1
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                   &mut *(*out).planes.as_mut_ptr().offset(3
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                   &mut *(*out).planes.as_mut_ptr().offset(5
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                   (*corners.offset(5 as libc::c_int as
                                                        isize)).as_mut_ptr());
        PlanesGetIntersectionPoint(&mut *(*out).planes.as_mut_ptr().offset(0
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                   &mut *(*out).planes.as_mut_ptr().offset(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                   &mut *(*out).planes.as_mut_ptr().offset(5
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                   (*corners.offset(6 as libc::c_int as
                                                        isize)).as_mut_ptr());
        PlanesGetIntersectionPoint(&mut *(*out).planes.as_mut_ptr().offset(1
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                   &mut *(*out).planes.as_mut_ptr().offset(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                   &mut *(*out).planes.as_mut_ptr().offset(5
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                   (*corners.offset(7 as libc::c_int as
                                                        isize)).as_mut_ptr());
    } else {
        PlanesGetIntersectionPoint(&mut *(*out).planes.as_mut_ptr().offset(0
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                   &mut *(*out).planes.as_mut_ptr().offset(1
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                   &mut *(*out).planes.as_mut_ptr().offset(3
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                   (*corners.offset(4 as libc::c_int as
                                                        isize)).as_mut_ptr());
        (*corners.offset(5 as libc::c_int as
                             isize))[0 as libc::c_int as usize] =
            (*corners.offset(4 as libc::c_int as
                                 isize))[0 as libc::c_int as usize];
        (*corners.offset(5 as libc::c_int as
                             isize))[1 as libc::c_int as usize] =
            (*corners.offset(4 as libc::c_int as
                                 isize))[1 as libc::c_int as usize];
        (*corners.offset(5 as libc::c_int as
                             isize))[2 as libc::c_int as usize] =
            (*corners.offset(4 as libc::c_int as
                                 isize))[2 as libc::c_int as usize];
        (*corners.offset(6 as libc::c_int as
                             isize))[0 as libc::c_int as usize] =
            (*corners.offset(4 as libc::c_int as
                                 isize))[0 as libc::c_int as usize];
        (*corners.offset(6 as libc::c_int as
                             isize))[1 as libc::c_int as usize] =
            (*corners.offset(4 as libc::c_int as
                                 isize))[1 as libc::c_int as usize];
        (*corners.offset(6 as libc::c_int as
                             isize))[2 as libc::c_int as usize] =
            (*corners.offset(4 as libc::c_int as
                                 isize))[2 as libc::c_int as usize];
        (*corners.offset(7 as libc::c_int as
                             isize))[0 as libc::c_int as usize] =
            (*corners.offset(4 as libc::c_int as
                                 isize))[0 as libc::c_int as usize];
        (*corners.offset(7 as libc::c_int as
                             isize))[1 as libc::c_int as usize] =
            (*corners.offset(4 as libc::c_int as
                                 isize))[1 as libc::c_int as usize];
        (*corners.offset(7 as libc::c_int as
                             isize))[2 as libc::c_int as usize] =
            (*corners.offset(4 as libc::c_int as
                                 isize))[2 as libc::c_int as usize]
    };
}
#[no_mangle]
pub unsafe extern "C" fn GL_FrustumComputeBounds(mut out: *mut gl_frustum_t,
                                                 mut mins: *mut vec_t,
                                                 mut maxs: *mut vec_t) {
    let mut corners: [vec3_t; 8] = [[0.; 3]; 8];
    let mut i: libc::c_int = 0;
    GL_FrustumComputeCorners(out, corners.as_mut_ptr());
    ClearBounds(mins, maxs);
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        AddPointToBounds(corners[i as usize].as_mut_ptr() as *const vec_t,
                         mins, maxs);
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn GL_FrustumDrawDebug(mut out: *mut gl_frustum_t) {
    let mut bbox: [vec3_t; 8] = [[0.; 3]; 8];
    let mut i: libc::c_int = 0;
    GL_FrustumComputeCorners(out, bbox.as_mut_ptr());
    // g-cont. frustum must be yellow :-)
    pglColor4f.expect("non-null function pointer")(1.0f32, 1.0f32, 0.0f32,
                                                   1.0f32);
    pglDisable.expect("non-null function pointer")(0xde1 as libc::c_int as
                                                       GLenum);
    pglBegin.expect("non-null function pointer")(0x1 as libc::c_int as
                                                     GLenum);
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        pglVertex3fv.expect("non-null function pointer")(bbox[(i +
                                                                   0 as
                                                                       libc::c_int)
                                                                  as
                                                                  usize].as_mut_ptr());
        pglVertex3fv.expect("non-null function pointer")(bbox[(i +
                                                                   2 as
                                                                       libc::c_int)
                                                                  as
                                                                  usize].as_mut_ptr());
        pglVertex3fv.expect("non-null function pointer")(bbox[(i +
                                                                   4 as
                                                                       libc::c_int)
                                                                  as
                                                                  usize].as_mut_ptr());
        pglVertex3fv.expect("non-null function pointer")(bbox[(i +
                                                                   6 as
                                                                       libc::c_int)
                                                                  as
                                                                  usize].as_mut_ptr());
        pglVertex3fv.expect("non-null function pointer")(bbox[(i +
                                                                   0 as
                                                                       libc::c_int)
                                                                  as
                                                                  usize].as_mut_ptr());
        pglVertex3fv.expect("non-null function pointer")(bbox[(i +
                                                                   4 as
                                                                       libc::c_int)
                                                                  as
                                                                  usize].as_mut_ptr());
        pglVertex3fv.expect("non-null function pointer")(bbox[(i +
                                                                   2 as
                                                                       libc::c_int)
                                                                  as
                                                                  usize].as_mut_ptr());
        pglVertex3fv.expect("non-null function pointer")(bbox[(i +
                                                                   6 as
                                                                       libc::c_int)
                                                                  as
                                                                  usize].as_mut_ptr());
        pglVertex3fv.expect("non-null function pointer")(bbox[(i *
                                                                   2 as
                                                                       libc::c_int
                                                                   +
                                                                   0 as
                                                                       libc::c_int)
                                                                  as
                                                                  usize].as_mut_ptr());
        pglVertex3fv.expect("non-null function pointer")(bbox[(i *
                                                                   2 as
                                                                       libc::c_int
                                                                   +
                                                                   1 as
                                                                       libc::c_int)
                                                                  as
                                                                  usize].as_mut_ptr());
        pglVertex3fv.expect("non-null function pointer")(bbox[(i *
                                                                   2 as
                                                                       libc::c_int
                                                                   +
                                                                   4 as
                                                                       libc::c_int)
                                                                  as
                                                                  usize].as_mut_ptr());
        pglVertex3fv.expect("non-null function pointer")(bbox[(i *
                                                                   2 as
                                                                       libc::c_int
                                                                   +
                                                                   5 as
                                                                       libc::c_int)
                                                                  as
                                                                  usize].as_mut_ptr());
        i += 1 as libc::c_int
    }
    pglEnd.expect("non-null function pointer")();
    pglEnable.expect("non-null function pointer")(0xde1 as libc::c_int as
                                                      GLenum);
}
// cull methods
#[no_mangle]
pub unsafe extern "C" fn GL_FrustumCullBox(mut out: *mut gl_frustum_t,
                                           mut mins: *const vec_t,
                                           mut maxs: *const vec_t,
                                           mut userClipFlags: libc::c_int)
 -> qboolean {
    let mut iClipFlags: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut bit: libc::c_int = 0;
    if (*r_nocull).value != 0. { return false_0 }
    if userClipFlags != 0 as libc::c_int {
        iClipFlags = userClipFlags
    } else { iClipFlags = (*out).clipFlags as libc::c_int }
    i = 6 as libc::c_int;
    bit = 1 as libc::c_int;
    while i > 0 as libc::c_int {
        let mut p: *const mplane_t =
            &mut *(*out).planes.as_mut_ptr().offset((6 as libc::c_int - i) as
                                                        isize) as
                *mut mplane_t;
        if !(iClipFlags & bit == 0) {
            match (*p).signbits as libc::c_int {
                0 => {
                    if (*p).normal[0 as libc::c_int as usize] *
                           *maxs.offset(0 as libc::c_int as isize) +
                           (*p).normal[1 as libc::c_int as usize] *
                               *maxs.offset(1 as libc::c_int as isize) +
                           (*p).normal[2 as libc::c_int as usize] *
                               *maxs.offset(2 as libc::c_int as isize) <
                           (*p).dist {
                        return true_0
                    }
                }
                1 => {
                    if (*p).normal[0 as libc::c_int as usize] *
                           *mins.offset(0 as libc::c_int as isize) +
                           (*p).normal[1 as libc::c_int as usize] *
                               *maxs.offset(1 as libc::c_int as isize) +
                           (*p).normal[2 as libc::c_int as usize] *
                               *maxs.offset(2 as libc::c_int as isize) <
                           (*p).dist {
                        return true_0
                    }
                }
                2 => {
                    if (*p).normal[0 as libc::c_int as usize] *
                           *maxs.offset(0 as libc::c_int as isize) +
                           (*p).normal[1 as libc::c_int as usize] *
                               *mins.offset(1 as libc::c_int as isize) +
                           (*p).normal[2 as libc::c_int as usize] *
                               *maxs.offset(2 as libc::c_int as isize) <
                           (*p).dist {
                        return true_0
                    }
                }
                3 => {
                    if (*p).normal[0 as libc::c_int as usize] *
                           *mins.offset(0 as libc::c_int as isize) +
                           (*p).normal[1 as libc::c_int as usize] *
                               *mins.offset(1 as libc::c_int as isize) +
                           (*p).normal[2 as libc::c_int as usize] *
                               *maxs.offset(2 as libc::c_int as isize) <
                           (*p).dist {
                        return true_0
                    }
                }
                4 => {
                    if (*p).normal[0 as libc::c_int as usize] *
                           *maxs.offset(0 as libc::c_int as isize) +
                           (*p).normal[1 as libc::c_int as usize] *
                               *maxs.offset(1 as libc::c_int as isize) +
                           (*p).normal[2 as libc::c_int as usize] *
                               *mins.offset(2 as libc::c_int as isize) <
                           (*p).dist {
                        return true_0
                    }
                }
                5 => {
                    if (*p).normal[0 as libc::c_int as usize] *
                           *mins.offset(0 as libc::c_int as isize) +
                           (*p).normal[1 as libc::c_int as usize] *
                               *maxs.offset(1 as libc::c_int as isize) +
                           (*p).normal[2 as libc::c_int as usize] *
                               *mins.offset(2 as libc::c_int as isize) <
                           (*p).dist {
                        return true_0
                    }
                }
                6 => {
                    if (*p).normal[0 as libc::c_int as usize] *
                           *maxs.offset(0 as libc::c_int as isize) +
                           (*p).normal[1 as libc::c_int as usize] *
                               *mins.offset(1 as libc::c_int as isize) +
                           (*p).normal[2 as libc::c_int as usize] *
                               *mins.offset(2 as libc::c_int as isize) <
                           (*p).dist {
                        return true_0
                    }
                }
                7 => {
                    if (*p).normal[0 as libc::c_int as usize] *
                           *mins.offset(0 as libc::c_int as isize) +
                           (*p).normal[1 as libc::c_int as usize] *
                               *mins.offset(1 as libc::c_int as isize) +
                           (*p).normal[2 as libc::c_int as usize] *
                               *mins.offset(2 as libc::c_int as isize) <
                           (*p).dist {
                        return true_0
                    }
                }
                _ => { return false_0 }
            }
        }
        i -= 1;
        bit <<= 1 as libc::c_int
    }
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn GL_FrustumCullSphere(mut out: *mut gl_frustum_t,
                                              mut center: *const vec_t,
                                              mut radius: libc::c_float,
                                              mut userClipFlags: libc::c_int)
 -> qboolean {
    let mut iClipFlags: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut bit: libc::c_int = 0;
    if (*r_nocull).value != 0. { return false_0 }
    if userClipFlags != 0 as libc::c_int {
        iClipFlags = userClipFlags
    } else { iClipFlags = (*out).clipFlags as libc::c_int }
    i = 6 as libc::c_int;
    bit = 1 as libc::c_int;
    while i > 0 as libc::c_int {
        let mut p: *const mplane_t =
            &mut *(*out).planes.as_mut_ptr().offset((6 as libc::c_int - i) as
                                                        isize) as
                *mut mplane_t;
        if !(iClipFlags & bit == 0) {
            if *center.offset(0 as libc::c_int as isize) *
                   (*p).normal[0 as libc::c_int as usize] +
                   *center.offset(1 as libc::c_int as isize) *
                       (*p).normal[1 as libc::c_int as usize] +
                   *center.offset(2 as libc::c_int as isize) *
                       (*p).normal[2 as libc::c_int as usize] - (*p).dist <=
                   -radius {
                return true_0
            }
        }
        i -= 1;
        bit <<= 1 as libc::c_int
    }
    return false_0;
}
