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
    fn fabsf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn Matrix4x4_VectorITransform(in_0: *const [vec_t; 4],
                                  v: *const libc::c_float,
                                  out: *mut libc::c_float);
    #[no_mangle]
    static mut r_temppool: poolhandle_t;
    #[no_mangle]
    static mut vid: viddef_t;
    #[no_mangle]
    static mut RI: ref_instance_t;
    #[no_mangle]
    static mut tr: gl_globals_t;
    #[no_mangle]
    fn R_GetTexture(texnum: libc::c_uint) -> *mut image_t;
    #[no_mangle]
    static mut gEngfuncs: ref_api_t;
    #[no_mangle]
    static mut gpGlobals: *mut ref_globals_t;
    #[no_mangle]
    static mut d_roverwrapped: qboolean;
    #[no_mangle]
    static mut d_initial_rover: *mut surfcache_t;
    #[no_mangle]
    fn Q_pretifymem(value: libc::c_float, digitsafterdecimal: libc::c_int)
     -> *mut libc::c_char;
    #[no_mangle]
    static mut sw_surfcacheoverride: *mut cvar_t;
    //void R_BuildLightMap (void);
    #[no_mangle]
    static mut blocklights: [libc::c_uint; 10240];
    //=============================================================================
    #[no_mangle]
    fn R_DecalComputeBasis(surf: *mut msurface_t, flags: libc::c_int,
                           textureSpaceBasis: *mut vec3_t);
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
pub type fixed8_t = libc::c_int;
pub type imagetype_t = libc::c_uint;
pub const it_sky: imagetype_t = 4;
pub const it_pic: imagetype_t = 3;
pub const it_wall: imagetype_t = 2;
pub const it_sprite: imagetype_t = 1;
pub const it_skin: imagetype_t = 0;
pub type pixel_t = libc::c_ushort;
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
pub struct image_s {
    pub name: [libc::c_char; 256],
    pub srcWidth: word,
    pub srcHeight: word,
    pub width: word,
    pub height: word,
    pub depth: word,
    pub numMips: byte,
    pub flags: texFlags_t,
    pub fogParams: rgba_t,
    pub original: *mut rgbdata_t,
    pub size: size_t,
    pub xscale: libc::c_float,
    pub yscale: libc::c_float,
    pub type_0: imagetype_t,
    pub pixels: [*mut pixel_t; 4],
    pub alpha_pixels: *mut pixel_t,
    pub servercount: libc::c_int,
    pub hashValue: uint,
    pub nextHash: *mut image_s,
}
pub type image_t = image_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct drawsurf_t {
    pub surfdat: *mut pixel_t,
    pub rowbytes: libc::c_int,
    pub surf: *mut msurface_t,
    pub lightadj: [fixed8_t; 4],
    pub image: *mut image_t,
    pub surfmip: libc::c_int,
    pub surfwidth: libc::c_int,
    pub surfheight: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct surfcache_s {
    pub next: *mut surfcache_s,
    pub owner: *mut *mut surfcache_s,
    pub lightadj: [libc::c_int; 4],
    pub dlight: libc::c_int,
    pub size: libc::c_int,
    pub width: libc::c_uint,
    pub height: libc::c_uint,
    pub mipscale: libc::c_float,
    pub image: *mut image_t,
    pub data: [byte; 4],
}
pub type surfcache_t = surfcache_s;
#[inline(always)]
unsafe extern "C" fn __tg_fabs(mut __x: libc::c_float) -> libc::c_float {
    return fabsf(__x);
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
// r_surf.c: surface-related refresh code
#[no_mangle]
pub static mut r_drawsurf: drawsurf_t =
    drawsurf_t{surfdat: 0 as *const pixel_t as *mut pixel_t,
               rowbytes: 0,
               surf: 0 as *const msurface_t as *mut msurface_t,
               lightadj: [0; 4],
               image: 0 as *const image_t as *mut image_t,
               surfmip: 0,
               surfwidth: 0,
               surfheight: 0,};
#[no_mangle]
pub static mut lightleft: uint = 0;
#[no_mangle]
pub static mut sourcesstep: uint = 0;
#[no_mangle]
pub static mut blocksize: uint = 0;
#[no_mangle]
pub static mut sourcetstep: uint = 0;
#[no_mangle]
pub static mut lightdelta: uint = 0;
#[no_mangle]
pub static mut lightdeltastep: uint = 0;
#[no_mangle]
pub static mut lightright: uint = 0;
#[no_mangle]
pub static mut lightleftstep: uint = 0;
#[no_mangle]
pub static mut lightrightstep: uint = 0;
#[no_mangle]
pub static mut blockdivshift: uint = 0;
#[no_mangle]
pub static mut blockdivmask: libc::c_uint = 0;
#[no_mangle]
pub static mut prowdestbase: *mut libc::c_void =
    0 as *const libc::c_void as *mut libc::c_void;
#[no_mangle]
pub static mut pbasesource: *mut pixel_t =
    0 as *const pixel_t as *mut pixel_t;
#[no_mangle]
pub static mut surfrowbytes: libc::c_int = 0;
// used by ASM files
#[no_mangle]
pub static mut r_lightptr: *mut libc::c_uint =
    0 as *const libc::c_uint as *mut libc::c_uint;
#[no_mangle]
pub static mut r_stepback: libc::c_int = 0;
#[no_mangle]
pub static mut r_lightwidth: libc::c_int = 0;
#[no_mangle]
pub static mut r_numhblocks: libc::c_int = 0;
#[no_mangle]
pub static mut r_numvblocks: libc::c_int = 0;
#[no_mangle]
pub static mut r_source: *mut pixel_t = 0 as *const pixel_t as *mut pixel_t;
#[no_mangle]
pub static mut r_sourcemax: *mut pixel_t =
    0 as *const pixel_t as *mut pixel_t;
static mut worldlux_s: libc::c_float = 0.;
static mut worldlux_t: libc::c_float = 0.;
static mut surfmiptable: [Option<unsafe extern "C" fn() -> ()>; 4] =
    unsafe {
        [Some(R_DrawSurfaceBlock8_mip0 as unsafe extern "C" fn() -> ()),
         Some(R_DrawSurfaceBlock8_mip1 as unsafe extern "C" fn() -> ()),
         Some(R_DrawSurfaceBlock8_mip2 as unsafe extern "C" fn() -> ()),
         Some(R_DrawSurfaceBlock8_mip3 as unsafe extern "C" fn() -> ())]
    };
// allow some very large lightmaps
#[no_mangle]
pub static mut surfscale: libc::c_float = 0.;
#[no_mangle]
pub static mut r_cache_thrash: qboolean = false_0;
// set if surface cache is thrashing
#[no_mangle]
pub static mut sc_size: libc::c_int = 0;
#[no_mangle]
pub static mut sc_rover: *mut surfcache_t =
    0 as *const surfcache_t as *mut surfcache_t;
#[no_mangle]
pub static mut sc_base: *mut surfcache_t =
    0 as *const surfcache_t as *mut surfcache_t;
static mut rtable: [[libc::c_int; 20]; 20] = [[0; 20]; 20];
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
	//if( !R_CountSurfaceDlights( surf )) return;
    sample_size =
        gEngfuncs.Mod_SampleSizeForFace.expect("non-null function pointer")(surf)
            as libc::c_float; // not lit by this light
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
                bl = blocklights.as_mut_ptr();
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
                            //printf("dlight %f\n", dist);
					//*(void**)0 = 0;
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
                                                                     gEngfuncs.LightToTexGamma.expect("non-null function pointer")((((*dl).color.r
                                                                                                                                         as
                                                                                                                                         libc::c_int
                                                                                                                                         +
                                                                                                                                         (*dl).color.g
                                                                                                                                             as
                                                                                                                                             libc::c_int
                                                                                                                                         +
                                                                                                                                         (*dl).color.b
                                                                                                                                             as
                                                                                                                                             libc::c_int)
                                                                                                                                        /
                                                                                                                                        3
                                                                                                                                            as
                                                                                                                                            libc::c_int)
                                                                                                                                       as
                                                                                                                                       byte)
                                                                         as
                                                                         libc::c_int
                                                                     *
                                                                     3 as
                                                                         libc::c_int
                                                                     /
                                                                     256 as
                                                                         libc::c_int)
                                                                    as
                                                                    libc::c_uint)
                                    as uint as uint
                            //bl[1] += ((int)((rad - dist) * 256) * 2.5) / 256;
					//bl[2] += ((int)((rad - dist) * 256) * 2.5) / 256;
                        }
                        s += 1;
                        sacc += sample_size;
                        bl = bl.offset(1 as libc::c_int as isize)
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
=================
R_BuildLightmap

Combine and scale multiple lightmaps into the floating
format in r_blocklights
=================
*/
unsafe extern "C" fn R_BuildLightMap() {
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
    let mut surf: *mut msurface_t = r_drawsurf.surf;
    let mut info: *mut mextrasurf_t = (*surf).info;
    let mut lm: *mut color24 = 0 as *mut color24;
    let mut dynamic: qboolean = false_0;
    sample_size =
        gEngfuncs.Mod_SampleSizeForFace.expect("non-null function pointer")(surf);
    smax =
        (*info).lightextents[0 as libc::c_int as usize] as libc::c_int /
            sample_size + 1 as libc::c_int;
    tmax =
        (*info).lightextents[1 as libc::c_int as usize] as libc::c_int /
            sample_size + 1 as libc::c_int;
    //smax = (surf->extents[0]>>4)+1;
	//tmax = (surf->extents[1]>>4)+1;
    size = smax * tmax;
    if (*surf).flags as libc::c_uint & (1 as libc::c_uint) << 6 as libc::c_int
           != 0 {
        smax =
            (*info).lightextents[0 as libc::c_int as usize] as libc::c_int *
                3 as libc::c_int / sample_size + 1 as libc::c_int;
        size = smax * tmax;
        memset(blocklights.as_mut_ptr() as *mut libc::c_void,
               0xff as libc::c_int,
               (::std::mem::size_of::<uint>() as
                    libc::c_ulong).wrapping_mul(size as libc::c_ulong));
        return
    }
    lm = (*surf).samples;
    memset(blocklights.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<uint>() as
                libc::c_ulong).wrapping_mul(size as libc::c_ulong));
    // add all the lightmaps
    map = 0 as libc::c_int;
    while map < 4 as libc::c_int &&
              (*surf).styles[map as usize] as libc::c_int !=
                  255 as libc::c_int {
        scale =
            tr.lightstylevalue[(*surf).styles[map as usize] as usize] as uint;
        i = 0 as libc::c_int;
        bl = blocklights.as_mut_ptr();
        while i < size {
            let ref mut fresh1 = *bl.offset(0 as libc::c_int as isize);
            *fresh1 =
                (*fresh1 as
                     libc::c_uint).wrapping_add((gEngfuncs.LightToTexGamma.expect("non-null function pointer")((*lm).r)
                                                     as
                                                     libc::c_uint).wrapping_mul(scale))
                    as uint as uint;
            let ref mut fresh2 = *bl.offset(0 as libc::c_int as isize);
            *fresh2 =
                (*fresh2 as
                     libc::c_uint).wrapping_add((gEngfuncs.LightToTexGamma.expect("non-null function pointer")((*lm).g)
                                                     as
                                                     libc::c_uint).wrapping_mul(scale))
                    as uint as uint;
            let ref mut fresh3 = *bl.offset(0 as libc::c_int as isize);
            *fresh3 =
                (*fresh3 as
                     libc::c_uint).wrapping_add((gEngfuncs.LightToTexGamma.expect("non-null function pointer")((*lm).b)
                                                     as
                                                     libc::c_uint).wrapping_mul(scale))
                    as uint as uint;
            i += 1;
            bl = bl.offset(1 as libc::c_int as isize);
            lm = lm.offset(1)
            //printf("test\n");
			//bl[1] += gEngfuncs.LightToTexGamma( lm->g ) * scale;
			//bl[2] += gEngfuncs.LightToTexGamma( lm->b ) * scale;
        }
        map += 1
    }
    // add all the dynamic lights
    if (*surf).dlightframe == tr.framecount { R_AddDynamicLights(surf); }
    // Put into texture format
	//stride -= (smax << 2);
	//bl = blocklights;
    /*for( t = 0; t < tmax; t++, dest += stride )
	{
		for( s = 0; s < smax; s++ )
		{
			dest[0] = Q_min((bl[0] >> 7), 255 );
			//dest[1] = Q_min((bl[1] >> 7), 255 );
			//dest[2] = Q_min((bl[2] >> 7), 255 );
			//dest[3] = 255;

			bl += 3;
			dest += 4;
		}
	}*/
	// bound, invert, and shift
    i = 0 as libc::c_int; //(255*256 - t) >> (8 - VID_CBITS);
    while i < size {
        t = blocklights[i as usize] as libc::c_int;
        if t < 0 as libc::c_int { t = 0 as libc::c_int }
        if t > 65535 as libc::c_int * 3 as libc::c_int {
            t = 65535 as libc::c_int * 3 as libc::c_int
        }
        t = t / 2048 as libc::c_int / 3 as libc::c_int;
        //if (t < (1 << 6))
				//t = (1 << 6);
        t = t << 8 as libc::c_int;
        blocklights[i as usize] = t as libc::c_uint;
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_InitRandomTable() {
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
        if (*R_GetTexture((*base).gl_texturenum as libc::c_uint)).flags as
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
        if (*R_GetTexture((*base).gl_texturenum as libc::c_uint)).flags as
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
R_DrawSurface
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_DrawSurface() {
    let mut basetptr: *mut pixel_t = 0 as *mut pixel_t;
    let mut smax: libc::c_int = 0;
    let mut tmax: libc::c_int = 0;
    let mut twidth: libc::c_int = 0;
    let mut u: libc::c_int = 0;
    let mut soffset: libc::c_int = 0;
    let mut basetoffset: libc::c_int = 0;
    let mut texwidth: libc::c_int = 0;
    let mut horzblockstep: libc::c_int = 0;
    let mut pcolumndest: *mut pixel_t = 0 as *mut pixel_t;
    let mut pblockdrawer: Option<unsafe extern "C" fn() -> ()> = None;
    let mut mt: *mut image_t = 0 as *mut image_t;
    let mut sample_size: uint = 0;
    let mut sample_bits: uint = 0;
    let mut sample_pot: uint = 0;
    surfrowbytes = r_drawsurf.rowbytes;
    sample_size =
        if tr.sample_size == -(1 as libc::c_int) {
            gEngfuncs.Mod_SampleSizeForFace.expect("non-null function pointer")(r_drawsurf.surf)
        } else { tr.sample_size } as uint;
    if sample_size == 16 as libc::c_int as libc::c_uint {
        sample_bits = 4 as libc::c_int as uint;
        sample_pot = sample_size
    } else {
        sample_bits = tr.sample_bits;
        if sample_bits == -(1 as libc::c_int) as libc::c_uint {
            sample_bits = 0 as libc::c_int as uint;
            sample_pot = 1 as libc::c_int as uint;
            while sample_pot < sample_size {
                sample_pot <<= 1 as libc::c_int;
                sample_bits = sample_bits.wrapping_add(1)
            }
        } else { sample_pot = ((1 as libc::c_int) << sample_bits) as uint }
    }
    mt = r_drawsurf.image;
    r_source = (*mt).pixels[r_drawsurf.surfmip as usize];
    // the fractional light values should range from 0 to (VID_GRADES - 1) << 16
// from a source range of 0 - 255
    texwidth = (*mt).width as libc::c_int >> r_drawsurf.surfmip;
    blocksize = sample_pot >> r_drawsurf.surfmip;
    blockdivshift =
        sample_bits.wrapping_sub(r_drawsurf.surfmip as libc::c_uint);
    blockdivmask =
        (((1 as libc::c_int) << blockdivshift) - 1 as libc::c_int) as
            libc::c_uint;
    if sample_size == 16 as libc::c_int as libc::c_uint {
        r_lightwidth =
            ((*(*r_drawsurf.surf).info).lightextents[0 as libc::c_int as
                                                         usize] as libc::c_int
                 >> 4 as libc::c_int) + 1 as libc::c_int
    } else {
        r_lightwidth =
            ((*(*r_drawsurf.surf).info).lightextents[0 as libc::c_int as
                                                         usize] as
                 libc::c_uint).wrapping_div(sample_size).wrapping_add(1 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint)
                as libc::c_int
    }
    r_numhblocks = r_drawsurf.surfwidth >> blockdivshift;
    r_numvblocks = r_drawsurf.surfheight >> blockdivshift;
    //==============================
    if sample_size == 16 as libc::c_int as libc::c_uint {
        pblockdrawer = surfmiptable[r_drawsurf.surfmip as usize]
    } else {
        pblockdrawer =
            Some(R_DrawSurfaceBlock8_Generic as unsafe extern "C" fn() -> ())
    }
    // TODO: only needs to be set when there is a display settings change
    horzblockstep = blocksize as libc::c_int;
    smax = (*mt).width as libc::c_int >> r_drawsurf.surfmip;
    twidth = texwidth;
    tmax = (*mt).height as libc::c_int >> r_drawsurf.surfmip;
    sourcetstep = texwidth as uint;
    r_stepback = tmax * twidth;
    r_sourcemax = r_source.offset((tmax * smax) as isize);
    // glitchy and slow way to draw some lightmap
    if (*(*r_drawsurf.surf).texinfo).flags as libc::c_uint &
           (1 as libc::c_uint) << 1 as libc::c_int != 0 {
        worldlux_s =
            ((*r_drawsurf.surf).extents[0 as libc::c_int as usize] as
                 libc::c_int /
                 (*(*r_drawsurf.surf).info).lightextents[0 as libc::c_int as
                                                             usize] as
                     libc::c_int) as libc::c_float;
        worldlux_t =
            ((*r_drawsurf.surf).extents[1 as libc::c_int as usize] as
                 libc::c_int /
                 (*(*r_drawsurf.surf).info).lightextents[1 as libc::c_int as
                                                             usize] as
                     libc::c_int) as libc::c_float;
        if worldlux_s == 0 as libc::c_int as libc::c_float {
            worldlux_s = 1 as libc::c_int as libc::c_float
        }
        if worldlux_t == 0 as libc::c_int as libc::c_float {
            worldlux_t = 1 as libc::c_int as libc::c_float
        }
        soffset =
            (*r_drawsurf.surf).texturemins[0 as libc::c_int as usize] as
                libc::c_int;
        basetoffset =
            (*r_drawsurf.surf).texturemins[1 as libc::c_int as usize] as
                libc::c_int;
        //soffset =  r_drawsurf.surf->info->lightmapmins[0] * worldlux_s;
		//basetoffset = r_drawsurf.surf->info->lightmapmins[1] * worldlux_t;
		// << 16 components are to guarantee positive values for %
        soffset =
            ((soffset >> r_drawsurf.surfmip) + (smax << 16 as libc::c_int)) %
                smax;
        basetptr =
            &mut *r_source.offset((((basetoffset >> r_drawsurf.surfmip) +
                                        (tmax << 16 as libc::c_int)) % tmax *
                                       twidth) as isize) as *mut pixel_t;
        pcolumndest = r_drawsurf.surfdat;
        u = 0 as libc::c_int;
        while u < r_numhblocks {
            r_lightptr =
                blocklights.as_mut_ptr().offset((u as libc::c_float /
                                                     (worldlux_s + 0.5f32)) as
                                                    libc::c_int as isize);
            prowdestbase = pcolumndest as *mut libc::c_void;
            pbasesource = basetptr.offset(soffset as isize);
            R_DrawSurfaceBlock8_World();
            soffset =
                (soffset as libc::c_uint).wrapping_add(blocksize) as
                    libc::c_int;
            if soffset >= smax { soffset = 0 as libc::c_int }
            pcolumndest = pcolumndest.offset(horzblockstep as isize);
            u += 1
        }
        return
    }
    soffset =
        (*(*r_drawsurf.surf).info).lightmapmins[0 as libc::c_int as usize] as
            libc::c_int;
    basetoffset =
        (*(*r_drawsurf.surf).info).lightmapmins[1 as libc::c_int as usize] as
            libc::c_int;
    // << 16 components are to guarantee positive values for %
    soffset =
        ((soffset >> r_drawsurf.surfmip) + (smax << 16 as libc::c_int)) %
            smax;
    basetptr =
        &mut *r_source.offset((((basetoffset >> r_drawsurf.surfmip) +
                                    (tmax << 16 as libc::c_int)) % tmax *
                                   twidth) as isize) as *mut pixel_t;
    pcolumndest = r_drawsurf.surfdat;
    u = 0 as libc::c_int;
    while u < r_numhblocks {
        r_lightptr = blocklights.as_mut_ptr().offset(u as isize);
        prowdestbase = pcolumndest as *mut libc::c_void;
        pbasesource = basetptr.offset(soffset as isize);
        Some(pblockdrawer.expect("non-null function pointer")).expect("non-null function pointer")();
        soffset =
            (soffset as libc::c_uint).wrapping_add(blocksize) as libc::c_int;
        if soffset >= smax { soffset = 0 as libc::c_int }
        pcolumndest = pcolumndest.offset(horzblockstep as isize);
        u += 1
    };
    // test what if we have very slow cache building
    //usleep(10000);
}
//=============================================================================
/*
================
R_DrawSurfaceBlock8_World

Does not draw lightmap correclty, but scale it correctly. Better than nothing
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_DrawSurfaceBlock8_World() {
    let mut v: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut lightstep: uint = 0;
    let mut lighttemp: uint = 0;
    let mut light: uint = 0;
    let mut pix: pixel_t = 0;
    let mut psource: *mut pixel_t = 0 as *mut pixel_t;
    let mut prowdest: *mut pixel_t = 0 as *mut pixel_t;
    let mut lightpos: libc::c_int = 0 as libc::c_int;
    psource = pbasesource;
    prowdest = prowdestbase as *mut pixel_t;
    v = 0 as libc::c_int;
    while v < r_numvblocks {
        // FIXME: make these locals?
	// FIXME: use delta rather than both right and left, like ASM?
        lightleft =
            *r_lightptr.offset((lightpos / r_lightwidth * r_lightwidth) as
                                   isize);
        lightright =
            *r_lightptr.offset((lightpos / r_lightwidth * r_lightwidth +
                                    1 as libc::c_int) as isize);
        lightpos =
            (lightpos as libc::c_float +
                 r_lightwidth as libc::c_float / worldlux_s) as libc::c_int;
        lightleftstep =
            (*r_lightptr.offset((lightpos / r_lightwidth * r_lightwidth) as
                                    isize)).wrapping_sub(lightleft) >>
                4 as libc::c_int - r_drawsurf.surfmip;
        lightrightstep =
            (*r_lightptr.offset((lightpos / r_lightwidth * r_lightwidth +
                                     1 as libc::c_int) as
                                    isize)).wrapping_sub(lightright) >>
                4 as libc::c_int - r_drawsurf.surfmip;
        i = 0 as libc::c_int;
        while (i as libc::c_uint) < blocksize {
            lighttemp = lightleft.wrapping_sub(lightright);
            lightstep = lighttemp >> 4 as libc::c_int - r_drawsurf.surfmip;
            light = lightright;
            b =
                blocksize.wrapping_sub(1 as libc::c_int as libc::c_uint) as
                    libc::c_int;
            while b >= 0 as libc::c_int {
                //pix = psource[(uint)(b * worldlux_s)];
                pix = *psource.offset(b as isize);
                *prowdest.offset(b as isize) =
                    (vid.colormap[((pix as libc::c_int >> 3 as libc::c_int) as
                                       libc::c_uint |
                                       (light &
                                            0x1f00 as libc::c_int as
                                                libc::c_uint) <<
                                           5 as libc::c_int) as usize] as
                         libc::c_int | pix as libc::c_int & 7 as libc::c_int)
                        as pixel_t;
                if pix as libc::c_int == 0x349 as libc::c_int {
                    *prowdest.offset(b as isize) =
                        0x349 as libc::c_int as pixel_t
                }
                //((unsigned char *)vid.colormap)
						//[(light & 0xFF00) + pix];
                light =
                    (light as libc::c_uint).wrapping_add(lightstep) as uint as
                        uint;
                b -= 1
            }
            psource = psource.offset(sourcetstep as isize);
            lightright =
                (lightright as libc::c_uint).wrapping_add(lightrightstep) as
                    uint as uint;
            lightleft =
                (lightleft as libc::c_uint).wrapping_add(lightleftstep) as
                    uint as uint;
            prowdest = prowdest.offset(surfrowbytes as isize);
            i += 1
        }
        if psource >= r_sourcemax {
            psource = psource.offset(-(r_stepback as isize))
        }
        v += 1
    };
}
/*
================
R_DrawSurfaceBlock8_Generic
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_DrawSurfaceBlock8_Generic() {
    let mut v: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut lightstep: uint = 0;
    let mut lighttemp: uint = 0;
    let mut light: uint = 0;
    let mut pix: pixel_t = 0;
    let mut psource: *mut pixel_t = 0 as *mut pixel_t;
    let mut prowdest: *mut pixel_t = 0 as *mut pixel_t;
    psource = pbasesource;
    prowdest = prowdestbase as *mut pixel_t;
    v = 0 as libc::c_int;
    while v < r_numvblocks {
        // FIXME: make these locals?
	// FIXME: use delta rather than both right and left, like ASM?
        lightleft = *r_lightptr.offset(0 as libc::c_int as isize);
        lightright = *r_lightptr.offset(1 as libc::c_int as isize);
        r_lightptr = r_lightptr.offset(r_lightwidth as isize);
        lightleftstep =
            (*r_lightptr.offset(0 as libc::c_int as
                                    isize)).wrapping_sub(lightleft) >>
                4 as libc::c_int - r_drawsurf.surfmip;
        lightrightstep =
            (*r_lightptr.offset(1 as libc::c_int as
                                    isize)).wrapping_sub(lightright) >>
                4 as libc::c_int - r_drawsurf.surfmip;
        i = 0 as libc::c_int;
        while (i as libc::c_uint) < blocksize {
            lighttemp = lightleft.wrapping_sub(lightright);
            lightstep = lighttemp >> 4 as libc::c_int - r_drawsurf.surfmip;
            light = lightright;
            b =
                blocksize.wrapping_sub(1 as libc::c_int as libc::c_uint) as
                    libc::c_int;
            while b >= 0 as libc::c_int {
                pix = *psource.offset(b as isize);
                *prowdest.offset(b as isize) =
                    (vid.colormap[((pix as libc::c_int >> 3 as libc::c_int) as
                                       libc::c_uint |
                                       (light &
                                            0x1f00 as libc::c_int as
                                                libc::c_uint) <<
                                           5 as libc::c_int) as usize] as
                         libc::c_int | pix as libc::c_int & 7 as libc::c_int)
                        as pixel_t;
                if pix as libc::c_int == 0x349 as libc::c_int {
                    *prowdest.offset(b as isize) =
                        0x349 as libc::c_int as pixel_t
                }
                //((unsigned char *)vid.colormap)
						//[(light & 0xFF00) + pix];
                light =
                    (light as libc::c_uint).wrapping_add(lightstep) as uint as
                        uint;
                b -= 1
            }
            psource = psource.offset(sourcetstep as isize);
            lightright =
                (lightright as libc::c_uint).wrapping_add(lightrightstep) as
                    uint as uint;
            lightleft =
                (lightleft as libc::c_uint).wrapping_add(lightleftstep) as
                    uint as uint;
            prowdest = prowdest.offset(surfrowbytes as isize);
            i += 1
        }
        if psource >= r_sourcemax {
            psource = psource.offset(-(r_stepback as isize))
        }
        v += 1
    };
}
/*
================
R_DrawSurfaceBlock8_mip0
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_DrawSurfaceBlock8_mip0() {
    let mut v: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut lightstep: uint = 0;
    let mut lighttemp: uint = 0;
    let mut light: uint = 0;
    let mut pix: pixel_t = 0;
    let mut psource: *mut pixel_t = 0 as *mut pixel_t;
    let mut prowdest: *mut pixel_t = 0 as *mut pixel_t;
    psource = pbasesource;
    prowdest = prowdestbase as *mut pixel_t;
    v = 0 as libc::c_int;
    while v < r_numvblocks {
        // FIXME: make these locals?
	// FIXME: use delta rather than both right and left, like ASM?
        lightleft = *r_lightptr.offset(0 as libc::c_int as isize);
        lightright = *r_lightptr.offset(1 as libc::c_int as isize);
        r_lightptr = r_lightptr.offset(r_lightwidth as isize);
        lightleftstep =
            (*r_lightptr.offset(0 as libc::c_int as
                                    isize)).wrapping_sub(lightleft) >>
                4 as libc::c_int;
        lightrightstep =
            (*r_lightptr.offset(1 as libc::c_int as
                                    isize)).wrapping_sub(lightright) >>
                4 as libc::c_int;
        i = 0 as libc::c_int;
        while i < 16 as libc::c_int {
            lighttemp = lightleft.wrapping_sub(lightright);
            lightstep = lighttemp >> 4 as libc::c_int;
            light = lightright;
            b = 15 as libc::c_int;
            while b >= 0 as libc::c_int {
                pix = *psource.offset(b as isize);
                *prowdest.offset(b as isize) =
                    (vid.colormap[((pix as libc::c_int >> 3 as libc::c_int) as
                                       libc::c_uint |
                                       (light &
                                            0x1f00 as libc::c_int as
                                                libc::c_uint) <<
                                           5 as libc::c_int) as usize] as
                         libc::c_int | pix as libc::c_int & 7 as libc::c_int)
                        as pixel_t;
                if pix as libc::c_int == 0x349 as libc::c_int {
                    *prowdest.offset(b as isize) =
                        0x349 as libc::c_int as pixel_t
                }
                // pix;
						//((unsigned char *)vid.colormap)
						//[(light & 0xFF00) + pix];
                light =
                    (light as libc::c_uint).wrapping_add(lightstep) as uint as
                        uint;
                b -= 1
            }
            psource = psource.offset(sourcetstep as isize);
            lightright =
                (lightright as libc::c_uint).wrapping_add(lightrightstep) as
                    uint as uint;
            lightleft =
                (lightleft as libc::c_uint).wrapping_add(lightleftstep) as
                    uint as uint;
            prowdest = prowdest.offset(surfrowbytes as isize);
            i += 1
        }
        if psource >= r_sourcemax {
            psource = psource.offset(-(r_stepback as isize))
        }
        v += 1
    };
}
/*
================
R_DrawSurfaceBlock8_mip1
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_DrawSurfaceBlock8_mip1() {
    let mut v: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut lightstep: uint = 0;
    let mut lighttemp: uint = 0;
    let mut light: uint = 0;
    let mut pix: pixel_t = 0;
    let mut psource: *mut pixel_t = 0 as *mut pixel_t;
    let mut prowdest: *mut pixel_t = 0 as *mut pixel_t;
    psource = pbasesource;
    prowdest = prowdestbase as *mut pixel_t;
    v = 0 as libc::c_int;
    while v < r_numvblocks {
        // FIXME: make these locals?
	// FIXME: use delta rather than both right and left, like ASM?
        lightleft = *r_lightptr.offset(0 as libc::c_int as isize);
        lightright = *r_lightptr.offset(1 as libc::c_int as isize);
        r_lightptr = r_lightptr.offset(r_lightwidth as isize);
        lightleftstep =
            (*r_lightptr.offset(0 as libc::c_int as
                                    isize)).wrapping_sub(lightleft) >>
                3 as libc::c_int;
        lightrightstep =
            (*r_lightptr.offset(1 as libc::c_int as
                                    isize)).wrapping_sub(lightright) >>
                3 as libc::c_int;
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            lighttemp = lightleft.wrapping_sub(lightright);
            lightstep = lighttemp >> 3 as libc::c_int;
            light = lightright;
            b = 7 as libc::c_int;
            while b >= 0 as libc::c_int {
                pix = *psource.offset(b as isize);
                *prowdest.offset(b as isize) =
                    (vid.colormap[((pix as libc::c_int >> 3 as libc::c_int) as
                                       libc::c_uint |
                                       (light &
                                            0x1f00 as libc::c_int as
                                                libc::c_uint) <<
                                           5 as libc::c_int) as usize] as
                         libc::c_int | pix as libc::c_int & 7 as libc::c_int)
                        as pixel_t;
                //((unsigned char *)vid.colormap)
						//[(light & 0xFF00) + pix];
                light =
                    (light as libc::c_uint).wrapping_add(lightstep) as uint as
                        uint;
                b -= 1
            }
            psource = psource.offset(sourcetstep as isize);
            lightright =
                (lightright as libc::c_uint).wrapping_add(lightrightstep) as
                    uint as uint;
            lightleft =
                (lightleft as libc::c_uint).wrapping_add(lightleftstep) as
                    uint as uint;
            prowdest = prowdest.offset(surfrowbytes as isize);
            i += 1
        }
        if psource >= r_sourcemax {
            psource = psource.offset(-(r_stepback as isize))
        }
        v += 1
    };
}
/*
================
R_DrawSurfaceBlock8_mip2
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_DrawSurfaceBlock8_mip2() {
    let mut v: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut lightstep: uint = 0;
    let mut lighttemp: uint = 0;
    let mut light: uint = 0;
    let mut pix: pixel_t = 0;
    let mut psource: *mut pixel_t = 0 as *mut pixel_t;
    let mut prowdest: *mut pixel_t = 0 as *mut pixel_t;
    psource = pbasesource;
    prowdest = prowdestbase as *mut pixel_t;
    v = 0 as libc::c_int;
    while v < r_numvblocks {
        // FIXME: make these locals?
	// FIXME: use delta rather than both right and left, like ASM?
        lightleft = *r_lightptr.offset(0 as libc::c_int as isize);
        lightright = *r_lightptr.offset(1 as libc::c_int as isize);
        r_lightptr = r_lightptr.offset(r_lightwidth as isize);
        lightleftstep =
            (*r_lightptr.offset(0 as libc::c_int as
                                    isize)).wrapping_sub(lightleft) >>
                2 as libc::c_int;
        lightrightstep =
            (*r_lightptr.offset(1 as libc::c_int as
                                    isize)).wrapping_sub(lightright) >>
                2 as libc::c_int;
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            lighttemp = lightleft.wrapping_sub(lightright);
            lightstep = lighttemp >> 2 as libc::c_int;
            light = lightright;
            b = 3 as libc::c_int;
            while b >= 0 as libc::c_int {
                pix = *psource.offset(b as isize);
                *prowdest.offset(b as isize) =
                    (vid.colormap[((pix as libc::c_int >> 3 as libc::c_int) as
                                       libc::c_uint |
                                       (light &
                                            0x1f00 as libc::c_int as
                                                libc::c_uint) <<
                                           5 as libc::c_int) as usize] as
                         libc::c_int | pix as libc::c_int & 7 as libc::c_int)
                        as pixel_t;
                //((unsigned char *)vid.colormap)
						//[(light & 0xFF00) + pix];
                light =
                    (light as libc::c_uint).wrapping_add(lightstep) as uint as
                        uint;
                b -= 1
            }
            psource = psource.offset(sourcetstep as isize);
            lightright =
                (lightright as libc::c_uint).wrapping_add(lightrightstep) as
                    uint as uint;
            lightleft =
                (lightleft as libc::c_uint).wrapping_add(lightleftstep) as
                    uint as uint;
            prowdest = prowdest.offset(surfrowbytes as isize);
            i += 1
        }
        if psource >= r_sourcemax {
            psource = psource.offset(-(r_stepback as isize))
        }
        v += 1
    };
}
/*
================
R_DrawSurfaceBlock8_mip3
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_DrawSurfaceBlock8_mip3() {
    let mut v: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut lightstep: uint = 0;
    let mut lighttemp: uint = 0;
    let mut light: uint = 0;
    let mut pix: pixel_t = 0;
    let mut psource: *mut pixel_t = 0 as *mut pixel_t;
    let mut prowdest: *mut pixel_t = 0 as *mut pixel_t;
    psource = pbasesource;
    prowdest = prowdestbase as *mut pixel_t;
    v = 0 as libc::c_int;
    while v < r_numvblocks {
        // FIXME: make these locals?
	// FIXME: use delta rather than both right and left, like ASM?
        lightleft = *r_lightptr.offset(0 as libc::c_int as isize);
        lightright = *r_lightptr.offset(1 as libc::c_int as isize);
        r_lightptr = r_lightptr.offset(r_lightwidth as isize);
        lightleftstep =
            (*r_lightptr.offset(0 as libc::c_int as
                                    isize)).wrapping_sub(lightleft) >>
                1 as libc::c_int;
        lightrightstep =
            (*r_lightptr.offset(1 as libc::c_int as
                                    isize)).wrapping_sub(lightright) >>
                1 as libc::c_int;
        i = 0 as libc::c_int;
        while i < 2 as libc::c_int {
            lighttemp = lightleft.wrapping_sub(lightright);
            lightstep = lighttemp >> 1 as libc::c_int;
            light = lightright;
            b = 1 as libc::c_int;
            while b >= 0 as libc::c_int {
                pix = *psource.offset(b as isize);
                *prowdest.offset(b as isize) =
                    (vid.colormap[((pix as libc::c_int >> 3 as libc::c_int) as
                                       libc::c_uint |
                                       (light &
                                            0x1f00 as libc::c_int as
                                                libc::c_uint) <<
                                           5 as libc::c_int) as usize] as
                         libc::c_int | pix as libc::c_int & 7 as libc::c_int)
                        as pixel_t;
                //((unsigned char *)vid.colormap)
						//[(light & 0xFF00) + pix];
                light =
                    (light as libc::c_uint).wrapping_add(lightstep) as uint as
                        uint;
                b -= 1
            }
            psource = psource.offset(sourcetstep as isize);
            lightright =
                (lightright as libc::c_uint).wrapping_add(lightrightstep) as
                    uint as uint;
            lightleft =
                (lightleft as libc::c_uint).wrapping_add(lightleftstep) as
                    uint as uint;
            prowdest = prowdest.offset(surfrowbytes as isize);
            i += 1
        }
        if psource >= r_sourcemax {
            psource = psource.offset(-(r_stepback as isize))
        }
        v += 1
    };
}
//============================================================================
/*
================
R_InitCaches

================
*/
#[no_mangle]
pub unsafe extern "C" fn R_InitCaches() {
    let mut size: libc::c_int = 0;
    let mut pix: libc::c_int = 0;
    // calculate size to allocate
    if (*sw_surfcacheoverride).value != 0. {
        size = (*sw_surfcacheoverride).value as libc::c_int
    } else {
        size = 1024 as libc::c_int * 768 as libc::c_int * 2 as libc::c_int;
        pix = vid.width * vid.height * 2 as libc::c_int;
        if pix > 64000 as libc::c_int {
            size += (pix - 64000 as libc::c_int) * 3 as libc::c_int
        }
    }
    // round up to page size
    size = size + 8191 as libc::c_int & !(8191 as libc::c_int);
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"%s surface cache\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             Q_pretifymem(size
                                                                              as
                                                                              libc::c_float,
                                                                          2 as
                                                                              libc::c_int));
    sc_size = size;
    if !sc_base.is_null() {
        D_FlushCaches();
        gEngfuncs._Mem_Free.expect("non-null function pointer")(sc_base as
                                                                    *mut libc::c_void,
                                                                b"../ref_soft/r_surf.c\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char,
                                                                990 as
                                                                    libc::c_int);
    }
    sc_base =
        gEngfuncs._Mem_Alloc.expect("non-null function pointer")(r_temppool,
                                                                 size as
                                                                     size_t,
                                                                 true_0,
                                                                 b"../ref_soft/r_surf.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 992 as
                                                                     libc::c_int)
            as *mut surfcache_t;
    sc_rover = sc_base;
    (*sc_base).next = 0 as *mut surfcache_s;
    (*sc_base).owner = 0 as *mut *mut surfcache_s;
    (*sc_base).size = sc_size;
}
/*
==================
D_FlushCaches
==================
*/
#[no_mangle]
pub unsafe extern "C" fn D_FlushCaches() {
    let mut c: *mut surfcache_t = 0 as *mut surfcache_t;
    // if newmap, surfaces already freed
    if tr.map_unload as u64 == 0 {
        c = sc_base;
        while !c.is_null() {
            if !(*c).owner.is_null() { *(*c).owner = 0 as *mut surfcache_s }
            c = (*c).next
        }
    }
    sc_rover = sc_base;
    (*sc_base).next = 0 as *mut surfcache_s;
    (*sc_base).owner = 0 as *mut *mut surfcache_s;
    (*sc_base).size = sc_size;
}
/*
=================
D_SCAlloc
=================
*/
#[no_mangle]
pub unsafe extern "C" fn D_SCAlloc(mut width: libc::c_int,
                                   mut size: libc::c_int)
 -> *mut surfcache_t {
    let mut new: *mut surfcache_t = 0 as *mut surfcache_t;
    let mut wrapped_this_time: qboolean = false_0;
    if width < 0 as libc::c_int {
        // || (width > 256))
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"D_SCAlloc: bad cache width %d\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 width);
    }
    if size <= 0 as libc::c_int || size > 0x10000000 as libc::c_int {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"D_SCAlloc: bad cache size %d\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 size);
    }
    size =
        &mut *(*(0 as
                     *mut surfcache_t)).data.as_mut_ptr().offset(size as
                                                                     isize) as
            *mut byte as libc::c_int;
    size = size + 3 as libc::c_int & !(3 as libc::c_int);
    if size > sc_size {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"D_SCAlloc: %i > cache size of %i\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 size,
                                                                 sc_size);
    }
    // if there is not size bytes after the rover, reset to the start
    wrapped_this_time = false_0;
    if sc_rover.is_null() ||
           (sc_rover as *mut byte).wrapping_offset_from(sc_base as *mut byte)
               as libc::c_long > (sc_size - size) as libc::c_long {
        if !sc_rover.is_null() { wrapped_this_time = true_0 }
        sc_rover = sc_base
    }
    // colect and free surfcache_t blocks until the rover block is large enough
    new = sc_rover;
    if !(*sc_rover).owner.is_null() {
        *(*sc_rover).owner = 0 as *mut surfcache_s
    }
    while (*new).size < size {
        // free another
        sc_rover = (*sc_rover).next;
        if sc_rover.is_null() {
            gEngfuncs.Host_Error.expect("non-null function pointer")(b"D_SCAlloc: hit the end of memory\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char);
        }
        if !(*sc_rover).owner.is_null() {
            *(*sc_rover).owner = 0 as *mut surfcache_s
        }
        (*new).size += (*sc_rover).size;
        (*new).next = (*sc_rover).next
    }
    // create a fragment out of any leftovers
    if (*new).size - size > 256 as libc::c_int {
        sc_rover =
            (new as *mut byte).offset(size as isize) as *mut surfcache_t;
        (*sc_rover).size = (*new).size - size;
        (*sc_rover).next = (*new).next;
        (*sc_rover).width = 0 as libc::c_int as libc::c_uint;
        (*sc_rover).owner = 0 as *mut *mut surfcache_s;
        (*new).next = sc_rover;
        (*new).size = size
    } else { sc_rover = (*new).next }
    (*new).width = width as libc::c_uint;
    // DEBUG
    if width > 0 as libc::c_int {
        (*new).height =
            (size as
                 libc::c_ulong).wrapping_sub(::std::mem::size_of::<surfcache_t>()
                                                 as
                                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<[byte; 4]>()
                                                                                 as
                                                                                 libc::c_ulong).wrapping_div(width
                                                                                                                 as
                                                                                                                 libc::c_ulong)
                as libc::c_uint
    } // should be set properly after return
    (*new).owner = 0 as *mut *mut surfcache_s;
    if d_roverwrapped as u64 != 0 {
        if wrapped_this_time as libc::c_uint != 0 ||
               sc_rover >= d_initial_rover {
            r_cache_thrash = true_0
        }
    } else if wrapped_this_time as u64 != 0 { d_roverwrapped = true_0 }
    return new;
}
/*
=================
D_SCDump
=================
*/
#[no_mangle]
pub unsafe extern "C" fn D_SCDump() {
    let mut test: *mut surfcache_t = 0 as *mut surfcache_t;
    test = sc_base;
    while !test.is_null() {
        if test == sc_rover {
            gEngfuncs.Con_Printf.expect("non-null function pointer")(b"ROVER:\n\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char);
        }
        gEngfuncs.Con_Printf.expect("non-null function pointer")(b"%p : %i bytes     %i width\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 test,
                                                                 (*test).size,
                                                                 (*test).width);
        test = (*test).next
    };
}
//=============================================================================
// if the num is not a power of 2, assume it will not repeat
#[no_mangle]
pub unsafe extern "C" fn MaskForNum(mut num: libc::c_int) -> libc::c_int {
    if num == 128 as libc::c_int { return 127 as libc::c_int }
    if num == 64 as libc::c_int { return 63 as libc::c_int }
    if num == 32 as libc::c_int { return 31 as libc::c_int }
    if num == 16 as libc::c_int { return 15 as libc::c_int }
    return 255 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn D_log2(mut num: libc::c_int) -> libc::c_int {
    let mut c: libc::c_int = 0;
    c = 0 as libc::c_int;
    loop  { num >>= 1 as libc::c_int; if !(num != 0) { break ; } c += 1 }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawSurfaceDecals() {
    let mut fa: *mut msurface_t = r_drawsurf.surf;
    let mut p: *mut decal_t = 0 as *mut decal_t;
    p = (*fa).pdecals;
    while !p.is_null() {
        let mut dest: *mut pixel_t = 0 as *mut pixel_t;
        let mut source: *mut pixel_t = 0 as *mut pixel_t;
        let mut textureU: vec4_t = [0.; 4];
        let mut textureV: vec4_t = [0.; 4];
        let mut tex: *mut image_t =
            R_GetTexture((*p).texture as libc::c_uint);
        let mut s1: libc::c_int = 0 as libc::c_int;
        let mut t1: libc::c_int = 0 as libc::c_int;
        let mut s2: libc::c_int = (*tex).width as libc::c_int;
        let mut t2: libc::c_int = (*tex).height as libc::c_int;
        let mut height: libc::c_uint = 0;
        let mut f: libc::c_uint = 0;
        let mut fstep: libc::c_uint = 0;
        let mut skip: libc::c_int = 0;
        let mut buffer: *mut pixel_t = 0 as *mut pixel_t;
        let mut transparent: qboolean = false_0;
        let mut x: libc::c_int = 0;
        let mut y: libc::c_int = 0;
        let mut u: libc::c_int = 0;
        let mut v: libc::c_int = 0;
        let mut sv: libc::c_int = 0;
        let mut w: libc::c_int = 0;
        let mut h: libc::c_int = 0;
        let mut basis: [vec3_t; 3] = [[0.; 3]; 3];
        textureU[0 as libc::c_int as usize] =
            (*(*fa).texinfo).vecs[0 as libc::c_int as
                                      usize][0 as libc::c_int as usize];
        textureU[1 as libc::c_int as usize] =
            (*(*fa).texinfo).vecs[0 as libc::c_int as
                                      usize][1 as libc::c_int as usize];
        textureU[2 as libc::c_int as usize] =
            (*(*fa).texinfo).vecs[0 as libc::c_int as
                                      usize][2 as libc::c_int as usize];
        textureU[3 as libc::c_int as usize] =
            (*(*fa).texinfo).vecs[0 as libc::c_int as
                                      usize][3 as libc::c_int as usize];
        textureV[0 as libc::c_int as usize] =
            (*(*fa).texinfo).vecs[1 as libc::c_int as
                                      usize][0 as libc::c_int as usize];
        textureV[1 as libc::c_int as usize] =
            (*(*fa).texinfo).vecs[1 as libc::c_int as
                                      usize][1 as libc::c_int as usize];
        textureV[2 as libc::c_int as usize] =
            (*(*fa).texinfo).vecs[1 as libc::c_int as
                                      usize][2 as libc::c_int as usize];
        textureV[3 as libc::c_int as usize] =
            (*(*fa).texinfo).vecs[1 as libc::c_int as
                                      usize][3 as libc::c_int as usize];
        R_DecalComputeBasis(fa, 0 as libc::c_int, basis.as_mut_ptr());
        w =
            (__tg_fabs((*tex).width as libc::c_int as libc::c_float *
                           (textureU[0 as libc::c_int as usize] *
                                basis[0 as libc::c_int as
                                          usize][0 as libc::c_int as usize] +
                                textureU[1 as libc::c_int as usize] *
                                    basis[0 as libc::c_int as
                                              usize][1 as libc::c_int as
                                                         usize] +
                                textureU[2 as libc::c_int as usize] *
                                    basis[0 as libc::c_int as
                                              usize][2 as libc::c_int as
                                                         usize])) +
                 __tg_fabs((*tex).height as libc::c_int as libc::c_float *
                               (textureU[0 as libc::c_int as usize] *
                                    basis[1 as libc::c_int as
                                              usize][0 as libc::c_int as
                                                         usize] +
                                    textureU[1 as libc::c_int as usize] *
                                        basis[1 as libc::c_int as
                                                  usize][1 as libc::c_int as
                                                             usize] +
                                    textureU[2 as libc::c_int as usize] *
                                        basis[1 as libc::c_int as
                                                  usize][2 as libc::c_int as
                                                             usize]))) as
                libc::c_int;
        h =
            (__tg_fabs((*tex).width as libc::c_int as libc::c_float *
                           (textureV[0 as libc::c_int as usize] *
                                basis[0 as libc::c_int as
                                          usize][0 as libc::c_int as usize] +
                                textureV[1 as libc::c_int as usize] *
                                    basis[0 as libc::c_int as
                                              usize][1 as libc::c_int as
                                                         usize] +
                                textureV[2 as libc::c_int as usize] *
                                    basis[0 as libc::c_int as
                                              usize][2 as libc::c_int as
                                                         usize])) +
                 __tg_fabs((*tex).height as libc::c_int as libc::c_float *
                               (textureV[0 as libc::c_int as usize] *
                                    basis[1 as libc::c_int as
                                              usize][0 as libc::c_int as
                                                         usize] +
                                    textureV[1 as libc::c_int as usize] *
                                        basis[1 as libc::c_int as
                                                  usize][1 as libc::c_int as
                                                             usize] +
                                    textureV[2 as libc::c_int as usize] *
                                        basis[1 as libc::c_int as
                                                  usize][2 as libc::c_int as
                                                             usize]))) as
                libc::c_int;
        // project decal center into the texture space of the surface
        x =
            ((*p).position[0 as libc::c_int as usize] *
                 textureU[0 as libc::c_int as usize] +
                 (*p).position[1 as libc::c_int as usize] *
                     textureU[1 as libc::c_int as usize] +
                 (*p).position[2 as libc::c_int as usize] *
                     textureU[2 as libc::c_int as usize] +
                 textureU[3 as libc::c_int as usize] -
                 (*fa).texturemins[0 as libc::c_int as usize] as libc::c_int
                     as libc::c_float -
                 (w / 2 as libc::c_int) as libc::c_float) as libc::c_int;
        y =
            ((*p).position[0 as libc::c_int as usize] *
                 textureV[0 as libc::c_int as usize] +
                 (*p).position[1 as libc::c_int as usize] *
                     textureV[1 as libc::c_int as usize] +
                 (*p).position[2 as libc::c_int as usize] *
                     textureV[2 as libc::c_int as usize] +
                 textureV[3 as libc::c_int as usize] -
                 (*fa).texturemins[1 as libc::c_int as usize] as libc::c_int
                     as libc::c_float -
                 (h / 2 as libc::c_int) as libc::c_float) as libc::c_int;
        x = x >> r_drawsurf.surfmip;
        y = y >> r_drawsurf.surfmip;
        w = w >> r_drawsurf.surfmip;
        h = h >> r_drawsurf.surfmip;
        if !(w < 1 as libc::c_int || h < 1 as libc::c_int) {
            if x < 0 as libc::c_int {
                s1 += -x * (s2 - s1) / w;
                x = 0 as libc::c_int
            }
            if x + w > r_drawsurf.surfwidth {
                s2 -= (x + w - r_drawsurf.surfwidth) * (s2 - s1) / w;
                w = r_drawsurf.surfwidth - x
            }
            if y + h > r_drawsurf.surfheight {
                t2 -= (y + h - r_drawsurf.surfheight) * (t2 - t1) / h;
                h = r_drawsurf.surfheight - y
            }
            if s1 < 0 as libc::c_int { s1 = 0 as libc::c_int }
            if t1 < 0 as libc::c_int { t1 = 0 as libc::c_int }
            if s2 > (*tex).width as libc::c_int {
                s2 = (*tex).width as libc::c_int
            }
            if t2 > (*tex).height as libc::c_int {
                t2 = (*tex).height as libc::c_int
            }
            if !((*tex).pixels[0 as libc::c_int as usize].is_null() ||
                     s1 >= s2 || t1 >= t2 || w == 0) {
                if !(*tex).alpha_pixels.is_null() {
                    buffer = (*tex).alpha_pixels;
                    transparent = true_0
                } else { buffer = (*tex).pixels[0 as libc::c_int as usize] }
                height = h as libc::c_uint;
                if y < 0 as libc::c_int {
                    skip = -y;
                    height = height.wrapping_add(y as libc::c_uint);
                    y = 0 as libc::c_int
                } else { skip = 0 as libc::c_int }
                dest =
                    r_drawsurf.surfdat.offset((y * r_drawsurf.rowbytes) as
                                                  isize).offset(x as isize);
                v = 0 as libc::c_int;
                while (v as libc::c_uint) < height {
                    //int alpha1 = vid.alpha;
                    sv = (skip + v) * (t2 - t1) / h + t1;
                    source =
                        buffer.offset((sv * (*tex).width as libc::c_int) as
                                          isize).offset(s1 as isize);
                    f = 0 as libc::c_int as libc::c_uint;
                    fstep =
                        ((s2 - s1) * 0x10000 as libc::c_int / w) as
                            libc::c_uint;
                    if w == s2 - s1 {
                        fstep = 0x10000 as libc::c_int as libc::c_uint
                    }
                    u = 0 as libc::c_int;
                    while u < w {
                        let mut src: pixel_t =
                            *source.offset((f >> 16 as libc::c_int) as isize);
                        let mut alpha: libc::c_int = 7 as libc::c_int;
                        f = f.wrapping_add(fstep);
                        if transparent as u64 != 0 {
                            alpha &=
                                src as libc::c_int >>
                                    16 as libc::c_int - 3 as libc::c_int;
                            src =
                                ((src as libc::c_int) << 3 as libc::c_int) as
                                    pixel_t
                        }
                        if !(alpha <= 0 as libc::c_int) {
                            if alpha < 7 as libc::c_int {
                                // && (vid.rendermode == kRenderTransAlpha || vid.rendermode == kRenderTransTexture ) )
                                let mut screen: pixel_t =
                                    *dest.offset(u as
                                                     isize); //  | 0xff & screen & src ;
                                if !(screen as libc::c_int ==
                                         0x349 as libc::c_int) {
                                    *dest.offset(u as isize) =
                                        if alpha > 3 as libc::c_int {
                                            (vid.alphamap[(7 as libc::c_int -
                                                               1 as
                                                                   libc::c_int
                                                               - alpha <<
                                                               18 as
                                                                   libc::c_int
                                                               |
                                                               (screen as
                                                                    libc::c_int
                                                                    &
                                                                    0xff00 as
                                                                        libc::c_int)
                                                                   <<
                                                                   2 as
                                                                       libc::c_int
                                                               |
                                                               src as
                                                                   libc::c_int
                                                                   >>
                                                                   6 as
                                                                       libc::c_int)
                                                              as usize] as
                                                 libc::c_int) |
                                                src as libc::c_int &
                                                    0x3f as libc::c_int
                                        } else {
                                            (vid.alphamap[((alpha -
                                                                1 as
                                                                    libc::c_int)
                                                               <<
                                                               18 as
                                                                   libc::c_int
                                                               |
                                                               (src as
                                                                    libc::c_int
                                                                    &
                                                                    0xff00 as
                                                                        libc::c_int)
                                                                   <<
                                                                   2 as
                                                                       libc::c_int
                                                               |
                                                               screen as
                                                                   libc::c_int
                                                                   >>
                                                                   6 as
                                                                       libc::c_int)
                                                              as usize] as
                                                 libc::c_int) |
                                                screen as libc::c_int &
                                                    0x3f as libc::c_int
                                        } as pixel_t
                                }
                            } else { *dest.offset(u as isize) = src }
                        }
                        u += 1
                    }
                    dest = dest.offset(r_drawsurf.rowbytes as isize);
                    v += 1
                }
            }
        }
        p = (*p).pnext
    };
}
/*
================
D_CacheSurface
================
*/
#[no_mangle]
pub unsafe extern "C" fn D_CacheSurface(mut surface: *mut msurface_t,
                                        mut miplevel: libc::c_int)
 -> *mut surfcache_t {
    let mut cache: *mut surfcache_t = 0 as *mut surfcache_t;
    let mut maps: libc::c_int = 0;
    //
// if the surface is animating or flashing, flush the cache
//
    r_drawsurf.image =
        R_GetTexture((*R_TextureAnimation(surface)).gl_texturenum as
                         libc::c_uint);
    // does not support conveyors with world luxels now
    if (*(*surface).texinfo).flags as libc::c_uint &
           (1 as libc::c_uint) << 1 as libc::c_int != 0 {
        (*surface).flags =
            ((*surface).flags as libc::c_uint &
                 !((1 as libc::c_uint) << 6 as libc::c_int)) as libc::c_int
    }
    if (*surface).flags as libc::c_uint &
           (1 as libc::c_uint) << 6 as libc::c_int != 0 {
        if miplevel >= 1 as libc::c_int {
            (*surface).extents[0 as libc::c_int as usize] =
                ((*(*surface).info).lightextents[0 as libc::c_int as usize] as
                     libc::c_int *
                     (if tr.sample_size == -(1 as libc::c_int) {
                          gEngfuncs.Mod_SampleSizeForFace.expect("non-null function pointer")(r_drawsurf.surf)
                      } else { tr.sample_size }) * 2 as libc::c_int) as
                    libc::c_short;
            (*(*surface).info).lightmapmins[0 as libc::c_int as usize] =
                (-((*(*surface).info).lightextents[0 as libc::c_int as usize]
                       as libc::c_int) *
                     (if tr.sample_size == -(1 as libc::c_int) {
                          gEngfuncs.Mod_SampleSizeForFace.expect("non-null function pointer")(r_drawsurf.surf)
                      } else { tr.sample_size })) as libc::c_short
        } else {
            (*surface).extents[0 as libc::c_int as usize] =
                ((*(*surface).info).lightextents[0 as libc::c_int as usize] as
                     libc::c_int *
                     (if tr.sample_size == -(1 as libc::c_int) {
                          gEngfuncs.Mod_SampleSizeForFace.expect("non-null function pointer")(r_drawsurf.surf)
                      } else { tr.sample_size })) as libc::c_short;
            (*(*surface).info).lightmapmins[0 as libc::c_int as usize] =
                (-((*(*surface).info).lightextents[0 as libc::c_int as usize]
                       as libc::c_int) *
                     (if tr.sample_size == -(1 as libc::c_int) {
                          gEngfuncs.Mod_SampleSizeForFace.expect("non-null function pointer")(r_drawsurf.surf)
                      } else { tr.sample_size }) / 2 as libc::c_int) as
                    libc::c_short
        }
    }
    // / todo: port this
	//r_drawsurf.lightadj[0] = r_newrefdef.lightstyles[surface->styles[0]].white*128;
	//r_drawsurf.lightadj[1] = r_newrefdef.lightstyles[surface->styles[1]].white*128;
	//r_drawsurf.lightadj[2] = r_newrefdef.lightstyles[surface->styles[2]].white*128;
	//r_drawsurf.lightadj[3] = r_newrefdef.lightstyles[surface->styles[3]].white*128;
    //
// see if the cache holds apropriate data
//
    cache =
        *((*(*surface).info).reserved.as_mut_ptr() as
              *mut *mut surfcache_t).offset(miplevel as isize);
    // check for lightmap modification
    maps = 0 as libc::c_int;
    while maps < 4 as libc::c_int &&
              (*surface).styles[maps as usize] as libc::c_int !=
                  255 as libc::c_int {
        if tr.lightstylevalue[(*surface).styles[maps as usize] as usize] !=
               (*surface).cached_light[maps as usize] {
            (*surface).dlightframe = tr.framecount
        }
        maps += 1
    }
    if !cache.is_null() && (*cache).dlight == 0 &&
           (*surface).dlightframe != tr.framecount &&
           (*cache).image == r_drawsurf.image &&
           (*cache).lightadj[0 as libc::c_int as usize] ==
               r_drawsurf.lightadj[0 as libc::c_int as usize] &&
           (*cache).lightadj[1 as libc::c_int as usize] ==
               r_drawsurf.lightadj[1 as libc::c_int as usize] &&
           (*cache).lightadj[2 as libc::c_int as usize] ==
               r_drawsurf.lightadj[2 as libc::c_int as usize] &&
           (*cache).lightadj[3 as libc::c_int as usize] ==
               r_drawsurf.lightadj[3 as libc::c_int as usize] {
        return cache
    }
    if (*surface).dlightframe == tr.framecount {
        let mut i: libc::c_int = 0;
        // invalidate dlight cache
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            if !(*((*(*surface).info).reserved.as_mut_ptr() as
                       *mut *mut surfcache_t).offset(i as isize)).is_null() {
                let ref mut fresh4 =
                    (**((*(*surface).info).reserved.as_mut_ptr() as
                            *mut *mut surfcache_t).offset(i as isize)).image;
                *fresh4 = 0 as *mut image_t
            }
            i += 1
        }
    }
    //
// determine shape of surface
//
    surfscale =
        (1.0f64 / ((1 as libc::c_int) << miplevel) as libc::c_double) as
            libc::c_float;
    r_drawsurf.surfmip = miplevel;
    if (*surface).flags as libc::c_uint &
           (1 as libc::c_uint) << 6 as libc::c_int != 0 {
        r_drawsurf.surfwidth =
            (*surface).extents[0 as libc::c_int as usize] as libc::c_int >>
                miplevel
    } else {
        r_drawsurf.surfwidth =
            (*(*surface).info).lightextents[0 as libc::c_int as usize] as
                libc::c_int >> miplevel
    }
    r_drawsurf.rowbytes = r_drawsurf.surfwidth;
    r_drawsurf.surfheight =
        (*(*surface).info).lightextents[1 as libc::c_int as usize] as
            libc::c_int >> miplevel;
    // use texture space if world luxels used
    if (*(*surface).texinfo).flags as libc::c_uint &
           (1 as libc::c_uint) << 1 as libc::c_int != 0 {
        r_drawsurf.surfwidth =
            (*surface).extents[0 as libc::c_int as usize] as libc::c_int >>
                miplevel;
        r_drawsurf.rowbytes = r_drawsurf.surfwidth;
        r_drawsurf.surfheight =
            (*surface).extents[1 as libc::c_int as usize] as libc::c_int >>
                miplevel
    }
    //
// allocate memory if needed
//
    if cache.is_null() {
        // if a texture just animated, don't reallocate it
        cache =
            D_SCAlloc(r_drawsurf.surfwidth,
                      r_drawsurf.surfwidth * r_drawsurf.surfheight *
                          2 as libc::c_int);
        let ref mut fresh5 =
            *((*(*surface).info).reserved.as_mut_ptr() as
                  *mut *mut surfcache_t).offset(miplevel as isize);
        *fresh5 = cache;
        (*cache).owner =
            &mut *((*(*surface).info).reserved.as_mut_ptr() as
                       *mut *mut surfcache_t).offset(miplevel as isize) as
                *mut *mut surfcache_t;
        (*cache).mipscale = surfscale
    }
    if (*surface).dlightframe == tr.framecount {
        (*cache).dlight = 1 as libc::c_int
    } else { (*cache).dlight = 0 as libc::c_int }
    r_drawsurf.surfdat = (*cache).data.as_mut_ptr() as *mut pixel_t;
    (*cache).image = r_drawsurf.image;
    (*cache).lightadj[0 as libc::c_int as usize] =
        r_drawsurf.lightadj[0 as libc::c_int as usize];
    (*cache).lightadj[1 as libc::c_int as usize] =
        r_drawsurf.lightadj[1 as libc::c_int as usize];
    (*cache).lightadj[2 as libc::c_int as usize] =
        r_drawsurf.lightadj[2 as libc::c_int as usize];
    (*cache).lightadj[3 as libc::c_int as usize] =
        r_drawsurf.lightadj[3 as libc::c_int as usize];
    maps = 0 as libc::c_int;
    while maps < 4 as libc::c_int &&
              (*surface).styles[maps as usize] as libc::c_int !=
                  255 as libc::c_int {
        (*surface).cached_light[maps as usize] =
            tr.lightstylevalue[(*surface).styles[maps as usize] as usize];
        maps += 1
    }
    //
// draw and light the surface texture
//
    r_drawsurf.surf = surface;
    //c_surf++;
    // calculate the lightings
    R_BuildLightMap();
    // rasterize the surface into the cache
    R_DrawSurface();
    R_DrawSurfaceDecals();
    return cache;
}
