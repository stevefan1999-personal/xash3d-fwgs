#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, const_raw_ptr_to_usize_cast, extern_types,
           ptr_wrapping_offset_from, register_tool)]
extern "C" {
    pub type file_s;
    pub type grasshdr_s;
    pub type IVoiceTweak_s;
    pub type demo_api_s;
    pub type engine_studio_api_s;
    pub type r_studio_interface_s;
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn fabsf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn Q_strlen(string: *const libc::c_char) -> size_t;
    #[no_mangle]
    fn Q_tolower(in_0: libc::c_char) -> libc::c_char;
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Q_atoi(str: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn Q_strnicmp(s1: *const libc::c_char, s2: *const libc::c_char,
                  n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Q_vsnprintf(buffer: *mut libc::c_char, buffersize: size_t,
                   format: *const libc::c_char, args: ::std::ffi::VaList)
     -> libc::c_int;
    #[no_mangle]
    fn Q_snprintf(buffer: *mut libc::c_char, buffersize: size_t,
                  format: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn Q_sprintf(buffer: *mut libc::c_char, format: *const libc::c_char,
                 _: ...) -> libc::c_int;
    #[no_mangle]
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn Sys_DoubleTime() -> libc::c_double;
    #[no_mangle]
    fn Sys_GetClipboardData() -> *mut libc::c_char;
    #[no_mangle]
    fn Sys_CheckParm(parm: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn Sys_Error(error: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Cvar_Get(var_name: *const libc::c_char, value: *const libc::c_char,
                flags: libc::c_int, description: *const libc::c_char)
     -> *mut convar_t;
    #[no_mangle]
    fn Cvar_DirectSet(var: *mut convar_t, value: *const libc::c_char);
    #[no_mangle]
    fn Cvar_VariableInteger(var_name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn Cvar_VariableString(var_name: *const libc::c_char)
     -> *const libc::c_char;
    #[no_mangle]
    static mut scr_download: *mut convar_t;
    #[no_mangle]
    static mut cl_allow_levelshots: *mut convar_t;
    #[no_mangle]
    static mut host_developer: convar_t;
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    static mut SI: sysinfo_t;
    #[no_mangle]
    fn Cbuf_AddText(text: *const libc::c_char);
    #[no_mangle]
    fn Cbuf_AddFilteredText(text: *const libc::c_char);
    #[no_mangle]
    fn Cmd_CurrentCommandIsPrivileged() -> qboolean;
    #[no_mangle]
    fn Cmd_Argc() -> libc::c_int;
    #[no_mangle]
    fn Cmd_Argv(arg: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn Cmd_AddCommand(cmd_name: *const libc::c_char, function: xcommand_t,
                      cmd_desc: *const libc::c_char);
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
    fn CRC32_File(crcvalue: *mut dword, filename: *const libc::c_char)
     -> qboolean;
    #[no_mangle]
    fn FS_Open(filepath: *const libc::c_char, mode: *const libc::c_char,
               gamedironly: qboolean) -> *mut file_t;
    #[no_mangle]
    fn FS_Printf(file: *mut file_t, format: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn FS_FileExists(filename: *const libc::c_char, gamedironly: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn FS_Close(file: *mut file_t) -> libc::c_int;
    #[no_mangle]
    fn Q_buildnum() -> libc::c_int;
    #[no_mangle]
    fn Q_buildos() -> *const libc::c_char;
    #[no_mangle]
    fn Q_buildarch() -> *const libc::c_char;
    #[no_mangle]
    fn SV_Active() -> qboolean;
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_CompleteCommand(field: *mut field_t);
    #[no_mangle]
    fn Cmd_AutoCompleteClear();
    #[no_mangle]
    fn UI_CreditsActive() -> qboolean;
    #[no_mangle]
    fn SCR_UpdateScreen();
    #[no_mangle]
    fn UI_SetActiveMenu(fActive: qboolean);
    #[no_mangle]
    static mut cls: client_static_t;
    #[link_name = "ref"]
    static mut ref_0: ref_state_s;
    #[no_mangle]
    static mut refState: ref_globals_t;
    #[no_mangle]
    fn Key_SetKeyDest(key_dest: libc::c_int);
    #[no_mangle]
    static mut cl: client_t;
    #[no_mangle]
    fn R_GetTextureParms(w: *mut libc::c_int, h: *mut libc::c_int,
                         texnum: libc::c_int);
    #[no_mangle]
    static mut clgame: clgame_static_t;
    #[no_mangle]
    static mut cl_charset: *mut convar_t;
    #[no_mangle]
    static mut net_graph: *mut convar_t;
    #[no_mangle]
    fn CL_IsDevOverviewMode() -> libc::c_int;
    #[no_mangle]
    fn SCR_LoadCreditsFont();
    #[no_mangle]
    fn SCR_DrawFPS(height: libc::c_int);
    #[no_mangle]
    fn Key_IsDown(keynum: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Host_InputFrame();
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
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
pub type va_list = __builtin_va_list;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type C2RustUnnamed = libc::c_uint;
pub const DEV_EXTENDED: C2RustUnnamed = 2;
pub const DEV_NORMAL: C2RustUnnamed = 1;
pub const DEV_NONE: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const HOST_DEDICATED: C2RustUnnamed_0 = 1;
pub const HOST_NORMAL: C2RustUnnamed_0 = 0;
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
pub type C2RustUnnamed_1 = libc::c_uint;
pub const kRenderTransAdd: C2RustUnnamed_1 = 5;
pub const kRenderTransAlpha: C2RustUnnamed_1 = 4;
pub const kRenderGlow: C2RustUnnamed_1 = 3;
pub const kRenderTransTexture: C2RustUnnamed_1 = 2;
pub const kRenderTransColor: C2RustUnnamed_1 = 1;
pub const kRenderNormal: C2RustUnnamed_1 = 0;
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
pub type con_nprint_t = con_nprint_s;
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
pub struct field_t {
    pub buffer: string,
    pub cursor: libc::c_int,
    pub scroll: libc::c_int,
    pub widthInChars: libc::c_int,
}
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
pub type xcommand_t = Option<unsafe extern "C" fn() -> ()>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct console_t {
    pub initialized: qboolean,
    pub buffer: *mut libc::c_char,
    pub bufsize: libc::c_int,
    pub lines: *mut con_lineinfo_t,
    pub maxlines: libc::c_int,
    pub lines_first: libc::c_int,
    pub lines_count: libc::c_int,
    pub num_times: libc::c_int,
    pub backscroll: libc::c_int,
    pub linewidth: libc::c_int,
    pub showlines: libc::c_float,
    pub vislines: libc::c_float,
    pub background: libc::c_int,
    pub chars: [cl_font_t; 3],
    pub curFont: *mut cl_font_t,
    pub lastUsedFont: *mut cl_font_t,
    pub input: field_t,
    pub chat: field_t,
    pub chat_cmd: string,
    pub historyLines: [field_t; 64],
    pub historyLine: libc::c_int,
    pub nextHistoryLine: libc::c_int,
    pub backup: field_t,
    pub notify: [notify_t; 128],
    pub draw_notify: qboolean,
    pub lastupdate: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct notify_t {
    pub szNotify: string,
    pub expire: libc::c_float,
    pub color: rgba_t,
    pub key_dest: libc::c_int,
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
pub type wrect_t = wrect_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wrect_s {
    pub left: libc::c_int,
    pub right: libc::c_int,
    pub top: libc::c_int,
    pub bottom: libc::c_int,
}
pub type con_lineinfo_t = con_lineinfo_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct con_lineinfo_s {
    pub start: *mut libc::c_char,
    pub length: size_t,
    pub addtime: libc::c_double,
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
pub struct downloadtime_t {
    pub bUsed: qboolean,
    pub fTime: libc::c_float,
    pub nBytesRemaining: libc::c_int,
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
pub type predicted_player_t = cl_predicted_player_s;
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
pub type local_state_t = local_state_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct local_state_s {
    pub playerstate: entity_state_t,
    pub client: clientdata_t,
    pub weapondata: [weapon_data_t; 64],
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
pub type connstate_t = connstate_e;
pub type connstate_e = libc::c_uint;
pub const ca_cinematic: connstate_e = 5;
pub const ca_active: connstate_e = 4;
pub const ca_validate: connstate_e = 3;
pub const ca_connected: connstate_e = 2;
pub const ca_connecting: connstate_e = 1;
pub const ca_disconnected: connstate_e = 0;
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
pub struct ref_viewpass_s {
    pub viewport: [libc::c_int; 4],
    pub vieworigin: vec3_t,
    pub viewangles: vec3_t,
    pub viewentity: libc::c_int,
    pub fov_x: libc::c_float,
    pub fov_y: libc::c_float,
    pub flags: libc::c_int,
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
pub struct mip_s {
    pub name: [libc::c_char; 16],
    pub width: libc::c_uint,
    pub height: libc::c_uint,
    pub offsets: [libc::c_uint; 4],
}
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
pub type ref_globals_t = ref_globals_s;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sortedface_t {
    pub surf: *mut msurface_t,
    pub cull: libc::c_int,
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
pub struct pred_viewangle_t {
    pub starttime: libc::c_float,
    pub total: libc::c_float,
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
pub type cmdalias_t = cmdalias_s;
pub type HSPRITE = libc::c_int;
pub type pfnUserMsgHook
    =
    Option<unsafe extern "C" fn(_: *const libc::c_char, _: libc::c_int,
                                _: *mut libc::c_void) -> libc::c_int>;
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
pub type cl_enginefunc_t = cl_enginefuncs_s;
pub type efrag_t = efrag_s;
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
pub struct remap_info_s {
    pub textures: [libc::c_ushort; 32],
    pub ptexture: *mut mstudiotex_s,
    pub numtextures: libc::c_short,
    pub topcolor: libc::c_short,
    pub bottomcolor: libc::c_short,
    pub model: *mut model_t,
}
pub type remap_info_t = remap_info_s;
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
pub type net_response_t = net_response_s;
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
pub type lmp_t = lmp_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lmp_s {
    pub width: libc::c_uint,
    pub height: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct charinfo {
    pub startoffset: libc::c_short,
    pub charwidth: libc::c_short,
}
pub type qfont_t = qfont_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct qfont_s {
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub rowcount: libc::c_int,
    pub rowheight: libc::c_int,
    pub fontinfo: [charinfo; 256],
    pub data: [byte; 4],
}
#[inline(always)]
unsafe extern "C" fn __tg_fabs(mut __x: libc::c_float) -> libc::c_float {
    return fabsf(__x);
}
// notify stuff
/*
console.c - developer console
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
// get the protocol version
#[no_mangle]
pub static mut con_notifytime: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut scr_conspeed: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut con_fontsize: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut con_charset: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut con_fontscale: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut con_fontnum: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut con_color: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
static mut g_codepage: libc::c_int = 0 as libc::c_int;
static mut g_utf8: qboolean = false_0;
static mut g_messagemode_privileged: qboolean = true_0;
// console color typeing
#[no_mangle]
pub static mut g_color_table: [rgba_t; 8] =
    [[0 as libc::c_int as byte, 0 as libc::c_int as byte,
      0 as libc::c_int as byte, 255 as libc::c_int as byte],
     [255 as libc::c_int as byte, 0 as libc::c_int as byte,
      0 as libc::c_int as byte, 255 as libc::c_int as byte],
     [0 as libc::c_int as byte, 255 as libc::c_int as byte,
      0 as libc::c_int as byte, 255 as libc::c_int as byte],
     [255 as libc::c_int as byte, 255 as libc::c_int as byte,
      0 as libc::c_int as byte, 255 as libc::c_int as byte],
     [0 as libc::c_int as byte, 0 as libc::c_int as byte,
      255 as libc::c_int as byte, 255 as libc::c_int as byte],
     [0 as libc::c_int as byte, 255 as libc::c_int as byte,
      255 as libc::c_int as byte, 255 as libc::c_int as byte],
     [255 as libc::c_int as byte, 0 as libc::c_int as byte,
      255 as libc::c_int as byte, 255 as libc::c_int as byte],
     [240 as libc::c_int as byte, 180 as libc::c_int as byte,
      24 as libc::c_int as byte, 255 as libc::c_int as byte]];
static mut con: console_t =
    console_t{initialized: false_0,
              buffer: 0 as *const libc::c_char as *mut libc::c_char,
              bufsize: 0,
              lines: 0 as *const con_lineinfo_t as *mut con_lineinfo_t,
              maxlines: 0,
              lines_first: 0,
              lines_count: 0,
              num_times: 0,
              backscroll: 0,
              linewidth: 0,
              showlines: 0.,
              vislines: 0.,
              background: 0,
              chars:
                  [cl_font_t{hFontTexture: 0,
                             fontRc:
                                 [wrect_t{left: 0,
                                          right: 0,
                                          top: 0,
                                          bottom: 0,}; 256],
                             charWidths: [0; 256],
                             charHeight: 0,
                             type_0: 0,
                             valid: false_0,}; 3],
              curFont: 0 as *const cl_font_t as *mut cl_font_t,
              lastUsedFont: 0 as *const cl_font_t as *mut cl_font_t,
              input:
                  field_t{buffer: [0; 256],
                          cursor: 0,
                          scroll: 0,
                          widthInChars: 0,},
              chat:
                  field_t{buffer: [0; 256],
                          cursor: 0,
                          scroll: 0,
                          widthInChars: 0,},
              chat_cmd: [0; 256],
              historyLines:
                  [field_t{buffer: [0; 256],
                           cursor: 0,
                           scroll: 0,
                           widthInChars: 0,}; 64],
              historyLine: 0,
              nextHistoryLine: 0,
              backup:
                  field_t{buffer: [0; 256],
                          cursor: 0,
                          scroll: 0,
                          widthInChars: 0,},
              notify:
                  [notify_t{szNotify: [0; 256],
                            expire: 0.,
                            color: [0; 4],
                            key_dest: 0,}; 128],
              draw_notify: false_0,
              lastupdate: 0.,};
/*
================
Con_Clear_f
================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_Clear_f() {
    con.lines_count = 0 as libc::c_int;
    con.backscroll = 0 as libc::c_int;
    // go to end
}
/*
================
Con_SetColor
================
*/
unsafe extern "C" fn Con_SetColor() {
    let mut color: vec3_t = [0.; 3];
    let mut r: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    if (*con_color).flags & (1 as libc::c_int) << 13 as libc::c_int == 0 {
        return
    }
    num =
        sscanf((*con_color).string,
               b"%i %i %i\x00" as *const u8 as *const libc::c_char,
               &mut r as *mut libc::c_int, &mut g as *mut libc::c_int,
               &mut b as *mut libc::c_int);
    match num {
        1 => { Con_DefaultColor(r, r, r); }
        3 => { Con_DefaultColor(r, g, b); }
        _ => { Cvar_DirectSet(con_color, (*con_color).def_string); }
    }
    (*con_color).flags =
        (*con_color).flags & !((1 as libc::c_int) << 13 as libc::c_int);
}
/*
================
Con_ClearNotify
================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_ClearNotify() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < con.lines_count {
        (*con.lines.offset(((con.lines_first + i) % con.maxlines) as
                               isize)).addtime = 0.0f64;
        i += 1
    };
}
/*
================
Con_ClearTyping
================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_ClearTyping() {
    let mut i: libc::c_int = 0;
    Con_ClearField(&mut con.input);
    con.input.widthInChars = con.linewidth;
    Cmd_AutoCompleteClear();
}
/*
============
Con_StringLength

skipped color prefixes
============
*/
#[no_mangle]
pub unsafe extern "C" fn Con_StringLength(mut string: *const libc::c_char)
 -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    if string.is_null() { return 0 as libc::c_int }
    len = 0 as libc::c_int;
    p = string;
    while *p != 0 {
        if !p.is_null() && *p as libc::c_int == '^' as i32 &&
               *p.offset(1 as libc::c_int as isize) as libc::c_int != 0 &&
               *p.offset(1 as libc::c_int as isize) as libc::c_int >=
                   '0' as i32 &&
               *p.offset(1 as libc::c_int as isize) as libc::c_int <=
                   '9' as i32 {
            p = p.offset(2 as libc::c_int as isize)
        } else { len += 1; p = p.offset(1) }
    }
    return len;
}
/*
================
Con_MessageMode_f
================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_MessageMode_f() {
    g_messagemode_privileged = Cmd_CurrentCommandIsPrivileged();
    if Cmd_Argc() == 2 as libc::c_int {
        Q_strncpy(con.chat_cmd.as_mut_ptr(), Cmd_Argv(1 as libc::c_int),
                  ::std::mem::size_of::<string>() as libc::c_ulong);
    } else {
        Q_strncpy(con.chat_cmd.as_mut_ptr(),
                  b"say\x00" as *const u8 as *const libc::c_char,
                  ::std::mem::size_of::<string>() as libc::c_ulong);
    }
    Key_SetKeyDest(key_message as libc::c_int);
}
/*
================
Con_MessageMode2_f
================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_MessageMode2_f() {
    g_messagemode_privileged = Cmd_CurrentCommandIsPrivileged();
    Q_strncpy(con.chat_cmd.as_mut_ptr(),
              b"say_team\x00" as *const u8 as *const libc::c_char,
              ::std::mem::size_of::<string>() as libc::c_ulong);
    Key_SetKeyDest(key_message as libc::c_int);
}
/*
================
Con_ToggleConsole_f
================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_ToggleConsole_f() {
    if host.allow_console as u64 == 0 ||
           UI_CreditsActive() as libc::c_uint != 0 {
        return
    } // disabled
    // show console only in game or by special call from menu
    if cls.state as libc::c_uint != ca_active as libc::c_int as libc::c_uint
           ||
           cls.key_dest as libc::c_uint ==
               key_menu as libc::c_int as libc::c_uint {
        return
    }
    Con_ClearTyping();
    Con_ClearNotify();
    if cls.key_dest as libc::c_uint ==
           key_console as libc::c_int as libc::c_uint {
        if Cvar_VariableInteger(b"sv_background\x00" as *const u8 as
                                    *const libc::c_char) != 0 ||
               Cvar_VariableInteger(b"cl_background\x00" as *const u8 as
                                        *const libc::c_char) != 0 {
            UI_SetActiveMenu(true_0);
        } else { UI_SetActiveMenu(false_0); }
    } else {
        UI_SetActiveMenu(false_0);
        Key_SetKeyDest(key_console as libc::c_int);
    };
}
/*
================
Con_SetTimes_f
================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_SetTimes_f() {
    let mut newtimes: libc::c_int = 0;
    if Cmd_Argc() != 2 as libc::c_int {
        Con_Printf(b"Usage: contimes <n lines>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    newtimes = Q_atoi(Cmd_Argv(1 as libc::c_int));
    con.num_times =
        if newtimes >= 4 as libc::c_int {
            if newtimes < 64 as libc::c_int {
                newtimes
            } else { 64 as libc::c_int }
        } else { 4 as libc::c_int };
}
/*
================
Con_FixTimes

Notifies the console code about the current time
(and shifts back times of other entries when the time
went backwards)
================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_FixTimes() {
    let mut diff: libc::c_double = 0.; // nothing to fix
    let mut i: libc::c_int = 0;
    if con.lines_count <= 0 as libc::c_int { return }
    diff =
        cl.time -
            (*con.lines.offset(((con.lines_first +
                                     (con.lines_count - 1 as libc::c_int)) %
                                    con.maxlines) as isize)).addtime;
    if diff >= 0.0f64 { return }
    i = 0 as libc::c_int;
    while i < con.lines_count {
        (*con.lines.offset(((con.lines_first + i) % con.maxlines) as
                               isize)).addtime += diff;
        i += 1
    };
}
/*
================
Con_DeleteLine

Deletes the first line from the console history.
================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_DeleteLine() {
    if con.lines_count == 0 as libc::c_int { return }
    con.lines_count -= 1;
    con.lines_first = (con.lines_first + 1 as libc::c_int) % con.maxlines;
}
/*
================
Con_DeleteLastLine

Deletes the last line from the console history.
================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_DeleteLastLine() {
    if con.lines_count == 0 as libc::c_int { return }
    con.lines_count -= 1;
}
/*
================
Con_BytesLeft

Checks if there is space for a line of the given length, and if yes, returns a
pointer to the start of such a space, and NULL otherwise.
================
*/
unsafe extern "C" fn Con_BytesLeft(mut length: libc::c_int)
 -> *mut libc::c_char {
    if length > con.bufsize { return 0 as *mut libc::c_char }
    if con.lines_count == 0 as libc::c_int {
        return con.buffer
    } else {
        let mut firstline_start: *mut libc::c_char =
            (*con.lines.offset(con.lines_first as isize)).start;
        let mut lastline_onepastend: *mut libc::c_char =
            (*con.lines.offset(((con.lines_first +
                                     (con.lines_count - 1 as libc::c_int)) %
                                    con.maxlines) as
                                   isize)).start.offset((*con.lines.offset(((con.lines_first
                                                                                 +
                                                                                 (con.lines_count
                                                                                      -
                                                                                      1
                                                                                          as
                                                                                          libc::c_int))
                                                                                %
                                                                                con.maxlines)
                                                                               as
                                                                               isize)).length
                                                            as isize);
        // the buffer is cyclic, so we first have two cases...
        if firstline_start < lastline_onepastend {
            // buffer is contiguous
            // put at end?
            if length as libc::c_long <=
                   con.buffer.offset(con.bufsize as
                                         isize).wrapping_offset_from(lastline_onepastend)
                       as libc::c_long {
                return lastline_onepastend
            } else {
                // put at beginning?
                if length as libc::c_long <=
                       firstline_start.wrapping_offset_from(con.buffer) as
                           libc::c_long {
                    return con.buffer
                }
            }
            return 0 as *mut libc::c_char
        } else {
            // buffer has a contiguous hole
            if length as libc::c_long <=
                   firstline_start.wrapping_offset_from(lastline_onepastend)
                       as libc::c_long {
                return lastline_onepastend
            }
            return 0 as *mut libc::c_char
        }
    };
}
/*
================
Con_AddLine

Appends a given string as a new line to the console.
================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_AddLine(mut line: *const libc::c_char,
                                     mut length: libc::c_int,
                                     mut newline: qboolean) {
    let mut putpos: *mut libc::c_char =
        0 as *mut libc::c_char; // reserve space for term
    let mut p: *mut con_lineinfo_t = 0 as *mut con_lineinfo_t;
    if con.initialized as u64 == 0 || con.buffer.is_null() { return }
    Con_FixTimes();
    length += 1;
    if !(length < 1048576 as libc::c_int) {
        Sys_Error(b"assert failed at %s:%i\n\x00" as *const u8 as
                      *const libc::c_char,
                  b"../engine/client/console.c\x00" as *const u8 as
                      *const libc::c_char, 433 as libc::c_int);
    }
    loop  {
        putpos = Con_BytesLeft(length);
        if !(putpos.is_null() || con.lines_count >= con.maxlines) { break ; }
        Con_DeleteLine();
    }
    if newline as u64 != 0 {
        memcpy(putpos as *mut libc::c_void, line as *const libc::c_void,
               length as libc::c_ulong);
        *putpos.offset((length - 1 as libc::c_int) as isize) =
            '\u{0}' as i32 as libc::c_char;
        con.lines_count += 1;
        p =
            &mut *con.lines.offset(((con.lines_first +
                                         (con.lines_count - 1 as libc::c_int))
                                        % con.maxlines) as isize) as
                *mut con_lineinfo_t;
        (*p).start = putpos;
        (*p).length = length as size_t;
        (*p).addtime = cl.time
    } else {
        p =
            &mut *con.lines.offset(((con.lines_first +
                                         (con.lines_count - 1 as libc::c_int))
                                        % con.maxlines) as isize) as
                *mut con_lineinfo_t;
        putpos = (*p).start.offset(Q_strlen((*p).start) as isize);
        memcpy(putpos as *mut libc::c_void, line as *const libc::c_void,
               (length - 1 as libc::c_int) as libc::c_ulong);
        (*p).length = Q_strlen((*p).start);
        *putpos.offset((*p).length as isize) = '\u{0}' as i32 as libc::c_char;
        (*p).addtime = cl.time;
        (*p).length = (*p).length.wrapping_add(1)
    };
}
/*
================
Con_CheckResize

If the line width has changed, reformat the buffer.
================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_CheckResize() {
    let mut charWidth: libc::c_int = 8 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    if !con.curFont.is_null() && (*con.curFont).hFontTexture != 0 {
        charWidth =
            (*con.curFont).charWidths['O' as i32 as usize] as libc::c_int -
                1 as libc::c_int
    }
    width = refState.width / charWidth - 2 as libc::c_int;
    if ref_0.initialized as u64 == 0 {
        width = 640 as libc::c_int / 5 as libc::c_int
    }
    if width == con.linewidth { return }
    Con_ClearNotify();
    con.linewidth = width;
    con.backscroll = 0 as libc::c_int;
    con.input.widthInChars = con.linewidth;
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        con.historyLines[i as usize].widthInChars = con.linewidth;
        i += 1
    };
}
/*
================
Con_PageUp
================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_PageUp(mut lines: libc::c_int) {
    con.backscroll += abs(lines);
}
/*
================
Con_PageDown
================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_PageDown(mut lines: libc::c_int) {
    con.backscroll -= abs(lines);
}
/*
================
Con_Top
================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_Top() { con.backscroll = 16384 as libc::c_int; }
/*
================
Con_Bottom
================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_Bottom() { con.backscroll = 0 as libc::c_int; }
/*
================
Con_Visible
================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_Visible() -> libc::c_int {
    return (con.vislines > 0 as libc::c_int as libc::c_float) as libc::c_int;
}
/*
================
Con_FixedFont
================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_FixedFont() -> qboolean {
    if !con.curFont.is_null() && (*con.curFont).valid as libc::c_uint != 0 &&
           (*con.curFont).type_0 == 0 as libc::c_int {
        return true_0
    } // already loaded
    return false_0;
}
unsafe extern "C" fn Con_LoadFixedWidthFont(mut fontname: *const libc::c_char,
                                            mut font: *mut cl_font_t)
 -> qboolean {
    let mut i: libc::c_int = 0;
    let mut fontWidth: libc::c_int = 0;
    if (*font).valid as u64 != 0 { return true_0 }
    if FS_FileExists(fontname, false_0 as libc::c_int) == 0 { return false_0 }
    // keep source to print directly into conback image
    (*font).hFontTexture =
        ref_0.dllFuncs.GL_LoadTexture.expect("non-null function pointer")(fontname,
                                                                          0 as
                                                                              *const byte,
                                                                          0 as
                                                                              libc::c_int
                                                                              as
                                                                              size_t,
                                                                          TF_NOMIPMAP
                                                                              as
                                                                              libc::c_int
                                                                              |
                                                                              TF_CLAMP
                                                                                  as
                                                                                  libc::c_int
                                                                              |
                                                                              TF_KEEP_SOURCE
                                                                                  as
                                                                                  libc::c_int);
    R_GetTextureParms(&mut fontWidth, 0 as *mut libc::c_int,
                      (*font).hFontTexture);
    if (*font).hFontTexture != 0 && fontWidth != 0 as libc::c_int {
        (*font).charHeight =
            ((fontWidth / 16 as libc::c_int) as libc::c_float *
                 (*con_fontscale).value) as libc::c_int;
        (*font).type_0 = 0 as libc::c_int;
        // build fixed rectangles
        i = 0 as libc::c_int; // already loaded
        while i < 256 as libc::c_int {
            (*font).fontRc[i as usize].left =
                i * (fontWidth / 16 as libc::c_int) % fontWidth;
            (*font).fontRc[i as usize].right =
                (*font).fontRc[i as usize].left +
                    fontWidth / 16 as libc::c_int;
            (*font).fontRc[i as usize].top =
                i / 16 as libc::c_int * (fontWidth / 16 as libc::c_int);
            (*font).fontRc[i as usize].bottom =
                (*font).fontRc[i as usize].top +
                    fontWidth / 16 as libc::c_int;
            (*font).charWidths[i as usize] =
                ((fontWidth / 16 as libc::c_int) as libc::c_float *
                     (*con_fontscale).value) as byte;
            i += 1
        }
        (*font).valid = true_0
    }
    return true_0;
}
unsafe extern "C" fn Con_LoadVariableWidthFont(mut fontname:
                                                   *const libc::c_char,
                                               mut font: *mut cl_font_t)
 -> qboolean {
    let mut i: libc::c_int = 0;
    let mut fontWidth: libc::c_int = 0;
    let mut buffer: *mut byte = 0 as *mut byte;
    let mut length: fs_offset_t = 0;
    let mut src: *mut qfont_t = 0 as *mut qfont_t;
    if (*font).valid as u64 != 0 { return true_0 }
    if FS_FileExists(fontname, false_0 as libc::c_int) == 0 { return false_0 }
    (*font).hFontTexture =
        ref_0.dllFuncs.GL_LoadTexture.expect("non-null function pointer")(fontname,
                                                                          0 as
                                                                              *const byte,
                                                                          0 as
                                                                              libc::c_int
                                                                              as
                                                                              size_t,
                                                                          TF_NOMIPMAP
                                                                              as
                                                                              libc::c_int
                                                                              |
                                                                              TF_CLAMP
                                                                                  as
                                                                                  libc::c_int
                                                                              |
                                                                              TF_NEAREST
                                                                                  as
                                                                                  libc::c_int);
    R_GetTextureParms(&mut fontWidth, 0 as *mut libc::c_int,
                      (*font).hFontTexture);
    // setup consolefont
    if (*font).hFontTexture != 0 && fontWidth != 0 as libc::c_int {
        // half-life font with variable chars witdh
        buffer = FS_LoadFile(fontname, &mut length, false_0);
        if !buffer.is_null() &&
               length as libc::c_ulong >=
                   ::std::mem::size_of::<qfont_t>() as libc::c_ulong {
            src = buffer as *mut qfont_t;
            (*font).charHeight =
                ((*src).rowheight as libc::c_float * (*con_fontscale).value)
                    as libc::c_int;
            (*font).type_0 = 1 as libc::c_int;
            // build rectangles
            i = 0 as libc::c_int;
            while i < 256 as libc::c_int {
                (*font).fontRc[i as usize].left =
                    (*src).fontinfo[i as usize].startoffset as word as
                        libc::c_int % fontWidth;
                (*font).fontRc[i as usize].right =
                    (*font).fontRc[i as usize].left +
                        (*src).fontinfo[i as usize].charwidth as libc::c_int;
                (*font).fontRc[i as usize].top =
                    (*src).fontinfo[i as usize].startoffset as word as
                        libc::c_int / fontWidth;
                (*font).fontRc[i as usize].bottom =
                    (*font).fontRc[i as usize].top + (*src).rowheight;
                (*font).charWidths[i as usize] =
                    ((*src).fontinfo[i as usize].charwidth as libc::c_int as
                         libc::c_float * (*con_fontscale).value) as byte;
                i += 1
            }
            (*font).valid = true_0
        }
        if !buffer.is_null() {
            _Mem_Free(buffer as *mut libc::c_void,
                      b"../engine/client/console.c\x00" as *const u8 as
                          *const libc::c_char, 627 as libc::c_int);
        }
    }
    return true_0;
}
/*
================
Con_LoadConsoleFont

INTERNAL RESOURCE
================
*/
unsafe extern "C" fn Con_LoadConsoleFont(mut fontNumber: libc::c_int,
                                         mut font: *mut cl_font_t) {
    let mut path: *const libc::c_char =
        0 as *const libc::c_char; // already loaded
    let mut crc: dword = 0 as libc::c_int as dword;
    if (*font).valid as u64 != 0 { return }
    // replace default fonts.wad textures by current charset's font
    if CRC32_File(&mut crc,
                  b"fonts.wad\x00" as *const u8 as *const libc::c_char) as u64
           == 0 || crc == 0x3c0a0029 as libc::c_int as libc::c_uint {
        let mut path2: *const libc::c_char =
            va(b"font%i_%s.fnt\x00" as *const u8 as *const libc::c_char,
               fontNumber,
               Cvar_VariableString(b"con_charset\x00" as *const u8 as
                                       *const libc::c_char));
        if FS_FileExists(path2, false_0 as libc::c_int) != 0 { path = path2 }
    }
    // loading conchars
    if Sys_CheckParm(b"-oldfont\x00" as *const u8 as *const libc::c_char) != 0
       {
        Con_LoadVariableWidthFont(b"gfx/conchars.fnt\x00" as *const u8 as
                                      *const libc::c_char, font);
    } else {
        if path.is_null() {
            path =
                va(b"fonts/font%i\x00" as *const u8 as *const libc::c_char,
                   fontNumber)
        }
        Con_LoadVariableWidthFont(path, font);
    }
    // quake fixed font as fallback
    if (*font).valid as u64 == 0 {
        Con_LoadFixedWidthFont(b"gfx/conchars\x00" as *const u8 as
                                   *const libc::c_char, font);
    };
}
/*
================
Con_LoadConchars
================
*/
unsafe extern "C" fn Con_LoadConchars() {
    let mut i: libc::c_int = 0;
    let mut fontSize: libc::c_int = 0;
    // load all the console fonts
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        Con_LoadConsoleFont(i, con.chars.as_mut_ptr().offset(i as isize));
        i += 1
    }
    // select properly fontsize
    if (*con_fontnum).value >= 0 as libc::c_int as libc::c_float &&
           (*con_fontnum).value <=
               (3 as libc::c_int - 1 as libc::c_int) as libc::c_float {
        fontSize = (*con_fontnum).value as libc::c_int
    } else if refState.width <= 640 as libc::c_int {
        fontSize = 0 as libc::c_int
    } else if refState.width >= 1280 as libc::c_int {
        fontSize = 2 as libc::c_int
    } else { fontSize = 1 as libc::c_int }
    if fontSize > 3 as libc::c_int - 1 as libc::c_int {
        fontSize = 3 as libc::c_int - 1 as libc::c_int
    }
    // sets the current font
    con.curFont =
        &mut *con.chars.as_mut_ptr().offset(fontSize as isize) as
            *mut cl_font_t;
    con.lastUsedFont = con.curFont;
}
// CP1251 table
#[no_mangle]
pub static mut table_cp1251: [libc::c_int; 64] =
    [0x402 as libc::c_int, 0x403 as libc::c_int, 0x201a as libc::c_int,
     0x453 as libc::c_int, 0x201e as libc::c_int, 0x2026 as libc::c_int,
     0x2020 as libc::c_int, 0x2021 as libc::c_int, 0x20ac as libc::c_int,
     0x2030 as libc::c_int, 0x409 as libc::c_int, 0x2039 as libc::c_int,
     0x40a as libc::c_int, 0x40c as libc::c_int, 0x40b as libc::c_int,
     0x40f as libc::c_int, 0x452 as libc::c_int, 0x2018 as libc::c_int,
     0x2019 as libc::c_int, 0x201c as libc::c_int, 0x201d as libc::c_int,
     0x2022 as libc::c_int, 0x2013 as libc::c_int, 0x2014 as libc::c_int,
     0x7f as libc::c_int, 0x2122 as libc::c_int, 0x459 as libc::c_int,
     0x203a as libc::c_int, 0x45a as libc::c_int, 0x45c as libc::c_int,
     0x45b as libc::c_int, 0x45f as libc::c_int, 0xa0 as libc::c_int,
     0x40e as libc::c_int, 0x45e as libc::c_int, 0x408 as libc::c_int,
     0xa4 as libc::c_int, 0x490 as libc::c_int, 0xa6 as libc::c_int,
     0xa7 as libc::c_int, 0x401 as libc::c_int, 0xa9 as libc::c_int,
     0x404 as libc::c_int, 0xab as libc::c_int, 0xac as libc::c_int,
     0xad as libc::c_int, 0xae as libc::c_int, 0x407 as libc::c_int,
     0xb0 as libc::c_int, 0xb1 as libc::c_int, 0x406 as libc::c_int,
     0x456 as libc::c_int, 0x491 as libc::c_int, 0xb5 as libc::c_int,
     0xb6 as libc::c_int, 0xb7 as libc::c_int, 0x451 as libc::c_int,
     0x2116 as libc::c_int, 0x454 as libc::c_int, 0xbb as libc::c_int,
     0x458 as libc::c_int, 0x405 as libc::c_int, 0x455 as libc::c_int,
     0x457 as libc::c_int];
/*
============================
Con_UtfProcessChar

Convert utf char to current font's single-byte encoding
============================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_UtfProcessCharForce(mut in_0: libc::c_int)
 -> libc::c_int {
    static mut m: libc::c_int = -(1 as libc::c_int); //multibyte state
    static mut k: libc::c_int = 0 as libc::c_int; //unicode char
    static mut uc: libc::c_int = 0 as libc::c_int;
    if in_0 == 0 {
        m = -(1 as libc::c_int);
        k = 0 as libc::c_int;
        uc = 0 as libc::c_int;
        return 0 as libc::c_int
    }
    // Get character length
    if m == -(1 as libc::c_int) {
        uc = 0 as libc::c_int; //ascii
        if in_0 >= 0xf8 as libc::c_int {
            return 0 as libc::c_int
        } else {
            if in_0 >= 0xf0 as libc::c_int {
                uc = in_0 & 0x7 as libc::c_int;
                m = 3 as libc::c_int
            } else if in_0 >= 0xe0 as libc::c_int {
                uc = in_0 & 0xf as libc::c_int;
                m = 2 as libc::c_int
            } else if in_0 >= 0xc0 as libc::c_int {
                uc = in_0 & 0x1f as libc::c_int;
                m = 1 as libc::c_int
            } else if in_0 <= 0x7f as libc::c_int { return in_0 }
        }
        // return 0 if we need more chars to decode one
        k = 0 as libc::c_int;
        return 0 as libc::c_int
    } else {
        // get more chars
        if k <= m {
            uc <<= 6 as libc::c_int;
            uc += in_0 & 0x3f as libc::c_int;
            k += 1
        }
    }
    if in_0 > 0xbf as libc::c_int || m < 0 as libc::c_int {
        m = -(1 as libc::c_int);
        return 0 as libc::c_int
    }
    if k == m {
        m = -(1 as libc::c_int);
        k = m;
        if g_codepage == 1251 as libc::c_int {
            // cp1251 now
            if uc >= 0x410 as libc::c_int && uc <= 0x42f as libc::c_int {
                return uc - 0x410 as libc::c_int + 0xc0 as libc::c_int
            }
            if uc >= 0x430 as libc::c_int && uc <= 0x44f as libc::c_int {
                return uc - 0x430 as libc::c_int + 0xe0 as libc::c_int
            } else {
                let mut i: libc::c_int = 0;
                i = 0 as libc::c_int;
                while i < 64 as libc::c_int {
                    if table_cp1251[i as usize] == uc {
                        return i + 0x80 as libc::c_int
                    }
                    i += 1
                }
            }
        } else if g_codepage == 1252 as libc::c_int {
            if uc < 255 as libc::c_int { return uc }
        }
        // not implemented yet
        return '?' as i32
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Con_UtfProcessChar(mut in_0: libc::c_int)
 -> libc::c_int {
    if g_utf8 as u64 == 0 {
        return in_0
    } else { return Con_UtfProcessCharForce(in_0) };
}
/*
=================
Con_UtfMoveLeft

get position of previous printful char
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_UtfMoveLeft(mut str: *mut libc::c_char,
                                         mut pos: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0 as libc::c_int;
    // int j;
    if g_utf8 as u64 == 0 { return pos - 1 as libc::c_int }
    Con_UtfProcessChar(0 as libc::c_int);
    if pos == 1 as libc::c_int { return 0 as libc::c_int }
    i = 0 as libc::c_int;
    while i < pos - 1 as libc::c_int {
        if Con_UtfProcessChar(*str.offset(i as isize) as libc::c_uchar as
                                  libc::c_int) != 0 {
            k = i + 1 as libc::c_int
        }
        i += 1
    }
    Con_UtfProcessChar(0 as libc::c_int);
    return k;
}
/*
=================
Con_UtfMoveRight

get next of previous printful char
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_UtfMoveRight(mut str: *mut libc::c_char,
                                          mut pos: libc::c_int,
                                          mut length: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    if g_utf8 as u64 == 0 { return pos + 1 as libc::c_int }
    Con_UtfProcessChar(0 as libc::c_int);
    i = pos;
    while i <= length {
        if Con_UtfProcessChar(*str.offset(i as isize) as libc::c_uchar as
                                  libc::c_int) != 0 {
            return i + 1 as libc::c_int
        }
        i += 1
    }
    Con_UtfProcessChar(0 as libc::c_int);
    return pos + 1 as libc::c_int;
}
unsafe extern "C" fn Con_DrawCharToConback(mut num: libc::c_int,
                                           mut conchars: *const byte,
                                           mut dest: *mut byte) {
    let mut row: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut source: *const byte = 0 as *const byte;
    let mut drawline: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    row = num >> 4 as libc::c_int;
    col = num & 15 as libc::c_int;
    source =
        conchars.offset((row << 10 as libc::c_int) as
                            isize).offset((col << 3 as libc::c_int) as isize);
    drawline = 8 as libc::c_int;
    loop  {
        let fresh0 = drawline;
        drawline = drawline - 1;
        if !(fresh0 != 0) { break ; }
        x = 0 as libc::c_int;
        while x < 8 as libc::c_int {
            if *source.offset(x as isize) as libc::c_int != 255 as libc::c_int
               {
                *dest.offset(x as isize) =
                    (0x60 as libc::c_int +
                         *source.offset(x as isize) as libc::c_int) as byte
            }
            x += 1
        }
        source = source.offset(128 as libc::c_int as isize);
        dest = dest.offset(320 as libc::c_int as isize)
    };
}
/*
====================
Con_TextAdjustSize

draw charcters routine
====================
*/
unsafe extern "C" fn Con_TextAdjustSize(mut x: *mut libc::c_int,
                                        mut y: *mut libc::c_int,
                                        mut w: *mut libc::c_int,
                                        mut h: *mut libc::c_int) {
    let mut xscale: libc::c_float = 0.;
    let mut yscale: libc::c_float = 0.;
    if x.is_null() && y.is_null() && w.is_null() && h.is_null() { return }
    // scale for screen sizes
    xscale =
        refState.width as libc::c_float /
            clgame.scrInfo.iWidth as libc::c_float;
    yscale =
        refState.height as libc::c_float /
            clgame.scrInfo.iHeight as libc::c_float;
    if !x.is_null() { *x = (*x as libc::c_float * xscale) as libc::c_int }
    if !y.is_null() { *y = (*y as libc::c_float * yscale) as libc::c_int }
    if !w.is_null() { *w = (*w as libc::c_float * xscale) as libc::c_int }
    if !h.is_null() { *h = (*h as libc::c_float * yscale) as libc::c_int };
}
/*
====================
Con_DrawGenericChar

draw console single character
====================
*/
unsafe extern "C" fn Con_DrawGenericChar(mut x: libc::c_int,
                                         mut y: libc::c_int,
                                         mut number: libc::c_int,
                                         mut color: *mut byte)
 -> libc::c_int {
    let mut width: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    let mut s1: libc::c_float = 0.;
    let mut t1: libc::c_float = 0.;
    let mut s2: libc::c_float = 0.;
    let mut t2: libc::c_float = 0.;
    let mut rc: *mut wrect_t = 0 as *mut wrect_t;
    number &= 255 as libc::c_int;
    if con.curFont.is_null() || (*con.curFont).valid as u64 == 0 {
        return 0 as libc::c_int
    }
    number = Con_UtfProcessChar(number);
    if number == 0 { return 0 as libc::c_int }
    if y < -(*con.curFont).charHeight { return 0 as libc::c_int }
    rc =
        &mut *(*con.curFont).fontRc.as_mut_ptr().offset(number as isize) as
            *mut wrect_t;
    R_GetTextureParms(&mut width, &mut height, (*con.curFont).hFontTexture);
    if width == 0 || height == 0 {
        return (*con.curFont).charWidths[number as usize] as libc::c_int
    }
    // don't apply color to fixed fonts it's already colored
    if (*con.curFont).type_0 != 0 as libc::c_int ||
           ref_0.dllFuncs.RefGetParm.expect("non-null function pointer")(13 as
                                                                             libc::c_int,
                                                                         (*con.curFont).hFontTexture)
               == 0x8045 as libc::c_int {
        // GL_LUMINANCE8_ALPHA8
        ref_0.dllFuncs.Color4ub.expect("non-null function pointer")(*color.offset(0
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      isize),
                                                                    *color.offset(1
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      isize),
                                                                    *color.offset(2
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      isize),
                                                                    *color.offset(3
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      isize));
    } else {
        ref_0.dllFuncs.Color4ub.expect("non-null function pointer")(255 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uchar,
                                                                    255 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uchar,
                                                                    255 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uchar,
                                                                    *color.offset(3
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      isize));
    }
    // calc rectangle
    s1 =
        (*rc).left as libc::c_float /
            width as libc::c_float; // don't forget reset color
    t1 = (*rc).top as libc::c_float / height as libc::c_float;
    s2 = (*rc).right as libc::c_float / width as libc::c_float;
    t2 = (*rc).bottom as libc::c_float / height as libc::c_float;
    width =
        (((*rc).right - (*rc).left) as libc::c_float * (*con_fontscale).value)
            as libc::c_int;
    height =
        (((*rc).bottom - (*rc).top) as libc::c_float * (*con_fontscale).value)
            as libc::c_int;
    if clgame.ds.adjust_size as u64 != 0 {
        Con_TextAdjustSize(&mut x, &mut y, &mut width, &mut height);
    }
    ref_0.dllFuncs.R_DrawStretchPic.expect("non-null function pointer")(x as
                                                                            libc::c_float,
                                                                        y as
                                                                            libc::c_float,
                                                                        width
                                                                            as
                                                                            libc::c_float,
                                                                        height
                                                                            as
                                                                            libc::c_float,
                                                                        s1,
                                                                        t1,
                                                                        s2,
                                                                        t2,
                                                                        (*con.curFont).hFontTexture);
    ref_0.dllFuncs.Color4ub.expect("non-null function pointer")(255 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uchar,
                                                                255 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uchar,
                                                                255 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uchar,
                                                                255 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uchar);
    return (*con.curFont).charWidths[number as usize] as libc::c_int;
}
/*
====================
Con_SetFont

choose font size
====================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_SetFont(mut fontNum: libc::c_int) {
    fontNum =
        if fontNum >= 0 as libc::c_int {
            if fontNum < 3 as libc::c_int - 1 as libc::c_int {
                fontNum
            } else { (3 as libc::c_int) - 1 as libc::c_int }
        } else { 0 as libc::c_int };
    con.curFont =
        &mut *con.chars.as_mut_ptr().offset(fontNum as isize) as
            *mut cl_font_t;
}
/*
====================
Con_RestoreFont

restore auto-selected console font
(that based on screen resolution)
====================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_RestoreFont() { con.curFont = con.lastUsedFont; }
/*
====================
Con_DrawCharacter

client version of routine
====================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_DrawCharacter(mut x: libc::c_int,
                                           mut y: libc::c_int,
                                           mut number: libc::c_int,
                                           mut color: *mut byte)
 -> libc::c_int {
    ref_0.dllFuncs.GL_SetRenderMode.expect("non-null function pointer")(kRenderTransTexture
                                                                            as
                                                                            libc::c_int);
    return Con_DrawGenericChar(x, y, number, color);
}
/*
====================
Con_DrawCharacterLen

returns character sizes in screen pixels
====================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_DrawCharacterLen(mut number: libc::c_int,
                                              mut width: *mut libc::c_int,
                                              mut height: *mut libc::c_int) {
    if !width.is_null() && !con.curFont.is_null() {
        *width = (*con.curFont).charWidths[number as usize] as libc::c_int
    }
    if !height.is_null() && !con.curFont.is_null() {
        *height = (*con.curFont).charHeight
    };
}
/*
====================
Con_DrawStringLen

compute string width and height in screen pixels
====================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_DrawStringLen(mut pText: *const libc::c_char,
                                           mut length: *mut libc::c_int,
                                           mut height: *mut libc::c_int) {
    let mut curLength: libc::c_int = 0 as libc::c_int;
    if con.curFont.is_null() { return }
    if !height.is_null() { *height = (*con.curFont).charHeight }
    if length.is_null() { return }
    *length = 0 as libc::c_int;
    while *pText != 0 {
        let mut c: byte = *pText as byte;
        if *pText as libc::c_int == '\n' as i32 {
            pText = pText.offset(1);
            curLength = 0 as libc::c_int
        }
        // skip color strings they are not drawing
        if !pText.is_null() && *pText as libc::c_int == '^' as i32 &&
               *pText.offset(1 as libc::c_int as isize) as libc::c_int != 0 &&
               *pText.offset(1 as libc::c_int as isize) as libc::c_int >=
                   '0' as i32 &&
               *pText.offset(1 as libc::c_int as isize) as libc::c_int <=
                   '9' as i32 {
            pText = pText.offset(2 as libc::c_int as isize)
        } else {
            // Convert to unicode
            c = Con_UtfProcessChar(c as libc::c_int) as byte;
            if c != 0 {
                curLength +=
                    (*con.curFont).charWidths[c as usize] as libc::c_int
            }
            pText = pText.offset(1);
            if curLength > *length { *length = curLength }
        }
    };
}
/*
==================
Con_DrawString

Draws a multi-colored string, optionally forcing
to a fixed color.
==================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_DrawGenericString(mut x: libc::c_int,
                                               mut y: libc::c_int,
                                               mut string:
                                                   *const libc::c_char,
                                               mut setColor: *mut byte,
                                               mut forceColor: qboolean,
                                               mut hideChar: libc::c_int)
 -> libc::c_int {
    let mut color: rgba_t = [0; 4]; // no font set
    let mut drawLen: libc::c_int = 0 as libc::c_int;
    let mut numDraws: libc::c_int = 0 as libc::c_int;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    if con.curFont.is_null() { return 0 as libc::c_int }
    Con_UtfProcessChar(0 as libc::c_int);
    // draw the colored text
    *(color.as_mut_ptr() as *mut uint) =
        *(setColor as *mut uint); // at end the string
    s = string; // begin new row
    while *s != 0 {
        if *s as libc::c_int == '\n' as i32 {
            s = s.offset(1);
            if *s == 0 { break ; }
            drawLen = 0 as libc::c_int;
            y += (*con.curFont).charHeight
        }
        if !s.is_null() && *s as libc::c_int == '^' as i32 &&
               *s.offset(1 as libc::c_int as isize) as libc::c_int != 0 &&
               *s.offset(1 as libc::c_int as isize) as libc::c_int >=
                   '0' as i32 &&
               *s.offset(1 as libc::c_int as isize) as libc::c_int <=
                   '9' as i32 {
            if forceColor as u64 == 0 {
                memcpy(color.as_mut_ptr() as *mut libc::c_void,
                       g_color_table[(*s.offset(1 as libc::c_int as isize) as
                                          libc::c_int - '0' as i32 &
                                          7 as libc::c_int) as
                                         usize].as_mut_ptr() as
                           *const libc::c_void,
                       ::std::mem::size_of::<rgba_t>() as libc::c_ulong);
                color[3 as libc::c_int as usize] =
                    *setColor.offset(3 as libc::c_int as isize)
            }
            s = s.offset(2 as libc::c_int as isize);
            numDraws += 1
        } else {
            // hide char for overstrike mode
            if hideChar == numDraws {
                drawLen +=
                    (*con.curFont).charWidths[*s as usize] as libc::c_int
            } else {
                drawLen +=
                    Con_DrawCharacter(x + drawLen, y, *s as libc::c_int,
                                      color.as_mut_ptr())
            }
            numDraws += 1;
            s = s.offset(1)
        }
    }
    ref_0.dllFuncs.Color4ub.expect("non-null function pointer")(255 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uchar,
                                                                255 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uchar,
                                                                255 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uchar,
                                                                255 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uchar);
    return drawLen;
}
/*
====================
Con_DrawString

client version of routine
====================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_DrawString(mut x: libc::c_int,
                                        mut y: libc::c_int,
                                        mut string: *const libc::c_char,
                                        mut setColor: *mut byte)
 -> libc::c_int {
    return Con_DrawGenericString(x, y, string, setColor, false_0,
                                 -(1 as libc::c_int));
}
#[no_mangle]
pub unsafe extern "C" fn Con_LoadHistory() {
    let mut aFile: *const byte =
        FS_LoadFile(b"console_history.txt\x00" as *const u8 as
                        *const libc::c_char, 0 as *mut fs_offset_t, true_0);
    let mut pLine: *const libc::c_char = aFile as *mut libc::c_char;
    let mut pFile: *const libc::c_char = aFile as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    if aFile.is_null() { return }
    while true_0 as libc::c_int != 0 {
        if *pFile == 0 { break ; }
        if *pFile as libc::c_int == '\n' as i32 {
            let mut len: libc::c_int =
                (pFile.wrapping_offset_from(pLine) as libc::c_long +
                     1 as libc::c_int as libc::c_long) as libc::c_int;
            let mut f: *mut field_t = 0 as *mut field_t;
            if len > 255 as libc::c_int { len = 255 as libc::c_int }
            Con_ClearField(&mut *con.historyLines.as_mut_ptr().offset(con.nextHistoryLine
                                                                          as
                                                                          isize));
            f =
                &mut *con.historyLines.as_mut_ptr().offset((con.nextHistoryLine
                                                                %
                                                                64 as
                                                                    libc::c_int)
                                                               as isize) as
                    *mut field_t;
            (*f).widthInChars = con.linewidth;
            (*f).cursor = len - 1 as libc::c_int;
            Q_strncpy((*f).buffer.as_mut_ptr(), pLine, len as size_t);
            con.nextHistoryLine += 1;
            pLine = pFile.offset(1 as libc::c_int as isize)
        }
        pFile = pFile.offset(1)
    }
    i = con.nextHistoryLine;
    while i < 64 as libc::c_int {
        Con_ClearField(&mut *con.historyLines.as_mut_ptr().offset(i as
                                                                      isize));
        con.historyLines[i as usize].widthInChars = con.linewidth;
        i += 1
    }
    con.historyLine = con.nextHistoryLine;
}
#[no_mangle]
pub unsafe extern "C" fn Con_SaveHistory() {
    let mut historyStart: libc::c_int =
        con.nextHistoryLine - 64 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut f: *mut file_t = 0 as *mut file_t;
    if historyStart < 0 as libc::c_int { historyStart = 0 as libc::c_int }
    f =
        FS_Open(b"console_history.txt\x00" as *const u8 as
                    *const libc::c_char,
                b"w\x00" as *const u8 as *const libc::c_char, true_0);
    i = historyStart;
    while i < con.nextHistoryLine {
        FS_Printf(f, b"%s\n\x00" as *const u8 as *const libc::c_char,
                  con.historyLines[(i % 64 as libc::c_int) as
                                       usize].buffer.as_mut_ptr());
        i += 1
    }
    FS_Close(f);
}
/*
================
Con_Init
================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_Init() {
    let mut i: libc::c_int = 0; // dedicated server already have console
    if host.type_0 == HOST_DEDICATED as libc::c_int as libc::c_uint { return }
    // must be init before startup video subsystem
    scr_conspeed =
        Cvar_Get(b"scr_conspeed\x00" as *const u8 as *const libc::c_char,
                 b"600\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int,
                 b"console moving speed\x00" as *const u8 as
                     *const libc::c_char);
    con_notifytime =
        Cvar_Get(b"con_notifytime\x00" as *const u8 as *const libc::c_char,
                 b"3\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int,
                 b"notify time to live\x00" as *const u8 as
                     *const libc::c_char);
    con_fontsize =
        Cvar_Get(b"con_fontsize\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int,
                 b"console font number (0, 1 or 2)\x00" as *const u8 as
                     *const libc::c_char);
    con_charset =
        Cvar_Get(b"con_charset\x00" as *const u8 as *const libc::c_char,
                 b"cp1251\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int,
                 b"console font charset (only cp1251 supported now)\x00" as
                     *const u8 as *const libc::c_char);
    con_fontscale =
        Cvar_Get(b"con_fontscale\x00" as *const u8 as *const libc::c_char,
                 b"1.0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int,
                 b"scale font texture\x00" as *const u8 as
                     *const libc::c_char);
    con_fontnum =
        Cvar_Get(b"con_fontnum\x00" as *const u8 as *const libc::c_char,
                 b"-1\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int,
                 b"console font number (0, 1 or 2), -1 for autoselect\x00" as
                     *const u8 as *const libc::c_char);
    con_color =
        Cvar_Get(b"con_color\x00" as *const u8 as *const libc::c_char,
                 b"240 180 24\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int,
                 b"set a custom console color\x00" as *const u8 as
                     *const libc::c_char);
    // init the console buffer
    con.bufsize = 1048576 as libc::c_int; // default as 4
    con.buffer =
        _Mem_Alloc(host.mempool, con.bufsize as size_t, true_0,
                   b"../engine/client/console.c\x00" as *const u8 as
                       *const libc::c_char, 1195 as libc::c_int) as
            *mut libc::c_char;
    con.maxlines = 16384 as libc::c_int;
    con.lines =
        _Mem_Alloc(host.mempool,
                   (con.maxlines as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<con_lineinfo_t>()
                                                        as libc::c_ulong),
                   true_0,
                   b"../engine/client/console.c\x00" as *const u8 as
                       *const libc::c_char, 1197 as libc::c_int) as
            *mut con_lineinfo_t;
    con.lines_count = 0 as libc::c_int;
    con.lines_first = con.lines_count;
    con.num_times = 4 as libc::c_int;
    Con_CheckResize();
    Con_ClearField(&mut con.input);
    con.input.widthInChars = con.linewidth;
    Con_ClearField(&mut con.chat);
    con.chat.widthInChars = con.linewidth;
    Cmd_AddCommand(b"toggleconsole\x00" as *const u8 as *const libc::c_char,
                   Some(Con_ToggleConsole_f as unsafe extern "C" fn() -> ()),
                   b"opens or closes the console\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"clear\x00" as *const u8 as *const libc::c_char,
                   Some(Con_Clear_f as unsafe extern "C" fn() -> ()),
                   b"clear console history\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"messagemode\x00" as *const u8 as *const libc::c_char,
                   Some(Con_MessageMode_f as unsafe extern "C" fn() -> ()),
                   b"enable message mode \"say\"\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"messagemode2\x00" as *const u8 as *const libc::c_char,
                   Some(Con_MessageMode2_f as unsafe extern "C" fn() -> ()),
                   b"enable message mode \"say_team\"\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"contimes\x00" as *const u8 as *const libc::c_char,
                   Some(Con_SetTimes_f as unsafe extern "C" fn() -> ()),
                   b"change number of console overlay lines (4-64)\x00" as
                       *const u8 as *const libc::c_char);
    con.initialized = true_0;
    Con_Printf(b"Console initialized.\n\x00" as *const u8 as
                   *const libc::c_char);
}
/*
================
Con_Shutdown
================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_Shutdown() {
    con.initialized = false_0;
    if !con.buffer.is_null() {
        _Mem_Free(con.buffer as *mut libc::c_void,
                  b"../engine/client/console.c\x00" as *const u8 as
                      *const libc::c_char, 1229 as libc::c_int);
    }
    if !con.lines.is_null() {
        _Mem_Free(con.lines as *mut libc::c_void,
                  b"../engine/client/console.c\x00" as *const u8 as
                      *const libc::c_char, 1232 as libc::c_int);
    }
    con.buffer = 0 as *mut libc::c_char;
    con.lines = 0 as *mut con_lineinfo_t;
    Con_SaveHistory();
}
/*
================
Con_Print

Handles cursor positioning, line wrapping, etc
All console printing must go through this in order to be displayed
If no console is visible, the notify window will pop up.
================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_Print(mut txt: *const libc::c_char) {
    static mut cr_pending: libc::c_int = 0 as libc::c_int;
    static mut buf: [libc::c_char; 8192] = [0; 8192];
    let mut norefresh: qboolean = false_0;
    static mut lastlength: libc::c_int = 0 as libc::c_int;
    static mut inupdate: qboolean = false_0;
    static mut bufpos: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_int = 0;
    let mut mask: libc::c_int = 0 as libc::c_int;
    // client not running
    if con.initialized as u64 == 0 || con.buffer.is_null() { return }
    if *txt.offset(0 as libc::c_int as isize) as libc::c_int ==
           2 as libc::c_int {
        // go to colored text
        if Con_FixedFont() as u64 != 0 { mask = 128 as libc::c_int }
        txt = txt.offset(1)
    }
    if *txt.offset(0 as libc::c_int as isize) as libc::c_int ==
           3 as libc::c_int {
        norefresh = true_0;
        txt = txt.offset(1)
    }
    while *txt != 0 {
        if cr_pending != 0 {
            Con_DeleteLastLine();
            cr_pending = 0 as libc::c_int
        }
        c = *txt as libc::c_int;
        match c {
            0 => { }
            13 => {
                Con_AddLine(buf.as_mut_ptr(), bufpos, true_0);
                lastlength =
                    (*con.lines.offset(((con.lines_first +
                                             (con.lines_count -
                                                  1 as libc::c_int)) %
                                            con.maxlines) as isize)).length as
                        libc::c_int;
                cr_pending = 1 as libc::c_int;
                bufpos = 0 as libc::c_int
            }
            10 => {
                Con_AddLine(buf.as_mut_ptr(), bufpos, true_0);
                lastlength =
                    (*con.lines.offset(((con.lines_first +
                                             (con.lines_count -
                                                  1 as libc::c_int)) %
                                            con.maxlines) as isize)).length as
                        libc::c_int;
                bufpos = 0 as libc::c_int
            }
            _ => {
                let fresh1 = bufpos;
                bufpos = bufpos + 1;
                buf[fresh1 as usize] = (c | mask) as libc::c_char;
                if bufpos as libc::c_ulong >=
                       (::std::mem::size_of::<[libc::c_char; 8192]>() as
                            libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                            libc::c_ulong) ||
                       bufpos >= con.linewidth - 1 as libc::c_int {
                    Con_AddLine(buf.as_mut_ptr(), bufpos, true_0);
                    lastlength =
                        (*con.lines.offset(((con.lines_first +
                                                 (con.lines_count -
                                                      1 as libc::c_int)) %
                                                con.maxlines) as
                                               isize)).length as libc::c_int;
                    bufpos = 0 as libc::c_int
                }
            }
        }
        txt = txt.offset(1)
    }
    if norefresh as u64 != 0 { return }
    // custom renderer cause problems while updates screen on-loading
    if SV_Active() as libc::c_uint != 0 &&
           (cls.state as libc::c_uint) <
               ca_active as libc::c_int as libc::c_uint &&
           cl.video_prepped as u64 == 0 && cls.disable_screen == 0. {
        if bufpos != 0 as libc::c_int {
            Con_AddLine(buf.as_mut_ptr(), bufpos,
                        (lastlength != 0 as libc::c_int) as libc::c_int as
                            qboolean);
            lastlength = 0 as libc::c_int;
            bufpos = 0 as libc::c_int
        }
        // FIXME: disable updating screen, because when texture is bound any console print
		// can re-bound it to console font texture
        if con.lastupdate < Sys_DoubleTime() {
            con.lastupdate = Sys_DoubleTime() + 1.0f64;
            Host_InputFrame();
        }
    };
}
// pump messages to avoid window hanging
/*
================
Con_NPrint

Draw a single debug line with specified height
================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_NPrintf(mut idx: libc::c_int,
                                     mut fmt: *const libc::c_char,
                                     mut args: ...) {
    let mut args_0: ::std::ffi::VaListImpl;
    if idx < 0 as libc::c_int || idx >= 128 as libc::c_int { return }
    memset(con.notify[idx as usize].szNotify.as_mut_ptr() as
               *mut libc::c_void, 0 as libc::c_int,
           256 as libc::c_int as libc::c_ulong);
    args_0 = args.clone();
    Q_vsnprintf(con.notify[idx as usize].szNotify.as_mut_ptr(),
                256 as libc::c_int as size_t, fmt, args_0.as_va_list());
    // reset values
    con.notify[idx as usize].key_dest = key_game as libc::c_int;
    con.notify[idx as usize].expire =
        (host.realtime + 4.0f32 as libc::c_double) as libc::c_float;
    con.notify[idx as usize].color[0 as libc::c_int as usize] =
        255 as libc::c_int as byte;
    con.notify[idx as usize].color[1 as libc::c_int as usize] =
        255 as libc::c_int as byte;
    con.notify[idx as usize].color[2 as libc::c_int as usize] =
        255 as libc::c_int as byte;
    con.notify[idx as usize].color[3 as libc::c_int as usize] =
        255 as libc::c_int as byte;
    con.draw_notify = true_0;
}
/*
================
Con_NXPrint

Draw a single debug line with specified height, color and time to live
================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_NXPrintf(mut info: *mut con_nprint_t,
                                      mut fmt: *const libc::c_char,
                                      mut args: ...) {
    let mut args_0: ::std::ffi::VaListImpl;
    if info.is_null() { return }
    if (*info).index < 0 as libc::c_int || (*info).index >= 128 as libc::c_int
       {
        return
    }
    memset(con.notify[(*info).index as usize].szNotify.as_mut_ptr() as
               *mut libc::c_void, 0 as libc::c_int,
           256 as libc::c_int as libc::c_ulong);
    args_0 = args.clone();
    Q_vsnprintf(con.notify[(*info).index as usize].szNotify.as_mut_ptr(),
                256 as libc::c_int as size_t, fmt, args_0.as_va_list());
    // setup values
    con.notify[(*info).index as usize].key_dest = key_game as libc::c_int;
    con.notify[(*info).index as usize].expire =
        (host.realtime + (*info).time_to_live as libc::c_double) as
            libc::c_float;
    con.notify[(*info).index as usize].color[0 as libc::c_int as usize] =
        ((*info).color[0 as libc::c_int as usize] *
             255 as libc::c_int as libc::c_float) as byte;
    con.notify[(*info).index as usize].color[1 as libc::c_int as usize] =
        ((*info).color[1 as libc::c_int as usize] *
             255 as libc::c_int as libc::c_float) as byte;
    con.notify[(*info).index as usize].color[2 as libc::c_int as usize] =
        ((*info).color[2 as libc::c_int as usize] *
             255 as libc::c_int as libc::c_float) as byte;
    con.notify[(*info).index as usize].color[3 as libc::c_int as usize] =
        255 as libc::c_int as byte;
    con.draw_notify = true_0;
}
/*
================
UI_NPrint

Draw a single debug line with specified height (menu version)
================
*/
#[no_mangle]
pub unsafe extern "C" fn UI_NPrintf(mut idx: libc::c_int,
                                    mut fmt: *const libc::c_char,
                                    mut args: ...) {
    let mut args_0: ::std::ffi::VaListImpl;
    if idx < 0 as libc::c_int || idx >= 128 as libc::c_int { return }
    memset(con.notify[idx as usize].szNotify.as_mut_ptr() as
               *mut libc::c_void, 0 as libc::c_int,
           256 as libc::c_int as libc::c_ulong);
    args_0 = args.clone();
    Q_vsnprintf(con.notify[idx as usize].szNotify.as_mut_ptr(),
                256 as libc::c_int as size_t, fmt, args_0.as_va_list());
    // reset values
    con.notify[idx as usize].key_dest = key_menu as libc::c_int;
    con.notify[idx as usize].expire =
        (host.realtime + 4.0f32 as libc::c_double) as libc::c_float;
    con.notify[idx as usize].color[0 as libc::c_int as usize] =
        255 as libc::c_int as byte;
    con.notify[idx as usize].color[1 as libc::c_int as usize] =
        255 as libc::c_int as byte;
    con.notify[idx as usize].color[2 as libc::c_int as usize] =
        255 as libc::c_int as byte;
    con.notify[idx as usize].color[3 as libc::c_int as usize] =
        255 as libc::c_int as byte;
    con.draw_notify = true_0;
}
/*
================
UI_NXPrint

Draw a single debug line with specified height, color and time to live (menu version)
================
*/
#[no_mangle]
pub unsafe extern "C" fn UI_NXPrintf(mut info: *mut con_nprint_t,
                                     mut fmt: *const libc::c_char,
                                     mut args: ...) {
    let mut args_0: ::std::ffi::VaListImpl;
    if info.is_null() { return }
    if (*info).index < 0 as libc::c_int || (*info).index >= 128 as libc::c_int
       {
        return
    }
    memset(con.notify[(*info).index as usize].szNotify.as_mut_ptr() as
               *mut libc::c_void, 0 as libc::c_int,
           256 as libc::c_int as libc::c_ulong);
    args_0 = args.clone();
    Q_vsnprintf(con.notify[(*info).index as usize].szNotify.as_mut_ptr(),
                256 as libc::c_int as size_t, fmt, args_0.as_va_list());
    // setup values
    con.notify[(*info).index as usize].key_dest = key_menu as libc::c_int;
    con.notify[(*info).index as usize].expire =
        (host.realtime + (*info).time_to_live as libc::c_double) as
            libc::c_float;
    con.notify[(*info).index as usize].color[0 as libc::c_int as usize] =
        ((*info).color[0 as libc::c_int as usize] *
             255 as libc::c_int as libc::c_float) as byte;
    con.notify[(*info).index as usize].color[1 as libc::c_int as usize] =
        ((*info).color[1 as libc::c_int as usize] *
             255 as libc::c_int as libc::c_float) as byte;
    con.notify[(*info).index as usize].color[2 as libc::c_int as usize] =
        ((*info).color[2 as libc::c_int as usize] *
             255 as libc::c_int as libc::c_float) as byte;
    con.notify[(*info).index as usize].color[3 as libc::c_int as usize] =
        255 as libc::c_int as byte;
    con.draw_notify = true_0;
}
/*
=============================================================================

EDIT FIELDS

=============================================================================
*/
/*
================
Con_ClearField
================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_ClearField(mut edit: *mut field_t) {
    memset((*edit).buffer.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           256 as libc::c_int as libc::c_ulong);
    (*edit).cursor = 0 as libc::c_int;
    (*edit).scroll = 0 as libc::c_int;
}
/*
================
Field_Paste
================
*/
#[no_mangle]
pub unsafe extern "C" fn Field_Paste(mut edit: *mut field_t) {
    let mut cbd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut pasteLen: libc::c_int = 0;
    cbd = Sys_GetClipboardData();
    if cbd.is_null() { return }
    // send as if typed, so insert / overstrike works properly
    pasteLen = Q_strlen(cbd) as libc::c_int;
    i = 0 as libc::c_int;
    while i < pasteLen {
        Field_CharEvent(edit, *cbd.offset(i as isize) as libc::c_int);
        i += 1
    };
}
/*
=================
Field_KeyDownEvent

Performs the basic line editing functions for the console,
in-game talk, and menu fields

Key events are used for non-printable characters, others are gotten from char events.
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Field_KeyDownEvent(mut edit: *mut field_t,
                                            mut key: libc::c_int) {
    let mut len: libc::c_int = 0;
    // shift-insert is paste
    if (key == 147 as libc::c_int || key == 170 as libc::c_int) &&
           Key_IsDown(134 as libc::c_int) != 0 {
        Field_Paste(edit);
        return
    }
    len = Q_strlen((*edit).buffer.as_mut_ptr()) as libc::c_int;
    if key == 148 as libc::c_int {
        if (*edit).cursor < len {
            memmove((*edit).buffer.as_mut_ptr().offset((*edit).cursor as
                                                           isize) as
                        *mut libc::c_void,
                    (*edit).buffer.as_mut_ptr().offset((*edit).cursor as
                                                           isize).offset(1 as
                                                                             libc::c_int
                                                                             as
                                                                             isize)
                        as *const libc::c_void,
                    (len - (*edit).cursor) as libc::c_ulong);
        }
        return
    }
    if key == 127 as libc::c_int {
        if (*edit).cursor > 0 as libc::c_int {
            let mut newcursor: libc::c_int =
                Con_UtfMoveLeft((*edit).buffer.as_mut_ptr(), (*edit).cursor);
            memmove((*edit).buffer.as_mut_ptr().offset(newcursor as isize) as
                        *mut libc::c_void,
                    (*edit).buffer.as_mut_ptr().offset((*edit).cursor as
                                                           isize) as
                        *const libc::c_void,
                    (len - (*edit).cursor + 1 as libc::c_int) as
                        libc::c_ulong);
            (*edit).cursor = newcursor;
            if (*edit).scroll != 0 { (*edit).scroll -= 1 }
        }
        return
    }
    if key == 131 as libc::c_int {
        if (*edit).cursor < len {
            (*edit).cursor =
                Con_UtfMoveRight((*edit).buffer.as_mut_ptr(), (*edit).cursor,
                                 (*edit).widthInChars)
        }
        if (*edit).cursor >= (*edit).scroll + (*edit).widthInChars &&
               (*edit).cursor <= len {
            (*edit).scroll += 1
        }
        return
    }
    if key == 130 as libc::c_int {
        if (*edit).cursor > 0 as libc::c_int {
            (*edit).cursor =
                Con_UtfMoveLeft((*edit).buffer.as_mut_ptr(), (*edit).cursor)
        }
        if (*edit).cursor < (*edit).scroll { (*edit).scroll -= 1 }
        return
    }
    if key == 151 as libc::c_int ||
           Q_tolower(key as libc::c_char) as libc::c_int == 'a' as i32 &&
               Key_IsDown(133 as libc::c_int) != 0 {
        (*edit).cursor = 0 as libc::c_int;
        return
    }
    if key == 152 as libc::c_int ||
           Q_tolower(key as libc::c_char) as libc::c_int == 'e' as i32 &&
               Key_IsDown(133 as libc::c_int) != 0 {
        (*edit).cursor = len;
        return
    }
    if key == 147 as libc::c_int {
        host.key_overstrike =
            (host.key_overstrike as u64 == 0) as libc::c_int as qboolean;
        return
    };
}
/*
==================
Field_CharEvent
==================
*/
#[no_mangle]
pub unsafe extern "C" fn Field_CharEvent(mut edit: *mut field_t,
                                         mut ch: libc::c_int) {
    let mut len: libc::c_int = 0;
    if ch == 'v' as i32 - 'a' as i32 + 1 as libc::c_int {
        // ctrl-v is paste
        Field_Paste(edit);
        return
    }
    if ch == 'c' as i32 - 'a' as i32 + 1 as libc::c_int {
        // ctrl-c clears the field
        Con_ClearField(edit);
        return
    }
    len = Q_strlen((*edit).buffer.as_mut_ptr()) as libc::c_int;
    if ch == 'a' as i32 - 'a' as i32 + 1 as libc::c_int {
        // ctrl-a is home
        (*edit).cursor = 0 as libc::c_int;
        (*edit).scroll = 0 as libc::c_int;
        return
    }
    if ch == 'e' as i32 - 'a' as i32 + 1 as libc::c_int {
        // ctrl-e is end
        (*edit).cursor = len;
        (*edit).scroll = (*edit).cursor - (*edit).widthInChars;
        return
    }
    // ignore any other non printable chars
    if ch < 32 as libc::c_int { return }
    if host.key_overstrike as u64 != 0 {
        if (*edit).cursor == 256 as libc::c_int - 1 as libc::c_int { return }
        (*edit).buffer[(*edit).cursor as usize] = ch as libc::c_char;
        (*edit).cursor += 1
    } else {
        // insert mode
        if len == 256 as libc::c_int - 1 as libc::c_int { return } // all full
        memmove((*edit).buffer.as_mut_ptr().offset((*edit).cursor as
                                                       isize).offset(1 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)
                    as *mut libc::c_void,
                (*edit).buffer.as_mut_ptr().offset((*edit).cursor as isize) as
                    *const libc::c_void,
                (len + 1 as libc::c_int - (*edit).cursor) as libc::c_ulong);
        (*edit).buffer[(*edit).cursor as usize] = ch as libc::c_char;
        (*edit).cursor += 1
    }
    if (*edit).cursor >= (*edit).widthInChars { (*edit).scroll += 1 }
    if (*edit).cursor == len + 1 as libc::c_int {
        (*edit).buffer[(*edit).cursor as usize] =
            0 as libc::c_int as libc::c_char
    };
}
/*
==================
Field_DrawInputLine
==================
*/
#[no_mangle]
pub unsafe extern "C" fn Field_DrawInputLine(mut x: libc::c_int,
                                             mut y: libc::c_int,
                                             mut edit: *mut field_t) {
    let mut len: libc::c_int = 0;
    let mut cursorChar: libc::c_int = 0;
    let mut drawLen: libc::c_int = 0;
    let mut hideChar: libc::c_int = -(1 as libc::c_int);
    let mut prestep: libc::c_int = 0;
    let mut curPos: libc::c_int = 0;
    let mut str: [libc::c_char; 1024] = [0; 1024];
    let mut colorDefault: *mut byte = 0 as *mut byte;
    drawLen = (*edit).widthInChars;
    len =
        Q_strlen((*edit).buffer.as_mut_ptr()).wrapping_add(1 as libc::c_int as
                                                               libc::c_ulong)
            as libc::c_int;
    colorDefault =
        g_color_table[('7' as i32 - '0' as i32 & 7 as libc::c_int) as
                          usize].as_mut_ptr();
    // guarantee that cursor will be visible
    if len <= drawLen {
        prestep = 0 as libc::c_int
    } else {
        if (*edit).scroll + drawLen > len {
            (*edit).scroll = len - drawLen;
            if (*edit).scroll < 0 as libc::c_int {
                (*edit).scroll = 0 as libc::c_int
            }
        }
        prestep = (*edit).scroll
    }
    if prestep + drawLen > len { drawLen = len - prestep }
    // extract <drawLen> characters from the field at <prestep>
    drawLen =
        if drawLen < 1024 as libc::c_int - 1 as libc::c_int {
            drawLen
        } else { (1024 as libc::c_int) - 1 as libc::c_int };
    memcpy(str.as_mut_ptr() as *mut libc::c_void,
           (*edit).buffer.as_mut_ptr().offset(prestep as isize) as
               *const libc::c_void, drawLen as libc::c_ulong);
    str[drawLen as usize] = 0 as libc::c_int as libc::c_char;
    // save char for overstrike
    cursorChar =
        str[((*edit).cursor - prestep) as usize] as
            libc::c_int; // skip this char
    if host.key_overstrike as libc::c_uint != 0 && cursorChar != 0 &&
           (host.realtime * 4 as libc::c_int as libc::c_double) as libc::c_int
               & 1 as libc::c_int == 0 {
        hideChar = (*edit).cursor - prestep
    }
    // draw it
    Con_DrawGenericString(x, y, str.as_mut_ptr(), colorDefault, false_0,
                          hideChar);
    // draw the cursor
    if (host.realtime * 4 as libc::c_int as libc::c_double) as libc::c_int &
           1 as libc::c_int != 0 {
        return
    } // off blink
    // calc cursor position
    str[((*edit).cursor - prestep) as usize] =
        0 as libc::c_int as libc::c_char;
    Con_DrawStringLen(str.as_mut_ptr(), &mut curPos, 0 as *mut libc::c_int);
    Con_UtfProcessChar(0 as libc::c_int);
    if host.key_overstrike as libc::c_uint != 0 && cursorChar != 0 {
        // overstrike cursor
        Con_DrawGenericChar(x + curPos, y, cursorChar, colorDefault);
    } else {
        Con_UtfProcessChar(0 as libc::c_int);
        Con_DrawCharacter(x + curPos, y, '_' as i32, colorDefault);
    };
}
/*
=============================================================================

CONSOLE LINE EDITING

==============================================================================
*/
/*
====================
Key_Console

Handles history and console scrollback
====================
*/
#[no_mangle]
pub unsafe extern "C" fn Key_Console(mut key: libc::c_int) {
    // ctrl-L clears screen
    if key == 'l' as i32 && Key_IsDown(133 as libc::c_int) != 0 {
        Cbuf_AddText(b"clear\n\x00" as *const u8 as *const libc::c_char);
        return
    }
    // enter finishes the line
    if key == 13 as libc::c_int || key == 169 as libc::c_int {
        // if not in the game explicitly prepent a slash if needed
        if cls.state as libc::c_uint !=
               ca_active as libc::c_int as libc::c_uint &&
               con.input.buffer[0 as libc::c_int as usize] as libc::c_int !=
                   '\\' as i32 &&
               con.input.buffer[0 as libc::c_int as usize] as libc::c_int !=
                   '/' as i32 {
            let mut temp: [libc::c_char; 1024] = [0; 1024];
            Q_strncpy(temp.as_mut_ptr(), con.input.buffer.as_mut_ptr(),
                      ::std::mem::size_of::<[libc::c_char; 1024]>() as
                          libc::c_ulong);
            Q_sprintf(con.input.buffer.as_mut_ptr(),
                      b"\\%s\x00" as *const u8 as *const libc::c_char,
                      temp.as_mut_ptr());
            con.input.cursor += 1
        }
        // backslash text are commands, else chat
        if con.input.buffer[0 as libc::c_int as usize] as libc::c_int ==
               '\\' as i32 ||
               con.input.buffer[0 as libc::c_int as usize] as libc::c_int ==
                   '/' as i32 { // valid command
            Cbuf_AddText(con.input.buffer.as_mut_ptr().offset(1 as libc::c_int
                                                                  as
                                                                  isize)); // skip backslash
        } else { Cbuf_AddText(con.input.buffer.as_mut_ptr()); }
        Cbuf_AddText(b"\n\x00" as *const u8 as *const libc::c_char);
        // echo to console
        Con_Printf(b">%s\n\x00" as *const u8 as *const libc::c_char,
                   con.input.buffer.as_mut_ptr());
        // copy line to history buffer
        con.historyLines[(con.nextHistoryLine % 64 as libc::c_int) as usize] =
            con.input;
        con.nextHistoryLine += 1;
        con.historyLine = con.nextHistoryLine;
        Con_ClearField(&mut con.input);
        con.input.widthInChars = con.linewidth;
        Con_Bottom();
        if cls.state as libc::c_uint ==
               ca_disconnected as libc::c_int as libc::c_uint {
            // force an update, because the command may take some time
            SCR_UpdateScreen();
        }
        return
    }
    // command completion
    if key == 9 as libc::c_int {
        Con_CompleteCommand(&mut con.input);
        Con_Bottom();
        return
    }
    // command history (ctrl-p ctrl-n for unix style)
    if key == 240 as libc::c_int && Key_IsDown(134 as libc::c_int) != 0 ||
           key == 128 as libc::c_int ||
           Q_tolower(key as libc::c_char) as libc::c_int == 'p' as i32 &&
               Key_IsDown(133 as libc::c_int) != 0 {
        if con.historyLine == con.nextHistoryLine { con.backup = con.input }
        if con.nextHistoryLine - con.historyLine < 64 as libc::c_int &&
               con.historyLine > 0 as libc::c_int {
            con.historyLine -= 1
        }
        con.input =
            con.historyLines[(con.historyLine % 64 as libc::c_int) as usize];
        return
    }
    if key == 239 as libc::c_int && Key_IsDown(134 as libc::c_int) != 0 ||
           key == 129 as libc::c_int ||
           Q_tolower(key as libc::c_char) as libc::c_int == 'n' as i32 &&
               Key_IsDown(133 as libc::c_int) != 0 {
        if con.historyLine >= con.nextHistoryLine - 1 as libc::c_int {
            con.input = con.backup
        } else {
            con.historyLine += 1;
            con.input =
                con.historyLines[(con.historyLine % 64 as libc::c_int) as
                                     usize]
        }
        return
    }
    // console scrolling
    if key == 150 as libc::c_int { Con_PageUp(1 as libc::c_int); return }
    if key == 149 as libc::c_int { Con_PageDown(1 as libc::c_int); return }
    if key == 240 as libc::c_int {
        if Key_IsDown(133 as libc::c_int) != 0 {
            Con_PageUp(8 as libc::c_int);
        } else { Con_PageUp(2 as libc::c_int); }
        return
    }
    if key == 239 as libc::c_int {
        if Key_IsDown(133 as libc::c_int) != 0 {
            Con_PageDown(8 as libc::c_int);
        } else { Con_PageDown(2 as libc::c_int); }
        return
    }
    // ctrl-home = top of console
    if key == 151 as libc::c_int && Key_IsDown(133 as libc::c_int) != 0 {
        Con_Top();
        return
    }
    // ctrl-end = bottom of console
    if key == 152 as libc::c_int && Key_IsDown(133 as libc::c_int) != 0 {
        Con_Bottom();
        return
    }
    // pass to the normal editline routine
    Field_KeyDownEvent(&mut con.input, key);
}
/*
================
Key_Message

In game talk message
================
*/
#[no_mangle]
pub unsafe extern "C" fn Key_Message(mut key: libc::c_int) {
    let mut buffer: [libc::c_char; 1024] = [0; 1024];
    if key == 27 as libc::c_int {
        Key_SetKeyDest(key_game as libc::c_int);
        Con_ClearField(&mut con.chat);
        return
    }
    if key == 13 as libc::c_int || key == 169 as libc::c_int {
        if con.chat.buffer[0 as libc::c_int as usize] as libc::c_int != 0 &&
               cls.state as libc::c_uint ==
                   ca_active as libc::c_int as libc::c_uint {
            Q_snprintf(buffer.as_mut_ptr(),
                       ::std::mem::size_of::<[libc::c_char; 1024]>() as
                           libc::c_ulong,
                       b"%s \"%s\"\n\x00" as *const u8 as *const libc::c_char,
                       con.chat_cmd.as_mut_ptr(),
                       con.chat.buffer.as_mut_ptr());
            if g_messagemode_privileged as u64 != 0 {
                Cbuf_AddText(buffer.as_mut_ptr());
            } else { Cbuf_AddFilteredText(buffer.as_mut_ptr()); }
        }
        Key_SetKeyDest(key_game as libc::c_int);
        Con_ClearField(&mut con.chat);
        return
    }
    Field_KeyDownEvent(&mut con.chat, key);
}
/*
==============================================================================

DRAWING

==============================================================================
*/
/*
================
Con_DrawInput

The input line scrolls horizontally if typing goes beyond the right edge
================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_DrawInput(mut lines: libc::c_int) {
    let mut y: libc::c_int = 0;
    // don't draw anything (always draw if not active)
    if cls.key_dest as libc::c_uint !=
           key_console as libc::c_int as libc::c_uint || con.curFont.is_null()
       {
        return
    }
    y = lines - (*con.curFont).charHeight * 2 as libc::c_int;
    Con_DrawCharacter((*con.curFont).charWidths[' ' as i32 as usize] as
                          libc::c_int, y, ']' as i32,
                      g_color_table[7 as libc::c_int as usize].as_mut_ptr());
    Field_DrawInputLine((*con.curFont).charWidths[' ' as i32 as usize] as
                            libc::c_int * 2 as libc::c_int, y,
                        &mut con.input);
}
/*
================
Con_DrawDebugLines

Custom debug messages
================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_DrawDebugLines() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut defaultX: libc::c_int = 0;
    let mut y: libc::c_int = 20 as libc::c_int;
    defaultX = refState.width / 4 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 128 as libc::c_int {
        if host.realtime < con.notify[i as usize].expire as libc::c_double &&
               con.notify[i as usize].key_dest as libc::c_uint ==
                   cls.key_dest as libc::c_uint {
            let mut x: libc::c_int = 0;
            let mut len: libc::c_int = 0;
            let mut fontTall: libc::c_int = 0 as libc::c_int;
            Con_DrawStringLen(con.notify[i as usize].szNotify.as_mut_ptr(),
                              &mut len, &mut fontTall);
            x =
                refState.width - (if defaultX > len { defaultX } else { len })
                    - 10 as libc::c_int;
            fontTall += 1 as libc::c_int;
            if y + fontTall > refState.height - 20 as libc::c_int {
                return count
            }
            count += 1;
            y = 20 as libc::c_int + fontTall * i;
            Con_DrawString(x, y, con.notify[i as usize].szNotify.as_mut_ptr(),
                           con.notify[i as usize].color.as_mut_ptr());
        }
        i += 1
    }
    return count;
}
/*
================
Con_DrawDebug

Draws the debug messages (not passed to console history)
================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_DrawDebug() {
    static mut timeStart: libc::c_double = 0.;
    let mut dlstring: string = [0; 256];
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    if (*scr_download).value != -1.0f32 {
        Q_snprintf(dlstring.as_mut_ptr(),
                   ::std::mem::size_of::<string>() as libc::c_ulong,
                   b"Downloading [%d remaining]: ^2%s^7 %5.1f%% time %.f secs\x00"
                       as *const u8 as *const libc::c_char,
                   host.downloadcount, host.downloadfile.as_mut_ptr(),
                   (*scr_download).value as libc::c_double,
                   Sys_DoubleTime() - timeStart);
        x = refState.width - 500 as libc::c_int;
        y =
            ((*con.curFont).charHeight as libc::c_float * 1.05f32) as
                libc::c_int;
        Con_DrawString(x, y, dlstring.as_mut_ptr(),
                       g_color_table[7 as libc::c_int as usize].as_mut_ptr());
    } else { timeStart = Sys_DoubleTime() }
    if host_developer.value == 0. ||
           Cvar_VariableInteger(b"cl_background\x00" as *const u8 as
                                    *const libc::c_char) != 0 ||
           Cvar_VariableInteger(b"sv_background\x00" as *const u8 as
                                    *const libc::c_char) != 0 {
        return
    }
    if con.draw_notify as libc::c_uint != 0 && Con_Visible() == 0 {
        if Con_DrawDebugLines() == 0 as libc::c_int {
            con.draw_notify = false_0
        }
    };
}
/*
================
Con_DrawNotify

Draws the last few lines of output transparently over the game top
================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_DrawNotify() {
    let mut time: libc::c_double =
        cl.time; // offset one space at left screen side
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0 as libc::c_int;
    if con.curFont.is_null() { return }
    x = (*con.curFont).charWidths[' ' as i32 as usize] as libc::c_int;
    if host_developer.value != 0. &&
           (Cvar_VariableInteger(b"cl_background\x00" as *const u8 as
                                     *const libc::c_char) == 0 &&
                Cvar_VariableInteger(b"sv_background\x00" as *const u8 as
                                         *const libc::c_char) == 0) {
        i = con.lines_count - con.num_times;
        while i < con.lines_count {
            let mut l: *mut con_lineinfo_t =
                &mut *con.lines.offset(((con.lines_first + i) % con.maxlines)
                                           as isize) as *mut con_lineinfo_t;
            if !((*l).addtime <
                     time - (*con_notifytime).value as libc::c_double) {
                Con_DrawString(x, y, (*l).start,
                               g_color_table[7 as libc::c_int as
                                                 usize].as_mut_ptr());
                y += (*con.curFont).charHeight
            }
            i += 1
        }
    }
    if cls.key_dest as libc::c_uint ==
           key_message as libc::c_int as libc::c_uint {
        let mut buf: string = [0; 256];
        let mut len: libc::c_int = 0;
        // update chatline position from client.dll
        if clgame.dllFuncs.pfnChatInputPosition.is_some() {
            clgame.dllFuncs.pfnChatInputPosition.expect("non-null function pointer")(&mut x,
                                                                                     &mut y);
        }
        Q_snprintf(buf.as_mut_ptr(),
                   ::std::mem::size_of::<string>() as libc::c_ulong,
                   b"%s: \x00" as *const u8 as *const libc::c_char,
                   con.chat_cmd.as_mut_ptr());
        Con_DrawStringLen(buf.as_mut_ptr(), &mut len, 0 as *mut libc::c_int);
        Con_DrawString(x, y, buf.as_mut_ptr(),
                       g_color_table[7 as libc::c_int as usize].as_mut_ptr());
        Field_DrawInputLine(x + len, y, &mut con.chat);
    }
    ref_0.dllFuncs.Color4ub.expect("non-null function pointer")(255 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uchar,
                                                                255 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uchar,
                                                                255 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uchar,
                                                                255 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uchar);
}
/*
================
Con_DrawConsoleLine

Draws a line of the console; returns its height in lines.
If alpha is 0, the line is not drawn, but still wrapped and its height
returned.
================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_DrawConsoleLine(mut y: libc::c_int,
                                             mut lineno: libc::c_int)
 -> libc::c_int {
    let mut li: *mut con_lineinfo_t =
        &mut *con.lines.offset(((con.lines_first + lineno) % con.maxlines) as
                                   isize) as
            *mut con_lineinfo_t; // this string will be shown only at notify
    if li.is_null() || (*li).start.is_null() ||
           *(*li).start as libc::c_int == '\u{1}' as i32 {
        return 0 as libc::c_int
    }
    if y >= (*con.curFont).charHeight {
        Con_DrawGenericString((*con.curFont).charWidths[' ' as i32 as usize]
                                  as libc::c_int, y, (*li).start,
                              g_color_table[7 as libc::c_int as
                                                usize].as_mut_ptr(), false_0,
                              -(1 as libc::c_int));
    }
    return (*con.curFont).charHeight;
}
/*
================
Con_LastVisibleLine

Calculates the last visible line index and how much to show
of it based on con.backscroll.
================
*/
unsafe extern "C" fn Con_LastVisibleLine(mut lastline: *mut libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut lines_seen: libc::c_int = 0 as libc::c_int;
    con.backscroll =
        if 0 as libc::c_int > con.backscroll {
            0 as libc::c_int
        } else { con.backscroll };
    *lastline = 0 as libc::c_int;
    // now count until we saw con_backscroll actual lines
    i = con.lines_count - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        // line is the last visible line?
        *lastline = i;
        if lines_seen + 1 as libc::c_int > con.backscroll &&
               lines_seen <= con.backscroll {
            return
        }
        lines_seen += 1 as libc::c_int;
        i -= 1
    }
    // if we get here, no line was on screen - scroll so that one line is visible then.
    con.backscroll = lines_seen - 1 as libc::c_int;
}
/*
================
Con_DrawConsole

Draws the console with the solid background
================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_DrawSolidConsole(mut lines: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut fraction: libc::c_float = 0.;
    let mut start: libc::c_int = 0;
    if lines <= 0 as libc::c_int { return }
    // draw the background
    ref_0.dllFuncs.GL_SetRenderMode.expect("non-null function pointer")(kRenderNormal
                                                                            as
                                                                            libc::c_int); // to prevent grab color from screenfade
    ref_0.dllFuncs.Color4ub.expect("non-null function pointer")(255 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uchar,
                                                                255 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uchar,
                                                                255 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uchar,
                                                                255 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uchar); // nothing to draw
    if (refState.width * 3 as libc::c_int / 4 as libc::c_int) <
           refState.height && lines >= refState.height {
        ref_0.dllFuncs.R_DrawStretchPic.expect("non-null function pointer")(0
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_float,
                                                                            (lines
                                                                                 -
                                                                                 refState.height)
                                                                                as
                                                                                libc::c_float,
                                                                            refState.width
                                                                                as
                                                                                libc::c_float,
                                                                            (refState.height
                                                                                 -
                                                                                 refState.width
                                                                                     *
                                                                                     3
                                                                                         as
                                                                                         libc::c_int
                                                                                     /
                                                                                     4
                                                                                         as
                                                                                         libc::c_int)
                                                                                as
                                                                                libc::c_float,
                                                                            0
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_float,
                                                                            0
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_float,
                                                                            1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_float,
                                                                            1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_float,
                                                                            ref_0.dllFuncs.GL_LoadTexture.expect("non-null function pointer")(b"*black\x00"
                                                                                                                                                  as
                                                                                                                                                  *const u8
                                                                                                                                                  as
                                                                                                                                                  *const libc::c_char,
                                                                                                                                              0
                                                                                                                                                  as
                                                                                                                                                  *const byte,
                                                                                                                                              0
                                                                                                                                                  as
                                                                                                                                                  libc::c_int
                                                                                                                                                  as
                                                                                                                                                  size_t,
                                                                                                                                              0
                                                                                                                                                  as
                                                                                                                                                  libc::c_int));
    }
    ref_0.dllFuncs.R_DrawStretchPic.expect("non-null function pointer")(0 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_float,
                                                                        (lines
                                                                             -
                                                                             refState.width
                                                                                 *
                                                                                 3
                                                                                     as
                                                                                     libc::c_int
                                                                                 /
                                                                                 4
                                                                                     as
                                                                                     libc::c_int)
                                                                            as
                                                                            libc::c_float,
                                                                        refState.width
                                                                            as
                                                                            libc::c_float,
                                                                        (refState.width
                                                                             *
                                                                             3
                                                                                 as
                                                                                 libc::c_int
                                                                             /
                                                                             4
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            libc::c_float,
                                                                        0 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_float,
                                                                        0 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_float,
                                                                        1 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_float,
                                                                        1 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_float,
                                                                        con.background);
    if con.curFont.is_null() || host.allow_console as u64 == 0 { return }
    if host.allow_console as u64 != 0 {
        // draw current version
        let mut stringLen: libc::c_int = 0; // fadeout version number
        let mut width: libc::c_int = 0 as libc::c_int;
        let mut charH: libc::c_int = 0;
        let mut curbuild: string = [0; 256];
        let mut color: [byte; 4] = [0; 4];
        memcpy(color.as_mut_ptr() as *mut libc::c_void,
               g_color_table[7 as libc::c_int as usize].as_mut_ptr() as
                   *const libc::c_void,
               ::std::mem::size_of::<[byte; 4]>() as libc::c_ulong);
        Q_snprintf(curbuild.as_mut_ptr(), 256 as libc::c_int as size_t,
                   b"%s %i/%s (%s-%s build %i)\x00" as *const u8 as
                       *const libc::c_char,
                   b"Xash3D FWGS\x00" as *const u8 as *const libc::c_char,
                   49 as libc::c_int,
                   b"0.20\x00" as *const u8 as *const libc::c_char,
                   Q_buildos(), Q_buildarch(), Q_buildnum());
        Con_DrawStringLen(curbuild.as_mut_ptr(), &mut stringLen, &mut charH);
        start = refState.width - stringLen;
        stringLen = Con_StringLength(curbuild.as_mut_ptr());
        fraction = lines as libc::c_float / refState.height as libc::c_float;
        color[3 as libc::c_int as usize] =
            ((if fraction * 2.0f32 < 1.0f32 {
                  (fraction) * 2.0f32
              } else { 1.0f32 }) * 255 as libc::c_int as libc::c_float) as
                byte;
        i = 0 as libc::c_int;
        while i < stringLen {
            width +=
                Con_DrawCharacter(start + width, 0 as libc::c_int,
                                  curbuild[i as usize] as libc::c_int,
                                  color.as_mut_ptr());
            i += 1
        }
    }
    // draw the text
    if con.lines_count > 0 as libc::c_int {
        let mut ymax: libc::c_int =
            (lines as libc::c_float -
                 (*con.curFont).charHeight as libc::c_float * 2.0f32) as
                libc::c_int; // offset one space at left screen side
        let mut lastline: libc::c_int = 0;
        Con_LastVisibleLine(&mut lastline);
        y = ymax - (*con.curFont).charHeight;
        if con.backscroll != 0 {
            start =
                (*con.curFont).charWidths[' ' as i32 as usize] as libc::c_int;
            // draw red arrows to show the buffer is backscrolled
            x = 0 as libc::c_int;
            while x < con.linewidth {
                Con_DrawCharacter((x + 1 as libc::c_int) * start, y,
                                  '^' as i32,
                                  g_color_table[1 as libc::c_int as
                                                    usize].as_mut_ptr());
                x += 4 as libc::c_int
            }
            y -= (*con.curFont).charHeight
        }
        x = lastline;
        loop  {
            y -= Con_DrawConsoleLine(y, x);
            // top of console buffer or console window
            if x == 0 as libc::c_int || y < (*con.curFont).charHeight {
                break ;
            }
            x -= 1
        }
    }
    // draw the input prompt, user text, and cursor if desired
    Con_DrawInput(lines); // to avoid to hide fps counter
    y =
        (lines as libc::c_float -
             (*con.curFont).charHeight as libc::c_float * 1.2f32) as
            libc::c_int;
    SCR_DrawFPS(if y > 4 as libc::c_int { y } else { 4 as libc::c_int });
    ref_0.dllFuncs.Color4ub.expect("non-null function pointer")(255 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uchar,
                                                                255 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uchar,
                                                                255 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uchar,
                                                                255 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uchar);
}
/*
==================
Con_DrawConsole
==================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_DrawConsole() {
    // never draw console when changelevel in-progress
    if cls.state as libc::c_uint !=
           ca_disconnected as libc::c_int as libc::c_uint &&
           (cls.changelevel as libc::c_uint != 0 ||
                cls.changedemo as libc::c_uint != 0) {
        return
    }
    // check for console width changes from a vid mode change
    Con_CheckResize();
    if cls.state as libc::c_uint ==
           ca_connecting as libc::c_int as libc::c_uint ||
           cls.state as libc::c_uint ==
               ca_connected as libc::c_int as libc::c_uint {
        if (*cl_allow_levelshots).value == 0. {
            if (Cvar_VariableInteger(b"cl_background\x00" as *const u8 as
                                         *const libc::c_char) != 0 ||
                    Cvar_VariableInteger(b"sv_background\x00" as *const u8 as
                                             *const libc::c_char) != 0) &&
                   cls.key_dest as libc::c_uint !=
                       key_console as libc::c_int as libc::c_uint {
                con.showlines = 0 as libc::c_int as libc::c_float;
                con.vislines = con.showlines
            } else {
                con.showlines = refState.height as libc::c_float;
                con.vislines = con.showlines
            }
        } else {
            con.showlines = 0 as libc::c_int as libc::c_float;
            if host_developer.value >=
                   DEV_EXTENDED as libc::c_int as libc::c_float {
                Con_DrawNotify();
            }
            // draw notify lines
        }
    }
    // if disconnected, render console full screen
    match cls.state as libc::c_uint {
        0 => {
            if cls.key_dest as libc::c_uint !=
                   key_menu as libc::c_int as libc::c_uint {
                Con_DrawSolidConsole(refState.height);
                Key_SetKeyDest(key_console as libc::c_int);
            }
        }
        1 | 2 | 3 => {
            // force to show console always for -dev 3 and higher
            Con_DrawSolidConsole(con.vislines as libc::c_int);
        }
        4 | 5 => {
            if Cvar_VariableInteger(b"cl_background\x00" as *const u8 as
                                        *const libc::c_char) != 0 ||
                   Cvar_VariableInteger(b"sv_background\x00" as *const u8 as
                                            *const libc::c_char) != 0 {
                if cls.key_dest as libc::c_uint ==
                       key_console as libc::c_int as libc::c_uint {
                    Con_DrawSolidConsole(refState.height);
                }
            } else if con.vislines != 0. {
                Con_DrawSolidConsole(con.vislines as libc::c_int);
            } else if cls.state as libc::c_uint ==
                          ca_active as libc::c_int as libc::c_uint &&
                          (cls.key_dest as libc::c_uint ==
                               key_game as libc::c_int as libc::c_uint ||
                               cls.key_dest as libc::c_uint ==
                                   key_message as libc::c_int as libc::c_uint)
             {
                Con_DrawNotify();
            }
        }
        _ => { }
    }
    if Con_Visible() == 0 { SCR_DrawFPS(4 as libc::c_int); };
}
// draw notify lines
/*
==================
Con_DrawVersion

Used by menu
==================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_DrawVersion() {
    // draws the current build
    let mut color: *mut byte =
        g_color_table[7 as libc::c_int as usize].as_mut_ptr();
    let mut i: libc::c_int = 0;
    let mut stringLen: libc::c_int = 0;
    let mut width: libc::c_int = 0 as libc::c_int;
    let mut charH: libc::c_int = 0 as libc::c_int;
    let mut start: libc::c_int = 0;
    let mut height: libc::c_int = refState.height;
    let mut draw_version: qboolean = false_0;
    let mut curbuild: string = [0; 256];
    match cls.scrshot_action as libc::c_uint {
        1 | 2 => { draw_version = true_0 }
        _ => { }
    }
    if host.force_draw_version as u64 == 0 {
        if cls.key_dest as libc::c_uint !=
               key_menu as libc::c_int as libc::c_uint &&
               draw_version as u64 == 0 ||
               CL_IsDevOverviewMode() == 2 as libc::c_int ||
               (*net_graph).value != 0. {
            return
        }
    }
    if host.force_draw_version_time as libc::c_double > host.realtime {
        host.force_draw_version = false_0
    }
    if host.force_draw_version as libc::c_uint != 0 ||
           draw_version as libc::c_uint != 0 {
        Q_snprintf(curbuild.as_mut_ptr(), 256 as libc::c_int as size_t,
                   b"%s v%i/%s (%s-%s build %i)\x00" as *const u8 as
                       *const libc::c_char,
                   b"Xash3D FWGS\x00" as *const u8 as *const libc::c_char,
                   49 as libc::c_int,
                   b"0.20\x00" as *const u8 as *const libc::c_char,
                   Q_buildos(), Q_buildarch(), Q_buildnum());
    } else {
        Q_snprintf(curbuild.as_mut_ptr(), 256 as libc::c_int as size_t,
                   b"v%i/%s (%s-%s build %i)\x00" as *const u8 as
                       *const libc::c_char, 49 as libc::c_int,
                   b"0.20\x00" as *const u8 as *const libc::c_char,
                   Q_buildos(), Q_buildarch(), Q_buildnum());
    }
    Con_DrawStringLen(curbuild.as_mut_ptr(), &mut stringLen, &mut charH);
    start =
        (refState.width as libc::c_float -
             stringLen as libc::c_float * 1.05f32) as libc::c_int;
    stringLen = Con_StringLength(curbuild.as_mut_ptr());
    height =
        (height as libc::c_float - charH as libc::c_float * 1.05f32) as
            libc::c_int;
    i = 0 as libc::c_int;
    while i < stringLen {
        width +=
            Con_DrawCharacter(start + width, height,
                              curbuild[i as usize] as libc::c_int, color);
        i += 1
    };
}
/*
==================
Con_RunConsole

Scroll it up or down
==================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_RunConsole() {
    let mut lines_per_frame: libc::c_float = 0.;
    Con_SetColor();
    // decide on the destination height of the console
    if host.allow_console as libc::c_uint != 0 &&
           cls.key_dest as libc::c_uint ==
               key_console as libc::c_int as libc::c_uint {
        if (cls.state as libc::c_uint) <
               ca_active as libc::c_int as libc::c_uint ||
               cl.first_frame as libc::c_uint != 0 { // none visible
            con.showlines = refState.height as libc::c_float
        } else {
            con.showlines =
                (refState.height >> 1 as libc::c_int) as libc::c_float
        } // full screen
        // half screen
    } else { con.showlines = 0 as libc::c_int as libc::c_float }
    lines_per_frame =
        (__tg_fabs((*scr_conspeed).value) as libc::c_double *
             host.realframetime) as libc::c_float;
    if con.showlines < con.vislines {
        con.vislines -= lines_per_frame;
        if con.showlines > con.vislines { con.vislines = con.showlines }
    } else if con.showlines > con.vislines {
        con.vislines += lines_per_frame;
        if con.showlines < con.vislines { con.vislines = con.showlines }
    }
    if (*con_charset).flags & (1 as libc::c_int) << 13 as libc::c_int != 0 ||
           (*con_fontscale).flags & (1 as libc::c_int) << 13 as libc::c_int !=
               0 ||
           (*con_fontnum).flags & (1 as libc::c_int) << 13 as libc::c_int != 0
           ||
           (*cl_charset).flags & (1 as libc::c_int) << 13 as libc::c_int != 0
       {
        // update codepage parameters
        g_codepage = 0 as libc::c_int;
        if Q_strnicmp((*con_charset).string,
                      b"cp1251\x00" as *const u8 as *const libc::c_char,
                      99999 as libc::c_int) == 0 {
            g_codepage = 1251 as libc::c_int
        } else if Q_strnicmp((*con_charset).string,
                             b"cp1252\x00" as *const u8 as
                                 *const libc::c_char, 99999 as libc::c_int) ==
                      0 {
            g_codepage = 1252 as libc::c_int
        }
        g_utf8 =
            (Q_strnicmp((*cl_charset).string,
                        b"utf-8\x00" as *const u8 as *const libc::c_char,
                        99999 as libc::c_int) == 0) as libc::c_int as
                qboolean;
        Con_InvalidateFonts();
        Con_LoadConchars();
        cls.creditsFont.valid = false_0;
        SCR_LoadCreditsFont();
        (*con_charset).flags =
            (*con_charset).flags & !((1 as libc::c_int) << 13 as libc::c_int);
        (*con_fontnum).flags =
            (*con_fontnum).flags & !((1 as libc::c_int) << 13 as libc::c_int);
        (*con_fontscale).flags =
            (*con_fontscale).flags &
                !((1 as libc::c_int) << 13 as libc::c_int);
        (*cl_charset).flags =
            (*cl_charset).flags & !((1 as libc::c_int) << 13 as libc::c_int)
    };
}
/*
==============================================================================

CONSOLE INTERFACE

==============================================================================
*/
/*
================
Con_CharEvent

Console input
================
*/
#[no_mangle]
pub unsafe extern "C" fn Con_CharEvent(mut key: libc::c_int) {
    // distribute the key down event to the apropriate handler
    if cls.key_dest as libc::c_uint ==
           key_console as libc::c_int as libc::c_uint {
        Field_CharEvent(&mut con.input, key);
    } else if cls.key_dest as libc::c_uint ==
                  key_message as libc::c_int as libc::c_uint {
        Field_CharEvent(&mut con.chat, key);
    };
}
/*
=========
Con_VidInit

INTERNAL RESOURCE
=========
*/
#[no_mangle]
pub unsafe extern "C" fn Con_VidInit() {
    Con_LoadConchars();
    Con_CheckResize();
    // loading console image
    if host.allow_console as u64 != 0 {
        // trying to load truecolor image first
        if FS_FileExists(b"gfx/shell/conback.bmp\x00" as *const u8 as
                             *const libc::c_char, false_0 as libc::c_int) != 0
               ||
               FS_FileExists(b"gfx/shell/conback.tga\x00" as *const u8 as
                                 *const libc::c_char, false_0 as libc::c_int)
                   != 0 {
            con.background =
                ref_0.dllFuncs.GL_LoadTexture.expect("non-null function pointer")(b"gfx/shell/conback\x00"
                                                                                      as
                                                                                      *const u8
                                                                                      as
                                                                                      *const libc::c_char,
                                                                                  0
                                                                                      as
                                                                                      *const byte,
                                                                                  0
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      size_t,
                                                                                  TF_NOMIPMAP
                                                                                      as
                                                                                      libc::c_int
                                                                                      |
                                                                                      TF_CLAMP
                                                                                          as
                                                                                          libc::c_int)
        }
        if con.background == 0 {
            if FS_FileExists(b"cached/conback640\x00" as *const u8 as
                                 *const libc::c_char, false_0 as libc::c_int)
                   != 0 {
                con.background =
                    ref_0.dllFuncs.GL_LoadTexture.expect("non-null function pointer")(b"cached/conback640\x00"
                                                                                          as
                                                                                          *const u8
                                                                                          as
                                                                                          *const libc::c_char,
                                                                                      0
                                                                                          as
                                                                                          *const byte,
                                                                                      0
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          size_t,
                                                                                      TF_NOMIPMAP
                                                                                          as
                                                                                          libc::c_int
                                                                                          |
                                                                                          TF_CLAMP
                                                                                              as
                                                                                              libc::c_int)
            } else if FS_FileExists(b"cached/conback\x00" as *const u8 as
                                        *const libc::c_char,
                                    false_0 as libc::c_int) != 0 {
                con.background =
                    ref_0.dllFuncs.GL_LoadTexture.expect("non-null function pointer")(b"cached/conback\x00"
                                                                                          as
                                                                                          *const u8
                                                                                          as
                                                                                          *const libc::c_char,
                                                                                      0
                                                                                          as
                                                                                          *const byte,
                                                                                      0
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          size_t,
                                                                                      TF_NOMIPMAP
                                                                                          as
                                                                                          libc::c_int
                                                                                          |
                                                                                          TF_CLAMP
                                                                                              as
                                                                                              libc::c_int)
            }
        }
    } else {
        // trying to load truecolor image first
        if FS_FileExists(b"gfx/shell/loading.bmp\x00" as *const u8 as
                             *const libc::c_char, false_0 as libc::c_int) != 0
               ||
               FS_FileExists(b"gfx/shell/loading.tga\x00" as *const u8 as
                                 *const libc::c_char, false_0 as libc::c_int)
                   != 0 {
            con.background =
                ref_0.dllFuncs.GL_LoadTexture.expect("non-null function pointer")(b"gfx/shell/loading\x00"
                                                                                      as
                                                                                      *const u8
                                                                                      as
                                                                                      *const libc::c_char,
                                                                                  0
                                                                                      as
                                                                                      *const byte,
                                                                                  0
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      size_t,
                                                                                  TF_NOMIPMAP
                                                                                      as
                                                                                      libc::c_int
                                                                                      |
                                                                                      TF_CLAMP
                                                                                          as
                                                                                          libc::c_int)
        }
        if con.background == 0 {
            if FS_FileExists(b"cached/loading640\x00" as *const u8 as
                                 *const libc::c_char, false_0 as libc::c_int)
                   != 0 {
                con.background =
                    ref_0.dllFuncs.GL_LoadTexture.expect("non-null function pointer")(b"cached/loading640\x00"
                                                                                          as
                                                                                          *const u8
                                                                                          as
                                                                                          *const libc::c_char,
                                                                                      0
                                                                                          as
                                                                                          *const byte,
                                                                                      0
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          size_t,
                                                                                      TF_NOMIPMAP
                                                                                          as
                                                                                          libc::c_int
                                                                                          |
                                                                                          TF_CLAMP
                                                                                              as
                                                                                              libc::c_int)
            } else if FS_FileExists(b"cached/loading\x00" as *const u8 as
                                        *const libc::c_char,
                                    false_0 as libc::c_int) != 0 {
                con.background =
                    ref_0.dllFuncs.GL_LoadTexture.expect("non-null function pointer")(b"cached/loading\x00"
                                                                                          as
                                                                                          *const u8
                                                                                          as
                                                                                          *const libc::c_char,
                                                                                      0
                                                                                          as
                                                                                          *const byte,
                                                                                      0
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          size_t,
                                                                                      TF_NOMIPMAP
                                                                                          as
                                                                                          libc::c_int
                                                                                          |
                                                                                          TF_CLAMP
                                                                                              as
                                                                                              libc::c_int)
            }
        }
    }
    if con.background == 0 {
        // last chance - quake conback image
        let mut draw_to_console: qboolean = false_0;
        let mut length: fs_offset_t = 0 as libc::c_int as fs_offset_t;
        let mut buf: *const byte = 0 as *const byte;
        // NOTE: only these games want to draw build number into console background
        if Q_strnicmp((*SI.GameInfo).gamefolder.as_mut_ptr(),
                      b"id1\x00" as *const u8 as *const libc::c_char,
                      99999 as libc::c_int) == 0 {
            draw_to_console = true_0
        } // can store only buildnum
        if Q_strnicmp((*SI.GameInfo).gamefolder.as_mut_ptr(),
                      b"hipnotic\x00" as *const u8 as *const libc::c_char,
                      99999 as libc::c_int) == 0 {
            draw_to_console = true_0
        }
        if Q_strnicmp((*SI.GameInfo).gamefolder.as_mut_ptr(),
                      b"rogue\x00" as *const u8 as *const libc::c_char,
                      99999 as libc::c_int) == 0 {
            draw_to_console = true_0
        }
        if draw_to_console as libc::c_uint != 0 && !con.curFont.is_null() &&
               {
                   buf =
                       ref_0.dllFuncs.R_GetTextureOriginalBuffer.expect("non-null function pointer")((*con.curFont).hFontTexture
                                                                                                         as
                                                                                                         libc::c_uint);
                   !buf.is_null()
               } {
            let mut cb: *mut lmp_t =
                FS_LoadFile(b"gfx/conback.lmp\x00" as *const u8 as
                                *const libc::c_char, &mut length, false_0) as
                    *mut lmp_t;
            let mut ver: [libc::c_char; 64] = [0; 64];
            let mut dest: *mut byte = 0 as *mut byte;
            let mut x: libc::c_int = 0;
            let mut y: libc::c_int = 0;
            let mut len: libc::c_int = 0;
            if !cb.is_null() &&
                   (*cb).width == 320 as libc::c_int as libc::c_uint &&
                   (*cb).height == 200 as libc::c_int as libc::c_uint {
                len =
                    Q_snprintf(ver.as_mut_ptr(), 64 as libc::c_int as size_t,
                               b"%i\x00" as *const u8 as *const libc::c_char,
                               Q_buildnum());
                dest =
                    (cb.offset(1 as libc::c_int as isize) as
                         *mut byte).offset((320 as libc::c_int *
                                                186 as libc::c_int) as
                                               isize).offset(320 as
                                                                 libc::c_int
                                                                 as
                                                                 isize).offset(-(11
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).offset(-((8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           *
                                                                                                           len)
                                                                                                          as
                                                                                                          isize));
                y = len;
                x = 0 as libc::c_int;
                while x < y {
                    Con_DrawCharToConback(ver[x as usize] as libc::c_int, buf,
                                          dest.offset((x << 3 as libc::c_int)
                                                          as isize));
                    x += 1
                }
                con.background =
                    ref_0.dllFuncs.GL_LoadTexture.expect("non-null function pointer")(b"#gfx/conback.lmp\x00"
                                                                                          as
                                                                                          *const u8
                                                                                          as
                                                                                          *const libc::c_char,
                                                                                      cb
                                                                                          as
                                                                                          *mut byte,
                                                                                      length
                                                                                          as
                                                                                          size_t,
                                                                                      TF_NOMIPMAP
                                                                                          as
                                                                                          libc::c_int
                                                                                          |
                                                                                          TF_CLAMP
                                                                                              as
                                                                                              libc::c_int)
            }
            if !cb.is_null() {
                _Mem_Free(cb as *mut libc::c_void,
                          b"../engine/client/console.c\x00" as *const u8 as
                              *const libc::c_char, 2460 as libc::c_int);
            }
        }
        if con.background == 0 {
            // trying the load unmodified conback
            con.background =
                ref_0.dllFuncs.GL_LoadTexture.expect("non-null function pointer")(b"gfx/conback.lmp\x00"
                                                                                      as
                                                                                      *const u8
                                                                                      as
                                                                                      *const libc::c_char,
                                                                                  0
                                                                                      as
                                                                                      *const byte,
                                                                                  0
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      size_t,
                                                                                  TF_NOMIPMAP
                                                                                      as
                                                                                      libc::c_int
                                                                                      |
                                                                                      TF_CLAMP
                                                                                          as
                                                                                          libc::c_int)
        }
    }
    // missed console image will be replaced as gray background like X-Ray or Crysis
    if con.background ==
           ref_0.dllFuncs.GL_LoadTexture.expect("non-null function pointer")(b"*default\x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char,
                                                                             0
                                                                                 as
                                                                                 *const byte,
                                                                             0
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 size_t,
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
           || con.background == 0 as libc::c_int {
        con.background =
            ref_0.dllFuncs.GL_LoadTexture.expect("non-null function pointer")(b"*gray\x00"
                                                                                  as
                                                                                  *const u8
                                                                                  as
                                                                                  *const libc::c_char,
                                                                              0
                                                                                  as
                                                                                  *const byte,
                                                                              0
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  size_t,
                                                                              0
                                                                                  as
                                                                                  libc::c_int)
    };
}
/*
=========
Con_InvalidateFonts

=========
*/
#[no_mangle]
pub unsafe extern "C" fn Con_InvalidateFonts() {
    memset(con.chars.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[cl_font_t; 3]>() as libc::c_ulong);
    con.lastUsedFont = 0 as *mut cl_font_t;
    con.curFont = con.lastUsedFont;
}
/*
=========
Con_FastClose

immediately close the console
=========
*/
#[no_mangle]
pub unsafe extern "C" fn Con_FastClose() {
    Con_ClearField(&mut con.input);
    Con_ClearNotify();
    con.showlines = 0 as libc::c_int as libc::c_float;
    con.vislines = 0 as libc::c_int as libc::c_float;
}
/*
=========
Con_DefaultColor

called from MainUI
=========
*/
#[no_mangle]
pub unsafe extern "C" fn Con_DefaultColor(mut r: libc::c_int,
                                          mut g: libc::c_int,
                                          mut b: libc::c_int) {
    r =
        if r >= 0 as libc::c_int {
            if r < 255 as libc::c_int { r } else { 255 as libc::c_int }
        } else { 0 as libc::c_int };
    g =
        if g >= 0 as libc::c_int {
            if g < 255 as libc::c_int { g } else { 255 as libc::c_int }
        } else { 0 as libc::c_int };
    b =
        if b >= 0 as libc::c_int {
            if b < 255 as libc::c_int { b } else { 255 as libc::c_int }
        } else { 0 as libc::c_int };
    g_color_table[7 as libc::c_int as usize][0 as libc::c_int as usize] =
        r as byte;
    g_color_table[7 as libc::c_int as usize][1 as libc::c_int as usize] =
        g as byte;
    g_color_table[7 as libc::c_int as usize][2 as libc::c_int as usize] =
        b as byte;
    g_color_table[7 as libc::c_int as usize][3 as libc::c_int as usize] =
        255 as libc::c_int as byte;
}
