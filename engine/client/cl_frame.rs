#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type file_s;
    pub type grasshdr_s;
    pub type engine_studio_api_s;
    pub type r_studio_interface_s;
    pub type IVoiceTweak_s;
    pub type demo_api_s;
    pub type mip_s;
    #[no_mangle]
    fn sqrtf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn NET_IsLocalAddress(adr: netadr_t) -> qboolean;
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    fn COM_NormalizeAngles(angles: *mut vec_t);
    #[no_mangle]
    fn Host_IsQuakeCompatible() -> qboolean;
    #[no_mangle]
    fn Host_Error(error: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_Reportf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_DPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn CL_IsThirdPerson() -> qboolean;
    #[no_mangle]
    fn CL_GetEntityByIndex(index: libc::c_int) -> *mut cl_entity_s;
    #[no_mangle]
    fn Con_NPrintf(idx: libc::c_int, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    static mut CL_UPDATE_BACKUP: libc::c_int;
    #[no_mangle]
    static mut cl: client_t;
    #[no_mangle]
    static mut cls: client_static_t;
    #[no_mangle]
    static mut clgame: clgame_static_t;
    #[no_mangle]
    static mut cl_interp: *mut convar_t;
    #[no_mangle]
    static mut cl_nointerp: *mut convar_t;
    #[no_mangle]
    static mut cl_bmodelinterp: *mut convar_t;
    #[no_mangle]
    fn CL_DecayLights();
    #[no_mangle]
    fn VectorAngles(forward: *const libc::c_float,
                    angles: *mut libc::c_float);
    #[no_mangle]
    fn AngleQuaternion(angles: *const vec_t, q: *mut vec_t, studio: qboolean);
    #[no_mangle]
    fn QuaternionAngle(q: *const vec_t, angles: *mut vec_t);
    #[no_mangle]
    fn QuaternionSlerp(p: *const vec_t, q: *const vec_t, t: libc::c_float,
                       qt: *mut vec_t);
    #[no_mangle]
    fn Mod_StudioExtradata(mod_0: *mut model_t) -> *mut libc::c_void;
    #[no_mangle]
    fn MSG_CheckOverflow(sb: *mut sizebuf_t) -> qboolean;
    #[no_mangle]
    fn MSG_ReadUBitLong(sb: *mut sizebuf_t, numbits: libc::c_int) -> uint;
    #[no_mangle]
    fn MSG_ReadByte(sb: *mut sizebuf_t) -> libc::c_int;
    #[no_mangle]
    fn MSG_ReadWord(sb: *mut sizebuf_t) -> libc::c_int;
    #[link_name = "ref"]
    static mut ref_0: ref_state_s;
    #[no_mangle]
    static mut refState: ref_globals_t;
    #[no_mangle]
    fn S_ExtraUpdate();
    #[no_mangle]
    fn CL_UpdateFrameLerp();
    #[no_mangle]
    fn CL_SignonReply();
    #[no_mangle]
    fn CL_WriteDemoJumpTime();
    #[no_mangle]
    fn CL_FireEvents();
    #[no_mangle]
    fn CL_ModelHandle(modelindex: libc::c_int) -> *mut model_t;
    #[no_mangle]
    fn CL_GetLocalPlayer() -> *mut cl_entity_t;
    #[no_mangle]
    fn CL_SetSolidEntities();
    #[no_mangle]
    fn CL_CheckPredictionError();
    #[no_mangle]
    fn CL_MoveSpectatorCamera();
    #[no_mangle]
    fn CL_KillDeadBeams(pDeadEntity: *mut cl_entity_t);
    #[no_mangle]
    fn CL_SetIdealPitch();
    #[no_mangle]
    fn CL_TestLights();
    #[no_mangle]
    fn CL_TempEntUpdate();
    #[no_mangle]
    fn MSG_ReadDeltaEntity(msg: *mut sizebuf_t, from: *mut entity_state_s,
                           to: *mut entity_state_s, num: libc::c_int,
                           type_0: libc::c_int, timebase: libc::c_double)
     -> qboolean;
    #[no_mangle]
    fn CL_AddEntityEffects(ent: *mut cl_entity_t);
    #[no_mangle]
    fn CL_AddModelEffects(ent: *mut cl_entity_t);
}
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type size_t = libc::c_ulong;
pub type HANDLE = *mut libc::c_void;
pub type HINSTANCE = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagPOINT {
    pub x: libc::c_int,
    pub y: libc::c_int,
}
pub type uint = libc::c_uint;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type uint8_t = __uint8_t;
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
pub type vec2_t = [vec_t; 2];
pub type vec3_t = [vec_t; 3];
pub type vec4_t = [vec_t; 4];
pub type rgba_t = [byte; 4];
pub type poolhandle_t = uint32_t;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type dword = libc::c_uint;
pub type string = [libc::c_char; 256];
pub type file_t = file_s;
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
pub struct cvar_s {
    pub name: *mut libc::c_char,
    pub string: *mut libc::c_char,
    pub flags: libc::c_int,
    pub value: libc::c_float,
    pub next: *mut cvar_s,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct con_nprint_s {
    pub index: libc::c_int,
    pub time_to_live: libc::c_float,
    pub color: [libc::c_float; 3],
}
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
pub type keydest_t = libc::c_uint;
pub const key_message: keydest_t = 3;
pub const key_menu: keydest_t = 2;
pub const key_game: keydest_t = 1;
pub const key_console: keydest_t = 0;
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
pub struct cmdalias_s {
    pub next: *mut cmdalias_s,
    pub name: [libc::c_char; 32],
    pub value: *mut libc::c_char,
}
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
pub type cldll_func_t = cldll_func_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cldll_func_s {
    pub pfnInitialize: Option<unsafe extern "C" fn(_: *mut cl_enginefunc_t,
                                                   _: libc::c_int)
                                  -> libc::c_int>,
    pub pfnInit: Option<unsafe extern "C" fn() -> ()>,
    pub pfnVidInit: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub pfnRedraw: Option<unsafe extern "C" fn(_: libc::c_float,
                                               _: libc::c_int)
                              -> libc::c_int>,
    pub pfnUpdateClientData: Option<unsafe extern "C" fn(_:
                                                             *mut client_data_t,
                                                         _: libc::c_float)
                                        -> libc::c_int>,
    pub pfnReset: Option<unsafe extern "C" fn() -> ()>,
    pub pfnPlayerMove: Option<unsafe extern "C" fn(_: *mut playermove_s,
                                                   _: libc::c_int) -> ()>,
    pub pfnPlayerMoveInit: Option<unsafe extern "C" fn(_: *mut playermove_s)
                                      -> ()>,
    pub pfnPlayerMoveTexture: Option<unsafe extern "C" fn(_:
                                                              *mut libc::c_char)
                                         -> libc::c_char>,
    pub IN_ActivateMouse: Option<unsafe extern "C" fn() -> ()>,
    pub IN_DeactivateMouse: Option<unsafe extern "C" fn() -> ()>,
    pub IN_MouseEvent: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub IN_ClearStates: Option<unsafe extern "C" fn() -> ()>,
    pub IN_Accumulate: Option<unsafe extern "C" fn() -> ()>,
    pub CL_CreateMove: Option<unsafe extern "C" fn(_: libc::c_float,
                                                   _: *mut usercmd_s,
                                                   _: libc::c_int) -> ()>,
    pub CL_IsThirdPerson: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub CL_CameraOffset: Option<unsafe extern "C" fn(_: *mut libc::c_float)
                                    -> ()>,
    pub KB_Find: Option<unsafe extern "C" fn(_: *const libc::c_char)
                            -> *mut libc::c_void>,
    pub CAM_Think: Option<unsafe extern "C" fn() -> ()>,
    pub pfnCalcRefdef: Option<unsafe extern "C" fn(_: *mut ref_params_t)
                                  -> ()>,
    pub pfnAddEntity: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _: *mut cl_entity_t,
                                                  _: *const libc::c_char)
                                 -> libc::c_int>,
    pub pfnCreateEntities: Option<unsafe extern "C" fn() -> ()>,
    pub pfnDrawNormalTriangles: Option<unsafe extern "C" fn() -> ()>,
    pub pfnDrawTransparentTriangles: Option<unsafe extern "C" fn() -> ()>,
    pub pfnStudioEvent: Option<unsafe extern "C" fn(_: *const mstudioevent_s,
                                                    _: *const cl_entity_t)
                                   -> ()>,
    pub pfnPostRunCmd: Option<unsafe extern "C" fn(_: *mut local_state_s,
                                                   _: *mut local_state_s,
                                                   _: *mut usercmd_t,
                                                   _: libc::c_int,
                                                   _: libc::c_double,
                                                   _: libc::c_uint) -> ()>,
    pub pfnShutdown: Option<unsafe extern "C" fn() -> ()>,
    pub pfnTxferLocalOverrides: Option<unsafe extern "C" fn(_:
                                                                *mut entity_state_t,
                                                            _:
                                                                *const clientdata_t)
                                           -> ()>,
    pub pfnProcessPlayerState: Option<unsafe extern "C" fn(_:
                                                               *mut entity_state_t,
                                                           _:
                                                               *const entity_state_t)
                                          -> ()>,
    pub pfnTxferPredictionData: Option<unsafe extern "C" fn(_:
                                                                *mut entity_state_t,
                                                            _:
                                                                *const entity_state_t,
                                                            _:
                                                                *mut clientdata_t,
                                                            _:
                                                                *const clientdata_t,
                                                            _:
                                                                *mut weapon_data_t,
                                                            _:
                                                                *const weapon_data_t)
                                           -> ()>,
    pub pfnDemo_ReadBuffer: Option<unsafe extern "C" fn(_: libc::c_int,
                                                        _: *mut byte) -> ()>,
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
    pub pfnFrame: Option<unsafe extern "C" fn(_: libc::c_double) -> ()>,
    pub pfnKey_Event: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _: libc::c_int,
                                                  _: *const libc::c_char)
                                 -> libc::c_int>,
    pub pfnTempEntUpdate: Option<unsafe extern "C" fn(_: libc::c_double,
                                                      _: libc::c_double,
                                                      _: libc::c_double,
                                                      _: *mut *mut tempent_s,
                                                      _: *mut *mut tempent_s,
                                                      _:
                                                          Option<unsafe extern "C" fn(_:
                                                                                          *mut cl_entity_t)
                                                                     ->
                                                                         libc::c_int>,
                                                      _:
                                                          Option<unsafe extern "C" fn(_:
                                                                                          *mut tempent_s,
                                                                                      _:
                                                                                          libc::c_float)
                                                                     -> ()>)
                                     -> ()>,
    pub pfnGetUserEntity: Option<unsafe extern "C" fn(_: libc::c_int)
                                     -> *mut cl_entity_t>,
    pub pfnVoiceStatus: Option<unsafe extern "C" fn(_: libc::c_int,
                                                    _: qboolean) -> ()>,
    pub pfnDirectorMessage: Option<unsafe extern "C" fn(_: libc::c_int,
                                                        _: *mut libc::c_void)
                                       -> ()>,
    pub pfnGetStudioModelInterface: Option<unsafe extern "C" fn(_:
                                                                    libc::c_int,
                                                                _:
                                                                    *mut *mut r_studio_interface_s,
                                                                _:
                                                                    *mut engine_studio_api_s)
                                               -> libc::c_int>,
    pub pfnChatInputPosition: Option<unsafe extern "C" fn(_: *mut libc::c_int,
                                                          _: *mut libc::c_int)
                                         -> ()>,
    pub pfnGetRenderInterface: Option<unsafe extern "C" fn(_: libc::c_int,
                                                           _:
                                                               *mut render_api_t,
                                                           _:
                                                               *mut render_interface_t)
                                          -> libc::c_int>,
    pub pfnClipMoveToEntity: Option<unsafe extern "C" fn(_: *mut physent_s,
                                                         _: *const vec_t,
                                                         _: *mut vec_t,
                                                         _: *mut vec_t,
                                                         _: *const vec_t,
                                                         _: *mut pmtrace_s)
                                        -> ()>,
    pub pfnTouchEvent: Option<unsafe extern "C" fn(_: libc::c_int,
                                                   _: libc::c_int,
                                                   _: libc::c_float,
                                                   _: libc::c_float,
                                                   _: libc::c_float,
                                                   _: libc::c_float)
                                  -> libc::c_int>,
    pub pfnMoveEvent: Option<unsafe extern "C" fn(_: libc::c_float,
                                                  _: libc::c_float) -> ()>,
    pub pfnLookEvent: Option<unsafe extern "C" fn(_: libc::c_float,
                                                  _: libc::c_float) -> ()>,
}
pub type render_interface_t = render_interface_s;
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
pub type render_api_t = render_api_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct render_api_s {
    pub RenderGetParm: Option<unsafe extern "C" fn(_: libc::c_int,
                                                   _: libc::c_int)
                                  -> libc::c_int>,
    pub GetDetailScaleForTexture: Option<unsafe extern "C" fn(_: libc::c_int,
                                                              _:
                                                                  *mut libc::c_float,
                                                              _:
                                                                  *mut libc::c_float)
                                             -> ()>,
    pub GetExtraParmsForTexture: Option<unsafe extern "C" fn(_: libc::c_int,
                                                             _: *mut byte,
                                                             _: *mut byte,
                                                             _: *mut byte,
                                                             _: *mut byte)
                                            -> ()>,
    pub GetLightStyle: Option<unsafe extern "C" fn(_: libc::c_int)
                                  -> *mut lightstyle_t>,
    pub GetDynamicLight: Option<unsafe extern "C" fn(_: libc::c_int)
                                    -> *mut dlight_t>,
    pub GetEntityLight: Option<unsafe extern "C" fn(_: libc::c_int)
                                   -> *mut dlight_t>,
    pub LightToTexGamma: Option<unsafe extern "C" fn(_: byte) -> byte>,
    pub GetFrameTime: Option<unsafe extern "C" fn() -> libc::c_float>,
    pub R_SetCurrentEntity: Option<unsafe extern "C" fn(_: *mut cl_entity_s)
                                       -> ()>,
    pub R_SetCurrentModel: Option<unsafe extern "C" fn(_: *mut model_s)
                                      -> ()>,
    pub R_FatPVS: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                              _: libc::c_float, _: *mut byte,
                                              _: qboolean, _: qboolean)
                             -> libc::c_int>,
    pub R_StoreEfrags: Option<unsafe extern "C" fn(_: *mut *mut efrag_s,
                                                   _: libc::c_int) -> ()>,
    pub GL_FindTexture: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                   -> libc::c_int>,
    pub GL_TextureName: Option<unsafe extern "C" fn(_: libc::c_uint)
                                   -> *const libc::c_char>,
    pub GL_TextureData: Option<unsafe extern "C" fn(_: libc::c_uint)
                                   -> *const byte>,
    pub GL_LoadTexture: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                    _: *const byte, _: size_t,
                                                    _: libc::c_int)
                                   -> libc::c_int>,
    pub GL_CreateTexture: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: *const libc::c_void,
                                                      _: texFlags_t)
                                     -> libc::c_int>,
    pub GL_LoadTextureArray: Option<unsafe extern "C" fn(_:
                                                             *mut *const libc::c_char,
                                                         _: libc::c_int)
                                        -> libc::c_int>,
    pub GL_CreateTextureArray: Option<unsafe extern "C" fn(_:
                                                               *const libc::c_char,
                                                           _: libc::c_int,
                                                           _: libc::c_int,
                                                           _: libc::c_int,
                                                           _:
                                                               *const libc::c_void,
                                                           _: texFlags_t)
                                          -> libc::c_int>,
    pub GL_FreeTexture: Option<unsafe extern "C" fn(_: libc::c_uint) -> ()>,
    pub DrawSingleDecal: Option<unsafe extern "C" fn(_: *mut decal_s,
                                                     _: *mut msurface_s)
                                    -> ()>,
    pub R_DecalSetupVerts: Option<unsafe extern "C" fn(_: *mut decal_s,
                                                       _: *mut msurface_s,
                                                       _: libc::c_int,
                                                       _: *mut libc::c_int)
                                      -> *mut libc::c_float>,
    pub R_EntityRemoveDecals: Option<unsafe extern "C" fn(_: *mut model_s)
                                         -> ()>,
    pub AVI_LoadVideo: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                   _: qboolean)
                                  -> *mut libc::c_void>,
    pub AVI_GetVideoInfo: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                      _: *mut libc::c_int,
                                                      _: *mut libc::c_int,
                                                      _: *mut libc::c_float)
                                     -> libc::c_int>,
    pub AVI_GetVideoFrameNumber: Option<unsafe extern "C" fn(_:
                                                                 *mut libc::c_void,
                                                             _: libc::c_float)
                                            -> libc::c_int>,
    pub AVI_GetVideoFrame: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                       _: libc::c_int)
                                      -> *mut byte>,
    pub AVI_UploadRawFrame: Option<unsafe extern "C" fn(_: libc::c_int,
                                                        _: libc::c_int,
                                                        _: libc::c_int,
                                                        _: libc::c_int,
                                                        _: libc::c_int,
                                                        _: *const byte)
                                       -> ()>,
    pub AVI_FreeVideo: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                  -> ()>,
    pub AVI_IsActive: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                 -> libc::c_int>,
    pub AVI_StreamSound: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                     _: libc::c_int,
                                                     _: libc::c_float,
                                                     _: libc::c_float,
                                                     _: libc::c_float) -> ()>,
    pub AVI_Reserved0: Option<unsafe extern "C" fn() -> ()>,
    pub AVI_Reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub GL_Bind: Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_uint)
                            -> ()>,
    pub GL_SelectTexture: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub GL_LoadTextureMatrix: Option<unsafe extern "C" fn(_:
                                                              *const libc::c_float)
                                         -> ()>,
    pub GL_TexMatrixIdentity: Option<unsafe extern "C" fn() -> ()>,
    pub GL_CleanUpTextureUnits: Option<unsafe extern "C" fn(_: libc::c_int)
                                           -> ()>,
    pub GL_TexGen: Option<unsafe extern "C" fn(_: libc::c_uint,
                                               _: libc::c_uint) -> ()>,
    pub GL_TextureTarget: Option<unsafe extern "C" fn(_: libc::c_uint) -> ()>,
    pub GL_TexCoordArrayMode: Option<unsafe extern "C" fn(_: libc::c_uint)
                                         -> ()>,
    pub GL_GetProcAddress: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                      -> *mut libc::c_void>,
    pub GL_UpdateTexSize: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int) -> ()>,
    pub GL_Reserved0: Option<unsafe extern "C" fn() -> ()>,
    pub GL_Reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub GL_DrawParticles: Option<unsafe extern "C" fn(_:
                                                          *const ref_viewpass_s,
                                                      _: qboolean,
                                                      _: libc::c_float)
                                     -> ()>,
    pub EnvShot: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                             _: *const libc::c_char,
                                             _: qboolean, _: libc::c_int)
                            -> ()>,
    pub SPR_LoadExt: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                 _: libc::c_uint)
                                -> libc::c_int>,
    pub LightVec: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                              _: *const libc::c_float,
                                              _: *mut libc::c_float,
                                              _: *mut libc::c_float)
                             -> colorVec>,
    pub StudioGetTexture: Option<unsafe extern "C" fn(_: *mut cl_entity_s)
                                     -> *mut mstudiotex_s>,
    pub GetOverviewParms: Option<unsafe extern "C" fn()
                                     -> *const ref_overview_s>,
    pub GetFileByIndex: Option<unsafe extern "C" fn(_: libc::c_int)
                                   -> *const libc::c_char>,
    pub pfnSaveFile: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                 _: *const libc::c_void,
                                                 _: libc::c_int)
                                -> libc::c_int>,
    pub R_Reserved0: Option<unsafe extern "C" fn() -> ()>,
    pub pfnMemAlloc: Option<unsafe extern "C" fn(_: size_t,
                                                 _: *const libc::c_char,
                                                 _: libc::c_int)
                                -> *mut libc::c_void>,
    pub pfnMemFree: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                _: *const libc::c_char,
                                                _: libc::c_int) -> ()>,
    pub pfnGetFilesList: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                     _: *mut libc::c_int,
                                                     _: libc::c_int)
                                    -> *mut *mut libc::c_char>,
    pub pfnFileBufferCRC32: Option<unsafe extern "C" fn(_:
                                                            *const libc::c_void,
                                                        _: libc::c_int)
                                       -> libc::c_uint>,
    pub COM_CompareFileTime: Option<unsafe extern "C" fn(_:
                                                             *const libc::c_char,
                                                         _:
                                                             *const libc::c_char,
                                                         _: *mut libc::c_int)
                                        -> libc::c_int>,
    pub Host_Error: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                _: ...) -> ()>,
    pub pfnGetModel: Option<unsafe extern "C" fn(_: libc::c_int)
                                -> *mut libc::c_void>,
    pub pfnTime: Option<unsafe extern "C" fn() -> libc::c_float>,
    pub Cvar_Set: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                              _: *const libc::c_char) -> ()>,
    pub S_FadeMusicVolume: Option<unsafe extern "C" fn(_: libc::c_float)
                                      -> ()>,
    pub SetRandomSeed: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
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
pub type dlight_t = dlight_s;
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
pub type cl_entity_t = cl_entity_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tempent_s {
    pub flags: libc::c_int,
    pub die: libc::c_float,
    pub frameMax: libc::c_float,
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub z: libc::c_float,
    pub fadeSpeed: libc::c_float,
    pub bounceFactor: libc::c_float,
    pub hitSound: libc::c_int,
    pub hitcallback: Option<unsafe extern "C" fn(_: *mut tempent_s,
                                                 _: *mut pmtrace_s) -> ()>,
    pub callback: Option<unsafe extern "C" fn(_: *mut tempent_s,
                                              _: libc::c_float,
                                              _: libc::c_float) -> ()>,
    pub next: *mut tempent_s,
    pub priority: libc::c_int,
    pub clientIndex: libc::c_short,
    pub tentOffset: vec3_t,
    pub entity: cl_entity_t,
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
pub struct local_state_s {
    pub playerstate: entity_state_t,
    pub client: clientdata_t,
    pub weapondata: [weapon_data_t; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mstudioevent_s {
    pub frame: int32_t,
    pub event: int32_t,
    pub unused: int32_t,
    pub options: [libc::c_char; 64],
}
pub type ref_params_t = ref_params_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ref_params_s {
    pub vieworg: vec3_t,
    pub viewangles: vec3_t,
    pub forward: vec3_t,
    pub right: vec3_t,
    pub up: vec3_t,
    pub frametime: libc::c_float,
    pub time: libc::c_float,
    pub intermission: libc::c_int,
    pub paused: libc::c_int,
    pub spectator: libc::c_int,
    pub onground: libc::c_int,
    pub waterlevel: libc::c_int,
    pub simvel: vec3_t,
    pub simorg: vec3_t,
    pub viewheight: vec3_t,
    pub idealpitch: libc::c_float,
    pub cl_viewangles: vec3_t,
    pub health: libc::c_int,
    pub crosshairangle: vec3_t,
    pub viewsize: libc::c_float,
    pub punchangle: vec3_t,
    pub maxclients: libc::c_int,
    pub viewentity: libc::c_int,
    pub playernum: libc::c_int,
    pub max_entities: libc::c_int,
    pub demoplayback: libc::c_int,
    pub hardware: libc::c_int,
    pub smoothing: libc::c_int,
    pub cmd: *mut usercmd_s,
    pub movevars: *mut movevars_s,
    pub viewport: [libc::c_int; 4],
    pub nextView: libc::c_int,
    pub onlyClientDraw: libc::c_int,
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
pub type client_data_t = client_data_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client_data_s {
    pub origin: vec3_t,
    pub viewangles: vec3_t,
    pub iWeaponBits: libc::c_int,
    pub fov: libc::c_float,
}
pub type cl_enginefunc_t = cl_enginefuncs_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cl_enginefuncs_s {
    pub pfnSPR_Load: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                -> HSPRITE>,
    pub pfnSPR_Frames: Option<unsafe extern "C" fn(_: HSPRITE)
                                  -> libc::c_int>,
    pub pfnSPR_Height: Option<unsafe extern "C" fn(_: HSPRITE, _: libc::c_int)
                                  -> libc::c_int>,
    pub pfnSPR_Width: Option<unsafe extern "C" fn(_: HSPRITE, _: libc::c_int)
                                 -> libc::c_int>,
    pub pfnSPR_Set: Option<unsafe extern "C" fn(_: HSPRITE, _: libc::c_int,
                                                _: libc::c_int,
                                                _: libc::c_int) -> ()>,
    pub pfnSPR_Draw: Option<unsafe extern "C" fn(_: libc::c_int,
                                                 _: libc::c_int,
                                                 _: libc::c_int,
                                                 _: *const wrect_t) -> ()>,
    pub pfnSPR_DrawHoles: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: *const wrect_t)
                                     -> ()>,
    pub pfnSPR_DrawAdditive: Option<unsafe extern "C" fn(_: libc::c_int,
                                                         _: libc::c_int,
                                                         _: libc::c_int,
                                                         _: *const wrect_t)
                                        -> ()>,
    pub pfnSPR_EnableScissor: Option<unsafe extern "C" fn(_: libc::c_int,
                                                          _: libc::c_int,
                                                          _: libc::c_int,
                                                          _: libc::c_int)
                                         -> ()>,
    pub pfnSPR_DisableScissor: Option<unsafe extern "C" fn() -> ()>,
    pub pfnSPR_GetList: Option<unsafe extern "C" fn(_: *mut libc::c_char,
                                                    _: *mut libc::c_int)
                                   -> *mut client_sprite_t>,
    pub pfnFillRGBA: Option<unsafe extern "C" fn(_: libc::c_int,
                                                 _: libc::c_int,
                                                 _: libc::c_int,
                                                 _: libc::c_int,
                                                 _: libc::c_int,
                                                 _: libc::c_int,
                                                 _: libc::c_int,
                                                 _: libc::c_int) -> ()>,
    pub pfnGetScreenInfo: Option<unsafe extern "C" fn(_: *mut SCREENINFO)
                                     -> libc::c_int>,
    pub pfnSetCrosshair: Option<unsafe extern "C" fn(_: HSPRITE, _: wrect_t,
                                                     _: libc::c_int,
                                                     _: libc::c_int,
                                                     _: libc::c_int) -> ()>,
    pub pfnRegisterVariable: Option<unsafe extern "C" fn(_:
                                                             *const libc::c_char,
                                                         _:
                                                             *const libc::c_char,
                                                         _: libc::c_int)
                                        -> *mut cvar_s>,
    pub pfnGetCvarFloat: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                    -> libc::c_float>,
    pub pfnGetCvarString: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                     -> *const libc::c_char>,
    pub pfnAddCommand: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                   _:
                                                       Option<unsafe extern "C" fn()
                                                                  -> ()>)
                                  -> libc::c_int>,
    pub pfnHookUserMsg: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                    _: pfnUserMsgHook)
                                   -> libc::c_int>,
    pub pfnServerCmd: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                 -> libc::c_int>,
    pub pfnClientCmd: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                 -> libc::c_int>,
    pub pfnGetPlayerInfo: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _:
                                                          *mut hud_player_info_t)
                                     -> ()>,
    pub pfnPlaySoundByName: Option<unsafe extern "C" fn(_:
                                                            *const libc::c_char,
                                                        _: libc::c_float)
                                       -> ()>,
    pub pfnPlaySoundByIndex: Option<unsafe extern "C" fn(_: libc::c_int,
                                                         _: libc::c_float)
                                        -> ()>,
    pub pfnAngleVectors: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                     _: *mut libc::c_float,
                                                     _: *mut libc::c_float,
                                                     _: *mut libc::c_float)
                                    -> ()>,
    pub pfnTextMessageGet: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                      -> *mut client_textmessage_t>,
    pub pfnDrawCharacter: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int)
                                     -> libc::c_int>,
    pub pfnDrawConsoleString: Option<unsafe extern "C" fn(_: libc::c_int,
                                                          _: libc::c_int,
                                                          _:
                                                              *mut libc::c_char)
                                         -> libc::c_int>,
    pub pfnDrawSetTextColor: Option<unsafe extern "C" fn(_: libc::c_float,
                                                         _: libc::c_float,
                                                         _: libc::c_float)
                                        -> ()>,
    pub pfnDrawConsoleStringLen: Option<unsafe extern "C" fn(_:
                                                                 *const libc::c_char,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _:
                                                                 *mut libc::c_int)
                                            -> ()>,
    pub pfnConsolePrint: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                    -> ()>,
    pub pfnCenterPrint: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                   -> ()>,
    pub GetWindowCenterX: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub GetWindowCenterY: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub GetViewAngles: Option<unsafe extern "C" fn(_: *mut libc::c_float)
                                  -> ()>,
    pub SetViewAngles: Option<unsafe extern "C" fn(_: *mut libc::c_float)
                                  -> ()>,
    pub GetMaxClients: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub Cvar_SetValue: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                   _: libc::c_float) -> ()>,
    pub Cmd_Argc: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub Cmd_Argv: Option<unsafe extern "C" fn(_: libc::c_int)
                             -> *const libc::c_char>,
    pub Con_Printf: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                _: ...) -> ()>,
    pub Con_DPrintf: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                 _: ...) -> ()>,
    pub Con_NPrintf: Option<unsafe extern "C" fn(_: libc::c_int,
                                                 _: *const libc::c_char,
                                                 _: ...) -> ()>,
    pub Con_NXPrintf: Option<unsafe extern "C" fn(_: *mut con_nprint_s,
                                                  _: *const libc::c_char,
                                                  _: ...) -> ()>,
    pub PhysInfo_ValueForKey: Option<unsafe extern "C" fn(_:
                                                              *const libc::c_char)
                                         -> *const libc::c_char>,
    pub ServerInfo_ValueForKey: Option<unsafe extern "C" fn(_:
                                                                *const libc::c_char)
                                           -> *const libc::c_char>,
    pub GetClientMaxspeed: Option<unsafe extern "C" fn() -> libc::c_float>,
    pub CheckParm: Option<unsafe extern "C" fn(_: *mut libc::c_char,
                                               _: *mut *mut libc::c_char)
                              -> libc::c_int>,
    pub Key_Event: Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int)
                              -> ()>,
    pub GetMousePosition: Option<unsafe extern "C" fn(_: *mut libc::c_int,
                                                      _: *mut libc::c_int)
                                     -> ()>,
    pub IsNoClipping: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub GetLocalPlayer: Option<unsafe extern "C" fn() -> *mut cl_entity_s>,
    pub GetViewModel: Option<unsafe extern "C" fn() -> *mut cl_entity_s>,
    pub GetEntityByIndex: Option<unsafe extern "C" fn(_: libc::c_int)
                                     -> *mut cl_entity_s>,
    pub GetClientTime: Option<unsafe extern "C" fn() -> libc::c_float>,
    pub V_CalcShake: Option<unsafe extern "C" fn() -> ()>,
    pub V_ApplyShake: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                  _: *mut libc::c_float,
                                                  _: libc::c_float) -> ()>,
    pub PM_PointContents: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                      _: *mut libc::c_int)
                                     -> libc::c_int>,
    pub PM_WaterEntity: Option<unsafe extern "C" fn(_: *const libc::c_float)
                                   -> libc::c_int>,
    pub PM_TraceLine: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                  _: *mut libc::c_float,
                                                  _: libc::c_int,
                                                  _: libc::c_int,
                                                  _: libc::c_int)
                                 -> *mut pmtrace_s>,
    pub CL_LoadModel: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                  _: *mut libc::c_int)
                                 -> *mut model_s>,
    pub CL_CreateVisibleEntity: Option<unsafe extern "C" fn(_: libc::c_int,
                                                            _:
                                                                *mut cl_entity_s)
                                           -> libc::c_int>,
    pub GetSpritePointer: Option<unsafe extern "C" fn(_: HSPRITE)
                                     -> *const model_s>,
    pub pfnPlaySoundByNameAtLocation: Option<unsafe extern "C" fn(_:
                                                                      *mut libc::c_char,
                                                                  _:
                                                                      libc::c_float,
                                                                  _:
                                                                      *mut libc::c_float)
                                                 -> ()>,
    pub pfnPrecacheEvent: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: *const libc::c_char)
                                     -> libc::c_ushort>,
    pub pfnPlaybackEvent: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: *const edict_s,
                                                      _: libc::c_ushort,
                                                      _: libc::c_float,
                                                      _: *mut libc::c_float,
                                                      _: *mut libc::c_float,
                                                      _: libc::c_float,
                                                      _: libc::c_float,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int) -> ()>,
    pub pfnWeaponAnim: Option<unsafe extern "C" fn(_: libc::c_int,
                                                   _: libc::c_int) -> ()>,
    pub pfnRandomFloat: Option<unsafe extern "C" fn(_: libc::c_float,
                                                    _: libc::c_float)
                                   -> libc::c_float>,
    pub pfnRandomLong: Option<unsafe extern "C" fn(_: libc::c_int,
                                                   _: libc::c_int)
                                  -> libc::c_int>,
    pub pfnHookEvent: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                  _:
                                                      Option<unsafe extern "C" fn(_:
                                                                                      *mut event_args_s)
                                                                 -> ()>)
                                 -> ()>,
    pub Con_IsVisible: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub pfnGetGameDirectory: Option<unsafe extern "C" fn()
                                        -> *const libc::c_char>,
    pub pfnGetCvarPointer: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                      -> *mut cvar_s>,
    pub Key_LookupBinding: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                      -> *const libc::c_char>,
    pub pfnGetLevelName: Option<unsafe extern "C" fn()
                                    -> *const libc::c_char>,
    pub pfnGetScreenFade: Option<unsafe extern "C" fn(_: *mut screenfade_s)
                                     -> ()>,
    pub pfnSetScreenFade: Option<unsafe extern "C" fn(_: *mut screenfade_s)
                                     -> ()>,
    pub VGui_GetPanel: Option<unsafe extern "C" fn() -> *mut libc::c_void>,
    pub VGui_ViewportPaintBackground: Option<unsafe extern "C" fn(_:
                                                                      *mut libc::c_int)
                                                 -> ()>,
    pub COM_LoadFile: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                  _: libc::c_int,
                                                  _: *mut libc::c_int)
                                 -> *mut byte>,
    pub COM_ParseFile: Option<unsafe extern "C" fn(_: *mut libc::c_char,
                                                   _: *mut libc::c_char)
                                  -> *mut libc::c_char>,
    pub COM_FreeFile: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                 -> ()>,
    pub pTriAPI: *mut triangleapi_s,
    pub pEfxAPI: *mut efx_api_s,
    pub pEventAPI: *mut event_api_s,
    pub pDemoAPI: *mut demo_api_s,
    pub pNetAPI: *mut net_api_s,
    pub pVoiceTweak: *mut IVoiceTweak_s,
    pub IsSpectateOnly: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub LoadMapSprite: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                  -> *mut model_s>,
    pub COM_AddAppDirectoryToSearchPath: Option<unsafe extern "C" fn(_:
                                                                         *const libc::c_char,
                                                                     _:
                                                                         *const libc::c_char)
                                                    -> ()>,
    pub COM_ExpandFilename: Option<unsafe extern "C" fn(_:
                                                            *const libc::c_char,
                                                        _: *mut libc::c_char,
                                                        _: libc::c_int)
                                       -> libc::c_int>,
    pub PlayerInfo_ValueForKey: Option<unsafe extern "C" fn(_: libc::c_int,
                                                            _:
                                                                *const libc::c_char)
                                           -> *const libc::c_char>,
    pub PlayerInfo_SetValueForKey: Option<unsafe extern "C" fn(_:
                                                                   *const libc::c_char,
                                                               _:
                                                                   *const libc::c_char)
                                              -> ()>,
    pub GetPlayerUniqueID: Option<unsafe extern "C" fn(_: libc::c_int,
                                                       _: *mut libc::c_char)
                                      -> qboolean>,
    pub GetTrackerIDForPlayer: Option<unsafe extern "C" fn(_: libc::c_int)
                                          -> libc::c_int>,
    pub GetPlayerForTrackerID: Option<unsafe extern "C" fn(_: libc::c_int)
                                          -> libc::c_int>,
    pub pfnServerCmdUnreliable: Option<unsafe extern "C" fn(_:
                                                                *mut libc::c_char)
                                           -> libc::c_int>,
    pub pfnGetMousePos: Option<unsafe extern "C" fn(_: *mut tagPOINT) -> ()>,
    pub pfnSetMousePos: Option<unsafe extern "C" fn(_: libc::c_int,
                                                    _: libc::c_int) -> ()>,
    pub pfnSetMouseEnable: Option<unsafe extern "C" fn(_: qboolean) -> ()>,
    pub pfnGetFirstCvarPtr: Option<unsafe extern "C" fn() -> *mut cvar_s>,
    pub pfnGetFirstCmdFunctionHandle: Option<unsafe extern "C" fn()
                                                 -> *mut libc::c_void>,
    pub pfnGetNextCmdFunctionHandle: Option<unsafe extern "C" fn(_:
                                                                     *mut libc::c_void)
                                                -> *mut libc::c_void>,
    pub pfnGetCmdFunctionName: Option<unsafe extern "C" fn(_:
                                                               *mut libc::c_void)
                                          -> *const libc::c_char>,
    pub pfnGetClientOldTime: Option<unsafe extern "C" fn() -> libc::c_float>,
    pub pfnGetGravity: Option<unsafe extern "C" fn() -> libc::c_float>,
    pub pfnGetModelByIndex: Option<unsafe extern "C" fn(_: libc::c_int)
                                       -> *mut model_s>,
    pub pfnSetFilterMode: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub pfnSetFilterColor: Option<unsafe extern "C" fn(_: libc::c_float,
                                                       _: libc::c_float,
                                                       _: libc::c_float)
                                      -> ()>,
    pub pfnSetFilterBrightness: Option<unsafe extern "C" fn(_: libc::c_float)
                                           -> ()>,
    pub pfnSequenceGet: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                    _: *const libc::c_char)
                                   -> *mut libc::c_void>,
    pub pfnSPR_DrawGeneric: Option<unsafe extern "C" fn(_: libc::c_int,
                                                        _: libc::c_int,
                                                        _: libc::c_int,
                                                        _: *const wrect_t,
                                                        _: libc::c_int,
                                                        _: libc::c_int,
                                                        _: libc::c_int,
                                                        _: libc::c_int)
                                       -> ()>,
    pub pfnSequencePickSentence: Option<unsafe extern "C" fn(_:
                                                                 *const libc::c_char,
                                                             _: libc::c_int,
                                                             _:
                                                                 *mut libc::c_int)
                                            -> *mut libc::c_void>,
    pub pfnDrawString: Option<unsafe extern "C" fn(_: libc::c_int,
                                                   _: libc::c_int,
                                                   _: *const libc::c_char,
                                                   _: libc::c_int,
                                                   _: libc::c_int,
                                                   _: libc::c_int)
                                  -> libc::c_int>,
    pub pfnDrawStringReverse: Option<unsafe extern "C" fn(_: libc::c_int,
                                                          _: libc::c_int,
                                                          _:
                                                              *const libc::c_char,
                                                          _: libc::c_int,
                                                          _: libc::c_int,
                                                          _: libc::c_int)
                                         -> libc::c_int>,
    pub LocalPlayerInfo_ValueForKey: Option<unsafe extern "C" fn(_:
                                                                     *const libc::c_char)
                                                -> *const libc::c_char>,
    pub pfnVGUI2DrawCharacter: Option<unsafe extern "C" fn(_: libc::c_int,
                                                           _: libc::c_int,
                                                           _: libc::c_int,
                                                           _: libc::c_uint)
                                          -> libc::c_int>,
    pub pfnVGUI2DrawCharacterAdditive: Option<unsafe extern "C" fn(_:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_uint)
                                                  -> libc::c_int>,
    pub pfnGetApproxWavePlayLen: Option<unsafe extern "C" fn(_:
                                                                 *const libc::c_char)
                                            -> libc::c_uint>,
    pub GetCareerGameUI: Option<unsafe extern "C" fn() -> *mut libc::c_void>,
    pub Cvar_Set: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                              _: *const libc::c_char) -> ()>,
    pub pfnIsPlayingCareerMatch: Option<unsafe extern "C" fn()
                                            -> libc::c_int>,
    pub pfnPlaySoundVoiceByName: Option<unsafe extern "C" fn(_:
                                                                 *mut libc::c_char,
                                                             _: libc::c_float,
                                                             _: libc::c_int)
                                            -> ()>,
    pub pfnPrimeMusicStream: Option<unsafe extern "C" fn(_: *mut libc::c_char,
                                                         _: libc::c_int)
                                        -> ()>,
    pub pfnSys_FloatTime: Option<unsafe extern "C" fn() -> libc::c_double>,
    pub pfnProcessTutorMessageDecayBuffer: Option<unsafe extern "C" fn(_:
                                                                           *mut libc::c_int,
                                                                       _:
                                                                           libc::c_int)
                                                      -> ()>,
    pub pfnConstructTutorMessageDecayBuffer: Option<unsafe extern "C" fn(_:
                                                                             *mut libc::c_int,
                                                                         _:
                                                                             libc::c_int)
                                                        -> ()>,
    pub pfnResetTutorMessageDecayData: Option<unsafe extern "C" fn() -> ()>,
    pub pfnPlaySoundByNameAtPitch: Option<unsafe extern "C" fn(_:
                                                                   *mut libc::c_char,
                                                               _:
                                                                   libc::c_float,
                                                               _: libc::c_int)
                                              -> ()>,
    pub pfnFillRGBABlend: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int) -> ()>,
    pub pfnGetAppID: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub pfnGetAliases: Option<unsafe extern "C" fn() -> *mut cmdalias_t>,
    pub pfnVguiWrap2_GetMouseDelta: Option<unsafe extern "C" fn(_:
                                                                    *mut libc::c_int,
                                                                _:
                                                                    *mut libc::c_int)
                                               -> ()>,
    pub pfnFilteredClientCmd: Option<unsafe extern "C" fn(_:
                                                              *const libc::c_char)
                                         -> libc::c_int>,
}
pub type cmdalias_t = cmdalias_s;
pub type wrect_t = wrect_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wrect_s {
    pub left: libc::c_int,
    pub right: libc::c_int,
    pub top: libc::c_int,
    pub bottom: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct net_api_s {
    pub InitNetworking: Option<unsafe extern "C" fn() -> ()>,
    pub Status: Option<unsafe extern "C" fn(_: *mut net_status_s) -> ()>,
    pub SendRequest: Option<unsafe extern "C" fn(_: libc::c_int,
                                                 _: libc::c_int,
                                                 _: libc::c_int,
                                                 _: libc::c_double,
                                                 _: *mut netadr_s,
                                                 _: net_api_response_func_t)
                                -> ()>,
    pub CancelRequest: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub CancelAllRequests: Option<unsafe extern "C" fn() -> ()>,
    pub AdrToString: Option<unsafe extern "C" fn(_: *mut netadr_s)
                                -> *const libc::c_char>,
    pub CompareAdr: Option<unsafe extern "C" fn(_: *mut netadr_s,
                                                _: *mut netadr_s)
                               -> libc::c_int>,
    pub StringToAdr: Option<unsafe extern "C" fn(_: *mut libc::c_char,
                                                 _: *mut netadr_s)
                                -> libc::c_int>,
    pub ValueForKey: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                 _: *const libc::c_char)
                                -> *const libc::c_char>,
    pub RemoveKey: Option<unsafe extern "C" fn(_: *mut libc::c_char,
                                               _: *const libc::c_char) -> ()>,
    pub SetValueForKey: Option<unsafe extern "C" fn(_: *mut libc::c_char,
                                                    _: *const libc::c_char,
                                                    _: *const libc::c_char,
                                                    _: libc::c_int) -> ()>,
}
pub type net_api_response_func_t
    =
    Option<unsafe extern "C" fn(_: *mut net_response_s) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct net_response_s {
    pub error: libc::c_int,
    pub context: libc::c_int,
    pub type_0: libc::c_int,
    pub remote_address: netadr_t,
    pub ping: libc::c_double,
    pub response: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct net_status_s {
    pub connected: libc::c_int,
    pub local_address: netadr_t,
    pub remote_address: netadr_t,
    pub packet_loss: libc::c_int,
    pub latency: libc::c_double,
    pub connection_time: libc::c_double,
    pub rate: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_api_s {
    pub version: libc::c_int,
    pub EV_PlaySound: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _: *mut libc::c_float,
                                                  _: libc::c_int,
                                                  _: *const libc::c_char,
                                                  _: libc::c_float,
                                                  _: libc::c_float,
                                                  _: libc::c_int,
                                                  _: libc::c_int) -> ()>,
    pub EV_StopSound: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _: libc::c_int,
                                                  _: *const libc::c_char)
                                 -> ()>,
    pub EV_FindModelIndex: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                      -> libc::c_int>,
    pub EV_IsLocal: Option<unsafe extern "C" fn(_: libc::c_int)
                               -> libc::c_int>,
    pub EV_LocalPlayerDucking: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub EV_LocalPlayerViewheight: Option<unsafe extern "C" fn(_:
                                                                  *mut libc::c_float)
                                             -> ()>,
    pub EV_LocalPlayerBounds: Option<unsafe extern "C" fn(_: libc::c_int,
                                                          _:
                                                              *mut libc::c_float,
                                                          _:
                                                              *mut libc::c_float)
                                         -> ()>,
    pub EV_IndexFromTrace: Option<unsafe extern "C" fn(_: *mut pmtrace_s)
                                      -> libc::c_int>,
    pub EV_GetPhysent: Option<unsafe extern "C" fn(_: libc::c_int)
                                  -> *mut physent_s>,
    pub EV_SetUpPlayerPrediction: Option<unsafe extern "C" fn(_: libc::c_int,
                                                              _: libc::c_int)
                                             -> ()>,
    pub EV_PushPMStates: Option<unsafe extern "C" fn() -> ()>,
    pub EV_PopPMStates: Option<unsafe extern "C" fn() -> ()>,
    pub EV_SetSolidPlayers: Option<unsafe extern "C" fn(_: libc::c_int)
                                       -> ()>,
    pub EV_SetTraceHull: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub EV_PlayerTrace: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                    _: *mut libc::c_float,
                                                    _: libc::c_int,
                                                    _: libc::c_int,
                                                    _: *mut pmtrace_s) -> ()>,
    pub EV_WeaponAnimation: Option<unsafe extern "C" fn(_: libc::c_int,
                                                        _: libc::c_int)
                                       -> ()>,
    pub EV_PrecacheEvent: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: *const libc::c_char)
                                     -> libc::c_ushort>,
    pub EV_PlaybackEvent: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: *const edict_s,
                                                      _: libc::c_ushort,
                                                      _: libc::c_float,
                                                      _: *mut libc::c_float,
                                                      _: *mut libc::c_float,
                                                      _: libc::c_float,
                                                      _: libc::c_float,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int) -> ()>,
    pub EV_TraceTexture: Option<unsafe extern "C" fn(_: libc::c_int,
                                                     _: *mut libc::c_float,
                                                     _: *mut libc::c_float)
                                    -> *const libc::c_char>,
    pub EV_StopAllSounds: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: libc::c_int) -> ()>,
    pub EV_KillEvents: Option<unsafe extern "C" fn(_: libc::c_int,
                                                   _: *const libc::c_char)
                                  -> ()>,
    pub EV_PlayerTraceExt: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                       _: *mut libc::c_float,
                                                       _: libc::c_int,
                                                       _:
                                                           Option<unsafe extern "C" fn(_:
                                                                                           *mut physent_s)
                                                                      ->
                                                                          libc::c_int>,
                                                       _: *mut pmtrace_s)
                                      -> ()>,
    pub EV_SoundForIndex: Option<unsafe extern "C" fn(_: libc::c_int)
                                     -> *const libc::c_char>,
    pub EV_TraceSurface: Option<unsafe extern "C" fn(_: libc::c_int,
                                                     _: *mut libc::c_float,
                                                     _: *mut libc::c_float)
                                    -> *mut msurface_s>,
    pub EV_GetMovevars: Option<unsafe extern "C" fn() -> *mut movevars_s>,
    pub EV_VisTraceLine: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                     _: *mut libc::c_float,
                                                     _: libc::c_int)
                                    -> *mut pmtrace_s>,
    pub EV_GetVisent: Option<unsafe extern "C" fn(_: libc::c_int)
                                 -> *mut physent_s>,
    pub EV_TestLine: Option<unsafe extern "C" fn(_: *const vec_t,
                                                 _: *const vec_t,
                                                 _: libc::c_int)
                                -> libc::c_int>,
    pub EV_PushTraceBounds: Option<unsafe extern "C" fn(_: libc::c_int,
                                                        _:
                                                            *const libc::c_float,
                                                        _:
                                                            *const libc::c_float)
                                       -> ()>,
    pub EV_PopTraceBounds: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct efx_api_s {
    pub R_AllocParticle: Option<unsafe extern "C" fn(_:
                                                         Option<unsafe extern "C" fn(_:
                                                                                         *mut particle_s,
                                                                                     _:
                                                                                         libc::c_float)
                                                                    -> ()>)
                                    -> *mut particle_t>,
    pub R_BlobExplosion: Option<unsafe extern "C" fn(_: *const libc::c_float)
                                    -> ()>,
    pub R_Blood: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                             _: *const libc::c_float,
                                             _: libc::c_int, _: libc::c_int)
                            -> ()>,
    pub R_BloodSprite: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                   _: libc::c_int,
                                                   _: libc::c_int,
                                                   _: libc::c_int,
                                                   _: libc::c_float) -> ()>,
    pub R_BloodStream: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                   _: *const libc::c_float,
                                                   _: libc::c_int,
                                                   _: libc::c_int) -> ()>,
    pub R_BreakModel: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                  _: *const libc::c_float,
                                                  _: *const libc::c_float,
                                                  _: libc::c_float,
                                                  _: libc::c_float,
                                                  _: libc::c_int,
                                                  _: libc::c_int,
                                                  _: libc::c_char) -> ()>,
    pub R_Bubbles: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                               _: *const libc::c_float,
                                               _: libc::c_float,
                                               _: libc::c_int, _: libc::c_int,
                                               _: libc::c_float) -> ()>,
    pub R_BubbleTrail: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                   _: *const libc::c_float,
                                                   _: libc::c_float,
                                                   _: libc::c_int,
                                                   _: libc::c_int,
                                                   _: libc::c_float) -> ()>,
    pub R_BulletImpactParticles: Option<unsafe extern "C" fn(_:
                                                                 *const libc::c_float)
                                            -> ()>,
    pub R_EntityParticles: Option<unsafe extern "C" fn(_: *mut cl_entity_s)
                                      -> ()>,
    pub R_Explosion: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                 _: libc::c_int,
                                                 _: libc::c_float,
                                                 _: libc::c_float,
                                                 _: libc::c_int) -> ()>,
    pub R_FizzEffect: Option<unsafe extern "C" fn(_: *mut cl_entity_s,
                                                  _: libc::c_int,
                                                  _: libc::c_int) -> ()>,
    pub R_FireField: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                 _: libc::c_int,
                                                 _: libc::c_int,
                                                 _: libc::c_int,
                                                 _: libc::c_int,
                                                 _: libc::c_float) -> ()>,
    pub R_FlickerParticles: Option<unsafe extern "C" fn(_:
                                                            *const libc::c_float)
                                       -> ()>,
    pub R_FunnelSprite: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                    _: libc::c_int,
                                                    _: libc::c_int) -> ()>,
    pub R_Implosion: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                 _: libc::c_float,
                                                 _: libc::c_int,
                                                 _: libc::c_float) -> ()>,
    pub R_LargeFunnel: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                   _: libc::c_int) -> ()>,
    pub R_LavaSplash: Option<unsafe extern "C" fn(_: *const libc::c_float)
                                 -> ()>,
    pub R_MultiGunshot: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                    _: *const libc::c_float,
                                                    _: *const libc::c_float,
                                                    _: libc::c_int,
                                                    _: libc::c_int,
                                                    _: *mut libc::c_int)
                                   -> ()>,
    pub R_MuzzleFlash: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                   _: libc::c_int) -> ()>,
    pub R_ParticleBox: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                   _: *const libc::c_float,
                                                   _: libc::c_uchar,
                                                   _: libc::c_uchar,
                                                   _: libc::c_uchar,
                                                   _: libc::c_float) -> ()>,
    pub R_ParticleBurst: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                     _: libc::c_int,
                                                     _: libc::c_int,
                                                     _: libc::c_float) -> ()>,
    pub R_ParticleExplosion: Option<unsafe extern "C" fn(_:
                                                             *const libc::c_float)
                                        -> ()>,
    pub R_ParticleExplosion2: Option<unsafe extern "C" fn(_:
                                                              *const libc::c_float,
                                                          _: libc::c_int,
                                                          _: libc::c_int)
                                         -> ()>,
    pub R_ParticleLine: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                    _: *const libc::c_float,
                                                    _: libc::c_uchar,
                                                    _: libc::c_uchar,
                                                    _: libc::c_uchar,
                                                    _: libc::c_float) -> ()>,
    pub R_PlayerSprites: Option<unsafe extern "C" fn(_: libc::c_int,
                                                     _: libc::c_int,
                                                     _: libc::c_int,
                                                     _: libc::c_int) -> ()>,
    pub R_Projectile: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                  _: *const libc::c_float,
                                                  _: libc::c_int,
                                                  _: libc::c_int,
                                                  _: libc::c_int,
                                                  _:
                                                      Option<unsafe extern "C" fn(_:
                                                                                      *mut tempent_s,
                                                                                  _:
                                                                                      *mut pmtrace_s)
                                                                 -> ()>)
                                 -> ()>,
    pub R_RicochetSound: Option<unsafe extern "C" fn(_: *const libc::c_float)
                                    -> ()>,
    pub R_RicochetSprite: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                      _: *mut model_s,
                                                      _: libc::c_float,
                                                      _: libc::c_float)
                                     -> ()>,
    pub R_RocketFlare: Option<unsafe extern "C" fn(_: *const libc::c_float)
                                  -> ()>,
    pub R_RocketTrail: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                   _: *mut libc::c_float,
                                                   _: libc::c_int) -> ()>,
    pub R_RunParticleEffect: Option<unsafe extern "C" fn(_:
                                                             *const libc::c_float,
                                                         _:
                                                             *const libc::c_float,
                                                         _: libc::c_int,
                                                         _: libc::c_int)
                                        -> ()>,
    pub R_ShowLine: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                _: *const libc::c_float)
                               -> ()>,
    pub R_SparkEffect: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                   _: libc::c_int,
                                                   _: libc::c_int,
                                                   _: libc::c_int) -> ()>,
    pub R_SparkShower: Option<unsafe extern "C" fn(_: *const libc::c_float)
                                  -> ()>,
    pub R_SparkStreaks: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                    _: libc::c_int,
                                                    _: libc::c_int,
                                                    _: libc::c_int) -> ()>,
    pub R_Spray: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                             _: *const libc::c_float,
                                             _: libc::c_int, _: libc::c_int,
                                             _: libc::c_int, _: libc::c_int,
                                             _: libc::c_int) -> ()>,
    pub R_Sprite_Explode: Option<unsafe extern "C" fn(_: *mut TEMPENTITY,
                                                      _: libc::c_float,
                                                      _: libc::c_int) -> ()>,
    pub R_Sprite_Smoke: Option<unsafe extern "C" fn(_: *mut TEMPENTITY,
                                                    _: libc::c_float) -> ()>,
    pub R_Sprite_Spray: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                    _: *const libc::c_float,
                                                    _: libc::c_int,
                                                    _: libc::c_int,
                                                    _: libc::c_int,
                                                    _: libc::c_int) -> ()>,
    pub R_Sprite_Trail: Option<unsafe extern "C" fn(_: libc::c_int,
                                                    _: *mut libc::c_float,
                                                    _: *mut libc::c_float,
                                                    _: libc::c_int,
                                                    _: libc::c_int,
                                                    _: libc::c_float,
                                                    _: libc::c_float,
                                                    _: libc::c_float,
                                                    _: libc::c_int,
                                                    _: libc::c_float) -> ()>,
    pub R_Sprite_WallPuff: Option<unsafe extern "C" fn(_: *mut TEMPENTITY,
                                                       _: libc::c_float)
                                      -> ()>,
    pub R_StreakSplash: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                    _: *const libc::c_float,
                                                    _: libc::c_int,
                                                    _: libc::c_int,
                                                    _: libc::c_float,
                                                    _: libc::c_int,
                                                    _: libc::c_int) -> ()>,
    pub R_TracerEffect: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                    _: *const libc::c_float)
                                   -> ()>,
    pub R_UserTracerParticle: Option<unsafe extern "C" fn(_:
                                                              *mut libc::c_float,
                                                          _:
                                                              *mut libc::c_float,
                                                          _: libc::c_float,
                                                          _: libc::c_int,
                                                          _: libc::c_float,
                                                          _: libc::c_uchar,
                                                          _:
                                                              Option<unsafe extern "C" fn(_:
                                                                                              *mut particle_s)
                                                                         ->
                                                                             ()>)
                                         -> ()>,
    pub R_TracerParticles: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                       _: *mut libc::c_float,
                                                       _: libc::c_float)
                                      -> *mut particle_t>,
    pub R_TeleportSplash: Option<unsafe extern "C" fn(_: *const libc::c_float)
                                     -> ()>,
    pub R_TempSphereModel: Option<unsafe extern "C" fn(_:
                                                           *const libc::c_float,
                                                       _: libc::c_float,
                                                       _: libc::c_float,
                                                       _: libc::c_int,
                                                       _: libc::c_int) -> ()>,
    pub R_TempModel: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                 _: *const libc::c_float,
                                                 _: *const libc::c_float,
                                                 _: libc::c_float,
                                                 _: libc::c_int,
                                                 _: libc::c_int)
                                -> *mut TEMPENTITY>,
    pub R_DefaultSprite: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                     _: libc::c_int,
                                                     _: libc::c_float)
                                    -> *mut TEMPENTITY>,
    pub R_TempSprite: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                  _: *const libc::c_float,
                                                  _: libc::c_float,
                                                  _: libc::c_int,
                                                  _: libc::c_int,
                                                  _: libc::c_int,
                                                  _: libc::c_float,
                                                  _: libc::c_float,
                                                  _: libc::c_int)
                                 -> *mut TEMPENTITY>,
    pub Draw_DecalIndex: Option<unsafe extern "C" fn(_: libc::c_int)
                                    -> libc::c_int>,
    pub Draw_DecalIndexFromName: Option<unsafe extern "C" fn(_:
                                                                 *const libc::c_char)
                                            -> libc::c_int>,
    pub R_DecalShoot: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _: libc::c_int,
                                                  _: libc::c_int,
                                                  _: *mut libc::c_float,
                                                  _: libc::c_int) -> ()>,
    pub R_AttachTentToPlayer: Option<unsafe extern "C" fn(_: libc::c_int,
                                                          _: libc::c_int,
                                                          _: libc::c_float,
                                                          _: libc::c_float)
                                         -> ()>,
    pub R_KillAttachedTents: Option<unsafe extern "C" fn(_: libc::c_int)
                                        -> ()>,
    pub R_BeamCirclePoints: Option<unsafe extern "C" fn(_: libc::c_int,
                                                        _: *mut libc::c_float,
                                                        _: *mut libc::c_float,
                                                        _: libc::c_int,
                                                        _: libc::c_float,
                                                        _: libc::c_float,
                                                        _: libc::c_float,
                                                        _: libc::c_float,
                                                        _: libc::c_float,
                                                        _: libc::c_int,
                                                        _: libc::c_float,
                                                        _: libc::c_float,
                                                        _: libc::c_float,
                                                        _: libc::c_float)
                                       -> *mut BEAM>,
    pub R_BeamEntPoint: Option<unsafe extern "C" fn(_: libc::c_int,
                                                    _: *mut libc::c_float,
                                                    _: libc::c_int,
                                                    _: libc::c_float,
                                                    _: libc::c_float,
                                                    _: libc::c_float,
                                                    _: libc::c_float,
                                                    _: libc::c_float,
                                                    _: libc::c_int,
                                                    _: libc::c_float,
                                                    _: libc::c_float,
                                                    _: libc::c_float,
                                                    _: libc::c_float)
                                   -> *mut BEAM>,
    pub R_BeamEnts: Option<unsafe extern "C" fn(_: libc::c_int,
                                                _: libc::c_int,
                                                _: libc::c_int,
                                                _: libc::c_float,
                                                _: libc::c_float,
                                                _: libc::c_float,
                                                _: libc::c_float,
                                                _: libc::c_float,
                                                _: libc::c_int,
                                                _: libc::c_float,
                                                _: libc::c_float,
                                                _: libc::c_float,
                                                _: libc::c_float)
                               -> *mut BEAM>,
    pub R_BeamFollow: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _: libc::c_int,
                                                  _: libc::c_float,
                                                  _: libc::c_float,
                                                  _: libc::c_float,
                                                  _: libc::c_float,
                                                  _: libc::c_float,
                                                  _: libc::c_float)
                                 -> *mut BEAM>,
    pub R_BeamKill: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub R_BeamLightning: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                     _: *mut libc::c_float,
                                                     _: libc::c_int,
                                                     _: libc::c_float,
                                                     _: libc::c_float,
                                                     _: libc::c_float,
                                                     _: libc::c_float,
                                                     _: libc::c_float)
                                    -> *mut BEAM>,
    pub R_BeamPoints: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                  _: *mut libc::c_float,
                                                  _: libc::c_int,
                                                  _: libc::c_float,
                                                  _: libc::c_float,
                                                  _: libc::c_float,
                                                  _: libc::c_float,
                                                  _: libc::c_float,
                                                  _: libc::c_int,
                                                  _: libc::c_float,
                                                  _: libc::c_float,
                                                  _: libc::c_float,
                                                  _: libc::c_float)
                                 -> *mut BEAM>,
    pub R_BeamRing: Option<unsafe extern "C" fn(_: libc::c_int,
                                                _: libc::c_int,
                                                _: libc::c_int,
                                                _: libc::c_float,
                                                _: libc::c_float,
                                                _: libc::c_float,
                                                _: libc::c_float,
                                                _: libc::c_float,
                                                _: libc::c_int,
                                                _: libc::c_float,
                                                _: libc::c_float,
                                                _: libc::c_float,
                                                _: libc::c_float)
                               -> *mut BEAM>,
    pub CL_AllocDlight: Option<unsafe extern "C" fn(_: libc::c_int)
                                   -> *mut dlight_t>,
    pub CL_AllocElight: Option<unsafe extern "C" fn(_: libc::c_int)
                                   -> *mut dlight_t>,
    pub CL_TempEntAlloc: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                     _: *mut model_s)
                                    -> *mut TEMPENTITY>,
    pub CL_TempEntAllocNoModel: Option<unsafe extern "C" fn(_:
                                                                *const libc::c_float)
                                           -> *mut TEMPENTITY>,
    pub CL_TempEntAllocHigh: Option<unsafe extern "C" fn(_:
                                                             *const libc::c_float,
                                                         _: *mut model_s)
                                        -> *mut TEMPENTITY>,
    pub CL_TentEntAllocCustom: Option<unsafe extern "C" fn(_:
                                                               *const libc::c_float,
                                                           _: *mut model_s,
                                                           _: libc::c_int,
                                                           _:
                                                               Option<unsafe extern "C" fn(_:
                                                                                               *mut tempent_s,
                                                                                           _:
                                                                                               libc::c_float,
                                                                                           _:
                                                                                               libc::c_float)
                                                                          ->
                                                                              ()>)
                                          -> *mut TEMPENTITY>,
    pub R_GetPackedColor: Option<unsafe extern "C" fn(_: *mut libc::c_short,
                                                      _: libc::c_short)
                                     -> ()>,
    pub R_LookupColor: Option<unsafe extern "C" fn(_: libc::c_uchar,
                                                   _: libc::c_uchar,
                                                   _: libc::c_uchar)
                                  -> libc::c_short>,
    pub R_DecalRemoveAll: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub R_FireCustomDecal: Option<unsafe extern "C" fn(_: libc::c_int,
                                                       _: libc::c_int,
                                                       _: libc::c_int,
                                                       _: *mut libc::c_float,
                                                       _: libc::c_int,
                                                       _: libc::c_float)
                                      -> ()>,
}
pub type TEMPENTITY = tempent_s;
pub type BEAM = beam_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct beam_s {
    pub next: *mut BEAM,
    pub type_0: libc::c_int,
    pub flags: libc::c_int,
    pub source: vec3_t,
    pub target: vec3_t,
    pub delta: vec3_t,
    pub t: libc::c_float,
    pub freq: libc::c_float,
    pub die: libc::c_float,
    pub width: libc::c_float,
    pub amplitude: libc::c_float,
    pub r: libc::c_float,
    pub g: libc::c_float,
    pub b: libc::c_float,
    pub brightness: libc::c_float,
    pub speed: libc::c_float,
    pub frameRate: libc::c_float,
    pub frame: libc::c_float,
    pub segments: libc::c_int,
    pub startEntity: libc::c_int,
    pub endEntity: libc::c_int,
    pub modelIndex: libc::c_int,
    pub frameCount: libc::c_int,
    pub pFollowModel: *mut model_s,
    pub particles: *mut particle_s,
}
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
pub type particle_t = particle_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct triangleapi_s {
    pub version: libc::c_int,
    pub RenderMode: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub Begin: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub End: Option<unsafe extern "C" fn() -> ()>,
    pub Color4f: Option<unsafe extern "C" fn(_: libc::c_float,
                                             _: libc::c_float,
                                             _: libc::c_float,
                                             _: libc::c_float) -> ()>,
    pub Color4ub: Option<unsafe extern "C" fn(_: libc::c_uchar,
                                              _: libc::c_uchar,
                                              _: libc::c_uchar,
                                              _: libc::c_uchar) -> ()>,
    pub TexCoord2f: Option<unsafe extern "C" fn(_: libc::c_float,
                                                _: libc::c_float) -> ()>,
    pub Vertex3fv: Option<unsafe extern "C" fn(_: *const libc::c_float)
                              -> ()>,
    pub Vertex3f: Option<unsafe extern "C" fn(_: libc::c_float,
                                              _: libc::c_float,
                                              _: libc::c_float) -> ()>,
    pub Brightness: Option<unsafe extern "C" fn(_: libc::c_float) -> ()>,
    pub CullFace: Option<unsafe extern "C" fn(_: TRICULLSTYLE) -> ()>,
    pub SpriteTexture: Option<unsafe extern "C" fn(_: *mut model_s,
                                                   _: libc::c_int)
                                  -> libc::c_int>,
    pub WorldToScreen: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                   _: *mut libc::c_float)
                                  -> libc::c_int>,
    pub Fog: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                         _: libc::c_float, _: libc::c_float,
                                         _: libc::c_int) -> ()>,
    pub ScreenToWorld: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                   _: *mut libc::c_float)
                                  -> ()>,
    pub GetMatrix: Option<unsafe extern "C" fn(_: libc::c_int,
                                               _: *mut libc::c_float) -> ()>,
    pub BoxInPVS: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                              _: *mut libc::c_float)
                             -> libc::c_int>,
    pub LightAtPoint: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                  _: *mut libc::c_float)
                                 -> ()>,
    pub Color4fRendermode: Option<unsafe extern "C" fn(_: libc::c_float,
                                                       _: libc::c_float,
                                                       _: libc::c_float,
                                                       _: libc::c_float,
                                                       _: libc::c_int) -> ()>,
    pub FogParams: Option<unsafe extern "C" fn(_: libc::c_float,
                                               _: libc::c_int) -> ()>,
}
pub type TRICULLSTYLE = libc::c_uint;
pub const TRI_NONE: TRICULLSTYLE = 1;
pub const TRI_FRONT: TRICULLSTYLE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct screenfade_s {
    pub fadeSpeed: libc::c_float,
    pub fadeEnd: libc::c_float,
    pub fadeTotalEnd: libc::c_float,
    pub fadeReset: libc::c_float,
    pub fader: byte,
    pub fadeg: byte,
    pub fadeb: byte,
    pub fadealpha: byte,
    pub fadeFlags: libc::c_int,
}
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
pub type HSPRITE = libc::c_int;
pub type client_textmessage_t = client_textmessage_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client_textmessage_s {
    pub effect: libc::c_int,
    pub r1: byte,
    pub g1: byte,
    pub b1: byte,
    pub a1: byte,
    pub r2: byte,
    pub g2: byte,
    pub b2: byte,
    pub a2: byte,
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub fadein: libc::c_float,
    pub fadeout: libc::c_float,
    pub holdtime: libc::c_float,
    pub fxtime: libc::c_float,
    pub pName: *const libc::c_char,
    pub pMessage: *const libc::c_char,
}
pub type hud_player_info_t = hud_player_info_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hud_player_info_s {
    pub name: *mut libc::c_char,
    pub ping: libc::c_short,
    pub thisplayer: byte,
    pub spectator: byte,
    pub packetloss: byte,
    pub model: *mut libc::c_char,
    pub topcolor: libc::c_short,
    pub bottomcolor: libc::c_short,
    pub m_nSteamID: uint64_t,
}
pub type pfnUserMsgHook
    =
    Option<unsafe extern "C" fn(_: *const libc::c_char, _: libc::c_int,
                                _: *mut libc::c_void) -> libc::c_int>;
pub type SCREENINFO = SCREENINFO_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCREENINFO_s {
    pub iSize: libc::c_int,
    pub iWidth: libc::c_int,
    pub iHeight: libc::c_int,
    pub iFlags: libc::c_int,
    pub iCharHeight: libc::c_int,
    pub charWidths: [libc::c_short; 256],
}
pub type client_sprite_t = client_sprite_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client_sprite_s {
    pub szName: [libc::c_char; 64],
    pub szSprite: [libc::c_char; 64],
    pub hspr: libc::c_int,
    pub iRes: libc::c_int,
    pub rc: wrect_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clgame_static_t {
    pub hInstance: *mut libc::c_void,
    pub dllFuncs: cldll_func_t,
    pub drawFuncs: render_interface_t,
    pub mempool: poolhandle_t,
    pub mapname: string,
    pub maptitle: string,
    pub itemspath: string,
    pub entities: *mut cl_entity_t,
    pub static_entities: *mut cl_entity_t,
    pub remap_info: *mut *mut remap_info_t,
    pub maxEntities: libc::c_int,
    pub maxRemapInfos: libc::c_int,
    pub numStatics: libc::c_int,
    pub maxModels: libc::c_int,
    pub movevars: movevars_t,
    pub oldmovevars: movevars_t,
    pub pmove: *mut playermove_t,
    pub pushed: qboolean,
    pub oldviscount: libc::c_int,
    pub oldphyscount: libc::c_int,
    pub msg: [cl_user_message_t; 197],
    pub events: [*mut cl_user_event_t; 1024],
    pub cdtracks: [string; 32],
    pub sprites: [model_t; 256],
    pub viewport: [libc::c_int; 4],
    pub ds: client_draw_t,
    pub fade: screenfade_t,
    pub shake: screen_shake_t,
    pub centerPrint: center_print_t,
    pub scrInfo: SCREENINFO,
    pub overView: ref_overview_t,
    pub palette: [color24; 256],
    pub sprlist: [cached_spritelist_t; 256],
    pub titles: *mut client_textmessage_t,
    pub numTitles: libc::c_int,
    pub request_type: net_request_type_t,
    pub net_requests: [net_request_t; 64],
    pub master_request: *mut net_request_t,
    pub free_efrags: *mut efrag_t,
    pub viewent: cl_entity_t,
    pub client_dll_uses_sdl: qboolean,
}
pub type efrag_t = efrag_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct net_request_t {
    pub resp: net_response_t,
    pub pfnFunc: net_api_response_func_t,
    pub timeout: libc::c_double,
    pub timesend: libc::c_double,
    pub flags: libc::c_int,
}
pub type net_response_t = net_response_s;
pub type net_request_type_t = libc::c_uint;
pub const NET_REQUEST_CLIENT: net_request_type_t = 2;
pub const NET_REQUEST_GAMEUI: net_request_type_t = 1;
pub const NET_REQUEST_CANCEL: net_request_type_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cached_spritelist_t {
    pub szListName: [libc::c_char; 64],
    pub pList: *mut client_sprite_t,
    pub count: libc::c_int,
}
pub type ref_overview_t = ref_overview_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct center_print_t {
    pub time: libc::c_float,
    pub y: libc::c_int,
    pub lines: libc::c_int,
    pub message: [libc::c_char; 2048],
    pub totalWidth: libc::c_int,
    pub totalHeight: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct screen_shake_t {
    pub time: libc::c_float,
    pub duration: libc::c_float,
    pub amplitude: libc::c_float,
    pub frequency: libc::c_float,
    pub next_shake: libc::c_float,
    pub offset: vec3_t,
    pub angle: libc::c_float,
    pub applied_offset: vec3_t,
    pub applied_angle: libc::c_float,
}
pub type screenfade_t = screenfade_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client_draw_t {
    pub pSprite: *const model_t,
    pub scissor_x: libc::c_int,
    pub scissor_y: libc::c_int,
    pub scissor_width: libc::c_int,
    pub scissor_height: libc::c_int,
    pub scissor_test: qboolean,
    pub adjust_size: qboolean,
    pub renderMode: libc::c_int,
    pub cullMode: TRICULLSTYLE,
    pub textColor: rgba_t,
    pub spriteColor: rgba_t,
    pub triRGBA: vec4_t,
    pub pCrosshair: *const model_t,
    pub rcCrosshair: wrect_t,
    pub rgbaCrosshair: rgba_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cl_user_event_t {
    pub name: [libc::c_char; 64],
    pub index: word,
    pub func: pfnEventHook,
}
pub type pfnEventHook
    =
    Option<unsafe extern "C" fn(_: *mut event_args_t) -> ()>;
pub type event_args_t = event_args_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cl_user_message_t {
    pub name: [libc::c_char; 32],
    pub number: libc::c_int,
    pub size: libc::c_int,
    pub func: pfnUserMsgHook,
}
pub type playermove_t = playermove_s;
pub type movevars_t = movevars_s;
pub type remap_info_t = remap_info_s;
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
pub type local_state_t = local_state_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpoint_t {
    pub point: vec2_t,
    pub coord: vec2_t,
}
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
pub type C2RustUnnamed_1 = libc::c_uint;
pub const DEMO_QUAKE1: C2RustUnnamed_1 = 2;
pub const DEMO_XASH3D: C2RustUnnamed_1 = 1;
pub const DEMO_INACTIVE: C2RustUnnamed_1 = 0;
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
pub type connstate_e = libc::c_uint;
pub const ca_cinematic: connstate_e = 5;
pub const ca_active: connstate_e = 4;
pub const ca_validate: connstate_e = 3;
pub const ca_connected: connstate_e = 2;
pub const ca_connecting: connstate_e = 1;
pub const ca_disconnected: connstate_e = 0;
pub type connstate_t = connstate_e;
pub type ref_screen_rotation_e = libc::c_uint;
pub const REF_ROTATE_CCW: ref_screen_rotation_e = 3;
pub const REF_ROTATE_UD: ref_screen_rotation_e = 2;
pub const REF_ROTATE_CW: ref_screen_rotation_e = 1;
pub const REF_ROTATE_NONE: ref_screen_rotation_e = 0;
pub type ref_screen_rotation_t = ref_screen_rotation_e;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ref_interface_s {
    pub R_Init: Option<unsafe extern "C" fn() -> qboolean>,
    pub R_Shutdown: Option<unsafe extern "C" fn() -> ()>,
    pub R_GetConfigName: Option<unsafe extern "C" fn()
                                    -> *const libc::c_char>,
    pub R_SetDisplayTransform: Option<unsafe extern "C" fn(_:
                                                               ref_screen_rotation_t,
                                                           _: libc::c_int,
                                                           _: libc::c_int,
                                                           _: libc::c_float,
                                                           _: libc::c_float)
                                          -> qboolean>,
    pub GL_SetupAttributes: Option<unsafe extern "C" fn(_: libc::c_int)
                                       -> ()>,
    pub GL_InitExtensions: Option<unsafe extern "C" fn() -> ()>,
    pub GL_ClearExtensions: Option<unsafe extern "C" fn() -> ()>,
    pub R_BeginFrame: Option<unsafe extern "C" fn(_: qboolean) -> ()>,
    pub R_RenderScene: Option<unsafe extern "C" fn() -> ()>,
    pub R_EndFrame: Option<unsafe extern "C" fn() -> ()>,
    pub R_PushScene: Option<unsafe extern "C" fn() -> ()>,
    pub R_PopScene: Option<unsafe extern "C" fn() -> ()>,
    pub GL_BackendStartFrame: Option<unsafe extern "C" fn() -> ()>,
    pub GL_BackendEndFrame: Option<unsafe extern "C" fn() -> ()>,
    pub R_ClearScreen: Option<unsafe extern "C" fn() -> ()>,
    pub R_AllowFog: Option<unsafe extern "C" fn(_: qboolean) -> ()>,
    pub GL_SetRenderMode: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub R_AddEntity: Option<unsafe extern "C" fn(_: *mut cl_entity_s,
                                                 _: libc::c_int) -> qboolean>,
    pub CL_AddCustomBeam: Option<unsafe extern "C" fn(_: *mut cl_entity_t)
                                     -> ()>,
    pub R_ProcessEntData: Option<unsafe extern "C" fn(_: qboolean) -> ()>,
    pub R_ShowTextures: Option<unsafe extern "C" fn() -> ()>,
    pub R_GetTextureOriginalBuffer: Option<unsafe extern "C" fn(_:
                                                                    libc::c_uint)
                                               -> *const byte>,
    pub GL_LoadTextureFromBuffer: Option<unsafe extern "C" fn(_:
                                                                  *const libc::c_char,
                                                              _:
                                                                  *mut rgbdata_t,
                                                              _: texFlags_t,
                                                              _: qboolean)
                                             -> libc::c_int>,
    pub GL_ProcessTexture: Option<unsafe extern "C" fn(_: libc::c_int,
                                                       _: libc::c_float,
                                                       _: libc::c_int,
                                                       _: libc::c_int) -> ()>,
    pub R_SetupSky: Option<unsafe extern "C" fn(_: *const libc::c_char)
                               -> ()>,
    pub R_Set2DMode: Option<unsafe extern "C" fn(_: qboolean) -> ()>,
    pub R_DrawStretchRaw: Option<unsafe extern "C" fn(_: libc::c_float,
                                                      _: libc::c_float,
                                                      _: libc::c_float,
                                                      _: libc::c_float,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: *const byte,
                                                      _: qboolean) -> ()>,
    pub R_DrawStretchPic: Option<unsafe extern "C" fn(_: libc::c_float,
                                                      _: libc::c_float,
                                                      _: libc::c_float,
                                                      _: libc::c_float,
                                                      _: libc::c_float,
                                                      _: libc::c_float,
                                                      _: libc::c_float,
                                                      _: libc::c_float,
                                                      _: libc::c_int) -> ()>,
    pub R_DrawTileClear: Option<unsafe extern "C" fn(_: libc::c_int,
                                                     _: libc::c_int,
                                                     _: libc::c_int,
                                                     _: libc::c_int,
                                                     _: libc::c_int) -> ()>,
    pub FillRGBA: Option<unsafe extern "C" fn(_: libc::c_float,
                                              _: libc::c_float,
                                              _: libc::c_float,
                                              _: libc::c_float,
                                              _: libc::c_int, _: libc::c_int,
                                              _: libc::c_int, _: libc::c_int)
                             -> ()>,
    pub FillRGBABlend: Option<unsafe extern "C" fn(_: libc::c_float,
                                                   _: libc::c_float,
                                                   _: libc::c_float,
                                                   _: libc::c_float,
                                                   _: libc::c_int,
                                                   _: libc::c_int,
                                                   _: libc::c_int,
                                                   _: libc::c_int) -> ()>,
    pub WorldToScreen: Option<unsafe extern "C" fn(_: *const vec_t,
                                                   _: *mut vec_t)
                                  -> libc::c_int>,
    pub VID_ScreenShot: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                    _: libc::c_int)
                                   -> qboolean>,
    pub VID_CubemapShot: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                     _: uint,
                                                     _: *const libc::c_float,
                                                     _: qboolean)
                                    -> qboolean>,
    pub R_LightPoint: Option<unsafe extern "C" fn(_: *const libc::c_float)
                                 -> colorVec>,
    pub R_DecalShoot: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _: libc::c_int,
                                                  _: libc::c_int,
                                                  _: *mut vec_t,
                                                  _: libc::c_int,
                                                  _: libc::c_float) -> ()>,
    pub R_DecalRemoveAll: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub R_CreateDecalList: Option<unsafe extern "C" fn(_: *mut decallist_s)
                                      -> libc::c_int>,
    pub R_ClearAllDecals: Option<unsafe extern "C" fn() -> ()>,
    pub R_StudioEstimateFrame: Option<unsafe extern "C" fn(_:
                                                               *mut cl_entity_t,
                                                           _:
                                                               *mut mstudioseqdesc_t)
                                          -> libc::c_float>,
    pub R_StudioLerpMovement: Option<unsafe extern "C" fn(_: *mut cl_entity_t,
                                                          _: libc::c_double,
                                                          _: *mut vec_t,
                                                          _: *mut vec_t)
                                         -> ()>,
    pub CL_InitStudioAPI: Option<unsafe extern "C" fn() -> ()>,
    pub R_InitSkyClouds: Option<unsafe extern "C" fn(_: *mut mip_s,
                                                     _: *mut texture_s,
                                                     _: qboolean) -> ()>,
    pub GL_SubdivideSurface: Option<unsafe extern "C" fn(_: *mut msurface_t)
                                        -> ()>,
    pub CL_RunLightStyles: Option<unsafe extern "C" fn() -> ()>,
    pub R_GetSpriteParms: Option<unsafe extern "C" fn(_: *mut libc::c_int,
                                                      _: *mut libc::c_int,
                                                      _: *mut libc::c_int,
                                                      _: libc::c_int,
                                                      _: *const model_t)
                                     -> ()>,
    pub R_GetSpriteTexture: Option<unsafe extern "C" fn(_: *const model_t,
                                                        _: libc::c_int)
                                       -> libc::c_int>,
    pub Mod_LoadMapSprite: Option<unsafe extern "C" fn(_: *mut model_s,
                                                       _: *const libc::c_void,
                                                       _: size_t,
                                                       _: *mut qboolean)
                                      -> ()>,
    pub Mod_ProcessRenderData: Option<unsafe extern "C" fn(_: *mut model_t,
                                                           _: qboolean,
                                                           _: *const byte)
                                          -> qboolean>,
    pub Mod_StudioLoadTextures: Option<unsafe extern "C" fn(_: *mut model_t,
                                                            _:
                                                                *mut libc::c_void)
                                           -> ()>,
    pub CL_DrawParticles: Option<unsafe extern "C" fn(_: libc::c_double,
                                                      _: *mut particle_t,
                                                      _: libc::c_float)
                                     -> ()>,
    pub CL_DrawTracers: Option<unsafe extern "C" fn(_: libc::c_double,
                                                    _: *mut particle_t)
                                   -> ()>,
    pub CL_DrawBeams: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _: *mut BEAM) -> ()>,
    pub R_BeamCull: Option<unsafe extern "C" fn(_: *const vec_t,
                                                _: *const vec_t, _: qboolean)
                               -> qboolean>,
    pub RefGetParm: Option<unsafe extern "C" fn(_: libc::c_int,
                                                _: libc::c_int)
                               -> libc::c_int>,
    pub GetDetailScaleForTexture: Option<unsafe extern "C" fn(_: libc::c_int,
                                                              _:
                                                                  *mut libc::c_float,
                                                              _:
                                                                  *mut libc::c_float)
                                             -> ()>,
    pub GetExtraParmsForTexture: Option<unsafe extern "C" fn(_: libc::c_int,
                                                             _: *mut byte,
                                                             _: *mut byte,
                                                             _: *mut byte,
                                                             _: *mut byte)
                                            -> ()>,
    pub GetFrameTime: Option<unsafe extern "C" fn() -> libc::c_float>,
    pub R_SetCurrentEntity: Option<unsafe extern "C" fn(_: *mut cl_entity_s)
                                       -> ()>,
    pub R_SetCurrentModel: Option<unsafe extern "C" fn(_: *mut model_s)
                                      -> ()>,
    pub GL_FindTexture: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                   -> libc::c_int>,
    pub GL_TextureName: Option<unsafe extern "C" fn(_: libc::c_uint)
                                   -> *const libc::c_char>,
    pub GL_TextureData: Option<unsafe extern "C" fn(_: libc::c_uint)
                                   -> *const byte>,
    pub GL_LoadTexture: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                    _: *const byte, _: size_t,
                                                    _: libc::c_int)
                                   -> libc::c_int>,
    pub GL_CreateTexture: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: *const libc::c_void,
                                                      _: texFlags_t)
                                     -> libc::c_int>,
    pub GL_LoadTextureArray: Option<unsafe extern "C" fn(_:
                                                             *mut *const libc::c_char,
                                                         _: libc::c_int)
                                        -> libc::c_int>,
    pub GL_CreateTextureArray: Option<unsafe extern "C" fn(_:
                                                               *const libc::c_char,
                                                           _: libc::c_int,
                                                           _: libc::c_int,
                                                           _: libc::c_int,
                                                           _:
                                                               *const libc::c_void,
                                                           _: texFlags_t)
                                          -> libc::c_int>,
    pub GL_FreeTexture: Option<unsafe extern "C" fn(_: libc::c_uint) -> ()>,
    pub DrawSingleDecal: Option<unsafe extern "C" fn(_: *mut decal_s,
                                                     _: *mut msurface_s)
                                    -> ()>,
    pub R_DecalSetupVerts: Option<unsafe extern "C" fn(_: *mut decal_s,
                                                       _: *mut msurface_s,
                                                       _: libc::c_int,
                                                       _: *mut libc::c_int)
                                      -> *mut libc::c_float>,
    pub R_EntityRemoveDecals: Option<unsafe extern "C" fn(_: *mut model_s)
                                         -> ()>,
    pub AVI_UploadRawFrame: Option<unsafe extern "C" fn(_: libc::c_int,
                                                        _: libc::c_int,
                                                        _: libc::c_int,
                                                        _: libc::c_int,
                                                        _: libc::c_int,
                                                        _: *const byte)
                                       -> ()>,
    pub GL_Bind: Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_uint)
                            -> ()>,
    pub GL_SelectTexture: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub GL_LoadTextureMatrix: Option<unsafe extern "C" fn(_:
                                                              *const libc::c_float)
                                         -> ()>,
    pub GL_TexMatrixIdentity: Option<unsafe extern "C" fn() -> ()>,
    pub GL_CleanUpTextureUnits: Option<unsafe extern "C" fn(_: libc::c_int)
                                           -> ()>,
    pub GL_TexGen: Option<unsafe extern "C" fn(_: libc::c_uint,
                                               _: libc::c_uint) -> ()>,
    pub GL_TextureTarget: Option<unsafe extern "C" fn(_: libc::c_uint) -> ()>,
    pub GL_TexCoordArrayMode: Option<unsafe extern "C" fn(_: libc::c_uint)
                                         -> ()>,
    pub GL_UpdateTexSize: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int) -> ()>,
    pub GL_Reserved0: Option<unsafe extern "C" fn() -> ()>,
    pub GL_Reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub GL_DrawParticles: Option<unsafe extern "C" fn(_:
                                                          *const ref_viewpass_s,
                                                      _: qboolean,
                                                      _: libc::c_float)
                                     -> ()>,
    pub LightVec: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                              _: *const libc::c_float,
                                              _: *mut libc::c_float,
                                              _: *mut libc::c_float)
                             -> colorVec>,
    pub StudioGetTexture: Option<unsafe extern "C" fn(_: *mut cl_entity_s)
                                     -> *mut mstudiotex_s>,
    pub GL_RenderFrame: Option<unsafe extern "C" fn(_: *const ref_viewpass_s)
                                   -> ()>,
    pub GL_OrthoBounds: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                    _: *const libc::c_float)
                                   -> ()>,
    pub R_SpeedsMessage: Option<unsafe extern "C" fn(_: *mut libc::c_char,
                                                     _: size_t) -> qboolean>,
    pub Mod_GetCurrentVis: Option<unsafe extern "C" fn() -> *mut byte>,
    pub R_NewMap: Option<unsafe extern "C" fn() -> ()>,
    pub R_ClearScene: Option<unsafe extern "C" fn() -> ()>,
    pub R_GetProcAddress: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                     -> *mut libc::c_void>,
    pub TriRenderMode: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub Begin: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub End: Option<unsafe extern "C" fn() -> ()>,
    pub Color4f: Option<unsafe extern "C" fn(_: libc::c_float,
                                             _: libc::c_float,
                                             _: libc::c_float,
                                             _: libc::c_float) -> ()>,
    pub Color4ub: Option<unsafe extern "C" fn(_: libc::c_uchar,
                                              _: libc::c_uchar,
                                              _: libc::c_uchar,
                                              _: libc::c_uchar) -> ()>,
    pub TexCoord2f: Option<unsafe extern "C" fn(_: libc::c_float,
                                                _: libc::c_float) -> ()>,
    pub Vertex3fv: Option<unsafe extern "C" fn(_: *const libc::c_float)
                              -> ()>,
    pub Vertex3f: Option<unsafe extern "C" fn(_: libc::c_float,
                                              _: libc::c_float,
                                              _: libc::c_float) -> ()>,
    pub Fog: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                         _: libc::c_float, _: libc::c_float,
                                         _: libc::c_int) -> ()>,
    pub ScreenToWorld: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                   _: *mut libc::c_float)
                                  -> ()>,
    pub GetMatrix: Option<unsafe extern "C" fn(_: libc::c_int,
                                               _: *mut libc::c_float) -> ()>,
    pub FogParams: Option<unsafe extern "C" fn(_: libc::c_float,
                                               _: libc::c_int) -> ()>,
    pub CullFace: Option<unsafe extern "C" fn(_: TRICULLSTYLE) -> ()>,
    pub VGUI_DrawInit: Option<unsafe extern "C" fn() -> ()>,
    pub VGUI_DrawShutdown: Option<unsafe extern "C" fn() -> ()>,
    pub VGUI_SetupDrawingText: Option<unsafe extern "C" fn(_:
                                                               *mut libc::c_int)
                                          -> ()>,
    pub VGUI_SetupDrawingRect: Option<unsafe extern "C" fn(_:
                                                               *mut libc::c_int)
                                          -> ()>,
    pub VGUI_SetupDrawingImage: Option<unsafe extern "C" fn(_:
                                                                *mut libc::c_int)
                                           -> ()>,
    pub VGUI_BindTexture: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub VGUI_EnableTexture: Option<unsafe extern "C" fn(_: qboolean) -> ()>,
    pub VGUI_CreateTexture: Option<unsafe extern "C" fn(_: libc::c_int,
                                                        _: libc::c_int,
                                                        _: libc::c_int)
                                       -> ()>,
    pub VGUI_UploadTexture: Option<unsafe extern "C" fn(_: libc::c_int,
                                                        _:
                                                            *const libc::c_char,
                                                        _: libc::c_int,
                                                        _: libc::c_int)
                                       -> ()>,
    pub VGUI_UploadTextureBlock: Option<unsafe extern "C" fn(_: libc::c_int,
                                                             _: libc::c_int,
                                                             _: libc::c_int,
                                                             _: *const byte,
                                                             _: libc::c_int,
                                                             _: libc::c_int)
                                            -> ()>,
    pub VGUI_DrawQuad: Option<unsafe extern "C" fn(_: *const vpoint_t,
                                                   _: *const vpoint_t) -> ()>,
    pub VGUI_GetTextureSizes: Option<unsafe extern "C" fn(_: *mut libc::c_int,
                                                          _: *mut libc::c_int)
                                         -> ()>,
    pub VGUI_GenerateTexture: Option<unsafe extern "C" fn() -> libc::c_int>,
}
pub type ref_interface_t = ref_interface_s;
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
pub struct netsplit_chain_packet_s {
    pub recieved_v: [uint32_t; 8],
    pub id: uint32_t,
    pub data: [byte; 131072],
    pub received: byte,
    pub count: byte,
}
pub type netsplit_chain_packet_t = netsplit_chain_packet_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct netsplit_s {
    pub packets: [netsplit_chain_packet_t; 8],
    pub total_received: uint64_t,
    pub total_received_uncompressed: uint64_t,
}
pub type netsplit_t = netsplit_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flowstats_t {
    pub size: libc::c_int,
    pub time: libc::c_double,
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
pub type fragbuf_t = fragbuf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fbufqueue_s {
    pub next: *mut fbufqueue_s,
    pub fragbufcount: libc::c_int,
    pub fragbufs: *mut fragbuf_t,
}
pub type fragbufwaiting_t = fbufqueue_s;
pub type fragsize_e = libc::c_uint;
pub const FRAGSIZE_UNRELIABLE: fragsize_e = 2;
pub const FRAGSIZE_SPLIT: fragsize_e = 1;
pub const FRAGSIZE_FRAG: fragsize_e = 0;
pub type fragsize_t = fragsize_e;
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
pub type netchan_t = netchan_s;
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
pub type event_info_t = event_info_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_state_s {
    pub ei: [event_info_t; 64],
}
pub type event_state_t = event_state_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ref_state_s {
    pub initialized: qboolean,
    pub hInstance: HINSTANCE,
    pub dllFuncs: ref_interface_t,
    pub numRenderers: libc::c_int,
    pub shortNames: [string; 5],
    pub readableNames: [string; 5],
}
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
pub type netbandwidthgraph_t = netbandwithgraph_s;
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
pub type frame_t = frame_s;
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
pub type runcmd_t = runcmd_s;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct downloadtime_t {
    pub bUsed: qboolean,
    pub fTime: libc::c_float,
    pub nBytesRemaining: libc::c_int,
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
pub type scrshot_t = libc::c_uint;
pub const scrshot_mapshot: scrshot_t = 7;
pub const scrshot_skyshot: scrshot_t = 6;
pub const scrshot_envshot: scrshot_t = 5;
pub const scrshot_savegame: scrshot_t = 4;
pub const scrshot_plaque: scrshot_t = 3;
pub const scrshot_snapshot: scrshot_t = 2;
pub const scrshot_normal: scrshot_t = 1;
pub const scrshot_inactive: scrshot_t = 0;
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
pub type predicted_player_t = cl_predicted_player_s;
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
pub type voxword_t = voxword_s;
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
pub type sfx_t = sfx_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfx_s {
    pub name: [libc::c_char; 64],
    pub cache: *mut wavdata_t,
    pub servercount: libc::c_int,
    pub hashValue: uint,
    pub hashNext: *mut sfx_s,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct portable_samplepair_t {
    pub left: libc::c_int,
    pub right: libc::c_int,
}
pub const DELTA_ENTITY: C2RustUnnamed_2 = 0;
pub const DELTA_PLAYER: C2RustUnnamed_2 = 1;
pub type channel_t = channel_s;
pub type rawchan_t = rawchan_s;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const DELTA_STATIC: C2RustUnnamed_2 = 2;
#[inline(always)]
unsafe extern "C" fn __tg_sqrt(mut __x: libc::c_float) -> libc::c_float {
    return sqrtf(__x);
}
#[inline]
unsafe extern "C" fn BitByte(mut bits: libc::c_int) -> libc::c_int {
    return (bits + (8 as libc::c_int - 1 as libc::c_int)) / 8 as libc::c_int *
               8 as libc::c_int >> 3 as libc::c_int;
}
#[inline]
unsafe extern "C" fn MSG_GetNumBytesWritten(mut sb: *mut sizebuf_t)
 -> libc::c_int {
    return BitByte((*sb).iCurBit);
}
#[inline]
unsafe extern "C" fn CL_EDICT_NUM(mut n: libc::c_int) -> *mut cl_entity_t {
    if clgame.entities.is_null() {
        Host_Error(b"CL_EDICT_NUM: clgame.entities is NULL\n\x00" as *const u8
                       as *const libc::c_char);
        return 0 as *mut cl_entity_t
    }
    if n >= 0 as libc::c_int && n < clgame.maxEntities {
        return clgame.entities.offset(n as isize)
    }
    Host_Error(b"CL_EDICT_NUM: bad number %i\n\x00" as *const u8 as
                   *const libc::c_char, n);
    return 0 as *mut cl_entity_t;
}
/*
cl_frame.c - client world snapshot
Copyright (C) 2008 Uncle Mike

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
*/
/*
==================
CL_IsPlayerIndex

detect player entity
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_IsPlayerIndex(mut idx: libc::c_int) -> qboolean {
    return (idx >= 1 as libc::c_int && idx <= cl.maxclients) as libc::c_int as
               qboolean;
}
/*
=========================================================================

FRAME INTERPOLATION

=========================================================================
*/
/*
==================
CL_UpdatePositions

Store another position into interpolation circular buffer
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_UpdatePositions(mut ent: *mut cl_entity_t) {
    let mut ph: *mut position_history_t = 0 as *mut position_history_t;
    (*ent).current_position =
        (*ent).current_position + 1 as libc::c_int &
            64 as libc::c_int - 1 as libc::c_int;
    ph =
        &mut *(*ent).ph.as_mut_ptr().offset((*ent).current_position as isize)
            as *mut position_history_t;
    (*ph).origin[0 as libc::c_int as usize] =
        (*ent).curstate.origin[0 as libc::c_int as usize];
    (*ph).origin[1 as libc::c_int as usize] =
        (*ent).curstate.origin[1 as libc::c_int as usize];
    (*ph).origin[2 as libc::c_int as usize] =
        (*ent).curstate.origin[2 as libc::c_int as usize];
    (*ph).angles[0 as libc::c_int as usize] =
        (*ent).curstate.angles[0 as libc::c_int as usize];
    (*ph).angles[1 as libc::c_int as usize] =
        (*ent).curstate.angles[1 as libc::c_int as usize];
    (*ph).angles[2 as libc::c_int as usize] =
        (*ent).curstate.angles[2 as libc::c_int as usize];
    (*ph).animtime = (*ent).curstate.animtime;
    // !!!
}
/*
==================
CL_ResetPositions

Interpolation init or reset after teleporting
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ResetPositions(mut ent: *mut cl_entity_t) {
    let mut store: position_history_t =
        position_history_t{animtime: 0., origin: [0.; 3], angles: [0.; 3],};
    if ent.is_null() { return }
    store = (*ent).ph[(*ent).current_position as usize];
    (*ent).current_position = 1 as libc::c_int;
    memset((*ent).ph.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<position_history_t>() as
                libc::c_ulong).wrapping_mul(64 as libc::c_int as
                                                libc::c_ulong));
    memcpy(&mut *(*ent).ph.as_mut_ptr().offset(1 as libc::c_int as isize) as
               *mut position_history_t as *mut libc::c_void,
           &mut store as *mut position_history_t as *const libc::c_void,
           ::std::mem::size_of::<position_history_t>() as libc::c_ulong);
    memcpy(&mut *(*ent).ph.as_mut_ptr().offset(0 as libc::c_int as isize) as
               *mut position_history_t as *mut libc::c_void,
           &mut store as *mut position_history_t as *const libc::c_void,
           ::std::mem::size_of::<position_history_t>() as libc::c_ulong);
}
/*
==================
CL_EntityTeleported

check for instant movement in case
we don't want interpolate this
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_EntityTeleported(mut ent: *mut cl_entity_t)
 -> qboolean {
    let mut len: libc::c_float = 0.;
    let mut maxlen: libc::c_float = 0.;
    let mut delta: vec3_t = [0.; 3];
    delta[0 as libc::c_int as usize] =
        (*ent).curstate.origin[0 as libc::c_int as usize] -
            (*ent).prevstate.origin[0 as libc::c_int as usize];
    delta[1 as libc::c_int as usize] =
        (*ent).curstate.origin[1 as libc::c_int as usize] -
            (*ent).prevstate.origin[1 as libc::c_int as usize];
    delta[2 as libc::c_int as usize] =
        (*ent).curstate.origin[2 as libc::c_int as usize] -
            (*ent).prevstate.origin[2 as libc::c_int as usize];
    // compute potential max movement in units per frame and compare with entity movement
    maxlen = clgame.movevars.maxvelocity * (1.0f32 / 20.0f32);
    len =
        __tg_sqrt(delta[0 as libc::c_int as usize] *
                      delta[0 as libc::c_int as usize] +
                      delta[1 as libc::c_int as usize] *
                          delta[1 as libc::c_int as usize] +
                      delta[2 as libc::c_int as usize] *
                          delta[2 as libc::c_int as usize]);
    return (len > maxlen) as libc::c_int as qboolean;
}
/*
==================
CL_CompareTimestamps

round-off floating errors
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_CompareTimestamps(mut t1: libc::c_float,
                                              mut t2: libc::c_float)
 -> qboolean {
    let mut iTime1: libc::c_int =
        (t1 * 1000 as libc::c_int as libc::c_float) as libc::c_int;
    let mut iTime2: libc::c_int =
        (t2 * 1000 as libc::c_int as libc::c_float) as libc::c_int;
    return (iTime1 - iTime2 <= 1 as libc::c_int) as libc::c_int as qboolean;
}
/*
==================
CL_EntityIgnoreLerp

some ents will be ignore lerping
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_EntityIgnoreLerp(mut e: *mut cl_entity_t)
 -> qboolean {
    if (*cl_nointerp).value > 0.0f32 { return true_0 }
    if !(*e).model.is_null() &&
           (*(*e).model).type_0 as libc::c_int == mod_alias as libc::c_int {
        return false_0
    }
    return if (*e).curstate.movetype == 0 as libc::c_int {
               true_0 as libc::c_int
           } else { false_0 as libc::c_int } as qboolean;
}
/*
==================
CL_EntityCustomLerp

==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_EntityCustomLerp(mut e: *mut cl_entity_t)
 -> qboolean {
    match (*e).curstate.movetype {
        0 | 4 | 3 | 5 | 14 => { return false_0 }
        _ => { }
    }
    return true_0;
}
/*
==================
CL_ParametricMove

check for parametrical moved entities
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParametricMove(mut ent: *mut cl_entity_t)
 -> qboolean {
    let mut frac: libc::c_float = 0.; // re-aim projectile
    let mut dt: libc::c_float = 0.;
    let mut t: libc::c_float = 0.;
    let mut delta: vec3_t = [0.; 3];
    if (*ent).curstate.starttime == 0.0f32 ||
           (*ent).curstate.impacttime == 0.0f32 {
        return false_0
    }
    delta[0 as libc::c_int as usize] =
        (*ent).curstate.endpos[0 as libc::c_int as usize] -
            (*ent).curstate.startpos[0 as libc::c_int as usize];
    delta[1 as libc::c_int as usize] =
        (*ent).curstate.endpos[1 as libc::c_int as usize] -
            (*ent).curstate.startpos[1 as libc::c_int as usize];
    delta[2 as libc::c_int as usize] =
        (*ent).curstate.endpos[2 as libc::c_int as usize] -
            (*ent).curstate.startpos[2 as libc::c_int as usize];
    dt = (*ent).curstate.impacttime - (*ent).curstate.starttime;
    if dt != 0.0f32 {
        if (*ent).lastmove as libc::c_double > cl.time {
            t = (*ent).lastmove
        } else { t = cl.time as libc::c_float }
        frac = (t - (*ent).curstate.starttime) / dt;
        frac =
            if frac >= 0.0f32 {
                if frac < 1.0f32 { frac } else { 1.0f32 }
            } else { 0.0f32 };
        (*ent).curstate.origin[0 as libc::c_int as usize] =
            (*ent).curstate.startpos[0 as libc::c_int as usize] +
                frac * delta[0 as libc::c_int as usize];
        (*ent).curstate.origin[1 as libc::c_int as usize] =
            (*ent).curstate.startpos[1 as libc::c_int as usize] +
                frac * delta[1 as libc::c_int as usize];
        (*ent).curstate.origin[2 as libc::c_int as usize] =
            (*ent).curstate.startpos[2 as libc::c_int as usize] +
                frac * delta[2 as libc::c_int as usize];
        (*ent).lastmove = t
    }
    let mut ilength: libc::c_float =
        __tg_sqrt(delta[0 as libc::c_int as usize] *
                      delta[0 as libc::c_int as usize] +
                      delta[1 as libc::c_int as usize] *
                          delta[1 as libc::c_int as usize] +
                      delta[2 as libc::c_int as usize] *
                          delta[2 as libc::c_int as usize]);
    if ilength != 0. { ilength = 1.0f32 / ilength }
    delta[0 as libc::c_int as usize] *= ilength;
    delta[1 as libc::c_int as usize] *= ilength;
    delta[2 as libc::c_int as usize] *= ilength;
    if __tg_sqrt(delta[0 as libc::c_int as usize] *
                     delta[0 as libc::c_int as usize] +
                     delta[1 as libc::c_int as usize] *
                         delta[1 as libc::c_int as usize] +
                     delta[2 as libc::c_int as usize] *
                         delta[2 as libc::c_int as usize]) > 0.0f32 {
        VectorAngles(delta.as_mut_ptr(), (*ent).curstate.angles.as_mut_ptr());
    }
    return true_0;
}
/*
====================
CL_UpdateLatchedVars

====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_UpdateLatchedVars(mut ent: *mut cl_entity_t) {
    if (*ent).model.is_null() ||
           (*(*ent).model).type_0 as libc::c_int != mod_alias as libc::c_int
               &&
               (*(*ent).model).type_0 as libc::c_int !=
                   mod_studio as libc::c_int {
        return
    } // below fields used only for alias and studio interpolation
    (*ent).latched.prevorigin[0 as libc::c_int as usize] =
        (*ent).prevstate.origin[0 as libc::c_int as usize];
    (*ent).latched.prevorigin[1 as libc::c_int as usize] =
        (*ent).prevstate.origin[1 as libc::c_int as usize];
    (*ent).latched.prevorigin[2 as libc::c_int as usize] =
        (*ent).prevstate.origin[2 as libc::c_int as usize];
    (*ent).latched.prevangles[0 as libc::c_int as usize] =
        (*ent).prevstate.angles[0 as libc::c_int as usize];
    (*ent).latched.prevangles[1 as libc::c_int as usize] =
        (*ent).prevstate.angles[1 as libc::c_int as usize];
    (*ent).latched.prevangles[2 as libc::c_int as usize] =
        (*ent).prevstate.angles[2 as libc::c_int as usize];
    if (*(*ent).model).type_0 as libc::c_int == mod_alias as libc::c_int {
        (*ent).latched.prevframe = (*ent).prevstate.frame
    }
    (*ent).latched.prevanimtime = (*ent).prevstate.animtime;
    if (*ent).curstate.sequence != (*ent).prevstate.sequence {
        memcpy((*ent).prevstate.blending.as_mut_ptr() as *mut libc::c_void,
               (*ent).latched.prevseqblending.as_mut_ptr() as
                   *const libc::c_void,
               ::std::mem::size_of::<[byte; 4]>() as libc::c_ulong);
        (*ent).latched.prevsequence = (*ent).prevstate.sequence;
        (*ent).latched.sequencetime = (*ent).curstate.animtime
    }
    memcpy((*ent).latched.prevcontroller.as_mut_ptr() as *mut libc::c_void,
           (*ent).prevstate.controller.as_mut_ptr() as *const libc::c_void,
           ::std::mem::size_of::<[byte; 4]>() as libc::c_ulong);
    memcpy((*ent).latched.prevblending.as_mut_ptr() as *mut libc::c_void,
           (*ent).prevstate.blending.as_mut_ptr() as *const libc::c_void,
           ::std::mem::size_of::<[byte; 2]>() as libc::c_ulong);
    // update custom latched vars
    if clgame.drawFuncs.CL_UpdateLatchedVars.is_some() {
        clgame.drawFuncs.CL_UpdateLatchedVars.expect("non-null function pointer")(ent,
                                                                                  false_0);
    };
}
/*
====================
CL_GetStudioEstimatedFrame

====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_GetStudioEstimatedFrame(mut ent: *mut cl_entity_t)
 -> libc::c_float {
    let mut pstudiohdr: *mut studiohdr_t = 0 as *mut studiohdr_t;
    let mut pseqdesc: *mut mstudioseqdesc_t = 0 as *mut mstudioseqdesc_t;
    let mut sequence: libc::c_int = 0;
    if !(*ent).model.is_null() &&
           (*(*ent).model).type_0 as libc::c_int == mod_studio as libc::c_int
       {
        pstudiohdr = Mod_StudioExtradata((*ent).model) as *mut studiohdr_t;
        if !pstudiohdr.is_null() {
            sequence =
                if (*ent).curstate.sequence >= 0 as libc::c_int {
                    if (*ent).curstate.sequence <
                           (*pstudiohdr).numseq - 1 as libc::c_int {
                        (*ent).curstate.sequence
                    } else { ((*pstudiohdr).numseq) - 1 as libc::c_int }
                } else { 0 as libc::c_int };
            pseqdesc =
                ((pstudiohdr as
                      *mut byte).offset((*pstudiohdr).seqindex as isize) as
                     *mut mstudioseqdesc_t).offset(sequence as isize);
            return ref_0.dllFuncs.R_StudioEstimateFrame.expect("non-null function pointer")(ent,
                                                                                            pseqdesc)
        }
    }
    return 0 as libc::c_int as libc::c_float;
}
/*
====================
CL_ResetLatchedVars

====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ResetLatchedVars(mut ent: *mut cl_entity_t,
                                             mut full_reset: qboolean) {
    if (*ent).model.is_null() ||
           (*(*ent).model).type_0 as libc::c_int != mod_alias as libc::c_int
               &&
               (*(*ent).model).type_0 as libc::c_int !=
                   mod_studio as libc::c_int {
        return
    } // below fields used only for alias and studio interpolation
    if full_reset as u64 != 0 {
        // don't modify for sprites to avoid broke sprite interp
        memcpy((*ent).latched.prevblending.as_mut_ptr() as *mut libc::c_void,
               (*ent).curstate.blending.as_mut_ptr() as *const libc::c_void,
               ::std::mem::size_of::<[byte; 2]>() as libc::c_ulong);
        (*ent).latched.sequencetime = (*ent).curstate.animtime;
        memcpy((*ent).latched.prevcontroller.as_mut_ptr() as
                   *mut libc::c_void,
               (*ent).curstate.controller.as_mut_ptr() as *const libc::c_void,
               ::std::mem::size_of::<[byte; 4]>() as libc::c_ulong);
        if (*(*ent).model).type_0 as libc::c_int == mod_studio as libc::c_int
           {
            (*ent).latched.prevframe = CL_GetStudioEstimatedFrame(ent)
        } else if (*(*ent).model).type_0 as libc::c_int ==
                      mod_alias as libc::c_int {
            (*ent).latched.prevframe = (*ent).curstate.frame
        }
        (*ent).prevstate = (*ent).curstate
    }
    (*ent).curstate.animtime =
        cl.mtime[0 as libc::c_int as usize] as libc::c_float;
    (*ent).latched.prevanimtime = (*ent).curstate.animtime;
    (*ent).latched.prevorigin[0 as libc::c_int as usize] =
        (*ent).curstate.origin[0 as libc::c_int as usize];
    (*ent).latched.prevorigin[1 as libc::c_int as usize] =
        (*ent).curstate.origin[1 as libc::c_int as usize];
    (*ent).latched.prevorigin[2 as libc::c_int as usize] =
        (*ent).curstate.origin[2 as libc::c_int as usize];
    (*ent).latched.prevangles[0 as libc::c_int as usize] =
        (*ent).curstate.angles[0 as libc::c_int as usize];
    (*ent).latched.prevangles[1 as libc::c_int as usize] =
        (*ent).curstate.angles[1 as libc::c_int as usize];
    (*ent).latched.prevangles[2 as libc::c_int as usize] =
        (*ent).curstate.angles[2 as libc::c_int as usize];
    (*ent).latched.prevsequence = (*ent).curstate.sequence;
    // update custom latched vars
    if clgame.drawFuncs.CL_UpdateLatchedVars.is_some() {
        clgame.drawFuncs.CL_UpdateLatchedVars.expect("non-null function pointer")(ent,
                                                                                  true_0);
    };
}
/*
==================
CL_ProcessEntityUpdate

apply changes since new frame received
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ProcessEntityUpdate(mut ent: *mut cl_entity_t) {
    let mut parametric: qboolean = false_0;
    (*ent).model = CL_ModelHandle((*ent).curstate.modelindex);
    (*ent).index = (*ent).curstate.number;
    if (*ent).curstate.entityType & (1 as libc::c_int) << 0 as libc::c_int !=
           0 {
        COM_NormalizeAngles((*ent).curstate.angles.as_mut_ptr());
    }
    parametric = CL_ParametricMove(ent);
    // allow interpolation on bmodels too
    if !(*ent).model.is_null() &&
           (*(*ent).model).type_0 as libc::c_int == mod_brush as libc::c_int {
        (*ent).curstate.animtime = (*ent).curstate.msg_time
    }
    if CL_EntityCustomLerp(ent) as libc::c_uint != 0 && parametric as u64 == 0
       {
        (*ent).curstate.animtime = (*ent).curstate.msg_time
    }
    if CL_CompareTimestamps((*ent).curstate.animtime,
                            (*ent).prevstate.animtime) as u64 == 0 ||
           CL_EntityIgnoreLerp(ent) as libc::c_uint != 0 {
        CL_UpdateLatchedVars(ent);
        CL_UpdatePositions(ent);
    }
    // g-cont. it should be done for all the players?
    if (*ent).player as libc::c_uint != 0 &&
           host.features &
               ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint == 0 {
        (*ent).curstate.angles[0 as libc::c_int as usize] /= -3.0f32
    }
    (*ent).origin[0 as libc::c_int as usize] =
        (*ent).curstate.origin[0 as libc::c_int as usize];
    (*ent).origin[1 as libc::c_int as usize] =
        (*ent).curstate.origin[1 as libc::c_int as usize];
    (*ent).origin[2 as libc::c_int as usize] =
        (*ent).curstate.origin[2 as libc::c_int as usize];
    (*ent).angles[0 as libc::c_int as usize] =
        (*ent).curstate.angles[0 as libc::c_int as usize];
    (*ent).angles[1 as libc::c_int as usize] =
        (*ent).curstate.angles[1 as libc::c_int as usize];
    (*ent).angles[2 as libc::c_int as usize] =
        (*ent).curstate.angles[2 as libc::c_int as usize];
    // initialize attachments for now
    (*ent).attachment[0 as libc::c_int as usize][0 as libc::c_int as usize] =
        (*ent).origin[0 as libc::c_int as usize];
    (*ent).attachment[0 as libc::c_int as usize][1 as libc::c_int as usize] =
        (*ent).origin[1 as libc::c_int as usize];
    (*ent).attachment[0 as libc::c_int as usize][2 as libc::c_int as usize] =
        (*ent).origin[2 as libc::c_int as usize];
    (*ent).attachment[1 as libc::c_int as usize][0 as libc::c_int as usize] =
        (*ent).origin[0 as libc::c_int as usize];
    (*ent).attachment[1 as libc::c_int as usize][1 as libc::c_int as usize] =
        (*ent).origin[1 as libc::c_int as usize];
    (*ent).attachment[1 as libc::c_int as usize][2 as libc::c_int as usize] =
        (*ent).origin[2 as libc::c_int as usize];
    (*ent).attachment[2 as libc::c_int as usize][0 as libc::c_int as usize] =
        (*ent).origin[0 as libc::c_int as usize];
    (*ent).attachment[2 as libc::c_int as usize][1 as libc::c_int as usize] =
        (*ent).origin[1 as libc::c_int as usize];
    (*ent).attachment[2 as libc::c_int as usize][2 as libc::c_int as usize] =
        (*ent).origin[2 as libc::c_int as usize];
    (*ent).attachment[3 as libc::c_int as usize][0 as libc::c_int as usize] =
        (*ent).origin[0 as libc::c_int as usize];
    (*ent).attachment[3 as libc::c_int as usize][1 as libc::c_int as usize] =
        (*ent).origin[1 as libc::c_int as usize];
    (*ent).attachment[3 as libc::c_int as usize][2 as libc::c_int as usize] =
        (*ent).origin[2 as libc::c_int as usize];
}
/*
==================
CL_FindInterpolationUpdates

find two timestamps
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_FindInterpolationUpdates(mut ent:
                                                         *mut cl_entity_t,
                                                     mut targettime:
                                                         libc::c_float,
                                                     mut ph0:
                                                         *mut *mut position_history_t,
                                                     mut ph1:
                                                         *mut *mut position_history_t)
 -> qboolean {
    let mut extrapolate: qboolean = true_0; // curpos (lerp end)
    let mut i: libc::c_int = 0; // oldpos (lerp start)
    let mut i0: libc::c_int = 0;
    let mut i1: libc::c_int = 0;
    let mut imod: libc::c_int = 0;
    let mut at: libc::c_float = 0.;
    imod = (*ent).current_position;
    i0 = imod - 0 as libc::c_int & 64 as libc::c_int - 1 as libc::c_int;
    i1 = imod - 1 as libc::c_int & 64 as libc::c_int - 1 as libc::c_int;
    i = 1 as libc::c_int;
    while i < 64 as libc::c_int - 1 as libc::c_int {
        at =
            (*ent).ph[(imod - i & 64 as libc::c_int - 1 as libc::c_int) as
                          usize].animtime;
        if at == 0.0f32 { break ; }
        if targettime > at {
            // found it
            i0 =
                imod - i + 1 as libc::c_int &
                    64 as libc::c_int - 1 as libc::c_int;
            i1 =
                imod - i + 0 as libc::c_int &
                    64 as libc::c_int - 1 as libc::c_int;
            extrapolate = false_0;
            break ;
        } else { i += 1 }
    }
    if !ph0.is_null() {
        *ph0 =
            &mut *(*ent).ph.as_mut_ptr().offset(i0 as isize) as
                *mut position_history_t
    }
    if !ph1.is_null() {
        *ph1 =
            &mut *(*ent).ph.as_mut_ptr().offset(i1 as isize) as
                *mut position_history_t
    }
    return extrapolate;
}
/*
==================
CL_PureOrigin

non-local players interpolation
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_PureOrigin(mut ent: *mut cl_entity_t,
                                       mut t: libc::c_float,
                                       mut outorigin: *mut vec_t,
                                       mut outangles: *mut vec_t) {
    let mut extrapolate: qboolean = false_0;
    let mut t1: libc::c_float = 0.;
    let mut t0: libc::c_float = 0.;
    let mut frac: libc::c_float = 0.;
    let mut ph0: *mut position_history_t = 0 as *mut position_history_t;
    let mut ph1: *mut position_history_t = 0 as *mut position_history_t;
    let mut delta: vec3_t = [0.; 3];
    // NOTE: ph0 is next, ph1 is a prev
    extrapolate = CL_FindInterpolationUpdates(ent, t, &mut ph0, &mut ph1);
    if ph0.is_null() || ph1.is_null() { return }
    t0 = (*ph0).animtime;
    t1 = (*ph1).animtime;
    if t0 != 0.0f32 {
        let mut q: vec4_t = [0.; 4];
        let mut q1: vec4_t = [0.; 4];
        let mut q2: vec4_t = [0.; 4];
        delta[0 as libc::c_int as usize] =
            (*ph0).origin[0 as libc::c_int as usize] -
                (*ph1).origin[0 as libc::c_int as usize];
        delta[1 as libc::c_int as usize] =
            (*ph0).origin[1 as libc::c_int as usize] -
                (*ph1).origin[1 as libc::c_int as usize];
        delta[2 as libc::c_int as usize] =
            (*ph0).origin[2 as libc::c_int as usize] -
                (*ph1).origin[2 as libc::c_int as usize];
        if t0 != t1 { frac = (t - t1) / (t0 - t1) } else { frac = 1.0f32 }
        frac =
            if frac >= 0.0f32 {
                if frac < 1.2f32 { frac } else { 1.2f32 }
            } else { 0.0f32 };
        *outorigin.offset(0 as libc::c_int as isize) =
            (*ph1).origin[0 as libc::c_int as usize] +
                frac * delta[0 as libc::c_int as usize];
        *outorigin.offset(1 as libc::c_int as isize) =
            (*ph1).origin[1 as libc::c_int as usize] +
                frac * delta[1 as libc::c_int as usize];
        *outorigin.offset(2 as libc::c_int as isize) =
            (*ph1).origin[2 as libc::c_int as usize] +
                frac * delta[2 as libc::c_int as usize];
        AngleQuaternion((*ph0).angles.as_mut_ptr() as *const vec_t,
                        q1.as_mut_ptr(), false_0);
        AngleQuaternion((*ph1).angles.as_mut_ptr() as *const vec_t,
                        q2.as_mut_ptr(), false_0);
        QuaternionSlerp(q2.as_mut_ptr() as *const vec_t,
                        q1.as_mut_ptr() as *const vec_t, frac,
                        q.as_mut_ptr());
        QuaternionAngle(q.as_mut_ptr() as *const vec_t, outangles);
    } else {
        // no backup found
        *outorigin.offset(0 as libc::c_int as isize) =
            (*ph1).origin[0 as libc::c_int as usize];
        *outorigin.offset(1 as libc::c_int as isize) =
            (*ph1).origin[1 as libc::c_int as usize];
        *outorigin.offset(2 as libc::c_int as isize) =
            (*ph1).origin[2 as libc::c_int as usize];
        *outangles.offset(0 as libc::c_int as isize) =
            (*ph1).angles[0 as libc::c_int as usize];
        *outangles.offset(1 as libc::c_int as isize) =
            (*ph1).angles[1 as libc::c_int as usize];
        *outangles.offset(2 as libc::c_int as isize) =
            (*ph1).angles[2 as libc::c_int as usize]
    };
}
/*
==================
CL_InterpolateModel

non-players interpolation
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_InterpolateModel(mut e: *mut cl_entity_t)
 -> libc::c_int {
    let mut ph0: *mut position_history_t = 0 as *mut position_history_t;
    let mut ph1: *mut position_history_t = 0 as *mut position_history_t;
    let mut origin: vec3_t = [0.; 3];
    let mut angles: vec3_t = [0.; 3];
    let mut delta: vec3_t = [0.; 3];
    let mut t: libc::c_float = 0.;
    let mut t1: libc::c_float = 0.;
    let mut t2: libc::c_float = 0.;
    let mut frac: libc::c_float = 0.;
    let mut q: vec4_t = [0.; 4];
    let mut q1: vec4_t = [0.; 4];
    let mut q2: vec4_t = [0.; 4];
    (*e).origin[0 as libc::c_int as usize] =
        (*e).curstate.origin[0 as libc::c_int as usize];
    (*e).origin[1 as libc::c_int as usize] =
        (*e).curstate.origin[1 as libc::c_int as usize];
    (*e).origin[2 as libc::c_int as usize] =
        (*e).curstate.origin[2 as libc::c_int as usize];
    (*e).angles[0 as libc::c_int as usize] =
        (*e).curstate.angles[0 as libc::c_int as usize];
    (*e).angles[1 as libc::c_int as usize] =
        (*e).curstate.angles[1 as libc::c_int as usize];
    (*e).angles[2 as libc::c_int as usize] =
        (*e).curstate.angles[2 as libc::c_int as usize];
    if cls.timedemo as libc::c_uint != 0 || (*e).model.is_null() {
        return 1 as libc::c_int
    }
    if cls.demoplayback == DEMO_QUAKE1 as libc::c_int {
        // quake lerping is easy
        (*e).origin[0 as libc::c_int as usize] =
            (*e).prevstate.origin[0 as libc::c_int as usize] +
                cl.lerpFrac *
                    ((*e).curstate.origin[0 as libc::c_int as usize] -
                         (*e).prevstate.origin[0 as libc::c_int as usize]);
        (*e).origin[1 as libc::c_int as usize] =
            (*e).prevstate.origin[1 as libc::c_int as usize] +
                cl.lerpFrac *
                    ((*e).curstate.origin[1 as libc::c_int as usize] -
                         (*e).prevstate.origin[1 as libc::c_int as usize]);
        (*e).origin[2 as libc::c_int as usize] =
            (*e).prevstate.origin[2 as libc::c_int as usize] +
                cl.lerpFrac *
                    ((*e).curstate.origin[2 as libc::c_int as usize] -
                         (*e).prevstate.origin[2 as libc::c_int as usize]);
        AngleQuaternion((*e).prevstate.angles.as_mut_ptr() as *const vec_t,
                        q1.as_mut_ptr(), false_0);
        AngleQuaternion((*e).curstate.angles.as_mut_ptr() as *const vec_t,
                        q2.as_mut_ptr(), false_0);
        QuaternionSlerp(q1.as_mut_ptr() as *const vec_t,
                        q2.as_mut_ptr() as *const vec_t, cl.lerpFrac,
                        q.as_mut_ptr());
        QuaternionAngle(q.as_mut_ptr() as *const vec_t,
                        (*e).angles.as_mut_ptr());
        return 1 as libc::c_int
    }
    if cl.maxclients <= 1 as libc::c_int { return 1 as libc::c_int }
    if (*(*e).model).type_0 as libc::c_int == mod_brush as libc::c_int &&
           (*cl_bmodelinterp).value == 0. {
        return 1 as libc::c_int
    }
    if cl.local.moving != 0 && cl.local.onground == (*e).index {
        return 1 as libc::c_int
    }
    t = (cl.time - (*cl_interp).value as libc::c_double) as libc::c_float;
    CL_FindInterpolationUpdates(e, t, &mut ph0, &mut ph1);
    if ph0.is_null() || ph1.is_null() { return 0 as libc::c_int }
    t1 = (*ph1).animtime;
    t2 = (*ph0).animtime;
    if t - t1 < 0.0f32 { return 0 as libc::c_int }
    if t1 == 0.0f32 {
        (*e).origin[0 as libc::c_int as usize] =
            (*ph0).origin[0 as libc::c_int as usize];
        (*e).origin[1 as libc::c_int as usize] =
            (*ph0).origin[1 as libc::c_int as usize];
        (*e).origin[2 as libc::c_int as usize] =
            (*ph0).origin[2 as libc::c_int as usize];
        (*e).angles[0 as libc::c_int as usize] =
            (*ph0).angles[0 as libc::c_int as usize];
        (*e).angles[1 as libc::c_int as usize] =
            (*ph0).angles[1 as libc::c_int as usize];
        (*e).angles[2 as libc::c_int as usize] =
            (*ph0).angles[2 as libc::c_int as usize];
        return 0 as libc::c_int
    }
    if t2 == t1 {
        (*e).origin[0 as libc::c_int as usize] =
            (*ph0).origin[0 as libc::c_int as usize];
        (*e).origin[1 as libc::c_int as usize] =
            (*ph0).origin[1 as libc::c_int as usize];
        (*e).origin[2 as libc::c_int as usize] =
            (*ph0).origin[2 as libc::c_int as usize];
        (*e).angles[0 as libc::c_int as usize] =
            (*ph0).angles[0 as libc::c_int as usize];
        (*e).angles[1 as libc::c_int as usize] =
            (*ph0).angles[1 as libc::c_int as usize];
        (*e).angles[2 as libc::c_int as usize] =
            (*ph0).angles[2 as libc::c_int as usize];
        return 1 as libc::c_int
    }
    delta[0 as libc::c_int as usize] =
        (*ph0).origin[0 as libc::c_int as usize] -
            (*ph1).origin[0 as libc::c_int as usize];
    delta[1 as libc::c_int as usize] =
        (*ph0).origin[1 as libc::c_int as usize] -
            (*ph1).origin[1 as libc::c_int as usize];
    delta[2 as libc::c_int as usize] =
        (*ph0).origin[2 as libc::c_int as usize] -
            (*ph1).origin[2 as libc::c_int as usize];
    frac = (t - t1) / (t2 - t1);
    if frac < 0.0f32 { return 0 as libc::c_int }
    if frac > 1.0f32 { frac = 1.0f32 }
    origin[0 as libc::c_int as usize] =
        (*ph1).origin[0 as libc::c_int as usize] +
            frac * delta[0 as libc::c_int as usize];
    origin[1 as libc::c_int as usize] =
        (*ph1).origin[1 as libc::c_int as usize] +
            frac * delta[1 as libc::c_int as usize];
    origin[2 as libc::c_int as usize] =
        (*ph1).origin[2 as libc::c_int as usize] +
            frac * delta[2 as libc::c_int as usize];
    AngleQuaternion((*ph0).angles.as_mut_ptr() as *const vec_t,
                    q1.as_mut_ptr(), false_0);
    AngleQuaternion((*ph1).angles.as_mut_ptr() as *const vec_t,
                    q2.as_mut_ptr(), false_0);
    QuaternionSlerp(q2.as_mut_ptr() as *const vec_t,
                    q1.as_mut_ptr() as *const vec_t, frac, q.as_mut_ptr());
    QuaternionAngle(q.as_mut_ptr() as *const vec_t, angles.as_mut_ptr());
    (*e).origin[0 as libc::c_int as usize] =
        origin[0 as libc::c_int as usize];
    (*e).origin[1 as libc::c_int as usize] =
        origin[1 as libc::c_int as usize];
    (*e).origin[2 as libc::c_int as usize] =
        origin[2 as libc::c_int as usize];
    (*e).angles[0 as libc::c_int as usize] =
        angles[0 as libc::c_int as usize];
    (*e).angles[1 as libc::c_int as usize] =
        angles[1 as libc::c_int as usize];
    (*e).angles[2 as libc::c_int as usize] =
        angles[2 as libc::c_int as usize];
    return 1 as libc::c_int;
}
/*
=============
CL_ComputePlayerOrigin

interpolate non-local clients
=============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ComputePlayerOrigin(mut ent: *mut cl_entity_t) {
    let mut targettime: libc::c_float = 0.;
    let mut q: vec4_t = [0.; 4];
    let mut q1: vec4_t = [0.; 4];
    let mut q2: vec4_t = [0.; 4];
    let mut origin: vec3_t = [0.; 3];
    let mut angles: vec3_t = [0.; 3];
    if (*ent).player as u64 == 0 ||
           (*ent).index == cl.playernum + 1 as libc::c_int {
        return
    }
    if (*cl_nointerp).value > 0.0f32 {
        (*ent).angles[0 as libc::c_int as usize] =
            (*ent).curstate.angles[0 as libc::c_int as usize];
        (*ent).angles[1 as libc::c_int as usize] =
            (*ent).curstate.angles[1 as libc::c_int as usize];
        (*ent).angles[2 as libc::c_int as usize] =
            (*ent).curstate.angles[2 as libc::c_int as usize];
        (*ent).origin[0 as libc::c_int as usize] =
            (*ent).curstate.origin[0 as libc::c_int as usize];
        (*ent).origin[1 as libc::c_int as usize] =
            (*ent).curstate.origin[1 as libc::c_int as usize];
        (*ent).origin[2 as libc::c_int as usize] =
            (*ent).curstate.origin[2 as libc::c_int as usize];
        return
    }
    if cls.demoplayback == DEMO_QUAKE1 as libc::c_int {
        // quake lerping is easy
        (*ent).origin[0 as libc::c_int as usize] =
            (*ent).prevstate.origin[0 as libc::c_int as usize] +
                cl.lerpFrac *
                    ((*ent).curstate.origin[0 as libc::c_int as usize] -
                         (*ent).prevstate.origin[0 as libc::c_int as usize]);
        (*ent).origin[1 as libc::c_int as usize] =
            (*ent).prevstate.origin[1 as libc::c_int as usize] +
                cl.lerpFrac *
                    ((*ent).curstate.origin[1 as libc::c_int as usize] -
                         (*ent).prevstate.origin[1 as libc::c_int as usize]);
        (*ent).origin[2 as libc::c_int as usize] =
            (*ent).prevstate.origin[2 as libc::c_int as usize] +
                cl.lerpFrac *
                    ((*ent).curstate.origin[2 as libc::c_int as usize] -
                         (*ent).prevstate.origin[2 as libc::c_int as usize]);
        AngleQuaternion((*ent).prevstate.angles.as_mut_ptr() as *const vec_t,
                        q1.as_mut_ptr(), false_0);
        AngleQuaternion((*ent).curstate.angles.as_mut_ptr() as *const vec_t,
                        q2.as_mut_ptr(), false_0);
        QuaternionSlerp(q1.as_mut_ptr() as *const vec_t,
                        q2.as_mut_ptr() as *const vec_t, cl.lerpFrac,
                        q.as_mut_ptr());
        QuaternionAngle(q.as_mut_ptr() as *const vec_t,
                        (*ent).angles.as_mut_ptr());
        return
    }
    targettime =
        (cl.time - (*cl_interp).value as libc::c_double) as libc::c_float;
    CL_PureOrigin(ent, targettime, origin.as_mut_ptr(), angles.as_mut_ptr());
    (*ent).angles[0 as libc::c_int as usize] =
        angles[0 as libc::c_int as usize];
    (*ent).angles[1 as libc::c_int as usize] =
        angles[1 as libc::c_int as usize];
    (*ent).angles[2 as libc::c_int as usize] =
        angles[2 as libc::c_int as usize];
    (*ent).origin[0 as libc::c_int as usize] =
        origin[0 as libc::c_int as usize];
    (*ent).origin[1 as libc::c_int as usize] =
        origin[1 as libc::c_int as usize];
    (*ent).origin[2 as libc::c_int as usize] =
        origin[2 as libc::c_int as usize];
}
/*
=================
CL_ProcessPlayerState

process player states after the new packet has received
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ProcessPlayerState(mut playerindex: libc::c_int,
                                               mut state:
                                                   *mut entity_state_t) {
    let mut ps: *mut entity_state_t = 0 as *mut entity_state_t;
    ps =
        &mut *(*cl.frames.as_mut_ptr().offset(cl.parsecountmod as
                                                  isize)).playerstate.as_mut_ptr().offset(playerindex
                                                                                              as
                                                                                              isize)
            as *mut entity_state_t;
    (*ps).number = (*state).number;
    (*ps).messagenum = cl.parsecount;
    (*ps).msg_time = cl.mtime[0 as libc::c_int as usize] as libc::c_float;
    clgame.dllFuncs.pfnProcessPlayerState.expect("non-null function pointer")(ps,
                                                                              state);
}
/*
=================
CL_ResetLatchedState

reset latched state if this frame entity was teleported
or just EF_NOINTERP was set
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ResetLatchedState(mut pnum: libc::c_int,
                                              mut frame: *mut frame_t,
                                              mut ent: *mut cl_entity_t) {
    if if pnum >= 0 as libc::c_int {
           ((*frame).flags[(pnum >> 3 as libc::c_int) as usize] as libc::c_int
                & (1 as libc::c_int) << (pnum & 7 as libc::c_int)) as byte as
               libc::c_int
       } else { false_0 as libc::c_int as byte as libc::c_int } != 0 {
        (*ent).latched.prevorigin[0 as libc::c_int as usize] =
            (*ent).curstate.origin[0 as libc::c_int as usize];
        (*ent).latched.prevorigin[1 as libc::c_int as usize] =
            (*ent).curstate.origin[1 as libc::c_int as usize];
        (*ent).latched.prevorigin[2 as libc::c_int as usize] =
            (*ent).curstate.origin[2 as libc::c_int as usize];
        (*ent).latched.prevangles[0 as libc::c_int as usize] =
            (*ent).curstate.angles[0 as libc::c_int as usize];
        (*ent).latched.prevangles[1 as libc::c_int as usize] =
            (*ent).curstate.angles[1 as libc::c_int as usize];
        (*ent).latched.prevangles[2 as libc::c_int as usize] =
            (*ent).curstate.angles[2 as libc::c_int as usize];
        CL_ResetLatchedVars(ent, true_0);
        CL_ResetPositions(ent);
        // parametric interpolation will starts at this point
        if (*ent).curstate.starttime != 0.0f32 &&
               (*ent).curstate.impacttime != 0.0f32 {
            (*ent).lastmove = cl.time as libc::c_float
        }
    };
}
/*
=================
CL_ProcessPacket

process player states after the new packet has received
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ProcessPacket(mut frame: *mut frame_t) {
    let mut state: *mut entity_state_t = 0 as *mut entity_state_t;
    let mut ent: *mut cl_entity_t = 0 as *mut cl_entity_t;
    let mut pnum: libc::c_int = 0;
    pnum = 0 as libc::c_int;
    while pnum < (*frame).num_entities {
        // request the entity state from circular buffer
        state =
            &mut *cls.packet_entities.offset((((*frame).first_entity + pnum) %
                                                  cls.num_client_entities) as
                                                 isize) as
                *mut entity_state_t;
        (*state).messagenum = cl.parsecount;
        (*state).msg_time =
            cl.mtime[0 as libc::c_int as usize] as libc::c_float;
        // mark all the players
        ent =
            &mut *clgame.entities.offset((*state).number as isize) as
                *mut cl_entity_t;
        (*ent).player = CL_IsPlayerIndex((*state).number);
        if (*state).number == cl.playernum + 1 as libc::c_int {
            clgame.dllFuncs.pfnTxferLocalOverrides.expect("non-null function pointer")(state,
                                                                                       &mut (*frame).clientdata);
        }
        // shuffle states
        (*ent).prevstate = (*ent).curstate;
        (*ent).curstate = *state;
        CL_ProcessEntityUpdate(ent);
        CL_ResetLatchedState(pnum, frame, ent);
        if !((*ent).player as u64 == 0) {
            CL_ProcessPlayerState((*state).number - 1 as libc::c_int, state);
            if (*state).number == cl.playernum + 1 as libc::c_int {
                CL_CheckPredictionError();
            }
        }
        pnum += 1
    };
}
/*
=========================================================================

FRAME PARSING

=========================================================================
*/
/*
=================
CL_FlushEntityPacket

Read and ignore whole entity packet.
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_FlushEntityPacket(mut msg: *mut sizebuf_t) {
    let mut newnum: libc::c_int = 0; // can't render a frame
    let mut from: entity_state_t =
        entity_state_t{entityType: 0,
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
                       vuser4: [0.; 3],};
    let mut to: entity_state_t =
        entity_state_t{entityType: 0,
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
                       vuser4: [0.; 3],};
    memset(&mut from as *mut entity_state_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<entity_state_t>() as libc::c_ulong);
    cl.frames[cl.parsecountmod as usize].valid = false_0;
    cl.validsequence = 0 as libc::c_int;
    loop 
         // read it all, but ignore it
         {
        newnum =
            MSG_ReadUBitLong(msg, 13 as libc::c_int) as libc::c_int; // done
        if newnum ==
               ((1 as libc::c_int) << 13 as libc::c_int) - 1 as libc::c_int {
            break ;
        }
        if MSG_CheckOverflow(msg) as u64 != 0 {
            Host_Error(b"CL_FlushEntityPacket: overflow\n\x00" as *const u8 as
                           *const libc::c_char);
        }
        MSG_ReadDeltaEntity(msg, &mut from, &mut to, newnum,
                            if CL_IsPlayerIndex(newnum) as libc::c_uint != 0 {
                                DELTA_PLAYER as libc::c_int
                            } else { DELTA_ENTITY as libc::c_int },
                            cl.mtime[0 as libc::c_int as usize]);
    };
}
/*
=================
CL_DeltaEntity

processing delta update
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_DeltaEntity(mut msg: *mut sizebuf_t,
                                        mut frame: *mut frame_t,
                                        mut newnum: libc::c_int,
                                        mut old: *mut entity_state_t,
                                        mut has_update: qboolean) {
    let mut ent: *mut cl_entity_t = 0 as *mut cl_entity_t;
    let mut state: *mut entity_state_t = 0 as *mut entity_state_t;
    let mut newent: qboolean =
        if !old.is_null() {
            false_0 as libc::c_int
        } else { true_0 as libc::c_int } as qboolean;
    let mut pack: libc::c_int = (*frame).num_entities;
    let mut delta_type: libc::c_int = DELTA_ENTITY as libc::c_int;
    let mut alive: qboolean = true_0;
    // alloc next slot to store update
    state =
        &mut *cls.packet_entities.offset((cls.next_client_entities %
                                              cls.num_client_entities) as
                                             isize) as
            *mut entity_state_t; // enumerate entity index
    if CL_IsPlayerIndex(newnum) as u64 != 0 {
        delta_type = DELTA_PLAYER as libc::c_int
    } // release dead beams
    if newnum < 0 as libc::c_int || newnum >= clgame.maxEntities {
        Con_DPrintf(b"^1Error:^7 CL_DeltaEntity: invalid newnum: %d\n\x00" as
                        *const u8 as *const libc::c_char, newnum);
        if has_update as u64 != 0 {
            MSG_ReadDeltaEntity(msg, old, state, newnum, delta_type,
                                cl.mtime[0 as libc::c_int as usize]);
        }
        return
    }
    ent = CL_EDICT_NUM(newnum);
    (*ent).index = newnum;
    if newent as u64 != 0 { old = &mut (*ent).baseline }
    if has_update as u64 != 0 {
        alive =
            MSG_ReadDeltaEntity(msg, old, state, newnum, delta_type,
                                cl.mtime[0 as libc::c_int as usize])
    } else {
        memcpy(state as *mut libc::c_void, old as *const libc::c_void,
               ::std::mem::size_of::<entity_state_t>() as libc::c_ulong);
    }
    if alive as u64 == 0 { CL_KillDeadBeams(ent); return }
    if newent as u64 != 0 {
        // interpolation must be reset
        if pack >= 0 as libc::c_int {
            (*frame).flags[(pack >> 3 as libc::c_int) as usize] =
                ((*frame).flags[(pack >> 3 as libc::c_int) as usize] as
                     libc::c_int |
                     (1 as libc::c_int) << (pack & 7 as libc::c_int)) as byte
        } else { };
        // release beams from previous entity
        CL_KillDeadBeams(ent);
    }
    // add entity to packet
    cls.next_client_entities += 1;
    (*frame).num_entities += 1;
}
/*
==================
CL_ParsePacketEntities

An svc_packetentities has just been parsed, deal with the
rest of the data stream.
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParsePacketEntities(mut msg: *mut sizebuf_t,
                                                mut delta: qboolean)
 -> libc::c_int {
    let mut newframe: *mut frame_t = 0 as *mut frame_t;
    let mut oldframe: *mut frame_t = 0 as *mut frame_t;
    let mut oldindex: libc::c_int = 0;
    let mut newnum: libc::c_int = 0;
    let mut oldnum: libc::c_int = 0;
    let mut playerbytes: libc::c_int = 0 as libc::c_int;
    let mut oldpacket: libc::c_int = 0;
    let mut bufStart: libc::c_int = 0;
    let mut oldent: *mut entity_state_t = 0 as *mut entity_state_t;
    let mut player: qboolean = false_0;
    let mut count: libc::c_int = 0;
    // save first uncompressed packet as timestamp
    if cls.changelevel as libc::c_uint != 0 && delta as u64 == 0 &&
           cls.demorecording as libc::c_uint != 0 {
        CL_WriteDemoJumpTime();
    }
    // sentinel count. save it for debug checking
    if cls.legacymode as u64 != 0 {
        count = MSG_ReadWord(msg)
    } else {
        count =
            MSG_ReadUBitLong(msg,
                             11 as
                                 libc::c_int).wrapping_add(1 as libc::c_int as
                                                               libc::c_uint)
                as libc::c_int
    }
    newframe =
        &mut *cl.frames.as_mut_ptr().offset(cl.parsecountmod as isize) as
            *mut frame_t;
    // allocate parse entities
    memset((*newframe).flags.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<[byte; 256]>() as
               libc::c_ulong); // assume valid
    (*newframe).first_entity = cls.next_client_entities;
    (*newframe).num_entities = 0 as libc::c_int;
    (*newframe).valid = true_0;
    if delta as u64 != 0 {
        let mut subtracted: libc::c_int = 0;
        oldpacket = MSG_ReadByte(msg);
        subtracted =
            (cls.netchan.incoming_sequence.wrapping_sub(oldpacket as
                                                            libc::c_uint) &
                 0xff as libc::c_int as libc::c_uint) as libc::c_int;
        if subtracted == 0 as libc::c_int {
            Con_NPrintf(2 as libc::c_int,
                        b"^3Warning:^1 update too old\n^7\n\x00" as *const u8
                            as *const libc::c_char);
            CL_FlushEntityPacket(msg);
            return playerbytes
        }
        if subtracted >= CL_UPDATE_BACKUP - 1 as libc::c_int {
            // we can't use this, it is too old
            Con_NPrintf(2 as libc::c_int,
                        b"^3Warning:^1 delta frame is too old^7\n\x00" as
                            *const u8 as *const libc::c_char);
            CL_FlushEntityPacket(msg);
            return playerbytes
        }
        oldframe =
            &mut *cl.frames.as_mut_ptr().offset((oldpacket &
                                                     CL_UPDATE_BACKUP -
                                                         1 as libc::c_int) as
                                                    isize) as *mut frame_t;
        if cls.next_client_entities - (*oldframe).first_entity >
               cls.num_client_entities - 256 as libc::c_int {
            Con_NPrintf(2 as libc::c_int,
                        b"^3Warning:^1 delta frame is too old^7\n\x00" as
                            *const u8 as *const libc::c_char);
            CL_FlushEntityPacket(msg);
            return playerbytes
        }
    } else {
        // this is a full update that we can start delta compressing from now
        oldframe = 0 as *mut frame_t;
        // we can start recording now
        oldpacket =
            -(1 as libc::c_int); // delta too old or is initial message
        cl.send_reply = true_0; // send reply
        cls.demowaiting = false_0
    }
    // mark current delta state
    cl.validsequence =
        cls.netchan.incoming_sequence as
            libc::c_int; // end of packet entities
    oldent = 0 as *mut entity_state_t;
    oldindex = 0 as libc::c_int;
    if oldframe.is_null() {
        oldnum = 99999 as libc::c_int
    } else if oldindex >= (*oldframe).num_entities {
        oldnum = 99999 as libc::c_int
    } else {
        oldent =
            &mut *cls.packet_entities.offset((((*oldframe).first_entity +
                                                   oldindex) %
                                                  cls.num_client_entities) as
                                                 isize) as
                *mut entity_state_t;
        oldnum = (*oldent).number
    }
    loop  {
        let mut lastedict: libc::c_int = 0;
        if cls.legacymode as u64 != 0 {
            newnum = MSG_ReadWord(msg);
            lastedict = 0 as libc::c_int
        } else {
            newnum = MSG_ReadUBitLong(msg, 13 as libc::c_int) as libc::c_int;
            lastedict =
                ((1 as libc::c_int) << 13 as libc::c_int) - 1 as libc::c_int
        }
        if newnum == lastedict { break ; }
        if MSG_CheckOverflow(msg) as u64 != 0 {
            Host_Error(b"CL_ParsePacketEntities: overflow\n\x00" as *const u8
                           as *const libc::c_char);
        }
        player = CL_IsPlayerIndex(newnum);
        while oldnum < newnum {
            // one or more entities from the old packet are unchanged
            CL_DeltaEntity(msg, newframe, oldnum, oldent, false_0);
            oldindex += 1;
            if oldindex >= (*oldframe).num_entities {
                oldnum = 99999 as libc::c_int
            } else {
                oldent =
                    &mut *cls.packet_entities.offset((((*oldframe).first_entity
                                                           + oldindex) %
                                                          cls.num_client_entities)
                                                         as isize) as
                        *mut entity_state_t;
                oldnum = (*oldent).number
            }
        }
        if oldnum == newnum {
            // delta from previous state
            bufStart = MSG_GetNumBytesWritten(msg);
            CL_DeltaEntity(msg, newframe, newnum, oldent, true_0);
            if player as u64 != 0 {
                playerbytes += MSG_GetNumBytesWritten(msg) - bufStart
            }
            oldindex += 1;
            if oldindex >= (*oldframe).num_entities {
                oldnum = 99999 as libc::c_int
            } else {
                oldent =
                    &mut *cls.packet_entities.offset((((*oldframe).first_entity
                                                           + oldindex) %
                                                          cls.num_client_entities)
                                                         as isize) as
                        *mut entity_state_t;
                oldnum = (*oldent).number
            }
        } else {
            if !(oldnum > newnum) { continue ; }
            // delta from baseline ?
            bufStart = MSG_GetNumBytesWritten(msg);
            CL_DeltaEntity(msg, newframe, newnum, 0 as *mut entity_state_t,
                           true_0);
            if player as u64 != 0 {
                playerbytes += MSG_GetNumBytesWritten(msg) - bufStart
            }
        }
    }
    // any remaining entities in the old frame are copied over
    while oldnum != 99999 as libc::c_int {
        // one or more entities from the old packet are unchanged
        CL_DeltaEntity(msg, newframe, oldnum, oldent,
                       false_0); // frame is not valid but message was parsed
        oldindex += 1;
        if oldindex >= (*oldframe).num_entities {
            oldnum = 99999 as libc::c_int
        } else {
            oldent =
                &mut *cls.packet_entities.offset((((*oldframe).first_entity +
                                                       oldindex) %
                                                      cls.num_client_entities)
                                                     as isize) as
                    *mut entity_state_t;
            oldnum = (*oldent).number
        }
    }
    if (*newframe).num_entities != count &&
           (*newframe).num_entities != 0 as libc::c_int {
        Con_Reportf(b"^3Warning:^7 CL_Parse%sPacketEntities: (%i should be %i)\n\x00"
                        as *const u8 as *const libc::c_char,
                    if delta as libc::c_uint != 0 {
                        b"Delta\x00" as *const u8 as *const libc::c_char
                    } else { b"\x00" as *const u8 as *const libc::c_char },
                    (*newframe).num_entities, count);
    }
    if (*newframe).valid as u64 == 0 { return playerbytes }
    // now process packet.
    CL_ProcessPacket(newframe);
    // add new entities into physic lists
    CL_SetSolidEntities();
    // first update is the final signon stage where we actually receive an entity (i.e., the world at least)
    if cls.signon == 2 as libc::c_int - 1 as libc::c_int {
        // we are done with signon sequence.
        cls.signon = 2 as libc::c_int;
        // Clear loading plaque.
        CL_SignonReply();
    }
    return playerbytes;
}
/*
==========================================================================

INTERPOLATE BETWEEN FRAMES TO GET RENDERING PARMS

==========================================================================
*/
/*
=============
CL_AddVisibleEntity

all the visible entities should pass this filter
=============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_AddVisibleEntity(mut ent: *mut cl_entity_t,
                                             mut entityType: libc::c_int)
 -> qboolean {
    if ent.is_null() || (*ent).model.is_null() { return false_0 }
    // check for adding this entity
    if clgame.dllFuncs.pfnAddEntity.expect("non-null function pointer")(entityType,
                                                                        ent,
                                                                        (*(*ent).model).name.as_mut_ptr())
           == 0 {
        // local player was reject by game code, so ignore any effects
        if !ent.is_null() && (*ent).index == cl.playernum + 1 as libc::c_int
               && (*ent).player as libc::c_uint != 0 {
            cl.local.apply_effects = false_0
        }
        return false_0
    }
    // don't add the player in firstperson mode
    if !ent.is_null() && (*ent).index == cl.playernum + 1 as libc::c_int &&
           (*ent).player as libc::c_uint != 0 &&
           CL_IsThirdPerson() as u64 == 0 && (*ent).index == cl.viewentity {
        return false_0
    }
    if entityType == 3 as libc::c_int {
        ref_0.dllFuncs.CL_AddCustomBeam.expect("non-null function pointer")(ent);
        return true_0
    } else {
        if ref_0.dllFuncs.R_AddEntity.expect("non-null function pointer")(ent,
                                                                          entityType)
               as u64 == 0 {
            return false_0
        }
    }
    // because pTemp->entity.curstate.effects
	// is already occupied by FTENT_FLICKER
    if entityType != 2 as libc::c_int {
        // no reason to do it twice
        if !ent.is_null() && (*ent).index == cl.playernum + 1 as libc::c_int
               && (*ent).player as libc::c_uint != 0 {
            cl.local.apply_effects = false_0
        }
        // apply client-side effects
        CL_AddEntityEffects(ent);
        // alias & studiomodel efefcts
        CL_AddModelEffects(ent);
    }
    return true_0;
}
/*
=============
CL_LinkCustomEntity

Add server beam to draw list
=============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_LinkCustomEntity(mut ent: *mut cl_entity_t,
                                             mut state: *mut entity_state_t) {
    (*ent).curstate.movetype = (*state).modelindex; // !!!
    if (*(*ent).model).type_0 as libc::c_int != mod_sprite as libc::c_int {
        Con_Reportf(b"^3Warning:^7 bad model on beam ( %s )\n\x00" as
                        *const u8 as *const libc::c_char,
                    (*(*ent).model).name.as_mut_ptr());
    }
    (*ent).latched.prevsequence = (*ent).curstate.sequence;
    (*ent).latched.prevorigin[0 as libc::c_int as usize] =
        (*ent).origin[0 as libc::c_int as usize];
    (*ent).latched.prevorigin[1 as libc::c_int as usize] =
        (*ent).origin[1 as libc::c_int as usize];
    (*ent).latched.prevorigin[2 as libc::c_int as usize] =
        (*ent).origin[2 as libc::c_int as usize];
    (*ent).latched.prevangles[0 as libc::c_int as usize] =
        (*ent).angles[0 as libc::c_int as usize];
    (*ent).latched.prevangles[1 as libc::c_int as usize] =
        (*ent).angles[1 as libc::c_int as usize];
    (*ent).latched.prevangles[2 as libc::c_int as usize] =
        (*ent).angles[2 as libc::c_int as usize];
    (*ent).prevstate = (*ent).curstate;
    CL_AddVisibleEntity(ent, 3 as libc::c_int);
}
/*
=============
CL_LinkPlayers

Create visible entities in the correct position
for all current players
=============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_LinkPlayers(mut frame: *mut frame_t) {
    let mut state: *mut entity_state_t = 0 as *mut entity_state_t;
    let mut ent: *mut cl_entity_t = 0 as *mut cl_entity_t;
    let mut i: libc::c_int = 0;
    ent = CL_GetLocalPlayer();
    // apply muzzleflash to weaponmodel
    if !ent.is_null() && (*ent).curstate.effects & 2 as libc::c_int != 0 {
        clgame.viewent.curstate.effects =
            clgame.viewent.curstate.effects | 2 as libc::c_int
    }
    cl.local.apply_effects = true_0;
    // check all the clients but add only visible
    i = 0 as libc::c_int; // not present this frame
    state = (*frame).playerstate.as_mut_ptr();
    while i < (1 as libc::c_int) << 5 as libc::c_int {
        if !((*state).messagenum != cl.parsecount) {
            if !((*state).modelindex == 0 ||
                     (*state).effects & 128 as libc::c_int != 0) {
                ent =
                    &mut *clgame.entities.offset((i + 1 as libc::c_int) as
                                                     isize) as
                        *mut cl_entity_t;
                // fixup the player indexes...
                if (*ent).index != i + 1 as libc::c_int {
                    (*ent).index = i + 1 as libc::c_int
                }
                if i == cl.playernum {
                    if cls.demoplayback != DEMO_QUAKE1 as libc::c_int {
                        (*ent).origin[0 as libc::c_int as usize] =
                            (*state).origin[0 as libc::c_int as usize];
                        (*ent).origin[1 as libc::c_int as usize] =
                            (*state).origin[1 as libc::c_int as usize];
                        (*ent).origin[2 as libc::c_int as usize] =
                            (*state).origin[2 as libc::c_int as usize];
                        (*ent).prevstate.origin[0 as libc::c_int as usize] =
                            (*state).origin[0 as libc::c_int as usize];
                        (*ent).prevstate.origin[1 as libc::c_int as usize] =
                            (*state).origin[1 as libc::c_int as usize];
                        (*ent).prevstate.origin[2 as libc::c_int as usize] =
                            (*state).origin[2 as libc::c_int as usize];
                        (*ent).curstate.origin[0 as libc::c_int as usize] =
                            (*state).origin[0 as libc::c_int as usize];
                        (*ent).curstate.origin[1 as libc::c_int as usize] =
                            (*state).origin[1 as libc::c_int as usize];
                        (*ent).curstate.origin[2 as libc::c_int as usize] =
                            (*state).origin[2 as libc::c_int as usize]
                    }
                    (*ent).angles[0 as libc::c_int as usize] =
                        (*ent).curstate.angles[0 as libc::c_int as usize];
                    (*ent).angles[1 as libc::c_int as usize] =
                        (*ent).curstate.angles[1 as libc::c_int as usize];
                    (*ent).angles[2 as libc::c_int as usize] =
                        (*ent).curstate.angles[2 as libc::c_int as usize]
                }
                if (*ent).curstate.effects & 32 as libc::c_int != 0 {
                    CL_ResetLatchedVars(ent, false_0);
                }
                if CL_EntityTeleported(ent) as u64 != 0 {
                    (*ent).latched.prevorigin[0 as libc::c_int as usize] =
                        (*ent).curstate.origin[0 as libc::c_int as usize];
                    (*ent).latched.prevorigin[1 as libc::c_int as usize] =
                        (*ent).curstate.origin[1 as libc::c_int as usize];
                    (*ent).latched.prevorigin[2 as libc::c_int as usize] =
                        (*ent).curstate.origin[2 as libc::c_int as usize];
                    (*ent).latched.prevangles[0 as libc::c_int as usize] =
                        (*ent).curstate.angles[0 as libc::c_int as usize];
                    (*ent).latched.prevangles[1 as libc::c_int as usize] =
                        (*ent).curstate.angles[1 as libc::c_int as usize];
                    (*ent).latched.prevangles[2 as libc::c_int as usize] =
                        (*ent).curstate.angles[2 as libc::c_int as usize];
                    CL_ResetPositions(ent);
                }
                if i == cl.playernum {
                    if cls.demoplayback == DEMO_QUAKE1 as libc::c_int {
                        cl.simorg[0 as libc::c_int as usize] =
                            (*ent).prevstate.origin[0 as libc::c_int as usize]
                                +
                                cl.lerpFrac *
                                    ((*ent).curstate.origin[0 as libc::c_int
                                                                as usize] -
                                         (*ent).prevstate.origin[0 as
                                                                     libc::c_int
                                                                     as
                                                                     usize]);
                        cl.simorg[1 as libc::c_int as usize] =
                            (*ent).prevstate.origin[1 as libc::c_int as usize]
                                +
                                cl.lerpFrac *
                                    ((*ent).curstate.origin[1 as libc::c_int
                                                                as usize] -
                                         (*ent).prevstate.origin[1 as
                                                                     libc::c_int
                                                                     as
                                                                     usize]);
                        cl.simorg[2 as libc::c_int as usize] =
                            (*ent).prevstate.origin[2 as libc::c_int as usize]
                                +
                                cl.lerpFrac *
                                    ((*ent).curstate.origin[2 as libc::c_int
                                                                as usize] -
                                         (*ent).prevstate.origin[2 as
                                                                     libc::c_int
                                                                     as
                                                                     usize])
                    }
                    (*ent).origin[0 as libc::c_int as usize] =
                        cl.simorg[0 as libc::c_int as usize];
                    (*ent).origin[1 as libc::c_int as usize] =
                        cl.simorg[1 as libc::c_int as usize];
                    (*ent).origin[2 as libc::c_int as usize] =
                        cl.simorg[2 as libc::c_int as usize]
                } else {
                    (*ent).origin[0 as libc::c_int as usize] =
                        (*ent).curstate.origin[0 as libc::c_int as usize];
                    (*ent).origin[1 as libc::c_int as usize] =
                        (*ent).curstate.origin[1 as libc::c_int as usize];
                    (*ent).origin[2 as libc::c_int as usize] =
                        (*ent).curstate.origin[2 as libc::c_int as usize];
                    (*ent).angles[0 as libc::c_int as usize] =
                        (*ent).curstate.angles[0 as libc::c_int as usize];
                    (*ent).angles[1 as libc::c_int as usize] =
                        (*ent).curstate.angles[1 as libc::c_int as usize];
                    (*ent).angles[2 as libc::c_int as usize] =
                        (*ent).curstate.angles[2 as libc::c_int as usize];
                    // interpolate non-local clients
                    CL_ComputePlayerOrigin(ent);
                }
                (*ent).attachment[0 as libc::c_int as
                                      usize][0 as libc::c_int as usize] =
                    (*ent).origin[0 as libc::c_int as usize];
                (*ent).attachment[0 as libc::c_int as
                                      usize][1 as libc::c_int as usize] =
                    (*ent).origin[1 as libc::c_int as usize];
                (*ent).attachment[0 as libc::c_int as
                                      usize][2 as libc::c_int as usize] =
                    (*ent).origin[2 as libc::c_int as usize];
                (*ent).attachment[1 as libc::c_int as
                                      usize][0 as libc::c_int as usize] =
                    (*ent).origin[0 as libc::c_int as usize];
                (*ent).attachment[1 as libc::c_int as
                                      usize][1 as libc::c_int as usize] =
                    (*ent).origin[1 as libc::c_int as usize];
                (*ent).attachment[1 as libc::c_int as
                                      usize][2 as libc::c_int as usize] =
                    (*ent).origin[2 as libc::c_int as usize];
                (*ent).attachment[2 as libc::c_int as
                                      usize][0 as libc::c_int as usize] =
                    (*ent).origin[0 as libc::c_int as usize];
                (*ent).attachment[2 as libc::c_int as
                                      usize][1 as libc::c_int as usize] =
                    (*ent).origin[1 as libc::c_int as usize];
                (*ent).attachment[2 as libc::c_int as
                                      usize][2 as libc::c_int as usize] =
                    (*ent).origin[2 as libc::c_int as usize];
                (*ent).attachment[3 as libc::c_int as
                                      usize][0 as libc::c_int as usize] =
                    (*ent).origin[0 as libc::c_int as usize];
                (*ent).attachment[3 as libc::c_int as
                                      usize][1 as libc::c_int as usize] =
                    (*ent).origin[1 as libc::c_int as usize];
                (*ent).attachment[3 as libc::c_int as
                                      usize][2 as libc::c_int as usize] =
                    (*ent).origin[2 as libc::c_int as usize];
                CL_AddVisibleEntity(ent, 1 as libc::c_int);
            }
        }
        i += 1;
        state = state.offset(1)
    }
    // apply local player effects if entity is not added
    if cl.local.apply_effects as u64 != 0 {
        CL_AddEntityEffects(CL_GetLocalPlayer());
    };
}
/*
===============
CL_LinkPacketEntities

===============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_LinkPacketEntities(mut frame: *mut frame_t) {
    let mut ent: *mut cl_entity_t = 0 as *mut cl_entity_t;
    let mut state: *mut entity_state_t = 0 as *mut entity_state_t;
    let mut parametric: qboolean = false_0;
    let mut interpolate: qboolean = false_0;
    let mut i: libc::c_int = 0;
    let mut current_block_60: u64;
    i = 0 as libc::c_int;
    while i < (*frame).num_entities {
        state =
            &mut *cls.packet_entities.offset((((*frame).first_entity + i) %
                                                  cls.num_client_entities) as
                                                 isize) as
                *mut entity_state_t;
        // clients are should be done in CL_LinkPlayers
        if !((*state).number >= 1 as libc::c_int &&
                 (*state).number <= cl.maxclients) {
            // if set to invisible, skip
            if !((*state).modelindex == 0 ||
                     (*state).effects & 128 as libc::c_int != 0) {
                ent = CL_GetEntityByIndex((*state).number);
                if ent.is_null() {
                    Con_Reportf(b"^1Error:^7 CL_LinkPacketEntity: bad entity %i\n\x00"
                                    as *const u8 as *const libc::c_char,
                                (*state).number);
                } else {
                    // animtime must keep an actual
                    (*ent).curstate.animtime = (*state).animtime;
                    (*ent).curstate.frame = (*state).frame;
                    interpolate = false_0;
                    if !(*ent).model.is_null() {
                        if (*ent).curstate.rendermode ==
                               kRenderNormal as libc::c_int {
                            // auto 'solid' faces
                            if (*(*ent).model).flags as libc::c_uint &
                                   (1 as libc::c_uint) << 3 as libc::c_int !=
                                   0 &&
                                   Host_IsQuakeCompatible() as libc::c_uint !=
                                       0 {
                                (*ent).curstate.rendermode =
                                    kRenderTransAlpha as libc::c_int;
                                (*ent).curstate.renderamt = 255 as libc::c_int
                            }
                        }
                        parametric =
                            ((*ent).curstate.impacttime != 0.0f32 &&
                                 (*ent).curstate.starttime != 0.0f32) as
                                libc::c_int as qboolean;
                        if parametric as u64 == 0 &&
                               (*ent).curstate.movetype != 14 as libc::c_int {
                            if (*ent).curstate.animtime ==
                                   (*ent).prevstate.animtime &&
                                   !((*ent).curstate.origin[0 as libc::c_int
                                                                as usize] ==
                                         (*ent).prevstate.origin[0 as
                                                                     libc::c_int
                                                                     as usize]
                                         &&
                                         (*ent).curstate.origin[1 as
                                                                    libc::c_int
                                                                    as usize]
                                             ==
                                             (*ent).prevstate.origin[1 as
                                                                         libc::c_int
                                                                         as
                                                                         usize]
                                         &&
                                         (*ent).curstate.origin[2 as
                                                                    libc::c_int
                                                                    as usize]
                                             ==
                                             (*ent).prevstate.origin[2 as
                                                                         libc::c_int
                                                                         as
                                                                         usize])
                               {
                                (*ent).lastmove =
                                    (cl.time + 0.2f64) as libc::c_float
                            }
                            if (*ent).curstate.eflags as libc::c_int &
                                   1 as libc::c_int != 0 {
                                if (*ent).curstate.animtime != 0.0f32 &&
                                       ((*(*ent).model).type_0 as libc::c_int
                                            == mod_alias as libc::c_int ||
                                            (*(*ent).model).type_0 as
                                                libc::c_int ==
                                                mod_studio as libc::c_int) {
                                    if (*ent).lastmove as libc::c_double >=
                                           cl.time {
                                        (*ent).latched.prevorigin[0 as
                                                                      libc::c_int
                                                                      as
                                                                      usize] =
                                            (*ent).curstate.origin[0 as
                                                                       libc::c_int
                                                                       as
                                                                       usize];
                                        (*ent).latched.prevorigin[1 as
                                                                      libc::c_int
                                                                      as
                                                                      usize] =
                                            (*ent).curstate.origin[1 as
                                                                       libc::c_int
                                                                       as
                                                                       usize];
                                        (*ent).latched.prevorigin[2 as
                                                                      libc::c_int
                                                                      as
                                                                      usize] =
                                            (*ent).curstate.origin[2 as
                                                                       libc::c_int
                                                                       as
                                                                       usize]
                                    }
                                    if host.features &
                                           ((1 as libc::c_int) <<
                                                7 as libc::c_int) as
                                               libc::c_uint != 0 {
                                        interpolate = true_0
                                    } else {
                                        (*ent).curstate.movetype =
                                            4 as libc::c_int
                                    }
                                }
                            }
                        }
                        if (*(*ent).model).type_0 as libc::c_int ==
                               mod_brush as libc::c_int {
                            CL_InterpolateModel(ent);
                            current_block_60 = 17747245473264231573;
                        } else {
                            if parametric as u64 != 0 {
                                CL_ParametricMove(ent);
                                (*ent).origin[0 as libc::c_int as usize] =
                                    (*ent).curstate.origin[0 as libc::c_int as
                                                               usize];
                                (*ent).origin[1 as libc::c_int as usize] =
                                    (*ent).curstate.origin[1 as libc::c_int as
                                                               usize];
                                (*ent).origin[2 as libc::c_int as usize] =
                                    (*ent).curstate.origin[2 as libc::c_int as
                                                               usize];
                                (*ent).angles[0 as libc::c_int as usize] =
                                    (*ent).curstate.angles[0 as libc::c_int as
                                                               usize];
                                (*ent).angles[1 as libc::c_int as usize] =
                                    (*ent).curstate.angles[1 as libc::c_int as
                                                               usize];
                                (*ent).angles[2 as libc::c_int as usize] =
                                    (*ent).curstate.angles[2 as libc::c_int as
                                                               usize];
                                current_block_60 = 1924505913685386279;
                            } else if CL_EntityCustomLerp(ent) as u64 != 0 {
                                if CL_InterpolateModel(ent) == 0 {
                                    current_block_60 = 4644295000439058019;
                                } else {
                                    current_block_60 = 1924505913685386279;
                                }
                            } else if (*ent).curstate.movetype ==
                                          4 as libc::c_int &&
                                          NET_IsLocalAddress(cls.netchan.remote_address)
                                              as u64 == 0 {
                                if CL_InterpolateModel(ent) == 0 {
                                    current_block_60 = 4644295000439058019;
                                } else {
                                    current_block_60 = 1924505913685386279;
                                }
                            } else {
                                // no interpolation right now
                                (*ent).origin[0 as libc::c_int as usize] =
                                    (*ent).curstate.origin[0 as libc::c_int as
                                                               usize];
                                (*ent).origin[1 as libc::c_int as usize] =
                                    (*ent).curstate.origin[1 as libc::c_int as
                                                               usize];
                                (*ent).origin[2 as libc::c_int as usize] =
                                    (*ent).curstate.origin[2 as libc::c_int as
                                                               usize];
                                (*ent).angles[0 as libc::c_int as usize] =
                                    (*ent).curstate.angles[0 as libc::c_int as
                                                               usize];
                                (*ent).angles[1 as libc::c_int as usize] =
                                    (*ent).curstate.angles[1 as libc::c_int as
                                                               usize];
                                (*ent).angles[2 as libc::c_int as usize] =
                                    (*ent).curstate.angles[2 as libc::c_int as
                                                               usize];
                                current_block_60 = 1924505913685386279;
                            }
                            match current_block_60 {
                                4644295000439058019 => { }
                                _ => {
                                    if (*(*ent).model).type_0 as libc::c_int
                                           == mod_studio as libc::c_int {
                                        if interpolate as libc::c_uint != 0 &&
                                               host.features &
                                                   ((1 as libc::c_int) <<
                                                        7 as libc::c_int) as
                                                       libc::c_uint != 0 {
                                            ref_0.dllFuncs.R_StudioLerpMovement.expect("non-null function pointer")(ent,
                                                                                                                    cl.time,
                                                                                                                    (*ent).origin.as_mut_ptr(),
                                                                                                                    (*ent).angles.as_mut_ptr());
                                        }
                                    }
                                    current_block_60 = 17747245473264231573;
                                }
                            }
                        }
                        match current_block_60 {
                            4644295000439058019 => { }
                            _ => {
                                if (*state).entityType &
                                       (1 as libc::c_int) << 0 as libc::c_int
                                       == 0 {
                                    CL_LinkCustomEntity(ent, state);
                                } else {
                                    if (*(*ent).model).type_0 as libc::c_int
                                           != mod_brush as libc::c_int {
                                        // NOTE: never pass sprites with rendercolor '0 0 0' it's a stupid Valve Hammer Editor bug
                                        if (*ent).curstate.rendercolor.r == 0
                                               &&
                                               (*ent).curstate.rendercolor.g
                                                   == 0 &&
                                               (*ent).curstate.rendercolor.b
                                                   == 0 {
                                            (*ent).curstate.rendercolor.b =
                                                255 as libc::c_int as byte;
                                            (*ent).curstate.rendercolor.g =
                                                (*ent).curstate.rendercolor.b;
                                            (*ent).curstate.rendercolor.r =
                                                (*ent).curstate.rendercolor.g
                                        }
                                    }
                                    // XASH SPECIFIC
                                    if (*ent).curstate.rendermode ==
                                           kRenderNormal as libc::c_int &&
                                           (*ent).curstate.renderfx ==
                                               kRenderFxNone as libc::c_int {
                                        (*ent).curstate.renderamt =
                                            255.0f32 as libc::c_int
                                    }
                                    if (*ent).curstate.aiment !=
                                           0 as libc::c_int &&
                                           (*ent).curstate.movetype !=
                                               14 as libc::c_int {
                                        (*ent).curstate.movetype =
                                            12 as libc::c_int
                                    }
                                    if (*ent).curstate.effects &
                                           32 as libc::c_int != 0 {
                                        CL_ResetLatchedVars(ent, false_0);
                                    }
                                    if CL_EntityTeleported(ent) as u64 != 0 {
                                        (*ent).latched.prevorigin[0 as
                                                                      libc::c_int
                                                                      as
                                                                      usize] =
                                            (*ent).curstate.origin[0 as
                                                                       libc::c_int
                                                                       as
                                                                       usize];
                                        (*ent).latched.prevorigin[1 as
                                                                      libc::c_int
                                                                      as
                                                                      usize] =
                                            (*ent).curstate.origin[1 as
                                                                       libc::c_int
                                                                       as
                                                                       usize];
                                        (*ent).latched.prevorigin[2 as
                                                                      libc::c_int
                                                                      as
                                                                      usize] =
                                            (*ent).curstate.origin[2 as
                                                                       libc::c_int
                                                                       as
                                                                       usize];
                                        (*ent).latched.prevangles[0 as
                                                                      libc::c_int
                                                                      as
                                                                      usize] =
                                            (*ent).curstate.angles[0 as
                                                                       libc::c_int
                                                                       as
                                                                       usize];
                                        (*ent).latched.prevangles[1 as
                                                                      libc::c_int
                                                                      as
                                                                      usize] =
                                            (*ent).curstate.angles[1 as
                                                                       libc::c_int
                                                                       as
                                                                       usize];
                                        (*ent).latched.prevangles[2 as
                                                                      libc::c_int
                                                                      as
                                                                      usize] =
                                            (*ent).curstate.angles[2 as
                                                                       libc::c_int
                                                                       as
                                                                       usize];
                                        CL_ResetPositions(ent);
                                    }
                                    (*ent).attachment[0 as libc::c_int as
                                                          usize][0 as
                                                                     libc::c_int
                                                                     as usize]
                                        =
                                        (*ent).origin[0 as libc::c_int as
                                                          usize];
                                    (*ent).attachment[0 as libc::c_int as
                                                          usize][1 as
                                                                     libc::c_int
                                                                     as usize]
                                        =
                                        (*ent).origin[1 as libc::c_int as
                                                          usize];
                                    (*ent).attachment[0 as libc::c_int as
                                                          usize][2 as
                                                                     libc::c_int
                                                                     as usize]
                                        =
                                        (*ent).origin[2 as libc::c_int as
                                                          usize];
                                    (*ent).attachment[1 as libc::c_int as
                                                          usize][0 as
                                                                     libc::c_int
                                                                     as usize]
                                        =
                                        (*ent).origin[0 as libc::c_int as
                                                          usize];
                                    (*ent).attachment[1 as libc::c_int as
                                                          usize][1 as
                                                                     libc::c_int
                                                                     as usize]
                                        =
                                        (*ent).origin[1 as libc::c_int as
                                                          usize];
                                    (*ent).attachment[1 as libc::c_int as
                                                          usize][2 as
                                                                     libc::c_int
                                                                     as usize]
                                        =
                                        (*ent).origin[2 as libc::c_int as
                                                          usize];
                                    (*ent).attachment[2 as libc::c_int as
                                                          usize][0 as
                                                                     libc::c_int
                                                                     as usize]
                                        =
                                        (*ent).origin[0 as libc::c_int as
                                                          usize];
                                    (*ent).attachment[2 as libc::c_int as
                                                          usize][1 as
                                                                     libc::c_int
                                                                     as usize]
                                        =
                                        (*ent).origin[1 as libc::c_int as
                                                          usize];
                                    (*ent).attachment[2 as libc::c_int as
                                                          usize][2 as
                                                                     libc::c_int
                                                                     as usize]
                                        =
                                        (*ent).origin[2 as libc::c_int as
                                                          usize];
                                    (*ent).attachment[3 as libc::c_int as
                                                          usize][0 as
                                                                     libc::c_int
                                                                     as usize]
                                        =
                                        (*ent).origin[0 as libc::c_int as
                                                          usize];
                                    (*ent).attachment[3 as libc::c_int as
                                                          usize][1 as
                                                                     libc::c_int
                                                                     as usize]
                                        =
                                        (*ent).origin[1 as libc::c_int as
                                                          usize];
                                    (*ent).attachment[3 as libc::c_int as
                                                          usize][2 as
                                                                     libc::c_int
                                                                     as usize]
                                        =
                                        (*ent).origin[2 as libc::c_int as
                                                          usize];
                                    CL_AddVisibleEntity(ent,
                                                        0 as libc::c_int);
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1
    };
}
/*
===============
CL_MoveThirdpersonCamera

think thirdperson
===============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_MoveThirdpersonCamera() {
    if cls.state as libc::c_uint ==
           ca_disconnected as libc::c_int as libc::c_uint ||
           cls.state as libc::c_uint ==
               ca_cinematic as libc::c_int as libc::c_uint {
        return
    }
    // think thirdperson camera
    clgame.dllFuncs.CAM_Think.expect("non-null function pointer")();
}
/*
===============
CL_EmitEntities

add visible entities to refresh list
process frame interpolation etc
===============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_EmitEntities() {
    if cl.paused as u64 != 0 { return } // don't waste time
    // not in server yet, no entities to redraw
    if cls.state as libc::c_uint != ca_active as libc::c_int as libc::c_uint
           || cl.validsequence == 0 {
        return
    }
    // make sure we have at least one valid update
    if cl.frames[cl.parsecountmod as usize].valid as u64 == 0 { return }
    // animate lightestyles
    ref_0.dllFuncs.CL_RunLightStyles.expect("non-null function pointer")();
    // decay dynamic lights
    CL_DecayLights();
    // compute last interpolation amount
    CL_UpdateFrameLerp();
    // set client ideal pitch when mlook is disabled
    CL_SetIdealPitch();
    ref_0.dllFuncs.R_ClearScene.expect("non-null function pointer")();
    // link all the visible clients first
    CL_LinkPlayers(&mut *cl.frames.as_mut_ptr().offset(cl.parsecountmod as
                                                           isize));
    // link all the entities that actually have update
    CL_LinkPacketEntities(&mut *cl.frames.as_mut_ptr().offset(cl.parsecountmod
                                                                  as isize));
    // link custom user temp entities
    clgame.dllFuncs.pfnCreateEntities.expect("non-null function pointer")();
    // evaluate temp entities
    CL_TempEntUpdate();
    // fire events (client and server)
    CL_FireEvents();
    // handle spectator camera movement
    CL_MoveSpectatorCamera();
    // perfomance test
    CL_TestLights();
}
/*
==========================================================================

SOUND ENGINE IMPLEMENTATION

==========================================================================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_GetEntitySpatialization(mut ch: *mut channel_t)
 -> qboolean {
    let mut ent: *mut cl_entity_t = 0 as *mut cl_entity_t;
    let mut valid_origin: qboolean = false_0;
    if (*ch).entnum == 0 as libc::c_int {
        (*ch).staticsound = true_0;
        return true_0
        // static sound
    }
    if (*ch).entnum - 1 as libc::c_int == cl.playernum {
        (*ch).origin[0 as libc::c_int as usize] =
            refState.vieworg[0 as libc::c_int as usize];
        (*ch).origin[1 as libc::c_int as usize] =
            refState.vieworg[1 as libc::c_int as usize];
        (*ch).origin[2 as libc::c_int as usize] =
            refState.vieworg[2 as libc::c_int as usize];
        return true_0
    }
    valid_origin =
        if (*ch).origin[0 as libc::c_int as usize] == 0.0f32 &&
               (*ch).origin[1 as libc::c_int as usize] == 0.0f32 &&
               (*ch).origin[2 as libc::c_int as usize] == 0.0f32 {
            false_0 as libc::c_int
        } else { true_0 as libc::c_int } as qboolean;
    ent = CL_GetEntityByIndex((*ch).entnum);
    // entity is not present on the client but has valid origin
    if ent.is_null() || (*ent).model.is_null() ||
           (*ent).curstate.messagenum != cl.parsecount {
        return valid_origin
    }
    // setup origin
    if (*(*ent).model).type_0 as libc::c_int == mod_brush as libc::c_int {
        (*ch).origin[0 as libc::c_int as usize] =
            ((*(*ent).model).mins[0 as libc::c_int as usize] +
                 (*(*ent).model).maxs[0 as libc::c_int as usize]) * 0.5f32;
        (*ch).origin[1 as libc::c_int as usize] =
            ((*(*ent).model).mins[1 as libc::c_int as usize] +
                 (*(*ent).model).maxs[1 as libc::c_int as usize]) * 0.5f32;
        (*ch).origin[2 as libc::c_int as usize] =
            ((*(*ent).model).mins[2 as libc::c_int as usize] +
                 (*(*ent).model).maxs[2 as libc::c_int as usize]) * 0.5f32;
        (*ch).origin[0 as libc::c_int as usize] =
            (*ent).origin[0 as libc::c_int as usize] +
                (*ch).origin[0 as libc::c_int as usize];
        (*ch).origin[1 as libc::c_int as usize] =
            (*ent).origin[1 as libc::c_int as usize] +
                (*ch).origin[1 as libc::c_int as usize];
        (*ch).origin[2 as libc::c_int as usize] =
            (*ent).origin[2 as libc::c_int as usize] +
                (*ch).origin[2 as libc::c_int as usize]
    } else {
        (*ch).origin[0 as libc::c_int as usize] =
            (*ent).origin[0 as libc::c_int as usize];
        (*ch).origin[1 as libc::c_int as usize] =
            (*ent).origin[1 as libc::c_int as usize];
        (*ch).origin[2 as libc::c_int as usize] =
            (*ent).origin[2 as libc::c_int as usize]
    }
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn CL_GetMovieSpatialization(mut ch: *mut rawchan_t)
 -> qboolean {
    let mut ent: *mut cl_entity_t = 0 as *mut cl_entity_t;
    let mut valid_origin: qboolean = false_0;
    valid_origin =
        if (*ch).origin[0 as libc::c_int as usize] == 0.0f32 &&
               (*ch).origin[1 as libc::c_int as usize] == 0.0f32 &&
               (*ch).origin[2 as libc::c_int as usize] == 0.0f32 {
            false_0 as libc::c_int
        } else { true_0 as libc::c_int } as qboolean;
    ent = CL_GetEntityByIndex((*ch).entnum);
    // entity is not present on the client but has valid origin
    if ent.is_null() || (*ent).index == 0 ||
           (*ent).curstate.messagenum == 0 as libc::c_int {
        return valid_origin
    }
    // setup origin
    if (*(*ent).model).type_0 as libc::c_int == mod_brush as libc::c_int {
        (*ch).origin[0 as libc::c_int as usize] =
            ((*(*ent).model).mins[0 as libc::c_int as usize] +
                 (*(*ent).model).maxs[0 as libc::c_int as usize]) * 0.5f32;
        (*ch).origin[1 as libc::c_int as usize] =
            ((*(*ent).model).mins[1 as libc::c_int as usize] +
                 (*(*ent).model).maxs[1 as libc::c_int as usize]) * 0.5f32;
        (*ch).origin[2 as libc::c_int as usize] =
            ((*(*ent).model).mins[2 as libc::c_int as usize] +
                 (*(*ent).model).maxs[2 as libc::c_int as usize]) * 0.5f32;
        (*ch).origin[0 as libc::c_int as usize] =
            (*ent).origin[0 as libc::c_int as usize] +
                (*ch).origin[0 as libc::c_int as usize];
        (*ch).origin[1 as libc::c_int as usize] =
            (*ent).origin[1 as libc::c_int as usize] +
                (*ch).origin[1 as libc::c_int as usize];
        (*ch).origin[2 as libc::c_int as usize] =
            (*ent).origin[2 as libc::c_int as usize] +
                (*ch).origin[2 as libc::c_int as usize]
    } else {
        (*ch).origin[0 as libc::c_int as usize] =
            (*ent).origin[0 as libc::c_int as usize];
        (*ch).origin[1 as libc::c_int as usize] =
            (*ent).origin[1 as libc::c_int as usize];
        (*ch).origin[2 as libc::c_int as usize] =
            (*ent).origin[2 as libc::c_int as usize]
    }
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn CL_ExtraUpdate() {
    clgame.dllFuncs.IN_Accumulate.expect("non-null function pointer")();
    S_ExtraUpdate();
}
