#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, extern_types,
           register_tool)]
extern "C" {
    pub type file_s;
    pub type grasshdr_s;
    pub type IVoiceTweak_s;
    pub type demo_api_s;
    pub type engine_studio_api_s;
    pub type r_studio_interface_s;
    #[no_mangle]
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn UI_SetActiveMenu(fActive: qboolean);
    #[no_mangle]
    fn SCR_BeginLoadingPlaque(is_background: qboolean);
    #[no_mangle]
    fn CL_Disconnect();
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Q_atoi(str: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn Q_strnicmp(s1: *const libc::c_char, s2: *const libc::c_char,
                  n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn Q_snprintf(buffer: *mut libc::c_char, buffersize: size_t,
                  format: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn Q_sprintf(buffer: *mut libc::c_char, format: *const libc::c_char,
                 _: ...) -> libc::c_int;
    #[no_mangle]
    fn Q_pretifymem(value: libc::c_float, digitsafterdecimal: libc::c_int)
     -> *mut libc::c_char;
    #[no_mangle]
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn COM_StripExtension(path: *mut libc::c_char);
    #[no_mangle]
    fn S_StopAllSounds(ambient: qboolean);
    #[no_mangle]
    fn S_StopBackgroundTrack();
    #[no_mangle]
    fn Cvar_FullSet(var_name: *const libc::c_char, value: *const libc::c_char,
                    flags: libc::c_int);
    #[no_mangle]
    fn Cvar_Set(var_name: *const libc::c_char, value: *const libc::c_char);
    #[no_mangle]
    fn Cvar_SetValue(var_name: *const libc::c_char, value: libc::c_float);
    #[no_mangle]
    fn Cvar_VariableInteger(var_name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    static mut host_developer: convar_t;
    #[no_mangle]
    static mut host_maxfps: *mut convar_t;
    #[no_mangle]
    static mut host: host_parm_t;
    #[no_mangle]
    static mut SI: sysinfo_t;
    #[no_mangle]
    fn Cbuf_InsertText(text: *const libc::c_char);
    #[no_mangle]
    fn Cbuf_Execute();
    #[no_mangle]
    fn Cmd_Argc() -> libc::c_int;
    #[no_mangle]
    fn Cmd_Argv(arg: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn _Mem_Alloc(poolptr: poolhandle_t, size: size_t, clear: qboolean,
                  filename: *const libc::c_char, fileline: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    fn _Mem_Free(data: *mut libc::c_void, filename: *const libc::c_char,
                 fileline: libc::c_int);
    #[no_mangle]
    fn FS_Open(filepath: *const libc::c_char, mode: *const libc::c_char,
               gamedironly: qboolean) -> *mut file_t;
    #[no_mangle]
    fn FS_Write(file: *mut file_t, data: *const libc::c_void,
                datasize: size_t) -> fs_offset_t;
    #[no_mangle]
    fn FS_Read(file: *mut file_t, buffer: *mut libc::c_void,
               buffersize: size_t) -> fs_offset_t;
    #[no_mangle]
    fn FS_Seek(file: *mut file_t, offset: fs_offset_t, whence: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn FS_FileExists(filename: *const libc::c_char, gamedironly: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn FS_FileCopy(pOutput: *mut file_t, pInput: *mut file_t,
                   fileSize: libc::c_int) -> qboolean;
    #[no_mangle]
    fn FS_Delete(path: *const libc::c_char) -> qboolean;
    #[no_mangle]
    fn FS_Tell(file: *mut file_t) -> fs_offset_t;
    #[no_mangle]
    fn FS_Close(file: *mut file_t) -> libc::c_int;
    #[no_mangle]
    fn FS_Getc(file: *mut file_t) -> libc::c_int;
    #[no_mangle]
    fn Host_ShutdownServer();
    #[no_mangle]
    fn SV_Active() -> qboolean;
    #[no_mangle]
    fn Con_Reportf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_DPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    static mut gameui: gameui_static_t;
    #[no_mangle]
    fn CL_ClearState();
    #[no_mangle]
    static mut cls: client_static_t;
    #[no_mangle]
    fn AngleQuaternion(angles: *const vec_t, q: *mut vec_t, studio: qboolean);
    #[no_mangle]
    fn QuaternionAngle(q: *const vec_t, angles: *mut vec_t);
    #[no_mangle]
    fn QuaternionSlerp(p: *const vec_t, q: *const vec_t, t: libc::c_float,
                       qt: *mut vec_t);
    #[no_mangle]
    fn MSG_InitExt(sb: *mut sizebuf_t, pDebugName: *const libc::c_char,
                   pData: *mut libc::c_void, nBytes: libc::c_int,
                   nMaxBits: libc::c_int);
    #[no_mangle]
    static mut net_from: netadr_t;
    #[no_mangle]
    fn Netchan_Setup(sock: netsrc_t, chan: *mut netchan_t, adr: netadr_t,
                     qport: libc::c_int, client: *mut libc::c_void,
                     pfnBlockSize:
                         Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                     _: fragsize_t)
                                    -> libc::c_int>);
    #[no_mangle]
    static mut refState: ref_globals_t;
    #[no_mangle]
    static mut CL_UPDATE_BACKUP: libc::c_int;
    #[no_mangle]
    static mut cl: client_t;
    #[no_mangle]
    static mut clgame: clgame_static_t;
    #[no_mangle]
    fn CL_WriteUsercmd(msg: *mut sizebuf_t, from: libc::c_int,
                       to: libc::c_int);
    #[no_mangle]
    fn CL_GetFragmentSize(unused: *mut libc::c_void, mode: fragsize_t)
     -> libc::c_int;
    #[no_mangle]
    fn Con_DrawString(x: libc::c_int, y: libc::c_int,
                      string: *const libc::c_char, setColor: *mut byte)
     -> libc::c_int;
    #[no_mangle]
    fn Con_DrawStringLen(pText: *const libc::c_char, length: *mut libc::c_int,
                         height: *mut libc::c_int);
    #[no_mangle]
    fn Con_FastClose();
    #[no_mangle]
    fn CL_InitEdicts();
    #[no_mangle]
    fn SCR_StopCinematic();
    #[no_mangle]
    fn MSG_ReadDeltaUsercmd(msg: *mut sizebuf_t, from: *mut usercmd_s,
                            to: *mut usercmd_s);
}
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
#[repr(C, packed)]
pub struct demoheader_t {
    pub id: libc::c_int,
    pub dem_protocol: libc::c_int,
    pub net_protocol: libc::c_int,
    pub host_fps: libc::c_double,
    pub mapname: [libc::c_char; 64],
    pub comment: [libc::c_char; 64],
    pub gamedir: [libc::c_char; 64],
    pub directory_offset: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct demoentry_t {
    pub entrytype: libc::c_int,
    pub playback_time: libc::c_float,
    pub playback_frames: libc::c_int,
    pub offset: libc::c_int,
    pub length: libc::c_int,
    pub flags: libc::c_int,
    pub description: [libc::c_char; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct demodirectory_t {
    pub entries: *mut demoentry_t,
    pub numentries: libc::c_int,
}
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
// private demo states
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub header: demoheader_t,
    pub entry: *mut demoentry_t,
    pub directory: demodirectory_t,
    pub framecount: libc::c_int,
    pub starttime: libc::c_float,
    pub realstarttime: libc::c_float,
    pub timestamp: libc::c_float,
    pub lasttime: libc::c_float,
    pub entryIndex: libc::c_int,
    pub cmds: [demoangle_t; 16],
    pub angle_position: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct demoangle_t {
    pub starttime: libc::c_float,
    pub viewangles: vec3_t,
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
pub struct ref_viewpass_s {
    pub viewport: [libc::c_int; 4],
    pub vieworigin: vec3_t,
    pub viewangles: vec3_t,
    pub viewentity: libc::c_int,
    pub fov_x: libc::c_float,
    pub fov_y: libc::c_float,
    pub flags: libc::c_int,
}
pub type efrag_t = efrag_s;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const DEMO_QUAKE1: C2RustUnnamed_0 = 2;
pub const DEMO_XASH3D: C2RustUnnamed_0 = 1;
pub const DEMO_INACTIVE: C2RustUnnamed_0 = 0;
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
#[inline(always)]
unsafe extern "C" fn __tg_fmod(mut __x: libc::c_double,
                               mut __y: libc::c_double) -> libc::c_double {
    return fmod(__x, __y);
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
unsafe extern "C" fn MSG_GetData(mut sb: *mut sizebuf_t) -> *mut byte {
    return (*sb).pData;
}
#[no_mangle]
pub static mut demo_cmd: [*const libc::c_char; 7] =
    [b"dem_unknown\x00" as *const u8 as *const libc::c_char,
     b"dem_norewind\x00" as *const u8 as *const libc::c_char,
     b"dem_read\x00" as *const u8 as *const libc::c_char,
     b"dem_jumptime\x00" as *const u8 as *const libc::c_char,
     b"dem_userdata\x00" as *const u8 as *const libc::c_char,
     b"dem_usercmd\x00" as *const u8 as *const libc::c_char,
     b"dem_stop\x00" as *const u8 as *const libc::c_char];
#[no_mangle]
pub static mut demo: C2RustUnnamed =
    C2RustUnnamed{header:
                      demoheader_t{id: 0,
                                   dem_protocol: 0,
                                   net_protocol: 0,
                                   host_fps: 0.,
                                   mapname: [0; 64],
                                   comment: [0; 64],
                                   gamedir: [0; 64],
                                   directory_offset: 0,},
                  entry: 0 as *const demoentry_t as *mut demoentry_t,
                  directory:
                      demodirectory_t{entries:
                                          0 as *const demoentry_t as
                                              *mut demoentry_t,
                                      numentries: 0,},
                  framecount: 0,
                  starttime: 0.,
                  realstarttime: 0.,
                  timestamp: 0.,
                  lasttime: 0.,
                  entryIndex: 0,
                  cmds:
                      [demoangle_t{starttime: 0., viewangles: [0.; 3],}; 16],
                  angle_position: 0,};
/*
====================
CL_StartupDemoHeader

spooling demo header in case
we record a demo on this level
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_StartupDemoHeader() {
    if !cls.demoheader.is_null() { FS_Close(cls.demoheader); }
    // Note: this is replacing tmpfile()
    cls.demoheader =
        FS_Open(b"demoheader.tmp\x00" as *const u8 as *const libc::c_char,
                b"w+b\x00" as *const u8 as *const libc::c_char, true_0);
    if cls.demoheader.is_null() {
        Con_DPrintf(b"^1Error:^7 couldn\'t open temporary header file.\n\x00"
                        as *const u8 as *const libc::c_char);
        return
    }
    Con_Printf(b"Spooling demo header.\n\x00" as *const u8 as
                   *const libc::c_char);
}
/*
====================
CL_CloseDemoHeader

close demoheader file on engine shutdown
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_CloseDemoHeader() {
    if cls.demoheader.is_null() { return }
    FS_Close(cls.demoheader);
}
/*
====================
CL_GetDemoRecordClock

write time while demo is recording
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_GetDemoRecordClock() -> libc::c_float {
    return cl.mtime[0 as libc::c_int as usize] as libc::c_float;
}
/*
====================
CL_GetDemoPlaybackClock

overwrite host.realtime
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_GetDemoPlaybackClock() -> libc::c_float {
    return (host.realtime + host.frametime) as libc::c_float;
}
/*
====================
CL_GetDemoFramerate

overwrite host.frametime
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_GetDemoFramerate() -> libc::c_double {
    if cls.timedemo as u64 != 0 { return 0.0f64 }
    return if demo.header.host_fps >= 20.0f32 as libc::c_double {
               if demo.header.host_fps < 200.0f32 as libc::c_double {
                   demo.header.host_fps
               } else { 200.0f32 as libc::c_double }
           } else { 20.0f32 as libc::c_double };
}
/*
====================
CL_WriteDemoCmdHeader

Writes the demo command header and time-delta
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_WriteDemoCmdHeader(mut cmd: byte,
                                               mut file: *mut file_t) {
    let mut dt: libc::c_float = 0.;
    if file.is_null() { return }
    // command
    FS_Write(file, &mut cmd as *mut byte as *const libc::c_void,
             ::std::mem::size_of::<byte>() as libc::c_ulong);
    // time offset
    dt = CL_GetDemoRecordClock() - demo.starttime;
    FS_Write(file, &mut dt as *mut libc::c_float as *const libc::c_void,
             ::std::mem::size_of::<libc::c_float>() as libc::c_ulong);
}
/*
====================
CL_WriteDemoJumpTime

Update level time on a next level
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_WriteDemoJumpTime() {
    if cls.demowaiting as libc::c_uint != 0 || cls.demofile.is_null() {
        return
    } // setup the demo starttime
    demo.starttime = CL_GetDemoRecordClock();
    // demo playback should read this as an incoming message.
	// write the client's realtime value out so we can synchronize the reads.
    CL_WriteDemoCmdHeader(3 as libc::c_int as byte, cls.demofile);
}
/*
====================
CL_WriteDemoUserCmd

Writes the current user cmd
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_WriteDemoUserCmd(mut cmdnumber: libc::c_int) {
    let mut buf: sizebuf_t =
        sizebuf_t{bOverflow: false_0,
                  pDebugName: 0 as *const libc::c_char,
                  pData: 0 as *const byte as *mut byte,
                  iCurBit: 0,
                  nDataBits: 0,};
    let mut bytes: word = 0;
    let mut data: [byte; 1024] = [0; 1024];
    if cls.demorecording as u64 == 0 || cls.demofile.is_null() { return }
    CL_WriteDemoCmdHeader(5 as libc::c_int as byte, cls.demofile);
    FS_Write(cls.demofile,
             &mut cls.netchan.outgoing_sequence as *mut libc::c_uint as
                 *const libc::c_void,
             ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    FS_Write(cls.demofile,
             &mut cmdnumber as *mut libc::c_int as *const libc::c_void,
             ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    // write usercmd_t
    MSG_InitExt(&mut buf, b"UserCmd\x00" as *const u8 as *const libc::c_char,
                data.as_mut_ptr() as *mut libc::c_void,
                ::std::mem::size_of::<[byte; 1024]>() as libc::c_ulong as
                    libc::c_int, -(1 as libc::c_int)); // always no delta
    CL_WriteUsercmd(&mut buf, -(1 as libc::c_int), cmdnumber);
    bytes = MSG_GetNumBytesWritten(&mut buf) as word;
    FS_Write(cls.demofile, &mut bytes as *mut word as *const libc::c_void,
             ::std::mem::size_of::<word>() as libc::c_ulong);
    FS_Write(cls.demofile, data.as_mut_ptr() as *const libc::c_void,
             bytes as size_t);
}
/*
====================
CL_WriteDemoSequence

Save state of cls.netchan sequences
so that we can play the demo correctly.
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_WriteDemoSequence(mut file: *mut file_t) {
    FS_Write(file,
             &mut cls.netchan.incoming_sequence as *mut libc::c_uint as
                 *const libc::c_void,
             ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    FS_Write(file,
             &mut cls.netchan.incoming_acknowledged as *mut libc::c_uint as
                 *const libc::c_void,
             ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    FS_Write(file,
             &mut cls.netchan.incoming_reliable_acknowledged as
                 *mut libc::c_uint as *const libc::c_void,
             ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    FS_Write(file,
             &mut cls.netchan.incoming_reliable_sequence as *mut libc::c_uint
                 as *const libc::c_void,
             ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    FS_Write(file,
             &mut cls.netchan.outgoing_sequence as *mut libc::c_uint as
                 *const libc::c_void,
             ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    FS_Write(file,
             &mut cls.netchan.reliable_sequence as *mut libc::c_uint as
                 *const libc::c_void,
             ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    FS_Write(file,
             &mut cls.netchan.last_reliable_sequence as *mut libc::c_uint as
                 *const libc::c_void,
             ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
}
/*
====================
CL_WriteDemoMessage

Dumps the current net message, prefixed by the length
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_WriteDemoMessage(mut startup: qboolean,
                                             mut start: libc::c_int,
                                             mut msg: *mut sizebuf_t) {
    let mut file: *mut file_t =
        if startup as libc::c_uint != 0 {
            cls.demoheader
        } else { cls.demofile };
    let mut swlen: libc::c_int = 0;
    let mut c: byte = 0;
    if file.is_null() { return }
    // past the start but not recording a demo.
    if startup as u64 == 0 && cls.demorecording as u64 == 0 { return }
    swlen = MSG_GetNumBytesWritten(msg) - start;
    if swlen <= 0 as libc::c_int { return }
    if startup as u64 == 0 { demo.framecount += 1 }
    // demo playback should read this as an incoming message.
    c =
        if cls.state as libc::c_uint !=
               ca_active as libc::c_int as libc::c_uint {
            1 as libc::c_int
        } else { 2 as libc::c_int } as byte;
    CL_WriteDemoCmdHeader(c, file);
    CL_WriteDemoSequence(file);
    // write the length out.
    FS_Write(file, &mut swlen as *mut libc::c_int as *const libc::c_void,
             ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    // output the buffer. Skip the network packet stuff.
    FS_Write(file,
             MSG_GetData(msg).offset(start as isize) as *const libc::c_void,
             swlen as size_t);
}
/*
====================
CL_WriteDemoUserMessage

Dumps the user message (demoaction)
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_WriteDemoUserMessage(mut buffer: *const byte,
                                                 mut size: size_t) {
    if cls.demorecording as u64 == 0 || cls.demowaiting as libc::c_uint != 0 {
        return
    }
    if cls.demofile.is_null() || buffer.is_null() ||
           size <= 0 as libc::c_int as libc::c_ulong {
        return
    }
    CL_WriteDemoCmdHeader(4 as libc::c_int as byte, cls.demofile);
    // write the length out.
    FS_Write(cls.demofile, &mut size as *mut size_t as *const libc::c_void,
             ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    // output the buffer.
    FS_Write(cls.demofile, buffer as *const libc::c_void, size);
}
/*
====================
CL_WriteDemoHeader

Write demo header
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_WriteDemoHeader(mut name: *const libc::c_char) {
    let mut copysize: libc::c_int =
        0; // don't start saving messages until a non-delta compressed message is received
    let mut savepos: libc::c_int = 0;
    let mut curpos: libc::c_int = 0;
    Con_Printf(b"recording to %s.\n\x00" as *const u8 as *const libc::c_char,
               name);
    cls.demofile =
        FS_Open(name, b"wb\x00" as *const u8 as *const libc::c_char, false_0);
    cls.demotime = 0.0f64;
    if cls.demofile.is_null() {
        Con_Printf(b"^1Error:^7 couldn\'t open %s.\n\x00" as *const u8 as
                       *const libc::c_char, name);
        return
    }
    cls.demorecording = true_0;
    cls.demowaiting = true_0;
    memset(&mut demo.header as *mut demoheader_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<demoheader_t>() as libc::c_ulong);
    demo.header.id =
        (('M' as i32) << 24 as libc::c_int) +
            (('E' as i32) << 16 as libc::c_int) +
            (('D' as i32) << 8 as libc::c_int) + 'I' as i32;
    demo.header.dem_protocol = 3 as libc::c_int;
    demo.header.net_protocol =
        if cls.legacymode as libc::c_uint != 0 {
            48 as libc::c_int
        } else { 49 as libc::c_int };
    demo.header.host_fps =
        if (*host_maxfps).value >= 20.0f32 {
            if (*host_maxfps).value < 200.0f32 {
                (*host_maxfps).value
            } else { 200.0f32 }
        } else { 20.0f32 } as libc::c_double;
    Q_strncpy(demo.header.mapname.as_mut_ptr(), clgame.mapname.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    Q_strncpy(demo.header.comment.as_mut_ptr(), clgame.maptitle.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    Q_strncpy(demo.header.gamedir.as_mut_ptr(),
              (*SI.GameInfo).gamefolder.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    // write header
    FS_Write(cls.demofile,
             &mut demo.header as *mut demoheader_t as *const libc::c_void,
             ::std::mem::size_of::<demoheader_t>() as libc::c_ulong);
    demo.directory.numentries = 2 as libc::c_int;
    demo.directory.entries =
        _Mem_Alloc(cls.mempool,
                   (::std::mem::size_of::<demoentry_t>() as
                        libc::c_ulong).wrapping_mul(demo.directory.numentries
                                                        as libc::c_ulong),
                   true_0,
                   b"../engine/client/cl_demo.c\x00" as *const u8 as
                       *const libc::c_char, 384 as libc::c_int) as
            *mut demoentry_t;
    // DIRECTORY ENTRY # 0
    demo.entry =
        &mut *demo.directory.entries.offset(0 as libc::c_int as isize) as
            *mut demoentry_t; // only one here.
    (*demo.entry).entrytype = 0 as libc::c_int; // startup takes 0 time.
    (*demo.entry).playback_time = 0.0f32; // position for this chunk.
    (*demo.entry).offset = FS_Tell(cls.demofile) as libc::c_int;
    // finish off the startup info.
    CL_WriteDemoCmdHeader(6 as libc::c_int as byte, cls.demoheader);
    // now copy the stuff we cached from the server.
    savepos = FS_Tell(cls.demoheader) as libc::c_int;
    copysize = savepos;
    FS_Seek(cls.demoheader, 0 as libc::c_int as fs_offset_t,
            0 as libc::c_int);
    FS_FileCopy(cls.demofile, cls.demoheader, copysize);
    // jump back to end, in case we record another demo for this session.
    FS_Seek(cls.demoheader, savepos as fs_offset_t,
            0 as libc::c_int); // setup the demo starttime
    demo.starttime = CL_GetDemoRecordClock(); // get a new message this frame
    demo.realstarttime = demo.starttime;
    demo.framecount = 0 as libc::c_int;
    cls.td_startframe = host.framecount as libc::c_int;
    cls.td_lastframe = -(1 as libc::c_int);
    // now move on to entry # 1, the first data chunk.
    curpos = FS_Tell(cls.demofile) as libc::c_int;
    (*demo.entry).length = curpos - (*demo.entry).offset;
    // now we are writing the first real lump.
    demo.entry =
        &mut *demo.directory.entries.offset(1 as libc::c_int as isize) as
            *mut demoentry_t; // first real data lump
    (*demo.entry).entrytype = 1 as libc::c_int; // startup takes 0 time.
    (*demo.entry).playback_time = 0.0f32;
    (*demo.entry).offset = FS_Tell(cls.demofile) as libc::c_int;
    // demo playback should read this as an incoming message.
	// write the client's realtime value out so we can synchronize the reads.
    CL_WriteDemoCmdHeader(3 as libc::c_int as byte, cls.demofile);
    if !clgame.hInstance.is_null() {
        clgame.dllFuncs.pfnReset.expect("non-null function pointer")();
    }
    Cbuf_InsertText(b"fullupdate\n\x00" as *const u8 as *const libc::c_char);
    Cbuf_Execute();
}
/*
=================
CL_StopRecord

finish recording demo
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_StopRecord() {
    let mut i: libc::c_int = 0;
    let mut curpos: libc::c_int = 0;
    let mut stoptime: libc::c_float = 0.;
    let mut frames: libc::c_int = 0;
    if cls.demorecording as u64 == 0 { return }
    // demo playback should read this as an incoming message.
    CL_WriteDemoCmdHeader(6 as libc::c_int as byte, cls.demofile);
    stoptime = CL_GetDemoRecordClock();
    if !clgame.hInstance.is_null() {
        clgame.dllFuncs.pfnReset.expect("non-null function pointer")();
    }
    curpos = FS_Tell(cls.demofile) as libc::c_int;
    (*demo.entry).length = curpos - (*demo.entry).offset;
    (*demo.entry).playback_time = stoptime - demo.realstarttime;
    (*demo.entry).playback_frames = demo.framecount;
    //  Now write out the directory and free it and touch up the demo header.
    FS_Write(cls.demofile,
             &mut demo.directory.numentries as *mut libc::c_int as
                 *const libc::c_void,
             ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    i = 0 as libc::c_int;
    while i < demo.directory.numentries {
        FS_Write(cls.demofile,
                 &mut *demo.directory.entries.offset(i as isize) as
                     *mut demoentry_t as *const libc::c_void,
                 ::std::mem::size_of::<demoentry_t>() as libc::c_ulong);
        i += 1
    }
    _Mem_Free(demo.directory.entries as *mut libc::c_void,
              b"../engine/client/cl_demo.c\x00" as *const u8 as
                  *const libc::c_char, 464 as libc::c_int);
    demo.directory.numentries = 0 as libc::c_int;
    demo.header.directory_offset = curpos;
    FS_Seek(cls.demofile, 0 as libc::c_int as fs_offset_t, 0 as libc::c_int);
    FS_Write(cls.demofile,
             &mut demo.header as *mut demoheader_t as *const libc::c_void,
             ::std::mem::size_of::<demoheader_t>() as libc::c_ulong);
    FS_Close(cls.demofile);
    cls.demofile = 0 as *mut file_t;
    cls.demorecording = false_0;
    cls.demoname[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    cls.td_lastframe = host.framecount as libc::c_int;
    (*gameui.globals).demoname[0 as libc::c_int as usize] =
        '\u{0}' as i32 as libc::c_char;
    demo.header.host_fps = 0.0f64;
    frames = cls.td_lastframe - cls.td_startframe;
    Con_Printf(b"Completed demo\nRecording time: %02d:%02d, frames %i\n\x00"
                   as *const u8 as *const libc::c_char,
               (cls.demotime / 60.0f32 as libc::c_double) as libc::c_int,
               __tg_fmod(cls.demotime, 60.0f32 as libc::c_double) as
                   libc::c_int, frames);
    cls.demotime = 0.0f64;
}
/*
=================
CL_DrawDemoRecording
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_DrawDemoRecording() {
    let mut string: [libc::c_char; 64] = [0; 64];
    let mut color: rgba_t =
        [255 as libc::c_int as byte, 255 as libc::c_int as byte,
         255 as libc::c_int as byte, 255 as libc::c_int as byte];
    let mut pos: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    if !(host_developer.value != 0. && cls.demorecording as libc::c_uint != 0)
       {
        return
    }
    pos = FS_Tell(cls.demofile) as libc::c_int;
    Q_snprintf(string.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
               b"^1RECORDING:^7 %s: %s time: %02d:%02d\x00" as *const u8 as
                   *const libc::c_char, cls.demoname.as_mut_ptr(),
               Q_pretifymem(pos as libc::c_float, 2 as libc::c_int),
               (cls.demotime / 60.0f32 as libc::c_double) as libc::c_int,
               __tg_fmod(cls.demotime, 60.0f32 as libc::c_double) as
                   libc::c_int);
    Con_DrawStringLen(string.as_mut_ptr(), &mut len, 0 as *mut libc::c_int);
    Con_DrawString(refState.width - len >> 1 as libc::c_int,
                   refState.height >> 4 as libc::c_int, string.as_mut_ptr(),
                   color.as_mut_ptr());
}
/*
=======================================================================

CLIENT SIDE DEMO PLAYBACK

=======================================================================
*/
/*
=================
CL_ReadDemoCmdHeader

read the demo command
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ReadDemoCmdHeader(mut cmd: *mut byte,
                                              mut dt: *mut libc::c_float) {
    // read the command
    FS_Read(cls.demofile, cmd as *mut libc::c_void,
            ::std::mem::size_of::<byte>() as libc::c_ulong);
    // read the timestamp
    FS_Read(cls.demofile, dt as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_float>() as libc::c_ulong);
}
/*
=================
CL_ReadDemoUserCmd

read the demo usercmd for predicting
and smooth movement during playback the demo
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ReadDemoUserCmd(mut discard: qboolean) {
    let mut data: [byte; 1024] = [0; 1024];
    let mut cmdnumber: libc::c_int = 0;
    let mut outgoing_sequence: libc::c_int = 0;
    let mut pcmd: *mut runcmd_t = 0 as *mut runcmd_t;
    let mut bytes: word = 0;
    FS_Read(cls.demofile,
            &mut outgoing_sequence as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    FS_Read(cls.demofile,
            &mut cmdnumber as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    FS_Read(cls.demofile, &mut bytes as *mut word as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_short>() as libc::c_ulong);
    FS_Read(cls.demofile, data.as_mut_ptr() as *mut libc::c_void,
            bytes as size_t);
    if discard as u64 == 0 {
        let mut nullcmd: usercmd_t =
            usercmd_t{lerp_msec: 0,
                      msec: 0,
                      viewangles: [0.; 3],
                      forwardmove: 0.,
                      sidemove: 0.,
                      upmove: 0.,
                      lightlevel: 0,
                      buttons: 0,
                      impulse: 0,
                      weaponselect: 0,
                      impact_index: 0,
                      impact_position: [0.; 3],};
        let mut buf: sizebuf_t =
            sizebuf_t{bOverflow: false_0,
                      pDebugName: 0 as *const libc::c_char,
                      pData: 0 as *const byte as *mut byte,
                      iCurBit: 0,
                      nDataBits: 0,};
        let mut a: *mut demoangle_t = 0 as *mut demoangle_t;
        memset(&mut nullcmd as *mut usercmd_t as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<usercmd_t>() as libc::c_ulong);
        MSG_InitExt(&mut buf,
                    b"UserCmd\x00" as *const u8 as *const libc::c_char,
                    data.as_mut_ptr() as *mut libc::c_void,
                    ::std::mem::size_of::<[byte; 1024]>() as libc::c_ulong as
                        libc::c_int, -(1 as libc::c_int));
        pcmd =
            &mut *cl.commands.as_mut_ptr().offset((cmdnumber &
                                                       CL_UPDATE_BACKUP -
                                                           1 as libc::c_int)
                                                      as isize) as
                *mut runcmd_t;
        (*pcmd).processedfuncs = false_0;
        (*pcmd).senttime = 0.0f32 as libc::c_double;
        (*pcmd).receivedtime = 0.1f32 as libc::c_double;
        (*pcmd).frame_lerp = 0.1f32;
        (*pcmd).heldback = false_0;
        (*pcmd).sendsize = 1 as libc::c_int;
        // always delta'ing from null
        cl.cmd = &mut (*pcmd).cmd;
        MSG_ReadDeltaUsercmd(&mut buf, &mut nullcmd, cl.cmd);
        // make sure what interp info contain angles from different frames
		// or lerping will stop working
        if demo.lasttime != demo.timestamp {
            // select entry into circular buffer
            demo.angle_position =
                demo.angle_position + 1 as libc::c_int &
                    16 as libc::c_int - 1 as libc::c_int;
            a =
                &mut *demo.cmds.as_mut_ptr().offset(demo.angle_position as
                                                        isize) as
                    *mut demoangle_t;
            // record update
            (*a).starttime = demo.timestamp;
            (*a).viewangles[0 as libc::c_int as usize] =
                (*cl.cmd).viewangles[0 as libc::c_int as usize];
            (*a).viewangles[1 as libc::c_int as usize] =
                (*cl.cmd).viewangles[1 as libc::c_int as usize];
            (*a).viewangles[2 as libc::c_int as usize] =
                (*cl.cmd).viewangles[2 as libc::c_int as usize];
            demo.lasttime = demo.timestamp
        }
        // NOTE: we need to have the current outgoing sequence correct
		// so we can do prediction correctly during playback
        cls.netchan.outgoing_sequence = outgoing_sequence as libc::c_uint
    };
}
/*
=================
CL_ReadDemoSequence

read netchan sequences
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ReadDemoSequence(mut discard: qboolean) {
    let mut incoming_sequence: libc::c_int = 0;
    let mut incoming_acknowledged: libc::c_int = 0;
    let mut incoming_reliable_acknowledged: libc::c_int = 0;
    let mut incoming_reliable_sequence: libc::c_int = 0;
    let mut outgoing_sequence: libc::c_int = 0;
    let mut reliable_sequence: libc::c_int = 0;
    let mut last_reliable_sequence: libc::c_int = 0;
    FS_Read(cls.demofile,
            &mut incoming_sequence as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    FS_Read(cls.demofile,
            &mut incoming_acknowledged as *mut libc::c_int as
                *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    FS_Read(cls.demofile,
            &mut incoming_reliable_acknowledged as *mut libc::c_int as
                *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    FS_Read(cls.demofile,
            &mut incoming_reliable_sequence as *mut libc::c_int as
                *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    FS_Read(cls.demofile,
            &mut outgoing_sequence as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    FS_Read(cls.demofile,
            &mut reliable_sequence as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    FS_Read(cls.demofile,
            &mut last_reliable_sequence as *mut libc::c_int as
                *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    if discard as u64 != 0 { return }
    cls.netchan.incoming_sequence = incoming_sequence as libc::c_uint;
    cls.netchan.incoming_acknowledged = incoming_acknowledged as libc::c_uint;
    cls.netchan.incoming_reliable_acknowledged =
        incoming_reliable_acknowledged as libc::c_uint;
    cls.netchan.incoming_reliable_sequence =
        incoming_reliable_sequence as libc::c_uint;
    cls.netchan.outgoing_sequence = outgoing_sequence as libc::c_uint;
    cls.netchan.reliable_sequence = reliable_sequence as libc::c_uint;
    cls.netchan.last_reliable_sequence =
        last_reliable_sequence as libc::c_uint;
}
/*
=================
CL_DemoStartPlayback
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_DemoStartPlayback(mut mode: libc::c_int) {
    if cls.changedemo as u64 != 0 {
        S_StopAllSounds(true_0);
        SCR_BeginLoadingPlaque(false_0);
        CL_ClearState();
        CL_InitEdicts();
        // re-arrange edicts
    } else {
        // NOTE: at this point demo is still valid
        CL_Disconnect(); // for determining whether to read another message
        Host_ShutdownServer();
        Con_FastClose();
        UI_SetActiveMenu(false_0);
    }
    cls.demoplayback = mode;
    cls.state = ca_connected;
    cl.background =
        if cls.demonum != -(1 as libc::c_int) {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int } as qboolean;
    cls.spectator = false_0;
    cls.signon = 0 as libc::c_int;
    demo.starttime = CL_GetDemoPlaybackClock();
    Netchan_Setup(NS_CLIENT, &mut cls.netchan, net_from,
                  Cvar_VariableInteger(b"net_qport\x00" as *const u8 as
                                           *const libc::c_char),
                  0 as *mut libc::c_void,
                  Some(CL_GetFragmentSize as
                           unsafe extern "C" fn(_: *mut libc::c_void,
                                                _: fragsize_t)
                               -> libc::c_int));
    memset(demo.cmds.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[demoangle_t; 16]>() as libc::c_ulong);
    demo.angle_position = 1 as libc::c_int;
    demo.framecount = 0 as libc::c_int;
    cls.lastoutgoingcommand = -(1 as libc::c_int);
    cls.nextcmdtime = host.realtime as libc::c_float;
    cl.last_command_ack = -(1 as libc::c_int);
}
/*
=================
CL_DemoAborted
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_DemoAborted() {
    if !cls.demofile.is_null() { FS_Close(cls.demofile); }
    cls.demoplayback = false_0 as libc::c_int;
    cls.changedemo = false_0;
    cls.timedemo = false_0;
    demo.framecount = 0 as libc::c_int;
    cls.demofile = 0 as *mut file_t;
    cls.demonum = -(1 as libc::c_int);
    Cvar_SetValue(b"v_dark\x00" as *const u8 as *const libc::c_char, 0.0f32);
}
/*
=================
CL_DemoCompleted
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_DemoCompleted() {
    if cls.demonum != -(1 as libc::c_int) { cls.changedemo = true_0 }
    CL_StopPlayback();
    if CL_NextDemo() as u64 == 0 && cls.changedemo as u64 == 0 {
        UI_SetActiveMenu(true_0);
    }
    Cvar_SetValue(b"v_dark\x00" as *const u8 as *const libc::c_char, 0.0f32);
}
/*
=================
CL_DemoMoveToNextSection

returns true on success, false on failure
g-cont. probably captain obvious mode is ON
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_DemoMoveToNextSection() -> qboolean {
    demo.entryIndex += 1;
    if demo.entryIndex >= demo.directory.numentries {
        // done
        CL_DemoCompleted();
        return false_0
    }
    // switch to next section, we got a dem_stop
    demo.entry =
        &mut *demo.directory.entries.offset(demo.entryIndex as isize) as
            *mut demoentry_t;
    // ready to continue reading, reset clock.
    FS_Seek(cls.demofile, (*demo.entry).offset as fs_offset_t,
            0 as libc::c_int);
    // time is now relative to this chunk's clock.
    demo.starttime = CL_GetDemoPlaybackClock(); // assume we fail
    demo.framecount = 0 as libc::c_int;
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn CL_ReadRawNetworkData(mut buffer: *mut byte,
                                               mut length: *mut size_t)
 -> qboolean {
    let mut msglen: libc::c_int = 0 as libc::c_int;
    *length = 0 as libc::c_int as size_t;
    FS_Read(cls.demofile,
            &mut msglen as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    if msglen < 0 as libc::c_int {
        Con_Reportf(b"^1Error:^7 Demo message length < 0\n\x00" as *const u8
                        as *const libc::c_char);
        CL_DemoCompleted();
        return false_0
    }
    if msglen > 0x20000 as libc::c_int {
        Con_Reportf(b"^1Error:^7 Demo message %i > %i\n\x00" as *const u8 as
                        *const libc::c_char, msglen, 0x20000 as libc::c_int);
        CL_DemoCompleted();
        return false_0
    }
    if msglen > 0 as libc::c_int {
        if FS_Read(cls.demofile, buffer as *mut libc::c_void,
                   msglen as size_t) != msglen as libc::c_long {
            Con_Reportf(b"^1Error:^7 Error reading demo message data\n\x00" as
                            *const u8 as *const libc::c_char);
            CL_DemoCompleted();
            return false_0
        }
    }
    cls.netchan.last_received = host.realtime;
    cls.netchan.total_received =
        (cls.netchan.total_received as
             libc::c_ulong).wrapping_add(msglen as libc::c_ulong) as size_t as
            size_t;
    *length = msglen as size_t;
    if cls.state as libc::c_uint != ca_active as libc::c_int as libc::c_uint {
        Cbuf_Execute();
    }
    return true_0;
}
/*
=================
CL_DemoReadMessageQuake

reads demo data and write it to client
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_DemoReadMessageQuake(mut buffer: *mut byte,
                                                 mut length: *mut size_t)
 -> qboolean {
    let mut viewangles: vec3_t = [0.; 3]; // assume we fail
    let mut msglen: libc::c_int = 0 as libc::c_int;
    let mut a: *mut demoangle_t = 0 as *mut demoangle_t;
    *length = 0 as libc::c_int as size_t;
    // decide if it is time to grab the next message
    if cls.signon == 2 as libc::c_int {
        // allways grab until fully connected
        if cls.timedemo as u64 != 0 {
            if host.framecount == cls.td_lastframe as libc::c_uint {
                return false_0
            } // already read this frame's message
            cls.td_lastframe = host.framecount as libc::c_int;
            // if this is the second frame, grab the real td_starttime
			// so the bogus time on the first frame doesn't count
            if host.framecount ==
                   (cls.td_startframe + 1 as libc::c_int) as libc::c_uint {
                cls.td_starttime = host.realtime
            }
        } else if cl.time <= cl.mtime[0 as libc::c_int as usize] {
            // don't need another message yet
            return false_0
        }
    }
    // get the next message
    FS_Read(cls.demofile,
            &mut msglen as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    FS_Read(cls.demofile,
            &mut *viewangles.as_mut_ptr().offset(0 as libc::c_int as isize) as
                *mut vec_t as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_float>() as libc::c_ulong);
    FS_Read(cls.demofile,
            &mut *viewangles.as_mut_ptr().offset(1 as libc::c_int as isize) as
                *mut vec_t as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_float>() as libc::c_ulong);
    FS_Read(cls.demofile,
            &mut *viewangles.as_mut_ptr().offset(2 as libc::c_int as isize) as
                *mut vec_t as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_float>() as libc::c_ulong);
    cls.netchan.incoming_sequence =
        cls.netchan.incoming_sequence.wrapping_add(1);
    demo.timestamp = cl.mtime[0 as libc::c_int as usize] as libc::c_float;
    cl.skip_interp = false_0;
    // make sure what interp info contain angles from different frames
	// or lerping will stop working
    if demo.lasttime != demo.timestamp {
        // select entry into circular buffer
        demo.angle_position =
            demo.angle_position + 1 as libc::c_int &
                16 as libc::c_int - 1 as libc::c_int;
        a =
            &mut *demo.cmds.as_mut_ptr().offset(demo.angle_position as isize)
                as *mut demoangle_t;
        // record update
        (*a).starttime = demo.timestamp;
        (*a).viewangles[0 as libc::c_int as usize] =
            viewangles[0 as libc::c_int as usize];
        (*a).viewangles[1 as libc::c_int as usize] =
            viewangles[1 as libc::c_int as usize];
        (*a).viewangles[2 as libc::c_int as usize] =
            viewangles[2 as libc::c_int as usize];
        demo.lasttime = demo.timestamp
    }
    if msglen < 0 as libc::c_int {
        Con_Reportf(b"^1Error:^7 Demo message length < 0\n\x00" as *const u8
                        as *const libc::c_char);
        CL_DemoCompleted();
        return false_0
    }
    if msglen > 0x20000 as libc::c_int {
        Con_Reportf(b"^1Error:^7 Demo message %i > %i\n\x00" as *const u8 as
                        *const libc::c_char, msglen, 0x20000 as libc::c_int);
        CL_DemoCompleted();
        return false_0
    }
    if msglen > 0 as libc::c_int {
        if FS_Read(cls.demofile, buffer as *mut libc::c_void,
                   msglen as size_t) != msglen as libc::c_long {
            Con_Reportf(b"^1Error:^7 Error reading demo message data\n\x00" as
                            *const u8 as *const libc::c_char);
            CL_DemoCompleted();
            return false_0
        }
    }
    cls.netchan.last_received = host.realtime;
    cls.netchan.total_received =
        (cls.netchan.total_received as
             libc::c_ulong).wrapping_add(msglen as libc::c_ulong) as size_t as
            size_t;
    *length = msglen as size_t;
    if cls.state as libc::c_uint != ca_active as libc::c_int as libc::c_uint {
        Cbuf_Execute();
    }
    return true_0;
}
/*
=================
CL_DemoReadMessage

reads demo data and write it to client
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_DemoReadMessage(mut buffer: *mut byte,
                                            mut length: *mut size_t)
 -> qboolean {
    let mut curpos: size_t = 0 as libc::c_int as size_t;
    let mut lastpos: size_t = 0 as libc::c_int as size_t;
    let mut fElapsedTime: libc::c_float = 0.0f32;
    let mut swallowmessages: qboolean = true_0;
    static mut tdlastdemoframe: libc::c_int = 0 as libc::c_int;
    let mut userbuf: *mut byte = 0 as *mut byte;
    let mut size: size_t = 0;
    let mut cmd: byte = 0;
    if cls.demofile.is_null() { CL_DemoCompleted(); return false_0 }
    if cl.background as u64 == 0 &&
           (cl.paused as libc::c_uint != 0 ||
                cls.key_dest as libc::c_uint !=
                    key_game as libc::c_int as libc::c_uint) ||
           cls.key_dest as libc::c_uint ==
               key_console as libc::c_int as libc::c_uint {
        demo.starttime =
            (demo.starttime as libc::c_double + host.frametime) as
                libc::c_float;
        return false_0
        // paused
    }
    if cls.demoplayback == DEMO_QUAKE1 as libc::c_int {
        return CL_DemoReadMessageQuake(buffer, length)
    }
    loop  {
        let mut bSkipMessage: qboolean = false_0;
        if cls.demofile.is_null() { break ; }
        curpos = FS_Tell(cls.demofile) as size_t;
        CL_ReadDemoCmdHeader(&mut cmd, &mut demo.timestamp);
        fElapsedTime = CL_GetDemoPlaybackClock() - demo.starttime;
        if cls.timedemo as u64 == 0 {
            bSkipMessage =
                if demo.timestamp as libc::c_double -
                       (cl.mtime[0 as libc::c_int as usize] -
                            cl.mtime[1 as libc::c_int as usize]) >=
                       fElapsedTime as libc::c_double {
                    true_0 as libc::c_int
                } else { false_0 as libc::c_int } as qboolean
        }
        if cls.changelevel as u64 != 0 { demo.framecount = 1 as libc::c_int }
        // changelevel issues
        if demo.framecount <= 2 as libc::c_int &&
               (fElapsedTime - demo.timestamp) as libc::c_double >
                   host.frametime {
            demo.starttime = CL_GetDemoPlaybackClock()
        }
        // not ready for a message yet, put it back on the file.
        if cmd as libc::c_int != 1 as libc::c_int &&
               cmd as libc::c_int != 6 as libc::c_int &&
               bSkipMessage as libc::c_uint != 0 {
            // never skip first message
            if demo.framecount != 0 as libc::c_int {
                FS_Seek(cls.demofile, curpos as fs_offset_t,
                        0 as libc::c_int);
                return false_0
                // not time yet.
            }
        }
        // we already have the usercmd_t for this frame
		// don't read next usercmd_t so predicting will work properly
        if cmd as libc::c_int == 5 as libc::c_int &&
               lastpos != 0 as libc::c_int as libc::c_ulong &&
               demo.framecount != 0 as libc::c_int {
            FS_Seek(cls.demofile, lastpos as fs_offset_t, 0 as libc::c_int);
            return false_0
            // not time yet.
        }
        // COMMAND HANDLERS
        match cmd as libc::c_int {
            3 => {
                demo.starttime =
                    CL_GetDemoPlaybackClock(); // time is changed, skip frame
                return false_0
            }
            6 => {
                CL_DemoMoveToNextSection(); // header is ended, skip frame
                return false_0
            }
            4 => {
                FS_Read(cls.demofile,
                        &mut size as *mut size_t as *mut libc::c_void,
                        ::std::mem::size_of::<libc::c_int>() as
                            libc::c_ulong);
                userbuf =
                    _Mem_Alloc(cls.mempool, size, false_0,
                               b"../engine/client/cl_demo.c\x00" as *const u8
                                   as *const libc::c_char, 956 as libc::c_int)
                        as *mut byte;
                FS_Read(cls.demofile, userbuf as *mut libc::c_void, size);
                if !clgame.hInstance.is_null() {
                    clgame.dllFuncs.pfnDemo_ReadBuffer.expect("non-null function pointer")(size
                                                                                               as
                                                                                               libc::c_int,
                                                                                           userbuf);
                }
                _Mem_Free(userbuf as *mut libc::c_void,
                          b"../engine/client/cl_demo.c\x00" as *const u8 as
                              *const libc::c_char, 961 as libc::c_int);
                userbuf = 0 as *mut byte
            }
            5 => {
                CL_ReadDemoUserCmd(false_0);
                lastpos = FS_Tell(cls.demofile) as size_t
            }
            _ => { swallowmessages = false_0 }
        }
        if !(swallowmessages as u64 != 0) { break ; }
    }
    // If we are playing back a timedemo, and we've already passed on a
	//  frame update for this host_frame tag, then we'll just skip this message.
    if cls.timedemo as libc::c_uint != 0 &&
           tdlastdemoframe as libc::c_uint == host.framecount {
        FS_Seek(cls.demofile,
                FS_Tell(cls.demofile) - 5 as libc::c_int as libc::c_long,
                0 as libc::c_int);
        return false_0
    }
    tdlastdemoframe = host.framecount as libc::c_int;
    if cls.demofile.is_null() { return false_0 }
    // if not on "LOADING" section, check a few things
    if demo.entryIndex != 0 {
        // We are now on the second frame of a new section,
		// if so, reset start time (unless in a timedemo)
        if demo.framecount == 1 as libc::c_int && cls.timedemo as u64 == 0 {
            // cheat by moving the relative start time forward.
            demo.starttime = CL_GetDemoPlaybackClock()
        }
    }
    demo.framecount += 1;
    CL_ReadDemoSequence(false_0);
    return CL_ReadRawNetworkData(buffer, length);
}
#[no_mangle]
pub unsafe extern "C" fn CL_DemoFindInterpolatedViewAngles(mut t:
                                                               libc::c_float,
                                                           mut frac:
                                                               *mut libc::c_float,
                                                           mut prev:
                                                               *mut *mut demoangle_t,
                                                           mut next:
                                                               *mut *mut demoangle_t) {
    let mut i: libc::c_int = 0;
    let mut i0: libc::c_int = 0;
    let mut i1: libc::c_int = 0;
    let mut imod: libc::c_int = 0;
    let mut at: libc::c_float = 0.;
    if cls.timedemo as u64 != 0 { return }
    imod = demo.angle_position - 1 as libc::c_int;
    i0 = imod + 1 as libc::c_int & 16 as libc::c_int - 1 as libc::c_int;
    i1 = imod + 0 as libc::c_int & 16 as libc::c_int - 1 as libc::c_int;
    if demo.cmds[i0 as usize].starttime >= t {
        i = 0 as libc::c_int;
        while i < 16 as libc::c_int - 2 as libc::c_int {
            at =
                demo.cmds[(imod & 16 as libc::c_int - 1 as libc::c_int) as
                              usize].starttime;
            if at == 0.0f32 { break ; }
            if at < t {
                i0 =
                    imod + 1 as libc::c_int &
                        16 as libc::c_int - 1 as libc::c_int;
                i1 =
                    imod + 0 as libc::c_int &
                        16 as libc::c_int - 1 as libc::c_int;
                break ;
            } else { imod -= 1; i += 1 }
        }
    }
    *next =
        &mut *demo.cmds.as_mut_ptr().offset(i0 as isize) as *mut demoangle_t;
    *prev =
        &mut *demo.cmds.as_mut_ptr().offset(i1 as isize) as *mut demoangle_t;
    // avoid division by zero (probably this should never happens)
    if (**prev).starttime == (**next).starttime {
        *prev = *next;
        *frac = 0.0f32;
        return
    }
    // time spans the two entries
    *frac =
        (t - (**prev).starttime) / ((**next).starttime - (**prev).starttime);
    *frac =
        if *frac >= 0.0f32 {
            if *frac < 1.0f32 { *frac } else { 1.0f32 }
        } else { 0.0f32 };
}
/*
==============
CL_DemoInterpolateAngles

We can predict or inpolate player movement with standed client code
but viewangles interpolate here
==============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_DemoInterpolateAngles() {
    let mut prev: *mut demoangle_t = 0 as *mut demoangle_t;
    let mut next: *mut demoangle_t = 0 as *mut demoangle_t;
    let mut frac: libc::c_float = 0.0f32;
    let mut curtime: libc::c_float = 0.;
    if cls.demoplayback == DEMO_QUAKE1 as libc::c_int {
        // manually select next & prev states
        next =
            &mut *demo.cmds.as_mut_ptr().offset((demo.angle_position -
                                                     0 as libc::c_int &
                                                     16 as libc::c_int -
                                                         1 as libc::c_int) as
                                                    isize) as
                *mut demoangle_t; // camera was teleported
        prev =
            &mut *demo.cmds.as_mut_ptr().offset((demo.angle_position -
                                                     1 as libc::c_int &
                                                     16 as libc::c_int -
                                                         1 as libc::c_int) as
                                                    isize) as
                *mut demoangle_t; // don't run too far
        if cl.skip_interp as u64 != 0 { *prev = *next }
        frac = cl.lerpFrac
    } else {
        curtime =
            ((CL_GetDemoPlaybackClock() - demo.starttime) as libc::c_double -
                 host.frametime) as libc::c_float;
        if curtime > demo.timestamp { curtime = demo.timestamp }
        CL_DemoFindInterpolatedViewAngles(curtime, &mut frac, &mut prev,
                                          &mut next);
    }
    if !prev.is_null() && !next.is_null() {
        let mut q: vec4_t = [0.; 4];
        let mut q1: vec4_t = [0.; 4];
        let mut q2: vec4_t = [0.; 4];
        AngleQuaternion((*next).viewangles.as_mut_ptr() as *const vec_t,
                        q1.as_mut_ptr(), false_0);
        AngleQuaternion((*prev).viewangles.as_mut_ptr() as *const vec_t,
                        q2.as_mut_ptr(), false_0);
        QuaternionSlerp(q2.as_mut_ptr() as *const vec_t,
                        q1.as_mut_ptr() as *const vec_t, frac,
                        q.as_mut_ptr());
        QuaternionAngle(q.as_mut_ptr() as *const vec_t,
                        cl.viewangles.as_mut_ptr());
    } else if !cl.cmd.is_null() {
        cl.viewangles[0 as libc::c_int as usize] =
            (*cl.cmd).viewangles[0 as libc::c_int as usize];
        cl.viewangles[1 as libc::c_int as usize] =
            (*cl.cmd).viewangles[1 as libc::c_int as usize];
        cl.viewangles[2 as libc::c_int as usize] =
            (*cl.cmd).viewangles[2 as libc::c_int as usize]
    };
}
/*
==============
CL_FinishTimeDemo

show stats
==============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_FinishTimeDemo() {
    let mut frames: libc::c_int = 0;
    let mut time: libc::c_double = 0.;
    cls.timedemo = false_0;
    // the first frame didn't count
    frames =
        host.framecount.wrapping_sub(cls.td_startframe as
                                         libc::c_uint).wrapping_sub(1 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint)
            as libc::c_int;
    time = host.realtime - cls.td_starttime;
    if time == 0. { time = 1.0f64 }
    Con_Printf(b"%i frames %5.3f seconds %5.3f fps\n\x00" as *const u8 as
                   *const libc::c_char, frames, time,
               frames as libc::c_double / time);
}
/*
==============
CL_StopPlayback

Called when a demo file runs out, or the user starts a game
==============
*/
#[no_mangle]
pub unsafe extern "C" fn CL_StopPlayback() {
    if cls.demoplayback == 0 { return }
    // release demofile
    FS_Close(cls.demofile); // clear demoname too
    cls.demoplayback = false_0 as libc::c_int;
    demo.framecount = 0 as libc::c_int;
    cls.demofile = 0 as *mut file_t;
    cls.olddemonum =
        if -(1 as libc::c_int) > cls.demonum - 1 as libc::c_int {
            -(1 as libc::c_int)
        } else { (cls.demonum) - 1 as libc::c_int };
    if !demo.directory.entries.is_null() {
        _Mem_Free(demo.directory.entries as *mut libc::c_void,
                  b"../engine/client/cl_demo.c\x00" as *const u8 as
                      *const libc::c_char, 1134 as libc::c_int);
    }
    cls.td_lastframe = host.framecount as libc::c_int;
    demo.directory.numentries = 0 as libc::c_int;
    demo.directory.entries = 0 as *mut demoentry_t;
    demo.header.host_fps = 0.0f64;
    demo.entry = 0 as *mut demoentry_t;
    cls.demoname[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    (*gameui.globals).demoname[0 as libc::c_int as usize] =
        '\u{0}' as i32 as libc::c_char;
    if cls.timedemo as u64 != 0 { CL_FinishTimeDemo(); }
    if cls.changedemo as u64 != 0 {
        S_StopAllSounds(true_0);
        S_StopBackgroundTrack();
    } else {
        // let game known about demo state
        Cvar_FullSet(b"cl_background\x00" as *const u8 as *const libc::c_char,
                     b"0\x00" as *const u8 as *const libc::c_char,
                     (1 as libc::c_int) << 17 as libc::c_int);
        cls.state = ca_disconnected;
        memset(&mut cls.serveradr as *mut netadr_t as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<netadr_t>() as libc::c_ulong);
        cls.set_lastdemo = false_0;
        S_StopBackgroundTrack();
        cls.connect_time = 0 as libc::c_int as libc::c_double;
        cls.demonum = -(1 as libc::c_int);
        cls.signon = 0 as libc::c_int;
        // and finally clear the state
        CL_ClearState();
    };
}
/*
==================
CL_GetDemoComment
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_GetDemoComment(mut demoname: *const libc::c_char,
                                           mut comment: *mut libc::c_char)
 -> libc::c_int {
    let mut demfile: *mut file_t = 0 as *mut file_t;
    let mut demohdr: demoheader_t =
        demoheader_t{id: 0,
                     dem_protocol: 0,
                     net_protocol: 0,
                     host_fps: 0.,
                     mapname: [0; 64],
                     comment: [0; 64],
                     gamedir: [0; 64],
                     directory_offset: 0,};
    let mut directory: demodirectory_t =
        demodirectory_t{entries: 0 as *const demoentry_t as *mut demoentry_t,
                        numentries: 0,};
    let mut entry: demoentry_t =
        demoentry_t{entrytype: 0,
                    playback_time: 0.,
                    playback_frames: 0,
                    offset: 0,
                    length: 0,
                    flags: 0,
                    description: [0; 64],};
    let mut playtime: libc::c_float = 0.0f32;
    let mut i: libc::c_int = 0;
    if comment.is_null() { return false_0 as libc::c_int }
    demfile =
        FS_Open(demoname, b"rb\x00" as *const u8 as *const libc::c_char,
                false_0);
    if demfile.is_null() {
        *comment.offset(0 as libc::c_int as isize) =
            '\u{0}' as i32 as libc::c_char;
        return false_0 as libc::c_int
    }
    // read in the m_DemoHeader
    FS_Read(demfile, &mut demohdr as *mut demoheader_t as *mut libc::c_void,
            ::std::mem::size_of::<demoheader_t>() as libc::c_ulong);
    if demohdr.id !=
           (('M' as i32) << 24 as libc::c_int) +
               (('E' as i32) << 16 as libc::c_int) +
               (('D' as i32) << 8 as libc::c_int) + 'I' as i32 {
        FS_Close(demfile);
        Q_strncpy(comment,
                  b"<corrupted>\x00" as *const u8 as *const libc::c_char,
                  256 as libc::c_int as size_t);
        return false_0 as libc::c_int
    }
    if demohdr.net_protocol != 49 as libc::c_int &&
           demohdr.net_protocol != 48 as libc::c_int ||
           demohdr.dem_protocol != 3 as libc::c_int {
        FS_Close(demfile);
        Q_strncpy(comment,
                  b"<invalid protocol>\x00" as *const u8 as
                      *const libc::c_char, 256 as libc::c_int as size_t);
        return false_0 as libc::c_int
    }
    // now read in the directory structure.
    FS_Seek(demfile, demohdr.directory_offset as fs_offset_t,
            0 as libc::c_int);
    FS_Read(demfile,
            &mut directory.numentries as *mut libc::c_int as
                *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    if directory.numentries < 1 as libc::c_int ||
           directory.numentries > 1024 as libc::c_int {
        FS_Close(demfile);
        Q_strncpy(comment,
                  b"<corrupted>\x00" as *const u8 as *const libc::c_char,
                  256 as libc::c_int as size_t);
        return false_0 as libc::c_int
    }
    i = 0 as libc::c_int;
    while i < directory.numentries {
        FS_Read(demfile, &mut entry as *mut demoentry_t as *mut libc::c_void,
                ::std::mem::size_of::<demoentry_t>() as libc::c_ulong);
        playtime += entry.playback_time;
        i += 1
    }
    // split comment to sections
    Q_strncpy(comment, demohdr.mapname.as_mut_ptr(),
              64 as libc::c_int as size_t);
    Q_strncpy(comment.offset(64 as libc::c_int as isize),
              demohdr.comment.as_mut_ptr(), 64 as libc::c_int as size_t);
    Q_snprintf(comment.offset((64 as libc::c_int * 2 as libc::c_int) as
                                  isize), 16 as libc::c_int as size_t,
               b"%g sec\x00" as *const u8 as *const libc::c_char,
               playtime as libc::c_double);
    // all done
    FS_Close(demfile);
    return true_0 as libc::c_int;
}
/*
==================
CL_NextDemo

Called when a demo finishes
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_NextDemo() -> qboolean {
    let mut str: [libc::c_char; 64] = [0; 64]; // don't play demos
    if cls.demonum == -(1 as libc::c_int) { return false_0 }
    S_StopAllSounds(true_0);
    if cls.demos[cls.demonum as usize][0 as libc::c_int as usize] == 0 ||
           cls.demonum == 32 as libc::c_int {
        cls.demonum = 0 as libc::c_int;
        if cls.demos[cls.demonum as usize][0 as libc::c_int as usize] == 0 {
            Con_Printf(b"no demos listed with startdemos\n\x00" as *const u8
                           as *const libc::c_char);
            cls.demonum = -(1 as libc::c_int);
            return false_0
        }
    }
    Q_snprintf(str.as_mut_ptr(), 256 as libc::c_int as size_t,
               b"playdemo %s\n\x00" as *const u8 as *const libc::c_char,
               cls.demos[cls.demonum as usize].as_mut_ptr());
    Cbuf_InsertText(str.as_mut_ptr());
    cls.demonum += 1;
    return true_0;
}
/*
==================
CL_CheckStartupDemos

queue demos loop after movie playing
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_CheckStartupDemos() {
    if cls.demos_pending as u64 == 0 { return } // no demos in loop
    if cls.movienum != -(1 as libc::c_int) {
        return
    } // wait until movies finished
    if host.game.nextstate as libc::c_uint !=
           STATE_RUNFRAME as libc::c_int as libc::c_uint ||
           cls.demoplayback != 0 {
        // commandline override
        cls.demos_pending = false_0;
        cls.demonum = -(1 as libc::c_int);
        return
    }
    // run demos loop in background mode
    Cvar_SetValue(b"v_dark\x00" as *const u8 as *const libc::c_char, 1.0f32);
    cls.demos_pending = false_0;
    cls.demonum = 0 as libc::c_int;
    CL_NextDemo();
}
/*
==================
CL_DemoGetName
==================
*/
unsafe extern "C" fn CL_DemoGetName(mut lastnum: libc::c_int,
                                    mut filename: *mut libc::c_char) {
    if lastnum < 0 as libc::c_int || lastnum > 9999 as libc::c_int {
        // bound
        Q_strncpy(filename,
                  b"demo9999\x00" as *const u8 as *const libc::c_char,
                  99999 as libc::c_int as size_t);
        return
    }
    Q_sprintf(filename, b"demo%04d\x00" as *const u8 as *const libc::c_char,
              lastnum);
}
/*
====================
CL_Record_f

record <demoname>
Begins recording a demo from the current position
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_Record_f() {
    let mut demoname: string = [0; 256];
    let mut demopath: string = [0; 256];
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut n: libc::c_int = 0;
    if Cmd_Argc() == 1 as libc::c_int {
        name = b"new\x00" as *const u8 as *const libc::c_char
    } else if Cmd_Argc() == 2 as libc::c_int {
        name = Cmd_Argv(1 as libc::c_int)
    } else {
        Con_Printf(b"Usage: record <demoname>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    if cls.demorecording as u64 != 0 {
        Con_Printf(b"Already recording.\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    if cls.demoplayback != 0 {
        Con_Printf(b"Can\'t record during demo playback.\n\x00" as *const u8
                       as *const libc::c_char);
        return
    }
    if cls.demoheader.is_null() ||
           cls.state as libc::c_uint !=
               ca_active as libc::c_int as libc::c_uint {
        Con_Printf(b"You must be in a level to record.\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    if Q_strnicmp(name, b"new\x00" as *const u8 as *const libc::c_char,
                  99999 as libc::c_int) == 0 {
        // scan for a free filename
        n = 0 as libc::c_int;
        while n < 10000 as libc::c_int {
            CL_DemoGetName(n, demoname.as_mut_ptr());
            if FS_FileExists(va(b"%s.dem\x00" as *const u8 as
                                    *const libc::c_char,
                                demoname.as_mut_ptr()), true_0 as libc::c_int)
                   == 0 {
                break ;
            }
            n += 1
        }
        if n == 10000 as libc::c_int {
            Con_Printf(b"^1Error:^7 no free slots for demo recording\n\x00" as
                           *const u8 as *const libc::c_char);
            return
        }
    } else {
        Q_strncpy(demoname.as_mut_ptr(), name,
                  ::std::mem::size_of::<string>() as libc::c_ulong);
    }
    // open the demo file
    Q_sprintf(demopath.as_mut_ptr(),
              b"%s.dem\x00" as *const u8 as *const libc::c_char,
              demoname.as_mut_ptr());
    // make sure that old demo is removed
    if FS_FileExists(demopath.as_mut_ptr(), false_0 as libc::c_int) != 0 {
        FS_Delete(demopath.as_mut_ptr());
    }
    Q_strncpy(cls.demoname.as_mut_ptr(), demoname.as_mut_ptr(),
              ::std::mem::size_of::<string>() as libc::c_ulong);
    Q_strncpy((*gameui.globals).demoname.as_mut_ptr(), demoname.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    CL_WriteDemoHeader(demopath.as_mut_ptr());
}
/*
====================
CL_PlayDemo_f

playdemo <demoname>
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_PlayDemo_f() {
    let mut filename: [libc::c_char; 64] = [0; 64];
    let mut demoname: [libc::c_char; 64] = [0; 64];
    let mut i: libc::c_int = 0;
    let mut ident: libc::c_int = 0;
    if Cmd_Argc() < 2 as libc::c_int {
        Con_Printf(b"Usage: playdemo <demoname>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    if cls.demoplayback != 0 { CL_StopPlayback(); }
    if cls.demorecording as u64 != 0 {
        Con_Printf(b"Can\'t playback during demo record.\n\x00" as *const u8
                       as *const libc::c_char);
        return
    }
    Q_strncpy(demoname.as_mut_ptr(), Cmd_Argv(1 as libc::c_int),
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    COM_StripExtension(demoname.as_mut_ptr());
    Q_snprintf(filename.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
               b"%s.dem\x00" as *const u8 as *const libc::c_char,
               demoname.as_mut_ptr());
    // hidden parameter
    if Cmd_Argc() > 2 as libc::c_int {
        cls.set_lastdemo = Q_atoi(Cmd_Argv(2 as libc::c_int)) as qboolean
    }
    // member last demo
    if cls.set_lastdemo as u64 != 0 {
        Cvar_Set(b"lastdemo\x00" as *const u8 as *const libc::c_char,
                 demoname.as_mut_ptr()); // rewind back to start
    }
    if FS_FileExists(filename.as_mut_ptr(), true_0 as libc::c_int) == 0 {
        Con_Printf(b"^1Error:^7 couldn\'t open %s\n\x00" as *const u8 as
                       *const libc::c_char, filename.as_mut_ptr());
        CL_DemoAborted();
        return
    }
    cls.demofile =
        FS_Open(filename.as_mut_ptr(),
                b"rb\x00" as *const u8 as *const libc::c_char, true_0);
    Q_strncpy(cls.demoname.as_mut_ptr(), demoname.as_mut_ptr(),
              ::std::mem::size_of::<string>() as libc::c_ulong);
    Q_strncpy((*gameui.globals).demoname.as_mut_ptr(), demoname.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    FS_Read(cls.demofile, &mut ident as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    FS_Seek(cls.demofile, 0 as libc::c_int as fs_offset_t, 0 as libc::c_int);
    cls.forcetrack = 0 as libc::c_int;
    // check for quake demos
    if ident !=
           (('M' as i32) << 24 as libc::c_int) +
               (('E' as i32) << 16 as libc::c_int) +
               (('D' as i32) << 8 as libc::c_int) + 'I' as i32 {
        let mut c: libc::c_int = 0;
        let mut neg: libc::c_int = false_0 as libc::c_int;
        demo.header.host_fps = (*host_maxfps).value as libc::c_double;
        loop  {
            c = FS_Getc(cls.demofile);
            if !(c != '\n' as i32) { break ; }
            if c == '-' as i32 {
                neg = true_0 as libc::c_int
            } else {
                cls.forcetrack =
                    cls.forcetrack * 10 as libc::c_int + (c - '0' as i32)
            }
        }
        if neg != 0 { cls.forcetrack = -cls.forcetrack }
        CL_DemoStartPlayback(DEMO_QUAKE1 as libc::c_int);
        return
        // quake demo is started
    }
    // read in the demo header
    FS_Read(cls.demofile,
            &mut demo.header as *mut demoheader_t as *mut libc::c_void,
            ::std::mem::size_of::<demoheader_t>() as libc::c_ulong);
    if demo.header.id !=
           (('M' as i32) << 24 as libc::c_int) +
               (('E' as i32) << 16 as libc::c_int) +
               (('D' as i32) << 8 as libc::c_int) + 'I' as i32 {
        Con_Printf(b"^1Error:^7 %s is not a demo file\n\x00" as *const u8 as
                       *const libc::c_char, demoname.as_mut_ptr());
        CL_DemoAborted();
        return
    }
    if demo.header.dem_protocol != 3 as libc::c_int {
        Con_Printf(b"^1Error:^7 playdemo: demo protocol outdated (%i should be %i)\n\x00"
                       as *const u8 as *const libc::c_char,
                   demo.header.dem_protocol, 3 as libc::c_int);
        CL_DemoAborted();
        return
    }
    if demo.header.net_protocol != 49 as libc::c_int &&
           demo.header.net_protocol != 48 as libc::c_int {
        Con_Printf(b"^1Error:^7 playdemo: net protocol outdated (%i should be %i)\n\x00"
                       as *const u8 as *const libc::c_char,
                   demo.header.net_protocol, 49 as libc::c_int);
        CL_DemoAborted();
        return
    }
    // now read in the directory structure.
    FS_Seek(cls.demofile, demo.header.directory_offset as fs_offset_t,
            0 as libc::c_int);
    FS_Read(cls.demofile,
            &mut demo.directory.numentries as *mut libc::c_int as
                *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    if demo.directory.numentries < 1 as libc::c_int ||
           demo.directory.numentries > 1024 as libc::c_int {
        Con_Printf(b"^1Error:^7 demo had bogus # of directory entries: %i\n\x00"
                       as *const u8 as *const libc::c_char,
                   demo.directory.numentries);
        CL_DemoAborted();
        return
    }
    // allocate demo entries
    demo.directory.entries =
        _Mem_Alloc(cls.mempool,
                   (::std::mem::size_of::<demoentry_t>() as
                        libc::c_ulong).wrapping_mul(demo.directory.numentries
                                                        as libc::c_ulong),
                   false_0,
                   b"../engine/client/cl_demo.c\x00" as *const u8 as
                       *const libc::c_char, 1508 as libc::c_int) as
            *mut demoentry_t;
    i = 0 as libc::c_int;
    while i < demo.directory.numentries {
        FS_Read(cls.demofile,
                &mut *demo.directory.entries.offset(i as isize) as
                    *mut demoentry_t as *mut libc::c_void,
                ::std::mem::size_of::<demoentry_t>() as libc::c_ulong);
        i += 1
    }
    demo.entryIndex = 0 as libc::c_int;
    demo.entry =
        &mut *demo.directory.entries.offset(demo.entryIndex as isize) as
            *mut demoentry_t;
    FS_Seek(cls.demofile, (*demo.entry).offset as fs_offset_t,
            0 as libc::c_int);
    CL_DemoStartPlayback(DEMO_XASH3D as libc::c_int);
    // g-cont. is this need?
    Q_strncpy(cls.servername.as_mut_ptr(), demoname.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    cls.legacymode =
        (demo.header.net_protocol == 48 as libc::c_int) as libc::c_int as
            qboolean;
    // begin a playback demo
}
/*
====================
CL_TimeDemo_f

timedemo <demoname>
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_TimeDemo_f() {
    if Cmd_Argc() != 2 as libc::c_int {
        Con_Printf(b"Usage: timedemo <demoname>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    CL_PlayDemo_f();
    // cls.td_starttime will be grabbed at the second frame of the demo, so
	// all the loading time doesn't get counted
    cls.timedemo = true_0;
    cls.td_starttime = host.realtime;
    cls.td_startframe = host.framecount as libc::c_int;
    cls.td_lastframe = -(1 as libc::c_int);
    // get a new message this frame
}
/*
==================
CL_StartDemos_f
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_StartDemos_f() {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    if cls.key_dest as libc::c_uint != key_menu as libc::c_int as libc::c_uint
       {
        Con_Printf(b"\'startdemos\' is not valid from the console\n\x00" as
                       *const u8 as *const libc::c_char);
        return
    }
    c = Cmd_Argc() - 1 as libc::c_int;
    if c > 32 as libc::c_int {
        Con_DPrintf(b"^3Warning:^7 Host_StartDemos: max %i demos in demoloop\n\x00"
                        as *const u8 as *const libc::c_char,
                    32 as libc::c_int);
        c = 32 as libc::c_int
    }
    Con_Printf(b"%i demo%s in loop\n\x00" as *const u8 as *const libc::c_char,
               c,
               if c > 1 as libc::c_int {
                   b"s\x00" as *const u8 as *const libc::c_char
               } else { b"\x00" as *const u8 as *const libc::c_char });
    i = 1 as libc::c_int;
    while i < c + 1 as libc::c_int {
        Q_strncpy(cls.demos[(i - 1 as libc::c_int) as usize].as_mut_ptr(),
                  Cmd_Argv(i),
                  ::std::mem::size_of::<[libc::c_char; 64]>() as
                      libc::c_ulong);
        i += 1
    }
    cls.demos_pending = true_0;
}
/*
==================
CL_Demos_f

Return to looping demos
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_Demos_f() {
    if cls.key_dest as libc::c_uint != key_menu as libc::c_int as libc::c_uint
       {
        Con_Printf(b"\'demos\' is not valid from the console\n\x00" as
                       *const u8 as *const libc::c_char);
        return
    }
    // demos loop are not running
    if cls.olddemonum == -(1 as libc::c_int) { return }
    cls.demonum = cls.olddemonum;
    // run demos loop in background mode
    if SV_Active() as u64 == 0 && cls.demoplayback == 0 { CL_NextDemo(); };
}
/*
====================
CL_Stop_f

stop any client activity
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_Stop_f() {
    // stop all
    CL_StopRecord();
    CL_StopPlayback();
    SCR_StopCinematic();
    // stop background track that was runned from the console
    if SV_Active() as u64 == 0 { S_StopBackgroundTrack(); };
}
