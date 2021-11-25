#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, extern_types,
           label_break_value, register_tool)]
extern "C" {
    pub type file_s;
    pub type grasshdr_s;
    pub type ref_viewpass_s;
    pub type mip_s;
    #[no_mangle]
    fn sqrtf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn fabsf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn fmodf(_: libc::c_float, _: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn Sys_DoubleTime() -> libc::c_double;
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    fn _Mem_Alloc(poolptr: poolhandle_t, size: size_t, clear: qboolean,
                  filename: *const libc::c_char, fileline: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    fn _Mem_Free(data: *mut libc::c_void, filename: *const libc::c_char,
                 fileline: libc::c_int);
    #[no_mangle]
    fn Mem_IsAllocatedExt(poolptr: poolhandle_t, data: *mut libc::c_void)
     -> qboolean;
    #[no_mangle]
    fn FS_Search(pattern: *const libc::c_char, caseinsensitive: libc::c_int,
                 gamedironly: libc::c_int) -> *mut search_t;
    #[no_mangle]
    fn FS_LoadImage(filename: *const libc::c_char, buffer: *const byte,
                    size: size_t) -> *mut rgbdata_t;
    #[no_mangle]
    fn FS_FreeImage(pack: *mut rgbdata_t);
    #[no_mangle]
    fn Host_Error(error: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Host_PrintEngineFeatures();
    #[no_mangle]
    fn COM_SaveFile(filename: *const libc::c_char, data: *const libc::c_void,
                    len: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn pfnDrawConsoleString(x: libc::c_int, y: libc::c_int,
                            string: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn pfnDrawSetTextColor(r: libc::c_float, g: libc::c_float,
                           b: libc::c_float);
    #[no_mangle]
    fn pfnDrawConsoleStringLen(pText: *const libc::c_char,
                               length: *mut libc::c_int,
                               height: *mut libc::c_int);
    #[no_mangle]
    fn Con_Reportf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_NXPrintf(info: *mut con_nprint_t, fmt: *const libc::c_char,
                    _: ...);
    #[no_mangle]
    fn Con_NPrintf(idx: libc::c_int, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn SV_StartSound(ent: *mut edict_t, chan: libc::c_int,
                     sample: *const libc::c_char, vol: libc::c_float,
                     attn: libc::c_float, flags: libc::c_int,
                     pitch: libc::c_int);
    #[no_mangle]
    fn SV_GetLightStyle(style: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn SV_TraceSurface(ent: *mut edict_t, start: *const vec_t,
                       end: *const vec_t) -> *mut msurface_t;
    #[no_mangle]
    fn SV_MoveNormal(start: *const vec_t, mins: *mut vec_t, maxs: *mut vec_t,
                     end: *const vec_t, type_0: libc::c_int, e: *mut edict_t)
     -> trace_t;
    #[no_mangle]
    fn SV_MoveNoEnts(start: *const vec_t, mins: *mut vec_t, maxs: *mut vec_t,
                     end: *const vec_t, type_0: libc::c_int, e: *mut edict_t)
     -> trace_t;
    #[no_mangle]
    fn SV_BoxInPVS(org: *const vec_t, absmin: *const vec_t,
                   absmax: *const vec_t) -> qboolean;
    #[no_mangle]
    fn pfnWriteBytes(bytes: *const byte, count: libc::c_int);
    #[no_mangle]
    fn SV_ClassName(e: *const edict_t) -> *const libc::c_char;
    #[no_mangle]
    fn Matrix4x4_VectorTransform(in_0: *const [vec_t; 4],
                                 v: *const libc::c_float,
                                 out: *mut libc::c_float);
    #[no_mangle]
    fn Matrix4x4_VectorITransform(in_0: *const [vec_t; 4],
                                  v: *const libc::c_float,
                                  out: *mut libc::c_float);
    #[no_mangle]
    fn Matrix4x4_ConcatTransforms(out: *mut [vec_t; 4],
                                  in1: *const [vec_t; 4],
                                  in2: *const [vec_t; 4]);
    #[no_mangle]
    fn Matrix4x4_CreateFromEntity(out: *mut [vec_t; 4], angles: *const vec_t,
                                  origin: *const vec_t, scale: libc::c_float);
    #[no_mangle]
    fn Matrix4x4_ConvertToEntity(in_0: *const [vec_t; 4], angles: *mut vec_t,
                                 origin: *mut vec_t);
    #[no_mangle]
    fn Matrix4x4_Invert_Simple(out: *mut [vec_t; 4], in1: *const [vec_t; 4]);
    #[no_mangle]
    static mut vec3_origin: vec3_t;
    #[no_mangle]
    fn VectorCompareEpsilon(vec1: *const vec_t, vec2: *const vec_t,
                            epsilon: vec_t) -> qboolean;
    #[no_mangle]
    fn Mod_CheckLump(filename: *const libc::c_char, lump: libc::c_int,
                     lumpsize: *mut libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Mod_ReadLump(filename: *const libc::c_char, lump: libc::c_int,
                    lumpdata: *mut *mut libc::c_void,
                    lumpsize: *mut libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Mod_SaveLump(filename: *const libc::c_char, lump: libc::c_int,
                    lumpdata: *mut libc::c_void, lumpsize: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    static mut svs: server_static_t;
    #[no_mangle]
    static mut sv: server_t;
    #[no_mangle]
    static mut sv_areanodes: [areanode_t; 0];
    #[no_mangle]
    static mut sv_friction: convar_t;
    #[no_mangle]
    static mut sv_gravity: convar_t;
    #[no_mangle]
    static mut sv_stopspeed: convar_t;
    #[no_mangle]
    static mut sv_maxvelocity: convar_t;
    #[no_mangle]
    static mut sv_check_errors: *mut convar_t;
    #[no_mangle]
    fn SV_ModelHandle(modelindex: libc::c_int) -> *mut model_t;
    #[no_mangle]
    fn SV_EdictNum(n: libc::c_int) -> *mut edict_t;
    #[no_mangle]
    fn SV_RunLightStyles();
    #[no_mangle]
    fn SV_FreeEdict(pEdict: *mut edict_t);
    #[no_mangle]
    fn SV_WaterMove(ent: *mut edict_t);
    #[no_mangle]
    fn SV_LinkEdict(ent: *mut edict_t, touch_triggers: qboolean);
    #[no_mangle]
    fn SV_CheckBottom(ent: *mut edict_t, iMode: libc::c_int) -> qboolean;
    #[no_mangle]
    fn SV_TestEntityPosition(ent: *mut edict_t, blocker: *mut edict_t)
     -> qboolean;
    #[no_mangle]
    fn SV_CopyTraceToGlobal(trace: *mut trace_t);
    #[no_mangle]
    fn SV_CheckEdict(e: *const edict_t, file: *const libc::c_char,
                     line: libc::c_int) -> qboolean;
    #[no_mangle]
    static mut svgame: svgame_static_t;
    #[no_mangle]
    fn SV_TruePointContents(p: *const vec_t) -> libc::c_int;
    #[no_mangle]
    fn SV_Move(start: *const vec_t, mins: *mut vec_t, maxs: *mut vec_t,
               end: *const vec_t, type_0: libc::c_int, e: *mut edict_t,
               monsterclip: qboolean) -> trace_t;
    #[no_mangle]
    fn SV_PointContents(p: *const vec_t) -> libc::c_int;
    #[no_mangle]
    fn SV_GetString(iString: string_t) -> *const libc::c_char;
    #[no_mangle]
    fn COM_GetProcAddress(hInstance: *mut libc::c_void,
                          name: *const libc::c_char) -> *mut libc::c_void;
    #[link_name = "ref"]
    static mut ref_0: ref_state_s;
    #[no_mangle]
    static mut gTriApi: triangleapi_t;
}
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type size_t = libc::c_ulong;
pub type HANDLE = *mut libc::c_void;
pub type HINSTANCE = *mut libc::c_void;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct search_t {
    pub numfilenames: libc::c_int,
    pub filenames: *mut *mut libc::c_char,
    pub filenamesbuffer: *mut libc::c_char,
}
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
pub type vec2_t = [vec_t; 2];
pub type vec3_t = [vec_t; 3];
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
pub type con_nprint_t = con_nprint_s;
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
pub struct server_physics_api_s {
    pub pfnLinkEdict: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                  _: qboolean) -> ()>,
    pub pfnGetServerTime: Option<unsafe extern "C" fn() -> libc::c_double>,
    pub pfnGetFrameTime: Option<unsafe extern "C" fn() -> libc::c_double>,
    pub pfnGetModel: Option<unsafe extern "C" fn(_: libc::c_int)
                                -> *mut libc::c_void>,
    pub pfnGetHeadnode: Option<unsafe extern "C" fn() -> *mut areanode_t>,
    pub pfnServerState: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub pfnHost_Error: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                   _: ...) -> ()>,
    pub pTriAPI: *mut triangleapi_s,
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
    pub Con_NPrintf: Option<unsafe extern "C" fn(_: libc::c_int,
                                                 _: *const libc::c_char,
                                                 _: ...) -> ()>,
    pub Con_NXPrintf: Option<unsafe extern "C" fn(_: *mut con_nprint_s,
                                                  _: *const libc::c_char,
                                                  _: ...) -> ()>,
    pub pfnGetLightStyle: Option<unsafe extern "C" fn(_: libc::c_int)
                                     -> *const libc::c_char>,
    pub pfnUpdateFogSettings: Option<unsafe extern "C" fn(_: libc::c_uint)
                                         -> ()>,
    pub pfnGetFilesList: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                     _: *mut libc::c_int,
                                                     _: libc::c_int)
                                    -> *mut *mut libc::c_char>,
    pub pfnTraceSurface: Option<unsafe extern "C" fn(_: *mut edict_t,
                                                     _: *const libc::c_float,
                                                     _: *const libc::c_float)
                                    -> *mut msurface_s>,
    pub pfnGetTextureData: Option<unsafe extern "C" fn(_: libc::c_uint)
                                      -> *const byte>,
    pub pfnMemAlloc: Option<unsafe extern "C" fn(_: size_t,
                                                 _: *const libc::c_char,
                                                 _: libc::c_int)
                                -> *mut libc::c_void>,
    pub pfnMemFree: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                _: *const libc::c_char,
                                                _: libc::c_int) -> ()>,
    pub pfnMaskPointContents: Option<unsafe extern "C" fn(_:
                                                              *const libc::c_float,
                                                          _: libc::c_int)
                                         -> libc::c_int>,
    pub pfnTrace: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                              _: *mut libc::c_float,
                                              _: *mut libc::c_float,
                                              _: *const libc::c_float,
                                              _: libc::c_int, _: *mut edict_t)
                             -> trace_t>,
    pub pfnTraceNoEnts: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                    _: *mut libc::c_float,
                                                    _: *mut libc::c_float,
                                                    _: *const libc::c_float,
                                                    _: libc::c_int,
                                                    _: *mut edict_t)
                                   -> trace_t>,
    pub pfnBoxInPVS: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                 _: *const libc::c_float,
                                                 _: *const libc::c_float)
                                -> libc::c_int>,
    pub pfnWriteBytes: Option<unsafe extern "C" fn(_: *const byte,
                                                   _: libc::c_int) -> ()>,
    pub pfnCheckLump: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                  _: libc::c_int,
                                                  _: *mut libc::c_int)
                                 -> libc::c_int>,
    pub pfnReadLump: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                 _: libc::c_int,
                                                 _: *mut *mut libc::c_void,
                                                 _: *mut libc::c_int)
                                -> libc::c_int>,
    pub pfnSaveLump: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                 _: libc::c_int,
                                                 _: *mut libc::c_void,
                                                 _: libc::c_int)
                                -> libc::c_int>,
    pub pfnSaveFile: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                 _: *const libc::c_void,
                                                 _: libc::c_int)
                                -> libc::c_int>,
    pub pfnLoadImagePixels: Option<unsafe extern "C" fn(_:
                                                            *const libc::c_char,
                                                        _: *mut libc::c_int,
                                                        _: *mut libc::c_int)
                                       -> *const byte>,
    pub pfnGetModelName: Option<unsafe extern "C" fn(_: libc::c_int)
                                    -> *const libc::c_char>,
}
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
pub type server_physics_api_t = server_physics_api_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpoint_t {
    pub point: vec2_t,
    pub coord: vec2_t,
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
pub struct mstudiotex_s {
    pub name: [libc::c_char; 64],
    pub flags: uint32_t,
    pub width: int32_t,
    pub height: int32_t,
    pub index: int32_t,
}
pub type triangleapi_t = triangleapi_s;
pub type cl_entity_t = cl_entity_s;
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
pub type BEAM = beam_s;
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
/*
sv_phys.c - server physic
Copyright (C) 2007 Uncle Mike

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
*/
pub type PHYSICAPI
    =
    Option<unsafe extern "C" fn(_: libc::c_int, _: *mut server_physics_api_t,
                                _: *mut physics_interface_t) -> libc::c_int>;
#[inline(always)]
unsafe extern "C" fn __tg_sqrt(mut __x: libc::c_float) -> libc::c_float {
    return sqrtf(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_fabs(mut __x: libc::c_float) -> libc::c_float {
    return fabsf(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_fmod(mut __x: libc::c_float, mut __y: libc::c_float)
 -> libc::c_float {
    return fmodf(__x, __y);
}
static mut current_table: [vec3_t; 6] =
    [[1 as libc::c_int as vec_t, 0 as libc::c_int as vec_t,
      0 as libc::c_int as vec_t],
     [0 as libc::c_int as vec_t, 1 as libc::c_int as vec_t,
      0 as libc::c_int as vec_t],
     [-(1 as libc::c_int) as vec_t, 0 as libc::c_int as vec_t,
      0 as libc::c_int as vec_t],
     [0 as libc::c_int as vec_t, -(1 as libc::c_int) as vec_t,
      0 as libc::c_int as vec_t],
     [0 as libc::c_int as vec_t, 0 as libc::c_int as vec_t,
      1 as libc::c_int as vec_t],
     [0 as libc::c_int as vec_t, 0 as libc::c_int as vec_t,
      -(1 as libc::c_int) as vec_t]];
/*
===============================================================================

Utility functions

===============================================================================
*/
/*
================
SV_CheckAllEnts
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_CheckAllEnts() {
    static mut nextcheck: libc::c_double = 0.;
    let mut e: *mut edict_t = 0 as *mut edict_t;
    let mut i: libc::c_int = 0;
    if (*sv_check_errors).value == 0. ||
           sv.state as libc::c_uint !=
               ss_active as libc::c_int as libc::c_uint {
        return
    }
    if nextcheck - Sys_DoubleTime() > 0.0f64 { return }
    // don't check entities every frame (but every 5 secs)
    nextcheck = Sys_DoubleTime() + 5.0f64;
    // check edicts errors
    i = svs.maxclients + 1 as libc::c_int;
    while i < svgame.numEntities {
        e = SV_EdictNum(i);
        if (*e).free as libc::c_uint != 0 && !(*e).pvPrivateData.is_null() {
            Con_Printf(b"^1Error:^7 Freed entity %s (%i) has private data.\n\x00"
                           as *const u8 as *const libc::c_char,
                       SV_ClassName(e), i);
        } else if !(SV_CheckEdict(e,
                                  b"../engine/server/sv_phys.c\x00" as
                                      *const u8 as *const libc::c_char,
                                  94 as libc::c_int) as u64 == 0) {
            if (*e).v.pContainingEntity.is_null() ||
                   (*e).v.pContainingEntity != e {
                Con_Printf(b"^1Error:^7 Entity %s (%i) has invalid container, fixed.\n\x00"
                               as *const u8 as *const libc::c_char,
                           SV_ClassName(e), i);
                (*e).v.pContainingEntity = e
            } else if (*e).pvPrivateData.is_null() ||
                          Mem_IsAllocatedExt(svgame.mempool,
                                             (*e).pvPrivateData) as u64 == 0 {
                Con_Printf(b"^1Error:^7 Entity %s (%i) trashed private data.\n\x00"
                               as *const u8 as *const libc::c_char,
                           SV_ClassName(e), i);
                (*e).pvPrivateData = 0 as *mut libc::c_void
            } else { SV_CheckVelocity(e); }
        }
        i += 1
    };
}
/*
================
SV_CheckVelocity
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_CheckVelocity(mut ent: *mut edict_t) {
    let mut wishspd: libc::c_float = 0.;
    let mut maxspd: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    // bound velocity
    i = 0 as libc::c_int; // half-diagonal
    while i < 3 as libc::c_int {
        if (*ent).v.velocity[i as usize].is_nan() as i32 != 0 {
            if (*sv_check_errors).value != 0. {
                Con_Printf(b"Got a NaN velocity on %s\n\x00" as *const u8 as
                               *const libc::c_char,
                           SV_GetString((*ent).v.classname));
            }
            (*ent).v.velocity[i as usize] = 0.0f32
        }
        if (*ent).v.origin[i as usize].is_nan() as i32 != 0 {
            if (*sv_check_errors).value != 0. {
                Con_Printf(b"Got a NaN origin on %s\n\x00" as *const u8 as
                               *const libc::c_char,
                           SV_GetString((*ent).v.classname));
            }
            (*ent).v.origin[i as usize] = 0.0f32
        }
        i += 1
    }
    wishspd =
        (*ent).v.velocity[0 as libc::c_int as usize] *
            (*ent).v.velocity[0 as libc::c_int as usize] +
            (*ent).v.velocity[1 as libc::c_int as usize] *
                (*ent).v.velocity[1 as libc::c_int as usize] +
            (*ent).v.velocity[2 as libc::c_int as usize] *
                (*ent).v.velocity[2 as libc::c_int as usize];
    maxspd = sv_maxvelocity.value * sv_maxvelocity.value * 1.73f32;
    if wishspd > maxspd {
        wishspd = __tg_sqrt(wishspd);
        if (*sv_check_errors).value != 0. {
            Con_Printf(b"Got a velocity too high on %s ( %.2f > %.2f )\n\x00"
                           as *const u8 as *const libc::c_char,
                       SV_GetString((*ent).v.classname),
                       wishspd as libc::c_double,
                       __tg_sqrt(maxspd) as libc::c_double);
        }
        wishspd = sv_maxvelocity.value / wishspd;
        (*ent).v.velocity[0 as libc::c_int as usize] =
            (*ent).v.velocity[0 as libc::c_int as usize] * wishspd;
        (*ent).v.velocity[1 as libc::c_int as usize] =
            (*ent).v.velocity[1 as libc::c_int as usize] * wishspd;
        (*ent).v.velocity[2 as libc::c_int as usize] =
            (*ent).v.velocity[2 as libc::c_int as usize] * wishspd
    };
}
/*
================
SV_UpdateBaseVelocity
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_UpdateBaseVelocity(mut ent: *mut edict_t) {
    if (*ent).v.flags as libc::c_uint &
           (1 as libc::c_uint) << 9 as libc::c_int != 0 {
        let mut groundentity: *mut edict_t = (*ent).v.groundentity;
        if SV_CheckEdict(groundentity,
                         b"../engine/server/sv_phys.c\x00" as *const u8 as
                             *const libc::c_char, 168 as libc::c_int) as u64
               != 0 {
            // On conveyor belt that's moving?
            if (*groundentity).v.flags as libc::c_uint &
                   (1 as libc::c_uint) << 2 as libc::c_int != 0 {
                let mut new_basevel: vec3_t = [0.; 3];
                new_basevel[0 as libc::c_int as usize] =
                    (*groundentity).v.movedir[0 as libc::c_int as usize] *
                        (*groundentity).v.speed;
                new_basevel[1 as libc::c_int as usize] =
                    (*groundentity).v.movedir[1 as libc::c_int as usize] *
                        (*groundentity).v.speed;
                new_basevel[2 as libc::c_int as usize] =
                    (*groundentity).v.movedir[2 as libc::c_int as usize] *
                        (*groundentity).v.speed;
                if (*ent).v.flags as libc::c_uint &
                       (1 as libc::c_uint) << 22 as libc::c_int != 0 {
                    new_basevel[0 as libc::c_int as usize] =
                        new_basevel[0 as libc::c_int as usize] +
                            (*ent).v.basevelocity[0 as libc::c_int as usize];
                    new_basevel[1 as libc::c_int as usize] =
                        new_basevel[1 as libc::c_int as usize] +
                            (*ent).v.basevelocity[1 as libc::c_int as usize];
                    new_basevel[2 as libc::c_int as usize] =
                        new_basevel[2 as libc::c_int as usize] +
                            (*ent).v.basevelocity[2 as libc::c_int as usize]
                }
                (*ent).v.flags =
                    ((*ent).v.flags as libc::c_uint |
                         (1 as libc::c_uint) << 22 as libc::c_int) as
                        libc::c_int;
                (*ent).v.basevelocity[0 as libc::c_int as usize] =
                    new_basevel[0 as libc::c_int as usize];
                (*ent).v.basevelocity[1 as libc::c_int as usize] =
                    new_basevel[1 as libc::c_int as usize];
                (*ent).v.basevelocity[2 as libc::c_int as usize] =
                    new_basevel[2 as libc::c_int as usize]
            }
        }
    };
}
/*
=============
SV_RunThink

Runs thinking code if time.  There is some play in the exact time the think
function will be called, because it is called before any movement is done
in a frame.  Not used for pushmove objects, because they must be exact.
Returns false if the entity removed itself.
=============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_RunThink(mut ent: *mut edict_t) -> qboolean {
    let mut thinktime: libc::c_float =
        0.; // don't let things stay in the past.
    if (*ent).v.flags as libc::c_uint &
           (1 as libc::c_uint) << 30 as libc::c_int == 0 {
        thinktime = (*ent).v.nextthink;
        if thinktime <= 0.0f32 ||
               thinktime as libc::c_double >
                   sv.time + sv.frametime as libc::c_double {
            return true_0
        }
        if (thinktime as libc::c_double) < sv.time {
            thinktime = sv.time as libc::c_float
        }
        // it is possible to start that way
						// by a trigger with a local time.
        (*ent).v.nextthink = 0.0f32;
        (*svgame.globals).time = thinktime;
        svgame.dllFuncs.pfnThink.expect("non-null function pointer")(ent);
    }
    if (*ent).v.flags as libc::c_uint &
           (1 as libc::c_uint) << 30 as libc::c_int != 0 {
        SV_FreeEdict(ent);
    }
    return ((*ent).free as u64 == 0) as libc::c_int as qboolean;
}
/*
=============
SV_PlayerRunThink

Runs thinking code if player time.  There is some play in the exact time the think
function will be called, because it is called before any movement is done
in a frame.  Not used for pushmove objects, because they must be exact.
Returns false if the entity removed itself.
=============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_PlayerRunThink(mut ent: *mut edict_t,
                                           mut frametime: libc::c_float,
                                           mut time: libc::c_double)
 -> qboolean {
    let mut thinktime: libc::c_float =
        0.; // don't let things stay in the past.
    if svgame.physFuncs.SV_PlayerThink.is_some() {
        return svgame.physFuncs.SV_PlayerThink.expect("non-null function pointer")(ent,
                                                                                   frametime,
                                                                                   time)
                   as qboolean
    }
    if (*ent).v.flags as libc::c_uint &
           ((1 as libc::c_uint) << 30 as libc::c_int |
                (1 as libc::c_uint) << 31 as libc::c_int) == 0 {
        thinktime = (*ent).v.nextthink;
        if thinktime <= 0.0f32 ||
               thinktime as libc::c_double >
                   time + frametime as libc::c_double {
            return true_0
        }
        if (thinktime as libc::c_double) < time {
            thinktime = time as libc::c_float
        }
        // it is possible to start that way
					// by a trigger with a local time.
        (*ent).v.nextthink = 0.0f32;
        (*svgame.globals).time = thinktime;
        svgame.dllFuncs.pfnThink.expect("non-null function pointer")(ent);
    }
    if (*ent).v.flags as libc::c_uint &
           (1 as libc::c_uint) << 30 as libc::c_int != 0 {
        (*ent).v.flags =
            ((*ent).v.flags as libc::c_uint &
                 !((1 as libc::c_uint) << 30 as libc::c_int)) as libc::c_int
    }
    return ((*ent).free as u64 == 0) as libc::c_int as qboolean;
}
/*
==================
SV_Impact

Two entities have touched, so run their touch functions
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_Impact(mut e1: *mut edict_t, mut e2: *mut edict_t,
                                   mut trace: *mut trace_t) {
    (*svgame.globals).time = sv.time as libc::c_float;
    if ((*e1).v.flags | (*e2).v.flags) as libc::c_uint &
           (1 as libc::c_uint) << 30 as libc::c_int != 0 {
        return
    }
    if (*e1).v.groupinfo != 0 && (*e2).v.groupinfo != 0 {
        if svs.groupop == 0 as libc::c_int &&
               (*e1).v.groupinfo & (*e2).v.groupinfo == 0 {
            return
        }
        if svs.groupop == 1 as libc::c_int &&
               (*e1).v.groupinfo & (*e2).v.groupinfo != 0 {
            return
        }
    }
    if (*e1).v.solid != 0 as libc::c_int {
        SV_CopyTraceToGlobal(trace);
        svgame.dllFuncs.pfnTouch.expect("non-null function pointer")(e1, e2);
    }
    if (*e2).v.solid != 0 as libc::c_int {
        SV_CopyTraceToGlobal(trace);
        svgame.dllFuncs.pfnTouch.expect("non-null function pointer")(e2, e1);
    };
}
/*
=============
SV_AngularMove

may use friction for smooth stopping
=============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_AngularMove(mut ent: *mut edict_t,
                                        mut frametime: libc::c_float,
                                        mut friction: libc::c_float) {
    let mut adjustment: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    (*ent).v.angles[0 as libc::c_int as usize] =
        (*ent).v.angles[0 as libc::c_int as usize] +
            frametime * (*ent).v.avelocity[0 as libc::c_int as usize];
    (*ent).v.angles[1 as libc::c_int as usize] =
        (*ent).v.angles[1 as libc::c_int as usize] +
            frametime * (*ent).v.avelocity[1 as libc::c_int as usize];
    (*ent).v.angles[2 as libc::c_int as usize] =
        (*ent).v.angles[2 as libc::c_int as usize] +
            frametime * (*ent).v.avelocity[2 as libc::c_int as usize];
    if friction == 0.0f32 { return }
    adjustment =
        frametime * (sv_stopspeed.value / 10.0f32) * sv_friction.value *
            __tg_fabs(friction);
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if (*ent).v.avelocity[i as usize] > 0.0f32 {
            (*ent).v.avelocity[i as usize] -= adjustment;
            if (*ent).v.avelocity[i as usize] < 0.0f32 {
                (*ent).v.avelocity[i as usize] = 0.0f32
            }
        } else {
            (*ent).v.avelocity[i as usize] += adjustment;
            if (*ent).v.avelocity[i as usize] > 0.0f32 {
                (*ent).v.avelocity[i as usize] = 0.0f32
            }
        }
        i += 1
    };
}
/*
=============
SV_LinearMove

use friction for smooth stopping
=============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_LinearMove(mut ent: *mut edict_t,
                                       mut frametime: libc::c_float,
                                       mut friction: libc::c_float) {
    let mut i: libc::c_int = 0;
    let mut adjustment: libc::c_float = 0.;
    (*ent).v.origin[0 as libc::c_int as usize] =
        (*ent).v.origin[0 as libc::c_int as usize] +
            frametime * (*ent).v.velocity[0 as libc::c_int as usize];
    (*ent).v.origin[1 as libc::c_int as usize] =
        (*ent).v.origin[1 as libc::c_int as usize] +
            frametime * (*ent).v.velocity[1 as libc::c_int as usize];
    (*ent).v.origin[2 as libc::c_int as usize] =
        (*ent).v.origin[2 as libc::c_int as usize] +
            frametime * (*ent).v.velocity[2 as libc::c_int as usize];
    if friction == 0.0f32 { return }
    adjustment =
        frametime * (sv_stopspeed.value / 10.0f32) * sv_friction.value *
            __tg_fabs(friction);
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if (*ent).v.velocity[i as usize] > 0.0f32 {
            (*ent).v.velocity[i as usize] -= adjustment;
            if (*ent).v.velocity[i as usize] < 0.0f32 {
                (*ent).v.velocity[i as usize] = 0.0f32
            }
        } else {
            (*ent).v.velocity[i as usize] += adjustment;
            if (*ent).v.velocity[i as usize] > 0.0f32 {
                (*ent).v.velocity[i as usize] = 0.0f32
            }
        }
        i += 1
    };
}
/*
=============
SV_RecursiveWaterLevel

recursively recalculating the middle
=============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_RecursiveWaterLevel(mut origin: *mut vec_t,
                                                mut out: libc::c_float,
                                                mut in_0: libc::c_float,
                                                mut count: libc::c_int)
 -> libc::c_float {
    let mut point: vec3_t = [0.; 3];
    let mut offset: libc::c_float = 0.;
    offset = (out - in_0) * 0.5f32 + in_0;
    count += 1;
    if count > 5 as libc::c_int { return offset }
    point[0 as libc::c_int as usize] =
        *origin.offset(0 as libc::c_int as isize);
    point[1 as libc::c_int as usize] =
        *origin.offset(1 as libc::c_int as isize);
    point[2 as libc::c_int as usize] =
        *origin.offset(2 as libc::c_int as isize) + offset;
    if SV_PointContents(point.as_mut_ptr() as *const vec_t) ==
           -(3 as libc::c_int) {
        return SV_RecursiveWaterLevel(origin, out, offset, count)
    }
    return SV_RecursiveWaterLevel(origin, offset, in_0, count);
}
/*
=============
SV_Submerged

determine how deep the entity is
=============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_Submerged(mut ent: *mut edict_t)
 -> libc::c_float {
    let mut start: libc::c_float = 0.;
    let mut bottom: libc::c_float = 0.;
    let mut point: vec3_t = [0.; 3];
    let mut center: vec3_t = [0.; 3];
    center[0 as libc::c_int as usize] =
        ((*ent).v.absmin[0 as libc::c_int as usize] +
             (*ent).v.absmax[0 as libc::c_int as usize]) * 0.5f32;
    center[1 as libc::c_int as usize] =
        ((*ent).v.absmin[1 as libc::c_int as usize] +
             (*ent).v.absmax[1 as libc::c_int as usize]) * 0.5f32;
    center[2 as libc::c_int as usize] =
        ((*ent).v.absmin[2 as libc::c_int as usize] +
             (*ent).v.absmax[2 as libc::c_int as usize]) * 0.5f32;
    start =
        (*ent).v.absmin[2 as libc::c_int as usize] -
            center[2 as libc::c_int as usize];
    's_60:
        {
            match (*ent).v.waterlevel {
                1 => {
                    bottom =
                        SV_RecursiveWaterLevel(center.as_mut_ptr(), 0.0f32,
                                               start, 0 as libc::c_int);
                    return bottom - start
                }
                3 => {
                    point[0 as libc::c_int as usize] =
                        center[0 as libc::c_int as usize];
                    point[1 as libc::c_int as usize] =
                        center[1 as libc::c_int as usize];
                    point[2 as libc::c_int as usize] =
                        (*ent).v.absmax[2 as libc::c_int as usize];
                    svs.groupmask = (*ent).v.groupinfo;
                    if SV_PointContents(point.as_mut_ptr() as *const vec_t) ==
                           -(3 as libc::c_int) {
                        return (*ent).v.maxs[2 as libc::c_int as usize] -
                                   (*ent).v.mins[2 as libc::c_int as usize]
                    }
                }
                2 => { }
                _ => { break 's_60 ; }
            }
            // intentionally fallthrough
            bottom =
                SV_RecursiveWaterLevel(center.as_mut_ptr(),
                                       (*ent).v.absmax[2 as libc::c_int as
                                                           usize] -
                                           center[2 as libc::c_int as usize],
                                       0.0f32, 0 as libc::c_int);
            return bottom - start
        }
    return 0.0f32;
}
/*
=============
SV_CheckWater
=============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_CheckWater(mut ent: *mut edict_t) -> qboolean {
    let mut cont: libc::c_int = 0;
    let mut truecont: libc::c_int = 0;
    let mut point: vec3_t = [0.; 3];
    point[0 as libc::c_int as usize] =
        ((*ent).v.absmax[0 as libc::c_int as usize] +
             (*ent).v.absmin[0 as libc::c_int as usize]) * 0.5f32;
    point[1 as libc::c_int as usize] =
        ((*ent).v.absmax[1 as libc::c_int as usize] +
             (*ent).v.absmin[1 as libc::c_int as usize]) * 0.5f32;
    point[2 as libc::c_int as usize] =
        (*ent).v.absmin[2 as libc::c_int as usize] + 1.0f32;
    (*ent).v.watertype = -(1 as libc::c_int);
    svs.groupmask = (*ent).v.groupinfo;
    (*ent).v.waterlevel = 0 as libc::c_int;
    cont = SV_PointContents(point.as_mut_ptr() as *const vec_t);
    if cont <= -(3 as libc::c_int) && cont > -(15 as libc::c_int) {
        svs.groupmask = (*ent).v.groupinfo;
        truecont = SV_TruePointContents(point.as_mut_ptr() as *const vec_t);
        (*ent).v.watertype = cont;
        (*ent).v.waterlevel = 1 as libc::c_int;
        if (*ent).v.absmin[2 as libc::c_int as usize] !=
               (*ent).v.absmax[2 as libc::c_int as usize] {
            point[2 as libc::c_int as usize] =
                ((*ent).v.absmin[2 as libc::c_int as usize] +
                     (*ent).v.absmax[2 as libc::c_int as usize]) * 0.5f32;
            svs.groupmask = (*ent).v.groupinfo;
            cont = SV_PointContents(point.as_mut_ptr() as *const vec_t);
            if cont <= -(3 as libc::c_int) && cont > -(15 as libc::c_int) {
                (*ent).v.waterlevel = 2 as libc::c_int;
                point[0 as libc::c_int as usize] =
                    point[0 as libc::c_int as usize] +
                        (*ent).v.view_ofs[0 as libc::c_int as usize];
                point[1 as libc::c_int as usize] =
                    point[1 as libc::c_int as usize] +
                        (*ent).v.view_ofs[1 as libc::c_int as usize];
                point[2 as libc::c_int as usize] =
                    point[2 as libc::c_int as usize] +
                        (*ent).v.view_ofs[2 as libc::c_int as usize];
                svs.groupmask = (*ent).v.groupinfo;
                cont = SV_PointContents(point.as_mut_ptr() as *const vec_t);
                if cont <= -(3 as libc::c_int) && cont > -(15 as libc::c_int)
                   {
                    (*ent).v.waterlevel = 3 as libc::c_int
                }
            }
        } else {
            // a point entity
            (*ent).v.waterlevel = 3 as libc::c_int
        }
        // Quake2 feature. Probably never was used in Half-Life...
        if truecont <= -(9 as libc::c_int) && truecont >= -(14 as libc::c_int)
           {
            let mut speed: libc::c_float =
                150.0f32 * (*ent).v.waterlevel as libc::c_float / 3.0f32;
            let mut dir: *const libc::c_float =
                current_table[(-(9 as libc::c_int) - truecont) as
                                  usize].as_ptr();
            (*ent).v.basevelocity[0 as libc::c_int as usize] =
                (*ent).v.basevelocity[0 as libc::c_int as usize] +
                    speed * *dir.offset(0 as libc::c_int as isize);
            (*ent).v.basevelocity[1 as libc::c_int as usize] =
                (*ent).v.basevelocity[1 as libc::c_int as usize] +
                    speed * *dir.offset(1 as libc::c_int as isize);
            (*ent).v.basevelocity[2 as libc::c_int as usize] =
                (*ent).v.basevelocity[2 as libc::c_int as usize] +
                    speed * *dir.offset(2 as libc::c_int as isize)
        }
    }
    return ((*ent).v.waterlevel > 1 as libc::c_int) as libc::c_int as
               qboolean;
}
/*
=============
SV_CheckMover

test thing (applies the friction to pushables while standing on moving platform)
=============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_CheckMover(mut ent: *mut edict_t) -> qboolean {
    let mut gnd: *mut edict_t = (*ent).v.groundentity;
    if SV_CheckEdict(gnd,
                     b"../engine/server/sv_phys.c\x00" as *const u8 as
                         *const libc::c_char, 498 as libc::c_int) as u64 == 0
       {
        return false_0
    }
    if (*gnd).v.movetype != 7 as libc::c_int { return false_0 }
    if (*gnd).v.velocity[0 as libc::c_int as usize] == 0.0f32 &&
           (*gnd).v.velocity[1 as libc::c_int as usize] == 0.0f32 &&
           (*gnd).v.velocity[2 as libc::c_int as usize] == 0.0f32 &&
           ((*gnd).v.avelocity[0 as libc::c_int as usize] == 0.0f32 &&
                (*gnd).v.avelocity[1 as libc::c_int as usize] == 0.0f32 &&
                (*gnd).v.avelocity[2 as libc::c_int as usize] == 0.0f32) {
        return false_0
    }
    return true_0;
}
/*
==================
SV_ClipVelocity

Slide off of the impacting object
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_ClipVelocity(mut in_0: *mut vec_t,
                                         mut normal: *mut vec_t,
                                         mut out: *mut vec_t,
                                         mut overbounce: libc::c_float)
 -> libc::c_int {
    let mut backoff: libc::c_float = 0.; // floor
    let mut change: libc::c_float = 0.; // step
    let mut i: libc::c_int = 0;
    let mut blocked: libc::c_int = 0;
    blocked = 0 as libc::c_int;
    if *normal.offset(2 as libc::c_int as isize) > 0.0f32 {
        blocked |= 1 as libc::c_int
    }
    if *normal.offset(2 as libc::c_int as isize) == 0. {
        blocked |= 2 as libc::c_int
    }
    backoff =
        (*in_0.offset(0 as libc::c_int as isize) *
             *normal.offset(0 as libc::c_int as isize) +
             *in_0.offset(1 as libc::c_int as isize) *
                 *normal.offset(1 as libc::c_int as isize) +
             *in_0.offset(2 as libc::c_int as isize) *
                 *normal.offset(2 as libc::c_int as isize)) * overbounce;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        change = *normal.offset(i as isize) * backoff;
        *out.offset(i as isize) = *in_0.offset(i as isize) - change;
        if *out.offset(i as isize) > -1.0f32 &&
               *out.offset(i as isize) < 1.0f32 {
            *out.offset(i as isize) = 0.0f32
        }
        i += 1
    }
    return blocked;
}
/*
===============================================================================

	FLYING MOVEMENT CODE

===============================================================================
*/
/*
============
SV_FlyMove

The basic solid body movement clip that slides along multiple planes
*steptrace - if not NULL, the trace results of any vertical wall hit will be stored
Returns the clipflags if the velocity was modified (hit something solid)
1 = floor
2 = wall / step
4 = dead stop
============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_FlyMove(mut ent: *mut edict_t,
                                    mut time: libc::c_float,
                                    mut steptrace: *mut trace_t)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut numplanes: libc::c_int = 0;
    let mut bumpcount: libc::c_int = 0;
    let mut blocked: libc::c_int = 0;
    let mut dir: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut planes: [vec3_t; 5] = [[0.; 3]; 5];
    let mut primal_velocity: vec3_t = [0.; 3];
    let mut original_velocity: vec3_t = [0.; 3];
    let mut new_velocity: vec3_t = [0.; 3];
    let mut d: libc::c_float = 0.;
    let mut time_left: libc::c_float = 0.;
    let mut allFraction: libc::c_float = 0.;
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
    blocked = 0 as libc::c_int;
    monsterClip =
        if (*ent).v.flags as libc::c_uint &
               (1 as libc::c_uint) << 23 as libc::c_int != 0 {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int } as qboolean;
    original_velocity[0 as libc::c_int as usize] =
        (*ent).v.velocity[0 as libc::c_int as usize];
    original_velocity[1 as libc::c_int as usize] =
        (*ent).v.velocity[1 as libc::c_int as usize];
    original_velocity[2 as libc::c_int as usize] =
        (*ent).v.velocity[2 as libc::c_int as usize];
    primal_velocity[0 as libc::c_int as usize] =
        (*ent).v.velocity[0 as libc::c_int as usize];
    primal_velocity[1 as libc::c_int as usize] =
        (*ent).v.velocity[1 as libc::c_int as usize];
    primal_velocity[2 as libc::c_int as usize] =
        (*ent).v.velocity[2 as libc::c_int as usize];
    numplanes = 0 as libc::c_int;
    allFraction = 0.0f32;
    time_left = time;
    bumpcount = 0 as libc::c_int;
    while bumpcount < 5 as libc::c_int - 1 as libc::c_int {
        if (*ent).v.velocity[0 as libc::c_int as usize] == 0.0f32 &&
               (*ent).v.velocity[1 as libc::c_int as usize] == 0.0f32 &&
               (*ent).v.velocity[2 as libc::c_int as usize] == 0.0f32 {
            break ;
        }
        end[0 as libc::c_int as usize] =
            (*ent).v.origin[0 as libc::c_int as usize] +
                time_left * (*ent).v.velocity[0 as libc::c_int as usize];
        end[1 as libc::c_int as usize] =
            (*ent).v.origin[1 as libc::c_int as usize] +
                time_left * (*ent).v.velocity[1 as libc::c_int as usize];
        end[2 as libc::c_int as usize] =
            (*ent).v.origin[2 as libc::c_int as usize] +
                time_left * (*ent).v.velocity[2 as libc::c_int as usize];
        trace =
            SV_Move((*ent).v.origin.as_mut_ptr() as *const vec_t,
                    (*ent).v.mins.as_mut_ptr(), (*ent).v.maxs.as_mut_ptr(),
                    end.as_mut_ptr() as *const vec_t, 0 as libc::c_int, ent,
                    monsterClip);
        allFraction += trace.fraction;
        if trace.allsolid as u64 != 0 {
            // entity is trapped in another solid
            (*ent).v.velocity[2 as libc::c_int as usize] =
                0 as libc::c_int as vec_t;
            (*ent).v.velocity[1 as libc::c_int as usize] =
                (*ent).v.velocity[2 as libc::c_int as usize];
            (*ent).v.velocity[0 as libc::c_int as usize] =
                (*ent).v.velocity[1 as libc::c_int as usize];
            return 4 as libc::c_int
        }
        if trace.fraction > 0.0f32 {
            // actually covered some distance
            (*ent).v.origin[0 as libc::c_int as usize] =
                trace.endpos[0 as libc::c_int as
                                 usize]; // moved the entire distance
            (*ent).v.origin[1 as libc::c_int as usize] =
                trace.endpos[1 as libc::c_int as
                                 usize]; // g-cont. this should never happens
            (*ent).v.origin[2 as libc::c_int as usize] =
                trace.endpos[2 as libc::c_int as usize]; // floor
            original_velocity[0 as libc::c_int as usize] =
                (*ent).v.velocity[0 as libc::c_int as usize]; // step
            original_velocity[1 as libc::c_int as usize] =
                (*ent).v.velocity[1 as libc::c_int as usize];
            original_velocity[2 as libc::c_int as usize] =
                (*ent).v.velocity[2 as libc::c_int as usize];
            numplanes = 0 as libc::c_int
        }
        if trace.fraction == 1.0f32 { break ; }
        if SV_CheckEdict(trace.ent,
                         b"../engine/server/sv_phys.c\x00" as *const u8 as
                             *const libc::c_char, 606 as libc::c_int) as u64
               == 0 {
            break ;
        }
        if trace.plane.normal[2 as libc::c_int as usize] > 0.7f32 {
            blocked |= 1 as libc::c_int;
            if (*trace.ent).v.solid == 4 as libc::c_int ||
                   (*trace.ent).v.solid == 3 as libc::c_int ||
                   (*trace.ent).v.movetype == 13 as libc::c_int ||
                   (*trace.ent).v.flags as libc::c_uint &
                       (1 as libc::c_uint) << 3 as libc::c_int != 0 {
                (*ent).v.flags =
                    ((*ent).v.flags as libc::c_uint |
                         (1 as libc::c_uint) << 9 as libc::c_int) as
                        libc::c_int;
                (*ent).v.groundentity = trace.ent
            }
        }
        if trace.plane.normal[2 as libc::c_int as usize] == 0.0f32 {
            blocked |= 2 as libc::c_int;
            if !steptrace.is_null() { *steptrace = trace }
            // save for player extrafriction
        }
        // run the impact function
        SV_Impact(ent, trace.ent, &mut trace);
        // break if removed by the impact function
        if (*ent).free as u64 != 0 { break ; }
        time_left -= time_left * trace.fraction;
        // clipped to another plane
        if numplanes >= 5 as libc::c_int {
            // this shouldn't really happen
            (*ent).v.velocity[2 as libc::c_int as usize] =
                0 as libc::c_int as vec_t;
            (*ent).v.velocity[1 as libc::c_int as usize] =
                (*ent).v.velocity[2 as libc::c_int as usize];
            (*ent).v.velocity[0 as libc::c_int as usize] =
                (*ent).v.velocity[1 as libc::c_int as usize];
            break ;
        } else {
            planes[numplanes as usize][0 as libc::c_int as usize] =
                trace.plane.normal[0 as libc::c_int as usize];
            planes[numplanes as usize][1 as libc::c_int as usize] =
                trace.plane.normal[1 as libc::c_int as usize];
            planes[numplanes as usize][2 as libc::c_int as usize] =
                trace.plane.normal[2 as libc::c_int as usize];
            numplanes += 1;
            // modify original_velocity so it parallels all of the clip planes
            i = 0 as libc::c_int;
            while i < numplanes {
                SV_ClipVelocity(original_velocity.as_mut_ptr(),
                                planes[i as usize].as_mut_ptr(),
                                new_velocity.as_mut_ptr(), 1.0f32);
                j = 0 as libc::c_int;
                while j < numplanes {
                    if j != i {
                        if new_velocity[0 as libc::c_int as usize] *
                               planes[j as usize][0 as libc::c_int as usize] +
                               new_velocity[1 as libc::c_int as usize] *
                                   planes[j as
                                              usize][1 as libc::c_int as
                                                         usize] +
                               new_velocity[2 as libc::c_int as usize] *
                                   planes[j as
                                              usize][2 as libc::c_int as
                                                         usize] < 0.0f32 {
                            break ;
                            // not ok
                        }
                    }
                    j += 1
                }
                if j == numplanes { break ; }
                i += 1
            }
            if i != numplanes {
                // go along this plane
                (*ent).v.velocity[0 as libc::c_int as usize] =
                    new_velocity[0 as libc::c_int as usize];
                (*ent).v.velocity[1 as libc::c_int as usize] =
                    new_velocity[1 as libc::c_int as usize];
                (*ent).v.velocity[2 as libc::c_int as usize] =
                    new_velocity[2 as libc::c_int as usize]
            } else if numplanes != 2 as libc::c_int {
                (*ent).v.velocity[2 as libc::c_int as usize] =
                    0 as libc::c_int as vec_t;
                (*ent).v.velocity[1 as libc::c_int as usize] =
                    (*ent).v.velocity[2 as libc::c_int as usize];
                (*ent).v.velocity[0 as libc::c_int as usize] =
                    (*ent).v.velocity[1 as libc::c_int as usize];
                break ;
            } else {
                dir[0 as libc::c_int as usize] =
                    planes[0 as libc::c_int as
                               usize][1 as libc::c_int as usize] *
                        planes[1 as libc::c_int as
                                   usize][2 as libc::c_int as usize] -
                        planes[0 as libc::c_int as
                                   usize][2 as libc::c_int as usize] *
                            planes[1 as libc::c_int as
                                       usize][1 as libc::c_int as usize];
                dir[1 as libc::c_int as usize] =
                    planes[0 as libc::c_int as
                               usize][2 as libc::c_int as usize] *
                        planes[1 as libc::c_int as
                                   usize][0 as libc::c_int as usize] -
                        planes[0 as libc::c_int as
                                   usize][0 as libc::c_int as usize] *
                            planes[1 as libc::c_int as
                                       usize][2 as libc::c_int as usize];
                dir[2 as libc::c_int as usize] =
                    planes[0 as libc::c_int as
                               usize][0 as libc::c_int as usize] *
                        planes[1 as libc::c_int as
                                   usize][1 as libc::c_int as usize] -
                        planes[0 as libc::c_int as
                                   usize][1 as libc::c_int as usize] *
                            planes[1 as libc::c_int as
                                       usize][0 as libc::c_int as usize];
                d =
                    dir[0 as libc::c_int as usize] *
                        (*ent).v.velocity[0 as libc::c_int as usize] +
                        dir[1 as libc::c_int as usize] *
                            (*ent).v.velocity[1 as libc::c_int as usize] +
                        dir[2 as libc::c_int as usize] *
                            (*ent).v.velocity[2 as libc::c_int as usize];
                (*ent).v.velocity[0 as libc::c_int as usize] =
                    dir[0 as libc::c_int as usize] * d;
                (*ent).v.velocity[1 as libc::c_int as usize] =
                    dir[1 as libc::c_int as usize] * d;
                (*ent).v.velocity[2 as libc::c_int as usize] =
                    dir[2 as libc::c_int as usize] * d
            }
            // go along the crease
            // if current velocity is against the original velocity,
		// stop dead to avoid tiny occilations in sloping corners
            if (*ent).v.velocity[0 as libc::c_int as usize] *
                   primal_velocity[0 as libc::c_int as usize] +
                   (*ent).v.velocity[1 as libc::c_int as usize] *
                       primal_velocity[1 as libc::c_int as usize] +
                   (*ent).v.velocity[2 as libc::c_int as usize] *
                       primal_velocity[2 as libc::c_int as usize] <= 0.0f32 {
                (*ent).v.velocity[2 as libc::c_int as usize] =
                    0 as libc::c_int as vec_t;
                (*ent).v.velocity[1 as libc::c_int as usize] =
                    (*ent).v.velocity[2 as libc::c_int as usize];
                (*ent).v.velocity[0 as libc::c_int as usize] =
                    (*ent).v.velocity[1 as libc::c_int as usize];
                break ;
            } else { bumpcount += 1 }
        }
    }
    if allFraction == 0.0f32 {
        (*ent).v.velocity[2 as libc::c_int as usize] =
            0 as libc::c_int as vec_t;
        (*ent).v.velocity[1 as libc::c_int as usize] =
            (*ent).v.velocity[2 as libc::c_int as usize];
        (*ent).v.velocity[0 as libc::c_int as usize] =
            (*ent).v.velocity[1 as libc::c_int as usize]
    }
    return blocked;
}
/*
============
SV_AddGravity

============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_AddGravity(mut ent: *mut edict_t) {
    let mut ent_gravity: libc::c_float = 0.;
    if (*ent).v.gravity != 0. {
        ent_gravity = (*ent).v.gravity
    } else { ent_gravity = 1.0f32 }
    // add gravity incorrectly
    (*ent).v.velocity[2 as libc::c_int as usize] -=
        ent_gravity * sv_gravity.value * sv.frametime;
    (*ent).v.velocity[2 as libc::c_int as usize] +=
        (*ent).v.basevelocity[2 as libc::c_int as usize] * sv.frametime;
    (*ent).v.basevelocity[2 as libc::c_int as usize] = 0.0f32;
    // bound velocity
    SV_CheckVelocity(ent);
}
/*
============
SV_AddHalfGravity

============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_AddHalfGravity(mut ent: *mut edict_t,
                                           mut timestep: libc::c_float) {
    let mut ent_gravity: libc::c_float = 0.;
    if (*ent).v.gravity != 0. {
        ent_gravity = (*ent).v.gravity
    } else { ent_gravity = 1.0f32 }
    // Add 1/2 of the total gravitational effects over this timestep
    (*ent).v.velocity[2 as libc::c_int as usize] -=
        0.5f32 * ent_gravity * sv_gravity.value * timestep;
    (*ent).v.velocity[2 as libc::c_int as usize] +=
        (*ent).v.basevelocity[2 as libc::c_int as usize] * sv.frametime;
    (*ent).v.basevelocity[2 as libc::c_int as usize] = 0.0f32;
    // bound velocity
    SV_CheckVelocity(ent);
}
/*
===============================================================================

PUSHMOVE

===============================================================================
*/
/*
============
SV_AllowPushRotate

Allows to change entity yaw?
============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_AllowPushRotate(mut ent: *mut edict_t)
 -> qboolean {
    let mut mod_0: *mut model_t = 0 as *mut model_t;
    mod_0 = SV_ModelHandle((*ent).v.modelindex);
    if mod_0.is_null() ||
           (*mod_0).type_0 as libc::c_int != mod_brush as libc::c_int {
        return true_0
    }
    if host.features &
           ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint == 0 {
        return false_0
    }
    if (*mod_0).flags as libc::c_uint &
           (1 as libc::c_uint) << 1 as libc::c_int != 0 {
        return true_0
    }
    return false_0;
}
/*
============
SV_PushEntity

Does not change the entities velocity at all
============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_PushEntity(mut ent: *mut edict_t,
                                       mut lpush: *const vec_t,
                                       mut apush: *const vec_t,
                                       mut blocked: *mut libc::c_int,
                                       mut flDamage: libc::c_float)
 -> trace_t {
    let mut trace: trace_t =
        trace_t{allsolid: false_0,
                startsolid: false_0,
                inopen: false_0,
                inwater: false_0,
                fraction: 0.,
                endpos: [0.; 3],
                plane: plane_t{normal: [0.; 3], dist: 0.,},
                ent: 0 as *mut edict_t,
                hitgroup: 0,}; // only clip against bmodels
    let mut monsterBlock: qboolean = false_0;
    let mut monsterClip: qboolean = false_0;
    let mut type_0: libc::c_int = 0;
    let mut end: vec3_t = [0.; 3];
    monsterClip =
        if (*ent).v.flags as libc::c_uint &
               (1 as libc::c_uint) << 23 as libc::c_int != 0 {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int } as qboolean;
    end[0 as libc::c_int as usize] =
        (*ent).v.origin[0 as libc::c_int as usize] +
            *lpush.offset(0 as libc::c_int as isize);
    end[1 as libc::c_int as usize] =
        (*ent).v.origin[1 as libc::c_int as usize] +
            *lpush.offset(1 as libc::c_int as isize);
    end[2 as libc::c_int as usize] =
        (*ent).v.origin[2 as libc::c_int as usize] +
            *lpush.offset(2 as libc::c_int as isize);
    if (*ent).v.movetype == 9 as libc::c_int {
        type_0 = 2 as libc::c_int
    } else if (*ent).v.solid == 1 as libc::c_int ||
                  (*ent).v.solid == 0 as libc::c_int {
        type_0 = 1 as libc::c_int
    } else { type_0 = 0 as libc::c_int }
    trace =
        SV_Move((*ent).v.origin.as_mut_ptr() as *const vec_t,
                (*ent).v.mins.as_mut_ptr(), (*ent).v.maxs.as_mut_ptr(),
                end.as_mut_ptr() as *const vec_t, type_0, ent, monsterClip);
    if trace.fraction != 0.0f32 {
        (*ent).v.origin[0 as libc::c_int as usize] =
            trace.endpos[0 as libc::c_int as usize];
        (*ent).v.origin[1 as libc::c_int as usize] =
            trace.endpos[1 as libc::c_int as usize];
        (*ent).v.origin[2 as libc::c_int as usize] =
            trace.endpos[2 as libc::c_int as usize];
        if sv.state as libc::c_uint ==
               ss_active as libc::c_int as libc::c_uint &&
               *apush.offset(1 as libc::c_int as isize) != 0. &&
               (*ent).v.flags as libc::c_uint &
                   (1 as libc::c_uint) << 3 as libc::c_int != 0 {
            (*ent).v.avelocity[1 as libc::c_int as usize] +=
                *apush.offset(1 as libc::c_int as isize);
            (*ent).v.fixangle = 2 as libc::c_int
        }
        // don't rotate pushables!
        if SV_AllowPushRotate(ent) as u64 != 0 {
            (*ent).v.angles[1 as libc::c_int as usize] +=
                trace.fraction * *apush.offset(1 as libc::c_int as isize)
        }
    }
    SV_LinkEdict(ent, true_0);
    if (*ent).v.movetype == 3 as libc::c_int ||
           (*ent).v.movetype == 4 as libc::c_int ||
           (*ent).v.movetype == 13 as libc::c_int {
        monsterBlock = true_0
    } else { monsterBlock = false_0 }
    if !blocked.is_null() {
        // more accuracy blocking code
        if monsterBlock as u64 != 0 {
            *blocked =
                (VectorCompareEpsilon((*ent).v.origin.as_mut_ptr() as
                                          *const vec_t,
                                      end.as_mut_ptr() as *const vec_t,
                                      0.1f32) as u64 == 0) as libc::c_int
        } else { *blocked = true_0 as libc::c_int }
    } // can't move full distance
    // so we can run impact function afterwards.
    if SV_CheckEdict(trace.ent,
                     b"../engine/server/sv_phys.c\x00" as *const u8 as
                         *const libc::c_char, 832 as libc::c_int) as u64 != 0
       {
        SV_Impact(ent, trace.ent, &mut trace);
    }
    return trace;
}
/*
============
SV_CanPushed

filter entities for push
============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_CanPushed(mut ent: *mut edict_t) -> qboolean {
    // filter movetypes to collide with
    match (*ent).v.movetype {
        0 | 7 | 12 | 8 | 14 => { return false_0 }
        _ => { }
    }
    return true_0;
}
/*
============
SV_CanBlock

allow entity to block pusher?
============
*/
unsafe extern "C" fn SV_CanBlock(mut ent: *mut edict_t) -> qboolean {
    if (*ent).v.mins[0 as libc::c_int as usize] ==
           (*ent).v.maxs[0 as libc::c_int as usize] {
        return false_0
    }
    if (*ent).v.solid == 0 as libc::c_int ||
           (*ent).v.solid == 1 as libc::c_int {
        // clear bounds for deadbody
        (*ent).v.mins[1 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
        (*ent).v.mins[0 as libc::c_int as usize] =
            (*ent).v.mins[1 as libc::c_int as usize];
        (*ent).v.maxs[0 as libc::c_int as usize] =
            (*ent).v.mins[0 as libc::c_int as usize];
        (*ent).v.maxs[1 as libc::c_int as usize] =
            (*ent).v.mins[1 as libc::c_int as usize];
        (*ent).v.maxs[2 as libc::c_int as usize] =
            (*ent).v.mins[2 as libc::c_int as usize];
        return false_0
    }
    return true_0;
}
/*
============
SV_PushMove

============
*/
unsafe extern "C" fn SV_PushMove(mut pusher: *mut edict_t,
                                 mut movetime: libc::c_float)
 -> *mut edict_t {
    let mut i: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    let mut block: libc::c_int = 0;
    let mut num_moved: libc::c_int = 0;
    let mut oldsolid: libc::c_int = 0;
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut lmove: vec3_t = [0.; 3];
    let mut p: *mut sv_pushed_t = 0 as *mut sv_pushed_t;
    let mut pushed_p: *mut sv_pushed_t = 0 as *mut sv_pushed_t;
    let mut check: *mut edict_t = 0 as *mut edict_t;
    if (*svgame.globals).changelevel != 0 ||
           (*pusher).v.velocity[0 as libc::c_int as usize] == 0.0f32 &&
               (*pusher).v.velocity[1 as libc::c_int as usize] == 0.0f32 &&
               (*pusher).v.velocity[2 as libc::c_int as usize] == 0.0f32 {
        (*pusher).v.ltime += movetime;
        return 0 as *mut edict_t
    }
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        lmove[i as usize] = (*pusher).v.velocity[i as usize] * movetime;
        mins[i as usize] = (*pusher).v.absmin[i as usize] + lmove[i as usize];
        maxs[i as usize] = (*pusher).v.absmax[i as usize] + lmove[i as usize];
        i += 1
    }
    pushed_p = svgame.pushed.as_mut_ptr();
    // save the pusher's original position
    (*pushed_p).ent = pusher;
    (*pushed_p).origin[0 as libc::c_int as usize] =
        (*pusher).v.origin[0 as libc::c_int as usize];
    (*pushed_p).origin[1 as libc::c_int as usize] =
        (*pusher).v.origin[1 as libc::c_int as usize];
    (*pushed_p).origin[2 as libc::c_int as usize] =
        (*pusher).v.origin[2 as libc::c_int as usize];
    (*pushed_p).angles[0 as libc::c_int as usize] =
        (*pusher).v.angles[0 as libc::c_int as usize];
    (*pushed_p).angles[1 as libc::c_int as usize] =
        (*pusher).v.angles[1 as libc::c_int as usize];
    (*pushed_p).angles[2 as libc::c_int as usize] =
        (*pusher).v.angles[2 as libc::c_int as usize];
    pushed_p = pushed_p.offset(1);
    // move the pusher to it's final position
    SV_LinearMove(pusher, movetime, 0.0f32);
    SV_LinkEdict(pusher, false_0);
    (*pusher).v.ltime += movetime;
    oldsolid = (*pusher).v.solid;
    // non-solid pushers can't push anything
    if (*pusher).v.solid == 0 as libc::c_int { return 0 as *mut edict_t }
    // see if any solid entities are inside the final position
    num_moved = 0 as libc::c_int;
    let mut current_block_43: u64;
    e = 1 as libc::c_int;
    while e < svgame.numEntities {
        check = SV_EdictNum(e);
        if !(SV_CheckEdict(check,
                           b"../engine/server/sv_phys.c\x00" as *const u8 as
                               *const libc::c_char, 934 as libc::c_int) as u64
                 == 0) {
            // filter movetypes to collide with
            if !(SV_CanPushed(check) as u64 == 0) {
                (*pusher).v.solid = 0 as libc::c_int;
                block = SV_TestEntityPosition(check, pusher) as libc::c_int;
                (*pusher).v.solid = oldsolid;
                if !(block != 0) {
                    // if the entity is standing on the pusher, it will definately be moved
                    if !((*check).v.flags as libc::c_uint &
                             (1 as libc::c_uint) << 9 as libc::c_int != 0 &&
                             (*check).v.groundentity == pusher) {
                        if (*check).v.absmin[0 as libc::c_int as usize] >=
                               maxs[0 as libc::c_int as usize] ||
                               (*check).v.absmin[1 as libc::c_int as usize] >=
                                   maxs[1 as libc::c_int as usize] ||
                               (*check).v.absmin[2 as libc::c_int as usize] >=
                                   maxs[2 as libc::c_int as usize] ||
                               (*check).v.absmax[0 as libc::c_int as usize] <=
                                   mins[0 as libc::c_int as usize] ||
                               (*check).v.absmax[1 as libc::c_int as usize] <=
                                   mins[1 as libc::c_int as usize] ||
                               (*check).v.absmax[2 as libc::c_int as usize] <=
                                   mins[2 as libc::c_int as usize] {
                            current_block_43 = 11042950489265723346;
                        } else if SV_TestEntityPosition(check,
                                                        0 as *mut edict_t) as
                                      u64 == 0 {
                            current_block_43 = 11042950489265723346;
                        } else { current_block_43 = 14434620278749266018; }
                    } else { current_block_43 = 14434620278749266018; }
                    match current_block_43 {
                        11042950489265723346 => { }
                        _ => {
                            // see if the ent's bbox is inside the pusher's final position
                            // remove the onground flag for non-players
                            if (*check).v.movetype != 3 as libc::c_int {
                                (*check).v.flags =
                                    ((*check).v.flags as libc::c_uint &
                                         !((1 as libc::c_uint) <<
                                               9 as libc::c_int)) as
                                        libc::c_int
                            }
                            // save original position of contacted entity
                            (*pushed_p).ent = check;
                            (*pushed_p).origin[0 as libc::c_int as usize] =
                                (*check).v.origin[0 as libc::c_int as usize];
                            (*pushed_p).origin[1 as libc::c_int as usize] =
                                (*check).v.origin[1 as libc::c_int as usize];
                            (*pushed_p).origin[2 as libc::c_int as usize] =
                                (*check).v.origin[2 as libc::c_int as usize];
                            (*pushed_p).angles[0 as libc::c_int as usize] =
                                (*check).v.angles[0 as libc::c_int as usize];
                            (*pushed_p).angles[1 as libc::c_int as usize] =
                                (*check).v.angles[1 as libc::c_int as usize];
                            (*pushed_p).angles[2 as libc::c_int as usize] =
                                (*check).v.angles[2 as libc::c_int as usize];
                            pushed_p = pushed_p.offset(1);
                            // try moving the contacted entity
                            (*pusher).v.solid = 0 as libc::c_int;
                            SV_PushEntity(check,
                                          lmove.as_mut_ptr() as *const vec_t,
                                          vec3_origin.as_mut_ptr() as
                                              *const vec_t, &mut block,
                                          (*pusher).v.dmg);
                            (*pusher).v.solid = oldsolid;
                            // if it is still inside the pusher, block
                            if SV_TestEntityPosition(check, 0 as *mut edict_t)
                                   as libc::c_uint != 0 && block != 0 {
                                if !(SV_CanBlock(check) as u64 == 0) {
                                    (*pusher).v.ltime -= movetime;
                                    // move back any entities we already moved
			// go backwards, so if the same entity was pushed
			// twice, it goes back to the original position
                                    p =
                                        pushed_p.offset(-(1 as libc::c_int as
                                                              isize));
                                    while p >= svgame.pushed.as_mut_ptr() {
                                        (*(*p).ent).v.origin[0 as libc::c_int
                                                                 as usize] =
                                            (*p).origin[0 as libc::c_int as
                                                            usize];
                                        (*(*p).ent).v.origin[1 as libc::c_int
                                                                 as usize] =
                                            (*p).origin[1 as libc::c_int as
                                                            usize];
                                        (*(*p).ent).v.origin[2 as libc::c_int
                                                                 as usize] =
                                            (*p).origin[2 as libc::c_int as
                                                            usize];
                                        (*(*p).ent).v.angles[0 as libc::c_int
                                                                 as usize] =
                                            (*p).angles[0 as libc::c_int as
                                                            usize];
                                        (*(*p).ent).v.angles[1 as libc::c_int
                                                                 as usize] =
                                            (*p).angles[1 as libc::c_int as
                                                            usize];
                                        (*(*p).ent).v.angles[2 as libc::c_int
                                                                 as usize] =
                                            (*p).angles[2 as libc::c_int as
                                                            usize];
                                        SV_LinkEdict((*p).ent,
                                                     if (*p).ent == check {
                                                         true_0 as libc::c_int
                                                     } else {
                                                         false_0 as
                                                             libc::c_int
                                                     } as qboolean);
                                        p = p.offset(-1)
                                    }
                                    return check
                                }
                            }
                        }
                    }
                }
            }
        }
        e += 1
    }
    return 0 as *mut edict_t;
}
/*
============
SV_PushRotate

============
*/
unsafe extern "C" fn SV_PushRotate(mut pusher: *mut edict_t,
                                   mut movetime: libc::c_float)
 -> *mut edict_t {
    let mut i: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    let mut block: libc::c_int = 0;
    let mut oldsolid: libc::c_int = 0;
    let mut start_l: matrix4x4 = [[0.; 4]; 4];
    let mut end_l: matrix4x4 = [[0.; 4]; 4];
    let mut lmove: vec3_t = [0.; 3];
    let mut amove: vec3_t = [0.; 3];
    let mut p: *mut sv_pushed_t = 0 as *mut sv_pushed_t;
    let mut pushed_p: *mut sv_pushed_t = 0 as *mut sv_pushed_t;
    let mut org: vec3_t = [0.; 3];
    let mut org2: vec3_t = [0.; 3];
    let mut temp: vec3_t = [0.; 3];
    let mut check: *mut edict_t = 0 as *mut edict_t;
    if (*svgame.globals).changelevel != 0 ||
           (*pusher).v.avelocity[0 as libc::c_int as usize] == 0.0f32 &&
               (*pusher).v.avelocity[1 as libc::c_int as usize] == 0.0f32 &&
               (*pusher).v.avelocity[2 as libc::c_int as usize] == 0.0f32 {
        (*pusher).v.ltime += movetime;
        return 0 as *mut edict_t
    }
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        amove[i as usize] = (*pusher).v.avelocity[i as usize] * movetime;
        i += 1
    }
    // create pusher initial position
    Matrix4x4_CreateFromEntity(start_l.as_mut_ptr(),
                               (*pusher).v.angles.as_mut_ptr() as
                                   *const vec_t,
                               (*pusher).v.origin.as_mut_ptr() as
                                   *const vec_t, 1.0f32);
    pushed_p = svgame.pushed.as_mut_ptr();
    // save the pusher's original position
    (*pushed_p).ent = pusher;
    (*pushed_p).origin[0 as libc::c_int as usize] =
        (*pusher).v.origin[0 as libc::c_int as usize];
    (*pushed_p).origin[1 as libc::c_int as usize] =
        (*pusher).v.origin[1 as libc::c_int as usize];
    (*pushed_p).origin[2 as libc::c_int as usize] =
        (*pusher).v.origin[2 as libc::c_int as usize];
    (*pushed_p).angles[0 as libc::c_int as usize] =
        (*pusher).v.angles[0 as libc::c_int as usize];
    (*pushed_p).angles[1 as libc::c_int as usize] =
        (*pusher).v.angles[1 as libc::c_int as usize];
    (*pushed_p).angles[2 as libc::c_int as usize] =
        (*pusher).v.angles[2 as libc::c_int as usize];
    pushed_p = pushed_p.offset(1);
    // move the pusher to it's final position
    SV_AngularMove(pusher, movetime, (*pusher).v.friction);
    SV_LinkEdict(pusher, false_0);
    (*pusher).v.ltime += movetime;
    oldsolid = (*pusher).v.solid;
    // non-solid pushers can't push anything
    if (*pusher).v.solid == 0 as libc::c_int { return 0 as *mut edict_t }
    // create pusher final position
    Matrix4x4_CreateFromEntity(end_l.as_mut_ptr(),
                               (*pusher).v.angles.as_mut_ptr() as
                                   *const vec_t,
                               (*pusher).v.origin.as_mut_ptr() as
                                   *const vec_t, 1.0f32);
    let mut current_block_55: u64;
    // see if any solid entities are inside the final position
    e = 1 as libc::c_int;
    while e < svgame.numEntities {
        check = SV_EdictNum(e);
        if !(SV_CheckEdict(check,
                           b"../engine/server/sv_phys.c\x00" as *const u8 as
                               *const libc::c_char, 1052 as libc::c_int) as
                 u64 == 0) {
            // filter movetypes to collide with
            if !(SV_CanPushed(check) as u64 == 0) {
                (*pusher).v.solid = 0 as libc::c_int;
                block = SV_TestEntityPosition(check, pusher) as libc::c_int;
                (*pusher).v.solid = oldsolid;
                if !(block != 0) {
                    // if the entity is standing on the pusher, it will definately be moved
                    if !((*check).v.flags as libc::c_uint &
                             (1 as libc::c_uint) << 9 as libc::c_int != 0 &&
                             (*check).v.groundentity == pusher) {
                        if (*check).v.absmin[0 as libc::c_int as usize] >=
                               (*pusher).v.absmax[0 as libc::c_int as usize]
                               ||
                               (*check).v.absmin[1 as libc::c_int as usize] >=
                                   (*pusher).v.absmax[1 as libc::c_int as
                                                          usize] ||
                               (*check).v.absmin[2 as libc::c_int as usize] >=
                                   (*pusher).v.absmax[2 as libc::c_int as
                                                          usize] ||
                               (*check).v.absmax[0 as libc::c_int as usize] <=
                                   (*pusher).v.absmin[0 as libc::c_int as
                                                          usize] ||
                               (*check).v.absmax[1 as libc::c_int as usize] <=
                                   (*pusher).v.absmin[1 as libc::c_int as
                                                          usize] ||
                               (*check).v.absmax[2 as libc::c_int as usize] <=
                                   (*pusher).v.absmin[2 as libc::c_int as
                                                          usize] {
                            current_block_55 = 224731115979188411;
                        } else if SV_TestEntityPosition(check,
                                                        0 as *mut edict_t) as
                                      u64 == 0 {
                            current_block_55 = 224731115979188411;
                        } else { current_block_55 = 1538046216550696469; }
                    } else { current_block_55 = 1538046216550696469; }
                    match current_block_55 {
                        224731115979188411 => { }
                        _ => {
                            // see if the ent's bbox is inside the pusher's final position
                            // save original position of contacted entity
                            (*pushed_p).ent = check;
                            (*pushed_p).origin[0 as libc::c_int as usize] =
                                (*check).v.origin[0 as libc::c_int as usize];
                            (*pushed_p).origin[1 as libc::c_int as usize] =
                                (*check).v.origin[1 as libc::c_int as usize];
                            (*pushed_p).origin[2 as libc::c_int as usize] =
                                (*check).v.origin[2 as libc::c_int as usize];
                            (*pushed_p).angles[0 as libc::c_int as usize] =
                                (*check).v.angles[0 as libc::c_int as usize];
                            (*pushed_p).angles[1 as libc::c_int as usize] =
                                (*check).v.angles[1 as libc::c_int as usize];
                            (*pushed_p).angles[2 as libc::c_int as usize] =
                                (*check).v.angles[2 as libc::c_int as usize];
                            (*pushed_p).fixangle = (*check).v.fixangle;
                            pushed_p = pushed_p.offset(1);
                            // calculate destination position
                            if (*check).v.movetype == 13 as libc::c_int ||
                                   (*check).v.movetype == 4 as libc::c_int {
                                org[0 as libc::c_int as usize] =
                                    ((*check).v.absmin[0 as libc::c_int as
                                                           usize] +
                                         (*check).v.absmax[0 as libc::c_int as
                                                               usize]) *
                                        0.5f32;
                                org[1 as libc::c_int as usize] =
                                    ((*check).v.absmin[1 as libc::c_int as
                                                           usize] +
                                         (*check).v.absmax[1 as libc::c_int as
                                                               usize]) *
                                        0.5f32;
                                org[2 as libc::c_int as usize] =
                                    ((*check).v.absmin[2 as libc::c_int as
                                                           usize] +
                                         (*check).v.absmax[2 as libc::c_int as
                                                               usize]) *
                                        0.5f32
                            } else {
                                org[0 as libc::c_int as usize] =
                                    (*check).v.origin[0 as libc::c_int as
                                                          usize];
                                org[1 as libc::c_int as usize] =
                                    (*check).v.origin[1 as libc::c_int as
                                                          usize];
                                org[2 as libc::c_int as usize] =
                                    (*check).v.origin[2 as libc::c_int as
                                                          usize]
                            }
                            Matrix4x4_VectorITransform(start_l.as_mut_ptr() as
                                                           *const [vec_t; 4],
                                                       org.as_mut_ptr() as
                                                           *const libc::c_float,
                                                       temp.as_mut_ptr());
                            Matrix4x4_VectorTransform(end_l.as_mut_ptr() as
                                                          *const [vec_t; 4],
                                                      temp.as_mut_ptr() as
                                                          *const libc::c_float,
                                                      org2.as_mut_ptr());
                            lmove[0 as libc::c_int as usize] =
                                org2[0 as libc::c_int as usize] -
                                    org[0 as libc::c_int as usize];
                            lmove[1 as libc::c_int as usize] =
                                org2[1 as libc::c_int as usize] -
                                    org[1 as libc::c_int as usize];
                            lmove[2 as libc::c_int as usize] =
                                org2[2 as libc::c_int as usize] -
                                    org[2 as libc::c_int as usize];
                            // i can't clear FL_ONGROUND in all cases because many bad things may be happen
                            if (*check).v.movetype != 3 as libc::c_int {
                                if lmove[2 as libc::c_int as usize] != 0.0f32
                                   {
                                    (*check).v.flags =
                                        ((*check).v.flags as libc::c_uint &
                                             !((1 as libc::c_uint) <<
                                                   9 as libc::c_int)) as
                                            libc::c_int
                                }
                                if lmove[2 as libc::c_int as usize] < 0.0f32
                                       && (*pusher).v.dmg == 0. {
                                    lmove[2 as libc::c_int as usize] = 0.0f32
                                }
                                // let's the free falling
                            }
                            // try moving the contacted entity
                            (*pusher).v.solid = 0 as libc::c_int;
                            SV_PushEntity(check,
                                          lmove.as_mut_ptr() as *const vec_t,
                                          amove.as_mut_ptr() as *const vec_t,
                                          &mut block, (*pusher).v.dmg);
                            (*pusher).v.solid = oldsolid;
                            // pushed entity blocked by wall
                            if block != 0 &&
                                   (*check).v.movetype != 3 as libc::c_int {
                                (*check).v.flags =
                                    ((*check).v.flags as libc::c_uint &
                                         !((1 as libc::c_uint) <<
                                               9 as libc::c_int)) as
                                        libc::c_int
                            }
                            // if it is still inside the pusher, block
                            if SV_TestEntityPosition(check, 0 as *mut edict_t)
                                   as libc::c_uint != 0 && block != 0 {
                                if !(SV_CanBlock(check) as u64 == 0) {
                                    (*pusher).v.ltime -= movetime;
                                    // move back any entities we already moved
			// go backwards, so if the same entity was pushed
			// twice, it goes back to the original position
                                    p =
                                        pushed_p.offset(-(1 as libc::c_int as
                                                              isize));
                                    while p >= svgame.pushed.as_mut_ptr() {
                                        (*(*p).ent).v.origin[0 as libc::c_int
                                                                 as usize] =
                                            (*p).origin[0 as libc::c_int as
                                                            usize];
                                        (*(*p).ent).v.origin[1 as libc::c_int
                                                                 as usize] =
                                            (*p).origin[1 as libc::c_int as
                                                            usize];
                                        (*(*p).ent).v.origin[2 as libc::c_int
                                                                 as usize] =
                                            (*p).origin[2 as libc::c_int as
                                                            usize];
                                        (*(*p).ent).v.angles[0 as libc::c_int
                                                                 as usize] =
                                            (*p).angles[0 as libc::c_int as
                                                            usize];
                                        (*(*p).ent).v.angles[1 as libc::c_int
                                                                 as usize] =
                                            (*p).angles[1 as libc::c_int as
                                                            usize];
                                        (*(*p).ent).v.angles[2 as libc::c_int
                                                                 as usize] =
                                            (*p).angles[2 as libc::c_int as
                                                            usize];
                                        SV_LinkEdict((*p).ent,
                                                     if (*p).ent == check {
                                                         true_0 as libc::c_int
                                                     } else {
                                                         false_0 as
                                                             libc::c_int
                                                     } as qboolean);
                                        (*(*p).ent).v.fixangle =
                                            (*p).fixangle;
                                        p = p.offset(-1)
                                    }
                                    return check
                                }
                            }
                        }
                    }
                }
            }
        }
        e += 1
    }
    return 0 as *mut edict_t;
}
/*
================
SV_Physics_Pusher

================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_Physics_Pusher(mut ent: *mut edict_t) {
    let mut oldtime: libc::c_float = 0.;
    let mut oldtime2: libc::c_float = 0.;
    let mut thinktime: libc::c_float = 0.;
    let mut movetime: libc::c_float = 0.;
    let mut pBlocker: *mut edict_t = 0 as *mut edict_t;
    let mut i: libc::c_int = 0;
    pBlocker = 0 as *mut edict_t;
    oldtime = (*ent).v.ltime;
    thinktime = (*ent).v.nextthink;
    if thinktime < oldtime + sv.frametime {
        movetime = thinktime - oldtime;
        if movetime < 0.0f32 { movetime = 0.0f32 }
    } else { movetime = sv.frametime }
    if movetime != 0. {
        if !((*ent).v.avelocity[0 as libc::c_int as usize] == 0.0f32 &&
                 (*ent).v.avelocity[1 as libc::c_int as usize] == 0.0f32 &&
                 (*ent).v.avelocity[2 as libc::c_int as usize] == 0.0f32) {
            if !((*ent).v.velocity[0 as libc::c_int as usize] == 0.0f32 &&
                     (*ent).v.velocity[1 as libc::c_int as usize] == 0.0f32 &&
                     (*ent).v.velocity[2 as libc::c_int as usize] == 0.0f32) {
                pBlocker = SV_PushRotate(ent, movetime);
                if pBlocker.is_null() {
                    oldtime2 = (*ent).v.ltime;
                    // reset the local time to what it was before we rotated
                    (*ent).v.ltime = oldtime;
                    pBlocker = SV_PushMove(ent, movetime);
                    if (*ent).v.ltime < oldtime2 { (*ent).v.ltime = oldtime2 }
                }
            } else { pBlocker = SV_PushRotate(ent, movetime) }
        } else { pBlocker = SV_PushMove(ent, movetime) }
    }
    // if the pusher has a "blocked" function, call it
	// otherwise, just stay in place until the obstacle is gone
    if !pBlocker.is_null() {
        svgame.dllFuncs.pfnBlocked.expect("non-null function pointer")(ent,
                                                                       pBlocker);
    }
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if (*ent).v.angles[i as usize] < -3600.0f32 ||
               (*ent).v.angles[i as usize] > 3600.0f32 {
            (*ent).v.angles[i as usize] =
                __tg_fmod((*ent).v.angles[i as usize], 3600.0f32)
        }
        i += 1
    }
    if thinktime > oldtime &&
           ((*ent).v.flags as libc::c_uint &
                (1 as libc::c_uint) << 21 as libc::c_int != 0 ||
                thinktime <= (*ent).v.ltime) {
        (*ent).v.nextthink = 0.0f32;
        (*svgame.globals).time = sv.time as libc::c_float;
        svgame.dllFuncs.pfnThink.expect("non-null function pointer")(ent);
    };
}
//============================================================================
/*
=============
SV_Physics_Follow

just copy angles and origin of parent
=============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_Physics_Follow(mut ent: *mut edict_t) {
    let mut parent: *mut edict_t = 0 as *mut edict_t;
    // regular thinking
    if SV_RunThink(ent) as u64 == 0 { return }
    parent = (*ent).v.aiment;
    if SV_CheckEdict(parent,
                     b"../engine/server/sv_phys.c\x00" as *const u8 as
                         *const libc::c_char, 1227 as libc::c_int) as u64 == 0
       {
        (*ent).v.movetype = 0 as libc::c_int;
        return
    }
    (*ent).v.origin[0 as libc::c_int as usize] =
        (*parent).v.origin[0 as libc::c_int as usize] +
            (*ent).v.v_angle[0 as libc::c_int as usize];
    (*ent).v.origin[1 as libc::c_int as usize] =
        (*parent).v.origin[1 as libc::c_int as usize] +
            (*ent).v.v_angle[1 as libc::c_int as usize];
    (*ent).v.origin[2 as libc::c_int as usize] =
        (*parent).v.origin[2 as libc::c_int as usize] +
            (*ent).v.v_angle[2 as libc::c_int as usize];
    (*ent).v.angles[0 as libc::c_int as usize] =
        (*parent).v.angles[0 as libc::c_int as usize];
    (*ent).v.angles[1 as libc::c_int as usize] =
        (*parent).v.angles[1 as libc::c_int as usize];
    (*ent).v.angles[2 as libc::c_int as usize] =
        (*parent).v.angles[2 as libc::c_int as usize];
    SV_LinkEdict(ent, true_0);
}
/*
=============
SV_Physics_Compound

a glue two entities together
=============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_Physics_Compound(mut ent: *mut edict_t) {
    let mut parent: *mut edict_t = 0 as *mut edict_t;
    // regular thinking
    if SV_RunThink(ent) as u64 == 0 { return }
    parent = (*ent).v.aiment;
    if SV_CheckEdict(parent,
                     b"../engine/server/sv_phys.c\x00" as *const u8 as
                         *const libc::c_char, 1255 as libc::c_int) as u64 == 0
       {
        (*ent).v.movetype = 0 as libc::c_int;
        return
    }
    if (*ent).v.solid != 1 as libc::c_int {
        (*ent).v.solid = 0 as libc::c_int
    }
    match (*parent).v.movetype { 7 | 13 => { } _ => { return } }
    // not initialized ?
    if (*ent).v.ltime == 0.0f32 {
        (*ent).v.oldorigin[0 as libc::c_int as usize] =
            (*parent).v.origin[0 as libc::c_int as usize];
        (*ent).v.oldorigin[1 as libc::c_int as usize] =
            (*parent).v.origin[1 as libc::c_int as usize];
        (*ent).v.oldorigin[2 as libc::c_int as usize] =
            (*parent).v.origin[2 as libc::c_int as usize];
        (*ent).v.avelocity[0 as libc::c_int as usize] =
            (*parent).v.angles[0 as libc::c_int as usize];
        (*ent).v.avelocity[1 as libc::c_int as usize] =
            (*parent).v.angles[1 as libc::c_int as usize];
        (*ent).v.avelocity[2 as libc::c_int as usize] =
            (*parent).v.angles[2 as libc::c_int as usize];
        (*ent).v.ltime = sv.frametime;
        return
    }
    if !((*parent).v.origin[0 as libc::c_int as usize] ==
             (*ent).v.oldorigin[0 as libc::c_int as usize] &&
             (*parent).v.origin[1 as libc::c_int as usize] ==
                 (*ent).v.oldorigin[1 as libc::c_int as usize] &&
             (*parent).v.origin[2 as libc::c_int as usize] ==
                 (*ent).v.oldorigin[2 as libc::c_int as usize]) ||
           !((*parent).v.angles[0 as libc::c_int as usize] ==
                 (*ent).v.avelocity[0 as libc::c_int as usize] &&
                 (*parent).v.angles[1 as libc::c_int as usize] ==
                     (*ent).v.avelocity[1 as libc::c_int as usize] &&
                 (*parent).v.angles[2 as libc::c_int as usize] ==
                     (*ent).v.avelocity[2 as libc::c_int as usize]) {
        let mut start_l: matrix4x4 = [[0.; 4]; 4];
        let mut end_l: matrix4x4 = [[0.; 4]; 4];
        let mut temp_l: matrix4x4 = [[0.; 4]; 4];
        let mut child: matrix4x4 = [[0.; 4]; 4];
        // create parent old position
        Matrix4x4_CreateFromEntity(temp_l.as_mut_ptr(),
                                   (*ent).v.avelocity.as_mut_ptr() as
                                       *const vec_t,
                                   (*ent).v.oldorigin.as_mut_ptr() as
                                       *const vec_t, 1.0f32);
        Matrix4x4_Invert_Simple(start_l.as_mut_ptr(),
                                temp_l.as_mut_ptr() as *const [vec_t; 4]);
        // create parent actual position
        Matrix4x4_CreateFromEntity(end_l.as_mut_ptr(),
                                   (*parent).v.angles.as_mut_ptr() as
                                       *const vec_t,
                                   (*parent).v.origin.as_mut_ptr() as
                                       *const vec_t, 1.0f32);
        // stupid quake bug!!!
        if host.features &
               ((1 as libc::c_int) << 5 as libc::c_int) as libc::c_uint == 0 {
            (*ent).v.angles[0 as libc::c_int as usize] =
                -(*ent).v.angles[0 as libc::c_int as usize]
        }
        // create child actual position
        Matrix4x4_CreateFromEntity(child.as_mut_ptr(),
                                   (*ent).v.angles.as_mut_ptr() as
                                       *const vec_t,
                                   (*ent).v.origin.as_mut_ptr() as
                                       *const vec_t, 1.0f32);
        // transform child from start to end
        Matrix4x4_ConcatTransforms(temp_l.as_mut_ptr(),
                                   start_l.as_mut_ptr() as *const [vec_t; 4],
                                   child.as_mut_ptr() as *const [vec_t; 4]);
        Matrix4x4_ConcatTransforms(child.as_mut_ptr(),
                                   end_l.as_mut_ptr() as *const [vec_t; 4],
                                   temp_l.as_mut_ptr() as *const [vec_t; 4]);
        // create child final position
        Matrix4x4_ConvertToEntity(child.as_mut_ptr() as *const [vec_t; 4],
                                  (*ent).v.angles.as_mut_ptr(),
                                  (*ent).v.origin.as_mut_ptr());
        // stupid quake bug!!!
        if host.features &
               ((1 as libc::c_int) << 5 as libc::c_int) as libc::c_uint == 0 {
            (*ent).v.angles[0 as libc::c_int as usize] =
                -(*ent).v.angles[0 as libc::c_int as usize]
        }
    }
    // notsolid ents never touch triggers
    SV_LinkEdict(ent,
                 if (*ent).v.solid == 0 as libc::c_int {
                     false_0 as libc::c_int
                 } else { true_0 as libc::c_int } as qboolean);
    // shuffle states
    (*ent).v.oldorigin[0 as libc::c_int as usize] =
        (*parent).v.origin[0 as libc::c_int as usize];
    (*ent).v.oldorigin[1 as libc::c_int as usize] =
        (*parent).v.origin[1 as libc::c_int as usize];
    (*ent).v.oldorigin[2 as libc::c_int as usize] =
        (*parent).v.origin[2 as libc::c_int as usize];
    (*ent).v.avelocity[0 as libc::c_int as usize] =
        (*parent).v.angles[0 as libc::c_int as usize];
    (*ent).v.avelocity[1 as libc::c_int as usize] =
        (*parent).v.angles[1 as libc::c_int as usize];
    (*ent).v.avelocity[2 as libc::c_int as usize] =
        (*parent).v.angles[2 as libc::c_int as usize];
}
/*
=============
SV_PhysicsNoclip

A moving object that doesn't obey physics
=============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_Physics_Noclip(mut ent: *mut edict_t) {
    // regular thinking
    if SV_RunThink(ent) as u64 == 0 { return }
    SV_CheckWater(ent);
    (*ent).v.origin[0 as libc::c_int as usize] =
        (*ent).v.origin[0 as libc::c_int as usize] +
            sv.frametime * (*ent).v.velocity[0 as libc::c_int as usize];
    (*ent).v.origin[1 as libc::c_int as usize] =
        (*ent).v.origin[1 as libc::c_int as usize] +
            sv.frametime * (*ent).v.velocity[1 as libc::c_int as usize];
    (*ent).v.origin[2 as libc::c_int as usize] =
        (*ent).v.origin[2 as libc::c_int as usize] +
            sv.frametime * (*ent).v.velocity[2 as libc::c_int as usize];
    (*ent).v.angles[0 as libc::c_int as usize] =
        (*ent).v.angles[0 as libc::c_int as usize] +
            sv.frametime * (*ent).v.avelocity[0 as libc::c_int as usize];
    (*ent).v.angles[1 as libc::c_int as usize] =
        (*ent).v.angles[1 as libc::c_int as usize] +
            sv.frametime * (*ent).v.avelocity[1 as libc::c_int as usize];
    (*ent).v.angles[2 as libc::c_int as usize] =
        (*ent).v.angles[2 as libc::c_int as usize] +
            sv.frametime * (*ent).v.avelocity[2 as libc::c_int as usize];
    // noclip ents never touch triggers
    SV_LinkEdict(ent, false_0);
}
/*
==============================================================================

TOSS / BOUNCE

==============================================================================
*/
/*
=============
SV_CheckWaterTransition

=============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_CheckWaterTransition(mut ent: *mut edict_t) {
    let mut point: vec3_t = [0.; 3];
    let mut cont: libc::c_int = 0;
    point[0 as libc::c_int as usize] =
        ((*ent).v.absmax[0 as libc::c_int as usize] +
             (*ent).v.absmin[0 as libc::c_int as usize]) * 0.5f32;
    point[1 as libc::c_int as usize] =
        ((*ent).v.absmax[1 as libc::c_int as usize] +
             (*ent).v.absmin[1 as libc::c_int as usize]) * 0.5f32;
    point[2 as libc::c_int as usize] =
        (*ent).v.absmin[2 as libc::c_int as usize] + 1.0f32;
    svs.groupmask = (*ent).v.groupinfo;
    cont = SV_PointContents(point.as_mut_ptr() as *const vec_t);
    if (*ent).v.watertype == 0 {
        // just spawned here
        (*ent).v.watertype = cont;
        (*ent).v.waterlevel = 1 as libc::c_int;
        return
    }
    if cont <= -(3 as libc::c_int) && cont > -(15 as libc::c_int) {
        if (*ent).v.watertype == -(1 as libc::c_int) {
            // just crossed into water
            SV_StartSound(ent, 0 as libc::c_int,
                          b"player/pl_wade1.wav\x00" as *const u8 as
                              *const libc::c_char, 1.0f32,
                          0.8f64 as libc::c_float, 0 as libc::c_int,
                          100 as libc::c_int);
            (*ent).v.velocity[2 as libc::c_int as usize] *= 0.5f32
        }
        (*ent).v.watertype = cont;
        (*ent).v.waterlevel = 1 as libc::c_int;
        if (*ent).v.absmin[2 as libc::c_int as usize] !=
               (*ent).v.absmax[2 as libc::c_int as usize] {
            point[2 as libc::c_int as usize] =
                ((*ent).v.absmin[2 as libc::c_int as usize] +
                     (*ent).v.absmax[2 as libc::c_int as usize]) * 0.5f32;
            svs.groupmask = (*ent).v.groupinfo;
            cont = SV_PointContents(point.as_mut_ptr() as *const vec_t);
            if cont <= -(3 as libc::c_int) && cont > -(15 as libc::c_int) {
                (*ent).v.waterlevel = 2 as libc::c_int;
                point[0 as libc::c_int as usize] =
                    point[0 as libc::c_int as usize] +
                        (*ent).v.view_ofs[0 as libc::c_int as usize];
                point[1 as libc::c_int as usize] =
                    point[1 as libc::c_int as usize] +
                        (*ent).v.view_ofs[1 as libc::c_int as usize];
                point[2 as libc::c_int as usize] =
                    point[2 as libc::c_int as usize] +
                        (*ent).v.view_ofs[2 as libc::c_int as usize];
                svs.groupmask = (*ent).v.groupinfo;
                cont = SV_PointContents(point.as_mut_ptr() as *const vec_t);
                if cont <= -(3 as libc::c_int) && cont > -(15 as libc::c_int)
                   {
                    (*ent).v.waterlevel = 3 as libc::c_int
                }
            }
        } else {
            // point entity
            (*ent).v.waterlevel = 3 as libc::c_int
        }
    } else {
        if (*ent).v.watertype != -(1 as libc::c_int) {
            // just crossed into water
            SV_StartSound(ent, 0 as libc::c_int,
                          b"player/pl_wade2.wav\x00" as *const u8 as
                              *const libc::c_char, 1.0f32,
                          0.8f64 as libc::c_float, 0 as libc::c_int,
                          100 as libc::c_int);
        }
        (*ent).v.watertype = -(1 as libc::c_int);
        (*ent).v.waterlevel = 0 as libc::c_int
    };
}
/*
=============
SV_Physics_Toss

Toss, bounce, and fly movement.  When onground, do nothing.
=============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_Physics_Toss(mut ent: *mut edict_t) {
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
    let mut move_0: vec3_t = [0.; 3];
    let mut backoff: libc::c_float = 0.;
    let mut ground: *mut edict_t = 0 as *mut edict_t;
    SV_CheckWater(ent);
    // regular thinking
    if SV_RunThink(ent) as u64 == 0 { return }
    ground = (*ent).v.groundentity;
    if (*ent).v.velocity[2 as libc::c_int as usize] >
           0 as libc::c_int as libc::c_float {
        (*ent).v.flags =
            ((*ent).v.flags as libc::c_uint &
                 !((1 as libc::c_uint) << 9 as libc::c_int)) as libc::c_int
    }
    if SV_CheckEdict(ground,
                     b"../engine/server/sv_phys.c\x00" as *const u8 as
                         *const libc::c_char, 1443 as libc::c_int) as u64 == 0
           ||
           (*ground).v.flags as libc::c_uint &
               ((1 as libc::c_uint) << 5 as libc::c_int |
                    (1 as libc::c_uint) << 3 as libc::c_int) != 0 {
        (*ent).v.flags =
            ((*ent).v.flags as libc::c_uint &
                 !((1 as libc::c_uint) << 9 as libc::c_int)) as libc::c_int
    }
    // if on ground and not moving, return.
    if (*ent).v.flags as libc::c_uint &
           (1 as libc::c_uint) << 9 as libc::c_int != 0 &&
           ((*ent).v.velocity[0 as libc::c_int as usize] == 0.0f32 &&
                (*ent).v.velocity[1 as libc::c_int as usize] == 0.0f32 &&
                (*ent).v.velocity[2 as libc::c_int as usize] == 0.0f32) {
        (*ent).v.avelocity[2 as libc::c_int as usize] =
            0 as libc::c_int as vec_t;
        (*ent).v.avelocity[1 as libc::c_int as usize] =
            (*ent).v.avelocity[2 as libc::c_int as usize];
        (*ent).v.avelocity[0 as libc::c_int as usize] =
            (*ent).v.avelocity[1 as libc::c_int as usize];
        if (*ent).v.basevelocity[0 as libc::c_int as usize] == 0.0f32 &&
               (*ent).v.basevelocity[1 as libc::c_int as usize] == 0.0f32 &&
               (*ent).v.basevelocity[2 as libc::c_int as usize] == 0.0f32 {
            return
        }
        // at rest
    }
    SV_CheckVelocity(ent);
    // add gravity
    match (*ent).v.movetype { 5 | 9 | 11 => { } _ => { SV_AddGravity(ent); } }
    // move angles (with friction)
    match (*ent).v.movetype {
        6 | 10 => { SV_AngularMove(ent, sv.frametime, (*ent).v.friction); }
        _ => { SV_AngularMove(ent, sv.frametime, 0.0f32); }
    }
    // move origin
	// Base velocity is not properly accounted for since this entity will move again
	// after the bounce without taking it into account
    (*ent).v.velocity[0 as libc::c_int as usize] =
        (*ent).v.velocity[0 as libc::c_int as usize] +
            (*ent).v.basevelocity[0 as libc::c_int as usize];
    (*ent).v.velocity[1 as libc::c_int as usize] =
        (*ent).v.velocity[1 as libc::c_int as usize] +
            (*ent).v.basevelocity[1 as libc::c_int as usize];
    (*ent).v.velocity[2 as libc::c_int as usize] =
        (*ent).v.velocity[2 as libc::c_int as usize] +
            (*ent).v.basevelocity[2 as libc::c_int as usize];
    SV_CheckVelocity(ent);
    move_0[0 as libc::c_int as usize] =
        (*ent).v.velocity[0 as libc::c_int as usize] * sv.frametime;
    move_0[1 as libc::c_int as usize] =
        (*ent).v.velocity[1 as libc::c_int as usize] * sv.frametime;
    move_0[2 as libc::c_int as usize] =
        (*ent).v.velocity[2 as libc::c_int as usize] * sv.frametime;
    (*ent).v.velocity[0 as libc::c_int as usize] =
        (*ent).v.velocity[0 as libc::c_int as usize] -
            (*ent).v.basevelocity[0 as libc::c_int as usize];
    (*ent).v.velocity[1 as libc::c_int as usize] =
        (*ent).v.velocity[1 as libc::c_int as usize] -
            (*ent).v.basevelocity[1 as libc::c_int as usize];
    (*ent).v.velocity[2 as libc::c_int as usize] =
        (*ent).v.velocity[2 as libc::c_int as usize] -
            (*ent).v.basevelocity[2 as libc::c_int as usize];
    trace =
        SV_PushEntity(ent, move_0.as_mut_ptr() as *const vec_t,
                      vec3_origin.as_mut_ptr() as *const vec_t,
                      0 as *mut libc::c_int, 0.0f32);
    if (*ent).free as u64 != 0 { return }
    SV_CheckVelocity(ent);
    if trace.allsolid as u64 != 0 {
        // entity is trapped in another solid
        (*ent).v.avelocity[2 as libc::c_int as usize] =
            0 as libc::c_int as vec_t;
        (*ent).v.avelocity[1 as libc::c_int as usize] =
            (*ent).v.avelocity[2 as libc::c_int as usize];
        (*ent).v.avelocity[0 as libc::c_int as usize] =
            (*ent).v.avelocity[1 as libc::c_int as usize];
        (*ent).v.velocity[2 as libc::c_int as usize] =
            0 as libc::c_int as vec_t;
        (*ent).v.velocity[1 as libc::c_int as usize] =
            (*ent).v.velocity[2 as libc::c_int as usize];
        (*ent).v.velocity[0 as libc::c_int as usize] =
            (*ent).v.velocity[1 as libc::c_int as usize];
        return
    }
    if trace.fraction == 1.0f32 { SV_CheckWaterTransition(ent); return }
    if (*ent).v.movetype == 10 as libc::c_int {
        backoff = 2.0f32 - (*ent).v.friction
    } else if (*ent).v.movetype == 11 as libc::c_int {
        backoff = 2.0f32
    } else { backoff = 1.0f32 }
    SV_ClipVelocity((*ent).v.velocity.as_mut_ptr(),
                    trace.plane.normal.as_mut_ptr(),
                    (*ent).v.velocity.as_mut_ptr(), backoff);
    // stop if on ground
    if trace.plane.normal[2 as libc::c_int as usize] > 0.7f32 {
        let mut vel: libc::c_float = 0.;
        move_0[0 as libc::c_int as usize] =
            (*ent).v.velocity[0 as libc::c_int as usize] +
                (*ent).v.basevelocity[0 as libc::c_int as usize];
        move_0[1 as libc::c_int as usize] =
            (*ent).v.velocity[1 as libc::c_int as usize] +
                (*ent).v.basevelocity[1 as libc::c_int as usize];
        move_0[2 as libc::c_int as usize] =
            (*ent).v.velocity[2 as libc::c_int as usize] +
                (*ent).v.basevelocity[2 as libc::c_int as usize];
        vel =
            move_0[0 as libc::c_int as usize] *
                move_0[0 as libc::c_int as usize] +
                move_0[1 as libc::c_int as usize] *
                    move_0[1 as libc::c_int as usize] +
                move_0[2 as libc::c_int as usize] *
                    move_0[2 as libc::c_int as usize];
        if (*ent).v.velocity[2 as libc::c_int as usize] <
               sv_gravity.value * sv.frametime {
            // we're rolling on the ground, add static friction.
            (*ent).v.groundentity = trace.ent;
            (*ent).v.flags =
                ((*ent).v.flags as libc::c_uint |
                     (1 as libc::c_uint) << 9 as libc::c_int) as libc::c_int;
            (*ent).v.velocity[2 as libc::c_int as usize] = 0.0f32
        }
        if vel < 900.0f32 ||
               (*ent).v.movetype != 10 as libc::c_int &&
                   (*ent).v.movetype != 11 as libc::c_int {
            (*ent).v.flags =
                ((*ent).v.flags as libc::c_uint |
                     (1 as libc::c_uint) << 9 as libc::c_int) as libc::c_int;
            (*ent).v.groundentity = trace.ent;
            (*ent).v.avelocity[2 as libc::c_int as usize] =
                0 as libc::c_int as vec_t;
            (*ent).v.avelocity[1 as libc::c_int as usize] =
                (*ent).v.avelocity[2 as libc::c_int as usize];
            (*ent).v.avelocity[0 as libc::c_int as usize] =
                (*ent).v.avelocity[1 as libc::c_int as usize];
            (*ent).v.velocity[2 as libc::c_int as usize] =
                0 as libc::c_int as vec_t;
            (*ent).v.velocity[1 as libc::c_int as usize] =
                (*ent).v.velocity[2 as libc::c_int as usize];
            (*ent).v.velocity[0 as libc::c_int as usize] =
                (*ent).v.velocity[1 as libc::c_int as usize]
        } else {
            move_0[0 as libc::c_int as usize] =
                (*ent).v.velocity[0 as libc::c_int as usize] *
                    ((1.0f32 - trace.fraction) * sv.frametime * 0.9f32);
            move_0[1 as libc::c_int as usize] =
                (*ent).v.velocity[1 as libc::c_int as usize] *
                    ((1.0f32 - trace.fraction) * sv.frametime * 0.9f32);
            move_0[2 as libc::c_int as usize] =
                (*ent).v.velocity[2 as libc::c_int as usize] *
                    ((1.0f32 - trace.fraction) * sv.frametime * 0.9f32);
            move_0[0 as libc::c_int as usize] =
                move_0[0 as libc::c_int as usize] +
                    (1.0f32 - trace.fraction) * sv.frametime * 0.9f32 *
                        (*ent).v.basevelocity[0 as libc::c_int as usize];
            move_0[1 as libc::c_int as usize] =
                move_0[1 as libc::c_int as usize] +
                    (1.0f32 - trace.fraction) * sv.frametime * 0.9f32 *
                        (*ent).v.basevelocity[1 as libc::c_int as usize];
            move_0[2 as libc::c_int as usize] =
                move_0[2 as libc::c_int as usize] +
                    (1.0f32 - trace.fraction) * sv.frametime * 0.9f32 *
                        (*ent).v.basevelocity[2 as libc::c_int as usize];
            trace =
                SV_PushEntity(ent, move_0.as_mut_ptr() as *const vec_t,
                              vec3_origin.as_mut_ptr() as *const vec_t,
                              0 as *mut libc::c_int, 0.0f32);
            if (*ent).free as u64 != 0 { return }
        }
    }
    // check for in water
    SV_CheckWaterTransition(ent);
}
/*
===============================================================================

STEPPING MOVEMENT

===============================================================================
*/
/*
=============
SV_Physics_Step

Monsters freefall when they don't have a ground entity, otherwise
all movement is done with discrete steps.

This is also used for objects that have become still on the ground, but
will fall if the floor is pulled out from under them.
=============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_Physics_Step(mut ent: *mut edict_t) {
    let mut inwater: qboolean = false_0; // DotProduct2D
    let mut wasonground: qboolean = false_0; // factor
    let mut wasonmover: qboolean = false_0; // g-cont. ???
    let mut mins: vec3_t = [0.; 3]; // add a little friction
    let mut maxs: vec3_t = [0.; 3];
    let mut point: vec3_t = [0.; 3];
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
    SV_WaterMove(ent);
    SV_CheckVelocity(ent);
    wasonground =
        ((*ent).v.flags as libc::c_uint &
             (1 as libc::c_uint) << 9 as libc::c_int) as qboolean;
    wasonmover = SV_CheckMover(ent);
    inwater = SV_CheckWater(ent);
    if (*ent).v.flags as libc::c_uint &
           (1 as libc::c_uint) << 15 as libc::c_int != 0 &&
           (*ent).v.waterlevel > 0 as libc::c_int {
        let mut buoyancy: libc::c_float =
            SV_Submerged(ent) * (*ent).v.skin as libc::c_float * sv.frametime;
        SV_AddGravity(ent);
        (*ent).v.velocity[2 as libc::c_int as usize] += buoyancy
    }
    if wasonground as u64 == 0 {
        if (*ent).v.flags as libc::c_uint &
               (1 as libc::c_uint) << 0 as libc::c_int == 0 {
            if (*ent).v.flags as libc::c_uint &
                   (1 as libc::c_uint) << 1 as libc::c_int == 0 ||
                   (*ent).v.waterlevel <= 0 as libc::c_int {
                if inwater as u64 == 0 { SV_AddGravity(ent); }
            }
        }
    }
    if !((*ent).v.velocity[0 as libc::c_int as usize] == 0.0f32 &&
             (*ent).v.velocity[1 as libc::c_int as usize] == 0.0f32 &&
             (*ent).v.velocity[2 as libc::c_int as usize] == 0.0f32) ||
           !((*ent).v.basevelocity[0 as libc::c_int as usize] == 0.0f32 &&
                 (*ent).v.basevelocity[1 as libc::c_int as usize] == 0.0f32 &&
                 (*ent).v.basevelocity[2 as libc::c_int as usize] == 0.0f32) {
        (*ent).v.flags =
            ((*ent).v.flags as libc::c_uint &
                 !((1 as libc::c_uint) << 9 as libc::c_int)) as libc::c_int;
        if (wasonground as libc::c_uint != 0 ||
                wasonmover as libc::c_uint != 0) &&
               ((*ent).v.health > 0 as libc::c_int as libc::c_float ||
                    SV_CheckBottom(ent, 0 as libc::c_int) as libc::c_uint !=
                        0) {
            let mut vel: *mut libc::c_float = (*ent).v.velocity.as_mut_ptr();
            let mut control: libc::c_float = 0.;
            let mut speed: libc::c_float = 0.;
            let mut newspeed: libc::c_float = 0.;
            let mut friction: libc::c_float = 0.;
            speed =
                __tg_sqrt(*vel.offset(0 as libc::c_int as isize) *
                              *vel.offset(0 as libc::c_int as isize) +
                              *vel.offset(1 as libc::c_int as isize) *
                                  *vel.offset(1 as libc::c_int as isize));
            if speed != 0. {
                friction = sv_friction.value * (*ent).v.friction;
                (*ent).v.friction = 1.0f32;
                if wasonmover as u64 != 0 { friction *= 0.5f32 }
                control =
                    if speed < sv_stopspeed.value {
                        sv_stopspeed.value
                    } else { speed };
                newspeed = speed - sv.frametime * control * friction;
                if newspeed < 0 as libc::c_int as libc::c_float {
                    newspeed = 0 as libc::c_int as libc::c_float
                }
                newspeed /= speed;
                *vel.offset(0 as libc::c_int as isize) =
                    *vel.offset(0 as libc::c_int as isize) * newspeed;
                *vel.offset(1 as libc::c_int as isize) =
                    *vel.offset(1 as libc::c_int as isize) * newspeed
            }
        }
        (*ent).v.velocity[0 as libc::c_int as usize] =
            (*ent).v.velocity[0 as libc::c_int as usize] +
                (*ent).v.basevelocity[0 as libc::c_int as usize];
        (*ent).v.velocity[1 as libc::c_int as usize] =
            (*ent).v.velocity[1 as libc::c_int as usize] +
                (*ent).v.basevelocity[1 as libc::c_int as usize];
        (*ent).v.velocity[2 as libc::c_int as usize] =
            (*ent).v.velocity[2 as libc::c_int as usize] +
                (*ent).v.basevelocity[2 as libc::c_int as usize];
        SV_CheckVelocity(ent);
        SV_FlyMove(ent, sv.frametime, 0 as *mut trace_t);
        if (*ent).free as u64 != 0 { return }
        SV_CheckVelocity(ent);
        (*ent).v.velocity[0 as libc::c_int as usize] =
            (*ent).v.velocity[0 as libc::c_int as usize] -
                (*ent).v.basevelocity[0 as libc::c_int as usize];
        (*ent).v.velocity[1 as libc::c_int as usize] =
            (*ent).v.velocity[1 as libc::c_int as usize] -
                (*ent).v.basevelocity[1 as libc::c_int as usize];
        (*ent).v.velocity[2 as libc::c_int as usize] =
            (*ent).v.velocity[2 as libc::c_int as usize] -
                (*ent).v.basevelocity[2 as libc::c_int as usize];
        SV_CheckVelocity(ent);
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
        point[2 as libc::c_int as usize] =
            mins[2 as libc::c_int as usize] - 1.0f32;
        x = 0 as libc::c_int;
        while x <= 1 as libc::c_int {
            if (*ent).v.flags as libc::c_uint &
                   (1 as libc::c_uint) << 9 as libc::c_int != 0 {
                break ;
            }
            y = 0 as libc::c_int;
            while y <= 1 as libc::c_int {
                point[0 as libc::c_int as usize] =
                    if x != 0 {
                        maxs[0 as libc::c_int as usize]
                    } else { mins[0 as libc::c_int as usize] };
                point[1 as libc::c_int as usize] =
                    if y != 0 {
                        maxs[1 as libc::c_int as usize]
                    } else { mins[1 as libc::c_int as usize] };
                trace =
                    SV_Move(point.as_mut_ptr() as *const vec_t,
                            vec3_origin.as_mut_ptr(),
                            vec3_origin.as_mut_ptr(),
                            point.as_mut_ptr() as *const vec_t,
                            0 as libc::c_int, ent, false_0);
                if trace.startsolid as u64 != 0 {
                    (*ent).v.flags =
                        ((*ent).v.flags as libc::c_uint |
                             (1 as libc::c_uint) << 9 as libc::c_int) as
                            libc::c_int;
                    (*ent).v.groundentity = trace.ent;
                    (*ent).v.friction = 1.0f32;
                    break ;
                } else { y += 1 }
            }
            x += 1
        }
        SV_LinkEdict(ent, true_0);
    } else if (*svgame.globals).force_retouch !=
                  0 as libc::c_int as libc::c_float {
        let mut monsterClip: qboolean =
            if (*ent).v.flags as libc::c_uint &
                   (1 as libc::c_uint) << 23 as libc::c_int != 0 {
                true_0 as libc::c_int
            } else { false_0 as libc::c_int } as qboolean;
        trace =
            SV_Move((*ent).v.origin.as_mut_ptr() as *const vec_t,
                    (*ent).v.mins.as_mut_ptr(), (*ent).v.maxs.as_mut_ptr(),
                    (*ent).v.origin.as_mut_ptr() as *const vec_t,
                    0 as libc::c_int, ent, monsterClip);
        // hentacle impact code
        if (trace.fraction < 1.0f32 || trace.startsolid as libc::c_uint != 0)
               &&
               SV_CheckEdict(trace.ent,
                             b"../engine/server/sv_phys.c\x00" as *const u8 as
                                 *const libc::c_char, 1684 as libc::c_int) as
                   libc::c_uint != 0 {
            SV_Impact(ent, trace.ent, &mut trace);
            if (*ent).free as u64 != 0 { return }
        }
    }
    if SV_RunThink(ent) as u64 == 0 { return }
    SV_CheckWaterTransition(ent);
}
/*
=============
SV_PhysicsNone

Non moving objects can only think
=============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_Physics_None(mut ent: *mut edict_t) {
    SV_RunThink(ent);
}
//============================================================================
unsafe extern "C" fn SV_Physics_Entity(mut ent: *mut edict_t) {
    // user dll can override movement type (Xash3D extension)
    if svgame.physFuncs.SV_PhysicsEntity.is_some() &&
           svgame.physFuncs.SV_PhysicsEntity.expect("non-null function pointer")(ent)
               != 0 {
        return
    } // overrided
    SV_UpdateBaseVelocity(ent);
    if (*ent).v.flags as libc::c_uint &
           (1 as libc::c_uint) << 22 as libc::c_int == 0 &&
           !((*ent).v.basevelocity[0 as libc::c_int as usize] == 0.0f32 &&
                 (*ent).v.basevelocity[1 as libc::c_int as usize] == 0.0f32 &&
                 (*ent).v.basevelocity[2 as libc::c_int as usize] == 0.0f32) {
        // Apply momentum (add in half of the previous frame of velocity first)
        (*ent).v.velocity[0 as libc::c_int as usize] =
            (*ent).v.velocity[0 as libc::c_int as usize] +
                (1.0f32 + sv.frametime * 0.5f32) *
                    (*ent).v.basevelocity[0 as libc::c_int as usize];
        (*ent).v.velocity[1 as libc::c_int as usize] =
            (*ent).v.velocity[1 as libc::c_int as usize] +
                (1.0f32 + sv.frametime * 0.5f32) *
                    (*ent).v.basevelocity[1 as libc::c_int as usize];
        (*ent).v.velocity[2 as libc::c_int as usize] =
            (*ent).v.velocity[2 as libc::c_int as usize] +
                (1.0f32 + sv.frametime * 0.5f32) *
                    (*ent).v.basevelocity[2 as libc::c_int as usize];
        (*ent).v.basevelocity[2 as libc::c_int as usize] =
            0 as libc::c_int as vec_t;
        (*ent).v.basevelocity[1 as libc::c_int as usize] =
            (*ent).v.basevelocity[2 as libc::c_int as usize];
        (*ent).v.basevelocity[0 as libc::c_int as usize] =
            (*ent).v.basevelocity[1 as libc::c_int as usize]
    }
    (*ent).v.flags =
        ((*ent).v.flags as libc::c_uint &
             !((1 as libc::c_uint) << 22 as libc::c_int)) as libc::c_int;
    if (*svgame.globals).force_retouch != 0.0f32 {
        // force retouch even for stationary
        SV_LinkEdict(ent, true_0);
    }
    match (*ent).v.movetype {
        0 => { SV_Physics_None(ent); }
        8 => { SV_Physics_Noclip(ent); }
        12 => { SV_Physics_Follow(ent); }
        14 => { SV_Physics_Compound(ent); }
        4 | 13 => { SV_Physics_Step(ent); }
        5 | 6 | 10 | 9 | 11 => { SV_Physics_Toss(ent); }
        7 => { SV_Physics_Pusher(ent); }
        3 => {
            Host_Error(b"SV_Physics: bad movetype %i\n\x00" as *const u8 as
                           *const libc::c_char, (*ent).v.movetype);
        }
        _ => { }
    }
    // g-cont. don't alow free entities during loading because
	// this produce a corrupted baselines
    if sv.state as libc::c_uint == ss_active as libc::c_int as libc::c_uint &&
           (*ent).v.flags as libc::c_uint &
               (1 as libc::c_uint) << 30 as libc::c_int != 0 {
        SV_FreeEdict(ent);
    };
}
/*
================
SV_Physics

================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_Physics() {
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    let mut i: libc::c_int = 0;
    SV_CheckAllEnts();
    (*svgame.globals).time = sv.time as libc::c_float;
    // let the progs know that a new frame has started
    svgame.dllFuncs.pfnStartFrame.expect("non-null function pointer")();
    // treat each object in turn
    i = 0 as libc::c_int;
    while i < svgame.numEntities {
        ent = SV_EdictNum(i);
        if !(SV_CheckEdict(ent,
                           b"../engine/server/sv_phys.c\x00" as *const u8 as
                               *const libc::c_char, 1795 as libc::c_int) as
                 u64 == 0) {
            if !(i > 0 as libc::c_int && i <= svs.maxclients) {
                SV_Physics_Entity(ent);
            }
        }
        i += 1
    }
    if (*svgame.globals).force_retouch != 0.0f32 {
        (*svgame.globals).force_retouch -= 1.
    }
    if svgame.physFuncs.SV_EndFrame.is_some() {
        svgame.physFuncs.SV_EndFrame.expect("non-null function pointer")();
    }
    // animate lightstyles (used for GetEntityIllum)
    SV_RunLightStyles();
    // increase framecount
    sv.framecount += 1;
    // decrement svgame.numEntities if the highest number entities died
    while (*SV_EdictNum(svgame.numEntities - 1 as libc::c_int)).free as u64 !=
              0 {
        svgame.numEntities -= 1
    };
}
/*
================
SV_GetServerTime

Inplementation for new physics interface
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_GetServerTime() -> libc::c_double {
    return sv.time;
}
/*
================
SV_GetFrameTime

Inplementation for new physics interface
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_GetFrameTime() -> libc::c_double {
    return sv.frametime as libc::c_double;
}
/*
================
SV_GetHeadNode

Inplementation for new physics interface
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_GetHeadNode() -> *mut areanode_t {
    return sv_areanodes.as_mut_ptr();
}
/*
================
SV_ServerState

Inplementation for new physics interface
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_ServerState() -> libc::c_int {
    return sv.state as libc::c_int;
}
/*
================
SV_DrawDebugTriangles

Called from renderer for debug purposes
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_DrawDebugTriangles() {
    if host.type_0 != HOST_NORMAL as libc::c_int as libc::c_uint { return }
    if svgame.physFuncs.DrawNormalTriangles.is_some() {
        // draw solid overlay
        svgame.physFuncs.DrawNormalTriangles.expect("non-null function pointer")();
    }
    if svgame.physFuncs.DrawDebugTriangles.is_some() {
        // draw wireframe overlay
        svgame.physFuncs.DrawDebugTriangles.expect("non-null function pointer")();
    };
}
/*
================
SV_DrawOrthoTriangles

Called from renderer for debug purposes
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_DrawOrthoTriangles() {
    if host.type_0 != HOST_NORMAL as libc::c_int as libc::c_uint { return }
    if svgame.physFuncs.DrawOrthoTriangles.is_some() {
        // draw solid overlay
        svgame.physFuncs.DrawOrthoTriangles.expect("non-null function pointer")();
    };
}
#[no_mangle]
pub unsafe extern "C" fn SV_UpdateFogSettings(mut packed_fog: libc::c_uint) {
    svgame.movevars.fog_settings = packed_fog as libc::c_int;
    host.movevars_changed = true_0;
    // force to transmit
}
/*
=========
pfnGetFilesList

=========
*/
unsafe extern "C" fn pfnGetFilesList(mut pattern: *const libc::c_char,
                                     mut numFiles: *mut libc::c_int,
                                     mut gamedironly: libc::c_int)
 -> *mut *mut libc::c_char {
    static mut t: *mut search_t =
        0 as *const search_t as *mut search_t; // release prev search
    if !t.is_null() {
        _Mem_Free(t as *mut libc::c_void,
                  b"../engine/server/sv_phys.c\x00" as *const u8 as
                      *const libc::c_char, 1939 as libc::c_int);
    }
    t = FS_Search(pattern, true_0 as libc::c_int, gamedironly);
    if t.is_null() {
        if !numFiles.is_null() { *numFiles = 0 as libc::c_int }
        return 0 as *mut *mut libc::c_char
    }
    if !numFiles.is_null() { *numFiles = (*t).numfilenames }
    return (*t).filenames;
}
unsafe extern "C" fn pfnMem_Alloc(mut cb: size_t,
                                  mut filename: *const libc::c_char,
                                  fileline: libc::c_int)
 -> *mut libc::c_void {
    return _Mem_Alloc(svgame.mempool, cb, true_0, filename, fileline);
}
unsafe extern "C" fn pfnMem_Free(mut mem: *mut libc::c_void,
                                 mut filename: *const libc::c_char,
                                 fileline: libc::c_int) {
    if mem.is_null() { return }
    _Mem_Free(mem, filename, fileline);
}
/*
=============
pfnPointContents

=============
*/
unsafe extern "C" fn pfnPointContents(mut pos: *const libc::c_float,
                                      mut groupmask: libc::c_int)
 -> libc::c_int {
    let mut oldmask: libc::c_int = 0; // restore old mask
    let mut cont: libc::c_int = 0;
    if pos.is_null() { return 0 as libc::c_int }
    oldmask = svs.groupmask;
    svs.groupmask = groupmask;
    cont = SV_PointContents(pos);
    svs.groupmask = oldmask;
    return cont;
}
#[no_mangle]
pub unsafe extern "C" fn pfnLoadImagePixels(mut filename: *const libc::c_char,
                                            mut width: *mut libc::c_int,
                                            mut height: *mut libc::c_int)
 -> *const byte {
    let mut pic: *mut rgbdata_t =
        FS_LoadImage(filename, 0 as *const byte, 0 as libc::c_int as size_t);
    let mut buffer: *mut byte = 0 as *mut byte;
    if pic.is_null() { return 0 as *const byte }
    buffer =
        _Mem_Alloc(svgame.mempool, (*pic).size, false_0,
                   b"../engine/server/sv_phys.c\x00" as *const u8 as
                       *const libc::c_char, 1991 as libc::c_int) as *mut byte;
    if !buffer.is_null() {
        memcpy(buffer as *mut libc::c_void,
               (*pic).buffer as *const libc::c_void, (*pic).size);
    }
    if !width.is_null() { *width = (*pic).width as libc::c_int }
    if !height.is_null() { *height = (*pic).height as libc::c_int }
    FS_FreeImage(pic);
    return buffer;
}
#[no_mangle]
pub unsafe extern "C" fn pfnGetModelName(mut modelindex: libc::c_int)
 -> *const libc::c_char {
    if modelindex < 0 as libc::c_int || modelindex >= 1024 as libc::c_int {
        return 0 as *const libc::c_char
    }
    return sv.model_precache[modelindex as usize].as_mut_ptr();
}
unsafe extern "C" fn GL_TextureData(mut texnum: libc::c_uint) -> *const byte {
    return if host.type_0 == HOST_DEDICATED as libc::c_int as libc::c_uint {
               0 as *const byte
           } else {
               ref_0.dllFuncs.GL_TextureData.expect("non-null function pointer")(texnum)
           };
    // XASH_DEDICATED
    // XASH_DEDICATED
}
static mut gPhysicsAPI: server_physics_api_t =
    unsafe {
        {
            let mut init =
                server_physics_api_s{pfnLinkEdict:
                                         Some(SV_LinkEdict as
                                                  unsafe extern "C" fn(_:
                                                                           *mut edict_t,
                                                                       _:
                                                                           qboolean)
                                                      -> ()),
                                     pfnGetServerTime:
                                         Some(SV_GetServerTime as
                                                  unsafe extern "C" fn()
                                                      -> libc::c_double),
                                     pfnGetFrameTime:
                                         Some(SV_GetFrameTime as
                                                  unsafe extern "C" fn()
                                                      -> libc::c_double),
                                     pfnGetModel:
                                         ::std::mem::transmute::<*mut libc::c_void,
                                                                 Option<unsafe extern "C" fn(_:
                                                                                                 libc::c_int)
                                                                            ->
                                                                                *mut libc::c_void>>(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                                                                            libc::c_int)
                                                                                                                                       ->
                                                                                                                                           *mut model_t>,
                                                                                                                            *mut libc::c_void>(Some(SV_ModelHandle
                                                                                                                                                        as
                                                                                                                                                        unsafe extern "C" fn(_:
                                                                                                                                                                                 libc::c_int)
                                                                                                                                                            ->
                                                                                                                                                                *mut model_t))),
                                     pfnGetHeadnode:
                                         Some(SV_GetHeadNode as
                                                  unsafe extern "C" fn()
                                                      -> *mut areanode_t),
                                     pfnServerState:
                                         Some(SV_ServerState as
                                                  unsafe extern "C" fn()
                                                      -> libc::c_int),
                                     pfnHost_Error:
                                         Some(Host_Error as
                                                  unsafe extern "C" fn(_:
                                                                           *const libc::c_char,
                                                                       _: ...)
                                                      -> ()),
                                     pTriAPI:
                                         &gTriApi as *const triangleapi_t as
                                             *mut triangleapi_t,
                                     pfnDrawConsoleString:
                                         Some(pfnDrawConsoleString as
                                                  unsafe extern "C" fn(_:
                                                                           libc::c_int,
                                                                       _:
                                                                           libc::c_int,
                                                                       _:
                                                                           *mut libc::c_char)
                                                      -> libc::c_int),
                                     pfnDrawSetTextColor:
                                         Some(pfnDrawSetTextColor as
                                                  unsafe extern "C" fn(_:
                                                                           libc::c_float,
                                                                       _:
                                                                           libc::c_float,
                                                                       _:
                                                                           libc::c_float)
                                                      -> ()),
                                     pfnDrawConsoleStringLen:
                                         Some(pfnDrawConsoleStringLen as
                                                  unsafe extern "C" fn(_:
                                                                           *const libc::c_char,
                                                                       _:
                                                                           *mut libc::c_int,
                                                                       _:
                                                                           *mut libc::c_int)
                                                      -> ()),
                                     Con_NPrintf:
                                         Some(Con_NPrintf as
                                                  unsafe extern "C" fn(_:
                                                                           libc::c_int,
                                                                       _:
                                                                           *const libc::c_char,
                                                                       _: ...)
                                                      -> ()),
                                     Con_NXPrintf:
                                         Some(Con_NXPrintf as
                                                  unsafe extern "C" fn(_:
                                                                           *mut con_nprint_t,
                                                                       _:
                                                                           *const libc::c_char,
                                                                       _: ...)
                                                      -> ()),
                                     pfnGetLightStyle:
                                         Some(SV_GetLightStyle as
                                                  unsafe extern "C" fn(_:
                                                                           libc::c_int)
                                                      -> *const libc::c_char),
                                     pfnUpdateFogSettings:
                                         Some(SV_UpdateFogSettings as
                                                  unsafe extern "C" fn(_:
                                                                           libc::c_uint)
                                                      -> ()),
                                     pfnGetFilesList:
                                         Some(pfnGetFilesList as
                                                  unsafe extern "C" fn(_:
                                                                           *const libc::c_char,
                                                                       _:
                                                                           *mut libc::c_int,
                                                                       _:
                                                                           libc::c_int)
                                                      ->
                                                          *mut *mut libc::c_char),
                                     pfnTraceSurface:
                                         Some(SV_TraceSurface as
                                                  unsafe extern "C" fn(_:
                                                                           *mut edict_t,
                                                                       _:
                                                                           *const vec_t,
                                                                       _:
                                                                           *const vec_t)
                                                      -> *mut msurface_t),
                                     pfnGetTextureData:
                                         Some(GL_TextureData as
                                                  unsafe extern "C" fn(_:
                                                                           libc::c_uint)
                                                      -> *const byte),
                                     pfnMemAlloc:
                                         Some(pfnMem_Alloc as
                                                  unsafe extern "C" fn(_:
                                                                           size_t,
                                                                       _:
                                                                           *const libc::c_char,
                                                                       _:
                                                                           libc::c_int)
                                                      -> *mut libc::c_void),
                                     pfnMemFree:
                                         Some(pfnMem_Free as
                                                  unsafe extern "C" fn(_:
                                                                           *mut libc::c_void,
                                                                       _:
                                                                           *const libc::c_char,
                                                                       _:
                                                                           libc::c_int)
                                                      -> ()),
                                     pfnMaskPointContents:
                                         Some(pfnPointContents as
                                                  unsafe extern "C" fn(_:
                                                                           *const libc::c_float,
                                                                       _:
                                                                           libc::c_int)
                                                      -> libc::c_int),
                                     pfnTrace:
                                         Some(SV_MoveNormal as
                                                  unsafe extern "C" fn(_:
                                                                           *const vec_t,
                                                                       _:
                                                                           *mut vec_t,
                                                                       _:
                                                                           *mut vec_t,
                                                                       _:
                                                                           *const vec_t,
                                                                       _:
                                                                           libc::c_int,
                                                                       _:
                                                                           *mut edict_t)
                                                      -> trace_t),
                                     pfnTraceNoEnts:
                                         Some(SV_MoveNoEnts as
                                                  unsafe extern "C" fn(_:
                                                                           *const vec_t,
                                                                       _:
                                                                           *mut vec_t,
                                                                       _:
                                                                           *mut vec_t,
                                                                       _:
                                                                           *const vec_t,
                                                                       _:
                                                                           libc::c_int,
                                                                       _:
                                                                           *mut edict_t)
                                                      -> trace_t),
                                     pfnBoxInPVS:
                                         ::std::mem::transmute::<*mut libc::c_void,
                                                                 Option<unsafe extern "C" fn(_:
                                                                                                 *const libc::c_float,
                                                                                             _:
                                                                                                 *const libc::c_float,
                                                                                             _:
                                                                                                 *const libc::c_float)
                                                                            ->
                                                                                libc::c_int>>(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                                                                      *const vec_t,
                                                                                                                                                  _:
                                                                                                                                                      *const vec_t,
                                                                                                                                                  _:
                                                                                                                                                      *const vec_t)
                                                                                                                                 ->
                                                                                                                                     qboolean>,
                                                                                                                      *mut libc::c_void>(Some(SV_BoxInPVS
                                                                                                                                                  as
                                                                                                                                                  unsafe extern "C" fn(_:
                                                                                                                                                                           *const vec_t,
                                                                                                                                                                       _:
                                                                                                                                                                           *const vec_t,
                                                                                                                                                                       _:
                                                                                                                                                                           *const vec_t)
                                                                                                                                                      ->
                                                                                                                                                          qboolean))),
                                     pfnWriteBytes:
                                         Some(pfnWriteBytes as
                                                  unsafe extern "C" fn(_:
                                                                           *const byte,
                                                                       _:
                                                                           libc::c_int)
                                                      -> ()),
                                     pfnCheckLump:
                                         Some(Mod_CheckLump as
                                                  unsafe extern "C" fn(_:
                                                                           *const libc::c_char,
                                                                       _:
                                                                           libc::c_int,
                                                                       _:
                                                                           *mut libc::c_int)
                                                      -> libc::c_int),
                                     pfnReadLump:
                                         Some(Mod_ReadLump as
                                                  unsafe extern "C" fn(_:
                                                                           *const libc::c_char,
                                                                       _:
                                                                           libc::c_int,
                                                                       _:
                                                                           *mut *mut libc::c_void,
                                                                       _:
                                                                           *mut libc::c_int)
                                                      -> libc::c_int),
                                     pfnSaveLump:
                                         Some(Mod_SaveLump as
                                                  unsafe extern "C" fn(_:
                                                                           *const libc::c_char,
                                                                       _:
                                                                           libc::c_int,
                                                                       _:
                                                                           *mut libc::c_void,
                                                                       _:
                                                                           libc::c_int)
                                                      -> libc::c_int),
                                     pfnSaveFile:
                                         Some(COM_SaveFile as
                                                  unsafe extern "C" fn(_:
                                                                           *const libc::c_char,
                                                                       _:
                                                                           *const libc::c_void,
                                                                       _:
                                                                           libc::c_int)
                                                      -> libc::c_int),
                                     pfnLoadImagePixels:
                                         Some(pfnLoadImagePixels as
                                                  unsafe extern "C" fn(_:
                                                                           *const libc::c_char,
                                                                       _:
                                                                           *mut libc::c_int,
                                                                       _:
                                                                           *mut libc::c_int)
                                                      -> *const byte),
                                     pfnGetModelName:
                                         Some(pfnGetModelName as
                                                  unsafe extern "C" fn(_:
                                                                           libc::c_int)
                                                      ->
                                                          *const libc::c_char),};
            init
        }
    };
/*
===============
SV_InitPhysicsAPI

Initialize server external physics
===============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_InitPhysicsAPI() -> qboolean {
    static mut pPhysIface: PHYSICAPI = None;
    pPhysIface =
        ::std::mem::transmute::<*mut libc::c_void,
                                PHYSICAPI>(COM_GetProcAddress(svgame.hInstance,
                                                              b"Server_GetPhysicsInterface\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char));
    if pPhysIface.is_some() {
        if pPhysIface.expect("non-null function pointer")(6 as libc::c_int,
                                                          &mut gPhysicsAPI,
                                                          &mut svgame.physFuncs)
               != 0 {
            Con_Reportf(b"SV_LoadProgs: ^2initailized extended PhysicAPI ^7ver. %i\n\x00"
                            as *const u8 as *const libc::c_char,
                        6 as libc::c_int);
            if svgame.physFuncs.SV_CheckFeatures.is_some() {
                // grab common engine features (it will be shared across the network)
                host.features =
                    svgame.physFuncs.SV_CheckFeatures.expect("non-null function pointer")();
                Host_PrintEngineFeatures();
            }
            return true_0
        }
        // just tell user about problems
        memset(&mut svgame.physFuncs as *mut physics_interface_t as
                   *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<physics_interface_t>() as libc::c_ulong);
        return false_0
    }
    // make sure what physic functions is cleared
    // physic interface is missed
    return true_0;
}
