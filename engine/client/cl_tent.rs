#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, extern_types,
           ptr_wrapping_offset_from, register_tool)]
extern "C" {
    pub type file_s;
    pub type grasshdr_s;
    pub type engine_studio_api_s;
    pub type r_studio_interface_s;
    pub type IVoiceTweak_s;
    pub type demo_api_s;
    #[no_mangle]
    fn sqrtf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn fabsf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Q_strlen(string: *const libc::c_char) -> size_t;
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Q_strnicmp(s1: *const libc::c_char, s2: *const libc::c_char,
                  n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Q_snprintf(buffer: *mut libc::c_char, buffersize: size_t,
                  format: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn COM_RandomFloat(fMin: libc::c_float, fMax: libc::c_float)
     -> libc::c_float;
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    static mut SI: sysinfo_t;
    #[no_mangle]
    fn _Mem_Alloc(poolptr: poolhandle_t, size: size_t, clear: qboolean,
                  filename: *const libc::c_char, fileline: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    fn _Mem_Free(data: *mut libc::c_void, filename: *const libc::c_char,
                 fileline: libc::c_int);
    #[no_mangle]
    fn FS_FileExists(filename: *const libc::c_char, gamedironly: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn Image_SetForceFlags(flags: uint);
    #[no_mangle]
    fn Image_ClearForceFlags();
    #[no_mangle]
    fn Host_IsQuakeCompatible() -> qboolean;
    #[no_mangle]
    fn Con_Reportf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_DPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn COM_RandomLong(lMin: libc::c_int, lMax: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn CL_PointContents(point: *const vec_t) -> libc::c_int;
    #[no_mangle]
    fn CL_GetEntityByIndex(index: libc::c_int) -> *mut cl_entity_s;
    #[no_mangle]
    static mut cl: client_t;
    #[no_mangle]
    static mut cls: client_static_t;
    #[no_mangle]
    static mut clgame: clgame_static_t;
    #[no_mangle]
    fn CL_ClearViewBeams();
    #[no_mangle]
    fn CL_ClearParticles();
    #[no_mangle]
    fn CL_AddVisibleEntity(ent: *mut cl_entity_t, entityType: libc::c_int)
     -> qboolean;
    #[no_mangle]
    fn CL_VisTraceLine(start: *mut vec_t, end: *mut vec_t, flags: libc::c_int)
     -> *mut pmtrace_t;
    #[no_mangle]
    fn anglemod(a: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn SinCos(radians: libc::c_float, sine: *mut libc::c_float,
              cosine: *mut libc::c_float);
    #[no_mangle]
    fn VectorNormalizeLength2(v: *const vec_t, out: *mut vec_t)
     -> libc::c_float;
    #[no_mangle]
    fn VectorVectors(forward: *const vec_t, right: *mut vec_t,
                     up: *mut vec_t);
    #[no_mangle]
    fn VectorAngles(forward: *const libc::c_float,
                    angles: *mut libc::c_float);
    #[no_mangle]
    fn AngleVectors(angles: *const vec_t, forward: *mut vec_t,
                    right: *mut vec_t, up: *mut vec_t);
    #[no_mangle]
    static mut vec3_origin: vec3_t;
    #[no_mangle]
    fn Mod_AliasExtradata(mod_0: *mut model_t) -> *mut libc::c_void;
    #[no_mangle]
    fn MSG_InitExt(sb: *mut sizebuf_t, pDebugName: *const libc::c_char,
                   pData: *mut libc::c_void, nBytes: libc::c_int,
                   nMaxBits: libc::c_int);
    #[no_mangle]
    fn MSG_CheckOverflow(sb: *mut sizebuf_t) -> qboolean;
    #[no_mangle]
    fn MSG_ReadChar(sb: *mut sizebuf_t) -> libc::c_int;
    #[no_mangle]
    fn MSG_ReadByte(sb: *mut sizebuf_t) -> libc::c_int;
    #[no_mangle]
    fn MSG_ReadShort(sb: *mut sizebuf_t) -> libc::c_int;
    #[no_mangle]
    fn MSG_ReadWord(sb: *mut sizebuf_t) -> libc::c_int;
    #[no_mangle]
    fn MSG_ReadCoord(sb: *mut sizebuf_t) -> libc::c_float;
    #[no_mangle]
    fn MSG_ReadBytes(sb: *mut sizebuf_t, pOut: *mut libc::c_void,
                     nBytes: libc::c_int) -> qboolean;
    #[link_name = "ref"]
    static mut ref_0: ref_state_s;
    #[no_mangle]
    static mut cl_testlights: *mut convar_t;
    #[no_mangle]
    fn CL_AddToResourceList(pResource: *mut resource_t,
                            pList: *mut resource_t);
    #[no_mangle]
    fn CL_ParseTextMessage(msg: *mut sizebuf_t);
    #[no_mangle]
    fn CL_ModelHandle(modelindex: libc::c_int) -> *mut model_t;
    #[no_mangle]
    fn CL_LoadClientSprite(filename: *const libc::c_char) -> *mut model_t;
    #[no_mangle]
    fn S_StartSound(pos: *const vec_t, ent: libc::c_int, chan: libc::c_int,
                    sfx: sound_t, vol: libc::c_float, attn: libc::c_float,
                    pitch: libc::c_int, flags: libc::c_int);
    #[no_mangle]
    fn S_RegisterSound(sample: *const libc::c_char) -> sound_t;
    #[no_mangle]
    fn CL_TraceLine(start: *mut vec_t, end: *mut vec_t, flags: libc::c_int)
     -> pmtrace_t;
    #[no_mangle]
    fn CL_ParseViewBeam(msg: *mut sizebuf_t, beamType: libc::c_int);
    #[no_mangle]
    fn R_ParticleExplosion2(org: *const vec_t, colorStart: libc::c_int,
                            colorLength: libc::c_int);
    #[no_mangle]
    fn R_Implosion(end: *const vec_t, radius: libc::c_float,
                   count: libc::c_int, life: libc::c_float);
    #[no_mangle]
    fn R_Blood(org: *const vec_t, dir: *const vec_t, pcolor: libc::c_int,
               speed: libc::c_int);
    #[no_mangle]
    fn R_BloodStream(org: *const vec_t, dir: *const vec_t,
                     pcolor: libc::c_int, speed: libc::c_int);
    #[no_mangle]
    fn R_BlobExplosion(org: *const vec_t);
    #[no_mangle]
    fn R_EntityParticles(ent: *mut cl_entity_t);
    #[no_mangle]
    fn R_FlickerParticles(org: *const vec_t);
    #[no_mangle]
    fn R_RunParticleEffect(org: *const vec_t, dir: *const vec_t,
                           color: libc::c_int, count: libc::c_int);
    #[no_mangle]
    fn R_ParticleBurst(org: *const vec_t, size: libc::c_int,
                       color: libc::c_int, life: libc::c_float);
    #[no_mangle]
    fn R_LavaSplash(org: *const vec_t);
    #[no_mangle]
    fn R_TeleportSplash(org: *const vec_t);
    #[no_mangle]
    fn R_RocketTrail(start: *mut vec_t, end: *mut vec_t, type_0: libc::c_int);
    #[no_mangle]
    fn R_TracerEffect(start: *const vec_t, end: *const vec_t);
    #[no_mangle]
    fn R_UserTracerParticle(org: *mut libc::c_float, vel: *mut libc::c_float,
                            life: libc::c_float, colorIndex: libc::c_int,
                            length: libc::c_float, deathcontext: byte,
                            deathfunc:
                                Option<unsafe extern "C" fn(_:
                                                                *mut particle_s)
                                           -> ()>);
    #[no_mangle]
    fn R_ParticleLine(start: *const vec_t, end: *const vec_t, r: byte,
                      g: byte, b: byte, life: libc::c_float);
    #[no_mangle]
    fn R_ParticleBox(mins: *const vec_t, maxs: *const vec_t, r: byte, g: byte,
                     b: byte, life: libc::c_float);
    #[no_mangle]
    fn R_ShowLine(start: *const vec_t, end: *const vec_t);
    #[no_mangle]
    fn R_BulletImpactParticles(pos: *const vec_t);
    #[no_mangle]
    fn R_LargeFunnel(pos: *const vec_t, reverse: libc::c_int);
    #[no_mangle]
    fn R_StreakSplash(pos: *const vec_t, dir: *const vec_t,
                      color: libc::c_int, count: libc::c_int,
                      speed: libc::c_float, velMin: libc::c_int,
                      velMax: libc::c_int);
    #[no_mangle]
    fn R_SparkStreaks(pos: *const vec_t, count: libc::c_int,
                      velocityMin: libc::c_int, velocityMax: libc::c_int);
    #[no_mangle]
    fn TriBoxInPVS(mins: *mut libc::c_float, maxs: *mut libc::c_float)
     -> libc::c_int;
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
pub type C2RustUnnamed = libc::c_uint;
pub const HOST_DEDICATED: C2RustUnnamed = 1;
pub const HOST_NORMAL: C2RustUnnamed = 0;
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
pub type sound_t = libc::c_int;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const kRenderTransAdd: C2RustUnnamed_0 = 5;
pub const kRenderTransAlpha: C2RustUnnamed_0 = 4;
pub const kRenderGlow: C2RustUnnamed_0 = 3;
pub const kRenderTransTexture: C2RustUnnamed_0 = 2;
pub const kRenderTransColor: C2RustUnnamed_0 = 1;
pub const kRenderNormal: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const kRenderFxClampMinScale: C2RustUnnamed_1 = 20;
pub const kRenderFxGlowShell: C2RustUnnamed_1 = 19;
pub const kRenderFxExplode: C2RustUnnamed_1 = 18;
pub const kRenderFxDeadPlayer: C2RustUnnamed_1 = 17;
pub const kRenderFxHologram: C2RustUnnamed_1 = 16;
pub const kRenderFxDistort: C2RustUnnamed_1 = 15;
pub const kRenderFxNoDissipation: C2RustUnnamed_1 = 14;
pub const kRenderFxFlickerFast: C2RustUnnamed_1 = 13;
pub const kRenderFxFlickerSlow: C2RustUnnamed_1 = 12;
pub const kRenderFxStrobeFaster: C2RustUnnamed_1 = 11;
pub const kRenderFxStrobeFast: C2RustUnnamed_1 = 10;
pub const kRenderFxStrobeSlow: C2RustUnnamed_1 = 9;
pub const kRenderFxSolidFast: C2RustUnnamed_1 = 8;
pub const kRenderFxSolidSlow: C2RustUnnamed_1 = 7;
pub const kRenderFxFadeFast: C2RustUnnamed_1 = 6;
pub const kRenderFxFadeSlow: C2RustUnnamed_1 = 5;
pub const kRenderFxPulseFastWide: C2RustUnnamed_1 = 4;
pub const kRenderFxPulseSlowWide: C2RustUnnamed_1 = 3;
pub const kRenderFxPulseFast: C2RustUnnamed_1 = 2;
pub const kRenderFxPulseSlow: C2RustUnnamed_1 = 1;
pub const kRenderFxNone: C2RustUnnamed_1 = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gameinfo_s {
    pub gamefolder: [libc::c_char; 64],
    pub basedir: [libc::c_char; 64],
    pub falldir: [libc::c_char; 64],
    pub startmap: [libc::c_char; 64],
    pub trainmap: [libc::c_char; 64],
    pub title: [libc::c_char; 64],
    pub version: libc::c_float,
    pub dll_path: [libc::c_char; 64],
    pub game_dll: [libc::c_char; 64],
    pub iconpath: [libc::c_char; 64],
    pub game_url: string,
    pub update_url: string,
    pub type_0: [libc::c_char; 64],
    pub date: [libc::c_char; 64],
    pub size: size_t,
    pub gamemode: libc::c_int,
    pub secure: qboolean,
    pub nomodels: qboolean,
    pub noskills: qboolean,
    pub sp_entity: [libc::c_char; 32],
    pub mp_entity: [libc::c_char; 32],
    pub mp_filter: [libc::c_char; 32],
    pub ambientsound: [[libc::c_char; 64]; 4],
    pub max_edicts: libc::c_int,
    pub max_tents: libc::c_int,
    pub max_beams: libc::c_int,
    pub max_particles: libc::c_int,
    pub game_dll_linux: [libc::c_char; 64],
    pub game_dll_osx: [libc::c_char; 64],
    pub added: qboolean,
}
pub type gameinfo_t = gameinfo_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sysinfo_s {
    pub exeName: string,
    pub rcName: string,
    pub basedirName: string,
    pub gamedll: string,
    pub clientlib: string,
    pub GameInfo: *mut gameinfo_t,
    pub games: [*mut gameinfo_t; 512],
    pub numgames: libc::c_int,
}
pub type sysinfo_t = sysinfo_s;
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
pub type C2RustUnnamed_2 = libc::c_uint;
pub const IL_OVERVIEW: C2RustUnnamed_2 = 64;
pub const IL_LOAD_DECAL: C2RustUnnamed_2 = 32;
pub const IL_DDS_HARDWARE: C2RustUnnamed_2 = 16;
pub const IL_DONTFLIP_TGA: C2RustUnnamed_2 = 8;
pub const IL_ALLOW_OVERWRITE: C2RustUnnamed_2 = 4;
pub const IL_KEEP_8BIT: C2RustUnnamed_2 = 2;
pub const IL_USE_LERPING: C2RustUnnamed_2 = 1;
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
pub type efrag_t = efrag_s;
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
pub type cl_entity_t = cl_entity_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct net_request_t {
    pub resp: net_response_t,
    pub pfnFunc: net_api_response_func_t,
    pub timeout: libc::c_double,
    pub timesend: libc::c_double,
    pub flags: libc::c_int,
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
pub type net_response_t = net_response_s;
pub type net_request_type_t = libc::c_uint;
pub const NET_REQUEST_CLIENT: net_request_type_t = 2;
pub const NET_REQUEST_GAMEUI: net_request_type_t = 1;
pub const NET_REQUEST_CANCEL: net_request_type_t = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cached_spritelist_t {
    pub szListName: [libc::c_char; 64],
    pub pList: *mut client_sprite_t,
    pub count: libc::c_int,
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
pub type wrect_t = wrect_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wrect_s {
    pub left: libc::c_int,
    pub right: libc::c_int,
    pub top: libc::c_int,
    pub bottom: libc::c_int,
}
pub type ref_overview_t = ref_overview_s;
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
pub type TRICULLSTYLE = libc::c_uint;
pub const TRI_NONE: TRICULLSTYLE = 1;
pub const TRI_FRONT: TRICULLSTYLE = 0;
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
pub struct cl_user_message_t {
    pub name: [libc::c_char; 32],
    pub number: libc::c_int,
    pub size: libc::c_int,
    pub func: pfnUserMsgHook,
}
pub type pfnUserMsgHook
    =
    Option<unsafe extern "C" fn(_: *const libc::c_char, _: libc::c_int,
                                _: *mut libc::c_void) -> libc::c_int>;
pub type playermove_t = playermove_s;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mstudiotex_s {
    pub name: [libc::c_char; 64],
    pub flags: uint32_t,
    pub width: int32_t,
    pub height: int32_t,
    pub index: int32_t,
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
pub type HSPRITE = libc::c_int;
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
pub type local_state_t = local_state_s;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpoint_t {
    pub point: vec2_t,
    pub coord: vec2_t,
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
pub type C2RustUnnamed_3 = libc::c_uint;
pub const DEMO_QUAKE1: C2RustUnnamed_3 = 2;
pub const DEMO_XASH3D: C2RustUnnamed_3 = 1;
pub const DEMO_INACTIVE: C2RustUnnamed_3 = 0;
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
pub struct mip_s {
    pub name: [libc::c_char; 16],
    pub width: libc::c_uint,
    pub height: libc::c_uint,
    pub offsets: [libc::c_uint; 4],
}
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
#[inline(always)]
unsafe extern "C" fn __tg_sqrt(mut __x: libc::c_float) -> libc::c_float {
    return sqrtf(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_fabs(mut __x: libc::c_float) -> libc::c_float {
    return fabsf(__x);
}
#[no_mangle]
pub static mut cl_active_tents: *mut TEMPENTITY =
    0 as *const TEMPENTITY as *mut TEMPENTITY;
#[no_mangle]
pub static mut cl_free_tents: *mut TEMPENTITY =
    0 as *const TEMPENTITY as *mut TEMPENTITY;
#[no_mangle]
pub static mut cl_tempents: *mut TEMPENTITY =
    0 as *const TEMPENTITY as *mut TEMPENTITY;
// entities pool
#[no_mangle]
pub static mut cl_sprite_muzzleflash: [*mut model_t; 3] =
    [0 as *const model_t as *mut model_t; 3];
// muzzle flashes
#[no_mangle]
pub static mut cl_sprite_dot: *mut model_t =
    0 as *const model_t as *mut model_t;
#[no_mangle]
pub static mut cl_sprite_ricochet: *mut model_t =
    0 as *const model_t as *mut model_t;
#[no_mangle]
pub static mut cl_sprite_shell: *mut model_t =
    0 as *const model_t as *mut model_t;
#[no_mangle]
pub static mut cl_sprite_glow: *mut model_t =
    0 as *const model_t as *mut model_t;
#[no_mangle]
pub static mut cl_default_sprites: [*const libc::c_char; 7] =
    [b"sprites/muzzleflash1.spr\x00" as *const u8 as *const libc::c_char,
     b"sprites/muzzleflash2.spr\x00" as *const u8 as *const libc::c_char,
     b"sprites/muzzleflash3.spr\x00" as *const u8 as *const libc::c_char,
     b"sprites/dot.spr\x00" as *const u8 as *const libc::c_char,
     b"sprites/animglow01.spr\x00" as *const u8 as *const libc::c_char,
     b"sprites/richo1.spr\x00" as *const u8 as *const libc::c_char,
     b"sprites/shellchrome.spr\x00" as *const u8 as *const libc::c_char];
#[no_mangle]
pub static mut cl_player_shell_sounds: [*const libc::c_char; 3] =
    [b"player/pl_shell1.wav\x00" as *const u8 as *const libc::c_char,
     b"player/pl_shell2.wav\x00" as *const u8 as *const libc::c_char,
     b"player/pl_shell3.wav\x00" as *const u8 as *const libc::c_char];
#[no_mangle]
pub static mut cl_weapon_shell_sounds: [*const libc::c_char; 3] =
    [b"weapons/sshell1.wav\x00" as *const u8 as *const libc::c_char,
     b"weapons/sshell2.wav\x00" as *const u8 as *const libc::c_char,
     b"weapons/sshell3.wav\x00" as *const u8 as *const libc::c_char];
#[no_mangle]
pub static mut cl_ricochet_sounds: [*const libc::c_char; 5] =
    [b"weapons/ric1.wav\x00" as *const u8 as *const libc::c_char,
     b"weapons/ric2.wav\x00" as *const u8 as *const libc::c_char,
     b"weapons/ric3.wav\x00" as *const u8 as *const libc::c_char,
     b"weapons/ric4.wav\x00" as *const u8 as *const libc::c_char,
     b"weapons/ric5.wav\x00" as *const u8 as *const libc::c_char];
#[no_mangle]
pub static mut cl_explode_sounds: [*const libc::c_char; 3] =
    [b"weapons/explode3.wav\x00" as *const u8 as *const libc::c_char,
     b"weapons/explode4.wav\x00" as *const u8 as *const libc::c_char,
     b"weapons/explode5.wav\x00" as *const u8 as *const libc::c_char];
/*
================
CL_LoadClientSprites

INTERNAL RESOURCE
================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_LoadClientSprites() {
    cl_sprite_muzzleflash[0 as libc::c_int as usize] =
        CL_LoadClientSprite(cl_default_sprites[0 as libc::c_int as usize]);
    cl_sprite_muzzleflash[1 as libc::c_int as usize] =
        CL_LoadClientSprite(cl_default_sprites[1 as libc::c_int as usize]);
    cl_sprite_muzzleflash[2 as libc::c_int as usize] =
        CL_LoadClientSprite(cl_default_sprites[2 as libc::c_int as usize]);
    cl_sprite_dot =
        CL_LoadClientSprite(cl_default_sprites[3 as libc::c_int as usize]);
    cl_sprite_glow =
        CL_LoadClientSprite(cl_default_sprites[4 as libc::c_int as usize]);
    cl_sprite_ricochet =
        CL_LoadClientSprite(cl_default_sprites[5 as libc::c_int as usize]);
    cl_sprite_shell =
        CL_LoadClientSprite(cl_default_sprites[6 as libc::c_int as usize]);
}
/*
================
CL_AddClientResource

add client-side resource to list
================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_AddClientResource(mut filename:
                                                  *const libc::c_char,
                                              mut type_0: libc::c_int) {
    let mut p: *mut resource_t = 0 as *mut resource_t; // already in list?
    let mut pResource: *mut resource_t =
        0 as *mut resource_t; // client resource marker
    p = cl.resourcesneeded.pNext;
    while p != &mut cl.resourcesneeded as *mut resource_t {
        if Q_strnicmp((*p).szFileName.as_mut_ptr(), filename,
                      99999 as libc::c_int) == 0 {
            break ;
        }
        p = (*p).pNext
    }
    if p != &mut cl.resourcesneeded as *mut resource_t { return }
    pResource =
        _Mem_Alloc(cls.mempool,
                   ::std::mem::size_of::<resource_t>() as libc::c_ulong,
                   true_0,
                   b"../engine/client/cl_tent.c\x00" as *const u8 as
                       *const libc::c_char, 129 as libc::c_int) as
            *mut resource_t;
    Q_strncpy((*pResource).szFileName.as_mut_ptr(), filename,
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    (*pResource).type_0 = type_0 as resourcetype_t;
    (*pResource).nIndex = -(1 as libc::c_int);
    (*pResource).nDownloadSize = 1 as libc::c_int;
    (*pResource).ucFlags =
        ((*pResource).ucFlags as libc::c_int |
             (1 as libc::c_int) << 1 as libc::c_int) as libc::c_uchar;
    CL_AddToResourceList(pResource, &mut cl.resourcesneeded);
}
/*
================
CL_AddClientResources

client resources not precached by server
================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_AddClientResources() {
    let mut filepath: [libc::c_char; 64] = [0; 64];
    let mut i: libc::c_int = 0;
    // don't request resources from localhost or in quake-compatibility mode
    if cl.maxclients <= 1 as libc::c_int ||
           Host_IsQuakeCompatible() as libc::c_uint != 0 {
        return
    }
    // check sprites first
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[*const libc::c_char; 7]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<*const libc::c_char>()
                                                   as libc::c_ulong) {
        if FS_FileExists(cl_default_sprites[i as usize],
                         false_0 as libc::c_int) == 0 {
            CL_AddClientResource(cl_default_sprites[i as usize],
                                 t_model as libc::c_int);
        }
        i += 1
    }
    // then check sounds
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[*const libc::c_char; 3]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<*const libc::c_char>()
                                                   as libc::c_ulong) {
        Q_snprintf(filepath.as_mut_ptr(),
                   ::std::mem::size_of::<[libc::c_char; 64]>() as
                       libc::c_ulong,
                   b"sound/%s\x00" as *const u8 as *const libc::c_char,
                   cl_player_shell_sounds[i as usize]);
        if FS_FileExists(filepath.as_mut_ptr(), false_0 as libc::c_int) == 0 {
            CL_AddClientResource(cl_player_shell_sounds[i as usize],
                                 t_sound as libc::c_int);
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[*const libc::c_char; 3]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<*const libc::c_char>()
                                                   as libc::c_ulong) {
        Q_snprintf(filepath.as_mut_ptr(),
                   ::std::mem::size_of::<[libc::c_char; 64]>() as
                       libc::c_ulong,
                   b"sound/%s\x00" as *const u8 as *const libc::c_char,
                   cl_weapon_shell_sounds[i as usize]);
        if FS_FileExists(filepath.as_mut_ptr(), false_0 as libc::c_int) == 0 {
            CL_AddClientResource(cl_weapon_shell_sounds[i as usize],
                                 t_sound as libc::c_int);
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[*const libc::c_char; 3]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<*const libc::c_char>()
                                                   as libc::c_ulong) {
        Q_snprintf(filepath.as_mut_ptr(),
                   ::std::mem::size_of::<[libc::c_char; 64]>() as
                       libc::c_ulong,
                   b"sound/%s\x00" as *const u8 as *const libc::c_char,
                   cl_explode_sounds[i as usize]);
        if FS_FileExists(filepath.as_mut_ptr(), false_0 as libc::c_int) == 0 {
            CL_AddClientResource(cl_explode_sounds[i as usize],
                                 t_sound as libc::c_int);
        }
        i += 1
    };
    // ric sounds was precached by server-side
}
/*
================
CL_InitTempents

================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_InitTempEnts() {
    cl_tempents =
        _Mem_Alloc(cls.mempool,
                   (::std::mem::size_of::<TEMPENTITY>() as
                        libc::c_ulong).wrapping_mul((*SI.GameInfo).max_tents
                                                        as libc::c_ulong),
                   true_0,
                   b"../engine/client/cl_tent.c\x00" as *const u8 as
                       *const libc::c_char, 208 as libc::c_int) as
            *mut TEMPENTITY;
    CL_ClearTempEnts();
    // load tempent sprites (glowshell, muzzleflashes etc)
    CL_LoadClientSprites();
}
/*
================
CL_ClearTempEnts

================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ClearTempEnts() {
    let mut i: libc::c_int = 0;
    if cl_tempents.is_null() { return }
    i = 0 as libc::c_int;
    while i < (*SI.GameInfo).max_tents - 1 as libc::c_int {
        let ref mut fresh0 = (*cl_tempents.offset(i as isize)).next;
        *fresh0 =
            &mut *cl_tempents.offset((i + 1 as libc::c_int) as isize) as
                *mut TEMPENTITY;
        (*cl_tempents.offset(i as isize)).entity.trivial_accept =
            0xffff as libc::c_int;
        i += 1
    }
    let ref mut fresh1 =
        (*cl_tempents.offset(((*SI.GameInfo).max_tents - 1 as libc::c_int) as
                                 isize)).next;
    *fresh1 = 0 as *mut tempent_s;
    cl_free_tents = cl_tempents;
    cl_active_tents = 0 as *mut TEMPENTITY;
}
/*
================
CL_FreeTempEnts

================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_FreeTempEnts() {
    if !cl_tempents.is_null() {
        _Mem_Free(cl_tempents as *mut libc::c_void,
                  b"../engine/client/cl_tent.c\x00" as *const u8 as
                      *const libc::c_char, 247 as libc::c_int);
    }
    cl_tempents = 0 as *mut TEMPENTITY;
}
/*
==============
CL_PrepareTEnt

set default values
==============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_PrepareTEnt(mut pTemp: *mut TEMPENTITY,
                                        mut pmodel: *mut model_t) {
    let mut frameCount: libc::c_int = 0 as libc::c_int;
    let mut modelIndex: libc::c_int = 0 as libc::c_int;
    let mut modelHandle: libc::c_int = (*pTemp).entity.trivial_accept;
    memset(pTemp as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<TEMPENTITY>() as libc::c_ulong);
    // use these to set per-frame and termination conditions / actions
    (*pTemp).entity.trivial_accept = modelHandle; // keep unchanged
    (*pTemp).flags = 0 as libc::c_int;
    (*pTemp).die = (cl.time + 0.75f32 as libc::c_double) as libc::c_float;
    if !pmodel.is_null() {
        frameCount = (*pmodel).numframes
    } else { (*pTemp).flags |= 0x40000 as libc::c_int }
    (*pTemp).entity.curstate.modelindex = modelIndex;
    (*pTemp).entity.curstate.rendermode = kRenderNormal as libc::c_int;
    (*pTemp).entity.curstate.renderfx = kRenderFxNone as libc::c_int;
    (*pTemp).entity.curstate.rendercolor.r = 255 as libc::c_int as byte;
    (*pTemp).entity.curstate.rendercolor.g = 255 as libc::c_int as byte;
    (*pTemp).entity.curstate.rendercolor.b = 255 as libc::c_int as byte;
    (*pTemp).frameMax =
        if 0 as libc::c_int > frameCount - 1 as libc::c_int {
            0 as libc::c_int
        } else { (frameCount) - 1 as libc::c_int } as libc::c_float;
    (*pTemp).entity.curstate.renderamt = 255 as libc::c_int;
    (*pTemp).entity.curstate.body = 0 as libc::c_int;
    (*pTemp).entity.curstate.skin = 0 as libc::c_int as libc::c_short;
    (*pTemp).entity.model = pmodel;
    (*pTemp).fadeSpeed = 0.5f32;
    (*pTemp).hitSound = 0 as libc::c_int;
    (*pTemp).clientIndex = 0 as libc::c_int as libc::c_short;
    (*pTemp).bounceFactor = 1 as libc::c_int as libc::c_float;
    (*pTemp).entity.curstate.scale = 1.0f32;
}
/*
==============
CL_TempEntPlaySound

play collide sound
==============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_TempEntPlaySound(mut pTemp: *mut TEMPENTITY,
                                             mut damp: libc::c_float) {
    let mut fvol: libc::c_float =
        0.; // shell casings have different playback parameters
    let mut soundname: [libc::c_char; 32] =
        [0; 32]; // shell casings have different playback parameters
    let mut isshellcasing: qboolean = false_0;
    let mut zvel: libc::c_int = 0;
    fvol = 0.8f32;
    match (*pTemp).hitSound {
        1 => {
            Q_snprintf(soundname.as_mut_ptr(),
                       ::std::mem::size_of::<[libc::c_char; 32]>() as
                           libc::c_ulong,
                       b"debris/glass%i.wav\x00" as *const u8 as
                           *const libc::c_char,
                       COM_RandomLong(1 as libc::c_int, 4 as libc::c_int));
        }
        2 => {
            Q_snprintf(soundname.as_mut_ptr(),
                       ::std::mem::size_of::<[libc::c_char; 32]>() as
                           libc::c_ulong,
                       b"debris/metal%i.wav\x00" as *const u8 as
                           *const libc::c_char,
                       COM_RandomLong(1 as libc::c_int, 6 as libc::c_int));
        }
        4 => {
            Q_snprintf(soundname.as_mut_ptr(),
                       ::std::mem::size_of::<[libc::c_char; 32]>() as
                           libc::c_ulong,
                       b"debris/flesh%i.wav\x00" as *const u8 as
                           *const libc::c_char,
                       COM_RandomLong(1 as libc::c_int, 7 as libc::c_int));
        }
        8 => {
            Q_snprintf(soundname.as_mut_ptr(),
                       ::std::mem::size_of::<[libc::c_char; 32]>() as
                           libc::c_ulong,
                       b"debris/wood%i.wav\x00" as *const u8 as
                           *const libc::c_char,
                       COM_RandomLong(1 as libc::c_int, 4 as libc::c_int));
        }
        16 => {
            Q_strncpy(soundname.as_mut_ptr(),
                      cl_ricochet_sounds[COM_RandomLong(0 as libc::c_int,
                                                        4 as libc::c_int) as
                                             usize],
                      ::std::mem::size_of::<[libc::c_char; 32]>() as
                          libc::c_ulong);
        }
        128 => {
            Q_strncpy(soundname.as_mut_ptr(),
                      cl_weapon_shell_sounds[COM_RandomLong(0 as libc::c_int,
                                                            2 as libc::c_int)
                                                 as usize],
                      ::std::mem::size_of::<[libc::c_char; 32]>() as
                          libc::c_ulong);
            isshellcasing = true_0;
            fvol = 0.5f32
        }
        32 => {
            Q_strncpy(soundname.as_mut_ptr(),
                      cl_player_shell_sounds[COM_RandomLong(0 as libc::c_int,
                                                            2 as libc::c_int)
                                                 as usize],
                      ::std::mem::size_of::<[libc::c_char; 32]>() as
                          libc::c_ulong);
            isshellcasing = true_0
        }
        64 => {
            Q_snprintf(soundname.as_mut_ptr(),
                       ::std::mem::size_of::<[libc::c_char; 32]>() as
                           libc::c_ulong,
                       b"debris/concrete%i.wav\x00" as *const u8 as
                           *const libc::c_char,
                       COM_RandomLong(1 as libc::c_int, 3 as libc::c_int));
        }
        _ => {
            // null sound
            return
        }
    }
    zvel =
        abs((*pTemp).entity.baseline.origin[2 as libc::c_int as usize] as
                libc::c_int);
    // only play one out of every n
    if isshellcasing as u64 != 0 {
        // play first bounce, then 1 out of 3
        if zvel < 200 as libc::c_int &&
               COM_RandomLong(0 as libc::c_int, 3 as libc::c_int) != 0 {
            return
        }
    } else if COM_RandomLong(0 as libc::c_int, 5 as libc::c_int) != 0 {
        return
    }
    if damp > 0.0f32 {
        let mut pitch: libc::c_int = 0;
        let mut handle: sound_t = 0;
        if isshellcasing as u64 != 0 {
            fvol *=
                if 1.0f32 < zvel as libc::c_float / 350.0f32 {
                    1.0f32
                } else { (zvel as libc::c_float) / 350.0f32 }
        } else {
            fvol *=
                if 1.0f32 < zvel as libc::c_float / 450.0f32 {
                    1.0f32
                } else { (zvel as libc::c_float) / 450.0f32 }
        }
        if COM_RandomLong(0 as libc::c_int, 3 as libc::c_int) == 0 &&
               isshellcasing as u64 == 0 {
            pitch = COM_RandomLong(95 as libc::c_int, 105 as libc::c_int)
        } else { pitch = 100 as libc::c_int }
        handle = S_RegisterSound(soundname.as_mut_ptr());
        S_StartSound((*pTemp).entity.origin.as_mut_ptr() as *const vec_t,
                     -(pTemp.wrapping_offset_from(cl_tempents) as
                           libc::c_long) as libc::c_int, 4 as libc::c_int,
                     handle, fvol, 0.8f64 as libc::c_float, pitch,
                     (1 as libc::c_int) << 10 as libc::c_int);
    };
}
/*
==============
CL_TEntAddEntity

add entity to renderlist
==============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_TempEntAddEntity(mut pEntity: *mut cl_entity_t)
 -> libc::c_int {
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    if (*pEntity).model.is_null() { return 0 as libc::c_int }
    mins[0 as libc::c_int as usize] =
        (*pEntity).origin[0 as libc::c_int as usize] +
            (*(*pEntity).model).mins[0 as libc::c_int as usize];
    mins[1 as libc::c_int as usize] =
        (*pEntity).origin[1 as libc::c_int as usize] +
            (*(*pEntity).model).mins[1 as libc::c_int as usize];
    mins[2 as libc::c_int as usize] =
        (*pEntity).origin[2 as libc::c_int as usize] +
            (*(*pEntity).model).mins[2 as libc::c_int as usize];
    maxs[0 as libc::c_int as usize] =
        (*pEntity).origin[0 as libc::c_int as usize] +
            (*(*pEntity).model).maxs[0 as libc::c_int as usize];
    maxs[1 as libc::c_int as usize] =
        (*pEntity).origin[1 as libc::c_int as usize] +
            (*(*pEntity).model).maxs[1 as libc::c_int as usize];
    maxs[2 as libc::c_int as usize] =
        (*pEntity).origin[2 as libc::c_int as usize] +
            (*(*pEntity).model).maxs[2 as libc::c_int as usize];
    // g-cont. just use PVS from previous frame
    if TriBoxInPVS(mins.as_mut_ptr(), maxs.as_mut_ptr()) != 0 {
        (*pEntity).curstate.angles[0 as libc::c_int as usize] =
            (*pEntity).angles[0 as libc::c_int as usize];
        (*pEntity).curstate.angles[1 as libc::c_int as usize] =
            (*pEntity).angles[1 as libc::c_int as usize];
        (*pEntity).curstate.angles[2 as libc::c_int as usize] =
            (*pEntity).angles[2 as libc::c_int as usize];
        (*pEntity).curstate.origin[0 as libc::c_int as usize] =
            (*pEntity).origin[0 as libc::c_int as usize];
        (*pEntity).curstate.origin[1 as libc::c_int as usize] =
            (*pEntity).origin[1 as libc::c_int as usize];
        (*pEntity).curstate.origin[2 as libc::c_int as usize] =
            (*pEntity).origin[2 as libc::c_int as usize];
        (*pEntity).latched.prevangles[0 as libc::c_int as usize] =
            (*pEntity).angles[0 as libc::c_int as usize];
        (*pEntity).latched.prevangles[1 as libc::c_int as usize] =
            (*pEntity).angles[1 as libc::c_int as usize];
        (*pEntity).latched.prevangles[2 as libc::c_int as usize] =
            (*pEntity).angles[2 as libc::c_int as usize];
        (*pEntity).latched.prevorigin[0 as libc::c_int as usize] =
            (*pEntity).origin[0 as libc::c_int as usize];
        (*pEntity).latched.prevorigin[1 as libc::c_int as usize] =
            (*pEntity).origin[1 as libc::c_int as usize];
        (*pEntity).latched.prevorigin[2 as libc::c_int as usize] =
            (*pEntity).origin[2 as libc::c_int as usize];
        // add to list
        CL_AddVisibleEntity(pEntity, 2 as libc::c_int);
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/*
==============
CL_AddTempEnts

temp-entities will be added on a user-side
setup client callback
==============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_TempEntUpdate() {
    let mut ft: libc::c_double = cl.time - cl.oldtime;
    let mut gravity: libc::c_float = clgame.movevars.gravity;
    clgame.dllFuncs.pfnTempEntUpdate.expect("non-null function pointer")(ft,
                                                                         cl.time,
                                                                         gravity
                                                                             as
                                                                             libc::c_double,
                                                                         &mut cl_free_tents,
                                                                         &mut cl_active_tents,
                                                                         Some(CL_TempEntAddEntity
                                                                                  as
                                                                                  unsafe extern "C" fn(_:
                                                                                                           *mut cl_entity_t)
                                                                                      ->
                                                                                          libc::c_int),
                                                                         Some(CL_TempEntPlaySound
                                                                                  as
                                                                                  unsafe extern "C" fn(_:
                                                                                                           *mut TEMPENTITY,
                                                                                                       _:
                                                                                                           libc::c_float)
                                                                                      ->
                                                                                          ()));
}
/*
==============
CL_TEntAddEntity

free the first low priority tempent it finds.
==============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_FreeLowPriorityTempEnt() -> qboolean {
    let mut pActive: *mut TEMPENTITY = cl_active_tents;
    let mut pPrev: *mut TEMPENTITY = 0 as *mut TEMPENTITY;
    while !pActive.is_null() {
        if (*pActive).priority == 0 as libc::c_int {
            // remove from the active list.
            if !pPrev.is_null() {
                (*pPrev).next = (*pActive).next
            } else { cl_active_tents = (*pActive).next }
            // add to the free list.
            (*pActive).next = cl_free_tents;
            cl_free_tents = pActive;
            return true_0
        }
        pPrev = pActive;
        pActive = (*pActive).next
    }
    return false_0;
}
/*
==============
CL_TempEntAlloc

alloc normal\low priority tempentity
==============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_TempEntAlloc(mut org: *const vec_t,
                                         mut pmodel: *mut model_t)
 -> *mut tempent_s {
    let mut pTemp: *mut TEMPENTITY = 0 as *mut TEMPENTITY;
    if cl_free_tents.is_null() {
        Con_DPrintf(b"Overflow %d temporary ents!\n\x00" as *const u8 as
                        *const libc::c_char, (*SI.GameInfo).max_tents);
        return 0 as *mut tempent_s
    }
    pTemp = cl_free_tents;
    cl_free_tents = (*pTemp).next;
    CL_PrepareTEnt(pTemp, pmodel);
    (*pTemp).priority = 0 as libc::c_int;
    if !org.is_null() {
        (*pTemp).entity.origin[0 as libc::c_int as usize] =
            *org.offset(0 as libc::c_int as isize);
        (*pTemp).entity.origin[1 as libc::c_int as usize] =
            *org.offset(1 as libc::c_int as isize);
        (*pTemp).entity.origin[2 as libc::c_int as usize] =
            *org.offset(2 as libc::c_int as isize)
    }
    (*pTemp).next = cl_active_tents;
    cl_active_tents = pTemp;
    return pTemp;
}
/*
==============
CL_TempEntAllocHigh

alloc high priority tempentity
==============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_TempEntAllocHigh(mut org: *const vec_t,
                                             mut pmodel: *mut model_t)
 -> *mut tempent_s {
    let mut pTemp: *mut TEMPENTITY = 0 as *mut TEMPENTITY;
    if cl_free_tents.is_null() {
        // no temporary ents free, so find the first active low-priority temp ent
		// and overwrite it.
        CL_FreeLowPriorityTempEnt();
    }
    if cl_free_tents.is_null() {
        // didn't find anything? The tent list is either full of high-priority tents
		// or all tents in the list are still due to live for > 10 seconds.
        Con_DPrintf(b"Couldn\'t alloc a high priority TENT!\n\x00" as
                        *const u8 as *const libc::c_char);
        return 0 as *mut tempent_s
    }
    // Move out of the free list and into the active list.
    pTemp = cl_free_tents;
    cl_free_tents = (*pTemp).next;
    CL_PrepareTEnt(pTemp, pmodel);
    (*pTemp).priority = 1 as libc::c_int;
    if !org.is_null() {
        (*pTemp).entity.origin[0 as libc::c_int as usize] =
            *org.offset(0 as libc::c_int as isize);
        (*pTemp).entity.origin[1 as libc::c_int as usize] =
            *org.offset(1 as libc::c_int as isize);
        (*pTemp).entity.origin[2 as libc::c_int as usize] =
            *org.offset(2 as libc::c_int as isize)
    }
    (*pTemp).next = cl_active_tents;
    cl_active_tents = pTemp;
    return pTemp;
}
/*
==============
CL_TempEntAlloc

alloc normal priority tempentity with no model
==============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_TempEntAllocNoModel(mut org: *const vec_t)
 -> *mut tempent_s {
    return CL_TempEntAlloc(org, 0 as *mut model_t);
}
/*
==============
CL_TempEntAlloc

custom tempentity allocation
==============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_TempEntAllocCustom(mut org: *const vec_t,
                                               mut model: *mut model_t,
                                               mut high: libc::c_int,
                                               mut pfn:
                                                   Option<unsafe extern "C" fn(_:
                                                                                   *mut TEMPENTITY,
                                                                               _:
                                                                                   libc::c_float,
                                                                               _:
                                                                                   libc::c_float)
                                                              -> ()>)
 -> *mut tempent_s {
    let mut pTemp: *mut TEMPENTITY = 0 as *mut TEMPENTITY;
    if high != 0 {
        pTemp = CL_TempEntAllocHigh(org, model)
    } else { pTemp = CL_TempEntAlloc(org, model) }
    if !pTemp.is_null() && pfn.is_some() {
        (*pTemp).flags |= 0x80000 as libc::c_int;
        (*pTemp).callback = pfn;
        (*pTemp).die = cl.time as libc::c_float
    }
    return pTemp;
}
/*
==============================================================

	EFFECTS BASED ON TEMPENTS (presets)

==============================================================
*/
/*
==============
R_FizzEffect

Create a fizz effect
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_FizzEffect(mut pent: *mut cl_entity_t,
                                      mut modelIndex: libc::c_int,
                                      mut density: libc::c_int) {
    let mut pTemp: *mut TEMPENTITY = 0 as *mut TEMPENTITY;
    let mut i: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    let mut depth: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut angle: libc::c_float = 0.;
    let mut maxHeight: libc::c_float = 0.;
    let mut speed: libc::c_float = 0.;
    let mut xspeed: libc::c_float = 0.;
    let mut yspeed: libc::c_float = 0.;
    let mut zspeed: libc::c_float = 0.;
    let mut origin: vec3_t = [0.; 3];
    let mut mod_0: *mut model_t = 0 as *mut model_t;
    if pent.is_null() || (*pent).curstate.modelindex <= 0 as libc::c_int {
        return
    }
    mod_0 = CL_ModelHandle((*pent).curstate.modelindex);
    if mod_0.is_null() { return }
    count = density + 1 as libc::c_int;
    density = count * 3 as libc::c_int + 6 as libc::c_int;
    maxHeight =
        (*mod_0).maxs[2 as libc::c_int as usize] -
            (*mod_0).mins[2 as libc::c_int as usize];
    width =
        ((*mod_0).maxs[0 as libc::c_int as usize] -
             (*mod_0).mins[0 as libc::c_int as usize]) as libc::c_int;
    depth =
        ((*mod_0).maxs[1 as libc::c_int as usize] -
             (*mod_0).mins[1 as libc::c_int as usize]) as libc::c_int;
    speed =
        (((*pent).curstate.rendercolor.r as libc::c_int) << 8 as libc::c_int |
             (*pent).curstate.rendercolor.g as libc::c_int) as libc::c_float;
    if (*pent).curstate.rendercolor.b != 0 { speed = -speed }
    angle =
        (*pent).angles[1 as libc::c_int as usize] *
            (3.14159265358979323846f64 as libc::c_float / 180.0f32);
    SinCos(angle, &mut yspeed, &mut xspeed);
    xspeed *= speed;
    yspeed *= speed;
    i = 0 as libc::c_int;
    while i < count {
        origin[0 as libc::c_int as usize] =
            (*mod_0).mins[0 as libc::c_int as usize] +
                COM_RandomLong(0 as libc::c_int, width - 1 as libc::c_int) as
                    libc::c_float;
        origin[1 as libc::c_int as usize] =
            (*mod_0).mins[1 as libc::c_int as usize] +
                COM_RandomLong(0 as libc::c_int, depth - 1 as libc::c_int) as
                    libc::c_float;
        origin[2 as libc::c_int as usize] =
            (*mod_0).mins[2 as libc::c_int as usize];
        pTemp =
            CL_TempEntAlloc(origin.as_mut_ptr() as *const vec_t,
                            CL_ModelHandle(modelIndex));
        if pTemp.is_null() { return }
        (*pTemp).flags |= 0x1 as libc::c_int;
        (*pTemp).x = origin[0 as libc::c_int as usize];
        (*pTemp).y = origin[1 as libc::c_int as usize];
        zspeed =
            COM_RandomLong(80 as libc::c_int, 140 as libc::c_int) as
                libc::c_float;
        (*pTemp).entity.baseline.origin[0 as libc::c_int as usize] = xspeed;
        (*pTemp).entity.baseline.origin[1 as libc::c_int as usize] = yspeed;
        (*pTemp).entity.baseline.origin[2 as libc::c_int as usize] = zspeed;
        (*pTemp).die =
            (cl.time + (maxHeight / zspeed) as libc::c_double -
                 0.1f32 as libc::c_double) as libc::c_float;
        (*pTemp).entity.curstate.frame =
            COM_RandomLong(0 as libc::c_int, (*pTemp).frameMax as libc::c_int)
                as libc::c_float;
        // Set sprite scale
        (*pTemp).entity.curstate.scale =
            1.0f32 / COM_RandomFloat(2.0f32, 5.0f32);
        (*pTemp).entity.curstate.rendermode =
            kRenderTransAlpha as libc::c_int;
        (*pTemp).entity.curstate.renderamt = 255 as libc::c_int;
        i += 1
    };
}
/*
==============
R_Bubbles

Create bubbles
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_Bubbles(mut mins: *const vec_t,
                                   mut maxs: *const vec_t,
                                   mut height: libc::c_float,
                                   mut modelIndex: libc::c_int,
                                   mut count: libc::c_int,
                                   mut speed: libc::c_float) {
    let mut pTemp: *mut TEMPENTITY = 0 as *mut TEMPENTITY;
    let mut sine: libc::c_float = 0.;
    let mut cosine: libc::c_float = 0.;
    let mut angle: libc::c_float = 0.;
    let mut zspeed: libc::c_float = 0.;
    let mut origin: vec3_t = [0.; 3];
    let mut mod_0: *mut model_t = 0 as *mut model_t;
    let mut i: libc::c_int = 0;
    mod_0 = CL_ModelHandle(modelIndex);
    if mod_0.is_null() { return }
    i = 0 as libc::c_int;
    while i < count {
        origin[0 as libc::c_int as usize] =
            COM_RandomLong(*mins.offset(0 as libc::c_int as isize) as
                               libc::c_int,
                           *maxs.offset(0 as libc::c_int as isize) as
                               libc::c_int) as vec_t;
        origin[1 as libc::c_int as usize] =
            COM_RandomLong(*mins.offset(1 as libc::c_int as isize) as
                               libc::c_int,
                           *maxs.offset(1 as libc::c_int as isize) as
                               libc::c_int) as vec_t;
        origin[2 as libc::c_int as usize] =
            COM_RandomLong(*mins.offset(2 as libc::c_int as isize) as
                               libc::c_int,
                           *maxs.offset(2 as libc::c_int as isize) as
                               libc::c_int) as vec_t;
        pTemp = CL_TempEntAlloc(origin.as_mut_ptr() as *const vec_t, mod_0);
        if pTemp.is_null() { return }
        (*pTemp).flags |= 0x1 as libc::c_int;
        (*pTemp).x = origin[0 as libc::c_int as usize];
        (*pTemp).y = origin[1 as libc::c_int as usize];
        angle =
            COM_RandomFloat(-3.14159265358979323846f64 as libc::c_float,
                            3.14159265358979323846f64 as libc::c_float);
        SinCos(angle, &mut sine, &mut cosine);
        zspeed =
            COM_RandomLong(80 as libc::c_int, 140 as libc::c_int) as
                libc::c_float;
        (*pTemp).entity.baseline.origin[0 as libc::c_int as usize] =
            speed * cosine;
        (*pTemp).entity.baseline.origin[1 as libc::c_int as usize] =
            speed * sine;
        (*pTemp).entity.baseline.origin[2 as libc::c_int as usize] = zspeed;
        (*pTemp).die =
            (cl.time +
                 ((height -
                       (origin[2 as libc::c_int as usize] -
                            *mins.offset(2 as libc::c_int as isize))) /
                      zspeed) as libc::c_double - 0.1f32 as libc::c_double) as
                libc::c_float;
        (*pTemp).entity.curstate.frame =
            COM_RandomLong(0 as libc::c_int, (*pTemp).frameMax as libc::c_int)
                as libc::c_float;
        // Set sprite scale
        (*pTemp).entity.curstate.scale =
            1.0f32 / COM_RandomFloat(2.0f32, 5.0f32);
        (*pTemp).entity.curstate.rendermode =
            kRenderTransAlpha as libc::c_int;
        (*pTemp).entity.curstate.renderamt = 255 as libc::c_int;
        i += 1
    };
}
/*
==============
R_BubbleTrail

Create bubble trail
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_BubbleTrail(mut start: *const vec_t,
                                       mut end: *const vec_t,
                                       mut height: libc::c_float,
                                       mut modelIndex: libc::c_int,
                                       mut count: libc::c_int,
                                       mut speed: libc::c_float) {
    let mut pTemp: *mut TEMPENTITY = 0 as *mut TEMPENTITY;
    let mut sine: libc::c_float = 0.;
    let mut cosine: libc::c_float = 0.;
    let mut zspeed: libc::c_float = 0.;
    let mut dist: libc::c_float = 0.;
    let mut angle: libc::c_float = 0.;
    let mut origin: vec3_t = [0.; 3];
    let mut mod_0: *mut model_t = 0 as *mut model_t;
    let mut i: libc::c_int = 0;
    mod_0 = CL_ModelHandle(modelIndex);
    if mod_0.is_null() { return }
    i = 0 as libc::c_int;
    while i < count {
        dist =
            COM_RandomFloat(0 as libc::c_int as libc::c_float,
                            1.0f64 as libc::c_float);
        origin[0 as libc::c_int as usize] =
            *start.offset(0 as libc::c_int as isize) +
                dist *
                    (*end.offset(0 as libc::c_int as isize) -
                         *start.offset(0 as libc::c_int as isize));
        origin[1 as libc::c_int as usize] =
            *start.offset(1 as libc::c_int as isize) +
                dist *
                    (*end.offset(1 as libc::c_int as isize) -
                         *start.offset(1 as libc::c_int as isize));
        origin[2 as libc::c_int as usize] =
            *start.offset(2 as libc::c_int as isize) +
                dist *
                    (*end.offset(2 as libc::c_int as isize) -
                         *start.offset(2 as libc::c_int as isize));
        pTemp = CL_TempEntAlloc(origin.as_mut_ptr() as *const vec_t, mod_0);
        if pTemp.is_null() { return }
        (*pTemp).flags |= 0x1 as libc::c_int;
        (*pTemp).x = origin[0 as libc::c_int as usize];
        (*pTemp).y = origin[1 as libc::c_int as usize];
        angle =
            COM_RandomFloat(-3.14159265358979323846f64 as libc::c_float,
                            3.14159265358979323846f64 as libc::c_float);
        SinCos(angle, &mut sine, &mut cosine);
        zspeed =
            COM_RandomLong(80 as libc::c_int, 140 as libc::c_int) as
                libc::c_float;
        (*pTemp).entity.baseline.origin[0 as libc::c_int as usize] =
            speed * cosine;
        (*pTemp).entity.baseline.origin[1 as libc::c_int as usize] =
            speed * sine;
        (*pTemp).entity.baseline.origin[2 as libc::c_int as usize] = zspeed;
        (*pTemp).die =
            (cl.time +
                 ((height -
                       (origin[2 as libc::c_int as usize] -
                            *start.offset(2 as libc::c_int as isize))) /
                      zspeed) as libc::c_double - 0.1f32 as libc::c_double) as
                libc::c_float;
        (*pTemp).entity.curstate.frame =
            COM_RandomLong(0 as libc::c_int, (*pTemp).frameMax as libc::c_int)
                as libc::c_float;
        // Set sprite scale
        (*pTemp).entity.curstate.scale =
            1.0f32 / COM_RandomFloat(2.0f32, 5.0f32);
        (*pTemp).entity.curstate.rendermode =
            kRenderTransAlpha as libc::c_int;
        (*pTemp).entity.curstate.renderamt = 255 as libc::c_int;
        i += 1
    };
}
/*
==============
R_AttachTentToPlayer

Attaches entity to player
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_AttachTentToPlayer(mut client: libc::c_int,
                                              mut modelIndex: libc::c_int,
                                              mut zoffset: libc::c_float,
                                              mut life: libc::c_float) {
    let mut pTemp: *mut TEMPENTITY = 0 as *mut TEMPENTITY;
    let mut position: vec3_t = [0.; 3];
    let mut pClient: *mut cl_entity_t = 0 as *mut cl_entity_t;
    let mut pModel: *mut model_t = 0 as *mut model_t;
    if client <= 0 as libc::c_int || client > cl.maxclients { return }
    pClient = CL_GetEntityByIndex(client);
    if pClient.is_null() || (*pClient).curstate.messagenum != cl.parsecount {
        return
    }
    pModel = CL_ModelHandle(modelIndex);
    if pModel.is_null() { return }
    position[0 as libc::c_int as usize] =
        (*pClient).origin[0 as libc::c_int as usize];
    position[1 as libc::c_int as usize] =
        (*pClient).origin[1 as libc::c_int as usize];
    position[2 as libc::c_int as usize] =
        (*pClient).origin[2 as libc::c_int as usize];
    position[2 as libc::c_int as usize] += zoffset;
    pTemp =
        CL_TempEntAllocHigh(position.as_mut_ptr() as *const vec_t, pModel);
    if pTemp.is_null() { return }
    (*pTemp).entity.curstate.renderfx = kRenderFxNoDissipation as libc::c_int;
    (*pTemp).entity.curstate.framerate = 1 as libc::c_int as libc::c_float;
    (*pTemp).clientIndex = client as libc::c_short;
    (*pTemp).tentOffset[0 as libc::c_int as usize] =
        0 as libc::c_int as vec_t;
    (*pTemp).tentOffset[1 as libc::c_int as usize] =
        0 as libc::c_int as vec_t;
    (*pTemp).tentOffset[2 as libc::c_int as usize] = zoffset;
    (*pTemp).die = (cl.time + life as libc::c_double) as libc::c_float;
    (*pTemp).flags |= 0x8000 as libc::c_int | 0x2000 as libc::c_int;
    // is the model a sprite?
    if (*pModel).type_0 as libc::c_int == mod_sprite as libc::c_int {
        (*pTemp).flags |= 0x100 as libc::c_int | 0x10000 as libc::c_int;
        (*pTemp).entity.curstate.framerate =
            10 as libc::c_int as libc::c_float
    } else {
        // no animation support for attached clientside studio models.
        (*pTemp).frameMax = 0 as libc::c_int as libc::c_float
    }
    (*pTemp).entity.curstate.frame = 0 as libc::c_int as libc::c_float;
}
/*
==============
R_KillAttachedTents

Detach entity from player
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_KillAttachedTents(mut client: libc::c_int) {
    let mut i: libc::c_int = 0;
    if client <= 0 as libc::c_int || client > cl.maxclients { return }
    i = 0 as libc::c_int;
    while i < (*SI.GameInfo).max_tents {
        let mut pTemp: *mut TEMPENTITY =
            &mut *cl_tempents.offset(i as isize) as *mut TEMPENTITY;
        if !((*pTemp).flags & 0x8000 as libc::c_int == 0) {
            // this TEMPENTITY is player attached.
		// if it is attached to this client, set it to die instantly.
            if (*pTemp).clientIndex as libc::c_int == client {
                // good enough, it will die on next tent update.
                (*pTemp).die = cl.time as libc::c_float
            }
        }
        i += 1
    };
}
/*
==============
R_RicochetSprite

Create ricochet sprite
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_RicochetSprite(mut pos: *const vec_t,
                                          mut pmodel: *mut model_t,
                                          mut duration: libc::c_float,
                                          mut scale: libc::c_float) {
    let mut pTemp: *mut TEMPENTITY = 0 as *mut TEMPENTITY;
    pTemp = CL_TempEntAlloc(pos, pmodel);
    if pTemp.is_null() { return }
    (*pTemp).entity.curstate.rendermode = kRenderGlow as libc::c_int;
    (*pTemp).entity.baseline.renderamt = 200 as libc::c_int;
    (*pTemp).entity.curstate.renderamt = (*pTemp).entity.baseline.renderamt;
    (*pTemp).entity.curstate.renderfx = kRenderFxNoDissipation as libc::c_int;
    (*pTemp).entity.curstate.scale = scale;
    (*pTemp).die = (cl.time + duration as libc::c_double) as libc::c_float;
    (*pTemp).flags = 0x80 as libc::c_int;
    (*pTemp).fadeSpeed = 8 as libc::c_int as libc::c_float;
    (*pTemp).entity.curstate.frame = 0 as libc::c_int as libc::c_float;
    (*pTemp).entity.angles[2 as libc::c_int as usize] =
        45.0f32 *
            COM_RandomLong(0 as libc::c_int, 7 as libc::c_int) as
                libc::c_float;
}
/*
==============
R_RocketFlare

Create rocket flare
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_RocketFlare(mut pos: *const vec_t) {
    let mut pTemp: *mut TEMPENTITY =
        0 as *mut TEMPENTITY; // when 100 fps die at next frame
    if cl_sprite_glow.is_null() { return }
    pTemp = CL_TempEntAlloc(pos, cl_sprite_glow);
    if pTemp.is_null() { return }
    (*pTemp).entity.curstate.rendermode = kRenderGlow as libc::c_int;
    (*pTemp).entity.curstate.renderfx = kRenderFxNoDissipation as libc::c_int;
    (*pTemp).entity.curstate.renderamt = 200 as libc::c_int;
    (*pTemp).entity.curstate.framerate = 1.0f64 as libc::c_float;
    (*pTemp).entity.curstate.frame =
        COM_RandomLong(0 as libc::c_int, (*pTemp).frameMax as libc::c_int) as
            libc::c_float;
    (*pTemp).entity.curstate.scale = 1.0f64 as libc::c_float;
    (*pTemp).die = (cl.time + 0.01f32 as libc::c_double) as libc::c_float;
    (*pTemp).entity.curstate.effects = 32 as libc::c_int;
}
/*
==============
R_MuzzleFlash

Do muzzleflash
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_MuzzleFlash(mut pos: *const vec_t,
                                       mut type_0: libc::c_int) {
    let mut pTemp: *mut TEMPENTITY = 0 as *mut TEMPENTITY;
    let mut index: libc::c_int = 0;
    let mut scale: libc::c_float = 0.;
    index = type_0 % 10 as libc::c_int % 3 as libc::c_int;
    scale = (type_0 / 10 as libc::c_int) as libc::c_float * 0.1f32;
    if scale == 0.0f32 { scale = 0.5f32 }
    if cl_sprite_muzzleflash[index as usize].is_null() { return }
    // must set position for right culling on render
    pTemp =
        CL_TempEntAllocHigh(pos,
                            cl_sprite_muzzleflash[index as
                                                      usize]); // die at next frame
    if pTemp.is_null() { return } // rifle flash
    (*pTemp).entity.curstate.rendermode = kRenderTransAdd as libc::c_int;
    (*pTemp).entity.curstate.renderamt = 255 as libc::c_int;
    (*pTemp).entity.curstate.framerate = 10 as libc::c_int as libc::c_float;
    (*pTemp).entity.curstate.renderfx = 0 as libc::c_int;
    (*pTemp).die = (cl.time + 0.01f64) as libc::c_float;
    (*pTemp).entity.curstate.frame =
        COM_RandomLong(0 as libc::c_int, (*pTemp).frameMax as libc::c_int) as
            libc::c_float;
    (*pTemp).flags |= 0x100 as libc::c_int | 0x10000 as libc::c_int;
    (*pTemp).entity.curstate.scale = scale;
    if index == 0 as libc::c_int {
        (*pTemp).entity.angles[2 as libc::c_int as usize] =
            COM_RandomLong(0 as libc::c_int, 20 as libc::c_int) as vec_t
    } else {
        (*pTemp).entity.angles[2 as libc::c_int as usize] =
            COM_RandomLong(0 as libc::c_int, 359 as libc::c_int) as vec_t
    }
    CL_TempEntAddEntity(&mut (*pTemp).entity);
}
/*
==============
R_BloodSprite

Create a high priority blood sprite
and some blood drops. This is high-priority tent
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_BloodSprite(mut org: *const vec_t,
                                       mut colorIndex: libc::c_int,
                                       mut modelIndex: libc::c_int,
                                       mut modelIndex2: libc::c_int,
                                       mut size: libc::c_float) {
    let mut pModel: *mut model_t = 0 as *mut model_t;
    let mut pModel2: *mut model_t = 0 as *mut model_t;
    let mut impactindex: libc::c_int = 0;
    let mut spatterindex: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut splatter: libc::c_int = 0;
    let mut pTemp: *mut TEMPENTITY = 0 as *mut TEMPENTITY;
    let mut pos: vec3_t = [0.; 3];
    colorIndex += COM_RandomLong(1 as libc::c_int, 3 as libc::c_int);
    impactindex = colorIndex;
    spatterindex = colorIndex - 1 as libc::c_int;
    // validate the model first
    pModel =
        CL_ModelHandle(modelIndex); // make offset from ground (snarks issues)
    if !pModel.is_null() {
        pos[0 as libc::c_int as usize] =
            *org.offset(0 as libc::c_int as isize);
        pos[1 as libc::c_int as usize] =
            *org.offset(1 as libc::c_int as isize);
        pos[2 as libc::c_int as usize] =
            *org.offset(2 as libc::c_int as isize);
        pos[2 as libc::c_int as usize] += COM_RandomFloat(2.0f32, 4.0f32);
        // large, single blood sprite is a high-priority tent
        pTemp =
            CL_TempEntAllocHigh(pos.as_mut_ptr() as *const vec_t,
                                pModel); // Finish in 0.250 seconds
        if !pTemp.is_null() {
            (*pTemp).entity.curstate.rendermode =
                kRenderTransTexture as
                    libc::c_int; // play the whole thing once
            (*pTemp).entity.curstate.renderfx =
                kRenderFxClampMinScale as libc::c_int;
            (*pTemp).entity.curstate.scale =
                COM_RandomFloat(size / 25.0f32, size / 35.0f32);
            (*pTemp).flags = 0x100 as libc::c_int;
            (*pTemp).entity.curstate.rendercolor =
                clgame.palette[impactindex as usize];
            (*pTemp).entity.curstate.renderamt = 250 as libc::c_int;
            (*pTemp).entity.baseline.renderamt =
                (*pTemp).entity.curstate.renderamt;
            (*pTemp).entity.curstate.framerate = (*pTemp).frameMax * 4.0f32;
            (*pTemp).die =
                (cl.time +
                     ((*pTemp).frameMax / (*pTemp).entity.curstate.framerate)
                         as libc::c_double) as libc::c_float;
            (*pTemp).entity.curstate.frame =
                0 as libc::c_int as libc::c_float;
            (*pTemp).bounceFactor = 0 as libc::c_int as libc::c_float;
            (*pTemp).entity.angles[2 as libc::c_int as usize] =
                COM_RandomLong(0 as libc::c_int, 360 as libc::c_int) as vec_t
        }
    }
    // validate the model first
    pModel2 = CL_ModelHandle(modelIndex2);
    if !pModel2.is_null() {
        splatter =
            (size +
                 (COM_RandomLong(1 as libc::c_int, 8 as libc::c_int) +
                      COM_RandomLong(1 as libc::c_int, 8 as libc::c_int)) as
                     libc::c_float) as libc::c_int;
        i = 0 as libc::c_int;
        while i < splatter {
            // create blood drips
            pTemp = CL_TempEntAlloc(org, pModel2);
            if !pTemp.is_null() {
                (*pTemp).entity.curstate.rendermode =
                    kRenderTransTexture as libc::c_int;
                (*pTemp).entity.curstate.renderfx =
                    kRenderFxClampMinScale as libc::c_int;
                (*pTemp).entity.curstate.scale =
                    COM_RandomFloat(size / 15.0f32, size / 25.0f32);
                (*pTemp).flags =
                    0x4 as libc::c_int | 0x8 as libc::c_int |
                        0x20 as libc::c_int;
                (*pTemp).entity.curstate.rendercolor =
                    clgame.palette[spatterindex as usize];
                (*pTemp).entity.curstate.renderamt = 250 as libc::c_int;
                (*pTemp).entity.baseline.renderamt =
                    (*pTemp).entity.curstate.renderamt;
                (*pTemp).entity.baseline.origin[0 as libc::c_int as usize] =
                    COM_RandomFloat(-96.0f32, 95.0f32);
                (*pTemp).entity.baseline.origin[1 as libc::c_int as usize] =
                    COM_RandomFloat(-96.0f32, 95.0f32);
                (*pTemp).entity.baseline.origin[2 as libc::c_int as usize] =
                    COM_RandomFloat(-32.0f32, 95.0f32);
                (*pTemp).entity.baseline.angles[0 as libc::c_int as usize] =
                    COM_RandomFloat(-256.0f32, -255.0f32);
                (*pTemp).entity.baseline.angles[1 as libc::c_int as usize] =
                    COM_RandomFloat(-256.0f32, -255.0f32);
                (*pTemp).entity.baseline.angles[2 as libc::c_int as usize] =
                    COM_RandomFloat(-256.0f32, -255.0f32);
                (*pTemp).die =
                    (cl.time +
                         COM_RandomFloat(1.0f32, 3.0f32) as libc::c_double) as
                        libc::c_float;
                (*pTemp).entity.curstate.frame =
                    COM_RandomLong(1 as libc::c_int,
                                   (*pTemp).frameMax as libc::c_int) as
                        libc::c_float;
                if (*pTemp).entity.curstate.frame > 8.0f32 {
                    (*pTemp).entity.curstate.frame = (*pTemp).frameMax
                }
                (*pTemp).entity.baseline.origin[2 as libc::c_int as usize] +=
                    COM_RandomFloat(4.0f32, 16.0f32) * size;
                (*pTemp).entity.angles[2 as libc::c_int as usize] =
                    COM_RandomFloat(0.0f32, 360.0f32);
                (*pTemp).bounceFactor = 0.0f32
            }
            i += 1
        }
    };
}
/*
==============
R_BreakModel

Create a shards
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_BreakModel(mut pos: *const vec_t,
                                      mut size: *const vec_t,
                                      mut dir: *const vec_t,
                                      mut random: libc::c_float,
                                      mut life: libc::c_float,
                                      mut count: libc::c_int,
                                      mut modelIndex: libc::c_int,
                                      mut flags: libc::c_char) {
    let mut pTemp: *mut TEMPENTITY = 0 as *mut TEMPENTITY;
    let mut pmodel: *mut model_t = 0 as *mut model_t;
    let mut type_0: libc::c_char = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    pmodel = CL_ModelHandle(modelIndex);
    if pmodel.is_null() { return }
    type_0 = (flags as libc::c_int & 0x4f as libc::c_int) as libc::c_char;
    if count == 0 as libc::c_int {
        // assume surface (not volume)
        count =
            ((*size.offset(0 as libc::c_int as isize) *
                  *size.offset(1 as libc::c_int as isize) +
                  *size.offset(1 as libc::c_int as isize) *
                      *size.offset(2 as libc::c_int as isize) +
                  *size.offset(2 as libc::c_int as isize) *
                      *size.offset(0 as libc::c_int as isize)) /
                 (3 as libc::c_int as libc::c_float * 12.0f32 * 12.0f32)) as
                libc::c_int
    }
    // limit to 100 pieces
    if count > 100 as libc::c_int { count = 100 as libc::c_int }
    i = 0 as libc::c_int;
    while i < count {
        let mut vecSpot: vec3_t = [0.; 3];
        j = 0 as libc::c_int;
        while j < 32 as libc::c_int {
            // Add an extra 0-1 secs of life
            // fill up the box with stuff
            vecSpot[0 as libc::c_int as usize] =
                *pos.offset(0 as libc::c_int as isize) +
                    COM_RandomFloat(-0.5f32, 0.5f32) *
                        *size.offset(0 as libc::c_int as isize);
            vecSpot[1 as libc::c_int as usize] =
                *pos.offset(1 as libc::c_int as isize) +
                    COM_RandomFloat(-0.5f32, 0.5f32) *
                        *size.offset(1 as libc::c_int as isize);
            vecSpot[2 as libc::c_int as usize] =
                *pos.offset(2 as libc::c_int as isize) +
                    COM_RandomFloat(-0.5f32, 0.5f32) *
                        *size.offset(2 as libc::c_int as isize);
            if CL_PointContents(vecSpot.as_mut_ptr() as *const vec_t) !=
                   -(2 as libc::c_int) {
                break ;
            }
            j += 1
            // valid spot
        } // a piece completely stuck in the wall, ignore it
        if !(j == 32 as libc::c_int) {
            pTemp =
                CL_TempEntAlloc(vecSpot.as_mut_ptr() as *const vec_t, pmodel);
            if pTemp.is_null() { return }
            // keep track of break_type, so we know how to play sound on collision
            (*pTemp).hitSound = type_0 as libc::c_int;
            if (*pmodel).type_0 as libc::c_int == mod_sprite as libc::c_int {
                (*pTemp).entity.curstate.frame =
                    COM_RandomLong(0 as libc::c_int,
                                   (*pTemp).frameMax as libc::c_int) as
                        libc::c_float
            } else if (*pmodel).type_0 as libc::c_int ==
                          mod_studio as libc::c_int {
                (*pTemp).entity.curstate.body =
                    COM_RandomLong(0 as libc::c_int,
                                   (*pTemp).frameMax as libc::c_int)
            }
            (*pTemp).flags |=
                0x20 as libc::c_int | 0x80 as libc::c_int |
                    0x8 as libc::c_int;
            if COM_RandomLong(0 as libc::c_int, 255 as libc::c_int) <
                   200 as libc::c_int {
                (*pTemp).flags |= 0x4 as libc::c_int;
                (*pTemp).entity.baseline.angles[0 as libc::c_int as usize] =
                    COM_RandomFloat(-(256 as libc::c_int) as libc::c_float,
                                    255 as libc::c_int as libc::c_float);
                (*pTemp).entity.baseline.angles[1 as libc::c_int as usize] =
                    COM_RandomFloat(-(256 as libc::c_int) as libc::c_float,
                                    255 as libc::c_int as libc::c_float);
                (*pTemp).entity.baseline.angles[2 as libc::c_int as usize] =
                    COM_RandomFloat(-(256 as libc::c_int) as libc::c_float,
                                    255 as libc::c_int as libc::c_float)
            }
            if COM_RandomLong(0 as libc::c_int, 255 as libc::c_int) <
                   100 as libc::c_int &&
                   flags as libc::c_int & 0x10 as libc::c_int != 0 {
                (*pTemp).flags |= 0x10 as libc::c_int
            }
            if type_0 as libc::c_int == 0x1 as libc::c_int ||
                   flags as libc::c_int & 0x20 as libc::c_int != 0 {
                (*pTemp).entity.curstate.rendermode =
                    kRenderTransTexture as libc::c_int;
                (*pTemp).entity.baseline.renderamt = 128 as libc::c_int;
                (*pTemp).entity.curstate.renderamt =
                    (*pTemp).entity.baseline.renderamt
            } else {
                (*pTemp).entity.curstate.rendermode =
                    kRenderNormal as libc::c_int;
                (*pTemp).entity.baseline.renderamt = 255 as libc::c_int;
                (*pTemp).entity.curstate.renderamt =
                    (*pTemp).entity.baseline.renderamt
                // set this for fadeout
            }
            (*pTemp).entity.baseline.origin[0 as libc::c_int as usize] =
                *dir.offset(0 as libc::c_int as isize) +
                    COM_RandomFloat(-random, random);
            (*pTemp).entity.baseline.origin[1 as libc::c_int as usize] =
                *dir.offset(1 as libc::c_int as isize) +
                    COM_RandomFloat(-random, random);
            (*pTemp).entity.baseline.origin[2 as libc::c_int as usize] =
                *dir.offset(2 as libc::c_int as isize) +
                    COM_RandomFloat(0 as libc::c_int as libc::c_float,
                                    random);
            (*pTemp).die =
                (cl.time + life as libc::c_double +
                     COM_RandomFloat(0.0f32, 1.0f32) as libc::c_double) as
                    libc::c_float
        }
        i += 1
    };
}
/*
==============
R_TempModel

Create a temp model with gravity, sounds and fadeout
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_TempModel(mut pos: *const vec_t,
                                     mut dir: *const vec_t,
                                     mut angles: *const vec_t,
                                     mut life: libc::c_float,
                                     mut modelIndex: libc::c_int,
                                     mut soundtype: libc::c_int)
 -> *mut tempent_s {
    // alloc a new tempent
    let mut pTemp: *mut TEMPENTITY = 0 as *mut TEMPENTITY;
    let mut pmodel: *mut model_t = 0 as *mut model_t;
    pmodel = CL_ModelHandle(modelIndex);
    if pmodel.is_null() { return 0 as *mut tempent_s }
    pTemp = CL_TempEntAlloc(pos, pmodel);
    if pTemp.is_null() { return 0 as *mut tempent_s }
    (*pTemp).flags = 0x20 as libc::c_int | 0x2 as libc::c_int;
    (*pTemp).entity.baseline.origin[0 as libc::c_int as usize] =
        *dir.offset(0 as libc::c_int as isize);
    (*pTemp).entity.baseline.origin[1 as libc::c_int as usize] =
        *dir.offset(1 as libc::c_int as isize);
    (*pTemp).entity.baseline.origin[2 as libc::c_int as usize] =
        *dir.offset(2 as libc::c_int as isize);
    (*pTemp).entity.angles[0 as libc::c_int as usize] =
        *angles.offset(0 as libc::c_int as isize);
    (*pTemp).entity.angles[1 as libc::c_int as usize] =
        *angles.offset(1 as libc::c_int as isize);
    (*pTemp).entity.angles[2 as libc::c_int as usize] =
        *angles.offset(2 as libc::c_int as isize);
    // keep track of shell type
    match soundtype {
        1 => {
            (*pTemp).hitSound = 0x20 as libc::c_int;
            (*pTemp).entity.baseline.angles[0 as libc::c_int as usize] =
                COM_RandomFloat(-(512 as libc::c_int) as libc::c_float,
                                511 as libc::c_int as libc::c_float);
            (*pTemp).entity.baseline.angles[1 as libc::c_int as usize] =
                COM_RandomFloat(-(255 as libc::c_int) as libc::c_float,
                                255 as libc::c_int as libc::c_float);
            (*pTemp).entity.baseline.angles[2 as libc::c_int as usize] =
                COM_RandomFloat(-(255 as libc::c_int) as libc::c_float,
                                255 as libc::c_int as libc::c_float);
            (*pTemp).flags |= 0x4 as libc::c_int
        }
        2 => {
            (*pTemp).hitSound = 0x80 as libc::c_int;
            (*pTemp).entity.baseline.angles[0 as libc::c_int as usize] =
                COM_RandomFloat(-(512 as libc::c_int) as libc::c_float,
                                511 as libc::c_int as libc::c_float);
            (*pTemp).entity.baseline.angles[1 as libc::c_int as usize] =
                COM_RandomFloat(-(255 as libc::c_int) as libc::c_float,
                                255 as libc::c_int as libc::c_float);
            (*pTemp).entity.baseline.angles[2 as libc::c_int as usize] =
                COM_RandomFloat(-(255 as libc::c_int) as libc::c_float,
                                255 as libc::c_int as libc::c_float);
            (*pTemp).flags |= 0x4 as libc::c_int | 0x8 as libc::c_int
        }
        _ => { }
    }
    if (*pmodel).type_0 as libc::c_int == mod_sprite as libc::c_int {
        (*pTemp).entity.curstate.frame =
            COM_RandomLong(0 as libc::c_int, (*pTemp).frameMax as libc::c_int)
                as libc::c_float
    } else {
        (*pTemp).entity.curstate.body =
            COM_RandomLong(0 as libc::c_int, (*pTemp).frameMax as libc::c_int)
    }
    (*pTemp).die = (cl.time + life as libc::c_double) as libc::c_float;
    return pTemp;
}
/*
==============
R_DefaultSprite

Create an animated sprite
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_DefaultSprite(mut pos: *const vec_t,
                                         mut spriteIndex: libc::c_int,
                                         mut framerate: libc::c_float)
 -> *mut tempent_s {
    let mut pTemp: *mut TEMPENTITY = 0 as *mut TEMPENTITY;
    let mut psprite: *mut model_t = 0 as *mut model_t;
    // don't spawn while paused
    if cl.time == cl.oldtime { return 0 as *mut tempent_s }
    psprite = CL_ModelHandle(spriteIndex);
    if psprite.is_null() ||
           (*psprite).type_0 as libc::c_int != mod_sprite as libc::c_int {
        Con_Reportf(b"No Sprite %d!\n\x00" as *const u8 as
                        *const libc::c_char, spriteIndex);
        return 0 as *mut tempent_s
    }
    pTemp = CL_TempEntAlloc(pos, psprite);
    if pTemp.is_null() { return 0 as *mut tempent_s }
    (*pTemp).entity.curstate.scale = 1.0f32;
    (*pTemp).flags |= 0x100 as libc::c_int;
    if framerate == 0 as libc::c_int as libc::c_float {
        framerate = 10 as libc::c_int as libc::c_float
    }
    (*pTemp).entity.curstate.framerate = framerate;
    (*pTemp).die =
        (cl.time + ((*pTemp).frameMax / framerate) as libc::c_double) as
            libc::c_float;
    (*pTemp).entity.curstate.frame = 0 as libc::c_int as libc::c_float;
    return pTemp;
}
/*
===============
R_SparkShower

Create an animated moving sprite
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_SparkShower(mut pos: *const vec_t) {
    let mut pTemp: *mut TEMPENTITY = 0 as *mut TEMPENTITY;
    pTemp = CL_TempEntAllocNoModel(pos);
    if pTemp.is_null() { return }
    (*pTemp).entity.baseline.origin[0 as libc::c_int as usize] =
        COM_RandomFloat(-300.0f32, 300.0f32);
    (*pTemp).entity.baseline.origin[1 as libc::c_int as usize] =
        COM_RandomFloat(-300.0f32, 300.0f32);
    (*pTemp).entity.baseline.origin[2 as libc::c_int as usize] =
        COM_RandomFloat(-200.0f32, 200.0f32);
    (*pTemp).flags |=
        0x8 as libc::c_int | 0x20 as libc::c_int | 0x20000 as libc::c_int;
    (*pTemp).entity.curstate.framerate = COM_RandomFloat(0.5f32, 1.5f32);
    (*pTemp).entity.curstate.scale = cl.time as libc::c_float;
    (*pTemp).die = (cl.time + 0.5f64) as libc::c_float;
}
/*
===============
R_TempSprite

Create an animated moving sprite
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_TempSprite(mut pos: *mut vec_t,
                                      mut dir: *const vec_t,
                                      mut scale: libc::c_float,
                                      mut modelIndex: libc::c_int,
                                      mut rendermode: libc::c_int,
                                      mut renderfx: libc::c_int,
                                      mut a: libc::c_float,
                                      mut life: libc::c_float,
                                      mut flags: libc::c_int)
 -> *mut tempent_s {
    let mut pTemp: *mut TEMPENTITY = 0 as *mut TEMPENTITY;
    let mut pmodel: *mut model_t = 0 as *mut model_t;
    pmodel = CL_ModelHandle(modelIndex);
    if pmodel.is_null() {
        Con_Reportf(b"^1Error:^7 No model %d!\n\x00" as *const u8 as
                        *const libc::c_char, modelIndex);
        return 0 as *mut tempent_s
    }
    pTemp = CL_TempEntAlloc(pos as *const vec_t, pmodel);
    if pTemp.is_null() { return 0 as *mut tempent_s }
    (*pTemp).entity.curstate.framerate = 10 as libc::c_int as libc::c_float;
    (*pTemp).entity.curstate.rendermode = rendermode;
    (*pTemp).entity.curstate.renderfx = renderfx;
    (*pTemp).entity.curstate.scale = scale;
    (*pTemp).entity.baseline.renderamt =
        (a * 255 as libc::c_int as libc::c_float) as libc::c_int;
    (*pTemp).entity.curstate.renderamt =
        (a * 255 as libc::c_int as libc::c_float) as libc::c_int;
    (*pTemp).flags |= flags;
    (*pTemp).entity.baseline.origin[0 as libc::c_int as usize] =
        *dir.offset(0 as libc::c_int as isize);
    (*pTemp).entity.baseline.origin[1 as libc::c_int as usize] =
        *dir.offset(1 as libc::c_int as isize);
    (*pTemp).entity.baseline.origin[2 as libc::c_int as usize] =
        *dir.offset(2 as libc::c_int as isize);
    if life != 0. {
        (*pTemp).die = (cl.time + life as libc::c_double) as libc::c_float
    } else {
        (*pTemp).die =
            (cl.time + ((*pTemp).frameMax * 0.1f32) as libc::c_double +
                 1.0f32 as libc::c_double) as libc::c_float
    }
    (*pTemp).entity.curstate.frame = 0 as libc::c_int as libc::c_float;
    return pTemp;
}
/*
===============
R_Sprite_Explode

apply params for exploding sprite
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_Sprite_Explode(mut pTemp: *mut TEMPENTITY,
                                          mut scale: libc::c_float,
                                          mut flags: libc::c_int) {
    if pTemp.is_null() { return }
    if flags & 1 as libc::c_int != 0 {
        // solid sprite
        (*pTemp).entity.curstate.rendermode = kRenderNormal as libc::c_int;
        (*pTemp).entity.curstate.renderamt = 255 as libc::c_int
    } else if flags & 16 as libc::c_int != 0 {
        // alpha sprite (came from hl2)
        (*pTemp).entity.curstate.rendermode =
            kRenderTransAlpha as libc::c_int;
        (*pTemp).entity.curstate.renderamt = 180 as libc::c_int
    } else {
        // additive sprite
        (*pTemp).entity.curstate.rendermode = kRenderTransAdd as libc::c_int;
        (*pTemp).entity.curstate.renderamt = 180 as libc::c_int
    }
    if flags & 32 as libc::c_int != 0 {
        // came from hl2
        (*pTemp).entity.angles[2 as libc::c_int as usize] =
            COM_RandomLong(0 as libc::c_int, 360 as libc::c_int) as vec_t
    }
    (*pTemp).entity.curstate.renderfx = kRenderFxNone as libc::c_int;
    (*pTemp).entity.baseline.origin[2 as libc::c_int as usize] =
        8 as libc::c_int as vec_t;
    (*pTemp).entity.origin[2 as libc::c_int as usize] +=
        10 as libc::c_int as libc::c_float;
    (*pTemp).entity.curstate.scale = scale;
}
/*
===============
R_Sprite_Smoke

apply params for smoke sprite
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_Sprite_Smoke(mut pTemp: *mut TEMPENTITY,
                                        mut scale: libc::c_float) {
    let mut iColor: libc::c_int = 0;
    if pTemp.is_null() { return }
    iColor = COM_RandomLong(20 as libc::c_int, 35 as libc::c_int);
    (*pTemp).entity.curstate.rendermode = kRenderTransAlpha as libc::c_int;
    (*pTemp).entity.curstate.renderfx = kRenderFxNone as libc::c_int;
    (*pTemp).entity.baseline.origin[2 as libc::c_int as usize] =
        30 as libc::c_int as vec_t;
    (*pTemp).entity.curstate.rendercolor.r = iColor as byte;
    (*pTemp).entity.curstate.rendercolor.g = iColor as byte;
    (*pTemp).entity.curstate.rendercolor.b = iColor as byte;
    (*pTemp).entity.origin[2 as libc::c_int as usize] +=
        20 as libc::c_int as libc::c_float;
    (*pTemp).entity.curstate.scale = scale;
}
/*
===============
R_Spray

Throws a shower of sprites or models
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_Spray(mut pos: *const vec_t, mut dir: *const vec_t,
                                 mut modelIndex: libc::c_int,
                                 mut count: libc::c_int,
                                 mut speed: libc::c_int,
                                 mut spread: libc::c_int,
                                 mut rendermode: libc::c_int) {
    let mut pTemp: *mut TEMPENTITY = 0 as *mut TEMPENTITY;
    let mut noise: libc::c_float = 0.;
    let mut znoise: libc::c_float = 0.;
    let mut pmodel: *mut model_t = 0 as *mut model_t;
    let mut i: libc::c_int = 0;
    pmodel = CL_ModelHandle(modelIndex);
    if pmodel.is_null() {
        Con_Reportf(b"No model %d!\n\x00" as *const u8 as *const libc::c_char,
                    modelIndex);
        return
    }
    noise = spread as libc::c_float / 100.0f32;
    // more vertical displacement
    znoise = if 1.0f32 < noise * 1.5f32 { 1.0f32 } else { (noise) * 1.5f32 };
    i = 0 as libc::c_int;
    while i < count {
        pTemp = CL_TempEntAlloc(pos, pmodel);
        if pTemp.is_null() { return }
        (*pTemp).entity.curstate.rendermode = rendermode;
        (*pTemp).entity.curstate.renderamt = 255 as libc::c_int;
        (*pTemp).entity.baseline.renderamt =
            (*pTemp).entity.curstate.renderamt;
        (*pTemp).entity.curstate.renderfx =
            kRenderFxNoDissipation as libc::c_int;
        if rendermode != kRenderGlow as libc::c_int {
            // spray
            (*pTemp).flags |= 0x20 as libc::c_int | 0x8 as libc::c_int;
            if (*pTemp).frameMax > 1 as libc::c_int as libc::c_float {
                (*pTemp).flags |=
                    0x20 as libc::c_int | 0x8 as libc::c_int |
                        0x100 as libc::c_int;
                (*pTemp).die =
                    (cl.time + ((*pTemp).frameMax * 0.1f32) as libc::c_double)
                        as libc::c_float;
                (*pTemp).entity.curstate.framerate =
                    10 as libc::c_int as libc::c_float
            } else {
                (*pTemp).die =
                    (cl.time + 0.35f32 as libc::c_double) as libc::c_float
            }
        } else {
            // sprite spray
            (*pTemp).entity.curstate.frame =
                COM_RandomLong(0 as libc::c_int,
                               (*pTemp).frameMax as libc::c_int) as
                    libc::c_float;
            (*pTemp).flags |= 0x80 as libc::c_int | 0x8 as libc::c_int;
            (*pTemp).entity.curstate.framerate = 0.5f64 as libc::c_float;
            (*pTemp).die =
                (cl.time + 0.35f32 as libc::c_double) as libc::c_float;
            (*pTemp).fadeSpeed = 2.0f64 as libc::c_float
        }
        // make the spittle fly the direction indicated, but mix in some noise.
        (*pTemp).entity.baseline.origin[0 as libc::c_int as usize] =
            *dir.offset(0 as libc::c_int as isize) +
                COM_RandomFloat(-noise, noise);
        (*pTemp).entity.baseline.origin[1 as libc::c_int as usize] =
            *dir.offset(1 as libc::c_int as isize) +
                COM_RandomFloat(-noise, noise);
        (*pTemp).entity.baseline.origin[2 as libc::c_int as usize] =
            *dir.offset(2 as libc::c_int as isize) +
                COM_RandomFloat(0 as libc::c_int as libc::c_float, znoise);
        (*pTemp).entity.baseline.origin[0 as libc::c_int as usize] =
            (*pTemp).entity.baseline.origin[0 as libc::c_int as usize] *
                COM_RandomFloat(speed as libc::c_float * 0.8f32,
                                speed as libc::c_float * 1.2f32);
        (*pTemp).entity.baseline.origin[1 as libc::c_int as usize] =
            (*pTemp).entity.baseline.origin[1 as libc::c_int as usize] *
                COM_RandomFloat(speed as libc::c_float * 0.8f32,
                                speed as libc::c_float * 1.2f32);
        (*pTemp).entity.baseline.origin[2 as libc::c_int as usize] =
            (*pTemp).entity.baseline.origin[2 as libc::c_int as usize] *
                COM_RandomFloat(speed as libc::c_float * 0.8f32,
                                speed as libc::c_float * 1.2f32);
        i += 1
    };
}
/*
===============
R_Sprite_Spray

Spray of alpha sprites
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_Sprite_Spray(mut pos: *const vec_t,
                                        mut dir: *const vec_t,
                                        mut modelIndex: libc::c_int,
                                        mut count: libc::c_int,
                                        mut speed: libc::c_int,
                                        mut spread: libc::c_int) {
    R_Spray(pos, dir, modelIndex, count, speed, spread,
            kRenderGlow as libc::c_int);
}
/*
===============
R_Sprite_Trail

Line of moving glow sprites with gravity,
fadeout, and collisions
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_Sprite_Trail(mut type_0: libc::c_int,
                                        mut start: *mut vec_t,
                                        mut end: *mut vec_t,
                                        mut modelIndex: libc::c_int,
                                        mut count: libc::c_int,
                                        mut life: libc::c_float,
                                        mut size: libc::c_float,
                                        mut amp: libc::c_float,
                                        mut renderamt: libc::c_int,
                                        mut speed: libc::c_float) {
    let mut pTemp: *mut TEMPENTITY = 0 as *mut TEMPENTITY;
    let mut delta: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
    let mut pmodel: *mut model_t = 0 as *mut model_t;
    let mut i: libc::c_int = 0;
    pmodel = CL_ModelHandle(modelIndex);
    if pmodel.is_null() { return }
    delta[0 as libc::c_int as usize] =
        *end.offset(0 as libc::c_int as isize) -
            *start.offset(0 as libc::c_int as isize);
    delta[1 as libc::c_int as usize] =
        *end.offset(1 as libc::c_int as isize) -
            *start.offset(1 as libc::c_int as isize);
    delta[2 as libc::c_int as usize] =
        *end.offset(2 as libc::c_int as isize) -
            *start.offset(2 as libc::c_int as isize);
    let mut ilength: libc::c_float =
        __tg_sqrt(delta[0 as libc::c_int as usize] *
                      delta[0 as libc::c_int as usize] +
                      delta[1 as libc::c_int as usize] *
                          delta[1 as libc::c_int as usize] +
                      delta[2 as libc::c_int as usize] *
                          delta[2 as libc::c_int as usize]);
    if ilength != 0. { ilength = 1.0f32 / ilength }
    dir[0 as libc::c_int as usize] =
        delta[0 as libc::c_int as usize] * ilength;
    dir[1 as libc::c_int as usize] =
        delta[1 as libc::c_int as usize] * ilength;
    dir[2 as libc::c_int as usize] =
        delta[2 as libc::c_int as usize] * ilength;
    amp /= 256.0f32;
    i = 0 as libc::c_int;
    while i < count {
        let mut pos: vec3_t = [0.; 3];
        let mut vel: vec3_t = [0.; 3];
        // Be careful of divide by 0 when using 'count' here...
        if i == 0 as libc::c_int {
            pos[0 as libc::c_int as usize] =
                *start.offset(0 as libc::c_int as isize);
            pos[1 as libc::c_int as usize] =
                *start.offset(1 as libc::c_int as isize);
            pos[2 as libc::c_int as usize] =
                *start.offset(2 as libc::c_int as isize)
        } else {
            pos[0 as libc::c_int as usize] =
                *start.offset(0 as libc::c_int as isize) +
                    i as libc::c_float / (count as libc::c_float - 1.0f32) *
                        delta[0 as libc::c_int as usize];
            pos[1 as libc::c_int as usize] =
                *start.offset(1 as libc::c_int as isize) +
                    i as libc::c_float / (count as libc::c_float - 1.0f32) *
                        delta[1 as libc::c_int as usize];
            pos[2 as libc::c_int as usize] =
                *start.offset(2 as libc::c_int as isize) +
                    i as libc::c_float / (count as libc::c_float - 1.0f32) *
                        delta[2 as libc::c_int as usize]
        }
        pTemp = CL_TempEntAlloc(pos.as_mut_ptr() as *const vec_t, pmodel);
        if pTemp.is_null() { return }
        (*pTemp).flags =
            0x20 as libc::c_int | 0x800 as libc::c_int | 0x80 as libc::c_int |
                0x8 as libc::c_int;
        vel[0 as libc::c_int as usize] =
            dir[0 as libc::c_int as usize] * speed;
        vel[1 as libc::c_int as usize] =
            dir[1 as libc::c_int as usize] * speed;
        vel[2 as libc::c_int as usize] =
            dir[2 as libc::c_int as usize] * speed;
        vel[0 as libc::c_int as usize] +=
            COM_RandomFloat(-127.0f32, 128.0f32) * amp;
        vel[1 as libc::c_int as usize] +=
            COM_RandomFloat(-127.0f32, 128.0f32) * amp;
        vel[2 as libc::c_int as usize] +=
            COM_RandomFloat(-127.0f32, 128.0f32) * amp;
        (*pTemp).entity.baseline.origin[0 as libc::c_int as usize] =
            vel[0 as libc::c_int as usize];
        (*pTemp).entity.baseline.origin[1 as libc::c_int as usize] =
            vel[1 as libc::c_int as usize];
        (*pTemp).entity.baseline.origin[2 as libc::c_int as usize] =
            vel[2 as libc::c_int as usize];
        (*pTemp).entity.origin[0 as libc::c_int as usize] =
            pos[0 as libc::c_int as usize];
        (*pTemp).entity.origin[1 as libc::c_int as usize] =
            pos[1 as libc::c_int as usize];
        (*pTemp).entity.origin[2 as libc::c_int as usize] =
            pos[2 as libc::c_int as usize];
        (*pTemp).entity.curstate.scale = size;
        (*pTemp).entity.curstate.rendermode = kRenderGlow as libc::c_int;
        (*pTemp).entity.curstate.renderfx =
            kRenderFxNoDissipation as libc::c_int;
        (*pTemp).entity.baseline.renderamt = renderamt;
        (*pTemp).entity.curstate.renderamt =
            (*pTemp).entity.baseline.renderamt;
        (*pTemp).entity.curstate.frame =
            COM_RandomLong(0 as libc::c_int, (*pTemp).frameMax as libc::c_int)
                as libc::c_float;
        (*pTemp).die =
            (cl.time + life as libc::c_double +
                 COM_RandomFloat(0.0f32, 4.0f32) as libc::c_double) as
                libc::c_float;
        i += 1
    };
}
/*
===============
R_FunnelSprite

Create a funnel effect with custom sprite
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_FunnelSprite(mut org: *const vec_t,
                                        mut modelIndex: libc::c_int,
                                        mut reverse: libc::c_int) {
    let mut pTemp: *mut TEMPENTITY = 0 as *mut TEMPENTITY;
    let mut dir: vec3_t = [0.; 3];
    let mut dest: vec3_t = [0.; 3];
    let mut dist: libc::c_float = 0.;
    let mut vel: libc::c_float = 0.;
    let mut pmodel: *mut model_t = 0 as *mut model_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    pmodel = CL_ModelHandle(modelIndex);
    if pmodel.is_null() {
        Con_Reportf(b"^1Error:^7 no model %d!\n\x00" as *const u8 as
                        *const libc::c_char, modelIndex);
        return
    }
    i = -(8 as libc::c_int);
    while i < 8 as libc::c_int {
        j = -(8 as libc::c_int);
        while j < 8 as libc::c_int {
            pTemp = CL_TempEntAlloc(org, pmodel);
            if pTemp.is_null() { return }
            dest[0 as libc::c_int as usize] =
                i as libc::c_float * 32.0f32 +
                    *org.offset(0 as libc::c_int as isize);
            dest[1 as libc::c_int as usize] =
                j as libc::c_float * 32.0f32 +
                    *org.offset(1 as libc::c_int as isize);
            dest[2 as libc::c_int as usize] =
                *org.offset(2 as libc::c_int as isize) +
                    COM_RandomFloat(100.0f32, 800.0f32);
            if reverse != 0 {
                (*pTemp).entity.origin[0 as libc::c_int as usize] =
                    *org.offset(0 as libc::c_int as isize);
                (*pTemp).entity.origin[1 as libc::c_int as usize] =
                    *org.offset(1 as libc::c_int as isize);
                (*pTemp).entity.origin[2 as libc::c_int as usize] =
                    *org.offset(2 as libc::c_int as isize);
                dir[0 as libc::c_int as usize] =
                    dest[0 as libc::c_int as usize] -
                        (*pTemp).entity.origin[0 as libc::c_int as usize];
                dir[1 as libc::c_int as usize] =
                    dest[1 as libc::c_int as usize] -
                        (*pTemp).entity.origin[1 as libc::c_int as usize];
                dir[2 as libc::c_int as usize] =
                    dest[2 as libc::c_int as usize] -
                        (*pTemp).entity.origin[2 as libc::c_int as usize]
            } else {
                (*pTemp).entity.origin[0 as libc::c_int as usize] =
                    dest[0 as libc::c_int as usize];
                (*pTemp).entity.origin[1 as libc::c_int as usize] =
                    dest[1 as libc::c_int as usize];
                (*pTemp).entity.origin[2 as libc::c_int as usize] =
                    dest[2 as libc::c_int as usize];
                dir[0 as libc::c_int as usize] =
                    *org.offset(0 as libc::c_int as isize) -
                        (*pTemp).entity.origin[0 as libc::c_int as usize];
                dir[1 as libc::c_int as usize] =
                    *org.offset(1 as libc::c_int as isize) -
                        (*pTemp).entity.origin[1 as libc::c_int as usize];
                dir[2 as libc::c_int as usize] =
                    *org.offset(2 as libc::c_int as isize) -
                        (*pTemp).entity.origin[2 as libc::c_int as usize]
            }
            (*pTemp).entity.curstate.rendermode = kRenderGlow as libc::c_int;
            (*pTemp).entity.curstate.renderfx =
                kRenderFxNoDissipation as libc::c_int;
            (*pTemp).entity.curstate.renderamt = 200 as libc::c_int;
            (*pTemp).entity.baseline.renderamt =
                (*pTemp).entity.curstate.renderamt;
            (*pTemp).entity.baseline.angles[2 as libc::c_int as usize] =
                COM_RandomFloat(-100.0f32, 100.0f32);
            (*pTemp).entity.curstate.framerate =
                COM_RandomFloat(0.1f32, 0.4f32);
            (*pTemp).flags = 0x4 as libc::c_int | 0x80 as libc::c_int;
            (*pTemp).entity.curstate.framerate =
                10 as libc::c_int as libc::c_float;
            vel = dest[2 as libc::c_int as usize] / 8.0f32;
            if vel < 64.0f32 { vel = 64.0f32 }
            dist =
                VectorNormalizeLength2(dir.as_mut_ptr() as *const vec_t,
                                       dir.as_mut_ptr());
            vel += COM_RandomFloat(64.0f32, 128.0f32);
            (*pTemp).entity.baseline.origin[0 as libc::c_int as usize] =
                dir[0 as libc::c_int as usize] * vel;
            (*pTemp).entity.baseline.origin[1 as libc::c_int as usize] =
                dir[1 as libc::c_int as usize] * vel;
            (*pTemp).entity.baseline.origin[2 as libc::c_int as usize] =
                dir[2 as libc::c_int as usize] * vel;
            (*pTemp).die =
                (cl.time + (dist / vel) as libc::c_double -
                     0.5f32 as libc::c_double) as libc::c_float;
            (*pTemp).fadeSpeed = 2.0f32;
            j += 1
        }
        i += 1
    };
}
/*
===============
R_SparkEffect

Create a streaks + richochet sprite
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_SparkEffect(mut pos: *const vec_t,
                                       mut count: libc::c_int,
                                       mut velocityMin: libc::c_int,
                                       mut velocityMax: libc::c_int) {
    R_RicochetSprite(pos, cl_sprite_ricochet, 0.1f32,
                     COM_RandomFloat(0.5f32, 1.0f32));
    R_SparkStreaks(pos, count, velocityMin, velocityMax);
}
/*
==============
R_RicochetSound

Make a random ricochet sound
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_RicochetSound(mut pos: *const vec_t) {
    let mut iPitch: libc::c_int =
        COM_RandomLong(90 as libc::c_int, 105 as libc::c_int);
    let mut fvol: libc::c_float = COM_RandomFloat(0.7f32, 0.9f32);
    let mut soundpath: [libc::c_char; 32] = [0; 32];
    let mut handle: sound_t = 0;
    Q_strncpy(soundpath.as_mut_ptr(),
              cl_ricochet_sounds[COM_RandomLong(0 as libc::c_int,
                                                4 as libc::c_int) as usize],
              ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong);
    handle = S_RegisterSound(soundpath.as_mut_ptr());
    S_StartSound(pos, 0 as libc::c_int, 0 as libc::c_int, handle, fvol,
                 0.8f64 as libc::c_float, iPitch, 0 as libc::c_int);
}
/*
==============
R_Projectile

Create an projectile entity
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_Projectile(mut origin: *const vec_t,
                                      mut velocity: *const vec_t,
                                      mut modelIndex: libc::c_int,
                                      mut life: libc::c_int,
                                      mut owner: libc::c_int,
                                      mut hitcallback:
                                          Option<unsafe extern "C" fn(_:
                                                                          *mut TEMPENTITY,
                                                                      _:
                                                                          *mut pmtrace_t)
                                                     -> ()>) {
    let mut pTemp: *mut TEMPENTITY = 0 as *mut TEMPENTITY;
    let mut pmodel: *mut model_t = 0 as *mut model_t;
    let mut dir: vec3_t = [0.; 3];
    pmodel = CL_ModelHandle(modelIndex);
    if pmodel.is_null() { return }
    pTemp = CL_TempEntAllocHigh(origin, pmodel);
    if pTemp.is_null() { return }
    (*pTemp).entity.baseline.origin[0 as libc::c_int as usize] =
        *velocity.offset(0 as libc::c_int as isize);
    (*pTemp).entity.baseline.origin[1 as libc::c_int as usize] =
        *velocity.offset(1 as libc::c_int as isize);
    (*pTemp).entity.baseline.origin[2 as libc::c_int as usize] =
        *velocity.offset(2 as libc::c_int as isize);
    if (*pmodel).type_0 as libc::c_int == mod_sprite as libc::c_int {
        (*pTemp).flags = (*pTemp).flags | 0x100 as libc::c_int;
        if (*pTemp).frameMax < 10 as libc::c_int as libc::c_float {
            (*pTemp).flags =
                (*pTemp).flags |
                    (0x100 as libc::c_int | 0x10000 as libc::c_int);
            (*pTemp).entity.curstate.framerate =
                10 as libc::c_int as libc::c_float
        } else {
            (*pTemp).entity.curstate.framerate =
                (*pTemp).frameMax / life as libc::c_float
        }
    } else {
        (*pTemp).frameMax = 0 as libc::c_int as libc::c_float;
        let mut ilength: libc::c_float =
            __tg_sqrt(*velocity.offset(0 as libc::c_int as isize) *
                          *velocity.offset(0 as libc::c_int as isize) +
                          *velocity.offset(1 as libc::c_int as isize) *
                              *velocity.offset(1 as libc::c_int as isize) +
                          *velocity.offset(2 as libc::c_int as isize) *
                              *velocity.offset(2 as libc::c_int as isize));
        if ilength != 0. { ilength = 1.0f32 / ilength }
        dir[0 as libc::c_int as usize] =
            *velocity.offset(0 as libc::c_int as isize) * ilength;
        dir[1 as libc::c_int as usize] =
            *velocity.offset(1 as libc::c_int as isize) * ilength;
        dir[2 as libc::c_int as usize] =
            *velocity.offset(2 as libc::c_int as isize) * ilength;
        VectorAngles(dir.as_mut_ptr(), (*pTemp).entity.angles.as_mut_ptr());
    }
    (*pTemp).flags |=
        0x1000 as libc::c_int | 0x2000 as libc::c_int | 0x4000 as libc::c_int;
    (*pTemp).clientIndex =
        if owner >= 1 as libc::c_int {
            if owner < cl.maxclients { owner } else { cl.maxclients }
        } else { 1 as libc::c_int } as libc::c_short;
    (*pTemp).entity.baseline.renderamt = 255 as libc::c_int;
    (*pTemp).hitcallback = hitcallback;
    (*pTemp).die = (cl.time + life as libc::c_double) as libc::c_float;
}
/*
==============
R_TempSphereModel

Spherical shower of models, picks from set
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_TempSphereModel(mut pos: *const vec_t,
                                           mut speed: libc::c_float,
                                           mut life: libc::c_float,
                                           mut count: libc::c_int,
                                           mut modelIndex: libc::c_int) {
    let mut pTemp: *mut TEMPENTITY = 0 as *mut TEMPENTITY;
    let mut i: libc::c_int = 0;
    // create temp models
    i = 0 as libc::c_int;
    while i < count {
        pTemp = CL_TempEntAlloc(pos, CL_ModelHandle(modelIndex));
        if pTemp.is_null() { return }
        (*pTemp).entity.curstate.body =
            COM_RandomLong(0 as libc::c_int,
                           (*pTemp).frameMax as libc::c_int);
        if COM_RandomLong(0 as libc::c_int, 255 as libc::c_int) <
               10 as libc::c_int {
            (*pTemp).flags |= 0x8 as libc::c_int
        } else { (*pTemp).flags |= 0x2 as libc::c_int }
        if COM_RandomLong(0 as libc::c_int, 255 as libc::c_int) <
               200 as libc::c_int {
            (*pTemp).flags |= 0x4 as libc::c_int;
            (*pTemp).entity.baseline.angles[0 as libc::c_int as usize] =
                COM_RandomFloat(-256.0f32, -255.0f32);
            (*pTemp).entity.baseline.angles[1 as libc::c_int as usize] =
                COM_RandomFloat(-256.0f32, -255.0f32);
            (*pTemp).entity.baseline.angles[2 as libc::c_int as usize] =
                COM_RandomFloat(-256.0f32, -255.0f32)
        }
        if COM_RandomLong(0 as libc::c_int, 255 as libc::c_int) <
               100 as libc::c_int {
            (*pTemp).flags |= 0x10 as libc::c_int
        }
        (*pTemp).flags |= 0x40 as libc::c_int | 0x20 as libc::c_int;
        (*pTemp).entity.curstate.rendermode = kRenderNormal as libc::c_int;
        (*pTemp).entity.curstate.effects = i & 31 as libc::c_int;
        (*pTemp).entity.baseline.origin[0 as libc::c_int as usize] =
            COM_RandomFloat(-1.0f32, 1.0f32);
        (*pTemp).entity.baseline.origin[1 as libc::c_int as usize] =
            COM_RandomFloat(-1.0f32, 1.0f32);
        (*pTemp).entity.baseline.origin[2 as libc::c_int as usize] =
            COM_RandomFloat(-1.0f32, 1.0f32);
        let mut ilength: libc::c_float =
            __tg_sqrt((*pTemp).entity.baseline.origin[0 as libc::c_int as
                                                          usize] *
                          (*pTemp).entity.baseline.origin[0 as libc::c_int as
                                                              usize] +
                          (*pTemp).entity.baseline.origin[1 as libc::c_int as
                                                              usize] *
                              (*pTemp).entity.baseline.origin[1 as libc::c_int
                                                                  as usize] +
                          (*pTemp).entity.baseline.origin[2 as libc::c_int as
                                                              usize] *
                              (*pTemp).entity.baseline.origin[2 as libc::c_int
                                                                  as usize]);
        if ilength != 0. { ilength = 1.0f32 / ilength }
        (*pTemp).entity.baseline.origin[0 as libc::c_int as usize] *= ilength;
        (*pTemp).entity.baseline.origin[1 as libc::c_int as usize] *= ilength;
        (*pTemp).entity.baseline.origin[2 as libc::c_int as usize] *= ilength;
        (*pTemp).entity.baseline.origin[0 as libc::c_int as usize] =
            (*pTemp).entity.baseline.origin[0 as libc::c_int as usize] *
                speed;
        (*pTemp).entity.baseline.origin[1 as libc::c_int as usize] =
            (*pTemp).entity.baseline.origin[1 as libc::c_int as usize] *
                speed;
        (*pTemp).entity.baseline.origin[2 as libc::c_int as usize] =
            (*pTemp).entity.baseline.origin[2 as libc::c_int as usize] *
                speed;
        (*pTemp).die = (cl.time + life as libc::c_double) as libc::c_float;
        i += 1
    };
}
/*
==============
R_Explosion

Create an explosion (scale is magnitude)
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_Explosion(mut pos: *mut vec_t,
                                     mut model: libc::c_int,
                                     mut scale: libc::c_float,
                                     mut framerate: libc::c_float,
                                     mut flags: libc::c_int) {
    let mut hSound: sound_t = 0;
    if scale != 0.0f32 {
        // create explosion sprite
        R_Sprite_Explode(R_DefaultSprite(pos as *const vec_t, model,
                                         framerate), scale, flags);
        if flags & 8 as libc::c_int == 0 {
            R_FlickerParticles(pos as *const vec_t);
        }
        if flags & 2 as libc::c_int == 0 {
            let mut dl: *mut dlight_t = 0 as *mut dlight_t;
            // big flash
            dl = CL_AllocDlight(0 as libc::c_int);
            (*dl).origin[0 as libc::c_int as usize] =
                *pos.offset(0 as libc::c_int as isize);
            (*dl).origin[1 as libc::c_int as usize] =
                *pos.offset(1 as libc::c_int as isize);
            (*dl).origin[2 as libc::c_int as usize] =
                *pos.offset(2 as libc::c_int as isize);
            (*dl).radius = 200 as libc::c_int as libc::c_float;
            (*dl).color.r = 250 as libc::c_int as byte;
            (*dl).color.g = 250 as libc::c_int as byte;
            (*dl).color.b = 150 as libc::c_int as byte;
            (*dl).die =
                (cl.time + 0.01f32 as libc::c_double) as libc::c_float;
            (*dl).decay = 80 as libc::c_int as libc::c_float;
            // red glow
            dl = CL_AllocDlight(0 as libc::c_int);
            (*dl).origin[0 as libc::c_int as usize] =
                *pos.offset(0 as libc::c_int as isize);
            (*dl).origin[1 as libc::c_int as usize] =
                *pos.offset(1 as libc::c_int as isize);
            (*dl).origin[2 as libc::c_int as usize] =
                *pos.offset(2 as libc::c_int as isize);
            (*dl).radius = 150 as libc::c_int as libc::c_float;
            (*dl).color.r = 255 as libc::c_int as byte;
            (*dl).color.g = 190 as libc::c_int as byte;
            (*dl).color.b = 40 as libc::c_int as byte;
            (*dl).die = (cl.time + 1.0f32 as libc::c_double) as libc::c_float;
            (*dl).decay = 200 as libc::c_int as libc::c_float
        }
    }
    if flags & 4 as libc::c_int == 0 {
        hSound =
            S_RegisterSound(cl_explode_sounds[COM_RandomLong(0 as libc::c_int,
                                                             2 as libc::c_int)
                                                  as usize]);
        S_StartSound(pos as *const vec_t, 0 as libc::c_int, 6 as libc::c_int,
                     hSound, 1.0f64 as libc::c_float, 0.3f32,
                     100 as libc::c_int, 0 as libc::c_int);
    };
}
/*
==============
R_PlayerSprites

Create a particle smoke around player
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_PlayerSprites(mut client: libc::c_int,
                                         mut modelIndex: libc::c_int,
                                         mut count: libc::c_int,
                                         mut size: libc::c_int) {
    let mut pTemp: *mut TEMPENTITY = 0 as *mut TEMPENTITY;
    let mut pEnt: *mut cl_entity_t = 0 as *mut cl_entity_t;
    let mut position: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
    let mut vel: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    pEnt = CL_GetEntityByIndex(client);
    if pEnt.is_null() || (*pEnt).player as u64 == 0 { return }
    vel = 128 as libc::c_int as libc::c_float;
    i = 0 as libc::c_int;
    while i < count {
        position[0 as libc::c_int as usize] =
            (*pEnt).origin[0 as libc::c_int as usize];
        position[1 as libc::c_int as usize] =
            (*pEnt).origin[1 as libc::c_int as usize];
        position[2 as libc::c_int as usize] =
            (*pEnt).origin[2 as libc::c_int as usize];
        position[0 as libc::c_int as usize] +=
            COM_RandomFloat(-10.0f32, 10.0f32);
        position[1 as libc::c_int as usize] +=
            COM_RandomFloat(-10.0f32, 10.0f32);
        position[2 as libc::c_int as usize] +=
            COM_RandomFloat(-20.0f32, 36.0f32);
        pTemp =
            CL_TempEntAlloc(position.as_mut_ptr() as *const vec_t,
                            CL_ModelHandle(modelIndex));
        if pTemp.is_null() { return }
        (*pTemp).tentOffset[0 as libc::c_int as usize] =
            (*pTemp).entity.origin[0 as libc::c_int as usize] -
                (*pEnt).origin[0 as libc::c_int as usize];
        (*pTemp).tentOffset[1 as libc::c_int as usize] =
            (*pTemp).entity.origin[1 as libc::c_int as usize] -
                (*pEnt).origin[1 as libc::c_int as usize];
        (*pTemp).tentOffset[2 as libc::c_int as usize] =
            (*pTemp).entity.origin[2 as libc::c_int as usize] -
                (*pEnt).origin[2 as libc::c_int as usize];
        if i != 0 as libc::c_int {
            (*pTemp).flags |= 0x8000 as libc::c_int;
            (*pTemp).clientIndex = client as libc::c_short
        } else {
            dir[0 as libc::c_int as usize] =
                position[0 as libc::c_int as usize] -
                    (*pEnt).origin[0 as libc::c_int as usize];
            dir[1 as libc::c_int as usize] =
                position[1 as libc::c_int as usize] -
                    (*pEnt).origin[1 as libc::c_int as usize];
            dir[2 as libc::c_int as usize] =
                position[2 as libc::c_int as usize] -
                    (*pEnt).origin[2 as libc::c_int as usize];
            let mut ilength: libc::c_float =
                __tg_sqrt(dir[0 as libc::c_int as usize] *
                              dir[0 as libc::c_int as usize] +
                              dir[1 as libc::c_int as usize] *
                                  dir[1 as libc::c_int as usize] +
                              dir[2 as libc::c_int as usize] *
                                  dir[2 as libc::c_int as usize]);
            if ilength != 0. { ilength = 1.0f32 / ilength }
            dir[0 as libc::c_int as usize] *= ilength;
            dir[1 as libc::c_int as usize] *= ilength;
            dir[2 as libc::c_int as usize] *= ilength;
            dir[0 as libc::c_int as usize] =
                dir[0 as libc::c_int as usize] *
                    60 as libc::c_int as libc::c_float;
            dir[1 as libc::c_int as usize] =
                dir[1 as libc::c_int as usize] *
                    60 as libc::c_int as libc::c_float;
            dir[2 as libc::c_int as usize] =
                dir[2 as libc::c_int as usize] *
                    60 as libc::c_int as libc::c_float;
            (*pTemp).entity.baseline.origin[0 as libc::c_int as usize] =
                dir[0 as libc::c_int as usize];
            (*pTemp).entity.baseline.origin[1 as libc::c_int as usize] =
                dir[1 as libc::c_int as usize];
            (*pTemp).entity.baseline.origin[2 as libc::c_int as usize] =
                dir[2 as libc::c_int as usize];
            (*pTemp).entity.baseline.origin[1 as libc::c_int as usize] =
                COM_RandomFloat(20.0f32, 60.0f32)
        }
        (*pTemp).entity.curstate.renderfx =
            kRenderFxNoDissipation as libc::c_int;
        (*pTemp).entity.curstate.framerate =
            COM_RandomFloat(1.0f32 - size as libc::c_float / 100.0f32,
                            1.0f32);
        if (*pTemp).frameMax > 1 as libc::c_int as libc::c_float {
            (*pTemp).flags |= 0x100 as libc::c_int;
            (*pTemp).entity.curstate.framerate = 20.0f32;
            (*pTemp).die =
                (cl.time + ((*pTemp).frameMax * 0.05f32) as libc::c_double) as
                    libc::c_float
        } else {
            (*pTemp).die =
                (cl.time + 0.35f32 as libc::c_double) as libc::c_float
        }
        i += 1
    };
}
/*
==============
R_FireField

Makes a field of fire
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_FireField(mut org: *mut libc::c_float,
                                     mut radius: libc::c_int,
                                     mut modelIndex: libc::c_int,
                                     mut count: libc::c_int,
                                     mut flags: libc::c_int,
                                     mut life: libc::c_float) {
    let mut pTemp: *mut TEMPENTITY = 0 as *mut TEMPENTITY;
    let mut pmodel: *mut model_t = 0 as *mut model_t;
    let mut time: libc::c_float = 0.;
    let mut pos: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    pmodel = CL_ModelHandle(modelIndex);
    if pmodel.is_null() { return }
    i = 0 as libc::c_int;
    while i < count {
        pos[0 as libc::c_int as usize] =
            *org.offset(0 as libc::c_int as isize);
        pos[1 as libc::c_int as usize] =
            *org.offset(1 as libc::c_int as isize);
        pos[2 as libc::c_int as usize] =
            *org.offset(2 as libc::c_int as isize);
        pos[0 as libc::c_int as usize] +=
            COM_RandomFloat(-radius as libc::c_float,
                            radius as libc::c_float);
        pos[1 as libc::c_int as usize] +=
            COM_RandomFloat(-radius as libc::c_float,
                            radius as libc::c_float);
        if flags & 16 as libc::c_int == 0 {
            pos[2 as libc::c_int as usize] +=
                COM_RandomFloat(-radius as libc::c_float,
                                radius as libc::c_float)
        }
        pTemp = CL_TempEntAlloc(pos.as_mut_ptr() as *const vec_t, pmodel);
        if pTemp.is_null() { return }
        if flags & 8 as libc::c_int != 0 {
            (*pTemp).entity.curstate.rendermode =
                kRenderTransAlpha as libc::c_int;
            (*pTemp).entity.curstate.renderfx =
                kRenderFxNoDissipation as libc::c_int;
            (*pTemp).entity.curstate.renderamt = 128 as libc::c_int;
            (*pTemp).entity.baseline.renderamt =
                (*pTemp).entity.curstate.renderamt
        } else if flags & 32 as libc::c_int != 0 {
            (*pTemp).entity.curstate.rendermode =
                kRenderTransAdd as libc::c_int;
            (*pTemp).entity.curstate.renderamt = 80 as libc::c_int
        } else {
            (*pTemp).entity.curstate.rendermode =
                kRenderNormal as libc::c_int;
            (*pTemp).entity.curstate.renderfx =
                kRenderFxNoDissipation as libc::c_int;
            (*pTemp).entity.curstate.renderamt = 255 as libc::c_int;
            (*pTemp).entity.baseline.renderamt =
                (*pTemp).entity.curstate.renderamt
        }
        (*pTemp).entity.curstate.framerate =
            COM_RandomFloat(0.75f32, 1.25f32);
        time = life + COM_RandomFloat(-0.25f32, 0.5f32);
        (*pTemp).die = (cl.time + time as libc::c_double) as libc::c_float;
        if (*pTemp).frameMax > 1 as libc::c_int as libc::c_float {
            (*pTemp).flags |= 0x100 as libc::c_int;
            if flags & 4 as libc::c_int != 0 {
                (*pTemp).entity.curstate.framerate = 15.0f32;
                (*pTemp).flags |= 0x10000 as libc::c_int
            } else {
                (*pTemp).entity.curstate.framerate = (*pTemp).frameMax / time
            }
        }
        if flags & 1 as libc::c_int != 0 ||
               flags & 2 as libc::c_int != 0 &&
                   COM_RandomLong(0 as libc::c_int, 1 as libc::c_int) == 0 {
            // drift sprite upward
            (*pTemp).entity.baseline.origin[2 as libc::c_int as usize] =
                COM_RandomFloat(10.0f32, 30.0f32)
        }
        i += 1
    };
}
/*
==============
R_MultiGunshot

Client version of shotgun shot
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_MultiGunshot(mut org: *const vec_t,
                                        mut dir: *const vec_t,
                                        mut noise: *const vec_t,
                                        mut count: libc::c_int,
                                        mut decalCount: libc::c_int,
                                        mut decalIndices: *mut libc::c_int) {
    let mut trace: pmtrace_t =
        pmtrace_t{allsolid: false_0,
                  startsolid: false_0,
                  inopen: false_0,
                  inwater: false_0,
                  fraction: 0.,
                  endpos: [0.; 3],
                  plane: pmplane_t{normal: [0.; 3], dist: 0.,},
                  ent: 0,
                  deltavelocity: [0.; 3],
                  hitgroup: 0,};
    let mut right: vec3_t = [0.; 3];
    let mut up: vec3_t = [0.; 3];
    let mut vecSrc: vec3_t = [0.; 3];
    let mut vecDir: vec3_t = [0.; 3];
    let mut vecEnd: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut decalIndex: libc::c_int = 0;
    VectorVectors(dir, right.as_mut_ptr(), up.as_mut_ptr());
    vecSrc[0 as libc::c_int as usize] =
        *org.offset(0 as libc::c_int as isize);
    vecSrc[1 as libc::c_int as usize] =
        *org.offset(1 as libc::c_int as isize);
    vecSrc[2 as libc::c_int as usize] =
        *org.offset(2 as libc::c_int as isize);
    i = 0 as libc::c_int;
    while i < count {
        // get circular gaussian spread
        let mut x: libc::c_float = 0.;
        let mut y: libc::c_float = 0.;
        let mut z: libc::c_float = 0.;
        loop  {
            x =
                COM_RandomFloat(-0.5f32, 0.5f32) +
                    COM_RandomFloat(-0.5f32, 0.5f32);
            y =
                COM_RandomFloat(-0.5f32, 0.5f32) +
                    COM_RandomFloat(-0.5f32, 0.5f32);
            z = x * x + y * y;
            if !(z > 1.0f32) { break ; }
        }
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            vecDir[j as usize] =
                *dir.offset(j as isize) +
                    x * *noise.offset(0 as libc::c_int as isize) *
                        right[j as usize] +
                    y * *noise.offset(1 as libc::c_int as isize) *
                        up[j as usize];
            vecEnd[j as usize] =
                vecSrc[j as usize] + 4096.0f32 * vecDir[j as usize];
            j += 1
        }
        trace =
            CL_TraceLine(vecSrc.as_mut_ptr(), vecEnd.as_mut_ptr(),
                         0x1 as libc::c_int);
        // paint decals
        if trace.fraction != 1.0f32 {
            let mut pe: *mut physent_t = 0 as *mut physent_t;
            if i & 2 as libc::c_int != 0 {
                R_RicochetSound(trace.endpos.as_mut_ptr() as *const vec_t);
            }
            R_BulletImpactParticles(trace.endpos.as_mut_ptr() as
                                        *const vec_t);
            if trace.ent >= 0 as libc::c_int &&
                   trace.ent < (*clgame.pmove).numphysent {
                pe =
                    &mut *(*clgame.pmove).physents.as_mut_ptr().offset(trace.ent
                                                                           as
                                                                           isize)
                        as *mut physent_t
            }
            if !pe.is_null() &&
                   ((*pe).solid == 4 as libc::c_int ||
                        (*pe).movetype == 13 as libc::c_int) {
                let mut e: *mut cl_entity_t = CL_GetEntityByIndex((*pe).info);
                decalIndex =
                    CL_DecalIndex(*decalIndices.offset(COM_RandomLong(0 as
                                                                          libc::c_int,
                                                                      decalCount
                                                                          -
                                                                          1 as
                                                                              libc::c_int)
                                                           as isize));
                CL_DecalShoot(decalIndex, (*e).index, 0 as libc::c_int,
                              trace.endpos.as_mut_ptr(), 0 as libc::c_int);
            }
        }
        i += 1
    };
}
/*
==============
R_Sprite_WallPuff

Create a wallpuff
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_Sprite_WallPuff(mut pTemp: *mut TEMPENTITY,
                                           mut scale: libc::c_float) {
    if pTemp.is_null() { return }
    (*pTemp).entity.curstate.renderamt = 255 as libc::c_int;
    (*pTemp).entity.curstate.rendermode = kRenderTransAlpha as libc::c_int;
    (*pTemp).entity.angles[2 as libc::c_int as usize] =
        COM_RandomLong(0 as libc::c_int, 359 as libc::c_int) as vec_t;
    (*pTemp).entity.baseline.origin[2 as libc::c_int as usize] =
        30 as libc::c_int as vec_t;
    (*pTemp).entity.curstate.scale = scale;
    (*pTemp).die = (cl.time + 0.01f32 as libc::c_double) as libc::c_float;
}
/*
==============
CL_ParseTempEntity

handle temp-entity messages
==============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseTempEntity(mut msg: *mut sizebuf_t) {
    let mut buf: sizebuf_t =
        sizebuf_t{bOverflow: false_0,
                  pDebugName: 0 as *const libc::c_char,
                  pData: 0 as *const byte as *mut byte,
                  iCurBit: 0,
                  nDataBits: 0,}; // just stub
    let mut pbuf: [byte; 256] = [0; 256];
    let mut iSize: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut color: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut decalIndex: libc::c_int = 0;
    let mut modelIndex: libc::c_int = 0;
    let mut entityIndex: libc::c_int = 0;
    let mut scale: libc::c_float = 0.;
    let mut life: libc::c_float = 0.;
    let mut frameRate: libc::c_float = 0.;
    let mut vel: libc::c_float = 0.;
    let mut random: libc::c_float = 0.;
    let mut brightness: libc::c_float = 0.;
    let mut r: libc::c_float = 0.;
    let mut g: libc::c_float = 0.;
    let mut b: libc::c_float = 0.;
    let mut pos: vec3_t = [0.; 3];
    let mut pos2: vec3_t = [0.; 3];
    let mut ang: vec3_t = [0.; 3];
    let mut decalIndices: [libc::c_int; 1] = [0; 1];
    let mut pTemp: *mut TEMPENTITY = 0 as *mut TEMPENTITY;
    let mut pEnt: *mut cl_entity_t = 0 as *mut cl_entity_t;
    let mut dl: *mut dlight_t = 0 as *mut dlight_t;
    if cls.legacymode as u64 != 0 {
        iSize = MSG_ReadByte(msg)
    } else { iSize = MSG_ReadWord(msg) }
    entityIndex = 0 as libc::c_int;
    modelIndex = entityIndex;
    decalIndex = modelIndex;
    // parse user message into buffer
    MSG_ReadBytes(msg, pbuf.as_mut_ptr() as *mut libc::c_void, iSize);
    // init a safe tempbuffer
    MSG_InitExt(&mut buf,
                b"TempEntity\x00" as *const u8 as *const libc::c_char,
                pbuf.as_mut_ptr() as *mut libc::c_void, iSize,
                -(1 as libc::c_int)); // same as density
    type_0 = MSG_ReadByte(&mut buf); // yaw angle
    match type_0 {
        0 | 1 | 7 | 8 | 16 | 18 | 19 | 20 | 21 | 22 | 24 | 26 | 99 => {
            CL_ParseViewBeam(&mut buf, type_0); // sound flags
        }
        2 => {
            pos[0 as libc::c_int as usize] =
                MSG_ReadCoord(&mut buf); // rendermode
            pos[1 as libc::c_int as usize] =
                MSG_ReadCoord(&mut buf); // playernum
            pos[2 as libc::c_int as usize] =
                MSG_ReadCoord(&mut buf); // water height
            R_RicochetSound(pos.as_mut_ptr() as *const vec_t); // sprite #1
            R_RunParticleEffect(pos.as_mut_ptr() as *const vec_t,
                                vec3_origin.as_mut_ptr() as *const vec_t,
                                0 as libc::c_int,
                                20 as libc::c_int); // sprite #2
        }
        3 => {
            pos[0 as libc::c_int as usize] =
                MSG_ReadCoord(&mut buf); // playernum
            pos[1 as libc::c_int as usize] =
                MSG_ReadCoord(&mut buf); // entitynum
            pos[2 as libc::c_int as usize] =
                MSG_ReadCoord(&mut buf); // playernum
            modelIndex = MSG_ReadShort(&mut buf); // height
            scale =
                MSG_ReadByte(&mut buf) as libc::c_float * 0.1f32; // playernum
            frameRate = MSG_ReadByte(&mut buf) as libc::c_float;
            flags = MSG_ReadByte(&mut buf);
            R_Explosion(pos.as_mut_ptr(), modelIndex, scale, frameRate,
                        flags);
        }
        4 => {
            pos[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            R_BlobExplosion(pos.as_mut_ptr() as *const vec_t);
        }
        5 => {
            pos[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            modelIndex = MSG_ReadShort(&mut buf);
            scale = MSG_ReadByte(&mut buf) as libc::c_float * 0.1f32;
            frameRate = MSG_ReadByte(&mut buf) as libc::c_float;
            pTemp =
                R_DefaultSprite(pos.as_mut_ptr() as *const vec_t, modelIndex,
                                frameRate);
            R_Sprite_Smoke(pTemp, scale);
        }
        6 => {
            pos[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos2[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos2[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos2[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            R_TracerEffect(pos.as_mut_ptr() as *const vec_t,
                           pos2.as_mut_ptr() as *const vec_t);
        }
        9 => {
            pos[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            R_SparkShower(pos.as_mut_ptr() as *const vec_t);
        }
        10 => {
            pos[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            R_LavaSplash(pos.as_mut_ptr() as *const vec_t);
        }
        11 => {
            pos[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            R_TeleportSplash(pos.as_mut_ptr() as *const vec_t);
        }
        12 => {
            pos[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            color = MSG_ReadByte(&mut buf);
            count = MSG_ReadByte(&mut buf);
            R_ParticleExplosion2(pos.as_mut_ptr() as *const vec_t, color,
                                 count);
        }
        13 => {
            pos[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            decalIndex = MSG_ReadShort(&mut buf);
            entityIndex = MSG_ReadShort(&mut buf);
            if entityIndex != 0 { modelIndex = MSG_ReadShort(&mut buf) }
            CL_DecalShoot(CL_DecalIndex(decalIndex), entityIndex, modelIndex,
                          pos.as_mut_ptr(), 0x1 as libc::c_int);
        }
        14 => {
            pos[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            scale = MSG_ReadByte(&mut buf) as libc::c_float;
            count = MSG_ReadByte(&mut buf);
            life = MSG_ReadByte(&mut buf) as libc::c_float * 0.1f32;
            R_Implosion(pos.as_mut_ptr() as *const vec_t, scale, count, life);
        }
        15 => {
            pos[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos2[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos2[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos2[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            modelIndex = MSG_ReadShort(&mut buf);
            count = MSG_ReadByte(&mut buf);
            life = MSG_ReadByte(&mut buf) as libc::c_float * 0.1f32;
            scale = MSG_ReadByte(&mut buf) as libc::c_float * 0.1f32;
            vel = MSG_ReadByte(&mut buf) as libc::c_float;
            random = MSG_ReadByte(&mut buf) as libc::c_float;
            R_Sprite_Trail(type_0, pos.as_mut_ptr(), pos2.as_mut_ptr(),
                           modelIndex, count, life, scale, random,
                           255 as libc::c_int, vel);
        }
        17 => {
            pos[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            modelIndex = MSG_ReadShort(&mut buf);
            scale = MSG_ReadByte(&mut buf) as libc::c_float * 0.1f32;
            brightness = MSG_ReadByte(&mut buf) as libc::c_float;
            pTemp =
                R_DefaultSprite(pos.as_mut_ptr() as *const vec_t, modelIndex,
                                0 as libc::c_int as libc::c_float);
            if !pTemp.is_null() {
                (*pTemp).entity.curstate.scale = scale;
                (*pTemp).entity.baseline.renderamt =
                    brightness as libc::c_int;
                (*pTemp).entity.curstate.renderamt =
                    brightness as libc::c_int;
                (*pTemp).entity.curstate.rendermode =
                    kRenderTransAdd as libc::c_int
            }
        }
        23 => {
            pos[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            modelIndex = MSG_ReadShort(&mut buf);
            life = MSG_ReadByte(&mut buf) as libc::c_float * 0.1f32;
            scale = MSG_ReadByte(&mut buf) as libc::c_float * 0.1f32;
            brightness = MSG_ReadByte(&mut buf) as libc::c_float;
            pTemp =
                R_DefaultSprite(pos.as_mut_ptr() as *const vec_t, modelIndex,
                                0 as libc::c_int as libc::c_float);
            if !pTemp.is_null() {
                (*pTemp).entity.curstate.scale = scale;
                (*pTemp).entity.curstate.rendermode =
                    kRenderGlow as libc::c_int;
                (*pTemp).entity.curstate.renderfx =
                    kRenderFxNoDissipation as libc::c_int;
                (*pTemp).entity.baseline.renderamt =
                    brightness as libc::c_int;
                (*pTemp).entity.curstate.renderamt =
                    brightness as libc::c_int;
                (*pTemp).flags = 0x80 as libc::c_int;
                (*pTemp).die =
                    (cl.time + life as libc::c_double) as libc::c_float
            }
        }
        25 => {
            pos[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos2[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos2[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos2[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            color = MSG_ReadByte(&mut buf);
            count = MSG_ReadShort(&mut buf);
            vel = MSG_ReadShort(&mut buf) as libc::c_float;
            random = MSG_ReadShort(&mut buf) as libc::c_float;
            R_StreakSplash(pos.as_mut_ptr() as *const vec_t,
                           pos2.as_mut_ptr() as *const vec_t, color, count,
                           vel, -random as libc::c_int,
                           random as libc::c_int);
        }
        27 => {
            dl = CL_AllocDlight(0 as libc::c_int);
            (*dl).origin[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            (*dl).origin[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            (*dl).origin[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            (*dl).radius = MSG_ReadByte(&mut buf) as libc::c_float * 10.0f32;
            (*dl).color.r = MSG_ReadByte(&mut buf) as byte;
            (*dl).color.g = MSG_ReadByte(&mut buf) as byte;
            (*dl).color.b = MSG_ReadByte(&mut buf) as byte;
            (*dl).die =
                (cl.time +
                     (MSG_ReadByte(&mut buf) as libc::c_float * 0.1f32) as
                         libc::c_double) as libc::c_float;
            (*dl).decay = MSG_ReadByte(&mut buf) as libc::c_float * 10.0f32
        }
        28 => {
            dl = CL_AllocElight(MSG_ReadShort(&mut buf));
            (*dl).origin[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            (*dl).origin[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            (*dl).origin[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            (*dl).radius = MSG_ReadCoord(&mut buf);
            (*dl).color.r = MSG_ReadByte(&mut buf) as byte;
            (*dl).color.g = MSG_ReadByte(&mut buf) as byte;
            (*dl).color.b = MSG_ReadByte(&mut buf) as byte;
            life = MSG_ReadByte(&mut buf) as libc::c_float * 0.1f32;
            (*dl).die = (cl.time + life as libc::c_double) as libc::c_float;
            (*dl).decay = MSG_ReadCoord(&mut buf);
            if life != 0 as libc::c_int as libc::c_float {
                (*dl).decay /= life
            }
        }
        29 => { CL_ParseTextMessage(&mut buf); }
        30 | 31 => {
            pos[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos2[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos2[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos2[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            life = MSG_ReadShort(&mut buf) as libc::c_float * 0.1f32;
            r = MSG_ReadByte(&mut buf) as libc::c_float;
            g = MSG_ReadByte(&mut buf) as libc::c_float;
            b = MSG_ReadByte(&mut buf) as libc::c_float;
            if type_0 == 30 as libc::c_int {
                R_ParticleLine(pos.as_mut_ptr() as *const vec_t,
                               pos2.as_mut_ptr() as *const vec_t, r as byte,
                               g as byte, b as byte, life);
            } else {
                R_ParticleBox(pos.as_mut_ptr() as *const vec_t,
                              pos2.as_mut_ptr() as *const vec_t, r as byte,
                              g as byte, b as byte, life);
            }
        }
        100 => {
            pos[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            modelIndex = MSG_ReadShort(&mut buf);
            flags = MSG_ReadShort(&mut buf);
            R_LargeFunnel(pos.as_mut_ptr() as *const vec_t, flags);
            R_FunnelSprite(pos.as_mut_ptr() as *const vec_t, modelIndex,
                           flags);
        }
        101 | 103 => {
            pos[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos2[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos2[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos2[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            color = MSG_ReadByte(&mut buf);
            count = MSG_ReadByte(&mut buf);
            if type_0 == 103 as libc::c_int {
                R_Blood(pos.as_mut_ptr() as *const vec_t,
                        pos2.as_mut_ptr() as *const vec_t, color, count);
            } else {
                R_BloodStream(pos.as_mut_ptr() as *const vec_t,
                              pos2.as_mut_ptr() as *const vec_t, color,
                              count);
            }
        }
        102 => {
            pos[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos2[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos2[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos2[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            R_ShowLine(pos.as_mut_ptr() as *const vec_t,
                       pos2.as_mut_ptr() as *const vec_t);
        }
        104 | 118 | 116 | 117 => {
            pos[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            decalIndex = MSG_ReadByte(&mut buf);
            if type_0 == 104 as libc::c_int || type_0 == 118 as libc::c_int {
                entityIndex = MSG_ReadShort(&mut buf)
            } else { entityIndex = 0 as libc::c_int }
            if type_0 == 118 as libc::c_int || type_0 == 117 as libc::c_int {
                decalIndex += 256 as libc::c_int
            }
            pEnt = CL_GetEntityByIndex(entityIndex);
            if !pEnt.is_null() { modelIndex = (*pEnt).curstate.modelindex }
            CL_DecalShoot(CL_DecalIndex(decalIndex), entityIndex, modelIndex,
                          pos.as_mut_ptr(), 0 as libc::c_int);
        }
        105 => {
            entityIndex = MSG_ReadShort(&mut buf);
            modelIndex = MSG_ReadShort(&mut buf);
            scale = MSG_ReadByte(&mut buf) as libc::c_float;
            pEnt = CL_GetEntityByIndex(entityIndex);
            R_FizzEffect(pEnt, modelIndex, scale as libc::c_int);
        }
        106 => {
            pos[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos2[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos2[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos2[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            ang[0 as libc::c_int as usize] = 0.0f32;
            ang[1 as libc::c_int as usize] =
                MSG_ReadChar(&mut buf) as libc::c_float *
                    (360.0f32 / 256.0f32);
            ang[2 as libc::c_int as usize] = 0.0f32;
            modelIndex = MSG_ReadShort(&mut buf);
            flags = MSG_ReadByte(&mut buf);
            life = MSG_ReadByte(&mut buf) as libc::c_float * 0.1f32;
            R_TempModel(pos.as_mut_ptr() as *const vec_t,
                        pos2.as_mut_ptr() as *const vec_t,
                        ang.as_mut_ptr() as *const vec_t, life, modelIndex,
                        flags);
        }
        107 => {
            pos[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            vel = MSG_ReadCoord(&mut buf);
            modelIndex = MSG_ReadShort(&mut buf);
            count = MSG_ReadShort(&mut buf);
            life = MSG_ReadByte(&mut buf) as libc::c_float * 0.1f32;
            R_TempSphereModel(pos.as_mut_ptr() as *const vec_t, vel, life,
                              count, modelIndex);
        }
        108 => {
            pos[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos2[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos2[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos2[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            ang[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            ang[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            ang[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            random = MSG_ReadByte(&mut buf) as libc::c_float * 10.0f32;
            modelIndex = MSG_ReadShort(&mut buf);
            count = MSG_ReadByte(&mut buf);
            life = MSG_ReadByte(&mut buf) as libc::c_float * 0.1f32;
            flags = MSG_ReadByte(&mut buf);
            R_BreakModel(pos.as_mut_ptr() as *const vec_t,
                         pos2.as_mut_ptr() as *const vec_t,
                         ang.as_mut_ptr() as *const vec_t, random, life,
                         count, modelIndex, flags as libc::c_char);
        }
        109 => {
            pos[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            entityIndex = MSG_ReadShort(&mut buf);
            decalIndex = MSG_ReadByte(&mut buf);
            pEnt = CL_GetEntityByIndex(entityIndex);
            CL_DecalShoot(CL_DecalIndex(decalIndex), entityIndex,
                          0 as libc::c_int, pos.as_mut_ptr(),
                          0 as libc::c_int);
            R_BulletImpactParticles(pos.as_mut_ptr() as *const vec_t);
            R_RicochetSound(pos.as_mut_ptr() as *const vec_t);
        }
        120 | 110 => {
            pos[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos2[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos2[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos2[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            modelIndex = MSG_ReadShort(&mut buf);
            count = MSG_ReadByte(&mut buf);
            vel = MSG_ReadByte(&mut buf) as libc::c_float;
            random = MSG_ReadByte(&mut buf) as libc::c_float;
            if type_0 == 120 as libc::c_int {
                flags = MSG_ReadByte(&mut buf);
                R_Spray(pos.as_mut_ptr() as *const vec_t,
                        pos2.as_mut_ptr() as *const vec_t, modelIndex, count,
                        vel as libc::c_int, random as libc::c_int, flags);
            } else {
                R_Sprite_Spray(pos.as_mut_ptr() as *const vec_t,
                               pos2.as_mut_ptr() as *const vec_t, modelIndex,
                               count, (vel * 2.0f32) as libc::c_int,
                               random as libc::c_int);
            }
        }
        111 => {
            pos[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            scale = MSG_ReadByte(&mut buf) as libc::c_float * 0.1f32;
            R_RicochetSprite(pos.as_mut_ptr() as *const vec_t,
                             cl_sprite_ricochet, 0.1f32, scale);
            R_RicochetSound(pos.as_mut_ptr() as *const vec_t);
        }
        112 => {
            color = MSG_ReadByte(&mut buf) - 1 as libc::c_int;
            pos[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            entityIndex = MSG_ReadShort(&mut buf);
            decalIndex = MSG_ReadByte(&mut buf);
            CL_PlayerDecal(color, decalIndex, entityIndex, pos.as_mut_ptr());
        }
        113 | 114 => {
            pos[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos2[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos2[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos2[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            scale = MSG_ReadCoord(&mut buf);
            modelIndex = MSG_ReadShort(&mut buf);
            count = MSG_ReadByte(&mut buf);
            vel = MSG_ReadCoord(&mut buf);
            if type_0 == 113 as libc::c_int {
                R_Bubbles(pos.as_mut_ptr() as *const vec_t,
                          pos2.as_mut_ptr() as *const vec_t, scale,
                          modelIndex, count, vel);
            } else {
                R_BubbleTrail(pos.as_mut_ptr() as *const vec_t,
                              pos2.as_mut_ptr() as *const vec_t, scale,
                              modelIndex, count, vel);
            }
        }
        115 => {
            pos[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            modelIndex = MSG_ReadShort(&mut buf);
            decalIndex = MSG_ReadShort(&mut buf);
            color = MSG_ReadByte(&mut buf);
            scale = MSG_ReadByte(&mut buf) as libc::c_float;
            R_BloodSprite(pos.as_mut_ptr() as *const vec_t, color, modelIndex,
                          decalIndex, scale);
        }
        119 => {
            pos[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos2[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos2[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos2[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            modelIndex = MSG_ReadShort(&mut buf);
            life = MSG_ReadByte(&mut buf) as libc::c_float;
            color = MSG_ReadByte(&mut buf);
            R_Projectile(pos.as_mut_ptr() as *const vec_t,
                         pos2.as_mut_ptr() as *const vec_t, modelIndex,
                         life as libc::c_int, color, None);
        }
        121 => {
            color = MSG_ReadShort(&mut buf);
            modelIndex = MSG_ReadShort(&mut buf);
            count = MSG_ReadByte(&mut buf);
            random = MSG_ReadByte(&mut buf) as libc::c_float;
            R_PlayerSprites(color, modelIndex, count, random as libc::c_int);
        }
        122 => {
            pos[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            scale = MSG_ReadShort(&mut buf) as libc::c_float;
            color = MSG_ReadByte(&mut buf);
            life = MSG_ReadByte(&mut buf) as libc::c_float * 0.1f32;
            R_ParticleBurst(pos.as_mut_ptr() as *const vec_t,
                            scale as libc::c_int, color, life);
        }
        123 => {
            pos[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            scale = MSG_ReadShort(&mut buf) as libc::c_float;
            modelIndex = MSG_ReadShort(&mut buf);
            count = MSG_ReadByte(&mut buf);
            flags = MSG_ReadByte(&mut buf);
            life = MSG_ReadByte(&mut buf) as libc::c_float * 0.1f32;
            R_FireField(pos.as_mut_ptr(), scale as libc::c_int, modelIndex,
                        count, flags, life);
        }
        124 => {
            color = MSG_ReadByte(&mut buf);
            scale = MSG_ReadCoord(&mut buf);
            modelIndex = MSG_ReadShort(&mut buf);
            life = MSG_ReadShort(&mut buf) as libc::c_float * 0.1f32;
            R_AttachTentToPlayer(color, modelIndex, scale, life);
        }
        125 => { color = MSG_ReadByte(&mut buf); R_KillAttachedTents(color); }
        126 => {
            pos[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos2[0 as libc::c_int as usize] =
                MSG_ReadCoord(&mut buf) * 0.1f32;
            pos2[1 as libc::c_int as usize] =
                MSG_ReadCoord(&mut buf) * 0.1f32;
            pos2[2 as libc::c_int as usize] =
                MSG_ReadCoord(&mut buf) * 0.1f32;
            ang[0 as libc::c_int as usize] =
                MSG_ReadCoord(&mut buf) * 0.01f32;
            ang[1 as libc::c_int as usize] =
                MSG_ReadCoord(&mut buf) * 0.01f32;
            ang[2 as libc::c_int as usize] = 0.0f32;
            count = MSG_ReadByte(&mut buf);
            decalIndices[0 as libc::c_int as usize] = MSG_ReadByte(&mut buf);
            R_MultiGunshot(pos.as_mut_ptr() as *const vec_t,
                           pos2.as_mut_ptr() as *const vec_t,
                           ang.as_mut_ptr() as *const vec_t, count,
                           1 as libc::c_int, decalIndices.as_mut_ptr());
        }
        127 => {
            pos[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos2[0 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos2[1 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            pos2[2 as libc::c_int as usize] = MSG_ReadCoord(&mut buf);
            life = MSG_ReadByte(&mut buf) as libc::c_float * 0.1f32;
            color = MSG_ReadByte(&mut buf);
            scale = MSG_ReadByte(&mut buf) as libc::c_float * 0.1f32;
            R_UserTracerParticle(pos.as_mut_ptr(), pos2.as_mut_ptr(), life,
                                 color, scale, 0 as libc::c_int as byte,
                                 None);
        }
        _ => {
            Con_DPrintf(b"^1Error:^7 ParseTempEntity: illegible TE message %i\n\x00"
                            as *const u8 as *const libc::c_char, type_0);
        }
    }
    // throw warning
    if MSG_CheckOverflow(&mut buf) as u64 != 0 {
        Con_DPrintf(b"^3Warning:^7 ParseTempEntity: overflow TE message\n\x00"
                        as *const u8 as *const libc::c_char);
    };
}
// because we wan't interpolate fast sequences (like on\off)
/*
================
CL_ClearLightStyles
================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ClearLightStyles() {
    memset(cl.lightstyles.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[lightstyle_t; 64]>() as
               libc::c_ulong); // set local time
}
#[no_mangle]
pub unsafe extern "C" fn CL_SetLightstyle(mut style: libc::c_int,
                                          mut s: *const libc::c_char,
                                          mut f: libc::c_float) {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut ls: *mut lightstyle_t = 0 as *mut lightstyle_t;
    let mut val1: libc::c_float = 0.;
    let mut val2: libc::c_float = 0.;
    ls =
        &mut *cl.lightstyles.as_mut_ptr().offset(style as isize) as
            *mut lightstyle_t;
    Q_strncpy((*ls).pattern.as_mut_ptr(), s,
              ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong);
    (*ls).length = Q_strlen(s) as libc::c_int;
    (*ls).time = f;
    i = 0 as libc::c_int;
    while i < (*ls).length {
        (*ls).map[i as usize] =
            (*s.offset(i as isize) as libc::c_int - 'a' as i32) as
                libc::c_float;
        i += 1
    }
    (*ls).interp =
        if (*ls).length <= 1 as libc::c_int {
            false_0 as libc::c_int
        } else { true_0 as libc::c_int } as qboolean;
    // check for allow interpolate
	// NOTE: fast flickering styles looks ugly when interpolation is running
    k = 0 as libc::c_int;
    while k < (*ls).length - 1 as libc::c_int {
        val1 = (*ls).map[((k + 0 as libc::c_int) % (*ls).length) as usize];
        val2 = (*ls).map[((k + 1 as libc::c_int) % (*ls).length) as usize];
        if __tg_fabs(val1 - val2) > 3.0f32 {
            (*ls).interp = false_0;
            break ;
        } else { k += 1 }
    }
    Con_Reportf(b"Lightstyle %i (%s), interp %s\n\x00" as *const u8 as
                    *const libc::c_char, style, (*ls).pattern.as_mut_ptr(),
                if (*ls).interp as libc::c_uint != 0 {
                    b"Yes\x00" as *const u8 as *const libc::c_char
                } else { b"No\x00" as *const u8 as *const libc::c_char });
}
/*
==============================================================

DLIGHT MANAGEMENT

==============================================================
*/
#[no_mangle]
pub static mut cl_dlights: [dlight_t; 32] =
    [dlight_t{origin: [0.; 3],
              radius: 0.,
              color: color24{r: 0, g: 0, b: 0,},
              die: 0.,
              decay: 0.,
              minlight: 0.,
              key: 0,
              dark: false_0,}; 32];
#[no_mangle]
pub static mut cl_elights: [dlight_t; 64] =
    [dlight_t{origin: [0.; 3],
              radius: 0.,
              color: color24{r: 0, g: 0, b: 0,},
              die: 0.,
              decay: 0.,
              minlight: 0.,
              key: 0,
              dark: false_0,}; 64];
/*
================
CL_ClearDlights
================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ClearDlights() {
    memset(cl_dlights.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[dlight_t; 32]>() as libc::c_ulong);
    memset(cl_elights.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[dlight_t; 64]>() as libc::c_ulong);
}
/*
===============
CL_AllocDlight

===============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_AllocDlight(mut key: libc::c_int)
 -> *mut dlight_s {
    let mut dl: *mut dlight_t = 0 as *mut dlight_t;
    let mut i: libc::c_int = 0;
    // first look for an exact key match
    if key != 0 {
        i = 0 as libc::c_int;
        dl = cl_dlights.as_mut_ptr();
        while i < 32 as libc::c_int {
            if (*dl).key == key {
                // reuse this light
                memset(dl as *mut libc::c_void, 0 as libc::c_int,
                       ::std::mem::size_of::<dlight_t>() as libc::c_ulong);
                (*dl).key = key;
                return dl
            }
            i += 1;
            dl = dl.offset(1)
        }
    }
    // then look for anything else
    i = 0 as libc::c_int;
    dl = cl_dlights.as_mut_ptr();
    while i < 32 as libc::c_int {
        if ((*dl).die as libc::c_double) < cl.time &&
               (*dl).key == 0 as libc::c_int {
            memset(dl as *mut libc::c_void, 0 as libc::c_int,
                   ::std::mem::size_of::<dlight_t>() as libc::c_ulong);
            (*dl).key = key;
            return dl
        }
        i += 1;
        dl = dl.offset(1)
    }
    // otherwise grab first dlight
    dl =
        &mut *cl_dlights.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut dlight_t;
    memset(dl as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<dlight_t>() as libc::c_ulong);
    (*dl).key = key;
    return dl;
}
/*
===============
CL_AllocElight

===============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_AllocElight(mut key: libc::c_int)
 -> *mut dlight_s {
    let mut dl: *mut dlight_t = 0 as *mut dlight_t;
    let mut i: libc::c_int = 0;
    // first look for an exact key match
    if key != 0 {
        i = 0 as libc::c_int;
        dl = cl_elights.as_mut_ptr();
        while i < 64 as libc::c_int {
            if (*dl).key == key {
                // reuse this light
                memset(dl as *mut libc::c_void, 0 as libc::c_int,
                       ::std::mem::size_of::<dlight_t>() as libc::c_ulong);
                (*dl).key = key;
                return dl
            }
            i += 1;
            dl = dl.offset(1)
        }
    }
    // then look for anything else
    i = 0 as libc::c_int;
    dl = cl_elights.as_mut_ptr();
    while i < 64 as libc::c_int {
        if ((*dl).die as libc::c_double) < cl.time &&
               (*dl).key == 0 as libc::c_int {
            memset(dl as *mut libc::c_void, 0 as libc::c_int,
                   ::std::mem::size_of::<dlight_t>() as libc::c_ulong);
            (*dl).key = key;
            return dl
        }
        i += 1;
        dl = dl.offset(1)
    }
    // otherwise grab first dlight
    dl =
        &mut *cl_elights.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut dlight_t;
    memset(dl as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<dlight_t>() as libc::c_ulong);
    (*dl).key = key;
    return dl;
}
/*
===============
CL_DecayLights

===============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_DecayLights() {
    let mut dl: *mut dlight_t = 0 as *mut dlight_t;
    let mut time: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    time = (cl.time - cl.oldtime) as libc::c_float;
    i = 0 as libc::c_int;
    dl = cl_dlights.as_mut_ptr();
    while i < 32 as libc::c_int {
        if !((*dl).radius == 0.) {
            (*dl).radius -= time * (*dl).decay;
            if (*dl).radius < 0 as libc::c_int as libc::c_float {
                (*dl).radius = 0 as libc::c_int as libc::c_float
            }
            if ((*dl).die as libc::c_double) < cl.time || (*dl).radius == 0. {
                memset(dl as *mut libc::c_void, 0 as libc::c_int,
                       ::std::mem::size_of::<dlight_t>() as libc::c_ulong);
            }
        }
        i += 1;
        dl = dl.offset(1)
    }
    i = 0 as libc::c_int;
    dl = cl_elights.as_mut_ptr();
    while i < 64 as libc::c_int {
        if !((*dl).radius == 0.) {
            (*dl).radius -= time * (*dl).decay;
            if (*dl).radius < 0 as libc::c_int as libc::c_float {
                (*dl).radius = 0 as libc::c_int as libc::c_float
            }
            if ((*dl).die as libc::c_double) < cl.time || (*dl).radius == 0. {
                memset(dl as *mut libc::c_void, 0 as libc::c_int,
                       ::std::mem::size_of::<dlight_t>() as libc::c_ulong);
            }
        }
        i += 1;
        dl = dl.offset(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn CL_GetDynamicLight(mut number: libc::c_int)
 -> *mut dlight_t {
    return &mut *cl_dlights.as_mut_ptr().offset(number as isize) as
               *mut dlight_t;
}
#[no_mangle]
pub unsafe extern "C" fn CL_GetEntityLight(mut number: libc::c_int)
 -> *mut dlight_t {
    return &mut *cl_elights.as_mut_ptr().offset(number as isize) as
               *mut dlight_t;
}
/*
================
CL_UpdateFlashlight

update client flashlight
================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_UpdateFlashlight(mut ent: *mut cl_entity_t) {
    let mut forward: vec3_t = [0.; 3];
    let mut view_ofs: vec3_t = [0.; 3];
    let mut vecSrc: vec3_t = [0.; 3];
    let mut vecEnd: vec3_t = [0.; 3];
    let mut falloff: libc::c_float = 0.;
    let mut trace: *mut pmtrace_t = 0 as *mut pmtrace_t;
    let mut hit: *mut cl_entity_t = 0 as *mut cl_entity_t;
    let mut dl: *mut dlight_t = 0 as *mut dlight_t;
    if (*ent).index == cl.playernum + 1 as libc::c_int {
        // local player case
        AngleVectors(cl.viewangles.as_mut_ptr() as *const vec_t,
                     forward.as_mut_ptr(), 0 as *mut vec_t, 0 as *mut vec_t);
        view_ofs[0 as libc::c_int as usize] =
            cl.viewheight[0 as libc::c_int as usize];
        view_ofs[1 as libc::c_int as usize] =
            cl.viewheight[1 as libc::c_int as usize];
        view_ofs[2 as libc::c_int as usize] =
            cl.viewheight[2 as libc::c_int as usize]
    } else {
        // non-local player case
        let mut v_angle: vec3_t = [0.; 3];
        // DEFAULT_VIEWHEIGHT
        v_angle[0 as libc::c_int as usize] =
            (*ent).curstate.angles[0 as libc::c_int as usize] * 9.0f32;
        v_angle[1 as libc::c_int as usize] =
            (*ent).angles[1 as libc::c_int as usize];
        v_angle[2 as libc::c_int as usize] = 0.0f32;
        AngleVectors(v_angle.as_mut_ptr() as *const vec_t,
                     forward.as_mut_ptr(), 0 as *mut vec_t, 0 as *mut vec_t);
        view_ofs[1 as libc::c_int as usize] = 0.0f32;
        view_ofs[0 as libc::c_int as usize] =
            view_ofs[1 as libc::c_int as usize];
        if (*ent).curstate.usehull == 1 as libc::c_int {
            // NOTE: pitch divided by 3.0 twice. So we need apply 3^2 = 9
            // roll not used
            // FIXME: these values are hardcoded ...
            view_ofs[2 as libc::c_int as usize] = 12.0f32
        } else { view_ofs[2 as libc::c_int as usize] = 28.0f32 }
    } // VEC_DUCK_VIEW;
    vecSrc[0 as libc::c_int as usize] =
        (*ent).origin[0 as libc::c_int as usize] +
            view_ofs[0 as libc::c_int as usize];
    vecSrc[1 as libc::c_int as usize] =
        (*ent).origin[1 as libc::c_int as usize] +
            view_ofs[1 as libc::c_int as usize];
    vecSrc[2 as libc::c_int as usize] =
        (*ent).origin[2 as libc::c_int as usize] +
            view_ofs[2 as libc::c_int as usize];
    vecEnd[0 as libc::c_int as usize] =
        vecSrc[0 as libc::c_int as usize] +
            2000 as libc::c_int as libc::c_float *
                forward[0 as libc::c_int as usize];
    vecEnd[1 as libc::c_int as usize] =
        vecSrc[1 as libc::c_int as usize] +
            2000 as libc::c_int as libc::c_float *
                forward[1 as libc::c_int as usize];
    vecEnd[2 as libc::c_int as usize] =
        vecSrc[2 as libc::c_int as usize] +
            2000 as libc::c_int as libc::c_float *
                forward[2 as libc::c_int as usize];
    trace =
        CL_VisTraceLine(vecSrc.as_mut_ptr(), vecEnd.as_mut_ptr(),
                        0x2 as libc::c_int);
    // update flashlight endpos
    dl = CL_AllocDlight((*ent).index);
    hit =
        CL_GetEntityByIndex((*clgame.pmove).visents[(*trace).ent as
                                                        usize].info);
    if !hit.is_null() && !(*hit).model.is_null() &&
           ((*(*hit).model).type_0 as libc::c_int == mod_alias as libc::c_int
                ||
                (*(*hit).model).type_0 as libc::c_int ==
                    mod_studio as libc::c_int) {
        (*dl).origin[0 as libc::c_int as usize] =
            (*hit).origin[0 as libc::c_int as usize];
        (*dl).origin[1 as libc::c_int as usize] =
            (*hit).origin[1 as libc::c_int as usize];
        (*dl).origin[2 as libc::c_int as usize] =
            (*hit).origin[2 as libc::c_int as usize]
    } else {
        (*dl).origin[0 as libc::c_int as usize] =
            (*trace).endpos[0 as libc::c_int as usize];
        (*dl).origin[1 as libc::c_int as usize] =
            (*trace).endpos[1 as libc::c_int as usize];
        (*dl).origin[2 as libc::c_int as usize] =
            (*trace).endpos[2 as libc::c_int as usize]
    }
    // compute falloff
    falloff = (*trace).fraction * 2000 as libc::c_int as libc::c_float;
    if falloff < 500.0f32 {
        falloff = 1.0f32
    } else { falloff = 500.0f32 / falloff }
    falloff *= falloff;
    // apply brigthness to dlight
    (*dl).color.r =
        if falloff * 255 as libc::c_int as libc::c_float >=
               0 as libc::c_int as libc::c_float {
            if (falloff * 255 as libc::c_int as libc::c_float) <
                   255 as libc::c_int as libc::c_float {
                (falloff) * 255 as libc::c_int as libc::c_float
            } else { 255 as libc::c_int as libc::c_float }
        } else { 0 as libc::c_int as libc::c_float } as
            byte; // die on next frame
    (*dl).color.g =
        if falloff * 255 as libc::c_int as libc::c_float >=
               0 as libc::c_int as libc::c_float {
            if (falloff * 255 as libc::c_int as libc::c_float) <
                   255 as libc::c_int as libc::c_float {
                (falloff) * 255 as libc::c_int as libc::c_float
            } else { 255 as libc::c_int as libc::c_float }
        } else { 0 as libc::c_int as libc::c_float } as byte;
    (*dl).color.b =
        if falloff * 255 as libc::c_int as libc::c_float >=
               0 as libc::c_int as libc::c_float {
            if (falloff * 255 as libc::c_int as libc::c_float) <
                   255 as libc::c_int as libc::c_float {
                (falloff) * 255 as libc::c_int as libc::c_float
            } else { 255 as libc::c_int as libc::c_float }
        } else { 0 as libc::c_int as libc::c_float } as byte;
    (*dl).die = (cl.time + 0.01f32 as libc::c_double) as libc::c_float;
    (*dl).radius = 80 as libc::c_int as libc::c_float;
}
/*
================
CL_AddEntityEffects

apply various effects to entity origin or attachment
================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_AddEntityEffects(mut ent: *mut cl_entity_t) {
    // yellow flies effect 'monster stuck in the wall'
    if (*ent).curstate.effects & 1 as libc::c_int != 0 {
        R_EntityParticles(ent); // don't flickering
    }
    if (*ent).curstate.effects & 8 as libc::c_int != 0 {
        if (*ent).player as libc::c_uint != 0 &&
               Host_IsQuakeCompatible() as u64 == 0 {
            CL_UpdateFlashlight(ent);
        } else {
            let mut dl: *mut dlight_t = CL_AllocDlight((*ent).index);
            (*dl).color.b = 100 as libc::c_int as byte;
            (*dl).color.g = (*dl).color.b;
            (*dl).color.r = (*dl).color.g;
            (*dl).radius =
                COM_RandomFloat(200 as libc::c_int as libc::c_float,
                                231 as libc::c_int as libc::c_float);
            (*dl).origin[0 as libc::c_int as usize] =
                (*ent).origin[0 as libc::c_int as usize];
            (*dl).origin[1 as libc::c_int as usize] =
                (*ent).origin[1 as libc::c_int as usize];
            (*dl).origin[2 as libc::c_int as usize] =
                (*ent).origin[2 as libc::c_int as usize];
            (*dl).die = (cl.time + 0.001f64) as libc::c_float
        }
    }
    if (*ent).curstate.effects & 4 as libc::c_int != 0 {
        let mut dl_0: *mut dlight_t = CL_AllocDlight((*ent).index);
        (*dl_0).color.b = 250 as libc::c_int as byte;
        (*dl_0).color.g = (*dl_0).color.b;
        (*dl_0).color.r = (*dl_0).color.g;
        if (*ent).player as u64 != 0 {
            (*dl_0).radius = 400 as libc::c_int as libc::c_float
        } else {
            (*dl_0).radius =
                COM_RandomFloat(400 as libc::c_int as libc::c_float,
                                431 as libc::c_int as libc::c_float)
        }
        (*dl_0).origin[0 as libc::c_int as usize] =
            (*ent).origin[0 as libc::c_int as usize];
        (*dl_0).origin[1 as libc::c_int as usize] =
            (*ent).origin[1 as libc::c_int as usize];
        (*dl_0).origin[2 as libc::c_int as usize] =
            (*ent).origin[2 as libc::c_int as usize];
        (*dl_0).die = (cl.time + 0.001f64) as libc::c_float;
        (*dl_0).origin[2 as libc::c_int as usize] += 16.0f32
    }
    // add light effect
    if (*ent).curstate.effects & 64 as libc::c_int != 0 {
        let mut dl_1: *mut dlight_t = CL_AllocDlight((*ent).index);
        (*dl_1).color.b = 100 as libc::c_int as byte;
        (*dl_1).color.g = (*dl_1).color.b;
        (*dl_1).color.r = (*dl_1).color.g;
        (*dl_1).origin[0 as libc::c_int as usize] =
            (*ent).origin[0 as libc::c_int as usize];
        (*dl_1).origin[1 as libc::c_int as usize] =
            (*ent).origin[1 as libc::c_int as usize];
        (*dl_1).origin[2 as libc::c_int as usize] =
            (*ent).origin[2 as libc::c_int as usize];
        R_RocketFlare((*ent).origin.as_mut_ptr() as *const vec_t);
        (*dl_1).die = (cl.time + 0.001f64) as libc::c_float;
        (*dl_1).radius = 200 as libc::c_int as libc::c_float
    }
    // studio models are handle muzzleflashes difference
    if (*ent).curstate.effects & 2 as libc::c_int != 0 &&
           !Mod_AliasExtradata((*ent).model).is_null() {
        let mut dl_2: *mut dlight_t = CL_AllocDlight((*ent).index);
        let mut fv: vec3_t = [0.; 3];
        (*ent).curstate.effects =
            (*ent).curstate.effects & !(2 as libc::c_int);
        (*dl_2).color.b = 100 as libc::c_int as byte;
        (*dl_2).color.g = (*dl_2).color.b;
        (*dl_2).color.r = (*dl_2).color.g;
        (*dl_2).origin[0 as libc::c_int as usize] =
            (*ent).origin[0 as libc::c_int as usize];
        (*dl_2).origin[1 as libc::c_int as usize] =
            (*ent).origin[1 as libc::c_int as usize];
        (*dl_2).origin[2 as libc::c_int as usize] =
            (*ent).origin[2 as libc::c_int as usize];
        AngleVectors((*ent).angles.as_mut_ptr() as *const vec_t,
                     fv.as_mut_ptr(), 0 as *mut vec_t, 0 as *mut vec_t);
        (*dl_2).origin[2 as libc::c_int as usize] += 16.0f32;
        (*dl_2).origin[0 as libc::c_int as usize] =
            (*dl_2).origin[0 as libc::c_int as usize] +
                18 as libc::c_int as libc::c_float *
                    fv[0 as libc::c_int as usize];
        (*dl_2).origin[1 as libc::c_int as usize] =
            (*dl_2).origin[1 as libc::c_int as usize] +
                18 as libc::c_int as libc::c_float *
                    fv[1 as libc::c_int as usize];
        (*dl_2).origin[2 as libc::c_int as usize] =
            (*dl_2).origin[2 as libc::c_int as usize] +
                18 as libc::c_int as libc::c_float *
                    fv[2 as libc::c_int as usize];
        (*dl_2).radius =
            COM_RandomFloat(200 as libc::c_int as libc::c_float,
                            231 as libc::c_int as libc::c_float);
        (*dl_2).die = (cl.time + 0.1f64) as libc::c_float;
        (*dl_2).minlight = 32 as libc::c_int as libc::c_float
    };
}
/*
================
CL_AddModelEffects

these effects will be enable by flag in model header
================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_AddModelEffects(mut ent: *mut cl_entity_t) {
    let mut neworigin: vec3_t = [0.; 3];
    let mut oldorigin: vec3_t = [0.; 3];
    if (*ent).model.is_null() { return }
    match (*(*ent).model).type_0 as libc::c_int {
        2 | 3 => { }
        _ => { return }
    }
    if cls.demoplayback == DEMO_QUAKE1 as libc::c_int {
        oldorigin[0 as libc::c_int as usize] =
            (*ent).baseline.vuser1[0 as libc::c_int as usize];
        oldorigin[1 as libc::c_int as usize] =
            (*ent).baseline.vuser1[1 as libc::c_int as usize];
        oldorigin[2 as libc::c_int as usize] =
            (*ent).baseline.vuser1[2 as libc::c_int as usize];
        (*ent).baseline.vuser1[0 as libc::c_int as usize] =
            (*ent).origin[0 as libc::c_int as usize];
        (*ent).baseline.vuser1[1 as libc::c_int as usize] =
            (*ent).origin[1 as libc::c_int as usize];
        (*ent).baseline.vuser1[2 as libc::c_int as usize] =
            (*ent).origin[2 as libc::c_int as usize];
        neworigin[0 as libc::c_int as usize] =
            (*ent).origin[0 as libc::c_int as usize];
        neworigin[1 as libc::c_int as usize] =
            (*ent).origin[1 as libc::c_int as usize];
        neworigin[2 as libc::c_int as usize] =
            (*ent).origin[2 as libc::c_int as usize]
    } else {
        oldorigin[0 as libc::c_int as usize] =
            (*ent).prevstate.origin[0 as libc::c_int as usize];
        oldorigin[1 as libc::c_int as usize] =
            (*ent).prevstate.origin[1 as libc::c_int as usize];
        oldorigin[2 as libc::c_int as usize] =
            (*ent).prevstate.origin[2 as libc::c_int as usize];
        neworigin[0 as libc::c_int as usize] =
            (*ent).curstate.origin[0 as libc::c_int as usize];
        neworigin[1 as libc::c_int as usize] =
            (*ent).curstate.origin[1 as libc::c_int as usize];
        neworigin[2 as libc::c_int as usize] =
            (*ent).curstate.origin[2 as libc::c_int as usize]
    }
    // NOTE: this completely over control about angles and don't broke interpolation
    if (*(*ent).model).flags as libc::c_uint &
           (1 as libc::c_uint) << 3 as libc::c_int != 0 {
        (*ent).angles[1 as libc::c_int as usize] =
            anglemod((100.0f32 as libc::c_double * cl.time) as libc::c_float)
    }
    if (*(*ent).model).flags as libc::c_uint &
           (1 as libc::c_uint) << 2 as libc::c_int != 0 {
        R_RocketTrail(oldorigin.as_mut_ptr(), neworigin.as_mut_ptr(),
                      2 as libc::c_int);
    }
    if (*(*ent).model).flags as libc::c_uint &
           (1 as libc::c_uint) << 5 as libc::c_int != 0 {
        R_RocketTrail(oldorigin.as_mut_ptr(), neworigin.as_mut_ptr(),
                      4 as libc::c_int);
    }
    if (*(*ent).model).flags as libc::c_uint &
           (1 as libc::c_uint) << 4 as libc::c_int != 0 {
        R_RocketTrail(oldorigin.as_mut_ptr(), neworigin.as_mut_ptr(),
                      3 as libc::c_int);
    }
    if (*(*ent).model).flags as libc::c_uint &
           (1 as libc::c_uint) << 6 as libc::c_int != 0 {
        R_RocketTrail(oldorigin.as_mut_ptr(), neworigin.as_mut_ptr(),
                      5 as libc::c_int);
    }
    if (*(*ent).model).flags as libc::c_uint &
           (1 as libc::c_uint) << 0 as libc::c_int != 0 {
        let mut dl: *mut dlight_t = CL_AllocDlight((*ent).index);
        (*dl).color.b = 200 as libc::c_int as byte;
        (*dl).color.g = (*dl).color.b;
        (*dl).color.r = (*dl).color.g;
        (*dl).origin[0 as libc::c_int as usize] =
            (*ent).origin[0 as libc::c_int as usize];
        (*dl).origin[1 as libc::c_int as usize] =
            (*ent).origin[1 as libc::c_int as usize];
        (*dl).origin[2 as libc::c_int as usize] =
            (*ent).origin[2 as libc::c_int as usize];
        // XASH SPECIFIC: get radius from head entity
        if (*ent).curstate.rendermode != kRenderNormal as libc::c_int {
            (*dl).radius =
                if 0 as libc::c_int >
                       (*ent).curstate.renderamt - 55 as libc::c_int {
                    0 as libc::c_int
                } else { ((*ent).curstate.renderamt) - 55 as libc::c_int } as
                    libc::c_float
        } else { (*dl).radius = 200 as libc::c_int as libc::c_float }
        (*dl).die = (cl.time + 0.01f32 as libc::c_double) as libc::c_float;
        R_RocketTrail(oldorigin.as_mut_ptr(), neworigin.as_mut_ptr(),
                      0 as libc::c_int);
    }
    if (*(*ent).model).flags as libc::c_uint &
           (1 as libc::c_uint) << 1 as libc::c_int != 0 {
        R_RocketTrail(oldorigin.as_mut_ptr(), neworigin.as_mut_ptr(),
                      1 as libc::c_int);
    }
    if (*(*ent).model).flags as libc::c_uint &
           (1 as libc::c_uint) << 7 as libc::c_int != 0 {
        R_RocketTrail(oldorigin.as_mut_ptr(), neworigin.as_mut_ptr(),
                      6 as libc::c_int);
    };
}
/*
================
CL_TestLights

if cl_testlights is set, create 32 lights models
================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_TestLights() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut numLights: libc::c_int = 0;
    let mut forward: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    let mut f: libc::c_float = 0.;
    let mut r: libc::c_float = 0.;
    let mut dl: *mut dlight_t = 0 as *mut dlight_t;
    if if !cl_testlights.is_null() && (*cl_testlights).value != 0.0f32 {
           true_0 as libc::c_int
       } else { false_0 as libc::c_int } == 0 {
        return
    }
    numLights =
        if (*cl_testlights).value >= 1 as libc::c_int as libc::c_float {
            if (*cl_testlights).value < 32 as libc::c_int as libc::c_float {
                (*cl_testlights).value
            } else { 32 as libc::c_int as libc::c_float }
        } else { 1 as libc::c_int as libc::c_float } as libc::c_int;
    AngleVectors(cl.viewangles.as_mut_ptr() as *const vec_t,
                 forward.as_mut_ptr(), right.as_mut_ptr(), 0 as *mut vec_t);
    i = 0 as libc::c_int;
    while i < numLights {
        dl =
            &mut *cl_dlights.as_mut_ptr().offset(i as isize) as *mut dlight_t;
        r =
            64 as libc::c_int as libc::c_float *
                ((i % 4 as libc::c_int) as libc::c_float - 1.5f32);
        f =
            (64 as libc::c_int * (i / 4 as libc::c_int) + 128 as libc::c_int)
                as libc::c_float;
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            (*dl).origin[j as usize] =
                cl.simorg[j as usize] + forward[j as usize] * f +
                    right[j as usize] * r;
            j += 1
        }
        (*dl).color.r =
            (((i % 6 as libc::c_int + 1 as libc::c_int & 1 as libc::c_int) >>
                  0 as libc::c_int) * 255 as libc::c_int) as byte;
        (*dl).color.g =
            (((i % 6 as libc::c_int + 1 as libc::c_int & 2 as libc::c_int) >>
                  1 as libc::c_int) * 255 as libc::c_int) as byte;
        (*dl).color.b =
            (((i % 6 as libc::c_int + 1 as libc::c_int & 4 as libc::c_int) >>
                  2 as libc::c_int) * 255 as libc::c_int) as byte;
        (*dl).radius =
            if 64 as libc::c_int >
                   200 as libc::c_int - 5 as libc::c_int * numLights {
                64 as libc::c_int
            } else { (200 as libc::c_int) - 5 as libc::c_int * numLights } as
                libc::c_float;
        (*dl).die = (cl.time + host.frametime) as libc::c_float;
        i += 1
    };
}
/*
==============================================================

DECAL MANAGEMENT

==============================================================
*/
/*
===============
CL_FireCustomDecal

custom temporary decal
===============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_FireCustomDecal(mut textureIndex: libc::c_int,
                                            mut entityIndex: libc::c_int,
                                            mut modelIndex: libc::c_int,
                                            mut pos: *mut libc::c_float,
                                            mut flags: libc::c_int,
                                            mut scale: libc::c_float) {
    ref_0.dllFuncs.R_DecalShoot.expect("non-null function pointer")(textureIndex,
                                                                    entityIndex,
                                                                    modelIndex,
                                                                    pos,
                                                                    flags,
                                                                    scale);
}
/*
===============
CL_DecalShoot

normal temporary decal
===============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_DecalShoot(mut textureIndex: libc::c_int,
                                       mut entityIndex: libc::c_int,
                                       mut modelIndex: libc::c_int,
                                       mut pos: *mut libc::c_float,
                                       mut flags: libc::c_int) {
    CL_FireCustomDecal(textureIndex, entityIndex, modelIndex, pos, flags,
                       1.0f32);
}
/*
===============
CL_PlayerDecal

spray custom colored decal (clan logo etc)
===============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_PlayerDecal(mut playernum: libc::c_int,
                                        mut customIndex: libc::c_int,
                                        mut entityIndex: libc::c_int,
                                        mut pos: *mut libc::c_float) {
    let mut textureIndex: libc::c_int = 0 as libc::c_int;
    let mut pCust: *mut customization_t = 0 as *mut customization_t;
    if playernum < (1 as libc::c_int) << 5 as libc::c_int {
        pCust = cl.players[playernum as usize].customdata.pNext
    }
    if !pCust.is_null() && !(*pCust).pBuffer.is_null() &&
           !(*pCust).pInfo.is_null() {
        if (*pCust).resource.ucFlags as libc::c_int &
               (1 as libc::c_int) << 2 as libc::c_int != 0 &&
               (*pCust).resource.type_0 as libc::c_uint ==
                   t_decal as libc::c_int as libc::c_uint &&
               (*pCust).bTranslated as libc::c_uint != 0 {
            if (*pCust).nUserData1 == 0 && !(*pCust).pInfo.is_null() {
                let mut decalname: *const libc::c_char =
                    va(b"player%dlogo%d\x00" as *const u8 as
                           *const libc::c_char, playernum, customIndex);
                (*pCust).nUserData1 =
                    ref_0.dllFuncs.GL_LoadTextureFromBuffer.expect("non-null function pointer")(decalname,
                                                                                                (*pCust).pInfo
                                                                                                    as
                                                                                                    *mut rgbdata_t,
                                                                                                TF_CLAMP,
                                                                                                false_0)
            }
            textureIndex = (*pCust).nUserData1
        }
    }
    CL_DecalShoot(textureIndex, entityIndex, 0 as libc::c_int, pos,
                  0x4 as libc::c_int);
}
/*
===============
CL_DecalIndexFromName

get decal global index from decalname
===============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_DecalIndexFromName(mut name: *const libc::c_char)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    if if name.is_null() || *name == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return 0 as libc::c_int
    }
    // look through the loaded sprite name list for SpriteName
    i = 1 as libc::c_int;
    while i < 512 as libc::c_int &&
              host.draw_decals[i as usize][0 as libc::c_int as usize] as
                  libc::c_int != 0 {
        if Q_strnicmp(name, host.draw_decals[i as usize].as_mut_ptr(),
                      99999 as libc::c_int) == 0 {
            return i
        }
        i += 1
    }
    return 0 as libc::c_int;
    // invalid decal
}
/*
===============
CL_DecalIndex

get texture index from decal global index
===============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_DecalIndex(mut id: libc::c_int) -> libc::c_int {
    id =
        if id >= 0 as libc::c_int {
            if id < 512 as libc::c_int - 1 as libc::c_int {
                id
            } else { (512 as libc::c_int) - 1 as libc::c_int }
        } else { 0 as libc::c_int };
    if cl.decal_index[id as usize] as libc::c_int == 0 as libc::c_int {
        Image_SetForceFlags(IL_LOAD_DECAL as libc::c_int as uint);
        cl.decal_index[id as usize] =
            ref_0.dllFuncs.GL_LoadTexture.expect("non-null function pointer")(host.draw_decals[id
                                                                                                   as
                                                                                                   usize].as_mut_ptr(),
                                                                              0
                                                                                  as
                                                                                  *const byte,
                                                                              0
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  size_t,
                                                                              TF_CLAMP
                                                                                  as
                                                                                  libc::c_int)
                as libc::c_short;
        Image_ClearForceFlags();
    }
    return cl.decal_index[id as usize] as libc::c_int;
}
/*
===============
CL_DecalRemoveAll

remove all decals with specified texture
===============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_DecalRemoveAll(mut textureIndex: libc::c_int) {
    let mut id: libc::c_int =
        if textureIndex >= 0 as libc::c_int {
            if textureIndex < 512 as libc::c_int - 1 as libc::c_int {
                textureIndex
            } else { (512 as libc::c_int) - 1 as libc::c_int }
        } else { 0 as libc::c_int };
    ref_0.dllFuncs.R_DecalRemoveAll.expect("non-null function pointer")(cl.decal_index[id
                                                                                           as
                                                                                           usize]
                                                                            as
                                                                            libc::c_int);
}
/*
==============================================================

EFRAGS MANAGEMENT

==============================================================
*/
#[no_mangle]
pub static mut cl_efrags: [efrag_t; 8192] =
    [efrag_t{leaf: 0 as *const mleaf_s as *mut mleaf_s,
             leafnext: 0 as *const efrag_s as *mut efrag_s,
             entity: 0 as *const cl_entity_s as *mut cl_entity_s,
             entnext: 0 as *const efrag_s as *mut efrag_s,}; 8192];
/*
==============
CL_ClearEfrags
==============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ClearEfrags() {
    let mut i: libc::c_int = 0;
    memset(cl_efrags.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[efrag_t; 8192]>() as libc::c_ulong);
    // allocate the efrags and chain together into a free list
    clgame.free_efrags = cl_efrags.as_mut_ptr();
    i = 0 as libc::c_int;
    while i < 8192 as libc::c_int - 1 as libc::c_int {
        let ref mut fresh2 = (*clgame.free_efrags.offset(i as isize)).entnext;
        *fresh2 =
            &mut *clgame.free_efrags.offset((i + 1 as libc::c_int) as isize)
                as *mut efrag_t;
        i += 1
    }
    let ref mut fresh3 = (*clgame.free_efrags.offset(i as isize)).entnext;
    *fresh3 = 0 as *mut efrag_s;
}
/*
=======================
R_ClearStaticEntities

e.g. by demo request
=======================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ClearStaticEntities() {
    let mut i: libc::c_int = 0;
    if host.type_0 == HOST_DEDICATED as libc::c_int as libc::c_uint { return }
    // clear out efrags in case the level hasn't been reloaded
    i = 0 as libc::c_int;
    while i < (*cl.worldmodel).numleafs {
        let ref mut fresh4 =
            (*(*cl.worldmodel).leafs.offset((i + 1 as libc::c_int) as
                                                isize)).efrags;
        *fresh4 = 0 as *mut efrag_s;
        i += 1
    }
    clgame.numStatics = 0 as libc::c_int;
    CL_ClearEfrags();
}
/*
==============
CL_ClearEffects
==============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ClearEffects() {
    CL_ClearEfrags();
    CL_ClearDlights();
    CL_ClearTempEnts();
    CL_ClearViewBeams();
    CL_ClearParticles();
    CL_ClearLightStyles();
}
