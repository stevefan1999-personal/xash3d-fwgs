#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type grasshdr_s;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    fn Host_IsLocalClient() -> qboolean;
    #[no_mangle]
    fn Host_Error(error: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_Reportf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn SV_ClipPMoveToEntity(pe: *mut physent_s, start: *const vec_t,
                            mins: *mut vec_t, maxs: *mut vec_t,
                            end: *const vec_t, tr: *mut pmtrace_s);
    #[no_mangle]
    fn CL_ClipPMoveToEntity(pe: *mut physent_s, start: *const vec_t,
                            mins: *mut vec_t, maxs: *mut vec_t,
                            end: *const vec_t, tr: *mut pmtrace_s);
    #[no_mangle]
    fn Matrix4x4_TransformPositivePlane(in_0: *const [vec_t; 4],
                                        normal: *const vec_t,
                                        d: libc::c_float, out: *mut vec_t,
                                        dist: *mut libc::c_float);
    #[no_mangle]
    fn Matrix4x4_CreateFromEntity(out: *mut [vec_t; 4], angles: *const vec_t,
                                  origin: *const vec_t, scale: libc::c_float);
    #[no_mangle]
    fn Matrix4x4_VectorITransform(in_0: *const [vec_t; 4],
                                  v: *const libc::c_float,
                                  out: *mut libc::c_float);
    #[no_mangle]
    fn Mod_HullForStudio(m: *mut model_t, frame: libc::c_float,
                         seq: libc::c_int, ang: *mut vec_t, org: *mut vec_t,
                         size: *mut vec_t, pcnt: *mut byte, pbl: *mut byte,
                         hitboxes: *mut libc::c_int, ed: *mut edict_t)
     -> *mut hull_t;
    #[no_mangle]
    fn Mod_HitgroupForStudioHull(index: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn World_TransformAABB(transform: *mut [vec_t; 4], mins: *const vec_t,
                           maxs: *const vec_t, outmins: *mut vec_t,
                           outmaxs: *mut vec_t);
    #[no_mangle]
    fn RankForContents(contents: libc::c_int) -> libc::c_int;
}
pub type __uint32_t = libc::c_uint;
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
pub type C2RustUnnamed = libc::c_uint;
pub const kRenderTransAdd: C2RustUnnamed = 5;
pub const kRenderTransAlpha: C2RustUnnamed = 4;
pub const kRenderGlow: C2RustUnnamed = 3;
pub const kRenderTransTexture: C2RustUnnamed = 2;
pub const kRenderTransColor: C2RustUnnamed = 1;
pub const kRenderNormal: C2RustUnnamed = 0;
pub type string_t = libc::c_int;
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
pub type playermove_t = playermove_s;
pub type pfnIgnore
    =
    Option<unsafe extern "C" fn(_: *mut physent_t) -> libc::c_int>;
static mut pm_boxplanes: [mplane_t; 6] =
    [mplane_t{normal: [0.; 3],
              dist: 0.,
              type_0: 0,
              signbits: 0,
              pad: [0; 2],}; 6];
static mut pm_boxclipnodes: [mclipnode_t; 6] =
    [mclipnode_t{planenum: 0, children: [0; 2],}; 6];
static mut pm_boxhull: hull_t =
    hull_t{clipnodes: 0 as *const mclipnode_t as *mut mclipnode_t,
           planes: 0 as *const mplane_t as *mut mplane_t,
           firstclipnode: 0,
           lastclipnode: 0,
           clip_mins: [0.; 3],
           clip_maxs: [0.; 3],};
// default hullmins
static mut pm_hullmins: [vec3_t; 4] =
    [[-(16 as libc::c_int) as vec_t, -(16 as libc::c_int) as vec_t,
      -(36 as libc::c_int) as vec_t],
     [-(16 as libc::c_int) as vec_t, -(16 as libc::c_int) as vec_t,
      -(18 as libc::c_int) as vec_t],
     [0 as libc::c_int as vec_t, 0 as libc::c_int as vec_t,
      0 as libc::c_int as vec_t],
     [-(32 as libc::c_int) as vec_t, -(32 as libc::c_int) as vec_t,
      -(32 as libc::c_int) as vec_t]];
// defualt hullmaxs
static mut pm_hullmaxs: [vec3_t; 4] =
    [[16 as libc::c_int as vec_t, 16 as libc::c_int as vec_t,
      36 as libc::c_int as vec_t],
     [16 as libc::c_int as vec_t, 16 as libc::c_int as vec_t,
      18 as libc::c_int as vec_t],
     [0 as libc::c_int as vec_t, 0 as libc::c_int as vec_t,
      0 as libc::c_int as vec_t],
     [32 as libc::c_int as vec_t, 32 as libc::c_int as vec_t,
      32 as libc::c_int as vec_t]];
#[no_mangle]
pub unsafe extern "C" fn Pmove_Init() {
    PM_InitBoxHull();
    // init default hull sizes
    memcpy(host.player_mins.as_mut_ptr() as *mut libc::c_void,
           pm_hullmins.as_ptr() as *const libc::c_void,
           ::std::mem::size_of::<[vec3_t; 4]>() as libc::c_ulong);
    memcpy(host.player_maxs.as_mut_ptr() as *mut libc::c_void,
           pm_hullmaxs.as_ptr() as *const libc::c_void,
           ::std::mem::size_of::<[vec3_t; 4]>() as libc::c_ulong);
}
/*
===================
PM_InitBoxHull

Set up the planes and clipnodes so that the six floats of a bounding box
can just be stored out and get a proper hull_t structure.
===================
*/
#[no_mangle]
pub unsafe extern "C" fn PM_InitBoxHull() {
    let mut i: libc::c_int = 0;
    let mut side: libc::c_int = 0;
    pm_boxhull.clipnodes = pm_boxclipnodes.as_mut_ptr();
    pm_boxhull.planes = pm_boxplanes.as_mut_ptr();
    pm_boxhull.firstclipnode = 0 as libc::c_int;
    pm_boxhull.lastclipnode = 5 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        pm_boxclipnodes[i as usize].planenum = i;
        side = i & 1 as libc::c_int;
        pm_boxclipnodes[i as usize].children[side as usize] =
            -(1 as libc::c_int) as libc::c_short;
        if i != 5 as libc::c_int {
            pm_boxclipnodes[i as
                                usize].children[(side ^ 1 as libc::c_int) as
                                                    usize] =
                (i + 1 as libc::c_int) as libc::c_short
        } else {
            pm_boxclipnodes[i as
                                usize].children[(side ^ 1 as libc::c_int) as
                                                    usize] =
                -(2 as libc::c_int) as libc::c_short
        }
        pm_boxplanes[i as usize].type_0 = (i >> 1 as libc::c_int) as byte;
        pm_boxplanes[i as usize].normal[(i >> 1 as libc::c_int) as usize] =
            1.0f32;
        pm_boxplanes[i as usize].signbits = 0 as libc::c_int as byte;
        i += 1
    };
}
/*
===================
PM_HullForBox

To keep everything totally uniform, bounding boxes are turned into small
BSP trees instead of being compared directly.
===================
*/
#[no_mangle]
pub unsafe extern "C" fn PM_HullForBox(mut mins: *const vec_t,
                                       mut maxs: *const vec_t)
 -> *mut hull_t {
    pm_boxplanes[0 as libc::c_int as usize].dist =
        *maxs.offset(0 as libc::c_int as isize); // matched
    pm_boxplanes[1 as libc::c_int as usize].dist =
        *mins.offset(0 as libc::c_int as isize);
    pm_boxplanes[2 as libc::c_int as usize].dist =
        *maxs.offset(1 as libc::c_int as isize);
    pm_boxplanes[3 as libc::c_int as usize].dist =
        *mins.offset(1 as libc::c_int as isize);
    pm_boxplanes[4 as libc::c_int as usize].dist =
        *maxs.offset(2 as libc::c_int as isize);
    pm_boxplanes[5 as libc::c_int as usize].dist =
        *mins.offset(2 as libc::c_int as isize);
    return &mut pm_boxhull;
}
#[no_mangle]
pub unsafe extern "C" fn PM_ConvertTrace(mut out: *mut trace_t,
                                         mut in_0: *mut pmtrace_t,
                                         mut ent: *mut edict_t) {
    memcpy(out as *mut libc::c_void, in_0 as *const libc::c_void,
           48 as libc::c_int as libc::c_ulong);
    (*out).hitgroup = (*in_0).hitgroup;
    (*out).ent = ent;
}
/*
==================
PM_HullPointContents

==================
*/
#[no_mangle]
pub unsafe extern "C" fn PM_HullPointContents(mut hull: *mut hull_t,
                                              mut num: libc::c_int,
                                              mut p: *const vec_t)
 -> libc::c_int {
    let mut plane: *mut mplane_t = 0 as *mut mplane_t;
    if hull.is_null() || (*hull).planes.is_null() {
        // fantom bmodels?
        return 0 as libc::c_int
    }
    while num >= 0 as libc::c_int {
        plane =
            &mut *(*hull).planes.offset((*(*hull).clipnodes.offset(num as
                                                                       isize)).planenum
                                            as isize) as *mut mplane_t;
        num =
            (*(*hull).clipnodes.offset(num as
                                           isize)).children[((if ((*plane).type_0
                                                                      as
                                                                      libc::c_int)
                                                                     <
                                                                     3 as
                                                                         libc::c_int
                                                                 {
                                                                  *p.offset((*plane).type_0
                                                                                as
                                                                                isize)
                                                              } else {
                                                                  (*p.offset(0
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 isize)
                                                                       *
                                                                       (*plane).normal[0
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           usize]
                                                                       +
                                                                       *p.offset(1
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)
                                                                           *
                                                                           (*plane).normal[1
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               usize])
                                                                      +
                                                                      *p.offset(2
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize)
                                                                          *
                                                                          (*plane).normal[2
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              usize]
                                                              }) -
                                                                 (*plane).dist
                                                                 <
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_float)
                                                                as libc::c_int
                                                                as usize] as
                libc::c_int
    }
    return num;
}
/*
==================
PM_HullForBsp

assume physent is valid
==================
*/
#[no_mangle]
pub unsafe extern "C" fn PM_HullForBsp(mut pe: *mut physent_t,
                                       mut pmove: *mut playermove_t,
                                       mut offset: *mut libc::c_float)
 -> *mut hull_t {
    let mut hull: *mut hull_t = 0 as *mut hull_t;
    match (*pmove).usehull {
        1 => {
            hull =
                &mut *(*(*pe).model).hulls.as_mut_ptr().offset(3 as
                                                                   libc::c_int
                                                                   as isize)
                    as *mut hull_t
        }
        2 => {
            hull =
                &mut *(*(*pe).model).hulls.as_mut_ptr().offset(0 as
                                                                   libc::c_int
                                                                   as isize)
                    as *mut hull_t
        }
        3 => {
            hull =
                &mut *(*(*pe).model).hulls.as_mut_ptr().offset(2 as
                                                                   libc::c_int
                                                                   as isize)
                    as *mut hull_t
        }
        _ => {
            hull =
                &mut *(*(*pe).model).hulls.as_mut_ptr().offset(1 as
                                                                   libc::c_int
                                                                   as isize)
                    as *mut hull_t
        }
    }
    // calculate an offset value to center the origin
    *offset.offset(0 as libc::c_int as isize) =
        (*hull).clip_mins[0 as libc::c_int as usize] -
            (*pmove).player_mins[(*pmove).usehull as
                                     usize][0 as libc::c_int as usize];
    *offset.offset(1 as libc::c_int as isize) =
        (*hull).clip_mins[1 as libc::c_int as usize] -
            (*pmove).player_mins[(*pmove).usehull as
                                     usize][1 as libc::c_int as usize];
    *offset.offset(2 as libc::c_int as isize) =
        (*hull).clip_mins[2 as libc::c_int as usize] -
            (*pmove).player_mins[(*pmove).usehull as
                                     usize][2 as libc::c_int as usize];
    *offset.offset(0 as libc::c_int as isize) =
        *offset.offset(0 as libc::c_int as isize) +
            (*pe).origin[0 as libc::c_int as usize];
    *offset.offset(1 as libc::c_int as isize) =
        *offset.offset(1 as libc::c_int as isize) +
            (*pe).origin[1 as libc::c_int as usize];
    *offset.offset(2 as libc::c_int as isize) =
        *offset.offset(2 as libc::c_int as isize) +
            (*pe).origin[2 as libc::c_int as usize];
    return hull;
}
/*
==================
PM_HullForStudio

generate multiple hulls as hitboxes
==================
*/
#[no_mangle]
pub unsafe extern "C" fn PM_HullForStudio(mut pe: *mut physent_t,
                                          mut pmove: *mut playermove_t,
                                          mut numhitboxes: *mut libc::c_int)
 -> *mut hull_t {
    let mut size: vec3_t = [0.; 3];
    size[0 as libc::c_int as usize] =
        (*pmove).player_maxs[(*pmove).usehull as
                                 usize][0 as libc::c_int as usize] -
            (*pmove).player_mins[(*pmove).usehull as
                                     usize][0 as libc::c_int as usize];
    size[1 as libc::c_int as usize] =
        (*pmove).player_maxs[(*pmove).usehull as
                                 usize][1 as libc::c_int as usize] -
            (*pmove).player_mins[(*pmove).usehull as
                                     usize][1 as libc::c_int as usize];
    size[2 as libc::c_int as usize] =
        (*pmove).player_maxs[(*pmove).usehull as
                                 usize][2 as libc::c_int as usize] -
            (*pmove).player_mins[(*pmove).usehull as
                                     usize][2 as libc::c_int as usize];
    size[0 as libc::c_int as usize] =
        size[0 as libc::c_int as usize] * 0.5f32;
    size[1 as libc::c_int as usize] =
        size[1 as libc::c_int as usize] * 0.5f32;
    size[2 as libc::c_int as usize] =
        size[2 as libc::c_int as usize] * 0.5f32;
    return Mod_HullForStudio((*pe).studiomodel, (*pe).frame, (*pe).sequence,
                             (*pe).angles.as_mut_ptr(),
                             (*pe).origin.as_mut_ptr(), size.as_mut_ptr(),
                             (*pe).controller.as_mut_ptr(),
                             (*pe).blending.as_mut_ptr(), numhitboxes,
                             0 as *mut edict_t);
}
/*
==================
PM_RecursiveHullCheck
==================
*/
#[no_mangle]
pub unsafe extern "C" fn PM_RecursiveHullCheck(mut hull: *mut hull_t,
                                               mut num: libc::c_int,
                                               mut p1f: libc::c_float,
                                               mut p2f: libc::c_float,
                                               mut p1: *mut vec_t,
                                               mut p2: *mut vec_t,
                                               mut trace: *mut pmtrace_t)
 -> qboolean {
    let mut node: *mut mclipnode_t = 0 as *mut mclipnode_t;
    let mut plane: *mut mplane_t = 0 as *mut mplane_t;
    let mut t1: libc::c_float = 0.;
    let mut t2: libc::c_float = 0.;
    let mut frac: libc::c_float = 0.;
    let mut midf: libc::c_float = 0.;
    let mut side: libc::c_int = 0;
    let mut mid: vec3_t = [0.; 3];
    loop  {
        // check for empty
        if num < 0 as libc::c_int {
            if num != -(2 as libc::c_int) {
                (*trace).allsolid = false_0;
                if num == -(1 as libc::c_int) {
                    (*trace).inopen = true_0
                } else { (*trace).inwater = true_0 }
            } else { (*trace).startsolid = true_0 }
            return true_0
            // empty
        }
        if (*hull).firstclipnode >= (*hull).lastclipnode {
            // empty hull?
            (*trace).allsolid = false_0;
            (*trace).inopen = true_0;
            return true_0
        }
        if num < (*hull).firstclipnode || num > (*hull).lastclipnode {
            Host_Error(b"PM_RecursiveHullCheck: bad node number %i\n\x00" as
                           *const u8 as *const libc::c_char, num);
        }
        // find the point distances
        node = (*hull).clipnodes.offset(num as isize);
        plane = (*hull).planes.offset((*node).planenum as isize);
        t1 =
            (if ((*plane).type_0 as libc::c_int) < 3 as libc::c_int {
                 *p1.offset((*plane).type_0 as isize)
             } else {
                 (*p1.offset(0 as libc::c_int as isize) *
                      (*plane).normal[0 as libc::c_int as usize] +
                      *p1.offset(1 as libc::c_int as isize) *
                          (*plane).normal[1 as libc::c_int as usize]) +
                     *p1.offset(2 as libc::c_int as isize) *
                         (*plane).normal[2 as libc::c_int as usize]
             }) - (*plane).dist;
        t2 =
            (if ((*plane).type_0 as libc::c_int) < 3 as libc::c_int {
                 *p2.offset((*plane).type_0 as isize)
             } else {
                 (*p2.offset(0 as libc::c_int as isize) *
                      (*plane).normal[0 as libc::c_int as usize] +
                      *p2.offset(1 as libc::c_int as isize) *
                          (*plane).normal[1 as libc::c_int as usize]) +
                     *p2.offset(2 as libc::c_int as isize) *
                         (*plane).normal[2 as libc::c_int as usize]
             }) - (*plane).dist;
        if t1 >= 0.0f32 && t2 >= 0.0f32 {
            num = (*node).children[0 as libc::c_int as usize] as libc::c_int
        } else {
            if !(t1 < 0.0f32 && t2 < 0.0f32) { break ; }
            num = (*node).children[1 as libc::c_int as usize] as libc::c_int
        }
    }
    // put the crosspoint DIST_EPSILON pixels on the near side
    side = (t1 < 0.0f32) as libc::c_int;
    if side != 0 {
        frac = (t1 + 1.0f32 / 32.0f32) / (t1 - t2)
    } else { frac = (t1 - 1.0f32 / 32.0f32) / (t1 - t2) }
    if frac < 0.0f32 { frac = 0.0f32 }
    if frac > 1.0f32 { frac = 1.0f32 }
    midf = p1f + (p2f - p1f) * frac;
    mid[0 as libc::c_int as usize] =
        *p1.offset(0 as libc::c_int as isize) +
            frac *
                (*p2.offset(0 as libc::c_int as isize) -
                     *p1.offset(0 as libc::c_int as isize));
    mid[1 as libc::c_int as usize] =
        *p1.offset(1 as libc::c_int as isize) +
            frac *
                (*p2.offset(1 as libc::c_int as isize) -
                     *p1.offset(1 as libc::c_int as isize));
    mid[2 as libc::c_int as usize] =
        *p1.offset(2 as libc::c_int as isize) +
            frac *
                (*p2.offset(2 as libc::c_int as isize) -
                     *p1.offset(2 as libc::c_int as isize));
    // move up to the node
    if PM_RecursiveHullCheck(hull,
                             (*node).children[side as usize] as libc::c_int,
                             p1f, midf, p1, mid.as_mut_ptr(), trace) as u64 ==
           0 {
        return false_0
    }
    // this recursion can not be optimized because mid would need to be duplicated on a stack
    if PM_HullPointContents(hull,
                            (*node).children[(side ^ 1 as libc::c_int) as
                                                 usize] as libc::c_int,
                            mid.as_mut_ptr() as *const vec_t) !=
           -(2 as libc::c_int) {
        // go past the node
        return PM_RecursiveHullCheck(hull,
                                     (*node).children[(side ^
                                                           1 as libc::c_int)
                                                          as usize] as
                                         libc::c_int, midf, p2f,
                                     mid.as_mut_ptr(), p2, trace)
    }
    // never got out of the solid area
    if (*trace).allsolid as u64 != 0 { return false_0 }
    // the other side of the node is solid, this is the impact point
    if side == 0 {
        (*trace).plane.normal[0 as libc::c_int as usize] =
            (*plane).normal[0 as libc::c_int as usize];
        (*trace).plane.normal[1 as libc::c_int as usize] =
            (*plane).normal[1 as libc::c_int as usize];
        (*trace).plane.normal[2 as libc::c_int as usize] =
            (*plane).normal[2 as libc::c_int as usize];
        (*trace).plane.dist = (*plane).dist
    } else {
        (*trace).plane.normal[0 as libc::c_int as usize] =
            -(*plane).normal[0 as libc::c_int as usize];
        (*trace).plane.normal[1 as libc::c_int as usize] =
            -(*plane).normal[1 as libc::c_int as usize];
        (*trace).plane.normal[2 as libc::c_int as usize] =
            -(*plane).normal[2 as libc::c_int as usize];
        (*trace).plane.dist = -(*plane).dist
    }
    while PM_HullPointContents(hull, (*hull).firstclipnode,
                               mid.as_mut_ptr() as *const vec_t) ==
              -(2 as libc::c_int) {
        // shouldn't really happen, but does occasionally
        frac -= 0.1f32;
        if frac < 0.0f32 {
            (*trace).fraction = midf;
            (*trace).endpos[0 as libc::c_int as usize] =
                mid[0 as libc::c_int as usize];
            (*trace).endpos[1 as libc::c_int as usize] =
                mid[1 as libc::c_int as usize];
            (*trace).endpos[2 as libc::c_int as usize] =
                mid[2 as libc::c_int as usize];
            Con_Reportf(b"^3Warning:^7 trace backed up past 0.0\n\x00" as
                            *const u8 as *const libc::c_char);
            return false_0
        }
        midf = p1f + (p2f - p1f) * frac;
        mid[0 as libc::c_int as usize] =
            *p1.offset(0 as libc::c_int as isize) +
                frac *
                    (*p2.offset(0 as libc::c_int as isize) -
                         *p1.offset(0 as libc::c_int as isize));
        mid[1 as libc::c_int as usize] =
            *p1.offset(1 as libc::c_int as isize) +
                frac *
                    (*p2.offset(1 as libc::c_int as isize) -
                         *p1.offset(1 as libc::c_int as isize));
        mid[2 as libc::c_int as usize] =
            *p1.offset(2 as libc::c_int as isize) +
                frac *
                    (*p2.offset(2 as libc::c_int as isize) -
                         *p1.offset(2 as libc::c_int as isize))
    }
    (*trace).fraction = midf;
    (*trace).endpos[0 as libc::c_int as usize] =
        mid[0 as libc::c_int as usize];
    (*trace).endpos[1 as libc::c_int as usize] =
        mid[1 as libc::c_int as usize];
    (*trace).endpos[2 as libc::c_int as usize] =
        mid[2 as libc::c_int as usize];
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn PM_PlayerTraceExt(mut pmove: *mut playermove_t,
                                           mut start: *mut vec_t,
                                           mut end: *mut vec_t,
                                           mut flags: libc::c_int,
                                           mut numents: libc::c_int,
                                           mut ents: *mut physent_t,
                                           mut ignore_pe: libc::c_int,
                                           mut pmFilter: pfnIgnore)
 -> pmtrace_t {
    let mut pe: *mut physent_t = 0 as *mut physent_t;
    let mut matrix: matrix4x4 = [[0.; 4]; 4];
    let mut trace_bbox: pmtrace_t =
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
    let mut trace_hitbox: pmtrace_t =
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
    let mut trace_total: pmtrace_t =
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
    let mut offset: vec3_t = [0.; 3];
    let mut start_l: vec3_t = [0.; 3];
    let mut end_l: vec3_t = [0.; 3];
    let mut temp: vec3_t = [0.; 3];
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut hullcount: libc::c_int = 0;
    let mut rotated: qboolean = false_0;
    let mut transform_bbox: qboolean = false_0;
    let mut hull: *mut hull_t = 0 as *mut hull_t;
    memset(&mut trace_total as *mut pmtrace_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<pmtrace_t>() as libc::c_ulong);
    trace_total.endpos[0 as libc::c_int as usize] =
        *end.offset(0 as libc::c_int as isize);
    trace_total.endpos[1 as libc::c_int as usize] =
        *end.offset(1 as libc::c_int as isize);
    trace_total.endpos[2 as libc::c_int as usize] =
        *end.offset(2 as libc::c_int as isize);
    trace_total.fraction = 1.0f32;
    trace_total.ent = -(1 as libc::c_int);
    let mut current_block_108: u64;
    i = 0 as libc::c_int;
    while i < numents {
        pe = &mut *ents.offset(i as isize) as *mut physent_t;
        if i != 0 as libc::c_int && flags & 0x8 as libc::c_int != 0 {
            break ;
        }
        // run custom user filter
        if pmFilter.is_some() {
            if pmFilter.expect("non-null function pointer")(pe) != 0 {
                current_block_108 =
                    11812396948646013369; // calc new local offset
            } else { current_block_108 = 15904375183555213903; }
        } else if ignore_pe != -(1 as libc::c_int) {
            if i == ignore_pe {
                current_block_108 = 11812396948646013369;
            } else { current_block_108 = 15904375183555213903; }
        } else { current_block_108 = 15904375183555213903; }
        match current_block_108 {
            15904375183555213903 => {
                if !(!(*pe).model.is_null() && (*pe).solid == 0 as libc::c_int
                         && (*pe).skin != 0 as libc::c_int) {
                    if !(flags & 0x4 as libc::c_int != 0 &&
                             (*pe).rendermode != kRenderNormal as libc::c_int)
                       {
                        if !(flags & 0x10 as libc::c_int != 0 &&
                                 (*pe).solid == 5 as libc::c_int) {
                            hullcount = 1 as libc::c_int;
                            if (*pe).solid == 5 as libc::c_int {
                                mins[0 as libc::c_int as usize] =
                                    (*pmove).player_mins[(*pmove).usehull as
                                                             usize][0 as
                                                                        libc::c_int
                                                                        as
                                                                        usize];
                                mins[1 as libc::c_int as usize] =
                                    (*pmove).player_mins[(*pmove).usehull as
                                                             usize][1 as
                                                                        libc::c_int
                                                                        as
                                                                        usize];
                                mins[2 as libc::c_int as usize] =
                                    (*pmove).player_mins[(*pmove).usehull as
                                                             usize][2 as
                                                                        libc::c_int
                                                                        as
                                                                        usize];
                                maxs[0 as libc::c_int as usize] =
                                    (*pmove).player_maxs[(*pmove).usehull as
                                                             usize][0 as
                                                                        libc::c_int
                                                                        as
                                                                        usize];
                                maxs[1 as libc::c_int as usize] =
                                    (*pmove).player_maxs[(*pmove).usehull as
                                                             usize][1 as
                                                                        libc::c_int
                                                                        as
                                                                        usize];
                                maxs[2 as libc::c_int as usize] =
                                    (*pmove).player_maxs[(*pmove).usehull as
                                                             usize][2 as
                                                                        libc::c_int
                                                                        as
                                                                        usize];
                                offset[2 as libc::c_int as usize] =
                                    0 as libc::c_int as vec_t;
                                offset[1 as libc::c_int as usize] =
                                    offset[2 as libc::c_int as usize];
                                offset[0 as libc::c_int as usize] =
                                    offset[1 as libc::c_int as usize];
                                current_block_108 = 16799951812150840583;
                            } else if !(*pe).model.is_null() {
                                hull =
                                    PM_HullForBsp(pe, pmove,
                                                  offset.as_mut_ptr());
                                current_block_108 = 16799951812150840583;
                            } else if !(*pe).studiomodel.is_null() {
                                if flags & 0x1 as libc::c_int != 0 {
                                    current_block_108 = 11812396948646013369;
                                } else {
                                    if !(*pe).studiomodel.is_null() &&
                                           (*(*pe).studiomodel).type_0 as
                                               libc::c_int ==
                                               mod_studio as libc::c_int &&
                                           ((*(*pe).studiomodel).flags as
                                                libc::c_uint &
                                                (1 as libc::c_uint) <<
                                                    9 as libc::c_int != 0 ||
                                                (*pmove).usehull ==
                                                    2 as libc::c_int) &&
                                           flags & 0x2 as libc::c_int == 0 {
                                        hull =
                                            PM_HullForStudio(pe, pmove,
                                                             &mut hullcount);
                                        offset[2 as libc::c_int as usize] =
                                            0 as libc::c_int as vec_t;
                                        offset[1 as libc::c_int as usize] =
                                            offset[2 as libc::c_int as usize];
                                        offset[0 as libc::c_int as usize] =
                                            offset[1 as libc::c_int as usize]
                                    } else {
                                        mins[0 as libc::c_int as usize] =
                                            (*pe).mins[0 as libc::c_int as
                                                           usize] -
                                                (*pmove).player_maxs[(*pmove).usehull
                                                                         as
                                                                         usize][0
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    usize];
                                        mins[1 as libc::c_int as usize] =
                                            (*pe).mins[1 as libc::c_int as
                                                           usize] -
                                                (*pmove).player_maxs[(*pmove).usehull
                                                                         as
                                                                         usize][1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    usize];
                                        mins[2 as libc::c_int as usize] =
                                            (*pe).mins[2 as libc::c_int as
                                                           usize] -
                                                (*pmove).player_maxs[(*pmove).usehull
                                                                         as
                                                                         usize][2
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    usize];
                                        maxs[0 as libc::c_int as usize] =
                                            (*pe).maxs[0 as libc::c_int as
                                                           usize] -
                                                (*pmove).player_mins[(*pmove).usehull
                                                                         as
                                                                         usize][0
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    usize];
                                        maxs[1 as libc::c_int as usize] =
                                            (*pe).maxs[1 as libc::c_int as
                                                           usize] -
                                                (*pmove).player_mins[(*pmove).usehull
                                                                         as
                                                                         usize][1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    usize];
                                        maxs[2 as libc::c_int as usize] =
                                            (*pe).maxs[2 as libc::c_int as
                                                           usize] -
                                                (*pmove).player_mins[(*pmove).usehull
                                                                         as
                                                                         usize][2
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    usize];
                                        hull =
                                            PM_HullForBox(mins.as_mut_ptr() as
                                                              *const vec_t,
                                                          maxs.as_mut_ptr() as
                                                              *const vec_t);
                                        offset[0 as libc::c_int as usize] =
                                            (*pe).origin[0 as libc::c_int as
                                                             usize];
                                        offset[1 as libc::c_int as usize] =
                                            (*pe).origin[1 as libc::c_int as
                                                             usize];
                                        offset[2 as libc::c_int as usize] =
                                            (*pe).origin[2 as libc::c_int as
                                                             usize]
                                    }
                                    current_block_108 = 16799951812150840583;
                                }
                            } else {
                                mins[0 as libc::c_int as usize] =
                                    (*pe).mins[0 as libc::c_int as usize] -
                                        (*pmove).player_maxs[(*pmove).usehull
                                                                 as
                                                                 usize][0 as
                                                                            libc::c_int
                                                                            as
                                                                            usize];
                                mins[1 as libc::c_int as usize] =
                                    (*pe).mins[1 as libc::c_int as usize] -
                                        (*pmove).player_maxs[(*pmove).usehull
                                                                 as
                                                                 usize][1 as
                                                                            libc::c_int
                                                                            as
                                                                            usize];
                                mins[2 as libc::c_int as usize] =
                                    (*pe).mins[2 as libc::c_int as usize] -
                                        (*pmove).player_maxs[(*pmove).usehull
                                                                 as
                                                                 usize][2 as
                                                                            libc::c_int
                                                                            as
                                                                            usize];
                                maxs[0 as libc::c_int as usize] =
                                    (*pe).maxs[0 as libc::c_int as usize] -
                                        (*pmove).player_mins[(*pmove).usehull
                                                                 as
                                                                 usize][0 as
                                                                            libc::c_int
                                                                            as
                                                                            usize];
                                maxs[1 as libc::c_int as usize] =
                                    (*pe).maxs[1 as libc::c_int as usize] -
                                        (*pmove).player_mins[(*pmove).usehull
                                                                 as
                                                                 usize][1 as
                                                                            libc::c_int
                                                                            as
                                                                            usize];
                                maxs[2 as libc::c_int as usize] =
                                    (*pe).maxs[2 as libc::c_int as usize] -
                                        (*pmove).player_mins[(*pmove).usehull
                                                                 as
                                                                 usize][2 as
                                                                            libc::c_int
                                                                            as
                                                                            usize];
                                hull =
                                    PM_HullForBox(mins.as_mut_ptr() as
                                                      *const vec_t,
                                                  maxs.as_mut_ptr() as
                                                      *const vec_t);
                                offset[0 as libc::c_int as usize] =
                                    (*pe).origin[0 as libc::c_int as usize];
                                offset[1 as libc::c_int as usize] =
                                    (*pe).origin[1 as libc::c_int as usize];
                                offset[2 as libc::c_int as usize] =
                                    (*pe).origin[2 as libc::c_int as usize];
                                current_block_108 = 16799951812150840583;
                            }
                            match current_block_108 {
                                11812396948646013369 => { }
                                _ => {
                                    if (*pe).solid == 4 as libc::c_int &&
                                           !((*pe).angles[0 as libc::c_int as
                                                              usize] == 0.0f32
                                                 &&
                                                 (*pe).angles[1 as libc::c_int
                                                                  as usize] ==
                                                     0.0f32 &&
                                                 (*pe).angles[2 as libc::c_int
                                                                  as usize] ==
                                                     0.0f32) {
                                        rotated = true_0
                                    } else { rotated = false_0 }
                                    if host.features &
                                           ((1 as libc::c_int) <<
                                                3 as libc::c_int) as
                                               libc::c_uint != 0 {
                                        if ((*pe).angles[0 as libc::c_int as
                                                             usize] as
                                                libc::c_int ==
                                                90 as libc::c_int ||
                                                (*pe).angles[0 as libc::c_int
                                                                 as usize] as
                                                    libc::c_int ==
                                                    180 as libc::c_int ||
                                                (*pe).angles[0 as libc::c_int
                                                                 as usize] as
                                                    libc::c_int ==
                                                    270 as libc::c_int ||
                                                (*pe).angles[0 as libc::c_int
                                                                 as usize] as
                                                    libc::c_int ==
                                                    -(90 as libc::c_int) ||
                                                (*pe).angles[0 as libc::c_int
                                                                 as usize] as
                                                    libc::c_int ==
                                                    -(180 as libc::c_int) ||
                                                (*pe).angles[0 as libc::c_int
                                                                 as usize] as
                                                    libc::c_int ==
                                                    -(270 as libc::c_int) ||
                                                ((*pe).angles[2 as libc::c_int
                                                                  as usize] as
                                                     libc::c_int ==
                                                     90 as libc::c_int ||
                                                     (*pe).angles[2 as
                                                                      libc::c_int
                                                                      as
                                                                      usize]
                                                         as libc::c_int ==
                                                         180 as libc::c_int ||
                                                     (*pe).angles[2 as
                                                                      libc::c_int
                                                                      as
                                                                      usize]
                                                         as libc::c_int ==
                                                         270 as libc::c_int ||
                                                     (*pe).angles[2 as
                                                                      libc::c_int
                                                                      as
                                                                      usize]
                                                         as libc::c_int ==
                                                         -(90 as libc::c_int)
                                                     ||
                                                     (*pe).angles[2 as
                                                                      libc::c_int
                                                                      as
                                                                      usize]
                                                         as libc::c_int ==
                                                         -(180 as libc::c_int)
                                                     ||
                                                     (*pe).angles[2 as
                                                                      libc::c_int
                                                                      as
                                                                      usize]
                                                         as libc::c_int ==
                                                         -(270 as
                                                               libc::c_int)))
                                               &&
                                               (*pmove).usehull !=
                                                   2 as libc::c_int {
                                            transform_bbox = true_0
                                        } else { transform_bbox = false_0 }
                                    } else { transform_bbox = false_0 }
                                    if rotated as u64 != 0 {
                                        if transform_bbox as u64 != 0 {
                                            Matrix4x4_CreateFromEntity(matrix.as_mut_ptr(),
                                                                       (*pe).angles.as_mut_ptr()
                                                                           as
                                                                           *const vec_t,
                                                                       (*pe).origin.as_mut_ptr()
                                                                           as
                                                                           *const vec_t,
                                                                       1.0f32);
                                        } else {
                                            Matrix4x4_CreateFromEntity(matrix.as_mut_ptr(),
                                                                       (*pe).angles.as_mut_ptr()
                                                                           as
                                                                           *const vec_t,
                                                                       offset.as_mut_ptr()
                                                                           as
                                                                           *const vec_t,
                                                                       1.0f32);
                                        }
                                        Matrix4x4_VectorITransform(matrix.as_mut_ptr()
                                                                       as
                                                                       *const [vec_t; 4],
                                                                   start as
                                                                       *const libc::c_float,
                                                                   start_l.as_mut_ptr());
                                        Matrix4x4_VectorITransform(matrix.as_mut_ptr()
                                                                       as
                                                                       *const [vec_t; 4],
                                                                   end as
                                                                       *const libc::c_float,
                                                                   end_l.as_mut_ptr());
                                        if transform_bbox as u64 != 0 {
                                            World_TransformAABB(matrix.as_mut_ptr(),
                                                                (*pmove).player_mins[(*pmove).usehull
                                                                                         as
                                                                                         usize].as_mut_ptr()
                                                                    as
                                                                    *const vec_t,
                                                                (*pmove).player_maxs[(*pmove).usehull
                                                                                         as
                                                                                         usize].as_mut_ptr()
                                                                    as
                                                                    *const vec_t,
                                                                mins.as_mut_ptr(),
                                                                maxs.as_mut_ptr());
                                            offset[0 as libc::c_int as usize]
                                                =
                                                (*hull).clip_mins[0 as
                                                                      libc::c_int
                                                                      as
                                                                      usize] -
                                                    mins[0 as libc::c_int as
                                                             usize];
                                            offset[1 as libc::c_int as usize]
                                                =
                                                (*hull).clip_mins[1 as
                                                                      libc::c_int
                                                                      as
                                                                      usize] -
                                                    mins[1 as libc::c_int as
                                                             usize];
                                            offset[2 as libc::c_int as usize]
                                                =
                                                (*hull).clip_mins[2 as
                                                                      libc::c_int
                                                                      as
                                                                      usize] -
                                                    mins[2 as libc::c_int as
                                                             usize];
                                            j = 0 as libc::c_int;
                                            while j < 3 as libc::c_int {
                                                if start_l[j as usize] >=
                                                       0.0f32 {
                                                    start_l[j as usize] -=
                                                        offset[j as usize]
                                                } else {
                                                    start_l[j as usize] +=
                                                        offset[j as usize]
                                                }
                                                if end_l[j as usize] >= 0.0f32
                                                   {
                                                    end_l[j as usize] -=
                                                        offset[j as usize]
                                                } else {
                                                    end_l[j as usize] +=
                                                        offset[j as usize]
                                                }
                                                j += 1
                                            }
                                        }
                                    } else {
                                        start_l[0 as libc::c_int as usize] =
                                            *start.offset(0 as libc::c_int as
                                                              isize) -
                                                offset[0 as libc::c_int as
                                                           usize];
                                        start_l[1 as libc::c_int as usize] =
                                            *start.offset(1 as libc::c_int as
                                                              isize) -
                                                offset[1 as libc::c_int as
                                                           usize];
                                        start_l[2 as libc::c_int as usize] =
                                            *start.offset(2 as libc::c_int as
                                                              isize) -
                                                offset[2 as libc::c_int as
                                                           usize];
                                        end_l[0 as libc::c_int as usize] =
                                            *end.offset(0 as libc::c_int as
                                                            isize) -
                                                offset[0 as libc::c_int as
                                                           usize];
                                        end_l[1 as libc::c_int as usize] =
                                            *end.offset(1 as libc::c_int as
                                                            isize) -
                                                offset[1 as libc::c_int as
                                                           usize];
                                        end_l[2 as libc::c_int as usize] =
                                            *end.offset(2 as libc::c_int as
                                                            isize) -
                                                offset[2 as libc::c_int as
                                                           usize]
                                    }
                                    memset(&mut trace_bbox as *mut pmtrace_t
                                               as *mut libc::c_void,
                                           0 as libc::c_int,
                                           ::std::mem::size_of::<pmtrace_t>()
                                               as libc::c_ulong);
                                    trace_bbox.endpos[0 as libc::c_int as
                                                          usize] =
                                        *end.offset(0 as libc::c_int as
                                                        isize);
                                    trace_bbox.endpos[1 as libc::c_int as
                                                          usize] =
                                        *end.offset(1 as libc::c_int as
                                                        isize);
                                    trace_bbox.endpos[2 as libc::c_int as
                                                          usize] =
                                        *end.offset(2 as libc::c_int as
                                                        isize);
                                    trace_bbox.allsolid = true_0;
                                    trace_bbox.fraction = 1.0f32;
                                    if hullcount < 1 as libc::c_int {
                                        // g-cont. probably this never happens
                                        trace_bbox.allsolid = false_0
                                    } else if (*pe).solid == 5 as libc::c_int
                                     {
                                        // run custom sweep callback
                                        if (*pmove).server as libc::c_uint !=
                                               0 ||
                                               Host_IsLocalClient() as
                                                   libc::c_uint != 0 {
                                            SV_ClipPMoveToEntity(pe,
                                                                 start as
                                                                     *const vec_t,
                                                                 mins.as_mut_ptr(),
                                                                 maxs.as_mut_ptr(),
                                                                 end as
                                                                     *const vec_t,
                                                                 &mut trace_bbox);
                                        } else {
                                            CL_ClipPMoveToEntity(pe,
                                                                 start as
                                                                     *const vec_t,
                                                                 mins.as_mut_ptr(),
                                                                 maxs.as_mut_ptr(),
                                                                 end as
                                                                     *const vec_t,
                                                                 &mut trace_bbox);
                                        }
                                    } else if hullcount == 1 as libc::c_int {
                                        PM_RecursiveHullCheck(hull,
                                                              (*hull).firstclipnode,
                                                              0 as libc::c_int
                                                                  as
                                                                  libc::c_float,
                                                              1 as libc::c_int
                                                                  as
                                                                  libc::c_float,
                                                              start_l.as_mut_ptr(),
                                                              end_l.as_mut_ptr(),
                                                              &mut trace_bbox);
                                    } else {
                                        let mut last_hitgroup: libc::c_int =
                                            0;
                                        last_hitgroup = 0 as libc::c_int;
                                        j = 0 as libc::c_int;
                                        while j < hullcount {
                                            memset(&mut trace_hitbox as
                                                       *mut pmtrace_t as
                                                       *mut libc::c_void,
                                                   0 as libc::c_int,
                                                   ::std::mem::size_of::<pmtrace_t>()
                                                       as libc::c_ulong);
                                            trace_hitbox.endpos[0 as
                                                                    libc::c_int
                                                                    as usize]
                                                =
                                                *end.offset(0 as libc::c_int
                                                                as isize);
                                            trace_hitbox.endpos[1 as
                                                                    libc::c_int
                                                                    as usize]
                                                =
                                                *end.offset(1 as libc::c_int
                                                                as isize);
                                            trace_hitbox.endpos[2 as
                                                                    libc::c_int
                                                                    as usize]
                                                =
                                                *end.offset(2 as libc::c_int
                                                                as isize);
                                            trace_hitbox.allsolid = true_0;
                                            trace_hitbox.fraction = 1.0f32;
                                            PM_RecursiveHullCheck(&mut *hull.offset(j
                                                                                        as
                                                                                        isize),
                                                                  (*hull.offset(j
                                                                                    as
                                                                                    isize)).firstclipnode,
                                                                  0 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_float,
                                                                  1 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_float,
                                                                  start_l.as_mut_ptr(),
                                                                  end_l.as_mut_ptr(),
                                                                  &mut trace_hitbox);
                                            if j == 0 as libc::c_int ||
                                                   trace_hitbox.allsolid as
                                                       libc::c_uint != 0 ||
                                                   trace_hitbox.startsolid as
                                                       libc::c_uint != 0 ||
                                                   trace_hitbox.fraction <
                                                       trace_bbox.fraction {
                                                if trace_bbox.startsolid as
                                                       u64 != 0 {
                                                    trace_bbox = trace_hitbox;
                                                    trace_bbox.startsolid =
                                                        true_0
                                                } else {
                                                    trace_bbox = trace_hitbox
                                                }
                                                last_hitgroup = j
                                            }
                                            j += 1
                                        }
                                        trace_bbox.hitgroup =
                                            Mod_HitgroupForStudioHull(last_hitgroup)
                                    }
                                    if trace_bbox.allsolid as u64 != 0 {
                                        trace_bbox.startsolid = true_0
                                    }
                                    if trace_bbox.startsolid as u64 != 0 {
                                        trace_bbox.fraction = 0.0f32
                                    }
                                    if trace_bbox.startsolid as u64 == 0 {
                                        trace_bbox.endpos[0 as libc::c_int as
                                                              usize] =
                                            *start.offset(0 as libc::c_int as
                                                              isize) +
                                                trace_bbox.fraction *
                                                    (*end.offset(0 as
                                                                     libc::c_int
                                                                     as isize)
                                                         -
                                                         *start.offset(0 as
                                                                           libc::c_int
                                                                           as
                                                                           isize));
                                        trace_bbox.endpos[1 as libc::c_int as
                                                              usize] =
                                            *start.offset(1 as libc::c_int as
                                                              isize) +
                                                trace_bbox.fraction *
                                                    (*end.offset(1 as
                                                                     libc::c_int
                                                                     as isize)
                                                         -
                                                         *start.offset(1 as
                                                                           libc::c_int
                                                                           as
                                                                           isize));
                                        trace_bbox.endpos[2 as libc::c_int as
                                                              usize] =
                                            *start.offset(2 as libc::c_int as
                                                              isize) +
                                                trace_bbox.fraction *
                                                    (*end.offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                                         -
                                                         *start.offset(2 as
                                                                           libc::c_int
                                                                           as
                                                                           isize));
                                        if rotated as u64 != 0 {
                                            temp[0 as libc::c_int as usize] =
                                                trace_bbox.plane.normal[0 as
                                                                            libc::c_int
                                                                            as
                                                                            usize];
                                            temp[1 as libc::c_int as usize] =
                                                trace_bbox.plane.normal[1 as
                                                                            libc::c_int
                                                                            as
                                                                            usize];
                                            temp[2 as libc::c_int as usize] =
                                                trace_bbox.plane.normal[2 as
                                                                            libc::c_int
                                                                            as
                                                                            usize];
                                            Matrix4x4_TransformPositivePlane(matrix.as_mut_ptr()
                                                                                 as
                                                                                 *const [vec_t; 4],
                                                                             temp.as_mut_ptr()
                                                                                 as
                                                                                 *const vec_t,
                                                                             trace_bbox.plane.dist,
                                                                             trace_bbox.plane.normal.as_mut_ptr(),
                                                                             &mut trace_bbox.plane.dist);
                                        } else {
                                            trace_bbox.plane.dist =
                                                trace_bbox.endpos[0 as
                                                                      libc::c_int
                                                                      as
                                                                      usize] *
                                                    trace_bbox.plane.normal[0
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]
                                                    +
                                                    trace_bbox.endpos[1 as
                                                                          libc::c_int
                                                                          as
                                                                          usize]
                                                        *
                                                        trace_bbox.plane.normal[1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    usize]
                                                    +
                                                    trace_bbox.endpos[2 as
                                                                          libc::c_int
                                                                          as
                                                                          usize]
                                                        *
                                                        trace_bbox.plane.normal[2
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    usize]
                                        }
                                    }
                                    if trace_bbox.fraction <
                                           trace_total.fraction {
                                        trace_total = trace_bbox;
                                        trace_total.ent = i
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => { }
        }
        i += 1
    }
    return trace_total;
}
#[no_mangle]
pub unsafe extern "C" fn PM_TestPlayerPosition(mut pmove: *mut playermove_t,
                                               mut pos: *mut vec_t,
                                               mut ptrace: *mut pmtrace_t,
                                               mut pmFilter: pfnIgnore)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut hullcount: libc::c_int = 0;
    let mut pos_l: vec3_t = [0.; 3];
    let mut offset: vec3_t = [0.; 3];
    let mut hull: *mut hull_t = 0 as *mut hull_t;
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
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
    let mut pe: *mut physent_t = 0 as *mut physent_t;
    trace =
        PM_PlayerTraceExt(pmove, (*pmove).origin.as_mut_ptr(),
                          (*pmove).origin.as_mut_ptr(), 0 as libc::c_int,
                          (*pmove).numphysent, (*pmove).physents.as_mut_ptr(),
                          -(1 as libc::c_int), pmFilter);
    if !ptrace.is_null() { *ptrace = trace }
    let mut current_block_65: u64;
    i = 0 as libc::c_int;
    while i < (*pmove).numphysent {
        pe =
            &mut *(*pmove).physents.as_mut_ptr().offset(i as isize) as
                *mut physent_t;
        // run custom user filter
        if pmFilter.is_some() {
            if pmFilter.expect("non-null function pointer")(pe) != 0 {
                current_block_65 = 10879442775620481940;
            } else { current_block_65 = 2968425633554183086; }
        } else { current_block_65 = 2968425633554183086; }
        match current_block_65 {
            2968425633554183086 => {
                if !(!(*pe).model.is_null() && (*pe).solid == 0 as libc::c_int
                         && (*pe).skin != 0 as libc::c_int) {
                    hullcount = 1 as libc::c_int;
                    if (*pe).solid == 5 as libc::c_int {
                        mins[0 as libc::c_int as usize] =
                            (*pmove).player_mins[(*pmove).usehull as
                                                     usize][0 as libc::c_int
                                                                as usize];
                        mins[1 as libc::c_int as usize] =
                            (*pmove).player_mins[(*pmove).usehull as
                                                     usize][1 as libc::c_int
                                                                as usize];
                        mins[2 as libc::c_int as usize] =
                            (*pmove).player_mins[(*pmove).usehull as
                                                     usize][2 as libc::c_int
                                                                as usize];
                        maxs[0 as libc::c_int as usize] =
                            (*pmove).player_maxs[(*pmove).usehull as
                                                     usize][0 as libc::c_int
                                                                as usize];
                        maxs[1 as libc::c_int as usize] =
                            (*pmove).player_maxs[(*pmove).usehull as
                                                     usize][1 as libc::c_int
                                                                as usize];
                        maxs[2 as libc::c_int as usize] =
                            (*pmove).player_maxs[(*pmove).usehull as
                                                     usize][2 as libc::c_int
                                                                as usize];
                        offset[2 as libc::c_int as usize] =
                            0 as libc::c_int as vec_t;
                        offset[1 as libc::c_int as usize] =
                            offset[2 as libc::c_int as usize];
                        offset[0 as libc::c_int as usize] =
                            offset[1 as libc::c_int as usize]
                    } else if !(*pe).model.is_null() {
                        hull = PM_HullForBsp(pe, pmove, offset.as_mut_ptr())
                    } else if !(*pe).studiomodel.is_null() &&
                                  (*(*pe).studiomodel).type_0 as libc::c_int
                                      == mod_studio as libc::c_int &&
                                  ((*(*pe).studiomodel).flags as libc::c_uint
                                       &
                                       (1 as libc::c_uint) << 9 as libc::c_int
                                       != 0 ||
                                       (*pmove).usehull == 2 as libc::c_int) {
                        hull = PM_HullForStudio(pe, pmove, &mut hullcount);
                        offset[2 as libc::c_int as usize] =
                            0 as libc::c_int as vec_t;
                        offset[1 as libc::c_int as usize] =
                            offset[2 as libc::c_int as usize];
                        offset[0 as libc::c_int as usize] =
                            offset[1 as libc::c_int as usize]
                    } else {
                        mins[0 as libc::c_int as usize] =
                            (*pe).mins[0 as libc::c_int as usize] -
                                (*pmove).player_maxs[(*pmove).usehull as
                                                         usize][0 as
                                                                    libc::c_int
                                                                    as usize];
                        mins[1 as libc::c_int as usize] =
                            (*pe).mins[1 as libc::c_int as usize] -
                                (*pmove).player_maxs[(*pmove).usehull as
                                                         usize][1 as
                                                                    libc::c_int
                                                                    as usize];
                        mins[2 as libc::c_int as usize] =
                            (*pe).mins[2 as libc::c_int as usize] -
                                (*pmove).player_maxs[(*pmove).usehull as
                                                         usize][2 as
                                                                    libc::c_int
                                                                    as usize];
                        maxs[0 as libc::c_int as usize] =
                            (*pe).maxs[0 as libc::c_int as usize] -
                                (*pmove).player_mins[(*pmove).usehull as
                                                         usize][0 as
                                                                    libc::c_int
                                                                    as usize];
                        maxs[1 as libc::c_int as usize] =
                            (*pe).maxs[1 as libc::c_int as usize] -
                                (*pmove).player_mins[(*pmove).usehull as
                                                         usize][1 as
                                                                    libc::c_int
                                                                    as usize];
                        maxs[2 as libc::c_int as usize] =
                            (*pe).maxs[2 as libc::c_int as usize] -
                                (*pmove).player_mins[(*pmove).usehull as
                                                         usize][2 as
                                                                    libc::c_int
                                                                    as usize];
                        hull =
                            PM_HullForBox(mins.as_mut_ptr() as *const vec_t,
                                          maxs.as_mut_ptr() as *const vec_t);
                        offset[0 as libc::c_int as usize] =
                            (*pe).origin[0 as libc::c_int as usize];
                        offset[1 as libc::c_int as usize] =
                            (*pe).origin[1 as libc::c_int as usize];
                        offset[2 as libc::c_int as usize] =
                            (*pe).origin[2 as libc::c_int as usize]
                    }
                    // CM_TransformedPointContents :-)
                    if (*pe).solid == 4 as libc::c_int &&
                           !((*pe).angles[0 as libc::c_int as usize] == 0.0f32
                                 &&
                                 (*pe).angles[1 as libc::c_int as usize] ==
                                     0.0f32 &&
                                 (*pe).angles[2 as libc::c_int as usize] ==
                                     0.0f32) {
                        let mut transform_bbox: qboolean =
                            false_0; // calc new local offset
                        let mut matrix: matrix4x4 = [[0.; 4]; 4];
                        if host.features &
                               ((1 as libc::c_int) << 3 as libc::c_int) as
                                   libc::c_uint != 0 {
                            if ((*pe).angles[0 as libc::c_int as usize] as
                                    libc::c_int == 90 as libc::c_int ||
                                    (*pe).angles[0 as libc::c_int as usize] as
                                        libc::c_int == 180 as libc::c_int ||
                                    (*pe).angles[0 as libc::c_int as usize] as
                                        libc::c_int == 270 as libc::c_int ||
                                    (*pe).angles[0 as libc::c_int as usize] as
                                        libc::c_int == -(90 as libc::c_int) ||
                                    (*pe).angles[0 as libc::c_int as usize] as
                                        libc::c_int == -(180 as libc::c_int)
                                    ||
                                    (*pe).angles[0 as libc::c_int as usize] as
                                        libc::c_int == -(270 as libc::c_int)
                                    ||
                                    ((*pe).angles[2 as libc::c_int as usize]
                                         as libc::c_int == 90 as libc::c_int
                                         ||
                                         (*pe).angles[2 as libc::c_int as
                                                          usize] as
                                             libc::c_int == 180 as libc::c_int
                                         ||
                                         (*pe).angles[2 as libc::c_int as
                                                          usize] as
                                             libc::c_int == 270 as libc::c_int
                                         ||
                                         (*pe).angles[2 as libc::c_int as
                                                          usize] as
                                             libc::c_int ==
                                             -(90 as libc::c_int) ||
                                         (*pe).angles[2 as libc::c_int as
                                                          usize] as
                                             libc::c_int ==
                                             -(180 as libc::c_int) ||
                                         (*pe).angles[2 as libc::c_int as
                                                          usize] as
                                             libc::c_int ==
                                             -(270 as libc::c_int))) &&
                                   (*pmove).usehull != 2 as libc::c_int {
                                transform_bbox = true_0
                            }
                        }
                        if transform_bbox as u64 != 0 {
                            Matrix4x4_CreateFromEntity(matrix.as_mut_ptr(),
                                                       (*pe).angles.as_mut_ptr()
                                                           as *const vec_t,
                                                       (*pe).origin.as_mut_ptr()
                                                           as *const vec_t,
                                                       1.0f32);
                        } else {
                            Matrix4x4_CreateFromEntity(matrix.as_mut_ptr(),
                                                       (*pe).angles.as_mut_ptr()
                                                           as *const vec_t,
                                                       offset.as_mut_ptr() as
                                                           *const vec_t,
                                                       1.0f32);
                        }
                        Matrix4x4_VectorITransform(matrix.as_mut_ptr() as
                                                       *const [vec_t; 4],
                                                   pos as
                                                       *const libc::c_float,
                                                   pos_l.as_mut_ptr());
                        if transform_bbox as u64 != 0 {
                            World_TransformAABB(matrix.as_mut_ptr(),
                                                (*pmove).player_mins[(*pmove).usehull
                                                                         as
                                                                         usize].as_mut_ptr()
                                                    as *const vec_t,
                                                (*pmove).player_maxs[(*pmove).usehull
                                                                         as
                                                                         usize].as_mut_ptr()
                                                    as *const vec_t,
                                                mins.as_mut_ptr(),
                                                maxs.as_mut_ptr());
                            offset[0 as libc::c_int as usize] =
                                (*hull).clip_mins[0 as libc::c_int as usize] -
                                    mins[0 as libc::c_int as usize];
                            offset[1 as libc::c_int as usize] =
                                (*hull).clip_mins[1 as libc::c_int as usize] -
                                    mins[1 as libc::c_int as usize];
                            offset[2 as libc::c_int as usize] =
                                (*hull).clip_mins[2 as libc::c_int as usize] -
                                    mins[2 as libc::c_int as usize];
                            j = 0 as libc::c_int;
                            while j < 3 as libc::c_int {
                                if pos_l[j as usize] >= 0.0f32 {
                                    pos_l[j as usize] -= offset[j as usize]
                                } else {
                                    pos_l[j as usize] += offset[j as usize]
                                }
                                j += 1
                            }
                        }
                    } else {
                        // offset the test point appropriately for this hull.
                        pos_l[0 as libc::c_int as usize] =
                            *pos.offset(0 as libc::c_int as isize) -
                                offset[0 as libc::c_int as usize];
                        pos_l[1 as libc::c_int as usize] =
                            *pos.offset(1 as libc::c_int as isize) -
                                offset[1 as libc::c_int as usize];
                        pos_l[2 as libc::c_int as usize] =
                            *pos.offset(2 as libc::c_int as isize) -
                                offset[2 as libc::c_int as usize]
                    }
                    if (*pe).solid == 5 as libc::c_int {
                        let mut trace_0: pmtrace_t =
                            pmtrace_t{allsolid: false_0,
                                      startsolid: false_0,
                                      inopen: false_0,
                                      inwater: false_0,
                                      fraction: 0.,
                                      endpos: [0.; 3],
                                      plane:
                                          pmplane_t{normal: [0.; 3],
                                                    dist: 0.,},
                                      ent: 0,
                                      deltavelocity: [0.; 3],
                                      hitgroup: 0,};
                        memset(&mut trace_0 as *mut pmtrace_t as
                                   *mut libc::c_void, 0 as libc::c_int,
                               ::std::mem::size_of::<pmtrace_t>() as
                                   libc::c_ulong);
                        trace_0.endpos[0 as libc::c_int as usize] =
                            *pos.offset(0 as libc::c_int as isize);
                        trace_0.endpos[1 as libc::c_int as usize] =
                            *pos.offset(1 as libc::c_int as isize);
                        trace_0.endpos[2 as libc::c_int as usize] =
                            *pos.offset(2 as libc::c_int as isize);
                        trace_0.allsolid = true_0;
                        trace_0.fraction = 1.0f32;
                        // run custom sweep callback
                        if (*pmove).server as libc::c_uint != 0 ||
                               Host_IsLocalClient() as libc::c_uint != 0 {
                            SV_ClipPMoveToEntity(pe, pos as *const vec_t,
                                                 mins.as_mut_ptr(),
                                                 maxs.as_mut_ptr(),
                                                 pos as *const vec_t,
                                                 &mut trace_0);
                        } else {
                            CL_ClipPMoveToEntity(pe, pos as *const vec_t,
                                                 mins.as_mut_ptr(),
                                                 maxs.as_mut_ptr(),
                                                 pos as *const vec_t,
                                                 &mut trace_0);
                        }
                        // if we inside the custom hull
                        if trace_0.allsolid as u64 != 0 { return i }
                    } else if hullcount == 1 as libc::c_int {
                        if PM_HullPointContents(hull, (*hull).firstclipnode,
                                                pos_l.as_mut_ptr() as
                                                    *const vec_t) ==
                               -(2 as libc::c_int) {
                            return i
                        }
                    } else {
                        j = 0 as libc::c_int;
                        while j < hullcount {
                            if PM_HullPointContents(&mut *hull.offset(j as
                                                                          isize),
                                                    (*hull.offset(j as
                                                                      isize)).firstclipnode,
                                                    pos_l.as_mut_ptr() as
                                                        *const vec_t) ==
                                   -(2 as libc::c_int) {
                                return i
                            }
                            j += 1
                        }
                    }
                }
            }
            _ => { }
        }
        i += 1
    }
    return -(1 as libc::c_int);
    // didn't hit anything
}
/*
=============
PM_TruePointContents

=============
*/
#[no_mangle]
pub unsafe extern "C" fn PM_TruePointContents(mut pmove: *mut playermove_t,
                                              mut p: *const vec_t)
 -> libc::c_int {
    let mut hull: *mut hull_t =
        &mut *(*(*(*pmove).physents.as_mut_ptr().offset(0 as libc::c_int as
                                                            isize)).model).hulls.as_mut_ptr().offset(0
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         isize)
            as *mut hull_t;
    if !hull.is_null() {
        return PM_HullPointContents(hull, (*hull).firstclipnode, p)
    } else { return -(1 as libc::c_int) };
}
/*
=============
PM_PointContents

=============
*/
#[no_mangle]
pub unsafe extern "C" fn PM_PointContents(mut pmove: *mut playermove_t,
                                          mut p: *const vec_t)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut contents: libc::c_int = 0;
    let mut hull: *mut hull_t = 0 as *mut hull_t;
    let mut test: vec3_t = [0.; 3];
    let mut pe: *mut physent_t = 0 as *mut physent_t;
    // sanity check
    if p.is_null() ||
           (*pmove).physents[0 as libc::c_int as usize].model.is_null() {
        return 0 as libc::c_int
    }
    // get base contents from world
    contents =
        PM_HullPointContents(&mut *(*(*(*pmove).physents.as_mut_ptr().offset(0
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 isize)).model).hulls.as_mut_ptr().offset(0
                                                                                                                              as
                                                                                                                              libc::c_int
                                                                                                                              as
                                                                                                                              isize),
                             0 as libc::c_int, p);
    i = 1 as libc::c_int;
    while i < (*pmove).numphysent {
        pe =
            &mut *(*pmove).physents.as_mut_ptr().offset(i as isize) as
                *mut physent_t;
        if !((*pe).solid != 0 as libc::c_int) {
            // new content has more priority
            // only brushes can have special contents
            if !(*pe).model.is_null() {
                // check water brushes accuracy
                hull =
                    &mut *(*(*pe).model).hulls.as_mut_ptr().offset(0 as
                                                                       libc::c_int
                                                                       as
                                                                       isize)
                        as *mut hull_t;
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
                                               (*pe).origin.as_mut_ptr() as
                                                   *const vec_t, 1.0f32);
                    Matrix4x4_VectorITransform(matrix.as_mut_ptr() as
                                                   *const [vec_t; 4], p,
                                               test.as_mut_ptr());
                } else {
                    // offset the test point appropriately for this hull.
                    test[0 as libc::c_int as usize] =
                        *p.offset(0 as libc::c_int as isize) -
                            (*pe).origin[0 as libc::c_int as usize];
                    test[1 as libc::c_int as usize] =
                        *p.offset(1 as libc::c_int as isize) -
                            (*pe).origin[1 as libc::c_int as usize];
                    test[2 as libc::c_int as usize] =
                        *p.offset(2 as libc::c_int as isize) -
                            (*pe).origin[2 as libc::c_int as usize]
                }
                // test hull for intersection with this model
                if !(PM_HullPointContents(hull, (*hull).firstclipnode,
                                          test.as_mut_ptr() as *const vec_t)
                         == -(1 as libc::c_int)) {
                    // compare contents ranking
                    if RankForContents((*pe).skin) > RankForContents(contents)
                       {
                        contents = (*pe).skin
                    }
                }
            }
        }
        i += 1
    }
    return contents;
}
// disabled ?
