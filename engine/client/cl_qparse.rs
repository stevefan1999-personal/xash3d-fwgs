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
    #[no_mangle]
    fn sqrtf(_: libc::c_float) -> libc::c_float;
    #[no_mangle]
    fn S_StopBackgroundTrack();
    #[no_mangle]
    fn S_StopSound(entnum: libc::c_int, channel: libc::c_int,
                   soundname: *const libc::c_char);
    #[no_mangle]
    fn UI_SetActiveMenu(fActive: qboolean);
    #[no_mangle]
    fn Con_Print(txt: *const libc::c_char);
    #[no_mangle]
    fn CL_StopPlayback();
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn Q_strlen(string: *const libc::c_char) -> size_t;
    #[no_mangle]
    fn Q_strncat(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Q_strnicmp(s1: *const libc::c_char, s2: *const libc::c_char,
                  n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn _COM_ParseFileSafe(data: *mut libc::c_char, token: *mut libc::c_char,
                          size: libc::c_int, flags: libc::c_uint,
                          len: *mut libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn Sys_DoubleTime() -> libc::c_double;
    #[no_mangle]
    fn Cvar_FullSet(var_name: *const libc::c_char, value: *const libc::c_char,
                    flags: libc::c_int);
    #[no_mangle]
    fn Cvar_Set(var_name: *const libc::c_char, value: *const libc::c_char);
    #[no_mangle]
    fn Cvar_SetValue(var_name: *const libc::c_char, value: libc::c_float);
    #[no_mangle]
    fn Cvar_Exists(var_name: *const libc::c_char) -> qboolean;
    #[no_mangle]
    fn Cvar_Reset(var_name: *const libc::c_char);
    #[no_mangle]
    static mut cl_allow_levelshots: *mut convar_t;
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    static mut SI: sysinfo_t;
    #[no_mangle]
    fn Cbuf_AddText(text: *const libc::c_char);
    #[no_mangle]
    fn Cbuf_Execute();
    #[no_mangle]
    fn Cmd_Exists(cmd_name: *const libc::c_char) -> qboolean;
    #[no_mangle]
    fn Cmd_ExecuteString(text: *const libc::c_char);
    #[no_mangle]
    fn _Mem_Alloc(poolptr: poolhandle_t, size: size_t, clear: qboolean,
                  filename: *const libc::c_char, fileline: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    fn FS_FileExists(filename: *const libc::c_char, gamedironly: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn Host_IsQuakeCompatible() -> qboolean;
    #[no_mangle]
    fn Host_AbortCurrentFrame();
    #[no_mangle]
    fn Host_Error(error: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_Reportf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn CSCR_LoadDefaultCVars(scriptfilename: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    static mut CL_UPDATE_BACKUP: libc::c_int;
    #[no_mangle]
    static mut cl: client_t;
    #[no_mangle]
    static mut cls: client_static_t;
    #[no_mangle]
    static mut clgame: clgame_static_t;
    #[no_mangle]
    static mut gameui: gameui_static_t;
    #[no_mangle]
    fn MSG_InitExt(sb: *mut sizebuf_t, pDebugName: *const libc::c_char,
                   pData: *mut libc::c_void, nBytes: libc::c_int,
                   nMaxBits: libc::c_int);
    #[no_mangle]
    fn MSG_CheckOverflow(sb: *mut sizebuf_t) -> qboolean;
    #[no_mangle]
    fn MSG_Clear(sb: *mut sizebuf_t);
    #[no_mangle]
    fn MSG_WriteByte(sb: *mut sizebuf_t, val: libc::c_int);
    #[no_mangle]
    fn MSG_WriteWord(sb: *mut sizebuf_t, val: libc::c_int);
    #[no_mangle]
    fn MSG_WriteLong(sb: *mut sizebuf_t, val: libc::c_int);
    #[no_mangle]
    fn MSG_WriteCoord(sb: *mut sizebuf_t, val: libc::c_float);
    #[no_mangle]
    fn MSG_WriteString(sb: *mut sizebuf_t, pStr: *const libc::c_char)
     -> qboolean;
    #[no_mangle]
    fn MSG_ReadCmd(sb: *mut sizebuf_t, type_0: netsrc_t) -> libc::c_int;
    #[no_mangle]
    fn MSG_ReadChar(sb: *mut sizebuf_t) -> libc::c_int;
    #[no_mangle]
    fn MSG_ReadByte(sb: *mut sizebuf_t) -> libc::c_int;
    #[no_mangle]
    fn MSG_ReadShort(sb: *mut sizebuf_t) -> libc::c_int;
    #[no_mangle]
    fn MSG_ReadWord(sb: *mut sizebuf_t) -> libc::c_int;
    #[no_mangle]
    fn MSG_ReadLong(sb: *mut sizebuf_t) -> libc::c_int;
    #[no_mangle]
    fn MSG_ReadCoord(sb: *mut sizebuf_t) -> libc::c_float;
    #[no_mangle]
    fn MSG_ReadFloat(sb: *mut sizebuf_t) -> libc::c_float;
    #[no_mangle]
    fn MSG_ReadVec3Coord(sb: *mut sizebuf_t, fa: *mut vec_t);
    #[no_mangle]
    fn MSG_ReadStringExt(sb: *mut sizebuf_t, bLine: qboolean)
     -> *mut libc::c_char;
    #[no_mangle]
    static mut refState: ref_globals_t;
    #[no_mangle]
    static mut r_decals: *mut convar_t;
    #[no_mangle]
    static mut mp_decals: convar_t;
    #[no_mangle]
    static mut cl_levelshot_name: *mut convar_t;
    #[no_mangle]
    fn CL_SetLightstyle(style: libc::c_int, s: *const libc::c_char,
                        f: libc::c_float);
    #[no_mangle]
    fn CL_AddToResourceList(pResource: *mut resource_t,
                            pList: *mut resource_t);
    #[no_mangle]
    fn CL_Parse_Debug(enable: qboolean);
    #[no_mangle]
    fn CL_Parse_RecordCommand(cmd: libc::c_int, startoffset: libc::c_int);
    #[no_mangle]
    fn CL_ResetFrame(frame: *mut frame_t);
    #[no_mangle]
    fn CL_SignonReply();
    #[no_mangle]
    fn CL_ClearState();
    #[no_mangle]
    fn CL_DemoCompleted();
    #[no_mangle]
    fn CL_BatchResourceRequest(initialize: qboolean);
    #[no_mangle]
    fn CL_ParseFinaleCutscene(msg: *mut sizebuf_t, level: libc::c_int);
    #[no_mangle]
    fn CL_InitEdicts();
    #[no_mangle]
    fn CL_ModelHandle(modelindex: libc::c_int) -> *mut model_t;
    #[no_mangle]
    fn CL_DispatchUserMessage(pszName: *const libc::c_char,
                              iSize: libc::c_int, pbuf: *mut libc::c_void)
     -> qboolean;
    #[no_mangle]
    fn CL_ParseViewEntity(msg: *mut sizebuf_t);
    #[no_mangle]
    fn CL_ParseServerTime(msg: *mut sizebuf_t);
    #[no_mangle]
    fn CL_SetSolidEntities();
    #[no_mangle]
    fn CL_ProcessPacket(frame: *mut frame_t);
    #[no_mangle]
    fn S_StartBackgroundTrack(intro: *const libc::c_char,
                              loop_0: *const libc::c_char,
                              position: libc::c_int, fullpath: qboolean);
    #[no_mangle]
    fn S_StartSound(pos: *const vec_t, ent: libc::c_int, chan: libc::c_int,
                    sfx: sound_t, vol: libc::c_float, attn: libc::c_float,
                    pitch: libc::c_int, flags: libc::c_int);
    #[no_mangle]
    fn CL_IsPlayerIndex(idx: libc::c_int) -> qboolean;
    #[no_mangle]
    fn R_AddEfrags(ent: *mut cl_entity_t);
    #[no_mangle]
    fn CL_ResetLatchedVars(ent: *mut cl_entity_t, full_reset: qboolean);
    #[no_mangle]
    fn Key_SetKeyDest(key_dest: libc::c_int);
    #[no_mangle]
    fn Con_FixedFont() -> qboolean;
    #[no_mangle]
    fn CL_KillDeadBeams(pDeadEntity: *mut cl_entity_t);
    #[no_mangle]
    fn R_RunParticleEffect(org: *const vec_t, dir: *const vec_t,
                           color: libc::c_int, count: libc::c_int);
    #[no_mangle]
    fn IN_MouseRestorePos();
}
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
pub type sound_t = libc::c_int;
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
pub type ui_globalvars_t = ui_globalvars_s;
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
pub type ADDTOUCHBUTTONTOLIST
    =
    Option<unsafe extern "C" fn(_: *const libc::c_char,
                                _: *const libc::c_char,
                                _: *const libc::c_char, _: *mut libc::c_uchar,
                                _: libc::c_int) -> ()>;
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
pub struct gameui_draw_t {
    pub gl_texturenum: libc::c_int,
    pub scissor_x: libc::c_int,
    pub scissor_y: libc::c_int,
    pub scissor_width: libc::c_int,
    pub scissor_height: libc::c_int,
    pub scissor_test: qboolean,
    pub textColor: rgba_t,
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
#[inline(always)]
unsafe extern "C" fn __tg_sqrt(mut __x: libc::c_float) -> libc::c_float {
    return sqrtf(__x);
}
#[inline]
unsafe extern "C" fn BitByte(mut bits: libc::c_int) -> libc::c_int {
    return (bits + (8 as libc::c_int - 1 as libc::c_int)) / 8 as libc::c_int *
               8 as libc::c_int >> 3 as libc::c_int;
}
#[inline]
unsafe extern "C" fn MSG_GetNumBytesWritten(mut sb: *mut sizebuf_t)
 -> libc::c_int {
    return BitByte((*sb).iCurBit);
}
#[inline]
unsafe extern "C" fn MSG_GetNumBitsLeft(mut sb: *mut sizebuf_t)
 -> libc::c_int {
    return (*sb).nDataBits - (*sb).iCurBit;
}
#[inline]
unsafe extern "C" fn CL_EDICT_NUM(mut n: libc::c_int) -> *mut cl_entity_t {
    if clgame.entities.is_null() {
        Host_Error(b"CL_EDICT_NUM: clgame.entities is NULL\n\x00" as *const u8
                       as *const libc::c_char);
        return 0 as *mut cl_entity_t
    }
    if n >= 0 as libc::c_int && n < clgame.maxEntities {
        return clgame.entities.offset(n as isize)
    }
    Host_Error(b"CL_EDICT_NUM: bad number %i\n\x00" as *const u8 as
                   *const libc::c_char, n);
    return 0 as *mut cl_entity_t;
}
// bumped on client side by svc_foundsecret
// bumped by svc_killedmonster
static mut cmd_buf: [libc::c_char; 8192] = [0; 8192];
static mut msg_buf: [libc::c_char; 8192] = [0; 8192];
static mut msg_demo: sizebuf_t =
    sizebuf_t{bOverflow: false_0,
              pDebugName: 0 as *const libc::c_char,
              pData: 0 as *const byte as *mut byte,
              iCurBit: 0,
              nDataBits: 0,};
/*
==================
CL_DispatchQuakeMessage

==================
*/
unsafe extern "C" fn CL_DispatchQuakeMessage(mut name: *const libc::c_char) {
    CL_DispatchUserMessage(name, msg_demo.iCurBit >> 3 as libc::c_int,
                           msg_demo.pData as *mut libc::c_void);
    MSG_Clear(&mut msg_demo);
    // don't forget to clear buffer
}
/*
==================
CL_ParseQuakeStats

redirect to qwrap->client
==================
*/
unsafe extern "C" fn CL_ParseQuakeStats(mut msg: *mut sizebuf_t) {
    MSG_WriteByte(&mut msg_demo, MSG_ReadByte(msg)); // stat num
    MSG_WriteLong(&mut msg_demo, MSG_ReadLong(msg)); // stat value
    CL_DispatchQuakeMessage(b"Stats\x00" as *const u8 as *const libc::c_char);
}
/*
==================
CL_EntityTeleported

check for instant movement in case
we don't want interpolate this
==================
*/
unsafe extern "C" fn CL_QuakeEntityTeleported(mut ent: *mut cl_entity_t,
                                              mut newstate:
                                                  *mut entity_state_t)
 -> qboolean {
    let mut len: libc::c_float = 0.;
    let mut maxlen: libc::c_float = 0.;
    let mut delta: vec3_t = [0.; 3];
    delta[0 as libc::c_int as usize] =
        (*newstate).origin[0 as libc::c_int as usize] -
            (*ent).prevstate.origin[0 as libc::c_int as usize];
    delta[1 as libc::c_int as usize] =
        (*newstate).origin[1 as libc::c_int as usize] -
            (*ent).prevstate.origin[1 as libc::c_int as usize];
    delta[2 as libc::c_int as usize] =
        (*newstate).origin[2 as libc::c_int as usize] -
            (*ent).prevstate.origin[2 as libc::c_int as usize];
    // compute potential max movement in units per frame and compare with entity movement
    maxlen = clgame.movevars.maxvelocity * (1.0f32 / 20.0f32);
    len =
        __tg_sqrt(delta[0 as libc::c_int as usize] *
                      delta[0 as libc::c_int as usize] +
                      delta[1 as libc::c_int as usize] *
                          delta[1 as libc::c_int as usize] +
                      delta[2 as libc::c_int as usize] *
                          delta[2 as libc::c_int as usize]);
    return (len > maxlen) as libc::c_int as qboolean;
}
/*
==================
CL_ParseQuakeStats

redirect to qwrap->client
==================
*/
unsafe extern "C" fn CL_UpdateQuakeStats(mut msg: *mut sizebuf_t,
                                         mut statnum: libc::c_int,
                                         mut has_update: qboolean)
 -> libc::c_int {
    let mut value: libc::c_int = 0 as libc::c_int; // stat num
    MSG_WriteByte(&mut msg_demo, statnum);
    if has_update as u64 != 0 {
        if statnum == 0 as libc::c_int {
            value = MSG_ReadShort(msg)
        } else { value = MSG_ReadByte(msg) }
    }
    MSG_WriteLong(&mut msg_demo, value);
    CL_DispatchQuakeMessage(b"Stats\x00" as *const u8 as *const libc::c_char);
    return value;
}
/*
==================
CL_UpdateQuakeGameMode

redirect to qwrap->client
==================
*/
unsafe extern "C" fn CL_UpdateQuakeGameMode(mut gamemode: libc::c_int) {
    MSG_WriteByte(&mut msg_demo, gamemode);
    CL_DispatchQuakeMessage(b"GameMode\x00" as *const u8 as
                                *const libc::c_char);
}
/*
==================
CL_ParseQuakeSound

==================
*/
unsafe extern "C" fn CL_ParseQuakeSound(mut msg: *mut sizebuf_t) {
    let mut channel: libc::c_int =
        0; // Quake1 have max 255 precached sounds. erm
    let mut sound: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut entnum: libc::c_int = 0;
    let mut volume: libc::c_float = 0.;
    let mut attn: libc::c_float = 0.;
    let mut handle: sound_t = 0;
    let mut pos: vec3_t = [0.; 3];
    flags = MSG_ReadByte(msg);
    if flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        volume = MSG_ReadByte(msg) as libc::c_float / 255.0f32
    } else { volume = 1.0f64 as libc::c_float }
    if flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        attn = MSG_ReadByte(msg) as libc::c_float / 64.0f32
    } else { attn = 0 as libc::c_int as libc::c_float }
    channel = MSG_ReadWord(msg);
    sound = MSG_ReadByte(msg);
    // positioned in space
    MSG_ReadVec3Coord(msg, pos.as_mut_ptr()); // entity reletive
    entnum = channel >> 3 as libc::c_int;
    channel &= 7 as libc::c_int;
    // see precached sound
    handle = cl.sound_index[sound as usize] as sound_t; // too early
    if cl.audio_prepped as u64 == 0 { return }
    S_StartSound(pos.as_mut_ptr() as *const vec_t, entnum, channel, handle,
                 volume, attn, 100 as libc::c_int, flags);
}
/*
==================
CL_ParseQuakeServerInfo

==================
*/
unsafe extern "C" fn CL_ParseQuakeServerInfo(mut msg: *mut sizebuf_t) {
    let mut pResource: *mut resource_t =
        0 as *mut resource_t; // server is changed
    let mut pResName: *const libc::c_char = 0 as *const libc::c_char;
    let mut gametype: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    Con_Reportf(b"Serverdata packet received.\n\x00" as *const u8 as
                    *const libc::c_char);
    cls.timestart = Sys_DoubleTime();
    cls.demowaiting = false_0;
    // wipe the client_t struct
    if cls.changelevel as u64 == 0 && cls.changedemo as u64 == 0 {
        CL_ClearState();
    }
    cl.background =
        if cls.demonum != -(1 as libc::c_int) {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int } as qboolean;
    cls.state = ca_connected;
    // parse protocol version number
    i = MSG_ReadLong(msg);
    if i != 15 as libc::c_int {
        Con_Printf(b"\n^1Error:^7 Server use invalid protocol (%i should be %i)\n\x00"
                       as *const u8 as *const libc::c_char, i,
                   15 as libc::c_int);
        CL_StopPlayback();
        Host_AbortCurrentFrame();
    }
    cl.maxclients = MSG_ReadByte(msg);
    gametype = MSG_ReadByte(msg);
    clgame.maxEntities = (*SI.GameInfo).max_edicts;
    clgame.maxEntities =
        if clgame.maxEntities >= 600 as libc::c_int {
            if clgame.maxEntities < (1 as libc::c_int) << 13 as libc::c_int {
                clgame.maxEntities
            } else { ((1 as libc::c_int)) << 13 as libc::c_int }
        } else { 600 as libc::c_int };
    clgame.maxModels = 1024 as libc::c_int;
    Q_strncpy(clgame.maptitle.as_mut_ptr(), MSG_ReadStringExt(msg, false_0),
              256 as libc::c_int as size_t);
    // Re-init hud video, especially if we changed game directories
    clgame.dllFuncs.pfnVidInit.expect("non-null function pointer")();
    if Con_FixedFont() as u64 != 0 {
        // seperate the printfs so the server message can have a color
        Con_Print(b"\n\x1d\x1e\x1e\x1e\x1e\x1e\x1e\x1e\x1e\x1e\x1e\x1e\x1e\x1e\x1e\x1e\x1e\x1e\x1e\x1e\x1e\x1e\x1e\x1e\x1e\x1e\x1e\x1e\x1e\x1e\x1e\x1e\x1e\x1e\x1e\x1e\x1f\n\x00"
                      as *const u8 as *const libc::c_char);
        Con_Print(va(b"%c%s\n\n\x00" as *const u8 as *const libc::c_char,
                     2 as libc::c_int, clgame.maptitle.as_mut_ptr()));
    }
    // multiplayer game?
    if cl.maxclients > 1 as libc::c_int {
        // allow console in multiplayer games
        host.allow_console = true_0;
        // loading user settings
        CSCR_LoadDefaultCVars(b"user.scr\x00" as *const u8 as
                                  *const libc::c_char);
        if (*r_decals).value > mp_decals.value {
            Cvar_SetValue(b"r_decals\x00" as *const u8 as *const libc::c_char,
                          mp_decals.value);
        }
    } else {
        Cvar_Reset(b"r_decals\x00" as *const u8 as *const libc::c_char);
    }
    // re-init mouse
    if cl.background as u64 != 0 { host.mouse_visible = false_0 }
    if cl.background as u64 != 0 {
        // tell the game parts about background state
        Cvar_FullSet(b"cl_background\x00" as *const u8 as *const libc::c_char,
                     b"1\x00" as *const u8 as *const libc::c_char,
                     (1 as libc::c_int) << 17 as libc::c_int);
    } else {
        Cvar_FullSet(b"cl_background\x00" as *const u8 as *const libc::c_char,
                     b"0\x00" as *const u8 as *const libc::c_char,
                     (1 as libc::c_int) << 17 as libc::c_int);
    }
    S_StopBackgroundTrack();
    if cls.changedemo as u64 == 0 {
        UI_SetActiveMenu(cl.background);
    } else if cls.demoplayback == 0 {
        Key_SetKeyDest(key_menu as libc::c_int);
    }
    // don't reset cursor in background mode
    if cl.background as u64 != 0 { IN_MouseRestorePos(); }
    // will be changed later
    cl.viewentity = cl.playernum + 1 as libc::c_int; // re-arrange edicts
    (*gameui.globals).maxClients = cl.maxclients;
    Q_strncpy((*gameui.globals).maptitle.as_mut_ptr(),
              clgame.maptitle.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    if cls.changelevel as u64 == 0 && cls.changedemo as u64 == 0 {
        CL_InitEdicts();
    }
    // Quake just have a large packet of initialization data
    i = 1 as libc::c_int; // end of list
    while i < 1024 as libc::c_int {
        pResName = MSG_ReadStringExt(msg, false_0); // end of list
        if if pResName.is_null() || *pResName == 0 {
               0 as libc::c_int
           } else { 1 as libc::c_int } == 0 {
            break ;
        }
        pResource =
            _Mem_Alloc(cls.mempool,
                       ::std::mem::size_of::<resource_t>() as libc::c_ulong,
                       true_0,
                       b"../engine/client/cl_qparse.c\x00" as *const u8 as
                           *const libc::c_char, 275 as libc::c_int) as
                *mut resource_t;
        (*pResource).type_0 = t_model;
        Q_strncpy((*pResource).szFileName.as_mut_ptr(), pResName,
                  ::std::mem::size_of::<[libc::c_char; 64]>() as
                      libc::c_ulong);
        if i == 1 as libc::c_int {
            Q_strncpy(clgame.mapname.as_mut_ptr(), pResName,
                      ::std::mem::size_of::<string>() as libc::c_ulong);
        }
        (*pResource).nDownloadSize = -(1 as libc::c_int);
        (*pResource).nIndex = i;
        CL_AddToResourceList(pResource, &mut cl.resourcesneeded);
        i += 1
    }
    i = 1 as libc::c_int;
    while i < (1 as libc::c_int) << 11 as libc::c_int {
        pResName = MSG_ReadStringExt(msg, false_0);
        if if pResName.is_null() || *pResName == 0 {
               0 as libc::c_int
           } else { 1 as libc::c_int } == 0 {
            break ;
        }
        pResource =
            _Mem_Alloc(cls.mempool,
                       ::std::mem::size_of::<resource_t>() as libc::c_ulong,
                       true_0,
                       b"../engine/client/cl_qparse.c\x00" as *const u8 as
                           *const libc::c_char, 293 as libc::c_int) as
                *mut resource_t;
        (*pResource).type_0 = t_sound;
        Q_strncpy((*pResource).szFileName.as_mut_ptr(), pResName,
                  ::std::mem::size_of::<[libc::c_char; 64]>() as
                      libc::c_ulong);
        (*pResource).nDownloadSize = -(1 as libc::c_int);
        (*pResource).nIndex = i;
        CL_AddToResourceList(pResource, &mut cl.resourcesneeded);
        i += 1
    }
    // get splash name
    if cls.demoplayback != 0 && cls.demonum != -(1 as libc::c_int) {
        Cvar_Set(b"cl_levelshot_name\x00" as *const u8 as *const libc::c_char,
                 va(b"levelshots/%s_%s\x00" as *const u8 as
                        *const libc::c_char, cls.demoname.as_mut_ptr(),
                    if refState.wideScreen as libc::c_uint != 0 {
                        b"16x9\x00" as *const u8 as *const libc::c_char
                    } else {
                        b"4x3\x00" as *const u8 as *const libc::c_char
                    })); // reset progress bar
    } else {
        Cvar_Set(b"cl_levelshot_name\x00" as *const u8 as *const libc::c_char,
                 va(b"levelshots/%s_%s\x00" as *const u8 as
                        *const libc::c_char, clgame.mapname.as_mut_ptr(),
                    if refState.wideScreen as libc::c_uint != 0 {
                        b"16x9\x00" as *const u8 as *const libc::c_char
                    } else {
                        b"4x3\x00" as *const u8 as *const libc::c_char
                    })); // render a black screen
    }
    Cvar_SetValue(b"scr_loading\x00" as *const u8 as *const libc::c_char,
                  0.0f32);
    if (*cl_allow_levelshots).value != 0. && cls.changelevel as u64 == 0 ||
           cl.background as libc::c_uint != 0 {
        if FS_FileExists(va(b"%s.bmp\x00" as *const u8 as *const libc::c_char,
                            (*cl_levelshot_name).string),
                         true_0 as libc::c_int) == 0 {
            Cvar_Set(b"cl_levelshot_name\x00" as *const u8 as
                         *const libc::c_char,
                     b"*black\x00" as *const u8 as *const libc::c_char);
        }
        cls.scrshot_request = scrshot_plaque
        // request levelshot even if exist (check filetime)
    }
    memset(&mut clgame.movevars as *mut movevars_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<movevars_t>() as libc::c_ulong);
    memset(&mut clgame.oldmovevars as *mut movevars_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<movevars_t>() as libc::c_ulong);
    memset(&mut clgame.centerPrint as *mut center_print_t as
               *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<center_print_t>() as libc::c_ulong);
    cl.video_prepped = false_0;
    cl.audio_prepped = false_0;
    // GAME_COOP or GAME_DEATHMATCH
    CL_UpdateQuakeGameMode(gametype);
    // now we can start to precache
    CL_BatchResourceRequest(true_0); // 8192 * 1.74
    clgame.movevars.wateralpha =
        1.0f32; // quake doesn't write gravity in demos
    (*clgame.entities).curstate.scale = 0.0f32;
    clgame.movevars.waveHeight = 0.0f32;
    clgame.movevars.zmax = 14172.0f32;
    clgame.movevars.gravity = 800.0f32;
    clgame.movevars.maxvelocity = 2000.0f32;
    memcpy(&mut clgame.oldmovevars as *mut movevars_t as *mut libc::c_void,
           &mut clgame.movevars as *mut movevars_t as *const libc::c_void,
           ::std::mem::size_of::<movevars_t>() as libc::c_ulong);
}
/*
==================
CL_ParseQuakeClientData

==================
*/
unsafe extern "C" fn CL_ParseQuakeClientData(mut msg: *mut sizebuf_t) {
    let mut i: libc::c_int = 0;
    let mut bits: libc::c_int = MSG_ReadWord(msg);
    let mut frame: *mut frame_t = 0 as *mut frame_t;
    // this is the frame update that this message corresponds to
    i =
        cls.netchan.incoming_sequence as
            libc::c_int; // ack'd incoming messages.
    cl.parsecount = i; // index into window.
    cl.parsecountmod =
        cl.parsecount &
            CL_UPDATE_BACKUP - 1 as libc::c_int; // frame at index.
    frame =
        &mut *cl.frames.as_mut_ptr().offset(cl.parsecountmod as isize) as
            *mut frame_t; // mark network received time
    (*frame).time =
        cl.mtime[0 as libc::c_int as usize]; // time now that we are parsing.
    (*frame).receivedtime = host.realtime; // assume valid
    memset(&mut (*frame).graphdata as *mut netbandwidthgraph_t as
               *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<netbandwidthgraph_t>() as libc::c_ulong);
    memset((*frame).flags.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[byte; 256]>() as libc::c_ulong);
    (*frame).first_entity = cls.next_client_entities;
    (*frame).num_entities = 0 as libc::c_int;
    (*frame).valid = true_0;
    if bits & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        (*frame).clientdata.view_ofs[2 as libc::c_int as usize] =
            MSG_ReadChar(msg) as vec_t
    } else {
        (*frame).clientdata.view_ofs[2 as libc::c_int as usize] = 22.0f32
    }
    if bits & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        cl.local.idealpitch = MSG_ReadChar(msg) as libc::c_float
    } else { cl.local.idealpitch = 0 as libc::c_int as libc::c_float }
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if bits & ((1 as libc::c_int) << 2 as libc::c_int) << i != 0 {
            (*frame).clientdata.punchangle[i as usize] =
                MSG_ReadChar(msg) as libc::c_float
        } else { (*frame).clientdata.punchangle[i as usize] = 0.0f32 }
        if bits & ((1 as libc::c_int) << 5 as libc::c_int) << i != 0 {
            (*frame).clientdata.velocity[i as usize] =
                MSG_ReadChar(msg) as libc::c_float * 16.0f32
        } else {
            (*frame).clientdata.velocity[i as usize] =
                0 as libc::c_int as vec_t
        }
        i += 1
    }
    if bits & (1 as libc::c_int) << 10 as libc::c_int != 0 {
        (*frame).clientdata.flags =
            ((*frame).clientdata.flags as libc::c_uint |
                 (1 as libc::c_uint) << 9 as libc::c_int) as libc::c_int
    }
    if bits & (1 as libc::c_int) << 11 as libc::c_int != 0 {
        (*frame).clientdata.flags =
            ((*frame).clientdata.flags as libc::c_uint |
                 (1 as libc::c_uint) << 4 as libc::c_int) as libc::c_int
    }
    // [always sent]
    MSG_WriteLong(&mut msg_demo, MSG_ReadLong(msg));
    CL_DispatchQuakeMessage(b"Items\x00" as *const u8 as *const libc::c_char);
    if bits & (1 as libc::c_int) << 12 as libc::c_int != 0 {
        CL_UpdateQuakeStats(msg, 5 as libc::c_int, true_0);
    } else { CL_UpdateQuakeStats(msg, 5 as libc::c_int, false_0); }
    if bits & (1 as libc::c_int) << 13 as libc::c_int != 0 {
        CL_UpdateQuakeStats(msg, 4 as libc::c_int, true_0);
    } else { CL_UpdateQuakeStats(msg, 4 as libc::c_int, false_0); }
    if bits & (1 as libc::c_int) << 14 as libc::c_int != 0 {
        (*frame).clientdata.viewmodel =
            CL_UpdateQuakeStats(msg, 2 as libc::c_int, true_0)
    } else {
        (*frame).clientdata.viewmodel =
            CL_UpdateQuakeStats(msg, 2 as libc::c_int, false_0)
    }
    cl.local.health = CL_UpdateQuakeStats(msg, 0 as libc::c_int, true_0);
    CL_UpdateQuakeStats(msg, 3 as libc::c_int, true_0);
    CL_UpdateQuakeStats(msg, 6 as libc::c_int, true_0);
    CL_UpdateQuakeStats(msg, 7 as libc::c_int, true_0);
    CL_UpdateQuakeStats(msg, 8 as libc::c_int, true_0);
    CL_UpdateQuakeStats(msg, 9 as libc::c_int, true_0);
    CL_UpdateQuakeStats(msg, 10 as libc::c_int, true_0);
}
/*
==================
CL_ParseQuakeEntityData

Parse an entity update message from the server
If an entities model or origin changes from frame to frame, it must be
relinked.  Other attributes can change without relinking.
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseQuakeEntityData(mut msg: *mut sizebuf_t,
                                                 mut bits: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut newnum: libc::c_int = 0;
    let mut pack: libc::c_int = 0;
    let mut forcelink: qboolean = false_0;
    let mut state: *mut entity_state_t = 0 as *mut entity_state_t;
    let mut frame: *mut frame_t = 0 as *mut frame_t;
    let mut ent: *mut cl_entity_t = 0 as *mut cl_entity_t;
    // first update is the final signon stage where we actually receive an entity (i.e., the world at least)
    if cls.signon == 2 as libc::c_int - 1 as libc::c_int {
        // we are done with signon sequence.
        cls.signon = 2 as libc::c_int;
        // Clear loading plaque.
        CL_SignonReply();
    }
    // alloc next slot to store update
    state =
        &mut *cls.packet_entities.offset((cls.next_client_entities %
                                              cls.num_client_entities) as
                                             isize) as *mut entity_state_t;
    cl.validsequence = cls.netchan.incoming_sequence as libc::c_int;
    frame =
        &mut *cl.frames.as_mut_ptr().offset(cl.parsecountmod as isize) as
            *mut frame_t;
    pack = (*frame).num_entities;
    if bits & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        i = MSG_ReadByte(msg);
        bits = bits | i << 8 as libc::c_int
    }
    if bits & (1 as libc::c_int) << 14 as libc::c_int != 0 {
        newnum = MSG_ReadWord(msg)
    } else { newnum = MSG_ReadByte(msg) }
    memset(state as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<entity_state_t>() as libc::c_ulong);
    (*state).entityType =
        (*state).entityType | (1 as libc::c_int) << 0 as libc::c_int;
    (*state).number = newnum;
    // mark all the players
    ent = CL_EDICT_NUM(newnum); // enumerate entity index
    (*ent).index = newnum; // no previous frame to lerp from
    (*ent).player = CL_IsPlayerIndex(newnum);
    (*state).animtime = cl.mtime[0 as libc::c_int as usize] as libc::c_float;
    if (*ent).curstate.msg_time as libc::c_double !=
           cl.mtime[1 as libc::c_int as usize] {
        forcelink = true_0
    } else { forcelink = false_0 }
    if bits & (1 as libc::c_int) << 10 as libc::c_int != 0 {
        (*state).modelindex = MSG_ReadByte(msg)
    } else { (*state).modelindex = (*ent).baseline.modelindex }
    if bits & (1 as libc::c_int) << 6 as libc::c_int != 0 {
        (*state).frame = MSG_ReadByte(msg) as libc::c_float
    } else { (*state).frame = (*ent).baseline.frame }
    if bits & (1 as libc::c_int) << 11 as libc::c_int != 0 {
        (*state).colormap = MSG_ReadByte(msg)
    } else { (*state).colormap = (*ent).baseline.colormap }
    if bits & (1 as libc::c_int) << 12 as libc::c_int != 0 {
        (*state).skin = MSG_ReadByte(msg) as libc::c_short
    } else { (*state).skin = (*ent).baseline.skin }
    if bits & (1 as libc::c_int) << 13 as libc::c_int != 0 {
        (*state).effects = MSG_ReadByte(msg)
    } else { (*state).effects = (*ent).baseline.effects }
    if bits & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        (*state).origin[0 as libc::c_int as usize] = MSG_ReadCoord(msg)
    } else {
        (*state).origin[0 as libc::c_int as usize] =
            (*ent).baseline.origin[0 as libc::c_int as usize]
    }
    if bits & (1 as libc::c_int) << 8 as libc::c_int != 0 {
        (*state).angles[0 as libc::c_int as usize] =
            MSG_ReadChar(msg) as libc::c_float * (360.0f32 / 256.0f32)
    } else {
        (*state).angles[0 as libc::c_int as usize] =
            (*ent).baseline.angles[0 as libc::c_int as usize]
    }
    if bits & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        (*state).origin[1 as libc::c_int as usize] = MSG_ReadCoord(msg)
    } else {
        (*state).origin[1 as libc::c_int as usize] =
            (*ent).baseline.origin[1 as libc::c_int as usize]
    }
    if bits & (1 as libc::c_int) << 4 as libc::c_int != 0 {
        (*state).angles[1 as libc::c_int as usize] =
            MSG_ReadChar(msg) as libc::c_float * (360.0f32 / 256.0f32)
    } else {
        (*state).angles[1 as libc::c_int as usize] =
            (*ent).baseline.angles[1 as libc::c_int as usize]
    }
    if bits & (1 as libc::c_int) << 3 as libc::c_int != 0 {
        (*state).origin[2 as libc::c_int as usize] = MSG_ReadCoord(msg)
    } else {
        (*state).origin[2 as libc::c_int as usize] =
            (*ent).baseline.origin[2 as libc::c_int as usize]
    }
    if bits & (1 as libc::c_int) << 9 as libc::c_int != 0 {
        (*state).angles[2 as libc::c_int as usize] =
            MSG_ReadChar(msg) as libc::c_float * (360.0f32 / 256.0f32)
    } else {
        (*state).angles[2 as libc::c_int as usize] =
            (*ent).baseline.angles[2 as libc::c_int as usize]
    }
    if bits & (1 as libc::c_int) << 15 as libc::c_int != 0 {
        let mut temp: libc::c_int = MSG_ReadFloat(msg) as libc::c_int;
        let mut alpha: libc::c_float = MSG_ReadFloat(msg);
        if alpha == 0.0f32 { alpha = 1.0f32 }
        if alpha < 1.0f32 {
            (*state).rendermode = kRenderTransTexture as libc::c_int;
            (*state).renderamt = (alpha * 255.0f32) as libc::c_int
        }
        if temp == 2 as libc::c_int && MSG_ReadFloat(msg) != 0. {
            (*state).effects =
                ((*state).effects as libc::c_uint |
                     (1 as libc::c_uint) << 27 as libc::c_int) as libc::c_int
        }
    }
    if bits & (1 as libc::c_int) << 5 as libc::c_int != 0 {
        (*state).movetype = 4 as libc::c_int
    } else { (*state).movetype = 8 as libc::c_int }
    if CL_QuakeEntityTeleported(ent, state) as u64 != 0 {
        // remove smooth stepping
        if cl.viewentity == (*ent).index { cl.skip_interp = true_0 }
        forcelink = true_0
    }
    if (*state).effects & 16 as libc::c_int != 0 {
        (*state).effects = (*state).effects | 128 as libc::c_int
    }
    if newnum - 1 as libc::c_int == cl.playernum {
        (*frame).clientdata.origin[0 as libc::c_int as usize] =
            (*state).origin[0 as libc::c_int as usize];
        (*frame).clientdata.origin[1 as libc::c_int as usize] =
            (*state).origin[1 as libc::c_int as usize];
        (*frame).clientdata.origin[2 as libc::c_int as usize] =
            (*state).origin[2 as libc::c_int as usize]
    }
    if forcelink as u64 != 0 {
        (*ent).baseline.vuser1[0 as libc::c_int as usize] =
            (*state).origin[0 as libc::c_int as usize];
        (*ent).baseline.vuser1[1 as libc::c_int as usize] =
            (*state).origin[1 as libc::c_int as usize];
        (*ent).baseline.vuser1[2 as libc::c_int as usize] =
            (*state).origin[2 as libc::c_int as usize];
        (*state).effects = (*state).effects | 32 as libc::c_int;
        // interpolation must be reset
        if pack >= 0 as libc::c_int {
            (*frame).flags[(pack >> 3 as libc::c_int) as usize] =
                ((*frame).flags[(pack >> 3 as libc::c_int) as usize] as
                     libc::c_int |
                     (1 as libc::c_int) << (pack & 7 as libc::c_int)) as byte
        } else { };
        // release beams from previous entity
        CL_KillDeadBeams(ent);
    }
    // add entity to packet
    cls.next_client_entities += 1;
    (*frame).num_entities += 1;
}
/*
==================
CL_ParseQuakeParticles

==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseQuakeParticle(mut msg: *mut sizebuf_t) {
    let mut count: libc::c_int = 0;
    let mut color: libc::c_int = 0;
    let mut org: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
    MSG_ReadVec3Coord(msg, org.as_mut_ptr());
    dir[0 as libc::c_int as usize] =
        MSG_ReadChar(msg) as libc::c_float * 0.0625f32;
    dir[1 as libc::c_int as usize] =
        MSG_ReadChar(msg) as libc::c_float * 0.0625f32;
    dir[2 as libc::c_int as usize] =
        MSG_ReadChar(msg) as libc::c_float * 0.0625f32;
    count = MSG_ReadByte(msg);
    color = MSG_ReadByte(msg);
    if count == 255 as libc::c_int { count = 1024 as libc::c_int }
    R_RunParticleEffect(org.as_mut_ptr() as *const vec_t,
                        dir.as_mut_ptr() as *const vec_t, color, count);
}
/*
===================
CL_ParseQuakeStaticSound

===================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseQuakeStaticSound(mut msg: *mut sizebuf_t) {
    let mut sound_num: libc::c_int = 0;
    let mut vol: libc::c_float = 0.;
    let mut attn: libc::c_float = 0.;
    let mut org: vec3_t = [0.; 3];
    MSG_ReadVec3Coord(msg, org.as_mut_ptr());
    sound_num = MSG_ReadByte(msg);
    vol = MSG_ReadByte(msg) as libc::c_float / 255.0f32;
    attn = MSG_ReadByte(msg) as libc::c_float / 64.0f32;
    S_StartSound(org.as_mut_ptr() as *const vec_t, 0 as libc::c_int,
                 6 as libc::c_int,
                 cl.sound_index[sound_num as usize] as sound_t, vol, attn,
                 100 as libc::c_int, 0 as libc::c_int);
}
/*
==================
CL_ParseQuakeDamage

redirect to qwrap->client
==================
*/
unsafe extern "C" fn CL_ParseQuakeDamage(mut msg: *mut sizebuf_t) {
    MSG_WriteByte(&mut msg_demo, MSG_ReadByte(msg)); // armor
    MSG_WriteByte(&mut msg_demo, MSG_ReadByte(msg)); // blood
    MSG_WriteCoord(&mut msg_demo, MSG_ReadCoord(msg)); // direction
    MSG_WriteCoord(&mut msg_demo, MSG_ReadCoord(msg)); // direction
    MSG_WriteCoord(&mut msg_demo, MSG_ReadCoord(msg)); // direction
    CL_DispatchQuakeMessage(b"Damage\x00" as *const u8 as
                                *const libc::c_char);
}
/*
===================
CL_ParseQuakeStaticEntity

===================
*/
unsafe extern "C" fn CL_ParseQuakeStaticEntity(mut msg: *mut sizebuf_t) {
    let mut state: entity_state_t =
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
                       rendercolor: color24{r: 0, g: 0, b: 0,},
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
                       vuser4: [0.; 3],}; // ???
    let mut ent: *mut cl_entity_t = 0 as *mut cl_entity_t;
    let mut i: libc::c_int = 0;
    memset(&mut state as *mut entity_state_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<entity_state_t>() as libc::c_ulong);
    state.modelindex = MSG_ReadByte(msg);
    state.frame = MSG_ReadByte(msg) as libc::c_float;
    state.colormap = MSG_ReadByte(msg);
    state.skin = MSG_ReadByte(msg) as libc::c_short;
    state.origin[0 as libc::c_int as usize] = MSG_ReadCoord(msg);
    state.angles[0 as libc::c_int as usize] =
        MSG_ReadChar(msg) as libc::c_float * (360.0f32 / 256.0f32);
    state.origin[1 as libc::c_int as usize] = MSG_ReadCoord(msg);
    state.angles[1 as libc::c_int as usize] =
        MSG_ReadChar(msg) as libc::c_float * (360.0f32 / 256.0f32);
    state.origin[2 as libc::c_int as usize] = MSG_ReadCoord(msg);
    state.angles[2 as libc::c_int as usize] =
        MSG_ReadChar(msg) as libc::c_float * (360.0f32 / 256.0f32);
    i = clgame.numStatics;
    if i >= 3096 as libc::c_int {
        Con_Printf(b"^1Error:^7 CL_ParseStaticEntity: static entities limit exceeded!\n\x00"
                       as *const u8 as *const libc::c_char);
        return
    }
    ent = &mut *clgame.static_entities.offset(i as isize) as *mut cl_entity_t;
    clgame.numStatics += 1;
    (*ent).index = 0 as libc::c_int;
    (*ent).baseline = state;
    (*ent).curstate = state;
    (*ent).prevstate = state;
    // statics may be respawned in game e.g. for demo recording
    if cls.state as libc::c_uint ==
           ca_connected as libc::c_int as libc::c_uint ||
           cls.state as libc::c_uint ==
               ca_validate as libc::c_int as libc::c_uint {
        (*ent).trivial_accept = 0xffff as libc::c_int
    }
    // setup the new static entity
    (*ent).origin[0 as libc::c_int as usize] =
        (*ent).curstate.origin[0 as libc::c_int as usize];
    (*ent).origin[1 as libc::c_int as usize] =
        (*ent).curstate.origin[1 as libc::c_int as usize];
    (*ent).origin[2 as libc::c_int as usize] =
        (*ent).curstate.origin[2 as libc::c_int as usize];
    (*ent).angles[0 as libc::c_int as usize] =
        (*ent).curstate.angles[0 as libc::c_int as usize];
    (*ent).angles[1 as libc::c_int as usize] =
        (*ent).curstate.angles[1 as libc::c_int as usize];
    (*ent).angles[2 as libc::c_int as usize] =
        (*ent).curstate.angles[2 as libc::c_int as usize];
    (*ent).model = CL_ModelHandle(state.modelindex);
    (*ent).curstate.framerate = 1.0f32;
    CL_ResetLatchedVars(ent, true_0);
    if !(*ent).model.is_null() {
        // auto 'solid' faces
        if (*(*ent).model).flags as libc::c_uint &
               (1 as libc::c_uint) << 3 as libc::c_int != 0 &&
               Host_IsQuakeCompatible() as libc::c_uint != 0 {
            (*ent).curstate.rendermode = kRenderTransAlpha as libc::c_int;
            (*ent).curstate.renderamt = 255 as libc::c_int
        }
    }
    R_AddEfrags(ent);
    // add link
}
/*
===================
CL_ParseQuakeBaseline

===================
*/
unsafe extern "C" fn CL_ParseQuakeBaseline(mut msg: *mut sizebuf_t) {
    let mut state: entity_state_t =
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
                       rendercolor: color24{r: 0, g: 0, b: 0,},
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
                       vuser4: [0.; 3],}; // entnum
    let mut ent: *mut cl_entity_t = 0 as *mut cl_entity_t;
    let mut newnum: libc::c_int = 0;
    memset(&mut state as *mut entity_state_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<entity_state_t>() as libc::c_ulong);
    newnum = MSG_ReadWord(msg);
    if newnum >= clgame.maxEntities {
        Host_Error(b"CL_AllocEdict: no free edicts\n\x00" as *const u8 as
                       *const libc::c_char);
    }
    ent = CL_EDICT_NUM(newnum);
    memset(&mut (*ent).prevstate as *mut entity_state_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<entity_state_t>() as libc::c_ulong);
    (*ent).index = newnum;
    // parse baseline
    state.modelindex = MSG_ReadByte(msg);
    state.frame = MSG_ReadByte(msg) as libc::c_float;
    state.colormap = MSG_ReadByte(msg);
    state.skin = MSG_ReadByte(msg) as libc::c_short;
    state.origin[0 as libc::c_int as usize] = MSG_ReadCoord(msg);
    state.angles[0 as libc::c_int as usize] =
        MSG_ReadChar(msg) as libc::c_float * (360.0f32 / 256.0f32);
    state.origin[1 as libc::c_int as usize] = MSG_ReadCoord(msg);
    state.angles[1 as libc::c_int as usize] =
        MSG_ReadChar(msg) as libc::c_float * (360.0f32 / 256.0f32);
    state.origin[2 as libc::c_int as usize] = MSG_ReadCoord(msg);
    state.angles[2 as libc::c_int as usize] =
        MSG_ReadChar(msg) as libc::c_float * (360.0f32 / 256.0f32);
    (*ent).player = CL_IsPlayerIndex(newnum);
    memcpy(&mut (*ent).baseline as *mut entity_state_t as *mut libc::c_void,
           &mut state as *mut entity_state_t as *const libc::c_void,
           ::std::mem::size_of::<entity_state_t>() as libc::c_ulong);
    memcpy(&mut (*ent).prevstate as *mut entity_state_t as *mut libc::c_void,
           &mut state as *mut entity_state_t as *const libc::c_void,
           ::std::mem::size_of::<entity_state_t>() as libc::c_ulong);
}
/*
===================
CL_ParseQuakeTempEntity

===================
*/
unsafe extern "C" fn CL_ParseQuakeTempEntity(mut msg: *mut sizebuf_t) {
    let mut type_0: libc::c_int = MSG_ReadByte(msg);
    MSG_WriteByte(&mut msg_demo, type_0);
    if type_0 == 17 as libc::c_int {
        MSG_WriteString(&mut msg_demo, MSG_ReadStringExt(msg, false_0));
    }
    // TE_LIGHTNING1, TE_LIGHTNING2, TE_LIGHTNING3, TE_BEAM, TE_LIGHTNING4
    if type_0 == 5 as libc::c_int || type_0 == 6 as libc::c_int ||
           type_0 == 9 as libc::c_int || type_0 == 13 as libc::c_int ||
           type_0 == 17 as libc::c_int {
        MSG_WriteWord(&mut msg_demo, MSG_ReadWord(msg));
    }
    // all temp ents have position at beginning
    MSG_WriteCoord(&mut msg_demo, MSG_ReadCoord(msg));
    MSG_WriteCoord(&mut msg_demo, MSG_ReadCoord(msg));
    MSG_WriteCoord(&mut msg_demo, MSG_ReadCoord(msg));
    // TE_LIGHTNING1, TE_LIGHTNING2, TE_LIGHTNING3, TE_BEAM, TE_EXPLOSION3, TE_LIGHTNING4
    if type_0 == 5 as libc::c_int || type_0 == 6 as libc::c_int ||
           type_0 == 9 as libc::c_int || type_0 == 13 as libc::c_int ||
           type_0 == 16 as libc::c_int || type_0 == 17 as libc::c_int {
        // write endpos for beams
        MSG_WriteCoord(&mut msg_demo, MSG_ReadCoord(msg));
        MSG_WriteCoord(&mut msg_demo, MSG_ReadCoord(msg));
        MSG_WriteCoord(&mut msg_demo, MSG_ReadCoord(msg));
    }
    // TE_EXPLOSION2
    if type_0 == 12 as libc::c_int {
        MSG_WriteByte(&mut msg_demo, MSG_ReadByte(msg));
        MSG_WriteByte(&mut msg_demo, MSG_ReadByte(msg));
    }
    // TE_SMOKE (nehahra)
    if type_0 == 18 as libc::c_int {
        MSG_WriteByte(&mut msg_demo, MSG_ReadByte(msg));
    }
    CL_DispatchQuakeMessage(b"TempEntity\x00" as *const u8 as
                                *const libc::c_char);
}
/*
===================
CL_ParseQuakeSignon

very important message
===================
*/
unsafe extern "C" fn CL_ParseQuakeSignon(mut msg: *mut sizebuf_t) {
    let mut i: libc::c_int = MSG_ReadByte(msg);
    if i == 3 as libc::c_int {
        cls.signon = 2 as libc::c_int - 1 as libc::c_int
    }
    Con_Reportf(b"CL_Signon: %d\n\x00" as *const u8 as *const libc::c_char,
                i);
}
/*
==================
CL_ParseNehahraShowLMP

redirect to qwrap->client
==================
*/
unsafe extern "C" fn CL_ParseNehahraShowLMP(mut msg: *mut sizebuf_t) {
    MSG_WriteString(&mut msg_demo, MSG_ReadStringExt(msg, false_0));
    MSG_WriteString(&mut msg_demo, MSG_ReadStringExt(msg, false_0));
    MSG_WriteByte(&mut msg_demo, MSG_ReadByte(msg));
    MSG_WriteByte(&mut msg_demo, MSG_ReadByte(msg));
    CL_DispatchQuakeMessage(b"Stats\x00" as *const u8 as *const libc::c_char);
}
/*
==================
CL_ParseNehahraHideLMP

redirect to qwrap->client
==================
*/
unsafe extern "C" fn CL_ParseNehahraHideLMP(mut msg: *mut sizebuf_t) {
    MSG_WriteString(&mut msg_demo, MSG_ReadStringExt(msg, false_0));
    CL_DispatchQuakeMessage(b"Stats\x00" as *const u8 as *const libc::c_char);
}
/*
==================
CL_QuakeStuffText

==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_QuakeStuffText(mut text: *const libc::c_char) {
    Q_strncat(cmd_buf.as_mut_ptr(), text,
              ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong);
    // a1ba: didn't filtered, anyway quake protocol
	// only supported for demos, not network games
    Cbuf_AddText(text);
}
/*
==================
CL_QuakeExecStuff

==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_QuakeExecStuff() {
    let mut text: *mut libc::c_char = cmd_buf.as_mut_ptr();
    let mut token: [libc::c_char; 256] = [0; 256];
    let mut argc: libc::c_int = 0 as libc::c_int;
    // check if no commands this frame
    if if text.is_null() || *text == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return
    }
    loop  {
        // skip whitespace up to a /n
        while *text as libc::c_int != 0 &&
                  *text as byte as libc::c_int <= ' ' as i32 &&
                  *text as libc::c_int != '\r' as i32 &&
                  *text as libc::c_int != '\n' as i32 {
            text = text.offset(1)
        }
        if *text as libc::c_int == '\n' as i32 ||
               *text as libc::c_int == '\r' as i32 {
            // a newline seperates commands in the buffer
            if *text as libc::c_int == '\r' as i32 &&
                   *text.offset(1 as libc::c_int as isize) as libc::c_int ==
                       '\n' as i32 {
                text = text.offset(1)
            }
            argc = 0 as libc::c_int;
            text = text.offset(1)
        }
        if *text == 0 { break ; }
        text =
            _COM_ParseFileSafe(text, token.as_mut_ptr(),
                               ::std::mem::size_of::<[libc::c_char; 256]>() as
                                   libc::c_ulong as libc::c_int,
                               ((1 as libc::c_int) << 0 as libc::c_int) as
                                   libc::c_uint, 0 as *mut libc::c_int);
        if text.is_null() { break ; }
        if argc == 0 as libc::c_int {
            // debug: find all missed commands and cvars to add them into QWrap
            if Cvar_Exists(token.as_mut_ptr()) as u64 == 0 &&
                   Cmd_Exists(token.as_mut_ptr()) as u64 == 0 {
                Con_Printf(b"^3Warning:^7 \'%s\' is not exist\n\x00" as
                               *const u8 as *const libc::c_char,
                           token.as_mut_ptr());
            }
            //			else Msg( "cmd: %s\n", token );
            // process some special commands
            if Q_strnicmp(token.as_mut_ptr(),
                          b"playdemo\x00" as *const u8 as *const libc::c_char,
                          99999 as libc::c_int) == 0 {
                cls.changedemo = true_0
            }
            argc += 1
        }
    }
    // reset the buffer
    cmd_buf[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
}
/*
==================
CL_ParseQuakeMessage

==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseQuakeMessage(mut msg: *mut sizebuf_t,
                                              mut normal_message: qboolean) {
    let mut cmd: libc::c_int = 0; // updates each frame
    let mut param1: libc::c_int = 0; // begin parsing
    let mut param2: libc::c_int = 0;
    let mut bufStart: size_t = 0;
    let mut str: *const libc::c_char = 0 as *const libc::c_char;
    cls.starting_count = MSG_GetNumBytesWritten(msg);
    CL_Parse_Debug(true_0);
    // init excise buffer
    MSG_InitExt(&mut msg_demo,
                b"UserMsg\x00" as *const u8 as *const libc::c_char,
                msg_buf.as_mut_ptr() as *mut libc::c_void,
                ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong
                    as libc::c_int, -(1 as libc::c_int));
    if normal_message as u64 != 0 {
        // assume no entity/player update this packet
        if cls.state as libc::c_uint ==
               ca_active as libc::c_int as libc::c_uint {
            cl.frames[(cls.netchan.incoming_sequence &
                           (CL_UPDATE_BACKUP - 1 as libc::c_int) as
                               libc::c_uint) as usize].valid = false_0;
            cl.frames[(cls.netchan.incoming_sequence &
                           (CL_UPDATE_BACKUP - 1 as libc::c_int) as
                               libc::c_uint) as usize].choked = false_0
        } else {
            CL_ResetFrame(&mut *cl.frames.as_mut_ptr().offset((cls.netchan.incoming_sequence
                                                                   &
                                                                   (CL_UPDATE_BACKUP
                                                                        -
                                                                        1 as
                                                                            libc::c_int)
                                                                       as
                                                                       libc::c_uint)
                                                                  as isize));
        }
    }
    loop 
         // parse the message
         {
        if MSG_CheckOverflow(msg) as u64 != 0 {
            Host_Error(b"CL_ParseServerMessage: overflow!\n\x00" as *const u8
                           as *const libc::c_char);
            return
        }
        // mark start position
        bufStart = MSG_GetNumBytesWritten(msg) as size_t;
        // end of message (align bits)
        if MSG_GetNumBitsLeft(msg) < 8 as libc::c_int { break ; }
        cmd = MSG_ReadCmd(msg, NS_SERVER);
        // if the high bit of the command byte is set, it is a fast update
        if cmd & 128 as libc::c_int != 0 {
            CL_ParseQuakeEntityData(msg, cmd & 127 as libc::c_int);
        } else {
            // record command for debugging spew on parse problem
            CL_Parse_RecordCommand(cmd, bufStart as libc::c_int);
            // other commands
            match cmd {
                1 => { }
                2 => {
                    CL_DemoCompleted(); // new frame was started
                }
                3 => {
                    CL_ParseQuakeStats(msg); // make sure any stuffed commands are done
                }
                4 => {
                    param1 = MSG_ReadLong(msg);
                    if param1 != 15 as libc::c_int {
                        Host_Error(b"Server is protocol %i instead of %i\n\x00"
                                       as *const u8 as *const libc::c_char,
                                   param1, 15 as libc::c_int);
                    }
                }
                5 => { CL_ParseViewEntity(msg); }
                6 => {
                    CL_ParseQuakeSound(msg);
                    cl.frames[cl.parsecountmod as usize].graphdata.sound =
                        (cl.frames[cl.parsecountmod as usize].graphdata.sound
                             as
                             libc::c_ulong).wrapping_add((MSG_GetNumBytesWritten(msg)
                                                              as
                                                              libc::c_ulong).wrapping_sub(bufStart))
                            as word as word
                }
                7 => {
                    Cbuf_AddText(b"\n\x00" as *const u8 as
                                     *const libc::c_char);
                    CL_ParseServerTime(msg);
                }
                8 => {
                    str = MSG_ReadStringExt(msg, false_0);
                    Con_Printf(b"%s%s\x00" as *const u8 as
                                   *const libc::c_char, str,
                               if *str as libc::c_int == 2 as libc::c_int {
                                   b"\n\x00" as *const u8 as
                                       *const libc::c_char
                               } else {
                                   b"\x00" as *const u8 as *const libc::c_char
                               });
                }
                9 => { CL_QuakeStuffText(MSG_ReadStringExt(msg, false_0)); }
                10 => {
                    cl.viewangles[0 as libc::c_int as usize] =
                        MSG_ReadChar(msg) as libc::c_float *
                            (360.0f32 / 256.0f32);
                    cl.viewangles[1 as libc::c_int as usize] =
                        MSG_ReadChar(msg) as libc::c_float *
                            (360.0f32 / 256.0f32);
                    cl.viewangles[2 as libc::c_int as usize] =
                        MSG_ReadChar(msg) as libc::c_float *
                            (360.0f32 / 256.0f32)
                }
                11 => { Cbuf_Execute(); CL_ParseQuakeServerInfo(msg); }
                12 => {
                    param1 = MSG_ReadByte(msg);
                    str = MSG_ReadStringExt(msg, false_0);
                    CL_SetLightstyle(param1, str,
                                     cl.mtime[0 as libc::c_int as usize] as
                                         libc::c_float);
                }
                13 => {
                    param1 = MSG_ReadByte(msg);
                    Q_strncpy(cl.players[param1 as usize].name.as_mut_ptr(),
                              MSG_ReadStringExt(msg, false_0),
                              ::std::mem::size_of::<[libc::c_char; 32]>() as
                                  libc::c_ulong);
                    Q_strncpy(cl.players[param1 as usize].model.as_mut_ptr(),
                              b"player\x00" as *const u8 as
                                  *const libc::c_char,
                              ::std::mem::size_of::<[libc::c_char; 32]>() as
                                  libc::c_ulong);
                }
                14 => {
                    param1 = MSG_ReadByte(msg);
                    param2 = MSG_ReadShort(msg);
                    // HACKHACK: store frags into spectator
                    cl.players[param1 as usize].spectator = param2
                }
                15 => {
                    CL_ParseQuakeClientData(msg); // just an event
                    cl.frames[cl.parsecountmod as usize].graphdata.client =
                        (cl.frames[cl.parsecountmod as usize].graphdata.client
                             as
                             libc::c_ulong).wrapping_add((MSG_GetNumBytesWritten(msg)
                                                              as
                                                              libc::c_ulong).wrapping_sub(bufStart))
                            as word as word
                }
                16 => {
                    param1 = MSG_ReadWord(msg); // just an event
                    S_StopSound(param1 >> 3 as libc::c_int,
                                param1 & 7 as libc::c_int,
                                0 as *const libc::c_char); // tracknum
                    cl.frames[cl.parsecountmod as usize].graphdata.sound =
                        (cl.frames[cl.parsecountmod as usize].graphdata.sound
                             as
                             libc::c_ulong).wrapping_add((MSG_GetNumBytesWritten(msg)
                                                              as
                                                              libc::c_ulong).wrapping_sub(bufStart))
                            as word as word
                }
                17 => {
                    param1 = MSG_ReadByte(msg); // loopnum
                    param2 = MSG_ReadByte(msg); // open quake menu
                    cl.players[param1 as usize].topcolor =
                        param2 & 0xf as libc::c_int; // obsolete
                    cl.players[param1 as usize].bottomcolor =
                        (param2 & 0xf0 as libc::c_int) >> 4 as libc::c_int
                }
                18 => {
                    CL_ParseQuakeParticle(msg); // density
                }
                19 => {
                    CL_ParseQuakeDamage(msg); // red
                }
                20 => {
                    CL_ParseQuakeStaticEntity(msg); // green
                }
                21 => { }
                22 => {
                    CL_ParseQuakeBaseline(msg); // blue
                }
                23 => {
                    CL_ParseQuakeTempEntity(msg); // done
                    cl.frames[cl.parsecountmod as usize].graphdata.tentities =
                        (cl.frames[cl.parsecountmod as
                                       usize].graphdata.tentities as
                             libc::c_ulong).wrapping_add((MSG_GetNumBytesWritten(msg)
                                                              as
                                                              libc::c_ulong).wrapping_sub(bufStart))
                            as word as word
                }
                24 => { cl.paused = MSG_ReadByte(msg) as qboolean }
                25 => { CL_ParseQuakeSignon(msg); }
                26 => {
                    str = MSG_ReadStringExt(msg, false_0);
                    CL_DispatchUserMessage(b"HudText\x00" as *const u8 as
                                               *const libc::c_char,
                                           Q_strlen(str) as libc::c_int,
                                           str as *mut libc::c_void);
                }
                27 => {
                    CL_DispatchQuakeMessage(b"KillMonster\x00" as *const u8 as
                                                *const libc::c_char);
                }
                28 => {
                    CL_DispatchQuakeMessage(b"FoundSecret\x00" as *const u8 as
                                                *const libc::c_char);
                }
                29 => { CL_ParseQuakeStaticSound(msg); }
                30 => { cl.intermission = 1 as libc::c_int }
                31 => { CL_ParseFinaleCutscene(msg, 2 as libc::c_int); }
                32 => {
                    param1 = MSG_ReadByte(msg);
                    param1 =
                        if param1 >= 0 as libc::c_int {
                            if param1 < 32 as libc::c_int - 1 as libc::c_int {
                                param1
                            } else { (32 as libc::c_int) - 1 as libc::c_int }
                        } else { 0 as libc::c_int };
                    param2 = MSG_ReadByte(msg);
                    param2 =
                        if param2 >= 0 as libc::c_int {
                            if param2 < 32 as libc::c_int - 1 as libc::c_int {
                                param2
                            } else { (32 as libc::c_int) - 1 as libc::c_int }
                        } else { 0 as libc::c_int };
                    if (cls.demoplayback != 0 ||
                            cls.demorecording as libc::c_uint != 0) &&
                           cls.forcetrack != -(1 as libc::c_int) {
                        S_StartBackgroundTrack(clgame.cdtracks[cls.forcetrack
                                                                   as
                                                                   usize].as_mut_ptr(),
                                               clgame.cdtracks[cls.forcetrack
                                                                   as
                                                                   usize].as_mut_ptr(),
                                               0 as libc::c_int, false_0);
                    } else {
                        S_StartBackgroundTrack(clgame.cdtracks[param1 as
                                                                   usize].as_mut_ptr(),
                                               clgame.cdtracks[param2 as
                                                                   usize].as_mut_ptr(),
                                               0 as libc::c_int, false_0);
                    }
                }
                33 => {
                    Cmd_ExecuteString(b"help\x00" as *const u8 as
                                          *const libc::c_char);
                }
                34 => { CL_ParseFinaleCutscene(msg, 3 as libc::c_int); }
                36 => { CL_ParseNehahraHideLMP(msg); }
                35 => { CL_ParseNehahraShowLMP(msg); }
                37 => {
                    Q_strncpy(clgame.movevars.skyName.as_mut_ptr(),
                              MSG_ReadStringExt(msg, false_0),
                              ::std::mem::size_of::<[libc::c_char; 32]>() as
                                  libc::c_ulong);
                }
                50 => { MSG_ReadCoord(msg); }
                51 => {
                    if MSG_ReadByte(msg) != 0 {
                        let mut fog_settings: [libc::c_float; 4] = [0.; 4];
                        let mut packed_fog: [libc::c_int; 4] = [0; 4];
                        fog_settings[3 as libc::c_int as usize] =
                            MSG_ReadFloat(msg);
                        fog_settings[0 as libc::c_int as usize] =
                            MSG_ReadByte(msg) as libc::c_float;
                        fog_settings[1 as libc::c_int as usize] =
                            MSG_ReadByte(msg) as libc::c_float;
                        fog_settings[2 as libc::c_int as usize] =
                            MSG_ReadByte(msg) as libc::c_float;
                        packed_fog[0 as libc::c_int as usize] =
                            (fog_settings[0 as libc::c_int as usize] *
                                 255 as libc::c_int as libc::c_float) as
                                libc::c_int;
                        packed_fog[1 as libc::c_int as usize] =
                            (fog_settings[1 as libc::c_int as usize] *
                                 255 as libc::c_int as libc::c_float) as
                                libc::c_int;
                        packed_fog[2 as libc::c_int as usize] =
                            (fog_settings[2 as libc::c_int as usize] *
                                 255 as libc::c_int as libc::c_float) as
                                libc::c_int;
                        packed_fog[3 as libc::c_int as usize] =
                            (fog_settings[3 as libc::c_int as usize] *
                                 255 as libc::c_int as libc::c_float) as
                                libc::c_int;
                        clgame.movevars.fog_settings =
                            packed_fog[1 as libc::c_int as usize] <<
                                24 as libc::c_int |
                                packed_fog[2 as libc::c_int as usize] <<
                                    16 as libc::c_int |
                                packed_fog[3 as libc::c_int as usize] <<
                                    8 as libc::c_int |
                                packed_fog[0 as libc::c_int as usize]
                    } else { clgame.movevars.fog_settings = 0 as libc::c_int }
                }
                _ => {
                    Host_Error(b"CL_ParseServerMessage: Illegible server message\n\x00"
                                   as *const u8 as *const libc::c_char);
                }
            }
        }
    }
    cl.frames[cl.parsecountmod as usize].graphdata.msgbytes =
        (cl.frames[cl.parsecountmod as usize].graphdata.msgbytes as
             libc::c_int + (MSG_GetNumBytesWritten(msg) - cls.starting_count))
            as word;
    CL_Parse_Debug(false_0);
    // now process packet.
    CL_ProcessPacket(&mut *cl.frames.as_mut_ptr().offset(cl.parsecountmod as
                                                             isize));
    // add new entities into physic lists
    CL_SetSolidEntities();
    // check deferred cmds
    CL_QuakeExecStuff();
}
