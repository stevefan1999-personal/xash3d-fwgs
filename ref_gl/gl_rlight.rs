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
    fn GL_FrustumCullSphere(out: *mut gl_frustum_t, centre: *const vec_t,
                            radius: libc::c_float, userClipFlags: libc::c_int)
     -> qboolean;
    #[no_mangle]
    fn sqrtf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn BoundsAndSphereIntersect(mins: *const vec_t, maxs: *const vec_t,
                                origin: *const vec_t, radius: libc::c_float)
     -> qboolean;
    #[no_mangle]
    fn Matrix3x4_VectorIRotate(in_0: *const [vec_t; 4],
                               v: *const libc::c_float,
                               out: *mut libc::c_float);
    #[no_mangle]
    fn Matrix4x4_VectorITransform(in_0: *const [vec_t; 4],
                                  v: *const libc::c_float,
                                  out: *mut libc::c_float);
    #[no_mangle]
    fn Matrix4x4_CreateFromEntity(out: *mut [vec_t; 4], angles: *const vec_t,
                                  origin: *const vec_t, scale: libc::c_float);
    #[no_mangle]
    static mut vec3_origin: vec3_t;
    #[no_mangle]
    static mut RI: ref_instance_t;
    #[no_mangle]
    static mut tr: gl_globals_t;
    #[no_mangle]
    static mut cl_lightstyle_lerping: *mut cvar_t;
    #[no_mangle]
    static mut gpGlobals: *mut ref_globals_t;
    #[no_mangle]
    static mut gEngfuncs: ref_api_t;
    #[no_mangle]
    static mut r_lighting_modulate: *mut cvar_t;
    #[no_mangle]
    static mut r_lighting_extended: *mut cvar_t;
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
pub type physent_t = physent_s;
#[inline(always)]
unsafe extern "C" fn __tg_sqrt(mut __x: libc::c_float) -> libc::c_float {
    return sqrtf(__x);
}
/*
gl_rlight.c - dynamic and static lights
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
/*
=============================================================================

DYNAMIC LIGHTS

=============================================================================
*/
/*
==================
CL_RunLightStyles

==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_RunLightStyles() {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut flight: libc::c_int = 0;
    let mut clight: libc::c_int = 0;
    let mut l: libc::c_float = 0.;
    let mut lerpfrac: libc::c_float = 0.;
    let mut backlerp: libc::c_float = 0.;
    let mut frametime: libc::c_float =
        (*gpGlobals).time - (*gpGlobals).oldtime;
    let mut scale: libc::c_float = 0.;
    let mut ls: *mut lightstyle_t = 0 as *mut lightstyle_t;
    if gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1 as
                                                                            libc::c_int).is_null()
       {
        return
    }
    scale = (*r_lighting_modulate).value;
    // light animations
	// 'm' is normal light, 'a' is no light, 'z' is double bright
    i = 0 as libc::c_int; // evaluate local time
    while i < 64 as libc::c_int {
        ls = gEngfuncs.GetLightStyle.expect("non-null function pointer")(i);
        if (*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                  as
                                                                                  libc::c_int)).lightdata.is_null()
           {
            tr.lightstylevalue[i as usize] =
                256 as libc::c_int * 256 as libc::c_int
        } else {
            if Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(18
                                                                                                                         as
                                                                                                                         libc::c_int,
                                                                                                                     0
                                                                                                                         as
                                                                                                                         libc::c_int)
                   == 0 && frametime <= 0.1f32 {
                (*ls).time += frametime
            }
            flight =
                ((*ls).time * 10 as libc::c_int as libc::c_float) as
                    libc::c_int as libc::c_float as libc::c_int;
            clight =
                ((*ls).time * 10 as libc::c_int as libc::c_float +
                     1 as libc::c_int as libc::c_float) as libc::c_int as
                    libc::c_float as libc::c_int;
            lerpfrac =
                (*ls).time * 10 as libc::c_int as libc::c_float -
                    flight as libc::c_float;
            backlerp = 1.0f32 - lerpfrac;
            if (*ls).length == 0 {
                tr.lightstylevalue[i as usize] =
                    (256 as libc::c_int as libc::c_float * scale) as
                        libc::c_int
            } else if (*ls).length == 1 as libc::c_int {
                // single length style so don't bother interpolating
                tr.lightstylevalue[i as usize] =
                    ((*ls).map[0 as libc::c_int as usize] *
                         22 as libc::c_int as libc::c_float * scale) as
                        libc::c_int
            } else if (*ls).interp as u64 == 0 ||
                          (if !cl_lightstyle_lerping.is_null() &&
                                  (*cl_lightstyle_lerping).value != 0.0f32 {
                               true_0 as libc::c_int
                           } else { false_0 as libc::c_int }) == 0 {
                tr.lightstylevalue[i as usize] =
                    ((*ls).map[(flight % (*ls).length) as usize] *
                         22 as libc::c_int as libc::c_float * scale) as
                        libc::c_int
            } else {
                // interpolate animating light
		// frame just gone
                k =
                    (*ls).map[(flight % (*ls).length) as usize] as
                        libc::c_int;
                l = k as libc::c_float * 22.0f32 * backlerp;
                // upcoming frame
                k =
                    (*ls).map[(clight % (*ls).length) as usize] as
                        libc::c_int;
                l += k as libc::c_float * 22.0f32 * lerpfrac;
                tr.lightstylevalue[i as usize] =
                    (l as libc::c_int as libc::c_float * scale) as libc::c_int
            }
        }
        i += 1
    };
}
/*
=============
R_MarkLights
=============
*/
#[no_mangle]
pub unsafe extern "C" fn R_MarkLights(mut light: *mut dlight_t,
                                      mut bit: libc::c_int,
                                      mut node: *mut mnode_t) {
    let mut dist: libc::c_float = 0.;
    let mut surf: *mut msurface_t = 0 as *mut msurface_t;
    let mut i: libc::c_int = 0;
    if node.is_null() || (*node).contents < 0 as libc::c_int { return }
    dist =
        (if ((*(*node).plane).type_0 as libc::c_int) < 3 as libc::c_int {
             (*light).origin[(*(*node).plane).type_0 as usize]
         } else {
             ((*light).origin[0 as libc::c_int as usize] *
                  (*(*node).plane).normal[0 as libc::c_int as usize] +
                  (*light).origin[1 as libc::c_int as usize] *
                      (*(*node).plane).normal[1 as libc::c_int as usize]) +
                 (*light).origin[2 as libc::c_int as usize] *
                     (*(*node).plane).normal[2 as libc::c_int as usize]
         }) - (*(*node).plane).dist;
    if dist > (*light).radius {
        R_MarkLights(light, bit, (*node).children[0 as libc::c_int as usize]);
        return
    }
    if dist < -(*light).radius {
        R_MarkLights(light, bit, (*node).children[1 as libc::c_int as usize]);
        return
    }
    // mark the polygons
    surf =
        (*RI.currentmodel).surfaces.offset((*node).firstsurface as libc::c_int
                                               as isize); // no intersection
    i = 0 as libc::c_int;
    while i < (*node).numsurfaces as libc::c_int {
        if !(BoundsAndSphereIntersect((*(*surf).info).mins.as_mut_ptr() as
                                          *const vec_t,
                                      (*(*surf).info).maxs.as_mut_ptr() as
                                          *const vec_t,
                                      (*light).origin.as_mut_ptr() as
                                          *const vec_t, (*light).radius) as
                 u64 == 0) {
            if (*surf).dlightframe != tr.dlightframecount {
                (*surf).dlightbits = 0 as libc::c_int;
                (*surf).dlightframe = tr.dlightframecount
            }
            (*surf).dlightbits |= bit
        }
        i += 1;
        surf = surf.offset(1)
    }
    R_MarkLights(light, bit, (*node).children[0 as libc::c_int as usize]);
    R_MarkLights(light, bit, (*node).children[1 as libc::c_int as usize]);
}
/*
=============
R_PushDlights
=============
*/
#[no_mangle]
pub unsafe extern "C" fn R_PushDlights() {
    let mut l: *mut dlight_t = 0 as *mut dlight_t;
    let mut i: libc::c_int = 0;
    tr.dlightframecount = tr.framecount;
    RI.currententity =
        gEngfuncs.GetEntityByIndex.expect("non-null function pointer")(0 as
                                                                           libc::c_int);
    RI.currentmodel = (*RI.currententity).model;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        l = gEngfuncs.GetDynamicLight.expect("non-null function pointer")(i);
        if !((*l).die < (*gpGlobals).time || (*l).radius == 0.) {
            if !(GL_FrustumCullSphere(&mut RI.frustum,
                                      (*l).origin.as_mut_ptr() as
                                          *const vec_t, (*l).radius,
                                      15 as libc::c_int) as u64 != 0) {
                R_MarkLights(l, (1 as libc::c_int) << i,
                             (*RI.currentmodel).nodes);
            }
        }
        i += 1;
        l = l.offset(1)
    };
}
/*
=============
R_CountDlights
=============
*/
#[no_mangle]
pub unsafe extern "C" fn R_CountDlights() -> libc::c_int {
    let mut l: *mut dlight_t = 0 as *mut dlight_t;
    let mut i: libc::c_int = 0;
    let mut numDlights: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        l = gEngfuncs.GetDynamicLight.expect("non-null function pointer")(i);
        if !((*l).die < (*gpGlobals).time || (*l).radius == 0.) {
            numDlights += 1
        }
        i += 1
    }
    return numDlights;
}
/*
=============
R_CountSurfaceDlights
=============
*/
#[no_mangle]
pub unsafe extern "C" fn R_CountSurfaceDlights(mut surf: *mut msurface_t)
 -> libc::c_int {
    let mut i: libc::c_int = 0; // not lit by this light
    let mut numDlights: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if !((*surf).dlightbits as libc::c_uint & (1 as libc::c_uint) << i ==
                 0) {
            numDlights += 1
        }
        i += 1
    }
    return numDlights;
}
/*
=======================================================================

	AMBIENT LIGHTING

=======================================================================
*/
static mut g_trace_lightspot: vec3_t = [0.; 3];
static mut g_trace_lightvec: vec3_t = [0.; 3];
static mut g_trace_fraction: libc::c_float = 0.;
/*
=================
R_RecursiveLightPoint
=================
*/
unsafe extern "C" fn R_RecursiveLightPoint(mut model: *mut model_t,
                                           mut node: *mut mnode_t,
                                           mut p1f: libc::c_float,
                                           mut p2f: libc::c_float,
                                           mut cv: *mut colorVec,
                                           mut start: *const vec_t,
                                           mut end: *const vec_t)
 -> qboolean {
    let mut front: libc::c_float = 0.;
    let mut back: libc::c_float = 0.;
    let mut frac: libc::c_float = 0.;
    let mut midf: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    let mut map: libc::c_int = 0;
    let mut side: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut ds: libc::c_float = 0.;
    let mut dt: libc::c_float = 0.;
    let mut s: libc::c_float = 0.;
    let mut t: libc::c_float = 0.;
    let mut sample_size: libc::c_int = 0;
    let mut lm: *mut color24 = 0 as *mut color24;
    let mut dm: *mut color24 = 0 as *mut color24;
    let mut info: *mut mextrasurf_t = 0 as *mut mextrasurf_t;
    let mut surf: *mut msurface_t = 0 as *mut msurface_t;
    let mut tex: *mut mtexinfo_t = 0 as *mut mtexinfo_t;
    let mut tbn: matrix3x4 = [[0.; 4]; 3];
    let mut mid: vec3_t = [0.; 3];
    // didn't hit anything
    if node.is_null() || (*node).contents < 0 as libc::c_int {
        (*cv).a = 0 as libc::c_int as libc::c_uint;
        (*cv).b = (*cv).a;
        (*cv).g = (*cv).b;
        (*cv).r = (*cv).g;
        return false_0
    }
    // calculate mid point
    front =
        (if ((*(*node).plane).type_0 as libc::c_int) < 3 as libc::c_int {
             *start.offset((*(*node).plane).type_0 as isize)
         } else {
             (*start.offset(0 as libc::c_int as isize) *
                  (*(*node).plane).normal[0 as libc::c_int as usize] +
                  *start.offset(1 as libc::c_int as isize) *
                      (*(*node).plane).normal[1 as libc::c_int as usize]) +
                 *start.offset(2 as libc::c_int as isize) *
                     (*(*node).plane).normal[2 as libc::c_int as usize]
         }) - (*(*node).plane).dist;
    back =
        (if ((*(*node).plane).type_0 as libc::c_int) < 3 as libc::c_int {
             *end.offset((*(*node).plane).type_0 as isize)
         } else {
             (*end.offset(0 as libc::c_int as isize) *
                  (*(*node).plane).normal[0 as libc::c_int as usize] +
                  *end.offset(1 as libc::c_int as isize) *
                      (*(*node).plane).normal[1 as libc::c_int as usize]) +
                 *end.offset(2 as libc::c_int as isize) *
                     (*(*node).plane).normal[2 as libc::c_int as usize]
         }) - (*(*node).plane).dist;
    side = (front < 0 as libc::c_int as libc::c_float) as libc::c_int;
    if (back < 0 as libc::c_int as libc::c_float) as libc::c_int == side {
        return R_RecursiveLightPoint(model, (*node).children[side as usize],
                                     p1f, p2f, cv, start, end)
    }
    frac = front / (front - back);
    mid[0 as libc::c_int as usize] =
        *start.offset(0 as libc::c_int as isize) +
            frac *
                (*end.offset(0 as libc::c_int as isize) -
                     *start.offset(0 as libc::c_int as isize));
    mid[1 as libc::c_int as usize] =
        *start.offset(1 as libc::c_int as isize) +
            frac *
                (*end.offset(1 as libc::c_int as isize) -
                     *start.offset(1 as libc::c_int as isize));
    mid[2 as libc::c_int as usize] =
        *start.offset(2 as libc::c_int as isize) +
            frac *
                (*end.offset(2 as libc::c_int as isize) -
                     *start.offset(2 as libc::c_int as isize));
    midf = p1f + (p2f - p1f) * frac;
    // co down front side
    if R_RecursiveLightPoint(model, (*node).children[side as usize], p1f,
                             midf, cv, start,
                             mid.as_mut_ptr() as *const vec_t) as u64 != 0 {
        return true_0
    } // hit something
    if (back < 0 as libc::c_int as libc::c_float) as libc::c_int == side {
        (*cv).a = 0 as libc::c_int as libc::c_uint;
        (*cv).b = (*cv).a;
        (*cv).g = (*cv).b;
        (*cv).r = (*cv).g;
        return false_0
        // didn't hit anything
    }
    // check for impact on this node
    surf =
        (*model).surfaces.offset((*node).firstsurface as libc::c_int as
                                     isize); // no lightmaps
    g_trace_lightspot[0 as libc::c_int as usize] =
        mid[0 as libc::c_int as usize];
    g_trace_lightspot[1 as libc::c_int as usize] =
        mid[1 as libc::c_int as usize];
    g_trace_lightspot[2 as libc::c_int as usize] =
        mid[2 as libc::c_int as usize];
    i = 0 as libc::c_int;
    while i < (*node).numsurfaces as libc::c_int {
        let mut smax: libc::c_int = 0;
        let mut tmax: libc::c_int = 0;
        tex = (*surf).texinfo;
        info = (*surf).info;
        if !((*surf).flags as libc::c_uint &
                 (1 as libc::c_uint) << 5 as libc::c_int != 0) {
            s =
                mid[0 as libc::c_int as usize] *
                    (*info).lmvecs[0 as libc::c_int as
                                       usize][0 as libc::c_int as usize] +
                    mid[1 as libc::c_int as usize] *
                        (*info).lmvecs[0 as libc::c_int as
                                           usize][1 as libc::c_int as usize] +
                    mid[2 as libc::c_int as usize] *
                        (*info).lmvecs[0 as libc::c_int as
                                           usize][2 as libc::c_int as usize] +
                    (*info).lmvecs[0 as libc::c_int as
                                       usize][3 as libc::c_int as usize];
            t =
                mid[0 as libc::c_int as usize] *
                    (*info).lmvecs[1 as libc::c_int as
                                       usize][0 as libc::c_int as usize] +
                    mid[1 as libc::c_int as usize] *
                        (*info).lmvecs[1 as libc::c_int as
                                           usize][1 as libc::c_int as usize] +
                    mid[2 as libc::c_int as usize] *
                        (*info).lmvecs[1 as libc::c_int as
                                           usize][2 as libc::c_int as usize] +
                    (*info).lmvecs[1 as libc::c_int as
                                       usize][3 as libc::c_int as usize];
            if !(s <
                     (*info).lightmapmins[0 as libc::c_int as usize] as
                         libc::c_int as libc::c_float ||
                     t <
                         (*info).lightmapmins[1 as libc::c_int as usize] as
                             libc::c_int as libc::c_float) {
                ds =
                    s -
                        (*info).lightmapmins[0 as libc::c_int as usize] as
                            libc::c_int as libc::c_float;
                dt =
                    t -
                        (*info).lightmapmins[1 as libc::c_int as usize] as
                            libc::c_int as libc::c_float;
                if !(ds >
                         (*info).lightextents[0 as libc::c_int as usize] as
                             libc::c_int as libc::c_float ||
                         dt >
                             (*info).lightextents[1 as libc::c_int as usize]
                                 as libc::c_int as libc::c_float) {
                    (*cv).a = 0 as libc::c_int as libc::c_uint;
                    (*cv).b = (*cv).a;
                    (*cv).g = (*cv).b;
                    (*cv).r = (*cv).g;
                    if (*surf).samples.is_null() { return true_0 }
                    sample_size =
                        gEngfuncs.Mod_SampleSizeForFace.expect("non-null function pointer")(surf);
                    smax =
                        (*info).lightextents[0 as libc::c_int as usize] as
                            libc::c_int / sample_size + 1 as libc::c_int;
                    tmax =
                        (*info).lightextents[1 as libc::c_int as usize] as
                            libc::c_int / sample_size + 1 as libc::c_int;
                    ds /= sample_size as libc::c_float;
                    dt /= sample_size as libc::c_float;
                    lm =
                        (*surf).samples.offset(((if dt < 0.0f32 {
                                                     (dt - 0.5f32) as
                                                         libc::c_int
                                                 } else {
                                                     (dt + 0.5f32) as
                                                         libc::c_int
                                                 }) * smax) as
                                                   isize).offset((if ds <
                                                                         0.0f32
                                                                     {
                                                                      (ds -
                                                                           0.5f32)
                                                                          as
                                                                          libc::c_int
                                                                  } else {
                                                                      (ds +
                                                                           0.5f32)
                                                                          as
                                                                          libc::c_int
                                                                  }) as
                                                                     isize);
                    g_trace_fraction = midf;
                    size = smax * tmax;
                    dm = 0 as *mut color24;
                    if !(*(*surf).info).deluxemap.is_null() {
                        let mut faceNormal: vec3_t = [0.; 3];
                        if (*surf).flags as libc::c_uint &
                               (1 as libc::c_uint) << 1 as libc::c_int != 0 {
                            faceNormal[0 as libc::c_int as usize] =
                                -(*(*surf).plane).normal[0 as libc::c_int as
                                                             usize];
                            faceNormal[1 as libc::c_int as usize] =
                                -(*(*surf).plane).normal[1 as libc::c_int as
                                                             usize];
                            faceNormal[2 as libc::c_int as usize] =
                                -(*(*surf).plane).normal[2 as libc::c_int as
                                                             usize]
                        } else {
                            faceNormal[0 as libc::c_int as usize] =
                                (*(*surf).plane).normal[0 as libc::c_int as
                                                            usize];
                            faceNormal[1 as libc::c_int as usize] =
                                (*(*surf).plane).normal[1 as libc::c_int as
                                                            usize];
                            faceNormal[2 as libc::c_int as usize] =
                                (*(*surf).plane).normal[2 as libc::c_int as
                                                            usize]
                        }
                        // compute face TBN
                        tbn[0 as libc::c_int as
                                usize][0 as libc::c_int as usize] =
                            (*(*surf).info).lmvecs[0 as libc::c_int as
                                                       usize][0 as libc::c_int
                                                                  as
                                                                  usize]; // skip to next lightmap
                        tbn[0 as libc::c_int as
                                usize][1 as libc::c_int as usize] =
                            (*(*surf).info).lmvecs[0 as libc::c_int as
                                                       usize][1 as libc::c_int
                                                                  as usize];
                        tbn[0 as libc::c_int as
                                usize][2 as libc::c_int as usize] =
                            (*(*surf).info).lmvecs[0 as libc::c_int as
                                                       usize][2 as libc::c_int
                                                                  as usize];
                        tbn[0 as libc::c_int as
                                usize][3 as libc::c_int as usize] = 0.0f32;
                        tbn[1 as libc::c_int as
                                usize][0 as libc::c_int as usize] =
                            -(*(*surf).info).lmvecs[1 as libc::c_int as
                                                        usize][0 as
                                                                   libc::c_int
                                                                   as usize];
                        tbn[1 as libc::c_int as
                                usize][1 as libc::c_int as usize] =
                            -(*(*surf).info).lmvecs[1 as libc::c_int as
                                                        usize][1 as
                                                                   libc::c_int
                                                                   as usize];
                        tbn[1 as libc::c_int as
                                usize][2 as libc::c_int as usize] =
                            -(*(*surf).info).lmvecs[1 as libc::c_int as
                                                        usize][2 as
                                                                   libc::c_int
                                                                   as usize];
                        tbn[1 as libc::c_int as
                                usize][3 as libc::c_int as usize] = 0.0f32;
                        tbn[2 as libc::c_int as
                                usize][0 as libc::c_int as usize] =
                            faceNormal[0 as libc::c_int as usize];
                        tbn[2 as libc::c_int as
                                usize][1 as libc::c_int as usize] =
                            faceNormal[1 as libc::c_int as usize];
                        tbn[2 as libc::c_int as
                                usize][2 as libc::c_int as usize] =
                            faceNormal[2 as libc::c_int as usize];
                        tbn[2 as libc::c_int as
                                usize][3 as libc::c_int as usize] = 0.0f32;
                        let mut ilength: libc::c_float =
                            __tg_sqrt(tbn[0 as libc::c_int as
                                              usize][0 as libc::c_int as
                                                         usize] *
                                          tbn[0 as libc::c_int as
                                                  usize][0 as libc::c_int as
                                                             usize] +
                                          tbn[0 as libc::c_int as
                                                  usize][1 as libc::c_int as
                                                             usize] *
                                              tbn[0 as libc::c_int as
                                                      usize][1 as libc::c_int
                                                                 as usize] +
                                          tbn[0 as libc::c_int as
                                                  usize][2 as libc::c_int as
                                                             usize] *
                                              tbn[0 as libc::c_int as
                                                      usize][2 as libc::c_int
                                                                 as usize]);
                        if ilength != 0. { ilength = 1.0f32 / ilength }
                        tbn[0 as libc::c_int as
                                usize][0 as libc::c_int as usize] *= ilength;
                        tbn[0 as libc::c_int as
                                usize][1 as libc::c_int as usize] *= ilength;
                        tbn[0 as libc::c_int as
                                usize][2 as libc::c_int as usize] *= ilength;
                        let mut ilength_0: libc::c_float =
                            __tg_sqrt(tbn[1 as libc::c_int as
                                              usize][0 as libc::c_int as
                                                         usize] *
                                          tbn[1 as libc::c_int as
                                                  usize][0 as libc::c_int as
                                                             usize] +
                                          tbn[1 as libc::c_int as
                                                  usize][1 as libc::c_int as
                                                             usize] *
                                              tbn[1 as libc::c_int as
                                                      usize][1 as libc::c_int
                                                                 as usize] +
                                          tbn[1 as libc::c_int as
                                                  usize][2 as libc::c_int as
                                                             usize] *
                                              tbn[1 as libc::c_int as
                                                      usize][2 as libc::c_int
                                                                 as usize]);
                        if ilength_0 != 0. { ilength_0 = 1.0f32 / ilength_0 }
                        tbn[1 as libc::c_int as
                                usize][0 as libc::c_int as usize] *=
                            ilength_0;
                        tbn[1 as libc::c_int as
                                usize][1 as libc::c_int as usize] *=
                            ilength_0;
                        tbn[1 as libc::c_int as
                                usize][2 as libc::c_int as usize] *=
                            ilength_0;
                        let mut ilength_1: libc::c_float =
                            __tg_sqrt(tbn[2 as libc::c_int as
                                              usize][0 as libc::c_int as
                                                         usize] *
                                          tbn[2 as libc::c_int as
                                                  usize][0 as libc::c_int as
                                                             usize] +
                                          tbn[2 as libc::c_int as
                                                  usize][1 as libc::c_int as
                                                             usize] *
                                              tbn[2 as libc::c_int as
                                                      usize][1 as libc::c_int
                                                                 as usize] +
                                          tbn[2 as libc::c_int as
                                                  usize][2 as libc::c_int as
                                                             usize] *
                                              tbn[2 as libc::c_int as
                                                      usize][2 as libc::c_int
                                                                 as usize]);
                        if ilength_1 != 0. { ilength_1 = 1.0f32 / ilength_1 }
                        tbn[2 as libc::c_int as
                                usize][0 as libc::c_int as usize] *=
                            ilength_1;
                        tbn[2 as libc::c_int as
                                usize][1 as libc::c_int as usize] *=
                            ilength_1;
                        tbn[2 as libc::c_int as
                                usize][2 as libc::c_int as usize] *=
                            ilength_1;
                        dm =
                            (*(*surf).info).deluxemap.offset(((if dt < 0.0f32
                                                                  {
                                                                   (dt -
                                                                        0.5f32)
                                                                       as
                                                                       libc::c_int
                                                               } else {
                                                                   (dt +
                                                                        0.5f32)
                                                                       as
                                                                       libc::c_int
                                                               }) * smax) as
                                                                 isize).offset((if ds
                                                                                       <
                                                                                       0.0f32
                                                                                   {
                                                                                    (ds
                                                                                         -
                                                                                         0.5f32)
                                                                                        as
                                                                                        libc::c_int
                                                                                } else {
                                                                                    (ds
                                                                                         +
                                                                                         0.5f32)
                                                                                        as
                                                                                        libc::c_int
                                                                                })
                                                                                   as
                                                                                   isize)
                    }
                    map = 0 as libc::c_int;
                    while map < 4 as libc::c_int &&
                              (*surf).styles[map as usize] as libc::c_int !=
                                  255 as libc::c_int {
                        let mut scale: uint =
                            tr.lightstylevalue[(*surf).styles[map as usize] as
                                                   usize] as uint;
                        if tr.ignore_lightgamma as u64 != 0 {
                            (*cv).r =
                                (*cv).r.wrapping_add(((*lm).r as
                                                          libc::c_uint).wrapping_mul(scale));
                            (*cv).g =
                                (*cv).g.wrapping_add(((*lm).g as
                                                          libc::c_uint).wrapping_mul(scale));
                            (*cv).b =
                                (*cv).b.wrapping_add(((*lm).b as
                                                          libc::c_uint).wrapping_mul(scale))
                        } else {
                            (*cv).r =
                                (*cv).r.wrapping_add((gEngfuncs.LightToTexGamma.expect("non-null function pointer")((*lm).r)
                                                          as
                                                          libc::c_uint).wrapping_mul(scale));
                            (*cv).g =
                                (*cv).g.wrapping_add((gEngfuncs.LightToTexGamma.expect("non-null function pointer")((*lm).g)
                                                          as
                                                          libc::c_uint).wrapping_mul(scale));
                            (*cv).b =
                                (*cv).b.wrapping_add((gEngfuncs.LightToTexGamma.expect("non-null function pointer")((*lm).b)
                                                          as
                                                          libc::c_uint).wrapping_mul(scale))
                        }
                        lm = lm.offset(size as isize);
                        if !dm.is_null() {
                            let mut srcNormal: vec3_t = [0.; 3];
                            let mut lightNormal: vec3_t = [0.; 3];
                            let mut f: libc::c_float = 1.0f32 / 128.0f32;
                            srcNormal[0 as libc::c_int as usize] =
                                ((*dm).r as libc::c_float - 128.0f32) * f;
                            srcNormal[1 as libc::c_int as usize] =
                                ((*dm).g as libc::c_float - 128.0f32) * f;
                            srcNormal[2 as libc::c_int as usize] =
                                ((*dm).b as libc::c_float - 128.0f32) * f;
                            // skip to next deluxmap
                            Matrix3x4_VectorIRotate(tbn.as_mut_ptr() as
                                                        *const [vec_t; 4],
                                                    srcNormal.as_mut_ptr() as
                                                        *const libc::c_float,
                                                    lightNormal.as_mut_ptr()); // turn to world space
                            lightNormal[0 as libc::c_int as usize] =
                                lightNormal[0 as libc::c_int as usize] *
                                    (scale as libc::c_float *
                                         -1.0f32); // turn direction from light
                            lightNormal[1 as libc::c_int as usize] =
                                lightNormal[1 as libc::c_int as usize] *
                                    (scale as libc::c_float * -1.0f32);
                            lightNormal[2 as libc::c_int as usize] =
                                lightNormal[2 as libc::c_int as usize] *
                                    (scale as libc::c_float * -1.0f32);
                            g_trace_lightvec[0 as libc::c_int as usize] =
                                g_trace_lightvec[0 as libc::c_int as usize] +
                                    lightNormal[0 as libc::c_int as usize];
                            g_trace_lightvec[1 as libc::c_int as usize] =
                                g_trace_lightvec[1 as libc::c_int as usize] +
                                    lightNormal[1 as libc::c_int as usize];
                            g_trace_lightvec[2 as libc::c_int as usize] =
                                g_trace_lightvec[2 as libc::c_int as usize] +
                                    lightNormal[2 as libc::c_int as usize];
                            dm = dm.offset(size as isize)
                        }
                        map += 1
                    }
                    return true_0
                }
            }
        }
        i += 1;
        surf = surf.offset(1)
    }
    // go down back side
    return R_RecursiveLightPoint(model,
                                 (*node).children[(side == 0) as libc::c_int
                                                      as usize], midf, p2f,
                                 cv, mid.as_mut_ptr() as *const vec_t, end);
}
/*
=================
R_LightVec

check bspmodels to get light from
=================
*/
#[no_mangle]
pub unsafe extern "C" fn R_LightVecInternal(mut start: *const vec_t,
                                            mut end: *const vec_t,
                                            mut lspot: *mut vec_t,
                                            mut lvec: *mut vec_t)
 -> colorVec {
    let mut last_fraction: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    let mut maxEnts: libc::c_int = 1 as libc::c_int;
    let mut light: colorVec = colorVec{r: 0, g: 0, b: 0, a: 0,};
    let mut cv: colorVec = colorVec{r: 0, g: 0, b: 0, a: 0,};
    if !lspot.is_null() {
        let ref mut fresh0 = *lspot.offset(2 as libc::c_int as isize);
        *fresh0 = 0 as libc::c_int as vec_t;
        let ref mut fresh1 = *lspot.offset(1 as libc::c_int as isize);
        *fresh1 = *fresh0;
        *lspot.offset(0 as libc::c_int as isize) = *fresh1
    }
    if !lvec.is_null() {
        let ref mut fresh2 = *lvec.offset(2 as libc::c_int as isize);
        *fresh2 = 0 as libc::c_int as vec_t;
        let ref mut fresh3 = *lvec.offset(1 as libc::c_int as isize);
        *fresh3 = *fresh2;
        *lvec.offset(0 as libc::c_int as isize) = *fresh3
    }
    if !gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1 as
                                                                             libc::c_int).is_null()
           &&
           !(*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                   as
                                                                                   libc::c_int)).lightdata.is_null()
       {
        light.a = 0 as libc::c_int as libc::c_uint;
        light.b = light.a;
        light.g = light.b;
        light.r = light.g;
        last_fraction = 1.0f32;
        // get light from bmodels too
        if if !r_lighting_extended.is_null() &&
                  (*r_lighting_extended).value != 0.0f32 {
               true_0 as libc::c_int
           } else { false_0 as libc::c_int } != 0 {
            maxEnts = 600 as libc::c_int
        }
        // check all the bsp-models
        i = 0 as libc::c_int; // skip non-bsp models
        while i < maxEnts {
            let mut pe: *mut physent_t =
                gEngfuncs.EV_GetPhysent.expect("non-null function pointer")(i);
            let mut offset: vec3_t = [0.; 3];
            let mut start_l: vec3_t = [0.; 3];
            let mut end_l: vec3_t = [0.; 3];
            let mut pnodes: *mut mnode_t = 0 as *mut mnode_t;
            let mut matrix: matrix4x4 = [[0.; 4]; 4];
            if pe.is_null() { break ; }
            if !((*pe).model.is_null() ||
                     (*(*pe).model).type_0 as libc::c_int !=
                         mod_brush as libc::c_int) {
                pnodes =
                    &mut *(*(*pe).model).nodes.offset((*(*(*pe).model).hulls.as_mut_ptr().offset(0
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     isize)).firstclipnode
                                                          as isize) as
                        *mut mnode_t;
                offset[0 as libc::c_int as usize] =
                    (*(*pe).model).hulls[0 as libc::c_int as
                                             usize].clip_mins[0 as libc::c_int
                                                                  as usize] -
                        vec3_origin[0 as libc::c_int as usize];
                offset[1 as libc::c_int as usize] =
                    (*(*pe).model).hulls[0 as libc::c_int as
                                             usize].clip_mins[1 as libc::c_int
                                                                  as usize] -
                        vec3_origin[1 as libc::c_int as usize];
                offset[2 as libc::c_int as usize] =
                    (*(*pe).model).hulls[0 as libc::c_int as
                                             usize].clip_mins[2 as libc::c_int
                                                                  as usize] -
                        vec3_origin[2 as libc::c_int as usize];
                offset[0 as libc::c_int as usize] =
                    offset[0 as libc::c_int as usize] +
                        (*pe).origin[0 as libc::c_int as usize];
                offset[1 as libc::c_int as usize] =
                    offset[1 as libc::c_int as usize] +
                        (*pe).origin[1 as libc::c_int as usize];
                offset[2 as libc::c_int as usize] =
                    offset[2 as libc::c_int as usize] +
                        (*pe).origin[2 as libc::c_int as usize];
                start_l[0 as libc::c_int as usize] =
                    *start.offset(0 as libc::c_int as isize) -
                        offset[0 as libc::c_int as usize];
                start_l[1 as libc::c_int as usize] =
                    *start.offset(1 as libc::c_int as isize) -
                        offset[1 as libc::c_int as usize];
                start_l[2 as libc::c_int as usize] =
                    *start.offset(2 as libc::c_int as isize) -
                        offset[2 as libc::c_int as usize];
                end_l[0 as libc::c_int as usize] =
                    *end.offset(0 as libc::c_int as isize) -
                        offset[0 as libc::c_int as usize];
                end_l[1 as libc::c_int as usize] =
                    *end.offset(1 as libc::c_int as isize) -
                        offset[1 as libc::c_int as usize];
                end_l[2 as libc::c_int as usize] =
                    *end.offset(2 as libc::c_int as isize) -
                        offset[2 as libc::c_int as usize];
                // rotate start and end into the models frame of reference
                if !((*pe).angles[0 as libc::c_int as usize] == 0.0f32 &&
                         (*pe).angles[1 as libc::c_int as usize] == 0.0f32 &&
                         (*pe).angles[2 as libc::c_int as usize] == 0.0f32) {
                    Matrix4x4_CreateFromEntity(matrix.as_mut_ptr(),
                                               (*pe).angles.as_mut_ptr() as
                                                   *const vec_t,
                                               offset.as_mut_ptr() as
                                                   *const vec_t,
                                               1.0f32); // didn't hit anything
                    Matrix4x4_VectorITransform(matrix.as_mut_ptr() as
                                                   *const [vec_t; 4], start,
                                               start_l.as_mut_ptr());
                    Matrix4x4_VectorITransform(matrix.as_mut_ptr() as
                                                   *const [vec_t; 4], end,
                                               end_l.as_mut_ptr());
                }
                g_trace_lightspot[2 as libc::c_int as usize] =
                    0 as libc::c_int as vec_t;
                g_trace_lightspot[1 as libc::c_int as usize] =
                    g_trace_lightspot[2 as libc::c_int as usize];
                g_trace_lightspot[0 as libc::c_int as usize] =
                    g_trace_lightspot[1 as libc::c_int as usize];
                g_trace_lightvec[2 as libc::c_int as usize] =
                    0 as libc::c_int as vec_t;
                g_trace_lightvec[1 as libc::c_int as usize] =
                    g_trace_lightvec[2 as libc::c_int as usize];
                g_trace_lightvec[0 as libc::c_int as usize] =
                    g_trace_lightvec[1 as libc::c_int as usize];
                g_trace_fraction = 1.0f32;
                if !(R_RecursiveLightPoint((*pe).model, pnodes, 0.0f32,
                                           1.0f32, &mut cv,
                                           start_l.as_mut_ptr() as
                                               *const vec_t,
                                           end_l.as_mut_ptr() as *const vec_t)
                         as u64 == 0) {
                    if g_trace_fraction < last_fraction {
                        if !lspot.is_null() {
                            *lspot.offset(0 as libc::c_int as isize) =
                                g_trace_lightspot[0 as libc::c_int as usize];
                            *lspot.offset(1 as libc::c_int as isize) =
                                g_trace_lightspot[1 as libc::c_int as usize];
                            *lspot.offset(2 as libc::c_int as isize) =
                                g_trace_lightspot[2 as libc::c_int as usize]
                        }
                        if !lvec.is_null() {
                            let mut ilength: libc::c_float =
                                __tg_sqrt(g_trace_lightvec[0 as libc::c_int as
                                                               usize] *
                                              g_trace_lightvec[0 as
                                                                   libc::c_int
                                                                   as usize] +
                                              g_trace_lightvec[1 as
                                                                   libc::c_int
                                                                   as usize] *
                                                  g_trace_lightvec[1 as
                                                                       libc::c_int
                                                                       as
                                                                       usize]
                                              +
                                              g_trace_lightvec[2 as
                                                                   libc::c_int
                                                                   as usize] *
                                                  g_trace_lightvec[2 as
                                                                       libc::c_int
                                                                       as
                                                                       usize]);
                            if ilength != 0. { ilength = 1.0f32 / ilength }
                            *lvec.offset(0 as libc::c_int as isize) =
                                g_trace_lightvec[0 as libc::c_int as usize] *
                                    ilength;
                            *lvec.offset(1 as libc::c_int as isize) =
                                g_trace_lightvec[1 as libc::c_int as usize] *
                                    ilength;
                            *lvec.offset(2 as libc::c_int as isize) =
                                g_trace_lightvec[2 as libc::c_int as usize] *
                                    ilength
                        }
                        light.r =
                            if (cv.r >> 7 as libc::c_int) <
                                   255 as libc::c_int as libc::c_uint {
                                (cv.r) >> 7 as libc::c_int
                            } else { 255 as libc::c_int as libc::c_uint };
                        light.g =
                            if (cv.g >> 7 as libc::c_int) <
                                   255 as libc::c_int as libc::c_uint {
                                (cv.g) >> 7 as libc::c_int
                            } else { 255 as libc::c_int as libc::c_uint };
                        light.b =
                            if (cv.b >> 7 as libc::c_int) <
                                   255 as libc::c_int as libc::c_uint {
                                (cv.b) >> 7 as libc::c_int
                            } else { 255 as libc::c_int as libc::c_uint };
                        last_fraction = g_trace_fraction;
                        if light.r.wrapping_add(light.g).wrapping_add(light.b)
                               != 0 as libc::c_int as libc::c_uint {
                            break ;
                        }
                        // we get light now
                    }
                }
            }
            i += 1
        }
    } else {
        light.b = 255 as libc::c_int as libc::c_uint;
        light.g = light.b;
        light.r = light.g;
        light.a = 0 as libc::c_int as libc::c_uint
    }
    return light;
}
/*
=================
R_LightVec

check bspmodels to get light from
=================
*/
#[no_mangle]
pub unsafe extern "C" fn R_LightVec(mut start: *const vec_t,
                                    mut end: *const vec_t,
                                    mut lspot: *mut vec_t,
                                    mut lvec: *mut vec_t) -> colorVec {
    let mut light: colorVec = R_LightVecInternal(start, end, lspot, lvec);
    if (if !r_lighting_extended.is_null() &&
               (*r_lighting_extended).value != 0.0f32 {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int }) != 0 && !lspot.is_null() &&
           !lvec.is_null() {
        // trying to get light from ceiling (but ignore gradient analyze)
        if light.r.wrapping_add(light.g).wrapping_add(light.b) ==
               0 as libc::c_int as libc::c_uint {
            return R_LightVecInternal(end, start, lspot, lvec)
        }
    }
    return light;
}
/*
=================
R_LightPoint

light from floor
=================
*/
#[no_mangle]
pub unsafe extern "C" fn R_LightPoint(mut p0: *const vec_t) -> colorVec {
    let mut p1: vec3_t = [0.; 3];
    p1[0 as libc::c_int as usize] = *p0.offset(0 as libc::c_int as isize);
    p1[1 as libc::c_int as usize] = *p0.offset(1 as libc::c_int as isize);
    p1[2 as libc::c_int as usize] =
        *p0.offset(2 as libc::c_int as isize) - 2048.0f32;
    return R_LightVec(p0, p1.as_mut_ptr() as *const vec_t, 0 as *mut vec_t,
                      0 as *mut vec_t);
}
