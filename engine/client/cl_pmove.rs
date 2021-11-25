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
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn sqrtf(_: libc::c_float) -> libc::c_float;
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
    fn Q_snprintf(buffer: *mut libc::c_char, buffersize: size_t,
                  format: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn Sys_DoubleTime() -> libc::c_double;
    #[no_mangle]
    fn Cvar_DirectSet(var: *mut convar_t, value: *const libc::c_char);
    #[no_mangle]
    static mut host_developer: convar_t;
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    fn COM_FileSize(filename: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn COM_FreeFile(buffer: *mut libc::c_void);
    #[no_mangle]
    fn SV_Active() -> qboolean;
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
    fn COM_LoadFile(filename: *const libc::c_char, usehunk: libc::c_int,
                    pLength: *mut libc::c_int) -> *mut byte;
    #[no_mangle]
    fn CL_GetEntityByIndex(index: libc::c_int) -> *mut cl_entity_s;
    #[no_mangle]
    static mut clgame: clgame_static_t;
    #[no_mangle]
    fn SinCos(radians: libc::c_float, sine: *mut libc::c_float,
              cosine: *mut libc::c_float);
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
    static mut CL_UPDATE_BACKUP: libc::c_int;
    #[no_mangle]
    static mut cl: client_t;
    #[no_mangle]
    static mut cls: client_static_t;
    #[no_mangle]
    static mut cl_nopred: *mut convar_t;
    #[no_mangle]
    static mut cl_showerror: *mut convar_t;
    #[no_mangle]
    static mut cl_nosmooth: *mut convar_t;
    #[no_mangle]
    static mut cl_smoothtime: *mut convar_t;
    #[no_mangle]
    static mut cl_solid_players: *mut convar_t;
    #[no_mangle]
    static mut cl_idealpitchscale: *mut convar_t;
    #[no_mangle]
    static mut cl_lw: *mut convar_t;
    #[no_mangle]
    fn CL_Particle(org: *const vec_t, color: libc::c_int, life: libc::c_float,
                   zpos: libc::c_int, zvel: libc::c_int);
    #[no_mangle]
    fn CL_DemoInterpolateAngles();
    #[no_mangle]
    fn CL_PlaybackEvent(flags: libc::c_int, pInvoker: *const edict_t,
                        eventindex: word, delay: libc::c_float,
                        origin: *mut libc::c_float,
                        angles: *mut libc::c_float, fparam1: libc::c_float,
                        fparam2: libc::c_float, iparam1: libc::c_int,
                        iparam2: libc::c_int, bparam1: libc::c_int,
                        bparam2: libc::c_int);
    #[no_mangle]
    fn CL_ModelHandle(modelindex: libc::c_int) -> *mut model_t;
    #[no_mangle]
    fn pfnTraceSurface(ground: libc::c_int, vstart: *mut libc::c_float,
                       vend: *mut libc::c_float) -> *mut msurface_s;
    #[no_mangle]
    fn S_RegisterSound(sample: *const libc::c_char) -> sound_t;
    #[no_mangle]
    fn S_StartSound(pos: *const vec_t, ent: libc::c_int, chan: libc::c_int,
                    sfx: sound_t, vol: libc::c_float, attn: libc::c_float,
                    pitch: libc::c_int, flags: libc::c_int);
    #[no_mangle]
    fn CL_ComputePlayerOrigin(clent: *mut cl_entity_t);
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
    fn PM_TraceTexture(pe: *mut physent_t, vstart: *mut vec_t,
                       vend: *mut vec_t) -> *const libc::c_char;
    #[no_mangle]
    fn PM_TestLineExt(pmove: *mut playermove_t, ents: *mut physent_t,
                      numents: libc::c_int, start: *const vec_t,
                      end: *const vec_t, flags: libc::c_int) -> libc::c_int;
}
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
pub type sound_t = libc::c_int;
pub type vec_t = libc::c_float;
pub type vec3_t = [vec_t; 3];
pub type vec4_t = [vec_t; 4];
pub type rgba_t = [byte; 4];
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
pub type pmtrace_t = pmtrace_s;
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
pub type pfnIgnore
    =
    Option<unsafe extern "C" fn(_: *mut physent_t) -> libc::c_int>;
pub type local_state_t = local_state_s;
pub type C2RustUnnamed = libc::c_uint;
pub const DEMO_QUAKE1: C2RustUnnamed = 2;
pub const DEMO_XASH3D: C2RustUnnamed = 1;
pub const DEMO_INACTIVE: C2RustUnnamed = 0;
pub type connstate_e = libc::c_uint;
pub const ca_cinematic: connstate_e = 5;
pub const ca_active: connstate_e = 4;
pub const ca_validate: connstate_e = 3;
pub const ca_connected: connstate_e = 2;
pub const ca_connecting: connstate_e = 1;
pub const ca_disconnected: connstate_e = 0;
pub type connstate_t = connstate_e;
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
#[inline(always)]
unsafe extern "C" fn __tg_sqrt(mut __x: libc::c_float) -> libc::c_float {
    return sqrtf(__x);
}
// above this is assumed to be a teleport, don't smooth, etc.
/*
=============
CL_ClearPhysEnts

=============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ClearPhysEnts() {
    (*clgame.pmove).numtouch = 0 as libc::c_int;
    (*clgame.pmove).numvisent = 0 as libc::c_int;
    (*clgame.pmove).nummoveent = 0 as libc::c_int;
    (*clgame.pmove).numphysent = 0 as libc::c_int;
}
/*
=============
CL_PushPMStates

=============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_PushPMStates() {
    if clgame.pushed as u64 != 0 { return }
    clgame.oldphyscount = (*clgame.pmove).numphysent;
    clgame.oldviscount = (*clgame.pmove).numvisent;
    clgame.pushed = true_0;
}
/*
=============
CL_PopPMStates

=============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_PopPMStates() {
    if clgame.pushed as u64 == 0 { return }
    (*clgame.pmove).numphysent = clgame.oldphyscount;
    (*clgame.pmove).numvisent = clgame.oldviscount;
    clgame.pushed = false_0;
}
/*
=============
CL_PushTraceBounds

=============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_PushTraceBounds(mut hullnum: libc::c_int,
                                            mut mins: *const libc::c_float,
                                            mut maxs: *const libc::c_float) {
    hullnum =
        if hullnum >= 0 as libc::c_int {
            if hullnum < 3 as libc::c_int {
                hullnum
            } else { 3 as libc::c_int }
        } else { 0 as libc::c_int };
    (*clgame.pmove).player_mins[hullnum as usize][0 as libc::c_int as usize] =
        *mins.offset(0 as libc::c_int as isize);
    (*clgame.pmove).player_mins[hullnum as usize][1 as libc::c_int as usize] =
        *mins.offset(1 as libc::c_int as isize);
    (*clgame.pmove).player_mins[hullnum as usize][2 as libc::c_int as usize] =
        *mins.offset(2 as libc::c_int as isize);
    (*clgame.pmove).player_maxs[hullnum as usize][0 as libc::c_int as usize] =
        *maxs.offset(0 as libc::c_int as isize);
    (*clgame.pmove).player_maxs[hullnum as usize][1 as libc::c_int as usize] =
        *maxs.offset(1 as libc::c_int as isize);
    (*clgame.pmove).player_maxs[hullnum as usize][2 as libc::c_int as usize] =
        *maxs.offset(2 as libc::c_int as isize);
}
/*
=============
CL_PopTraceBounds

=============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_PopTraceBounds() {
    memcpy((*clgame.pmove).player_mins.as_mut_ptr() as *mut libc::c_void,
           host.player_mins.as_mut_ptr() as *const libc::c_void,
           ::std::mem::size_of::<[vec3_t; 4]>() as libc::c_ulong);
    memcpy((*clgame.pmove).player_maxs.as_mut_ptr() as *mut libc::c_void,
           host.player_maxs.as_mut_ptr() as *const libc::c_void,
           ::std::mem::size_of::<[vec3_t; 4]>() as libc::c_ulong);
}
/*
===============
CL_IsPredicted
===============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_IsPredicted() -> qboolean {
    if (*cl_nopred).value != 0. || cl.intermission != 0 { return false_0 }
    // never predict the quake demos
    if cls.demoplayback == DEMO_QUAKE1 as libc::c_int { return false_0 }
    return true_0;
}
/*
===============
CL_SetLastUpdate
===============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_SetLastUpdate() {
    cls.lastupdate_sequence = cls.netchan.incoming_sequence as libc::c_int;
}
/*
===============
CL_RedoPrediction
===============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_RedoPrediction() {
    if cls.netchan.incoming_sequence !=
           cls.lastupdate_sequence as libc::c_uint {
        CL_PredictMovement(true_0);
        CL_CheckPredictionError();
    };
}
/*
===============
CL_SetIdealPitch
===============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_SetIdealPitch() {
    let mut angleval: libc::c_float = 0.;
    let mut sinval: libc::c_float = 0.;
    let mut cosval: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut step: libc::c_int = 0;
    let mut dir: libc::c_int = 0;
    let mut steps: libc::c_int = 0;
    let mut z: [libc::c_float; 6] = [0.; 6];
    let mut top: vec3_t = [0.; 3];
    let mut bottom: vec3_t = [0.; 3];
    let mut tr: pmtrace_t =
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
    if cl.local.onground == -(1 as libc::c_int) { return }
    angleval =
        cl.viewangles[1 as libc::c_int as usize] *
            (3.14159265358979323846f64 * 2 as libc::c_int as libc::c_double)
                as libc::c_float / 360.0f32;
    SinCos(angleval, &mut sinval, &mut cosval);
    // Now move forward by 36, 48, 60, etc. units from the eye position and drop lines straight down
	// 160 or so units to see what's below
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        top[0 as libc::c_int as usize] =
            cl.simorg[0 as libc::c_int as usize] +
                cosval * (i as libc::c_float + 3.0f32) * 12.0f32;
        top[1 as libc::c_int as usize] =
            cl.simorg[1 as libc::c_int as usize] +
                sinval * (i as libc::c_float + 3.0f32) * 12.0f32;
        top[2 as libc::c_int as usize] =
            cl.simorg[2 as libc::c_int as usize] +
                cl.viewheight[2 as libc::c_int as usize];
        bottom[0 as libc::c_int as usize] = top[0 as libc::c_int as usize];
        bottom[1 as libc::c_int as usize] = top[1 as libc::c_int as usize];
        bottom[2 as libc::c_int as usize] =
            top[2 as libc::c_int as usize] - 160.0f32;
        // skip any monsters (only world and brush models)
        tr =
            CL_TraceLine(top.as_mut_ptr(), bottom.as_mut_ptr(),
                         0x2 as
                             libc::c_int); // looking at a wall, leave ideal the way is was
        if tr.allsolid as u64 != 0 { return } // near a dropoff
        if tr.fraction == 1.0f32 { return } // mixed changes
        z[i as usize] =
            top[2 as libc::c_int as usize] +
                tr.fraction *
                    (bottom[2 as libc::c_int as usize] -
                         top[2 as libc::c_int as usize]);
        i += 1
    }
    dir = 0 as libc::c_int;
    steps = 0 as libc::c_int;
    j = 1 as libc::c_int;
    while j < i {
        step =
            (z[j as usize] - z[(j - 1 as libc::c_int) as usize]) as
                libc::c_int;
        if !(step as libc::c_float > -0.1f32 &&
                 (step as libc::c_float) < 0.1f32) {
            if dir != 0 &&
                   ((step - dir) as libc::c_float > 0.1f32 ||
                        ((step - dir) as libc::c_float) < -0.1f32) {
                return
            }
            steps += 1;
            dir = step
        }
        j += 1
    }
    if dir == 0 { cl.local.idealpitch = 0.0f32; return }
    if steps < 2 as libc::c_int { return }
    cl.local.idealpitch = -dir as libc::c_float * (*cl_idealpitchscale).value;
}
/*
==================
CL_PlayerTeleported

check for instant movement in case
we don't want interpolate this
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_PlayerTeleported(mut from: *mut local_state_t,
                                             mut to: *mut local_state_t)
 -> qboolean {
    let mut len: libc::c_int = 0;
    let mut maxlen: libc::c_int = 0;
    let mut delta: vec3_t = [0.; 3];
    delta[0 as libc::c_int as usize] =
        (*to).playerstate.origin[0 as libc::c_int as usize] -
            (*from).playerstate.origin[0 as libc::c_int as usize];
    delta[1 as libc::c_int as usize] =
        (*to).playerstate.origin[1 as libc::c_int as usize] -
            (*from).playerstate.origin[1 as libc::c_int as usize];
    delta[2 as libc::c_int as usize] =
        (*to).playerstate.origin[2 as libc::c_int as usize] -
            (*from).playerstate.origin[2 as libc::c_int as usize];
    // compute potential max movement in units per frame and compare with entity movement
    maxlen =
        (clgame.movevars.maxvelocity * (1.0f32 / 20.0f32)) as libc::c_int;
    len =
        __tg_sqrt(delta[0 as libc::c_int as usize] *
                      delta[0 as libc::c_int as usize] +
                      delta[1 as libc::c_int as usize] *
                          delta[1 as libc::c_int as usize] +
                      delta[2 as libc::c_int as usize] *
                          delta[2 as libc::c_int as usize]) as libc::c_int;
    return (len > maxlen) as libc::c_int as qboolean;
}
/*
===================
CL_CheckPredictionError
===================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_CheckPredictionError() {
    let mut frame: libc::c_int = 0;
    let mut cmd: libc::c_int = 0;
    static mut pos: libc::c_int = 0 as libc::c_int;
    let mut delta: vec3_t = [0.; 3];
    let mut dist: libc::c_float = 0.;
    if CL_IsPredicted() as u64 == 0 { return }
    // calculate the last usercmd_t we sent that the server has processed
    frame =
        (cls.netchan.incoming_acknowledged &
             (CL_UPDATE_BACKUP - 1 as libc::c_int) as libc::c_uint) as
            libc::c_int;
    cmd = cl.parsecountmod;
    // compare what the server returned with what we had predicted it to be
    delta[0 as libc::c_int as usize] =
        cl.frames[cmd as
                      usize].playerstate[cl.playernum as
                                             usize].origin[0 as libc::c_int as
                                                               usize] -
            cl.local.predicted_origins[frame as
                                           usize][0 as libc::c_int as usize];
    delta[1 as libc::c_int as usize] =
        cl.frames[cmd as
                      usize].playerstate[cl.playernum as
                                             usize].origin[1 as libc::c_int as
                                                               usize] -
            cl.local.predicted_origins[frame as
                                           usize][1 as libc::c_int as usize];
    delta[2 as libc::c_int as usize] =
        cl.frames[cmd as
                      usize].playerstate[cl.playernum as
                                             usize].origin[2 as libc::c_int as
                                                               usize] -
            cl.local.predicted_origins[frame as
                                           usize][2 as libc::c_int as usize];
    dist =
        __tg_sqrt(delta[0 as libc::c_int as usize] *
                      delta[0 as libc::c_int as usize] +
                      delta[1 as libc::c_int as usize] *
                          delta[1 as libc::c_int as usize] +
                      delta[2 as libc::c_int as usize] *
                          delta[2 as libc::c_int as usize]);
    // save the prediction error for interpolation
    if dist > 64.0f32 {
        if (*cl_showerror).value != 0. && host_developer.value != 0. {
            pos += 1;
            Con_NPrintf(10 as libc::c_int + (pos & 3 as libc::c_int),
                        b"^3player teleported:^7 %.3f units\n\x00" as
                            *const u8 as *const libc::c_char,
                        dist as libc::c_double);
        }
        // a teleport or something or gamepaused
        cl.local.prediction_error[2 as libc::c_int as usize] =
            0 as libc::c_int as vec_t;
        cl.local.prediction_error[1 as libc::c_int as usize] =
            cl.local.prediction_error[2 as libc::c_int as usize];
        cl.local.prediction_error[0 as libc::c_int as usize] =
            cl.local.prediction_error[1 as libc::c_int as usize]
    } else {
        if (*cl_showerror).value != 0. && dist > 0.5f32 &&
               host_developer.value != 0. {
            pos += 1;
            Con_NPrintf(10 as libc::c_int + (pos & 3 as libc::c_int),
                        b"^1prediction error:^7 %.3f units\n\x00" as *const u8
                            as *const libc::c_char, dist as libc::c_double);
        }
        cl.local.predicted_origins[frame as usize][0 as libc::c_int as usize]
            =
            cl.frames[cmd as
                          usize].playerstate[cl.playernum as
                                                 usize].origin[0 as
                                                                   libc::c_int
                                                                   as usize];
        cl.local.predicted_origins[frame as usize][1 as libc::c_int as usize]
            =
            cl.frames[cmd as
                          usize].playerstate[cl.playernum as
                                                 usize].origin[1 as
                                                                   libc::c_int
                                                                   as usize];
        cl.local.predicted_origins[frame as usize][2 as libc::c_int as usize]
            =
            cl.frames[cmd as
                          usize].playerstate[cl.playernum as
                                                 usize].origin[2 as
                                                                   libc::c_int
                                                                   as usize];
        // save for error interpolation
        cl.local.prediction_error[0 as libc::c_int as usize] =
            delta[0 as libc::c_int as usize];
        cl.local.prediction_error[1 as libc::c_int as usize] =
            delta[1 as libc::c_int as usize];
        cl.local.prediction_error[2 as libc::c_int as usize] =
            delta[2 as libc::c_int as usize];
        // GoldSrc checks for singleplayer
		// we would check for local server
        if dist > 0.25f32 && SV_Active() as u64 == 0 {
            cls.correction_time = (*cl_smoothtime).value as libc::c_double
        }
    };
}
/*
=============
CL_SetUpPlayerPrediction

Calculate the new position of players, without other player clipping
We do this to set up real player prediction.
Players are predicted twice, first without clipping other players,
then with clipping against them.
This sets up the first phase.
=============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_SetUpPlayerPrediction(mut dopred: libc::c_int,
                                                  mut bIncludeLocalClient:
                                                      libc::c_int) {
    let mut state: *mut entity_state_t =
        0 as *mut entity_state_t; // not present this frame
    let mut player: *mut predicted_player_t = 0 as *mut predicted_player_t;
    let mut ent: *mut cl_entity_t = 0 as *mut cl_entity_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (1 as libc::c_int) << 5 as libc::c_int {
        state =
            &mut *(*cl.frames.as_mut_ptr().offset(cl.parsecountmod as
                                                      isize)).playerstate.as_mut_ptr().offset(i
                                                                                                  as
                                                                                                  isize)
                as *mut entity_state_t;
        player =
            &mut *cls.predicted_players.as_mut_ptr().offset(i as isize) as
                *mut predicted_player_t;
        (*player).active = false_0;
        if !((*state).messagenum != cl.parsecount) {
            if !((*state).modelindex == 0) {
                (*player).active = true_0;
                (*player).movetype = (*state).movetype;
                (*player).solid = (*state).solid as libc::c_int;
                (*player).usehull = (*state).usehull;
                if !((*state).effects & 128 as libc::c_int != 0 &&
                         bIncludeLocalClient == 0 && cl.playernum == i) {
                    // note that the local player is special, since he moves locally
		// we use his last predicted postition
                    if cl.playernum == i {
                        (*player).origin[0 as libc::c_int as usize] =
                            (*state).origin[0 as libc::c_int as usize];
                        (*player).origin[1 as libc::c_int as usize] =
                            (*state).origin[1 as libc::c_int as usize];
                        (*player).origin[2 as libc::c_int as usize] =
                            (*state).origin[2 as libc::c_int as usize];
                        (*player).angles[0 as libc::c_int as usize] =
                            (*state).angles[0 as libc::c_int as usize];
                        (*player).angles[1 as libc::c_int as usize] =
                            (*state).angles[1 as libc::c_int as usize];
                        (*player).angles[2 as libc::c_int as usize] =
                            (*state).angles[2 as libc::c_int as usize]
                    } else {
                        ent = CL_GetEntityByIndex(i + 1 as libc::c_int);
                        CL_ComputePlayerOrigin(ent);
                        (*player).origin[0 as libc::c_int as usize] =
                            (*ent).origin[0 as libc::c_int as usize];
                        (*player).origin[1 as libc::c_int as usize] =
                            (*ent).origin[1 as libc::c_int as usize];
                        (*player).origin[2 as libc::c_int as usize] =
                            (*ent).origin[2 as libc::c_int as usize];
                        (*player).angles[0 as libc::c_int as usize] =
                            (*ent).angles[0 as libc::c_int as usize];
                        (*player).angles[1 as libc::c_int as usize] =
                            (*ent).angles[1 as libc::c_int as usize];
                        (*player).angles[2 as libc::c_int as usize] =
                            (*ent).angles[2 as libc::c_int as usize]
                    }
                }
            }
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn CL_ClipPMoveToEntity(mut pe: *mut physent_t,
                                              mut start: *const vec_t,
                                              mut mins: *mut vec_t,
                                              mut maxs: *mut vec_t,
                                              mut end: *const vec_t,
                                              mut tr: *mut pmtrace_t) {
    if clgame.dllFuncs.pfnClipMoveToEntity.is_some() {
        // do custom sweep test
        clgame.dllFuncs.pfnClipMoveToEntity.expect("non-null function pointer")(pe,
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
unsafe extern "C" fn CL_CopyEntityToPhysEnt(mut pe: *mut physent_t,
                                            mut state: *mut entity_state_t,
                                            mut visent: qboolean) {
    let mut mod_0: *mut model_t = CL_ModelHandle((*state).modelindex);
    (*pe).player = 0 as libc::c_int;
    if (*state).number >= 1 as libc::c_int && (*state).number <= cl.maxclients
       {
        (*pe).player = (*state).number
    }
    if (*pe).player != 0 {
        // client or bot
        Q_snprintf((*pe).name.as_mut_ptr(),
                   ::std::mem::size_of::<[libc::c_char; 32]>() as
                       libc::c_ulong,
                   b"player %i\x00" as *const u8 as *const libc::c_char,
                   (*pe).player - 1 as libc::c_int);
    } else {
        // otherwise copy the modelname
        Q_strncpy((*pe).name.as_mut_ptr(), (*mod_0).name.as_mut_ptr(),
                  ::std::mem::size_of::<[libc::c_char; 32]>() as
                      libc::c_ulong);
    }
    (*pe).studiomodel = 0 as *mut model_s;
    (*pe).model = (*pe).studiomodel;
    (*pe).mins[0 as libc::c_int as usize] =
        (*state).mins[0 as libc::c_int as usize];
    (*pe).mins[1 as libc::c_int as usize] =
        (*state).mins[1 as libc::c_int as usize];
    (*pe).mins[2 as libc::c_int as usize] =
        (*state).mins[2 as libc::c_int as usize];
    (*pe).maxs[0 as libc::c_int as usize] =
        (*state).maxs[0 as libc::c_int as usize];
    (*pe).maxs[1 as libc::c_int as usize] =
        (*state).maxs[1 as libc::c_int as usize];
    (*pe).maxs[2 as libc::c_int as usize] =
        (*state).maxs[2 as libc::c_int as usize];
    if (*state).solid as libc::c_int == 2 as libc::c_int {
        if (*mod_0).flags as libc::c_uint &
               (1 as libc::c_uint) << 9 as libc::c_int != 0 {
            (*pe).studiomodel = mod_0
        }
    } else if (*pe).solid != 4 as libc::c_int && !mod_0.is_null() &&
                  (*mod_0).type_0 as libc::c_int == mod_studio as libc::c_int
     {
        (*pe).studiomodel = mod_0
    } else { (*pe).model = mod_0 }
    // rare case: not solid entities in vistrace
    if visent as libc::c_uint != 0 &&
           ((*pe).mins[0 as libc::c_int as usize] == 0.0f32 &&
                (*pe).mins[1 as libc::c_int as usize] == 0.0f32 &&
                (*pe).mins[2 as libc::c_int as usize] == 0.0f32) {
        (*pe).mins[0 as libc::c_int as usize] =
            (*mod_0).mins[0 as libc::c_int as usize]; // unused in GoldSrc
        (*pe).mins[1 as libc::c_int as usize] =
            (*mod_0).mins[1 as libc::c_int as usize];
        (*pe).mins[2 as libc::c_int as usize] =
            (*mod_0).mins[2 as libc::c_int as usize];
        (*pe).maxs[0 as libc::c_int as usize] =
            (*mod_0).maxs[0 as libc::c_int as usize];
        (*pe).maxs[1 as libc::c_int as usize] =
            (*mod_0).maxs[1 as libc::c_int as usize];
        (*pe).maxs[2 as libc::c_int as usize] =
            (*mod_0).maxs[2 as libc::c_int as usize]
    }
    (*pe).info = (*state).number;
    (*pe).origin[0 as libc::c_int as usize] =
        (*state).origin[0 as libc::c_int as usize];
    (*pe).origin[1 as libc::c_int as usize] =
        (*state).origin[1 as libc::c_int as usize];
    (*pe).origin[2 as libc::c_int as usize] =
        (*state).origin[2 as libc::c_int as usize];
    (*pe).angles[0 as libc::c_int as usize] =
        (*state).angles[0 as libc::c_int as usize];
    (*pe).angles[1 as libc::c_int as usize] =
        (*state).angles[1 as libc::c_int as usize];
    (*pe).angles[2 as libc::c_int as usize] =
        (*state).angles[2 as libc::c_int as usize];
    (*pe).solid = (*state).solid as libc::c_int;
    (*pe).rendermode = (*state).rendermode;
    (*pe).skin = (*state).skin as libc::c_int;
    (*pe).frame = (*state).frame;
    (*pe).sequence = (*state).sequence;
    memcpy(&mut *(*pe).controller.as_mut_ptr().offset(0 as libc::c_int as
                                                          isize) as *mut byte
               as *mut libc::c_void,
           &mut *(*state).controller.as_mut_ptr().offset(0 as libc::c_int as
                                                             isize) as
               *mut byte as *const libc::c_void,
           ::std::mem::size_of::<[byte; 4]>() as libc::c_ulong);
    memcpy(&mut *(*pe).blending.as_mut_ptr().offset(0 as libc::c_int as isize)
               as *mut byte as *mut libc::c_void,
           &mut *(*state).blending.as_mut_ptr().offset(0 as libc::c_int as
                                                           isize) as *mut byte
               as *const libc::c_void,
           ::std::mem::size_of::<[byte; 2]>() as libc::c_ulong);
    (*pe).movetype = (*state).movetype;
    (*pe).takedamage =
        if (*pe).player != 0 { 1 as libc::c_int } else { 0 as libc::c_int };
    (*pe).team = (*state).team;
    (*pe).classnumber = (*state).playerclass;
    (*pe).blooddecal = 0 as libc::c_int;
    // for mods
    (*pe).iuser1 = (*state).iuser1;
    (*pe).iuser2 = (*state).iuser2;
    (*pe).iuser3 = (*state).iuser3;
    (*pe).iuser4 = (*state).iuser4;
    (*pe).fuser1 = (*state).fuser1;
    (*pe).fuser2 = (*state).fuser2;
    (*pe).fuser3 = (*state).fuser3;
    (*pe).fuser4 = (*state).fuser4;
    (*pe).vuser1[0 as libc::c_int as usize] =
        (*state).vuser1[0 as libc::c_int as usize];
    (*pe).vuser1[1 as libc::c_int as usize] =
        (*state).vuser1[1 as libc::c_int as usize];
    (*pe).vuser1[2 as libc::c_int as usize] =
        (*state).vuser1[2 as libc::c_int as usize];
    (*pe).vuser2[0 as libc::c_int as usize] =
        (*state).vuser2[0 as libc::c_int as usize];
    (*pe).vuser2[1 as libc::c_int as usize] =
        (*state).vuser2[1 as libc::c_int as usize];
    (*pe).vuser2[2 as libc::c_int as usize] =
        (*state).vuser2[2 as libc::c_int as usize];
    (*pe).vuser3[0 as libc::c_int as usize] =
        (*state).vuser3[0 as libc::c_int as usize];
    (*pe).vuser3[1 as libc::c_int as usize] =
        (*state).vuser3[1 as libc::c_int as usize];
    (*pe).vuser3[2 as libc::c_int as usize] =
        (*state).vuser3[2 as libc::c_int as usize];
    (*pe).vuser4[0 as libc::c_int as usize] =
        (*state).vuser4[0 as libc::c_int as usize];
    (*pe).vuser4[1 as libc::c_int as usize] =
        (*state).vuser4[1 as libc::c_int as usize];
    (*pe).vuser4[2 as libc::c_int as usize] =
        (*state).vuser4[2 as libc::c_int as usize];
}
/*
====================
CL_AddLinksToPmove

collect solid entities
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_AddLinksToPmove(mut frame: *mut frame_t) {
    let mut state: *mut entity_state_t = 0 as *mut entity_state_t;
    let mut model: *mut model_t = 0 as *mut model_t;
    let mut pe: *mut physent_t = 0 as *mut physent_t;
    let mut i: libc::c_int = 0;
    if (*frame).valid as u64 == 0 { return }
    i = 0 as libc::c_int;
    while i < (*frame).num_entities {
        state =
            &mut *cls.packet_entities.offset((((*frame).first_entity + i) %
                                                  cls.num_client_entities) as
                                                 isize) as
                *mut entity_state_t;
        if !((*state).number >= 1 as libc::c_int &&
                 (*state).number <= cl.maxclients) {
            if !((*state).modelindex == 0) {
                model = CL_ModelHandle((*state).modelindex);
                if !model.is_null() {
                    if !((*state).owner != 0 as libc::c_int &&
                             (*state).owner ==
                                 cl.playernum + 1 as libc::c_int) {
                        if ((*model).hulls[1 as libc::c_int as
                                               usize].lastclipnode != 0 ||
                                (*model).type_0 as libc::c_int ==
                                    mod_studio as libc::c_int) &&
                               (*clgame.pmove).numvisent < 600 as libc::c_int
                           {
                            pe =
                                &mut *(*clgame.pmove).visents.as_mut_ptr().offset((*clgame.pmove).numvisent
                                                                                      as
                                                                                      isize)
                                    as *mut physent_t;
                            CL_CopyEntityToPhysEnt(pe, state, true_0);
                            (*clgame.pmove).numvisent += 1
                        }
                        if !((*state).solid as libc::c_int == 1 as libc::c_int
                                 ||
                                 (*state).solid as libc::c_int ==
                                     0 as libc::c_int &&
                                     (*state).skin as libc::c_int >=
                                         -(1 as libc::c_int)) {
                            // dead body
                            if !((*state).mins[2 as libc::c_int as usize] ==
                                     0.0f32 &&
                                     (*state).maxs[2 as libc::c_int as usize]
                                         == 1.0f32) {
                                // can't collide with zeroed hull
                                if !((*state).mins[0 as libc::c_int as usize]
                                         == 0.0f32 &&
                                         (*state).mins[1 as libc::c_int as
                                                           usize] == 0.0f32 &&
                                         (*state).mins[2 as libc::c_int as
                                                           usize] == 0.0f32 &&
                                         ((*state).maxs[0 as libc::c_int as
                                                            usize] == 0.0f32
                                              &&
                                              (*state).maxs[1 as libc::c_int
                                                                as usize] ==
                                                  0.0f32 &&
                                              (*state).maxs[2 as libc::c_int
                                                                as usize] ==
                                                  0.0f32)) {
                                    if (*state).solid as libc::c_int ==
                                           0 as libc::c_int &&
                                           (*state).skin as libc::c_int ==
                                               -(16 as libc::c_int) {
                                        if !((*clgame.pmove).nummoveent >=
                                                 64 as libc::c_int) {
                                            pe =
                                                &mut *(*clgame.pmove).moveents.as_mut_ptr().offset((*clgame.pmove).nummoveent
                                                                                                       as
                                                                                                       isize)
                                                    as *mut physent_t;
                                            CL_CopyEntityToPhysEnt(pe, state,
                                                                   false_0);
                                            (*clgame.pmove).nummoveent += 1
                                        }
                                    } else if !((*model).hulls[1 as
                                                                   libc::c_int
                                                                   as
                                                                   usize].lastclipnode
                                                    == 0 &&
                                                    (*model).type_0 as
                                                        libc::c_int !=
                                                        mod_studio as
                                                            libc::c_int) {
                                        // reserve slots for all the clients
                                        if !((*clgame.pmove).numphysent >=
                                                 600 as libc::c_int -
                                                     cl.maxclients) {
                                            pe =
                                                &mut *(*clgame.pmove).physents.as_mut_ptr().offset((*clgame.pmove).numphysent
                                                                                                       as
                                                                                                       isize)
                                                    as *mut physent_t;
                                            CL_CopyEntityToPhysEnt(pe, state,
                                                                   false_0);
                                            (*clgame.pmove).numphysent += 1
                                        }
                                    }
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
CL_SetSolidEntities

Builds all the pmove physents for the current frame
===============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_SetSolidEntities() {
    let mut pe: *mut physent_t = (*clgame.pmove).physents.as_mut_ptr();
    // setup physents
    (*clgame.pmove).numvisent = 1 as libc::c_int;
    (*clgame.pmove).numphysent = 1 as libc::c_int;
    (*clgame.pmove).nummoveent = 0 as libc::c_int;
    memset((*clgame.pmove).physents.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<physent_t>() as libc::c_ulong);
    memset((*clgame.pmove).visents.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<physent_t>() as libc::c_ulong);
    (*pe).model = cl.worldmodel;
    if !(*pe).model.is_null() {
        Q_strncpy((*pe).name.as_mut_ptr(), (*(*pe).model).name.as_mut_ptr(),
                  ::std::mem::size_of::<[libc::c_char; 32]>() as
                      libc::c_ulong);
    }
    (*pe).takedamage = 1 as libc::c_int;
    (*pe).solid = 4 as libc::c_int;
    // share to visents
    (*clgame.pmove).visents[0 as libc::c_int as usize] =
        (*clgame.pmove).physents[0 as libc::c_int as usize];
    // add all other entities exlucde players
    CL_AddLinksToPmove(&mut *cl.frames.as_mut_ptr().offset(cl.parsecountmod as
                                                               isize));
}
/*
===============
CL_SetSolidPlayers

Builds all the pmove physents for the current frame
Note that CL_SetUpPlayerPrediction() must be called first!
pmove must be setup with world and solid entity hulls before calling
(via CL_PredictMove)
===============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_SetSolidPlayers(mut playernum: libc::c_int) {
    let mut state: *mut entity_state_t =
        0 as *mut entity_state_t; // not present this frame
    let mut player: *mut predicted_player_t = 0 as *mut predicted_player_t;
    let mut pe: *mut physent_t = 0 as *mut physent_t;
    let mut i: libc::c_int = 0;
    if (*cl_solid_players).value == 0. { return }
    let mut current_block_13: u64;
    i = 0 as libc::c_int;
    while i < (1 as libc::c_int) << 5 as libc::c_int {
        state =
            &mut *(*cl.frames.as_mut_ptr().offset(cl.parsecountmod as
                                                      isize)).playerstate.as_mut_ptr().offset(i
                                                                                                  as
                                                                                                  isize)
                as *mut entity_state_t;
        player =
            &mut *cls.predicted_players.as_mut_ptr().offset(i as isize) as
                *mut predicted_player_t;
        if playernum == -(1 as libc::c_int) {
            if i != cl.playernum && (*player).active as u64 == 0 {
                current_block_13 = 7502529970979898288;
            } else { current_block_13 = 12800627514080957624; }
        } else if (*player).active as u64 == 0 {
            current_block_13 = 7502529970979898288;
        } else if playernum == i {
            current_block_13 = 7502529970979898288;
        } else { current_block_13 = 12800627514080957624; }
        match current_block_13 {
            12800627514080957624 =>
            // the player object never gets added
            {
                if !((*player).solid == 0 as libc::c_int) {
                    if (*clgame.pmove).numphysent >= 600 as libc::c_int {
                        break ; // dead body
                    }
                    pe =
                        &mut *(*clgame.pmove).physents.as_mut_ptr().offset((*clgame.pmove).numphysent
                                                                               as
                                                                               isize)
                            as *mut physent_t;
                    CL_CopyEntityToPhysEnt(pe, state, false_0);
                    (*clgame.pmove).numphysent += 1;
                    // some fields needs to be override from cls.predicted_players
                    (*pe).origin[0 as libc::c_int as usize] =
                        (*player).origin[0 as libc::c_int as usize];
                    (*pe).origin[1 as libc::c_int as usize] =
                        (*player).origin[1 as libc::c_int as usize];
                    (*pe).origin[2 as libc::c_int as usize] =
                        (*player).origin[2 as libc::c_int as usize];
                    (*pe).angles[0 as libc::c_int as usize] =
                        (*player).angles[0 as libc::c_int as usize];
                    (*pe).angles[1 as libc::c_int as usize] =
                        (*player).angles[1 as libc::c_int as usize];
                    (*pe).angles[2 as libc::c_int as usize] =
                        (*player).angles[2 as libc::c_int as usize];
                    (*pe).mins[0 as libc::c_int as usize] =
                        (*clgame.pmove).player_mins[(*player).usehull as
                                                        usize][0 as
                                                                   libc::c_int
                                                                   as usize];
                    (*pe).mins[1 as libc::c_int as usize] =
                        (*clgame.pmove).player_mins[(*player).usehull as
                                                        usize][1 as
                                                                   libc::c_int
                                                                   as usize];
                    (*pe).mins[2 as libc::c_int as usize] =
                        (*clgame.pmove).player_mins[(*player).usehull as
                                                        usize][2 as
                                                                   libc::c_int
                                                                   as usize];
                    (*pe).maxs[0 as libc::c_int as usize] =
                        (*clgame.pmove).player_maxs[(*player).usehull as
                                                        usize][0 as
                                                                   libc::c_int
                                                                   as usize];
                    (*pe).maxs[1 as libc::c_int as usize] =
                        (*clgame.pmove).player_maxs[(*player).usehull as
                                                        usize][1 as
                                                                   libc::c_int
                                                                   as usize];
                    (*pe).maxs[2 as libc::c_int as usize] =
                        (*clgame.pmove).player_maxs[(*player).usehull as
                                                        usize][2 as
                                                                   libc::c_int
                                                                   as usize];
                    (*pe).movetype = (*player).movetype;
                    (*pe).solid = (*player).solid
                }
            }
            _ => { }
        }
        i += 1
    };
}
/*
=============
CL_WaterEntity

=============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_WaterEntity(mut rgflPos: *const libc::c_float)
 -> libc::c_int {
    let mut pe: *mut physent_t = 0 as *mut physent_t;
    let mut hull: *mut hull_t = 0 as *mut hull_t;
    let mut test: vec3_t = [0.; 3];
    let mut offset: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    let mut oldhull: libc::c_int = 0;
    if rgflPos.is_null() { return -(1 as libc::c_int) }
    oldhull = (*clgame.pmove).usehull;
    i = 0 as libc::c_int;
    while i < (*clgame.pmove).numphysent {
        pe =
            &mut *(*clgame.pmove).physents.as_mut_ptr().offset(i as isize) as
                *mut physent_t;
        if !((*pe).solid != 0 as libc::c_int) {
            // only brushes can have special contents
            if !((*pe).model.is_null() ||
                     (*(*pe).model).type_0 as libc::c_int !=
                         mod_brush as libc::c_int) {
                // check water brushes accuracy
                (*clgame.pmove).usehull = 2 as libc::c_int;
                hull = PM_HullForBsp(pe, clgame.pmove, offset.as_mut_ptr());
                (*clgame.pmove).usehull = oldhull;
                // offset the test point appropriately for this hull.
                test[0 as libc::c_int as usize] =
                    *rgflPos.offset(0 as libc::c_int as isize) -
                        offset[0 as libc::c_int as usize];
                test[1 as libc::c_int as usize] =
                    *rgflPos.offset(1 as libc::c_int as isize) -
                        offset[1 as libc::c_int as usize];
                test[2 as libc::c_int as usize] =
                    *rgflPos.offset(2 as libc::c_int as isize) -
                        offset[2 as libc::c_int as usize];
                if (*(*pe).model).flags as libc::c_uint &
                       (1 as libc::c_uint) << 1 as libc::c_int != 0 &&
                       !((*pe).angles[0 as libc::c_int as usize] == 0.0f32 &&
                             (*pe).angles[1 as libc::c_int as usize] == 0.0f32
                             &&
                             (*pe).angles[2 as libc::c_int as usize] ==
                                 0.0f32) {
                    let mut matrix: matrix4x4 = [[0.; 4]; 4];
                    Matrix4x4_CreateFromEntity(matrix.as_mut_ptr(),
                                               (*pe).angles.as_mut_ptr() as
                                                   *const vec_t,
                                               offset.as_mut_ptr() as
                                                   *const vec_t, 1.0f32);
                    Matrix4x4_VectorITransform(matrix.as_mut_ptr() as
                                                   *const [vec_t; 4], rgflPos,
                                               test.as_mut_ptr());
                }
                // test hull for intersection with this model
                if !(PM_HullPointContents(hull, (*hull).firstclipnode,
                                          test.as_mut_ptr() as *const vec_t)
                         == -(1 as libc::c_int)) {
                    // found water entity
                    return (*pe).info
                }
            }
        }
        // disabled ?
        i += 1
    }
    return -(1 as libc::c_int);
}
/*
=============
CL_TraceLine

a simple engine traceline
=============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_TraceLine(mut start: *mut vec_t,
                                      mut end: *mut vec_t,
                                      mut flags: libc::c_int) -> pmtrace_t {
    let mut old_usehull: libc::c_int = 0;
    let mut tr: pmtrace_t =
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
    old_usehull = (*clgame.pmove).usehull;
    (*clgame.pmove).usehull = 2 as libc::c_int;
    tr =
        PM_PlayerTraceExt(clgame.pmove, start, end, flags,
                          (*clgame.pmove).numphysent,
                          (*clgame.pmove).physents.as_mut_ptr(),
                          -(1 as libc::c_int), None);
    (*clgame.pmove).usehull = old_usehull;
    return tr;
}
/*
=============
CL_VisTraceLine

trace by visible objects (thats can be non-solid)
=============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_VisTraceLine(mut start: *mut vec_t,
                                         mut end: *mut vec_t,
                                         mut flags: libc::c_int)
 -> *mut pmtrace_t {
    let mut old_usehull: libc::c_int = 0;
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
    old_usehull = (*clgame.pmove).usehull;
    (*clgame.pmove).usehull = 2 as libc::c_int;
    tr =
        PM_PlayerTraceExt(clgame.pmove, start, end, flags,
                          (*clgame.pmove).numvisent,
                          (*clgame.pmove).visents.as_mut_ptr(),
                          -(1 as libc::c_int), None);
    (*clgame.pmove).usehull = old_usehull;
    return &mut tr;
}
/*
=============
CL_GetWaterEntity

returns water brush where inside pos
=============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_GetWaterEntity(mut rgflPos: *const libc::c_float)
 -> *mut cl_entity_t {
    let mut entnum: libc::c_int = 0; // world or not water
    entnum = CL_WaterEntity(rgflPos); // bad ground
    if entnum <= 0 as libc::c_int { return 0 as *mut cl_entity_t }
    return CL_GetEntityByIndex(entnum);
}
#[no_mangle]
pub unsafe extern "C" fn CL_TestLine(mut start: *const vec_t,
                                     mut end: *const vec_t,
                                     mut flags: libc::c_int) -> libc::c_int {
    return PM_TestLineExt(clgame.pmove, (*clgame.pmove).physents.as_mut_ptr(),
                          (*clgame.pmove).numphysent, start, end, flags);
}
unsafe extern "C" fn pfnTestPlayerPosition(mut pos: *mut libc::c_float,
                                           mut ptrace: *mut pmtrace_t)
 -> libc::c_int {
    return PM_TestPlayerPosition(clgame.pmove, pos, ptrace, None);
}
unsafe extern "C" fn pfnStuckTouch(mut hitent: libc::c_int,
                                   mut tr: *mut pmtrace_t) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*clgame.pmove).numtouch {
        if (*clgame.pmove).touchindex[i as usize].ent == hitent { return }
        i += 1
    }
    if (*clgame.pmove).numtouch >= 600 as libc::c_int { return }
    (*tr).deltavelocity[0 as libc::c_int as usize] =
        (*clgame.pmove).velocity[0 as libc::c_int as usize];
    (*tr).deltavelocity[1 as libc::c_int as usize] =
        (*clgame.pmove).velocity[1 as libc::c_int as usize];
    (*tr).deltavelocity[2 as libc::c_int as usize] =
        (*clgame.pmove).velocity[2 as libc::c_int as usize];
    (*tr).ent = hitent;
    let fresh0 = (*clgame.pmove).numtouch;
    (*clgame.pmove).numtouch = (*clgame.pmove).numtouch + 1;
    (*clgame.pmove).touchindex[fresh0 as usize] = *tr;
}
unsafe extern "C" fn pfnPointContents(mut p: *mut libc::c_float,
                                      mut truecontents: *mut libc::c_int)
 -> libc::c_int {
    let mut cont: libc::c_int = 0;
    let mut truecont: libc::c_int = 0;
    cont = PM_PointContents(clgame.pmove, p as *const vec_t);
    truecont = cont;
    if !truecontents.is_null() { *truecontents = truecont }
    if cont <= -(9 as libc::c_int) && cont >= -(14 as libc::c_int) {
        cont = -(3 as libc::c_int)
    }
    return cont;
}
unsafe extern "C" fn pfnTruePointContents(mut p: *mut libc::c_float)
 -> libc::c_int {
    return PM_TruePointContents(clgame.pmove, p as *const vec_t);
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
    return PM_PlayerTraceExt(clgame.pmove, start, end, traceFlags,
                             (*clgame.pmove).numphysent,
                             (*clgame.pmove).physents.as_mut_ptr(), ignore_pe,
                             None);
}
#[no_mangle]
pub unsafe extern "C" fn PM_TraceLine(mut start: *mut libc::c_float,
                                      mut end: *mut libc::c_float,
                                      mut flags: libc::c_int,
                                      mut usehull: libc::c_int,
                                      mut ignore_pe: libc::c_int)
 -> *mut pmtrace_s {
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
    old_usehull = (*clgame.pmove).usehull;
    (*clgame.pmove).usehull = usehull;
    match flags {
        0 => {
            tr =
                PM_PlayerTraceExt(clgame.pmove, start, end, 0 as libc::c_int,
                                  (*clgame.pmove).numphysent,
                                  (*clgame.pmove).physents.as_mut_ptr(),
                                  ignore_pe, None)
        }
        1 => {
            tr =
                PM_PlayerTraceExt(clgame.pmove, start, end, 0 as libc::c_int,
                                  (*clgame.pmove).numvisent,
                                  (*clgame.pmove).visents.as_mut_ptr(),
                                  ignore_pe, None)
        }
        _ => { }
    }
    (*clgame.pmove).usehull = old_usehull;
    return &mut tr;
}
unsafe extern "C" fn pfnHullForBsp(mut pe: *mut physent_t,
                                   mut offset: *mut libc::c_float)
 -> *mut hull_t {
    return PM_HullForBsp(pe, clgame.pmove, offset);
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
    old_usehull = (*clgame.pmove).usehull;
    (*clgame.pmove).usehull = 2 as libc::c_int;
    hull = PM_HullForBsp(pe, clgame.pmove, offset.as_mut_ptr());
    (*clgame.pmove).usehull = old_usehull;
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
    if ground < 0 as libc::c_int || ground >= (*clgame.pmove).numphysent {
        return 0 as *const libc::c_char
    }
    pe =
        &mut *(*clgame.pmove).physents.as_mut_ptr().offset(ground as isize) as
            *mut physent_t;
    return PM_TraceTexture(pe, vstart, vend);
}
unsafe extern "C" fn pfnPlaySound(mut channel: libc::c_int,
                                  mut sample: *const libc::c_char,
                                  mut volume: libc::c_float,
                                  mut attenuation: libc::c_float,
                                  mut fFlags: libc::c_int,
                                  mut pitch: libc::c_int) {
    if (*clgame.pmove).runfuncs as u64 == 0 { return }
    S_StartSound(0 as *const vec_t,
                 (*clgame.pmove).player_index + 1 as libc::c_int, channel,
                 S_RegisterSound(sample), volume, attenuation, pitch, fFlags);
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
    CL_PlaybackEvent(flags, 0 as *const edict_t, eventindex, delay, origin,
                     angles, fparam1, fparam2, iparam1, iparam2, bparam1,
                     bparam2);
}
unsafe extern "C" fn pfnPlayerTraceEx(mut start: *mut libc::c_float,
                                      mut end: *mut libc::c_float,
                                      mut traceFlags: libc::c_int,
                                      mut pmFilter: pfnIgnore) -> pmtrace_t {
    return PM_PlayerTraceExt(clgame.pmove, start, end, traceFlags,
                             (*clgame.pmove).numphysent,
                             (*clgame.pmove).physents.as_mut_ptr(),
                             -(1 as libc::c_int), pmFilter);
}
unsafe extern "C" fn pfnTestPlayerPositionEx(mut pos: *mut libc::c_float,
                                             mut ptrace: *mut pmtrace_t,
                                             mut pmFilter: pfnIgnore)
 -> libc::c_int {
    return PM_TestPlayerPosition(clgame.pmove, pos, ptrace, pmFilter);
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
    old_usehull = (*clgame.pmove).usehull;
    (*clgame.pmove).usehull = usehull;
    match flags {
        0 => {
            tr =
                PM_PlayerTraceExt(clgame.pmove, start, end, 0 as libc::c_int,
                                  (*clgame.pmove).numphysent,
                                  (*clgame.pmove).physents.as_mut_ptr(),
                                  -(1 as libc::c_int), pmFilter)
        }
        1 => {
            tr =
                PM_PlayerTraceExt(clgame.pmove, start, end, 0 as libc::c_int,
                                  (*clgame.pmove).numvisent,
                                  (*clgame.pmove).visents.as_mut_ptr(),
                                  -(1 as libc::c_int), pmFilter)
        }
        _ => { }
    }
    (*clgame.pmove).usehull = old_usehull;
    return &mut tr;
}
/*
===============
CL_InitClientMove

===============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_InitClientMove() {
    let mut i: libc::c_int = 0; // running at client
    Pmove_Init();
    (*clgame.pmove).server = false_0;
    (*clgame.pmove).movevars = &mut clgame.movevars;
    (*clgame.pmove).runfuncs = false_0;
    // enumerate client hulls
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if clgame.dllFuncs.pfnGetHullBounds.expect("non-null function pointer")(i,
                                                                                host.player_mins[i
                                                                                                     as
                                                                                                     usize].as_mut_ptr(),
                                                                                host.player_maxs[i
                                                                                                     as
                                                                                                     usize].as_mut_ptr())
               != 0 {
            Con_Reportf(b"CL: hull%i, player_mins: %g %g %g, player_maxs: %g %g %g\n\x00"
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
    memcpy((*clgame.pmove).player_mins.as_mut_ptr() as *mut libc::c_void,
           host.player_mins.as_mut_ptr() as *const libc::c_void,
           ::std::mem::size_of::<[vec3_t; 4]>() as libc::c_ulong);
    memcpy((*clgame.pmove).player_maxs.as_mut_ptr() as *mut libc::c_void,
           host.player_maxs.as_mut_ptr() as *const libc::c_void,
           ::std::mem::size_of::<[vec3_t; 4]>() as libc::c_ulong);
    // common utilities
    (*clgame.pmove).PM_Info_ValueForKey =
        Some(Info_ValueForKey as
                 unsafe extern "C" fn(_: *const libc::c_char,
                                      _: *const libc::c_char)
                     ->
                         *const libc::c_char); // ref should be initialized here already
    (*clgame.pmove).PM_Particle =
        Some(CL_Particle as
                 unsafe extern "C" fn(_: *const vec_t, _: libc::c_int,
                                      _: libc::c_float, _: libc::c_int,
                                      _: libc::c_int) -> ());
    (*clgame.pmove).PM_TestPlayerPosition =
        Some(pfnTestPlayerPosition as
                 unsafe extern "C" fn(_: *mut libc::c_float,
                                      _: *mut pmtrace_t) -> libc::c_int);
    (*clgame.pmove).Con_NPrintf =
        Some(Con_NPrintf as
                 unsafe extern "C" fn(_: libc::c_int, _: *const libc::c_char,
                                      _: ...) -> ());
    (*clgame.pmove).Con_DPrintf =
        Some(Con_DPrintf as
                 unsafe extern "C" fn(_: *const libc::c_char, _: ...) -> ());
    (*clgame.pmove).Con_Printf =
        Some(Con_Printf as
                 unsafe extern "C" fn(_: *const libc::c_char, _: ...) -> ());
    (*clgame.pmove).Sys_FloatTime =
        Some(Sys_DoubleTime as unsafe extern "C" fn() -> libc::c_double);
    (*clgame.pmove).PM_StuckTouch =
        Some(pfnStuckTouch as
                 unsafe extern "C" fn(_: libc::c_int, _: *mut pmtrace_t)
                     -> ());
    (*clgame.pmove).PM_PointContents =
        Some(pfnPointContents as
                 unsafe extern "C" fn(_: *mut libc::c_float,
                                      _: *mut libc::c_int) -> libc::c_int);
    (*clgame.pmove).PM_TruePointContents =
        Some(pfnTruePointContents as
                 unsafe extern "C" fn(_: *mut libc::c_float) -> libc::c_int);
    (*clgame.pmove).PM_HullPointContents =
        Some(pfnHullPointContents as
                 unsafe extern "C" fn(_: *mut hull_s, _: libc::c_int,
                                      _: *mut libc::c_float) -> libc::c_int);
    (*clgame.pmove).PM_PlayerTrace =
        Some(pfnPlayerTrace as
                 unsafe extern "C" fn(_: *mut libc::c_float,
                                      _: *mut libc::c_float, _: libc::c_int,
                                      _: libc::c_int) -> pmtrace_t);
    (*clgame.pmove).PM_TraceLine =
        Some(PM_TraceLine as
                 unsafe extern "C" fn(_: *mut libc::c_float,
                                      _: *mut libc::c_float, _: libc::c_int,
                                      _: libc::c_int, _: libc::c_int)
                     -> *mut pmtrace_s);
    (*clgame.pmove).RandomLong =
        Some(COM_RandomLong as
                 unsafe extern "C" fn(_: libc::c_int, _: libc::c_int)
                     -> libc::c_int);
    (*clgame.pmove).RandomFloat =
        Some(COM_RandomFloat as
                 unsafe extern "C" fn(_: libc::c_float, _: libc::c_float)
                     -> libc::c_float);
    (*clgame.pmove).PM_GetModelType =
        Some(pfnGetModelType as
                 unsafe extern "C" fn(_: *mut model_t) -> libc::c_int);
    (*clgame.pmove).PM_GetModelBounds =
        Some(pfnGetModelBounds as
                 unsafe extern "C" fn(_: *mut model_t, _: *mut libc::c_float,
                                      _: *mut libc::c_float) -> ());
    (*clgame.pmove).PM_HullForBsp =
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
    (*clgame.pmove).PM_TraceModel =
        Some(pfnTraceModel as
                 unsafe extern "C" fn(_: *mut physent_t,
                                      _: *mut libc::c_float,
                                      _: *mut libc::c_float, _: *mut trace_t)
                     -> libc::c_float);
    (*clgame.pmove).COM_FileSize =
        Some(COM_FileSize as
                 unsafe extern "C" fn(_: *const libc::c_char) -> libc::c_int);
    (*clgame.pmove).COM_LoadFile =
        Some(COM_LoadFile as
                 unsafe extern "C" fn(_: *const libc::c_char, _: libc::c_int,
                                      _: *mut libc::c_int) -> *mut byte);
    (*clgame.pmove).COM_FreeFile =
        Some(COM_FreeFile as
                 unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    (*clgame.pmove).memfgets =
        Some(COM_MemFgets as
                 unsafe extern "C" fn(_: *mut byte, _: libc::c_int,
                                      _: *mut libc::c_int,
                                      _: *mut libc::c_char, _: libc::c_int)
                     -> *mut libc::c_char);
    (*clgame.pmove).PM_PlaySound =
        Some(pfnPlaySound as
                 unsafe extern "C" fn(_: libc::c_int, _: *const libc::c_char,
                                      _: libc::c_float, _: libc::c_float,
                                      _: libc::c_int, _: libc::c_int) -> ());
    (*clgame.pmove).PM_TraceTexture =
        Some(pfnTraceTexture as
                 unsafe extern "C" fn(_: libc::c_int, _: *mut libc::c_float,
                                      _: *mut libc::c_float)
                     -> *const libc::c_char);
    (*clgame.pmove).PM_PlaybackEventFull =
        Some(pfnPlaybackEventFull as
                 unsafe extern "C" fn(_: libc::c_int, _: libc::c_int, _: word,
                                      _: libc::c_float, _: *mut libc::c_float,
                                      _: *mut libc::c_float, _: libc::c_float,
                                      _: libc::c_float, _: libc::c_int,
                                      _: libc::c_int, _: libc::c_int,
                                      _: libc::c_int) -> ());
    (*clgame.pmove).PM_PlayerTraceEx =
        Some(pfnPlayerTraceEx as
                 unsafe extern "C" fn(_: *mut libc::c_float,
                                      _: *mut libc::c_float, _: libc::c_int,
                                      _: pfnIgnore) -> pmtrace_t);
    (*clgame.pmove).PM_TestPlayerPositionEx =
        Some(pfnTestPlayerPositionEx as
                 unsafe extern "C" fn(_: *mut libc::c_float,
                                      _: *mut pmtrace_t, _: pfnIgnore)
                     -> libc::c_int);
    (*clgame.pmove).PM_TraceLineEx =
        Some(pfnTraceLineEx as
                 unsafe extern "C" fn(_: *mut libc::c_float,
                                      _: *mut libc::c_float, _: libc::c_int,
                                      _: libc::c_int, _: pfnIgnore)
                     -> *mut pmtrace_t);
    (*clgame.pmove).PM_TraceSurface =
        Some(pfnTraceSurface as
                 unsafe extern "C" fn(_: libc::c_int, _: *mut libc::c_float,
                                      _: *mut libc::c_float)
                     -> *mut msurface_s);
    // initalize pmove
    clgame.dllFuncs.pfnPlayerMoveInit.expect("non-null function pointer")(clgame.pmove); // not used by PM_ code
}
#[no_mangle]
pub unsafe extern "C" fn CL_SetupPMove(mut pmove: *mut playermove_t,
                                       mut from: *mut local_state_t,
                                       mut ucmd: *mut usercmd_t,
                                       mut runfuncs: qboolean,
                                       mut time: libc::c_double) {
    let mut ps: *mut entity_state_t =
        0 as *mut entity_state_t; // copy current cmds
    let mut cd: *mut clientdata_t = 0 as *mut clientdata_t;
    ps = &mut (*from).playerstate;
    cd = &mut (*from).client;
    (*pmove).player_index = (*ps).number - 1 as libc::c_int;
    (*pmove).multiplayer =
        (cl.maxclients > 1 as libc::c_int) as libc::c_int as qboolean;
    (*pmove).runfuncs = runfuncs;
    (*pmove).time = (time * 1000.0f32 as libc::c_double) as libc::c_float;
    (*pmove).frametime =
        (*ucmd).msec as libc::c_int as libc::c_float / 1000.0f32;
    (*pmove).origin[0 as libc::c_int as usize] =
        (*ps).origin[0 as libc::c_int as usize];
    (*pmove).origin[1 as libc::c_int as usize] =
        (*ps).origin[1 as libc::c_int as usize];
    (*pmove).origin[2 as libc::c_int as usize] =
        (*ps).origin[2 as libc::c_int as usize];
    (*pmove).angles[0 as libc::c_int as usize] =
        (*ps).angles[0 as libc::c_int as usize];
    (*pmove).angles[1 as libc::c_int as usize] =
        (*ps).angles[1 as libc::c_int as usize];
    (*pmove).angles[2 as libc::c_int as usize] =
        (*ps).angles[2 as libc::c_int as usize];
    (*pmove).oldangles[0 as libc::c_int as usize] =
        (*pmove).angles[0 as libc::c_int as usize];
    (*pmove).oldangles[1 as libc::c_int as usize] =
        (*pmove).angles[1 as libc::c_int as usize];
    (*pmove).oldangles[2 as libc::c_int as usize] =
        (*pmove).angles[2 as libc::c_int as usize];
    (*pmove).velocity[0 as libc::c_int as usize] =
        (*cd).velocity[0 as libc::c_int as usize];
    (*pmove).velocity[1 as libc::c_int as usize] =
        (*cd).velocity[1 as libc::c_int as usize];
    (*pmove).velocity[2 as libc::c_int as usize] =
        (*cd).velocity[2 as libc::c_int as usize];
    (*pmove).basevelocity[0 as libc::c_int as usize] =
        (*ps).basevelocity[0 as libc::c_int as usize];
    (*pmove).basevelocity[1 as libc::c_int as usize] =
        (*ps).basevelocity[1 as libc::c_int as usize];
    (*pmove).basevelocity[2 as libc::c_int as usize] =
        (*ps).basevelocity[2 as libc::c_int as usize];
    (*pmove).view_ofs[0 as libc::c_int as usize] =
        (*cd).view_ofs[0 as libc::c_int as usize];
    (*pmove).view_ofs[1 as libc::c_int as usize] =
        (*cd).view_ofs[1 as libc::c_int as usize];
    (*pmove).view_ofs[2 as libc::c_int as usize] =
        (*cd).view_ofs[2 as libc::c_int as usize];
    (*pmove).movedir[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    (*pmove).movedir[1 as libc::c_int as usize] =
        (*pmove).movedir[2 as libc::c_int as usize];
    (*pmove).movedir[0 as libc::c_int as usize] =
        (*pmove).movedir[1 as libc::c_int as usize];
    (*pmove).flDuckTime = (*cd).flDuckTime as libc::c_float;
    (*pmove).bInDuck = (*cd).bInDuck as qboolean;
    (*pmove).usehull = (*ps).usehull;
    (*pmove).flTimeStepSound = (*cd).flTimeStepSound;
    (*pmove).iStepLeft = (*ps).iStepLeft;
    (*pmove).flFallVelocity = (*ps).flFallVelocity;
    (*pmove).flSwimTime = (*cd).flSwimTime as libc::c_float;
    (*pmove).punchangle[0 as libc::c_int as usize] =
        (*cd).punchangle[0 as libc::c_int as usize];
    (*pmove).punchangle[1 as libc::c_int as usize] =
        (*cd).punchangle[1 as libc::c_int as usize];
    (*pmove).punchangle[2 as libc::c_int as usize] =
        (*cd).punchangle[2 as libc::c_int as usize];
    (*pmove).flSwimTime = (*cd).flSwimTime as libc::c_float;
    (*pmove).flNextPrimaryAttack = 0.0f32;
    (*pmove).effects = (*ps).effects;
    (*pmove).flags = (*cd).flags;
    (*pmove).gravity = (*ps).gravity;
    (*pmove).friction = (*ps).friction;
    (*pmove).oldbuttons = (*ps).oldbuttons;
    (*pmove).waterjumptime = (*cd).waterjumptime as libc::c_float;
    (*pmove).dead =
        (cl.local.health <= 0 as libc::c_int) as libc::c_int as qboolean;
    (*pmove).deadflag = (*cd).deadflag;
    (*pmove).spectator =
        (cls.spectator as libc::c_uint != 0 as libc::c_int as libc::c_uint) as
            libc::c_int;
    (*pmove).movetype = (*ps).movetype;
    (*pmove).onground = (*ps).onground;
    (*pmove).waterlevel = (*cd).waterlevel;
    (*pmove).watertype = (*cd).watertype;
    (*pmove).maxspeed = clgame.movevars.maxspeed;
    (*pmove).clientmaxspeed = (*cd).maxspeed;
    (*pmove).iuser1 = (*cd).iuser1;
    (*pmove).iuser2 = (*cd).iuser2;
    (*pmove).iuser3 = (*cd).iuser3;
    (*pmove).iuser4 = (*cd).iuser4;
    (*pmove).fuser1 = (*cd).fuser1;
    (*pmove).fuser2 = (*cd).fuser2;
    (*pmove).fuser3 = (*cd).fuser3;
    (*pmove).fuser4 = (*cd).fuser4;
    (*pmove).vuser1[0 as libc::c_int as usize] =
        (*cd).vuser1[0 as libc::c_int as usize];
    (*pmove).vuser1[1 as libc::c_int as usize] =
        (*cd).vuser1[1 as libc::c_int as usize];
    (*pmove).vuser1[2 as libc::c_int as usize] =
        (*cd).vuser1[2 as libc::c_int as usize];
    (*pmove).vuser2[0 as libc::c_int as usize] =
        (*cd).vuser2[0 as libc::c_int as usize];
    (*pmove).vuser2[1 as libc::c_int as usize] =
        (*cd).vuser2[1 as libc::c_int as usize];
    (*pmove).vuser2[2 as libc::c_int as usize] =
        (*cd).vuser2[2 as libc::c_int as usize];
    (*pmove).vuser3[0 as libc::c_int as usize] =
        (*cd).vuser3[0 as libc::c_int as usize];
    (*pmove).vuser3[1 as libc::c_int as usize] =
        (*cd).vuser3[1 as libc::c_int as usize];
    (*pmove).vuser3[2 as libc::c_int as usize] =
        (*cd).vuser3[2 as libc::c_int as usize];
    (*pmove).vuser4[0 as libc::c_int as usize] =
        (*cd).vuser4[0 as libc::c_int as usize];
    (*pmove).vuser4[1 as libc::c_int as usize] =
        (*cd).vuser4[1 as libc::c_int as usize];
    (*pmove).vuser4[2 as libc::c_int as usize] =
        (*cd).vuser4[2 as libc::c_int as usize];
    (*pmove).cmd = *ucmd;
    Q_strncpy((*pmove).physinfo.as_mut_ptr(), cls.physinfo.as_mut_ptr(),
              256 as libc::c_int as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn CL_FinishPMove(mut pmove: *mut playermove_t,
                                        mut to: *mut local_state_t) {
    let mut ps: *mut entity_state_t = 0 as *mut entity_state_t;
    let mut cd: *mut clientdata_t = 0 as *mut clientdata_t;
    ps = &mut (*to).playerstate;
    cd = &mut (*to).client;
    (*cd).flags = (*pmove).flags;
    (*cd).bInDuck = (*pmove).bInDuck as libc::c_int;
    (*cd).flTimeStepSound = (*pmove).flTimeStepSound;
    (*cd).flDuckTime = (*pmove).flDuckTime as libc::c_int;
    (*cd).flSwimTime = (*pmove).flSwimTime as libc::c_int;
    (*cd).waterjumptime = (*pmove).waterjumptime as libc::c_int;
    (*cd).watertype = (*pmove).watertype;
    (*cd).waterlevel = (*pmove).waterlevel;
    (*cd).maxspeed = (*pmove).clientmaxspeed;
    (*cd).deadflag = (*pmove).deadflag;
    (*cd).velocity[0 as libc::c_int as usize] =
        (*pmove).velocity[0 as libc::c_int as usize];
    (*cd).velocity[1 as libc::c_int as usize] =
        (*pmove).velocity[1 as libc::c_int as usize];
    (*cd).velocity[2 as libc::c_int as usize] =
        (*pmove).velocity[2 as libc::c_int as usize];
    (*cd).view_ofs[0 as libc::c_int as usize] =
        (*pmove).view_ofs[0 as libc::c_int as usize];
    (*cd).view_ofs[1 as libc::c_int as usize] =
        (*pmove).view_ofs[1 as libc::c_int as usize];
    (*cd).view_ofs[2 as libc::c_int as usize] =
        (*pmove).view_ofs[2 as libc::c_int as usize];
    (*ps).origin[0 as libc::c_int as usize] =
        (*pmove).origin[0 as libc::c_int as usize];
    (*ps).origin[1 as libc::c_int as usize] =
        (*pmove).origin[1 as libc::c_int as usize];
    (*ps).origin[2 as libc::c_int as usize] =
        (*pmove).origin[2 as libc::c_int as usize];
    (*ps).angles[0 as libc::c_int as usize] =
        (*pmove).angles[0 as libc::c_int as usize];
    (*ps).angles[1 as libc::c_int as usize] =
        (*pmove).angles[1 as libc::c_int as usize];
    (*ps).angles[2 as libc::c_int as usize] =
        (*pmove).angles[2 as libc::c_int as usize];
    (*ps).basevelocity[0 as libc::c_int as usize] =
        (*pmove).basevelocity[0 as libc::c_int as usize];
    (*ps).basevelocity[1 as libc::c_int as usize] =
        (*pmove).basevelocity[1 as libc::c_int as usize];
    (*ps).basevelocity[2 as libc::c_int as usize] =
        (*pmove).basevelocity[2 as libc::c_int as usize];
    (*cd).punchangle[0 as libc::c_int as usize] =
        (*pmove).punchangle[0 as libc::c_int as usize];
    (*cd).punchangle[1 as libc::c_int as usize] =
        (*pmove).punchangle[1 as libc::c_int as usize];
    (*cd).punchangle[2 as libc::c_int as usize] =
        (*pmove).punchangle[2 as libc::c_int as usize];
    (*ps).oldbuttons = (*pmove).cmd.buttons as libc::c_int;
    (*ps).friction = (*pmove).friction;
    (*ps).movetype = (*pmove).movetype;
    (*ps).onground = (*pmove).onground;
    (*ps).effects = (*pmove).effects;
    (*ps).usehull = (*pmove).usehull;
    (*ps).iStepLeft = (*pmove).iStepLeft;
    (*ps).flFallVelocity = (*pmove).flFallVelocity;
    (*cd).iuser1 = (*pmove).iuser1;
    (*cd).iuser2 = (*pmove).iuser2;
    (*cd).iuser3 = (*pmove).iuser3;
    (*cd).iuser4 = (*pmove).iuser4;
    (*cd).fuser1 = (*pmove).fuser1;
    (*cd).fuser2 = (*pmove).fuser2;
    (*cd).fuser3 = (*pmove).fuser3;
    (*cd).fuser4 = (*pmove).fuser4;
    (*cd).vuser1[0 as libc::c_int as usize] =
        (*pmove).vuser1[0 as libc::c_int as usize];
    (*cd).vuser1[1 as libc::c_int as usize] =
        (*pmove).vuser1[1 as libc::c_int as usize];
    (*cd).vuser1[2 as libc::c_int as usize] =
        (*pmove).vuser1[2 as libc::c_int as usize];
    (*cd).vuser2[0 as libc::c_int as usize] =
        (*pmove).vuser2[0 as libc::c_int as usize];
    (*cd).vuser2[1 as libc::c_int as usize] =
        (*pmove).vuser2[1 as libc::c_int as usize];
    (*cd).vuser2[2 as libc::c_int as usize] =
        (*pmove).vuser2[2 as libc::c_int as usize];
    (*cd).vuser3[0 as libc::c_int as usize] =
        (*pmove).vuser3[0 as libc::c_int as usize];
    (*cd).vuser3[1 as libc::c_int as usize] =
        (*pmove).vuser3[1 as libc::c_int as usize];
    (*cd).vuser3[2 as libc::c_int as usize] =
        (*pmove).vuser3[2 as libc::c_int as usize];
    (*cd).vuser4[0 as libc::c_int as usize] =
        (*pmove).vuser4[0 as libc::c_int as usize];
    (*cd).vuser4[1 as libc::c_int as usize] =
        (*pmove).vuser4[1 as libc::c_int as usize];
    (*cd).vuser4[2 as libc::c_int as usize] =
        (*pmove).vuser4[2 as libc::c_int as usize];
}
/*
=================
CL_RunUsercmd

Runs prediction code for user cmd
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_RunUsercmd(mut from: *mut local_state_t,
                                       mut to: *mut local_state_t,
                                       mut u: *mut usercmd_t,
                                       mut runfuncs: qboolean,
                                       mut time: *mut libc::c_double,
                                       mut random_seed: libc::c_uint) {
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
                  impact_position: [0.; 3],}; // deal with local copy
    if (*u).msec as libc::c_int > 50 as libc::c_int {
        let mut temp: local_state_t =
            local_state_t{playerstate:
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
                                             rendercolor:
                                                 color24{r: 0, g: 0, b: 0,},
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
                                             vuser4: [0.; 3],},
                          client:
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
                                           vuser4: [0.; 3],},
                          weapondata:
                              [weapon_data_t{m_iId: 0,
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
                                             fuser4: 0.,}; 64],};
        let mut split: usercmd_t =
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
        memset(&mut temp as *mut local_state_t as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<local_state_t>() as libc::c_ulong);
        split = *u;
        split.msec = (split.msec as libc::c_int / 2 as libc::c_int) as byte;
        CL_RunUsercmd(from, &mut temp, &mut split, runfuncs, time,
                      random_seed);
        split.weaponselect = 0 as libc::c_int as byte;
        split.impulse = split.weaponselect;
        CL_RunUsercmd(&mut temp, to, &mut split, runfuncs, time, random_seed);
        return
    }
    cmd = *u;
    *to = *from;
    if CL_IsPredicted() as u64 != 0 {
        // setup playermove state
        CL_SetupPMove(clgame.pmove, from, &mut cmd, runfuncs, *time);
        // world(0) or in air(-1)
        clgame.dllFuncs.pfnPlayerMove.expect("non-null function pointer")(clgame.pmove,
                                                                          false_0
                                                                              as
                                                                              libc::c_int);
        CL_FinishPMove(clgame.pmove, to);
        if (*clgame.pmove).onground > 0 as libc::c_int &&
               (*clgame.pmove).onground < (*clgame.pmove).numphysent {
            cl.local.lastground =
                (*clgame.pmove).physents[(*clgame.pmove).onground as
                                             usize].info
        } else { cl.local.lastground = (*clgame.pmove).onground }
    }
    clgame.dllFuncs.pfnPostRunCmd.expect("non-null function pointer")(from,
                                                                      to,
                                                                      &mut cmd,
                                                                      runfuncs
                                                                          as
                                                                          libc::c_int,
                                                                      *time,
                                                                      random_seed);
    *time += cmd.msec as libc::c_double / 1000.0f64;
}
// motor!
// copy results back to client
/*
=================
CL_MoveSpectatorCamera

spectator movement code
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_MoveSpectatorCamera() {
    let mut time: libc::c_double = cl.time;
    if cls.spectator as u64 == 0 { return }
    CL_SetUpPlayerPrediction(false_0 as libc::c_int, true_0 as libc::c_int);
    CL_SetSolidPlayers(cl.playernum);
    CL_RunUsercmd(&mut cls.spectator_state, &mut cls.spectator_state, cl.cmd,
                  true_0, &mut time, (time * 100.0f64) as uint);
    cl.simvel[0 as libc::c_int as usize] =
        cls.spectator_state.client.velocity[0 as libc::c_int as usize];
    cl.simvel[1 as libc::c_int as usize] =
        cls.spectator_state.client.velocity[1 as libc::c_int as usize];
    cl.simvel[2 as libc::c_int as usize] =
        cls.spectator_state.client.velocity[2 as libc::c_int as usize];
    cl.simorg[0 as libc::c_int as usize] =
        cls.spectator_state.client.origin[0 as libc::c_int as usize];
    cl.simorg[1 as libc::c_int as usize] =
        cls.spectator_state.client.origin[1 as libc::c_int as usize];
    cl.simorg[2 as libc::c_int as usize] =
        cls.spectator_state.client.origin[2 as libc::c_int as usize];
    cl.punchangle[0 as libc::c_int as usize] =
        cls.spectator_state.client.punchangle[0 as libc::c_int as usize];
    cl.punchangle[1 as libc::c_int as usize] =
        cls.spectator_state.client.punchangle[1 as libc::c_int as usize];
    cl.punchangle[2 as libc::c_int as usize] =
        cls.spectator_state.client.punchangle[2 as libc::c_int as usize];
    cl.viewheight[0 as libc::c_int as usize] =
        cls.spectator_state.client.view_ofs[0 as libc::c_int as usize];
    cl.viewheight[1 as libc::c_int as usize] =
        cls.spectator_state.client.view_ofs[1 as libc::c_int as usize];
    cl.viewheight[2 as libc::c_int as usize] =
        cls.spectator_state.client.view_ofs[2 as libc::c_int as usize];
}
/*
=================
CL_PredictMovement

Sets cl.predicted.origin and cl.predicted.angles
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_PredictMovement(mut repredicting: qboolean) {
    let mut to_cmd: *mut runcmd_t = 0 as *mut runcmd_t;
    let mut from_cmd: *mut runcmd_t = 0 as *mut runcmd_t;
    let mut from: *mut local_state_t = 0 as *mut local_state_t;
    let mut to: *mut local_state_t = 0 as *mut local_state_t;
    let mut current_command: uint = 0;
    let mut current_command_mod: uint = 0;
    let mut frame: *mut frame_t = 0 as *mut frame_t;
    let mut i: uint = 0;
    let mut stoppoint: uint = 0;
    let mut runfuncs: qboolean = false_0;
    let mut f: libc::c_double = 1.0f64;
    let mut ent: *mut cl_entity_t = 0 as *mut cl_entity_t;
    let mut time: libc::c_double = 0.;
    if cls.state as libc::c_uint != ca_active as libc::c_int as libc::c_uint
           || cls.spectator as libc::c_uint != 0 {
        return
    }
    if cls.demoplayback != 0 && repredicting as u64 == 0 {
        CL_DemoInterpolateAngles();
    }
    CL_SetUpPlayerPrediction(false_0 as libc::c_int, false_0 as libc::c_int);
    if cls.state as libc::c_uint != ca_active as libc::c_int as libc::c_uint
           || cl.validsequence == 0 {
        return
    }
    if cls.netchan.outgoing_sequence.wrapping_sub(cls.netchan.incoming_acknowledged)
           >= (CL_UPDATE_BACKUP - 1 as libc::c_int) as libc::c_uint {
        return
    }
    // this is the last frame received from the server
    frame =
        &mut *cl.frames.as_mut_ptr().offset(cl.parsecountmod as isize) as
            *mut frame_t;
    if CL_IsPredicted() as u64 == 0 {
        cl.simvel[0 as libc::c_int as usize] =
            (*frame).clientdata.velocity[0 as libc::c_int as usize];
        cl.simvel[1 as libc::c_int as usize] =
            (*frame).clientdata.velocity[1 as libc::c_int as usize];
        cl.simvel[2 as libc::c_int as usize] =
            (*frame).clientdata.velocity[2 as libc::c_int as usize];
        cl.simorg[0 as libc::c_int as usize] =
            (*frame).clientdata.origin[0 as libc::c_int as usize];
        cl.simorg[1 as libc::c_int as usize] =
            (*frame).clientdata.origin[1 as libc::c_int as usize];
        cl.simorg[2 as libc::c_int as usize] =
            (*frame).clientdata.origin[2 as libc::c_int as usize];
        cl.punchangle[0 as libc::c_int as usize] =
            (*frame).clientdata.punchangle[0 as libc::c_int as usize];
        cl.punchangle[1 as libc::c_int as usize] =
            (*frame).clientdata.punchangle[1 as libc::c_int as usize];
        cl.punchangle[2 as libc::c_int as usize] =
            (*frame).clientdata.punchangle[2 as libc::c_int as usize];
        cl.viewheight[0 as libc::c_int as usize] =
            (*frame).clientdata.view_ofs[0 as libc::c_int as usize];
        cl.viewheight[1 as libc::c_int as usize] =
            (*frame).clientdata.view_ofs[1 as libc::c_int as usize];
        cl.viewheight[2 as libc::c_int as usize] =
            (*frame).clientdata.view_ofs[2 as libc::c_int as usize];
        cl.local.usehull =
            (*frame).playerstate[cl.playernum as usize].usehull;
        cl.local.waterlevel = (*frame).clientdata.waterlevel;
        if (*frame).clientdata.flags as libc::c_uint &
               (1 as libc::c_uint) << 9 as libc::c_int != 0 {
            cl.local.onground =
                (*frame).playerstate[cl.playernum as usize].onground
        } else { cl.local.onground = -(1 as libc::c_int) }
    }
    from =
        &mut *cl.predicted_frames.as_mut_ptr().offset(cl.parsecountmod as
                                                          isize) as
            *mut local_state_t;
    from_cmd =
        &mut *cl.commands.as_mut_ptr().offset((cls.netchan.incoming_acknowledged
                                                   &
                                                   (CL_UPDATE_BACKUP -
                                                        1 as libc::c_int) as
                                                       libc::c_uint) as isize)
            as *mut runcmd_t;
    memcpy((*from).weapondata.as_mut_ptr() as *mut libc::c_void,
           (*frame).weapondata.as_mut_ptr() as *const libc::c_void,
           ::std::mem::size_of::<[weapon_data_t; 64]>() as libc::c_ulong);
    (*from).playerstate = (*frame).playerstate[cl.playernum as usize];
    (*from).client = (*frame).clientdata;
    if (*frame).valid as u64 == 0 { return }
    time = (*frame).time;
    stoppoint =
        if repredicting as libc::c_uint != 0 {
            0 as libc::c_int
        } else { 1 as libc::c_int } as uint;
    cl.local.repredicting = repredicting;
    cl.local.onground = -(1 as libc::c_int);
    // predict forward until cl.time <= to->senttime
    CL_PushPMStates();
    CL_SetSolidPlayers(cl.playernum);
    i = 1 as libc::c_int as uint;
    while i < (CL_UPDATE_BACKUP - 1 as libc::c_int) as libc::c_uint &&
              cls.netchan.incoming_acknowledged.wrapping_add(i) <
                  cls.netchan.outgoing_sequence.wrapping_add(stoppoint) {
        current_command = cls.netchan.incoming_acknowledged.wrapping_add(i);
        current_command_mod =
            current_command &
                (CL_UPDATE_BACKUP - 1 as libc::c_int) as libc::c_uint;
        to =
            &mut *cl.predicted_frames.as_mut_ptr().offset(((cl.parsecountmod
                                                                as
                                                                libc::c_uint).wrapping_add(i)
                                                               &
                                                               (CL_UPDATE_BACKUP
                                                                    -
                                                                    1 as
                                                                        libc::c_int)
                                                                   as
                                                                   libc::c_uint)
                                                              as isize) as
                *mut local_state_t;
        to_cmd =
            &mut *cl.commands.as_mut_ptr().offset(current_command_mod as
                                                      isize) as *mut runcmd_t;
        runfuncs =
            (repredicting as u64 == 0 && (*to_cmd).processedfuncs as u64 == 0)
                as libc::c_int as qboolean;
        CL_RunUsercmd(from, to, &mut (*to_cmd).cmd, runfuncs, &mut time,
                      current_command);
        cl.local.predicted_origins[current_command_mod as
                                       usize][0 as libc::c_int as usize] =
            (*to).playerstate.origin[0 as libc::c_int as usize];
        cl.local.predicted_origins[current_command_mod as
                                       usize][1 as libc::c_int as usize] =
            (*to).playerstate.origin[1 as libc::c_int as usize];
        cl.local.predicted_origins[current_command_mod as
                                       usize][2 as libc::c_int as usize] =
            (*to).playerstate.origin[2 as libc::c_int as usize];
        (*to_cmd).processedfuncs = true_0;
        if (*to_cmd).senttime >= host.realtime { break ; }
        from = to;
        from_cmd = to_cmd;
        i = i.wrapping_add(1)
    }
    CL_PopPMStates();
    if i == (CL_UPDATE_BACKUP - 1 as libc::c_int) as libc::c_uint ||
           to.is_null() && repredicting as u64 == 0 {
        cl.local.repredicting = false_0;
        return
        // net hasn't deliver packets in a long time...
    }
    if to.is_null() { to = from; to_cmd = from_cmd }
    if CL_IsPredicted() as u64 == 0 {
        // keep onground actual
        if (*frame).clientdata.flags as libc::c_uint &
               (1 as libc::c_uint) << 9 as libc::c_int != 0 {
            cl.local.onground =
                (*frame).playerstate[cl.playernum as usize].onground
        } else { cl.local.onground = -(1 as libc::c_int) }
        if repredicting as u64 == 0 ||
               (if !cl_lw.is_null() && (*cl_lw).value != 0.0f32 {
                    true_0 as libc::c_int
                } else { false_0 as libc::c_int }) == 0 {
            cl.local.viewmodel = (*to).client.viewmodel
        }
        cl.local.repredicting = false_0;
        cl.local.moving = false_0 as libc::c_int;
        return
    }
    // now interpolate some fraction of the final frame
    if (*to_cmd).senttime != (*from_cmd).senttime {
        f =
            if (host.realtime - (*from_cmd).senttime) /
                   ((*to_cmd).senttime - (*from_cmd).senttime) * 0.1f64 >=
                   0.0f64 {
                if (host.realtime - (*from_cmd).senttime) /
                       ((*to_cmd).senttime - (*from_cmd).senttime) * 0.1f64 <
                       1.0f64 {
                    ((host.realtime - (*from_cmd).senttime) /
                         ((*to_cmd).senttime - (*from_cmd).senttime)) * 0.1f64
                } else { 1.0f64 }
            } else { 0.0f64 }
    } else { f = 0.0f64 }
    if CL_PlayerTeleported(from, to) as u64 != 0 {
        cl.simvel[0 as libc::c_int as usize] =
            (*to).client.velocity[0 as libc::c_int as usize];
        cl.simvel[1 as libc::c_int as usize] =
            (*to).client.velocity[1 as libc::c_int as usize];
        cl.simvel[2 as libc::c_int as usize] =
            (*to).client.velocity[2 as libc::c_int as usize];
        cl.simorg[0 as libc::c_int as usize] =
            (*to).playerstate.origin[0 as libc::c_int as usize];
        cl.simorg[1 as libc::c_int as usize] =
            (*to).playerstate.origin[1 as libc::c_int as usize];
        cl.simorg[2 as libc::c_int as usize] =
            (*to).playerstate.origin[2 as libc::c_int as usize];
        cl.punchangle[0 as libc::c_int as usize] =
            (*to).client.punchangle[0 as libc::c_int as usize];
        cl.punchangle[1 as libc::c_int as usize] =
            (*to).client.punchangle[1 as libc::c_int as usize];
        cl.punchangle[2 as libc::c_int as usize] =
            (*to).client.punchangle[2 as libc::c_int as usize];
        cl.viewheight[0 as libc::c_int as usize] =
            (*to).client.view_ofs[0 as libc::c_int as usize];
        cl.viewheight[1 as libc::c_int as usize] =
            (*to).client.view_ofs[1 as libc::c_int as usize];
        cl.viewheight[2 as libc::c_int as usize] =
            (*to).client.view_ofs[2 as libc::c_int as usize]
    } else {
        cl.simorg[0 as libc::c_int as usize] =
            ((*from).playerstate.origin[0 as libc::c_int as usize] as
                 libc::c_double +
                 f *
                     ((*to).playerstate.origin[0 as libc::c_int as usize] -
                          (*from).playerstate.origin[0 as libc::c_int as
                                                         usize]) as
                         libc::c_double) as vec_t;
        cl.simorg[1 as libc::c_int as usize] =
            ((*from).playerstate.origin[1 as libc::c_int as usize] as
                 libc::c_double +
                 f *
                     ((*to).playerstate.origin[1 as libc::c_int as usize] -
                          (*from).playerstate.origin[1 as libc::c_int as
                                                         usize]) as
                         libc::c_double) as vec_t;
        cl.simorg[2 as libc::c_int as usize] =
            ((*from).playerstate.origin[2 as libc::c_int as usize] as
                 libc::c_double +
                 f *
                     ((*to).playerstate.origin[2 as libc::c_int as usize] -
                          (*from).playerstate.origin[2 as libc::c_int as
                                                         usize]) as
                         libc::c_double) as vec_t;
        cl.simvel[0 as libc::c_int as usize] =
            ((*from).client.velocity[0 as libc::c_int as usize] as
                 libc::c_double +
                 f *
                     ((*to).client.velocity[0 as libc::c_int as usize] -
                          (*from).client.velocity[0 as libc::c_int as usize])
                         as libc::c_double) as vec_t;
        cl.simvel[1 as libc::c_int as usize] =
            ((*from).client.velocity[1 as libc::c_int as usize] as
                 libc::c_double +
                 f *
                     ((*to).client.velocity[1 as libc::c_int as usize] -
                          (*from).client.velocity[1 as libc::c_int as usize])
                         as libc::c_double) as vec_t;
        cl.simvel[2 as libc::c_int as usize] =
            ((*from).client.velocity[2 as libc::c_int as usize] as
                 libc::c_double +
                 f *
                     ((*to).client.velocity[2 as libc::c_int as usize] -
                          (*from).client.velocity[2 as libc::c_int as usize])
                         as libc::c_double) as vec_t;
        cl.punchangle[0 as libc::c_int as usize] =
            ((*from).client.punchangle[0 as libc::c_int as usize] as
                 libc::c_double +
                 f *
                     ((*to).client.punchangle[0 as libc::c_int as usize] -
                          (*from).client.punchangle[0 as libc::c_int as
                                                        usize]) as
                         libc::c_double) as vec_t;
        cl.punchangle[1 as libc::c_int as usize] =
            ((*from).client.punchangle[1 as libc::c_int as usize] as
                 libc::c_double +
                 f *
                     ((*to).client.punchangle[1 as libc::c_int as usize] -
                          (*from).client.punchangle[1 as libc::c_int as
                                                        usize]) as
                         libc::c_double) as vec_t;
        cl.punchangle[2 as libc::c_int as usize] =
            ((*from).client.punchangle[2 as libc::c_int as usize] as
                 libc::c_double +
                 f *
                     ((*to).client.punchangle[2 as libc::c_int as usize] -
                          (*from).client.punchangle[2 as libc::c_int as
                                                        usize]) as
                         libc::c_double) as vec_t;
        if (*from).playerstate.usehull == (*to).playerstate.usehull {
            cl.viewheight[0 as libc::c_int as usize] =
                ((*from).client.view_ofs[0 as libc::c_int as usize] as
                     libc::c_double +
                     f *
                         ((*to).client.view_ofs[0 as libc::c_int as usize] -
                              (*from).client.view_ofs[0 as libc::c_int as
                                                          usize]) as
                             libc::c_double) as vec_t;
            cl.viewheight[1 as libc::c_int as usize] =
                ((*from).client.view_ofs[1 as libc::c_int as usize] as
                     libc::c_double +
                     f *
                         ((*to).client.view_ofs[1 as libc::c_int as usize] -
                              (*from).client.view_ofs[1 as libc::c_int as
                                                          usize]) as
                             libc::c_double) as vec_t;
            cl.viewheight[2 as libc::c_int as usize] =
                ((*from).client.view_ofs[2 as libc::c_int as usize] as
                     libc::c_double +
                     f *
                         ((*to).client.view_ofs[2 as libc::c_int as usize] -
                              (*from).client.view_ofs[2 as libc::c_int as
                                                          usize]) as
                             libc::c_double) as vec_t
        } else {
            cl.viewheight[0 as libc::c_int as usize] =
                (*to).client.view_ofs[0 as libc::c_int as usize];
            cl.viewheight[1 as libc::c_int as usize] =
                (*to).client.view_ofs[1 as libc::c_int as usize];
            cl.viewheight[2 as libc::c_int as usize] =
                (*to).client.view_ofs[2 as libc::c_int as usize]
        }
    }
    cl.local.waterlevel = (*to).client.waterlevel;
    cl.local.usehull = (*to).playerstate.usehull;
    if repredicting as u64 == 0 ||
           (if !cl_lw.is_null() && (*cl_lw).value != 0.0f32 {
                true_0 as libc::c_int
            } else { false_0 as libc::c_int }) == 0 {
        cl.local.viewmodel = (*to).client.viewmodel
    }
    if (*to).client.flags as libc::c_uint &
           (1 as libc::c_uint) << 9 as libc::c_int != 0 {
        ent = CL_GetEntityByIndex(cl.local.lastground);
        cl.local.onground = cl.local.lastground;
        cl.local.moving = false_0 as libc::c_int;
        if !ent.is_null() {
            let mut delta: vec3_t = [0.; 3];
            delta[0 as libc::c_int as usize] =
                (*ent).curstate.origin[0 as libc::c_int as usize] -
                    (*ent).prevstate.origin[0 as libc::c_int as usize];
            delta[1 as libc::c_int as usize] =
                (*ent).curstate.origin[1 as libc::c_int as usize] -
                    (*ent).prevstate.origin[1 as libc::c_int as usize];
            delta[2 as libc::c_int as usize] = 0.0f32;
            if __tg_sqrt(delta[0 as libc::c_int as usize] *
                             delta[0 as libc::c_int as usize] +
                             delta[1 as libc::c_int as usize] *
                                 delta[1 as libc::c_int as usize] +
                             delta[2 as libc::c_int as usize] *
                                 delta[2 as libc::c_int as usize]) > 0.0f32 {
                cls.correction_time = 0 as libc::c_int as libc::c_double;
                cl.local.moving = true_0 as libc::c_int
            }
        }
    } else {
        cl.local.onground = -(1 as libc::c_int);
        cl.local.moving = 0 as libc::c_int
    }
    if cls.correction_time > 0 as libc::c_int as libc::c_double &&
           (*cl_nosmooth).value == 0. && (*cl_smoothtime).value != 0. {
        let mut delta_0: vec3_t = [0.; 3];
        let mut frac: libc::c_float = 0.;
        // only decay timer once per frame
        if repredicting as u64 == 0 { cls.correction_time -= host.frametime }
        // Make sure smoothtime is postive
        if (*cl_smoothtime).value <= 0.0f32 {
            Cvar_DirectSet(cl_smoothtime,
                           b"0.1\x00" as *const u8 as *const libc::c_char);
        }
        // Clamp from 0 to cl_smoothtime.value
        cls.correction_time =
            if cls.correction_time >= 0.0f64 {
                if cls.correction_time <
                       (*cl_smoothtime).value as libc::c_double {
                    cls.correction_time
                } else { (*cl_smoothtime).value as libc::c_double }
            } else { 0.0f64 };
        // Compute backward interpolation fraction along full correction
        frac =
            (1.0f32 as libc::c_double -
                 cls.correction_time /
                     (*cl_smoothtime).value as libc::c_double) as
                libc::c_float;
        // Determine how much error we still have to make up for
        delta_0[0 as libc::c_int as usize] =
            cl.simorg[0 as libc::c_int as usize] -
                cl.local.lastorigin[0 as libc::c_int as usize];
        delta_0[1 as libc::c_int as usize] =
            cl.simorg[1 as libc::c_int as usize] -
                cl.local.lastorigin[1 as libc::c_int as usize];
        delta_0[2 as libc::c_int as usize] =
            cl.simorg[2 as libc::c_int as usize] -
                cl.local.lastorigin[2 as libc::c_int as usize];
        // Scale the error by the backlerp fraction
        delta_0[0 as libc::c_int as usize] =
            delta_0[0 as libc::c_int as usize] * frac;
        delta_0[1 as libc::c_int as usize] =
            delta_0[1 as libc::c_int as usize] * frac;
        delta_0[2 as libc::c_int as usize] =
            delta_0[2 as libc::c_int as usize] * frac;
        // Go some fraction of the way
		// FIXME, Probably can't do this any more
        cl.simorg[0 as libc::c_int as usize] =
            cl.local.lastorigin[0 as libc::c_int as usize] +
                delta_0[0 as libc::c_int as usize];
        cl.simorg[1 as libc::c_int as usize] =
            cl.local.lastorigin[1 as libc::c_int as usize] +
                delta_0[1 as libc::c_int as usize];
        cl.simorg[2 as libc::c_int as usize] =
            cl.local.lastorigin[2 as libc::c_int as usize] +
                delta_0[2 as libc::c_int as usize]
    }
    cl.local.lastorigin[0 as libc::c_int as usize] =
        cl.simorg[0 as libc::c_int as usize];
    cl.local.lastorigin[1 as libc::c_int as usize] =
        cl.simorg[1 as libc::c_int as usize];
    cl.local.lastorigin[2 as libc::c_int as usize] =
        cl.simorg[2 as libc::c_int as usize];
    cl.local.repredicting = false_0;
}
