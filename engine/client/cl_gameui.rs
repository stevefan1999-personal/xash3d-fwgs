#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, extern_types,
           register_tool)]
extern "C" {
    pub type file_s;
    pub type grasshdr_s;
    pub type movie_state_s;
    pub type IVoiceTweak_s;
    pub type demo_api_s;
    pub type engine_studio_api_s;
    pub type r_studio_interface_s;
    pub type mip_s;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Q_atoi(str: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn Q_snprintf(buffer: *mut libc::c_char, buffersize: size_t,
                  format: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn Q_pretifymem(value: libc::c_float, digitsafterdecimal: libc::c_int)
     -> *mut libc::c_char;
    #[no_mangle]
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn COM_DefaultExtension(path: *mut libc::c_char,
                            extension: *const libc::c_char);
    #[no_mangle]
    fn _COM_ParseFileSafe(data: *mut libc::c_char, token: *mut libc::c_char,
                          size: libc::c_int, flags: libc::c_uint,
                          len: *mut libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn Platform_ShellExecute(path: *const libc::c_char,
                             parms: *const libc::c_char);
    #[no_mangle]
    fn Sys_DoubleTime() -> libc::c_double;
    #[no_mangle]
    fn Sys_GetClipboardData() -> *mut libc::c_char;
    #[no_mangle]
    fn Sys_Quit() -> !;
    #[no_mangle]
    fn AVI_GetState(num: libc::c_int) -> *mut movie_state_t;
    #[no_mangle]
    fn AVI_IsActive(Avi: *mut movie_state_t) -> qboolean;
    #[no_mangle]
    fn AVI_CloseVideo(Avi: *mut movie_state_t);
    #[no_mangle]
    fn Cvar_FullSet(var_name: *const libc::c_char, value: *const libc::c_char,
                    flags: libc::c_int);
    #[no_mangle]
    fn Cvar_Set(var_name: *const libc::c_char, value: *const libc::c_char);
    #[no_mangle]
    fn Cvar_SetValue(var_name: *const libc::c_char, value: libc::c_float);
    #[no_mangle]
    fn Cvar_VariableValue(var_name: *const libc::c_char) -> libc::c_float;
    #[no_mangle]
    fn Cvar_VariableString(var_name: *const libc::c_char)
     -> *const libc::c_char;
    #[no_mangle]
    fn Cvar_Unlink(group: libc::c_int);
    #[no_mangle]
    fn AVI_OpenVideo(Avi: *mut movie_state_t, filename: *const libc::c_char,
                     load_audio: qboolean, quiet: libc::c_int);
    #[no_mangle]
    fn AVI_GetVideoInfo(Avi: *mut movie_state_t, xres: *mut libc::c_int,
                        yres: *mut libc::c_int, duration: *mut libc::c_float)
     -> qboolean;
    #[no_mangle]
    fn AVI_GetVideoFrame(Avi: *mut movie_state_t, frame: libc::c_int)
     -> *mut byte;
    #[no_mangle]
    fn AVI_GetVideoFrameNumber(Avi: *mut movie_state_t, time: libc::c_float)
     -> libc::c_int;
    #[no_mangle]
    fn NET_AdrToString(a: netadr_t) -> *const libc::c_char;
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    static mut SI: sysinfo_t;
    #[no_mangle]
    fn Cbuf_AddText(text: *const libc::c_char);
    #[no_mangle]
    fn Cbuf_Execute();
    #[no_mangle]
    fn Cmd_Argc() -> libc::c_int;
    #[no_mangle]
    fn Cmd_Args() -> *const libc::c_char;
    #[no_mangle]
    fn Cmd_Argv(arg: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn Cmd_Unlink(group: libc::c_int);
    #[no_mangle]
    fn Cmd_AddGameUICommand(cmd_name: *const libc::c_char,
                            function: xcommand_t) -> libc::c_int;
    #[no_mangle]
    fn Cmd_RemoveCommand(cmd_name: *const libc::c_char);
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
    fn FS_AllowDirectPaths(enable: qboolean);
    #[no_mangle]
    fn FS_GetDiskPath(name: *const libc::c_char, gamedironly: qboolean)
     -> *const libc::c_char;
    #[no_mangle]
    fn COM_FreeFile(buffer: *mut libc::c_void);
    #[no_mangle]
    fn COM_CompareFileTime(filename1: *const libc::c_char,
                           filename2: *const libc::c_char,
                           iCompare: *mut libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn FS_Search(pattern: *const libc::c_char, caseinsensitive: libc::c_int,
                 gamedironly: libc::c_int) -> *mut search_t;
    #[no_mangle]
    fn FS_FileExists(filename: *const libc::c_char, gamedironly: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn FS_Delete(path: *const libc::c_char) -> qboolean;
    #[no_mangle]
    fn Image_SetForceFlags(flags: uint);
    #[no_mangle]
    fn Image_ClearForceFlags();
    #[no_mangle]
    fn Host_NewInstance(name: *const libc::c_char,
                        finalmsg: *const libc::c_char);
    #[no_mangle]
    fn Host_EndGame(abort: qboolean, message: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Host_WriteServerConfig(name: *const libc::c_char);
    #[no_mangle]
    fn Host_IsLocalClient() -> qboolean;
    #[no_mangle]
    fn Host_Error(error: *const libc::c_char, _: ...);
    #[no_mangle]
    fn CL_Active() -> libc::c_int;
    #[no_mangle]
    fn pfnCvar_RegisterGameUIVariable(szName: *const libc::c_char,
                                      szValue: *const libc::c_char,
                                      flags: libc::c_int) -> *mut cvar_t;
    #[no_mangle]
    fn COM_SaveFile(filename: *const libc::c_char, data: *const libc::c_void,
                    len: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn COM_LoadFileForMe(filename: *const libc::c_char,
                         pLength: *mut libc::c_int) -> *mut byte;
    #[no_mangle]
    fn pfnGetGameDir(szGetGameDir: *mut libc::c_char);
    #[no_mangle]
    fn pfnIsMapValid(filename: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn Con_Reportf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_DPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn CL_GetDemoComment(demoname: *const libc::c_char,
                         comment: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn CL_Disconnect();
    #[no_mangle]
    fn SV_GetSaveComment(savename: *const libc::c_char,
                         comment: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn COM_RandomFloat(fMin: libc::c_float, fMax: libc::c_float)
     -> libc::c_float;
    #[no_mangle]
    fn COM_RandomLong(lMin: libc::c_int, lMax: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Cmd_CheckMapsList(fRefresh: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Info_ValueForKey(s: *const libc::c_char, key: *const libc::c_char)
     -> *const libc::c_char;
    #[no_mangle]
    fn UI_NXPrintf(info: *mut con_nprint_t, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn UI_NPrintf(idx: libc::c_int, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn GL_FreeImage(name: *const libc::c_char);
    #[no_mangle]
    static mut cl: client_t;
    #[no_mangle]
    static mut cls: client_static_t;
    #[no_mangle]
    static mut clgame: clgame_static_t;
    #[no_mangle]
    fn PicAdjustSize(x: *mut libc::c_float, y: *mut libc::c_float,
                     w: *mut libc::c_float, h: *mut libc::c_float);
    #[no_mangle]
    fn Mod_ForName(name: *const libc::c_char, crash: qboolean,
                   trackCRC: qboolean) -> *mut model_t;
    #[link_name = "ref"]
    static mut ref_0: ref_state_s;
    #[no_mangle]
    fn R_GetTextureParms(w: *mut libc::c_int, h: *mut libc::c_int,
                         texnum: libc::c_int);
    #[no_mangle]
    fn GL_RenderFrame(rvp: *const ref_viewpass_s);
    #[no_mangle]
    fn Key_SetKeyDest(key_dest: libc::c_int);
    #[no_mangle]
    fn Con_UtfProcessChar(in_0: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Con_UtfMoveLeft(str: *mut libc::c_char, pos: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn Con_UtfMoveRight(str: *mut libc::c_char, pos: libc::c_int,
                        length: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Con_DrawStringLen(pText: *const libc::c_char, length: *mut libc::c_int,
                         height: *mut libc::c_int);
    #[no_mangle]
    fn Con_DrawString(x: libc::c_int, y: libc::c_int,
                      string: *const libc::c_char, setColor: *mut byte)
     -> libc::c_int;
    #[no_mangle]
    fn Con_DefaultColor(r: libc::c_int, g: libc::c_int, b: libc::c_int);
    #[no_mangle]
    fn S_StartBackgroundTrack(intro: *const libc::c_char,
                              loop_0: *const libc::c_char,
                              position: libc::c_int, fullpath: qboolean);
    #[no_mangle]
    fn S_StartLocalSound(name: *const libc::c_char, volume: libc::c_float,
                         reliable: qboolean);
    #[no_mangle]
    fn Key_EnableTextInput(enable: qboolean, force: qboolean);
    #[no_mangle]
    fn Key_ClearStates();
    #[no_mangle]
    fn Key_KeynumToString(keynum: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn Key_GetBinding(keynum: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn Key_SetBinding(keynum: libc::c_int, binding: *const libc::c_char);
    #[no_mangle]
    fn Key_IsDown(keynum: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn COM_LoadLibrary(dllname: *const libc::c_char,
                       build_ordinals_table: libc::c_int,
                       directpath: qboolean) -> *mut libc::c_void;
    #[no_mangle]
    fn COM_GetProcAddress(hInstance: *mut libc::c_void,
                          name: *const libc::c_char) -> *mut libc::c_void;
    #[no_mangle]
    fn COM_FreeLibrary(hInstance: *mut libc::c_void);
    #[no_mangle]
    fn COM_GetLibraryError() -> *const libc::c_char;
    #[no_mangle]
    fn COM_GetCommonLibraryPath(eLibType: ECommonLibraryType,
                                out: *mut libc::c_char, size: size_t);
    #[no_mangle]
    fn IN_SetCursor(hCursor: *mut libc::c_void);
    #[no_mangle]
    static mut svgame: svgame_static_t;
    #[no_mangle]
    fn VID_GetModeString(vid_mode: libc::c_int) -> *const libc::c_char;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct search_t {
    pub numfilenames: libc::c_int,
    pub filenames: *mut *mut libc::c_char,
    pub filenamesbuffer: *mut libc::c_char,
}
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
pub type cvar_t = cvar_s;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const IL_OVERVIEW: C2RustUnnamed_0 = 64;
pub const IL_LOAD_DECAL: C2RustUnnamed_0 = 32;
pub const IL_DDS_HARDWARE: C2RustUnnamed_0 = 16;
pub const IL_DONTFLIP_TGA: C2RustUnnamed_0 = 8;
pub const IL_ALLOW_OVERWRITE: C2RustUnnamed_0 = 4;
pub const IL_KEEP_8BIT: C2RustUnnamed_0 = 2;
pub const IL_USE_LERPING: C2RustUnnamed_0 = 1;
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
pub struct UI_FUNCTIONS {
    pub pfnVidInit: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub pfnInit: Option<unsafe extern "C" fn() -> ()>,
    pub pfnShutdown: Option<unsafe extern "C" fn() -> ()>,
    pub pfnRedraw: Option<unsafe extern "C" fn(_: libc::c_float) -> ()>,
    pub pfnKeyEvent: Option<unsafe extern "C" fn(_: libc::c_int,
                                                 _: libc::c_int) -> ()>,
    pub pfnMouseMove: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _: libc::c_int) -> ()>,
    pub pfnSetActiveMenu: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub pfnAddServerToList: Option<unsafe extern "C" fn(_: netadr_s,
                                                        _:
                                                            *const libc::c_char)
                                       -> ()>,
    pub pfnGetCursorPos: Option<unsafe extern "C" fn(_: *mut libc::c_int,
                                                     _: *mut libc::c_int)
                                    -> ()>,
    pub pfnSetCursorPos: Option<unsafe extern "C" fn(_: libc::c_int,
                                                     _: libc::c_int) -> ()>,
    pub pfnShowCursor: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub pfnCharEvent: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub pfnMouseInRect: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub pfnIsVisible: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub pfnCreditsActive: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub pfnFinalCredits: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gameui_static_t {
    pub hInstance: *mut libc::c_void,
    pub dllFuncs: UI_FUNCTIONS,
    pub dllFuncs2: UI_EXTENDED_FUNCTIONS,
    pub mempool: poolhandle_t,
    pub playermodel: cl_entity_t,
    pub playerinfo: player_info_t,
    pub ds: gameui_draw_t,
    pub gameInfo: GAMEINFO,
    pub modsInfo: [*mut GAMEINFO; 512],
    pub globals: *mut ui_globalvars_t,
    pub drawLogo: qboolean,
    pub logo_xres: libc::c_int,
    pub logo_yres: libc::c_int,
    pub logo_length: libc::c_float,
    pub use_text_api: qboolean,
}
pub type ui_globalvars_t = ui_globalvars_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ui_globalvars_s {
    pub time: libc::c_float,
    pub frametime: libc::c_float,
    pub scrWidth: libc::c_int,
    pub scrHeight: libc::c_int,
    pub maxClients: libc::c_int,
    pub developer: libc::c_int,
    pub demoplayback: libc::c_int,
    pub demorecording: libc::c_int,
    pub demoname: [libc::c_char; 64],
    pub maptitle: [libc::c_char; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GAMEINFO {
    pub gamefolder: [libc::c_char; 64],
    pub startmap: [libc::c_char; 64],
    pub trainmap: [libc::c_char; 64],
    pub title: [libc::c_char; 64],
    pub version: [libc::c_char; 14],
    pub flags: libc::c_short,
    pub game_url: [libc::c_char; 256],
    pub update_url: [libc::c_char; 256],
    pub type_0: [libc::c_char; 64],
    pub date: [libc::c_char; 64],
    pub size: [libc::c_char; 64],
    pub gamemode: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gameui_draw_t {
    pub gl_texturenum: libc::c_int,
    pub scissor_x: libc::c_int,
    pub scissor_y: libc::c_int,
    pub scissor_width: libc::c_int,
    pub scissor_height: libc::c_int,
    pub scissor_test: qboolean,
    pub textColor: rgba_t,
}
pub type cl_entity_t = cl_entity_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UI_EXTENDED_FUNCTIONS {
    pub pfnAddTouchButtonToList: ADDTOUCHBUTTONTOLIST,
    pub pfnResetPing: Option<unsafe extern "C" fn() -> ()>,
    pub pfnShowConnectionWarning: Option<unsafe extern "C" fn() -> ()>,
    pub pfnShowUpdateDialog: Option<unsafe extern "C" fn(_: libc::c_int)
                                        -> ()>,
    pub pfnShowMessageBox: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                      -> ()>,
    pub pfnConnectionProgress_Disconnect: Option<unsafe extern "C" fn()
                                                     -> ()>,
    pub pfnConnectionProgress_Download: Option<unsafe extern "C" fn(_:
                                                                        *const libc::c_char,
                                                                    _:
                                                                        *const libc::c_char,
                                                                    _:
                                                                        libc::c_int,
                                                                    _:
                                                                        libc::c_int,
                                                                    _:
                                                                        *const libc::c_char)
                                                   -> ()>,
    pub pfnConnectionProgress_DownloadEnd: Option<unsafe extern "C" fn()
                                                      -> ()>,
    pub pfnConnectionProgress_Precache: Option<unsafe extern "C" fn() -> ()>,
    pub pfnConnectionProgress_Connect: Option<unsafe extern "C" fn(_:
                                                                       *const libc::c_char)
                                                  -> ()>,
    pub pfnConnectionProgress_ChangeLevel: Option<unsafe extern "C" fn()
                                                      -> ()>,
    pub pfnConnectionProgress_ParseServerInfo: Option<unsafe extern "C" fn(_:
                                                                               *const libc::c_char)
                                                          -> ()>,
}
pub type ADDTOUCHBUTTONTOLIST
    =
    Option<unsafe extern "C" fn(_: *const libc::c_char,
                                _: *const libc::c_char,
                                _: *const libc::c_char, _: *mut libc::c_uchar,
                                _: libc::c_int) -> ()>;
pub type movie_state_t = movie_state_s;
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
pub type local_state_t = local_state_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct local_state_s {
    pub playerstate: entity_state_t,
    pub client: clientdata_t,
    pub weapondata: [weapon_data_t; 64],
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
pub const ca_connected: connstate_e = 2;
pub type connstate_t = connstate_e;
pub type connstate_e = libc::c_uint;
pub const ca_cinematic: connstate_e = 5;
pub const ca_active: connstate_e = 4;
pub const ca_validate: connstate_e = 3;
pub const ca_connecting: connstate_e = 1;
pub const ca_disconnected: connstate_e = 0;
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
pub type HIMAGE = libc::c_int;
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
pub struct ui_enginefuncs_s {
    pub pfnPIC_Load: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                 _: *const byte,
                                                 _: libc::c_int,
                                                 _: libc::c_int) -> HIMAGE>,
    pub pfnPIC_Free: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                -> ()>,
    pub pfnPIC_Width: Option<unsafe extern "C" fn(_: HIMAGE) -> libc::c_int>,
    pub pfnPIC_Height: Option<unsafe extern "C" fn(_: HIMAGE) -> libc::c_int>,
    pub pfnPIC_Set: Option<unsafe extern "C" fn(_: HIMAGE, _: libc::c_int,
                                                _: libc::c_int,
                                                _: libc::c_int,
                                                _: libc::c_int) -> ()>,
    pub pfnPIC_Draw: Option<unsafe extern "C" fn(_: libc::c_int,
                                                 _: libc::c_int,
                                                 _: libc::c_int,
                                                 _: libc::c_int,
                                                 _: *const wrect_t) -> ()>,
    pub pfnPIC_DrawHoles: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: *const wrect_t)
                                     -> ()>,
    pub pfnPIC_DrawTrans: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: *const wrect_t)
                                     -> ()>,
    pub pfnPIC_DrawAdditive: Option<unsafe extern "C" fn(_: libc::c_int,
                                                         _: libc::c_int,
                                                         _: libc::c_int,
                                                         _: libc::c_int,
                                                         _: *const wrect_t)
                                        -> ()>,
    pub pfnPIC_EnableScissor: Option<unsafe extern "C" fn(_: libc::c_int,
                                                          _: libc::c_int,
                                                          _: libc::c_int,
                                                          _: libc::c_int)
                                         -> ()>,
    pub pfnPIC_DisableScissor: Option<unsafe extern "C" fn() -> ()>,
    pub pfnFillRGBA: Option<unsafe extern "C" fn(_: libc::c_int,
                                                 _: libc::c_int,
                                                 _: libc::c_int,
                                                 _: libc::c_int,
                                                 _: libc::c_int,
                                                 _: libc::c_int,
                                                 _: libc::c_int,
                                                 _: libc::c_int) -> ()>,
    pub pfnRegisterVariable: Option<unsafe extern "C" fn(_:
                                                             *const libc::c_char,
                                                         _:
                                                             *const libc::c_char,
                                                         _: libc::c_int)
                                        -> *mut cvar_t>,
    pub pfnGetCvarFloat: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                    -> libc::c_float>,
    pub pfnGetCvarString: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                     -> *const libc::c_char>,
    pub pfnCvarSetString: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                      _: *const libc::c_char)
                                     -> ()>,
    pub pfnCvarSetValue: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                     _: libc::c_float) -> ()>,
    pub pfnAddCommand: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                   _:
                                                       Option<unsafe extern "C" fn()
                                                                  -> ()>)
                                  -> libc::c_int>,
    pub pfnClientCmd: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _: *const libc::c_char)
                                 -> ()>,
    pub pfnDelCommand: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                  -> ()>,
    pub pfnCmdArgc: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub pfnCmdArgv: Option<unsafe extern "C" fn(_: libc::c_int)
                               -> *const libc::c_char>,
    pub pfnCmd_Args: Option<unsafe extern "C" fn() -> *const libc::c_char>,
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
    pub pfnPlayLocalSound: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                      -> ()>,
    pub pfnDrawLogo: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                 _: libc::c_float,
                                                 _: libc::c_float,
                                                 _: libc::c_float,
                                                 _: libc::c_float) -> ()>,
    pub pfnGetLogoWidth: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub pfnGetLogoHeight: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub pfnGetLogoLength: Option<unsafe extern "C" fn() -> libc::c_float>,
    pub pfnDrawCharacter: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: HIMAGE) -> ()>,
    pub pfnDrawConsoleString: Option<unsafe extern "C" fn(_: libc::c_int,
                                                          _: libc::c_int,
                                                          _:
                                                              *const libc::c_char)
                                         -> libc::c_int>,
    pub pfnDrawSetTextColor: Option<unsafe extern "C" fn(_: libc::c_int,
                                                         _: libc::c_int,
                                                         _: libc::c_int,
                                                         _: libc::c_int)
                                        -> ()>,
    pub pfnDrawConsoleStringLen: Option<unsafe extern "C" fn(_:
                                                                 *const libc::c_char,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _:
                                                                 *mut libc::c_int)
                                            -> ()>,
    pub pfnSetConsoleDefaultColor: Option<unsafe extern "C" fn(_: libc::c_int,
                                                               _: libc::c_int,
                                                               _: libc::c_int)
                                              -> ()>,
    pub pfnGetPlayerModel: Option<unsafe extern "C" fn() -> *mut cl_entity_s>,
    pub pfnSetModel: Option<unsafe extern "C" fn(_: *mut cl_entity_s,
                                                 _: *const libc::c_char)
                                -> ()>,
    pub pfnClearScene: Option<unsafe extern "C" fn() -> ()>,
    pub pfnRenderScene: Option<unsafe extern "C" fn(_: *const ref_viewpass_s)
                                   -> ()>,
    pub CL_CreateVisibleEntity: Option<unsafe extern "C" fn(_: libc::c_int,
                                                            _:
                                                                *mut cl_entity_s)
                                           -> libc::c_int>,
    pub pfnHostError: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                  _: ...) -> ()>,
    pub pfnFileExists: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                   _: libc::c_int)
                                  -> libc::c_int>,
    pub pfnGetGameDir: Option<unsafe extern "C" fn(_: *mut libc::c_char)
                                  -> ()>,
    pub pfnCreateMapsList: Option<unsafe extern "C" fn(_: libc::c_int)
                                      -> libc::c_int>,
    pub pfnClientInGame: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub pfnClientJoin: Option<unsafe extern "C" fn(_: netadr_s) -> ()>,
    pub COM_LoadFile: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                  _: *mut libc::c_int)
                                 -> *mut byte>,
    pub COM_ParseFile: Option<unsafe extern "C" fn(_: *mut libc::c_char,
                                                   _: *mut libc::c_char)
                                  -> *mut libc::c_char>,
    pub COM_FreeFile: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                 -> ()>,
    pub pfnKeyClearStates: Option<unsafe extern "C" fn() -> ()>,
    pub pfnSetKeyDest: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub pfnKeynumToString: Option<unsafe extern "C" fn(_: libc::c_int)
                                      -> *const libc::c_char>,
    pub pfnKeyGetBinding: Option<unsafe extern "C" fn(_: libc::c_int)
                                     -> *const libc::c_char>,
    pub pfnKeySetBinding: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: *const libc::c_char)
                                     -> ()>,
    pub pfnKeyIsDown: Option<unsafe extern "C" fn(_: libc::c_int)
                                 -> libc::c_int>,
    pub pfnKeyGetOverstrikeMode: Option<unsafe extern "C" fn()
                                            -> libc::c_int>,
    pub pfnKeySetOverstrikeMode: Option<unsafe extern "C" fn(_: libc::c_int)
                                            -> ()>,
    pub pfnKeyGetState: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                   -> *mut libc::c_void>,
    pub pfnMemAlloc: Option<unsafe extern "C" fn(_: size_t,
                                                 _: *const libc::c_char,
                                                 _: libc::c_int)
                                -> *mut libc::c_void>,
    pub pfnMemFree: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                _: *const libc::c_char,
                                                _: libc::c_int) -> ()>,
    pub pfnGetGameInfo: Option<unsafe extern "C" fn(_: *mut GAMEINFO)
                                   -> libc::c_int>,
    pub pfnGetGamesList: Option<unsafe extern "C" fn(_: *mut libc::c_int)
                                    -> *mut *mut GAMEINFO>,
    pub pfnGetFilesList: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                     _: *mut libc::c_int,
                                                     _: libc::c_int)
                                    -> *mut *mut libc::c_char>,
    pub pfnGetSaveComment: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                       _: *mut libc::c_char)
                                      -> libc::c_int>,
    pub pfnGetDemoComment: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                       _: *mut libc::c_char)
                                      -> libc::c_int>,
    pub pfnCheckGameDll: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub pfnGetClipboardData: Option<unsafe extern "C" fn()
                                        -> *mut libc::c_char>,
    pub pfnShellExecute: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                     _: *const libc::c_char,
                                                     _: libc::c_int) -> ()>,
    pub pfnWriteServerConfig: Option<unsafe extern "C" fn(_:
                                                              *const libc::c_char)
                                         -> ()>,
    pub pfnChangeInstance: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                       _: *const libc::c_char)
                                      -> ()>,
    pub pfnPlayBackgroundTrack: Option<unsafe extern "C" fn(_:
                                                                *const libc::c_char,
                                                            _:
                                                                *const libc::c_char)
                                           -> ()>,
    pub pfnHostEndGame: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                   -> ()>,
    pub pfnRandomFloat: Option<unsafe extern "C" fn(_: libc::c_float,
                                                    _: libc::c_float)
                                   -> libc::c_float>,
    pub pfnRandomLong: Option<unsafe extern "C" fn(_: libc::c_int,
                                                   _: libc::c_int)
                                  -> libc::c_int>,
    pub pfnSetCursor: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                 -> ()>,
    pub pfnIsMapValid: Option<unsafe extern "C" fn(_: *mut libc::c_char)
                                  -> libc::c_int>,
    pub pfnProcessImage: Option<unsafe extern "C" fn(_: libc::c_int,
                                                     _: libc::c_float,
                                                     _: libc::c_int,
                                                     _: libc::c_int) -> ()>,
    pub pfnCompareFileTime: Option<unsafe extern "C" fn(_:
                                                            *const libc::c_char,
                                                        _:
                                                            *const libc::c_char,
                                                        _: *mut libc::c_int)
                                       -> libc::c_int>,
    pub pfnGetModeString: Option<unsafe extern "C" fn(_: libc::c_int)
                                     -> *const libc::c_char>,
    pub COM_SaveFile: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                  _: *const libc::c_void,
                                                  _: libc::c_int)
                                 -> libc::c_int>,
    pub COM_RemoveFile: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                   -> libc::c_int>,
}
pub type ui_enginefuncs_t = ui_enginefuncs_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ui_extendedfuncs_s {
    pub pfnEnableTextInput: Option<unsafe extern "C" fn(_: libc::c_int)
                                       -> ()>,
    pub pfnUtfProcessChar: Option<unsafe extern "C" fn(_: libc::c_int)
                                      -> libc::c_int>,
    pub pfnUtfMoveLeft: Option<unsafe extern "C" fn(_: *mut libc::c_char,
                                                    _: libc::c_int)
                                   -> libc::c_int>,
    pub pfnUtfMoveRight: Option<unsafe extern "C" fn(_: *mut libc::c_char,
                                                     _: libc::c_int,
                                                     _: libc::c_int)
                                    -> libc::c_int>,
    pub pfnGetRenderers: Option<unsafe extern "C" fn(_: libc::c_uint,
                                                     _: *mut libc::c_char,
                                                     _: size_t,
                                                     _: *mut libc::c_char,
                                                     _: size_t)
                                    -> libc::c_int>,
    pub pfnDoubleTime: Option<unsafe extern "C" fn() -> libc::c_double>,
    pub pfnParseFile: Option<unsafe extern "C" fn(_: *mut libc::c_char,
                                                  _: *mut libc::c_char,
                                                  _: libc::c_int,
                                                  _: libc::c_uint,
                                                  _: *mut libc::c_int)
                                 -> *mut libc::c_char>,
}
pub type ui_extendedfuncs_t = ui_extendedfuncs_s;
pub type MENUAPI
    =
    Option<unsafe extern "C" fn(_: *mut UI_FUNCTIONS,
                                _: *mut ui_enginefuncs_t,
                                _: *mut ui_globalvars_t) -> libc::c_int>;
pub type UIEXTENEDEDAPI
    =
    Option<unsafe extern "C" fn(_: libc::c_int, _: *mut UI_EXTENDED_FUNCTIONS,
                                _: *mut ui_extendedfuncs_t) -> libc::c_int>;
pub type UITEXTAPI
    =
    Option<unsafe extern "C" fn(_: *mut ui_extendedfuncs_t) -> libc::c_int>;
pub type efrag_t = efrag_s;
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
pub struct vpoint_t {
    pub point: vec2_t,
    pub coord: vec2_t,
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
pub type ref_viewpass_t = ref_viewpass_s;
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
pub type ECommonLibraryType = libc::c_uint;
pub const LIBRARY_GAMEUI: ECommonLibraryType = 2;
pub const LIBRARY_SERVER: ECommonLibraryType = 1;
pub const LIBRARY_CLIENT: ECommonLibraryType = 0;
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
pub struct sv_user_message_t {
    pub name: [libc::c_char; 32],
    pub number: libc::c_int,
    pub size: libc::c_int,
}
#[no_mangle]
pub static mut gameui: gameui_static_t =
    gameui_static_t{hInstance: 0 as *const libc::c_void as *mut libc::c_void,
                    dllFuncs:
                        UI_FUNCTIONS{pfnVidInit: None,
                                     pfnInit: None,
                                     pfnShutdown: None,
                                     pfnRedraw: None,
                                     pfnKeyEvent: None,
                                     pfnMouseMove: None,
                                     pfnSetActiveMenu: None,
                                     pfnAddServerToList: None,
                                     pfnGetCursorPos: None,
                                     pfnSetCursorPos: None,
                                     pfnShowCursor: None,
                                     pfnCharEvent: None,
                                     pfnMouseInRect: None,
                                     pfnIsVisible: None,
                                     pfnCreditsActive: None,
                                     pfnFinalCredits: None,},
                    dllFuncs2:
                        UI_EXTENDED_FUNCTIONS{pfnAddTouchButtonToList: None,
                                              pfnResetPing: None,
                                              pfnShowConnectionWarning: None,
                                              pfnShowUpdateDialog: None,
                                              pfnShowMessageBox: None,
                                              pfnConnectionProgress_Disconnect:
                                                  None,
                                              pfnConnectionProgress_Download:
                                                  None,
                                              pfnConnectionProgress_DownloadEnd:
                                                  None,
                                              pfnConnectionProgress_Precache:
                                                  None,
                                              pfnConnectionProgress_Connect:
                                                  None,
                                              pfnConnectionProgress_ChangeLevel:
                                                  None,
                                              pfnConnectionProgress_ParseServerInfo:
                                                  None,},
                    mempool: 0,
                    playermodel:
                        cl_entity_t{index: 0,
                                    player: false_0,
                                    baseline:
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
                                                           color24{r: 0,
                                                                   g: 0,
                                                                   b: 0,},
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
                                    prevstate:
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
                                                           color24{r: 0,
                                                                   g: 0,
                                                                   b: 0,},
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
                                    curstate:
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
                                                           color24{r: 0,
                                                                   g: 0,
                                                                   b: 0,},
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
                                    current_position: 0,
                                    ph:
                                        [position_history_t{animtime: 0.,
                                                            origin: [0.; 3],
                                                            angles: [0.; 3],};
                                            64],
                                    mouth:
                                        mouth_t{mouthopen: 0,
                                                sndcount: 0,
                                                sndavg: 0,},
                                    latched:
                                        latchedvars_t{prevanimtime: 0.,
                                                      sequencetime: 0.,
                                                      prevseqblending: [0; 2],
                                                      prevorigin: [0.; 3],
                                                      prevangles: [0.; 3],
                                                      prevsequence: 0,
                                                      prevframe: 0.,
                                                      prevcontroller: [0; 4],
                                                      prevblending: [0; 2],},
                                    lastmove: 0.,
                                    origin: [0.; 3],
                                    angles: [0.; 3],
                                    attachment: [[0.; 3]; 4],
                                    trivial_accept: 0,
                                    model:
                                        0 as *const model_s as *mut model_s,
                                    efrag:
                                        0 as *const efrag_s as *mut efrag_s,
                                    topnode:
                                        0 as *const mnode_s as *mut mnode_s,
                                    syncbase: 0.,
                                    visframe: 0,
                                    cvFloorColor:
                                        colorVec{r: 0, g: 0, b: 0, a: 0,},},
                    playerinfo:
                        player_info_t{userid: 0,
                                      userinfo: [0; 256],
                                      name: [0; 32],
                                      spectator: 0,
                                      ping: 0,
                                      packet_loss: 0,
                                      model: [0; 64],
                                      topcolor: 0,
                                      bottomcolor: 0,
                                      renderframe: 0,
                                      gaitsequence: 0,
                                      gaitframe: 0.,
                                      gaityaw: 0.,
                                      prevgaitorigin: [0.; 3],
                                      customdata:
                                          customization_t{bInUse: false_0,
                                                          resource:
                                                              resource_t{szFileName:
                                                                             [0;
                                                                                 64],
                                                                         type_0:
                                                                             t_sound,
                                                                         nIndex:
                                                                             0,
                                                                         nDownloadSize:
                                                                             0,
                                                                         ucFlags:
                                                                             0,
                                                                         rgucMD5_hash:
                                                                             [0;
                                                                                 16],
                                                                         playernum:
                                                                             0,
                                                                         rguc_reserved:
                                                                             [0;
                                                                                 32],
                                                                         pNext:
                                                                             0
                                                                                 as
                                                                                 *const resource_s
                                                                                 as
                                                                                 *mut resource_s,
                                                                         pPrev:
                                                                             0
                                                                                 as
                                                                                 *const resource_s
                                                                                 as
                                                                                 *mut resource_s,},
                                                          bTranslated:
                                                              false_0,
                                                          nUserData1: 0,
                                                          nUserData2: 0,
                                                          pInfo:
                                                              0 as
                                                                  *const libc::c_void
                                                                  as
                                                                  *mut libc::c_void,
                                                          pBuffer:
                                                              0 as
                                                                  *const libc::c_void
                                                                  as
                                                                  *mut libc::c_void,
                                                          pNext:
                                                              0 as
                                                                  *const customization_s
                                                                  as
                                                                  *mut customization_s,},
                                      hashedcdkey: [0; 16],},
                    ds:
                        gameui_draw_t{gl_texturenum: 0,
                                      scissor_x: 0,
                                      scissor_y: 0,
                                      scissor_width: 0,
                                      scissor_height: 0,
                                      scissor_test: false_0,
                                      textColor: [0; 4],},
                    gameInfo:
                        GAMEINFO{gamefolder: [0; 64],
                                 startmap: [0; 64],
                                 trainmap: [0; 64],
                                 title: [0; 64],
                                 version: [0; 14],
                                 flags: 0,
                                 game_url: [0; 256],
                                 update_url: [0; 256],
                                 type_0: [0; 64],
                                 date: [0; 64],
                                 size: [0; 64],
                                 gamemode: 0,},
                    modsInfo: [0 as *const GAMEINFO as *mut GAMEINFO; 512],
                    globals:
                        0 as *const ui_globalvars_t as *mut ui_globalvars_t,
                    drawLogo: false_0,
                    logo_xres: 0,
                    logo_yres: 0,
                    logo_length: 0.,
                    use_text_api: false_0,};
#[no_mangle]
pub unsafe extern "C" fn UI_UpdateMenu(mut realtime: libc::c_float) {
    if gameui.hInstance.is_null() { return }
    // if some deferred cmds is waiting
    if UI_IsVisible() as libc::c_uint != 0 &&
           (if host.deferred_cmd.as_mut_ptr().is_null() ||
                   *host.deferred_cmd.as_mut_ptr() == 0 {
                0 as libc::c_int
            } else { 1 as libc::c_int }) != 0 {
        Cbuf_AddText(host.deferred_cmd.as_mut_ptr());
        host.deferred_cmd[0 as libc::c_int as usize] =
            '\u{0}' as i32 as libc::c_char;
        Cbuf_Execute();
        return
    }
    // don't show menu while level is loaded
    if host.game.nextstate as libc::c_uint !=
           STATE_RUNFRAME as libc::c_int as libc::c_uint &&
           host.game.loadGame as u64 == 0 {
        return
    }
    // menu time (not paused, not clamped)
    (*gameui.globals).time = host.realtime as libc::c_float;
    (*gameui.globals).frametime = host.realframetime as libc::c_float;
    (*gameui.globals).demoplayback = cls.demoplayback;
    (*gameui.globals).demorecording = cls.demorecording as libc::c_int;
    (*gameui.globals).developer = host.allow_console as libc::c_int;
    gameui.dllFuncs.pfnRedraw.expect("non-null function pointer")(realtime);
    UI_UpdateUserinfo();
}
#[no_mangle]
pub unsafe extern "C" fn UI_KeyEvent(mut key: libc::c_int,
                                     mut down: qboolean) {
    if gameui.hInstance.is_null() { return }
    gameui.dllFuncs.pfnKeyEvent.expect("non-null function pointer")(key,
                                                                    down as
                                                                        libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn UI_MouseMove(mut x: libc::c_int,
                                      mut y: libc::c_int) {
    if gameui.hInstance.is_null() { return }
    gameui.dllFuncs.pfnMouseMove.expect("non-null function pointer")(x, y);
}
#[no_mangle]
pub unsafe extern "C" fn UI_SetActiveMenu(mut fActive: qboolean) {
    let mut cin_state: *mut movie_state_t = 0 as *mut movie_state_t;
    if gameui.hInstance.is_null() {
        if fActive as u64 == 0 { Key_SetKeyDest(key_game as libc::c_int); }
        return
    }
    gameui.drawLogo = fActive;
    gameui.dllFuncs.pfnSetActiveMenu.expect("non-null function pointer")(fActive
                                                                             as
                                                                             libc::c_int);
    if fActive as u64 == 0 {
        // close logo when menu is shutdown
        cin_state = AVI_GetState(1 as libc::c_int);
        AVI_CloseVideo(cin_state);
    };
}
#[no_mangle]
pub unsafe extern "C" fn UI_AddServerToList(mut adr: netadr_t,
                                            mut info: *const libc::c_char) {
    if gameui.hInstance.is_null() { return }
    gameui.dllFuncs.pfnAddServerToList.expect("non-null function pointer")(adr,
                                                                           info);
}
#[no_mangle]
pub unsafe extern "C" fn UI_GetCursorPos(mut pos_x: *mut libc::c_int,
                                         mut pos_y: *mut libc::c_int) {
    if gameui.hInstance.is_null() { return }
    gameui.dllFuncs.pfnGetCursorPos.expect("non-null function pointer")(pos_x,
                                                                        pos_y);
}
#[no_mangle]
pub unsafe extern "C" fn UI_SetCursorPos(mut pos_x: libc::c_int,
                                         mut pos_y: libc::c_int) {
    if gameui.hInstance.is_null() { return }
    gameui.dllFuncs.pfnSetCursorPos.expect("non-null function pointer")(pos_x,
                                                                        pos_y);
}
#[no_mangle]
pub unsafe extern "C" fn UI_ShowCursor(mut show: qboolean) {
    if gameui.hInstance.is_null() { return }
    gameui.dllFuncs.pfnShowCursor.expect("non-null function pointer")(show as
                                                                          libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn UI_CreditsActive() -> qboolean {
    if gameui.hInstance.is_null() { return false_0 }
    return gameui.dllFuncs.pfnCreditsActive.expect("non-null function pointer")()
               as qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn UI_CharEvent(mut key: libc::c_int) {
    if gameui.hInstance.is_null() { return }
    gameui.dllFuncs.pfnCharEvent.expect("non-null function pointer")(key);
}
#[no_mangle]
pub unsafe extern "C" fn UI_MouseInRect() -> qboolean {
    if gameui.hInstance.is_null() { return true_0 }
    return gameui.dllFuncs.pfnMouseInRect.expect("non-null function pointer")()
               as qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn UI_IsVisible() -> qboolean {
    if gameui.hInstance.is_null() { return false_0 }
    return gameui.dllFuncs.pfnIsVisible.expect("non-null function pointer")()
               as qboolean;
}
/*
=======================
UI_AddTouchButtonToList

send button parameters to menu
=======================
*/
#[no_mangle]
pub unsafe extern "C" fn UI_AddTouchButtonToList(mut name:
                                                     *const libc::c_char,
                                                 mut texture:
                                                     *const libc::c_char,
                                                 mut command:
                                                     *const libc::c_char,
                                                 mut color:
                                                     *mut libc::c_uchar,
                                                 mut flags: libc::c_int) {
    if gameui.dllFuncs2.pfnAddTouchButtonToList.is_some() {
        gameui.dllFuncs2.pfnAddTouchButtonToList.expect("non-null function pointer")(name,
                                                                                     texture,
                                                                                     command,
                                                                                     color,
                                                                                     flags);
    };
}
/*
=================
UI_ResetPing

notify gameui dll about latency reset
=================
*/
#[no_mangle]
pub unsafe extern "C" fn UI_ResetPing() {
    if gameui.dllFuncs2.pfnResetPing.is_some() {
        gameui.dllFuncs2.pfnResetPing.expect("non-null function pointer")();
    };
}
/*
=================
UI_ShowConnectionWarning

show connection warning dialog implemented by gameui dll
=================
*/
#[no_mangle]
pub unsafe extern "C" fn UI_ShowConnectionWarning() {
    if cls.state as libc::c_uint !=
           ca_connected as libc::c_int as libc::c_uint {
        return
    }
    if Host_IsLocalClient() as u64 != 0 { return }
    cl.lostpackets += 1;
    if cl.lostpackets == 8 as libc::c_int {
        CL_Disconnect();
        if gameui.dllFuncs2.pfnShowConnectionWarning.is_some() {
            gameui.dllFuncs2.pfnShowConnectionWarning.expect("non-null function pointer")();
        }
        Con_DPrintf(b"^3Warning:^7 Too many lost packets! Showing Network options menu\n\x00"
                        as *const u8 as *const libc::c_char);
    };
}
/*
=================
UI_ShowConnectionWarning

show update dialog
=================
*/
#[no_mangle]
pub unsafe extern "C" fn UI_ShowUpdateDialog(mut preferStore: qboolean) {
    if gameui.dllFuncs2.pfnShowUpdateDialog.is_some() {
        gameui.dllFuncs2.pfnShowUpdateDialog.expect("non-null function pointer")(preferStore
                                                                                     as
                                                                                     libc::c_int);
    }
    Con_Printf(b"^3Warning:^7 This version is not supported anymore. To continue, install latest engine version\n\x00"
                   as *const u8 as *const libc::c_char);
}
/*
=================
UI_ShowConnectionWarning

show message box
=================
*/
#[no_mangle]
pub unsafe extern "C" fn UI_ShowMessageBox(mut text: *const libc::c_char) {
    if gameui.dllFuncs2.pfnShowMessageBox.is_some() {
        gameui.dllFuncs2.pfnShowMessageBox.expect("non-null function pointer")(text);
    };
}
#[no_mangle]
pub unsafe extern "C" fn UI_ConnectionProgress_Disconnect() {
    if gameui.dllFuncs2.pfnConnectionProgress_Disconnect.is_some() {
        gameui.dllFuncs2.pfnConnectionProgress_Disconnect.expect("non-null function pointer")();
    };
}
#[no_mangle]
pub unsafe extern "C" fn UI_ConnectionProgress_Download(mut pszFileName:
                                                            *const libc::c_char,
                                                        mut pszServerName:
                                                            *const libc::c_char,
                                                        mut pszServerPath:
                                                            *const libc::c_char,
                                                        mut iCurrent:
                                                            libc::c_int,
                                                        mut iTotal:
                                                            libc::c_int,
                                                        mut comment:
                                                            *const libc::c_char) {
    if gameui.dllFuncs2.pfnConnectionProgress_Download.is_none() { return }
    if !pszServerPath.is_null() {
        let mut serverpath: [libc::c_char; 1024] = [0; 1024];
        Q_snprintf(serverpath.as_mut_ptr(),
                   ::std::mem::size_of::<[libc::c_char; 1024]>() as
                       libc::c_ulong,
                   b"%s%s\x00" as *const u8 as *const libc::c_char,
                   pszServerName, pszServerPath);
        gameui.dllFuncs2.pfnConnectionProgress_Download.expect("non-null function pointer")(pszFileName,
                                                                                            serverpath.as_mut_ptr(),
                                                                                            iCurrent,
                                                                                            iTotal,
                                                                                            comment);
    } else {
        gameui.dllFuncs2.pfnConnectionProgress_Download.expect("non-null function pointer")(pszFileName,
                                                                                            pszServerName,
                                                                                            iCurrent,
                                                                                            iTotal,
                                                                                            comment);
    };
}
#[no_mangle]
pub unsafe extern "C" fn UI_ConnectionProgress_DownloadEnd() {
    if gameui.dllFuncs2.pfnConnectionProgress_DownloadEnd.is_some() {
        gameui.dllFuncs2.pfnConnectionProgress_DownloadEnd.expect("non-null function pointer")();
    };
}
#[no_mangle]
pub unsafe extern "C" fn UI_ConnectionProgress_Precache() {
    if gameui.dllFuncs2.pfnConnectionProgress_Precache.is_some() {
        gameui.dllFuncs2.pfnConnectionProgress_Precache.expect("non-null function pointer")();
    };
}
#[no_mangle]
pub unsafe extern "C" fn UI_ConnectionProgress_Connect(mut server:
                                                           *const libc::c_char) 
 // NULL for local server
 {
    if gameui.dllFuncs2.pfnConnectionProgress_Connect.is_some() {
        gameui.dllFuncs2.pfnConnectionProgress_Connect.expect("non-null function pointer")(server);
    };
}
#[no_mangle]
pub unsafe extern "C" fn UI_ConnectionProgress_ChangeLevel() {
    if gameui.dllFuncs2.pfnConnectionProgress_ChangeLevel.is_some() {
        gameui.dllFuncs2.pfnConnectionProgress_ChangeLevel.expect("non-null function pointer")();
    };
}
#[no_mangle]
pub unsafe extern "C" fn UI_ConnectionProgress_ParseServerInfo(mut server:
                                                                   *const libc::c_char) {
    if gameui.dllFuncs2.pfnConnectionProgress_ParseServerInfo.is_some() {
        gameui.dllFuncs2.pfnConnectionProgress_ParseServerInfo.expect("non-null function pointer")(server);
    };
}
unsafe extern "C" fn UI_DrawLogo(mut filename: *const libc::c_char,
                                 mut x: libc::c_float, mut y: libc::c_float,
                                 mut width: libc::c_float,
                                 mut height: libc::c_float) {
    static mut cin_time: libc::c_float = 0.;
    static mut last_frame: libc::c_int = -(1 as libc::c_int);
    let mut cin_data: *mut byte = 0 as *mut byte;
    let mut cin_state: *mut movie_state_t = 0 as *mut movie_state_t;
    let mut cin_frame: libc::c_int = 0;
    let mut redraw: qboolean = false_0;
    if gameui.drawLogo as u64 == 0 { return }
    cin_state = AVI_GetState(1 as libc::c_int);
    if AVI_IsActive(cin_state) as u64 == 0 {
        let mut path: string = [0; 256];
        let mut fullpath: *const libc::c_char = 0 as *const libc::c_char;
        // run cinematic if not
        Q_snprintf(path.as_mut_ptr(),
                   ::std::mem::size_of::<string>() as libc::c_ulong,
                   b"media/%s\x00" as *const u8 as *const libc::c_char,
                   filename);
        COM_DefaultExtension(path.as_mut_ptr(),
                             b".avi\x00" as *const u8 as *const libc::c_char);
        fullpath = FS_GetDiskPath(path.as_mut_ptr(), false_0);
        if FS_FileExists(path.as_mut_ptr(), false_0 as libc::c_int) != 0 &&
               fullpath.is_null() {
            Con_Printf(b"^1Error:^7 Couldn\'t load %s from packfile. Please extract it\n\x00"
                           as *const u8 as *const libc::c_char,
                       path.as_mut_ptr());
            gameui.drawLogo = false_0;
            return
        }
        AVI_OpenVideo(cin_state, fullpath, false_0, true_0 as libc::c_int);
        if AVI_GetVideoInfo(cin_state, &mut gameui.logo_xres,
                            &mut gameui.logo_yres, &mut gameui.logo_length) as
               u64 == 0 {
            AVI_CloseVideo(cin_state);
            gameui.drawLogo = false_0;
            return
        }
        cin_time = 0.0f32;
        last_frame = -(1 as libc::c_int)
    }
    if width <= 0 as libc::c_int as libc::c_float ||
           height <= 0 as libc::c_int as libc::c_float {
        // precache call, don't draw
        cin_time = 0.0f32;
        last_frame = -(1 as libc::c_int);
        return
    }
    // advances cinematic time (ignores maxfps and host_framerate settings)
    cin_time =
        (cin_time as libc::c_double + host.realframetime) as libc::c_float;
    // restarts the cinematic
    if cin_time > gameui.logo_length { cin_time = 0.0f32 }
    // read the next frame
    cin_frame = AVI_GetVideoFrameNumber(cin_state, cin_time);
    if cin_frame != last_frame {
        cin_data = AVI_GetVideoFrame(cin_state, cin_frame);
        last_frame = cin_frame;
        redraw = true_0
    }
    ref_0.dllFuncs.R_DrawStretchRaw.expect("non-null function pointer")(x, y,
                                                                        width,
                                                                        height,
                                                                        gameui.logo_xres,
                                                                        gameui.logo_yres,
                                                                        cin_data,
                                                                        redraw);
}
unsafe extern "C" fn UI_GetLogoWidth() -> libc::c_int {
    return gameui.logo_xres;
}
unsafe extern "C" fn UI_GetLogoHeight() -> libc::c_int {
    return gameui.logo_yres;
}
unsafe extern "C" fn UI_GetLogoLength() -> libc::c_float {
    return gameui.logo_length;
}
/*
cl_menu.c - menu dlls interaction
Copyright (C) 2010 Uncle Mike

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
*/
// !!svgame.hInstance
unsafe extern "C" fn UI_UpdateUserinfo() {
    let mut player: *mut player_info_t = 0 as *mut player_info_t;
    if host.userinfo_changed as u64 == 0 { return }
    player = &mut gameui.playerinfo;
    Q_strncpy((*player).userinfo.as_mut_ptr(), cls.userinfo.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong);
    Q_strncpy((*player).name.as_mut_ptr(),
              Info_ValueForKey((*player).userinfo.as_mut_ptr(),
                               b"name\x00" as *const u8 as
                                   *const libc::c_char),
              ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong);
    Q_strncpy((*player).model.as_mut_ptr(),
              Info_ValueForKey((*player).userinfo.as_mut_ptr(),
                               b"model\x00" as *const u8 as
                                   *const libc::c_char),
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    (*player).topcolor =
        Q_atoi(Info_ValueForKey((*player).userinfo.as_mut_ptr(),
                                b"topcolor\x00" as *const u8 as
                                    *const libc::c_char));
    (*player).bottomcolor =
        Q_atoi(Info_ValueForKey((*player).userinfo.as_mut_ptr(),
                                b"bottomcolor\x00" as *const u8 as
                                    *const libc::c_char));
    host.userinfo_changed = false_0;
    // we got it
}
#[no_mangle]
pub unsafe extern "C" fn Host_Credits() {
    if gameui.hInstance.is_null() { return }
    gameui.dllFuncs.pfnFinalCredits.expect("non-null function pointer")();
}
unsafe extern "C" fn UI_ConvertGameInfo(mut out: *mut GAMEINFO,
                                        mut in_0: *mut gameinfo_t) {
    Q_strncpy((*out).gamefolder.as_mut_ptr(), (*in_0).gamefolder.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    Q_strncpy((*out).startmap.as_mut_ptr(), (*in_0).startmap.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    Q_strncpy((*out).trainmap.as_mut_ptr(), (*in_0).trainmap.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    Q_strncpy((*out).title.as_mut_ptr(), (*in_0).title.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    Q_snprintf((*out).version.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong,
               b"%g\x00" as *const u8 as *const libc::c_char,
               (*in_0).version as libc::c_double);
    Q_strncpy((*out).game_url.as_mut_ptr(), (*in_0).game_url.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong);
    Q_strncpy((*out).update_url.as_mut_ptr(), (*in_0).update_url.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong);
    Q_strncpy((*out).size.as_mut_ptr(),
              Q_pretifymem((*in_0).size as libc::c_float, 0 as libc::c_int),
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    Q_strncpy((*out).type_0.as_mut_ptr(), (*in_0).type_0.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    Q_strncpy((*out).date.as_mut_ptr(), (*in_0).date.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    (*out).gamemode = (*in_0).gamemode;
    if (*in_0).nomodels as u64 != 0 {
        (*out).flags =
            ((*out).flags as libc::c_int |
                 (1 as libc::c_int) << 0 as libc::c_int) as libc::c_short
    }
    if (*in_0).noskills as u64 != 0 {
        (*out).flags =
            ((*out).flags as libc::c_int |
                 (1 as libc::c_int) << 1 as libc::c_int) as libc::c_short
    };
}
unsafe extern "C" fn PIC_Scissor(mut x: *mut libc::c_float,
                                 mut y: *mut libc::c_float,
                                 mut width: *mut libc::c_float,
                                 mut height: *mut libc::c_float,
                                 mut u0: *mut libc::c_float,
                                 mut v0: *mut libc::c_float,
                                 mut u1: *mut libc::c_float,
                                 mut v1: *mut libc::c_float) -> qboolean {
    let mut dudx: libc::c_float = 0.;
    let mut dvdy: libc::c_float = 0.;
    // clip sub rect to sprite
    if width.is_null() || height.is_null() { return false_0 }
    if *x + *width <= gameui.ds.scissor_x as libc::c_float { return false_0 }
    if *x >= (gameui.ds.scissor_x + gameui.ds.scissor_width) as libc::c_float
       {
        return false_0
    }
    if *y + *height <= gameui.ds.scissor_y as libc::c_float { return false_0 }
    if *y >= (gameui.ds.scissor_y + gameui.ds.scissor_height) as libc::c_float
       {
        return false_0
    }
    dudx = (*u1 - *u0) / *width;
    dvdy = (*v1 - *v0) / *height;
    if *x < gameui.ds.scissor_x as libc::c_float {
        *u0 += (gameui.ds.scissor_x as libc::c_float - *x) * dudx;
        *width -= gameui.ds.scissor_x as libc::c_float - *x;
        *x = gameui.ds.scissor_x as libc::c_float
    }
    if *x + *width >
           (gameui.ds.scissor_x + gameui.ds.scissor_width) as libc::c_float {
        *u1 -=
            (*x + *width -
                 (gameui.ds.scissor_x + gameui.ds.scissor_width) as
                     libc::c_float) * dudx;
        *width =
            (gameui.ds.scissor_x + gameui.ds.scissor_width) as libc::c_float -
                *x
    }
    if *y < gameui.ds.scissor_y as libc::c_float {
        *v0 += (gameui.ds.scissor_y as libc::c_float - *y) * dvdy;
        *height -= gameui.ds.scissor_y as libc::c_float - *y;
        *y = gameui.ds.scissor_y as libc::c_float
    }
    if *y + *height >
           (gameui.ds.scissor_y + gameui.ds.scissor_height) as libc::c_float {
        *v1 -=
            (*y + *height -
                 (gameui.ds.scissor_y + gameui.ds.scissor_height) as
                     libc::c_float) * dvdy;
        *height =
            (gameui.ds.scissor_y + gameui.ds.scissor_height) as libc::c_float
                - *y
    }
    return true_0;
}
/*
====================
PIC_DrawGeneric

draw hudsprite routine
====================
*/
unsafe extern "C" fn PIC_DrawGeneric(mut x: libc::c_float,
                                     mut y: libc::c_float,
                                     mut width: libc::c_float,
                                     mut height: libc::c_float,
                                     mut prc: *const wrect_t) {
    let mut s1: libc::c_float = 0.;
    let mut s2: libc::c_float = 0.;
    let mut t1: libc::c_float = 0.;
    let mut t2: libc::c_float = 0.;
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    // assume we get sizes from image
    R_GetTextureParms(&mut w, &mut h, gameui.ds.gl_texturenum);
    if !prc.is_null() {
        // calc user-defined rectangle
        s1 = (*prc).left as libc::c_float / w as libc::c_float;
        t1 = (*prc).top as libc::c_float / h as libc::c_float;
        s2 = (*prc).right as libc::c_float / w as libc::c_float;
        t2 = (*prc).bottom as libc::c_float / h as libc::c_float;
        if width == -(1 as libc::c_int) as libc::c_float &&
               height == -(1 as libc::c_int) as libc::c_float {
            width = ((*prc).right - (*prc).left) as libc::c_float;
            height = ((*prc).bottom - (*prc).top) as libc::c_float
        }
    } else { t1 = 0.0f32; s1 = t1; t2 = 1.0f32; s2 = t2 }
    if width == -(1 as libc::c_int) as libc::c_float &&
           height == -(1 as libc::c_int) as libc::c_float {
        width = w as libc::c_float;
        height = h as libc::c_float
    }
    // pass scissor test if supposed
    if gameui.ds.scissor_test as libc::c_uint != 0 &&
           PIC_Scissor(&mut x, &mut y, &mut width, &mut height, &mut s1,
                       &mut t1, &mut s2, &mut t2) as u64 == 0 {
        return
    }
    PicAdjustSize(&mut x, &mut y, &mut width, &mut height);
    ref_0.dllFuncs.R_DrawStretchPic.expect("non-null function pointer")(x, y,
                                                                        width,
                                                                        height,
                                                                        s1,
                                                                        t1,
                                                                        s2,
                                                                        t2,
                                                                        gameui.ds.gl_texturenum);
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
===============================================================================
	MainUI Builtin Functions

===============================================================================
*/
/*
=========
pfnPIC_Load

=========
*/
unsafe extern "C" fn pfnPIC_Load(mut szPicName: *const libc::c_char,
                                 mut image_buf: *const byte,
                                 mut image_size: libc::c_int,
                                 mut flags: libc::c_int) -> HIMAGE {
    let mut tx: HIMAGE = 0;
    if if szPicName.is_null() || *szPicName == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        Con_Reportf(b"^1Error:^7 CL_LoadImage: refusing to load image with empty name\n\x00"
                        as *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    // add default parms to image
    flags =
        flags |
            (TF_NOMIPMAP as libc::c_int |
                 TF_CLAMP as libc::c_int); // allow decal images for menu
    Image_SetForceFlags(IL_LOAD_DECAL as libc::c_int as uint);
    tx =
        ref_0.dllFuncs.GL_LoadTexture.expect("non-null function pointer")(szPicName,
                                                                          image_buf,
                                                                          image_size
                                                                              as
                                                                              size_t,
                                                                          flags);
    Image_ClearForceFlags();
    return tx;
}
/*
=========
pfnPIC_Width

=========
*/
unsafe extern "C" fn pfnPIC_Width(mut hPic: HIMAGE) -> libc::c_int {
    let mut picWidth: libc::c_int = 0;
    R_GetTextureParms(&mut picWidth, 0 as *mut libc::c_int, hPic);
    return picWidth;
}
/*
=========
pfnPIC_Height

=========
*/
unsafe extern "C" fn pfnPIC_Height(mut hPic: HIMAGE) -> libc::c_int {
    let mut picHeight: libc::c_int = 0;
    R_GetTextureParms(0 as *mut libc::c_int, &mut picHeight, hPic);
    return picHeight;
}
/*
=========
pfnPIC_Set

=========
*/
#[no_mangle]
pub unsafe extern "C" fn pfnPIC_Set(mut hPic: HIMAGE, mut r: libc::c_int,
                                    mut g: libc::c_int, mut b: libc::c_int,
                                    mut a: libc::c_int) {
    gameui.ds.gl_texturenum = hPic;
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
    a =
        if a >= 0 as libc::c_int {
            if a < 255 as libc::c_int { a } else { 255 as libc::c_int }
        } else { 0 as libc::c_int };
    ref_0.dllFuncs.Color4ub.expect("non-null function pointer")(r as
                                                                    libc::c_uchar,
                                                                g as
                                                                    libc::c_uchar,
                                                                b as
                                                                    libc::c_uchar,
                                                                a as
                                                                    libc::c_uchar);
}
/*
=========
pfnPIC_Draw

=========
*/
#[no_mangle]
pub unsafe extern "C" fn pfnPIC_Draw(mut x: libc::c_int, mut y: libc::c_int,
                                     mut width: libc::c_int,
                                     mut height: libc::c_int,
                                     mut prc: *const wrect_t) {
    ref_0.dllFuncs.GL_SetRenderMode.expect("non-null function pointer")(kRenderNormal
                                                                            as
                                                                            libc::c_int);
    PIC_DrawGeneric(x as libc::c_float, y as libc::c_float,
                    width as libc::c_float, height as libc::c_float, prc);
}
/*
=========
pfnPIC_DrawTrans

=========
*/
#[no_mangle]
pub unsafe extern "C" fn pfnPIC_DrawTrans(mut x: libc::c_int,
                                          mut y: libc::c_int,
                                          mut width: libc::c_int,
                                          mut height: libc::c_int,
                                          mut prc: *const wrect_t) {
    ref_0.dllFuncs.GL_SetRenderMode.expect("non-null function pointer")(kRenderTransTexture
                                                                            as
                                                                            libc::c_int);
    PIC_DrawGeneric(x as libc::c_float, y as libc::c_float,
                    width as libc::c_float, height as libc::c_float, prc);
}
/*
=========
pfnPIC_DrawHoles

=========
*/
#[no_mangle]
pub unsafe extern "C" fn pfnPIC_DrawHoles(mut x: libc::c_int,
                                          mut y: libc::c_int,
                                          mut width: libc::c_int,
                                          mut height: libc::c_int,
                                          mut prc: *const wrect_t) {
    ref_0.dllFuncs.GL_SetRenderMode.expect("non-null function pointer")(kRenderTransAlpha
                                                                            as
                                                                            libc::c_int);
    PIC_DrawGeneric(x as libc::c_float, y as libc::c_float,
                    width as libc::c_float, height as libc::c_float, prc);
}
/*
=========
pfnPIC_DrawAdditive

=========
*/
#[no_mangle]
pub unsafe extern "C" fn pfnPIC_DrawAdditive(mut x: libc::c_int,
                                             mut y: libc::c_int,
                                             mut width: libc::c_int,
                                             mut height: libc::c_int,
                                             mut prc: *const wrect_t) {
    ref_0.dllFuncs.GL_SetRenderMode.expect("non-null function pointer")(kRenderTransAdd
                                                                            as
                                                                            libc::c_int);
    PIC_DrawGeneric(x as libc::c_float, y as libc::c_float,
                    width as libc::c_float, height as libc::c_float, prc);
}
/*
=========
pfnPIC_EnableScissor

=========
*/
unsafe extern "C" fn pfnPIC_EnableScissor(mut x: libc::c_int,
                                          mut y: libc::c_int,
                                          mut width: libc::c_int,
                                          mut height: libc::c_int) {
    // check bounds
    x =
        if x >= 0 as libc::c_int {
            if x < (*gameui.globals).scrWidth {
                x
            } else { (*gameui.globals).scrWidth }
        } else { 0 as libc::c_int };
    y =
        if y >= 0 as libc::c_int {
            if y < (*gameui.globals).scrHeight {
                y
            } else { (*gameui.globals).scrHeight }
        } else { 0 as libc::c_int };
    width =
        if width >= 0 as libc::c_int {
            if width < (*gameui.globals).scrWidth - x {
                width
            } else { ((*gameui.globals).scrWidth) - x }
        } else { 0 as libc::c_int };
    height =
        if height >= 0 as libc::c_int {
            if height < (*gameui.globals).scrHeight - y {
                height
            } else { ((*gameui.globals).scrHeight) - y }
        } else { 0 as libc::c_int };
    gameui.ds.scissor_x = x;
    gameui.ds.scissor_width = width;
    gameui.ds.scissor_y = y;
    gameui.ds.scissor_height = height;
    gameui.ds.scissor_test = true_0;
}
/*
=========
pfnPIC_DisableScissor

=========
*/
unsafe extern "C" fn pfnPIC_DisableScissor() {
    gameui.ds.scissor_x = 0 as libc::c_int;
    gameui.ds.scissor_width = 0 as libc::c_int;
    gameui.ds.scissor_y = 0 as libc::c_int;
    gameui.ds.scissor_height = 0 as libc::c_int;
    gameui.ds.scissor_test = false_0;
}
/*
=============
pfnFillRGBA

=============
*/
unsafe extern "C" fn pfnFillRGBA(mut x: libc::c_int, mut y: libc::c_int,
                                 mut width: libc::c_int,
                                 mut height: libc::c_int, mut r: libc::c_int,
                                 mut g: libc::c_int, mut b: libc::c_int,
                                 mut a: libc::c_int) {
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
    a =
        if a >= 0 as libc::c_int {
            if a < 255 as libc::c_int { a } else { 255 as libc::c_int }
        } else { 0 as libc::c_int };
    ref_0.dllFuncs.Color4ub.expect("non-null function pointer")(r as
                                                                    libc::c_uchar,
                                                                g as
                                                                    libc::c_uchar,
                                                                b as
                                                                    libc::c_uchar,
                                                                a as
                                                                    libc::c_uchar);
    ref_0.dllFuncs.GL_SetRenderMode.expect("non-null function pointer")(kRenderTransTexture
                                                                            as
                                                                            libc::c_int);
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
                                                                        ref_0.dllFuncs.GL_LoadTexture.expect("non-null function pointer")(b"*white\x00"
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
=============
pfnClientCmd

=============
*/
unsafe extern "C" fn pfnClientCmd(mut exec_now: libc::c_int,
                                  mut szCmdString: *const libc::c_char) {
    if szCmdString.is_null() ||
           *szCmdString.offset(0 as libc::c_int as isize) == 0 {
        return
    }
    Cbuf_AddText(szCmdString);
    Cbuf_AddText(b"\n\x00" as *const u8 as *const libc::c_char);
    // client command executes immediately
    if exec_now != 0 { Cbuf_Execute(); };
}
/*
=============
pfnPlaySound

=============
*/
unsafe extern "C" fn pfnPlaySound(mut szSound: *const libc::c_char) {
    if if szSound.is_null() || *szSound == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return
    }
    S_StartLocalSound(szSound, 1.0f64 as libc::c_float, false_0);
}
/*
=============
pfnDrawCharacter

quakefont draw character
=============
*/
unsafe extern "C" fn pfnDrawCharacter(mut ix: libc::c_int,
                                      mut iy: libc::c_int,
                                      mut iwidth: libc::c_int,
                                      mut iheight: libc::c_int,
                                      mut ch: libc::c_int,
                                      mut ulRGBA: libc::c_int,
                                      mut hFont: HIMAGE) {
    let mut color: rgba_t = [0; 4];
    let mut row: libc::c_float = 0.;
    let mut col: libc::c_float = 0.;
    let mut size: libc::c_float = 0.;
    let mut s1: libc::c_float = 0.;
    let mut t1: libc::c_float = 0.;
    let mut s2: libc::c_float = 0.;
    let mut t2: libc::c_float = 0.;
    let mut x: libc::c_float = ix as libc::c_float;
    let mut y: libc::c_float = iy as libc::c_float;
    let mut width: libc::c_float = iwidth as libc::c_float;
    let mut height: libc::c_float = iheight as libc::c_float;
    ch &= 255 as libc::c_int;
    if ch == ' ' as i32 { return }
    if y < -height { return }
    color[3 as libc::c_int as usize] =
        ((ulRGBA as libc::c_uint & 0xff000000 as libc::c_uint) >>
             24 as libc::c_int) as byte;
    color[0 as libc::c_int as usize] =
        ((ulRGBA & 0xff0000 as libc::c_int) >> 16 as libc::c_int) as byte;
    color[1 as libc::c_int as usize] =
        ((ulRGBA & 0xff00 as libc::c_int) >> 8 as libc::c_int) as byte;
    color[2 as libc::c_int as usize] =
        ((ulRGBA & 0xff as libc::c_int) >> 0 as libc::c_int) as byte;
    ref_0.dllFuncs.Color4ub.expect("non-null function pointer")(color[0 as
                                                                          libc::c_int
                                                                          as
                                                                          usize],
                                                                color[1 as
                                                                          libc::c_int
                                                                          as
                                                                          usize],
                                                                color[2 as
                                                                          libc::c_int
                                                                          as
                                                                          usize],
                                                                color[3 as
                                                                          libc::c_int
                                                                          as
                                                                          usize]);
    col =
        (ch & 15 as libc::c_int) as libc::c_float * 0.0625f32 +
            0.5f32 / 256.0f32;
    row =
        (ch >> 4 as libc::c_int) as libc::c_float * 0.0625f32 +
            0.5f32 / 256.0f32;
    size = 0.0625f32 - 1.0f32 / 256.0f32;
    s1 = col;
    t1 = row;
    s2 = s1 + size;
    t2 = t1 + size;
    // pass scissor test if supposed
    if gameui.ds.scissor_test as libc::c_uint != 0 &&
           PIC_Scissor(&mut x, &mut y, &mut width, &mut height, &mut s1,
                       &mut t1, &mut s2, &mut t2) as u64 == 0 {
        return
    }
    ref_0.dllFuncs.GL_SetRenderMode.expect("non-null function pointer")(kRenderTransTexture
                                                                            as
                                                                            libc::c_int);
    ref_0.dllFuncs.R_DrawStretchPic.expect("non-null function pointer")(x, y,
                                                                        width,
                                                                        height,
                                                                        s1,
                                                                        t1,
                                                                        s2,
                                                                        t2,
                                                                        hFont);
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
=============
UI_DrawConsoleString

drawing string like a console string
=============
*/
unsafe extern "C" fn UI_DrawConsoleString(mut x: libc::c_int,
                                          mut y: libc::c_int,
                                          mut string: *const libc::c_char)
 -> libc::c_int {
    let mut drawLen: libc::c_int = 0; // silent ignore
    if string.is_null() || *string == 0 { return 0 as libc::c_int }
    drawLen = Con_DrawString(x, y, string, gameui.ds.textColor.as_mut_ptr());
    gameui.ds.textColor[0 as libc::c_int as usize] =
        255 as libc::c_int as byte;
    gameui.ds.textColor[1 as libc::c_int as usize] =
        255 as libc::c_int as byte;
    gameui.ds.textColor[2 as libc::c_int as usize] =
        255 as libc::c_int as byte;
    gameui.ds.textColor[3 as libc::c_int as usize] =
        255 as libc::c_int as byte;
    return x + drawLen;
    // exclude color prexfixes
}
/*
=============
pfnDrawSetTextColor

set color for anything
=============
*/
unsafe extern "C" fn UI_DrawSetTextColor(mut r: libc::c_int,
                                         mut g: libc::c_int,
                                         mut b: libc::c_int,
                                         mut alpha: libc::c_int) {
    // bound color and convert to byte
    gameui.ds.textColor[0 as libc::c_int as usize] = r as byte;
    gameui.ds.textColor[1 as libc::c_int as usize] = g as byte;
    gameui.ds.textColor[2 as libc::c_int as usize] = b as byte;
    gameui.ds.textColor[3 as libc::c_int as usize] = alpha as byte;
}
/*
====================
pfnGetPlayerModel

for drawing playermodel previews
====================
*/
unsafe extern "C" fn pfnGetPlayerModel() -> *mut cl_entity_t {
    return &mut gameui.playermodel;
}
/*
====================
pfnSetPlayerModel

for drawing playermodel previews
====================
*/
unsafe extern "C" fn pfnSetPlayerModel(mut ent: *mut cl_entity_t,
                                       mut path: *const libc::c_char) {
    (*ent).model = Mod_ForName(path, false_0, false_0);
    (*ent).curstate.modelindex = 1024 as libc::c_int;
    // unreachable index
}
/*
====================
pfnClearScene

for drawing playermodel previews
====================
*/
unsafe extern "C" fn pfnClearScene() {
    ref_0.dllFuncs.R_PushScene.expect("non-null function pointer")();
    ref_0.dllFuncs.R_ClearScene.expect("non-null function pointer")();
}
/*
====================
pfnRenderScene

for drawing playermodel previews
====================
*/
unsafe extern "C" fn pfnRenderScene(mut rvp: *const ref_viewpass_t) {
    let mut copy: ref_viewpass_t =
        ref_viewpass_t{viewport: [0; 4],
                       vieworigin: [0.; 3],
                       viewangles: [0.; 3],
                       viewentity: 0,
                       fov_x: 0.,
                       fov_y: 0.,
                       flags: 0,};
    // to avoid division by zero
    if rvp.is_null() || (*rvp).fov_x <= 0.0f32 || (*rvp).fov_y <= 0.0f32 {
        return
    }
    copy = *rvp;
    // don't allow special modes from menu
    copy.flags = 0 as libc::c_int;
    ref_0.dllFuncs.R_Set2DMode.expect("non-null function pointer")(false_0);
    GL_RenderFrame(&mut copy);
    ref_0.dllFuncs.R_Set2DMode.expect("non-null function pointer")(true_0);
    ref_0.dllFuncs.R_PopScene.expect("non-null function pointer")();
}
/*
====================
pfnAddEntity

adding player model into visible list
====================
*/
unsafe extern "C" fn pfnAddEntity(mut entityType: libc::c_int,
                                  mut ent: *mut cl_entity_t) -> libc::c_int {
    if ref_0.dllFuncs.R_AddEntity.expect("non-null function pointer")(ent,
                                                                      entityType)
           as u64 == 0 {
        return false_0 as libc::c_int
    }
    return true_0 as libc::c_int;
}
/*
====================
pfnClientJoin

send client connect
====================
*/
unsafe extern "C" fn pfnClientJoin(adr: netadr_t) {
    Cbuf_AddText(va(b"connect %s\n\x00" as *const u8 as *const libc::c_char,
                    NET_AdrToString(adr)));
}
/*
====================
pfnKeyGetOverstrikeMode

get global key overstrike state
====================
*/
unsafe extern "C" fn pfnKeyGetOverstrikeMode() -> libc::c_int {
    return host.key_overstrike as libc::c_int;
}
/*
====================
pfnKeySetOverstrikeMode

set global key overstrike mode
====================
*/
unsafe extern "C" fn pfnKeySetOverstrikeMode(mut fActive: libc::c_int) {
    host.key_overstrike = fActive as qboolean;
}
/*
====================
pfnKeyGetState

returns kbutton struct if found
====================
*/
unsafe extern "C" fn pfnKeyGetState(mut name: *const libc::c_char)
 -> *mut libc::c_void {
    if clgame.dllFuncs.KB_Find.is_some() {
        return clgame.dllFuncs.KB_Find.expect("non-null function pointer")(name)
    }
    return 0 as *mut libc::c_void;
}
/*
=========
pfnMemAlloc

=========
*/
unsafe extern "C" fn pfnMemAlloc(mut cb: size_t,
                                 mut filename: *const libc::c_char,
                                 fileline: libc::c_int) -> *mut libc::c_void {
    return _Mem_Alloc(gameui.mempool, cb, true_0, filename, fileline);
}
/*
=========
pfnMemFree

=========
*/
unsafe extern "C" fn pfnMemFree(mut mem: *mut libc::c_void,
                                mut filename: *const libc::c_char,
                                fileline: libc::c_int) {
    _Mem_Free(mem, filename, fileline);
}
/*
=========
pfnGetGameInfo

=========
*/
unsafe extern "C" fn pfnGetGameInfo(mut pgameinfo: *mut GAMEINFO)
 -> libc::c_int {
    if pgameinfo.is_null() { return 0 as libc::c_int }
    *pgameinfo = gameui.gameInfo;
    return 1 as libc::c_int;
}
/*
=========
pfnGetGamesList

=========
*/
unsafe extern "C" fn pfnGetGamesList(mut numGames: *mut libc::c_int)
 -> *mut *mut GAMEINFO {
    if !numGames.is_null() { *numGames = SI.numgames }
    return gameui.modsInfo.as_mut_ptr();
}
/*
=========
pfnGetFilesList

release prev search on a next call
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
                  b"../engine/client/cl_gameui.c\x00" as *const u8 as
                      *const libc::c_char, 996 as libc::c_int);
    }
    t = FS_Search(pattern, true_0 as libc::c_int, gamedironly);
    if t.is_null() {
        if !numFiles.is_null() { *numFiles = 0 as libc::c_int }
        return 0 as *mut *mut libc::c_char
    }
    if !numFiles.is_null() { *numFiles = (*t).numfilenames }
    return (*t).filenames;
}
/*
=========
pfnGetClipboardData

pointer must be released in call place
=========
*/
unsafe extern "C" fn pfnGetClipboardData() -> *mut libc::c_char {
    return Sys_GetClipboardData();
}
/*
=========
pfnCheckGameDll

=========
*/
#[no_mangle]
pub unsafe extern "C" fn pfnCheckGameDll() -> libc::c_int {
    let mut dllpath: string =
        [0; 256]; // don't increase linker's reference counter
    let mut hInst: *mut libc::c_void = 0 as *mut libc::c_void;
    if !svgame.hInstance.is_null() { return true_0 as libc::c_int }
    COM_GetCommonLibraryPath(LIBRARY_SERVER, dllpath.as_mut_ptr(),
                             ::std::mem::size_of::<string>() as
                                 libc::c_ulong);
    hInst =
        COM_LoadLibrary(dllpath.as_mut_ptr(), true_0 as libc::c_int, false_0);
    if !hInst.is_null() {
        COM_FreeLibrary(hInst);
        return true_0 as libc::c_int
    }
    Con_Reportf(b"^3Warning:^7 Could not load server library: %s\n\x00" as
                    *const u8 as *const libc::c_char, COM_GetLibraryError());
    return false_0 as libc::c_int;
}
/*
=========
pfnChangeInstance

=========
*/
unsafe extern "C" fn pfnChangeInstance(mut newInstance: *const libc::c_char,
                                       mut szFinalMessage:
                                           *const libc::c_char) {
    if szFinalMessage.is_null() {
        szFinalMessage = b"\x00" as *const u8 as *const libc::c_char
    }
    if newInstance.is_null() || *newInstance == 0 { return }
    Host_NewInstance(newInstance, szFinalMessage);
}
/*
=========
pfnHostEndGame

=========
*/
unsafe extern "C" fn pfnHostEndGame(mut szFinalMessage: *const libc::c_char) {
    if szFinalMessage.is_null() {
        szFinalMessage = b"\x00" as *const u8 as *const libc::c_char
    }
    Host_EndGame(false_0, b"%s\x00" as *const u8 as *const libc::c_char,
                 szFinalMessage);
}
/*
=========
pfnStartBackgroundTrack

=========
*/
unsafe extern "C" fn pfnStartBackgroundTrack(mut introTrack:
                                                 *const libc::c_char,
                                             mut mainTrack:
                                                 *const libc::c_char) {
    S_StartBackgroundTrack(introTrack, mainTrack, 0 as libc::c_int, false_0);
}
unsafe extern "C" fn GL_ProcessTexture(mut texnum: libc::c_int,
                                       mut gamma: libc::c_float,
                                       mut topColor: libc::c_int,
                                       mut bottomColor: libc::c_int) {
    ref_0.dllFuncs.GL_ProcessTexture.expect("non-null function pointer")(texnum,
                                                                         gamma,
                                                                         topColor,
                                                                         bottomColor);
}
/*
=================
UI_ShellExecute
=================
*/
unsafe extern "C" fn UI_ShellExecute(mut path: *const libc::c_char,
                                     mut parms: *const libc::c_char,
                                     mut shouldExit: libc::c_int) {
    Platform_ShellExecute(path, parms);
    if shouldExit != 0 { Sys_Quit(); };
}
/*
==============
pfnParseFile

legacy wrapper
==============
*/
unsafe extern "C" fn pfnParseFile(mut buf: *mut libc::c_char,
                                  mut token: *mut libc::c_char)
 -> *mut libc::c_char {
    return _COM_ParseFileSafe(buf, token, 2147483647 as libc::c_int,
                              0 as libc::c_int as libc::c_uint,
                              0 as *mut libc::c_int);
}
// engine callbacks
static mut gEngfuncs: ui_enginefuncs_t =
    unsafe {
        {
            let mut init =
                ui_enginefuncs_s{pfnPIC_Load:
                                     Some(pfnPIC_Load as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char,
                                                                   _:
                                                                       *const byte,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int)
                                                  -> HIMAGE),
                                 pfnPIC_Free:
                                     Some(GL_FreeImage as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char)
                                                  -> ()),
                                 pfnPIC_Width:
                                     Some(pfnPIC_Width as
                                              unsafe extern "C" fn(_: HIMAGE)
                                                  -> libc::c_int),
                                 pfnPIC_Height:
                                     Some(pfnPIC_Height as
                                              unsafe extern "C" fn(_: HIMAGE)
                                                  -> libc::c_int),
                                 pfnPIC_Set:
                                     Some(pfnPIC_Set as
                                              unsafe extern "C" fn(_: HIMAGE,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int)
                                                  -> ()),
                                 pfnPIC_Draw:
                                     Some(pfnPIC_Draw as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       *const wrect_t)
                                                  -> ()),
                                 pfnPIC_DrawHoles:
                                     Some(pfnPIC_DrawHoles as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       *const wrect_t)
                                                  -> ()),
                                 pfnPIC_DrawTrans:
                                     Some(pfnPIC_DrawTrans as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       *const wrect_t)
                                                  -> ()),
                                 pfnPIC_DrawAdditive:
                                     Some(pfnPIC_DrawAdditive as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       *const wrect_t)
                                                  -> ()),
                                 pfnPIC_EnableScissor:
                                     Some(pfnPIC_EnableScissor as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int)
                                                  -> ()),
                                 pfnPIC_DisableScissor:
                                     Some(pfnPIC_DisableScissor as
                                              unsafe extern "C" fn() -> ()),
                                 pfnFillRGBA:
                                     Some(pfnFillRGBA as
                                              unsafe extern "C" fn(_:
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
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int)
                                                  -> ()),
                                 pfnRegisterVariable:
                                     Some(pfnCvar_RegisterGameUIVariable as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char,
                                                                   _:
                                                                       *const libc::c_char,
                                                                   _:
                                                                       libc::c_int)
                                                  -> *mut cvar_t),
                                 pfnGetCvarFloat:
                                     Some(Cvar_VariableValue as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char)
                                                  -> libc::c_float),
                                 pfnGetCvarString:
                                     Some(Cvar_VariableString as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char)
                                                  -> *const libc::c_char),
                                 pfnCvarSetString:
                                     Some(Cvar_Set as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char,
                                                                   _:
                                                                       *const libc::c_char)
                                                  -> ()),
                                 pfnCvarSetValue:
                                     Some(Cvar_SetValue as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char,
                                                                   _:
                                                                       libc::c_float)
                                                  -> ()),
                                 pfnAddCommand:
                                     Some(Cmd_AddGameUICommand as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char,
                                                                   _:
                                                                       xcommand_t)
                                                  -> libc::c_int),
                                 pfnClientCmd:
                                     Some(pfnClientCmd as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int,
                                                                   _:
                                                                       *const libc::c_char)
                                                  -> ()),
                                 pfnDelCommand:
                                     Some(Cmd_RemoveCommand as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char)
                                                  -> ()),
                                 pfnCmdArgc:
                                     Some(Cmd_Argc as
                                              unsafe extern "C" fn()
                                                  -> libc::c_int),
                                 pfnCmdArgv:
                                     Some(Cmd_Argv as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int)
                                                  -> *const libc::c_char),
                                 pfnCmd_Args:
                                     Some(Cmd_Args as
                                              unsafe extern "C" fn()
                                                  -> *const libc::c_char),
                                 Con_Printf:
                                     Some(Con_Printf as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char,
                                                                   _: ...)
                                                  -> ()),
                                 Con_DPrintf:
                                     Some(Con_DPrintf as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char,
                                                                   _: ...)
                                                  -> ()),
                                 Con_NPrintf:
                                     Some(UI_NPrintf as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int,
                                                                   _:
                                                                       *const libc::c_char,
                                                                   _: ...)
                                                  -> ()),
                                 Con_NXPrintf:
                                     Some(UI_NXPrintf as
                                              unsafe extern "C" fn(_:
                                                                       *mut con_nprint_t,
                                                                   _:
                                                                       *const libc::c_char,
                                                                   _: ...)
                                                  -> ()),
                                 pfnPlayLocalSound:
                                     Some(pfnPlaySound as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char)
                                                  -> ()),
                                 pfnDrawLogo:
                                     Some(UI_DrawLogo as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char,
                                                                   _:
                                                                       libc::c_float,
                                                                   _:
                                                                       libc::c_float,
                                                                   _:
                                                                       libc::c_float,
                                                                   _:
                                                                       libc::c_float)
                                                  -> ()),
                                 pfnGetLogoWidth:
                                     Some(UI_GetLogoWidth as
                                              unsafe extern "C" fn()
                                                  -> libc::c_int),
                                 pfnGetLogoHeight:
                                     Some(UI_GetLogoHeight as
                                              unsafe extern "C" fn()
                                                  -> libc::c_int),
                                 pfnGetLogoLength:
                                     Some(UI_GetLogoLength as
                                              unsafe extern "C" fn()
                                                  -> libc::c_float),
                                 pfnDrawCharacter:
                                     Some(pfnDrawCharacter as
                                              unsafe extern "C" fn(_:
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
                                                                   _: HIMAGE)
                                                  -> ()),
                                 pfnDrawConsoleString:
                                     Some(UI_DrawConsoleString as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       *const libc::c_char)
                                                  -> libc::c_int),
                                 pfnDrawSetTextColor:
                                     Some(UI_DrawSetTextColor as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int)
                                                  -> ()),
                                 pfnDrawConsoleStringLen:
                                     Some(Con_DrawStringLen as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char,
                                                                   _:
                                                                       *mut libc::c_int,
                                                                   _:
                                                                       *mut libc::c_int)
                                                  -> ()),
                                 pfnSetConsoleDefaultColor:
                                     Some(Con_DefaultColor as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int)
                                                  -> ()),
                                 pfnGetPlayerModel:
                                     Some(pfnGetPlayerModel as
                                              unsafe extern "C" fn()
                                                  -> *mut cl_entity_t),
                                 pfnSetModel:
                                     Some(pfnSetPlayerModel as
                                              unsafe extern "C" fn(_:
                                                                       *mut cl_entity_t,
                                                                   _:
                                                                       *const libc::c_char)
                                                  -> ()),
                                 pfnClearScene:
                                     Some(pfnClearScene as
                                              unsafe extern "C" fn() -> ()),
                                 pfnRenderScene:
                                     Some(pfnRenderScene as
                                              unsafe extern "C" fn(_:
                                                                       *const ref_viewpass_t)
                                                  -> ()),
                                 CL_CreateVisibleEntity:
                                     Some(pfnAddEntity as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int,
                                                                   _:
                                                                       *mut cl_entity_t)
                                                  -> libc::c_int),
                                 pfnHostError:
                                     Some(Host_Error as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char,
                                                                   _: ...)
                                                  -> ()),
                                 pfnFileExists:
                                     Some(FS_FileExists as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char,
                                                                   _:
                                                                       libc::c_int)
                                                  -> libc::c_int),
                                 pfnGetGameDir:
                                     Some(pfnGetGameDir as
                                              unsafe extern "C" fn(_:
                                                                       *mut libc::c_char)
                                                  -> ()),
                                 pfnCreateMapsList:
                                     Some(Cmd_CheckMapsList as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int)
                                                  -> libc::c_int),
                                 pfnClientInGame:
                                     Some(CL_Active as
                                              unsafe extern "C" fn()
                                                  -> libc::c_int),
                                 pfnClientJoin:
                                     Some(pfnClientJoin as
                                              unsafe extern "C" fn(_:
                                                                       netadr_t)
                                                  -> ()),
                                 COM_LoadFile:
                                     Some(COM_LoadFileForMe as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char,
                                                                   _:
                                                                       *mut libc::c_int)
                                                  -> *mut byte),
                                 COM_ParseFile:
                                     Some(pfnParseFile as
                                              unsafe extern "C" fn(_:
                                                                       *mut libc::c_char,
                                                                   _:
                                                                       *mut libc::c_char)
                                                  -> *mut libc::c_char),
                                 COM_FreeFile:
                                     Some(COM_FreeFile as
                                              unsafe extern "C" fn(_:
                                                                       *mut libc::c_void)
                                                  -> ()),
                                 pfnKeyClearStates:
                                     Some(Key_ClearStates as
                                              unsafe extern "C" fn() -> ()),
                                 pfnSetKeyDest:
                                     Some(Key_SetKeyDest as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int)
                                                  -> ()),
                                 pfnKeynumToString:
                                     Some(Key_KeynumToString as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int)
                                                  -> *const libc::c_char),
                                 pfnKeyGetBinding:
                                     Some(Key_GetBinding as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int)
                                                  -> *const libc::c_char),
                                 pfnKeySetBinding:
                                     Some(Key_SetBinding as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int,
                                                                   _:
                                                                       *const libc::c_char)
                                                  -> ()),
                                 pfnKeyIsDown:
                                     Some(Key_IsDown as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int)
                                                  -> libc::c_int),
                                 pfnKeyGetOverstrikeMode:
                                     Some(pfnKeyGetOverstrikeMode as
                                              unsafe extern "C" fn()
                                                  -> libc::c_int),
                                 pfnKeySetOverstrikeMode:
                                     Some(pfnKeySetOverstrikeMode as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int)
                                                  -> ()),
                                 pfnKeyGetState:
                                     Some(pfnKeyGetState as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char)
                                                  -> *mut libc::c_void),
                                 pfnMemAlloc:
                                     Some(pfnMemAlloc as
                                              unsafe extern "C" fn(_: size_t,
                                                                   _:
                                                                       *const libc::c_char,
                                                                   _:
                                                                       libc::c_int)
                                                  -> *mut libc::c_void),
                                 pfnMemFree:
                                     Some(pfnMemFree as
                                              unsafe extern "C" fn(_:
                                                                       *mut libc::c_void,
                                                                   _:
                                                                       *const libc::c_char,
                                                                   _:
                                                                       libc::c_int)
                                                  -> ()),
                                 pfnGetGameInfo:
                                     Some(pfnGetGameInfo as
                                              unsafe extern "C" fn(_:
                                                                       *mut GAMEINFO)
                                                  -> libc::c_int),
                                 pfnGetGamesList:
                                     Some(pfnGetGamesList as
                                              unsafe extern "C" fn(_:
                                                                       *mut libc::c_int)
                                                  -> *mut *mut GAMEINFO),
                                 pfnGetFilesList:
                                     Some(pfnGetFilesList as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char,
                                                                   _:
                                                                       *mut libc::c_int,
                                                                   _:
                                                                       libc::c_int)
                                                  -> *mut *mut libc::c_char),
                                 pfnGetSaveComment:
                                     Some(SV_GetSaveComment as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char,
                                                                   _:
                                                                       *mut libc::c_char)
                                                  -> libc::c_int),
                                 pfnGetDemoComment:
                                     Some(CL_GetDemoComment as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char,
                                                                   _:
                                                                       *mut libc::c_char)
                                                  -> libc::c_int),
                                 pfnCheckGameDll:
                                     Some(pfnCheckGameDll as
                                              unsafe extern "C" fn()
                                                  -> libc::c_int),
                                 pfnGetClipboardData:
                                     Some(pfnGetClipboardData as
                                              unsafe extern "C" fn()
                                                  -> *mut libc::c_char),
                                 pfnShellExecute:
                                     Some(UI_ShellExecute as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char,
                                                                   _:
                                                                       *const libc::c_char,
                                                                   _:
                                                                       libc::c_int)
                                                  -> ()),
                                 pfnWriteServerConfig:
                                     Some(Host_WriteServerConfig as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char)
                                                  -> ()),
                                 pfnChangeInstance:
                                     Some(pfnChangeInstance as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char,
                                                                   _:
                                                                       *const libc::c_char)
                                                  -> ()),
                                 pfnPlayBackgroundTrack:
                                     Some(pfnStartBackgroundTrack as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char,
                                                                   _:
                                                                       *const libc::c_char)
                                                  -> ()),
                                 pfnHostEndGame:
                                     Some(pfnHostEndGame as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char)
                                                  -> ()),
                                 pfnRandomFloat:
                                     Some(COM_RandomFloat as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_float,
                                                                   _:
                                                                       libc::c_float)
                                                  -> libc::c_float),
                                 pfnRandomLong:
                                     Some(COM_RandomLong as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int)
                                                  -> libc::c_int),
                                 pfnSetCursor:
                                     Some(IN_SetCursor as
                                              unsafe extern "C" fn(_:
                                                                       *mut libc::c_void)
                                                  -> ()),
                                 pfnIsMapValid:
                                     Some(pfnIsMapValid as
                                              unsafe extern "C" fn(_:
                                                                       *mut libc::c_char)
                                                  -> libc::c_int),
                                 pfnProcessImage:
                                     Some(GL_ProcessTexture as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_float,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int)
                                                  -> ()),
                                 pfnCompareFileTime:
                                     Some(COM_CompareFileTime as
                                              unsafe extern "C" fn(_:
                                                                       *const libc::c_char,
                                                                   _:
                                                                       *const libc::c_char,
                                                                   _:
                                                                       *mut libc::c_int)
                                                  -> libc::c_int),
                                 pfnGetModeString:
                                     Some(VID_GetModeString as
                                              unsafe extern "C" fn(_:
                                                                       libc::c_int)
                                                  -> *const libc::c_char),
                                 COM_SaveFile:
                                     ::std::mem::transmute::<*mut libc::c_void,
                                                             Option<unsafe extern "C" fn(_:
                                                                                             *const libc::c_char,
                                                                                         _:
                                                                                             *const libc::c_void,
                                                                                         _:
                                                                                             libc::c_int)
                                                                        ->
                                                                            libc::c_int>>(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                                                                  *const libc::c_char,
                                                                                                                                              _:
                                                                                                                                                  *const libc::c_void,
                                                                                                                                              _:
                                                                                                                                                  libc::c_int)
                                                                                                                             ->
                                                                                                                                 libc::c_int>,
                                                                                                                  *mut libc::c_void>(Some(COM_SaveFile
                                                                                                                                              as
                                                                                                                                              unsafe extern "C" fn(_:
                                                                                                                                                                       *const libc::c_char,
                                                                                                                                                                   _:
                                                                                                                                                                       *const libc::c_void,
                                                                                                                                                                   _:
                                                                                                                                                                       libc::c_int)
                                                                                                                                                  ->
                                                                                                                                                      libc::c_int))),
                                 COM_RemoveFile:
                                     ::std::mem::transmute::<*mut libc::c_void,
                                                             Option<unsafe extern "C" fn(_:
                                                                                             *const libc::c_char)
                                                                        ->
                                                                            libc::c_int>>(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                                                                  *const libc::c_char)
                                                                                                                             ->
                                                                                                                                 qboolean>,
                                                                                                                  *mut libc::c_void>(Some(FS_Delete
                                                                                                                                              as
                                                                                                                                              unsafe extern "C" fn(_:
                                                                                                                                                                       *const libc::c_char)
                                                                                                                                                  ->
                                                                                                                                                      qboolean))),};
            init
        }
    };
unsafe extern "C" fn pfnEnableTextInput(mut enable: libc::c_int) {
    Key_EnableTextInput(enable as qboolean, false_0);
}
unsafe extern "C" fn pfnGetRenderers(mut num: libc::c_uint,
                                     mut shortName: *mut libc::c_char,
                                     mut size1: size_t,
                                     mut readableName: *mut libc::c_char,
                                     mut size2: size_t) -> libc::c_int {
    if num >= ref_0.numRenderers as libc::c_uint { return 0 as libc::c_int }
    if !shortName.is_null() && size1 != 0 {
        Q_strncpy(shortName, ref_0.shortNames[num as usize].as_mut_ptr(),
                  size1);
    }
    if !readableName.is_null() && size2 != 0 {
        Q_strncpy(readableName,
                  ref_0.readableNames[num as usize].as_mut_ptr(), size2);
    }
    return 1 as libc::c_int;
}
static mut gExtendedfuncs: ui_extendedfuncs_t =
    unsafe {
        {
            let mut init =
                ui_extendedfuncs_s{pfnEnableTextInput:
                                       Some(pfnEnableTextInput as
                                                unsafe extern "C" fn(_:
                                                                         libc::c_int)
                                                    -> ()),
                                   pfnUtfProcessChar:
                                       Some(Con_UtfProcessChar as
                                                unsafe extern "C" fn(_:
                                                                         libc::c_int)
                                                    -> libc::c_int),
                                   pfnUtfMoveLeft:
                                       Some(Con_UtfMoveLeft as
                                                unsafe extern "C" fn(_:
                                                                         *mut libc::c_char,
                                                                     _:
                                                                         libc::c_int)
                                                    -> libc::c_int),
                                   pfnUtfMoveRight:
                                       Some(Con_UtfMoveRight as
                                                unsafe extern "C" fn(_:
                                                                         *mut libc::c_char,
                                                                     _:
                                                                         libc::c_int,
                                                                     _:
                                                                         libc::c_int)
                                                    -> libc::c_int),
                                   pfnGetRenderers:
                                       Some(pfnGetRenderers as
                                                unsafe extern "C" fn(_:
                                                                         libc::c_uint,
                                                                     _:
                                                                         *mut libc::c_char,
                                                                     _:
                                                                         size_t,
                                                                     _:
                                                                         *mut libc::c_char,
                                                                     _:
                                                                         size_t)
                                                    -> libc::c_int),
                                   pfnDoubleTime:
                                       Some(Sys_DoubleTime as
                                                unsafe extern "C" fn()
                                                    -> libc::c_double),
                                   pfnParseFile:
                                       Some(_COM_ParseFileSafe as
                                                unsafe extern "C" fn(_:
                                                                         *mut libc::c_char,
                                                                     _:
                                                                         *mut libc::c_char,
                                                                     _:
                                                                         libc::c_int,
                                                                     _:
                                                                         libc::c_uint,
                                                                     _:
                                                                         *mut libc::c_int)
                                                    -> *mut libc::c_char),};
            init
        }
    };
#[no_mangle]
pub unsafe extern "C" fn UI_UnloadProgs() {
    if gameui.hInstance.is_null() { return }
    // deinitialize game
    gameui.dllFuncs.pfnShutdown.expect("non-null function pointer")();
    Cvar_FullSet(b"host_gameuiloaded\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 17 as libc::c_int);
    COM_FreeLibrary(gameui.hInstance);
    _Mem_FreePool(&mut gameui.mempool,
                  b"../engine/client/cl_gameui.c\x00" as *const u8 as
                      *const libc::c_char, 1249 as libc::c_int);
    memset(&mut gameui as *mut gameui_static_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<gameui_static_t>() as libc::c_ulong);
    Cvar_Unlink((1 as libc::c_int) << 14 as libc::c_int);
    Cmd_Unlink(((1 as libc::c_uint) << 2 as libc::c_int) as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn UI_LoadProgs() -> qboolean {
    static mut gpEngfuncs: ui_enginefuncs_t =
        ui_enginefuncs_t{pfnPIC_Load: None,
                         pfnPIC_Free: None,
                         pfnPIC_Width: None,
                         pfnPIC_Height: None,
                         pfnPIC_Set: None,
                         pfnPIC_Draw: None,
                         pfnPIC_DrawHoles: None,
                         pfnPIC_DrawTrans: None,
                         pfnPIC_DrawAdditive: None,
                         pfnPIC_EnableScissor: None,
                         pfnPIC_DisableScissor: None,
                         pfnFillRGBA: None,
                         pfnRegisterVariable: None,
                         pfnGetCvarFloat: None,
                         pfnGetCvarString: None,
                         pfnCvarSetString: None,
                         pfnCvarSetValue: None,
                         pfnAddCommand: None,
                         pfnClientCmd: None,
                         pfnDelCommand: None,
                         pfnCmdArgc: None,
                         pfnCmdArgv: None,
                         pfnCmd_Args: None,
                         Con_Printf: None,
                         Con_DPrintf: None,
                         Con_NPrintf: None,
                         Con_NXPrintf: None,
                         pfnPlayLocalSound: None,
                         pfnDrawLogo: None,
                         pfnGetLogoWidth: None,
                         pfnGetLogoHeight: None,
                         pfnGetLogoLength: None,
                         pfnDrawCharacter: None,
                         pfnDrawConsoleString: None,
                         pfnDrawSetTextColor: None,
                         pfnDrawConsoleStringLen: None,
                         pfnSetConsoleDefaultColor: None,
                         pfnGetPlayerModel: None,
                         pfnSetModel: None,
                         pfnClearScene: None,
                         pfnRenderScene: None,
                         CL_CreateVisibleEntity: None,
                         pfnHostError: None,
                         pfnFileExists: None,
                         pfnGetGameDir: None,
                         pfnCreateMapsList: None,
                         pfnClientInGame: None,
                         pfnClientJoin: None,
                         COM_LoadFile: None,
                         COM_ParseFile: None,
                         COM_FreeFile: None,
                         pfnKeyClearStates: None,
                         pfnSetKeyDest: None,
                         pfnKeynumToString: None,
                         pfnKeyGetBinding: None,
                         pfnKeySetBinding: None,
                         pfnKeyIsDown: None,
                         pfnKeyGetOverstrikeMode: None,
                         pfnKeySetOverstrikeMode: None,
                         pfnKeyGetState: None,
                         pfnMemAlloc: None,
                         pfnMemFree: None,
                         pfnGetGameInfo: None,
                         pfnGetGamesList: None,
                         pfnGetFilesList: None,
                         pfnGetSaveComment: None,
                         pfnGetDemoComment: None,
                         pfnCheckGameDll: None,
                         pfnGetClipboardData: None,
                         pfnShellExecute: None,
                         pfnWriteServerConfig: None,
                         pfnChangeInstance: None,
                         pfnPlayBackgroundTrack: None,
                         pfnHostEndGame: None,
                         pfnRandomFloat: None,
                         pfnRandomLong: None,
                         pfnSetCursor: None,
                         pfnIsMapValid: None,
                         pfnProcessImage: None,
                         pfnCompareFileTime: None,
                         pfnGetModeString: None,
                         COM_SaveFile: None,
                         COM_RemoveFile: None,};
    static mut gpExtendedfuncs: ui_extendedfuncs_t =
        ui_extendedfuncs_t{pfnEnableTextInput: None,
                           pfnUtfProcessChar: None,
                           pfnUtfMoveLeft: None,
                           pfnUtfMoveRight: None,
                           pfnGetRenderers: None,
                           pfnDoubleTime: None,
                           pfnParseFile: None,};
    static mut gpGlobals: ui_globalvars_t =
        ui_globalvars_t{time: 0.,
                        frametime: 0.,
                        scrWidth: 0,
                        scrHeight: 0,
                        maxClients: 0,
                        developer: 0,
                        demoplayback: 0,
                        demorecording: 0,
                        demoname: [0; 64],
                        maptitle: [0; 64],};
    let mut GetExtAPI: UIEXTENEDEDAPI = None;
    let mut GiveTextApi: UITEXTAPI = None;
    let mut GetMenuAPI: MENUAPI = None;
    let mut dllpath: string = [0; 256];
    let mut i: libc::c_int = 0;
    if !gameui.hInstance.is_null() { UI_UnloadProgs(); }
    // setup globals
    gameui.globals = &mut gpGlobals;
    COM_GetCommonLibraryPath(LIBRARY_GAMEUI, dllpath.as_mut_ptr(),
                             ::std::mem::size_of::<string>() as
                                 libc::c_ulong);
    gameui.hInstance =
        COM_LoadLibrary(dllpath.as_mut_ptr(), false_0 as libc::c_int,
                        false_0);
    if gameui.hInstance.is_null() {
        let mut path: string =
            *::std::mem::transmute::<&[u8; 256],
                                     &mut string>(b"libmenu.so\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
        FS_AllowDirectPaths(true_0);
        // no use to load it from engine directory, as library loader
		// that implements internal gamelibs already knows how to load it
        gameui.hInstance =
            COM_LoadLibrary(path.as_mut_ptr(), false_0 as libc::c_int,
                            true_0);
        if gameui.hInstance.is_null() {
            FS_AllowDirectPaths(false_0);
            return false_0
        }
    }
    FS_AllowDirectPaths(false_0);
    GetMenuAPI =
        ::std::mem::transmute::<*mut libc::c_void,
                                MENUAPI>(COM_GetProcAddress(gameui.hInstance,
                                                            b"GetMenuAPI\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char));
    if GetMenuAPI.is_none() {
        COM_FreeLibrary(gameui.hInstance);
        Con_Reportf(b"UI_LoadProgs: can\'t init menu API\n\x00" as *const u8
                        as *const libc::c_char);
        gameui.hInstance = 0 as *mut libc::c_void;
        return false_0
    }
    gameui.use_text_api = false_0;
    // make local copy of engfuncs to prevent overwrite it with user dll
    memcpy(&mut gpEngfuncs as *mut ui_enginefuncs_t as *mut libc::c_void,
           &mut gEngfuncs as *mut ui_enginefuncs_t as *const libc::c_void,
           ::std::mem::size_of::<ui_enginefuncs_t>() as libc::c_ulong);
    gameui.mempool =
        _Mem_AllocPool(b"Menu Pool\x00" as *const u8 as *const libc::c_char,
                       b"../engine/client/cl_gameui.c\x00" as *const u8 as
                           *const libc::c_char, 1307 as libc::c_int);
    if GetMenuAPI.expect("non-null function pointer")(&mut gameui.dllFuncs,
                                                      &mut gpEngfuncs,
                                                      gameui.globals) == 0 {
        COM_FreeLibrary(gameui.hInstance);
        Con_Reportf(b"UI_LoadProgs: can\'t init menu API\n\x00" as *const u8
                        as *const libc::c_char);
        _Mem_FreePool(&mut gameui.mempool,
                      b"../engine/client/cl_gameui.c\x00" as *const u8 as
                          *const libc::c_char, 1313 as libc::c_int);
        gameui.hInstance = 0 as *mut libc::c_void;
        return false_0
    }
    // make local copy of engfuncs to prevent overwrite it with user dll
    memcpy(&mut gpExtendedfuncs as *mut ui_extendedfuncs_t as
               *mut libc::c_void,
           &mut gExtendedfuncs as *mut ui_extendedfuncs_t as
               *const libc::c_void,
           ::std::mem::size_of::<ui_extendedfuncs_t>() as libc::c_ulong);
    memset(&mut gameui.dllFuncs2 as *mut UI_EXTENDED_FUNCTIONS as
               *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<UI_EXTENDED_FUNCTIONS>() as libc::c_ulong);
    // try to initialize new extended API
    GetExtAPI =
        ::std::mem::transmute::<*mut libc::c_void,
                                UIEXTENEDEDAPI>(COM_GetProcAddress(gameui.hInstance,
                                                                   b"GetExtAPI\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char));
    if GetExtAPI.is_some() {
        Con_Reportf(b"UI_LoadProgs: extended Menu API found\n\x00" as
                        *const u8 as *const libc::c_char);
        if GetExtAPI.expect("non-null function pointer")(1 as libc::c_int,
                                                         &mut gameui.dllFuncs2,
                                                         &mut gpExtendedfuncs)
               != 0 {
            Con_Reportf(b"UI_LoadProgs: extended Menu API initialized\n\x00"
                            as *const u8 as *const libc::c_char);
            gameui.use_text_api = true_0
        }
    } else {
        // otherwise, fallback to old and deprecated extensions
        GiveTextApi =
            ::std::mem::transmute::<*mut libc::c_void,
                                    UITEXTAPI>(COM_GetProcAddress(gameui.hInstance,
                                                                  b"GiveTextAPI\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char));
        if GiveTextApi.is_some() {
            Con_Reportf(b"UI_LoadProgs: extended text API found\n\x00" as
                            *const u8 as *const libc::c_char);
            Con_Reportf(b"^3Warning:^7 Text API is deprecated! If you are mod developer, consider moving to Extended Menu API!\n\x00"
                            as *const u8 as *const libc::c_char);
            if GiveTextApi.expect("non-null function pointer")(&mut gpExtendedfuncs)
                   != 0 {
                // they are binary compatible, so we can just pass extended funcs API to menu
                Con_Reportf(b"UI_LoadProgs: extended text API initialized\n\x00"
                                as *const u8 as *const libc::c_char);
                gameui.use_text_api = true_0
            }
        }
        gameui.dllFuncs2.pfnAddTouchButtonToList =
            ::std::mem::transmute::<*mut libc::c_void,
                                    ADDTOUCHBUTTONTOLIST>(COM_GetProcAddress(gameui.hInstance,
                                                                             b"AddTouchButtonToList\x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char));
        if gameui.dllFuncs2.pfnAddTouchButtonToList.is_some() {
            Con_Reportf(b"UI_LoadProgs: AddTouchButtonToList call found\n\x00"
                            as *const u8 as *const libc::c_char);
            Con_Reportf(b"^3Warning:^7 AddTouchButtonToList is deprecated! If you are mod developer, consider moving to Extended Menu API!\n\x00"
                            as *const u8 as *const libc::c_char);
        }
    }
    Cvar_FullSet(b"host_gameuiloaded\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 17 as libc::c_int);
    // setup gameinfo
    i = 0 as libc::c_int; // current gameinfo
    while i < SI.numgames {
        gameui.modsInfo[i as usize] =
            _Mem_Alloc(gameui.mempool,
                       ::std::mem::size_of::<GAMEINFO>() as libc::c_ulong,
                       true_0,
                       b"../engine/client/cl_gameui.c\x00" as *const u8 as
                           *const libc::c_char, 1358 as libc::c_int) as
                *mut GAMEINFO;
        UI_ConvertGameInfo(gameui.modsInfo[i as usize], SI.games[i as usize]);
        i += 1
    }
    UI_ConvertGameInfo(&mut gameui.gameInfo, SI.GameInfo);
    // setup globals
    (*gameui.globals).developer = host.allow_console as libc::c_int;
    // initialize game
    gameui.dllFuncs.pfnInit.expect("non-null function pointer")();
    return true_0;
}
