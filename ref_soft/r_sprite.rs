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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn sqrt(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn sqrtf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn SinCos(radians: libc::c_float, sine: *mut libc::c_float,
              cosine: *mut libc::c_float);
    #[no_mangle]
    fn AngleVectors(angles: *const vec_t, forward: *mut vec_t,
                    right: *mut vec_t, up: *mut vec_t);
    #[no_mangle]
    fn RadiusFromBounds(mins: *const vec_t, maxs: *const vec_t)
     -> libc::c_float;
    #[no_mangle]
    static mut r_temppool: poolhandle_t;
    #[no_mangle]
    static mut r_stats: ref_speeds_t;
    #[no_mangle]
    static mut RI: ref_instance_t;
    #[no_mangle]
    static mut tr: gl_globals_t;
    #[no_mangle]
    fn GL_Bind(tmu: libc::c_int, texnum: libc::c_uint);
    #[no_mangle]
    fn GL_LoadTexture(name: *const libc::c_char, buf: *const byte,
                      size: size_t, flags: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn GL_LoadTextureFromBuffer(name: *const libc::c_char,
                                pic: *mut rgbdata_t, flags: texFlags_t,
                                update: qboolean) -> libc::c_int;
    #[no_mangle]
    fn GL_FreeTexture(texnum: libc::c_uint);
    #[no_mangle]
    fn R_LightPoint(p0: *const vec_t) -> colorVec;
    #[no_mangle]
    fn R_AllowFog(allowed: qboolean);
    #[no_mangle]
    static mut gEngfuncs: ref_api_t;
    #[no_mangle]
    fn Q_snprintf(buffer: *mut libc::c_char, buffersize: size_t,
                  format: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn COM_StripExtension(path: *mut libc::c_char);
    #[no_mangle]
    static mut gpGlobals: *mut ref_globals_t;
    #[no_mangle]
    fn TriEnd();
    #[no_mangle]
    fn TriVertex3fv(v: *const libc::c_float);
    #[no_mangle]
    fn TriTexCoord2f(u: libc::c_float, v: libc::c_float);
    #[no_mangle]
    fn TriBegin(mode: libc::c_int);
    #[no_mangle]
    fn _TriColor4f(r: libc::c_float, g: libc::c_float, b: libc::c_float,
                   a: libc::c_float);
    #[no_mangle]
    fn GL_SetRenderMode(mode: libc::c_int);
    #[no_mangle]
    static mut r_traceglow: *mut cvar_t;
    #[no_mangle]
    fn TriWorldToScreen(world: *const libc::c_float,
                        screen: *mut libc::c_float) -> libc::c_int;
    #[no_mangle]
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const kRenderFxClampMinScale: C2RustUnnamed_0 = 20;
pub const kRenderFxGlowShell: C2RustUnnamed_0 = 19;
pub const kRenderFxExplode: C2RustUnnamed_0 = 18;
pub const kRenderFxDeadPlayer: C2RustUnnamed_0 = 17;
pub const kRenderFxHologram: C2RustUnnamed_0 = 16;
pub const kRenderFxDistort: C2RustUnnamed_0 = 15;
pub const kRenderFxNoDissipation: C2RustUnnamed_0 = 14;
pub const kRenderFxFlickerFast: C2RustUnnamed_0 = 13;
pub const kRenderFxFlickerSlow: C2RustUnnamed_0 = 12;
pub const kRenderFxStrobeFaster: C2RustUnnamed_0 = 11;
pub const kRenderFxStrobeFast: C2RustUnnamed_0 = 10;
pub const kRenderFxStrobeSlow: C2RustUnnamed_0 = 9;
pub const kRenderFxSolidFast: C2RustUnnamed_0 = 8;
pub const kRenderFxSolidSlow: C2RustUnnamed_0 = 7;
pub const kRenderFxFadeFast: C2RustUnnamed_0 = 6;
pub const kRenderFxFadeSlow: C2RustUnnamed_0 = 5;
pub const kRenderFxPulseFastWide: C2RustUnnamed_0 = 4;
pub const kRenderFxPulseSlowWide: C2RustUnnamed_0 = 3;
pub const kRenderFxPulseFast: C2RustUnnamed_0 = 2;
pub const kRenderFxPulseSlow: C2RustUnnamed_0 = 1;
pub const kRenderFxNone: C2RustUnnamed_0 = 0;
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
pub type spriteframetype_t = libc::c_uint;
pub const SPR_ANGLED: spriteframetype_t = 2;
pub const SPR_GROUP: spriteframetype_t = 1;
pub const SPR_SINGLE: spriteframetype_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mspriteframe_s {
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub up: libc::c_float,
    pub down: libc::c_float,
    pub left: libc::c_float,
    pub right: libc::c_float,
    pub gl_texturenum: libc::c_int,
}
pub type mspriteframe_t = mspriteframe_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mspritegroup_t {
    pub numframes: libc::c_int,
    pub intervals: *mut libc::c_float,
    pub frames: [*mut mspriteframe_t; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mspriteframedesc_t {
    pub type_0: spriteframetype_t,
    pub frameptr: *mut mspriteframe_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msprite_t {
    pub type_0: libc::c_short,
    pub texFormat: libc::c_short,
    pub maxwidth: libc::c_int,
    pub maxheight: libc::c_int,
    pub numframes: libc::c_int,
    pub radius: libc::c_int,
    pub facecull: libc::c_int,
    pub synctype: libc::c_int,
    pub frames: [mspriteframedesc_t; 1],
}
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
pub type C2RustUnnamed_1 = libc::c_uint;
pub const IL_OVERVIEW: C2RustUnnamed_1 = 64;
pub const IL_LOAD_DECAL: C2RustUnnamed_1 = 32;
pub const IL_DDS_HARDWARE: C2RustUnnamed_1 = 16;
pub const IL_DONTFLIP_TGA: C2RustUnnamed_1 = 8;
pub const IL_ALLOW_OVERWRITE: C2RustUnnamed_1 = 4;
pub const IL_KEEP_8BIT: C2RustUnnamed_1 = 2;
pub const IL_USE_LERPING: C2RustUnnamed_1 = 1;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const IMAGE_REMAP: C2RustUnnamed_2 = 134217728;
pub const IMAGE_LIGHTGAMMA: C2RustUnnamed_2 = 67108864;
pub const IMAGE_QUANTIZE: C2RustUnnamed_2 = 33554432;
pub const IMAGE_MAKE_LUMA: C2RustUnnamed_2 = 16777216;
pub const IMAGE_FORCE_RGBA: C2RustUnnamed_2 = 8388608;
pub const IMAGE_RESAMPLE: C2RustUnnamed_2 = 1048576;
pub const IMAGE_EMBOSS: C2RustUnnamed_2 = 524288;
pub const IMAGE_ROT270: C2RustUnnamed_2 = 458752;
pub const IMAGE_ROT180: C2RustUnnamed_2 = 196608;
pub const IMAGE_ROT_90: C2RustUnnamed_2 = 262144;
pub const IMAGE_FLIP_Y: C2RustUnnamed_2 = 131072;
pub const IMAGE_FLIP_X: C2RustUnnamed_2 = 65536;
pub const IMAGE_QUAKEPAL: C2RustUnnamed_2 = 1024;
pub const IMAGE_ONEBIT_ALPHA: C2RustUnnamed_2 = 512;
pub const IMAGE_MULTILAYER: C2RustUnnamed_2 = 256;
pub const IMAGE_DDS_FORMAT: C2RustUnnamed_2 = 128;
pub const IMAGE_QUAKESKY: C2RustUnnamed_2 = 64;
pub const IMAGE_SKYBOX: C2RustUnnamed_2 = 32;
pub const IMAGE_HAS_LUMA: C2RustUnnamed_2 = 16;
pub const IMAGE_COLORINDEX: C2RustUnnamed_2 = 8;
pub const IMAGE_HAS_COLOR: C2RustUnnamed_2 = 4;
pub const IMAGE_HAS_ALPHA: C2RustUnnamed_2 = 2;
pub const IMAGE_CUBEMAP: C2RustUnnamed_2 = 1;
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
pub type C2RustUnnamed_3 = libc::c_int;
pub const MAX_TEXTURE_UNITS: C2RustUnnamed_3 = 32;
pub const XASH_TEXTURE4: C2RustUnnamed_3 = 4;
pub const XASH_TEXTURE3: C2RustUnnamed_3 = 3;
pub const XASH_TEXTURE2: C2RustUnnamed_3 = 2;
pub const XASH_TEXTURE1: C2RustUnnamed_3 = 1;
pub const XASH_TEXTURE0: C2RustUnnamed_3 = 0;
pub const GL_KEEP_UNIT: C2RustUnnamed_3 = -1;
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
pub struct dframetype_t {
    pub type_0: frametype_t,
}
pub type frametype_t = libc::c_uint;
pub const FRAME_ANGLED: frametype_t = 2;
pub const FRAME_GROUP: frametype_t = 1;
pub const FRAME_SINGLE: frametype_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dspriteframe_t {
    pub origin: [libc::c_int; 2],
    pub width: libc::c_int,
    pub height: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dspriteinterval_t {
    pub interval: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dspritegroup_t {
    pub numframes: libc::c_int,
}
pub const SPR_ALPHTEST: drawtype_t = 3;
pub const SPR_INDEXALPHA: drawtype_t = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dsprite_q1_t {
    pub ident: libc::c_int,
    pub version: libc::c_int,
    pub type_0: libc::c_int,
    pub boundingradius: libc::c_float,
    pub bounds: [libc::c_int; 2],
    pub numframes: libc::c_int,
    pub beamlength: libc::c_float,
    pub synctype: synctype_t,
}
pub type synctype_t = libc::c_uint;
pub const ST_RAND: synctype_t = 1;
pub const ST_SYNC: synctype_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dsprite_t {
    pub ident: libc::c_int,
    pub version: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dsprite_hl_t {
    pub ident: libc::c_int,
    pub version: libc::c_int,
    pub type_0: angletype_t,
    pub texFormat: drawtype_t,
    pub boundingradius: libc::c_int,
    pub bounds: [libc::c_int; 2],
    pub numframes: libc::c_int,
    pub facetype: facetype_t,
    pub synctype: synctype_t,
}
pub type facetype_t = libc::c_uint;
pub const SPR_CULL_NONE: facetype_t = 1;
pub const SPR_CULL_FRONT: facetype_t = 0;
pub type drawtype_t = libc::c_uint;
pub const SPR_ADDITIVE: drawtype_t = 1;
pub const SPR_NORMAL: drawtype_t = 0;
pub type angletype_t = libc::c_uint;
pub const SPR_FWD_PARALLEL_ORIENTED: angletype_t = 4;
pub const SPR_ORIENTED: angletype_t = 3;
pub const SPR_FWD_PARALLEL: angletype_t = 2;
pub const SPR_FACING_UPRIGHT: angletype_t = 1;
pub const SPR_FWD_PARALLEL_UPRIGHT: angletype_t = 0;
pub type pmtrace_t = pmtrace_s;
#[inline(always)]
unsafe extern "C" fn __tg_sqrt(mut __x: libc::c_float) -> libc::c_float {
    return sqrtf(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_sqrt_0(mut __x: libc::c_double) -> libc::c_double {
    return sqrt(__x);
}
#[no_mangle]
pub static mut r_sprite_lerping: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_sprite_lighting: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sprite_name: [libc::c_char; 64] = [0; 64];
#[no_mangle]
pub static mut group_suffix: [libc::c_char; 8] = [0; 8];
static mut r_texFlags: uint = 0 as libc::c_int as uint;
static mut sprite_version: libc::c_int = 0;
#[no_mangle]
pub static mut sprite_radius: libc::c_float = 0.;
/*
====================
R_SpriteInit

====================
*/
#[no_mangle]
pub unsafe extern "C" fn R_SpriteInit() {
    r_sprite_lerping =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"r_sprite_lerping\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"1\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   0 as
                                                                       libc::c_int,
                                                               b"enables sprite animation lerping\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    r_sprite_lighting =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"r_sprite_lighting\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"1\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   0 as
                                                                       libc::c_int,
                                                               b"enables sprite lighting (blood etc)\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
}
/*
====================
R_SpriteLoadFrame

upload a single frame
====================
*/
unsafe extern "C" fn R_SpriteLoadFrame(mut mod_0: *mut model_t,
                                       mut pin: *const libc::c_void,
                                       mut ppframe: *mut *mut mspriteframe_t,
                                       mut num: libc::c_int)
 -> *const dframetype_t {
    let mut pinframe: dspriteframe_t =
        dspriteframe_t{origin: [0; 2], width: 0, height: 0,};
    let mut pspriteframe: *mut mspriteframe_t = 0 as *mut mspriteframe_t;
    let mut gl_texturenum: libc::c_int = 0 as libc::c_int;
    let mut texname: [libc::c_char; 128] = [0; 128];
    let mut bytes: libc::c_int = 1 as libc::c_int;
    memcpy(&mut pinframe as *mut dspriteframe_t as *mut libc::c_void, pin,
           ::std::mem::size_of::<dspriteframe_t>() as libc::c_ulong);
    if sprite_version == 32 as libc::c_int { bytes = 4 as libc::c_int }
    // build uinque frame name
    if (*mod_0).flags as libc::c_uint &
           (1 as libc::c_uint) << 30 as libc::c_int != 0 {
        // it's a HUD sprite
        Q_snprintf(texname.as_mut_ptr(),
                   ::std::mem::size_of::<[libc::c_char; 128]>() as
                       libc::c_ulong,
                   b"#HUD/%s(%s:%i%i).spr\x00" as *const u8 as
                       *const libc::c_char, sprite_name.as_mut_ptr(),
                   group_suffix.as_mut_ptr(), num / 10 as libc::c_int,
                   num % 10 as libc::c_int);
        gl_texturenum =
            GL_LoadTexture(texname.as_mut_ptr(), pin as *const byte,
                           (pinframe.width * pinframe.height * bytes) as
                               size_t, r_texFlags as libc::c_int)
    } else {
        Q_snprintf(texname.as_mut_ptr(),
                   ::std::mem::size_of::<[libc::c_char; 128]>() as
                       libc::c_ulong,
                   b"#%s(%s:%i%i).spr\x00" as *const u8 as
                       *const libc::c_char, sprite_name.as_mut_ptr(),
                   group_suffix.as_mut_ptr(), num / 10 as libc::c_int,
                   num % 10 as libc::c_int);
        gl_texturenum =
            GL_LoadTexture(texname.as_mut_ptr(), pin as *const byte,
                           (pinframe.width * pinframe.height * bytes) as
                               size_t, r_texFlags as libc::c_int)
    }
    // setup frame description
    pspriteframe =
        gEngfuncs._Mem_Alloc.expect("non-null function pointer")((*mod_0).mempool,
                                                                 ::std::mem::size_of::<mspriteframe_t>()
                                                                     as
                                                                     libc::c_ulong,
                                                                 false_0,
                                                                 b"../ref_soft/r_sprite.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 80 as
                                                                     libc::c_int)
            as *mut mspriteframe_t;
    (*pspriteframe).width = pinframe.width;
    (*pspriteframe).height = pinframe.height;
    (*pspriteframe).up =
        pinframe.origin[1 as libc::c_int as usize] as libc::c_float;
    (*pspriteframe).left =
        pinframe.origin[0 as libc::c_int as usize] as libc::c_float;
    (*pspriteframe).down =
        (pinframe.origin[1 as libc::c_int as usize] - pinframe.height) as
            libc::c_float;
    (*pspriteframe).right =
        (pinframe.width + pinframe.origin[0 as libc::c_int as usize]) as
            libc::c_float;
    (*pspriteframe).gl_texturenum = gl_texturenum;
    *ppframe = pspriteframe;
    return (pin as
                *const byte).offset(::std::mem::size_of::<dspriteframe_t>() as
                                        libc::c_ulong as
                                        isize).offset((pinframe.width *
                                                           pinframe.height *
                                                           bytes) as isize) as
               *const dframetype_t;
}
/*
====================
R_SpriteLoadGroup

upload a group frames
====================
*/
unsafe extern "C" fn R_SpriteLoadGroup(mut mod_0: *mut model_t,
                                       mut pin: *const libc::c_void,
                                       mut ppframe: *mut *mut mspriteframe_t,
                                       mut framenum: libc::c_int)
 -> *const dframetype_t {
    let mut pingroup: *const dspritegroup_t =
        0 as *const dspritegroup_t; // set error value
    let mut pspritegroup: *mut mspritegroup_t = 0 as *mut mspritegroup_t;
    let mut pin_intervals: *const dspriteinterval_t =
        0 as *const dspriteinterval_t;
    let mut poutintervals: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut i: libc::c_int = 0;
    let mut groupsize: libc::c_int = 0;
    let mut numframes: libc::c_int = 0;
    let mut ptemp: *const libc::c_void = 0 as *const libc::c_void;
    pingroup = pin as *const dspritegroup_t;
    numframes = (*pingroup).numframes;
    groupsize =
        (::std::mem::size_of::<mspritegroup_t>() as
             libc::c_ulong).wrapping_add(((numframes - 1 as libc::c_int) as
                                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut mspriteframe_t>()
                                                                              as
                                                                              libc::c_ulong))
            as libc::c_int;
    pspritegroup =
        gEngfuncs._Mem_Alloc.expect("non-null function pointer")((*mod_0).mempool,
                                                                 groupsize as
                                                                     size_t,
                                                                 true_0,
                                                                 b"../ref_soft/r_sprite.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 113 as
                                                                     libc::c_int)
            as *mut mspritegroup_t;
    (*pspritegroup).numframes = numframes;
    *ppframe = pspritegroup as *mut mspriteframe_t;
    pin_intervals =
        pingroup.offset(1 as libc::c_int as isize) as
            *const dspriteinterval_t;
    poutintervals =
        gEngfuncs._Mem_Alloc.expect("non-null function pointer")((*mod_0).mempool,
                                                                 (numframes as
                                                                      libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_float>()
                                                                                                      as
                                                                                                      libc::c_ulong),
                                                                 true_0,
                                                                 b"../ref_soft/r_sprite.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 118 as
                                                                     libc::c_int)
            as *mut libc::c_float;
    (*pspritegroup).intervals = poutintervals;
    i = 0 as libc::c_int;
    while i < numframes {
        *poutintervals = (*pin_intervals).interval;
        if *poutintervals <= 0.0f32 { *poutintervals = 1.0f32 }
        poutintervals = poutintervals.offset(1);
        pin_intervals = pin_intervals.offset(1);
        i += 1
    }
    ptemp = pin_intervals as *const libc::c_void;
    i = 0 as libc::c_int;
    while i < numframes {
        ptemp =
            R_SpriteLoadFrame(mod_0, ptemp,
                              &mut *(*pspritegroup).frames.as_mut_ptr().offset(i
                                                                                   as
                                                                                   isize),
                              framenum * 10 as libc::c_int + i) as
                *const libc::c_void;
        i += 1
    }
    return ptemp as *const dframetype_t;
}
/*
====================
Mod_LoadSpriteModel

load sprite model
====================
*/
#[no_mangle]
pub unsafe extern "C" fn Mod_LoadSpriteModel(mut mod_0: *mut model_t,
                                             mut buffer: *const libc::c_void,
                                             mut loaded: *mut qboolean,
                                             mut texFlags: uint) {
    let mut pin: *const dsprite_t = 0 as *const dsprite_t;
    let mut numi: *const libc::c_short = 0 as *const libc::c_short;
    let mut pframetype: *const dframetype_t = 0 as *const dframetype_t;
    let mut psprite: *mut msprite_t = 0 as *mut msprite_t;
    let mut i: libc::c_int = 0;
    pin = buffer as *const dsprite_t;
    psprite = (*mod_0).cache.data as *mut msprite_t;
    if (*pin).version == 1 as libc::c_int ||
           (*pin).version == 32 as libc::c_int {
        numi = 0 as *const libc::c_short
    } else if (*pin).version == 2 as libc::c_int {
        numi =
            (buffer as
                 *const byte).offset(::std::mem::size_of::<dsprite_hl_t>() as
                                         libc::c_ulong as isize) as
                *const libc::c_short
    }
    r_texFlags = texFlags;
    sprite_version = (*pin).version;
    Q_strncpy(sprite_name.as_mut_ptr(), (*mod_0).name.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    COM_StripExtension(sprite_name.as_mut_ptr());
    if numi.is_null() {
        let mut pal: *mut rgbdata_t = 0 as *mut rgbdata_t;
        pal =
            gEngfuncs.FS_LoadImage.expect("non-null function pointer")(b"#id.pal\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const libc::c_char,
                                                                       &mut i
                                                                           as
                                                                           *mut libc::c_int
                                                                           as
                                                                           *mut byte,
                                                                       768 as
                                                                           libc::c_int
                                                                           as
                                                                           size_t);
        // palette installed, no reason to keep this data
        pframetype =
            (buffer as
                 *const byte).offset(::std::mem::size_of::<dsprite_q1_t>() as
                                         libc::c_ulong as isize) as
                *const dframetype_t; // pinq1 + 1
        gEngfuncs.FS_FreeImage.expect("non-null function pointer")(pal);
    } else if *numi as libc::c_int == 256 as libc::c_int {
        let mut src: *const byte =
            numi.offset(1 as libc::c_int as isize) as *const byte;
        let mut pal_0: *mut rgbdata_t = 0 as *mut rgbdata_t;
        // palette installed, no reason to keep this data
        match (*psprite).texFormat as libc::c_int {
            2 => {
                pal_0 =
                    gEngfuncs.FS_LoadImage.expect("non-null function pointer")(b"#gradient.pal\x00"
                                                                                   as
                                                                                   *const u8
                                                                                   as
                                                                                   *const libc::c_char,
                                                                               src,
                                                                               768
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   size_t)
            }
            3 => {
                pal_0 =
                    gEngfuncs.FS_LoadImage.expect("non-null function pointer")(b"#masked.pal\x00"
                                                                                   as
                                                                                   *const u8
                                                                                   as
                                                                                   *const libc::c_char,
                                                                               src,
                                                                               768
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   size_t)
            }
            _ => {
                pal_0 =
                    gEngfuncs.FS_LoadImage.expect("non-null function pointer")(b"#normal.pal\x00"
                                                                                   as
                                                                                   *const u8
                                                                                   as
                                                                                   *const libc::c_char,
                                                                               src,
                                                                               768
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   size_t)
            }
        }
        pframetype =
            src.offset(768 as libc::c_int as isize) as *const dframetype_t;
        gEngfuncs.FS_FreeImage.expect("non-null function pointer")(pal_0);
    } else {
        gEngfuncs.Con_DPrintf.expect("non-null function pointer")(b"^1Error:^7 %s has wrong number of palette colors %i (should be 256)\n\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char,
                                                                  (*mod_0).name.as_mut_ptr(),
                                                                  *numi as
                                                                      libc::c_int);
        return
    }
    if (*mod_0).numframes < 1 as libc::c_int { return }
    i = 0 as libc::c_int;
    while i < (*mod_0).numframes {
        let mut frametype: frametype_t = (*pframetype).type_0;
        (*(*psprite).frames.as_mut_ptr().offset(i as isize)).type_0 =
            frametype as spriteframetype_t;
        match frametype as libc::c_uint {
            0 => {
                Q_strncpy(group_suffix.as_mut_ptr(),
                          b"frame\x00" as *const u8 as *const libc::c_char,
                          ::std::mem::size_of::<[libc::c_char; 8]>() as
                              libc::c_ulong);
                pframetype =
                    R_SpriteLoadFrame(mod_0,
                                      pframetype.offset(1 as libc::c_int as
                                                            isize) as
                                          *const libc::c_void,
                                      &mut (*(*psprite).frames.as_mut_ptr().offset(i
                                                                                       as
                                                                                       isize)).frameptr,
                                      i)
            }
            1 => {
                Q_strncpy(group_suffix.as_mut_ptr(),
                          b"group\x00" as *const u8 as *const libc::c_char,
                          ::std::mem::size_of::<[libc::c_char; 8]>() as
                              libc::c_ulong);
                pframetype =
                    R_SpriteLoadGroup(mod_0,
                                      pframetype.offset(1 as libc::c_int as
                                                            isize) as
                                          *const libc::c_void,
                                      &mut (*(*psprite).frames.as_mut_ptr().offset(i
                                                                                       as
                                                                                       isize)).frameptr,
                                      i)
            }
            2 => {
                Q_strncpy(group_suffix.as_mut_ptr(),
                          b"angle\x00" as *const u8 as *const libc::c_char,
                          ::std::mem::size_of::<[libc::c_char; 8]>() as
                              libc::c_ulong);
                pframetype =
                    R_SpriteLoadGroup(mod_0,
                                      pframetype.offset(1 as libc::c_int as
                                                            isize) as
                                          *const libc::c_void,
                                      &mut (*(*psprite).frames.as_mut_ptr().offset(i
                                                                                       as
                                                                                       isize)).frameptr,
                                      i)
            }
            _ => { }
        }
        if pframetype.is_null() { break ; }
        i += 1
        // install palette
        // technically an error
    }
    if !loaded.is_null() { *loaded = true_0 };
    // done
}
/*
====================
Mod_LoadMapSprite

Loading a bitmap image as sprite with multiple frames
as pieces of input image
====================
*/
#[no_mangle]
pub unsafe extern "C" fn Mod_LoadMapSprite(mut mod_0: *mut model_t,
                                           mut buffer: *const libc::c_void,
                                           mut size: size_t,
                                           mut loaded: *mut qboolean) {
    let mut src: *mut byte = 0 as *mut byte; // bad image or something else
    let mut dst: *mut byte =
        0 as *mut byte; // no custom flags for map sprites
    let mut pix: *mut rgbdata_t = 0 as *mut rgbdata_t;
    let mut temp: rgbdata_t =
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
    let mut texname: [libc::c_char; 128] = [0; 128];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut xl: libc::c_int = 0;
    let mut yl: libc::c_int = 0;
    let mut xh: libc::c_int = 0;
    let mut yh: libc::c_int = 0;
    let mut linedelta: libc::c_int = 0;
    let mut numframes: libc::c_int = 0;
    let mut pspriteframe: *mut mspriteframe_t = 0 as *mut mspriteframe_t;
    let mut psprite: *mut msprite_t = 0 as *mut msprite_t;
    if !loaded.is_null() { *loaded = false_0 }
    Q_snprintf(texname.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
               b"#%s\x00" as *const u8 as *const libc::c_char,
               (*mod_0).name.as_mut_ptr());
    gEngfuncs.Image_SetForceFlags.expect("non-null function pointer")(IL_OVERVIEW
                                                                          as
                                                                          libc::c_int
                                                                          as
                                                                          uint);
    pix =
        gEngfuncs.FS_LoadImage.expect("non-null function pointer")(texname.as_mut_ptr(),
                                                                   buffer as
                                                                       *const byte,
                                                                   size);
    gEngfuncs.Image_ClearForceFlags.expect("non-null function pointer")();
    if pix.is_null() { return }
    (*mod_0).type_0 = mod_sprite;
    r_texFlags = 0 as libc::c_int as uint;
    if (*pix).width as libc::c_int % 128 as libc::c_int != 0 {
        w =
            (*pix).width as libc::c_int -
                (*pix).width as libc::c_int % 128 as libc::c_int
    } else { w = (*pix).width as libc::c_int }
    if (*pix).height as libc::c_int % 128 as libc::c_int != 0 {
        h =
            (*pix).height as libc::c_int -
                (*pix).height as libc::c_int % 128 as libc::c_int
    } else { h = (*pix).height as libc::c_int }
    if w < 128 as libc::c_int { w = 128 as libc::c_int }
    if h < 128 as libc::c_int { h = 128 as libc::c_int }
    // resample image if needed
    gEngfuncs.Image_Process.expect("non-null function pointer")(&mut pix, w,
                                                                h,
                                                                (IMAGE_FORCE_RGBA
                                                                     as
                                                                     libc::c_int
                                                                     |
                                                                     IMAGE_RESAMPLE
                                                                         as
                                                                         libc::c_int)
                                                                    as uint,
                                                                0.0f32);
    h = 128 as libc::c_int;
    w = h;
    // check range
    if w > (*pix).width as libc::c_int { w = (*pix).width as libc::c_int }
    if h > (*pix).height as libc::c_int { h = (*pix).height as libc::c_int }
    // determine how many frames we needs
    numframes =
        (*pix).width as libc::c_int * (*pix).height as libc::c_int /
            (w * h); // make link to extradata
    (*mod_0).mempool =
        gEngfuncs._Mem_AllocPool.expect("non-null function pointer")(va(b"^2%s^7\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char,
                                                                        (*mod_0).name.as_mut_ptr()),
                                                                     b"../ref_soft/r_sprite.c\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     283 as
                                                                         libc::c_int);
    psprite =
        gEngfuncs._Mem_Alloc.expect("non-null function pointer")((*mod_0).mempool,
                                                                 (::std::mem::size_of::<msprite_t>()
                                                                      as
                                                                      libc::c_ulong).wrapping_add(((numframes
                                                                                                        -
                                                                                                        1
                                                                                                            as
                                                                                                            libc::c_int)
                                                                                                       as
                                                                                                       libc::c_ulong).wrapping_mul(::std::mem::size_of::<[mspriteframedesc_t; 1]>()
                                                                                                                                       as
                                                                                                                                       libc::c_ulong)),
                                                                 true_0,
                                                                 b"../ref_soft/r_sprite.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 284 as
                                                                     libc::c_int)
            as *mut msprite_t;
    (*mod_0).cache.data = psprite as *mut libc::c_void;
    (*psprite).type_0 =
        SPR_FWD_PARALLEL_ORIENTED as libc::c_int as libc::c_short;
    (*psprite).texFormat = SPR_ALPHTEST as libc::c_int as libc::c_short;
    (*mod_0).numframes = numframes;
    (*psprite).numframes = (*mod_0).numframes;
    (*psprite).radius =
        __tg_sqrt_0(((w >> 1 as libc::c_int) * (w >> 1 as libc::c_int) +
                         (h >> 1 as libc::c_int) * (h >> 1 as libc::c_int)) as
                        libc::c_double) as libc::c_int;
    (*mod_0).mins[1 as libc::c_int as usize] =
        (-w / 2 as libc::c_int) as vec_t;
    (*mod_0).mins[0 as libc::c_int as usize] =
        (*mod_0).mins[1 as libc::c_int as usize];
    (*mod_0).maxs[1 as libc::c_int as usize] =
        (w / 2 as libc::c_int) as vec_t;
    (*mod_0).maxs[0 as libc::c_int as usize] =
        (*mod_0).maxs[1 as libc::c_int as usize];
    (*mod_0).mins[2 as libc::c_int as usize] =
        (-h / 2 as libc::c_int) as vec_t;
    (*mod_0).maxs[2 as libc::c_int as usize] =
        (h / 2 as libc::c_int) as vec_t;
    // create a temporary pic
    memset(&mut temp as *mut rgbdata_t as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<rgbdata_t>() as libc::c_ulong);
    temp.width = w as word;
    temp.height = h as word;
    temp.type_0 = (*pix).type_0;
    temp.flags = (*pix).flags;
    temp.size =
        (w * h *
             (*gEngfuncs.Image_GetPFDesc.expect("non-null function pointer")(temp.type_0
                                                                                 as
                                                                                 libc::c_int)).bpp)
            as size_t;
    temp.buffer =
        gEngfuncs._Mem_Alloc.expect("non-null function pointer")(r_temppool,
                                                                 temp.size,
                                                                 false_0,
                                                                 b"../ref_soft/r_sprite.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 304 as
                                                                     libc::c_int)
            as *mut byte;
    temp.palette = 0 as *mut byte;
    // chop the image and upload into video memory
    yl = 0 as libc::c_int;
    xl = yl;
    i = xl;
    while i < numframes {
        xh = xl + w;
        yh = yl + h;
        src =
            (*pix).buffer.offset(((yl * (*pix).width as libc::c_int + xl) *
                                      4 as libc::c_int) as isize);
        linedelta = ((*pix).width as libc::c_int - w) * 4 as libc::c_int;
        dst = temp.buffer;
        // cut block from source
        y = yl;
        while y < yh {
            x = xl;
            while x < xh {
                j = 0 as libc::c_int;
                while j < 4 as libc::c_int {
                    let fresh0 = src;
                    src = src.offset(1);
                    let fresh1 = dst;
                    dst = dst.offset(1);
                    *fresh1 = *fresh0;
                    j += 1
                }
                x += 1
            }
            src = src.offset(linedelta as isize);
            y += 1
        }
        // build uinque frame name
        Q_snprintf(texname.as_mut_ptr(),
                   ::std::mem::size_of::<[libc::c_char; 128]>() as
                       libc::c_ulong,
                   b"#MAP/%s_%i%i.spr\x00" as *const u8 as
                       *const libc::c_char, (*mod_0).name.as_mut_ptr(),
                   i / 10 as libc::c_int, i % 10 as libc::c_int);
        let ref mut fresh2 =
            (*(*psprite).frames.as_mut_ptr().offset(i as isize)).frameptr;
        *fresh2 =
            gEngfuncs._Mem_Alloc.expect("non-null function pointer")((*mod_0).mempool,
                                                                     ::std::mem::size_of::<mspriteframe_t>()
                                                                         as
                                                                         libc::c_ulong,
                                                                     true_0,
                                                                     b"../ref_soft/r_sprite.c\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     329 as
                                                                         libc::c_int)
                as *mut mspriteframe_t;
        pspriteframe =
            (*(*psprite).frames.as_mut_ptr().offset(i as isize)).frameptr;
        (*pspriteframe).width = w;
        (*pspriteframe).height = h;
        (*pspriteframe).up = (h >> 1 as libc::c_int) as libc::c_float;
        (*pspriteframe).left = -(w >> 1 as libc::c_int) as libc::c_float;
        (*pspriteframe).down = ((h >> 1 as libc::c_int) - h) as libc::c_float;
        (*pspriteframe).right =
            (w + -(w >> 1 as libc::c_int)) as libc::c_float;
        (*pspriteframe).gl_texturenum =
            GL_LoadTextureFromBuffer(texname.as_mut_ptr(), &mut temp,
                                     (TF_NOMIPMAP as libc::c_int |
                                          TF_CLAMP as libc::c_int) as
                                         texFlags_t, false_0);
        xl += w;
        if xl >= (*pix).width as libc::c_int {
            xl = 0 as libc::c_int;
            yl += h
        }
        i += 1
    }
    gEngfuncs.FS_FreeImage.expect("non-null function pointer")(pix);
    gEngfuncs._Mem_Free.expect("non-null function pointer")(temp.buffer as
                                                                *mut libc::c_void,
                                                            b"../ref_soft/r_sprite.c\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char,
                                                            348 as
                                                                libc::c_int);
    if !loaded.is_null() { *loaded = true_0 };
}
/*
====================
Mod_UnloadSpriteModel

release sprite model and frames
====================
*/
#[no_mangle]
pub unsafe extern "C" fn Mod_SpriteUnloadTextures(mut data:
                                                      *mut libc::c_void) {
    let mut psprite: *mut msprite_t = 0 as *mut msprite_t;
    let mut pspritegroup: *mut mspritegroup_t = 0 as *mut mspritegroup_t;
    let mut pspriteframe: *mut mspriteframe_t = 0 as *mut mspriteframe_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    psprite = data as *mut msprite_t;
    if !psprite.is_null() {
        // release all textures
        i = 0 as libc::c_int;
        while i < (*psprite).numframes {
            if (*(*psprite).frames.as_mut_ptr().offset(i as isize)).type_0 as
                   libc::c_uint == SPR_SINGLE as libc::c_int as libc::c_uint {
                pspriteframe =
                    (*(*psprite).frames.as_mut_ptr().offset(i as
                                                                isize)).frameptr;
                GL_FreeTexture((*pspriteframe).gl_texturenum as libc::c_uint);
            } else {
                pspritegroup =
                    (*(*psprite).frames.as_mut_ptr().offset(i as
                                                                isize)).frameptr
                        as *mut mspritegroup_t;
                j = 0 as libc::c_int;
                while j < (*pspritegroup).numframes {
                    pspriteframe =
                        *(*pspritegroup).frames.as_mut_ptr().offset(i as
                                                                        isize);
                    GL_FreeTexture((*pspriteframe).gl_texturenum as
                                       libc::c_uint);
                    j += 1
                }
            }
            i += 1
        }
    };
}
/*
================
R_GetSpriteFrame

assume pModel is valid
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_GetSpriteFrame(mut pModel: *const model_t,
                                          mut frame: libc::c_int,
                                          mut yaw: libc::c_float)
 -> *mut mspriteframe_t {
    let mut psprite: *mut msprite_t = 0 as *mut msprite_t;
    let mut pspritegroup: *mut mspritegroup_t = 0 as *mut mspritegroup_t;
    let mut pspriteframe: *mut mspriteframe_t = 0 as *mut mspriteframe_t;
    let mut pintervals: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut fullinterval: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    let mut numframes: libc::c_int = 0;
    let mut targettime: libc::c_float = 0.;
    if pModel.is_null() {
        gEngfuncs.Host_Error.expect("non-null function pointer")(b"assert failed at %s:%i\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 b"../ref_soft/r_sprite.c\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 409 as
                                                                     libc::c_int);
    }
    psprite = (*pModel).cache.data as *mut msprite_t;
    if frame < 0 as libc::c_int {
        frame = 0 as libc::c_int
    } else if frame >= (*psprite).numframes {
        if frame > (*psprite).numframes {
            gEngfuncs.Con_Printf.expect("non-null function pointer")(b"^3Warning:^7 R_GetSpriteFrame: no such frame %d (%s)\n\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     frame,
                                                                     (*pModel).name.as_ptr());
        }
        frame = (*psprite).numframes - 1 as libc::c_int
    }
    if (*(*psprite).frames.as_mut_ptr().offset(frame as isize)).type_0 as
           libc::c_uint == SPR_SINGLE as libc::c_int as libc::c_uint {
        pspriteframe =
            (*(*psprite).frames.as_mut_ptr().offset(frame as isize)).frameptr
    } else if (*(*psprite).frames.as_mut_ptr().offset(frame as isize)).type_0
                  as libc::c_uint == SPR_GROUP as libc::c_int as libc::c_uint
     {
        pspritegroup =
            (*(*psprite).frames.as_mut_ptr().offset(frame as isize)).frameptr
                as *mut mspritegroup_t;
        pintervals = (*pspritegroup).intervals;
        numframes = (*pspritegroup).numframes;
        fullinterval =
            *pintervals.offset((numframes - 1 as libc::c_int) as isize);
        // when loading in Mod_LoadSpriteGroup, we guaranteed all interval values
		// are positive, so we don't have to worry about division by zero
        targettime =
            (*gpGlobals).time -
                ((*gpGlobals).time / fullinterval) as libc::c_int as
                    libc::c_float * fullinterval;
        i = 0 as libc::c_int;
        while i < numframes - 1 as libc::c_int {
            if *pintervals.offset(i as isize) > targettime { break ; }
            i += 1
        }
        pspriteframe = *(*pspritegroup).frames.as_mut_ptr().offset(i as isize)
    } else if (*(*psprite).frames.as_mut_ptr().offset(frame as isize)).type_0
                  as libc::c_uint ==
                  FRAME_ANGLED as libc::c_int as libc::c_uint {
        let mut angleframe: libc::c_int =
            (if ((RI.viewangles[1 as libc::c_int as usize] - yaw + 45.0f32) /
                     360 as libc::c_int as libc::c_float *
                     8 as libc::c_int as libc::c_float) < 0.0f32 {
                 ((RI.viewangles[1 as libc::c_int as usize] - yaw + 45.0f32) /
                      360 as libc::c_int as libc::c_float *
                      8 as libc::c_int as libc::c_float - 0.5f32) as
                     libc::c_int
             } else {
                 ((RI.viewangles[1 as libc::c_int as usize] - yaw + 45.0f32) /
                      360 as libc::c_int as libc::c_float *
                      8 as libc::c_int as libc::c_float + 0.5f32) as
                     libc::c_int
             }) - 4 as libc::c_int & 7 as libc::c_int;
        // e.g. doom-style sprite monsters
        pspritegroup =
            (*(*psprite).frames.as_mut_ptr().offset(frame as isize)).frameptr
                as *mut mspritegroup_t;
        pspriteframe =
            *(*pspritegroup).frames.as_mut_ptr().offset(angleframe as isize)
    }
    return pspriteframe;
}
/*
================
R_GetSpriteFrameInterpolant

NOTE: we using prevblending[0] and [1] for holds interval
between frames where are we lerping
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_GetSpriteFrameInterpolant(mut ent:
                                                         *mut cl_entity_t,
                                                     mut oldframe:
                                                         *mut *mut mspriteframe_t,
                                                     mut curframe:
                                                         *mut *mut mspriteframe_t)
 -> libc::c_float {
    let mut psprite: *mut msprite_t = 0 as *mut msprite_t;
    let mut pspritegroup: *mut mspritegroup_t = 0 as *mut mspritegroup_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut numframes: libc::c_int = 0;
    let mut frame: libc::c_int = 0;
    let mut lerpFrac: libc::c_float = 0.;
    let mut time: libc::c_float = 0.;
    let mut jtime: libc::c_float = 0.;
    let mut jinterval: libc::c_float = 0.;
    let mut pintervals: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut fullinterval: libc::c_float = 0.;
    let mut targettime: libc::c_float = 0.;
    let mut m_fDoInterp: libc::c_int = 0;
    psprite = (*(*ent).model).cache.data as *mut msprite_t;
    frame = (*ent).curstate.frame as libc::c_int;
    lerpFrac = 1.0f32;
    // misc info
    m_fDoInterp =
        if (*ent).curstate.effects & 32 as libc::c_int != 0 {
            false_0 as libc::c_int
        } else { true_0 as libc::c_int };
    if frame < 0 as libc::c_int {
        frame = 0 as libc::c_int
    } else if frame >= (*psprite).numframes {
        gEngfuncs.Con_Reportf.expect("non-null function pointer")(b"^3Warning:^7 R_GetSpriteFrameInterpolant: no such frame %d (%s)\n\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char,
                                                                  frame,
                                                                  (*(*ent).model).name.as_mut_ptr());
        frame = (*psprite).numframes - 1 as libc::c_int
    }
    if (*(*psprite).frames.as_mut_ptr().offset(frame as isize)).type_0 as
           libc::c_uint == FRAME_SINGLE as libc::c_int as libc::c_uint {
        if m_fDoInterp != 0 {
            if (*ent).latched.prevblending[0 as libc::c_int as usize] as
                   libc::c_int >= (*psprite).numframes ||
                   (*(*psprite).frames.as_mut_ptr().offset((*ent).latched.prevblending[0
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           usize]
                                                               as
                                                               isize)).type_0
                       as libc::c_uint !=
                       FRAME_SINGLE as libc::c_int as libc::c_uint {
                // this can be happens when rendering switched between single and angled frames
				// or change model on replace delta-entity
                (*ent).latched.prevblending[1 as libc::c_int as usize] =
                    frame as byte;
                (*ent).latched.prevblending[0 as libc::c_int as usize] =
                    (*ent).latched.prevblending[1 as libc::c_int as usize];
                (*ent).latched.sequencetime = (*gpGlobals).time;
                lerpFrac = 1.0f32
            }
            if (*ent).latched.sequencetime < (*gpGlobals).time {
                if frame !=
                       (*ent).latched.prevblending[1 as libc::c_int as usize]
                           as libc::c_int {
                    (*ent).latched.prevblending[0 as libc::c_int as usize] =
                        (*ent).latched.prevblending[1 as libc::c_int as
                                                        usize];
                    (*ent).latched.prevblending[1 as libc::c_int as usize] =
                        frame as byte;
                    (*ent).latched.sequencetime = (*gpGlobals).time;
                    lerpFrac = 0.0f32
                } else {
                    lerpFrac =
                        ((*gpGlobals).time - (*ent).latched.sequencetime) *
                            11.0f32
                }
            } else {
                (*ent).latched.prevblending[1 as libc::c_int as usize] =
                    frame as byte;
                (*ent).latched.prevblending[0 as libc::c_int as usize] =
                    (*ent).latched.prevblending[1 as libc::c_int as usize];
                (*ent).latched.sequencetime = (*gpGlobals).time;
                lerpFrac = 0.0f32
            }
        } else {
            (*ent).latched.prevblending[1 as libc::c_int as usize] =
                frame as byte;
            (*ent).latched.prevblending[0 as libc::c_int as usize] =
                (*ent).latched.prevblending[1 as libc::c_int as usize];
            lerpFrac = 1.0f32
        }
        if (*ent).latched.prevblending[0 as libc::c_int as usize] as
               libc::c_int >= (*psprite).numframes {
            // reset interpolation on change model
            (*ent).latched.prevblending[1 as libc::c_int as usize] =
                frame as byte;
            (*ent).latched.prevblending[0 as libc::c_int as usize] =
                (*ent).latched.prevblending[1 as libc::c_int as usize];
            (*ent).latched.sequencetime = (*gpGlobals).time;
            lerpFrac = 0.0f32
        }
        // get the interpolated frames
        if !oldframe.is_null() {
            *oldframe =
                (*(*psprite).frames.as_mut_ptr().offset((*ent).latched.prevblending[0
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        usize]
                                                            as
                                                            isize)).frameptr
        }
        if !curframe.is_null() {
            *curframe =
                (*(*psprite).frames.as_mut_ptr().offset(frame as
                                                            isize)).frameptr
        }
    } else if (*(*psprite).frames.as_mut_ptr().offset(frame as isize)).type_0
                  as libc::c_uint ==
                  FRAME_GROUP as libc::c_int as libc::c_uint {
        pspritegroup =
            (*(*psprite).frames.as_mut_ptr().offset(frame as isize)).frameptr
                as *mut mspritegroup_t;
        pintervals = (*pspritegroup).intervals;
        numframes = (*pspritegroup).numframes;
        fullinterval =
            *pintervals.offset((numframes - 1 as libc::c_int) as isize);
        jinterval =
            *pintervals.offset(1 as libc::c_int as isize) -
                *pintervals.offset(0 as libc::c_int as isize);
        time = (*gpGlobals).time;
        jtime = 0.0f32;
        // when loading in Mod_LoadSpriteGroup, we guaranteed all interval values
		// are positive, so we don't have to worry about division by zero
        targettime =
            time -
                (time / fullinterval) as libc::c_int as libc::c_float *
                    fullinterval;
        // LordHavoc: since I can't measure the time properly when it loops from numframes - 1 to 0,
		// i instead measure the time of the first frame, hoping it is consistent
        i = 0 as libc::c_int; // no lerping
        j = numframes - 1 as libc::c_int;
        while i < numframes - 1 as libc::c_int {
            if *pintervals.offset(i as isize) > targettime { break ; }
            j = i;
            jinterval = *pintervals.offset(i as isize) - jtime;
            jtime = *pintervals.offset(i as isize);
            i += 1
        }
        if m_fDoInterp != 0 {
            lerpFrac = (targettime - jtime) / jinterval
        } else { j = i }
        // get the interpolated frames
        if !oldframe.is_null() {
            *oldframe =
                *(*pspritegroup).frames.as_mut_ptr().offset(j as isize)
        }
        if !curframe.is_null() {
            *curframe =
                *(*pspritegroup).frames.as_mut_ptr().offset(i as isize)
        }
    } else if (*(*psprite).frames.as_mut_ptr().offset(frame as isize)).type_0
                  as libc::c_uint ==
                  FRAME_ANGLED as libc::c_int as libc::c_uint {
        // e.g. doom-style sprite monsters
        let mut yaw: libc::c_float = (*ent).angles[1 as libc::c_int as usize];
        let mut angleframe: libc::c_int =
            (if ((RI.viewangles[1 as libc::c_int as usize] - yaw + 45.0f32) /
                     360 as libc::c_int as libc::c_float *
                     8 as libc::c_int as libc::c_float) < 0.0f32 {
                 ((RI.viewangles[1 as libc::c_int as usize] - yaw + 45.0f32) /
                      360 as libc::c_int as libc::c_float *
                      8 as libc::c_int as libc::c_float - 0.5f32) as
                     libc::c_int
             } else {
                 ((RI.viewangles[1 as libc::c_int as usize] - yaw + 45.0f32) /
                      360 as libc::c_int as libc::c_float *
                      8 as libc::c_int as libc::c_float + 0.5f32) as
                     libc::c_int
             }) - 4 as libc::c_int & 7 as libc::c_int;
        if m_fDoInterp != 0 {
            if (*ent).latched.prevblending[0 as libc::c_int as usize] as
                   libc::c_int >= (*psprite).numframes ||
                   (*(*psprite).frames.as_mut_ptr().offset((*ent).latched.prevblending[0
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           usize]
                                                               as
                                                               isize)).type_0
                       as libc::c_uint !=
                       FRAME_ANGLED as libc::c_int as libc::c_uint {
                // this can be happens when rendering switched between single and angled frames
				// or change model on replace delta-entity
                (*ent).latched.prevblending[1 as libc::c_int as usize] =
                    frame as byte;
                (*ent).latched.prevblending[0 as libc::c_int as usize] =
                    (*ent).latched.prevblending[1 as libc::c_int as usize];
                (*ent).latched.sequencetime = (*gpGlobals).time;
                lerpFrac = 1.0f32
            }
            if (*ent).latched.sequencetime < (*gpGlobals).time {
                if frame !=
                       (*ent).latched.prevblending[1 as libc::c_int as usize]
                           as libc::c_int {
                    (*ent).latched.prevblending[0 as libc::c_int as usize] =
                        (*ent).latched.prevblending[1 as libc::c_int as
                                                        usize];
                    (*ent).latched.prevblending[1 as libc::c_int as usize] =
                        frame as byte;
                    (*ent).latched.sequencetime = (*gpGlobals).time;
                    lerpFrac = 0.0f32
                } else {
                    lerpFrac =
                        ((*gpGlobals).time - (*ent).latched.sequencetime) *
                            (*ent).curstate.framerate
                }
            } else {
                (*ent).latched.prevblending[1 as libc::c_int as usize] =
                    frame as byte;
                (*ent).latched.prevblending[0 as libc::c_int as usize] =
                    (*ent).latched.prevblending[1 as libc::c_int as usize];
                (*ent).latched.sequencetime = (*gpGlobals).time;
                lerpFrac = 0.0f32
            }
        } else {
            (*ent).latched.prevblending[1 as libc::c_int as usize] =
                frame as byte;
            (*ent).latched.prevblending[0 as libc::c_int as usize] =
                (*ent).latched.prevblending[1 as libc::c_int as usize];
            lerpFrac = 1.0f32
        }
        pspritegroup =
            (*(*psprite).frames.as_mut_ptr().offset((*ent).latched.prevblending[0
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    usize]
                                                        as isize)).frameptr as
                *mut mspritegroup_t;
        if !oldframe.is_null() {
            *oldframe =
                *(*pspritegroup).frames.as_mut_ptr().offset(angleframe as
                                                                isize)
        }
        pspritegroup =
            (*(*psprite).frames.as_mut_ptr().offset(frame as isize)).frameptr
                as *mut mspritegroup_t;
        if !curframe.is_null() {
            *curframe =
                *(*pspritegroup).frames.as_mut_ptr().offset(angleframe as
                                                                isize)
        }
    }
    return lerpFrac;
}
/*
================
R_CullSpriteModel

Cull sprite model by bbox
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_CullSpriteModel(mut e: *mut cl_entity_t,
                                           mut origin: *mut vec_t)
 -> qboolean {
    let mut sprite_mins: vec3_t = [0.; 3];
    let mut sprite_maxs: vec3_t = [0.; 3];
    let mut scale: libc::c_float = 1.0f32;
    if (*(*e).model).cache.data.is_null() { return true_0 }
    if (*e).curstate.scale > 0.0f32 { scale = (*e).curstate.scale }
    // scale original bbox (no rotation for sprites)
    sprite_mins[0 as libc::c_int as usize] =
        (*(*e).model).mins[0 as libc::c_int as usize] * scale;
    sprite_mins[1 as libc::c_int as usize] =
        (*(*e).model).mins[1 as libc::c_int as usize] * scale;
    sprite_mins[2 as libc::c_int as usize] =
        (*(*e).model).mins[2 as libc::c_int as usize] * scale;
    sprite_maxs[0 as libc::c_int as usize] =
        (*(*e).model).maxs[0 as libc::c_int as usize] * scale;
    sprite_maxs[1 as libc::c_int as usize] =
        (*(*e).model).maxs[1 as libc::c_int as usize] * scale;
    sprite_maxs[2 as libc::c_int as usize] =
        (*(*e).model).maxs[2 as libc::c_int as usize] * scale;
    sprite_radius =
        RadiusFromBounds(sprite_mins.as_mut_ptr() as *const vec_t,
                         sprite_maxs.as_mut_ptr() as *const vec_t);
    sprite_mins[0 as libc::c_int as usize] =
        sprite_mins[0 as libc::c_int as usize] +
            *origin.offset(0 as libc::c_int as isize);
    sprite_mins[1 as libc::c_int as usize] =
        sprite_mins[1 as libc::c_int as usize] +
            *origin.offset(1 as libc::c_int as isize);
    sprite_mins[2 as libc::c_int as usize] =
        sprite_mins[2 as libc::c_int as usize] +
            *origin.offset(2 as libc::c_int as isize);
    sprite_maxs[0 as libc::c_int as usize] =
        sprite_maxs[0 as libc::c_int as usize] +
            *origin.offset(0 as libc::c_int as isize);
    sprite_maxs[1 as libc::c_int as usize] =
        sprite_maxs[1 as libc::c_int as usize] +
            *origin.offset(1 as libc::c_int as isize);
    sprite_maxs[2 as libc::c_int as usize] =
        sprite_maxs[2 as libc::c_int as usize] +
            *origin.offset(2 as libc::c_int as isize);
    return false_0;
}
/*
================
R_GlowSightDistance

Set sprite brightness factor
================
*/
unsafe extern "C" fn R_SpriteGlowBlend(mut origin: *mut vec_t,
                                       mut rendermode: libc::c_int,
                                       mut renderfx: libc::c_int,
                                       mut pscale: *mut libc::c_float)
 -> libc::c_float {
    let mut dist: libc::c_float = 0.;
    let mut brightness: libc::c_float = 0.;
    let mut glowDist: vec3_t = [0.; 3];
    let mut tr_0: *mut pmtrace_t = 0 as *mut pmtrace_t;
    glowDist[0 as libc::c_int as usize] =
        *origin.offset(0 as libc::c_int as isize) -
            RI.vieworg[0 as libc::c_int as usize];
    glowDist[1 as libc::c_int as usize] =
        *origin.offset(1 as libc::c_int as isize) -
            RI.vieworg[1 as libc::c_int as usize];
    glowDist[2 as libc::c_int as usize] =
        *origin.offset(2 as libc::c_int as isize) -
            RI.vieworg[2 as libc::c_int as usize];
    dist =
        __tg_sqrt(glowDist[0 as libc::c_int as usize] *
                      glowDist[0 as libc::c_int as usize] +
                      glowDist[1 as libc::c_int as usize] *
                          glowDist[1 as libc::c_int as usize] +
                      glowDist[2 as libc::c_int as usize] *
                          glowDist[2 as libc::c_int as usize]);
    if RI.params as libc::c_uint & (1 as libc::c_uint) << 0 as libc::c_int ==
           0 as libc::c_int as libc::c_uint {
        tr_0 =
            gEngfuncs.EV_VisTraceLine.expect("non-null function pointer")(RI.vieworg.as_mut_ptr(),
                                                                          origin,
                                                                          if (*r_traceglow).value
                                                                                 !=
                                                                                 0.
                                                                             {
                                                                              0x4
                                                                                  as
                                                                                  libc::c_int
                                                                          } else {
                                                                              (0x4
                                                                                   as
                                                                                   libc::c_int)
                                                                                  |
                                                                                  0x1
                                                                                      as
                                                                                      libc::c_int
                                                                          });
        if (1.0f32 - (*tr_0).fraction) * dist > 8.0f32 { return 0.0f32 }
    }
    if renderfx == kRenderFxNoDissipation as libc::c_int { return 1.0f32 }
    brightness = 19000.0f32 / (dist * dist);
    brightness =
        if brightness >= 0.05f32 {
            if brightness < 1.0f32 { brightness } else { 1.0f32 }
        } else { 0.05f32 };
    *pscale *= dist * (1.0f32 / 200.0f32);
    return brightness;
}
/*
================
R_SpriteOccluded

Do occlusion test for glow-sprites
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_SpriteOccluded(mut e: *mut cl_entity_t,
                                          mut origin: *mut vec_t,
                                          mut pscale: *mut libc::c_float)
 -> qboolean {
    if (*e).curstate.rendermode == kRenderGlow as libc::c_int {
        let mut blend: libc::c_float = 0.;
        let mut v: vec3_t = [0.; 3];
        TriWorldToScreen(origin as *const libc::c_float, v.as_mut_ptr());
        // faded
        if v[0 as libc::c_int as usize] <
               RI.viewport[0 as libc::c_int as usize] as libc::c_float ||
               v[0 as libc::c_int as usize] >
                   (RI.viewport[0 as libc::c_int as usize] +
                        RI.viewport[2 as libc::c_int as usize]) as
                       libc::c_float {
            return true_0
        } // do scissor
        if v[1 as libc::c_int as usize] <
               RI.viewport[1 as libc::c_int as usize] as libc::c_float ||
               v[1 as libc::c_int as usize] >
                   (RI.viewport[1 as libc::c_int as usize] +
                        RI.viewport[3 as libc::c_int as usize]) as
                       libc::c_float {
            return true_0
        } // do scissor
        blend =
            R_SpriteGlowBlend(origin, (*e).curstate.rendermode,
                              (*e).curstate.renderfx, pscale);
        tr.blend *= blend;
        if blend <= 0.01f32 { return true_0 }
    } else if R_CullSpriteModel(e, origin) as u64 != 0 { return true_0 }
    return false_0;
}
/*
=================
R_DrawSpriteQuad
=================
*/
unsafe extern "C" fn R_DrawSpriteQuad(mut frame: *mut mspriteframe_t,
                                      mut org: *mut vec_t,
                                      mut v_right: *mut vec_t,
                                      mut v_up: *mut vec_t,
                                      mut scale: libc::c_float) {
    let mut point: vec3_t = [0.; 3];
    let mut image: *mut image_t = 0 as *mut image_t;
    r_stats.c_sprite_polys = r_stats.c_sprite_polys.wrapping_add(1);
    /*image = R_GetTexture(frame->gl_texturenum);
	r_affinetridesc.pskin = image->pixels[0];
	r_affinetridesc.skinwidth = image->width;
	r_affinetridesc.skinheight = image->height;*/
    TriBegin(2 as libc::c_int);
    TriTexCoord2f(0.0f32, 1.0f32);
    point[0 as libc::c_int as usize] =
        *org.offset(0 as libc::c_int as isize) +
            (*frame).down * scale * *v_up.offset(0 as libc::c_int as isize);
    point[1 as libc::c_int as usize] =
        *org.offset(1 as libc::c_int as isize) +
            (*frame).down * scale * *v_up.offset(1 as libc::c_int as isize);
    point[2 as libc::c_int as usize] =
        *org.offset(2 as libc::c_int as isize) +
            (*frame).down * scale * *v_up.offset(2 as libc::c_int as isize);
    point[0 as libc::c_int as usize] =
        point[0 as libc::c_int as usize] +
            (*frame).left * scale *
                *v_right.offset(0 as libc::c_int as isize);
    point[1 as libc::c_int as usize] =
        point[1 as libc::c_int as usize] +
            (*frame).left * scale *
                *v_right.offset(1 as libc::c_int as isize);
    point[2 as libc::c_int as usize] =
        point[2 as libc::c_int as usize] +
            (*frame).left * scale *
                *v_right.offset(2 as libc::c_int as isize);
    TriVertex3fv(point.as_mut_ptr());
    TriTexCoord2f(0.0f32, 0.0f32);
    point[0 as libc::c_int as usize] =
        *org.offset(0 as libc::c_int as isize) +
            (*frame).up * scale * *v_up.offset(0 as libc::c_int as isize);
    point[1 as libc::c_int as usize] =
        *org.offset(1 as libc::c_int as isize) +
            (*frame).up * scale * *v_up.offset(1 as libc::c_int as isize);
    point[2 as libc::c_int as usize] =
        *org.offset(2 as libc::c_int as isize) +
            (*frame).up * scale * *v_up.offset(2 as libc::c_int as isize);
    point[0 as libc::c_int as usize] =
        point[0 as libc::c_int as usize] +
            (*frame).left * scale *
                *v_right.offset(0 as libc::c_int as isize);
    point[1 as libc::c_int as usize] =
        point[1 as libc::c_int as usize] +
            (*frame).left * scale *
                *v_right.offset(1 as libc::c_int as isize);
    point[2 as libc::c_int as usize] =
        point[2 as libc::c_int as usize] +
            (*frame).left * scale *
                *v_right.offset(2 as libc::c_int as isize);
    TriVertex3fv(point.as_mut_ptr());
    TriTexCoord2f(1.0f32, 0.0f32);
    point[0 as libc::c_int as usize] =
        *org.offset(0 as libc::c_int as isize) +
            (*frame).up * scale * *v_up.offset(0 as libc::c_int as isize);
    point[1 as libc::c_int as usize] =
        *org.offset(1 as libc::c_int as isize) +
            (*frame).up * scale * *v_up.offset(1 as libc::c_int as isize);
    point[2 as libc::c_int as usize] =
        *org.offset(2 as libc::c_int as isize) +
            (*frame).up * scale * *v_up.offset(2 as libc::c_int as isize);
    point[0 as libc::c_int as usize] =
        point[0 as libc::c_int as usize] +
            (*frame).right * scale *
                *v_right.offset(0 as libc::c_int as isize);
    point[1 as libc::c_int as usize] =
        point[1 as libc::c_int as usize] +
            (*frame).right * scale *
                *v_right.offset(1 as libc::c_int as isize);
    point[2 as libc::c_int as usize] =
        point[2 as libc::c_int as usize] +
            (*frame).right * scale *
                *v_right.offset(2 as libc::c_int as isize);
    TriVertex3fv(point.as_mut_ptr());
    TriTexCoord2f(1.0f32, 1.0f32);
    point[0 as libc::c_int as usize] =
        *org.offset(0 as libc::c_int as isize) +
            (*frame).down * scale * *v_up.offset(0 as libc::c_int as isize);
    point[1 as libc::c_int as usize] =
        *org.offset(1 as libc::c_int as isize) +
            (*frame).down * scale * *v_up.offset(1 as libc::c_int as isize);
    point[2 as libc::c_int as usize] =
        *org.offset(2 as libc::c_int as isize) +
            (*frame).down * scale * *v_up.offset(2 as libc::c_int as isize);
    point[0 as libc::c_int as usize] =
        point[0 as libc::c_int as usize] +
            (*frame).right * scale *
                *v_right.offset(0 as libc::c_int as isize);
    point[1 as libc::c_int as usize] =
        point[1 as libc::c_int as usize] +
            (*frame).right * scale *
                *v_right.offset(1 as libc::c_int as isize);
    point[2 as libc::c_int as usize] =
        point[2 as libc::c_int as usize] +
            (*frame).right * scale *
                *v_right.offset(2 as libc::c_int as isize);
    TriVertex3fv(point.as_mut_ptr());
    TriEnd();
}
unsafe extern "C" fn R_SpriteHasLightmap(mut e: *mut cl_entity_t,
                                         mut texFormat: libc::c_int)
 -> qboolean {
    if (*r_sprite_lighting).value == 0. { return false_0 }
    if texFormat != SPR_ALPHTEST as libc::c_int { return false_0 }
    if (*e).curstate.effects as libc::c_uint &
           (1 as libc::c_uint) << 27 as libc::c_int != 0 {
        return false_0
    }
    if (*e).curstate.renderamt <= 127 as libc::c_int { return false_0 }
    match (*e).curstate.rendermode {
        0 | 4 | 2 => { }
        _ => { return false_0 }
    }
    return true_0;
}
/*
=================
R_SpriteAllowLerping
=================
*/
unsafe extern "C" fn R_SpriteAllowLerping(mut e: *mut cl_entity_t,
                                          mut psprite: *mut msprite_t)
 -> qboolean {
    if (*r_sprite_lerping).value == 0. { return false_0 }
    if (*psprite).numframes <= 1 as libc::c_int { return false_0 }
    if (*psprite).texFormat as libc::c_int != SPR_ADDITIVE as libc::c_int {
        return false_0
    }
    if (*e).curstate.rendermode == kRenderNormal as libc::c_int ||
           (*e).curstate.rendermode == kRenderTransAlpha as libc::c_int {
        return false_0
    }
    return true_0;
}
/*
=================
R_DrawSpriteModel
=================
*/
#[no_mangle]
pub unsafe extern "C" fn R_DrawSpriteModel(mut e: *mut cl_entity_t) {
    let mut frame: *mut mspriteframe_t =
        0 as *mut mspriteframe_t; // set render origin
    let mut oldframe: *mut mspriteframe_t = 0 as *mut mspriteframe_t;
    let mut psprite: *mut msprite_t = 0 as *mut msprite_t;
    let mut model: *mut model_t = 0 as *mut model_t;
    let mut i: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut angle: libc::c_float = 0.;
    let mut dot: libc::c_float = 0.;
    let mut sr: libc::c_float = 0.;
    let mut cr: libc::c_float = 0.;
    let mut lerp: libc::c_float = 1.0f32;
    let mut ilerp: libc::c_float = 0.;
    let mut scale: libc::c_float = 0.;
    let mut v_forward: vec3_t = [0.; 3];
    let mut v_right: vec3_t = [0.; 3];
    let mut v_up: vec3_t = [0.; 3];
    let mut origin: vec3_t = [0.; 3];
    let mut color: vec3_t = [0.; 3];
    let mut color2: vec3_t = [0.; 3];
    if RI.params as libc::c_uint & (1 as libc::c_uint) << 0 as libc::c_int !=
           0 {
        return
    }
    model = (*e).model;
    psprite = (*model).cache.data as *mut msprite_t;
    origin[0 as libc::c_int as usize] =
        (*e).origin[0 as libc::c_int as usize];
    origin[1 as libc::c_int as usize] =
        (*e).origin[1 as libc::c_int as usize];
    origin[2 as libc::c_int as usize] =
        (*e).origin[2 as libc::c_int as usize];
    // do movewith
    if (*e).curstate.aiment > 0 as libc::c_int &&
           (*e).curstate.movetype == 12 as libc::c_int {
        let mut parent: *mut cl_entity_t =
            0 as *mut cl_entity_t; // sprite culled
        parent =
            gEngfuncs.GetEntityByIndex.expect("non-null function pointer")((*e).curstate.aiment);
        if !parent.is_null() && !(*parent).model.is_null() {
            if (*(*parent).model).type_0 as libc::c_int ==
                   mod_studio as libc::c_int &&
                   (*e).curstate.body > 0 as libc::c_int {
                let mut num: libc::c_int =
                    if (*e).curstate.body >= 1 as libc::c_int {
                        if (*e).curstate.body < 64 as libc::c_int {
                            (*e).curstate.body
                        } else { 64 as libc::c_int }
                    } else { 1 as libc::c_int };
                origin[0 as libc::c_int as usize] =
                    (*parent).attachment[(num - 1 as libc::c_int) as
                                             usize][0 as libc::c_int as
                                                        usize];
                origin[1 as libc::c_int as usize] =
                    (*parent).attachment[(num - 1 as libc::c_int) as
                                             usize][1 as libc::c_int as
                                                        usize];
                origin[2 as libc::c_int as usize] =
                    (*parent).attachment[(num - 1 as libc::c_int) as
                                             usize][2 as libc::c_int as usize]
            } else {
                origin[0 as libc::c_int as usize] =
                    (*parent).origin[0 as libc::c_int as usize];
                origin[1 as libc::c_int as usize] =
                    (*parent).origin[1 as libc::c_int as usize];
                origin[2 as libc::c_int as usize] =
                    (*parent).origin[2 as libc::c_int as usize]
            }
        }
    }
    scale = (*e).curstate.scale;
    if scale == 0. { scale = 1.0f32 }
    if R_SpriteOccluded(e, origin.as_mut_ptr(), &mut scale) as u64 != 0 {
        return
    }
    r_stats.c_sprite_models_drawn =
        r_stats.c_sprite_models_drawn.wrapping_add(1);
    if (*e).curstate.rendermode == kRenderGlow as libc::c_int ||
           (*e).curstate.rendermode == kRenderTransAdd as libc::c_int {
        R_AllowFog(false_0);
    }
    GL_SetRenderMode((*e).curstate.rendermode);
    // NOTE: never pass sprites with rendercolor '0 0 0' it's a stupid Valve Hammer Editor bug
    if (*e).curstate.rendercolor.r as libc::c_int != 0 ||
           (*e).curstate.rendercolor.g as libc::c_int != 0 ||
           (*e).curstate.rendercolor.b as libc::c_int != 0 {
        color[0 as libc::c_int as usize] =
            (*e).curstate.rendercolor.r as libc::c_float *
                (1.0f32 / 255.0f32);
        color[1 as libc::c_int as usize] =
            (*e).curstate.rendercolor.g as libc::c_float *
                (1.0f32 / 255.0f32);
        color[2 as libc::c_int as usize] =
            (*e).curstate.rendercolor.b as libc::c_float * (1.0f32 / 255.0f32)
    } else {
        color[0 as libc::c_int as usize] = 1.0f32;
        color[1 as libc::c_int as usize] = 1.0f32;
        color[2 as libc::c_int as usize] = 1.0f32
    }
    if R_SpriteHasLightmap(e, (*psprite).texFormat as libc::c_int) as u64 != 0
       {
        let mut lightColor: colorVec =
            R_LightPoint(origin.as_mut_ptr() as *const vec_t);
        // NOTE: sprites with 'lightmap' looks ugly when alpha func is GL_GREATER 0.0
	//	pglAlphaFunc( GL_GREATER, 0.5f );
        color2[0 as libc::c_int as usize] =
            lightColor.r as libc::c_float * (1.0f32 / 255.0f32);
        color2[1 as libc::c_int as usize] =
            lightColor.g as libc::c_float * (1.0f32 / 255.0f32);
        color2[2 as libc::c_int as usize] =
            lightColor.b as libc::c_float * (1.0f32 / 255.0f32)
    }
    if R_SpriteAllowLerping(e, psprite) as u64 != 0 {
        lerp = R_GetSpriteFrameInterpolant(e, &mut oldframe, &mut frame)
    } else {
        oldframe =
            R_GetSpriteFrame(model, (*e).curstate.frame as libc::c_int,
                             (*e).angles[1 as libc::c_int as usize]);
        frame = oldframe
    }
    type_0 = (*psprite).type_0 as libc::c_int;
    // FIXME: collect light from dlights?
    // automatically roll parallel sprites if requested
    if (*e).angles[2 as libc::c_int as usize] != 0.0f32 &&
           type_0 == SPR_FWD_PARALLEL as libc::c_int {
        type_0 = SPR_FWD_PARALLEL_ORIENTED as libc::c_int
    } // to avoid z-fighting
    match type_0 {
        3 => {
            AngleVectors((*e).angles.as_mut_ptr() as *const vec_t,
                         v_forward.as_mut_ptr(), v_right.as_mut_ptr(),
                         v_up.as_mut_ptr()); // invisible
            v_forward[0 as libc::c_int as usize] =
                v_forward[0 as libc::c_int as usize] * 0.01f32;
            v_forward[1 as libc::c_int as usize] =
                v_forward[1 as libc::c_int as usize] * 0.01f32;
            v_forward[2 as libc::c_int as usize] =
                v_forward[2 as libc::c_int as usize] * 0.01f32;
            origin[0 as libc::c_int as usize] =
                origin[0 as libc::c_int as usize] -
                    v_forward[0 as libc::c_int as usize];
            origin[1 as libc::c_int as usize] =
                origin[1 as libc::c_int as usize] -
                    v_forward[1 as libc::c_int as usize];
            origin[2 as libc::c_int as usize] =
                origin[2 as libc::c_int as usize] -
                    v_forward[2 as libc::c_int as usize]
        }
        1 => {
            v_right[0 as libc::c_int as usize] =
                origin[1 as libc::c_int as usize] -
                    RI.vieworg[1 as libc::c_int as usize];
            v_right[1 as libc::c_int as usize] =
                -(origin[0 as libc::c_int as usize] -
                      RI.vieworg[0 as libc::c_int as usize]);
            v_right[2 as libc::c_int as usize] = 0.0f32;
            v_up[0 as libc::c_int as usize] = 0.0f32;
            v_up[1 as libc::c_int as usize] = 0.0f32;
            v_up[2 as libc::c_int as usize] = 1.0f32;
            let mut ilength: libc::c_float =
                __tg_sqrt(v_right[0 as libc::c_int as usize] *
                              v_right[0 as libc::c_int as usize] +
                              v_right[1 as libc::c_int as usize] *
                                  v_right[1 as libc::c_int as usize] +
                              v_right[2 as libc::c_int as usize] *
                                  v_right[2 as libc::c_int as usize]);
            if ilength != 0. { ilength = 1.0f32 / ilength }
            v_right[0 as libc::c_int as usize] *= ilength;
            v_right[1 as libc::c_int as usize] *= ilength;
            v_right[2 as libc::c_int as usize] *= ilength
        }
        0 => {
            dot = RI.vforward[2 as libc::c_int as usize];
            if dot > 0.999848f32 || dot < -0.999848f32 {
                // cos(1 degree) = 0.999848
                return
            }
            v_up[0 as libc::c_int as usize] = 0.0f32;
            v_up[1 as libc::c_int as usize] = 0.0f32;
            v_up[2 as libc::c_int as usize] = 1.0f32;
            v_right[0 as libc::c_int as usize] =
                RI.vforward[1 as libc::c_int as usize];
            v_right[1 as libc::c_int as usize] =
                -RI.vforward[0 as libc::c_int as usize];
            v_right[2 as libc::c_int as usize] = 0.0f32;
            let mut ilength_0: libc::c_float =
                __tg_sqrt(v_right[0 as libc::c_int as usize] *
                              v_right[0 as libc::c_int as usize] +
                              v_right[1 as libc::c_int as usize] *
                                  v_right[1 as libc::c_int as usize] +
                              v_right[2 as libc::c_int as usize] *
                                  v_right[2 as libc::c_int as usize]);
            if ilength_0 != 0. { ilength_0 = 1.0f32 / ilength_0 }
            v_right[0 as libc::c_int as usize] *= ilength_0;
            v_right[1 as libc::c_int as usize] *= ilength_0;
            v_right[2 as libc::c_int as usize] *= ilength_0
        }
        4 => {
            angle =
                (*e).angles[2 as libc::c_int as usize] *
                    ((3.14159265358979323846f64 *
                          2 as libc::c_int as libc::c_double) as libc::c_float
                         / 360.0f32);
            SinCos(angle, &mut sr, &mut cr);
            i = 0 as libc::c_int;
            while i < 3 as libc::c_int {
                v_right[i as usize] =
                    RI.vright[i as usize] * cr + RI.vup[i as usize] * sr;
                v_up[i as usize] =
                    RI.vright[i as usize] * -sr + RI.vup[i as usize] * cr;
                i += 1
            }
        }
        2 | _ => {
            // normal sprite
            v_right[0 as libc::c_int as usize] =
                RI.vright[0 as libc::c_int as usize];
            v_right[1 as libc::c_int as usize] =
                RI.vright[1 as libc::c_int as usize];
            v_right[2 as libc::c_int as usize] =
                RI.vright[2 as libc::c_int as usize];
            v_up[0 as libc::c_int as usize] =
                RI.vup[0 as libc::c_int as usize];
            v_up[1 as libc::c_int as usize] =
                RI.vup[1 as libc::c_int as usize];
            v_up[2 as libc::c_int as usize] =
                RI.vup[2 as libc::c_int as usize]
        }
    }
    //if( psprite->facecull == SPR_CULL_NONE )
		//GL_Cull( GL_NONE );
    if oldframe == frame {
        // draw the single non-lerped frame
        _TriColor4f(color[0 as libc::c_int as usize],
                    color[1 as libc::c_int as usize],
                    color[2 as libc::c_int as usize], tr.blend);
        GL_Bind(XASH_TEXTURE0 as libc::c_int,
                (*frame).gl_texturenum as libc::c_uint);
        R_DrawSpriteQuad(frame, origin.as_mut_ptr(), v_right.as_mut_ptr(),
                         v_up.as_mut_ptr(), scale);
    } else {
        // draw two combined lerped frames
        lerp =
            if lerp >= 0.0f32 {
                if lerp < 1.0f32 { lerp } else { 1.0f32 }
            } else { 0.0f32 };
        ilerp = 1.0f32 - lerp;
        if ilerp != 0.0f32 {
            _TriColor4f(color[0 as libc::c_int as usize],
                        color[1 as libc::c_int as usize],
                        color[2 as libc::c_int as usize], tr.blend * ilerp);
            GL_Bind(XASH_TEXTURE0 as libc::c_int,
                    (*oldframe).gl_texturenum as libc::c_uint);
            R_DrawSpriteQuad(oldframe, origin.as_mut_ptr(),
                             v_right.as_mut_ptr(), v_up.as_mut_ptr(), scale);
        }
        if lerp != 0.0f32 {
            _TriColor4f(color[0 as libc::c_int as usize],
                        color[1 as libc::c_int as usize],
                        color[2 as libc::c_int as usize], tr.blend * lerp);
            GL_Bind(XASH_TEXTURE0 as libc::c_int,
                    (*frame).gl_texturenum as libc::c_uint);
            R_DrawSpriteQuad(frame, origin.as_mut_ptr(), v_right.as_mut_ptr(),
                             v_up.as_mut_ptr(), scale);
        }
    };
}
