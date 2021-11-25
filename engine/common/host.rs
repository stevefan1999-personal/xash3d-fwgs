#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, const_raw_ptr_to_usize_cast, const_transmute,
           extern_types, register_tool)]
extern "C" {
    pub type SDL_Window;
    pub type file_s;
    pub type grasshdr_s;
    pub type engine_studio_api_s;
    pub type r_studio_interface_s;
    pub type IVoiceTweak_s;
    pub type demo_api_s;
    #[no_mangle]
    fn SDL_free(mem: *mut libc::c_void);
    #[no_mangle]
    fn SDL_GetError() -> *const libc::c_char;
    #[no_mangle]
    fn SDL_GetBasePath() -> *mut libc::c_char;
    #[no_mangle]
    fn SDL_ShowCursor(toggle: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn SDL_MinimizeWindow(window: *mut SDL_Window);
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char)
     -> libc::c_double;
    #[no_mangle]
    static mut gl_vsync: *mut convar_t;
    #[no_mangle]
    static mut vid_displayfrequency: *mut convar_t;
    #[no_mangle]
    fn Cbuf_Clear();
    #[no_mangle]
    fn Cbuf_AddText(text: *const libc::c_char);
    #[no_mangle]
    fn Cbuf_InsertText(text: *const libc::c_char);
    #[no_mangle]
    fn Cbuf_ExecStuffCmds();
    #[no_mangle]
    fn Cbuf_Execute();
    #[no_mangle]
    fn Cmd_CurrentCommandIsPrivileged() -> qboolean;
    #[no_mangle]
    fn Cmd_Argc() -> libc::c_int;
    #[no_mangle]
    fn Cmd_Argv(arg: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn Cmd_Init();
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
    fn Memory_Init();
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
    fn Mem_PrintList(minallocationsize: size_t);
    #[no_mangle]
    fn Mem_PrintStats();
    #[no_mangle]
    fn FS_Init();
    #[no_mangle]
    fn FS_Shutdown();
    #[no_mangle]
    fn FS_LoadGameInfo(rootfolder: *const libc::c_char);
    #[no_mangle]
    fn FS_LoadFile(path: *const libc::c_char, filesizeptr: *mut fs_offset_t,
                   gamedironly: qboolean) -> *mut byte;
    #[no_mangle]
    fn FS_Search(pattern: *const libc::c_char, caseinsensitive: libc::c_int,
                 gamedironly: libc::c_int) -> *mut search_t;
    #[no_mangle]
    fn FS_FileExists(filename: *const libc::c_char, gamedironly: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn FS_SetCurrentDirectory(path: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn Image_Init();
    #[no_mangle]
    fn Image_Shutdown();
    #[no_mangle]
    fn Sound_Init();
    #[no_mangle]
    fn Sound_Shutdown();
    #[no_mangle]
    fn Q_buildnum() -> libc::c_int;
    #[no_mangle]
    fn Q_buildnum_compat() -> libc::c_int;
    #[no_mangle]
    fn Q_buildos() -> *const libc::c_char;
    #[no_mangle]
    fn Q_buildarch() -> *const libc::c_char;
    #[no_mangle]
    fn Q_buildcommit() -> *const libc::c_char;
    #[no_mangle]
    fn Host_WriteConfig();
    #[no_mangle]
    fn COM_InitHostState();
    #[no_mangle]
    fn COM_Frame(time: libc::c_float);
    #[no_mangle]
    fn CL_Init();
    #[no_mangle]
    fn CL_Shutdown();
    #[no_mangle]
    fn Host_ClientBegin();
    #[no_mangle]
    fn Host_ClientFrame();
    #[no_mangle]
    fn SV_Init();
    #[no_mangle]
    fn SV_Shutdown(finalmsg: *const libc::c_char);
    #[no_mangle]
    fn SV_ShutdownFilter();
    #[no_mangle]
    fn Host_ServerFrame();
    #[no_mangle]
    fn SV_Active() -> qboolean;
    #[no_mangle]
    fn Con_Reportf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_DPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn HPAK_Init();
    #[no_mangle]
    fn HPAK_FlushHostQueue();
    #[no_mangle]
    fn CL_IsInGame() -> qboolean;
    #[no_mangle]
    fn CL_IsInMenu() -> qboolean;
    #[no_mangle]
    fn CL_Initialized() -> qboolean;
    #[no_mangle]
    fn CL_GetDemoFramerate() -> libc::c_double;
    #[no_mangle]
    fn CL_GetMaxClients() -> libc::c_int;
    #[no_mangle]
    fn SV_GetMaxClients() -> libc::c_int;
    #[no_mangle]
    fn CL_IsRecordDemo() -> qboolean;
    #[no_mangle]
    fn CL_IsTimeDemo() -> qboolean;
    #[no_mangle]
    fn CL_IsPlaybackDemo() -> qboolean;
    #[no_mangle]
    fn SV_Initialized() -> qboolean;
    #[no_mangle]
    fn CL_WriteMessageHistory();
    #[no_mangle]
    fn CL_Disconnect();
    #[no_mangle]
    fn CL_ClearEdicts();
    #[no_mangle]
    fn CL_Drop();
    #[no_mangle]
    fn Con_Init();
    #[no_mangle]
    fn SCR_CheckStartupVids();
    #[no_mangle]
    fn VID_InitDefaultResolution();
    #[no_mangle]
    fn UI_SetActiveMenu(fActive: qboolean);
    #[no_mangle]
    fn Cvar_VariableString(var_name: *const libc::c_char)
     -> *const libc::c_char;
    #[no_mangle]
    fn Cvar_SetValue(var_name: *const libc::c_char, value: libc::c_float);
    #[no_mangle]
    fn Cvar_DirectSet(var: *mut convar_t, value: *const libc::c_char);
    #[no_mangle]
    fn Cvar_Get(var_name: *const libc::c_char, value: *const libc::c_char,
                flags: libc::c_int, description: *const libc::c_char)
     -> *mut convar_t;
    #[no_mangle]
    fn Cvar_RegisterVariable(var: *mut convar_t);
    #[no_mangle]
    fn NET_Init();
    #[no_mangle]
    fn NET_Shutdown();
    #[no_mangle]
    fn HTTP_Shutdown();
    #[no_mangle]
    fn HTTP_Init();
    #[no_mangle]
    fn HTTP_Run();
    #[no_mangle]
    fn Sys_Input() -> *mut libc::c_char;
    #[no_mangle]
    fn Sys_Quit() -> !;
    #[no_mangle]
    fn Sys_CloseLog();
    #[no_mangle]
    fn Sys_InitLog();
    #[no_mangle]
    fn _Sys_GetParmFromCmdLine(parm: *const libc::c_char,
                               out: *mut libc::c_char, size: size_t)
     -> qboolean;
    #[no_mangle]
    fn Sys_RestoreCrashHandler();
    #[no_mangle]
    fn Sys_SetupCrashHandler();
    #[no_mangle]
    fn Sys_MergeCommandLine();
    #[no_mangle]
    fn Sys_ParseCommandLine(argc: libc::c_int, argv: *mut *mut libc::c_char);
    #[no_mangle]
    fn Sys_Error(error: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Sys_CheckParm(parm: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn Sys_DoubleTime() -> libc::c_double;
    #[no_mangle]
    fn Sys_Sleep(msec: libc::c_int);
    #[no_mangle]
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    #[no_mangle]
    fn ID_Init();
    #[no_mangle]
    fn NET_InitMasters();
    #[no_mangle]
    fn Platform_MessageBox(title: *const libc::c_char,
                           message: *const libc::c_char,
                           parentMainWindow: qboolean);
    #[no_mangle]
    fn Platform_Shutdown();
    #[no_mangle]
    fn Platform_Init();
    #[no_mangle]
    fn Cvar_Init();
    #[no_mangle]
    fn Q_sprintf(buffer: *mut libc::c_char, format: *const libc::c_char,
                 _: ...) -> libc::c_int;
    #[no_mangle]
    fn Q_strlen(string: *const libc::c_char) -> size_t;
    #[no_mangle]
    fn Q_strncat(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Q_isdigit(str: *const libc::c_char) -> qboolean;
    #[no_mangle]
    fn Q_atoi(str: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn Q_strnicmp(s1: *const libc::c_char, s2: *const libc::c_char,
                  n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Q_strncmp(s1: *const libc::c_char, s2: *const libc::c_char,
                 n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Q_vsnprintf(buffer: *mut libc::c_char, buffersize: size_t,
                   format: *const libc::c_char, args: ::std::ffi::VaList)
     -> libc::c_int;
    #[no_mangle]
    fn Q_snprintf(buffer: *mut libc::c_char, buffersize: size_t,
                  format: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn COM_FileBase(in_0: *const libc::c_char, out: *mut libc::c_char);
    #[no_mangle]
    fn COM_DefaultExtension(path: *mut libc::c_char,
                            extension: *const libc::c_char);
    #[no_mangle]
    fn BaseCmd_Init();
    #[no_mangle]
    static mut clgame: clgame_static_t;
    #[no_mangle]
    fn Key_Init();
    #[no_mangle]
    fn Key_SetKeyDest(key_dest: libc::c_int);
    #[no_mangle]
    fn Mod_FreeAll();
    #[no_mangle]
    fn Mod_Init();
    #[no_mangle]
    fn Netchan_Init();
    #[no_mangle]
    fn Mod_Shutdown();
    #[no_mangle]
    fn Netchan_Shutdown();
    #[no_mangle]
    static mut cls: client_static_t;
    #[no_mangle]
    fn IN_Init();
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
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type off_t = __off_t;
pub type size_t = libc::c_ulong;
pub type uint = libc::c_uint;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type va_list = __builtin_va_list;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type HANDLE = *mut libc::c_void;
pub type HINSTANCE = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagPOINT {
    pub x: libc::c_int,
    pub y: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct search_t {
    pub numfilenames: libc::c_int,
    pub filenames: *mut *mut libc::c_char,
    pub filenamesbuffer: *mut libc::c_char,
}
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
pub type xcommand_t = Option<unsafe extern "C" fn() -> ()>;
pub type pfnChangeGame
    =
    Option<unsafe extern "C" fn(_: *const libc::c_char) -> ()>;
pub const DEMO_QUAKE1: C2RustUnnamed_1 = 2;
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
pub struct cmdalias_s {
    pub next: *mut cmdalias_s,
    pub name: [libc::c_char; 32],
    pub value: *mut libc::c_char,
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
pub type C2RustUnnamed_1 = libc::c_uint;
pub const DEMO_XASH3D: C2RustUnnamed_1 = 1;
pub const DEMO_INACTIVE: C2RustUnnamed_1 = 0;
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
/*
host.c - dedicated and normal host
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
// XASH_SDL
// va_args
// errno
// strerror
// fork
// decallist_t
#[no_mangle]
pub static mut pChangeGame: pfnChangeGame = None;
#[no_mangle]
pub static mut host: host_parm_t =
    host_parm_t{hInst: 0 as *const libc::c_void as *mut libc::c_void,
                hMutex: 0 as *const libc::c_void as *mut libc::c_void,
                status: HOST_INIT,
                game:
                    game_status_t{curstate: STATE_RUNFRAME,
                                  nextstate: STATE_RUNFRAME,
                                  levelName: [0; 64],
                                  landmarkName: [0; 64],
                                  backgroundMap: false_0,
                                  loadGame: false_0,
                                  newGame: false_0,},
                type_0: 0,
                abortframe:
                    [__jmp_buf_tag{__jmpbuf: [0; 8],
                                   __mask_was_saved: 0,
                                   __saved_mask:
                                       __sigset_t{__val: [0; 16],},}; 1],
                errorframe: 0,
                mempool: 0,
                finalmsg: [0; 256],
                downloadfile: [0; 256],
                downloadcount: 0,
                deferred_cmd: [0; 128],
                rd:
                    host_redirect_t{target: RD_NONE,
                                    buffer:
                                        0 as *const libc::c_char as
                                            *mut libc::c_char,
                                    buffersize: 0,
                                    address:
                                        netadr_t{type_0: NA_UNUSED,
                                                 ip: [0; 4],
                                                 ipx: [0; 10],
                                                 port: 0,},
                                    flush: None,
                                    lines: 0,},
                argc: 0,
                argv: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
                realtime: 0.,
                frametime: 0.,
                realframetime: 0.,
                framecount: 0,
                draw_decals: [[0; 64]; 512],
                player_mins: [[0.; 3]; 4],
                player_maxs: [[0.; 3]; 4],
                hWnd: 0 as *const libc::c_void as *mut libc::c_void,
                allow_console: false_0,
                allow_console_init: false_0,
                key_overstrike: false_0,
                stuffcmds_pending: false_0,
                allow_cheats: false_0,
                con_showalways: false_0,
                change_game: false_0,
                mouse_visible: false_0,
                shutdown_issued: false_0,
                force_draw_version: false_0,
                force_draw_version_time: 0.,
                apply_game_config: false_0,
                apply_opengl_config: false_0,
                config_executed: false_0,
                sv_cvars_restored: 0,
                crashed: false_0,
                daemonized: false_0,
                enabledll: false_0,
                textmode: false_0,
                userinfo_changed: false_0,
                movevars_changed: false_0,
                renderinfo_changed: false_0,
                rootdir: [0; 260],
                rodir: [0; 260],
                gamefolder: [0; 64],
                imagepool: 0,
                soundpool: 0,
                features: 0,
                window_center_x: 0,
                window_center_y: 0,
                decalList: 0 as *const decallist_s as *mut decallist_s,
                numdecals: 0,};
// host parms
#[no_mangle]
pub static mut SI: sysinfo_t =
    sysinfo_t{exeName: [0; 256],
              rcName: [0; 256],
              basedirName: [0; 256],
              gamedll: [0; 256],
              clientlib: [0; 256],
              GameInfo: 0 as *const gameinfo_t as *mut gameinfo_t,
              games: [0 as *const gameinfo_t as *mut gameinfo_t; 512],
              numgames: 0,};
#[no_mangle]
pub static mut host_developer: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"developer\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"0\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: (1 as libc::c_int) << 11 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"engine is in development-mode\x00" as *const u8
                                 as *const libc::c_char as *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut sys_ticrate: convar_t =
    unsafe {
        {
            let mut init =
                convar_s{name:
                             b"sys_ticrate\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                         string:
                             b"100\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                         flags: 0 as libc::c_int,
                         value: 0.0f32,
                         next:
                             0xdeadbeefdeadbeef as libc::c_ulong as
                                 *mut libc::c_void as *mut convar_s,
                         desc:
                             b"framerate in dedicated mode\x00" as *const u8
                                 as *const libc::c_char as *mut libc::c_char,
                         def_string:
                             0 as *const libc::c_char as *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
pub static mut host_serverstate: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut host_gameloaded: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut host_clientloaded: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut host_limitlocal: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut host_maxfps: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut host_framerate: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut host_sleeptime: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut con_gamemaps: *mut convar_t =
    0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut build: *mut convar_t = 0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub static mut ver: *mut convar_t = 0 as *const convar_t as *mut convar_t;
#[no_mangle]
pub unsafe extern "C" fn Sys_PrintUsage() {
    let mut usage_str: *const libc::c_char = 0 as *const libc::c_char;
    usage_str =
        b"Usage:\n   <xash> [options] [+command1] [+command2 arg]  \nOptions:\n   -dev [level]       set log verbosity 0-2\n   -log               write log to \"engine.log\"\n   -nowriteconfig     disable config save\n   -casesensitive     disable case-insensitive FS emulation\n   -daemonize         run engine in background, dedicated only\n   -toconsole         run engine witn console open\n   -width <n>         set window width\n   -height <n>        set window height\n   -oldfont           enable unused Quake font in Half-Life\n   -fullscreen        run engine in fullscreen mode\n   -windowed          run engine in windowed mode\n   -dedicated         run engine in dedicated server mode\n   -nojoy             disable joystick support\n   -sdl_joy_old_api   use SDL legacy joystick API\n   -sdl_renderer <n>  use alternative SDL_Renderer for software\n   -nosound           disable sound\n   -noenginemouse     disable mouse completely\n   -ref <name>        use selected renderer dll\n   -gldebug           enable OpenGL debug log\n   -noip              disable TCP/IP\n   -noch              disable crashhandler\n   -disablehelp       disable this message\n   -dll <path>        override server DLL path\n   -clientlib <path>  override client DLL path\n   -rodir <path>      set read-only base directory, experimental\n   -ip <ip>           set custom ip\n   -port <port>       set custom host port\n   -clockwindow <cw>  adjust clockwindow\n\x00"
            as *const u8 as *const libc::c_char;
    Sys_Error(b"%s\x00" as *const u8 as *const libc::c_char, usage_str);
}
#[no_mangle]
pub unsafe extern "C" fn Host_CompareFileTime(mut ft1: libc::c_int,
                                              mut ft2: libc::c_int)
 -> libc::c_int {
    if ft1 < ft2 {
        return -(1 as libc::c_int)
    } else { if ft1 > ft2 { return 1 as libc::c_int } }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Host_ShutdownServer() {
    SV_Shutdown(b"Server was killed\n\x00" as *const u8 as
                    *const libc::c_char);
}
/*
================
Host_PrintEngineFeatures
================
*/
#[no_mangle]
pub unsafe extern "C" fn Host_PrintEngineFeatures() {
    if host.features &
           ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint != 0 {
        Con_Reportf(b"^3EXT:^7 big world support enabled\n\x00" as *const u8
                        as *const libc::c_char);
    }
    if host.features &
           ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint != 0 {
        Con_Reportf(b"^3EXT:^7 deluxemap support enabled\n\x00" as *const u8
                        as *const libc::c_char);
    }
    if host.features &
           ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint != 0 {
        Con_Reportf(b"^3EXT:^7 Improved MOVETYPE_PUSH is used\n\x00" as
                        *const u8 as *const libc::c_char);
    }
    if host.features &
           ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_uint != 0 {
        Con_Reportf(b"^3EXT:^7 Large lightmaps enabled\n\x00" as *const u8 as
                        *const libc::c_char);
    }
    if host.features &
           ((1 as libc::c_int) << 5 as libc::c_int) as libc::c_uint != 0 {
        Con_Reportf(b"^3EXT:^7 Compensate quake bug enabled\n\x00" as
                        *const u8 as *const libc::c_char);
    };
}
/*
==============
Host_IsQuakeCompatible

==============
*/
#[no_mangle]
pub unsafe extern "C" fn Host_IsQuakeCompatible() -> qboolean {
    // feature set
    if host.features &
           ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint != 0 {
        return true_0
    }
    // quake demo playing
    if cls.demoplayback == DEMO_QUAKE1 as libc::c_int { return true_0 }
    // XASH_DEDICATED
    return false_0;
}
/*
================
Host_EndGame
================
*/
#[no_mangle]
pub unsafe extern "C" fn Host_EndGame(mut abort: qboolean,
                                      mut message: *const libc::c_char,
                                      mut args: ...) {
    let mut argptr: ::std::ffi::VaListImpl;
    static mut string: [libc::c_char; 1024] = [0; 1024];
    argptr = args.clone();
    Q_vsnprintf(string.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as
                    libc::c_ulong, message, argptr.as_va_list());
    Con_Printf(b"Host_EndGame: %s\n\x00" as *const u8 as *const libc::c_char,
               string.as_mut_ptr());
    SV_Shutdown(b"\n\x00" as *const u8 as *const libc::c_char);
    CL_Disconnect();
    // recreate world if needs
    CL_ClearEdicts();
    // release all models
    Mod_FreeAll();
    if abort as u64 != 0 { Host_AbortCurrentFrame(); };
}
/*
================
Host_AbortCurrentFrame

aborts the current host frame and goes on with the next one
================
*/
#[no_mangle]
pub unsafe extern "C" fn Host_AbortCurrentFrame() {
    longjmp(host.abortframe.as_mut_ptr(), 1 as libc::c_int);
}
/*
==================
Host_CheckSleep
==================
*/
#[no_mangle]
pub unsafe extern "C" fn Host_CheckSleep() {
    let mut sleeptime: libc::c_int = (*host_sleeptime).value as libc::c_int;
    if host.type_0 == HOST_DEDICATED as libc::c_int as libc::c_uint {
        // let the dedicated server some sleep
        Sys_Sleep(sleeptime); // listenserver
    } else if host.status as libc::c_uint ==
                  HOST_NOFOCUS as libc::c_int as libc::c_uint {
        if SV_Active() as libc::c_uint != 0 &&
               CL_IsInGame() as libc::c_uint != 0 {
            Sys_Sleep(sleeptime);
        } else { Sys_Sleep(20 as libc::c_int); }
        // sleep 20 ms otherwise
    } else if host.status as libc::c_uint ==
                  HOST_SLEEP as libc::c_int as libc::c_uint {
        // completely sleep in minimized state
        Sys_Sleep(20 as libc::c_int);
    } else { Sys_Sleep(sleeptime); };
}
#[no_mangle]
pub unsafe extern "C" fn Host_NewInstance(mut name: *const libc::c_char,
                                          mut finalmsg: *const libc::c_char) {
    if pChangeGame.is_none() { return }
    host.change_game = true_0;
    Q_strncpy(host.finalmsg.as_mut_ptr(), finalmsg,
              ::std::mem::size_of::<string>() as libc::c_ulong);
    pChangeGame.expect("non-null function pointer")(name);
    // call from hl.exe
}
/*
=================
Host_ChangeGame_f

Change game modification
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Host_ChangeGame_f() {
    let mut i: libc::c_int = 0;
    if Cmd_Argc() != 2 as libc::c_int {
        Con_Printf(b"Usage: game <directory>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    // validate gamedir
    i = 0 as libc::c_int;
    while i < SI.numgames {
        if Q_strnicmp((*SI.games[i as usize]).gamefolder.as_mut_ptr(),
                      Cmd_Argv(1 as libc::c_int), 99999 as libc::c_int) == 0 {
            break ;
        }
        i += 1
    }
    if i == SI.numgames {
        Con_Printf(b"%s not exist\n\x00" as *const u8 as *const libc::c_char,
                   Cmd_Argv(1 as libc::c_int));
    } else if Q_strnicmp((*SI.GameInfo).gamefolder.as_mut_ptr(),
                         Cmd_Argv(1 as libc::c_int), 99999 as libc::c_int) ==
                  0 {
        Con_Printf(b"%s already active\n\x00" as *const u8 as
                       *const libc::c_char, Cmd_Argv(1 as libc::c_int));
    } else {
        let mut arg1: *const libc::c_char =
            va(b"%s%s\x00" as *const u8 as *const libc::c_char,
               if host.type_0 == HOST_NORMAL as libc::c_int as libc::c_uint {
                   b"\x00" as *const u8 as *const libc::c_char
               } else { b"#\x00" as *const u8 as *const libc::c_char },
               Cmd_Argv(1 as libc::c_int));
        let mut arg2: *const libc::c_char =
            va(b"change game to \'%s\'\x00" as *const u8 as
                   *const libc::c_char,
               (*SI.games[i as usize]).title.as_mut_ptr());
        Host_NewInstance(arg1, arg2);
    };
}
/*
===============
Host_Exec_f
===============
*/
#[no_mangle]
pub unsafe extern "C" fn Host_Exec_f() {
    let mut cfgpath: string = [0; 256];
    let mut f: *mut byte = 0 as *mut byte;
    let mut txt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: fs_offset_t = 0;
    let mut arg: *mut libc::c_char = 0 as *mut libc::c_char;
    if Cmd_Argc() != 2 as libc::c_int {
        Con_Printf(b"Usage: exec <filename>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    arg = Cmd_Argv(1 as libc::c_int) as *mut libc::c_char;
    if Cmd_CurrentCommandIsPrivileged() as u64 == 0 {
        let mut unprivilegedWhitelist: [*const libc::c_char; 12] =
            [0 as *const libc::c_char,
             b"mapdefault.cfg\x00" as *const u8 as *const libc::c_char,
             b"scout.cfg\x00" as *const u8 as *const libc::c_char,
             b"sniper.cfg\x00" as *const u8 as *const libc::c_char,
             b"soldier.cfg\x00" as *const u8 as *const libc::c_char,
             b"demoman.cfg\x00" as *const u8 as *const libc::c_char,
             b"medic.cfg\x00" as *const u8 as *const libc::c_char,
             b"hwguy.cfg\x00" as *const u8 as *const libc::c_char,
             b"pyro.cfg\x00" as *const u8 as *const libc::c_char,
             b"spy.cfg\x00" as *const u8 as *const libc::c_char,
             b"engineer.cfg\x00" as *const u8 as *const libc::c_char,
             b"civilian.cfg\x00" as *const u8 as *const libc::c_char];
        let mut i: libc::c_int = 0;
        let mut allow: qboolean = false_0;
        unprivilegedWhitelist[0 as libc::c_int as usize] =
            va(b"%s.cfg\x00" as *const u8 as *const libc::c_char,
               clgame.mapname.as_mut_ptr());
        i = 0 as libc::c_int;
        while (i as libc::c_ulong) <
                  (::std::mem::size_of::<[*const libc::c_char; 12]>() as
                       libc::c_ulong).wrapping_div(::std::mem::size_of::<*const libc::c_char>()
                                                       as libc::c_ulong) {
            if Q_strncmp(arg, unprivilegedWhitelist[i as usize],
                         99999 as libc::c_int) == 0 {
                allow = true_0;
                break ;
            } else { i += 1 }
        }
        if allow as u64 == 0 {
            Con_Printf(b"exec %s: not privileged or in whitelist\n\x00" as
                           *const u8 as *const libc::c_char, arg);
            return
        }
    }
    // XASH_DEDICATED
    if Q_strnicmp(b"game.cfg\x00" as *const u8 as *const libc::c_char, arg,
                  99999 as libc::c_int) == 0 {
        // don't execute game.cfg in singleplayer
        if SV_GetMaxClients() == 1 as libc::c_int { return }
    } // append as default
    Q_strncpy(cfgpath.as_mut_ptr(), arg,
              ::std::mem::size_of::<string>() as libc::c_ulong);
    COM_DefaultExtension(cfgpath.as_mut_ptr(),
                         b".cfg\x00" as *const u8 as *const libc::c_char);
    f = FS_LoadFile(cfgpath.as_mut_ptr(), &mut len, false_0);
    if f.is_null() {
        Con_Reportf(b"couldn\'t exec %s\n\x00" as *const u8 as
                        *const libc::c_char, Cmd_Argv(1 as libc::c_int));
        return
    }
    if Q_strnicmp(b"config.cfg\x00" as *const u8 as *const libc::c_char, arg,
                  99999 as libc::c_int) == 0 {
        host.config_executed = true_0
    }
    // adds \n\0 at end of the file
    txt =
        _Mem_Alloc(host.mempool,
                   (len + 2 as libc::c_int as libc::c_long) as size_t, true_0,
                   b"../engine/common/host.c\x00" as *const u8 as
                       *const libc::c_char, 422 as libc::c_int) as
            *mut libc::c_char;
    memcpy(txt as *mut libc::c_void, f as *const libc::c_void,
           len as libc::c_ulong);
    Q_strncat(txt, b"\n\x00" as *const u8 as *const libc::c_char,
              (len + 2 as libc::c_int as libc::c_long) as size_t);
    _Mem_Free(f as *mut libc::c_void,
              b"../engine/common/host.c\x00" as *const u8 as
                  *const libc::c_char, 425 as libc::c_int);
    if host.apply_game_config as u64 == 0 {
        Con_Printf(b"execing %s\n\x00" as *const u8 as *const libc::c_char,
                   arg);
    }
    Cbuf_InsertText(txt);
    _Mem_Free(txt as *mut libc::c_void,
              b"../engine/common/host.c\x00" as *const u8 as
                  *const libc::c_char, 430 as libc::c_int);
}
/*
===============
Host_MemStats_f
===============
*/
#[no_mangle]
pub unsafe extern "C" fn Host_MemStats_f() {
    match Cmd_Argc() {
        1 => {
            Mem_PrintList(((1 as libc::c_int) << 30 as libc::c_int) as
                              size_t);
            Mem_PrintStats();
        }
        2 => {
            Mem_PrintList((Q_atoi(Cmd_Argv(1 as libc::c_int)) *
                               1024 as libc::c_int) as size_t);
            Mem_PrintStats();
        }
        _ => {
            Con_Printf(b"Usage: memlist <all>\n\x00" as *const u8 as
                           *const libc::c_char);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Host_Minimize_f() {
    if !host.hWnd.is_null() {
        SDL_MinimizeWindow(host.hWnd as *mut SDL_Window);
    };
}
/*
=================
Host_IsLocalGame

singleplayer game detect
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Host_IsLocalGame() -> qboolean {
    if SV_Active() as u64 != 0 {
        return if SV_GetMaxClients() == 1 as libc::c_int {
                   true_0 as libc::c_int
               } else { false_0 as libc::c_int } as qboolean
    } else {
        return if CL_GetMaxClients() == 1 as libc::c_int {
                   true_0 as libc::c_int
               } else { false_0 as libc::c_int } as qboolean
    };
}
#[no_mangle]
pub unsafe extern "C" fn Host_IsLocalClient() -> qboolean {
    // only the local client have the active server
    if CL_Initialized() as libc::c_uint != 0 &&
           SV_Initialized() as libc::c_uint != 0 {
        return true_0
    }
    return false_0;
}
/*
=================
Host_RegisterDecal
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Host_RegisterDecal(mut name: *const libc::c_char,
                                            mut count: *mut libc::c_int)
 -> qboolean {
    let mut shortname: [libc::c_char; 64] = [0; 64];
    let mut i: libc::c_int = 0;
    if if name.is_null() || *name == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return false_0
    }
    COM_FileBase(name, shortname.as_mut_ptr());
    i = 1 as libc::c_int;
    while i < 512 as libc::c_int &&
              host.draw_decals[i as usize][0 as libc::c_int as usize] as
                  libc::c_int != 0 {
        if Q_strnicmp(host.draw_decals[i as usize].as_mut_ptr(),
                      shortname.as_mut_ptr(), 99999 as libc::c_int) == 0 {
            return true_0
        }
        i += 1
    }
    if i == 512 as libc::c_int {
        Con_DPrintf(b"^1Error:^7 MAX_DECALS limit exceeded (%d)\n\x00" as
                        *const u8 as *const libc::c_char, 512 as libc::c_int);
        return false_0
    }
    // register new decal
    Q_strncpy(host.draw_decals[i as usize].as_mut_ptr(),
              shortname.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    *count += 1 as libc::c_int;
    return true_0;
}
/*
=================
Host_InitDecals
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Host_InitDecals() {
    let mut i: libc::c_int = 0;
    let mut num_decals: libc::c_int = 0 as libc::c_int;
    let mut t: *mut search_t = 0 as *mut search_t;
    // NOTE: only once resource without which engine can't continue work
    if FS_FileExists(b"gfx/conchars\x00" as *const u8 as *const libc::c_char,
                     false_0 as libc::c_int) == 0 {
        Sys_Error(b"W_LoadWadFile: couldn\'t load gfx.wad\n\x00" as *const u8
                      as *const libc::c_char);
    }
    memset(host.draw_decals.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<[[libc::c_char; 64]; 512]>() as
               libc::c_ulong);
    // lookup all the decals in decals.wad (basedir, gamedir, falldir)
    t =
        FS_Search(b"decals.wad/*.*\x00" as *const u8 as *const libc::c_char,
                  true_0 as libc::c_int, false_0 as libc::c_int);
    i = 0 as libc::c_int;
    while !t.is_null() && i < (*t).numfilenames {
        if Host_RegisterDecal(*(*t).filenames.offset(i as isize),
                              &mut num_decals) as u64 == 0 {
            break ;
        }
        i += 1
    }
    if !t.is_null() {
        _Mem_Free(t as *mut libc::c_void,
                  b"../engine/common/host.c\x00" as *const u8 as
                      *const libc::c_char, 549 as libc::c_int);
    }
    Con_Reportf(b"InitDecals: %i decals\n\x00" as *const u8 as
                    *const libc::c_char, num_decals);
}
/*
===================
Host_GetCommands

Add them exactly as if they had been typed at the console
===================
*/
#[no_mangle]
pub unsafe extern "C" fn Host_GetCommands() {
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    loop  {
        cmd = Sys_Input();
        if cmd.is_null() { break ; }
        Cbuf_AddText(cmd);
        Cbuf_Execute();
    };
}
/*
===================
Host_CalcFPS

compute actual FPS for various modes
===================
*/
#[no_mangle]
pub unsafe extern "C" fn Host_CalcFPS() -> libc::c_double {
    let mut fps: libc::c_double = 0.0f64;
    if host.type_0 == HOST_DEDICATED as libc::c_int as libc::c_uint {
        fps = sys_ticrate.value as libc::c_double
    } else if CL_IsPlaybackDemo() as libc::c_uint != 0 ||
                  CL_IsRecordDemo() as libc::c_uint != 0 {
        // NOTE: we should play demos with same fps as it was recorded
        fps = CL_GetDemoFramerate()
    } else if Host_IsLocalGame() as u64 != 0 {
        fps = (*host_maxfps).value as libc::c_double
    } else {
        fps = (*host_maxfps).value as libc::c_double;
        if fps == 0.0f64 { fps = 200.0f32 as libc::c_double }
        fps =
            if fps >= 20.0f32 as libc::c_double {
                if fps < 200.0f32 as libc::c_double {
                    fps
                } else { 200.0f32 as libc::c_double }
            } else { 20.0f32 as libc::c_double }
    }
    // probably left part of this condition is redundant :-)
    if host.type_0 != HOST_DEDICATED as libc::c_int as libc::c_uint &&
           Host_IsLocalGame() as libc::c_uint != 0 &&
           CL_IsTimeDemo() as u64 == 0 {
        // ajdust fps for vertical synchronization
        if if !gl_vsync.is_null() && (*gl_vsync).value != 0.0f32 {
               true_0 as libc::c_int
           } else { false_0 as libc::c_int } != 0 {
            if (*vid_displayfrequency).value != 0.0f32 {
                fps = (*vid_displayfrequency).value as libc::c_double
            } else { fps = 60.0f64 }
            // default
        }
    }
    return fps;
}
/*
===================
Host_FilterTime

Returns false if the time is too short to run a frame
===================
*/
#[no_mangle]
pub unsafe extern "C" fn Host_FilterTime(mut time: libc::c_float)
 -> qboolean {
    static mut oldtime: libc::c_double = 0.;
    let mut fps: libc::c_double = 0.;
    host.realtime += time as libc::c_double;
    fps = Host_CalcFPS();
    // clamp the fps in multiplayer games
    if fps != 0.0f64 {
        // limit fps to withing tolerable range
        fps =
            if fps >= 20.0f32 as libc::c_double {
                if fps < 200.0f32 as libc::c_double {
                    fps
                } else { 200.0f32 as libc::c_double }
            } else { 20.0f32 as libc::c_double };
        if host.type_0 == HOST_DEDICATED as libc::c_int as libc::c_uint {
            if host.realtime - oldtime < 1.0f64 / (fps + 1.0f64) {
                return false_0
            }
        } else if host.realtime - oldtime < 1.0f64 / fps { return false_0 }
    }
    host.frametime = host.realtime - oldtime;
    host.realframetime =
        if host.frametime >= 0.0001f32 as libc::c_double {
            if host.frametime < 0.25f32 as libc::c_double {
                host.frametime
            } else { 0.25f32 as libc::c_double }
        } else { 0.0001f32 as libc::c_double };
    oldtime = host.realtime;
    // NOTE: allow only in singleplayer while demos are not active
    if (*host_framerate).value > 0.0f32 &&
           Host_IsLocalGame() as libc::c_uint != 0 &&
           CL_IsPlaybackDemo() as u64 == 0 && CL_IsRecordDemo() as u64 == 0 {
        host.frametime =
            if (*host_framerate).value >= 0.0001f32 {
                if (*host_framerate).value < 0.25f32 {
                    (*host_framerate).value
                } else { 0.25f32 }
            } else { 0.0001f32 } as libc::c_double
    } else {
        host.frametime =
            if host.frametime >= 0.0001f32 as libc::c_double {
                if host.frametime < 0.25f32 as libc::c_double {
                    host.frametime
                } else { 0.25f32 as libc::c_double }
            } else { 0.0001f32 as libc::c_double }
    }
    return true_0;
}
/*
=================
Host_Frame
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Host_Frame(mut time: libc::c_float) {
    Host_CheckSleep();
    // decide the simulation time
    if Host_FilterTime(time) as u64 == 0 { return } // input frame
    Host_InputFrame(); // begin client
    Host_ClientBegin(); // dedicated in
    Host_GetCommands(); // server frame
    Host_ServerFrame(); // client frame
    Host_ClientFrame(); // both server and client
    HTTP_Run();
    host.framecount = host.framecount.wrapping_add(1);
}
/*
=================
Host_Error
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Host_Error(mut error: *const libc::c_char,
                                    mut args: ...) {
    static mut hosterror1: [libc::c_char; 1024] = [0; 1024];
    static mut hosterror2: [libc::c_char; 1024] = [0; 1024];
    static mut recursive: qboolean = false_0;
    let mut argptr: ::std::ffi::VaListImpl;
    if host.mouse_visible as libc::c_uint != 0 && CL_IsInMenu() as u64 == 0 {
        // hide VGUI mouse
        SDL_ShowCursor(0 as libc::c_int); // before Q_error call
        host.mouse_visible = false_0
    }
    argptr = args.clone();
    Q_vsnprintf(hosterror1.as_mut_ptr(), 99999 as libc::c_int as size_t,
                error, argptr.as_va_list());
    CL_WriteMessageHistory();
    if host.framecount < 3 as libc::c_int as libc::c_uint {
        Sys_Error(b"Host_InitError: %s\x00" as *const u8 as
                      *const libc::c_char, hosterror1.as_mut_ptr());
    } else if host.framecount == host.errorframe {
        Sys_Error(b"Host_MultiError: %s\x00" as *const u8 as
                      *const libc::c_char, hosterror2.as_mut_ptr());
        return
    } else {
        if host.allow_console as u64 != 0 {
            UI_SetActiveMenu(false_0);
            Key_SetKeyDest(key_console as libc::c_int);
            Con_Printf(b"Host_Error: %s\x00" as *const u8 as
                           *const libc::c_char, hosterror1.as_mut_ptr());
        } else {
            Platform_MessageBox(b"Host Error\x00" as *const u8 as
                                    *const libc::c_char,
                                hosterror1.as_mut_ptr(), true_0);
        }
    }
    // host is shutting down. don't invoke infinite loop
    if host.status as libc::c_uint ==
           HOST_SHUTDOWN as libc::c_int as libc::c_uint {
        return
    }
    if recursive as u64 != 0 {
        Con_Printf(b"Host_RecursiveError: %s\x00" as *const u8 as
                       *const libc::c_char, hosterror2.as_mut_ptr());
        Sys_Error(b"%s\x00" as *const u8 as *const libc::c_char,
                  hosterror1.as_mut_ptr());
        return
        // don't multiple executes
    } // to avoid multply calls per frame
    recursive = true_0;
    Q_strncpy(hosterror2.as_mut_ptr(), hosterror1.as_mut_ptr(),
              1024 as libc::c_int as size_t);
    host.errorframe = host.framecount;
    Q_sprintf(host.finalmsg.as_mut_ptr(),
              b"Server crashed: %s\x00" as *const u8 as *const libc::c_char,
              hosterror1.as_mut_ptr());
    // clearing cmd buffer to prevent execute any commands
    COM_InitHostState(); // drop clients
    Cbuf_Clear();
    Host_ShutdownServer();
    CL_Drop();
    // recreate world if needs
    CL_ClearEdicts();
    // release all models
    Mod_FreeAll();
    recursive = false_0;
    Host_AbortCurrentFrame();
}
#[no_mangle]
pub unsafe extern "C" fn Host_Error_f() {
    let mut error: *const libc::c_char = Cmd_Argv(1 as libc::c_int);
    if *error == 0 {
        error = b"Invoked host error\x00" as *const u8 as *const libc::c_char
    }
    Host_Error(b"%s\n\x00" as *const u8 as *const libc::c_char, error);
}
#[no_mangle]
pub unsafe extern "C" fn Sys_Error_f() {
    let mut error: *const libc::c_char = Cmd_Argv(1 as libc::c_int);
    if *error == 0 {
        error = b"Invoked sys error\x00" as *const u8 as *const libc::c_char
    }
    Sys_Error(b"%s\n\x00" as *const u8 as *const libc::c_char, error);
}
/*
=================
Host_Crash_f
=================
*/
unsafe extern "C" fn Host_Crash_f() {
    ::std::ptr::write_volatile(0 as *mut libc::c_int,
                               0xffffffff as libc::c_uint as libc::c_int);
}
/*
=================
Host_Userconfigd_f
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Host_Userconfigd_f() {
    let mut t: *mut search_t = 0 as *mut search_t;
    let mut i: libc::c_int = 0;
    t =
        FS_Search(b"userconfig.d/*.cfg\x00" as *const u8 as
                      *const libc::c_char, true_0 as libc::c_int,
                  false_0 as libc::c_int);
    if t.is_null() { return }
    i = 0 as libc::c_int;
    while i < (*t).numfilenames {
        Cbuf_AddText(va(b"exec %s\n\x00" as *const u8 as *const libc::c_char,
                        *(*t).filenames.offset(i as isize)));
        i += 1
    }
    _Mem_Free(t as *mut libc::c_void,
              b"../engine/common/host.c\x00" as *const u8 as
                  *const libc::c_char, 809 as libc::c_int);
}
/*
=================
Host_InitCommon
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Host_InitCommon(mut argc: libc::c_int,
                                         mut argv: *mut *mut libc::c_char,
                                         mut progname: *const libc::c_char,
                                         mut bChangeGame: qboolean) {
    let mut dev_level: [libc::c_char; 4] = [0; 4];
    let mut developer: libc::c_int = 0 as libc::c_int;
    let mut baseDir: *const libc::c_char = 0 as *const libc::c_char;
    let mut ticrate: [libc::c_char; 16] = [0; 16];
    let mut len: libc::c_int = 0;
    // some commands may turn engine into infinite loop,
	// e.g. xash.exe +game xash -game xash
	// so we clear all cmd_args, but leave dbg states as well
    Sys_ParseCommandLine(argc, argv); // initialzation started
    if Sys_CheckParm(b"-disablehelp\x00" as *const u8 as *const libc::c_char)
           == 0 {
        if Sys_CheckParm(b"-help\x00" as *const u8 as *const libc::c_char) !=
               0 ||
               Sys_CheckParm(b"-h\x00" as *const u8 as *const libc::c_char) !=
                   0 ||
               Sys_CheckParm(b"--help\x00" as *const u8 as
                                 *const libc::c_char) != 0 {
            Sys_PrintUsage(); // init memory subsystem
        }
    }
    if Sys_CheckParm(b"-noch\x00" as *const u8 as *const libc::c_char) == 0 {
        Sys_SetupCrashHandler();
    }
    host.enabledll =
        (Sys_CheckParm(b"-nodll\x00" as *const u8 as *const libc::c_char) ==
             0) as libc::c_int as qboolean;
    host.change_game = bChangeGame;
    host.config_executed = false_0;
    host.status = HOST_INIT;
    Memory_Init();
    host.mempool =
        _Mem_AllocPool(b"Zone Engine\x00" as *const u8 as *const libc::c_char,
                       b"../engine/common/host.c\x00" as *const u8 as
                           *const libc::c_char, 869 as libc::c_int);
    // HACKHACK: Quake console is always allowed
	// TODO: determine if we are running QWrap more reliable
    if Sys_CheckParm(b"-console\x00" as *const u8 as *const libc::c_char) != 0
           ||
           Q_strnicmp(SI.exeName.as_mut_ptr(),
                      b"quake\x00" as *const u8 as *const libc::c_char,
                      99999 as libc::c_int) == 0 {
        host.allow_console = true_0
    }
    if Sys_CheckParm(b"-dev\x00" as *const u8 as *const libc::c_char) != 0 {
        host.allow_console = true_0;
        developer = DEV_NORMAL as libc::c_int;
        if _Sys_GetParmFromCmdLine(b"-dev\x00" as *const u8 as
                                       *const libc::c_char,
                                   dev_level.as_mut_ptr(),
                                   ::std::mem::size_of::<[libc::c_char; 4]>()
                                       as libc::c_ulong) as u64 != 0 {
            if Q_isdigit(dev_level.as_mut_ptr()) as u64 != 0 {
                developer =
                    if abs(Q_atoi(dev_level.as_mut_ptr())) >=
                           DEV_NONE as libc::c_int {
                        if abs(Q_atoi(dev_level.as_mut_ptr())) <
                               DEV_EXTENDED as libc::c_int {
                            abs(Q_atoi(dev_level.as_mut_ptr()))
                        } else { DEV_EXTENDED as libc::c_int }
                    } else { DEV_NONE as libc::c_int }
            }
        }
    }
    host.con_showalways = true_0;
    if Sys_CheckParm(b"-dedicated\x00" as *const u8 as *const libc::c_char) !=
           0 ||
           *progname.offset(0 as libc::c_int as isize) as libc::c_int ==
               '#' as i32 {
        host.type_0 = HOST_DEDICATED as libc::c_int as uint
    } else { host.type_0 = HOST_NORMAL as libc::c_int as uint }
    // set default gamedir
    if *progname.offset(0 as libc::c_int as isize) as libc::c_int ==
           '#' as i32 {
        progname = progname.offset(1)
    }
    Q_strncpy(SI.exeName.as_mut_ptr(), progname,
              ::std::mem::size_of::<string>() as libc::c_ulong);
    Q_strncpy(SI.basedirName.as_mut_ptr(), progname,
              ::std::mem::size_of::<string>() as libc::c_ulong);
    if host.type_0 == HOST_DEDICATED as libc::c_int as libc::c_uint {
        Sys_MergeCommandLine();
        host.allow_console = true_0
    } else if developer <= DEV_NORMAL as libc::c_int {
        host.con_showalways = false_0
    }
    // don't show console as default
    // member console allowing
    host.allow_console_init = host.allow_console;
    // timeBeginPeriod( 1 ); // a1ba: Do we need this?
    // NOTE: this message couldn't be passed into game console but it doesn't matter
//	Con_Reportf( "Sys_LoadLibrary: Loading xash.dll - ok\n" );
    // get default screen res
    VID_InitDefaultResolution();
    // init host state machine
    COM_InitHostState();
    // init hashed commands
    BaseCmd_Init();
    // startup cmds and cvars subsystem
    Cmd_Init();
    Cvar_Init();
    // share developer level across all dlls
    Q_snprintf(dev_level.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong,
               b"%i\x00" as *const u8 as *const libc::c_char,
               developer); // early console running to catch all the messages
    Cvar_DirectSet(&mut host_developer, dev_level.as_mut_ptr());
    Cvar_RegisterVariable(&mut sys_ticrate);
    if _Sys_GetParmFromCmdLine(b"-sys_ticrate\x00" as *const u8 as
                                   *const libc::c_char, ticrate.as_mut_ptr(),
                               ::std::mem::size_of::<[libc::c_char; 16]>() as
                                   libc::c_ulong) as u64 != 0 {
        let mut fps: libc::c_double =
            if atof(ticrate.as_mut_ptr()) >= 20.0f32 as libc::c_double {
                if atof(ticrate.as_mut_ptr()) < 200.0f32 as libc::c_double {
                    atof(ticrate.as_mut_ptr())
                } else { 200.0f32 as libc::c_double }
            } else { 20.0f32 as libc::c_double };
        Cvar_SetValue(b"sys_ticrate\x00" as *const u8 as *const libc::c_char,
                      fps as libc::c_float);
    }
    Con_Init();
    Platform_Init();
    baseDir =
        getenv(b"XASH3D_BASEDIR\x00" as *const u8 as *const libc::c_char);
    if if baseDir.is_null() || *baseDir == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } != 0 {
        Q_strncpy(host.rootdir.as_mut_ptr(), baseDir,
                  ::std::mem::size_of::<[libc::c_char; 260]>() as
                      libc::c_ulong);
    } else {
        let mut szBasePath: *mut libc::c_char = 0 as *mut libc::c_char;
        szBasePath = SDL_GetBasePath();
        if szBasePath.is_null() {
            Sys_Error(b"couldn\'t determine current directory: %s\x00" as
                          *const u8 as *const libc::c_char, SDL_GetError());
        }
        Q_strncpy(host.rootdir.as_mut_ptr(), szBasePath,
                  ::std::mem::size_of::<[libc::c_char; 260]>() as
                      libc::c_ulong);
        SDL_free(szBasePath as *mut libc::c_void);
    }
    len = Q_strlen(host.rootdir.as_mut_ptr()) as libc::c_int;
    if host.rootdir[(len - 1 as libc::c_int) as usize] as libc::c_int ==
           '/' as i32 {
        host.rootdir[(len - 1 as libc::c_int) as usize] =
            0 as libc::c_int as libc::c_char
    }
    // get readonly root. The order is: check for arg, then env.
	// if still not got it, rodir is disabled.
    host.rodir[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    if _Sys_GetParmFromCmdLine(b"-rodir\x00" as *const u8 as
                                   *const libc::c_char,
                               host.rodir.as_mut_ptr(),
                               ::std::mem::size_of::<[libc::c_char; 260]>() as
                                   libc::c_ulong) as u64 == 0 {
        let mut roDir: *mut libc::c_char =
            getenv(b"XASH3D_RODIR\x00" as *const u8 as *const libc::c_char);
        if if roDir.is_null() || *roDir == 0 {
               0 as libc::c_int
           } else { 1 as libc::c_int } != 0 {
            Q_strncpy(host.rodir.as_mut_ptr(), roDir,
                      ::std::mem::size_of::<[libc::c_char; 260]>() as
                          libc::c_ulong);
        }
    }
    len = Q_strlen(host.rodir.as_mut_ptr()) as libc::c_int;
    if len != 0 &&
           host.rodir[(len - 1 as libc::c_int) as usize] as libc::c_int ==
               '/' as i32 {
        host.rodir[(len - 1 as libc::c_int) as usize] =
            0 as libc::c_int as libc::c_char
    }
    if (if *host.rootdir.as_mut_ptr() == 0 {
            0 as libc::c_int
        } else { 1 as libc::c_int }) == 0 ||
           FS_SetCurrentDirectory(host.rootdir.as_mut_ptr()) !=
               0 as libc::c_int {
        Con_Reportf(b"%s is working directory now\n\x00" as *const u8 as
                        *const libc::c_char, host.rootdir.as_mut_ptr());
    } else {
        Sys_Error(b"Changing working directory to %s failed.\n\x00" as
                      *const u8 as *const libc::c_char,
                  host.rootdir.as_mut_ptr());
    }
    Sys_InitLog();
    Cmd_AddCommand(b"exec\x00" as *const u8 as *const libc::c_char,
                   Some(Host_Exec_f as unsafe extern "C" fn() -> ()),
                   b"execute a script file\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"memlist\x00" as *const u8 as *const libc::c_char,
                   Some(Host_MemStats_f as unsafe extern "C" fn() -> ()),
                   b"prints memory pool information\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddRestrictedCommand(b"userconfigd\x00" as *const u8 as
                                 *const libc::c_char,
                             Some(Host_Userconfigd_f as
                                      unsafe extern "C" fn() -> ()),
                             b"execute all scripts from userconfig.d\x00" as
                                 *const u8 as *const libc::c_char);
    FS_Init();
    Image_Init();
    Sound_Init();
    FS_LoadGameInfo(0 as *const libc::c_char);
    Q_strncpy(host.gamefolder.as_mut_ptr(),
              (*SI.GameInfo).gamefolder.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    // DEPRECATED: by FWGS fork
    HPAK_Init();
    IN_Init();
    Key_Init();
}
#[no_mangle]
pub unsafe extern "C" fn Host_FreeCommon() {
    Image_Shutdown();
    Sound_Shutdown();
    Netchan_Shutdown();
    HPAK_FlushHostQueue();
    FS_Shutdown();
}
/*
=================
Host_Main
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Host_Main(mut argc: libc::c_int,
                                   mut argv: *mut *mut libc::c_char,
                                   mut progname: *const libc::c_char,
                                   mut bChangeGame: libc::c_int,
                                   mut func: pfnChangeGame) -> libc::c_int {
    static mut oldtime: libc::c_double = 0.; // may be NULL
    static mut newtime: libc::c_double = 0.;
    pChangeGame = func;
    Host_InitCommon(argc, argv, progname, bChangeGame as qboolean);
    // init commands and vars
    if host_developer.value >= DEV_EXTENDED as libc::c_int as libc::c_float {
        Cmd_AddRestrictedCommand(b"sys_error\x00" as *const u8 as
                                     *const libc::c_char,
                                 Some(Sys_Error_f as
                                          unsafe extern "C" fn() -> ()),
                                 b"just throw a fatal error to test shutdown procedures\x00"
                                     as *const u8 as *const libc::c_char);
        Cmd_AddRestrictedCommand(b"host_error\x00" as *const u8 as
                                     *const libc::c_char,
                                 Some(Host_Error_f as
                                          unsafe extern "C" fn() -> ()),
                                 b"just throw a host error to test shutdown procedures\x00"
                                     as *const u8 as *const libc::c_char);
        Cmd_AddRestrictedCommand(b"crash\x00" as *const u8 as
                                     *const libc::c_char,
                                 Some(Host_Crash_f as
                                          unsafe extern "C" fn() -> ()),
                                 b"a way to force a bus error for development reasons\x00"
                                     as *const u8 as *const libc::c_char);
    }
    host_serverstate =
        Cvar_Get(b"host_serverstate\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 17 as libc::c_int,
                 b"displays current server state\x00" as *const u8 as
                     *const libc::c_char);
    host_maxfps =
        Cvar_Get(b"fps_max\x00" as *const u8 as *const libc::c_char,
                 b"72\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int |
                     (1 as libc::c_int) << 11 as libc::c_int,
                 b"host fps upper limit\x00" as *const u8 as
                     *const libc::c_char);
    host_framerate =
        Cvar_Get(b"host_framerate\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 11 as libc::c_int,
                 b"locks frame timing to this value in seconds\x00" as
                     *const u8 as *const libc::c_char);
    host_sleeptime =
        Cvar_Get(b"sleeptime\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int |
                     (1 as libc::c_int) << 11 as libc::c_int,
                 b"milliseconds to sleep for each frame. higher values reduce fps accuracy\x00"
                     as *const u8 as *const libc::c_char);
    host_gameloaded =
        Cvar_Get(b"host_gameloaded\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 17 as libc::c_int,
                 b"inidcates a loaded game.dll\x00" as *const u8 as
                     *const libc::c_char);
    host_clientloaded =
        Cvar_Get(b"host_clientloaded\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 17 as libc::c_int,
                 b"inidcates a loaded client.dll\x00" as *const u8 as
                     *const libc::c_char);
    host_limitlocal =
        Cvar_Get(b"host_limitlocal\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 0 as libc::c_int,
                 b"apply cl_cmdrate and rate to loopback connection\x00" as
                     *const u8 as *const libc::c_char);
    con_gamemaps =
        Cvar_Get(b"con_mapfilter\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 0 as libc::c_int,
                 b"when true show only maps in game folder\x00" as *const u8
                     as *const libc::c_char);
    build =
        Cvar_Get(b"buildnum\x00" as *const u8 as *const libc::c_char,
                 va(b"%i\x00" as *const u8 as *const libc::c_char,
                    Q_buildnum_compat()),
                 (1 as libc::c_int) << 17 as libc::c_int,
                 b"returns a current build number\x00" as *const u8 as
                     *const libc::c_char);
    ver =
        Cvar_Get(b"ver\x00" as *const u8 as *const libc::c_char,
                 va(b"%i/%s (hw build %i)\x00" as *const u8 as
                        *const libc::c_char, 49 as libc::c_int,
                    b"0.99\x00" as *const u8 as *const libc::c_char,
                    Q_buildnum_compat()),
                 (1 as libc::c_int) << 17 as libc::c_int,
                 b"shows an engine version\x00" as *const u8 as
                     *const libc::c_char);
    Cvar_Get(b"host_ver\x00" as *const u8 as *const libc::c_char,
             va(b"%i %s %s %s %s\x00" as *const u8 as *const libc::c_char,
                Q_buildnum(), b"0.20\x00" as *const u8 as *const libc::c_char,
                Q_buildos(), Q_buildarch(), Q_buildcommit()),
             (1 as libc::c_int) << 17 as libc::c_int,
             b"detailed info about this build\x00" as *const u8 as
                 *const libc::c_char);
    Mod_Init();
    NET_Init();
    NET_InitMasters();
    Netchan_Init();
    // allow to change game from the console
    if pChangeGame.is_some() {
        Cmd_AddRestrictedCommand(b"game\x00" as *const u8 as
                                     *const libc::c_char,
                                 Some(Host_ChangeGame_f as
                                          unsafe extern "C" fn() -> ()),
                                 b"change game\x00" as *const u8 as
                                     *const libc::c_char);
        Cvar_Get(b"host_allow_changegame\x00" as *const u8 as
                     *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 17 as libc::c_int,
                 b"allows to change games\x00" as *const u8 as
                     *const libc::c_char);
    } else {
        Cvar_Get(b"host_allow_changegame\x00" as *const u8 as
                     *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 17 as libc::c_int,
                 b"allows to change games\x00" as *const u8 as
                     *const libc::c_char);
    }
    SV_Init();
    CL_Init();
    HTTP_Init();
    ID_Init();
    if host.type_0 == HOST_DEDICATED as libc::c_int as libc::c_uint {
        Cmd_AddRestrictedCommand(b"quit\x00" as *const u8 as
                                     *const libc::c_char,
                                 ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                    -> !>,
                                                         xcommand_t>(Some(Sys_Quit
                                                                              as
                                                                              unsafe extern "C" fn()
                                                                                  ->
                                                                                      !)),
                                 b"quit the game\x00" as *const u8 as
                                     *const libc::c_char);
        Cmd_AddRestrictedCommand(b"exit\x00" as *const u8 as
                                     *const libc::c_char,
                                 ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                    -> !>,
                                                         xcommand_t>(Some(Sys_Quit
                                                                              as
                                                                              unsafe extern "C" fn()
                                                                                  ->
                                                                                      !)),
                                 b"quit the game\x00" as *const u8 as
                                     *const libc::c_char);
    } else {
        Cmd_AddRestrictedCommand(b"minimize\x00" as *const u8 as
                                     *const libc::c_char,
                                 Some(Host_Minimize_f as
                                          unsafe extern "C" fn() -> ()),
                                 b"minimize main window to tray\x00" as
                                     *const u8 as *const libc::c_char);
    }
    host.errorframe = 0 as libc::c_int as dword;
    // post initializations
    match host.type_0 {
        0 => {
            // execute startup config and cmdline
            Cbuf_AddText(va(b"exec %s.rc\n\x00" as *const u8 as
                                *const libc::c_char, SI.rcName.as_mut_ptr()));
            Cbuf_Execute();
            if host.config_executed as u64 == 0 {
                Cbuf_AddText(b"exec config.cfg\n\x00" as *const u8 as
                                 *const libc::c_char);
                Cbuf_Execute();
            }
            // exec all files from userconfig.d
            Host_Userconfigd_f();
        }
        1 => {
            // allways parse commandline in dedicated-mode
            host.stuffcmds_pending = true_0
        }
        _ => { }
    } // done
    host.change_game = false_0; // execute stuffcmds (commandline)
    Cmd_RemoveCommand(b"setgl\x00" as *const u8 as
                          *const libc::c_char); // must be last
    Cbuf_ExecStuffCmds();
    SCR_CheckStartupVids();
    oldtime = Sys_DoubleTime() - 0.1f64;
    if host.type_0 == HOST_DEDICATED as libc::c_int as libc::c_uint &&
           host.game.nextstate as libc::c_uint ==
               STATE_RUNFRAME as libc::c_int as libc::c_uint {
        // execute server.cfg after commandline
		// so we have a chance to set servercfgfile
        Con_Printf(b"Type \'map <mapname>\' to start game... (TAB-autocomplete is working too)\n\x00"
                       as *const u8 as *const libc::c_char);
        Cbuf_AddText(va(b"exec %s\n\x00" as *const u8 as *const libc::c_char,
                        Cvar_VariableString(b"servercfgfile\x00" as *const u8
                                                as *const libc::c_char)));
        Cbuf_Execute();
    }
    // main window message loop
    while host.crashed as u64 == 0 {
        newtime = Sys_DoubleTime();
        COM_Frame((newtime - oldtime) as libc::c_float);
        oldtime = newtime
    }
    // never reached
    return 0 as libc::c_int;
}
/*
=================
Host_Shutdown
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Host_Shutdown() {
    if host.shutdown_issued as u64 != 0 {
        return
    } // prepare host to normal shutdown
    host.shutdown_issued = true_0;
    if host.status as libc::c_uint !=
           HOST_ERR_FATAL as libc::c_int as libc::c_uint {
        host.status = HOST_SHUTDOWN
    }
    if host.change_game as u64 == 0 {
        Q_strncpy(host.finalmsg.as_mut_ptr(),
                  b"Server shutdown\x00" as *const u8 as *const libc::c_char,
                  ::std::mem::size_of::<string>() as libc::c_ulong);
    }
    if host.type_0 == HOST_NORMAL as libc::c_int as libc::c_uint {
        Host_WriteConfig();
    }
    SV_Shutdown(b"Server shutdown\n\x00" as *const u8 as *const libc::c_char);
    SV_ShutdownFilter();
    CL_Shutdown();
    Mod_Shutdown();
    NET_Shutdown();
    HTTP_Shutdown();
    Host_FreeCommon();
    Platform_Shutdown();
    // must be last, console uses this
    _Mem_FreePool(&mut host.mempool,
                  b"../engine/common/host.c\x00" as *const u8 as
                      *const libc::c_char, 1219 as libc::c_int);
    // restore filter
    Sys_RestoreCrashHandler();
    Sys_CloseLog();
}
