#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type grasshdr_s;
    #[no_mangle]
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    fn _Mem_Alloc(poolptr: poolhandle_t, size: size_t, clear: qboolean,
                  filename: *const libc::c_char, fileline: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    fn _Mem_AllocPool(name: *const libc::c_char,
                      filename: *const libc::c_char, fileline: libc::c_int)
     -> poolhandle_t;
    #[no_mangle]
    fn Con_DPrintf(fmt: *const libc::c_char, _: ...);
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
pub type poolhandle_t = uint32_t;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type dword = libc::c_uint;
pub type string = [libc::c_char; 256];
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
pub type spriteframetype_t = libc::c_uint;
pub const SPR_ANGLED: spriteframetype_t = 2;
pub const SPR_GROUP: spriteframetype_t = 1;
pub const SPR_SINGLE: spriteframetype_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mspriteframe_s {
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub up: libc::c_float,
    pub down: libc::c_float,
    pub left: libc::c_float,
    pub right: libc::c_float,
    pub gl_texturenum: libc::c_int,
}
pub type mspriteframe_t = mspriteframe_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mspriteframedesc_t {
    pub type_0: spriteframetype_t,
    pub frameptr: *mut mspriteframe_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msprite_t {
    pub type_0: libc::c_short,
    pub texFormat: libc::c_short,
    pub maxwidth: libc::c_int,
    pub maxheight: libc::c_int,
    pub numframes: libc::c_int,
    pub radius: libc::c_int,
    pub facecull: libc::c_int,
    pub synctype: libc::c_int,
    pub frames: [mspriteframedesc_t; 1],
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
pub type synctype_t = libc::c_uint;
pub const ST_RAND: synctype_t = 1;
pub const ST_SYNC: synctype_t = 0;
pub type drawtype_t = libc::c_uint;
pub const SPR_ALPHTEST: drawtype_t = 3;
pub const SPR_INDEXALPHA: drawtype_t = 2;
pub const SPR_ADDITIVE: drawtype_t = 1;
pub const SPR_NORMAL: drawtype_t = 0;
pub type angletype_t = libc::c_uint;
pub const SPR_FWD_PARALLEL_ORIENTED: angletype_t = 4;
pub const SPR_ORIENTED: angletype_t = 3;
pub const SPR_FWD_PARALLEL: angletype_t = 2;
pub const SPR_FACING_UPRIGHT: angletype_t = 1;
pub const SPR_FWD_PARALLEL_UPRIGHT: angletype_t = 0;
pub type facetype_t = libc::c_uint;
pub const SPR_CULL_NONE: facetype_t = 1;
pub const SPR_CULL_FRONT: facetype_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dsprite_t {
    pub ident: libc::c_int,
    pub version: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dsprite_q1_t {
    pub ident: libc::c_int,
    pub version: libc::c_int,
    pub type_0: libc::c_int,
    pub boundingradius: libc::c_float,
    pub bounds: [libc::c_int; 2],
    pub numframes: libc::c_int,
    pub beamlength: libc::c_float,
    pub synctype: synctype_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dsprite_hl_t {
    pub ident: libc::c_int,
    pub version: libc::c_int,
    pub type_0: angletype_t,
    pub texFormat: drawtype_t,
    pub boundingradius: libc::c_int,
    pub bounds: [libc::c_int; 2],
    pub numframes: libc::c_int,
    pub facetype: facetype_t,
    pub synctype: synctype_t,
}
/*
mod_sprite.c - sprite loading
Copyright (C) 2010 Uncle Mike
Copyright (C) 2019 a1batross

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
*/
// XASH_DEDICATED
/*
====================
Mod_LoadSpriteModel

load sprite model
====================
*/
#[no_mangle]
pub unsafe extern "C" fn Mod_LoadSpriteModel(mut mod_0: *mut model_t,
                                             mut buffer: *const libc::c_void,
                                             mut loaded: *mut qboolean,
                                             mut texFlags: uint) {
    let mut pinq1: *mut dsprite_q1_t =
        0 as *mut dsprite_q1_t; // make link to extradata
    let mut pinhl: *mut dsprite_hl_t = 0 as *mut dsprite_hl_t; //SPR_ALPHTEST;
    let mut pin: *mut dsprite_t = 0 as *mut dsprite_t;
    let mut psprite: *mut msprite_t = 0 as *mut msprite_t;
    let mut i: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    if !loaded.is_null() { *loaded = false_0 }
    pin = buffer as *mut dsprite_t;
    (*mod_0).type_0 = mod_sprite;
    i = (*pin).version;
    if (*pin).ident !=
           (('P' as i32) << 24 as libc::c_int) +
               (('S' as i32) << 16 as libc::c_int) +
               (('D' as i32) << 8 as libc::c_int) + 'I' as i32 {
        Con_DPrintf(b"^1Error:^7 %s has wrong id (%x should be %x)\n\x00" as
                        *const u8 as *const libc::c_char,
                    (*mod_0).name.as_mut_ptr(), (*pin).ident,
                    (('P' as i32) << 24 as libc::c_int) +
                        (('S' as i32) << 16 as libc::c_int) +
                        (('D' as i32) << 8 as libc::c_int) + 'I' as i32);
        return
    }
    if i != 1 as libc::c_int && i != 2 as libc::c_int &&
           i != 32 as libc::c_int {
        Con_DPrintf(b"^1Error:^7 %s has wrong version number (%i should be %i or %i)\n\x00"
                        as *const u8 as *const libc::c_char,
                    (*mod_0).name.as_mut_ptr(), i, 1 as libc::c_int,
                    2 as libc::c_int);
        return
    }
    (*mod_0).mempool =
        _Mem_AllocPool(va(b"^2%s^7\x00" as *const u8 as *const libc::c_char,
                          (*mod_0).name.as_mut_ptr()),
                       b"../engine/common/mod_sprite.c\x00" as *const u8 as
                           *const libc::c_char, 57 as libc::c_int);
    if i == 1 as libc::c_int || i == 32 as libc::c_int {
        pinq1 = buffer as *mut dsprite_q1_t;
        size =
            (::std::mem::size_of::<msprite_t>() as
                 libc::c_ulong).wrapping_add((((*pinq1).numframes -
                                                   1 as libc::c_int) as
                                                  libc::c_ulong).wrapping_mul(::std::mem::size_of::<[mspriteframedesc_t; 1]>()
                                                                                  as
                                                                                  libc::c_ulong))
                as libc::c_int;
        psprite =
            _Mem_Alloc((*mod_0).mempool, size as size_t, true_0,
                       b"../engine/common/mod_sprite.c\x00" as *const u8 as
                           *const libc::c_char, 63 as libc::c_int) as
                *mut msprite_t;
        (*mod_0).cache.data = psprite as *mut libc::c_void;
        (*psprite).type_0 = (*pinq1).type_0 as libc::c_short;
        (*psprite).texFormat = SPR_ADDITIVE as libc::c_int as libc::c_short;
        (*mod_0).numframes = (*pinq1).numframes;
        (*psprite).numframes = (*mod_0).numframes;
        (*psprite).facecull = SPR_CULL_FRONT as libc::c_int;
        (*psprite).radius = (*pinq1).boundingradius as libc::c_int;
        (*psprite).synctype = (*pinq1).synctype as libc::c_int;
        // LordHavoc: hack to allow sprites to be non-fullbright
        i = 0 as libc::c_int;
        while i < 64 as libc::c_int &&
                  (*mod_0).name[i as usize] as libc::c_int != 0 {
            if (*mod_0).name[i as usize] as libc::c_int == '!' as i32 {
                (*psprite).texFormat =
                    SPR_ALPHTEST as libc::c_int as libc::c_short
            }
            i += 1
        }
        (*mod_0).mins[1 as libc::c_int as usize] =
            -(*pinq1).bounds[0 as libc::c_int as usize] as libc::c_float *
                0.5f32;
        (*mod_0).mins[0 as libc::c_int as usize] =
            (*mod_0).mins[1 as libc::c_int as usize];
        (*mod_0).maxs[1 as libc::c_int as usize] =
            (*pinq1).bounds[0 as libc::c_int as usize] as libc::c_float *
                0.5f32;
        (*mod_0).maxs[0 as libc::c_int as usize] =
            (*mod_0).maxs[1 as libc::c_int as usize];
        (*mod_0).mins[2 as libc::c_int as usize] =
            -(*pinq1).bounds[1 as libc::c_int as usize] as libc::c_float *
                0.5f32;
        (*mod_0).maxs[2 as libc::c_int as usize] =
            (*pinq1).bounds[1 as libc::c_int as usize] as libc::c_float *
                0.5f32
    } else {
        // if( i == SPRITE_VERSION_HL )
        pinhl = buffer as *mut dsprite_hl_t; // make link to extradata
        size =
            (::std::mem::size_of::<msprite_t>() as
                 libc::c_ulong).wrapping_add((((*pinhl).numframes -
                                                   1 as libc::c_int) as
                                                  libc::c_ulong).wrapping_mul(::std::mem::size_of::<[mspriteframedesc_t; 1]>()
                                                                                  as
                                                                                  libc::c_ulong))
                as libc::c_int; // done
        psprite =
            _Mem_Alloc((*mod_0).mempool, size as size_t, true_0,
                       b"../engine/common/mod_sprite.c\x00" as *const u8 as
                           *const libc::c_char, 87 as libc::c_int) as
                *mut msprite_t;
        (*mod_0).cache.data = psprite as *mut libc::c_void;
        (*psprite).type_0 = (*pinhl).type_0 as libc::c_short;
        (*psprite).texFormat = (*pinhl).texFormat as libc::c_short;
        (*mod_0).numframes = (*pinhl).numframes;
        (*psprite).numframes = (*mod_0).numframes;
        (*psprite).facecull = (*pinhl).facetype as libc::c_int;
        (*psprite).radius = (*pinhl).boundingradius;
        (*psprite).synctype = (*pinhl).synctype as libc::c_int;
        (*mod_0).mins[1 as libc::c_int as usize] =
            -(*pinhl).bounds[0 as libc::c_int as usize] as libc::c_float *
                0.5f32;
        (*mod_0).mins[0 as libc::c_int as usize] =
            (*mod_0).mins[1 as libc::c_int as usize];
        (*mod_0).maxs[1 as libc::c_int as usize] =
            (*pinhl).bounds[0 as libc::c_int as usize] as libc::c_float *
                0.5f32;
        (*mod_0).maxs[0 as libc::c_int as usize] =
            (*mod_0).maxs[1 as libc::c_int as usize];
        (*mod_0).mins[2 as libc::c_int as usize] =
            -(*pinhl).bounds[1 as libc::c_int as usize] as libc::c_float *
                0.5f32;
        (*mod_0).maxs[2 as libc::c_int as usize] =
            (*pinhl).bounds[1 as libc::c_int as usize] as libc::c_float *
                0.5f32
    }
    if !loaded.is_null() { *loaded = true_0 }
    if host.type_0 == HOST_DEDICATED as libc::c_int as libc::c_uint {
        // skip frames loading
        (*psprite).numframes = 0 as libc::c_int;
        return
    };
}
