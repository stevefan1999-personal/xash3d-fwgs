#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type grasshdr_s;
    #[no_mangle]
    fn sqrtf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn fabsf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn Q_snprintf(buffer: *mut libc::c_char, buffersize: size_t,
                  format: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn Sys_DoubleTime() -> libc::c_double;
    #[no_mangle]
    fn Sys_Error(error: *const libc::c_char, _: ...);
    #[no_mangle]
    fn _Mem_Alloc(poolptr: poolhandle_t, size: size_t, clear: qboolean,
                  filename: *const libc::c_char, fileline: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    fn Host_Error(error: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_Reportf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    static mut world: world_static_t;
    #[no_mangle]
    static mut loadmodel: *mut model_t;
    #[no_mangle]
    fn Mod_FindName(name: *const libc::c_char, trackCRC: qboolean)
     -> *mut model_t;
    #[no_mangle]
    static mut cl: client_t;
}
pub type __uint32_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type uint = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type byte = libc::c_uchar;
pub type vec_t = libc::c_float;
pub type vec3_t = [vec_t; 3];
pub type poolhandle_t = uint32_t;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type string = [libc::c_char; 256];
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
pub type clientdata_t = clientdata_s;
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
pub type weapon_data_t = weapon_data_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct local_state_s {
    pub playerstate: entity_state_t,
    pub client: clientdata_t,
    pub weapondata: [weapon_data_t; 64],
}
pub type local_state_t = local_state_s;
pub type event_args_t = event_args_s;
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
pub type consistency_t = consistency_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winding_s {
    pub plane: *const mplane_t,
    pub pair: *mut winding_s,
    pub chain: hullnode_t,
    pub numpoints: libc::c_int,
    pub p: [vec3_t; 4],
}
pub type winding_t = winding_s;
pub type world_static_t = world_static_s;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pred_viewangle_t {
    pub starttime: libc::c_float,
    pub total: libc::c_float,
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
unsafe extern "C" fn __tg_sqrt(mut __x: libc::c_float) -> libc::c_float {
    return sqrtf(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_fabs(mut __x: libc::c_float) -> libc::c_float {
    return fabsf(__x);
}
#[inline]
unsafe extern "C" fn list_add__(mut new: *mut hullnode_t,
                                mut prev: *mut hullnode_t,
                                mut next: *mut hullnode_t) {
    (*next).prev = new;
    (*new).next = next;
    (*new).prev = prev;
    (*prev).next = new;
}
// add the new entry after the give list entry
#[inline]
unsafe extern "C" fn list_add(mut newobj: *mut hullnode_t,
                              mut head: *mut hullnode_t) {
    list_add__(newobj, head, (*head).next);
}
#[inline]
unsafe extern "C" fn list_del(mut entry: *mut hullnode_t) {
    (*(*entry).next).prev = (*entry).prev;
    (*(*entry).prev).next = (*entry).next;
}
unsafe extern "C" fn winding_alloc(mut numpoints: uint) -> *mut winding_t {
    return malloc(offset_of!(winding_t, p [ numpoints as usize ]) as
                      libc::c_ulong) as *mut winding_t;
}
unsafe extern "C" fn free_winding(mut w: *mut winding_t) {
    // simple sentinel by Carmack
    if *(w as *mut libc::c_uint) == 0xdeadc0de as libc::c_uint {
        Host_Error(b"free_winding: freed a freed winding\n\x00" as *const u8
                       as *const libc::c_char);
    }
    *(w as *mut libc::c_uint) = 0xdeadc0de as libc::c_uint;
    free(w as *mut libc::c_void);
}
unsafe extern "C" fn winding_copy(mut w: *mut winding_t) -> *mut winding_t {
    let mut neww: *mut winding_t = 0 as *mut winding_t;
    neww = winding_alloc((*w).numpoints as uint);
    memcpy(neww as *mut libc::c_void, w as *const libc::c_void,
           offset_of!(winding_t, p [ (*w).numpoints as usize ]) as
               libc::c_ulong);
    return neww;
}
unsafe extern "C" fn winding_reverse(mut w: *mut winding_t) {
    let mut point: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*w).numpoints / 2 as libc::c_int {
        point[0 as libc::c_int as usize] =
            (*w).p[i as usize][0 as libc::c_int as usize];
        point[1 as libc::c_int as usize] =
            (*w).p[i as usize][1 as libc::c_int as usize];
        point[2 as libc::c_int as usize] =
            (*w).p[i as usize][2 as libc::c_int as usize];
        (*w).p[i as usize][0 as libc::c_int as usize] =
            (*w).p[((*w).numpoints - i - 1 as libc::c_int) as
                       usize][0 as libc::c_int as usize];
        (*w).p[i as usize][1 as libc::c_int as usize] =
            (*w).p[((*w).numpoints - i - 1 as libc::c_int) as
                       usize][1 as libc::c_int as usize];
        (*w).p[i as usize][2 as libc::c_int as usize] =
            (*w).p[((*w).numpoints - i - 1 as libc::c_int) as
                       usize][2 as libc::c_int as usize];
        (*w).p[((*w).numpoints - i - 1 as libc::c_int) as
                   usize][0 as libc::c_int as usize] =
            point[0 as libc::c_int as usize];
        (*w).p[((*w).numpoints - i - 1 as libc::c_int) as
                   usize][1 as libc::c_int as usize] =
            point[1 as libc::c_int as usize];
        (*w).p[((*w).numpoints - i - 1 as libc::c_int) as
                   usize][2 as libc::c_int as usize] =
            point[2 as libc::c_int as usize];
        i += 1
    };
}
/*
 * winding_shrink
 *
 * Takes an over-allocated winding and allocates a new winding with just the
 * required number of points. The input winding is freed.
 */
unsafe extern "C" fn winding_shrink(mut w: *mut winding_t) -> *mut winding_t {
    let mut neww: *mut winding_t = winding_alloc((*w).numpoints as uint);
    memcpy(neww as *mut libc::c_void, w as *const libc::c_void,
           offset_of!(winding_t, p [ (*w).numpoints as usize ]) as
               libc::c_ulong);
    free_winding(w);
    return neww;
}
/*
====================
winding_for_plane
====================
*/
unsafe extern "C" fn winding_for_plane(mut p: *const mplane_t)
 -> *mut winding_t {
    let mut org: vec3_t = [0.; 3];
    let mut vright: vec3_t = [0.; 3];
    let mut vup: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    let mut axis: libc::c_int = 0;
    let mut max: vec_t = 0.;
    let mut v: vec_t = 0.;
    let mut w: *mut winding_t = 0 as *mut winding_t;
    // find the major axis
    max = -(114032.64f64 as vec_t);
    axis = -(1 as libc::c_int);
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        v = __tg_fabs((*p).normal[i as usize]);
        if v > max { axis = i; max = v }
        i += 1
    }
    vup[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    vup[1 as libc::c_int as usize] = vup[2 as libc::c_int as usize];
    vup[0 as libc::c_int as usize] = vup[1 as libc::c_int as usize];
    match axis {
        0 | 1 => {
            vup[2 as libc::c_int as usize] = 1 as libc::c_int as vec_t
        }
        2 => { vup[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t }
        _ => {
            Host_Error(b"BaseWindingForPlane: no axis found\n\x00" as
                           *const u8 as *const libc::c_char);
            return 0 as *mut winding_t
        }
    }
    v =
        vup[0 as libc::c_int as usize] *
            (*p).normal[0 as libc::c_int as usize] +
            vup[1 as libc::c_int as usize] *
                (*p).normal[1 as libc::c_int as usize] +
            vup[2 as libc::c_int as usize] *
                (*p).normal[2 as libc::c_int as usize];
    vup[0 as libc::c_int as usize] =
        vup[0 as libc::c_int as usize] +
            -v * (*p).normal[0 as libc::c_int as usize];
    vup[1 as libc::c_int as usize] =
        vup[1 as libc::c_int as usize] +
            -v * (*p).normal[1 as libc::c_int as usize];
    vup[2 as libc::c_int as usize] =
        vup[2 as libc::c_int as usize] +
            -v * (*p).normal[2 as libc::c_int as usize];
    let mut ilength: libc::c_float =
        __tg_sqrt(vup[0 as libc::c_int as usize] *
                      vup[0 as libc::c_int as usize] +
                      vup[1 as libc::c_int as usize] *
                          vup[1 as libc::c_int as usize] +
                      vup[2 as libc::c_int as usize] *
                          vup[2 as libc::c_int as usize]);
    if ilength != 0. { ilength = 1.0f32 / ilength }
    vup[0 as libc::c_int as usize] *= ilength;
    vup[1 as libc::c_int as usize] *= ilength;
    vup[2 as libc::c_int as usize] *= ilength;
    org[0 as libc::c_int as usize] =
        (*p).normal[0 as libc::c_int as usize] * (*p).dist;
    org[1 as libc::c_int as usize] =
        (*p).normal[1 as libc::c_int as usize] * (*p).dist;
    org[2 as libc::c_int as usize] =
        (*p).normal[2 as libc::c_int as usize] * (*p).dist;
    vright[0 as libc::c_int as usize] =
        vup[1 as libc::c_int as usize] *
            (*p).normal[2 as libc::c_int as usize] -
            vup[2 as libc::c_int as usize] *
                (*p).normal[1 as libc::c_int as usize];
    vright[1 as libc::c_int as usize] =
        vup[2 as libc::c_int as usize] *
            (*p).normal[0 as libc::c_int as usize] -
            vup[0 as libc::c_int as usize] *
                (*p).normal[2 as libc::c_int as usize];
    vright[2 as libc::c_int as usize] =
        vup[0 as libc::c_int as usize] *
            (*p).normal[1 as libc::c_int as usize] -
            vup[1 as libc::c_int as usize] *
                (*p).normal[0 as libc::c_int as usize];
    vup[0 as libc::c_int as usize] =
        vup[0 as libc::c_int as usize] * 114032.64f64 as vec_t;
    vup[1 as libc::c_int as usize] =
        vup[1 as libc::c_int as usize] * 114032.64f64 as vec_t;
    vup[2 as libc::c_int as usize] =
        vup[2 as libc::c_int as usize] * 114032.64f64 as vec_t;
    vright[0 as libc::c_int as usize] =
        vright[0 as libc::c_int as usize] * 114032.64f64 as vec_t;
    vright[1 as libc::c_int as usize] =
        vright[1 as libc::c_int as usize] * 114032.64f64 as vec_t;
    vright[2 as libc::c_int as usize] =
        vright[2 as libc::c_int as usize] * 114032.64f64 as vec_t;
    // project a really big axis aligned box onto the plane
    w = winding_alloc(4 as libc::c_int as uint);
    memset((*w).p.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<vec3_t>() as
                libc::c_ulong).wrapping_mul(4 as libc::c_int as
                                                libc::c_ulong));
    (*w).numpoints = 4 as libc::c_int;
    (*w).plane = p;
    (*w).p[0 as libc::c_int as usize][0 as libc::c_int as usize] =
        org[0 as libc::c_int as usize] - vright[0 as libc::c_int as usize];
    (*w).p[0 as libc::c_int as usize][1 as libc::c_int as usize] =
        org[1 as libc::c_int as usize] - vright[1 as libc::c_int as usize];
    (*w).p[0 as libc::c_int as usize][2 as libc::c_int as usize] =
        org[2 as libc::c_int as usize] - vright[2 as libc::c_int as usize];
    (*w).p[0 as libc::c_int as usize][0 as libc::c_int as usize] =
        (*w).p[0 as libc::c_int as usize][0 as libc::c_int as usize] +
            vup[0 as libc::c_int as usize];
    (*w).p[0 as libc::c_int as usize][1 as libc::c_int as usize] =
        (*w).p[0 as libc::c_int as usize][1 as libc::c_int as usize] +
            vup[1 as libc::c_int as usize];
    (*w).p[0 as libc::c_int as usize][2 as libc::c_int as usize] =
        (*w).p[0 as libc::c_int as usize][2 as libc::c_int as usize] +
            vup[2 as libc::c_int as usize];
    (*w).p[1 as libc::c_int as usize][0 as libc::c_int as usize] =
        org[0 as libc::c_int as usize] + vright[0 as libc::c_int as usize];
    (*w).p[1 as libc::c_int as usize][1 as libc::c_int as usize] =
        org[1 as libc::c_int as usize] + vright[1 as libc::c_int as usize];
    (*w).p[1 as libc::c_int as usize][2 as libc::c_int as usize] =
        org[2 as libc::c_int as usize] + vright[2 as libc::c_int as usize];
    (*w).p[1 as libc::c_int as usize][0 as libc::c_int as usize] =
        (*w).p[1 as libc::c_int as usize][0 as libc::c_int as usize] +
            vup[0 as libc::c_int as usize];
    (*w).p[1 as libc::c_int as usize][1 as libc::c_int as usize] =
        (*w).p[1 as libc::c_int as usize][1 as libc::c_int as usize] +
            vup[1 as libc::c_int as usize];
    (*w).p[1 as libc::c_int as usize][2 as libc::c_int as usize] =
        (*w).p[1 as libc::c_int as usize][2 as libc::c_int as usize] +
            vup[2 as libc::c_int as usize];
    (*w).p[2 as libc::c_int as usize][0 as libc::c_int as usize] =
        org[0 as libc::c_int as usize] + vright[0 as libc::c_int as usize];
    (*w).p[2 as libc::c_int as usize][1 as libc::c_int as usize] =
        org[1 as libc::c_int as usize] + vright[1 as libc::c_int as usize];
    (*w).p[2 as libc::c_int as usize][2 as libc::c_int as usize] =
        org[2 as libc::c_int as usize] + vright[2 as libc::c_int as usize];
    (*w).p[2 as libc::c_int as usize][0 as libc::c_int as usize] =
        (*w).p[2 as libc::c_int as usize][0 as libc::c_int as usize] -
            vup[0 as libc::c_int as usize];
    (*w).p[2 as libc::c_int as usize][1 as libc::c_int as usize] =
        (*w).p[2 as libc::c_int as usize][1 as libc::c_int as usize] -
            vup[1 as libc::c_int as usize];
    (*w).p[2 as libc::c_int as usize][2 as libc::c_int as usize] =
        (*w).p[2 as libc::c_int as usize][2 as libc::c_int as usize] -
            vup[2 as libc::c_int as usize];
    (*w).p[3 as libc::c_int as usize][0 as libc::c_int as usize] =
        org[0 as libc::c_int as usize] - vright[0 as libc::c_int as usize];
    (*w).p[3 as libc::c_int as usize][1 as libc::c_int as usize] =
        org[1 as libc::c_int as usize] - vright[1 as libc::c_int as usize];
    (*w).p[3 as libc::c_int as usize][2 as libc::c_int as usize] =
        org[2 as libc::c_int as usize] - vright[2 as libc::c_int as usize];
    (*w).p[3 as libc::c_int as usize][0 as libc::c_int as usize] =
        (*w).p[3 as libc::c_int as usize][0 as libc::c_int as usize] -
            vup[0 as libc::c_int as usize];
    (*w).p[3 as libc::c_int as usize][1 as libc::c_int as usize] =
        (*w).p[3 as libc::c_int as usize][1 as libc::c_int as usize] -
            vup[1 as libc::c_int as usize];
    (*w).p[3 as libc::c_int as usize][2 as libc::c_int as usize] =
        (*w).p[3 as libc::c_int as usize][2 as libc::c_int as usize] -
            vup[2 as libc::c_int as usize];
    return w;
}
/*
 * ===========================
 * Helper for for the clipping functions
 *  (winding_clip, winding_split)
 * ===========================
 */
unsafe extern "C" fn CalcSides(mut in_0: *const winding_t,
                               mut split: *const mplane_t,
                               mut sides: *mut libc::c_int,
                               mut dists: *mut vec_t,
                               mut counts: *mut libc::c_int,
                               mut epsilon: vec_t) {
    let mut p: *const vec_t = 0 as *const vec_t;
    let mut i: libc::c_int = 0;
    let ref mut fresh0 = *counts.offset(2 as libc::c_int as isize);
    *fresh0 = 0 as libc::c_int;
    let ref mut fresh1 = *counts.offset(1 as libc::c_int as isize);
    *fresh1 = *fresh0;
    *counts.offset(0 as libc::c_int as isize) = *fresh1;
    match (*split).type_0 as libc::c_int {
        0 | 1 | 2 => {
            p =
                (*in_0).p[0 as libc::c_int as
                              usize].as_ptr().offset((*split).type_0 as
                                                         libc::c_int as
                                                         isize);
            i = 0 as libc::c_int;
            while i < (*in_0).numpoints {
                let dot: vec_t = *p - (*split).dist;
                *dists.offset(i as isize) = dot;
                if dot > epsilon {
                    *sides.offset(i as isize) = 0 as libc::c_int
                } else if dot < -epsilon {
                    *sides.offset(i as isize) = 1 as libc::c_int
                } else { *sides.offset(i as isize) = 2 as libc::c_int }
                let ref mut fresh2 =
                    *counts.offset(*sides.offset(i as isize) as isize);
                *fresh2 += 1;
                i += 1;
                p = p.offset(3 as libc::c_int as isize)
            }
        }
        _ => {
            p = (*in_0).p[0 as libc::c_int as usize].as_ptr();
            i = 0 as libc::c_int;
            while i < (*in_0).numpoints {
                let dot_0: vec_t =
                    (*split).normal[0 as libc::c_int as usize] *
                        *p.offset(0 as libc::c_int as isize) +
                        (*split).normal[1 as libc::c_int as usize] *
                            *p.offset(1 as libc::c_int as isize) +
                        (*split).normal[2 as libc::c_int as usize] *
                            *p.offset(2 as libc::c_int as isize) -
                        (*split).dist;
                *dists.offset(i as isize) = dot_0;
                if dot_0 > epsilon {
                    *sides.offset(i as isize) = 0 as libc::c_int
                } else if dot_0 < -epsilon {
                    *sides.offset(i as isize) = 1 as libc::c_int
                } else { *sides.offset(i as isize) = 2 as libc::c_int }
                let ref mut fresh3 =
                    *counts.offset(*sides.offset(i as isize) as isize);
                *fresh3 += 1;
                i += 1;
                p = p.offset(3 as libc::c_int as isize)
            }
        }
    }
    *sides.offset(i as isize) = *sides.offset(0 as libc::c_int as isize);
    *dists.offset(i as isize) = *dists.offset(0 as libc::c_int as isize);
}
unsafe extern "C" fn PushToPlaneAxis(mut v: *mut vec_t,
                                     mut p: *const mplane_t) {
    let t: libc::c_int = (*p).type_0 as libc::c_int % 3 as libc::c_int;
    *v.offset(t as isize) =
        ((*p).dist -
             (*p).normal[((t + 1 as libc::c_int) % 3 as libc::c_int) as usize]
                 *
                 *v.offset(((t + 1 as libc::c_int) % 3 as libc::c_int) as
                               isize) -
             (*p).normal[((t + 2 as libc::c_int) % 3 as libc::c_int) as usize]
                 *
                 *v.offset(((t + 2 as libc::c_int) % 3 as libc::c_int) as
                               isize)) / (*p).normal[t as usize];
}
/*
==================
winding_clip

Clips the winding to the plane, returning the new winding on 'side'.
Frees the input winding.
If keepon is true, an exactly on-plane winding will be saved, otherwise
  it will be clipped away.
==================
*/
unsafe extern "C" fn winding_clip(mut in_0: *mut winding_t,
                                  mut split: *const mplane_t,
                                  mut keepon: qboolean, mut side: libc::c_int,
                                  mut epsilon: vec_t) -> *mut winding_t {
    let mut dists: *mut vec_t = 0 as *mut vec_t;
    let mut sides: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut counts: [libc::c_int; 3] = [0; 3];
    let mut dot: vec_t = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut neww: *mut winding_t = 0 as *mut winding_t;
    let mut p1: *mut vec_t = 0 as *mut vec_t;
    let mut p2: *mut vec_t = 0 as *mut vec_t;
    let mut mid: *mut vec_t = 0 as *mut vec_t;
    let mut maxpts: libc::c_int = 0;
    dists =
        malloc((((*in_0).numpoints + 1 as libc::c_int) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<vec_t>()
                                                    as libc::c_ulong)) as
            *mut vec_t;
    sides =
        malloc((((*in_0).numpoints + 1 as libc::c_int) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                    as libc::c_ulong)) as
            *mut libc::c_int;
    CalcSides(in_0, split, sides, dists, counts.as_mut_ptr(), epsilon);
    if keepon as libc::c_uint != 0 && counts[0 as libc::c_int as usize] == 0
           && counts[1 as libc::c_int as usize] == 0 {
        neww = in_0
    } else if counts[side as usize] == 0 {
        free_winding(in_0);
        neww = 0 as *mut winding_t
    } else if counts[(side ^ 1 as libc::c_int) as usize] == 0 {
        neww = in_0
    } else {
        maxpts = (*in_0).numpoints + 4 as libc::c_int;
        neww = winding_alloc(maxpts as uint);
        (*neww).numpoints = 0 as libc::c_int;
        (*neww).plane = (*in_0).plane;
        i = 0 as libc::c_int;
        while i < (*in_0).numpoints {
            p1 = (*in_0).p[i as usize].as_mut_ptr();
            if *sides.offset(i as isize) == 2 as libc::c_int {
                (*neww).p[(*neww).numpoints as
                              usize][0 as libc::c_int as usize] =
                    *p1.offset(0 as libc::c_int as isize);
                (*neww).p[(*neww).numpoints as
                              usize][1 as libc::c_int as usize] =
                    *p1.offset(1 as libc::c_int as isize);
                (*neww).p[(*neww).numpoints as
                              usize][2 as libc::c_int as usize] =
                    *p1.offset(2 as libc::c_int as isize);
                (*neww).numpoints += 1
            } else {
                if *sides.offset(i as isize) == side {
                    (*neww).p[(*neww).numpoints as
                                  usize][0 as libc::c_int as usize] =
                        *p1.offset(0 as libc::c_int as isize);
                    (*neww).p[(*neww).numpoints as
                                  usize][1 as libc::c_int as usize] =
                        *p1.offset(1 as libc::c_int as isize);
                    (*neww).p[(*neww).numpoints as
                                  usize][2 as libc::c_int as usize] =
                        *p1.offset(2 as libc::c_int as isize);
                    (*neww).numpoints += 1
                }
                if !(*sides.offset((i + 1 as libc::c_int) as isize) ==
                         2 as libc::c_int ||
                         *sides.offset((i + 1 as libc::c_int) as isize) ==
                             *sides.offset(i as isize)) {
                    // generate a split point
                    p2 =
                        (*in_0).p[((i + 1 as libc::c_int) % (*in_0).numpoints)
                                      as usize].as_mut_ptr();
                    let fresh4 = (*neww).numpoints;
                    (*neww).numpoints = (*neww).numpoints + 1;
                    mid = (*neww).p[fresh4 as usize].as_mut_ptr();
                    dot =
                        *dists.offset(i as isize) /
                            (*dists.offset(i as isize) -
                                 *dists.offset((i + 1 as libc::c_int) as
                                                   isize));
                    j = 0 as libc::c_int;
                    while j < 3 as libc::c_int {
                        // avoid round off error when possible
                        if (*(*in_0).plane).normal[j as usize] == 1.0f32 {
                            *mid.offset(j as isize) = (*(*in_0).plane).dist
                        } else if (*(*in_0).plane).normal[j as usize] ==
                                      -1.0f32 {
                            *mid.offset(j as isize) = -(*(*in_0).plane).dist
                        } else if (*split).normal[j as usize] == 1.0f32 {
                            *mid.offset(j as isize) = (*split).dist
                        } else if (*split).normal[j as usize] == -1.0f32 {
                            *mid.offset(j as isize) = -(*split).dist
                        } else {
                            *mid.offset(j as isize) =
                                *p1.offset(j as isize) +
                                    dot *
                                        (*p2.offset(j as isize) -
                                             *p1.offset(j as isize))
                        }
                        j += 1
                    }
                    if ((*(*in_0).plane).type_0 as libc::c_int) <
                           3 as libc::c_int {
                        PushToPlaneAxis(mid, (*in_0).plane);
                    }
                }
            }
            i += 1
        }
        // free the original winding
        free_winding(in_0);
        // Shrink the winding back to just what it needs...
        neww = winding_shrink(neww)
    }
    free(dists as *mut libc::c_void);
    free(sides as *mut libc::c_void);
    return neww;
}
/*
==================
winding_split

Splits a winding by a plane, producing one or two windings.  The
original winding is not damaged or freed.  If only on one side, the
returned winding will be the input winding.  If on both sides, two
new windings will be created.
==================
*/
unsafe extern "C" fn winding_split(mut in_0: *mut winding_t,
                                   mut split: *const mplane_t,
                                   mut pfront: *mut *mut winding_t,
                                   mut pback: *mut *mut winding_t) {
    let mut dists: *mut vec_t = 0 as *mut vec_t;
    let mut sides: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut counts: [libc::c_int; 3] = [0; 3];
    let mut dot: vec_t = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut front: *mut winding_t = 0 as *mut winding_t;
    let mut back: *mut winding_t = 0 as *mut winding_t;
    let mut p1: *mut vec_t = 0 as *mut vec_t;
    let mut p2: *mut vec_t = 0 as *mut vec_t;
    let mut mid: *mut vec_t = 0 as *mut vec_t;
    let mut maxpts: libc::c_int = 0;
    dists =
        malloc((((*in_0).numpoints + 1 as libc::c_int) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<vec_t>()
                                                    as libc::c_ulong)) as
            *mut vec_t;
    sides =
        malloc((((*in_0).numpoints + 1 as libc::c_int) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                    as libc::c_ulong)) as
            *mut libc::c_int;
    CalcSides(in_0, split, sides, dists, counts.as_mut_ptr(), 0.04f32);
    if counts[0 as libc::c_int as usize] == 0 &&
           counts[1 as libc::c_int as usize] == 0 {
        // winding on the split plane - return copies on both sides
        *pfront = winding_copy(in_0);
        *pback = winding_copy(in_0)
    } else if counts[0 as libc::c_int as usize] == 0 {
        *pfront = 0 as *mut winding_t;
        *pback = in_0
    } else if counts[1 as libc::c_int as usize] == 0 {
        *pfront = in_0;
        *pback = 0 as *mut winding_t
    } else {
        maxpts = (*in_0).numpoints + 4 as libc::c_int;
        front = winding_alloc(maxpts as uint);
        (*front).numpoints = 0 as libc::c_int;
        (*front).plane = (*in_0).plane;
        back = winding_alloc(maxpts as uint);
        (*back).numpoints = 0 as libc::c_int;
        (*back).plane = (*in_0).plane;
        i = 0 as libc::c_int;
        while i < (*in_0).numpoints {
            p1 = (*in_0).p[i as usize].as_mut_ptr();
            if *sides.offset(i as isize) == 2 as libc::c_int {
                (*front).p[(*front).numpoints as
                               usize][0 as libc::c_int as usize] =
                    *p1.offset(0 as libc::c_int as isize);
                (*front).p[(*front).numpoints as
                               usize][1 as libc::c_int as usize] =
                    *p1.offset(1 as libc::c_int as isize);
                (*front).p[(*front).numpoints as
                               usize][2 as libc::c_int as usize] =
                    *p1.offset(2 as libc::c_int as isize);
                (*back).p[(*back).numpoints as
                              usize][0 as libc::c_int as usize] =
                    *p1.offset(0 as libc::c_int as isize);
                (*back).p[(*back).numpoints as
                              usize][1 as libc::c_int as usize] =
                    *p1.offset(1 as libc::c_int as isize);
                (*back).p[(*back).numpoints as
                              usize][2 as libc::c_int as usize] =
                    *p1.offset(2 as libc::c_int as isize);
                (*front).numpoints += 1;
                (*back).numpoints += 1
            } else {
                if *sides.offset(i as isize) == 0 as libc::c_int {
                    (*front).p[(*front).numpoints as
                                   usize][0 as libc::c_int as usize] =
                        *p1.offset(0 as libc::c_int as isize);
                    (*front).p[(*front).numpoints as
                                   usize][1 as libc::c_int as usize] =
                        *p1.offset(1 as libc::c_int as isize);
                    (*front).p[(*front).numpoints as
                                   usize][2 as libc::c_int as usize] =
                        *p1.offset(2 as libc::c_int as isize);
                    (*front).numpoints += 1
                } else if *sides.offset(i as isize) == 1 as libc::c_int {
                    (*back).p[(*back).numpoints as
                                  usize][0 as libc::c_int as usize] =
                        *p1.offset(0 as libc::c_int as isize);
                    (*back).p[(*back).numpoints as
                                  usize][1 as libc::c_int as usize] =
                        *p1.offset(1 as libc::c_int as isize);
                    (*back).p[(*back).numpoints as
                                  usize][2 as libc::c_int as usize] =
                        *p1.offset(2 as libc::c_int as isize);
                    (*back).numpoints += 1
                }
                if !(*sides.offset((i + 1 as libc::c_int) as isize) ==
                         2 as libc::c_int ||
                         *sides.offset((i + 1 as libc::c_int) as isize) ==
                             *sides.offset(i as isize)) {
                    // generate a split point
                    p2 =
                        (*in_0).p[((i + 1 as libc::c_int) % (*in_0).numpoints)
                                      as usize].as_mut_ptr();
                    let fresh5 = (*front).numpoints;
                    (*front).numpoints = (*front).numpoints + 1;
                    mid = (*front).p[fresh5 as usize].as_mut_ptr();
                    dot =
                        *dists.offset(i as isize) /
                            (*dists.offset(i as isize) -
                                 *dists.offset((i + 1 as libc::c_int) as
                                                   isize));
                    j = 0 as libc::c_int;
                    while j < 3 as libc::c_int {
                        // avoid round off error when possible
                        if (*(*in_0).plane).normal[j as usize] == 1.0f32 {
                            *mid.offset(j as isize) = (*(*in_0).plane).dist
                        } else if (*(*in_0).plane).normal[j as usize] ==
                                      -1.0f32 {
                            *mid.offset(j as isize) = -(*(*in_0).plane).dist
                        } else if (*split).normal[j as usize] == 1.0f32 {
                            *mid.offset(j as isize) = (*split).dist
                        } else if (*split).normal[j as usize] == -1.0f32 {
                            *mid.offset(j as isize) = -(*split).dist
                        } else {
                            *mid.offset(j as isize) =
                                *p1.offset(j as isize) +
                                    dot *
                                        (*p2.offset(j as isize) -
                                             *p1.offset(j as isize))
                        }
                        j += 1
                    }
                    if ((*(*in_0).plane).type_0 as libc::c_int) <
                           3 as libc::c_int {
                        PushToPlaneAxis(mid, (*in_0).plane);
                    }
                    (*back).p[(*back).numpoints as
                                  usize][0 as libc::c_int as usize] =
                        *mid.offset(0 as libc::c_int as isize);
                    (*back).p[(*back).numpoints as
                                  usize][1 as libc::c_int as usize] =
                        *mid.offset(1 as libc::c_int as isize);
                    (*back).p[(*back).numpoints as
                                  usize][2 as libc::c_int as usize] =
                        *mid.offset(2 as libc::c_int as isize);
                    (*back).numpoints += 1
                }
            }
            i += 1
        }
        *pfront = winding_shrink(front);
        *pback = winding_shrink(back)
    }
    free(dists as *mut libc::c_void);
    free(sides as *mut libc::c_void);
}
/* ------------------------------------------------------------------------- */
/*
 * This is a stack of the clipnodes we have traversed
 * "sides" indicates which side we went down each time
 */
static mut node_stack: [*mut mclipnode_t; 256] =
    [0 as *const mclipnode_t as *mut mclipnode_t; 256];
static mut side_stack: [libc::c_int; 256] = [0; 256];
static mut node_stack_depth: uint = 0;
unsafe extern "C" fn push_node(mut node: *mut mclipnode_t,
                               mut side: libc::c_int) {
    if node_stack_depth == 256 as libc::c_int as libc::c_uint {
        Host_Error(b"node stack overflow\n\x00" as *const u8 as
                       *const libc::c_char);
    }
    node_stack[node_stack_depth as usize] = node;
    side_stack[node_stack_depth as usize] = side;
    node_stack_depth = node_stack_depth.wrapping_add(1);
}
unsafe extern "C" fn pop_node() {
    if node_stack_depth == 0 {
        Host_Error(b"node stack underflow\n\x00" as *const u8 as
                       *const libc::c_char);
    }
    node_stack_depth = node_stack_depth.wrapping_sub(1);
}
unsafe extern "C" fn free_hull_polys(mut hull_polys: *mut hullnode_t) {
    let mut w: *mut winding_t = 0 as *mut winding_t;
    let mut next: *mut winding_t = 0 as *mut winding_t;
    w =
        ((*hull_polys).next as
             *mut libc::c_char).offset(-(&mut (*(0 as *mut winding_t)).chain
                                             as *mut hullnode_t as size_t as
                                             isize)) as *mut winding_t;
    next =
        ((*w).chain.next as
             *mut libc::c_char).offset(-(&mut (*(0 as *mut winding_t)).chain
                                             as *mut hullnode_t as size_t as
                                             isize)) as *mut winding_t;
    while &mut (*w).chain as *mut hullnode_t != hull_polys {
        list_del(&mut (*w).chain);
        free_winding(w);
        w = next;
        next =
            ((*next).chain.next as
                 *mut libc::c_char).offset(-(&mut (*(0 as
                                                         *mut winding_t)).chain
                                                 as *mut hullnode_t as size_t
                                                 as isize)) as *mut winding_t
    };
}
unsafe extern "C" fn do_hull_recursion(mut hull: *mut hull_t,
                                       mut node: *mut mclipnode_t,
                                       mut side: libc::c_int,
                                       mut polys: *mut hullnode_t,
                                       mut model: *mut hull_model_t) {
    let mut w: *mut winding_t = 0 as *mut winding_t;
    let mut next: *mut winding_t = 0 as *mut winding_t;
    if (*node).children[side as usize] as libc::c_int >= 0 as libc::c_int {
        let mut child: *mut mclipnode_t =
            (*hull).clipnodes.offset((*node).children[side as usize] as
                                         libc::c_int as isize);
        push_node(node, side);
        hull_windings_r(hull, child, polys, model);
        pop_node();
    } else {
        match (*node).children[side as usize] as libc::c_int {
            -1 | -3 | -4 | -5 => {
                w =
                    ((*polys).next as
                         *mut libc::c_char).offset(-(&mut (*(0 as
                                                                 *mut winding_t)).chain
                                                         as *mut hullnode_t as
                                                         size_t as isize)) as
                        *mut winding_t;
                next =
                    ((*w).chain.next as
                         *mut libc::c_char).offset(-(&mut (*(0 as
                                                                 *mut winding_t)).chain
                                                         as *mut hullnode_t as
                                                         size_t as isize)) as
                        *mut winding_t;
                while &mut (*w).chain as *mut hullnode_t != polys {
                    list_del(&mut (*w).chain);
                    list_add(&mut (*w).chain, &mut (*model).polys);
                    w = next;
                    next =
                        ((*next).chain.next as
                             *mut libc::c_char).offset(-(&mut (*(0 as
                                                                     *mut winding_t)).chain
                                                             as
                                                             *mut hullnode_t
                                                             as size_t as
                                                             isize)) as
                            *mut winding_t
                }
            }
            -2 | -6 => {
                // throw away polys...
                w =
                    ((*polys).next as
                         *mut libc::c_char).offset(-(&mut (*(0 as
                                                                 *mut winding_t)).chain
                                                         as *mut hullnode_t as
                                                         size_t as isize)) as
                        *mut winding_t;
                next =
                    ((*w).chain.next as
                         *mut libc::c_char).offset(-(&mut (*(0 as
                                                                 *mut winding_t)).chain
                                                         as *mut hullnode_t as
                                                         size_t as isize)) as
                        *mut winding_t;
                while &mut (*w).chain as *mut hullnode_t != polys {
                    if !(*w).pair.is_null() {
                        (*(*w).pair).pair = 0 as *mut winding_s
                    }
                    list_del(&mut (*w).chain);
                    free_winding(w);
                    (*model).num_polys = (*model).num_polys.wrapping_sub(1);
                    w = next;
                    next =
                        ((*next).chain.next as
                             *mut libc::c_char).offset(-(&mut (*(0 as
                                                                     *mut winding_t)).chain
                                                             as
                                                             *mut hullnode_t
                                                             as size_t as
                                                             isize)) as
                            *mut winding_t
                }
            }
            _ => {
                Host_Error(b"bad contents: %i\n\x00" as *const u8 as
                               *const libc::c_char,
                           (*node).children[side as usize] as libc::c_int);
            }
        }
    };
}
unsafe extern "C" fn hull_windings_r(mut hull: *mut hull_t,
                                     mut node: *mut mclipnode_t,
                                     mut polys: *mut hullnode_t,
                                     mut model: *mut hull_model_t) {
    let mut plane: *mut mplane_t =
        (*hull).planes.offset((*node).planenum as isize);
    let mut frontlist: hullnode_t =
        hullnode_t{next: 0 as *mut hullnode_s, prev: 0 as *mut hullnode_s,};
    frontlist =
        {
            let mut init =
                hullnode_s{next: &mut frontlist, prev: &mut frontlist,};
            init
        };
    let mut backlist: hullnode_t =
        hullnode_t{next: 0 as *mut hullnode_s, prev: 0 as *mut hullnode_s,};
    backlist =
        {
            let mut init =
                hullnode_s{next: &mut backlist, prev: &mut backlist,};
            init
        };
    let mut w: *mut winding_t = 0 as *mut winding_t;
    let mut next: *mut winding_t = 0 as *mut winding_t;
    let mut front: *mut winding_t = 0 as *mut winding_t;
    let mut back: *mut winding_t = 0 as *mut winding_t;
    let mut i: libc::c_int = 0;
    w =
        ((*polys).next as
             *mut libc::c_char).offset(-(&mut (*(0 as *mut winding_t)).chain
                                             as *mut hullnode_t as size_t as
                                             isize)) as *mut winding_t;
    next =
        ((*w).chain.next as
             *mut libc::c_char).offset(-(&mut (*(0 as *mut winding_t)).chain
                                             as *mut hullnode_t as size_t as
                                             isize)) as *mut winding_t;
    while &mut (*w).chain as *mut hullnode_t != polys {
        // PARANIOA - PAIR CHECK
        if !((*w).pair.is_null() || (*(*w).pair).pair == w) {
            Sys_Error(b"assert failed at %s:%i\n\x00" as *const u8 as
                          *const libc::c_char,
                      b"../engine/client/mod_dbghulls.c\x00" as *const u8 as
                          *const libc::c_char, 560 as libc::c_int);
        }
        list_del(&mut (*w).chain);
        winding_split(w, plane, &mut front, &mut back);
        if !front.is_null() { list_add(&mut (*front).chain, &mut frontlist); }
        if !back.is_null() { list_add(&mut (*back).chain, &mut backlist); }
        if !front.is_null() && !back.is_null() {
            if !(*w).pair.is_null() {
                // split the paired poly, preserve pairing
                let mut front2: *mut winding_t = 0 as *mut winding_t;
                let mut back2: *mut winding_t = 0 as *mut winding_t;
                winding_split((*w).pair, plane, &mut front2, &mut back2);
                (*front2).pair = front;
                (*front).pair = front2;
                (*back2).pair = back;
                (*back).pair = back2;
                list_add(&mut (*front2).chain, &mut (*(*w).pair).chain);
                list_add(&mut (*back2).chain, &mut (*(*w).pair).chain);
                list_del(&mut (*(*w).pair).chain);
                free_winding((*w).pair);
                (*model).num_polys = (*model).num_polys.wrapping_add(1)
            } else {
                (*front).pair = 0 as *mut winding_s;
                (*back).pair = 0 as *mut winding_s
            }
            (*model).num_polys = (*model).num_polys.wrapping_add(1);
            free_winding(w);
        }
        w = next;
        next =
            ((*next).chain.next as
                 *mut libc::c_char).offset(-(&mut (*(0 as
                                                         *mut winding_t)).chain
                                                 as *mut hullnode_t as size_t
                                                 as isize)) as *mut winding_t
    }
    w = winding_for_plane(plane);
    i = 0 as libc::c_int;
    while !w.is_null() && (i as libc::c_uint) < node_stack_depth {
        let mut p: *mut mplane_t =
            (*hull).planes.offset((*node_stack[i as usize]).planenum as
                                      isize);
        w =
            winding_clip(w, p, false_0, side_stack[i as usize],
                         0.00001f64 as vec_t);
        i += 1
    }
    if !w.is_null() {
        let mut tmp: *mut winding_t = winding_copy(w);
        winding_reverse(tmp);
        (*w).pair = tmp;
        (*tmp).pair = w;
        list_add(&mut (*w).chain, &mut frontlist);
        list_add(&mut (*tmp).chain, &mut backlist);
        // PARANIOA - PAIR CHECK
        if !((*w).pair.is_null() || (*(*w).pair).pair == w) {
            Sys_Error(b"assert failed at %s:%i\n\x00" as *const u8 as
                          *const libc::c_char,
                      b"../engine/client/mod_dbghulls.c\x00" as *const u8 as
                          *const libc::c_char, 618 as libc::c_int);
        }
        (*model).num_polys =
            ((*model).num_polys as
                 libc::c_uint).wrapping_add(2 as libc::c_int as libc::c_uint)
                as uint as uint
    } else {
        Con_Printf(b"^3Warning:^7 new winding was clipped away!\n\x00" as
                       *const u8 as *const libc::c_char);
    }
    do_hull_recursion(hull, node, 0 as libc::c_int, &mut frontlist, model);
    do_hull_recursion(hull, node, 1 as libc::c_int, &mut backlist, model);
}
unsafe extern "C" fn remove_paired_polys(mut model: *mut hull_model_t) {
    let mut w: *mut winding_t = 0 as *mut winding_t;
    let mut next: *mut winding_t = 0 as *mut winding_t;
    w =
        ((*model).polys.next as
             *mut libc::c_char).offset(-(&mut (*(0 as *mut winding_t)).chain
                                             as *mut hullnode_t as size_t as
                                             isize)) as *mut winding_t;
    next =
        ((*w).chain.next as
             *mut libc::c_char).offset(-(&mut (*(0 as *mut winding_t)).chain
                                             as *mut hullnode_t as size_t as
                                             isize)) as *mut winding_t;
    while &mut (*w).chain as *mut hullnode_t !=
              &mut (*model).polys as *mut hullnode_t {
        if !(*w).pair.is_null() {
            list_del(&mut (*w).chain);
            free_winding(w);
            (*model).num_polys = (*model).num_polys.wrapping_sub(1)
        }
        w = next;
        next =
            ((*next).chain.next as
                 *mut libc::c_char).offset(-(&mut (*(0 as
                                                         *mut winding_t)).chain
                                                 as *mut hullnode_t as size_t
                                                 as isize)) as *mut winding_t
    };
}
unsafe extern "C" fn make_hull_windings(mut hull: *mut hull_t,
                                        mut model: *mut hull_model_t) {
    let mut head: hullnode_t =
        hullnode_t{next: 0 as *mut hullnode_s, prev: 0 as *mut hullnode_s,};
    head =
        {
            let mut init = hullnode_s{next: &mut head, prev: &mut head,};
            init
        };
    Con_Reportf(b"%i clipnodes...\n\x00" as *const u8 as *const libc::c_char,
                (*hull).lastclipnode - (*hull).firstclipnode);
    node_stack_depth = 0 as libc::c_int as uint;
    (*model).num_polys = 0 as libc::c_int as uint;
    if !(*hull).planes.is_null() {
        hull_windings_r(hull,
                        (*hull).clipnodes.offset((*hull).firstclipnode as
                                                     isize), &mut head,
                        model);
        remove_paired_polys(model);
    }
    Con_Reportf(b"%i hull polys\n\x00" as *const u8 as *const libc::c_char,
                (*model).num_polys);
}
#[no_mangle]
pub unsafe extern "C" fn Mod_InitDebugHulls() {
    let mut i: libc::c_int = 0;
    world.hull_models =
        _Mem_Alloc((*loadmodel).mempool,
                   (::std::mem::size_of::<hull_model_t>() as
                        libc::c_ulong).wrapping_mul((*loadmodel).numsubmodels
                                                        as libc::c_ulong),
                   true_0,
                   b"../engine/client/mod_dbghulls.c\x00" as *const u8 as
                       *const libc::c_char, 666 as libc::c_int) as
            *mut hull_model_t;
    world.num_hull_models = (*loadmodel).numsubmodels;
    // initialize list
    i = 0 as libc::c_int;
    while i < world.num_hull_models {
        let mut poly: *mut hullnode_t =
            &mut (*world.hull_models.offset(i as isize)).polys;
        (*poly).next = poly;
        (*poly).prev = poly;
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn Mod_CreatePolygonsForHull(mut hullnum: libc::c_int) {
    let mut mod_0: *mut model_t = cl.worldmodel;
    let mut start: libc::c_double = 0.;
    let mut end: libc::c_double = 0.;
    let mut name: [libc::c_char; 8] = [0; 8];
    let mut i: libc::c_int = 0;
    if hullnum < 1 as libc::c_int || hullnum > 3 as libc::c_int { return }
    Con_Printf(b"generating polygons for hull %u...\n\x00" as *const u8 as
                   *const libc::c_char, hullnum);
    start = Sys_DoubleTime();
    // rebuild hulls list
    i = 0 as libc::c_int;
    while i < world.num_hull_models {
        let mut model: *mut hull_model_t =
            &mut *world.hull_models.offset(i as isize) as *mut hull_model_t;
        free_hull_polys(&mut (*model).polys);
        make_hull_windings(&mut *(*mod_0).hulls.as_mut_ptr().offset(hullnum as
                                                                        isize),
                           model);
        Q_snprintf(name.as_mut_ptr(),
                   ::std::mem::size_of::<[libc::c_char; 8]>() as
                       libc::c_ulong,
                   b"*%i\x00" as *const u8 as *const libc::c_char,
                   i + 1 as libc::c_int);
        mod_0 = Mod_FindName(name.as_mut_ptr(), false_0);
        i += 1
    }
    end = Sys_DoubleTime();
    Con_Printf(b"build time %.3f secs\n\x00" as *const u8 as
                   *const libc::c_char, end - start);
}
#[no_mangle]
pub unsafe extern "C" fn Mod_ReleaseHullPolygons() {
    let mut i: libc::c_int = 0;
    // release ploygons
    i = 0 as libc::c_int;
    while i < world.num_hull_models {
        let mut model: *mut hull_model_t =
            &mut *world.hull_models.offset(i as isize) as *mut hull_model_t;
        free_hull_polys(&mut (*model).polys);
        i += 1
    }
    world.num_hull_models = 0 as libc::c_int;
}
