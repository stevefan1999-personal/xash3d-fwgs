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
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn R_InitCaches();
    #[no_mangle]
    static mut d_pzbuffer: *mut libc::c_short;
    #[no_mangle]
    static mut r_temppool: poolhandle_t;
    #[no_mangle]
    static mut vid: viddef_t;
    #[no_mangle]
    static mut gEngfuncs: ref_api_t;
    #[no_mangle]
    static mut gpGlobals: *mut ref_globals_t;
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
pub type C2RustUnnamed = libc::c_uint;
pub const REF_GL_ATTRIBUTES_COUNT: C2RustUnnamed = 20;
pub const REF_GL_CONTEXT_NO_ERROR: C2RustUnnamed = 19;
pub const REF_GL_CONTEXT_RESET_NOTIFICATION: C2RustUnnamed = 18;
pub const REF_GL_CONTEXT_RELEASE_BEHAVIOR: C2RustUnnamed = 17;
pub const REF_GL_FRAMEBUFFER_SRGB_CAPABLE: C2RustUnnamed = 16;
pub const REF_GL_SHARE_WITH_CURRENT_CONTEXT: C2RustUnnamed = 15;
pub const REF_GL_CONTEXT_PROFILE_MASK: C2RustUnnamed = 14;
pub const REF_GL_CONTEXT_FLAGS: C2RustUnnamed = 13;
pub const REF_GL_CONTEXT_EGL: C2RustUnnamed = 12;
pub const REF_GL_CONTEXT_MINOR_VERSION: C2RustUnnamed = 11;
pub const REF_GL_CONTEXT_MAJOR_VERSION: C2RustUnnamed = 10;
pub const REF_GL_ACCELERATED_VISUAL: C2RustUnnamed = 9;
pub const REF_GL_MULTISAMPLESAMPLES: C2RustUnnamed = 8;
pub const REF_GL_MULTISAMPLEBUFFERS: C2RustUnnamed = 7;
pub const REF_GL_STENCIL_SIZE: C2RustUnnamed = 6;
pub const REF_GL_DEPTH_SIZE: C2RustUnnamed = 5;
pub const REF_GL_DOUBLEBUFFER: C2RustUnnamed = 4;
pub const REF_GL_ALPHA_SIZE: C2RustUnnamed = 3;
pub const REF_GL_BLUE_SIZE: C2RustUnnamed = 2;
pub const REF_GL_GREEN_SIZE: C2RustUnnamed = 1;
pub const REF_GL_RED_SIZE: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const REF_GL_CONTEXT_PROFILE_ES: C2RustUnnamed_0 = 4;
pub const REF_GL_CONTEXT_PROFILE_COMPATIBILITY: C2RustUnnamed_0 = 2;
pub const REF_GL_CONTEXT_PROFILE_CORE: C2RustUnnamed_0 = 1;
pub type ref_screen_rotation_e = libc::c_uint;
pub const REF_ROTATE_CCW: ref_screen_rotation_e = 3;
pub const REF_ROTATE_UD: ref_screen_rotation_e = 2;
pub const REF_ROTATE_CW: ref_screen_rotation_e = 1;
pub const REF_ROTATE_NONE: ref_screen_rotation_e = 0;
pub type ref_screen_rotation_t = ref_screen_rotation_e;
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
pub type pixel_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct viddef_t {
    pub buffer: *mut pixel_t,
    pub colormap: [pixel_t; 262144],
    pub screen: [pixel_t; 65536],
    pub screen32: [libc::c_uint; 65536],
    pub addmap: [byte; 65536],
    pub modmap: [byte; 65536],
    pub alphamap: [pixel_t; 786432],
    pub color: pixel_t,
    pub is2d: qboolean,
    pub alpha: byte,
    pub rendermode: libc::c_int,
    pub rowbytes: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swblit_s {
    pub stride: uint,
    pub bpp: uint,
    pub rmask: uint,
    pub gmask: uint,
    pub bmask: uint,
    pub pLockBuffer: Option<unsafe extern "C" fn() -> *mut libc::c_void>,
    pub pUnlockBuffer: Option<unsafe extern "C" fn() -> ()>,
    pub pCreateBuffer: Option<unsafe extern "C" fn(_: libc::c_int,
                                                   _: libc::c_int,
                                                   _: *mut uint, _: *mut uint,
                                                   _: *mut uint, _: *mut uint,
                                                   _: *mut uint) -> qboolean>,
    pub rotate: uint,
    pub gl1: qboolean,
}
pub type GLubyte = byte;
pub type GLenum = uint;
pub type GLuint = uint;
pub type GLsizei = libc::c_int;
pub type GLbitfield = uint;
pub type GLint = libc::c_int;
pub type GLboolean = byte;
pub type GLvoid = ();
pub type GLsizeiptrARB = libc::c_int;
pub type GLfloat = libc::c_float;
pub type GLclampf = libc::c_float;
pub type GL_DEBUG_PROC_ARB
    =
    Option<unsafe extern "C" fn(_: GLenum, _: GLenum, _: GLuint, _: GLenum,
                                _: GLsizei, _: *const GLcharARB,
                                _: *mut libc::c_void) -> ()>;
pub type GLcharARB = libc::c_char;
pub type GLdouble = libc::c_double;
static mut pglGetError: Option<unsafe extern "C" fn() -> GLenum> = None;
static mut pglGetString:
       Option<unsafe extern "C" fn(_: GLenum) -> *const GLubyte> =
    None;
static mut pglBegin: Option<unsafe extern "C" fn(_: GLenum) -> ()> = None;
static mut pglBindTexture:
       Option<unsafe extern "C" fn(_: GLenum, _: GLuint) -> ()> =
    None;
static mut pglClear: Option<unsafe extern "C" fn(_: GLbitfield) -> ()> = None;
static mut pglClearColor:
       Option<unsafe extern "C" fn(_: GLclampf, _: GLclampf, _: GLclampf,
                                   _: GLclampf) -> ()> =
    None;
static mut pglColor4f:
       Option<unsafe extern "C" fn(_: GLfloat, _: GLfloat, _: GLfloat,
                                   _: GLfloat) -> ()> =
    None;
static mut pglDisable: Option<unsafe extern "C" fn(_: GLenum) -> ()> = None;
static mut pglDisableClientState:
       Option<unsafe extern "C" fn(_: GLenum) -> ()> =
    None;
static mut pglDrawArrays:
       Option<unsafe extern "C" fn(_: GLenum, _: GLint, _: GLsizei) -> ()> =
    None;
static mut pglEnable: Option<unsafe extern "C" fn(_: GLenum) -> ()> = None;
static mut pglEnableClientState: Option<unsafe extern "C" fn(_: GLenum) -> ()>
       =
    None;
static mut pglEnd: Option<unsafe extern "C" fn() -> ()> = None;
static mut pglGenTextures:
       Option<unsafe extern "C" fn(_: GLsizei, _: *mut GLuint) -> ()> =
    None;
static mut pglLoadIdentity: Option<unsafe extern "C" fn() -> ()> = None;
static mut pglMatrixMode: Option<unsafe extern "C" fn(_: GLenum) -> ()> =
    None;
static mut pglOrtho:
       Option<unsafe extern "C" fn(_: GLdouble, _: GLdouble, _: GLdouble,
                                   _: GLdouble, _: GLdouble, _: GLdouble)
                  -> ()> =
    None;
static mut pglTexCoord2f:
       Option<unsafe extern "C" fn(_: GLfloat, _: GLfloat) -> ()> =
    None;
static mut pglTexImage2D:
       Option<unsafe extern "C" fn(_: GLenum, _: GLint, _: GLint, _: GLsizei,
                                   _: GLsizei, _: GLint, _: GLenum, _: GLenum,
                                   _: *const libc::c_void) -> ()> =
    None;
static mut pglTexParameteri:
       Option<unsafe extern "C" fn(_: GLenum, _: GLenum, _: GLint) -> ()> =
    None;
static mut pglVertex2f:
       Option<unsafe extern "C" fn(_: GLfloat, _: GLfloat) -> ()> =
    None;
static mut pglViewport:
       Option<unsafe extern "C" fn(_: GLint, _: GLint, _: GLsizei, _: GLsizei)
                  -> ()> =
    None;
static mut pglDrawElements:
       Option<unsafe extern "C" fn(_: GLenum, _: GLsizei, _: GLenum,
                                   _: *const libc::c_void) -> ()> =
    None;
static mut pglVertexPointer:
       Option<unsafe extern "C" fn(_: GLint, _: GLenum, _: GLsizei,
                                   _: *const libc::c_void) -> ()> =
    None;
static mut pglTexCoordPointer:
       Option<unsafe extern "C" fn(_: GLint, _: GLenum, _: GLsizei,
                                   _: *const libc::c_void) -> ()> =
    None;
static mut pglDebugMessageControlARB:
       Option<unsafe extern "C" fn(_: GLenum, _: GLenum, _: GLenum,
                                   _: GLsizei, _: *const GLuint, _: GLboolean)
                  -> ()> =
    None;
static mut pglDebugMessageCallbackARB:
       Option<unsafe extern "C" fn(_: GL_DEBUG_PROC_ARB, _: *mut libc::c_void)
                  -> ()> =
    None;
static mut pglBindFramebuffer:
       Option<unsafe extern "C" fn(_: GLenum, _: GLuint) -> ()> =
    None;
static mut pglGenFramebuffers:
       Option<unsafe extern "C" fn(_: GLsizei, _: *mut GLuint) -> ()> =
    None;
static mut pglFramebufferTexture2D:
       Option<unsafe extern "C" fn(_: GLenum, _: GLenum, _: GLenum, _: GLuint,
                                   _: GLint) -> ()> =
    None;
static mut pglBlitFramebuffer:
       Option<unsafe extern "C" fn(_: GLint, _: GLint, _: GLint, _: GLint,
                                   _: GLint, _: GLint, _: GLint, _: GLint,
                                   _: GLbitfield, _: GLenum) -> ()> =
    None;
#[no_mangle]
pub static mut swblit: swblit_s =
    swblit_s{stride: 0,
             bpp: 0,
             rmask: 0,
             gmask: 0,
             bmask: 0,
             pLockBuffer: None,
             pUnlockBuffer: None,
             pCreateBuffer: None,
             rotate: 0,
             gl1: false_0,};
#[no_mangle]
pub unsafe extern "C" fn R_SetDisplayTransform(mut rotate:
                                                   ref_screen_rotation_t,
                                               mut offset_x: libc::c_int,
                                               mut offset_y: libc::c_int,
                                               mut scale_x: libc::c_float,
                                               mut scale_y: libc::c_float)
 -> qboolean {
    let mut ret: qboolean = true_0;
    if rotate as libc::c_uint > 1 as libc::c_int as libc::c_uint {
        gEngfuncs.Con_Printf.expect("non-null function pointer")(b"only 0-1 rotation supported\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char);
        ret = false_0
    } else { swblit.rotate = rotate as uint }
    if offset_x != 0 || offset_y != 0 {
        // it is possible implement for offset > 0
        gEngfuncs.Con_Printf.expect("non-null function pointer")(b"offset transform not supported\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char);
        ret = false_0
    }
    if scale_x != 1.0f32 || scale_y != 1.0f32 {
        // maybe implement 2x2?
        gEngfuncs.Con_Printf.expect("non-null function pointer")(b"scale transform not supported\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char);
        ret = false_0
    }
    return ret;
}
static mut glbuf: *mut libc::c_ushort =
    0 as *const libc::c_ushort as *mut libc::c_ushort;
#[no_mangle]
pub unsafe extern "C" fn GL_SetupAttributes(mut safegl: libc::c_int) {
    // untill we have any blitter in ref api, setup GL
    gEngfuncs.GL_SetAttribute.expect("non-null function pointer")(REF_GL_CONTEXT_PROFILE_MASK
                                                                      as
                                                                      libc::c_int,
                                                                  REF_GL_CONTEXT_PROFILE_ES
                                                                      as
                                                                      libc::c_int);
    gEngfuncs.GL_SetAttribute.expect("non-null function pointer")(REF_GL_CONTEXT_EGL
                                                                      as
                                                                      libc::c_int,
                                                                  1 as
                                                                      libc::c_int);
    //	safegl=1;
    if safegl != 0 {
        gEngfuncs.GL_SetAttribute.expect("non-null function pointer")(REF_GL_CONTEXT_MAJOR_VERSION
                                                                          as
                                                                          libc::c_int,
                                                                      1 as
                                                                          libc::c_int);
        gEngfuncs.GL_SetAttribute.expect("non-null function pointer")(REF_GL_CONTEXT_MINOR_VERSION
                                                                          as
                                                                          libc::c_int,
                                                                      1 as
                                                                          libc::c_int);
        swblit.gl1 = true_0
    } else {
        gEngfuncs.GL_SetAttribute.expect("non-null function pointer")(REF_GL_CONTEXT_MAJOR_VERSION
                                                                          as
                                                                          libc::c_int,
                                                                      3 as
                                                                          libc::c_int);
        gEngfuncs.GL_SetAttribute.expect("non-null function pointer")(REF_GL_CONTEXT_MINOR_VERSION
                                                                          as
                                                                          libc::c_int,
                                                                      0 as
                                                                          libc::c_int);
    }
    gEngfuncs.GL_SetAttribute.expect("non-null function pointer")(REF_GL_DOUBLEBUFFER
                                                                      as
                                                                      libc::c_int,
                                                                  1 as
                                                                      libc::c_int);
    gEngfuncs.GL_SetAttribute.expect("non-null function pointer")(REF_GL_RED_SIZE
                                                                      as
                                                                      libc::c_int,
                                                                  5 as
                                                                      libc::c_int);
    gEngfuncs.GL_SetAttribute.expect("non-null function pointer")(REF_GL_GREEN_SIZE
                                                                      as
                                                                      libc::c_int,
                                                                  6 as
                                                                      libc::c_int);
    gEngfuncs.GL_SetAttribute.expect("non-null function pointer")(REF_GL_BLUE_SIZE
                                                                      as
                                                                      libc::c_int,
                                                                  5 as
                                                                      libc::c_int);
}
#[no_mangle]
pub static mut pglOrthof:
           Option<unsafe extern "C" fn(_: GLfloat, _: GLfloat, _: GLfloat,
                                       _: GLfloat, _: GLfloat, _: GLfloat)
                      -> ()> =
    None;
#[no_mangle]
pub static mut pglBindBuffer:
           Option<unsafe extern "C" fn(_: GLenum, _: GLuint) -> ()> =
    None;
#[no_mangle]
pub static mut pglBufferData:
           Option<unsafe extern "C" fn(_: GLenum, _: GLsizeiptrARB,
                                       _: *const libc::c_void, _: GLenum)
                      -> ()> =
    None;
#[no_mangle]
pub static mut pglGenBuffers:
           Option<unsafe extern "C" fn(_: GLsizei, _: *mut GLuint) -> ()> =
    None;
#[no_mangle]
pub static mut pglDeleteBuffers:
           Option<unsafe extern "C" fn(_: GLsizei, _: *const GLuint) -> ()> =
    None;
#[no_mangle]
pub static mut pglMapBufferOES:
           Option<unsafe extern "C" fn(_: GLenum, _: GLenum)
                      -> *mut libc::c_void> =
    None;
#[no_mangle]
pub static mut pglUnmapBufferOES:
           Option<unsafe extern "C" fn(_: GLenum) -> GLboolean> =
    None;
#[no_mangle]
pub unsafe extern "C" fn GL_InitExtensions() {
    pglBegin =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_: GLenum)
                                           ->
                                               ()>>(gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")(b"glBegin\x00"
                                                                                                                        as
                                                                                                                        *const u8
                                                                                                                        as
                                                                                                                        *const libc::c_char));
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"glBegin : %p\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             pglBegin);
    pglEnd =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn()
                                           ->
                                               ()>>(gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")(b"glEnd\x00"
                                                                                                                        as
                                                                                                                        *const u8
                                                                                                                        as
                                                                                                                        *const libc::c_char));
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"glEnd : %p\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             pglEnd);
    pglTexCoord2f =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_: GLfloat,
                                                            _: GLfloat)
                                           ->
                                               ()>>(gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")(b"glTexCoord2f\x00"
                                                                                                                        as
                                                                                                                        *const u8
                                                                                                                        as
                                                                                                                        *const libc::c_char));
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"glTexCoord2f : %p\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             pglTexCoord2f);
    pglVertex2f =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_: GLfloat,
                                                            _: GLfloat)
                                           ->
                                               ()>>(gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")(b"glVertex2f\x00"
                                                                                                                        as
                                                                                                                        *const u8
                                                                                                                        as
                                                                                                                        *const libc::c_char));
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"glVertex2f : %p\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             pglVertex2f);
    pglEnable =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_: GLenum)
                                           ->
                                               ()>>(gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")(b"glEnable\x00"
                                                                                                                        as
                                                                                                                        *const u8
                                                                                                                        as
                                                                                                                        *const libc::c_char));
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"glEnable : %p\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             pglEnable);
    pglDisable =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_: GLenum)
                                           ->
                                               ()>>(gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")(b"glDisable\x00"
                                                                                                                        as
                                                                                                                        *const u8
                                                                                                                        as
                                                                                                                        *const libc::c_char));
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"glDisable : %p\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             pglDisable);
    pglTexImage2D =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_: GLenum,
                                                            _: GLint,
                                                            _: GLint,
                                                            _: GLsizei,
                                                            _: GLsizei,
                                                            _: GLint,
                                                            _: GLenum,
                                                            _: GLenum,
                                                            _:
                                                                *const libc::c_void)
                                           ->
                                               ()>>(gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")(b"glTexImage2D\x00"
                                                                                                                        as
                                                                                                                        *const u8
                                                                                                                        as
                                                                                                                        *const libc::c_char));
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"glTexImage2D : %p\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             pglTexImage2D);
    pglOrtho =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_: GLdouble,
                                                            _: GLdouble,
                                                            _: GLdouble,
                                                            _: GLdouble,
                                                            _: GLdouble,
                                                            _: GLdouble)
                                           ->
                                               ()>>(gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")(b"glOrtho\x00"
                                                                                                                        as
                                                                                                                        *const u8
                                                                                                                        as
                                                                                                                        *const libc::c_char));
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"glOrtho : %p\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             pglOrtho);
    pglOrthof =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_: GLfloat,
                                                            _: GLfloat,
                                                            _: GLfloat,
                                                            _: GLfloat,
                                                            _: GLfloat,
                                                            _: GLfloat)
                                           ->
                                               ()>>(gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")(b"glOrthof\x00"
                                                                                                                        as
                                                                                                                        *const u8
                                                                                                                        as
                                                                                                                        *const libc::c_char));
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"glOrthof : %p\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             pglOrthof);
    pglMatrixMode =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_: GLenum)
                                           ->
                                               ()>>(gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")(b"glMatrixMode\x00"
                                                                                                                        as
                                                                                                                        *const u8
                                                                                                                        as
                                                                                                                        *const libc::c_char));
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"glMatrixMode : %p\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             pglMatrixMode);
    pglLoadIdentity =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn()
                                           ->
                                               ()>>(gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")(b"glLoadIdentity\x00"
                                                                                                                        as
                                                                                                                        *const u8
                                                                                                                        as
                                                                                                                        *const libc::c_char));
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"glLoadIdentity : %p\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             pglLoadIdentity);
    pglViewport =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_: GLint,
                                                            _: GLint,
                                                            _: GLsizei,
                                                            _: GLsizei)
                                           ->
                                               ()>>(gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")(b"glViewport\x00"
                                                                                                                        as
                                                                                                                        *const u8
                                                                                                                        as
                                                                                                                        *const libc::c_char));
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"glViewport : %p\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             pglViewport);
    pglBindTexture =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_: GLenum,
                                                            _: GLuint)
                                           ->
                                               ()>>(gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")(b"glBindTexture\x00"
                                                                                                                        as
                                                                                                                        *const u8
                                                                                                                        as
                                                                                                                        *const libc::c_char));
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"glBindTexture : %p\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             pglBindTexture);
    pglDebugMessageCallbackARB =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_:
                                                                GL_DEBUG_PROC_ARB,
                                                            _:
                                                                *mut libc::c_void)
                                           ->
                                               ()>>(gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")(b"glDebugMessageCallbackARB\x00"
                                                                                                                        as
                                                                                                                        *const u8
                                                                                                                        as
                                                                                                                        *const libc::c_char));
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"glDebugMessageCallbackARB : %p\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             pglDebugMessageCallbackARB);
    pglDebugMessageControlARB =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_: GLenum,
                                                            _: GLenum,
                                                            _: GLenum,
                                                            _: GLsizei,
                                                            _: *const GLuint,
                                                            _: GLboolean)
                                           ->
                                               ()>>(gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")(b"glDebugMessageControlARB\x00"
                                                                                                                        as
                                                                                                                        *const u8
                                                                                                                        as
                                                                                                                        *const libc::c_char));
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"glDebugMessageControlARB : %p\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             pglDebugMessageControlARB);
    pglGetError =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn()
                                           ->
                                               GLenum>>(gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")(b"glGetError\x00"
                                                                                                                            as
                                                                                                                            *const u8
                                                                                                                            as
                                                                                                                            *const libc::c_char));
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"glGetError : %p\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             pglGetError);
    pglGenTextures =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_: GLsizei,
                                                            _: *mut GLuint)
                                           ->
                                               ()>>(gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")(b"glGenTextures\x00"
                                                                                                                        as
                                                                                                                        *const u8
                                                                                                                        as
                                                                                                                        *const libc::c_char));
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"glGenTextures : %p\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             pglGenTextures);
    pglTexParameteri =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_: GLenum,
                                                            _: GLenum,
                                                            _: GLint)
                                           ->
                                               ()>>(gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")(b"glTexParameteri\x00"
                                                                                                                        as
                                                                                                                        *const u8
                                                                                                                        as
                                                                                                                        *const libc::c_char));
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"glTexParameteri : %p\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             pglTexParameteri);
    pglEnableClientState =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_: GLenum)
                                           ->
                                               ()>>(gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")(b"glEnableClientState\x00"
                                                                                                                        as
                                                                                                                        *const u8
                                                                                                                        as
                                                                                                                        *const libc::c_char));
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"glEnableClientState : %p\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             pglEnableClientState);
    pglDisableClientState =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_: GLenum)
                                           ->
                                               ()>>(gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")(b"glDisableClientState\x00"
                                                                                                                        as
                                                                                                                        *const u8
                                                                                                                        as
                                                                                                                        *const libc::c_char));
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"glDisableClientState : %p\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             pglDisableClientState);
    pglVertexPointer =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_: GLint,
                                                            _: GLenum,
                                                            _: GLsizei,
                                                            _:
                                                                *const libc::c_void)
                                           ->
                                               ()>>(gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")(b"glVertexPointer\x00"
                                                                                                                        as
                                                                                                                        *const u8
                                                                                                                        as
                                                                                                                        *const libc::c_char));
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"glVertexPointer : %p\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             pglVertexPointer);
    pglTexCoordPointer =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_: GLint,
                                                            _: GLenum,
                                                            _: GLsizei,
                                                            _:
                                                                *const libc::c_void)
                                           ->
                                               ()>>(gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")(b"glTexCoordPointer\x00"
                                                                                                                        as
                                                                                                                        *const u8
                                                                                                                        as
                                                                                                                        *const libc::c_char));
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"glTexCoordPointer : %p\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             pglTexCoordPointer);
    pglDrawElements =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_: GLenum,
                                                            _: GLsizei,
                                                            _: GLenum,
                                                            _:
                                                                *const libc::c_void)
                                           ->
                                               ()>>(gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")(b"glDrawElements\x00"
                                                                                                                        as
                                                                                                                        *const u8
                                                                                                                        as
                                                                                                                        *const libc::c_char));
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"glDrawElements : %p\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             pglDrawElements);
    pglClear =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_: GLbitfield)
                                           ->
                                               ()>>(gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")(b"glClear\x00"
                                                                                                                        as
                                                                                                                        *const u8
                                                                                                                        as
                                                                                                                        *const libc::c_char));
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"glClear : %p\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             pglClear);
    pglClearColor =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_: GLclampf,
                                                            _: GLclampf,
                                                            _: GLclampf,
                                                            _: GLclampf)
                                           ->
                                               ()>>(gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")(b"glClearColor\x00"
                                                                                                                        as
                                                                                                                        *const u8
                                                                                                                        as
                                                                                                                        *const libc::c_char));
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"glClearColor : %p\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             pglClearColor);
    pglGetString =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_: GLenum)
                                           ->
                                               *const GLubyte>>(gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")(b"glGetString\x00"
                                                                                                                                    as
                                                                                                                                    *const u8
                                                                                                                                    as
                                                                                                                                    *const libc::c_char));
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"glGetString : %p\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             pglGetString);
    pglColor4f =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_: GLfloat,
                                                            _: GLfloat,
                                                            _: GLfloat,
                                                            _: GLfloat)
                                           ->
                                               ()>>(gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")(b"glColor4f\x00"
                                                                                                                        as
                                                                                                                        *const u8
                                                                                                                        as
                                                                                                                        *const libc::c_char));
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"glColor4f : %p\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             pglColor4f);
    pglDrawArrays =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_: GLenum,
                                                            _: GLint,
                                                            _: GLsizei)
                                           ->
                                               ()>>(gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")(b"glDrawArrays\x00"
                                                                                                                        as
                                                                                                                        *const u8
                                                                                                                        as
                                                                                                                        *const libc::c_char));
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"glDrawArrays : %p\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             pglDrawArrays);
    pglBindBuffer =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_: GLenum,
                                                            _: GLuint)
                                           ->
                                               ()>>(gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")(b"glBindBuffer\x00"
                                                                                                                        as
                                                                                                                        *const u8
                                                                                                                        as
                                                                                                                        *const libc::c_char));
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"glBindBuffer : %p\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             pglBindBuffer);
    pglBufferData =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_: GLenum,
                                                            _: GLsizeiptrARB,
                                                            _:
                                                                *const libc::c_void,
                                                            _: GLenum)
                                           ->
                                               ()>>(gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")(b"glBufferData\x00"
                                                                                                                        as
                                                                                                                        *const u8
                                                                                                                        as
                                                                                                                        *const libc::c_char));
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"glBufferData : %p\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             pglBufferData);
    pglGenBuffers =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_: GLsizei,
                                                            _: *mut GLuint)
                                           ->
                                               ()>>(gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")(b"glGenBuffers\x00"
                                                                                                                        as
                                                                                                                        *const u8
                                                                                                                        as
                                                                                                                        *const libc::c_char));
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"glGenBuffers : %p\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             pglGenBuffers);
    pglDeleteBuffers =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_: GLsizei,
                                                            _: *const GLuint)
                                           ->
                                               ()>>(gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")(b"glDeleteBuffers\x00"
                                                                                                                        as
                                                                                                                        *const u8
                                                                                                                        as
                                                                                                                        *const libc::c_char));
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"glDeleteBuffers : %p\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             pglDeleteBuffers);
    pglMapBufferOES =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_: GLenum,
                                                            _: GLenum)
                                           ->
                                               *mut libc::c_void>>(gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")(b"glMapBufferOES\x00"
                                                                                                                                       as
                                                                                                                                       *const u8
                                                                                                                                       as
                                                                                                                                       *const libc::c_char));
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"glMapBufferOES : %p\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             pglMapBufferOES);
    if pglMapBufferOES.is_none() {
        pglMapBufferOES =
            ::std::mem::transmute::<*mut libc::c_void,
                                    Option<unsafe extern "C" fn(_: GLenum,
                                                                _: GLenum)
                                               ->
                                                   *mut libc::c_void>>(gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")(b"glMapBuffer\x00"
                                                                                                                                           as
                                                                                                                                           *const u8
                                                                                                                                           as
                                                                                                                                           *const libc::c_char))
    }
    pglUnmapBufferOES =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_: GLenum)
                                           ->
                                               GLboolean>>(gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")(b"glUnmapBufferOES\x00"
                                                                                                                               as
                                                                                                                               *const u8
                                                                                                                               as
                                                                                                                               *const libc::c_char));
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"glUnmapBufferOES : %p\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             pglUnmapBufferOES);
    if pglUnmapBufferOES.is_none() {
        pglUnmapBufferOES =
            ::std::mem::transmute::<*mut libc::c_void,
                                    Option<unsafe extern "C" fn(_: GLenum)
                                               ->
                                                   GLboolean>>(gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")(b"glUnmapBuffer\x00"
                                                                                                                                   as
                                                                                                                                   *const u8
                                                                                                                                   as
                                                                                                                                   *const libc::c_char))
    }
    pglGenFramebuffers =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_: GLsizei,
                                                            _: *mut GLuint)
                                           ->
                                               ()>>(gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")(b"glGenFramebuffers\x00"
                                                                                                                        as
                                                                                                                        *const u8
                                                                                                                        as
                                                                                                                        *const libc::c_char));
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"glGenFramebuffers : %p\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             pglGenFramebuffers);
    pglBindFramebuffer =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_: GLenum,
                                                            _: GLuint)
                                           ->
                                               ()>>(gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")(b"glBindFramebuffer\x00"
                                                                                                                        as
                                                                                                                        *const u8
                                                                                                                        as
                                                                                                                        *const libc::c_char));
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"glBindFramebuffer : %p\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             pglBindFramebuffer);
    pglFramebufferTexture2D =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_: GLenum,
                                                            _: GLenum,
                                                            _: GLenum,
                                                            _: GLuint,
                                                            _: GLint)
                                           ->
                                               ()>>(gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")(b"glFramebufferTexture2D\x00"
                                                                                                                        as
                                                                                                                        *const u8
                                                                                                                        as
                                                                                                                        *const libc::c_char));
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"glFramebufferTexture2D : %p\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             pglFramebufferTexture2D);
    pglBlitFramebuffer =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_: GLint,
                                                            _: GLint,
                                                            _: GLint,
                                                            _: GLint,
                                                            _: GLint,
                                                            _: GLint,
                                                            _: GLint,
                                                            _: GLint,
                                                            _: GLbitfield,
                                                            _: GLenum)
                                           ->
                                               ()>>(gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")(b"glBlitFramebuffer\x00"
                                                                                                                        as
                                                                                                                        *const u8
                                                                                                                        as
                                                                                                                        *const libc::c_char));
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"glBlitFramebuffer : %p\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             pglBlitFramebuffer);
    pglGenTextures =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_: GLsizei,
                                                            _: *mut GLuint)
                                           ->
                                               ()>>(gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")(b"glGenTextures\x00"
                                                                                                                        as
                                                                                                                        *const u8
                                                                                                                        as
                                                                                                                        *const libc::c_char));
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"glGenTextures : %p\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             pglGenTextures);
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"version:%s\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             pglGetString.expect("non-null function pointer")(0x1f02
                                                                                                                  as
                                                                                                                  libc::c_int
                                                                                                                  as
                                                                                                                  GLenum));
}
#[no_mangle]
pub unsafe extern "C" fn GL_ClearExtensions() { }
unsafe extern "C" fn R_Lock_GL1() -> *mut libc::c_void {
    return glbuf as *mut libc::c_void;
}
unsafe extern "C" fn R_Unlock_GLES1() {
    pglTexImage2D.expect("non-null function pointer")(0xde1 as libc::c_int as
                                                          GLenum,
                                                      0 as libc::c_int,
                                                      0x1907 as libc::c_int,
                                                      vid.width, vid.height,
                                                      0 as libc::c_int,
                                                      0x1907 as libc::c_int as
                                                          GLenum,
                                                      0x8363 as libc::c_int as
                                                          GLenum,
                                                      glbuf as
                                                          *const libc::c_void);
    pglDrawArrays.expect("non-null function pointer")(0x6 as libc::c_int as
                                                          GLenum,
                                                      0 as libc::c_int,
                                                      4 as libc::c_int);
    gEngfuncs.GL_SwapBuffers.expect("non-null function pointer")();
}
unsafe extern "C" fn R_CreateBuffer_GLES1(mut width: libc::c_int,
                                          mut height: libc::c_int,
                                          mut stride: *mut uint,
                                          mut bpp: *mut uint,
                                          mut r: *mut uint, mut g: *mut uint,
                                          mut b: *mut uint) -> qboolean {
    let mut data: [libc::c_float; 8] =
        [0 as libc::c_int as libc::c_float, 0 as libc::c_int as libc::c_float,
         1 as libc::c_int as libc::c_float, 0 as libc::c_int as libc::c_float,
         1 as libc::c_int as libc::c_float, 1 as libc::c_int as libc::c_float,
         0 as libc::c_int as libc::c_float,
         1 as libc::c_int as libc::c_float];
    let mut vbo: libc::c_int = 0;
    pglViewport.expect("non-null function pointer")(0 as libc::c_int,
                                                    0 as libc::c_int, width,
                                                    height);
    pglMatrixMode.expect("non-null function pointer")(0x1701 as libc::c_int as
                                                          GLenum);
    pglLoadIdentity.expect("non-null function pointer")();
    // project 0..1 to screen size
    pglOrthof.expect("non-null function pointer")(0 as libc::c_int as GLfloat,
                                                  1 as libc::c_int as GLfloat,
                                                  1 as libc::c_int as GLfloat,
                                                  0 as libc::c_int as GLfloat,
                                                  -(99999 as libc::c_int) as
                                                      GLfloat,
                                                  99999 as libc::c_int as
                                                      GLfloat);
    pglMatrixMode.expect("non-null function pointer")(0x1700 as libc::c_int as
                                                          GLenum);
    pglLoadIdentity.expect("non-null function pointer")();
    pglEnable.expect("non-null function pointer")(0xde1 as libc::c_int as
                                                      GLenum);
    pglTexParameteri.expect("non-null function pointer")(0xde1 as libc::c_int
                                                             as GLenum,
                                                         0x2800 as libc::c_int
                                                             as GLenum,
                                                         0x2600 as
                                                             libc::c_int);
    pglTexParameteri.expect("non-null function pointer")(0xde1 as libc::c_int
                                                             as GLenum,
                                                         0x2801 as libc::c_int
                                                             as GLenum,
                                                         0x2600 as
                                                             libc::c_int);
    //if( vbo )
	//	pglDeleteBuffers( 1,&vbo );
    pglGenBuffers.expect("non-null function pointer")(1 as libc::c_int,
                                                      &mut vbo as
                                                          *mut libc::c_int as
                                                          *mut GLuint);
    pglBindBuffer.expect("non-null function pointer")(0x8892 as libc::c_int as
                                                          GLenum,
                                                      vbo as GLuint);
    pglBufferData.expect("non-null function pointer")(0x8892 as libc::c_int as
                                                          GLenum,
                                                      ::std::mem::size_of::<[libc::c_float; 8]>()
                                                          as libc::c_ulong as
                                                          GLsizeiptrARB,
                                                      data.as_mut_ptr() as
                                                          *const libc::c_void,
                                                      0x88e4 as libc::c_int as
                                                          GLenum);
    pglEnableClientState.expect("non-null function pointer")(0x8074 as
                                                                 libc::c_int
                                                                 as GLenum);
    pglEnableClientState.expect("non-null function pointer")(0x8078 as
                                                                 libc::c_int
                                                                 as GLenum);
    pglVertexPointer.expect("non-null function pointer")(2 as libc::c_int,
                                                         0x1406 as libc::c_int
                                                             as GLenum,
                                                         8 as libc::c_int,
                                                         0 as
                                                             *const libc::c_void);
    pglTexCoordPointer.expect("non-null function pointer")(2 as libc::c_int,
                                                           0x1406 as
                                                               libc::c_int as
                                                               GLenum,
                                                           8 as libc::c_int,
                                                           0 as
                                                               *const libc::c_void);
    pglBindBuffer.expect("non-null function pointer")(0x8892 as libc::c_int as
                                                          GLenum,
                                                      0 as libc::c_int as
                                                          GLuint);
    pglColor4f.expect("non-null function pointer")(1 as libc::c_int as
                                                       GLfloat,
                                                   1 as libc::c_int as
                                                       GLfloat,
                                                   1 as libc::c_int as
                                                       GLfloat,
                                                   1 as libc::c_int as
                                                       GLfloat);
    if !glbuf.is_null() {
        gEngfuncs._Mem_Free.expect("non-null function pointer")(glbuf as
                                                                    *mut libc::c_void,
                                                                b"../ref_soft/r_glblit.c\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char,
                                                                296 as
                                                                    libc::c_int);
    }
    glbuf =
        gEngfuncs._Mem_Alloc.expect("non-null function pointer")(r_temppool,
                                                                 (width *
                                                                      height *
                                                                      2 as
                                                                          libc::c_int)
                                                                     as
                                                                     size_t,
                                                                 false_0,
                                                                 b"../ref_soft/r_glblit.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 298 as
                                                                     libc::c_int)
            as *mut libc::c_ushort;
    *stride = width as uint;
    *bpp = 2 as libc::c_int as uint;
    *r =
        ((1 as libc::c_uint) <<
             5 as libc::c_int).wrapping_sub(1 as libc::c_int as libc::c_uint)
            << 6 as libc::c_int + 5 as libc::c_int;
    *g =
        ((1 as libc::c_uint) <<
             6 as libc::c_int).wrapping_sub(1 as libc::c_int as libc::c_uint)
            << 5 as libc::c_int;
    *b =
        ((1 as libc::c_uint) <<
             5 as libc::c_int).wrapping_sub(1 as libc::c_int as libc::c_uint);
    return true_0;
}
unsafe extern "C" fn R_Lock_GLES3() -> *mut libc::c_void {
    let mut buf: *mut libc::c_void = 0 as *mut libc::c_void;
    if vid.width == 0 || vid.height == 0 { return 0 as *mut libc::c_void }
    if !glbuf.is_null() { return glbuf as *mut libc::c_void }
    pglBufferData.expect("non-null function pointer")(0x88ec as libc::c_int as
                                                          GLenum,
                                                      vid.width * vid.height *
                                                          2 as libc::c_int,
                                                      0 as
                                                          *const libc::c_void,
                                                      0x88e0 as libc::c_int as
                                                          GLenum);
    if pglMapBufferOES.is_some() {
        buf =
            pglMapBufferOES.expect("non-null function pointer")(0x88ec as
                                                                    libc::c_int
                                                                    as GLenum,
                                                                0x88b9 as
                                                                    libc::c_int
                                                                    as GLenum)
    }
    if buf.is_null() {
        if pglUnmapBufferOES.is_some() {
            pglUnmapBufferOES.expect("non-null function pointer")(0x88ec as
                                                                      libc::c_int
                                                                      as
                                                                      GLenum);
        }
        pglBindBuffer.expect("non-null function pointer")(0x88ec as
                                                              libc::c_int as
                                                              GLenum,
                                                          0 as libc::c_int as
                                                              GLuint);
        glbuf =
            gEngfuncs._Mem_Alloc.expect("non-null function pointer")(r_temppool,
                                                                     (vid.width
                                                                          *
                                                                          vid.height
                                                                          *
                                                                          2 as
                                                                              libc::c_int)
                                                                         as
                                                                         size_t,
                                                                     false_0,
                                                                     b"../ref_soft/r_glblit.c\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     327 as
                                                                         libc::c_int)
                as *mut libc::c_ushort;
        pglTexImage2D.expect("non-null function pointer")(0xde1 as libc::c_int
                                                              as GLenum,
                                                          0 as libc::c_int,
                                                          0x1907 as
                                                              libc::c_int,
                                                          vid.width,
                                                          vid.height,
                                                          0 as libc::c_int,
                                                          0x1907 as
                                                              libc::c_int as
                                                              GLenum,
                                                          0x8363 as
                                                              libc::c_int as
                                                              GLenum,
                                                          glbuf as
                                                              *const libc::c_void);
        return glbuf as *mut libc::c_void
    } else { return buf };
}
unsafe extern "C" fn R_Unlock_GLES3() {
    gEngfuncs.GL_SwapBuffers.expect("non-null function pointer")();
    if !glbuf.is_null() {
        pglTexImage2D.expect("non-null function pointer")(0xde1 as libc::c_int
                                                              as GLenum,
                                                          0 as libc::c_int,
                                                          0x1907 as
                                                              libc::c_int,
                                                          vid.width,
                                                          vid.height,
                                                          0 as libc::c_int,
                                                          0x1907 as
                                                              libc::c_int as
                                                              GLenum,
                                                          0x8363 as
                                                              libc::c_int as
                                                              GLenum,
                                                          glbuf as
                                                              *const libc::c_void);
    } else {
        if pglUnmapBufferOES.is_some() {
            pglUnmapBufferOES.expect("non-null function pointer")(0x88ec as
                                                                      libc::c_int
                                                                      as
                                                                      GLenum);
        }
        pglTexImage2D.expect("non-null function pointer")(0xde1 as libc::c_int
                                                              as GLenum,
                                                          0 as libc::c_int,
                                                          0x1907 as
                                                              libc::c_int,
                                                          vid.width,
                                                          vid.height,
                                                          0 as libc::c_int,
                                                          0x1907 as
                                                              libc::c_int as
                                                              GLenum,
                                                          0x8363 as
                                                              libc::c_int as
                                                              GLenum,
                                                          0 as
                                                              *const libc::c_void);
    }
    //pglDrawArrays( GL_TRIANGLE_FAN, 0,4 );
    pglBlitFramebuffer.expect("non-null function pointer")(0 as libc::c_int,
                                                           vid.height,
                                                           vid.width,
                                                           0 as libc::c_int,
                                                           0 as libc::c_int,
                                                           0 as libc::c_int,
                                                           vid.width,
                                                           vid.height,
                                                           0x4000 as
                                                               libc::c_int as
                                                               GLbitfield,
                                                           0x2600 as
                                                               libc::c_int as
                                                               GLenum);
}
unsafe extern "C" fn R_CreateBuffer_GLES3(mut width: libc::c_int,
                                          mut height: libc::c_int,
                                          mut stride: *mut uint,
                                          mut bpp: *mut uint,
                                          mut r: *mut uint, mut g: *mut uint,
                                          mut b: *mut uint) -> qboolean {
    let mut data: [libc::c_float; 8] =
        [0 as libc::c_int as libc::c_float, 0 as libc::c_int as libc::c_float,
         1 as libc::c_int as libc::c_float, 0 as libc::c_int as libc::c_float,
         1 as libc::c_int as libc::c_float, 1 as libc::c_int as libc::c_float,
         0 as libc::c_int as libc::c_float,
         1 as libc::c_int as libc::c_float];
    let mut vbo: libc::c_int = 0;
    let mut pbo: libc::c_int = 0;
    let mut fbo: libc::c_int = 0;
    let mut to: libc::c_int = 0;
    // shitty fbo does not work without texture objects :(
    pglGenTextures.expect("non-null function pointer")(1 as libc::c_int,
                                                       &mut to as
                                                           *mut libc::c_int as
                                                           *mut GLuint);
    pglBindTexture.expect("non-null function pointer")(0xde1 as libc::c_int as
                                                           GLenum,
                                                       to as GLuint);
    pglViewport.expect("non-null function pointer")(0 as libc::c_int,
                                                    0 as libc::c_int, width,
                                                    height);
    /*
	pglMatrixMode( GL_PROJECTION );
	pglLoadIdentity();
	// project 0..1 to screen size
	pglOrtho( 0, 1, 1, 0, -99999, 99999 );
	pglMatrixMode( GL_MODELVIEW );
	pglLoadIdentity();

	pglEnable( GL_TEXTURE_2D );
	pglTexParameteri( GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST );
	pglTexParameteri( GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST );

	if( vbo )
		pglDeleteBuffers( 1,&vbo );

	if( pbo )
		pglDeleteBuffers( 1,&pbo );
	*/
    //pglGenBuffers( 1,&vbo );
    pglGenBuffers.expect("non-null function pointer")(1 as libc::c_int,
                                                      &mut pbo as
                                                          *mut libc::c_int as
                                                          *mut GLuint);
    //pglBindBuffer( GL_ARRAY_BUFFER_ARB, vbo );
	//pglBufferData( GL_ARRAY_BUFFER_ARB, sizeof(data), data, GL_STATIC_DRAW_ARB );
    //pglEnableClientState( GL_VERTEX_ARRAY );
	//pglEnableClientState( GL_TEXTURE_COORD_ARRAY );
    //pglVertexPointer( 2, GL_FLOAT, 8, 0 );
	//pglTexCoordPointer( 2, GL_FLOAT, 8, 0 );
	//pglBindBuffer( GL_ARRAY_BUFFER_ARB, 0 );
    pglBindBuffer.expect("non-null function pointer")(0x88ec as libc::c_int as
                                                          GLenum,
                                                      pbo as GLuint);
    pglBufferData.expect("non-null function pointer")(0x88ec as libc::c_int as
                                                          GLenum,
                                                      width * height *
                                                          2 as libc::c_int,
                                                      0 as
                                                          *const libc::c_void,
                                                      0x88e0 as libc::c_int as
                                                          GLenum);
    pglTexImage2D.expect("non-null function pointer")(0xde1 as libc::c_int as
                                                          GLenum,
                                                      0 as libc::c_int,
                                                      0x1907 as libc::c_int,
                                                      width, height,
                                                      0 as libc::c_int,
                                                      0x1907 as libc::c_int as
                                                          GLenum,
                                                      0x8363 as libc::c_int as
                                                          GLenum,
                                                      0 as
                                                          *const libc::c_void);
    pglGenFramebuffers.expect("non-null function pointer")(1 as libc::c_int,
                                                           &mut fbo as
                                                               *mut libc::c_int
                                                               as
                                                               *mut GLuint);
    pglBindFramebuffer.expect("non-null function pointer")(0x8ca8 as
                                                               libc::c_int as
                                                               GLenum,
                                                           fbo as GLuint);
    pglFramebufferTexture2D.expect("non-null function pointer")(0x8ca8 as
                                                                    libc::c_int
                                                                    as GLenum,
                                                                0x8ce0 as
                                                                    libc::c_int
                                                                    as GLenum,
                                                                0xde1 as
                                                                    libc::c_int
                                                                    as GLenum,
                                                                to as GLuint,
                                                                0 as
                                                                    libc::c_int);
    pglBindFramebuffer.expect("non-null function pointer")(0x8ca9 as
                                                               libc::c_int as
                                                               GLenum,
                                                           0 as libc::c_int as
                                                               GLuint);
    //pglColor4f( 1, 1, 1, 1 );
    *stride = width as uint;
    *bpp = 2 as libc::c_int as uint;
    *r =
        ((1 as libc::c_uint) <<
             5 as libc::c_int).wrapping_sub(1 as libc::c_int as libc::c_uint)
            << 6 as libc::c_int + 5 as libc::c_int;
    *g =
        ((1 as libc::c_uint) <<
             6 as libc::c_int).wrapping_sub(1 as libc::c_int as libc::c_uint)
            << 5 as libc::c_int;
    *b =
        ((1 as libc::c_uint) <<
             5 as libc::c_int).wrapping_sub(1 as libc::c_int as libc::c_uint);
    return true_0;
}
unsafe extern "C" fn FIRST_BIT(mut mask: uint) -> libc::c_int {
    let mut i: uint = 0;
    i = 0 as libc::c_int as uint;
    while (1 as libc::c_uint) << i & mask == 0 { i = i.wrapping_add(1) }
    return i as libc::c_int;
}
unsafe extern "C" fn COUNT_BITS(mut mask: uint) -> libc::c_int {
    let mut i: uint = 0;
    i = 0 as libc::c_int as uint;
    while mask != 0 {
        i =
            (i as
                 libc::c_uint).wrapping_add(mask &
                                                1 as libc::c_int as
                                                    libc::c_uint) as uint as
                uint;
        mask = mask >> 1 as libc::c_int
    }
    return i as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn R_BuildScreenMap() {
    let mut i: libc::c_int = 0;
    let mut rshift: uint = FIRST_BIT(swblit.rmask) as uint;
    let mut gshift: uint = FIRST_BIT(swblit.gmask) as uint;
    let mut bshift: uint = FIRST_BIT(swblit.bmask) as uint;
    let mut rbits: uint = COUNT_BITS(swblit.rmask) as uint;
    let mut gbits: uint = COUNT_BITS(swblit.gmask) as uint;
    let mut bbits: uint = COUNT_BITS(swblit.bmask) as uint;
    let mut rmult: uint = (1 as libc::c_uint) << rbits;
    let mut gmult: uint = (1 as libc::c_uint) << gbits;
    let mut bmult: uint = (1 as libc::c_uint) << bbits;
    let mut rdiv: uint =
        ((1 as libc::c_uint) <<
             5 as libc::c_int).wrapping_sub(1 as libc::c_int as libc::c_uint);
    let mut gdiv: uint =
        ((1 as libc::c_uint) <<
             6 as libc::c_int).wrapping_sub(1 as libc::c_int as libc::c_uint);
    let mut bdiv: uint =
        ((1 as libc::c_uint) <<
             5 as libc::c_int).wrapping_sub(1 as libc::c_int as libc::c_uint);
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"Blit table: %d %d %d %d %d %d\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             rmult, gmult,
                                                             bmult, rdiv,
                                                             gdiv, bdiv);
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        let mut r: libc::c_uint = 0;
        let mut g: libc::c_uint = 0;
        let mut b: libc::c_uint = 0;
        let mut major: libc::c_uint = 0;
        let mut j: libc::c_uint = 0;
        // 332 to 565
        r =
            ((i >> 8 as libc::c_int - 3 as libc::c_int) << 2 as libc::c_int)
                as libc::c_uint &
                ((1 as libc::c_uint) <<
                     5 as
                         libc::c_int).wrapping_sub(1 as libc::c_int as
                                                       libc::c_uint);
        g =
            ((i >> 8 as libc::c_int - 3 as libc::c_int - 3 as libc::c_int) <<
                 3 as libc::c_int) as libc::c_uint &
                ((1 as libc::c_uint) <<
                     6 as
                         libc::c_int).wrapping_sub(1 as libc::c_int as
                                                       libc::c_uint);
        b =
            ((i >>
                  8 as libc::c_int - 3 as libc::c_int - 3 as libc::c_int -
                      2 as libc::c_int) << 3 as libc::c_int) as libc::c_uint &
                ((1 as libc::c_uint) <<
                     5 as
                         libc::c_int).wrapping_sub(1 as libc::c_int as
                                                       libc::c_uint);
        //major = r << (6 + 5) | (g << 5) | b;
        major =
            r.wrapping_mul(rmult).wrapping_div(rdiv) << rshift |
                g.wrapping_mul(gmult).wrapping_div(gdiv) << gshift |
                b.wrapping_mul(bmult).wrapping_div(bdiv) << bshift;
        j = 0 as libc::c_int as libc::c_uint;
        while j < 256 as libc::c_int as libc::c_uint {
            let mut minor: uint = 0;
            // restore minor GBRGBRGB
            r =
                ((j &
                      ((1 as libc::c_int) << 5 as libc::c_int) as
                          libc::c_uint) >> 5 as libc::c_int) <<
                    1 as libc::c_int |
                    ((j &
                          ((1 as libc::c_int) << 2 as libc::c_int) as
                              libc::c_uint) >> 2 as libc::c_int) <<
                        0 as libc::c_int;
            g =
                ((j &
                      ((1 as libc::c_int) << 7 as libc::c_int) as
                          libc::c_uint) >> 7 as libc::c_int) <<
                    2 as libc::c_int |
                    ((j &
                          ((1 as libc::c_int) << 4 as libc::c_int) as
                              libc::c_uint) >> 4 as libc::c_int) <<
                        1 as libc::c_int |
                    ((j &
                          ((1 as libc::c_int) << 1 as libc::c_int) as
                              libc::c_uint) >> 1 as libc::c_int) <<
                        0 as libc::c_int;
            b =
                ((j &
                      ((1 as libc::c_int) << 6 as libc::c_int) as
                          libc::c_uint) >> 6 as libc::c_int) <<
                    2 as libc::c_int |
                    ((j &
                          ((1 as libc::c_int) << 3 as libc::c_int) as
                              libc::c_uint) >> 3 as libc::c_int) <<
                        1 as libc::c_int |
                    ((j &
                          ((1 as libc::c_int) << 0 as libc::c_int) as
                              libc::c_uint) >> 0 as libc::c_int) <<
                        0 as libc::c_int;
            //vid.screen[(i<<8)|j] = r << (6 + 5) | (g << 5) | b | major;
            minor =
                r.wrapping_mul(rmult).wrapping_div(rdiv) << rshift |
                    g.wrapping_mul(gmult).wrapping_div(gdiv) << gshift |
                    b.wrapping_mul(bmult).wrapping_div(bdiv) << bshift;
            if swblit.bpp == 2 as libc::c_int as libc::c_uint {
                vid.screen[((i << 8 as libc::c_int) as libc::c_uint | j) as
                               usize] = (major | minor) as pixel_t
            } else {
                vid.screen32[((i << 8 as libc::c_int) as libc::c_uint | j) as
                                 usize] = major | minor
            }
            j = j.wrapping_add(1)
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_BuildBlendMaps() {
    let mut r1: libc::c_uint = 0;
    let mut g1: libc::c_uint = 0;
    let mut b1: libc::c_uint = 0;
    let mut r2: libc::c_uint = 0;
    let mut g2: libc::c_uint = 0;
    let mut b2: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    r1 = 0 as libc::c_int as libc::c_uint;
    while r1 < (1 as libc::c_uint) << 3 as libc::c_int {
        g1 = 0 as libc::c_int as libc::c_uint;
        while g1 < (1 as libc::c_uint) << 3 as libc::c_int {
            b1 = 0 as libc::c_int as libc::c_uint;
            while b1 < (1 as libc::c_uint) << 2 as libc::c_int {
                r2 = 0 as libc::c_int as libc::c_uint;
                while r2 < (1 as libc::c_uint) << 3 as libc::c_int {
                    g2 = 0 as libc::c_int as libc::c_uint;
                    while g2 < (1 as libc::c_uint) << 3 as libc::c_int {
                        b2 = 0 as libc::c_int as libc::c_uint;
                        while b2 < (1 as libc::c_uint) << 2 as libc::c_int {
                            let mut r: libc::c_uint = 0;
                            let mut g: libc::c_uint = 0;
                            let mut b: libc::c_uint = 0;
                            let mut index1: libc::c_ushort =
                                (r1 << 2 as libc::c_int + 3 as libc::c_int |
                                     g1 << 2 as libc::c_int | b1) as
                                    libc::c_ushort;
                            let mut index2: libc::c_ushort =
                                ((r2 << 2 as libc::c_int + 3 as libc::c_int |
                                      g2 << 2 as libc::c_int | b2) <<
                                     8 as libc::c_int) as libc::c_ushort;
                            let mut a: libc::c_uint = 0;
                            r = r1.wrapping_add(r2);
                            g = g1.wrapping_add(g2);
                            b = b1.wrapping_add(b2);
                            if r >
                                   ((1 as libc::c_uint) <<
                                        3 as
                                            libc::c_int).wrapping_sub(1 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint)
                               {
                                r =
                                    ((1 as libc::c_uint) <<
                                         3 as
                                             libc::c_int).wrapping_sub(1 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_uint)
                            }
                            if g >
                                   ((1 as libc::c_uint) <<
                                        3 as
                                            libc::c_int).wrapping_sub(1 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint)
                               {
                                g =
                                    ((1 as libc::c_uint) <<
                                         3 as
                                             libc::c_int).wrapping_sub(1 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_uint)
                            }
                            if b >
                                   ((1 as libc::c_uint) <<
                                        2 as
                                            libc::c_int).wrapping_sub(1 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint)
                               {
                                b =
                                    ((1 as libc::c_uint) <<
                                         2 as
                                             libc::c_int).wrapping_sub(1 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_uint)
                            }
                            if vid.addmap[(index2 as libc::c_int |
                                               index1 as libc::c_int) as
                                              usize] != 0 {
                                gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"../ref_soft/r_glblit.c\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         526
                                                                                             as
                                                                                             libc::c_int);
                            }
                            vid.addmap[(index2 as libc::c_int |
                                            index1 as libc::c_int) as usize] =
                                (r << 2 as libc::c_int + 3 as libc::c_int |
                                     g << 2 as libc::c_int | b) as byte;
                            r =
                                r1.wrapping_mul(r2).wrapping_div(((1 as
                                                                       libc::c_uint)
                                                                      <<
                                                                      3 as
                                                                          libc::c_int).wrapping_sub(1
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_uint));
                            g =
                                g1.wrapping_mul(g2).wrapping_div(((1 as
                                                                       libc::c_uint)
                                                                      <<
                                                                      3 as
                                                                          libc::c_int).wrapping_sub(1
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_uint));
                            b =
                                b1.wrapping_mul(b2).wrapping_div(((1 as
                                                                       libc::c_uint)
                                                                      <<
                                                                      2 as
                                                                          libc::c_int).wrapping_sub(1
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_uint));
                            vid.modmap[(index2 as libc::c_int |
                                            index1 as libc::c_int) as usize] =
                                (r << 2 as libc::c_int + 3 as libc::c_int |
                                     g << 2 as libc::c_int | b) as byte;
                            b2 = b2.wrapping_add(1)
                        }
                        g2 = g2.wrapping_add(1)
                    }
                    r2 = r2.wrapping_add(1)
                }
                b1 = b1.wrapping_add(1)
            }
            g1 = g1.wrapping_add(1)
        }
        r1 = r1.wrapping_add(1)
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < 8192 as libc::c_int as libc::c_uint {
        let mut r_0: libc::c_uint = 0;
        let mut g_0: libc::c_uint = 0;
        let mut b_0: libc::c_uint = 0;
        let mut color: uint = i << 3 as libc::c_int;
        let mut m: uint = color >> 8 as libc::c_int;
        let mut j_0: uint = color & 0xff as libc::c_int as libc::c_uint;
        let mut index1_0: libc::c_ushort = i as libc::c_ushort;
        r1 =
            (m >> 8 as libc::c_int - 3 as libc::c_int) << 2 as libc::c_int &
                ((1 as libc::c_uint) <<
                     5 as
                         libc::c_int).wrapping_sub(1 as libc::c_int as
                                                       libc::c_uint);
        g1 =
            (m >> 8 as libc::c_int - 3 as libc::c_int - 3 as libc::c_int) <<
                3 as libc::c_int &
                ((1 as libc::c_uint) <<
                     6 as
                         libc::c_int).wrapping_sub(1 as libc::c_int as
                                                       libc::c_uint);
        b1 =
            (m >>
                 8 as libc::c_int - 3 as libc::c_int - 3 as libc::c_int -
                     2 as libc::c_int) << 3 as libc::c_int &
                ((1 as libc::c_uint) <<
                     5 as
                         libc::c_int).wrapping_sub(1 as libc::c_int as
                                                       libc::c_uint);
        r1 |=
            ((j_0 & ((1 as libc::c_int) << 5 as libc::c_int) as libc::c_uint)
                 >> 5 as libc::c_int) << 1 as libc::c_int |
                ((j_0 &
                      ((1 as libc::c_int) << 2 as libc::c_int) as
                          libc::c_uint) >> 2 as libc::c_int) <<
                    0 as libc::c_int;
        g1 |=
            ((j_0 & ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint)
                 >> 7 as libc::c_int) << 2 as libc::c_int |
                ((j_0 &
                      ((1 as libc::c_int) << 4 as libc::c_int) as
                          libc::c_uint) >> 4 as libc::c_int) <<
                    1 as libc::c_int |
                ((j_0 &
                      ((1 as libc::c_int) << 1 as libc::c_int) as
                          libc::c_uint) >> 1 as libc::c_int) <<
                    0 as libc::c_int;
        b1 |=
            ((j_0 & ((1 as libc::c_int) << 6 as libc::c_int) as libc::c_uint)
                 >> 6 as libc::c_int) << 2 as libc::c_int |
                ((j_0 &
                      ((1 as libc::c_int) << 3 as libc::c_int) as
                          libc::c_uint) >> 3 as libc::c_int) <<
                    1 as libc::c_int |
                ((j_0 &
                      ((1 as libc::c_int) << 0 as libc::c_int) as
                          libc::c_uint) >> 0 as libc::c_int) <<
                    0 as libc::c_int;
        j_0 = 0 as libc::c_int as uint;
        while j_0 < 32 as libc::c_int as libc::c_uint {
            let mut index2_0: libc::c_uint = j_0 << 13 as libc::c_int;
            let mut major: libc::c_uint = 0;
            let mut minor: libc::c_uint = 0;
            r_0 =
                r1.wrapping_mul(j_0).wrapping_div(32 as libc::c_int as
                                                      libc::c_uint);
            g_0 =
                g1.wrapping_mul(j_0).wrapping_div(32 as libc::c_int as
                                                      libc::c_uint);
            b_0 =
                b1.wrapping_mul(j_0).wrapping_div(32 as libc::c_int as
                                                      libc::c_uint);
            major =
                (r_0 >> 2 as libc::c_int &
                     ((1 as libc::c_uint) <<
                          3 as
                              libc::c_int).wrapping_sub(1 as libc::c_int as
                                                            libc::c_uint)) <<
                    5 as libc::c_int |
                    (g_0 >> 3 as libc::c_int &
                         ((1 as libc::c_uint) <<
                              3 as
                                  libc::c_int).wrapping_sub(1 as libc::c_int
                                                                as
                                                                libc::c_uint))
                        << 2 as libc::c_int |
                    b_0 >> 3 as libc::c_int &
                        ((1 as libc::c_uint) <<
                             2 as
                                 libc::c_int).wrapping_sub(1 as libc::c_int as
                                                               libc::c_uint);
            // save minor GBRGBRGB
            minor =
                ((r_0 &
                      ((1 as libc::c_int) << 1 as libc::c_int) as
                          libc::c_uint) >> 1 as libc::c_int) <<
                    5 as libc::c_int |
                    ((r_0 &
                          ((1 as libc::c_int) << 0 as libc::c_int) as
                              libc::c_uint) >> 0 as libc::c_int) <<
                        2 as libc::c_int |
                    ((g_0 &
                          ((1 as libc::c_int) << 2 as libc::c_int) as
                              libc::c_uint) >> 2 as libc::c_int) <<
                        7 as libc::c_int |
                    ((g_0 &
                          ((1 as libc::c_int) << 1 as libc::c_int) as
                              libc::c_uint) >> 1 as libc::c_int) <<
                        4 as libc::c_int |
                    ((g_0 &
                          ((1 as libc::c_int) << 0 as libc::c_int) as
                              libc::c_uint) >> 0 as libc::c_int) <<
                        1 as libc::c_int |
                    ((b_0 &
                          ((1 as libc::c_int) << 2 as libc::c_int) as
                              libc::c_uint) >> 2 as libc::c_int) <<
                        6 as libc::c_int |
                    ((b_0 &
                          ((1 as libc::c_int) << 1 as libc::c_int) as
                              libc::c_uint) >> 1 as libc::c_int) <<
                        3 as libc::c_int |
                    ((b_0 &
                          ((1 as libc::c_int) << 0 as libc::c_int) as
                              libc::c_uint) >> 0 as libc::c_int) <<
                        0 as libc::c_int;
            vid.colormap[(index2_0 | index1_0 as libc::c_uint) as usize] =
                (major << 8 as libc::c_int |
                     minor & 0xff as libc::c_int as libc::c_uint) as pixel_t;
            j_0 = j_0.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < 1024 as libc::c_int as libc::c_uint {
        let mut r_1: libc::c_uint = 0;
        let mut g_1: libc::c_uint = 0;
        let mut b_1: libc::c_uint = 0;
        let mut color_0: uint =
            i << 6 as libc::c_int | (1 as libc::c_uint) << 5 as libc::c_int |
                (1 as libc::c_uint) << 4 as libc::c_int |
                (1 as libc::c_uint) << 3 as libc::c_int;
        let mut m_0: uint = color_0 >> 8 as libc::c_int;
        let mut j_1: uint = color_0 & 0xff as libc::c_int as libc::c_uint;
        let mut index1_1: libc::c_ushort = i as libc::c_ushort;
        r1 =
            (m_0 >> 8 as libc::c_int - 3 as libc::c_int) << 2 as libc::c_int &
                ((1 as libc::c_uint) <<
                     5 as
                         libc::c_int).wrapping_sub(1 as libc::c_int as
                                                       libc::c_uint);
        g1 =
            (m_0 >> 8 as libc::c_int - 3 as libc::c_int - 3 as libc::c_int) <<
                3 as libc::c_int &
                ((1 as libc::c_uint) <<
                     6 as
                         libc::c_int).wrapping_sub(1 as libc::c_int as
                                                       libc::c_uint);
        b1 =
            (m_0 >>
                 8 as libc::c_int - 3 as libc::c_int - 3 as libc::c_int -
                     2 as libc::c_int) << 3 as libc::c_int &
                ((1 as libc::c_uint) <<
                     5 as
                         libc::c_int).wrapping_sub(1 as libc::c_int as
                                                       libc::c_uint);
        r1 |=
            ((j_1 & ((1 as libc::c_int) << 5 as libc::c_int) as libc::c_uint)
                 >> 5 as libc::c_int) << 1 as libc::c_int |
                ((j_1 &
                      ((1 as libc::c_int) << 2 as libc::c_int) as
                          libc::c_uint) >> 2 as libc::c_int) <<
                    0 as libc::c_int;
        g1 |=
            ((j_1 & ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint)
                 >> 7 as libc::c_int) << 2 as libc::c_int |
                ((j_1 &
                      ((1 as libc::c_int) << 4 as libc::c_int) as
                          libc::c_uint) >> 4 as libc::c_int) <<
                    1 as libc::c_int |
                ((j_1 &
                      ((1 as libc::c_int) << 1 as libc::c_int) as
                          libc::c_uint) >> 1 as libc::c_int) <<
                    0 as libc::c_int;
        b1 |=
            ((j_1 & ((1 as libc::c_int) << 6 as libc::c_int) as libc::c_uint)
                 >> 6 as libc::c_int) << 2 as libc::c_int |
                ((j_1 &
                      ((1 as libc::c_int) << 3 as libc::c_int) as
                          libc::c_uint) >> 3 as libc::c_int) <<
                    1 as libc::c_int |
                ((j_1 &
                      ((1 as libc::c_int) << 0 as libc::c_int) as
                          libc::c_uint) >> 0 as libc::c_int) <<
                    0 as libc::c_int;
        r2 = 0 as libc::c_int as libc::c_uint;
        while r2 < (1 as libc::c_uint) << 3 as libc::c_int {
            g2 = 0 as libc::c_int as libc::c_uint;
            while g2 < (1 as libc::c_uint) << 3 as libc::c_int {
                b2 = 0 as libc::c_int as libc::c_uint;
                while b2 < (1 as libc::c_uint) << 2 as libc::c_int {
                    let mut index2_1: libc::c_uint =
                        (r2 << 2 as libc::c_int + 3 as libc::c_int |
                             g2 << 2 as libc::c_int | b2) <<
                            10 as libc::c_int;
                    let mut k: libc::c_uint = 0;
                    k = 0 as libc::c_int as libc::c_uint;
                    while k < 3 as libc::c_int as libc::c_uint {
                        let mut major_0: libc::c_uint = 0;
                        let mut minor_0: libc::c_uint = 0;
                        let mut a_0: libc::c_uint =
                            k.wrapping_add(2 as libc::c_int as libc::c_uint);
                        r_1 =
                            r1.wrapping_mul((7 as libc::c_int as
                                                 libc::c_uint).wrapping_sub(a_0)).wrapping_div(7
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_uint).wrapping_add((r2
                                                                                                                                   <<
                                                                                                                                   2
                                                                                                                                       as
                                                                                                                                       libc::c_int
                                                                                                                                   |
                                                                                                                                   (1
                                                                                                                                        as
                                                                                                                                        libc::c_uint)
                                                                                                                                       <<
                                                                                                                                       2
                                                                                                                                           as
                                                                                                                                           libc::c_int).wrapping_mul(a_0).wrapping_div(7
                                                                                                                                                                                           as
                                                                                                                                                                                           libc::c_int
                                                                                                                                                                                           as
                                                                                                                                                                                           libc::c_uint));
                        g_1 =
                            g1.wrapping_mul((7 as libc::c_int as
                                                 libc::c_uint).wrapping_sub(a_0)).wrapping_div(7
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_uint).wrapping_add((g2
                                                                                                                                   <<
                                                                                                                                   3
                                                                                                                                       as
                                                                                                                                       libc::c_int
                                                                                                                                   |
                                                                                                                                   ((1
                                                                                                                                         as
                                                                                                                                         libc::c_uint)
                                                                                                                                        <<
                                                                                                                                        2
                                                                                                                                            as
                                                                                                                                            libc::c_int).wrapping_sub(1
                                                                                                                                                                          as
                                                                                                                                                                          libc::c_int
                                                                                                                                                                          as
                                                                                                                                                                          libc::c_uint)).wrapping_mul(a_0).wrapping_div(7
                                                                                                                                                                                                                            as
                                                                                                                                                                                                                            libc::c_int
                                                                                                                                                                                                                            as
                                                                                                                                                                                                                            libc::c_uint));
                        b_1 =
                            b1.wrapping_mul((7 as libc::c_int as
                                                 libc::c_uint).wrapping_sub(a_0)).wrapping_div(7
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_uint).wrapping_add((b2
                                                                                                                                   <<
                                                                                                                                   3
                                                                                                                                       as
                                                                                                                                       libc::c_int
                                                                                                                                   |
                                                                                                                                   ((1
                                                                                                                                         as
                                                                                                                                         libc::c_uint)
                                                                                                                                        <<
                                                                                                                                        2
                                                                                                                                            as
                                                                                                                                            libc::c_int).wrapping_sub(1
                                                                                                                                                                          as
                                                                                                                                                                          libc::c_int
                                                                                                                                                                          as
                                                                                                                                                                          libc::c_uint)).wrapping_mul(a_0).wrapping_div(7
                                                                                                                                                                                                                            as
                                                                                                                                                                                                                            libc::c_int
                                                                                                                                                                                                                            as
                                                                                                                                                                                                                            libc::c_uint));
                        if r_1 >
                               ((1 as libc::c_uint) <<
                                    5 as
                                        libc::c_int).wrapping_sub(1 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint)
                           {
                            r_1 =
                                ((1 as libc::c_uint) <<
                                     5 as
                                         libc::c_int).wrapping_sub(1 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint)
                        }
                        if g_1 >
                               ((1 as libc::c_uint) <<
                                    6 as
                                        libc::c_int).wrapping_sub(1 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint)
                           {
                            g_1 =
                                ((1 as libc::c_uint) <<
                                     6 as
                                         libc::c_int).wrapping_sub(1 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint)
                        }
                        if b_1 >
                               ((1 as libc::c_uint) <<
                                    5 as
                                        libc::c_int).wrapping_sub(1 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint)
                           {
                            b_1 =
                                ((1 as libc::c_uint) <<
                                     5 as
                                         libc::c_int).wrapping_sub(1 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint)
                        }
                        if !(b_1 < 32 as libc::c_int as libc::c_uint) {
                            gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"../ref_soft/r_glblit.c\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     614
                                                                                         as
                                                                                         libc::c_int);
                        }
                        major_0 =
                            (r_1 >> 2 as libc::c_int &
                                 ((1 as libc::c_uint) <<
                                      3 as
                                          libc::c_int).wrapping_sub(1 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint))
                                << 5 as libc::c_int |
                                (g_1 >> 3 as libc::c_int &
                                     ((1 as libc::c_uint) <<
                                          3 as
                                              libc::c_int).wrapping_sub(1 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint))
                                    << 2 as libc::c_int |
                                b_1 >> 3 as libc::c_int &
                                    ((1 as libc::c_uint) <<
                                         2 as
                                             libc::c_int).wrapping_sub(1 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_uint);
                        // save minor GBRGBRGB
                        minor_0 =
                            ((r_1 &
                                  ((1 as libc::c_int) << 1 as libc::c_int) as
                                      libc::c_uint) >> 1 as libc::c_int) <<
                                5 as libc::c_int |
                                ((r_1 &
                                      ((1 as libc::c_int) << 0 as libc::c_int)
                                          as libc::c_uint) >>
                                     0 as libc::c_int) << 2 as libc::c_int |
                                ((g_1 &
                                      ((1 as libc::c_int) << 2 as libc::c_int)
                                          as libc::c_uint) >>
                                     2 as libc::c_int) << 7 as libc::c_int |
                                ((g_1 &
                                      ((1 as libc::c_int) << 1 as libc::c_int)
                                          as libc::c_uint) >>
                                     1 as libc::c_int) << 4 as libc::c_int |
                                ((g_1 &
                                      ((1 as libc::c_int) << 0 as libc::c_int)
                                          as libc::c_uint) >>
                                     0 as libc::c_int) << 1 as libc::c_int |
                                ((b_1 &
                                      ((1 as libc::c_int) << 2 as libc::c_int)
                                          as libc::c_uint) >>
                                     2 as libc::c_int) << 6 as libc::c_int |
                                ((b_1 &
                                      ((1 as libc::c_int) << 1 as libc::c_int)
                                          as libc::c_uint) >>
                                     1 as libc::c_int) << 3 as libc::c_int |
                                ((b_1 &
                                      ((1 as libc::c_int) << 0 as libc::c_int)
                                          as libc::c_uint) >>
                                     0 as libc::c_int) <<
                                    0 as libc::c_int; // rowpixels
                        minor_0 =
                            minor_0 & !(0x3f as libc::c_int) as libc::c_uint;
                        vid.alphamap[(k << 18 as libc::c_int | index2_1 |
                                          index1_1 as libc::c_uint) as usize]
                            =
                            (major_0 << 8 as libc::c_int |
                                 minor_0 &
                                     0xff as libc::c_int as libc::c_uint) as
                                pixel_t;
                        k = k.wrapping_add(1)
                    }
                    b2 = b2.wrapping_add(1)
                }
                g2 = g2.wrapping_add(1)
            }
            r2 = r2.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_InitBlit(mut glblit: qboolean) {
    R_BuildBlendMaps();
    if glblit as libc::c_uint != 0 && swblit.gl1 as libc::c_uint != 0 {
        swblit.pLockBuffer =
            Some(R_Lock_GL1 as unsafe extern "C" fn() -> *mut libc::c_void);
        swblit.pUnlockBuffer =
            Some(R_Unlock_GLES1 as unsafe extern "C" fn() -> ());
        swblit.pCreateBuffer =
            Some(R_CreateBuffer_GLES1 as
                     unsafe extern "C" fn(_: libc::c_int, _: libc::c_int,
                                          _: *mut uint, _: *mut uint,
                                          _: *mut uint, _: *mut uint,
                                          _: *mut uint) -> qboolean)
    } else if glblit as u64 != 0 {
        swblit.pLockBuffer =
            Some(R_Lock_GLES3 as unsafe extern "C" fn() -> *mut libc::c_void);
        swblit.pUnlockBuffer =
            Some(R_Unlock_GLES3 as unsafe extern "C" fn() -> ());
        swblit.pCreateBuffer =
            Some(R_CreateBuffer_GLES3 as
                     unsafe extern "C" fn(_: libc::c_int, _: libc::c_int,
                                          _: *mut uint, _: *mut uint,
                                          _: *mut uint, _: *mut uint,
                                          _: *mut uint) -> qboolean)
    } else {
        swblit.pLockBuffer = gEngfuncs.SW_LockBuffer;
        swblit.pUnlockBuffer = gEngfuncs.SW_UnlockBuffer;
        swblit.pCreateBuffer = gEngfuncs.SW_CreateBuffer
    }
    R_AllocScreen();
}
#[no_mangle]
pub unsafe extern "C" fn R_AllocScreen() {
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    if (*gpGlobals).width < 128 as libc::c_int {
        (*gpGlobals).width = 128 as libc::c_int
    }
    if (*gpGlobals).height < 128 as libc::c_int {
        (*gpGlobals).height = 128 as libc::c_int
    }
    R_InitCaches();
    if swblit.rotate != 0 {
        w = (*gpGlobals).height;
        h = (*gpGlobals).width
    } else { h = (*gpGlobals).height; w = (*gpGlobals).width }
    swblit.pCreateBuffer.expect("non-null function pointer")(w, h,
                                                             &mut swblit.stride,
                                                             &mut swblit.bpp,
                                                             &mut swblit.rmask,
                                                             &mut swblit.gmask,
                                                             &mut swblit.bmask);
    R_BuildScreenMap();
    vid.width = (*gpGlobals).width;
    vid.height = (*gpGlobals).height;
    vid.rowbytes = (*gpGlobals).width;
    if !d_pzbuffer.is_null() { free(d_pzbuffer as *mut libc::c_void); }
    d_pzbuffer =
        malloc((vid.width * vid.height * 2 as libc::c_int + 64 as libc::c_int)
                   as libc::c_ulong) as *mut libc::c_short;
    if !vid.buffer.is_null() { free(vid.buffer as *mut libc::c_void); }
    vid.buffer =
        malloc(((vid.width * vid.height) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<pixel_t>()
                                                    as libc::c_ulong)) as
            *mut pixel_t;
}
#[no_mangle]
pub unsafe extern "C" fn R_BlitScreen() {
    let mut u: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut buffer: *mut libc::c_void =
        swblit.pLockBuffer.expect("non-null function pointer")();
    //	gEngfuncs.Con_Printf("blit begin\n");
	//memset( vid.buffer, 10, vid.width * vid.height );
    if buffer.is_null() || (*gpGlobals).width != vid.width ||
           (*gpGlobals).height != vid.height {
        gEngfuncs.Con_Printf.expect("non-null function pointer")(b"pre allocscrn\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char);
        R_AllocScreen();
        gEngfuncs.Con_Printf.expect("non-null function pointer")(b"post allocscrn\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char);
        return
    }
    //return;
	//byte *buf = vid.buffer;
    //#pragma omp parallel for schedule(static)
	//gEngfuncs.Con_Printf("swblit %d %d", swblit.bpp, vid.height );
    if swblit.rotate != 0 {
        if swblit.bpp == 2 as libc::c_int as libc::c_uint {
            let mut pbuf: *mut libc::c_ushort = buffer as *mut libc::c_ushort;
            v = 0 as libc::c_int;
            while v < vid.height {
                let mut start: uint = (vid.rowbytes * v) as uint;
                let mut d: uint =
                    swblit.stride.wrapping_sub(v as
                                                   libc::c_uint).wrapping_sub(1
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_uint);
                u = 0 as libc::c_int;
                while u < vid.width {
                    let mut s: libc::c_uint =
                        vid.screen[*vid.buffer.offset(start.wrapping_add(u as
                                                                             libc::c_uint)
                                                          as isize) as usize]
                            as libc::c_uint;
                    *pbuf.offset(d as isize) = s as libc::c_ushort;
                    d =
                        (d as libc::c_uint).wrapping_add(swblit.stride) as
                            uint as uint;
                    u += 1
                }
                v += 1
            }
        } else if swblit.bpp == 4 as libc::c_int as libc::c_uint {
            let mut pbuf_0: *mut libc::c_uint = buffer as *mut libc::c_uint;
            v = 0 as libc::c_int;
            while v < vid.height {
                let mut start_0: uint = (vid.rowbytes * v) as uint;
                let mut d_0: uint =
                    swblit.stride.wrapping_sub(v as
                                                   libc::c_uint).wrapping_sub(1
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_uint);
                u = 0 as libc::c_int;
                while u < vid.width {
                    let mut s_0: libc::c_uint =
                        vid.screen32[*vid.buffer.offset(start_0.wrapping_add(u
                                                                                 as
                                                                                 libc::c_uint)
                                                            as isize) as
                                         usize];
                    *pbuf_0.offset(d_0 as isize) = s_0;
                    d_0 =
                        (d_0 as libc::c_uint).wrapping_add(swblit.stride) as
                            uint as uint;
                    u += 1
                }
                v += 1
            }
        } else if swblit.bpp == 3 as libc::c_int as libc::c_uint {
            let mut pbuf_1: *mut byte = buffer as *mut byte;
            v = 0 as libc::c_int;
            while v < vid.height {
                let mut start_1: uint = (vid.rowbytes * v) as uint;
                let mut d_1: uint =
                    swblit.stride.wrapping_sub(v as
                                                   libc::c_uint).wrapping_sub(1
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_uint);
                u = 0 as libc::c_int;
                while u < vid.width {
                    let mut s_1: libc::c_uint =
                        vid.screen32[*vid.buffer.offset(start_1.wrapping_add(u
                                                                                 as
                                                                                 libc::c_uint)
                                                            as isize) as
                                         usize];
                    *pbuf_1.offset(d_1.wrapping_mul(3 as libc::c_int as
                                                        libc::c_uint) as
                                       isize) = s_1 as byte;
                    s_1 = s_1 >> 8 as libc::c_int;
                    *pbuf_1.offset(d_1.wrapping_mul(3 as libc::c_int as
                                                        libc::c_uint).wrapping_add(1
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       libc::c_uint)
                                       as isize) = s_1 as byte;
                    s_1 = s_1 >> 8 as libc::c_int;
                    *pbuf_1.offset(d_1.wrapping_mul(3 as libc::c_int as
                                                        libc::c_uint).wrapping_add(2
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       libc::c_uint)
                                       as isize) = s_1 as byte;
                    d_1 =
                        (d_1 as libc::c_uint).wrapping_add(swblit.stride) as
                            uint as uint;
                    u += 1
                }
                v += 1
            }
        }
    } else if swblit.bpp == 2 as libc::c_int as libc::c_uint {
        let mut pbuf_2: *mut libc::c_ushort = buffer as *mut libc::c_ushort;
        v = 0 as libc::c_int;
        while v < vid.height {
            let mut start_2: uint = (vid.rowbytes * v) as uint;
            let mut dstart: uint =
                swblit.stride.wrapping_mul(v as libc::c_uint);
            u = 0 as libc::c_int;
            while u < vid.width {
                let mut s_2: libc::c_uint =
                    vid.screen[*vid.buffer.offset(start_2.wrapping_add(u as
                                                                           libc::c_uint)
                                                      as isize) as usize] as
                        libc::c_uint;
                *pbuf_2.offset(dstart.wrapping_add(u as libc::c_uint) as
                                   isize) = s_2 as libc::c_ushort;
                u += 1
            }
            v += 1
        }
    } else if swblit.bpp == 4 as libc::c_int as libc::c_uint {
        let mut pbuf_3: *mut libc::c_uint = buffer as *mut libc::c_uint;
        v = 0 as libc::c_int;
        while v < vid.height {
            let mut start_3: uint = (vid.rowbytes * v) as uint;
            let mut dstart_0: uint =
                swblit.stride.wrapping_mul(v as libc::c_uint);
            u = 0 as libc::c_int;
            while u < vid.width {
                let mut s_3: libc::c_uint =
                    vid.screen32[*vid.buffer.offset(start_3.wrapping_add(u as
                                                                             libc::c_uint)
                                                        as isize) as usize];
                *pbuf_3.offset(dstart_0.wrapping_add(u as libc::c_uint) as
                                   isize) = s_3;
                u += 1
            }
            v += 1
        }
    } else if swblit.bpp == 3 as libc::c_int as libc::c_uint {
        let mut pbuf_4: *mut byte = buffer as *mut byte;
        v = 0 as libc::c_int;
        while v < vid.height {
            let mut start_4: uint = (vid.rowbytes * v) as uint;
            let mut dstart_1: uint =
                swblit.stride.wrapping_mul(v as libc::c_uint);
            u = 0 as libc::c_int;
            while u < vid.width {
                let mut s_4: libc::c_uint =
                    vid.screen32[*vid.buffer.offset(start_4.wrapping_add(u as
                                                                             libc::c_uint)
                                                        as isize) as usize];
                *pbuf_4.offset(dstart_1.wrapping_add(u as
                                                         libc::c_uint).wrapping_mul(3
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_uint)
                                   as isize) = s_4 as byte;
                s_4 = s_4 >> 8 as libc::c_int;
                *pbuf_4.offset(dstart_1.wrapping_add(u as
                                                         libc::c_uint).wrapping_mul(3
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_uint).wrapping_add(1
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       libc::c_uint)
                                   as isize) = s_4 as byte;
                s_4 = s_4 >> 8 as libc::c_int;
                *pbuf_4.offset(dstart_1.wrapping_add(u as
                                                         libc::c_uint).wrapping_mul(3
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_uint).wrapping_add(2
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       libc::c_uint)
                                   as isize) = s_4 as byte;
                u += 1
            }
            v += 1
        }
    }
    swblit.pUnlockBuffer.expect("non-null function pointer")();
    //	gEngfuncs.Con_Printf("blit end\n");
}
