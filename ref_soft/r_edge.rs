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
    fn Matrix4x4_CreateFromEntity(out: *mut [vec_t; 4], angles: *const vec_t,
                                  origin: *const vec_t, scale: libc::c_float);
    #[no_mangle]
    static mut vec3_origin: vec3_t;
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
    fn R_TransformFrustum();
    #[no_mangle]
    fn D_DrawZSpans(pspans: *mut espan_t);
    #[no_mangle]
    static mut r_screenwidth: libc::c_int;
    #[no_mangle]
    static mut d_viewbuffer: *mut pixel_t;
    #[no_mangle]
    static mut d_ziorigin: libc::c_float;
    #[no_mangle]
    static mut d_zistepv: libc::c_float;
    #[no_mangle]
    static mut d_zistepu: libc::c_float;
    #[no_mangle]
    fn D_DrawSpans16(pspans: *mut espan_t);
    #[no_mangle]
    static mut bbextentt: fixed16_t;
    #[no_mangle]
    static mut bbextents: fixed16_t;
    #[no_mangle]
    static mut sadjust: fixed16_t;
    #[no_mangle]
    static mut tadjust: fixed16_t;
    #[no_mangle]
    static mut d_tdivzstepv: libc::c_float;
    #[no_mangle]
    static mut ycenter: libc::c_float;
    #[no_mangle]
    static mut d_tdivzstepu: libc::c_float;
    #[no_mangle]
    static mut xcenter: libc::c_float;
    #[no_mangle]
    static mut d_tdivzorigin: libc::c_float;
    #[no_mangle]
    static mut d_sdivzstepv: libc::c_float;
    #[no_mangle]
    static mut d_sdivzstepu: libc::c_float;
    #[no_mangle]
    static mut d_sdivzorigin: libc::c_float;
    #[no_mangle]
    static mut yscaleinv: libc::c_float;
    #[no_mangle]
    static mut xscaleinv: libc::c_float;
    #[no_mangle]
    fn TransformVector(in_0: *mut vec_t, out: *mut vec_t);
    #[no_mangle]
    static mut cachewidth: libc::c_int;
    #[no_mangle]
    static mut cacheblock: *mut pixel_t;
    #[no_mangle]
    fn D_CacheSurface(surface: *mut msurface_t, miplevel_0: libc::c_int)
     -> *mut surfcache_t;
    #[no_mangle]
    static mut d_minmip: libc::c_int;
    #[no_mangle]
    static mut d_scalemip: [libc::c_float; 3];
    // FIXME: should go away
    #[no_mangle]
    fn R_RotateBmodel();
    #[no_mangle]
    fn Turbulent8(pspan: *mut espan_t);
    #[no_mangle]
    fn NonTurbulent8(pspan: *mut espan_t);
    #[no_mangle]
    static mut sw_clearcolor: *mut cvar_t;
    #[no_mangle]
    static mut sw_drawflat: *mut cvar_t;
    #[no_mangle]
    static mut r_numallocatededges: libc::c_int;
    #[no_mangle]
    static mut sw_draworder: *mut cvar_t;
    #[no_mangle]
    fn D_AlphaSpans16(pspan: *mut espan_t);
    #[no_mangle]
    fn D_AddSpans16(pspan: *mut espan_t);
    #[no_mangle]
    fn D_BlendSpans16(pspan: *mut espan_t, alpha: libc::c_int);
    #[no_mangle]
    fn TurbulentZ8(pspan: *mut espan_t, alpha: libc::c_int);
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
pub type uintptr_t = libc::c_ulong;
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
pub type ref_api_t = ref_api_s;
pub type fixed16_t = libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct espan_s {
    pub u: libc::c_int,
    pub v: libc::c_int,
    pub count: libc::c_int,
    pub pnext: *mut espan_s,
}
pub type espan_t = espan_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct surf_s {
    pub next: *mut surf_s,
    pub prev: *mut surf_s,
    pub spans: *mut espan_s,
    pub key: libc::c_int,
    pub last_u: libc::c_int,
    pub spanstate: libc::c_int,
    pub flags: libc::c_int,
    pub msurf: *mut msurface_t,
    pub entity: *mut cl_entity_t,
    pub nearzi: libc::c_float,
    pub insubmodel: qboolean,
    pub d_ziorigin: libc::c_float,
    pub d_zistepu: libc::c_float,
    pub d_zistepv: libc::c_float,
    pub pad: [libc::c_int; 2],
}
pub type surf_t = surf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct edge_s {
    pub u: fixed16_t,
    pub u_step: fixed16_t,
    pub prev: *mut edge_s,
    pub next: *mut edge_s,
    pub surfs: [libc::c_ushort; 2],
    pub nextremove: *mut edge_s,
    pub nearzi: libc::c_float,
    pub owner: *mut medge_t,
}
pub type edge_t = edge_s;
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
// r_edge.c
#[no_mangle]
pub unsafe extern "C" fn R_SurfacePatch() { }
#[no_mangle]
pub unsafe extern "C" fn R_EdgeCodeStart() { }
#[no_mangle]
pub unsafe extern "C" fn R_EdgeCodeEnd() { }
#[no_mangle]
pub static mut auxedges: *mut edge_t = 0 as *const edge_t as *mut edge_t;
#[no_mangle]
pub static mut r_edges: *mut edge_t = 0 as *const edge_t as *mut edge_t;
#[no_mangle]
pub static mut edge_p: *mut edge_t = 0 as *const edge_t as *mut edge_t;
#[no_mangle]
pub static mut edge_max: *mut edge_t = 0 as *const edge_t as *mut edge_t;
#[no_mangle]
pub static mut surfaces: *mut surf_t = 0 as *const surf_t as *mut surf_t;
#[no_mangle]
pub static mut surface_p: *mut surf_t = 0 as *const surf_t as *mut surf_t;
#[no_mangle]
pub static mut surf_max: *mut surf_t = 0 as *const surf_t as *mut surf_t;
// surfaces are generated in back to front order by the bsp, so if a surf
// pointer is greater than another one, it should be drawn in front
// surfaces[1] is the background, and is used as the active surface stack
#[no_mangle]
pub static mut newedges: [*mut edge_t; 1200] =
    [0 as *const edge_t as *mut edge_t; 1200];
#[no_mangle]
pub static mut removeedges: [*mut edge_t; 1200] =
    [0 as *const edge_t as *mut edge_t; 1200];
#[no_mangle]
pub static mut span_p: *mut espan_t = 0 as *const espan_t as *mut espan_t;
#[no_mangle]
pub static mut max_span_p: *mut espan_t = 0 as *const espan_t as *mut espan_t;
#[no_mangle]
pub static mut r_currentkey: libc::c_int = 0;
#[no_mangle]
pub static mut current_iv: libc::c_int = 0;
#[no_mangle]
pub static mut edge_head_u_shift20: libc::c_int = 0;
#[no_mangle]
pub static mut edge_tail_u_shift20: libc::c_int = 0;
static mut pdrawfunc: Option<unsafe extern "C" fn() -> ()> = None;
#[no_mangle]
pub static mut edge_head: edge_t =
    edge_t{u: 0,
           u_step: 0,
           prev: 0 as *const edge_s as *mut edge_s,
           next: 0 as *const edge_s as *mut edge_s,
           surfs: [0; 2],
           nextremove: 0 as *const edge_s as *mut edge_s,
           nearzi: 0.,
           owner: 0 as *const medge_t as *mut medge_t,};
#[no_mangle]
pub static mut edge_tail: edge_t =
    edge_t{u: 0,
           u_step: 0,
           prev: 0 as *const edge_s as *mut edge_s,
           next: 0 as *const edge_s as *mut edge_s,
           surfs: [0; 2],
           nextremove: 0 as *const edge_s as *mut edge_s,
           nearzi: 0.,
           owner: 0 as *const medge_t as *mut medge_t,};
#[no_mangle]
pub static mut edge_aftertail: edge_t =
    edge_t{u: 0,
           u_step: 0,
           prev: 0 as *const edge_s as *mut edge_s,
           next: 0 as *const edge_s as *mut edge_s,
           surfs: [0; 2],
           nextremove: 0 as *const edge_s as *mut edge_s,
           nearzi: 0.,
           owner: 0 as *const medge_t as *mut medge_t,};
#[no_mangle]
pub static mut edge_sentinel: edge_t =
    edge_t{u: 0,
           u_step: 0,
           prev: 0 as *const edge_s as *mut edge_s,
           next: 0 as *const edge_s as *mut edge_s,
           surfs: [0; 2],
           nextremove: 0 as *const edge_s as *mut edge_s,
           nearzi: 0.,
           owner: 0 as *const medge_t as *mut medge_t,};
static mut fv: libc::c_float = 0.;
static mut miplevel: libc::c_int = 0;
#[no_mangle]
pub static mut scale_for_mip: libc::c_float = 0.;
/*
===============================================================================

EDGE SCANNING

===============================================================================
*/
/*
==============
R_BeginEdgeFrame
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_BeginEdgeFrame() {
    let mut v: libc::c_int = 0; // background is surface 1,
    edge_p = r_edges;
    edge_max =
        &mut *r_edges.offset(r_numallocatededges as isize) as *mut edge_t;
    surface_p =
        &mut *surfaces.offset(2 as libc::c_int as isize) as *mut surf_t;
    //  surface 0 is a dummy
    let ref mut fresh0 =
        (*surfaces.offset(1 as libc::c_int as
                              isize)).spans; // no background spans yet
    *fresh0 = 0 as *mut espan_s; // SURF_DRAWBACKGROUND;
    (*surfaces.offset(1 as libc::c_int as isize)).flags = 0 as libc::c_int;
    // put the background behind everything in the world
    if (*sw_draworder).value != 0. {
        pdrawfunc =
            Some(R_GenerateSpansBackward as unsafe extern "C" fn() -> ());
        (*surfaces.offset(1 as libc::c_int as isize)).key = 0 as libc::c_int;
        r_currentkey = 1 as libc::c_int
    } else {
        pdrawfunc = Some(R_GenerateSpans as unsafe extern "C" fn() -> ());
        (*surfaces.offset(1 as libc::c_int as isize)).key =
            0x7fffffff as libc::c_int;
        r_currentkey = 0 as libc::c_int
    }
    // FIXME: set with memset
    v = RI.vrect.y;
    while v < RI.vrectbottom {
        removeedges[v as usize] = 0 as *mut edge_t;
        newedges[v as usize] = removeedges[v as usize];
        v += 1
    };
}
/*
==============
R_InsertNewEdges

Adds the edges in the linked list edgestoadd, adding them to the edges in the
linked list edgelist.  edgestoadd is assumed to be sorted on u, and non-empty (this is actually newedges[v]).  edgelist is assumed to be sorted on u, with a
sentinel at the end (actually, this is the active edge table starting at
edge_head.next).
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_InsertNewEdges(mut edgestoadd: *mut edge_t,
                                          mut edgelist: *mut edge_t) {
    let mut next_edge: *mut edge_t = 0 as *mut edge_t;
    loop  {
        next_edge = (*edgestoadd).next;
        loop  {
            edgelist.is_null();
            if (*edgelist).u >= (*edgestoadd).u { break ; }
            edgelist = (*edgelist).next;
            if (*edgelist).u >= (*edgestoadd).u { break ; }
            edgelist = (*edgelist).next;
            if (*edgelist).u >= (*edgestoadd).u { break ; }
            edgelist = (*edgelist).next;
            if (*edgelist).u >= (*edgestoadd).u { break ; }
            edgelist = (*edgelist).next
        }
        // insert edgestoadd before edgelist
        (*edgestoadd).next = edgelist;
        (*edgestoadd).prev = (*edgelist).prev;
        (*(*edgelist).prev).next = edgestoadd;
        (*edgelist).prev = edgestoadd;
        edgestoadd = next_edge;
        if edgestoadd.is_null() { break ; }
    };
}
// !id386
/*
==============
R_RemoveEdges
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_RemoveEdges(mut pedge: *mut edge_t) {
    loop  {
        (*(*pedge).next).prev = (*pedge).prev;
        (*(*pedge).prev).next = (*pedge).next;
        pedge = (*pedge).nextremove;
        if pedge.is_null() { break ; }
    };
}
// !id386
/*
==============
R_StepActiveU
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_StepActiveU(mut pedge: *mut edge_t) {
    let mut pnext_edge: *mut edge_t = 0 as *mut edge_t;
    let mut pwedge: *mut edge_t = 0 as *mut edge_t;
    loop  {
        (*pedge).u += (*pedge).u_step;
        if !((*pedge).u < (*(*pedge).prev).u) {
            pedge = (*pedge).next;
            (*pedge).u += (*pedge).u_step;
            if !((*pedge).u < (*(*pedge).prev).u) {
                pedge = (*pedge).next;
                (*pedge).u += (*pedge).u_step;
                if !((*pedge).u < (*(*pedge).prev).u) {
                    pedge = (*pedge).next;
                    (*pedge).u += (*pedge).u_step;
                    if !((*pedge).u < (*(*pedge).prev).u) {
                        pedge = (*pedge).next;
                        continue ;
                    }
                }
            }
        }
        if pedge == &mut edge_aftertail as *mut edge_t { return }
        // push it back to keep it sorted
        pnext_edge = (*pedge).next;
        // pull the edge out of the edge list
        (*(*pedge).next).prev = (*pedge).prev;
        (*(*pedge).prev).next = (*pedge).next;
        // find out where the edge goes in the edge list
        pwedge = (*(*pedge).prev).prev;
        //	if( !pwedge )
		//	return;
        while (*pwedge).u > (*pedge).u {
            pwedge = (*pwedge).prev
            //if( !pwedge )
				//return;
        }
        // put the edge back into the edge list
        (*pedge).next = (*pwedge).next;
        (*pedge).prev = pwedge;
        (*(*pedge).next).prev = pedge;
        (*pwedge).next = pedge;
        pedge = pnext_edge;
        if pedge == &mut edge_tail as *mut edge_t { return }
    };
}
// !id386
/*
==============
R_CleanupSpan
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_CleanupSpan() {
    let mut surf: *mut surf_t = 0 as *mut surf_t;
    let mut iu: libc::c_int = 0;
    let mut span: *mut espan_t = 0 as *mut espan_t;
    // now that we've reached the right edge of the screen, we're done with any
// unfinished surfaces, so emit a span for whatever's on top
    surf = (*surfaces.offset(1 as libc::c_int as isize)).next;
    iu = edge_tail_u_shift20;
    if iu > (*surf).last_u {
        let fresh1 = span_p;
        span_p = span_p.offset(1);
        span = fresh1;
        (*span).u = (*surf).last_u;
        (*span).count = iu - (*span).u;
        (*span).v = current_iv;
        (*span).pnext = (*surf).spans;
        (*surf).spans = span
    }
    loop 
         // reset spanstate for all surfaces in the surface stack
         {
        (*surf).spanstate = 0 as libc::c_int;
        surf = (*surf).next;
        if !(surf !=
                 &mut *surfaces.offset(1 as libc::c_int as isize) as
                     *mut surf_t) {
            break ;
        }
    };
}
/*
==============
R_LeadingEdgeBackwards
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_LeadingEdgeBackwards(mut edge: *mut edge_t) {
    let mut current_block: u64;
    let mut span: *mut espan_t = 0 as *mut espan_t;
    let mut surf: *mut surf_t = 0 as *mut surf_t;
    let mut surf2: *mut surf_t = 0 as *mut surf_t;
    let mut iu: libc::c_int = 0;
    // it's adding a new surface in, so find the correct place
    surf =
        &mut *surfaces.offset(*(*edge).surfs.as_mut_ptr().offset(1 as
                                                                     libc::c_int
                                                                     as isize)
                                  as isize) as *mut surf_t;
    // don't start a span if this is an inverted span, with the end
// edge preceding the start edge (that is, we've already seen the
// end edge)
    (*surf).spanstate += 1;
    if (*surf).spanstate == 1 as libc::c_int {
        surf2 = (*surfaces.offset(1 as libc::c_int as isize)).next;
        if (*surf).key > (*surf2).key {
            current_block = 15801985133671466941;
        } else if (*surf).insubmodel as libc::c_uint != 0 &&
                      (*surf).key == (*surf2).key {
            current_block = 15801985133671466941;
        } else {
            loop  {
                loop  {
                    surf2 = (*surf2).next;
                    if !((*surf).key < (*surf2).key) { break ; }
                }
                if !((*surf).key == (*surf2).key) { break ; }
                // if it's two surfaces on the same plane, the one that's already
	// active is in front, so keep going unless it's a bmodel
                // if it's two surfaces on the same plane, the one that's already
		// active is in front, so keep going unless it's a bmodel
                if !((*surf).insubmodel as u64 == 0) { break ; }
                // must be two bmodels in the same leaf; don't care which is really
		// in front, because they'll never be farthest anyway
            }
            current_block = 14181132614457621749;
        }
        match current_block {
            15801985133671466941 =>
            // must be two bmodels in the same leaf; don't care, because they'll
		// never be farthest anyway
            // emit a span (obscures current top)
            {
                iu = (*edge).u >> 20 as libc::c_int;
                if iu > (*surf2).last_u {
                    let fresh2 = span_p;
                    span_p = span_p.offset(1);
                    span = fresh2;
                    (*span).u = (*surf2).last_u;
                    (*span).count = iu - (*span).u;
                    (*span).v = current_iv;
                    (*span).pnext = (*surf2).spans;
                    (*surf2).spans = span
                }
                // set last_u on the new span
                (*surf).last_u = iu
            }
            _ => { }
        }
        // insert before surf2
        (*surf).next = surf2;
        (*surf).prev = (*surf2).prev;
        (*(*surf2).prev).next = surf;
        (*surf2).prev = surf
    };
}
/*
==============
R_TrailingEdge
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_TrailingEdge(mut surf: *mut surf_t,
                                        mut edge: *mut edge_t) {
    let mut span: *mut espan_t = 0 as *mut espan_t;
    let mut iu: libc::c_int = 0;
    // don't generate a span if this is an inverted span, with the end
// edge preceding the start edge (that is, we haven't seen the
// start edge yet)
    (*surf).spanstate -= 1;
    if (*surf).spanstate == 0 as libc::c_int {
        if surf == (*surfaces.offset(1 as libc::c_int as isize)).next {
            // emit a span (current top going away)
            iu = (*edge).u >> 20 as libc::c_int;
            if iu > (*surf).last_u {
                let fresh3 = span_p;
                span_p = span_p.offset(1);
                span = fresh3;
                (*span).u = (*surf).last_u;
                (*span).count = iu - (*span).u;
                (*span).v = current_iv;
                (*span).pnext = (*surf).spans;
                (*surf).spans = span
            }
            // set last_u on the surface below
            (*(*surf).next).last_u = iu
        }
        (*(*surf).prev).next = (*surf).next;
        (*(*surf).next).prev = (*surf).prev
    };
}
/*
==============
R_LeadingEdge
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_LeadingEdge(mut edge: *mut edge_t) {
    let mut current_block: u64;
    let mut span: *mut espan_t = 0 as *mut espan_t;
    let mut surf: *mut surf_t = 0 as *mut surf_t;
    let mut surf2: *mut surf_t = 0 as *mut surf_t;
    let mut iu: libc::c_int = 0;
    let mut fu: libc::c_float = 0.;
    let mut newzi: libc::c_float = 0.;
    let mut testzi: libc::c_float = 0.;
    let mut newzitop: libc::c_float = 0.;
    let mut newzibottom: libc::c_float = 0.;
    if (*edge).surfs[1 as libc::c_int as usize] != 0 {
        // it's adding a new surface in, so find the correct place
        surf =
            &mut *surfaces.offset(*(*edge).surfs.as_mut_ptr().offset(1 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)
                                      as isize) as *mut surf_t;
        // don't start a span if this is an inverted span, with the end
	// edge preceding the start edge (that is, we've already seen the
	// end edge)
        (*surf).spanstate += 1;
        if (*surf).spanstate == 1 as libc::c_int {
            surf2 = (*surfaces.offset(1 as libc::c_int as isize)).next;
            if (*surf).key < (*surf2).key {
                current_block = 14231283526678333949;
            } else {
                // if it's two surfaces on the same plane, the one that's already
		// active is in front, so keep going unless it's a bmodel
                if (*surf).insubmodel as libc::c_uint != 0 &&
                       (*surf).key == (*surf2).key {
                    // must be two bmodels in the same leaf; sort on 1/z
                    fu =
                        ((*edge).u - 0xfffff as libc::c_int) as libc::c_float
                            *
                            (1.0f32 /
                                 0x100000 as libc::c_int as libc::c_float);
                    newzi =
                        (*surf).d_ziorigin + fv * (*surf).d_zistepv +
                            fu * (*surf).d_zistepu;
                    newzibottom = newzi * 0.99f32;
                    testzi =
                        (*surf2).d_ziorigin + fv * (*surf2).d_zistepv +
                            fu * (*surf2).d_zistepu;
                    if newzibottom >= testzi {
                        current_block = 14231283526678333949;
                    } else {
                        newzitop = newzi * 1.01f32;
                        if newzitop >= testzi {
                            if (*surf).d_zistepu >= (*surf2).d_zistepu {
                                current_block = 14231283526678333949;
                            } else { current_block = 18027285485958405329; }
                        } else { current_block = 18027285485958405329; }
                    }
                } else { current_block = 18027285485958405329; }
                match current_block {
                    14231283526678333949 => { }
                    _ => {
                        loop  {
                            loop  {
                                surf2 = (*surf2).next;
                                if !((*surf).key > (*surf2).key) { break ; }
                            }
                            if !((*surf).key == (*surf2).key) { break ; }
                            // if it's two surfaces on the same plane, the one that's already
			// active is in front, so keep going unless it's a bmodel
                            if (*surf).insubmodel as u64 == 0 { continue ; }
                            // must be two bmodels in the same leaf; sort on 1/z
                            fu =
                                ((*edge).u - 0xfffff as libc::c_int) as
                                    libc::c_float *
                                    (1.0f32 /
                                         0x100000 as libc::c_int as
                                             libc::c_float);
                            newzi =
                                (*surf).d_ziorigin + fv * (*surf).d_zistepv +
                                    fu * (*surf).d_zistepu;
                            newzibottom = newzi * 0.99f32;
                            testzi =
                                (*surf2).d_ziorigin + fv * (*surf2).d_zistepv
                                    + fu * (*surf2).d_zistepu;
                            if newzibottom >= testzi { break ; }
                            newzitop = newzi * 1.01f32;
                            if !(newzitop >= testzi) { continue ; }
                            if (*surf).d_zistepu >= (*surf2).d_zistepu {
                                break ;
                            }
                        }
                        current_block = 4668288037983582731;
                    }
                }
            }
            match current_block {
                14231283526678333949 => {
                    // emit a span (obscures current top)
                    iu = (*edge).u >> 20 as libc::c_int;
                    if iu > (*surf2).last_u {
                        let fresh4 = span_p;
                        span_p = span_p.offset(1);
                        span = fresh4;
                        (*span).u = (*surf2).last_u;
                        (*span).count = iu - (*span).u;
                        (*span).v = current_iv;
                        (*span).pnext = (*surf2).spans;
                        (*surf2).spans = span
                    }
                    // set last_u on the new span
                    (*surf).last_u = iu
                }
                _ => { }
            }
            // insert before surf2
            (*surf).next = surf2;
            (*surf).prev = (*surf2).prev;
            (*(*surf2).prev).next = surf;
            (*surf2).prev = surf
        }
    };
}
/*
==============
R_GenerateSpans
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_GenerateSpans() {
    let mut edge: *mut edge_t = 0 as *mut edge_t;
    let mut surf: *mut surf_t = 0 as *mut surf_t;
    // clear active surfaces to just the background surface
    let ref mut fresh5 = (*surfaces.offset(1 as libc::c_int as isize)).prev;
    *fresh5 = &mut *surfaces.offset(1 as libc::c_int as isize) as *mut surf_t;
    let ref mut fresh6 = (*surfaces.offset(1 as libc::c_int as isize)).next;
    *fresh6 = *fresh5;
    (*surfaces.offset(1 as libc::c_int as isize)).last_u =
        edge_head_u_shift20;
    let mut current_block_5: u64;
    // generate spans
    edge = edge_head.next;
    while edge != &mut edge_tail as *mut edge_t {
        if (*edge).surfs[0 as libc::c_int as usize] != 0 {
            // it has a left surface, so a surface is going away for this span
            surf =
                &mut *surfaces.offset(*(*edge).surfs.as_mut_ptr().offset(0 as
                                                                             libc::c_int
                                                                             as
                                                                             isize)
                                          as isize) as *mut surf_t;
            R_TrailingEdge(surf, edge);
            if (*edge).surfs[1 as libc::c_int as usize] == 0 {
                current_block_5 = 10680521327981672866;
            } else { current_block_5 = 5720623009719927633; }
        } else { current_block_5 = 5720623009719927633; }
        match current_block_5 {
            5720623009719927633 => { R_LeadingEdge(edge); }
            _ => { }
        }
        edge = (*edge).next
    }
    R_CleanupSpan();
}
// !id386
/*
==============
R_GenerateSpansBackward
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_GenerateSpansBackward() {
    let mut edge: *mut edge_t = 0 as *mut edge_t;
    // clear active surfaces to just the background surface
    let ref mut fresh7 = (*surfaces.offset(1 as libc::c_int as isize)).prev;
    *fresh7 = &mut *surfaces.offset(1 as libc::c_int as isize) as *mut surf_t;
    let ref mut fresh8 = (*surfaces.offset(1 as libc::c_int as isize)).next;
    *fresh8 = *fresh7;
    (*surfaces.offset(1 as libc::c_int as isize)).last_u =
        edge_head_u_shift20;
    // generate spans
    edge = edge_head.next;
    while edge != &mut edge_tail as *mut edge_t {
        if (*edge).surfs[0 as libc::c_int as usize] != 0 {
            R_TrailingEdge(&mut *surfaces.offset(*(*edge).surfs.as_mut_ptr().offset(0
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)
                                                     as isize), edge);
        }
        if (*edge).surfs[1 as libc::c_int as usize] != 0 {
            R_LeadingEdgeBackwards(edge);
        }
        edge = (*edge).next
    }
    R_CleanupSpan();
}
/*
==============
R_ScanEdges

Input:
newedges[] array
	this has links to edges, which have links to surfaces

Output:
Each surface has a linked list of its visible spans
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_ScanEdges() {
    let mut iv: libc::c_int = 0;
    let mut bottom: libc::c_int = 0;
    let mut basespans: [byte; 144032] = [0; 144032];
    let mut basespan_p: *mut espan_t = 0 as *mut espan_t;
    let mut s: *mut surf_t = 0 as *mut surf_t;
    basespan_p =
        (basespans.as_mut_ptr().offset(32 as libc::c_int as
                                           isize).offset(-(1 as libc::c_int as
                                                               isize)) as
             libc::c_long &
             !(32 as libc::c_int - 1 as libc::c_int) as libc::c_long) as
            *mut espan_t;
    max_span_p =
        &mut *basespan_p.offset((6000 as libc::c_int - RI.vrect.width) as
                                    isize) as *mut espan_t;
    span_p = basespan_p;
    // clear active edges to just the background edges around the whole screen
// FIXME: most of this only needs to be set up once
    edge_head.u =
        RI.vrect.x <<
            20 as libc::c_int; // (r_refdef.vrectright << 20) + 0xFFFFF;
    edge_head_u_shift20 = edge_head.u >> 20 as libc::c_int; // force a move
    edge_head.u_step = 0 as libc::c_int;
    edge_head.prev = 0 as *mut edge_s;
    edge_head.next = &mut edge_tail;
    edge_head.surfs[0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_ushort;
    edge_head.surfs[1 as libc::c_int as usize] =
        1 as libc::c_int as libc::c_ushort;
    edge_tail.u =
        (RI.vrectright << 20 as libc::c_int) + 0xfffff as libc::c_int;
    edge_tail_u_shift20 = edge_tail.u >> 20 as libc::c_int;
    edge_tail.u_step = 0 as libc::c_int;
    edge_tail.prev = &mut edge_head;
    edge_tail.next = &mut edge_aftertail;
    edge_tail.surfs[0 as libc::c_int as usize] =
        1 as libc::c_int as libc::c_ushort;
    edge_tail.surfs[1 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_ushort;
    edge_aftertail.u = -(1 as libc::c_int);
    edge_aftertail.u_step = 0 as libc::c_int;
    edge_aftertail.next = &mut edge_sentinel;
    edge_aftertail.prev = &mut edge_tail;
    // FIXME: do we need this now that we clamp x in r_draw.c?
    edge_sentinel.u =
        (2000 as libc::c_int) <<
            20 as libc::c_int; // make sure nothing sorts past this
    edge_sentinel.prev = &mut edge_aftertail;
    //
// process all scan lines
//
    bottom = RI.vrectbottom - 1 as libc::c_int;
    iv = 0 as libc::c_int;
    while iv < bottom {
        current_iv = iv;
        fv = iv as libc::c_float;
        // mark that the head (background start) span is pre-included
        (*surfaces.offset(1 as libc::c_int as isize)).spanstate =
            1 as libc::c_int;
        if !newedges[iv as usize].is_null() {
            R_InsertNewEdges(newedges[iv as usize], edge_head.next);
        }
        Some(pdrawfunc.expect("non-null function pointer")).expect("non-null function pointer")();
        // flush the span list if we can't be sure we have enough spans left for
	// the next scan
        if span_p > max_span_p {
            D_DrawSurfaces();
            // clear the surface span pointers
            s =
                &mut *surfaces.offset(1 as libc::c_int as isize) as
                    *mut surf_t;
            while s < surface_p {
                (*s).spans = 0 as *mut espan_s;
                s = s.offset(1)
            }
            span_p = basespan_p
        }
        if !removeedges[iv as usize].is_null() {
            R_RemoveEdges(removeedges[iv as usize]);
        }
        if edge_head.next != &mut edge_tail as *mut edge_t {
            R_StepActiveU(edge_head.next);
        }
        iv += 1
    }
    // do the last scan (no need to step or sort or remove on the last scan)
    current_iv = iv;
    fv = iv as libc::c_float;
    // mark that the head (background start) span is pre-included
    (*surfaces.offset(1 as libc::c_int as isize)).spanstate =
        1 as libc::c_int;
    if !newedges[iv as usize].is_null() {
        R_InsertNewEdges(newedges[iv as usize], edge_head.next);
    }
    Some(pdrawfunc.expect("non-null function pointer")).expect("non-null function pointer")();
    // draw whatever's left in the span list
    D_DrawSurfaces();
}
/*
=========================================================================

SURFACE FILLING

=========================================================================
*/
#[no_mangle]
pub static mut pface: *mut msurface_t =
    0 as *const msurface_t as *mut msurface_t;
#[no_mangle]
pub static mut pcurrentcache: *mut surfcache_t =
    0 as *const surfcache_t as *mut surfcache_t;
#[no_mangle]
pub static mut transformed_modelorg: vec3_t = [0.; 3];
#[no_mangle]
pub static mut world_transformed_modelorg: vec3_t = [0.; 3];
#[no_mangle]
pub static mut local_modelorg: vec3_t = [0.; 3];
/*
=============
D_MipLevelForScale
=============
*/
#[no_mangle]
pub unsafe extern "C" fn D_MipLevelForScale(mut scale: libc::c_float)
 -> libc::c_int {
    let mut lmiplevel: libc::c_int = 0;
    if scale >= d_scalemip[0 as libc::c_int as usize] {
        lmiplevel = 0 as libc::c_int
    } else if scale >= d_scalemip[1 as libc::c_int as usize] {
        lmiplevel = 1 as libc::c_int
    } else if scale >= d_scalemip[2 as libc::c_int as usize] {
        lmiplevel = 2 as libc::c_int
    } else { lmiplevel = 3 as libc::c_int }
    if lmiplevel < d_minmip { lmiplevel = d_minmip }
    return lmiplevel;
}
/*
==============
D_FlatFillSurface

Simple single color fill with no texture mapping
==============
*/
#[no_mangle]
pub unsafe extern "C" fn D_FlatFillSurface(mut surf: *mut surf_t,
                                           mut color: libc::c_int) {
    let mut span: *mut espan_t = 0 as *mut espan_t;
    let mut pdest: *mut pixel_t = 0 as *mut pixel_t;
    let mut u: libc::c_int = 0;
    let mut u2: libc::c_int = 0;
    span = (*surf).spans;
    while !span.is_null() {
        pdest = d_viewbuffer.offset((r_screenwidth * (*span).v) as isize);
        u = (*span).u;
        u2 = (*span).u + (*span).count - 1 as libc::c_int;
        while u <= u2 { *pdest.offset(u as isize) = color as pixel_t; u += 1 }
        span = (*span).pnext
    };
}
/*
==============
D_CalcGradients
==============
*/
#[no_mangle]
pub unsafe extern "C" fn D_CalcGradients(mut pface_0: *mut msurface_t) {
    let mut pplane: *mut mplane_t = 0 as *mut mplane_t;
    let mut mipscale: libc::c_float = 0.;
    let mut p_temp1: vec3_t = [0.; 3];
    let mut p_saxis: vec3_t = [0.; 3];
    let mut p_taxis: vec3_t = [0.; 3];
    let mut t: libc::c_float = 0.;
    pplane = (*pface_0).plane;
    mipscale = 1.0f32 / ((1 as libc::c_int) << miplevel) as libc::c_float;
    if (*(*pface_0).texinfo).flags as libc::c_uint &
           (1 as libc::c_uint) << 1 as libc::c_int != 0 {
        TransformVector((*(*pface_0).texinfo).vecs[0 as libc::c_int as
                                                       usize].as_mut_ptr(),
                        p_saxis.as_mut_ptr());
        TransformVector((*(*pface_0).texinfo).vecs[1 as libc::c_int as
                                                       usize].as_mut_ptr(),
                        p_taxis.as_mut_ptr());
    } else {
        TransformVector((*(*pface_0).info).lmvecs[0 as libc::c_int as
                                                      usize].as_mut_ptr(),
                        p_saxis.as_mut_ptr());
        TransformVector((*(*pface_0).info).lmvecs[1 as libc::c_int as
                                                      usize].as_mut_ptr(),
                        p_taxis.as_mut_ptr());
    }
    t = xscaleinv * mipscale;
    d_sdivzstepu = p_saxis[0 as libc::c_int as usize] * t;
    d_tdivzstepu = p_taxis[0 as libc::c_int as usize] * t;
    t = yscaleinv * mipscale;
    d_sdivzstepv = -p_saxis[1 as libc::c_int as usize] * t;
    d_tdivzstepv = -p_taxis[1 as libc::c_int as usize] * t;
    d_sdivzorigin =
        p_saxis[2 as libc::c_int as usize] * mipscale - xcenter * d_sdivzstepu
            - ycenter * d_sdivzstepv;
    d_tdivzorigin =
        p_taxis[2 as libc::c_int as usize] * mipscale - xcenter * d_tdivzstepu
            - ycenter * d_tdivzstepv;
    p_temp1[0 as libc::c_int as usize] =
        transformed_modelorg[0 as libc::c_int as usize] * mipscale;
    p_temp1[1 as libc::c_int as usize] =
        transformed_modelorg[1 as libc::c_int as usize] * mipscale;
    p_temp1[2 as libc::c_int as usize] =
        transformed_modelorg[2 as libc::c_int as usize] * mipscale;
    t = 0x10000 as libc::c_int as libc::c_float * mipscale;
    if (*(*pface_0).texinfo).flags as libc::c_uint &
           (1 as libc::c_uint) << 1 as libc::c_int != 0 {
        sadjust =
            ((((p_temp1[0 as libc::c_int as usize] *
                    p_saxis[0 as libc::c_int as usize] +
                    p_temp1[1 as libc::c_int as usize] *
                        p_saxis[1 as libc::c_int as usize] +
                    p_temp1[2 as libc::c_int as usize] *
                        p_saxis[2 as libc::c_int as usize]) *
                   0x10000 as libc::c_int as libc::c_float + 0.5f32) as
                  fixed16_t -
                  (((*pface_0).texturemins[0 as libc::c_int as usize] as
                        libc::c_int) << 16 as libc::c_int >> miplevel)) as
                 libc::c_float +
                 (*(*pface_0).texinfo).vecs[0 as libc::c_int as
                                                usize][3 as libc::c_int as
                                                           usize] * t) as
                fixed16_t;
        tadjust =
            ((((p_temp1[0 as libc::c_int as usize] *
                    p_taxis[0 as libc::c_int as usize] +
                    p_temp1[1 as libc::c_int as usize] *
                        p_taxis[1 as libc::c_int as usize] +
                    p_temp1[2 as libc::c_int as usize] *
                        p_taxis[2 as libc::c_int as usize]) *
                   0x10000 as libc::c_int as libc::c_float + 0.5f32) as
                  fixed16_t -
                  (((*pface_0).texturemins[1 as libc::c_int as usize] as
                        libc::c_int) << 16 as libc::c_int >> miplevel)) as
                 libc::c_float +
                 (*(*pface_0).texinfo).vecs[1 as libc::c_int as
                                                usize][3 as libc::c_int as
                                                           usize] * t) as
                fixed16_t
    } else {
        sadjust =
            ((((p_temp1[0 as libc::c_int as usize] *
                    p_saxis[0 as libc::c_int as usize] +
                    p_temp1[1 as libc::c_int as usize] *
                        p_saxis[1 as libc::c_int as usize] +
                    p_temp1[2 as libc::c_int as usize] *
                        p_saxis[2 as libc::c_int as usize]) *
                   0x10000 as libc::c_int as libc::c_float + 0.5f32) as
                  fixed16_t -
                  (((*(*pface_0).info).lightmapmins[0 as libc::c_int as usize]
                        as libc::c_int) << 16 as libc::c_int >> miplevel)) as
                 libc::c_float +
                 (*(*pface_0).info).lmvecs[0 as libc::c_int as
                                               usize][3 as libc::c_int as
                                                          usize] * t) as
                fixed16_t;
        tadjust =
            ((((p_temp1[0 as libc::c_int as usize] *
                    p_taxis[0 as libc::c_int as usize] +
                    p_temp1[1 as libc::c_int as usize] *
                        p_taxis[1 as libc::c_int as usize] +
                    p_temp1[2 as libc::c_int as usize] *
                        p_taxis[2 as libc::c_int as usize]) *
                   0x10000 as libc::c_int as libc::c_float + 0.5f32) as
                  fixed16_t -
                  (((*(*pface_0).info).lightmapmins[1 as libc::c_int as usize]
                        as libc::c_int) << 16 as libc::c_int >> miplevel)) as
                 libc::c_float +
                 (*(*pface_0).info).lmvecs[1 as libc::c_int as
                                               usize][3 as libc::c_int as
                                                          usize] * t) as
                fixed16_t
    }
    // PGM - changing flow speed for non-warping textures.
    if (*pface_0).flags as libc::c_uint &
           (1 as libc::c_uint) << 6 as libc::c_int != 0 {
        if (*pface_0).flags as libc::c_uint &
               (1 as libc::c_uint) << 4 as libc::c_int != 0 {
            sadjust =
                (sadjust as libc::c_float +
                     0x10000 as libc::c_int as libc::c_float *
                         (-(128 as libc::c_int) as libc::c_float *
                              ((*gpGlobals).time * 0.25f32 -
                                   ((*gpGlobals).time * 0.25f32) as
                                       libc::c_int as libc::c_float))) as
                    fixed16_t
        } else {
            sadjust =
                (sadjust as libc::c_float +
                     0x10000 as libc::c_int as libc::c_float *
                         (-(128 as libc::c_int) as libc::c_float *
                              ((*gpGlobals).time * 0.77f32 -
                                   ((*gpGlobals).time * 0.77f32) as
                                       libc::c_int as libc::c_float))) as
                    fixed16_t
        }
        bbextents =
            (((*pface_0).extents[0 as libc::c_int as usize] as libc::c_int) <<
                 16 as libc::c_int >> miplevel) - 1 as libc::c_int
    } else {
        bbextents =
            (((*(*pface_0).info).lightextents[0 as libc::c_int as usize] as
                  libc::c_int) << 16 as libc::c_int >> miplevel) -
                1 as libc::c_int
    }
    bbextentt =
        (((*(*pface_0).info).lightextents[1 as libc::c_int as usize] as
              libc::c_int) << 16 as libc::c_int >> miplevel) -
            1 as libc::c_int;
    if (*(*pface_0).texinfo).flags as libc::c_uint &
           (1 as libc::c_uint) << 1 as libc::c_int != 0 {
        bbextents =
            (((*pface_0).extents[0 as libc::c_int as usize] as libc::c_int) <<
                 16 as libc::c_int >> miplevel) - 1 as libc::c_int;
        bbextentt =
            (((*pface_0).extents[1 as libc::c_int as usize] as libc::c_int) <<
                 16 as libc::c_int >> miplevel) - 1 as libc::c_int
    };
}
/*
==============
D_BackgroundSurf

The grey background filler seen when there is a hole in the map
==============
*/
#[no_mangle]
pub unsafe extern "C" fn D_BackgroundSurf(mut s: *mut surf_t) {
    // set up a gradient for the background surface that places it
// effectively at infinity distance from the viewpoint
    d_zistepu = 0 as libc::c_int as libc::c_float;
    d_zistepv = 0 as libc::c_int as libc::c_float;
    d_ziorigin = -0.9f64 as libc::c_float;
    D_FlatFillSurface(s,
                      (*sw_clearcolor).value as libc::c_int &
                          0xffff as libc::c_int);
    D_DrawZSpans((*s).spans);
}
/*
=================
D_TurbulentSurf
=================
*/
#[no_mangle]
pub unsafe extern "C" fn D_TurbulentSurf(mut s: *mut surf_t) {
    d_zistepu = (*s).d_zistepu;
    d_zistepv = (*s).d_zistepv;
    d_ziorigin = (*s).d_ziorigin;
    pface = (*s).msurf;
    miplevel = 0 as libc::c_int;
    cacheblock =
        (*R_GetTexture((*(*(*pface).texinfo).texture).gl_texturenum as
                           libc::c_uint)).pixels[0 as libc::c_int as usize];
    cachewidth = 64 as libc::c_int;
    if (*s).insubmodel as u64 != 0 {
        // FIXME: we don't want to do all this for every polygon!
	// TODO: store once at start of frame
        RI.currententity = (*s).entity; //FIXME: make this passed in to
        // FIXME: don't mess with the frustum,
        // make entity passed in
        local_modelorg[0 as libc::c_int as usize] =
            RI.vieworg[0 as libc::c_int as usize] -
                (*RI.currententity).origin[0 as libc::c_int as usize];
        local_modelorg[1 as libc::c_int as usize] =
            RI.vieworg[1 as libc::c_int as usize] -
                (*RI.currententity).origin[1 as libc::c_int as usize];
        local_modelorg[2 as libc::c_int as usize] =
            RI.vieworg[2 as libc::c_int as usize] -
                (*RI.currententity).origin[2 as libc::c_int as usize];
        TransformVector(local_modelorg.as_mut_ptr(),
                        transformed_modelorg.as_mut_ptr());
        R_RotateBmodel();
    }
    D_CalcGradients(pface);
    // R_RotateBmodel ()
    //============
//PGM
	// textures that aren't warping are just flowing. Use NonTurbulent8 instead
    if (*pface).flags as libc::c_uint &
           (1 as libc::c_uint) << 4 as libc::c_int == 0 {
        NonTurbulent8((*s).spans);
    } else { Turbulent8((*s).spans); }
    //PGM
//============
    D_DrawZSpans((*s).spans);
    if (*s).insubmodel as u64 != 0 {
        //
	// restore the old drawing state
	// FIXME: we don't want to do this every time!
	// TODO: speed up
	//
        RI.currententity = 0 as *mut cl_entity_t; // &r_worldentity;
        transformed_modelorg[0 as libc::c_int as usize] =
            world_transformed_modelorg[0 as libc::c_int as usize];
        transformed_modelorg[1 as libc::c_int as usize] =
            world_transformed_modelorg[1 as libc::c_int as usize];
        transformed_modelorg[2 as libc::c_int as usize] =
            world_transformed_modelorg[2 as libc::c_int as usize];
        RI.vforward[0 as libc::c_int as usize] =
            RI.base_vpn[0 as libc::c_int as usize];
        RI.vforward[1 as libc::c_int as usize] =
            RI.base_vpn[1 as libc::c_int as usize];
        RI.vforward[2 as libc::c_int as usize] =
            RI.base_vpn[2 as libc::c_int as usize];
        RI.vup[0 as libc::c_int as usize] =
            RI.base_vup[0 as libc::c_int as usize];
        RI.vup[1 as libc::c_int as usize] =
            RI.base_vup[1 as libc::c_int as usize];
        RI.vup[2 as libc::c_int as usize] =
            RI.base_vup[2 as libc::c_int as usize];
        RI.vright[0 as libc::c_int as usize] =
            RI.base_vright[0 as libc::c_int as usize];
        RI.vright[1 as libc::c_int as usize] =
            RI.base_vright[1 as libc::c_int as usize];
        RI.vright[2 as libc::c_int as usize] =
            RI.base_vright[2 as libc::c_int as usize];
        R_TransformFrustum();
    };
}
/*
==============
D_SkySurf
==============
*/
#[no_mangle]
pub unsafe extern "C" fn D_SkySurf(mut s: *mut surf_t) {
    pface = (*s).msurf;
    miplevel = 0 as libc::c_int;
    if (*(*pface).texinfo).texture.is_null() { return }
    cacheblock =
        (*R_GetTexture((*(*(*pface).texinfo).texture).gl_texturenum as
                           libc::c_uint)).pixels[0 as libc::c_int as usize];
    cachewidth = 256 as libc::c_int;
    d_zistepu = (*s).d_zistepu;
    d_zistepv = (*s).d_zistepv;
    d_ziorigin = (*s).d_ziorigin;
    D_CalcGradients(pface);
    D_DrawSpans16((*s).spans);
    // set up a gradient for the background surface that places it
// effectively at infinity distance from the viewpoint
    d_zistepu = 0 as libc::c_int as libc::c_float;
    d_zistepv = 0 as libc::c_int as libc::c_float;
    d_ziorigin = -0.9f64 as libc::c_float;
    D_DrawZSpans((*s).spans);
}
#[no_mangle]
pub static mut alphaspans: qboolean = false_0;
/*
==============
D_SolidSurf

Normal surface cached, texture mapped surface
==============
*/
#[no_mangle]
pub unsafe extern "C" fn D_AlphaSurf(mut s: *mut surf_t) {
    let mut alpha: libc::c_int = 0;
    d_zistepu = (*s).d_zistepu;
    d_zistepv = (*s).d_zistepv;
    d_ziorigin = (*s).d_ziorigin;
    if (*s).flags as libc::c_uint & (1 as libc::c_uint) << 2 as libc::c_int !=
           0 {
        return
    }
    if (*s).insubmodel as u64 == 0 {
        // wtf? how it is possible?
        return
    }
    // FIXME: we don't want to do all this for every polygon!
// TODO: store once at start of frame
    RI.currententity = (*s).entity; //FIXME: make this passed in to
    // R_RotateBmodel ()
    local_modelorg[0 as libc::c_int as usize] =
        RI.vieworg[0 as libc::c_int as usize] -
            (*RI.currententity).origin[0 as libc::c_int as
                                           usize]; // FIXME: don't mess with the frustum,
    local_modelorg[1 as libc::c_int as usize] =
        RI.vieworg[1 as libc::c_int as usize] -
            (*RI.currententity).origin[1 as libc::c_int as usize];
    local_modelorg[2 as libc::c_int as usize] =
        RI.vieworg[2 as libc::c_int as usize] -
            (*RI.currententity).origin[2 as libc::c_int as usize];
    TransformVector(local_modelorg.as_mut_ptr(),
                    transformed_modelorg.as_mut_ptr());
    R_RotateBmodel();
    // make entity passed in
    pface = (*s).msurf;
    if pface.is_null() { return }
    if (*pface).flags as libc::c_uint &
           (1 as libc::c_uint) << 6 as libc::c_int != 0 {
        miplevel = 1 as libc::c_int
    } else { miplevel = 0 as libc::c_int }
    alpha =
        (*RI.currententity).curstate.renderamt * 7 as libc::c_int /
            255 as libc::c_int;
    if alpha <= 0 as libc::c_int &&
           (*RI.currententity).curstate.renderamt > 0 as libc::c_int {
        alpha = 1 as libc::c_int
    }
    if (*s).flags as libc::c_uint & (1 as libc::c_uint) << 4 as libc::c_int !=
           0 {
        cacheblock =
            (*R_GetTexture((*(*(*pface).texinfo).texture).gl_texturenum as
                               libc::c_uint)).pixels[0 as libc::c_int as
                                                         usize];
        cachewidth = 64 as libc::c_int;
        D_CalcGradients(pface);
        TurbulentZ8((*s).spans, alpha);
    } else {
        // FIXME: make this passed in to D_CacheSurface
        pcurrentcache = D_CacheSurface(pface, miplevel);
        cacheblock = (*pcurrentcache).data.as_mut_ptr() as *mut pixel_t;
        cachewidth = (*pcurrentcache).width as libc::c_int;
        D_CalcGradients(pface);
        if (*RI.currententity).curstate.rendermode ==
               kRenderTransAlpha as libc::c_int {
            D_AlphaSpans16((*s).spans);
        } else if (*RI.currententity).curstate.rendermode ==
                      kRenderTransAdd as libc::c_int {
            D_AddSpans16((*s).spans);
        } else { D_BlendSpans16((*s).spans, alpha); }
    }
    transformed_modelorg[0 as libc::c_int as usize] =
        world_transformed_modelorg[0 as libc::c_int as usize];
    transformed_modelorg[1 as libc::c_int as usize] =
        world_transformed_modelorg[1 as libc::c_int as usize];
    transformed_modelorg[2 as libc::c_int as usize] =
        world_transformed_modelorg[2 as libc::c_int as usize];
    RI.vforward[0 as libc::c_int as usize] =
        RI.base_vpn[0 as libc::c_int as usize];
    RI.vforward[1 as libc::c_int as usize] =
        RI.base_vpn[1 as libc::c_int as usize];
    RI.vforward[2 as libc::c_int as usize] =
        RI.base_vpn[2 as libc::c_int as usize];
    RI.vup[0 as libc::c_int as usize] =
        RI.base_vup[0 as libc::c_int as usize];
    RI.vup[1 as libc::c_int as usize] =
        RI.base_vup[1 as libc::c_int as usize];
    RI.vup[2 as libc::c_int as usize] =
        RI.base_vup[2 as libc::c_int as usize];
    RI.vright[0 as libc::c_int as usize] =
        RI.base_vright[0 as libc::c_int as usize];
    RI.vright[1 as libc::c_int as usize] =
        RI.base_vright[1 as libc::c_int as usize];
    RI.vright[2 as libc::c_int as usize] =
        RI.base_vright[2 as libc::c_int as usize];
    R_TransformFrustum();
}
/*
==============
D_SolidSurf

Normal surface cached, texture mapped surface
==============
*/
#[no_mangle]
pub unsafe extern "C" fn D_SolidSurf(mut s: *mut surf_t) {
    d_zistepu = (*s).d_zistepu;
    d_zistepv = (*s).d_zistepv;
    d_ziorigin = (*s).d_ziorigin;
    if (*s).flags as libc::c_uint & (1 as libc::c_uint) << 2 as libc::c_int !=
           0 {
        return
    }
    if (*s).flags as libc::c_uint & (1 as libc::c_uint) << 4 as libc::c_int !=
           0 {
        return
    }
    if (*s).insubmodel as u64 != 0 {
        // FIXME: we don't want to do all this for every polygon!
	// TODO: store once at start of frame
        RI.currententity = (*s).entity; //FIXME: make this passed in to
        // R_RotateBmodel ()
        local_modelorg[0 as libc::c_int as usize] =
            RI.vieworg[0 as libc::c_int as usize] -
                (*RI.currententity).origin[0 as libc::c_int as
                                               usize]; // FIXME: don't mess with the frustum,
        local_modelorg[1 as libc::c_int as usize] =
            RI.vieworg[1 as libc::c_int as usize] -
                (*RI.currententity).origin[1 as libc::c_int as usize];
        local_modelorg[2 as libc::c_int as usize] =
            RI.vieworg[2 as libc::c_int as usize] -
                (*RI.currententity).origin[2 as libc::c_int as usize];
        TransformVector(local_modelorg.as_mut_ptr(),
                        transformed_modelorg.as_mut_ptr());
        R_RotateBmodel();
        // make entity passed in
		// setup dlight transform
        if !(*s).msurf.is_null() && (*(*s).msurf).dlightframe == tr.framecount
           {
            Matrix4x4_CreateFromEntity(RI.objectMatrix.as_mut_ptr(),
                                       (*RI.currententity).angles.as_mut_ptr()
                                           as *const vec_t,
                                       (*RI.currententity).origin.as_mut_ptr()
                                           as *const vec_t,
                                       1 as libc::c_int as
                                           libc::c_float); //r_worldentity;
            tr.modelviewIdentity = false_0
        }
    } else {
        if alphaspans as u64 != 0 { return }
        RI.currententity =
            gEngfuncs.GetEntityByIndex.expect("non-null function pointer")(0
                                                                               as
                                                                               libc::c_int);
        tr.modelviewIdentity = true_0
    }
    pface = (*s).msurf;
    if pface.is_null() { return }
    if (*pface).flags as libc::c_uint &
           (1 as libc::c_uint) << 6 as libc::c_int != 0 {
        miplevel = 1 as libc::c_int
    } else { miplevel = D_MipLevelForScale((*s).nearzi * scale_for_mip) }
    while (1 as libc::c_int) << miplevel >
              gEngfuncs.Mod_SampleSizeForFace.expect("non-null function pointer")(pface)
          {
        miplevel -= 1
    }
    // FIXME: make this passed in to D_CacheSurface
    pcurrentcache = D_CacheSurface(pface, miplevel);
    cacheblock = (*pcurrentcache).data.as_mut_ptr() as *mut pixel_t;
    cachewidth = (*pcurrentcache).width as libc::c_int;
    D_CalcGradients(pface);
    D_DrawSpans16((*s).spans);
    D_DrawZSpans((*s).spans);
    if (*s).insubmodel as u64 != 0 {
        //
	// restore the old drawing state
	// FIXME: we don't want to do this every time!
	// TODO: speed up
	//
        transformed_modelorg[0 as libc::c_int as usize] =
            world_transformed_modelorg[0 as libc::c_int as usize];
        transformed_modelorg[1 as libc::c_int as usize] =
            world_transformed_modelorg[1 as libc::c_int as usize];
        transformed_modelorg[2 as libc::c_int as usize] =
            world_transformed_modelorg[2 as libc::c_int as usize];
        RI.vforward[0 as libc::c_int as usize] =
            RI.base_vpn[0 as libc::c_int as usize];
        RI.vforward[1 as libc::c_int as usize] =
            RI.base_vpn[1 as libc::c_int as usize];
        RI.vforward[2 as libc::c_int as usize] =
            RI.base_vpn[2 as libc::c_int as usize];
        RI.vup[0 as libc::c_int as usize] =
            RI.base_vup[0 as libc::c_int as usize];
        RI.vup[1 as libc::c_int as usize] =
            RI.base_vup[1 as libc::c_int as usize];
        RI.vup[2 as libc::c_int as usize] =
            RI.base_vup[2 as libc::c_int as usize];
        RI.vright[0 as libc::c_int as usize] =
            RI.base_vright[0 as libc::c_int as usize];
        RI.vright[1 as libc::c_int as usize] =
            RI.base_vright[1 as libc::c_int as usize];
        RI.vright[2 as libc::c_int as usize] =
            RI.base_vright[2 as libc::c_int as usize];
        R_TransformFrustum();
        RI.currententity = 0 as *mut cl_entity_t
        //&r_worldentity;
    };
}
/*
=============
D_DrawflatSurfaces

To allow developers to see the polygon carving of the world
=============
*/
#[no_mangle]
pub unsafe extern "C" fn D_DrawflatSurfaces() {
    let mut s: *mut surf_t = 0 as *mut surf_t;
    s = &mut *surfaces.offset(1 as libc::c_int as isize) as *mut surf_t;
    while s < surface_p {
        if !(*s).spans.is_null() {
            d_zistepu = (*s).d_zistepu;
            d_zistepv = (*s).d_zistepv;
            d_ziorigin = (*s).d_ziorigin;
            // make a stable color for each surface by taking the low
		// bits of the msurface pointer
            D_FlatFillSurface(s,
                              ((*s).msurf as uintptr_t &
                                   0xffff as libc::c_int as libc::c_ulong) as
                                  libc::c_int);
            D_DrawZSpans((*s).spans);
        }
        s = s.offset(1)
    };
}
/*
==============
D_DrawSurfaces

Rasterize all the span lists.  Guaranteed zero overdraw.
May be called more than once a frame if the surf list overflows (higher res)
==============
*/
#[no_mangle]
pub unsafe extern "C" fn D_DrawSurfaces() {
    let mut s: *mut surf_t = 0 as *mut surf_t;
    //	currententity = NULL;	//&r_worldentity;
    tr.modelorg[0 as libc::c_int as usize] =
        RI.vieworg[0 as libc::c_int as usize] -
            vec3_origin[0 as libc::c_int as usize];
    tr.modelorg[1 as libc::c_int as usize] =
        RI.vieworg[1 as libc::c_int as usize] -
            vec3_origin[1 as libc::c_int as usize];
    tr.modelorg[2 as libc::c_int as usize] =
        RI.vieworg[2 as libc::c_int as usize] -
            vec3_origin[2 as libc::c_int as usize];
    TransformVector(tr.modelorg.as_mut_ptr(),
                    transformed_modelorg.as_mut_ptr());
    world_transformed_modelorg[0 as libc::c_int as usize] =
        transformed_modelorg[0 as libc::c_int as usize];
    world_transformed_modelorg[1 as libc::c_int as usize] =
        transformed_modelorg[1 as libc::c_int as usize];
    world_transformed_modelorg[2 as libc::c_int as usize] =
        transformed_modelorg[2 as libc::c_int as usize];
    if (*sw_drawflat).value == 0. {
        s = &mut *surfaces.offset(1 as libc::c_int as isize) as *mut surf_t;
        while s < surface_p {
            if !(*s).spans.is_null() {
                //r_drawnpolycount++;
                if alphaspans as u64 != 0 {
                    D_AlphaSurf(s);
                } else if (*s).flags as libc::c_uint &
                              (1 as libc::c_uint) << 2 as libc::c_int != 0 {
                    D_BackgroundSurf(s);
                } else if (*s).flags as libc::c_uint &
                              (1 as libc::c_uint) << 4 as libc::c_int != 0 {
                    D_TurbulentSurf(s);
                } else { D_SolidSurf(s); }
            }
            s = s.offset(1)
        }
    } else { D_DrawflatSurfaces(); }
    //RI.currententity = NULL;	//&r_worldentity;
    tr.modelorg[0 as libc::c_int as usize] =
        RI.vieworg[0 as libc::c_int as usize] -
            vec3_origin[0 as libc::c_int as usize];
    tr.modelorg[1 as libc::c_int as usize] =
        RI.vieworg[1 as libc::c_int as usize] -
            vec3_origin[1 as libc::c_int as usize];
    tr.modelorg[2 as libc::c_int as usize] =
        RI.vieworg[2 as libc::c_int as usize] -
            vec3_origin[2 as libc::c_int as usize];
    R_TransformFrustum();
}
