#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type file_s;
    pub type grasshdr_s;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn ceil(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn floor(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn CL_GetEntityByIndex(index: libc::c_int) -> *mut cl_entity_s;
    #[no_mangle]
    fn CheckNewDspPresets();
    #[no_mangle]
    fn S_FindRawChannel(entnum: libc::c_int, create: qboolean)
     -> *mut rawchan_t;
    #[no_mangle]
    fn VOX_ModifyPitch(ch: *mut channel_t, pitch: libc::c_float)
     -> libc::c_float;
    #[no_mangle]
    fn SND_MoveMouth8(ch: *mut channel_t, pSource: *mut wavdata_t,
                      count: libc::c_int);
    #[no_mangle]
    fn SND_MoveMouth16(ch: *mut channel_t, pSource: *mut wavdata_t,
                       count: libc::c_int);
    #[no_mangle]
    fn VOX_MixDataToDevice(pChannel: *mut channel_t, sampleCount: libc::c_int,
                           outputRate: libc::c_int, outputOffset: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn DSP_Process(idsp: libc::c_int, pbfront: *mut portable_samplepair_t,
                   sampleCount: libc::c_int);
    #[no_mangle]
    fn S_GetOutputData(pSource: *mut wavdata_t, pData: *mut *mut libc::c_void,
                       samplePosition: libc::c_int, sampleCount: libc::c_int,
                       use_loop: qboolean) -> libc::c_int;
    #[no_mangle]
    fn S_FreeChannel(ch: *mut channel_t);
    #[no_mangle]
    fn S_GetMusicVolume() -> libc::c_float;
    #[no_mangle]
    fn S_GetMasterVolume() -> libc::c_float;
    #[no_mangle]
    fn S_LoadSound(sfx: *mut sfx_t) -> *mut wavdata_t;
    #[no_mangle]
    static mut s_lerping: convar_t;
    #[no_mangle]
    static mut dma: dma_t;
    #[no_mangle]
    static mut idsp_room: libc::c_int;
    #[no_mangle]
    static mut s_listener: listener_t;
    #[no_mangle]
    static mut paintedtime: libc::c_int;
    #[no_mangle]
    static mut total_channels: libc::c_int;
    #[no_mangle]
    static mut raw_channels: [*mut rawchan_t; 16];
    #[no_mangle]
    static mut channels: [channel_t; 320];
    #[no_mangle]
    static mut cl: client_t;
    #[no_mangle]
    static mut cls: client_static_t;
}
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type size_t = libc::c_ulong;
pub type uint = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type byte = libc::c_uchar;
pub type vec_t = libc::c_float;
pub type vec3_t = [vec_t; 3];
pub type poolhandle_t = uint32_t;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type dword = libc::c_uint;
pub type string = [libc::c_char; 256];
pub type file_t = file_s;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct convar_s {
    pub name: *mut libc::c_char,
    pub string: *mut libc::c_char,
    pub flags: libc::c_int,
    pub value: libc::c_float,
    pub next: *mut convar_s,
    pub desc: *mut libc::c_char,
    pub def_string: *mut libc::c_char,
}
pub type convar_t = convar_s;
pub type keydest_t = libc::c_uint;
pub const key_message: keydest_t = 3;
pub const key_menu: keydest_t = 2;
pub const key_game: keydest_t = 1;
pub const key_console: keydest_t = 0;
pub type netsrc_t = libc::c_uint;
pub const NS_COUNT: netsrc_t = 2;
pub const NS_SERVER: netsrc_t = 1;
pub const NS_CLIENT: netsrc_t = 0;
pub type netadrtype_t = libc::c_uint;
pub const NA_BROADCAST_IPX: netadrtype_t = 5;
pub const NA_IPX: netadrtype_t = 4;
pub const NA_IP: netadrtype_t = 3;
pub const NA_BROADCAST: netadrtype_t = 2;
pub const NA_LOOPBACK: netadrtype_t = 1;
pub const NA_UNUSED: netadrtype_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct netadr_s {
    pub type_0: netadrtype_t,
    pub ip: [libc::c_uchar; 4],
    pub ipx: [libc::c_uchar; 10],
    pub port: libc::c_ushort,
}
pub type netadr_t = netadr_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wavdata_t {
    pub rate: word,
    pub width: byte,
    pub channels: byte,
    pub loopStart: libc::c_int,
    pub samples: libc::c_int,
    pub type_0: uint,
    pub flags: uint,
    pub buffer: *mut byte,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sizebuf_s {
    pub bOverflow: qboolean,
    pub pDebugName: *const libc::c_char,
    pub pData: *mut byte,
    pub iCurBit: libc::c_int,
    pub nDataBits: libc::c_int,
}
pub type sizebuf_t = sizebuf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct portable_samplepair_t {
    pub left: libc::c_int,
    pub right: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct paintbuffer_t {
    pub factive: qboolean,
    pub pbuf: *mut portable_samplepair_t,
    pub ifilter: libc::c_int,
    pub fltmem: [[portable_samplepair_t; 3]; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfx_s {
    pub name: [libc::c_char; 64],
    pub cache: *mut wavdata_t,
    pub servercount: libc::c_int,
    pub hashValue: uint,
    pub hashNext: *mut sfx_s,
}
pub type sfx_t = sfx_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct snd_format_s {
    pub speed: libc::c_uint,
    pub width: libc::c_uint,
    pub channels: libc::c_uint,
}
pub type snd_format_t = snd_format_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dma_t {
    pub format: snd_format_t,
    pub samples: libc::c_int,
    pub samplepos: libc::c_int,
    pub buffer: *mut byte,
    pub initialized: qboolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct voxword_s {
    pub volume: libc::c_int,
    pub pitch: libc::c_int,
    pub start: libc::c_int,
    pub end: libc::c_int,
    pub cbtrim: libc::c_int,
    pub fKeepCached: libc::c_int,
    pub samplefrac: libc::c_int,
    pub timecompress: libc::c_int,
    pub sfx: *mut sfx_t,
}
pub type voxword_t = voxword_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct channel_s {
    pub name: [libc::c_char; 16],
    pub sfx: *mut sfx_t,
    pub leftvol: libc::c_int,
    pub rightvol: libc::c_int,
    pub entnum: libc::c_int,
    pub entchannel: libc::c_int,
    pub origin: vec3_t,
    pub dist_mult: libc::c_float,
    pub master_vol: libc::c_int,
    pub isSentence: qboolean,
    pub basePitch: libc::c_int,
    pub pitch: libc::c_float,
    pub use_loop: qboolean,
    pub staticsound: qboolean,
    pub localsound: qboolean,
    pub pMixer: mixer_t,
    pub wordIndex: libc::c_int,
    pub currentWord: *mut mixer_t,
    pub words: [voxword_t; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mixer_t {
    pub sample: libc::c_double,
    pub pData: *mut wavdata_t,
    pub forcedEndSample: libc::c_double,
    pub finished: qboolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rawchan_s {
    pub entnum: libc::c_int,
    pub master_vol: libc::c_int,
    pub leftvol: libc::c_int,
    pub rightvol: libc::c_int,
    pub dist_mult: libc::c_float,
    pub origin: vec3_t,
    pub s_rawend: uint,
    pub sound_info: wavdata_t,
    pub oldtime: libc::c_float,
    pub max_samples: size_t,
    pub rawsamples: [portable_samplepair_t; 1],
}
pub type rawchan_t = rawchan_s;
pub type channel_t = channel_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct listener_t {
    pub origin: vec3_t,
    pub velocity: vec3_t,
    pub forward: vec3_t,
    pub right: vec3_t,
    pub up: vec3_t,
    pub entnum: libc::c_int,
    pub waterlevel: libc::c_int,
    pub frametime: libc::c_float,
    pub active: qboolean,
    pub inmenu: qboolean,
    pub paused: qboolean,
    pub streaming: qboolean,
    pub stream_paused: qboolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client_static_t {
    pub state: connstate_t,
    pub initialized: qboolean,
    pub changelevel: qboolean,
    pub changedemo: qboolean,
    pub timestart: libc::c_double,
    pub disable_screen: libc::c_float,
    pub disable_servercount: libc::c_int,
    pub draw_changelevel: qboolean,
    pub key_dest: keydest_t,
    pub mempool: poolhandle_t,
    pub hltv_listen_address: netadr_t,
    pub signon: libc::c_int,
    pub quakePort: libc::c_int,
    pub servername: [libc::c_char; 64],
    pub connect_time: libc::c_double,
    pub max_fragment_size: libc::c_int,
    pub connect_retry: libc::c_int,
    pub spectator: qboolean,
    pub spectator_state: local_state_t,
    pub userinfo: [libc::c_char; 256],
    pub physinfo: [libc::c_char; 256],
    pub datagram: sizebuf_t,
    pub datagram_buf: [byte; 16384],
    pub netchan: netchan_t,
    pub challenge: libc::c_int,
    pub packet_loss: libc::c_float,
    pub packet_loss_recalc_time: libc::c_double,
    pub starting_count: libc::c_int,
    pub nextcmdtime: libc::c_float,
    pub lastoutgoingcommand: libc::c_int,
    pub lastupdate_sequence: libc::c_int,
    pub td_lastframe: libc::c_int,
    pub td_startframe: libc::c_int,
    pub td_starttime: libc::c_double,
    pub forcetrack: libc::c_int,
    pub pauseIcon: libc::c_int,
    pub tileImage: libc::c_int,
    pub loadingBar: libc::c_int,
    pub creditsFont: cl_font_t,
    pub latency: libc::c_float,
    pub num_client_entities: libc::c_int,
    pub next_client_entities: libc::c_int,
    pub packet_entities: *mut entity_state_t,
    pub predicted_players: [predicted_player_t; 32],
    pub correction_time: libc::c_double,
    pub scrshot_request: scrshot_t,
    pub scrshot_action: scrshot_t,
    pub envshot_vieworg: *const libc::c_float,
    pub envshot_viewsize: libc::c_int,
    pub envshot_disable_vis: qboolean,
    pub shotname: string,
    pub dl: incomingtransfer_t,
    pub demonum: libc::c_int,
    pub olddemonum: libc::c_int,
    pub demos: [[libc::c_char; 64]; 32],
    pub demos_pending: qboolean,
    pub movienum: libc::c_int,
    pub movies: [string; 8],
    pub demorecording: qboolean,
    pub demoplayback: libc::c_int,
    pub demowaiting: qboolean,
    pub timedemo: qboolean,
    pub demoname: string,
    pub demotime: libc::c_double,
    pub set_lastdemo: qboolean,
    pub demofile: *mut file_t,
    pub demoheader: *mut file_t,
    pub internetservers_wait: qboolean,
    pub internetservers_pending: qboolean,
    pub legacymode: qboolean,
    pub legacyserver: netadr_t,
    pub legacyservers: [netadr_t; 256],
    pub legacyservercount: libc::c_int,
    pub extensions: libc::c_int,
    pub serveradr: netadr_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct incomingtransfer_t {
    pub doneregistering: qboolean,
    pub percent: libc::c_int,
    pub downloadrequested: qboolean,
    pub rgStats: [downloadtime_t; 8],
    pub nCurStat: libc::c_int,
    pub nTotalSize: libc::c_int,
    pub nTotalToTransfer: libc::c_int,
    pub nRemainingToTransfer: libc::c_int,
    pub fLastStatusUpdate: libc::c_float,
    pub custom: qboolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct downloadtime_t {
    pub bUsed: qboolean,
    pub fTime: libc::c_float,
    pub nBytesRemaining: libc::c_int,
}
pub type scrshot_t = libc::c_uint;
pub const scrshot_mapshot: scrshot_t = 7;
pub const scrshot_skyshot: scrshot_t = 6;
pub const scrshot_envshot: scrshot_t = 5;
pub const scrshot_savegame: scrshot_t = 4;
pub const scrshot_plaque: scrshot_t = 3;
pub const scrshot_snapshot: scrshot_t = 2;
pub const scrshot_normal: scrshot_t = 1;
pub const scrshot_inactive: scrshot_t = 0;
pub type predicted_player_t = cl_predicted_player_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cl_predicted_player_s {
    pub movetype: libc::c_int,
    pub solid: libc::c_int,
    pub usehull: libc::c_int,
    pub active: qboolean,
    pub origin: vec3_t,
    pub angles: vec3_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cl_font_t {
    pub hFontTexture: libc::c_int,
    pub fontRc: [wrect_t; 256],
    pub charWidths: [byte; 256],
    pub charHeight: libc::c_int,
    pub type_0: libc::c_int,
    pub valid: qboolean,
}
pub type wrect_t = wrect_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wrect_s {
    pub left: libc::c_int,
    pub right: libc::c_int,
    pub top: libc::c_int,
    pub bottom: libc::c_int,
}
pub type netchan_t = netchan_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct netchan_s {
    pub sock: netsrc_t,
    pub remote_address: netadr_t,
    pub qport: libc::c_int,
    pub last_received: libc::c_double,
    pub connect_time: libc::c_double,
    pub rate: libc::c_double,
    pub cleartime: libc::c_double,
    pub incoming_sequence: libc::c_uint,
    pub incoming_acknowledged: libc::c_uint,
    pub incoming_reliable_acknowledged: libc::c_uint,
    pub incoming_reliable_sequence: libc::c_uint,
    pub outgoing_sequence: libc::c_uint,
    pub reliable_sequence: libc::c_uint,
    pub last_reliable_sequence: libc::c_uint,
    pub client: *mut libc::c_void,
    pub pfnBlockSize: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                  _: fragsize_t)
                                 -> libc::c_int>,
    pub message: sizebuf_t,
    pub message_buf: [byte; 131120],
    pub reliable_length: libc::c_int,
    pub reliable_buf: [byte; 131120],
    pub waitlist: [*mut fragbufwaiting_t; 2],
    pub reliable_fragment: [libc::c_int; 2],
    pub reliable_fragid: [uint; 2],
    pub fragbufs: [*mut fragbuf_t; 2],
    pub fragbufcount: [libc::c_int; 2],
    pub frag_startpos: [libc::c_int; 2],
    pub frag_length: [libc::c_int; 2],
    pub incomingbufs: [*mut fragbuf_t; 2],
    pub incomingready: [qboolean; 2],
    pub incomingfilename: [libc::c_char; 260],
    pub tempbuffer: *mut libc::c_void,
    pub tempbuffersize: libc::c_int,
    pub flow: [flow_t; 2],
    pub total_sended: size_t,
    pub total_received: size_t,
    pub split: qboolean,
    pub maxpacket: libc::c_uint,
    pub splitid: libc::c_uint,
    pub netsplit: netsplit_t,
}
pub type netsplit_t = netsplit_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct netsplit_s {
    pub packets: [netsplit_chain_packet_t; 8],
    pub total_received: uint64_t,
    pub total_received_uncompressed: uint64_t,
}
pub type netsplit_chain_packet_t = netsplit_chain_packet_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct netsplit_chain_packet_s {
    pub recieved_v: [uint32_t; 8],
    pub id: uint32_t,
    pub data: [byte; 131072],
    pub received: byte,
    pub count: byte,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flow_t {
    pub stats: [flowstats_t; 32],
    pub current: libc::c_int,
    pub nextcompute: libc::c_double,
    pub kbytespersec: libc::c_float,
    pub avgkbytespersec: libc::c_float,
    pub totalbytes: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flowstats_t {
    pub size: libc::c_int,
    pub time: libc::c_double,
}
pub type fragbuf_t = fragbuf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fragbuf_s {
    pub next: *mut fragbuf_s,
    pub bufferid: libc::c_int,
    pub frag_message: sizebuf_t,
    pub frag_message_buf: [byte; 65535],
    pub isfile: qboolean,
    pub isbuffer: qboolean,
    pub iscompressed: qboolean,
    pub filename: [libc::c_char; 260],
    pub foffset: libc::c_int,
    pub size: libc::c_int,
}
pub type fragbufwaiting_t = fbufqueue_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fbufqueue_s {
    pub next: *mut fbufqueue_s,
    pub fragbufcount: libc::c_int,
    pub fragbufs: *mut fragbuf_t,
}
pub type fragsize_t = fragsize_e;
pub type fragsize_e = libc::c_uint;
pub const FRAGSIZE_UNRELIABLE: fragsize_e = 2;
pub const FRAGSIZE_SPLIT: fragsize_e = 1;
pub const FRAGSIZE_FRAG: fragsize_e = 0;
pub type local_state_t = local_state_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct local_state_s {
    pub playerstate: entity_state_t,
    pub client: clientdata_t,
    pub weapondata: [weapon_data_t; 64],
}
pub type weapon_data_t = weapon_data_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct weapon_data_s {
    pub m_iId: libc::c_int,
    pub m_iClip: libc::c_int,
    pub m_flNextPrimaryAttack: libc::c_float,
    pub m_flNextSecondaryAttack: libc::c_float,
    pub m_flTimeWeaponIdle: libc::c_float,
    pub m_fInReload: libc::c_int,
    pub m_fInSpecialReload: libc::c_int,
    pub m_flNextReload: libc::c_float,
    pub m_flPumpTime: libc::c_float,
    pub m_fReloadTime: libc::c_float,
    pub m_fAimedDamage: libc::c_float,
    pub m_fNextAimBonus: libc::c_float,
    pub m_fInZoom: libc::c_int,
    pub m_iWeaponState: libc::c_int,
    pub iuser1: libc::c_int,
    pub iuser2: libc::c_int,
    pub iuser3: libc::c_int,
    pub iuser4: libc::c_int,
    pub fuser1: libc::c_float,
    pub fuser2: libc::c_float,
    pub fuser3: libc::c_float,
    pub fuser4: libc::c_float,
}
pub type clientdata_t = clientdata_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clientdata_s {
    pub origin: vec3_t,
    pub velocity: vec3_t,
    pub viewmodel: libc::c_int,
    pub punchangle: vec3_t,
    pub flags: libc::c_int,
    pub waterlevel: libc::c_int,
    pub watertype: libc::c_int,
    pub view_ofs: vec3_t,
    pub health: libc::c_float,
    pub bInDuck: libc::c_int,
    pub weapons: libc::c_int,
    pub flTimeStepSound: libc::c_int,
    pub flDuckTime: libc::c_int,
    pub flSwimTime: libc::c_int,
    pub waterjumptime: libc::c_int,
    pub maxspeed: libc::c_float,
    pub fov: libc::c_float,
    pub weaponanim: libc::c_int,
    pub m_iId: libc::c_int,
    pub ammo_shells: libc::c_int,
    pub ammo_nails: libc::c_int,
    pub ammo_cells: libc::c_int,
    pub ammo_rockets: libc::c_int,
    pub m_flNextAttack: libc::c_float,
    pub tfstate: libc::c_int,
    pub pushmsec: libc::c_int,
    pub deadflag: libc::c_int,
    pub physinfo: [libc::c_char; 256],
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
pub type connstate_t = connstate_e;
pub type connstate_e = libc::c_uint;
pub const ca_cinematic: connstate_e = 5;
pub const ca_active: connstate_e = 4;
pub const ca_validate: connstate_e = 3;
pub const ca_connected: connstate_e = 2;
pub const ca_connecting: connstate_e = 1;
pub const ca_disconnected: connstate_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client_t {
    pub servercount: libc::c_int,
    pub validsequence: libc::c_int,
    pub parsecount: libc::c_int,
    pub parsecountmod: libc::c_int,
    pub video_prepped: qboolean,
    pub audio_prepped: qboolean,
    pub paused: qboolean,
    pub delta_sequence: libc::c_int,
    pub mtime: [libc::c_double; 2],
    pub lerpFrac: libc::c_float,
    pub last_command_ack: libc::c_int,
    pub last_incoming_sequence: libc::c_int,
    pub send_reply: qboolean,
    pub background: qboolean,
    pub first_frame: qboolean,
    pub proxy_redirect: qboolean,
    pub skip_interp: qboolean,
    pub checksum: uint,
    pub frames: [frame_t; 64],
    pub commands: [runcmd_t; 64],
    pub predicted_frames: [local_state_t; 64],
    pub time: libc::c_double,
    pub oldtime: libc::c_double,
    pub timedelta: libc::c_float,
    pub serverinfo: [libc::c_char; 512],
    pub players: [player_info_t; 32],
    pub lastresourcecheck: libc::c_double,
    pub downloadUrl: string,
    pub events: event_state_t,
    pub local: cl_local_data_t,
    pub cmd: *mut usercmd_t,
    pub viewentity: libc::c_int,
    pub viewangles: vec3_t,
    pub viewheight: vec3_t,
    pub punchangle: vec3_t,
    pub intermission: libc::c_int,
    pub crosshairangle: vec3_t,
    pub predicted_angle: [pred_viewangle_t; 16],
    pub angle_position: libc::c_int,
    pub addangletotal: libc::c_float,
    pub prevaddangletotal: libc::c_float,
    pub simorg: vec3_t,
    pub simvel: vec3_t,
    pub playernum: libc::c_int,
    pub maxclients: libc::c_int,
    pub instanced_baseline: [entity_state_t; 64],
    pub instanced_baseline_count: libc::c_int,
    pub sound_precache: [[libc::c_char; 64]; 2048],
    pub event_precache: [[libc::c_char; 64]; 1024],
    pub files_precache: [[libc::c_char; 64]; 1024],
    pub lightstyles: [lightstyle_t; 64],
    pub models: [*mut model_t; 1025],
    pub nummodels: libc::c_int,
    pub numfiles: libc::c_int,
    pub consistency_list: [consistency_t; 1024],
    pub num_consistency: libc::c_int,
    pub need_force_consistency_response: qboolean,
    pub resourcesonhand: resource_t,
    pub resourcesneeded: resource_t,
    pub resourcelist: [resource_t; 5120],
    pub num_resources: libc::c_int,
    pub sound_index: [libc::c_short; 2048],
    pub decal_index: [libc::c_short; 512],
    pub worldmodel: *mut model_t,
    pub lostpackets: libc::c_int,
}
pub type consistency_t = consistency_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct consistency_s {
    pub filename: *const libc::c_char,
    pub orig_index: libc::c_int,
    pub check_type: libc::c_int,
    pub issound: qboolean,
    pub value: libc::c_int,
    pub mins: vec3_t,
    pub maxs: vec3_t,
}
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
pub struct pred_viewangle_t {
    pub starttime: libc::c_float,
    pub total: libc::c_float,
}
pub type usercmd_t = usercmd_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct usercmd_s {
    pub lerp_msec: libc::c_short,
    pub msec: byte,
    pub viewangles: vec3_t,
    pub forwardmove: libc::c_float,
    pub sidemove: libc::c_float,
    pub upmove: libc::c_float,
    pub lightlevel: byte,
    pub buttons: libc::c_ushort,
    pub impulse: byte,
    pub weaponselect: byte,
    pub impact_index: libc::c_int,
    pub impact_position: vec3_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cl_local_data_t {
    pub predicted_origins: [vec3_t; 64],
    pub prediction_error: vec3_t,
    pub lastorigin: vec3_t,
    pub lastground: libc::c_int,
    pub interp_amount: libc::c_float,
    pub repredicting: qboolean,
    pub thirdperson: qboolean,
    pub apply_effects: qboolean,
    pub idealpitch: libc::c_float,
    pub viewmodel: libc::c_int,
    pub health: libc::c_int,
    pub onground: libc::c_int,
    pub light_level: libc::c_int,
    pub waterlevel: libc::c_int,
    pub usehull: libc::c_int,
    pub moving: libc::c_int,
    pub pushmsec: libc::c_int,
    pub weapons: libc::c_int,
    pub maxspeed: libc::c_float,
    pub scr_fov: libc::c_float,
    pub weaponsequence: libc::c_int,
    pub weaponstarttime: libc::c_float,
}
pub type event_state_t = event_state_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_state_s {
    pub ei: [event_info_t; 64],
}
pub type event_info_t = event_info_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_info_s {
    pub index: word,
    pub packet_index: libc::c_short,
    pub entity_index: libc::c_short,
    pub fire_time: libc::c_float,
    pub args: event_args_t,
    pub flags: libc::c_int,
}
pub type event_args_t = event_args_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_args_s {
    pub flags: libc::c_int,
    pub entindex: libc::c_int,
    pub origin: [libc::c_float; 3],
    pub angles: [libc::c_float; 3],
    pub velocity: [libc::c_float; 3],
    pub ducking: libc::c_int,
    pub fparam1: libc::c_float,
    pub fparam2: libc::c_float,
    pub iparam1: libc::c_int,
    pub iparam2: libc::c_int,
    pub bparam1: libc::c_int,
    pub bparam2: libc::c_int,
}
pub type runcmd_t = runcmd_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct runcmd_s {
    pub senttime: libc::c_double,
    pub receivedtime: libc::c_double,
    pub frame_lerp: libc::c_float,
    pub cmd: usercmd_t,
    pub processedfuncs: qboolean,
    pub heldback: qboolean,
    pub sendsize: libc::c_int,
}
pub type frame_t = frame_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct frame_s {
    pub receivedtime: libc::c_double,
    pub latency: libc::c_double,
    pub time: libc::c_double,
    pub valid: qboolean,
    pub choked: qboolean,
    pub clientdata: clientdata_t,
    pub playerstate: [entity_state_t; 32],
    pub weapondata: [weapon_data_t; 64],
    pub graphdata: netbandwidthgraph_t,
    pub flags: [byte; 256],
    pub num_entities: libc::c_int,
    pub first_entity: libc::c_int,
}
pub type netbandwidthgraph_t = netbandwithgraph_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct netbandwithgraph_s {
    pub client: word,
    pub players: word,
    pub entities: word,
    pub tentities: word,
    pub sound: word,
    pub event: word,
    pub usr: word,
    pub msgbytes: word,
    pub voicebytes: word,
}
#[inline(always)]
unsafe extern "C" fn __tg_ceil(mut __x: libc::c_double) -> libc::c_double {
    return ceil(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_floor(mut __x: libc::c_double) -> libc::c_double {
    return floor(__x);
}
#[no_mangle]
pub static mut g_curpaintbuffer: *mut portable_samplepair_t =
    0 as *const portable_samplepair_t as *mut portable_samplepair_t;
#[no_mangle]
pub static mut streambuffer: [portable_samplepair_t; 1025] =
    [portable_samplepair_t{left: 0, right: 0,}; 1025];
#[no_mangle]
pub static mut paintbuffer: [portable_samplepair_t; 1025] =
    [portable_samplepair_t{left: 0, right: 0,}; 1025];
#[no_mangle]
pub static mut roombuffer: [portable_samplepair_t; 1025] =
    [portable_samplepair_t{left: 0, right: 0,}; 1025];
#[no_mangle]
pub static mut facingbuffer: [portable_samplepair_t; 1025] =
    [portable_samplepair_t{left: 0, right: 0,}; 1025];
#[no_mangle]
pub static mut temppaintbuffer: [portable_samplepair_t; 1025] =
    [portable_samplepair_t{left: 0, right: 0,}; 1025];
#[no_mangle]
pub static mut paintbuffers: [paintbuffer_t; 3] =
    [paintbuffer_t{factive: false_0,
                   pbuf:
                       0 as *const portable_samplepair_t as
                           *mut portable_samplepair_t,
                   ifilter: 0,
                   fltmem:
                       [[portable_samplepair_t{left: 0, right: 0,}; 3]; 4],};
        3];
#[no_mangle]
pub static mut snd_scaletable: [[libc::c_int; 256]; 128] = [[0; 256]; 128];
#[no_mangle]
pub unsafe extern "C" fn S_InitScaletable() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (1 as libc::c_int) << 7 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 256 as libc::c_int {
            snd_scaletable[i as usize][j as usize] =
                j as libc::c_schar as libc::c_int * i *
                    ((1 as libc::c_int) <<
                         8 as libc::c_int - 7 as libc::c_int);
            j += 1
        }
        i += 1
    };
}
/*
===================
S_TransferPaintBuffer

===================
*/
#[no_mangle]
pub unsafe extern "C" fn S_TransferPaintBuffer(mut endtime: libc::c_int) {
    let mut snd_p: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut snd_linear_count: libc::c_int = 0;
    let mut lpos: libc::c_int = 0;
    let mut lpaintedtime: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    let mut sampleMask: libc::c_int = 0;
    let mut snd_out: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut pbuf: *mut dword = 0 as *mut dword;
    pbuf = dma.buffer as *mut dword;
    snd_p = g_curpaintbuffer as *mut libc::c_int;
    lpaintedtime = paintedtime;
    sampleMask = (dma.samples >> 1 as libc::c_int) - 1 as libc::c_int;
    while lpaintedtime < endtime {
        // handle recirculating buffer issues
        lpos = lpaintedtime & sampleMask;
        snd_out =
            (pbuf as
                 *mut libc::c_short).offset((lpos << 1 as libc::c_int) as
                                                isize);
        snd_linear_count = (dma.samples >> 1 as libc::c_int) - lpos;
        if lpaintedtime + snd_linear_count > endtime {
            snd_linear_count = endtime - lpaintedtime
        }
        snd_linear_count <<= 1 as libc::c_int;
        // write a linear blast of samples
        i = 0 as libc::c_int;
        while i < snd_linear_count {
            val =
                *snd_p.offset((i + 0 as libc::c_int) as isize) *
                    256 as libc::c_int >> 8 as libc::c_int;
            if val > 0x7fff as libc::c_int {
                *snd_out.offset((i + 0 as libc::c_int) as isize) =
                    0x7fff as libc::c_int as libc::c_short
            } else if val <
                          0x8000 as libc::c_int as libc::c_short as
                              libc::c_int {
                *snd_out.offset((i + 0 as libc::c_int) as isize) =
                    0x8000 as libc::c_int as libc::c_short
            } else {
                *snd_out.offset((i + 0 as libc::c_int) as isize) =
                    val as libc::c_short
            }
            val =
                *snd_p.offset((i + 1 as libc::c_int) as isize) *
                    256 as libc::c_int >> 8 as libc::c_int;
            if val > 0x7fff as libc::c_int {
                *snd_out.offset((i + 1 as libc::c_int) as isize) =
                    0x7fff as libc::c_int as libc::c_short
            } else if val <
                          0x8000 as libc::c_int as libc::c_short as
                              libc::c_int {
                *snd_out.offset((i + 1 as libc::c_int) as isize) =
                    0x8000 as libc::c_int as libc::c_short
            } else {
                *snd_out.offset((i + 1 as libc::c_int) as isize) =
                    val as libc::c_short
            }
            i += 2 as libc::c_int
        }
        snd_p = snd_p.offset(snd_linear_count as isize);
        lpaintedtime += snd_linear_count >> 1 as libc::c_int
    };
}
//===============================================================================
// Mix buffer (paintbuffer) management routines
//===============================================================================
// Activate a paintbuffer.  All active paintbuffers are mixed in parallel within
// MIX_MixChannelsToPaintbuffer, according to flags
#[inline]
unsafe extern "C" fn MIX_ActivatePaintbuffer(mut ipaintbuffer: libc::c_int) {
    paintbuffers[ipaintbuffer as usize].factive = true_0;
}
#[inline]
unsafe extern "C" fn MIX_SetCurrentPaintbuffer(mut ipaintbuffer:
                                                   libc::c_int) {
    g_curpaintbuffer = paintbuffers[ipaintbuffer as usize].pbuf;
}
#[inline]
unsafe extern "C" fn MIX_GetCurrentPaintbufferIndex() -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if g_curpaintbuffer == paintbuffers[i as usize].pbuf { return i }
        i += 1
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn MIX_GetCurrentPaintbufferPtr() -> *mut paintbuffer_t {
    let mut ipaint: libc::c_int = MIX_GetCurrentPaintbufferIndex();
    return &mut *paintbuffers.as_mut_ptr().offset(ipaint as isize) as
               *mut paintbuffer_t;
}
// Don't mix into any paintbuffers
#[inline]
unsafe extern "C" fn MIX_DeactivateAllPaintbuffers() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        paintbuffers[i as usize].factive = false_0;
        i += 1
    };
}
// set upsampling filter indexes back to 0
#[inline]
unsafe extern "C" fn MIX_ResetPaintbufferFilterCounters() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        paintbuffers[i as usize].ifilter = 0 as libc::c_int;
        i += 1
    };
}
// return pointer to front paintbuffer pbuf, given index
#[inline]
unsafe extern "C" fn MIX_GetPFrontFromIPaint(mut ipaintbuffer: libc::c_int)
 -> *mut portable_samplepair_t {
    return paintbuffers[ipaintbuffer as usize].pbuf;
}
#[inline]
unsafe extern "C" fn MIX_GetPPaintFromIPaint(mut ipaint: libc::c_int)
 -> *mut paintbuffer_t {
    return &mut *paintbuffers.as_mut_ptr().offset(ipaint as isize) as
               *mut paintbuffer_t;
}
#[no_mangle]
pub unsafe extern "C" fn MIX_FreeAllPaintbuffers() {
    // clear paintbuffer structs
    memset(paintbuffers.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           (3 as libc::c_int as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<paintbuffer_t>()
                                                as libc::c_ulong));
}
// Initialize paintbuffers array, set current paint buffer to main output buffer IPAINTBUFFER
#[no_mangle]
pub unsafe extern "C" fn MIX_InitAllPaintbuffers() {
    // clear paintbuffer structs
    memset(paintbuffers.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           (3 as libc::c_int as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<paintbuffer_t>()
                                                as libc::c_ulong));
    paintbuffers[0 as libc::c_int as usize].pbuf = paintbuffer.as_mut_ptr();
    paintbuffers[1 as libc::c_int as usize].pbuf = roombuffer.as_mut_ptr();
    paintbuffers[2 as libc::c_int as usize].pbuf = streambuffer.as_mut_ptr();
    MIX_SetCurrentPaintbuffer(0 as libc::c_int);
}
/*
===============================================================================

CHANNEL MIXING

===============================================================================
*/
#[no_mangle]
pub unsafe extern "C" fn S_PaintMonoFrom8(mut pbuf:
                                              *mut portable_samplepair_t,
                                          mut volume: *mut libc::c_int,
                                          mut pData: *mut byte,
                                          mut outCount: libc::c_int) {
    let mut lscale: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut rscale: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut data: libc::c_int = 0;
    lscale =
        snd_scaletable[(*volume.offset(0 as libc::c_int as isize) >>
                            8 as libc::c_int - 7 as libc::c_int) as
                           usize].as_mut_ptr();
    rscale =
        snd_scaletable[(*volume.offset(1 as libc::c_int as isize) >>
                            8 as libc::c_int - 7 as libc::c_int) as
                           usize].as_mut_ptr();
    i = 0 as libc::c_int;
    while i < outCount {
        data = *pData.offset(i as isize) as libc::c_int;
        (*pbuf.offset(i as isize)).left += *lscale.offset(data as isize);
        (*pbuf.offset(i as isize)).right += *rscale.offset(data as isize);
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn S_PaintStereoFrom8(mut pbuf:
                                                *mut portable_samplepair_t,
                                            mut volume: *mut libc::c_int,
                                            mut pData: *mut byte,
                                            mut outCount: libc::c_int) {
    let mut lscale: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut rscale: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut left: uint = 0;
    let mut right: uint = 0;
    let mut data: *mut word = 0 as *mut word;
    let mut i: libc::c_int = 0;
    lscale =
        snd_scaletable[(*volume.offset(0 as libc::c_int as isize) >>
                            8 as libc::c_int - 7 as libc::c_int) as
                           usize].as_mut_ptr();
    rscale =
        snd_scaletable[(*volume.offset(1 as libc::c_int as isize) >>
                            8 as libc::c_int - 7 as libc::c_int) as
                           usize].as_mut_ptr();
    data = pData as *mut word;
    i = 0 as libc::c_int;
    while i < outCount {
        left = (*data as libc::c_int & 0xff as libc::c_int) as byte as uint;
        right =
            ((*data as libc::c_int & 0xff00 as libc::c_int) >>
                 8 as libc::c_int) as byte as uint;
        (*pbuf.offset(i as isize)).left += *lscale.offset(left as isize);
        (*pbuf.offset(i as isize)).right += *rscale.offset(right as isize);
        i += 1;
        data = data.offset(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn S_PaintMonoFrom16(mut pbuf:
                                               *mut portable_samplepair_t,
                                           mut volume: *mut libc::c_int,
                                           mut pData: *mut libc::c_short,
                                           mut outCount: libc::c_int) {
    let mut left: libc::c_int = 0;
    let mut right: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut data: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < outCount {
        data = *pData.offset(i as isize) as libc::c_int;
        left =
            data * *volume.offset(0 as libc::c_int as isize) >>
                8 as libc::c_int;
        right =
            data * *volume.offset(1 as libc::c_int as isize) >>
                8 as libc::c_int;
        (*pbuf.offset(i as isize)).left += left;
        (*pbuf.offset(i as isize)).right += right;
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn S_PaintStereoFrom16(mut pbuf:
                                                 *mut portable_samplepair_t,
                                             mut volume: *mut libc::c_int,
                                             mut pData: *mut libc::c_short,
                                             mut outCount: libc::c_int) {
    let mut data: *mut uint = 0 as *mut uint;
    let mut left: libc::c_int = 0;
    let mut right: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    data = pData as *mut uint;
    i = 0 as libc::c_int;
    while i < outCount {
        left =
            (*data & 0xffff as libc::c_int as libc::c_uint) as libc::c_short
                as libc::c_int;
        right =
            ((*data & 0xffff0000 as libc::c_uint) >> 16 as libc::c_int) as
                libc::c_short as libc::c_int;
        left =
            left * *volume.offset(0 as libc::c_int as isize) >>
                8 as libc::c_int;
        right =
            right * *volume.offset(1 as libc::c_int as isize) >>
                8 as libc::c_int;
        (*pbuf.offset(i as isize)).left += left;
        (*pbuf.offset(i as isize)).right += right;
        i += 1;
        data = data.offset(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn S_Mix8MonoTimeCompress(mut pbuf:
                                                    *mut portable_samplepair_t,
                                                mut volume: *mut libc::c_int,
                                                mut pData: *mut byte,
                                                mut inputOffset: libc::c_int,
                                                mut rateScale: uint,
                                                mut outCount: libc::c_int,
                                                mut timecompress:
                                                    libc::c_int) {
}
#[no_mangle]
pub unsafe extern "C" fn S_Mix8Mono(mut pbuf: *mut portable_samplepair_t,
                                    mut volume: *mut libc::c_int,
                                    mut pData: *mut byte,
                                    mut inputOffset: libc::c_int,
                                    mut rateScale: uint,
                                    mut outCount: libc::c_int,
                                    mut timecompress: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut sampleIndex: libc::c_int = 0 as libc::c_int;
    let mut sampleFrac: uint = inputOffset as uint;
    let mut lscale: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut rscale: *mut libc::c_int = 0 as *mut libc::c_int;
    if timecompress != 0 as libc::c_int {
        S_Mix8MonoTimeCompress(pbuf, volume, pData, inputOffset, rateScale,
                               outCount, timecompress);
        //		return;
    }
    // Not using pitch shift?
    if rateScale == ((1 as libc::c_int) << 28 as libc::c_int) as libc::c_uint
       {
        S_PaintMonoFrom8(pbuf, volume, pData, outCount);
        return
    }
    lscale =
        snd_scaletable[(*volume.offset(0 as libc::c_int as isize) >>
                            8 as libc::c_int - 7 as libc::c_int) as
                           usize].as_mut_ptr();
    rscale =
        snd_scaletable[(*volume.offset(1 as libc::c_int as isize) >>
                            8 as libc::c_int - 7 as libc::c_int) as
                           usize].as_mut_ptr();
    i = 0 as libc::c_int;
    while i < outCount {
        (*pbuf.offset(i as isize)).left +=
            *lscale.offset(*pData.offset(sampleIndex as isize) as isize);
        (*pbuf.offset(i as isize)).right +=
            *rscale.offset(*pData.offset(sampleIndex as isize) as isize);
        sampleFrac =
            (sampleFrac as libc::c_uint).wrapping_add(rateScale) as uint as
                uint;
        sampleIndex += sampleFrac as libc::c_int >> 28 as libc::c_int;
        sampleFrac =
            sampleFrac &
                (((1 as libc::c_int) << 28 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_uint;
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn S_Mix8Stereo(mut pbuf: *mut portable_samplepair_t,
                                      mut volume: *mut libc::c_int,
                                      mut pData: *mut byte,
                                      mut inputOffset: libc::c_int,
                                      mut rateScale: uint,
                                      mut outCount: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut sampleIndex: libc::c_int = 0 as libc::c_int;
    let mut sampleFrac: uint = inputOffset as uint;
    let mut lscale: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut rscale: *mut libc::c_int = 0 as *mut libc::c_int;
    // Not using pitch shift?
    if rateScale == ((1 as libc::c_int) << 28 as libc::c_int) as libc::c_uint
       {
        S_PaintStereoFrom8(pbuf, volume, pData, outCount);
        return
    }
    lscale =
        snd_scaletable[(*volume.offset(0 as libc::c_int as isize) >>
                            8 as libc::c_int - 7 as libc::c_int) as
                           usize].as_mut_ptr();
    rscale =
        snd_scaletable[(*volume.offset(1 as libc::c_int as isize) >>
                            8 as libc::c_int - 7 as libc::c_int) as
                           usize].as_mut_ptr();
    i = 0 as libc::c_int;
    while i < outCount {
        (*pbuf.offset(i as isize)).left +=
            *lscale.offset(*pData.offset((sampleIndex + 0 as libc::c_int) as
                                             isize) as isize);
        (*pbuf.offset(i as isize)).right +=
            *rscale.offset(*pData.offset((sampleIndex + 1 as libc::c_int) as
                                             isize) as isize);
        sampleFrac =
            (sampleFrac as libc::c_uint).wrapping_add(rateScale) as uint as
                uint;
        sampleIndex +=
            (sampleFrac as libc::c_int >> 28 as libc::c_int) <<
                1 as libc::c_int;
        sampleFrac =
            sampleFrac &
                (((1 as libc::c_int) << 28 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_uint;
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn S_Mix16Mono(mut pbuf: *mut portable_samplepair_t,
                                     mut volume: *mut libc::c_int,
                                     mut pData: *mut libc::c_short,
                                     mut inputOffset: libc::c_int,
                                     mut rateScale: uint,
                                     mut outCount: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut sampleIndex: libc::c_int = 0 as libc::c_int;
    let mut sampleFrac: uint = inputOffset as uint;
    // Not using pitch shift?
    if rateScale == ((1 as libc::c_int) << 28 as libc::c_int) as libc::c_uint
       {
        S_PaintMonoFrom16(pbuf, volume, pData, outCount);
        return
    }
    i = 0 as libc::c_int;
    while i < outCount {
        (*pbuf.offset(i as isize)).left +=
            *volume.offset(0 as libc::c_int as isize) *
                *pData.offset(sampleIndex as isize) as libc::c_int >>
                8 as libc::c_int;
        (*pbuf.offset(i as isize)).right +=
            *volume.offset(1 as libc::c_int as isize) *
                *pData.offset(sampleIndex as isize) as libc::c_int >>
                8 as libc::c_int;
        sampleFrac =
            (sampleFrac as libc::c_uint).wrapping_add(rateScale) as uint as
                uint;
        sampleIndex += sampleFrac as libc::c_int >> 28 as libc::c_int;
        sampleFrac =
            sampleFrac &
                (((1 as libc::c_int) << 28 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_uint;
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn S_Mix16Stereo(mut pbuf: *mut portable_samplepair_t,
                                       mut volume: *mut libc::c_int,
                                       mut pData: *mut libc::c_short,
                                       mut inputOffset: libc::c_int,
                                       mut rateScale: uint,
                                       mut outCount: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut sampleIndex: libc::c_int = 0 as libc::c_int;
    let mut sampleFrac: uint = inputOffset as uint;
    // Not using pitch shift?
    if rateScale == ((1 as libc::c_int) << 28 as libc::c_int) as libc::c_uint
       {
        S_PaintStereoFrom16(pbuf, volume, pData, outCount);
        return
    }
    i = 0 as libc::c_int;
    while i < outCount {
        (*pbuf.offset(i as isize)).left +=
            *volume.offset(0 as libc::c_int as isize) *
                *pData.offset((sampleIndex + 0 as libc::c_int) as isize) as
                    libc::c_int >> 8 as libc::c_int;
        (*pbuf.offset(i as isize)).right +=
            *volume.offset(1 as libc::c_int as isize) *
                *pData.offset((sampleIndex + 1 as libc::c_int) as isize) as
                    libc::c_int >> 8 as libc::c_int;
        sampleFrac =
            (sampleFrac as libc::c_uint).wrapping_add(rateScale) as uint as
                uint;
        sampleIndex +=
            (sampleFrac as libc::c_int >> 28 as libc::c_int) <<
                1 as libc::c_int;
        sampleFrac =
            sampleFrac &
                (((1 as libc::c_int) << 28 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_uint;
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn S_MixChannel(mut pChannel: *mut channel_t,
                                      mut pData: *mut libc::c_void,
                                      mut outputOffset: libc::c_int,
                                      mut inputOffset: libc::c_int,
                                      mut fracRate: uint,
                                      mut outCount: libc::c_int,
                                      mut timecompress: libc::c_int) {
    let mut pvol: [libc::c_int; 2] = [0; 2];
    let mut ppaint: *mut paintbuffer_t = MIX_GetCurrentPaintbufferPtr();
    let mut pSource: *mut wavdata_t = (*(*pChannel).sfx).cache;
    let mut pbuf: *mut portable_samplepair_t =
        0 as *mut portable_samplepair_t;
    pvol[0 as libc::c_int as usize] =
        if (*pChannel).leftvol >= 0 as libc::c_int {
            if (*pChannel).leftvol < 255 as libc::c_int {
                (*pChannel).leftvol
            } else { 255 as libc::c_int }
        } else { 0 as libc::c_int };
    pvol[1 as libc::c_int as usize] =
        if (*pChannel).rightvol >= 0 as libc::c_int {
            if (*pChannel).rightvol < 255 as libc::c_int {
                (*pChannel).rightvol
            } else { 255 as libc::c_int }
        } else { 0 as libc::c_int };
    pbuf = (*ppaint).pbuf.offset(outputOffset as isize);
    if (*pSource).channels as libc::c_int == 1 as libc::c_int {
        if (*pSource).width as libc::c_int == 1 as libc::c_int {
            S_Mix8Mono(pbuf, pvol.as_mut_ptr(), pData as *mut byte,
                       inputOffset, fracRate, outCount, timecompress);
        } else {
            S_Mix16Mono(pbuf, pvol.as_mut_ptr(), pData as *mut libc::c_short,
                        inputOffset, fracRate, outCount);
        }
    } else if (*pSource).width as libc::c_int == 1 as libc::c_int {
        S_Mix8Stereo(pbuf, pvol.as_mut_ptr(), pData as *mut byte, inputOffset,
                     fracRate, outCount);
    } else {
        S_Mix16Stereo(pbuf, pvol.as_mut_ptr(), pData as *mut libc::c_short,
                      inputOffset, fracRate, outCount);
    };
}
#[no_mangle]
pub unsafe extern "C" fn S_MixDataToDevice(mut pChannel: *mut channel_t,
                                           mut sampleCount: libc::c_int,
                                           mut outRate: libc::c_int,
                                           mut outOffset: libc::c_int,
                                           mut timeCompress: libc::c_int)
 -> libc::c_int {
    // save this to compute total output
    let mut startingOffset: libc::c_int = outOffset;
    let mut inputRate: libc::c_float =
        (*pChannel).pitch *
            (*(*(*pChannel).sfx).cache).rate as libc::c_int as libc::c_float;
    let mut rate: libc::c_float = inputRate / outRate as libc::c_float;
    // shouldn't be playing this if finished, but return if we are
    if (*pChannel).pMixer.finished as u64 != 0 { return 0 as libc::c_int }
    // If we are terminating this wave prematurely, then make sure we detect the limit
    if (*pChannel).pMixer.forcedEndSample != 0. {
        // how many total input samples will we need?
        let mut samplesRequired: libc::c_int =
            (sampleCount as libc::c_float * rate) as libc::c_int;
        // will this hit the end?
        if (*pChannel).pMixer.sample + samplesRequired as libc::c_double >=
               (*pChannel).pMixer.forcedEndSample {
            // yes, mark finished and truncate the sample request
            (*pChannel).pMixer.finished = true_0;
            sampleCount =
                (((*pChannel).pMixer.forcedEndSample -
                      (*pChannel).pMixer.sample) / rate as libc::c_double) as
                    libc::c_int
        }
    }
    while sampleCount > 0 as libc::c_int {
        let mut availableSamples: libc::c_int = 0;
        let mut outSampleCount: libc::c_int = 0;
        let mut pSource: *mut wavdata_t = (*(*pChannel).sfx).cache;
        let mut use_loop: qboolean = (*pChannel).use_loop;
        let mut pData: *mut libc::c_void = 0 as *mut libc::c_void;
        let mut sampleFrac: libc::c_double = 0.;
        let mut i: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        // compute number of input samples required
        let mut end: libc::c_double =
            (*pChannel).pMixer.sample +
                (rate * sampleCount as libc::c_float) as libc::c_double;
        let mut inputSampleCount: libc::c_int =
            (__tg_ceil(end) - __tg_floor((*pChannel).pMixer.sample)) as
                libc::c_int;
        availableSamples =
            S_GetOutputData(pSource, &mut pData,
                            (*pChannel).pMixer.sample as libc::c_int,
                            inputSampleCount, use_loop);
        // none available, bail out
        if availableSamples == 0 { break ; }
        sampleFrac =
            (*pChannel).pMixer.sample - __tg_floor((*pChannel).pMixer.sample);
        if availableSamples < inputSampleCount {
            // how many samples are there given the number of input samples and the rate.
            outSampleCount =
                __tg_ceil((availableSamples as libc::c_double - sampleFrac) /
                              rate as libc::c_double) as libc::c_int
        } else { outSampleCount = sampleCount }
        // Verify that we won't get a buffer overrun.
        // save current paintbuffer
        j = MIX_GetCurrentPaintbufferIndex();
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            if !(paintbuffers[i as usize].factive as u64 == 0) {
                // mix chan into all active paintbuffers
                MIX_SetCurrentPaintbuffer(i);
                S_MixChannel(pChannel, pData, outOffset,
                             (sampleFrac *
                                  ((1 as libc::c_int) << 28 as libc::c_int) as
                                      libc::c_double) as libc::c_int,
                             (rate *
                                  ((1 as libc::c_int) << 28 as libc::c_int) as
                                      libc::c_float) as libc::c_int as uint,
                             outSampleCount, timeCompress);
            }
            i += 1
        }
        MIX_SetCurrentPaintbuffer(j);
        (*pChannel).pMixer.sample +=
            (outSampleCount as libc::c_float * rate) as libc::c_double;
        outOffset += outSampleCount;
        sampleCount -= outSampleCount
    }
    // Did we run out of samples? if so, mark finished
    if sampleCount > 0 as libc::c_int { (*pChannel).pMixer.finished = true_0 }
    // total number of samples mixed !!! at the output clock rate !!!
    return outOffset - startingOffset;
}
#[no_mangle]
pub unsafe extern "C" fn S_ShouldContinueMixing(mut ch: *mut channel_t)
 -> qboolean {
    if (*ch).isSentence as u64 != 0 {
        if !(*ch).currentWord.is_null() { return true_0 }
        return false_0
    }
    return ((*ch).pMixer.finished as u64 == 0) as libc::c_int as qboolean;
}
// Mix all channels into active paintbuffers until paintbuffer is full or 'endtime' is reached.
// endtime: time in 44khz samples to mix
// rate: ignore samples which are not natively at this rate (for multipass mixing/filtering)
// if rate == SOUND_ALL_RATES then mix all samples this pass
// flags: if SOUND_MIX_DRY, then mix only samples with channel flagged as 'dry'
// outputRate: target mix rate for all samples.  Note, if outputRate = SOUND_DMA_SPEED, then
// this routine will fill the paintbuffer to endtime.  Otherwise, fewer samples are mixed.
// if( endtime - paintedtime ) is not aligned on boundaries of 4,
// we'll miss data if outputRate < SOUND_DMA_SPEED!
#[no_mangle]
pub unsafe extern "C" fn MIX_MixChannelsToPaintbuffer(mut endtime:
                                                          libc::c_int,
                                                      mut rate: libc::c_int,
                                                      mut outputRate:
                                                          libc::c_int) {
    let mut ch: *mut channel_t = 0 as *mut channel_t;
    let mut pSource: *mut wavdata_t = 0 as *mut wavdata_t;
    let mut i: libc::c_int = 0;
    let mut sampleCount: libc::c_int = 0;
    let mut bZeroVolume: qboolean = false_0;
    // mix each channel into paintbuffer
    ch = channels.as_mut_ptr();
    // validate parameters
    // make sure we're not discarding data
    // 44k: try to mix this many samples at outputRate
    sampleCount =
        (endtime - paintedtime) / (44100 as libc::c_int / outputRate);
    if sampleCount <= 0 as libc::c_int { return }
    let mut current_block_28: u64;
    i = 0 as libc::c_int;
    while i < total_channels {
        if !(*ch).sfx.is_null() {
            // NOTE: background map is allow both type sounds: menu and game
            if cl.background as u64 == 0 {
                if cls.key_dest as libc::c_uint ==
                       key_console as libc::c_int as libc::c_uint &&
                       (*ch).localsound as libc::c_uint != 0 {
                    current_block_28 =
                        5948590327928692120; // silent mode in console
                } else if (s_listener.inmenu as libc::c_uint != 0 ||
                               s_listener.paused as libc::c_uint != 0) &&
                              (*ch).localsound as u64 == 0 {
                    // play only local sounds, keep pause for other
                    current_block_28 = 10886091980245723256;
                } else if s_listener.inmenu as u64 == 0 &&
                              s_listener.active as u64 == 0 &&
                              (*ch).staticsound as u64 == 0 {
                    current_block_28 = 10886091980245723256;
                } else { current_block_28 = 5948590327928692120; }
            } else if cls.key_dest as libc::c_uint ==
                          key_console as libc::c_int as libc::c_uint {
                current_block_28 = 10886091980245723256;
            } else { current_block_28 = 5948590327928692120; }
            match current_block_28 {
                10886091980245723256 => { }
                _ =>
                // play, playvol
                {
                    pSource = S_LoadSound((*ch).sfx);
                    // Don't mix sound data for sounds with zero volume. If it's a non-looping sound,
		// just remove the sound when its volume goes to zero.
                    bZeroVolume =
                        ((*ch).leftvol == 0 && (*ch).rightvol == 0) as
                            libc::c_int as qboolean;
                    if bZeroVolume as u64 == 0 {
                        // this values matched with GoldSrc
                        if (*ch).leftvol < 8 as libc::c_int &&
                               (*ch).rightvol < 8 as libc::c_int {
                            bZeroVolume = true_0
                        }
                    }
                    if pSource.is_null() ||
                           bZeroVolume as libc::c_uint != 0 &&
                               (*pSource).loopStart == -(1 as libc::c_int) {
                        if pSource.is_null() {
                            S_FreeChannel(ch);
                            current_block_28 = 10886091980245723256;
                        } else { current_block_28 = 5689316957504528238; }
                    } else if bZeroVolume as u64 != 0 {
                        current_block_28 = 10886091980245723256;
                    } else { current_block_28 = 5689316957504528238; }
                    match current_block_28 {
                        10886091980245723256 => { }
                        _ =>
                        // multipass mixing - only mix samples of specified sample rate
                        {
                            match rate {
                                11025 | 22050 | 44100 => {
                                    if rate != (*pSource).rate as libc::c_int
                                       {
                                        current_block_28 =
                                            10886091980245723256;
                                    } else {
                                        current_block_28 =
                                            11385396242402735691;
                                    }
                                }
                                _ => {
                                    current_block_28 = 11385396242402735691;
                                }
                            }
                            match current_block_28 {
                                10886091980245723256 => { }
                                _ => {
                                    // get playback pitch
                                    if (*ch).isSentence as u64 != 0 {
                                        (*ch).pitch =
                                            VOX_ModifyPitch(ch,
                                                            (*ch).basePitch as
                                                                libc::c_float
                                                                * 0.01f32)
                                    } else {
                                        (*ch).pitch =
                                            (*ch).basePitch as libc::c_float *
                                                0.01f32
                                    }
                                    if !CL_GetEntityByIndex((*ch).entnum).is_null()
                                           &&
                                           (*ch).entchannel ==
                                               2 as libc::c_int {
                                        if (*pSource).width as libc::c_int ==
                                               1 as libc::c_int {
                                            SND_MoveMouth8(ch, pSource,
                                                           sampleCount);
                                        } else {
                                            SND_MoveMouth16(ch, pSource,
                                                            sampleCount);
                                        }
                                    }
                                    // mix channel to all active paintbuffers.
		// NOTE: must be called once per channel only - consecutive calls retrieve additional data.
                                    if (*ch).isSentence as u64 != 0 {
                                        VOX_MixDataToDevice(ch, sampleCount,
                                                            outputRate,
                                                            0 as libc::c_int);
                                    } else {
                                        S_MixDataToDevice(ch, sampleCount,
                                                          outputRate,
                                                          0 as libc::c_int,
                                                          0 as libc::c_int);
                                    }
                                    if S_ShouldContinueMixing(ch) as u64 == 0
                                       {
                                        S_FreeChannel(ch);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        // play only ambient sounds, keep pause for other
        i += 1;
        ch = ch.offset(1)
    };
}
// pass in index -1...count+2, return pointer to source sample in either paintbuffer or delay buffer
#[inline]
unsafe extern "C" fn S_GetNextpFilter(mut i: libc::c_int,
                                      mut pbuffer: *mut portable_samplepair_t,
                                      mut pfiltermem:
                                          *mut portable_samplepair_t)
 -> *mut portable_samplepair_t {
    // The delay buffer is assumed to precede the paintbuffer by 6 duplicated samples
    if i == -(1 as libc::c_int) {
        return &mut *pfiltermem.offset(0 as libc::c_int as isize) as
                   *mut portable_samplepair_t
    }
    if i == 0 as libc::c_int {
        return &mut *pfiltermem.offset(1 as libc::c_int as isize) as
                   *mut portable_samplepair_t
    }
    if i == 1 as libc::c_int {
        return &mut *pfiltermem.offset(2 as libc::c_int as isize) as
                   *mut portable_samplepair_t
    }
    // return from paintbuffer, where samples are doubled.
	// even samples are to be replaced with interpolated value.
    return &mut *pbuffer.offset(((i - 2 as libc::c_int) * 2 as libc::c_int +
                                     1 as libc::c_int) as isize) as
               *mut portable_samplepair_t;
}
// pass forward over passed in buffer and cubic interpolate all odd samples
// pbuffer: buffer to filter (in place)
// prevfilter:  filter memory. NOTE: this must match the filtertype ie: filtercubic[] for FILTERTYPE_CUBIC
// if NULL then perform no filtering.
// count: how many samples to upsample. will become count*2 samples in buffer, in place.
#[no_mangle]
pub unsafe extern "C" fn S_Interpolate2xCubic(mut pbuffer:
                                                  *mut portable_samplepair_t,
                                              mut pfiltermem:
                                                  *mut portable_samplepair_t,
                                              mut cfltmem: libc::c_int,
                                              mut count: libc::c_int) {
    // implement cubic interpolation on 2x upsampled buffer.   Effectively delays buffer contents by 2 samples.
// pbuffer: contains samples at 0, 2, 4, 6...
// temppaintbuffer is temp buffer, same size as paintbuffer, used to store processed values
// count: number of samples to process in buffer ie: how many samples at 0, 2, 4, 6...
    // finpos is the fractional, inpos the integer part.
//		finpos = 0.5 for upsampling by 2x
//		inpos is the position of the sample
    //		xm1 = x [inpos - 1];
//		x0 = x [inpos + 0];
//		x1 = x [inpos + 1];
//		x2 = x [inpos + 2];
//		a = (3 * (x0-x1) - xm1 + x2) / 2;
//		b = 2*x1 + xm1 - (5*x0 + x2) / 2;
//		c = (x1 - xm1) / 2;
//		y [outpos] = (((a * finpos) + b) * finpos + c) * finpos + x0;
    let mut i: libc::c_int = 0;
    let mut upCount: libc::c_int = count << 1 as libc::c_int;
    let mut a: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut xm1: libc::c_int = 0;
    let mut x0: libc::c_int = 0;
    let mut x1: libc::c_int = 0;
    let mut x2: libc::c_int = 0;
    let mut psamp0: *mut portable_samplepair_t =
        0 as *mut portable_samplepair_t;
    let mut psamp1: *mut portable_samplepair_t =
        0 as *mut portable_samplepair_t;
    let mut psamp2: *mut portable_samplepair_t =
        0 as *mut portable_samplepair_t;
    let mut psamp3: *mut portable_samplepair_t =
        0 as *mut portable_samplepair_t;
    let mut outpos: libc::c_int = 0 as libc::c_int;
    // pfiltermem holds 6 samples from previous buffer pass
	// process 'count' samples
    i = 0 as libc::c_int;
    while i < count {
        // get source sample pointer
        psamp0 = S_GetNextpFilter(i - 1 as libc::c_int, pbuffer, pfiltermem);
        psamp1 = S_GetNextpFilter(i + 0 as libc::c_int, pbuffer, pfiltermem);
        psamp2 = S_GetNextpFilter(i + 1 as libc::c_int, pbuffer, pfiltermem);
        psamp3 = S_GetNextpFilter(i + 2 as libc::c_int, pbuffer, pfiltermem);
        // write out original sample to interpolation buffer
        let fresh0 = outpos;
        outpos = outpos + 1;
        temppaintbuffer[fresh0 as usize] = *psamp1;
        // get all left samples for interpolation window
        xm1 = (*psamp0).left;
        x0 = (*psamp1).left;
        x1 = (*psamp2).left;
        x2 = (*psamp3).left;
        // interpolate
        a = (3 as libc::c_int * (x0 - x1) - xm1 + x2) / 2 as libc::c_int;
        b =
            2 as libc::c_int * x1 + xm1 -
                (5 as libc::c_int * x0 + x2) / 2 as libc::c_int;
        c = (x1 - xm1) / 2 as libc::c_int;
        // write out interpolated sample
        temppaintbuffer[outpos as usize].left =
            a / 8 as libc::c_int + b / 4 as libc::c_int + c / 2 as libc::c_int
                + x0;
        // get all right samples for window
        xm1 = (*psamp0).right;
        x0 = (*psamp1).right;
        x1 = (*psamp2).right;
        x2 = (*psamp3).right;
        // interpolate
        a = (3 as libc::c_int * (x0 - x1) - xm1 + x2) / 2 as libc::c_int;
        b =
            2 as libc::c_int * x1 + xm1 -
                (5 as libc::c_int * x0 + x2) / 2 as libc::c_int;
        c = (x1 - xm1) / 2 as libc::c_int;
        // write out interpolated sample, increment output counter
        let fresh1 = outpos;
        outpos = outpos + 1;
        temppaintbuffer[fresh1 as usize].right =
            a / 8 as libc::c_int + b / 4 as libc::c_int + c / 2 as libc::c_int
                + x0;
        i += 1
    }
    // save last 3 samples from paintbuffer
    *pfiltermem.offset(0 as libc::c_int as isize) =
        *pbuffer.offset((upCount - 5 as libc::c_int) as isize);
    *pfiltermem.offset(1 as libc::c_int as isize) =
        *pbuffer.offset((upCount - 3 as libc::c_int) as isize);
    *pfiltermem.offset(2 as libc::c_int as isize) =
        *pbuffer.offset((upCount - 1 as libc::c_int) as isize);
    // copy temppaintbuffer back into paintbuffer
    i = 0 as libc::c_int;
    while i < upCount {
        *pbuffer.offset(i as isize) = temppaintbuffer[i as usize];
        i += 1
    };
}
// pass forward over passed in buffer and linearly interpolate all odd samples
// pbuffer: buffer to filter (in place)
// prevfilter:  filter memory. NOTE: this must match the filtertype ie: filterlinear[] for FILTERTYPE_LINEAR
// if NULL then perform no filtering.
// count: how many samples to upsample. will become count*2 samples in buffer, in place.
#[no_mangle]
pub unsafe extern "C" fn S_Interpolate2xLinear(mut pbuffer:
                                                   *mut portable_samplepair_t,
                                               mut pfiltermem:
                                                   *mut portable_samplepair_t,
                                               mut cfltmem: libc::c_int,
                                               mut count: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut upCount: libc::c_int = count << 1 as libc::c_int;
    // use interpolation value from previous mix
    (*pbuffer.offset(0 as libc::c_int as isize)).left =
        (*pfiltermem).left + (*pbuffer.offset(0 as libc::c_int as isize)).left
            >> 1 as libc::c_int;
    (*pbuffer.offset(0 as libc::c_int as isize)).right =
        (*pfiltermem).right +
            (*pbuffer.offset(0 as libc::c_int as isize)).right >>
            1 as libc::c_int;
    i = 2 as libc::c_int;
    while i < upCount {
        // use linear interpolation for upsampling
        (*pbuffer.offset(i as isize)).left =
            (*pbuffer.offset(i as isize)).left +
                (*pbuffer.offset((i - 1 as libc::c_int) as isize)).left >>
                1 as libc::c_int;
        (*pbuffer.offset(i as isize)).right =
            (*pbuffer.offset(i as isize)).right +
                (*pbuffer.offset((i - 1 as libc::c_int) as isize)).right >>
                1 as libc::c_int;
        i += 2 as libc::c_int
    }
    // save last value to be played out in buffer
    *pfiltermem = *pbuffer.offset((upCount - 1 as libc::c_int) as isize);
}
// upsample by 2x, optionally using interpolation
// count: how many samples to upsample. will become count*2 samples in buffer, in place.
// pbuffer: buffer to upsample into (in place)
// pfiltermem:  filter memory. NOTE: this must match the filtertype ie: filterlinear[] for FILTERTYPE_LINEAR
// if NULL then perform no filtering.
// cfltmem: max number of sample pairs filter can use
// filtertype: FILTERTYPE_NONE, _LINEAR, _CUBIC etc.  Must match prevfilter.
#[no_mangle]
pub unsafe extern "C" fn S_MixBufferUpsample2x(mut count: libc::c_int,
                                               mut pbuffer:
                                                   *mut portable_samplepair_t,
                                               mut pfiltermem:
                                                   *mut portable_samplepair_t,
                                               mut cfltmem: libc::c_int,
                                               mut filtertype: libc::c_int) {
    let mut upCount: libc::c_int = count << 1 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    // reverse through buffer, duplicating contents for 'count' samples
    i = upCount - 1 as libc::c_int;
    j = count - 1 as libc::c_int;
    while j >= 0 as libc::c_int {
        *pbuffer.offset(i as isize) = *pbuffer.offset(j as isize);
        *pbuffer.offset((i - 1 as libc::c_int) as isize) =
            *pbuffer.offset(j as isize);
        i -= 2 as libc::c_int;
        j -= 1
    }
    // pass forward through buffer, interpolate all even slots
    match filtertype {
        1 => { S_Interpolate2xLinear(pbuffer, pfiltermem, cfltmem, count); }
        2 => { S_Interpolate2xCubic(pbuffer, pfiltermem, cfltmem, count); }
        _ => { }
    };
}
// zero out all paintbuffers
#[no_mangle]
pub unsafe extern "C" fn MIX_ClearAllPaintBuffers(mut SampleCount:
                                                      libc::c_int,
                                                  mut clearFilters:
                                                      qboolean) {
    let mut count: libc::c_int =
        if SampleCount < 1024 as libc::c_int {
            SampleCount
        } else { 1024 as libc::c_int };
    let mut i: libc::c_int = 0;
    // zero out all paintbuffer data (ignore sampleCount)
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if !paintbuffers[i as usize].pbuf.is_null() {
            memset(paintbuffers[i as usize].pbuf as *mut libc::c_void,
                   0 as libc::c_int,
                   ((count + 1 as libc::c_int) as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<portable_samplepair_t>()
                                                        as libc::c_ulong));
        }
        if clearFilters as u64 != 0 {
            memset(paintbuffers[i as usize].fltmem.as_mut_ptr() as
                       *mut libc::c_void, 0 as libc::c_int,
                   ::std::mem::size_of::<[[portable_samplepair_t; 3]; 4]>() as
                       libc::c_ulong);
        }
        i += 1
    }
    if clearFilters as u64 != 0 { MIX_ResetPaintbufferFilterCounters(); };
}
// mixes pbuf1 + pbuf2 into pbuf3, count samples
// fgain is output gain 0-1.0
// NOTE: pbuf3 may equal pbuf1 or pbuf2!
#[no_mangle]
pub unsafe extern "C" fn MIX_MixPaintbuffers(mut ibuf1: libc::c_int,
                                             mut ibuf2: libc::c_int,
                                             mut ibuf3: libc::c_int,
                                             mut count: libc::c_int,
                                             mut fgain: libc::c_float) {
    let mut pbuf1: *mut portable_samplepair_t =
        0 as *mut portable_samplepair_t;
    let mut pbuf2: *mut portable_samplepair_t =
        0 as *mut portable_samplepair_t;
    let mut pbuf3: *mut portable_samplepair_t =
        0 as *mut portable_samplepair_t;
    let mut i: libc::c_int = 0;
    let mut gain: libc::c_int = 0;
    gain = (256 as libc::c_int as libc::c_float * fgain) as libc::c_int;
    pbuf1 = paintbuffers[ibuf1 as usize].pbuf;
    pbuf2 = paintbuffers[ibuf2 as usize].pbuf;
    pbuf3 = paintbuffers[ibuf3 as usize].pbuf;
    // destination buffer stereo - average n chans down to stereo
    // destination 2ch:
	// pb1 2ch + pb2 2ch		-> pb3 2ch
	// pb1 2ch + pb2 (4ch->2ch)		-> pb3 2ch
	// pb1 (4ch->2ch) + pb2 (4ch->2ch)	-> pb3 2ch
    // mix front channels
    i = 0 as libc::c_int;
    while i < count {
        (*pbuf3.offset(i as isize)).left = (*pbuf1.offset(i as isize)).left;
        (*pbuf3.offset(i as isize)).right = (*pbuf1.offset(i as isize)).right;
        (*pbuf3.offset(i as isize)).left +=
            (*pbuf2.offset(i as isize)).left * gain >> 8 as libc::c_int;
        (*pbuf3.offset(i as isize)).right +=
            (*pbuf2.offset(i as isize)).right * gain >> 8 as libc::c_int;
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn MIX_CompressPaintbuffer(mut ipaint: libc::c_int,
                                                 mut count: libc::c_int) {
    let mut pbuf: *mut portable_samplepair_t =
        0 as *mut portable_samplepair_t;
    let mut ppaint: *mut paintbuffer_t = 0 as *mut paintbuffer_t;
    let mut i: libc::c_int = 0;
    ppaint = MIX_GetPPaintFromIPaint(ipaint);
    pbuf = (*ppaint).pbuf;
    i = 0 as libc::c_int;
    while i < count {
        (*pbuf).left =
            if (*pbuf).left > 32760 as libc::c_int {
                32760 as libc::c_int
            } else if (*pbuf).left < -(32760 as libc::c_int) {
                -(32760 as libc::c_int)
            } else { (*pbuf).left };
        (*pbuf).right =
            if (*pbuf).right > 32760 as libc::c_int {
                32760 as libc::c_int
            } else if (*pbuf).right < -(32760 as libc::c_int) {
                -(32760 as libc::c_int)
            } else { (*pbuf).right };
        i += 1;
        pbuf = pbuf.offset(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn S_MixUpsample(mut sampleCount: libc::c_int,
                                       mut filtertype: libc::c_int) {
    let mut ppaint: *mut paintbuffer_t = MIX_GetCurrentPaintbufferPtr();
    let mut ifilter: libc::c_int = (*ppaint).ifilter;
    S_MixBufferUpsample2x(sampleCount, (*ppaint).pbuf,
                          &mut *(*(*ppaint).fltmem.as_mut_ptr().offset(ifilter
                                                                           as
                                                                           isize)).as_mut_ptr().offset(0
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           isize),
                          3 as libc::c_int, filtertype);
    // make sure on next upsample pass for this paintbuffer, new filter memory is used
    (*ppaint).ifilter += 1;
}
#[no_mangle]
pub unsafe extern "C" fn MIX_MixStreamBuffer(mut end: libc::c_int) {
    let mut pbuf: *mut portable_samplepair_t =
        0 as *mut portable_samplepair_t;
    let mut ch: *mut rawchan_t = 0 as *mut rawchan_t;
    pbuf = MIX_GetPFrontFromIPaint(2 as libc::c_int);
    ch = S_FindRawChannel(-(2 as libc::c_int), false_0);
    // clear the paint buffer
    if s_listener.paused as libc::c_uint != 0 || ch.is_null() ||
           (*ch).s_rawend < paintedtime as libc::c_uint {
        memset(pbuf as *mut libc::c_void, 0 as libc::c_int,
               ((end - paintedtime) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<portable_samplepair_t>()
                                                    as libc::c_ulong));
    } else {
        let mut i: libc::c_int = 0;
        let mut stop: libc::c_int = 0;
        // copy from the streaming sound source
        stop =
            if (end as libc::c_uint) < (*ch).s_rawend {
                end as libc::c_uint
            } else { (*ch).s_rawend } as libc::c_int;
        i = paintedtime;
        while i < stop {
            (*pbuf.offset((i - paintedtime) as isize)).left =
                (*(*ch).rawsamples.as_mut_ptr().offset((i as libc::c_ulong &
                                                            (*ch).max_samples.wrapping_sub(1
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               libc::c_ulong))
                                                           as isize)).left *
                    (*ch).leftvol >> 8 as libc::c_int;
            (*pbuf.offset((i - paintedtime) as isize)).right =
                (*(*ch).rawsamples.as_mut_ptr().offset((i as libc::c_ulong &
                                                            (*ch).max_samples.wrapping_sub(1
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               libc::c_ulong))
                                                           as isize)).right *
                    (*ch).rightvol >> 8 as libc::c_int;
            i += 1
        }
        while i < end {
            let ref mut fresh2 =
                (*pbuf.offset((i - paintedtime) as isize)).right;
            *fresh2 = 0 as libc::c_int;
            (*pbuf.offset((i - paintedtime) as isize)).left = *fresh2;
            i += 1
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn MIX_MixRawSamplesBuffer(mut end: libc::c_int) {
    let mut pbuf: *mut portable_samplepair_t =
        0 as *mut portable_samplepair_t;
    let mut i: uint = 0;
    let mut j: uint = 0;
    let mut stop: uint = 0;
    pbuf = (*MIX_GetCurrentPaintbufferPtr()).pbuf;
    if s_listener.paused as u64 != 0 { return }
    // paint in the raw channels
    i = 0 as libc::c_int as uint;
    while i < 16 as libc::c_int as libc::c_uint {
        // copy from the streaming sound source
        let mut ch: *mut rawchan_t = raw_channels[i as usize];
        // background track should be mixing into another buffer
        if !(ch.is_null() || (*ch).entnum == -(2 as libc::c_int)) {
            // not audible
            if !((*ch).leftvol == 0 && (*ch).rightvol == 0) {
                stop =
                    if (end as libc::c_uint) < (*ch).s_rawend {
                        end as libc::c_uint
                    } else { (*ch).s_rawend };
                j = paintedtime as uint;
                while j < stop {
                    (*pbuf.offset(j.wrapping_sub(paintedtime as libc::c_uint)
                                      as isize)).left +=
                        (*(*ch).rawsamples.as_mut_ptr().offset((j as
                                                                    libc::c_ulong
                                                                    &
                                                                    (*ch).max_samples.wrapping_sub(1
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                                                   as
                                                                   isize)).left
                            * (*ch).leftvol >> 8 as libc::c_int;
                    (*pbuf.offset(j.wrapping_sub(paintedtime as libc::c_uint)
                                      as isize)).right +=
                        (*(*ch).rawsamples.as_mut_ptr().offset((j as
                                                                    libc::c_ulong
                                                                    &
                                                                    (*ch).max_samples.wrapping_sub(1
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                                                   as
                                                                   isize)).right
                            * (*ch).rightvol >> 8 as libc::c_int;
                    j = j.wrapping_add(1)
                }
            }
        }
        i = i.wrapping_add(1)
    };
}
// upsample and mix sounds into final 44khz versions of:
// IROOMBUFFER, IFACINGBUFFER, IFACINGAWAY
// dsp fx are then applied to these buffers by the caller.
// caller also remixes all into final IPAINTBUFFER output.
#[no_mangle]
pub unsafe extern "C" fn MIX_UpsampleAllPaintbuffers(mut end: libc::c_int,
                                                     mut count: libc::c_int) {
    // process stream buffer
    MIX_MixStreamBuffer(end);
    // 11khz sounds are mixed into 3 buffers based on distance from listener, and facing direction
	// These buffers are facing, facingaway, room
	// These 3 mixed buffers are then each upsampled to 22khz.
    // 22khz sounds are mixed into the 3 buffers based on distance from listener, and facing direction
	// These 3 mixed buffers are then each upsampled to 44khz.
    // 44khz sounds are mixed into the 3 buffers based on distance from listener, and facing direction
    MIX_DeactivateAllPaintbuffers();
    // set paintbuffer upsample filter indices to 0
    MIX_ResetPaintbufferFilterCounters();
    // only mix to roombuffer if dsp fx are on KDB: perf
    MIX_ActivatePaintbuffer(1 as
                                libc::c_int); // operates on MIX_MixChannelsToPaintbuffer
    // mix 11khz sounds:
    MIX_MixChannelsToPaintbuffer(end, 11025 as libc::c_int,
                                 11025 as libc::c_int);
    // upsample all 11khz buffers by 2x
	// only upsample roombuffer if dsp fx are on KDB: perf
    MIX_SetCurrentPaintbuffer(1 as libc::c_int); // operates on MixUpSample
    S_MixUpsample(count / (44100 as libc::c_int / 11025 as libc::c_int),
                  s_lerping.value as libc::c_int);
    // mix 22khz sounds:
    MIX_MixChannelsToPaintbuffer(end, 22050 as libc::c_int,
                                 22050 as libc::c_int);
    // upsample all 22khz buffers by 2x
	// only upsample roombuffer if dsp fx are on KDB: perf
    MIX_SetCurrentPaintbuffer(1 as libc::c_int);
    S_MixUpsample(count / (44100 as libc::c_int / 22050 as libc::c_int),
                  s_lerping.value as libc::c_int);
    // mix all 44khz sounds to all active paintbuffers
    MIX_MixChannelsToPaintbuffer(end, 44100 as libc::c_int,
                                 44100 as libc::c_int);
    // mix raw samples from the video streams
    MIX_SetCurrentPaintbuffer(1 as libc::c_int);
    MIX_MixRawSamplesBuffer(end);
    MIX_DeactivateAllPaintbuffers();
    MIX_SetCurrentPaintbuffer(0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn MIX_PaintChannels(mut endtime: libc::c_int) {
    let mut end: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    CheckNewDspPresets();
    while paintedtime < endtime {
        // if paintbuffer is smaller than DMA buffer
        end = endtime;
        if endtime - paintedtime > 1024 as libc::c_int {
            end = paintedtime + 1024 as libc::c_int
        }
        // number of 44khz samples to mix into paintbuffer, up to paintbuffer size
        count = end - paintedtime;
        // clear the all mix buffers
        MIX_ClearAllPaintBuffers(count, false_0);
        MIX_UpsampleAllPaintbuffers(end, count);
        // process all sounds with DSP
        DSP_Process(idsp_room, MIX_GetPFrontFromIPaint(1 as libc::c_int),
                    count);
        // add music or soundtrack from movie (no dsp)
        MIX_MixPaintbuffers(0 as libc::c_int, 1 as libc::c_int,
                            0 as libc::c_int, count, S_GetMasterVolume());
        // add music or soundtrack from movie (no dsp)
        MIX_MixPaintbuffers(0 as libc::c_int, 2 as libc::c_int,
                            0 as libc::c_int, count, S_GetMusicVolume());
        // clip all values > 16 bit down to 16 bit
        MIX_CompressPaintbuffer(0 as libc::c_int, count);
        // transfer IPAINTBUFFER paintbuffer out to DMA buffer
        MIX_SetCurrentPaintbuffer(0 as libc::c_int);
        // transfer out according to DMA format
        S_TransferPaintBuffer(end);
        paintedtime = end
    };
}
