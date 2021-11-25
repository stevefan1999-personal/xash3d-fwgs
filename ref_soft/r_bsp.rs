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
    fn cosf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn sinf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn R_RenderFace(fa: *mut msurface_t, clipflags: libc::c_int);
    #[no_mangle]
    fn R_RenderBmodelFace(pedges: *mut bedge_t, psurf: *mut msurface_t);
    #[no_mangle]
    fn R_TransformFrustum();
    #[no_mangle]
    static mut RI: ref_instance_t;
    #[no_mangle]
    static mut tr: gl_globals_t;
    #[no_mangle]
    static mut gEngfuncs: ref_api_t;
    #[no_mangle]
    static mut qfrustum: qfrustum_s;
    #[no_mangle]
    static mut r_currentkey: libc::c_int;
    #[no_mangle]
    static mut r_pcurrentvertbase: *mut mvertex_t;
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
pub type C2RustUnnamed = libc::c_int;
pub const PARM_NUMMODELS: C2RustUnnamed = -13;
pub const PARM_NUMENTITIES: C2RustUnnamed = -12;
pub const PARM_LOCAL_GAME: C2RustUnnamed = -11;
pub const PARM_LOCAL_HEALTH: C2RustUnnamed = -10;
pub const PARM_MAX_CLIENTS: C2RustUnnamed = -9;
pub const PARM_WATER_LEVEL: C2RustUnnamed = -8;
pub const PARM_PLAYING_DEMO: C2RustUnnamed = -7;
pub const PARM_CONNSTATE: C2RustUnnamed = -6;
pub const PARM_VIEWENT_INDEX: C2RustUnnamed = -5;
pub const PARM_PLAYER_INDEX: C2RustUnnamed = -4;
pub const PARM_QUAKE_COMPATIBLE: C2RustUnnamed = -3;
pub const PARM_THIRDPERSON: C2RustUnnamed = -2;
pub const PARM_DEV_OVERVIEW: C2RustUnnamed = -1;
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
pub struct vrect_s {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub pnext: *mut vrect_s,
}
pub type vrect_t = vrect_s;
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
    pub viewleaf: *mut mleaf_t,
    pub oldviewleaf: *mut mleaf_t,
    pub pvsorigin: vec3_t,
    pub vieworg: vec3_t,
    pub viewangles: vec3_t,
    pub vforward: vec3_t,
    pub vright: vec3_t,
    pub vup: vec3_t,
    pub base_vup: vec3_t,
    pub base_vpn: vec3_t,
    pub base_vright: vec3_t,
    pub cullorigin: vec3_t,
    pub cull_vforward: vec3_t,
    pub cull_vright: vec3_t,
    pub cull_vup: vec3_t,
    pub cached_contents: libc::c_int,
    pub cached_waterlevel: libc::c_int,
    pub farClip: libc::c_float,
    pub skyMins: [[libc::c_float; 6]; 2],
    pub skyMaxs: [[libc::c_float; 6]; 2],
    pub objectMatrix: matrix4x4,
    pub worldviewMatrix: matrix4x4,
    pub modelviewMatrix: matrix4x4,
    pub projectionMatrix: matrix4x4,
    pub worldviewProjectionMatrix: matrix4x4,
    pub visbytes: [byte; 4096],
    pub viewplanedist: libc::c_float,
    pub vrect: vrect_t,
    pub aliasvrect: vrect_t,
    pub vrectright: libc::c_int,
    pub vrectbottom: libc::c_int,
    pub aliasvrectright: libc::c_int,
    pub aliasvrectbottom: libc::c_int,
    pub vrectrightedge: libc::c_float,
    pub fvrectx: libc::c_float,
    pub fvrecty: libc::c_float,
    pub fvrectx_adj: libc::c_float,
    pub fvrecty_adj: libc::c_float,
    pub vrect_x_adj_shift20: libc::c_int,
    pub vrectright_adj_shift20: libc::c_int,
    pub fvrectright_adj: libc::c_float,
    pub fvrectbottom_adj: libc::c_float,
    pub fvrectright: libc::c_float,
    pub fvrectbottom: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct draw_list_t {
    pub edge_entities: [*mut cl_entity_t; 2048],
    pub solid_entities: [*mut cl_entity_t; 2048],
    pub trans_entities: [*mut cl_entity_t; 2048],
    pub beam_entities: [*mut cl_entity_t; 2048],
    pub num_edge_entities: uint,
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
    pub recursion_level: libc::c_int,
    pub max_recursion: libc::c_int,
    pub visbytes: [byte; 4096],
    pub lightstylevalue: [libc::c_int; 64],
    pub block_size: libc::c_int,
    pub frametime: libc::c_double,
    pub blend: libc::c_float,
    pub modelorg: vec3_t,
    pub fCustomSkybox: qboolean,
    pub sample_size: libc::c_int,
    pub sample_bits: uint,
    pub map_unload: qboolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bedge_s {
    pub v: [*mut mvertex_t; 2],
    pub pnext: *mut bedge_s,
}
pub type bedge_t = bedge_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clipplane_s {
    pub normal: vec3_t,
    pub dist: libc::c_float,
    pub next: *mut clipplane_s,
    pub leftedge: byte,
    pub rightedge: byte,
    pub reserved: [byte; 2],
}
pub type clipplane_t = clipplane_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct qfrustum_s {
    pub screenedge: [mplane_t; 4],
    pub view_clipplanes: [clipplane_t; 4],
    pub frustum_indexes: [libc::c_int; 24],
    pub pfrustum_indexes: [*mut libc::c_int; 4],
}
#[inline(always)]
unsafe extern "C" fn __tg_cos(mut __x: libc::c_float) -> libc::c_float {
    return cosf(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_sin(mut __x: libc::c_float) -> libc::c_float {
    return sinf(__x);
}
/*
Copyright (C) 1997-2001 Id Software, Inc.

This program is free software; you can redistribute it and/or
modify it under the terms of the GNU General Public License
as published by the Free Software Foundation; either version 2
of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.

See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program; if not, write to the Free Software
Foundation, Inc., 59 Temple Place - Suite 330, Boston, MA  02111-1307, USA.

*/
// r_bsp.c
//
// current entity info
//
#[no_mangle]
pub static mut r_entorigin: vec3_t = [0.; 3];
// the currently rendering entity in world
// coordinates
#[no_mangle]
pub static mut entity_rotation: [[libc::c_float; 3]; 3] = [[0.; 3]; 3];
#[no_mangle]
pub static mut r_currentbkey: libc::c_int = 0;
// 24K
static mut pbverts: *mut mvertex_t = 0 as *const mvertex_t as *mut mvertex_t;
static mut pbedges: *mut bedge_t = 0 as *const bedge_t as *mut bedge_t;
static mut numbverts: libc::c_int = 0;
static mut numbedges: libc::c_int = 0;
static mut pfrontenter: *mut mvertex_t =
    0 as *const mvertex_t as *mut mvertex_t;
static mut pfrontexit: *mut mvertex_t =
    0 as *const mvertex_t as *mut mvertex_t;
static mut makeclippededge: qboolean = false_0;
/*
================
R_ConcatRotations
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_ConcatRotations(mut in1: *mut [libc::c_float; 3],
                                           mut in2: *mut [libc::c_float; 3],
                                           mut out: *mut [libc::c_float; 3]) {
    (*out.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (*in1.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] *
            (*in2.offset(0 as libc::c_int as
                             isize))[0 as libc::c_int as usize] +
            (*in1.offset(0 as libc::c_int as
                             isize))[1 as libc::c_int as usize] *
                (*in2.offset(1 as libc::c_int as
                                 isize))[0 as libc::c_int as usize] +
            (*in1.offset(0 as libc::c_int as
                             isize))[2 as libc::c_int as usize] *
                (*in2.offset(2 as libc::c_int as
                                 isize))[0 as libc::c_int as usize];
    (*out.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] =
        (*in1.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] *
            (*in2.offset(0 as libc::c_int as
                             isize))[1 as libc::c_int as usize] +
            (*in1.offset(0 as libc::c_int as
                             isize))[1 as libc::c_int as usize] *
                (*in2.offset(1 as libc::c_int as
                                 isize))[1 as libc::c_int as usize] +
            (*in1.offset(0 as libc::c_int as
                             isize))[2 as libc::c_int as usize] *
                (*in2.offset(2 as libc::c_int as
                                 isize))[1 as libc::c_int as usize];
    (*out.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize] =
        (*in1.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] *
            (*in2.offset(0 as libc::c_int as
                             isize))[2 as libc::c_int as usize] +
            (*in1.offset(0 as libc::c_int as
                             isize))[1 as libc::c_int as usize] *
                (*in2.offset(1 as libc::c_int as
                                 isize))[2 as libc::c_int as usize] +
            (*in1.offset(0 as libc::c_int as
                             isize))[2 as libc::c_int as usize] *
                (*in2.offset(2 as libc::c_int as
                                 isize))[2 as libc::c_int as usize];
    (*out.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (*in1.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] *
            (*in2.offset(0 as libc::c_int as
                             isize))[0 as libc::c_int as usize] +
            (*in1.offset(1 as libc::c_int as
                             isize))[1 as libc::c_int as usize] *
                (*in2.offset(1 as libc::c_int as
                                 isize))[0 as libc::c_int as usize] +
            (*in1.offset(1 as libc::c_int as
                             isize))[2 as libc::c_int as usize] *
                (*in2.offset(2 as libc::c_int as
                                 isize))[0 as libc::c_int as usize];
    (*out.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] =
        (*in1.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] *
            (*in2.offset(0 as libc::c_int as
                             isize))[1 as libc::c_int as usize] +
            (*in1.offset(1 as libc::c_int as
                             isize))[1 as libc::c_int as usize] *
                (*in2.offset(1 as libc::c_int as
                                 isize))[1 as libc::c_int as usize] +
            (*in1.offset(1 as libc::c_int as
                             isize))[2 as libc::c_int as usize] *
                (*in2.offset(2 as libc::c_int as
                                 isize))[1 as libc::c_int as usize];
    (*out.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize] =
        (*in1.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] *
            (*in2.offset(0 as libc::c_int as
                             isize))[2 as libc::c_int as usize] +
            (*in1.offset(1 as libc::c_int as
                             isize))[1 as libc::c_int as usize] *
                (*in2.offset(1 as libc::c_int as
                                 isize))[2 as libc::c_int as usize] +
            (*in1.offset(1 as libc::c_int as
                             isize))[2 as libc::c_int as usize] *
                (*in2.offset(2 as libc::c_int as
                                 isize))[2 as libc::c_int as usize];
    (*out.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (*in1.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] *
            (*in2.offset(0 as libc::c_int as
                             isize))[0 as libc::c_int as usize] +
            (*in1.offset(2 as libc::c_int as
                             isize))[1 as libc::c_int as usize] *
                (*in2.offset(1 as libc::c_int as
                                 isize))[0 as libc::c_int as usize] +
            (*in1.offset(2 as libc::c_int as
                             isize))[2 as libc::c_int as usize] *
                (*in2.offset(2 as libc::c_int as
                                 isize))[0 as libc::c_int as usize];
    (*out.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize] =
        (*in1.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] *
            (*in2.offset(0 as libc::c_int as
                             isize))[1 as libc::c_int as usize] +
            (*in1.offset(2 as libc::c_int as
                             isize))[1 as libc::c_int as usize] *
                (*in2.offset(1 as libc::c_int as
                                 isize))[1 as libc::c_int as usize] +
            (*in1.offset(2 as libc::c_int as
                             isize))[2 as libc::c_int as usize] *
                (*in2.offset(2 as libc::c_int as
                                 isize))[1 as libc::c_int as usize];
    (*out.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize] =
        (*in1.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] *
            (*in2.offset(0 as libc::c_int as
                             isize))[2 as libc::c_int as usize] +
            (*in1.offset(2 as libc::c_int as
                             isize))[1 as libc::c_int as usize] *
                (*in2.offset(1 as libc::c_int as
                                 isize))[2 as libc::c_int as usize] +
            (*in1.offset(2 as libc::c_int as
                             isize))[2 as libc::c_int as usize] *
                (*in2.offset(2 as libc::c_int as
                                 isize))[2 as libc::c_int as usize];
}
//===========================================================================
/*
================
R_EntityRotate
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_EntityRotate(mut vec: *mut vec_t) {
    let mut tvec: vec3_t = [0.; 3];
    tvec[0 as libc::c_int as usize] = *vec.offset(0 as libc::c_int as isize);
    tvec[1 as libc::c_int as usize] = *vec.offset(1 as libc::c_int as isize);
    tvec[2 as libc::c_int as usize] = *vec.offset(2 as libc::c_int as isize);
    *vec.offset(0 as libc::c_int as isize) =
        entity_rotation[0 as libc::c_int as usize][0 as libc::c_int as usize]
            * tvec[0 as libc::c_int as usize] +
            entity_rotation[0 as libc::c_int as
                                usize][1 as libc::c_int as usize] *
                tvec[1 as libc::c_int as usize] +
            entity_rotation[0 as libc::c_int as
                                usize][2 as libc::c_int as usize] *
                tvec[2 as libc::c_int as usize];
    *vec.offset(1 as libc::c_int as isize) =
        entity_rotation[1 as libc::c_int as usize][0 as libc::c_int as usize]
            * tvec[0 as libc::c_int as usize] +
            entity_rotation[1 as libc::c_int as
                                usize][1 as libc::c_int as usize] *
                tvec[1 as libc::c_int as usize] +
            entity_rotation[1 as libc::c_int as
                                usize][2 as libc::c_int as usize] *
                tvec[2 as libc::c_int as usize];
    *vec.offset(2 as libc::c_int as isize) =
        entity_rotation[2 as libc::c_int as usize][0 as libc::c_int as usize]
            * tvec[0 as libc::c_int as usize] +
            entity_rotation[2 as libc::c_int as
                                usize][1 as libc::c_int as usize] *
                tvec[1 as libc::c_int as usize] +
            entity_rotation[2 as libc::c_int as
                                usize][2 as libc::c_int as usize] *
                tvec[2 as libc::c_int as usize];
}
/*
================
R_RotateBmodel
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_RotateBmodel() {
    let mut angle: libc::c_float = 0.;
    let mut s: libc::c_float = 0.;
    let mut c: libc::c_float = 0.;
    let mut temp1: [[libc::c_float; 3]; 3] = [[0.; 3]; 3];
    let mut temp2: [[libc::c_float; 3]; 3] = [[0.; 3]; 3];
    let mut temp3: [[libc::c_float; 3]; 3] = [[0.; 3]; 3];
    // TODO: should use a look-up table
// TODO: should really be stored with the entity instead of being reconstructed
// TODO: could cache lazily, stored in the entity
// TODO: share work with R_SetUpAliasTransform
    // yaw
    angle = (*RI.currententity).angles[1 as libc::c_int as usize];
    angle =
        angle * 3.14159265358979323846f64 as libc::c_float *
            2 as libc::c_int as libc::c_float / 360.0f32;
    s = __tg_sin(angle);
    c = __tg_cos(angle);
    temp1[0 as libc::c_int as usize][0 as libc::c_int as usize] = c;
    temp1[0 as libc::c_int as usize][1 as libc::c_int as usize] = s;
    temp1[0 as libc::c_int as usize][2 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_float;
    temp1[1 as libc::c_int as usize][0 as libc::c_int as usize] = -s;
    temp1[1 as libc::c_int as usize][1 as libc::c_int as usize] = c;
    temp1[1 as libc::c_int as usize][2 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_float;
    temp1[2 as libc::c_int as usize][0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_float;
    temp1[2 as libc::c_int as usize][1 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_float;
    temp1[2 as libc::c_int as usize][2 as libc::c_int as usize] =
        1 as libc::c_int as libc::c_float;
    // pitch
    angle = (*RI.currententity).angles[0 as libc::c_int as usize];
    angle =
        angle * 3.14159265358979323846f64 as libc::c_float *
            2 as libc::c_int as libc::c_float / 360.0f32;
    s = __tg_sin(angle);
    c = __tg_cos(angle);
    temp2[0 as libc::c_int as usize][0 as libc::c_int as usize] = c;
    temp2[0 as libc::c_int as usize][1 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_float;
    temp2[0 as libc::c_int as usize][2 as libc::c_int as usize] = -s;
    temp2[1 as libc::c_int as usize][0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_float;
    temp2[1 as libc::c_int as usize][1 as libc::c_int as usize] =
        1 as libc::c_int as libc::c_float;
    temp2[1 as libc::c_int as usize][2 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_float;
    temp2[2 as libc::c_int as usize][0 as libc::c_int as usize] = s;
    temp2[2 as libc::c_int as usize][1 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_float;
    temp2[2 as libc::c_int as usize][2 as libc::c_int as usize] = c;
    R_ConcatRotations(temp2.as_mut_ptr(), temp1.as_mut_ptr(),
                      temp3.as_mut_ptr());
    // roll
    angle = (*RI.currententity).angles[2 as libc::c_int as usize];
    angle =
        angle * 3.14159265358979323846f64 as libc::c_float *
            2 as libc::c_int as libc::c_float / 360.0f32;
    s = __tg_sin(angle);
    c = __tg_cos(angle);
    temp1[0 as libc::c_int as usize][0 as libc::c_int as usize] =
        1 as libc::c_int as libc::c_float;
    temp1[0 as libc::c_int as usize][1 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_float;
    temp1[0 as libc::c_int as usize][2 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_float;
    temp1[1 as libc::c_int as usize][0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_float;
    temp1[1 as libc::c_int as usize][1 as libc::c_int as usize] = c;
    temp1[1 as libc::c_int as usize][2 as libc::c_int as usize] = s;
    temp1[2 as libc::c_int as usize][0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_float;
    temp1[2 as libc::c_int as usize][1 as libc::c_int as usize] = -s;
    temp1[2 as libc::c_int as usize][2 as libc::c_int as usize] = c;
    R_ConcatRotations(temp1.as_mut_ptr(), temp3.as_mut_ptr(),
                      entity_rotation.as_mut_ptr());
    //
// rotate modelorg and the transformation matrix
//
    R_EntityRotate(tr.modelorg.as_mut_ptr());
    R_EntityRotate(RI.vforward.as_mut_ptr());
    R_EntityRotate(RI.vright.as_mut_ptr());
    R_EntityRotate(RI.vup.as_mut_ptr());
    R_TransformFrustum();
}
/*
================
R_RecursiveClipBPoly
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_RecursiveClipBPoly(mut pedges: *mut bedge_t,
                                              mut pnode: *mut mnode_t,
                                              mut psurf: *mut msurface_t) {
    let mut psideedges: [*mut bedge_t; 2] = [0 as *mut bedge_t; 2];
    let mut pnextedge: *mut bedge_t = 0 as *mut bedge_t;
    let mut ptedge: *mut bedge_t = 0 as *mut bedge_t;
    let mut i: libc::c_int = 0;
    let mut side: libc::c_int = 0;
    let mut lastside: libc::c_int = 0;
    let mut dist: libc::c_float = 0.;
    let mut frac: libc::c_float = 0.;
    let mut lastdist: libc::c_float = 0.;
    let mut splitplane: *mut mplane_t = 0 as *mut mplane_t;
    let mut tplane: mplane_t =
        mplane_t{normal: [0.; 3],
                 dist: 0.,
                 type_0: 0,
                 signbits: 0,
                 pad: [0; 2],};
    let mut pvert: *mut mvertex_t = 0 as *mut mvertex_t;
    let mut plastvert: *mut mvertex_t = 0 as *mut mvertex_t;
    let mut ptvert: *mut mvertex_t = 0 as *mut mvertex_t;
    let mut pn: *mut mnode_t = 0 as *mut mnode_t;
    psideedges[1 as libc::c_int as usize] = 0 as *mut bedge_t;
    psideedges[0 as libc::c_int as usize] =
        psideedges[1 as libc::c_int as usize];
    makeclippededge = false_0;
    // transform the BSP plane into model space
// FIXME: cache these?
    splitplane = (*pnode).plane;
    tplane.dist =
        (*splitplane).dist -
            (r_entorigin[0 as libc::c_int as usize] *
                 (*splitplane).normal[0 as libc::c_int as usize] +
                 r_entorigin[1 as libc::c_int as usize] *
                     (*splitplane).normal[1 as libc::c_int as usize] +
                 r_entorigin[2 as libc::c_int as usize] *
                     (*splitplane).normal[2 as libc::c_int as usize]);
    tplane.normal[0 as libc::c_int as usize] =
        entity_rotation[0 as libc::c_int as usize][0 as libc::c_int as usize]
            * (*splitplane).normal[0 as libc::c_int as usize] +
            entity_rotation[0 as libc::c_int as
                                usize][1 as libc::c_int as usize] *
                (*splitplane).normal[1 as libc::c_int as usize] +
            entity_rotation[0 as libc::c_int as
                                usize][2 as libc::c_int as usize] *
                (*splitplane).normal[2 as libc::c_int as usize];
    tplane.normal[1 as libc::c_int as usize] =
        entity_rotation[1 as libc::c_int as usize][0 as libc::c_int as usize]
            * (*splitplane).normal[0 as libc::c_int as usize] +
            entity_rotation[1 as libc::c_int as
                                usize][1 as libc::c_int as usize] *
                (*splitplane).normal[1 as libc::c_int as usize] +
            entity_rotation[1 as libc::c_int as
                                usize][2 as libc::c_int as usize] *
                (*splitplane).normal[2 as libc::c_int as usize];
    tplane.normal[2 as libc::c_int as usize] =
        entity_rotation[2 as libc::c_int as usize][0 as libc::c_int as usize]
            * (*splitplane).normal[0 as libc::c_int as usize] +
            entity_rotation[2 as libc::c_int as
                                usize][1 as libc::c_int as usize] *
                (*splitplane).normal[1 as libc::c_int as usize] +
            entity_rotation[2 as libc::c_int as
                                usize][2 as libc::c_int as usize] *
                (*splitplane).normal[2 as libc::c_int as usize];
    // clip edges to BSP plane
    while !pedges.is_null() {
        pnextedge = (*pedges).pnext;
        // set the status for the last point as the previous point
	// FIXME: cache this stuff somehow?
        plastvert = (*pedges).v[0 as libc::c_int as usize];
        lastdist =
            (*plastvert).position[0 as libc::c_int as usize] *
                tplane.normal[0 as libc::c_int as usize] +
                (*plastvert).position[1 as libc::c_int as usize] *
                    tplane.normal[1 as libc::c_int as usize] +
                (*plastvert).position[2 as libc::c_int as usize] *
                    tplane.normal[2 as libc::c_int as usize] - tplane.dist;
        if lastdist > 0 as libc::c_int as libc::c_float {
            lastside = 0 as libc::c_int
        } else { lastside = 1 as libc::c_int }
        pvert = (*pedges).v[1 as libc::c_int as usize];
        dist =
            (*pvert).position[0 as libc::c_int as usize] *
                tplane.normal[0 as libc::c_int as usize] +
                (*pvert).position[1 as libc::c_int as usize] *
                    tplane.normal[1 as libc::c_int as usize] +
                (*pvert).position[2 as libc::c_int as usize] *
                    tplane.normal[2 as libc::c_int as usize] - tplane.dist;
        if dist > 0 as libc::c_int as libc::c_float {
            side = 0 as libc::c_int
        } else { side = 1 as libc::c_int }
        if side != lastside {
            // clipped
            if numbverts >= 1000 as libc::c_int { return }
            // generate the clipped vertex
            frac = lastdist / (lastdist - dist);
            let fresh0 = numbverts;
            numbverts = numbverts + 1;
            ptvert = &mut *pbverts.offset(fresh0 as isize) as *mut mvertex_t;
            (*ptvert).position[0 as libc::c_int as usize] =
                (*plastvert).position[0 as libc::c_int as usize] +
                    frac *
                        ((*pvert).position[0 as libc::c_int as usize] -
                             (*plastvert).position[0 as libc::c_int as
                                                       usize]);
            (*ptvert).position[1 as libc::c_int as usize] =
                (*plastvert).position[1 as libc::c_int as usize] +
                    frac *
                        ((*pvert).position[1 as libc::c_int as usize] -
                             (*plastvert).position[1 as libc::c_int as
                                                       usize]);
            (*ptvert).position[2 as libc::c_int as usize] =
                (*plastvert).position[2 as libc::c_int as usize] +
                    frac *
                        ((*pvert).position[2 as libc::c_int as usize] -
                             (*plastvert).position[2 as libc::c_int as
                                                       usize]);
            // split into two edges, one on each side, and remember entering
		// and exiting points
		// FIXME: share the clip edge by having a winding direction flag?
            if numbedges >= 2000 as libc::c_int - 1 as libc::c_int {
                //gEngfuncs.Con_Printf ("Out of edges for bmodel\n");
                return
            }
            ptedge = &mut *pbedges.offset(numbedges as isize) as *mut bedge_t;
            (*ptedge).pnext = psideedges[lastside as usize];
            psideedges[lastside as usize] = ptedge;
            (*ptedge).v[0 as libc::c_int as usize] = plastvert;
            (*ptedge).v[1 as libc::c_int as usize] = ptvert;
            ptedge =
                &mut *pbedges.offset((numbedges + 1 as libc::c_int) as isize)
                    as *mut bedge_t;
            (*ptedge).pnext = psideedges[side as usize];
            psideedges[side as usize] = ptedge;
            (*ptedge).v[0 as libc::c_int as usize] = ptvert;
            (*ptedge).v[1 as libc::c_int as usize] = pvert;
            numbedges += 2 as libc::c_int;
            if side == 0 as libc::c_int {
                // entering for front, exiting for back
                pfrontenter = ptvert;
                makeclippededge = true_0
            } else { pfrontexit = ptvert; makeclippededge = true_0 }
        } else {
            // add the edge to the appropriate side
            (*pedges).pnext = psideedges[side as usize];
            psideedges[side as usize] = pedges
        }
        pedges = pnextedge
    }
    // if anything was clipped, reconstitute and add the edges along the clip
// plane to both sides (but in opposite directions)
    if makeclippededge as u64 != 0 {
        if numbedges >= 2000 as libc::c_int - 2 as libc::c_int {
            //gEngfuncs.Con_Printf ("Out of edges for bmodel\n");
            return
        }
        ptedge = &mut *pbedges.offset(numbedges as isize) as *mut bedge_t;
        (*ptedge).pnext = psideedges[0 as libc::c_int as usize];
        psideedges[0 as libc::c_int as usize] = ptedge;
        (*ptedge).v[0 as libc::c_int as usize] = pfrontexit;
        (*ptedge).v[1 as libc::c_int as usize] = pfrontenter;
        ptedge =
            &mut *pbedges.offset((numbedges + 1 as libc::c_int) as isize) as
                *mut bedge_t;
        (*ptedge).pnext = psideedges[1 as libc::c_int as usize];
        psideedges[1 as libc::c_int as usize] = ptedge;
        (*ptedge).v[0 as libc::c_int as usize] = pfrontenter;
        (*ptedge).v[1 as libc::c_int as usize] = pfrontexit;
        numbedges += 2 as libc::c_int
    }
    // draw or recurse further
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        if !psideedges[i as usize].is_null() {
            // draw if we've reached a non-solid leaf, done if all that's left is a
		// solid leaf, and continue down the tree if it's not a leaf
            pn = (*pnode).children[i as usize];
            // we're done with this branch if the node or leaf isn't in the PVS
            if (*pn).visframe == tr.visframecount {
                if (*pn).contents < 0 as libc::c_int {
                    if (*pn).contents != -(2 as libc::c_int) {
                        //r_currentbkey = ((mleaf_t *)pn)->cluster;
                        r_currentbkey =
                            r_leafkeys[(pn as
                                            *mut mleaf_t).wrapping_offset_from((*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                                                                                      as
                                                                                                                                                      libc::c_int)).leafs)
                                           as libc::c_long as usize];
                        R_RenderBmodelFace(psideedges[i as usize], psurf);
                    }
                } else {
                    R_RecursiveClipBPoly(psideedges[i as usize],
                                         (*pnode).children[i as usize],
                                         psurf);
                }
            }
        }
        i += 1
    };
}
/*
================
R_DrawSolidClippedSubmodelPolygons

Bmodel crosses multiple leafs
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_DrawSolidClippedSubmodelPolygons(mut pmodel:
                                                                *mut model_t,
                                                            mut topnode:
                                                                *mut mnode_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut lindex: libc::c_int = 0;
    let mut dot: vec_t = 0.;
    let mut psurf: *mut msurface_t = 0 as *mut msurface_t;
    let mut numsurfaces: libc::c_int = 0;
    let mut pplane: *mut mplane_t = 0 as *mut mplane_t;
    let mut bverts: [mvertex_t; 1000] = [mvertex_t{position: [0.; 3],}; 1000];
    let mut bedges: [bedge_t; 2000] =
        [bedge_t{v: [0 as *mut mvertex_t; 2], pnext: 0 as *mut bedge_s,};
            2000];
    let mut pbedge: *mut bedge_t = 0 as *mut bedge_t;
    let mut pedge: *mut medge_t = 0 as *mut medge_t;
    let mut pedges: *mut medge_t = 0 as *mut medge_t;
    // FIXME: use bounding-box-based frustum clipping info?
    psurf =
        &mut *(*pmodel).surfaces.offset((*pmodel).firstmodelsurface as isize)
            as *mut msurface_t;
    numsurfaces = (*pmodel).nummodelsurfaces;
    pedges = (*pmodel).edges;
    let mut current_block_27: u64;
    i = 0 as libc::c_int;
    while i < numsurfaces {
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
                   (*RI.currententity).curstate.effects as libc::c_uint &
                       (1 as libc::c_uint) << 26 as libc::c_int == 0 {
                current_block_27 = 10886091980245723256;
            } else if r_entorigin[2 as libc::c_int as usize] +
                          (*pmodel).mins[2 as libc::c_int as usize] + 1.0f32
                          >= (*(*psurf).plane).dist {
                current_block_27 = 10886091980245723256;
            } else { current_block_27 = 11050875288958768710; }
        } else { current_block_27 = 11050875288958768710; }
        match current_block_27 {
            11050875288958768710 => {
                //else
		//	R_RenderBmodelFace( pbedge, psurf );
                // find which side of the node we are on
                pplane = (*psurf).plane;
                dot =
                    tr.modelorg[0 as libc::c_int as usize] *
                        (*pplane).normal[0 as libc::c_int as usize] +
                        tr.modelorg[1 as libc::c_int as usize] *
                            (*pplane).normal[1 as libc::c_int as usize] +
                        tr.modelorg[2 as libc::c_int as usize] *
                            (*pplane).normal[2 as libc::c_int as usize] -
                        (*pplane).dist;
                // draw the polygon
                if !((*psurf).flags as libc::c_uint &
                         (1 as libc::c_uint) << 1 as libc::c_int == 0 &&
                         dot < -0.01f32 ||
                         (*psurf).flags as libc::c_uint &
                             (1 as libc::c_uint) << 1 as libc::c_int != 0 &&
                             dot > 0.01f32) {
                    // FIXME: use bounding-box-based frustum clipping info?
                    // copy the edges to bedges, flipping if necessary so always
	// clockwise winding
	// FIXME: if edges and vertices get caches, these assignments must move
	// outside the loop, and overflow checking must be done here
                    pbverts = bverts.as_mut_ptr(); // mark end of edges
                    pbedges = bedges.as_mut_ptr();
                    numbedges = 0 as libc::c_int;
                    numbverts = numbedges;
                    pbedge =
                        &mut *bedges.as_mut_ptr().offset(numbedges as isize)
                            as *mut bedge_t;
                    numbedges += (*psurf).numedges;
                    j = 0 as libc::c_int;
                    while j < (*psurf).numedges {
                        lindex =
                            *(*pmodel).surfedges.offset(((*psurf).firstedge +
                                                             j) as isize);
                        if lindex > 0 as libc::c_int {
                            pedge =
                                &mut *pedges.offset(lindex as isize) as
                                    *mut medge_t;
                            let ref mut fresh1 =
                                (*pbedge.offset(j as
                                                    isize)).v[0 as libc::c_int
                                                                  as usize];
                            *fresh1 =
                                &mut *r_pcurrentvertbase.offset(*(*pedge).v.as_mut_ptr().offset(0
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    isize)
                                                                    as isize)
                                    as *mut mvertex_t;
                            let ref mut fresh2 =
                                (*pbedge.offset(j as
                                                    isize)).v[1 as libc::c_int
                                                                  as usize];
                            *fresh2 =
                                &mut *r_pcurrentvertbase.offset(*(*pedge).v.as_mut_ptr().offset(1
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    isize)
                                                                    as isize)
                                    as *mut mvertex_t
                        } else {
                            lindex = -lindex;
                            pedge =
                                &mut *pedges.offset(lindex as isize) as
                                    *mut medge_t;
                            let ref mut fresh3 =
                                (*pbedge.offset(j as
                                                    isize)).v[0 as libc::c_int
                                                                  as usize];
                            *fresh3 =
                                &mut *r_pcurrentvertbase.offset(*(*pedge).v.as_mut_ptr().offset(1
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    isize)
                                                                    as isize)
                                    as *mut mvertex_t;
                            let ref mut fresh4 =
                                (*pbedge.offset(j as
                                                    isize)).v[1 as libc::c_int
                                                                  as usize];
                            *fresh4 =
                                &mut *r_pcurrentvertbase.offset(*(*pedge).v.as_mut_ptr().offset(0
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    isize)
                                                                    as isize)
                                    as *mut mvertex_t
                        }
                        let ref mut fresh5 =
                            (*pbedge.offset(j as isize)).pnext;
                        *fresh5 =
                            &mut *pbedge.offset((j + 1 as libc::c_int) as
                                                    isize) as *mut bedge_t;
                        j += 1
                    }
                    let ref mut fresh6 =
                        (*pbedge.offset((j - 1 as libc::c_int) as
                                            isize)).pnext;
                    *fresh6 = 0 as *mut bedge_s;
                    //if ( !( psurf->texinfo->flags & ( SURF_TRANS66 | SURF_TRANS33 ) ) )
                    R_RecursiveClipBPoly(pbedge, topnode, psurf);
                }
            }
            _ => { }
        }
        i += 1;
        psurf = psurf.offset(1)
    };
}
/*
================
R_DrawSubmodelPolygons

All in one leaf
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_DrawSubmodelPolygons(mut pmodel: *mut model_t,
                                                mut clipflags: libc::c_int,
                                                mut topnode: *mut mnode_t) {
    let mut i: libc::c_int = 0;
    let mut dot: vec_t = 0.;
    let mut psurf: *mut msurface_t = 0 as *mut msurface_t;
    let mut numsurfaces: libc::c_int = 0;
    let mut pplane: *mut mplane_t = 0 as *mut mplane_t;
    // FIXME: use bounding-box-based frustum clipping info?
    psurf =
        &mut *(*pmodel).surfaces.offset((*pmodel).firstmodelsurface as isize)
            as *mut msurface_t;
    numsurfaces = (*pmodel).nummodelsurfaces;
    let mut current_block_8: u64;
    i = 0 as libc::c_int;
    while i < numsurfaces {
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
                   (*RI.currententity).curstate.effects as libc::c_uint &
                       (1 as libc::c_uint) << 26 as libc::c_int == 0 {
                current_block_8 = 15240798224410183470;
            } else if r_entorigin[2 as libc::c_int as usize] +
                          (*pmodel).mins[2 as libc::c_int as usize] + 1.0f32
                          >= (*(*psurf).plane).dist {
                current_block_8 = 15240798224410183470;
            } else { current_block_8 = 13109137661213826276; }
        } else { current_block_8 = 13109137661213826276; }
        match current_block_8 {
            13109137661213826276 => {
                // find which side of the node we are on
                pplane = (*psurf).plane;
                dot =
                    tr.modelorg[0 as libc::c_int as usize] *
                        (*pplane).normal[0 as libc::c_int as usize] +
                        tr.modelorg[1 as libc::c_int as usize] *
                            (*pplane).normal[1 as libc::c_int as usize] +
                        tr.modelorg[2 as libc::c_int as usize] *
                            (*pplane).normal[2 as libc::c_int as usize] -
                        (*pplane).dist;
                // draw the polygon
                if (*psurf).flags as libc::c_uint &
                       (1 as libc::c_uint) << 1 as libc::c_int != 0 &&
                       dot < -0.01f32 ||
                       (*psurf).flags as libc::c_uint &
                           (1 as libc::c_uint) << 1 as libc::c_int == 0 &&
                           dot > 0.01f32 {
                    r_currentkey =
                        r_leafkeys[(topnode as
                                        *mut mleaf_t).wrapping_offset_from((*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                                                                                  as
                                                                                                                                                  libc::c_int)).leafs)
                                       as libc::c_long as usize];
                    // FIXME: use bounding-box-based frustum clipping info?
                    R_RenderFace(psurf, clipflags);
                }
            }
            _ => { }
        }
        i += 1;
        psurf = psurf.offset(1)
    };
}
#[no_mangle]
pub static mut c_drawnode: libc::c_int = 0;
#[no_mangle]
pub static mut r_leafkeys: [libc::c_int; 32767] = [0; 32767];
/*
================
R_RecursiveWorldNode
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_RecursiveWorldNode(mut node: *mut mnode_t,
                                              mut clipflags: libc::c_int) {
    let mut i: libc::c_int = 0; // solid
    let mut c: libc::c_int = 0;
    let mut side: libc::c_int = 0;
    let mut pindex: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut acceptpt: vec3_t = [0.; 3];
    let mut rejectpt: vec3_t = [0.; 3];
    let mut plane: *mut mplane_t = 0 as *mut mplane_t;
    let mut surf: *mut msurface_t = 0 as *mut msurface_t;
    let mut mark: *mut *mut msurface_t = 0 as *mut *mut msurface_t;
    let mut pleaf: *mut mleaf_t = 0 as *mut mleaf_t;
    let mut d: libc::c_double = 0.;
    let mut dot: libc::c_double = 0.;
    if (*node).contents == -(2 as libc::c_int) { return }
    if (*node).visframe != tr.visframecount { return }
    // cull the clipping planes if not trivial accept
// FIXME: the compiler is doing a lousy job of optimizing here; it could be
//  twice as fast in ASM
    if clipflags != 0 {
        i = 0 as libc::c_int; // don't need to clip against it
        while i < 4 as libc::c_int {
            if !(clipflags & (1 as libc::c_int) << i == 0) {
                // generate accept and reject points
		// FIXME: do with fast look-ups or integer tests based on the sign bit
		// of the floating point values
                pindex = qfrustum.pfrustum_indexes[i as usize];
                rejectpt[0 as libc::c_int as usize] =
                    (*node).minmaxs[*pindex.offset(0 as libc::c_int as isize)
                                        as usize];
                rejectpt[1 as libc::c_int as usize] =
                    (*node).minmaxs[*pindex.offset(1 as libc::c_int as isize)
                                        as usize];
                rejectpt[2 as libc::c_int as usize] =
                    (*node).minmaxs[*pindex.offset(2 as libc::c_int as isize)
                                        as usize];
                d =
                    (rejectpt[0 as libc::c_int as usize] *
                         qfrustum.view_clipplanes[i as
                                                      usize].normal[0 as
                                                                        libc::c_int
                                                                        as
                                                                        usize]
                         +
                         rejectpt[1 as libc::c_int as usize] *
                             qfrustum.view_clipplanes[i as
                                                          usize].normal[1 as
                                                                            libc::c_int
                                                                            as
                                                                            usize]
                         +
                         rejectpt[2 as libc::c_int as usize] *
                             qfrustum.view_clipplanes[i as
                                                          usize].normal[2 as
                                                                            libc::c_int
                                                                            as
                                                                            usize])
                        as libc::c_double;
                d -=
                    qfrustum.view_clipplanes[i as usize].dist as
                        libc::c_double;
                if d <= 0 as libc::c_int as libc::c_double { return }
                acceptpt[0 as libc::c_int as usize] =
                    (*node).minmaxs[*pindex.offset((3 as libc::c_int +
                                                        0 as libc::c_int) as
                                                       isize) as usize];
                acceptpt[1 as libc::c_int as usize] =
                    (*node).minmaxs[*pindex.offset((3 as libc::c_int +
                                                        1 as libc::c_int) as
                                                       isize) as usize];
                acceptpt[2 as libc::c_int as usize] =
                    (*node).minmaxs[*pindex.offset((3 as libc::c_int +
                                                        2 as libc::c_int) as
                                                       isize) as usize];
                d =
                    (acceptpt[0 as libc::c_int as usize] *
                         qfrustum.view_clipplanes[i as
                                                      usize].normal[0 as
                                                                        libc::c_int
                                                                        as
                                                                        usize]
                         +
                         acceptpt[1 as libc::c_int as usize] *
                             qfrustum.view_clipplanes[i as
                                                          usize].normal[1 as
                                                                            libc::c_int
                                                                            as
                                                                            usize]
                         +
                         acceptpt[2 as libc::c_int as usize] *
                             qfrustum.view_clipplanes[i as
                                                          usize].normal[2 as
                                                                            libc::c_int
                                                                            as
                                                                            usize])
                        as libc::c_double;
                d -=
                    qfrustum.view_clipplanes[i as usize].dist as
                        libc::c_double;
                if d >= 0 as libc::c_int as libc::c_double {
                    clipflags &= !((1 as libc::c_int) << i)
                }
            }
            i += 1
            // node is entirely on screen
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
        // all bmodels in a leaf share the same key
        if !(*pleaf).efrags.is_null() {
            gEngfuncs.R_StoreEfrags.expect("non-null function pointer")(&mut (*pleaf).efrags,
                                                                        tr.realframecount);
        }
        r_leafkeys[pleaf.wrapping_offset_from((*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                                                     as
                                                                                                                     libc::c_int)).leafs)
                       as libc::c_long as usize] = r_currentkey;
        r_currentkey += 1
    } else {
        // deal with model fragments in this leaf
        //	pleaf->cluster
        // node is just a decision point, so go down the apropriate sides
        // find which side of the node we are on
        plane = (*node).plane;
        match (*plane).type_0 as libc::c_int {
            0 => {
                dot =
                    (tr.modelorg[0 as libc::c_int as usize] - (*plane).dist)
                        as libc::c_double
            }
            1 => {
                dot =
                    (tr.modelorg[1 as libc::c_int as usize] - (*plane).dist)
                        as libc::c_double
            }
            2 => {
                dot =
                    (tr.modelorg[2 as libc::c_int as usize] - (*plane).dist)
                        as libc::c_double
            }
            _ => {
                dot =
                    (tr.modelorg[0 as libc::c_int as usize] *
                         (*plane).normal[0 as libc::c_int as usize] +
                         tr.modelorg[1 as libc::c_int as usize] *
                             (*plane).normal[1 as libc::c_int as usize] +
                         tr.modelorg[2 as libc::c_int as usize] *
                             (*plane).normal[2 as libc::c_int as usize] -
                         (*plane).dist) as libc::c_double
            }
        }
        if dot >= 0 as libc::c_int as libc::c_double {
            side = 0 as libc::c_int
        } else { side = 1 as libc::c_int }
        // recurse down the children, front side first
        R_RecursiveWorldNode((*node).children[side as usize], clipflags);
        // draw stuff
        c = (*node).numsurfaces as libc::c_int;
        if c != 0 {
            surf =
                (*gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                       as
                                                                                       libc::c_int)).surfaces.offset((*node).firstsurface
                                                                                                                         as
                                                                                                                         libc::c_int
                                                                                                                         as
                                                                                                                         isize);
            if dot < -0.01f32 as libc::c_double {
                loop  {
                    if (*surf).flags as libc::c_uint &
                           (1 as libc::c_uint) << 1 as libc::c_int != 0 &&
                           (*surf).visframe == tr.framecount {
                        R_RenderFace(surf, clipflags);
                    }
                    surf = surf.offset(1);
                    c -= 1;
                    if !(c != 0) { break ; }
                }
            } else if dot > 0.01f32 as libc::c_double {
                loop  {
                    if (*surf).flags as libc::c_uint &
                           (1 as libc::c_uint) << 1 as libc::c_int == 0 &&
                           (*surf).visframe == tr.framecount {
                        R_RenderFace(surf, clipflags);
                    }
                    surf = surf.offset(1);
                    c -= 1;
                    if !(c != 0) { break ; }
                }
            }
            // all surfaces on the same node share the same sequence number
            r_currentkey += 1
        }
        // recurse down the back side
        R_RecursiveWorldNode((*node).children[(side == 0) as libc::c_int as
                                                  usize], clipflags);
    };
}
/*
================
R_RenderWorld
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_RenderWorld() {
    //if (!r_drawworld->value)
	//	return;
    if RI.drawWorld as u64 == 0 { return }
    c_drawnode = 0 as libc::c_int;
    // auto cycle the world frame for texture animation
    RI.currententity =
        gEngfuncs.GetEntityByIndex.expect("non-null function pointer")(0 as
                                                                           libc::c_int);
    //RI.currententity->frame = (int)(gpGlobals->time*2);
    tr.modelorg[0 as libc::c_int as usize] =
        RI.vieworg[0 as libc::c_int as usize];
    tr.modelorg[1 as libc::c_int as usize] =
        RI.vieworg[1 as libc::c_int as usize];
    tr.modelorg[2 as libc::c_int as usize] =
        RI.vieworg[2 as libc::c_int as usize];
    RI.currentmodel =
        gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1 as
                                                                             libc::c_int);
    r_pcurrentvertbase = (*RI.currentmodel).vertexes;
    R_RecursiveWorldNode((*RI.currentmodel).nodes, 15 as libc::c_int);
}
