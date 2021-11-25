#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type file_s;
    pub type stream_s;
    pub type grasshdr_s;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn SCR_GetMovieInfo() -> *mut wavdata_t;
    #[no_mangle]
    fn SCR_GetAudioChunk(rawdata: *mut libc::c_char, length: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn FS_FreeStream(stream: *mut stream_t);
    #[no_mangle]
    fn FS_GetStreamPos(stream: *mut stream_t) -> libc::c_int;
    #[no_mangle]
    fn FS_SetStreamPos(stream: *mut stream_t, newpos: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn FS_ReadStream(stream: *mut stream_t, bytes: libc::c_int,
                     buffer: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn FS_StreamInfo(stream: *mut stream_t) -> *mut wavdata_t;
    #[no_mangle]
    fn FS_OpenStream(filename: *const libc::c_char) -> *mut stream_t;
    #[no_mangle]
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    static mut s_listener: listener_t;
    #[no_mangle]
    static mut dma: dma_t;
    #[no_mangle]
    fn S_RawSamples(samples: uint, rate: uint, width: word, channels: word,
                    data: *const byte, entnum: libc::c_int);
    #[no_mangle]
    fn S_FindRawChannel(entnum: libc::c_int, create: qboolean)
     -> *mut rawchan_t;
    #[no_mangle]
    static mut soundtime: libc::c_int;
    #[no_mangle]
    static mut s_musicvolume: convar_t;
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
pub type string = [libc::c_char; 256];
pub type file_t = file_s;
pub type stream_t = stream_s;
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
pub struct bg_track_t {
    pub current: string,
    pub loopName: string,
    pub stream: *mut stream_t,
    pub source: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct musicfade_t {
    pub percent: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dma_t {
    pub format: snd_format_t,
    pub samples: libc::c_int,
    pub samplepos: libc::c_int,
    pub buffer: *mut byte,
    pub initialized: qboolean,
}
pub type snd_format_t = snd_format_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct snd_format_s {
    pub speed: libc::c_uint,
    pub width: libc::c_uint,
    pub channels: libc::c_uint,
}
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
pub struct portable_samplepair_t {
    pub left: libc::c_int,
    pub right: libc::c_int,
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
/*
s_stream.c - sound streaming
Copyright (C) 2009 Uncle Mike

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
*/
static mut s_bgTrack: bg_track_t =
    bg_track_t{current: [0; 256],
               loopName: [0; 256],
               stream: 0 as *const stream_t as *mut stream_t,
               source: 0,};
static mut musicfade: musicfade_t = musicfade_t{percent: 0.,};
// controlled by game dlls
/*
=================
S_PrintBackgroundTrackState
=================
*/
#[no_mangle]
pub unsafe extern "C" fn S_PrintBackgroundTrackState() {
    Con_Printf(b"BackgroundTrack: \x00" as *const u8 as *const libc::c_char);
    if s_bgTrack.current[0 as libc::c_int as usize] as libc::c_int != 0 &&
           s_bgTrack.loopName[0 as libc::c_int as usize] as libc::c_int != 0 {
        Con_Printf(b"intro %s, loop %s\n\x00" as *const u8 as
                       *const libc::c_char, s_bgTrack.current.as_mut_ptr(),
                   s_bgTrack.loopName.as_mut_ptr());
    } else if s_bgTrack.current[0 as libc::c_int as usize] != 0 {
        Con_Printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
                   s_bgTrack.current.as_mut_ptr());
    } else if s_bgTrack.loopName[0 as libc::c_int as usize] != 0 {
        Con_Printf(b"%s [loop]\n\x00" as *const u8 as *const libc::c_char,
                   s_bgTrack.loopName.as_mut_ptr());
    } else {
        Con_Printf(b"not playing\n\x00" as *const u8 as *const libc::c_char);
    };
}
/*
=================
S_FadeMusicVolume
=================
*/
#[no_mangle]
pub unsafe extern "C" fn S_FadeMusicVolume(mut fadePercent: libc::c_float) {
    musicfade.percent =
        if fadePercent >= 0.0f32 {
            if fadePercent < 100.0f32 { fadePercent } else { 100.0f32 }
        } else { 0.0f32 };
}
/*
=================
S_GetMusicVolume
=================
*/
#[no_mangle]
pub unsafe extern "C" fn S_GetMusicVolume() -> libc::c_float {
    let mut scale: libc::c_float = 1.0f32;
    if s_listener.inmenu as u64 == 0 &&
           musicfade.percent != 0 as libc::c_int as libc::c_float {
        scale =
            if musicfade.percent / 100.0f32 >= 0.0f32 {
                if musicfade.percent / 100.0f32 < 1.0f32 {
                    (musicfade.percent) / 100.0f32
                } else { 1.0f32 }
            } else { 0.0f32 };
        scale = 1.0f32 - scale
    }
    return s_musicvolume.value * scale;
}
/*
=================
S_StartBackgroundTrack
=================
*/
#[no_mangle]
pub unsafe extern "C" fn S_StartBackgroundTrack(mut introTrack:
                                                    *const libc::c_char,
                                                mut mainTrack:
                                                    *const libc::c_char,
                                                mut position: libc::c_int,
                                                mut fullpath: qboolean) {
    S_StopBackgroundTrack();
    if dma.initialized as u64 == 0 { return }
    // check for special symbols
    if !introTrack.is_null() && *introTrack as libc::c_int == '*' as i32 {
        introTrack = 0 as *const libc::c_char
    }
    if !mainTrack.is_null() && *mainTrack as libc::c_int == '*' as i32 {
        mainTrack = 0 as *const libc::c_char
    }
    if (if introTrack.is_null() || *introTrack == 0 {
            0 as libc::c_int
        } else { 1 as libc::c_int }) == 0 &&
           (if mainTrack.is_null() || *mainTrack == 0 {
                0 as libc::c_int
            } else { 1 as libc::c_int }) == 0 {
        return
    }
    if introTrack.is_null() { introTrack = mainTrack }
    if *introTrack == 0 { return }
    if if mainTrack.is_null() || *mainTrack == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        s_bgTrack.loopName[0 as libc::c_int as usize] =
            '\u{0}' as i32 as libc::c_char
    } else {
        Q_strncpy(s_bgTrack.loopName.as_mut_ptr(), mainTrack,
                  ::std::mem::size_of::<string>() as libc::c_ulong);
    }
    // open stream
    s_bgTrack.stream =
        FS_OpenStream(va(b"media/%s\x00" as *const u8 as *const libc::c_char,
                         introTrack)); // clear any soundfade
    Q_strncpy(s_bgTrack.current.as_mut_ptr(), introTrack,
              ::std::mem::size_of::<string>() as libc::c_ulong);
    memset(&mut musicfade as *mut musicfade_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<musicfade_t>() as libc::c_ulong);
    s_bgTrack.source = cls.key_dest as libc::c_int;
    if position != 0 as libc::c_int {
        // restore message, update song position
        FS_SetStreamPos(s_bgTrack.stream, position);
    };
}
/*
=================
S_StopBackgroundTrack
=================
*/
#[no_mangle]
pub unsafe extern "C" fn S_StopBackgroundTrack() {
    s_listener.stream_paused = false_0;
    if dma.initialized as u64 == 0 { return }
    if s_bgTrack.stream.is_null() { return }
    FS_FreeStream(s_bgTrack.stream);
    memset(&mut s_bgTrack as *mut bg_track_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<bg_track_t>() as libc::c_ulong);
    memset(&mut musicfade as *mut musicfade_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<musicfade_t>() as libc::c_ulong);
}
/*
=================
S_StreamSetPause
=================
*/
#[no_mangle]
pub unsafe extern "C" fn S_StreamSetPause(mut pause: libc::c_int) {
    s_listener.stream_paused = pause as qboolean;
}
/*
=================
S_StreamGetCurrentState

save\restore code
=================
*/
#[no_mangle]
pub unsafe extern "C" fn S_StreamGetCurrentState(mut currentTrack:
                                                     *mut libc::c_char,
                                                 mut loopTrack:
                                                     *mut libc::c_char,
                                                 mut position:
                                                     *mut libc::c_int)
 -> qboolean {
    if s_bgTrack.stream.is_null() { return false_0 } // not active
    if !currentTrack.is_null() {
        if s_bgTrack.current[0 as libc::c_int as usize] != 0 {
            Q_strncpy(currentTrack, s_bgTrack.current.as_mut_ptr(),
                      256 as libc::c_int as size_t);
        } else {
            Q_strncpy(currentTrack,
                      b"*\x00" as *const u8 as *const libc::c_char,
                      256 as libc::c_int as size_t);
        }
        // no track
    }
    if !loopTrack.is_null() {
        if s_bgTrack.loopName[0 as libc::c_int as usize] != 0 {
            Q_strncpy(loopTrack, s_bgTrack.loopName.as_mut_ptr(),
                      256 as libc::c_int as size_t);
        } else {
            Q_strncpy(loopTrack, b"*\x00" as *const u8 as *const libc::c_char,
                      256 as libc::c_int as size_t);
        }
        // no track
    }
    if !position.is_null() { *position = FS_GetStreamPos(s_bgTrack.stream) }
    return true_0;
}
/*
=================
S_StreamBackgroundTrack
=================
*/
#[no_mangle]
pub unsafe extern "C" fn S_StreamBackgroundTrack() {
    let mut bufferSamples: libc::c_int = 0;
    let mut fileSamples: libc::c_int = 0;
    let mut raw: [byte; 8192] = [0; 8192];
    let mut r: libc::c_int = 0;
    let mut fileBytes: libc::c_int = 0;
    let mut ch: *mut rawchan_t = 0 as *mut rawchan_t;
    if dma.initialized as u64 == 0 || s_bgTrack.stream.is_null() ||
           s_listener.streaming as libc::c_uint != 0 {
        return
    }
    // don't bother playing anything if musicvolume is 0
    if s_musicvolume.value == 0. || s_listener.paused as libc::c_uint != 0 ||
           s_listener.stream_paused as libc::c_uint != 0 {
        return
    }
    if cl.background as u64 == 0 {
        // pause music by source type
        if s_bgTrack.source == key_game as libc::c_int &&
               cls.key_dest as libc::c_uint ==
                   key_menu as libc::c_int as libc::c_uint {
            return
        }
        if s_bgTrack.source == key_menu as libc::c_int &&
               cls.key_dest as libc::c_uint !=
                   key_menu as libc::c_int as libc::c_uint {
            return
        }
    } else if cls.key_dest as libc::c_uint ==
                  key_console as libc::c_int as libc::c_uint {
        return
    }
    ch = S_FindRawChannel(-(2 as libc::c_int), true_0);
    // see how many samples should be copied into the raw buffer
    if (*ch).s_rawend < soundtime as libc::c_uint {
        ::std::ptr::write_volatile(&mut (*ch).s_rawend as *mut uint,
                                   soundtime as uint)
    }
    while ((*ch).s_rawend as libc::c_ulong) <
              (soundtime as libc::c_ulong).wrapping_add((*ch).max_samples) {
        let mut info: *mut wavdata_t = FS_StreamInfo(s_bgTrack.stream);
        bufferSamples =
            (*ch).max_samples.wrapping_sub((*ch).s_rawend.wrapping_sub(soundtime
                                                                           as
                                                                           libc::c_uint)
                                               as libc::c_ulong) as
                libc::c_int;
        // decide how much data needs to be read from the file
        fileSamples =
            (bufferSamples as libc::c_float *
                 ((*info).rate as libc::c_float /
                      44100 as libc::c_int as libc::c_float)) as
                libc::c_int; // no more samples need
        if fileSamples <= 1 as libc::c_int { return }
        // our max buffer size
        fileBytes =
            fileSamples *
                ((*info).width as libc::c_int *
                     (*info).channels as libc::c_int);
        if fileBytes as libc::c_ulong >
               ::std::mem::size_of::<[byte; 8192]>() as libc::c_ulong {
            fileBytes =
                ::std::mem::size_of::<[byte; 8192]>() as libc::c_ulong as
                    libc::c_int;
            fileSamples =
                fileBytes /
                    ((*info).width as libc::c_int *
                         (*info).channels as libc::c_int)
        }
        // read
        r =
            FS_ReadStream(s_bgTrack.stream, fileBytes,
                          raw.as_mut_ptr() as *mut libc::c_void);
        if r < fileBytes {
            fileBytes = r;
            fileSamples =
                r /
                    ((*info).width as libc::c_int *
                         (*info).channels as libc::c_int)
        }
        if r > 0 as libc::c_int {
            // add to raw buffer
            S_RawSamples(fileSamples as uint, (*info).rate as uint,
                         (*info).width as word, (*info).channels as word,
                         raw.as_mut_ptr(), -(2 as libc::c_int));
        } else if s_bgTrack.loopName[0 as libc::c_int as usize] != 0 {
            FS_FreeStream(s_bgTrack.stream);
            s_bgTrack.stream =
                FS_OpenStream(va(b"media/%s\x00" as *const u8 as
                                     *const libc::c_char,
                                 s_bgTrack.loopName.as_mut_ptr()));
            Q_strncpy(s_bgTrack.current.as_mut_ptr(),
                      s_bgTrack.loopName.as_mut_ptr(),
                      ::std::mem::size_of::<string>() as libc::c_ulong);
            if s_bgTrack.stream.is_null() { return }
        } else { S_StopBackgroundTrack(); return }
    };
}
// loop
/*
=================
S_StartStreaming
=================
*/
#[no_mangle]
pub unsafe extern "C" fn S_StartStreaming() {
    if dma.initialized as u64 == 0 { return }
    // begin streaming movie soundtrack
    s_listener.streaming = true_0;
}
/*
=================
S_StopStreaming
=================
*/
#[no_mangle]
pub unsafe extern "C" fn S_StopStreaming() {
    if dma.initialized as u64 == 0 { return }
    s_listener.streaming = false_0;
}
/*
=================
S_StreamSoundTrack
=================
*/
#[no_mangle]
pub unsafe extern "C" fn S_StreamSoundTrack() {
    let mut bufferSamples: libc::c_int = 0;
    let mut fileSamples: libc::c_int = 0;
    let mut raw: [byte; 8192] = [0; 8192];
    let mut r: libc::c_int = 0;
    let mut fileBytes: libc::c_int = 0;
    let mut ch: *mut rawchan_t = 0 as *mut rawchan_t;
    if dma.initialized as u64 == 0 || s_listener.streaming as u64 == 0 ||
           s_listener.paused as libc::c_uint != 0 {
        return
    }
    ch = S_FindRawChannel(-(1 as libc::c_int), true_0);
    // see how many samples should be copied into the raw buffer
    if (*ch).s_rawend < soundtime as libc::c_uint {
        ::std::ptr::write_volatile(&mut (*ch).s_rawend as *mut uint,
                                   soundtime as uint)
    }
    while ((*ch).s_rawend as libc::c_ulong) <
              (soundtime as libc::c_ulong).wrapping_add((*ch).max_samples) {
        let mut info: *mut wavdata_t = SCR_GetMovieInfo();
        // no more samples for this frame
        if info.is_null() {
            break ; // bad soundtrack?
        }
        bufferSamples =
            (*ch).max_samples.wrapping_sub((*ch).s_rawend.wrapping_sub(soundtime
                                                                           as
                                                                           libc::c_uint)
                                               as libc::c_ulong) as
                libc::c_int;
        // decide how much data needs to be read from the file
        fileSamples =
            (bufferSamples as libc::c_float *
                 ((*info).rate as libc::c_float /
                      44100 as libc::c_int as libc::c_float)) as
                libc::c_int; // no more samples need
        if fileSamples <= 1 as libc::c_int { return }
        // our max buffer size
        fileBytes =
            fileSamples *
                ((*info).width as libc::c_int *
                     (*info).channels as libc::c_int);
        if fileBytes as libc::c_ulong >
               ::std::mem::size_of::<[byte; 8192]>() as libc::c_ulong {
            fileBytes =
                ::std::mem::size_of::<[byte; 8192]>() as libc::c_ulong as
                    libc::c_int;
            fileSamples =
                fileBytes /
                    ((*info).width as libc::c_int *
                         (*info).channels as libc::c_int)
        }
        // read audio stream
        r =
            SCR_GetAudioChunk(raw.as_mut_ptr() as *mut libc::c_char,
                              fileBytes);
        if r < fileBytes {
            fileBytes = r;
            fileSamples =
                r /
                    ((*info).width as libc::c_int *
                         (*info).channels as libc::c_int)
        }
        if !(r > 0 as libc::c_int) { break ; }
        S_RawSamples(fileSamples as uint, (*info).rate as uint,
                     (*info).width as word, (*info).channels as word,
                     raw.as_mut_ptr(), -(1 as libc::c_int));
    };
}
// add to raw buffer
