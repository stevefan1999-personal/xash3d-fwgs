#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type file_s;
    pub type grasshdr_s;
    pub type IVoiceTweak_s;
    pub type demo_api_s;
    pub type engine_studio_api_s;
    pub type r_studio_interface_s;
    pub type mip_s;
    #[no_mangle]
    fn sqrtf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn COM_RandomFloat(fMin: libc::c_float, fMax: libc::c_float)
     -> libc::c_float;
    #[no_mangle]
    fn COM_RandomLong(lMin: libc::c_int, lMax: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn CL_GetEntityByIndex(index: libc::c_int) -> *mut cl_entity_s;
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_DPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Q_atof(str: *const libc::c_char) -> libc::c_float;
    #[no_mangle]
    fn Q_snprintf(buffer: *mut libc::c_char, buffersize: size_t,
                  format: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn _COM_ParseFileSafe(data: *mut libc::c_char, token: *mut libc::c_char,
                          size: libc::c_int, flags: libc::c_uint,
                          len: *mut libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn Cvar_Get(var_name: *const libc::c_char, value: *const libc::c_char,
                flags: libc::c_int, description: *const libc::c_char)
     -> *mut convar_t;
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
    fn FS_LoadFile(path: *const libc::c_char, filesizeptr: *mut fs_offset_t,
                   gamedironly: qboolean) -> *mut byte;
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
    static mut vec3_origin: vec3_t;
    #[no_mangle]
    static m_bytenormals: [[libc::c_float; 3]; 162];
    #[no_mangle]
    fn MSG_ReadByte(sb: *mut sizebuf_t) -> libc::c_int;
    #[no_mangle]
    fn MSG_ReadShort(sb: *mut sizebuf_t) -> libc::c_int;
    #[no_mangle]
    fn MSG_ReadCoord(sb: *mut sizebuf_t) -> libc::c_float;
    #[link_name = "ref"]
    static mut ref_0: ref_state_s;
    #[no_mangle]
    static mut refState: ref_globals_t;
    #[no_mangle]
    static mut cl: client_t;
    #[no_mangle]
    static mut cls: client_static_t;
    #[no_mangle]
    static mut clgame: clgame_static_t;
    #[no_mangle]
    static mut cl_draw_particles: *mut convar_t;
    #[no_mangle]
    static mut cl_draw_tracers: *mut convar_t;
    #[no_mangle]
    static mut cl_draw_beams: *mut convar_t;
    #[no_mangle]
    fn CL_ModelHandle(modelindex: libc::c_int) -> *mut model_t;
    #[no_mangle]
    fn CL_LoadModel(modelname: *const libc::c_char, index: *mut libc::c_int)
     -> *mut model_t;
    #[no_mangle]
    fn R_TempSprite(pos: *mut vec_t, dir: *const vec_t, scale: libc::c_float,
                    modelIndex: libc::c_int, rendermode: libc::c_int,
                    renderfx: libc::c_int, a: libc::c_float,
                    life: libc::c_float, flags: libc::c_int)
     -> *mut tempent_s;
    #[no_mangle]
    fn PM_ParticleLine(start: *const vec_t, end: *const vec_t,
                       pcolor: libc::c_int, life: libc::c_float,
                       zvel: libc::c_float);
    #[no_mangle]
    fn PM_DrawBBox(mins: *const vec_t, maxs: *const vec_t,
                   origin: *const vec_t, pcolor: libc::c_int,
                   life: libc::c_float);
}
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type off_t = __off_t;
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
pub type fs_offset_t = off_t;
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
pub type cmdalias_t = cmdalias_s;
pub type HSPRITE = libc::c_int;
pub type pfnUserMsgHook
    =
    Option<unsafe extern "C" fn(_: *const libc::c_char, _: libc::c_int,
                                _: *mut libc::c_void) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wrect_s {
    pub left: libc::c_int,
    pub right: libc::c_int,
    pub top: libc::c_int,
    pub bottom: libc::c_int,
}
pub type wrect_t = wrect_s;
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
pub type SCREENINFO = SCREENINFO_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client_data_s {
    pub origin: vec3_t,
    pub viewangles: vec3_t,
    pub iWeaponBits: libc::c_int,
    pub fov: libc::c_float,
}
pub type client_data_t = client_data_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client_sprite_s {
    pub szName: [libc::c_char; 64],
    pub szSprite: [libc::c_char; 64],
    pub hspr: libc::c_int,
    pub iRes: libc::c_int,
    pub rc: wrect_t,
}
pub type client_sprite_t = client_sprite_s;
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
pub type client_textmessage_t = client_textmessage_s;
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
pub type hud_player_info_t = hud_player_info_s;
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
pub type cl_entity_t = cl_entity_s;
pub type TEMPENTITY = tempent_s;
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
pub type cl_enginefunc_t = cl_enginefuncs_s;
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
pub type efrag_t = efrag_s;
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
pub type decallist_t = decallist_s;
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
pub type render_api_t = render_api_s;
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
pub type render_interface_t = render_interface_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mstudioevent_s {
    pub frame: int32_t,
    pub event: int32_t,
    pub unused: int32_t,
    pub options: [libc::c_char; 64],
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
pub struct remap_info_s {
    pub textures: [libc::c_ushort; 32],
    pub ptexture: *mut mstudiotex_s,
    pub numtextures: libc::c_short,
    pub topcolor: libc::c_short,
    pub bottomcolor: libc::c_short,
    pub model: *mut model_t,
}
pub type remap_info_t = remap_info_s;
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
pub type playermove_t = playermove_s;
pub type movevars_t = movevars_s;
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
pub type ref_params_t = ref_params_s;
pub type ref_overview_t = ref_overview_s;
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
pub type cldll_func_t = cldll_func_s;
pub type screenfade_t = screenfade_s;
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
pub type net_response_t = net_response_s;
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
pub struct cl_user_message_t {
    pub name: [libc::c_char; 32],
    pub number: libc::c_int,
    pub size: libc::c_int,
    pub func: pfnUserMsgHook,
}
pub type pfnEventHook
    =
    Option<unsafe extern "C" fn(_: *mut event_args_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cl_user_event_t {
    pub name: [libc::c_char; 64],
    pub index: word,
    pub func: pfnEventHook,
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
pub struct cached_spritelist_t {
    pub szListName: [libc::c_char; 64],
    pub pList: *mut client_sprite_t,
    pub count: libc::c_int,
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
pub type net_request_type_t = libc::c_uint;
pub const NET_REQUEST_CLIENT: net_request_type_t = 2;
pub const NET_REQUEST_GAMEUI: net_request_type_t = 1;
pub const NET_REQUEST_CANCEL: net_request_type_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct net_request_t {
    pub resp: net_response_t,
    pub pfnFunc: net_api_response_func_t,
    pub timeout: libc::c_double,
    pub timesend: libc::c_double,
    pub flags: libc::c_int,
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
pub const BEAM_POINTS: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const BEAM_HOSE: C2RustUnnamed_1 = 3;
pub const BEAM_ENTS: C2RustUnnamed_1 = 2;
pub const BEAM_ENTPOINT: C2RustUnnamed_1 = 1;
#[inline(always)]
unsafe extern "C" fn __tg_sqrt(mut __x: libc::c_float) -> libc::c_float {
    return sqrtf(__x);
}
/*
==============================================================

PARTICLES MANAGEMENT

==============================================================
*/
// particle ramps
static mut ramp1: [libc::c_int; 8] =
    [0x6f as libc::c_int, 0x6d as libc::c_int, 0x6b as libc::c_int,
     0x69 as libc::c_int, 0x67 as libc::c_int, 0x65 as libc::c_int,
     0x63 as libc::c_int, 0x61 as libc::c_int];
static mut ramp2: [libc::c_int; 8] =
    [0x6f as libc::c_int, 0x6e as libc::c_int, 0x6d as libc::c_int,
     0x6c as libc::c_int, 0x6b as libc::c_int, 0x6a as libc::c_int,
     0x68 as libc::c_int, 0x66 as libc::c_int];
static mut ramp3: [libc::c_int; 6] =
    [0x6d as libc::c_int, 0x6b as libc::c_int, 6 as libc::c_int,
     5 as libc::c_int, 4 as libc::c_int, 3 as libc::c_int];
static mut gSparkRamp: [libc::c_int; 9] =
    [0xfe as libc::c_int, 0xfd as libc::c_int, 0xfc as libc::c_int,
     0x6f as libc::c_int, 0x6e as libc::c_int, 0x6d as libc::c_int,
     0x6c as libc::c_int, 0x67 as libc::c_int, 0x60 as libc::c_int];
#[no_mangle]
pub static mut tracerspeed: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut tracerlength: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut traceroffset: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut cl_active_particles: *mut particle_t =
    0 as *const particle_t as *mut particle_t;
#[no_mangle]
pub static mut cl_active_tracers: *mut particle_t =
    0 as *const particle_t as *mut particle_t;
#[no_mangle]
pub static mut cl_free_particles: *mut particle_t =
    0 as *const particle_t as *mut particle_t;
#[no_mangle]
pub static mut cl_particles: *mut particle_t =
    0 as *const particle_t as *mut particle_t;
// particle pool
static mut cl_avelocities: [vec3_t; 162] = [[0.; 3]; 162];
static mut cl_lasttimewarn: libc::c_float = 0.0f32;
/*
================
R_LookupColor

find nearest color in particle palette
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_LookupColor(mut r: byte, mut g: byte, mut b: byte)
 -> libc::c_short {
    let mut i: libc::c_int = 0;
    let mut best: libc::c_int = 0;
    let mut diff: libc::c_float = 0.;
    let mut bestdiff: libc::c_float = 0.;
    let mut rf: libc::c_float = 0.;
    let mut gf: libc::c_float = 0.;
    let mut bf: libc::c_float = 0.;
    bestdiff = 999999 as libc::c_int as libc::c_float;
    best = 65535 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        rf =
            (r as libc::c_int - clgame.palette[i as usize].r as libc::c_int)
                as libc::c_float;
        gf =
            (g as libc::c_int - clgame.palette[i as usize].g as libc::c_int)
                as libc::c_float;
        bf =
            (b as libc::c_int - clgame.palette[i as usize].b as libc::c_int)
                as libc::c_float;
        // convert color to monochrome
        diff = rf * (rf * 0.2f32) + gf * (gf * 0.5f32) + bf * (bf * 0.3f32);
        if diff < bestdiff { bestdiff = diff; best = i }
        i += 1
    }
    return best as libc::c_short;
}
/*
================
R_GetPackedColor

in hardware mode does nothing
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_GetPackedColor(mut packed: *mut libc::c_short,
                                          mut color: libc::c_short) {
    if !packed.is_null() { *packed = 0 as libc::c_int as libc::c_short };
}
/*
================
CL_InitParticles

================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_InitParticles() {
    let mut i: libc::c_int = 0;
    cl_particles =
        _Mem_Alloc(cls.mempool,
                   (::std::mem::size_of::<particle_t>() as
                        libc::c_ulong).wrapping_mul((*SI.GameInfo).max_particles
                                                        as libc::c_ulong),
                   true_0,
                   b"../engine/client/cl_efx.c\x00" as *const u8 as
                       *const libc::c_char, 92 as libc::c_int) as
            *mut particle_t;
    CL_ClearParticles();
    // this is used for EF_BRIGHTFIELD
    i = 0 as libc::c_int;
    while i < 162 as libc::c_int {
        cl_avelocities[i as usize][0 as libc::c_int as usize] =
            COM_RandomFloat(0.0f32, 2.55f32);
        cl_avelocities[i as usize][1 as libc::c_int as usize] =
            COM_RandomFloat(0.0f32, 2.55f32);
        cl_avelocities[i as usize][2 as libc::c_int as usize] =
            COM_RandomFloat(0.0f32, 2.55f32);
        i += 1
    }
    tracerspeed =
        Cvar_Get(b"tracerspeed\x00" as *const u8 as *const libc::c_char,
                 b"6000\x00" as *const u8 as *const libc::c_char,
                 0 as libc::c_int,
                 b"tracer speed\x00" as *const u8 as *const libc::c_char);
    tracerlength =
        Cvar_Get(b"tracerlength\x00" as *const u8 as *const libc::c_char,
                 b"0.8\x00" as *const u8 as *const libc::c_char,
                 0 as libc::c_int,
                 b"tracer length factor\x00" as *const u8 as
                     *const libc::c_char);
    traceroffset =
        Cvar_Get(b"traceroffset\x00" as *const u8 as *const libc::c_char,
                 b"30\x00" as *const u8 as *const libc::c_char,
                 0 as libc::c_int,
                 b"tracer starting offset\x00" as *const u8 as
                     *const libc::c_char);
}
/*
================
CL_ClearParticles

================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ClearParticles() {
    let mut i: libc::c_int = 0;
    if cl_particles.is_null() { return }
    cl_free_particles = cl_particles;
    cl_active_particles = 0 as *mut particle_t;
    cl_active_tracers = 0 as *mut particle_t;
    i = 0 as libc::c_int;
    while i < (*SI.GameInfo).max_particles - 1 as libc::c_int {
        let ref mut fresh0 = (*cl_particles.offset(i as isize)).next;
        *fresh0 =
            &mut *cl_particles.offset((i + 1 as libc::c_int) as isize) as
                *mut particle_t;
        i += 1
    }
    let ref mut fresh1 =
        (*cl_particles.offset(((*SI.GameInfo).max_particles -
                                   1 as libc::c_int) as isize)).next;
    *fresh1 = 0 as *mut particle_s;
}
/*
================
CL_FreeParticles

================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_FreeParticles() {
    if !cl_particles.is_null() {
        _Mem_Free(cl_particles as *mut libc::c_void,
                  b"../engine/client/cl_efx.c\x00" as *const u8 as
                      *const libc::c_char, 139 as libc::c_int);
    }
    cl_particles = 0 as *mut particle_t;
}
/*
================
CL_FreeParticle

move particle to freelist
================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_FreeParticle(mut p: *mut particle_t) {
    if (*p).deathfunc.is_some() {
        // call right the deathfunc before die
        (*p).deathfunc.expect("non-null function pointer")(p);
        (*p).deathfunc = None
    }
    (*p).next = cl_free_particles;
    cl_free_particles = p;
}
/*
================
CL_AllocParticleFast

unconditionally give new particle pointer from cl_free_particles
================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_AllocParticleFast() -> *mut particle_s {
    let mut p: *mut particle_t = 0 as *mut particle_t;
    if !cl_free_particles.is_null() {
        p = cl_free_particles;
        cl_free_particles = (*p).next
    }
    return p;
}
/*
================
R_AllocParticle

can return NULL if particles is out
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_AllocParticle(mut callback:
                                             Option<unsafe extern "C" fn(_:
                                                                             *mut particle_t,
                                                                         _:
                                                                             libc::c_float)
                                                        -> ()>)
 -> *mut particle_s {
    let mut p: *mut particle_t = 0 as *mut particle_t;
    if (*cl_draw_particles).value == 0. { return 0 as *mut particle_s }
    // never alloc particles when we not in game
//	if( tr.frametime == 0.0 ) return NULL;
    if cl_free_particles.is_null() {
        if (cl_lasttimewarn as libc::c_double) < host.realtime {
            // don't spam about overflow
            Con_DPrintf(b"^1Error:^7 Overflow %d particles\n\x00" as *const u8
                            as *const libc::c_char,
                        (*SI.GameInfo).max_particles);
            cl_lasttimewarn =
                (host.realtime + 1.0f32 as libc::c_double) as libc::c_float
        }
        return 0 as *mut particle_s
    }
    p = cl_free_particles;
    cl_free_particles = (*p).next;
    (*p).next = cl_active_particles;
    cl_active_particles = p;
    // clear old particle
    (*p).type_0 = pt_static;
    (*p).vel[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    (*p).vel[1 as libc::c_int as usize] = (*p).vel[2 as libc::c_int as usize];
    (*p).vel[0 as libc::c_int as usize] = (*p).vel[1 as libc::c_int as usize];
    (*p).org[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    (*p).org[1 as libc::c_int as usize] = (*p).org[2 as libc::c_int as usize];
    (*p).org[0 as libc::c_int as usize] = (*p).org[1 as libc::c_int as usize];
    (*p).packedColor = 0 as libc::c_int as libc::c_short;
    (*p).die = cl.time as libc::c_float;
    (*p).color = 0 as libc::c_int as libc::c_short;
    (*p).ramp = 0 as libc::c_int as libc::c_float;
    if callback.is_some() {
        (*p).type_0 = pt_clientcustom;
        (*p).callback = callback
    }
    return p;
}
/*
================
R_AllocTracer

can return NULL if particles is out
================
*/
#[no_mangle]
pub unsafe extern "C" fn R_AllocTracer(mut org: *const vec_t,
                                       mut vel: *const vec_t,
                                       mut life: libc::c_float)
 -> *mut particle_t {
    let mut p: *mut particle_t = 0 as *mut particle_t;
    if (*cl_draw_tracers).value == 0. { return 0 as *mut particle_t }
    // never alloc particles when we not in game
	//if( tr.frametime == 0.0 ) return NULL;
    if cl_free_particles.is_null() {
        if (cl_lasttimewarn as libc::c_double) < host.realtime {
            // don't spam about overflow
            Con_DPrintf(b"^1Error:^7 Overflow %d tracers\n\x00" as *const u8
                            as *const libc::c_char,
                        (*SI.GameInfo).max_particles);
            cl_lasttimewarn =
                (host.realtime + 1.0f32 as libc::c_double) as libc::c_float
        }
        return 0 as *mut particle_t
    }
    p = cl_free_particles;
    cl_free_particles = (*p).next;
    (*p).next = cl_active_tracers;
    cl_active_tracers = p;
    // clear old particle
    (*p).type_0 = pt_static; // select custom color
    (*p).org[0 as libc::c_int as usize] =
        *org.offset(0 as libc::c_int as isize); // alpha
    (*p).org[1 as libc::c_int as usize] =
        *org.offset(1 as libc::c_int as isize);
    (*p).org[2 as libc::c_int as usize] =
        *org.offset(2 as libc::c_int as isize);
    (*p).vel[0 as libc::c_int as usize] =
        *vel.offset(0 as libc::c_int as isize);
    (*p).vel[1 as libc::c_int as usize] =
        *vel.offset(1 as libc::c_int as isize);
    (*p).vel[2 as libc::c_int as usize] =
        *vel.offset(2 as libc::c_int as isize);
    (*p).die = (cl.time + life as libc::c_double) as libc::c_float;
    (*p).ramp = (*tracerlength).value;
    (*p).color = 4 as libc::c_int as libc::c_short;
    (*p).packedColor = 255 as libc::c_int as libc::c_short;
    return p;
}
/*
==============================================================

VIEWBEAMS MANAGEMENT

==============================================================
*/
#[no_mangle]
pub static mut cl_active_beams: *mut BEAM = 0 as *const BEAM as *mut BEAM;
#[no_mangle]
pub static mut cl_free_beams: *mut BEAM = 0 as *const BEAM as *mut BEAM;
#[no_mangle]
pub static mut cl_viewbeams: *mut BEAM = 0 as *const BEAM as *mut BEAM;
// beams pool
/*
==============================================================

BEAM ALLOCATE & PROCESSING

==============================================================
*/
/*
==============
R_BeamSetAttributes

set beam attributes
==============
*/
unsafe extern "C" fn R_BeamSetAttributes(mut pbeam: *mut BEAM,
                                         mut r: libc::c_float,
                                         mut g: libc::c_float,
                                         mut b: libc::c_float,
                                         mut framerate: libc::c_float,
                                         mut startFrame: libc::c_int) {
    (*pbeam).frame = startFrame as libc::c_float;
    (*pbeam).frameRate = framerate;
    (*pbeam).r = r;
    (*pbeam).g = g;
    (*pbeam).b = b;
}
/*
==============
R_BeamAlloc

==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_BeamAlloc() -> *mut BEAM {
    let mut pBeam: *mut BEAM = 0 as *mut BEAM;
    if cl_free_beams.is_null() { return 0 as *mut BEAM }
    pBeam = cl_free_beams;
    cl_free_beams = (*pBeam).next;
    memset(pBeam as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<BEAM>() as libc::c_ulong);
    (*pBeam).next = cl_active_beams;
    cl_active_beams = pBeam;
    (*pBeam).die = cl.time as libc::c_float;
    return pBeam;
}
/*
==============
R_BeamFree

==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_BeamFree(mut pBeam: *mut BEAM) {
    // free particles that have died off.
    R_FreeDeadParticles(&mut (*pBeam).particles);
    // now link into free list;
    (*pBeam).next = cl_free_beams;
    cl_free_beams = pBeam;
}
/*
================
CL_InitViewBeams

================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_InitViewBeams() {
    cl_viewbeams =
        _Mem_Alloc(cls.mempool,
                   (::std::mem::size_of::<BEAM>() as
                        libc::c_ulong).wrapping_mul((*SI.GameInfo).max_beams
                                                        as libc::c_ulong),
                   true_0,
                   b"../engine/client/cl_efx.c\x00" as *const u8 as
                       *const libc::c_char, 365 as libc::c_int) as *mut BEAM;
    CL_ClearViewBeams();
}
/*
================
CL_ClearViewBeams

================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ClearViewBeams() {
    let mut i: libc::c_int = 0;
    if cl_viewbeams.is_null() { return }
    // clear beams
    cl_free_beams = cl_viewbeams;
    cl_active_beams = 0 as *mut BEAM;
    i = 0 as libc::c_int;
    while i < (*SI.GameInfo).max_beams - 1 as libc::c_int {
        let ref mut fresh2 = (*cl_viewbeams.offset(i as isize)).next;
        *fresh2 =
            &mut *cl_viewbeams.offset((i + 1 as libc::c_int) as isize) as
                *mut BEAM;
        i += 1
    }
    let ref mut fresh3 =
        (*cl_viewbeams.offset(((*SI.GameInfo).max_beams - 1 as libc::c_int) as
                                  isize)).next;
    *fresh3 = 0 as *mut BEAM;
}
/*
================
CL_FreeViewBeams

================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_FreeViewBeams() {
    if !cl_viewbeams.is_null() {
        _Mem_Free(cl_viewbeams as *mut libc::c_void,
                  b"../engine/client/cl_efx.c\x00" as *const u8 as
                      *const libc::c_char, 399 as libc::c_int);
    }
    cl_viewbeams = 0 as *mut BEAM;
}
/*
==============
R_BeamGetEntity

extract entity number from index
handle user entities
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_BeamGetEntity(mut index: libc::c_int)
 -> *mut cl_entity_t {
    if index < 0 as libc::c_int {
        return clgame.dllFuncs.pfnGetUserEntity.expect("non-null function pointer")(-index
                                                                                        &
                                                                                        0xfff
                                                                                            as
                                                                                            libc::c_int)
    }
    return CL_GetEntityByIndex(index & 0xfff as libc::c_int);
}
/*
==============
CL_KillDeadBeams

==============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_KillDeadBeams(mut pDeadEntity: *mut cl_entity_t) {
    let mut pbeam: *mut BEAM =
        0 as *mut BEAM; // build a new list to replace cl_active_beams.
    let mut pnewlist: *mut BEAM = 0 as *mut BEAM; // old list.
    let mut pnext: *mut BEAM = 0 as *mut BEAM; // new list.
    let mut pHead: *mut particle_t = 0 as *mut particle_t;
    pbeam = cl_active_beams;
    pnewlist = 0 as *mut BEAM;
    while !pbeam.is_null() {
        let mut beament: *mut cl_entity_t = 0 as *mut cl_entity_t;
        pnext = (*pbeam).next;
        // link into new list.
        if R_BeamGetEntity((*pbeam).startEntity) != pDeadEntity {
            (*pbeam).next = pnewlist;
            pnewlist = pbeam;
            pbeam = pnext
        } else {
            (*pbeam).flags &= !(0x1 as libc::c_int | 0x2 as libc::c_int);
            if (*pbeam).type_0 != 22 as libc::c_int {
                // remove beam
                (*pbeam).die =
                    (cl.time - 0.1f32 as libc::c_double) as libc::c_float;
                // kill off particles
                pHead = (*pbeam).particles;
                while !pHead.is_null() {
                    (*pHead).die =
                        (cl.time - 0.1f32 as libc::c_double) as libc::c_float;
                    pHead = (*pHead).next
                }
                // free the beam
                R_BeamFree(pbeam);
            } else {
                // stay active
                (*pbeam).next = pnewlist;
                pnewlist = pbeam
            }
            pbeam = pnext
        }
    }
    // We now have a new list with the bogus stuff released.
    cl_active_beams = pnewlist;
}
/*
===============
CL_ReadLineFile_f

Optimized version of pointfile - use beams instead of particles
===============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ReadLineFile_f() {
    let mut afile: *mut byte = 0 as *mut byte;
    let mut pfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p1: vec3_t = [0.; 3];
    let mut p2: vec3_t = [0.; 3];
    let mut count: libc::c_int = 0;
    let mut modelIndex: libc::c_int = 0;
    let mut filename: [libc::c_char; 64] = [0; 64];
    let mut model: *mut model_t = 0 as *mut model_t;
    let mut token: string = [0; 256];
    Q_snprintf(filename.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
               b"maps/%s.lin\x00" as *const u8 as *const libc::c_char,
               clgame.mapname.as_mut_ptr());
    afile =
        FS_LoadFile(filename.as_mut_ptr(), 0 as *mut fs_offset_t, false_0);
    if afile.is_null() {
        Con_Printf(b"^1Error:^7 couldn\'t open %s\n\x00" as *const u8 as
                       *const libc::c_char, filename.as_mut_ptr());
        return
    }
    Con_Printf(b"Reading %s...\n\x00" as *const u8 as *const libc::c_char,
               filename.as_mut_ptr());
    count = 0 as libc::c_int;
    pfile = afile as *mut libc::c_char;
    model =
        CL_LoadModel(b"sprites/laserbeam.spr\x00" as *const u8 as
                         *const libc::c_char, &mut modelIndex);
    loop  {
        pfile =
            _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                               ::std::mem::size_of::<string>() as
                                   libc::c_ulong as libc::c_int,
                               0 as libc::c_int as libc::c_uint,
                               0 as *mut libc::c_int);
        if pfile.is_null() { break ; }
        p1[0 as libc::c_int as usize] = Q_atof(token.as_mut_ptr());
        pfile =
            _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                               ::std::mem::size_of::<string>() as
                                   libc::c_ulong as libc::c_int,
                               0 as libc::c_int as libc::c_uint,
                               0 as *mut libc::c_int);
        if pfile.is_null() { break ; }
        p1[1 as libc::c_int as usize] = Q_atof(token.as_mut_ptr());
        pfile =
            _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                               ::std::mem::size_of::<string>() as
                                   libc::c_ulong as libc::c_int,
                               0 as libc::c_int as libc::c_uint,
                               0 as *mut libc::c_int);
        if pfile.is_null() { break ; }
        p1[2 as libc::c_int as usize] = Q_atof(token.as_mut_ptr());
        pfile =
            _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                               ::std::mem::size_of::<string>() as
                                   libc::c_ulong as libc::c_int,
                               0 as libc::c_int as libc::c_uint,
                               0 as *mut libc::c_int);
        if pfile.is_null() { break ; }
        if token[0 as libc::c_int as usize] as libc::c_int != '-' as i32 {
            Con_Printf(b"^1Error:^7 %s is corrupted\n\x00" as *const u8 as
                           *const libc::c_char, filename.as_mut_ptr());
            break ;
        } else {
            pfile =
                _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                                   ::std::mem::size_of::<string>() as
                                       libc::c_ulong as libc::c_int,
                                   0 as libc::c_int as libc::c_uint,
                                   0 as *mut libc::c_int);
            if pfile.is_null() { break ; }
            p2[0 as libc::c_int as usize] = Q_atof(token.as_mut_ptr());
            pfile =
                _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                                   ::std::mem::size_of::<string>() as
                                       libc::c_ulong as libc::c_int,
                                   0 as libc::c_int as libc::c_uint,
                                   0 as *mut libc::c_int);
            if pfile.is_null() { break ; }
            p2[1 as libc::c_int as usize] = Q_atof(token.as_mut_ptr());
            pfile =
                _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                                   ::std::mem::size_of::<string>() as
                                       libc::c_ulong as libc::c_int,
                                   0 as libc::c_int as libc::c_uint,
                                   0 as *mut libc::c_int);
            if pfile.is_null() { break ; }
            p2[2 as libc::c_int as usize] = Q_atof(token.as_mut_ptr());
            count += 1;
            if !R_BeamPoints(p1.as_mut_ptr(), p2.as_mut_ptr(), modelIndex,
                             0 as libc::c_int as libc::c_float,
                             2 as libc::c_int as libc::c_float,
                             0 as libc::c_int as libc::c_float,
                             255 as libc::c_int as libc::c_float,
                             0 as libc::c_int as libc::c_float,
                             0 as libc::c_int,
                             0 as libc::c_int as libc::c_float, 255.0f32,
                             0.0f32, 0.0f32).is_null() {
                continue ;
            }
            if model.is_null() ||
                   (*model).type_0 as libc::c_int != mod_sprite as libc::c_int
               {
                Con_Printf(b"^1Error:^7 failed to load \"%s\"!\n\x00" as
                               *const u8 as *const libc::c_char,
                           b"sprites/laserbeam.spr\x00" as *const u8 as
                               *const libc::c_char);
            } else {
                Con_Printf(b"^1Error:^7 not enough free beams!\n\x00" as
                               *const u8 as *const libc::c_char);
            }
            break ;
        }
    }
    _Mem_Free(afile as *mut libc::c_void,
              b"../engine/client/cl_efx.c\x00" as *const u8 as
                  *const libc::c_char, 560 as libc::c_int);
    if count != 0 {
        Con_Printf(b"%i lines read\n\x00" as *const u8 as *const libc::c_char,
                   count);
    } else {
        Con_Printf(b"map %s has no leaks!\n\x00" as *const u8 as
                       *const libc::c_char, clgame.mapname.as_mut_ptr());
    };
}
/*
==============
R_BeamSprite

Create a beam with sprite at the end
Valve legacy
==============
*/
unsafe extern "C" fn CL_BeamSprite(mut start: *mut vec_t, mut end: *mut vec_t,
                                   mut beamIndex: libc::c_int,
                                   mut spriteIndex: libc::c_int) {
    R_BeamPoints(start, end, beamIndex, 0.01f32, 0.4f32,
                 0 as libc::c_int as libc::c_float,
                 COM_RandomFloat(0.5f32, 0.655f32), 5.0f32,
                 0.0f32 as libc::c_int, 0.0f32, 1.0f32, 0.0f32, 0.0f32);
    R_TempSprite(end, vec3_origin.as_mut_ptr() as *const vec_t, 0.1f32,
                 spriteIndex, kRenderTransAdd as libc::c_int,
                 kRenderFxNone as libc::c_int, 0.35f32, 0.01f32,
                 0.0f32 as libc::c_int);
}
/*
==============
R_BeamSetup

generic function. all beams must be
passed through this
==============
*/
unsafe extern "C" fn R_BeamSetup(mut pbeam: *mut BEAM, mut start: *mut vec_t,
                                 mut end: *mut vec_t,
                                 mut modelIndex: libc::c_int,
                                 mut life: libc::c_float,
                                 mut width: libc::c_float,
                                 mut amplitude: libc::c_float,
                                 mut brightness: libc::c_float,
                                 mut speed: libc::c_float) {
    let mut sprite: *mut model_t =
        CL_ModelHandle(modelIndex); // one per 16 pixels
    if sprite.is_null() { return } // one per 4 pixels
    (*pbeam).type_0 = BEAM_POINTS as libc::c_int;
    (*pbeam).modelIndex = modelIndex;
    (*pbeam).frame = 0 as libc::c_int as libc::c_float;
    (*pbeam).frameRate = 0 as libc::c_int as libc::c_float;
    (*pbeam).frameCount = (*sprite).numframes;
    (*pbeam).source[0 as libc::c_int as usize] =
        *start.offset(0 as libc::c_int as isize);
    (*pbeam).source[1 as libc::c_int as usize] =
        *start.offset(1 as libc::c_int as isize);
    (*pbeam).source[2 as libc::c_int as usize] =
        *start.offset(2 as libc::c_int as isize);
    (*pbeam).target[0 as libc::c_int as usize] =
        *end.offset(0 as libc::c_int as isize);
    (*pbeam).target[1 as libc::c_int as usize] =
        *end.offset(1 as libc::c_int as isize);
    (*pbeam).target[2 as libc::c_int as usize] =
        *end.offset(2 as libc::c_int as isize);
    (*pbeam).delta[0 as libc::c_int as usize] =
        *end.offset(0 as libc::c_int as isize) -
            *start.offset(0 as libc::c_int as isize);
    (*pbeam).delta[1 as libc::c_int as usize] =
        *end.offset(1 as libc::c_int as isize) -
            *start.offset(1 as libc::c_int as isize);
    (*pbeam).delta[2 as libc::c_int as usize] =
        *end.offset(2 as libc::c_int as isize) -
            *start.offset(2 as libc::c_int as isize);
    (*pbeam).freq = (speed as libc::c_double * cl.time) as libc::c_float;
    (*pbeam).die = (life as libc::c_double + cl.time) as libc::c_float;
    (*pbeam).amplitude = amplitude;
    (*pbeam).brightness = brightness;
    (*pbeam).width = width;
    (*pbeam).speed = speed;
    if amplitude >= 0.50f32 {
        (*pbeam).segments =
            (__tg_sqrt((*pbeam).delta[0 as libc::c_int as usize] *
                           (*pbeam).delta[0 as libc::c_int as usize] +
                           (*pbeam).delta[1 as libc::c_int as usize] *
                               (*pbeam).delta[1 as libc::c_int as usize] +
                           (*pbeam).delta[2 as libc::c_int as usize] *
                               (*pbeam).delta[2 as libc::c_int as usize]) *
                 0.25f32 + 3.0f32) as libc::c_int
    } else {
        (*pbeam).segments =
            (__tg_sqrt((*pbeam).delta[0 as libc::c_int as usize] *
                           (*pbeam).delta[0 as libc::c_int as usize] +
                           (*pbeam).delta[1 as libc::c_int as usize] *
                               (*pbeam).delta[1 as libc::c_int as usize] +
                           (*pbeam).delta[2 as libc::c_int as usize] *
                               (*pbeam).delta[2 as libc::c_int as usize]) *
                 0.075f32 + 3.0f32) as libc::c_int
    }
    (*pbeam).pFollowModel = 0 as *mut model_s;
    (*pbeam).flags = 0 as libc::c_int;
}
/*
==============
CL_BeamAttemptToDie

Check for expired beams
==============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_BeamAttemptToDie(mut pBeam: *mut BEAM)
 -> qboolean {
    // premanent beams never die automatically
    if (*pBeam).flags as libc::c_uint & 0x80000000 as libc::c_uint != 0 {
        return false_0
    }
    if (*pBeam).type_0 == 22 as libc::c_int && !(*pBeam).particles.is_null() {
        // wait for all trails are dead
        return false_0
    }
    // other beams
    if (*pBeam).die as libc::c_double > cl.time { return false_0 }
    return true_0;
}
/*
==============
R_BeamKill

Remove beam attached to specified entity
and all particle trails (if this is a beamfollow)
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_BeamKill(mut deadEntity: libc::c_int) {
    let mut pDeadEntity: *mut cl_entity_t = 0 as *mut cl_entity_t;
    pDeadEntity = R_BeamGetEntity(deadEntity);
    if pDeadEntity.is_null() { return }
    CL_KillDeadBeams(pDeadEntity);
}
/*
==============
CL_ParseViewBeam

handle beam messages
==============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseViewBeam(mut msg: *mut sizebuf_t,
                                          mut beamType: libc::c_int) {
    let mut start: vec3_t = [0.; 3]; // beam model
    let mut end: vec3_t = [0.; 3]; // sprite model
    let mut modelIndex: libc::c_int = 0;
    let mut startFrame: libc::c_int = 0;
    let mut frameRate: libc::c_float = 0.;
    let mut life: libc::c_float = 0.;
    let mut width: libc::c_float = 0.;
    let mut startEnt: libc::c_int = 0;
    let mut endEnt: libc::c_int = 0;
    let mut noise: libc::c_float = 0.;
    let mut speed: libc::c_float = 0.;
    let mut r: libc::c_float = 0.;
    let mut g: libc::c_float = 0.;
    let mut b: libc::c_float = 0.;
    let mut a: libc::c_float = 0.;
    match beamType {
        0 => {
            start[0 as libc::c_int as usize] = MSG_ReadCoord(msg);
            start[1 as libc::c_int as usize] = MSG_ReadCoord(msg);
            start[2 as libc::c_int as usize] = MSG_ReadCoord(msg);
            end[0 as libc::c_int as usize] = MSG_ReadCoord(msg);
            end[1 as libc::c_int as usize] = MSG_ReadCoord(msg);
            end[2 as libc::c_int as usize] = MSG_ReadCoord(msg);
            modelIndex = MSG_ReadShort(msg);
            startFrame = MSG_ReadByte(msg);
            frameRate = MSG_ReadByte(msg) as libc::c_float;
            life = MSG_ReadByte(msg) as libc::c_float * 0.1f32;
            width = MSG_ReadByte(msg) as libc::c_float * 0.1f32;
            noise = MSG_ReadByte(msg) as libc::c_float * 0.01f32;
            r = MSG_ReadByte(msg) as libc::c_float / 255.0f32;
            g = MSG_ReadByte(msg) as libc::c_float / 255.0f32;
            b = MSG_ReadByte(msg) as libc::c_float / 255.0f32;
            a = MSG_ReadByte(msg) as libc::c_float / 255.0f32;
            speed = MSG_ReadByte(msg) as libc::c_float * 0.1f32;
            R_BeamPoints(start.as_mut_ptr(), end.as_mut_ptr(), modelIndex,
                         life, width, noise, a, speed, startFrame, frameRate,
                         r, g, b);
        }
        1 => {
            startEnt = MSG_ReadShort(msg);
            end[0 as libc::c_int as usize] = MSG_ReadCoord(msg);
            end[1 as libc::c_int as usize] = MSG_ReadCoord(msg);
            end[2 as libc::c_int as usize] = MSG_ReadCoord(msg);
            modelIndex = MSG_ReadShort(msg);
            startFrame = MSG_ReadByte(msg);
            frameRate = MSG_ReadByte(msg) as libc::c_float;
            life = MSG_ReadByte(msg) as libc::c_float * 0.1f32;
            width = MSG_ReadByte(msg) as libc::c_float * 0.1f32;
            noise = MSG_ReadByte(msg) as libc::c_float * 0.01f32;
            r = MSG_ReadByte(msg) as libc::c_float / 255.0f32;
            g = MSG_ReadByte(msg) as libc::c_float / 255.0f32;
            b = MSG_ReadByte(msg) as libc::c_float / 255.0f32;
            a = MSG_ReadByte(msg) as libc::c_float / 255.0f32;
            speed = MSG_ReadByte(msg) as libc::c_float * 0.1f32;
            R_BeamEntPoint(startEnt, end.as_mut_ptr(), modelIndex, life,
                           width, noise, a, speed, startFrame, frameRate, r,
                           g, b);
        }
        7 => {
            start[0 as libc::c_int as usize] = MSG_ReadCoord(msg);
            start[1 as libc::c_int as usize] = MSG_ReadCoord(msg);
            start[2 as libc::c_int as usize] = MSG_ReadCoord(msg);
            end[0 as libc::c_int as usize] = MSG_ReadCoord(msg);
            end[1 as libc::c_int as usize] = MSG_ReadCoord(msg);
            end[2 as libc::c_int as usize] = MSG_ReadCoord(msg);
            life = MSG_ReadByte(msg) as libc::c_float * 0.1f32;
            width = MSG_ReadByte(msg) as libc::c_float * 0.1f32;
            noise = MSG_ReadByte(msg) as libc::c_float * 0.01f32;
            modelIndex = MSG_ReadShort(msg);
            R_BeamLightning(start.as_mut_ptr(), end.as_mut_ptr(), modelIndex,
                            life, width, noise, 0.6f32, 3.5f32);
        }
        8 => {
            startEnt = MSG_ReadShort(msg);
            endEnt = MSG_ReadShort(msg);
            modelIndex = MSG_ReadShort(msg);
            startFrame = MSG_ReadByte(msg);
            frameRate = MSG_ReadByte(msg) as libc::c_float * 0.1f32;
            life = MSG_ReadByte(msg) as libc::c_float * 0.1f32;
            width = MSG_ReadByte(msg) as libc::c_float * 0.1f32;
            noise = MSG_ReadByte(msg) as libc::c_float * 0.01f32;
            r = MSG_ReadByte(msg) as libc::c_float / 255.0f32;
            g = MSG_ReadByte(msg) as libc::c_float / 255.0f32;
            b = MSG_ReadByte(msg) as libc::c_float / 255.0f32;
            a = MSG_ReadByte(msg) as libc::c_float / 255.0f32;
            speed = MSG_ReadByte(msg) as libc::c_float * 0.1f32;
            R_BeamEnts(startEnt, endEnt, modelIndex, life, width, noise, a,
                       speed, startFrame, frameRate, r, g, b);
        }
        18 => {
            start[0 as libc::c_int as usize] = MSG_ReadCoord(msg);
            start[1 as libc::c_int as usize] = MSG_ReadCoord(msg);
            start[2 as libc::c_int as usize] = MSG_ReadCoord(msg);
            end[0 as libc::c_int as usize] = MSG_ReadCoord(msg);
            end[1 as libc::c_int as usize] = MSG_ReadCoord(msg);
            end[2 as libc::c_int as usize] = MSG_ReadCoord(msg);
            modelIndex = MSG_ReadShort(msg);
            startFrame = MSG_ReadShort(msg);
            CL_BeamSprite(start.as_mut_ptr(), end.as_mut_ptr(), modelIndex,
                          startFrame);
        }
        19 | 20 | 21 => {
            start[0 as libc::c_int as usize] = MSG_ReadCoord(msg);
            start[1 as libc::c_int as usize] = MSG_ReadCoord(msg);
            start[2 as libc::c_int as usize] = MSG_ReadCoord(msg);
            end[0 as libc::c_int as usize] = MSG_ReadCoord(msg);
            end[1 as libc::c_int as usize] = MSG_ReadCoord(msg);
            end[2 as libc::c_int as usize] = MSG_ReadCoord(msg);
            modelIndex = MSG_ReadShort(msg);
            startFrame = MSG_ReadByte(msg);
            frameRate = MSG_ReadByte(msg) as libc::c_float;
            life = MSG_ReadByte(msg) as libc::c_float * 0.1f32;
            width = MSG_ReadByte(msg) as libc::c_float;
            noise = MSG_ReadByte(msg) as libc::c_float * 0.1f32;
            r = MSG_ReadByte(msg) as libc::c_float / 255.0f32;
            g = MSG_ReadByte(msg) as libc::c_float / 255.0f32;
            b = MSG_ReadByte(msg) as libc::c_float / 255.0f32;
            a = MSG_ReadByte(msg) as libc::c_float / 255.0f32;
            speed = MSG_ReadByte(msg) as libc::c_float / 0.1f32;
            R_BeamCirclePoints(beamType, start.as_mut_ptr(), end.as_mut_ptr(),
                               modelIndex, life, width, noise, a, speed,
                               startFrame, frameRate, r, g, b);
        }
        22 => {
            startEnt = MSG_ReadShort(msg);
            modelIndex = MSG_ReadShort(msg);
            life = MSG_ReadByte(msg) as libc::c_float * 0.1f32;
            width = MSG_ReadByte(msg) as libc::c_float;
            r = MSG_ReadByte(msg) as libc::c_float / 255.0f32;
            g = MSG_ReadByte(msg) as libc::c_float / 255.0f32;
            b = MSG_ReadByte(msg) as libc::c_float / 255.0f32;
            a = MSG_ReadByte(msg) as libc::c_float / 255.0f32;
            R_BeamFollow(startEnt, modelIndex, life, width, r, g, b, a);
        }
        24 => {
            startEnt = MSG_ReadShort(msg);
            endEnt = MSG_ReadShort(msg);
            modelIndex = MSG_ReadShort(msg);
            startFrame = MSG_ReadByte(msg);
            frameRate = MSG_ReadByte(msg) as libc::c_float;
            life = MSG_ReadByte(msg) as libc::c_float * 0.1f32;
            width = MSG_ReadByte(msg) as libc::c_float * 0.1f32;
            noise = MSG_ReadByte(msg) as libc::c_float * 0.01f32;
            r = MSG_ReadByte(msg) as libc::c_float / 255.0f32;
            g = MSG_ReadByte(msg) as libc::c_float / 255.0f32;
            b = MSG_ReadByte(msg) as libc::c_float / 255.0f32;
            a = MSG_ReadByte(msg) as libc::c_float / 255.0f32;
            speed = MSG_ReadByte(msg) as libc::c_float * 0.1f32;
            R_BeamRing(startEnt, endEnt, modelIndex, life, width, noise, a,
                       speed, startFrame, frameRate, r, g, b);
        }
        99 => { startEnt = MSG_ReadShort(msg); R_BeamKill(startEnt); }
        16 | 26 | _ => { }
    };
}
/*
==============
R_BeamEnts

Create beam between two ents
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_BeamEnts(mut startEnt: libc::c_int,
                                    mut endEnt: libc::c_int,
                                    mut modelIndex: libc::c_int,
                                    mut life: libc::c_float,
                                    mut width: libc::c_float,
                                    mut amplitude: libc::c_float,
                                    mut brightness: libc::c_float,
                                    mut speed: libc::c_float,
                                    mut startFrame: libc::c_int,
                                    mut framerate: libc::c_float,
                                    mut r: libc::c_float,
                                    mut g: libc::c_float,
                                    mut b: libc::c_float) -> *mut beam_s {
    let mut start: *mut cl_entity_t = 0 as *mut cl_entity_t;
    let mut end: *mut cl_entity_t = 0 as *mut cl_entity_t;
    let mut pbeam: *mut BEAM = 0 as *mut BEAM;
    let mut mod_0: *mut model_t = 0 as *mut model_t;
    mod_0 = CL_ModelHandle(modelIndex);
    // need a valid model.
    if mod_0.is_null() ||
           (*mod_0).type_0 as libc::c_int != mod_sprite as libc::c_int {
        return 0 as *mut beam_s
    }
    start = R_BeamGetEntity(startEnt);
    end = R_BeamGetEntity(endEnt);
    if start.is_null() || end.is_null() { return 0 as *mut beam_s }
    // don't start temporary beams out of the PVS
    if life != 0 as libc::c_int as libc::c_float &&
           ((*start).model.is_null() || (*end).model.is_null()) {
        return 0 as *mut beam_s
    }
    pbeam =
        R_BeamLightning(vec3_origin.as_mut_ptr(), vec3_origin.as_mut_ptr(),
                        modelIndex, life, width, amplitude, brightness,
                        speed);
    if pbeam.is_null() { return 0 as *mut beam_s }
    (*pbeam).type_0 = 0 as libc::c_int;
    (*pbeam).flags =
        (*pbeam).flags | (0x1 as libc::c_int | 0x2 as libc::c_int);
    if life == 0 as libc::c_int as libc::c_float {
        (*pbeam).flags =
            ((*pbeam).flags as libc::c_uint | 0x80000000 as libc::c_uint) as
                libc::c_int
    }
    (*pbeam).startEntity = startEnt;
    (*pbeam).endEntity = endEnt;
    R_BeamSetAttributes(pbeam, r, g, b, framerate, startFrame);
    return pbeam;
}
/*
==============
R_BeamPoints

Create beam between two points
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_BeamPoints(mut start: *mut vec_t,
                                      mut end: *mut vec_t,
                                      mut modelIndex: libc::c_int,
                                      mut life: libc::c_float,
                                      mut width: libc::c_float,
                                      mut amplitude: libc::c_float,
                                      mut brightness: libc::c_float,
                                      mut speed: libc::c_float,
                                      mut startFrame: libc::c_int,
                                      mut framerate: libc::c_float,
                                      mut r: libc::c_float,
                                      mut g: libc::c_float,
                                      mut b: libc::c_float) -> *mut beam_s {
    let mut pbeam: *mut BEAM = 0 as *mut BEAM;
    if life != 0 as libc::c_int as libc::c_float &&
           ref_0.dllFuncs.R_BeamCull.expect("non-null function pointer")(start
                                                                             as
                                                                             *const vec_t,
                                                                         end
                                                                             as
                                                                             *const vec_t,
                                                                         true_0)
               as libc::c_uint != 0 {
        return 0 as *mut beam_s
    }
    pbeam = R_BeamAlloc();
    if pbeam.is_null() { return 0 as *mut beam_s }
    (*pbeam).die = cl.time as libc::c_float;
    if modelIndex < 0 as libc::c_int { return 0 as *mut beam_s }
    R_BeamSetup(pbeam, start, end, modelIndex, life, width, amplitude,
                brightness, speed);
    if life == 0 as libc::c_int as libc::c_float {
        (*pbeam).flags =
            ((*pbeam).flags as libc::c_uint | 0x80000000 as libc::c_uint) as
                libc::c_int
    }
    R_BeamSetAttributes(pbeam, r, g, b, framerate, startFrame);
    return pbeam;
}
/*
==============
R_BeamCirclePoints

Create beam cicrle
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_BeamCirclePoints(mut type_0: libc::c_int,
                                            mut start: *mut vec_t,
                                            mut end: *mut vec_t,
                                            mut modelIndex: libc::c_int,
                                            mut life: libc::c_float,
                                            mut width: libc::c_float,
                                            mut amplitude: libc::c_float,
                                            mut brightness: libc::c_float,
                                            mut speed: libc::c_float,
                                            mut startFrame: libc::c_int,
                                            mut framerate: libc::c_float,
                                            mut r: libc::c_float,
                                            mut g: libc::c_float,
                                            mut b: libc::c_float)
 -> *mut beam_s {
    let mut pbeam: *mut BEAM =
        R_BeamLightning(start, end, modelIndex, life, width, amplitude,
                        brightness, speed);
    if pbeam.is_null() { return 0 as *mut beam_s }
    (*pbeam).type_0 = type_0;
    if life == 0 as libc::c_int as libc::c_float {
        (*pbeam).flags =
            ((*pbeam).flags as libc::c_uint | 0x80000000 as libc::c_uint) as
                libc::c_int
    }
    R_BeamSetAttributes(pbeam, r, g, b, framerate, startFrame);
    return pbeam;
}
/*
==============
R_BeamEntPoint

Create beam between entity and point
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_BeamEntPoint(mut startEnt: libc::c_int,
                                        mut end: *mut vec_t,
                                        mut modelIndex: libc::c_int,
                                        mut life: libc::c_float,
                                        mut width: libc::c_float,
                                        mut amplitude: libc::c_float,
                                        mut brightness: libc::c_float,
                                        mut speed: libc::c_float,
                                        mut startFrame: libc::c_int,
                                        mut framerate: libc::c_float,
                                        mut r: libc::c_float,
                                        mut g: libc::c_float,
                                        mut b: libc::c_float) -> *mut beam_s {
    let mut pbeam: *mut BEAM = 0 as *mut BEAM;
    let mut start: *mut cl_entity_t = 0 as *mut cl_entity_t;
    start = R_BeamGetEntity(startEnt);
    if start.is_null() { return 0 as *mut beam_s }
    if life == 0 as libc::c_int as libc::c_float && (*start).model.is_null() {
        return 0 as *mut beam_s
    }
    pbeam = R_BeamAlloc();
    if pbeam.is_null() { return 0 as *mut beam_s }
    (*pbeam).die = cl.time as libc::c_float;
    if modelIndex < 0 as libc::c_int { return 0 as *mut beam_s }
    R_BeamSetup(pbeam, vec3_origin.as_mut_ptr(), end, modelIndex, life, width,
                amplitude, brightness, speed);
    (*pbeam).type_0 = 0 as libc::c_int;
    (*pbeam).flags = (*pbeam).flags | 0x1 as libc::c_int;
    if life == 0 as libc::c_int as libc::c_float {
        (*pbeam).flags =
            ((*pbeam).flags as libc::c_uint | 0x80000000 as libc::c_uint) as
                libc::c_int
    }
    (*pbeam).startEntity = startEnt;
    (*pbeam).endEntity = 0 as libc::c_int;
    R_BeamSetAttributes(pbeam, r, g, b, framerate, startFrame);
    return pbeam;
}
/*
==============
R_BeamRing

Create beam between two ents
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_BeamRing(mut startEnt: libc::c_int,
                                    mut endEnt: libc::c_int,
                                    mut modelIndex: libc::c_int,
                                    mut life: libc::c_float,
                                    mut width: libc::c_float,
                                    mut amplitude: libc::c_float,
                                    mut brightness: libc::c_float,
                                    mut speed: libc::c_float,
                                    mut startFrame: libc::c_int,
                                    mut framerate: libc::c_float,
                                    mut r: libc::c_float,
                                    mut g: libc::c_float,
                                    mut b: libc::c_float) -> *mut beam_s {
    let mut pbeam: *mut BEAM = 0 as *mut BEAM;
    let mut start: *mut cl_entity_t = 0 as *mut cl_entity_t;
    let mut end: *mut cl_entity_t = 0 as *mut cl_entity_t;
    start = R_BeamGetEntity(startEnt);
    end = R_BeamGetEntity(endEnt);
    if start.is_null() || end.is_null() { return 0 as *mut beam_s }
    if life != 0 as libc::c_int as libc::c_float &&
           ((*start).model.is_null() || (*end).model.is_null()) {
        return 0 as *mut beam_s
    }
    pbeam =
        R_BeamLightning(vec3_origin.as_mut_ptr(), vec3_origin.as_mut_ptr(),
                        modelIndex, life, width, amplitude, brightness,
                        speed);
    if pbeam.is_null() { return 0 as *mut beam_s }
    (*pbeam).type_0 = 24 as libc::c_int;
    (*pbeam).flags =
        (*pbeam).flags | (0x1 as libc::c_int | 0x2 as libc::c_int);
    if life == 0 as libc::c_int as libc::c_float {
        (*pbeam).flags =
            ((*pbeam).flags as libc::c_uint | 0x80000000 as libc::c_uint) as
                libc::c_int
    }
    (*pbeam).startEntity = startEnt;
    (*pbeam).endEntity = endEnt;
    R_BeamSetAttributes(pbeam, r, g, b, framerate, startFrame);
    return pbeam;
}
/*
==============
R_BeamFollow

Create beam following with entity
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_BeamFollow(mut startEnt: libc::c_int,
                                      mut modelIndex: libc::c_int,
                                      mut life: libc::c_float,
                                      mut width: libc::c_float,
                                      mut r: libc::c_float,
                                      mut g: libc::c_float,
                                      mut b: libc::c_float,
                                      mut brightness: libc::c_float)
 -> *mut beam_s {
    let mut pbeam: *mut BEAM = R_BeamAlloc();
    if pbeam.is_null() { return 0 as *mut beam_s }
    (*pbeam).die = cl.time as libc::c_float;
    if modelIndex < 0 as libc::c_int { return 0 as *mut beam_s }
    R_BeamSetup(pbeam, vec3_origin.as_mut_ptr(), vec3_origin.as_mut_ptr(),
                modelIndex, life, width, life, brightness, 1.0f32);
    (*pbeam).type_0 = 22 as libc::c_int;
    (*pbeam).flags = (*pbeam).flags | 0x1 as libc::c_int;
    (*pbeam).startEntity = startEnt;
    R_BeamSetAttributes(pbeam, r, g, b, 1.0f32, 0 as libc::c_int);
    return pbeam;
}
/*
==============
R_BeamLightning

template for new beams
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_BeamLightning(mut start: *mut vec_t,
                                         mut end: *mut vec_t,
                                         mut modelIndex: libc::c_int,
                                         mut life: libc::c_float,
                                         mut width: libc::c_float,
                                         mut amplitude: libc::c_float,
                                         mut brightness: libc::c_float,
                                         mut speed: libc::c_float)
 -> *mut beam_s {
    let mut pbeam: *mut BEAM = R_BeamAlloc();
    if pbeam.is_null() { return 0 as *mut beam_s }
    (*pbeam).die = cl.time as libc::c_float;
    if modelIndex < 0 as libc::c_int { return 0 as *mut beam_s }
    R_BeamSetup(pbeam, start, end, modelIndex, life, width, amplitude,
                brightness, speed);
    return pbeam;
}
/*
===============
R_EntityParticles

set EF_BRIGHTFIELD effect
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_EntityParticles(mut ent: *mut cl_entity_t) {
    let mut angle: libc::c_float = 0.; // yellow
    let mut sr: libc::c_float = 0.;
    let mut sp: libc::c_float = 0.;
    let mut sy: libc::c_float = 0.;
    let mut cr: libc::c_float = 0.;
    let mut cp: libc::c_float = 0.;
    let mut cy: libc::c_float = 0.;
    let mut forward: vec3_t = [0.; 3];
    let mut p: *mut particle_t = 0 as *mut particle_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 162 as libc::c_int {
        p = R_AllocParticle(None);
        if p.is_null() { return }
        angle =
            (cl.time *
                 cl_avelocities[i as usize][0 as libc::c_int as usize] as
                     libc::c_double) as libc::c_float;
        SinCos(angle, &mut sy, &mut cy);
        angle =
            (cl.time *
                 cl_avelocities[i as usize][1 as libc::c_int as usize] as
                     libc::c_double) as libc::c_float;
        SinCos(angle, &mut sp, &mut cp);
        angle =
            (cl.time *
                 cl_avelocities[i as usize][2 as libc::c_int as usize] as
                     libc::c_double) as libc::c_float;
        SinCos(angle, &mut sr, &mut cr);
        forward[0 as libc::c_int as usize] = cp * cy;
        forward[1 as libc::c_int as usize] = cp * sy;
        forward[2 as libc::c_int as usize] = -sp;
        (*p).die = (cl.time + 0.001f32 as libc::c_double) as libc::c_float;
        (*p).color = 111 as libc::c_int as libc::c_short;
        (*p).org[0 as libc::c_int as usize] =
            1.0f32 * (*ent).origin[0 as libc::c_int as usize] +
                64.0f32 * m_bytenormals[i as usize][0 as libc::c_int as usize]
                + 16.0f32 * forward[0 as libc::c_int as usize];
        (*p).org[1 as libc::c_int as usize] =
            1.0f32 * (*ent).origin[1 as libc::c_int as usize] +
                64.0f32 * m_bytenormals[i as usize][1 as libc::c_int as usize]
                + 16.0f32 * forward[1 as libc::c_int as usize];
        (*p).org[2 as libc::c_int as usize] =
            1.0f32 * (*ent).origin[2 as libc::c_int as usize] +
                64.0f32 * m_bytenormals[i as usize][2 as libc::c_int as usize]
                + 16.0f32 * forward[2 as libc::c_int as usize];
        i += 1
    };
}
/*
===============
R_ParticleExplosion

===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_ParticleExplosion(mut org: *const vec_t) {
    let mut p: *mut particle_t = 0 as *mut particle_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 1024 as libc::c_int {
        p = R_AllocParticle(None);
        if p.is_null() { return }
        (*p).die = (cl.time + 5.0f32 as libc::c_double) as libc::c_float;
        (*p).ramp =
            COM_RandomLong(0 as libc::c_int, 3 as libc::c_int) as
                libc::c_float;
        (*p).color = ramp1[0 as libc::c_int as usize] as libc::c_short;
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            (*p).org[j as usize] =
                *org.offset(j as isize) + COM_RandomFloat(-16.0f32, 16.0f32);
            (*p).vel[j as usize] = COM_RandomFloat(-256.0f32, 256.0f32);
            j += 1
        }
        if i & 1 as libc::c_int != 0 {
            (*p).type_0 = pt_explode
        } else { (*p).type_0 = pt_explode2 }
        i += 1
    };
}
/*
===============
R_ParticleExplosion2

===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_ParticleExplosion2(mut org: *const vec_t,
                                              mut colorStart: libc::c_int,
                                              mut colorLength: libc::c_int) {
    let mut i: libc::c_int = 0; // use old code for blob particles
    let mut j: libc::c_int = 0;
    let mut colorMod: libc::c_int = 0 as libc::c_int;
    let mut p: *mut particle_t = 0 as *mut particle_t;
    i = 0 as libc::c_int;
    while i < 512 as libc::c_int {
        p = R_AllocParticle(None);
        if p.is_null() { return }
        (*p).die = (cl.time + 0.3f32 as libc::c_double) as libc::c_float;
        (*p).color = (colorStart + colorMod % colorLength) as libc::c_short;
        (*p).packedColor = 255 as libc::c_int as libc::c_short;
        colorMod += 1;
        (*p).type_0 = pt_blob;
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            (*p).org[j as usize] =
                *org.offset(j as isize) + COM_RandomFloat(-16.0f32, 16.0f32);
            (*p).vel[j as usize] = COM_RandomFloat(-256.0f32, 256.0f32);
            j += 1
        }
        i += 1
    };
}
/*
===============
R_BlobExplosion

===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_BlobExplosion(mut org: *const vec_t) {
    let mut p: *mut particle_t =
        0 as *mut particle_t; // use old code for blob particles
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 1024 as libc::c_int {
        p = R_AllocParticle(None);
        if p.is_null() { return }
        (*p).die =
            (cl.time + COM_RandomFloat(2.0f32, 2.4f32) as libc::c_double) as
                libc::c_float;
        (*p).packedColor = 255 as libc::c_int as libc::c_short;
        if i & 1 as libc::c_int != 0 {
            (*p).type_0 = pt_blob;
            (*p).color =
                COM_RandomLong(66 as libc::c_int, 71 as libc::c_int) as
                    libc::c_short
        } else {
            (*p).type_0 = pt_blob2;
            (*p).color =
                COM_RandomLong(150 as libc::c_int, 155 as libc::c_int) as
                    libc::c_short
        }
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            (*p).org[j as usize] =
                *org.offset(j as isize) + COM_RandomFloat(-16.0f32, 16.0f32);
            (*p).vel[j as usize] = COM_RandomFloat(-256.0f32, 256.0f32);
            j += 1
        }
        i += 1
    };
}
/*
===============
ParticleEffect

PARTICLE_EFFECT on server
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_RunParticleEffect(mut org: *const vec_t,
                                             mut dir: *const vec_t,
                                             mut color: libc::c_int,
                                             mut count: libc::c_int) {
    let mut p: *mut particle_t = 0 as *mut particle_t;
    let mut i: libc::c_int = 0;
    if count == 1024 as libc::c_int {
        // rocket explosion
        R_ParticleExplosion(org);
        return
    }
    i = 0 as libc::c_int;
    while i < count {
        p = R_AllocParticle(None);
        if p.is_null() { return }
        (*p).color =
            ((color & !(7 as libc::c_int)) +
                 COM_RandomLong(0 as libc::c_int, 7 as libc::c_int)) as
                libc::c_short;
        (*p).die =
            (cl.time + COM_RandomFloat(0.1f32, 0.4f32) as libc::c_double) as
                libc::c_float;
        (*p).type_0 = pt_slowgrav;
        (*p).org[0 as libc::c_int as usize] =
            *org.offset(0 as libc::c_int as isize) +
                COM_RandomFloat(-8.0f32, 8.0f32);
        (*p).org[1 as libc::c_int as usize] =
            *org.offset(1 as libc::c_int as isize) +
                COM_RandomFloat(-8.0f32, 8.0f32);
        (*p).org[2 as libc::c_int as usize] =
            *org.offset(2 as libc::c_int as isize) +
                COM_RandomFloat(-8.0f32, 8.0f32);
        (*p).vel[0 as libc::c_int as usize] =
            *dir.offset(0 as libc::c_int as isize) * 15.0f32;
        (*p).vel[1 as libc::c_int as usize] =
            *dir.offset(1 as libc::c_int as isize) * 15.0f32;
        (*p).vel[2 as libc::c_int as usize] =
            *dir.offset(2 as libc::c_int as isize) * 15.0f32;
        i += 1
    };
}
/*
===============
R_Blood

particle spray
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_Blood(mut org: *const vec_t,
                                 mut ndir: *const vec_t,
                                 mut pcolor: libc::c_int,
                                 mut speed: libc::c_int) {
    let mut pos: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
    let mut vec: vec3_t = [0.; 3];
    let mut pspeed: libc::c_float = speed as libc::c_float * 3.0f32;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut p: *mut particle_t = 0 as *mut particle_t;
    let mut ilength: libc::c_float =
        __tg_sqrt(*ndir.offset(0 as libc::c_int as isize) *
                      *ndir.offset(0 as libc::c_int as isize) +
                      *ndir.offset(1 as libc::c_int as isize) *
                          *ndir.offset(1 as libc::c_int as isize) +
                      *ndir.offset(2 as libc::c_int as isize) *
                          *ndir.offset(2 as libc::c_int as isize));
    if ilength != 0. { ilength = 1.0f32 / ilength }
    dir[0 as libc::c_int as usize] =
        *ndir.offset(0 as libc::c_int as isize) * ilength;
    dir[1 as libc::c_int as usize] =
        *ndir.offset(1 as libc::c_int as isize) * ilength;
    dir[2 as libc::c_int as usize] =
        *ndir.offset(2 as libc::c_int as isize) * ilength;
    i = 0 as libc::c_int;
    while i < speed / 2 as libc::c_int {
        pos[0 as libc::c_int as usize] =
            *org.offset(0 as libc::c_int as isize) +
                COM_RandomFloat(-3.0f32, 3.0f32);
        pos[1 as libc::c_int as usize] =
            *org.offset(1 as libc::c_int as isize) +
                COM_RandomFloat(-3.0f32, 3.0f32);
        pos[2 as libc::c_int as usize] =
            *org.offset(2 as libc::c_int as isize) +
                COM_RandomFloat(-3.0f32, 3.0f32);
        vec[0 as libc::c_int as usize] =
            dir[0 as libc::c_int as usize] +
                COM_RandomFloat(-0.06f32, 0.06f32);
        vec[1 as libc::c_int as usize] =
            dir[1 as libc::c_int as usize] +
                COM_RandomFloat(-0.06f32, 0.06f32);
        vec[2 as libc::c_int as usize] =
            dir[2 as libc::c_int as usize] +
                COM_RandomFloat(-0.06f32, 0.06f32);
        j = 0 as libc::c_int;
        while j < 7 as libc::c_int {
            p = R_AllocParticle(None);
            if p.is_null() { return }
            (*p).die = (cl.time + 1.5f32 as libc::c_double) as libc::c_float;
            (*p).color =
                (pcolor + COM_RandomLong(0 as libc::c_int, 9 as libc::c_int))
                    as libc::c_short;
            (*p).type_0 = pt_vox_grav;
            (*p).org[0 as libc::c_int as usize] =
                pos[0 as libc::c_int as usize] +
                    COM_RandomFloat(-1.0f32, 1.0f32);
            (*p).org[1 as libc::c_int as usize] =
                pos[1 as libc::c_int as usize] +
                    COM_RandomFloat(-1.0f32, 1.0f32);
            (*p).org[2 as libc::c_int as usize] =
                pos[2 as libc::c_int as usize] +
                    COM_RandomFloat(-1.0f32, 1.0f32);
            (*p).vel[0 as libc::c_int as usize] =
                vec[0 as libc::c_int as usize] * pspeed;
            (*p).vel[1 as libc::c_int as usize] =
                vec[1 as libc::c_int as usize] * pspeed;
            (*p).vel[2 as libc::c_int as usize] =
                vec[2 as libc::c_int as usize] * pspeed;
            j += 1
        }
        i += 1
    };
}
/*
===============
R_BloodStream

particle spray 2
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_BloodStream(mut org: *const vec_t,
                                       mut dir: *const vec_t,
                                       mut pcolor: libc::c_int,
                                       mut speed: libc::c_int) {
    let mut p: *mut particle_t = 0 as *mut particle_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut arc: libc::c_float = 0.;
    let mut accel: libc::c_float = speed as libc::c_float;
    arc = 0.05f32;
    i = 0 as libc::c_int;
    while i < 100 as libc::c_int {
        p = R_AllocParticle(None);
        if p.is_null() { return }
        (*p).die = (cl.time + 2.0f32 as libc::c_double) as libc::c_float;
        (*p).type_0 = pt_vox_grav;
        (*p).color =
            (pcolor + COM_RandomLong(0 as libc::c_int, 9 as libc::c_int)) as
                libc::c_short;
        (*p).org[0 as libc::c_int as usize] =
            *org.offset(0 as libc::c_int as isize);
        (*p).org[1 as libc::c_int as usize] =
            *org.offset(1 as libc::c_int as isize);
        (*p).org[2 as libc::c_int as usize] =
            *org.offset(2 as libc::c_int as isize);
        (*p).vel[0 as libc::c_int as usize] =
            *dir.offset(0 as libc::c_int as isize);
        (*p).vel[1 as libc::c_int as usize] =
            *dir.offset(1 as libc::c_int as isize);
        (*p).vel[2 as libc::c_int as usize] =
            *dir.offset(2 as libc::c_int as isize);
        (*p).vel[2 as libc::c_int as usize] -= arc;
        arc -= 0.005f32;
        (*p).vel[0 as libc::c_int as usize] =
            (*p).vel[0 as libc::c_int as usize] * accel;
        (*p).vel[1 as libc::c_int as usize] =
            (*p).vel[1 as libc::c_int as usize] * accel;
        (*p).vel[2 as libc::c_int as usize] =
            (*p).vel[2 as libc::c_int as usize] * accel;
        accel -= 0.00001f32;
        i += 1
        // so last few will drip
    }
    arc = 0.075f32;
    i = 0 as libc::c_int;
    while i < speed / 5 as libc::c_int {
        let mut num: libc::c_float = 0.;
        p = R_AllocParticle(None);
        if p.is_null() { return }
        (*p).die = (cl.time + 3.0f32 as libc::c_double) as libc::c_float;
        (*p).color =
            (pcolor + COM_RandomLong(0 as libc::c_int, 9 as libc::c_int)) as
                libc::c_short;
        (*p).type_0 = pt_vox_slowgrav;
        (*p).org[0 as libc::c_int as usize] =
            *org.offset(0 as libc::c_int as isize);
        (*p).org[1 as libc::c_int as usize] =
            *org.offset(1 as libc::c_int as isize);
        (*p).org[2 as libc::c_int as usize] =
            *org.offset(2 as libc::c_int as isize);
        (*p).vel[0 as libc::c_int as usize] =
            *dir.offset(0 as libc::c_int as isize);
        (*p).vel[1 as libc::c_int as usize] =
            *dir.offset(1 as libc::c_int as isize);
        (*p).vel[2 as libc::c_int as usize] =
            *dir.offset(2 as libc::c_int as isize);
        (*p).vel[2 as libc::c_int as usize] -= arc;
        arc -= 0.005f32;
        num = COM_RandomFloat(0.0f32, 1.0f32);
        accel = speed as libc::c_float * num;
        num *= 1.7f32;
        (*p).vel[0 as libc::c_int as usize] =
            (*p).vel[0 as libc::c_int as usize] * num;
        (*p).vel[1 as libc::c_int as usize] =
            (*p).vel[1 as libc::c_int as usize] * num;
        (*p).vel[2 as libc::c_int as usize] =
            (*p).vel[2 as libc::c_int as usize] * num;
        (*p).vel[0 as libc::c_int as usize] =
            (*p).vel[0 as libc::c_int as usize] * accel;
        (*p).vel[1 as libc::c_int as usize] =
            (*p).vel[1 as libc::c_int as usize] * accel;
        (*p).vel[2 as libc::c_int as usize] =
            (*p).vel[2 as libc::c_int as usize] * accel;
        j = 0 as libc::c_int;
        while j < 2 as libc::c_int {
            p = R_AllocParticle(None);
            if p.is_null() { return }
            (*p).die = (cl.time + 3.0f32 as libc::c_double) as libc::c_float;
            (*p).color =
                (pcolor + COM_RandomLong(0 as libc::c_int, 9 as libc::c_int))
                    as libc::c_short;
            (*p).type_0 = pt_vox_slowgrav;
            (*p).org[0 as libc::c_int as usize] =
                *org.offset(0 as libc::c_int as isize) +
                    COM_RandomFloat(-1.0f32, 1.0f32);
            (*p).org[1 as libc::c_int as usize] =
                *org.offset(1 as libc::c_int as isize) +
                    COM_RandomFloat(-1.0f32, 1.0f32);
            (*p).org[2 as libc::c_int as usize] =
                *org.offset(2 as libc::c_int as isize) +
                    COM_RandomFloat(-1.0f32, 1.0f32);
            (*p).vel[0 as libc::c_int as usize] =
                *dir.offset(0 as libc::c_int as isize);
            (*p).vel[1 as libc::c_int as usize] =
                *dir.offset(1 as libc::c_int as isize);
            (*p).vel[2 as libc::c_int as usize] =
                *dir.offset(2 as libc::c_int as isize);
            (*p).vel[2 as libc::c_int as usize] -= arc;
            (*p).vel[0 as libc::c_int as usize] =
                (*p).vel[0 as libc::c_int as usize] * num;
            (*p).vel[1 as libc::c_int as usize] =
                (*p).vel[1 as libc::c_int as usize] * num;
            (*p).vel[2 as libc::c_int as usize] =
                (*p).vel[2 as libc::c_int as usize] * num;
            (*p).vel[0 as libc::c_int as usize] =
                (*p).vel[0 as libc::c_int as usize] * accel;
            (*p).vel[1 as libc::c_int as usize] =
                (*p).vel[1 as libc::c_int as usize] * accel;
            (*p).vel[2 as libc::c_int as usize] =
                (*p).vel[2 as libc::c_int as usize] * accel;
            j += 1
        }
        i += 1
    };
}
/*
===============
R_LavaSplash

===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_LavaSplash(mut org: *const vec_t) {
    let mut p: *mut particle_t = 0 as *mut particle_t;
    let mut vel: libc::c_float = 0.;
    let mut dir: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    i = -(16 as libc::c_int);
    while i < 16 as libc::c_int {
        j = -(16 as libc::c_int);
        while j < 16 as libc::c_int {
            k = 0 as libc::c_int;
            while k < 1 as libc::c_int {
                p = R_AllocParticle(None);
                if p.is_null() { return }
                (*p).die =
                    (cl.time +
                         COM_RandomFloat(2.0f32, 2.62f32) as libc::c_double)
                        as libc::c_float;
                (*p).color =
                    COM_RandomLong(224 as libc::c_int, 231 as libc::c_int) as
                        libc::c_short;
                (*p).type_0 = pt_slowgrav;
                dir[0 as libc::c_int as usize] =
                    j as libc::c_float * 8.0f32 +
                        COM_RandomFloat(0.0f32, 7.0f32);
                dir[1 as libc::c_int as usize] =
                    i as libc::c_float * 8.0f32 +
                        COM_RandomFloat(0.0f32, 7.0f32);
                dir[2 as libc::c_int as usize] = 256.0f32;
                (*p).org[0 as libc::c_int as usize] =
                    *org.offset(0 as libc::c_int as isize) +
                        dir[0 as libc::c_int as usize];
                (*p).org[1 as libc::c_int as usize] =
                    *org.offset(1 as libc::c_int as isize) +
                        dir[1 as libc::c_int as usize];
                (*p).org[2 as libc::c_int as usize] =
                    *org.offset(2 as libc::c_int as isize) +
                        COM_RandomFloat(0.0f32, 63.0f32);
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
                vel = COM_RandomFloat(50.0f32, 113.0f32);
                (*p).vel[0 as libc::c_int as usize] =
                    dir[0 as libc::c_int as usize] * vel;
                (*p).vel[1 as libc::c_int as usize] =
                    dir[1 as libc::c_int as usize] * vel;
                (*p).vel[2 as libc::c_int as usize] =
                    dir[2 as libc::c_int as usize] * vel;
                k += 1
            }
            j += 1
        }
        i += 1
    };
}
/*
===============
R_ParticleBurst

===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_ParticleBurst(mut org: *const vec_t,
                                         mut size: libc::c_int,
                                         mut color: libc::c_int,
                                         mut life: libc::c_float) {
    let mut p: *mut particle_t = 0 as *mut particle_t;
    let mut dir: vec3_t = [0.; 3];
    let mut dest: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut dist: libc::c_float = 0.;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 32 as libc::c_int {
            p = R_AllocParticle(None);
            if p.is_null() { return }
            (*p).die =
                (cl.time + life as libc::c_double +
                     COM_RandomFloat(-0.5f32, 0.5f32) as libc::c_double) as
                    libc::c_float;
            (*p).color =
                (color + COM_RandomLong(0 as libc::c_int, 10 as libc::c_int))
                    as libc::c_short;
            (*p).ramp = 1.0f32;
            (*p).org[0 as libc::c_int as usize] =
                *org.offset(0 as libc::c_int as isize);
            (*p).org[1 as libc::c_int as usize] =
                *org.offset(1 as libc::c_int as isize);
            (*p).org[2 as libc::c_int as usize] =
                *org.offset(2 as libc::c_int as isize);
            dest[0 as libc::c_int as usize] =
                *org.offset(0 as libc::c_int as isize) +
                    COM_RandomFloat(-size as libc::c_float,
                                    size as libc::c_float);
            dest[1 as libc::c_int as usize] =
                *org.offset(1 as libc::c_int as isize) +
                    COM_RandomFloat(-size as libc::c_float,
                                    size as libc::c_float);
            dest[2 as libc::c_int as usize] =
                *org.offset(2 as libc::c_int as isize) +
                    COM_RandomFloat(-size as libc::c_float,
                                    size as libc::c_float);
            dir[0 as libc::c_int as usize] =
                dest[0 as libc::c_int as usize] -
                    (*p).org[0 as libc::c_int as usize];
            dir[1 as libc::c_int as usize] =
                dest[1 as libc::c_int as usize] -
                    (*p).org[1 as libc::c_int as usize];
            dir[2 as libc::c_int as usize] =
                dest[2 as libc::c_int as usize] -
                    (*p).org[2 as libc::c_int as usize];
            dist =
                VectorNormalizeLength2(dir.as_mut_ptr() as *const vec_t,
                                       dir.as_mut_ptr());
            (*p).vel[0 as libc::c_int as usize] =
                dir[0 as libc::c_int as usize] * (dist / life);
            (*p).vel[1 as libc::c_int as usize] =
                dir[1 as libc::c_int as usize] * (dist / life);
            (*p).vel[2 as libc::c_int as usize] =
                dir[2 as libc::c_int as usize] * (dist / life);
            j += 1
        }
        i += 1
    };
}
/*
===============
R_LargeFunnel

===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_LargeFunnel(mut org: *const vec_t,
                                       mut reverse: libc::c_int) {
    let mut p: *mut particle_t = 0 as *mut particle_t;
    let mut vel: libc::c_float = 0.;
    let mut dist: libc::c_float = 0.;
    let mut dir: vec3_t = [0.; 3];
    let mut dest: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = -(8 as libc::c_int);
    while i < 8 as libc::c_int {
        j = -(8 as libc::c_int);
        while j < 8 as libc::c_int {
            p = R_AllocParticle(None);
            if p.is_null() { return }
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
                (*p).org[0 as libc::c_int as usize] =
                    *org.offset(0 as libc::c_int as isize);
                (*p).org[1 as libc::c_int as usize] =
                    *org.offset(1 as libc::c_int as isize);
                (*p).org[2 as libc::c_int as usize] =
                    *org.offset(2 as libc::c_int as isize);
                dir[0 as libc::c_int as usize] =
                    dest[0 as libc::c_int as usize] -
                        (*p).org[0 as libc::c_int as usize];
                dir[1 as libc::c_int as usize] =
                    dest[1 as libc::c_int as usize] -
                        (*p).org[1 as libc::c_int as usize];
                dir[2 as libc::c_int as usize] =
                    dest[2 as libc::c_int as usize] -
                        (*p).org[2 as libc::c_int as usize]
            } else {
                (*p).org[0 as libc::c_int as usize] =
                    dest[0 as libc::c_int as usize];
                (*p).org[1 as libc::c_int as usize] =
                    dest[1 as libc::c_int as usize];
                (*p).org[2 as libc::c_int as usize] =
                    dest[2 as libc::c_int as usize];
                dir[0 as libc::c_int as usize] =
                    *org.offset(0 as libc::c_int as isize) -
                        (*p).org[0 as libc::c_int as usize];
                dir[1 as libc::c_int as usize] =
                    *org.offset(1 as libc::c_int as isize) -
                        (*p).org[1 as libc::c_int as usize];
                dir[2 as libc::c_int as usize] =
                    *org.offset(2 as libc::c_int as isize) -
                        (*p).org[2 as libc::c_int as usize]
            }
            vel = dest[2 as libc::c_int as usize] / 8.0f32;
            if vel < 64.0f32 { vel = 64.0f32 }
            dist =
                VectorNormalizeLength2(dir.as_mut_ptr() as *const vec_t,
                                       dir.as_mut_ptr());
            vel += COM_RandomFloat(64.0f32, 128.0f32);
            (*p).vel[0 as libc::c_int as usize] =
                dir[0 as libc::c_int as usize] * vel;
            (*p).vel[1 as libc::c_int as usize] =
                dir[1 as libc::c_int as usize] * vel;
            (*p).vel[2 as libc::c_int as usize] =
                dir[2 as libc::c_int as usize] * vel;
            (*p).die =
                (cl.time + (dist / vel) as libc::c_double) as libc::c_float;
            (*p).color = 244 as libc::c_int as libc::c_short;
            j += 1
            // green color
        }
        i += 1
    };
}
/*
===============
R_TeleportSplash

===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_TeleportSplash(mut org: *const vec_t) {
    let mut p: *mut particle_t = 0 as *mut particle_t;
    let mut dir: vec3_t = [0.; 3];
    let mut vel: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    i = -(16 as libc::c_int);
    while i < 16 as libc::c_int {
        j = -(16 as libc::c_int);
        while j < 16 as libc::c_int {
            k = -(24 as libc::c_int);
            while k < 32 as libc::c_int {
                p = R_AllocParticle(None);
                if p.is_null() { return }
                (*p).die =
                    (cl.time +
                         COM_RandomFloat(0.2f32, 0.34f32) as libc::c_double)
                        as libc::c_float;
                (*p).color =
                    COM_RandomLong(7 as libc::c_int, 14 as libc::c_int) as
                        libc::c_short;
                (*p).type_0 = pt_slowgrav;
                dir[0 as libc::c_int as usize] = j as libc::c_float * 8.0f32;
                dir[1 as libc::c_int as usize] = i as libc::c_float * 8.0f32;
                dir[2 as libc::c_int as usize] = k as libc::c_float * 8.0f32;
                (*p).org[0 as libc::c_int as usize] =
                    *org.offset(0 as libc::c_int as isize) +
                        i as libc::c_float + COM_RandomFloat(0.0f32, 3.0f32);
                (*p).org[1 as libc::c_int as usize] =
                    *org.offset(1 as libc::c_int as isize) +
                        j as libc::c_float + COM_RandomFloat(0.0f32, 3.0f32);
                (*p).org[2 as libc::c_int as usize] =
                    *org.offset(2 as libc::c_int as isize) +
                        k as libc::c_float + COM_RandomFloat(0.0f32, 3.0f32);
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
                vel = COM_RandomFloat(50.0f32, 113.0f32);
                (*p).vel[0 as libc::c_int as usize] =
                    dir[0 as libc::c_int as usize] * vel;
                (*p).vel[1 as libc::c_int as usize] =
                    dir[1 as libc::c_int as usize] * vel;
                (*p).vel[2 as libc::c_int as usize] =
                    dir[2 as libc::c_int as usize] * vel;
                k += 4 as libc::c_int
            }
            j += 4 as libc::c_int
        }
        i += 4 as libc::c_int
    };
}
/*
===============
R_RocketTrail

===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_RocketTrail(mut start: *mut vec_t,
                                       mut end: *mut vec_t,
                                       mut type_0: libc::c_int) {
    let mut vec: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    let mut up: vec3_t = [0.; 3];
    static mut tracercount: libc::c_int = 0;
    let mut s: libc::c_float = 0.;
    let mut c: libc::c_float = 0.;
    let mut x: libc::c_float = 0.;
    let mut y: libc::c_float = 0.;
    let mut len: libc::c_float = 0.;
    let mut dec: libc::c_float = 0.;
    let mut p: *mut particle_t = 0 as *mut particle_t;
    vec[0 as libc::c_int as usize] =
        *end.offset(0 as libc::c_int as isize) -
            *start.offset(0 as libc::c_int as isize);
    vec[1 as libc::c_int as usize] =
        *end.offset(1 as libc::c_int as isize) -
            *start.offset(1 as libc::c_int as isize);
    vec[2 as libc::c_int as usize] =
        *end.offset(2 as libc::c_int as isize) -
            *start.offset(2 as libc::c_int as isize);
    len =
        VectorNormalizeLength2(vec.as_mut_ptr() as *const vec_t,
                               vec.as_mut_ptr());
    if type_0 == 7 as libc::c_int {
        VectorVectors(vec.as_mut_ptr() as *const vec_t, right.as_mut_ptr(),
                      up.as_mut_ptr());
    }
    if type_0 < 128 as libc::c_int {
        dec = 3.0f32
    } else { dec = 1.0f32; type_0 -= 128 as libc::c_int }
    vec[0 as libc::c_int as usize] = vec[0 as libc::c_int as usize] * dec;
    vec[1 as libc::c_int as usize] = vec[1 as libc::c_int as usize] * dec;
    vec[2 as libc::c_int as usize] = vec[2 as libc::c_int as usize] * dec;
    while len > 0 as libc::c_int as libc::c_float {
        len -= dec;
        p = R_AllocParticle(None);
        if p.is_null() { return }
        (*p).die = (cl.time + 2.0f32 as libc::c_double) as libc::c_float;
        match type_0 {
            0 => {
                // rocket trail
                (*p).ramp =
                    COM_RandomLong(0 as libc::c_int, 3 as libc::c_int) as
                        libc::c_float;
                (*p).color =
                    ramp3[(*p).ramp as libc::c_int as usize] as libc::c_short;
                (*p).type_0 = pt_fire;
                (*p).org[0 as libc::c_int as usize] =
                    *start.offset(0 as libc::c_int as isize) +
                        COM_RandomFloat(-3.0f32, 3.0f32);
                (*p).org[1 as libc::c_int as usize] =
                    *start.offset(1 as libc::c_int as isize) +
                        COM_RandomFloat(-3.0f32, 3.0f32);
                (*p).org[2 as libc::c_int as usize] =
                    *start.offset(2 as libc::c_int as isize) +
                        COM_RandomFloat(-3.0f32, 3.0f32)
            }
            1 => {
                // smoke smoke
                (*p).ramp =
                    COM_RandomLong(2 as libc::c_int, 5 as libc::c_int) as
                        libc::c_float;
                (*p).color =
                    ramp3[(*p).ramp as libc::c_int as usize] as libc::c_short;
                (*p).type_0 = pt_fire;
                (*p).org[0 as libc::c_int as usize] =
                    *start.offset(0 as libc::c_int as isize) +
                        COM_RandomFloat(-3.0f32, 3.0f32);
                (*p).org[1 as libc::c_int as usize] =
                    *start.offset(1 as libc::c_int as isize) +
                        COM_RandomFloat(-3.0f32, 3.0f32);
                (*p).org[2 as libc::c_int as usize] =
                    *start.offset(2 as libc::c_int as isize) +
                        COM_RandomFloat(-3.0f32, 3.0f32)
            }
            2 => {
                // blood
                (*p).type_0 = pt_grav;
                (*p).color =
                    COM_RandomLong(67 as libc::c_int, 74 as libc::c_int) as
                        libc::c_short;
                (*p).org[0 as libc::c_int as usize] =
                    *start.offset(0 as libc::c_int as isize) +
                        COM_RandomFloat(-3.0f32, 3.0f32);
                (*p).org[1 as libc::c_int as usize] =
                    *start.offset(1 as libc::c_int as isize) +
                        COM_RandomFloat(-3.0f32, 3.0f32);
                (*p).org[2 as libc::c_int as usize] =
                    *start.offset(2 as libc::c_int as isize) +
                        COM_RandomFloat(-3.0f32, 3.0f32)
            }
            3 | 5 => {
                // tracer
                (*p).die =
                    (cl.time + 0.5f32 as libc::c_double) as libc::c_float;
                if type_0 == 3 as libc::c_int {
                    (*p).color =
                        (52 as libc::c_int +
                             ((tracercount & 4 as libc::c_int) <<
                                  1 as libc::c_int)) as libc::c_short
                } else {
                    (*p).color =
                        (230 as libc::c_int +
                             ((tracercount & 4 as libc::c_int) <<
                                  1 as libc::c_int)) as libc::c_short
                }
                (*p).org[0 as libc::c_int as usize] =
                    *start.offset(0 as libc::c_int as isize);
                (*p).org[1 as libc::c_int as usize] =
                    *start.offset(1 as libc::c_int as isize);
                (*p).org[2 as libc::c_int as usize] =
                    *start.offset(2 as libc::c_int as isize);
                tracercount += 1;
                if tracercount & 1 as libc::c_int != 0 {
                    (*p).vel[0 as libc::c_int as usize] =
                        30.0f32 * vec[1 as libc::c_int as usize];
                    (*p).vel[1 as libc::c_int as usize] =
                        30.0f32 * -vec[0 as libc::c_int as usize]
                } else {
                    (*p).vel[0 as libc::c_int as usize] =
                        30.0f32 * -vec[1 as libc::c_int as usize];
                    (*p).vel[1 as libc::c_int as usize] =
                        30.0f32 * vec[0 as libc::c_int as usize]
                }
            }
            4 => {
                // slight blood
                (*p).type_0 = pt_grav;
                (*p).color =
                    COM_RandomLong(67 as libc::c_int, 70 as libc::c_int) as
                        libc::c_short;
                (*p).org[0 as libc::c_int as usize] =
                    *start.offset(0 as libc::c_int as isize) +
                        COM_RandomFloat(-3.0f32, 3.0f32);
                (*p).org[1 as libc::c_int as usize] =
                    *start.offset(1 as libc::c_int as isize) +
                        COM_RandomFloat(-3.0f32, 3.0f32);
                (*p).org[2 as libc::c_int as usize] =
                    *start.offset(2 as libc::c_int as isize) +
                        COM_RandomFloat(-3.0f32, 3.0f32);
                len -= 3.0f32
            }
            6 => {
                // voor trail
                (*p).color =
                    COM_RandomLong(152 as libc::c_int, 155 as libc::c_int) as
                        libc::c_short;
                (*p).die += 0.3f32;
                (*p).org[0 as libc::c_int as usize] =
                    *start.offset(0 as libc::c_int as isize) +
                        COM_RandomFloat(-8.0f32, 8.0f32);
                (*p).org[1 as libc::c_int as usize] =
                    *start.offset(1 as libc::c_int as isize) +
                        COM_RandomFloat(-8.0f32, 8.0f32);
                (*p).org[2 as libc::c_int as usize] =
                    *start.offset(2 as libc::c_int as isize) +
                        COM_RandomFloat(-8.0f32, 8.0f32)
            }
            7 => {
                // explosion tracer
                x =
                    COM_RandomLong(0 as libc::c_int, 65535 as libc::c_int) as
                        libc::c_float;
                y =
                    COM_RandomLong(8 as libc::c_int, 16 as libc::c_int) as
                        libc::c_float;
                SinCos(x, &mut s, &mut c);
                s *= y;
                c *= y;
                (*p).org[0 as libc::c_int as usize] =
                    1.0f32 * *start.offset(0 as libc::c_int as isize) +
                        s * right[0 as libc::c_int as usize] +
                        c * up[0 as libc::c_int as usize];
                (*p).org[1 as libc::c_int as usize] =
                    1.0f32 * *start.offset(1 as libc::c_int as isize) +
                        s * right[1 as libc::c_int as usize] +
                        c * up[1 as libc::c_int as usize];
                (*p).org[2 as libc::c_int as usize] =
                    1.0f32 * *start.offset(2 as libc::c_int as isize) +
                        s * right[2 as libc::c_int as usize] +
                        c * up[2 as libc::c_int as usize];
                (*p).vel[0 as libc::c_int as usize] =
                    *start.offset(0 as libc::c_int as isize) -
                        (*p).org[0 as libc::c_int as usize];
                (*p).vel[1 as libc::c_int as usize] =
                    *start.offset(1 as libc::c_int as isize) -
                        (*p).org[1 as libc::c_int as usize];
                (*p).vel[2 as libc::c_int as usize] =
                    *start.offset(2 as libc::c_int as isize) -
                        (*p).org[2 as libc::c_int as usize];
                (*p).vel[0 as libc::c_int as usize] =
                    (*p).vel[0 as libc::c_int as usize] * 2.0f32;
                (*p).vel[1 as libc::c_int as usize] =
                    (*p).vel[1 as libc::c_int as usize] * 2.0f32;
                (*p).vel[2 as libc::c_int as usize] =
                    (*p).vel[2 as libc::c_int as usize] * 2.0f32;
                (*p).vel[0 as libc::c_int as usize] =
                    (*p).vel[0 as libc::c_int as usize] +
                        COM_RandomFloat(96.0f32, 111.0f32) *
                            vec[0 as libc::c_int as usize];
                (*p).vel[1 as libc::c_int as usize] =
                    (*p).vel[1 as libc::c_int as usize] +
                        COM_RandomFloat(96.0f32, 111.0f32) *
                            vec[1 as libc::c_int as usize];
                (*p).vel[2 as libc::c_int as usize] =
                    (*p).vel[2 as libc::c_int as usize] +
                        COM_RandomFloat(96.0f32, 111.0f32) *
                            vec[2 as libc::c_int as usize];
                (*p).ramp =
                    COM_RandomLong(0 as libc::c_int, 3 as libc::c_int) as
                        libc::c_float;
                (*p).color =
                    ramp3[(*p).ramp as libc::c_int as usize] as libc::c_short;
                (*p).type_0 = pt_explode2
            }
            _ => {
                // just build line to show error
                (*p).org[0 as libc::c_int as usize] =
                    *start.offset(0 as libc::c_int as isize);
                (*p).org[1 as libc::c_int as usize] =
                    *start.offset(1 as libc::c_int as isize);
                (*p).org[2 as libc::c_int as usize] =
                    *start.offset(2 as libc::c_int as isize)
            }
        }
        *start.offset(0 as libc::c_int as isize) =
            *start.offset(0 as libc::c_int as isize) +
                vec[0 as libc::c_int as usize];
        *start.offset(1 as libc::c_int as isize) =
            *start.offset(1 as libc::c_int as isize) +
                vec[1 as libc::c_int as usize];
        *start.offset(2 as libc::c_int as isize) =
            *start.offset(2 as libc::c_int as isize) +
                vec[2 as libc::c_int as usize]
    };
}
/*
================
R_ParticleLine

================
*/
#[no_mangle]
pub unsafe extern "C" fn R_ParticleLine(mut start: *const vec_t,
                                        mut end: *const vec_t, mut r: byte,
                                        mut g: byte, mut b: byte,
                                        mut life: libc::c_float) {
    let mut pcolor: libc::c_int = 0;
    pcolor = R_LookupColor(r, g, b) as libc::c_int;
    PM_ParticleLine(start, end, pcolor, life,
                    0 as libc::c_int as libc::c_float);
}
/*
================
R_ParticleBox

================
*/
#[no_mangle]
pub unsafe extern "C" fn R_ParticleBox(mut absmin: *const vec_t,
                                       mut absmax: *const vec_t, mut r: byte,
                                       mut g: byte, mut b: byte,
                                       mut life: libc::c_float) {
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut origin: vec3_t = [0.; 3];
    let mut pcolor: libc::c_int = 0;
    pcolor = R_LookupColor(r, g, b) as libc::c_int;
    origin[0 as libc::c_int as usize] =
        (*absmax.offset(0 as libc::c_int as isize) +
             *absmin.offset(0 as libc::c_int as isize)) * 0.5f32;
    origin[1 as libc::c_int as usize] =
        (*absmax.offset(1 as libc::c_int as isize) +
             *absmin.offset(1 as libc::c_int as isize)) * 0.5f32;
    origin[2 as libc::c_int as usize] =
        (*absmax.offset(2 as libc::c_int as isize) +
             *absmin.offset(2 as libc::c_int as isize)) * 0.5f32;
    maxs[0 as libc::c_int as usize] =
        *absmax.offset(0 as libc::c_int as isize) -
            origin[0 as libc::c_int as usize];
    maxs[1 as libc::c_int as usize] =
        *absmax.offset(1 as libc::c_int as isize) -
            origin[1 as libc::c_int as usize];
    maxs[2 as libc::c_int as usize] =
        *absmax.offset(2 as libc::c_int as isize) -
            origin[2 as libc::c_int as usize];
    mins[0 as libc::c_int as usize] =
        *absmin.offset(0 as libc::c_int as isize) -
            origin[0 as libc::c_int as usize];
    mins[1 as libc::c_int as usize] =
        *absmin.offset(1 as libc::c_int as isize) -
            origin[1 as libc::c_int as usize];
    mins[2 as libc::c_int as usize] =
        *absmin.offset(2 as libc::c_int as isize) -
            origin[2 as libc::c_int as usize];
    PM_DrawBBox(mins.as_mut_ptr() as *const vec_t,
                maxs.as_mut_ptr() as *const vec_t,
                origin.as_mut_ptr() as *const vec_t, pcolor, life);
}
/*
================
R_ShowLine

================
*/
#[no_mangle]
pub unsafe extern "C" fn R_ShowLine(mut start: *const vec_t,
                                    mut end: *const vec_t) {
    let mut dir: vec3_t = [0.; 3];
    let mut org: vec3_t = [0.; 3];
    let mut len: libc::c_float = 0.;
    let mut p: *mut particle_t = 0 as *mut particle_t;
    dir[0 as libc::c_int as usize] =
        *end.offset(0 as libc::c_int as isize) -
            *start.offset(0 as libc::c_int as isize);
    dir[1 as libc::c_int as usize] =
        *end.offset(1 as libc::c_int as isize) -
            *start.offset(1 as libc::c_int as isize);
    dir[2 as libc::c_int as usize] =
        *end.offset(2 as libc::c_int as isize) -
            *start.offset(2 as libc::c_int as isize);
    len =
        VectorNormalizeLength2(dir.as_mut_ptr() as *const vec_t,
                               dir.as_mut_ptr());
    dir[0 as libc::c_int as usize] = dir[0 as libc::c_int as usize] * 5.0f32;
    dir[1 as libc::c_int as usize] = dir[1 as libc::c_int as usize] * 5.0f32;
    dir[2 as libc::c_int as usize] = dir[2 as libc::c_int as usize] * 5.0f32;
    org[0 as libc::c_int as usize] = *start.offset(0 as libc::c_int as isize);
    org[1 as libc::c_int as usize] = *start.offset(1 as libc::c_int as isize);
    org[2 as libc::c_int as usize] = *start.offset(2 as libc::c_int as isize);
    while len > 0 as libc::c_int as libc::c_float {
        len -= 5.0f32;
        p = R_AllocParticle(None);
        if p.is_null() { return }
        (*p).die =
            (cl.time + 30 as libc::c_int as libc::c_double) as libc::c_float;
        (*p).color = 75 as libc::c_int as libc::c_short;
        (*p).org[0 as libc::c_int as usize] = org[0 as libc::c_int as usize];
        (*p).org[1 as libc::c_int as usize] = org[1 as libc::c_int as usize];
        (*p).org[2 as libc::c_int as usize] = org[2 as libc::c_int as usize];
        org[0 as libc::c_int as usize] =
            org[0 as libc::c_int as usize] + dir[0 as libc::c_int as usize];
        org[1 as libc::c_int as usize] =
            org[1 as libc::c_int as usize] + dir[1 as libc::c_int as usize];
        org[2 as libc::c_int as usize] =
            org[2 as libc::c_int as usize] + dir[2 as libc::c_int as usize]
    };
}
/*
===============
R_BulletImpactParticles

===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_BulletImpactParticles(mut pos: *const vec_t) {
    let mut i: libc::c_int = 0;
    let mut quantity: libc::c_int = 0;
    let mut color: libc::c_int = 0;
    let mut dist: libc::c_float = 0.;
    let mut dir: vec3_t = [0.; 3];
    let mut p: *mut particle_t = 0 as *mut particle_t;
    dir[0 as libc::c_int as usize] =
        *pos.offset(0 as libc::c_int as isize) -
            refState.vieworg[0 as libc::c_int as usize];
    dir[1 as libc::c_int as usize] =
        *pos.offset(1 as libc::c_int as isize) -
            refState.vieworg[1 as libc::c_int as usize];
    dir[2 as libc::c_int as usize] =
        *pos.offset(2 as libc::c_int as isize) -
            refState.vieworg[2 as libc::c_int as usize];
    dist =
        __tg_sqrt(dir[0 as libc::c_int as usize] *
                      dir[0 as libc::c_int as usize] +
                      dir[1 as libc::c_int as usize] *
                          dir[1 as libc::c_int as usize] +
                      dir[2 as libc::c_int as usize] *
                          dir[2 as libc::c_int as usize]);
    if dist > 1000.0f32 { dist = 1000.0f32 }
    quantity = ((1000.0f32 - dist) / 100.0f32) as libc::c_int;
    if quantity == 0 as libc::c_int { quantity = 1 as libc::c_int }
    color =
        3 as libc::c_int - 30 as libc::c_int * quantity / 100 as libc::c_int;
    R_SparkStreaks(pos, 2 as libc::c_int, -(200 as libc::c_int),
                   200 as libc::c_int);
    i = 0 as libc::c_int;
    while i < quantity * 4 as libc::c_int {
        p = R_AllocParticle(None);
        if p.is_null() { return }
        (*p).org[0 as libc::c_int as usize] =
            *pos.offset(0 as libc::c_int as isize);
        (*p).org[1 as libc::c_int as usize] =
            *pos.offset(1 as libc::c_int as isize);
        (*p).org[2 as libc::c_int as usize] =
            *pos.offset(2 as libc::c_int as isize);
        (*p).vel[0 as libc::c_int as usize] =
            COM_RandomFloat(-1.0f32, 1.0f32);
        (*p).vel[1 as libc::c_int as usize] =
            COM_RandomFloat(-1.0f32, 1.0f32);
        (*p).vel[2 as libc::c_int as usize] =
            COM_RandomFloat(-1.0f32, 1.0f32);
        (*p).vel[0 as libc::c_int as usize] =
            (*p).vel[0 as libc::c_int as usize] *
                COM_RandomFloat(50.0f32, 100.0f32);
        (*p).vel[1 as libc::c_int as usize] =
            (*p).vel[1 as libc::c_int as usize] *
                COM_RandomFloat(50.0f32, 100.0f32);
        (*p).vel[2 as libc::c_int as usize] =
            (*p).vel[2 as libc::c_int as usize] *
                COM_RandomFloat(50.0f32, 100.0f32);
        (*p).die = (cl.time + 0.5f64) as libc::c_float;
        (*p).color = (3 as libc::c_int - color) as libc::c_short;
        (*p).type_0 = pt_grav;
        i += 1
    };
}
/*
===============
R_FlickerParticles

===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_FlickerParticles(mut org: *const vec_t) {
    let mut p: *mut particle_t = 0 as *mut particle_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 15 as libc::c_int {
        p = R_AllocParticle(None);
        if p.is_null() { return }
        (*p).org[0 as libc::c_int as usize] =
            *org.offset(0 as libc::c_int as isize);
        (*p).org[1 as libc::c_int as usize] =
            *org.offset(1 as libc::c_int as isize);
        (*p).org[2 as libc::c_int as usize] =
            *org.offset(2 as libc::c_int as isize);
        (*p).vel[0 as libc::c_int as usize] =
            COM_RandomFloat(-32.0f32, 32.0f32);
        (*p).vel[1 as libc::c_int as usize] =
            COM_RandomFloat(-32.0f32, 32.0f32);
        (*p).vel[2 as libc::c_int as usize] =
            COM_RandomFloat(80.0f32, 143.0f32);
        (*p).die = (cl.time + 2.0f32 as libc::c_double) as libc::c_float;
        (*p).type_0 = pt_blob2;
        (*p).color = 254 as libc::c_int as libc::c_short;
        i += 1
    };
}
/*
===============
R_StreakSplash

create a splash of streaks
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_StreakSplash(mut pos: *const vec_t,
                                        mut dir: *const vec_t,
                                        mut color: libc::c_int,
                                        mut count: libc::c_int,
                                        mut speed: libc::c_float,
                                        mut velocityMin: libc::c_int,
                                        mut velocityMax: libc::c_int) {
    let mut vel: vec3_t = [0.; 3];
    let mut vel2: vec3_t = [0.; 3];
    let mut p: *mut particle_t = 0 as *mut particle_t;
    let mut i: libc::c_int = 0;
    vel[0 as libc::c_int as usize] =
        *dir.offset(0 as libc::c_int as isize) * speed;
    vel[1 as libc::c_int as usize] =
        *dir.offset(1 as libc::c_int as isize) * speed;
    vel[2 as libc::c_int as usize] =
        *dir.offset(2 as libc::c_int as isize) * speed;
    i = 0 as libc::c_int;
    while i < count {
        vel2[0 as libc::c_int as usize] =
            vel[0 as libc::c_int as usize] +
                COM_RandomFloat(velocityMin as libc::c_float,
                                velocityMax as libc::c_float);
        vel2[1 as libc::c_int as usize] =
            vel[1 as libc::c_int as usize] +
                COM_RandomFloat(velocityMin as libc::c_float,
                                velocityMax as libc::c_float);
        vel2[2 as libc::c_int as usize] =
            vel[2 as libc::c_int as usize] +
                COM_RandomFloat(velocityMin as libc::c_float,
                                velocityMax as libc::c_float);
        p =
            R_AllocTracer(pos, vel2.as_mut_ptr() as *const vec_t,
                          COM_RandomFloat(0.1f32, 0.5f32));
        if p.is_null() { return }
        (*p).type_0 = pt_grav;
        (*p).color = color as libc::c_short;
        (*p).ramp = 1.0f32;
        i += 1
    };
}
/*
===============
R_DebugParticle

just for debug purposes
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_DebugParticle(mut pos: *const vec_t, mut r: byte,
                                         mut g: byte, mut b: byte) {
    let mut p: *mut particle_t = 0 as *mut particle_t;
    p = R_AllocParticle(None);
    if p.is_null() { return }
    (*p).org[0 as libc::c_int as usize] =
        *pos.offset(0 as libc::c_int as isize);
    (*p).org[1 as libc::c_int as usize] =
        *pos.offset(1 as libc::c_int as isize);
    (*p).org[2 as libc::c_int as usize] =
        *pos.offset(2 as libc::c_int as isize);
    (*p).color = R_LookupColor(r, g, b);
    (*p).die = (cl.time + 0.01f32 as libc::c_double) as libc::c_float;
}
/*
===============
CL_Particle

pmove debugging particle
===============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_Particle(mut org: *const vec_t,
                                     mut color: libc::c_int,
                                     mut life: libc::c_float,
                                     mut zpos: libc::c_int,
                                     mut zvel: libc::c_int) {
    let mut p: *mut particle_t = 0 as *mut particle_t; // ???
    p = R_AllocParticle(None);
    if p.is_null() { return }
    if !org.is_null() {
        (*p).org[0 as libc::c_int as usize] =
            *org.offset(0 as libc::c_int as isize);
        (*p).org[1 as libc::c_int as usize] =
            *org.offset(1 as libc::c_int as isize);
        (*p).org[2 as libc::c_int as usize] =
            *org.offset(2 as libc::c_int as isize)
    }
    (*p).die = (cl.time + life as libc::c_double) as libc::c_float;
    (*p).vel[2 as libc::c_int as usize] += zvel as libc::c_float;
    (*p).color = color as libc::c_short;
}
/*
===============
R_TracerEffect

===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_TracerEffect(mut start: *const vec_t,
                                        mut end: *const vec_t) {
    let mut pos: vec3_t = [0.; 3]; // normalize
    let mut vel: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
    let mut len: libc::c_float = 0.;
    let mut speed: libc::c_float = 0.;
    let mut offset: libc::c_float = 0.;
    speed =
        if (*tracerspeed).value > 3.0f32 {
            (*tracerspeed).value
        } else { 3.0f32 };
    dir[0 as libc::c_int as usize] =
        *end.offset(0 as libc::c_int as isize) -
            *start.offset(0 as libc::c_int as isize);
    dir[1 as libc::c_int as usize] =
        *end.offset(1 as libc::c_int as isize) -
            *start.offset(1 as libc::c_int as isize);
    dir[2 as libc::c_int as usize] =
        *end.offset(2 as libc::c_int as isize) -
            *start.offset(2 as libc::c_int as isize);
    len =
        __tg_sqrt(dir[0 as libc::c_int as usize] *
                      dir[0 as libc::c_int as usize] +
                      dir[1 as libc::c_int as usize] *
                          dir[1 as libc::c_int as usize] +
                      dir[2 as libc::c_int as usize] *
                          dir[2 as libc::c_int as usize]);
    if len == 0.0f32 { return }
    dir[0 as libc::c_int as usize] =
        dir[0 as libc::c_int as usize] * (1.0f32 / len);
    dir[1 as libc::c_int as usize] =
        dir[1 as libc::c_int as usize] * (1.0f32 / len);
    dir[2 as libc::c_int as usize] =
        dir[2 as libc::c_int as usize] * (1.0f32 / len);
    offset = COM_RandomFloat(-10.0f32, 9.0f32) + (*traceroffset).value;
    vel[0 as libc::c_int as usize] = dir[0 as libc::c_int as usize] * offset;
    vel[1 as libc::c_int as usize] = dir[1 as libc::c_int as usize] * offset;
    vel[2 as libc::c_int as usize] = dir[2 as libc::c_int as usize] * offset;
    pos[0 as libc::c_int as usize] =
        *start.offset(0 as libc::c_int as isize) +
            vel[0 as libc::c_int as usize];
    pos[1 as libc::c_int as usize] =
        *start.offset(1 as libc::c_int as isize) +
            vel[1 as libc::c_int as usize];
    pos[2 as libc::c_int as usize] =
        *start.offset(2 as libc::c_int as isize) +
            vel[2 as libc::c_int as usize];
    vel[0 as libc::c_int as usize] = dir[0 as libc::c_int as usize] * speed;
    vel[1 as libc::c_int as usize] = dir[1 as libc::c_int as usize] * speed;
    vel[2 as libc::c_int as usize] = dir[2 as libc::c_int as usize] * speed;
    R_AllocTracer(pos.as_mut_ptr() as *const vec_t,
                  vel.as_mut_ptr() as *const vec_t, len / speed);
}
/*
===============
R_UserTracerParticle

===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_UserTracerParticle(mut org: *mut libc::c_float,
                                              mut vel: *mut libc::c_float,
                                              mut life: libc::c_float,
                                              mut colorIndex: libc::c_int,
                                              mut length: libc::c_float,
                                              mut deathcontext: byte,
                                              mut deathfunc:
                                                  Option<unsafe extern "C" fn(_:
                                                                                  *mut particle_t)
                                                             -> ()>) {
    let mut p: *mut particle_t = 0 as *mut particle_t;
    if colorIndex < 0 as libc::c_int { return }
    p = R_AllocTracer(org as *const vec_t, vel as *const vec_t, life);
    if !p.is_null() {
        (*p).context = deathcontext;
        (*p).deathfunc = deathfunc;
        (*p).color = colorIndex as libc::c_short;
        (*p).ramp = length
    };
}
/*
===============
R_TracerParticles

allow more customization
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_TracerParticles(mut org: *mut libc::c_float,
                                           mut vel: *mut libc::c_float,
                                           mut life: libc::c_float)
 -> *mut particle_s {
    return R_AllocTracer(org as *const vec_t, vel as *const vec_t, life);
}
/*
===============
R_SparkStreaks

create a streak tracers
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_SparkStreaks(mut pos: *const vec_t,
                                        mut count: libc::c_int,
                                        mut velocityMin: libc::c_int,
                                        mut velocityMax: libc::c_int) {
    let mut p: *mut particle_t = 0 as *mut particle_t;
    let mut vel: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < count {
        vel[0 as libc::c_int as usize] =
            COM_RandomFloat(velocityMin as libc::c_float,
                            velocityMax as libc::c_float);
        vel[1 as libc::c_int as usize] =
            COM_RandomFloat(velocityMin as libc::c_float,
                            velocityMax as libc::c_float);
        vel[2 as libc::c_int as usize] =
            COM_RandomFloat(velocityMin as libc::c_float,
                            velocityMax as libc::c_float);
        p =
            R_AllocTracer(pos, vel.as_mut_ptr() as *const vec_t,
                          COM_RandomFloat(0.1f32, 0.5f32));
        if p.is_null() { return }
        (*p).color = 5 as libc::c_int as libc::c_short;
        (*p).type_0 = pt_grav;
        (*p).ramp = 0.5f32;
        i += 1
    };
}
/*
===============
R_Implosion

make implosion tracers
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_Implosion(mut end: *const vec_t,
                                     mut radius: libc::c_float,
                                     mut count: libc::c_int,
                                     mut life: libc::c_float) {
    let mut dist: libc::c_float =
        radius / 100.0f32; // to avoid divide by zero
    let mut start: vec3_t = [0.; 3];
    let mut temp: vec3_t = [0.; 3];
    let mut vel: vec3_t = [0.; 3];
    let mut factor: libc::c_float = 0.;
    let mut p: *mut particle_t = 0 as *mut particle_t;
    let mut i: libc::c_int = 0;
    if life <= 0.0f32 { life = 0.1f32 }
    factor = (-1.0f64 / life as libc::c_double) as libc::c_float;
    i = 0 as libc::c_int;
    while i < count {
        temp[0 as libc::c_int as usize] =
            dist * COM_RandomFloat(-100.0f32, 100.0f32);
        temp[1 as libc::c_int as usize] =
            dist * COM_RandomFloat(-100.0f32, 100.0f32);
        temp[2 as libc::c_int as usize] =
            dist * COM_RandomFloat(0.0f32, 100.0f32);
        vel[0 as libc::c_int as usize] =
            temp[0 as libc::c_int as usize] * factor;
        vel[1 as libc::c_int as usize] =
            temp[1 as libc::c_int as usize] * factor;
        vel[2 as libc::c_int as usize] =
            temp[2 as libc::c_int as usize] * factor;
        start[0 as libc::c_int as usize] =
            temp[0 as libc::c_int as usize] +
                *end.offset(0 as libc::c_int as isize);
        start[1 as libc::c_int as usize] =
            temp[1 as libc::c_int as usize] +
                *end.offset(1 as libc::c_int as isize);
        start[2 as libc::c_int as usize] =
            temp[2 as libc::c_int as usize] +
                *end.offset(2 as libc::c_int as isize);
        p =
            R_AllocTracer(start.as_mut_ptr() as *const vec_t,
                          vel.as_mut_ptr() as *const vec_t, life);
        if p.is_null() { return }
        (*p).type_0 = pt_explode;
        i += 1
    };
}
/*
==============
R_FreeDeadParticles

Free particles that time has expired
==============
*/
#[no_mangle]
pub unsafe extern "C" fn R_FreeDeadParticles(mut ppparticles:
                                                 *mut *mut particle_t) {
    let mut p: *mut particle_t = 0 as *mut particle_t;
    let mut kill: *mut particle_t = 0 as *mut particle_t;
    loop 
         // kill all the ones hanging direcly off the base pointer
         {
        kill = *ppparticles;
        if !(!kill.is_null() && ((*kill).die as libc::c_double) < cl.time) {
            break ;
        }
        if (*kill).deathfunc.is_some() {
            (*kill).deathfunc.expect("non-null function pointer")(kill);
        }
        (*kill).deathfunc = None;
        *ppparticles = (*kill).next;
        (*kill).next = cl_free_particles;
        cl_free_particles = kill
    }
    // kill off all the others
    p = *ppparticles;
    while !p.is_null() {
        loop  {
            kill = (*p).next;
            if !(!kill.is_null() && ((*kill).die as libc::c_double) < cl.time)
               {
                break ;
            }
            if (*kill).deathfunc.is_some() {
                (*kill).deathfunc.expect("non-null function pointer")(kill);
            }
            (*kill).deathfunc = None;
            (*p).next = (*kill).next;
            (*kill).next = cl_free_particles;
            cl_free_particles = kill
        }
        p = (*p).next
    };
}
/*
===============
CL_ReadPointFile_f

===============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ReadPointFile_f() {
    let mut afile: *mut byte = 0 as *mut byte;
    let mut pfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut org: vec3_t = [0.; 3];
    let mut count: libc::c_int = 0;
    let mut p: *mut particle_t = 0 as *mut particle_t;
    let mut filename: [libc::c_char; 64] = [0; 64];
    let mut token: string = [0; 256];
    Q_snprintf(filename.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
               b"maps/%s.pts\x00" as *const u8 as *const libc::c_char,
               clgame.mapname.as_mut_ptr());
    afile =
        FS_LoadFile(filename.as_mut_ptr(), 0 as *mut fs_offset_t, false_0);
    if afile.is_null() {
        Con_Printf(b"^1Error:^7 couldn\'t open %s\n\x00" as *const u8 as
                       *const libc::c_char, filename.as_mut_ptr());
        return
    }
    Con_Printf(b"Reading %s...\n\x00" as *const u8 as *const libc::c_char,
               filename.as_mut_ptr());
    count = 0 as libc::c_int;
    pfile = afile as *mut libc::c_char;
    loop  {
        pfile =
            _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                               ::std::mem::size_of::<string>() as
                                   libc::c_ulong as libc::c_int,
                               0 as libc::c_int as libc::c_uint,
                               0 as *mut libc::c_int);
        if pfile.is_null() { break ; }
        org[0 as libc::c_int as usize] = Q_atof(token.as_mut_ptr());
        pfile =
            _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                               ::std::mem::size_of::<string>() as
                                   libc::c_ulong as libc::c_int,
                               0 as libc::c_int as libc::c_uint,
                               0 as *mut libc::c_int);
        if pfile.is_null() { break ; }
        org[1 as libc::c_int as usize] = Q_atof(token.as_mut_ptr());
        pfile =
            _COM_ParseFileSafe(pfile, token.as_mut_ptr(),
                               ::std::mem::size_of::<string>() as
                                   libc::c_ulong as libc::c_int,
                               0 as libc::c_int as libc::c_uint,
                               0 as *mut libc::c_int);
        if pfile.is_null() { break ; }
        org[2 as libc::c_int as usize] = Q_atof(token.as_mut_ptr());
        count += 1;
        if cl_free_particles.is_null() {
            Con_Printf(b"^1Error:^7 not enough free particles!\n\x00" as
                           *const u8 as *const libc::c_char);
            break ;
        } else {
            // NOTE: can't use R_AllocParticle because this command
		// may be executed from the console, while frametime is 0
            p = cl_free_particles;
            cl_free_particles = (*p).next;
            (*p).next = cl_active_particles;
            cl_active_particles = p;
            (*p).ramp = 0 as libc::c_int as libc::c_float;
            (*p).type_0 = pt_static;
            (*p).die =
                (cl.time + 99999 as libc::c_int as libc::c_double) as
                    libc::c_float;
            (*p).color = (-count & 15 as libc::c_int) as libc::c_short;
            (*p).org[0 as libc::c_int as usize] =
                org[0 as libc::c_int as usize];
            (*p).org[1 as libc::c_int as usize] =
                org[1 as libc::c_int as usize];
            (*p).org[2 as libc::c_int as usize] =
                org[2 as libc::c_int as usize];
            (*p).vel[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            (*p).vel[1 as libc::c_int as usize] =
                (*p).vel[2 as libc::c_int as usize];
            (*p).vel[0 as libc::c_int as usize] =
                (*p).vel[1 as libc::c_int as usize]
        }
    }
    _Mem_Free(afile as *mut libc::c_void,
              b"../engine/client/cl_efx.c\x00" as *const u8 as
                  *const libc::c_char, 2058 as libc::c_int);
    if count != 0 {
        Con_Printf(b"%i points read\n\x00" as *const u8 as
                       *const libc::c_char, count);
    } else {
        Con_Printf(b"map %s has no leaks!\n\x00" as *const u8 as
                       *const libc::c_char, clgame.mapname.as_mut_ptr());
    };
}
#[no_mangle]
pub unsafe extern "C" fn CL_FreeDeadBeams() {
    let mut pBeam: *mut BEAM = 0 as *mut BEAM;
    let mut pNext: *mut BEAM = 0 as *mut BEAM;
    let mut pPrev: *mut BEAM = 0 as *mut BEAM;
    // draw temporary entity beams
    pBeam = cl_active_beams;
    while !pBeam.is_null() {
        // need to store the next one since we may delete this one
        pNext = (*pBeam).next;
        // retire old beams
        if CL_BeamAttemptToDie(pBeam) as u64 != 0 {
            // reset links
            if !pPrev.is_null() {
                (*pPrev).next = pNext
            } else { cl_active_beams = pNext }
            // free the beam
            R_BeamFree(pBeam);
            pBeam = 0 as *mut BEAM
        } else { pPrev = pBeam }
        pBeam = pNext
    };
}
#[no_mangle]
pub unsafe extern "C" fn CL_DrawEFX(mut time: libc::c_float,
                                    mut fTrans: qboolean) {
    CL_FreeDeadBeams();
    if if !cl_draw_beams.is_null() && (*cl_draw_beams).value != 0.0f32 {
           true_0 as libc::c_int
       } else { false_0 as libc::c_int } != 0 {
        ref_0.dllFuncs.CL_DrawBeams.expect("non-null function pointer")(fTrans
                                                                            as
                                                                            libc::c_int,
                                                                        cl_active_beams);
    }
    if fTrans as u64 != 0 {
        R_FreeDeadParticles(&mut cl_active_particles);
        if if !cl_draw_particles.is_null() &&
                  (*cl_draw_particles).value != 0.0f32 {
               true_0 as libc::c_int
           } else { false_0 as libc::c_int } != 0 {
            ref_0.dllFuncs.CL_DrawParticles.expect("non-null function pointer")(time
                                                                                    as
                                                                                    libc::c_double,
                                                                                cl_active_particles,
                                                                                if 0.5f32
                                                                                       >
                                                                                       (*cl_draw_particles).value
                                                                                   {
                                                                                    0.5f32
                                                                                } else {
                                                                                    (*cl_draw_particles).value
                                                                                });
        }
        R_FreeDeadParticles(&mut cl_active_tracers);
        if if !cl_draw_tracers.is_null() && (*cl_draw_tracers).value != 0.0f32
              {
               true_0 as libc::c_int
           } else { false_0 as libc::c_int } != 0 {
            ref_0.dllFuncs.CL_DrawTracers.expect("non-null function pointer")(time
                                                                                  as
                                                                                  libc::c_double,
                                                                              cl_active_tracers);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn CL_ThinkParticle(mut frametime: libc::c_double,
                                          mut p: *mut particle_t) {
    let mut time3: libc::c_float =
        (15.0f32 as libc::c_double * frametime) as libc::c_float;
    let mut time2: libc::c_float =
        (10.0f32 as libc::c_double * frametime) as libc::c_float;
    let mut time1: libc::c_float =
        (5.0f32 as libc::c_double * frametime) as libc::c_float;
    let mut dvel: libc::c_float =
        (4.0f32 as libc::c_double * frametime) as libc::c_float;
    let mut grav: libc::c_float =
        (frametime * clgame.movevars.gravity as libc::c_double *
             0.05f32 as libc::c_double) as libc::c_float;
    if (*p).type_0 as libc::c_uint !=
           pt_clientcustom as libc::c_int as libc::c_uint {
        // update position.
        (*p).org[0 as libc::c_int as usize] =
            ((*p).org[0 as libc::c_int as usize] as libc::c_double +
                 frametime *
                     (*p).vel[0 as libc::c_int as usize] as libc::c_double) as
                vec_t;
        (*p).org[1 as libc::c_int as usize] =
            ((*p).org[1 as libc::c_int as usize] as libc::c_double +
                 frametime *
                     (*p).vel[1 as libc::c_int as usize] as libc::c_double) as
                vec_t;
        (*p).org[2 as libc::c_int as usize] =
            ((*p).org[2 as libc::c_int as usize] as libc::c_double +
                 frametime *
                     (*p).vel[2 as libc::c_int as usize] as libc::c_double) as
                vec_t
    }
    let mut current_block_40: u64;
    match (*p).type_0 as libc::c_uint {
        3 => {
            (*p).ramp += time1;
            if (*p).ramp >= 6.0f32 {
                (*p).die = -1.0f32
            } else {
                (*p).color =
                    ramp3[(*p).ramp as libc::c_int as usize] as libc::c_short
            }
            (*p).vel[2 as libc::c_int as usize] += grav;
            current_block_40 = 6476622998065200121;
        }
        4 => {
            (*p).ramp += time2;
            if (*p).ramp >= 8.0f32 {
                (*p).die = -1.0f32
            } else {
                (*p).color =
                    ramp1[(*p).ramp as libc::c_int as usize] as libc::c_short
            }
            (*p).vel[0 as libc::c_int as usize] =
                (*p).vel[0 as libc::c_int as usize] +
                    dvel * (*p).vel[0 as libc::c_int as usize];
            (*p).vel[1 as libc::c_int as usize] =
                (*p).vel[1 as libc::c_int as usize] +
                    dvel * (*p).vel[1 as libc::c_int as usize];
            (*p).vel[2 as libc::c_int as usize] =
                (*p).vel[2 as libc::c_int as usize] +
                    dvel * (*p).vel[2 as libc::c_int as usize];
            (*p).vel[2 as libc::c_int as usize] -= grav;
            current_block_40 = 6476622998065200121;
        }
        5 => {
            (*p).ramp += time3;
            if (*p).ramp >= 8.0f32 {
                (*p).die = -1.0f32
            } else {
                (*p).color =
                    ramp2[(*p).ramp as libc::c_int as usize] as libc::c_short
            }
            (*p).vel[0 as libc::c_int as usize] =
                ((*p).vel[0 as libc::c_int as usize] as libc::c_double +
                     -frametime *
                         (*p).vel[0 as libc::c_int as usize] as
                             libc::c_double) as vec_t;
            (*p).vel[1 as libc::c_int as usize] =
                ((*p).vel[1 as libc::c_int as usize] as libc::c_double +
                     -frametime *
                         (*p).vel[1 as libc::c_int as usize] as
                             libc::c_double) as vec_t;
            (*p).vel[2 as libc::c_int as usize] =
                ((*p).vel[2 as libc::c_int as usize] as libc::c_double +
                     -frametime *
                         (*p).vel[2 as libc::c_int as usize] as
                             libc::c_double) as vec_t;
            (*p).vel[2 as libc::c_int as usize] -= grav;
            current_block_40 = 6476622998065200121;
        }
        6 => {
            if (*p).packedColor as libc::c_int == 255 as libc::c_int {
                // normal blob explosion
                (*p).vel[0 as libc::c_int as usize] =
                    (*p).vel[0 as libc::c_int as usize] +
                        dvel * (*p).vel[0 as libc::c_int as usize];
                (*p).vel[1 as libc::c_int as usize] =
                    (*p).vel[1 as libc::c_int as usize] +
                        dvel * (*p).vel[1 as libc::c_int as usize];
                (*p).vel[2 as libc::c_int as usize] =
                    (*p).vel[2 as libc::c_int as usize] +
                        dvel * (*p).vel[2 as libc::c_int as usize];
                (*p).vel[2 as libc::c_int as usize] -= grav;
                current_block_40 = 6476622998065200121;
            } else { current_block_40 = 4699730441606062842; }
        }
        7 => { current_block_40 = 4699730441606062842; }
        1 => {
            (*p).vel[2 as libc::c_int as usize] -= grav * 20.0f32;
            current_block_40 = 6476622998065200121;
        }
        2 => {
            (*p).vel[2 as libc::c_int as usize] -= grav;
            current_block_40 = 6476622998065200121;
        }
        9 => {
            (*p).vel[2 as libc::c_int as usize] -= grav * 8.0f32;
            current_block_40 = 6476622998065200121;
        }
        8 => {
            (*p).vel[2 as libc::c_int as usize] -= grav * 4.0f32;
            current_block_40 = 6476622998065200121;
        }
        10 => {
            if (*p).callback.is_some() {
                (*p).callback.expect("non-null function pointer")(p,
                                                                  frametime as
                                                                      libc::c_float);
            }
            current_block_40 = 6476622998065200121;
        }
        0 | _ => { current_block_40 = 6476622998065200121; }
    }
    match current_block_40 {
        4699730441606062842 =>
        // intentionally fallthrough
        {
            if (*p).packedColor as libc::c_int == 255 as libc::c_int {
                // normal blob explosion
                (*p).vel[0 as libc::c_int as usize] -=
                    (*p).vel[0 as libc::c_int as usize] * dvel;
                (*p).vel[1 as libc::c_int as usize] -=
                    (*p).vel[1 as libc::c_int as usize] * dvel;
                (*p).vel[2 as libc::c_int as usize] -= grav
            } else {
                (*p).ramp += time2;
                if (*p).ramp >= 9.0f32 { (*p).ramp = 0.0f32 }
                (*p).color =
                    gSparkRamp[(*p).ramp as libc::c_int as usize] as
                        libc::c_short;
                (*p).vel[0 as libc::c_int as usize] =
                    ((*p).vel[0 as libc::c_int as usize] as libc::c_double +
                         -frametime * 0.5f32 as libc::c_double *
                             (*p).vel[0 as libc::c_int as usize] as
                                 libc::c_double) as vec_t;
                (*p).vel[1 as libc::c_int as usize] =
                    ((*p).vel[1 as libc::c_int as usize] as libc::c_double +
                         -frametime * 0.5f32 as libc::c_double *
                             (*p).vel[1 as libc::c_int as usize] as
                                 libc::c_double) as vec_t;
                (*p).vel[2 as libc::c_int as usize] =
                    ((*p).vel[2 as libc::c_int as usize] as libc::c_double +
                         -frametime * 0.5f32 as libc::c_double *
                             (*p).vel[2 as libc::c_int as usize] as
                                 libc::c_double) as vec_t;
                (*p).type_0 =
                    if COM_RandomLong(0 as libc::c_int, 3 as libc::c_int) != 0
                       {
                        pt_blob as libc::c_int
                    } else { pt_blob2 as libc::c_int } as ptype_t;
                (*p).vel[2 as libc::c_int as usize] -= grav * 5.0f32
            }
        }
        _ => { }
    };
}
