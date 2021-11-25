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
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn sqrt(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn VectorNormalizeLength2(v: *const vec_t, out: *mut vec_t)
     -> libc::c_float;
    #[no_mangle]
    static mut r_temppool: poolhandle_t;
    #[no_mangle]
    static mut vid: viddef_t;
    #[no_mangle]
    static mut tr: gl_globals_t;
    #[no_mangle]
    static mut r_affinetridesc: affinetridesc_t;
    #[no_mangle]
    static mut gEngfuncs: ref_api_t;
    #[no_mangle]
    static mut sw_noalphabrushes: *mut cvar_t;
    #[no_mangle]
    static mut gl_emboss_scale: *mut cvar_t;
    #[no_mangle]
    fn COM_HashKey(string: *const libc::c_char, hashSize: uint) -> uint;
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Q_strnicmp(s1: *const libc::c_char, s2: *const libc::c_char,
                  n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Q_strlen(string: *const libc::c_char) -> size_t;
    #[no_mangle]
    fn Q_pretifymem(value: libc::c_float, digitsafterdecimal: libc::c_int)
     -> *mut libc::c_char;
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
pub type pixformat_t = libc::c_uint;
pub const PF_TOTALCOUNT: pixformat_t = 12;
pub const PF_ATI2: pixformat_t = 11;
pub const PF_DXT5: pixformat_t = 10;
pub const PF_DXT3: pixformat_t = 9;
pub const PF_DXT1: pixformat_t = 8;
pub const PF_LUMINANCE: pixformat_t = 7;
pub const PF_BGR_24: pixformat_t = 6;
pub const PF_RGB_24: pixformat_t = 5;
pub const PF_BGRA_32: pixformat_t = 4;
pub const PF_RGBA_32: pixformat_t = 3;
pub const PF_INDEXED_32: pixformat_t = 2;
pub const PF_INDEXED_24: pixformat_t = 1;
pub const PF_UNKNOWN: pixformat_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bpc_desc_s {
    pub format: libc::c_int,
    pub name: [libc::c_char; 16],
    pub glFormat: uint,
    pub bpp: libc::c_int,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const IL_OVERVIEW: C2RustUnnamed_0 = 64;
pub const IL_LOAD_DECAL: C2RustUnnamed_0 = 32;
pub const IL_DDS_HARDWARE: C2RustUnnamed_0 = 16;
pub const IL_DONTFLIP_TGA: C2RustUnnamed_0 = 8;
pub const IL_ALLOW_OVERWRITE: C2RustUnnamed_0 = 4;
pub const IL_KEEP_8BIT: C2RustUnnamed_0 = 2;
pub const IL_USE_LERPING: C2RustUnnamed_0 = 1;
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
pub type ref_api_t = ref_api_s;
pub type imagetype_t = libc::c_uint;
pub const it_sky: imagetype_t = 4;
pub const it_pic: imagetype_t = 3;
pub const it_wall: imagetype_t = 2;
pub const it_sprite: imagetype_t = 1;
pub const it_skin: imagetype_t = 0;
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
pub type finalvert_t = finalvert_s;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dtriangle_t {
    pub index_xyz: [libc::c_short; 3],
    pub index_st: [libc::c_short; 3],
}
#[inline(always)]
unsafe extern "C" fn __tg_sqrt(mut __x: libc::c_double) -> libc::c_double {
    return sqrt(__x);
}
static mut r_images: [image_t; 4096] =
    [image_t{name: [0; 256],
             srcWidth: 0,
             srcHeight: 0,
             width: 0,
             height: 0,
             depth: 0,
             numMips: 0,
             flags: TF_COLORMAP,
             fogParams: [0; 4],
             original: 0 as *const rgbdata_t as *mut rgbdata_t,
             size: 0,
             xscale: 0.,
             yscale: 0.,
             type_0: it_skin,
             pixels: [0 as *const pixel_t as *mut pixel_t; 4],
             alpha_pixels: 0 as *const pixel_t as *mut pixel_t,
             servercount: 0,
             hashValue: 0,
             nextHash: 0 as *const image_s as *mut image_s,}; 4096];
static mut r_imagesHashTable: [*mut image_t; 1024] =
    [0 as *const image_t as *mut image_t; 1024];
static mut r_numImages: uint = 0;
/*
=================
R_GetTexture

acess to array elem
=================
*/
#[no_mangle]
pub unsafe extern "C" fn R_GetTexture(mut texnum: libc::c_uint)
 -> *mut image_t {
    if !(texnum >= 0 as libc::c_int as libc::c_uint &&
             texnum < 4096 as libc::c_int as libc::c_uint) {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 b"../ref_soft/r_image.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 34 as
                                                                     libc::c_int);
    }
    return &mut *r_images.as_mut_ptr().offset(texnum as isize) as
               *mut image_t;
}
/*
=================
GL_Bind
=================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_Bind(mut tmu: libc::c_int,
                                 mut texnum: libc::c_uint) {
    let mut image: *mut image_t = 0 as *mut image_t;
    extern "C" {
        #[no_mangle]
        static mut d_pdrawspans:
               Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
    }
    extern "C" {
        #[link_name = "R_PolysetFillSpans8"]
        fn R_PolysetFillSpans8_0(_: *mut libc::c_void);
    }
    extern "C" {
        #[link_name = "R_PolysetDrawSpansConstant8_33"]
        fn R_PolysetDrawSpansConstant8_33_0(pspanpackage: *mut libc::c_void);
    }
    extern "C" {
        #[link_name = "R_PolysetDrawSpansTextureBlended"]
        fn R_PolysetDrawSpansTextureBlended_0(pspanpackage:
                                                  *mut libc::c_void);
    }
    extern "C" {
        #[link_name = "R_PolysetDrawSpansBlended"]
        fn R_PolysetDrawSpansBlended_0(pspanpackage: *mut libc::c_void);
    }
    extern "C" {
        #[link_name = "R_PolysetDrawSpansAdditive"]
        fn R_PolysetDrawSpansAdditive_0(pspanpackage: *mut libc::c_void);
    }
    extern "C" {
        #[link_name = "R_PolysetDrawSpansGlow"]
        fn R_PolysetDrawSpansGlow_0(pspanpackage: *mut libc::c_void);
    }
    image =
        &mut *r_images.as_mut_ptr().offset(texnum as isize) as *mut image_t;
    //vid.rendermode = kRenderNormal;
    if vid.rendermode == kRenderNormal as libc::c_int {
        r_affinetridesc.pskin =
            (*image).pixels[0 as libc::c_int as usize] as *mut libc::c_void;
        d_pdrawspans =
            Some(R_PolysetFillSpans8_0 as
                     unsafe extern "C" fn(_: *mut libc::c_void) -> ())
    } else if vid.rendermode == kRenderTransAdd as libc::c_int {
        r_affinetridesc.pskin =
            (*image).pixels[0 as libc::c_int as usize] as *mut libc::c_void;
        d_pdrawspans =
            Some(R_PolysetDrawSpansAdditive_0 as
                     unsafe extern "C" fn(_: *mut libc::c_void) -> ())
    } else if vid.rendermode == kRenderGlow as libc::c_int {
        r_affinetridesc.pskin =
            (*image).pixels[0 as libc::c_int as usize] as *mut libc::c_void;
        d_pdrawspans =
            Some(R_PolysetDrawSpansGlow_0 as
                     unsafe extern "C" fn(_: *mut libc::c_void) -> ())
    } else if !(*image).alpha_pixels.is_null() {
        r_affinetridesc.pskin = (*image).alpha_pixels as *mut libc::c_void;
        d_pdrawspans =
            Some(R_PolysetDrawSpansTextureBlended_0 as
                     unsafe extern "C" fn(_: *mut libc::c_void) -> ())
    } else {
        r_affinetridesc.pskin =
            (*image).pixels[0 as libc::c_int as usize] as *mut libc::c_void;
        d_pdrawspans =
            Some(R_PolysetDrawSpansBlended_0 as
                     unsafe extern "C" fn(_: *mut libc::c_void) -> ())
    }
    r_affinetridesc.skinwidth = (*image).width as libc::c_int;
    r_affinetridesc.skinheight = (*image).height as libc::c_int;
}
/*
=================
GL_ApplyTextureParams
=================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_ApplyTextureParams(mut tex: *mut image_t) {
    if tex.is_null() {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 b"../ref_soft/r_image.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 96 as
                                                                     libc::c_int);
    };
}
/*
=================
GL_UpdateTextureParams
=================
*/
unsafe extern "C" fn GL_UpdateTextureParams(mut iTexture: libc::c_int) {
    let mut tex: *mut image_t =
        &mut *r_images.as_mut_ptr().offset(iTexture as isize) as
            *mut image_t; // free slot
    if tex.is_null() {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 b"../ref_soft/r_image.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 108 as
                                                                     libc::c_int);
    }
    if (*tex).pixels.as_mut_ptr().is_null() { return }
    GL_Bind(XASH_TEXTURE0 as libc::c_int, iTexture as libc::c_uint);
}
/*
=================
R_SetTextureParameters
=================
*/
#[no_mangle]
pub unsafe extern "C" fn R_SetTextureParameters() {
    let mut i: libc::c_int = 0;
    // change all the existing mipmapped texture objects
    i = 0 as libc::c_int;
    while (i as libc::c_uint) < r_numImages {
        GL_UpdateTextureParams(i);
        i += 1
    };
}
/*
==================
GL_CalcImageSize
==================
*/
unsafe extern "C" fn GL_CalcImageSize(mut format: pixformat_t,
                                      mut width: libc::c_int,
                                      mut height: libc::c_int,
                                      mut depth: libc::c_int) -> size_t {
    let mut size: size_t = 0 as libc::c_int as size_t;
    // check the depth error
    depth = if 1 as libc::c_int > depth { 1 as libc::c_int } else { depth };
    match format as libc::c_uint {
        5 | 6 => {
            size = (width * height * depth * 3 as libc::c_int) as size_t
        }
        4 | 3 => {
            size = (width * height * depth * 4 as libc::c_int) as size_t
        }
        8 => {
            size =
                ((width + 3 as libc::c_int >> 2 as libc::c_int) *
                     (height + 3 as libc::c_int >> 2 as libc::c_int) *
                     8 as libc::c_int * depth) as size_t
        }
        9 | 10 | 11 => {
            size =
                ((width + 3 as libc::c_int >> 2 as libc::c_int) *
                     (height + 3 as libc::c_int >> 2 as libc::c_int) *
                     16 as libc::c_int * depth) as size_t
        }
        _ => { }
    }
    return size;
}
/*
==================
GL_CalcTextureSize
==================
*/
unsafe extern "C" fn GL_CalcTextureSize(mut width: libc::c_int,
                                        mut height: libc::c_int,
                                        mut depth: libc::c_int) -> size_t {
    return (width * height * 2 as libc::c_int) as size_t;
}
/*
================
GL_SetTextureDimensions
================
*/
unsafe extern "C" fn GL_SetTextureDimensions(mut tex: *mut image_t,
                                             mut width: libc::c_int,
                                             mut height: libc::c_int,
                                             mut depth: libc::c_int) {
    let mut maxTextureSize: libc::c_int = 1024 as libc::c_int;
    let mut maxDepthSize: libc::c_int = 1 as libc::c_int;
    if tex.is_null() {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 b"../ref_soft/r_image.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 211 as
                                                                     libc::c_int);
    }
    // store original sizes
    (*tex).srcWidth = width as word;
    (*tex).srcHeight = height as word;
    if width > maxTextureSize || height > maxTextureSize ||
           depth > maxDepthSize {
        while width > maxTextureSize || height > maxTextureSize {
            width >>= 1 as libc::c_int;
            height >>= 1 as libc::c_int
        }
    }
    // set the texture dimensions
    (*tex).width =
        if 1 as libc::c_int > width { 1 as libc::c_int } else { width } as
            word;
    (*tex).height =
        if 1 as libc::c_int > height { 1 as libc::c_int } else { height } as
            word;
    (*tex).depth =
        if 1 as libc::c_int > depth { 1 as libc::c_int } else { depth } as
            word;
}
/*
===============
GL_SetTextureFormat
===============
*/
unsafe extern "C" fn GL_SetTextureFormat(mut tex: *mut image_t,
                                         mut format: pixformat_t,
                                         mut channelMask: libc::c_int) {
    let mut haveColor: qboolean =
        (channelMask & IMAGE_HAS_COLOR as libc::c_int) as qboolean;
    let mut haveAlpha: qboolean =
        (channelMask & IMAGE_HAS_ALPHA as libc::c_int) as qboolean;
    if tex.is_null() {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 b"../ref_soft/r_image.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 260 as
                                                                     libc::c_int);
    };
    //tex->transparent = !!( channelMask & IMAGE_HAS_ALPHA );
}
/*
=================
GL_ResampleTexture

Assume input buffer is RGBA
=================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_ResampleTexture(mut source: *const byte,
                                            mut inWidth: libc::c_int,
                                            mut inHeight: libc::c_int,
                                            mut outWidth: libc::c_int,
                                            mut outHeight: libc::c_int,
                                            mut isNormalMap: qboolean)
 -> *mut byte {
    let mut frac: uint = 0; // pointer to a scaled image
    let mut fracStep: uint = 0;
    let mut in_0: *mut uint = source as *mut uint;
    let mut p1: [uint; 4096] = [0; 4096];
    let mut p2: [uint; 4096] = [0; 4096];
    let mut pix1: *mut byte = 0 as *mut byte;
    let mut pix2: *mut byte = 0 as *mut byte;
    let mut pix3: *mut byte = 0 as *mut byte;
    let mut pix4: *mut byte = 0 as *mut byte;
    let mut out: *mut uint = 0 as *mut uint;
    let mut inRow1: *mut uint = 0 as *mut uint;
    let mut inRow2: *mut uint = 0 as *mut uint;
    static mut scaledImage: *mut byte = 0 as *const byte as *mut byte;
    let mut normal: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    if source.is_null() { return 0 as *mut byte }
    scaledImage =
        gEngfuncs._Mem_Realloc.expect("non-null function pointer")(r_temppool,
                                                                   scaledImage
                                                                       as
                                                                       *mut libc::c_void,
                                                                   (outWidth *
                                                                        outHeight
                                                                        *
                                                                        4 as
                                                                            libc::c_int)
                                                                       as
                                                                       size_t,
                                                                   true_0,
                                                                   b"../ref_soft/r_image.c\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char,
                                                                   284 as
                                                                       libc::c_int)
            as *mut byte;
    fracStep = (inWidth * 0x10000 as libc::c_int / outWidth) as uint;
    out = scaledImage as *mut uint;
    frac = fracStep >> 2 as libc::c_int;
    i = 0 as libc::c_int;
    while i < outWidth {
        p1[i as usize] =
            (4 as libc::c_int as
                 libc::c_uint).wrapping_mul(frac >> 16 as libc::c_int);
        frac = (frac as libc::c_uint).wrapping_add(fracStep) as uint as uint;
        i += 1
    }
    frac =
        (fracStep >>
             2 as libc::c_int).wrapping_mul(3 as libc::c_int as libc::c_uint);
    i = 0 as libc::c_int;
    while i < outWidth {
        p2[i as usize] =
            (4 as libc::c_int as
                 libc::c_uint).wrapping_mul(frac >> 16 as libc::c_int);
        frac = (frac as libc::c_uint).wrapping_add(fracStep) as uint as uint;
        i += 1
    }
    if isNormalMap as u64 != 0 {
        y = 0 as libc::c_int;
        while y < outHeight {
            inRow1 =
                in_0.offset((inWidth *
                                 ((y as libc::c_float + 0.25f32) *
                                      inHeight as libc::c_float /
                                      outHeight as libc::c_float) as
                                     libc::c_int) as isize);
            inRow2 =
                in_0.offset((inWidth *
                                 ((y as libc::c_float + 0.75f32) *
                                      inHeight as libc::c_float /
                                      outHeight as libc::c_float) as
                                     libc::c_int) as isize);
            x = 0 as libc::c_int;
            while x < outWidth {
                pix1 = (inRow1 as *mut byte).offset(p1[x as usize] as isize);
                pix2 = (inRow1 as *mut byte).offset(p2[x as usize] as isize);
                pix3 = (inRow2 as *mut byte).offset(p1[x as usize] as isize);
                pix4 = (inRow2 as *mut byte).offset(p2[x as usize] as isize);
                normal[0 as libc::c_int as usize] =
                    *pix1.offset(0 as libc::c_int as isize) as libc::c_int as
                        libc::c_float * (1.0f32 / 127.0f32) - 1.0f32 +
                        (*pix2.offset(0 as libc::c_int as isize) as
                             libc::c_int as libc::c_float *
                             (1.0f32 / 127.0f32) - 1.0f32) +
                        (*pix3.offset(0 as libc::c_int as isize) as
                             libc::c_int as libc::c_float *
                             (1.0f32 / 127.0f32) - 1.0f32) +
                        (*pix4.offset(0 as libc::c_int as isize) as
                             libc::c_int as libc::c_float *
                             (1.0f32 / 127.0f32) - 1.0f32);
                normal[1 as libc::c_int as usize] =
                    *pix1.offset(1 as libc::c_int as isize) as libc::c_int as
                        libc::c_float * (1.0f32 / 127.0f32) - 1.0f32 +
                        (*pix2.offset(1 as libc::c_int as isize) as
                             libc::c_int as libc::c_float *
                             (1.0f32 / 127.0f32) - 1.0f32) +
                        (*pix3.offset(1 as libc::c_int as isize) as
                             libc::c_int as libc::c_float *
                             (1.0f32 / 127.0f32) - 1.0f32) +
                        (*pix4.offset(1 as libc::c_int as isize) as
                             libc::c_int as libc::c_float *
                             (1.0f32 / 127.0f32) - 1.0f32);
                normal[2 as libc::c_int as usize] =
                    *pix1.offset(2 as libc::c_int as isize) as libc::c_int as
                        libc::c_float * (1.0f32 / 127.0f32) - 1.0f32 +
                        (*pix2.offset(2 as libc::c_int as isize) as
                             libc::c_int as libc::c_float *
                             (1.0f32 / 127.0f32) - 1.0f32) +
                        (*pix3.offset(2 as libc::c_int as isize) as
                             libc::c_int as libc::c_float *
                             (1.0f32 / 127.0f32) - 1.0f32) +
                        (*pix4.offset(2 as libc::c_int as isize) as
                             libc::c_int as libc::c_float *
                             (1.0f32 / 127.0f32) - 1.0f32);
                if VectorNormalizeLength2(normal.as_mut_ptr() as *const vec_t,
                                          normal.as_mut_ptr()) == 0. {
                    normal[0 as libc::c_int as usize] = 0.5f32;
                    normal[1 as libc::c_int as usize] = 0.5f32;
                    normal[2 as libc::c_int as usize] = 1.0f32
                }
                *(out.offset(x as isize) as
                      *mut byte).offset(0 as libc::c_int as isize) =
                    (128 as libc::c_int +
                         (127.0f32 * normal[0 as libc::c_int as usize]) as
                             byte as libc::c_int) as byte;
                *(out.offset(x as isize) as
                      *mut byte).offset(1 as libc::c_int as isize) =
                    (128 as libc::c_int +
                         (127.0f32 * normal[1 as libc::c_int as usize]) as
                             byte as libc::c_int) as byte;
                *(out.offset(x as isize) as
                      *mut byte).offset(2 as libc::c_int as isize) =
                    (128 as libc::c_int +
                         (127.0f32 * normal[2 as libc::c_int as usize]) as
                             byte as libc::c_int) as byte;
                *(out.offset(x as isize) as
                      *mut byte).offset(3 as libc::c_int as isize) =
                    255 as libc::c_int as byte;
                x += 1
            }
            y += 1;
            out = out.offset(outWidth as isize)
        }
    } else {
        y = 0 as libc::c_int;
        while y < outHeight {
            inRow1 =
                in_0.offset((inWidth *
                                 ((y as libc::c_float + 0.25f32) *
                                      inHeight as libc::c_float /
                                      outHeight as libc::c_float) as
                                     libc::c_int) as isize);
            inRow2 =
                in_0.offset((inWidth *
                                 ((y as libc::c_float + 0.75f32) *
                                      inHeight as libc::c_float /
                                      outHeight as libc::c_float) as
                                     libc::c_int) as isize);
            x = 0 as libc::c_int;
            while x < outWidth {
                pix1 = (inRow1 as *mut byte).offset(p1[x as usize] as isize);
                pix2 = (inRow1 as *mut byte).offset(p2[x as usize] as isize);
                pix3 = (inRow2 as *mut byte).offset(p1[x as usize] as isize);
                pix4 = (inRow2 as *mut byte).offset(p2[x as usize] as isize);
                *(out.offset(x as isize) as
                      *mut byte).offset(0 as libc::c_int as isize) =
                    (*pix1.offset(0 as libc::c_int as isize) as libc::c_int +
                         *pix2.offset(0 as libc::c_int as isize) as
                             libc::c_int +
                         *pix3.offset(0 as libc::c_int as isize) as
                             libc::c_int +
                         *pix4.offset(0 as libc::c_int as isize) as
                             libc::c_int >> 2 as libc::c_int) as byte;
                *(out.offset(x as isize) as
                      *mut byte).offset(1 as libc::c_int as isize) =
                    (*pix1.offset(1 as libc::c_int as isize) as libc::c_int +
                         *pix2.offset(1 as libc::c_int as isize) as
                             libc::c_int +
                         *pix3.offset(1 as libc::c_int as isize) as
                             libc::c_int +
                         *pix4.offset(1 as libc::c_int as isize) as
                             libc::c_int >> 2 as libc::c_int) as byte;
                *(out.offset(x as isize) as
                      *mut byte).offset(2 as libc::c_int as isize) =
                    (*pix1.offset(2 as libc::c_int as isize) as libc::c_int +
                         *pix2.offset(2 as libc::c_int as isize) as
                             libc::c_int +
                         *pix3.offset(2 as libc::c_int as isize) as
                             libc::c_int +
                         *pix4.offset(2 as libc::c_int as isize) as
                             libc::c_int >> 2 as libc::c_int) as byte;
                *(out.offset(x as isize) as
                      *mut byte).offset(3 as libc::c_int as isize) =
                    (*pix1.offset(3 as libc::c_int as isize) as libc::c_int +
                         *pix2.offset(3 as libc::c_int as isize) as
                             libc::c_int +
                         *pix3.offset(3 as libc::c_int as isize) as
                             libc::c_int +
                         *pix4.offset(3 as libc::c_int as isize) as
                             libc::c_int >> 2 as libc::c_int) as byte;
                x += 1
            }
            y += 1;
            out = out.offset(outWidth as isize)
        }
    }
    return scaledImage;
}
/*
=================
GL_BoxFilter3x3

box filter 3x3
=================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_BoxFilter3x3(mut out: *mut byte,
                                         mut in_0: *const byte,
                                         mut w: libc::c_int,
                                         mut h: libc::c_int,
                                         mut x: libc::c_int,
                                         mut y: libc::c_int) {
    let mut r: libc::c_int = 0 as libc::c_int;
    let mut g: libc::c_int = 0 as libc::c_int;
    let mut b: libc::c_int = 0 as libc::c_int;
    let mut a: libc::c_int = 0 as libc::c_int;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut acount: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut u: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut pixel: *const byte = 0 as *const byte;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        u = i - 1 as libc::c_int + x;
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            v = j - 1 as libc::c_int + y;
            if u >= 0 as libc::c_int && u < w && v >= 0 as libc::c_int &&
                   v < h {
                pixel =
                    &*in_0.offset(((u + v * w) * 4 as libc::c_int) as isize)
                        as *const byte;
                if *pixel.offset(3 as libc::c_int as isize) as libc::c_int !=
                       0 as libc::c_int {
                    r +=
                        *pixel.offset(0 as libc::c_int as isize) as
                            libc::c_int;
                    g +=
                        *pixel.offset(1 as libc::c_int as isize) as
                            libc::c_int;
                    b +=
                        *pixel.offset(2 as libc::c_int as isize) as
                            libc::c_int;
                    a +=
                        *pixel.offset(3 as libc::c_int as isize) as
                            libc::c_int;
                    acount += 1
                }
            }
            j += 1
        }
        i += 1
    }
    if acount == 0 as libc::c_int { acount = 1 as libc::c_int }
    *out.offset(0 as libc::c_int as isize) = (r / acount) as byte;
    *out.offset(1 as libc::c_int as isize) = (g / acount) as byte;
    *out.offset(2 as libc::c_int as isize) = (b / acount) as byte;
    //	out[3] = (int)( SimpleSpline( ( a / 12.0f ) / 255.0f ) * 255 );
}
/*
=================
GL_ApplyFilter

Apply box-filter to 1-bit alpha
=================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_ApplyFilter(mut source: *const byte,
                                        mut width: libc::c_int,
                                        mut height: libc::c_int)
 -> *mut byte {
    let mut in_0: *mut byte = source as *mut byte;
    let mut out: *mut byte = source as *mut byte;
    let mut i: libc::c_int = 0;
    if Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(PARM_QUAKE_COMPATIBLE
                                                                                                                 as
                                                                                                                 libc::c_int,
                                                                                                             0
                                                                                                                 as
                                                                                                                 libc::c_int)
           != 0 {
        return in_0
    }
    i = 0 as libc::c_int;
    while !source.is_null() && i < width * height {
        if *in_0.offset(0 as libc::c_int as isize) as libc::c_int ==
               0 as libc::c_int &&
               *in_0.offset(1 as libc::c_int as isize) as libc::c_int ==
                   0 as libc::c_int &&
               *in_0.offset(2 as libc::c_int as isize) as libc::c_int ==
                   0 as libc::c_int &&
               *in_0.offset(3 as libc::c_int as isize) as libc::c_int ==
                   0 as libc::c_int {
            GL_BoxFilter3x3(in_0, source, width, height, i % width,
                            i / width);
        }
        i += 1;
        in_0 = in_0.offset(4 as libc::c_int as isize)
    }
    return out;
}
/*
=================
GL_BuildMipMap

Operates in place, quartering the size of the texture
=================
*/
unsafe extern "C" fn GL_BuildMipMap(mut in_0: *mut byte,
                                    mut srcWidth: libc::c_int,
                                    mut srcHeight: libc::c_int,
                                    mut srcDepth: libc::c_int,
                                    mut flags: libc::c_int) {
    let mut out: *mut byte = in_0;
    let mut instride: libc::c_int =
        (((srcWidth * 4 as libc::c_int) as
              libc::c_ulong).wrapping_add((1 as libc::c_int as
                                               size_t).wrapping_sub(1 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong))
             &
             !(1 as libc::c_int as
                   size_t).wrapping_sub(1 as libc::c_int as libc::c_ulong)) as
            libc::c_int;
    let mut mipWidth: libc::c_int = 0;
    let mut mipHeight: libc::c_int = 0;
    let mut outpadding: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    let mut normal: vec3_t = [0.; 3];
    if in_0.is_null() { return }
    mipWidth =
        if 1 as libc::c_int > srcWidth >> 1 as libc::c_int {
            1 as libc::c_int
        } else { (srcWidth) >> 1 as libc::c_int };
    mipHeight =
        if 1 as libc::c_int > srcHeight >> 1 as libc::c_int {
            1 as libc::c_int
        } else { (srcHeight) >> 1 as libc::c_int };
    outpadding =
        (((mipWidth * 4 as libc::c_int) as
              libc::c_ulong).wrapping_add((1 as libc::c_int as
                                               size_t).wrapping_sub(1 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong))
             &
             !(1 as libc::c_int as
                   size_t).wrapping_sub(1 as libc::c_int as
                                            libc::c_ulong)).wrapping_sub((mipWidth
                                                                              *
                                                                              4
                                                                                  as
                                                                                  libc::c_int)
                                                                             as
                                                                             libc::c_ulong)
            as libc::c_int;
    row = srcWidth << 2 as libc::c_int;
    if flags & TF_ALPHACONTRAST as libc::c_int != 0 {
        memset(in_0 as *mut libc::c_void, mipWidth,
               (mipWidth * mipHeight * 4 as libc::c_int) as libc::c_ulong);
        return
    }
    // move through all layers
    z = 0 as libc::c_int;
    while z < srcDepth {
        if flags & TF_NORMALMAP as libc::c_int != 0 {
            y = 0 as libc::c_int;
            while y < mipHeight {
                let mut next: *mut byte =
                    if ((y << 1 as libc::c_int) + 1 as libc::c_int) <
                           srcHeight {
                        in_0.offset(instride as isize)
                    } else { in_0 };
                x = 0 as libc::c_int;
                row = 0 as libc::c_int;
                while x < mipWidth {
                    if ((x << 1 as libc::c_int) + 1 as libc::c_int) < srcWidth
                       {
                        normal[0 as libc::c_int as usize] =
                            *in_0.offset((row + 0 as libc::c_int) as isize) as
                                libc::c_int as libc::c_float *
                                (1.0f32 / 127.0f32) - 1.0f32 +
                                (*in_0.offset((row + 4 as libc::c_int) as
                                                  isize) as libc::c_int as
                                     libc::c_float * (1.0f32 / 127.0f32) -
                                     1.0f32) +
                                (*next.offset((row + 0 as libc::c_int) as
                                                  isize) as libc::c_int as
                                     libc::c_float * (1.0f32 / 127.0f32) -
                                     1.0f32) +
                                (*next.offset((row + 4 as libc::c_int) as
                                                  isize) as libc::c_int as
                                     libc::c_float * (1.0f32 / 127.0f32) -
                                     1.0f32);
                        normal[1 as libc::c_int as usize] =
                            *in_0.offset((row + 1 as libc::c_int) as isize) as
                                libc::c_int as libc::c_float *
                                (1.0f32 / 127.0f32) - 1.0f32 +
                                (*in_0.offset((row + 5 as libc::c_int) as
                                                  isize) as libc::c_int as
                                     libc::c_float * (1.0f32 / 127.0f32) -
                                     1.0f32) +
                                (*next.offset((row + 1 as libc::c_int) as
                                                  isize) as libc::c_int as
                                     libc::c_float * (1.0f32 / 127.0f32) -
                                     1.0f32) +
                                (*next.offset((row + 5 as libc::c_int) as
                                                  isize) as libc::c_int as
                                     libc::c_float * (1.0f32 / 127.0f32) -
                                     1.0f32);
                        normal[2 as libc::c_int as usize] =
                            *in_0.offset((row + 2 as libc::c_int) as isize) as
                                libc::c_int as libc::c_float *
                                (1.0f32 / 127.0f32) - 1.0f32 +
                                (*in_0.offset((row + 6 as libc::c_int) as
                                                  isize) as libc::c_int as
                                     libc::c_float * (1.0f32 / 127.0f32) -
                                     1.0f32) +
                                (*next.offset((row + 2 as libc::c_int) as
                                                  isize) as libc::c_int as
                                     libc::c_float * (1.0f32 / 127.0f32) -
                                     1.0f32) +
                                (*next.offset((row + 6 as libc::c_int) as
                                                  isize) as libc::c_int as
                                     libc::c_float * (1.0f32 / 127.0f32) -
                                     1.0f32)
                    } else {
                        normal[0 as libc::c_int as usize] =
                            *in_0.offset((row + 0 as libc::c_int) as isize) as
                                libc::c_int as libc::c_float *
                                (1.0f32 / 127.0f32) - 1.0f32 +
                                (*next.offset((row + 0 as libc::c_int) as
                                                  isize) as libc::c_int as
                                     libc::c_float * (1.0f32 / 127.0f32) -
                                     1.0f32);
                        normal[1 as libc::c_int as usize] =
                            *in_0.offset((row + 1 as libc::c_int) as isize) as
                                libc::c_int as libc::c_float *
                                (1.0f32 / 127.0f32) - 1.0f32 +
                                (*next.offset((row + 1 as libc::c_int) as
                                                  isize) as libc::c_int as
                                     libc::c_float * (1.0f32 / 127.0f32) -
                                     1.0f32);
                        normal[2 as libc::c_int as usize] =
                            *in_0.offset((row + 2 as libc::c_int) as isize) as
                                libc::c_int as libc::c_float *
                                (1.0f32 / 127.0f32) - 1.0f32 +
                                (*next.offset((row + 2 as libc::c_int) as
                                                  isize) as libc::c_int as
                                     libc::c_float * (1.0f32 / 127.0f32) -
                                     1.0f32)
                    }
                    if VectorNormalizeLength2(normal.as_mut_ptr() as
                                                  *const vec_t,
                                              normal.as_mut_ptr()) == 0. {
                        normal[0 as libc::c_int as usize] = 0.5f32;
                        normal[1 as libc::c_int as usize] = 0.5f32;
                        normal[2 as libc::c_int as usize] = 1.0f32
                    }
                    *out.offset(0 as libc::c_int as isize) =
                        (128 as libc::c_int +
                             (127.0f32 * normal[0 as libc::c_int as usize]) as
                                 byte as libc::c_int) as byte;
                    *out.offset(1 as libc::c_int as isize) =
                        (128 as libc::c_int +
                             (127.0f32 * normal[1 as libc::c_int as usize]) as
                                 byte as libc::c_int) as byte;
                    *out.offset(2 as libc::c_int as isize) =
                        (128 as libc::c_int +
                             (127.0f32 * normal[2 as libc::c_int as usize]) as
                                 byte as libc::c_int) as byte;
                    *out.offset(3 as libc::c_int as isize) =
                        255 as libc::c_int as byte;
                    x += 1;
                    row += 8 as libc::c_int;
                    out = out.offset(4 as libc::c_int as isize)
                }
                y += 1;
                in_0 = in_0.offset((instride * 2 as libc::c_int) as isize);
                out = out.offset(outpadding as isize)
            }
        } else {
            y = 0 as libc::c_int;
            while y < mipHeight {
                let mut next_0: *mut byte =
                    if ((y << 1 as libc::c_int) + 1 as libc::c_int) <
                           srcHeight {
                        in_0.offset(instride as isize)
                    } else { in_0 };
                x = 0 as libc::c_int;
                row = 0 as libc::c_int;
                while x < mipWidth {
                    if ((x << 1 as libc::c_int) + 1 as libc::c_int) < srcWidth
                       {
                        *out.offset(0 as libc::c_int as isize) =
                            (*in_0.offset((row + 0 as libc::c_int) as isize)
                                 as libc::c_int +
                                 *in_0.offset((row + 4 as libc::c_int) as
                                                  isize) as libc::c_int +
                                 *next_0.offset((row + 0 as libc::c_int) as
                                                    isize) as libc::c_int +
                                 *next_0.offset((row + 4 as libc::c_int) as
                                                    isize) as libc::c_int >>
                                 2 as libc::c_int) as byte;
                        *out.offset(1 as libc::c_int as isize) =
                            (*in_0.offset((row + 1 as libc::c_int) as isize)
                                 as libc::c_int +
                                 *in_0.offset((row + 5 as libc::c_int) as
                                                  isize) as libc::c_int +
                                 *next_0.offset((row + 1 as libc::c_int) as
                                                    isize) as libc::c_int +
                                 *next_0.offset((row + 5 as libc::c_int) as
                                                    isize) as libc::c_int >>
                                 2 as libc::c_int) as byte;
                        *out.offset(2 as libc::c_int as isize) =
                            (*in_0.offset((row + 2 as libc::c_int) as isize)
                                 as libc::c_int +
                                 *in_0.offset((row + 6 as libc::c_int) as
                                                  isize) as libc::c_int +
                                 *next_0.offset((row + 2 as libc::c_int) as
                                                    isize) as libc::c_int +
                                 *next_0.offset((row + 6 as libc::c_int) as
                                                    isize) as libc::c_int >>
                                 2 as libc::c_int) as byte;
                        *out.offset(3 as libc::c_int as isize) =
                            (*in_0.offset((row + 3 as libc::c_int) as isize)
                                 as libc::c_int +
                                 *in_0.offset((row + 7 as libc::c_int) as
                                                  isize) as libc::c_int +
                                 *next_0.offset((row + 3 as libc::c_int) as
                                                    isize) as libc::c_int +
                                 *next_0.offset((row + 7 as libc::c_int) as
                                                    isize) as libc::c_int >>
                                 2 as libc::c_int) as byte
                    } else {
                        *out.offset(0 as libc::c_int as isize) =
                            (*in_0.offset((row + 0 as libc::c_int) as isize)
                                 as libc::c_int +
                                 *next_0.offset((row + 0 as libc::c_int) as
                                                    isize) as libc::c_int >>
                                 1 as libc::c_int) as byte;
                        *out.offset(1 as libc::c_int as isize) =
                            (*in_0.offset((row + 1 as libc::c_int) as isize)
                                 as libc::c_int +
                                 *next_0.offset((row + 1 as libc::c_int) as
                                                    isize) as libc::c_int >>
                                 1 as libc::c_int) as byte;
                        *out.offset(2 as libc::c_int as isize) =
                            (*in_0.offset((row + 2 as libc::c_int) as isize)
                                 as libc::c_int +
                                 *next_0.offset((row + 2 as libc::c_int) as
                                                    isize) as libc::c_int >>
                                 1 as libc::c_int) as byte;
                        *out.offset(3 as libc::c_int as isize) =
                            (*in_0.offset((row + 3 as libc::c_int) as isize)
                                 as libc::c_int +
                                 *next_0.offset((row + 3 as libc::c_int) as
                                                    isize) as libc::c_int >>
                                 1 as libc::c_int) as byte
                    }
                    x += 1;
                    row += 8 as libc::c_int;
                    out = out.offset(4 as libc::c_int as isize)
                }
                y += 1;
                in_0 = in_0.offset((instride * 2 as libc::c_int) as isize);
                out = out.offset(outpadding as isize)
            }
        }
        z += 1
    };
}
/*
===============
GL_UploadTexture

upload texture into video memory
===============
*/
unsafe extern "C" fn GL_UploadTexture(mut tex: *mut image_t,
                                      mut pic: *mut rgbdata_t) -> qboolean {
    let mut buf: *mut byte = 0 as *mut byte;
    let mut data: *mut byte = 0 as *mut byte;
    let mut texsize: size_t = 0;
    let mut size: size_t = 0;
    let mut width: uint = 0;
    let mut height: uint = 0;
    let mut i: uint = 0;
    let mut j: uint = 0;
    let mut numSides: uint = 0;
    let mut offset: uint = 0 as libc::c_int as uint;
    let mut normalMap: qboolean = false_0;
    let mut bufend: *const byte = 0 as *const byte;
    let mut mipCount: libc::c_int = 0;
    (*tex).fogParams[0 as libc::c_int as usize] =
        (*pic).fogParams[0 as libc::c_int as usize];
    (*tex).fogParams[1 as libc::c_int as usize] =
        (*pic).fogParams[1 as libc::c_int as usize];
    (*tex).fogParams[2 as libc::c_int as usize] =
        (*pic).fogParams[2 as libc::c_int as usize];
    (*tex).fogParams[3 as libc::c_int as usize] =
        (*pic).fogParams[3 as libc::c_int as usize];
    GL_SetTextureDimensions(tex, (*pic).width as libc::c_int,
                            (*pic).height as libc::c_int,
                            (*pic).depth as libc::c_int);
    GL_SetTextureFormat(tex, (*pic).type_0 as pixformat_t,
                        (*pic).flags as libc::c_int);
    //gEngfuncs.Con_Printf("%s %d %d\n", tex->name, tex->width, tex->height );
    if pic.is_null() {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 b"../ref_soft/r_image.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 545 as
                                                                     libc::c_int); //GL_CalcMipmapCount( tex, ( buf != NULL ));
    }
    if tex.is_null() {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 b"../ref_soft/r_image.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 546 as
                                                                     libc::c_int);
    }
    if (*pic).buffer.is_null() { return true_0 }
    buf = (*pic).buffer;
    mipCount = 4 as libc::c_int;
    // NOTE: only single uncompressed textures can be resamples, no mips, no layers, no sides
    if (*pic).width as libc::c_int != (*tex).width as libc::c_int ||
           (*pic).height as libc::c_int != (*tex).height as libc::c_int {
        data =
            GL_ResampleTexture(buf, (*pic).width as libc::c_int,
                               (*pic).height as libc::c_int,
                               (*tex).width as libc::c_int,
                               (*tex).height as libc::c_int, normalMap)
    } else { data = buf }
    //if( !ImageDXT( pic->type ) && !FBitSet( tex->flags, TF_NOMIPMAP ) && FBitSet( pic->flags, IMAGE_ONEBIT_ALPHA ))
	//	data = GL_ApplyFilter( data, tex->width, tex->height );
    // mips will be auto-generated if desired
    j = 0 as libc::c_int as uint;
    while j < mipCount as libc::c_uint {
        let mut x: libc::c_int = 0;
        let mut y: libc::c_int = 0;
        width =
            if 1 as libc::c_int > (*tex).width as libc::c_int >> j {
                1 as libc::c_int
            } else { ((*tex).width as libc::c_int) >> j } as uint;
        height =
            if 1 as libc::c_int > (*tex).height as libc::c_int >> j {
                1 as libc::c_int
            } else { ((*tex).height as libc::c_int) >> j } as uint;
        texsize =
            GL_CalcTextureSize(width as libc::c_int, height as libc::c_int,
                               (*tex).depth as libc::c_int);
        size =
            GL_CalcImageSize((*pic).type_0 as pixformat_t,
                             width as libc::c_int, height as libc::c_int,
                             (*tex).depth as libc::c_int);
        //GL_CheckTexImageError( tex );
        (*tex).pixels[j as usize] =
            gEngfuncs._Mem_Alloc.expect("non-null function pointer")(r_temppool,
                                                                     (width.wrapping_mul(height)
                                                                          as
                                                                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<pixel_t>()
                                                                                                          as
                                                                                                          libc::c_ulong),
                                                                     true_0,
                                                                     b"../ref_soft/r_image.c\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     575 as
                                                                         libc::c_int)
                as *mut pixel_t;
        if j == 0 as libc::c_int as libc::c_uint &&
               (*tex).flags as libc::c_uint &
                   TF_HAS_ALPHA as libc::c_int as libc::c_uint != 0 {
            (*tex).alpha_pixels =
                gEngfuncs._Mem_Alloc.expect("non-null function pointer")(r_temppool,
                                                                         (width.wrapping_mul(height)
                                                                              as
                                                                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<pixel_t>()
                                                                                                              as
                                                                                                              libc::c_ulong),
                                                                         true_0,
                                                                         b"../ref_soft/r_image.c\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char,
                                                                         582
                                                                             as
                                                                             libc::c_int)
                    as *mut pixel_t
        }
        i = 0 as libc::c_int as uint;
        while i < height.wrapping_mul(width) {
            let mut r: libc::c_uint = 0;
            let mut g: libc::c_uint = 0;
            let mut b: libc::c_uint = 0;
            let mut major: libc::c_uint = 0;
            let mut minor: libc::c_uint = 0;
            //GL_TextureImageRAW( tex, i, j, width, height, tex->depth, pic->type, data );
		// increase size to workaround triangle renderer bugs
		// it seems to assume memory readable. maybe it was pointed to WAD?
		//tex->pixels[j] = (byte*)Mem_Calloc( r_temppool, width * height * sizeof(pixel_t) + 1024 ) + 512;
            //memset( (byte*)tex->pixels[j] - 512, 0xFF, 512 );
		//memset( (byte*)tex->pixels[j] + width * height * sizeof(pixel_t), 0xFF, 512 );
            // seems to look better
            r =
                (*data.offset(i.wrapping_mul(4 as libc::c_int as
                                                 libc::c_uint).wrapping_add(0
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uint)
                                  as isize) as
                     libc::c_uint).wrapping_mul((1 as libc::c_uint) <<
                                                    5 as
                                                        libc::c_int).wrapping_div(256
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_uint);
            g =
                (*data.offset(i.wrapping_mul(4 as libc::c_int as
                                                 libc::c_uint).wrapping_add(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uint)
                                  as isize) as
                     libc::c_uint).wrapping_mul((1 as libc::c_uint) <<
                                                    6 as
                                                        libc::c_int).wrapping_div(256
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_uint);
            b =
                (*data.offset(i.wrapping_mul(4 as libc::c_int as
                                                 libc::c_uint).wrapping_add(2
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uint)
                                  as isize) as
                     libc::c_uint).wrapping_mul((1 as libc::c_uint) <<
                                                    5 as
                                                        libc::c_int).wrapping_div(256
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_uint);
            // 565 to 332
            major =
                (r >> 2 as libc::c_int &
                     ((1 as libc::c_uint) <<
                          3 as
                              libc::c_int).wrapping_sub(1 as libc::c_int as
                                                            libc::c_uint)) <<
                    5 as libc::c_int |
                    (g >> 3 as libc::c_int &
                         ((1 as libc::c_uint) <<
                              3 as
                                  libc::c_int).wrapping_sub(1 as libc::c_int
                                                                as
                                                                libc::c_uint))
                        << 2 as libc::c_int |
                    b >> 3 as libc::c_int &
                        ((1 as libc::c_uint) <<
                             2 as
                                 libc::c_int).wrapping_sub(1 as libc::c_int as
                                                               libc::c_uint);
            // save minor GBRGBRGB
            minor =
                ((r &
                      ((1 as libc::c_int) << 1 as libc::c_int) as
                          libc::c_uint) >> 1 as libc::c_int) <<
                    5 as libc::c_int |
                    ((r &
                          ((1 as libc::c_int) << 0 as libc::c_int) as
                              libc::c_uint) >> 0 as libc::c_int) <<
                        2 as libc::c_int |
                    ((g &
                          ((1 as libc::c_int) << 2 as libc::c_int) as
                              libc::c_uint) >> 2 as libc::c_int) <<
                        7 as libc::c_int |
                    ((g &
                          ((1 as libc::c_int) << 1 as libc::c_int) as
                              libc::c_uint) >> 1 as libc::c_int) <<
                        4 as libc::c_int |
                    ((g &
                          ((1 as libc::c_int) << 0 as libc::c_int) as
                              libc::c_uint) >> 0 as libc::c_int) <<
                        1 as libc::c_int |
                    ((b &
                          ((1 as libc::c_int) << 2 as libc::c_int) as
                              libc::c_uint) >> 2 as libc::c_int) <<
                        6 as libc::c_int |
                    ((b &
                          ((1 as libc::c_int) << 1 as libc::c_int) as
                              libc::c_uint) >> 1 as libc::c_int) <<
                        3 as libc::c_int |
                    ((b &
                          ((1 as libc::c_int) << 0 as libc::c_int) as
                              libc::c_uint) >> 0 as libc::c_int) <<
                        0 as libc::c_int;
            *(*tex).pixels[j as usize].offset(i as isize) =
                (major << 8 as libc::c_int |
                     minor & 0xff as libc::c_int as libc::c_uint) as pixel_t;
            if j == 0 as libc::c_int as libc::c_uint &&
                   !(*tex).alpha_pixels.is_null() {
                let mut alpha: libc::c_uint =
                    ((*data.offset(i.wrapping_mul(4 as libc::c_int as
                                                      libc::c_uint).wrapping_add(3
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_uint)
                                       as isize) as libc::c_int *
                          8 as libc::c_int / 256 as libc::c_int) <<
                         16 as libc::c_int - 3 as libc::c_int) as
                        libc::c_uint;
                *(*tex).alpha_pixels.offset(i as isize) =
                    ((*(*tex).pixels[j as usize].offset(i as isize) as
                          libc::c_int >> 3 as libc::c_int) as libc::c_uint |
                         alpha) as pixel_t;
                if (*sw_noalphabrushes).value == 0. &&
                       (*data.offset(i.wrapping_mul(4 as libc::c_int as
                                                        libc::c_uint).wrapping_add(3
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       libc::c_uint)
                                         as isize) as libc::c_int) <
                           128 as libc::c_int &&
                       (*pic).flags &
                           IMAGE_ONEBIT_ALPHA as libc::c_int as libc::c_uint
                           != 0 {
                    *(*tex).pixels[j as usize].offset(i as isize) =
                        0x349 as libc::c_int as pixel_t
                }
                //0000 0011 0100 1001;
            }
            i = i.wrapping_add(1)
        }
        if mipCount > 1 as libc::c_int {
            GL_BuildMipMap(data, width as libc::c_int, height as libc::c_int,
                           (*tex).depth as libc::c_int,
                           (*tex).flags as libc::c_int);
        }
        (*tex).size =
            ((*tex).size as libc::c_ulong).wrapping_add(texsize) as size_t as
                size_t;
        (*tex).numMips = (*tex).numMips.wrapping_add(1);
        j = j.wrapping_add(1)
    }
    return true_0;
}
/*
===============
GL_ProcessImage

do specified actions on pixels
===============
*/
unsafe extern "C" fn GL_ProcessImage(mut tex: *mut image_t,
                                     mut pic: *mut rgbdata_t) {
    let mut img_flags: uint = 0 as libc::c_int as uint;
    // force upload texture as RGB or RGBA (detail textures requires this)
    if (*tex).flags as libc::c_uint &
           TF_FORCE_COLOR as libc::c_int as libc::c_uint != 0 {
        (*pic).flags |= IMAGE_HAS_COLOR as libc::c_int as libc::c_uint
    } // disable mipmapping by user request
    if (*pic).flags & IMAGE_HAS_ALPHA as libc::c_int as libc::c_uint != 0 {
        (*tex).flags =
            ::std::mem::transmute::<libc::c_uint,
                                    texFlags_t>((*tex).flags as libc::c_uint |
                                                    TF_HAS_ALPHA as
                                                        libc::c_int as
                                                        libc::c_uint)
    }
    if (*pic).type_0 == PF_DXT1 as libc::c_int as libc::c_uint ||
           (*pic).type_0 == PF_DXT3 as libc::c_int as libc::c_uint ||
           (*pic).type_0 == PF_DXT5 as libc::c_int as libc::c_uint ||
           (*pic).type_0 == PF_ATI2 as libc::c_int as libc::c_uint {
        if (*pic).numMips == 0 {
            (*tex).flags =
                ::std::mem::transmute::<libc::c_uint,
                                        texFlags_t>((*tex).flags as
                                                        libc::c_uint |
                                                        TF_NOMIPMAP as
                                                            libc::c_int as
                                                            libc::c_uint)
        }
        // clear all the unsupported flags
        (*tex).flags =
            ::std::mem::transmute::<libc::c_uint,
                                    texFlags_t>((*tex).flags as libc::c_uint &
                                                    !(TF_KEEP_SOURCE as
                                                          libc::c_int) as
                                                        libc::c_uint)
    } else {
        // copy flag about luma pixels
        if (*pic).flags & IMAGE_HAS_LUMA as libc::c_int as libc::c_uint != 0 {
            (*tex).flags =
                ::std::mem::transmute::<libc::c_uint,
                                        texFlags_t>((*tex).flags as
                                                        libc::c_uint |
                                                        TF_HAS_LUMA as
                                                            libc::c_int as
                                                            libc::c_uint)
        }
        if (*pic).flags & IMAGE_QUAKEPAL as libc::c_int as libc::c_uint != 0 {
            (*tex).flags =
                ::std::mem::transmute::<libc::c_uint,
                                        texFlags_t>((*tex).flags as
                                                        libc::c_uint |
                                                        TF_QUAKEPAL as
                                                            libc::c_int as
                                                            libc::c_uint)
        }
        // create luma texture from quake texture
        if (*tex).flags as libc::c_uint &
               TF_MAKELUMA as libc::c_int as libc::c_uint != 0 {
            img_flags |=
                IMAGE_MAKE_LUMA as libc::c_int as
                    libc::c_uint; // because current pic will be expanded to rgba
            (*tex).flags =
                ::std::mem::transmute::<libc::c_uint,
                                        texFlags_t>((*tex).flags as
                                                        libc::c_uint &
                                                        !(TF_MAKELUMA as
                                                              libc::c_int) as
                                                            libc::c_uint)
        }
        if (*tex).flags as libc::c_uint &
               TF_ALLOW_EMBOSS as libc::c_int as libc::c_uint != 0 {
            img_flags |= IMAGE_EMBOSS as libc::c_int as libc::c_uint;
            (*tex).flags =
                ::std::mem::transmute::<libc::c_uint,
                                        texFlags_t>((*tex).flags as
                                                        libc::c_uint &
                                                        !(TF_ALLOW_EMBOSS as
                                                              libc::c_int) as
                                                            libc::c_uint)
        }
        if (*tex).flags as libc::c_uint &
               TF_IMG_UPLOADED as libc::c_int as libc::c_uint == 0 &&
               (*tex).flags as libc::c_uint &
                   TF_KEEP_SOURCE as libc::c_int as libc::c_uint != 0 {
            (*tex).original =
                gEngfuncs.FS_CopyImage.expect("non-null function pointer")(pic)
        }
        // we need to expand image into RGBA buffer
        if (*pic).type_0 == PF_INDEXED_24 as libc::c_int as libc::c_uint ||
               (*pic).type_0 == PF_INDEXED_32 as libc::c_int as libc::c_uint {
            img_flags |= IMAGE_FORCE_RGBA as libc::c_int as libc::c_uint
        }
        // processing image before uploading (force to rgba, make luma etc)
        if !(*pic).buffer.is_null() {
            gEngfuncs.Image_Process.expect("non-null function pointer")(&mut pic,
                                                                        0 as
                                                                            libc::c_int,
                                                                        0 as
                                                                            libc::c_int,
                                                                        img_flags,
                                                                        (*gl_emboss_scale).value);
        }
        if (*tex).flags as libc::c_uint &
               TF_LUMINANCE as libc::c_int as libc::c_uint != 0 {
            (*pic).flags =
                (*pic).flags &
                    !(IMAGE_HAS_COLOR as libc::c_int) as libc::c_uint
        }
    };
}
/*
================
GL_CheckTexName
================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_CheckTexName(mut name: *const libc::c_char)
 -> qboolean {
    let mut len: libc::c_int = 0;
    if if name.is_null() || *name == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return false_0
    }
    len = Q_strlen(name) as libc::c_int;
    // because multi-layered textures can exceed name string
    if len as libc::c_ulong >=
           ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong {
        gEngfuncs.Con_Printf.expect("non-null function pointer")(b"^1Error:^7 LoadTexture: too long name %s (%d)\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 name, len);
        return false_0
    }
    return true_0;
}
/*
================
GL_TextureForName
================
*/
unsafe extern "C" fn GL_TextureForName(mut name: *const libc::c_char)
 -> *mut image_t {
    let mut tex: *mut image_t = 0 as *mut image_t;
    let mut hash: uint = 0;
    // find the texture in array
    hash =
        COM_HashKey(name, (4096 as libc::c_int >> 2 as libc::c_int) as uint);
    tex = r_imagesHashTable[hash as usize];
    while !tex.is_null() {
        if Q_strnicmp((*tex).name.as_mut_ptr(), name, 99999 as libc::c_int) ==
               0 {
            return tex
        }
        tex = (*tex).nextHash
    }
    return 0 as *mut image_t;
}
/*
================
GL_AllocTexture
================
*/
unsafe extern "C" fn GL_AllocTexture(mut name: *const libc::c_char,
                                     mut flags: texFlags_t) -> *mut image_t {
    let mut tex: *mut image_t = 0 as *mut image_t;
    let mut i: uint = 0;
    // find a free texture_t slot
    i = 0 as libc::c_int as uint;
    tex = r_images.as_mut_ptr();
    while i < r_numImages {
        if (*tex).name[0 as libc::c_int as usize] == 0 { break ; }
        i = i.wrapping_add(1);
        tex = tex.offset(1)
    }
    if i == r_numImages {
        if r_numImages == 4096 as libc::c_int as libc::c_uint {
            gEngfuncs.Host_Error.expect("non-null function pointer")(b"GL_AllocTexture: MAX_TEXTURES limit exceeds\n\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char);
        }
        r_numImages = r_numImages.wrapping_add(1)
    }
    tex = &mut *r_images.as_mut_ptr().offset(i as isize) as *mut image_t;
    // copy initial params
    Q_strncpy((*tex).name.as_mut_ptr(), name,
              ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong);
    //tex->texnum = i; // texnum is used for fast acess into gl_textures array too
    (*tex).flags = flags;
    // add to hash table
    (*tex).hashValue =
        COM_HashKey(name, (4096 as libc::c_int >> 2 as libc::c_int) as uint);
    (*tex).nextHash = r_imagesHashTable[(*tex).hashValue as usize];
    r_imagesHashTable[(*tex).hashValue as usize] = tex;
    return tex;
}
/*
================
GL_DeleteTexture
================
*/
unsafe extern "C" fn GL_DeleteTexture(mut tex: *mut image_t) {
    let mut prev: *mut *mut image_t = 0 as *mut *mut image_t;
    let mut cur: *mut image_t = 0 as *mut image_t;
    let mut i: libc::c_int = 0;
    if tex.is_null() {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 b"../ref_soft/r_image.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 895 as
                                                                     libc::c_int);
    }
    // already freed?
    if (*tex).pixels[0 as libc::c_int as usize].is_null() { return }
    // debug
    if (*tex).name[0 as libc::c_int as usize] == 0 {
        gEngfuncs.Con_Printf.expect("non-null function pointer")(b"^1Error:^7 GL_DeleteTexture: trying to free unnamed texture\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char);
        return
    }
    // remove from hash table
    prev =
        &mut *r_imagesHashTable.as_mut_ptr().offset((*tex).hashValue as isize)
            as *mut *mut image_t;
    loop  {
        cur = *prev;
        if cur.is_null() { break ; }
        if cur == tex {
            *prev = (*cur).nextHash;
            break ;
        } else { prev = &mut (*cur).nextHash }
    }
    // release source
    if !(*tex).original.is_null() {
        gEngfuncs.FS_FreeImage.expect("non-null function pointer")((*tex).original);
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if !(*tex).pixels[i as usize].is_null() {
            gEngfuncs._Mem_Free.expect("non-null function pointer")((*tex).pixels[i
                                                                                      as
                                                                                      usize]
                                                                        as
                                                                        *mut libc::c_void,
                                                                    b"../ref_soft/r_image.c\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char,
                                                                    928 as
                                                                        libc::c_int);
        }
        i += 1
    }
    if !(*tex).alpha_pixels.is_null() {
        gEngfuncs._Mem_Free.expect("non-null function pointer")((*tex).alpha_pixels
                                                                    as
                                                                    *mut libc::c_void,
                                                                b"../ref_soft/r_image.c\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char,
                                                                929 as
                                                                    libc::c_int);
    }
    memset(tex as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<image_t>() as libc::c_ulong);
}
/*
================
GL_UpdateTexSize

recalc image room
================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_UpdateTexSize(mut texnum: libc::c_int,
                                          mut width: libc::c_int,
                                          mut height: libc::c_int,
                                          mut depth: libc::c_int) {
    let mut i: libc::c_int = 0; // recompute now
    let mut j: libc::c_int = 0;
    let mut texsize: libc::c_int = 0;
    let mut numSides: libc::c_int = 0;
    let mut tex: *mut image_t = 0 as *mut image_t;
    if texnum <= 0 as libc::c_int || texnum >= 4096 as libc::c_int { return }
    tex = &mut *r_images.as_mut_ptr().offset(texnum as isize) as *mut image_t;
    numSides =
        if (*tex).flags as libc::c_uint &
               TF_CUBEMAP as libc::c_int as libc::c_uint != 0 {
            6 as libc::c_int
        } else { 1 as libc::c_int };
    GL_SetTextureDimensions(tex, width, height, depth);
    (*tex).size = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int;
    while i < numSides {
        j = 0 as libc::c_int;
        while j <
                  (if 1 as libc::c_int > (*tex).numMips as libc::c_int {
                       1 as libc::c_int
                   } else { (*tex).numMips as libc::c_int }) {
            width =
                if 1 as libc::c_int > (*tex).width as libc::c_int >> j {
                    1 as libc::c_int
                } else { ((*tex).width as libc::c_int) >> j };
            height =
                if 1 as libc::c_int > (*tex).height as libc::c_int >> j {
                    1 as libc::c_int
                } else { ((*tex).height as libc::c_int) >> j };
            texsize =
                GL_CalcTextureSize(width, height, (*tex).depth as libc::c_int)
                    as libc::c_int;
            (*tex).size =
                ((*tex).size as
                     libc::c_ulong).wrapping_add(texsize as libc::c_ulong) as
                    size_t as size_t;
            j += 1
        }
        i += 1
    };
}
/*
================
GL_LoadTexture
================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_LoadTexture(mut name: *const libc::c_char,
                                        mut buf: *const byte,
                                        mut size: size_t,
                                        mut flags: libc::c_int)
 -> libc::c_int {
    let mut tex: *mut image_t = 0 as *mut image_t;
    let mut pic: *mut rgbdata_t = 0 as *mut rgbdata_t;
    let mut picFlags: uint = 0 as libc::c_int as uint;
    if GL_CheckTexName(name) as u64 == 0 { return 0 as libc::c_int }
    // see if already loaded
    tex = GL_TextureForName(name);
    if !tex.is_null() {
        return tex.wrapping_offset_from(r_images.as_mut_ptr()) as libc::c_long
                   as libc::c_int
    }
    if flags & TF_NOFLIP_TGA as libc::c_int != 0 {
        picFlags = picFlags | IL_DONTFLIP_TGA as libc::c_int as libc::c_uint
    }
    if flags & TF_KEEP_SOURCE as libc::c_int != 0 &&
           flags & TF_EXPAND_SOURCE as libc::c_int == 0 {
        picFlags = picFlags | IL_KEEP_8BIT as libc::c_int as libc::c_uint
    }
    // set some image flags
    gEngfuncs.Image_SetForceFlags.expect("non-null function pointer")(picFlags); // couldn't loading image
    pic =
        gEngfuncs.FS_LoadImage.expect("non-null function pointer")(name, buf,
                                                                   size);
    if pic.is_null() { return 0 as libc::c_int }
    // allocate the new one
    tex =
        GL_AllocTexture(name, flags as texFlags_t); // release source texture
    GL_ProcessImage(tex, pic); // update texture filter, wrap etc
    if GL_UploadTexture(tex, pic) as u64 == 0 {
        memset(tex as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<image_t>() as
                   libc::c_ulong); // release source texture
        gEngfuncs.FS_FreeImage.expect("non-null function pointer")(pic);
        return 0 as libc::c_int
    }
    GL_ApplyTextureParams(tex);
    gEngfuncs.FS_FreeImage.expect("non-null function pointer")(pic);
    // NOTE: always return texnum as index in array or engine will stop work !!!
    return tex.wrapping_offset_from(r_images.as_mut_ptr()) as libc::c_long as
               libc::c_int;
}
/*
================
GL_LoadTextureArray
================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_LoadTextureArray(mut names:
                                                 *mut *const libc::c_char,
                                             mut flags: libc::c_int)
 -> libc::c_int {
    return 0 as libc::c_int;
}
/*
================
GL_LoadTextureFromBuffer
================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_LoadTextureFromBuffer(mut name:
                                                      *const libc::c_char,
                                                  mut pic: *mut rgbdata_t,
                                                  mut flags: texFlags_t,
                                                  mut update: qboolean)
 -> libc::c_int {
    let mut tex: *mut image_t = 0 as *mut image_t;
    if GL_CheckTexName(name) as u64 == 0 { return 0 as libc::c_int }
    // see if already loaded
    tex = GL_TextureForName(name);
    if !tex.is_null() && update as u64 == 0 {
        return tex.wrapping_offset_from(r_images.as_mut_ptr()) as libc::c_long
                   as libc::c_int
    }
    // couldn't loading image
    if pic.is_null() { return 0 as libc::c_int }
    if update as u64 != 0 {
        if tex.is_null() {
            gEngfuncs.Host_Error.expect("non-null function pointer")(b"GL_LoadTextureFromBuffer: couldn\'t find texture %s for update\n\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     name);
        }
        (*tex).flags =
            ((*tex).flags as libc::c_uint | flags as libc::c_uint) as
                texFlags_t
    } else {
        // allocate the new one
        tex = GL_AllocTexture(name, flags)
    } // update texture filter, wrap etc
    GL_ProcessImage(tex, pic);
    if GL_UploadTexture(tex, pic) as u64 == 0 {
        memset(tex as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<image_t>() as libc::c_ulong);
        return 0 as libc::c_int
    }
    GL_ApplyTextureParams(tex);
    return tex.wrapping_offset_from(r_images.as_mut_ptr()) as libc::c_long as
               libc::c_int;
}
/*
================
GL_CreateTexture

creates texture from buffer
================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_CreateTexture(mut name: *const libc::c_char,
                                          mut width: libc::c_int,
                                          mut height: libc::c_int,
                                          mut buffer: *const libc::c_void,
                                          mut flags: texFlags_t)
 -> libc::c_int {
    let mut datasize: libc::c_int = 1 as libc::c_int;
    let mut r_empty: rgbdata_t =
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
    if flags as libc::c_uint & TF_ARB_16BIT as libc::c_int as libc::c_uint !=
           0 {
        datasize = 2 as libc::c_int
    } else if flags as libc::c_uint &
                  TF_ARB_FLOAT as libc::c_int as libc::c_uint != 0 {
        datasize = 4 as libc::c_int
    }
    memset(&mut r_empty as *mut rgbdata_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<rgbdata_t>() as libc::c_ulong);
    r_empty.width = width as word;
    r_empty.height = height as word;
    r_empty.type_0 = PF_RGBA_32 as libc::c_int as uint;
    r_empty.size =
        (r_empty.width as libc::c_int * r_empty.height as libc::c_int *
             datasize * 4 as libc::c_int) as size_t;
    r_empty.buffer = buffer as *mut byte;
    // clear invalid combinations
    flags =
        (flags as libc::c_uint &
             !(TF_TEXTURE_3D as libc::c_int) as libc::c_uint) as texFlags_t;
    // if image not luminance and not alphacontrast it will have color
    if flags as libc::c_uint & TF_LUMINANCE as libc::c_int as libc::c_uint ==
           0 &&
           flags as libc::c_uint &
               TF_ALPHACONTRAST as libc::c_int as libc::c_uint == 0 {
        r_empty.flags =
            r_empty.flags | IMAGE_HAS_COLOR as libc::c_int as libc::c_uint
    }
    if flags as libc::c_uint & TF_HAS_ALPHA as libc::c_int as libc::c_uint !=
           0 {
        r_empty.flags =
            r_empty.flags | IMAGE_HAS_ALPHA as libc::c_int as libc::c_uint
    }
    if flags as libc::c_uint & TF_CUBEMAP as libc::c_int as libc::c_uint != 0
       {
        return 0 as libc::c_int
    }
    return GL_LoadTextureFromBuffer(name, &mut r_empty, flags, false_0);
}
/*
================
GL_CreateTextureArray

creates texture array from buffer
================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_CreateTextureArray(mut name: *const libc::c_char,
                                               mut width: libc::c_int,
                                               mut height: libc::c_int,
                                               mut depth: libc::c_int,
                                               mut buffer:
                                                   *const libc::c_void,
                                               mut flags: texFlags_t)
 -> libc::c_int {
    return 0 as libc::c_int;
}
/*
================
GL_FindTexture
================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_FindTexture(mut name: *const libc::c_char)
 -> libc::c_int {
    let mut tex: *mut image_t = 0 as *mut image_t;
    if GL_CheckTexName(name) as u64 == 0 { return 0 as libc::c_int }
    // see if already loaded
    tex = GL_TextureForName(name);
    if !tex.is_null() {
        return tex.wrapping_offset_from(r_images.as_mut_ptr()) as libc::c_long
                   as libc::c_int
    }
    return 0 as libc::c_int;
}
/*
================
GL_FreeTexture
================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_FreeTexture(mut texnum: libc::c_uint) {
    // number 0 it's already freed
    if texnum <= 0 as libc::c_int as libc::c_uint { return }
    GL_DeleteTexture(&mut *r_images.as_mut_ptr().offset(texnum as isize));
}
/*
================
GL_ProcessTexture
================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_ProcessTexture(mut texnum: libc::c_int,
                                           mut gamma: libc::c_float,
                                           mut topColor: libc::c_int,
                                           mut bottomColor: libc::c_int) {
    let mut image: *mut image_t = 0 as *mut image_t; // missed image
    let mut pic: *mut rgbdata_t = 0 as *mut rgbdata_t;
    let mut flags: libc::c_int = 0 as libc::c_int;
    if texnum <= 0 as libc::c_int || texnum >= 4096 as libc::c_int { return }
    image =
        &mut *r_images.as_mut_ptr().offset(texnum as isize) as *mut image_t;
    // select mode
    if gamma != -1.0f32 {
        flags = IMAGE_LIGHTGAMMA as libc::c_int
    } else if topColor != -(1 as libc::c_int) &&
                  bottomColor != -(1 as libc::c_int) {
        flags = IMAGE_REMAP as libc::c_int
    } else {
        gEngfuncs.Con_Printf.expect("non-null function pointer")(b"^1Error:^7 GL_ProcessTexture: bad operation for %s\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 (*image).name.as_mut_ptr());
        return
    }
    if (*image).original.is_null() {
        gEngfuncs.Con_Printf.expect("non-null function pointer")(b"^1Error:^7 GL_ProcessTexture: no input data for %s\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 (*image).name.as_mut_ptr());
        return
    }
    if (*(*image).original).type_0 == PF_DXT1 as libc::c_int as libc::c_uint
           ||
           (*(*image).original).type_0 ==
               PF_DXT3 as libc::c_int as libc::c_uint ||
           (*(*image).original).type_0 ==
               PF_DXT5 as libc::c_int as libc::c_uint ||
           (*(*image).original).type_0 ==
               PF_ATI2 as libc::c_int as libc::c_uint {
        gEngfuncs.Con_Printf.expect("non-null function pointer")(b"^1Error:^7 GL_ProcessTexture: can\'t process compressed texture %s\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 (*image).name.as_mut_ptr());
        return
    }
    // all the operations makes over the image copy not an original
    pic =
        gEngfuncs.FS_CopyImage.expect("non-null function pointer")((*image).original); // update texture filter, wrap etc
    gEngfuncs.Image_Process.expect("non-null function pointer")(&mut pic,
                                                                topColor,
                                                                bottomColor,
                                                                flags as uint,
                                                                0.0f32);
    GL_UploadTexture(image, pic);
    GL_ApplyTextureParams(image);
    gEngfuncs.FS_FreeImage.expect("non-null function pointer")(pic);
}
/*
==============================================================================

INTERNAL TEXTURES

==============================================================================
*/
/*
==================
GL_FakeImage
==================
*/
unsafe extern "C" fn GL_FakeImage(mut width: libc::c_int,
                                  mut height: libc::c_int,
                                  mut depth: libc::c_int,
                                  mut flags: libc::c_int) -> *mut rgbdata_t {
    static mut data2D: [byte; 1024] = [0; 1024]; // 16x16x4
    static mut r_image: rgbdata_t =
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
    // also use this for bad textures, but without alpha
    r_image.width =
        if 1 as libc::c_int > width { 1 as libc::c_int } else { width } as
            word;
    r_image.height =
        if 1 as libc::c_int > height { 1 as libc::c_int } else { height } as
            word;
    r_image.depth =
        if 1 as libc::c_int > depth { 1 as libc::c_int } else { depth } as
            word;
    r_image.flags = flags as uint;
    r_image.type_0 = PF_RGBA_32 as libc::c_int as uint;
    r_image.size =
        (r_image.width as libc::c_int * r_image.height as libc::c_int *
             r_image.depth as libc::c_int * 4 as libc::c_int) as size_t;
    r_image.buffer =
        if r_image.size >
               ::std::mem::size_of::<[byte; 1024]>() as libc::c_ulong {
            0 as *mut byte
        } else { data2D.as_mut_ptr() };
    r_image.palette = 0 as *mut byte;
    r_image.numMips = 1 as libc::c_int as byte;
    r_image.encode = 0 as libc::c_int as word;
    if r_image.flags & IMAGE_CUBEMAP as libc::c_int as libc::c_uint != 0 {
        r_image.size =
            (r_image.size as
                 libc::c_ulong).wrapping_mul(6 as libc::c_int as
                                                 libc::c_ulong) as size_t as
                size_t
    }
    memset(data2D.as_mut_ptr() as *mut libc::c_void, 0xff as libc::c_int,
           ::std::mem::size_of::<[byte; 1024]>() as libc::c_ulong);
    return &mut r_image;
}
/*
==================
R_InitDlightTexture
==================
*/
#[no_mangle]
pub unsafe extern "C" fn R_InitDlightTexture() {
    let mut r_image: rgbdata_t =
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
                  size: 0,}; // already initialized
    if tr.dlightTexture != 0 as libc::c_int { return }
    memset(&mut r_image as *mut rgbdata_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<rgbdata_t>() as libc::c_ulong);
    r_image.width = tr.block_size as word;
    r_image.height = tr.block_size as word;
    r_image.flags = IMAGE_HAS_COLOR as libc::c_int as uint;
    r_image.type_0 = PF_RGBA_32 as libc::c_int as uint;
    r_image.size =
        (r_image.width as libc::c_int * r_image.height as libc::c_int *
             4 as libc::c_int) as size_t;
    tr.dlightTexture =
        GL_LoadTextureFromBuffer(b"*dlight\x00" as *const u8 as
                                     *const libc::c_char, &mut r_image,
                                 (TF_NOMIPMAP as libc::c_int |
                                      TF_CLAMP as libc::c_int |
                                      TF_ATLAS_PAGE as libc::c_int) as
                                     texFlags_t, false_0);
}
/*
==================
GL_CreateInternalTextures
==================
*/
unsafe extern "C" fn GL_CreateInternalTextures() {
    let mut dx2: libc::c_int = 0;
    let mut dy: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut pic: *mut rgbdata_t = 0 as *mut rgbdata_t;
    // emo-texture from quake1
    pic =
        GL_FakeImage(16 as libc::c_int, 16 as libc::c_int, 1 as libc::c_int,
                     IMAGE_HAS_COLOR as libc::c_int);
    y = 0 as libc::c_int;
    while y < 16 as libc::c_int {
        x = 0 as libc::c_int;
        while x < 16 as libc::c_int {
            if (y < 8 as libc::c_int) as libc::c_int ^
                   (x < 8 as libc::c_int) as libc::c_int != 0 {
                *((*pic).buffer as
                      *mut uint).offset((y * 16 as libc::c_int + x) as isize)
                    = 0xffff00ff as libc::c_uint
            } else {
                *((*pic).buffer as
                      *mut uint).offset((y * 16 as libc::c_int + x) as isize)
                    = 0xff000000 as libc::c_uint
            }
            x += 1
        }
        y += 1
    }
    tr.defaultTexture =
        GL_LoadTextureFromBuffer(b"*default\x00" as *const u8 as
                                     *const libc::c_char, pic, TF_COLORMAP,
                                 false_0);
    // particle texture from quake1
    pic =
        GL_FakeImage(16 as libc::c_int, 16 as libc::c_int, 1 as libc::c_int,
                     IMAGE_HAS_COLOR as libc::c_int |
                         IMAGE_HAS_ALPHA as libc::c_int);
    x = 0 as libc::c_int;
    while x < 16 as libc::c_int {
        dx2 = x - 8 as libc::c_int;
        dx2 = dx2 * dx2;
        y = 0 as libc::c_int;
        while y < 16 as libc::c_int {
            dy = y - 8 as libc::c_int;
            d =
                (255 as libc::c_int as libc::c_double -
                     35 as libc::c_int as libc::c_double *
                         __tg_sqrt((dx2 + dy * dy) as libc::c_double)) as
                    libc::c_int;
            *(*pic).buffer.offset(((y * 16 as libc::c_int + x) *
                                       4 as libc::c_int + 3 as libc::c_int) as
                                      isize) =
                if d >= 0 as libc::c_int {
                    if d < 255 as libc::c_int {
                        d
                    } else { 255 as libc::c_int }
                } else { 0 as libc::c_int } as byte;
            y += 1
        }
        x += 1
    }
    tr.particleTexture =
        GL_LoadTextureFromBuffer(b"*particle\x00" as *const u8 as
                                     *const libc::c_char, pic, TF_CLAMP,
                                 false_0);
    // white texture
    pic =
        GL_FakeImage(4 as libc::c_int, 4 as libc::c_int, 1 as libc::c_int,
                     IMAGE_HAS_COLOR as libc::c_int);
    x = 0 as libc::c_int;
    while x < 16 as libc::c_int {
        *((*pic).buffer as *mut uint).offset(x as isize) =
            0xffffffff as libc::c_uint;
        x += 1
    }
    tr.whiteTexture =
        GL_LoadTextureFromBuffer(b"*white\x00" as *const u8 as
                                     *const libc::c_char, pic, TF_COLORMAP,
                                 false_0);
    // gray texture
    pic =
        GL_FakeImage(4 as libc::c_int, 4 as libc::c_int, 1 as libc::c_int,
                     IMAGE_HAS_COLOR as libc::c_int);
    x = 0 as libc::c_int;
    while x < 16 as libc::c_int {
        *((*pic).buffer as *mut uint).offset(x as isize) =
            0xff7f7f7f as libc::c_uint;
        x += 1
    }
    tr.grayTexture =
        GL_LoadTextureFromBuffer(b"*gray\x00" as *const u8 as
                                     *const libc::c_char, pic, TF_COLORMAP,
                                 false_0);
    // black texture
    pic =
        GL_FakeImage(4 as libc::c_int, 4 as libc::c_int, 1 as libc::c_int,
                     IMAGE_HAS_COLOR as libc::c_int);
    x = 0 as libc::c_int;
    while x < 16 as libc::c_int {
        *((*pic).buffer as *mut uint).offset(x as isize) =
            0xff000000 as libc::c_uint;
        x += 1
    }
    tr.blackTexture =
        GL_LoadTextureFromBuffer(b"*black\x00" as *const u8 as
                                     *const libc::c_char, pic, TF_COLORMAP,
                                 false_0);
    // cinematic dummy
    pic =
        GL_FakeImage(640 as libc::c_int, 100 as libc::c_int, 1 as libc::c_int,
                     IMAGE_HAS_COLOR as libc::c_int);
    tr.cinTexture =
        GL_LoadTextureFromBuffer(b"*cintexture\x00" as *const u8 as
                                     *const libc::c_char, pic,
                                 (TF_NOMIPMAP as libc::c_int |
                                      TF_CLAMP as libc::c_int) as texFlags_t,
                                 false_0);
}
/*
===============
R_TextureList_f
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_TextureList_f() {
    let mut image: *mut image_t = 0 as *mut image_t;
    let mut i: libc::c_int = 0;
    let mut texCount: libc::c_int = 0;
    let mut bytes: libc::c_int = 0 as libc::c_int;
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"\n\x00" as
                                                                 *const u8 as
                                                                 *const libc::c_char);
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b" -id-   -w-  -h-     -size- -fmt- -type- -data-  -encode- -wrap- -depth- -name--------\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char);
    texCount = 0 as libc::c_int;
    i = texCount;
    image = r_images.as_mut_ptr();
    while (i as libc::c_uint) < r_numImages {
        if !(*image).pixels.as_mut_ptr().is_null() {
            bytes =
                (bytes as libc::c_ulong).wrapping_add((*image).size) as
                    libc::c_int as libc::c_int;
            texCount += 1;
            gEngfuncs.Con_Printf.expect("non-null function pointer")(b"%4i: \x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     i);
            gEngfuncs.Con_Printf.expect("non-null function pointer")(b"%4i %4i \x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     (*image).width
                                                                         as
                                                                         libc::c_int,
                                                                     (*image).height
                                                                         as
                                                                         libc::c_int);
            gEngfuncs.Con_Printf.expect("non-null function pointer")(b"%12s \x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     Q_pretifymem((*image).size
                                                                                      as
                                                                                      libc::c_float,
                                                                                  2
                                                                                      as
                                                                                      libc::c_int));
            if (*image).flags as libc::c_uint &
                   TF_NORMALMAP as libc::c_int as libc::c_uint != 0 {
                gEngfuncs.Con_Printf.expect("non-null function pointer")(b"normal  \x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char);
            } else {
                gEngfuncs.Con_Printf.expect("non-null function pointer")(b"diffuse \x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char);
            }
            if (*image).flags as libc::c_uint &
                   TF_CLAMP as libc::c_int as libc::c_uint != 0 {
                gEngfuncs.Con_Printf.expect("non-null function pointer")(b"clamp  \x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char);
            } else if (*image).flags as libc::c_uint &
                          TF_BORDER as libc::c_int as libc::c_uint != 0 {
                gEngfuncs.Con_Printf.expect("non-null function pointer")(b"border \x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char);
            } else {
                gEngfuncs.Con_Printf.expect("non-null function pointer")(b"repeat \x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char);
            }
            gEngfuncs.Con_Printf.expect("non-null function pointer")(b"   %d  \x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     (*image).depth
                                                                         as
                                                                         libc::c_int);
            gEngfuncs.Con_Printf.expect("non-null function pointer")(b"  %s\n\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     (*image).name.as_mut_ptr());
        }
        i += 1;
        image = image.offset(1)
    }
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"---------------------------------------------------------\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char);
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"%i total textures\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             texCount);
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"%s total memory used\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             Q_pretifymem(bytes
                                                                              as
                                                                              libc::c_float,
                                                                          2 as
                                                                              libc::c_int));
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"\n\x00" as
                                                                 *const u8 as
                                                                 *const libc::c_char);
}
/*
===============
R_InitImages
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_InitImages() {
    memset(r_images.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[image_t; 4096]>() as libc::c_ulong);
    memset(r_imagesHashTable.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<[*mut image_t; 1024]>() as libc::c_ulong);
    r_numImages = 0 as libc::c_int as uint;
    // create unused 0-entry
    Q_strncpy((*r_images.as_mut_ptr()).name.as_mut_ptr(),
              b"*unused*\x00" as *const u8 as *const libc::c_char,
              ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong);
    (*r_images.as_mut_ptr()).hashValue =
        COM_HashKey((*r_images.as_mut_ptr()).name.as_mut_ptr(),
                    (4096 as libc::c_int >> 2 as libc::c_int) as uint);
    let ref mut fresh0 = (*r_images.as_mut_ptr()).nextHash;
    *fresh0 = r_imagesHashTable[(*r_images.as_mut_ptr()).hashValue as usize];
    r_imagesHashTable[(*r_images.as_mut_ptr()).hashValue as usize] =
        r_images.as_mut_ptr();
    r_numImages = 1 as libc::c_int as uint;
    // validate cvars
    R_SetTextureParameters();
    GL_CreateInternalTextures();
    gEngfuncs.Cmd_AddCommand.expect("non-null function pointer")(b"texturelist\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 Some(R_TextureList_f
                                                                          as
                                                                          unsafe extern "C" fn()
                                                                              ->
                                                                                  ()),
                                                                 b"display loaded textures list\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char);
}
/*
===============
R_ShutdownImages
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_ShutdownImages() {
    let mut tex: *mut image_t = 0 as *mut image_t;
    let mut i: libc::c_int = 0;
    gEngfuncs.Cmd_RemoveCommand.expect("non-null function pointer")(b"texturelist\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char);
    i = 0 as libc::c_int;
    tex = r_images.as_mut_ptr();
    while (i as libc::c_uint) < r_numImages {
        GL_DeleteTexture(tex);
        i += 1;
        tex = tex.offset(1)
    }
    memset(tr.lightmapTextures.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<[libc::c_int; 256]>() as libc::c_ulong);
    memset(r_imagesHashTable.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<[*mut image_t; 1024]>() as libc::c_ulong);
    memset(r_images.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[image_t; 4096]>() as libc::c_ulong);
    r_numImages = 0 as libc::c_int as uint;
}
