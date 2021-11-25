#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type edict_s;
    pub type grasshdr_s;
    pub type movevars_s;
    pub type ref_viewpass_s;
    pub type mip_s;
    #[no_mangle]
    fn fabsf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn Sys_Error(error: *const libc::c_char, _: ...);
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    fn Matrix4x4_VectorITransform(in_0: *const [vec_t; 4],
                                  v: *const libc::c_float,
                                  out: *mut libc::c_float);
    #[no_mangle]
    fn Matrix4x4_CreateFromEntity(out: *mut [vec_t; 4], angles: *const vec_t,
                                  origin: *const vec_t, scale: libc::c_float);
    #[no_mangle]
    static mut vec3_origin: vec3_t;
    #[no_mangle]
    fn PM_HullForBsp(pe: *mut physent_t, pmove: *mut playermove_t,
                     offset: *mut libc::c_float) -> *mut hull_t;
    #[link_name = "ref"]
    static mut ref_0: ref_state_s;
}
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const kRenderTransAdd: C2RustUnnamed_0 = 5;
pub const kRenderTransAlpha: C2RustUnnamed_0 = 4;
pub const kRenderGlow: C2RustUnnamed_0 = 3;
pub const kRenderTransTexture: C2RustUnnamed_0 = 2;
pub const kRenderTransColor: C2RustUnnamed_0 = 1;
pub const kRenderNormal: C2RustUnnamed_0 = 0;
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
pub type usercmd_t = usercmd_s;
pub type physent_t = physent_s;
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
pub type playermove_t = playermove_s;
pub type ref_interface_t = ref_interface_s;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpoint_t {
    pub point: vec2_t,
    pub coord: vec2_t,
}
pub type TRICULLSTYLE = libc::c_uint;
pub const TRI_NONE: TRICULLSTYLE = 1;
pub const TRI_FRONT: TRICULLSTYLE = 0;
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
pub type cl_entity_t = cl_entity_s;
pub type mstudioseqdesc_t = mstudioseqdesc_s;
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
pub type ref_screen_rotation_t = ref_screen_rotation_e;
pub type ref_screen_rotation_e = libc::c_uint;
pub const REF_ROTATE_CCW: ref_screen_rotation_e = 3;
pub const REF_ROTATE_UD: ref_screen_rotation_e = 2;
pub const REF_ROTATE_CW: ref_screen_rotation_e = 1;
pub const REF_ROTATE_NONE: ref_screen_rotation_e = 0;
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
pub struct linetrace_t {
    pub fraction: libc::c_float,
    pub contents: libc::c_int,
    pub surface: *mut msurface_t,
}
#[inline(always)]
unsafe extern "C" fn __tg_fabs(mut __x: libc::c_float) -> libc::c_float {
    return fabsf(__x);
}
/*
==============
fix_coord

converts the reletive tex coords to absolute
==============
*/
unsafe extern "C" fn fix_coord(mut in_0: vec_t, mut width: uint) -> uint {
    if in_0 > 0 as libc::c_int as libc::c_float {
        return (in_0 as uint).wrapping_rem(width)
    }
    return width.wrapping_sub((__tg_fabs(in_0) as uint).wrapping_rem(width));
}
/*
=============
SampleMiptex

fence texture testing
=============
*/
#[no_mangle]
pub unsafe extern "C" fn PM_SampleMiptex(mut surf: *const msurface_t,
                                         mut point: *const vec_t)
 -> libc::c_int {
    let mut info: *mut mextrasurf_t = (*surf).info;
    let mut fb: *mut mfacebevel_t = (*info).bevel;
    let mut contents: libc::c_int = 0;
    let mut ds: vec_t = 0.;
    let mut dt: vec_t = 0.;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut tx: *mut mtexinfo_t = 0 as *mut mtexinfo_t;
    let mut mt: *mut texture_t = 0 as *mut texture_t;
    // fill the default contents
    if !fb.is_null() {
        contents = (*fb).contents
    } else { contents = -(2 as libc::c_int) }
    if (*surf).texinfo.is_null() || (*(*surf).texinfo).texture.is_null() {
        return contents
    }
    tx = (*surf).texinfo;
    mt = (*tx).texture;
    if (*mt).name[0 as libc::c_int as usize] as libc::c_int != '{' as i32 {
        return contents
    }
    // TODO: this won't work under dedicated
	// should we bring up imagelib and keep original buffers?
    if !(host.type_0 == HOST_DEDICATED as libc::c_int as libc::c_uint) {
        let mut data: *const byte = 0 as *const byte; // original doesn't kept
        data =
            ref_0.dllFuncs.R_GetTextureOriginalBuffer.expect("non-null function pointer")((*mt).gl_texturenum
                                                                                              as
                                                                                              libc::c_uint);
        if data.is_null() { return contents }
        ds =
            *point.offset(0 as libc::c_int as isize) *
                (*tx).vecs[0 as libc::c_int as
                               usize][0 as libc::c_int as usize] +
                *point.offset(1 as libc::c_int as isize) *
                    (*tx).vecs[0 as libc::c_int as
                                   usize][1 as libc::c_int as usize] +
                *point.offset(2 as libc::c_int as isize) *
                    (*tx).vecs[0 as libc::c_int as
                                   usize][2 as libc::c_int as usize] +
                (*tx).vecs[0 as libc::c_int as
                               usize][3 as libc::c_int as usize];
        dt =
            *point.offset(0 as libc::c_int as isize) *
                (*tx).vecs[1 as libc::c_int as
                               usize][0 as libc::c_int as usize] +
                *point.offset(1 as libc::c_int as isize) *
                    (*tx).vecs[1 as libc::c_int as
                                   usize][1 as libc::c_int as usize] +
                *point.offset(2 as libc::c_int as isize) *
                    (*tx).vecs[1 as libc::c_int as
                                   usize][2 as libc::c_int as usize] +
                (*tx).vecs[1 as libc::c_int as
                               usize][3 as libc::c_int as usize];
        // convert ST to real pixels position
        x =
            fix_coord(ds,
                      (*mt).width.wrapping_sub(1 as libc::c_int as
                                                   libc::c_uint)) as
                libc::c_int;
        y =
            fix_coord(dt,
                      (*mt).height.wrapping_sub(1 as libc::c_int as
                                                    libc::c_uint)) as
                libc::c_int;
        if !(x >= 0 as libc::c_int && y >= 0 as libc::c_int) {
            Sys_Error(b"assert failed at %s:%i\n\x00" as *const u8 as
                          *const libc::c_char,
                      b"../engine/common/pm_surface.c\x00" as *const u8 as
                          *const libc::c_char, 92 as libc::c_int);
        }
        if *data.offset((*mt).width.wrapping_mul(y as
                                                     libc::c_uint).wrapping_add(x
                                                                                    as
                                                                                    libc::c_uint)
                            as isize) as libc::c_int == 255 as libc::c_int {
            return -(1 as libc::c_int)
        }
        return -(2 as libc::c_int)
    }
    // !XASH_DEDICATED
    return contents;
}
/*
==================
PM_RecursiveSurfCheck

==================
*/
#[no_mangle]
pub unsafe extern "C" fn PM_RecursiveSurfCheck(mut mod_0: *mut model_t,
                                               mut node: *mut mnode_t,
                                               mut p1: *mut vec_t,
                                               mut p2: *mut vec_t)
 -> *mut msurface_t {
    let mut t1: libc::c_float = 0.;
    let mut t2: libc::c_float = 0.;
    let mut frac: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    let mut side: libc::c_int = 0;
    let mut surf: *mut msurface_t = 0 as *mut msurface_t;
    let mut mid: vec3_t = [0.; 3];
    loop  {
        if (*node).contents < 0 as libc::c_int { return 0 as *mut msurface_t }
        t1 =
            (if ((*(*node).plane).type_0 as libc::c_int) < 3 as libc::c_int {
                 *p1.offset((*(*node).plane).type_0 as isize)
             } else {
                 (*p1.offset(0 as libc::c_int as isize) *
                      (*(*node).plane).normal[0 as libc::c_int as usize] +
                      *p1.offset(1 as libc::c_int as isize) *
                          (*(*node).plane).normal[1 as libc::c_int as usize])
                     +
                     *p1.offset(2 as libc::c_int as isize) *
                         (*(*node).plane).normal[2 as libc::c_int as usize]
             }) - (*(*node).plane).dist;
        t2 =
            (if ((*(*node).plane).type_0 as libc::c_int) < 3 as libc::c_int {
                 *p2.offset((*(*node).plane).type_0 as isize)
             } else {
                 (*p2.offset(0 as libc::c_int as isize) *
                      (*(*node).plane).normal[0 as libc::c_int as usize] +
                      *p2.offset(1 as libc::c_int as isize) *
                          (*(*node).plane).normal[1 as libc::c_int as usize])
                     +
                     *p2.offset(2 as libc::c_int as isize) *
                         (*(*node).plane).normal[2 as libc::c_int as usize]
             }) - (*(*node).plane).dist;
        if t1 >= -(1.0f32 / 32.0f32) && t2 >= -(1.0f32 / 32.0f32) {
            node = (*node).children[0 as libc::c_int as usize]
        } else {
            if !(t1 < 1.0f32 / 32.0f32 && t2 < 1.0f32 / 32.0f32) { break ; }
            node = (*node).children[1 as libc::c_int as usize]
        }
    }
    side = (t1 < 0.0f32) as libc::c_int;
    frac = t1 / (t1 - t2);
    frac =
        if frac >= 0.0f32 {
            if frac < 1.0f32 { frac } else { 1.0f32 }
        } else { 0.0f32 };
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
    surf =
        PM_RecursiveSurfCheck(mod_0, (*node).children[side as usize], p1,
                              mid.as_mut_ptr());
    if !surf.is_null() { return surf }
    // walk through real faces
    i = 0 as libc::c_int;
    while i < (*node).numsurfaces as libc::c_int {
        let mut surf_0: *mut msurface_t =
            &mut *(*mod_0).surfaces.offset(((*node).firstsurface as
                                                libc::c_int + i) as isize) as
                *mut msurface_t;
        let mut info: *mut mextrasurf_t = (*surf_0).info;
        let mut fb: *mut mfacebevel_t = (*info).bevel;
        let mut j: libc::c_int = 0;
        let mut contents: libc::c_int = 0;
        let mut delta: vec3_t = [0.; 3];
        // through the fence
        if !fb.is_null() {
            delta[0 as libc::c_int as usize] =
                mid[0 as libc::c_int as usize] -
                    (*fb).origin[0 as libc::c_int as usize]; // ???
            delta[1 as libc::c_int as usize] =
                mid[1 as libc::c_int as usize] -
                    (*fb).origin[1 as libc::c_int as
                                     usize]; // no intersection
            delta[2 as libc::c_int as usize] =
                mid[2 as libc::c_int as usize] -
                    (*fb).origin[2 as libc::c_int as usize];
            if !(delta[0 as libc::c_int as usize] *
                     delta[0 as libc::c_int as usize] +
                     delta[1 as libc::c_int as usize] *
                         delta[1 as libc::c_int as usize] +
                     delta[2 as libc::c_int as usize] *
                         delta[2 as libc::c_int as usize] >= (*fb).radius) {
                j = 0 as libc::c_int;
                while j < (*fb).numedges {
                    if (if ((*(*fb).edges.offset(j as isize)).type_0 as
                                libc::c_int) < 3 as libc::c_int {
                            mid[(*(*fb).edges.offset(j as isize)).type_0 as
                                    usize]
                        } else {
                            (mid[0 as libc::c_int as usize] *
                                 (*(*fb).edges.offset(j as
                                                          isize)).normal[0 as
                                                                             libc::c_int
                                                                             as
                                                                             usize]
                                 +
                                 mid[1 as libc::c_int as usize] *
                                     (*(*fb).edges.offset(j as
                                                              isize)).normal[1
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 usize])
                                +
                                mid[2 as libc::c_int as usize] *
                                    (*(*fb).edges.offset(j as
                                                             isize)).normal[2
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]
                        }) - (*(*fb).edges.offset(j as isize)).dist >
                           1.0f32 / 32.0f32 {
                        break ;
                        // outside the bounds
                    } // we are outside the bounds of the facet
                    j += 1
                }
                if !(j != (*fb).numedges) {
                    // hit the surface
                    contents =
                        PM_SampleMiptex(surf_0,
                                        mid.as_mut_ptr() as *const vec_t);
                    if contents != -(1 as libc::c_int) { return surf_0 }
                    return 0 as *mut msurface_t
                }
            }
        }
        i += 1
    }
    return PM_RecursiveSurfCheck(mod_0,
                                 (*node).children[(side ^ 1 as libc::c_int) as
                                                      usize],
                                 mid.as_mut_ptr(), p2);
}
/*
==================
PM_TraceTexture

find the face where the traceline hit
assume physentity is valid
==================
*/
#[no_mangle]
pub unsafe extern "C" fn PM_TraceSurface(mut pe: *mut physent_t,
                                         mut start: *mut vec_t,
                                         mut end: *mut vec_t)
 -> *mut msurface_t {
    let mut matrix: matrix4x4 = [[0.; 4]; 4];
    let mut bmodel: *mut model_t = 0 as *mut model_t;
    let mut hull: *mut hull_t = 0 as *mut hull_t;
    let mut start_l: vec3_t = [0.; 3];
    let mut end_l: vec3_t = [0.; 3];
    let mut offset: vec3_t = [0.; 3];
    bmodel = (*pe).model;
    if bmodel.is_null() ||
           (*bmodel).type_0 as libc::c_int != mod_brush as libc::c_int {
        return 0 as *mut msurface_t
    }
    hull =
        &mut *(*(*pe).model).hulls.as_mut_ptr().offset(0 as libc::c_int as
                                                           isize) as
            *mut hull_t;
    offset[0 as libc::c_int as usize] =
        (*hull).clip_mins[0 as libc::c_int as usize] -
            vec3_origin[0 as libc::c_int as usize];
    offset[1 as libc::c_int as usize] =
        (*hull).clip_mins[1 as libc::c_int as usize] -
            vec3_origin[1 as libc::c_int as usize];
    offset[2 as libc::c_int as usize] =
        (*hull).clip_mins[2 as libc::c_int as usize] -
            vec3_origin[2 as libc::c_int as usize];
    offset[0 as libc::c_int as usize] =
        offset[0 as libc::c_int as usize] +
            (*pe).origin[0 as libc::c_int as usize];
    offset[1 as libc::c_int as usize] =
        offset[1 as libc::c_int as usize] +
            (*pe).origin[1 as libc::c_int as usize];
    offset[2 as libc::c_int as usize] =
        offset[2 as libc::c_int as usize] +
            (*pe).origin[2 as libc::c_int as usize];
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
    if !((*pe).angles[0 as libc::c_int as usize] == 0.0f32 &&
             (*pe).angles[1 as libc::c_int as usize] == 0.0f32 &&
             (*pe).angles[2 as libc::c_int as usize] == 0.0f32) {
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
    }
    return PM_RecursiveSurfCheck(bmodel,
                                 &mut *(*bmodel).nodes.offset((*hull).firstclipnode
                                                                  as isize),
                                 start_l.as_mut_ptr(), end_l.as_mut_ptr());
}
/*
==================
PM_TraceTexture

find the face where the traceline hit
assume physentity is valid
==================
*/
#[no_mangle]
pub unsafe extern "C" fn PM_TraceTexture(mut pe: *mut physent_t,
                                         mut start: *mut vec_t,
                                         mut end: *mut vec_t)
 -> *const libc::c_char {
    let mut surf: *mut msurface_t = PM_TraceSurface(pe, start, end);
    if surf.is_null() || (*surf).texinfo.is_null() ||
           (*(*surf).texinfo).texture.is_null() {
        return 0 as *const libc::c_char
    }
    return (*(*(*surf).texinfo).texture).name.as_mut_ptr();
}
/*
==================
PM_TestLine_r

optimized trace for light gathering
==================
*/
#[no_mangle]
pub unsafe extern "C" fn PM_TestLine_r(mut mod_0: *mut model_t,
                                       mut node: *mut mnode_t, mut p1f: vec_t,
                                       mut p2f: vec_t,
                                       mut start: *const vec_t,
                                       mut stop: *const vec_t,
                                       mut trace: *mut linetrace_t)
 -> libc::c_int {
    let mut front: libc::c_float = 0.;
    let mut back: libc::c_float = 0.;
    let mut frac: libc::c_float = 0.;
    let mut midf: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut side: libc::c_int = 0;
    let mut mid: vec3_t = [0.; 3];
    loop  {
        if (*node).contents < 0 as libc::c_int {
            // water, slime or lava interpret as empty
            if (*node).contents == -(2 as libc::c_int) {
                return -(2 as libc::c_int)
            }
            if (*node).contents == -(6 as libc::c_int) {
                return -(6 as libc::c_int)
            }
            (*trace).fraction = 1.0f32;
            return -(1 as libc::c_int)
        }
        front =
            (if ((*(*node).plane).type_0 as libc::c_int) < 3 as libc::c_int {
                 *start.offset((*(*node).plane).type_0 as isize)
             } else {
                 (*start.offset(0 as libc::c_int as isize) *
                      (*(*node).plane).normal[0 as libc::c_int as usize] +
                      *start.offset(1 as libc::c_int as isize) *
                          (*(*node).plane).normal[1 as libc::c_int as usize])
                     +
                     *start.offset(2 as libc::c_int as isize) *
                         (*(*node).plane).normal[2 as libc::c_int as usize]
             }) - (*(*node).plane).dist;
        back =
            (if ((*(*node).plane).type_0 as libc::c_int) < 3 as libc::c_int {
                 *stop.offset((*(*node).plane).type_0 as isize)
             } else {
                 (*stop.offset(0 as libc::c_int as isize) *
                      (*(*node).plane).normal[0 as libc::c_int as usize] +
                      *stop.offset(1 as libc::c_int as isize) *
                          (*(*node).plane).normal[1 as libc::c_int as usize])
                     +
                     *stop.offset(2 as libc::c_int as isize) *
                         (*(*node).plane).normal[2 as libc::c_int as usize]
             }) - (*(*node).plane).dist;
        if front >= -(1.0f32 / 32.0f32) && back >= -(1.0f32 / 32.0f32) {
            node = (*node).children[0 as libc::c_int as usize]
        } else {
            if !(front < 1.0f32 / 32.0f32 && back < 1.0f32 / 32.0f32) {
                break ;
            }
            node = (*node).children[1 as libc::c_int as usize]
        }
    }
    side = (front < 0 as libc::c_int as libc::c_float) as libc::c_int;
    frac = front / (front - back);
    frac =
        if frac >= 0.0f32 {
            if frac < 1.0f32 { frac } else { 1.0f32 }
        } else { 0.0f32 };
    mid[0 as libc::c_int as usize] =
        *start.offset(0 as libc::c_int as isize) +
            frac *
                (*stop.offset(0 as libc::c_int as isize) -
                     *start.offset(0 as libc::c_int as isize));
    mid[1 as libc::c_int as usize] =
        *start.offset(1 as libc::c_int as isize) +
            frac *
                (*stop.offset(1 as libc::c_int as isize) -
                     *start.offset(1 as libc::c_int as isize));
    mid[2 as libc::c_int as usize] =
        *start.offset(2 as libc::c_int as isize) +
            frac *
                (*stop.offset(2 as libc::c_int as isize) -
                     *start.offset(2 as libc::c_int as isize));
    midf = p1f + (p2f - p1f) * frac;
    r =
        PM_TestLine_r(mod_0, (*node).children[side as usize], p1f, midf,
                      start, mid.as_mut_ptr() as *const vec_t, trace);
    if r != -(1 as libc::c_int) {
        if (*trace).surface.is_null() { (*trace).fraction = midf }
        (*trace).contents = r;
        return r
    }
    // walk through real faces
    i = 0 as libc::c_int; // no intersection
    while i < (*node).numsurfaces as libc::c_int {
        let mut surf: *mut msurface_t =
            &mut *(*mod_0).surfaces.offset(((*node).firstsurface as
                                                libc::c_int + i) as isize) as
                *mut msurface_t;
        let mut info: *mut mextrasurf_t = (*surf).info;
        let mut fb: *mut mfacebevel_t = (*info).bevel;
        let mut j: libc::c_int = 0;
        let mut contents: libc::c_int = 0;
        let mut delta: vec3_t = [0.; 3];
        if !fb.is_null() {
            delta[0 as libc::c_int as usize] =
                mid[0 as libc::c_int as usize] -
                    (*fb).origin[0 as libc::c_int as usize];
            delta[1 as libc::c_int as usize] =
                mid[1 as libc::c_int as usize] -
                    (*fb).origin[1 as libc::c_int as usize];
            delta[2 as libc::c_int as usize] =
                mid[2 as libc::c_int as usize] -
                    (*fb).origin[2 as libc::c_int as usize];
            if !(delta[0 as libc::c_int as usize] *
                     delta[0 as libc::c_int as usize] +
                     delta[1 as libc::c_int as usize] *
                         delta[1 as libc::c_int as usize] +
                     delta[2 as libc::c_int as usize] *
                         delta[2 as libc::c_int as usize] >= (*fb).radius) {
                j = 0 as libc::c_int;
                while j < (*fb).numedges {
                    if (if ((*(*fb).edges.offset(j as isize)).type_0 as
                                libc::c_int) < 3 as libc::c_int {
                            mid[(*(*fb).edges.offset(j as isize)).type_0 as
                                    usize]
                        } else {
                            (mid[0 as libc::c_int as usize] *
                                 (*(*fb).edges.offset(j as
                                                          isize)).normal[0 as
                                                                             libc::c_int
                                                                             as
                                                                             usize]
                                 +
                                 mid[1 as libc::c_int as usize] *
                                     (*(*fb).edges.offset(j as
                                                              isize)).normal[1
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 usize])
                                +
                                mid[2 as libc::c_int as usize] *
                                    (*(*fb).edges.offset(j as
                                                             isize)).normal[2
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]
                        }) - (*(*fb).edges.offset(j as isize)).dist >
                           1.0f32 / 32.0f32 {
                        break ;
                        // outside the bounds
                    } // we are outside the bounds of the facet
                    j += 1
                }
                if !(j != (*fb).numedges) {
                    // hit the surface
                    contents =
                        PM_SampleMiptex(surf,
                                        mid.as_mut_ptr() as *const vec_t);
                    // fill the trace and out
                    (*trace).contents = contents;
                    (*trace).fraction = midf;
                    if contents != -(1 as libc::c_int) {
                        (*trace).surface = surf
                    }
                    return contents
                }
            }
        }
        i += 1
    }
    return PM_TestLine_r(mod_0,
                         (*node).children[(side == 0) as libc::c_int as
                                              usize], midf, p2f,
                         mid.as_mut_ptr() as *const vec_t, stop, trace);
}
#[no_mangle]
pub unsafe extern "C" fn PM_TestLineExt(mut pmove: *mut playermove_t,
                                        mut ents: *mut physent_t,
                                        mut numents: libc::c_int,
                                        mut start: *const vec_t,
                                        mut end: *const vec_t,
                                        mut flags: libc::c_int)
 -> libc::c_int {
    let mut trace: linetrace_t =
        linetrace_t{fraction: 0.,
                    contents: 0,
                    surface: 0 as *mut msurface_t,};
    let mut trace_bbox: linetrace_t =
        linetrace_t{fraction: 0.,
                    contents: 0,
                    surface: 0 as *mut msurface_t,};
    let mut matrix: matrix4x4 = [[0.; 4]; 4];
    let mut hull: *mut hull_t = 0 as *mut hull_t;
    let mut offset: vec3_t = [0.; 3];
    let mut start_l: vec3_t = [0.; 3];
    let mut end_l: vec3_t = [0.; 3];
    let mut rotated: qboolean = false_0;
    let mut pe: *mut physent_t = 0 as *mut physent_t;
    let mut i: libc::c_int = 0;
    trace.contents = -(1 as libc::c_int);
    trace.fraction = 1.0f32;
    trace.surface = 0 as *mut msurface_t;
    i = 0 as libc::c_int;
    while i < numents {
        pe = &mut *ents.offset(i as isize) as *mut physent_t;
        if i != 0 as libc::c_int && flags & 0x8 as libc::c_int != 0 {
            break ;
        }
        if !((*pe).model.is_null() ||
                 (*(*pe).model).type_0 as libc::c_int !=
                     mod_brush as libc::c_int ||
                 (*pe).solid != 4 as libc::c_int) {
            if !(flags & 0x4 as libc::c_int != 0 &&
                     (*pe).rendermode != kRenderNormal as libc::c_int) {
                hull =
                    &mut *(*(*pe).model).hulls.as_mut_ptr().offset(0 as
                                                                       libc::c_int
                                                                       as
                                                                       isize)
                        as *mut hull_t;
                hull = PM_HullForBsp(pe, pmove, offset.as_mut_ptr());
                if (*pe).solid == 4 as libc::c_int &&
                       !((*pe).angles[0 as libc::c_int as usize] == 0.0f32 &&
                             (*pe).angles[1 as libc::c_int as usize] == 0.0f32
                             &&
                             (*pe).angles[2 as libc::c_int as usize] ==
                                 0.0f32) {
                    rotated = true_0
                } else { rotated = false_0 }
                if rotated as u64 != 0 {
                    Matrix4x4_CreateFromEntity(matrix.as_mut_ptr(),
                                               (*pe).angles.as_mut_ptr() as
                                                   *const vec_t,
                                               offset.as_mut_ptr() as
                                                   *const vec_t, 1.0f32);
                    Matrix4x4_VectorITransform(matrix.as_mut_ptr() as
                                                   *const [vec_t; 4], start,
                                               start_l.as_mut_ptr());
                    Matrix4x4_VectorITransform(matrix.as_mut_ptr() as
                                                   *const [vec_t; 4], end,
                                               end_l.as_mut_ptr());
                } else {
                    start_l[0 as libc::c_int as usize] =
                        *start.offset(0 as libc::c_int as isize) -
                            (*pe).origin[0 as libc::c_int as usize];
                    start_l[1 as libc::c_int as usize] =
                        *start.offset(1 as libc::c_int as isize) -
                            (*pe).origin[1 as libc::c_int as usize];
                    start_l[2 as libc::c_int as usize] =
                        *start.offset(2 as libc::c_int as isize) -
                            (*pe).origin[2 as libc::c_int as usize];
                    end_l[0 as libc::c_int as usize] =
                        *end.offset(0 as libc::c_int as isize) -
                            (*pe).origin[0 as libc::c_int as usize];
                    end_l[1 as libc::c_int as usize] =
                        *end.offset(1 as libc::c_int as isize) -
                            (*pe).origin[1 as libc::c_int as usize];
                    end_l[2 as libc::c_int as usize] =
                        *end.offset(2 as libc::c_int as isize) -
                            (*pe).origin[2 as libc::c_int as usize]
                }
                trace_bbox.contents = -(1 as libc::c_int);
                trace_bbox.fraction = 1.0f32;
                trace_bbox.surface = 0 as *mut msurface_t;
                PM_TestLine_r((*pe).model,
                              &mut *(*(*pe).model).nodes.offset((*hull).firstclipnode
                                                                    as isize),
                              0.0f32, 1.0f32,
                              start_l.as_mut_ptr() as *const vec_t,
                              end_l.as_mut_ptr() as *const vec_t,
                              &mut trace_bbox);
                if trace_bbox.contents != -(1 as libc::c_int) ||
                       trace_bbox.fraction < trace.fraction {
                    trace = trace_bbox
                }
            }
        }
        i += 1
    }
    return trace.contents;
}
