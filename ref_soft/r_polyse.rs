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
    fn ceilf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn floorf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    static mut vid: viddef_t;
    #[no_mangle]
    static mut tr: gl_globals_t;
    #[no_mangle]
    static mut gEngfuncs: ref_api_t;
    #[no_mangle]
    static mut r_affinetridesc: affinetridesc_t;
    #[no_mangle]
    static mut d_viewbuffer: *mut pixel_t;
    #[no_mangle]
    static mut d_pzbuffer: *mut libc::c_short;
    #[no_mangle]
    static mut d_zwidth: libc::c_uint;
    #[no_mangle]
    static mut r_screenwidth: libc::c_int;
    #[no_mangle]
    static mut r_aliasblendcolor: libc::c_int;
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
pub type fixed8_t = libc::c_int;
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
pub struct finalvert_s {
    pub u: libc::c_int,
    pub v: libc::c_int,
    pub s: libc::c_int,
    pub t: libc::c_int,
    pub l: libc::c_int,
    pub zi: libc::c_int,
    pub flags: libc::c_int,
    pub xyz: [libc::c_float; 3],
}
pub type finalvert_t = finalvert_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dtriangle_t {
    pub index_xyz: [libc::c_short; 3],
    pub index_st: [libc::c_short; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct affinetridesc_t {
    pub pskin: *mut libc::c_void,
    pub pskindesc: libc::c_int,
    pub skinwidth: libc::c_int,
    pub skinheight: libc::c_int,
    pub ptriangles: *mut dtriangle_t,
    pub pfinalverts: *mut finalvert_t,
    pub numtriangles: libc::c_int,
    pub drawtype: libc::c_int,
    pub seamfixupX16: libc::c_int,
    pub do_vis_thresh: qboolean,
    pub vis_thresh: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aliastriangleparms_t {
    pub a: *mut finalvert_t,
    pub b: *mut finalvert_t,
    pub c: *mut finalvert_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spanpackage_t {
    pub pdest: *mut libc::c_void,
    pub pz: *mut libc::c_short,
    pub count: libc::c_int,
    pub ptex: *mut pixel_t,
    pub sfrac: libc::c_int,
    pub tfrac: libc::c_int,
    pub light: libc::c_int,
    pub zi: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct adivtab_t {
    pub quotient: libc::c_int,
    pub remainder: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct edgetable {
    pub isflattop: libc::c_int,
    pub numleftedges: libc::c_int,
    pub pleftedgevert0: *mut libc::c_int,
    pub pleftedgevert1: *mut libc::c_int,
    pub pleftedgevert2: *mut libc::c_int,
    pub numrightedges: libc::c_int,
    pub prightedgevert0: *mut libc::c_int,
    pub prightedgevert1: *mut libc::c_int,
    pub prightedgevert2: *mut libc::c_int,
}
#[inline(always)]
unsafe extern "C" fn __tg_ceil(mut __x: libc::c_float) -> libc::c_float {
    return ceilf(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_floor(mut __x: libc::c_float) -> libc::c_float {
    return floorf(__x);
}
#[no_mangle]
pub static mut aliastriangleparms: aliastriangleparms_t =
    aliastriangleparms_t{a: 0 as *const finalvert_t as *mut finalvert_t,
                         b: 0 as *const finalvert_t as *mut finalvert_t,
                         c: 0 as *const finalvert_t as *mut finalvert_t,};
#[no_mangle]
pub static mut r_p0: [libc::c_int; 6] = [0; 6];
#[no_mangle]
pub static mut r_p1: [libc::c_int; 6] = [0; 6];
#[no_mangle]
pub static mut r_p2: [libc::c_int; 6] = [0; 6];
#[no_mangle]
pub static mut d_xdenom: libc::c_int = 0;
#[no_mangle]
pub static mut pedgetable: *mut edgetable =
    0 as *const edgetable as *mut edgetable;
#[no_mangle]
pub static mut edgetables: [edgetable; 12] =
    unsafe {
        [{
             let mut init =
                 edgetable{isflattop: 0 as libc::c_int,
                           numleftedges: 1 as libc::c_int,
                           pleftedgevert0: r_p0.as_ptr() as *mut _,
                           pleftedgevert1: r_p2.as_ptr() as *mut _,
                           pleftedgevert2:
                               0 as *const libc::c_int as *mut libc::c_int,
                           numrightedges: 2 as libc::c_int,
                           prightedgevert0: r_p0.as_ptr() as *mut _,
                           prightedgevert1: r_p1.as_ptr() as *mut _,
                           prightedgevert2: r_p2.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 edgetable{isflattop: 0 as libc::c_int,
                           numleftedges: 2 as libc::c_int,
                           pleftedgevert0: r_p1.as_ptr() as *mut _,
                           pleftedgevert1: r_p0.as_ptr() as *mut _,
                           pleftedgevert2: r_p2.as_ptr() as *mut _,
                           numrightedges: 1 as libc::c_int,
                           prightedgevert0: r_p1.as_ptr() as *mut _,
                           prightedgevert1: r_p2.as_ptr() as *mut _,
                           prightedgevert2:
                               0 as *const libc::c_int as *mut libc::c_int,};
             init
         },
         {
             let mut init =
                 edgetable{isflattop: 1 as libc::c_int,
                           numleftedges: 1 as libc::c_int,
                           pleftedgevert0: r_p0.as_ptr() as *mut _,
                           pleftedgevert1: r_p2.as_ptr() as *mut _,
                           pleftedgevert2:
                               0 as *const libc::c_int as *mut libc::c_int,
                           numrightedges: 1 as libc::c_int,
                           prightedgevert0: r_p1.as_ptr() as *mut _,
                           prightedgevert1: r_p2.as_ptr() as *mut _,
                           prightedgevert2:
                               0 as *const libc::c_int as *mut libc::c_int,};
             init
         },
         {
             let mut init =
                 edgetable{isflattop: 0 as libc::c_int,
                           numleftedges: 1 as libc::c_int,
                           pleftedgevert0: r_p1.as_ptr() as *mut _,
                           pleftedgevert1: r_p0.as_ptr() as *mut _,
                           pleftedgevert2:
                               0 as *const libc::c_int as *mut libc::c_int,
                           numrightedges: 2 as libc::c_int,
                           prightedgevert0: r_p1.as_ptr() as *mut _,
                           prightedgevert1: r_p2.as_ptr() as *mut _,
                           prightedgevert2: r_p0.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 edgetable{isflattop: 0 as libc::c_int,
                           numleftedges: 2 as libc::c_int,
                           pleftedgevert0: r_p0.as_ptr() as *mut _,
                           pleftedgevert1: r_p2.as_ptr() as *mut _,
                           pleftedgevert2: r_p1.as_ptr() as *mut _,
                           numrightedges: 1 as libc::c_int,
                           prightedgevert0: r_p0.as_ptr() as *mut _,
                           prightedgevert1: r_p1.as_ptr() as *mut _,
                           prightedgevert2:
                               0 as *const libc::c_int as *mut libc::c_int,};
             init
         },
         {
             let mut init =
                 edgetable{isflattop: 0 as libc::c_int,
                           numleftedges: 1 as libc::c_int,
                           pleftedgevert0: r_p2.as_ptr() as *mut _,
                           pleftedgevert1: r_p1.as_ptr() as *mut _,
                           pleftedgevert2:
                               0 as *const libc::c_int as *mut libc::c_int,
                           numrightedges: 1 as libc::c_int,
                           prightedgevert0: r_p2.as_ptr() as *mut _,
                           prightedgevert1: r_p0.as_ptr() as *mut _,
                           prightedgevert2:
                               0 as *const libc::c_int as *mut libc::c_int,};
             init
         },
         {
             let mut init =
                 edgetable{isflattop: 0 as libc::c_int,
                           numleftedges: 1 as libc::c_int,
                           pleftedgevert0: r_p2.as_ptr() as *mut _,
                           pleftedgevert1: r_p1.as_ptr() as *mut _,
                           pleftedgevert2:
                               0 as *const libc::c_int as *mut libc::c_int,
                           numrightedges: 2 as libc::c_int,
                           prightedgevert0: r_p2.as_ptr() as *mut _,
                           prightedgevert1: r_p0.as_ptr() as *mut _,
                           prightedgevert2: r_p1.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 edgetable{isflattop: 0 as libc::c_int,
                           numleftedges: 2 as libc::c_int,
                           pleftedgevert0: r_p2.as_ptr() as *mut _,
                           pleftedgevert1: r_p1.as_ptr() as *mut _,
                           pleftedgevert2: r_p0.as_ptr() as *mut _,
                           numrightedges: 1 as libc::c_int,
                           prightedgevert0: r_p2.as_ptr() as *mut _,
                           prightedgevert1: r_p0.as_ptr() as *mut _,
                           prightedgevert2:
                               0 as *const libc::c_int as *mut libc::c_int,};
             init
         },
         {
             let mut init =
                 edgetable{isflattop: 0 as libc::c_int,
                           numleftedges: 1 as libc::c_int,
                           pleftedgevert0: r_p1.as_ptr() as *mut _,
                           pleftedgevert1: r_p0.as_ptr() as *mut _,
                           pleftedgevert2:
                               0 as *const libc::c_int as *mut libc::c_int,
                           numrightedges: 1 as libc::c_int,
                           prightedgevert0: r_p1.as_ptr() as *mut _,
                           prightedgevert1: r_p2.as_ptr() as *mut _,
                           prightedgevert2:
                               0 as *const libc::c_int as *mut libc::c_int,};
             init
         },
         {
             let mut init =
                 edgetable{isflattop: 1 as libc::c_int,
                           numleftedges: 1 as libc::c_int,
                           pleftedgevert0: r_p2.as_ptr() as *mut _,
                           pleftedgevert1: r_p1.as_ptr() as *mut _,
                           pleftedgevert2:
                               0 as *const libc::c_int as *mut libc::c_int,
                           numrightedges: 1 as libc::c_int,
                           prightedgevert0: r_p0.as_ptr() as *mut _,
                           prightedgevert1: r_p1.as_ptr() as *mut _,
                           prightedgevert2:
                               0 as *const libc::c_int as *mut libc::c_int,};
             init
         },
         {
             let mut init =
                 edgetable{isflattop: 1 as libc::c_int,
                           numleftedges: 1 as libc::c_int,
                           pleftedgevert0: r_p1.as_ptr() as *mut _,
                           pleftedgevert1: r_p0.as_ptr() as *mut _,
                           pleftedgevert2:
                               0 as *const libc::c_int as *mut libc::c_int,
                           numrightedges: 1 as libc::c_int,
                           prightedgevert0: r_p2.as_ptr() as *mut _,
                           prightedgevert1: r_p0.as_ptr() as *mut _,
                           prightedgevert2:
                               0 as *const libc::c_int as *mut libc::c_int,};
             init
         },
         {
             let mut init =
                 edgetable{isflattop: 0 as libc::c_int,
                           numleftedges: 1 as libc::c_int,
                           pleftedgevert0: r_p0.as_ptr() as *mut _,
                           pleftedgevert1: r_p2.as_ptr() as *mut _,
                           pleftedgevert2:
                               0 as *const libc::c_int as *mut libc::c_int,
                           numrightedges: 1 as libc::c_int,
                           prightedgevert0: r_p0.as_ptr() as *mut _,
                           prightedgevert1: r_p1.as_ptr() as *mut _,
                           prightedgevert2:
                               0 as *const libc::c_int as *mut libc::c_int,};
             init
         }]
    };
// FIXME: some of these can become statics
#[no_mangle]
pub static mut a_sstepxfrac: libc::c_int = 0;
#[no_mangle]
pub static mut a_tstepxfrac: libc::c_int = 0;
#[no_mangle]
pub static mut r_lstepx: libc::c_int = 0;
#[no_mangle]
pub static mut a_ststepxwhole: libc::c_int = 0;
#[no_mangle]
pub static mut r_sstepx: libc::c_int = 0;
#[no_mangle]
pub static mut r_tstepx: libc::c_int = 0;
#[no_mangle]
pub static mut r_lstepy: libc::c_int = 0;
#[no_mangle]
pub static mut r_sstepy: libc::c_int = 0;
#[no_mangle]
pub static mut r_tstepy: libc::c_int = 0;
#[no_mangle]
pub static mut r_zistepx: libc::c_int = 0;
#[no_mangle]
pub static mut r_zistepy: libc::c_int = 0;
#[no_mangle]
pub static mut d_aspancount: libc::c_int = 0;
#[no_mangle]
pub static mut d_countextrastep: libc::c_int = 0;
#[no_mangle]
pub static mut a_spans: *mut spanpackage_t =
    0 as *const spanpackage_t as *mut spanpackage_t;
#[no_mangle]
pub static mut d_pedgespanpackage: *mut spanpackage_t =
    0 as *const spanpackage_t as *mut spanpackage_t;
static mut ystart: libc::c_int = 0;
#[no_mangle]
pub static mut d_pdest: *mut pixel_t = 0 as *const pixel_t as *mut pixel_t;
#[no_mangle]
pub static mut d_ptex: *mut pixel_t = 0 as *const pixel_t as *mut pixel_t;
#[no_mangle]
pub static mut d_pz: *mut libc::c_short =
    0 as *const libc::c_short as *mut libc::c_short;
#[no_mangle]
pub static mut d_sfrac: libc::c_int = 0;
#[no_mangle]
pub static mut d_tfrac: libc::c_int = 0;
#[no_mangle]
pub static mut d_light: libc::c_int = 0;
#[no_mangle]
pub static mut d_zi: libc::c_int = 0;
#[no_mangle]
pub static mut d_ptexextrastep: libc::c_int = 0;
#[no_mangle]
pub static mut d_sfracextrastep: libc::c_int = 0;
#[no_mangle]
pub static mut d_tfracextrastep: libc::c_int = 0;
#[no_mangle]
pub static mut d_lightextrastep: libc::c_int = 0;
#[no_mangle]
pub static mut d_pdestextrastep: libc::c_int = 0;
#[no_mangle]
pub static mut d_lightbasestep: libc::c_int = 0;
#[no_mangle]
pub static mut d_pdestbasestep: libc::c_int = 0;
#[no_mangle]
pub static mut d_ptexbasestep: libc::c_int = 0;
#[no_mangle]
pub static mut d_sfracbasestep: libc::c_int = 0;
#[no_mangle]
pub static mut d_tfracbasestep: libc::c_int = 0;
#[no_mangle]
pub static mut d_ziextrastep: libc::c_int = 0;
#[no_mangle]
pub static mut d_zibasestep: libc::c_int = 0;
#[no_mangle]
pub static mut d_pzextrastep: libc::c_int = 0;
#[no_mangle]
pub static mut d_pzbasestep: libc::c_int = 0;
static mut ubasestep: libc::c_int = 0;
static mut errorterm: libc::c_int = 0;
static mut erroradjustup: libc::c_int = 0;
static mut erroradjustdown: libc::c_int = 0;
static mut adivtab: [adivtab_t; 1024] =
    [{
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(4 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(5 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(6 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(7 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 3 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 3 as libc::c_int,
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 5 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 7 as libc::c_int,
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 15 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(15 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(8 as libc::c_int),
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(5 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(4 as libc::c_int),
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(3 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(3 as libc::c_int),
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(3 as libc::c_int),
                       remainder: 6 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 5 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 7 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 9 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 11 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 13 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(14 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(4 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(5 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(6 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: -(4 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 3 as libc::c_int,
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 4 as libc::c_int,
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 7 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 14 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(14 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(7 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(5 as libc::c_int),
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(4 as libc::c_int),
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(3 as libc::c_int),
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(3 as libc::c_int),
                       remainder: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 6 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 8 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 10 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 12 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(13 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(13 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(4 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(5 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(6 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 3 as libc::c_int,
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 4 as libc::c_int,
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 6 as libc::c_int,
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 13 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(13 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(7 as libc::c_int),
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(5 as libc::c_int),
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(4 as libc::c_int),
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(3 as libc::c_int),
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(3 as libc::c_int),
                       remainder: 5 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 5 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 7 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 9 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 11 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(12 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(12 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(12 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(4 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(5 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 3 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 4 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 6 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 12 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(12 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(6 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(4 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(3 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(3 as libc::c_int),
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 6 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 8 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 10 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(11 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(11 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(11 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(11 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(4 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(5 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 3 as libc::c_int,
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 5 as libc::c_int,
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 11 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(11 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(6 as libc::c_int),
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(4 as libc::c_int),
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(3 as libc::c_int),
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(3 as libc::c_int),
                       remainder: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 5 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 7 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 9 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 5 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(10 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(10 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(10 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(10 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(10 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(4 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 3 as libc::c_int,
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 5 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 10 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(10 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(5 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(4 as libc::c_int),
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(3 as libc::c_int),
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 6 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 8 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 5 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 6 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(9 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(9 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(9 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(9 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(9 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(9 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(4 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 3 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 4 as libc::c_int,
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 9 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(9 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(5 as libc::c_int),
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(3 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(3 as libc::c_int),
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 5 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 7 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 5 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 6 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 7 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 4 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 8 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(8 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(4 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(3 as libc::c_int),
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 6 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 5 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 6 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 7 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 8 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(7 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(7 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(7 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(7 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(7 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(7 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(7 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(7 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 3 as libc::c_int,
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 7 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(7 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(4 as libc::c_int),
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(3 as libc::c_int),
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 5 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 5 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 6 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 7 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 8 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 9 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(6 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(6 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(6 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(6 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(6 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(6 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(6 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(6 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(6 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 3 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 6 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(6 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(3 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 5 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 6 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 7 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 8 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 9 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 10 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(5 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(5 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(5 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(5 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(5 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(5 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(5 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(5 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(5 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(5 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 5 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(5 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(3 as libc::c_int),
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 5 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 6 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 7 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 8 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 9 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 10 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 11 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(4 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(4 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(4 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(4 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(4 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(4 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(4 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(4 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(4 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(4 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(4 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 4 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(4 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 5 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 6 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 7 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 8 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 9 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 10 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 11 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 12 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 3 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(3 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 5 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 6 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 7 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 8 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 9 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 10 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 11 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 12 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 13 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 5 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 6 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 7 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 8 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 9 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 10 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 11 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 12 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 13 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 14 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 5 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 6 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 7 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 8 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 9 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 10 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 11 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 12 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 13 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 14 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 15 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(14 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(13 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(12 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(11 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(10 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(9 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(7 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(6 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(5 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(4 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(13 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(12 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(11 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(10 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(9 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(7 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(6 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(5 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(4 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(12 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(11 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(10 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(9 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(7 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(6 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(5 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(4 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(3 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 3 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(11 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(10 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(9 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(7 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(6 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(5 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(4 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(4 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 4 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(10 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(9 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(7 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(6 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(5 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(4 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(3 as libc::c_int),
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(5 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 5 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 5 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 5 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 5 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 5 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 5 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 5 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 5 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 5 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 5 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 5 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 5 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(9 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(7 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(6 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(5 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(4 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(4 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(3 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(6 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 6 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 3 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 6 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 6 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 6 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 6 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 6 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 6 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 6 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 6 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 6 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 6 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(7 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(6 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(5 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(4 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(5 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(3 as libc::c_int),
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(4 as libc::c_int),
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(7 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 7 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 3 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 7 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 7 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 7 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 7 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 7 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 7 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 7 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 7 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 7 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(7 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(6 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(5 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(4 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(6 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(4 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(3 as libc::c_int),
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(4 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(8 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 8 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 4 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 8 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 8 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 8 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 8 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 8 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 8 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 8 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 8 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(6 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(5 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(4 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(7 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(5 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(3 as libc::c_int),
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(3 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(5 as libc::c_int),
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(9 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 9 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 4 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 3 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 9 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 9 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 9 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 9 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 9 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 9 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 9 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(5 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(4 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(6 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(4 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(3 as libc::c_int),
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(4 as libc::c_int),
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(5 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(10 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 10 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 5 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 3 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 10 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 10 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 10 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 10 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 10 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 10 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(4 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(9 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(7 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(5 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(3 as libc::c_int),
                       remainder: -(4 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(3 as libc::c_int),
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(4 as libc::c_int),
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(6 as libc::c_int),
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(11 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 11 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 5 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 3 as libc::c_int,
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 5 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 11 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 11 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 11 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 11 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 11 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(10 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(6 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(4 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(3 as libc::c_int),
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(3 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(4 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(6 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(12 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 12 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 6 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 4 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 3 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 5 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 12 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 12 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 12 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 12 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(11 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(9 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(7 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(5 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(3 as libc::c_int),
                       remainder: -(5 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(3 as libc::c_int),
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(4 as libc::c_int),
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(5 as libc::c_int),
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(7 as libc::c_int),
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(13 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 13 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 6 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 4 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 3 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 6 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 5 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 13 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 13 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 13 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(12 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(10 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(6 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(4 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(3 as libc::c_int),
                       remainder: -(4 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(3 as libc::c_int),
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(4 as libc::c_int),
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(5 as libc::c_int),
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(7 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(14 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 14 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 7 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 4 as libc::c_int,
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 3 as libc::c_int,
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 6 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 5 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 14 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 14 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(1 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(13 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(11 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(9 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(7 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(5 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(3 as libc::c_int),
                       remainder: -(6 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(3 as libc::c_int),
                       remainder: -(3 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(3 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(4 as libc::c_int),
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(5 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(8 as libc::c_int),
                       remainder: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(15 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 15 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 7 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 5 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 3 as libc::c_int,
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 3 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 7 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 6 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 5 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 15 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(14 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(12 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(10 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(6 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(4 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(2 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(3 as libc::c_int),
                       remainder: -(5 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(3 as libc::c_int),
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(4 as libc::c_int),
                       remainder: -(4 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(4 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(6 as libc::c_int),
                       remainder: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(8 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: -(16 as libc::c_int),
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 0 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 16 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 8 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 5 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 4 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 3 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 2 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 7 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 6 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 5 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             adivtab_t{quotient: 1 as libc::c_int,
                       remainder: 0 as libc::c_int,};
         init
     }];
#[no_mangle]
pub static mut skintable: [*mut byte; 480] =
    [0 as *const byte as *mut byte; 480];
#[no_mangle]
pub static mut skinwidth: libc::c_int = 0;
#[no_mangle]
pub static mut skinstart: *mut byte = 0 as *const byte as *mut byte;
#[no_mangle]
pub static mut d_pdrawspans:
           Option<unsafe extern "C" fn(_: *mut spanpackage_t) -> ()> =
    None;
#[no_mangle]
pub unsafe extern "C" fn R_PolysetStub(mut pspanpackage: *mut spanpackage_t) {
}
/*
================
R_PolysetUpdateTables
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_PolysetUpdateTables() {
    let mut i: libc::c_int = 0;
    let mut s: *mut byte = 0 as *mut byte;
    if r_affinetridesc.skinwidth != skinwidth ||
           r_affinetridesc.pskin != skinstart as *mut libc::c_void {
        skinwidth = r_affinetridesc.skinwidth;
        skinstart = r_affinetridesc.pskin as *mut byte;
        s = skinstart;
        i = 0 as libc::c_int;
        while i < 480 as libc::c_int {
            skintable[i as usize] = s;
            i += 1;
            s = s.offset(skinwidth as isize)
        }
    };
}
/*
================
R_DrawTriangle
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_DrawTriangle() {
    let mut spans: [spanpackage_t; 1201] =
        [spanpackage_t{pdest: 0 as *mut libc::c_void,
                       pz: 0 as *mut libc::c_short,
                       count: 0,
                       ptex: 0 as *mut pixel_t,
                       sfrac: 0,
                       tfrac: 0,
                       light: 0,
                       zi: 0,}; 1201];
    let mut dv1_ab: libc::c_int = 0;
    let mut dv0_ac: libc::c_int = 0;
    let mut dv0_ab: libc::c_int = 0;
    let mut dv1_ac: libc::c_int = 0;
    /*
	d_xdenom = ( aliastriangleparms.a->v[1] - aliastriangleparms.b->v[1] ) * ( aliastriangleparms.a->v[0] - aliastriangleparms.c->v[0] ) -
			   ( aliastriangleparms.a->v[0] - aliastriangleparms.b->v[0] ) * ( aliastriangleparms.a->v[1] - aliastriangleparms.c->v[1] );
	*/
    dv0_ab = (*aliastriangleparms.a).u - (*aliastriangleparms.b).u; // u
    dv1_ab = (*aliastriangleparms.a).v - (*aliastriangleparms.b).v; // v
    if dv0_ab | dv1_ab == 0 { return } // s
    dv0_ac = (*aliastriangleparms.a).u - (*aliastriangleparms.c).u; // t
    dv1_ac = (*aliastriangleparms.a).v - (*aliastriangleparms.c).v; // light
    if dv0_ac | dv1_ac == 0 { return } // iz
    d_xdenom = dv0_ac * dv1_ab - dv0_ab * dv1_ac;
    if d_xdenom < 0 as libc::c_int {
        a_spans = spans.as_mut_ptr();
        r_p0[0 as libc::c_int as usize] = (*aliastriangleparms.a).u;
        r_p0[1 as libc::c_int as usize] = (*aliastriangleparms.a).v;
        r_p0[2 as libc::c_int as usize] = (*aliastriangleparms.a).s;
        r_p0[3 as libc::c_int as usize] = (*aliastriangleparms.a).t;
        r_p0[4 as libc::c_int as usize] = (*aliastriangleparms.a).l;
        r_p0[5 as libc::c_int as usize] = (*aliastriangleparms.a).zi;
        r_p1[0 as libc::c_int as usize] = (*aliastriangleparms.b).u;
        r_p1[1 as libc::c_int as usize] = (*aliastriangleparms.b).v;
        r_p1[2 as libc::c_int as usize] = (*aliastriangleparms.b).s;
        r_p1[3 as libc::c_int as usize] = (*aliastriangleparms.b).t;
        r_p1[4 as libc::c_int as usize] = (*aliastriangleparms.b).l;
        r_p1[5 as libc::c_int as usize] = (*aliastriangleparms.b).zi;
        r_p2[0 as libc::c_int as usize] = (*aliastriangleparms.c).u;
        r_p2[1 as libc::c_int as usize] = (*aliastriangleparms.c).v;
        r_p2[2 as libc::c_int as usize] = (*aliastriangleparms.c).s;
        r_p2[3 as libc::c_int as usize] = (*aliastriangleparms.c).t;
        r_p2[4 as libc::c_int as usize] = (*aliastriangleparms.c).l;
        r_p2[5 as libc::c_int as usize] = (*aliastriangleparms.c).zi;
        R_PolysetSetEdgeTable();
        R_RasterizeAliasPolySmooth();
    };
}
static mut skinend: *mut pixel_t = 0 as *const pixel_t as *mut pixel_t;
#[inline]
unsafe extern "C" fn R_PolysetCheckBounds(mut lptex: *mut pixel_t,
                                          mut lsfrac: libc::c_int,
                                          mut ltfrac: libc::c_int,
                                          mut lcount: libc::c_int)
 -> qboolean {
    let mut start: *mut pixel_t = 0 as *mut pixel_t;
    let mut end: *mut pixel_t = 0 as *mut pixel_t;
    start = r_affinetridesc.pskin as *mut pixel_t;
    end = skinend;
    // span is linear, so only need to check first and last
    if (lptex.wrapping_offset_from(start) as libc::c_long) <
           0 as libc::c_int as libc::c_long ||
           lptex.wrapping_offset_from(end) as libc::c_long >=
               0 as libc::c_int as libc::c_long {
        return false_0
    }
    lcount -= 1;
    if lcount == 0 { return true_0 }
    lptex =
        lptex.offset((a_ststepxwhole * lcount) as
                         isize).offset((lsfrac + a_sstepxfrac * lcount >>
                                            16 as libc::c_int) as
                                           isize).offset(((ltfrac +
                                                               a_tstepxfrac *
                                                                   lcount >>
                                                               16 as
                                                                   libc::c_int)
                                                              *
                                                              r_affinetridesc.skinwidth)
                                                             as isize);
    if (lptex.wrapping_offset_from(start) as libc::c_long) <
           0 as libc::c_int as libc::c_long ||
           lptex.wrapping_offset_from(end) as libc::c_long >=
               0 as libc::c_int as libc::c_long {
        return false_0
    }
    return true_0;
}
/*
===================
R_PolysetScanLeftEdge_C
====================
*/
#[no_mangle]
pub unsafe extern "C" fn R_PolysetScanLeftEdge_C(mut height: libc::c_int)
 -> qboolean {
    loop  {
        (*d_pedgespanpackage).pdest = d_pdest as *mut libc::c_void;
        (*d_pedgespanpackage).pz = d_pz;
        (*d_pedgespanpackage).count = d_aspancount;
        (*d_pedgespanpackage).ptex = d_ptex;
        (*d_pedgespanpackage).sfrac = d_sfrac;
        (*d_pedgespanpackage).tfrac = d_tfrac;
        // FIXME: need to clamp l, s, t, at both ends?
        (*d_pedgespanpackage).light = d_light;
        (*d_pedgespanpackage).zi = d_zi;
        d_pedgespanpackage = d_pedgespanpackage.offset(1);
        errorterm += erroradjustup;
        if errorterm >= 0 as libc::c_int {
            d_pdest = d_pdest.offset(d_pdestextrastep as isize);
            d_pz = d_pz.offset(d_pzextrastep as isize);
            d_aspancount += d_countextrastep;
            d_ptex = d_ptex.offset(d_ptexextrastep as isize);
            d_sfrac += d_sfracextrastep;
            d_ptex = d_ptex.offset((d_sfrac >> 16 as libc::c_int) as isize);
            d_sfrac &= 0xffff as libc::c_int;
            d_tfrac += d_tfracextrastep;
            if d_tfrac & 0x10000 as libc::c_int != 0 {
                d_ptex = d_ptex.offset(r_affinetridesc.skinwidth as isize);
                d_tfrac &= 0xffff as libc::c_int
            }
            d_light += d_lightextrastep;
            d_zi += d_ziextrastep;
            errorterm -= erroradjustdown
        } else {
            d_pdest = d_pdest.offset(d_pdestbasestep as isize);
            d_pz = d_pz.offset(d_pzbasestep as isize);
            d_aspancount += ubasestep;
            d_ptex = d_ptex.offset(d_ptexbasestep as isize);
            d_sfrac += d_sfracbasestep;
            d_ptex = d_ptex.offset((d_sfrac >> 16 as libc::c_int) as isize);
            d_sfrac &= 0xffff as libc::c_int;
            d_tfrac += d_tfracbasestep;
            if d_tfrac & 0x10000 as libc::c_int != 0 {
                d_ptex = d_ptex.offset(r_affinetridesc.skinwidth as isize);
                d_tfrac &= 0xffff as libc::c_int
            }
            d_light += d_lightbasestep;
            d_zi += d_zibasestep
        }
        height -= 1;
        if !(height != 0) { break ; }
    }
    return true_0;
}
/*
===================
FloorDivMod

Returns mathematically correct (floor-based) quotient and remainder for
numer and denom, both of which should contain no fractional part. The
quotient must fit in 32 bits.
FIXME: GET RID OF THIS! (FloorDivMod)
====================
*/
#[no_mangle]
pub unsafe extern "C" fn FloorDivMod(mut numer: libc::c_float,
                                     mut denom: libc::c_float,
                                     mut quotient: *mut libc::c_int,
                                     mut rem: *mut libc::c_int) {
    let mut q: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut x: libc::c_float = 0.;
    if numer >= 0.0f32 {
        x = __tg_floor(numer / denom);
        q = x as libc::c_int;
        r = __tg_floor(numer - x * denom) as libc::c_int
    } else {
        //
	// perform operations with positive values, and fix mod to make floor-based
	//
        x = __tg_floor(-numer / denom);
        q = -(x as libc::c_int);
        r = __tg_floor(-numer - x * denom) as libc::c_int;
        if r != 0 as libc::c_int { q -= 1; r = denom as libc::c_int - r }
    }
    if q > 2147483647 as libc::c_int / 2 as libc::c_int ||
           q <
               (-(2147483647 as libc::c_int) - 1 as libc::c_int) /
                   2 as libc::c_int {
        let mut i: libc::c_int = 0;
        d_pdrawspans =
            Some(R_PolysetStub as
                     unsafe extern "C" fn(_: *mut spanpackage_t) -> ());
        gEngfuncs.Con_Printf.expect("non-null function pointer")(b"^1Error:^7 FloorDivMod: q overflow!\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char);
        q = 1 as libc::c_int
    }
    if r > 2147483647 as libc::c_int / 2 as libc::c_int ||
           r <
               (-(2147483647 as libc::c_int) - 1 as libc::c_int) /
                   2 as libc::c_int {
        let mut i_0: libc::c_int = 0;
        d_pdrawspans =
            Some(R_PolysetStub as
                     unsafe extern "C" fn(_: *mut spanpackage_t) -> ());
        gEngfuncs.Con_Printf.expect("non-null function pointer")(b"^1Error:^7 FloorDivMod: r overflow!\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char);
        r = 1 as libc::c_int
    }
    *quotient = q;
    *rem = r;
}
/*
===================
R_PolysetSetUpForLineScan
====================
*/
#[no_mangle]
pub unsafe extern "C" fn R_PolysetSetUpForLineScan(mut startvertu: fixed8_t,
                                                   mut startvertv: fixed8_t,
                                                   mut endvertu: fixed8_t,
                                                   mut endvertv: fixed8_t) {
    let mut dm: libc::c_float = 0.;
    let mut dn: libc::c_float = 0.;
    let mut tm: libc::c_int = 0;
    let mut tn: libc::c_int = 0;
    let mut ptemp: *mut adivtab_t = 0 as *mut adivtab_t;
    // TODO: implement x86 version
    errorterm = -(1 as libc::c_int);
    tm = endvertu - startvertu;
    tn = endvertv - startvertv;
    if tm <= 16 as libc::c_int && tm >= -(15 as libc::c_int) &&
           (tn <= 16 as libc::c_int && tn >= -(15 as libc::c_int)) {
        ptemp =
            &mut *adivtab.as_mut_ptr().offset((((tm + 15 as libc::c_int) <<
                                                    5 as libc::c_int) +
                                                   (tn + 15 as libc::c_int))
                                                  as isize) as *mut adivtab_t;
        ubasestep = (*ptemp).quotient;
        erroradjustup = (*ptemp).remainder;
        erroradjustdown = tn
    } else {
        dm = tm as libc::c_float;
        dn = tn as libc::c_float;
        FloorDivMod(dm, dn, &mut ubasestep, &mut erroradjustup);
        erroradjustdown = dn as libc::c_int
    };
}
/*
================
R_PolysetCalcGradients
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_PolysetCalcGradients(mut skinwidth_0: libc::c_int)
 -> qboolean {
    let mut xstepdenominv: libc::c_float = 0.;
    let mut ystepdenominv: libc::c_float = 0.;
    let mut t0: libc::c_float = 0.;
    let mut t1: libc::c_float = 0.;
    let mut p01_minus_p21: libc::c_float = 0.;
    let mut p11_minus_p21: libc::c_float = 0.;
    let mut p00_minus_p20: libc::c_float = 0.;
    let mut p10_minus_p20: libc::c_float = 0.;
    p00_minus_p20 =
        (r_p0[0 as libc::c_int as usize] - r_p2[0 as libc::c_int as usize]) as
            libc::c_float;
    p01_minus_p21 =
        (r_p0[1 as libc::c_int as usize] - r_p2[1 as libc::c_int as usize]) as
            libc::c_float;
    p10_minus_p20 =
        (r_p1[0 as libc::c_int as usize] - r_p2[0 as libc::c_int as usize]) as
            libc::c_float;
    p11_minus_p21 =
        (r_p1[1 as libc::c_int as usize] - r_p2[1 as libc::c_int as usize]) as
            libc::c_float;
    /*printf("gradients for triangle\n");
	printf("%d %d %d %d %d %d\n" ,  r_p0[0], r_p0[1], r_p0[2] >> 16, r_p0[3] >> 16, r_p0[4], r_p0[5]);
	printf("%d %d %d %d %d %d\n" ,  r_p1[0], r_p1[1], r_p1[2] >> 16, r_p1[3] >> 16, r_p1[4], r_p1[5]);
	printf("%d %d %d %d %d %d\n\n", r_p2[0], r_p2[1], r_p2[2] >> 16, r_p2[3] >> 16, r_p2[4], r_p2[5]);
*/
    xstepdenominv = 1.0f32 / d_xdenom as libc::c_float;
    ystepdenominv = -xstepdenominv;
    // ceil () for light so positive steps are exaggerated, negative steps
// diminished,  pushing us away from underflow toward overflow. Underflow is
// very visible, overflow is very unlikely, because of ambient lighting
    t0 =
        (r_p0[4 as libc::c_int as usize] - r_p2[4 as libc::c_int as usize]) as
            libc::c_float;
    t1 =
        (r_p1[4 as libc::c_int as usize] - r_p2[4 as libc::c_int as usize]) as
            libc::c_float;
    r_lstepx =
        __tg_ceil((t1 * p01_minus_p21 - t0 * p11_minus_p21) * xstepdenominv)
            as libc::c_int;
    r_lstepy =
        __tg_ceil((t1 * p00_minus_p20 - t0 * p10_minus_p20) * ystepdenominv)
            as libc::c_int;
    t0 =
        (r_p0[2 as libc::c_int as usize] - r_p2[2 as libc::c_int as usize]) as
            libc::c_float;
    t1 =
        (r_p1[2 as libc::c_int as usize] - r_p2[2 as libc::c_int as usize]) as
            libc::c_float;
    r_sstepx =
        ((t1 * p01_minus_p21 - t0 * p11_minus_p21) * xstepdenominv) as
            libc::c_int;
    r_sstepy =
        ((t1 * p00_minus_p20 - t0 * p10_minus_p20) * ystepdenominv) as
            libc::c_int;
    t0 =
        (r_p0[3 as libc::c_int as usize] - r_p2[3 as libc::c_int as usize]) as
            libc::c_float;
    t1 =
        (r_p1[3 as libc::c_int as usize] - r_p2[3 as libc::c_int as usize]) as
            libc::c_float;
    r_tstepx =
        ((t1 * p01_minus_p21 - t0 * p11_minus_p21) * xstepdenominv) as
            libc::c_int;
    r_tstepy =
        ((t1 * p00_minus_p20 - t0 * p10_minus_p20) * ystepdenominv) as
            libc::c_int;
    t0 =
        (r_p0[5 as libc::c_int as usize] - r_p2[5 as libc::c_int as usize]) as
            libc::c_float;
    t1 =
        (r_p1[5 as libc::c_int as usize] - r_p2[5 as libc::c_int as usize]) as
            libc::c_float;
    r_zistepx =
        ((t1 * p01_minus_p21 - t0 * p11_minus_p21) * xstepdenominv) as
            libc::c_int;
    r_zistepy =
        ((t1 * p00_minus_p20 - t0 * p10_minus_p20) * ystepdenominv) as
            libc::c_int;
    /*if( r_zistepx > INT_MAX / 2 )
		return false;
	if( r_zistepx < INT_MIN / 2 )
		return false;
	if( r_zistepy > INT_MAX / 2 )
		return false;
	if( r_zistepy < INT_MIN / 2 )
		return false;*/
    //#if	id386ALIAS
    //#else
    a_sstepxfrac = r_sstepx & 0xffff as libc::c_int;
    a_tstepxfrac = r_tstepx & 0xffff as libc::c_int;
    //#endif
    // do not allow big steps to make 512 byte extra bounds enough (still f**ng not)
	/*if( r_sstepx <= -65535*8 )
		return false;
	if( r_tstepx <= -65535*8)
		return false;
	if( r_sstepx >= 65535*8 )
		return false;
	if( r_tstepx >= 65535*8 )
		return false;*/
    a_ststepxwhole =
        skinwidth_0 * (r_tstepx >> 16 as libc::c_int) +
            (r_sstepx >> 16 as libc::c_int);
    //	printf("%d %d %d %d\n",a_ststepxwhole, r_sstepx, r_tstepx, skinwidth );
    skinend =
        (r_affinetridesc.pskin as
             *mut pixel_t).offset((r_affinetridesc.skinwidth *
                                       r_affinetridesc.skinheight) as isize);
    return true_0;
}
/*
================
R_PolysetDrawSpans8
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_PolysetDrawSpansBlended(mut pspanpackage:
                                                       *mut spanpackage_t) {
    let mut lcount: libc::c_int =
        0; //vid.colormap[*lptex + ( llight & 0xFF00 )];
    let mut lpdest: *mut pixel_t = 0 as *mut pixel_t;
    let mut lptex: *mut pixel_t = 0 as *mut pixel_t;
    let mut lsfrac: libc::c_int = 0;
    let mut ltfrac: libc::c_int = 0;
    let mut llight: libc::c_int = 0;
    let mut lzi: libc::c_int = 0;
    let mut lpz: *mut libc::c_short = 0 as *mut libc::c_short;
    loop  {
        lcount = d_aspancount - (*pspanpackage).count;
        errorterm += erroradjustup;
        if errorterm >= 0 as libc::c_int {
            d_aspancount += d_countextrastep;
            errorterm -= erroradjustdown
        } else { d_aspancount += ubasestep }
        if lcount != 0 {
            lpdest = (*pspanpackage).pdest as *mut pixel_t;
            lptex = (*pspanpackage).ptex;
            lpz = (*pspanpackage).pz;
            lsfrac = (*pspanpackage).sfrac;
            ltfrac = (*pspanpackage).tfrac;
            llight = (*pspanpackage).light;
            lzi = (*pspanpackage).zi;
            pspanpackage = pspanpackage.offset(1);
            if !(R_PolysetCheckBounds(lptex, lsfrac, ltfrac, lcount) as u64 ==
                     0) {
                loop  {
                    if lzi >> 16 as libc::c_int >= *lpz as libc::c_int {
                        let mut temp: pixel_t = *lptex;
                        let mut alpha: libc::c_int = vid.alpha as libc::c_int;
                        temp =
                            ((vid.modmap[(temp as libc::c_int &
                                              0xff00 as libc::c_int |
                                              vid.color as libc::c_int >>
                                                  8 as libc::c_int) as usize]
                                  as libc::c_int) << 8 as libc::c_int |
                                 temp as libc::c_int &
                                     vid.color as libc::c_int &
                                     0xff as libc::c_int |
                                 (temp as libc::c_int & 0xff as libc::c_int)
                                     >> 3 as libc::c_int) as pixel_t;
                        if alpha == 7 as libc::c_int {
                            *lpdest = temp
                        } else if alpha != 0 {
                            *lpdest =
                                if alpha > 3 as libc::c_int {
                                    (vid.alphamap[(7 as libc::c_int -
                                                       1 as libc::c_int -
                                                       alpha <<
                                                       18 as libc::c_int |
                                                       (*lpdest as libc::c_int
                                                            &
                                                            0xff00 as
                                                                libc::c_int)
                                                           << 2 as libc::c_int
                                                       |
                                                       temp as libc::c_int >>
                                                           6 as libc::c_int)
                                                      as usize] as
                                         libc::c_int) |
                                        temp as libc::c_int &
                                            0x3f as libc::c_int
                                } else {
                                    (vid.alphamap[((alpha - 1 as libc::c_int)
                                                       << 18 as libc::c_int |
                                                       (temp as libc::c_int &
                                                            0xff00 as
                                                                libc::c_int)
                                                           << 2 as libc::c_int
                                                       |
                                                       *lpdest as libc::c_int
                                                           >>
                                                           6 as libc::c_int)
                                                      as usize] as
                                         libc::c_int) |
                                        *lpdest as libc::c_int &
                                            0x3f as libc::c_int
                                } as pixel_t
                        }
                    }
                    lpdest = lpdest.offset(1);
                    lzi += r_zistepx;
                    lpz = lpz.offset(1);
                    llight += r_lstepx;
                    lptex = lptex.offset(a_ststepxwhole as isize);
                    lsfrac += a_sstepxfrac;
                    lptex =
                        lptex.offset((lsfrac >> 16 as libc::c_int) as isize);
                    lsfrac &= 0xffff as libc::c_int;
                    ltfrac += a_tstepxfrac;
                    if ltfrac & 0x10000 as libc::c_int != 0 {
                        lptex =
                            lptex.offset(r_affinetridesc.skinwidth as isize);
                        ltfrac &= 0xffff as libc::c_int
                    }
                    lcount -= 1;
                    if !(lcount != 0) { break ; }
                }
            }
        } else { pspanpackage = pspanpackage.offset(1) }
        if !((*pspanpackage).count != -(999999 as libc::c_int)) { break ; }
    };
}
/*
================
R_PolysetDrawSpans8
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_PolysetDrawSpansAdditive(mut pspanpackage:
                                                        *mut spanpackage_t) {
    let mut lcount: libc::c_int =
        0; //vid.colormap[*lptex + ( llight & 0xFF00 )];
    let mut lpdest: *mut pixel_t = 0 as *mut pixel_t;
    let mut lptex: *mut pixel_t = 0 as *mut pixel_t;
    let mut lsfrac: libc::c_int = 0;
    let mut ltfrac: libc::c_int = 0;
    let mut llight: libc::c_int = 0;
    let mut lzi: libc::c_int = 0;
    let mut lpz: *mut libc::c_short = 0 as *mut libc::c_short;
    loop  {
        lcount = d_aspancount - (*pspanpackage).count;
        errorterm += erroradjustup;
        if errorterm >= 0 as libc::c_int {
            d_aspancount += d_countextrastep;
            errorterm -= erroradjustdown
        } else { d_aspancount += ubasestep }
        if lcount != 0 {
            lpdest = (*pspanpackage).pdest as *mut pixel_t;
            lptex = (*pspanpackage).ptex;
            lpz = (*pspanpackage).pz;
            lsfrac = (*pspanpackage).sfrac;
            ltfrac = (*pspanpackage).tfrac;
            llight = (*pspanpackage).light;
            lzi = (*pspanpackage).zi;
            pspanpackage = pspanpackage.offset(1);
            if !(R_PolysetCheckBounds(lptex, lsfrac, ltfrac, lcount) as u64 ==
                     0) {
                loop  {
                    if lzi >> 16 as libc::c_int >= *lpz as libc::c_int {
                        let mut temp: pixel_t = *lptex;
                        temp =
                            ((vid.modmap[(temp as libc::c_int &
                                              0xff00 as libc::c_int |
                                              vid.color as libc::c_int >>
                                                  8 as libc::c_int) as usize]
                                  as libc::c_int) << 8 as libc::c_int |
                                 temp as libc::c_int &
                                     vid.color as libc::c_int &
                                     0xff as libc::c_int |
                                 (temp as libc::c_int & 0xff as libc::c_int)
                                     >> 3 as libc::c_int) as pixel_t;
                        *lpdest =
                            ((vid.addmap[(temp as libc::c_int &
                                              0xff00 as libc::c_int |
                                              *lpdest as libc::c_int >>
                                                  8 as libc::c_int) as usize]
                                  as libc::c_int) << 8 as libc::c_int |
                                 *lpdest as libc::c_int & 0xff as libc::c_int
                                 |
                                 (temp as libc::c_int & 0xff as libc::c_int)
                                     >> 0 as libc::c_int) as pixel_t
                    }
                    lpdest = lpdest.offset(1);
                    lzi += r_zistepx;
                    lpz = lpz.offset(1);
                    llight += r_lstepx;
                    lptex = lptex.offset(a_ststepxwhole as isize);
                    lsfrac += a_sstepxfrac;
                    lptex =
                        lptex.offset((lsfrac >> 16 as libc::c_int) as isize);
                    lsfrac &= 0xffff as libc::c_int;
                    ltfrac += a_tstepxfrac;
                    if ltfrac & 0x10000 as libc::c_int != 0 {
                        lptex =
                            lptex.offset(r_affinetridesc.skinwidth as isize);
                        ltfrac &= 0xffff as libc::c_int
                    }
                    lcount -= 1;
                    if !(lcount != 0) { break ; }
                }
            }
        } else { pspanpackage = pspanpackage.offset(1) }
        if !((*pspanpackage).count != -(999999 as libc::c_int)) { break ; }
    };
}
/*
================
R_PolysetDrawSpans8
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_PolysetDrawSpansGlow(mut pspanpackage:
                                                    *mut spanpackage_t) {
    let mut lcount: libc::c_int = 0;
    let mut lpdest: *mut pixel_t = 0 as *mut pixel_t;
    let mut lptex: *mut pixel_t = 0 as *mut pixel_t;
    let mut lsfrac: libc::c_int = 0;
    let mut ltfrac: libc::c_int = 0;
    let mut llight: libc::c_int = 0;
    let mut lzi: libc::c_int = 0;
    let mut lpz: *mut libc::c_short = 0 as *mut libc::c_short;
    loop  {
        lcount = d_aspancount - (*pspanpackage).count;
        errorterm += erroradjustup;
        if errorterm >= 0 as libc::c_int {
            d_aspancount += d_countextrastep;
            errorterm -= erroradjustdown
        } else { d_aspancount += ubasestep }
        if lcount != 0 {
            lpdest = (*pspanpackage).pdest as *mut pixel_t;
            lptex = (*pspanpackage).ptex;
            lpz = (*pspanpackage).pz;
            lsfrac = (*pspanpackage).sfrac;
            ltfrac = (*pspanpackage).tfrac;
            llight = (*pspanpackage).light;
            lzi = (*pspanpackage).zi;
            pspanpackage = pspanpackage.offset(1);
            if !(R_PolysetCheckBounds(lptex, lsfrac, ltfrac, lcount) as u64 ==
                     0) {
                loop  {
                    //if ((lzi >> 16) >= *lpz)
                    let mut temp: pixel_t =
                        *lptex; //vid.colormap[*lptex + ( llight & 0xFF00 )];
                    temp =
                        ((vid.modmap[(temp as libc::c_int &
                                          0xff00 as libc::c_int |
                                          vid.color as libc::c_int >>
                                              8 as libc::c_int) as usize] as
                              libc::c_int) << 8 as libc::c_int |
                             temp as libc::c_int & vid.color as libc::c_int &
                                 0xff as libc::c_int |
                             (temp as libc::c_int & 0xff as libc::c_int) >>
                                 3 as libc::c_int) as pixel_t;
                    *lpdest =
                        ((vid.addmap[(temp as libc::c_int &
                                          0xff00 as libc::c_int |
                                          *lpdest as libc::c_int >>
                                              8 as libc::c_int) as usize] as
                              libc::c_int) << 8 as libc::c_int |
                             *lpdest as libc::c_int & 0xff as libc::c_int |
                             (temp as libc::c_int & 0xff as libc::c_int) >>
                                 0 as libc::c_int) as pixel_t;
                    lpdest = lpdest.offset(1);
                    lzi += r_zistepx;
                    lpz = lpz.offset(1);
                    llight += r_lstepx;
                    lptex = lptex.offset(a_ststepxwhole as isize);
                    lsfrac += a_sstepxfrac;
                    lptex =
                        lptex.offset((lsfrac >> 16 as libc::c_int) as isize);
                    lsfrac &= 0xffff as libc::c_int;
                    ltfrac += a_tstepxfrac;
                    if ltfrac & 0x10000 as libc::c_int != 0 {
                        lptex =
                            lptex.offset(r_affinetridesc.skinwidth as isize);
                        ltfrac &= 0xffff as libc::c_int
                    }
                    lcount -= 1;
                    if !(lcount != 0) { break ; }
                }
            }
        } else { pspanpackage = pspanpackage.offset(1) }
        if !((*pspanpackage).count != -(999999 as libc::c_int)) { break ; }
    };
}
/*
================
R_PolysetDrawSpans8
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_PolysetDrawSpansTextureBlended(mut pspanpackage:
                                                              *mut spanpackage_t) {
    let mut lcount: libc::c_int =
        0; //vid.colormap[*lptex + ( llight & 0xFF00 )];
    let mut lpdest: *mut pixel_t = 0 as *mut pixel_t;
    let mut lptex: *mut pixel_t = 0 as *mut pixel_t;
    let mut lsfrac: libc::c_int = 0;
    let mut ltfrac: libc::c_int = 0;
    let mut llight: libc::c_int = 0;
    let mut lzi: libc::c_int = 0;
    let mut lpz: *mut libc::c_short = 0 as *mut libc::c_short;
    loop  {
        lcount = d_aspancount - (*pspanpackage).count;
        errorterm += erroradjustup;
        if errorterm >= 0 as libc::c_int {
            d_aspancount += d_countextrastep;
            errorterm -= erroradjustdown
        } else { d_aspancount += ubasestep }
        if lcount != 0 {
            lpdest = (*pspanpackage).pdest as *mut pixel_t;
            lptex = (*pspanpackage).ptex;
            lpz = (*pspanpackage).pz;
            lsfrac = (*pspanpackage).sfrac;
            ltfrac = (*pspanpackage).tfrac;
            llight = (*pspanpackage).light;
            lzi = (*pspanpackage).zi;
            pspanpackage = pspanpackage.offset(1);
            if !(R_PolysetCheckBounds(lptex, lsfrac, ltfrac, lcount) as u64 ==
                     0) {
                loop  {
                    if lzi >> 16 as libc::c_int >= *lpz as libc::c_int {
                        let mut temp: pixel_t = *lptex;
                        let mut alpha: libc::c_int =
                            temp as libc::c_int >> 13 as libc::c_int;
                        temp =
                            ((temp as libc::c_int) << 3 as libc::c_int) as
                                pixel_t;
                        temp =
                            ((vid.modmap[(temp as libc::c_int &
                                              0xff00 as libc::c_int |
                                              vid.color as libc::c_int >>
                                                  8 as libc::c_int) as usize]
                                  as libc::c_int) << 8 as libc::c_int |
                                 temp as libc::c_int &
                                     vid.color as libc::c_int &
                                     0xff as libc::c_int |
                                 (temp as libc::c_int & 0xff as libc::c_int)
                                     >> 3 as libc::c_int) as pixel_t;
                        if alpha == 7 as libc::c_int {
                            *lpdest = temp
                        } else if alpha != 0 {
                            *lpdest =
                                if alpha > 3 as libc::c_int {
                                    (vid.alphamap[(7 as libc::c_int -
                                                       1 as libc::c_int -
                                                       alpha <<
                                                       18 as libc::c_int |
                                                       (*lpdest as libc::c_int
                                                            &
                                                            0xff00 as
                                                                libc::c_int)
                                                           << 2 as libc::c_int
                                                       |
                                                       temp as libc::c_int >>
                                                           6 as libc::c_int)
                                                      as usize] as
                                         libc::c_int) |
                                        temp as libc::c_int &
                                            0x3f as libc::c_int
                                } else {
                                    (vid.alphamap[((alpha - 1 as libc::c_int)
                                                       << 18 as libc::c_int |
                                                       (temp as libc::c_int &
                                                            0xff00 as
                                                                libc::c_int)
                                                           << 2 as libc::c_int
                                                       |
                                                       *lpdest as libc::c_int
                                                           >>
                                                           6 as libc::c_int)
                                                      as usize] as
                                         libc::c_int) |
                                        *lpdest as libc::c_int &
                                            0x3f as libc::c_int
                                } as pixel_t
                        }
                    }
                    lpdest = lpdest.offset(1);
                    lzi += r_zistepx;
                    lpz = lpz.offset(1);
                    llight += r_lstepx;
                    lptex = lptex.offset(a_ststepxwhole as isize);
                    lsfrac += a_sstepxfrac;
                    lptex =
                        lptex.offset((lsfrac >> 16 as libc::c_int) as isize);
                    lsfrac &= 0xffff as libc::c_int;
                    ltfrac += a_tstepxfrac;
                    if ltfrac & 0x10000 as libc::c_int != 0 {
                        lptex =
                            lptex.offset(r_affinetridesc.skinwidth as isize);
                        ltfrac &= 0xffff as libc::c_int
                    }
                    lcount -= 1;
                    if !(lcount != 0) { break ; }
                }
            }
        } else { pspanpackage = pspanpackage.offset(1) }
        if !((*pspanpackage).count != -(999999 as libc::c_int)) { break ; }
    };
}
/*
================
R_PolysetDrawSpans8
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_PolysetDrawSpans8_33(mut pspanpackage:
                                                    *mut spanpackage_t) {
    let mut lcount: libc::c_int =
        0; //vid.colormap[*lptex + ( llight & 0xFF00 )];
    let mut lpdest: *mut pixel_t = 0 as *mut pixel_t;
    let mut lptex: *mut pixel_t = 0 as *mut pixel_t;
    let mut lsfrac: libc::c_int = 0;
    let mut ltfrac: libc::c_int = 0;
    let mut llight: libc::c_int = 0;
    let mut lzi: libc::c_int = 0;
    let mut lpz: *mut libc::c_short = 0 as *mut libc::c_short;
    loop  {
        lcount = d_aspancount - (*pspanpackage).count;
        errorterm += erroradjustup;
        if errorterm >= 0 as libc::c_int {
            d_aspancount += d_countextrastep;
            errorterm -= erroradjustdown
        } else { d_aspancount += ubasestep }
        if lcount != 0 {
            lpdest = (*pspanpackage).pdest as *mut pixel_t;
            lptex = (*pspanpackage).ptex;
            lpz = (*pspanpackage).pz;
            lsfrac = (*pspanpackage).sfrac;
            ltfrac = (*pspanpackage).tfrac;
            llight = (*pspanpackage).light;
            lzi = (*pspanpackage).zi;
            loop  {
                if lzi >> 16 as libc::c_int >= *lpz as libc::c_int {
                    let mut temp: pixel_t = *lptex;
                    let mut alpha: libc::c_int =
                        (tr.blend * 7 as libc::c_int as libc::c_float) as
                            libc::c_int;
                    if alpha == 7 as libc::c_int {
                        *lpdest = temp
                    } else if alpha != 0 {
                        *lpdest =
                            if alpha > 3 as libc::c_int {
                                (vid.alphamap[(7 as libc::c_int -
                                                   1 as libc::c_int - alpha <<
                                                   18 as libc::c_int |
                                                   (*lpdest as libc::c_int &
                                                        0xff00 as libc::c_int)
                                                       << 2 as libc::c_int |
                                                   temp as libc::c_int >>
                                                       6 as libc::c_int) as
                                                  usize] as libc::c_int) |
                                    temp as libc::c_int & 0x3f as libc::c_int
                            } else {
                                (vid.alphamap[((alpha - 1 as libc::c_int) <<
                                                   18 as libc::c_int |
                                                   (temp as libc::c_int &
                                                        0xff00 as libc::c_int)
                                                       << 2 as libc::c_int |
                                                   *lpdest as libc::c_int >>
                                                       6 as libc::c_int) as
                                                  usize] as libc::c_int) |
                                    *lpdest as libc::c_int &
                                        0x3f as libc::c_int
                            } as pixel_t
                    }
                }
                lpdest = lpdest.offset(1);
                lzi += r_zistepx;
                lpz = lpz.offset(1);
                llight += r_lstepx;
                lptex = lptex.offset(a_ststepxwhole as isize);
                lsfrac += a_sstepxfrac;
                lptex = lptex.offset((lsfrac >> 16 as libc::c_int) as isize);
                lsfrac &= 0xffff as libc::c_int;
                ltfrac += a_tstepxfrac;
                if ltfrac & 0x10000 as libc::c_int != 0 {
                    lptex = lptex.offset(r_affinetridesc.skinwidth as isize);
                    ltfrac &= 0xffff as libc::c_int
                }
                lcount -= 1;
                if !(lcount != 0) { break ; }
            }
        }
        pspanpackage = pspanpackage.offset(1);
        if !((*pspanpackage).count != -(999999 as libc::c_int)) { break ; }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_PolysetDrawSpansConstant8_33(mut pspanpackage:
                                                            *mut spanpackage_t) {
    let mut lcount: libc::c_int = 0;
    let mut lpdest: *mut pixel_t = 0 as *mut pixel_t;
    let mut lzi: libc::c_int = 0;
    let mut lpz: *mut libc::c_short = 0 as *mut libc::c_short;
    loop  {
        lcount = d_aspancount - (*pspanpackage).count;
        errorterm += erroradjustup;
        if errorterm >= 0 as libc::c_int {
            d_aspancount += d_countextrastep;
            errorterm -= erroradjustdown
        } else { d_aspancount += ubasestep }
        if lcount != 0 {
            lpdest = (*pspanpackage).pdest as *mut pixel_t;
            lpz = (*pspanpackage).pz;
            lzi = (*pspanpackage).zi;
            loop  {
                if lzi >> 16 as libc::c_int >= *lpz as libc::c_int {
                    *lpdest =
                        if 2 as libc::c_int > 3 as libc::c_int {
                            (vid.alphamap[((7 as libc::c_int -
                                                1 as libc::c_int -
                                                2 as libc::c_int) <<
                                               18 as libc::c_int |
                                               (*lpdest as libc::c_int &
                                                    0xff00 as libc::c_int) <<
                                                   2 as libc::c_int |
                                               r_aliasblendcolor >>
                                                   6 as libc::c_int) as usize]
                                 as libc::c_int) |
                                r_aliasblendcolor & 0x3f as libc::c_int
                        } else {
                            (vid.alphamap[((2 as libc::c_int -
                                                1 as libc::c_int) <<
                                               18 as libc::c_int |
                                               (r_aliasblendcolor &
                                                    0xff00 as libc::c_int) <<
                                                   2 as libc::c_int |
                                               *lpdest as libc::c_int >>
                                                   6 as libc::c_int) as usize]
                                 as libc::c_int) |
                                *lpdest as libc::c_int & 0x3f as libc::c_int
                        } as pixel_t
                    //vid.alphamap[r_aliasblendcolor + *lpdest*256];
                } //vid.alphamap[temp*256 + *lpdest];
                lpdest = lpdest.offset(1);
                lzi += r_zistepx;
                lpz = lpz.offset(1);
                lcount -= 1;
                if !(lcount != 0) { break ; }
            }
        }
        pspanpackage = pspanpackage.offset(1);
        if !((*pspanpackage).count != -(999999 as libc::c_int)) { break ; }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_PolysetDrawSpans8_66(mut pspanpackage:
                                                    *mut spanpackage_t) {
    let mut lcount: libc::c_int = 0;
    let mut lpdest: *mut pixel_t = 0 as *mut pixel_t;
    let mut lptex: *mut pixel_t = 0 as *mut pixel_t;
    let mut lsfrac: libc::c_int = 0;
    let mut ltfrac: libc::c_int = 0;
    let mut llight: libc::c_int = 0;
    let mut lzi: libc::c_int = 0;
    let mut lpz: *mut libc::c_short = 0 as *mut libc::c_short;
    loop  {
        lcount = d_aspancount - (*pspanpackage).count;
        errorterm += erroradjustup;
        if errorterm >= 0 as libc::c_int {
            d_aspancount += d_countextrastep;
            errorterm -= erroradjustdown
        } else { d_aspancount += ubasestep }
        if lcount != 0 {
            lpdest = (*pspanpackage).pdest as *mut pixel_t;
            lptex = (*pspanpackage).ptex;
            lpz = (*pspanpackage).pz;
            lsfrac = (*pspanpackage).sfrac;
            ltfrac = (*pspanpackage).tfrac;
            llight = (*pspanpackage).light;
            lzi = (*pspanpackage).zi;
            loop  {
                if lzi >> 16 as libc::c_int >= *lpz as libc::c_int {
                    let mut temp: libc::c_int =
                        vid.colormap[(*lptex as libc::c_int +
                                          (llight & 0xff00 as libc::c_int)) as
                                         usize] as libc::c_int;
                    *lpdest =
                        if 5 as libc::c_int > 3 as libc::c_int {
                            (vid.alphamap[((7 as libc::c_int -
                                                1 as libc::c_int -
                                                5 as libc::c_int) <<
                                               18 as libc::c_int |
                                               (*lpdest as libc::c_int &
                                                    0xff00 as libc::c_int) <<
                                                   2 as libc::c_int |
                                               temp >> 6 as libc::c_int) as
                                              usize] as libc::c_int) |
                                temp & 0x3f as libc::c_int
                        } else {
                            (vid.alphamap[((5 as libc::c_int -
                                                1 as libc::c_int) <<
                                               18 as libc::c_int |
                                               (temp & 0xff00 as libc::c_int)
                                                   << 2 as libc::c_int |
                                               *lpdest as libc::c_int >>
                                                   6 as libc::c_int) as usize]
                                 as libc::c_int) |
                                *lpdest as libc::c_int & 0x3f as libc::c_int
                        } as pixel_t;
                    *lpz = (lzi >> 16 as libc::c_int) as libc::c_short
                }
                lpdest = lpdest.offset(1);
                lzi += r_zistepx;
                lpz = lpz.offset(1);
                llight += r_lstepx;
                lptex = lptex.offset(a_ststepxwhole as isize);
                lsfrac += a_sstepxfrac;
                lptex = lptex.offset((lsfrac >> 16 as libc::c_int) as isize);
                lsfrac &= 0xffff as libc::c_int;
                ltfrac += a_tstepxfrac;
                if ltfrac & 0x10000 as libc::c_int != 0 {
                    lptex = lptex.offset(r_affinetridesc.skinwidth as isize);
                    ltfrac &= 0xffff as libc::c_int
                }
                lcount -= 1;
                if !(lcount != 0) { break ; }
            }
        }
        pspanpackage = pspanpackage.offset(1);
        if !((*pspanpackage).count != -(999999 as libc::c_int)) { break ; }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_PolysetDrawSpansConstant8_66(mut pspanpackage:
                                                            *mut spanpackage_t) {
    let mut lcount: libc::c_int = 0;
    let mut lpdest: *mut pixel_t = 0 as *mut pixel_t;
    let mut lzi: libc::c_int = 0;
    let mut lpz: *mut libc::c_short = 0 as *mut libc::c_short;
    loop  {
        lcount = d_aspancount - (*pspanpackage).count;
        errorterm += erroradjustup;
        if errorterm >= 0 as libc::c_int {
            d_aspancount += d_countextrastep;
            errorterm -= erroradjustdown
        } else { d_aspancount += ubasestep }
        if lcount != 0 {
            lpdest = (*pspanpackage).pdest as *mut pixel_t;
            lpz = (*pspanpackage).pz;
            lzi = (*pspanpackage).zi;
            loop  {
                if lzi >> 16 as libc::c_int >= *lpz as libc::c_int {
                    *lpdest =
                        if 5 as libc::c_int > 3 as libc::c_int {
                            (vid.alphamap[((7 as libc::c_int -
                                                1 as libc::c_int -
                                                5 as libc::c_int) <<
                                               18 as libc::c_int |
                                               (*lpdest as libc::c_int &
                                                    0xff00 as libc::c_int) <<
                                                   2 as libc::c_int |
                                               r_aliasblendcolor >>
                                                   6 as libc::c_int) as usize]
                                 as libc::c_int) |
                                r_aliasblendcolor & 0x3f as libc::c_int
                        } else {
                            (vid.alphamap[((5 as libc::c_int -
                                                1 as libc::c_int) <<
                                               18 as libc::c_int |
                                               (r_aliasblendcolor &
                                                    0xff00 as libc::c_int) <<
                                                   2 as libc::c_int |
                                               *lpdest as libc::c_int >>
                                                   6 as libc::c_int) as usize]
                                 as libc::c_int) |
                                *lpdest as libc::c_int & 0x3f as libc::c_int
                        } as pixel_t
                    //vid.alphamap[r_aliasblendcolor*256 + *lpdest];
                }
                lpdest = lpdest.offset(1);
                lzi += r_zistepx;
                lpz = lpz.offset(1);
                lcount -= 1;
                if !(lcount != 0) { break ; }
            }
        }
        pspanpackage = pspanpackage.offset(1);
        if !((*pspanpackage).count != -(999999 as libc::c_int)) { break ; }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_PolysetDrawSpans8_Opaque(mut pspanpackage:
                                                        *mut spanpackage_t) {
    let mut lcount: libc::c_int = 0;
    loop  {
        lcount = d_aspancount - (*pspanpackage).count;
        errorterm += erroradjustup;
        if errorterm >= 0 as libc::c_int {
            d_aspancount += d_countextrastep;
            errorterm -= erroradjustdown
        } else { d_aspancount += ubasestep }
        if lcount != 0 {
            let mut lsfrac: libc::c_int = 0;
            let mut ltfrac: libc::c_int = 0;
            let mut lpdest: *mut pixel_t = 0 as *mut pixel_t;
            let mut lptex: *mut pixel_t = 0 as *mut pixel_t;
            let mut llight: libc::c_int = 0;
            let mut lzi: libc::c_int = 0;
            let mut lpz: *mut libc::c_short = 0 as *mut libc::c_short;
            lpdest = (*pspanpackage).pdest as *mut pixel_t;
            lptex = (*pspanpackage).ptex;
            lpz = (*pspanpackage).pz;
            lsfrac = (*pspanpackage).sfrac;
            ltfrac = (*pspanpackage).tfrac;
            llight = (*pspanpackage).light;
            lzi = (*pspanpackage).zi;
            loop  {
                if lzi >> 16 as libc::c_int >= *lpz as libc::c_int {
                    //PGM
					/*if(r_newrefdef.rdflags & RDF_IRGOGGLES && RI.currententity->flags & RF_IR_VISIBLE)
						*lpdest = ((byte *)vid.colormap)[irtable[*lptex]];
					else*/
                    *lpdest =
                        *(vid.colormap.as_mut_ptr() as
                              *mut byte).offset((*lptex as libc::c_int +
                                                     (llight &
                                                          0xff00 as
                                                              libc::c_int)) as
                                                    isize) as pixel_t;
                    //PGM
                    *lpz = (lzi >> 16 as libc::c_int) as libc::c_short
                }
                lpdest = lpdest.offset(1);
                lzi += r_zistepx;
                lpz = lpz.offset(1);
                llight += r_lstepx;
                lptex = lptex.offset(a_ststepxwhole as isize);
                lsfrac += a_sstepxfrac;
                lptex = lptex.offset((lsfrac >> 16 as libc::c_int) as isize);
                lsfrac &= 0xffff as libc::c_int;
                ltfrac += a_tstepxfrac;
                if ltfrac & 0x10000 as libc::c_int != 0 {
                    lptex = lptex.offset(r_affinetridesc.skinwidth as isize);
                    ltfrac &= 0xffff as libc::c_int
                }
                lcount -= 1;
                if !(lcount != 0) { break ; }
            }
        }
        pspanpackage = pspanpackage.offset(1);
        if !((*pspanpackage).count != -(999999 as libc::c_int)) { break ; }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_PolysetFillSpans8(mut pspanpackage:
                                                 *mut spanpackage_t) {
    //int				color;
    let mut lcount: libc::c_int = 0;
    loop 
         // FIXME: do z buffering
         //color = d_aflatcolor++ * 10;
         {
        lcount = d_aspancount - (*pspanpackage).count;
        // d_ptex + a_ststepxwhole * lcount  + ((a_sstepxfrac * lcount) >> 16) + ((a_tstepxfrac * lcount) >> 16)*r_affinetridesc.skinwidth;
        errorterm += erroradjustup;
        if errorterm >= 0 as libc::c_int {
            d_aspancount += d_countextrastep;
            errorterm -= erroradjustdown
        } else { d_aspancount += ubasestep }
        if lcount != 0 {
            let mut lsfrac: libc::c_int = 0;
            let mut ltfrac: libc::c_int = 0;
            let mut lpdest: *mut pixel_t = 0 as *mut pixel_t;
            let mut lptex: *mut pixel_t = 0 as *mut pixel_t;
            let mut llight: libc::c_int = 0;
            let mut lzi: libc::c_int = 0;
            let mut lpz: *mut libc::c_short = 0 as *mut libc::c_short;
            lpdest = (*pspanpackage).pdest as *mut pixel_t;
            lptex = (*pspanpackage).ptex;
            lpz = (*pspanpackage).pz;
            lsfrac = (*pspanpackage).sfrac;
            ltfrac = (*pspanpackage).tfrac;
            llight = (*pspanpackage).light;
            lzi = (*pspanpackage).zi;
            pspanpackage = pspanpackage.offset(1);
            if !(R_PolysetCheckBounds(lptex, lsfrac, ltfrac, lcount) as u64 ==
                     0) {
                loop  {
                    if lzi >> 16 as libc::c_int >= *lpz as libc::c_int {
                        //PGM
					/*if(r_newrefdef.rdflags & RDF_IRGOGGLES && RI.currententity->flags & RF_IR_VISIBLE)
						*lpdest = ((byte *)vid.colormap)[irtable[*lptex]];
					else*/
					//*lpdest = *lptex; //((byte *)vid.colormap)[*lptex + (llight & 0xFF00)];
                        // check for texture bounds to make asan happy
                        let mut src: pixel_t = *lptex;
                        //*lpdest = //vid.colormap[src & 0xff00|(llight>>8)] << 8 | (src & llight & 0xff) | ((src & 0xff) >> 3);
					// very dirty, maybe need dual colormap?
					//*lpdest = (vid.colormap[src >> 8 | (llight & 0xFF00)] << 8) | src & 0xff;
					// 13 bit lighting, 32 light levels
                        *lpdest =
                            (vid.colormap[(src as libc::c_int >>
                                               3 as libc::c_int |
                                               (llight &
                                                    0x1f00 as libc::c_int) <<
                                                   5 as libc::c_int) as usize]
                                 as libc::c_int |
                                 src as libc::c_int & 7 as libc::c_int) as
                                pixel_t;
                        //PGM
                        *lpz = (lzi >> 16 as libc::c_int) as libc::c_short
                    }
                    lpdest = lpdest.offset(1);
                    lzi += r_zistepx;
                    lpz = lpz.offset(1);
                    llight += r_lstepx;
                    lptex = lptex.offset(a_ststepxwhole as isize);
                    lsfrac += a_sstepxfrac;
                    lptex =
                        lptex.offset((lsfrac >> 16 as libc::c_int) as isize);
                    lsfrac &= 0xffff as libc::c_int;
                    ltfrac += a_tstepxfrac;
                    if ltfrac & 0x10000 as libc::c_int != 0 {
                        lptex =
                            lptex.offset(r_affinetridesc.skinwidth as isize);
                        ltfrac &= 0xffff as libc::c_int
                    }
                    lcount -= 1;
                    if !(lcount != 0) { break ; }
                }
            }
        } else { pspanpackage = pspanpackage.offset(1) }
        if !((*pspanpackage).count != -(999999 as libc::c_int)) { break ; }
    };
}
/*
================
R_RasterizeAliasPolySmooth
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_RasterizeAliasPolySmooth() {
    let mut initialleftheight: libc::c_int = 0;
    let mut initialrightheight: libc::c_int = 0;
    let mut plefttop: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut prighttop: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut pleftbottom: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut prightbottom: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut working_lstepx: libc::c_int = 0;
    let mut originalcount: libc::c_int = 0;
    plefttop = (*pedgetable).pleftedgevert0;
    prighttop = (*pedgetable).prightedgevert0;
    pleftbottom = (*pedgetable).pleftedgevert1;
    prightbottom = (*pedgetable).prightedgevert1;
    initialleftheight =
        *pleftbottom.offset(1 as libc::c_int as isize) -
            *plefttop.offset(1 as libc::c_int as isize);
    initialrightheight =
        *prightbottom.offset(1 as libc::c_int as isize) -
            *prighttop.offset(1 as libc::c_int as isize);
    //
// set the s, t, and light gradients, which are consistent across the triangle
// because being a triangle, things are affine
//
    if R_PolysetCalcGradients(r_affinetridesc.skinwidth) as u64 == 0 {
        return
    }
    //
// rasterize the polygon
//
    //
// scan out the top (and possibly only) part of the left edge
//
    d_pedgespanpackage = a_spans;
    ystart = *plefttop.offset(1 as libc::c_int as isize);
    d_aspancount =
        *plefttop.offset(0 as libc::c_int as isize) -
            *prighttop.offset(0 as libc::c_int as isize);
    d_ptex =
        (r_affinetridesc.pskin as
             *mut pixel_t).offset((*plefttop.offset(2 as libc::c_int as isize)
                                       >> 16 as libc::c_int) as
                                      isize).offset(((*plefttop.offset(3 as
                                                                           libc::c_int
                                                                           as
                                                                           isize)
                                                          >>
                                                          16 as libc::c_int) *
                                                         r_affinetridesc.skinwidth)
                                                        as isize);
    //#if	id386ALIAS
    d_sfrac =
        *plefttop.offset(2 as libc::c_int as isize) & 0xffff as libc::c_int;
    d_tfrac =
        *plefttop.offset(3 as libc::c_int as isize) & 0xffff as libc::c_int;
    //#endif
    d_light = *plefttop.offset(4 as libc::c_int as isize);
    d_zi = *plefttop.offset(5 as libc::c_int as isize);
    d_pdest =
        d_viewbuffer.offset((ystart * r_screenwidth) as
                                isize).offset(*plefttop.offset(0 as
                                                                   libc::c_int
                                                                   as isize)
                                                  as isize);
    d_pz =
        d_pzbuffer.offset((ystart as libc::c_uint).wrapping_mul(d_zwidth) as
                              isize).offset(*plefttop.offset(0 as libc::c_int
                                                                 as isize) as
                                                isize);
    if initialleftheight == 1 as libc::c_int {
        (*d_pedgespanpackage).pdest = d_pdest as *mut libc::c_void;
        (*d_pedgespanpackage).pz = d_pz;
        (*d_pedgespanpackage).count = d_aspancount;
        (*d_pedgespanpackage).ptex = d_ptex;
        (*d_pedgespanpackage).sfrac = d_sfrac;
        (*d_pedgespanpackage).tfrac = d_tfrac;
        // FIXME: need to clamp l, s, t, at both ends?
        (*d_pedgespanpackage).light = d_light;
        (*d_pedgespanpackage).zi = d_zi;
        d_pedgespanpackage = d_pedgespanpackage.offset(1)
    } else {
        R_PolysetSetUpForLineScan(*plefttop.offset(0 as libc::c_int as isize),
                                  *plefttop.offset(1 as libc::c_int as isize),
                                  *pleftbottom.offset(0 as libc::c_int as
                                                          isize),
                                  *pleftbottom.offset(1 as libc::c_int as
                                                          isize));
        //#if	id386ALIAS
        d_pzbasestep =
            d_zwidth.wrapping_add(ubasestep as libc::c_uint) as libc::c_int;
        d_pzextrastep = d_pzbasestep + 1 as libc::c_int;
        //#endif
        d_pdestbasestep = r_screenwidth + ubasestep;
        d_pdestextrastep = d_pdestbasestep + 1 as libc::c_int;
        // TODO: can reuse partial expressions here
        // for negative steps in x along left edge, bias toward overflow rather than
	// underflow (sort of turning the floor () we did in the gradient calcs into
	// ceil (), but plus a little bit)
        if ubasestep < 0 as libc::c_int {
            working_lstepx = r_lstepx - 1 as libc::c_int
        } else { working_lstepx = r_lstepx }
        d_countextrastep = ubasestep + 1 as libc::c_int;
        d_ptexbasestep =
            (r_sstepy + r_sstepx * ubasestep >> 16 as libc::c_int) +
                (r_tstepy + r_tstepx * ubasestep >> 16 as libc::c_int) *
                    r_affinetridesc.skinwidth;
        //#if	id386ALIAS
        //#else
        d_sfracbasestep =
            r_sstepy + r_sstepx * ubasestep & 0xffff as libc::c_int;
        d_tfracbasestep =
            r_tstepy + r_tstepx * ubasestep & 0xffff as libc::c_int;
        d_lightbasestep = r_lstepy + working_lstepx * ubasestep;
        d_zibasestep = r_zistepy + r_zistepx * ubasestep;
        d_ptexextrastep =
            (r_sstepy + r_sstepx * d_countextrastep >> 16 as libc::c_int) +
                (r_tstepy + r_tstepx * d_countextrastep >> 16 as libc::c_int)
                    * r_affinetridesc.skinwidth;
        //#endif
        //#if	id386ALIAS
        //#else
        d_sfracextrastep =
            r_sstepy + r_sstepx * d_countextrastep & 0xffff as libc::c_int;
        d_tfracextrastep =
            r_tstepy + r_tstepx * d_countextrastep & 0xffff as libc::c_int;
        d_lightextrastep = d_lightbasestep + working_lstepx;
        d_ziextrastep = d_zibasestep + r_zistepx;
        if R_PolysetScanLeftEdge_C(initialleftheight) as u64 == 0 { return }
    }
    //#endif
    //
// scan out the bottom part of the left edge, if it exists
//
    if (*pedgetable).numleftedges == 2 as libc::c_int {
        let mut height: libc::c_int = 0;
        plefttop = pleftbottom;
        pleftbottom = (*pedgetable).pleftedgevert2;
        height =
            *pleftbottom.offset(1 as libc::c_int as isize) -
                *plefttop.offset(1 as libc::c_int as isize);
        // TODO: make this a function; modularize this function in general
        ystart = *plefttop.offset(1 as libc::c_int as isize);
        d_aspancount =
            *plefttop.offset(0 as libc::c_int as isize) -
                *prighttop.offset(0 as libc::c_int as isize);
        d_ptex =
            (r_affinetridesc.pskin as
                 *mut pixel_t).offset((*plefttop.offset(2 as libc::c_int as
                                                            isize) >>
                                           16 as libc::c_int) as
                                          isize).offset(((*plefttop.offset(3
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize)
                                                              >>
                                                              16 as
                                                                  libc::c_int)
                                                             *
                                                             r_affinetridesc.skinwidth)
                                                            as isize);
        d_sfrac = 0 as libc::c_int;
        d_tfrac = 0 as libc::c_int;
        d_light = *plefttop.offset(4 as libc::c_int as isize);
        d_zi = *plefttop.offset(5 as libc::c_int as isize);
        d_pdest =
            d_viewbuffer.offset((ystart * r_screenwidth) as
                                    isize).offset(*plefttop.offset(0 as
                                                                       libc::c_int
                                                                       as
                                                                       isize)
                                                      as isize);
        d_pz =
            d_pzbuffer.offset((ystart as libc::c_uint).wrapping_mul(d_zwidth)
                                  as
                                  isize).offset(*plefttop.offset(0 as
                                                                     libc::c_int
                                                                     as isize)
                                                    as isize);
        if height == 1 as libc::c_int {
            (*d_pedgespanpackage).pdest = d_pdest as *mut libc::c_void;
            (*d_pedgespanpackage).pz = d_pz;
            (*d_pedgespanpackage).count = d_aspancount;
            (*d_pedgespanpackage).ptex = d_ptex;
            (*d_pedgespanpackage).sfrac = d_sfrac;
            (*d_pedgespanpackage).tfrac = d_tfrac;
            // FIXME: need to clamp l, s, t, at both ends?
            (*d_pedgespanpackage).light = d_light;
            (*d_pedgespanpackage).zi = d_zi;
            d_pedgespanpackage = d_pedgespanpackage.offset(1)
        } else {
            R_PolysetSetUpForLineScan(*plefttop.offset(0 as libc::c_int as
                                                           isize),
                                      *plefttop.offset(1 as libc::c_int as
                                                           isize),
                                      *pleftbottom.offset(0 as libc::c_int as
                                                              isize),
                                      *pleftbottom.offset(1 as libc::c_int as
                                                              isize));
            d_pdestbasestep = r_screenwidth + ubasestep;
            d_pdestextrastep = d_pdestbasestep + 1 as libc::c_int;
            //#if	id386ALIAS
            d_pzbasestep =
                d_zwidth.wrapping_add(ubasestep as libc::c_uint) as
                    libc::c_int;
            d_pzextrastep = d_pzbasestep + 1 as libc::c_int;
            //#endif
            if ubasestep < 0 as libc::c_int {
                working_lstepx = r_lstepx - 1 as libc::c_int
            } else { working_lstepx = r_lstepx }
            d_countextrastep = ubasestep + 1 as libc::c_int;
            d_ptexbasestep =
                (r_sstepy + r_sstepx * ubasestep >> 16 as libc::c_int) +
                    (r_tstepy + r_tstepx * ubasestep >> 16 as libc::c_int) *
                        r_affinetridesc.skinwidth;
            //#if	id386ALIAS
            d_sfracbasestep =
                r_sstepy + r_sstepx * ubasestep & 0xffff as libc::c_int;
            d_tfracbasestep =
                r_tstepy + r_tstepx * ubasestep & 0xffff as libc::c_int;
            //#endif
            d_lightbasestep = r_lstepy + working_lstepx * ubasestep;
            d_zibasestep = r_zistepy + r_zistepx * ubasestep;
            d_ptexextrastep =
                (r_sstepy + r_sstepx * d_countextrastep >> 16 as libc::c_int)
                    +
                    (r_tstepy + r_tstepx * d_countextrastep >>
                         16 as libc::c_int) * r_affinetridesc.skinwidth;
            //#if	id386ALIAS
            //#endif
            d_sfracextrastep =
                r_sstepy + r_sstepx * d_countextrastep &
                    0xffff as libc::c_int;
            d_tfracextrastep =
                r_tstepy + r_tstepx * d_countextrastep &
                    0xffff as libc::c_int;
            //#endif
            d_lightextrastep = d_lightbasestep + working_lstepx;
            d_ziextrastep = d_zibasestep + r_zistepx;
            if R_PolysetScanLeftEdge_C(height) as u64 == 0 { return }
        }
    }
    // scan out the top (and possibly only) part of the right edge, updating the
// count field
    d_pedgespanpackage = a_spans; // mark end of the spanpackages
    R_PolysetSetUpForLineScan(*prighttop.offset(0 as libc::c_int as isize),
                              *prighttop.offset(1 as libc::c_int as isize),
                              *prightbottom.offset(0 as libc::c_int as isize),
                              *prightbottom.offset(1 as libc::c_int as
                                                       isize));
    d_aspancount = 0 as libc::c_int;
    d_countextrastep = ubasestep + 1 as libc::c_int;
    originalcount = (*a_spans.offset(initialrightheight as isize)).count;
    (*a_spans.offset(initialrightheight as isize)).count =
        -(999999 as libc::c_int);
    Some(d_pdrawspans.expect("non-null function pointer")).expect("non-null function pointer")(a_spans);
    // scan out the bottom part of the right edge, if it exists
    if (*pedgetable).numrightedges == 2 as libc::c_int {
        let mut height_0: libc::c_int = 0;
        let mut pstart: *mut spanpackage_t = 0 as *mut spanpackage_t;
        pstart = a_spans.offset(initialrightheight as isize);
        (*pstart).count = originalcount;
        d_aspancount =
            *prightbottom.offset(0 as libc::c_int as isize) -
                *prighttop.offset(0 as libc::c_int as isize);
        prighttop = prightbottom;
        prightbottom = (*pedgetable).prightedgevert2;
        height_0 =
            *prightbottom.offset(1 as libc::c_int as isize) -
                *prighttop.offset(1 as libc::c_int as isize);
        R_PolysetSetUpForLineScan(*prighttop.offset(0 as libc::c_int as
                                                        isize),
                                  *prighttop.offset(1 as libc::c_int as
                                                        isize),
                                  *prightbottom.offset(0 as libc::c_int as
                                                           isize),
                                  *prightbottom.offset(1 as libc::c_int as
                                                           isize));
        d_countextrastep = ubasestep + 1 as libc::c_int;
        (*a_spans.offset((initialrightheight + height_0) as isize)).count =
            -(999999 as libc::c_int);
        // mark end of the spanpackages
        Some(d_pdrawspans.expect("non-null function pointer")).expect("non-null function pointer")(pstart);
    };
}
/*
================
R_PolysetSetEdgeTable
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_PolysetSetEdgeTable() {
    let mut edgetableindex: libc::c_int =
        0; // assume the vertices are already in
    edgetableindex = 0 as libc::c_int;
    //  top to bottom order
    //
// determine which edges are right & left, and the order in which
// to rasterize them
//
    if r_p0[1 as libc::c_int as usize] >= r_p1[1 as libc::c_int as usize] {
        if r_p0[1 as libc::c_int as usize] == r_p1[1 as libc::c_int as usize]
           {
            if r_p0[1 as libc::c_int as usize] <
                   r_p2[1 as libc::c_int as usize] {
                pedgetable =
                    &mut *edgetables.as_mut_ptr().offset(2 as libc::c_int as
                                                             isize) as
                        *mut edgetable
            } else {
                pedgetable =
                    &mut *edgetables.as_mut_ptr().offset(5 as libc::c_int as
                                                             isize) as
                        *mut edgetable
            }
            return
        } else { edgetableindex = 1 as libc::c_int }
    }
    if r_p0[1 as libc::c_int as usize] == r_p2[1 as libc::c_int as usize] {
        if edgetableindex != 0 {
            pedgetable =
                &mut *edgetables.as_mut_ptr().offset(8 as libc::c_int as
                                                         isize) as
                    *mut edgetable
        } else {
            pedgetable =
                &mut *edgetables.as_mut_ptr().offset(9 as libc::c_int as
                                                         isize) as
                    *mut edgetable
        }
        return
    } else {
        if r_p1[1 as libc::c_int as usize] == r_p2[1 as libc::c_int as usize]
           {
            if edgetableindex != 0 {
                pedgetable =
                    &mut *edgetables.as_mut_ptr().offset(10 as libc::c_int as
                                                             isize) as
                        *mut edgetable
            } else {
                pedgetable =
                    &mut *edgetables.as_mut_ptr().offset(11 as libc::c_int as
                                                             isize) as
                        *mut edgetable
            }
            return
        }
    }
    if r_p0[1 as libc::c_int as usize] > r_p2[1 as libc::c_int as usize] {
        edgetableindex += 2 as libc::c_int
    }
    if r_p1[1 as libc::c_int as usize] > r_p2[1 as libc::c_int as usize] {
        edgetableindex += 4 as libc::c_int
    }
    pedgetable =
        &mut *edgetables.as_mut_ptr().offset(edgetableindex as isize) as
            *mut edgetable;
}
