#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type file_s;
    pub type grasshdr_s;
    #[no_mangle]
    fn atan2f(_: libc::c_float, _: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn sqrtf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn fabsf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    fn COM_RandomLong(lMin: libc::c_int, lMax: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn SV_StartSound(ent: *mut edict_t, chan: libc::c_int,
                     sample: *const libc::c_char, vol: libc::c_float,
                     attn: libc::c_float, flags: libc::c_int,
                     pitch: libc::c_int);
    #[no_mangle]
    static mut vec3_origin: vec3_t;
    #[no_mangle]
    fn SinCos(radians: libc::c_float, sine: *mut libc::c_float,
              cosine: *mut libc::c_float);
    #[no_mangle]
    fn anglemod(a: libc::c_float) -> libc::c_float;
    #[no_mangle]
    static mut svs: server_static_t;
    #[no_mangle]
    static mut sv: server_t;
    #[no_mangle]
    static mut svgame: svgame_static_t;
    #[no_mangle]
    fn SV_LinkEdict(ent: *mut edict_t, touch_triggers: qboolean);
    #[no_mangle]
    fn SV_Move(start: *const vec_t, mins: *mut vec_t, maxs: *mut vec_t,
               end: *const vec_t, type_0: libc::c_int, e: *mut edict_t,
               monsterclip: qboolean) -> trace_t;
    #[no_mangle]
    fn SV_MoveNoEnts(start: *const vec_t, mins: *mut vec_t, maxs: *mut vec_t,
                     end: *const vec_t, type_0: libc::c_int, e: *mut edict_t)
     -> trace_t;
    #[no_mangle]
    fn SV_PointContents(p: *const vec_t) -> libc::c_int;
    #[no_mangle]
    fn SV_CheckEdict(e: *const edict_t, file: *const libc::c_char,
                     line: libc::c_int) -> qboolean;
}
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type size_t = libc::c_ulong;
pub type HANDLE = *mut libc::c_void;
pub type HINSTANCE = *mut libc::c_void;
pub type uint = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
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
pub type string_t = libc::c_int;
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
pub struct link_s {
    pub prev: *mut link_s,
    pub next: *mut link_s,
}
pub type link_t = link_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct edict_s {
    pub free: qboolean,
    pub serialnumber: libc::c_int,
    pub area: link_t,
    pub headnode: libc::c_int,
    pub num_leafs: libc::c_int,
    pub leafnums: [libc::c_short; 48],
    pub freetime: libc::c_float,
    pub pvPrivateData: *mut libc::c_void,
    pub v: entvars_t,
}
pub type entvars_t = entvars_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct entvars_s {
    pub classname: string_t,
    pub globalname: string_t,
    pub origin: vec3_t,
    pub oldorigin: vec3_t,
    pub velocity: vec3_t,
    pub basevelocity: vec3_t,
    pub clbasevelocity: vec3_t,
    pub movedir: vec3_t,
    pub angles: vec3_t,
    pub avelocity: vec3_t,
    pub punchangle: vec3_t,
    pub v_angle: vec3_t,
    pub endpos: vec3_t,
    pub startpos: vec3_t,
    pub impacttime: libc::c_float,
    pub starttime: libc::c_float,
    pub fixangle: libc::c_int,
    pub idealpitch: libc::c_float,
    pub pitch_speed: libc::c_float,
    pub ideal_yaw: libc::c_float,
    pub yaw_speed: libc::c_float,
    pub modelindex: libc::c_int,
    pub model: string_t,
    pub viewmodel: libc::c_int,
    pub weaponmodel: libc::c_int,
    pub absmin: vec3_t,
    pub absmax: vec3_t,
    pub mins: vec3_t,
    pub maxs: vec3_t,
    pub size: vec3_t,
    pub ltime: libc::c_float,
    pub nextthink: libc::c_float,
    pub movetype: libc::c_int,
    pub solid: libc::c_int,
    pub skin: libc::c_int,
    pub body: libc::c_int,
    pub effects: libc::c_int,
    pub gravity: libc::c_float,
    pub friction: libc::c_float,
    pub light_level: libc::c_int,
    pub sequence: libc::c_int,
    pub gaitsequence: libc::c_int,
    pub frame: libc::c_float,
    pub animtime: libc::c_float,
    pub framerate: libc::c_float,
    pub controller: [byte; 4],
    pub blending: [byte; 2],
    pub scale: libc::c_float,
    pub rendermode: libc::c_int,
    pub renderamt: libc::c_float,
    pub rendercolor: vec3_t,
    pub renderfx: libc::c_int,
    pub health: libc::c_float,
    pub frags: libc::c_float,
    pub weapons: libc::c_int,
    pub takedamage: libc::c_float,
    pub deadflag: libc::c_int,
    pub view_ofs: vec3_t,
    pub button: libc::c_int,
    pub impulse: libc::c_int,
    pub chain: *mut edict_t,
    pub dmg_inflictor: *mut edict_t,
    pub enemy: *mut edict_t,
    pub aiment: *mut edict_t,
    pub owner: *mut edict_t,
    pub groundentity: *mut edict_t,
    pub spawnflags: libc::c_int,
    pub flags: libc::c_int,
    pub colormap: libc::c_int,
    pub team: libc::c_int,
    pub max_health: libc::c_float,
    pub teleport_time: libc::c_float,
    pub armortype: libc::c_float,
    pub armorvalue: libc::c_float,
    pub waterlevel: libc::c_int,
    pub watertype: libc::c_int,
    pub target: string_t,
    pub targetname: string_t,
    pub netname: string_t,
    pub message: string_t,
    pub dmg_take: libc::c_float,
    pub dmg_save: libc::c_float,
    pub dmg: libc::c_float,
    pub dmgtime: libc::c_float,
    pub noise: string_t,
    pub noise1: string_t,
    pub noise2: string_t,
    pub noise3: string_t,
    pub speed: libc::c_float,
    pub air_finished: libc::c_float,
    pub pain_finished: libc::c_float,
    pub radsuit_finished: libc::c_float,
    pub pContainingEntity: *mut edict_t,
    pub playerclass: libc::c_int,
    pub maxspeed: libc::c_float,
    pub fov: libc::c_float,
    pub weaponanim: libc::c_int,
    pub pushmsec: libc::c_int,
    pub bInDuck: libc::c_int,
    pub flTimeStepSound: libc::c_int,
    pub flSwimTime: libc::c_int,
    pub flDuckTime: libc::c_int,
    pub iStepLeft: libc::c_int,
    pub flFallVelocity: libc::c_float,
    pub gamestate: libc::c_int,
    pub oldbuttons: libc::c_int,
    pub groupinfo: libc::c_int,
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
    pub euser1: *mut edict_t,
    pub euser2: *mut edict_t,
    pub euser3: *mut edict_t,
    pub euser4: *mut edict_t,
}
pub type edict_t = edict_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plane_t {
    pub normal: vec3_t,
    pub dist: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct trace_t {
    pub allsolid: qboolean,
    pub startsolid: qboolean,
    pub inopen: qboolean,
    pub inwater: qboolean,
    pub fraction: libc::c_float,
    pub endpos: vec3_t,
    pub plane: plane_t,
    pub ent: *mut edict_t,
    pub hitgroup: libc::c_int,
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
pub type host_status_t = libc::c_uint;
pub const HOST_CRASHED: host_status_t = 6;
pub const HOST_NOFOCUS: host_status_t = 5;
pub const HOST_SLEEP: host_status_t = 4;
pub const HOST_ERR_FATAL: host_status_t = 3;
pub const HOST_SHUTDOWN: host_status_t = 2;
pub const HOST_FRAME: host_status_t = 1;
pub const HOST_INIT: host_status_t = 0;
pub type host_state_t = libc::c_uint;
pub const STATE_GAME_SHUTDOWN: host_state_t = 4;
pub const STATE_CHANGELEVEL: host_state_t = 3;
pub const STATE_LOAD_GAME: host_state_t = 2;
pub const STATE_LOAD_LEVEL: host_state_t = 1;
pub const STATE_RUNFRAME: host_state_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct game_status_t {
    pub curstate: host_state_t,
    pub nextstate: host_state_t,
    pub levelName: [libc::c_char; 64],
    pub landmarkName: [libc::c_char; 64],
    pub backgroundMap: qboolean,
    pub loadGame: qboolean,
    pub newGame: qboolean,
}
pub type rdtype_t = libc::c_uint;
pub const RD_PACKET: rdtype_t = 2;
pub const RD_CLIENT: rdtype_t = 1;
pub const RD_NONE: rdtype_t = 0;
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
pub struct host_redirect_s {
    pub target: rdtype_t,
    pub buffer: *mut libc::c_char,
    pub buffersize: size_t,
    pub address: netadr_t,
    pub flush: Option<unsafe extern "C" fn(_: netadr_t, _: rdtype_t,
                                           _: *mut libc::c_char) -> ()>,
    pub lines: libc::c_int,
}
pub type host_redirect_t = host_redirect_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct host_parm_s {
    pub hInst: HINSTANCE,
    pub hMutex: HANDLE,
    pub status: host_status_t,
    pub game: game_status_t,
    pub type_0: uint,
    pub abortframe: jmp_buf,
    pub errorframe: dword,
    pub mempool: poolhandle_t,
    pub finalmsg: string,
    pub downloadfile: string,
    pub downloadcount: libc::c_int,
    pub deferred_cmd: [libc::c_char; 128],
    pub rd: host_redirect_t,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
    pub realtime: libc::c_double,
    pub frametime: libc::c_double,
    pub realframetime: libc::c_double,
    pub framecount: uint,
    pub draw_decals: [[libc::c_char; 64]; 512],
    pub player_mins: [vec3_t; 4],
    pub player_maxs: [vec3_t; 4],
    pub hWnd: *mut libc::c_void,
    pub allow_console: qboolean,
    pub allow_console_init: qboolean,
    pub key_overstrike: qboolean,
    pub stuffcmds_pending: qboolean,
    pub allow_cheats: qboolean,
    pub con_showalways: qboolean,
    pub change_game: qboolean,
    pub mouse_visible: qboolean,
    pub shutdown_issued: qboolean,
    pub force_draw_version: qboolean,
    pub force_draw_version_time: libc::c_float,
    pub apply_game_config: qboolean,
    pub apply_opengl_config: qboolean,
    pub config_executed: qboolean,
    pub sv_cvars_restored: libc::c_int,
    pub crashed: qboolean,
    pub daemonized: qboolean,
    pub enabledll: qboolean,
    pub textmode: qboolean,
    pub userinfo_changed: qboolean,
    pub movevars_changed: qboolean,
    pub renderinfo_changed: qboolean,
    pub rootdir: [libc::c_char; 260],
    pub rodir: [libc::c_char; 260],
    pub gamefolder: [libc::c_char; 64],
    pub imagepool: poolhandle_t,
    pub soundpool: poolhandle_t,
    pub features: uint,
    pub window_center_x: libc::c_int,
    pub window_center_y: libc::c_int,
    pub decalList: *mut decallist_s,
    pub numdecals: libc::c_int,
}
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
pub type modelstate_t = modelstate_s;
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
pub type host_parm_t = host_parm_s;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sv_client_s {
    pub state: cl_state_t,
    pub upstate: cl_upload_t,
    pub name: [libc::c_char; 32],
    pub flags: uint,
    pub crcValue: CRC32_t,
    pub userinfo: [libc::c_char; 256],
    pub physinfo: [libc::c_char; 256],
    pub netchan: netchan_t,
    pub chokecount: libc::c_int,
    pub delta_sequence: libc::c_int,
    pub next_messagetime: libc::c_double,
    pub next_checkpingtime: libc::c_double,
    pub next_sendinfotime: libc::c_double,
    pub cl_updaterate: libc::c_double,
    pub timebase: libc::c_double,
    pub connection_started: libc::c_double,
    pub hashedcdkey: [libc::c_char; 34],
    pub customdata: customization_t,
    pub resourcesonhand: resource_t,
    pub resourcesneeded: resource_t,
    pub lastcmd: usercmd_t,
    pub connecttime: libc::c_double,
    pub cmdtime: libc::c_double,
    pub ignorecmdtime: libc::c_double,
    pub packet_loss: libc::c_int,
    pub latency: libc::c_float,
    pub ignored_ents: libc::c_int,
    pub edict: *mut edict_t,
    pub pViewEntity: *mut edict_t,
    pub viewentity: [*mut edict_t; 128],
    pub num_viewents: libc::c_int,
    pub m_bLoopback: qboolean,
    pub listeners: uint,
    pub datagram: sizebuf_t,
    pub datagram_buf: [byte; 16384],
    pub frames: *mut client_frame_t,
    pub events: event_state_t,
    pub challenge: libc::c_int,
    pub userid: libc::c_int,
    pub extensions: libc::c_int,
    pub useragent: [libc::c_char; 256],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client_frame_t {
    pub senttime: libc::c_double,
    pub ping_time: libc::c_float,
    pub clientdata: clientdata_t,
    pub weapondata: [weapon_data_t; 64],
    pub num_entities: libc::c_int,
    pub first_entity: libc::c_int,
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
pub type sizebuf_t = sizebuf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sizebuf_s {
    pub bOverflow: qboolean,
    pub pDebugName: *const libc::c_char,
    pub pData: *mut byte,
    pub iCurBit: libc::c_int,
    pub nDataBits: libc::c_int,
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
pub type CRC32_t = libc::c_uint;
pub type cl_upload_t = libc::c_uint;
pub const us_complete: cl_upload_t = 2;
pub const us_processing: cl_upload_t = 1;
pub const us_inactive: cl_upload_t = 0;
pub type cl_state_t = libc::c_uint;
pub const cs_spawned: cl_state_t = 3;
pub const cs_connected: cl_state_t = 2;
pub const cs_zombie: cl_state_t = 1;
pub const cs_free: cl_state_t = 0;
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
pub struct globalvars_t {
    pub time: libc::c_float,
    pub frametime: libc::c_float,
    pub force_retouch: libc::c_float,
    pub mapname: string_t,
    pub startspot: string_t,
    pub deathmatch: libc::c_float,
    pub coop: libc::c_float,
    pub teamplay: libc::c_float,
    pub serverflags: libc::c_float,
    pub found_secrets: libc::c_float,
    pub v_forward: vec3_t,
    pub v_up: vec3_t,
    pub v_right: vec3_t,
    pub trace_allsolid: libc::c_float,
    pub trace_startsolid: libc::c_float,
    pub trace_fraction: libc::c_float,
    pub trace_endpos: vec3_t,
    pub trace_plane_normal: vec3_t,
    pub trace_plane_dist: libc::c_float,
    pub trace_ent: *mut edict_t,
    pub trace_inopen: libc::c_float,
    pub trace_inwater: libc::c_float,
    pub trace_hitgroup: libc::c_int,
    pub trace_flags: libc::c_int,
    pub changelevel: libc::c_int,
    pub cdAudioTrack: libc::c_int,
    pub maxClients: libc::c_int,
    pub maxEntities: libc::c_int,
    pub pStringBase: *const libc::c_char,
    pub pSaveData: *mut libc::c_void,
    pub vecLandmarkOffset: vec3_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct KeyValueData_s {
    pub szClassName: *mut libc::c_char,
    pub szKeyName: *mut libc::c_char,
    pub szValue: *mut libc::c_char,
    pub fHandled: libc::c_int,
}
pub type KeyValueData = KeyValueData_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LEVELLIST {
    pub mapName: [libc::c_char; 32],
    pub landmarkName: [libc::c_char; 32],
    pub pentLandmark: *mut edict_t,
    pub vecLandmarkOrigin: vec3_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ENTITYTABLE {
    pub id: libc::c_int,
    pub pent: *mut edict_t,
    pub location: libc::c_int,
    pub size: libc::c_int,
    pub flags: libc::c_int,
    pub classname: string_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct saverestore_s {
    pub pBaseData: *mut libc::c_char,
    pub pCurrentData: *mut libc::c_char,
    pub size: libc::c_int,
    pub bufferSize: libc::c_int,
    pub tokenSize: libc::c_int,
    pub tokenCount: libc::c_int,
    pub pTokens: *mut *mut libc::c_char,
    pub currentIndex: libc::c_int,
    pub tableCount: libc::c_int,
    pub connectionCount: libc::c_int,
    pub pTable: *mut ENTITYTABLE,
    pub levelList: [LEVELLIST; 16],
    pub fUseLandmark: libc::c_int,
    pub szLandmarkName: [libc::c_char; 20],
    pub vecLandmarkOffset: vec3_t,
    pub time: libc::c_float,
    pub szCurrentMapName: [libc::c_char; 32],
}
pub type SAVERESTOREDATA = saverestore_s;
pub type _fieldtypes = libc::c_uint;
pub const FIELD_TYPECOUNT: _fieldtypes = 18;
pub const FIELD_SOUNDNAME: _fieldtypes = 17;
pub const FIELD_MODELNAME: _fieldtypes = 16;
pub const FIELD_TIME: _fieldtypes = 15;
pub const FIELD_CHARACTER: _fieldtypes = 14;
pub const FIELD_SHORT: _fieldtypes = 13;
pub const FIELD_BOOLEAN: _fieldtypes = 12;
pub const FIELD_FUNCTION: _fieldtypes = 11;
pub const FIELD_INTEGER: _fieldtypes = 10;
pub const FIELD_POINTER: _fieldtypes = 9;
pub const FIELD_POSITION_VECTOR: _fieldtypes = 8;
pub const FIELD_VECTOR: _fieldtypes = 7;
pub const FIELD_EDICT: _fieldtypes = 6;
pub const FIELD_EVARS: _fieldtypes = 5;
pub const FIELD_EHANDLE: _fieldtypes = 4;
pub const FIELD_CLASSPTR: _fieldtypes = 3;
pub const FIELD_ENTITY: _fieldtypes = 2;
pub const FIELD_STRING: _fieldtypes = 1;
pub const FIELD_FLOAT: _fieldtypes = 0;
pub type FIELDTYPE = _fieldtypes;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TYPEDESCRIPTION {
    pub fieldType: FIELDTYPE,
    pub fieldName: *const libc::c_char,
    pub fieldOffset: libc::c_int,
    pub fieldSize: libc::c_short,
    pub flags: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct playermove_s {
    pub player_index: libc::c_int,
    pub server: qboolean,
    pub multiplayer: qboolean,
    pub time: libc::c_float,
    pub frametime: libc::c_float,
    pub forward: vec3_t,
    pub right: vec3_t,
    pub up: vec3_t,
    pub origin: vec3_t,
    pub angles: vec3_t,
    pub oldangles: vec3_t,
    pub velocity: vec3_t,
    pub movedir: vec3_t,
    pub basevelocity: vec3_t,
    pub view_ofs: vec3_t,
    pub flDuckTime: libc::c_float,
    pub bInDuck: qboolean,
    pub flTimeStepSound: libc::c_int,
    pub iStepLeft: libc::c_int,
    pub flFallVelocity: libc::c_float,
    pub punchangle: vec3_t,
    pub flSwimTime: libc::c_float,
    pub flNextPrimaryAttack: libc::c_float,
    pub effects: libc::c_int,
    pub flags: libc::c_int,
    pub usehull: libc::c_int,
    pub gravity: libc::c_float,
    pub friction: libc::c_float,
    pub oldbuttons: libc::c_int,
    pub waterjumptime: libc::c_float,
    pub dead: qboolean,
    pub deadflag: libc::c_int,
    pub spectator: libc::c_int,
    pub movetype: libc::c_int,
    pub onground: libc::c_int,
    pub waterlevel: libc::c_int,
    pub watertype: libc::c_int,
    pub oldwaterlevel: libc::c_int,
    pub sztexturename: [libc::c_char; 256],
    pub chtexturetype: libc::c_char,
    pub maxspeed: libc::c_float,
    pub clientmaxspeed: libc::c_float,
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
    pub numphysent: libc::c_int,
    pub physents: [physent_t; 600],
    pub nummoveent: libc::c_int,
    pub moveents: [physent_t; 64],
    pub numvisent: libc::c_int,
    pub visents: [physent_t; 600],
    pub cmd: usercmd_t,
    pub numtouch: libc::c_int,
    pub touchindex: [pmtrace_t; 600],
    pub physinfo: [libc::c_char; 256],
    pub movevars: *mut movevars_s,
    pub player_mins: [vec3_t; 4],
    pub player_maxs: [vec3_t; 4],
    pub PM_Info_ValueForKey: Option<unsafe extern "C" fn(_:
                                                             *const libc::c_char,
                                                         _:
                                                             *const libc::c_char)
                                        -> *const libc::c_char>,
    pub PM_Particle: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                 _: libc::c_int,
                                                 _: libc::c_float,
                                                 _: libc::c_int,
                                                 _: libc::c_int) -> ()>,
    pub PM_TestPlayerPosition: Option<unsafe extern "C" fn(_:
                                                               *mut libc::c_float,
                                                           _: *mut pmtrace_t)
                                          -> libc::c_int>,
    pub Con_NPrintf: Option<unsafe extern "C" fn(_: libc::c_int,
                                                 _: *const libc::c_char,
                                                 _: ...) -> ()>,
    pub Con_DPrintf: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                 _: ...) -> ()>,
    pub Con_Printf: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                _: ...) -> ()>,
    pub Sys_FloatTime: Option<unsafe extern "C" fn() -> libc::c_double>,
    pub PM_StuckTouch: Option<unsafe extern "C" fn(_: libc::c_int,
                                                   _: *mut pmtrace_t) -> ()>,
    pub PM_PointContents: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                      _: *mut libc::c_int)
                                     -> libc::c_int>,
    pub PM_TruePointContents: Option<unsafe extern "C" fn(_:
                                                              *mut libc::c_float)
                                         -> libc::c_int>,
    pub PM_HullPointContents: Option<unsafe extern "C" fn(_: *mut hull_s,
                                                          _: libc::c_int,
                                                          _:
                                                              *mut libc::c_float)
                                         -> libc::c_int>,
    pub PM_PlayerTrace: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                    _: *mut libc::c_float,
                                                    _: libc::c_int,
                                                    _: libc::c_int)
                                   -> pmtrace_t>,
    pub PM_TraceLine: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                  _: *mut libc::c_float,
                                                  _: libc::c_int,
                                                  _: libc::c_int,
                                                  _: libc::c_int)
                                 -> *mut pmtrace_s>,
    pub RandomLong: Option<unsafe extern "C" fn(_: libc::c_int,
                                                _: libc::c_int)
                               -> libc::c_int>,
    pub RandomFloat: Option<unsafe extern "C" fn(_: libc::c_float,
                                                 _: libc::c_float)
                                -> libc::c_float>,
    pub PM_GetModelType: Option<unsafe extern "C" fn(_: *mut model_s)
                                    -> libc::c_int>,
    pub PM_GetModelBounds: Option<unsafe extern "C" fn(_: *mut model_s,
                                                       _: *mut libc::c_float,
                                                       _: *mut libc::c_float)
                                      -> ()>,
    pub PM_HullForBsp: Option<unsafe extern "C" fn(_: *mut physent_t,
                                                   _: *mut libc::c_float)
                                  -> *mut libc::c_void>,
    pub PM_TraceModel: Option<unsafe extern "C" fn(_: *mut physent_t,
                                                   _: *mut libc::c_float,
                                                   _: *mut libc::c_float,
                                                   _: *mut trace_t)
                                  -> libc::c_float>,
    pub COM_FileSize: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                 -> libc::c_int>,
    pub COM_LoadFile: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                  _: libc::c_int,
                                                  _: *mut libc::c_int)
                                 -> *mut byte>,
    pub COM_FreeFile: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                 -> ()>,
    pub memfgets: Option<unsafe extern "C" fn(_: *mut byte, _: libc::c_int,
                                              _: *mut libc::c_int,
                                              _: *mut libc::c_char,
                                              _: libc::c_int)
                             -> *mut libc::c_char>,
    pub runfuncs: qboolean,
    pub PM_PlaySound: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _: *const libc::c_char,
                                                  _: libc::c_float,
                                                  _: libc::c_float,
                                                  _: libc::c_int,
                                                  _: libc::c_int) -> ()>,
    pub PM_TraceTexture: Option<unsafe extern "C" fn(_: libc::c_int,
                                                     _: *mut libc::c_float,
                                                     _: *mut libc::c_float)
                                    -> *const libc::c_char>,
    pub PM_PlaybackEventFull: Option<unsafe extern "C" fn(_: libc::c_int,
                                                          _: libc::c_int,
                                                          _: libc::c_ushort,
                                                          _: libc::c_float,
                                                          _:
                                                              *mut libc::c_float,
                                                          _:
                                                              *mut libc::c_float,
                                                          _: libc::c_float,
                                                          _: libc::c_float,
                                                          _: libc::c_int,
                                                          _: libc::c_int,
                                                          _: libc::c_int,
                                                          _: libc::c_int)
                                         -> ()>,
    pub PM_PlayerTraceEx: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                      _: *mut libc::c_float,
                                                      _: libc::c_int,
                                                      _:
                                                          Option<unsafe extern "C" fn(_:
                                                                                          *mut physent_t)
                                                                     ->
                                                                         libc::c_int>)
                                     -> pmtrace_t>,
    pub PM_TestPlayerPositionEx: Option<unsafe extern "C" fn(_:
                                                                 *mut libc::c_float,
                                                             _:
                                                                 *mut pmtrace_t,
                                                             _:
                                                                 Option<unsafe extern "C" fn(_:
                                                                                                 *mut physent_t)
                                                                            ->
                                                                                libc::c_int>)
                                            -> libc::c_int>,
    pub PM_TraceLineEx: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                    _: *mut libc::c_float,
                                                    _: libc::c_int,
                                                    _: libc::c_int,
                                                    _:
                                                        Option<unsafe extern "C" fn(_:
                                                                                        *mut physent_t)
                                                                   ->
                                                                       libc::c_int>)
                                   -> *mut pmtrace_s>,
    pub PM_TraceSurface: Option<unsafe extern "C" fn(_: libc::c_int,
                                                     _: *mut libc::c_float,
                                                     _: *mut libc::c_float)
                                    -> *mut msurface_s>,
}
pub type physent_t = physent_s;
pub type pmtrace_t = pmtrace_s;
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
pub struct DLL_FUNCTIONS {
    pub pfnGameInit: Option<unsafe extern "C" fn() -> ()>,
    pub pfnSpawn: Option<unsafe extern "C" fn(_: *mut edict_t)
                             -> libc::c_int>,
    pub pfnThink: Option<unsafe extern "C" fn(_: *mut edict_t) -> ()>,
    pub pfnUse: Option<unsafe extern "C" fn(_: *mut edict_t, _: *mut edict_t)
                           -> ()>,
    pub pfnTouch: Option<unsafe extern "C" fn(_: *mut edict_t,
                                              _: *mut edict_t) -> ()>,
    pub pfnBlocked: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                _: *mut edict_t) -> ()>,
    pub pfnKeyValue: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                 _: *mut KeyValueData) -> ()>,
    pub pfnSave: Option<unsafe extern "C" fn(_: *mut edict_t,
                                             _: *mut SAVERESTOREDATA) -> ()>,
    pub pfnRestore: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                _: *mut SAVERESTOREDATA,
                                                _: libc::c_int)
                               -> libc::c_int>,
    pub pfnSetAbsBox: Option<unsafe extern "C" fn(_: *mut edict_t) -> ()>,
    pub pfnSaveWriteFields: Option<unsafe extern "C" fn(_:
                                                            *mut SAVERESTOREDATA,
                                                        _:
                                                            *const libc::c_char,
                                                        _: *mut libc::c_void,
                                                        _:
                                                            *mut TYPEDESCRIPTION,
                                                        _: libc::c_int)
                                       -> ()>,
    pub pfnSaveReadFields: Option<unsafe extern "C" fn(_:
                                                           *mut SAVERESTOREDATA,
                                                       _: *const libc::c_char,
                                                       _: *mut libc::c_void,
                                                       _:
                                                           *mut TYPEDESCRIPTION,
                                                       _: libc::c_int) -> ()>,
    pub pfnSaveGlobalState: Option<unsafe extern "C" fn(_:
                                                            *mut SAVERESTOREDATA)
                                       -> ()>,
    pub pfnRestoreGlobalState: Option<unsafe extern "C" fn(_:
                                                               *mut SAVERESTOREDATA)
                                          -> ()>,
    pub pfnResetGlobalState: Option<unsafe extern "C" fn() -> ()>,
    pub pfnClientConnect: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                      _: *const libc::c_char,
                                                      _: *const libc::c_char,
                                                      _: *mut libc::c_char)
                                     -> qboolean>,
    pub pfnClientDisconnect: Option<unsafe extern "C" fn(_: *mut edict_t)
                                        -> ()>,
    pub pfnClientKill: Option<unsafe extern "C" fn(_: *mut edict_t) -> ()>,
    pub pfnClientPutInServer: Option<unsafe extern "C" fn(_: *mut edict_t)
                                         -> ()>,
    pub pfnClientCommand: Option<unsafe extern "C" fn(_: *mut edict_t) -> ()>,
    pub pfnClientUserInfoChanged: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                              _:
                                                                  *mut libc::c_char)
                                             -> ()>,
    pub pfnServerActivate: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                       _: libc::c_int,
                                                       _: libc::c_int) -> ()>,
    pub pfnServerDeactivate: Option<unsafe extern "C" fn() -> ()>,
    pub pfnPlayerPreThink: Option<unsafe extern "C" fn(_: *mut edict_t)
                                      -> ()>,
    pub pfnPlayerPostThink: Option<unsafe extern "C" fn(_: *mut edict_t)
                                       -> ()>,
    pub pfnStartFrame: Option<unsafe extern "C" fn() -> ()>,
    pub pfnParmsNewLevel: Option<unsafe extern "C" fn() -> ()>,
    pub pfnParmsChangeLevel: Option<unsafe extern "C" fn() -> ()>,
    pub pfnGetGameDescription: Option<unsafe extern "C" fn()
                                          -> *const libc::c_char>,
    pub pfnPlayerCustomization: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                            _:
                                                                *mut customization_t)
                                           -> ()>,
    pub pfnSpectatorConnect: Option<unsafe extern "C" fn(_: *mut edict_t)
                                        -> ()>,
    pub pfnSpectatorDisconnect: Option<unsafe extern "C" fn(_: *mut edict_t)
                                           -> ()>,
    pub pfnSpectatorThink: Option<unsafe extern "C" fn(_: *mut edict_t)
                                      -> ()>,
    pub pfnSys_Error: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                 -> ()>,
    pub pfnPM_Move: Option<unsafe extern "C" fn(_: *mut playermove_s,
                                                _: qboolean) -> ()>,
    pub pfnPM_Init: Option<unsafe extern "C" fn(_: *mut playermove_s) -> ()>,
    pub pfnPM_FindTextureType: Option<unsafe extern "C" fn(_:
                                                               *mut libc::c_char)
                                          -> libc::c_char>,
    pub pfnSetupVisibility: Option<unsafe extern "C" fn(_: *mut edict_s,
                                                        _: *mut edict_s,
                                                        _:
                                                            *mut *mut libc::c_uchar,
                                                        _:
                                                            *mut *mut libc::c_uchar)
                                       -> ()>,
    pub pfnUpdateClientData: Option<unsafe extern "C" fn(_: *const edict_s,
                                                         _: libc::c_int,
                                                         _: *mut clientdata_s)
                                        -> ()>,
    pub pfnAddToFullPack: Option<unsafe extern "C" fn(_: *mut entity_state_s,
                                                      _: libc::c_int,
                                                      _: *mut edict_t,
                                                      _: *mut edict_t,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: *mut libc::c_uchar)
                                     -> libc::c_int>,
    pub pfnCreateBaseline: Option<unsafe extern "C" fn(_: libc::c_int,
                                                       _: libc::c_int,
                                                       _: *mut entity_state_s,
                                                       _: *mut edict_s,
                                                       _: libc::c_int,
                                                       _: *mut vec_t,
                                                       _: *mut vec_t) -> ()>,
    pub pfnRegisterEncoders: Option<unsafe extern "C" fn() -> ()>,
    pub pfnGetWeaponData: Option<unsafe extern "C" fn(_: *mut edict_s,
                                                      _: *mut weapon_data_s)
                                     -> libc::c_int>,
    pub pfnCmdStart: Option<unsafe extern "C" fn(_: *const edict_t,
                                                 _: *const usercmd_s,
                                                 _: libc::c_uint) -> ()>,
    pub pfnCmdEnd: Option<unsafe extern "C" fn(_: *const edict_t) -> ()>,
    pub pfnConnectionlessPacket: Option<unsafe extern "C" fn(_:
                                                                 *const netadr_s,
                                                             _:
                                                                 *const libc::c_char,
                                                             _:
                                                                 *mut libc::c_char,
                                                             _:
                                                                 *mut libc::c_int)
                                            -> libc::c_int>,
    pub pfnGetHullBounds: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: *mut libc::c_float,
                                                      _: *mut libc::c_float)
                                     -> libc::c_int>,
    pub pfnCreateInstancedBaselines: Option<unsafe extern "C" fn() -> ()>,
    pub pfnInconsistentFile: Option<unsafe extern "C" fn(_: *const edict_s,
                                                         _:
                                                             *const libc::c_char,
                                                         _: *mut libc::c_char)
                                        -> libc::c_int>,
    pub pfnAllowLagCompensation: Option<unsafe extern "C" fn()
                                            -> libc::c_int>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NEW_DLL_FUNCTIONS {
    pub pfnOnFreeEntPrivateData: Option<unsafe extern "C" fn(_: *mut edict_t)
                                            -> ()>,
    pub pfnGameShutdown: Option<unsafe extern "C" fn() -> ()>,
    pub pfnShouldCollide: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                      _: *mut edict_t)
                                     -> libc::c_int>,
    pub pfnCvarValue: Option<unsafe extern "C" fn(_: *const edict_t,
                                                  _: *const libc::c_char)
                                 -> ()>,
    pub pfnCvarValue2: Option<unsafe extern "C" fn(_: *const edict_t,
                                                   _: libc::c_int,
                                                   _: *const libc::c_char,
                                                   _: *const libc::c_char)
                                  -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct physics_interface_s {
    pub version: libc::c_int,
    pub SV_CreateEntity: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                     _: *const libc::c_char)
                                    -> libc::c_int>,
    pub SV_PhysicsEntity: Option<unsafe extern "C" fn(_: *mut edict_t)
                                     -> libc::c_int>,
    pub SV_LoadEntities: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                     _: *mut libc::c_char)
                                    -> libc::c_int>,
    pub SV_UpdatePlayerBaseVelocity: Option<unsafe extern "C" fn(_:
                                                                     *mut edict_t)
                                                -> ()>,
    pub SV_AllowSaveGame: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub SV_TriggerTouch: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                     _: *mut edict_t)
                                    -> libc::c_int>,
    pub SV_CheckFeatures: Option<unsafe extern "C" fn() -> libc::c_uint>,
    pub DrawDebugTriangles: Option<unsafe extern "C" fn() -> ()>,
    pub DrawNormalTriangles: Option<unsafe extern "C" fn() -> ()>,
    pub DrawOrthoTriangles: Option<unsafe extern "C" fn() -> ()>,
    pub ClipMoveToEntity: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                      _: *const libc::c_float,
                                                      _: *mut libc::c_float,
                                                      _: *mut libc::c_float,
                                                      _: *const libc::c_float,
                                                      _: *mut trace_t) -> ()>,
    pub ClipPMoveToEntity: Option<unsafe extern "C" fn(_: *mut physent_s,
                                                       _:
                                                           *const libc::c_float,
                                                       _: *mut libc::c_float,
                                                       _: *mut libc::c_float,
                                                       _:
                                                           *const libc::c_float,
                                                       _: *mut pmtrace_s)
                                      -> ()>,
    pub SV_EndFrame: Option<unsafe extern "C" fn() -> ()>,
    pub pfnPrepWorldFrame: Option<unsafe extern "C" fn() -> ()>,
    pub pfnCreateEntitiesInRestoreList: Option<unsafe extern "C" fn(_:
                                                                        *mut SAVERESTOREDATA,
                                                                    _:
                                                                        libc::c_int,
                                                                    _:
                                                                        qboolean)
                                                   -> ()>,
    pub pfnAllocString: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                   -> string_t>,
    pub pfnMakeString: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                  -> string_t>,
    pub pfnGetString: Option<unsafe extern "C" fn(_: string_t)
                                 -> *const libc::c_char>,
    pub pfnRestoreDecal: Option<unsafe extern "C" fn(_: *mut decallist_s,
                                                     _: *mut edict_t,
                                                     _: qboolean)
                                    -> libc::c_int>,
    pub PM_PlayerTouch: Option<unsafe extern "C" fn(_: *mut playermove_s,
                                                    _: *mut edict_t) -> ()>,
    pub Mod_ProcessUserData: Option<unsafe extern "C" fn(_: *mut model_s,
                                                         _: qboolean,
                                                         _: *const byte)
                                        -> ()>,
    pub SV_HullForBsp: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                   _: *const libc::c_float,
                                                   _: *const libc::c_float,
                                                   _: *mut libc::c_float)
                                  -> *mut libc::c_void>,
    pub SV_PlayerThink: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                    _: libc::c_float,
                                                    _: libc::c_double)
                                   -> libc::c_int>,
}
pub type physics_interface_t = physics_interface_s;
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
pub type playermove_t = playermove_s;
pub type movevars_t = movevars_s;
pub type sv_state_t = libc::c_uint;
pub const ss_active: sv_state_t = 2;
pub const ss_loading: sv_state_t = 1;
pub const ss_dead: sv_state_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sv_baseline_t {
    pub classname: *const libc::c_char,
    pub baseline: entity_state_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct server_log_t {
    pub active: qboolean,
    pub net_log: qboolean,
    pub net_address: netadr_t,
    pub file: *mut file_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct server_s {
    pub state: sv_state_t,
    pub background: qboolean,
    pub loadgame: qboolean,
    pub time: libc::c_double,
    pub time_residual: libc::c_double,
    pub frametime: libc::c_float,
    pub framecount: libc::c_int,
    pub current_client: *mut sv_client_s,
    pub hostflags: libc::c_int,
    pub worldmapCRC: CRC32_t,
    pub progsCRC: libc::c_int,
    pub name: [libc::c_char; 64],
    pub startspot: [libc::c_char; 64],
    pub lastchecktime: libc::c_double,
    pub lastcheck: libc::c_int,
    pub model_precache: [[libc::c_char; 64]; 1024],
    pub sound_precache: [[libc::c_char; 64]; 2048],
    pub files_precache: [[libc::c_char; 64]; 1024],
    pub event_precache: [[libc::c_char; 64]; 1024],
    pub model_precache_flags: [byte; 1024],
    pub models: [*mut model_t; 1024],
    pub num_static_entities: libc::c_int,
    pub lightstyles: [lightstyle_t; 64],
    pub consistency_list: [consistency_t; 1024],
    pub resources: [resource_t; 5120],
    pub num_consistency: libc::c_int,
    pub num_resources: libc::c_int,
    pub instanced: [sv_baseline_t; 64],
    pub last_valid_baseline: libc::c_int,
    pub num_instanced: libc::c_int,
    pub datagram: sizebuf_t,
    pub datagram_buf: [byte; 16384],
    pub reliable_datagram: sizebuf_t,
    pub reliable_datagram_buf: [byte; 16384],
    pub multicast: sizebuf_t,
    pub multicast_buf: [byte; 8192],
    pub signon: sizebuf_t,
    pub signon_buf: [byte; 131072],
    pub spec_datagram: sizebuf_t,
    pub spectator_buf: [byte; 8192],
    pub worldmodel: *mut model_t,
    pub playersonly: qboolean,
    pub simulating: qboolean,
    pub paused: qboolean,
    pub ignored_static_ents: libc::c_int,
    pub ignored_world_decals: libc::c_int,
    pub static_ents_overflow: libc::c_int,
}
pub type server_t = server_s;
pub type sv_client_t = sv_client_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct challenge_t {
    pub adr: netadr_t,
    pub time: libc::c_double,
    pub challenge: libc::c_int,
    pub connected: qboolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sv_user_message_t {
    pub name: [libc::c_char; 32],
    pub number: libc::c_int,
    pub size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sv_pushed_t {
    pub ent: *mut edict_t,
    pub origin: vec3_t,
    pub angles: vec3_t,
    pub fixangle: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sv_interp_t {
    pub active: qboolean,
    pub moving: qboolean,
    pub firstframe: qboolean,
    pub nointerp: qboolean,
    pub mins: vec3_t,
    pub maxs: vec3_t,
    pub curpos: vec3_t,
    pub oldpos: vec3_t,
    pub newpos: vec3_t,
    pub finalpos: vec3_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct svgame_static_t {
    pub msg_name: *const libc::c_char,
    pub msg: [sv_user_message_t; 197],
    pub msg_size_index: libc::c_int,
    pub msg_realsize: libc::c_int,
    pub msg_index: libc::c_int,
    pub msg_dest: libc::c_int,
    pub msg_started: qboolean,
    pub msg_ent: *mut edict_t,
    pub msg_org: vec3_t,
    pub hInstance: *mut libc::c_void,
    pub config_executed: qboolean,
    pub edicts: *mut edict_t,
    pub numEntities: libc::c_int,
    pub movevars: movevars_t,
    pub oldmovevars: movevars_t,
    pub pmove: *mut playermove_t,
    pub interp: [sv_interp_t; 32],
    pub pushed: [sv_pushed_t; 256],
    pub globals: *mut globalvars_t,
    pub dllFuncs: DLL_FUNCTIONS,
    pub dllFuncs2: NEW_DLL_FUNCTIONS,
    pub physFuncs: physics_interface_t,
    pub mempool: poolhandle_t,
    pub stringspool: poolhandle_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct server_static_t {
    pub initialized: qboolean,
    pub game_library_loaded: qboolean,
    pub timestart: libc::c_double,
    pub maxclients: libc::c_int,
    pub groupmask: libc::c_int,
    pub groupop: libc::c_int,
    pub log: server_log_t,
    pub serverinfo: [libc::c_char; 512],
    pub localinfo: [libc::c_char; 32768],
    pub spawncount: libc::c_int,
    pub clients: *mut sv_client_t,
    pub num_client_entities: libc::c_int,
    pub next_client_entities: libc::c_int,
    pub packet_entities: *mut entity_state_t,
    pub baselines: *mut entity_state_t,
    pub static_entities: *mut entity_state_t,
    pub last_heartbeat: libc::c_double,
    pub challenges: [challenge_t; 1024],
}
#[inline(always)]
unsafe extern "C" fn __tg_sqrt(mut __x: libc::c_float) -> libc::c_float {
    return sqrtf(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_fabs(mut __x: libc::c_float) -> libc::c_float {
    return fabsf(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_atan2(mut __x: libc::c_float,
                                mut __y: libc::c_float) -> libc::c_float {
    return atan2f(__x, __y);
}
// normal move in the direction monster is facing
// moves in direction specified, no matter which way monster is facing
/*
=============
SV_CheckBottom

Returns false if any part of the bottom of the entity is off an edge that
is not a staircase.

=============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_CheckBottom(mut ent: *mut edict_t,
                                        mut iMode: libc::c_int) -> qboolean {
    let mut current_block: u64;
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut start: vec3_t = [0.; 3];
    let mut stop: vec3_t = [0.; 3];
    let mut mid: libc::c_float = 0.;
    let mut bottom: libc::c_float = 0.;
    let mut monsterClip: qboolean = false_0;
    let mut trace: trace_t =
        trace_t{allsolid: false_0,
                startsolid: false_0,
                inopen: false_0,
                inwater: false_0,
                fraction: 0.,
                endpos: [0.; 3],
                plane: plane_t{normal: [0.; 3], dist: 0.,},
                ent: 0 as *mut edict_t,
                hitgroup: 0,};
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    monsterClip =
        if (*ent).v.flags as libc::c_uint &
               (1 as libc::c_uint) << 23 as libc::c_int != 0 {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int } as qboolean;
    mins[0 as libc::c_int as usize] =
        (*ent).v.origin[0 as libc::c_int as usize] +
            (*ent).v.mins[0 as libc::c_int as usize];
    mins[1 as libc::c_int as usize] =
        (*ent).v.origin[1 as libc::c_int as usize] +
            (*ent).v.mins[1 as libc::c_int as usize];
    mins[2 as libc::c_int as usize] =
        (*ent).v.origin[2 as libc::c_int as usize] +
            (*ent).v.mins[2 as libc::c_int as usize];
    maxs[0 as libc::c_int as usize] =
        (*ent).v.origin[0 as libc::c_int as usize] +
            (*ent).v.maxs[0 as libc::c_int as usize];
    maxs[1 as libc::c_int as usize] =
        (*ent).v.origin[1 as libc::c_int as usize] +
            (*ent).v.maxs[1 as libc::c_int as usize];
    maxs[2 as libc::c_int as usize] =
        (*ent).v.origin[2 as libc::c_int as usize] +
            (*ent).v.maxs[2 as libc::c_int as usize];
    // if all of the points under the corners are solid world, don't bother
	// with the tougher checks
	// the corners must be within 16 of the midpoint
    start[2 as libc::c_int as usize] =
        mins[2 as libc::c_int as usize] - 1.0f32; // we got out easy
    x = 0 as libc::c_int;
    's_34:
        loop  {
            if !(x <= 1 as libc::c_int) {
                current_block = 7976072742316086414;
                break ;
            }
            y = 0 as libc::c_int;
            while y <= 1 as libc::c_int {
                start[0 as libc::c_int as usize] =
                    if x != 0 {
                        maxs[0 as libc::c_int as usize]
                    } else { mins[0 as libc::c_int as usize] };
                start[1 as libc::c_int as usize] =
                    if y != 0 {
                        maxs[1 as libc::c_int as usize]
                    } else { mins[1 as libc::c_int as usize] };
                svs.groupmask = (*ent).v.groupinfo;
                if SV_PointContents(start.as_mut_ptr() as *const vec_t) !=
                       -(2 as libc::c_int) {
                    current_block = 13377744963774632122;
                    break 's_34 ;
                }
                y += 1
            }
            x += 1
        }
    match current_block {
        7976072742316086414 => { return true_0 }
        _ => {
            // check it for real...
            start[2 as libc::c_int as usize] =
                mins[2 as libc::c_int as usize];
            if host.features &
                   ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint ==
                   0 {
                start[2 as libc::c_int as usize] += svgame.movevars.stepsize
            }
            // the midpoint must be within 16 of the bottom
            stop[0 as libc::c_int as usize] =
                (mins[0 as libc::c_int as usize] +
                     maxs[0 as libc::c_int as usize]) * 0.5f32;
            start[0 as libc::c_int as usize] =
                stop[0 as libc::c_int as usize];
            stop[1 as libc::c_int as usize] =
                (mins[1 as libc::c_int as usize] +
                     maxs[1 as libc::c_int as usize]) * 0.5f32;
            start[1 as libc::c_int as usize] =
                stop[1 as libc::c_int as usize];
            stop[2 as libc::c_int as usize] =
                start[2 as libc::c_int as usize] -
                    2.0f32 * svgame.movevars.stepsize;
            if iMode == 1 as libc::c_int {
                trace =
                    SV_MoveNoEnts(start.as_mut_ptr() as *const vec_t,
                                  vec3_origin.as_mut_ptr(),
                                  vec3_origin.as_mut_ptr(),
                                  stop.as_mut_ptr() as *const vec_t,
                                  1 as libc::c_int, ent)
            } else {
                trace =
                    SV_Move(start.as_mut_ptr() as *const vec_t,
                            vec3_origin.as_mut_ptr(),
                            vec3_origin.as_mut_ptr(),
                            stop.as_mut_ptr() as *const vec_t,
                            1 as libc::c_int, ent, monsterClip)
            }
            if trace.fraction == 1.0f32 { return false_0 }
            bottom = trace.endpos[2 as libc::c_int as usize];
            mid = bottom;
            // the corners must be within 16 of the midpoint
            x = 0 as libc::c_int;
            while x <= 1 as libc::c_int {
                y = 0 as libc::c_int;
                while y <= 1 as libc::c_int {
                    stop[0 as libc::c_int as usize] =
                        if x != 0 {
                            maxs[0 as libc::c_int as usize]
                        } else { mins[0 as libc::c_int as usize] };
                    start[0 as libc::c_int as usize] =
                        stop[0 as libc::c_int as usize];
                    stop[1 as libc::c_int as usize] =
                        if y != 0 {
                            maxs[1 as libc::c_int as usize]
                        } else { mins[1 as libc::c_int as usize] };
                    start[1 as libc::c_int as usize] =
                        stop[1 as libc::c_int as usize];
                    if iMode == 1 as libc::c_int {
                        trace =
                            SV_MoveNoEnts(start.as_mut_ptr() as *const vec_t,
                                          vec3_origin.as_mut_ptr(),
                                          vec3_origin.as_mut_ptr(),
                                          stop.as_mut_ptr() as *const vec_t,
                                          1 as libc::c_int, ent)
                    } else {
                        trace =
                            SV_Move(start.as_mut_ptr() as *const vec_t,
                                    vec3_origin.as_mut_ptr(),
                                    vec3_origin.as_mut_ptr(),
                                    stop.as_mut_ptr() as *const vec_t,
                                    1 as libc::c_int, ent, monsterClip)
                    }
                    if trace.fraction != 1.0f32 &&
                           trace.endpos[2 as libc::c_int as usize] > bottom {
                        bottom = trace.endpos[2 as libc::c_int as usize]
                    }
                    if trace.fraction == 1.0f32 ||
                           mid - trace.endpos[2 as libc::c_int as usize] >
                               svgame.movevars.stepsize {
                        return false_0
                    }
                    y += 1
                }
                x += 1
            }
            return true_0
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn SV_WaterMove(mut ent: *mut edict_t) {
    let mut drownlevel: libc::c_float = 0.;
    let mut waterlevel: libc::c_int = 0;
    let mut watertype: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    if (*ent).v.movetype == 8 as libc::c_int {
        (*ent).v.air_finished =
            (sv.time + 12.0f32 as libc::c_double) as libc::c_float;
        return
    }
    // no watermove for monsters but pushables
    if (*ent).v.flags as libc::c_uint &
           (1 as libc::c_uint) << 5 as libc::c_int != 0 &&
           (*ent).v.health <= 0.0f32 {
        return
    } // quake1 original code
    drownlevel =
        if (*ent).v.deadflag == 0 as libc::c_int { 3.0f64 } else { 1.0f64 } as
            libc::c_float;
    waterlevel = (*ent).v.waterlevel;
    watertype = (*ent).v.watertype;
    flags = (*ent).v.flags;
    if flags as libc::c_uint &
           ((1 as libc::c_uint) << 17 as libc::c_int |
                (1 as libc::c_uint) << 6 as libc::c_int) == 0 {
        if flags as libc::c_uint & (1 as libc::c_uint) << 1 as libc::c_int !=
               0 && waterlevel as libc::c_float > drownlevel ||
               waterlevel as libc::c_float <= drownlevel {
            if (*ent).v.air_finished as libc::c_double > sv.time &&
                   (*ent).v.pain_finished as libc::c_double > sv.time {
                (*ent).v.dmg += 2 as libc::c_int as libc::c_float;
                if (*ent).v.dmg < 15 as libc::c_int as libc::c_float {
                    (*ent).v.dmg = 10 as libc::c_int as libc::c_float
                }
                (*ent).v.pain_finished =
                    (sv.time + 1.0f32 as libc::c_double) as libc::c_float
            }
        } else {
            (*ent).v.air_finished =
                (sv.time + 12.0f32 as libc::c_double) as libc::c_float;
            (*ent).v.dmg = 2 as libc::c_int as libc::c_float
        }
    }
    if waterlevel == 0 {
        if flags as libc::c_uint & (1 as libc::c_uint) << 4 as libc::c_int !=
               0 {
            // leave the water.
            match COM_RandomLong(0 as libc::c_int, 3 as libc::c_int) {
                0 => {
                    SV_StartSound(ent, 4 as libc::c_int,
                                  b"player/pl_wade1.wav\x00" as *const u8 as
                                      *const libc::c_char, 1.0f32,
                                  0.8f64 as libc::c_float, 0 as libc::c_int,
                                  100 as libc::c_int);
                }
                1 => {
                    SV_StartSound(ent, 4 as libc::c_int,
                                  b"player/pl_wade2.wav\x00" as *const u8 as
                                      *const libc::c_char, 1.0f32,
                                  0.8f64 as libc::c_float, 0 as libc::c_int,
                                  100 as libc::c_int);
                }
                2 => {
                    SV_StartSound(ent, 4 as libc::c_int,
                                  b"player/pl_wade3.wav\x00" as *const u8 as
                                      *const libc::c_char, 1.0f32,
                                  0.8f64 as libc::c_float, 0 as libc::c_int,
                                  100 as libc::c_int);
                }
                3 => {
                    SV_StartSound(ent, 4 as libc::c_int,
                                  b"player/pl_wade4.wav\x00" as *const u8 as
                                      *const libc::c_char, 1.0f32,
                                  0.8f64 as libc::c_float, 0 as libc::c_int,
                                  100 as libc::c_int);
                }
                _ => { }
            }
            (*ent).v.flags =
                (flags as libc::c_uint &
                     !((1 as libc::c_uint) << 4 as libc::c_int)) as
                    libc::c_int
        }
        (*ent).v.air_finished =
            (sv.time + 12.0f32 as libc::c_double) as libc::c_float;
        return
    }
    if watertype == -(5 as libc::c_int) {
        if flags as libc::c_uint &
               ((1 as libc::c_uint) << 19 as libc::c_int |
                    (1 as libc::c_uint) << 6 as libc::c_int) == 0 &&
               ((*ent).v.dmgtime as libc::c_double) < sv.time {
            if ((*ent).v.radsuit_finished as libc::c_double) < sv.time {
                (*ent).v.dmgtime =
                    (sv.time + 0.2f32 as libc::c_double) as libc::c_float
            } else {
                (*ent).v.dmgtime =
                    (sv.time + 1.0f32 as libc::c_double) as libc::c_float
            }
        }
    } else if watertype == -(4 as libc::c_int) {
        if flags as libc::c_uint &
               ((1 as libc::c_uint) << 18 as libc::c_int |
                    (1 as libc::c_uint) << 6 as libc::c_int) == 0 &&
               ((*ent).v.dmgtime as libc::c_double) < sv.time {
            if ((*ent).v.radsuit_finished as libc::c_double) < sv.time {
                (*ent).v.dmgtime = (sv.time + 1.0f64) as libc::c_float
            }
            // otherwise radsuit is fully protect entity from slime
        }
    }
    if flags as libc::c_uint & (1 as libc::c_uint) << 4 as libc::c_int == 0 {
        if watertype == -(3 as libc::c_int) {
            // entering the water
            match COM_RandomLong(0 as libc::c_int, 3 as libc::c_int) {
                0 => {
                    SV_StartSound(ent, 4 as libc::c_int,
                                  b"player/pl_wade1.wav\x00" as *const u8 as
                                      *const libc::c_char, 1.0f32,
                                  0.8f64 as libc::c_float, 0 as libc::c_int,
                                  100 as libc::c_int);
                }
                1 => {
                    SV_StartSound(ent, 4 as libc::c_int,
                                  b"player/pl_wade2.wav\x00" as *const u8 as
                                      *const libc::c_char, 1.0f32,
                                  0.8f64 as libc::c_float, 0 as libc::c_int,
                                  100 as libc::c_int);
                }
                2 => {
                    SV_StartSound(ent, 4 as libc::c_int,
                                  b"player/pl_wade3.wav\x00" as *const u8 as
                                      *const libc::c_char, 1.0f32,
                                  0.8f64 as libc::c_float, 0 as libc::c_int,
                                  100 as libc::c_int);
                }
                3 => {
                    SV_StartSound(ent, 4 as libc::c_int,
                                  b"player/pl_wade4.wav\x00" as *const u8 as
                                      *const libc::c_char, 1.0f32,
                                  0.8f64 as libc::c_float, 0 as libc::c_int,
                                  100 as libc::c_int);
                }
                _ => { }
            }
        }
        (*ent).v.flags =
            (flags as libc::c_uint | (1 as libc::c_uint) << 4 as libc::c_int)
                as libc::c_int;
        (*ent).v.dmgtime = 0.0f32
    }
    if flags as libc::c_uint & (1 as libc::c_uint) << 11 as libc::c_int == 0 {
        (*ent).v.velocity[0 as libc::c_int as usize] =
            (*ent).v.velocity[0 as libc::c_int as usize] +
                (*ent).v.waterlevel as libc::c_float * -0.8f32 * sv.frametime
                    * (*ent).v.velocity[0 as libc::c_int as usize];
        (*ent).v.velocity[1 as libc::c_int as usize] =
            (*ent).v.velocity[1 as libc::c_int as usize] +
                (*ent).v.waterlevel as libc::c_float * -0.8f32 * sv.frametime
                    * (*ent).v.velocity[1 as libc::c_int as usize];
        (*ent).v.velocity[2 as libc::c_int as usize] =
            (*ent).v.velocity[2 as libc::c_int as usize] +
                (*ent).v.waterlevel as libc::c_float * -0.8f32 * sv.frametime
                    * (*ent).v.velocity[2 as libc::c_int as usize]
    };
}
/*
=============
SV_VecToYaw

converts dir to yaw
=============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_VecToYaw(mut src: *const vec_t) -> libc::c_float {
    let mut yaw: libc::c_float = 0.;
    if src.is_null() { return 0.0f32 }
    if *src.offset(1 as libc::c_int as isize) == 0.0f32 &&
           *src.offset(0 as libc::c_int as isize) == 0.0f32 {
        yaw = 0.0f32
    } else {
        yaw =
            (__tg_atan2(*src.offset(1 as libc::c_int as isize),
                        *src.offset(0 as libc::c_int as isize)) * 180.0f32 /
                 3.14159265358979323846f64 as libc::c_float) as libc::c_int as
                libc::c_float;
        if yaw < 0 as libc::c_int as libc::c_float { yaw += 360.0f32 }
    }
    return yaw;
}
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn SV_MoveStep(mut ent: *mut edict_t,
                                     mut move_0: *mut vec_t,
                                     mut relink: qboolean) -> qboolean {
    let mut i: libc::c_int = 0;
    let mut trace: trace_t =
        trace_t{allsolid: false_0,
                startsolid: false_0,
                inopen: false_0,
                inwater: false_0,
                fraction: 0.,
                endpos: [0.; 3],
                plane: plane_t{normal: [0.; 3], dist: 0.,},
                ent: 0 as *mut edict_t,
                hitgroup: 0,};
    let mut oldorg: vec3_t = [0.; 3];
    let mut neworg: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut monsterClip: qboolean = false_0;
    let mut enemy: *mut edict_t = 0 as *mut edict_t;
    let mut dz: libc::c_float = 0.;
    oldorg[0 as libc::c_int as usize] =
        (*ent).v.origin[0 as libc::c_int as usize];
    oldorg[1 as libc::c_int as usize] =
        (*ent).v.origin[1 as libc::c_int as usize];
    oldorg[2 as libc::c_int as usize] =
        (*ent).v.origin[2 as libc::c_int as usize];
    neworg[0 as libc::c_int as usize] =
        (*ent).v.origin[0 as libc::c_int as usize] +
            *move_0.offset(0 as libc::c_int as isize);
    neworg[1 as libc::c_int as usize] =
        (*ent).v.origin[1 as libc::c_int as usize] +
            *move_0.offset(1 as libc::c_int as isize);
    neworg[2 as libc::c_int as usize] =
        (*ent).v.origin[2 as libc::c_int as usize] +
            *move_0.offset(2 as libc::c_int as isize);
    monsterClip =
        if (*ent).v.flags as libc::c_uint &
               (1 as libc::c_uint) << 23 as libc::c_int != 0 {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int } as qboolean;
    // well, try it.  Flying and swimming monsters are easiest.
    if (*ent).v.flags as libc::c_uint &
           ((1 as libc::c_uint) << 1 as libc::c_int |
                (1 as libc::c_uint) << 0 as libc::c_int) != 0 {
        // try one move with vertical motion, then one without
        i = 0 as libc::c_int;
        while i < 2 as libc::c_int {
            neworg[0 as libc::c_int as usize] =
                (*ent).v.origin[0 as libc::c_int as usize] +
                    *move_0.offset(0 as libc::c_int as isize);
            neworg[1 as libc::c_int as usize] =
                (*ent).v.origin[1 as libc::c_int as usize] +
                    *move_0.offset(1 as libc::c_int as isize);
            neworg[2 as libc::c_int as usize] =
                (*ent).v.origin[2 as libc::c_int as usize] +
                    *move_0.offset(2 as libc::c_int as isize);
            enemy = (*ent).v.enemy;
            if i == 0 as libc::c_int && !enemy.is_null() {
                dz =
                    (*ent).v.origin[2 as libc::c_int as usize] -
                        (*enemy).v.origin[2 as libc::c_int as usize];
                if dz > 40.0f32 {
                    neworg[2 as libc::c_int as usize] -= 8.0f32
                } else if dz < 30.0f32 {
                    neworg[2 as libc::c_int as usize] += 8.0f32
                }
            }
            trace =
                SV_Move((*ent).v.origin.as_mut_ptr() as *const vec_t,
                        (*ent).v.mins.as_mut_ptr(),
                        (*ent).v.maxs.as_mut_ptr(),
                        neworg.as_mut_ptr() as *const vec_t, 0 as libc::c_int,
                        ent, monsterClip);
            if trace.fraction == 1.0f32 {
                svs.groupmask = (*ent).v.groupinfo;
                // that move takes us out of the water.
				// apparently though, it's okay to travel into solids, lava, sky, etc :)
                if (*ent).v.flags as libc::c_uint &
                       (1 as libc::c_uint) << 1 as libc::c_int != 0 &&
                       SV_PointContents(trace.endpos.as_mut_ptr() as
                                            *const vec_t) ==
                           -(1 as libc::c_int) {
                    return false_0
                }
                (*ent).v.origin[0 as libc::c_int as usize] =
                    trace.endpos[0 as libc::c_int as usize];
                (*ent).v.origin[1 as libc::c_int as usize] =
                    trace.endpos[1 as libc::c_int as usize];
                (*ent).v.origin[2 as libc::c_int as usize] =
                    trace.endpos[2 as libc::c_int as usize];
                if relink as u64 != 0 { SV_LinkEdict(ent, true_0); }
                return true_0
            } else {
                if SV_CheckEdict(enemy,
                                 b"../engine/server/sv_move.c\x00" as
                                     *const u8 as *const libc::c_char,
                                 302 as libc::c_int) as u64 == 0 {
                    break ;
                }
                i += 1
            }
        }
        return false_0
    } else {
        dz = svgame.movevars.stepsize;
        neworg[2 as libc::c_int as usize] += dz;
        end[0 as libc::c_int as usize] = neworg[0 as libc::c_int as usize];
        end[1 as libc::c_int as usize] = neworg[1 as libc::c_int as usize];
        end[2 as libc::c_int as usize] = neworg[2 as libc::c_int as usize];
        end[2 as libc::c_int as usize] -= dz * 2.0f32;
        trace =
            SV_Move(neworg.as_mut_ptr() as *const vec_t,
                    (*ent).v.mins.as_mut_ptr(), (*ent).v.maxs.as_mut_ptr(),
                    end.as_mut_ptr() as *const vec_t, 0 as libc::c_int, ent,
                    monsterClip);
        if trace.allsolid as u64 != 0 { return false_0 }
        if trace.startsolid as libc::c_uint !=
               0 as libc::c_int as libc::c_uint {
            neworg[2 as libc::c_int as usize] -= dz;
            trace =
                SV_Move(neworg.as_mut_ptr() as *const vec_t,
                        (*ent).v.mins.as_mut_ptr(),
                        (*ent).v.maxs.as_mut_ptr(),
                        end.as_mut_ptr() as *const vec_t, 0 as libc::c_int,
                        ent, monsterClip);
            if trace.allsolid as libc::c_uint !=
                   0 as libc::c_int as libc::c_uint ||
                   trace.startsolid as libc::c_uint !=
                       0 as libc::c_int as libc::c_uint {
                return false_0
            }
        }
        if trace.fraction == 1.0f32 {
            if (*ent).v.flags as libc::c_uint &
                   (1 as libc::c_uint) << 10 as libc::c_int != 0 {
                (*ent).v.origin[0 as libc::c_int as usize] =
                    (*ent).v.origin[0 as libc::c_int as usize] +
                        *move_0.offset(0 as libc::c_int as isize);
                (*ent).v.origin[1 as libc::c_int as usize] =
                    (*ent).v.origin[1 as libc::c_int as usize] +
                        *move_0.offset(1 as libc::c_int as isize);
                (*ent).v.origin[2 as libc::c_int as usize] =
                    (*ent).v.origin[2 as libc::c_int as usize] +
                        *move_0.offset(2 as libc::c_int as isize);
                if relink as u64 != 0 { SV_LinkEdict(ent, true_0); }
                (*ent).v.flags =
                    ((*ent).v.flags as libc::c_uint &
                         !((1 as libc::c_uint) << 9 as libc::c_int)) as
                        libc::c_int;
                return true_0
            }
            return false_0
        } else {
            (*ent).v.origin[0 as libc::c_int as usize] =
                trace.endpos[0 as libc::c_int as usize];
            (*ent).v.origin[1 as libc::c_int as usize] =
                trace.endpos[1 as libc::c_int as usize];
            (*ent).v.origin[2 as libc::c_int as usize] =
                trace.endpos[2 as libc::c_int as usize];
            if SV_CheckBottom(ent, 0 as libc::c_int) as libc::c_uint ==
                   0 as libc::c_int as libc::c_uint {
                if (*ent).v.flags as libc::c_uint &
                       (1 as libc::c_uint) << 10 as libc::c_int != 0 {
                    if relink as u64 != 0 { SV_LinkEdict(ent, true_0); }
                    return true_0
                }
                (*ent).v.origin[0 as libc::c_int as usize] =
                    oldorg[0 as libc::c_int as usize];
                (*ent).v.origin[1 as libc::c_int as usize] =
                    oldorg[1 as libc::c_int as usize];
                (*ent).v.origin[2 as libc::c_int as usize] =
                    oldorg[2 as libc::c_int as usize];
                return false_0
            } else {
                (*ent).v.flags =
                    ((*ent).v.flags as libc::c_uint &
                         !((1 as libc::c_uint) << 10 as libc::c_int)) as
                        libc::c_int;
                (*ent).v.groundentity = trace.ent;
                if relink as u64 != 0 { SV_LinkEdict(ent, true_0); }
                return true_0
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn SV_MoveTest(mut ent: *mut edict_t,
                                     mut move_0: *mut vec_t,
                                     mut relink: qboolean) -> qboolean {
    let mut temp: libc::c_float = 0.;
    let mut oldorg: vec3_t = [0.; 3];
    let mut neworg: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut trace: trace_t =
        trace_t{allsolid: false_0,
                startsolid: false_0,
                inopen: false_0,
                inwater: false_0,
                fraction: 0.,
                endpos: [0.; 3],
                plane: plane_t{normal: [0.; 3], dist: 0.,},
                ent: 0 as *mut edict_t,
                hitgroup: 0,};
    oldorg[0 as libc::c_int as usize] =
        (*ent).v.origin[0 as libc::c_int as usize];
    oldorg[1 as libc::c_int as usize] =
        (*ent).v.origin[1 as libc::c_int as usize];
    oldorg[2 as libc::c_int as usize] =
        (*ent).v.origin[2 as libc::c_int as usize];
    neworg[0 as libc::c_int as usize] =
        (*ent).v.origin[0 as libc::c_int as usize] +
            *move_0.offset(0 as libc::c_int as isize);
    neworg[1 as libc::c_int as usize] =
        (*ent).v.origin[1 as libc::c_int as usize] +
            *move_0.offset(1 as libc::c_int as isize);
    neworg[2 as libc::c_int as usize] =
        (*ent).v.origin[2 as libc::c_int as usize] +
            *move_0.offset(2 as libc::c_int as isize);
    temp = svgame.movevars.stepsize;
    neworg[2 as libc::c_int as usize] += temp;
    end[0 as libc::c_int as usize] = neworg[0 as libc::c_int as usize];
    end[1 as libc::c_int as usize] = neworg[1 as libc::c_int as usize];
    end[2 as libc::c_int as usize] = neworg[2 as libc::c_int as usize];
    end[2 as libc::c_int as usize] -= temp * 2.0f32;
    trace =
        SV_MoveNoEnts(neworg.as_mut_ptr() as *const vec_t,
                      (*ent).v.mins.as_mut_ptr(), (*ent).v.maxs.as_mut_ptr(),
                      end.as_mut_ptr() as *const vec_t, 0 as libc::c_int,
                      ent);
    if trace.allsolid as libc::c_uint != 0 as libc::c_int as libc::c_uint {
        return false_0
    }
    if trace.startsolid as libc::c_uint != 0 as libc::c_int as libc::c_uint {
        neworg[2 as libc::c_int as usize] -= temp;
        trace =
            SV_MoveNoEnts(neworg.as_mut_ptr() as *const vec_t,
                          (*ent).v.mins.as_mut_ptr(),
                          (*ent).v.maxs.as_mut_ptr(),
                          end.as_mut_ptr() as *const vec_t, 0 as libc::c_int,
                          ent);
        if trace.allsolid as libc::c_uint != 0 as libc::c_int as libc::c_uint
               ||
               trace.startsolid as libc::c_uint !=
                   0 as libc::c_int as libc::c_uint {
            return false_0
        }
    }
    if trace.fraction == 1.0f32 {
        if (*ent).v.flags as libc::c_uint &
               (1 as libc::c_uint) << 10 as libc::c_int != 0 {
            (*ent).v.origin[0 as libc::c_int as usize] =
                (*ent).v.origin[0 as libc::c_int as usize] +
                    *move_0.offset(0 as libc::c_int as isize);
            (*ent).v.origin[1 as libc::c_int as usize] =
                (*ent).v.origin[1 as libc::c_int as usize] +
                    *move_0.offset(1 as libc::c_int as isize);
            (*ent).v.origin[2 as libc::c_int as usize] =
                (*ent).v.origin[2 as libc::c_int as usize] +
                    *move_0.offset(2 as libc::c_int as isize);
            if relink as u64 != 0 { SV_LinkEdict(ent, true_0); }
            (*ent).v.flags =
                ((*ent).v.flags as libc::c_uint &
                     !((1 as libc::c_uint) << 9 as libc::c_int)) as
                    libc::c_int;
            return true_0
        }
        return false_0
    } else {
        (*ent).v.origin[0 as libc::c_int as usize] =
            trace.endpos[0 as libc::c_int as usize];
        (*ent).v.origin[1 as libc::c_int as usize] =
            trace.endpos[1 as libc::c_int as usize];
        (*ent).v.origin[2 as libc::c_int as usize] =
            trace.endpos[2 as libc::c_int as usize];
        if SV_CheckBottom(ent, 1 as libc::c_int) as libc::c_uint ==
               0 as libc::c_int as libc::c_uint {
            if (*ent).v.flags as libc::c_uint &
                   (1 as libc::c_uint) << 10 as libc::c_int != 0 {
                if relink as u64 != 0 { SV_LinkEdict(ent, true_0); }
                return true_0
            }
            (*ent).v.origin[0 as libc::c_int as usize] =
                oldorg[0 as libc::c_int as usize];
            (*ent).v.origin[1 as libc::c_int as usize] =
                oldorg[1 as libc::c_int as usize];
            (*ent).v.origin[2 as libc::c_int as usize] =
                oldorg[2 as libc::c_int as usize];
            return false_0
        } else {
            (*ent).v.flags =
                ((*ent).v.flags as libc::c_uint &
                     !((1 as libc::c_uint) << 10 as libc::c_int)) as
                    libc::c_int;
            (*ent).v.groundentity = trace.ent;
            if relink as u64 != 0 { SV_LinkEdict(ent, true_0); }
            return true_0
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn SV_StepDirection(mut ent: *mut edict_t,
                                          mut yaw: libc::c_float,
                                          mut dist: libc::c_float)
 -> qboolean {
    let mut ret: libc::c_int = 0;
    let mut cSin: libc::c_float = 0.;
    let mut cCos: libc::c_float = 0.;
    let mut move_0: vec3_t = [0.; 3];
    yaw =
        yaw *
            (3.14159265358979323846f64 * 2 as libc::c_int as libc::c_double)
                as libc::c_float / 360.0f32;
    SinCos(yaw, &mut cSin, &mut cCos);
    move_0[0 as libc::c_int as usize] = cCos * dist;
    move_0[1 as libc::c_int as usize] = cSin * dist;
    move_0[2 as libc::c_int as usize] = 0.0f32;
    ret = SV_MoveStep(ent, move_0.as_mut_ptr(), false_0) as libc::c_int;
    SV_LinkEdict(ent, true_0);
    return ret as qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn SV_FlyDirection(mut ent: *mut edict_t,
                                         mut move_0: *mut vec_t) -> qboolean {
    let mut ret: libc::c_int = 0;
    ret = SV_MoveStep(ent, move_0, false_0) as libc::c_int;
    SV_LinkEdict(ent, true_0);
    return ret as qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn SV_NewChaseDir(mut actor: *mut edict_t,
                                        mut destination: *mut vec_t,
                                        mut dist: libc::c_float) {
    let mut deltax: libc::c_float = 0.;
    let mut deltay: libc::c_float = 0.;
    let mut tempdir: libc::c_float = 0.;
    let mut olddir: libc::c_float = 0.;
    let mut turnaround: libc::c_float = 0.;
    let mut d: vec3_t = [0.; 3];
    olddir =
        anglemod(((*actor).v.ideal_yaw / 45.0f32) as libc::c_int as
                     libc::c_float * 45.0f32);
    turnaround = anglemod(olddir - 180.0f32);
    deltax =
        *destination.offset(0 as libc::c_int as isize) -
            (*actor).v.origin[0 as libc::c_int as usize];
    deltay =
        *destination.offset(1 as libc::c_int as isize) -
            (*actor).v.origin[1 as libc::c_int as usize];
    if deltax > 10.0f32 {
        d[1 as libc::c_int as usize] = 0.0f32
    } else if deltax < -10.0f32 {
        d[1 as libc::c_int as usize] = 180.0f32
    } else { d[1 as libc::c_int as usize] = -(1 as libc::c_int) as vec_t }
    if deltay < -10.0f32 {
        d[2 as libc::c_int as usize] = 270.0f32
    } else if deltay > 10.0f32 {
        d[2 as libc::c_int as usize] = 90.0f32
    } else { d[2 as libc::c_int as usize] = -1.0f32 }
    // try direct route
    if d[1 as libc::c_int as usize] != -1.0f32 &&
           d[2 as libc::c_int as usize] != -1.0f32 {
        if d[1 as libc::c_int as usize] == 0.0f32 {
            tempdir =
                if d[2 as libc::c_int as usize] == 90.0f32 {
                    45.0f32
                } else { 315.0f32 }
        } else {
            tempdir =
                if d[2 as libc::c_int as usize] == 90.0f32 {
                    135.0f32
                } else { 215.0f32 }
        }
        if tempdir != turnaround &&
               SV_StepDirection(actor, tempdir, dist) as libc::c_uint != 0 {
            return
        }
    }
    // try other directions
    if COM_RandomLong(0 as libc::c_int, 1 as libc::c_int) != 0 as libc::c_int
           || __tg_fabs(deltay) > __tg_fabs(deltax) {
        tempdir = d[1 as libc::c_int as usize];
        d[1 as libc::c_int as usize] = d[2 as libc::c_int as usize];
        d[2 as libc::c_int as usize] = tempdir
    }
    if d[1 as libc::c_int as usize] != -1.0f32 &&
           d[1 as libc::c_int as usize] != turnaround &&
           SV_StepDirection(actor, d[1 as libc::c_int as usize], dist) as
               libc::c_uint != 0 {
        return
    }
    if d[2 as libc::c_int as usize] != -1.0f32 &&
           d[2 as libc::c_int as usize] != turnaround &&
           SV_StepDirection(actor, d[2 as libc::c_int as usize], dist) as
               libc::c_uint != 0 {
        return
    }
    // there is no direct path to the player, so pick another direction
    if olddir != -1.0f32 &&
           SV_StepDirection(actor, olddir, dist) as libc::c_uint != 0 {
        return
    }
    // fine, just run somewhere.
    if COM_RandomLong(0 as libc::c_int, 1 as libc::c_int) != 1 as libc::c_int
       {
        tempdir = 0 as libc::c_int as libc::c_float;
        while tempdir <= 315.0f32 {
            if tempdir != turnaround &&
                   SV_StepDirection(actor, tempdir, dist) as libc::c_uint != 0
               {
                return
            }
            tempdir += 45.0f32
        }
    } else {
        tempdir = 315.0f32;
        while tempdir >= 0.0f32 {
            if tempdir != turnaround &&
                   SV_StepDirection(actor, tempdir, dist) as libc::c_uint != 0
               {
                return
            }
            tempdir -= 45.0f32
        }
    }
    // we tried. run backwards. that ought to work...
    if turnaround != -1.0f32 &&
           SV_StepDirection(actor, turnaround, dist) as libc::c_uint != 0 {
        return
    }
    // well, we're stuck somehow.
    (*actor).v.ideal_yaw = olddir;
    // if a bridge was pulled out from underneath a monster, it may not have
	// a valid standing position at all.
    if SV_CheckBottom(actor, 0 as libc::c_int) as u64 == 0 {
        (*actor).v.flags =
            ((*actor).v.flags as libc::c_uint |
                 (1 as libc::c_uint) << 10 as libc::c_int) as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn SV_MoveToOrigin(mut ent: *mut edict_t,
                                         mut pflGoal: *const vec_t,
                                         mut dist: libc::c_float,
                                         mut iMoveType: libc::c_int) {
    let mut vecDist: vec3_t = [0.; 3];
    vecDist[0 as libc::c_int as usize] =
        *pflGoal.offset(0 as libc::c_int as isize);
    vecDist[1 as libc::c_int as usize] =
        *pflGoal.offset(1 as libc::c_int as isize);
    vecDist[2 as libc::c_int as usize] =
        *pflGoal.offset(2 as libc::c_int as isize);
    if (*ent).v.flags as libc::c_uint &
           ((1 as libc::c_uint) << 0 as libc::c_int |
                (1 as libc::c_uint) << 1 as libc::c_int |
                (1 as libc::c_uint) << 9 as libc::c_int) != 0 {
        if iMoveType == 0 as libc::c_int {
            if SV_StepDirection(ent, (*ent).v.ideal_yaw, dist) as u64 == 0 {
                SV_NewChaseDir(ent, vecDist.as_mut_ptr(), dist);
            }
        } else {
            vecDist[0 as libc::c_int as usize] -=
                (*ent).v.origin[0 as libc::c_int as usize];
            vecDist[1 as libc::c_int as usize] -=
                (*ent).v.origin[1 as libc::c_int as usize];
            if (*ent).v.flags as libc::c_uint &
                   ((1 as libc::c_uint) << 0 as libc::c_int |
                        (1 as libc::c_uint) << 1 as libc::c_int) != 0 {
                vecDist[2 as libc::c_int as usize] -=
                    (*ent).v.origin[2 as libc::c_int as usize]
            } else { vecDist[2 as libc::c_int as usize] = 0.0f32 }
            let mut ilength: libc::c_float =
                __tg_sqrt(vecDist[0 as libc::c_int as usize] *
                              vecDist[0 as libc::c_int as usize] +
                              vecDist[1 as libc::c_int as usize] *
                                  vecDist[1 as libc::c_int as usize] +
                              vecDist[2 as libc::c_int as usize] *
                                  vecDist[2 as libc::c_int as usize]);
            if ilength != 0. { ilength = 1.0f32 / ilength }
            vecDist[0 as libc::c_int as usize] *= ilength;
            vecDist[1 as libc::c_int as usize] *= ilength;
            vecDist[2 as libc::c_int as usize] *= ilength;
            vecDist[0 as libc::c_int as usize] =
                vecDist[0 as libc::c_int as usize] * dist;
            vecDist[1 as libc::c_int as usize] =
                vecDist[1 as libc::c_int as usize] * dist;
            vecDist[2 as libc::c_int as usize] =
                vecDist[2 as libc::c_int as usize] * dist;
            SV_FlyDirection(ent, vecDist.as_mut_ptr());
        }
    };
}
