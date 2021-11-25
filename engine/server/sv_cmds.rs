#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, const_raw_ptr_to_usize_cast, extern_types,
           register_tool)]
extern "C" {
    pub type file_s;
    pub type grasshdr_s;
    #[no_mangle]
    fn Info_Print(s: *const libc::c_char);
    #[no_mangle]
    fn Info_SetValueForStarKey(s: *mut libc::c_char, key: *const libc::c_char,
                               value: *const libc::c_char,
                               maxsize: libc::c_int) -> qboolean;
    #[no_mangle]
    fn SV_Serverinfo() -> *mut libc::c_char;
    #[no_mangle]
    fn Log_Printf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Q_strlen(string: *const libc::c_char) -> size_t;
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Q_isdigit(str: *const libc::c_char) -> qboolean;
    #[no_mangle]
    fn Q_atoi(str: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn Q_atof(str: *const libc::c_char) -> libc::c_float;
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
    fn COM_FileExtension(in_0: *const libc::c_char) -> *const libc::c_char;
    #[no_mangle]
    fn COM_StripExtension(path: *mut libc::c_char);
    #[no_mangle]
    fn _copystring(mempool: poolhandle_t, s: *const libc::c_char,
                   filename: *const libc::c_char, fileline: libc::c_int)
     -> *mut libc::c_char;
    #[no_mangle]
    fn pfnNumberOfEntities() -> libc::c_int;
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_DPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Cvar_FindVarExt(var_name: *const libc::c_char,
                       ignore_group: libc::c_int) -> *mut convar_t;
    #[no_mangle]
    fn Cvar_FullSet(var_name: *const libc::c_char, value: *const libc::c_char,
                    flags: libc::c_int);
    #[no_mangle]
    fn Cvar_DirectSet(var: *mut convar_t, value: *const libc::c_char);
    #[no_mangle]
    fn Cvar_VariableInteger(var_name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn Cvar_VariableString(var_name: *const libc::c_char)
     -> *const libc::c_char;
    #[no_mangle]
    static mut sv_maxclients: *mut convar_t;
    #[no_mangle]
    fn NET_IsLocalAddress(adr: netadr_t) -> qboolean;
    #[no_mangle]
    fn NET_BaseAdrToString(a: netadr_t) -> *const libc::c_char;
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    static mut SI: sysinfo_t;
    #[no_mangle]
    fn Cbuf_AddText(text: *const libc::c_char);
    #[no_mangle]
    fn Cmd_Argc() -> libc::c_int;
    #[no_mangle]
    fn Cmd_Args() -> *const libc::c_char;
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
    fn Cmd_ListMaps(t: *mut search_t, lastmapname: *mut libc::c_char,
                    len: size_t) -> libc::c_int;
    #[no_mangle]
    fn _Mem_Free(data: *mut libc::c_void, filename: *const libc::c_char,
                 fileline: libc::c_int);
    #[no_mangle]
    fn FS_Search(pattern: *const libc::c_char, caseinsensitive: libc::c_int,
                 gamedironly: libc::c_int) -> *mut search_t;
    #[no_mangle]
    fn FS_FileExists(filename: *const libc::c_char, gamedironly: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn FS_Delete(path: *const libc::c_char) -> qboolean;
    #[no_mangle]
    fn Host_EndGame(abort: qboolean, message: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Host_ShutdownServer();
    #[no_mangle]
    fn COM_NewGame(pMapName: *const libc::c_char);
    #[no_mangle]
    fn COM_LoadLevel(pMapName: *const libc::c_char, background: qboolean);
    #[no_mangle]
    fn SV_Active() -> qboolean;
    #[no_mangle]
    static mut svs: server_static_t;
    #[no_mangle]
    fn MSG_WriteCmdExt(sb: *mut sizebuf_t, cmd: libc::c_int, type_0: netsrc_t,
                       name: *const libc::c_char);
    #[no_mangle]
    fn MSG_WriteString(sb: *mut sizebuf_t, pStr: *const libc::c_char)
     -> qboolean;
    #[no_mangle]
    static mut sv: server_t;
    #[no_mangle]
    fn Netchan_OutOfBandPrint(net_socket: libc::c_int, adr: netadr_t,
                              format: *const libc::c_char, _: ...);
    #[no_mangle]
    static mut svgame: svgame_static_t;
    #[no_mangle]
    static mut sv_hostmap: *mut convar_t;
    #[no_mangle]
    fn SV_DropClient(cl: *mut sv_client_t, crash: qboolean);
    #[no_mangle]
    fn SV_SaveGame(pName: *const libc::c_char);
    #[no_mangle]
    fn SV_ServerLog_f();
    #[no_mangle]
    fn SV_SetLogAddress_f();
    #[no_mangle]
    fn SV_QueueChangeLevel(level: *const libc::c_char,
                           landname: *const libc::c_char);
    #[no_mangle]
    fn SV_GetString(iString: string_t) -> *const libc::c_char;
    #[no_mangle]
    fn SV_CheckEdict(e: *const edict_t, file: *const libc::c_char,
                     line: libc::c_int) -> qboolean;
    #[no_mangle]
    fn SV_EdictNum(n: libc::c_int) -> *mut edict_t;
    #[no_mangle]
    fn SV_WriteEntityPatch(filename: *const libc::c_char);
    #[no_mangle]
    fn SV_CalcPing(cl: *mut sv_client_t) -> libc::c_int;
    #[no_mangle]
    fn SV_GetClientIDString(cl: *mut sv_client_t) -> *const libc::c_char;
    #[no_mangle]
    fn SV_ClientByName(name: *const libc::c_char) -> *mut sv_client_t;
    #[no_mangle]
    fn SV_ClientById(id: libc::c_int) -> *mut sv_client_t;
    #[no_mangle]
    fn SV_MapIsValid(filename: *const libc::c_char,
                     spawn_entity: *const libc::c_char,
                     landmark_name: *const libc::c_char) -> uint;
    #[no_mangle]
    fn SV_GetLatestSave() -> *const libc::c_char;
    #[no_mangle]
    fn SV_LoadGame(pName: *const libc::c_char) -> qboolean;
    /*
sv_cmds.c - server console commands
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
    #[no_mangle]
    static mut con_gamemaps: *mut convar_t;
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
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type size_t = libc::c_ulong;
pub type HANDLE = *mut libc::c_void;
pub type HINSTANCE = *mut libc::c_void;
pub type uint = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type va_list = __builtin_va_list;
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
pub type vec3_t = [vec_t; 3];
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
pub type server_t = server_s;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sv_baseline_t {
    pub classname: *const libc::c_char,
    pub baseline: entity_state_t,
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
pub type sv_state_t = libc::c_uint;
pub const ss_active: sv_state_t = 2;
pub const ss_loading: sv_state_t = 1;
pub const ss_dead: sv_state_t = 0;
pub type sv_client_t = sv_client_s;
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
pub struct challenge_t {
    pub adr: netadr_t,
    pub time: libc::c_double,
    pub challenge: libc::c_int,
    pub connected: qboolean,
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
/*
=================
SV_ClientPrintf

Sends text across to be displayed if the level passes
=================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_ClientPrintf(mut cl: *mut sv_client_t,
                                         mut fmt: *const libc::c_char,
                                         mut args: ...) {
    let mut string: [libc::c_char; 1024] = [0; 1024];
    let mut argptr: ::std::ffi::VaListImpl;
    if (*cl).flags & (1 as libc::c_uint) << 7 as libc::c_int != 0 { return }
    argptr = args.clone();
    Q_vsnprintf(string.as_mut_ptr(), 99999 as libc::c_int as size_t, fmt,
                argptr.as_va_list());
    MSG_WriteCmdExt(&mut (*cl).netchan.message, 8 as libc::c_int, NS_SERVER,
                    0 as *const libc::c_char);
    MSG_WriteString(&mut (*cl).netchan.message, string.as_mut_ptr());
}
/*
=================
SV_BroadcastPrintf

Sends text to all active clients
=================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_BroadcastPrintf(mut ignore: *mut sv_client_t,
                                            mut fmt: *const libc::c_char,
                                            mut args: ...) {
    let mut string: [libc::c_char; 1024] = [0; 1024];
    let mut argptr: ::std::ffi::VaListImpl;
    let mut cl: *mut sv_client_t = 0 as *mut sv_client_t;
    let mut i: libc::c_int = 0;
    argptr = args.clone();
    Q_vsnprintf(string.as_mut_ptr(), 99999 as libc::c_int as size_t, fmt,
                argptr.as_va_list());
    if sv.state as libc::c_uint == ss_active as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int;
        cl = svs.clients;
        while i < svs.maxclients {
            if !((*cl).flags & (1 as libc::c_uint) << 7 as libc::c_int != 0) {
                if !(cl == ignore ||
                         (*cl).state as libc::c_uint !=
                             cs_spawned as libc::c_int as libc::c_uint) {
                    MSG_WriteCmdExt(&mut (*cl).netchan.message,
                                    8 as libc::c_int, NS_SERVER,
                                    0 as *const libc::c_char);
                    MSG_WriteString(&mut (*cl).netchan.message,
                                    string.as_mut_ptr());
                }
            }
            i += 1;
            cl = cl.offset(1)
        }
    }
    if host.type_0 == HOST_DEDICATED as libc::c_int as libc::c_uint {
        // echo to console
        Con_DPrintf(b"%s\x00" as *const u8 as *const libc::c_char,
                    string.as_mut_ptr());
    };
}
/*
=================
SV_BroadcastCommand

Sends text to all active clients
=================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_BroadcastCommand(mut fmt: *const libc::c_char,
                                             mut args: ...) {
    let mut string: [libc::c_char; 1024] = [0; 1024];
    let mut argptr: ::std::ffi::VaListImpl;
    if sv.state as libc::c_uint == ss_dead as libc::c_int as libc::c_uint {
        return
    }
    argptr = args.clone();
    Q_vsnprintf(string.as_mut_ptr(), 99999 as libc::c_int as size_t, fmt,
                argptr.as_va_list());
    MSG_WriteCmdExt(&mut sv.reliable_datagram, 9 as libc::c_int, NS_SERVER,
                    0 as *const libc::c_char);
    MSG_WriteString(&mut sv.reliable_datagram, string.as_mut_ptr());
}
/*
==================
SV_SetPlayer

Sets sv_client and sv_player to the player with idnum Cmd_Argv(1)
==================
*/
unsafe extern "C" fn SV_SetPlayer() -> *mut sv_client_t {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut cl: *mut sv_client_t = 0 as *mut sv_client_t;
    let mut i: libc::c_int = 0;
    let mut idnum: libc::c_int = 0;
    if svs.clients.is_null() || sv.background as libc::c_uint != 0 {
        return 0 as *mut sv_client_t
    }
    if svs.maxclients == 1 as libc::c_int || Cmd_Argc() < 2 as libc::c_int {
        // special case for local client
        return svs.clients
    }
    s = Cmd_Argv(1 as libc::c_int);
    // numeric values are just slot numbers
    if Q_isdigit(s) as libc::c_uint != 0 ||
           *s.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
               &&
               Q_isdigit(s.offset(1 as libc::c_int as isize)) as libc::c_uint
                   != 0 {
        idnum = Q_atoi(s);
        if idnum < 0 as libc::c_int || idnum >= svs.maxclients {
            Con_Printf(b"Bad client slot: %i\n\x00" as *const u8 as
                           *const libc::c_char, idnum);
            return 0 as *mut sv_client_t
        }
        cl = &mut *svs.clients.offset(idnum as isize) as *mut sv_client_t;
        if (*cl).state as u64 == 0 {
            Con_Printf(b"Client %i is not active\n\x00" as *const u8 as
                           *const libc::c_char, idnum);
            return 0 as *mut sv_client_t
        }
        return cl
    }
    // check for a name match
    i = 0 as libc::c_int;
    cl = svs.clients;
    while i < svs.maxclients {
        if !((*cl).state as u64 == 0) {
            if Q_strncmp((*cl).name.as_mut_ptr(), s, 99999 as libc::c_int) ==
                   0 {
                return cl
            }
        }
        i += 1;
        cl = cl.offset(1)
    }
    Con_Printf(b"Userid %s is not on the server\n\x00" as *const u8 as
                   *const libc::c_char, s);
    return 0 as *mut sv_client_t;
}
/*
==================
SV_ValidateMap

check map for typically errors
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_ValidateMap(mut pMapName: *const libc::c_char,
                                        mut check_spawn: qboolean)
 -> qboolean {
    let mut spawn_entity: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut flags: libc::c_int = 0;
    // determine spawn entity classname
    if check_spawn as u64 == 0 ||
           (*sv_maxclients).value as libc::c_int <= 1 as libc::c_int {
        spawn_entity = (*SI.GameInfo).sp_entity.as_mut_ptr()
    } else { spawn_entity = (*SI.GameInfo).mp_entity.as_mut_ptr() }
    flags =
        SV_MapIsValid(pMapName, spawn_entity, 0 as *const libc::c_char) as
            libc::c_int;
    if flags as libc::c_uint & (1 as libc::c_uint) << 3 as libc::c_int != 0 {
        Con_Printf(b"^1Error:^7 map %s is invalid or not supported\n\x00" as
                       *const u8 as *const libc::c_char, pMapName);
        return false_0
    }
    if flags as libc::c_uint & (1 as libc::c_uint) << 0 as libc::c_int == 0 {
        Con_Printf(b"^1Error:^7 map %s doesn\'t exist\n\x00" as *const u8 as
                       *const libc::c_char, pMapName);
        return false_0
    }
    if check_spawn as libc::c_uint != 0 &&
           flags as libc::c_uint & (1 as libc::c_uint) << 1 as libc::c_int ==
               0 {
        Con_Printf(b"^1Error:^7 map %s doesn\'t have a valid spawnpoint\n\x00"
                       as *const u8 as *const libc::c_char, pMapName);
        return false_0
    }
    return true_0;
}
/*
==================
SV_Map_f

Goes directly to a given map without any savegame archiving.
For development work
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_Map_f() {
    let mut mapname: [libc::c_char; 64] = [0; 64];
    if Cmd_Argc() != 2 as libc::c_int {
        Con_Printf(b"Usage: map <mapname>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    // hold mapname to other place
    Q_strncpy(mapname.as_mut_ptr(), Cmd_Argv(1 as libc::c_int),
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    COM_StripExtension(mapname.as_mut_ptr());
    if SV_ValidateMap(mapname.as_mut_ptr(), true_0) as u64 == 0 { return }
    Cvar_DirectSet(sv_hostmap, mapname.as_mut_ptr());
    COM_LoadLevel(mapname.as_mut_ptr(), false_0);
}
/*
==================
SV_Maps_f

Lists maps according to given substring.

TODO: Make it more convenient. (Timestamp check, temporary file, ...)
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_Maps_f() {
    let mut separator: *const libc::c_char =
        b"-------------------\x00" as *const u8 as
            *const libc::c_char; // Substr
    let mut argStr: *const libc::c_char = Cmd_Argv(1 as libc::c_int);
    let mut nummaps: libc::c_int = 0;
    let mut mapList: *mut search_t = 0 as *mut search_t;
    if Cmd_Argc() != 2 as libc::c_int {
        Con_Printf(b"Usage: maps <substring>\nmaps * for full listing\n\x00"
                       as *const u8 as *const libc::c_char);
        return
    }
    mapList =
        FS_Search(va(b"maps/*%s*.bsp\x00" as *const u8 as *const libc::c_char,
                     argStr), true_0 as libc::c_int, true_0 as libc::c_int);
    if mapList.is_null() {
        Con_Printf(b"No related map found in \"%s/maps\"\n\x00" as *const u8
                       as *const libc::c_char,
                   (*SI.GameInfo).gamefolder.as_mut_ptr());
        return
    }
    nummaps =
        Cmd_ListMaps(mapList, 0 as *mut libc::c_char,
                     0 as libc::c_int as size_t);
    _Mem_Free(mapList as *mut libc::c_void,
              b"../engine/server/sv_cmds.c\x00" as *const u8 as
                  *const libc::c_char, 266 as libc::c_int);
    Con_Printf(b"%s\nDirectory: \"%s/maps\" - Maps listed: %d\n\x00" as
                   *const u8 as *const libc::c_char, separator,
               (*SI.GameInfo).basedir.as_mut_ptr(), nummaps);
}
/*
==================
SV_MapBackground_f

Set background map (enable physics in menu)
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_MapBackground_f() {
    let mut mapname: [libc::c_char; 64] = [0; 64];
    if Cmd_Argc() != 2 as libc::c_int {
        Con_Printf(b"Usage: map_background <mapname>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    if SV_Active() as libc::c_uint != 0 && sv.background as u64 == 0 {
        if host.game.nextstate as libc::c_uint ==
               STATE_RUNFRAME as libc::c_int as libc::c_uint {
            Con_Printf(b"^1Error:^7 can\'t set background map while game is active\n\x00"
                           as *const u8 as *const libc::c_char);
        }
        return
    }
    // hold mapname to other place
    Q_strncpy(mapname.as_mut_ptr(), Cmd_Argv(1 as libc::c_int),
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    COM_StripExtension(mapname.as_mut_ptr());
    if SV_ValidateMap(mapname.as_mut_ptr(), false_0) as u64 == 0 { return }
    // background map is always run as singleplayer
    Cvar_FullSet(b"maxplayers\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 30 as libc::c_int);
    Cvar_FullSet(b"deathmatch\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 30 as libc::c_int);
    Cvar_FullSet(b"coop\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 (1 as libc::c_int) << 30 as libc::c_int);
    COM_LoadLevel(mapname.as_mut_ptr(), true_0);
}
/*
==================
SV_NextMap_f

Change map for next in alpha-bethical ordering
For development work
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_NextMap_f() {
    let mut nextmap: [libc::c_char; 64] = [0; 64]; // only in gamedir
    let mut i: libc::c_int = 0; // only in gamedir
    let mut next: libc::c_int = 0;
    let mut t: *mut search_t = 0 as *mut search_t;
    t =
        FS_Search(b"maps\\*.bsp\x00" as *const u8 as *const libc::c_char,
                  true_0 as libc::c_int,
                  if !con_gamemaps.is_null() &&
                         (*con_gamemaps).value != 0.0f32 {
                      true_0 as libc::c_int
                  } else { false_0 as libc::c_int });
    if t.is_null() {
        t =
            FS_Search(b"maps/*.bsp\x00" as *const u8 as *const libc::c_char,
                      true_0 as libc::c_int,
                      if !con_gamemaps.is_null() &&
                             (*con_gamemaps).value != 0.0f32 {
                          true_0 as libc::c_int
                      } else { false_0 as libc::c_int })
    }
    if t.is_null() {
        Con_Printf(b"next map can\'t be found\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    i = 0 as libc::c_int;
    while i < (*t).numfilenames {
        let mut ext: *const libc::c_char =
            COM_FileExtension(*(*t).filenames.offset(i as isize));
        if !(Q_strnicmp(ext, b"bsp\x00" as *const u8 as *const libc::c_char,
                        99999 as libc::c_int) != 0) {
            COM_FileBase(*(*t).filenames.offset(i as isize),
                         nextmap.as_mut_ptr());
            if !(Q_strnicmp((*sv_hostmap).string, nextmap.as_mut_ptr(),
                            99999 as libc::c_int) != 0) {
                next = (i + 1 as libc::c_int) % (*t).numfilenames;
                COM_FileBase(*(*t).filenames.offset(next as isize),
                             nextmap.as_mut_ptr());
                Cvar_DirectSet(sv_hostmap, nextmap.as_mut_ptr());
                // jump to next map
                // found current point, check for valid
                if SV_ValidateMap(nextmap.as_mut_ptr(), true_0) as u64 != 0 {
                    // found and valid
                    COM_LoadLevel(nextmap.as_mut_ptr(), false_0);
                    _Mem_Free(t as *mut libc::c_void,
                              b"../engine/server/sv_cmds.c\x00" as *const u8
                                  as *const libc::c_char, 353 as libc::c_int);
                    return
                }
            }
        }
        i += 1
    }
    Con_Printf(b"failed to load next map\n\x00" as *const u8 as
                   *const libc::c_char);
    _Mem_Free(t as *mut libc::c_void,
              b"../engine/server/sv_cmds.c\x00" as *const u8 as
                  *const libc::c_char, 360 as libc::c_int);
}
/*
==============
SV_NewGame_f

==============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_NewGame_f() {
    if Cmd_Argc() != 1 as libc::c_int {
        Con_Printf(b"Usage: newgame\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    COM_NewGame((*SI.GameInfo).startmap.as_mut_ptr());
}
/*
==============
SV_HazardCourse_f

==============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_HazardCourse_f() {
    if Cmd_Argc() != 1 as libc::c_int {
        Con_Printf(b"Usage: hazardcourse\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    // special case for Gunman Chronicles: playing avi-file
    if FS_FileExists(va(b"media/%s.avi\x00" as *const u8 as
                            *const libc::c_char,
                        (*SI.GameInfo).trainmap.as_mut_ptr()),
                     false_0 as libc::c_int) != 0 {
        Cbuf_AddText(va(b"wait; movie %s\n\x00" as *const u8 as
                            *const libc::c_char,
                        (*SI.GameInfo).trainmap.as_mut_ptr()));
        Host_EndGame(true_0,
                     b"The End\x00" as *const u8 as *const libc::c_char);
    } else { COM_NewGame((*SI.GameInfo).trainmap.as_mut_ptr()); };
}
/*
==============
SV_Load_f

==============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_Load_f() {
    let mut path: [libc::c_char; 64] = [0; 64];
    if Cmd_Argc() != 2 as libc::c_int {
        Con_Printf(b"Usage: load <savename>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    Q_snprintf(path.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
               b"save/%s.sav\x00" as *const u8 as *const libc::c_char,
               Cmd_Argv(1 as libc::c_int));
    SV_LoadGame(path.as_mut_ptr());
}
/*
==============
SV_QuickLoad_f

==============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_QuickLoad_f() {
    Cbuf_AddText(b"echo Quick Loading...; wait; load quick\x00" as *const u8
                     as *const libc::c_char);
}
/*
==============
SV_Save_f

==============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_Save_f() {
    match Cmd_Argc() {
        1 => { SV_SaveGame(b"new\x00" as *const u8 as *const libc::c_char); }
        2 => { SV_SaveGame(Cmd_Argv(1 as libc::c_int)); }
        _ => {
            Con_Printf(b"Usage: save <savename>\n\x00" as *const u8 as
                           *const libc::c_char);
        }
    };
}
/*
==============
SV_QuickSave_f

==============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_QuickSave_f() {
    Cbuf_AddText(b"echo Quick Saving...; wait; save quick\x00" as *const u8 as
                     *const libc::c_char);
}
/*
==============
SV_DeleteSave_f

==============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_DeleteSave_f() {
    if Cmd_Argc() != 2 as libc::c_int {
        Con_Printf(b"Usage: killsave <name>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    // delete save and saveshot
    FS_Delete(va(b"save/%s.sav\x00" as *const u8 as *const libc::c_char,
                 Cmd_Argv(1 as libc::c_int)));
    FS_Delete(va(b"save/%s.bmp\x00" as *const u8 as *const libc::c_char,
                 Cmd_Argv(1 as libc::c_int)));
}
/*
==============
SV_AutoSave_f

==============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_AutoSave_f() {
    if Cmd_Argc() != 1 as libc::c_int {
        Con_Printf(b"Usage: autosave\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    SV_SaveGame(b"autosave\x00" as *const u8 as *const libc::c_char);
}
/*
==================
SV_Restart_f

restarts current level
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_Restart_f() {
    // because restart can be multiple issued
    if sv.state as libc::c_uint != ss_active as libc::c_int as libc::c_uint {
        return
    }
    COM_LoadLevel(sv.name.as_mut_ptr(), sv.background);
}
/*
==================
SV_Reload_f

continue from latest savedgame
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_Reload_f() {
    // because reload can be multiple issued
    if host.game.nextstate as libc::c_uint !=
           STATE_RUNFRAME as libc::c_int as libc::c_uint {
        return
    }
    if SV_LoadGame(SV_GetLatestSave()) as u64 == 0 {
        COM_LoadLevel((*sv_hostmap).string, false_0);
    };
}
/*
==================
SV_ChangeLevel_f

classic change level
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_ChangeLevel_f() {
    if Cmd_Argc() != 2 as libc::c_int {
        Con_Printf(b"Usage: changelevel <mapname>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    SV_QueueChangeLevel(Cmd_Argv(1 as libc::c_int), 0 as *const libc::c_char);
}
/*
==================
SV_ChangeLevel2_f

smooth change level
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_ChangeLevel2_f() {
    if Cmd_Argc() != 3 as libc::c_int {
        Con_Printf(b"Usage: changelevel2 <mapname> <landmark>\n\x00" as
                       *const u8 as *const libc::c_char);
        return
    }
    SV_QueueChangeLevel(Cmd_Argv(1 as libc::c_int),
                        Cmd_Argv(2 as libc::c_int));
}
/*
==================
SV_Kick_f

Kick a user off of the server
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_Kick_f() {
    let mut cl: *mut sv_client_t = 0 as *mut sv_client_t;
    let mut param: *const libc::c_char = 0 as *const libc::c_char;
    let mut clientId: *const libc::c_char = 0 as *const libc::c_char;
    if Cmd_Argc() != 2 as libc::c_int {
        Con_Printf(b"Usage: kick <#id|name> [reason]\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    param = Cmd_Argv(1 as libc::c_int);
    if *param as libc::c_int == '#' as i32 &&
           Q_isdigit(param.offset(1 as libc::c_int as isize)) as libc::c_uint
               != 0 {
        cl = SV_ClientById(Q_atoi(param.offset(1 as libc::c_int as isize)))
    } else { cl = SV_ClientByName(param) }
    if cl.is_null() {
        Con_Printf(b"Client is not on the server\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    if NET_IsLocalAddress((*cl).netchan.remote_address) as u64 != 0 {
        Con_Printf(b"The local player cannot be kicked!\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    param = Cmd_Argv(2 as libc::c_int);
    clientId = SV_GetClientIDString(cl);
    if *param != 0 {
        Log_Printf(b"Kick: \"%s<%i><%s><>\" was kicked by \"Console\" (message \"%s\")\n\x00"
                       as *const u8 as *const libc::c_char,
                   (*cl).name.as_mut_ptr(), (*cl).userid, clientId, param);
        SV_BroadcastPrintf(cl,
                           b"%s was kicked with message: \"%s\"\n\x00" as
                               *const u8 as *const libc::c_char,
                           (*cl).name.as_mut_ptr(), param);
        SV_ClientPrintf(cl,
                        b"You were kicked from the game with message: \"%s\"\n\x00"
                            as *const u8 as *const libc::c_char, param);
    } else {
        Log_Printf(b"Kick: \"%s<%i><%s><>\" was kicked by \"Console\"\n\x00"
                       as *const u8 as *const libc::c_char,
                   (*cl).name.as_mut_ptr(), (*cl).userid, clientId);
        SV_BroadcastPrintf(cl,
                           b"%s was kicked\n\x00" as *const u8 as
                               *const libc::c_char, (*cl).name.as_mut_ptr());
        SV_ClientPrintf(cl,
                        b"You were kicked from the game\n\x00" as *const u8 as
                            *const libc::c_char);
    }
    if (*cl).useragent[0 as libc::c_int as usize] != 0 {
        if *param != 0 {
            Netchan_OutOfBandPrint(NS_SERVER as libc::c_int,
                                   (*cl).netchan.remote_address,
                                   b"errormsg\nKicked with message:\n%s\n\x00"
                                       as *const u8 as *const libc::c_char,
                                   param);
        } else {
            Netchan_OutOfBandPrint(NS_SERVER as libc::c_int,
                                   (*cl).netchan.remote_address,
                                   b"errormsg\nYou were kicked from the game\n\x00"
                                       as *const u8 as *const libc::c_char);
        }
    }
    SV_DropClient(cl, false_0);
}
/*
==================
SV_EntPatch_f
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_EntPatch_f() {
    let mut mapname: *const libc::c_char = 0 as *const libc::c_char;
    if Cmd_Argc() < 2 as libc::c_int {
        if sv.state as libc::c_uint != ss_dead as libc::c_int as libc::c_uint
           {
            mapname = sv.name.as_mut_ptr()
        } else {
            Con_Printf(b"Usage: entpatch <mapname>\n\x00" as *const u8 as
                           *const libc::c_char);
            return
        }
    } else { mapname = Cmd_Argv(1 as libc::c_int) }
    SV_WriteEntityPatch(mapname);
}
/*
================
SV_Status_f
================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_Status_f() {
    let mut cl: *mut sv_client_t = 0 as *mut sv_client_t;
    let mut i: libc::c_int = 0;
    if svs.clients.is_null() || sv.background as libc::c_uint != 0 {
        Con_Printf(b"^3no server running.\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    Con_Printf(b"map: %s\n\x00" as *const u8 as *const libc::c_char,
               sv.name.as_mut_ptr());
    Con_Printf(b"num score ping    name            lastmsg address               port \n\x00"
                   as *const u8 as *const libc::c_char);
    Con_Printf(b"--- ----- ------- --------------- ------- --------------------- ------\n\x00"
                   as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    cl = svs.clients;
    while i < svs.maxclients {
        let mut j: libc::c_int = 0;
        let mut l: libc::c_int = 0;
        let mut s: *const libc::c_char = 0 as *const libc::c_char;
        if !((*cl).state as u64 == 0) {
            Con_Printf(b"%3i \x00" as *const u8 as *const libc::c_char, i);
            Con_Printf(b"%5i \x00" as *const u8 as *const libc::c_char,
                       (*(*cl).edict).v.frags as libc::c_int);
            if (*cl).state as libc::c_uint ==
                   cs_connected as libc::c_int as libc::c_uint {
                Con_Printf(b"Connect\x00" as *const u8 as
                               *const libc::c_char);
            } else if (*cl).state as libc::c_uint ==
                          cs_zombie as libc::c_int as libc::c_uint {
                Con_Printf(b"Zombie \x00" as *const u8 as
                               *const libc::c_char);
            } else if (*cl).flags & (1 as libc::c_uint) << 7 as libc::c_int !=
                          0 {
                Con_Printf(b"Bot   \x00" as *const u8 as *const libc::c_char);
            } else {
                Con_Printf(b"%7i \x00" as *const u8 as *const libc::c_char,
                           SV_CalcPing(cl));
            }
            Con_Printf(b"%s\x00" as *const u8 as *const libc::c_char,
                       (*cl).name.as_mut_ptr());
            l =
                (24 as libc::c_int as
                     libc::c_ulong).wrapping_sub(Q_strlen((*cl).name.as_mut_ptr()))
                    as libc::c_int;
            j = 0 as libc::c_int;
            while j < l {
                Con_Printf(b" \x00" as *const u8 as *const libc::c_char);
                j += 1
            }
            Con_Printf(b"%g \x00" as *const u8 as *const libc::c_char,
                       host.realtime - (*cl).netchan.last_received);
            s = NET_BaseAdrToString((*cl).netchan.remote_address);
            Con_Printf(b"%s\x00" as *const u8 as *const libc::c_char, s);
            l =
                (22 as libc::c_int as libc::c_ulong).wrapping_sub(Q_strlen(s))
                    as libc::c_int;
            j = 0 as libc::c_int;
            while j < l {
                Con_Printf(b" \x00" as *const u8 as *const libc::c_char);
                j += 1
            }
            Con_Printf(b"%5i\x00" as *const u8 as *const libc::c_char,
                       (*cl).netchan.qport);
            Con_Printf(b"\n\x00" as *const u8 as *const libc::c_char);
        }
        i += 1;
        cl = cl.offset(1)
    }
    Con_Printf(b"\n\x00" as *const u8 as *const libc::c_char);
}
/*
==================
SV_ConSay_f
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_ConSay_f() {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut text: [libc::c_char; 1024] = [0; 1024];
    if Cmd_Argc() < 2 as libc::c_int { return }
    if svs.clients.is_null() || sv.background as libc::c_uint != 0 {
        Con_Printf(b"^3no server running.\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    p = Cmd_Args();
    Q_strncpy(text.as_mut_ptr(),
              if *p as libc::c_int == '\"' as i32 {
                  p.offset(1 as libc::c_int as isize)
              } else { p }, 1024 as libc::c_int as size_t);
    if *p as libc::c_int == '\"' as i32 {
        text[Q_strlen(text.as_mut_ptr()).wrapping_sub(1 as libc::c_int as
                                                          libc::c_ulong) as
                 usize] = 0 as libc::c_int as libc::c_char
    }
    Log_Printf(b"Server say: \"%s\"\n\x00" as *const u8 as
                   *const libc::c_char, text.as_mut_ptr());
    Q_snprintf(text.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
               b"%s: %s\x00" as *const u8 as *const libc::c_char,
               Cvar_VariableString(b"hostname\x00" as *const u8 as
                                       *const libc::c_char), p);
    SV_BroadcastPrintf(0 as *mut sv_client_s,
                       b"%s\n\x00" as *const u8 as *const libc::c_char,
                       text.as_mut_ptr());
}
/*
==================
SV_Heartbeat_f
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_Heartbeat_f() {
    svs.last_heartbeat = -(99999 as libc::c_int) as libc::c_double;
}
/*
===========
SV_ServerInfo_f

Examine or change the serverinfo string
===========
*/
#[no_mangle]
pub unsafe extern "C" fn SV_ServerInfo_f() {
    let mut var: *mut convar_t = 0 as *mut convar_t;
    if Cmd_Argc() == 1 as libc::c_int {
        Con_Printf(b"Server info settings:\n\x00" as *const u8 as
                       *const libc::c_char);
        Info_Print(svs.serverinfo.as_mut_ptr());
        Con_Printf(b"Total %i symbols\n\x00" as *const u8 as
                       *const libc::c_char,
                   Q_strlen(svs.serverinfo.as_mut_ptr()));
        return
    }
    if Cmd_Argc() != 3 as libc::c_int {
        Con_Printf(b"Usage: serverinfo [ <key> <value> ]\n\x00" as *const u8
                       as *const libc::c_char);
        return
    }
    if *Cmd_Argv(1 as libc::c_int).offset(0 as libc::c_int as isize) as
           libc::c_int == '*' as i32 {
        Con_Printf(b"Star variables cannot be changed.\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    // if this is a cvar, change it too
    var = Cvar_FindVarExt(Cmd_Argv(1 as libc::c_int), 0 as libc::c_int);
    if !var.is_null() {
        if !(*var).string.is_null() {
            _Mem_Free((*var).string as *mut libc::c_void,
                      b"../engine/server/sv_cmds.c\x00" as *const u8 as
                          *const libc::c_char, 786 as libc::c_int);
            (*var).string = 0 as *mut libc::c_char
        }
        // free the old value string
        (*var).string =
            _copystring(host.mempool, Cmd_Argv(2 as libc::c_int),
                        b"../engine/server/sv_cmds.c\x00" as *const u8 as
                            *const libc::c_char, 787 as libc::c_int);
        (*var).value = Q_atof((*var).string)
    }
    Info_SetValueForStarKey(svs.serverinfo.as_mut_ptr(),
                            Cmd_Argv(1 as libc::c_int),
                            Cmd_Argv(2 as libc::c_int), 512 as libc::c_int);
    SV_BroadcastCommand(b"fullserverinfo \"%s\"\n\x00" as *const u8 as
                            *const libc::c_char, SV_Serverinfo());
}
/*
===========
SV_LocalInfo_f

Examine or change the localinfo string
===========
*/
#[no_mangle]
pub unsafe extern "C" fn SV_LocalInfo_f() {
    if Cmd_Argc() == 1 as libc::c_int {
        Con_Printf(b"Local info settings:\n\x00" as *const u8 as
                       *const libc::c_char);
        Info_Print(svs.localinfo.as_mut_ptr());
        Con_Printf(b"Total %i symbols\n\x00" as *const u8 as
                       *const libc::c_char,
                   Q_strlen(svs.localinfo.as_mut_ptr()));
        return
    }
    if Cmd_Argc() != 3 as libc::c_int {
        Con_Printf(b"Usage: localinfo [ <key> <value> ]\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    if *Cmd_Argv(1 as libc::c_int).offset(0 as libc::c_int as isize) as
           libc::c_int == '*' as i32 {
        Con_Printf(b"Star variables cannot be changed.\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    Info_SetValueForStarKey(svs.localinfo.as_mut_ptr(),
                            Cmd_Argv(1 as libc::c_int),
                            Cmd_Argv(2 as libc::c_int), 32768 as libc::c_int);
}
/*
===========
SV_ClientInfo_f

Examine all a users info strings
===========
*/
#[no_mangle]
pub unsafe extern "C" fn SV_ClientInfo_f() {
    let mut cl: *mut sv_client_t = 0 as *mut sv_client_t;
    if Cmd_Argc() != 2 as libc::c_int {
        Con_Printf(b"Usage: clientinfo <userid>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    cl = SV_SetPlayer();
    if cl.is_null() { return }
    Con_Printf(b"userinfo\n\x00" as *const u8 as *const libc::c_char);
    Con_Printf(b"--------\n\x00" as *const u8 as *const libc::c_char);
    Info_Print((*cl).userinfo.as_mut_ptr());
}
/*
===========
SV_ClientUserAgent_f

Examine useragent strings
===========
*/
#[no_mangle]
pub unsafe extern "C" fn SV_ClientUserAgent_f() {
    let mut cl: *mut sv_client_t = 0 as *mut sv_client_t;
    if Cmd_Argc() != 2 as libc::c_int {
        Con_Printf(b"Usage: clientuseragent <userid>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    cl = SV_SetPlayer();
    if cl.is_null() { return }
    Con_Printf(b"useragent\n\x00" as *const u8 as *const libc::c_char);
    Con_Printf(b"---------\n\x00" as *const u8 as *const libc::c_char);
    Info_Print((*cl).useragent.as_mut_ptr());
}
/*
===============
SV_KillServer_f

Kick everyone off, possibly in preparation for a new game
===============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_KillServer_f() { Host_ShutdownServer(); }
/*
===============
SV_PlayersOnly_f

disable plhysics but players
===============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_PlayersOnly_f() {
    if Cvar_VariableInteger(b"sv_cheats\x00" as *const u8 as
                                *const libc::c_char) == 0 {
        return
    }
    sv.playersonly =
        ::std::mem::transmute::<libc::c_uint,
                                qboolean>(sv.playersonly as libc::c_uint ^
                                              1 as libc::c_int as
                                                  libc::c_uint);
    SV_BroadcastPrintf(0 as *mut sv_client_s,
                       b"%s game physic\n\x00" as *const u8 as
                           *const libc::c_char,
                       if sv.playersonly as libc::c_uint != 0 {
                           b"Freeze\x00" as *const u8 as *const libc::c_char
                       } else {
                           b"Resume\x00" as *const u8 as *const libc::c_char
                       });
}
/*
===============
SV_EdictUsage_f

===============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_EdictUsage_f() {
    let mut active: libc::c_int = 0;
    if sv.state as libc::c_uint != ss_active as libc::c_int as libc::c_uint {
        Con_Printf(b"^3no server running.\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    active = pfnNumberOfEntities();
    Con_Printf(b"%5i edicts is used\n\x00" as *const u8 as
                   *const libc::c_char, active);
    Con_Printf(b"%5i edicts is free\n\x00" as *const u8 as
                   *const libc::c_char, (*SI.GameInfo).max_edicts - active);
    Con_Printf(b"%5i total\n\x00" as *const u8 as *const libc::c_char,
               (*SI.GameInfo).max_edicts);
}
/*
===============
SV_EntityInfo_f

===============
*/
#[no_mangle]
pub unsafe extern "C" fn SV_EntityInfo_f() {
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    let mut i: libc::c_int = 0;
    if sv.state as libc::c_uint != ss_active as libc::c_int as libc::c_uint {
        Con_Printf(b"^3no server running.\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    i = 0 as libc::c_int;
    while i < svgame.numEntities {
        ent = SV_EdictNum(i);
        if !(SV_CheckEdict(ent,
                           b"../engine/server/sv_cmds.c\x00" as *const u8 as
                               *const libc::c_char, 948 as libc::c_int) as u64
                 == 0) {
            Con_Printf(b"%5i origin: %.f %.f %.f\x00" as *const u8 as
                           *const libc::c_char, i,
                       (*ent).v.origin[0 as libc::c_int as usize] as
                           libc::c_double,
                       (*ent).v.origin[1 as libc::c_int as usize] as
                           libc::c_double,
                       (*ent).v.origin[2 as libc::c_int as usize] as
                           libc::c_double);
            if (*ent).v.classname != 0 {
                Con_Printf(b", class: %s\x00" as *const u8 as
                               *const libc::c_char,
                           SV_GetString((*ent).v.classname));
            }
            if (*ent).v.globalname != 0 {
                Con_Printf(b", global: %s\x00" as *const u8 as
                               *const libc::c_char,
                           SV_GetString((*ent).v.globalname));
            }
            if (*ent).v.targetname != 0 {
                Con_Printf(b", name: %s\x00" as *const u8 as
                               *const libc::c_char,
                           SV_GetString((*ent).v.targetname));
            }
            if (*ent).v.target != 0 {
                Con_Printf(b", target: %s\x00" as *const u8 as
                               *const libc::c_char,
                           SV_GetString((*ent).v.target));
            }
            if (*ent).v.model != 0 {
                Con_Printf(b", model: %s\x00" as *const u8 as
                               *const libc::c_char,
                           SV_GetString((*ent).v.model));
            }
            Con_Printf(b"\n\x00" as *const u8 as *const libc::c_char);
        }
        i += 1
    };
}
/*
================
Rcon_Redirect_f

Force redirect N lines of console output to client
================
*/
#[no_mangle]
pub unsafe extern "C" fn Rcon_Redirect_f() {
    let mut lines: libc::c_int = 2000 as libc::c_int;
    if host.rd.target as u64 == 0 {
        Con_Printf(b"redirect is only valid from rcon\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    if Cmd_Argc() == 2 as libc::c_int {
        lines = Q_atoi(Cmd_Argv(1 as libc::c_int))
    }
    host.rd.lines = lines;
    Con_Printf(b"Redirection enabled for next %d lines\n\x00" as *const u8 as
                   *const libc::c_char, lines);
}
/*
==================
SV_InitHostCommands

commands that create server
is available always
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_InitHostCommands() {
    Cmd_AddRestrictedCommand(b"map\x00" as *const u8 as *const libc::c_char,
                             Some(SV_Map_f as unsafe extern "C" fn() -> ()),
                             b"start new level\x00" as *const u8 as
                                 *const libc::c_char);
    Cmd_AddCommand(b"maps\x00" as *const u8 as *const libc::c_char,
                   Some(SV_Maps_f as unsafe extern "C" fn() -> ()),
                   b"list maps\x00" as *const u8 as *const libc::c_char);
    if host.type_0 == HOST_NORMAL as libc::c_int as libc::c_uint {
        Cmd_AddRestrictedCommand(b"newgame\x00" as *const u8 as
                                     *const libc::c_char,
                                 Some(SV_NewGame_f as
                                          unsafe extern "C" fn() -> ()),
                                 b"begin new game\x00" as *const u8 as
                                     *const libc::c_char);
        Cmd_AddRestrictedCommand(b"hazardcourse\x00" as *const u8 as
                                     *const libc::c_char,
                                 Some(SV_HazardCourse_f as
                                          unsafe extern "C" fn() -> ()),
                                 b"starting a Hazard Course\x00" as *const u8
                                     as *const libc::c_char);
        Cmd_AddRestrictedCommand(b"map_background\x00" as *const u8 as
                                     *const libc::c_char,
                                 Some(SV_MapBackground_f as
                                          unsafe extern "C" fn() -> ()),
                                 b"set background map\x00" as *const u8 as
                                     *const libc::c_char);
        Cmd_AddRestrictedCommand(b"load\x00" as *const u8 as
                                     *const libc::c_char,
                                 Some(SV_Load_f as
                                          unsafe extern "C" fn() -> ()),
                                 b"load a saved game file\x00" as *const u8 as
                                     *const libc::c_char);
        Cmd_AddRestrictedCommand(b"loadquick\x00" as *const u8 as
                                     *const libc::c_char,
                                 Some(SV_QuickLoad_f as
                                          unsafe extern "C" fn() -> ()),
                                 b"load a quick-saved game file\x00" as
                                     *const u8 as *const libc::c_char);
        Cmd_AddRestrictedCommand(b"reload\x00" as *const u8 as
                                     *const libc::c_char,
                                 Some(SV_Reload_f as
                                          unsafe extern "C" fn() -> ()),
                                 b"continue from latest save or restart level\x00"
                                     as *const u8 as *const libc::c_char);
        Cmd_AddRestrictedCommand(b"killsave\x00" as *const u8 as
                                     *const libc::c_char,
                                 Some(SV_DeleteSave_f as
                                          unsafe extern "C" fn() -> ()),
                                 b"delete a saved game file and saveshot\x00"
                                     as *const u8 as *const libc::c_char);
        Cmd_AddRestrictedCommand(b"nextmap\x00" as *const u8 as
                                     *const libc::c_char,
                                 Some(SV_NextMap_f as
                                          unsafe extern "C" fn() -> ()),
                                 b"load next level\x00" as *const u8 as
                                     *const libc::c_char);
    };
}
/*
==================
SV_InitOperatorCommands
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_InitOperatorCommands() {
    Cmd_AddCommand(b"heartbeat\x00" as *const u8 as *const libc::c_char,
                   Some(SV_Heartbeat_f as unsafe extern "C" fn() -> ()),
                   b"send a heartbeat to the master server\x00" as *const u8
                       as *const libc::c_char);
    Cmd_AddCommand(b"kick\x00" as *const u8 as *const libc::c_char,
                   Some(SV_Kick_f as unsafe extern "C" fn() -> ()),
                   b"kick a player off the server by number or name\x00" as
                       *const u8 as *const libc::c_char);
    Cmd_AddCommand(b"status\x00" as *const u8 as *const libc::c_char,
                   Some(SV_Status_f as unsafe extern "C" fn() -> ()),
                   b"print server status information\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"localinfo\x00" as *const u8 as *const libc::c_char,
                   Some(SV_LocalInfo_f as unsafe extern "C" fn() -> ()),
                   b"examine or change the localinfo string\x00" as *const u8
                       as *const libc::c_char);
    Cmd_AddCommand(b"serverinfo\x00" as *const u8 as *const libc::c_char,
                   Some(SV_ServerInfo_f as unsafe extern "C" fn() -> ()),
                   b"examine or change the serverinfo string\x00" as *const u8
                       as *const libc::c_char);
    Cmd_AddCommand(b"clientinfo\x00" as *const u8 as *const libc::c_char,
                   Some(SV_ClientInfo_f as unsafe extern "C" fn() -> ()),
                   b"print user infostring (player num required)\x00" as
                       *const u8 as *const libc::c_char);
    Cmd_AddCommand(b"clientuseragent\x00" as *const u8 as *const libc::c_char,
                   Some(SV_ClientUserAgent_f as unsafe extern "C" fn() -> ()),
                   b"print user agent (player num required)\x00" as *const u8
                       as *const libc::c_char);
    Cmd_AddCommand(b"playersonly\x00" as *const u8 as *const libc::c_char,
                   Some(SV_PlayersOnly_f as unsafe extern "C" fn() -> ()),
                   b"freezes time, except for players\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"restart\x00" as *const u8 as *const libc::c_char,
                   Some(SV_Restart_f as unsafe extern "C" fn() -> ()),
                   b"restarting current level\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"entpatch\x00" as *const u8 as *const libc::c_char,
                   Some(SV_EntPatch_f as unsafe extern "C" fn() -> ()),
                   b"write entity patch to allow external editing\x00" as
                       *const u8 as *const libc::c_char);
    Cmd_AddCommand(b"edict_usage\x00" as *const u8 as *const libc::c_char,
                   Some(SV_EdictUsage_f as unsafe extern "C" fn() -> ()),
                   b"show info about edicts usage\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"entity_info\x00" as *const u8 as *const libc::c_char,
                   Some(SV_EntityInfo_f as unsafe extern "C" fn() -> ()),
                   b"show more info about edicts\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"shutdownserver\x00" as *const u8 as *const libc::c_char,
                   Some(SV_KillServer_f as unsafe extern "C" fn() -> ()),
                   b"shutdown current server\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"changelevel\x00" as *const u8 as *const libc::c_char,
                   Some(SV_ChangeLevel_f as unsafe extern "C" fn() -> ()),
                   b"change level\x00" as *const u8 as *const libc::c_char);
    Cmd_AddCommand(b"changelevel2\x00" as *const u8 as *const libc::c_char,
                   Some(SV_ChangeLevel2_f as unsafe extern "C" fn() -> ()),
                   b"smooth change level\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"redirect\x00" as *const u8 as *const libc::c_char,
                   Some(Rcon_Redirect_f as unsafe extern "C" fn() -> ()),
                   b"force enable rcon redirection\x00" as *const u8 as
                       *const libc::c_char);
    Cmd_AddCommand(b"logaddress\x00" as *const u8 as *const libc::c_char,
                   Some(SV_SetLogAddress_f as unsafe extern "C" fn() -> ()),
                   b"sets address and port for remote logging host\x00" as
                       *const u8 as *const libc::c_char);
    Cmd_AddCommand(b"log\x00" as *const u8 as *const libc::c_char,
                   Some(SV_ServerLog_f as unsafe extern "C" fn() -> ()),
                   b"enables logging to file\x00" as *const u8 as
                       *const libc::c_char);
    if host.type_0 == HOST_NORMAL as libc::c_int as libc::c_uint {
        Cmd_AddCommand(b"save\x00" as *const u8 as *const libc::c_char,
                       Some(SV_Save_f as unsafe extern "C" fn() -> ()),
                       b"save the game to a file\x00" as *const u8 as
                           *const libc::c_char);
        Cmd_AddCommand(b"savequick\x00" as *const u8 as *const libc::c_char,
                       Some(SV_QuickSave_f as unsafe extern "C" fn() -> ()),
                       b"save the game to the quicksave\x00" as *const u8 as
                           *const libc::c_char);
        Cmd_AddCommand(b"autosave\x00" as *const u8 as *const libc::c_char,
                       Some(SV_AutoSave_f as unsafe extern "C" fn() -> ()),
                       b"save the game to \'autosave\' file\x00" as *const u8
                           as *const libc::c_char);
    } else if host.type_0 == HOST_DEDICATED as libc::c_int as libc::c_uint {
        Cmd_AddCommand(b"say\x00" as *const u8 as *const libc::c_char,
                       Some(SV_ConSay_f as unsafe extern "C" fn() -> ()),
                       b"send a chat message to everyone on the server\x00" as
                           *const u8 as *const libc::c_char);
    };
}
/*
==================
SV_KillOperatorCommands
==================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_KillOperatorCommands() {
    Cmd_RemoveCommand(b"heartbeat\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"kick\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"status\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"localinfo\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"serverinfo\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"clientinfo\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"playersonly\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"restart\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"entpatch\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"edict_usage\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"entity_info\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"shutdownserver\x00" as *const u8 as
                          *const libc::c_char);
    Cmd_RemoveCommand(b"changelevel\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"changelevel2\x00" as *const u8 as
                          *const libc::c_char);
    Cmd_RemoveCommand(b"logaddress\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"log\x00" as *const u8 as *const libc::c_char);
    if host.type_0 == HOST_NORMAL as libc::c_int as libc::c_uint {
        Cmd_RemoveCommand(b"save\x00" as *const u8 as *const libc::c_char);
        Cmd_RemoveCommand(b"savequick\x00" as *const u8 as
                              *const libc::c_char);
        Cmd_RemoveCommand(b"autosave\x00" as *const u8 as
                              *const libc::c_char);
    } else if host.type_0 == HOST_DEDICATED as libc::c_int as libc::c_uint {
        Cmd_RemoveCommand(b"say\x00" as *const u8 as *const libc::c_char);
    };
}
