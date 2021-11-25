#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type file_s;
    pub type grasshdr_s;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t,
             __compar: __compar_fn_t);
    #[no_mangle]
    fn SV_BroadcastPrintf(ignore: *mut sv_client_s, fmt: *const libc::c_char,
                          _: ...);
    #[no_mangle]
    fn Q_strlen(string: *const libc::c_char) -> size_t;
    #[no_mangle]
    fn Q_strncmp(s1: *const libc::c_char, s2: *const libc::c_char,
                 n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Sys_Error(error: *const libc::c_char, _: ...);
    #[no_mangle]
    static mut host_limitlocal: *mut convar_t;
    #[no_mangle]
    fn NET_IsLocalAddress(adr: netadr_t) -> qboolean;
    #[no_mangle]
    fn COM_ClearCustomizationList(pHead: *mut customization_t,
                                  bCleanDecals: qboolean);
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    fn Con_DPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn MSG_InitExt(sb: *mut sizebuf_t, pDebugName: *const libc::c_char,
                   pData: *mut libc::c_void, nBytes: libc::c_int,
                   nMaxBits: libc::c_int);
    #[no_mangle]
    fn MSG_CheckOverflow(sb: *mut sizebuf_t) -> qboolean;
    #[no_mangle]
    fn MSG_Clear(sb: *mut sizebuf_t);
    #[no_mangle]
    fn MSG_WriteOneBit(sb: *mut sizebuf_t, nValue: libc::c_int);
    #[no_mangle]
    fn MSG_WriteUBitLong(sb: *mut sizebuf_t, curData: uint,
                         numbits: libc::c_int);
    #[no_mangle]
    fn MSG_WriteBits(sb: *mut sizebuf_t, pData: *const libc::c_void,
                     nBits: libc::c_int) -> qboolean;
    #[no_mangle]
    fn MSG_WriteBitAngle(sb: *mut sizebuf_t, fAngle: libc::c_float,
                         numbits: libc::c_int);
    #[no_mangle]
    fn MSG_WriteCmdExt(sb: *mut sizebuf_t, cmd: libc::c_int, type_0: netsrc_t,
                       name: *const libc::c_char);
    #[no_mangle]
    fn MSG_WriteByte(sb: *mut sizebuf_t, val: libc::c_int);
    #[no_mangle]
    fn MSG_WriteWord(sb: *mut sizebuf_t, val: libc::c_int);
    #[no_mangle]
    fn MSG_WriteFloat(sb: *mut sizebuf_t, val: libc::c_float);
    #[no_mangle]
    fn MSG_WriteVec3Angles(sb: *mut sizebuf_t, fa: *const libc::c_float);
    #[no_mangle]
    fn Netchan_CreateFragments(chan: *mut netchan_t, msg: *mut sizebuf_t);
    #[no_mangle]
    fn Netchan_TransmitBits(chan: *mut netchan_t, lengthInBits: libc::c_int,
                            data: *mut byte);
    #[no_mangle]
    fn Netchan_CanPacket(chan: *mut netchan_t, choke: qboolean) -> qboolean;
    #[no_mangle]
    static mut SV_UPDATE_BACKUP: libc::c_int;
    #[no_mangle]
    static mut svs: server_static_t;
    #[no_mangle]
    static mut sv: server_t;
    #[no_mangle]
    static mut svgame: svgame_static_t;
    #[no_mangle]
    static mut sv_instancedbaseline: convar_t;
    #[no_mangle]
    static mut sv_minupdaterate: convar_t;
    #[no_mangle]
    static mut sv_maxupdaterate: convar_t;
    #[no_mangle]
    static mut sv_failuretime: convar_t;
    #[no_mangle]
    fn SV_FinalMessage(message: *const libc::c_char, reconnect: qboolean);
    #[no_mangle]
    fn SV_DropClient(cl: *mut sv_client_t, crash: qboolean);
    #[no_mangle]
    fn SV_GetPlayerStats(cl: *mut sv_client_t, ping: *mut libc::c_int,
                         packet_loss: *mut libc::c_int);
    #[no_mangle]
    fn SV_EdictNum(n: libc::c_int) -> *mut edict_t;
    #[no_mangle]
    fn SV_ClassName(e: *const edict_t) -> *const libc::c_char;
    #[no_mangle]
    fn SV_IsPlayerIndex(idx: libc::c_int) -> qboolean;
    #[no_mangle]
    fn SV_CheckEdict(e: *const edict_t, file: *const libc::c_char,
                     line: libc::c_int) -> qboolean;
    #[no_mangle]
    fn SV_ClientFromEdict(pEdict: *const edict_t, spawned_only: qboolean)
     -> *mut sv_client_t;
    #[no_mangle]
    fn SV_ShouldUpdatePing(cl: *mut sv_client_t) -> qboolean;
    #[no_mangle]
    fn SV_FullUpdateMovevars(cl: *mut sv_client_t, msg: *mut sizebuf_t);
    #[no_mangle]
    fn SV_FullClientUpdate(cl: *mut sv_client_t, msg: *mut sizebuf_t);
    #[no_mangle]
    fn MSG_WriteDeltaEvent(msg: *mut sizebuf_t, from: *mut event_args_s,
                           to: *mut event_args_s);
    #[no_mangle]
    fn MSG_WriteClientData(msg: *mut sizebuf_t, from: *mut clientdata_s,
                           to: *mut clientdata_s, timebase: libc::c_double);
    #[no_mangle]
    fn MSG_WriteWeaponData(msg: *mut sizebuf_t, from: *mut weapon_data_s,
                           to: *mut weapon_data_s, timebase: libc::c_double,
                           index: libc::c_int);
    #[no_mangle]
    fn MSG_WriteDeltaEntity(from: *mut entity_state_s,
                            to: *mut entity_state_s, msg: *mut sizebuf_t,
                            force: qboolean, type_0: libc::c_int,
                            timebase: libc::c_double, ofs: libc::c_int);
    #[no_mangle]
    fn Delta_TestBaseline(from: *mut entity_state_s, to: *mut entity_state_s,
                          player: qboolean, timebase: libc::c_double)
     -> libc::c_int;
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
pub type __compar_fn_t
    =
    Option<unsafe extern "C" fn(_: *const libc::c_void,
                                _: *const libc::c_void) -> libc::c_int>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sv_ents_t {
    pub num_entities: libc::c_int,
    pub entities: [entity_state_t; 2048],
    pub sended: [byte; 1024],
}
#[inline]
unsafe extern "C" fn BitByte(mut bits: libc::c_int) -> libc::c_int {
    return (bits + (8 as libc::c_int - 1 as libc::c_int)) / 8 as libc::c_int *
               8 as libc::c_int >> 3 as libc::c_int;
}
#[inline]
unsafe extern "C" fn MSG_GetName(mut sb: *mut sizebuf_t)
 -> *const libc::c_char {
    return (*sb).pDebugName;
}
#[inline]
unsafe extern "C" fn MSG_GetNumBytesWritten(mut sb: *mut sizebuf_t)
 -> libc::c_int {
    return BitByte((*sb).iCurBit);
}
#[inline]
unsafe extern "C" fn MSG_GetNumBitsWritten(mut sb: *mut sizebuf_t)
 -> libc::c_int {
    return (*sb).iCurBit;
}
#[inline]
unsafe extern "C" fn MSG_GetNumBitsLeft(mut sb: *mut sizebuf_t)
 -> libc::c_int {
    return (*sb).nDataBits - (*sb).iCurBit;
}
#[inline]
unsafe extern "C" fn MSG_GetNumBytesLeft(mut sb: *mut sizebuf_t)
 -> libc::c_int {
    return MSG_GetNumBitsLeft(sb) >> 3 as libc::c_int;
}
#[inline]
unsafe extern "C" fn MSG_GetData(mut sb: *mut sizebuf_t) -> *mut byte {
    return (*sb).pData;
}
#[inline]
unsafe extern "C" fn MSG_GetBuf(mut sb: *mut sizebuf_t) -> *mut byte {
    return (*sb).pData;
}
#[no_mangle]
pub static mut c_fullsend: libc::c_int = 0;
// just a debug counter
#[no_mangle]
pub static mut c_notsend: libc::c_int = 0;
/*
=======================
SV_EntityNumbers
=======================
*/
unsafe extern "C" fn SV_EntityNumbers(mut a: *const libc::c_void,
                                      mut b: *const libc::c_void)
 -> libc::c_int {
    let mut ent1: libc::c_int = 0;
    let mut ent2: libc::c_int = 0;
    ent1 = (*(a as *mut entity_state_t)).number;
    ent2 = (*(b as *mut entity_state_t)).number;
    // watcom libc compares ents with itself
    if ent1 == ent2 { return 0 as libc::c_int }
    if ent1 < ent2 { return -(1 as libc::c_int) }
    return 1 as libc::c_int;
}
/*
=============
SV_AddEntitiesToPacket

=============
*/
unsafe extern "C" fn SV_AddEntitiesToPacket(mut pViewEnt: *mut edict_t,
                                            mut pClient: *mut edict_t,
                                            mut frame: *mut client_frame_t,
                                            mut ents: *mut sv_ents_t,
                                            mut from_client: qboolean) {
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    let mut clientpvs: *mut byte = 0 as *mut byte;
    let mut clientphs: *mut byte = 0 as *mut byte;
    let mut fullvis: qboolean = false_0;
    let mut cl: *mut sv_client_t = 0 as *mut sv_client_t;
    let mut player: qboolean = false_0;
    let mut state: *mut entity_state_t = 0 as *mut entity_state_t;
    let mut e: libc::c_int = 0;
    // during an error shutdown message we may need to transmit
	// the shutdown message after the server has shutdown, so
	// specifically check for it
    if sv.state as libc::c_uint == ss_dead as libc::c_int as libc::c_uint {
        return
    }
    cl = SV_ClientFromEdict(pClient, true_0);
    if cl.is_null() {
        Sys_Error(b"assert failed at %s:%i\n\x00" as *const u8 as
                      *const libc::c_char,
                  b"../engine/server/sv_frame.c\x00" as *const u8 as
                      *const libc::c_char, 77 as libc::c_int);
    }
    // portals can't change hostflags
    if from_client as u64 != 0 {
        // setup hostflags
        if (*cl).flags & (1 as libc::c_uint) << 5 as libc::c_int != 0 {
            sv.hostflags =
                (sv.hostflags as libc::c_uint |
                     (1 as libc::c_uint) << 0 as libc::c_int) as libc::c_int
        } else {
            sv.hostflags =
                (sv.hostflags as libc::c_uint &
                     !((1 as libc::c_uint) << 0 as libc::c_int)) as
                    libc::c_int
        }
        // reset viewents each frame
        (*cl).num_viewents = 0 as libc::c_int
    }
    svgame.dllFuncs.pfnSetupVisibility.expect("non-null function pointer")(pViewEnt,
                                                                           pClient,
                                                                           &mut clientpvs,
                                                                           &mut clientphs);
    if clientpvs.is_null() { fullvis = true_0 }
    let mut current_block_42: u64;
    // g-cont: of course we can send world but not want to do it :-)
    e = 1 as libc::c_int;
    while e < svgame.numEntities {
        let mut pset: *mut byte = 0 as *mut byte;
        ent = SV_EdictNum(e);
        // don't double add an entity through portals (in case this already added)
        if !(if e >= 0 as libc::c_int {
                 ((*ents).sended[(e >> 3 as libc::c_int) as usize] as
                      libc::c_int &
                      (1 as libc::c_int) << (e & 7 as libc::c_int)) as byte as
                     libc::c_int
             } else { false_0 as libc::c_int as byte as libc::c_int } != 0) {
            if e >= 1 as libc::c_int && e <= svs.maxclients {
                player = true_0
            } else { player = false_0 }
            if player as u64 != 0 {
                let mut cl_0: *mut sv_client_t =
                    &mut *svs.clients.offset((e - 1 as libc::c_int) as isize)
                        as *mut sv_client_t;
                if (*cl_0).state as libc::c_uint !=
                       cs_spawned as libc::c_int as libc::c_uint {
                    current_block_42 = 8457315219000651999;
                } else if (*cl_0).flags &
                              (1 as libc::c_uint) << 8 as libc::c_int != 0 {
                    current_block_42 = 8457315219000651999;
                } else { current_block_42 = 1608152415753874203; }
            } else { current_block_42 = 1608152415753874203; }
            match current_block_42 {
                8457315219000651999 => { }
                _ => {
                    if (*ent).v.effects as libc::c_uint &
                           (1 as libc::c_uint) << 30 as libc::c_int != 0 {
                        pset = clientphs
                    } else { pset = clientpvs }
                    state =
                        &mut *(*ents).entities.as_mut_ptr().offset((*ents).num_entities
                                                                       as
                                                                       isize)
                            as *mut entity_state_t;
                    // add entity to the net packet
                    if svgame.dllFuncs.pfnAddToFullPack.expect("non-null function pointer")(state,
                                                                                            e,
                                                                                            ent,
                                                                                            pClient,
                                                                                            sv.hostflags,
                                                                                            player
                                                                                                as
                                                                                                libc::c_int,
                                                                                            pset)
                           != 0 {
                        // to prevent adds it twice through portals
                        if e >= 0 as libc::c_int {
                            (*ents).sended[(e >> 3 as libc::c_int) as usize] =
                                ((*ents).sended[(e >> 3 as libc::c_int) as
                                                    usize] as libc::c_int |
                                     (1 as libc::c_int) <<
                                         (e & 7 as libc::c_int)) as byte
                        } else { };
                        if SV_CheckEdict((*ent).v.aiment,
                                         b"../engine/server/sv_frame.c\x00" as
                                             *const u8 as *const libc::c_char,
                                         132 as libc::c_int) as libc::c_uint
                               != 0 &&
                               (*(*ent).v.aiment).v.effects as libc::c_uint &
                                   (1 as libc::c_uint) << 29 as libc::c_int !=
                                   0 {
                            if (*cl).num_viewents < 128 as libc::c_int {
                                (*cl).viewentity[(*cl).num_viewents as usize]
                                    = (*ent).v.aiment;
                                (*cl).num_viewents += 1
                            }
                        }
                        // if we are full, silently discard entities
                        if (*ents).num_entities <
                               ((1 as libc::c_int) << 11 as libc::c_int) -
                                   1 as libc::c_int {
                            (*ents).num_entities += 1;
                            c_fullsend += 1 // entity accepted
                            // debug counter
                        } else {
                            // visibility list is full
				// continue counting entities,
				// so we know how many it's ovreflowed
                            c_notsend += 1
                        }
                    } // portal ents will be added anyway, ignore recursion
                    if !(fullvis as u64 != 0) {
                        // if it's a portal entity, add everything visible from its camera position
                        if from_client as libc::c_uint != 0 &&
                               (*ent).v.effects as libc::c_uint &
                                   (1 as libc::c_uint) << 29 as libc::c_int !=
                                   0 {
                            sv.hostflags =
                                (sv.hostflags as libc::c_uint |
                                     (1 as libc::c_uint) << 1 as libc::c_int)
                                    as libc::c_int;
                            SV_AddEntitiesToPacket(ent, pClient, frame, ents,
                                                   false_0);
                            sv.hostflags =
                                (sv.hostflags as libc::c_uint &
                                     !((1 as libc::c_uint) <<
                                           1 as libc::c_int)) as libc::c_int
                        }
                    }
                }
            }
        }
        e += 1
    };
}
/*
=============================================================================

Encode a client frame onto the network channel

=============================================================================
*/
/*
=============
SV_FindBestBaseline

trying to deltas with previous entities
=============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_FindBestBaseline(mut cl: *mut sv_client_t,
                                             mut index: libc::c_int,
                                             mut baseline:
                                                 *mut *mut entity_state_t,
                                             mut to: *mut entity_state_t,
                                             mut frame: *mut client_frame_t,
                                             mut player: qboolean)
 -> libc::c_int {
    let mut bestBitCount: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut bitCount: libc::c_int = 0;
    let mut bestfound: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    j = Delta_TestBaseline(*baseline, to, player, sv.time);
    bestBitCount = j;
    bestfound = index;
    // lookup backward for previous 64 states and try to interpret current delta as baseline
    i = index - 1 as libc::c_int;
    while bestBitCount > 0 as libc::c_int && i >= 0 as libc::c_int &&
              index - i < 64 as libc::c_int - 1 as libc::c_int {
        // don't worry about underflow in circular buffer
        let mut test: *mut entity_state_t =
            &mut *svs.packet_entities.offset((((*frame).first_entity + i) %
                                                  svs.num_client_entities) as
                                                 isize) as
                *mut entity_state_t;
        if (*to).entityType == (*test).entityType {
            bitCount = Delta_TestBaseline(test, to, player, sv.time);
            if bitCount < bestBitCount {
                bestBitCount = bitCount;
                bestfound = i
            }
        }
        i -= 1
    }
    // using delta from previous entity as baseline for current
    if index != bestfound {
        *baseline =
            &mut *svs.packet_entities.offset((((*frame).first_entity +
                                                   bestfound) %
                                                  svs.num_client_entities) as
                                                 isize) as *mut entity_state_t
    }
    return index - bestfound;
}
/*
=============
SV_FindBestBaselineForStatic

trying to deltas with previous static entities
=============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_FindBestBaselineForStatic(mut index: libc::c_int,
                                                      mut baseline:
                                                          *mut *mut entity_state_t,
                                                      mut to:
                                                          *mut entity_state_t)
 -> libc::c_int {
    let mut bestBitCount: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut bitCount: libc::c_int = 0;
    let mut bestfound: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    j = Delta_TestBaseline(*baseline, to, false_0, sv.time);
    bestBitCount = j;
    bestfound = index;
    // lookup backward for previous 64 states and try to interpret current delta as baseline
    i = index - 1 as libc::c_int;
    while bestBitCount > 0 as libc::c_int && i >= 0 as libc::c_int &&
              index - i < 64 as libc::c_int - 1 as libc::c_int {
        // don't worry about underflow in circular buffer
        let mut test: *mut entity_state_t =
            &mut *svs.static_entities.offset(i as isize) as
                *mut entity_state_t;
        bitCount = Delta_TestBaseline(test, to, false_0, sv.time);
        if bitCount < bestBitCount { bestBitCount = bitCount; bestfound = i }
        i -= 1
    }
    // using delta from previous entity as baseline for current
    if index != bestfound {
        *baseline =
            &mut *svs.static_entities.offset(bestfound as isize) as
                *mut entity_state_t
    }
    return index - bestfound;
}
/*
=============
SV_EmitPacketEntities

Writes a delta update of an entity_state_t list to the message->
=============
*/
unsafe extern "C" fn SV_EmitPacketEntities(mut cl: *mut sv_client_t,
                                           mut to: *mut client_frame_t,
                                           mut msg: *mut sizebuf_t) {
    let mut oldent: *mut entity_state_t = 0 as *mut entity_state_t;
    let mut newent: *mut entity_state_t = 0 as *mut entity_state_t;
    let mut oldindex: libc::c_int = 0;
    let mut newindex: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut oldnum: libc::c_int = 0;
    let mut newnum: libc::c_int = 0;
    let mut player: qboolean = false_0;
    let mut oldmax: libc::c_int = 0;
    let mut from: *mut client_frame_t = 0 as *mut client_frame_t;
    // this is the frame that we are going to delta update from
    if (*cl).delta_sequence != -(1 as libc::c_int) {
        from =
            &mut *(*cl).frames.offset(((*cl).delta_sequence &
                                           SV_UPDATE_BACKUP -
                                               1 as libc::c_int) as isize) as
                *mut client_frame_t;
        oldmax = (*from).num_entities;
        // the snapshot's entities may still have rolled off the buffer, though
        if (*from).first_entity <=
               svs.next_client_entities - svs.num_client_entities {
            Con_DPrintf(b"^3Warning:^7 %s: delta request from out of date entities.\n\x00"
                            as *const u8 as *const libc::c_char,
                        (*cl).name.as_mut_ptr());
            MSG_WriteCmdExt(msg, 40 as libc::c_int, NS_SERVER,
                            0 as *const libc::c_char);
            MSG_WriteUBitLong(msg,
                              ((*to).num_entities - 1 as libc::c_int) as uint,
                              11 as libc::c_int);
            from = 0 as *mut client_frame_t;
            oldmax = 0 as libc::c_int
        } else {
            MSG_WriteCmdExt(msg, 41 as libc::c_int, NS_SERVER,
                            0 as *const libc::c_char);
            MSG_WriteUBitLong(msg,
                              ((*to).num_entities - 1 as libc::c_int) as uint,
                              11 as libc::c_int);
            MSG_WriteByte(msg, (*cl).delta_sequence);
        }
    } else {
        from = 0 as *mut client_frame_t;
        oldmax = 0 as libc::c_int;
        MSG_WriteCmdExt(msg, 40 as libc::c_int, NS_SERVER,
                        0 as *const libc::c_char);
        MSG_WriteUBitLong(msg,
                          ((*to).num_entities - 1 as libc::c_int) as uint,
                          11 as libc::c_int);
    }
    newent = 0 as *mut entity_state_t;
    oldent = 0 as *mut entity_state_t;
    newindex = 0 as libc::c_int;
    oldindex = 0 as libc::c_int;
    while newindex < (*to).num_entities || oldindex < oldmax {
        if newindex >= (*to).num_entities {
            newnum = 99999 as libc::c_int;
            player = false_0
        } else {
            newent =
                &mut *svs.packet_entities.offset((((*to).first_entity +
                                                       newindex) %
                                                      svs.num_client_entities)
                                                     as isize) as
                    *mut entity_state_t;
            player = SV_IsPlayerIndex((*newent).number);
            newnum = (*newent).number
        }
        if oldindex >= oldmax {
            oldnum = 99999 as libc::c_int
        } else {
            oldent =
                &mut *svs.packet_entities.offset((((*from).first_entity +
                                                       oldindex) %
                                                      svs.num_client_entities)
                                                     as isize) as
                    *mut entity_state_t;
            oldnum = (*oldent).number
        }
        if newnum == oldnum {
            // delta update from old position
			// because the force parm is false, this will not result
			// in any bytes being emited if the entity has not changed at all
            MSG_WriteDeltaEntity(oldent, newent, msg, false_0,
                                 player as libc::c_int, sv.time,
                                 0 as libc::c_int);
            oldindex += 1;
            newindex += 1
        } else if newnum < oldnum {
            let mut baseline: *mut entity_state_t =
                &mut *svs.baselines.offset(newnum as isize) as
                    *mut entity_state_t;
            let mut classname: *const libc::c_char =
                SV_ClassName(SV_EdictNum(newnum));
            let mut offset: libc::c_int = 0 as libc::c_int;
            // trying to reduce message by select optimal baseline
            if sv_instancedbaseline.value == 0. || sv.num_instanced == 0 ||
                   sv.last_valid_baseline > newnum {
                offset =
                    SV_FindBestBaseline(cl, newindex, &mut baseline, newent,
                                        to, player)
            } else {
                i = 0 as libc::c_int; // to avoid zero offset
                while i < sv.num_instanced {
                    if Q_strncmp(classname,
                                 sv.instanced[i as usize].classname,
                                 99999 as libc::c_int) == 0 {
                        baseline =
                            &mut (*sv.instanced.as_mut_ptr().offset(i as
                                                                        isize)).baseline;
                        offset = -i - 1 as libc::c_int;
                        break ;
                    } else { i += 1 }
                }
            }
            // this is a new entity, send it from the baseline
            MSG_WriteDeltaEntity(baseline, newent, msg, true_0,
                                 player as libc::c_int, sv.time, offset);
            newindex += 1
        } else {
            if !(newnum > oldnum) { continue ; }
            let mut ed: *mut edict_t = SV_EdictNum((*oldent).number);
            let mut force: qboolean = false_0;
            // check if entity completely removed from server
            if (*ed).free as libc::c_uint != 0 ||
                   (*ed).v.flags as libc::c_uint &
                       (1 as libc::c_uint) << 30 as libc::c_int != 0 {
                force = true_0
            }
            // remove from message
            MSG_WriteDeltaEntity(oldent, 0 as *mut entity_state_s, msg, force,
                                 false_0 as libc::c_int, sv.time,
                                 0 as libc::c_int);
            oldindex += 1
        }
    }
    MSG_WriteUBitLong(msg,
                      (((1 as libc::c_int) << 13 as libc::c_int) -
                           1 as libc::c_int) as uint, 13 as libc::c_int);
    // end of packetentities
}
/*
=============
SV_EmitEvents

=============
*/
unsafe extern "C" fn SV_EmitEvents(mut cl: *mut sv_client_t,
                                   mut to: *mut client_frame_t,
                                   mut msg: *mut sizebuf_t) {
    let mut es: *mut event_state_t = 0 as *mut event_state_t;
    let mut info: *mut event_info_t = 0 as *mut event_info_t;
    let mut state: *mut entity_state_t = 0 as *mut entity_state_t;
    let mut nullargs: event_args_t =
        event_args_t{flags: 0,
                     entindex: 0,
                     origin: [0.; 3],
                     angles: [0.; 3],
                     velocity: [0.; 3],
                     ducking: 0,
                     fparam1: 0.,
                     fparam2: 0.,
                     iparam1: 0,
                     iparam2: 0,
                     bparam1: 0,
                     bparam2: 0,};
    let mut ev_count: libc::c_int = 0 as libc::c_int;
    let mut count: libc::c_int = 0;
    let mut ent_index: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ev: libc::c_int = 0;
    memset(&mut nullargs as *mut event_args_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<event_args_t>() as libc::c_ulong);
    es = &mut (*cl).events;
    // count events
    ev = 0 as libc::c_int;
    while ev < 64 as libc::c_int {
        if (*es).ei[ev as usize].index != 0 { ev_count += 1 }
        ev += 1
    }
    // nothing to send
    if ev_count == 0 { return } // nothing to send
    if ev_count >= 64 as libc::c_int / 2 as libc::c_int {
        ev_count = 64 as libc::c_int / 2 as libc::c_int - 1 as libc::c_int
    }
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        info =
            &mut *(*es).ei.as_mut_ptr().offset(i as isize) as
                *mut event_info_t;
        if !((*info).index as libc::c_int == 0 as libc::c_int) {
            ent_index = (*info).entity_index as libc::c_int;
            j = 0 as libc::c_int;
            while j < (*to).num_entities {
                state =
                    &mut *svs.packet_entities.offset((((*to).first_entity + j)
                                                          %
                                                          svs.num_client_entities)
                                                         as isize) as
                        *mut entity_state_t;
                if (*state).number == ent_index { break ; }
                j += 1
            }
            if j < (*to).num_entities {
                (*info).packet_index = j as libc::c_short;
                (*info).args.ducking = 0 as libc::c_int;
                if (*info).args.flags & (1 as libc::c_int) << 0 as libc::c_int
                       == 0 {
                    (*info).args.origin[2 as libc::c_int as usize] =
                        0 as libc::c_int as libc::c_float;
                    (*info).args.origin[1 as libc::c_int as usize] =
                        (*info).args.origin[2 as libc::c_int as usize];
                    (*info).args.origin[0 as libc::c_int as usize] =
                        (*info).args.origin[1 as libc::c_int as usize]
                }
                if (*info).args.flags & (1 as libc::c_int) << 1 as libc::c_int
                       == 0 {
                    (*info).args.angles[2 as libc::c_int as usize] =
                        0 as libc::c_int as libc::c_float;
                    (*info).args.angles[1 as libc::c_int as usize] =
                        (*info).args.angles[2 as libc::c_int as usize];
                    (*info).args.angles[0 as libc::c_int as usize] =
                        (*info).args.angles[1 as libc::c_int as usize]
                }
                (*info).args.velocity[2 as libc::c_int as usize] =
                    0 as libc::c_int as libc::c_float;
                (*info).args.velocity[1 as libc::c_int as usize] =
                    (*info).args.velocity[2 as libc::c_int as usize];
                (*info).args.velocity[0 as libc::c_int as usize] =
                    (*info).args.velocity[1 as libc::c_int as usize]
            } else {
                // couldn't find
                (*info).packet_index =
                    (*to).num_entities as libc::c_short; // create message
                (*info).args.entindex = ent_index
            }
        } // up to MAX_EVENT_QUEUE events
        i += 1
    }
    MSG_WriteCmdExt(msg, 3 as libc::c_int, NS_SERVER,
                    0 as *const libc::c_char);
    MSG_WriteUBitLong(msg, ev_count as uint, 5 as libc::c_int);
    i = 0 as libc::c_int;
    count = i;
    while i < 64 as libc::c_int {
        info =
            &mut *(*es).ei.as_mut_ptr().offset(i as isize) as
                *mut event_info_t;
        if (*info).index as libc::c_int == 0 as libc::c_int {
            (*info).packet_index = -(1 as libc::c_int) as libc::c_short;
            (*info).entity_index = -(1 as libc::c_int) as libc::c_short
        } else {
            // only send if there's room
            if count < ev_count {
                MSG_WriteUBitLong(msg, (*info).index as uint,
                                  10 as libc::c_int); // 1024 events
                if (*info).packet_index as libc::c_int == -(1 as libc::c_int)
                   {
                    MSG_WriteOneBit(msg, 0 as libc::c_int);
                } else {
                    MSG_WriteOneBit(msg, 1 as libc::c_int);
                    MSG_WriteUBitLong(msg, (*info).packet_index as uint,
                                      13 as libc::c_int);
                    if memcmp(&mut nullargs as *mut event_args_t as
                                  *const libc::c_void,
                              &mut (*info).args as *mut event_args_t as
                                  *const libc::c_void,
                              ::std::mem::size_of::<event_args_t>() as
                                  libc::c_ulong) == 0 {
                        MSG_WriteOneBit(msg, 0 as libc::c_int);
                    } else {
                        MSG_WriteOneBit(msg, 1 as libc::c_int);
                        MSG_WriteDeltaEvent(msg, &mut nullargs,
                                            &mut (*info).args);
                    }
                }
                if (*info).fire_time != 0. {
                    MSG_WriteOneBit(msg, 1 as libc::c_int);
                    MSG_WriteWord(msg,
                                  ((*info).fire_time * 100.0f32) as
                                      libc::c_int);
                } else { MSG_WriteOneBit(msg, 0 as libc::c_int); }
            }
            (*info).index = 0 as libc::c_int as word;
            (*info).packet_index = -(1 as libc::c_int) as libc::c_short;
            (*info).entity_index = -(1 as libc::c_int) as libc::c_short;
            count += 1
        }
        i += 1
    };
}
/*
=============
SV_EmitPings

=============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_EmitPings(mut msg: *mut sizebuf_t) {
    let mut cl: *mut sv_client_t = 0 as *mut sv_client_t;
    let mut packet_loss: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ping: libc::c_int = 0;
    MSG_WriteCmdExt(msg, 17 as libc::c_int, NS_SERVER,
                    0 as *const libc::c_char);
    i = 0 as libc::c_int;
    cl = svs.clients;
    while i < svs.maxclients {
        if !((*cl).state as libc::c_uint !=
                 cs_spawned as libc::c_int as libc::c_uint) {
            SV_GetPlayerStats(cl, &mut ping, &mut packet_loss);
            // there are 25 bits for each client
            MSG_WriteOneBit(msg, 1 as libc::c_int);
            MSG_WriteUBitLong(msg, i as uint, 5 as libc::c_int);
            MSG_WriteUBitLong(msg, ping as uint, 12 as libc::c_int);
            MSG_WriteUBitLong(msg, packet_loss as uint, 7 as libc::c_int);
        }
        i += 1;
        cl = cl.offset(1)
    }
    // end marker
    MSG_WriteOneBit(msg, 0 as libc::c_int);
}
/*
==================
SV_WriteClientdataToMessage

==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_WriteClientdataToMessage(mut cl: *mut sv_client_t,
                                                     mut msg:
                                                         *mut sizebuf_t) {
    let mut nullcd: clientdata_t =
        clientdata_t{origin: [0.; 3],
                     velocity: [0.; 3],
                     viewmodel: 0,
                     punchangle: [0.; 3],
                     flags: 0,
                     waterlevel: 0,
                     watertype: 0,
                     view_ofs: [0.; 3],
                     health: 0.,
                     bInDuck: 0,
                     weapons: 0,
                     flTimeStepSound: 0,
                     flDuckTime: 0,
                     flSwimTime: 0,
                     waterjumptime: 0,
                     maxspeed: 0.,
                     fov: 0.,
                     weaponanim: 0,
                     m_iId: 0,
                     ammo_shells: 0,
                     ammo_nails: 0,
                     ammo_cells: 0,
                     ammo_rockets: 0,
                     m_flNextAttack: 0.,
                     tfstate: 0,
                     pushmsec: 0,
                     deadflag: 0,
                     physinfo: [0; 256],
                     iuser1: 0,
                     iuser2: 0,
                     iuser3: 0,
                     iuser4: 0,
                     fuser1: 0.,
                     fuser2: 0.,
                     fuser3: 0.,
                     fuser4: 0.,
                     vuser1: [0.; 3],
                     vuser2: [0.; 3],
                     vuser3: [0.; 3],
                     vuser4: [0.; 3],};
    let mut from_cd: *mut clientdata_t = 0 as *mut clientdata_t;
    let mut to_cd: *mut clientdata_t = 0 as *mut clientdata_t;
    let mut nullwd: weapon_data_t =
        weapon_data_t{m_iId: 0,
                      m_iClip: 0,
                      m_flNextPrimaryAttack: 0.,
                      m_flNextSecondaryAttack: 0.,
                      m_flTimeWeaponIdle: 0.,
                      m_fInReload: 0,
                      m_fInSpecialReload: 0,
                      m_flNextReload: 0.,
                      m_flPumpTime: 0.,
                      m_fReloadTime: 0.,
                      m_fAimedDamage: 0.,
                      m_fNextAimBonus: 0.,
                      m_fInZoom: 0,
                      m_iWeaponState: 0,
                      iuser1: 0,
                      iuser2: 0,
                      iuser3: 0,
                      iuser4: 0,
                      fuser1: 0.,
                      fuser2: 0.,
                      fuser3: 0.,
                      fuser4: 0.,};
    let mut from_wd: *mut weapon_data_t = 0 as *mut weapon_data_t;
    let mut to_wd: *mut weapon_data_t = 0 as *mut weapon_data_t;
    let mut frame: *mut client_frame_t = 0 as *mut client_frame_t;
    let mut clent: *mut edict_t = 0 as *mut edict_t;
    let mut i: libc::c_int = 0;
    memset(&mut nullcd as *mut clientdata_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<clientdata_t>() as libc::c_ulong);
    frame =
        &mut *(*cl).frames.offset(((*cl).netchan.outgoing_sequence &
                                       (SV_UPDATE_BACKUP - 1 as libc::c_int)
                                           as libc::c_uint) as isize) as
            *mut client_frame_t;
    (*frame).senttime = host.realtime;
    (*frame).ping_time = -1.0f32;
    clent = (*cl).edict;
    if (*cl).chokecount != 0 as libc::c_int {
        MSG_WriteCmdExt(msg, 42 as libc::c_int, NS_SERVER,
                        0 as *const libc::c_char);
        (*cl).chokecount = 0 as libc::c_int
    }
    // update client fixangle
    match (*clent).v.fixangle {
        1 => {
            MSG_WriteCmdExt(msg, 10 as libc::c_int, NS_SERVER,
                            0 as *const libc::c_char); // reset fixangle
            MSG_WriteVec3Angles(msg, (*clent).v.angles.as_mut_ptr());
        }
        2 => {
            MSG_WriteCmdExt(msg, 38 as libc::c_int, NS_SERVER,
                            0 as *const libc::c_char);
            MSG_WriteBitAngle(msg,
                              (*clent).v.avelocity[1 as libc::c_int as usize],
                              16 as libc::c_int);
            (*clent).v.avelocity[1 as libc::c_int as usize] = 0.0f32
        }
        _ => { }
    }
    (*clent).v.fixangle = 0 as libc::c_int;
    memset(&mut (*frame).clientdata as *mut clientdata_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<clientdata_t>() as libc::c_ulong);
    // update clientdata_t
    svgame.dllFuncs.pfnUpdateClientData.expect("non-null function pointer")(clent,
                                                                            ((*cl).flags
                                                                                 &
                                                                                 (1
                                                                                      as
                                                                                      libc::c_uint)
                                                                                     <<
                                                                                     5
                                                                                         as
                                                                                         libc::c_int)
                                                                                as
                                                                                libc::c_int,
                                                                            &mut (*frame).clientdata); // don't send more nothing
    MSG_WriteCmdExt(msg, 15 as libc::c_int, NS_SERVER,
                    0 as *const libc::c_char);
    if (*cl).flags & (1 as libc::c_uint) << 8 as libc::c_int != 0 { return }
    if (*cl).delta_sequence == -(1 as libc::c_int) {
        from_cd = &mut nullcd
    } else {
        from_cd =
            &mut (*(*cl).frames.offset(((*cl).delta_sequence &
                                            SV_UPDATE_BACKUP -
                                                1 as libc::c_int) as
                                           isize)).clientdata
    }
    to_cd = &mut (*frame).clientdata;
    if (*cl).delta_sequence == -(1 as libc::c_int) {
        MSG_WriteOneBit(msg, 0 as libc::c_int);
        // no delta-compression
    } else {
        MSG_WriteOneBit(msg, 1 as libc::c_int); // we are delta-ing from
        MSG_WriteByte(msg, (*cl).delta_sequence);
    }
    // write clientdata_t
    MSG_WriteClientData(msg, from_cd, to_cd, sv.time);
    if (*cl).flags & (1 as libc::c_uint) << 5 as libc::c_int != 0 &&
           svgame.dllFuncs.pfnGetWeaponData.expect("non-null function pointer")(clent,
                                                                                (*frame).weapondata.as_mut_ptr())
               != 0 {
        memset(&mut nullwd as *mut weapon_data_t as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<weapon_data_t>() as libc::c_ulong);
        i = 0 as libc::c_int;
        while i < 64 as libc::c_int {
            if (*cl).delta_sequence == -(1 as libc::c_int) {
                from_wd = &mut nullwd
            } else {
                from_wd =
                    &mut *(*(*cl).frames.offset(((*cl).delta_sequence &
                                                     SV_UPDATE_BACKUP -
                                                         1 as libc::c_int) as
                                                    isize)).weapondata.as_mut_ptr().offset(i
                                                                                               as
                                                                                               isize)
                        as *mut weapon_data_t
            }
            to_wd =
                &mut *(*frame).weapondata.as_mut_ptr().offset(i as isize) as
                    *mut weapon_data_t;
            MSG_WriteWeaponData(msg, from_wd, to_wd, sv.time, i);
            i += 1
        }
    }
    // end marker
    MSG_WriteOneBit(msg, 0 as libc::c_int);
}
/*
==================
SV_WriteEntitiesToClient

==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_WriteEntitiesToClient(mut cl: *mut sv_client_t,
                                                  mut msg: *mut sizebuf_t) {
    let mut frame: *mut client_frame_t = 0 as *mut client_frame_t;
    let mut state: *mut entity_state_t = 0 as *mut entity_state_t;
    static mut frame_ents: sv_ents_t =
        sv_ents_t{num_entities: 0,
                  entities:
                      [entity_state_t{entityType: 0,
                                      number: 0,
                                      msg_time: 0.,
                                      messagenum: 0,
                                      origin: [0.; 3],
                                      angles: [0.; 3],
                                      modelindex: 0,
                                      sequence: 0,
                                      frame: 0.,
                                      colormap: 0,
                                      skin: 0,
                                      solid: 0,
                                      effects: 0,
                                      scale: 0.,
                                      eflags: 0,
                                      rendermode: 0,
                                      renderamt: 0,
                                      rendercolor: color24{r: 0, g: 0, b: 0,},
                                      renderfx: 0,
                                      movetype: 0,
                                      animtime: 0.,
                                      framerate: 0.,
                                      body: 0,
                                      controller: [0; 4],
                                      blending: [0; 4],
                                      velocity: [0.; 3],
                                      mins: [0.; 3],
                                      maxs: [0.; 3],
                                      aiment: 0,
                                      owner: 0,
                                      friction: 0.,
                                      gravity: 0.,
                                      team: 0,
                                      playerclass: 0,
                                      health: 0,
                                      spectator: false_0,
                                      weaponmodel: 0,
                                      gaitsequence: 0,
                                      basevelocity: [0.; 3],
                                      usehull: 0,
                                      oldbuttons: 0,
                                      onground: 0,
                                      iStepLeft: 0,
                                      flFallVelocity: 0.,
                                      fov: 0.,
                                      weaponanim: 0,
                                      startpos: [0.; 3],
                                      endpos: [0.; 3],
                                      impacttime: 0.,
                                      starttime: 0.,
                                      iuser1: 0,
                                      iuser2: 0,
                                      iuser3: 0,
                                      iuser4: 0,
                                      fuser1: 0.,
                                      fuser2: 0.,
                                      fuser3: 0.,
                                      fuser4: 0.,
                                      vuser1: [0.; 3],
                                      vuser2: [0.; 3],
                                      vuser3: [0.; 3],
                                      vuser4: [0.; 3],}; 2048],
                  sended: [0; 1024],};
    let mut i: libc::c_int = 0;
    let mut send_pings: libc::c_int = 0;
    frame =
        &mut *(*cl).frames.offset(((*cl).netchan.outgoing_sequence &
                                       (SV_UPDATE_BACKUP - 1 as libc::c_int)
                                           as libc::c_uint) as isize) as
            *mut client_frame_t;
    send_pings = SV_ShouldUpdatePing(cl) as libc::c_int;
    memset(frame_ents.sended.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<[byte; 1024]>() as libc::c_ulong);
    sv.hostflags =
        (sv.hostflags as libc::c_uint &
             !((1 as libc::c_uint) << 1 as libc::c_int)) as libc::c_int;
    // clear everything in this snapshot
    c_notsend = 0 as libc::c_int;
    c_fullsend = c_notsend;
    frame_ents.num_entities = c_fullsend;
    // add all the entities directly visible to the eye, which
	// may include portal entities that merge other viewpoints
    SV_AddEntitiesToPacket((*cl).pViewEntity, (*cl).edict, frame,
                           &mut frame_ents, true_0);
    if c_notsend != (*cl).ignored_ents {
        if c_notsend > 0 as libc::c_int {
            Con_Printf(b"^1Error:^7 Too many entities in visible packet list. Ignored %d entities\n\x00"
                           as *const u8 as *const libc::c_char, c_notsend);
        }
        (*cl).ignored_ents = c_notsend
    }
    // if there were portals visible, there may be out of order entities
	// in the list which will need to be resorted for the delta compression
	// to work correctly.  This also catches the error condition
	// of an entity being included twice.
    qsort(frame_ents.entities.as_mut_ptr() as *mut libc::c_void,
          frame_ents.num_entities as size_t,
          ::std::mem::size_of::<entity_state_t>() as libc::c_ulong,
          Some(SV_EntityNumbers as
                   unsafe extern "C" fn(_: *const libc::c_void,
                                        _: *const libc::c_void)
                       -> libc::c_int));
    // it will break all connected clients, but it takes more than one week to overflow it
    if (svs.next_client_entities as
            uint).wrapping_add(frame_ents.num_entities as libc::c_uint) >=
           0x7ffffffe as libc::c_int as libc::c_uint {
        svs.next_client_entities = 0 as libc::c_int;
        // delta is broken for now, cannot keep connected clients
        SV_FinalMessage(b"Server will restart due delta is outdated\n\x00" as
                            *const u8 as *const libc::c_char, true_0);
    }
    // copy the entity states out
    (*frame).first_entity = svs.next_client_entities;
    (*frame).num_entities = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < frame_ents.num_entities {
        // add it to the circular packet_entities array
        state =
            &mut *svs.packet_entities.offset((svs.next_client_entities %
                                                  svs.num_client_entities) as
                                                 isize) as
                *mut entity_state_t;
        *state = frame_ents.entities[i as usize];
        svs.next_client_entities += 1;
        (*frame).num_entities += 1;
        i += 1
    }
    SV_EmitPacketEntities(cl, frame, msg);
    SV_EmitEvents(cl, frame, msg);
    if send_pings != 0 { SV_EmitPings(msg); };
}
/*
===============================================================================

FRAME UPDATES

===============================================================================
*/
/*
=======================
SV_SendClientDatagram
=======================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_SendClientDatagram(mut cl: *mut sv_client_t) {
    let mut msg_buf: [byte; 16384] = [0; 16384];
    let mut msg: sizebuf_t =
        sizebuf_t{bOverflow: false_0,
                  pDebugName: 0 as *const libc::c_char,
                  pData: 0 as *const byte as *mut byte,
                  iCurBit: 0,
                  nDataBits: 0,};
    MSG_InitExt(&mut msg, b"Datagram\x00" as *const u8 as *const libc::c_char,
                msg_buf.as_mut_ptr() as *mut libc::c_void,
                ::std::mem::size_of::<[byte; 16384]>() as libc::c_ulong as
                    libc::c_int, -(1 as libc::c_int));
    // always send servertime at new frame
    MSG_WriteCmdExt(&mut msg, 7 as libc::c_int, NS_SERVER,
                    0 as *const libc::c_char);
    MSG_WriteFloat(&mut msg, sv.time as libc::c_float);
    SV_WriteClientdataToMessage(cl, &mut msg);
    SV_WriteEntitiesToClient(cl, &mut msg);
    // copy the accumulated multicast datagram
	// for this client out to the message
    if MSG_CheckOverflow(&mut (*cl).datagram) as u64 != 0 {
        Con_Printf(b"^3Warning:^7 %s overflowed for %s\n\x00" as *const u8 as
                       *const libc::c_char, MSG_GetName(&mut (*cl).datagram),
                   (*cl).name.as_mut_ptr());
    } else if MSG_GetNumBytesWritten(&mut (*cl).datagram) <
                  MSG_GetNumBytesLeft(&mut msg) {
        MSG_WriteBits(&mut msg,
                      MSG_GetData(&mut (*cl).datagram) as *const libc::c_void,
                      MSG_GetNumBitsWritten(&mut (*cl).datagram));
    } else {
        Con_DPrintf(b"^3Warning:^7 Ignoring unreliable datagram for %s, would overflow on msg\n\x00"
                        as *const u8 as *const libc::c_char,
                    (*cl).name.as_mut_ptr());
    }
    MSG_Clear(&mut (*cl).datagram);
    if MSG_CheckOverflow(&mut msg) as u64 != 0 {
        // must have room left for the packet header
        Con_Printf(b"^1Error:^7 %s overflowed for %s\n\x00" as *const u8 as
                       *const libc::c_char, MSG_GetName(&mut msg),
                   (*cl).name.as_mut_ptr());
        MSG_Clear(&mut msg);
    }
    // send the datagram
    Netchan_TransmitBits(&mut (*cl).netchan, MSG_GetNumBitsWritten(&mut msg),
                         MSG_GetData(&mut msg));
}
/*
=======================
SV_UpdateUserInfo
=======================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_UpdateUserInfo(mut cl: *mut sv_client_t) {
    SV_FullClientUpdate(cl, &mut sv.reliable_datagram);
    (*cl).flags = (*cl).flags & !((1 as libc::c_uint) << 0 as libc::c_int);
    (*cl).next_sendinfotime = host.realtime + 1.0f64;
}
/*
=======================
SV_UpdateToReliableMessages
=======================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_UpdateToReliableMessages() {
    let mut cl: *mut sv_client_t = 0 as *mut sv_client_t;
    let mut i: libc::c_int = 0;
    // check for changes to be sent over the reliable streams to all clients
    i = 0 as libc::c_int; // not in game yet
    cl = svs.clients;
    while i < svs.maxclients {
        if !(*cl).edict.is_null() {
            if !((*cl).state as libc::c_uint !=
                     cs_spawned as libc::c_int as libc::c_uint) {
                if (*cl).flags & (1 as libc::c_uint) << 0 as libc::c_int != 0
                       && (*cl).next_sendinfotime <= host.realtime {
                    if MSG_GetNumBytesLeft(&mut sv.reliable_datagram) as
                           libc::c_ulong >=
                           Q_strlen((*cl).userinfo.as_mut_ptr()).wrapping_add(6
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_ulong)
                       {
                        SV_UpdateUserInfo(cl);
                    }
                }
                if (*cl).flags & (1 as libc::c_uint) << 1 as libc::c_int != 0
                   {
                    SV_FullUpdateMovevars(cl, &mut (*cl).netchan.message);
                    (*cl).flags =
                        (*cl).flags &
                            !((1 as libc::c_uint) << 1 as libc::c_int)
                }
            }
        }
        i += 1;
        cl = cl.offset(1)
    }
    // clear the server datagram if it overflowed.
    if MSG_CheckOverflow(&mut sv.datagram) as u64 != 0 {
        Con_DPrintf(b"^1Error:^7 sv.datagram overflowed!\n\x00" as *const u8
                        as *const libc::c_char);
        MSG_Clear(&mut sv.datagram);
    }
    // clear the server datagram if it overflowed.
    if MSG_CheckOverflow(&mut sv.spec_datagram) as u64 != 0 {
        Con_DPrintf(b"^1Error:^7 sv.spec_datagram overflowed!\n\x00" as
                        *const u8 as *const libc::c_char);
        MSG_Clear(&mut sv.spec_datagram);
    }
    // now send the reliable and server datagrams to all clients.
    i = 0 as libc::c_int; // reliables go to all connected or spawned
    cl = svs.clients;
    while i < svs.maxclients {
        if !(((*cl).state as libc::c_uint) <
                 cs_connected as libc::c_int as libc::c_uint ||
                 (*cl).flags & (1 as libc::c_uint) << 7 as libc::c_int != 0) {
            if MSG_GetNumBytesWritten(&mut sv.reliable_datagram) <
                   MSG_GetNumBytesLeft(&mut (*cl).netchan.message) {
                MSG_WriteBits(&mut (*cl).netchan.message,
                              MSG_GetBuf(&mut sv.reliable_datagram) as
                                  *const libc::c_void,
                              MSG_GetNumBitsWritten(&mut sv.reliable_datagram));
            } else {
                Netchan_CreateFragments(&mut (*cl).netchan,
                                        &mut sv.reliable_datagram);
            }
            if MSG_GetNumBytesWritten(&mut sv.datagram) <
                   MSG_GetNumBytesLeft(&mut (*cl).datagram) {
                MSG_WriteBits(&mut (*cl).datagram,
                              MSG_GetBuf(&mut sv.datagram) as
                                  *const libc::c_void,
                              MSG_GetNumBitsWritten(&mut sv.datagram));
            } else {
                Con_DPrintf(b"^3Warning:^7 Ignoring unreliable datagram for %s, would overflow\n\x00"
                                as *const u8 as *const libc::c_char,
                            (*cl).name.as_mut_ptr());
            }
            if (*cl).flags & (1 as libc::c_uint) << 8 as libc::c_int != 0 {
                if MSG_GetNumBytesWritten(&mut sv.spec_datagram) <
                       MSG_GetNumBytesLeft(&mut (*cl).datagram) {
                    MSG_WriteBits(&mut (*cl).datagram,
                                  MSG_GetBuf(&mut sv.spec_datagram) as
                                      *const libc::c_void,
                                  MSG_GetNumBitsWritten(&mut sv.spec_datagram));
                } else {
                    Con_DPrintf(b"^3Warning:^7 Ignoring spectator datagram for %s, would overflow\n\x00"
                                    as *const u8 as *const libc::c_char,
                                (*cl).name.as_mut_ptr());
                }
            }
        }
        i += 1;
        cl = cl.offset(1)
    }
    // now clear the reliable and datagram buffers.
    MSG_Clear(&mut sv.reliable_datagram);
    MSG_Clear(&mut sv.spec_datagram);
    MSG_Clear(&mut sv.datagram);
}
/*
=======================
SV_SendClientMessages
=======================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_SendClientMessages() {
    let mut cl: *mut sv_client_t = 0 as *mut sv_client_t;
    let mut i: libc::c_int = 0;
    let mut updaterate_time: libc::c_double = 0.;
    let mut time_until_next_message: libc::c_double = 0.;
    if sv.state as libc::c_uint == ss_dead as libc::c_int as libc::c_uint {
        return
    }
    SV_UpdateToReliableMessages();
    // send a message to each connected client
    i = 0 as libc::c_int;
    sv.current_client = svs.clients;
    while i < svs.maxclients {
        cl = sv.current_client;
        if !((*cl).state as libc::c_uint <=
                 cs_zombie as libc::c_int as libc::c_uint ||
                 (*cl).flags & (1 as libc::c_uint) << 7 as libc::c_int != 0) {
            if (*cl).flags & (1 as libc::c_uint) << 2 as libc::c_int != 0 {
                (*cl).flags =
                    (*cl).flags & !((1 as libc::c_uint) << 2 as libc::c_int)
            } else {
                if (*host_limitlocal).value == 0. &&
                       NET_IsLocalAddress((*cl).netchan.remote_address) as
                           libc::c_uint != 0 {
                    (*cl).flags =
                        (*cl).flags | (1 as libc::c_uint) << 3 as libc::c_int
                }
                if (*cl).state as libc::c_uint ==
                       cs_spawned as libc::c_int as libc::c_uint {
                    // Try to send a message as soon as we can.
			// If the target time for sending is within the next frame interval ( based on last frame ),
			// trigger the send now. Note that in single player,
			// FCL_SEND_NET_MESSAGE flag is also set any time a packet arrives from the client.
                    time_until_next_message =
                        (*cl).next_messagetime -
                            (host.realtime + sv.frametime as libc::c_double);
                    if time_until_next_message <= 0.0f64 {
                        (*cl).flags =
                            (*cl).flags |
                                (1 as libc::c_uint) << 3 as libc::c_int
                    } else if time_until_next_message > 2.0f64 {
                        // something got hosed
                        (*cl).flags =
                            (*cl).flags |
                                (1 as libc::c_uint) << 3 as libc::c_int
                    }
                }
                // if the reliable message overflowed, drop the client
                if MSG_CheckOverflow(&mut (*cl).netchan.message) as u64 != 0 {
                    MSG_Clear(&mut (*cl).netchan.message);
                    MSG_Clear(&mut (*cl).datagram);
                    SV_BroadcastPrintf(0 as *mut sv_client_s,
                                       b"%s overflowed\n\x00" as *const u8 as
                                           *const libc::c_char,
                                       (*cl).name.as_mut_ptr());
                    Con_DPrintf(b"^1Error:^7 reliable overflow for %s\n\x00"
                                    as *const u8 as *const libc::c_char,
                                (*cl).name.as_mut_ptr());
                    SV_DropClient(cl, false_0);
                    (*cl).flags =
                        (*cl).flags | (1 as libc::c_uint) << 3 as libc::c_int;
                    (*cl).netchan.cleartime = 0.0f64
                    // don't choke this message
                } else if (*cl).flags &
                              (1 as libc::c_uint) << 3 as libc::c_int != 0 {
                    // If we haven't gotten a message in sv_failuretime seconds, then stop sending messages to this client
			// until we get another packet in from the client. This prevents crash/drop and reconnect where they are
			// being hosed with "sequenced packet without connection" packets.
                    if (sv_failuretime.value as libc::c_double) <
                           host.realtime - (*cl).netchan.last_received {
                        (*cl).flags =
                            (*cl).flags &
                                !((1 as libc::c_uint) << 3 as libc::c_int)
                    }
                }
                // only send messages if the client has sent one
		// and the bandwidth is not choked
                if (*cl).flags & (1 as libc::c_uint) << 3 as libc::c_int != 0
                   {
                    // bandwidth choke active?
                    if Netchan_CanPacket(&mut (*cl).netchan,
                                         ((*cl).state as libc::c_uint ==
                                              cs_spawned as libc::c_int as
                                                  libc::c_uint) as libc::c_int
                                             as qboolean) as u64 == 0 {
                        (*cl).chokecount += 1
                    } else {
                        // now that we were able to send, reset timer to point to next possible send time.
			// check here also because sv_max/minupdaterate could been changed in runtime
                        updaterate_time =
                            if (*cl).cl_updaterate >=
                                   1.0f64 /
                                       sv_maxupdaterate.value as
                                           libc::c_double {
                                if (*cl).cl_updaterate <
                                       1.0f64 /
                                           sv_minupdaterate.value as
                                               libc::c_double {
                                    (*cl).cl_updaterate
                                } else {
                                    (1.0f64) /
                                        sv_minupdaterate.value as
                                            libc::c_double
                                }
                            } else {
                                (1.0f64) /
                                    sv_maxupdaterate.value as libc::c_double
                            };
                        (*cl).next_messagetime =
                            host.realtime + sv.frametime as libc::c_double +
                                updaterate_time;
                        (*cl).flags =
                            (*cl).flags &
                                !((1 as libc::c_uint) << 3 as libc::c_int);
                        // NOTE: we should send frame even if server is not simulated to prevent overflow
                        if (*cl).state as libc::c_uint ==
                               cs_spawned as libc::c_int as libc::c_uint {
                            SV_SendClientDatagram(cl);
                        } else {
                            Netchan_TransmitBits(&mut (*cl).netchan,
                                                 0 as libc::c_int,
                                                 0 as *mut byte);
                        }
                    }
                    // just update reliable
                }
            }
        }
        i += 1;
        sv.current_client = sv.current_client.offset(1)
    }
    // reset current client
    sv.current_client = 0 as *mut sv_client_s;
}
/*
=======================
SV_SendMessagesToAll

e.g. before changing level
=======================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_SendMessagesToAll() {
    let mut cl: *mut sv_client_t = 0 as *mut sv_client_t;
    let mut i: libc::c_int = 0;
    if sv.state as libc::c_uint == ss_dead as libc::c_int as libc::c_uint {
        return
    }
    i = 0 as libc::c_int;
    cl = svs.clients;
    while i < svs.maxclients {
        if (*cl).state as libc::c_uint >=
               cs_connected as libc::c_int as libc::c_uint {
            (*cl).flags =
                (*cl).flags | (1 as libc::c_uint) << 3 as libc::c_int
        }
        i += 1;
        cl = cl.offset(1)
    }
    SV_SendClientMessages();
}
/*
=======================
SV_SkipUpdates

used before changing level
=======================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_SkipUpdates() {
    let mut cl: *mut sv_client_t = 0 as *mut sv_client_t;
    let mut i: libc::c_int = 0;
    if sv.state as libc::c_uint == ss_dead as libc::c_int as libc::c_uint {
        return
    }
    i = 0 as libc::c_int;
    cl = svs.clients;
    while i < svs.maxclients {
        if !((*cl).state as libc::c_uint !=
                 cs_spawned as libc::c_int as libc::c_uint ||
                 (*cl).flags & (1 as libc::c_uint) << 7 as libc::c_int != 0) {
            (*cl).flags =
                (*cl).flags | (1 as libc::c_uint) << 2 as libc::c_int
        }
        i += 1;
        cl = cl.offset(1)
    };
}
/*
=======================
SV_InactivateClients

Purpose: Prepare for level transition, etc.
=======================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_InactivateClients() {
    let mut i: libc::c_int = 0;
    let mut cl: *mut sv_client_t = 0 as *mut sv_client_t;
    if sv.state as libc::c_uint == ss_dead as libc::c_int as libc::c_uint {
        return
    }
    // send a message to each connected client
    i = 0 as libc::c_int;
    cl = svs.clients;
    while i < svs.maxclients {
        if !((*cl).state as u64 == 0 || (*cl).edict.is_null()) {
            if !(*cl).edict.is_null() {
                if (*(*cl).edict).v.flags as libc::c_uint &
                       (1 as libc::c_uint) << 13 as libc::c_int != 0 {
                    SV_DropClient(cl, false_0);
                } else {
                    if (*cl).state as libc::c_uint >
                           cs_connected as libc::c_int as libc::c_uint {
                        (*cl).state = cs_connected
                    }
                    COM_ClearCustomizationList(&mut (*cl).customdata,
                                               false_0);
                    memset((*cl).physinfo.as_mut_ptr() as *mut libc::c_void,
                           0 as libc::c_int,
                           256 as libc::c_int as libc::c_ulong);
                    // NOTE: many mods sending messages that must be applied on a next level
		// e.g. CryOfFear sending HideHud and PlayMp3 that affected after map change
                    if !((*svgame.globals).changelevel != 0) {
                        MSG_Clear(&mut (*cl).netchan.message);
                        MSG_Clear(&mut (*cl).datagram);
                    }
                }
            }
        }
        i += 1;
        cl = cl.offset(1)
    };
}
