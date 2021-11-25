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
    fn floorf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn Q_strlen(string: *const libc::c_char) -> size_t;
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    fn Host_Error(error: *const libc::c_char, _: ...);
    #[no_mangle]
    fn AngleVectors(angles: *const vec_t, forward: *mut vec_t,
                    right: *mut vec_t, up: *mut vec_t);
    #[no_mangle]
    fn BoundsIntersect(mins1: *const vec_t, maxs1: *const vec_t,
                       mins2: *const vec_t, maxs2: *const vec_t) -> qboolean;
    #[no_mangle]
    fn SphereIntersect(vSphereCenter: *const vec_t,
                       fSphereRadiusSquared: libc::c_float,
                       vLinePt: *const vec_t, vLineDir: *const vec_t)
     -> qboolean;
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
    fn BoxOnPlaneSide(emins: *const vec_t, emaxs: *const vec_t,
                      p: *const mplane_t) -> libc::c_int;
    #[no_mangle]
    static mut vec3_origin: vec3_t;
    #[no_mangle]
    static mut world: world_static_t;
    #[no_mangle]
    fn Mod_StudioExtradata(mod_0: *mut model_t) -> *mut libc::c_void;
    #[no_mangle]
    fn Mod_SampleSizeForFace(surf: *mut msurface_t) -> libc::c_int;
    #[no_mangle]
    fn Mod_HullForStudio(m: *mut model_t, frame: libc::c_float,
                         seq: libc::c_int, ang: *mut vec_t, org: *mut vec_t,
                         size: *mut vec_t, pcnt: *mut byte, pbl: *mut byte,
                         hitboxes: *mut libc::c_int, ed: *mut edict_t)
     -> *mut hull_t;
    #[no_mangle]
    fn Mod_HitgroupForStudioHull(index: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn MSG_WriteCmdExt(sb: *mut sizebuf_t, cmd: libc::c_int, type_0: netsrc_t,
                       name: *const libc::c_char);
    #[no_mangle]
    fn MSG_WriteByte(sb: *mut sizebuf_t, val: libc::c_int);
    #[no_mangle]
    fn MSG_WriteFloat(sb: *mut sizebuf_t, val: libc::c_float);
    #[no_mangle]
    fn MSG_WriteString(sb: *mut sizebuf_t, pStr: *const libc::c_char)
     -> qboolean;
    #[no_mangle]
    fn InsertLinkBefore(l: *mut link_t, before: *mut link_t);
    #[no_mangle]
    fn RemoveLink(l: *mut link_t);
    #[no_mangle]
    fn ClearLink(l: *mut link_t);
    #[no_mangle]
    fn World_MoveBounds(start: *const vec_t, mins: *mut vec_t,
                        maxs: *mut vec_t, end: *const vec_t,
                        boxmins: *mut vec_t, boxmaxs: *mut vec_t);
    #[no_mangle]
    fn World_TransformAABB(transform: *mut [vec_t; 4], mins: *const vec_t,
                           maxs: *const vec_t, outmins: *mut vec_t,
                           outmaxs: *mut vec_t);
    #[no_mangle]
    fn World_CombineTraces(cliptrace: *mut trace_t, trace: *mut trace_t,
                           touch: *mut edict_t) -> trace_t;
    #[no_mangle]
    fn RankForContents(contents: libc::c_int) -> libc::c_int;
    #[no_mangle]
    static mut svs: server_static_t;
    #[no_mangle]
    static mut sv: server_t;
    #[no_mangle]
    static mut svgame: svgame_static_t;
    #[no_mangle]
    static mut sv_clienttrace: convar_t;
    #[no_mangle]
    static mut sv_lighting_modulate: *mut convar_t;
    #[no_mangle]
    fn SV_ModelHandle(modelindex: libc::c_int) -> *mut model_t;
    #[no_mangle]
    fn SV_CheckVelocity(ent: *mut edict_t);
    #[no_mangle]
    fn SV_CheckEdict(e: *const edict_t, file: *const libc::c_char,
                     line: libc::c_int) -> qboolean;
    #[no_mangle]
    fn SV_CopyTraceToGlobal(trace: *mut trace_t);
    #[no_mangle]
    fn SV_SetMinMaxSize(e: *mut edict_t, min: *const libc::c_float,
                        max: *const libc::c_float, relink: qboolean);
    #[no_mangle]
    fn SV_GetString(iString: string_t) -> *const libc::c_char;
    #[no_mangle]
    fn SV_ClassName(e: *const edict_t) -> *const libc::c_char;
    #[no_mangle]
    fn SV_EdictNum(n: libc::c_int) -> *mut edict_t;
    #[no_mangle]
    fn PM_RecursiveHullCheck(hull: *mut hull_t, num: libc::c_int,
                             p1f: libc::c_float, p2f: libc::c_float,
                             p1: *mut vec_t, p2: *mut vec_t,
                             trace: *mut pmtrace_t) -> qboolean;
    #[no_mangle]
    fn PM_HullPointContents(hull: *mut hull_t, num: libc::c_int,
                            p: *const vec_t) -> libc::c_int;
    #[no_mangle]
    fn PM_RecursiveSurfCheck(model: *mut model_t, node: *mut mnode_t,
                             p1: *mut vec_t, p2: *mut vec_t)
     -> *mut msurface_t;
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
pub type vec4_t = [vec_t; 4];
pub type matrix4x4 = [[vec_t; 4]; 4];
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
pub type world_static_t = world_static_s;
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
/*
sv_world.c - world query functions
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
pub type moveclip_t = moveclip_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct moveclip_s {
    pub boxmins: vec3_t,
    pub boxmaxs: vec3_t,
    pub mins: *mut libc::c_float,
    pub maxs: *mut libc::c_float,
    pub mins2: vec3_t,
    pub maxs2: vec3_t,
    pub start: *const libc::c_float,
    pub end: *const libc::c_float,
    pub passedict: *mut edict_t,
    pub trace: trace_t,
    pub type_0: libc::c_int,
    pub ignoretrans: qboolean,
    pub monsterclip: qboolean,
}
#[inline(always)]
unsafe extern "C" fn __tg_fabs(mut __x: libc::c_float) -> libc::c_float {
    return fabsf(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_floor(mut __x: libc::c_float) -> libc::c_float {
    return floorf(__x);
}
// enclose the test object along entire move
// size of the moving object
// size when clipping against mosnters
// move type
/*
===============================================================================

HULL BOXES

===============================================================================
*/
static mut box_hull: hull_t =
    hull_t{clipnodes: 0 as *const mclipnode_t as *mut mclipnode_t,
           planes: 0 as *const mplane_t as *mut mplane_t,
           firstclipnode: 0,
           lastclipnode: 0,
           clip_mins: [0.; 3],
           clip_maxs: [0.; 3],};
static mut box_clipnodes: [mclipnode_t; 6] =
    [mclipnode_t{planenum: 0, children: [0; 2],}; 6];
static mut box_planes: [mplane_t; 6] =
    [mplane_t{normal: [0.; 3],
              dist: 0.,
              type_0: 0,
              signbits: 0,
              pad: [0; 2],}; 6];
/*
===================
SV_InitBoxHull

Set up the planes and clipnodes so that the six floats of a bounding box
can just be stored out and get a proper hull_t structure.
===================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_InitBoxHull() {
    let mut i: libc::c_int = 0;
    let mut side: libc::c_int = 0;
    box_hull.clipnodes = box_clipnodes.as_mut_ptr();
    box_hull.planes = box_planes.as_mut_ptr();
    box_hull.firstclipnode = 0 as libc::c_int;
    box_hull.lastclipnode = 5 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        box_clipnodes[i as usize].planenum = i;
        side = i & 1 as libc::c_int;
        box_clipnodes[i as usize].children[side as usize] =
            -(1 as libc::c_int) as libc::c_short;
        if i != 5 as libc::c_int {
            box_clipnodes[i as
                              usize].children[(side ^ 1 as libc::c_int) as
                                                  usize] =
                (i + 1 as libc::c_int) as libc::c_short
        } else {
            box_clipnodes[i as
                              usize].children[(side ^ 1 as libc::c_int) as
                                                  usize] =
                -(2 as libc::c_int) as libc::c_short
        }
        box_planes[i as usize].type_0 = (i >> 1 as libc::c_int) as byte;
        box_planes[i as usize].normal[(i >> 1 as libc::c_int) as usize] =
            1 as libc::c_int as vec_t;
        box_planes[i as usize].signbits = 0 as libc::c_int as byte;
        i += 1
    };
}
/*
====================
StudioPlayerBlend

====================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_StudioPlayerBlend(mut pseqdesc:
                                                  *mut mstudioseqdesc_t,
                                              mut pBlend: *mut libc::c_int,
                                              mut pPitch:
                                                  *mut libc::c_float) {
    // calc up/down pointing
    *pBlend = (*pPitch * 3 as libc::c_int as libc::c_float) as libc::c_int;
    if (*pBlend as libc::c_float) <
           (*pseqdesc).blendstart[0 as libc::c_int as usize] {
        *pPitch -= (*pseqdesc).blendstart[0 as libc::c_int as usize] / 3.0f32;
        *pBlend = 0 as libc::c_int
    } else if *pBlend as libc::c_float >
                  (*pseqdesc).blendend[0 as libc::c_int as usize] {
        *pPitch -= (*pseqdesc).blendend[0 as libc::c_int as usize] / 3.0f32;
        *pBlend = 255 as libc::c_int
    } else {
        if (*pseqdesc).blendend[0 as libc::c_int as usize] -
               (*pseqdesc).blendstart[0 as libc::c_int as usize] < 0.1f32 {
            // catch qc error
            *pBlend = 127 as libc::c_int
        } else {
            *pBlend =
                (255.0f32 *
                     (*pBlend as libc::c_float -
                          (*pseqdesc).blendstart[0 as libc::c_int as usize]) /
                     ((*pseqdesc).blendend[0 as libc::c_int as usize] -
                          (*pseqdesc).blendstart[0 as libc::c_int as usize]))
                    as libc::c_int
        }
        *pPitch = 0 as libc::c_int as libc::c_float
    };
}
/*
====================
SV_CheckSphereIntersection

check clients only
====================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_CheckSphereIntersection(mut ent: *mut edict_t,
                                                    mut start: *const vec_t,
                                                    mut end: *const vec_t)
 -> qboolean {
    let mut i: libc::c_int = 0;
    let mut sequence: libc::c_int = 0;
    let mut radiusSquared: libc::c_float = 0.;
    let mut traceOrg: vec3_t = [0.; 3];
    let mut traceDir: vec3_t = [0.; 3];
    let mut pstudiohdr: *mut studiohdr_t = 0 as *mut studiohdr_t;
    let mut pseqdesc: *mut mstudioseqdesc_t = 0 as *mut mstudioseqdesc_t;
    let mut mod_0: *mut model_t = 0 as *mut model_t;
    if (*ent).v.flags as libc::c_uint &
           ((1 as libc::c_uint) << 3 as libc::c_int |
                (1 as libc::c_uint) << 13 as libc::c_int) == 0 {
        return true_0
    }
    mod_0 = SV_ModelHandle((*ent).v.modelindex);
    if mod_0.is_null() { return true_0 }
    pstudiohdr = Mod_StudioExtradata(mod_0) as *mut studiohdr_t;
    if pstudiohdr.is_null() { return true_0 }
    sequence = (*ent).v.sequence;
    if sequence < 0 as libc::c_int || sequence >= (*pstudiohdr).numseq {
        sequence = 0 as libc::c_int
    }
    pseqdesc =
        ((pstudiohdr as *mut byte).offset((*pstudiohdr).seqindex as isize) as
             *mut mstudioseqdesc_t).offset(sequence as isize);
    traceOrg[0 as libc::c_int as usize] =
        *start.offset(0 as libc::c_int as isize);
    traceOrg[1 as libc::c_int as usize] =
        *start.offset(1 as libc::c_int as isize);
    traceOrg[2 as libc::c_int as usize] =
        *start.offset(2 as libc::c_int as isize);
    traceDir[0 as libc::c_int as usize] =
        *end.offset(0 as libc::c_int as isize) -
            *start.offset(0 as libc::c_int as isize);
    traceDir[1 as libc::c_int as usize] =
        *end.offset(1 as libc::c_int as isize) -
            *start.offset(1 as libc::c_int as isize);
    traceDir[2 as libc::c_int as usize] =
        *end.offset(2 as libc::c_int as isize) -
            *start.offset(2 as libc::c_int as isize);
    radiusSquared = 0.0f32;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        radiusSquared +=
            if __tg_fabs((*pseqdesc).bbmin[i as usize]) >
                   __tg_fabs((*pseqdesc).bbmax[i as usize]) {
                __tg_fabs((*pseqdesc).bbmin[i as usize])
            } else { __tg_fabs((*pseqdesc).bbmax[i as usize]) };
        i += 1
    }
    return SphereIntersect((*ent).v.origin.as_mut_ptr() as *const vec_t,
                           radiusSquared,
                           traceOrg.as_mut_ptr() as *const vec_t,
                           traceDir.as_mut_ptr() as *const vec_t);
}
/*
===================
SV_HullForBox

To keep everything totally uniform, bounding boxes are turned into small
BSP trees instead of being compared directly.
===================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_HullForBox(mut mins: *const vec_t,
                                       mut maxs: *const vec_t)
 -> *mut hull_t {
    box_planes[0 as libc::c_int as usize].dist =
        *maxs.offset(0 as libc::c_int as isize);
    box_planes[1 as libc::c_int as usize].dist =
        *mins.offset(0 as libc::c_int as isize);
    box_planes[2 as libc::c_int as usize].dist =
        *maxs.offset(1 as libc::c_int as isize);
    box_planes[3 as libc::c_int as usize].dist =
        *mins.offset(1 as libc::c_int as isize);
    box_planes[4 as libc::c_int as usize].dist =
        *maxs.offset(2 as libc::c_int as isize);
    box_planes[5 as libc::c_int as usize].dist =
        *mins.offset(2 as libc::c_int as isize);
    return &mut box_hull;
}
/*
==================
SV_HullAutoSelect

select the apropriate hull automatically
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_HullAutoSelect(mut model: *mut model_t,
                                           mut mins: *const vec_t,
                                           mut maxs: *const vec_t,
                                           mut size: *const vec_t,
                                           mut offset: *mut vec_t)
 -> *mut hull_t {
    let mut curdiff: libc::c_float = 0.; // assume we fail
    let mut lastdiff: libc::c_float = 999 as libc::c_int as libc::c_float;
    let mut i: libc::c_int = 0;
    let mut hullNumber: libc::c_int = 0 as libc::c_int;
    let mut clip_size: vec3_t = [0.; 3];
    let mut hull: *mut hull_t = 0 as *mut hull_t;
    // NOTE: this is not matched with hardcoded values in some cases...
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        clip_size[0 as libc::c_int as usize] =
            (*model).hulls[i as usize].clip_maxs[0 as libc::c_int as usize] -
                (*model).hulls[i as
                                   usize].clip_mins[0 as libc::c_int as
                                                        usize];
        clip_size[1 as libc::c_int as usize] =
            (*model).hulls[i as usize].clip_maxs[1 as libc::c_int as usize] -
                (*model).hulls[i as
                                   usize].clip_mins[1 as libc::c_int as
                                                        usize];
        clip_size[2 as libc::c_int as usize] =
            (*model).hulls[i as usize].clip_maxs[2 as libc::c_int as usize] -
                (*model).hulls[i as
                                   usize].clip_mins[2 as libc::c_int as
                                                        usize];
        curdiff =
            __tg_floor((*size.offset(0 as libc::c_int as isize) +
                            *size.offset(1 as libc::c_int as isize) +
                            *size.offset(2 as libc::c_int as isize)) /
                           3 as libc::c_int as libc::c_float) -
                __tg_floor((clip_size[0 as libc::c_int as usize] +
                                clip_size[1 as libc::c_int as usize] +
                                clip_size[2 as libc::c_int as usize]) /
                               3 as libc::c_int as libc::c_float);
        curdiff = __tg_fabs(curdiff);
        if curdiff < lastdiff { hullNumber = i; lastdiff = curdiff }
        i += 1
    }
    // TraceHull stuff
    hull =
        &mut *(*model).hulls.as_mut_ptr().offset(hullNumber as isize) as
            *mut hull_t;
    // calculate an offset value to center the origin
	// NOTE: never get offset of drawing hull
    if hullNumber == 0 {
        *offset.offset(0 as libc::c_int as isize) =
            (*hull).clip_mins[0 as libc::c_int as usize];
        *offset.offset(1 as libc::c_int as isize) =
            (*hull).clip_mins[1 as libc::c_int as usize];
        *offset.offset(2 as libc::c_int as isize) =
            (*hull).clip_mins[2 as libc::c_int as usize]
    } else {
        *offset.offset(0 as libc::c_int as isize) =
            (*hull).clip_mins[0 as libc::c_int as usize] -
                *mins.offset(0 as libc::c_int as isize);
        *offset.offset(1 as libc::c_int as isize) =
            (*hull).clip_mins[1 as libc::c_int as usize] -
                *mins.offset(1 as libc::c_int as isize);
        *offset.offset(2 as libc::c_int as isize) =
            (*hull).clip_mins[2 as libc::c_int as usize] -
                *mins.offset(2 as libc::c_int as isize)
    }
    return hull;
}
/*
==================
SV_HullForBsp

forcing to select BSP hull
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_HullForBsp(mut ent: *mut edict_t,
                                       mut mins: *const vec_t,
                                       mut maxs: *const vec_t,
                                       mut offset: *mut vec_t)
 -> *mut hull_t {
    let mut hull: *mut hull_t = 0 as *mut hull_t;
    let mut model: *mut model_t = 0 as *mut model_t;
    let mut size: vec3_t = [0.; 3];
    if svgame.physFuncs.SV_HullForBsp.is_some() {
        hull =
            svgame.physFuncs.SV_HullForBsp.expect("non-null function pointer")(ent,
                                                                               mins,
                                                                               maxs,
                                                                               offset)
                as *mut hull_t;
        if !hull.is_null() { return hull }
    }
    // decide which clipping hull to use, based on the size
    model = SV_ModelHandle((*ent).v.modelindex);
    if model.is_null() ||
           (*model).type_0 as libc::c_int != mod_brush as libc::c_int {
        Host_Error(b"Entity %i (%s) SOLID_BSP with a non bsp model %s\n\x00"
                       as *const u8 as *const libc::c_char,
                   ent.wrapping_offset_from(svgame.edicts) as libc::c_long as
                       libc::c_int, SV_ClassName(ent),
                   SV_GetString((*ent).v.model));
    }
    size[0 as libc::c_int as usize] =
        *maxs.offset(0 as libc::c_int as isize) -
            *mins.offset(0 as libc::c_int as isize);
    size[1 as libc::c_int as usize] =
        *maxs.offset(1 as libc::c_int as isize) -
            *mins.offset(1 as libc::c_int as isize);
    size[2 as libc::c_int as usize] =
        *maxs.offset(2 as libc::c_int as isize) -
            *mins.offset(2 as libc::c_int as isize);
    // g-cont: find a better method to detect quake-maps?
    if world.flags as libc::c_uint & (1 as libc::c_uint) << 0 as libc::c_int
           != 0 {
        // alternate hull select for quake maps
        if size[0 as libc::c_int as usize] < 3.0f32 ||
               (*ent).v.solid == 6 as libc::c_int {
            hull =
                &mut *(*model).hulls.as_mut_ptr().offset(0 as libc::c_int as
                                                             isize) as
                    *mut hull_t
        } else if size[0 as libc::c_int as usize] <= 32.0f32 {
            hull =
                &mut *(*model).hulls.as_mut_ptr().offset(1 as libc::c_int as
                                                             isize) as
                    *mut hull_t
        } else {
            hull =
                &mut *(*model).hulls.as_mut_ptr().offset(2 as libc::c_int as
                                                             isize) as
                    *mut hull_t
        }
        *offset.offset(0 as libc::c_int as isize) =
            (*hull).clip_mins[0 as libc::c_int as usize] -
                *mins.offset(0 as libc::c_int as isize);
        *offset.offset(1 as libc::c_int as isize) =
            (*hull).clip_mins[1 as libc::c_int as usize] -
                *mins.offset(1 as libc::c_int as isize);
        *offset.offset(2 as libc::c_int as isize) =
            (*hull).clip_mins[2 as libc::c_int as usize] -
                *mins.offset(2 as libc::c_int as isize)
    } else if size[0 as libc::c_int as usize] <= 8.0f32 ||
                  (*ent).v.solid == 6 as libc::c_int {
        hull =
            &mut *(*model).hulls.as_mut_ptr().offset(0 as libc::c_int as
                                                         isize) as
                *mut hull_t;
        *offset.offset(0 as libc::c_int as isize) =
            (*hull).clip_mins[0 as libc::c_int as usize];
        *offset.offset(1 as libc::c_int as isize) =
            (*hull).clip_mins[1 as libc::c_int as usize];
        *offset.offset(2 as libc::c_int as isize) =
            (*hull).clip_mins[2 as libc::c_int as usize]
    } else {
        if size[0 as libc::c_int as usize] <= 36.0f32 {
            if size[2 as libc::c_int as usize] <= 36.0f32 {
                hull =
                    &mut *(*model).hulls.as_mut_ptr().offset(3 as libc::c_int
                                                                 as isize) as
                        *mut hull_t
            } else {
                hull =
                    &mut *(*model).hulls.as_mut_ptr().offset(1 as libc::c_int
                                                                 as isize) as
                        *mut hull_t
            }
        } else {
            hull =
                &mut *(*model).hulls.as_mut_ptr().offset(2 as libc::c_int as
                                                             isize) as
                    *mut hull_t
        }
        *offset.offset(0 as libc::c_int as isize) =
            (*hull).clip_mins[0 as libc::c_int as usize] -
                *mins.offset(0 as libc::c_int as isize);
        *offset.offset(1 as libc::c_int as isize) =
            (*hull).clip_mins[1 as libc::c_int as usize] -
                *mins.offset(1 as libc::c_int as isize);
        *offset.offset(2 as libc::c_int as isize) =
            (*hull).clip_mins[2 as libc::c_int as usize] -
                *mins.offset(2 as libc::c_int as isize)
    }
    *offset.offset(0 as libc::c_int as isize) =
        *offset.offset(0 as libc::c_int as isize) +
            (*ent).v.origin[0 as libc::c_int as usize];
    *offset.offset(1 as libc::c_int as isize) =
        *offset.offset(1 as libc::c_int as isize) +
            (*ent).v.origin[1 as libc::c_int as usize];
    *offset.offset(2 as libc::c_int as isize) =
        *offset.offset(2 as libc::c_int as isize) +
            (*ent).v.origin[2 as libc::c_int as usize];
    return hull;
}
/*
================
SV_HullForEntity

Returns a hull that can be used for testing or clipping an object of mins/maxs
size.
Offset is filled in to contain the adjustment that must be added to the
testing object's origin to get a point to use with the returned hull.
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_HullForEntity(mut ent: *mut edict_t,
                                          mut mins: *mut vec_t,
                                          mut maxs: *mut vec_t,
                                          mut offset: *mut vec_t)
 -> *mut hull_t {
    let mut hull: *mut hull_t = 0 as *mut hull_t;
    let mut hullmins: vec3_t = [0.; 3];
    let mut hullmaxs: vec3_t = [0.; 3];
    if (*ent).v.solid == 4 as libc::c_int ||
           (*ent).v.solid == 6 as libc::c_int {
        if (*ent).v.solid != 6 as libc::c_int {
            if (*ent).v.movetype != 7 as libc::c_int &&
                   (*ent).v.movetype != 13 as libc::c_int {
                Host_Error(b"\'%s\' has SOLID_BSP without MOVETYPE_PUSH or MOVETYPE_PUSHSTEP\n\x00"
                               as *const u8 as *const libc::c_char,
                           SV_ClassName(ent));
            }
        }
        hull =
            SV_HullForBsp(ent, mins as *const vec_t, maxs as *const vec_t,
                          offset)
    } else {
        // create a temp hull from bounding box sizes
        hullmins[0 as libc::c_int as usize] =
            (*ent).v.mins[0 as libc::c_int as usize] -
                *maxs.offset(0 as libc::c_int as isize);
        hullmins[1 as libc::c_int as usize] =
            (*ent).v.mins[1 as libc::c_int as usize] -
                *maxs.offset(1 as libc::c_int as isize);
        hullmins[2 as libc::c_int as usize] =
            (*ent).v.mins[2 as libc::c_int as usize] -
                *maxs.offset(2 as libc::c_int as isize);
        hullmaxs[0 as libc::c_int as usize] =
            (*ent).v.maxs[0 as libc::c_int as usize] -
                *mins.offset(0 as libc::c_int as isize);
        hullmaxs[1 as libc::c_int as usize] =
            (*ent).v.maxs[1 as libc::c_int as usize] -
                *mins.offset(1 as libc::c_int as isize);
        hullmaxs[2 as libc::c_int as usize] =
            (*ent).v.maxs[2 as libc::c_int as usize] -
                *mins.offset(2 as libc::c_int as isize);
        hull =
            SV_HullForBox(hullmins.as_mut_ptr() as *const vec_t,
                          hullmaxs.as_mut_ptr() as *const vec_t);
        *offset.offset(0 as libc::c_int as isize) =
            (*ent).v.origin[0 as libc::c_int as usize];
        *offset.offset(1 as libc::c_int as isize) =
            (*ent).v.origin[1 as libc::c_int as usize];
        *offset.offset(2 as libc::c_int as isize) =
            (*ent).v.origin[2 as libc::c_int as usize]
    }
    return hull;
}
/*
====================
SV_HullForStudioModel

====================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_HullForStudioModel(mut ent: *mut edict_t,
                                               mut mins: *mut vec_t,
                                               mut maxs: *mut vec_t,
                                               mut offset: *mut vec_t,
                                               mut numhitboxes:
                                                   *mut libc::c_int)
 -> *mut hull_t {
    let mut useComplexHull: qboolean = false_0;
    let mut scale: libc::c_float = 0.5f32;
    let mut hull: *mut hull_t = 0 as *mut hull_t;
    let mut size: vec3_t = [0.; 3];
    let mut mod_0: *mut model_t = 0 as *mut model_t;
    mod_0 = SV_ModelHandle((*ent).v.modelindex);
    if mod_0.is_null() {
        *numhitboxes = 1 as libc::c_int;
        return SV_HullForEntity(ent, mins, maxs, offset)
    }
    size[0 as libc::c_int as usize] =
        *maxs.offset(0 as libc::c_int as isize) -
            *mins.offset(0 as libc::c_int as isize);
    size[1 as libc::c_int as usize] =
        *maxs.offset(1 as libc::c_int as isize) -
            *mins.offset(1 as libc::c_int as isize);
    size[2 as libc::c_int as usize] =
        *maxs.offset(2 as libc::c_int as isize) -
            *mins.offset(2 as libc::c_int as isize);
    useComplexHull = false_0;
    if size[0 as libc::c_int as usize] == 0.0f32 &&
           size[1 as libc::c_int as usize] == 0.0f32 &&
           size[2 as libc::c_int as usize] == 0.0f32 &&
           (*svgame.globals).trace_flags as libc::c_uint &
               (1 as libc::c_uint) << 0 as libc::c_int == 0 {
        useComplexHull = true_0;
        if (*ent).v.flags as libc::c_uint &
               ((1 as libc::c_uint) << 3 as libc::c_int |
                    (1 as libc::c_uint) << 13 as libc::c_int) != 0 {
            if sv_clienttrace.value == 0.0f32 {
                // so no way to trace studiomodels by hitboxes
				// use bbox instead
                useComplexHull = false_0
            } else {
                scale = sv_clienttrace.value * 0.5f32;
                size[0 as libc::c_int as usize] = 1.0f32;
                size[1 as libc::c_int as usize] = 1.0f32;
                size[2 as libc::c_int as usize] = 1.0f32
            }
        }
    }
    if (*mod_0).flags as libc::c_uint &
           (1 as libc::c_uint) << 9 as libc::c_int != 0 ||
           useComplexHull as libc::c_uint != 0 {
        size[0 as libc::c_int as usize] =
            size[0 as libc::c_int as usize] * scale;
        size[1 as libc::c_int as usize] =
            size[1 as libc::c_int as usize] * scale;
        size[2 as libc::c_int as usize] =
            size[2 as libc::c_int as usize] * scale;
        let ref mut fresh0 = *offset.offset(2 as libc::c_int as isize);
        *fresh0 = 0 as libc::c_int as vec_t;
        let ref mut fresh1 = *offset.offset(1 as libc::c_int as isize);
        *fresh1 = *fresh0;
        *offset.offset(0 as libc::c_int as isize) = *fresh1;
        if (*ent).v.flags as libc::c_uint &
               ((1 as libc::c_uint) << 3 as libc::c_int |
                    (1 as libc::c_uint) << 13 as libc::c_int) != 0 {
            let mut pstudio: *mut studiohdr_t = 0 as *mut studiohdr_t;
            let mut pseqdesc: *mut mstudioseqdesc_t =
                0 as *mut mstudioseqdesc_t;
            let mut controller: [byte; 4] = [0; 4];
            let mut blending: [byte; 2] = [0; 2];
            let mut angles: vec3_t = [0.; 3];
            let mut iBlend: libc::c_int = 0;
            pstudio = Mod_StudioExtradata(mod_0) as *mut studiohdr_t;
            pseqdesc =
                ((pstudio as *mut byte).offset((*pstudio).seqindex as isize)
                     as
                     *mut mstudioseqdesc_t).offset((*ent).v.sequence as
                                                       isize);
            angles[0 as libc::c_int as usize] =
                (*ent).v.angles[0 as libc::c_int as usize];
            angles[1 as libc::c_int as usize] =
                (*ent).v.angles[1 as libc::c_int as usize];
            angles[2 as libc::c_int as usize] =
                (*ent).v.angles[2 as libc::c_int as usize];
            SV_StudioPlayerBlend(pseqdesc, &mut iBlend,
                                 &mut *angles.as_mut_ptr().offset(0 as
                                                                      libc::c_int
                                                                      as
                                                                      isize));
            controller[1 as libc::c_int as usize] =
                0x7f as libc::c_int as byte;
            controller[0 as libc::c_int as usize] =
                controller[1 as libc::c_int as usize];
            controller[3 as libc::c_int as usize] =
                0x7f as libc::c_int as byte;
            controller[2 as libc::c_int as usize] =
                controller[3 as libc::c_int as usize];
            blending[0 as libc::c_int as usize] = iBlend as byte;
            blending[1 as libc::c_int as usize] = 0 as libc::c_int as byte;
            hull =
                Mod_HullForStudio(mod_0, (*ent).v.frame, (*ent).v.sequence,
                                  angles.as_mut_ptr(),
                                  (*ent).v.origin.as_mut_ptr(),
                                  size.as_mut_ptr(), controller.as_mut_ptr(),
                                  blending.as_mut_ptr(), numhitboxes, ent)
        } else {
            hull =
                Mod_HullForStudio(mod_0, (*ent).v.frame, (*ent).v.sequence,
                                  (*ent).v.angles.as_mut_ptr(),
                                  (*ent).v.origin.as_mut_ptr(),
                                  size.as_mut_ptr(),
                                  (*ent).v.controller.as_mut_ptr(),
                                  (*ent).v.blending.as_mut_ptr(), numhitboxes,
                                  ent)
        }
    }
    if !hull.is_null() { return hull }
    *numhitboxes = 1 as libc::c_int;
    return SV_HullForEntity(ent, mins, maxs, offset);
}
/*
===============================================================================

ENTITY AREA CHECKING

===============================================================================
*/
static mut iTouchLinkSemaphore: libc::c_int = 0 as libc::c_int;
// prevent recursion when SV_TouchLinks is active
#[no_mangle]
pub static mut sv_areanodes: [areanode_t; 32] =
    [areanode_t{axis: 0,
                dist: 0.,
                children: [0 as *const areanode_s as *mut areanode_s; 2],
                trigger_edicts:
                    link_t{prev: 0 as *const link_s as *mut link_s,
                           next: 0 as *const link_s as *mut link_s,},
                solid_edicts:
                    link_t{prev: 0 as *const link_s as *mut link_s,
                           next: 0 as *const link_s as *mut link_s,},
                portal_edicts:
                    link_t{prev: 0 as *const link_s as *mut link_s,
                           next: 0 as *const link_s as *mut link_s,},}; 32];
static mut sv_numareanodes: libc::c_int = 0;
/*
===============
SV_CreateAreaNode

builds a uniformly subdivided tree for the given world size
===============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_CreateAreaNode(mut depth: libc::c_int,
                                           mut mins: *mut vec_t,
                                           mut maxs: *mut vec_t)
 -> *mut areanode_t {
    let mut anode: *mut areanode_t = 0 as *mut areanode_t;
    let mut size: vec3_t = [0.; 3];
    let mut mins1: vec3_t = [0.; 3];
    let mut maxs1: vec3_t = [0.; 3];
    let mut mins2: vec3_t = [0.; 3];
    let mut maxs2: vec3_t = [0.; 3];
    let fresh2 = sv_numareanodes;
    sv_numareanodes = sv_numareanodes + 1;
    anode =
        &mut *sv_areanodes.as_mut_ptr().offset(fresh2 as isize) as
            *mut areanode_t;
    ClearLink(&mut (*anode).trigger_edicts);
    ClearLink(&mut (*anode).solid_edicts);
    ClearLink(&mut (*anode).portal_edicts);
    if depth == 4 as libc::c_int {
        (*anode).axis = -(1 as libc::c_int);
        (*anode).children[1 as libc::c_int as usize] = 0 as *mut areanode_s;
        (*anode).children[0 as libc::c_int as usize] =
            (*anode).children[1 as libc::c_int as usize];
        return anode
    }
    size[0 as libc::c_int as usize] =
        *maxs.offset(0 as libc::c_int as isize) -
            *mins.offset(0 as libc::c_int as isize);
    size[1 as libc::c_int as usize] =
        *maxs.offset(1 as libc::c_int as isize) -
            *mins.offset(1 as libc::c_int as isize);
    size[2 as libc::c_int as usize] =
        *maxs.offset(2 as libc::c_int as isize) -
            *mins.offset(2 as libc::c_int as isize);
    if size[0 as libc::c_int as usize] > size[1 as libc::c_int as usize] {
        (*anode).axis = 0 as libc::c_int
    } else { (*anode).axis = 1 as libc::c_int }
    (*anode).dist =
        0.5f32 *
            (*maxs.offset((*anode).axis as isize) +
                 *mins.offset((*anode).axis as isize));
    mins1[0 as libc::c_int as usize] =
        *mins.offset(0 as libc::c_int as isize);
    mins1[1 as libc::c_int as usize] =
        *mins.offset(1 as libc::c_int as isize);
    mins1[2 as libc::c_int as usize] =
        *mins.offset(2 as libc::c_int as isize);
    mins2[0 as libc::c_int as usize] =
        *mins.offset(0 as libc::c_int as isize);
    mins2[1 as libc::c_int as usize] =
        *mins.offset(1 as libc::c_int as isize);
    mins2[2 as libc::c_int as usize] =
        *mins.offset(2 as libc::c_int as isize);
    maxs1[0 as libc::c_int as usize] =
        *maxs.offset(0 as libc::c_int as isize);
    maxs1[1 as libc::c_int as usize] =
        *maxs.offset(1 as libc::c_int as isize);
    maxs1[2 as libc::c_int as usize] =
        *maxs.offset(2 as libc::c_int as isize);
    maxs2[0 as libc::c_int as usize] =
        *maxs.offset(0 as libc::c_int as isize);
    maxs2[1 as libc::c_int as usize] =
        *maxs.offset(1 as libc::c_int as isize);
    maxs2[2 as libc::c_int as usize] =
        *maxs.offset(2 as libc::c_int as isize);
    mins2[(*anode).axis as usize] = (*anode).dist;
    maxs1[(*anode).axis as usize] = mins2[(*anode).axis as usize];
    (*anode).children[0 as libc::c_int as usize] =
        SV_CreateAreaNode(depth + 1 as libc::c_int, mins2.as_mut_ptr(),
                          maxs2.as_mut_ptr());
    (*anode).children[1 as libc::c_int as usize] =
        SV_CreateAreaNode(depth + 1 as libc::c_int, mins1.as_mut_ptr(),
                          maxs1.as_mut_ptr());
    return anode;
}
/*
===============
SV_ClearWorld

===============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_ClearWorld() {
    let mut i: libc::c_int = 0; // for box testing
    SV_InitBoxHull();
    // clear lightstyles
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        sv.lightstyles[i as usize].value = 256.0f32;
        sv.lightstyles[i as usize].time = 0.0f32;
        i += 1
    }
    memset(sv_areanodes.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[areanode_t; 32]>() as libc::c_ulong);
    iTouchLinkSemaphore = 0 as libc::c_int;
    sv_numareanodes = 0 as libc::c_int;
    SV_CreateAreaNode(0 as libc::c_int, (*sv.worldmodel).mins.as_mut_ptr(),
                      (*sv.worldmodel).maxs.as_mut_ptr());
}
/*
===============
SV_UnlinkEdict
===============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_UnlinkEdict(mut ent: *mut edict_t) {
    // not linked in anywhere
    if (*ent).area.prev.is_null() { return }
    RemoveLink(&mut (*ent).area);
    (*ent).area.prev = 0 as *mut link_s;
    (*ent).area.next = 0 as *mut link_s;
}
/*
====================
SV_TouchLinks
====================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_TouchLinks(mut ent: *mut edict_t,
                                       mut node: *mut areanode_t) {
    let mut l: *mut link_t = 0 as *mut link_t;
    let mut next: *mut link_t = 0 as *mut link_t;
    let mut touch: *mut edict_t = 0 as *mut edict_t;
    let mut hull: *mut hull_t = 0 as *mut hull_t;
    let mut test: vec3_t = [0.; 3];
    let mut offset: vec3_t = [0.; 3];
    let mut mod_0: *mut model_t = 0 as *mut model_t;
    let mut current_block_14: u64;
    // touch linked edicts
    l = (*node).trigger_edicts.next;
    while l != &mut (*node).trigger_edicts as *mut link_t {
        next = (*l).next;
        touch =
            (l as *mut byte).offset(-(8 as libc::c_ulong as isize)) as
                *mut edict_t;
        if svgame.physFuncs.SV_TriggerTouch.is_some() {
            // user dll can override trigger checking (Xash3D extension)
            if svgame.physFuncs.SV_TriggerTouch.expect("non-null function pointer")(ent,
                                                                                    touch)
                   == 0 {
                current_block_14 = 4644295000439058019;
            } else { current_block_14 = 11057878835866523405; }
        } else if touch == ent || (*touch).v.solid != 1 as libc::c_int {
            current_block_14 = 4644295000439058019;
        } else {
            if (*touch).v.groupinfo != 0 && (*ent).v.groupinfo != 0 {
                if svs.groupop == 0 as libc::c_int &&
                       (*touch).v.groupinfo & (*ent).v.groupinfo == 0 {
                    current_block_14 = 4644295000439058019;
                } else if svs.groupop == 1 as libc::c_int &&
                              (*touch).v.groupinfo & (*ent).v.groupinfo != 0 {
                    current_block_14 = 4644295000439058019;
                } else { current_block_14 = 12349973810996921269; }
            } else { current_block_14 = 12349973810996921269; }
            match current_block_14 {
                4644295000439058019 => { }
                _ => {
                    if BoundsIntersect((*ent).v.absmin.as_mut_ptr() as
                                           *const vec_t,
                                       (*ent).v.absmax.as_mut_ptr() as
                                           *const vec_t,
                                       (*touch).v.absmin.as_mut_ptr() as
                                           *const vec_t,
                                       (*touch).v.absmax.as_mut_ptr() as
                                           *const vec_t) as u64 == 0 {
                        current_block_14 = 4644295000439058019;
                    } else {
                        mod_0 = SV_ModelHandle((*touch).v.modelindex);
                        // check brush triggers accuracy
                        if !mod_0.is_null() &&
                               (*mod_0).type_0 as libc::c_int ==
                                   mod_brush as libc::c_int {
                            // force to select bsp-hull
                            hull =
                                SV_HullForBsp(touch,
                                              (*ent).v.mins.as_mut_ptr() as
                                                  *const vec_t,
                                              (*ent).v.maxs.as_mut_ptr() as
                                                  *const vec_t,
                                              offset.as_mut_ptr());
                            // support for rotational triggers
                            if (*mod_0).flags as libc::c_uint &
                                   (1 as libc::c_uint) << 1 as libc::c_int !=
                                   0 &&
                                   !((*touch).v.angles[0 as libc::c_int as
                                                           usize] == 0.0f32 &&
                                         (*touch).v.angles[1 as libc::c_int as
                                                               usize] ==
                                             0.0f32 &&
                                         (*touch).v.angles[2 as libc::c_int as
                                                               usize] ==
                                             0.0f32) {
                                let mut matrix: matrix4x4 = [[0.; 4]; 4];
                                Matrix4x4_CreateFromEntity(matrix.as_mut_ptr(),
                                                           (*touch).v.angles.as_mut_ptr()
                                                               as
                                                               *const vec_t,
                                                           offset.as_mut_ptr()
                                                               as
                                                               *const vec_t,
                                                           1.0f32);
                                Matrix4x4_VectorITransform(matrix.as_mut_ptr()
                                                               as
                                                               *const [vec_t; 4],
                                                           (*ent).v.origin.as_mut_ptr()
                                                               as
                                                               *const libc::c_float,
                                                           test.as_mut_ptr());
                            } else {
                                // offset the test point appropriately for this hull.
                                test[0 as libc::c_int as usize] =
                                    (*ent).v.origin[0 as libc::c_int as usize]
                                        - offset[0 as libc::c_int as usize];
                                test[1 as libc::c_int as usize] =
                                    (*ent).v.origin[1 as libc::c_int as usize]
                                        - offset[1 as libc::c_int as usize];
                                test[2 as libc::c_int as usize] =
                                    (*ent).v.origin[2 as libc::c_int as usize]
                                        - offset[2 as libc::c_int as usize]
                            }
                            // test hull for intersection with this model
                            if PM_HullPointContents(hull,
                                                    (*hull).firstclipnode,
                                                    test.as_mut_ptr() as
                                                        *const vec_t) !=
                                   -(2 as libc::c_int) {
                                current_block_14 = 4644295000439058019;
                            } else {
                                current_block_14 = 11057878835866523405;
                            }
                        } else { current_block_14 = 11057878835866523405; }
                    }
                }
            }
        }
        match current_block_14 {
            11057878835866523405 => {
                // never touch the triggers when "playersonly" is active
                if sv.playersonly as u64 == 0 {
                    (*svgame.globals).time = sv.time as libc::c_float;
                    svgame.dllFuncs.pfnTouch.expect("non-null function pointer")(touch,
                                                                                 ent);
                }
            }
            _ => { }
        }
        // disabled ?
        l = next
    }
    // recurse down both sides
    if (*node).axis == -(1 as libc::c_int) { return }
    if (*ent).v.absmax[(*node).axis as usize] > (*node).dist {
        SV_TouchLinks(ent, (*node).children[0 as libc::c_int as usize]);
    }
    if (*ent).v.absmin[(*node).axis as usize] < (*node).dist {
        SV_TouchLinks(ent, (*node).children[1 as libc::c_int as usize]);
    };
}
/*
===============
SV_FindTouchedLeafs

===============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_FindTouchedLeafs(mut ent: *mut edict_t,
                                             mut node: *mut mnode_t,
                                             mut headnode: *mut libc::c_int) {
    let mut sides: libc::c_int = 0;
    let mut leaf: *mut mleaf_t = 0 as *mut mleaf_t;
    if (*node).contents == -(2 as libc::c_int) { return }
    // add an efrag if the node is a leaf
    if (*node).contents < 0 as libc::c_int {
        if (*ent).num_leafs > 48 as libc::c_int - 1 as libc::c_int {
            // continue counting leafs,
			// so we know how many it's overrun
            (*ent).num_leafs = 48 as libc::c_int + 1 as libc::c_int
        } else {
            leaf = node as *mut mleaf_t;
            (*ent).leafnums[(*ent).num_leafs as usize] =
                (*leaf).cluster as libc::c_short;
            (*ent).num_leafs += 1
        }
        return
    }
    // NODE_MIXED
    sides =
        if ((*(*node).plane).type_0 as libc::c_int) < 3 as libc::c_int {
            if (*(*node).plane).dist <=
                   (*ent).v.absmin[(*(*node).plane).type_0 as usize] {
                1 as libc::c_int
            } else if (*(*node).plane).dist >=
                          (*ent).v.absmax[(*(*node).plane).type_0 as usize] {
                2 as libc::c_int
            } else { 3 as libc::c_int }
        } else {
            BoxOnPlaneSide((*ent).v.absmin.as_mut_ptr() as *const vec_t,
                           (*ent).v.absmax.as_mut_ptr() as *const vec_t,
                           (*node).plane)
        };
    if sides == 3 as libc::c_int && *headnode == -(1 as libc::c_int) {
        *headnode =
            node.wrapping_offset_from((*sv.worldmodel).nodes) as libc::c_long
                as libc::c_int
    }
    // recurse down the contacted sides
    if sides & 1 as libc::c_int != 0 {
        SV_FindTouchedLeafs(ent, (*node).children[0 as libc::c_int as usize],
                            headnode);
    }
    if sides & 2 as libc::c_int != 0 {
        SV_FindTouchedLeafs(ent, (*node).children[1 as libc::c_int as usize],
                            headnode);
    };
}
/*
===============
SV_LinkEdict
===============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_LinkEdict(mut ent: *mut edict_t,
                                      mut touch_triggers: qboolean) {
    let mut node: *mut areanode_t =
        0 as *mut areanode_t; // unlink from old position
    let mut headnode: libc::c_int = 0; // don't add the world
    if !(*ent).area.prev.is_null() {
        SV_UnlinkEdict(ent); // never add freed ents
    }
    if ent == svgame.edicts { return }
    if SV_CheckEdict(ent,
                     b"../engine/server/sv_world.c\x00" as *const u8 as
                         *const libc::c_char, 639 as libc::c_int) as u64 == 0
       {
        return
    }
    // set the abs box
    svgame.dllFuncs.pfnSetAbsBox.expect("non-null function pointer")(ent);
    if (*ent).v.movetype == 12 as libc::c_int &&
           SV_CheckEdict((*ent).v.aiment,
                         b"../engine/server/sv_world.c\x00" as *const u8 as
                             *const libc::c_char, 644 as libc::c_int) as
               libc::c_uint != 0 {
        memcpy((*ent).leafnums.as_mut_ptr() as *mut libc::c_void,
               (*(*ent).v.aiment).leafnums.as_mut_ptr() as
                   *const libc::c_void,
               ::std::mem::size_of::<[libc::c_short; 48]>() as libc::c_ulong);
        (*ent).num_leafs = (*(*ent).v.aiment).num_leafs;
        (*ent).headnode = (*(*ent).v.aiment).headnode
    } else {
        // link to PVS leafs
        (*ent).num_leafs = 0 as libc::c_int; // so we use headnode instead
        (*ent).headnode = -(1 as libc::c_int);
        headnode = -(1 as libc::c_int);
        if (*ent).v.modelindex != 0 {
            SV_FindTouchedLeafs(ent, (*sv.worldmodel).nodes, &mut headnode);
        }
        if (*ent).num_leafs > 48 as libc::c_int {
            memset((*ent).leafnums.as_mut_ptr() as *mut libc::c_void,
                   -(1 as libc::c_int),
                   ::std::mem::size_of::<[libc::c_short; 48]>() as
                       libc::c_ulong);
            (*ent).num_leafs = 0 as libc::c_int;
            (*ent).headnode = headnode
        }
    }
    // ignore non-solid bodies
    if (*ent).v.solid == 0 as libc::c_int &&
           (*ent).v.skin >= -(1 as libc::c_int) {
        return
    }
    // find the first node that the ent's box crosses
    node = sv_areanodes.as_mut_ptr();
    while !((*node).axis == -(1 as libc::c_int)) {
        if (*ent).v.absmin[(*node).axis as usize] > (*node).dist {
            node = (*node).children[0 as libc::c_int as usize]
        } else {
            if !((*ent).v.absmax[(*node).axis as usize] < (*node).dist) {
                break ;
            }
            node = (*node).children[1 as libc::c_int as usize]
        }
        // crosses the node
    }
    // link it in
    if (*ent).v.solid == 1 as libc::c_int {
        InsertLinkBefore(&mut (*ent).area, &mut (*node).trigger_edicts);
    } else if (*ent).v.solid == 6 as libc::c_int {
        InsertLinkBefore(&mut (*ent).area, &mut (*node).portal_edicts);
    } else { InsertLinkBefore(&mut (*ent).area, &mut (*node).solid_edicts); }
    if touch_triggers as libc::c_uint != 0 && iTouchLinkSemaphore == 0 {
        iTouchLinkSemaphore = true_0 as libc::c_int;
        SV_TouchLinks(ent, sv_areanodes.as_mut_ptr());
        iTouchLinkSemaphore = false_0 as libc::c_int
    };
}
/*
===============================================================================

POINT TESTING IN HULLS

===============================================================================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_WaterLinks(mut origin: *const vec_t,
                                       mut pCont: *mut libc::c_int,
                                       mut node: *mut areanode_t) {
    let mut l: *mut link_t = 0 as *mut link_t;
    let mut next: *mut link_t = 0 as *mut link_t;
    let mut touch: *mut edict_t = 0 as *mut edict_t;
    let mut hull: *mut hull_t = 0 as *mut hull_t;
    let mut test: vec3_t = [0.; 3];
    let mut offset: vec3_t = [0.; 3];
    let mut mod_0: *mut model_t = 0 as *mut model_t;
    let mut current_block_12: u64;
    // get water edicts
    l = (*node).solid_edicts.next;
    while l != &mut (*node).solid_edicts as *mut link_t {
        next = (*l).next;
        touch =
            (l as *mut byte).offset(-(8 as libc::c_ulong as isize)) as
                *mut edict_t;
        if !((*touch).v.solid != 0 as libc::c_int) {
            if (*touch).v.groupinfo != 0 {
                if svs.groupop == 0 as libc::c_int &&
                       (*touch).v.groupinfo & svs.groupmask == 0 {
                    current_block_12 = 4644295000439058019;
                } else if svs.groupop == 1 as libc::c_int &&
                              (*touch).v.groupinfo & svs.groupmask != 0 {
                    current_block_12 = 4644295000439058019;
                } else { current_block_12 = 2968425633554183086; }
            } else { current_block_12 = 2968425633554183086; }
            match current_block_12 {
                4644295000439058019 => { }
                _ => {
                    mod_0 = SV_ModelHandle((*touch).v.modelindex);
                    // new content has more priority
                    // only brushes can have special contents
                    if !(mod_0.is_null() ||
                             (*mod_0).type_0 as libc::c_int !=
                                 mod_brush as libc::c_int) {
                        if !(BoundsIntersect(origin, origin,
                                             (*touch).v.absmin.as_mut_ptr() as
                                                 *const vec_t,
                                             (*touch).v.absmax.as_mut_ptr() as
                                                 *const vec_t) as u64 == 0) {
                            // check water brushes accuracy
                            hull =
                                SV_HullForBsp(touch,
                                              vec3_origin.as_mut_ptr() as
                                                  *const vec_t,
                                              vec3_origin.as_mut_ptr() as
                                                  *const vec_t,
                                              offset.as_mut_ptr());
                            // support for rotational water
                            if (*mod_0).flags as libc::c_uint &
                                   (1 as libc::c_uint) << 1 as libc::c_int !=
                                   0 &&
                                   !((*touch).v.angles[0 as libc::c_int as
                                                           usize] == 0.0f32 &&
                                         (*touch).v.angles[1 as libc::c_int as
                                                               usize] ==
                                             0.0f32 &&
                                         (*touch).v.angles[2 as libc::c_int as
                                                               usize] ==
                                             0.0f32) {
                                let mut matrix: matrix4x4 = [[0.; 4]; 4];
                                Matrix4x4_CreateFromEntity(matrix.as_mut_ptr(),
                                                           (*touch).v.angles.as_mut_ptr()
                                                               as
                                                               *const vec_t,
                                                           offset.as_mut_ptr()
                                                               as
                                                               *const vec_t,
                                                           1.0f32);
                                Matrix4x4_VectorITransform(matrix.as_mut_ptr()
                                                               as
                                                               *const [vec_t; 4],
                                                           origin,
                                                           test.as_mut_ptr());
                            } else {
                                // offset the test point appropriately for this hull.
                                test[0 as libc::c_int as usize] =
                                    *origin.offset(0 as libc::c_int as isize)
                                        - offset[0 as libc::c_int as usize];
                                test[1 as libc::c_int as usize] =
                                    *origin.offset(1 as libc::c_int as isize)
                                        - offset[1 as libc::c_int as usize];
                                test[2 as libc::c_int as usize] =
                                    *origin.offset(2 as libc::c_int as isize)
                                        - offset[2 as libc::c_int as usize]
                            }
                            // test hull for intersection with this model
                            if !(PM_HullPointContents(hull,
                                                      (*hull).firstclipnode,
                                                      test.as_mut_ptr() as
                                                          *const vec_t) ==
                                     -(1 as libc::c_int)) {
                                // compare contents ranking
                                if RankForContents((*touch).v.skin) >
                                       RankForContents(*pCont) {
                                    *pCont = (*touch).v.skin
                                }
                            }
                        }
                    }
                }
            }
        }
        l = next
    }
    // disabled ?
    // recurse down both sides
    if (*node).axis == -(1 as libc::c_int) { return }
    if *origin.offset((*node).axis as isize) > (*node).dist {
        SV_WaterLinks(origin, pCont,
                      (*node).children[0 as libc::c_int as usize]);
    }
    if *origin.offset((*node).axis as isize) < (*node).dist {
        SV_WaterLinks(origin, pCont,
                      (*node).children[1 as libc::c_int as usize]);
    };
}
/*
=============
SV_TruePointContents

=============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_TruePointContents(mut p: *const vec_t)
 -> libc::c_int {
    let mut cont: libc::c_int = 0;
    // sanity check
    if p.is_null() { return 0 as libc::c_int }
    // get base contents from world
    cont =
        PM_HullPointContents(&mut *(*sv.worldmodel).hulls.as_mut_ptr().offset(0
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize),
                             0 as libc::c_int, p);
    // check all water entities
    SV_WaterLinks(p, &mut cont, sv_areanodes.as_mut_ptr());
    return cont;
}
/*
=============
SV_PointContents

=============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_PointContents(mut p: *const vec_t)
 -> libc::c_int {
    let mut cont: libc::c_int = SV_TruePointContents(p);
    if cont <= -(9 as libc::c_int) && cont >= -(14 as libc::c_int) {
        cont = -(3 as libc::c_int)
    }
    return cont;
}
//===========================================================================
/*
============
SV_TestEntityPosition

returns true if the entity is in solid currently
============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_TestEntityPosition(mut ent: *mut edict_t,
                                               mut blocker: *mut edict_t)
 -> qboolean {
    let mut monsterClip: qboolean =
        if (*ent).v.flags as libc::c_uint &
               (1 as libc::c_uint) << 23 as libc::c_int != 0 {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int } as qboolean;
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
    if (*ent).v.flags as libc::c_uint &
           ((1 as libc::c_uint) << 3 as libc::c_int |
                (1 as libc::c_uint) << 13 as libc::c_int) != 0 {
        // to avoid falling through tracktrain update client mins\maxs here
        if (*ent).v.flags as libc::c_uint &
               (1 as libc::c_uint) << 14 as libc::c_int != 0 {
            SV_SetMinMaxSize(ent,
                             (*svgame.pmove).player_mins[1 as libc::c_int as
                                                             usize].as_mut_ptr(),
                             (*svgame.pmove).player_maxs[1 as libc::c_int as
                                                             usize].as_mut_ptr(),
                             true_0);
        } else {
            SV_SetMinMaxSize(ent,
                             (*svgame.pmove).player_mins[0 as libc::c_int as
                                                             usize].as_mut_ptr(),
                             (*svgame.pmove).player_maxs[0 as libc::c_int as
                                                             usize].as_mut_ptr(),
                             true_0);
        }
    }
    trace =
        SV_Move((*ent).v.origin.as_mut_ptr() as *const vec_t,
                (*ent).v.mins.as_mut_ptr(), (*ent).v.maxs.as_mut_ptr(),
                (*ent).v.origin.as_mut_ptr() as *const vec_t,
                0 as libc::c_int, ent, monsterClip);
    if SV_CheckEdict(blocker,
                     b"../engine/server/sv_world.c\x00" as *const u8 as
                         *const libc::c_char, 837 as libc::c_int) as
           libc::c_uint != 0 &&
           SV_CheckEdict(trace.ent,
                         b"../engine/server/sv_world.c\x00" as *const u8 as
                             *const libc::c_char, 837 as libc::c_int) as
               libc::c_uint != 0 {
        if (*trace.ent).v.movetype == 7 as libc::c_int || trace.ent == blocker
           {
            return trace.startsolid
        }
        return false_0
    }
    return trace.startsolid;
}
/*
===============================================================================

LINE TESTING IN HULLS

===============================================================================
*/
/*
==================
SV_ClipMoveToEntity

Handles selection or creation of a clipping hull, and offseting (and
eventually rotation) of the end points
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_ClipMoveToEntity(mut ent: *mut edict_t,
                                             mut start: *const vec_t,
                                             mut mins: *mut vec_t,
                                             mut maxs: *mut vec_t,
                                             mut end: *const vec_t,
                                             mut trace: *mut trace_t) {
    let mut hull: *mut hull_t = 0 as *mut hull_t;
    let mut model: *mut model_t = 0 as *mut model_t;
    let mut start_l: vec3_t = [0.; 3];
    let mut end_l: vec3_t = [0.; 3];
    let mut offset: vec3_t = [0.; 3];
    let mut temp: vec3_t = [0.; 3];
    let mut last_hitgroup: libc::c_int = 0;
    let mut trace_hitbox: trace_t =
        trace_t{allsolid: false_0,
                startsolid: false_0,
                inopen: false_0,
                inwater: false_0,
                fraction: 0.,
                endpos: [0.; 3],
                plane: plane_t{normal: [0.; 3], dist: 0.,},
                ent: 0 as *mut edict_t,
                hitgroup: 0,};
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut hullcount: libc::c_int = 0;
    let mut rotated: qboolean = false_0;
    let mut transform_bbox: qboolean = false_0;
    let mut matrix: matrix4x4 = [[0.; 4]; 4];
    memset(trace as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<trace_t>() as libc::c_ulong);
    (*trace).endpos[0 as libc::c_int as usize] =
        *end.offset(0 as libc::c_int as isize);
    (*trace).endpos[1 as libc::c_int as usize] =
        *end.offset(1 as libc::c_int as isize);
    (*trace).endpos[2 as libc::c_int as usize] =
        *end.offset(2 as libc::c_int as isize);
    (*trace).fraction = 1.0f32;
    (*trace).allsolid = true_0;
    model = SV_ModelHandle((*ent).v.modelindex);
    if !model.is_null() &&
           (*model).type_0 as libc::c_int == mod_studio as libc::c_int {
        hull =
            SV_HullForStudioModel(ent, mins, maxs, offset.as_mut_ptr(),
                                  &mut hullcount)
    } else {
        hull = SV_HullForEntity(ent, mins, maxs, offset.as_mut_ptr());
        hullcount = 1 as libc::c_int
    }
    // rotate start and end into the models frame of reference
    if ((*ent).v.solid == 4 as libc::c_int ||
            (*ent).v.solid == 6 as libc::c_int) &&
           !((*ent).v.angles[0 as libc::c_int as usize] == 0.0f32 &&
                 (*ent).v.angles[1 as libc::c_int as usize] == 0.0f32 &&
                 (*ent).v.angles[2 as libc::c_int as usize] == 0.0f32) {
        rotated = true_0
    } else { rotated = false_0 }
    if host.features &
           ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint != 0 {
        // keep untransformed bbox less than 45 degress or train on subtransit.bsp will stop working
        if ((*ent).v.angles[0 as libc::c_int as usize] as libc::c_int ==
                90 as libc::c_int ||
                (*ent).v.angles[0 as libc::c_int as usize] as libc::c_int ==
                    180 as libc::c_int ||
                (*ent).v.angles[0 as libc::c_int as usize] as libc::c_int ==
                    270 as libc::c_int ||
                (*ent).v.angles[0 as libc::c_int as usize] as libc::c_int ==
                    -(90 as libc::c_int) ||
                (*ent).v.angles[0 as libc::c_int as usize] as libc::c_int ==
                    -(180 as libc::c_int) ||
                (*ent).v.angles[0 as libc::c_int as usize] as libc::c_int ==
                    -(270 as libc::c_int) ||
                ((*ent).v.angles[2 as libc::c_int as usize] as libc::c_int ==
                     90 as libc::c_int ||
                     (*ent).v.angles[2 as libc::c_int as usize] as libc::c_int
                         == 180 as libc::c_int ||
                     (*ent).v.angles[2 as libc::c_int as usize] as libc::c_int
                         == 270 as libc::c_int ||
                     (*ent).v.angles[2 as libc::c_int as usize] as libc::c_int
                         == -(90 as libc::c_int) ||
                     (*ent).v.angles[2 as libc::c_int as usize] as libc::c_int
                         == -(180 as libc::c_int) ||
                     (*ent).v.angles[2 as libc::c_int as usize] as libc::c_int
                         == -(270 as libc::c_int))) &&
               !(*mins.offset(0 as libc::c_int as isize) == 0.0f32 &&
                     *mins.offset(1 as libc::c_int as isize) == 0.0f32 &&
                     *mins.offset(2 as libc::c_int as isize) == 0.0f32) {
            transform_bbox = true_0
        } else { transform_bbox = false_0 }
    } else { transform_bbox = false_0 } // calc new local offset
    if rotated as u64 != 0 {
        let mut out_mins: vec3_t = [0.; 3];
        let mut out_maxs: vec3_t = [0.; 3];
        if transform_bbox as u64 != 0 {
            Matrix4x4_CreateFromEntity(matrix.as_mut_ptr(),
                                       (*ent).v.angles.as_mut_ptr() as
                                           *const vec_t,
                                       (*ent).v.origin.as_mut_ptr() as
                                           *const vec_t, 1.0f32);
        } else {
            Matrix4x4_CreateFromEntity(matrix.as_mut_ptr(),
                                       (*ent).v.angles.as_mut_ptr() as
                                           *const vec_t,
                                       offset.as_mut_ptr() as *const vec_t,
                                       1.0f32);
        }
        Matrix4x4_VectorITransform(matrix.as_mut_ptr() as *const [vec_t; 4],
                                   start, start_l.as_mut_ptr());
        Matrix4x4_VectorITransform(matrix.as_mut_ptr() as *const [vec_t; 4],
                                   end, end_l.as_mut_ptr());
        if transform_bbox as u64 != 0 {
            World_TransformAABB(matrix.as_mut_ptr(), mins as *const vec_t,
                                maxs as *const vec_t, out_mins.as_mut_ptr(),
                                out_maxs.as_mut_ptr());
            offset[0 as libc::c_int as usize] =
                (*hull).clip_mins[0 as libc::c_int as usize] -
                    out_mins[0 as libc::c_int as usize];
            offset[1 as libc::c_int as usize] =
                (*hull).clip_mins[1 as libc::c_int as usize] -
                    out_mins[1 as libc::c_int as usize];
            offset[2 as libc::c_int as usize] =
                (*hull).clip_mins[2 as libc::c_int as usize] -
                    out_mins[2 as libc::c_int as usize];
            j = 0 as libc::c_int;
            while j < 3 as libc::c_int {
                if start_l[j as usize] >= 0.0f32 {
                    start_l[j as usize] -= offset[j as usize]
                } else { start_l[j as usize] += offset[j as usize] }
                if end_l[j as usize] >= 0.0f32 {
                    end_l[j as usize] -= offset[j as usize]
                } else { end_l[j as usize] += offset[j as usize] }
                j += 1
            }
        }
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
    if hullcount == 1 as libc::c_int {
        PM_RecursiveHullCheck(hull, (*hull).firstclipnode, 0.0f32, 1.0f32,
                              start_l.as_mut_ptr(), end_l.as_mut_ptr(),
                              trace as *mut pmtrace_t);
    } else {
        last_hitgroup = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < hullcount {
            memset(&mut trace_hitbox as *mut trace_t as *mut libc::c_void,
                   0 as libc::c_int,
                   ::std::mem::size_of::<trace_t>() as libc::c_ulong);
            trace_hitbox.endpos[0 as libc::c_int as usize] =
                *end.offset(0 as libc::c_int as isize);
            trace_hitbox.endpos[1 as libc::c_int as usize] =
                *end.offset(1 as libc::c_int as isize);
            trace_hitbox.endpos[2 as libc::c_int as usize] =
                *end.offset(2 as libc::c_int as isize);
            trace_hitbox.fraction = 1.0f64 as libc::c_float;
            trace_hitbox.allsolid = true_0;
            PM_RecursiveHullCheck(&mut *hull.offset(i as isize),
                                  (*hull.offset(i as isize)).firstclipnode,
                                  0.0f32, 1.0f32, start_l.as_mut_ptr(),
                                  end_l.as_mut_ptr(),
                                  &mut trace_hitbox as *mut trace_t as
                                      *mut pmtrace_t);
            if i == 0 as libc::c_int ||
                   trace_hitbox.allsolid as libc::c_uint != 0 ||
                   trace_hitbox.startsolid as libc::c_uint != 0 ||
                   trace_hitbox.fraction < (*trace).fraction {
                if (*trace).startsolid as u64 != 0 {
                    *trace = trace_hitbox;
                    (*trace).startsolid = true_0
                } else { *trace = trace_hitbox }
                last_hitgroup = i
            }
            i += 1
        }
        (*trace).hitgroup = Mod_HitgroupForStudioHull(last_hitgroup)
    }
    if (*trace).fraction != 1.0f32 {
        // compute endpos (generic case)
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
        if rotated as u64 != 0 {
            // transform plane
            temp[0 as libc::c_int as usize] =
                (*trace).plane.normal[0 as libc::c_int as usize];
            temp[1 as libc::c_int as usize] =
                (*trace).plane.normal[1 as libc::c_int as usize];
            temp[2 as libc::c_int as usize] =
                (*trace).plane.normal[2 as libc::c_int as usize];
            Matrix4x4_TransformPositivePlane(matrix.as_mut_ptr() as
                                                 *const [vec_t; 4],
                                             temp.as_mut_ptr() as
                                                 *const vec_t,
                                             (*trace).plane.dist,
                                             (*trace).plane.normal.as_mut_ptr(),
                                             &mut (*trace).plane.dist);
        } else {
            (*trace).plane.dist =
                (*trace).endpos[0 as libc::c_int as usize] *
                    (*trace).plane.normal[0 as libc::c_int as usize] +
                    (*trace).endpos[1 as libc::c_int as usize] *
                        (*trace).plane.normal[1 as libc::c_int as usize] +
                    (*trace).endpos[2 as libc::c_int as usize] *
                        (*trace).plane.normal[2 as libc::c_int as usize]
        }
    }
    if (*trace).fraction < 1.0f32 || (*trace).startsolid as libc::c_uint != 0
       {
        (*trace).ent = ent
    };
}
/*
==================
SV_PortalCSG

a portal is flush with a world surface behind it. this causes problems. namely that we can't pass through the portal plane
if the bsp behind it prevents out origin from getting through. so if the trace was clipped and ended infront of the portal,
continue the trace to the edges of the portal cutout instead.
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_PortalCSG(mut portal: *mut edict_t,
                                      mut trace_mins: *const vec_t,
                                      mut trace_maxs: *const vec_t,
                                      mut start: *const vec_t,
                                      mut end: *const vec_t,
                                      mut trace: *mut trace_t) {
    let mut planes: [vec4_t; 6] =
        [[0.; 4]; 6]; //far, near, right, left, up, down
    let mut plane: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut worldpos: vec3_t = [0.; 3];
    let mut bestfrac: libc::c_float = 0.;
    let mut hitplane: libc::c_int = 0;
    let mut model: *mut model_t = 0 as *mut model_t;
    let mut portalradius: libc::c_float = 0.;
    // only run this code if we impacted on the portal's parent.
    if (*trace).fraction == 1.0f32 && (*trace).startsolid as u64 == 0 {
        return
    }
    // decide which clipping hull to use, based on the size
    model = SV_ModelHandle((*portal).v.modelindex);
    if model.is_null() ||
           (*model).type_0 as libc::c_int != mod_brush as libc::c_int {
        return
    }
    // make sure we use a sane valid position.
    if (*trace).startsolid as u64 != 0 {
        worldpos[0 as libc::c_int as usize] =
            *start.offset(0 as libc::c_int as isize);
        worldpos[1 as libc::c_int as usize] =
            *start.offset(1 as libc::c_int as isize);
        worldpos[2 as libc::c_int as usize] =
            *start.offset(2 as libc::c_int as isize)
    } else {
        worldpos[0 as libc::c_int as usize] =
            (*trace).endpos[0 as libc::c_int as usize];
        worldpos[1 as libc::c_int as usize] =
            (*trace).endpos[1 as libc::c_int as usize];
        worldpos[2 as libc::c_int as usize] =
            (*trace).endpos[2 as libc::c_int as usize]
    }
    // determine the csg area. normals should be facing in
    AngleVectors((*portal).v.angles.as_mut_ptr() as *const vec_t,
                 planes[1 as libc::c_int as usize].as_mut_ptr(),
                 planes[3 as libc::c_int as usize].as_mut_ptr(),
                 planes[5 as libc::c_int as
                            usize].as_mut_ptr()); //an epsilon beyond the portal
    planes[0 as libc::c_int as usize][0 as libc::c_int as usize] =
        -planes[1 as libc::c_int as usize][0 as libc::c_int as usize];
    planes[0 as libc::c_int as usize][1 as libc::c_int as usize] =
        -planes[1 as libc::c_int as usize][1 as libc::c_int as usize];
    planes[0 as libc::c_int as usize][2 as libc::c_int as usize] =
        -planes[1 as libc::c_int as usize][2 as libc::c_int as usize];
    planes[2 as libc::c_int as usize][0 as libc::c_int as usize] =
        -planes[3 as libc::c_int as usize][0 as libc::c_int as usize];
    planes[2 as libc::c_int as usize][1 as libc::c_int as usize] =
        -planes[3 as libc::c_int as usize][1 as libc::c_int as usize];
    planes[2 as libc::c_int as usize][2 as libc::c_int as usize] =
        -planes[3 as libc::c_int as usize][2 as libc::c_int as usize];
    planes[4 as libc::c_int as usize][0 as libc::c_int as usize] =
        -planes[5 as libc::c_int as usize][0 as libc::c_int as usize];
    planes[4 as libc::c_int as usize][1 as libc::c_int as usize] =
        -planes[5 as libc::c_int as usize][1 as libc::c_int as usize];
    planes[4 as libc::c_int as usize][2 as libc::c_int as usize] =
        -planes[5 as libc::c_int as usize][2 as libc::c_int as usize];
    portalradius = (*model).radius * 0.5f32;
    planes[0 as libc::c_int as usize][3 as libc::c_int as usize] =
        (*portal).v.origin[0 as libc::c_int as usize] *
            planes[0 as libc::c_int as usize][0 as libc::c_int as usize] +
            (*portal).v.origin[1 as libc::c_int as usize] *
                planes[0 as libc::c_int as usize][1 as libc::c_int as usize] +
            (*portal).v.origin[2 as libc::c_int as usize] *
                planes[0 as libc::c_int as usize][2 as libc::c_int as usize] -
            4.0f32 / 32.0f32;
    planes[1 as libc::c_int as usize][3 as libc::c_int as usize] =
        (*portal).v.origin[0 as libc::c_int as usize] *
            planes[1 as libc::c_int as usize][0 as libc::c_int as usize] +
            (*portal).v.origin[1 as libc::c_int as usize] *
                planes[1 as libc::c_int as usize][1 as libc::c_int as usize] +
            (*portal).v.origin[2 as libc::c_int as usize] *
                planes[1 as libc::c_int as usize][2 as libc::c_int as usize] -
            4.0f32 / 32.0f32;
    planes[2 as libc::c_int as usize][3 as libc::c_int as usize] =
        (*portal).v.origin[0 as libc::c_int as usize] *
            planes[2 as libc::c_int as usize][0 as libc::c_int as usize] +
            (*portal).v.origin[1 as libc::c_int as usize] *
                planes[2 as libc::c_int as usize][1 as libc::c_int as usize] +
            (*portal).v.origin[2 as libc::c_int as usize] *
                planes[2 as libc::c_int as usize][2 as libc::c_int as usize] -
            portalradius;
    planes[3 as libc::c_int as usize][3 as libc::c_int as usize] =
        (*portal).v.origin[0 as libc::c_int as usize] *
            planes[3 as libc::c_int as usize][0 as libc::c_int as usize] +
            (*portal).v.origin[1 as libc::c_int as usize] *
                planes[3 as libc::c_int as usize][1 as libc::c_int as usize] +
            (*portal).v.origin[2 as libc::c_int as usize] *
                planes[3 as libc::c_int as usize][2 as libc::c_int as usize] -
            portalradius;
    planes[4 as libc::c_int as usize][3 as libc::c_int as usize] =
        (*portal).v.origin[0 as libc::c_int as usize] *
            planes[4 as libc::c_int as usize][0 as libc::c_int as usize] +
            (*portal).v.origin[1 as libc::c_int as usize] *
                planes[4 as libc::c_int as usize][1 as libc::c_int as usize] +
            (*portal).v.origin[2 as libc::c_int as usize] *
                planes[4 as libc::c_int as usize][2 as libc::c_int as usize] -
            portalradius;
    planes[5 as libc::c_int as usize][3 as libc::c_int as usize] =
        (*portal).v.origin[0 as libc::c_int as usize] *
            planes[5 as libc::c_int as usize][0 as libc::c_int as usize] +
            (*portal).v.origin[1 as libc::c_int as usize] *
                planes[5 as libc::c_int as usize][1 as libc::c_int as usize] +
            (*portal).v.origin[2 as libc::c_int as usize] *
                planes[5 as libc::c_int as usize][2 as libc::c_int as usize] -
            portalradius;
    // if we're actually inside the csg region
    plane = 0 as libc::c_int;
    while plane < 6 as libc::c_int {
        let mut d: libc::c_float =
            worldpos[0 as libc::c_int as usize] *
                planes[plane as usize][0 as libc::c_int as usize] +
                worldpos[1 as libc::c_int as usize] *
                    planes[plane as usize][1 as libc::c_int as usize] +
                worldpos[2 as libc::c_int as usize] *
                    planes[plane as usize][2 as libc::c_int as usize];
        let mut nearest: vec3_t = [0.; 3];
        k = 0 as libc::c_int;
        while k < 3 as libc::c_int {
            nearest[k as usize] =
                if planes[plane as usize][k as usize] >=
                       0 as libc::c_int as libc::c_float {
                    *trace_maxs.offset(k as isize)
                } else { *trace_mins.offset(k as isize) };
            k += 1
        }
        // end is already outside
        // front plane gets further away with side
        if plane == 0 {
            planes[plane as usize][3 as libc::c_int as usize] -=
                nearest[0 as libc::c_int as usize] *
                    planes[plane as usize][0 as libc::c_int as usize] +
                    nearest[1 as libc::c_int as usize] *
                        planes[plane as usize][1 as libc::c_int as usize] +
                    nearest[2 as libc::c_int as usize] *
                        planes[plane as usize][2 as libc::c_int as usize]
        } else if plane > 1 as libc::c_int {
            // side planes get nearer with size
            planes[plane as usize][3 as libc::c_int as usize] +=
                24 as libc::c_int as libc::c_float
            // DotProduct( nearest, planes[plane] );
        } // endpos is inside
        if d - planes[plane as usize][3 as libc::c_int as usize] >=
               0 as libc::c_int as libc::c_float {
            plane += 1
        } else { return }
    }
    // yup, we're inside, the trace shouldn't end where it actually did
    bestfrac = 1 as libc::c_int as libc::c_float;
    hitplane = -(1 as libc::c_int);
    plane = 0 as libc::c_int;
    while plane < 6 as libc::c_int {
        let mut ds: libc::c_float =
            *start.offset(0 as libc::c_int as isize) *
                planes[plane as usize][0 as libc::c_int as usize] +
                *start.offset(1 as libc::c_int as isize) *
                    planes[plane as usize][1 as libc::c_int as usize] +
                *start.offset(2 as libc::c_int as isize) *
                    planes[plane as usize][2 as libc::c_int as usize] -
                planes[plane as usize][3 as libc::c_int as usize];
        let mut de: libc::c_float =
            *end.offset(0 as libc::c_int as isize) *
                planes[plane as usize][0 as libc::c_int as usize] +
                *end.offset(1 as libc::c_int as isize) *
                    planes[plane as usize][1 as libc::c_int as usize] +
                *end.offset(2 as libc::c_int as isize) *
                    planes[plane as usize][2 as libc::c_int as usize] -
                planes[plane as usize][3 as libc::c_int as usize];
        let mut frac: libc::c_float = 0.;
        if ds >= 0 as libc::c_int as libc::c_float &&
               de < 0 as libc::c_int as libc::c_float {
            frac = ds / (ds - de);
            if frac < bestfrac {
                if frac < 0 as libc::c_int as libc::c_float {
                    frac = 0 as libc::c_int as libc::c_float
                }
                bestfrac = frac;
                hitplane = plane
            }
        }
        plane += 1
    }
    (*trace).allsolid = false_0;
    (*trace).startsolid = (*trace).allsolid;
    // if we cross the front of the portal, don't shorten the trace,
	// that will artificially clip us
    if hitplane == 0 as libc::c_int && (*trace).fraction > bestfrac { return }
    // okay, elongate to clip to the portal hole properly.
    (*trace).endpos[0 as libc::c_int as usize] =
        *start.offset(0 as libc::c_int as isize) +
            bestfrac *
                (*end.offset(0 as libc::c_int as isize) -
                     *start.offset(0 as libc::c_int as isize));
    (*trace).endpos[1 as libc::c_int as usize] =
        *start.offset(1 as libc::c_int as isize) +
            bestfrac *
                (*end.offset(1 as libc::c_int as isize) -
                     *start.offset(1 as libc::c_int as isize));
    (*trace).endpos[2 as libc::c_int as usize] =
        *start.offset(2 as libc::c_int as isize) +
            bestfrac *
                (*end.offset(2 as libc::c_int as isize) -
                     *start.offset(2 as libc::c_int as isize));
    (*trace).fraction = bestfrac;
    if hitplane >= 0 as libc::c_int {
        (*trace).plane.normal[0 as libc::c_int as usize] =
            planes[hitplane as usize][0 as libc::c_int as usize];
        (*trace).plane.normal[1 as libc::c_int as usize] =
            planes[hitplane as usize][1 as libc::c_int as usize];
        (*trace).plane.normal[2 as libc::c_int as usize] =
            planes[hitplane as usize][2 as libc::c_int as usize];
        (*trace).plane.dist =
            planes[hitplane as usize][3 as libc::c_int as usize];
        if hitplane == 1 as libc::c_int { (*trace).ent = portal }
    };
}
/*
==================
SV_CustomClipMoveToEntity

A part of physics engine implementation
or custom physics implementation
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_CustomClipMoveToEntity(mut ent: *mut edict_t,
                                                   mut start: *const vec_t,
                                                   mut mins: *mut vec_t,
                                                   mut maxs: *mut vec_t,
                                                   mut end: *const vec_t,
                                                   mut trace: *mut trace_t) {
    // initialize custom trace
    memset(trace as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<trace_t>() as libc::c_ulong);
    (*trace).endpos[0 as libc::c_int as usize] =
        *end.offset(0 as libc::c_int as isize);
    (*trace).endpos[1 as libc::c_int as usize] =
        *end.offset(1 as libc::c_int as isize);
    (*trace).endpos[2 as libc::c_int as usize] =
        *end.offset(2 as libc::c_int as isize);
    (*trace).allsolid = true_0;
    (*trace).fraction = 1.0f32;
    if svgame.physFuncs.ClipMoveToEntity.is_some() {
        // do custom sweep test
        svgame.physFuncs.ClipMoveToEntity.expect("non-null function pointer")(ent,
                                                                              start,
                                                                              mins,
                                                                              maxs,
                                                                              end,
                                                                              trace);
    } else {
        // function is missed, so we didn't hit anything
        (*trace).allsolid = false_0
    };
}
/*
====================
SV_ClipToEntity

generic clip function
====================
*/
unsafe extern "C" fn SV_ClipToEntity(mut touch: *mut edict_t,
                                     mut clip: *mut moveclip_t) -> qboolean {
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
    let mut mod_0: *mut model_t = 0 as *mut model_t;
    if (*touch).v.groupinfo != 0 &&
           SV_CheckEdict((*clip).passedict,
                         b"../engine/server/sv_world.c\x00" as *const u8 as
                             *const libc::c_char, 1146 as libc::c_int) as
               libc::c_uint != 0 &&
           (*(*clip).passedict).v.groupinfo != 0 as libc::c_int {
        if svs.groupop == 0 as libc::c_int &&
               (*touch).v.groupinfo & (*(*clip).passedict).v.groupinfo == 0 {
            return true_0
        }
        if svs.groupop == 1 as libc::c_int &&
               (*touch).v.groupinfo & (*(*clip).passedict).v.groupinfo != 0 {
            return true_0
        }
    }
    if touch == (*clip).passedict || (*touch).v.solid == 0 as libc::c_int {
        return true_0
    }
    if (*touch).v.solid == 1 as libc::c_int {
        Host_Error(b"trigger in clipping list\n\x00" as *const u8 as
                       *const libc::c_char);
    }
    // custom user filter
    if svgame.dllFuncs2.pfnShouldCollide.is_some() {
        if svgame.dllFuncs2.pfnShouldCollide.expect("non-null function pointer")(touch,
                                                                                 (*clip).passedict)
               == 0 {
            return true_0
        }
    }
    // monsterclip filter (solid custom is a static or dynamic bodies)
    if (*touch).v.solid == 4 as libc::c_int ||
           (*touch).v.solid == 5 as libc::c_int {
        // func_monsterclip works only with monsters that have same flag!
        if (*touch).v.flags as libc::c_uint &
               (1 as libc::c_uint) << 23 as libc::c_int != 0 &&
               (*clip).monsterclip as u64 == 0 {
            return true_0
        }
    } else if (*clip).type_0 == 1 as libc::c_int &&
                  (*touch).v.movetype != 13 as libc::c_int {
        return true_0
    }
    mod_0 = SV_ModelHandle((*touch).v.modelindex);
    if !mod_0.is_null() &&
           (*mod_0).type_0 as libc::c_int == mod_brush as libc::c_int &&
           (*clip).ignoretrans as libc::c_uint != 0 {
        // ignore all monsters but pushables
        // we ignore brushes with rendermode != kRenderNormal and without FL_WORLDBRUSH set
        if (*touch).v.rendermode != kRenderNormal as libc::c_int &&
               (*touch).v.flags as libc::c_uint &
                   (1 as libc::c_uint) << 25 as libc::c_int == 0 {
            return true_0
        }
    }
    if BoundsIntersect((*clip).boxmins.as_mut_ptr() as *const vec_t,
                       (*clip).boxmaxs.as_mut_ptr() as *const vec_t,
                       (*touch).v.absmin.as_mut_ptr() as *const vec_t,
                       (*touch).v.absmax.as_mut_ptr() as *const vec_t) as u64
           == 0 {
        return true_0
    }
    // aditional check to intersects clients with sphere
    if (*touch).v.solid != 3 as libc::c_int &&
           SV_CheckSphereIntersection(touch, (*clip).start, (*clip).end) as
               u64 == 0 {
        return true_0
    }
    // Xash3D extension
    if SV_CheckEdict((*clip).passedict,
                     b"../engine/server/sv_world.c\x00" as *const u8 as
                         *const libc::c_char, 1199 as libc::c_int) as
           libc::c_uint != 0 &&
           (*(*clip).passedict).v.solid == 1 as libc::c_int {
        // never collide items and player (because call "give" always stuck item in player
		// and total trace returns fail (old half-life bug)
		// items touch should be done in SV_TouchLinks not here
        if (*touch).v.flags as libc::c_uint &
               ((1 as libc::c_uint) << 3 as libc::c_int |
                    (1 as libc::c_uint) << 13 as libc::c_int) != 0 {
            return true_0
        }
    }
    // g-cont. make sure what size is really zero - check all the components
    if SV_CheckEdict((*clip).passedict,
                     b"../engine/server/sv_world.c\x00" as *const u8 as
                         *const libc::c_char, 1209 as libc::c_int) as
           libc::c_uint != 0 &&
           !((*(*clip).passedict).v.size[0 as libc::c_int as usize] == 0.0f32
                 &&
                 (*(*clip).passedict).v.size[1 as libc::c_int as usize] ==
                     0.0f32 &&
                 (*(*clip).passedict).v.size[2 as libc::c_int as usize] ==
                     0.0f32) &&
           ((*touch).v.size[0 as libc::c_int as usize] == 0.0f32 &&
                (*touch).v.size[1 as libc::c_int as usize] == 0.0f32 &&
                (*touch).v.size[2 as libc::c_int as usize] == 0.0f32) {
        return true_0
    } // points never interact
    // might intersect, so do an exact clip
    if (*clip).trace.allsolid as u64 != 0 {
        return false_0
    } // don't clip against own missiles
    if SV_CheckEdict((*clip).passedict,
                     b"../engine/server/sv_world.c\x00" as *const u8 as
                         *const libc::c_char, 1215 as libc::c_int) as u64 != 0
       {
        if (*touch).v.owner == (*clip).passedict { return true_0 }
        if (*(*clip).passedict).v.owner == touch { return true_0 }
        // don't clip against owner
    }
    // make sure we don't hit the world if we're inside the portal
    if (*touch).v.solid == 6 as libc::c_int {
        SV_PortalCSG(touch, (*clip).mins as *const vec_t,
                     (*clip).maxs as *const vec_t, (*clip).start, (*clip).end,
                     &mut (*clip).trace);
    }
    if (*touch).v.solid == 5 as libc::c_int {
        SV_CustomClipMoveToEntity(touch, (*clip).start, (*clip).mins,
                                  (*clip).maxs, (*clip).end, &mut trace);
    } else if (*touch).v.flags as libc::c_uint &
                  (1 as libc::c_uint) << 5 as libc::c_int != 0 {
        SV_ClipMoveToEntity(touch, (*clip).start, (*clip).mins2.as_mut_ptr(),
                            (*clip).maxs2.as_mut_ptr(), (*clip).end,
                            &mut trace);
    } else {
        SV_ClipMoveToEntity(touch, (*clip).start, (*clip).mins, (*clip).maxs,
                            (*clip).end, &mut trace);
    }
    (*clip).trace =
        World_CombineTraces(&mut (*clip).trace, &mut trace, touch);
    return true_0;
}
/*
====================
SV_ClipToLinks

Mins and maxs enclose the entire area swept by the move
====================
*/
unsafe extern "C" fn SV_ClipToLinks(mut node: *mut areanode_t,
                                    mut clip: *mut moveclip_t) {
    let mut l: *mut link_t = 0 as *mut link_t;
    let mut next: *mut link_t = 0 as *mut link_t;
    let mut touch: *mut edict_t = 0 as *mut edict_t;
    // touch linked edicts
    l = (*node).solid_edicts.next;
    while l != &mut (*node).solid_edicts as *mut link_t {
        next = (*l).next;
        touch =
            (l as *mut byte).offset(-(8 as libc::c_ulong as isize)) as
                *mut edict_t;
        if SV_ClipToEntity(touch, clip) as u64 == 0 { return }
        l = next
        // trace.allsoild
    }
    // recurse down both sides
    if (*node).axis == -(1 as libc::c_int) { return }
    if (*clip).boxmaxs[(*node).axis as usize] > (*node).dist {
        SV_ClipToLinks((*node).children[0 as libc::c_int as usize], clip);
    }
    if (*clip).boxmins[(*node).axis as usize] < (*node).dist {
        SV_ClipToLinks((*node).children[1 as libc::c_int as usize], clip);
    };
}
/*
====================
SV_ClipToPortals

Mins and maxs enclose the entire area swept by the move
====================
*/
unsafe extern "C" fn SV_ClipToPortals(mut node: *mut areanode_t,
                                      mut clip: *mut moveclip_t) {
    let mut l: *mut link_t = 0 as *mut link_t;
    let mut next: *mut link_t = 0 as *mut link_t;
    let mut touch: *mut edict_t = 0 as *mut edict_t;
    // touch linked edicts
    l = (*node).portal_edicts.next;
    while l != &mut (*node).portal_edicts as *mut link_t {
        next = (*l).next;
        touch =
            (l as *mut byte).offset(-(8 as libc::c_ulong as isize)) as
                *mut edict_t;
        if SV_ClipToEntity(touch, clip) as u64 == 0 { return }
        l = next
        // trace.allsoild
    }
    // recurse down both sides
    if (*node).axis == -(1 as libc::c_int) { return }
    if (*clip).boxmaxs[(*node).axis as usize] > (*node).dist {
        SV_ClipToPortals((*node).children[0 as libc::c_int as usize], clip);
    }
    if (*clip).boxmins[(*node).axis as usize] < (*node).dist {
        SV_ClipToPortals((*node).children[1 as libc::c_int as usize], clip);
    };
}
/*
====================
SV_ClipToWorldBrush

Mins and maxs enclose the entire area swept by the move
====================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_ClipToWorldBrush(mut node: *mut areanode_t,
                                             mut clip: *mut moveclip_t) {
    let mut l: *mut link_t = 0 as *mut link_t;
    let mut next: *mut link_t = 0 as *mut link_t;
    let mut touch: *mut edict_t = 0 as *mut edict_t;
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
    l = (*node).solid_edicts.next;
    while l != &mut (*node).solid_edicts as *mut link_t {
        next = (*l).next;
        touch =
            (l as *mut byte).offset(-(8 as libc::c_ulong as isize)) as
                *mut edict_t;
        if !((*touch).v.solid != 4 as libc::c_int ||
                 touch == (*clip).passedict ||
                 (*touch).v.flags as libc::c_uint &
                     (1 as libc::c_uint) << 25 as libc::c_int == 0) {
            if !(BoundsIntersect((*clip).boxmins.as_mut_ptr() as *const vec_t,
                                 (*clip).boxmaxs.as_mut_ptr() as *const vec_t,
                                 (*touch).v.absmin.as_mut_ptr() as
                                     *const vec_t,
                                 (*touch).v.absmax.as_mut_ptr() as
                                     *const vec_t) as u64 == 0) {
                if (*clip).trace.allsolid as u64 != 0 { return }
                SV_ClipMoveToEntity(touch, (*clip).start, (*clip).mins,
                                    (*clip).maxs, (*clip).end, &mut trace);
                (*clip).trace =
                    World_CombineTraces(&mut (*clip).trace, &mut trace, touch)
            }
        }
        l = next
    }
    // recurse down both sides
    if (*node).axis == -(1 as libc::c_int) { return }
    if (*clip).boxmaxs[(*node).axis as usize] > (*node).dist {
        SV_ClipToWorldBrush((*node).children[0 as libc::c_int as usize],
                            clip);
    }
    if (*clip).boxmins[(*node).axis as usize] < (*node).dist {
        SV_ClipToWorldBrush((*node).children[1 as libc::c_int as usize],
                            clip);
    };
}
/*
==================
SV_Move
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_Move(mut start: *const vec_t,
                                 mut mins: *mut vec_t, mut maxs: *mut vec_t,
                                 mut end: *const vec_t,
                                 mut type_0: libc::c_int, mut e: *mut edict_t,
                                 mut monsterclip: qboolean) -> trace_t {
    let mut clip: moveclip_t =
        moveclip_t{boxmins: [0.; 3],
                   boxmaxs: [0.; 3],
                   mins: 0 as *mut libc::c_float,
                   maxs: 0 as *mut libc::c_float,
                   mins2: [0.; 3],
                   maxs2: [0.; 3],
                   start: 0 as *const libc::c_float,
                   end: 0 as *const libc::c_float,
                   passedict: 0 as *mut edict_t,
                   trace:
                       trace_t{allsolid: false_0,
                               startsolid: false_0,
                               inopen: false_0,
                               inwater: false_0,
                               fraction: 0.,
                               endpos: [0.; 3],
                               plane: plane_t{normal: [0.; 3], dist: 0.,},
                               ent: 0 as *mut edict_t,
                               hitgroup: 0,},
                   type_0: 0,
                   ignoretrans: false_0,
                   monsterclip: false_0,};
    let mut trace_endpos: vec3_t = [0.; 3];
    let mut trace_fraction: libc::c_float = 0.;
    memset(&mut clip as *mut moveclip_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<moveclip_t>() as libc::c_ulong);
    SV_ClipMoveToEntity(SV_EdictNum(0 as libc::c_int), start, mins, maxs, end,
                        &mut clip.trace);
    if clip.trace.fraction != 0.0f32 {
        trace_endpos[0 as libc::c_int as usize] =
            clip.trace.endpos[0 as libc::c_int as usize];
        trace_endpos[1 as libc::c_int as usize] =
            clip.trace.endpos[1 as libc::c_int as usize];
        trace_endpos[2 as libc::c_int as usize] =
            clip.trace.endpos[2 as libc::c_int as usize];
        trace_fraction = clip.trace.fraction;
        clip.trace.fraction = 1.0f32;
        clip.start = start;
        clip.end = trace_endpos.as_mut_ptr();
        clip.type_0 = type_0 & 0xff as libc::c_int;
        clip.ignoretrans = (type_0 >> 8 as libc::c_int) as qboolean;
        clip.monsterclip = false_0;
        clip.passedict =
            if !e.is_null() { e } else { SV_EdictNum(0 as libc::c_int) };
        clip.mins = mins;
        clip.maxs = maxs;
        if monsterclip as libc::c_uint != 0 &&
               host.features &
                   ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint ==
                   0 {
            clip.monsterclip = true_0
        }
        if clip.type_0 == 2 as libc::c_int {
            clip.mins2[0 as libc::c_int as usize] = -15.0f32;
            clip.mins2[1 as libc::c_int as usize] = -15.0f32;
            clip.mins2[2 as libc::c_int as usize] = -15.0f32;
            clip.maxs2[0 as libc::c_int as usize] = 15.0f32;
            clip.maxs2[1 as libc::c_int as usize] = 15.0f32;
            clip.maxs2[2 as libc::c_int as usize] = 15.0f32
        } else {
            clip.mins2[0 as libc::c_int as usize] =
                *mins.offset(0 as libc::c_int as isize);
            clip.mins2[1 as libc::c_int as usize] =
                *mins.offset(1 as libc::c_int as isize);
            clip.mins2[2 as libc::c_int as usize] =
                *mins.offset(2 as libc::c_int as isize);
            clip.maxs2[0 as libc::c_int as usize] =
                *maxs.offset(0 as libc::c_int as isize);
            clip.maxs2[1 as libc::c_int as usize] =
                *maxs.offset(1 as libc::c_int as isize);
            clip.maxs2[2 as libc::c_int as usize] =
                *maxs.offset(2 as libc::c_int as isize)
        }
        World_MoveBounds(start, clip.mins2.as_mut_ptr(),
                         clip.maxs2.as_mut_ptr(),
                         trace_endpos.as_mut_ptr() as *const vec_t,
                         clip.boxmins.as_mut_ptr(),
                         clip.boxmaxs.as_mut_ptr());
        SV_ClipToLinks(sv_areanodes.as_mut_ptr(), &mut clip);
        SV_ClipToPortals(sv_areanodes.as_mut_ptr(), &mut clip);
        clip.trace.fraction *= trace_fraction;
        (*svgame.globals).trace_ent = clip.trace.ent
    }
    SV_CopyTraceToGlobal(&mut clip.trace);
    return clip.trace;
}
#[no_mangle]
pub unsafe extern "C" fn SV_MoveNormal(mut start: *const vec_t,
                                       mut mins: *mut vec_t,
                                       mut maxs: *mut vec_t,
                                       mut end: *const vec_t,
                                       mut type_0: libc::c_int,
                                       mut e: *mut edict_t) -> trace_t {
    return SV_Move(start, mins, maxs, end, type_0, e, false_0);
}
/*
==================
SV_MoveNoEnts
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_MoveNoEnts(mut start: *const vec_t,
                                       mut mins: *mut vec_t,
                                       mut maxs: *mut vec_t,
                                       mut end: *const vec_t,
                                       mut type_0: libc::c_int,
                                       mut e: *mut edict_t) -> trace_t {
    let mut clip: moveclip_t =
        moveclip_t{boxmins: [0.; 3],
                   boxmaxs: [0.; 3],
                   mins: 0 as *mut libc::c_float,
                   maxs: 0 as *mut libc::c_float,
                   mins2: [0.; 3],
                   maxs2: [0.; 3],
                   start: 0 as *const libc::c_float,
                   end: 0 as *const libc::c_float,
                   passedict: 0 as *mut edict_t,
                   trace:
                       trace_t{allsolid: false_0,
                               startsolid: false_0,
                               inopen: false_0,
                               inwater: false_0,
                               fraction: 0.,
                               endpos: [0.; 3],
                               plane: plane_t{normal: [0.; 3], dist: 0.,},
                               ent: 0 as *mut edict_t,
                               hitgroup: 0,},
                   type_0: 0,
                   ignoretrans: false_0,
                   monsterclip: false_0,};
    let mut trace_endpos: vec3_t = [0.; 3];
    let mut trace_fraction: libc::c_float = 0.;
    memset(&mut clip as *mut moveclip_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<moveclip_t>() as libc::c_ulong);
    SV_ClipMoveToEntity(SV_EdictNum(0 as libc::c_int), start, mins, maxs, end,
                        &mut clip.trace);
    if clip.trace.fraction != 0.0f32 {
        trace_endpos[0 as libc::c_int as usize] =
            clip.trace.endpos[0 as libc::c_int as usize];
        trace_endpos[1 as libc::c_int as usize] =
            clip.trace.endpos[1 as libc::c_int as usize];
        trace_endpos[2 as libc::c_int as usize] =
            clip.trace.endpos[2 as libc::c_int as usize];
        trace_fraction = clip.trace.fraction;
        clip.trace.fraction = 1.0f32;
        clip.start = start;
        clip.end = trace_endpos.as_mut_ptr();
        clip.type_0 = type_0 & 0xff as libc::c_int;
        clip.ignoretrans = (type_0 >> 8 as libc::c_int) as qboolean;
        clip.monsterclip = false_0;
        clip.passedict =
            if !e.is_null() { e } else { SV_EdictNum(0 as libc::c_int) };
        clip.mins = mins;
        clip.maxs = maxs;
        clip.mins2[0 as libc::c_int as usize] =
            *mins.offset(0 as libc::c_int as isize);
        clip.mins2[1 as libc::c_int as usize] =
            *mins.offset(1 as libc::c_int as isize);
        clip.mins2[2 as libc::c_int as usize] =
            *mins.offset(2 as libc::c_int as isize);
        clip.maxs2[0 as libc::c_int as usize] =
            *maxs.offset(0 as libc::c_int as isize);
        clip.maxs2[1 as libc::c_int as usize] =
            *maxs.offset(1 as libc::c_int as isize);
        clip.maxs2[2 as libc::c_int as usize] =
            *maxs.offset(2 as libc::c_int as isize);
        World_MoveBounds(start, clip.mins2.as_mut_ptr(),
                         clip.maxs2.as_mut_ptr(),
                         trace_endpos.as_mut_ptr() as *const vec_t,
                         clip.boxmins.as_mut_ptr(),
                         clip.boxmaxs.as_mut_ptr());
        SV_ClipToWorldBrush(sv_areanodes.as_mut_ptr(), &mut clip);
        SV_ClipToPortals(sv_areanodes.as_mut_ptr(), &mut clip);
        clip.trace.fraction *= trace_fraction;
        (*svgame.globals).trace_ent = clip.trace.ent
    }
    SV_CopyTraceToGlobal(&mut clip.trace);
    return clip.trace;
}
/*
==================
SV_TraceSurface

find the face where the traceline hit
assume pTextureEntity is valid
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_TraceSurface(mut ent: *mut edict_t,
                                         mut start: *const vec_t,
                                         mut end: *const vec_t)
 -> *mut msurface_t {
    let mut matrix: matrix4x4 = [[0.; 4]; 4];
    let mut bmodel: *mut model_t = 0 as *mut model_t;
    let mut hull: *mut hull_t = 0 as *mut hull_t;
    let mut start_l: vec3_t = [0.; 3];
    let mut end_l: vec3_t = [0.; 3];
    let mut offset: vec3_t = [0.; 3];
    bmodel = SV_ModelHandle((*ent).v.modelindex);
    if bmodel.is_null() ||
           (*bmodel).type_0 as libc::c_int != mod_brush as libc::c_int {
        return 0 as *mut msurface_t
    }
    hull =
        SV_HullForBsp(ent, vec3_origin.as_mut_ptr() as *const vec_t,
                      vec3_origin.as_mut_ptr() as *const vec_t,
                      offset.as_mut_ptr());
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
            offset[2 as libc::c_int as usize];
    // rotate start and end into the models frame of reference
    if !((*ent).v.angles[0 as libc::c_int as usize] == 0.0f32 &&
             (*ent).v.angles[1 as libc::c_int as usize] == 0.0f32 &&
             (*ent).v.angles[2 as libc::c_int as usize] == 0.0f32) {
        Matrix4x4_CreateFromEntity(matrix.as_mut_ptr(),
                                   (*ent).v.angles.as_mut_ptr() as
                                       *const vec_t,
                                   offset.as_mut_ptr() as *const vec_t,
                                   1.0f32);
        Matrix4x4_VectorITransform(matrix.as_mut_ptr() as *const [vec_t; 4],
                                   start, start_l.as_mut_ptr());
        Matrix4x4_VectorITransform(matrix.as_mut_ptr() as *const [vec_t; 4],
                                   end, end_l.as_mut_ptr());
    }
    return PM_RecursiveSurfCheck(bmodel,
                                 &mut *(*bmodel).nodes.offset((*hull).firstclipnode
                                                                  as isize),
                                 start_l.as_mut_ptr(), end_l.as_mut_ptr());
}
/*
==================
SV_TraceTexture

find the face where the traceline hit
assume pTextureEntity is valid
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_TraceTexture(mut ent: *mut edict_t,
                                         mut start: *const vec_t,
                                         mut end: *const vec_t)
 -> *const libc::c_char {
    let mut surf: *mut msurface_t = SV_TraceSurface(ent, start, end);
    if surf.is_null() || (*surf).texinfo.is_null() ||
           (*(*surf).texinfo).texture.is_null() {
        return 0 as *const libc::c_char
    }
    return (*(*(*surf).texinfo).texture).name.as_mut_ptr();
}
/*
==================
SV_MoveToss
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_MoveToss(mut tossent: *mut edict_t,
                                     mut ignore: *mut edict_t) -> trace_t {
    let mut gravity: libc::c_float = 0.;
    let mut move_0: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut original_origin: vec3_t = [0.; 3];
    let mut original_velocity: vec3_t = [0.; 3];
    let mut original_angles: vec3_t = [0.; 3];
    let mut original_avelocity: vec3_t = [0.; 3];
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
    let mut i: libc::c_int = 0;
    original_origin[0 as libc::c_int as usize] =
        (*tossent).v.origin[0 as libc::c_int as usize];
    original_origin[1 as libc::c_int as usize] =
        (*tossent).v.origin[1 as libc::c_int as usize];
    original_origin[2 as libc::c_int as usize] =
        (*tossent).v.origin[2 as libc::c_int as usize];
    original_velocity[0 as libc::c_int as usize] =
        (*tossent).v.velocity[0 as libc::c_int as usize];
    original_velocity[1 as libc::c_int as usize] =
        (*tossent).v.velocity[1 as libc::c_int as usize];
    original_velocity[2 as libc::c_int as usize] =
        (*tossent).v.velocity[2 as libc::c_int as usize];
    original_angles[0 as libc::c_int as usize] =
        (*tossent).v.angles[0 as libc::c_int as usize];
    original_angles[1 as libc::c_int as usize] =
        (*tossent).v.angles[1 as libc::c_int as usize];
    original_angles[2 as libc::c_int as usize] =
        (*tossent).v.angles[2 as libc::c_int as usize];
    original_avelocity[0 as libc::c_int as usize] =
        (*tossent).v.avelocity[0 as libc::c_int as usize];
    original_avelocity[1 as libc::c_int as usize] =
        (*tossent).v.avelocity[1 as libc::c_int as usize];
    original_avelocity[2 as libc::c_int as usize] =
        (*tossent).v.avelocity[2 as libc::c_int as usize];
    gravity = (*tossent).v.gravity * svgame.movevars.gravity * 0.05f32;
    i = 0 as libc::c_int;
    while i < 200 as libc::c_int {
        SV_CheckVelocity(tossent);
        (*tossent).v.velocity[2 as libc::c_int as usize] -= gravity;
        (*tossent).v.angles[0 as libc::c_int as usize] =
            (*tossent).v.angles[0 as libc::c_int as usize] +
                0.05f32 * (*tossent).v.avelocity[0 as libc::c_int as usize];
        (*tossent).v.angles[1 as libc::c_int as usize] =
            (*tossent).v.angles[1 as libc::c_int as usize] +
                0.05f32 * (*tossent).v.avelocity[1 as libc::c_int as usize];
        (*tossent).v.angles[2 as libc::c_int as usize] =
            (*tossent).v.angles[2 as libc::c_int as usize] +
                0.05f32 * (*tossent).v.avelocity[2 as libc::c_int as usize];
        move_0[0 as libc::c_int as usize] =
            (*tossent).v.velocity[0 as libc::c_int as usize] * 0.05f32;
        move_0[1 as libc::c_int as usize] =
            (*tossent).v.velocity[1 as libc::c_int as usize] * 0.05f32;
        move_0[2 as libc::c_int as usize] =
            (*tossent).v.velocity[2 as libc::c_int as usize] * 0.05f32;
        end[0 as libc::c_int as usize] =
            (*tossent).v.origin[0 as libc::c_int as usize] +
                move_0[0 as libc::c_int as usize];
        end[1 as libc::c_int as usize] =
            (*tossent).v.origin[1 as libc::c_int as usize] +
                move_0[1 as libc::c_int as usize];
        end[2 as libc::c_int as usize] =
            (*tossent).v.origin[2 as libc::c_int as usize] +
                move_0[2 as libc::c_int as usize];
        trace =
            SV_Move((*tossent).v.origin.as_mut_ptr() as *const vec_t,
                    (*tossent).v.mins.as_mut_ptr(),
                    (*tossent).v.maxs.as_mut_ptr(),
                    end.as_mut_ptr() as *const vec_t, 0 as libc::c_int,
                    tossent, false_0);
        (*tossent).v.origin[0 as libc::c_int as usize] =
            trace.endpos[0 as libc::c_int as usize];
        (*tossent).v.origin[1 as libc::c_int as usize] =
            trace.endpos[1 as libc::c_int as usize];
        (*tossent).v.origin[2 as libc::c_int as usize] =
            trace.endpos[2 as libc::c_int as usize];
        if trace.fraction < 1.0f32 { break ; }
        i += 1
    }
    (*tossent).v.origin[0 as libc::c_int as usize] =
        original_origin[0 as libc::c_int as usize];
    (*tossent).v.origin[1 as libc::c_int as usize] =
        original_origin[1 as libc::c_int as usize];
    (*tossent).v.origin[2 as libc::c_int as usize] =
        original_origin[2 as libc::c_int as usize];
    (*tossent).v.velocity[0 as libc::c_int as usize] =
        original_velocity[0 as libc::c_int as usize];
    (*tossent).v.velocity[1 as libc::c_int as usize] =
        original_velocity[1 as libc::c_int as usize];
    (*tossent).v.velocity[2 as libc::c_int as usize] =
        original_velocity[2 as libc::c_int as usize];
    (*tossent).v.angles[0 as libc::c_int as usize] =
        original_angles[0 as libc::c_int as usize];
    (*tossent).v.angles[1 as libc::c_int as usize] =
        original_angles[1 as libc::c_int as usize];
    (*tossent).v.angles[2 as libc::c_int as usize] =
        original_angles[2 as libc::c_int as usize];
    (*tossent).v.avelocity[0 as libc::c_int as usize] =
        original_avelocity[0 as libc::c_int as usize];
    (*tossent).v.avelocity[1 as libc::c_int as usize] =
        original_avelocity[1 as libc::c_int as usize];
    (*tossent).v.avelocity[2 as libc::c_int as usize] =
        original_avelocity[2 as libc::c_int as usize];
    return trace;
}
/*
===============================================================================

	LIGHTING INFO

===============================================================================
*/
static mut sv_pointColor: vec3_t = [0.; 3];
/*
=================
SV_RecursiveLightPoint
=================
*/
unsafe extern "C" fn SV_RecursiveLightPoint(mut model: *mut model_t,
                                            mut node: *mut mnode_t,
                                            mut start: *const vec_t,
                                            mut end: *const vec_t)
 -> qboolean {
    let mut front: libc::c_float = 0.;
    let mut back: libc::c_float = 0.;
    let mut scale: libc::c_float = 0.;
    let mut frac: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    let mut map: libc::c_int = 0;
    let mut side: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut ds: libc::c_float = 0.;
    let mut dt: libc::c_float = 0.;
    let mut s: libc::c_float = 0.;
    let mut t: libc::c_float = 0.;
    let mut sample_size: libc::c_int = 0;
    let mut surf: *mut msurface_t = 0 as *mut msurface_t;
    let mut tex: *mut mtexinfo_t = 0 as *mut mtexinfo_t;
    let mut info: *mut mextrasurf_t = 0 as *mut mextrasurf_t;
    let mut lm: *mut color24 = 0 as *mut color24;
    let mut mid: vec3_t = [0.; 3];
    // didn't hit anything
    if node.is_null() || (*node).contents < 0 as libc::c_int {
        return false_0
    }
    // calculate mid point
    front =
        (if ((*(*node).plane).type_0 as libc::c_int) < 3 as libc::c_int {
             *start.offset((*(*node).plane).type_0 as isize)
         } else {
             (*start.offset(0 as libc::c_int as isize) *
                  (*(*node).plane).normal[0 as libc::c_int as usize] +
                  *start.offset(1 as libc::c_int as isize) *
                      (*(*node).plane).normal[1 as libc::c_int as usize]) +
                 *start.offset(2 as libc::c_int as isize) *
                     (*(*node).plane).normal[2 as libc::c_int as usize]
         }) - (*(*node).plane).dist;
    back =
        (if ((*(*node).plane).type_0 as libc::c_int) < 3 as libc::c_int {
             *end.offset((*(*node).plane).type_0 as isize)
         } else {
             (*end.offset(0 as libc::c_int as isize) *
                  (*(*node).plane).normal[0 as libc::c_int as usize] +
                  *end.offset(1 as libc::c_int as isize) *
                      (*(*node).plane).normal[1 as libc::c_int as usize]) +
                 *end.offset(2 as libc::c_int as isize) *
                     (*(*node).plane).normal[2 as libc::c_int as usize]
         }) - (*(*node).plane).dist;
    side = (front < 0.0f32) as libc::c_int;
    if (back < 0.0f32) as libc::c_int == side {
        return SV_RecursiveLightPoint(model, (*node).children[side as usize],
                                      start, end)
    }
    frac = front / (front - back);
    mid[0 as libc::c_int as usize] =
        *start.offset(0 as libc::c_int as isize) +
            frac *
                (*end.offset(0 as libc::c_int as isize) -
                     *start.offset(0 as libc::c_int as isize));
    mid[1 as libc::c_int as usize] =
        *start.offset(1 as libc::c_int as isize) +
            frac *
                (*end.offset(1 as libc::c_int as isize) -
                     *start.offset(1 as libc::c_int as isize));
    mid[2 as libc::c_int as usize] =
        *start.offset(2 as libc::c_int as isize) +
            frac *
                (*end.offset(2 as libc::c_int as isize) -
                     *start.offset(2 as libc::c_int as isize));
    // co down front side
    if SV_RecursiveLightPoint(model, (*node).children[side as usize], start,
                              mid.as_mut_ptr() as *const vec_t) as u64 != 0 {
        return true_0
    } // hit something
    if (back < 0.0f32) as libc::c_int == side {
        return false_0
    } // didn't hit anything
    // check for impact on this node
    surf =
        (*model).surfaces.offset((*node).firstsurface as libc::c_int as
                                     isize); // no lightmaps
    i = 0 as libc::c_int;
    while i < (*node).numsurfaces as libc::c_int {
        let mut smax: libc::c_int = 0;
        let mut tmax: libc::c_int = 0;
        tex = (*surf).texinfo;
        info = (*surf).info;
        if !((*surf).flags as libc::c_uint &
                 (1 as libc::c_uint) << 5 as libc::c_int != 0) {
            s =
                mid[0 as libc::c_int as usize] *
                    (*info).lmvecs[0 as libc::c_int as
                                       usize][0 as libc::c_int as usize] +
                    mid[1 as libc::c_int as usize] *
                        (*info).lmvecs[0 as libc::c_int as
                                           usize][1 as libc::c_int as usize] +
                    mid[2 as libc::c_int as usize] *
                        (*info).lmvecs[0 as libc::c_int as
                                           usize][2 as libc::c_int as usize] +
                    (*info).lmvecs[0 as libc::c_int as
                                       usize][3 as libc::c_int as usize];
            t =
                mid[0 as libc::c_int as usize] *
                    (*info).lmvecs[1 as libc::c_int as
                                       usize][0 as libc::c_int as usize] +
                    mid[1 as libc::c_int as usize] *
                        (*info).lmvecs[1 as libc::c_int as
                                           usize][1 as libc::c_int as usize] +
                    mid[2 as libc::c_int as usize] *
                        (*info).lmvecs[1 as libc::c_int as
                                           usize][2 as libc::c_int as usize] +
                    (*info).lmvecs[1 as libc::c_int as
                                       usize][3 as libc::c_int as usize];
            if !(s <
                     (*info).lightmapmins[0 as libc::c_int as usize] as
                         libc::c_int as libc::c_float ||
                     t <
                         (*info).lightmapmins[1 as libc::c_int as usize] as
                             libc::c_int as libc::c_float) {
                ds =
                    s -
                        (*info).lightmapmins[0 as libc::c_int as usize] as
                            libc::c_int as libc::c_float;
                dt =
                    t -
                        (*info).lightmapmins[1 as libc::c_int as usize] as
                            libc::c_int as libc::c_float;
                if !(ds >
                         (*info).lightextents[0 as libc::c_int as usize] as
                             libc::c_int as libc::c_float ||
                         dt >
                             (*info).lightextents[1 as libc::c_int as usize]
                                 as libc::c_int as libc::c_float) {
                    if (*surf).samples.is_null() { return true_0 }
                    sample_size = Mod_SampleSizeForFace(surf);
                    smax =
                        (*info).lightextents[0 as libc::c_int as usize] as
                            libc::c_int / sample_size + 1 as libc::c_int;
                    tmax =
                        (*info).lightextents[1 as libc::c_int as usize] as
                            libc::c_int / sample_size + 1 as libc::c_int;
                    ds /= sample_size as libc::c_float;
                    dt /= sample_size as libc::c_float;
                    sv_pointColor[2 as libc::c_int as usize] =
                        0 as libc::c_int as vec_t;
                    sv_pointColor[1 as libc::c_int as usize] =
                        sv_pointColor[2 as libc::c_int as usize];
                    sv_pointColor[0 as libc::c_int as usize] =
                        sv_pointColor[1 as libc::c_int as usize];
                    lm =
                        (*surf).samples.offset(((if dt < 0.0f32 {
                                                     (dt - 0.5f32) as
                                                         libc::c_int
                                                 } else {
                                                     (dt + 0.5f32) as
                                                         libc::c_int
                                                 }) * smax) as
                                                   isize).offset((if ds <
                                                                         0.0f32
                                                                     {
                                                                      (ds -
                                                                           0.5f32)
                                                                          as
                                                                          libc::c_int
                                                                  } else {
                                                                      (ds +
                                                                           0.5f32)
                                                                          as
                                                                          libc::c_int
                                                                  }) as
                                                                     isize);
                    size = smax * tmax;
                    map = 0 as libc::c_int;
                    while map < 4 as libc::c_int &&
                              (*surf).styles[map as usize] as libc::c_int !=
                                  255 as libc::c_int {
                        scale =
                            sv.lightstyles[(*surf).styles[map as usize] as
                                               usize].value;
                        sv_pointColor[0 as libc::c_int as usize] +=
                            (*lm).r as libc::c_int as libc::c_float * scale;
                        sv_pointColor[1 as libc::c_int as usize] +=
                            (*lm).g as libc::c_int as libc::c_float * scale;
                        sv_pointColor[2 as libc::c_int as usize] +=
                            (*lm).b as libc::c_int as libc::c_float * scale;
                        lm = lm.offset(size as isize);
                        map += 1
                        // skip to next lightmap
                    }
                    return true_0
                }
            }
        }
        i += 1;
        surf = surf.offset(1)
    }
    // go down back side
    return SV_RecursiveLightPoint(model,
                                  (*node).children[(side == 0) as libc::c_int
                                                       as usize],
                                  mid.as_mut_ptr() as *const vec_t, end);
}
#[no_mangle]
pub unsafe extern "C" fn SV_RunLightStyles() {
    let mut i: libc::c_int = 0;
    let mut ofs: libc::c_int = 0;
    let mut ls: *mut lightstyle_t = 0 as *mut lightstyle_t;
    let mut scale: libc::c_float = 0.;
    scale = (*sv_lighting_modulate).value;
    // run lightstyles animation
    i = 0 as libc::c_int; // disable this light
    ls = sv.lightstyles.as_mut_ptr();
    while i < 64 as libc::c_int {
        (*ls).time += sv.frametime;
        ofs =
            ((*ls).time * 10 as libc::c_int as libc::c_float) as libc::c_int;
        if (*ls).length == 0 as libc::c_int {
            (*ls).value = scale
        } else if (*ls).length == 1 as libc::c_int {
            (*ls).value =
                (*ls).map[0 as libc::c_int as usize] / 12.0f32 * scale
        } else {
            (*ls).value =
                (*ls).map[(ofs % (*ls).length) as usize] / 12.0f32 * scale
        }
        i += 1;
        ls = ls.offset(1)
    };
}
/*
==================
SV_SetLightStyle

needs to get correct working SV_LightPoint
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_SetLightStyle(mut style: libc::c_int,
                                          mut s: *const libc::c_char,
                                          mut f: libc::c_float) {
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    Q_strncpy(sv.lightstyles[style as usize].pattern.as_mut_ptr(), s,
              ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong);
    sv.lightstyles[style as usize].time = f;
    j = Q_strlen(s) as libc::c_int;
    sv.lightstyles[style as usize].length = j;
    k = 0 as libc::c_int;
    while k < j {
        sv.lightstyles[style as usize].map[k as usize] =
            (*s.offset(k as isize) as libc::c_int - 'a' as i32) as
                libc::c_float;
        k += 1
    }
    if sv.state as libc::c_uint != ss_active as libc::c_int as libc::c_uint {
        return
    }
    // tell the clients about changed lightstyle
    MSG_WriteCmdExt(&mut sv.reliable_datagram, 12 as libc::c_int, NS_SERVER,
                    0 as *const libc::c_char);
    MSG_WriteByte(&mut sv.reliable_datagram, style);
    MSG_WriteString(&mut sv.reliable_datagram,
                    sv.lightstyles[style as usize].pattern.as_mut_ptr());
    MSG_WriteFloat(&mut sv.reliable_datagram,
                   sv.lightstyles[style as usize].time);
}
/*
==================
SV_GetLightStyle

needs to get correct working SV_LightPoint
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_GetLightStyle(mut style: libc::c_int)
 -> *const libc::c_char {
    if style < 0 as libc::c_int { style = 0 as libc::c_int }
    if style >= 64 as libc::c_int {
        Host_Error(b"SV_GetLightStyle: style: %i >= %d\x00" as *const u8 as
                       *const libc::c_char, style, 64 as libc::c_int);
    }
    return sv.lightstyles[style as usize].pattern.as_mut_ptr();
}
/*
==================
SV_LightForEntity

grab the ambient lighting color for current point
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_LightForEntity(mut pEdict: *mut edict_t)
 -> libc::c_int {
    let mut start: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    if (*pEdict).v.effects as libc::c_uint &
           (1 as libc::c_uint) << 27 as libc::c_int != 0 ||
           (*sv.worldmodel).lightdata.is_null() {
        return 255 as libc::c_int
    }
    // player has more precision light level that come from client-side
    if (*pEdict).v.flags as libc::c_uint &
           (1 as libc::c_uint) << 3 as libc::c_int != 0 {
        return (*pEdict).v.light_level
    }
    start[0 as libc::c_int as usize] =
        (*pEdict).v.origin[0 as libc::c_int as usize];
    start[1 as libc::c_int as usize] =
        (*pEdict).v.origin[1 as libc::c_int as usize];
    start[2 as libc::c_int as usize] =
        (*pEdict).v.origin[2 as libc::c_int as usize];
    end[0 as libc::c_int as usize] =
        (*pEdict).v.origin[0 as libc::c_int as usize];
    end[1 as libc::c_int as usize] =
        (*pEdict).v.origin[1 as libc::c_int as usize];
    end[2 as libc::c_int as usize] =
        (*pEdict).v.origin[2 as libc::c_int as usize];
    if (*pEdict).v.effects & 16 as libc::c_int != 0 {
        end[2 as libc::c_int as usize] =
            start[2 as libc::c_int as usize] +
                world.size[2 as libc::c_int as usize]
    } else {
        end[2 as libc::c_int as usize] =
            start[2 as libc::c_int as usize] -
                world.size[2 as libc::c_int as usize]
    }
    sv_pointColor[0 as libc::c_int as usize] = 1.0f32;
    sv_pointColor[1 as libc::c_int as usize] = 1.0f32;
    sv_pointColor[2 as libc::c_int as usize] = 1.0f32;
    SV_RecursiveLightPoint(sv.worldmodel, (*sv.worldmodel).nodes,
                           start.as_mut_ptr() as *const vec_t,
                           end.as_mut_ptr() as *const vec_t);
    return ((sv_pointColor[0 as libc::c_int as usize] +
                 sv_pointColor[1 as libc::c_int as usize] +
                 sv_pointColor[2 as libc::c_int as usize]) /
                3 as libc::c_int as libc::c_float) as libc::c_int;
}
