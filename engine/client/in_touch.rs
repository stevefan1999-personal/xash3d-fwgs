#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type file_s;
    pub type grasshdr_s;
    pub type mip_s;
    pub type engine_studio_api_s;
    pub type r_studio_interface_s;
    pub type IVoiceTweak_s;
    pub type demo_api_s;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn expf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn powf(_: libc::c_float, _: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn sqrtf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn fabsf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn roundf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Q_atoi(str: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn Q_atof(str: *const libc::c_char) -> libc::c_float;
    #[no_mangle]
    fn Q_strchr(s: *const libc::c_char, c: libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn Q_strncmp(s1: *const libc::c_char, s2: *const libc::c_char,
                 n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Q_stricmpext(s1: *const libc::c_char, s2: *const libc::c_char)
     -> qboolean;
    #[no_mangle]
    fn Q_timestamp(format: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn Q_strstr(string: *const libc::c_char, string2: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn Q_snprintf(buffer: *mut libc::c_char, buffersize: size_t,
                  format: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn COM_FileBase(in_0: *const libc::c_char, out: *mut libc::c_char);
    #[no_mangle]
    fn Platform_GetMousePos(x: *mut libc::c_int, y: *mut libc::c_int);
    #[no_mangle]
    fn Sys_CheckParm(parm: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn Cvar_Get(var_name: *const libc::c_char, value: *const libc::c_char,
                flags: libc::c_int, description: *const libc::c_char)
     -> *mut convar_t;
    #[no_mangle]
    fn Cvar_Set(var_name: *const libc::c_char, value: *const libc::c_char);
    #[no_mangle]
    fn Cvar_SetValue(var_name: *const libc::c_char, value: libc::c_float);
    #[no_mangle]
    static mut host: host_parm_t;
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
    fn Cmd_AddRestrictedCommand(cmd_name: *const libc::c_char,
                                function: xcommand_t,
                                cmd_desc: *const libc::c_char);
    #[no_mangle]
    fn Cmd_RemoveCommand(cmd_name: *const libc::c_char);
    #[no_mangle]
    fn Cmd_TokenizeString(text: *const libc::c_char);
    #[no_mangle]
    fn Cmd_Escape(newCommand: *mut libc::c_char,
                  oldCommand: *const libc::c_char, len: libc::c_int);
    #[no_mangle]
    fn _Mem_Alloc(poolptr: poolhandle_t, size: size_t, clear: qboolean,
                  filename: *const libc::c_char, fileline: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    fn _Mem_AllocPool(name: *const libc::c_char,
                      filename: *const libc::c_char, fileline: libc::c_int)
     -> poolhandle_t;
    #[no_mangle]
    fn _Mem_FreePool(poolptr: *mut poolhandle_t,
                     filename: *const libc::c_char, fileline: libc::c_int);
    #[no_mangle]
    fn _Mem_Free(data: *mut libc::c_void, filename: *const libc::c_char,
                 fileline: libc::c_int);
    #[no_mangle]
    fn FS_Open(filepath: *const libc::c_char, mode: *const libc::c_char,
               gamedironly: qboolean) -> *mut file_t;
    #[no_mangle]
    fn FS_Printf(file: *mut file_t, format: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn FS_Rename(oldname: *const libc::c_char, newname: *const libc::c_char)
     -> qboolean;
    #[no_mangle]
    fn FS_FileExists(filename: *const libc::c_char, gamedironly: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn FS_Delete(path: *const libc::c_char) -> qboolean;
    #[no_mangle]
    fn FS_Close(file: *mut file_t) -> libc::c_int;
    #[no_mangle]
    fn Con_Reportf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_DPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn CL_GetMaxClients() -> libc::c_int;
    #[no_mangle]
    static mut clgame: clgame_static_t;
    #[no_mangle]
    fn UI_AddTouchButtonToList(name: *const libc::c_char,
                               texture: *const libc::c_char,
                               command: *const libc::c_char,
                               color: *mut libc::c_uchar, flags: libc::c_int);
    #[no_mangle]
    fn Key_Event(key: libc::c_int, down: libc::c_int);
    #[no_mangle]
    fn UI_MouseMove(x: libc::c_int, y: libc::c_int);
    #[no_mangle]
    fn Con_PageDown(lines: libc::c_int);
    #[no_mangle]
    fn Con_PageUp(lines: libc::c_int);
    #[no_mangle]
    fn Con_Bottom();
    #[no_mangle]
    fn Key_EnableTextInput(enable: qboolean, force: qboolean);
    #[no_mangle]
    fn UI_KeyEvent(key: libc::c_int, down: qboolean);
    #[no_mangle]
    fn Con_UtfProcessChar(in_0: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn R_GetTextureParms(w: *mut libc::c_int, h: *mut libc::c_int,
                         texnum: libc::c_int);
    #[no_mangle]
    static mut cls: client_static_t;
    #[no_mangle]
    fn Con_DrawString(x: libc::c_int, y: libc::c_int,
                      string: *const libc::c_char, setColor: *mut byte)
     -> libc::c_int;
    #[link_name = "ref"]
    static mut ref_0: ref_state_s;
    #[no_mangle]
    static mut refState: ref_globals_t;
    #[no_mangle]
    fn VGui_KeyEvent(key: libc::c_int, down: libc::c_int);
    #[no_mangle]
    fn VGui_MouseMove(x: libc::c_int, y: libc::c_int);
    #[no_mangle]
    fn VGui_IsActive() -> qboolean;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const TIME_FILENAME: C2RustUnnamed_0 = 5;
pub const TIME_YEAR_ONLY: C2RustUnnamed_0 = 4;
pub const TIME_NO_SECONDS: C2RustUnnamed_0 = 3;
pub const TIME_TIME_ONLY: C2RustUnnamed_0 = 2;
pub const TIME_DATE_ONLY: C2RustUnnamed_0 = 1;
pub const TIME_FULL: C2RustUnnamed_0 = 0;
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
pub type touchEventType = libc::c_uint;
pub const event_motion: touchEventType = 2;
pub const event_up: touchEventType = 1;
pub const event_down: touchEventType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct touch_s {
    pub initialized: qboolean,
    pub config_loaded: qboolean,
    pub list_user: touchbuttonlist_t,
    pub list_edit: touchbuttonlist_t,
    pub mempool: poolhandle_t,
    pub state: touchState,
    pub look_finger: libc::c_int,
    pub move_finger: libc::c_int,
    pub wheel_finger: libc::c_int,
    pub move_button: *mut touch_button_t,
    pub move_start_x: libc::c_float,
    pub move_start_y: libc::c_float,
    pub wheel_amount: libc::c_float,
    pub wheel_up: string,
    pub wheel_down: string,
    pub wheel_end: string,
    pub wheel_count: libc::c_int,
    pub wheel_horizontal: qboolean,
    pub forward: libc::c_float,
    pub side: libc::c_float,
    pub yaw: libc::c_float,
    pub pitch: libc::c_float,
    pub edit: *mut touch_button_t,
    pub selection: *mut touch_button_t,
    pub hidebutton: *mut touch_button_t,
    pub resize_finger: libc::c_int,
    pub showeditbuttons: qboolean,
    pub clientonly: qboolean,
    pub scolor: rgba_t,
    pub swidth: libc::c_int,
    pub precision: qboolean,
    pub whitetexture: libc::c_int,
    pub joytexture: libc::c_int,
    pub configchanged: qboolean,
}
pub type touch_button_t = touch_button_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct touch_button_s {
    pub type_0: touchButtonType,
    pub x1: libc::c_float,
    pub y1: libc::c_float,
    pub x2: libc::c_float,
    pub y2: libc::c_float,
    pub texture: libc::c_int,
    pub color: rgba_t,
    pub texturefile: [libc::c_char; 256],
    pub command: [libc::c_char; 256],
    pub name: [libc::c_char; 32],
    pub finger: libc::c_int,
    pub flags: libc::c_int,
    pub fade: libc::c_float,
    pub fadespeed: libc::c_float,
    pub fadeend: libc::c_float,
    pub aspect: libc::c_float,
    pub next: *mut touch_button_s,
    pub prev: *mut touch_button_s,
}
pub type touchButtonType = libc::c_uint;
pub const touch_wheel: touchButtonType = 5;
pub const touch_look: touchButtonType = 4;
pub const touch_dpad: touchButtonType = 3;
pub const touch_joy: touchButtonType = 2;
pub const touch_move: touchButtonType = 1;
pub const touch_command: touchButtonType = 0;
pub type touchState = libc::c_uint;
pub const state_edit_move: touchState = 2;
pub const state_edit: touchState = 1;
pub const state_none: touchState = 0;
pub type touchbuttonlist_t = touchbuttonlist_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct touchbuttonlist_s {
    pub first: *mut touch_button_t,
    pub last: *mut touch_button_t,
}
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
pub type touchdefaultbutton_t = touchdefaultbutton_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct touchdefaultbutton_s {
    pub name: [libc::c_char; 32],
    pub texturefile: [libc::c_char; 256],
    pub command: [libc::c_char; 256],
    pub x1: libc::c_float,
    pub y1: libc::c_float,
    pub x2: libc::c_float,
    pub y2: libc::c_float,
    pub color: rgba_t,
    pub round: touchRound,
    pub aspect: libc::c_float,
    pub flags: libc::c_int,
}
pub type touchRound = libc::c_uint;
pub const round_aspect: touchRound = 2;
pub const round_grid: touchRound = 1;
pub const round_none: touchRound = 0;
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
#[inline(always)]
unsafe extern "C" fn __tg_fabs(mut __x: libc::c_float) -> libc::c_float {
    return fabsf(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_pow(mut __x: libc::c_float, mut __y: libc::c_float)
 -> libc::c_float {
    return powf(__x, __y);
}
#[inline(always)]
unsafe extern "C" fn __tg_exp(mut __x: libc::c_float) -> libc::c_float {
    return expf(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_sqrt(mut __x: libc::c_float) -> libc::c_float {
    return sqrtf(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_round(mut __x: libc::c_float) -> libc::c_float {
    return roundf(__x);
}
#[no_mangle]
pub static mut touch: touch_s =
    touch_s{initialized: false_0,
            config_loaded: false_0,
            list_user:
                touchbuttonlist_t{first:
                                      0 as *const touch_button_t as
                                          *mut touch_button_t,
                                  last:
                                      0 as *const touch_button_t as
                                          *mut touch_button_t,},
            list_edit:
                touchbuttonlist_t{first:
                                      0 as *const touch_button_t as
                                          *mut touch_button_t,
                                  last:
                                      0 as *const touch_button_t as
                                          *mut touch_button_t,},
            mempool: 0,
            state: state_none,
            look_finger: 0,
            move_finger: 0,
            wheel_finger: 0,
            move_button: 0 as *const touch_button_t as *mut touch_button_t,
            move_start_x: 0.,
            move_start_y: 0.,
            wheel_amount: 0.,
            wheel_up: [0; 256],
            wheel_down: [0; 256],
            wheel_end: [0; 256],
            wheel_count: 0,
            wheel_horizontal: false_0,
            forward: 0.,
            side: 0.,
            yaw: 0.,
            pitch: 0.,
            edit: 0 as *const touch_button_t as *mut touch_button_t,
            selection: 0 as *const touch_button_t as *mut touch_button_t,
            hidebutton: 0 as *const touch_button_t as *mut touch_button_t,
            resize_finger: 0,
            showeditbuttons: false_0,
            clientonly: false_0,
            scolor: [0; 4],
            swidth: 0,
            precision: false_0,
            whitetexture: 0,
            joytexture: 0,
            configchanged: false_0,};
#[no_mangle]
pub static mut g_DefaultButtons: [touchdefaultbutton_t; 256] =
    [touchdefaultbutton_t{name: [0; 32],
                          texturefile: [0; 256],
                          command: [0; 256],
                          x1: 0.,
                          y1: 0.,
                          x2: 0.,
                          y2: 0.,
                          color: [0; 4],
                          round: round_none,
                          aspect: 0.,
                          flags: 0,}; 256];
#[no_mangle]
pub static mut g_LastDefaultButton: libc::c_int = 0;
#[no_mangle]
pub static mut touch_pitch: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut touch_yaw: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut touch_forwardzone: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut touch_sidezone: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut touch_nonlinear_look: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut touch_pow_mult: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut touch_pow_factor: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut touch_exp_mult: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut touch_grid_enable: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut touch_grid_count: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut touch_config_file: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut touch_enable: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut touch_in_menu: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut touch_joy_radius: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut touch_dpad_radius: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut touch_move_indicator: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut touch_highlight_r: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut touch_highlight_g: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut touch_highlight_b: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut touch_highlight_a: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut touch_precise_amount: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut touch_joy_texture: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut touch_emulate: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
// Touch button type: tap, stick or slider
// Field of button
// Button texture
// Double-linked list
/*
==========================
Touch_ExportButtonToConfig

writes button data to config
returns 0 on success, non-zero on error
==========================
*/
#[inline]
unsafe extern "C" fn Touch_ExportButtonToConfig(mut f: *mut file_t,
                                                mut button:
                                                    *mut touch_button_t,
                                                mut keepAspect: qboolean)
 -> libc::c_int {
    let mut newCommand: string = [0; 256]; // skip temporary buttons
    let mut flags: libc::c_int = (*button).flags;
    if flags as libc::c_uint & (1 as libc::c_uint) << 2 as libc::c_int != 0 {
        return 1 as libc::c_int
    }
    if flags as libc::c_uint & (1 as libc::c_uint) << 5 as libc::c_int != 0 {
        flags =
            (flags as libc::c_uint &
                 !((1 as libc::c_uint) << 0 as libc::c_int)) as libc::c_int
    }
    if flags as libc::c_uint & (1 as libc::c_uint) << 6 as libc::c_int != 0 {
        flags =
            (flags as libc::c_uint | (1 as libc::c_uint) << 0 as libc::c_int)
                as libc::c_int
    }
    Cmd_Escape(newCommand.as_mut_ptr(), (*button).command.as_mut_ptr(),
               ::std::mem::size_of::<string>() as libc::c_ulong as
                   libc::c_int);
    FS_Printf(f,
              b"touch_addbutton \"%s\" \"%s\" \"%s\" %f %f %f %f %d %d %d %d %d\x00"
                  as *const u8 as *const libc::c_char,
              (*button).name.as_mut_ptr(), (*button).texturefile.as_mut_ptr(),
              newCommand.as_mut_ptr(), (*button).x1 as libc::c_double,
              (*button).y1 as libc::c_double, (*button).x2 as libc::c_double,
              (*button).y2 as libc::c_double,
              (*button).color[0 as libc::c_int as usize] as libc::c_int,
              (*button).color[1 as libc::c_int as usize] as libc::c_int,
              (*button).color[2 as libc::c_int as usize] as libc::c_int,
              (*button).color[3 as libc::c_int as usize] as libc::c_int,
              flags);
    if keepAspect as u64 != 0 {
        let mut aspect: libc::c_float =
            ((*button).y2 - (*button).y1) /
                (((*button).x2 - (*button).x1) /
                     (refState.height as libc::c_float /
                          refState.width as libc::c_float));
        FS_Printf(f, b" %f\n\x00" as *const u8 as *const libc::c_char,
                  aspect as libc::c_double);
    } else { FS_Printf(f, b"\n\x00" as *const u8 as *const libc::c_char); }
    return 0 as libc::c_int;
}
/*
=================
Touch_WriteConfig

save current touch configuration
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Touch_WriteConfig() {
    let mut f: *mut file_t = 0 as *mut file_t;
    let mut newconfigfile: string = [0; 256];
    let mut oldconfigfile: string = [0; 256];
    if touch.list_user.first.is_null() { return }
    if Sys_CheckParm(b"-nowriteconfig\x00" as *const u8 as
                         *const libc::c_char) != 0 ||
           touch.configchanged as u64 == 0 || touch.config_loaded as u64 == 0
       {
        return
    }
    Con_DPrintf(b"Touch_WriteConfig(): %s\n\x00" as *const u8 as
                    *const libc::c_char, (*touch_config_file).string);
    Q_snprintf(newconfigfile.as_mut_ptr(),
               ::std::mem::size_of::<string>() as libc::c_ulong,
               b"%s.new\x00" as *const u8 as *const libc::c_char,
               (*touch_config_file).string);
    Q_snprintf(oldconfigfile.as_mut_ptr(),
               ::std::mem::size_of::<string>() as libc::c_ulong,
               b"%s.bak\x00" as *const u8 as *const libc::c_char,
               (*touch_config_file).string);
    f =
        FS_Open(newconfigfile.as_mut_ptr(),
                b"w\x00" as *const u8 as *const libc::c_char, true_0);
    if !f.is_null() {
        let mut button: *mut touch_button_t = 0 as *mut touch_button_t;
        FS_Printf(f,
                  b"//=======================================================================\n\x00"
                      as *const u8 as *const libc::c_char);
        FS_Printf(f,
                  b"//\tCopyright FWGS & XashXT group %s (c)\n\x00" as
                      *const u8 as *const libc::c_char,
                  Q_timestamp(TIME_YEAR_ONLY as libc::c_int));
        FS_Printf(f,
                  b"//\t\t\ttouchscreen config\n\x00" as *const u8 as
                      *const libc::c_char);
        FS_Printf(f,
                  b"//=======================================================================\n\x00"
                      as *const u8 as *const libc::c_char);
        FS_Printf(f,
                  b"\ntouch_config_file \"%s\"\n\x00" as *const u8 as
                      *const libc::c_char, (*touch_config_file).string);
        FS_Printf(f,
                  b"\n// touch cvars\n\x00" as *const u8 as
                      *const libc::c_char);
        FS_Printf(f,
                  b"\n// sensitivity settings\n\x00" as *const u8 as
                      *const libc::c_char);
        FS_Printf(f,
                  b"touch_pitch \"%f\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  (*touch_pitch).value as libc::c_double);
        FS_Printf(f,
                  b"touch_yaw \"%f\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  (*touch_yaw).value as libc::c_double);
        FS_Printf(f,
                  b"touch_forwardzone \"%f\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  (*touch_forwardzone).value as libc::c_double);
        FS_Printf(f,
                  b"touch_sidezone \"%f\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  (*touch_sidezone).value as libc::c_double);
        FS_Printf(f,
                  b"touch_nonlinear_look \"%d\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  if !touch_nonlinear_look.is_null() &&
                         (*touch_nonlinear_look).value != 0.0f32 {
                      true_0 as libc::c_int
                  } else { false_0 as libc::c_int });
        FS_Printf(f,
                  b"touch_pow_factor \"%f\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  (*touch_pow_factor).value as libc::c_double);
        FS_Printf(f,
                  b"touch_pow_mult \"%f\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  (*touch_pow_mult).value as libc::c_double);
        FS_Printf(f,
                  b"touch_exp_mult \"%f\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  (*touch_exp_mult).value as libc::c_double);
        FS_Printf(f,
                  b"\n// grid settings\n\x00" as *const u8 as
                      *const libc::c_char);
        FS_Printf(f,
                  b"touch_grid_count \"%d\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  (*touch_grid_count).value as libc::c_int);
        FS_Printf(f,
                  b"touch_grid_enable \"%d\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  if !touch_grid_enable.is_null() &&
                         (*touch_grid_enable).value != 0.0f32 {
                      true_0 as libc::c_int
                  } else { false_0 as libc::c_int });
        FS_Printf(f,
                  b"\n// global overstroke (width, r, g, b, a)\n\x00" as
                      *const u8 as *const libc::c_char);
        FS_Printf(f,
                  b"touch_set_stroke %d %d %d %d %d\n\x00" as *const u8 as
                      *const libc::c_char, touch.swidth,
                  touch.scolor[0 as libc::c_int as usize] as libc::c_int,
                  touch.scolor[1 as libc::c_int as usize] as libc::c_int,
                  touch.scolor[2 as libc::c_int as usize] as libc::c_int,
                  touch.scolor[3 as libc::c_int as usize] as libc::c_int);
        FS_Printf(f,
                  b"\n// highlight when pressed\n\x00" as *const u8 as
                      *const libc::c_char);
        FS_Printf(f,
                  b"touch_highlight_r \"%f\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  (*touch_highlight_r).value as libc::c_double);
        FS_Printf(f,
                  b"touch_highlight_g \"%f\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  (*touch_highlight_g).value as libc::c_double);
        FS_Printf(f,
                  b"touch_highlight_b \"%f\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  (*touch_highlight_b).value as libc::c_double);
        FS_Printf(f,
                  b"touch_highlight_a \"%f\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  (*touch_highlight_a).value as libc::c_double);
        FS_Printf(f,
                  b"\n// _joy and _dpad options\n\x00" as *const u8 as
                      *const libc::c_char);
        FS_Printf(f,
                  b"touch_dpad_radius \"%f\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  (*touch_dpad_radius).value as libc::c_double);
        FS_Printf(f,
                  b"touch_joy_radius \"%f\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  (*touch_joy_radius).value as libc::c_double);
        FS_Printf(f,
                  b"\n// how much slowdown when Precise Look button pressed\n\x00"
                      as *const u8 as *const libc::c_char);
        FS_Printf(f,
                  b"touch_precise_amount \"%f\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  (*touch_precise_amount).value as libc::c_double);
        FS_Printf(f,
                  b"\n// enable/disable move indicator\n\x00" as *const u8 as
                      *const libc::c_char);
        FS_Printf(f,
                  b"touch_move_indicator \"%f\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  (*touch_move_indicator).value as libc::c_double);
        FS_Printf(f,
                  b"\n// reset menu state when execing config\n\x00" as
                      *const u8 as *const libc::c_char);
        FS_Printf(f,
                  b"touch_setclientonly 0\n\x00" as *const u8 as
                      *const libc::c_char);
        FS_Printf(f,
                  b"\n// touch buttons\n\x00" as *const u8 as
                      *const libc::c_char);
        FS_Printf(f,
                  b"touch_removeall\n\x00" as *const u8 as
                      *const libc::c_char);
        button = touch.list_user.first;
        while !button.is_null() {
            Touch_ExportButtonToConfig(f, button, false_0);
            button = (*button).next
        }
        FS_Close(f);
        FS_Delete(oldconfigfile.as_mut_ptr());
        FS_Rename((*touch_config_file).string, oldconfigfile.as_mut_ptr());
        FS_Delete((*touch_config_file).string);
        FS_Rename(newconfigfile.as_mut_ptr(), (*touch_config_file).string);
    } else {
        Con_Printf(b"^1Error:^7 Couldn\'t write %s.\n\x00" as *const u8 as
                       *const libc::c_char, (*touch_config_file).string);
    };
}
/*
=================
Touch_ExportConfig_f

export current touch configuration into profile
=================
*/
unsafe extern "C" fn Touch_ExportConfig_f() {
    let mut f: *mut file_t = 0 as *mut file_t;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    if Cmd_Argc() != 2 as libc::c_int {
        Con_Printf(b"Usage: touch_exportconfig <name>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    if touch.list_user.first.is_null() { return }
    name = Cmd_Argv(1 as libc::c_int);
    Con_Reportf(b"Exporting config to %s\n\x00" as *const u8 as
                    *const libc::c_char, name);
    f = FS_Open(name, b"w\x00" as *const u8 as *const libc::c_char, true_0);
    if !f.is_null() {
        let mut profilename: string = [0; 256];
        let mut profilebase: string = [0; 256];
        let mut button: *mut touch_button_t = 0 as *mut touch_button_t;
        if !Q_strstr(name,
                     b"touch_presets/\x00" as *const u8 as
                         *const libc::c_char).is_null() {
            COM_FileBase(name, profilebase.as_mut_ptr());
            Q_snprintf(profilename.as_mut_ptr(),
                       ::std::mem::size_of::<string>() as libc::c_ulong,
                       b"touch_profiles/%s (copy).cfg\x00" as *const u8 as
                           *const libc::c_char, profilebase.as_mut_ptr());
        } else {
            Q_strncpy(profilename.as_mut_ptr(), name,
                      ::std::mem::size_of::<string>() as libc::c_ulong);
        }
        FS_Printf(f,
                  b"//=======================================================================\n\x00"
                      as *const u8 as *const libc::c_char);
        FS_Printf(f,
                  b"//\tCopyright FWGS & XashXT group %s (c)\n\x00" as
                      *const u8 as *const libc::c_char,
                  Q_timestamp(TIME_YEAR_ONLY as libc::c_int));
        FS_Printf(f,
                  b"//\t\t\ttouchscreen preset\n\x00" as *const u8 as
                      *const libc::c_char);
        FS_Printf(f,
                  b"//=======================================================================\n\x00"
                      as *const u8 as *const libc::c_char);
        FS_Printf(f,
                  b"\ntouch_config_file \"%s\"\n\x00" as *const u8 as
                      *const libc::c_char, profilename.as_mut_ptr());
        FS_Printf(f,
                  b"\n// touch cvars\n\x00" as *const u8 as
                      *const libc::c_char);
        FS_Printf(f,
                  b"\n// sensitivity settings\n\x00" as *const u8 as
                      *const libc::c_char);
        FS_Printf(f,
                  b"touch_pitch \"%f\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  (*touch_pitch).value as libc::c_double);
        FS_Printf(f,
                  b"touch_yaw \"%f\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  (*touch_yaw).value as libc::c_double);
        FS_Printf(f,
                  b"touch_forwardzone \"%f\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  (*touch_forwardzone).value as libc::c_double);
        FS_Printf(f,
                  b"touch_sidezone \"%f\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  (*touch_sidezone).value as libc::c_double);
        FS_Printf(f,
                  b"touch_nonlinear_look \"%d\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  if !touch_nonlinear_look.is_null() &&
                         (*touch_nonlinear_look).value != 0.0f32 {
                      true_0 as libc::c_int
                  } else { false_0 as libc::c_int });
        FS_Printf(f,
                  b"touch_pow_factor \"%f\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  (*touch_pow_factor).value as libc::c_double);
        FS_Printf(f,
                  b"touch_pow_mult \"%f\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  (*touch_pow_mult).value as libc::c_double);
        FS_Printf(f,
                  b"touch_exp_mult \"%f\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  (*touch_exp_mult).value as libc::c_double);
        FS_Printf(f,
                  b"\n// grid settings\n\x00" as *const u8 as
                      *const libc::c_char);
        FS_Printf(f,
                  b"touch_grid_count \"%d\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  (*touch_grid_count).value as libc::c_int);
        FS_Printf(f,
                  b"touch_grid_enable \"%d\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  if !touch_grid_enable.is_null() &&
                         (*touch_grid_enable).value != 0.0f32 {
                      true_0 as libc::c_int
                  } else { false_0 as libc::c_int });
        FS_Printf(f,
                  b"\n// global overstroke (width, r, g, b, a)\n\x00" as
                      *const u8 as *const libc::c_char);
        FS_Printf(f,
                  b"touch_set_stroke %d %d %d %d %d\n\x00" as *const u8 as
                      *const libc::c_char, touch.swidth,
                  touch.scolor[0 as libc::c_int as usize] as libc::c_int,
                  touch.scolor[1 as libc::c_int as usize] as libc::c_int,
                  touch.scolor[2 as libc::c_int as usize] as libc::c_int,
                  touch.scolor[3 as libc::c_int as usize] as libc::c_int);
        FS_Printf(f,
                  b"\n// highlight when pressed\n\x00" as *const u8 as
                      *const libc::c_char);
        FS_Printf(f,
                  b"touch_highlight_r \"%f\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  (*touch_highlight_r).value as libc::c_double);
        FS_Printf(f,
                  b"touch_highlight_g \"%f\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  (*touch_highlight_g).value as libc::c_double);
        FS_Printf(f,
                  b"touch_highlight_b \"%f\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  (*touch_highlight_b).value as libc::c_double);
        FS_Printf(f,
                  b"touch_highlight_a \"%f\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  (*touch_highlight_a).value as libc::c_double);
        FS_Printf(f,
                  b"\n// _joy and _dpad options\n\x00" as *const u8 as
                      *const libc::c_char);
        FS_Printf(f,
                  b"touch_dpad_radius \"%f\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  (*touch_dpad_radius).value as libc::c_double);
        FS_Printf(f,
                  b"touch_joy_radius \"%f\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  (*touch_joy_radius).value as libc::c_double);
        FS_Printf(f,
                  b"\n// how much slowdown when Precise Look button pressed\n\x00"
                      as *const u8 as *const libc::c_char);
        FS_Printf(f,
                  b"touch_precise_amount \"%f\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  (*touch_precise_amount).value as libc::c_double);
        FS_Printf(f,
                  b"\n// enable/disable move indicator\n\x00" as *const u8 as
                      *const libc::c_char);
        FS_Printf(f,
                  b"touch_move_indicator \"%f\"\n\x00" as *const u8 as
                      *const libc::c_char,
                  (*touch_move_indicator).value as libc::c_double);
        FS_Printf(f,
                  b"\n// reset menu state when execing config\n\x00" as
                      *const u8 as *const libc::c_char);
        FS_Printf(f,
                  b"touch_setclientonly 0\n\x00" as *const u8 as
                      *const libc::c_char);
        FS_Printf(f,
                  b"\n// touch buttons\n\x00" as *const u8 as
                      *const libc::c_char);
        FS_Printf(f,
                  b"touch_removeall\n\x00" as *const u8 as
                      *const libc::c_char);
        button = touch.list_user.first;
        while !button.is_null() {
            Touch_ExportButtonToConfig(f, button, true_0);
            button = (*button).next
        }
        FS_Printf(f,
                  b"\n// round button coordinates to grid\n\x00" as *const u8
                      as *const libc::c_char);
        FS_Printf(f,
                  b"touch_roundall\n\x00" as *const u8 as
                      *const libc::c_char);
        FS_Close(f);
    } else {
        Con_Printf(b"^1Error:^7 Couldn\'t write %s.\n\x00" as *const u8 as
                       *const libc::c_char, name);
    };
}
/*
=================
Touch_GenerateCode_f

export current touch configuration into C code
=================
*/
unsafe extern "C" fn Touch_GenerateCode_f() {
    let mut button: *mut touch_button_t =
        0 as *mut touch_button_t; // skip temporary buttons
    let mut c: rgba_t =
        [0 as libc::c_int as byte, 0 as libc::c_int as byte,
         0 as libc::c_int as byte, 0 as libc::c_int as byte];
    if touch.list_user.first.is_null() { return }
    button = touch.list_user.first;
    while !button.is_null() {
        let mut aspect: libc::c_float = 0.;
        let mut flags: libc::c_int = (*button).flags;
        if !(flags as libc::c_uint & (1 as libc::c_uint) << 2 as libc::c_int
                 != 0) {
            if flags as libc::c_uint & (1 as libc::c_uint) << 5 as libc::c_int
                   != 0 {
                flags =
                    (flags as libc::c_uint &
                         !((1 as libc::c_uint) << 0 as libc::c_int)) as
                        libc::c_int
            }
            if flags as libc::c_uint & (1 as libc::c_uint) << 6 as libc::c_int
                   != 0 {
                flags =
                    (flags as libc::c_uint |
                         (1 as libc::c_uint) << 0 as libc::c_int) as
                        libc::c_int
            }
            aspect =
                ((*button).y2 - (*button).y1) /
                    (((*button).x2 - (*button).x1) /
                         (refState.height as libc::c_float /
                              refState.width as libc::c_float));
            if memcmp(&mut c as *mut rgba_t as *const libc::c_void,
                      &mut (*button).color as *mut rgba_t as
                          *const libc::c_void,
                      ::std::mem::size_of::<rgba_t>() as libc::c_ulong) != 0 {
                Con_Printf(b"unsigned char color[] = { %d, %d, %d, %d };\n\x00"
                               as *const u8 as *const libc::c_char,
                           (*button).color[0 as libc::c_int as usize] as
                               libc::c_int,
                           (*button).color[1 as libc::c_int as usize] as
                               libc::c_int,
                           (*button).color[2 as libc::c_int as usize] as
                               libc::c_int,
                           (*button).color[3 as libc::c_int as usize] as
                               libc::c_int);
                memcpy(&mut c as *mut rgba_t as *mut libc::c_void,
                       &mut (*button).color as *mut rgba_t as
                           *const libc::c_void,
                       ::std::mem::size_of::<rgba_t>() as libc::c_ulong);
            }
            Con_Printf(b"TOUCH_ADDDEFAULT( \"%s\", \"%s\", \"%s\", %f, %f, %f, %f, color, %d, %f, %d );\n\x00"
                           as *const u8 as *const libc::c_char,
                       (*button).name.as_mut_ptr(),
                       (*button).texturefile.as_mut_ptr(),
                       (*button).command.as_mut_ptr(),
                       (*button).x1 as libc::c_double,
                       (*button).y1 as libc::c_double,
                       (*button).x2 as libc::c_double,
                       (*button).y2 as libc::c_double,
                       if (*button).type_0 as libc::c_uint ==
                              touch_command as libc::c_int as libc::c_uint {
                           if (__tg_fabs(aspect - 1.0f32) as libc::c_double) <
                                  0.0001f64 {
                               2 as libc::c_int
                           } else { 1 as libc::c_int }
                       } else { 0 as libc::c_int }, aspect as libc::c_double,
                       flags);
        }
        button = (*button).next
    };
}
unsafe extern "C" fn Touch_RoundAll_f() {
    let mut button: *mut touch_button_t = 0 as *mut touch_button_t;
    if (*touch_grid_enable).value == 0. { return }
    button = touch.list_user.first;
    while !button.is_null() {
        IN_TouchCheckCoords(&mut (*button).x1, &mut (*button).y1,
                            &mut (*button).x2, &mut (*button).y2);
        button = (*button).next
    };
}
unsafe extern "C" fn Touch_ListButtons_f() {
    let mut button: *mut touch_button_t = 0 as *mut touch_button_t;
    Touch_InitConfig();
    button = touch.list_user.first;
    while !button.is_null() {
        Con_Printf(b"%s %s %s %f %f %f %f %d %d %d %d %d\n\x00" as *const u8
                       as *const libc::c_char, (*button).name.as_mut_ptr(),
                   (*button).texturefile.as_mut_ptr(),
                   (*button).command.as_mut_ptr(),
                   (*button).x1 as libc::c_double,
                   (*button).y1 as libc::c_double,
                   (*button).x2 as libc::c_double,
                   (*button).y2 as libc::c_double,
                   (*button).color[0 as libc::c_int as usize] as libc::c_int,
                   (*button).color[1 as libc::c_int as usize] as libc::c_int,
                   (*button).color[2 as libc::c_int as usize] as libc::c_int,
                   (*button).color[3 as libc::c_int as usize] as libc::c_int,
                   (*button).flags);
        if !((*button).flags as libc::c_uint &
                 (1 as libc::c_uint) << 2 as libc::c_int != 0) {
            UI_AddTouchButtonToList((*button).name.as_mut_ptr(),
                                    (*button).texturefile.as_mut_ptr(),
                                    (*button).command.as_mut_ptr(),
                                    (*button).color.as_mut_ptr(),
                                    (*button).flags);
        }
        button = (*button).next
    }
    touch.configchanged = true_0;
}
unsafe extern "C" fn Touch_Stroke_f() {
    if Cmd_Argc() != 6 as libc::c_int {
        Con_Printf(b"Usage: touch_set_stroke <width> <r> <g> <b> <a>\n\x00" as
                       *const u8 as *const libc::c_char);
        return
    }
    touch.swidth = Q_atoi(Cmd_Argv(1 as libc::c_int));
    touch.scolor[0 as libc::c_int as usize] =
        Q_atoi(Cmd_Argv(2 as libc::c_int)) as byte;
    touch.scolor[1 as libc::c_int as usize] =
        Q_atoi(Cmd_Argv(3 as libc::c_int)) as byte;
    touch.scolor[2 as libc::c_int as usize] =
        Q_atoi(Cmd_Argv(4 as libc::c_int)) as byte;
    touch.scolor[3 as libc::c_int as usize] =
        Q_atoi(Cmd_Argv(5 as libc::c_int)) as byte;
}
unsafe extern "C" fn Touch_FindButton(mut list: *mut touchbuttonlist_t,
                                      mut name: *const libc::c_char,
                                      mut privileged: qboolean)
 -> *mut touch_button_t {
    let mut button: *mut touch_button_t = 0 as *mut touch_button_t;
    button = (*list).first;
    while !button.is_null() {
        if !(privileged as u64 == 0 &&
                 (*button).flags as libc::c_uint &
                     (1 as libc::c_uint) << 10 as libc::c_int == 0) {
            if !(Q_strncmp((*button).name.as_mut_ptr(), name,
                           ::std::mem::size_of::<[libc::c_char; 32]>() as
                               libc::c_ulong as libc::c_int) != 0) {
                return button
            }
        }
        button = (*button).next
    }
    return 0 as *mut touch_button_t;
}
unsafe extern "C" fn Touch_FindFirst(mut list: *mut touchbuttonlist_t,
                                     mut name: *const libc::c_char,
                                     mut privileged: qboolean)
 -> *mut touch_button_t {
    let mut button: *mut touch_button_t = 0 as *mut touch_button_t;
    button = (*list).first;
    while !button.is_null() {
        if !(privileged as u64 == 0 &&
                 (*button).flags as libc::c_uint &
                     (1 as libc::c_uint) << 10 as libc::c_int == 0) {
            if !Q_strchr(name, '*' as i32 as libc::c_char).is_null() &&
                   Q_stricmpext(name, (*button).name.as_mut_ptr()) as
                       libc::c_uint != 0 ||
                   Q_strncmp(name, (*button).name.as_mut_ptr(),
                             ::std::mem::size_of::<[libc::c_char; 32]>() as
                                 libc::c_ulong as libc::c_int) == 0 {
                return button
            }
        }
        button = (*button).next
    }
    return 0 as *mut touch_button_t;
}
#[no_mangle]
pub unsafe extern "C" fn Touch_SetClientOnly(mut state: byte) {
    touch.clientonly = state as qboolean;
    host.mouse_visible = state as qboolean;
    touch.look_finger = -(1 as libc::c_int);
    touch.move_finger = touch.look_finger;
    touch.side = 0 as libc::c_int as libc::c_float;
    touch.forward = touch.side;
    // / TODO: touch sdl platform
}
unsafe extern "C" fn Touch_SetClientOnly_f() {
    if Cmd_Argc() != 2 as libc::c_int {
        Con_Printf(b"Usage: touch_setclientonly <state>\n\x00" as *const u8 as
                       *const libc::c_char); // mark for texture load
        return
    } //replace if exist
    Touch_SetClientOnly(Q_atoi(Cmd_Argv(1 as libc::c_int)) as byte);
}
unsafe extern "C" fn Touch_RemoveButtonFromList(mut list:
                                                    *mut touchbuttonlist_t,
                                                mut name: *const libc::c_char,
                                                mut privileged: qboolean) {
    let mut button: *mut touch_button_t = 0 as *mut touch_button_t;
    IN_TouchEditClear();
    loop  {
        button =
            Touch_FindFirst(&mut touch.list_user, name,
                            (privileged as u64 == 0) as libc::c_int as
                                qboolean);
        if button.is_null() { break ; }
        if !(*button).prev.is_null() {
            (*(*button).prev).next = (*button).next
        } else { (*list).first = (*button).next }
        if !(*button).next.is_null() {
            (*(*button).next).prev = (*button).prev
        } else { (*list).last = (*button).prev }
        _Mem_Free(button as *mut libc::c_void,
                  b"../engine/client/in_touch.c\x00" as *const u8 as
                      *const libc::c_char, 551 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Touch_RemoveButton(mut name: *const libc::c_char,
                                            mut privileged: qboolean) {
    Touch_RemoveButtonFromList(&mut touch.list_user, name, privileged);
}
unsafe extern "C" fn IN_TouchRemoveButton_f() {
    if Cmd_Argc() != 2 as libc::c_int {
        Con_Printf(b"Usage: touch_removebutton <button>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    Touch_RemoveButton(Cmd_Argv(1 as libc::c_int),
                       Cmd_CurrentCommandIsPrivileged());
}
unsafe extern "C" fn Touch_ClearList(mut list: *mut touchbuttonlist_t) {
    while !(*list).first.is_null() {
        let mut remove: *mut touch_button_t = (*list).first;
        (*list).first = (*(*list).first).next;
        _Mem_Free(remove as *mut libc::c_void,
                  b"../engine/client/in_touch.c\x00" as *const u8 as
                      *const libc::c_char, 578 as libc::c_int);
    }
    (*list).last = 0 as *mut touch_button_t;
    (*list).first = (*list).last;
}
unsafe extern "C" fn Touch_RemoveAll_f() {
    IN_TouchEditClear();
    Touch_ClearList(&mut touch.list_user);
}
unsafe extern "C" fn Touch_SetColor(mut list: *mut touchbuttonlist_t,
                                    mut name: *const libc::c_char,
                                    mut color: *mut byte) {
    let mut button: *mut touch_button_t = 0 as *mut touch_button_t;
    button = (*list).first;
    while !button.is_null() {
        if !Q_strchr(name, '*' as i32 as libc::c_char).is_null() &&
               Q_stricmpext(name, (*button).name.as_mut_ptr()) as libc::c_uint
                   != 0 ||
               Q_strncmp(name, (*button).name.as_mut_ptr(),
                         ::std::mem::size_of::<[libc::c_char; 32]>() as
                             libc::c_ulong as libc::c_int) == 0 {
            (*button).color[0 as libc::c_int as usize] =
                *color.offset(0 as libc::c_int as isize);
            (*button).color[1 as libc::c_int as usize] =
                *color.offset(1 as libc::c_int as isize);
            (*button).color[2 as libc::c_int as usize] =
                *color.offset(2 as libc::c_int as isize);
            (*button).color[3 as libc::c_int as usize] =
                *color.offset(3 as libc::c_int as isize)
        }
        button = (*button).next
    };
}
unsafe extern "C" fn Touch_SetTexture(mut list: *mut touchbuttonlist_t,
                                      mut name: *const libc::c_char,
                                      mut texture: *const libc::c_char,
                                      mut privileged: qboolean) {
    let mut button: *mut touch_button_t =
        Touch_FindButton(list, name, privileged);
    if button.is_null() { return }
    (*button).texture = -(1 as libc::c_int);
    Q_strncpy((*button).texturefile.as_mut_ptr(), texture,
              ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong);
}
unsafe extern "C" fn Touch_SetCommand(mut button: *mut touch_button_t,
                                      mut command: *const libc::c_char) {
    Q_strncpy((*button).command.as_mut_ptr(), command,
              ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong);
    if Q_strncmp(command, b"_look\x00" as *const u8 as *const libc::c_char,
                 99999 as libc::c_int) == 0 {
        (*button).type_0 = touch_look
    } else if Q_strncmp(command,
                        b"_move\x00" as *const u8 as *const libc::c_char,
                        99999 as libc::c_int) == 0 {
        (*button).type_0 = touch_move
    } else if Q_strncmp(command,
                        b"_joy\x00" as *const u8 as *const libc::c_char,
                        99999 as libc::c_int) == 0 {
        (*button).type_0 = touch_joy
    } else if Q_strncmp(command,
                        b"_dpad\x00" as *const u8 as *const libc::c_char,
                        99999 as libc::c_int) == 0 {
        (*button).type_0 = touch_dpad
    } else if Q_stricmpext(b"_wheel *\x00" as *const u8 as
                               *const libc::c_char, command) as libc::c_uint
                  != 0 ||
                  Q_stricmpext(b"_hwheel *\x00" as *const u8 as
                                   *const libc::c_char, command) as
                      libc::c_uint != 0 {
        (*button).type_0 = touch_wheel
    };
}
#[no_mangle]
pub unsafe extern "C" fn Touch_HideButtons(mut name: *const libc::c_char,
                                           mut hide: byte,
                                           mut privileged: qboolean) {
    let mut button: *mut touch_button_t = 0 as *mut touch_button_t;
    button = touch.list_user.first;
    while !button.is_null() {
        if !(privileged as u64 == 0 &&
                 (*button).flags as libc::c_uint &
                     (1 as libc::c_uint) << 10 as libc::c_int == 0) {
            if !Q_strchr(name, '*' as i32 as libc::c_char).is_null() &&
                   Q_stricmpext(name, (*button).name.as_mut_ptr()) as
                       libc::c_uint != 0 ||
                   Q_strncmp(name, (*button).name.as_mut_ptr(),
                             ::std::mem::size_of::<[libc::c_char; 32]>() as
                                 libc::c_ulong as libc::c_int) == 0 {
                if hide != 0 {
                    (*button).flags =
                        ((*button).flags as libc::c_uint |
                             (1 as libc::c_uint) << 0 as libc::c_int) as
                            libc::c_int
                } else {
                    (*button).flags =
                        ((*button).flags as libc::c_uint &
                             !((1 as libc::c_uint) << 0 as libc::c_int)) as
                            libc::c_int
                }
            }
        }
        button = (*button).next
    };
}
unsafe extern "C" fn Touch_ToggleSelection_f() {
    if !touch.selection.is_null() {
        (*touch.selection).flags =
            ((*touch.selection).flags as libc::c_uint ^
                 (1 as libc::c_uint) << 0 as libc::c_int) as libc::c_int
    };
}
unsafe extern "C" fn Touch_Hide_f() {
    if Cmd_Argc() != 2 as libc::c_int {
        Con_Printf(b"Usage: touch_hide <button>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    Touch_HideButtons(Cmd_Argv(1 as libc::c_int),
                      true_0 as libc::c_int as libc::c_uchar,
                      Cmd_CurrentCommandIsPrivileged());
}
unsafe extern "C" fn Touch_Show_f() {
    if Cmd_Argc() != 2 as libc::c_int {
        Con_Printf(b"Usage: touch_show <button>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    Touch_HideButtons(Cmd_Argv(1 as libc::c_int),
                      false_0 as libc::c_int as libc::c_uchar,
                      Cmd_CurrentCommandIsPrivileged());
}
unsafe extern "C" fn Touch_FadeButtons(mut list: *mut touchbuttonlist_t,
                                       mut name: *const libc::c_char,
                                       mut speed: libc::c_float,
                                       mut end: libc::c_float,
                                       mut start: libc::c_float,
                                       mut privileged: qboolean) {
    let mut button: *mut touch_button_t = 0 as *mut touch_button_t;
    button = (*list).first;
    while !button.is_null() {
        if !(privileged as u64 == 0 &&
                 (*button).flags as libc::c_uint &
                     (1 as libc::c_uint) << 10 as libc::c_int == 0) {
            if !Q_strchr(name, '*' as i32 as libc::c_char).is_null() &&
                   Q_stricmpext(name, (*button).name.as_mut_ptr()) as
                       libc::c_uint != 0 ||
                   Q_strncmp(name, (*button).name.as_mut_ptr(),
                             ::std::mem::size_of::<[libc::c_char; 32]>() as
                                 libc::c_ulong as libc::c_int) == 0 {
                if start >= 0 as libc::c_int as libc::c_float {
                    (*button).fade = start
                }
                (*button).fadespeed = speed;
                (*button).fadeend = end
            }
        }
        button = (*button).next
    };
}
unsafe extern "C" fn Touch_Fade_f() {
    let mut start: libc::c_float = -(1 as libc::c_int) as libc::c_float;
    if Cmd_Argc() == 5 as libc::c_int {
        start = Q_atof(Cmd_Argv(4 as libc::c_int))
    } else if Cmd_Argc() != 4 as libc::c_int {
        Con_Printf(b"Usage: touch_fade <button> <speed> <end> [start]\n\x00"
                       as *const u8 as *const libc::c_char);
        return
    }
    Touch_FadeButtons(&mut touch.list_user, Cmd_Argv(1 as libc::c_int),
                      Q_atof(Cmd_Argv(2 as libc::c_int)),
                      Q_atof(Cmd_Argv(3 as libc::c_int)), start,
                      Cmd_CurrentCommandIsPrivileged());
}
unsafe extern "C" fn Touch_SetColor_f() {
    let mut color: rgba_t = [0; 4];
    if Cmd_Argc() == 6 as libc::c_int {
        color[0 as libc::c_int as usize] =
            Q_atoi(Cmd_Argv(2 as libc::c_int)) as byte;
        color[1 as libc::c_int as usize] =
            Q_atoi(Cmd_Argv(3 as libc::c_int)) as byte;
        color[2 as libc::c_int as usize] =
            Q_atoi(Cmd_Argv(4 as libc::c_int)) as byte;
        color[3 as libc::c_int as usize] =
            Q_atoi(Cmd_Argv(5 as libc::c_int)) as byte;
        Touch_SetColor(&mut touch.list_user, Cmd_Argv(1 as libc::c_int),
                       color.as_mut_ptr());
        return
    }
    Con_Printf(b"Usage: touch_setcolor <pattern> <r> <g> <b> <a>\n\x00" as
                   *const u8 as *const libc::c_char);
}
unsafe extern "C" fn Touch_SetTexture_f() {
    if Cmd_Argc() == 3 as libc::c_int {
        Touch_SetTexture(&mut touch.list_user, Cmd_Argv(1 as libc::c_int),
                         Cmd_Argv(2 as libc::c_int),
                         Cmd_CurrentCommandIsPrivileged());
        return
    }
    Con_Printf(b"Usage: touch_settexture <name> <file>\n\x00" as *const u8 as
                   *const libc::c_char);
}
unsafe extern "C" fn Touch_SetFlags_f() {
    if Cmd_Argc() == 3 as libc::c_int {
        let mut privileged: qboolean = Cmd_CurrentCommandIsPrivileged();
        let mut button: *mut touch_button_t =
            Touch_FindButton(&mut touch.list_user, Cmd_Argv(1 as libc::c_int),
                             privileged);
        if !button.is_null() {
            (*button).flags =
                ((if privileged as libc::c_uint != 0 {
                      0 as libc::c_int as libc::c_uint
                  } else {
                      ((1 as libc::c_uint) << 10 as libc::c_int) |
                          (1 as libc::c_uint) << 2 as libc::c_int
                  }) | Q_atoi(Cmd_Argv(2 as libc::c_int)) as libc::c_uint) as
                    libc::c_int
        }
        return
    }
    Con_Printf(b"Usage: touch_setflags <name> <file>\n\x00" as *const u8 as
                   *const libc::c_char);
}
unsafe extern "C" fn Touch_SetCommand_f() {
    if Cmd_Argc() == 3 as libc::c_int {
        let mut button: *mut touch_button_t =
            Touch_FindButton(&mut touch.list_user, Cmd_Argv(1 as libc::c_int),
                             Cmd_CurrentCommandIsPrivileged());
        if button.is_null() {
            Con_Printf(b"^1Error:^7 no such button\x00" as *const u8 as
                           *const libc::c_char);
        } else { Touch_SetCommand(button, Cmd_Argv(2 as libc::c_int)); }
        return
    }
    Con_Printf(b"Usage: touch_setcommand <name> <command>\n\x00" as *const u8
                   as *const libc::c_char);
}
unsafe extern "C" fn Touch_ReloadConfig_f() {
    touch.state = state_none;
    if !touch.edit.is_null() { (*touch.edit).finger = -(1 as libc::c_int) }
    if !touch.selection.is_null() {
        (*touch.selection).finger = -(1 as libc::c_int)
    }
    touch.selection = 0 as *mut touch_button_t;
    touch.edit = touch.selection;
    touch.wheel_finger = -(1 as libc::c_int);
    touch.look_finger = touch.wheel_finger;
    touch.move_finger = touch.look_finger;
    touch.resize_finger = touch.move_finger;
    Cbuf_AddText(va(b"exec %s\n\x00" as *const u8 as *const libc::c_char,
                    (*touch_config_file).string));
}
unsafe extern "C" fn Touch_AddButton(mut list: *mut touchbuttonlist_t,
                                     mut name: *const libc::c_char,
                                     mut texture: *const libc::c_char,
                                     mut command: *const libc::c_char,
                                     mut x1: libc::c_float,
                                     mut y1: libc::c_float,
                                     mut x2: libc::c_float,
                                     mut y2: libc::c_float,
                                     mut color: *mut byte,
                                     mut privileged: qboolean)
 -> *mut touch_button_t {
    let mut button: *mut touch_button_t =
        _Mem_Alloc(touch.mempool,
                   ::std::mem::size_of::<touch_button_t>() as libc::c_ulong,
                   true_0,
                   b"../engine/client/in_touch.c\x00" as *const u8 as
                       *const libc::c_char, 782 as libc::c_int) as
            *mut touch_button_t;
    (*button).texture = -(1 as libc::c_int);
    Q_strncpy((*button).texturefile.as_mut_ptr(), texture,
              ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong);
    Q_strncpy((*button).name.as_mut_ptr(), name,
              ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong);
    Touch_RemoveButtonFromList(list, name, privileged);
    (*button).x1 = x1;
    (*button).y1 = y1;
    (*button).x2 = x2;
    (*button).y2 = y2;
    (*button).flags =
        if privileged as libc::c_uint != 0 {
            0 as libc::c_int as libc::c_uint
        } else {
            ((1 as libc::c_uint) << 10 as libc::c_int) |
                (1 as libc::c_uint) << 2 as libc::c_int
        } as libc::c_int;
    (*button).color[0 as libc::c_int as usize] =
        *color.offset(0 as libc::c_int as isize);
    (*button).color[1 as libc::c_int as usize] =
        *color.offset(1 as libc::c_int as isize);
    (*button).color[2 as libc::c_int as usize] =
        *color.offset(2 as libc::c_int as isize);
    (*button).color[3 as libc::c_int as usize] =
        *color.offset(3 as libc::c_int as isize);
    (*button).command[0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_char;
    (*button).flags = 0 as libc::c_int;
    (*button).fade = 1 as libc::c_int as libc::c_float;
    Touch_SetCommand(button, command);
    (*button).finger = -(1 as libc::c_int);
    (*button).next = 0 as *mut touch_button_s;
    (*button).prev = (*list).last;
    if !(*list).last.is_null() { (*(*list).last).next = button }
    (*list).last = button;
    if (*list).first.is_null() { (*list).first = button }
    return button;
}
#[no_mangle]
pub unsafe extern "C" fn Touch_AddClientButton(mut name: *const libc::c_char,
                                               mut texture:
                                                   *const libc::c_char,
                                               mut command:
                                                   *const libc::c_char,
                                               mut x1: libc::c_float,
                                               mut y1: libc::c_float,
                                               mut x2: libc::c_float,
                                               mut y2: libc::c_float,
                                               mut color: *mut byte,
                                               mut round: libc::c_int,
                                               mut aspect: libc::c_float,
                                               mut flags: libc::c_int) {
    let mut button: *mut touch_button_t = 0 as *mut touch_button_t;
    if touch.initialized as u64 == 0 { return }
    if round != 0 { IN_TouchCheckCoords(&mut x1, &mut y1, &mut x2, &mut y2); }
    if round == round_aspect as libc::c_int {
        y2 =
            y1 +
                (x2 - x1) *
                    (refState.width as libc::c_float /
                         refState.height as libc::c_float) * aspect
    }
    button =
        Touch_AddButton(&mut touch.list_user, name, texture, command, x1, y1,
                        x2, y2, color, true_0);
    (*button).flags =
        ((*button).flags as libc::c_uint |
             (flags as libc::c_uint | (1 as libc::c_uint) << 2 as libc::c_int
                  | (1 as libc::c_uint) << 1 as libc::c_int)) as libc::c_int;
    (*button).aspect = aspect;
}
unsafe extern "C" fn Touch_LoadDefaults_f() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < g_LastDefaultButton {
        let mut button: *mut touch_button_t = 0 as *mut touch_button_t;
        let mut x1: libc::c_float = g_DefaultButtons[i as usize].x1;
        let mut y1: libc::c_float = g_DefaultButtons[i as usize].y1;
        let mut x2: libc::c_float = g_DefaultButtons[i as usize].x2;
        let mut y2: libc::c_float = g_DefaultButtons[i as usize].y2;
        IN_TouchCheckCoords(&mut x1, &mut y1, &mut x2, &mut y2);
        if g_DefaultButtons[i as usize].aspect != 0. &&
               g_DefaultButtons[i as usize].round as libc::c_uint ==
                   round_aspect as libc::c_int as libc::c_uint {
            if g_DefaultButtons[i as
                                    usize].texturefile[0 as libc::c_int as
                                                           usize] as
                   libc::c_int == '#' as i32 {
                y2 =
                    y1 +
                        clgame.scrInfo.iCharHeight as libc::c_float /
                            clgame.scrInfo.iHeight as libc::c_float *
                            g_DefaultButtons[i as usize].aspect +
                        (touch.swidth * 2 as libc::c_int) as libc::c_float /
                            refState.height as libc::c_float
            } else {
                y2 =
                    y1 +
                        (x2 - x1) *
                            (refState.width as libc::c_float /
                                 refState.height as libc::c_float) *
                            g_DefaultButtons[i as usize].aspect
            }
        }
        IN_TouchCheckCoords(&mut x1, &mut y1, &mut x2, &mut y2);
        button =
            Touch_AddButton(&mut touch.list_user,
                            g_DefaultButtons[i as usize].name.as_mut_ptr(),
                            g_DefaultButtons[i as
                                                 usize].texturefile.as_mut_ptr(),
                            g_DefaultButtons[i as usize].command.as_mut_ptr(),
                            x1, y1, x2, y2,
                            g_DefaultButtons[i as usize].color.as_mut_ptr(),
                            true_0);
        (*button).flags |= g_DefaultButtons[i as usize].flags;
        (*button).aspect = g_DefaultButtons[i as usize].aspect;
        i += 1
    };
}
// Add default button from client
#[no_mangle]
pub unsafe extern "C" fn Touch_AddDefaultButton(mut name: *const libc::c_char,
                                                mut texturefile:
                                                    *const libc::c_char,
                                                mut command:
                                                    *const libc::c_char,
                                                mut x1: libc::c_float,
                                                mut y1: libc::c_float,
                                                mut x2: libc::c_float,
                                                mut y2: libc::c_float,
                                                mut color: *mut byte,
                                                mut round: libc::c_int,
                                                mut aspect: libc::c_float,
                                                mut flags: libc::c_int) {
    let mut button: *mut touchdefaultbutton_t =
        0 as *mut touchdefaultbutton_t;
    if g_LastDefaultButton >= 255 as libc::c_int { return }
    button =
        g_DefaultButtons.as_mut_ptr().offset(g_LastDefaultButton as isize);
    Q_strncpy((*button).name.as_mut_ptr(), name,
              ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong);
    Q_strncpy((*button).texturefile.as_mut_ptr(), texturefile,
              ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong);
    Q_strncpy((*button).command.as_mut_ptr(), command,
              ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong);
    (*button).x1 = x1;
    (*button).y1 = y1;
    (*button).x2 = x2;
    (*button).y2 = y2;
    (*button).color[0 as libc::c_int as usize] =
        *color.offset(0 as libc::c_int as isize);
    (*button).color[1 as libc::c_int as usize] =
        *color.offset(1 as libc::c_int as isize);
    (*button).color[2 as libc::c_int as usize] =
        *color.offset(2 as libc::c_int as isize);
    (*button).color[3 as libc::c_int as usize] =
        *color.offset(3 as libc::c_int as isize);
    (*button).round = round as touchRound;
    (*button).aspect = aspect;
    (*button).flags = flags;
    g_LastDefaultButton += 1;
}
// Client may remove all default buttons from engine
#[no_mangle]
pub unsafe extern "C" fn Touch_ResetDefaultButtons() {
    g_LastDefaultButton = 0 as libc::c_int;
}
unsafe extern "C" fn Touch_AddButton_f() {
    let mut color: rgba_t = [0; 4];
    let mut argc: libc::c_int = Cmd_Argc();
    let mut button: *mut touch_button_t = 0 as *mut touch_button_t;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut texture: *const libc::c_char = 0 as *const libc::c_char;
    let mut command: *const libc::c_char = 0 as *const libc::c_char;
    let mut x1: libc::c_float = 0.;
    let mut y1: libc::c_float = 0.;
    let mut x2: libc::c_float = 0.;
    let mut y2: libc::c_float = 0.;
    let mut privileged: qboolean = Cmd_CurrentCommandIsPrivileged();
    if argc < 4 as libc::c_int {
        Con_Printf(b"Usage: touch_addbutton <name> <texture> <command> [<x1> <y1> <x2> <y2> [ r g b a ] ]\n\x00"
                       as *const u8 as *const libc::c_char);
        return
    }
    name = Cmd_Argv(1 as libc::c_int);
    texture = Cmd_Argv(2 as libc::c_int);
    command = Cmd_Argv(3 as libc::c_int);
    if argc < 8 as libc::c_int {
        y1 = 0.4f32;
        x1 = y1;
        y2 = 0.6f32;
        x2 = y2
    } else {
        x1 = Q_atof(Cmd_Argv(4 as libc::c_int));
        y1 = Q_atof(Cmd_Argv(5 as libc::c_int));
        x2 = Q_atof(Cmd_Argv(6 as libc::c_int));
        y2 = Q_atof(Cmd_Argv(7 as libc::c_int))
    }
    if argc < 12 as libc::c_int {
        color[0 as libc::c_int as usize] = 255 as libc::c_int as byte;
        color[1 as libc::c_int as usize] = 255 as libc::c_int as byte;
        color[2 as libc::c_int as usize] = 255 as libc::c_int as byte;
        color[3 as libc::c_int as usize] = 255 as libc::c_int as byte
    } else {
        color[0 as libc::c_int as usize] =
            Q_atoi(Cmd_Argv(8 as libc::c_int)) as byte;
        color[1 as libc::c_int as usize] =
            Q_atoi(Cmd_Argv(9 as libc::c_int)) as byte;
        color[2 as libc::c_int as usize] =
            Q_atoi(Cmd_Argv(10 as libc::c_int)) as byte;
        color[3 as libc::c_int as usize] =
            Q_atoi(Cmd_Argv(11 as libc::c_int)) as byte
    }
    button =
        Touch_AddButton(&mut touch.list_user, name, texture, command, x1, y1,
                        x2, y2, color.as_mut_ptr(), privileged);
    if argc >= 13 as libc::c_int {
        (*button).flags =
            (*button).flags | Q_atoi(Cmd_Argv(12 as libc::c_int))
    }
    if argc >= 14 as libc::c_int {
        // Recalculate button coordinates aspect ratio
		// This is feature for distributed configs
        let mut aspect: libc::c_float = Q_atof(Cmd_Argv(13 as libc::c_int));
        if aspect != 0. {
            if (*button).texturefile[0 as libc::c_int as usize] as libc::c_int
                   != '#' as i32 {
                (*button).y2 =
                    (*button).y1 +
                        ((*button).x2 - (*button).x1) *
                            (refState.width as libc::c_float /
                                 refState.height as libc::c_float) * aspect
            }
            (*button).aspect = aspect
        }
    };
}
unsafe extern "C" fn Touch_EnableEdit_f() {
    if touch.state as libc::c_uint ==
           state_none as libc::c_int as libc::c_uint {
        touch.state = state_edit
    }
    touch.wheel_finger = -(1 as libc::c_int);
    touch.look_finger = touch.wheel_finger;
    touch.move_finger = touch.look_finger;
    touch.resize_finger = touch.move_finger;
    touch.move_button = 0 as *mut touch_button_t;
    touch.configchanged = true_0;
}
unsafe extern "C" fn Touch_DisableEdit_f() {
    touch.state = state_none;
    if !touch.edit.is_null() { (*touch.edit).finger = -(1 as libc::c_int) }
    if !touch.selection.is_null() {
        (*touch.selection).finger = -(1 as libc::c_int)
    }
    touch.selection = 0 as *mut touch_button_t;
    touch.edit = touch.selection;
    touch.wheel_finger = -(1 as libc::c_int);
    touch.look_finger = touch.wheel_finger;
    touch.move_finger = touch.look_finger;
    touch.resize_finger = touch.move_finger;
    if if !touch_in_menu.is_null() && (*touch_in_menu).value != 0.0f32 {
           true_0 as libc::c_int
       } else { false_0 as libc::c_int } != 0 {
        Cvar_Set(b"touch_in_menu\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char);
    } else if cls.key_dest as libc::c_uint ==
                  key_game as libc::c_int as libc::c_uint {
        Touch_WriteConfig();
    };
}
unsafe extern "C" fn Touch_DeleteProfile_f() {
    if Cmd_Argc() != 2 as libc::c_int {
        Con_Printf(b"Usage: touch_deleteprofile <name>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    // delete profile
    FS_Delete(va(b"touch_profiles/%s.cfg\x00" as *const u8 as
                     *const libc::c_char, Cmd_Argv(1 as libc::c_int)));
}
unsafe extern "C" fn Touch_InitEditor() {
    let mut x: libc::c_float =
        0.1f32 *
            (refState.height as libc::c_float /
                 refState.width as libc::c_float);
    let mut y: libc::c_float = 0.05f32;
    let mut temp: *mut touch_button_t = 0 as *mut touch_button_t;
    let mut color: rgba_t = [0; 4];
    color[0 as libc::c_int as usize] = 255 as libc::c_int as byte;
    color[1 as libc::c_int as usize] = 255 as libc::c_int as byte;
    color[2 as libc::c_int as usize] = 255 as libc::c_int as byte;
    color[3 as libc::c_int as usize] = 255 as libc::c_int as byte;
    Touch_ClearList(&mut touch.list_edit);
    temp =
        Touch_AddButton(&mut touch.list_edit,
                        b"close\x00" as *const u8 as *const libc::c_char,
                        b"touch_default/edit_close.tga\x00" as *const u8 as
                            *const libc::c_char,
                        b"touch_disableedit\x00" as *const u8 as
                            *const libc::c_char,
                        0 as libc::c_int as libc::c_float, y, x, y + 0.1f32,
                        color.as_mut_ptr(), true_0);
    (*temp).flags =
        ((*temp).flags as libc::c_uint |
             (1 as libc::c_uint) << 1 as libc::c_int) as libc::c_int;
    temp =
        Touch_AddButton(&mut touch.list_edit,
                        b"close\x00" as *const u8 as *const libc::c_char,
                        b"#Close and save\x00" as *const u8 as
                            *const libc::c_char,
                        b"\x00" as *const u8 as *const libc::c_char, x, y,
                        x + 0.2f32, y + 0.1f32, color.as_mut_ptr(), true_0);
    (*temp).flags =
        ((*temp).flags as libc::c_uint |
             (1 as libc::c_uint) << 1 as libc::c_int) as libc::c_int;
    y += 0.2f32;
    temp =
        Touch_AddButton(&mut touch.list_edit,
                        b"cancel\x00" as *const u8 as *const libc::c_char,
                        b"touch_default/edit_reset.tga\x00" as *const u8 as
                            *const libc::c_char,
                        b"touch_reloadconfig\x00" as *const u8 as
                            *const libc::c_char,
                        0 as libc::c_int as libc::c_float, y, x, y + 0.1f32,
                        color.as_mut_ptr(), true_0);
    (*temp).flags =
        ((*temp).flags as libc::c_uint |
             (1 as libc::c_uint) << 1 as libc::c_int) as libc::c_int;
    temp =
        Touch_AddButton(&mut touch.list_edit,
                        b"close\x00" as *const u8 as *const libc::c_char,
                        b"#Cancel and reset\x00" as *const u8 as
                            *const libc::c_char,
                        b"\x00" as *const u8 as *const libc::c_char, x, y,
                        x + 0.2f32, y + 0.1f32, color.as_mut_ptr(), true_0);
    (*temp).flags =
        ((*temp).flags as libc::c_uint |
             (1 as libc::c_uint) << 1 as libc::c_int) as libc::c_int;
    y += 0.2f32;
    touch.hidebutton =
        Touch_AddButton(&mut touch.list_edit,
                        b"showhide\x00" as *const u8 as *const libc::c_char,
                        b"touch_default/edit_hide.tga\x00" as *const u8 as
                            *const libc::c_char,
                        b"touch_toggleselection\x00" as *const u8 as
                            *const libc::c_char,
                        0 as libc::c_int as libc::c_float, y, x, y + 0.1f32,
                        color.as_mut_ptr(), true_0);
    (*touch.hidebutton).flags =
        ((*touch.hidebutton).flags as libc::c_uint |
             ((1 as libc::c_uint) << 0 as libc::c_int |
                  (1 as libc::c_uint) << 1 as libc::c_int)) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Touch_Init() {
    let mut color: rgba_t = [0; 4];
    if touch.initialized as u64 != 0 { return }
    touch.mempool =
        _Mem_AllocPool(b"Touch\x00" as *const u8 as *const libc::c_char,
                       b"../engine/client/in_touch.c\x00" as *const u8 as
                           *const libc::c_char, 1029 as libc::c_int);
    //touch.first = touch.last = NULL;
    Con_Printf(b"IN_TouchInit()\n\x00" as *const u8 as *const libc::c_char);
    touch.wheel_finger = -(1 as libc::c_int);
    touch.look_finger = touch.wheel_finger;
    touch.resize_finger = touch.look_finger;
    touch.move_finger = touch.resize_finger;
    touch.state = state_none;
    touch.showeditbuttons = true_0;
    touch.clientonly = false_0;
    touch.precision = false_0;
    touch.scolor[0 as libc::c_int as usize] = 255 as libc::c_int as byte;
    touch.scolor[1 as libc::c_int as usize] = 255 as libc::c_int as byte;
    touch.scolor[2 as libc::c_int as usize] = 255 as libc::c_int as byte;
    touch.scolor[3 as libc::c_int as usize] = 255 as libc::c_int as byte;
    touch.swidth = 1 as libc::c_int;
    g_LastDefaultButton = 0 as libc::c_int;
    touch.list_edit.last = 0 as *mut touch_button_t;
    touch.list_edit.first = touch.list_edit.last;
    touch.list_user.last = 0 as *mut touch_button_t;
    touch.list_user.first = touch.list_user.last;
    // fill default buttons list
    color[0 as libc::c_int as usize] = 255 as libc::c_int as byte;
    color[1 as libc::c_int as usize] = 255 as libc::c_int as byte;
    color[2 as libc::c_int as usize] = 255 as libc::c_int as byte;
    color[3 as libc::c_int as usize] = 255 as libc::c_int as byte;
    Touch_AddDefaultButton(b"look\x00" as *const u8 as *const libc::c_char,
                           b"\x00" as *const u8 as *const libc::c_char,
                           b"_look\x00" as *const u8 as *const libc::c_char,
                           0.500000f64 as libc::c_float,
                           0.000000f64 as libc::c_float,
                           1.000000f64 as libc::c_float,
                           1 as libc::c_int as libc::c_float,
                           color.as_mut_ptr(), 0 as libc::c_int,
                           0 as libc::c_int as libc::c_float,
                           0 as libc::c_int);
    Touch_AddDefaultButton(b"move\x00" as *const u8 as *const libc::c_char,
                           b"\x00" as *const u8 as *const libc::c_char,
                           b"_move\x00" as *const u8 as *const libc::c_char,
                           0.000000f64 as libc::c_float,
                           0.000000f64 as libc::c_float,
                           0.500000f64 as libc::c_float,
                           1 as libc::c_int as libc::c_float,
                           color.as_mut_ptr(), 0 as libc::c_int,
                           0 as libc::c_int as libc::c_float,
                           0 as libc::c_int);
    Touch_AddDefaultButton(b"invnext\x00" as *const u8 as *const libc::c_char,
                           b"touch_default/next_weap.tga\x00" as *const u8 as
                               *const libc::c_char,
                           b"invnext\x00" as *const u8 as *const libc::c_char,
                           0.000000f64 as libc::c_float,
                           0.530200f64 as libc::c_float,
                           0.120000f64 as libc::c_float,
                           0.757428f64 as libc::c_float, color.as_mut_ptr(),
                           2 as libc::c_int,
                           1 as libc::c_int as libc::c_float,
                           0 as libc::c_int);
    Touch_AddDefaultButton(b"invprev\x00" as *const u8 as *const libc::c_char,
                           b"touch_default/prev_weap.tga\x00" as *const u8 as
                               *const libc::c_char,
                           b"invprev\x00" as *const u8 as *const libc::c_char,
                           0.000000f64 as libc::c_float,
                           0.075743f64 as libc::c_float,
                           0.120000f64 as libc::c_float,
                           0.302971f64 as libc::c_float, color.as_mut_ptr(),
                           2 as libc::c_int,
                           1 as libc::c_int as libc::c_float,
                           0 as libc::c_int);
    Touch_AddDefaultButton(b"use\x00" as *const u8 as *const libc::c_char,
                           b"touch_default/use.tga\x00" as *const u8 as
                               *const libc::c_char,
                           b"+use\x00" as *const u8 as *const libc::c_char,
                           0.880000f64 as libc::c_float,
                           0.454457f64 as libc::c_float,
                           1.000000f64 as libc::c_float,
                           0.681685f64 as libc::c_float, color.as_mut_ptr(),
                           2 as libc::c_int,
                           1 as libc::c_int as libc::c_float,
                           0 as libc::c_int);
    Touch_AddDefaultButton(b"jump\x00" as *const u8 as *const libc::c_char,
                           b"touch_default/jump.tga\x00" as *const u8 as
                               *const libc::c_char,
                           b"+jump\x00" as *const u8 as *const libc::c_char,
                           0.880000f64 as libc::c_float,
                           0.227228f64 as libc::c_float,
                           1.000000f64 as libc::c_float,
                           0.454457f64 as libc::c_float, color.as_mut_ptr(),
                           2 as libc::c_int,
                           1 as libc::c_int as libc::c_float,
                           0 as libc::c_int);
    Touch_AddDefaultButton(b"attack\x00" as *const u8 as *const libc::c_char,
                           b"touch_default/shoot.tga\x00" as *const u8 as
                               *const libc::c_char,
                           b"+attack\x00" as *const u8 as *const libc::c_char,
                           0.760000f64 as libc::c_float,
                           0.530200f64 as libc::c_float,
                           0.880000f64 as libc::c_float,
                           0.757428f64 as libc::c_float, color.as_mut_ptr(),
                           2 as libc::c_int,
                           1 as libc::c_int as libc::c_float,
                           0 as libc::c_int);
    Touch_AddDefaultButton(b"attack2\x00" as *const u8 as *const libc::c_char,
                           b"touch_default/shoot_alt.tga\x00" as *const u8 as
                               *const libc::c_char,
                           b"+attack2\x00" as *const u8 as
                               *const libc::c_char,
                           0.760000f64 as libc::c_float,
                           0.302971f64 as libc::c_float,
                           0.880000f64 as libc::c_float,
                           0.530200f64 as libc::c_float, color.as_mut_ptr(),
                           2 as libc::c_int,
                           1 as libc::c_int as libc::c_float,
                           0 as libc::c_int);
    Touch_AddDefaultButton(b"loadquick\x00" as *const u8 as
                               *const libc::c_char,
                           b"touch_default/load.tga\x00" as *const u8 as
                               *const libc::c_char,
                           b"loadquick\x00" as *const u8 as
                               *const libc::c_char,
                           0.760000f64 as libc::c_float,
                           0.000000f64 as libc::c_float,
                           0.840000f64 as libc::c_float,
                           0.151486f64 as libc::c_float, color.as_mut_ptr(),
                           2 as libc::c_int,
                           1 as libc::c_int as libc::c_float,
                           16 as libc::c_int);
    Touch_AddDefaultButton(b"savequick\x00" as *const u8 as
                               *const libc::c_char,
                           b"touch_default/save.tga\x00" as *const u8 as
                               *const libc::c_char,
                           b"savequick\x00" as *const u8 as
                               *const libc::c_char,
                           0.840000f64 as libc::c_float,
                           0.000000f64 as libc::c_float,
                           0.920000f64 as libc::c_float,
                           0.151486f64 as libc::c_float, color.as_mut_ptr(),
                           2 as libc::c_int,
                           1 as libc::c_int as libc::c_float,
                           16 as libc::c_int);
    Touch_AddDefaultButton(b"messagemode\x00" as *const u8 as
                               *const libc::c_char,
                           b"touch_default/keyboard.tga\x00" as *const u8 as
                               *const libc::c_char,
                           b"messagemode\x00" as *const u8 as
                               *const libc::c_char,
                           0.840000f64 as libc::c_float,
                           0.000000f64 as libc::c_float,
                           0.920000f64 as libc::c_float,
                           0.151486f64 as libc::c_float, color.as_mut_ptr(),
                           2 as libc::c_int,
                           1 as libc::c_int as libc::c_float,
                           8 as libc::c_int);
    Touch_AddDefaultButton(b"reload\x00" as *const u8 as *const libc::c_char,
                           b"touch_default/reload.tga\x00" as *const u8 as
                               *const libc::c_char,
                           b"+reload\x00" as *const u8 as *const libc::c_char,
                           0.000000f64 as libc::c_float,
                           0.302971f64 as libc::c_float,
                           0.120000f64 as libc::c_float,
                           0.530200f64 as libc::c_float, color.as_mut_ptr(),
                           2 as libc::c_int,
                           1 as libc::c_int as libc::c_float,
                           0 as libc::c_int);
    Touch_AddDefaultButton(b"flashlight\x00" as *const u8 as
                               *const libc::c_char,
                           b"touch_default/flash_light_filled.tga\x00" as
                               *const u8 as *const libc::c_char,
                           b"impulse 100\x00" as *const u8 as
                               *const libc::c_char,
                           0.920000f64 as libc::c_float,
                           0.000000f64 as libc::c_float,
                           1.000000f64 as libc::c_float,
                           0.151486f64 as libc::c_float, color.as_mut_ptr(),
                           2 as libc::c_int,
                           1 as libc::c_int as libc::c_float,
                           0 as libc::c_int);
    Touch_AddDefaultButton(b"scores\x00" as *const u8 as *const libc::c_char,
                           b"touch_default/map.tga\x00" as *const u8 as
                               *const libc::c_char,
                           b"+showscores\x00" as *const u8 as
                               *const libc::c_char,
                           0.760000f64 as libc::c_float,
                           0.000000f64 as libc::c_float,
                           0.840000f64 as libc::c_float,
                           0.151486f64 as libc::c_float, color.as_mut_ptr(),
                           2 as libc::c_int,
                           1 as libc::c_int as libc::c_float,
                           8 as libc::c_int);
    Touch_AddDefaultButton(b"show_numbers\x00" as *const u8 as
                               *const libc::c_char,
                           b"touch_default/show_weapons.tga\x00" as *const u8
                               as *const libc::c_char,
                           b"exec touch_default/numbers.cfg\x00" as *const u8
                               as *const libc::c_char,
                           0.440000f64 as libc::c_float,
                           0.833171f64 as libc::c_float,
                           0.520000f64 as libc::c_float,
                           0.984656f64 as libc::c_float, color.as_mut_ptr(),
                           2 as libc::c_int,
                           1 as libc::c_int as libc::c_float,
                           0 as libc::c_int);
    Touch_AddDefaultButton(b"duck\x00" as *const u8 as *const libc::c_char,
                           b"touch_default/crouch.tga\x00" as *const u8 as
                               *const libc::c_char,
                           b"+duck\x00" as *const u8 as *const libc::c_char,
                           0.880000f64 as libc::c_float,
                           0.757428f64 as libc::c_float,
                           1.000000f64 as libc::c_float,
                           0.984656f64 as libc::c_float, color.as_mut_ptr(),
                           2 as libc::c_int,
                           1 as libc::c_int as libc::c_float,
                           0 as libc::c_int);
    Touch_AddDefaultButton(b"tduck\x00" as *const u8 as *const libc::c_char,
                           b"touch_default/tduck.tga\x00" as *const u8 as
                               *const libc::c_char,
                           b";+duck\x00" as *const u8 as *const libc::c_char,
                           0.560000f64 as libc::c_float,
                           0.833171f64 as libc::c_float,
                           0.620000f64 as libc::c_float,
                           0.946785f64 as libc::c_float, color.as_mut_ptr(),
                           2 as libc::c_int,
                           1 as libc::c_int as libc::c_float,
                           0 as libc::c_int);
    Touch_AddDefaultButton(b"edit\x00" as *const u8 as *const libc::c_char,
                           b"touch_default/settings.tga\x00" as *const u8 as
                               *const libc::c_char,
                           b"touch_enableedit\x00" as *const u8 as
                               *const libc::c_char,
                           0.420000f64 as libc::c_float,
                           0.000000f64 as libc::c_float,
                           0.500000f64 as libc::c_float,
                           0.151486f64 as libc::c_float, color.as_mut_ptr(),
                           2 as libc::c_int,
                           1 as libc::c_int as libc::c_float,
                           32 as libc::c_int);
    Touch_AddDefaultButton(b"menu\x00" as *const u8 as *const libc::c_char,
                           b"touch_default/menu.tga\x00" as *const u8 as
                               *const libc::c_char,
                           b"escape\x00" as *const u8 as *const libc::c_char,
                           0.000000f64 as libc::c_float,
                           0.833171f64 as libc::c_float,
                           0.080000f64 as libc::c_float,
                           0.984656f64 as libc::c_float, color.as_mut_ptr(),
                           2 as libc::c_int,
                           1 as libc::c_int as libc::c_float,
                           0 as libc::c_int);
    Cmd_AddCommand(b"touch_addbutton\x00" as *const u8 as *const libc::c_char,
                   Some(Touch_AddButton_f as unsafe extern "C" fn() -> ()),
                   b"add native touch button\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"touch_removebutton\x00" as *const u8 as
                       *const libc::c_char,
                   Some(IN_TouchRemoveButton_f as
                            unsafe extern "C" fn() -> ()),
                   b"remove native touch button\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddRestrictedCommand(b"touch_enableedit\x00" as *const u8 as
                                 *const libc::c_char,
                             Some(Touch_EnableEdit_f as
                                      unsafe extern "C" fn() -> ()),
                             b"enable button editing mode\x00" as *const u8 as
                                 *const libc::c_char);
    Cmd_AddRestrictedCommand(b"touch_disableedit\x00" as *const u8 as
                                 *const libc::c_char,
                             Some(Touch_DisableEdit_f as
                                      unsafe extern "C" fn() -> ()),
                             b"disable button editing mode\x00" as *const u8
                                 as *const libc::c_char);
    Cmd_AddCommand(b"touch_settexture\x00" as *const u8 as
                       *const libc::c_char,
                   Some(Touch_SetTexture_f as unsafe extern "C" fn() -> ()),
                   b"change button texture\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"touch_setcolor\x00" as *const u8 as *const libc::c_char,
                   Some(Touch_SetColor_f as unsafe extern "C" fn() -> ()),
                   b"change button color\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"touch_setcommand\x00" as *const u8 as
                       *const libc::c_char,
                   Some(Touch_SetCommand_f as unsafe extern "C" fn() -> ()),
                   b"change button command\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"touch_setflags\x00" as *const u8 as *const libc::c_char,
                   Some(Touch_SetFlags_f as unsafe extern "C" fn() -> ()),
                   b"change button flags (be careful)\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"touch_show\x00" as *const u8 as *const libc::c_char,
                   Some(Touch_Show_f as unsafe extern "C" fn() -> ()),
                   b"show button\x00" as *const u8 as *const libc::c_char);
    Cmd_AddCommand(b"touch_hide\x00" as *const u8 as *const libc::c_char,
                   Some(Touch_Hide_f as unsafe extern "C" fn() -> ()),
                   b"hide button\x00" as *const u8 as *const libc::c_char);
    Cmd_AddRestrictedCommand(b"touch_list\x00" as *const u8 as
                                 *const libc::c_char,
                             Some(Touch_ListButtons_f as
                                      unsafe extern "C" fn() -> ()),
                             b"list buttons\x00" as *const u8 as
                                 *const libc::c_char);
    Cmd_AddRestrictedCommand(b"touch_removeall\x00" as *const u8 as
                                 *const libc::c_char,
                             Some(Touch_RemoveAll_f as
                                      unsafe extern "C" fn() -> ()),
                             b"remove all buttons\x00" as *const u8 as
                                 *const libc::c_char);
    Cmd_AddRestrictedCommand(b"touch_loaddefaults\x00" as *const u8 as
                                 *const libc::c_char,
                             Some(Touch_LoadDefaults_f as
                                      unsafe extern "C" fn() -> ()),
                             b"generate config from defaults\x00" as *const u8
                                 as *const libc::c_char);
    Cmd_AddRestrictedCommand(b"touch_roundall\x00" as *const u8 as
                                 *const libc::c_char,
                             Some(Touch_RoundAll_f as
                                      unsafe extern "C" fn() -> ()),
                             b"round all buttons coordinates to grid\x00" as
                                 *const u8 as *const libc::c_char);
    Cmd_AddRestrictedCommand(b"touch_exportconfig\x00" as *const u8 as
                                 *const libc::c_char,
                             Some(Touch_ExportConfig_f as
                                      unsafe extern "C" fn() -> ()),
                             b"export config keeping aspect ratio\x00" as
                                 *const u8 as *const libc::c_char);
    Cmd_AddCommand(b"touch_set_stroke\x00" as *const u8 as
                       *const libc::c_char,
                   Some(Touch_Stroke_f as unsafe extern "C" fn() -> ()),
                   b"set global stroke width and color\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"touch_setclientonly\x00" as *const u8 as
                       *const libc::c_char,
                   Some(Touch_SetClientOnly_f as
                            unsafe extern "C" fn() -> ()),
                   b"when 1, only client buttons are shown\x00" as *const u8
                       as *const libc::c_char);
    Cmd_AddRestrictedCommand(b"touch_reloadconfig\x00" as *const u8 as
                                 *const libc::c_char,
                             Some(Touch_ReloadConfig_f as
                                      unsafe extern "C" fn() -> ()),
                             b"load config, not saving changes\x00" as
                                 *const u8 as *const libc::c_char);
    Cmd_AddRestrictedCommand(b"touch_writeconfig\x00" as *const u8 as
                                 *const libc::c_char,
                             Some(Touch_WriteConfig as
                                      unsafe extern "C" fn() -> ()),
                             b"save current config\x00" as *const u8 as
                                 *const libc::c_char);
    Cmd_AddRestrictedCommand(b"touch_deleteprofile\x00" as *const u8 as
                                 *const libc::c_char,
                             Some(Touch_DeleteProfile_f as
                                      unsafe extern "C" fn() -> ()),
                             b"delete profile by name\x00" as *const u8 as
                                 *const libc::c_char);
    Cmd_AddRestrictedCommand(b"touch_generate_code\x00" as *const u8 as
                                 *const libc::c_char,
                             Some(Touch_GenerateCode_f as
                                      unsafe extern "C" fn() -> ()),
                             b"create code sample for mobility API\x00" as
                                 *const u8 as *const libc::c_char);
    Cmd_AddCommand(b"touch_fade\x00" as *const u8 as *const libc::c_char,
                   Some(Touch_Fade_f as unsafe extern "C" fn() -> ()),
                   b"start fade animation for selected buttons\x00" as
                       *const u8 as *const libc::c_char);
    Cmd_AddRestrictedCommand(b"touch_toggleselection\x00" as *const u8 as
                                 *const libc::c_char,
                             Some(Touch_ToggleSelection_f as
                                      unsafe extern "C" fn() -> ()),
                             b"toggle vidibility on selected button in editor\x00"
                                 as *const u8 as *const libc::c_char);
    // not saved, just runtime state for scripting
    touch_in_menu =
        Cvar_Get(b"touch_in_menu\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 11 as libc::c_int,
                 b"draw touch in menu (for internal use only)\x00" as
                     *const u8 as *const libc::c_char);
    // sensitivity configuration
    touch_forwardzone =
        Cvar_Get(b"touch_forwardzone\x00" as *const u8 as *const libc::c_char,
                 b"0.06\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 11 as libc::c_int,
                 b"forward touch zone\x00" as *const u8 as
                     *const libc::c_char);
    touch_sidezone =
        Cvar_Get(b"touch_sidezone\x00" as *const u8 as *const libc::c_char,
                 b"0.06\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 11 as libc::c_int,
                 b"side touch zone\x00" as *const u8 as *const libc::c_char);
    touch_pitch =
        Cvar_Get(b"touch_pitch\x00" as *const u8 as *const libc::c_char,
                 b"90\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 11 as libc::c_int,
                 b"touch pitch sensitivity\x00" as *const u8 as
                     *const libc::c_char);
    touch_yaw =
        Cvar_Get(b"touch_yaw\x00" as *const u8 as *const libc::c_char,
                 b"120\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 11 as libc::c_int,
                 b"touch yaw sensitivity\x00" as *const u8 as
                     *const libc::c_char);
    touch_nonlinear_look =
        Cvar_Get(b"touch_nonlinear_look\x00" as *const u8 as
                     *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 11 as libc::c_int,
                 b"enable nonlinear touch look\x00" as *const u8 as
                     *const libc::c_char);
    touch_pow_factor =
        Cvar_Get(b"touch_pow_factor\x00" as *const u8 as *const libc::c_char,
                 b"1.3\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 11 as libc::c_int,
                 b"set > 1 to enable\x00" as *const u8 as
                     *const libc::c_char);
    touch_pow_mult =
        Cvar_Get(b"touch_pow_mult\x00" as *const u8 as *const libc::c_char,
                 b"400.0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 11 as libc::c_int,
                 b"power multiplier, usually 200-1000\x00" as *const u8 as
                     *const libc::c_char);
    touch_exp_mult =
        Cvar_Get(b"touch_exp_mult\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 11 as libc::c_int,
                 b"exponent multiplier, usually 20-200, 0 to disable\x00" as
                     *const u8 as *const libc::c_char);
    // touch.cfg
    touch_grid_count =
        Cvar_Get(b"touch_grid_count\x00" as *const u8 as *const libc::c_char,
                 b"50\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 11 as libc::c_int,
                 b"touch grid count\x00" as *const u8 as *const libc::c_char);
    touch_grid_enable =
        Cvar_Get(b"touch_grid_enable\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 11 as libc::c_int,
                 b"enable touch grid\x00" as *const u8 as
                     *const libc::c_char);
    touch_config_file =
        Cvar_Get(b"touch_config_file\x00" as *const u8 as *const libc::c_char,
                 b"touch.cfg\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int |
                     (1 as libc::c_int) << 10 as libc::c_int,
                 b"current touch profile file\x00" as *const u8 as
                     *const libc::c_char);
    touch_precise_amount =
        Cvar_Get(b"touch_precise_amount\x00" as *const u8 as
                     *const libc::c_char,
                 b"0.5\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 11 as libc::c_int,
                 b"sensitivity multiplier for precise-look\x00" as *const u8
                     as *const libc::c_char);
    touch_highlight_r =
        Cvar_Get(b"touch_highlight_r\x00" as *const u8 as *const libc::c_char,
                 b"1.0\x00" as *const u8 as *const libc::c_char,
                 0 as libc::c_int,
                 b"highlight r color\x00" as *const u8 as
                     *const libc::c_char);
    touch_highlight_g =
        Cvar_Get(b"touch_highlight_g\x00" as *const u8 as *const libc::c_char,
                 b"1.0\x00" as *const u8 as *const libc::c_char,
                 0 as libc::c_int,
                 b"highlight g color\x00" as *const u8 as
                     *const libc::c_char);
    touch_highlight_b =
        Cvar_Get(b"touch_highlight_b\x00" as *const u8 as *const libc::c_char,
                 b"1.0\x00" as *const u8 as *const libc::c_char,
                 0 as libc::c_int,
                 b"highlight b color\x00" as *const u8 as
                     *const libc::c_char);
    touch_highlight_a =
        Cvar_Get(b"touch_highlight_a\x00" as *const u8 as *const libc::c_char,
                 b"1.0\x00" as *const u8 as *const libc::c_char,
                 0 as libc::c_int,
                 b"highlight alpha\x00" as *const u8 as *const libc::c_char);
    touch_dpad_radius =
        Cvar_Get(b"touch_dpad_radius\x00" as *const u8 as *const libc::c_char,
                 b"1.0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 11 as libc::c_int,
                 b"dpad radius multiplier\x00" as *const u8 as
                     *const libc::c_char);
    touch_joy_radius =
        Cvar_Get(b"touch_joy_radius\x00" as *const u8 as *const libc::c_char,
                 b"1.0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 11 as libc::c_int,
                 b"joy radius multiplier\x00" as *const u8 as
                     *const libc::c_char);
    touch_move_indicator =
        Cvar_Get(b"touch_move_indicator\x00" as *const u8 as
                     *const libc::c_char,
                 b"0.0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 11 as libc::c_int,
                 b"indicate move events (0 to disable)\x00" as *const u8 as
                     *const libc::c_char);
    touch_joy_texture =
        Cvar_Get(b"touch_joy_texture\x00" as *const u8 as *const libc::c_char,
                 b"touch_default/joy.tga\x00" as *const u8 as
                     *const libc::c_char,
                 (1 as libc::c_int) << 11 as libc::c_int,
                 b"texture for move indicator\x00" as *const u8 as
                     *const libc::c_char);
    // input devices cvar
    touch_enable =
        Cvar_Get(b"touch_enable\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int |
                     (1 as libc::c_int) << 11 as libc::c_int,
                 b"enable touch controls\x00" as *const u8 as
                     *const libc::c_char);
    touch_emulate =
        Cvar_Get(b"touch_emulate\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int |
                     (1 as libc::c_int) << 11 as libc::c_int,
                 b"emulate touch with mouse\x00" as *const u8 as
                     *const libc::c_char);
    // / TODO: touch sdl platform
	// SDL_SetHint( SDL_HINT_ANDROID_SEPARATE_MOUSE_AND_TOUCH, "1" );
    touch.initialized = true_0;
}
//int pfnGetScreenInfo( SCREENINFO *pscrinfo );
unsafe extern "C" fn Touch_InitConfig() {
    if touch.initialized as u64 == 0 { return }
    if host.config_executed as u64 == 0 { return }
    if touch.config_loaded as u64 != 0 { return }
    // / TODO: hud font
	//pfnGetScreenInfo( NULL ); //HACK: update hud screen parameters like iHeight
    if FS_FileExists((*touch_config_file).string, true_0 as libc::c_int) != 0
       {
        Cbuf_AddText(va(b"exec \"%s\"\n\x00" as *const u8 as
                            *const libc::c_char,
                        (*touch_config_file).string));
    } else { Touch_LoadDefaults_f(); }
    Touch_InitEditor();
    touch.joytexture =
        ref_0.dllFuncs.GL_LoadTexture.expect("non-null function pointer")((*touch_joy_texture).string,
                                                                          0 as
                                                                              *const byte,
                                                                          0 as
                                                                              libc::c_int
                                                                              as
                                                                              size_t,
                                                                          TF_NOMIPMAP
                                                                              as
                                                                              libc::c_int);
    touch.whitetexture =
        ref_0.dllFuncs.GL_LoadTexture.expect("non-null function pointer")(b"*white\x00"
                                                                              as
                                                                              *const u8
                                                                              as
                                                                              *const libc::c_char,
                                                                          0 as
                                                                              *const byte,
                                                                          0 as
                                                                              libc::c_int
                                                                              as
                                                                              size_t,
                                                                          0 as
                                                                              libc::c_int);
    touch.configchanged = false_0;
    touch.config_loaded = true_0;
}
/*
============================================================================

                     TOUCH CONTROLS RENDERING

============================================================================
*/
unsafe extern "C" fn Touch_IsVisible(mut button: *mut touch_button_t)
 -> qboolean {
    if (*button).flags as libc::c_uint &
           (1 as libc::c_uint) << 2 as libc::c_int == 0 &&
           touch.clientonly as libc::c_uint != 0 {
        return false_0
    } // skip nonclient buttons in clientonly mode
    if touch.state as libc::c_uint >=
           state_edit as libc::c_int as libc::c_uint {
        return true_0
    } // !!! Draw when editor is open
    if (*button).flags as libc::c_uint &
           (1 as libc::c_uint) << 0 as libc::c_int != 0 {
        return false_0
    } // skip hidden
    if (*button).flags as libc::c_uint &
           (1 as libc::c_uint) << 4 as libc::c_int != 0 &&
           CL_GetMaxClients() != 1 as libc::c_int {
        return false_0
    } // skip singleplayer(load, save) buttons in multiplayer
    if (*button).flags as libc::c_uint &
           (1 as libc::c_uint) << 3 as libc::c_int != 0 &&
           CL_GetMaxClients() == 1 as libc::c_int {
        return false_0
    } // skip multiplayer buttons in singleplayer
    return true_0;
}
unsafe extern "C" fn Touch_DrawTexture(mut x1: libc::c_float,
                                       mut y1: libc::c_float,
                                       mut x2: libc::c_float,
                                       mut y2: libc::c_float,
                                       mut texture: libc::c_int, mut r: byte,
                                       mut g: byte, mut b: byte,
                                       mut a: byte) {
    if x1 >= x2 { return }
    if y1 >= y2 { return }
    ref_0.dllFuncs.Color4ub.expect("non-null function pointer")(r, g, b, a);
    ref_0.dllFuncs.R_DrawStretchPic.expect("non-null function pointer")(refState.width
                                                                            as
                                                                            libc::c_float
                                                                            *
                                                                            x1,
                                                                        refState.height
                                                                            as
                                                                            libc::c_float
                                                                            *
                                                                            y1,
                                                                        refState.width
                                                                            as
                                                                            libc::c_float
                                                                            *
                                                                            (x2
                                                                                 -
                                                                                 x1),
                                                                        refState.height
                                                                            as
                                                                            libc::c_float
                                                                            *
                                                                            (y2
                                                                                 -
                                                                                 y1),
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
                                                                        texture);
}
unsafe extern "C" fn IN_TouchCheckCoords(mut x1: *mut libc::c_float,
                                         mut y1: *mut libc::c_float,
                                         mut x2: *mut libc::c_float,
                                         mut y2: *mut libc::c_float) {
    // / TODO: grid check here
    if *x2 - *x1 <
           1.0f32 / (*touch_grid_count).value as libc::c_int as libc::c_float
               * 2 as libc::c_int as libc::c_float {
        *x2 =
            *x1 +
                1.0f32 /
                    (*touch_grid_count).value as libc::c_int as libc::c_float
                    * 2 as libc::c_int as libc::c_float
    }
    if *y2 - *y1 <
           refState.width as libc::c_float / refState.height as libc::c_float
               / (*touch_grid_count).value as libc::c_int as libc::c_float *
               2 as libc::c_int as libc::c_float {
        *y2 =
            *y1 +
                refState.width as libc::c_float /
                    refState.height as libc::c_float /
                    (*touch_grid_count).value as libc::c_int as libc::c_float
                    * 2 as libc::c_int as libc::c_float
    }
    if *x1 < 0 as libc::c_int as libc::c_float {
        *x2 -= *x1;
        *x1 = 0 as libc::c_int as libc::c_float
    }
    if *y1 < 0 as libc::c_int as libc::c_float {
        *y2 -= *y1;
        *y1 = 0 as libc::c_int as libc::c_float
    }
    if *y2 > 1 as libc::c_int as libc::c_float {
        *y1 -= *y2 - 1 as libc::c_int as libc::c_float;
        *y2 = 1 as libc::c_int as libc::c_float
    }
    if *x2 > 1 as libc::c_int as libc::c_float {
        *x1 -= *x2 - 1 as libc::c_int as libc::c_float;
        *x2 = 1 as libc::c_int as libc::c_float
    }
    if if !touch_grid_enable.is_null() && (*touch_grid_enable).value != 0.0f32
          {
           true_0 as libc::c_int
       } else { false_0 as libc::c_int } != 0 {
        *x1 =
            __tg_round(*x1 *
                           (*touch_grid_count).value as libc::c_int as
                               libc::c_float) /
                (*touch_grid_count).value as libc::c_int as libc::c_float;
        *x2 =
            __tg_round(*x2 *
                           (*touch_grid_count).value as libc::c_int as
                               libc::c_float) /
                (*touch_grid_count).value as libc::c_int as libc::c_float;
        *y1 =
            __tg_round(*y1 *
                           ((*touch_grid_count).value as libc::c_int as
                                libc::c_float *
                                refState.height as libc::c_float /
                                refState.width as libc::c_float)) /
                ((*touch_grid_count).value as libc::c_int as libc::c_float *
                     refState.height as libc::c_float /
                     refState.width as libc::c_float);
        *y2 =
            __tg_round(*y2 *
                           ((*touch_grid_count).value as libc::c_int as
                                libc::c_float *
                                refState.height as libc::c_float /
                                refState.width as libc::c_float)) /
                ((*touch_grid_count).value as libc::c_int as libc::c_float *
                     refState.height as libc::c_float /
                     refState.width as libc::c_float)
    };
}
unsafe extern "C" fn Touch_DrawCharacter(mut x: libc::c_float,
                                         mut y: libc::c_float,
                                         mut number: libc::c_int,
                                         mut size: libc::c_float)
 -> libc::c_float {
    let mut s1: libc::c_float = 0.;
    let mut s2: libc::c_float = 0.;
    let mut t1: libc::c_float = 0.;
    let mut t2: libc::c_float = 0.;
    let mut width: libc::c_float = 0.;
    let mut height: libc::c_float = 0.;
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut prc: *mut wrect_t = 0 as *mut wrect_t;
    if cls.creditsFont.valid as u64 == 0 {
        return 0 as libc::c_int as libc::c_float
    }
    number &= 255 as libc::c_int;
    number = Con_UtfProcessChar(number);
    R_GetTextureParms(&mut w, &mut h, cls.creditsFont.hFontTexture);
    prc =
        &mut *cls.creditsFont.fontRc.as_mut_ptr().offset(number as isize) as
            *mut wrect_t;
    s1 = (*prc).left as libc::c_float / w as libc::c_float;
    t1 = (*prc).top as libc::c_float / h as libc::c_float;
    s2 = (*prc).right as libc::c_float / w as libc::c_float;
    t2 = (*prc).bottom as libc::c_float / h as libc::c_float;
    width = ((*prc).right - (*prc).left) as libc::c_float / 1024.0f32 * size;
    height = ((*prc).bottom - (*prc).top) as libc::c_float / 1024.0f32 * size;
    ref_0.dllFuncs.R_DrawStretchPic.expect("non-null function pointer")(refState.width
                                                                            as
                                                                            libc::c_float
                                                                            *
                                                                            x,
                                                                        refState.height
                                                                            as
                                                                            libc::c_float
                                                                            *
                                                                            y,
                                                                        refState.width
                                                                            as
                                                                            libc::c_float
                                                                            *
                                                                            width,
                                                                        refState.width
                                                                            as
                                                                            libc::c_float
                                                                            *
                                                                            height,
                                                                        s1,
                                                                        t1,
                                                                        s2,
                                                                        t2,
                                                                        cls.creditsFont.hFontTexture);
    return width;
}
unsafe extern "C" fn Touch_DrawText(mut x1: libc::c_float,
                                    mut y1: libc::c_float,
                                    mut x2: libc::c_float,
                                    mut y2: libc::c_float,
                                    mut s: *const libc::c_char,
                                    mut color: *mut byte,
                                    mut size: libc::c_float)
 -> libc::c_float {
    let mut x: libc::c_float = x1;
    let mut maxy: libc::c_float = y2;
    let mut maxx: libc::c_float = 0.;
    if x2 != 0. {
        maxx =
            x2 -
                cls.creditsFont.charWidths['M' as i32 as usize] as libc::c_int
                    as libc::c_float / 1024.0f32 * size
    } else { maxx = 1 as libc::c_int as libc::c_float }
    if cls.creditsFont.valid as u64 == 0 {
        return 1.0f32 /
                   (*touch_grid_count).value as libc::c_int as libc::c_float *
                   2 as libc::c_int as libc::c_float
    }
    Con_UtfProcessChar(0 as libc::c_int);
    ref_0.dllFuncs.GL_SetRenderMode.expect("non-null function pointer")(kRenderTransAdd
                                                                            as
                                                                            libc::c_int);
    // text is additive and alpha does not work
    ref_0.dllFuncs.Color4ub.expect("non-null function pointer")((*color.offset(0
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)
                                                                     as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_float
                                                                     *
                                                                     (*color.offset(3
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)
                                                                          as
                                                                          libc::c_float
                                                                          /
                                                                          255.0f32))
                                                                    as
                                                                    libc::c_uchar,
                                                                (*color.offset(1
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)
                                                                     as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_float
                                                                     *
                                                                     (*color.offset(3
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)
                                                                          as
                                                                          libc::c_float
                                                                          /
                                                                          255.0f32))
                                                                    as
                                                                    libc::c_uchar,
                                                                (*color.offset(2
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)
                                                                     as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_float
                                                                     *
                                                                     (*color.offset(3
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)
                                                                          as
                                                                          libc::c_float
                                                                          /
                                                                          255.0f32))
                                                                    as
                                                                    libc::c_uchar,
                                                                255 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uchar);
    while *s != 0 {
        while *s as libc::c_int != 0 && *s as libc::c_int != '\n' as i32 &&
                  *s as libc::c_int != ';' as i32 && x1 < maxx {
            let fresh0 = s;
            s = s.offset(1);
            x1 += Touch_DrawCharacter(x1, y1, *fresh0 as libc::c_int, size)
        }
        y1 +=
            cls.creditsFont.charHeight as libc::c_float / 1024.0f32 * size /
                refState.height as libc::c_float *
                refState.width as libc::c_float;
        if y1 >= maxy { break ; }
        if *s as libc::c_int == '\n' as i32 || *s as libc::c_int == ';' as i32
           {
            s = s.offset(1)
        }
        x1 = x
    }
    ref_0.dllFuncs.GL_SetRenderMode.expect("non-null function pointer")(kRenderTransTexture
                                                                            as
                                                                            libc::c_int);
    return x1;
}
unsafe extern "C" fn Touch_DrawButtons(mut list: *mut touchbuttonlist_t) {
    let mut button: *mut touch_button_t = 0 as *mut touch_button_t;
    button = (*list).first;
    while !button.is_null() {
        if Touch_IsVisible(button) as u64 != 0 {
            let mut color: rgba_t = [0; 4];
            color[0 as libc::c_int as usize] =
                (*button).color[0 as libc::c_int as usize];
            color[1 as libc::c_int as usize] =
                (*button).color[1 as libc::c_int as usize];
            color[2 as libc::c_int as usize] =
                (*button).color[2 as libc::c_int as usize];
            color[3 as libc::c_int as usize] =
                (*button).color[3 as libc::c_int as usize];
            if (*button).fadespeed != 0. {
                (*button).fade =
                    ((*button).fade as libc::c_double +
                         (*button).fadespeed as libc::c_double *
                             host.frametime) as libc::c_float;
                (*button).fade =
                    if (*button).fade >= 0 as libc::c_int as libc::c_float {
                        if (*button).fade < 1 as libc::c_int as libc::c_float
                           {
                            (*button).fade
                        } else { 1 as libc::c_int as libc::c_float }
                    } else { 0 as libc::c_int as libc::c_float };
                if (*button).fade == 0 as libc::c_int as libc::c_float ||
                       (*button).fade == 1 as libc::c_int as libc::c_float {
                    (*button).fadespeed = 0 as libc::c_int as libc::c_float
                }
                if (*button).fade >= (*button).fadeend &&
                       (*button).fadespeed > 0 as libc::c_int as libc::c_float
                       ||
                       (*button).fade <= (*button).fadeend &&
                           (*button).fadespeed <
                               0 as libc::c_int as libc::c_float {
                    (*button).fadespeed = 0 as libc::c_int as libc::c_float;
                    (*button).fade = (*button).fadeend
                }
            }
            if (*button).finger != -(1 as libc::c_int) &&
                   (*button).flags as libc::c_uint &
                       (1 as libc::c_uint) << 2 as libc::c_int == 0 {
                color[0 as libc::c_int as usize] =
                    if color[0 as libc::c_int as usize] as libc::c_float *
                           (*touch_highlight_r).value >=
                           0 as libc::c_int as libc::c_float {
                        if color[0 as libc::c_int as usize] as libc::c_float *
                               (*touch_highlight_r).value <
                               255 as libc::c_int as libc::c_float {
                            (color[0 as libc::c_int as usize] as
                                 libc::c_float) * (*touch_highlight_r).value
                        } else { 255 as libc::c_int as libc::c_float }
                    } else { 0 as libc::c_int as libc::c_float } as byte;
                color[1 as libc::c_int as usize] =
                    if color[1 as libc::c_int as usize] as libc::c_float *
                           (*touch_highlight_g).value >=
                           0 as libc::c_int as libc::c_float {
                        if color[1 as libc::c_int as usize] as libc::c_float *
                               (*touch_highlight_g).value <
                               255 as libc::c_int as libc::c_float {
                            (color[1 as libc::c_int as usize] as
                                 libc::c_float) * (*touch_highlight_g).value
                        } else { 255 as libc::c_int as libc::c_float }
                    } else { 0 as libc::c_int as libc::c_float } as byte;
                color[2 as libc::c_int as usize] =
                    if color[2 as libc::c_int as usize] as libc::c_float *
                           (*touch_highlight_b).value >=
                           0 as libc::c_int as libc::c_float {
                        if color[2 as libc::c_int as usize] as libc::c_float *
                               (*touch_highlight_b).value <
                               255 as libc::c_int as libc::c_float {
                            (color[2 as libc::c_int as usize] as
                                 libc::c_float) * (*touch_highlight_b).value
                        } else { 255 as libc::c_int as libc::c_float }
                    } else { 0 as libc::c_int as libc::c_float } as byte;
                color[3 as libc::c_int as usize] =
                    if color[3 as libc::c_int as usize] as libc::c_float *
                           (*touch_highlight_a).value >=
                           0 as libc::c_int as libc::c_float {
                        if color[3 as libc::c_int as usize] as libc::c_float *
                               (*touch_highlight_a).value <
                               255 as libc::c_int as libc::c_float {
                            (color[3 as libc::c_int as usize] as
                                 libc::c_float) * (*touch_highlight_a).value
                        } else { 255 as libc::c_int as libc::c_float }
                    } else { 0 as libc::c_int as libc::c_float } as byte
            }
            color[3 as libc::c_int as usize] =
                (color[3 as libc::c_int as usize] as libc::c_float *
                     (*button).fade) as byte;
            if (*button).texturefile[0 as libc::c_int as usize] as libc::c_int
                   == '#' as i32 {
                Touch_DrawText(touch.swidth as libc::c_float /
                                   refState.width as libc::c_float +
                                   (*button).x1,
                               touch.swidth as libc::c_float /
                                   refState.height as libc::c_float +
                                   (*button).y1, (*button).x2, (*button).y2,
                               (*button).texturefile.as_mut_ptr().offset(1 as
                                                                             libc::c_int
                                                                             as
                                                                             isize),
                               color.as_mut_ptr(),
                               if (*button).aspect != 0. {
                                   (*button).aspect
                               } else { 1 as libc::c_int as libc::c_float });
            } else if (*button).texturefile[0 as libc::c_int as usize] != 0 {
                if (*button).texture == -(1 as libc::c_int) {
                    (*button).texture =
                        ref_0.dllFuncs.GL_LoadTexture.expect("non-null function pointer")((*button).texturefile.as_mut_ptr(),
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
                                                                                              libc::c_int)
                }
                if (*button).flags as libc::c_uint &
                       (1 as libc::c_uint) << 7 as libc::c_int != 0 {
                    ref_0.dllFuncs.GL_SetRenderMode.expect("non-null function pointer")(kRenderTransAdd
                                                                                            as
                                                                                            libc::c_int);
                }
                Touch_DrawTexture((*button).x1, (*button).y1, (*button).x2,
                                  (*button).y2, (*button).texture,
                                  color[0 as libc::c_int as usize],
                                  color[1 as libc::c_int as usize],
                                  color[2 as libc::c_int as usize],
                                  color[3 as libc::c_int as usize]);
                ref_0.dllFuncs.GL_SetRenderMode.expect("non-null function pointer")(kRenderTransTexture
                                                                                        as
                                                                                        libc::c_int);
            }
            if (*button).flags as libc::c_uint &
                   (1 as libc::c_uint) << 8 as libc::c_int != 0 {
                ref_0.dllFuncs.Color4ub.expect("non-null function pointer")(touch.scolor[0
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             usize],
                                                                            touch.scolor[1
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             usize],
                                                                            touch.scolor[2
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             usize],
                                                                            (touch.scolor[3
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              usize]
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_float
                                                                                 *
                                                                                 (*button).fade)
                                                                                as
                                                                                libc::c_uchar);
                ref_0.dllFuncs.R_DrawStretchPic.expect("non-null function pointer")(refState.width
                                                                                        as
                                                                                        libc::c_float
                                                                                        *
                                                                                        (*button).x1,
                                                                                    refState.height
                                                                                        as
                                                                                        libc::c_float
                                                                                        *
                                                                                        (*button).y1,
                                                                                    touch.swidth
                                                                                        as
                                                                                        libc::c_float,
                                                                                    refState.height
                                                                                        as
                                                                                        libc::c_float
                                                                                        *
                                                                                        ((*button).y2
                                                                                             -
                                                                                             (*button).y1)
                                                                                        -
                                                                                        touch.swidth
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
                                                                                    touch.whitetexture);
                ref_0.dllFuncs.R_DrawStretchPic.expect("non-null function pointer")(refState.width
                                                                                        as
                                                                                        libc::c_float
                                                                                        *
                                                                                        (*button).x1
                                                                                        +
                                                                                        touch.swidth
                                                                                            as
                                                                                            libc::c_float,
                                                                                    refState.height
                                                                                        as
                                                                                        libc::c_float
                                                                                        *
                                                                                        (*button).y1,
                                                                                    refState.width
                                                                                        as
                                                                                        libc::c_float
                                                                                        *
                                                                                        ((*button).x2
                                                                                             -
                                                                                             (*button).x1)
                                                                                        -
                                                                                        touch.swidth
                                                                                            as
                                                                                            libc::c_float,
                                                                                    touch.swidth
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
                                                                                    touch.whitetexture);
                ref_0.dllFuncs.R_DrawStretchPic.expect("non-null function pointer")(refState.width
                                                                                        as
                                                                                        libc::c_float
                                                                                        *
                                                                                        (*button).x2
                                                                                        -
                                                                                        touch.swidth
                                                                                            as
                                                                                            libc::c_float,
                                                                                    refState.height
                                                                                        as
                                                                                        libc::c_float
                                                                                        *
                                                                                        (*button).y1
                                                                                        +
                                                                                        touch.swidth
                                                                                            as
                                                                                            libc::c_float,
                                                                                    touch.swidth
                                                                                        as
                                                                                        libc::c_float,
                                                                                    refState.height
                                                                                        as
                                                                                        libc::c_float
                                                                                        *
                                                                                        ((*button).y2
                                                                                             -
                                                                                             (*button).y1)
                                                                                        -
                                                                                        touch.swidth
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
                                                                                    touch.whitetexture);
                ref_0.dllFuncs.R_DrawStretchPic.expect("non-null function pointer")(refState.width
                                                                                        as
                                                                                        libc::c_float
                                                                                        *
                                                                                        (*button).x1,
                                                                                    refState.height
                                                                                        as
                                                                                        libc::c_float
                                                                                        *
                                                                                        (*button).y2
                                                                                        -
                                                                                        touch.swidth
                                                                                            as
                                                                                            libc::c_float,
                                                                                    refState.width
                                                                                        as
                                                                                        libc::c_float
                                                                                        *
                                                                                        ((*button).x2
                                                                                             -
                                                                                             (*button).x1)
                                                                                        -
                                                                                        touch.swidth
                                                                                            as
                                                                                            libc::c_float,
                                                                                    touch.swidth
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
                                                                                    touch.whitetexture);
                ref_0.dllFuncs.Color4ub.expect("non-null function pointer")(255
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uchar,
                                                                            255
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uchar,
                                                                            255
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uchar,
                                                                            255
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uchar);
            }
        }
        if touch.state as libc::c_uint >=
               state_edit as libc::c_int as libc::c_uint &&
               (*button).flags as libc::c_uint &
                   (1 as libc::c_uint) << 1 as libc::c_int == 0 {
            let mut color_0: rgba_t = [0; 4];
            if (*button).flags as libc::c_uint &
                   (1 as libc::c_uint) << 0 as libc::c_int == 0 {
                Touch_DrawTexture((*button).x1, (*button).y1, (*button).x2,
                                  (*button).y2, touch.whitetexture,
                                  255 as libc::c_int as byte,
                                  255 as libc::c_int as byte,
                                  0 as libc::c_int as byte,
                                  32 as libc::c_int as byte);
            } else {
                Touch_DrawTexture((*button).x1, (*button).y1, (*button).x2,
                                  (*button).y2, touch.whitetexture,
                                  128 as libc::c_int as byte,
                                  128 as libc::c_int as byte,
                                  128 as libc::c_int as byte,
                                  128 as libc::c_int as byte);
            }
            color_0[0 as libc::c_int as usize] = 255 as libc::c_int as byte;
            color_0[1 as libc::c_int as usize] = 255 as libc::c_int as byte;
            color_0[2 as libc::c_int as usize] = 127 as libc::c_int as byte;
            color_0[3 as libc::c_int as usize] = 255 as libc::c_int as byte;
            Con_DrawString((refState.width as libc::c_float * (*button).x1) as
                               libc::c_int,
                           (refState.height as libc::c_float * (*button).y1)
                               as libc::c_int, (*button).name.as_mut_ptr(),
                           color_0.as_mut_ptr());
        }
        button = (*button).next
    };
}
#[no_mangle]
pub unsafe extern "C" fn Touch_Draw() {
    let mut button: *mut touch_button_t = 0 as *mut touch_button_t;
    if touch.initialized as u64 == 0 ||
           (if !touch_enable.is_null() && (*touch_enable).value != 0.0f32 {
                true_0 as libc::c_int
            } else { false_0 as libc::c_int }) == 0 &&
               touch.clientonly as u64 == 0 {
        return
    }
    Touch_InitConfig();
    if cls.key_dest as libc::c_uint != key_game as libc::c_int as libc::c_uint
           &&
           (if !touch_in_menu.is_null() && (*touch_in_menu).value != 0.0f32 {
                true_0 as libc::c_int
            } else { false_0 as libc::c_int }) == 0 {
        return
    }
    ref_0.dllFuncs.GL_SetRenderMode.expect("non-null function pointer")(kRenderTransTexture
                                                                            as
                                                                            libc::c_int);
    if touch.state as libc::c_uint >=
           state_edit as libc::c_int as libc::c_uint &&
           (if !touch_grid_enable.is_null() &&
                   (*touch_grid_enable).value != 0.0f32 {
                true_0 as libc::c_int
            } else { false_0 as libc::c_int }) != 0 {
        let mut x: libc::c_float = 0.;
        if if !touch_in_menu.is_null() && (*touch_in_menu).value != 0.0f32 {
               true_0 as libc::c_int
           } else { false_0 as libc::c_int } != 0 {
            Touch_DrawTexture(0 as libc::c_int as libc::c_float,
                              0 as libc::c_int as libc::c_float,
                              1 as libc::c_int as libc::c_float,
                              1 as libc::c_int as libc::c_float,
                              touch.whitetexture, 32 as libc::c_int as byte,
                              32 as libc::c_int as byte,
                              32 as libc::c_int as byte,
                              255 as libc::c_int as byte);
        } else {
            Touch_DrawTexture(0 as libc::c_int as libc::c_float,
                              0 as libc::c_int as libc::c_float,
                              1 as libc::c_int as libc::c_float,
                              1 as libc::c_int as libc::c_float,
                              touch.whitetexture, 0 as libc::c_int as byte,
                              0 as libc::c_int as byte,
                              0 as libc::c_int as byte,
                              112 as libc::c_int as byte);
        }
        ref_0.dllFuncs.Color4ub.expect("non-null function pointer")(0 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uchar,
                                                                    224 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uchar,
                                                                    224 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uchar,
                                                                    112 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uchar);
        x = 0 as libc::c_int as libc::c_float;
        while x < 1 as libc::c_int as libc::c_float {
            ref_0.dllFuncs.R_DrawStretchPic.expect("non-null function pointer")(refState.width
                                                                                    as
                                                                                    libc::c_float
                                                                                    *
                                                                                    x,
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
                                                                                (refState.height
                                                                                     *
                                                                                     1
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
                                                                                touch.whitetexture);
            x +=
                1.0f32 /
                    (*touch_grid_count).value as libc::c_int as libc::c_float
        }
        x = 0 as libc::c_int as libc::c_float;
        while x < 1 as libc::c_int as libc::c_float {
            ref_0.dllFuncs.R_DrawStretchPic.expect("non-null function pointer")(0
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_float,
                                                                                refState.height
                                                                                    as
                                                                                    libc::c_float
                                                                                    *
                                                                                    x,
                                                                                (refState.width
                                                                                     *
                                                                                     1
                                                                                         as
                                                                                         libc::c_int)
                                                                                    as
                                                                                    libc::c_float,
                                                                                1
                                                                                    as
                                                                                    libc::c_int
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
                                                                                touch.whitetexture);
            x +=
                refState.width as libc::c_float /
                    refState.height as libc::c_float /
                    (*touch_grid_count).value as libc::c_int as libc::c_float
        }
    }
    Touch_DrawButtons(&mut touch.list_user);
    if touch.state as libc::c_uint >=
           state_edit as libc::c_int as libc::c_uint {
        let mut color: rgba_t = [0; 4];
        color[0 as libc::c_int as usize] = 255 as libc::c_int as byte;
        color[1 as libc::c_int as usize] = 255 as libc::c_int as byte;
        color[2 as libc::c_int as usize] = 255 as libc::c_int as byte;
        color[3 as libc::c_int as usize] = 255 as libc::c_int as byte;
        if !touch.edit.is_null() {
            let mut x1: libc::c_float = (*touch.edit).x1;
            let mut y1: libc::c_float = (*touch.edit).y1;
            let mut x2: libc::c_float = (*touch.edit).x2;
            let mut y2: libc::c_float = (*touch.edit).y2;
            IN_TouchCheckCoords(&mut x1, &mut y1, &mut x2, &mut y2);
            Touch_DrawTexture(x1, y1, x2, y2, touch.whitetexture,
                              0 as libc::c_int as byte,
                              255 as libc::c_int as byte,
                              0 as libc::c_int as byte,
                              32 as libc::c_int as byte);
        }
        Touch_DrawTexture(0 as libc::c_int as libc::c_float,
                          0 as libc::c_int as libc::c_float,
                          1.0f32 /
                              (*touch_grid_count).value as libc::c_int as
                                  libc::c_float,
                          refState.width as libc::c_float /
                              refState.height as libc::c_float /
                              (*touch_grid_count).value as libc::c_int as
                                  libc::c_float, touch.whitetexture,
                          255 as libc::c_int as byte,
                          255 as libc::c_int as byte,
                          255 as libc::c_int as byte,
                          64 as libc::c_int as byte);
        if touch.showeditbuttons as u64 != 0 {
            Touch_DrawButtons(&mut touch.list_edit);
        }
        // / TODO: move to mainui
        if !touch.selection.is_null() {
            button = touch.selection;
            Touch_DrawTexture((*button).x1, (*button).y1, (*button).x2,
                              (*button).y2, touch.whitetexture,
                              255 as libc::c_int as byte,
                              0 as libc::c_int as byte,
                              0 as libc::c_int as byte,
                              64 as libc::c_int as byte);
            Con_DrawString(0 as libc::c_int,
                           (refState.height as libc::c_float *
                                (refState.width as libc::c_float /
                                     refState.height as libc::c_float /
                                     (*touch_grid_count).value as libc::c_int
                                         as libc::c_float *
                                     11 as libc::c_int as libc::c_float)) as
                               libc::c_int,
                           b"Selection:\x00" as *const u8 as
                               *const libc::c_char, color.as_mut_ptr());
            Con_DrawString(Con_DrawString(0 as libc::c_int,
                                          (refState.height as libc::c_float *
                                               (refState.width as
                                                    libc::c_float /
                                                    refState.height as
                                                        libc::c_float /
                                                    (*touch_grid_count).value
                                                        as libc::c_int as
                                                        libc::c_float *
                                                    12 as libc::c_int as
                                                        libc::c_float)) as
                                              libc::c_int,
                                          b"Name: \x00" as *const u8 as
                                              *const libc::c_char,
                                          color.as_mut_ptr()),
                           (refState.height as libc::c_float *
                                (refState.width as libc::c_float /
                                     refState.height as libc::c_float /
                                     (*touch_grid_count).value as libc::c_int
                                         as libc::c_float *
                                     12 as libc::c_int as libc::c_float)) as
                               libc::c_int, (*button).name.as_mut_ptr(),
                           color.as_mut_ptr());
            Con_DrawString(Con_DrawString(0 as libc::c_int,
                                          (refState.height as libc::c_float *
                                               (refState.width as
                                                    libc::c_float /
                                                    refState.height as
                                                        libc::c_float /
                                                    (*touch_grid_count).value
                                                        as libc::c_int as
                                                        libc::c_float *
                                                    13 as libc::c_int as
                                                        libc::c_float)) as
                                              libc::c_int,
                                          b"Texture: \x00" as *const u8 as
                                              *const libc::c_char,
                                          color.as_mut_ptr()),
                           (refState.height as libc::c_float *
                                (refState.width as libc::c_float /
                                     refState.height as libc::c_float /
                                     (*touch_grid_count).value as libc::c_int
                                         as libc::c_float *
                                     13 as libc::c_int as libc::c_float)) as
                               libc::c_int,
                           (*button).texturefile.as_mut_ptr(),
                           color.as_mut_ptr());
            Con_DrawString(Con_DrawString(0 as libc::c_int,
                                          (refState.height as libc::c_float *
                                               (refState.width as
                                                    libc::c_float /
                                                    refState.height as
                                                        libc::c_float /
                                                    (*touch_grid_count).value
                                                        as libc::c_int as
                                                        libc::c_float *
                                                    14 as libc::c_int as
                                                        libc::c_float)) as
                                              libc::c_int,
                                          b"Command: \x00" as *const u8 as
                                              *const libc::c_char,
                                          color.as_mut_ptr()),
                           (refState.height as libc::c_float *
                                (refState.width as libc::c_float /
                                     refState.height as libc::c_float /
                                     (*touch_grid_count).value as libc::c_int
                                         as libc::c_float *
                                     14 as libc::c_int as libc::c_float)) as
                               libc::c_int, (*button).command.as_mut_ptr(),
                           color.as_mut_ptr());
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
    if touch.move_finger != -(1 as libc::c_int) &&
           !touch.move_button.is_null() && (*touch_move_indicator).value != 0.
       {
        let mut width: libc::c_float = 0.;
        let mut height: libc::c_float = 0.;
        if (*touch_joy_texture).flags &
               (1 as libc::c_int) << 13 as libc::c_int != 0 {
            (*touch_joy_texture).flags =
                (*touch_joy_texture).flags &
                    !((1 as libc::c_int) << 13 as libc::c_int);
            touch.joytexture =
                ref_0.dllFuncs.GL_LoadTexture.expect("non-null function pointer")((*touch_joy_texture).string,
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
                                                                                      libc::c_int)
        }
        if (*touch.move_button).type_0 as libc::c_uint ==
               touch_move as libc::c_int as libc::c_uint {
            width = (*touch_sidezone).value;
            height = (*touch_forwardzone).value
        } else {
            width =
                ((*touch.move_button).x2 - (*touch.move_button).x1) /
                    2 as libc::c_int as libc::c_float;
            height =
                ((*touch.move_button).y2 - (*touch.move_button).y1) /
                    2 as libc::c_int as libc::c_float
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
                                                                    128 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uchar);
        ref_0.dllFuncs.R_DrawStretchPic.expect("non-null function pointer")(refState.width
                                                                                as
                                                                                libc::c_float
                                                                                *
                                                                                (touch.move_start_x
                                                                                     -
                                                                                     1.0f32
                                                                                         /
                                                                                         (*touch_grid_count).value
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_float
                                                                                         *
                                                                                         (*touch_move_indicator).value),
                                                                            refState.height
                                                                                as
                                                                                libc::c_float
                                                                                *
                                                                                (touch.move_start_y
                                                                                     -
                                                                                     refState.width
                                                                                         as
                                                                                         libc::c_float
                                                                                         /
                                                                                         refState.height
                                                                                             as
                                                                                             libc::c_float
                                                                                         /
                                                                                         (*touch_grid_count).value
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_float
                                                                                         *
                                                                                         (*touch_move_indicator).value),
                                                                            refState.width
                                                                                as
                                                                                libc::c_float
                                                                                *
                                                                                (1.0f32
                                                                                     /
                                                                                     (*touch_grid_count).value
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_float
                                                                                     *
                                                                                     2
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_float
                                                                                     *
                                                                                     (*touch_move_indicator).value),
                                                                            refState.height
                                                                                as
                                                                                libc::c_float
                                                                                *
                                                                                (refState.width
                                                                                     as
                                                                                     libc::c_float
                                                                                     /
                                                                                     refState.height
                                                                                         as
                                                                                         libc::c_float
                                                                                     /
                                                                                     (*touch_grid_count).value
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_float
                                                                                     *
                                                                                     2
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_float
                                                                                     *
                                                                                     (*touch_move_indicator).value),
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
                                                                            touch.joytexture);
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
        ref_0.dllFuncs.R_DrawStretchPic.expect("non-null function pointer")(refState.width
                                                                                as
                                                                                libc::c_float
                                                                                *
                                                                                (touch.move_start_x
                                                                                     +
                                                                                     touch.side
                                                                                         *
                                                                                         width
                                                                                     -
                                                                                     1.0f32
                                                                                         /
                                                                                         (*touch_grid_count).value
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_float
                                                                                         *
                                                                                         (*touch_move_indicator).value),
                                                                            refState.height
                                                                                as
                                                                                libc::c_float
                                                                                *
                                                                                (touch.move_start_y
                                                                                     -
                                                                                     touch.forward
                                                                                         *
                                                                                         height
                                                                                     -
                                                                                     refState.width
                                                                                         as
                                                                                         libc::c_float
                                                                                         /
                                                                                         refState.height
                                                                                             as
                                                                                             libc::c_float
                                                                                         /
                                                                                         (*touch_grid_count).value
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_float
                                                                                         *
                                                                                         (*touch_move_indicator).value),
                                                                            refState.width
                                                                                as
                                                                                libc::c_float
                                                                                *
                                                                                (1.0f32
                                                                                     /
                                                                                     (*touch_grid_count).value
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_float
                                                                                     *
                                                                                     2
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_float
                                                                                     *
                                                                                     (*touch_move_indicator).value),
                                                                            refState.height
                                                                                as
                                                                                libc::c_float
                                                                                *
                                                                                (refState.width
                                                                                     as
                                                                                     libc::c_float
                                                                                     /
                                                                                     refState.height
                                                                                         as
                                                                                         libc::c_float
                                                                                     /
                                                                                     (*touch_grid_count).value
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_float
                                                                                     *
                                                                                     2
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_float
                                                                                     *
                                                                                     (*touch_move_indicator).value),
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
                                                                            touch.joytexture);
    };
}
// clear move and selection state
unsafe extern "C" fn IN_TouchEditClear() {
    // allow keep move/look fingers when doing touch_removeall
	//touch.move_finger = touch.look_finger = -1;
    if (touch.state as libc::c_uint) <
           state_edit as libc::c_int as libc::c_uint {
        return
    }
    touch.state = state_edit;
    if !touch.edit.is_null() { (*touch.edit).finger = -(1 as libc::c_int) }
    touch.resize_finger = -(1 as libc::c_int);
    touch.edit = 0 as *mut touch_button_t;
    touch.selection = 0 as *mut touch_button_t;
}
unsafe extern "C" fn Touch_EditMove(mut type_0: touchEventType,
                                    mut fingerID: libc::c_int,
                                    mut x: libc::c_float,
                                    mut y: libc::c_float,
                                    mut dx: libc::c_float,
                                    mut dy: libc::c_float) {
    if (*touch.edit).finger == fingerID {
        if type_0 as libc::c_uint == event_up as libc::c_int as libc::c_uint {
            // shutdown button move
            let mut button: *mut touch_button_t = touch.edit;
            IN_TouchCheckCoords(&mut (*button).x1, &mut (*button).y1,
                                &mut (*button).x2, &mut (*button).y2);
            IN_TouchEditClear();
            if (*button).type_0 as libc::c_uint ==
                   touch_command as libc::c_int as libc::c_uint {
                touch.selection = button;
                // update "hide" editor button
                (*touch.hidebutton).texture = -(1 as libc::c_int);
                (*touch.hidebutton).flags =
                    ((*touch.hidebutton).flags as libc::c_uint &
                         !((1 as libc::c_uint) << 0 as libc::c_int)) as
                        libc::c_int;
                if (*button).flags as libc::c_uint &
                       (1 as libc::c_uint) << 0 as libc::c_int != 0 {
                    Q_strncpy((*touch.hidebutton).texturefile.as_mut_ptr(),
                              b"touch_default/edit_show.tga\x00" as *const u8
                                  as *const libc::c_char,
                              99999 as libc::c_int as size_t);
                } else {
                    Q_strncpy((*touch.hidebutton).texturefile.as_mut_ptr(),
                              b"touch_default/edit_hide.tga\x00" as *const u8
                                  as *const libc::c_char,
                              99999 as libc::c_int as size_t);
                }
            }
        }
        if type_0 as libc::c_uint ==
               event_motion as libc::c_int as libc::c_uint {
            // shutdown button move
            (*touch.edit).y1 += dy;
            (*touch.edit).y2 += dy;
            (*touch.edit).x1 += dx;
            (*touch.edit).x2 += dx
        }
    } else {
        if type_0 as libc::c_uint == event_down as libc::c_int as libc::c_uint
           {
            // enable resizing
            if touch.resize_finger == -(1 as libc::c_int) {
                touch.resize_finger = fingerID
            }
        }
        if type_0 as libc::c_uint == event_up as libc::c_int as libc::c_uint {
            // disable resizing
            if touch.resize_finger == fingerID {
                touch.resize_finger = -(1 as libc::c_int)
            }
        }
        if type_0 as libc::c_uint ==
               event_motion as libc::c_int as libc::c_uint {
            // perform resizing
            if touch.resize_finger == fingerID {
                (*touch.edit).y2 += dy;
                (*touch.edit).x2 += dx
            }
        }
    };
}
unsafe extern "C" fn Touch_Motion(mut type_0: touchEventType,
                                  mut fingerID: libc::c_int,
                                  mut x: libc::c_float, mut y: libc::c_float,
                                  mut dx: libc::c_float,
                                  mut dy: libc::c_float) {
    // process wheel
    if fingerID == touch.wheel_finger {
        touch.wheel_amount +=
            if touch.wheel_horizontal as libc::c_uint != 0 { dx } else { dy };
        if touch.wheel_amount > 0.1f32 {
            Cbuf_AddText(touch.wheel_down.as_mut_ptr());
            touch.wheel_count += 1;
            touch.wheel_amount = 0 as libc::c_int as libc::c_float
        }
        if touch.wheel_amount < -0.1f32 {
            Cbuf_AddText(touch.wheel_up.as_mut_ptr());
            touch.wheel_count += 1;
            touch.wheel_amount = 0 as libc::c_int as libc::c_float
        }
        return
    }
    // walk
    if fingerID == touch.move_finger {
        // check bounds
        if (*touch_forwardzone).value <= 0 as libc::c_int as libc::c_float {
            Cvar_SetValue(b"touch_forwardzone\x00" as *const u8 as
                              *const libc::c_char, 0.5f64 as libc::c_float);
        }
        if (*touch_sidezone).value <= 0 as libc::c_int as libc::c_float {
            Cvar_SetValue(b"touch_sidezone\x00" as *const u8 as
                              *const libc::c_char, 0.3f64 as libc::c_float);
        }
        if touch.move_button.is_null() ||
               (*touch.move_button).type_0 as libc::c_uint ==
                   touch_move as libc::c_int as libc::c_uint {
            // move relative to touch start
            touch.forward =
                (touch.move_start_y - y) / (*touch_forwardzone).value;
            touch.side = (x - touch.move_start_x) / (*touch_sidezone).value
        } else if (*touch.move_button).type_0 as libc::c_uint ==
                      touch_joy as libc::c_int as libc::c_uint {
            // move relative to joy center
            touch.forward =
                ((*touch.move_button).y2 + (*touch.move_button).y1 -
                     y * 2 as libc::c_int as libc::c_float) /
                    ((*touch.move_button).y2 - (*touch.move_button).y1) *
                    (*touch_joy_radius).value;
            touch.side =
                (x * 2 as libc::c_int as libc::c_float -
                     ((*touch.move_button).x2 + (*touch.move_button).x1)) /
                    ((*touch.move_button).x2 - (*touch.move_button).x1) *
                    (*touch_joy_radius).value
        } else if (*touch.move_button).type_0 as libc::c_uint ==
                      touch_dpad as libc::c_int as libc::c_uint {
            // like joy, but without acceleration. useful for bhop
            touch.forward =
                __tg_round(((*touch.move_button).y2 + (*touch.move_button).y1
                                - y * 2 as libc::c_int as libc::c_float) /
                               ((*touch.move_button).y2 -
                                    (*touch.move_button).y1) *
                               (*touch_dpad_radius).value);
            touch.side =
                __tg_round((x * 2 as libc::c_int as libc::c_float -
                                ((*touch.move_button).x2 +
                                     (*touch.move_button).x1)) /
                               ((*touch.move_button).x2 -
                                    (*touch.move_button).x1) *
                               (*touch_dpad_radius).value)
        }
        touch.forward =
            if touch.forward >= -(1 as libc::c_int) as libc::c_float {
                if touch.forward < 1 as libc::c_int as libc::c_float {
                    touch.forward
                } else { 1 as libc::c_int as libc::c_float }
            } else { -(1 as libc::c_int) as libc::c_float };
        touch.side =
            if touch.side >= -(1 as libc::c_int) as libc::c_float {
                if touch.side < 1 as libc::c_int as libc::c_float {
                    touch.side
                } else { 1 as libc::c_int as libc::c_float }
            } else { -(1 as libc::c_int) as libc::c_float }
    }
    // process look
    if fingerID == touch.look_finger {
        if touch.precision as u64 != 0 {
            dx *= (*touch_precise_amount).value;
            dy *= (*touch_precise_amount).value
        }
        if if !touch_nonlinear_look.is_null() &&
                  (*touch_nonlinear_look).value != 0.0f32 {
               true_0 as libc::c_int
           } else { false_0 as libc::c_int } != 0 {
            let mut dabs: libc::c_float = 0.;
            let mut dcos: libc::c_float = 0.;
            let mut dsin: libc::c_float = 0.;
            // save angle, modify only velocity
            dabs =
                __tg_sqrt(dx * dx +
                              dy * dy); // no motion, avoid division by zero
            if dabs < 0.000001f32 { return }
            dcos = dx / dabs;
            dsin = dy / dabs;
            if (*touch_exp_mult).value > 1 as libc::c_int as libc::c_float {
                dabs =
                    (__tg_exp(dabs * (*touch_exp_mult).value) -
                         1 as libc::c_int as libc::c_float) /
                        (*touch_exp_mult).value
            }
            if (*touch_pow_mult).value > 1 as libc::c_int as libc::c_float &&
                   (*touch_pow_factor).value >
                       1 as libc::c_int as libc::c_float {
                dabs =
                    __tg_pow(dabs * (*touch_pow_mult).value,
                             (*touch_pow_factor).value) /
                        (*touch_pow_mult).value
            }
            dx = dabs * dcos;
            dy = dabs * dsin
        }
        // prevent breaking engine/client with bad values
        if dx.is_nan() as i32 != 0 || dy.is_nan() as i32 != 0 { return }
        // accumulate
        touch.yaw -= dx * (*touch_yaw).value;
        touch.pitch += dy * (*touch_pitch).value
    };
}
unsafe extern "C" fn Touch_ButtonPress(mut list: *mut touchbuttonlist_t,
                                       mut type_0: touchEventType,
                                       mut fingerID: libc::c_int,
                                       mut x: libc::c_float,
                                       mut y: libc::c_float,
                                       mut dx: libc::c_float,
                                       mut dy: libc::c_float) -> qboolean {
    let mut button: *mut touch_button_t = 0 as *mut touch_button_t;
    let mut result: qboolean = false_0;
    let mut current_block_99: u64;
    // run from end(front) to start(back)
    button = (*list).last;
    while !button.is_null() {
        // skip invisible buttons
        if !(Touch_IsVisible(button) as u64 == 0) {
            if type_0 as libc::c_uint ==
                   event_down as libc::c_int as libc::c_uint {
                // button bounds check
                if x > (*button).x1 && x < (*button).x2 &&
                       (y < (*button).y2 && y > (*button).y1) {
                    (*button).finger = fingerID;
                    if (*button).type_0 as libc::c_uint ==
                           touch_command as libc::c_int as libc::c_uint {
                        let mut command: [libc::c_char; 256] = [0; 256];
                        // command down: just execute command
                        Q_snprintf(command.as_mut_ptr(),
                                   ::std::mem::size_of::<[libc::c_char; 256]>()
                                       as libc::c_ulong,
                                   b"%s\n\x00" as *const u8 as
                                       *const libc::c_char,
                                   (*button).command.as_mut_ptr());
                        if (*button).flags as libc::c_uint &
                               (1 as libc::c_uint) << 10 as libc::c_int != 0 {
                            Cbuf_AddFilteredText(command.as_mut_ptr());
                        } else { Cbuf_AddText(command.as_mut_ptr()); }
                        // increase precision
                        if (*button).flags as libc::c_uint &
                               (1 as libc::c_uint) << 9 as libc::c_int != 0 {
                            touch.precision = true_0
                        }
                        result = true_0
                    }
                    if (*button).type_0 as libc::c_uint ==
                           touch_wheel as libc::c_int as libc::c_uint {
                        let mut command_0: string = [0; 256];
                        touch.wheel_finger = fingerID;
                        touch.wheel_count = 0 as libc::c_int;
                        touch.wheel_amount =
                            touch.wheel_count as libc::c_float;
                        Cmd_TokenizeString((*button).command.as_mut_ptr());
                        touch.wheel_horizontal =
                            (Q_strncmp(Cmd_Argv(0 as libc::c_int),
                                       b"_hwheel\x00" as *const u8 as
                                           *const libc::c_char,
                                       99999 as libc::c_int) == 0) as
                                libc::c_int as qboolean;
                        Q_snprintf(touch.wheel_up.as_mut_ptr(),
                                   ::std::mem::size_of::<string>() as
                                       libc::c_ulong,
                                   b"%s\n\x00" as *const u8 as
                                       *const libc::c_char,
                                   Cmd_Argv(1 as libc::c_int));
                        Q_snprintf(touch.wheel_down.as_mut_ptr(),
                                   ::std::mem::size_of::<string>() as
                                       libc::c_ulong,
                                   b"%s\n\x00" as *const u8 as
                                       *const libc::c_char,
                                   Cmd_Argv(2 as libc::c_int));
                        Q_snprintf(touch.wheel_end.as_mut_ptr(),
                                   ::std::mem::size_of::<string>() as
                                       libc::c_ulong,
                                   b"%s\n\x00" as *const u8 as
                                       *const libc::c_char,
                                   Cmd_Argv(3 as libc::c_int));
                        if Q_snprintf(command_0.as_mut_ptr(),
                                      ::std::mem::size_of::<string>() as
                                          libc::c_ulong,
                                      b"%s\n\x00" as *const u8 as
                                          *const libc::c_char,
                                      Cmd_Argv(4 as libc::c_int)) >
                               1 as libc::c_int {
                            if (*button).flags as libc::c_uint &
                                   (1 as libc::c_uint) << 10 as libc::c_int !=
                                   0 {
                                Cbuf_AddFilteredText(command_0.as_mut_ptr());
                            } else { Cbuf_AddText(command_0.as_mut_ptr()); }
                            touch.wheel_count += 1
                        }
                        // increase precision
                        if (*button).flags as libc::c_uint &
                               (1 as libc::c_uint) << 9 as libc::c_int != 0 {
                            touch.precision = true_0
                        }
                        result = true_0
                    }
                    // initialize motion when player touched motion zone
                    if (*button).type_0 as libc::c_uint ==
                           touch_move as libc::c_int as libc::c_uint ||
                           (*button).type_0 as libc::c_uint ==
                               touch_joy as libc::c_int as libc::c_uint ||
                           (*button).type_0 as libc::c_uint ==
                               touch_dpad as libc::c_int as libc::c_uint {
                        if touch.move_finger != -(1 as libc::c_int) {
                            // prevent initializing move while already moving
						// revert finger switch, leave first finger
                            (*button).finger = touch.move_finger;
                            current_block_99 = 16559507199688588974;
                        } else {
                            result = true_0;
                            if touch.look_finger == fingerID {
                                let mut newbutton: *mut touch_button_t =
                                    0 as *mut touch_button_t;
                                // this is an error, try recover
                                touch.look_finger = -(1 as libc::c_int);
                                touch.move_finger = touch.look_finger;
                                // player touched touch_move with enabled look mode
						// and same finger id. release all move triggers
                                newbutton = (*list).first;
                                while !newbutton.is_null() {
                                    if (*newbutton).type_0 as libc::c_uint ==
                                           touch_move as libc::c_int as
                                               libc::c_uint ||
                                           (*newbutton).type_0 as libc::c_uint
                                               ==
                                               touch_look as libc::c_int as
                                                   libc::c_uint {
                                        (*newbutton).finger =
                                            -(1 as libc::c_int)
                                    }
                                    newbutton = (*newbutton).next
                                }
                                Con_DPrintf(b"^1Error:^7 Touch: touch_move on look finger %d!\n\x00"
                                                as *const u8 as
                                                *const libc::c_char,
                                            fingerID);
                                current_block_99 = 16559507199688588974;
                            } else {
                                // initialize move mode
                                touch.move_finger = fingerID;
                                touch.move_button = button;
                                if (*touch.move_button).type_0 as libc::c_uint
                                       ==
                                       touch_move as libc::c_int as
                                           libc::c_uint {
                                    // initial position is first touch
                                    touch.move_start_x = x;
                                    touch.move_start_y = y
                                } else if (*touch.move_button).type_0 as
                                              libc::c_uint ==
                                              touch_joy as libc::c_int as
                                                  libc::c_uint {
                                    // initial position is button center
                                    touch.move_start_y =
                                        ((*touch.move_button).y2 +
                                             (*touch.move_button).y1) /
                                            2 as libc::c_int as libc::c_float;
                                    touch.move_start_x =
                                        ((*touch.move_button).x2 +
                                             (*touch.move_button).x1) /
                                            2 as libc::c_int as libc::c_float;
                                    // start move instanly
                                    touch.forward =
                                        ((*touch.move_button).y2 +
                                             (*touch.move_button).y1 -
                                             y *
                                                 2 as libc::c_int as
                                                     libc::c_float) /
                                            ((*touch.move_button).y2 -
                                                 (*touch.move_button).y1);
                                    touch.side =
                                        (x * 2 as libc::c_int as libc::c_float
                                             -
                                             ((*touch.move_button).x2 +
                                                  (*touch.move_button).x1)) /
                                            ((*touch.move_button).x2 -
                                                 (*touch.move_button).x1)
                                } else if (*touch.move_button).type_0 as
                                              libc::c_uint ==
                                              touch_dpad as libc::c_int as
                                                  libc::c_uint {
                                    // dame as joy, but round
                                    touch.move_start_y =
                                        ((*touch.move_button).y2 +
                                             (*touch.move_button).y1) /
                                            2 as libc::c_int as libc::c_float;
                                    touch.move_start_x =
                                        ((*touch.move_button).x2 +
                                             (*touch.move_button).x1) /
                                            2 as libc::c_int as libc::c_float;
                                    // start move instanly
                                    touch.forward =
                                        __tg_round(((*touch.move_button).y2 +
                                                        (*touch.move_button).y1
                                                        -
                                                        y *
                                                            2 as libc::c_int
                                                                as
                                                                libc::c_float)
                                                       /
                                                       ((*touch.move_button).y2
                                                            -
                                                            (*touch.move_button).y1));
                                    touch.side =
                                        __tg_round((x *
                                                        2 as libc::c_int as
                                                            libc::c_float -
                                                        ((*touch.move_button).x2
                                                             +
                                                             (*touch.move_button).x1))
                                                       /
                                                       ((*touch.move_button).x2
                                                            -
                                                            (*touch.move_button).x1))
                                }
                                current_block_99 = 9512719473022792396;
                            }
                        }
                    } else { current_block_99 = 9512719473022792396; }
                    match current_block_99 {
                        16559507199688588974 => { }
                        _ =>
                        // initialize look
                        {
                            if (*button).type_0 as libc::c_uint ==
                                   touch_look as libc::c_int as libc::c_uint {
                                if touch.look_finger != -(1 as libc::c_int) {
                                    // prevent initializing look while already looking
						// revert finger switch, leave first finger
                                    (*button).finger = touch.look_finger;
                                    current_block_99 = 16559507199688588974;
                                } else {
                                    result = true_0;
                                    if touch.move_finger == fingerID {
                                        let mut newbutton_0:
                                                *mut touch_button_t =
                                            0 as *mut touch_button_t;
                                        // this is an error, try recover
                                        touch.look_finger =
                                            -(1 as libc::c_int);
                                        touch.move_finger = touch.look_finger;
                                        // player touched touch_move with enabled look mode
						// and same finger id. release all move triggers
                                        newbutton_0 = (*list).first;
                                        while !newbutton_0.is_null() {
                                            if (*newbutton_0).type_0 as
                                                   libc::c_uint ==
                                                   touch_move as libc::c_int
                                                       as libc::c_uint ||
                                                   (*newbutton_0).type_0 as
                                                       libc::c_uint ==
                                                       touch_look as
                                                           libc::c_int as
                                                           libc::c_uint {
                                                (*newbutton_0).finger =
                                                    -(1 as libc::c_int)
                                            }
                                            newbutton_0 = (*newbutton_0).next
                                        }
                                        Con_Printf(b"^1Error:^7 touch: touch_look on move finger %d!\n\x00"
                                                       as *const u8 as
                                                       *const libc::c_char,
                                                   fingerID);
                                        current_block_99 =
                                            16559507199688588974;
                                    } else {
                                        touch.look_finger = fingerID;
                                        current_block_99 =
                                            3736434875406665187;
                                    }
                                }
                            } else { current_block_99 = 3736434875406665187; }
                        }
                    }
                } else { current_block_99 = 3736434875406665187; }
            } else { current_block_99 = 3736434875406665187; }
            match current_block_99 {
                16559507199688588974 => { }
                _ => {
                    if type_0 as libc::c_uint ==
                           event_up as libc::c_int as libc::c_uint {
                        // no bounds check here.
			// button released when finger released
                        if fingerID == (*button).finger {
                            (*button).finger = -(1 as libc::c_int);
                            // handle +command, replace by -command
                            if (*button).type_0 as libc::c_uint ==
                                   touch_command as libc::c_int as
                                       libc::c_uint {
                                if (*button).command[0 as libc::c_int as
                                                         usize] as libc::c_int
                                       == '+' as i32 {
                                    let mut command_1: [libc::c_char; 256] =
                                        [0; 256];
                                    Q_snprintf(command_1.as_mut_ptr(),
                                               ::std::mem::size_of::<[libc::c_char; 256]>()
                                                   as libc::c_ulong,
                                               b"%s\n\x00" as *const u8 as
                                                   *const libc::c_char,
                                               (*button).command.as_mut_ptr());
                                    command_1[0 as libc::c_int as usize] =
                                        '-' as i32 as libc::c_char;
                                    if (*button).flags as libc::c_uint &
                                           (1 as libc::c_uint) <<
                                               10 as libc::c_int != 0 {
                                        Cbuf_AddFilteredText(command_1.as_mut_ptr());
                                    } else {
                                        Cbuf_AddText(command_1.as_mut_ptr());
                                    }
                                }
                                // disable precision mode
                                if (*button).flags as libc::c_uint &
                                       (1 as libc::c_uint) << 9 as libc::c_int
                                       != 0 {
                                    touch.precision = false_0
                                }
                                result = true_0
                            }
                            // handle wheel end
                            if (*button).type_0 as libc::c_uint ==
                                   touch_wheel as libc::c_int as libc::c_uint
                               {
                                if touch.wheel_count != 0 {
                                    if (*button).flags as libc::c_uint &
                                           (1 as libc::c_uint) <<
                                               10 as libc::c_int != 0 {
                                        Cbuf_AddFilteredText(touch.wheel_end.as_mut_ptr());
                                    } else {
                                        Cbuf_AddText(touch.wheel_end.as_mut_ptr());
                                    }
                                }
                                // disable precision mode
                                if (*button).flags as libc::c_uint &
                                       (1 as libc::c_uint) << 9 as libc::c_int
                                       != 0 {
                                    touch.precision = false_0
                                }
                                touch.wheel_finger = -(1 as libc::c_int);
                                result = true_0
                            }
                            // release motion buttons
                            if (*button).type_0 as libc::c_uint ==
                                   touch_move as libc::c_int as libc::c_uint
                                   ||
                                   (*button).type_0 as libc::c_uint ==
                                       touch_joy as libc::c_int as
                                           libc::c_uint ||
                                   (*button).type_0 as libc::c_uint ==
                                       touch_dpad as libc::c_int as
                                           libc::c_uint {
                                touch.move_finger = -(1 as libc::c_int);
                                touch.side =
                                    0 as libc::c_int as libc::c_float;
                                touch.forward = touch.side;
                                touch.move_button = 0 as *mut touch_button_t
                            }
                            // release look buttons
                            if (*button).type_0 as libc::c_uint ==
                                   touch_look as libc::c_int as libc::c_uint {
                                touch.look_finger = -(1 as libc::c_int)
                            }
                        }
                    }
                }
            }
        }
        button = (*button).prev
    }
    return result;
}
unsafe extern "C" fn Touch_ButtonEdit(mut type_0: touchEventType,
                                      mut fingerID: libc::c_int,
                                      mut x: libc::c_float,
                                      mut y: libc::c_float,
                                      mut dx: libc::c_float,
                                      mut dy: libc::c_float) -> qboolean {
    let mut button: *mut touch_button_t = 0 as *mut touch_button_t;
    // edit buttons are on y1
    if type_0 as libc::c_uint == event_down as libc::c_int as libc::c_uint {
        if x <
               1.0f32 /
                   (*touch_grid_count).value as libc::c_int as libc::c_float
               &&
               y <
                   refState.width as libc::c_float /
                       refState.height as libc::c_float /
                       (*touch_grid_count).value as libc::c_int as
                           libc::c_float {
            touch.showeditbuttons =
                ::std::mem::transmute::<libc::c_uint,
                                        qboolean>(touch.showeditbuttons as
                                                      libc::c_uint ^
                                                      true_0 as libc::c_int as
                                                          libc::c_uint);
            return true_0
        }
        if touch.showeditbuttons as libc::c_uint != 0 &&
               Touch_ButtonPress(&mut touch.list_edit, type_0, fingerID, x, y,
                                 dx, dy) as libc::c_uint != 0 {
            return true_0
        }
    }
    let mut current_block_26: u64;
    // run from end(front) to start(back)
    button = touch.list_user.last;
    while !button.is_null() {
        if type_0 as libc::c_uint == event_down as libc::c_int as libc::c_uint
           {
            if x > (*button).x1 && x < (*button).x2 &&
                   (y < (*button).y2 && y > (*button).y1) {
                (*button).finger = fingerID;
                // do not edit NOEDIT buttons
                if (*button).flags as libc::c_uint &
                       (1 as libc::c_uint) << 1 as libc::c_int != 0 {
                    current_block_26 = 14523784380283086299;
                } else {
                    touch.edit = button;
                    touch.selection = 0 as *mut touch_button_t;
                    // make button last to bring it up
                    if !(*button).next.is_null() &&
                           (*button).type_0 as libc::c_uint ==
                               touch_command as libc::c_int as libc::c_uint {
                        if !(*button).prev.is_null() {
                            (*(*button).prev).next = (*button).next
                        } else { touch.list_user.first = (*button).next }
                        (*(*button).next).prev = (*button).prev;
                        (*touch.list_user.last).next = button;
                        (*button).prev = touch.list_user.last;
                        (*button).next = 0 as *mut touch_button_s;
                        touch.list_user.last = button
                    }
                    touch.state = state_edit_move;
                    return true_0
                }
            } else { current_block_26 = 18386322304582297246; }
        } else { current_block_26 = 18386322304582297246; }
        match current_block_26 {
            18386322304582297246 => {
                if type_0 as libc::c_uint ==
                       event_up as libc::c_int as libc::c_uint {
                    if fingerID == (*button).finger {
                        (*button).finger = -(1 as libc::c_int)
                    }
                }
            }
            _ => { }
        }
        button = (*button).prev
    }
    if type_0 as libc::c_uint == event_down as libc::c_int as libc::c_uint {
        touch.selection = 0 as *mut touch_button_t;
        (*touch.hidebutton).flags =
            ((*touch.hidebutton).flags as libc::c_uint |
                 (1 as libc::c_uint) << 0 as libc::c_int) as libc::c_int
    }
    return false_0;
}
unsafe extern "C" fn Touch_ControlsEvent(mut type_0: touchEventType,
                                         mut fingerID: libc::c_int,
                                         mut x: libc::c_float,
                                         mut y: libc::c_float,
                                         mut dx: libc::c_float,
                                         mut dy: libc::c_float)
 -> libc::c_int {
    if touch.state as libc::c_uint ==
           state_edit_move as libc::c_int as libc::c_uint {
        Touch_EditMove(type_0, fingerID, x, y, dx, dy);
        return 1 as libc::c_int
    }
    if touch.state as libc::c_uint ==
           state_edit as libc::c_int as libc::c_uint &&
           Touch_ButtonEdit(type_0, fingerID, x, y, dx, dy) as libc::c_uint !=
               0 {
        return true_0 as libc::c_int
    }
    if Touch_ButtonPress(&mut touch.list_user, type_0, fingerID, x, y, dx, dy)
           as u64 != 0 {
        return true_0 as libc::c_int
    }
    if type_0 as libc::c_uint == event_motion as libc::c_int as libc::c_uint {
        Touch_Motion(type_0, fingerID, x, y, dx, dy);
    }
    return true_0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn IN_TouchEvent(mut type_0: touchEventType,
                                       mut fingerID: libc::c_int,
                                       mut x: libc::c_float,
                                       mut y: libc::c_float,
                                       mut dx: libc::c_float,
                                       mut dy: libc::c_float) -> libc::c_int {
    // simulate menu mouse click
    if cls.key_dest as libc::c_uint != key_game as libc::c_int as libc::c_uint
           &&
           (if !touch_in_menu.is_null() && (*touch_in_menu).value != 0.0f32 {
                true_0 as libc::c_int
            } else { false_0 as libc::c_int }) == 0 {
        touch.look_finger = -(1 as libc::c_int);
        touch.resize_finger = touch.look_finger;
        touch.move_finger = touch.resize_finger;
        // Hack for keyboard, hope it help
        if cls.key_dest as libc::c_uint ==
               key_console as libc::c_int as libc::c_uint ||
               cls.key_dest as libc::c_uint ==
                   key_message as libc::c_int as libc::c_uint {
            Key_EnableTextInput(true_0, true_0);
            if cls.key_dest as libc::c_uint ==
                   key_console as libc::c_int as libc::c_uint {
                static mut y1: libc::c_float =
                    0 as libc::c_int as libc::c_float;
                y1 += dy;
                if dy > 0.4f32 { Con_Bottom(); }
                if y1 > 0.01f32 {
                    Con_PageUp(1 as libc::c_int);
                    y1 = 0 as libc::c_int as libc::c_float
                }
                if y1 < -0.01f32 {
                    Con_PageDown(1 as libc::c_int);
                    y1 = 0 as libc::c_int as libc::c_float
                }
            }
            // exit of console area
            if type_0 as libc::c_uint ==
                   event_down as libc::c_int as libc::c_uint && x < 0.1f32 &&
                   y > 0.9f32 {
                Cbuf_AddText(b"escape\n\x00" as *const u8 as
                                 *const libc::c_char);
            }
        }
        UI_MouseMove((refState.width as libc::c_float * x) as libc::c_int,
                     (refState.height as libc::c_float * y) as libc::c_int);
        //MsgDev( D_NOTE, "touch %d %d\n", TO_SCRN_X(x), TO_SCRN_Y(y) );
        if type_0 as libc::c_uint == event_down as libc::c_int as libc::c_uint
           {
            Key_Event(241 as libc::c_int, true_0 as libc::c_int);
        }
        if type_0 as libc::c_uint == event_up as libc::c_int as libc::c_uint {
            Key_Event(241 as libc::c_int, false_0 as libc::c_int);
        }
        return 0 as libc::c_int
    }
    if VGui_IsActive() as u64 != 0 {
        VGui_MouseMove((refState.width as libc::c_float * x) as libc::c_int,
                       (refState.height as libc::c_float * y) as libc::c_int);
        if type_0 as libc::c_uint !=
               event_motion as libc::c_int as libc::c_uint {
            VGui_KeyEvent(241 as libc::c_int,
                          if type_0 as libc::c_uint ==
                                 event_down as libc::c_int as libc::c_uint {
                              1 as libc::c_int
                          } else { 0 as libc::c_int });
        }
        // allow scoreboard scroll
        if host.mouse_visible as libc::c_uint != 0 &&
               type_0 as libc::c_uint ==
                   event_motion as libc::c_int as libc::c_uint {
            return 0 as libc::c_int
        }
    }
    if touch.initialized as u64 == 0 ||
           (if !touch_enable.is_null() && (*touch_enable).value != 0.0f32 {
                true_0 as libc::c_int
            } else { false_0 as libc::c_int }) == 0 &&
               touch.clientonly as u64 == 0 {
        return 0 as libc::c_int
    }
    if clgame.dllFuncs.pfnTouchEvent.is_some() &&
           clgame.dllFuncs.pfnTouchEvent.expect("non-null function pointer")(type_0
                                                                                 as
                                                                                 libc::c_int,
                                                                             fingerID,
                                                                             x,
                                                                             y,
                                                                             dx,
                                                                             dy)
               != 0 {
        return true_0 as libc::c_int
    }
    return Touch_ControlsEvent(type_0, fingerID, x, y, dx, dy);
}
#[no_mangle]
pub unsafe extern "C" fn Touch_GetMove(mut forward: *mut libc::c_float,
                                       mut side: *mut libc::c_float,
                                       mut yaw: *mut libc::c_float,
                                       mut pitch: *mut libc::c_float) {
    *forward += touch.forward;
    *side += touch.side;
    *yaw += touch.yaw;
    *pitch += touch.pitch;
    touch.pitch = 0 as libc::c_int as libc::c_float;
    touch.yaw = touch.pitch;
}
#[no_mangle]
pub unsafe extern "C" fn Touch_KeyEvent(mut key: libc::c_int,
                                        mut down: libc::c_int) {
    let mut xi: libc::c_int = 0;
    let mut yi: libc::c_int = 0;
    let mut x: libc::c_float = 0.;
    let mut y: libc::c_float = 0.;
    static mut lx: libc::c_float = 0.;
    static mut ly: libc::c_float = 0.;
    if if !touch_emulate.is_null() && (*touch_emulate).value != 0.0f32 {
           true_0 as libc::c_int
       } else { false_0 as libc::c_int } == 0 {
        if if !touch_enable.is_null() && (*touch_enable).value != 0.0f32 {
               true_0 as libc::c_int
           } else { false_0 as libc::c_int } != 0 {
            return
        }
        if touch.clientonly as u64 == 0 { return }
    }
    Platform_GetMousePos(&mut xi, &mut yi);
    x = xi as libc::c_float / refState.width as libc::c_float;
    y = yi as libc::c_float / refState.height as libc::c_float;
    if cls.key_dest as libc::c_uint == key_menu as libc::c_int as libc::c_uint
           && down < 2 as libc::c_int && key == 241 as libc::c_int {
        UI_MouseMove(xi, yi);
        UI_KeyEvent(key, down as qboolean);
    }
    if down == 1 as libc::c_int {
        Touch_ControlsEvent(event_down,
                            if key == 241 as libc::c_int {
                                0 as libc::c_int
                            } else { 1 as libc::c_int }, x, y,
                            0 as libc::c_int as libc::c_float,
                            0 as libc::c_int as libc::c_float);
    } else {
        Touch_ControlsEvent(if down != 0 {
                                event_motion as libc::c_int
                            } else { event_up as libc::c_int } as
                                touchEventType,
                            if key == 241 as libc::c_int {
                                0 as libc::c_int
                            } else { 1 as libc::c_int }, x, y, x - lx,
                            y - ly);
    }
    lx = x;
    ly = y;
}
#[no_mangle]
pub unsafe extern "C" fn Touch_Shutdown() {
    if touch.initialized as u64 == 0 { return }
    Touch_RemoveAll_f();
    Cmd_RemoveCommand(b"touch_addbutton\x00" as *const u8 as
                          *const libc::c_char);
    Cmd_RemoveCommand(b"touch_removebutton\x00" as *const u8 as
                          *const libc::c_char);
    Cmd_RemoveCommand(b"touch_enableedit\x00" as *const u8 as
                          *const libc::c_char);
    Cmd_RemoveCommand(b"touch_disableedit\x00" as *const u8 as
                          *const libc::c_char);
    Cmd_RemoveCommand(b"touch_settexture\x00" as *const u8 as
                          *const libc::c_char);
    Cmd_RemoveCommand(b"touch_setcolor\x00" as *const u8 as
                          *const libc::c_char);
    Cmd_RemoveCommand(b"touch_setcommand\x00" as *const u8 as
                          *const libc::c_char);
    Cmd_RemoveCommand(b"touch_setflags\x00" as *const u8 as
                          *const libc::c_char);
    Cmd_RemoveCommand(b"touch_show\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"touch_hide\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"touch_list\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"touch_removeall\x00" as *const u8 as
                          *const libc::c_char);
    Cmd_RemoveCommand(b"touch_loaddefaults\x00" as *const u8 as
                          *const libc::c_char);
    Cmd_RemoveCommand(b"touch_roundall\x00" as *const u8 as
                          *const libc::c_char);
    Cmd_RemoveCommand(b"touch_exportconfig\x00" as *const u8 as
                          *const libc::c_char);
    Cmd_RemoveCommand(b"touch_set_stroke\x00" as *const u8 as
                          *const libc::c_char);
    Cmd_RemoveCommand(b"touch_setclientonly\x00" as *const u8 as
                          *const libc::c_char);
    Cmd_RemoveCommand(b"touch_reloadconfig\x00" as *const u8 as
                          *const libc::c_char);
    Cmd_RemoveCommand(b"touch_writeconfig\x00" as *const u8 as
                          *const libc::c_char);
    Cmd_RemoveCommand(b"touch_generate_code\x00" as *const u8 as
                          *const libc::c_char);
    touch.initialized = false_0;
    _Mem_FreePool(&mut touch.mempool,
                  b"../engine/client/in_touch.c\x00" as *const u8 as
                      *const libc::c_char, 2116 as libc::c_int);
}
