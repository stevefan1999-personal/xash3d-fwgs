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
    fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t,
             __compar: __compar_fn_t);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn sqrtf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn fabsf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn Matrix4x4_VectorITransform(in_0: *const [vec_t; 4],
                                  v: *const libc::c_float,
                                  out: *mut libc::c_float);
    #[no_mangle]
    fn Matrix4x4_CreateFromEntity(out: *mut [vec_t; 4], angles: *const vec_t,
                                  origin: *const vec_t, scale: libc::c_float);
    #[no_mangle]
    static mut r_temppool: poolhandle_t;
    #[no_mangle]
    static mut tr: gl_globals_t;
    #[no_mangle]
    static mut gEngfuncs: ref_api_t;
    #[no_mangle]
    fn R_GetTextureParms(w: *mut libc::c_int, h: *mut libc::c_int,
                         texnum: libc::c_int);
    #[no_mangle]
    fn R_GetTexture(texnum: libc::c_uint) -> *mut image_t;
    #[no_mangle]
    static mut r_decals: *mut cvar_t;
    #[no_mangle]
    fn Q_strncmp(s1: *const libc::c_char, s2: *const libc::c_char,
                 n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn COM_FileBase(in_0: *const libc::c_char, out: *mut libc::c_char);
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
pub type vec2_t = [vec_t; 2];
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
pub type connstate_e = libc::c_uint;
pub const ca_cinematic: connstate_e = 5;
pub const ca_active: connstate_e = 4;
pub const ca_validate: connstate_e = 3;
pub const ca_connected: connstate_e = 2;
pub const ca_connecting: connstate_e = 1;
pub const ca_disconnected: connstate_e = 0;
pub type connstate_t = connstate_e;
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
pub type imagetype_t = libc::c_uint;
pub const it_sky: imagetype_t = 4;
pub const it_pic: imagetype_t = 3;
pub const it_wall: imagetype_t = 2;
pub const it_sprite: imagetype_t = 1;
pub const it_skin: imagetype_t = 0;
pub type pixel_t = libc::c_ushort;
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
pub struct decalinfo_t {
    pub m_Position: vec3_t,
    pub m_pModel: *mut model_t,
    pub m_iTexture: libc::c_int,
    pub m_Size: libc::c_int,
    pub m_Flags: libc::c_int,
    pub m_Entity: libc::c_int,
    pub m_scale: libc::c_float,
    pub m_decalWidth: libc::c_int,
    pub m_decalHeight: libc::c_int,
    pub m_Basis: [vec3_t; 3],
}
#[inline(always)]
unsafe extern "C" fn __tg_sqrt(mut __x: libc::c_float) -> libc::c_float {
    return sqrtf(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_fabs(mut __x: libc::c_float) -> libc::c_float {
    return fabsf(__x);
}
static mut g_DecalClipVerts: [[libc::c_float; 7]; 32] = [[0.; 7]; 32];
static mut g_DecalClipVerts2: [[libc::c_float; 7]; 32] = [[0.; 7]; 32];
#[no_mangle]
pub static mut gDecalPool: [decal_t; 4096] =
    [decal_t{pnext: 0 as *const decal_t as *mut decal_t,
             psurface: 0 as *const msurface_t as *mut msurface_t,
             dx: 0.,
             dy: 0.,
             scale: 0.,
             texture: 0,
             flags: 0,
             entityIndex: 0,
             position: [0.; 3],
             polys: 0 as *const glpoly_t as *mut glpoly_t,
             reserved: [0; 4],}; 4096];
static mut gDecalCount: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn R_ClearDecals() {
    memset(gDecalPool.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[decal_t; 4096]>() as libc::c_ulong);
    gDecalCount = 0 as libc::c_int;
}
// unlink pdecal from any surface it's attached to
unsafe extern "C" fn R_DecalUnlink(mut pdecal: *mut decal_t) {
    let mut tmp: *mut decal_t = 0 as *mut decal_t;
    if !(*pdecal).psurface.is_null() {
        if (*(*pdecal).psurface).pdecals == pdecal {
            (*(*pdecal).psurface).pdecals = (*pdecal).pnext
        } else {
            tmp = (*(*pdecal).psurface).pdecals;
            if tmp.is_null() {
                gEngfuncs.Host_Error.expect("non-null function pointer")(b"D_DecalUnlink: bad decal list\n\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char);
            }
            while !(*tmp).pnext.is_null() {
                if (*tmp).pnext == pdecal {
                    (*tmp).pnext = (*pdecal).pnext;
                    break ;
                } else { tmp = (*tmp).pnext }
            }
        }
    }
    if !(*pdecal).polys.is_null() {
        gEngfuncs._Mem_Free.expect("non-null function pointer")((*pdecal).polys
                                                                    as
                                                                    *mut libc::c_void,
                                                                b"../ref_soft/r_decals.c\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char,
                                                                93 as
                                                                    libc::c_int);
    }
    (*pdecal).psurface = 0 as *mut msurface_t;
    (*pdecal).polys = 0 as *mut glpoly_t;
}
// Just reuse next decal in list
// A decal that spans multiple surfaces will use multiple decal_t pool entries,
// as each surface needs it's own.
unsafe extern "C" fn R_DecalAlloc(mut pdecal: *mut decal_t) -> *mut decal_t {
    let mut limit: libc::c_int = 4096 as libc::c_int;
    if (*r_decals).value < limit as libc::c_float {
        limit = (*r_decals).value as libc::c_int
    }
    if limit == 0 { return 0 as *mut decal_t }
    if pdecal.is_null() {
        let mut count: libc::c_int = 0 as libc::c_int;
        loop 
             // check for the odd possiblity of infinte loop
             {
            if gDecalCount >= limit {
                gDecalCount = 0 as libc::c_int
            } // reuse next decal
            pdecal =
                &mut *gDecalPool.as_mut_ptr().offset(gDecalCount as isize) as
                    *mut decal_t;
            gDecalCount += 1;
            count += 1;
            if !((*pdecal).flags as libc::c_int & 0x1 as libc::c_int != 0 &&
                     count < limit) {
                break ;
            }
        }
    }
    // if decal is already linked to a surface, unlink it.
    R_DecalUnlink(pdecal);
    return pdecal;
}
//-----------------------------------------------------------------------------
// find decal image and grab size from it
//-----------------------------------------------------------------------------
unsafe extern "C" fn R_GetDecalDimensions(mut texture: libc::c_int,
                                          mut width: *mut libc::c_int,
                                          mut height: *mut libc::c_int) {
    if !width.is_null() {
        *width = 1 as libc::c_int
    } // to avoid divide by zero
    if !height.is_null() { *height = 1 as libc::c_int }
    R_GetTextureParms(width, height, texture);
}
//-----------------------------------------------------------------------------
// compute the decal basis based on surface normal
//-----------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn R_DecalComputeBasis(mut surf: *mut msurface_t,
                                             mut flags: libc::c_int,
                                             mut textureSpaceBasis:
                                                 *mut vec3_t) {
    let mut surfaceNormal: vec3_t = [0.; 3];
    // setup normal
    if (*surf).flags as libc::c_uint & (1 as libc::c_uint) << 1 as libc::c_int
           != 0 {
        surfaceNormal[0 as libc::c_int as usize] =
            -(*(*surf).plane).normal[0 as libc::c_int as usize];
        surfaceNormal[1 as libc::c_int as usize] =
            -(*(*surf).plane).normal[1 as libc::c_int as usize];
        surfaceNormal[2 as libc::c_int as usize] =
            -(*(*surf).plane).normal[2 as libc::c_int as usize]
    } else {
        surfaceNormal[0 as libc::c_int as usize] =
            (*(*surf).plane).normal[0 as libc::c_int as usize];
        surfaceNormal[1 as libc::c_int as usize] =
            (*(*surf).plane).normal[1 as libc::c_int as usize];
        surfaceNormal[2 as libc::c_int as usize] =
            (*(*surf).plane).normal[2 as libc::c_int as usize]
    }
    let mut ilength: libc::c_float =
        __tg_sqrt(surfaceNormal[0 as libc::c_int as usize] *
                      surfaceNormal[0 as libc::c_int as usize] +
                      surfaceNormal[1 as libc::c_int as usize] *
                          surfaceNormal[1 as libc::c_int as usize] +
                      surfaceNormal[2 as libc::c_int as usize] *
                          surfaceNormal[2 as libc::c_int as usize]);
    if ilength != 0. { ilength = 1.0f32 / ilength }
    (*textureSpaceBasis.offset(2 as libc::c_int as
                                   isize))[0 as libc::c_int as usize] =
        surfaceNormal[0 as libc::c_int as usize] * ilength;
    (*textureSpaceBasis.offset(2 as libc::c_int as
                                   isize))[1 as libc::c_int as usize] =
        surfaceNormal[1 as libc::c_int as usize] * ilength;
    (*textureSpaceBasis.offset(2 as libc::c_int as
                                   isize))[2 as libc::c_int as usize] =
        surfaceNormal[2 as libc::c_int as usize] * ilength;
    let mut ilength_0: libc::c_float =
        __tg_sqrt((*(*surf).texinfo).vecs[0 as libc::c_int as
                                              usize][0 as libc::c_int as
                                                         usize] *
                      (*(*surf).texinfo).vecs[0 as libc::c_int as
                                                  usize][0 as libc::c_int as
                                                             usize] +
                      (*(*surf).texinfo).vecs[0 as libc::c_int as
                                                  usize][1 as libc::c_int as
                                                             usize] *
                          (*(*surf).texinfo).vecs[0 as libc::c_int as
                                                      usize][1 as libc::c_int
                                                                 as usize] +
                      (*(*surf).texinfo).vecs[0 as libc::c_int as
                                                  usize][2 as libc::c_int as
                                                             usize] *
                          (*(*surf).texinfo).vecs[0 as libc::c_int as
                                                      usize][2 as libc::c_int
                                                                 as usize]);
    if ilength_0 != 0. { ilength_0 = 1.0f32 / ilength_0 }
    (*textureSpaceBasis.offset(0 as libc::c_int as
                                   isize))[0 as libc::c_int as usize] =
        (*(*surf).texinfo).vecs[0 as libc::c_int as
                                    usize][0 as libc::c_int as usize] *
            ilength_0;
    (*textureSpaceBasis.offset(0 as libc::c_int as
                                   isize))[1 as libc::c_int as usize] =
        (*(*surf).texinfo).vecs[0 as libc::c_int as
                                    usize][1 as libc::c_int as usize] *
            ilength_0;
    (*textureSpaceBasis.offset(0 as libc::c_int as
                                   isize))[2 as libc::c_int as usize] =
        (*(*surf).texinfo).vecs[0 as libc::c_int as
                                    usize][2 as libc::c_int as usize] *
            ilength_0;
    let mut ilength_1: libc::c_float =
        __tg_sqrt((*(*surf).texinfo).vecs[1 as libc::c_int as
                                              usize][0 as libc::c_int as
                                                         usize] *
                      (*(*surf).texinfo).vecs[1 as libc::c_int as
                                                  usize][0 as libc::c_int as
                                                             usize] +
                      (*(*surf).texinfo).vecs[1 as libc::c_int as
                                                  usize][1 as libc::c_int as
                                                             usize] *
                          (*(*surf).texinfo).vecs[1 as libc::c_int as
                                                      usize][1 as libc::c_int
                                                                 as usize] +
                      (*(*surf).texinfo).vecs[1 as libc::c_int as
                                                  usize][2 as libc::c_int as
                                                             usize] *
                          (*(*surf).texinfo).vecs[1 as libc::c_int as
                                                      usize][2 as libc::c_int
                                                                 as usize]);
    if ilength_1 != 0. { ilength_1 = 1.0f32 / ilength_1 }
    (*textureSpaceBasis.offset(1 as libc::c_int as
                                   isize))[0 as libc::c_int as usize] =
        (*(*surf).texinfo).vecs[1 as libc::c_int as
                                    usize][0 as libc::c_int as usize] *
            ilength_1;
    (*textureSpaceBasis.offset(1 as libc::c_int as
                                   isize))[1 as libc::c_int as usize] =
        (*(*surf).texinfo).vecs[1 as libc::c_int as
                                    usize][1 as libc::c_int as usize] *
            ilength_1;
    (*textureSpaceBasis.offset(1 as libc::c_int as
                                   isize))[2 as libc::c_int as usize] =
        (*(*surf).texinfo).vecs[1 as libc::c_int as
                                    usize][2 as libc::c_int as usize] *
            ilength_1;
}
#[no_mangle]
pub unsafe extern "C" fn R_SetupDecalTextureSpaceBasis(mut pDecal:
                                                           *mut decal_t,
                                                       mut surf:
                                                           *mut msurface_t,
                                                       mut texture:
                                                           libc::c_int,
                                                       mut textureSpaceBasis:
                                                           *mut vec3_t,
                                                       mut decalWorldScale:
                                                           *mut libc::c_float) {
    let mut width: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    // Compute the non-scaled decal basis
    R_DecalComputeBasis(surf, (*pDecal).flags as libc::c_int,
                        textureSpaceBasis);
    R_GetDecalDimensions(texture, &mut width, &mut height);
    // world width of decal = ptexture->width / pDecal->scale
	// world height of decal = ptexture->height / pDecal->scale
	// scale is inverse, scales world space to decal u/v space [0,1]
	// OPTIMIZE: Get rid of these divides
    *decalWorldScale.offset(0 as libc::c_int as isize) =
        (*pDecal).scale / width as libc::c_float;
    *decalWorldScale.offset(1 as libc::c_int as isize) =
        (*pDecal).scale / height as libc::c_float;
    (*textureSpaceBasis.offset(0 as libc::c_int as
                                   isize))[0 as libc::c_int as usize] =
        (*textureSpaceBasis.offset(0 as libc::c_int as
                                       isize))[0 as libc::c_int as usize] *
            *decalWorldScale.offset(0 as libc::c_int as isize);
    (*textureSpaceBasis.offset(0 as libc::c_int as
                                   isize))[1 as libc::c_int as usize] =
        (*textureSpaceBasis.offset(0 as libc::c_int as
                                       isize))[1 as libc::c_int as usize] *
            *decalWorldScale.offset(0 as libc::c_int as isize);
    (*textureSpaceBasis.offset(0 as libc::c_int as
                                   isize))[2 as libc::c_int as usize] =
        (*textureSpaceBasis.offset(0 as libc::c_int as
                                       isize))[2 as libc::c_int as usize] *
            *decalWorldScale.offset(0 as libc::c_int as isize);
    (*textureSpaceBasis.offset(1 as libc::c_int as
                                   isize))[0 as libc::c_int as usize] =
        (*textureSpaceBasis.offset(1 as libc::c_int as
                                       isize))[0 as libc::c_int as usize] *
            *decalWorldScale.offset(1 as libc::c_int as isize);
    (*textureSpaceBasis.offset(1 as libc::c_int as
                                   isize))[1 as libc::c_int as usize] =
        (*textureSpaceBasis.offset(1 as libc::c_int as
                                       isize))[1 as libc::c_int as usize] *
            *decalWorldScale.offset(1 as libc::c_int as isize);
    (*textureSpaceBasis.offset(1 as libc::c_int as
                                   isize))[2 as libc::c_int as usize] =
        (*textureSpaceBasis.offset(1 as libc::c_int as
                                       isize))[2 as libc::c_int as usize] *
            *decalWorldScale.offset(1 as libc::c_int as isize);
}
// Build the initial list of vertices from the surface verts into the global array, 'verts'.
#[no_mangle]
pub unsafe extern "C" fn R_SetupDecalVertsForMSurface(mut pDecal:
                                                          *mut decal_t,
                                                      mut surf:
                                                          *mut msurface_t,
                                                      mut textureSpaceBasis:
                                                          *mut vec3_t,
                                                      mut verts:
                                                          *mut libc::c_float) {
    let mut v: *mut libc::c_float =
        0 as *mut libc::c_float; // copy model space coordinates
    let mut i: libc::c_int = 0;
    if (*surf).polys.is_null() { return }
    i = 0 as libc::c_int;
    v = (*(*surf).polys).verts[0 as libc::c_int as usize].as_mut_ptr();
    while i < (*(*surf).polys).numverts {
        *verts.offset(0 as libc::c_int as isize) =
            *v.offset(0 as libc::c_int as isize);
        *verts.offset(1 as libc::c_int as isize) =
            *v.offset(1 as libc::c_int as isize);
        *verts.offset(2 as libc::c_int as isize) =
            *v.offset(2 as libc::c_int as isize);
        *verts.offset(3 as libc::c_int as isize) =
            *verts.offset(0 as libc::c_int as isize) *
                (*textureSpaceBasis.offset(0 as libc::c_int as
                                               isize))[0 as libc::c_int as
                                                           usize] +
                *verts.offset(1 as libc::c_int as isize) *
                    (*textureSpaceBasis.offset(0 as libc::c_int as
                                                   isize))[1 as libc::c_int as
                                                               usize] +
                *verts.offset(2 as libc::c_int as isize) *
                    (*textureSpaceBasis.offset(0 as libc::c_int as
                                                   isize))[2 as libc::c_int as
                                                               usize] -
                (*pDecal).dx + 0.5f32;
        *verts.offset(4 as libc::c_int as isize) =
            *verts.offset(0 as libc::c_int as isize) *
                (*textureSpaceBasis.offset(1 as libc::c_int as
                                               isize))[0 as libc::c_int as
                                                           usize] +
                *verts.offset(1 as libc::c_int as isize) *
                    (*textureSpaceBasis.offset(1 as libc::c_int as
                                                   isize))[1 as libc::c_int as
                                                               usize] +
                *verts.offset(2 as libc::c_int as isize) *
                    (*textureSpaceBasis.offset(1 as libc::c_int as
                                                   isize))[2 as libc::c_int as
                                                               usize] -
                (*pDecal).dy + 0.5f32;
        let ref mut fresh0 = *verts.offset(6 as libc::c_int as isize);
        *fresh0 = 0.0f32;
        *verts.offset(5 as libc::c_int as isize) = *fresh0;
        i += 1;
        v = v.offset(7 as libc::c_int as isize);
        verts = verts.offset(7 as libc::c_int as isize)
    };
}
// Figure out where the decal maps onto the surface.
#[no_mangle]
pub unsafe extern "C" fn R_SetupDecalClip(mut pDecal: *mut decal_t,
                                          mut surf: *mut msurface_t,
                                          mut texture: libc::c_int,
                                          mut textureSpaceBasis: *mut vec3_t,
                                          mut decalWorldScale:
                                              *mut libc::c_float) {
    R_SetupDecalTextureSpaceBasis(pDecal, surf, texture, textureSpaceBasis,
                                  decalWorldScale);
    // Generate texture coordinates for each vertex in decal s,t space
	// probably should pre-generate this, store it and use it for decal-decal collisions
	// as in R_DecalsIntersect()
    (*pDecal).dx =
        (*pDecal).position[0 as libc::c_int as usize] *
            (*textureSpaceBasis.offset(0 as libc::c_int as
                                           isize))[0 as libc::c_int as usize]
            +
            (*pDecal).position[1 as libc::c_int as usize] *
                (*textureSpaceBasis.offset(0 as libc::c_int as
                                               isize))[1 as libc::c_int as
                                                           usize] +
            (*pDecal).position[2 as libc::c_int as usize] *
                (*textureSpaceBasis.offset(0 as libc::c_int as
                                               isize))[2 as libc::c_int as
                                                           usize];
    (*pDecal).dy =
        (*pDecal).position[0 as libc::c_int as usize] *
            (*textureSpaceBasis.offset(1 as libc::c_int as
                                           isize))[0 as libc::c_int as usize]
            +
            (*pDecal).position[1 as libc::c_int as usize] *
                (*textureSpaceBasis.offset(1 as libc::c_int as
                                               isize))[1 as libc::c_int as
                                                           usize] +
            (*pDecal).position[2 as libc::c_int as usize] *
                (*textureSpaceBasis.offset(1 as libc::c_int as
                                               isize))[2 as libc::c_int as
                                                           usize];
}
// Quick and dirty sutherland Hodgman clipper
// Clip polygon to decal in texture space
// JAY: This code is lame, change it later.  It does way too much work per frame
// It can be made to recursively call the clipping code and only copy the vertex list once
#[no_mangle]
pub unsafe extern "C" fn R_ClipInside(mut vert: *mut libc::c_float,
                                      mut edge: libc::c_int) -> libc::c_int {
    match edge {
        0 => {
            if *vert.offset(3 as libc::c_int as isize) > 0.0f32 {
                return 1 as libc::c_int
            }
            return 0 as libc::c_int
        }
        1 => {
            if *vert.offset(3 as libc::c_int as isize) < 1.0f32 {
                return 1 as libc::c_int
            }
            return 0 as libc::c_int
        }
        2 => {
            if *vert.offset(4 as libc::c_int as isize) > 0.0f32 {
                return 1 as libc::c_int
            }
            return 0 as libc::c_int
        }
        3 => {
            if *vert.offset(4 as libc::c_int as isize) < 1.0f32 {
                return 1 as libc::c_int
            }
            return 0 as libc::c_int
        }
        _ => { }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn R_ClipIntersect(mut one: *mut libc::c_float,
                                         mut two: *mut libc::c_float,
                                         mut out: *mut libc::c_float,
                                         mut edge: libc::c_int) {
    let mut t: libc::c_float = 0.;
    // t is the parameter of the line between one and two clipped to the edge
	// or the fraction of the clipped point between one & two
	// vert[0], vert[1], vert[2] is X, Y, Z
	// vert[3] is u
	// vert[4] is v
	// vert[5] is lightmap u
	// vert[6] is lightmap v
    if edge < 2 as libc::c_int {
        if edge == 0 as libc::c_int {
            // left
            t =
                (*one.offset(3 as libc::c_int as isize) - 0.0f32) /
                    (*one.offset(3 as libc::c_int as isize) -
                         *two.offset(3 as libc::c_int as isize));
            let ref mut fresh1 = *out.offset(5 as libc::c_int as isize);
            *fresh1 = 0.0f32;
            *out.offset(3 as libc::c_int as isize) = *fresh1
        } else {
            // right
            t =
                (*one.offset(3 as libc::c_int as isize) - 1.0f32) /
                    (*one.offset(3 as libc::c_int as isize) -
                         *two.offset(3 as libc::c_int as isize));
            let ref mut fresh2 = *out.offset(5 as libc::c_int as isize);
            *fresh2 = 1.0f32;
            *out.offset(3 as libc::c_int as isize) = *fresh2
        }
        *out.offset(4 as libc::c_int as isize) =
            *one.offset(4 as libc::c_int as isize) +
                (*two.offset(4 as libc::c_int as isize) -
                     *one.offset(4 as libc::c_int as isize)) * t;
        *out.offset(6 as libc::c_int as isize) =
            *one.offset(6 as libc::c_int as isize) +
                (*two.offset(6 as libc::c_int as isize) -
                     *one.offset(6 as libc::c_int as isize)) * t
    } else {
        if edge == 2 as libc::c_int {
            // top
            t =
                (*one.offset(4 as libc::c_int as isize) - 0.0f32) /
                    (*one.offset(4 as libc::c_int as isize) -
                         *two.offset(4 as libc::c_int as isize));
            let ref mut fresh3 = *out.offset(6 as libc::c_int as isize);
            *fresh3 = 0.0f32;
            *out.offset(4 as libc::c_int as isize) = *fresh3
        } else {
            // bottom
            t =
                (*one.offset(4 as libc::c_int as isize) - 1.0f32) /
                    (*one.offset(4 as libc::c_int as isize) -
                         *two.offset(4 as libc::c_int as isize));
            let ref mut fresh4 = *out.offset(6 as libc::c_int as isize);
            *fresh4 = 1.0f32;
            *out.offset(4 as libc::c_int as isize) = *fresh4
        }
        *out.offset(3 as libc::c_int as isize) =
            *one.offset(3 as libc::c_int as isize) +
                (*two.offset(3 as libc::c_int as isize) -
                     *one.offset(3 as libc::c_int as isize)) * t;
        *out.offset(5 as libc::c_int as isize) =
            *one.offset(5 as libc::c_int as isize) +
                (*two.offset(4 as libc::c_int as isize) -
                     *one.offset(5 as libc::c_int as isize)) * t
    }
    *out.offset(0 as libc::c_int as isize) =
        *one.offset(0 as libc::c_int as isize) +
            t *
                (*two.offset(0 as libc::c_int as isize) -
                     *one.offset(0 as libc::c_int as isize));
    *out.offset(1 as libc::c_int as isize) =
        *one.offset(1 as libc::c_int as isize) +
            t *
                (*two.offset(1 as libc::c_int as isize) -
                     *one.offset(1 as libc::c_int as isize));
    *out.offset(2 as libc::c_int as isize) =
        *one.offset(2 as libc::c_int as isize) +
            t *
                (*two.offset(2 as libc::c_int as isize) -
                     *one.offset(2 as libc::c_int as isize));
}
unsafe extern "C" fn SHClip(mut vert: *mut libc::c_float,
                            mut vertCount: libc::c_int,
                            mut out: *mut libc::c_float,
                            mut edge: libc::c_int) -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut outCount: libc::c_int = 0;
    let mut s: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut p: *mut libc::c_float = 0 as *mut libc::c_float;
    outCount = 0 as libc::c_int;
    s =
        &mut *vert.offset(((vertCount - 1 as libc::c_int) * 7 as libc::c_int)
                              as isize) as *mut libc::c_float;
    j = 0 as libc::c_int;
    while j < vertCount {
        p =
            &mut *vert.offset((j * 7 as libc::c_int) as isize) as
                *mut libc::c_float;
        if R_ClipInside(p, edge) != 0 {
            if R_ClipInside(s, edge) != 0 {
                // Add a vertex and advance out to next vertex
                memcpy(out as *mut libc::c_void, p as *const libc::c_void,
                       (::std::mem::size_of::<libc::c_float>() as
                            libc::c_ulong).wrapping_mul(7 as libc::c_int as
                                                            libc::c_ulong));
                out = out.offset(7 as libc::c_int as isize);
                outCount += 1
            } else {
                R_ClipIntersect(s, p, out, edge);
                out = out.offset(7 as libc::c_int as isize);
                outCount += 1;
                memcpy(out as *mut libc::c_void, p as *const libc::c_void,
                       (::std::mem::size_of::<libc::c_float>() as
                            libc::c_ulong).wrapping_mul(7 as libc::c_int as
                                                            libc::c_ulong));
                out = out.offset(7 as libc::c_int as isize);
                outCount += 1
            }
        } else if R_ClipInside(s, edge) != 0 {
            R_ClipIntersect(p, s, out, edge);
            out = out.offset(7 as libc::c_int as isize);
            outCount += 1
        }
        s = p;
        j += 1
    }
    return outCount;
}
#[no_mangle]
pub unsafe extern "C" fn R_DoDecalSHClip(mut pInVerts: *mut libc::c_float,
                                         mut pDecal: *mut decal_t,
                                         mut nStartVerts: libc::c_int,
                                         mut pVertCount: *mut libc::c_int)
 -> *mut libc::c_float {
    let mut pOutVerts: *mut libc::c_float =
        g_DecalClipVerts[0 as libc::c_int as usize].as_mut_ptr();
    let mut outCount: libc::c_int = 0;
    // clip the polygon to the decal texture space
    outCount =
        SHClip(pInVerts, nStartVerts,
               g_DecalClipVerts2[0 as libc::c_int as usize].as_mut_ptr(),
               0 as libc::c_int);
    outCount =
        SHClip(g_DecalClipVerts2[0 as libc::c_int as usize].as_mut_ptr(),
               outCount,
               g_DecalClipVerts[0 as libc::c_int as usize].as_mut_ptr(),
               1 as libc::c_int);
    outCount =
        SHClip(g_DecalClipVerts[0 as libc::c_int as usize].as_mut_ptr(),
               outCount,
               g_DecalClipVerts2[0 as libc::c_int as usize].as_mut_ptr(),
               2 as libc::c_int);
    outCount =
        SHClip(g_DecalClipVerts2[0 as libc::c_int as usize].as_mut_ptr(),
               outCount, pOutVerts, 3 as libc::c_int);
    if !pVertCount.is_null() { *pVertCount = outCount }
    return pOutVerts;
}
//-----------------------------------------------------------------------------
// Generate clipped vertex list for decal pdecal projected onto polygon psurf
//-----------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn R_DecalVertsClip(mut pDecal: *mut decal_t,
                                          mut surf: *mut msurface_t,
                                          mut texture: libc::c_int,
                                          mut pVertCount: *mut libc::c_int)
 -> *mut libc::c_float {
    let mut decalWorldScale: [libc::c_float; 2] = [0.; 2];
    let mut textureSpaceBasis: [vec3_t; 3] = [[0.; 3]; 3];
    // figure out where the decal maps onto the surface.
    R_SetupDecalClip(pDecal, surf, texture, textureSpaceBasis.as_mut_ptr(),
                     decalWorldScale.as_mut_ptr());
    // build the initial list of vertices from the surface verts.
    R_SetupDecalVertsForMSurface(pDecal, surf, textureSpaceBasis.as_mut_ptr(),
                                 g_DecalClipVerts[0 as libc::c_int as
                                                      usize].as_mut_ptr());
    if (*surf).polys.is_null() { return 0 as *mut libc::c_float }
    return R_DoDecalSHClip(g_DecalClipVerts[0 as libc::c_int as
                                                usize].as_mut_ptr(), pDecal,
                           (*(*surf).polys).numverts, pVertCount);
}
// Generate lighting coordinates at each vertex for decal vertices v[] on surface psurf
unsafe extern "C" fn R_DecalVertsLight(mut v: *mut libc::c_float,
                                       mut surf: *mut msurface_t,
                                       mut vertCount: libc::c_int) {
    let mut s: libc::c_float = 0.;
    let mut t: libc::c_float = 0.;
    let mut tex: *mut mtexinfo_t = 0 as *mut mtexinfo_t;
    let mut info: *mut mextrasurf_t = (*surf).info;
    let mut sample_size: libc::c_float = 0.;
    let mut j: libc::c_int = 0;
    sample_size =
        gEngfuncs.Mod_SampleSizeForFace.expect("non-null function pointer")(surf)
            as libc::c_float;
    tex = (*surf).texinfo;
    j = 0 as libc::c_int;
    while j < vertCount {
        // lightmap texture coordinates
        s =
            *v.offset(0 as libc::c_int as isize) *
                (*info).lmvecs[0 as libc::c_int as
                                   usize][0 as libc::c_int as usize] +
                *v.offset(1 as libc::c_int as isize) *
                    (*info).lmvecs[0 as libc::c_int as
                                       usize][1 as libc::c_int as usize] +
                *v.offset(2 as libc::c_int as isize) *
                    (*info).lmvecs[0 as libc::c_int as
                                       usize][2 as libc::c_int as usize] +
                (*info).lmvecs[0 as libc::c_int as
                                   usize][3 as libc::c_int as usize] -
                (*info).lightmapmins[0 as libc::c_int as usize] as libc::c_int
                    as libc::c_float; //fa->texinfo->texture->width;
        s +=
            (*surf).light_s as libc::c_float *
                sample_size; //fa->texinfo->texture->height;
        s += sample_size * 0.5f32;
        s /= tr.block_size as libc::c_float * sample_size;
        t =
            *v.offset(0 as libc::c_int as isize) *
                (*info).lmvecs[1 as libc::c_int as
                                   usize][0 as libc::c_int as usize] +
                *v.offset(1 as libc::c_int as isize) *
                    (*info).lmvecs[1 as libc::c_int as
                                       usize][1 as libc::c_int as usize] +
                *v.offset(2 as libc::c_int as isize) *
                    (*info).lmvecs[1 as libc::c_int as
                                       usize][2 as libc::c_int as usize] +
                (*info).lmvecs[1 as libc::c_int as
                                   usize][3 as libc::c_int as usize] -
                (*info).lightmapmins[1 as libc::c_int as usize] as libc::c_int
                    as libc::c_float;
        t += (*surf).light_t as libc::c_float * sample_size;
        t += sample_size * 0.5f32;
        t /= tr.block_size as libc::c_float * sample_size;
        *v.offset(5 as libc::c_int as isize) = s;
        *v.offset(6 as libc::c_int as isize) = t;
        j += 1;
        v = v.offset(7 as libc::c_int as isize)
    };
}
// Check for intersecting decals on this surface
unsafe extern "C" fn R_DecalIntersect(mut decalinfo: *mut decalinfo_t,
                                      mut surf: *mut msurface_t,
                                      mut pcount: *mut libc::c_int)
 -> *mut decal_t {
    let mut texture: libc::c_int = 0;
    let mut plast: *mut decal_t = 0 as *mut decal_t;
    let mut pDecal: *mut decal_t = 0 as *mut decal_t;
    let mut decalExtents: [vec3_t; 2] = [[0.; 3]; 2];
    let mut lastArea: libc::c_float = 2 as libc::c_int as libc::c_float;
    let mut mapSize: [libc::c_int; 2] = [0; 2];
    plast = 0 as *mut decal_t;
    *pcount = 0 as libc::c_int;
    // (Same as R_SetupDecalClip).
    texture = (*decalinfo).m_iTexture;
    // precalculate the extents of decalinfo's decal in world space.
    R_GetDecalDimensions(texture,
                         &mut *mapSize.as_mut_ptr().offset(0 as libc::c_int as
                                                               isize),
                         &mut *mapSize.as_mut_ptr().offset(1 as libc::c_int as
                                                               isize));
    decalExtents[0 as libc::c_int as usize][0 as libc::c_int as usize] =
        (*decalinfo).m_Basis[0 as libc::c_int as
                                 usize][0 as libc::c_int as usize] *
            (mapSize[0 as libc::c_int as usize] as libc::c_float /
                 (*decalinfo).m_scale * 0.5f32);
    decalExtents[0 as libc::c_int as usize][1 as libc::c_int as usize] =
        (*decalinfo).m_Basis[0 as libc::c_int as
                                 usize][1 as libc::c_int as usize] *
            (mapSize[0 as libc::c_int as usize] as libc::c_float /
                 (*decalinfo).m_scale * 0.5f32);
    decalExtents[0 as libc::c_int as usize][2 as libc::c_int as usize] =
        (*decalinfo).m_Basis[0 as libc::c_int as
                                 usize][2 as libc::c_int as usize] *
            (mapSize[0 as libc::c_int as usize] as libc::c_float /
                 (*decalinfo).m_scale * 0.5f32);
    decalExtents[1 as libc::c_int as usize][0 as libc::c_int as usize] =
        (*decalinfo).m_Basis[1 as libc::c_int as
                                 usize][0 as libc::c_int as usize] *
            (mapSize[1 as libc::c_int as usize] as libc::c_float /
                 (*decalinfo).m_scale * 0.5f32);
    decalExtents[1 as libc::c_int as usize][1 as libc::c_int as usize] =
        (*decalinfo).m_Basis[1 as libc::c_int as
                                 usize][1 as libc::c_int as usize] *
            (mapSize[1 as libc::c_int as usize] as libc::c_float /
                 (*decalinfo).m_scale * 0.5f32);
    decalExtents[1 as libc::c_int as usize][2 as libc::c_int as usize] =
        (*decalinfo).m_Basis[1 as libc::c_int as
                                 usize][2 as libc::c_int as usize] *
            (mapSize[1 as libc::c_int as usize] as libc::c_float /
                 (*decalinfo).m_scale * 0.5f32);
    pDecal = (*surf).pdecals;
    while !pDecal.is_null() {
        texture = (*pDecal).texture as libc::c_int;
        // Don't steal bigger decals and replace them with smaller decals
		// Don't steal permanent decals
        if (*pDecal).flags as libc::c_int & 0x1 as libc::c_int == 0 {
            let mut testBasis: [vec3_t; 3] = [[0.; 3]; 3];
            let mut testPosition: [vec3_t; 2] = [[0.; 3]; 2];
            let mut testWorldScale: [libc::c_float; 2] = [0.; 2];
            let mut vDecalMin: vec2_t = [0.; 2];
            let mut vDecalMax: vec2_t = [0.; 2];
            let mut vUnionMin: vec2_t = [0.; 2];
            let mut vUnionMax: vec2_t = [0.; 2];
            R_SetupDecalTextureSpaceBasis(pDecal, surf, texture,
                                          testBasis.as_mut_ptr(),
                                          testWorldScale.as_mut_ptr());
            testPosition[0 as libc::c_int as usize][0 as libc::c_int as usize]
                =
                (*decalinfo).m_Position[0 as libc::c_int as usize] -
                    decalExtents[0 as libc::c_int as
                                     usize][0 as libc::c_int as usize];
            testPosition[0 as libc::c_int as usize][1 as libc::c_int as usize]
                =
                (*decalinfo).m_Position[1 as libc::c_int as usize] -
                    decalExtents[0 as libc::c_int as
                                     usize][1 as libc::c_int as usize];
            testPosition[0 as libc::c_int as usize][2 as libc::c_int as usize]
                =
                (*decalinfo).m_Position[2 as libc::c_int as usize] -
                    decalExtents[0 as libc::c_int as
                                     usize][2 as libc::c_int as usize];
            testPosition[1 as libc::c_int as usize][0 as libc::c_int as usize]
                =
                (*decalinfo).m_Position[0 as libc::c_int as usize] -
                    decalExtents[1 as libc::c_int as
                                     usize][0 as libc::c_int as usize];
            testPosition[1 as libc::c_int as usize][1 as libc::c_int as usize]
                =
                (*decalinfo).m_Position[1 as libc::c_int as usize] -
                    decalExtents[1 as libc::c_int as
                                     usize][1 as libc::c_int as usize];
            testPosition[1 as libc::c_int as usize][2 as libc::c_int as usize]
                =
                (*decalinfo).m_Position[2 as libc::c_int as usize] -
                    decalExtents[1 as libc::c_int as
                                     usize][2 as libc::c_int as usize];
            // Here, we project the min and max extents of the decal that got passed in into
			// this decal's (pDecal's) [0,0,1,1] clip space, just like we would if we were
			// clipping a triangle into pDecal's clip space.
            vDecalMin[0 as libc::c_int as usize] =
                testPosition[0 as libc::c_int as
                                 usize][0 as libc::c_int as usize] *
                    testBasis[0 as libc::c_int as
                                  usize][0 as libc::c_int as usize] +
                    testPosition[0 as libc::c_int as
                                     usize][1 as libc::c_int as usize] *
                        testBasis[0 as libc::c_int as
                                      usize][1 as libc::c_int as usize] +
                    testPosition[0 as libc::c_int as
                                     usize][2 as libc::c_int as usize] *
                        testBasis[0 as libc::c_int as
                                      usize][2 as libc::c_int as usize] -
                    (*pDecal).dx + 0.5f32;
            vDecalMin[1 as libc::c_int as usize] =
                testPosition[1 as libc::c_int as
                                 usize][0 as libc::c_int as usize] *
                    testBasis[1 as libc::c_int as
                                  usize][0 as libc::c_int as usize] +
                    testPosition[1 as libc::c_int as
                                     usize][1 as libc::c_int as usize] *
                        testBasis[1 as libc::c_int as
                                      usize][1 as libc::c_int as usize] +
                    testPosition[1 as libc::c_int as
                                     usize][2 as libc::c_int as usize] *
                        testBasis[1 as libc::c_int as
                                      usize][2 as libc::c_int as usize] -
                    (*pDecal).dy + 0.5f32;
            testPosition[0 as libc::c_int as usize][0 as libc::c_int as usize]
                =
                (*decalinfo).m_Position[0 as libc::c_int as usize] +
                    decalExtents[0 as libc::c_int as
                                     usize][0 as libc::c_int as usize];
            testPosition[0 as libc::c_int as usize][1 as libc::c_int as usize]
                =
                (*decalinfo).m_Position[1 as libc::c_int as usize] +
                    decalExtents[0 as libc::c_int as
                                     usize][1 as libc::c_int as usize];
            testPosition[0 as libc::c_int as usize][2 as libc::c_int as usize]
                =
                (*decalinfo).m_Position[2 as libc::c_int as usize] +
                    decalExtents[0 as libc::c_int as
                                     usize][2 as libc::c_int as usize];
            testPosition[1 as libc::c_int as usize][0 as libc::c_int as usize]
                =
                (*decalinfo).m_Position[0 as libc::c_int as usize] +
                    decalExtents[1 as libc::c_int as
                                     usize][0 as libc::c_int as usize];
            testPosition[1 as libc::c_int as usize][1 as libc::c_int as usize]
                =
                (*decalinfo).m_Position[1 as libc::c_int as usize] +
                    decalExtents[1 as libc::c_int as
                                     usize][1 as libc::c_int as usize];
            testPosition[1 as libc::c_int as usize][2 as libc::c_int as usize]
                =
                (*decalinfo).m_Position[2 as libc::c_int as usize] +
                    decalExtents[1 as libc::c_int as
                                     usize][2 as libc::c_int as usize];
            vDecalMax[0 as libc::c_int as usize] =
                testPosition[0 as libc::c_int as
                                 usize][0 as libc::c_int as usize] *
                    testBasis[0 as libc::c_int as
                                  usize][0 as libc::c_int as usize] +
                    testPosition[0 as libc::c_int as
                                     usize][1 as libc::c_int as usize] *
                        testBasis[0 as libc::c_int as
                                      usize][1 as libc::c_int as usize] +
                    testPosition[0 as libc::c_int as
                                     usize][2 as libc::c_int as usize] *
                        testBasis[0 as libc::c_int as
                                      usize][2 as libc::c_int as usize] -
                    (*pDecal).dx + 0.5f32;
            vDecalMax[1 as libc::c_int as usize] =
                testPosition[1 as libc::c_int as
                                 usize][0 as libc::c_int as usize] *
                    testBasis[1 as libc::c_int as
                                  usize][0 as libc::c_int as usize] +
                    testPosition[1 as libc::c_int as
                                     usize][1 as libc::c_int as usize] *
                        testBasis[1 as libc::c_int as
                                      usize][1 as libc::c_int as usize] +
                    testPosition[1 as libc::c_int as
                                     usize][2 as libc::c_int as usize] *
                        testBasis[1 as libc::c_int as
                                      usize][2 as libc::c_int as usize] -
                    (*pDecal).dy + 0.5f32;
            // Now figure out the part of the projection that intersects pDecal's
			// clip box [0,0,1,1].
            vUnionMin[0 as libc::c_int as usize] =
                (if vDecalMin[0 as libc::c_int as usize] >
                        0 as libc::c_int as libc::c_float {
                     vDecalMin[0 as libc::c_int as usize]
                 } else { 0 as libc::c_int as libc::c_float });
            vUnionMin[1 as libc::c_int as usize] =
                (if vDecalMin[1 as libc::c_int as usize] >
                        0 as libc::c_int as libc::c_float {
                     vDecalMin[1 as libc::c_int as usize]
                 } else { 0 as libc::c_int as libc::c_float });
            vUnionMax[0 as libc::c_int as usize] =
                (if vDecalMax[0 as libc::c_int as usize] <
                        1 as libc::c_int as libc::c_float {
                     vDecalMax[0 as libc::c_int as usize]
                 } else { 1 as libc::c_int as libc::c_float });
            vUnionMax[1 as libc::c_int as usize] =
                (if vDecalMax[1 as libc::c_int as usize] <
                        1 as libc::c_int as libc::c_float {
                     vDecalMax[1 as libc::c_int as usize]
                 } else { 1 as libc::c_int as libc::c_float });
            if vUnionMin[0 as libc::c_int as usize] <
                   1 as libc::c_int as libc::c_float &&
                   vUnionMin[1 as libc::c_int as usize] <
                       1 as libc::c_int as libc::c_float &&
                   vUnionMax[0 as libc::c_int as usize] >
                       0 as libc::c_int as libc::c_float &&
                   vUnionMax[1 as libc::c_int as usize] >
                       0 as libc::c_int as libc::c_float {
                // Figure out how much of this intersects the (0,0) - (1,1) bbox.
                let mut flArea: libc::c_float =
                    (vUnionMax[0 as libc::c_int as usize] -
                         vUnionMin[1 as libc::c_int as usize]) *
                        (vUnionMax[1 as libc::c_int as usize] -
                             vUnionMin[1 as libc::c_int as usize]);
                if flArea > 0.6f32 {
                    *pcount += 1 as libc::c_int;
                    if plast.is_null() || flArea <= lastArea {
                        plast = pDecal;
                        lastArea = flArea
                    }
                }
            }
        }
        pDecal = (*pDecal).pnext
    }
    return plast;
}
/*
====================
R_DecalCreatePoly

creates mesh for decal on first rendering
====================
*/
#[no_mangle]
pub unsafe extern "C" fn R_DecalCreatePoly(mut decalinfo: *mut decalinfo_t,
                                           mut pdecal: *mut decal_t,
                                           mut surf: *mut msurface_t)
 -> *mut glpoly_t {
    let mut lnumverts: libc::c_int = 0;
    let mut poly: *mut glpoly_t = 0 as *mut glpoly_t;
    let mut v: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut i: libc::c_int = 0;
    return 0 as *mut glpoly_t;
}
// Add the decal to the surface's list of decals.
unsafe extern "C" fn R_AddDecalToSurface(mut pdecal: *mut decal_t,
                                         mut surf: *mut msurface_t,
                                         mut decalinfo: *mut decalinfo_t) {
    let mut pold: *mut decal_t = 0 as *mut decal_t;
    (*pdecal).pnext = 0 as *mut decal_t;
    pold = (*surf).pdecals;
    if !pold.is_null() {
        while !(*pold).pnext.is_null() { pold = (*pold).pnext }
        (*pold).pnext = pdecal
    } else { (*surf).pdecals = pdecal }
    // force surface cache rebuild
    (*surf).dlightframe = tr.framecount + 1 as libc::c_int;
    // tag surface
    (*pdecal).psurface = surf;
    // at this point decal are linked with surface
	// and will be culled, drawing and sorting
	// together with surface
    // alloc clipped poly for decal
    R_DecalCreatePoly(decalinfo, pdecal, surf);
    //R_AddDecalVBO( pdecal, surf );
}
unsafe extern "C" fn R_DecalCreate(mut decalinfo: *mut decalinfo_t,
                                   mut surf: *mut msurface_t,
                                   mut x: libc::c_float,
                                   mut y: libc::c_float) {
    let mut pdecal: *mut decal_t = 0 as *mut decal_t; // ???
    let mut pold: *mut decal_t = 0 as *mut decal_t; // r_decals == 0 ???
    let mut count: libc::c_int = 0;
    let mut vertCount: libc::c_int = 0;
    if surf.is_null() { return }
    pold = R_DecalIntersect(decalinfo, surf, &mut count);
    if count < 6 as libc::c_int { pold = 0 as *mut decal_t }
    pdecal = R_DecalAlloc(pold);
    if pdecal.is_null() { return }
    (*pdecal).flags = (*decalinfo).m_Flags as libc::c_short;
    (*pdecal).position[0 as libc::c_int as usize] =
        (*decalinfo).m_Position[0 as libc::c_int as usize];
    (*pdecal).position[1 as libc::c_int as usize] =
        (*decalinfo).m_Position[1 as libc::c_int as usize];
    (*pdecal).position[2 as libc::c_int as usize] =
        (*decalinfo).m_Position[2 as libc::c_int as usize];
    (*pdecal).dx = x;
    (*pdecal).dy = y;
    // set scaling
    (*pdecal).scale = (*decalinfo).m_scale;
    (*pdecal).entityIndex = (*decalinfo).m_Entity as libc::c_short;
    (*pdecal).texture = (*decalinfo).m_iTexture as libc::c_short;
    // check to see if the decal actually intersects the surface
	// if not, then remove the decal
    R_DecalVertsClip(pdecal, surf, (*decalinfo).m_iTexture, &mut vertCount);
    if vertCount == 0 { R_DecalUnlink(pdecal); return }
    // add to the surface's list
    R_AddDecalToSurface(pdecal, surf, decalinfo);
}
#[no_mangle]
pub unsafe extern "C" fn R_DecalSurface(mut surf: *mut msurface_t,
                                        mut decalinfo: *mut decalinfo_t) {
    // get the texture associated with this surface
    let mut tex: *mut mtexinfo_t = (*surf).texinfo;
    let mut decal: *mut decal_t = (*surf).pdecals;
    let mut textureU: vec4_t = [0.; 4];
    let mut textureV: vec4_t = [0.; 4];
    let mut s: libc::c_float = 0.;
    let mut t: libc::c_float = 0.;
    let mut w: libc::c_float = 0.;
    let mut h: libc::c_float = 0.;
    let mut state: connstate_t =
        Some(gEngfuncs.EngineGetParm.expect("non-null function pointer")).expect("non-null function pointer")(PARM_CONNSTATE
                                                                                                                  as
                                                                                                                  libc::c_int,
                                                                                                              0
                                                                                                                  as
                                                                                                                  libc::c_int)
            as connstate_t;
    // we in restore mode
    if state as libc::c_uint == ca_connected as libc::c_int as libc::c_uint ||
           state as libc::c_uint == ca_validate as libc::c_int as libc::c_uint
       {
        // NOTE: we may have the decal on this surface that come from another level.
		// check duplicate with same position and texture
        while !decal.is_null() {
            if (*decal).position[0 as libc::c_int as usize] ==
                   (*decalinfo).m_Position[0 as libc::c_int as usize] &&
                   (*decal).position[1 as libc::c_int as usize] ==
                       (*decalinfo).m_Position[1 as libc::c_int as usize] &&
                   (*decal).position[2 as libc::c_int as usize] ==
                       (*decalinfo).m_Position[2 as libc::c_int as usize] &&
                   (*decal).texture as libc::c_int == (*decalinfo).m_iTexture
               {
                return
            } // decal already exists, don't place it again
            decal = (*decal).pnext
        }
    }
    textureU[0 as libc::c_int as usize] =
        (*tex).vecs[0 as libc::c_int as usize][0 as libc::c_int as usize];
    textureU[1 as libc::c_int as usize] =
        (*tex).vecs[0 as libc::c_int as usize][1 as libc::c_int as usize];
    textureU[2 as libc::c_int as usize] =
        (*tex).vecs[0 as libc::c_int as usize][2 as libc::c_int as usize];
    textureU[3 as libc::c_int as usize] =
        (*tex).vecs[0 as libc::c_int as usize][3 as libc::c_int as usize];
    textureV[0 as libc::c_int as usize] =
        (*tex).vecs[1 as libc::c_int as usize][0 as libc::c_int as usize];
    textureV[1 as libc::c_int as usize] =
        (*tex).vecs[1 as libc::c_int as usize][1 as libc::c_int as usize];
    textureV[2 as libc::c_int as usize] =
        (*tex).vecs[1 as libc::c_int as usize][2 as libc::c_int as usize];
    textureV[3 as libc::c_int as usize] =
        (*tex).vecs[1 as libc::c_int as usize][3 as libc::c_int as usize];
    // project decal center into the texture space of the surface
    s =
        (*decalinfo).m_Position[0 as libc::c_int as usize] *
            textureU[0 as libc::c_int as usize] +
            (*decalinfo).m_Position[1 as libc::c_int as usize] *
                textureU[1 as libc::c_int as usize] +
            (*decalinfo).m_Position[2 as libc::c_int as usize] *
                textureU[2 as libc::c_int as usize] +
            textureU[3 as libc::c_int as usize] -
            (*surf).texturemins[0 as libc::c_int as usize] as libc::c_int as
                libc::c_float;
    t =
        (*decalinfo).m_Position[0 as libc::c_int as usize] *
            textureV[0 as libc::c_int as usize] +
            (*decalinfo).m_Position[1 as libc::c_int as usize] *
                textureV[1 as libc::c_int as usize] +
            (*decalinfo).m_Position[2 as libc::c_int as usize] *
                textureV[2 as libc::c_int as usize] +
            textureV[3 as libc::c_int as usize] -
            (*surf).texturemins[1 as libc::c_int as usize] as libc::c_int as
                libc::c_float;
    // Determine the decal basis (measured in world space)
	// Note that the decal basis vectors 0 and 1 will always lie in the same
	// plane as the texture space basis vectorstextureVecsTexelsPerWorldUnits.
    R_DecalComputeBasis(surf, (*decalinfo).m_Flags,
                        (*decalinfo).m_Basis.as_mut_ptr());
    // Compute an effective width and height (axis aligned) in the parent texture space
	// How does this work? decalBasis[0] represents the u-direction (width)
	// of the decal measured in world space, decalBasis[1] represents the
	// v-direction (height) measured in world space.
	// textureVecsTexelsPerWorldUnits[0] represents the u direction of
	// the surface's texture space measured in world space (with the appropriate
	// scale factor folded in), and textureVecsTexelsPerWorldUnits[1]
	// represents the texture space v direction. We want to find the dimensions (w,h)
	// of a square measured in texture space, axis aligned to that coordinate system.
	// All we need to do is to find the components of the decal edge vectors
	// (decalWidth * decalBasis[0], decalHeight * decalBasis[1])
	// in texture coordinates:
    w =
        __tg_fabs((*decalinfo).m_decalWidth as libc::c_float *
                      (textureU[0 as libc::c_int as usize] *
                           (*decalinfo).m_Basis[0 as libc::c_int as
                                                    usize][0 as libc::c_int as
                                                               usize] +
                           textureU[1 as libc::c_int as usize] *
                               (*decalinfo).m_Basis[0 as libc::c_int as
                                                        usize][1 as
                                                                   libc::c_int
                                                                   as usize] +
                           textureU[2 as libc::c_int as usize] *
                               (*decalinfo).m_Basis[0 as libc::c_int as
                                                        usize][2 as
                                                                   libc::c_int
                                                                   as usize]))
            +
            __tg_fabs((*decalinfo).m_decalHeight as libc::c_float *
                          (textureU[0 as libc::c_int as usize] *
                               (*decalinfo).m_Basis[1 as libc::c_int as
                                                        usize][0 as
                                                                   libc::c_int
                                                                   as usize] +
                               textureU[1 as libc::c_int as usize] *
                                   (*decalinfo).m_Basis[1 as libc::c_int as
                                                            usize][1 as
                                                                       libc::c_int
                                                                       as
                                                                       usize]
                               +
                               textureU[2 as libc::c_int as usize] *
                                   (*decalinfo).m_Basis[1 as libc::c_int as
                                                            usize][2 as
                                                                       libc::c_int
                                                                       as
                                                                       usize]));
    h =
        __tg_fabs((*decalinfo).m_decalWidth as libc::c_float *
                      (textureV[0 as libc::c_int as usize] *
                           (*decalinfo).m_Basis[0 as libc::c_int as
                                                    usize][0 as libc::c_int as
                                                               usize] +
                           textureV[1 as libc::c_int as usize] *
                               (*decalinfo).m_Basis[0 as libc::c_int as
                                                        usize][1 as
                                                                   libc::c_int
                                                                   as usize] +
                           textureV[2 as libc::c_int as usize] *
                               (*decalinfo).m_Basis[0 as libc::c_int as
                                                        usize][2 as
                                                                   libc::c_int
                                                                   as usize]))
            +
            __tg_fabs((*decalinfo).m_decalHeight as libc::c_float *
                          (textureV[0 as libc::c_int as usize] *
                               (*decalinfo).m_Basis[1 as libc::c_int as
                                                        usize][0 as
                                                                   libc::c_int
                                                                   as usize] +
                               textureV[1 as libc::c_int as usize] *
                                   (*decalinfo).m_Basis[1 as libc::c_int as
                                                            usize][1 as
                                                                       libc::c_int
                                                                       as
                                                                       usize]
                               +
                               textureV[2 as libc::c_int as usize] *
                                   (*decalinfo).m_Basis[1 as libc::c_int as
                                                            usize][2 as
                                                                       libc::c_int
                                                                       as
                                                                       usize]));
    // move s,t to upper left corner
    s -= w * 0.5f32;
    t -= h * 0.5f32;
    // Is this rect within the surface? -- tex width & height are unsigned
    if s <= -w || t <= -h ||
           s >
               (*surf).extents[0 as libc::c_int as usize] as libc::c_int as
                   libc::c_float + w ||
           t >
               (*surf).extents[1 as libc::c_int as usize] as libc::c_int as
                   libc::c_float + h {
        return
        // nope
    }
    // stamp it
    R_DecalCreate(decalinfo, surf, s, t);
}
//-----------------------------------------------------------------------------
// iterate over all surfaces on a node, looking for surfaces to decal
//-----------------------------------------------------------------------------
unsafe extern "C" fn R_DecalNodeSurfaces(mut model: *mut model_t,
                                         mut node: *mut mnode_t,
                                         mut decalinfo: *mut decalinfo_t) {
    // iterate over all surfaces in the node
    let mut surf: *mut msurface_t = 0 as *mut msurface_t;
    let mut i: libc::c_int = 0;
    surf =
        (*model).surfaces.offset((*node).firstsurface as libc::c_int as
                                     isize);
    i = 0 as libc::c_int;
    while i < (*node).numsurfaces as libc::c_int {
        // never apply decals on the water or sky surfaces
        if !((*surf).flags as libc::c_uint &
                 ((1 as libc::c_uint) << 4 as libc::c_int |
                      (1 as libc::c_uint) << 2 as libc::c_int |
                      (1 as libc::c_uint) << 6 as libc::c_int) != 0) {
            // we can implement alpha testing without stencil
		//if( surf->flags & SURF_TRANSPARENT && !glState.stencilEnabled )
			//continue;
            R_DecalSurface(surf, decalinfo);
        }
        i += 1;
        surf = surf.offset(1)
    };
}
//-----------------------------------------------------------------------------
// Recursive routine to find surface to apply a decal to.  World coordinates of
// the decal are passed in r_recalpos like the rest of the engine.  This should
// be called through R_DecalShoot()
//-----------------------------------------------------------------------------
unsafe extern "C" fn R_DecalNode(mut model: *mut model_t,
                                 mut node: *mut mnode_t,
                                 mut decalinfo: *mut decalinfo_t) {
    let mut splitplane: *mut mplane_t = 0 as *mut mplane_t;
    let mut dist: libc::c_float = 0.;
    if node.is_null() {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 b"../ref_soft/r_decals.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 724 as
                                                                     libc::c_int);
    }
    if (*node).contents < 0 as libc::c_int {
        // hit a leaf
        return
    }
    splitplane = (*node).plane;
    dist =
        (*decalinfo).m_Position[0 as libc::c_int as usize] *
            (*splitplane).normal[0 as libc::c_int as usize] +
            (*decalinfo).m_Position[1 as libc::c_int as usize] *
                (*splitplane).normal[1 as libc::c_int as usize] +
            (*decalinfo).m_Position[2 as libc::c_int as usize] *
                (*splitplane).normal[2 as libc::c_int as usize] -
            (*splitplane).dist;
    // This is arbitrarily set to 10 right now. In an ideal world we'd have the
	// exact surface but we don't so, this tells me which planes are "sort of
	// close" to the gunshot -- the gunshot is actually 4 units in front of the
	// wall (see dlls\weapons.cpp). We also need to check to see if the decal
	// actually intersects the texture space of the surface, as this method tags
	// parallel surfaces in the same node always.
	// JAY: This still tags faces that aren't correct at edges because we don't
	// have a surface normal
    if dist > (*decalinfo).m_Size as libc::c_float {
        R_DecalNode(model, (*node).children[0 as libc::c_int as usize],
                    decalinfo);
    } else if dist < -(*decalinfo).m_Size as libc::c_float {
        R_DecalNode(model, (*node).children[1 as libc::c_int as usize],
                    decalinfo);
    } else {
        if dist < 4 as libc::c_int as libc::c_float &&
               dist > -(4 as libc::c_int) as libc::c_float {
            R_DecalNodeSurfaces(model, node, decalinfo);
        }
        R_DecalNode(model, (*node).children[0 as libc::c_int as usize],
                    decalinfo);
        R_DecalNode(model, (*node).children[1 as libc::c_int as usize],
                    decalinfo);
    };
}
// Shoots a decal onto the surface of the BSP.  position is the center of the decal in world coords
#[no_mangle]
pub unsafe extern "C" fn R_DecalShoot(mut textureIndex: libc::c_int,
                                      mut entityIndex: libc::c_int,
                                      mut modelIndex: libc::c_int,
                                      mut pos: *mut vec_t,
                                      mut flags: libc::c_int,
                                      mut scale: libc::c_float) {
    let mut decalInfo: decalinfo_t =
        decalinfo_t{m_Position: [0.; 3],
                    m_pModel: 0 as *mut model_t,
                    m_iTexture: 0,
                    m_Size: 0,
                    m_Flags: 0,
                    m_Entity: 0,
                    m_scale: 0.,
                    m_decalWidth: 0,
                    m_decalHeight: 0,
                    m_Basis: [[0.; 3]; 3],}; // always use #0 hull
    let mut ent: *mut cl_entity_t = 0 as *mut cl_entity_t;
    let mut model: *mut model_t = 0 as *mut model_t;
    let mut width: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    let mut hull: *mut hull_t = 0 as *mut hull_t;
    if textureIndex <= 0 as libc::c_int || textureIndex >= 4096 as libc::c_int
       {
        gEngfuncs.Con_Printf.expect("non-null function pointer")(b"^1Error:^7 Decal has invalid texture!\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char);
        return
    }
    if entityIndex > 0 as libc::c_int {
        ent =
            gEngfuncs.GetEntityByIndex.expect("non-null function pointer")(entityIndex);
        if modelIndex > 0 as libc::c_int {
            model =
                gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(modelIndex)
        } else if !ent.is_null() {
            model =
                gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")((*ent).curstate.modelindex)
        } else { return }
    } else if modelIndex > 0 as libc::c_int {
        model =
            gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(modelIndex)
    } else {
        model =
            gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1
                                                                                 as
                                                                                 libc::c_int)
    }
    if model.is_null() { return }
    if (*model).type_0 as libc::c_int != mod_brush as libc::c_int {
        gEngfuncs.Con_Printf.expect("non-null function pointer")(b"^1Error:^7 Decals must hit mod_brush!\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char);
        return
    }
    decalInfo.m_pModel = model;
    hull =
        &mut *(*model).hulls.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut hull_t;
    // NOTE: all the decals at 'first shoot' placed into local space of parent entity
	// and won't transform again on a next restore, levelchange etc
    if !ent.is_null() && flags & 0x80 as libc::c_int == 0 {
        let mut pos_l: vec3_t = [0.; 3];
        // transform decal position in local bmodel space
        if !((*ent).angles[0 as libc::c_int as usize] == 0.0f32 &&
                 (*ent).angles[1 as libc::c_int as usize] == 0.0f32 &&
                 (*ent).angles[2 as libc::c_int as usize] == 0.0f32) {
            let mut matrix: matrix4x4 = [[0.; 4]; 4];
            Matrix4x4_CreateFromEntity(matrix.as_mut_ptr(),
                                       (*ent).angles.as_mut_ptr() as
                                           *const vec_t,
                                       (*ent).origin.as_mut_ptr() as
                                           *const vec_t, 1.0f32);
            Matrix4x4_VectorITransform(matrix.as_mut_ptr() as
                                           *const [vec_t; 4],
                                       pos as *const libc::c_float,
                                       pos_l.as_mut_ptr());
        } else {
            pos_l[0 as libc::c_int as usize] =
                *pos.offset(0 as libc::c_int as isize) -
                    (*ent).origin[0 as libc::c_int as usize];
            pos_l[1 as libc::c_int as usize] =
                *pos.offset(1 as libc::c_int as isize) -
                    (*ent).origin[1 as libc::c_int as usize];
            pos_l[2 as libc::c_int as usize] =
                *pos.offset(2 as libc::c_int as isize) -
                    (*ent).origin[2 as libc::c_int as usize]
        }
        decalInfo.m_Position[0 as libc::c_int as usize] =
            pos_l[0 as libc::c_int as usize];
        decalInfo.m_Position[1 as libc::c_int as usize] =
            pos_l[1 as libc::c_int as usize];
        decalInfo.m_Position[2 as libc::c_int as usize] =
            pos_l[2 as libc::c_int as usize];
        // decal position moved into local space
        flags = flags | 0x80 as libc::c_int
    } else {
        // already in local space
        decalInfo.m_Position[0 as libc::c_int as usize] =
            *pos.offset(0 as libc::c_int as isize);
        decalInfo.m_Position[1 as libc::c_int as usize] =
            *pos.offset(1 as libc::c_int as isize);
        decalInfo.m_Position[2 as libc::c_int as usize] =
            *pos.offset(2 as libc::c_int as isize)
    }
    // this decal must use landmark for correct transition
	// because their model exist only in world-space
    if (*model).flags as libc::c_uint &
           (1 as libc::c_uint) << 1 as libc::c_int == 0 {
        flags = flags | 0x2 as libc::c_int
    }
    // more state used by R_DecalNode()
    decalInfo.m_iTexture = textureIndex;
    decalInfo.m_Entity = entityIndex;
    decalInfo.m_Flags = flags;
    R_GetDecalDimensions(textureIndex, &mut width, &mut height);
    decalInfo.m_Size = width >> 1 as libc::c_int;
    if height >> 1 as libc::c_int > decalInfo.m_Size {
        decalInfo.m_Size = height >> 1 as libc::c_int
    }
    decalInfo.m_scale =
        if scale >= 0.01f32 {
            if scale < 16.0f32 { scale } else { 16.0f32 }
        } else { 0.01f32 };
    // compute the decal dimensions in world space
    decalInfo.m_decalWidth =
        (width as libc::c_float / decalInfo.m_scale) as libc::c_int;
    decalInfo.m_decalHeight =
        (height as libc::c_float / decalInfo.m_scale) as libc::c_int;
    R_DecalNode(model,
                &mut *(*model).nodes.offset((*hull).firstclipnode as isize),
                &mut decalInfo);
}
// Build the vertex list for a decal on a surface and clip it to the surface.
// This is a template so it can work on world surfaces and dynamic displacement
// triangles the same way.
#[no_mangle]
pub unsafe extern "C" fn R_DecalSetupVerts(mut pDecal: *mut decal_t,
                                           mut surf: *mut msurface_t,
                                           mut texture: libc::c_int,
                                           mut outCount: *mut libc::c_int)
 -> *mut libc::c_float {
    let mut p: *mut glpoly_t = (*pDecal).polys;
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut v: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut v2: *mut libc::c_float = 0 as *mut libc::c_float;
    if !p.is_null() {
        v = g_DecalClipVerts[0 as libc::c_int as usize].as_mut_ptr();
        count = (*p).numverts;
        v2 = (*p).verts[0 as libc::c_int as usize].as_mut_ptr();
        // if we have mesh so skip clipping and just copy vertexes out (perf)
        i = 0 as libc::c_int;
        while i < count {
            *v.offset(0 as libc::c_int as isize) =
                *v2.offset(0 as libc::c_int as isize);
            *v.offset(1 as libc::c_int as isize) =
                *v2.offset(1 as libc::c_int as isize);
            *v.offset(2 as libc::c_int as isize) =
                *v2.offset(2 as libc::c_int as isize);
            *v.offset(3 as libc::c_int as isize) =
                *v2.offset(3 as libc::c_int as isize);
            *v.offset(4 as libc::c_int as isize) =
                *v2.offset(4 as libc::c_int as isize);
            *v.offset(5 as libc::c_int as isize) =
                *v2.offset(5 as libc::c_int as isize);
            *v.offset(6 as libc::c_int as isize) =
                *v2.offset(6 as libc::c_int as isize);
            i += 1;
            v = v.offset(7 as libc::c_int as isize);
            v2 = v2.offset(7 as libc::c_int as isize)
        }
        // restore pointer
        v = g_DecalClipVerts[0 as libc::c_int as usize].as_mut_ptr()
    } else {
        v = R_DecalVertsClip(pDecal, surf, texture, &mut count);
        R_DecalVertsLight(v, surf, count);
    }
    if !outCount.is_null() { *outCount = count }
    return v;
}
/*
=============================================================

  DECALS SERIALIZATION

=============================================================
*/
unsafe extern "C" fn R_DecalUnProject(mut pdecal: *mut decal_t,
                                      mut entry: *mut decallist_t)
 -> qboolean {
    if pdecal.is_null() || (*pdecal).psurface.is_null() { return false_0 }
    (*entry).position[0 as libc::c_int as usize] =
        (*pdecal).position[0 as libc::c_int as usize];
    (*entry).position[1 as libc::c_int as usize] =
        (*pdecal).position[1 as libc::c_int as usize];
    (*entry).position[2 as libc::c_int as usize] =
        (*pdecal).position[2 as libc::c_int as usize];
    (*entry).entityIndex = (*pdecal).entityIndex;
    // Grab surface plane equation
    if (*(*pdecal).psurface).flags as libc::c_uint &
           (1 as libc::c_uint) << 1 as libc::c_int != 0 {
        (*entry).impactPlaneNormal[0 as libc::c_int as usize] =
            -(*(*(*pdecal).psurface).plane).normal[0 as libc::c_int as usize];
        (*entry).impactPlaneNormal[1 as libc::c_int as usize] =
            -(*(*(*pdecal).psurface).plane).normal[1 as libc::c_int as usize];
        (*entry).impactPlaneNormal[2 as libc::c_int as usize] =
            -(*(*(*pdecal).psurface).plane).normal[2 as libc::c_int as usize]
    } else {
        (*entry).impactPlaneNormal[0 as libc::c_int as usize] =
            (*(*(*pdecal).psurface).plane).normal[0 as libc::c_int as usize];
        (*entry).impactPlaneNormal[1 as libc::c_int as usize] =
            (*(*(*pdecal).psurface).plane).normal[1 as libc::c_int as usize];
        (*entry).impactPlaneNormal[2 as libc::c_int as usize] =
            (*(*(*pdecal).psurface).plane).normal[2 as libc::c_int as usize]
    }
    return true_0;
}
//-----------------------------------------------------------------------------
// Purpose:
// Input  : *pList -
//			count -
// Output : static int
//-----------------------------------------------------------------------------
unsafe extern "C" fn DecalListAdd(mut pList: *mut decallist_t,
                                  mut count: libc::c_int) -> libc::c_int {
    let mut tmp: vec3_t = [0.; 3]; // Merge
    let mut pdecal: *mut decallist_t = 0 as *mut decallist_t;
    let mut i: libc::c_int = 0;
    pdecal = pList.offset(count as isize);
    i = 0 as libc::c_int;
    while i < count {
        if Q_strncmp((*pdecal).name.as_mut_ptr(),
                     (*pList.offset(i as isize)).name.as_mut_ptr(),
                     99999 as libc::c_int) == 0 &&
               (*pdecal).entityIndex as libc::c_int ==
                   (*pList.offset(i as isize)).entityIndex as libc::c_int {
            tmp[0 as libc::c_int as usize] =
                (*pdecal).position[0 as libc::c_int as usize] -
                    (*pList.offset(i as
                                       isize)).position[0 as libc::c_int as
                                                            usize];
            tmp[1 as libc::c_int as usize] =
                (*pdecal).position[1 as libc::c_int as usize] -
                    (*pList.offset(i as
                                       isize)).position[1 as libc::c_int as
                                                            usize];
            tmp[2 as libc::c_int as usize] =
                (*pdecal).position[2 as libc::c_int as usize] -
                    (*pList.offset(i as
                                       isize)).position[2 as libc::c_int as
                                                            usize];
            if __tg_sqrt(tmp[0 as libc::c_int as usize] *
                             tmp[0 as libc::c_int as usize] +
                             tmp[1 as libc::c_int as usize] *
                                 tmp[1 as libc::c_int as usize] +
                             tmp[2 as libc::c_int as usize] *
                                 tmp[2 as libc::c_int as usize]) <
                   2 as libc::c_int as libc::c_float {
                return count
            }
        }
        i += 1
    }
    // this is a new decal
    return count + 1 as libc::c_int;
}
unsafe extern "C" fn DecalDepthCompare(mut a: *const libc::c_void,
                                       mut b: *const libc::c_void)
 -> libc::c_int {
    let mut elem1: *const decallist_t = 0 as *const decallist_t;
    let mut elem2: *const decallist_t = 0 as *const decallist_t;
    elem1 = a as *const decallist_t;
    elem2 = b as *const decallist_t;
    if (*elem1).depth as libc::c_int > (*elem2).depth as libc::c_int {
        return 1 as libc::c_int
    }
    if ((*elem1).depth as libc::c_int) < (*elem2).depth as libc::c_int {
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
//-----------------------------------------------------------------------------
// Purpose: Called by CSaveRestore::SaveClientState
// Input  : *pList -
// Output : int
//-----------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn R_CreateDecalList(mut pList: *mut decallist_t)
 -> libc::c_int {
    let mut total: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut depth: libc::c_int = 0;
    //	return 0; // crash on changelevel. API bug?
    if !gEngfuncs.pfnGetModelByIndex.expect("non-null function pointer")(1 as
                                                                             libc::c_int).is_null()
       {
        i = 0 as libc::c_int;
        while i < 4096 as libc::c_int {
            let mut decal: *mut decal_t =
                &mut *gDecalPool.as_mut_ptr().offset(i as isize) as
                    *mut decal_t;
            let mut pdecals: *mut decal_t = 0 as *mut decal_t;
            // decal is in use and is not a custom decal
            if !((*decal).psurface.is_null() ||
                     (*decal).flags as libc::c_int & 0x20 as libc::c_int != 0)
               {
                // compute depth
                depth = 0 as libc::c_int;
                pdecals = (*(*decal).psurface).pdecals;
                while !pdecals.is_null() && pdecals != decal {
                    depth += 1;
                    pdecals = (*pdecals).pnext
                }
                (*pList.offset(total as isize)).depth = depth as byte;
                (*pList.offset(total as isize)).flags =
                    (*decal).flags as byte;
                (*pList.offset(total as isize)).scale = (*decal).scale;
                R_DecalUnProject(decal, &mut *pList.offset(total as isize));
                COM_FileBase((*R_GetTexture((*decal).texture as
                                                libc::c_uint)).name.as_mut_ptr(),
                             (*pList.offset(total as
                                                isize)).name.as_mut_ptr());
                // check to see if the decal should be added
                total = DecalListAdd(pList, total)
            }
            i += 1
        }
        if (*gEngfuncs.drawFuncs).R_CreateStudioDecalList.is_some() {
            total +=
                (*gEngfuncs.drawFuncs).R_CreateStudioDecalList.expect("non-null function pointer")(pList,
                                                                                                   total)
        }
    }
    // sort the decals lowest depth first, so they can be re-applied in order
    qsort(pList as *mut libc::c_void, total as size_t,
          ::std::mem::size_of::<decallist_t>() as libc::c_ulong,
          Some(DecalDepthCompare as
                   unsafe extern "C" fn(_: *const libc::c_void,
                                        _: *const libc::c_void)
                       -> libc::c_int));
    return total;
}
/*
===============
R_DecalRemoveAll

remove all decals with specified texture
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_DecalRemoveAll(mut textureIndex: libc::c_int) {
    let mut pdecal: *mut decal_t = 0 as *mut decal_t; // out of bounds
    let mut i: libc::c_int = 0;
    if textureIndex < 0 as libc::c_int || textureIndex >= 4096 as libc::c_int
       {
        return
    }
    i = 0 as libc::c_int;
    while i < gDecalCount {
        pdecal =
            &mut *gDecalPool.as_mut_ptr().offset(i as isize) as *mut decal_t;
        // don't remove permanent decals
        if !(textureIndex == 0 &&
                 (*pdecal).flags as libc::c_int & 0x1 as libc::c_int != 0) {
            if textureIndex == 0 ||
                   (*pdecal).texture as libc::c_int == textureIndex {
                R_DecalUnlink(pdecal);
            }
        }
        i += 1
    };
}
/*
===============
R_EntityRemoveDecals

remove all decals from specified entity
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_EntityRemoveDecals(mut mod_0: *mut model_t) {
    let mut psurf: *mut msurface_t = 0 as *mut msurface_t;
    let mut p: *mut decal_t = 0 as *mut decal_t;
    let mut i: libc::c_int = 0;
    if mod_0.is_null() ||
           (*mod_0).type_0 as libc::c_int != mod_brush as libc::c_int {
        return
    }
    psurf =
        &mut *(*mod_0).surfaces.offset((*mod_0).firstmodelsurface as isize) as
            *mut msurface_t;
    i = 0 as libc::c_int;
    while i < (*mod_0).nummodelsurfaces {
        p = (*psurf).pdecals;
        while !p.is_null() { R_DecalUnlink(p); p = (*p).pnext }
        i += 1;
        psurf = psurf.offset(1)
    };
}
/*
===============
R_ClearAllDecals

remove all decals from anything
used for full decals restart
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_ClearAllDecals() {
    let mut pdecal: *mut decal_t = 0 as *mut decal_t;
    let mut i: libc::c_int = 0;
    // because gDecalCount may be zeroed after recach the decal limit
    i = 0 as libc::c_int;
    while i < 4096 as libc::c_int {
        pdecal =
            &mut *gDecalPool.as_mut_ptr().offset(i as isize) as *mut decal_t;
        R_DecalUnlink(pdecal);
        i += 1
    }
    if (*gEngfuncs.drawFuncs).R_ClearStudioDecals.is_some() {
        (*gEngfuncs.drawFuncs).R_ClearStudioDecals.expect("non-null function pointer")();
    };
}
