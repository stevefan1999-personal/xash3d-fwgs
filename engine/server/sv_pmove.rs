#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types,
           ptr_wrapping_offset_from, register_tool)]
extern "C" {
    pub type file_s;
    pub type grasshdr_s;
    #[no_mangle]
    fn fabsf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn COM_RandomFloat(fMin: libc::c_float, fMax: libc::c_float)
     -> libc::c_float;
    #[no_mangle]
    fn COM_RandomLong(lMin: libc::c_int, lMax: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Info_ValueForKey(s: *const libc::c_char, key: *const libc::c_char)
     -> *const libc::c_char;
    #[no_mangle]
    fn Con_NPrintf(idx: libc::c_int, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Sys_DoubleTime() -> libc::c_double;
    #[no_mangle]
    fn Cvar_SetValue(var_name: *const libc::c_char, value: libc::c_float);
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    fn COM_FileSize(filename: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn COM_FreeFile(buffer: *mut libc::c_void);
    #[no_mangle]
    fn COM_MemFgets(pMemFile: *mut byte, fileSize: libc::c_int,
                    filePos: *mut libc::c_int, pBuffer: *mut libc::c_char,
                    bufferSize: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn pfnGetModelBounds(mod_0: *mut model_t, mins: *mut libc::c_float,
                         maxs: *mut libc::c_float);
    #[no_mangle]
    fn pfnGetModelType(mod_0: *mut model_t) -> libc::c_int;
    #[no_mangle]
    fn Con_Reportf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_DPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn SV_StartSound(ent: *mut edict_t, chan: libc::c_int,
                     sample: *const libc::c_char, vol: libc::c_float,
                     attn: libc::c_float, flags: libc::c_int,
                     pitch: libc::c_int);
    #[no_mangle]
    fn COM_LoadFile(filename: *const libc::c_char, usehunk: libc::c_int,
                    pLength: *mut libc::c_int) -> *mut byte;
    #[no_mangle]
    static mut svgame: svgame_static_t;
    #[no_mangle]
    fn SV_CheckEdict(e: *const edict_t, file: *const libc::c_char,
                     line: libc::c_int) -> qboolean;
    #[no_mangle]
    fn SV_PlaybackEventFull(flags: libc::c_int, pInvoker: *const edict_t,
                            eventindex: word, delay: libc::c_float,
                            origin: *mut libc::c_float,
                            angles: *mut libc::c_float,
                            fparam1: libc::c_float, fparam2: libc::c_float,
                            iparam1: libc::c_int, iparam2: libc::c_int,
                            bparam1: libc::c_int, bparam2: libc::c_int);
    #[no_mangle]
    fn SV_UpdateBaseVelocity(ent: *mut edict_t);
    #[no_mangle]
    fn VectorCompareEpsilon(vec1: *const vec_t, vec2: *const vec_t,
                            epsilon: vec_t) -> qboolean;
    #[no_mangle]
    fn BoundsIntersect(mins1: *const vec_t, maxs1: *const vec_t,
                       mins2: *const vec_t, maxs2: *const vec_t) -> qboolean;
    #[no_mangle]
    fn Matrix4x4_VectorITransform(in_0: *const [vec_t; 4],
                                  v: *const libc::c_float,
                                  out: *mut libc::c_float);
    #[no_mangle]
    fn Matrix4x4_CreateFromEntity(out: *mut [vec_t; 4], angles: *const vec_t,
                                  origin: *const vec_t, scale: libc::c_float);
    #[no_mangle]
    fn Matrix4x4_TransformPositivePlane(in_0: *const [vec_t; 4],
                                        normal: *const vec_t,
                                        d: libc::c_float, out: *mut vec_t,
                                        dist: *mut libc::c_float);
    #[no_mangle]
    fn MSG_WriteChar(sb: *mut sizebuf_t, val: libc::c_int);
    #[no_mangle]
    fn MSG_WriteByte(sb: *mut sizebuf_t, val: libc::c_int);
    #[no_mangle]
    fn MSG_WriteVec3Coord(sb: *mut sizebuf_t, fa: *const libc::c_float);
    #[no_mangle]
    static mut SV_UPDATE_BACKUP: libc::c_int;
    #[no_mangle]
    static mut svs: server_static_t;
    #[no_mangle]
    static mut sv: server_t;
    #[no_mangle]
    static mut sv_areanodes: [areanode_t; 0];
    #[no_mangle]
    static mut sv_unlag: convar_t;
    #[no_mangle]
    static mut sv_maxunlag: convar_t;
    #[no_mangle]
    static mut sv_unlagpush: convar_t;
    #[no_mangle]
    static mut sv_background_freeze: convar_t;
    #[no_mangle]
    fn SV_ModelHandle(modelindex: libc::c_int) -> *mut model_t;
    #[no_mangle]
    fn SV_PlayerRunThink(ent: *mut edict_t, frametime: libc::c_float,
                         time: libc::c_double) -> qboolean;
    #[no_mangle]
    fn SV_Impact(e1: *mut edict_t, e2: *mut edict_t, trace: *mut trace_t);
    #[no_mangle]
    fn SV_RefreshUserinfo();
    #[no_mangle]
    fn SV_LinkEdict(ent: *mut edict_t, touch_triggers: qboolean);
    #[no_mangle]
    fn SV_EdictNum(n: libc::c_int) -> *mut edict_t;
    #[no_mangle]
    fn SV_GetString(iString: string_t) -> *const libc::c_char;
    #[no_mangle]
    fn SV_SetMinMaxSize(e: *mut edict_t, min: *const libc::c_float,
                        max: *const libc::c_float, relink: qboolean);
    #[no_mangle]
    fn Pmove_Init();
    #[no_mangle]
    fn PM_HullForBsp(pe: *mut physent_t, pmove: *mut playermove_t,
                     offset: *mut libc::c_float) -> *mut hull_t;
    #[no_mangle]
    fn PM_RecursiveHullCheck(hull: *mut hull_t, num: libc::c_int,
                             p1f: libc::c_float, p2f: libc::c_float,
                             p1: *mut vec_t, p2: *mut vec_t,
                             trace: *mut pmtrace_t) -> qboolean;
    #[no_mangle]
    fn PM_PlayerTraceExt(pm: *mut playermove_t, p1: *mut vec_t,
                         p2: *mut vec_t, flags: libc::c_int,
                         numents: libc::c_int, ents: *mut physent_t,
                         ignore_pe: libc::c_int, pmFilter: pfnIgnore)
     -> pmtrace_t;
    #[no_mangle]
    fn PM_TestPlayerPosition(pmove: *mut playermove_t, pos: *mut vec_t,
                             ptrace: *mut pmtrace_t, pmFilter: pfnIgnore)
     -> libc::c_int;
    #[no_mangle]
    fn PM_HullPointContents(hull: *mut hull_t, num: libc::c_int,
                            p: *const vec_t) -> libc::c_int;
    #[no_mangle]
    fn PM_TruePointContents(pmove: *mut playermove_t, p: *const vec_t)
     -> libc::c_int;
    #[no_mangle]
    fn PM_PointContents(pmove: *mut playermove_t, p: *const vec_t)
     -> libc::c_int;
    #[no_mangle]
    fn PM_ConvertTrace(out: *mut trace_t, in_0: *mut pmtrace_t,
                       ent: *mut edict_t);
    #[no_mangle]
    fn PM_TraceTexture(pe: *mut physent_t, vstart: *mut vec_t,
                       vend: *mut vec_t) -> *const libc::c_char;
    #[no_mangle]
    fn PM_TraceSurface(pe: *mut physent_t, start: *mut vec_t, end: *mut vec_t)
     -> *mut msurface_t;
    #[no_mangle]
    fn PM_TestLineExt(pmove: *mut playermove_t, ents: *mut physent_t,
                      numents: libc::c_int, start: *const vec_t,
                      end: *const vec_t, flags: libc::c_int) -> libc::c_int;
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
pub type vec_t = libc::c_float;
pub type vec3_t = [vec_t; 3];
pub type matrix4x4 = [[vec_t; 4]; 4];
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
pub type pmtrace_t = pmtrace_s;
pub type physent_t = physent_s;
pub type physics_interface_t = physics_interface_s;
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
pub type SAVERESTOREDATA = saverestore_s;
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
pub struct TYPEDESCRIPTION {
    pub fieldType: FIELDTYPE,
    pub fieldName: *const libc::c_char,
    pub fieldOffset: libc::c_int,
    pub fieldSize: libc::c_short,
    pub flags: libc::c_short,
}
pub type FIELDTYPE = _fieldtypes;
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
pub type KeyValueData = KeyValueData_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct KeyValueData_s {
    pub szClassName: *mut libc::c_char,
    pub szKeyName: *mut libc::c_char,
    pub szValue: *mut libc::c_char,
    pub fHandled: libc::c_int,
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
pub type playermove_t = playermove_s;
pub type movevars_t = movevars_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sv_user_message_t {
    pub name: [libc::c_char; 32],
    pub number: libc::c_int,
    pub size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct areanode_s {
    pub axis: libc::c_int,
    pub dist: libc::c_float,
    pub children: [*mut areanode_s; 2],
    pub trigger_edicts: link_t,
    pub solid_edicts: link_t,
    pub portal_edicts: link_t,
}
pub type areanode_t = areanode_s;
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
pub type pfnIgnore
    =
    Option<unsafe extern "C" fn(_: *mut physent_t) -> libc::c_int>;
#[inline(always)]
unsafe extern "C" fn __tg_fabs(mut __x: libc::c_float) -> libc::c_float {
    return fabsf(__x);
}
/*
sv_pmove.c - server-side player physic
Copyright (C) 2010 Uncle Mike

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
*/
static mut has_update: qboolean = false_0;
#[no_mangle]
pub unsafe extern "C" fn SV_ClearPhysEnts() {
    (*svgame.pmove).numtouch = 0 as libc::c_int;
    (*svgame.pmove).numvisent = 0 as libc::c_int;
    (*svgame.pmove).nummoveent = 0 as libc::c_int;
    (*svgame.pmove).numphysent = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SV_PlayerIsFrozen(mut pClient: *mut edict_t)
 -> qboolean {
    if sv_background_freeze.value != 0. && sv.background as libc::c_uint != 0
       {
        return true_0
    }
    if host.features &
           ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint != 0 {
        return false_0
    }
    if (*pClient).v.flags as libc::c_uint &
           (1 as libc::c_uint) << 12 as libc::c_int != 0 {
        return true_0
    }
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn SV_ClipPMoveToEntity(mut pe: *mut physent_t,
                                              mut start: *const vec_t,
                                              mut mins: *mut vec_t,
                                              mut maxs: *mut vec_t,
                                              mut end: *const vec_t,
                                              mut tr: *mut pmtrace_t) {
    if svgame.physFuncs.ClipPMoveToEntity.is_some() {
        // do custom sweep test
        svgame.physFuncs.ClipPMoveToEntity.expect("non-null function pointer")(pe,
                                                                               start,
                                                                               mins,
                                                                               maxs,
                                                                               end,
                                                                               tr);
    } else {
        // function is missed, so we didn't hit anything
        (*tr).allsolid = false_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn SV_CopyEdictToPhysEnt(mut pe: *mut physent_t,
                                               mut ed: *mut edict_t)
 -> qboolean {
    let mut mod_0: *mut model_t = SV_ModelHandle((*ed).v.modelindex);
    if mod_0.is_null() { return false_0 }
    (*pe).player = false_0 as libc::c_int;
    (*pe).info =
        ed.wrapping_offset_from(svgame.edicts) as libc::c_long as libc::c_int;
    (*pe).origin[0 as libc::c_int as usize] =
        (*ed).v.origin[0 as libc::c_int as usize];
    (*pe).origin[1 as libc::c_int as usize] =
        (*ed).v.origin[1 as libc::c_int as usize];
    (*pe).origin[2 as libc::c_int as usize] =
        (*ed).v.origin[2 as libc::c_int as usize];
    (*pe).angles[0 as libc::c_int as usize] =
        (*ed).v.angles[0 as libc::c_int as usize];
    (*pe).angles[1 as libc::c_int as usize] =
        (*ed).v.angles[1 as libc::c_int as usize];
    (*pe).angles[2 as libc::c_int as usize] =
        (*ed).v.angles[2 as libc::c_int as usize];
    if (*ed).v.flags as libc::c_uint & (1 as libc::c_uint) << 3 as libc::c_int
           != 0 {
        // client
        SV_GetTrueOrigin(sv.current_client, (*pe).info,
                         (*pe).origin.as_mut_ptr());
        if (*ed).v.flags as libc::c_uint &
               (1 as libc::c_uint) << 13 as libc::c_int != 0 {
            // fakeclients have client flag too
            // bot
            Q_strncpy((*pe).name.as_mut_ptr(),
                      b"bot\x00" as *const u8 as *const libc::c_char,
                      ::std::mem::size_of::<[libc::c_char; 32]>() as
                          libc::c_ulong);
        } else {
            Q_strncpy((*pe).name.as_mut_ptr(),
                      b"player\x00" as *const u8 as *const libc::c_char,
                      ::std::mem::size_of::<[libc::c_char; 32]>() as
                          libc::c_ulong);
        }
        (*pe).player = (*pe).info
    } else {
        // otherwise copy the classname
        Q_strncpy((*pe).name.as_mut_ptr(), SV_GetString((*ed).v.classname),
                  ::std::mem::size_of::<[libc::c_char; 32]>() as
                      libc::c_ulong); // unused in GoldSrc
    }
    (*pe).studiomodel = 0 as *mut model_s;
    (*pe).model = (*pe).studiomodel;
    match (*ed).v.solid {
        0 | 4 => {
            (*pe).model = mod_0;
            (*pe).mins[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            (*pe).mins[1 as libc::c_int as usize] =
                (*pe).mins[2 as libc::c_int as usize];
            (*pe).mins[0 as libc::c_int as usize] =
                (*pe).mins[1 as libc::c_int as usize];
            (*pe).maxs[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            (*pe).maxs[1 as libc::c_int as usize] =
                (*pe).maxs[2 as libc::c_int as usize];
            (*pe).maxs[0 as libc::c_int as usize] =
                (*pe).maxs[1 as libc::c_int as usize]
        }
        2 => {
            if !mod_0.is_null() &&
                   (*mod_0).type_0 as libc::c_int == mod_studio as libc::c_int
                   &&
                   (*mod_0).flags as libc::c_uint &
                       (1 as libc::c_uint) << 9 as libc::c_int != 0 {
                (*pe).studiomodel = mod_0
            }
            (*pe).mins[0 as libc::c_int as usize] =
                (*ed).v.mins[0 as libc::c_int as usize];
            (*pe).mins[1 as libc::c_int as usize] =
                (*ed).v.mins[1 as libc::c_int as usize];
            (*pe).mins[2 as libc::c_int as usize] =
                (*ed).v.mins[2 as libc::c_int as usize];
            (*pe).maxs[0 as libc::c_int as usize] =
                (*ed).v.maxs[0 as libc::c_int as usize];
            (*pe).maxs[1 as libc::c_int as usize] =
                (*ed).v.maxs[1 as libc::c_int as usize];
            (*pe).maxs[2 as libc::c_int as usize] =
                (*ed).v.maxs[2 as libc::c_int as usize]
        }
        5 => {
            (*pe).model =
                if (*mod_0).type_0 as libc::c_int == mod_brush as libc::c_int
                   {
                    mod_0
                } else { 0 as *mut model_t };
            (*pe).studiomodel =
                if (*mod_0).type_0 as libc::c_int == mod_studio as libc::c_int
                   {
                    mod_0
                } else { 0 as *mut model_t };
            (*pe).mins[0 as libc::c_int as usize] =
                (*ed).v.mins[0 as libc::c_int as usize];
            (*pe).mins[1 as libc::c_int as usize] =
                (*ed).v.mins[1 as libc::c_int as usize];
            (*pe).mins[2 as libc::c_int as usize] =
                (*ed).v.mins[2 as libc::c_int as usize];
            (*pe).maxs[0 as libc::c_int as usize] =
                (*ed).v.maxs[0 as libc::c_int as usize];
            (*pe).maxs[1 as libc::c_int as usize] =
                (*ed).v.maxs[1 as libc::c_int as usize];
            (*pe).maxs[2 as libc::c_int as usize] =
                (*ed).v.maxs[2 as libc::c_int as usize]
        }
        _ => {
            (*pe).studiomodel =
                if (*mod_0).type_0 as libc::c_int == mod_studio as libc::c_int
                   {
                    mod_0
                } else { 0 as *mut model_t };
            (*pe).mins[0 as libc::c_int as usize] =
                (*ed).v.mins[0 as libc::c_int as usize];
            (*pe).mins[1 as libc::c_int as usize] =
                (*ed).v.mins[1 as libc::c_int as usize];
            (*pe).mins[2 as libc::c_int as usize] =
                (*ed).v.mins[2 as libc::c_int as usize];
            (*pe).maxs[0 as libc::c_int as usize] =
                (*ed).v.maxs[0 as libc::c_int as usize];
            (*pe).maxs[1 as libc::c_int as usize] =
                (*ed).v.maxs[1 as libc::c_int as usize];
            (*pe).maxs[2 as libc::c_int as usize] =
                (*ed).v.maxs[2 as libc::c_int as usize]
        }
    }
    (*pe).solid = (*ed).v.solid;
    (*pe).rendermode = (*ed).v.rendermode;
    (*pe).skin = (*ed).v.skin;
    (*pe).frame = (*ed).v.frame;
    (*pe).sequence = (*ed).v.sequence;
    memcpy(&mut *(*pe).controller.as_mut_ptr().offset(0 as libc::c_int as
                                                          isize) as *mut byte
               as *mut libc::c_void,
           &mut *(*ed).v.controller.as_mut_ptr().offset(0 as libc::c_int as
                                                            isize) as
               *mut byte as *const libc::c_void,
           (4 as libc::c_int as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<byte>() as
                                                libc::c_ulong));
    memcpy(&mut *(*pe).blending.as_mut_ptr().offset(0 as libc::c_int as isize)
               as *mut byte as *mut libc::c_void,
           &mut *(*ed).v.blending.as_mut_ptr().offset(0 as libc::c_int as
                                                          isize) as *mut byte
               as *const libc::c_void,
           (2 as libc::c_int as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<byte>() as
                                                libc::c_ulong));
    (*pe).movetype = (*ed).v.movetype;
    (*pe).takedamage = (*ed).v.takedamage as libc::c_int;
    (*pe).team = (*ed).v.team;
    (*pe).classnumber = (*ed).v.playerclass;
    (*pe).blooddecal = 0 as libc::c_int;
    // for mods
    (*pe).iuser1 = (*ed).v.iuser1;
    (*pe).iuser2 = (*ed).v.iuser2;
    (*pe).iuser3 = (*ed).v.iuser3;
    (*pe).iuser4 = (*ed).v.iuser4;
    (*pe).fuser1 = (*ed).v.fuser1;
    (*pe).fuser2 = (*ed).v.fuser2;
    (*pe).fuser3 = (*ed).v.fuser3;
    (*pe).fuser4 = (*ed).v.fuser4;
    (*pe).vuser1[0 as libc::c_int as usize] =
        (*ed).v.vuser1[0 as libc::c_int as usize];
    (*pe).vuser1[1 as libc::c_int as usize] =
        (*ed).v.vuser1[1 as libc::c_int as usize];
    (*pe).vuser1[2 as libc::c_int as usize] =
        (*ed).v.vuser1[2 as libc::c_int as usize];
    (*pe).vuser2[0 as libc::c_int as usize] =
        (*ed).v.vuser2[0 as libc::c_int as usize];
    (*pe).vuser2[1 as libc::c_int as usize] =
        (*ed).v.vuser2[1 as libc::c_int as usize];
    (*pe).vuser2[2 as libc::c_int as usize] =
        (*ed).v.vuser2[2 as libc::c_int as usize];
    (*pe).vuser3[0 as libc::c_int as usize] =
        (*ed).v.vuser3[0 as libc::c_int as usize];
    (*pe).vuser3[1 as libc::c_int as usize] =
        (*ed).v.vuser3[1 as libc::c_int as usize];
    (*pe).vuser3[2 as libc::c_int as usize] =
        (*ed).v.vuser3[2 as libc::c_int as usize];
    (*pe).vuser4[0 as libc::c_int as usize] =
        (*ed).v.vuser4[0 as libc::c_int as usize];
    (*pe).vuser4[1 as libc::c_int as usize] =
        (*ed).v.vuser4[1 as libc::c_int as usize];
    (*pe).vuser4[2 as libc::c_int as usize] =
        (*ed).v.vuser4[2 as libc::c_int as usize];
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn SV_ShouldUnlagForPlayer(mut cl: *mut sv_client_t)
 -> qboolean {
    // can't unlag in singleplayer
    if svs.maxclients <= 1 as libc::c_int { return false_0 }
    // unlag disabled globally
    if svgame.dllFuncs.pfnAllowLagCompensation.expect("non-null function pointer")()
           == 0 || sv_unlag.value == 0. {
        return false_0
    }
    if (*cl).flags & (1 as libc::c_uint) << 6 as libc::c_int == 0 {
        return false_0
    }
    // player not ready
    if (*cl).state as libc::c_uint !=
           cs_spawned as libc::c_int as libc::c_uint {
        return false_0
    }
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn SV_GetTrueOrigin(mut cl: *mut sv_client_t,
                                          mut edictnum: libc::c_int,
                                          mut origin: *mut vec_t) {
    if SV_ShouldUnlagForPlayer(cl) as u64 == 0 { return }
    if edictnum < 1 as libc::c_int || edictnum > svs.maxclients { return }
    if svgame.interp[(edictnum - 1 as libc::c_int) as usize].active as
           libc::c_uint != 0 &&
           svgame.interp[(edictnum - 1 as libc::c_int) as usize].moving as
               libc::c_uint != 0 {
        *origin.offset(0 as libc::c_int as isize) =
            svgame.interp[(edictnum - 1 as libc::c_int) as
                              usize].oldpos[0 as libc::c_int as usize];
        *origin.offset(1 as libc::c_int as isize) =
            svgame.interp[(edictnum - 1 as libc::c_int) as
                              usize].oldpos[1 as libc::c_int as usize];
        *origin.offset(2 as libc::c_int as isize) =
            svgame.interp[(edictnum - 1 as libc::c_int) as
                              usize].oldpos[2 as libc::c_int as usize]
    };
}
#[no_mangle]
pub unsafe extern "C" fn SV_GetTrueMinMax(mut cl: *mut sv_client_t,
                                          mut edictnum: libc::c_int,
                                          mut mins: *mut vec_t,
                                          mut maxs: *mut vec_t) {
    if SV_ShouldUnlagForPlayer(cl) as u64 == 0 { return }
    if edictnum < 1 as libc::c_int || edictnum > svs.maxclients { return }
    if svgame.interp[(edictnum - 1 as libc::c_int) as usize].active as
           libc::c_uint != 0 &&
           svgame.interp[(edictnum - 1 as libc::c_int) as usize].moving as
               libc::c_uint != 0 {
        *mins.offset(0 as libc::c_int as isize) =
            svgame.interp[(edictnum - 1 as libc::c_int) as
                              usize].mins[0 as libc::c_int as usize];
        *mins.offset(1 as libc::c_int as isize) =
            svgame.interp[(edictnum - 1 as libc::c_int) as
                              usize].mins[1 as libc::c_int as usize];
        *mins.offset(2 as libc::c_int as isize) =
            svgame.interp[(edictnum - 1 as libc::c_int) as
                              usize].mins[2 as libc::c_int as usize];
        *maxs.offset(0 as libc::c_int as isize) =
            svgame.interp[(edictnum - 1 as libc::c_int) as
                              usize].maxs[0 as libc::c_int as usize];
        *maxs.offset(1 as libc::c_int as isize) =
            svgame.interp[(edictnum - 1 as libc::c_int) as
                              usize].maxs[1 as libc::c_int as usize];
        *maxs.offset(2 as libc::c_int as isize) =
            svgame.interp[(edictnum - 1 as libc::c_int) as
                              usize].maxs[2 as libc::c_int as usize]
    };
}
/*
====================
SV_AddLinksToPmove

collect solid entities
====================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_AddLinksToPmove(mut node: *mut areanode_t,
                                            mut pmove_mins: *const vec_t,
                                            mut pmove_maxs: *const vec_t) {
    let mut l: *mut link_t = 0 as *mut link_t;
    let mut next: *mut link_t = 0 as *mut link_t;
    let mut check: *mut edict_t = 0 as *mut edict_t;
    let mut pl: *mut edict_t = 0 as *mut edict_t;
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut pe: *mut physent_t = 0 as *mut physent_t;
    pl = SV_EdictNum((*svgame.pmove).player_index + 1 as libc::c_int);
    let mut current_block_21: u64;
    // touch linked edicts
    l = (*node).solid_edicts.next; // player or player's own missile
    while l != &mut (*node).solid_edicts as *mut link_t {
        next = (*l).next;
        check =
            (l as *mut byte).offset(-(8 as libc::c_ulong as isize)) as
                *mut edict_t;
        if (*check).v.groupinfo != 0 as libc::c_int {
            if svs.groupop == 0 as libc::c_int &&
                   (*check).v.groupinfo & (*pl).v.groupinfo == 0 {
                current_block_21 = 7502529970979898288;
            } else if svs.groupop == 1 as libc::c_int &&
                          (*check).v.groupinfo & (*pl).v.groupinfo != 0 {
                current_block_21 = 7502529970979898288;
            } else { current_block_21 = 7746791466490516765; }
        } else { current_block_21 = 7746791466490516765; }
        match current_block_21 {
            7746791466490516765 => {
                if !((*check).v.owner == pl ||
                         (*check).v.solid == 1 as libc::c_int) {
                    if (*svgame.pmove).numvisent < 600 as libc::c_int {
                        pe =
                            &mut *(*svgame.pmove).visents.as_mut_ptr().offset((*svgame.pmove).numvisent
                                                                                  as
                                                                                  isize)
                                as *mut physent_t;
                        if SV_CopyEdictToPhysEnt(pe, check) as u64 != 0 {
                            (*svgame.pmove).numvisent += 1
                        }
                    }
                    if !((*check).v.solid == 0 as libc::c_int &&
                             ((*check).v.skin == 0 as libc::c_int ||
                                  (*check).v.modelindex == 0 as libc::c_int))
                       {
                        // ignore monsterclip brushes
                        if !((*check).v.flags as libc::c_uint &
                                 (1 as libc::c_uint) << 23 as libc::c_int != 0
                                 && (*check).v.solid == 4 as libc::c_int) {
                            if !(check == pl) { // himself
                                // nehahra collision flags
                                if (*check).v.movetype != 7 as libc::c_int {
                                    if (*check).v.flags as libc::c_uint &
                                           ((1 as libc::c_uint) <<
                                                3 as libc::c_int |
                                                (1 as libc::c_uint) <<
                                                    13 as libc::c_int) != 0 &&
                                           (*check).v.health <= 0.0f32 ||
                                           (*check).v.deadflag ==
                                               2 as libc::c_int {
                                        current_block_21 =
                                            7502529970979898288;
                                    } else {
                                        current_block_21 =
                                            11298138898191919651;
                                    }
                                    // dead body
                                } else {
                                    current_block_21 = 11298138898191919651;
                                }
                                match current_block_21 {
                                    7502529970979898288 => { }
                                    _ => {
                                        if !((*check).v.size[0 as libc::c_int
                                                                 as usize] ==
                                                 0.0f32 &&
                                                 (*check).v.size[1 as
                                                                     libc::c_int
                                                                     as usize]
                                                     == 0.0f32 &&
                                                 (*check).v.size[2 as
                                                                     libc::c_int
                                                                     as usize]
                                                     == 0.0f32) {
                                            mins[0 as libc::c_int as usize] =
                                                (*check).v.absmin[0 as
                                                                      libc::c_int
                                                                      as
                                                                      usize];
                                            mins[1 as libc::c_int as usize] =
                                                (*check).v.absmin[1 as
                                                                      libc::c_int
                                                                      as
                                                                      usize];
                                            mins[2 as libc::c_int as usize] =
                                                (*check).v.absmin[2 as
                                                                      libc::c_int
                                                                      as
                                                                      usize];
                                            maxs[0 as libc::c_int as usize] =
                                                (*check).v.absmax[0 as
                                                                      libc::c_int
                                                                      as
                                                                      usize];
                                            maxs[1 as libc::c_int as usize] =
                                                (*check).v.absmax[1 as
                                                                      libc::c_int
                                                                      as
                                                                      usize];
                                            maxs[2 as libc::c_int as usize] =
                                                (*check).v.absmax[2 as
                                                                      libc::c_int
                                                                      as
                                                                      usize];
                                            if (*check).v.flags as
                                                   libc::c_uint &
                                                   (1 as libc::c_uint) <<
                                                       3 as libc::c_int != 0
                                                   &&
                                                   (*check).v.flags as
                                                       libc::c_uint &
                                                       (1 as libc::c_uint) <<
                                                           13 as libc::c_int
                                                       == 0 {
                                                if !sv.current_client.is_null()
                                                   {
                                                    // trying to get interpolated values
                                                    SV_GetTrueMinMax(sv.current_client,
                                                                     check.wrapping_offset_from(svgame.edicts)
                                                                         as
                                                                         libc::c_long
                                                                         as
                                                                         libc::c_int,
                                                                     mins.as_mut_ptr(),
                                                                     maxs.as_mut_ptr());
                                                }
                                            }
                                            if !(BoundsIntersect(pmove_mins,
                                                                 pmove_maxs,
                                                                 mins.as_mut_ptr()
                                                                     as
                                                                     *const vec_t,
                                                                 maxs.as_mut_ptr()
                                                                     as
                                                                     *const vec_t)
                                                     as u64 == 0) {
                                                if (*svgame.pmove).numphysent
                                                       < 600 as libc::c_int {
                                                    pe =
                                                        &mut *(*svgame.pmove).physents.as_mut_ptr().offset((*svgame.pmove).numphysent
                                                                                                               as
                                                                                                               isize)
                                                            as *mut physent_t;
                                                    if SV_CopyEdictToPhysEnt(pe,
                                                                             check)
                                                           as u64 != 0 {
                                                        (*svgame.pmove).numphysent
                                                            += 1
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => { }
        }
        l = next
    }
    // recurse down both sides
    if (*node).axis == -(1 as libc::c_int) { return }
    if *pmove_maxs.offset((*node).axis as isize) > (*node).dist {
        SV_AddLinksToPmove((*node).children[0 as libc::c_int as usize],
                           pmove_mins, pmove_maxs);
    }
    if *pmove_mins.offset((*node).axis as isize) < (*node).dist {
        SV_AddLinksToPmove((*node).children[1 as libc::c_int as usize],
                           pmove_mins, pmove_maxs);
    };
}
/*
====================
SV_AddLaddersToPmove
====================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_AddLaddersToPmove(mut node: *mut areanode_t,
                                              mut pmove_mins: *const vec_t,
                                              mut pmove_maxs: *const vec_t) {
    let mut l: *mut link_t = 0 as *mut link_t;
    let mut next: *mut link_t = 0 as *mut link_t;
    let mut check: *mut edict_t = 0 as *mut edict_t;
    let mut mod_0: *mut model_t = 0 as *mut model_t;
    let mut pe: *mut physent_t = 0 as *mut physent_t;
    // get ladder edicts
    l = (*node).solid_edicts.next;
    while l != &mut (*node).solid_edicts as *mut link_t {
        next = (*l).next;
        check =
            (l as *mut byte).offset(-(8 as libc::c_ulong as isize)) as
                *mut edict_t;
        if !((*check).v.solid != 0 as libc::c_int ||
                 (*check).v.skin != -(16 as libc::c_int)) {
            mod_0 = SV_ModelHandle((*check).v.modelindex);
            // only brushes can have special contents
            if !(mod_0.is_null() ||
                     (*mod_0).type_0 as libc::c_int !=
                         mod_brush as libc::c_int) {
                if !(BoundsIntersect(pmove_mins, pmove_maxs,
                                     (*check).v.absmin.as_mut_ptr() as
                                         *const vec_t,
                                     (*check).v.absmax.as_mut_ptr() as
                                         *const vec_t) as u64 == 0) {
                    if (*svgame.pmove).nummoveent == 64 as libc::c_int {
                        return
                    }
                    pe =
                        &mut *(*svgame.pmove).moveents.as_mut_ptr().offset((*svgame.pmove).nummoveent
                                                                               as
                                                                               isize)
                            as *mut physent_t;
                    if SV_CopyEdictToPhysEnt(pe, check) as u64 != 0 {
                        (*svgame.pmove).nummoveent += 1
                    }
                }
            }
        }
        l = next
    }
    // recurse down both sides
    if (*node).axis == -(1 as libc::c_int) { return } // no x-vel
    if *pmove_maxs.offset((*node).axis as isize) > (*node).dist {
        SV_AddLaddersToPmove((*node).children[0 as libc::c_int as usize],
                             pmove_mins, pmove_maxs); // no y-vel
    } // write z-vel
    if *pmove_mins.offset((*node).axis as isize) < (*node).dist {
        SV_AddLaddersToPmove((*node).children[1 as libc::c_int as usize],
                             pmove_mins, pmove_maxs); // bad ground
    }; // no local clients for dedicated server
}
unsafe extern "C" fn pfnParticle(mut origin: *const libc::c_float,
                                 mut color: libc::c_int,
                                 mut life: libc::c_float,
                                 mut zpos: libc::c_int,
                                 mut zvel: libc::c_int) {
    let mut v: libc::c_int = 0; // bad ground
    if origin.is_null() {
        Con_Reportf(b"^1Error:^7 SV_StartParticle: NULL origin. Ignored\n\x00"
                        as *const u8 as *const libc::c_char);
        return
    }
    MSG_WriteByte(&mut sv.reliable_datagram, 18 as libc::c_int);
    MSG_WriteVec3Coord(&mut sv.reliable_datagram, origin);
    MSG_WriteChar(&mut sv.reliable_datagram, 0 as libc::c_int);
    MSG_WriteChar(&mut sv.reliable_datagram, 0 as libc::c_int);
    v =
        if (zpos * zvel) as libc::c_float * 16.0f32 >=
               -(128 as libc::c_int) as libc::c_float {
            if (zpos * zvel) as libc::c_float * 16.0f32 <
                   127 as libc::c_int as libc::c_float {
                ((zpos * zvel) as libc::c_float) * 16.0f32
            } else { 127 as libc::c_int as libc::c_float }
        } else { -(128 as libc::c_int) as libc::c_float } as libc::c_int;
    MSG_WriteChar(&mut sv.reliable_datagram, v);
    MSG_WriteByte(&mut sv.reliable_datagram, 1 as libc::c_int);
    MSG_WriteByte(&mut sv.reliable_datagram, color);
    MSG_WriteByte(&mut sv.reliable_datagram,
                  if life * 8 as libc::c_int as libc::c_float >=
                         0 as libc::c_int as libc::c_float {
                      if (life * 8 as libc::c_int as libc::c_float) <
                             255 as libc::c_int as libc::c_float {
                          (life) * 8 as libc::c_int as libc::c_float
                      } else { 255 as libc::c_int as libc::c_float }
                  } else { 0 as libc::c_int as libc::c_float } as
                      libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn SV_TestLine(mut start: *const vec_t,
                                     mut end: *const vec_t,
                                     mut flags: libc::c_int) -> libc::c_int {
    return PM_TestLineExt(svgame.pmove, (*svgame.pmove).physents.as_mut_ptr(),
                          (*svgame.pmove).numphysent, start, end, flags);
}
unsafe extern "C" fn pfnTestPlayerPosition(mut pos: *mut libc::c_float,
                                           mut ptrace: *mut pmtrace_t)
 -> libc::c_int {
    return PM_TestPlayerPosition(svgame.pmove, pos, ptrace, None);
}
unsafe extern "C" fn pfnStuckTouch(mut hitent: libc::c_int,
                                   mut tr: *mut pmtrace_t) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*svgame.pmove).numtouch {
        if (*svgame.pmove).touchindex[i as usize].ent == hitent { return }
        i += 1
    }
    if (*svgame.pmove).numtouch >= 600 as libc::c_int { return }
    (*tr).deltavelocity[0 as libc::c_int as usize] =
        (*svgame.pmove).velocity[0 as libc::c_int as usize];
    (*tr).deltavelocity[1 as libc::c_int as usize] =
        (*svgame.pmove).velocity[1 as libc::c_int as usize];
    (*tr).deltavelocity[2 as libc::c_int as usize] =
        (*svgame.pmove).velocity[2 as libc::c_int as usize];
    (*tr).ent = hitent;
    let fresh0 = (*svgame.pmove).numtouch;
    (*svgame.pmove).numtouch = (*svgame.pmove).numtouch + 1;
    (*svgame.pmove).touchindex[fresh0 as usize] = *tr;
}
unsafe extern "C" fn pfnPointContents(mut p: *mut libc::c_float,
                                      mut truecontents: *mut libc::c_int)
 -> libc::c_int {
    let mut cont: libc::c_int = 0;
    let mut truecont: libc::c_int = 0;
    cont = PM_PointContents(svgame.pmove, p as *const vec_t);
    truecont = cont;
    if !truecontents.is_null() { *truecontents = truecont }
    if cont <= -(9 as libc::c_int) && cont >= -(14 as libc::c_int) {
        cont = -(3 as libc::c_int)
    }
    return cont;
}
unsafe extern "C" fn pfnTruePointContents(mut p: *mut libc::c_float)
 -> libc::c_int {
    return PM_TruePointContents(svgame.pmove, p as *const vec_t);
}
unsafe extern "C" fn pfnHullPointContents(mut hull: *mut hull_s,
                                          mut num: libc::c_int,
                                          mut p: *mut libc::c_float)
 -> libc::c_int {
    return PM_HullPointContents(hull, num, p as *const vec_t);
}
unsafe extern "C" fn pfnPlayerTrace(mut start: *mut libc::c_float,
                                    mut end: *mut libc::c_float,
                                    mut traceFlags: libc::c_int,
                                    mut ignore_pe: libc::c_int) -> pmtrace_t {
    return PM_PlayerTraceExt(svgame.pmove, start, end, traceFlags,
                             (*svgame.pmove).numphysent,
                             (*svgame.pmove).physents.as_mut_ptr(), ignore_pe,
                             None);
}
unsafe extern "C" fn pfnTraceLine(mut start: *mut libc::c_float,
                                  mut end: *mut libc::c_float,
                                  mut flags: libc::c_int,
                                  mut usehull: libc::c_int,
                                  mut ignore_pe: libc::c_int)
 -> *mut pmtrace_t {
    static mut tr: pmtrace_t =
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
    let mut old_usehull: libc::c_int = 0;
    old_usehull = (*svgame.pmove).usehull;
    (*svgame.pmove).usehull = usehull;
    match flags {
        0 => {
            tr =
                PM_PlayerTraceExt(svgame.pmove, start, end, 0 as libc::c_int,
                                  (*svgame.pmove).numphysent,
                                  (*svgame.pmove).physents.as_mut_ptr(),
                                  ignore_pe, None)
        }
        1 => {
            tr =
                PM_PlayerTraceExt(svgame.pmove, start, end, 0 as libc::c_int,
                                  (*svgame.pmove).numvisent,
                                  (*svgame.pmove).visents.as_mut_ptr(),
                                  ignore_pe, None)
        }
        _ => { }
    }
    (*svgame.pmove).usehull = old_usehull;
    return &mut tr;
}
unsafe extern "C" fn pfnHullForBsp(mut pe: *mut physent_t,
                                   mut offset: *mut libc::c_float)
 -> *mut hull_t {
    return PM_HullForBsp(pe, svgame.pmove, offset);
}
unsafe extern "C" fn pfnTraceModel(mut pe: *mut physent_t,
                                   mut start: *mut libc::c_float,
                                   mut end: *mut libc::c_float,
                                   mut trace: *mut trace_t) -> libc::c_float {
    let mut old_usehull: libc::c_int = 0;
    let mut start_l: vec3_t = [0.; 3];
    let mut end_l: vec3_t = [0.; 3];
    let mut offset: vec3_t = [0.; 3];
    let mut temp: vec3_t = [0.; 3];
    let mut rotated: qboolean = false_0;
    let mut matrix: matrix4x4 = [[0.; 4]; 4];
    let mut hull: *mut hull_t = 0 as *mut hull_t;
    old_usehull = (*svgame.pmove).usehull;
    (*svgame.pmove).usehull = 2 as libc::c_int;
    hull = PM_HullForBsp(pe, svgame.pmove, offset.as_mut_ptr());
    (*svgame.pmove).usehull = old_usehull;
    if (*pe).solid == 4 as libc::c_int &&
           !((*pe).angles[0 as libc::c_int as usize] == 0.0f32 &&
                 (*pe).angles[1 as libc::c_int as usize] == 0.0f32 &&
                 (*pe).angles[2 as libc::c_int as usize] == 0.0f32) {
        rotated = true_0
    } else { rotated = false_0 }
    if rotated as u64 != 0 {
        Matrix4x4_CreateFromEntity(matrix.as_mut_ptr(),
                                   (*pe).angles.as_mut_ptr() as *const vec_t,
                                   offset.as_mut_ptr() as *const vec_t,
                                   1.0f32);
        Matrix4x4_VectorITransform(matrix.as_mut_ptr() as *const [vec_t; 4],
                                   start as *const libc::c_float,
                                   start_l.as_mut_ptr());
        Matrix4x4_VectorITransform(matrix.as_mut_ptr() as *const [vec_t; 4],
                                   end as *const libc::c_float,
                                   end_l.as_mut_ptr());
    } else {
        start_l[0 as libc::c_int as usize] =
            *start.offset(0 as libc::c_int as isize) -
                offset[0 as libc::c_int as usize];
        start_l[1 as libc::c_int as usize] =
            *start.offset(1 as libc::c_int as isize) -
                offset[1 as libc::c_int as usize];
        start_l[2 as libc::c_int as usize] =
            *start.offset(2 as libc::c_int as isize) -
                offset[2 as libc::c_int as usize];
        end_l[0 as libc::c_int as usize] =
            *end.offset(0 as libc::c_int as isize) -
                offset[0 as libc::c_int as usize];
        end_l[1 as libc::c_int as usize] =
            *end.offset(1 as libc::c_int as isize) -
                offset[1 as libc::c_int as usize];
        end_l[2 as libc::c_int as usize] =
            *end.offset(2 as libc::c_int as isize) -
                offset[2 as libc::c_int as usize]
    }
    PM_RecursiveHullCheck(hull, (*hull).firstclipnode,
                          0 as libc::c_int as libc::c_float,
                          1 as libc::c_int as libc::c_float,
                          start_l.as_mut_ptr(), end_l.as_mut_ptr(),
                          trace as *mut pmtrace_t);
    (*trace).ent = 0 as *mut edict_t;
    if rotated as u64 != 0 {
        temp[0 as libc::c_int as usize] =
            (*trace).plane.normal[0 as libc::c_int as usize];
        temp[1 as libc::c_int as usize] =
            (*trace).plane.normal[1 as libc::c_int as usize];
        temp[2 as libc::c_int as usize] =
            (*trace).plane.normal[2 as libc::c_int as usize];
        Matrix4x4_TransformPositivePlane(matrix.as_mut_ptr() as
                                             *const [vec_t; 4],
                                         temp.as_mut_ptr() as *const vec_t,
                                         (*trace).plane.dist,
                                         (*trace).plane.normal.as_mut_ptr(),
                                         &mut (*trace).plane.dist);
    }
    (*trace).endpos[0 as libc::c_int as usize] =
        *start.offset(0 as libc::c_int as isize) +
            (*trace).fraction *
                (*end.offset(0 as libc::c_int as isize) -
                     *start.offset(0 as libc::c_int as isize));
    (*trace).endpos[1 as libc::c_int as usize] =
        *start.offset(1 as libc::c_int as isize) +
            (*trace).fraction *
                (*end.offset(1 as libc::c_int as isize) -
                     *start.offset(1 as libc::c_int as isize));
    (*trace).endpos[2 as libc::c_int as usize] =
        *start.offset(2 as libc::c_int as isize) +
            (*trace).fraction *
                (*end.offset(2 as libc::c_int as isize) -
                     *start.offset(2 as libc::c_int as isize));
    return (*trace).fraction;
}
unsafe extern "C" fn pfnTraceTexture(mut ground: libc::c_int,
                                     mut vstart: *mut libc::c_float,
                                     mut vend: *mut libc::c_float)
 -> *const libc::c_char {
    let mut pe: *mut physent_t = 0 as *mut physent_t;
    if ground < 0 as libc::c_int || ground >= (*svgame.pmove).numphysent {
        return 0 as *const libc::c_char
    }
    pe =
        &mut *(*svgame.pmove).physents.as_mut_ptr().offset(ground as isize) as
            *mut physent_t;
    return PM_TraceTexture(pe, vstart, vend);
}
unsafe extern "C" fn pfnPlaySound(mut channel: libc::c_int,
                                  mut sample: *const libc::c_char,
                                  mut volume: libc::c_float,
                                  mut attenuation: libc::c_float,
                                  mut fFlags: libc::c_int,
                                  mut pitch: libc::c_int) {
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    ent = SV_EdictNum((*svgame.pmove).player_index + 1 as libc::c_int);
    if SV_CheckEdict(ent,
                     b"../engine/server/sv_pmove.c\x00" as *const u8 as
                         *const libc::c_char, 511 as libc::c_int) as u64 == 0
       {
        return
    }
    SV_StartSound(ent, channel, sample, volume, attenuation,
                  fFlags | (1 as libc::c_int) << 11 as libc::c_int, pitch);
}
unsafe extern "C" fn pfnPlaybackEventFull(mut flags: libc::c_int,
                                          mut clientindex: libc::c_int,
                                          mut eventindex: word,
                                          mut delay: libc::c_float,
                                          mut origin: *mut libc::c_float,
                                          mut angles: *mut libc::c_float,
                                          mut fparam1: libc::c_float,
                                          mut fparam2: libc::c_float,
                                          mut iparam1: libc::c_int,
                                          mut iparam2: libc::c_int,
                                          mut bparam1: libc::c_int,
                                          mut bparam2: libc::c_int) {
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    ent = SV_EdictNum(clientindex + 1 as libc::c_int);
    if SV_CheckEdict(ent,
                     b"../engine/server/sv_pmove.c\x00" as *const u8 as
                         *const libc::c_char, 522 as libc::c_int) as u64 == 0
       {
        return
    }
    if host.type_0 == HOST_DEDICATED as libc::c_int as libc::c_uint {
        flags |= (1 as libc::c_int) << 0 as libc::c_int
    }
    SV_PlaybackEventFull(flags, ent, eventindex, delay, origin, angles,
                         fparam1, fparam2, iparam1, iparam2, bparam1,
                         bparam2);
}
unsafe extern "C" fn pfnPlayerTraceEx(mut start: *mut libc::c_float,
                                      mut end: *mut libc::c_float,
                                      mut traceFlags: libc::c_int,
                                      mut pmFilter: pfnIgnore) -> pmtrace_t {
    return PM_PlayerTraceExt(svgame.pmove, start, end, traceFlags,
                             (*svgame.pmove).numphysent,
                             (*svgame.pmove).physents.as_mut_ptr(),
                             -(1 as libc::c_int), pmFilter);
}
unsafe extern "C" fn pfnTestPlayerPositionEx(mut pos: *mut libc::c_float,
                                             mut ptrace: *mut pmtrace_t,
                                             mut pmFilter: pfnIgnore)
 -> libc::c_int {
    return PM_TestPlayerPosition(svgame.pmove, pos, ptrace, pmFilter);
}
unsafe extern "C" fn pfnTraceLineEx(mut start: *mut libc::c_float,
                                    mut end: *mut libc::c_float,
                                    mut flags: libc::c_int,
                                    mut usehull: libc::c_int,
                                    mut pmFilter: pfnIgnore)
 -> *mut pmtrace_t {
    static mut tr: pmtrace_t =
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
    let mut old_usehull: libc::c_int = 0;
    old_usehull = (*svgame.pmove).usehull;
    (*svgame.pmove).usehull = usehull;
    match flags {
        0 => {
            tr =
                PM_PlayerTraceExt(svgame.pmove, start, end, 0 as libc::c_int,
                                  (*svgame.pmove).numphysent,
                                  (*svgame.pmove).physents.as_mut_ptr(),
                                  -(1 as libc::c_int), pmFilter)
        }
        1 => {
            tr =
                PM_PlayerTraceExt(svgame.pmove, start, end, 0 as libc::c_int,
                                  (*svgame.pmove).numvisent,
                                  (*svgame.pmove).visents.as_mut_ptr(),
                                  -(1 as libc::c_int), pmFilter)
        }
        _ => { }
    }
    (*svgame.pmove).usehull = old_usehull;
    return &mut tr;
}
unsafe extern "C" fn pfnTraceSurface(mut ground: libc::c_int,
                                     mut vstart: *mut libc::c_float,
                                     mut vend: *mut libc::c_float)
 -> *mut msurface_s {
    let mut pe: *mut physent_t = 0 as *mut physent_t;
    if ground < 0 as libc::c_int || ground >= (*svgame.pmove).numphysent {
        return 0 as *mut msurface_s
    }
    pe =
        &mut *(*svgame.pmove).physents.as_mut_ptr().offset(ground as isize) as
            *mut physent_t;
    return PM_TraceSurface(pe, vstart, vend);
}
/*
===============
SV_InitClientMove

===============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_InitClientMove() {
    let mut i: libc::c_int = 0;
    Pmove_Init();
    (*svgame.pmove).server = true_0;
    (*svgame.pmove).movevars = &mut svgame.movevars;
    (*svgame.pmove).runfuncs = false_0;
    // enumerate client hulls
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if svgame.dllFuncs.pfnGetHullBounds.expect("non-null function pointer")(i,
                                                                                host.player_mins[i
                                                                                                     as
                                                                                                     usize].as_mut_ptr(),
                                                                                host.player_maxs[i
                                                                                                     as
                                                                                                     usize].as_mut_ptr())
               != 0 {
            Con_Reportf(b"SV: hull%i, player_mins: %g %g %g, player_maxs: %g %g %g\n\x00"
                            as *const u8 as *const libc::c_char, i,
                        host.player_mins[i as
                                             usize][0 as libc::c_int as usize]
                            as libc::c_double,
                        host.player_mins[i as
                                             usize][1 as libc::c_int as usize]
                            as libc::c_double,
                        host.player_mins[i as
                                             usize][2 as libc::c_int as usize]
                            as libc::c_double,
                        host.player_maxs[i as
                                             usize][0 as libc::c_int as usize]
                            as libc::c_double,
                        host.player_maxs[i as
                                             usize][1 as libc::c_int as usize]
                            as libc::c_double,
                        host.player_maxs[i as
                                             usize][2 as libc::c_int as usize]
                            as libc::c_double);
        }
        i += 1
    }
    memcpy((*svgame.pmove).player_mins.as_mut_ptr() as *mut libc::c_void,
           host.player_mins.as_mut_ptr() as *const libc::c_void,
           ::std::mem::size_of::<[vec3_t; 4]>() as libc::c_ulong);
    memcpy((*svgame.pmove).player_maxs.as_mut_ptr() as *mut libc::c_void,
           host.player_maxs.as_mut_ptr() as *const libc::c_void,
           ::std::mem::size_of::<[vec3_t; 4]>() as libc::c_ulong);
    // common utilities
    (*svgame.pmove).PM_Info_ValueForKey =
        Some(Info_ValueForKey as
                 unsafe extern "C" fn(_: *const libc::c_char,
                                      _: *const libc::c_char)
                     -> *const libc::c_char);
    (*svgame.pmove).PM_Particle =
        Some(pfnParticle as
                 unsafe extern "C" fn(_: *const libc::c_float, _: libc::c_int,
                                      _: libc::c_float, _: libc::c_int,
                                      _: libc::c_int) -> ());
    (*svgame.pmove).PM_TestPlayerPosition =
        Some(pfnTestPlayerPosition as
                 unsafe extern "C" fn(_: *mut libc::c_float,
                                      _: *mut pmtrace_t) -> libc::c_int);
    (*svgame.pmove).Con_NPrintf =
        Some(Con_NPrintf as
                 unsafe extern "C" fn(_: libc::c_int, _: *const libc::c_char,
                                      _: ...) -> ());
    (*svgame.pmove).Con_DPrintf =
        Some(Con_DPrintf as
                 unsafe extern "C" fn(_: *const libc::c_char, _: ...) -> ());
    (*svgame.pmove).Con_Printf =
        Some(Con_Printf as
                 unsafe extern "C" fn(_: *const libc::c_char, _: ...) -> ());
    (*svgame.pmove).Sys_FloatTime =
        Some(Sys_DoubleTime as unsafe extern "C" fn() -> libc::c_double);
    (*svgame.pmove).PM_StuckTouch =
        Some(pfnStuckTouch as
                 unsafe extern "C" fn(_: libc::c_int, _: *mut pmtrace_t)
                     -> ());
    (*svgame.pmove).PM_PointContents =
        Some(pfnPointContents as
                 unsafe extern "C" fn(_: *mut libc::c_float,
                                      _: *mut libc::c_int) -> libc::c_int);
    (*svgame.pmove).PM_TruePointContents =
        Some(pfnTruePointContents as
                 unsafe extern "C" fn(_: *mut libc::c_float) -> libc::c_int);
    (*svgame.pmove).PM_HullPointContents =
        Some(pfnHullPointContents as
                 unsafe extern "C" fn(_: *mut hull_s, _: libc::c_int,
                                      _: *mut libc::c_float) -> libc::c_int);
    (*svgame.pmove).PM_PlayerTrace =
        Some(pfnPlayerTrace as
                 unsafe extern "C" fn(_: *mut libc::c_float,
                                      _: *mut libc::c_float, _: libc::c_int,
                                      _: libc::c_int) -> pmtrace_t);
    (*svgame.pmove).PM_TraceLine =
        Some(pfnTraceLine as
                 unsafe extern "C" fn(_: *mut libc::c_float,
                                      _: *mut libc::c_float, _: libc::c_int,
                                      _: libc::c_int, _: libc::c_int)
                     -> *mut pmtrace_t);
    (*svgame.pmove).RandomLong =
        Some(COM_RandomLong as
                 unsafe extern "C" fn(_: libc::c_int, _: libc::c_int)
                     -> libc::c_int);
    (*svgame.pmove).RandomFloat =
        Some(COM_RandomFloat as
                 unsafe extern "C" fn(_: libc::c_float, _: libc::c_float)
                     -> libc::c_float);
    (*svgame.pmove).PM_GetModelType =
        Some(pfnGetModelType as
                 unsafe extern "C" fn(_: *mut model_t) -> libc::c_int);
    (*svgame.pmove).PM_GetModelBounds =
        Some(pfnGetModelBounds as
                 unsafe extern "C" fn(_: *mut model_t, _: *mut libc::c_float,
                                      _: *mut libc::c_float) -> ());
    (*svgame.pmove).PM_HullForBsp =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_: *mut physent_t,
                                                            _:
                                                                *mut libc::c_float)
                                           ->
                                               *mut libc::c_void>>(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                                           *mut physent_t,
                                                                                                                       _:
                                                                                                                           *mut libc::c_float)
                                                                                                      ->
                                                                                                          *mut hull_t>,
                                                                                           *mut libc::c_void>(Some(pfnHullForBsp
                                                                                                                       as
                                                                                                                       unsafe extern "C" fn(_:
                                                                                                                                                *mut physent_t,
                                                                                                                                            _:
                                                                                                                                                *mut libc::c_float)
                                                                                                                           ->
                                                                                                                               *mut hull_t)));
    (*svgame.pmove).PM_TraceModel =
        Some(pfnTraceModel as
                 unsafe extern "C" fn(_: *mut physent_t,
                                      _: *mut libc::c_float,
                                      _: *mut libc::c_float, _: *mut trace_t)
                     -> libc::c_float);
    (*svgame.pmove).COM_FileSize =
        Some(COM_FileSize as
                 unsafe extern "C" fn(_: *const libc::c_char) -> libc::c_int);
    (*svgame.pmove).COM_LoadFile =
        Some(COM_LoadFile as
                 unsafe extern "C" fn(_: *const libc::c_char, _: libc::c_int,
                                      _: *mut libc::c_int) -> *mut byte);
    (*svgame.pmove).COM_FreeFile =
        Some(COM_FreeFile as
                 unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    (*svgame.pmove).memfgets =
        Some(COM_MemFgets as
                 unsafe extern "C" fn(_: *mut byte, _: libc::c_int,
                                      _: *mut libc::c_int,
                                      _: *mut libc::c_char, _: libc::c_int)
                     -> *mut libc::c_char);
    (*svgame.pmove).PM_PlaySound =
        Some(pfnPlaySound as
                 unsafe extern "C" fn(_: libc::c_int, _: *const libc::c_char,
                                      _: libc::c_float, _: libc::c_float,
                                      _: libc::c_int, _: libc::c_int) -> ());
    (*svgame.pmove).PM_TraceTexture =
        Some(pfnTraceTexture as
                 unsafe extern "C" fn(_: libc::c_int, _: *mut libc::c_float,
                                      _: *mut libc::c_float)
                     -> *const libc::c_char);
    (*svgame.pmove).PM_PlaybackEventFull =
        Some(pfnPlaybackEventFull as
                 unsafe extern "C" fn(_: libc::c_int, _: libc::c_int, _: word,
                                      _: libc::c_float, _: *mut libc::c_float,
                                      _: *mut libc::c_float, _: libc::c_float,
                                      _: libc::c_float, _: libc::c_int,
                                      _: libc::c_int, _: libc::c_int,
                                      _: libc::c_int) -> ());
    (*svgame.pmove).PM_PlayerTraceEx =
        Some(pfnPlayerTraceEx as
                 unsafe extern "C" fn(_: *mut libc::c_float,
                                      _: *mut libc::c_float, _: libc::c_int,
                                      _: pfnIgnore) -> pmtrace_t);
    (*svgame.pmove).PM_TestPlayerPositionEx =
        Some(pfnTestPlayerPositionEx as
                 unsafe extern "C" fn(_: *mut libc::c_float,
                                      _: *mut pmtrace_t, _: pfnIgnore)
                     -> libc::c_int);
    (*svgame.pmove).PM_TraceLineEx =
        Some(pfnTraceLineEx as
                 unsafe extern "C" fn(_: *mut libc::c_float,
                                      _: *mut libc::c_float, _: libc::c_int,
                                      _: libc::c_int, _: pfnIgnore)
                     -> *mut pmtrace_t);
    (*svgame.pmove).PM_TraceSurface =
        Some(pfnTraceSurface as
                 unsafe extern "C" fn(_: libc::c_int, _: *mut libc::c_float,
                                      _: *mut libc::c_float)
                     -> *mut msurface_s);
    // initalize pmove
    svgame.dllFuncs.pfnPM_Init.expect("non-null function pointer")(svgame.pmove);
}
unsafe extern "C" fn PM_CheckMovingGround(mut ent: *mut edict_t,
                                          mut frametime: libc::c_float) {
    if svgame.physFuncs.SV_UpdatePlayerBaseVelocity.is_some() {
        svgame.physFuncs.SV_UpdatePlayerBaseVelocity.expect("non-null function pointer")(ent);
    } else { SV_UpdateBaseVelocity(ent); }
    if (*ent).v.flags as libc::c_uint &
           (1 as libc::c_uint) << 22 as libc::c_int == 0 {
        // apply momentum (add in half of the previous frame of velocity first)
        (*ent).v.velocity[0 as libc::c_int as usize] =
            (*ent).v.velocity[0 as libc::c_int as usize] +
                (1.0f32 + frametime * 0.5f32) *
                    (*ent).v.basevelocity[0 as libc::c_int as
                                              usize]; // reset hull
        (*ent).v.velocity[1 as libc::c_int as usize] =
            (*ent).v.velocity[1 as libc::c_int as usize] +
                (1.0f32 + frametime * 0.5f32) *
                    (*ent).v.basevelocity[1 as libc::c_int as
                                              usize]; // not used by PM_ code
        (*ent).v.velocity[2 as libc::c_int as usize] =
            (*ent).v.velocity[2 as libc::c_int as usize] +
                (1.0f32 + frametime * 0.5f32) *
                    (*ent).v.basevelocity[2 as libc::c_int as
                                              usize]; // spectator physic all execute on client
        (*ent).v.basevelocity[2 as libc::c_int as usize] =
            0 as libc::c_int as vec_t; // setup current cmds
        (*ent).v.basevelocity[1 as libc::c_int as usize] =
            (*ent).v.basevelocity[2 as libc::c_int as usize];
        (*ent).v.basevelocity[0 as libc::c_int as usize] =
            (*ent).v.basevelocity[1 as libc::c_int as usize]
    }
    (*ent).v.flags =
        ((*ent).v.flags as libc::c_uint &
             !((1 as libc::c_uint) << 22 as libc::c_int)) as libc::c_int;
}
unsafe extern "C" fn SV_SetupPMove(mut pmove: *mut playermove_t,
                                   mut cl: *mut sv_client_t,
                                   mut ucmd: *mut usercmd_t,
                                   mut physinfo: *const libc::c_char) {
    let mut absmin: vec3_t = [0.; 3];
    let mut absmax: vec3_t = [0.; 3];
    let mut clent: *mut edict_t = (*cl).edict;
    let mut i: libc::c_int = 0;
    (*svgame.globals).frametime =
        (*ucmd).msec as libc::c_int as libc::c_float * 0.001f32;
    (*pmove).player_index =
        clent.wrapping_offset_from(svgame.edicts) as libc::c_long as
            libc::c_int - 1 as libc::c_int;
    (*pmove).multiplayer =
        if svs.maxclients > 1 as libc::c_int {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int } as qboolean;
    (*pmove).time = ((*cl).timebase * 1000.0f64) as libc::c_float;
    (*pmove).origin[0 as libc::c_int as usize] =
        (*clent).v.origin[0 as libc::c_int as usize];
    (*pmove).origin[1 as libc::c_int as usize] =
        (*clent).v.origin[1 as libc::c_int as usize];
    (*pmove).origin[2 as libc::c_int as usize] =
        (*clent).v.origin[2 as libc::c_int as usize];
    (*pmove).angles[0 as libc::c_int as usize] =
        (*clent).v.v_angle[0 as libc::c_int as usize];
    (*pmove).angles[1 as libc::c_int as usize] =
        (*clent).v.v_angle[1 as libc::c_int as usize];
    (*pmove).angles[2 as libc::c_int as usize] =
        (*clent).v.v_angle[2 as libc::c_int as usize];
    (*pmove).oldangles[0 as libc::c_int as usize] =
        (*clent).v.v_angle[0 as libc::c_int as usize];
    (*pmove).oldangles[1 as libc::c_int as usize] =
        (*clent).v.v_angle[1 as libc::c_int as usize];
    (*pmove).oldangles[2 as libc::c_int as usize] =
        (*clent).v.v_angle[2 as libc::c_int as usize];
    (*pmove).velocity[0 as libc::c_int as usize] =
        (*clent).v.velocity[0 as libc::c_int as usize];
    (*pmove).velocity[1 as libc::c_int as usize] =
        (*clent).v.velocity[1 as libc::c_int as usize];
    (*pmove).velocity[2 as libc::c_int as usize] =
        (*clent).v.velocity[2 as libc::c_int as usize];
    (*pmove).basevelocity[0 as libc::c_int as usize] =
        (*clent).v.basevelocity[0 as libc::c_int as usize];
    (*pmove).basevelocity[1 as libc::c_int as usize] =
        (*clent).v.basevelocity[1 as libc::c_int as usize];
    (*pmove).basevelocity[2 as libc::c_int as usize] =
        (*clent).v.basevelocity[2 as libc::c_int as usize];
    (*pmove).view_ofs[0 as libc::c_int as usize] =
        (*clent).v.view_ofs[0 as libc::c_int as usize];
    (*pmove).view_ofs[1 as libc::c_int as usize] =
        (*clent).v.view_ofs[1 as libc::c_int as usize];
    (*pmove).view_ofs[2 as libc::c_int as usize] =
        (*clent).v.view_ofs[2 as libc::c_int as usize];
    (*pmove).movedir[0 as libc::c_int as usize] =
        (*clent).v.movedir[0 as libc::c_int as usize];
    (*pmove).movedir[1 as libc::c_int as usize] =
        (*clent).v.movedir[1 as libc::c_int as usize];
    (*pmove).movedir[2 as libc::c_int as usize] =
        (*clent).v.movedir[2 as libc::c_int as usize];
    (*pmove).flDuckTime = (*clent).v.flDuckTime as libc::c_float;
    (*pmove).bInDuck = (*clent).v.bInDuck as qboolean;
    (*pmove).usehull =
        if (*clent).v.flags as libc::c_uint &
               (1 as libc::c_uint) << 14 as libc::c_int != 0 {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    (*pmove).flTimeStepSound = (*clent).v.flTimeStepSound;
    (*pmove).iStepLeft = (*clent).v.iStepLeft;
    (*pmove).flFallVelocity = (*clent).v.flFallVelocity;
    (*pmove).flSwimTime = (*clent).v.flSwimTime as libc::c_float;
    (*pmove).punchangle[0 as libc::c_int as usize] =
        (*clent).v.punchangle[0 as libc::c_int as usize];
    (*pmove).punchangle[1 as libc::c_int as usize] =
        (*clent).v.punchangle[1 as libc::c_int as usize];
    (*pmove).punchangle[2 as libc::c_int as usize] =
        (*clent).v.punchangle[2 as libc::c_int as usize];
    (*pmove).flSwimTime = (*clent).v.flSwimTime as libc::c_float;
    (*pmove).flNextPrimaryAttack = 0.0f32;
    (*pmove).effects = (*clent).v.effects;
    (*pmove).flags = (*clent).v.flags;
    (*pmove).gravity = (*clent).v.gravity;
    (*pmove).friction = (*clent).v.friction;
    (*pmove).oldbuttons = (*clent).v.oldbuttons;
    (*pmove).waterjumptime = (*clent).v.teleport_time;
    (*pmove).dead =
        if (*clent).v.health <= 0.0f32 {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int } as qboolean;
    (*pmove).deadflag = (*clent).v.deadflag;
    (*pmove).spectator = 0 as libc::c_int;
    (*pmove).movetype = (*clent).v.movetype;
    if (*pmove).multiplayer as u64 != 0 {
        (*pmove).onground = -(1 as libc::c_int)
    }
    (*pmove).waterlevel = (*clent).v.waterlevel;
    (*pmove).watertype = (*clent).v.watertype;
    (*pmove).maxspeed = svgame.movevars.maxspeed;
    (*pmove).clientmaxspeed = (*clent).v.maxspeed;
    (*pmove).iuser1 = (*clent).v.iuser1;
    (*pmove).iuser2 = (*clent).v.iuser2;
    (*pmove).iuser3 = (*clent).v.iuser3;
    (*pmove).iuser4 = (*clent).v.iuser4;
    (*pmove).fuser1 = (*clent).v.fuser1;
    (*pmove).fuser2 = (*clent).v.fuser2;
    (*pmove).fuser3 = (*clent).v.fuser3;
    (*pmove).fuser4 = (*clent).v.fuser4;
    (*pmove).vuser1[0 as libc::c_int as usize] =
        (*clent).v.vuser1[0 as libc::c_int as usize];
    (*pmove).vuser1[1 as libc::c_int as usize] =
        (*clent).v.vuser1[1 as libc::c_int as usize];
    (*pmove).vuser1[2 as libc::c_int as usize] =
        (*clent).v.vuser1[2 as libc::c_int as usize];
    (*pmove).vuser2[0 as libc::c_int as usize] =
        (*clent).v.vuser2[0 as libc::c_int as usize];
    (*pmove).vuser2[1 as libc::c_int as usize] =
        (*clent).v.vuser2[1 as libc::c_int as usize];
    (*pmove).vuser2[2 as libc::c_int as usize] =
        (*clent).v.vuser2[2 as libc::c_int as usize];
    (*pmove).vuser3[0 as libc::c_int as usize] =
        (*clent).v.vuser3[0 as libc::c_int as usize];
    (*pmove).vuser3[1 as libc::c_int as usize] =
        (*clent).v.vuser3[1 as libc::c_int as usize];
    (*pmove).vuser3[2 as libc::c_int as usize] =
        (*clent).v.vuser3[2 as libc::c_int as usize];
    (*pmove).vuser4[0 as libc::c_int as usize] =
        (*clent).v.vuser4[0 as libc::c_int as usize];
    (*pmove).vuser4[1 as libc::c_int as usize] =
        (*clent).v.vuser4[1 as libc::c_int as usize];
    (*pmove).vuser4[2 as libc::c_int as usize] =
        (*clent).v.vuser4[2 as libc::c_int as usize];
    (*pmove).cmd = *ucmd;
    (*pmove).runfuncs = true_0;
    Q_strncpy((*pmove).physinfo.as_mut_ptr(), physinfo,
              256 as libc::c_int as size_t);
    // setup physents
    (*pmove).numvisent = 0 as libc::c_int; // always have world
    (*pmove).numphysent = 0 as libc::c_int;
    (*pmove).nummoveent = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        absmin[i as usize] = (*clent).v.origin[i as usize] - 256.0f32;
        absmax[i as usize] = (*clent).v.origin[i as usize] + 256.0f32;
        i += 1
    }
    SV_CopyEdictToPhysEnt(&mut *(*svgame.pmove).physents.as_mut_ptr().offset(0
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 isize),
                          &mut *svgame.edicts.offset(0 as libc::c_int as
                                                         isize));
    (*svgame.pmove).visents[0 as libc::c_int as usize] =
        (*svgame.pmove).physents[0 as libc::c_int as usize];
    (*svgame.pmove).numphysent = 1 as libc::c_int;
    (*svgame.pmove).numvisent = 1 as libc::c_int;
    SV_AddLinksToPmove(sv_areanodes.as_mut_ptr(),
                       absmin.as_mut_ptr() as *const vec_t,
                       absmax.as_mut_ptr() as *const vec_t);
    SV_AddLaddersToPmove(sv_areanodes.as_mut_ptr(),
                         absmin.as_mut_ptr() as *const vec_t,
                         absmax.as_mut_ptr() as *const vec_t);
}
unsafe extern "C" fn SV_FinishPMove(mut pmove: *mut playermove_t,
                                    mut cl: *mut sv_client_t) {
    let mut clent: *mut edict_t = (*cl).edict;
    (*clent).v.teleport_time = (*pmove).waterjumptime;
    (*clent).v.origin[0 as libc::c_int as usize] =
        (*pmove).origin[0 as libc::c_int as usize];
    (*clent).v.origin[1 as libc::c_int as usize] =
        (*pmove).origin[1 as libc::c_int as usize];
    (*clent).v.origin[2 as libc::c_int as usize] =
        (*pmove).origin[2 as libc::c_int as usize];
    (*clent).v.view_ofs[0 as libc::c_int as usize] =
        (*pmove).view_ofs[0 as libc::c_int as usize];
    (*clent).v.view_ofs[1 as libc::c_int as usize] =
        (*pmove).view_ofs[1 as libc::c_int as usize];
    (*clent).v.view_ofs[2 as libc::c_int as usize] =
        (*pmove).view_ofs[2 as libc::c_int as usize];
    (*clent).v.velocity[0 as libc::c_int as usize] =
        (*pmove).velocity[0 as libc::c_int as usize];
    (*clent).v.velocity[1 as libc::c_int as usize] =
        (*pmove).velocity[1 as libc::c_int as usize];
    (*clent).v.velocity[2 as libc::c_int as usize] =
        (*pmove).velocity[2 as libc::c_int as usize];
    (*clent).v.basevelocity[0 as libc::c_int as usize] =
        (*pmove).basevelocity[0 as libc::c_int as usize];
    (*clent).v.basevelocity[1 as libc::c_int as usize] =
        (*pmove).basevelocity[1 as libc::c_int as usize];
    (*clent).v.basevelocity[2 as libc::c_int as usize] =
        (*pmove).basevelocity[2 as libc::c_int as usize];
    (*clent).v.punchangle[0 as libc::c_int as usize] =
        (*pmove).punchangle[0 as libc::c_int as usize];
    (*clent).v.punchangle[1 as libc::c_int as usize] =
        (*pmove).punchangle[1 as libc::c_int as usize];
    (*clent).v.punchangle[2 as libc::c_int as usize] =
        (*pmove).punchangle[2 as libc::c_int as usize];
    (*clent).v.movedir[0 as libc::c_int as usize] =
        (*pmove).movedir[0 as libc::c_int as usize];
    (*clent).v.movedir[1 as libc::c_int as usize] =
        (*pmove).movedir[1 as libc::c_int as usize];
    (*clent).v.movedir[2 as libc::c_int as usize] =
        (*pmove).movedir[2 as libc::c_int as usize];
    (*clent).v.flTimeStepSound = (*pmove).flTimeStepSound;
    (*clent).v.flFallVelocity = (*pmove).flFallVelocity;
    (*clent).v.oldbuttons = (*pmove).cmd.buttons as libc::c_int;
    (*clent).v.waterlevel = (*pmove).waterlevel;
    (*clent).v.watertype = (*pmove).watertype;
    (*clent).v.maxspeed = (*pmove).clientmaxspeed;
    (*clent).v.flDuckTime = (*pmove).flDuckTime as libc::c_int;
    (*clent).v.flSwimTime = (*pmove).flSwimTime as libc::c_int;
    (*clent).v.iStepLeft = (*pmove).iStepLeft;
    (*clent).v.movetype = (*pmove).movetype;
    (*clent).v.friction = (*pmove).friction;
    (*clent).v.deadflag = (*pmove).deadflag;
    (*clent).v.effects = (*pmove).effects;
    (*clent).v.bInDuck = (*pmove).bInDuck as libc::c_int;
    (*clent).v.flags = (*pmove).flags;
    // copy back user variables
    (*clent).v.iuser1 = (*pmove).iuser1;
    (*clent).v.iuser2 = (*pmove).iuser2;
    (*clent).v.iuser3 = (*pmove).iuser3;
    (*clent).v.iuser4 = (*pmove).iuser4;
    (*clent).v.fuser1 = (*pmove).fuser1;
    (*clent).v.fuser2 = (*pmove).fuser2;
    (*clent).v.fuser3 = (*pmove).fuser3;
    (*clent).v.fuser4 = (*pmove).fuser4;
    (*clent).v.vuser1[0 as libc::c_int as usize] =
        (*pmove).vuser1[0 as libc::c_int as usize];
    (*clent).v.vuser1[1 as libc::c_int as usize] =
        (*pmove).vuser1[1 as libc::c_int as usize];
    (*clent).v.vuser1[2 as libc::c_int as usize] =
        (*pmove).vuser1[2 as libc::c_int as usize];
    (*clent).v.vuser2[0 as libc::c_int as usize] =
        (*pmove).vuser2[0 as libc::c_int as usize];
    (*clent).v.vuser2[1 as libc::c_int as usize] =
        (*pmove).vuser2[1 as libc::c_int as usize];
    (*clent).v.vuser2[2 as libc::c_int as usize] =
        (*pmove).vuser2[2 as libc::c_int as usize];
    (*clent).v.vuser3[0 as libc::c_int as usize] =
        (*pmove).vuser3[0 as libc::c_int as usize];
    (*clent).v.vuser3[1 as libc::c_int as usize] =
        (*pmove).vuser3[1 as libc::c_int as usize];
    (*clent).v.vuser3[2 as libc::c_int as usize] =
        (*pmove).vuser3[2 as libc::c_int as usize];
    (*clent).v.vuser4[0 as libc::c_int as usize] =
        (*pmove).vuser4[0 as libc::c_int as usize];
    (*clent).v.vuser4[1 as libc::c_int as usize] =
        (*pmove).vuser4[1 as libc::c_int as usize];
    (*clent).v.vuser4[2 as libc::c_int as usize] =
        (*pmove).vuser4[2 as libc::c_int as usize];
    if (*pmove).onground == -(1 as libc::c_int) {
        (*clent).v.flags =
            ((*clent).v.flags as libc::c_uint &
                 !((1 as libc::c_uint) << 9 as libc::c_int)) as libc::c_int
    } else if (*pmove).onground >= 0 as libc::c_int &&
                  (*pmove).onground < (*pmove).numphysent {
        (*clent).v.flags =
            ((*clent).v.flags as libc::c_uint |
                 (1 as libc::c_uint) << 9 as libc::c_int) as libc::c_int;
        (*clent).v.groundentity =
            SV_EdictNum((*pmove).physents[(*pmove).onground as usize].info)
    }
    // angles
	// show 1/3 the pitch angle and all the roll angle
    if (*clent).v.fixangle == 0 {
        (*clent).v.v_angle[0 as libc::c_int as usize] =
            (*pmove).angles[0 as libc::c_int as usize];
        (*clent).v.v_angle[1 as libc::c_int as usize] =
            (*pmove).angles[1 as libc::c_int as usize];
        (*clent).v.v_angle[2 as libc::c_int as usize] =
            (*pmove).angles[2 as libc::c_int as usize];
        (*clent).v.angles[0 as libc::c_int as usize] =
            -((*clent).v.v_angle[0 as libc::c_int as usize] / 3.0f32);
        (*clent).v.angles[2 as libc::c_int as usize] =
            (*clent).v.v_angle[2 as libc::c_int as usize];
        (*clent).v.angles[1 as libc::c_int as usize] =
            (*clent).v.v_angle[1 as libc::c_int as usize]
    }
    SV_SetMinMaxSize(clent,
                     (*pmove).player_mins[(*pmove).usehull as
                                              usize].as_mut_ptr(),
                     (*pmove).player_maxs[(*pmove).usehull as
                                              usize].as_mut_ptr(), false_0);
    // all next calls ignore footstep sounds
    (*pmove).runfuncs = false_0; // pushed too much ?
}
#[no_mangle]
pub unsafe extern "C" fn SV_FindEntInPack(mut index: libc::c_int,
                                          mut frame: *mut client_frame_t)
 -> *mut entity_state_t {
    let mut state: *mut entity_state_t =
        0 as *mut entity_state_t; // they didn't actually move.
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*frame).num_entities {
        state =
            &mut *svs.packet_entities.offset((((*frame).first_entity + i) %
                                                  svs.num_client_entities) as
                                                 isize) as
                *mut entity_state_t;
        if (*state).number == index { return state }
        i += 1
    }
    return 0 as *mut entity_state_t;
}
#[no_mangle]
pub unsafe extern "C" fn SV_UnlagCheckTeleport(mut old_pos: *mut vec_t,
                                               mut new_pos: *mut vec_t)
 -> qboolean {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if __tg_fabs(*old_pos.offset(i as isize) -
                         *new_pos.offset(i as isize)) > 64.0f32 {
            return true_0
        }
        i += 1
    }
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn SV_SetupMoveInterpolant(mut cl: *mut sv_client_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut clientnum: libc::c_int = 0;
    let mut finalpush: libc::c_float = 0.;
    let mut lerp_msec: libc::c_float = 0.;
    let mut latency: libc::c_float = 0.;
    let mut lerpFrac: libc::c_float = 0.;
    let mut frame: *mut client_frame_t = 0 as *mut client_frame_t;
    let mut frame2: *mut client_frame_t = 0 as *mut client_frame_t;
    let mut state: *mut entity_state_t = 0 as *mut entity_state_t;
    let mut lerpstate: *mut entity_state_t = 0 as *mut entity_state_t;
    let mut curpos: vec3_t = [0.; 3];
    let mut newpos: vec3_t = [0.; 3];
    let mut check: *mut sv_client_t = 0 as *mut sv_client_t;
    let mut lerp: *mut sv_interp_t = 0 as *mut sv_interp_t;
    memset(svgame.interp.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[sv_interp_t; 32]>() as libc::c_ulong);
    has_update = false_0;
    if SV_ShouldUnlagForPlayer(cl) as u64 == 0 { return }
    has_update = true_0;
    i = 0 as libc::c_int;
    check = svs.clients;
    while i < svs.maxclients {
        if !((*check).state as libc::c_uint !=
                 cs_spawned as libc::c_int as libc::c_uint || check == cl) {
            lerp =
                &mut *svgame.interp.as_mut_ptr().offset(i as isize) as
                    *mut sv_interp_t;
            (*lerp).oldpos[0 as libc::c_int as usize] =
                (*(*check).edict).v.origin[0 as libc::c_int as usize];
            (*lerp).oldpos[1 as libc::c_int as usize] =
                (*(*check).edict).v.origin[1 as libc::c_int as usize];
            (*lerp).oldpos[2 as libc::c_int as usize] =
                (*(*check).edict).v.origin[2 as libc::c_int as usize];
            (*lerp).mins[0 as libc::c_int as usize] =
                (*(*check).edict).v.absmin[0 as libc::c_int as usize];
            (*lerp).mins[1 as libc::c_int as usize] =
                (*(*check).edict).v.absmin[1 as libc::c_int as usize];
            (*lerp).mins[2 as libc::c_int as usize] =
                (*(*check).edict).v.absmin[2 as libc::c_int as usize];
            (*lerp).maxs[0 as libc::c_int as usize] =
                (*(*check).edict).v.absmax[0 as libc::c_int as usize];
            (*lerp).maxs[1 as libc::c_int as usize] =
                (*(*check).edict).v.absmax[1 as libc::c_int as usize];
            (*lerp).maxs[2 as libc::c_int as usize] =
                (*(*check).edict).v.absmax[2 as libc::c_int as usize];
            (*lerp).active = true_0
        }
        i += 1;
        check = check.offset(1)
    }
    latency = if (*cl).latency < 1.5f32 { (*cl).latency } else { 1.5f32 };
    if sv_maxunlag.value != 0.0f32 {
        if sv_maxunlag.value < 0.0f32 {
            Cvar_SetValue(b"sv_maxunlag\x00" as *const u8 as
                              *const libc::c_char, 0.0f32);
        }
        latency =
            if latency < sv_maxunlag.value {
                latency
            } else { sv_maxunlag.value }
    }
    lerp_msec =
        (*cl).lastcmd.lerp_msec as libc::c_int as libc::c_float * 0.001f32;
    if lerp_msec > 0.1f32 { lerp_msec = 0.1f32 }
    if (lerp_msec as libc::c_double) < (*cl).cl_updaterate {
        lerp_msec = (*cl).cl_updaterate as libc::c_float
    }
    finalpush =
        (host.realtime - latency as libc::c_double -
             lerp_msec as libc::c_double +
             sv_unlagpush.value as libc::c_double) as libc::c_float;
    if finalpush as libc::c_double > host.realtime {
        finalpush = host.realtime as libc::c_float
    }
    frame2 = 0 as *mut client_frame_t;
    frame = frame2;
    i = 0 as libc::c_int;
    while i < SV_UPDATE_BACKUP {
        frame =
            &mut *(*cl).frames.offset(((*cl).netchan.outgoing_sequence.wrapping_sub((i
                                                                                         +
                                                                                         1
                                                                                             as
                                                                                             libc::c_int)
                                                                                        as
                                                                                        libc::c_uint)
                                           &
                                           (SV_UPDATE_BACKUP -
                                                1 as libc::c_int) as
                                               libc::c_uint) as isize) as
                *mut client_frame_t;
        j = 0 as libc::c_int;
        while j < (*frame).num_entities {
            state =
                &mut *svs.packet_entities.offset((((*frame).first_entity + j)
                                                      %
                                                      svs.num_client_entities)
                                                     as isize) as
                    *mut entity_state_t;
            if !((*state).number < 1 as libc::c_int ||
                     (*state).number > svs.maxclients) {
                lerp =
                    &mut *svgame.interp.as_mut_ptr().offset(((*state).number -
                                                                 1 as
                                                                     libc::c_int)
                                                                as isize) as
                        *mut sv_interp_t;
                if !((*lerp).nointerp as u64 != 0) {
                    if (*state).health <= 0 as libc::c_int ||
                           (*state).effects & 32 as libc::c_int != 0 {
                        (*lerp).nointerp = true_0
                    }
                    if (*lerp).firstframe as u64 != 0 {
                        if SV_UnlagCheckTeleport((*state).origin.as_mut_ptr(),
                                                 (*lerp).finalpos.as_mut_ptr())
                               as u64 != 0 {
                            (*lerp).nointerp = true_0
                        }
                    } else { (*lerp).firstframe = true_0 }
                    (*lerp).finalpos[0 as libc::c_int as usize] =
                        (*state).origin[0 as libc::c_int as usize];
                    (*lerp).finalpos[1 as libc::c_int as usize] =
                        (*state).origin[1 as libc::c_int as usize];
                    (*lerp).finalpos[2 as libc::c_int as usize] =
                        (*state).origin[2 as libc::c_int as usize]
                }
            }
            j += 1
        }
        if finalpush as libc::c_double > (*frame).senttime { break ; }
        i += 1;
        frame2 = frame
    }
    if i == SV_UPDATE_BACKUP ||
           finalpush as libc::c_double - (*frame).senttime >
               1.0f32 as libc::c_double {
        memset(svgame.interp.as_mut_ptr() as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<[sv_interp_t; 32]>() as libc::c_ulong);
        has_update = false_0;
        return
    }
    if frame2.is_null() {
        frame2 = frame;
        lerpFrac = 0 as libc::c_int as libc::c_float
    } else if (*frame2).senttime - (*frame).senttime == 0.0f64 {
        lerpFrac = 0 as libc::c_int as libc::c_float
    } else {
        lerpFrac =
            ((finalpush as libc::c_double - (*frame).senttime) /
                 ((*frame2).senttime - (*frame).senttime)) as libc::c_float;
        lerpFrac =
            if lerpFrac >= 0.0f32 {
                if lerpFrac < 1.0f32 { lerpFrac } else { 1.0f32 }
            } else { 0.0f32 }
    }
    i = 0 as libc::c_int;
    while i < (*frame).num_entities {
        state =
            &mut *svs.packet_entities.offset((((*frame).first_entity + i) %
                                                  svs.num_client_entities) as
                                                 isize) as
                *mut entity_state_t;
        if !((*state).number < 1 as libc::c_int ||
                 (*state).number > svs.maxclients) {
            clientnum = (*state).number - 1 as libc::c_int;
            check =
                &mut *svs.clients.offset(clientnum as isize) as
                    *mut sv_client_t;
            if !((*check).state as libc::c_uint !=
                     cs_spawned as libc::c_int as libc::c_uint || check == cl)
               {
                lerp =
                    &mut *svgame.interp.as_mut_ptr().offset(clientnum as
                                                                isize) as
                        *mut sv_interp_t;
                if !((*lerp).active as u64 == 0 ||
                         (*lerp).nointerp as libc::c_uint != 0) {
                    lerpstate = SV_FindEntInPack((*state).number, frame2);
                    if lerpstate.is_null() {
                        curpos[0 as libc::c_int as usize] =
                            (*state).origin[0 as libc::c_int as usize];
                        curpos[1 as libc::c_int as usize] =
                            (*state).origin[1 as libc::c_int as usize];
                        curpos[2 as libc::c_int as usize] =
                            (*state).origin[2 as libc::c_int as usize]
                    } else {
                        newpos[0 as libc::c_int as usize] =
                            (*lerpstate).origin[0 as libc::c_int as usize] -
                                (*state).origin[0 as libc::c_int as usize];
                        newpos[1 as libc::c_int as usize] =
                            (*lerpstate).origin[1 as libc::c_int as usize] -
                                (*state).origin[1 as libc::c_int as usize];
                        newpos[2 as libc::c_int as usize] =
                            (*lerpstate).origin[2 as libc::c_int as usize] -
                                (*state).origin[2 as libc::c_int as usize];
                        curpos[0 as libc::c_int as usize] =
                            (*state).origin[0 as libc::c_int as usize] +
                                lerpFrac * newpos[0 as libc::c_int as usize];
                        curpos[1 as libc::c_int as usize] =
                            (*state).origin[1 as libc::c_int as usize] +
                                lerpFrac * newpos[1 as libc::c_int as usize];
                        curpos[2 as libc::c_int as usize] =
                            (*state).origin[2 as libc::c_int as usize] +
                                lerpFrac * newpos[2 as libc::c_int as usize]
                    }
                    (*lerp).curpos[0 as libc::c_int as usize] =
                        curpos[0 as libc::c_int as usize];
                    (*lerp).curpos[1 as libc::c_int as usize] =
                        curpos[1 as libc::c_int as usize];
                    (*lerp).curpos[2 as libc::c_int as usize] =
                        curpos[2 as libc::c_int as usize];
                    (*lerp).newpos[0 as libc::c_int as usize] =
                        curpos[0 as libc::c_int as usize];
                    (*lerp).newpos[1 as libc::c_int as usize] =
                        curpos[1 as libc::c_int as usize];
                    (*lerp).newpos[2 as libc::c_int as usize] =
                        curpos[2 as libc::c_int as usize];
                    if !(curpos[0 as libc::c_int as usize] ==
                             (*(*check).edict).v.origin[0 as libc::c_int as
                                                            usize] &&
                             curpos[1 as libc::c_int as usize] ==
                                 (*(*check).edict).v.origin[1 as libc::c_int
                                                                as usize] &&
                             curpos[2 as libc::c_int as usize] ==
                                 (*(*check).edict).v.origin[2 as libc::c_int
                                                                as usize]) {
                        (*(*check).edict).v.origin[0 as libc::c_int as usize]
                            = curpos[0 as libc::c_int as usize];
                        (*(*check).edict).v.origin[1 as libc::c_int as usize]
                            = curpos[1 as libc::c_int as usize];
                        (*(*check).edict).v.origin[2 as libc::c_int as usize]
                            = curpos[2 as libc::c_int as usize];
                        SV_LinkEdict((*check).edict, false_0);
                        (*lerp).moving = true_0
                    }
                }
            }
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn SV_RestoreMoveInterpolant(mut cl: *mut sv_client_t) {
    let mut check: *mut sv_client_t = 0 as *mut sv_client_t;
    let mut oldlerp: *mut sv_interp_t = 0 as *mut sv_interp_t;
    let mut i: libc::c_int = 0;
    if has_update as u64 == 0 { has_update = true_0; return }
    if SV_ShouldUnlagForPlayer(cl) as u64 == 0 { return }
    i = 0 as libc::c_int;
    check = svs.clients;
    while i < svs.maxclients {
        if !((*check).state as libc::c_uint !=
                 cs_spawned as libc::c_int as libc::c_uint || check == cl) {
            oldlerp =
                &mut *svgame.interp.as_mut_ptr().offset(i as isize) as
                    *mut sv_interp_t;
            if !(VectorCompareEpsilon((*oldlerp).oldpos.as_mut_ptr() as
                                          *const vec_t,
                                      (*oldlerp).newpos.as_mut_ptr() as
                                          *const vec_t, 0.1f32) as u64 != 0) {
                if !((*oldlerp).moving as u64 == 0 ||
                         (*oldlerp).active as u64 == 0) {
                    if (*oldlerp).curpos[0 as libc::c_int as usize] ==
                           (*(*check).edict).v.origin[0 as libc::c_int as
                                                          usize] &&
                           (*oldlerp).curpos[1 as libc::c_int as usize] ==
                               (*(*check).edict).v.origin[1 as libc::c_int as
                                                              usize] &&
                           (*oldlerp).curpos[2 as libc::c_int as usize] ==
                               (*(*check).edict).v.origin[2 as libc::c_int as
                                                              usize] {
                        (*(*check).edict).v.origin[0 as libc::c_int as usize]
                            = (*oldlerp).oldpos[0 as libc::c_int as usize];
                        (*(*check).edict).v.origin[1 as libc::c_int as usize]
                            = (*oldlerp).oldpos[1 as libc::c_int as usize];
                        (*(*check).edict).v.origin[2 as libc::c_int as usize]
                            = (*oldlerp).oldpos[2 as libc::c_int as usize];
                        SV_LinkEdict((*check).edict, false_0);
                    }
                }
            }
        }
        i += 1;
        check = check.offset(1)
    };
}
/*
===========
SV_RunCmd
===========
*/
#[no_mangle]
pub unsafe extern "C" fn SV_RunCmd(mut cl: *mut sv_client_t,
                                   mut ucmd: *mut usercmd_t,
                                   mut random_seed: libc::c_int) {
    let mut clent: *mut edict_t = 0 as *mut edict_t;
    let mut touch: *mut edict_t = 0 as *mut edict_t;
    let mut frametime: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut oldmsec: libc::c_int = 0;
    let mut pmtrace: *mut pmtrace_t = 0 as *mut pmtrace_t;
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
    let mut oldvel: vec3_t = [0.; 3];
    let mut cmd: usercmd_t =
        usercmd_t{lerp_msec: 0,
                  msec: 0,
                  viewangles: [0.; 3],
                  forwardmove: 0.,
                  sidemove: 0.,
                  upmove: 0.,
                  lightlevel: 0,
                  buttons: 0,
                  impulse: 0,
                  weaponselect: 0,
                  impact_index: 0,
                  impact_position: [0.; 3],};
    clent = (*cl).edict;
    cmd = *ucmd;
    if (*cl).ignorecmdtime > host.realtime {
        (*cl).cmdtime += (*ucmd).msec as libc::c_double / 1000.0f64;
        return
    }
    (*cl).ignorecmdtime = 0.0f64;
    // chop up very long commands
    if cmd.msec as libc::c_int > 50 as libc::c_int {
        oldmsec = (*ucmd).msec as libc::c_int; // save oldangles
        cmd.msec = (oldmsec / 2 as libc::c_int) as byte;
        SV_RunCmd(cl, &mut cmd, random_seed);
        cmd.msec = (oldmsec / 2 as libc::c_int) as byte;
        cmd.impulse = 0 as libc::c_int as byte;
        SV_RunCmd(cl, &mut cmd, random_seed);
        return
    }
    if (*cl).flags & (1 as libc::c_uint) << 7 as libc::c_int == 0 {
        SV_SetupMoveInterpolant(cl);
    }
    svgame.dllFuncs.pfnCmdStart.expect("non-null function pointer")((*cl).edict,
                                                                    ucmd,
                                                                    random_seed
                                                                        as
                                                                        libc::c_uint);
    frametime = (*ucmd).msec as libc::c_double / 1000.0f64;
    (*cl).timebase += frametime;
    (*cl).cmdtime += frametime;
    PM_CheckMovingGround(clent, frametime as libc::c_float);
    (*svgame.pmove).oldangles[0 as libc::c_int as usize] =
        (*clent).v.v_angle[0 as libc::c_int as usize];
    (*svgame.pmove).oldangles[1 as libc::c_int as usize] =
        (*clent).v.v_angle[1 as libc::c_int as usize];
    (*svgame.pmove).oldangles[2 as libc::c_int as usize] =
        (*clent).v.v_angle[2 as libc::c_int as usize];
    if (*clent).v.fixangle == 0 {
        (*clent).v.v_angle[0 as libc::c_int as usize] =
            (*ucmd).viewangles[0 as libc::c_int as usize];
        (*clent).v.v_angle[1 as libc::c_int as usize] =
            (*ucmd).viewangles[1 as libc::c_int as usize];
        (*clent).v.v_angle[2 as libc::c_int as usize] =
            (*ucmd).viewangles[2 as libc::c_int as usize]
    }
    (*clent).v.clbasevelocity[2 as libc::c_int as usize] =
        0 as libc::c_int as vec_t;
    (*clent).v.clbasevelocity[1 as libc::c_int as usize] =
        (*clent).v.clbasevelocity[2 as libc::c_int as usize];
    (*clent).v.clbasevelocity[0 as libc::c_int as usize] =
        (*clent).v.clbasevelocity[1 as libc::c_int as usize];
    // copy player buttons
    (*clent).v.button = (*ucmd).buttons as libc::c_int;
    (*clent).v.light_level = (*ucmd).lightlevel as libc::c_int;
    if (*ucmd).impulse != 0 {
        (*clent).v.impulse = (*ucmd).impulse as libc::c_int
    }
    if (*ucmd).impulse as libc::c_int == 204 as libc::c_int {
        // force client.dll update
        SV_RefreshUserinfo();
    }
    (*svgame.globals).time = (*cl).timebase as libc::c_float;
    svgame.dllFuncs.pfnPlayerPreThink.expect("non-null function pointer")(clent);
    SV_PlayerRunThink(clent, frametime as libc::c_float, (*cl).timebase);
    // If conveyor, or think, set basevelocity, then send to client asap too.
    if !((*clent).v.basevelocity[0 as libc::c_int as usize] == 0.0f32 &&
             (*clent).v.basevelocity[1 as libc::c_int as usize] == 0.0f32 &&
             (*clent).v.basevelocity[2 as libc::c_int as usize] == 0.0f32) {
        (*clent).v.clbasevelocity[0 as libc::c_int as usize] =
            (*clent).v.basevelocity[0 as libc::c_int as usize];
        (*clent).v.clbasevelocity[1 as libc::c_int as usize] =
            (*clent).v.basevelocity[1 as libc::c_int as usize];
        (*clent).v.clbasevelocity[2 as libc::c_int as usize] =
            (*clent).v.basevelocity[2 as libc::c_int as usize]
    }
    // setup playermove state
    SV_SetupPMove(svgame.pmove, cl, ucmd, (*cl).physinfo.as_mut_ptr());
    // motor!
    svgame.dllFuncs.pfnPM_Move.expect("non-null function pointer")(svgame.pmove,
                                                                   true_0);
    // copy results back to client
    SV_FinishPMove(svgame.pmove, cl);
    if (*clent).v.solid != 0 as libc::c_int && sv.playersonly as u64 == 0 {
        if svgame.physFuncs.PM_PlayerTouch.is_some() {
            // run custom impact function
            svgame.physFuncs.PM_PlayerTouch.expect("non-null function pointer")(svgame.pmove,
                                                                                clent);
        } else {
            // link into place and touch triggers
            SV_LinkEdict(clent, true_0); // save velocity
            oldvel[0 as libc::c_int as usize] =
                (*clent).v.velocity[0 as libc::c_int as usize];
            oldvel[1 as libc::c_int as usize] =
                (*clent).v.velocity[1 as libc::c_int as usize];
            oldvel[2 as libc::c_int as usize] =
                (*clent).v.velocity[2 as libc::c_int as usize];
            // touch other objects
            i = 0 as libc::c_int;
            while i < (*svgame.pmove).numtouch {
                pmtrace =
                    &mut *(*svgame.pmove).touchindex.as_mut_ptr().offset(i as
                                                                             isize)
                        as *mut pmtrace_t;
                touch =
                    SV_EdictNum((*svgame.pmove).physents[(*pmtrace).ent as
                                                             usize].info);
                (*clent).v.velocity[0 as libc::c_int as usize] =
                    (*pmtrace).deltavelocity[0 as libc::c_int as usize];
                (*clent).v.velocity[1 as libc::c_int as usize] =
                    (*pmtrace).deltavelocity[1 as libc::c_int as usize];
                (*clent).v.velocity[2 as libc::c_int as usize] =
                    (*pmtrace).deltavelocity[2 as libc::c_int as usize];
                PM_ConvertTrace(&mut trace, pmtrace, touch);
                SV_Impact(touch, clent, &mut trace);
                i += 1
            }
            // restore velocity
            (*clent).v.velocity[0 as libc::c_int as usize] =
                oldvel[0 as libc::c_int as usize];
            (*clent).v.velocity[1 as libc::c_int as usize] =
                oldvel[1 as libc::c_int as usize];
            (*clent).v.velocity[2 as libc::c_int as usize] =
                oldvel[2 as libc::c_int as usize]
        }
    }
    (*svgame.pmove).numtouch = 0 as libc::c_int;
    (*svgame.globals).time = (*cl).timebase as libc::c_float;
    (*svgame.globals).frametime = frametime as libc::c_float;
    // run post-think
    svgame.dllFuncs.pfnPlayerPostThink.expect("non-null function pointer")(clent);
    svgame.dllFuncs.pfnCmdEnd.expect("non-null function pointer")(clent);
    if (*cl).flags & (1 as libc::c_uint) << 7 as libc::c_int == 0 {
        SV_RestoreMoveInterpolant(cl);
    };
}
